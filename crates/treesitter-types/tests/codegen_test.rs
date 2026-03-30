use treesitter_types::codegen;

const SIMPLE_NODE_TYPES: &str = include_str!("fixtures/simple_node_types.json");
const JSON_NODE_TYPES: &str = include_str!("fixtures/json_node_types.json");
const GO_NODE_TYPES: &str = include_str!("fixtures/go_node_types.json");

#[test]
fn test_generate_produces_valid_token_stream() {
    let tokens = codegen::generate(SIMPLE_NODE_TYPES).unwrap();
    let code = tokens.to_string();

    // Should contain all expected struct/enum definitions
    assert!(
        code.contains("pub struct Identifier"),
        "missing Identifier struct"
    );
    assert!(
        code.contains("pub struct InterpretedStringLiteral"),
        "missing InterpretedStringLiteral struct"
    );
    assert!(
        code.contains("pub struct ImportSpec"),
        "missing ImportSpec struct"
    );
    assert!(
        code.contains("pub struct ImportSpecList"),
        "missing ImportSpecList struct"
    );
    assert!(code.contains("pub struct Block"), "missing Block struct");
    assert!(
        code.contains("pub struct ExpressionStatement"),
        "missing ExpressionStatement struct"
    );

    // Should contain the alternation enum for import_spec.name
    assert!(
        code.contains("pub enum ImportSpecName"),
        "missing ImportSpecName alternation enum"
    );

    // Should contain the supertype enum
    assert!(
        code.contains("pub enum Expression"),
        "missing Expression supertype enum"
    );

    // Should contain AnyNode
    assert!(code.contains("pub enum AnyNode"), "missing AnyNode enum");
}

#[test]
fn test_generate_contains_from_node_impls() {
    let tokens = codegen::generate(SIMPLE_NODE_TYPES).unwrap();
    let code = tokens.to_string();

    // FromNode impl for struct
    assert!(
        code.contains("FromNode < 'tree > for ImportSpec"),
        "missing FromNode impl for ImportSpec"
    );

    // FromNode impl for leaf
    assert!(
        code.contains("FromNode < 'tree > for Identifier"),
        "missing FromNode impl for Identifier"
    );

    // LeafNode impl
    assert!(
        code.contains("LeafNode < 'tree > for Identifier"),
        "missing LeafNode impl for Identifier"
    );

    // Spanned impl
    assert!(
        code.contains("Spanned for ImportSpec"),
        "missing Spanned impl for ImportSpec"
    );
}

#[test]
fn test_generate_optional_field_uses_option() {
    let tokens = codegen::generate(SIMPLE_NODE_TYPES).unwrap();
    let code = tokens.to_string();

    // The `name` field on ImportSpec should be Option<ImportSpecName>
    assert!(
        code.contains("Option < ImportSpecName < 'tree > >"),
        "optional field should be wrapped in Option"
    );
}

#[test]
fn test_generate_repeated_field_uses_vec() {
    let tokens = codegen::generate(SIMPLE_NODE_TYPES).unwrap();
    let code = tokens.to_string();

    // The `statements` field on Block should be Vec
    assert!(
        code.contains("Vec < ExpressionStatement < 'tree > >"),
        "repeated field should be wrapped in Vec"
    );
}

#[test]
fn test_generate_children_field() {
    let tokens = codegen::generate(SIMPLE_NODE_TYPES).unwrap();
    let code = tokens.to_string();

    // ImportSpecList should have a children field
    assert!(
        code.contains("pub children"),
        "struct with children should have a children field"
    );
}

#[test]
fn test_generate_errors_on_invalid_json() {
    let result = codegen::generate("not valid json");
    assert!(result.is_err());
}

#[test]
fn test_generated_code_parses_as_rust() {
    let tokens = codegen::generate(SIMPLE_NODE_TYPES).unwrap();
    let code = tokens.to_string();

    // Verify the generated code is valid Rust syntax by parsing it
    let result = syn::parse_file(&code);
    assert!(
        result.is_ok(),
        "generated code is not valid Rust syntax: {:?}\n\nCode:\n{code}",
        result.err()
    );
}

// --- Real grammar tests ---

#[test]
fn test_json_grammar_generates_valid_rust() {
    let tokens = codegen::generate(JSON_NODE_TYPES).unwrap();
    let code = tokens.to_string();

    let result = syn::parse_file(&code);
    assert!(
        result.is_ok(),
        "JSON grammar generated invalid Rust: {:?}\n\nCode:\n{code}",
        result.err()
    );

    // Spot-check expected types from JSON grammar
    assert!(
        code.contains("pub struct Document"),
        "missing Document struct"
    );
    assert!(code.contains("pub struct Array"), "missing Array struct");
    assert!(code.contains("pub struct Object"), "missing Object struct");
    assert!(code.contains("pub struct Pair"), "missing Pair struct");
    assert!(code.contains("pub enum AnyNode"), "missing AnyNode enum");
}

#[test]
fn test_go_grammar_generates_valid_rust() {
    let tokens = codegen::generate(GO_NODE_TYPES).unwrap();
    let code = tokens.to_string();

    let result = syn::parse_file(&code);
    assert!(
        result.is_ok(),
        "Go grammar generated invalid Rust: {:?}\n\nCode:\n{code}",
        result.err()
    );

    // Spot-check expected types from Go grammar
    assert!(
        code.contains("pub struct SourceFile"),
        "missing SourceFile struct"
    );
    assert!(
        code.contains("pub struct ImportSpec"),
        "missing ImportSpec struct"
    );
    assert!(
        code.contains("pub struct FunctionDeclaration"),
        "missing FunctionDeclaration struct"
    );
    assert!(code.contains("pub enum AnyNode"), "missing AnyNode enum");
}
