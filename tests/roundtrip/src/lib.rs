pub use treesitter_types_go as go;
pub use treesitter_types_java as java;
pub use treesitter_types_javascript as javascript;
pub use treesitter_types_rust as rust;
pub use treesitter_types_typescript as typescript;

pub fn parse_go(src: &[u8]) -> tree_sitter::Tree {
    let mut parser = tree_sitter::Parser::new();
    parser
        .set_language(&tree_sitter_go::LANGUAGE.into())
        .expect("failed to set Go language");
    parser.parse(src, None).expect("parse failed")
}

pub fn parse_java(src: &[u8]) -> tree_sitter::Tree {
    let mut parser = tree_sitter::Parser::new();
    parser
        .set_language(&tree_sitter_java::LANGUAGE.into())
        .expect("failed to set Java language");
    parser.parse(src, None).expect("parse failed")
}

pub fn parse_javascript(src: &[u8]) -> tree_sitter::Tree {
    let mut parser = tree_sitter::Parser::new();
    parser
        .set_language(&tree_sitter_javascript::LANGUAGE.into())
        .expect("failed to set JavaScript language");
    parser.parse(src, None).expect("parse failed")
}

pub fn parse_rust(src: &[u8]) -> tree_sitter::Tree {
    let mut parser = tree_sitter::Parser::new();
    parser
        .set_language(&tree_sitter_rust::LANGUAGE.into())
        .expect("failed to set Rust language");
    parser.parse(src, None).expect("parse failed")
}

pub fn parse_typescript(src: &[u8]) -> tree_sitter::Tree {
    let mut parser = tree_sitter::Parser::new();
    parser
        .set_language(&tree_sitter_typescript::LANGUAGE_TYPESCRIPT.into())
        .expect("failed to set TypeScript language");
    parser.parse(src, None).expect("parse failed")
}
