use super::type_mapper::{
    AlternationEnumDef, ChildrenDef, FieldDef, FieldType, LeafStructDef, StructDef,
    SupertypeEnumDef, TypeDecision, TypeReference, VariantDef,
};
use proc_macro2::TokenStream;
use quote::{format_ident, quote};

/// Emits all generated code from a set of type decisions.
pub fn emit(decisions: &[TypeDecision]) -> TokenStream {
    let mut tokens = TokenStream::new();

    // Collect type names that have no lifetime parameter.
    // First pass: anonymous-only supertype enums (known statically).
    let mut no_lifetime_types: std::collections::HashSet<String> = decisions
        .iter()
        .filter_map(|d| {
            if let TypeDecision::SupertypeEnum(def) = d {
                if !def.variants.iter().any(|v| v.named) {
                    return Some(def.type_name.to_string());
                }
            }
            None
        })
        .collect();

    // Second pass: structs whose fields all resolve to no-lifetime types.
    // Iterate until stable (handles transitive dependencies).
    loop {
        let mut changed = false;
        for decision in decisions {
            if let TypeDecision::Struct(def) = decision {
                let name = def.type_name.to_string();
                if !no_lifetime_types.contains(&name)
                    && !struct_needs_lifetime(def, &no_lifetime_types)
                {
                    no_lifetime_types.insert(name);
                    changed = true;
                }
            }
        }
        if !changed {
            break;
        }
    }

    // Collect all alternation enums from struct fields to emit them alongside
    let mut alternation_enums: Vec<&AlternationEnumDef> = Vec::new();
    for decision in decisions {
        if let TypeDecision::Struct(def) = decision {
            collect_alternation_enums(def, &mut alternation_enums);
        }
    }

    for decision in decisions {
        match decision {
            TypeDecision::Struct(def) => tokens.extend(emit_struct(def, &no_lifetime_types)),
            TypeDecision::LeafStruct(def) => tokens.extend(emit_leaf_struct(def)),
            TypeDecision::SupertypeEnum(def) => {
                tokens.extend(emit_supertype_enum(def, &no_lifetime_types))
            }
        }
    }

    for alt in &alternation_enums {
        tokens.extend(emit_alternation_enum(alt, &no_lifetime_types));
    }

    // Emit the AnyNode top-level enum
    tokens.extend(emit_any_node(decisions, &no_lifetime_types));

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

/// Check whether a field type needs a lifetime parameter.
fn field_type_needs_lifetime(
    ft: &FieldType,
    no_lifetime_types: &std::collections::HashSet<String>,
) -> bool {
    let tr = match ft {
        FieldType::Direct(tr) | FieldType::Optional(tr) | FieldType::Repeated(tr) => tr,
    };
    match tr {
        TypeReference::Named(ident) => !no_lifetime_types.contains(&ident.to_string()),
        TypeReference::Alternation(alt) => alt.variants.iter().any(|v| v.named),
    }
}

/// Check whether a struct definition needs a lifetime parameter.
fn struct_needs_lifetime(
    def: &StructDef,
    no_lifetime_types: &std::collections::HashSet<String>,
) -> bool {
    def.fields
        .iter()
        .any(|f| field_type_needs_lifetime(&f.field_type, no_lifetime_types))
        || def
            .children
            .as_ref()
            .is_some_and(|c| field_type_needs_lifetime(&c.field_type, no_lifetime_types))
}

fn emit_struct(
    def: &StructDef,
    no_lifetime_types: &std::collections::HashSet<String>,
) -> TokenStream {
    let type_name = &def.type_name;
    let kind_str = &def.kind;
    let needs_lifetime = struct_needs_lifetime(def, no_lifetime_types);

    let field_decls: Vec<_> = def
        .fields
        .iter()
        .map(|f| emit_field_decl(f, type_name, no_lifetime_types))
        .collect();
    let field_parsers: Vec<_> = def
        .fields
        .iter()
        .map(|f| emit_field_parser(f, type_name, no_lifetime_types))
        .collect();

    let (children_decl, children_parser) = if let Some(children) = &def.children {
        (
            Some(emit_children_decl(children, type_name, no_lifetime_types)),
            Some(emit_children_parser(children, type_name)),
        )
    } else {
        (None, None)
    };

    if needs_lifetime {
        quote! {
            #[derive(Debug, Clone, PartialEq, Eq)]
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
    } else {
        // No-lifetime struct: fields are all anonymous-only types.
        // Still use `src` param since field parsers pass it to FromNode::from_node.
        let has_fields = !def.fields.is_empty() || def.children.is_some();
        let src_param = if has_fields {
            quote! { src: &'tree [u8] }
        } else {
            quote! { _src: &'tree [u8] }
        };
        quote! {
            #[derive(Debug, Clone, PartialEq, Eq)]
            pub struct #type_name {
                pub span: ::treesitter_types::Span,
                #(#field_decls)*
                #children_decl
            }

            impl<'tree> ::treesitter_types::FromNode<'tree> for #type_name {
                #[allow(clippy::match_single_binding, clippy::suspicious_else_formatting)]
                fn from_node(
                    node: ::tree_sitter::Node<'tree>,
                    #src_param,
                ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
                    debug_assert_eq!(node.kind(), #kind_str);
                    Ok(Self {
                        span: ::treesitter_types::Span::from(node),
                        #(#field_parsers)*
                        #children_parser
                    })
                }
            }

            impl ::treesitter_types::Spanned for #type_name {
                fn span(&self) -> ::treesitter_types::Span {
                    self.span
                }
            }
        }
    }
}

fn emit_field_decl(
    field: &FieldDef,
    parent_type: &proc_macro2::Ident,
    no_lifetime_types: &std::collections::HashSet<String>,
) -> TokenStream {
    let name = &field.field_name;
    let ty = emit_rust_type(&field.field_type, parent_type, no_lifetime_types);
    quote! { pub #name: #ty, }
}

fn emit_children_decl(
    children: &ChildrenDef,
    parent_type: &proc_macro2::Ident,
    no_lifetime_types: &std::collections::HashSet<String>,
) -> TokenStream {
    let ty = emit_rust_type(&children.field_type, parent_type, no_lifetime_types);
    quote! { pub children: #ty, }
}

/// Check if a type reference refers to the parent type (self-referential).
fn is_self_referential(tr: &TypeReference, parent_type: &proc_macro2::Ident) -> bool {
    match tr {
        TypeReference::Named(ident) => ident == parent_type,
        TypeReference::Alternation(_) => false,
    }
}

fn emit_rust_type(
    ft: &FieldType,
    parent_type: &proc_macro2::Ident,
    no_lifetime_types: &std::collections::HashSet<String>,
) -> TokenStream {
    match ft {
        FieldType::Direct(tr) => {
            let inner = emit_type_reference(tr, no_lifetime_types);
            if is_self_referential(tr, parent_type) {
                quote! { ::std::boxed::Box<#inner> }
            } else {
                quote! { #inner }
            }
        }
        FieldType::Optional(tr) => {
            let inner = emit_type_reference(tr, no_lifetime_types);
            if is_self_referential(tr, parent_type) {
                quote! { ::core::option::Option<::std::boxed::Box<#inner>> }
            } else {
                quote! { ::core::option::Option<#inner> }
            }
        }
        FieldType::Repeated(tr) => {
            let inner = emit_type_reference(tr, no_lifetime_types);
            quote! { ::std::vec::Vec<#inner> }
        }
    }
}

fn emit_type_reference(
    tr: &TypeReference,
    no_lifetime_types: &std::collections::HashSet<String>,
) -> TokenStream {
    match tr {
        TypeReference::Named(ident) => {
            if no_lifetime_types.contains(&ident.to_string()) {
                quote! { #ident }
            } else {
                quote! { #ident<'tree> }
            }
        }
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

fn emit_field_parser(
    field: &FieldDef,
    parent_type: &proc_macro2::Ident,
    no_lifetime_types: &std::collections::HashSet<String>,
) -> TokenStream {
    let _ = no_lifetime_types; // used via emit_rust_type for field decls
    let name = &field.field_name;
    let raw_name = &field.raw_field_name;

    match &field.field_type {
        FieldType::Direct(type_ref) => {
            let from_node = emit_from_node_call(type_ref);
            let self_ref = is_self_referential(type_ref, parent_type);
            let value_expr = if self_ref {
                quote! { ::std::boxed::Box::new(#from_node) }
            } else {
                from_node
            };
            quote! {
                #name: {
                    let child = node
                        .child_by_field_name(#raw_name)
                        .ok_or_else(|| ::treesitter_types::ParseError::missing_field(#raw_name, node))?;
                    #value_expr
                },
            }
        }
        FieldType::Optional(type_ref) => {
            let from_node = emit_from_node_call(type_ref);
            let self_ref = is_self_referential(type_ref, parent_type);
            let some_expr = if self_ref {
                quote! { Some(::std::boxed::Box::new(#from_node)) }
            } else {
                quote! { Some(#from_node) }
            };
            quote! {
                #name: match node.child_by_field_name(#raw_name) {
                    Some(child) => #some_expr,
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

fn emit_children_parser(children: &ChildrenDef, parent_type: &proc_macro2::Ident) -> TokenStream {
    let collector = emit_non_field_children_collector();
    match &children.field_type {
        FieldType::Direct(type_ref) => {
            let from_node = emit_from_node_call_named_children(type_ref);
            let self_ref = is_self_referential(type_ref, parent_type);
            let value_expr = if self_ref {
                quote! { ::std::boxed::Box::new(#from_node) }
            } else {
                from_node
            };
            // First try named non-field children (the common case). If none exist,
            // fall back to trying all non-field, non-extra children including anonymous
            // ones and pick the first that successfully parses. This handles grammars
            // where supertypes (e.g., `expression`) include anonymous subtypes
            // (e.g., `this`, `base` in C#) that `is_named()` would skip.
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
                                    let candidate = fallback_cursor.node();
                                    #[allow(clippy::needless_question_mark)]
                                    if (|| -> ::core::result::Result<_, ::treesitter_types::ParseError> {
                                        let child = candidate;
                                        Ok(#value_expr)
                                    })().is_ok() {
                                        fallback_child = Some(candidate);
                                        break;
                                    }
                                }
                                if !fallback_cursor.goto_next_sibling() {
                                    break;
                                }
                            }
                        }
                        // Second fallback: try children WITH field names.
                        // Some grammars (e.g., Haskell) have children that node-types.json
                        // lists as unnamed, but tree-sitter assigns inherited field names at
                        // runtime, causing the first fallback to skip them.
                        if fallback_child.is_none() {
                            let mut cursor2 = node.walk();
                            if cursor2.goto_first_child() {
                                loop {
                                    if cursor2.node().is_named() && !cursor2.node().is_extra() {
                                        let candidate = cursor2.node();
                                        #[allow(clippy::needless_question_mark)]
                                        if (|| -> ::core::result::Result<_, ::treesitter_types::ParseError> {
                                            let child = candidate;
                                            Ok(#value_expr)
                                        })().is_ok() {
                                            fallback_child = Some(candidate);
                                            break;
                                        }
                                    }
                                    if !cursor2.goto_next_sibling() {
                                        break;
                                    }
                                }
                            }
                        }
                        fallback_child.ok_or_else(|| ::treesitter_types::ParseError::missing_field("children", node))?
                    };
                    #value_expr
                },
            }
        }
        FieldType::Optional(type_ref) => {
            let from_node = emit_from_node_call_named_children(type_ref);
            let self_ref = is_self_referential(type_ref, parent_type);
            let some_expr = if self_ref {
                quote! { Some(::std::boxed::Box::new(#from_node)) }
            } else {
                quote! { Some(#from_node) }
            };
            quote! {
                children: {
                    #collector
                    match non_field_children.first() {
                        Some(&child) => #some_expr,
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
            quote! { ::treesitter_types::maybe_grow_stack(|| <#ident as ::treesitter_types::FromNode>::from_node(child, src))? }
        }
        TypeReference::Alternation(alt) => {
            let name = &alt.type_name;
            quote! { ::treesitter_types::maybe_grow_stack(|| <#name as ::treesitter_types::FromNode>::from_node(child, src))? }
        }
    }
}

fn emit_from_node_call_named_children(type_ref: &TypeReference) -> TokenStream {
    match type_ref {
        TypeReference::Named(ident) => {
            quote! { ::treesitter_types::maybe_grow_stack(|| <#ident as ::treesitter_types::FromNode>::from_node(child, src))? }
        }
        TypeReference::Alternation(alt) => {
            let name = &alt.type_name;
            quote! { ::treesitter_types::maybe_grow_stack(|| <#name as ::treesitter_types::FromNode>::from_node(child, src))? }
        }
    }
}

fn emit_leaf_struct(def: &LeafStructDef) -> TokenStream {
    let type_name = &def.type_name;
    let kind_str = &def.kind;

    quote! {
        #[derive(Debug, Clone, PartialEq, Eq)]
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

/// Deduplicate variants with the same name. When multiple anonymous nodes
/// produce the same variant name (e.g., "A" and "a" both → `A`), keep only
/// one variant and aggregate the extra kinds so we can match them all.
fn deduplicate_variants(variants: &[VariantDef]) -> Vec<VariantDef> {
    // Key includes both variant name and named flag, since anonymous variants
    // (Span payload) and named variants (Box<T> payload) are incompatible.
    let mut seen: std::collections::HashMap<(String, bool), usize> =
        std::collections::HashMap::new();
    let mut result: Vec<VariantDef> = Vec::new();
    let mut used_names: std::collections::HashSet<String> = std::collections::HashSet::new();
    for v in variants {
        let key = (v.variant_name.to_string(), v.named);
        if let Some(&idx) = seen.get(&key) {
            // Merge: same name and same named flag — can safely combine.
            result[idx].extra_kinds.push(v.kind.clone());
        } else {
            let mut v = v.clone();
            // If the name is already used by a variant with a different named flag,
            // disambiguate by appending "Kw" for the anonymous one
            let name_str = v.variant_name.to_string();
            if used_names.contains(&name_str) && !v.named {
                v.variant_name = format_ident!("{}Kw", name_str);
            }
            let actual_name = v.variant_name.to_string();
            seen.insert(key, result.len());
            used_names.insert(actual_name);
            used_names.insert(name_str);
            result.push(v);
        }
    }
    result
}

fn emit_alternation_enum(
    def: &AlternationEnumDef,
    no_lifetime_types: &std::collections::HashSet<String>,
) -> TokenStream {
    emit_enum_common(&def.type_name, &def.variants, no_lifetime_types)
}

fn emit_supertype_enum(
    def: &SupertypeEnumDef,
    no_lifetime_types: &std::collections::HashSet<String>,
) -> TokenStream {
    emit_enum_common(&def.type_name, &def.variants, no_lifetime_types)
}

fn emit_enum_common(
    type_name: &proc_macro2::Ident,
    variants: &[VariantDef],
    no_lifetime_types: &std::collections::HashSet<String>,
) -> TokenStream {
    // Deduplicate variant names: when multiple anonymous nodes map to the same
    // PascalCase name (e.g., "A" and "a" both become `A`), merge them into one
    // variant and handle both kinds in the match arm.
    let variants = deduplicate_variants(variants);
    let has_named = variants.iter().any(|v| v.named);
    let variant_decls: Vec<_> = variants
        .iter()
        .map(|v| emit_enum_variant_decl(v, no_lifetime_types))
        .collect();

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
                if let Ok(v) = ::treesitter_types::maybe_grow_stack(|| <#type_name as ::treesitter_types::FromNode>::from_node(node, src)) {
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
                if let Ok(v) = ::treesitter_types::maybe_grow_stack(|| <#type_name as ::treesitter_types::FromNode>::from_node(node, src)) {
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

    let spanned_arms: Vec<_> = variants.iter().map(emit_enum_spanned_arm).collect();

    if has_named {
        quote! {
            #[derive(Debug, Clone, PartialEq, Eq)]
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

            impl ::treesitter_types::Spanned for #type_name<'_> {
                fn span(&self) -> ::treesitter_types::Span {
                    match self {
                        #(#spanned_arms)*
                    }
                }
            }
        }
    } else {
        // All variants are anonymous — no lifetime needed.
        // Use `src` if supertype fallbacks need it, otherwise `_src`.
        let has_supertypes = !supertype_variants.is_empty();
        let src_param = if has_supertypes {
            quote! { src: &'tree [u8] }
        } else {
            quote! { _src: &'tree [u8] }
        };
        quote! {
            #[derive(Debug, Clone, PartialEq, Eq)]
            pub enum #type_name {
                #(#variant_decls)*
            }

            impl<'tree> ::treesitter_types::FromNode<'tree> for #type_name {
                #[allow(clippy::collapsible_else_if)]
                fn from_node(
                    node: ::tree_sitter::Node<'tree>,
                    #src_param,
                ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
                    #from_node_body
                }
            }

            impl ::treesitter_types::Spanned for #type_name {
                fn span(&self) -> ::treesitter_types::Span {
                    match self {
                        #(#spanned_arms)*
                    }
                }
            }
        }
    }
}

fn emit_enum_variant_decl(
    variant: &VariantDef,
    no_lifetime_types: &std::collections::HashSet<String>,
) -> TokenStream {
    let name = &variant.variant_name;
    if variant.named {
        let type_name = format_ident!("{}", name);
        if no_lifetime_types.contains(&type_name.to_string()) {
            quote! { #name(::std::boxed::Box<#type_name>), }
        } else {
            quote! { #name(::std::boxed::Box<#type_name<'tree>>), }
        }
    } else {
        // Anonymous variants carry a Span so consumers can locate them in source
        quote! { #name(::treesitter_types::Span), }
    }
}

fn emit_enum_match_arm(variant: &VariantDef) -> TokenStream {
    let kind_str = &variant.kind;
    let name = &variant.variant_name;
    let extra = &variant.extra_kinds;
    if variant.named {
        let type_name = format_ident!("{}", name);
        quote! {
            #kind_str #(| #extra)* => Ok(Self::#name(
                ::std::boxed::Box::new(::treesitter_types::maybe_grow_stack(|| <#type_name as ::treesitter_types::FromNode>::from_node(node, src))?)
            )),
        }
    } else {
        quote! {
            #kind_str #(| #extra)* => Ok(Self::#name(::treesitter_types::Span::from(node))),
        }
    }
}

fn emit_enum_spanned_arm(variant: &VariantDef) -> TokenStream {
    let name = &variant.variant_name;
    if variant.named {
        quote! { Self::#name(inner) => inner.span(), }
    } else {
        quote! { Self::#name(span) => *span, }
    }
}

fn emit_any_node(
    decisions: &[TypeDecision],
    no_lifetime_types: &std::collections::HashSet<String>,
) -> TokenStream {
    let mut variant_decls = Vec::new();
    let mut match_arms = Vec::new();
    let mut spanned_arms = Vec::new();

    for decision in decisions {
        let (type_name, kind_str) = match decision {
            TypeDecision::Struct(def) => (&def.type_name, &def.kind),
            TypeDecision::LeafStruct(def) => (&def.type_name, &def.kind),
            TypeDecision::SupertypeEnum(def) => (&def.type_name, &def.kind),
        };
        let needs_lifetime = !no_lifetime_types.contains(&type_name.to_string());

        if needs_lifetime {
            variant_decls.push(quote! {
                #type_name(#type_name<'tree>),
            });
        } else {
            variant_decls.push(quote! {
                #type_name(#type_name),
            });
        }

        match_arms.push(quote! {
            #kind_str => ::treesitter_types::maybe_grow_stack(|| <#type_name as ::treesitter_types::FromNode>::from_node(node, src))
                .map(Self::#type_name)
                .unwrap_or(Self::Unknown(node)),
        });

        spanned_arms.push(quote! {
            Self::#type_name(inner) => inner.span(),
        });
    }

    quote! {
        #[derive(Debug, Clone, PartialEq, Eq)]
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

        impl ::treesitter_types::Spanned for AnyNode<'_> {
            fn span(&self) -> ::treesitter_types::Span {
                match self {
                    #(#spanned_arms)*
                    Self::Unknown(node) => ::treesitter_types::Span::from(*node),
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
