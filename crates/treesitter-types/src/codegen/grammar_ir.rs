use serde::Deserialize;
use std::collections::BTreeMap;

/// A single type reference: a node kind + whether it's named.
#[derive(Debug, Clone, Deserialize, PartialEq, Eq)]
pub struct TypeRef {
    #[serde(rename = "type")]
    pub type_name: String,
    pub named: bool,
}

/// Describes a field or children slot: multiplicity, requiredness, and possible types.
#[derive(Debug, Clone, Deserialize, PartialEq, Eq)]
pub struct FieldInfo {
    pub multiple: bool,
    pub required: bool,
    pub types: Vec<TypeRef>,
}

/// A single entry from `node-types.json`.
#[derive(Debug, Clone, Deserialize, PartialEq, Eq)]
pub struct NodeType {
    #[serde(rename = "type")]
    pub type_name: String,
    pub named: bool,
    /// Named fields (only present on named nodes with fields).
    #[serde(default)]
    pub fields: BTreeMap<String, FieldInfo>,
    /// Unnamed children (present when the node has non-field children).
    pub children: Option<FieldInfo>,
    /// Subtypes (present on supertype/abstract nodes like `_expression`).
    pub subtypes: Option<Vec<TypeRef>>,
}

/// Parses the contents of a `node-types.json` file.
pub fn parse_node_types(json: &str) -> Result<Vec<NodeType>, serde_json::Error> {
    serde_json::from_str(json)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_leaf_node() {
        let json = r#"[{"type": "identifier", "named": true}]"#;
        let nodes = parse_node_types(json).unwrap();
        assert_eq!(nodes.len(), 1);
        assert_eq!(nodes[0].type_name, "identifier");
        assert!(nodes[0].named);
        assert!(nodes[0].fields.is_empty());
        assert!(nodes[0].children.is_none());
        assert!(nodes[0].subtypes.is_none());
    }

    #[test]
    fn test_parse_anonymous_node() {
        let json = r#"[{"type": ".", "named": false}]"#;
        let nodes = parse_node_types(json).unwrap();
        assert_eq!(nodes.len(), 1);
        assert_eq!(nodes[0].type_name, ".");
        assert!(!nodes[0].named);
    }

    #[test]
    fn test_parse_node_with_fields() {
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
                },
                "path": {
                    "multiple": false,
                    "required": true,
                    "types": [
                        {"type": "interpreted_string_literal", "named": true}
                    ]
                }
            }
        }]"#;
        let nodes = parse_node_types(json).unwrap();
        assert_eq!(nodes.len(), 1);
        let node = &nodes[0];
        assert_eq!(node.type_name, "import_spec");
        assert!(node.named);
        assert_eq!(node.fields.len(), 2);

        let name_field = &node.fields["name"];
        assert!(!name_field.multiple);
        assert!(!name_field.required);
        assert_eq!(name_field.types.len(), 2);
        assert_eq!(name_field.types[0].type_name, ".");
        assert!(!name_field.types[0].named);
        assert_eq!(name_field.types[1].type_name, "identifier");
        assert!(name_field.types[1].named);

        let path_field = &node.fields["path"];
        assert!(!path_field.multiple);
        assert!(path_field.required);
        assert_eq!(path_field.types.len(), 1);
    }

    #[test]
    fn test_parse_node_with_children() {
        let json = r#"[{
            "type": "import_spec_list",
            "named": true,
            "fields": {},
            "children": {
                "multiple": true,
                "required": false,
                "types": [
                    {"type": "import_spec", "named": true}
                ]
            }
        }]"#;
        let nodes = parse_node_types(json).unwrap();
        let node = &nodes[0];
        let children = node.children.as_ref().unwrap();
        assert!(children.multiple);
        assert!(!children.required);
        assert_eq!(children.types.len(), 1);
        assert_eq!(children.types[0].type_name, "import_spec");
    }

    #[test]
    fn test_parse_supertype_node() {
        let json = r#"[{
            "type": "_expression",
            "named": true,
            "subtypes": [
                {"type": "binary_expression", "named": true},
                {"type": "call_expression", "named": true},
                {"type": "identifier", "named": true}
            ]
        }]"#;
        let nodes = parse_node_types(json).unwrap();
        let node = &nodes[0];
        assert_eq!(node.type_name, "_expression");
        let subtypes = node.subtypes.as_ref().unwrap();
        assert_eq!(subtypes.len(), 3);
        assert_eq!(subtypes[0].type_name, "binary_expression");
        assert_eq!(subtypes[1].type_name, "call_expression");
        assert_eq!(subtypes[2].type_name, "identifier");
    }

    #[test]
    fn test_parse_multiple_nodes() {
        let json = r#"[
            {"type": "identifier", "named": true},
            {"type": ".", "named": false},
            {"type": "source_file", "named": true, "fields": {}, "children": {
                "multiple": true, "required": false,
                "types": [{"type": "identifier", "named": true}]
            }}
        ]"#;
        let nodes = parse_node_types(json).unwrap();
        assert_eq!(nodes.len(), 3);
    }
}
