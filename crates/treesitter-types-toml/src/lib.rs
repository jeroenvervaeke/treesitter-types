//! Strongly-typed AST types for TOML, generated from tree-sitter-toml-ng's `node-types.json`.

pub use treesitter_types::{FromNode, LeafNode, ParseError, Span, Spanned};

mod generated;
pub use generated::*;
