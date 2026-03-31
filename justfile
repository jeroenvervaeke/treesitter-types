# Generate typed AST code for all language crates from grammar crate NODE_TYPES constants
generate:
    cargo run -p treesitter-types-codegen-runner
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

# Run integration test: parse an entire real-world Python repository
integration-test-python:
    #!/usr/bin/env bash
    set -euo pipefail
    REPO_URL="https://github.com/python/cpython.git"
    TMPDIR=$(mktemp -d)
    trap 'rm -rf "$TMPDIR"' EXIT
    echo "==> Cloning python/cpython (shallow, sparse checkout)..."
    git clone --depth 1 --filter=blob:none --sparse "$REPO_URL" "$TMPDIR/cpython" 2>&1 | tail -1
    cd "$TMPDIR/cpython"
    git sparse-checkout set Lib 2>/dev/null
    cd - > /dev/null
    FILE_COUNT=$(find "$TMPDIR/cpython/Lib" -name '*.py' -not -path '*/test/*' | wc -l)
    echo "==> Found $FILE_COUNT .py files in Lib/"
    echo "==> Building parse_all_python..."
    cargo build --release -p test-roundtrip --bin parse_all_python 2>&1 | tail -1
    echo "==> Parsing all .py files..."
    echo
    ./target/release/parse_all_python "$TMPDIR/cpython/Lib"

# Run integration test: parse an entire real-world C repository
integration-test-c:
    #!/usr/bin/env bash
    set -euo pipefail
    REPO_URL="https://github.com/git/git.git"
    TMPDIR=$(mktemp -d)
    trap 'rm -rf "$TMPDIR"' EXIT
    echo "==> Cloning git/git (shallow)..."
    git clone --depth 1 --filter=blob:none "$REPO_URL" "$TMPDIR/git" 2>&1 | tail -1
    FILE_COUNT=$(find "$TMPDIR/git" -name '*.c' -not -path '*/t/*' | wc -l)
    echo "==> Found $FILE_COUNT .c files"
    echo "==> Building parse_all_c..."
    cargo build --release -p test-roundtrip --bin parse_all_c 2>&1 | tail -1
    echo "==> Parsing all .c files..."
    echo
    ./target/release/parse_all_c "$TMPDIR/git"

# Run integration test: parse an entire real-world C++ repository
integration-test-cpp:
    #!/usr/bin/env bash
    set -euo pipefail
    REPO_URL="https://github.com/nlohmann/json.git"
    TMPDIR=$(mktemp -d)
    trap 'rm -rf "$TMPDIR"' EXIT
    echo "==> Cloning nlohmann/json (shallow)..."
    git clone --depth 1 --filter=blob:none "$REPO_URL" "$TMPDIR/json" 2>&1 | tail -1
    FILE_COUNT=$(find "$TMPDIR/json" -name '*.cpp' -o -name '*.hpp' | wc -l)
    echo "==> Found $FILE_COUNT .cpp/.hpp files"
    echo "==> Building parse_all_cpp..."
    cargo build --release -p test-roundtrip --bin parse_all_cpp 2>&1 | tail -1
    echo "==> Parsing all .cpp files..."
    echo
    ./target/release/parse_all_cpp "$TMPDIR/json"

# Run integration test: parse an entire real-world Bash repository
integration-test-bash:
    #!/usr/bin/env bash
    set -euo pipefail
    REPO_URL="https://github.com/ohmyzsh/ohmyzsh.git"
    TMPDIR=$(mktemp -d)
    trap 'rm -rf "$TMPDIR"' EXIT
    echo "==> Cloning ohmyzsh/ohmyzsh (shallow)..."
    git clone --depth 1 --filter=blob:none "$REPO_URL" "$TMPDIR/ohmyzsh" 2>&1 | tail -1
    FILE_COUNT=$(find "$TMPDIR/ohmyzsh" -name '*.sh' | wc -l)
    echo "==> Found $FILE_COUNT .sh files"
    echo "==> Building parse_all_bash..."
    cargo build --release -p test-roundtrip --bin parse_all_bash 2>&1 | tail -1
    echo "==> Parsing all .sh files..."
    echo
    ./target/release/parse_all_bash "$TMPDIR/ohmyzsh"

