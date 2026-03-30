use test_roundtrip::*;
use treesitter_types::{FromNode, LeafNode, Spanned};

fn fixture(name: &str) -> Vec<u8> {
    let path = format!("{}/fixtures/{name}", env!("CARGO_MANIFEST_DIR"));
    std::fs::read(&path).unwrap_or_else(|e| panic!("failed to read fixture {path}: {e}"))
}

mod go_tests {
    use super::*;
    use test_roundtrip::go::*;

    #[test]
    fn test_source_file_has_children() {
        let src = fixture("go.go");
        let tree = parse_go(&src);
        let sf = SourceFile::from_node(tree.root_node(), &src).unwrap();
        // package_clause + import_declaration + type_declaration + 2 func_declarations = 5
        assert_eq!(sf.children.len(), 5);
    }

    #[test]
    fn test_package_clause() {
        let src = fixture("go.go");
        let tree = parse_go(&src);
        let sf = SourceFile::from_node(tree.root_node(), &src).unwrap();
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
        let src = fixture("go.go");
        let tree = parse_go(&src);
        let sf = SourceFile::from_node(tree.root_node(), &src).unwrap();
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
        let src = fixture("go.go");
        let tree = parse_go(&src);
        let sf = SourceFile::from_node(tree.root_node(), &src).unwrap();
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
        let src = fixture("go.go");
        let tree = parse_go(&src);
        let sf = SourceFile::from_node(tree.root_node(), &src).unwrap();
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
        let src = fixture("go.go");
        let tree = parse_go(&src);
        let sf = SourceFile::from_node(tree.root_node(), &src).unwrap();
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
        let src = fixture("go.go");
        let tree = parse_go(&src);
        let sf = SourceFile::from_node(tree.root_node(), &src).unwrap();
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
        let src = fixture("go.go");
        let tree = parse_go(&src);
        let sf = SourceFile::from_node(tree.root_node(), &src).unwrap();
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

mod rust_tests {
    use super::*;
    use test_roundtrip::rust::*;

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
        let src = fixture("rust.rs");
        let tree = parse_rust(&src);
        let sf = SourceFile::from_node(tree.root_node(), &src).unwrap();
        // Should have multiple children (use, trait, struct, impl, type, enum, const, fn, attributes)
        assert!(sf.children.len() >= 12);
    }

    #[test]
    fn test_use_declarations() {
        let src = fixture("rust.rs");
        let tree = parse_rust(&src);
        let sf = SourceFile::from_node(tree.root_node(), &src).unwrap();
        let decls = find_decl_stmts(&sf.children);
        let use_decls: Vec<_> = decls
            .iter()
            .filter(|d| matches!(d, DeclarationStatement::UseDeclaration(_)))
            .collect();
        assert_eq!(use_decls.len(), 2);
    }

    #[test]
    fn test_trait_with_generics() {
        let src = fixture("rust.rs");
        let tree = parse_rust(&src);
        let sf = SourceFile::from_node(tree.root_node(), &src).unwrap();
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
        let src = fixture("rust.rs");
        let tree = parse_rust(&src);
        let sf = SourceFile::from_node(tree.root_node(), &src).unwrap();
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
        let src = fixture("rust.rs");
        let tree = parse_rust(&src);
        let sf = SourceFile::from_node(tree.root_node(), &src).unwrap();
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
        let src = fixture("rust.rs");
        let tree = parse_rust(&src);
        let sf = SourceFile::from_node(tree.root_node(), &src).unwrap();
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
        let src = fixture("rust.rs");
        let tree = parse_rust(&src);
        let sf = SourceFile::from_node(tree.root_node(), &src).unwrap();
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
        let src = fixture("rust.rs");
        let tree = parse_rust(&src);
        let sf = SourceFile::from_node(tree.root_node(), &src).unwrap();
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
        let src = fixture("rust.rs");
        let tree = parse_rust(&src);
        let sf = SourceFile::from_node(tree.root_node(), &src).unwrap();
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
        let src = fixture("rust.rs");
        let tree = parse_rust(&src);
        let sf = SourceFile::from_node(tree.root_node(), &src).unwrap();
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
        let src = fixture("rust.rs");
        let tree = parse_rust(&src);
        let sf = SourceFile::from_node(tree.root_node(), &src).unwrap();
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
        let src = fixture("rust.rs");
        let tree = parse_rust(&src);
        let sf = SourceFile::from_node(tree.root_node(), &src).unwrap();
        assert_eq!(sf.span().start_byte, 0);
        assert!(sf.span().end_byte > 0);
        assert!(sf.span().end_byte <= src.len());
    }
}

mod typescript_tests {
    use super::*;
    use test_roundtrip::typescript::*;

