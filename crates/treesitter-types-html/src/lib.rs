//! Strongly-typed AST types for HTML, generated from tree-sitter-html's `node-types.json`.

pub use treesitter_types::{FromNode, LeafNode, ParseError, Span, Spanned};

mod generated;
pub use generated::*;
