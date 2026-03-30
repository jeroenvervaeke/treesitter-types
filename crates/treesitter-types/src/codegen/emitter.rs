use super::type_mapper::{
    AlternationEnumDef, ChildrenDef, FieldDef, FieldType, LeafStructDef, StructDef,
    SupertypeEnumDef, TypeDecision, TypeReference, VariantDef,
};
use proc_macro2::TokenStream;
use quote::{format_ident, quote};

/// Emits all generated code from a set of type decisions.
pub fn emit(decisions: &[TypeDecision]) -> TokenStream {
    let mut tokens = TokenStream::new();

    // Collect all alternation enums from struct fields to emit them alongside
    let mut alternation_enums: Vec<&AlternationEnumDef> = Vec::new();
    for decision in decisions {
        if let TypeDecision::Struct(def) = decision {
            collect_alternation_enums(def, &mut alternation_enums);
        }
    }

    for decision in decisions {
        match decision {
            TypeDecision::Struct(def) => tokens.extend(emit_struct(def)),
            TypeDecision::LeafStruct(def) => tokens.extend(emit_leaf_struct(def)),
            TypeDecision::SupertypeEnum(def) => tokens.extend(emit_supertype_enum(def)),
        }
    }

    for alt in &alternation_enums {
        tokens.extend(emit_alternation_enum(alt));
    }

    // Emit the AnyNode top-level enum
    tokens.extend(emit_any_node(decisions));

    tokens
}

fn collect_alternation_enums<'a>(def: &'a StructDef, out: &mut Vec<&'a AlternationEnumDef>) {
    for field in &def.fields {
        collect_alternation_from_field_type(&field.field_type, out);
    }
    if let Some(children) = &def.children {
        collect_alternation_from_field_type(&children.field_type, out);
    }
}

fn collect_alternation_from_field_type<'a>(
    ft: &'a FieldType,
    out: &mut Vec<&'a AlternationEnumDef>,
) {
    let type_ref = match ft {
        FieldType::Direct(tr) | FieldType::Optional(tr) | FieldType::Repeated(tr) => tr,
    };
    if let TypeReference::Alternation(alt) = type_ref {
        out.push(alt);
    }
}

fn emit_struct(def: &StructDef) -> TokenStream {
    let type_name = &def.type_name;
    let kind_str = &def.kind;

    let field_decls: Vec<_> = def.fields.iter().map(emit_field_decl).collect();
    let field_parsers: Vec<_> = def.fields.iter().map(emit_field_parser).collect();

    let (children_decl, children_parser) = if let Some(children) = &def.children {
        (
            Some(emit_children_decl(children)),
            Some(emit_children_parser(children)),
        )
    } else {
        (None, None)
    };

    quote! {
        #[derive(Debug, Clone)]
        pub struct #type_name<'tree> {
            pub span: ::treesitter_types::Span,
            #(#field_decls)*
            #children_decl
        }

        impl<'tree> ::treesitter_types::FromNode<'tree> for #type_name<'tree> {
            #[allow(clippy::match_single_binding, clippy::suspicious_else_formatting)]
            fn from_node(
                node: ::tree_sitter::Node<'tree>,
                src: &'tree [u8],
            ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
                debug_assert_eq!(node.kind(), #kind_str);
                Ok(Self {
                    span: ::treesitter_types::Span::from(node),
                    #(#field_parsers)*
                    #children_parser
                })
            }
        }

        impl ::treesitter_types::Spanned for #type_name<'_> {
            fn span(&self) -> ::treesitter_types::Span {
                self.span
            }
        }
    }
}

