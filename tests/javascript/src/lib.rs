include!(concat!(env!("OUT_DIR"), "/treesitter_types_generated.rs"));

pub fn parse_javascript(src: &[u8]) -> tree_sitter::Tree {
    let mut parser = tree_sitter::Parser::new();
    parser
        .set_language(&tree_sitter_javascript::LANGUAGE.into())
        .expect("failed to set JavaScript language");
    parser.parse(src, None).expect("parse failed")
}

#[cfg(test)]
mod tests {
    use super::*;
    use treesitter_types::{FromNode, LeafNode, Spanned};

    const JS_SOURCE: &[u8] = br#"const greeting = "hello";

function add(a, b) {
    return a + b;
}

class Counter {
    constructor(initial) {
        this.count = initial;
    }

    increment() {
        this.count += 1;
    }
}

for (let i = 0; i < 10; i++) {
    console.log(add(i, 1));
}

export { Counter, add };
"#;

    #[test]
    fn test_program_parses() {
        let tree = parse_javascript(JS_SOURCE);
        let program = Program::from_node(tree.root_node(), JS_SOURCE).unwrap();
        assert!(program.children.len() >= 5);
    }

    #[test]
    fn test_function_declaration() {
        let tree = parse_javascript(JS_SOURCE);
        let program = Program::from_node(tree.root_node(), JS_SOURCE).unwrap();
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
        let tree = parse_javascript(JS_SOURCE);
        let program = Program::from_node(tree.root_node(), JS_SOURCE).unwrap();
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
        let tree = parse_javascript(JS_SOURCE);
        let program = Program::from_node(tree.root_node(), JS_SOURCE).unwrap();
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
        let tree = parse_javascript(JS_SOURCE);
        let program = Program::from_node(tree.root_node(), JS_SOURCE).unwrap();
        assert_eq!(program.span().start_byte, 0);
        assert!(program.span().end_byte <= JS_SOURCE.len());
    }

    #[test]
    fn test_minimal_program() {
        let src = b"var x = 1;\n";
        let tree = parse_javascript(src);
        let program = Program::from_node(tree.root_node(), src).unwrap();
        assert!(!program.children.is_empty());
    }
}
