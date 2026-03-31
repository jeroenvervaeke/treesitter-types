pub use treesitter_types_bash as bash;
pub use treesitter_types_c as c;
pub use treesitter_types_c_sharp as c_sharp;
pub use treesitter_types_cpp as cpp;
pub use treesitter_types_css as css;
pub use treesitter_types_go as go;
pub use treesitter_types_haskell as haskell;
pub use treesitter_types_html as html;
pub use treesitter_types_java as java;
pub use treesitter_types_javascript as javascript;
pub use treesitter_types_json as json;
pub use treesitter_types_php as php;
pub use treesitter_types_python as python;
pub use treesitter_types_regex as regex;
pub use treesitter_types_ruby as ruby;
pub use treesitter_types_rust as rust;
pub use treesitter_types_scala as scala;
pub use treesitter_types_typescript as typescript;

pub fn parse_bash(src: &[u8]) -> tree_sitter::Tree {
    let mut parser = tree_sitter::Parser::new();
    parser
        .set_language(&tree_sitter_bash::LANGUAGE.into())
        .expect("failed to set Bash language");
    parser.parse(src, None).expect("parse failed")
}

pub fn parse_c(src: &[u8]) -> tree_sitter::Tree {
    let mut parser = tree_sitter::Parser::new();
    parser
        .set_language(&tree_sitter_c::LANGUAGE.into())
        .expect("failed to set C language");
    parser.parse(src, None).expect("parse failed")
}

pub fn parse_c_sharp(src: &[u8]) -> tree_sitter::Tree {
    let mut parser = tree_sitter::Parser::new();
    parser
        .set_language(&tree_sitter_c_sharp::LANGUAGE.into())
        .expect("failed to set C# language");
    parser.parse(src, None).expect("parse failed")
}

pub fn parse_cpp(src: &[u8]) -> tree_sitter::Tree {
    let mut parser = tree_sitter::Parser::new();
    parser
        .set_language(&tree_sitter_cpp::LANGUAGE.into())
        .expect("failed to set C++ language");
    parser.parse(src, None).expect("parse failed")
}

pub fn parse_css(src: &[u8]) -> tree_sitter::Tree {
    let mut parser = tree_sitter::Parser::new();
    parser
        .set_language(&tree_sitter_css::LANGUAGE.into())
        .expect("failed to set CSS language");
    parser.parse(src, None).expect("parse failed")
}

pub fn parse_go(src: &[u8]) -> tree_sitter::Tree {
    let mut parser = tree_sitter::Parser::new();
    parser
        .set_language(&tree_sitter_go::LANGUAGE.into())
        .expect("failed to set Go language");
    parser.parse(src, None).expect("parse failed")
}

pub fn parse_haskell(src: &[u8]) -> tree_sitter::Tree {
    let mut parser = tree_sitter::Parser::new();
    parser
        .set_language(&tree_sitter_haskell::LANGUAGE.into())
        .expect("failed to set Haskell language");
    parser.parse(src, None).expect("parse failed")
}

pub fn parse_html(src: &[u8]) -> tree_sitter::Tree {
    let mut parser = tree_sitter::Parser::new();
    parser
        .set_language(&tree_sitter_html::LANGUAGE.into())
        .expect("failed to set HTML language");
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

pub fn parse_json(src: &[u8]) -> tree_sitter::Tree {
    let mut parser = tree_sitter::Parser::new();
    parser
        .set_language(&tree_sitter_json::LANGUAGE.into())
        .expect("failed to set JSON language");
    parser.parse(src, None).expect("parse failed")
}

pub fn parse_php(src: &[u8]) -> tree_sitter::Tree {
    let mut parser = tree_sitter::Parser::new();
    parser
        .set_language(&tree_sitter_php::LANGUAGE_PHP.into())
        .expect("failed to set PHP language");
    parser.parse(src, None).expect("parse failed")
}

pub fn parse_python(src: &[u8]) -> tree_sitter::Tree {
    let mut parser = tree_sitter::Parser::new();
    parser
        .set_language(&tree_sitter_python::LANGUAGE.into())
        .expect("failed to set Python language");
    parser.parse(src, None).expect("parse failed")
}

pub fn parse_regex(src: &[u8]) -> tree_sitter::Tree {
    let mut parser = tree_sitter::Parser::new();
    parser
        .set_language(&tree_sitter_regex::LANGUAGE.into())
        .expect("failed to set Regex language");
    parser.parse(src, None).expect("parse failed")
}

pub fn parse_ruby(src: &[u8]) -> tree_sitter::Tree {
    let mut parser = tree_sitter::Parser::new();
    parser
        .set_language(&tree_sitter_ruby::LANGUAGE.into())
        .expect("failed to set Ruby language");
    parser.parse(src, None).expect("parse failed")
}

pub fn parse_rust(src: &[u8]) -> tree_sitter::Tree {
    let mut parser = tree_sitter::Parser::new();
    parser
        .set_language(&tree_sitter_rust::LANGUAGE.into())
        .expect("failed to set Rust language");
    parser.parse(src, None).expect("parse failed")
}

pub fn parse_scala(src: &[u8]) -> tree_sitter::Tree {
    let mut parser = tree_sitter::Parser::new();
    parser
        .set_language(&tree_sitter_scala::LANGUAGE.into())
        .expect("failed to set Scala language");
    parser.parse(src, None).expect("parse failed")
}

pub fn parse_typescript(src: &[u8]) -> tree_sitter::Tree {
    let mut parser = tree_sitter::Parser::new();
    parser
        .set_language(&tree_sitter_typescript::LANGUAGE_TYPESCRIPT.into())
        .expect("failed to set TypeScript language");
    parser.parse(src, None).expect("parse failed")
}
