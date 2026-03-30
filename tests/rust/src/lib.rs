include!(concat!(env!("OUT_DIR"), "/treesitter_types_generated.rs"));

pub fn parse_rust(src: &[u8]) -> tree_sitter::Tree {
    let mut parser = tree_sitter::Parser::new();
    parser
        .set_language(&tree_sitter_rust::LANGUAGE.into())
        .expect("failed to set Rust language");
    parser.parse(src, None).expect("parse failed")
}

#[cfg(test)]
mod tests {
    use super::*;
    use treesitter_types::{FromNode, LeafNode, Spanned};

    const COMPLEX_RUST: &[u8] = br#"use std::collections::HashMap;
use std::fmt;

pub trait Repository<T: Clone> {
    fn find_by_id(&self, id: &str) -> Option<&T>;
    fn save(&mut self, id: String, item: T);
}

#[derive(Debug, Clone)]
pub struct User {
    pub name: String,
    pub email: String,
}

impl fmt::Display for User {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{} <{}>", self.name, self.email)
    }
}

pub struct InMemoryRepo<T: Clone> {
    items: HashMap<String, T>,
}

impl<T: Clone> InMemoryRepo<T> {
    pub fn new() -> Self {
        Self {
            items: HashMap::new(),
        }
    }
}

impl<T: Clone> Repository<T> for InMemoryRepo<T> {
    fn find_by_id(&self, id: &str) -> Option<&T> {
        self.items.get(id)
    }

    fn save(&mut self, id: String, item: T) {
        self.items.insert(id, item);
    }
}

pub type UserRepo = InMemoryRepo<User>;

pub enum Status {
    Active,
    Inactive,
    Suspended(String),
}

const MAX_RETRIES: u32 = 3;

