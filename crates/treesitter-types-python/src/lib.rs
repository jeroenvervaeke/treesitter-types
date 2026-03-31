//! Strongly-typed AST types for Python, generated from tree-sitter-python's `node-types.json`.

pub use treesitter_types::{FromNode, LeafNode, ParseError, Span, Spanned};

mod generated;
pub use generated::*;