    #[test]
    fn test_program_children_count() {
        let src = fixture("typescript.ts");
        let tree = parse_typescript(&src);
        let program = Program::from_node(tree.root_node(), &src).unwrap();
        // import + 2 interfaces + type alias + class + function + const + export = 8
        assert_eq!(program.children.len(), 8);
    }

    #[test]
    fn test_import_statement() {
        let src = fixture("typescript.ts");
        let tree = parse_typescript(&src);
        let program = Program::from_node(tree.root_node(), &src).unwrap();
        let ProgramChildren::Statement(stmt) = &program.children[0] else {
            panic!("expected Statement");
        };
        let Statement::ImportStatement(import) = &**stmt else {
            panic!("expected ImportStatement");
        };
        assert!(import.span().start.row == 0);
    }

    #[test]
    fn test_interface_declaration() {
        let src = fixture("typescript.ts");
        let tree = parse_typescript(&src);
        let program = Program::from_node(tree.root_node(), &src).unwrap();
        // Second child is the Identifiable interface
        let ProgramChildren::Statement(stmt) = &program.children[1] else {
            panic!("expected Statement");
        };
        let Statement::Declaration(decl) = &**stmt else {
            panic!("expected Declaration");
        };
        let Declaration::InterfaceDeclaration(iface) = &**decl else {
            panic!("expected InterfaceDeclaration");
        };
        assert_eq!(iface.name.text(), "Identifiable");
        // No type parameters on this interface
        assert!(iface.type_parameters.is_none());
    }

    #[test]
    fn test_type_alias_with_generics() {
        let src = fixture("typescript.ts");
        let tree = parse_typescript(&src);
        let program = Program::from_node(tree.root_node(), &src).unwrap();
        // Third child is the Result type alias
        let ProgramChildren::Statement(stmt) = &program.children[2] else {
            panic!("expected Statement");
        };
        let Statement::Declaration(decl) = &**stmt else {
            panic!("expected Declaration");
        };
        let Declaration::TypeAliasDeclaration(type_alias) = &**decl else {
            panic!("expected TypeAliasDeclaration");
        };
        assert_eq!(type_alias.name.text(), "Result");
        // Should have type parameters (T, E = Error)
        let type_params = type_alias
            .type_parameters
            .as_ref()
            .expect("expected type_parameters");
        assert_eq!(type_params.children.len(), 2);
    }

    #[test]
    fn test_generic_interface() {
        let src = fixture("typescript.ts");
        let tree = parse_typescript(&src);
        let program = Program::from_node(tree.root_node(), &src).unwrap();
        // Fourth child: Repository<T extends Identifiable>
        let ProgramChildren::Statement(stmt) = &program.children[3] else {
            panic!("expected Statement");
        };
        let Statement::Declaration(decl) = &**stmt else {
            panic!("expected Declaration");
        };
        let Declaration::InterfaceDeclaration(iface) = &**decl else {
            panic!("expected InterfaceDeclaration");
        };
        assert_eq!(iface.name.text(), "Repository");
        let type_params = iface
            .type_parameters
            .as_ref()
            .expect("expected type_parameters");
        assert_eq!(type_params.children.len(), 1);
        // No extends clause on the interface itself — `extends` is on the type parameter
        assert!(iface.children.is_none());
    }

    #[test]
    fn test_generic_class() {
        let src = fixture("typescript.ts");
        let tree = parse_typescript(&src);
        let program = Program::from_node(tree.root_node(), &src).unwrap();
        // Fifth child: class InMemoryRepo<T extends Identifiable>
        let ProgramChildren::Statement(stmt) = &program.children[4] else {
            panic!("expected Statement");
        };
        let Statement::Declaration(decl) = &**stmt else {
            panic!("expected Declaration");
        };
        let Declaration::ClassDeclaration(class) = &**decl else {
            panic!("expected ClassDeclaration");
        };
        assert_eq!(class.name.text(), "InMemoryRepo");
        assert!(class.type_parameters.is_some());
        // implements clause (in class_heritage child)
        assert!(class.children.is_some());
    }

