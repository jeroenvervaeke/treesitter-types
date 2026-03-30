fn main() {
    treesitter_types::codegen::emit_to_out_dir("node-types.json").unwrap();
}
