use proc_macro2::Ident;
use quote::format_ident;

/// Keywords that can be escaped with `r#`.
/// Note: `self`, `Self`, `super`, and `crate` cannot be raw identifiers.
const RAW_ESCAPABLE_KEYWORDS: &[&str] = &[
    // Strict keywords
    "as", "async", "await", "break", "const", "continue", "dyn", "else", "enum", "extern", "false",
    "fn", "for", "if", "impl", "in", "let", "loop", "macro", "match", "mod", "move", "mut", "pub",
    "ref", "return", "static", "struct", "trait", "true", "type", "union", "unsafe", "use",
    "where", "while", "yield",
    // Reserved keywords (cannot be used as identifiers without r#)
    "abstract", "become", "box", "do", "final", "macro", "override", "priv", "try", "typeof",
    "unsized", "virtual",
];

/// Keywords that cannot be raw identifiers — must be renamed.
const NON_RAW_KEYWORDS: &[&str] = &["self", "Self", "super", "crate"];

/// Converts a tree-sitter node kind (snake_case) to a PascalCase Rust type name.
pub fn to_pascal_case(s: &str) -> String {
    s.split('_')
        .filter(|part| !part.is_empty())
        .map(|part| {
            let mut chars = part.chars();
            let first = chars.next().unwrap().to_uppercase().to_string();
            first + &chars.as_str().to_lowercase()
        })
        .collect()
}

/// Returns a valid Rust identifier for a field name. Escapes Rust keywords with `r#`,
/// or appends `_` for keywords that can't be raw identifiers.
pub fn field_ident(name: &str) -> Ident {
    if RAW_ESCAPABLE_KEYWORDS.contains(&name) {
        format_ident!("r#{}", name)
    } else if NON_RAW_KEYWORDS.contains(&name) {
        format_ident!("{}_", name)
    } else {
        format_ident!("{}", name)
    }
}

/// Returns a valid Rust identifier for a type name in PascalCase.
pub fn type_ident(kind: &str) -> Ident {
    let pascal = to_pascal_case(kind);
    // Type names in PascalCase generally don't collide with keywords,
    // but handle edge cases like "Self"
    if pascal == "Self" {
        format_ident!("SelfType")
    } else {
        format_ident!("{}", pascal)
    }
}

/// Maps an anonymous/punctuation node kind to a readable Rust identifier name.
pub fn anonymous_node_name(s: &str) -> String {
    match s {
        "." => "Dot".to_owned(),
        "," => "Comma".to_owned(),
        ":" => "Colon".to_owned(),
        "::" => "DoubleColon".to_owned(),
        ";" => "Semicolon".to_owned(),
        "(" => "LParen".to_owned(),
        ")" => "RParen".to_owned(),
        "[" => "LBracket".to_owned(),
        "]" => "RBracket".to_owned(),
        "{" => "LBrace".to_owned(),
        "}" => "RBrace".to_owned(),
        "<" => "Lt".to_owned(),
        ">" => "Gt".to_owned(),
        "<=" => "LtEq".to_owned(),
        ">=" => "GtEq".to_owned(),
        "==" => "EqEq".to_owned(),
        "!=" => "NotEq".to_owned(),
        "=" => "Eq".to_owned(),
        "+" => "Plus".to_owned(),
        "-" => "Minus".to_owned(),
        "*" => "Star".to_owned(),
        "/" => "Slash".to_owned(),
        "%" => "Percent".to_owned(),
        "&" => "Amp".to_owned(),
        "&&" => "AmpAmp".to_owned(),
        "|" => "Pipe".to_owned(),
        "||" => "PipePipe".to_owned(),
        "!" => "Bang".to_owned(),
        "~" => "Tilde".to_owned(),
        "^" => "Caret".to_owned(),
        "?" => "Question".to_owned(),
        "@" => "At".to_owned(),
        "#" => "Hash".to_owned(),
        "$" => "Dollar".to_owned(),
        "_" => "Blank".to_owned(),
        "\"" => "DoubleQuote".to_owned(),
        "'" => "SingleQuote".to_owned(),
        "`" => "Backtick".to_owned(),
        "\\" => "Backslash".to_owned(),
        "->" => "Arrow".to_owned(),
        "<-" => "LArrow".to_owned(),
        "=>" => "FatArrow".to_owned(),
        "+=" => "PlusEq".to_owned(),
        "-=" => "MinusEq".to_owned(),
        "*=" => "StarEq".to_owned(),
        "/=" => "SlashEq".to_owned(),
        "%=" => "PercentEq".to_owned(),
        "&=" => "AmpEq".to_owned(),
        "|=" => "PipeEq".to_owned(),
        "^=" => "CaretEq".to_owned(),
        "<<" => "Shl".to_owned(),
        ">>" => "Shr".to_owned(),
        "<<=" => "ShlEq".to_owned(),
        ">>=" => "ShrEq".to_owned(),
        "++" => "PlusPlus".to_owned(),
        "--" => "MinusMinus".to_owned(),
        "..." => "Ellipsis".to_owned(),
        ".." => "DotDot".to_owned(),
        ":=" => "ColonEq".to_owned(),
        // For keyword-like anonymous nodes (e.g., "if", "else", "for"), PascalCase them
        other => to_pascal_case_fallback(other),
    }
}