    #[test]
    fn test_generic_function() {
        let src = fixture("typescript.ts");
        let tree = parse_typescript(&src);
        let program = Program::from_node(tree.root_node(), &src).unwrap();
        // Sixth child: function createRepo<T>
        let ProgramChildren::Statement(stmt) = &program.children[5] else {
            panic!("expected Statement");
        };
        let Statement::Declaration(decl) = &**stmt else {
            panic!("expected Declaration");
        };
        let Declaration::FunctionDeclaration(func) = &**decl else {
            panic!("expected FunctionDeclaration");
        };
        assert_eq!(func.name.text(), "createRepo");
        assert!(func.type_parameters.is_some());
        assert!(func.return_type.is_some());
    }

    #[test]
    fn test_const_declaration() {
        let src = fixture("typescript.ts");
        let tree = parse_typescript(&src);
        let program = Program::from_node(tree.root_node(), &src).unwrap();
        // Seventh child: const defaultId
        let ProgramChildren::Statement(stmt) = &program.children[6] else {
            panic!("expected Statement");
        };
        let Statement::Declaration(decl) = &**stmt else {
            panic!("expected Declaration");
        };
        let Declaration::LexicalDeclaration(lex_decl) = &**decl else {
            panic!("expected LexicalDeclaration");
        };
        assert_eq!(lex_decl.children.len(), 1);
    }

    #[test]
    fn test_export_statement() {
        let src = fixture("typescript.ts");
        let tree = parse_typescript(&src);
        let program = Program::from_node(tree.root_node(), &src).unwrap();
        // Eighth child: export statement
        let ProgramChildren::Statement(stmt) = &program.children[7] else {
            panic!("expected Statement");
        };
        let Statement::ExportStatement(export) = &**stmt else {
            panic!("expected ExportStatement");
        };
        assert!(export.span().start.row > 0);
    }

    #[test]
    fn test_span_covers_source() {
        let src = fixture("typescript.ts");
        let tree = parse_typescript(&src);
        let program = Program::from_node(tree.root_node(), &src).unwrap();
        assert_eq!(program.span().start_byte, 0);
        assert!(program.span().end_byte > 0);
        assert!(program.span().end_byte <= src.len());
    }
}

mod javascript_tests {
    use super::*;
    use test_roundtrip::javascript::*;

    #[test]
    fn test_program_parses() {
        let src = fixture("javascript.js");
        let tree = parse_javascript(&src);
        let program = Program::from_node(tree.root_node(), &src).unwrap();
        assert!(program.children.len() >= 5);
    }

    #[test]
    fn test_function_declaration() {
        let src = fixture("javascript.js");
        let tree = parse_javascript(&src);
        let program = Program::from_node(tree.root_node(), &src).unwrap();
        let funcs: Vec<_> = program
            .children
            .iter()
            .filter_map(|c| match c {
                ProgramChildren::Statement(s) => match &**s {
                    Statement::Declaration(d) => match &**d {
                        Declaration::FunctionDeclaration(f) => Some(f),
                        _ => None,
                    },
                    _ => None,
                },
                _ => None,
            })
            .collect();
        assert_eq!(funcs.len(), 1);
        assert_eq!(funcs[0].name.text(), "add");
    }

    #[test]
    fn test_class_declaration() {
        let src = fixture("javascript.js");
        let tree = parse_javascript(&src);
        let program = Program::from_node(tree.root_node(), &src).unwrap();
        let classes: Vec<_> = program
            .children
            .iter()
            .filter_map(|c| match c {
                ProgramChildren::Statement(s) => match &**s {
                    Statement::Declaration(d) => match &**d {
                        Declaration::ClassDeclaration(cls) => Some(cls),
                        _ => None,
                    },
                    _ => None,
                },
                _ => None,
            })
            .collect();
        assert_eq!(classes.len(), 1);
        assert_eq!(classes[0].name.text(), "Counter");
    }

    #[test]
    fn test_export_statement() {
        let src = fixture("javascript.js");
        let tree = parse_javascript(&src);
        let program = Program::from_node(tree.root_node(), &src).unwrap();
        let exports: Vec<_> = program
            .children
            .iter()
            .filter_map(|c| match c {
                ProgramChildren::Statement(s) => match &**s {
                    Statement::ExportStatement(e) => Some(e),
                    _ => None,
                },
                _ => None,
            })
            .collect();
        assert_eq!(exports.len(), 1);
    }

    #[test]
    fn test_span_covers_source() {
        let src = fixture("javascript.js");
        let tree = parse_javascript(&src);
        let program = Program::from_node(tree.root_node(), &src).unwrap();
        assert_eq!(program.span().start_byte, 0);
        assert!(program.span().end_byte <= src.len());
    }

