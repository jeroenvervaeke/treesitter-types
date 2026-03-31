//! Proc-macro companion for [`treesitter-types`](https://docs.rs/treesitter-types).
//!
//! Provides the [`generate_types!`] macro, which reads a
//! [Tree-sitter](https://tree-sitter.github.io/tree-sitter/) `node-types.json` file at compile
//! time and expands it into strongly-typed Rust AST structs and enums.
//!
//! If you don't need compile-time generation, consider using one of the pre-generated language
//! crates (e.g. [`treesitter-types-go`](https://docs.rs/treesitter-types-go)) instead.

use proc_macro::TokenStream;

/// Generates typed AST structs and enums from a tree-sitter `node-types.json` file.
///
/// The path is resolved relative to the crate root (same semantics as `include_str!`).
///
/// # Example
///
/// ```ignore
/// treesitter_types_macros::generate_types!("src/node-types.json");
/// ```
#[proc_macro]
pub fn generate_types(input: TokenStream) -> TokenStream {
    let input: proc_macro2::TokenStream = input.into();
    match generate_types_impl(input) {
        Ok(tokens) => tokens.into(),
        Err(err) => err.into_compile_error().into(),
    }
}

fn generate_types_impl(
    input: proc_macro2::TokenStream,
) -> Result<proc_macro2::TokenStream, syn::Error> {
    let lit: syn::LitStr = syn::parse2(input)?;
    let rel_path = lit.value();

    // Resolve relative to CARGO_MANIFEST_DIR (the calling crate's root)
    let manifest_dir = std::env::var("CARGO_MANIFEST_DIR")
        .map_err(|_| syn::Error::new(lit.span(), "CARGO_MANIFEST_DIR not set"))?;
    let full_path = std::path::Path::new(&manifest_dir).join(&rel_path);

    let json = std::fs::read_to_string(&full_path).map_err(|e| {
        syn::Error::new(
            lit.span(),
            format!("failed to read {}: {e}", full_path.display()),
        )
    })?;

    treesitter_types::codegen::generate(&json)
        .map_err(|e| syn::Error::new(lit.span(), format!("codegen failed: {e}")))
}
