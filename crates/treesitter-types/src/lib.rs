//! Core library for generating strongly-typed Rust AST types from
//! [Tree-sitter](https://tree-sitter.github.io/tree-sitter/) grammars.
//!
//! This crate reads a tree-sitter `node-types.json` file and generates Rust structs and enums
//! that map one-to-one to the grammar's node types. It also provides the runtime traits
//! ([`FromNode`], [`LeafNode`], [`Spanned`]) needed to convert `tree_sitter::Node` values into
//! those typed representations.
//!
//! # Usage
//!
//! There are three ways to use the generated types:
//!
//! 1. **Pre-generated crates** — ready-made crates for popular languages (e.g.
//!    [`treesitter-types-python`](https://docs.rs/treesitter-types-python),
//!    [`treesitter-types-rust`](https://docs.rs/treesitter-types-rust)).
//! 2. **Proc-macro** — use [`treesitter-types-macros`](https://docs.rs/treesitter-types-macros)
//!    to generate types at compile time from your own `node-types.json`.
//! 3. **CLI / library** — call [`codegen::generate`] or [`codegen::generate_to_string`] directly.

pub mod codegen;
pub mod runtime;

pub use runtime::{maybe_grow_stack, FromNode, LeafNode, ParseError, Span, Spanned};
