//! Strongly-typed AST types for JSON, generated from tree-sitter-json's `node-types.json`.

pub use treesitter_types::{FromNode, LeafNode, ParseError, Span, Spanned};

mod generated;
pub use generated::*;
