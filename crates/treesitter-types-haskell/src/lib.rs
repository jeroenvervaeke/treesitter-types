//! Strongly-typed AST types for Haskell, generated from tree-sitter-haskell's `node-types.json`.

pub use treesitter_types::{FromNode, LeafNode, ParseError, Span, Spanned};

mod generated;
pub use generated::*;
