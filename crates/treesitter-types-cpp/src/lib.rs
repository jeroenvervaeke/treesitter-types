//! Strongly-typed AST types for C++, generated from tree-sitter-cpp's `node-types.json`.

pub use treesitter_types::{FromNode, LeafNode, ParseError, Span, Spanned};

mod generated;
pub use generated::*;
