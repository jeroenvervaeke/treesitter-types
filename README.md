# treesitter-types

Generate fully-typed Rust structs and enums from any tree-sitter grammar's `node-types.json`.

Instead of stringly-typed access like `node.kind() == "function_declaration"`, get compile-time checked types with proper field access, `Option<T>` for optional fields, `Vec<T>` for repeated fields, and enums for alternations.

## Usage

### Via `build.rs` (recommended)

Add to your `Cargo.toml`:

```toml
[dependencies]
treesitter-types = "0.1"
tree-sitter = "0.25"

[build-dependencies]
treesitter-types = "0.1"
```

Create a `build.rs`:

```rust
fn main() {
    treesitter_types::codegen::emit_to_out_dir("path/to/node-types.json").unwrap();
}
```

Include the generated code in `lib.rs`:

```rust
include!(concat!(env!("OUT_DIR"), "/treesitter_types_generated.rs"));
```

### Via proc-macro

```toml
[dependencies]
treesitter-types = "0.1"
treesitter-types-macros = "0.1"
tree-sitter = "0.25"
```

```rust
treesitter_types_macros::generate_types!("path/to/node-types.json");
```

## Example

Given tree-sitter-go's `node-types.json`, the generated types let you write:

```rust
use treesitter_types::{FromNode, LeafNode};

let tree = parser.parse(src, None).unwrap();
let source_file = SourceFile::from_node(tree.root_node(), src).unwrap();

for child in &source_file.children {
    match child {
        SourceFileChildren::FunctionDeclaration(func) => {
            println!("fn {}", func.name.text());
        }
        SourceFileChildren::PackageClause(pkg) => {
            println!("package {}", pkg.children.text());
        }
        _ => {}
    }
}
```

## What gets generated

| Grammar construct | Rust type |
|---|---|
| Named node with fields | `struct` with typed fields |
| Named terminal node | `struct` with `LeafNode::text()` |
| Supertype node (`_expression`) | `enum` with variant per subtype |
| Field with multiple types | Alternation `enum` |
| Optional field | `Option<T>` |
| Repeated field | `Vec<T>` |
| All named nodes | `AnyNode` top-level enum |

## Runtime traits

Every generated type implements:

- **`FromNode<'tree>`** - construct from a `tree_sitter::Node`
- **`Spanned`** - access source location (`span()`, `start()`, `end()`)
- **`LeafNode<'tree>`** (terminal nodes only) - access source text via `text()`

## Pre-generated language crates

Ready-to-use typed AST crates for 25 languages — no codegen step needed:

| Crate | Grammar |
|---|---|
| `treesitter-types-bash` | tree-sitter-bash |
| `treesitter-types-c` | tree-sitter-c |
| `treesitter-types-c-sharp` | tree-sitter-c-sharp |
| `treesitter-types-cpp` | tree-sitter-cpp |
| `treesitter-types-css` | tree-sitter-css |
| `treesitter-types-elixir` | tree-sitter-elixir |
| `treesitter-types-go` | tree-sitter-go |
| `treesitter-types-html` | tree-sitter-html |
| `treesitter-types-java` | tree-sitter-java |
| `treesitter-types-javascript` | tree-sitter-javascript |
| `treesitter-types-json` | tree-sitter-json |
| `treesitter-types-lua` | tree-sitter-lua |
| `treesitter-types-markdown` | tree-sitter-md |
| `treesitter-types-php` | tree-sitter-php |
| `treesitter-types-python` | tree-sitter-python |
| `treesitter-types-regex` | tree-sitter-regex |
| `treesitter-types-ruby` | tree-sitter-ruby |
| `treesitter-types-rust` | tree-sitter-rust |
| `treesitter-types-scala` | tree-sitter-scala |
| `treesitter-types-swift` | tree-sitter-swift |
| `treesitter-types-toml` | tree-sitter-toml-ng |
| `treesitter-types-typescript` | tree-sitter-typescript |
| `treesitter-types-yaml` | tree-sitter-yaml |

```toml
[dependencies]
treesitter-types-python = "0.1"
tree-sitter = "0.25"
```

## Tested against

The codegen is validated against real-world repositories with zero failures on cleanly-parsed files:

| Repository | Language | Files | Result |
|---|---|---|---|
| [golang/go](https://github.com/golang/go) | Go | 5,670 | 100% |
| [rust-lang/rust](https://github.com/rust-lang/rust) | Rust | 11,109 | 100% |
| [microsoft/TypeScript](https://github.com/microsoft/TypeScript) | TypeScript | 709 | 100% |
| [nodejs/node](https://github.com/nodejs/node) | JavaScript | 356 | 100% |
| [spring-projects/spring-framework](https://github.com/spring-projects/spring-framework) | Java | 4,715 | 100% |
| [python/cpython](https://github.com/python/cpython) | Python | - | - |
| [git/git](https://github.com/git/git) | C | - | - |
| [nlohmann/json](https://github.com/nlohmann/json) | C++ | - | - |
| [ohmyzsh/ohmyzsh](https://github.com/ohmyzsh/ohmyzsh) | Bash | - | - |
| [rails/rails](https://github.com/rails/rails) | Ruby | - | - |
| [dotnet/runtime](https://github.com/dotnet/runtime) | C# | - | - |
| [twbs/bootstrap](https://github.com/twbs/bootstrap) | CSS | - | - |
| [laravel/framework](https://github.com/laravel/framework) | PHP | - | - |
| [SchemaStore/schemastore](https://github.com/SchemaStore/schemastore) | JSON | - | - |
| [apache/spark](https://github.com/apache/spark) | Scala | - | - |

Run the integration tests yourself:

```bash
just integration-test-go
just integration-test-python
just integration-test-all    # run all languages
```

## License

Licensed under either of [Apache License, Version 2.0](LICENSE-APACHE) or [MIT License](LICENSE-MIT) at your option.
