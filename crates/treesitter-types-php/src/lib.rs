//! Strongly-typed AST types for PHP, generated from tree-sitter-php's `node-types.json`.

pub use treesitter_types::{FromNode, LeafNode, ParseError, Span, Spanned};

mod generated;
pub use generated::*;
