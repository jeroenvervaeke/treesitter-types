//! Strongly-typed AST types for Go, generated from tree-sitter-go's `node-types.json`.

pub use treesitter_types::{FromNode, LeafNode, ParseError, Span, Spanned};

mod generated;
pub use generated::*;
