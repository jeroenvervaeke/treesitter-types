use std::path::PathBuf;
use std::time::Instant;
use test_roundtrip::parse_python;
use test_roundtrip::python::Module;
use treesitter_types::FromNode;

fn main() {
    let dir = std::env::args()
        .nth(1)
        .expect("usage: parse_all_python <directory>");
    let dir = PathBuf::from(dir);

    if !dir.is_dir() {
        eprintln!("error: {} is not a directory", dir.display());
        std::process::exit(1);
    }

    let mut total = 0usize;
    let mut success = 0usize;
    let mut ts_clean = 0usize;
    let mut ts_errors = 0usize;
    let mut typed_failed_clean = 0usize;
    let mut typed_failed_dirty = 0usize;

    let start = Instant::now();

    for entry in walk_files(&dir, "py") {
        total += 1;

        let src = match std::fs::read(&entry) {
            Ok(s) => s,
            Err(e) => {
                eprintln!("SKIP {}: {e}", entry.display());
                continue;
            }
        };

        let tree = parse_python(&src);
        let root = tree.root_node();
        let clean_parse = !root.has_error();

        if clean_parse {
            ts_clean += 1;
        } else {
            ts_errors += 1;
        }

        match Module::from_node(root, &src) {
            Ok(_) => {
                success += 1;
            }
            Err(e) => {
                if clean_parse {
                    typed_failed_clean += 1;
                    eprintln!("FAIL {}: {e}", entry.display());
                } else {
                    typed_failed_dirty += 1;
                }
            }
        }
    }

    let elapsed = start.elapsed();

    println!();
    println!("=== Results ===");
    println!("Files scanned:              {total}");
    println!("tree-sitter clean parse:    {ts_clean}");
    println!("tree-sitter had errors:     {ts_errors}");
    println!("Typed AST success:          {success}");
    println!("Typed AST failed (clean):   {typed_failed_clean}  <- real failures");
    println!(
        "Typed AST failed (dirty):   {typed_failed_dirty}  <- tree-sitter couldn't parse either"
    );
    println!("Time:                       {elapsed:.2?}");

    if typed_failed_clean > 0 {
        let pct = (typed_failed_clean as f64 / ts_clean as f64) * 100.0;
        eprintln!(
            "\n{typed_failed_clean} files ({pct:.2}%) failed typed conversion despite clean tree-sitter parse"
        );
        std::process::exit(1);
    } else {
        println!("\nAll cleanly-parsed files converted successfully.");
    }
}

fn walk_files(dir: &PathBuf, ext: &str) -> Vec<PathBuf> {
    let mut files = Vec::new();
    walk_dir_recursive(dir, ext, &mut files);
    files.sort();
    files
}

fn walk_dir_recursive(dir: &PathBuf, ext: &str, files: &mut Vec<PathBuf>) {
    let Ok(entries) = std::fs::read_dir(dir) else {
        return;
    };
    for entry in entries {
        let Ok(entry) = entry else { continue };
        let path = entry.path();
        if path.is_dir() {
            let name = path.file_name().unwrap_or_default().to_string_lossy();
            if name == "testdata" || name == "vendor" || name.starts_with('.') {
                continue;
            }
            walk_dir_recursive(&path, ext, files);
        } else if path.extension().is_some_and(|e| e == ext) {
            files.push(path);
        }
    }
}
