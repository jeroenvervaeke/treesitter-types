//! Command-line tool for generating strongly-typed Rust AST types from a
//! [Tree-sitter](https://tree-sitter.github.io/tree-sitter/) `node-types.json` file.
//!
//! This is a thin wrapper around the [`treesitter-types`](https://docs.rs/treesitter-types)
//! code-generation library. It reads a `node-types.json` file and prints the generated Rust code
//! to stdout.
//!
//! # Usage
//!
//! ```sh
//! treesitter-types-cli path/to/node-types.json > generated.rs
//! ```

use std::path::PathBuf;

fn main() {
    let path = std::env::args()
        .nth(1)
        .map(PathBuf::from)
        .unwrap_or_else(|| {
            eprintln!("usage: treesitter-types-cli <node-types.json>");
            std::process::exit(1);
        });

    let json = std::fs::read_to_string(&path).unwrap_or_else(|e| {
        eprintln!("error: failed to read {}: {e}", path.display());
        std::process::exit(1);
    });

    let code = treesitter_types::codegen::generate_to_string(&json).unwrap_or_else(|e| {
        eprintln!("error: codegen failed: {e}");
        std::process::exit(1);
    });

    print!("{code}");
}
