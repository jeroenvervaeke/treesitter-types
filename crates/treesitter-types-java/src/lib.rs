//! Strongly-typed AST types for Java, generated from tree-sitter-java's `node-types.json`.

pub use treesitter_types::{FromNode, LeafNode, ParseError, Span, Spanned};

mod generated;
pub use generated::*;
