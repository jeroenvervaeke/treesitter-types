//! Strongly-typed AST types for Scala, generated from tree-sitter-scala's `node-types.json`.

pub use treesitter_types::{FromNode, LeafNode, ParseError, Span, Spanned};

mod generated;
pub use generated::*;
