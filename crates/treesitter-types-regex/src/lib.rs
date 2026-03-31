//! Strongly-typed AST types for Regex, generated from tree-sitter-regex's `node-types.json`.

pub use treesitter_types::{FromNode, LeafNode, ParseError, Span, Spanned};

mod generated;
pub use generated::*;
