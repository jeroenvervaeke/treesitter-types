#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Value<'tree> {
    Array(::std::boxed::Box<Array<'tree>>),
    False(::std::boxed::Box<False<'tree>>),
    Null(::std::boxed::Box<Null<'tree>>),
    Number(::std::boxed::Box<Number<'tree>>),
    Object(::std::boxed::Box<Object<'tree>>),
    String(::std::boxed::Box<String<'tree>>),
    True(::std::boxed::Box<True<'tree>>),
}
impl<'tree> ::treesitter_types::FromNode<'tree> for Value<'tree> {
    #[allow(clippy::collapsible_else_if)]
    fn from_node(
        node: ::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        match node.kind() {
            "array" => Ok(Self::Array(::std::boxed::Box::new(
                ::treesitter_types::maybe_grow_stack(|| {
                    <Array as ::treesitter_types::FromNode>::from_node(node, src)
                })?,
            ))),
            "false" => Ok(Self::False(::std::boxed::Box::new(
                ::treesitter_types::maybe_grow_stack(|| {
                    <False as ::treesitter_types::FromNode>::from_node(node, src)
                })?,
            ))),
            "null" => Ok(Self::Null(::std::boxed::Box::new(
                ::treesitter_types::maybe_grow_stack(|| {
                    <Null as ::treesitter_types::FromNode>::from_node(node, src)
                })?,
            ))),
            "number" => Ok(Self::Number(::std::boxed::Box::new(
                ::treesitter_types::maybe_grow_stack(|| {
                    <Number as ::treesitter_types::FromNode>::from_node(node, src)
                })?,
            ))),
            "object" => Ok(Self::Object(::std::boxed::Box::new(
                ::treesitter_types::maybe_grow_stack(|| {
                    <Object as ::treesitter_types::FromNode>::from_node(node, src)
                })?,
            ))),
            "string" => Ok(Self::String(::std::boxed::Box::new(
                ::treesitter_types::maybe_grow_stack(|| {
                    <String as ::treesitter_types::FromNode>::from_node(node, src)
                })?,
            ))),
            "true" => Ok(Self::True(::std::boxed::Box::new(
                ::treesitter_types::maybe_grow_stack(|| {
                    <True as ::treesitter_types::FromNode>::from_node(node, src)
                })?,
            ))),
            other => Err(::treesitter_types::ParseError::unexpected_kind(other, node)),
        }
    }
}
impl ::treesitter_types::Spanned for Value<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        match self {
            Self::Array(inner) => inner.span(),
            Self::False(inner) => inner.span(),
            Self::Null(inner) => inner.span(),
            Self::Number(inner) => inner.span(),
            Self::Object(inner) => inner.span(),
            Self::String(inner) => inner.span(),
            Self::True(inner) => inner.span(),
        }
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Array<'tree> {
    pub span: ::treesitter_types::Span,
    pub children: ::std::vec::Vec<Value<'tree>>,
}
impl<'tree> ::treesitter_types::FromNode<'tree> for Array<'tree> {
    #[allow(clippy::match_single_binding, clippy::suspicious_else_formatting)]
    fn from_node(
        node: ::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        debug_assert_eq!(node.kind(), "array");
        Ok(Self {
            span: ::treesitter_types::Span::from(node),
            children: {
                #[allow(clippy::suspicious_else_formatting)]
                let non_field_children = {
                    let mut cursor = node.walk();
                    let mut result = ::std::vec::Vec::new();
                    if cursor.goto_first_child() {
                        loop {
                            if cursor.field_name().is_none()
                                && cursor.node().is_named()
                                && !cursor.node().is_extra()
                            {
                                result.push(cursor.node());
                            }
                            if !cursor.goto_next_sibling() {
                                break;
                            }
                        }
                    }
                    result
                };
                let mut items = ::std::vec::Vec::new();
                for child in non_field_children {
                    items.push(::treesitter_types::maybe_grow_stack(|| {
                        <Value as ::treesitter_types::FromNode>::from_node(child, src)
                    })?);
                }
                items
            },
        })
    }
}
impl ::treesitter_types::Spanned for Array<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        self.span
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Document<'tree> {
    pub span: ::treesitter_types::Span,
    pub children: ::std::vec::Vec<Value<'tree>>,
}
impl<'tree> ::treesitter_types::FromNode<'tree> for Document<'tree> {
    #[allow(clippy::match_single_binding, clippy::suspicious_else_formatting)]
    fn from_node(
        node: ::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        debug_assert_eq!(node.kind(), "document");
        Ok(Self {
            span: ::treesitter_types::Span::from(node),
            children: {
                #[allow(clippy::suspicious_else_formatting)]
                let non_field_children = {
                    let mut cursor = node.walk();
                    let mut result = ::std::vec::Vec::new();
                    if cursor.goto_first_child() {
                        loop {
                            if cursor.field_name().is_none()
                                && cursor.node().is_named()
                                && !cursor.node().is_extra()
                            {
                                result.push(cursor.node());
                            }
                            if !cursor.goto_next_sibling() {
                                break;
                            }
                        }
                    }
                    result
                };
                let mut items = ::std::vec::Vec::new();
                for child in non_field_children {
                    items.push(::treesitter_types::maybe_grow_stack(|| {
                        <Value as ::treesitter_types::FromNode>::from_node(child, src)
                    })?);
                }
                items
            },
        })
    }
}
impl ::treesitter_types::Spanned for Document<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        self.span
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Object<'tree> {
    pub span: ::treesitter_types::Span,
    pub children: ::std::vec::Vec<Pair<'tree>>,
}
impl<'tree> ::treesitter_types::FromNode<'tree> for Object<'tree> {
    #[allow(clippy::match_single_binding, clippy::suspicious_else_formatting)]
    fn from_node(
        node: ::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        debug_assert_eq!(node.kind(), "object");
        Ok(Self {
            span: ::treesitter_types::Span::from(node),
            children: {
                #[allow(clippy::suspicious_else_formatting)]
                let non_field_children = {
                    let mut cursor = node.walk();
                    let mut result = ::std::vec::Vec::new();
                    if cursor.goto_first_child() {
                        loop {
                            if cursor.field_name().is_none()
                                && cursor.node().is_named()
                                && !cursor.node().is_extra()
                            {
                                result.push(cursor.node());
                            }
                            if !cursor.goto_next_sibling() {
                                break;
                            }
                        }
                    }
                    result
                };
                let mut items = ::std::vec::Vec::new();
                for child in non_field_children {
                    items.push(::treesitter_types::maybe_grow_stack(|| {
                        <Pair as ::treesitter_types::FromNode>::from_node(child, src)
                    })?);
                }
                items
            },
        })
    }
}
impl ::treesitter_types::Spanned for Object<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        self.span
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Pair<'tree> {
    pub span: ::treesitter_types::Span,
    pub key: String<'tree>,
    pub value: Value<'tree>,
}
impl<'tree> ::treesitter_types::FromNode<'tree> for Pair<'tree> {
    #[allow(clippy::match_single_binding, clippy::suspicious_else_formatting)]
    fn from_node(
        node: ::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        debug_assert_eq!(node.kind(), "pair");
        Ok(Self {
            span: ::treesitter_types::Span::from(node),
            key: {
                let child = node
                    .child_by_field_name("key")
                    .ok_or_else(|| ::treesitter_types::ParseError::missing_field("key", node))?;
                ::treesitter_types::maybe_grow_stack(|| {
                    <String as ::treesitter_types::FromNode>::from_node(child, src)
                })?
            },
            value: {
                let child = node
                    .child_by_field_name("value")
                    .ok_or_else(|| ::treesitter_types::ParseError::missing_field("value", node))?;
                ::treesitter_types::maybe_grow_stack(|| {
                    <Value as ::treesitter_types::FromNode>::from_node(child, src)
                })?
            },
        })
    }
}
impl ::treesitter_types::Spanned for Pair<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        self.span
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct String<'tree> {
    pub span: ::treesitter_types::Span,
    pub children: ::std::vec::Vec<StringChildren<'tree>>,
}
impl<'tree> ::treesitter_types::FromNode<'tree> for String<'tree> {
    #[allow(clippy::match_single_binding, clippy::suspicious_else_formatting)]
    fn from_node(
        node: ::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        debug_assert_eq!(node.kind(), "string");
        Ok(Self {
            span: ::treesitter_types::Span::from(node),
            children: {
                #[allow(clippy::suspicious_else_formatting)]
                let non_field_children = {
                    let mut cursor = node.walk();
                    let mut result = ::std::vec::Vec::new();
                    if cursor.goto_first_child() {
                        loop {
                            if cursor.field_name().is_none()
                                && cursor.node().is_named()
                                && !cursor.node().is_extra()
                            {
                                result.push(cursor.node());
                            }
                            if !cursor.goto_next_sibling() {
                                break;
                            }
                        }
                    }
                    result
                };
                let mut items = ::std::vec::Vec::new();
                for child in non_field_children {
                    items.push(::treesitter_types::maybe_grow_stack(|| {
                        <StringChildren as ::treesitter_types::FromNode>::from_node(child, src)
                    })?);
                }
                items
            },
        })
    }
}
impl ::treesitter_types::Spanned for String<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        self.span
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Comment<'tree> {
    pub span: ::treesitter_types::Span,
    text: &'tree str,
}
impl<'tree> ::treesitter_types::FromNode<'tree> for Comment<'tree> {
    fn from_node(
        node: ::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        debug_assert_eq!(node.kind(), "comment");
        Ok(Self {
            span: ::treesitter_types::Span::from(node),
            text: node.utf8_text(src)?,
        })
    }
}
impl<'tree> ::treesitter_types::LeafNode<'tree> for Comment<'tree> {
    fn text(&self) -> &'tree str {
        self.text
    }
}
impl ::treesitter_types::Spanned for Comment<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        self.span
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct EscapeSequence<'tree> {
    pub span: ::treesitter_types::Span,
    text: &'tree str,
}
impl<'tree> ::treesitter_types::FromNode<'tree> for EscapeSequence<'tree> {
    fn from_node(
        node: ::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        debug_assert_eq!(node.kind(), "escape_sequence");
        Ok(Self {
            span: ::treesitter_types::Span::from(node),
            text: node.utf8_text(src)?,
        })
    }
}
impl<'tree> ::treesitter_types::LeafNode<'tree> for EscapeSequence<'tree> {
    fn text(&self) -> &'tree str {
        self.text
    }
}
impl ::treesitter_types::Spanned for EscapeSequence<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        self.span
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct False<'tree> {
    pub span: ::treesitter_types::Span,
    text: &'tree str,
}
impl<'tree> ::treesitter_types::FromNode<'tree> for False<'tree> {
    fn from_node(
        node: ::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        debug_assert_eq!(node.kind(), "false");
        Ok(Self {
            span: ::treesitter_types::Span::from(node),
            text: node.utf8_text(src)?,
        })
    }
}
impl<'tree> ::treesitter_types::LeafNode<'tree> for False<'tree> {
    fn text(&self) -> &'tree str {
        self.text
    }
}
impl ::treesitter_types::Spanned for False<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        self.span
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Null<'tree> {
    pub span: ::treesitter_types::Span,
    text: &'tree str,
}
impl<'tree> ::treesitter_types::FromNode<'tree> for Null<'tree> {
    fn from_node(
        node: ::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        debug_assert_eq!(node.kind(), "null");
        Ok(Self {
            span: ::treesitter_types::Span::from(node),
            text: node.utf8_text(src)?,
        })
    }
}
impl<'tree> ::treesitter_types::LeafNode<'tree> for Null<'tree> {
    fn text(&self) -> &'tree str {
        self.text
    }
}
impl ::treesitter_types::Spanned for Null<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        self.span
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Number<'tree> {
    pub span: ::treesitter_types::Span,
    text: &'tree str,
}
impl<'tree> ::treesitter_types::FromNode<'tree> for Number<'tree> {
    fn from_node(
        node: ::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        debug_assert_eq!(node.kind(), "number");
        Ok(Self {
            span: ::treesitter_types::Span::from(node),
            text: node.utf8_text(src)?,
        })
    }
}
impl<'tree> ::treesitter_types::LeafNode<'tree> for Number<'tree> {
    fn text(&self) -> &'tree str {
        self.text
    }
}
impl ::treesitter_types::Spanned for Number<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        self.span
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct StringContent<'tree> {
    pub span: ::treesitter_types::Span,
    text: &'tree str,
}
impl<'tree> ::treesitter_types::FromNode<'tree> for StringContent<'tree> {
    fn from_node(
        node: ::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        debug_assert_eq!(node.kind(), "string_content");
        Ok(Self {
            span: ::treesitter_types::Span::from(node),
            text: node.utf8_text(src)?,
        })
    }
}
impl<'tree> ::treesitter_types::LeafNode<'tree> for StringContent<'tree> {
    fn text(&self) -> &'tree str {
        self.text
    }
}
impl ::treesitter_types::Spanned for StringContent<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        self.span
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct True<'tree> {
    pub span: ::treesitter_types::Span,
    text: &'tree str,
}
impl<'tree> ::treesitter_types::FromNode<'tree> for True<'tree> {
    fn from_node(
        node: ::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        debug_assert_eq!(node.kind(), "true");
        Ok(Self {
            span: ::treesitter_types::Span::from(node),
            text: node.utf8_text(src)?,
        })
    }
}
impl<'tree> ::treesitter_types::LeafNode<'tree> for True<'tree> {
    fn text(&self) -> &'tree str {
        self.text
    }
}
impl ::treesitter_types::Spanned for True<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        self.span
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum StringChildren<'tree> {
    EscapeSequence(::std::boxed::Box<EscapeSequence<'tree>>),
    StringContent(::std::boxed::Box<StringContent<'tree>>),
}
impl<'tree> ::treesitter_types::FromNode<'tree> for StringChildren<'tree> {
    #[allow(clippy::collapsible_else_if)]
    fn from_node(
        node: ::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        match node.kind() {
            "escape_sequence" => Ok(Self::EscapeSequence(::std::boxed::Box::new(
                ::treesitter_types::maybe_grow_stack(|| {
                    <EscapeSequence as ::treesitter_types::FromNode>::from_node(node, src)
                })?,
            ))),
            "string_content" => Ok(Self::StringContent(::std::boxed::Box::new(
                ::treesitter_types::maybe_grow_stack(|| {
                    <StringContent as ::treesitter_types::FromNode>::from_node(node, src)
                })?,
            ))),
            other => Err(::treesitter_types::ParseError::unexpected_kind(other, node)),
        }
    }
}
impl ::treesitter_types::Spanned for StringChildren<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        match self {
            Self::EscapeSequence(inner) => inner.span(),
            Self::StringContent(inner) => inner.span(),
        }
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum AnyNode<'tree> {
    Value(Value<'tree>),
    Array(Array<'tree>),
    Document(Document<'tree>),
    Object(Object<'tree>),
    Pair(Pair<'tree>),
    String(String<'tree>),
    Comment(Comment<'tree>),
    EscapeSequence(EscapeSequence<'tree>),
    False(False<'tree>),
    Null(Null<'tree>),
    Number(Number<'tree>),
    StringContent(StringContent<'tree>),
    True(True<'tree>),
    Unknown(::tree_sitter::Node<'tree>),
}
impl<'tree> AnyNode<'tree> {
    pub fn from_node(node: ::tree_sitter::Node<'tree>, src: &'tree [u8]) -> Self {
        match node.kind() {
            "_value" => ::treesitter_types::maybe_grow_stack(|| {
                <Value as ::treesitter_types::FromNode>::from_node(node, src)
            })
            .map(Self::Value)
            .unwrap_or(Self::Unknown(node)),
            "array" => ::treesitter_types::maybe_grow_stack(|| {
                <Array as ::treesitter_types::FromNode>::from_node(node, src)
            })
            .map(Self::Array)
            .unwrap_or(Self::Unknown(node)),
            "document" => ::treesitter_types::maybe_grow_stack(|| {
                <Document as ::treesitter_types::FromNode>::from_node(node, src)
            })
            .map(Self::Document)
            .unwrap_or(Self::Unknown(node)),
            "object" => ::treesitter_types::maybe_grow_stack(|| {
                <Object as ::treesitter_types::FromNode>::from_node(node, src)
            })
            .map(Self::Object)
            .unwrap_or(Self::Unknown(node)),
            "pair" => ::treesitter_types::maybe_grow_stack(|| {
                <Pair as ::treesitter_types::FromNode>::from_node(node, src)
            })
            .map(Self::Pair)
            .unwrap_or(Self::Unknown(node)),
            "string" => ::treesitter_types::maybe_grow_stack(|| {
                <String as ::treesitter_types::FromNode>::from_node(node, src)
            })
            .map(Self::String)
            .unwrap_or(Self::Unknown(node)),
            "comment" => ::treesitter_types::maybe_grow_stack(|| {
                <Comment as ::treesitter_types::FromNode>::from_node(node, src)
            })
            .map(Self::Comment)
            .unwrap_or(Self::Unknown(node)),
            "escape_sequence" => ::treesitter_types::maybe_grow_stack(|| {
                <EscapeSequence as ::treesitter_types::FromNode>::from_node(node, src)
            })
            .map(Self::EscapeSequence)
            .unwrap_or(Self::Unknown(node)),
            "false" => ::treesitter_types::maybe_grow_stack(|| {
                <False as ::treesitter_types::FromNode>::from_node(node, src)
            })
            .map(Self::False)
            .unwrap_or(Self::Unknown(node)),
            "null" => ::treesitter_types::maybe_grow_stack(|| {
                <Null as ::treesitter_types::FromNode>::from_node(node, src)
            })
            .map(Self::Null)
            .unwrap_or(Self::Unknown(node)),
            "number" => ::treesitter_types::maybe_grow_stack(|| {
                <Number as ::treesitter_types::FromNode>::from_node(node, src)
            })
            .map(Self::Number)
            .unwrap_or(Self::Unknown(node)),
            "string_content" => ::treesitter_types::maybe_grow_stack(|| {
                <StringContent as ::treesitter_types::FromNode>::from_node(node, src)
            })
            .map(Self::StringContent)
            .unwrap_or(Self::Unknown(node)),
            "true" => ::treesitter_types::maybe_grow_stack(|| {
                <True as ::treesitter_types::FromNode>::from_node(node, src)
            })
            .map(Self::True)
            .unwrap_or(Self::Unknown(node)),
            _ => Self::Unknown(node),
        }
    }
}
impl ::treesitter_types::Spanned for AnyNode<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        match self {
            Self::Value(inner) => inner.span(),
            Self::Array(inner) => inner.span(),
            Self::Document(inner) => inner.span(),
            Self::Object(inner) => inner.span(),
            Self::Pair(inner) => inner.span(),
            Self::String(inner) => inner.span(),
            Self::Comment(inner) => inner.span(),
            Self::EscapeSequence(inner) => inner.span(),
            Self::False(inner) => inner.span(),
            Self::Null(inner) => inner.span(),
            Self::Number(inner) => inner.span(),
            Self::StringContent(inner) => inner.span(),
            Self::True(inner) => inner.span(),
            Self::Unknown(node) => ::treesitter_types::Span::from(*node),
        }
    }
}
