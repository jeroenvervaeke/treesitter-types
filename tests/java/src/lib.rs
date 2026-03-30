include!(concat!(env!("OUT_DIR"), "/treesitter_types_generated.rs"));

pub fn parse_java(src: &[u8]) -> tree_sitter::Tree {
    let mut parser = tree_sitter::Parser::new();
    parser
        .set_language(&tree_sitter_java::LANGUAGE.into())
        .expect("failed to set Java language");
    parser.parse(src, None).expect("parse failed")
}

#[cfg(test)]
mod tests {
    use super::*;
    use treesitter_types::{FromNode, LeafNode, Spanned};

    const JAVA_SOURCE: &[u8] = br#"package com.example;

import java.util.List;
import java.util.ArrayList;

public interface Repository<T> {
    T findById(String id);
    void save(T item);
}

public class InMemoryRepo<T> implements Repository<T> {
    private final List<T> items = new ArrayList<>();

    @Override
    public T findById(String id) {
        return null;
    }

    @Override
    public void save(T item) {
        items.add(item);
    }
}

public enum Status {
    ACTIVE,
    INACTIVE
}
"#;

    #[test]
    fn test_program_parses() {
        let tree = parse_java(JAVA_SOURCE);
        let program = Program::from_node(tree.root_node(), JAVA_SOURCE).unwrap();
        assert!(program.children.len() >= 5);
    }

    #[test]
    fn test_package_declaration() {
        let tree = parse_java(JAVA_SOURCE);
        let program = Program::from_node(tree.root_node(), JAVA_SOURCE).unwrap();
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
        let tree = parse_java(JAVA_SOURCE);
        let program = Program::from_node(tree.root_node(), JAVA_SOURCE).unwrap();
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
        let tree = parse_java(JAVA_SOURCE);
        let program = Program::from_node(tree.root_node(), JAVA_SOURCE).unwrap();
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
        let tree = parse_java(JAVA_SOURCE);
        let program = Program::from_node(tree.root_node(), JAVA_SOURCE).unwrap();
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
        let tree = parse_java(JAVA_SOURCE);
        let program = Program::from_node(tree.root_node(), JAVA_SOURCE).unwrap();
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
        let tree = parse_java(JAVA_SOURCE);
        let program = Program::from_node(tree.root_node(), JAVA_SOURCE).unwrap();
        assert_eq!(program.span().start_byte, 0);
        assert!(program.span().end_byte <= JAVA_SOURCE.len());
    }

    #[test]
    fn test_minimal_program() {
        let src = b"class Foo {}\n";
        let tree = parse_java(src);
        let program = Program::from_node(tree.root_node(), src).unwrap();
        assert!(!program.children.is_empty());
    }
}