# Run integration test: parse an entire real-world Ruby repository
integration-test-ruby:
    #!/usr/bin/env bash
    set -euo pipefail
    REPO_URL="https://github.com/rails/rails.git"
    TMPDIR=$(mktemp -d)
    trap 'rm -rf "$TMPDIR"' EXIT
    echo "==> Cloning rails/rails (shallow, sparse checkout)..."
    git clone --depth 1 --filter=blob:none --sparse "$REPO_URL" "$TMPDIR/rails" 2>&1 | tail -1
    cd "$TMPDIR/rails"
    git sparse-checkout set activerecord activesupport actionpack 2>/dev/null
    cd - > /dev/null
    FILE_COUNT=$(find "$TMPDIR/rails" -name '*.rb' | wc -l)
    echo "==> Found $FILE_COUNT .rb files"
    echo "==> Building parse_all_ruby..."
    cargo build --release -p test-roundtrip --bin parse_all_ruby 2>&1 | tail -1
    echo "==> Parsing all .rb files..."
    echo
    ./target/release/parse_all_ruby "$TMPDIR/rails"

# Run integration test: parse an entire real-world C# repository
integration-test-c-sharp:
    #!/usr/bin/env bash
    set -euo pipefail
    REPO_URL="https://github.com/dotnet/runtime.git"
    TMPDIR=$(mktemp -d)
    trap 'rm -rf "$TMPDIR"' EXIT
    echo "==> Cloning dotnet/runtime (shallow, sparse checkout)..."
    git clone --depth 1 --filter=blob:none --sparse "$REPO_URL" "$TMPDIR/runtime" 2>&1 | tail -1
    cd "$TMPDIR/runtime"
    git sparse-checkout set src/libraries/System.Text.Json/src 2>/dev/null
    cd - > /dev/null
    FILE_COUNT=$(find "$TMPDIR/runtime" -name '*.cs' | wc -l)
    echo "==> Found $FILE_COUNT .cs files"
    echo "==> Building parse_all_c_sharp..."
    cargo build --release -p test-roundtrip --bin parse_all_c_sharp 2>&1 | tail -1
    echo "==> Parsing all .cs files..."
    echo
    ./target/release/parse_all_c_sharp "$TMPDIR/runtime"

# Run integration test: parse real-world CSS files
integration-test-css:
    #!/usr/bin/env bash
    set -euo pipefail
    REPO_URL="https://github.com/twbs/bootstrap.git"
    TMPDIR=$(mktemp -d)
    trap 'rm -rf "$TMPDIR"' EXIT
    echo "==> Cloning twbs/bootstrap (shallow, sparse checkout)..."
    git clone --depth 1 --filter=blob:none --sparse "$REPO_URL" "$TMPDIR/bootstrap" 2>&1 | tail -1
    cd "$TMPDIR/bootstrap"
    git sparse-checkout set dist/css 2>/dev/null
    cd - > /dev/null
    FILE_COUNT=$(find "$TMPDIR/bootstrap" -name '*.css' | wc -l)
    echo "==> Found $FILE_COUNT .css files"
    echo "==> Building parse_all_css..."
    cargo build --release -p test-roundtrip --bin parse_all_css 2>&1 | tail -1
    echo "==> Parsing all .css files..."
    echo
    ./target/release/parse_all_css "$TMPDIR/bootstrap"

