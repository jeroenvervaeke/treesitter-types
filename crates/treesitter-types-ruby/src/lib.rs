//! Strongly-typed AST types for Ruby, generated from tree-sitter-ruby's `node-types.json`.

pub use treesitter_types::{FromNode, LeafNode, ParseError, Span, Spanned};

mod generated;
pub use generated::*;
