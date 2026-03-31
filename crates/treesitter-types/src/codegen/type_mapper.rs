use super::grammar_ir::{FieldInfo, NodeType, TypeRef};
use super::name_mangler;
use proc_macro2::Ident;

/// What Rust type to emit for a top-level node.
#[derive(Debug, Clone)]
pub enum TypeDecision {
    /// A named node with fields and/or children → emit a struct.
    Struct(StructDef),
    /// A named terminal node (no fields, no children, no subtypes) → emit a leaf struct.
    LeafStruct(LeafStructDef),
    /// A supertype node (has `subtypes`) → emit an enum.
    SupertypeEnum(SupertypeEnumDef),
}

#[derive(Debug, Clone)]
pub struct StructDef {
    pub type_name: Ident,
    pub kind: String,
    pub fields: Vec<FieldDef>,
    pub children: Option<ChildrenDef>,
}

#[derive(Debug, Clone)]
pub struct LeafStructDef {
    pub type_name: Ident,
    pub kind: String,
}

#[derive(Debug, Clone)]
pub struct SupertypeEnumDef {
    pub type_name: Ident,
    pub kind: String,
    pub variants: Vec<VariantDef>,
}

#[derive(Debug, Clone)]
pub struct FieldDef {
    pub field_name: Ident,
    /// The original field name string (for `child_by_field_name`).
    pub raw_field_name: String,
    pub field_type: FieldType,
}

#[derive(Debug, Clone)]
pub struct ChildrenDef {
    pub field_type: FieldType,
}

/// How to represent a field's type in Rust.
#[derive(Debug, Clone)]
pub enum FieldType {
    /// A single concrete type (e.g., `Identifier<'tree>`).
    Direct(TypeReference),
    /// Wrapped in `Option<T>`.
    Optional(TypeReference),
    /// Wrapped in `Vec<T>`.
    Repeated(TypeReference),
}

/// Reference to a Rust type, which may be a single named type or an alternation enum.
#[derive(Debug, Clone)]
pub enum TypeReference {
    /// A single named node type.
    Named(Ident),
    /// An alternation enum generated for this field.
    Alternation(AlternationEnumDef),
}

#[derive(Debug, Clone)]
pub struct AlternationEnumDef {
    pub type_name: Ident,
    pub variants: Vec<VariantDef>,
}

#[derive(Debug, Clone)]
pub struct VariantDef {
    pub variant_name: Ident,
    pub kind: String,
    pub named: bool,
    /// True if this variant references a supertype (has subtypes in the grammar).
    /// Supertypes are abstract — tree-sitter never emits them as node kinds at runtime.
    pub is_supertype: bool,
    /// Additional kind strings that map to the same variant name (e.g., "a" and "A" both → `A`).
    /// Populated during deduplication in the emitter.
    pub extra_kinds: Vec<String>,
}

/// Maps all `NodeType` entries into `TypeDecision`s.
pub fn map_types(nodes: &[NodeType]) -> Vec<TypeDecision> {
    // Collect the set of type names that are supertypes
    let supertype_kinds: std::collections::HashSet<&str> = nodes
        .iter()
        .filter(|n| n.subtypes.is_some())
        .map(|n| n.type_name.as_str())
        .collect();

    // Collect concrete (non-supertype) node kinds for conflict detection
    let concrete_kinds: std::collections::HashSet<&str> = nodes
        .iter()
        .filter(|n| n.named && n.subtypes.is_none())
        .map(|n| n.type_name.as_str())
        .collect();

    let mut decisions: Vec<TypeDecision> = nodes
        .iter()
        .filter(|n| n.named)
        .map(|n| map_node(n, &supertype_kinds, &concrete_kinds))
        .collect();

    // Collect all defined type names
    let defined_kinds: std::collections::HashSet<String> = nodes
        .iter()
        .filter(|n| n.named)
        .map(|n| n.type_name.clone())
        .collect();

    // Collect all referenced named types from fields, children, and subtypes
    let mut referenced_kinds = std::collections::HashSet::new();
    for node in nodes.iter().filter(|n| n.named) {
        for field_info in node.fields.values() {
            for tr in &field_info.types {
                if tr.named {
                    referenced_kinds.insert(tr.type_name.clone());
                }
            }
        }
        if let Some(children) = &node.children {
            for tr in &children.types {
                if tr.named {
                    referenced_kinds.insert(tr.type_name.clone());
                }
            }
        }
        if let Some(subtypes) = &node.subtypes {
            for tr in subtypes {
                if tr.named {
                    referenced_kinds.insert(tr.type_name.clone());
                }
            }
        }
    }

    // Generate leaf structs for referenced but undefined types
    for kind in &referenced_kinds {
        if !defined_kinds.contains(kind) {
            decisions.push(TypeDecision::LeafStruct(LeafStructDef {
                type_name: name_mangler::type_ident(kind),
                kind: kind.clone(),
            }));
        }
    }

    decisions
}

