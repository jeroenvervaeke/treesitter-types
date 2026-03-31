//! Strongly-typed AST types for Lua, generated from tree-sitter-lua's `node-types.json`.

pub use treesitter_types::{FromNode, LeafNode, ParseError, Span, Spanned};

mod generated;
pub use generated::*;
