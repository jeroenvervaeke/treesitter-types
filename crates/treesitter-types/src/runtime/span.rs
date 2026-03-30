#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Span {
    pub start: tree_sitter::Point,
    pub end: tree_sitter::Point,
    pub start_byte: usize,
    pub end_byte: usize,
}

impl From<tree_sitter::Node<'_>> for Span {
    fn from(node: tree_sitter::Node<'_>) -> Self {
        Self {
            start: node.start_position(),
            end: node.end_position(),
            start_byte: node.start_byte(),
            end_byte: node.end_byte(),
        }
    }
}