fn map_node(
    node: &NodeType,
    supertype_kinds: &std::collections::HashSet<&str>,
    concrete_kinds: &std::collections::HashSet<&str>,
) -> TypeDecision {
    let raw_kind = &node.type_name;

    // Supertype nodes (e.g., _expression, statement) → enum
    if let Some(subtypes) = &node.subtypes {
        return TypeDecision::SupertypeEnum(SupertypeEnumDef {
            type_name: supertype_ident(raw_kind, concrete_kinds),
            kind: raw_kind.clone(),
            variants: subtypes
                .iter()
                .map(|tr| make_variant_def(tr, supertype_kinds))
                .collect(),
        });
    }

    // Named node with fields or children → struct
    if !node.fields.is_empty() || node.children.is_some() {
        let type_name = name_mangler::type_ident(raw_kind);
        let fields = node
            .fields
            .iter()
            .map(|(field_name, field_info)| {
                let parent_name = type_name.to_string();
                map_field(
                    field_name,
                    field_info,
                    &parent_name,
                    supertype_kinds,
                    concrete_kinds,
                )
            })
            .collect();
        let children = node.children.as_ref().map(|c| {
            let parent_name = type_name.to_string();
            map_children(c, &parent_name, supertype_kinds, concrete_kinds)
        });
        return TypeDecision::Struct(StructDef {
            type_name,
            kind: raw_kind.clone(),
            fields,
            children,
        });
    }

    // Leaf node (named, no fields, no children, no subtypes)
    TypeDecision::LeafStruct(LeafStructDef {
        type_name: name_mangler::type_ident(raw_kind),
        kind: raw_kind.clone(),
    })
}

fn make_variant_def(tr: &TypeRef, supertype_kinds: &std::collections::HashSet<&str>) -> VariantDef {
    VariantDef {
        variant_name: name_mangler::variant_name(&tr.type_name, tr.named),
        kind: tr.type_name.clone(),
        named: tr.named,
        is_supertype: tr.named && supertype_kinds.contains(tr.type_name.as_str()),
        extra_kinds: Vec::new(),
    }
}

fn map_field(
    field_name: &str,
    field_info: &FieldInfo,
    parent_name: &str,
    supertype_kinds: &std::collections::HashSet<&str>,
    concrete_kinds: &std::collections::HashSet<&str>,
) -> FieldDef {
    let type_ref = map_type_reference(
        &field_info.types,
        parent_name,
        field_name,
        supertype_kinds,
        concrete_kinds,
    );
    let field_type = match (field_info.required, field_info.multiple) {
        (_, true) => FieldType::Repeated(type_ref),
        (false, false) => FieldType::Optional(type_ref),
        (true, false) => FieldType::Direct(type_ref),
    };

    FieldDef {
        field_name: name_mangler::field_ident(field_name),
        raw_field_name: field_name.to_owned(),
        field_type,
    }
}

fn map_children(
    children: &FieldInfo,
    parent_name: &str,
    supertype_kinds: &std::collections::HashSet<&str>,
    concrete_kinds: &std::collections::HashSet<&str>,
) -> ChildrenDef {
    let type_ref = map_type_reference(
        &children.types,
        parent_name,
        "children",
        supertype_kinds,
        concrete_kinds,
    );
    let field_type = match (children.required, children.multiple) {
        (_, true) => FieldType::Repeated(type_ref),
        (false, false) => FieldType::Optional(type_ref),
        (true, false) => FieldType::Direct(type_ref),
    };
    ChildrenDef { field_type }
}

fn map_type_reference(
    types: &[TypeRef],
    parent_name: &str,
    field_name: &str,
    supertype_kinds: &std::collections::HashSet<&str>,
    concrete_kinds: &std::collections::HashSet<&str>,
) -> TypeReference {
    // Filter to only named types for the type reference.
    // Anonymous nodes (punctuation) in field types are unusual but can appear in alternations.
    if types.len() == 1 && types[0].named {
        let ident = if supertype_kinds.contains(types[0].type_name.as_str()) {
            supertype_ident(&types[0].type_name, concrete_kinds)
        } else {
            name_mangler::type_ident(&types[0].type_name)
        };
        TypeReference::Named(ident)
    } else {
        // Multiple types or contains anonymous → alternation enum
        let enum_name = format!(
            "{}{}",
            parent_name,
            name_mangler::to_pascal_case(field_name)
        );
        TypeReference::Alternation(AlternationEnumDef {
            type_name: quote::format_ident!("{}", enum_name),
            variants: types
                .iter()
                .map(|tr| make_variant_def(tr, supertype_kinds))
                .collect(),
        })
    }
}