    #[test]
    fn test_minimal_program() {
        let src = b"var x = 1;\n";
        let tree = parse_javascript(src);
        let program = Program::from_node(tree.root_node(), src).unwrap();
        assert!(!program.children.is_empty());
    }
}

mod java_tests {
    use super::*;
    use test_roundtrip::java::*;

    #[test]
    fn test_program_parses() {
        let src = fixture("java.java");
        let tree = parse_java(&src);
        let program = Program::from_node(tree.root_node(), &src).unwrap();
        assert!(program.children.len() >= 5);
    }

    #[test]
    fn test_package_declaration() {
        let src = fixture("java.java");
        let tree = parse_java(&src);
        let program = Program::from_node(tree.root_node(), &src).unwrap();
        let has_package = program.children.iter().any(|c| {
            matches!(
                c,
                ProgramChildren::Statement(s)
                    if matches!(&**s, Statement::Declaration(d)
                        if matches!(&**d, Declaration::PackageDeclaration(_)))
            )
        });
        assert!(has_package);
    }

    #[test]
    fn test_import_declarations() {
        let src = fixture("java.java");
        let tree = parse_java(&src);
        let program = Program::from_node(tree.root_node(), &src).unwrap();
        let imports: Vec<_> = program
            .children
            .iter()
            .filter(|c| {
                matches!(
                    c,
                    ProgramChildren::Statement(s)
                        if matches!(&**s, Statement::Declaration(d)
                            if matches!(&**d, Declaration::ImportDeclaration(_)))
                )
            })
            .collect();
        assert_eq!(imports.len(), 2);
    }

    #[test]
    fn test_interface_declaration() {
        let src = fixture("java.java");
        let tree = parse_java(&src);
        let program = Program::from_node(tree.root_node(), &src).unwrap();
        let interfaces: Vec<_> = program
            .children
            .iter()
            .filter_map(|c| match c {
                ProgramChildren::Statement(s) => match &**s {
                    Statement::Declaration(d) => match &**d {
                        Declaration::InterfaceDeclaration(i) => Some(i),
                        _ => None,
                    },
                    _ => None,
                },
                _ => None,
            })
            .collect();
        assert_eq!(interfaces.len(), 1);
        assert_eq!(interfaces[0].name.text(), "Repository");
        assert!(interfaces[0].type_parameters.is_some());
    }

    #[test]
    fn test_class_declaration() {
        let src = fixture("java.java");
        let tree = parse_java(&src);
        let program = Program::from_node(tree.root_node(), &src).unwrap();
        let classes: Vec<_> = program
            .children
            .iter()
            .filter_map(|c| match c {
                ProgramChildren::Statement(s) => match &**s {
                    Statement::Declaration(d) => match &**d {
                        Declaration::ClassDeclaration(cls) => Some(cls),
                        _ => None,
                    },
                    _ => None,
                },
                _ => None,
            })
            .collect();
        assert_eq!(classes.len(), 1);
        assert_eq!(classes[0].name.text(), "InMemoryRepo");
        assert!(classes[0].type_parameters.is_some());
        assert!(classes[0].interfaces.is_some());
    }

    #[test]
    fn test_enum_declaration() {
        let src = fixture("java.java");
        let tree = parse_java(&src);
        let program = Program::from_node(tree.root_node(), &src).unwrap();
        let enums: Vec<_> = program
            .children
            .iter()
            .filter_map(|c| match c {
                ProgramChildren::Statement(s) => match &**s {
                    Statement::Declaration(d) => match &**d {
                        Declaration::EnumDeclaration(e) => Some(e),
                        _ => None,
                    },
                    _ => None,
                },
                _ => None,
            })
            .collect();
        assert_eq!(enums.len(), 1);
        assert_eq!(enums[0].name.text(), "Status");
    }

    #[test]
    fn test_span_covers_source() {
        let src = fixture("java.java");
        let tree = parse_java(&src);
        let program = Program::from_node(tree.root_node(), &src).unwrap();
        assert_eq!(program.span().start_byte, 0);
        assert!(program.span().end_byte <= src.len());
    }

    #[test]
    fn test_minimal_program() {
        let src = b"class Foo {}\n";
        let tree = parse_java(src);
        let program = Program::from_node(tree.root_node(), src).unwrap();
        assert!(!program.children.is_empty());
    }
}
