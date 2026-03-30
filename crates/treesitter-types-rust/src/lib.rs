//! Strongly-typed AST types for Rust, generated from tree-sitter-rust's `node-types.json`.

pub use treesitter_types::{FromNode, LeafNode, ParseError, Span, Spanned};

mod generated;
pub use generated::*;
