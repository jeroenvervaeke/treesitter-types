//! Strongly-typed AST types for JavaScript, generated from tree-sitter-javascript's `node-types.json`.

pub use treesitter_types::{FromNode, LeafNode, ParseError, Span, Spanned};

mod generated;
pub use generated::*;