# Run integration test: parse real-world PHP files
integration-test-php:
    #!/usr/bin/env bash
    set -euo pipefail
    REPO_URL="https://github.com/laravel/framework.git"
    TMPDIR=$(mktemp -d)
    trap 'rm -rf "$TMPDIR"' EXIT
    echo "==> Cloning laravel/framework (shallow, sparse checkout)..."
    git clone --depth 1 --filter=blob:none --sparse "$REPO_URL" "$TMPDIR/laravel" 2>&1 | tail -1
    cd "$TMPDIR/laravel"
    git sparse-checkout set src 2>/dev/null
    cd - > /dev/null
    FILE_COUNT=$(find "$TMPDIR/laravel/src" -name '*.php' | wc -l)
    echo "==> Found $FILE_COUNT .php files in src/"
    echo "==> Building parse_all_php..."
    cargo build --release -p test-roundtrip --bin parse_all_php 2>&1 | tail -1
    echo "==> Parsing all .php files..."
    echo
    ./target/release/parse_all_php "$TMPDIR/laravel/src"

# Run integration test: parse real-world JSON files
integration-test-json:
    #!/usr/bin/env bash
    set -euo pipefail
    REPO_URL="https://github.com/SchemaStore/schemastore.git"
    TMPDIR=$(mktemp -d)
    trap 'rm -rf "$TMPDIR"' EXIT
    echo "==> Cloning SchemaStore/schemastore (shallow, sparse checkout)..."
    git clone --depth 1 --filter=blob:none --sparse "$REPO_URL" "$TMPDIR/schemastore" 2>&1 | tail -1
    cd "$TMPDIR/schemastore"
    git sparse-checkout set src/schemas/json 2>/dev/null
    cd - > /dev/null
    FILE_COUNT=$(find "$TMPDIR/schemastore" -name '*.json' | wc -l)
    echo "==> Found $FILE_COUNT .json files"
    echo "==> Building parse_all_json..."
    cargo build --release -p test-roundtrip --bin parse_all_json 2>&1 | tail -1
    echo "==> Parsing all .json files..."
    echo
    ./target/release/parse_all_json "$TMPDIR/schemastore"

# Run integration test: parse real-world HTML files
integration-test-html:
    #!/usr/bin/env bash
    set -euo pipefail
    REPO_URL="https://github.com/nicehash/NiceHashQuickMiner.git"
    TMPDIR=$(mktemp -d)
    trap 'rm -rf "$TMPDIR"' EXIT
    echo "==> Cloning a repo with HTML files..."
    git clone --depth 1 --filter=blob:none "$REPO_URL" "$TMPDIR/repo" 2>&1 | tail -1
    FILE_COUNT=$(find "$TMPDIR/repo" -name '*.html' | wc -l)
    echo "==> Found $FILE_COUNT .html files"
    echo "==> Building parse_all_html..."
    cargo build --release -p test-roundtrip --bin parse_all_html 2>&1 | tail -1
    echo "==> Parsing all .html files..."
    echo
    ./target/release/parse_all_html "$TMPDIR/repo"

# Run integration test: parse an entire real-world Scala repository
integration-test-scala:
    #!/usr/bin/env bash
    set -euo pipefail
    REPO_URL="https://github.com/apache/spark.git"
    TMPDIR=$(mktemp -d)
    trap 'rm -rf "$TMPDIR"' EXIT
    echo "==> Cloning apache/spark (shallow, sparse checkout)..."
    git clone --depth 1 --filter=blob:none --sparse "$REPO_URL" "$TMPDIR/spark" 2>&1 | tail -1
    cd "$TMPDIR/spark"
    git sparse-checkout set core/src 2>/dev/null
    cd - > /dev/null
    FILE_COUNT=$(find "$TMPDIR/spark" -name '*.scala' | wc -l)
    echo "==> Found $FILE_COUNT .scala files"
    echo "==> Building parse_all_scala..."
    cargo build --release -p test-roundtrip --bin parse_all_scala 2>&1 | tail -1
    echo "==> Parsing all .scala files..."
    echo
    ./target/release/parse_all_scala "$TMPDIR/spark"

# Run all integration tests
integration-test-all: integration-test-go integration-test-rust integration-test-typescript integration-test-javascript integration-test-java integration-test-python integration-test-c integration-test-cpp integration-test-ruby integration-test-c-sharp integration-test-css integration-test-php integration-test-json integration-test-html integration-test-scala
