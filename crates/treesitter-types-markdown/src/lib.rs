//! Strongly-typed AST types for Markdown, generated from tree-sitter-md's `node-types.json`.

pub use treesitter_types::{FromNode, LeafNode, ParseError, Span, Spanned};

mod generated;
pub use generated::*;
