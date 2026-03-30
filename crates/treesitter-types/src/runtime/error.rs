use super::span::Span;

#[derive(Debug, thiserror::Error)]
pub enum ParseError {
    #[error("missing required field '{field}' at {span:?}")]
    MissingField { field: &'static str, span: Span },

    #[error("unexpected node kind '{kind}' at {span:?}")]
    UnexpectedKind { kind: String, span: Span },

    #[error("UTF-8 error in source")]
    Utf8(#[from] std::str::Utf8Error),

    #[error("tree-sitter parse failed (returned None)")]
    ParseFailed,
}

impl ParseError {
    pub fn missing_field(field: &'static str, node: tree_sitter::Node<'_>) -> Self {
        Self::MissingField {
            field,
            span: Span::from(node),
        }
    }

    pub fn unexpected_kind(kind: &str, node: tree_sitter::Node<'_>) -> Self {
        Self::UnexpectedKind {
            kind: kind.to_owned(),
            span: Span::from(node),
        }
    }
}
