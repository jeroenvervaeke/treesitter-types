//! Strongly-typed AST types for Swift, generated from tree-sitter-swift's `node-types.json`.

pub use treesitter_types::{FromNode, LeafNode, ParseError, Span, Spanned};

mod generated;
pub use generated::*;
