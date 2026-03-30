include!(concat!(env!("OUT_DIR"), "/treesitter_types_generated.rs"));

/// Parse Go source code into a tree-sitter tree.
pub fn parse_go(src: &[u8]) -> tree_sitter::Tree {
    let mut parser = tree_sitter::Parser::new();
    parser
        .set_language(&tree_sitter_go::LANGUAGE.into())
        .expect("failed to set Go language");
    parser.parse(src, None).expect("parse failed")
}

#[cfg(test)]
mod tests {
    use super::*;
    use treesitter_types::{FromNode, LeafNode, Spanned};

    const HELLO_WORLD: &[u8] = br#"package main

import "fmt"

type Point struct {
    X int
    Y int
}

func greet(name string) string {
    return "Hello, " + name
}

func main() {
    fmt.Println(greet("world"))
}
"#;

    #[test]
    fn test_source_file_has_children() {
        let tree = parse_go(HELLO_WORLD);
        let sf = SourceFile::from_node(tree.root_node(), HELLO_WORLD).unwrap();
        // package_clause + import_declaration + type_declaration + 2 func_declarations = 5
        assert_eq!(sf.children.len(), 5);
    }

    #[test]
    fn test_package_clause() {
        let tree = parse_go(HELLO_WORLD);
        let sf = SourceFile::from_node(tree.root_node(), HELLO_WORLD).unwrap();
        let SourceFileChildren::PackageClause(pkg) = &sf.children[0] else {
            panic!(
                "expected PackageClause, got {:?}",
                std::mem::discriminant(&sf.children[0])
            );
        };
        assert_eq!(pkg.children.text(), "main");
    }

    #[test]
    fn test_import_declaration() {
        let tree = parse_go(HELLO_WORLD);
        let sf = SourceFile::from_node(tree.root_node(), HELLO_WORLD).unwrap();
        let SourceFileChildren::ImportDeclaration(import_decl) = &sf.children[1] else {
            panic!("expected ImportDeclaration");
        };
        let ImportDeclarationChildren::ImportSpec(spec) = &import_decl.children else {
            panic!("expected ImportSpec");
        };
        // name is optional, should be None for `import "fmt"`
        assert!(spec.name.is_none());
    }

    #[test]
    fn test_type_declaration_via_supertype() {
        let tree = parse_go(HELLO_WORLD);
        let sf = SourceFile::from_node(tree.root_node(), HELLO_WORLD).unwrap();
        // The type declaration is a _statement, routed through the supertype fallback
        let SourceFileChildren::Statement(stmt) = &sf.children[2] else {
            panic!(
                "expected Statement (type_declaration), got {:?}",
                std::mem::discriminant(&sf.children[2])
            );
        };
        let Statement::TypeDeclaration(type_decl) = &**stmt else {
            panic!("expected TypeDeclaration");
        };
        assert!(!type_decl.children.is_empty());
        let TypeDeclarationChildren::TypeSpec(type_spec) = &type_decl.children[0] else {
            panic!("expected TypeSpec");
        };
        assert_eq!(type_spec.name.text(), "Point");
    }

    #[test]
    fn test_function_declaration_greet() {
        let tree = parse_go(HELLO_WORLD);
        let sf = SourceFile::from_node(tree.root_node(), HELLO_WORLD).unwrap();
        let SourceFileChildren::FunctionDeclaration(func) = &sf.children[3] else {
            panic!(
                "expected FunctionDeclaration, got {:?}",
                std::mem::discriminant(&sf.children[3])
            );
        };
        assert_eq!(func.name.text(), "greet");
        assert!(func.result.is_some());
        assert!(func.body.is_some());
    }

    #[test]
    fn test_function_declaration_main() {
        let tree = parse_go(HELLO_WORLD);
        let sf = SourceFile::from_node(tree.root_node(), HELLO_WORLD).unwrap();
        let SourceFileChildren::FunctionDeclaration(func) = &sf.children[4] else {
            panic!(
                "expected FunctionDeclaration, got {:?}",
                std::mem::discriminant(&sf.children[4])
            );
        };
        assert_eq!(func.name.text(), "main");
        assert!(func.result.is_none());
    }

    #[test]
    fn test_struct_fields() {
        let tree = parse_go(HELLO_WORLD);
        let sf = SourceFile::from_node(tree.root_node(), HELLO_WORLD).unwrap();
        let SourceFileChildren::Statement(stmt) = &sf.children[2] else {
            panic!("expected Statement");
        };
        let Statement::TypeDeclaration(type_decl) = &**stmt else {
            panic!("expected TypeDeclaration");
        };
        let TypeDeclarationChildren::TypeSpec(type_spec) = &type_decl.children[0] else {
            panic!("expected TypeSpec");
        };
        // _type → _simple_type → struct_type (two levels of supertype enums)
        let Type::SimpleType(simple) = &type_spec.r#type else {
            panic!("expected SimpleType");
        };
        let SimpleType::StructType(struct_type) = &**simple else {
            panic!("expected StructType");
        };
        // struct_type.children is a FieldDeclarationList (required: true, multiple: false)
        let field_list = &struct_type.children;
        assert_eq!(field_list.children.len(), 2);

        assert_eq!(field_list.children[0].name.len(), 1);
        assert_eq!(field_list.children[0].name[0].text(), "X");

        assert_eq!(field_list.children[1].name.len(), 1);
        assert_eq!(field_list.children[1].name[0].text(), "Y");
    }

    #[test]
    fn test_span_positions() {
        let tree = parse_go(HELLO_WORLD);
        let sf = SourceFile::from_node(tree.root_node(), HELLO_WORLD).unwrap();
        assert_eq!(sf.span().start.row, 0);
        assert_eq!(sf.span().start.column, 0);
        assert_eq!(sf.span().start_byte, 0);

        let SourceFileChildren::PackageClause(pkg) = &sf.children[0] else {
            panic!("expected PackageClause");
        };
        assert_eq!(pkg.span().start.row, 0);
    }

    #[test]
    fn test_minimal_program() {
        let src = b"package main\n";
        let tree = parse_go(src);
        let sf = SourceFile::from_node(tree.root_node(), src).unwrap();
        assert_eq!(sf.children.len(), 1);
        let SourceFileChildren::PackageClause(pkg) = &sf.children[0] else {
            panic!("expected PackageClause");
        };
        assert_eq!(pkg.children.text(), "main");
    }

    #[test]
    fn test_multiple_imports() {
        let src = br#"package main

import (
    "fmt"
    "os"
)
"#;
        let tree = parse_go(src);
        let sf = SourceFile::from_node(tree.root_node(), src).unwrap();
        let SourceFileChildren::ImportDeclaration(import_decl) = &sf.children[1] else {
            panic!("expected ImportDeclaration");
        };
        let ImportDeclarationChildren::ImportSpecList(spec_list) = &import_decl.children else {
            panic!("expected ImportSpecList");
        };
        assert_eq!(spec_list.children.len(), 2);
    }
}
