//! Strongly-typed AST types for Elixir, generated from tree-sitter-elixir's `node-types.json`.

pub use treesitter_types::{FromNode, LeafNode, ParseError, Span, Spanned};

mod generated;
pub use generated::*;
