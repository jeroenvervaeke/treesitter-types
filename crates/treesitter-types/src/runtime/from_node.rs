use super::error::ParseError;
use super::span::Span;

/// Every generated struct and enum implements this.
pub trait FromNode<'tree>: Sized {
    fn from_node(node: tree_sitter::Node<'tree>, src: &'tree [u8]) -> Result<Self, ParseError>;
}

/// Implemented by every generated leaf type (identifiers, literals, etc.)
pub trait LeafNode<'tree>: FromNode<'tree> {
    /// Raw text of this node in the source, borrowed from src.
    fn text(&self) -> &'tree str;
}

/// Implemented by every generated type that has a source location.
pub trait Spanned {
    fn span(&self) -> Span;

    fn start(&self) -> tree_sitter::Point {
        self.span().start
    }

    fn end(&self) -> tree_sitter::Point {
        self.span().end
    }
}
