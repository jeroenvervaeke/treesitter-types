#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Attribute<'tree> {
    pub span: ::treesitter_types::Span,
    pub children: ::std::vec::Vec<AttributeChildren<'tree>>,
}
impl<'tree> ::treesitter_types::FromNode<'tree> for Attribute<'tree> {
    #[allow(clippy::match_single_binding, clippy::suspicious_else_formatting)]
    fn from_node(
        node: ::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        debug_assert_eq!(node.kind(), "attribute");
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
                    items.push(::treesitter_types::runtime::maybe_grow_stack(|| {
                        <AttributeChildren as ::treesitter_types::FromNode>::from_node(child, src)
                    })?);
                }
                items
            },
        })
    }
}
impl ::treesitter_types::Spanned for Attribute<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        self.span
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Doctype<'tree> {
    pub span: ::treesitter_types::Span,
    text: &'tree str,
}
impl<'tree> ::treesitter_types::FromNode<'tree> for Doctype<'tree> {
    fn from_node(
        node: ::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        debug_assert_eq!(node.kind(), "doctype");
        Ok(Self {
            span: ::treesitter_types::Span::from(node),
            text: node.utf8_text(src)?,
        })
    }
}
impl<'tree> ::treesitter_types::LeafNode<'tree> for Doctype<'tree> {
    fn text(&self) -> &'tree str {
        self.text
    }
}
impl ::treesitter_types::Spanned for Doctype<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        self.span
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Document<'tree> {
    pub span: ::treesitter_types::Span,
    pub children: ::std::vec::Vec<DocumentChildren<'tree>>,
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
                    items.push(::treesitter_types::runtime::maybe_grow_stack(|| {
                        <DocumentChildren as ::treesitter_types::FromNode>::from_node(child, src)
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
pub struct Element<'tree> {
    pub span: ::treesitter_types::Span,
    pub children: ::std::vec::Vec<ElementChildren<'tree>>,
}
impl<'tree> ::treesitter_types::FromNode<'tree> for Element<'tree> {
    #[allow(clippy::match_single_binding, clippy::suspicious_else_formatting)]
    fn from_node(
        node: ::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        debug_assert_eq!(node.kind(), "element");
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
                    items.push(::treesitter_types::runtime::maybe_grow_stack(|| {
                        <ElementChildren as ::treesitter_types::FromNode>::from_node(child, src)
                    })?);
                }
                items
            },
        })
    }
}
impl ::treesitter_types::Spanned for Element<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        self.span
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct EndTag<'tree> {
    pub span: ::treesitter_types::Span,
    pub children: TagName<'tree>,
}
impl<'tree> ::treesitter_types::FromNode<'tree> for EndTag<'tree> {
    #[allow(clippy::match_single_binding, clippy::suspicious_else_formatting)]
    fn from_node(
        node: ::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        debug_assert_eq!(node.kind(), "end_tag");
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
                let child = if let Some(&c) = non_field_children.first() {
                    c
                } else {
                    let mut fallback_cursor = node.walk();
                    let mut fallback_child = None;
                    if fallback_cursor.goto_first_child() {
                        loop {
                            if fallback_cursor.field_name().is_none()
                                && !fallback_cursor.node().is_extra()
                            {
                                let candidate = fallback_cursor.node();
                                #[allow(clippy::needless_question_mark)]
                                if (|| -> ::core::result::Result<
                                    _,
                                    ::treesitter_types::ParseError,
                                > {
                                    let child = candidate;
                                    Ok(
                                        ::treesitter_types::runtime::maybe_grow_stack(|| <TagName as ::treesitter_types::FromNode>::from_node(
                                            child,
                                            src,
                                        ))?,
                                    )
                                })()
                                    .is_ok()
                                {
                                    fallback_child = Some(candidate);
                                    break;
                                }
                            }
                            if !fallback_cursor.goto_next_sibling() {
                                break;
                            }
                        }
                    }
                    if fallback_child.is_none() {
                        let mut cursor2 = node.walk();
                        if cursor2.goto_first_child() {
                            loop {
                                if cursor2.node().is_named() && !cursor2.node().is_extra() {
                                    let candidate = cursor2.node();
                                    #[allow(clippy::needless_question_mark)]
                                    if (|| -> ::core::result::Result<
                                        _,
                                        ::treesitter_types::ParseError,
                                    > {
                                        let child = candidate;
                                        Ok(
                                            ::treesitter_types::runtime::maybe_grow_stack(|| <TagName as ::treesitter_types::FromNode>::from_node(
                                                child,
                                                src,
                                            ))?,
                                        )
                                    })()
                                        .is_ok()
                                    {
                                        fallback_child = Some(candidate);
                                        break;
                                    }
                                }
                                if !cursor2.goto_next_sibling() {
                                    break;
                                }
                            }
                        }
                    }
                    fallback_child.ok_or_else(|| {
                        ::treesitter_types::ParseError::missing_field("children", node)
                    })?
                };
                ::treesitter_types::runtime::maybe_grow_stack(|| {
                    <TagName as ::treesitter_types::FromNode>::from_node(child, src)
                })?
            },
        })
    }
}
impl ::treesitter_types::Spanned for EndTag<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        self.span
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ErroneousEndTag<'tree> {
    pub span: ::treesitter_types::Span,
    pub children: ErroneousEndTagName<'tree>,
}
impl<'tree> ::treesitter_types::FromNode<'tree> for ErroneousEndTag<'tree> {
    #[allow(clippy::match_single_binding, clippy::suspicious_else_formatting)]
    fn from_node(
        node: ::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        debug_assert_eq!(node.kind(), "erroneous_end_tag");
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
                let child = if let Some(&c) = non_field_children.first() {
                    c
                } else {
                    let mut fallback_cursor = node.walk();
                    let mut fallback_child = None;
                    if fallback_cursor.goto_first_child() {
                        loop {
                            if fallback_cursor.field_name().is_none()
                                && !fallback_cursor.node().is_extra()
                            {
                                let candidate = fallback_cursor.node();
                                #[allow(clippy::needless_question_mark)]
                                if (|| -> ::core::result::Result<
                                    _,
                                    ::treesitter_types::ParseError,
                                > {
                                    let child = candidate;
                                    Ok(
                                        ::treesitter_types::runtime::maybe_grow_stack(|| <ErroneousEndTagName as ::treesitter_types::FromNode>::from_node(
                                            child,
                                            src,
                                        ))?,
                                    )
                                })()
                                    .is_ok()
                                {
                                    fallback_child = Some(candidate);
                                    break;
                                }
                            }
                            if !fallback_cursor.goto_next_sibling() {
                                break;
                            }
                        }
                    }
                    if fallback_child.is_none() {
                        let mut cursor2 = node.walk();
                        if cursor2.goto_first_child() {
                            loop {
                                if cursor2.node().is_named() && !cursor2.node().is_extra() {
                                    let candidate = cursor2.node();
                                    #[allow(clippy::needless_question_mark)]
                                    if (|| -> ::core::result::Result<
                                        _,
                                        ::treesitter_types::ParseError,
                                    > {
                                        let child = candidate;
                                        Ok(
                                            ::treesitter_types::runtime::maybe_grow_stack(|| <ErroneousEndTagName as ::treesitter_types::FromNode>::from_node(
                                                child,
                                                src,
                                            ))?,
                                        )
                                    })()
                                        .is_ok()
                                    {
                                        fallback_child = Some(candidate);
                                        break;
                                    }
                                }
                                if !cursor2.goto_next_sibling() {
                                    break;
                                }
                            }
                        }
                    }
                    fallback_child.ok_or_else(|| {
                        ::treesitter_types::ParseError::missing_field("children", node)
                    })?
                };
                ::treesitter_types::runtime::maybe_grow_stack(|| {
                    <ErroneousEndTagName as ::treesitter_types::FromNode>::from_node(child, src)
                })?
            },
        })
    }
}
impl ::treesitter_types::Spanned for ErroneousEndTag<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        self.span
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct QuotedAttributeValue<'tree> {
    pub span: ::treesitter_types::Span,
    pub children: ::core::option::Option<AttributeValue<'tree>>,
}
impl<'tree> ::treesitter_types::FromNode<'tree> for QuotedAttributeValue<'tree> {
    #[allow(clippy::match_single_binding, clippy::suspicious_else_formatting)]
    fn from_node(
        node: ::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        debug_assert_eq!(node.kind(), "quoted_attribute_value");
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
                match non_field_children.first() {
                    Some(&child) => Some(::treesitter_types::runtime::maybe_grow_stack(|| {
                        <AttributeValue as ::treesitter_types::FromNode>::from_node(child, src)
                    })?),
                    None => None,
                }
            },
        })
    }
}
impl ::treesitter_types::Spanned for QuotedAttributeValue<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        self.span
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ScriptElement<'tree> {
    pub span: ::treesitter_types::Span,
    pub children: ::std::vec::Vec<ScriptElementChildren<'tree>>,
}
impl<'tree> ::treesitter_types::FromNode<'tree> for ScriptElement<'tree> {
    #[allow(clippy::match_single_binding, clippy::suspicious_else_formatting)]
    fn from_node(
        node: ::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        debug_assert_eq!(node.kind(), "script_element");
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
                    items.push(::treesitter_types::runtime::maybe_grow_stack(|| {
                        <ScriptElementChildren as ::treesitter_types::FromNode>::from_node(
                            child, src,
                        )
                    })?);
                }
                items
            },
        })
    }
}
impl ::treesitter_types::Spanned for ScriptElement<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        self.span
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SelfClosingTag<'tree> {
    pub span: ::treesitter_types::Span,
    pub children: ::std::vec::Vec<SelfClosingTagChildren<'tree>>,
}
impl<'tree> ::treesitter_types::FromNode<'tree> for SelfClosingTag<'tree> {
    #[allow(clippy::match_single_binding, clippy::suspicious_else_formatting)]
    fn from_node(
        node: ::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        debug_assert_eq!(node.kind(), "self_closing_tag");
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
                    items.push(::treesitter_types::runtime::maybe_grow_stack(|| {
                        <SelfClosingTagChildren as ::treesitter_types::FromNode>::from_node(
                            child, src,
                        )
                    })?);
                }
                items
            },
        })
    }
}
impl ::treesitter_types::Spanned for SelfClosingTag<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        self.span
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct StartTag<'tree> {
    pub span: ::treesitter_types::Span,
    pub children: ::std::vec::Vec<StartTagChildren<'tree>>,
}
impl<'tree> ::treesitter_types::FromNode<'tree> for StartTag<'tree> {
    #[allow(clippy::match_single_binding, clippy::suspicious_else_formatting)]
    fn from_node(
        node: ::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        debug_assert_eq!(node.kind(), "start_tag");
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
                    items.push(::treesitter_types::runtime::maybe_grow_stack(|| {
                        <StartTagChildren as ::treesitter_types::FromNode>::from_node(child, src)
                    })?);
                }
                items
            },
        })
    }
}
impl ::treesitter_types::Spanned for StartTag<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        self.span
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct StyleElement<'tree> {
    pub span: ::treesitter_types::Span,
    pub children: ::std::vec::Vec<StyleElementChildren<'tree>>,
}
impl<'tree> ::treesitter_types::FromNode<'tree> for StyleElement<'tree> {
    #[allow(clippy::match_single_binding, clippy::suspicious_else_formatting)]
    fn from_node(
        node: ::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        debug_assert_eq!(node.kind(), "style_element");
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
                    items.push(::treesitter_types::runtime::maybe_grow_stack(|| {
                        <StyleElementChildren as ::treesitter_types::FromNode>::from_node(
                            child, src,
                        )
                    })?);
                }
                items
            },
        })
    }
}
impl ::treesitter_types::Spanned for StyleElement<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        self.span
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct AttributeName<'tree> {
    pub span: ::treesitter_types::Span,
    text: &'tree str,
}
impl<'tree> ::treesitter_types::FromNode<'tree> for AttributeName<'tree> {
    fn from_node(
        node: ::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        debug_assert_eq!(node.kind(), "attribute_name");
        Ok(Self {
            span: ::treesitter_types::Span::from(node),
            text: node.utf8_text(src)?,
        })
    }
}
impl<'tree> ::treesitter_types::LeafNode<'tree> for AttributeName<'tree> {
    fn text(&self) -> &'tree str {
        self.text
    }
}
impl ::treesitter_types::Spanned for AttributeName<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        self.span
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct AttributeValue<'tree> {
    pub span: ::treesitter_types::Span,
    text: &'tree str,
}
impl<'tree> ::treesitter_types::FromNode<'tree> for AttributeValue<'tree> {
    fn from_node(
        node: ::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        debug_assert_eq!(node.kind(), "attribute_value");
        Ok(Self {
            span: ::treesitter_types::Span::from(node),
            text: node.utf8_text(src)?,
        })
    }
}
impl<'tree> ::treesitter_types::LeafNode<'tree> for AttributeValue<'tree> {
    fn text(&self) -> &'tree str {
        self.text
    }
}
impl ::treesitter_types::Spanned for AttributeValue<'_> {
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
pub struct Entity<'tree> {
    pub span: ::treesitter_types::Span,
    text: &'tree str,
}
impl<'tree> ::treesitter_types::FromNode<'tree> for Entity<'tree> {
    fn from_node(
        node: ::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        debug_assert_eq!(node.kind(), "entity");
        Ok(Self {
            span: ::treesitter_types::Span::from(node),
            text: node.utf8_text(src)?,
        })
    }
}
impl<'tree> ::treesitter_types::LeafNode<'tree> for Entity<'tree> {
    fn text(&self) -> &'tree str {
        self.text
    }
}
impl ::treesitter_types::Spanned for Entity<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        self.span
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ErroneousEndTagName<'tree> {
    pub span: ::treesitter_types::Span,
    text: &'tree str,
}
impl<'tree> ::treesitter_types::FromNode<'tree> for ErroneousEndTagName<'tree> {
    fn from_node(
        node: ::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        debug_assert_eq!(node.kind(), "erroneous_end_tag_name");
        Ok(Self {
            span: ::treesitter_types::Span::from(node),
            text: node.utf8_text(src)?,
        })
    }
}
impl<'tree> ::treesitter_types::LeafNode<'tree> for ErroneousEndTagName<'tree> {
    fn text(&self) -> &'tree str {
        self.text
    }
}
impl ::treesitter_types::Spanned for ErroneousEndTagName<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        self.span
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct RawText<'tree> {
    pub span: ::treesitter_types::Span,
    text: &'tree str,
}
impl<'tree> ::treesitter_types::FromNode<'tree> for RawText<'tree> {
    fn from_node(
        node: ::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        debug_assert_eq!(node.kind(), "raw_text");
        Ok(Self {
            span: ::treesitter_types::Span::from(node),
            text: node.utf8_text(src)?,
        })
    }
}
impl<'tree> ::treesitter_types::LeafNode<'tree> for RawText<'tree> {
    fn text(&self) -> &'tree str {
        self.text
    }
}
impl ::treesitter_types::Spanned for RawText<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        self.span
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct TagName<'tree> {
    pub span: ::treesitter_types::Span,
    text: &'tree str,
}
impl<'tree> ::treesitter_types::FromNode<'tree> for TagName<'tree> {
    fn from_node(
        node: ::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        debug_assert_eq!(node.kind(), "tag_name");
        Ok(Self {
            span: ::treesitter_types::Span::from(node),
            text: node.utf8_text(src)?,
        })
    }
}
impl<'tree> ::treesitter_types::LeafNode<'tree> for TagName<'tree> {
    fn text(&self) -> &'tree str {
        self.text
    }
}
impl ::treesitter_types::Spanned for TagName<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        self.span
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Text<'tree> {
    pub span: ::treesitter_types::Span,
    text: &'tree str,
}
impl<'tree> ::treesitter_types::FromNode<'tree> for Text<'tree> {
    fn from_node(
        node: ::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        debug_assert_eq!(node.kind(), "text");
        Ok(Self {
            span: ::treesitter_types::Span::from(node),
            text: node.utf8_text(src)?,
        })
    }
}
impl<'tree> ::treesitter_types::LeafNode<'tree> for Text<'tree> {
    fn text(&self) -> &'tree str {
        self.text
    }
}
impl ::treesitter_types::Spanned for Text<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        self.span
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum AttributeChildren<'tree> {
    AttributeName(::std::boxed::Box<AttributeName<'tree>>),
    AttributeValue(::std::boxed::Box<AttributeValue<'tree>>),
    QuotedAttributeValue(::std::boxed::Box<QuotedAttributeValue<'tree>>),
}
impl<'tree> ::treesitter_types::FromNode<'tree> for AttributeChildren<'tree> {
    #[allow(clippy::collapsible_else_if)]
    fn from_node(
        node: ::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        match node.kind() {
            "attribute_name" => Ok(Self::AttributeName(::std::boxed::Box::new(
                ::treesitter_types::runtime::maybe_grow_stack(|| {
                    <AttributeName as ::treesitter_types::FromNode>::from_node(node, src)
                })?,
            ))),
            "attribute_value" => Ok(Self::AttributeValue(::std::boxed::Box::new(
                ::treesitter_types::runtime::maybe_grow_stack(|| {
                    <AttributeValue as ::treesitter_types::FromNode>::from_node(node, src)
                })?,
            ))),
            "quoted_attribute_value" => Ok(Self::QuotedAttributeValue(::std::boxed::Box::new(
                ::treesitter_types::runtime::maybe_grow_stack(|| {
                    <QuotedAttributeValue as ::treesitter_types::FromNode>::from_node(node, src)
                })?,
            ))),
            other => Err(::treesitter_types::ParseError::unexpected_kind(other, node)),
        }
    }
}
impl ::treesitter_types::Spanned for AttributeChildren<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        match self {
            Self::AttributeName(inner) => inner.span(),
            Self::AttributeValue(inner) => inner.span(),
            Self::QuotedAttributeValue(inner) => inner.span(),
        }
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum DocumentChildren<'tree> {
    Doctype(::std::boxed::Box<Doctype<'tree>>),
    Element(::std::boxed::Box<Element<'tree>>),
    Entity(::std::boxed::Box<Entity<'tree>>),
    ErroneousEndTag(::std::boxed::Box<ErroneousEndTag<'tree>>),
    ScriptElement(::std::boxed::Box<ScriptElement<'tree>>),
    StyleElement(::std::boxed::Box<StyleElement<'tree>>),
    Text(::std::boxed::Box<Text<'tree>>),
}
impl<'tree> ::treesitter_types::FromNode<'tree> for DocumentChildren<'tree> {
    #[allow(clippy::collapsible_else_if)]
    fn from_node(
        node: ::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        match node.kind() {
            "doctype" => Ok(Self::Doctype(::std::boxed::Box::new(
                ::treesitter_types::runtime::maybe_grow_stack(|| {
                    <Doctype as ::treesitter_types::FromNode>::from_node(node, src)
                })?,
            ))),
            "element" => Ok(Self::Element(::std::boxed::Box::new(
                ::treesitter_types::runtime::maybe_grow_stack(|| {
                    <Element as ::treesitter_types::FromNode>::from_node(node, src)
                })?,
            ))),
            "entity" => Ok(Self::Entity(::std::boxed::Box::new(
                ::treesitter_types::runtime::maybe_grow_stack(|| {
                    <Entity as ::treesitter_types::FromNode>::from_node(node, src)
                })?,
            ))),
            "erroneous_end_tag" => Ok(Self::ErroneousEndTag(::std::boxed::Box::new(
                ::treesitter_types::runtime::maybe_grow_stack(|| {
                    <ErroneousEndTag as ::treesitter_types::FromNode>::from_node(node, src)
                })?,
            ))),
            "script_element" => Ok(Self::ScriptElement(::std::boxed::Box::new(
                ::treesitter_types::runtime::maybe_grow_stack(|| {
                    <ScriptElement as ::treesitter_types::FromNode>::from_node(node, src)
                })?,
            ))),
            "style_element" => Ok(Self::StyleElement(::std::boxed::Box::new(
                ::treesitter_types::runtime::maybe_grow_stack(|| {
                    <StyleElement as ::treesitter_types::FromNode>::from_node(node, src)
                })?,
            ))),
            "text" => Ok(Self::Text(::std::boxed::Box::new(
                ::treesitter_types::runtime::maybe_grow_stack(|| {
                    <Text as ::treesitter_types::FromNode>::from_node(node, src)
                })?,
            ))),
            other => Err(::treesitter_types::ParseError::unexpected_kind(other, node)),
        }
    }
}
impl ::treesitter_types::Spanned for DocumentChildren<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        match self {
            Self::Doctype(inner) => inner.span(),
            Self::Element(inner) => inner.span(),
            Self::Entity(inner) => inner.span(),
            Self::ErroneousEndTag(inner) => inner.span(),
            Self::ScriptElement(inner) => inner.span(),
            Self::StyleElement(inner) => inner.span(),
            Self::Text(inner) => inner.span(),
        }
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ElementChildren<'tree> {
    Doctype(::std::boxed::Box<Doctype<'tree>>),
    Element(::std::boxed::Box<Element<'tree>>),
    EndTag(::std::boxed::Box<EndTag<'tree>>),
    Entity(::std::boxed::Box<Entity<'tree>>),
    ErroneousEndTag(::std::boxed::Box<ErroneousEndTag<'tree>>),
    ScriptElement(::std::boxed::Box<ScriptElement<'tree>>),
    SelfClosingTag(::std::boxed::Box<SelfClosingTag<'tree>>),
    StartTag(::std::boxed::Box<StartTag<'tree>>),
    StyleElement(::std::boxed::Box<StyleElement<'tree>>),
    Text(::std::boxed::Box<Text<'tree>>),
}
impl<'tree> ::treesitter_types::FromNode<'tree> for ElementChildren<'tree> {
    #[allow(clippy::collapsible_else_if)]
    fn from_node(
        node: ::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        match node.kind() {
            "doctype" => Ok(Self::Doctype(::std::boxed::Box::new(
                ::treesitter_types::runtime::maybe_grow_stack(|| {
                    <Doctype as ::treesitter_types::FromNode>::from_node(node, src)
                })?,
            ))),
            "element" => Ok(Self::Element(::std::boxed::Box::new(
                ::treesitter_types::runtime::maybe_grow_stack(|| {
                    <Element as ::treesitter_types::FromNode>::from_node(node, src)
                })?,
            ))),
            "end_tag" => Ok(Self::EndTag(::std::boxed::Box::new(
                ::treesitter_types::runtime::maybe_grow_stack(|| {
                    <EndTag as ::treesitter_types::FromNode>::from_node(node, src)
                })?,
            ))),
            "entity" => Ok(Self::Entity(::std::boxed::Box::new(
                ::treesitter_types::runtime::maybe_grow_stack(|| {
                    <Entity as ::treesitter_types::FromNode>::from_node(node, src)
                })?,
            ))),
            "erroneous_end_tag" => Ok(Self::ErroneousEndTag(::std::boxed::Box::new(
                ::treesitter_types::runtime::maybe_grow_stack(|| {
                    <ErroneousEndTag as ::treesitter_types::FromNode>::from_node(node, src)
                })?,
            ))),
            "script_element" => Ok(Self::ScriptElement(::std::boxed::Box::new(
                ::treesitter_types::runtime::maybe_grow_stack(|| {
                    <ScriptElement as ::treesitter_types::FromNode>::from_node(node, src)
                })?,
            ))),
            "self_closing_tag" => Ok(Self::SelfClosingTag(::std::boxed::Box::new(
                ::treesitter_types::runtime::maybe_grow_stack(|| {
                    <SelfClosingTag as ::treesitter_types::FromNode>::from_node(node, src)
                })?,
            ))),
            "start_tag" => Ok(Self::StartTag(::std::boxed::Box::new(
                ::treesitter_types::runtime::maybe_grow_stack(|| {
                    <StartTag as ::treesitter_types::FromNode>::from_node(node, src)
                })?,
            ))),
            "style_element" => Ok(Self::StyleElement(::std::boxed::Box::new(
                ::treesitter_types::runtime::maybe_grow_stack(|| {
                    <StyleElement as ::treesitter_types::FromNode>::from_node(node, src)
                })?,
            ))),
            "text" => Ok(Self::Text(::std::boxed::Box::new(
                ::treesitter_types::runtime::maybe_grow_stack(|| {
                    <Text as ::treesitter_types::FromNode>::from_node(node, src)
                })?,
            ))),
            other => Err(::treesitter_types::ParseError::unexpected_kind(other, node)),
        }
    }
}
impl ::treesitter_types::Spanned for ElementChildren<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        match self {
            Self::Doctype(inner) => inner.span(),
            Self::Element(inner) => inner.span(),
            Self::EndTag(inner) => inner.span(),
            Self::Entity(inner) => inner.span(),
            Self::ErroneousEndTag(inner) => inner.span(),
            Self::ScriptElement(inner) => inner.span(),
            Self::SelfClosingTag(inner) => inner.span(),
            Self::StartTag(inner) => inner.span(),
            Self::StyleElement(inner) => inner.span(),
            Self::Text(inner) => inner.span(),
        }
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ScriptElementChildren<'tree> {
    EndTag(::std::boxed::Box<EndTag<'tree>>),
    RawText(::std::boxed::Box<RawText<'tree>>),
    StartTag(::std::boxed::Box<StartTag<'tree>>),
}
impl<'tree> ::treesitter_types::FromNode<'tree> for ScriptElementChildren<'tree> {
    #[allow(clippy::collapsible_else_if)]
    fn from_node(
        node: ::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        match node.kind() {
            "end_tag" => Ok(Self::EndTag(::std::boxed::Box::new(
                ::treesitter_types::runtime::maybe_grow_stack(|| {
                    <EndTag as ::treesitter_types::FromNode>::from_node(node, src)
                })?,
            ))),
            "raw_text" => Ok(Self::RawText(::std::boxed::Box::new(
                ::treesitter_types::runtime::maybe_grow_stack(|| {
                    <RawText as ::treesitter_types::FromNode>::from_node(node, src)
                })?,
            ))),
            "start_tag" => Ok(Self::StartTag(::std::boxed::Box::new(
                ::treesitter_types::runtime::maybe_grow_stack(|| {
                    <StartTag as ::treesitter_types::FromNode>::from_node(node, src)
                })?,
            ))),
            other => Err(::treesitter_types::ParseError::unexpected_kind(other, node)),
        }
    }
}
impl ::treesitter_types::Spanned for ScriptElementChildren<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        match self {
            Self::EndTag(inner) => inner.span(),
            Self::RawText(inner) => inner.span(),
            Self::StartTag(inner) => inner.span(),
        }
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum SelfClosingTagChildren<'tree> {
    Attribute(::std::boxed::Box<Attribute<'tree>>),
    TagName(::std::boxed::Box<TagName<'tree>>),
}
impl<'tree> ::treesitter_types::FromNode<'tree> for SelfClosingTagChildren<'tree> {
    #[allow(clippy::collapsible_else_if)]
    fn from_node(
        node: ::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        match node.kind() {
            "attribute" => Ok(Self::Attribute(::std::boxed::Box::new(
                ::treesitter_types::runtime::maybe_grow_stack(|| {
                    <Attribute as ::treesitter_types::FromNode>::from_node(node, src)
                })?,
            ))),
            "tag_name" => Ok(Self::TagName(::std::boxed::Box::new(
                ::treesitter_types::runtime::maybe_grow_stack(|| {
                    <TagName as ::treesitter_types::FromNode>::from_node(node, src)
                })?,
            ))),
            other => Err(::treesitter_types::ParseError::unexpected_kind(other, node)),
        }
    }
}
impl ::treesitter_types::Spanned for SelfClosingTagChildren<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        match self {
            Self::Attribute(inner) => inner.span(),
            Self::TagName(inner) => inner.span(),
        }
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum StartTagChildren<'tree> {
    Attribute(::std::boxed::Box<Attribute<'tree>>),
    TagName(::std::boxed::Box<TagName<'tree>>),
}
impl<'tree> ::treesitter_types::FromNode<'tree> for StartTagChildren<'tree> {
    #[allow(clippy::collapsible_else_if)]
    fn from_node(
        node: ::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        match node.kind() {
            "attribute" => Ok(Self::Attribute(::std::boxed::Box::new(
                ::treesitter_types::runtime::maybe_grow_stack(|| {
                    <Attribute as ::treesitter_types::FromNode>::from_node(node, src)
                })?,
            ))),
            "tag_name" => Ok(Self::TagName(::std::boxed::Box::new(
                ::treesitter_types::runtime::maybe_grow_stack(|| {
                    <TagName as ::treesitter_types::FromNode>::from_node(node, src)
                })?,
            ))),
            other => Err(::treesitter_types::ParseError::unexpected_kind(other, node)),
        }
    }
}
impl ::treesitter_types::Spanned for StartTagChildren<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        match self {
            Self::Attribute(inner) => inner.span(),
            Self::TagName(inner) => inner.span(),
        }
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum StyleElementChildren<'tree> {
    EndTag(::std::boxed::Box<EndTag<'tree>>),
    RawText(::std::boxed::Box<RawText<'tree>>),
    StartTag(::std::boxed::Box<StartTag<'tree>>),
}
impl<'tree> ::treesitter_types::FromNode<'tree> for StyleElementChildren<'tree> {
    #[allow(clippy::collapsible_else_if)]
    fn from_node(
        node: ::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        match node.kind() {
            "end_tag" => Ok(Self::EndTag(::std::boxed::Box::new(
                ::treesitter_types::runtime::maybe_grow_stack(|| {
                    <EndTag as ::treesitter_types::FromNode>::from_node(node, src)
                })?,
            ))),
            "raw_text" => Ok(Self::RawText(::std::boxed::Box::new(
                ::treesitter_types::runtime::maybe_grow_stack(|| {
                    <RawText as ::treesitter_types::FromNode>::from_node(node, src)
                })?,
            ))),
            "start_tag" => Ok(Self::StartTag(::std::boxed::Box::new(
                ::treesitter_types::runtime::maybe_grow_stack(|| {
                    <StartTag as ::treesitter_types::FromNode>::from_node(node, src)
                })?,
            ))),
            other => Err(::treesitter_types::ParseError::unexpected_kind(other, node)),
        }
    }
}
impl ::treesitter_types::Spanned for StyleElementChildren<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        match self {
            Self::EndTag(inner) => inner.span(),
            Self::RawText(inner) => inner.span(),
            Self::StartTag(inner) => inner.span(),
        }
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum AnyNode<'tree> {
    Attribute(Attribute<'tree>),
    Doctype(Doctype<'tree>),
    Document(Document<'tree>),
    Element(Element<'tree>),
    EndTag(EndTag<'tree>),
    ErroneousEndTag(ErroneousEndTag<'tree>),
    QuotedAttributeValue(QuotedAttributeValue<'tree>),
    ScriptElement(ScriptElement<'tree>),
    SelfClosingTag(SelfClosingTag<'tree>),
    StartTag(StartTag<'tree>),
    StyleElement(StyleElement<'tree>),
    AttributeName(AttributeName<'tree>),
    AttributeValue(AttributeValue<'tree>),
    Comment(Comment<'tree>),
    Entity(Entity<'tree>),
    ErroneousEndTagName(ErroneousEndTagName<'tree>),
    RawText(RawText<'tree>),
    TagName(TagName<'tree>),
    Text(Text<'tree>),
    Unknown(::tree_sitter::Node<'tree>),
}
impl<'tree> AnyNode<'tree> {
    pub fn from_node(node: ::tree_sitter::Node<'tree>, src: &'tree [u8]) -> Self {
        match node.kind() {
            "attribute" => ::treesitter_types::runtime::maybe_grow_stack(|| {
                <Attribute as ::treesitter_types::FromNode>::from_node(node, src)
            })
            .map(Self::Attribute)
            .unwrap_or(Self::Unknown(node)),
            "doctype" => ::treesitter_types::runtime::maybe_grow_stack(|| {
                <Doctype as ::treesitter_types::FromNode>::from_node(node, src)
            })
            .map(Self::Doctype)
            .unwrap_or(Self::Unknown(node)),
            "document" => ::treesitter_types::runtime::maybe_grow_stack(|| {
                <Document as ::treesitter_types::FromNode>::from_node(node, src)
            })
            .map(Self::Document)
            .unwrap_or(Self::Unknown(node)),
            "element" => ::treesitter_types::runtime::maybe_grow_stack(|| {
                <Element as ::treesitter_types::FromNode>::from_node(node, src)
            })
            .map(Self::Element)
            .unwrap_or(Self::Unknown(node)),
            "end_tag" => ::treesitter_types::runtime::maybe_grow_stack(|| {
                <EndTag as ::treesitter_types::FromNode>::from_node(node, src)
            })
            .map(Self::EndTag)
            .unwrap_or(Self::Unknown(node)),
            "erroneous_end_tag" => ::treesitter_types::runtime::maybe_grow_stack(|| {
                <ErroneousEndTag as ::treesitter_types::FromNode>::from_node(node, src)
            })
            .map(Self::ErroneousEndTag)
            .unwrap_or(Self::Unknown(node)),
            "quoted_attribute_value" => ::treesitter_types::runtime::maybe_grow_stack(|| {
                <QuotedAttributeValue as ::treesitter_types::FromNode>::from_node(node, src)
            })
            .map(Self::QuotedAttributeValue)
            .unwrap_or(Self::Unknown(node)),
            "script_element" => ::treesitter_types::runtime::maybe_grow_stack(|| {
                <ScriptElement as ::treesitter_types::FromNode>::from_node(node, src)
            })
            .map(Self::ScriptElement)
            .unwrap_or(Self::Unknown(node)),
            "self_closing_tag" => ::treesitter_types::runtime::maybe_grow_stack(|| {
                <SelfClosingTag as ::treesitter_types::FromNode>::from_node(node, src)
            })
            .map(Self::SelfClosingTag)
            .unwrap_or(Self::Unknown(node)),
            "start_tag" => ::treesitter_types::runtime::maybe_grow_stack(|| {
                <StartTag as ::treesitter_types::FromNode>::from_node(node, src)
            })
            .map(Self::StartTag)
            .unwrap_or(Self::Unknown(node)),
            "style_element" => ::treesitter_types::runtime::maybe_grow_stack(|| {
                <StyleElement as ::treesitter_types::FromNode>::from_node(node, src)
            })
            .map(Self::StyleElement)
            .unwrap_or(Self::Unknown(node)),
            "attribute_name" => ::treesitter_types::runtime::maybe_grow_stack(|| {
                <AttributeName as ::treesitter_types::FromNode>::from_node(node, src)
            })
            .map(Self::AttributeName)
            .unwrap_or(Self::Unknown(node)),
            "attribute_value" => ::treesitter_types::runtime::maybe_grow_stack(|| {
                <AttributeValue as ::treesitter_types::FromNode>::from_node(node, src)
            })
            .map(Self::AttributeValue)
            .unwrap_or(Self::Unknown(node)),
            "comment" => ::treesitter_types::runtime::maybe_grow_stack(|| {
                <Comment as ::treesitter_types::FromNode>::from_node(node, src)
            })
            .map(Self::Comment)
            .unwrap_or(Self::Unknown(node)),
            "entity" => ::treesitter_types::runtime::maybe_grow_stack(|| {
                <Entity as ::treesitter_types::FromNode>::from_node(node, src)
            })
            .map(Self::Entity)
            .unwrap_or(Self::Unknown(node)),
            "erroneous_end_tag_name" => ::treesitter_types::runtime::maybe_grow_stack(|| {
                <ErroneousEndTagName as ::treesitter_types::FromNode>::from_node(node, src)
            })
            .map(Self::ErroneousEndTagName)
            .unwrap_or(Self::Unknown(node)),
            "raw_text" => ::treesitter_types::runtime::maybe_grow_stack(|| {
                <RawText as ::treesitter_types::FromNode>::from_node(node, src)
            })
            .map(Self::RawText)
            .unwrap_or(Self::Unknown(node)),
            "tag_name" => ::treesitter_types::runtime::maybe_grow_stack(|| {
                <TagName as ::treesitter_types::FromNode>::from_node(node, src)
            })
            .map(Self::TagName)
            .unwrap_or(Self::Unknown(node)),
            "text" => ::treesitter_types::runtime::maybe_grow_stack(|| {
                <Text as ::treesitter_types::FromNode>::from_node(node, src)
            })
            .map(Self::Text)
            .unwrap_or(Self::Unknown(node)),
            _ => Self::Unknown(node),
        }
    }
}
impl ::treesitter_types::Spanned for AnyNode<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        match self {
            Self::Attribute(inner) => inner.span(),
            Self::Doctype(inner) => inner.span(),
            Self::Document(inner) => inner.span(),
            Self::Element(inner) => inner.span(),
            Self::EndTag(inner) => inner.span(),
            Self::ErroneousEndTag(inner) => inner.span(),
            Self::QuotedAttributeValue(inner) => inner.span(),
            Self::ScriptElement(inner) => inner.span(),
            Self::SelfClosingTag(inner) => inner.span(),
            Self::StartTag(inner) => inner.span(),
            Self::StyleElement(inner) => inner.span(),
            Self::AttributeName(inner) => inner.span(),
            Self::AttributeValue(inner) => inner.span(),
            Self::Comment(inner) => inner.span(),
            Self::Entity(inner) => inner.span(),
            Self::ErroneousEndTagName(inner) => inner.span(),
            Self::RawText(inner) => inner.span(),
            Self::TagName(inner) => inner.span(),
            Self::Text(inner) => inner.span(),
            Self::Unknown(node) => ::treesitter_types::Span::from(*node),
        }
    }
}
