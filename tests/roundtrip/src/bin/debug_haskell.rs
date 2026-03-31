fn main() {
    let tests: Vec<(&str, &[u8])> = vec![
        ("cabal rule.hs pattern", b"module Test where\n\ndata Scope\n  = User\n  | System\n\ndata SScope (scope :: Scope) where\n  SUser :: SScope User\n  SSystem :: SScope System\n" as &[u8]),
        ("simpler GADT", b"module Test where\n\ndata Foo where\n  Bar :: Foo\n" as &[u8]),
        ("GADT with kind param", b"module Test where\n\ndata Foo (a :: *) where\n  Bar :: Foo Int\n" as &[u8]),
    ];

    let mut parser = tree_sitter::Parser::new();
    parser
        .set_language(&tree_sitter_haskell::LANGUAGE.into())
        .unwrap();

    for (name, src) in &tests {
        let tree = parser.parse(*src, None).unwrap();
        let root = tree.root_node();
        println!("=== {name} ===");
        println!("Has errors: {}", root.has_error());

        match <test_roundtrip::haskell::Haskell as treesitter_types::FromNode>::from_node(root, src)
        {
            Ok(_) => println!("  Typed parse: OK"),
            Err(e) => println!("  Typed parse: FAIL: {e}"),
        }
        println!();
    }
}
