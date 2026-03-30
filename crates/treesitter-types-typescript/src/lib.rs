//! Strongly-typed AST types for TypeScript, generated from tree-sitter-typescript's `node-types.json`.

pub use treesitter_types::{FromNode, LeafNode, ParseError, Span, Spanned};

mod generated;
pub use generated::*;
