use test_roundtrip::bash::{Program, VariableAssignment};
use test_roundtrip::parse_bash;
use treesitter_types::FromNode;

fn main() {
    let src = b"x=\n";
    let tree = parse_bash(src);
    let root = tree.root_node();

    // Try parsing the whole program
    match Program::from_node(root, src) {
        Ok(p) => {
            println!("Program OK, children: {}", p.children.len());
        }
        Err(e) => println!("Program FAIL: {e}"),
    }

    // Try parsing the variable_assignment node directly
    let mut cursor = root.walk();
    let va_node = root
        .named_children(&mut cursor)
        .find(|c| c.kind() == "variable_assignment")
        .expect("no variable_assignment");

    match VariableAssignment::from_node(va_node, src) {
        Ok(va) => println!("VA OK, has value: {:?}", va.value),
        Err(e) => println!("VA FAIL: {e}"),
    }
}
