//! Strongly-typed AST types for OCaml, generated from tree-sitter-ocaml's `node-types.json`.

pub use treesitter_types::{FromNode, LeafNode, ParseError, Span, Spanned};

mod generated;
pub use generated::*;
