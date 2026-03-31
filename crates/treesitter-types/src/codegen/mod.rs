pub mod emitter;
pub mod grammar_ir;
pub mod name_mangler;
pub mod type_mapper;

use proc_macro2::TokenStream;
use std::path::Path;

/// Generates a `TokenStream` of typed AST definitions from the contents of a `node-types.json`.
pub fn generate(node_types_json: &str) -> Result<TokenStream, Error> {
    let nodes = grammar_ir::parse_node_types(node_types_json)?;
    let decisions = type_mapper::map_types(&nodes);
    Ok(emitter::emit(&decisions))
}

/// Formats a `TokenStream` into pretty-printed Rust source code.
pub fn format(tokens: &TokenStream) -> Result<String, Error> {
    let file = syn::parse2(tokens.clone()).map_err(Error::Syn)?;
    Ok(prettyplease::unparse(&file))
}

/// Generates typed AST code from `node-types.json` contents and returns it as formatted Rust source.
pub fn generate_to_string(node_types_json: &str) -> Result<String, Error> {
    let tokens = generate(node_types_json)?;
    format(&tokens)
}

/// Reads a `node-types.json` file, generates typed AST code, and writes it to `$OUT_DIR`.
///
/// The generated code is formatted with `prettyplease` for readability.
///
/// Intended for use in `build.rs`:
/// ```no_run
/// treesitter_types::codegen::emit_to_out_dir("path/to/node-types.json").unwrap();
/// ```
///
/// Then in `lib.rs`:
/// ```ignore
/// include!(concat!(env!("OUT_DIR"), "/treesitter_types_generated.rs"));
/// ```
pub fn emit_to_out_dir(node_types_path: impl AsRef<Path>) -> Result<(), Error> {
    let node_types_path = node_types_path.as_ref();
    let json = std::fs::read_to_string(node_types_path)
        .map_err(|e| Error::Io(node_types_path.to_path_buf(), e))?;

    emit_str_to_out_dir(&json)?;

    // Tell Cargo to re-run if the input changes
    println!("cargo:rerun-if-changed={}", node_types_path.display());

    Ok(())
}

/// Generates typed AST code from a `node-types.json` string and writes it to `$OUT_DIR`.
///
/// This is useful when the JSON is available as a constant (e.g., from a grammar crate's
/// `NODE_TYPES` constant) rather than a file path.
///
/// Intended for use in `build.rs`:
/// ```ignore
/// treesitter_types::codegen::emit_str_to_out_dir(tree_sitter_go::NODE_TYPES).unwrap();
/// ```
///
/// Then in `lib.rs`:
/// ```ignore
/// include!(concat!(env!("OUT_DIR"), "/treesitter_types_generated.rs"));
/// ```
pub fn emit_str_to_out_dir(node_types_json: &str) -> Result<(), Error> {
    let tokens = generate(node_types_json)?;
    let code = format(&tokens)?;

    let out_dir = std::env::var("OUT_DIR").map_err(|_| Error::NoOutDir)?;
    let out_path = Path::new(&out_dir).join("treesitter_types_generated.rs");
    std::fs::write(&out_path, code).map_err(|e| Error::Io(out_path, e))?;

    Ok(())
}

#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error("failed to parse node-types.json: {0}")]
    Json(#[from] serde_json::Error),

    #[error("generated code is not valid Rust syntax: {0}")]
    Syn(syn::Error),

    #[error("I/O error on {0}: {1}")]
    Io(std::path::PathBuf, #[source] std::io::Error),

    #[error("OUT_DIR environment variable not set (are you running from build.rs?)")]
    NoOutDir,
}