/// Supertype nodes start with `_` (e.g., `_expression`). Strip the prefix for the type name.
/// If stripping would conflict with a concrete node name, keep a suffix to disambiguate.
fn supertype_ident(kind: &str, concrete_kinds: &std::collections::HashSet<&str>) -> Ident {
    let stripped = kind.strip_prefix('_').unwrap_or(kind);
    // Check if the stripped name conflicts with a concrete (non-supertype) node
    if concrete_kinds.contains(stripped) {
        // Append "Type" to disambiguate (e.g., _parameter → ParameterType, parameter → Parameter)
        let pascal = name_mangler::to_pascal_case(stripped);
        quote::format_ident!("{}Type", pascal)
    } else {
        name_mangler::type_ident(stripped)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::codegen::grammar_ir::parse_node_types;

    #[test]
    fn test_leaf_node_maps_to_leaf_struct() {
        let nodes = parse_node_types(r#"[{"type": "identifier", "named": true}]"#).unwrap();
        let decisions = map_types(&nodes);
        assert_eq!(decisions.len(), 1);
        assert!(
            matches!(&decisions[0], TypeDecision::LeafStruct(def) if def.type_name == "Identifier")
        );
    }

    #[test]
    fn test_unnamed_nodes_are_skipped() {
        let nodes = parse_node_types(r#"[{"type": ".", "named": false}]"#).unwrap();
        let decisions = map_types(&nodes);
        assert!(decisions.is_empty());
    }

    #[test]
    fn test_node_with_fields_maps_to_struct() {
        let json = r#"[
            {"type": "interpreted_string_literal", "named": true},
            {
                "type": "import_spec",
                "named": true,
                "fields": {
                    "path": {
                        "multiple": false,
                        "required": true,
                        "types": [{"type": "interpreted_string_literal", "named": true}]
                    }
                }
            }
        ]"#;
        let nodes = parse_node_types(json).unwrap();
        let decisions = map_types(&nodes);
        assert_eq!(decisions.len(), 2);
        let TypeDecision::Struct(def) = &decisions[1] else {
            panic!("expected Struct");
        };
        assert_eq!(def.type_name.to_string(), "ImportSpec");
        assert_eq!(def.fields.len(), 1);
        assert_eq!(def.fields[0].field_name.to_string(), "path");
        assert!(matches!(&def.fields[0].field_type, FieldType::Direct(_)));
    }

    #[test]
    fn test_optional_field() {
        let json = r#"[{
            "type": "import_spec",
            "named": true,
            "fields": {
                "name": {
                    "multiple": false,
                    "required": false,
                    "types": [{"type": "identifier", "named": true}]
                }
            }
        }]"#;
        let nodes = parse_node_types(json).unwrap();
        let decisions = map_types(&nodes);
        let TypeDecision::Struct(def) = &decisions[0] else {
            panic!("expected Struct");
        };
        assert!(matches!(&def.fields[0].field_type, FieldType::Optional(_)));
    }

    #[test]
    fn test_repeated_field() {
        let json = r#"[{
            "type": "block",
            "named": true,
            "fields": {
                "statements": {
                    "multiple": true,
                    "required": false,
                    "types": [{"type": "statement", "named": true}]
                }
            }
        }]"#;
        let nodes = parse_node_types(json).unwrap();
        let decisions = map_types(&nodes);
        let TypeDecision::Struct(def) = &decisions[0] else {
            panic!("expected Struct");
        };
        assert!(matches!(&def.fields[0].field_type, FieldType::Repeated(_)));
    }

    #[test]
    fn test_alternation_field() {
        let json = r#"[{
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
        }]"#;
        let nodes = parse_node_types(json).unwrap();
        let decisions = map_types(&nodes);
        let TypeDecision::Struct(def) = &decisions[0] else {
            panic!("expected Struct");
        };
        let FieldType::Optional(TypeReference::Alternation(alt)) = &def.fields[0].field_type else {
            panic!("expected Optional(Alternation)");
        };
        assert_eq!(alt.type_name.to_string(), "ImportSpecName");
        assert_eq!(alt.variants.len(), 2);
        assert_eq!(alt.variants[0].variant_name.to_string(), "Dot");
        assert_eq!(alt.variants[1].variant_name.to_string(), "Identifier");
    }

    #[test]
    fn test_supertype_maps_to_enum() {
        let json = r#"[{
            "type": "_expression",
            "named": true,
            "subtypes": [
                {"type": "binary_expression", "named": true},
                {"type": "identifier", "named": true}
            ]
        }]"#;
        let nodes = parse_node_types(json).unwrap();
        let decisions = map_types(&nodes);
        let TypeDecision::SupertypeEnum(def) = &decisions[0] else {
            panic!("expected SupertypeEnum");
        };
        assert_eq!(def.type_name.to_string(), "Expression");
        assert_eq!(def.variants.len(), 2);
    }

    #[test]
    fn test_node_with_children() {
        let json = r#"[{
            "type": "import_spec_list",
            "named": true,
            "fields": {},
            "children": {
                "multiple": true,
                "required": false,
                "types": [{"type": "import_spec", "named": true}]
            }
        }]"#;
        let nodes = parse_node_types(json).unwrap();
        let decisions = map_types(&nodes);
        let TypeDecision::Struct(def) = &decisions[0] else {
            panic!("expected Struct");
        };
        assert!(def.children.is_some());
        assert!(matches!(
            &def.children.as_ref().unwrap().field_type,
            FieldType::Repeated(_)
        ));
    }
}
