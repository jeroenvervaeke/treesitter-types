use std::path::Path;

/// Each entry: (language name, NODE_TYPES json, output path relative to workspace root)
fn languages() -> Vec<(&'static str, &'static str, &'static str)> {
    vec![
        (
            "c",
            tree_sitter_c::NODE_TYPES,
            "crates/treesitter-types-c/src/generated.rs",
        ),
        (
            "c-sharp",
            tree_sitter_c_sharp::NODE_TYPES,
            "crates/treesitter-types-c-sharp/src/generated.rs",
        ),
        (
            "cpp",
            tree_sitter_cpp::NODE_TYPES,
            "crates/treesitter-types-cpp/src/generated.rs",
        ),
        (
            "css",
            tree_sitter_css::NODE_TYPES,
            "crates/treesitter-types-css/src/generated.rs",
        ),
        (
            "elixir",
            tree_sitter_elixir::NODE_TYPES,
            "crates/treesitter-types-elixir/src/generated.rs",
        ),
        (
            "go",
            tree_sitter_go::NODE_TYPES,
            "crates/treesitter-types-go/src/generated.rs",
        ),
        (
            "haskell",
            tree_sitter_haskell::NODE_TYPES,
            "crates/treesitter-types-haskell/src/generated.rs",
        ),
        (
            "html",
            tree_sitter_html::NODE_TYPES,
            "crates/treesitter-types-html/src/generated.rs",
        ),
        (
            "java",
            tree_sitter_java::NODE_TYPES,
            "crates/treesitter-types-java/src/generated.rs",
        ),
        (
            "javascript",
            tree_sitter_javascript::NODE_TYPES,
            "crates/treesitter-types-javascript/src/generated.rs",
        ),
        (
            "json",
            tree_sitter_json::NODE_TYPES,
            "crates/treesitter-types-json/src/generated.rs",
        ),
        (
            "lua",
            tree_sitter_lua::NODE_TYPES,
            "crates/treesitter-types-lua/src/generated.rs",
        ),
        (
            "markdown",
            tree_sitter_md::NODE_TYPES_BLOCK,
            "crates/treesitter-types-markdown/src/generated.rs",
        ),
        (
            "php",
            tree_sitter_php::PHP_NODE_TYPES,
            "crates/treesitter-types-php/src/generated.rs",
        ),
        (
            "python",
            tree_sitter_python::NODE_TYPES,
            "crates/treesitter-types-python/src/generated.rs",
        ),
        (
            "regex",
            tree_sitter_regex::NODE_TYPES,
            "crates/treesitter-types-regex/src/generated.rs",
        ),
        (
            "ruby",
            tree_sitter_ruby::NODE_TYPES,
            "crates/treesitter-types-ruby/src/generated.rs",
        ),
        (
            "rust",
            tree_sitter_rust::NODE_TYPES,
            "crates/treesitter-types-rust/src/generated.rs",
        ),
        (
            "scala",
            tree_sitter_scala::NODE_TYPES,
            "crates/treesitter-types-scala/src/generated.rs",
        ),
        (
            "swift",
            tree_sitter_swift::NODE_TYPES,
            "crates/treesitter-types-swift/src/generated.rs",
        ),
        (
            "toml",
            tree_sitter_toml_ng::NODE_TYPES,
            "crates/treesitter-types-toml/src/generated.rs",
        ),
        (
            "typescript",
            tree_sitter_typescript::TYPESCRIPT_NODE_TYPES,
            "crates/treesitter-types-typescript/src/generated.rs",
        ),
        (
            "yaml",
            tree_sitter_yaml::NODE_TYPES,
            "crates/treesitter-types-yaml/src/generated.rs",
        ),
    ]
}

fn main() {
    let mut failed = Vec::new();

    for (lang, node_types_json, output_path) in languages() {
        eprint!("Generating {lang}... ");
        match treesitter_types::codegen::generate_to_string(node_types_json) {
            Ok(code) => {
                let path = Path::new(output_path);
                if let Some(parent) = path.parent() {
                    std::fs::create_dir_all(parent).unwrap();
                }
                std::fs::write(path, code).unwrap();
                eprintln!("OK");
            }
            Err(e) => {
                eprintln!("FAILED: {e}");
                failed.push(lang);
            }
        }
    }

    if !failed.is_empty() {
        eprintln!("\nFailed languages: {}", failed.join(", "));
        std::process::exit(1);
    }

    eprintln!(
        "\nAll {} languages generated successfully.",
        languages().len()
    );
}