/// Fallback for anonymous nodes that are keyword-like strings.
/// Converts to PascalCase, replacing non-alphanumeric chars with descriptive names.
fn to_pascal_case_fallback(s: &str) -> String {
    if s.chars().all(|c| c.is_alphanumeric() || c == '_') {
        let pascal = to_pascal_case(s);
        if pascal.is_empty() {
            "Unknown".to_owned()
        } else {
            pascal
        }
    } else {
        // For remaining unknown punctuation, build a name char by char
        let mut result = String::new();
        for ch in s.chars() {
            match ch {
                '.' => result.push_str("Dot"),
                ',' => result.push_str("Comma"),
                ':' => result.push_str("Colon"),
                ';' => result.push_str("Semi"),
                '(' => result.push_str("LParen"),
                ')' => result.push_str("RParen"),
                '[' => result.push_str("LBracket"),
                ']' => result.push_str("RBracket"),
                '{' => result.push_str("LBrace"),
                '}' => result.push_str("RBrace"),
                '<' => result.push_str("Lt"),
                '>' => result.push_str("Gt"),
                '=' => result.push_str("Eq"),
                '+' => result.push_str("Plus"),
                '-' => result.push_str("Minus"),
                '*' => result.push_str("Star"),
                '/' => result.push_str("Slash"),
                '%' => result.push_str("Percent"),
                '&' => result.push_str("Amp"),
                '|' => result.push_str("Pipe"),
                '!' => result.push_str("Bang"),
                '~' => result.push_str("Tilde"),
                '^' => result.push_str("Caret"),
                '?' => result.push_str("Question"),
                '@' => result.push_str("At"),
                '#' => result.push_str("Hash"),
                '$' => result.push_str("Dollar"),
                '_' => result.push_str("Blank"),
                '"' => result.push_str("DQuote"),
                '\'' => result.push_str("SQuote"),
                '`' => result.push_str("Backtick"),
                '\\' => result.push_str("Backslash"),
                c if c.is_alphanumeric() => {
                    result.push(c.to_uppercase().next().unwrap());
                }
                _ => result.push('X'),
            }
        }
        if result.is_empty() {
            "Unknown".to_owned()
        } else {
            result
        }
    }
}

/// Returns a variant name for use in an enum, given a type reference from node-types.json.
/// Named nodes use the same logic as `type_ident` (handles `Self` → `SelfType` etc.),
/// anonymous nodes get descriptive names.
pub fn variant_name(kind: &str, named: bool) -> Ident {
    if named {
        type_ident(kind)
    } else {
        format_ident!("{}", anonymous_node_name(kind))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_to_pascal_case_simple() {
        assert_eq!(to_pascal_case("import_spec"), "ImportSpec");
        assert_eq!(to_pascal_case("source_file"), "SourceFile");
        assert_eq!(to_pascal_case("identifier"), "Identifier");
    }

    #[test]
    fn test_to_pascal_case_single_word() {
        assert_eq!(to_pascal_case("name"), "Name");
    }

    #[test]
    fn test_to_pascal_case_multiple_underscores() {
        assert_eq!(to_pascal_case("some__double"), "SomeDouble");
        assert_eq!(to_pascal_case("_leading"), "Leading");
        assert_eq!(to_pascal_case("trailing_"), "Trailing");
    }

    #[test]
    fn test_field_ident_normal() {
        assert_eq!(field_ident("name").to_string(), "name");
        assert_eq!(field_ident("path").to_string(), "path");
    }

    #[test]
    fn test_field_ident_keyword_escaping() {
        assert_eq!(field_ident("type").to_string(), "r#type");
        assert_eq!(field_ident("if").to_string(), "r#if");
        assert_eq!(field_ident("async").to_string(), "r#async");
    }

    #[test]
    fn test_field_ident_non_raw_keywords() {
        // self/Self/super/crate can't be raw idents, so they get a trailing underscore
        assert_eq!(field_ident("self").to_string(), "self_");
        assert_eq!(field_ident("Self").to_string(), "Self_");
        assert_eq!(field_ident("super").to_string(), "super_");
        assert_eq!(field_ident("crate").to_string(), "crate_");
    }

    #[test]
    fn test_type_ident() {
        assert_eq!(type_ident("import_spec").to_string(), "ImportSpec");
        assert_eq!(type_ident("identifier").to_string(), "Identifier");
    }

    #[test]
    fn test_type_ident_self_edge_case() {
        assert_eq!(type_ident("self").to_string(), "SelfType");
    }

    #[test]
    fn test_anonymous_node_name_punctuation() {
        assert_eq!(anonymous_node_name("."), "Dot");
        assert_eq!(anonymous_node_name("*"), "Star");
        assert_eq!(anonymous_node_name("+"), "Plus");
        assert_eq!(anonymous_node_name("->"), "Arrow");
        assert_eq!(anonymous_node_name("<-"), "LArrow");
        assert_eq!(anonymous_node_name("_"), "Blank");
        assert_eq!(anonymous_node_name("..."), "Ellipsis");
        assert_eq!(anonymous_node_name(":="), "ColonEq");
    }

    #[test]
    fn test_anonymous_node_name_keywords() {
        assert_eq!(anonymous_node_name("if"), "If");
        assert_eq!(anonymous_node_name("else"), "Else");
        assert_eq!(anonymous_node_name("for"), "For");
        assert_eq!(anonymous_node_name("func"), "Func");
        assert_eq!(anonymous_node_name("package"), "Package");
    }

    #[test]
    fn test_variant_name_named() {
        assert_eq!(variant_name("identifier", true).to_string(), "Identifier");
        assert_eq!(variant_name("import_spec", true).to_string(), "ImportSpec");
    }

    #[test]
    fn test_variant_name_anonymous() {
        assert_eq!(variant_name(".", false).to_string(), "Dot");
        assert_eq!(variant_name("*", false).to_string(), "Star");
        assert_eq!(variant_name("if", false).to_string(), "If");
    }
}
