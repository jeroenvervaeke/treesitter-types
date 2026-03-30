# Generate typed AST code for all language crates
generate:
    cargo build -p treesitter-types-cli
    ./target/debug/treesitter-types-cli crates/treesitter-types-go/node-types.json > crates/treesitter-types-go/src/generated.rs
    ./target/debug/treesitter-types-cli crates/treesitter-types-java/node-types.json > crates/treesitter-types-java/src/generated.rs
    ./target/debug/treesitter-types-cli crates/treesitter-types-javascript/node-types.json > crates/treesitter-types-javascript/src/generated.rs
    ./target/debug/treesitter-types-cli crates/treesitter-types-rust/node-types.json > crates/treesitter-types-rust/src/generated.rs
    ./target/debug/treesitter-types-cli crates/treesitter-types-typescript/node-types.json > crates/treesitter-types-typescript/src/generated.rs
    cargo fmt --all

# Run integration test: parse an entire real-world Go repository
integration-test-go:
    #!/usr/bin/env bash
    set -euo pipefail
    REPO_URL="https://github.com/golang/go.git"
    PARSE_DIR="src"
    TMPDIR=$(mktemp -d)
    trap 'rm -rf "$TMPDIR"' EXIT
    echo "==> Cloning Go repository (shallow, no blobs until checkout)..."
    git clone --depth 1 --filter=blob:none --sparse "$REPO_URL" "$TMPDIR/go" 2>&1 | tail -1
    cd "$TMPDIR/go"
    git sparse-checkout set "$PARSE_DIR" 2>/dev/null
    cd - > /dev/null
    FILE_COUNT=$(find "$TMPDIR/go/$PARSE_DIR" -name '*.go' -not -path '*/testdata/*' -not -path '*/vendor/*' | wc -l)
    echo "==> Found $FILE_COUNT .go files in $PARSE_DIR/"
    echo "==> Building parse_all_go..."
    cargo build --release -p test-roundtrip --bin parse_all_go 2>&1 | tail -1
    echo "==> Parsing all .go files..."
    echo
    ./target/release/parse_all_go "$TMPDIR/go/$PARSE_DIR"

# Run integration test: parse an entire real-world Rust repository
integration-test-rust:
    #!/usr/bin/env bash
    set -euo pipefail
    REPO_URL="https://github.com/rust-lang/rust.git"
    TMPDIR=$(mktemp -d)
    trap 'rm -rf "$TMPDIR"' EXIT
    echo "==> Cloning rust-lang/rust (shallow, sparse checkout)..."
    git clone --depth 1 --filter=blob:none --sparse "$REPO_URL" "$TMPDIR/rust" 2>&1 | tail -1
    cd "$TMPDIR/rust"
    git sparse-checkout set compiler library src/tools 2>/dev/null
    cd - > /dev/null
    FILE_COUNT=$(find "$TMPDIR/rust" -name '*.rs' -not -path '*/testdata/*' -not -path '*/vendor/*' | wc -l)
    echo "==> Found $FILE_COUNT .rs files"
    echo "==> Building parse_all_rust..."
    cargo build --release -p test-roundtrip --bin parse_all_rust 2>&1 | tail -1
    echo "==> Parsing all .rs files..."
    echo
    ./target/release/parse_all_rust "$TMPDIR/rust"

# Run integration test: parse an entire real-world TypeScript repository
integration-test-typescript:
    #!/usr/bin/env bash
    set -euo pipefail
    REPO_URL="https://github.com/microsoft/TypeScript.git"
    TMPDIR=$(mktemp -d)
    trap 'rm -rf "$TMPDIR"' EXIT
    echo "==> Cloning microsoft/TypeScript (shallow, sparse checkout)..."
    git clone --depth 1 --filter=blob:none --sparse "$REPO_URL" "$TMPDIR/typescript" 2>&1 | tail -1
    cd "$TMPDIR/typescript"
    git sparse-checkout set src 2>/dev/null
    cd - > /dev/null
    FILE_COUNT=$(find "$TMPDIR/typescript/src" -name '*.ts' -not -path '*/node_modules/*' | wc -l)
    echo "==> Found $FILE_COUNT .ts files in src/"
    echo "==> Building parse_all_typescript..."
    cargo build --release -p test-roundtrip --bin parse_all_typescript 2>&1 | tail -1
    echo "==> Parsing all .ts files..."
    echo
    ./target/release/parse_all_typescript "$TMPDIR/typescript/src"

# Run integration test: parse an entire real-world JavaScript repository
integration-test-javascript:
    #!/usr/bin/env bash
    set -euo pipefail
    REPO_URL="https://github.com/nodejs/node.git"
    TMPDIR=$(mktemp -d)
    trap 'rm -rf "$TMPDIR"' EXIT
    echo "==> Cloning nodejs/node (shallow, sparse checkout)..."
    git clone --depth 1 --filter=blob:none --sparse "$REPO_URL" "$TMPDIR/node" 2>&1 | tail -1
    cd "$TMPDIR/node"
    git sparse-checkout set lib 2>/dev/null
    cd - > /dev/null
    FILE_COUNT=$(find "$TMPDIR/node/lib" -name '*.js' -not -path '*/node_modules/*' | wc -l)
    echo "==> Found $FILE_COUNT .js files in lib/"
    echo "==> Building parse_all_javascript..."
    cargo build --release -p test-roundtrip --bin parse_all_javascript 2>&1 | tail -1
    echo "==> Parsing all .js files..."
    echo
    ./target/release/parse_all_javascript "$TMPDIR/node/lib"

# Run integration test: parse an entire real-world Java repository
integration-test-java:
    #!/usr/bin/env bash
    set -euo pipefail
    REPO_URL="https://github.com/spring-projects/spring-framework.git"
    TMPDIR=$(mktemp -d)
    trap 'rm -rf "$TMPDIR"' EXIT
    echo "==> Cloning spring-projects/spring-framework (shallow, sparse checkout)..."
    git clone --depth 1 --filter=blob:none --sparse "$REPO_URL" "$TMPDIR/spring" 2>&1 | tail -1
    cd "$TMPDIR/spring"
    git sparse-checkout set spring-core spring-beans spring-context spring-web spring-webmvc 2>/dev/null
    cd - > /dev/null
    FILE_COUNT=$(find "$TMPDIR/spring" -name '*.java' -not -path '*/testdata/*' | wc -l)
    echo "==> Found $FILE_COUNT .java files"
    echo "==> Building parse_all_java..."
    cargo build --release -p test-roundtrip --bin parse_all_java 2>&1 | tail -1
    echo "==> Parsing all .java files..."
    echo
    ./target/release/parse_all_java "$TMPDIR/spring"

# Run all integration tests
integration-test-all: integration-test-go integration-test-rust integration-test-typescript integration-test-javascript integration-test-java
