use std::path::PathBuf;

fn main() {
    let path = std::env::args()
        .nth(1)
        .map(PathBuf::from)
        .unwrap_or_else(|| {
            eprintln!("usage: treesitter-types-cli <node-types.json>");
            std::process::exit(1);
        });

    let json = std::fs::read_to_string(&path).unwrap_or_else(|e| {
        eprintln!("error: failed to read {}: {e}", path.display());
        std::process::exit(1);
    });

    let code = treesitter_types::codegen::generate_to_string(&json).unwrap_or_else(|e| {
        eprintln!("error: codegen failed: {e}");
        std::process::exit(1);
    });

    print!("{code}");
}
