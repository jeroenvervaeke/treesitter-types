//! Strongly-typed AST types for CSS, generated from tree-sitter-css's `node-types.json`.

pub use treesitter_types::{FromNode, LeafNode, ParseError, Span, Spanned};

mod generated;
pub use generated::*;
