#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Alias<'tree> {
    pub span: ::treesitter_types::Span,
    pub children: AliasName<'tree>,
}
impl<'tree> ::treesitter_types::FromNode<'tree> for Alias<'tree> {
    #[allow(clippy::match_single_binding, clippy::suspicious_else_formatting)]
    fn from_node(
        node: ::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        debug_assert_eq!(node.kind(), "alias");
        Ok(Self {
            span: ::treesitter_types::Span::from(node),
            children: {
                #[allow(clippy::suspicious_else_formatting)]
                let non_field_children = {
                    let mut cursor = node.walk();
                    let mut result = ::std::vec::Vec::new();
                    if cursor.goto_first_child() {
                        loop {
                            if cursor.field_name().is_none() && cursor.node().is_named()
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
                                        <AliasName as ::treesitter_types::FromNode>::from_node(
                                            child,
                                            src,
                                        )?,
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
                                            <AliasName as ::treesitter_types::FromNode>::from_node(
                                                child,
                                                src,
                                            )?,
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
                    fallback_child
                        .ok_or_else(|| ::treesitter_types::ParseError::missing_field(
                            "children",
                            node,
                        ))?
                };
                <AliasName as ::treesitter_types::FromNode>::from_node(child, src)?
            },
        })
    }
}
impl ::treesitter_types::Spanned for Alias<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        self.span
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Anchor<'tree> {
    pub span: ::treesitter_types::Span,
    pub children: AnchorName<'tree>,
}
impl<'tree> ::treesitter_types::FromNode<'tree> for Anchor<'tree> {
    #[allow(clippy::match_single_binding, clippy::suspicious_else_formatting)]
    fn from_node(
        node: ::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        debug_assert_eq!(node.kind(), "anchor");
        Ok(Self {
            span: ::treesitter_types::Span::from(node),
            children: {
                #[allow(clippy::suspicious_else_formatting)]
                let non_field_children = {
                    let mut cursor = node.walk();
                    let mut result = ::std::vec::Vec::new();
                    if cursor.goto_first_child() {
                        loop {
                            if cursor.field_name().is_none() && cursor.node().is_named()
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
                                        <AnchorName as ::treesitter_types::FromNode>::from_node(
                                            child,
                                            src,
                                        )?,
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
                                            <AnchorName as ::treesitter_types::FromNode>::from_node(
                                                child,
                                                src,
                                            )?,
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
                    fallback_child
                        .ok_or_else(|| ::treesitter_types::ParseError::missing_field(
                            "children",
                            node,
                        ))?
                };
                <AnchorName as ::treesitter_types::FromNode>::from_node(child, src)?
            },
        })
    }
}
impl ::treesitter_types::Spanned for Anchor<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        self.span
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct BlockMapping<'tree> {
    pub span: ::treesitter_types::Span,
    pub children: ::std::vec::Vec<BlockMappingPair<'tree>>,
}
impl<'tree> ::treesitter_types::FromNode<'tree> for BlockMapping<'tree> {
    #[allow(clippy::match_single_binding, clippy::suspicious_else_formatting)]
    fn from_node(
        node: ::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        debug_assert_eq!(node.kind(), "block_mapping");
        Ok(Self {
            span: ::treesitter_types::Span::from(node),
            children: {
                #[allow(clippy::suspicious_else_formatting)]
                let non_field_children = {
                    let mut cursor = node.walk();
                    let mut result = ::std::vec::Vec::new();
                    if cursor.goto_first_child() {
                        loop {
                            if cursor.field_name().is_none() && cursor.node().is_named()
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
                    items
                        .push(
                            <BlockMappingPair as ::treesitter_types::FromNode>::from_node(
                                child,
                                src,
                            )?,
                        );
                }
                items
            },
        })
    }
}
impl ::treesitter_types::Spanned for BlockMapping<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        self.span
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct BlockMappingPair<'tree> {
    pub span: ::treesitter_types::Span,
    pub key: ::core::option::Option<BlockMappingPairKey<'tree>>,
    pub value: ::core::option::Option<BlockMappingPairValue<'tree>>,
}
impl<'tree> ::treesitter_types::FromNode<'tree> for BlockMappingPair<'tree> {
    #[allow(clippy::match_single_binding, clippy::suspicious_else_formatting)]
    fn from_node(
        node: ::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        debug_assert_eq!(node.kind(), "block_mapping_pair");
        Ok(Self {
            span: ::treesitter_types::Span::from(node),
            key: match node.child_by_field_name("key") {
                Some(child) => {
                    Some(
                        <BlockMappingPairKey as ::treesitter_types::FromNode>::from_node(
                            child,
                            src,
                        )?,
                    )
                }
                None => None,
            },
            value: match node.child_by_field_name("value") {
                Some(child) => {
                    Some(
                        <BlockMappingPairValue as ::treesitter_types::FromNode>::from_node(
                            child,
                            src,
                        )?,
                    )
                }
                None => None,
            },
        })
    }
}
impl ::treesitter_types::Spanned for BlockMappingPair<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        self.span
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct BlockNode<'tree> {
    pub span: ::treesitter_types::Span,
    pub children: ::std::vec::Vec<BlockNodeChildren<'tree>>,
}
impl<'tree> ::treesitter_types::FromNode<'tree> for BlockNode<'tree> {
    #[allow(clippy::match_single_binding, clippy::suspicious_else_formatting)]
    fn from_node(
        node: ::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        debug_assert_eq!(node.kind(), "block_node");
        Ok(Self {
            span: ::treesitter_types::Span::from(node),
            children: {
                #[allow(clippy::suspicious_else_formatting)]
                let non_field_children = {
                    let mut cursor = node.walk();
                    let mut result = ::std::vec::Vec::new();
                    if cursor.goto_first_child() {
                        loop {
                            if cursor.field_name().is_none() && cursor.node().is_named()
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
                    items
                        .push(
                            <BlockNodeChildren as ::treesitter_types::FromNode>::from_node(
                                child,
                                src,
                            )?,
                        );
                }
                items
            },
        })
    }
}
impl ::treesitter_types::Spanned for BlockNode<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        self.span
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct BlockScalar<'tree> {
    pub span: ::treesitter_types::Span,
    text: &'tree str,
}
impl<'tree> ::treesitter_types::FromNode<'tree> for BlockScalar<'tree> {
    fn from_node(
        node: ::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        debug_assert_eq!(node.kind(), "block_scalar");
        Ok(Self {
            span: ::treesitter_types::Span::from(node),
            text: node.utf8_text(src)?,
        })
    }
}
impl<'tree> ::treesitter_types::LeafNode<'tree> for BlockScalar<'tree> {
    fn text(&self) -> &'tree str {
        self.text
    }
}
impl ::treesitter_types::Spanned for BlockScalar<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        self.span
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct BlockSequence<'tree> {
    pub span: ::treesitter_types::Span,
    pub children: ::std::vec::Vec<BlockSequenceItem<'tree>>,
}
impl<'tree> ::treesitter_types::FromNode<'tree> for BlockSequence<'tree> {
    #[allow(clippy::match_single_binding, clippy::suspicious_else_formatting)]
    fn from_node(
        node: ::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        debug_assert_eq!(node.kind(), "block_sequence");
        Ok(Self {
            span: ::treesitter_types::Span::from(node),
            children: {
                #[allow(clippy::suspicious_else_formatting)]
                let non_field_children = {
                    let mut cursor = node.walk();
                    let mut result = ::std::vec::Vec::new();
                    if cursor.goto_first_child() {
                        loop {
                            if cursor.field_name().is_none() && cursor.node().is_named()
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
                    items
                        .push(
                            <BlockSequenceItem as ::treesitter_types::FromNode>::from_node(
                                child,
                                src,
                            )?,
                        );
                }
                items
            },
        })
    }
}
impl ::treesitter_types::Spanned for BlockSequence<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        self.span
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct BlockSequenceItem<'tree> {
    pub span: ::treesitter_types::Span,
    pub children: ::core::option::Option<BlockSequenceItemChildren<'tree>>,
}
impl<'tree> ::treesitter_types::FromNode<'tree> for BlockSequenceItem<'tree> {
    #[allow(clippy::match_single_binding, clippy::suspicious_else_formatting)]
    fn from_node(
        node: ::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        debug_assert_eq!(node.kind(), "block_sequence_item");
        Ok(Self {
            span: ::treesitter_types::Span::from(node),
            children: {
                #[allow(clippy::suspicious_else_formatting)]
                let non_field_children = {
                    let mut cursor = node.walk();
                    let mut result = ::std::vec::Vec::new();
                    if cursor.goto_first_child() {
                        loop {
                            if cursor.field_name().is_none() && cursor.node().is_named()
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
                    Some(&child) => {
                        Some(
                            <BlockSequenceItemChildren as ::treesitter_types::FromNode>::from_node(
                                child,
                                src,
                            )?,
                        )
                    }
                    None => None,
                }
            },
        })
    }
}
impl ::treesitter_types::Spanned for BlockSequenceItem<'_> {
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
                            if cursor.field_name().is_none() && cursor.node().is_named()
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
                    items
                        .push(
                            <DocumentChildren as ::treesitter_types::FromNode>::from_node(
                                child,
                                src,
                            )?,
                        );
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
pub struct DoubleQuoteScalar<'tree> {
    pub span: ::treesitter_types::Span,
    pub children: ::std::vec::Vec<EscapeSequence<'tree>>,
}
impl<'tree> ::treesitter_types::FromNode<'tree> for DoubleQuoteScalar<'tree> {
    #[allow(clippy::match_single_binding, clippy::suspicious_else_formatting)]
    fn from_node(
        node: ::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        debug_assert_eq!(node.kind(), "double_quote_scalar");
        Ok(Self {
            span: ::treesitter_types::Span::from(node),
            children: {
                #[allow(clippy::suspicious_else_formatting)]
                let non_field_children = {
                    let mut cursor = node.walk();
                    let mut result = ::std::vec::Vec::new();
                    if cursor.goto_first_child() {
                        loop {
                            if cursor.field_name().is_none() && cursor.node().is_named()
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
                    items
                        .push(
                            <EscapeSequence as ::treesitter_types::FromNode>::from_node(
                                child,
                                src,
                            )?,
                        );
                }
                items
            },
        })
    }
}
impl ::treesitter_types::Spanned for DoubleQuoteScalar<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        self.span
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct FlowMapping<'tree> {
    pub span: ::treesitter_types::Span,
    pub children: ::std::vec::Vec<FlowMappingChildren<'tree>>,
}
impl<'tree> ::treesitter_types::FromNode<'tree> for FlowMapping<'tree> {
    #[allow(clippy::match_single_binding, clippy::suspicious_else_formatting)]
    fn from_node(
        node: ::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        debug_assert_eq!(node.kind(), "flow_mapping");
        Ok(Self {
            span: ::treesitter_types::Span::from(node),
            children: {
                #[allow(clippy::suspicious_else_formatting)]
                let non_field_children = {
                    let mut cursor = node.walk();
                    let mut result = ::std::vec::Vec::new();
                    if cursor.goto_first_child() {
                        loop {
                            if cursor.field_name().is_none() && cursor.node().is_named()
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
                    items
                        .push(
                            <FlowMappingChildren as ::treesitter_types::FromNode>::from_node(
                                child,
                                src,
                            )?,
                        );
                }
                items
            },
        })
    }
}
impl ::treesitter_types::Spanned for FlowMapping<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        self.span
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct FlowNode<'tree> {
    pub span: ::treesitter_types::Span,
    pub children: ::std::vec::Vec<FlowNodeChildren<'tree>>,
}
impl<'tree> ::treesitter_types::FromNode<'tree> for FlowNode<'tree> {
    #[allow(clippy::match_single_binding, clippy::suspicious_else_formatting)]
    fn from_node(
        node: ::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        debug_assert_eq!(node.kind(), "flow_node");
        Ok(Self {
            span: ::treesitter_types::Span::from(node),
            children: {
                #[allow(clippy::suspicious_else_formatting)]
                let non_field_children = {
                    let mut cursor = node.walk();
                    let mut result = ::std::vec::Vec::new();
                    if cursor.goto_first_child() {
                        loop {
                            if cursor.field_name().is_none() && cursor.node().is_named()
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
                    items
                        .push(
                            <FlowNodeChildren as ::treesitter_types::FromNode>::from_node(
                                child,
                                src,
                            )?,
                        );
                }
                items
            },
        })
    }
}
impl ::treesitter_types::Spanned for FlowNode<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        self.span
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct FlowPair<'tree> {
    pub span: ::treesitter_types::Span,
    pub key: ::core::option::Option<FlowNode<'tree>>,
    pub value: ::core::option::Option<FlowNode<'tree>>,
}
impl<'tree> ::treesitter_types::FromNode<'tree> for FlowPair<'tree> {
    #[allow(clippy::match_single_binding, clippy::suspicious_else_formatting)]
    fn from_node(
        node: ::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        debug_assert_eq!(node.kind(), "flow_pair");
        Ok(Self {
            span: ::treesitter_types::Span::from(node),
            key: match node.child_by_field_name("key") {
                Some(child) => {
                    Some(
                        <FlowNode as ::treesitter_types::FromNode>::from_node(
                            child,
                            src,
                        )?,
                    )
                }
                None => None,
            },
            value: match node.child_by_field_name("value") {
                Some(child) => {
                    Some(
                        <FlowNode as ::treesitter_types::FromNode>::from_node(
                            child,
                            src,
                        )?,
                    )
                }
                None => None,
            },
        })
    }
}
impl ::treesitter_types::Spanned for FlowPair<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        self.span
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct FlowSequence<'tree> {
    pub span: ::treesitter_types::Span,
    pub children: ::std::vec::Vec<FlowSequenceChildren<'tree>>,
}
impl<'tree> ::treesitter_types::FromNode<'tree> for FlowSequence<'tree> {
    #[allow(clippy::match_single_binding, clippy::suspicious_else_formatting)]
    fn from_node(
        node: ::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        debug_assert_eq!(node.kind(), "flow_sequence");
        Ok(Self {
            span: ::treesitter_types::Span::from(node),
            children: {
                #[allow(clippy::suspicious_else_formatting)]
                let non_field_children = {
                    let mut cursor = node.walk();
                    let mut result = ::std::vec::Vec::new();
                    if cursor.goto_first_child() {
                        loop {
                            if cursor.field_name().is_none() && cursor.node().is_named()
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
                    items
                        .push(
                            <FlowSequenceChildren as ::treesitter_types::FromNode>::from_node(
                                child,
                                src,
                            )?,
                        );
                }
                items
            },
        })
    }
}
impl ::treesitter_types::Spanned for FlowSequence<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        self.span
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PlainScalar<'tree> {
    pub span: ::treesitter_types::Span,
    pub children: PlainScalarChildren<'tree>,
}
impl<'tree> ::treesitter_types::FromNode<'tree> for PlainScalar<'tree> {
    #[allow(clippy::match_single_binding, clippy::suspicious_else_formatting)]
    fn from_node(
        node: ::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        debug_assert_eq!(node.kind(), "plain_scalar");
        Ok(Self {
            span: ::treesitter_types::Span::from(node),
            children: {
                #[allow(clippy::suspicious_else_formatting)]
                let non_field_children = {
                    let mut cursor = node.walk();
                    let mut result = ::std::vec::Vec::new();
                    if cursor.goto_first_child() {
                        loop {
                            if cursor.field_name().is_none() && cursor.node().is_named()
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
                                        <PlainScalarChildren as ::treesitter_types::FromNode>::from_node(
                                            child,
                                            src,
                                        )?,
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
                                            <PlainScalarChildren as ::treesitter_types::FromNode>::from_node(
                                                child,
                                                src,
                                            )?,
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
                    fallback_child
                        .ok_or_else(|| ::treesitter_types::ParseError::missing_field(
                            "children",
                            node,
                        ))?
                };
                <PlainScalarChildren as ::treesitter_types::FromNode>::from_node(
                    child,
                    src,
                )?
            },
        })
    }
}
impl ::treesitter_types::Spanned for PlainScalar<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        self.span
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ReservedDirective<'tree> {
    pub span: ::treesitter_types::Span,
    pub children: ::std::vec::Vec<ReservedDirectiveChildren<'tree>>,
}
impl<'tree> ::treesitter_types::FromNode<'tree> for ReservedDirective<'tree> {
    #[allow(clippy::match_single_binding, clippy::suspicious_else_formatting)]
    fn from_node(
        node: ::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        debug_assert_eq!(node.kind(), "reserved_directive");
        Ok(Self {
            span: ::treesitter_types::Span::from(node),
            children: {
                #[allow(clippy::suspicious_else_formatting)]
                let non_field_children = {
                    let mut cursor = node.walk();
                    let mut result = ::std::vec::Vec::new();
                    if cursor.goto_first_child() {
                        loop {
                            if cursor.field_name().is_none() && cursor.node().is_named()
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
                    items
                        .push(
                            <ReservedDirectiveChildren as ::treesitter_types::FromNode>::from_node(
                                child,
                                src,
                            )?,
                        );
                }
                items
            },
        })
    }
}
impl ::treesitter_types::Spanned for ReservedDirective<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        self.span
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SingleQuoteScalar<'tree> {
    pub span: ::treesitter_types::Span,
    pub children: ::std::vec::Vec<EscapeSequence<'tree>>,
}
impl<'tree> ::treesitter_types::FromNode<'tree> for SingleQuoteScalar<'tree> {
    #[allow(clippy::match_single_binding, clippy::suspicious_else_formatting)]
    fn from_node(
        node: ::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        debug_assert_eq!(node.kind(), "single_quote_scalar");
        Ok(Self {
            span: ::treesitter_types::Span::from(node),
            children: {
                #[allow(clippy::suspicious_else_formatting)]
                let non_field_children = {
                    let mut cursor = node.walk();
                    let mut result = ::std::vec::Vec::new();
                    if cursor.goto_first_child() {
                        loop {
                            if cursor.field_name().is_none() && cursor.node().is_named()
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
                    items
                        .push(
                            <EscapeSequence as ::treesitter_types::FromNode>::from_node(
                                child,
                                src,
                            )?,
                        );
                }
                items
            },
        })
    }
}
impl ::treesitter_types::Spanned for SingleQuoteScalar<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        self.span
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Stream<'tree> {
    pub span: ::treesitter_types::Span,
    pub children: ::std::vec::Vec<Document<'tree>>,
}
impl<'tree> ::treesitter_types::FromNode<'tree> for Stream<'tree> {
    #[allow(clippy::match_single_binding, clippy::suspicious_else_formatting)]
    fn from_node(
        node: ::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        debug_assert_eq!(node.kind(), "stream");
        Ok(Self {
            span: ::treesitter_types::Span::from(node),
            children: {
                #[allow(clippy::suspicious_else_formatting)]
                let non_field_children = {
                    let mut cursor = node.walk();
                    let mut result = ::std::vec::Vec::new();
                    if cursor.goto_first_child() {
                        loop {
                            if cursor.field_name().is_none() && cursor.node().is_named()
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
                    items
                        .push(
                            <Document as ::treesitter_types::FromNode>::from_node(
                                child,
                                src,
                            )?,
                        );
                }
                items
            },
        })
    }
}
impl ::treesitter_types::Spanned for Stream<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        self.span
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct TagDirective<'tree> {
    pub span: ::treesitter_types::Span,
    pub children: ::std::vec::Vec<TagDirectiveChildren<'tree>>,
}
impl<'tree> ::treesitter_types::FromNode<'tree> for TagDirective<'tree> {
    #[allow(clippy::match_single_binding, clippy::suspicious_else_formatting)]
    fn from_node(
        node: ::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        debug_assert_eq!(node.kind(), "tag_directive");
        Ok(Self {
            span: ::treesitter_types::Span::from(node),
            children: {
                #[allow(clippy::suspicious_else_formatting)]
                let non_field_children = {
                    let mut cursor = node.walk();
                    let mut result = ::std::vec::Vec::new();
                    if cursor.goto_first_child() {
                        loop {
                            if cursor.field_name().is_none() && cursor.node().is_named()
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
                    items
                        .push(
                            <TagDirectiveChildren as ::treesitter_types::FromNode>::from_node(
                                child,
                                src,
                            )?,
                        );
                }
                items
            },
        })
    }
}
impl ::treesitter_types::Spanned for TagDirective<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        self.span
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct YamlDirective<'tree> {
    pub span: ::treesitter_types::Span,
    pub children: YamlVersion<'tree>,
}
impl<'tree> ::treesitter_types::FromNode<'tree> for YamlDirective<'tree> {
    #[allow(clippy::match_single_binding, clippy::suspicious_else_formatting)]
    fn from_node(
        node: ::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        debug_assert_eq!(node.kind(), "yaml_directive");
        Ok(Self {
            span: ::treesitter_types::Span::from(node),
            children: {
                #[allow(clippy::suspicious_else_formatting)]
                let non_field_children = {
                    let mut cursor = node.walk();
                    let mut result = ::std::vec::Vec::new();
                    if cursor.goto_first_child() {
                        loop {
                            if cursor.field_name().is_none() && cursor.node().is_named()
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
                                        <YamlVersion as ::treesitter_types::FromNode>::from_node(
                                            child,
                                            src,
                                        )?,
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
                                            <YamlVersion as ::treesitter_types::FromNode>::from_node(
                                                child,
                                                src,
                                            )?,
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
                    fallback_child
                        .ok_or_else(|| ::treesitter_types::ParseError::missing_field(
                            "children",
                            node,
                        ))?
                };
                <YamlVersion as ::treesitter_types::FromNode>::from_node(child, src)?
            },
        })
    }
}
impl ::treesitter_types::Spanned for YamlDirective<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        self.span
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct AliasName<'tree> {
    pub span: ::treesitter_types::Span,
    text: &'tree str,
}
impl<'tree> ::treesitter_types::FromNode<'tree> for AliasName<'tree> {
    fn from_node(
        node: ::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        debug_assert_eq!(node.kind(), "alias_name");
        Ok(Self {
            span: ::treesitter_types::Span::from(node),
            text: node.utf8_text(src)?,
        })
    }
}
impl<'tree> ::treesitter_types::LeafNode<'tree> for AliasName<'tree> {
    fn text(&self) -> &'tree str {
        self.text
    }
}
impl ::treesitter_types::Spanned for AliasName<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        self.span
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct AnchorName<'tree> {
    pub span: ::treesitter_types::Span,
    text: &'tree str,
}
impl<'tree> ::treesitter_types::FromNode<'tree> for AnchorName<'tree> {
    fn from_node(
        node: ::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        debug_assert_eq!(node.kind(), "anchor_name");
        Ok(Self {
            span: ::treesitter_types::Span::from(node),
            text: node.utf8_text(src)?,
        })
    }
}
impl<'tree> ::treesitter_types::LeafNode<'tree> for AnchorName<'tree> {
    fn text(&self) -> &'tree str {
        self.text
    }
}
impl ::treesitter_types::Spanned for AnchorName<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        self.span
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct BooleanScalar<'tree> {
    pub span: ::treesitter_types::Span,
    text: &'tree str,
}
impl<'tree> ::treesitter_types::FromNode<'tree> for BooleanScalar<'tree> {
    fn from_node(
        node: ::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        debug_assert_eq!(node.kind(), "boolean_scalar");
        Ok(Self {
            span: ::treesitter_types::Span::from(node),
            text: node.utf8_text(src)?,
        })
    }
}
impl<'tree> ::treesitter_types::LeafNode<'tree> for BooleanScalar<'tree> {
    fn text(&self) -> &'tree str {
        self.text
    }
}
impl ::treesitter_types::Spanned for BooleanScalar<'_> {
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
pub struct DirectiveName<'tree> {
    pub span: ::treesitter_types::Span,
    text: &'tree str,
}
impl<'tree> ::treesitter_types::FromNode<'tree> for DirectiveName<'tree> {
    fn from_node(
        node: ::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        debug_assert_eq!(node.kind(), "directive_name");
        Ok(Self {
            span: ::treesitter_types::Span::from(node),
            text: node.utf8_text(src)?,
        })
    }
}
impl<'tree> ::treesitter_types::LeafNode<'tree> for DirectiveName<'tree> {
    fn text(&self) -> &'tree str {
        self.text
    }
}
impl ::treesitter_types::Spanned for DirectiveName<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        self.span
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct DirectiveParameter<'tree> {
    pub span: ::treesitter_types::Span,
    text: &'tree str,
}
impl<'tree> ::treesitter_types::FromNode<'tree> for DirectiveParameter<'tree> {
    fn from_node(
        node: ::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        debug_assert_eq!(node.kind(), "directive_parameter");
        Ok(Self {
            span: ::treesitter_types::Span::from(node),
            text: node.utf8_text(src)?,
        })
    }
}
impl<'tree> ::treesitter_types::LeafNode<'tree> for DirectiveParameter<'tree> {
    fn text(&self) -> &'tree str {
        self.text
    }
}
impl ::treesitter_types::Spanned for DirectiveParameter<'_> {
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
pub struct FloatScalar<'tree> {
    pub span: ::treesitter_types::Span,
    text: &'tree str,
}
impl<'tree> ::treesitter_types::FromNode<'tree> for FloatScalar<'tree> {
    fn from_node(
        node: ::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        debug_assert_eq!(node.kind(), "float_scalar");
        Ok(Self {
            span: ::treesitter_types::Span::from(node),
            text: node.utf8_text(src)?,
        })
    }
}
impl<'tree> ::treesitter_types::LeafNode<'tree> for FloatScalar<'tree> {
    fn text(&self) -> &'tree str {
        self.text
    }
}
impl ::treesitter_types::Spanned for FloatScalar<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        self.span
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct IntegerScalar<'tree> {
    pub span: ::treesitter_types::Span,
    text: &'tree str,
}
impl<'tree> ::treesitter_types::FromNode<'tree> for IntegerScalar<'tree> {
    fn from_node(
        node: ::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        debug_assert_eq!(node.kind(), "integer_scalar");
        Ok(Self {
            span: ::treesitter_types::Span::from(node),
            text: node.utf8_text(src)?,
        })
    }
}
impl<'tree> ::treesitter_types::LeafNode<'tree> for IntegerScalar<'tree> {
    fn text(&self) -> &'tree str {
        self.text
    }
}
impl ::treesitter_types::Spanned for IntegerScalar<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        self.span
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct NullScalar<'tree> {
    pub span: ::treesitter_types::Span,
    text: &'tree str,
}
impl<'tree> ::treesitter_types::FromNode<'tree> for NullScalar<'tree> {
    fn from_node(
        node: ::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        debug_assert_eq!(node.kind(), "null_scalar");
        Ok(Self {
            span: ::treesitter_types::Span::from(node),
            text: node.utf8_text(src)?,
        })
    }
}
impl<'tree> ::treesitter_types::LeafNode<'tree> for NullScalar<'tree> {
    fn text(&self) -> &'tree str {
        self.text
    }
}
impl ::treesitter_types::Spanned for NullScalar<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        self.span
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct StringScalar<'tree> {
    pub span: ::treesitter_types::Span,
    text: &'tree str,
}
impl<'tree> ::treesitter_types::FromNode<'tree> for StringScalar<'tree> {
    fn from_node(
        node: ::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        debug_assert_eq!(node.kind(), "string_scalar");
        Ok(Self {
            span: ::treesitter_types::Span::from(node),
            text: node.utf8_text(src)?,
        })
    }
}
impl<'tree> ::treesitter_types::LeafNode<'tree> for StringScalar<'tree> {
    fn text(&self) -> &'tree str {
        self.text
    }
}
impl ::treesitter_types::Spanned for StringScalar<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        self.span
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Tag<'tree> {
    pub span: ::treesitter_types::Span,
    text: &'tree str,
}
impl<'tree> ::treesitter_types::FromNode<'tree> for Tag<'tree> {
    fn from_node(
        node: ::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        debug_assert_eq!(node.kind(), "tag");
        Ok(Self {
            span: ::treesitter_types::Span::from(node),
            text: node.utf8_text(src)?,
        })
    }
}
impl<'tree> ::treesitter_types::LeafNode<'tree> for Tag<'tree> {
    fn text(&self) -> &'tree str {
        self.text
    }
}
impl ::treesitter_types::Spanned for Tag<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        self.span
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct TagHandle<'tree> {
    pub span: ::treesitter_types::Span,
    text: &'tree str,
}
impl<'tree> ::treesitter_types::FromNode<'tree> for TagHandle<'tree> {
    fn from_node(
        node: ::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        debug_assert_eq!(node.kind(), "tag_handle");
        Ok(Self {
            span: ::treesitter_types::Span::from(node),
            text: node.utf8_text(src)?,
        })
    }
}
impl<'tree> ::treesitter_types::LeafNode<'tree> for TagHandle<'tree> {
    fn text(&self) -> &'tree str {
        self.text
    }
}
impl ::treesitter_types::Spanned for TagHandle<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        self.span
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct TagPrefix<'tree> {
    pub span: ::treesitter_types::Span,
    text: &'tree str,
}
impl<'tree> ::treesitter_types::FromNode<'tree> for TagPrefix<'tree> {
    fn from_node(
        node: ::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        debug_assert_eq!(node.kind(), "tag_prefix");
        Ok(Self {
            span: ::treesitter_types::Span::from(node),
            text: node.utf8_text(src)?,
        })
    }
}
impl<'tree> ::treesitter_types::LeafNode<'tree> for TagPrefix<'tree> {
    fn text(&self) -> &'tree str {
        self.text
    }
}
impl ::treesitter_types::Spanned for TagPrefix<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        self.span
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct TimestampScalar<'tree> {
    pub span: ::treesitter_types::Span,
    text: &'tree str,
}
impl<'tree> ::treesitter_types::FromNode<'tree> for TimestampScalar<'tree> {
    fn from_node(
        node: ::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        debug_assert_eq!(node.kind(), "timestamp_scalar");
        Ok(Self {
            span: ::treesitter_types::Span::from(node),
            text: node.utf8_text(src)?,
        })
    }
}
impl<'tree> ::treesitter_types::LeafNode<'tree> for TimestampScalar<'tree> {
    fn text(&self) -> &'tree str {
        self.text
    }
}
impl ::treesitter_types::Spanned for TimestampScalar<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        self.span
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct YamlVersion<'tree> {
    pub span: ::treesitter_types::Span,
    text: &'tree str,
}
impl<'tree> ::treesitter_types::FromNode<'tree> for YamlVersion<'tree> {
    fn from_node(
        node: ::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        debug_assert_eq!(node.kind(), "yaml_version");
        Ok(Self {
            span: ::treesitter_types::Span::from(node),
            text: node.utf8_text(src)?,
        })
    }
}
impl<'tree> ::treesitter_types::LeafNode<'tree> for YamlVersion<'tree> {
    fn text(&self) -> &'tree str {
        self.text
    }
}
impl ::treesitter_types::Spanned for YamlVersion<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        self.span
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum BlockMappingPairKey<'tree> {
    BlockNode(::std::boxed::Box<BlockNode<'tree>>),
    FlowNode(::std::boxed::Box<FlowNode<'tree>>),
}
impl<'tree> ::treesitter_types::FromNode<'tree> for BlockMappingPairKey<'tree> {
    #[allow(clippy::collapsible_else_if)]
    fn from_node(
        node: ::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        match node.kind() {
            "block_node" => {
                Ok(
                    Self::BlockNode(
                        ::std::boxed::Box::new(
                            <BlockNode as ::treesitter_types::FromNode>::from_node(
                                node,
                                src,
                            )?,
                        ),
                    ),
                )
            }
            "flow_node" => {
                Ok(
                    Self::FlowNode(
                        ::std::boxed::Box::new(
                            <FlowNode as ::treesitter_types::FromNode>::from_node(
                                node,
                                src,
                            )?,
                        ),
                    ),
                )
            }
            other => Err(::treesitter_types::ParseError::unexpected_kind(other, node)),
        }
    }
}
impl ::treesitter_types::Spanned for BlockMappingPairKey<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        match self {
            Self::BlockNode(inner) => inner.span(),
            Self::FlowNode(inner) => inner.span(),
        }
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum BlockMappingPairValue<'tree> {
    BlockNode(::std::boxed::Box<BlockNode<'tree>>),
    FlowNode(::std::boxed::Box<FlowNode<'tree>>),
}
impl<'tree> ::treesitter_types::FromNode<'tree> for BlockMappingPairValue<'tree> {
    #[allow(clippy::collapsible_else_if)]
    fn from_node(
        node: ::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        match node.kind() {
            "block_node" => {
                Ok(
                    Self::BlockNode(
                        ::std::boxed::Box::new(
                            <BlockNode as ::treesitter_types::FromNode>::from_node(
                                node,
                                src,
                            )?,
                        ),
                    ),
                )
            }
            "flow_node" => {
                Ok(
                    Self::FlowNode(
                        ::std::boxed::Box::new(
                            <FlowNode as ::treesitter_types::FromNode>::from_node(
                                node,
                                src,
                            )?,
                        ),
                    ),
                )
            }
            other => Err(::treesitter_types::ParseError::unexpected_kind(other, node)),
        }
    }
}
impl ::treesitter_types::Spanned for BlockMappingPairValue<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        match self {
            Self::BlockNode(inner) => inner.span(),
            Self::FlowNode(inner) => inner.span(),
        }
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum BlockNodeChildren<'tree> {
    Anchor(::std::boxed::Box<Anchor<'tree>>),
    BlockMapping(::std::boxed::Box<BlockMapping<'tree>>),
    BlockScalar(::std::boxed::Box<BlockScalar<'tree>>),
    BlockSequence(::std::boxed::Box<BlockSequence<'tree>>),
    Tag(::std::boxed::Box<Tag<'tree>>),
}
impl<'tree> ::treesitter_types::FromNode<'tree> for BlockNodeChildren<'tree> {
    #[allow(clippy::collapsible_else_if)]
    fn from_node(
        node: ::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        match node.kind() {
            "anchor" => {
                Ok(
                    Self::Anchor(
                        ::std::boxed::Box::new(
                            <Anchor as ::treesitter_types::FromNode>::from_node(
                                node,
                                src,
                            )?,
                        ),
                    ),
                )
            }
            "block_mapping" => {
                Ok(
                    Self::BlockMapping(
                        ::std::boxed::Box::new(
                            <BlockMapping as ::treesitter_types::FromNode>::from_node(
                                node,
                                src,
                            )?,
                        ),
                    ),
                )
            }
            "block_scalar" => {
                Ok(
                    Self::BlockScalar(
                        ::std::boxed::Box::new(
                            <BlockScalar as ::treesitter_types::FromNode>::from_node(
                                node,
                                src,
                            )?,
                        ),
                    ),
                )
            }
            "block_sequence" => {
                Ok(
                    Self::BlockSequence(
                        ::std::boxed::Box::new(
                            <BlockSequence as ::treesitter_types::FromNode>::from_node(
                                node,
                                src,
                            )?,
                        ),
                    ),
                )
            }
            "tag" => {
                Ok(
                    Self::Tag(
                        ::std::boxed::Box::new(
                            <Tag as ::treesitter_types::FromNode>::from_node(node, src)?,
                        ),
                    ),
                )
            }
            other => Err(::treesitter_types::ParseError::unexpected_kind(other, node)),
        }
    }
}
impl ::treesitter_types::Spanned for BlockNodeChildren<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        match self {
            Self::Anchor(inner) => inner.span(),
            Self::BlockMapping(inner) => inner.span(),
            Self::BlockScalar(inner) => inner.span(),
            Self::BlockSequence(inner) => inner.span(),
            Self::Tag(inner) => inner.span(),
        }
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum BlockSequenceItemChildren<'tree> {
    BlockNode(::std::boxed::Box<BlockNode<'tree>>),
    FlowNode(::std::boxed::Box<FlowNode<'tree>>),
}
impl<'tree> ::treesitter_types::FromNode<'tree> for BlockSequenceItemChildren<'tree> {
    #[allow(clippy::collapsible_else_if)]
    fn from_node(
        node: ::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        match node.kind() {
            "block_node" => {
                Ok(
                    Self::BlockNode(
                        ::std::boxed::Box::new(
                            <BlockNode as ::treesitter_types::FromNode>::from_node(
                                node,
                                src,
                            )?,
                        ),
                    ),
                )
            }
            "flow_node" => {
                Ok(
                    Self::FlowNode(
                        ::std::boxed::Box::new(
                            <FlowNode as ::treesitter_types::FromNode>::from_node(
                                node,
                                src,
                            )?,
                        ),
                    ),
                )
            }
            other => Err(::treesitter_types::ParseError::unexpected_kind(other, node)),
        }
    }
}
impl ::treesitter_types::Spanned for BlockSequenceItemChildren<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        match self {
            Self::BlockNode(inner) => inner.span(),
            Self::FlowNode(inner) => inner.span(),
        }
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum DocumentChildren<'tree> {
    BlockNode(::std::boxed::Box<BlockNode<'tree>>),
    FlowNode(::std::boxed::Box<FlowNode<'tree>>),
    ReservedDirective(::std::boxed::Box<ReservedDirective<'tree>>),
    TagDirective(::std::boxed::Box<TagDirective<'tree>>),
    YamlDirective(::std::boxed::Box<YamlDirective<'tree>>),
}
impl<'tree> ::treesitter_types::FromNode<'tree> for DocumentChildren<'tree> {
    #[allow(clippy::collapsible_else_if)]
    fn from_node(
        node: ::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        match node.kind() {
            "block_node" => {
                Ok(
                    Self::BlockNode(
                        ::std::boxed::Box::new(
                            <BlockNode as ::treesitter_types::FromNode>::from_node(
                                node,
                                src,
                            )?,
                        ),
                    ),
                )
            }
            "flow_node" => {
                Ok(
                    Self::FlowNode(
                        ::std::boxed::Box::new(
                            <FlowNode as ::treesitter_types::FromNode>::from_node(
                                node,
                                src,
                            )?,
                        ),
                    ),
                )
            }
            "reserved_directive" => {
                Ok(
                    Self::ReservedDirective(
                        ::std::boxed::Box::new(
                            <ReservedDirective as ::treesitter_types::FromNode>::from_node(
                                node,
                                src,
                            )?,
                        ),
                    ),
                )
            }
            "tag_directive" => {
                Ok(
                    Self::TagDirective(
                        ::std::boxed::Box::new(
                            <TagDirective as ::treesitter_types::FromNode>::from_node(
                                node,
                                src,
                            )?,
                        ),
                    ),
                )
            }
            "yaml_directive" => {
                Ok(
                    Self::YamlDirective(
                        ::std::boxed::Box::new(
                            <YamlDirective as ::treesitter_types::FromNode>::from_node(
                                node,
                                src,
                            )?,
                        ),
                    ),
                )
            }
            other => Err(::treesitter_types::ParseError::unexpected_kind(other, node)),
        }
    }
}
impl ::treesitter_types::Spanned for DocumentChildren<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        match self {
            Self::BlockNode(inner) => inner.span(),
            Self::FlowNode(inner) => inner.span(),
            Self::ReservedDirective(inner) => inner.span(),
            Self::TagDirective(inner) => inner.span(),
            Self::YamlDirective(inner) => inner.span(),
        }
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum FlowMappingChildren<'tree> {
    FlowNode(::std::boxed::Box<FlowNode<'tree>>),
    FlowPair(::std::boxed::Box<FlowPair<'tree>>),
}
impl<'tree> ::treesitter_types::FromNode<'tree> for FlowMappingChildren<'tree> {
    #[allow(clippy::collapsible_else_if)]
    fn from_node(
        node: ::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        match node.kind() {
            "flow_node" => {
                Ok(
                    Self::FlowNode(
                        ::std::boxed::Box::new(
                            <FlowNode as ::treesitter_types::FromNode>::from_node(
                                node,
                                src,
                            )?,
                        ),
                    ),
                )
            }
            "flow_pair" => {
                Ok(
                    Self::FlowPair(
                        ::std::boxed::Box::new(
                            <FlowPair as ::treesitter_types::FromNode>::from_node(
                                node,
                                src,
                            )?,
                        ),
                    ),
                )
            }
            other => Err(::treesitter_types::ParseError::unexpected_kind(other, node)),
        }
    }
}
impl ::treesitter_types::Spanned for FlowMappingChildren<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        match self {
            Self::FlowNode(inner) => inner.span(),
            Self::FlowPair(inner) => inner.span(),
        }
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum FlowNodeChildren<'tree> {
    Alias(::std::boxed::Box<Alias<'tree>>),
    Anchor(::std::boxed::Box<Anchor<'tree>>),
    DoubleQuoteScalar(::std::boxed::Box<DoubleQuoteScalar<'tree>>),
    FlowMapping(::std::boxed::Box<FlowMapping<'tree>>),
    FlowSequence(::std::boxed::Box<FlowSequence<'tree>>),
    PlainScalar(::std::boxed::Box<PlainScalar<'tree>>),
    SingleQuoteScalar(::std::boxed::Box<SingleQuoteScalar<'tree>>),
    Tag(::std::boxed::Box<Tag<'tree>>),
}
impl<'tree> ::treesitter_types::FromNode<'tree> for FlowNodeChildren<'tree> {
    #[allow(clippy::collapsible_else_if)]
    fn from_node(
        node: ::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        match node.kind() {
            "alias" => {
                Ok(
                    Self::Alias(
                        ::std::boxed::Box::new(
                            <Alias as ::treesitter_types::FromNode>::from_node(
                                node,
                                src,
                            )?,
                        ),
                    ),
                )
            }
            "anchor" => {
                Ok(
                    Self::Anchor(
                        ::std::boxed::Box::new(
                            <Anchor as ::treesitter_types::FromNode>::from_node(
                                node,
                                src,
                            )?,
                        ),
                    ),
                )
            }
            "double_quote_scalar" => {
                Ok(
                    Self::DoubleQuoteScalar(
                        ::std::boxed::Box::new(
                            <DoubleQuoteScalar as ::treesitter_types::FromNode>::from_node(
                                node,
                                src,
                            )?,
                        ),
                    ),
                )
            }
            "flow_mapping" => {
                Ok(
                    Self::FlowMapping(
                        ::std::boxed::Box::new(
                            <FlowMapping as ::treesitter_types::FromNode>::from_node(
                                node,
                                src,
                            )?,
                        ),
                    ),
                )
            }
            "flow_sequence" => {
                Ok(
                    Self::FlowSequence(
                        ::std::boxed::Box::new(
                            <FlowSequence as ::treesitter_types::FromNode>::from_node(
                                node,
                                src,
                            )?,
                        ),
                    ),
                )
            }
            "plain_scalar" => {
                Ok(
                    Self::PlainScalar(
                        ::std::boxed::Box::new(
                            <PlainScalar as ::treesitter_types::FromNode>::from_node(
                                node,
                                src,
                            )?,
                        ),
                    ),
                )
            }
            "single_quote_scalar" => {
                Ok(
                    Self::SingleQuoteScalar(
                        ::std::boxed::Box::new(
                            <SingleQuoteScalar as ::treesitter_types::FromNode>::from_node(
                                node,
                                src,
                            )?,
                        ),
                    ),
                )
            }
            "tag" => {
                Ok(
                    Self::Tag(
                        ::std::boxed::Box::new(
                            <Tag as ::treesitter_types::FromNode>::from_node(node, src)?,
                        ),
                    ),
                )
            }
            other => Err(::treesitter_types::ParseError::unexpected_kind(other, node)),
        }
    }
}
impl ::treesitter_types::Spanned for FlowNodeChildren<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        match self {
            Self::Alias(inner) => inner.span(),
            Self::Anchor(inner) => inner.span(),
            Self::DoubleQuoteScalar(inner) => inner.span(),
            Self::FlowMapping(inner) => inner.span(),
            Self::FlowSequence(inner) => inner.span(),
            Self::PlainScalar(inner) => inner.span(),
            Self::SingleQuoteScalar(inner) => inner.span(),
            Self::Tag(inner) => inner.span(),
        }
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum FlowSequenceChildren<'tree> {
    FlowNode(::std::boxed::Box<FlowNode<'tree>>),
    FlowPair(::std::boxed::Box<FlowPair<'tree>>),
}
impl<'tree> ::treesitter_types::FromNode<'tree> for FlowSequenceChildren<'tree> {
    #[allow(clippy::collapsible_else_if)]
    fn from_node(
        node: ::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        match node.kind() {
            "flow_node" => {
                Ok(
                    Self::FlowNode(
                        ::std::boxed::Box::new(
                            <FlowNode as ::treesitter_types::FromNode>::from_node(
                                node,
                                src,
                            )?,
                        ),
                    ),
                )
            }
            "flow_pair" => {
                Ok(
                    Self::FlowPair(
                        ::std::boxed::Box::new(
                            <FlowPair as ::treesitter_types::FromNode>::from_node(
                                node,
                                src,
                            )?,
                        ),
                    ),
                )
            }
            other => Err(::treesitter_types::ParseError::unexpected_kind(other, node)),
        }
    }
}
impl ::treesitter_types::Spanned for FlowSequenceChildren<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        match self {
            Self::FlowNode(inner) => inner.span(),
            Self::FlowPair(inner) => inner.span(),
        }
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum PlainScalarChildren<'tree> {
    BooleanScalar(::std::boxed::Box<BooleanScalar<'tree>>),
    FloatScalar(::std::boxed::Box<FloatScalar<'tree>>),
    IntegerScalar(::std::boxed::Box<IntegerScalar<'tree>>),
    NullScalar(::std::boxed::Box<NullScalar<'tree>>),
    StringScalar(::std::boxed::Box<StringScalar<'tree>>),
    TimestampScalar(::std::boxed::Box<TimestampScalar<'tree>>),
}
impl<'tree> ::treesitter_types::FromNode<'tree> for PlainScalarChildren<'tree> {
    #[allow(clippy::collapsible_else_if)]
    fn from_node(
        node: ::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        match node.kind() {
            "boolean_scalar" => {
                Ok(
                    Self::BooleanScalar(
                        ::std::boxed::Box::new(
                            <BooleanScalar as ::treesitter_types::FromNode>::from_node(
                                node,
                                src,
                            )?,
                        ),
                    ),
                )
            }
            "float_scalar" => {
                Ok(
                    Self::FloatScalar(
                        ::std::boxed::Box::new(
                            <FloatScalar as ::treesitter_types::FromNode>::from_node(
                                node,
                                src,
                            )?,
                        ),
                    ),
                )
            }
            "integer_scalar" => {
                Ok(
                    Self::IntegerScalar(
                        ::std::boxed::Box::new(
                            <IntegerScalar as ::treesitter_types::FromNode>::from_node(
                                node,
                                src,
                            )?,
                        ),
                    ),
                )
            }
            "null_scalar" => {
                Ok(
                    Self::NullScalar(
                        ::std::boxed::Box::new(
                            <NullScalar as ::treesitter_types::FromNode>::from_node(
                                node,
                                src,
                            )?,
                        ),
                    ),
                )
            }
            "string_scalar" => {
                Ok(
                    Self::StringScalar(
                        ::std::boxed::Box::new(
                            <StringScalar as ::treesitter_types::FromNode>::from_node(
                                node,
                                src,
                            )?,
                        ),
                    ),
                )
            }
            "timestamp_scalar" => {
                Ok(
                    Self::TimestampScalar(
                        ::std::boxed::Box::new(
                            <TimestampScalar as ::treesitter_types::FromNode>::from_node(
                                node,
                                src,
                            )?,
                        ),
                    ),
                )
            }
            other => Err(::treesitter_types::ParseError::unexpected_kind(other, node)),
        }
    }
}
impl ::treesitter_types::Spanned for PlainScalarChildren<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        match self {
            Self::BooleanScalar(inner) => inner.span(),
            Self::FloatScalar(inner) => inner.span(),
            Self::IntegerScalar(inner) => inner.span(),
            Self::NullScalar(inner) => inner.span(),
            Self::StringScalar(inner) => inner.span(),
            Self::TimestampScalar(inner) => inner.span(),
        }
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ReservedDirectiveChildren<'tree> {
    DirectiveName(::std::boxed::Box<DirectiveName<'tree>>),
    DirectiveParameter(::std::boxed::Box<DirectiveParameter<'tree>>),
}
impl<'tree> ::treesitter_types::FromNode<'tree> for ReservedDirectiveChildren<'tree> {
    #[allow(clippy::collapsible_else_if)]
    fn from_node(
        node: ::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        match node.kind() {
            "directive_name" => {
                Ok(
                    Self::DirectiveName(
                        ::std::boxed::Box::new(
                            <DirectiveName as ::treesitter_types::FromNode>::from_node(
                                node,
                                src,
                            )?,
                        ),
                    ),
                )
            }
            "directive_parameter" => {
                Ok(
                    Self::DirectiveParameter(
                        ::std::boxed::Box::new(
                            <DirectiveParameter as ::treesitter_types::FromNode>::from_node(
                                node,
                                src,
                            )?,
                        ),
                    ),
                )
            }
            other => Err(::treesitter_types::ParseError::unexpected_kind(other, node)),
        }
    }
}
impl ::treesitter_types::Spanned for ReservedDirectiveChildren<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        match self {
            Self::DirectiveName(inner) => inner.span(),
            Self::DirectiveParameter(inner) => inner.span(),
        }
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum TagDirectiveChildren<'tree> {
    TagHandle(::std::boxed::Box<TagHandle<'tree>>),
    TagPrefix(::std::boxed::Box<TagPrefix<'tree>>),
}
impl<'tree> ::treesitter_types::FromNode<'tree> for TagDirectiveChildren<'tree> {
    #[allow(clippy::collapsible_else_if)]
    fn from_node(
        node: ::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        match node.kind() {
            "tag_handle" => {
                Ok(
                    Self::TagHandle(
                        ::std::boxed::Box::new(
                            <TagHandle as ::treesitter_types::FromNode>::from_node(
                                node,
                                src,
                            )?,
                        ),
                    ),
                )
            }
            "tag_prefix" => {
                Ok(
                    Self::TagPrefix(
                        ::std::boxed::Box::new(
                            <TagPrefix as ::treesitter_types::FromNode>::from_node(
                                node,
                                src,
                            )?,
                        ),
                    ),
                )
            }
            other => Err(::treesitter_types::ParseError::unexpected_kind(other, node)),
        }
    }
}
impl ::treesitter_types::Spanned for TagDirectiveChildren<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        match self {
            Self::TagHandle(inner) => inner.span(),
            Self::TagPrefix(inner) => inner.span(),
        }
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum AnyNode<'tree> {
    Alias(Alias<'tree>),
    Anchor(Anchor<'tree>),
    BlockMapping(BlockMapping<'tree>),
    BlockMappingPair(BlockMappingPair<'tree>),
    BlockNode(BlockNode<'tree>),
    BlockScalar(BlockScalar<'tree>),
    BlockSequence(BlockSequence<'tree>),
    BlockSequenceItem(BlockSequenceItem<'tree>),
    Document(Document<'tree>),
    DoubleQuoteScalar(DoubleQuoteScalar<'tree>),
    FlowMapping(FlowMapping<'tree>),
    FlowNode(FlowNode<'tree>),
    FlowPair(FlowPair<'tree>),
    FlowSequence(FlowSequence<'tree>),
    PlainScalar(PlainScalar<'tree>),
    ReservedDirective(ReservedDirective<'tree>),
    SingleQuoteScalar(SingleQuoteScalar<'tree>),
    Stream(Stream<'tree>),
    TagDirective(TagDirective<'tree>),
    YamlDirective(YamlDirective<'tree>),
    AliasName(AliasName<'tree>),
    AnchorName(AnchorName<'tree>),
    BooleanScalar(BooleanScalar<'tree>),
    Comment(Comment<'tree>),
    DirectiveName(DirectiveName<'tree>),
    DirectiveParameter(DirectiveParameter<'tree>),
    EscapeSequence(EscapeSequence<'tree>),
    FloatScalar(FloatScalar<'tree>),
    IntegerScalar(IntegerScalar<'tree>),
    NullScalar(NullScalar<'tree>),
    StringScalar(StringScalar<'tree>),
    Tag(Tag<'tree>),
    TagHandle(TagHandle<'tree>),
    TagPrefix(TagPrefix<'tree>),
    TimestampScalar(TimestampScalar<'tree>),
    YamlVersion(YamlVersion<'tree>),
    Unknown(::tree_sitter::Node<'tree>),
}
impl<'tree> AnyNode<'tree> {
    pub fn from_node(node: ::tree_sitter::Node<'tree>, src: &'tree [u8]) -> Self {
        match node.kind() {
            "alias" => {
                <Alias as ::treesitter_types::FromNode>::from_node(node, src)
                    .map(Self::Alias)
                    .unwrap_or(Self::Unknown(node))
            }
            "anchor" => {
                <Anchor as ::treesitter_types::FromNode>::from_node(node, src)
                    .map(Self::Anchor)
                    .unwrap_or(Self::Unknown(node))
            }
            "block_mapping" => {
                <BlockMapping as ::treesitter_types::FromNode>::from_node(node, src)
                    .map(Self::BlockMapping)
                    .unwrap_or(Self::Unknown(node))
            }
            "block_mapping_pair" => {
                <BlockMappingPair as ::treesitter_types::FromNode>::from_node(node, src)
                    .map(Self::BlockMappingPair)
                    .unwrap_or(Self::Unknown(node))
            }
            "block_node" => {
                <BlockNode as ::treesitter_types::FromNode>::from_node(node, src)
                    .map(Self::BlockNode)
                    .unwrap_or(Self::Unknown(node))
            }
            "block_scalar" => {
                <BlockScalar as ::treesitter_types::FromNode>::from_node(node, src)
                    .map(Self::BlockScalar)
                    .unwrap_or(Self::Unknown(node))
            }
            "block_sequence" => {
                <BlockSequence as ::treesitter_types::FromNode>::from_node(node, src)
                    .map(Self::BlockSequence)
                    .unwrap_or(Self::Unknown(node))
            }
            "block_sequence_item" => {
                <BlockSequenceItem as ::treesitter_types::FromNode>::from_node(node, src)
                    .map(Self::BlockSequenceItem)
                    .unwrap_or(Self::Unknown(node))
            }
            "document" => {
                <Document as ::treesitter_types::FromNode>::from_node(node, src)
                    .map(Self::Document)
                    .unwrap_or(Self::Unknown(node))
            }
            "double_quote_scalar" => {
                <DoubleQuoteScalar as ::treesitter_types::FromNode>::from_node(node, src)
                    .map(Self::DoubleQuoteScalar)
                    .unwrap_or(Self::Unknown(node))
            }
            "flow_mapping" => {
                <FlowMapping as ::treesitter_types::FromNode>::from_node(node, src)
                    .map(Self::FlowMapping)
                    .unwrap_or(Self::Unknown(node))
            }
            "flow_node" => {
                <FlowNode as ::treesitter_types::FromNode>::from_node(node, src)
                    .map(Self::FlowNode)
                    .unwrap_or(Self::Unknown(node))
            }
            "flow_pair" => {
                <FlowPair as ::treesitter_types::FromNode>::from_node(node, src)
                    .map(Self::FlowPair)
                    .unwrap_or(Self::Unknown(node))
            }
            "flow_sequence" => {
                <FlowSequence as ::treesitter_types::FromNode>::from_node(node, src)
                    .map(Self::FlowSequence)
                    .unwrap_or(Self::Unknown(node))
            }
            "plain_scalar" => {
                <PlainScalar as ::treesitter_types::FromNode>::from_node(node, src)
                    .map(Self::PlainScalar)
                    .unwrap_or(Self::Unknown(node))
            }
            "reserved_directive" => {
                <ReservedDirective as ::treesitter_types::FromNode>::from_node(node, src)
                    .map(Self::ReservedDirective)
                    .unwrap_or(Self::Unknown(node))
            }
            "single_quote_scalar" => {
                <SingleQuoteScalar as ::treesitter_types::FromNode>::from_node(node, src)
                    .map(Self::SingleQuoteScalar)
                    .unwrap_or(Self::Unknown(node))
            }
            "stream" => {
                <Stream as ::treesitter_types::FromNode>::from_node(node, src)
                    .map(Self::Stream)
                    .unwrap_or(Self::Unknown(node))
            }
            "tag_directive" => {
                <TagDirective as ::treesitter_types::FromNode>::from_node(node, src)
                    .map(Self::TagDirective)
                    .unwrap_or(Self::Unknown(node))
            }
            "yaml_directive" => {
                <YamlDirective as ::treesitter_types::FromNode>::from_node(node, src)
                    .map(Self::YamlDirective)
                    .unwrap_or(Self::Unknown(node))
            }
            "alias_name" => {
                <AliasName as ::treesitter_types::FromNode>::from_node(node, src)
                    .map(Self::AliasName)
                    .unwrap_or(Self::Unknown(node))
            }
            "anchor_name" => {
                <AnchorName as ::treesitter_types::FromNode>::from_node(node, src)
                    .map(Self::AnchorName)
                    .unwrap_or(Self::Unknown(node))
            }
            "boolean_scalar" => {
                <BooleanScalar as ::treesitter_types::FromNode>::from_node(node, src)
                    .map(Self::BooleanScalar)
                    .unwrap_or(Self::Unknown(node))
            }
            "comment" => {
                <Comment as ::treesitter_types::FromNode>::from_node(node, src)
                    .map(Self::Comment)
                    .unwrap_or(Self::Unknown(node))
            }
            "directive_name" => {
                <DirectiveName as ::treesitter_types::FromNode>::from_node(node, src)
                    .map(Self::DirectiveName)
                    .unwrap_or(Self::Unknown(node))
            }
            "directive_parameter" => {
                <DirectiveParameter as ::treesitter_types::FromNode>::from_node(
                        node,
                        src,
                    )
                    .map(Self::DirectiveParameter)
                    .unwrap_or(Self::Unknown(node))
            }
            "escape_sequence" => {
                <EscapeSequence as ::treesitter_types::FromNode>::from_node(node, src)
                    .map(Self::EscapeSequence)
                    .unwrap_or(Self::Unknown(node))
            }
            "float_scalar" => {
                <FloatScalar as ::treesitter_types::FromNode>::from_node(node, src)
                    .map(Self::FloatScalar)
                    .unwrap_or(Self::Unknown(node))
            }
            "integer_scalar" => {
                <IntegerScalar as ::treesitter_types::FromNode>::from_node(node, src)
                    .map(Self::IntegerScalar)
                    .unwrap_or(Self::Unknown(node))
            }
            "null_scalar" => {
                <NullScalar as ::treesitter_types::FromNode>::from_node(node, src)
                    .map(Self::NullScalar)
                    .unwrap_or(Self::Unknown(node))
            }
            "string_scalar" => {
                <StringScalar as ::treesitter_types::FromNode>::from_node(node, src)
                    .map(Self::StringScalar)
                    .unwrap_or(Self::Unknown(node))
            }
            "tag" => {
                <Tag as ::treesitter_types::FromNode>::from_node(node, src)
                    .map(Self::Tag)
                    .unwrap_or(Self::Unknown(node))
            }
            "tag_handle" => {
                <TagHandle as ::treesitter_types::FromNode>::from_node(node, src)
                    .map(Self::TagHandle)
                    .unwrap_or(Self::Unknown(node))
            }
            "tag_prefix" => {
                <TagPrefix as ::treesitter_types::FromNode>::from_node(node, src)
                    .map(Self::TagPrefix)
                    .unwrap_or(Self::Unknown(node))
            }
            "timestamp_scalar" => {
                <TimestampScalar as ::treesitter_types::FromNode>::from_node(node, src)
                    .map(Self::TimestampScalar)
                    .unwrap_or(Self::Unknown(node))
            }
            "yaml_version" => {
                <YamlVersion as ::treesitter_types::FromNode>::from_node(node, src)
                    .map(Self::YamlVersion)
                    .unwrap_or(Self::Unknown(node))
            }
            _ => Self::Unknown(node),
        }
    }
}
impl ::treesitter_types::Spanned for AnyNode<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        match self {
            Self::Alias(inner) => inner.span(),
            Self::Anchor(inner) => inner.span(),
            Self::BlockMapping(inner) => inner.span(),
            Self::BlockMappingPair(inner) => inner.span(),
            Self::BlockNode(inner) => inner.span(),
            Self::BlockScalar(inner) => inner.span(),
            Self::BlockSequence(inner) => inner.span(),
            Self::BlockSequenceItem(inner) => inner.span(),
            Self::Document(inner) => inner.span(),
            Self::DoubleQuoteScalar(inner) => inner.span(),
            Self::FlowMapping(inner) => inner.span(),
            Self::FlowNode(inner) => inner.span(),
            Self::FlowPair(inner) => inner.span(),
            Self::FlowSequence(inner) => inner.span(),
            Self::PlainScalar(inner) => inner.span(),
            Self::ReservedDirective(inner) => inner.span(),
            Self::SingleQuoteScalar(inner) => inner.span(),
            Self::Stream(inner) => inner.span(),
            Self::TagDirective(inner) => inner.span(),
            Self::YamlDirective(inner) => inner.span(),
            Self::AliasName(inner) => inner.span(),
            Self::AnchorName(inner) => inner.span(),
            Self::BooleanScalar(inner) => inner.span(),
            Self::Comment(inner) => inner.span(),
            Self::DirectiveName(inner) => inner.span(),
            Self::DirectiveParameter(inner) => inner.span(),
            Self::EscapeSequence(inner) => inner.span(),
            Self::FloatScalar(inner) => inner.span(),
            Self::IntegerScalar(inner) => inner.span(),
            Self::NullScalar(inner) => inner.span(),
            Self::StringScalar(inner) => inner.span(),
            Self::Tag(inner) => inner.span(),
            Self::TagHandle(inner) => inner.span(),
            Self::TagPrefix(inner) => inner.span(),
            Self::TimestampScalar(inner) => inner.span(),
            Self::YamlVersion(inner) => inner.span(),
            Self::Unknown(node) => ::treesitter_types::Span::from(*node),
        }
    }
}