fn create_user(name: &str, email: &str) -> User {
    User {
        name: name.to_string(),
        email: email.to_string(),
    }
}
"#;

    /// Helper: extract all DeclarationStatement children of a specific variant.
    fn find_decl_stmts<'a>(
        children: &'a [SourceFileChildren<'a>],
    ) -> Vec<&'a DeclarationStatement<'a>> {
        children
            .iter()
            .filter_map(|c| match c {
                SourceFileChildren::DeclarationStatement(d) => Some(&**d),
                _ => None,
            })
            .collect()
    }

    #[test]
    fn test_source_file_parses() {
        let tree = parse_rust(COMPLEX_RUST);
        let sf = SourceFile::from_node(tree.root_node(), COMPLEX_RUST).unwrap();
        // Should have multiple children (use, trait, struct, impl, type, enum, const, fn, attributes)
        assert!(sf.children.len() >= 12);
    }

    #[test]
    fn test_use_declarations() {
        let tree = parse_rust(COMPLEX_RUST);
        let sf = SourceFile::from_node(tree.root_node(), COMPLEX_RUST).unwrap();
        let decls = find_decl_stmts(&sf.children);
        let use_decls: Vec<_> = decls
            .iter()
            .filter(|d| matches!(d, DeclarationStatement::UseDeclaration(_)))
            .collect();
        assert_eq!(use_decls.len(), 2);
    }

    #[test]
    fn test_trait_with_generics() {
        let tree = parse_rust(COMPLEX_RUST);
        let sf = SourceFile::from_node(tree.root_node(), COMPLEX_RUST).unwrap();
        let decls = find_decl_stmts(&sf.children);
        let trait_items: Vec<_> = decls
            .iter()
            .filter_map(|d| match d {
                DeclarationStatement::TraitItem(t) => Some(t),
                _ => None,
            })
            .collect();
        assert_eq!(trait_items.len(), 1);
        let trait_item = trait_items[0];
        assert_eq!(trait_item.name.text(), "Repository");
        assert!(trait_item.type_parameters.is_some());
    }

    #[test]
    fn test_struct_declarations() {
        let tree = parse_rust(COMPLEX_RUST);
        let sf = SourceFile::from_node(tree.root_node(), COMPLEX_RUST).unwrap();
        let decls = find_decl_stmts(&sf.children);
        let structs: Vec<_> = decls
            .iter()
            .filter_map(|d| match d {
                DeclarationStatement::StructItem(s) => Some(s),
                _ => None,
            })
            .collect();
        // User + InMemoryRepo
        assert_eq!(structs.len(), 2);
        assert_eq!(structs[0].name.text(), "User");
        assert_eq!(structs[1].name.text(), "InMemoryRepo");
    }

    #[test]
    fn test_user_struct_fields() {
        let tree = parse_rust(COMPLEX_RUST);
        let sf = SourceFile::from_node(tree.root_node(), COMPLEX_RUST).unwrap();
        let decls = find_decl_stmts(&sf.children);
        let user = decls
            .iter()
            .find_map(|d| match d {
                DeclarationStatement::StructItem(s) if s.name.text() == "User" => Some(s),
                _ => None,
            })
            .expect("User struct not found");

        let body = user.body.as_ref().expect("expected struct body");
        let StructItemBody::FieldDeclarationList(field_list) = body else {
            panic!("expected FieldDeclarationList");
        };
        let fields: Vec<_> = field_list
            .children
            .iter()
            .filter_map(|c| match c {
                FieldDeclarationListChildren::FieldDeclaration(f) => Some(f),
                _ => None,
            })
            .collect();
        assert_eq!(fields.len(), 2);
        assert_eq!(fields[0].name.text(), "name");
        assert_eq!(fields[1].name.text(), "email");
    }

    #[test]
    fn test_generic_struct() {
        let tree = parse_rust(COMPLEX_RUST);
        let sf = SourceFile::from_node(tree.root_node(), COMPLEX_RUST).unwrap();
        let decls = find_decl_stmts(&sf.children);
        let repo = decls
            .iter()
            .find_map(|d| match d {
                DeclarationStatement::StructItem(s) if s.name.text() == "InMemoryRepo" => Some(s),
                _ => None,
            })
            .expect("InMemoryRepo struct not found");
        assert!(repo.type_parameters.is_some());
    }

    #[test]
    fn test_impl_blocks() {
        let tree = parse_rust(COMPLEX_RUST);
        let sf = SourceFile::from_node(tree.root_node(), COMPLEX_RUST).unwrap();
        let decls = find_decl_stmts(&sf.children);
        let impls: Vec<_> = decls
            .iter()
            .filter_map(|d| match d {
                DeclarationStatement::ImplItem(i) => Some(i),
                _ => None,
            })
            .collect();
        // impl Display for User, impl InMemoryRepo<T>, impl Repository<T> for InMemoryRepo<T>
        assert_eq!(impls.len(), 3);

        // First impl: impl fmt::Display for User (has trait)
        assert!(impls[0].r#trait.is_some());
        // Second impl: impl<T: Clone> InMemoryRepo<T> (inherent, no trait)
        assert!(impls[1].r#trait.is_none());
        assert!(impls[1].type_parameters.is_some());
        // Third impl: impl<T: Clone> Repository<T> for InMemoryRepo<T> (has trait + type params)
        assert!(impls[2].r#trait.is_some());
        assert!(impls[2].type_parameters.is_some());
    }

    #[test]
    fn test_type_alias() {
        let tree = parse_rust(COMPLEX_RUST);
        let sf = SourceFile::from_node(tree.root_node(), COMPLEX_RUST).unwrap();
        let decls = find_decl_stmts(&sf.children);
        let type_items: Vec<_> = decls
            .iter()
            .filter_map(|d| match d {
                DeclarationStatement::TypeItem(t) => Some(t),
                _ => None,
            })
            .collect();
        assert_eq!(type_items.len(), 1);
        assert_eq!(type_items[0].name.text(), "UserRepo");
    }

    #[test]
    fn test_enum_declaration() {
        let tree = parse_rust(COMPLEX_RUST);
        let sf = SourceFile::from_node(tree.root_node(), COMPLEX_RUST).unwrap();
        let decls = find_decl_stmts(&sf.children);
        let enums: Vec<_> = decls
            .iter()
            .filter_map(|d| match d {
                DeclarationStatement::EnumItem(e) => Some(e),
                _ => None,
            })
            .collect();
        assert_eq!(enums.len(), 1);
        assert_eq!(enums[0].name.text(), "Status");
        assert!(enums[0].type_parameters.is_none());
    }

    #[test]
    fn test_const_item() {
        let tree = parse_rust(COMPLEX_RUST);
        let sf = SourceFile::from_node(tree.root_node(), COMPLEX_RUST).unwrap();
        let decls = find_decl_stmts(&sf.children);
        let consts: Vec<_> = decls
            .iter()
            .filter_map(|d| match d {
                DeclarationStatement::ConstItem(c) => Some(c),
                _ => None,
            })
            .collect();
        assert_eq!(consts.len(), 1);
        assert_eq!(consts[0].name.text(), "MAX_RETRIES");
    }

    #[test]
    fn test_function_item() {
        let tree = parse_rust(COMPLEX_RUST);
        let sf = SourceFile::from_node(tree.root_node(), COMPLEX_RUST).unwrap();
        let decls = find_decl_stmts(&sf.children);
        let funcs: Vec<_> = decls
            .iter()
            .filter_map(|d| match d {
                DeclarationStatement::FunctionItem(f) => Some(f),
                _ => None,
            })
            .collect();
        assert_eq!(funcs.len(), 1);
        let FunctionItemName::Identifier(name) = &funcs[0].name else {
            panic!("expected Identifier");
        };
        assert_eq!(name.text(), "create_user");
        assert!(funcs[0].return_type.is_some());
        assert!(funcs[0].type_parameters.is_none());
    }

    #[test]
    fn test_match_expression_inside_impl() {
        let src = br#"struct Foo { x: u32 }
impl Foo {
    fn bar(&self) -> u32 {
        match self.x {
            _ => 32,
        }
    }
}
"#;
        let tree = parse_rust(src);
        let root = tree.root_node();

        // First: try parsing the impl_item node directly as DeclarationStatement
        let mut cursor = root.walk();
        let impl_node = root
            .named_children(&mut cursor)
            .find(|c| c.kind() == "impl_item")
            .expect("no impl_item found");

        // This should succeed — impl_item is a subtype of _declaration_statement
        let result = DeclarationStatement::from_node(impl_node, src);
        assert!(
            result.is_ok(),
            "DeclarationStatement::from_node failed for impl_item: {}",
            result.unwrap_err()
        );

        // Full SourceFile parse should also succeed
        let sf = SourceFile::from_node(root, src);
        assert!(sf.is_ok(), "SourceFile parse failed: {}", sf.unwrap_err());
    }

    #[test]
    fn test_span_covers_source() {
        let tree = parse_rust(COMPLEX_RUST);
        let sf = SourceFile::from_node(tree.root_node(), COMPLEX_RUST).unwrap();
        assert_eq!(sf.span().start_byte, 0);
        assert!(sf.span().end_byte > 0);
        assert!(sf.span().end_byte <= COMPLEX_RUST.len());
    }
}