fn emit_field_decl(field: &FieldDef) -> TokenStream {
    let name = &field.field_name;
    let ty = emit_rust_type(&field.field_type);
    quote! { pub #name: #ty, }
}

fn emit_children_decl(children: &ChildrenDef) -> TokenStream {
    let ty = emit_rust_type(&children.field_type);
    quote! { pub children: #ty, }
}

fn emit_rust_type(ft: &FieldType) -> TokenStream {
    match ft {
        FieldType::Direct(tr) => {
            let inner = emit_type_reference(tr);
            quote! { #inner }
        }
        FieldType::Optional(tr) => {
            let inner = emit_type_reference(tr);
            quote! { ::core::option::Option<#inner> }
        }
        FieldType::Repeated(tr) => {
            let inner = emit_type_reference(tr);
            quote! { ::std::vec::Vec<#inner> }
        }
    }
}

fn emit_type_reference(tr: &TypeReference) -> TokenStream {
    match tr {
        TypeReference::Named(ident) => quote! { #ident<'tree> },
        TypeReference::Alternation(alt) => {
            let name = &alt.type_name;
            let has_named = alt.variants.iter().any(|v| v.named);
            if has_named {
                quote! { #name<'tree> }
            } else {
                quote! { #name }
            }
        }
    }
}

fn emit_field_parser(field: &FieldDef) -> TokenStream {
    let name = &field.field_name;
    let raw_name = &field.raw_field_name;

    match &field.field_type {
        FieldType::Direct(type_ref) => {
            let from_node = emit_from_node_call(type_ref);
            quote! {
                #name: {
                    let child = node
                        .child_by_field_name(#raw_name)
                        .ok_or_else(|| ::treesitter_types::ParseError::missing_field(#raw_name, node))?;
                    #from_node
                },
            }
        }
        FieldType::Optional(type_ref) => {
            let from_node = emit_from_node_call(type_ref);
            quote! {
                #name: match node.child_by_field_name(#raw_name) {
                    Some(child) => Some(#from_node),
                    None => None,
                },
            }
        }
        FieldType::Repeated(type_ref) => {
            let from_node = emit_from_node_call(type_ref);
            quote! {
                #name: {
                    let mut cursor = node.walk();
                    let mut items = ::std::vec::Vec::new();
                    for child in node.children_by_field_name(#raw_name, &mut cursor) {
                        items.push(#from_node);
                    }
                    items
                },
            }
        }
    }
}

/// Emits a helper expression that collects non-field named children.
/// Filters by: no field name, is named, and not extra (comments).
/// Some grammars list anonymous nodes in children types (e.g., Rust `_` pattern),
/// but these are rare and handled by the `Direct` children parser which falls back
/// to trying all non-field children when no named children are found.
fn emit_non_field_children_collector() -> TokenStream {
    quote! {
        #[allow(clippy::suspicious_else_formatting)]
        let non_field_children = {
            let mut cursor = node.walk();
            let mut result = ::std::vec::Vec::new();
            if cursor.goto_first_child() {
                loop {
                    if cursor.field_name().is_none() && cursor.node().is_named() && !cursor.node().is_extra() {
                        result.push(cursor.node());
                    }
                    if !cursor.goto_next_sibling() {
                        break;
                    }
                }
            }
            result
        };
    }
}

fn emit_children_parser(children: &ChildrenDef) -> TokenStream {
    let collector = emit_non_field_children_collector();
    match &children.field_type {
        FieldType::Direct(type_ref) => {
            let from_node = emit_from_node_call_named_children(type_ref);
            // Fall back to all non-field non-extra children (including anonymous)
            // when no named children are found. This handles grammars that list
            // anonymous nodes as valid children (e.g., Rust `_` wildcard pattern).
            quote! {
                children: {
                    #collector
                    let child = if let Some(&c) = non_field_children.first() {
                        c
                    } else {
                        let mut fallback_cursor = node.walk();
                        let mut fallback_child = None;
                        if fallback_cursor.goto_first_child() {
                            loop {
                                if fallback_cursor.field_name().is_none() && !fallback_cursor.node().is_extra() {
                                    fallback_child = Some(fallback_cursor.node());
                                    break;
                                }
                                if !fallback_cursor.goto_next_sibling() {
                                    break;
                                }
                            }
                        }
                        fallback_child.ok_or_else(|| ::treesitter_types::ParseError::missing_field("children", node))?
                    };
                    #from_node
                },
            }
        }
        FieldType::Optional(type_ref) => {
            let from_node = emit_from_node_call_named_children(type_ref);
            quote! {
                children: {
                    #collector
                    match non_field_children.first() {
                        Some(&child) => Some(#from_node),
                        None => None,
                    }
                },
            }
        }
        FieldType::Repeated(type_ref) => {
            let from_node = emit_from_node_call_named_children(type_ref);
            quote! {
                children: {
                    #collector
                    let mut items = ::std::vec::Vec::new();
                    for child in non_field_children {
                        items.push(#from_node);
                    }
                    items
                },
            }
        }
    }
}

fn emit_from_node_call(type_ref: &TypeReference) -> TokenStream {
    match type_ref {
        TypeReference::Named(ident) => {
            quote! { <#ident as ::treesitter_types::FromNode>::from_node(child, src)? }
        }
        TypeReference::Alternation(alt) => {
            let name = &alt.type_name;
            quote! { <#name as ::treesitter_types::FromNode>::from_node(child, src)? }
        }
    }
}

fn emit_from_node_call_named_children(type_ref: &TypeReference) -> TokenStream {
    match type_ref {
        TypeReference::Named(ident) => {
            quote! { <#ident as ::treesitter_types::FromNode>::from_node(child, src)? }
        }
        TypeReference::Alternation(alt) => {
            let name = &alt.type_name;
            quote! { <#name as ::treesitter_types::FromNode>::from_node(child, src)? }
        }
    }
}

fn emit_leaf_struct(def: &LeafStructDef) -> TokenStream {
    let type_name = &def.type_name;
    let kind_str = &def.kind;

    quote! {
        #[derive(Debug, Clone)]
        pub struct #type_name<'tree> {
            pub span: ::treesitter_types::Span,
            text: &'tree str,
        }

        impl<'tree> ::treesitter_types::FromNode<'tree> for #type_name<'tree> {
            fn from_node(
                node: ::tree_sitter::Node<'tree>,
                src: &'tree [u8],
            ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
                debug_assert_eq!(node.kind(), #kind_str);
                Ok(Self {
                    span: ::treesitter_types::Span::from(node),
                    text: node.utf8_text(src)?,
                })
            }
        }

        impl<'tree> ::treesitter_types::LeafNode<'tree> for #type_name<'tree> {
            fn text(&self) -> &'tree str {
                self.text
            }
        }

        impl ::treesitter_types::Spanned for #type_name<'_> {
            fn span(&self) -> ::treesitter_types::Span {
                self.span
            }
        }
    }
}

fn emit_alternation_enum(def: &AlternationEnumDef) -> TokenStream {
    emit_enum_common(&def.type_name, &def.variants)
}

fn emit_supertype_enum(def: &SupertypeEnumDef) -> TokenStream {
    emit_enum_common(&def.type_name, &def.variants)
}

fn emit_enum_common(type_name: &proc_macro2::Ident, variants: &[VariantDef]) -> TokenStream {
    let has_named = variants.iter().any(|v| v.named);
    let variant_decls: Vec<_> = variants.iter().map(emit_enum_variant_decl).collect();

    // Separate concrete variants (direct kind match) from supertype variants
    // (kinds starting with `_` — tree-sitter never emits these as node kinds,
    // they represent abstract supertypes whose concrete subtypes appear at runtime)
    let (supertype_variants, concrete_variants): (Vec<_>, Vec<_>) =
        variants.iter().partition(|v| v.is_supertype);

    let concrete_arms: Vec<_> = concrete_variants
        .iter()
        .map(|v| emit_enum_match_arm(v))
        .collect();
    // Build a chained if/else-if for supertype fallbacks to avoid
    // clippy::suspicious_else_formatting on consecutive `if let` blocks
    let fallback_branch = if supertype_variants.is_empty() {
        quote! { other => Err(::treesitter_types::ParseError::unexpected_kind(other, node)) }
    } else {
        let mut chain = quote! {
            Err(::treesitter_types::ParseError::unexpected_kind(_other, node))
        };
        for v in supertype_variants.iter().rev() {
            let name = &v.variant_name;
            let type_name = format_ident!("{}", name);
            chain = quote! {
                if let Ok(v) = <#type_name as ::treesitter_types::FromNode>::from_node(node, src) {
                    Ok(Self::#name(::std::boxed::Box::new(v)))
                } else {
                    #chain
                }
            };
        }
        quote! { _other => { #chain } }
    };

    // When all variants are supertypes, skip the match and emit the fallback directly
    let from_node_body = if concrete_arms.is_empty() {
        // All variants are supertypes — no concrete kind matches needed
        let mut chain = quote! {
            Err(::treesitter_types::ParseError::unexpected_kind(node.kind(), node))
        };
        for v in supertype_variants.iter().rev() {
            let name = &v.variant_name;
            let type_name = format_ident!("{}", name);
            chain = quote! {
                if let Ok(v) = <#type_name as ::treesitter_types::FromNode>::from_node(node, src) {
                    Ok(Self::#name(::std::boxed::Box::new(v)))
                } else {
                    #chain
                }
            };
        }
        chain
    } else {
        quote! {
            match node.kind() {
                #(#concrete_arms)*
                #fallback_branch
            }
        }
    };

    if has_named {
        quote! {
            #[derive(Debug, Clone)]
            pub enum #type_name<'tree> {
                #(#variant_decls)*
            }

            impl<'tree> ::treesitter_types::FromNode<'tree> for #type_name<'tree> {
                #[allow(clippy::collapsible_else_if)]
                fn from_node(
                    node: ::tree_sitter::Node<'tree>,
                    src: &'tree [u8],
                ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
                    #from_node_body
                }
            }
        }
    } else {
        // All variants are anonymous — no lifetime needed, suppress unused `src` warning
        quote! {
            #[derive(Debug, Clone)]
            pub enum #type_name {
                #(#variant_decls)*
            }

            impl<'tree> ::treesitter_types::FromNode<'tree> for #type_name {
                #[allow(clippy::collapsible_else_if)]
                fn from_node(
                    node: ::tree_sitter::Node<'tree>,
                    _src: &'tree [u8],
                ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
                    #from_node_body
                }
            }
        }
    }
}

fn emit_enum_variant_decl(variant: &VariantDef) -> TokenStream {
    let name = &variant.variant_name;
    if variant.named {
        let type_name = format_ident!("{}", name);
        // Box the payload to handle recursive types (e.g., Expression → BinaryExpression → Expression)
        quote! { #name(::std::boxed::Box<#type_name<'tree>>), }
    } else {
        quote! { #name, }
    }
}

fn emit_enum_match_arm(variant: &VariantDef) -> TokenStream {
    let kind_str = &variant.kind;
    let name = &variant.variant_name;
    if variant.named {
        let type_name = format_ident!("{}", name);
        quote! {
            #kind_str => Ok(Self::#name(
                ::std::boxed::Box::new(<#type_name as ::treesitter_types::FromNode>::from_node(node, src)?)
            )),
        }
    } else {
        quote! {
            #kind_str => Ok(Self::#name),
        }
    }
}

fn emit_any_node(decisions: &[TypeDecision]) -> TokenStream {
    let mut variant_decls = Vec::new();
    let mut match_arms = Vec::new();

    for decision in decisions {
        let (type_name, kind_str) = match decision {
            TypeDecision::Struct(def) => (&def.type_name, &def.kind),
            TypeDecision::LeafStruct(def) => (&def.type_name, &def.kind),
            TypeDecision::SupertypeEnum(def) => (&def.type_name, &def.kind),
        };

        variant_decls.push(quote! {
            #type_name(#type_name<'tree>),
        });

        match_arms.push(quote! {
            #kind_str => <#type_name as ::treesitter_types::FromNode>::from_node(node, src)
                .map(Self::#type_name)
                .unwrap_or(Self::Unknown(node)),
        });
    }

    quote! {
        #[derive(Debug, Clone)]
        pub enum AnyNode<'tree> {
            #(#variant_decls)*
            Unknown(::tree_sitter::Node<'tree>),
        }

        impl<'tree> AnyNode<'tree> {
            pub fn from_node(node: ::tree_sitter::Node<'tree>, src: &'tree [u8]) -> Self {
                match node.kind() {
                    #(#match_arms)*
                    _ => Self::Unknown(node),
                }
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::codegen::grammar_ir::parse_node_types;
    use crate::codegen::type_mapper::map_types;

    fn generate_and_stringify(json: &str) -> String {
        let nodes = parse_node_types(json).unwrap();
        let decisions = map_types(&nodes);
        let tokens = emit(&decisions);
        tokens.to_string()
    }

    #[test]
    fn test_emit_leaf_struct() {
        let code = generate_and_stringify(r#"[{"type": "identifier", "named": true}]"#);
        assert!(code.contains("pub struct Identifier"));
        assert!(code
            .contains("impl < 'tree > :: treesitter_types :: LeafNode < 'tree > for Identifier"));
        assert!(code.contains("fn text"));
    }

    #[test]
    fn test_emit_struct_with_required_field() {
        let json = r#"[
            {"type": "identifier", "named": true},
            {
                "type": "import_spec",
                "named": true,
                "fields": {
                    "path": {
                        "multiple": false,
                        "required": true,
                        "types": [{"type": "identifier", "named": true}]
                    }
                }
            }
        ]"#;
        let code = generate_and_stringify(json);
        assert!(code.contains("pub struct ImportSpec"));
        assert!(code.contains("pub path : Identifier < 'tree >"));
        assert!(code.contains("child_by_field_name (\"path\")"));
    }

    #[test]
    fn test_emit_struct_with_optional_field() {
        let json = r#"[
            {"type": "identifier", "named": true},
            {
                "type": "import_spec",
                "named": true,
                "fields": {
                    "name": {
                        "multiple": false,
                        "required": false,
                        "types": [{"type": "identifier", "named": true}]
                    }
                }
            }
        ]"#;
        let code = generate_and_stringify(json);
        assert!(code.contains("Option < Identifier < 'tree > >"));
    }

    #[test]
    fn test_emit_alternation_enum() {
        let json = r#"[
            {"type": "identifier", "named": true},
            {
                "type": "import_spec",
                "named": true,
                "fields": {
                    "name": {
                        "multiple": false,
                        "required": false,
                        "types": [
                            {"type": ".", "named": false},
                            {"type": "identifier", "named": true}
                        ]
                    }
                }
            }
        ]"#;
        let code = generate_and_stringify(json);
        assert!(code.contains("pub enum ImportSpecName"));
        assert!(code.contains("Dot"));
        assert!(code.contains("Identifier (Identifier < 'tree >)"));
    }

    #[test]
    fn test_emit_supertype_enum() {
        let json = r#"[
            {"type": "identifier", "named": true},
            {"type": "binary_expression", "named": true},
            {
                "type": "_expression",
                "named": true,
                "subtypes": [
                    {"type": "binary_expression", "named": true},
                    {"type": "identifier", "named": true}
                ]
            }
        ]"#;
        let code = generate_and_stringify(json);
        assert!(code.contains("pub enum Expression"));
        assert!(code.contains("BinaryExpression (BinaryExpression < 'tree >)"));
        assert!(code.contains("Identifier (Identifier < 'tree >)"));
    }

    #[test]
    fn test_emit_any_node() {
        let json = r#"[
            {"type": "identifier", "named": true},
            {"type": ".", "named": false}
        ]"#;
        let code = generate_and_stringify(json);
        assert!(code.contains("pub enum AnyNode"));
        assert!(code.contains("Identifier (Identifier < 'tree >)"));
        assert!(code.contains("Unknown (:: tree_sitter :: Node < 'tree >)"));
    }
}
