#[derive(Debug, Clone)]
pub struct AtxHeading<'tree> {
    pub span: ::treesitter_types::Span,
    pub heading_content: ::core::option::Option<Inline<'tree>>,
    pub children: ::std::vec::Vec<AtxHeadingChildren<'tree>>,
}
impl<'tree> ::treesitter_types::FromNode<'tree> for AtxHeading<'tree> {
    #[allow(clippy::match_single_binding, clippy::suspicious_else_formatting)]
    fn from_node(
        node: ::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        debug_assert_eq!(node.kind(), "atx_heading");
        Ok(Self {
            span: ::treesitter_types::Span::from(node),
            heading_content: match node.child_by_field_name("heading_content") {
                Some(child) => {
                    Some(
                        <Inline as ::treesitter_types::FromNode>::from_node(child, src)?,
                    )
                }
                None => None,
            },
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
                            <AtxHeadingChildren as ::treesitter_types::FromNode>::from_node(
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
impl ::treesitter_types::Spanned for AtxHeading<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        self.span
    }
}
#[derive(Debug, Clone)]
pub struct BackslashEscape<'tree> {
    pub span: ::treesitter_types::Span,
    text: &'tree str,
}
impl<'tree> ::treesitter_types::FromNode<'tree> for BackslashEscape<'tree> {
    fn from_node(
        node: ::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        debug_assert_eq!(node.kind(), "backslash_escape");
        Ok(Self {
            span: ::treesitter_types::Span::from(node),
            text: node.utf8_text(src)?,
        })
    }
}
impl<'tree> ::treesitter_types::LeafNode<'tree> for BackslashEscape<'tree> {
    fn text(&self) -> &'tree str {
        self.text
    }
}
impl ::treesitter_types::Spanned for BackslashEscape<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        self.span
    }
}
#[derive(Debug, Clone)]
pub struct BlockQuote<'tree> {
    pub span: ::treesitter_types::Span,
    pub children: ::std::vec::Vec<BlockQuoteChildren<'tree>>,
}
impl<'tree> ::treesitter_types::FromNode<'tree> for BlockQuote<'tree> {
    #[allow(clippy::match_single_binding, clippy::suspicious_else_formatting)]
    fn from_node(
        node: ::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        debug_assert_eq!(node.kind(), "block_quote");
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
                            <BlockQuoteChildren as ::treesitter_types::FromNode>::from_node(
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
impl ::treesitter_types::Spanned for BlockQuote<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        self.span
    }
}
#[derive(Debug, Clone)]
pub struct CodeFenceContent<'tree> {
    pub span: ::treesitter_types::Span,
    pub children: ::std::vec::Vec<BlockContinuation<'tree>>,
}
impl<'tree> ::treesitter_types::FromNode<'tree> for CodeFenceContent<'tree> {
    #[allow(clippy::match_single_binding, clippy::suspicious_else_formatting)]
    fn from_node(
        node: ::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        debug_assert_eq!(node.kind(), "code_fence_content");
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
                            <BlockContinuation as ::treesitter_types::FromNode>::from_node(
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
impl ::treesitter_types::Spanned for CodeFenceContent<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        self.span
    }
}
#[derive(Debug, Clone)]
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
#[derive(Debug, Clone)]
pub struct FencedCodeBlock<'tree> {
    pub span: ::treesitter_types::Span,
    pub children: ::std::vec::Vec<FencedCodeBlockChildren<'tree>>,
}
impl<'tree> ::treesitter_types::FromNode<'tree> for FencedCodeBlock<'tree> {
    #[allow(clippy::match_single_binding, clippy::suspicious_else_formatting)]
    fn from_node(
        node: ::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        debug_assert_eq!(node.kind(), "fenced_code_block");
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
                            <FencedCodeBlockChildren as ::treesitter_types::FromNode>::from_node(
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
impl ::treesitter_types::Spanned for FencedCodeBlock<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        self.span
    }
}
#[derive(Debug, Clone)]
pub struct HtmlBlock<'tree> {
    pub span: ::treesitter_types::Span,
    pub children: ::std::vec::Vec<BlockContinuation<'tree>>,
}
impl<'tree> ::treesitter_types::FromNode<'tree> for HtmlBlock<'tree> {
    #[allow(clippy::match_single_binding, clippy::suspicious_else_formatting)]
    fn from_node(
        node: ::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        debug_assert_eq!(node.kind(), "html_block");
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
                            <BlockContinuation as ::treesitter_types::FromNode>::from_node(
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
impl ::treesitter_types::Spanned for HtmlBlock<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        self.span
    }
}
#[derive(Debug, Clone)]
pub struct IndentedCodeBlock<'tree> {
    pub span: ::treesitter_types::Span,
    pub children: ::std::vec::Vec<BlockContinuation<'tree>>,
}
impl<'tree> ::treesitter_types::FromNode<'tree> for IndentedCodeBlock<'tree> {
    #[allow(clippy::match_single_binding, clippy::suspicious_else_formatting)]
    fn from_node(
        node: ::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        debug_assert_eq!(node.kind(), "indented_code_block");
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
                            <BlockContinuation as ::treesitter_types::FromNode>::from_node(
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
impl ::treesitter_types::Spanned for IndentedCodeBlock<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        self.span
    }
}
#[derive(Debug, Clone)]
pub struct InfoString<'tree> {
    pub span: ::treesitter_types::Span,
    pub children: ::std::vec::Vec<InfoStringChildren<'tree>>,
}
impl<'tree> ::treesitter_types::FromNode<'tree> for InfoString<'tree> {
    #[allow(clippy::match_single_binding, clippy::suspicious_else_formatting)]
    fn from_node(
        node: ::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        debug_assert_eq!(node.kind(), "info_string");
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
                            <InfoStringChildren as ::treesitter_types::FromNode>::from_node(
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
impl ::treesitter_types::Spanned for InfoString<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        self.span
    }
}
#[derive(Debug, Clone)]
pub struct Inline<'tree> {
    pub span: ::treesitter_types::Span,
    pub children: ::std::vec::Vec<BlockContinuation<'tree>>,
}
impl<'tree> ::treesitter_types::FromNode<'tree> for Inline<'tree> {
    #[allow(clippy::match_single_binding, clippy::suspicious_else_formatting)]
    fn from_node(
        node: ::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        debug_assert_eq!(node.kind(), "inline");
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
                            <BlockContinuation as ::treesitter_types::FromNode>::from_node(
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
impl ::treesitter_types::Spanned for Inline<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        self.span
    }
}
#[derive(Debug, Clone)]
pub struct Language<'tree> {
    pub span: ::treesitter_types::Span,
    pub children: ::std::vec::Vec<LanguageChildren<'tree>>,
}
impl<'tree> ::treesitter_types::FromNode<'tree> for Language<'tree> {
    #[allow(clippy::match_single_binding, clippy::suspicious_else_formatting)]
    fn from_node(
        node: ::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        debug_assert_eq!(node.kind(), "language");
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
                            <LanguageChildren as ::treesitter_types::FromNode>::from_node(
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
impl ::treesitter_types::Spanned for Language<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        self.span
    }
}
#[derive(Debug, Clone)]
pub struct LinkDestination<'tree> {
    pub span: ::treesitter_types::Span,
    pub children: ::std::vec::Vec<LinkDestinationChildren<'tree>>,
}
impl<'tree> ::treesitter_types::FromNode<'tree> for LinkDestination<'tree> {
    #[allow(clippy::match_single_binding, clippy::suspicious_else_formatting)]
    fn from_node(
        node: ::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        debug_assert_eq!(node.kind(), "link_destination");
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
                            <LinkDestinationChildren as ::treesitter_types::FromNode>::from_node(
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
impl ::treesitter_types::Spanned for LinkDestination<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        self.span
    }
}
#[derive(Debug, Clone)]
pub struct LinkLabel<'tree> {
    pub span: ::treesitter_types::Span,
    pub children: ::std::vec::Vec<LinkLabelChildren<'tree>>,
}
impl<'tree> ::treesitter_types::FromNode<'tree> for LinkLabel<'tree> {
    #[allow(clippy::match_single_binding, clippy::suspicious_else_formatting)]
    fn from_node(
        node: ::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        debug_assert_eq!(node.kind(), "link_label");
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
                            <LinkLabelChildren as ::treesitter_types::FromNode>::from_node(
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
impl ::treesitter_types::Spanned for LinkLabel<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        self.span
    }
}
#[derive(Debug, Clone)]
pub struct LinkReferenceDefinition<'tree> {
    pub span: ::treesitter_types::Span,
    pub children: ::std::vec::Vec<LinkReferenceDefinitionChildren<'tree>>,
}
impl<'tree> ::treesitter_types::FromNode<'tree> for LinkReferenceDefinition<'tree> {
    #[allow(clippy::match_single_binding, clippy::suspicious_else_formatting)]
    fn from_node(
        node: ::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        debug_assert_eq!(node.kind(), "link_reference_definition");
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
                            <LinkReferenceDefinitionChildren as ::treesitter_types::FromNode>::from_node(
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
impl ::treesitter_types::Spanned for LinkReferenceDefinition<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        self.span
    }
}
#[derive(Debug, Clone)]
pub struct LinkTitle<'tree> {
    pub span: ::treesitter_types::Span,
    pub children: ::std::vec::Vec<LinkTitleChildren<'tree>>,
}
impl<'tree> ::treesitter_types::FromNode<'tree> for LinkTitle<'tree> {
    #[allow(clippy::match_single_binding, clippy::suspicious_else_formatting)]
    fn from_node(
        node: ::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        debug_assert_eq!(node.kind(), "link_title");
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
                            <LinkTitleChildren as ::treesitter_types::FromNode>::from_node(
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
impl ::treesitter_types::Spanned for LinkTitle<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        self.span
    }
}
#[derive(Debug, Clone)]
pub struct List<'tree> {
    pub span: ::treesitter_types::Span,
    pub children: ::std::vec::Vec<ListItem<'tree>>,
}
impl<'tree> ::treesitter_types::FromNode<'tree> for List<'tree> {
    #[allow(clippy::match_single_binding, clippy::suspicious_else_formatting)]
    fn from_node(
        node: ::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        debug_assert_eq!(node.kind(), "list");
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
                            <ListItem as ::treesitter_types::FromNode>::from_node(
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
impl ::treesitter_types::Spanned for List<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        self.span
    }
}
#[derive(Debug, Clone)]
pub struct ListItem<'tree> {
    pub span: ::treesitter_types::Span,
    pub children: ::std::vec::Vec<ListItemChildren<'tree>>,
}
impl<'tree> ::treesitter_types::FromNode<'tree> for ListItem<'tree> {
    #[allow(clippy::match_single_binding, clippy::suspicious_else_formatting)]
    fn from_node(
        node: ::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        debug_assert_eq!(node.kind(), "list_item");
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
                            <ListItemChildren as ::treesitter_types::FromNode>::from_node(
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
impl ::treesitter_types::Spanned for ListItem<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        self.span
    }
}
#[derive(Debug, Clone)]
pub struct ListMarkerDot<'tree> {
    pub span: ::treesitter_types::Span,
    text: &'tree str,
}
impl<'tree> ::treesitter_types::FromNode<'tree> for ListMarkerDot<'tree> {
    fn from_node(
        node: ::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        debug_assert_eq!(node.kind(), "list_marker_dot");
        Ok(Self {
            span: ::treesitter_types::Span::from(node),
            text: node.utf8_text(src)?,
        })
    }
}
impl<'tree> ::treesitter_types::LeafNode<'tree> for ListMarkerDot<'tree> {
    fn text(&self) -> &'tree str {
        self.text
    }
}
impl ::treesitter_types::Spanned for ListMarkerDot<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        self.span
    }
}
#[derive(Debug, Clone)]
pub struct ListMarkerMinus<'tree> {
    pub span: ::treesitter_types::Span,
    text: &'tree str,
}
impl<'tree> ::treesitter_types::FromNode<'tree> for ListMarkerMinus<'tree> {
    fn from_node(
        node: ::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        debug_assert_eq!(node.kind(), "list_marker_minus");
        Ok(Self {
            span: ::treesitter_types::Span::from(node),
            text: node.utf8_text(src)?,
        })
    }
}
impl<'tree> ::treesitter_types::LeafNode<'tree> for ListMarkerMinus<'tree> {
    fn text(&self) -> &'tree str {
        self.text
    }
}
impl ::treesitter_types::Spanned for ListMarkerMinus<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        self.span
    }
}
#[derive(Debug, Clone)]
pub struct ListMarkerParenthesis<'tree> {
    pub span: ::treesitter_types::Span,
    text: &'tree str,
}
impl<'tree> ::treesitter_types::FromNode<'tree> for ListMarkerParenthesis<'tree> {
    fn from_node(
        node: ::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        debug_assert_eq!(node.kind(), "list_marker_parenthesis");
        Ok(Self {
            span: ::treesitter_types::Span::from(node),
            text: node.utf8_text(src)?,
        })
    }
}
impl<'tree> ::treesitter_types::LeafNode<'tree> for ListMarkerParenthesis<'tree> {
    fn text(&self) -> &'tree str {
        self.text
    }
}
impl ::treesitter_types::Spanned for ListMarkerParenthesis<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        self.span
    }
}
#[derive(Debug, Clone)]
pub struct ListMarkerPlus<'tree> {
    pub span: ::treesitter_types::Span,
    text: &'tree str,
}
impl<'tree> ::treesitter_types::FromNode<'tree> for ListMarkerPlus<'tree> {
    fn from_node(
        node: ::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        debug_assert_eq!(node.kind(), "list_marker_plus");
        Ok(Self {
            span: ::treesitter_types::Span::from(node),
            text: node.utf8_text(src)?,
        })
    }
}
impl<'tree> ::treesitter_types::LeafNode<'tree> for ListMarkerPlus<'tree> {
    fn text(&self) -> &'tree str {
        self.text
    }
}
impl ::treesitter_types::Spanned for ListMarkerPlus<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        self.span
    }
}
#[derive(Debug, Clone)]
pub struct ListMarkerStar<'tree> {
    pub span: ::treesitter_types::Span,
    text: &'tree str,
}
impl<'tree> ::treesitter_types::FromNode<'tree> for ListMarkerStar<'tree> {
    fn from_node(
        node: ::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        debug_assert_eq!(node.kind(), "list_marker_star");
        Ok(Self {
            span: ::treesitter_types::Span::from(node),
            text: node.utf8_text(src)?,
        })
    }
}
impl<'tree> ::treesitter_types::LeafNode<'tree> for ListMarkerStar<'tree> {
    fn text(&self) -> &'tree str {
        self.text
    }
}
impl ::treesitter_types::Spanned for ListMarkerStar<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        self.span
    }
}
#[derive(Debug, Clone)]
pub struct Paragraph<'tree> {
    pub span: ::treesitter_types::Span,
    pub children: ::std::vec::Vec<ParagraphChildren<'tree>>,
}
impl<'tree> ::treesitter_types::FromNode<'tree> for Paragraph<'tree> {
    #[allow(clippy::match_single_binding, clippy::suspicious_else_formatting)]
    fn from_node(
        node: ::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        debug_assert_eq!(node.kind(), "paragraph");
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
                            <ParagraphChildren as ::treesitter_types::FromNode>::from_node(
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
impl ::treesitter_types::Spanned for Paragraph<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        self.span
    }
}
#[derive(Debug, Clone)]
pub struct PipeTable<'tree> {
    pub span: ::treesitter_types::Span,
    pub children: ::std::vec::Vec<PipeTableChildren<'tree>>,
}
impl<'tree> ::treesitter_types::FromNode<'tree> for PipeTable<'tree> {
    #[allow(clippy::match_single_binding, clippy::suspicious_else_formatting)]
    fn from_node(
        node: ::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        debug_assert_eq!(node.kind(), "pipe_table");
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
                            <PipeTableChildren as ::treesitter_types::FromNode>::from_node(
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
impl ::treesitter_types::Spanned for PipeTable<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        self.span
    }
}
#[derive(Debug, Clone)]
pub struct PipeTableCell<'tree> {
    pub span: ::treesitter_types::Span,
    text: &'tree str,
}
impl<'tree> ::treesitter_types::FromNode<'tree> for PipeTableCell<'tree> {
    fn from_node(
        node: ::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        debug_assert_eq!(node.kind(), "pipe_table_cell");
        Ok(Self {
            span: ::treesitter_types::Span::from(node),
            text: node.utf8_text(src)?,
        })
    }
}
impl<'tree> ::treesitter_types::LeafNode<'tree> for PipeTableCell<'tree> {
    fn text(&self) -> &'tree str {
        self.text
    }
}
impl ::treesitter_types::Spanned for PipeTableCell<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        self.span
    }
}
#[derive(Debug, Clone)]
pub struct PipeTableDelimiterCell<'tree> {
    pub span: ::treesitter_types::Span,
    pub children: ::std::vec::Vec<PipeTableDelimiterCellChildren<'tree>>,
}
impl<'tree> ::treesitter_types::FromNode<'tree> for PipeTableDelimiterCell<'tree> {
    #[allow(clippy::match_single_binding, clippy::suspicious_else_formatting)]
    fn from_node(
        node: ::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        debug_assert_eq!(node.kind(), "pipe_table_delimiter_cell");
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
                            <PipeTableDelimiterCellChildren as ::treesitter_types::FromNode>::from_node(
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
impl ::treesitter_types::Spanned for PipeTableDelimiterCell<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        self.span
    }
}
#[derive(Debug, Clone)]
pub struct PipeTableDelimiterRow<'tree> {
    pub span: ::treesitter_types::Span,
    pub children: ::std::vec::Vec<PipeTableDelimiterCell<'tree>>,
}
impl<'tree> ::treesitter_types::FromNode<'tree> for PipeTableDelimiterRow<'tree> {
    #[allow(clippy::match_single_binding, clippy::suspicious_else_formatting)]
    fn from_node(
        node: ::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        debug_assert_eq!(node.kind(), "pipe_table_delimiter_row");
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
                            <PipeTableDelimiterCell as ::treesitter_types::FromNode>::from_node(
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
impl ::treesitter_types::Spanned for PipeTableDelimiterRow<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        self.span
    }
}
#[derive(Debug, Clone)]
pub struct PipeTableHeader<'tree> {
    pub span: ::treesitter_types::Span,
    pub children: ::std::vec::Vec<PipeTableCell<'tree>>,
}
impl<'tree> ::treesitter_types::FromNode<'tree> for PipeTableHeader<'tree> {
    #[allow(clippy::match_single_binding, clippy::suspicious_else_formatting)]
    fn from_node(
        node: ::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        debug_assert_eq!(node.kind(), "pipe_table_header");
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
                            <PipeTableCell as ::treesitter_types::FromNode>::from_node(
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
impl ::treesitter_types::Spanned for PipeTableHeader<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        self.span
    }
}
#[derive(Debug, Clone)]
pub struct PipeTableRow<'tree> {
    pub span: ::treesitter_types::Span,
    pub children: ::std::vec::Vec<PipeTableCell<'tree>>,
}
impl<'tree> ::treesitter_types::FromNode<'tree> for PipeTableRow<'tree> {
    #[allow(clippy::match_single_binding, clippy::suspicious_else_formatting)]
    fn from_node(
        node: ::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        debug_assert_eq!(node.kind(), "pipe_table_row");
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
                            <PipeTableCell as ::treesitter_types::FromNode>::from_node(
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
impl ::treesitter_types::Spanned for PipeTableRow<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        self.span
    }
}
#[derive(Debug, Clone)]
pub struct Section<'tree> {
    pub span: ::treesitter_types::Span,
    pub children: ::std::vec::Vec<SectionChildren<'tree>>,
}
impl<'tree> ::treesitter_types::FromNode<'tree> for Section<'tree> {
    #[allow(clippy::match_single_binding, clippy::suspicious_else_formatting)]
    fn from_node(
        node: ::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        debug_assert_eq!(node.kind(), "section");
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
                            <SectionChildren as ::treesitter_types::FromNode>::from_node(
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
impl ::treesitter_types::Spanned for Section<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        self.span
    }
}
#[derive(Debug, Clone)]
pub struct SetextHeading<'tree> {
    pub span: ::treesitter_types::Span,
    pub heading_content: Paragraph<'tree>,
    pub children: ::std::vec::Vec<SetextHeadingChildren<'tree>>,
}
impl<'tree> ::treesitter_types::FromNode<'tree> for SetextHeading<'tree> {
    #[allow(clippy::match_single_binding, clippy::suspicious_else_formatting)]
    fn from_node(
        node: ::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        debug_assert_eq!(node.kind(), "setext_heading");
        Ok(Self {
            span: ::treesitter_types::Span::from(node),
            heading_content: {
                let child = node
                    .child_by_field_name("heading_content")
                    .ok_or_else(|| ::treesitter_types::ParseError::missing_field(
                        "heading_content",
                        node,
                    ))?;
                <Paragraph as ::treesitter_types::FromNode>::from_node(child, src)?
            },
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
                            <SetextHeadingChildren as ::treesitter_types::FromNode>::from_node(
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
impl ::treesitter_types::Spanned for SetextHeading<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        self.span
    }
}
#[derive(Debug, Clone)]
pub struct TaskListMarkerChecked<'tree> {
    pub span: ::treesitter_types::Span,
    text: &'tree str,
}
impl<'tree> ::treesitter_types::FromNode<'tree> for TaskListMarkerChecked<'tree> {
    fn from_node(
        node: ::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        debug_assert_eq!(node.kind(), "task_list_marker_checked");
        Ok(Self {
            span: ::treesitter_types::Span::from(node),
            text: node.utf8_text(src)?,
        })
    }
}
impl<'tree> ::treesitter_types::LeafNode<'tree> for TaskListMarkerChecked<'tree> {
    fn text(&self) -> &'tree str {
        self.text
    }
}
impl ::treesitter_types::Spanned for TaskListMarkerChecked<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        self.span
    }
}
#[derive(Debug, Clone)]
pub struct TaskListMarkerUnchecked<'tree> {
    pub span: ::treesitter_types::Span,
    text: &'tree str,
}
impl<'tree> ::treesitter_types::FromNode<'tree> for TaskListMarkerUnchecked<'tree> {
    fn from_node(
        node: ::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        debug_assert_eq!(node.kind(), "task_list_marker_unchecked");
        Ok(Self {
            span: ::treesitter_types::Span::from(node),
            text: node.utf8_text(src)?,
        })
    }
}
impl<'tree> ::treesitter_types::LeafNode<'tree> for TaskListMarkerUnchecked<'tree> {
    fn text(&self) -> &'tree str {
        self.text
    }
}
impl ::treesitter_types::Spanned for TaskListMarkerUnchecked<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        self.span
    }
}
#[derive(Debug, Clone)]
pub struct ThematicBreak<'tree> {
    pub span: ::treesitter_types::Span,
    pub children: ::core::option::Option<BlockContinuation<'tree>>,
}
impl<'tree> ::treesitter_types::FromNode<'tree> for ThematicBreak<'tree> {
    #[allow(clippy::match_single_binding, clippy::suspicious_else_formatting)]
    fn from_node(
        node: ::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        debug_assert_eq!(node.kind(), "thematic_break");
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
                            <BlockContinuation as ::treesitter_types::FromNode>::from_node(
                                child,
                                src,
                            )?,
                        )
                    }
                    None => {
                        let mut _result = None;
                        let mut _fc = node.walk();
                        if _fc.goto_first_child() {
                            loop {
                                if !_fc.node().is_extra() {
                                    let child = _fc.node();
                                    if let Ok(v) = (|| -> ::core::result::Result<
                                        _,
                                        ::treesitter_types::ParseError,
                                    > {
                                        Ok(
                                            Some(
                                                <BlockContinuation as ::treesitter_types::FromNode>::from_node(
                                                    child,
                                                    src,
                                                )?,
                                            ),
                                        )
                                    })() {
                                        _result = Some(v);
                                        break;
                                    }
                                }
                                if !_fc.goto_next_sibling() {
                                    break;
                                }
                            }
                        }
                        _result.flatten()
                    }
                }
            },
        })
    }
}
impl ::treesitter_types::Spanned for ThematicBreak<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        self.span
    }
}
#[derive(Debug, Clone)]
pub struct AtxH1Marker<'tree> {
    pub span: ::treesitter_types::Span,
    text: &'tree str,
}
impl<'tree> ::treesitter_types::FromNode<'tree> for AtxH1Marker<'tree> {
    fn from_node(
        node: ::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        debug_assert_eq!(node.kind(), "atx_h1_marker");
        Ok(Self {
            span: ::treesitter_types::Span::from(node),
            text: node.utf8_text(src)?,
        })
    }
}
impl<'tree> ::treesitter_types::LeafNode<'tree> for AtxH1Marker<'tree> {
    fn text(&self) -> &'tree str {
        self.text
    }
}
impl ::treesitter_types::Spanned for AtxH1Marker<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        self.span
    }
}
#[derive(Debug, Clone)]
pub struct AtxH2Marker<'tree> {
    pub span: ::treesitter_types::Span,
    text: &'tree str,
}
impl<'tree> ::treesitter_types::FromNode<'tree> for AtxH2Marker<'tree> {
    fn from_node(
        node: ::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        debug_assert_eq!(node.kind(), "atx_h2_marker");
        Ok(Self {
            span: ::treesitter_types::Span::from(node),
            text: node.utf8_text(src)?,
        })
    }
}
impl<'tree> ::treesitter_types::LeafNode<'tree> for AtxH2Marker<'tree> {
    fn text(&self) -> &'tree str {
        self.text
    }
}
impl ::treesitter_types::Spanned for AtxH2Marker<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        self.span
    }
}
#[derive(Debug, Clone)]
pub struct AtxH3Marker<'tree> {
    pub span: ::treesitter_types::Span,
    text: &'tree str,
}
impl<'tree> ::treesitter_types::FromNode<'tree> for AtxH3Marker<'tree> {
    fn from_node(
        node: ::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        debug_assert_eq!(node.kind(), "atx_h3_marker");
        Ok(Self {
            span: ::treesitter_types::Span::from(node),
            text: node.utf8_text(src)?,
        })
    }
}
impl<'tree> ::treesitter_types::LeafNode<'tree> for AtxH3Marker<'tree> {
    fn text(&self) -> &'tree str {
        self.text
    }
}
impl ::treesitter_types::Spanned for AtxH3Marker<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        self.span
    }
}
#[derive(Debug, Clone)]
pub struct AtxH4Marker<'tree> {
    pub span: ::treesitter_types::Span,
    text: &'tree str,
}
impl<'tree> ::treesitter_types::FromNode<'tree> for AtxH4Marker<'tree> {
    fn from_node(
        node: ::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        debug_assert_eq!(node.kind(), "atx_h4_marker");
        Ok(Self {
            span: ::treesitter_types::Span::from(node),
            text: node.utf8_text(src)?,
        })
    }
}
impl<'tree> ::treesitter_types::LeafNode<'tree> for AtxH4Marker<'tree> {
    fn text(&self) -> &'tree str {
        self.text
    }
}
impl ::treesitter_types::Spanned for AtxH4Marker<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        self.span
    }
}
#[derive(Debug, Clone)]
pub struct AtxH5Marker<'tree> {
    pub span: ::treesitter_types::Span,
    text: &'tree str,
}
impl<'tree> ::treesitter_types::FromNode<'tree> for AtxH5Marker<'tree> {
    fn from_node(
        node: ::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        debug_assert_eq!(node.kind(), "atx_h5_marker");
        Ok(Self {
            span: ::treesitter_types::Span::from(node),
            text: node.utf8_text(src)?,
        })
    }
}
impl<'tree> ::treesitter_types::LeafNode<'tree> for AtxH5Marker<'tree> {
    fn text(&self) -> &'tree str {
        self.text
    }
}
impl ::treesitter_types::Spanned for AtxH5Marker<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        self.span
    }
}
#[derive(Debug, Clone)]
pub struct AtxH6Marker<'tree> {
    pub span: ::treesitter_types::Span,
    text: &'tree str,
}
impl<'tree> ::treesitter_types::FromNode<'tree> for AtxH6Marker<'tree> {
    fn from_node(
        node: ::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        debug_assert_eq!(node.kind(), "atx_h6_marker");
        Ok(Self {
            span: ::treesitter_types::Span::from(node),
            text: node.utf8_text(src)?,
        })
    }
}
impl<'tree> ::treesitter_types::LeafNode<'tree> for AtxH6Marker<'tree> {
    fn text(&self) -> &'tree str {
        self.text
    }
}
impl ::treesitter_types::Spanned for AtxH6Marker<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        self.span
    }
}
#[derive(Debug, Clone)]
pub struct BlockContinuation<'tree> {
    pub span: ::treesitter_types::Span,
    text: &'tree str,
}
impl<'tree> ::treesitter_types::FromNode<'tree> for BlockContinuation<'tree> {
    fn from_node(
        node: ::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        debug_assert_eq!(node.kind(), "block_continuation");
        Ok(Self {
            span: ::treesitter_types::Span::from(node),
            text: node.utf8_text(src)?,
        })
    }
}
impl<'tree> ::treesitter_types::LeafNode<'tree> for BlockContinuation<'tree> {
    fn text(&self) -> &'tree str {
        self.text
    }
}
impl ::treesitter_types::Spanned for BlockContinuation<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        self.span
    }
}
#[derive(Debug, Clone)]
pub struct BlockQuoteMarker<'tree> {
    pub span: ::treesitter_types::Span,
    text: &'tree str,
}
impl<'tree> ::treesitter_types::FromNode<'tree> for BlockQuoteMarker<'tree> {
    fn from_node(
        node: ::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        debug_assert_eq!(node.kind(), "block_quote_marker");
        Ok(Self {
            span: ::treesitter_types::Span::from(node),
            text: node.utf8_text(src)?,
        })
    }
}
impl<'tree> ::treesitter_types::LeafNode<'tree> for BlockQuoteMarker<'tree> {
    fn text(&self) -> &'tree str {
        self.text
    }
}
impl ::treesitter_types::Spanned for BlockQuoteMarker<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        self.span
    }
}
#[derive(Debug, Clone)]
pub struct EntityReference<'tree> {
    pub span: ::treesitter_types::Span,
    text: &'tree str,
}
impl<'tree> ::treesitter_types::FromNode<'tree> for EntityReference<'tree> {
    fn from_node(
        node: ::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        debug_assert_eq!(node.kind(), "entity_reference");
        Ok(Self {
            span: ::treesitter_types::Span::from(node),
            text: node.utf8_text(src)?,
        })
    }
}
impl<'tree> ::treesitter_types::LeafNode<'tree> for EntityReference<'tree> {
    fn text(&self) -> &'tree str {
        self.text
    }
}
impl ::treesitter_types::Spanned for EntityReference<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        self.span
    }
}
#[derive(Debug, Clone)]
pub struct FencedCodeBlockDelimiter<'tree> {
    pub span: ::treesitter_types::Span,
    text: &'tree str,
}
impl<'tree> ::treesitter_types::FromNode<'tree> for FencedCodeBlockDelimiter<'tree> {
    fn from_node(
        node: ::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        debug_assert_eq!(node.kind(), "fenced_code_block_delimiter");
        Ok(Self {
            span: ::treesitter_types::Span::from(node),
            text: node.utf8_text(src)?,
        })
    }
}
impl<'tree> ::treesitter_types::LeafNode<'tree> for FencedCodeBlockDelimiter<'tree> {
    fn text(&self) -> &'tree str {
        self.text
    }
}
impl ::treesitter_types::Spanned for FencedCodeBlockDelimiter<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        self.span
    }
}
#[derive(Debug, Clone)]
pub struct MinusMetadata<'tree> {
    pub span: ::treesitter_types::Span,
    text: &'tree str,
}
impl<'tree> ::treesitter_types::FromNode<'tree> for MinusMetadata<'tree> {
    fn from_node(
        node: ::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        debug_assert_eq!(node.kind(), "minus_metadata");
        Ok(Self {
            span: ::treesitter_types::Span::from(node),
            text: node.utf8_text(src)?,
        })
    }
}
impl<'tree> ::treesitter_types::LeafNode<'tree> for MinusMetadata<'tree> {
    fn text(&self) -> &'tree str {
        self.text
    }
}
impl ::treesitter_types::Spanned for MinusMetadata<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        self.span
    }
}
#[derive(Debug, Clone)]
pub struct NumericCharacterReference<'tree> {
    pub span: ::treesitter_types::Span,
    text: &'tree str,
}
impl<'tree> ::treesitter_types::FromNode<'tree> for NumericCharacterReference<'tree> {
    fn from_node(
        node: ::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        debug_assert_eq!(node.kind(), "numeric_character_reference");
        Ok(Self {
            span: ::treesitter_types::Span::from(node),
            text: node.utf8_text(src)?,
        })
    }
}
impl<'tree> ::treesitter_types::LeafNode<'tree> for NumericCharacterReference<'tree> {
    fn text(&self) -> &'tree str {
        self.text
    }
}
impl ::treesitter_types::Spanned for NumericCharacterReference<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        self.span
    }
}
#[derive(Debug, Clone)]
pub struct PipeTableAlignLeft<'tree> {
    pub span: ::treesitter_types::Span,
    text: &'tree str,
}
impl<'tree> ::treesitter_types::FromNode<'tree> for PipeTableAlignLeft<'tree> {
    fn from_node(
        node: ::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        debug_assert_eq!(node.kind(), "pipe_table_align_left");
        Ok(Self {
            span: ::treesitter_types::Span::from(node),
            text: node.utf8_text(src)?,
        })
    }
}
impl<'tree> ::treesitter_types::LeafNode<'tree> for PipeTableAlignLeft<'tree> {
    fn text(&self) -> &'tree str {
        self.text
    }
}
impl ::treesitter_types::Spanned for PipeTableAlignLeft<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        self.span
    }
}
#[derive(Debug, Clone)]
pub struct PipeTableAlignRight<'tree> {
    pub span: ::treesitter_types::Span,
    text: &'tree str,
}
impl<'tree> ::treesitter_types::FromNode<'tree> for PipeTableAlignRight<'tree> {
    fn from_node(
        node: ::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        debug_assert_eq!(node.kind(), "pipe_table_align_right");
        Ok(Self {
            span: ::treesitter_types::Span::from(node),
            text: node.utf8_text(src)?,
        })
    }
}
impl<'tree> ::treesitter_types::LeafNode<'tree> for PipeTableAlignRight<'tree> {
    fn text(&self) -> &'tree str {
        self.text
    }
}
impl ::treesitter_types::Spanned for PipeTableAlignRight<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        self.span
    }
}
#[derive(Debug, Clone)]
pub struct PlusMetadata<'tree> {
    pub span: ::treesitter_types::Span,
    text: &'tree str,
}
impl<'tree> ::treesitter_types::FromNode<'tree> for PlusMetadata<'tree> {
    fn from_node(
        node: ::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        debug_assert_eq!(node.kind(), "plus_metadata");
        Ok(Self {
            span: ::treesitter_types::Span::from(node),
            text: node.utf8_text(src)?,
        })
    }
}
impl<'tree> ::treesitter_types::LeafNode<'tree> for PlusMetadata<'tree> {
    fn text(&self) -> &'tree str {
        self.text
    }
}
impl ::treesitter_types::Spanned for PlusMetadata<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        self.span
    }
}
#[derive(Debug, Clone)]
pub struct SetextH1Underline<'tree> {
    pub span: ::treesitter_types::Span,
    text: &'tree str,
}
impl<'tree> ::treesitter_types::FromNode<'tree> for SetextH1Underline<'tree> {
    fn from_node(
        node: ::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        debug_assert_eq!(node.kind(), "setext_h1_underline");
        Ok(Self {
            span: ::treesitter_types::Span::from(node),
            text: node.utf8_text(src)?,
        })
    }
}
impl<'tree> ::treesitter_types::LeafNode<'tree> for SetextH1Underline<'tree> {
    fn text(&self) -> &'tree str {
        self.text
    }
}
impl ::treesitter_types::Spanned for SetextH1Underline<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        self.span
    }
}
#[derive(Debug, Clone)]
pub struct SetextH2Underline<'tree> {
    pub span: ::treesitter_types::Span,
    text: &'tree str,
}
impl<'tree> ::treesitter_types::FromNode<'tree> for SetextH2Underline<'tree> {
    fn from_node(
        node: ::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        debug_assert_eq!(node.kind(), "setext_h2_underline");
        Ok(Self {
            span: ::treesitter_types::Span::from(node),
            text: node.utf8_text(src)?,
        })
    }
}
impl<'tree> ::treesitter_types::LeafNode<'tree> for SetextH2Underline<'tree> {
    fn text(&self) -> &'tree str {
        self.text
    }
}
impl ::treesitter_types::Spanned for SetextH2Underline<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        self.span
    }
}
#[derive(Debug, Clone)]
pub enum AtxHeadingChildren<'tree> {
    AtxH1Marker(::std::boxed::Box<AtxH1Marker<'tree>>),
    AtxH2Marker(::std::boxed::Box<AtxH2Marker<'tree>>),
    AtxH3Marker(::std::boxed::Box<AtxH3Marker<'tree>>),
    AtxH4Marker(::std::boxed::Box<AtxH4Marker<'tree>>),
    AtxH5Marker(::std::boxed::Box<AtxH5Marker<'tree>>),
    AtxH6Marker(::std::boxed::Box<AtxH6Marker<'tree>>),
    BlockContinuation(::std::boxed::Box<BlockContinuation<'tree>>),
}
impl<'tree> ::treesitter_types::FromNode<'tree> for AtxHeadingChildren<'tree> {
    #[allow(clippy::collapsible_else_if)]
    fn from_node(
        node: ::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        match node.kind() {
            "atx_h1_marker" => {
                Ok(
                    Self::AtxH1Marker(
                        ::std::boxed::Box::new(
                            <AtxH1Marker as ::treesitter_types::FromNode>::from_node(
                                node,
                                src,
                            )?,
                        ),
                    ),
                )
            }
            "atx_h2_marker" => {
                Ok(
                    Self::AtxH2Marker(
                        ::std::boxed::Box::new(
                            <AtxH2Marker as ::treesitter_types::FromNode>::from_node(
                                node,
                                src,
                            )?,
                        ),
                    ),
                )
            }
            "atx_h3_marker" => {
                Ok(
                    Self::AtxH3Marker(
                        ::std::boxed::Box::new(
                            <AtxH3Marker as ::treesitter_types::FromNode>::from_node(
                                node,
                                src,
                            )?,
                        ),
                    ),
                )
            }
            "atx_h4_marker" => {
                Ok(
                    Self::AtxH4Marker(
                        ::std::boxed::Box::new(
                            <AtxH4Marker as ::treesitter_types::FromNode>::from_node(
                                node,
                                src,
                            )?,
                        ),
                    ),
                )
            }
            "atx_h5_marker" => {
                Ok(
                    Self::AtxH5Marker(
                        ::std::boxed::Box::new(
                            <AtxH5Marker as ::treesitter_types::FromNode>::from_node(
                                node,
                                src,
                            )?,
                        ),
                    ),
                )
            }
            "atx_h6_marker" => {
                Ok(
                    Self::AtxH6Marker(
                        ::std::boxed::Box::new(
                            <AtxH6Marker as ::treesitter_types::FromNode>::from_node(
                                node,
                                src,
                            )?,
                        ),
                    ),
                )
            }
            "block_continuation" => {
                Ok(
                    Self::BlockContinuation(
                        ::std::boxed::Box::new(
                            <BlockContinuation as ::treesitter_types::FromNode>::from_node(
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
impl ::treesitter_types::Spanned for AtxHeadingChildren<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        match self {
            Self::AtxH1Marker(inner) => inner.span(),
            Self::AtxH2Marker(inner) => inner.span(),
            Self::AtxH3Marker(inner) => inner.span(),
            Self::AtxH4Marker(inner) => inner.span(),
            Self::AtxH5Marker(inner) => inner.span(),
            Self::AtxH6Marker(inner) => inner.span(),
            Self::BlockContinuation(inner) => inner.span(),
        }
    }
}
#[derive(Debug, Clone)]
pub enum BlockQuoteChildren<'tree> {
    BlockContinuation(::std::boxed::Box<BlockContinuation<'tree>>),
    BlockQuote(::std::boxed::Box<BlockQuote<'tree>>),
    BlockQuoteMarker(::std::boxed::Box<BlockQuoteMarker<'tree>>),
    FencedCodeBlock(::std::boxed::Box<FencedCodeBlock<'tree>>),
    HtmlBlock(::std::boxed::Box<HtmlBlock<'tree>>),
    IndentedCodeBlock(::std::boxed::Box<IndentedCodeBlock<'tree>>),
    LinkReferenceDefinition(::std::boxed::Box<LinkReferenceDefinition<'tree>>),
    List(::std::boxed::Box<List<'tree>>),
    Paragraph(::std::boxed::Box<Paragraph<'tree>>),
    PipeTable(::std::boxed::Box<PipeTable<'tree>>),
    Section(::std::boxed::Box<Section<'tree>>),
    SetextHeading(::std::boxed::Box<SetextHeading<'tree>>),
    ThematicBreak(::std::boxed::Box<ThematicBreak<'tree>>),
}
impl<'tree> ::treesitter_types::FromNode<'tree> for BlockQuoteChildren<'tree> {
    #[allow(clippy::collapsible_else_if)]
    fn from_node(
        node: ::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        match node.kind() {
            "block_continuation" => {
                Ok(
                    Self::BlockContinuation(
                        ::std::boxed::Box::new(
                            <BlockContinuation as ::treesitter_types::FromNode>::from_node(
                                node,
                                src,
                            )?,
                        ),
                    ),
                )
            }
            "block_quote" => {
                Ok(
                    Self::BlockQuote(
                        ::std::boxed::Box::new(
                            <BlockQuote as ::treesitter_types::FromNode>::from_node(
                                node,
                                src,
                            )?,
                        ),
                    ),
                )
            }
            "block_quote_marker" => {
                Ok(
                    Self::BlockQuoteMarker(
                        ::std::boxed::Box::new(
                            <BlockQuoteMarker as ::treesitter_types::FromNode>::from_node(
                                node,
                                src,
                            )?,
                        ),
                    ),
                )
            }
            "fenced_code_block" => {
                Ok(
                    Self::FencedCodeBlock(
                        ::std::boxed::Box::new(
                            <FencedCodeBlock as ::treesitter_types::FromNode>::from_node(
                                node,
                                src,
                            )?,
                        ),
                    ),
                )
            }
            "html_block" => {
                Ok(
                    Self::HtmlBlock(
                        ::std::boxed::Box::new(
                            <HtmlBlock as ::treesitter_types::FromNode>::from_node(
                                node,
                                src,
                            )?,
                        ),
                    ),
                )
            }
            "indented_code_block" => {
                Ok(
                    Self::IndentedCodeBlock(
                        ::std::boxed::Box::new(
                            <IndentedCodeBlock as ::treesitter_types::FromNode>::from_node(
                                node,
                                src,
                            )?,
                        ),
                    ),
                )
            }
            "link_reference_definition" => {
                Ok(
                    Self::LinkReferenceDefinition(
                        ::std::boxed::Box::new(
                            <LinkReferenceDefinition as ::treesitter_types::FromNode>::from_node(
                                node,
                                src,
                            )?,
                        ),
                    ),
                )
            }
            "list" => {
                Ok(
                    Self::List(
                        ::std::boxed::Box::new(
                            <List as ::treesitter_types::FromNode>::from_node(node, src)?,
                        ),
                    ),
                )
            }
            "paragraph" => {
                Ok(
                    Self::Paragraph(
                        ::std::boxed::Box::new(
                            <Paragraph as ::treesitter_types::FromNode>::from_node(
                                node,
                                src,
                            )?,
                        ),
                    ),
                )
            }
            "pipe_table" => {
                Ok(
                    Self::PipeTable(
                        ::std::boxed::Box::new(
                            <PipeTable as ::treesitter_types::FromNode>::from_node(
                                node,
                                src,
                            )?,
                        ),
                    ),
                )
            }
            "section" => {
                Ok(
                    Self::Section(
                        ::std::boxed::Box::new(
                            <Section as ::treesitter_types::FromNode>::from_node(
                                node,
                                src,
                            )?,
                        ),
                    ),
                )
            }
            "setext_heading" => {
                Ok(
                    Self::SetextHeading(
                        ::std::boxed::Box::new(
                            <SetextHeading as ::treesitter_types::FromNode>::from_node(
                                node,
                                src,
                            )?,
                        ),
                    ),
                )
            }
            "thematic_break" => {
                Ok(
                    Self::ThematicBreak(
                        ::std::boxed::Box::new(
                            <ThematicBreak as ::treesitter_types::FromNode>::from_node(
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
impl ::treesitter_types::Spanned for BlockQuoteChildren<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        match self {
            Self::BlockContinuation(inner) => inner.span(),
            Self::BlockQuote(inner) => inner.span(),
            Self::BlockQuoteMarker(inner) => inner.span(),
            Self::FencedCodeBlock(inner) => inner.span(),
            Self::HtmlBlock(inner) => inner.span(),
            Self::IndentedCodeBlock(inner) => inner.span(),
            Self::LinkReferenceDefinition(inner) => inner.span(),
            Self::List(inner) => inner.span(),
            Self::Paragraph(inner) => inner.span(),
            Self::PipeTable(inner) => inner.span(),
            Self::Section(inner) => inner.span(),
            Self::SetextHeading(inner) => inner.span(),
            Self::ThematicBreak(inner) => inner.span(),
        }
    }
}
#[derive(Debug, Clone)]
pub enum DocumentChildren<'tree> {
    MinusMetadata(::std::boxed::Box<MinusMetadata<'tree>>),
    PlusMetadata(::std::boxed::Box<PlusMetadata<'tree>>),
    Section(::std::boxed::Box<Section<'tree>>),
}
impl<'tree> ::treesitter_types::FromNode<'tree> for DocumentChildren<'tree> {
    #[allow(clippy::collapsible_else_if)]
    fn from_node(
        node: ::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        match node.kind() {
            "minus_metadata" => {
                Ok(
                    Self::MinusMetadata(
                        ::std::boxed::Box::new(
                            <MinusMetadata as ::treesitter_types::FromNode>::from_node(
                                node,
                                src,
                            )?,
                        ),
                    ),
                )
            }
            "plus_metadata" => {
                Ok(
                    Self::PlusMetadata(
                        ::std::boxed::Box::new(
                            <PlusMetadata as ::treesitter_types::FromNode>::from_node(
                                node,
                                src,
                            )?,
                        ),
                    ),
                )
            }
            "section" => {
                Ok(
                    Self::Section(
                        ::std::boxed::Box::new(
                            <Section as ::treesitter_types::FromNode>::from_node(
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
            Self::MinusMetadata(inner) => inner.span(),
            Self::PlusMetadata(inner) => inner.span(),
            Self::Section(inner) => inner.span(),
        }
    }
}
#[derive(Debug, Clone)]
pub enum FencedCodeBlockChildren<'tree> {
    BlockContinuation(::std::boxed::Box<BlockContinuation<'tree>>),
    CodeFenceContent(::std::boxed::Box<CodeFenceContent<'tree>>),
    FencedCodeBlockDelimiter(::std::boxed::Box<FencedCodeBlockDelimiter<'tree>>),
    InfoString(::std::boxed::Box<InfoString<'tree>>),
}
impl<'tree> ::treesitter_types::FromNode<'tree> for FencedCodeBlockChildren<'tree> {
    #[allow(clippy::collapsible_else_if)]
    fn from_node(
        node: ::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        match node.kind() {
            "block_continuation" => {
                Ok(
                    Self::BlockContinuation(
                        ::std::boxed::Box::new(
                            <BlockContinuation as ::treesitter_types::FromNode>::from_node(
                                node,
                                src,
                            )?,
                        ),
                    ),
                )
            }
            "code_fence_content" => {
                Ok(
                    Self::CodeFenceContent(
                        ::std::boxed::Box::new(
                            <CodeFenceContent as ::treesitter_types::FromNode>::from_node(
                                node,
                                src,
                            )?,
                        ),
                    ),
                )
            }
            "fenced_code_block_delimiter" => {
                Ok(
                    Self::FencedCodeBlockDelimiter(
                        ::std::boxed::Box::new(
                            <FencedCodeBlockDelimiter as ::treesitter_types::FromNode>::from_node(
                                node,
                                src,
                            )?,
                        ),
                    ),
                )
            }
            "info_string" => {
                Ok(
                    Self::InfoString(
                        ::std::boxed::Box::new(
                            <InfoString as ::treesitter_types::FromNode>::from_node(
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
impl ::treesitter_types::Spanned for FencedCodeBlockChildren<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        match self {
            Self::BlockContinuation(inner) => inner.span(),
            Self::CodeFenceContent(inner) => inner.span(),
            Self::FencedCodeBlockDelimiter(inner) => inner.span(),
            Self::InfoString(inner) => inner.span(),
        }
    }
}
#[derive(Debug, Clone)]
pub enum InfoStringChildren<'tree> {
    BackslashEscape(::std::boxed::Box<BackslashEscape<'tree>>),
    EntityReference(::std::boxed::Box<EntityReference<'tree>>),
    Language(::std::boxed::Box<Language<'tree>>),
    NumericCharacterReference(::std::boxed::Box<NumericCharacterReference<'tree>>),
}
impl<'tree> ::treesitter_types::FromNode<'tree> for InfoStringChildren<'tree> {
    #[allow(clippy::collapsible_else_if)]
    fn from_node(
        node: ::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        match node.kind() {
            "backslash_escape" => {
                Ok(
                    Self::BackslashEscape(
                        ::std::boxed::Box::new(
                            <BackslashEscape as ::treesitter_types::FromNode>::from_node(
                                node,
                                src,
                            )?,
                        ),
                    ),
                )
            }
            "entity_reference" => {
                Ok(
                    Self::EntityReference(
                        ::std::boxed::Box::new(
                            <EntityReference as ::treesitter_types::FromNode>::from_node(
                                node,
                                src,
                            )?,
                        ),
                    ),
                )
            }
            "language" => {
                Ok(
                    Self::Language(
                        ::std::boxed::Box::new(
                            <Language as ::treesitter_types::FromNode>::from_node(
                                node,
                                src,
                            )?,
                        ),
                    ),
                )
            }
            "numeric_character_reference" => {
                Ok(
                    Self::NumericCharacterReference(
                        ::std::boxed::Box::new(
                            <NumericCharacterReference as ::treesitter_types::FromNode>::from_node(
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
impl ::treesitter_types::Spanned for InfoStringChildren<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        match self {
            Self::BackslashEscape(inner) => inner.span(),
            Self::EntityReference(inner) => inner.span(),
            Self::Language(inner) => inner.span(),
            Self::NumericCharacterReference(inner) => inner.span(),
        }
    }
}
#[derive(Debug, Clone)]
pub enum LanguageChildren<'tree> {
    BackslashEscape(::std::boxed::Box<BackslashEscape<'tree>>),
    EntityReference(::std::boxed::Box<EntityReference<'tree>>),
    NumericCharacterReference(::std::boxed::Box<NumericCharacterReference<'tree>>),
}
impl<'tree> ::treesitter_types::FromNode<'tree> for LanguageChildren<'tree> {
    #[allow(clippy::collapsible_else_if)]
    fn from_node(
        node: ::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        match node.kind() {
            "backslash_escape" => {
                Ok(
                    Self::BackslashEscape(
                        ::std::boxed::Box::new(
                            <BackslashEscape as ::treesitter_types::FromNode>::from_node(
                                node,
                                src,
                            )?,
                        ),
                    ),
                )
            }
            "entity_reference" => {
                Ok(
                    Self::EntityReference(
                        ::std::boxed::Box::new(
                            <EntityReference as ::treesitter_types::FromNode>::from_node(
                                node,
                                src,
                            )?,
                        ),
                    ),
                )
            }
            "numeric_character_reference" => {
                Ok(
                    Self::NumericCharacterReference(
                        ::std::boxed::Box::new(
                            <NumericCharacterReference as ::treesitter_types::FromNode>::from_node(
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
impl ::treesitter_types::Spanned for LanguageChildren<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        match self {
            Self::BackslashEscape(inner) => inner.span(),
            Self::EntityReference(inner) => inner.span(),
            Self::NumericCharacterReference(inner) => inner.span(),
        }
    }
}
#[derive(Debug, Clone)]
pub enum LinkDestinationChildren<'tree> {
    BackslashEscape(::std::boxed::Box<BackslashEscape<'tree>>),
    EntityReference(::std::boxed::Box<EntityReference<'tree>>),
    NumericCharacterReference(::std::boxed::Box<NumericCharacterReference<'tree>>),
}
impl<'tree> ::treesitter_types::FromNode<'tree> for LinkDestinationChildren<'tree> {
    #[allow(clippy::collapsible_else_if)]
    fn from_node(
        node: ::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        match node.kind() {
            "backslash_escape" => {
                Ok(
                    Self::BackslashEscape(
                        ::std::boxed::Box::new(
                            <BackslashEscape as ::treesitter_types::FromNode>::from_node(
                                node,
                                src,
                            )?,
                        ),
                    ),
                )
            }
            "entity_reference" => {
                Ok(
                    Self::EntityReference(
                        ::std::boxed::Box::new(
                            <EntityReference as ::treesitter_types::FromNode>::from_node(
                                node,
                                src,
                            )?,
                        ),
                    ),
                )
            }
            "numeric_character_reference" => {
                Ok(
                    Self::NumericCharacterReference(
                        ::std::boxed::Box::new(
                            <NumericCharacterReference as ::treesitter_types::FromNode>::from_node(
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
impl ::treesitter_types::Spanned for LinkDestinationChildren<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        match self {
            Self::BackslashEscape(inner) => inner.span(),
            Self::EntityReference(inner) => inner.span(),
            Self::NumericCharacterReference(inner) => inner.span(),
        }
    }
}
#[derive(Debug, Clone)]
pub enum LinkLabelChildren<'tree> {
    BackslashEscape(::std::boxed::Box<BackslashEscape<'tree>>),
    BlockContinuation(::std::boxed::Box<BlockContinuation<'tree>>),
    EntityReference(::std::boxed::Box<EntityReference<'tree>>),
    NumericCharacterReference(::std::boxed::Box<NumericCharacterReference<'tree>>),
}
impl<'tree> ::treesitter_types::FromNode<'tree> for LinkLabelChildren<'tree> {
    #[allow(clippy::collapsible_else_if)]
    fn from_node(
        node: ::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        match node.kind() {
            "backslash_escape" => {
                Ok(
                    Self::BackslashEscape(
                        ::std::boxed::Box::new(
                            <BackslashEscape as ::treesitter_types::FromNode>::from_node(
                                node,
                                src,
                            )?,
                        ),
                    ),
                )
            }
            "block_continuation" => {
                Ok(
                    Self::BlockContinuation(
                        ::std::boxed::Box::new(
                            <BlockContinuation as ::treesitter_types::FromNode>::from_node(
                                node,
                                src,
                            )?,
                        ),
                    ),
                )
            }
            "entity_reference" => {
                Ok(
                    Self::EntityReference(
                        ::std::boxed::Box::new(
                            <EntityReference as ::treesitter_types::FromNode>::from_node(
                                node,
                                src,
                            )?,
                        ),
                    ),
                )
            }
            "numeric_character_reference" => {
                Ok(
                    Self::NumericCharacterReference(
                        ::std::boxed::Box::new(
                            <NumericCharacterReference as ::treesitter_types::FromNode>::from_node(
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
impl ::treesitter_types::Spanned for LinkLabelChildren<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        match self {
            Self::BackslashEscape(inner) => inner.span(),
            Self::BlockContinuation(inner) => inner.span(),
            Self::EntityReference(inner) => inner.span(),
            Self::NumericCharacterReference(inner) => inner.span(),
        }
    }
}
#[derive(Debug, Clone)]
pub enum LinkReferenceDefinitionChildren<'tree> {
    BlockContinuation(::std::boxed::Box<BlockContinuation<'tree>>),
    LinkDestination(::std::boxed::Box<LinkDestination<'tree>>),
    LinkLabel(::std::boxed::Box<LinkLabel<'tree>>),
    LinkTitle(::std::boxed::Box<LinkTitle<'tree>>),
}
impl<'tree> ::treesitter_types::FromNode<'tree>
for LinkReferenceDefinitionChildren<'tree> {
    #[allow(clippy::collapsible_else_if)]
    fn from_node(
        node: ::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        match node.kind() {
            "block_continuation" => {
                Ok(
                    Self::BlockContinuation(
                        ::std::boxed::Box::new(
                            <BlockContinuation as ::treesitter_types::FromNode>::from_node(
                                node,
                                src,
                            )?,
                        ),
                    ),
                )
            }
            "link_destination" => {
                Ok(
                    Self::LinkDestination(
                        ::std::boxed::Box::new(
                            <LinkDestination as ::treesitter_types::FromNode>::from_node(
                                node,
                                src,
                            )?,
                        ),
                    ),
                )
            }
            "link_label" => {
                Ok(
                    Self::LinkLabel(
                        ::std::boxed::Box::new(
                            <LinkLabel as ::treesitter_types::FromNode>::from_node(
                                node,
                                src,
                            )?,
                        ),
                    ),
                )
            }
            "link_title" => {
                Ok(
                    Self::LinkTitle(
                        ::std::boxed::Box::new(
                            <LinkTitle as ::treesitter_types::FromNode>::from_node(
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
impl ::treesitter_types::Spanned for LinkReferenceDefinitionChildren<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        match self {
            Self::BlockContinuation(inner) => inner.span(),
            Self::LinkDestination(inner) => inner.span(),
            Self::LinkLabel(inner) => inner.span(),
            Self::LinkTitle(inner) => inner.span(),
        }
    }
}
#[derive(Debug, Clone)]
pub enum LinkTitleChildren<'tree> {
    BackslashEscape(::std::boxed::Box<BackslashEscape<'tree>>),
    BlockContinuation(::std::boxed::Box<BlockContinuation<'tree>>),
    EntityReference(::std::boxed::Box<EntityReference<'tree>>),
    NumericCharacterReference(::std::boxed::Box<NumericCharacterReference<'tree>>),
}
impl<'tree> ::treesitter_types::FromNode<'tree> for LinkTitleChildren<'tree> {
    #[allow(clippy::collapsible_else_if)]
    fn from_node(
        node: ::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        match node.kind() {
            "backslash_escape" => {
                Ok(
                    Self::BackslashEscape(
                        ::std::boxed::Box::new(
                            <BackslashEscape as ::treesitter_types::FromNode>::from_node(
                                node,
                                src,
                            )?,
                        ),
                    ),
                )
            }
            "block_continuation" => {
                Ok(
                    Self::BlockContinuation(
                        ::std::boxed::Box::new(
                            <BlockContinuation as ::treesitter_types::FromNode>::from_node(
                                node,
                                src,
                            )?,
                        ),
                    ),
                )
            }
            "entity_reference" => {
                Ok(
                    Self::EntityReference(
                        ::std::boxed::Box::new(
                            <EntityReference as ::treesitter_types::FromNode>::from_node(
                                node,
                                src,
                            )?,
                        ),
                    ),
                )
            }
            "numeric_character_reference" => {
                Ok(
                    Self::NumericCharacterReference(
                        ::std::boxed::Box::new(
                            <NumericCharacterReference as ::treesitter_types::FromNode>::from_node(
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
impl ::treesitter_types::Spanned for LinkTitleChildren<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        match self {
            Self::BackslashEscape(inner) => inner.span(),
            Self::BlockContinuation(inner) => inner.span(),
            Self::EntityReference(inner) => inner.span(),
            Self::NumericCharacterReference(inner) => inner.span(),
        }
    }
}
#[derive(Debug, Clone)]
pub enum ListItemChildren<'tree> {
    BlockContinuation(::std::boxed::Box<BlockContinuation<'tree>>),
    BlockQuote(::std::boxed::Box<BlockQuote<'tree>>),
    FencedCodeBlock(::std::boxed::Box<FencedCodeBlock<'tree>>),
    HtmlBlock(::std::boxed::Box<HtmlBlock<'tree>>),
    IndentedCodeBlock(::std::boxed::Box<IndentedCodeBlock<'tree>>),
    LinkReferenceDefinition(::std::boxed::Box<LinkReferenceDefinition<'tree>>),
    List(::std::boxed::Box<List<'tree>>),
    ListMarkerDot(::std::boxed::Box<ListMarkerDot<'tree>>),
    ListMarkerMinus(::std::boxed::Box<ListMarkerMinus<'tree>>),
    ListMarkerParenthesis(::std::boxed::Box<ListMarkerParenthesis<'tree>>),
    ListMarkerPlus(::std::boxed::Box<ListMarkerPlus<'tree>>),
    ListMarkerStar(::std::boxed::Box<ListMarkerStar<'tree>>),
    Paragraph(::std::boxed::Box<Paragraph<'tree>>),
    PipeTable(::std::boxed::Box<PipeTable<'tree>>),
    Section(::std::boxed::Box<Section<'tree>>),
    SetextHeading(::std::boxed::Box<SetextHeading<'tree>>),
    TaskListMarkerChecked(::std::boxed::Box<TaskListMarkerChecked<'tree>>),
    TaskListMarkerUnchecked(::std::boxed::Box<TaskListMarkerUnchecked<'tree>>),
    ThematicBreak(::std::boxed::Box<ThematicBreak<'tree>>),
}
impl<'tree> ::treesitter_types::FromNode<'tree> for ListItemChildren<'tree> {
    #[allow(clippy::collapsible_else_if)]
    fn from_node(
        node: ::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        match node.kind() {
            "block_continuation" => {
                Ok(
                    Self::BlockContinuation(
                        ::std::boxed::Box::new(
                            <BlockContinuation as ::treesitter_types::FromNode>::from_node(
                                node,
                                src,
                            )?,
                        ),
                    ),
                )
            }
            "block_quote" => {
                Ok(
                    Self::BlockQuote(
                        ::std::boxed::Box::new(
                            <BlockQuote as ::treesitter_types::FromNode>::from_node(
                                node,
                                src,
                            )?,
                        ),
                    ),
                )
            }
            "fenced_code_block" => {
                Ok(
                    Self::FencedCodeBlock(
                        ::std::boxed::Box::new(
                            <FencedCodeBlock as ::treesitter_types::FromNode>::from_node(
                                node,
                                src,
                            )?,
                        ),
                    ),
                )
            }
            "html_block" => {
                Ok(
                    Self::HtmlBlock(
                        ::std::boxed::Box::new(
                            <HtmlBlock as ::treesitter_types::FromNode>::from_node(
                                node,
                                src,
                            )?,
                        ),
                    ),
                )
            }
            "indented_code_block" => {
                Ok(
                    Self::IndentedCodeBlock(
                        ::std::boxed::Box::new(
                            <IndentedCodeBlock as ::treesitter_types::FromNode>::from_node(
                                node,
                                src,
                            )?,
                        ),
                    ),
                )
            }
            "link_reference_definition" => {
                Ok(
                    Self::LinkReferenceDefinition(
                        ::std::boxed::Box::new(
                            <LinkReferenceDefinition as ::treesitter_types::FromNode>::from_node(
                                node,
                                src,
                            )?,
                        ),
                    ),
                )
            }
            "list" => {
                Ok(
                    Self::List(
                        ::std::boxed::Box::new(
                            <List as ::treesitter_types::FromNode>::from_node(node, src)?,
                        ),
                    ),
                )
            }
            "list_marker_dot" => {
                Ok(
                    Self::ListMarkerDot(
                        ::std::boxed::Box::new(
                            <ListMarkerDot as ::treesitter_types::FromNode>::from_node(
                                node,
                                src,
                            )?,
                        ),
                    ),
                )
            }
            "list_marker_minus" => {
                Ok(
                    Self::ListMarkerMinus(
                        ::std::boxed::Box::new(
                            <ListMarkerMinus as ::treesitter_types::FromNode>::from_node(
                                node,
                                src,
                            )?,
                        ),
                    ),
                )
            }
            "list_marker_parenthesis" => {
                Ok(
                    Self::ListMarkerParenthesis(
                        ::std::boxed::Box::new(
                            <ListMarkerParenthesis as ::treesitter_types::FromNode>::from_node(
                                node,
                                src,
                            )?,
                        ),
                    ),
                )
            }
            "list_marker_plus" => {
                Ok(
                    Self::ListMarkerPlus(
                        ::std::boxed::Box::new(
                            <ListMarkerPlus as ::treesitter_types::FromNode>::from_node(
                                node,
                                src,
                            )?,
                        ),
                    ),
                )
            }
            "list_marker_star" => {
                Ok(
                    Self::ListMarkerStar(
                        ::std::boxed::Box::new(
                            <ListMarkerStar as ::treesitter_types::FromNode>::from_node(
                                node,
                                src,
                            )?,
                        ),
                    ),
                )
            }
            "paragraph" => {
                Ok(
                    Self::Paragraph(
                        ::std::boxed::Box::new(
                            <Paragraph as ::treesitter_types::FromNode>::from_node(
                                node,
                                src,
                            )?,
                        ),
                    ),
                )
            }
            "pipe_table" => {
                Ok(
                    Self::PipeTable(
                        ::std::boxed::Box::new(
                            <PipeTable as ::treesitter_types::FromNode>::from_node(
                                node,
                                src,
                            )?,
                        ),
                    ),
                )
            }
            "section" => {
                Ok(
                    Self::Section(
                        ::std::boxed::Box::new(
                            <Section as ::treesitter_types::FromNode>::from_node(
                                node,
                                src,
                            )?,
                        ),
                    ),
                )
            }
            "setext_heading" => {
                Ok(
                    Self::SetextHeading(
                        ::std::boxed::Box::new(
                            <SetextHeading as ::treesitter_types::FromNode>::from_node(
                                node,
                                src,
                            )?,
                        ),
                    ),
                )
            }
            "task_list_marker_checked" => {
                Ok(
                    Self::TaskListMarkerChecked(
                        ::std::boxed::Box::new(
                            <TaskListMarkerChecked as ::treesitter_types::FromNode>::from_node(
                                node,
                                src,
                            )?,
                        ),
                    ),
                )
            }
            "task_list_marker_unchecked" => {
                Ok(
                    Self::TaskListMarkerUnchecked(
                        ::std::boxed::Box::new(
                            <TaskListMarkerUnchecked as ::treesitter_types::FromNode>::from_node(
                                node,
                                src,
                            )?,
                        ),
                    ),
                )
            }
            "thematic_break" => {
                Ok(
                    Self::ThematicBreak(
                        ::std::boxed::Box::new(
                            <ThematicBreak as ::treesitter_types::FromNode>::from_node(
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
impl ::treesitter_types::Spanned for ListItemChildren<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        match self {
            Self::BlockContinuation(inner) => inner.span(),
            Self::BlockQuote(inner) => inner.span(),
            Self::FencedCodeBlock(inner) => inner.span(),
            Self::HtmlBlock(inner) => inner.span(),
            Self::IndentedCodeBlock(inner) => inner.span(),
            Self::LinkReferenceDefinition(inner) => inner.span(),
            Self::List(inner) => inner.span(),
            Self::ListMarkerDot(inner) => inner.span(),
            Self::ListMarkerMinus(inner) => inner.span(),
            Self::ListMarkerParenthesis(inner) => inner.span(),
            Self::ListMarkerPlus(inner) => inner.span(),
            Self::ListMarkerStar(inner) => inner.span(),
            Self::Paragraph(inner) => inner.span(),
            Self::PipeTable(inner) => inner.span(),
            Self::Section(inner) => inner.span(),
            Self::SetextHeading(inner) => inner.span(),
            Self::TaskListMarkerChecked(inner) => inner.span(),
            Self::TaskListMarkerUnchecked(inner) => inner.span(),
            Self::ThematicBreak(inner) => inner.span(),
        }
    }
}
#[derive(Debug, Clone)]
pub enum ParagraphChildren<'tree> {
    BlockContinuation(::std::boxed::Box<BlockContinuation<'tree>>),
    Inline(::std::boxed::Box<Inline<'tree>>),
}
impl<'tree> ::treesitter_types::FromNode<'tree> for ParagraphChildren<'tree> {
    #[allow(clippy::collapsible_else_if)]
    fn from_node(
        node: ::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        match node.kind() {
            "block_continuation" => {
                Ok(
                    Self::BlockContinuation(
                        ::std::boxed::Box::new(
                            <BlockContinuation as ::treesitter_types::FromNode>::from_node(
                                node,
                                src,
                            )?,
                        ),
                    ),
                )
            }
            "inline" => {
                Ok(
                    Self::Inline(
                        ::std::boxed::Box::new(
                            <Inline as ::treesitter_types::FromNode>::from_node(
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
impl ::treesitter_types::Spanned for ParagraphChildren<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        match self {
            Self::BlockContinuation(inner) => inner.span(),
            Self::Inline(inner) => inner.span(),
        }
    }
}
#[derive(Debug, Clone)]
pub enum PipeTableChildren<'tree> {
    BlockContinuation(::std::boxed::Box<BlockContinuation<'tree>>),
    PipeTableDelimiterRow(::std::boxed::Box<PipeTableDelimiterRow<'tree>>),
    PipeTableHeader(::std::boxed::Box<PipeTableHeader<'tree>>),
    PipeTableRow(::std::boxed::Box<PipeTableRow<'tree>>),
}
impl<'tree> ::treesitter_types::FromNode<'tree> for PipeTableChildren<'tree> {
    #[allow(clippy::collapsible_else_if)]
    fn from_node(
        node: ::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        match node.kind() {
            "block_continuation" => {
                Ok(
                    Self::BlockContinuation(
                        ::std::boxed::Box::new(
                            <BlockContinuation as ::treesitter_types::FromNode>::from_node(
                                node,
                                src,
                            )?,
                        ),
                    ),
                )
            }
            "pipe_table_delimiter_row" => {
                Ok(
                    Self::PipeTableDelimiterRow(
                        ::std::boxed::Box::new(
                            <PipeTableDelimiterRow as ::treesitter_types::FromNode>::from_node(
                                node,
                                src,
                            )?,
                        ),
                    ),
                )
            }
            "pipe_table_header" => {
                Ok(
                    Self::PipeTableHeader(
                        ::std::boxed::Box::new(
                            <PipeTableHeader as ::treesitter_types::FromNode>::from_node(
                                node,
                                src,
                            )?,
                        ),
                    ),
                )
            }
            "pipe_table_row" => {
                Ok(
                    Self::PipeTableRow(
                        ::std::boxed::Box::new(
                            <PipeTableRow as ::treesitter_types::FromNode>::from_node(
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
impl ::treesitter_types::Spanned for PipeTableChildren<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        match self {
            Self::BlockContinuation(inner) => inner.span(),
            Self::PipeTableDelimiterRow(inner) => inner.span(),
            Self::PipeTableHeader(inner) => inner.span(),
            Self::PipeTableRow(inner) => inner.span(),
        }
    }
}
#[derive(Debug, Clone)]
pub enum PipeTableDelimiterCellChildren<'tree> {
    PipeTableAlignLeft(::std::boxed::Box<PipeTableAlignLeft<'tree>>),
    PipeTableAlignRight(::std::boxed::Box<PipeTableAlignRight<'tree>>),
}
impl<'tree> ::treesitter_types::FromNode<'tree>
for PipeTableDelimiterCellChildren<'tree> {
    #[allow(clippy::collapsible_else_if)]
    fn from_node(
        node: ::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        match node.kind() {
            "pipe_table_align_left" => {
                Ok(
                    Self::PipeTableAlignLeft(
                        ::std::boxed::Box::new(
                            <PipeTableAlignLeft as ::treesitter_types::FromNode>::from_node(
                                node,
                                src,
                            )?,
                        ),
                    ),
                )
            }
            "pipe_table_align_right" => {
                Ok(
                    Self::PipeTableAlignRight(
                        ::std::boxed::Box::new(
                            <PipeTableAlignRight as ::treesitter_types::FromNode>::from_node(
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
impl ::treesitter_types::Spanned for PipeTableDelimiterCellChildren<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        match self {
            Self::PipeTableAlignLeft(inner) => inner.span(),
            Self::PipeTableAlignRight(inner) => inner.span(),
        }
    }
}
#[derive(Debug, Clone)]
pub enum SectionChildren<'tree> {
    AtxHeading(::std::boxed::Box<AtxHeading<'tree>>),
    BlockContinuation(::std::boxed::Box<BlockContinuation<'tree>>),
    BlockQuote(::std::boxed::Box<BlockQuote<'tree>>),
    FencedCodeBlock(::std::boxed::Box<FencedCodeBlock<'tree>>),
    HtmlBlock(::std::boxed::Box<HtmlBlock<'tree>>),
    IndentedCodeBlock(::std::boxed::Box<IndentedCodeBlock<'tree>>),
    LinkReferenceDefinition(::std::boxed::Box<LinkReferenceDefinition<'tree>>),
    List(::std::boxed::Box<List<'tree>>),
    Paragraph(::std::boxed::Box<Paragraph<'tree>>),
    PipeTable(::std::boxed::Box<PipeTable<'tree>>),
    Section(::std::boxed::Box<Section<'tree>>),
    SetextHeading(::std::boxed::Box<SetextHeading<'tree>>),
    ThematicBreak(::std::boxed::Box<ThematicBreak<'tree>>),
}
impl<'tree> ::treesitter_types::FromNode<'tree> for SectionChildren<'tree> {
    #[allow(clippy::collapsible_else_if)]
    fn from_node(
        node: ::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        match node.kind() {
            "atx_heading" => {
                Ok(
                    Self::AtxHeading(
                        ::std::boxed::Box::new(
                            <AtxHeading as ::treesitter_types::FromNode>::from_node(
                                node,
                                src,
                            )?,
                        ),
                    ),
                )
            }
            "block_continuation" => {
                Ok(
                    Self::BlockContinuation(
                        ::std::boxed::Box::new(
                            <BlockContinuation as ::treesitter_types::FromNode>::from_node(
                                node,
                                src,
                            )?,
                        ),
                    ),
                )
            }
            "block_quote" => {
                Ok(
                    Self::BlockQuote(
                        ::std::boxed::Box::new(
                            <BlockQuote as ::treesitter_types::FromNode>::from_node(
                                node,
                                src,
                            )?,
                        ),
                    ),
                )
            }
            "fenced_code_block" => {
                Ok(
                    Self::FencedCodeBlock(
                        ::std::boxed::Box::new(
                            <FencedCodeBlock as ::treesitter_types::FromNode>::from_node(
                                node,
                                src,
                            )?,
                        ),
                    ),
                )
            }
            "html_block" => {
                Ok(
                    Self::HtmlBlock(
                        ::std::boxed::Box::new(
                            <HtmlBlock as ::treesitter_types::FromNode>::from_node(
                                node,
                                src,
                            )?,
                        ),
                    ),
                )
            }
            "indented_code_block" => {
                Ok(
                    Self::IndentedCodeBlock(
                        ::std::boxed::Box::new(
                            <IndentedCodeBlock as ::treesitter_types::FromNode>::from_node(
                                node,
                                src,
                            )?,
                        ),
                    ),
                )
            }
            "link_reference_definition" => {
                Ok(
                    Self::LinkReferenceDefinition(
                        ::std::boxed::Box::new(
                            <LinkReferenceDefinition as ::treesitter_types::FromNode>::from_node(
                                node,
                                src,
                            )?,
                        ),
                    ),
                )
            }
            "list" => {
                Ok(
                    Self::List(
                        ::std::boxed::Box::new(
                            <List as ::treesitter_types::FromNode>::from_node(node, src)?,
                        ),
                    ),
                )
            }
            "paragraph" => {
                Ok(
                    Self::Paragraph(
                        ::std::boxed::Box::new(
                            <Paragraph as ::treesitter_types::FromNode>::from_node(
                                node,
                                src,
                            )?,
                        ),
                    ),
                )
            }
            "pipe_table" => {
                Ok(
                    Self::PipeTable(
                        ::std::boxed::Box::new(
                            <PipeTable as ::treesitter_types::FromNode>::from_node(
                                node,
                                src,
                            )?,
                        ),
                    ),
                )
            }
            "section" => {
                Ok(
                    Self::Section(
                        ::std::boxed::Box::new(
                            <Section as ::treesitter_types::FromNode>::from_node(
                                node,
                                src,
                            )?,
                        ),
                    ),
                )
            }
            "setext_heading" => {
                Ok(
                    Self::SetextHeading(
                        ::std::boxed::Box::new(
                            <SetextHeading as ::treesitter_types::FromNode>::from_node(
                                node,
                                src,
                            )?,
                        ),
                    ),
                )
            }
            "thematic_break" => {
                Ok(
                    Self::ThematicBreak(
                        ::std::boxed::Box::new(
                            <ThematicBreak as ::treesitter_types::FromNode>::from_node(
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
impl ::treesitter_types::Spanned for SectionChildren<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        match self {
            Self::AtxHeading(inner) => inner.span(),
            Self::BlockContinuation(inner) => inner.span(),
            Self::BlockQuote(inner) => inner.span(),
            Self::FencedCodeBlock(inner) => inner.span(),
            Self::HtmlBlock(inner) => inner.span(),
            Self::IndentedCodeBlock(inner) => inner.span(),
            Self::LinkReferenceDefinition(inner) => inner.span(),
            Self::List(inner) => inner.span(),
            Self::Paragraph(inner) => inner.span(),
            Self::PipeTable(inner) => inner.span(),
            Self::Section(inner) => inner.span(),
            Self::SetextHeading(inner) => inner.span(),
            Self::ThematicBreak(inner) => inner.span(),
        }
    }
}
#[derive(Debug, Clone)]
pub enum SetextHeadingChildren<'tree> {
    BlockContinuation(::std::boxed::Box<BlockContinuation<'tree>>),
    SetextH1Underline(::std::boxed::Box<SetextH1Underline<'tree>>),
    SetextH2Underline(::std::boxed::Box<SetextH2Underline<'tree>>),
}
impl<'tree> ::treesitter_types::FromNode<'tree> for SetextHeadingChildren<'tree> {
    #[allow(clippy::collapsible_else_if)]
    fn from_node(
        node: ::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        match node.kind() {
            "block_continuation" => {
                Ok(
                    Self::BlockContinuation(
                        ::std::boxed::Box::new(
                            <BlockContinuation as ::treesitter_types::FromNode>::from_node(
                                node,
                                src,
                            )?,
                        ),
                    ),
                )
            }
            "setext_h1_underline" => {
                Ok(
                    Self::SetextH1Underline(
                        ::std::boxed::Box::new(
                            <SetextH1Underline as ::treesitter_types::FromNode>::from_node(
                                node,
                                src,
                            )?,
                        ),
                    ),
                )
            }
            "setext_h2_underline" => {
                Ok(
                    Self::SetextH2Underline(
                        ::std::boxed::Box::new(
                            <SetextH2Underline as ::treesitter_types::FromNode>::from_node(
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
impl ::treesitter_types::Spanned for SetextHeadingChildren<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        match self {
            Self::BlockContinuation(inner) => inner.span(),
            Self::SetextH1Underline(inner) => inner.span(),
            Self::SetextH2Underline(inner) => inner.span(),
        }
    }
}
#[derive(Debug, Clone)]
pub enum AnyNode<'tree> {
    AtxHeading(AtxHeading<'tree>),
    BackslashEscape(BackslashEscape<'tree>),
    BlockQuote(BlockQuote<'tree>),
    CodeFenceContent(CodeFenceContent<'tree>),
    Document(Document<'tree>),
    FencedCodeBlock(FencedCodeBlock<'tree>),
    HtmlBlock(HtmlBlock<'tree>),
    IndentedCodeBlock(IndentedCodeBlock<'tree>),
    InfoString(InfoString<'tree>),
    Inline(Inline<'tree>),
    Language(Language<'tree>),
    LinkDestination(LinkDestination<'tree>),
    LinkLabel(LinkLabel<'tree>),
    LinkReferenceDefinition(LinkReferenceDefinition<'tree>),
    LinkTitle(LinkTitle<'tree>),
    List(List<'tree>),
    ListItem(ListItem<'tree>),
    ListMarkerDot(ListMarkerDot<'tree>),
    ListMarkerMinus(ListMarkerMinus<'tree>),
    ListMarkerParenthesis(ListMarkerParenthesis<'tree>),
    ListMarkerPlus(ListMarkerPlus<'tree>),
    ListMarkerStar(ListMarkerStar<'tree>),
    Paragraph(Paragraph<'tree>),
    PipeTable(PipeTable<'tree>),
    PipeTableCell(PipeTableCell<'tree>),
    PipeTableDelimiterCell(PipeTableDelimiterCell<'tree>),
    PipeTableDelimiterRow(PipeTableDelimiterRow<'tree>),
    PipeTableHeader(PipeTableHeader<'tree>),
    PipeTableRow(PipeTableRow<'tree>),
    Section(Section<'tree>),
    SetextHeading(SetextHeading<'tree>),
    TaskListMarkerChecked(TaskListMarkerChecked<'tree>),
    TaskListMarkerUnchecked(TaskListMarkerUnchecked<'tree>),
    ThematicBreak(ThematicBreak<'tree>),
    AtxH1Marker(AtxH1Marker<'tree>),
    AtxH2Marker(AtxH2Marker<'tree>),
    AtxH3Marker(AtxH3Marker<'tree>),
    AtxH4Marker(AtxH4Marker<'tree>),
    AtxH5Marker(AtxH5Marker<'tree>),
    AtxH6Marker(AtxH6Marker<'tree>),
    BlockContinuation(BlockContinuation<'tree>),
    BlockQuoteMarker(BlockQuoteMarker<'tree>),
    EntityReference(EntityReference<'tree>),
    FencedCodeBlockDelimiter(FencedCodeBlockDelimiter<'tree>),
    MinusMetadata(MinusMetadata<'tree>),
    NumericCharacterReference(NumericCharacterReference<'tree>),
    PipeTableAlignLeft(PipeTableAlignLeft<'tree>),
    PipeTableAlignRight(PipeTableAlignRight<'tree>),
    PlusMetadata(PlusMetadata<'tree>),
    SetextH1Underline(SetextH1Underline<'tree>),
    SetextH2Underline(SetextH2Underline<'tree>),
    Unknown(::tree_sitter::Node<'tree>),
}
impl<'tree> AnyNode<'tree> {
    pub fn from_node(node: ::tree_sitter::Node<'tree>, src: &'tree [u8]) -> Self {
        match node.kind() {
            "atx_heading" => {
                <AtxHeading as ::treesitter_types::FromNode>::from_node(node, src)
                    .map(Self::AtxHeading)
                    .unwrap_or(Self::Unknown(node))
            }
            "backslash_escape" => {
                <BackslashEscape as ::treesitter_types::FromNode>::from_node(node, src)
                    .map(Self::BackslashEscape)
                    .unwrap_or(Self::Unknown(node))
            }
            "block_quote" => {
                <BlockQuote as ::treesitter_types::FromNode>::from_node(node, src)
                    .map(Self::BlockQuote)
                    .unwrap_or(Self::Unknown(node))
            }
            "code_fence_content" => {
                <CodeFenceContent as ::treesitter_types::FromNode>::from_node(node, src)
                    .map(Self::CodeFenceContent)
                    .unwrap_or(Self::Unknown(node))
            }
            "document" => {
                <Document as ::treesitter_types::FromNode>::from_node(node, src)
                    .map(Self::Document)
                    .unwrap_or(Self::Unknown(node))
            }
            "fenced_code_block" => {
                <FencedCodeBlock as ::treesitter_types::FromNode>::from_node(node, src)
                    .map(Self::FencedCodeBlock)
                    .unwrap_or(Self::Unknown(node))
            }
            "html_block" => {
                <HtmlBlock as ::treesitter_types::FromNode>::from_node(node, src)
                    .map(Self::HtmlBlock)
                    .unwrap_or(Self::Unknown(node))
            }
            "indented_code_block" => {
                <IndentedCodeBlock as ::treesitter_types::FromNode>::from_node(node, src)
                    .map(Self::IndentedCodeBlock)
                    .unwrap_or(Self::Unknown(node))
            }
            "info_string" => {
                <InfoString as ::treesitter_types::FromNode>::from_node(node, src)
                    .map(Self::InfoString)
                    .unwrap_or(Self::Unknown(node))
            }
            "inline" => {
                <Inline as ::treesitter_types::FromNode>::from_node(node, src)
                    .map(Self::Inline)
                    .unwrap_or(Self::Unknown(node))
            }
            "language" => {
                <Language as ::treesitter_types::FromNode>::from_node(node, src)
                    .map(Self::Language)
                    .unwrap_or(Self::Unknown(node))
            }
            "link_destination" => {
                <LinkDestination as ::treesitter_types::FromNode>::from_node(node, src)
                    .map(Self::LinkDestination)
                    .unwrap_or(Self::Unknown(node))
            }
            "link_label" => {
                <LinkLabel as ::treesitter_types::FromNode>::from_node(node, src)
                    .map(Self::LinkLabel)
                    .unwrap_or(Self::Unknown(node))
            }
            "link_reference_definition" => {
                <LinkReferenceDefinition as ::treesitter_types::FromNode>::from_node(
                        node,
                        src,
                    )
                    .map(Self::LinkReferenceDefinition)
                    .unwrap_or(Self::Unknown(node))
            }
            "link_title" => {
                <LinkTitle as ::treesitter_types::FromNode>::from_node(node, src)
                    .map(Self::LinkTitle)
                    .unwrap_or(Self::Unknown(node))
            }
            "list" => {
                <List as ::treesitter_types::FromNode>::from_node(node, src)
                    .map(Self::List)
                    .unwrap_or(Self::Unknown(node))
            }
            "list_item" => {
                <ListItem as ::treesitter_types::FromNode>::from_node(node, src)
                    .map(Self::ListItem)
                    .unwrap_or(Self::Unknown(node))
            }
            "list_marker_dot" => {
                <ListMarkerDot as ::treesitter_types::FromNode>::from_node(node, src)
                    .map(Self::ListMarkerDot)
                    .unwrap_or(Self::Unknown(node))
            }
            "list_marker_minus" => {
                <ListMarkerMinus as ::treesitter_types::FromNode>::from_node(node, src)
                    .map(Self::ListMarkerMinus)
                    .unwrap_or(Self::Unknown(node))
            }
            "list_marker_parenthesis" => {
                <ListMarkerParenthesis as ::treesitter_types::FromNode>::from_node(
                        node,
                        src,
                    )
                    .map(Self::ListMarkerParenthesis)
                    .unwrap_or(Self::Unknown(node))
            }
            "list_marker_plus" => {
                <ListMarkerPlus as ::treesitter_types::FromNode>::from_node(node, src)
                    .map(Self::ListMarkerPlus)
                    .unwrap_or(Self::Unknown(node))
            }
            "list_marker_star" => {
                <ListMarkerStar as ::treesitter_types::FromNode>::from_node(node, src)
                    .map(Self::ListMarkerStar)
                    .unwrap_or(Self::Unknown(node))
            }
            "paragraph" => {
                <Paragraph as ::treesitter_types::FromNode>::from_node(node, src)
                    .map(Self::Paragraph)
                    .unwrap_or(Self::Unknown(node))
            }
            "pipe_table" => {
                <PipeTable as ::treesitter_types::FromNode>::from_node(node, src)
                    .map(Self::PipeTable)
                    .unwrap_or(Self::Unknown(node))
            }
            "pipe_table_cell" => {
                <PipeTableCell as ::treesitter_types::FromNode>::from_node(node, src)
                    .map(Self::PipeTableCell)
                    .unwrap_or(Self::Unknown(node))
            }
            "pipe_table_delimiter_cell" => {
                <PipeTableDelimiterCell as ::treesitter_types::FromNode>::from_node(
                        node,
                        src,
                    )
                    .map(Self::PipeTableDelimiterCell)
                    .unwrap_or(Self::Unknown(node))
            }
            "pipe_table_delimiter_row" => {
                <PipeTableDelimiterRow as ::treesitter_types::FromNode>::from_node(
                        node,
                        src,
                    )
                    .map(Self::PipeTableDelimiterRow)
                    .unwrap_or(Self::Unknown(node))
            }
            "pipe_table_header" => {
                <PipeTableHeader as ::treesitter_types::FromNode>::from_node(node, src)
                    .map(Self::PipeTableHeader)
                    .unwrap_or(Self::Unknown(node))
            }
            "pipe_table_row" => {
                <PipeTableRow as ::treesitter_types::FromNode>::from_node(node, src)
                    .map(Self::PipeTableRow)
                    .unwrap_or(Self::Unknown(node))
            }
            "section" => {
                <Section as ::treesitter_types::FromNode>::from_node(node, src)
                    .map(Self::Section)
                    .unwrap_or(Self::Unknown(node))
            }
            "setext_heading" => {
                <SetextHeading as ::treesitter_types::FromNode>::from_node(node, src)
                    .map(Self::SetextHeading)
                    .unwrap_or(Self::Unknown(node))
            }
            "task_list_marker_checked" => {
                <TaskListMarkerChecked as ::treesitter_types::FromNode>::from_node(
                        node,
                        src,
                    )
                    .map(Self::TaskListMarkerChecked)
                    .unwrap_or(Self::Unknown(node))
            }
            "task_list_marker_unchecked" => {
                <TaskListMarkerUnchecked as ::treesitter_types::FromNode>::from_node(
                        node,
                        src,
                    )
                    .map(Self::TaskListMarkerUnchecked)
                    .unwrap_or(Self::Unknown(node))
            }
            "thematic_break" => {
                <ThematicBreak as ::treesitter_types::FromNode>::from_node(node, src)
                    .map(Self::ThematicBreak)
                    .unwrap_or(Self::Unknown(node))
            }
            "atx_h1_marker" => {
                <AtxH1Marker as ::treesitter_types::FromNode>::from_node(node, src)
                    .map(Self::AtxH1Marker)
                    .unwrap_or(Self::Unknown(node))
            }
            "atx_h2_marker" => {
                <AtxH2Marker as ::treesitter_types::FromNode>::from_node(node, src)
                    .map(Self::AtxH2Marker)
                    .unwrap_or(Self::Unknown(node))
            }
            "atx_h3_marker" => {
                <AtxH3Marker as ::treesitter_types::FromNode>::from_node(node, src)
                    .map(Self::AtxH3Marker)
                    .unwrap_or(Self::Unknown(node))
            }
            "atx_h4_marker" => {
                <AtxH4Marker as ::treesitter_types::FromNode>::from_node(node, src)
                    .map(Self::AtxH4Marker)
                    .unwrap_or(Self::Unknown(node))
            }
            "atx_h5_marker" => {
                <AtxH5Marker as ::treesitter_types::FromNode>::from_node(node, src)
                    .map(Self::AtxH5Marker)
                    .unwrap_or(Self::Unknown(node))
            }
            "atx_h6_marker" => {
                <AtxH6Marker as ::treesitter_types::FromNode>::from_node(node, src)
                    .map(Self::AtxH6Marker)
                    .unwrap_or(Self::Unknown(node))
            }
            "block_continuation" => {
                <BlockContinuation as ::treesitter_types::FromNode>::from_node(node, src)
                    .map(Self::BlockContinuation)
                    .unwrap_or(Self::Unknown(node))
            }
            "block_quote_marker" => {
                <BlockQuoteMarker as ::treesitter_types::FromNode>::from_node(node, src)
                    .map(Self::BlockQuoteMarker)
                    .unwrap_or(Self::Unknown(node))
            }
            "entity_reference" => {
                <EntityReference as ::treesitter_types::FromNode>::from_node(node, src)
                    .map(Self::EntityReference)
                    .unwrap_or(Self::Unknown(node))
            }
            "fenced_code_block_delimiter" => {
                <FencedCodeBlockDelimiter as ::treesitter_types::FromNode>::from_node(
                        node,
                        src,
                    )
                    .map(Self::FencedCodeBlockDelimiter)
                    .unwrap_or(Self::Unknown(node))
            }
            "minus_metadata" => {
                <MinusMetadata as ::treesitter_types::FromNode>::from_node(node, src)
                    .map(Self::MinusMetadata)
                    .unwrap_or(Self::Unknown(node))
            }
            "numeric_character_reference" => {
                <NumericCharacterReference as ::treesitter_types::FromNode>::from_node(
                        node,
                        src,
                    )
                    .map(Self::NumericCharacterReference)
                    .unwrap_or(Self::Unknown(node))
            }
            "pipe_table_align_left" => {
                <PipeTableAlignLeft as ::treesitter_types::FromNode>::from_node(
                        node,
                        src,
                    )
                    .map(Self::PipeTableAlignLeft)
                    .unwrap_or(Self::Unknown(node))
            }
            "pipe_table_align_right" => {
                <PipeTableAlignRight as ::treesitter_types::FromNode>::from_node(
                        node,
                        src,
                    )
                    .map(Self::PipeTableAlignRight)
                    .unwrap_or(Self::Unknown(node))
            }
            "plus_metadata" => {
                <PlusMetadata as ::treesitter_types::FromNode>::from_node(node, src)
                    .map(Self::PlusMetadata)
                    .unwrap_or(Self::Unknown(node))
            }
            "setext_h1_underline" => {
                <SetextH1Underline as ::treesitter_types::FromNode>::from_node(node, src)
                    .map(Self::SetextH1Underline)
                    .unwrap_or(Self::Unknown(node))
            }
            "setext_h2_underline" => {
                <SetextH2Underline as ::treesitter_types::FromNode>::from_node(node, src)
                    .map(Self::SetextH2Underline)
                    .unwrap_or(Self::Unknown(node))
            }
            _ => Self::Unknown(node),
        }
    }
}
impl ::treesitter_types::Spanned for AnyNode<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        match self {
            Self::AtxHeading(inner) => inner.span(),
            Self::BackslashEscape(inner) => inner.span(),
            Self::BlockQuote(inner) => inner.span(),
            Self::CodeFenceContent(inner) => inner.span(),
            Self::Document(inner) => inner.span(),
            Self::FencedCodeBlock(inner) => inner.span(),
            Self::HtmlBlock(inner) => inner.span(),
            Self::IndentedCodeBlock(inner) => inner.span(),
            Self::InfoString(inner) => inner.span(),
            Self::Inline(inner) => inner.span(),
            Self::Language(inner) => inner.span(),
            Self::LinkDestination(inner) => inner.span(),
            Self::LinkLabel(inner) => inner.span(),
            Self::LinkReferenceDefinition(inner) => inner.span(),
            Self::LinkTitle(inner) => inner.span(),
            Self::List(inner) => inner.span(),
            Self::ListItem(inner) => inner.span(),
            Self::ListMarkerDot(inner) => inner.span(),
            Self::ListMarkerMinus(inner) => inner.span(),
            Self::ListMarkerParenthesis(inner) => inner.span(),
            Self::ListMarkerPlus(inner) => inner.span(),
            Self::ListMarkerStar(inner) => inner.span(),
            Self::Paragraph(inner) => inner.span(),
            Self::PipeTable(inner) => inner.span(),
            Self::PipeTableCell(inner) => inner.span(),
            Self::PipeTableDelimiterCell(inner) => inner.span(),
            Self::PipeTableDelimiterRow(inner) => inner.span(),
            Self::PipeTableHeader(inner) => inner.span(),
            Self::PipeTableRow(inner) => inner.span(),
            Self::Section(inner) => inner.span(),
            Self::SetextHeading(inner) => inner.span(),
            Self::TaskListMarkerChecked(inner) => inner.span(),
            Self::TaskListMarkerUnchecked(inner) => inner.span(),
            Self::ThematicBreak(inner) => inner.span(),
            Self::AtxH1Marker(inner) => inner.span(),
            Self::AtxH2Marker(inner) => inner.span(),
            Self::AtxH3Marker(inner) => inner.span(),
            Self::AtxH4Marker(inner) => inner.span(),
            Self::AtxH5Marker(inner) => inner.span(),
            Self::AtxH6Marker(inner) => inner.span(),
            Self::BlockContinuation(inner) => inner.span(),
            Self::BlockQuoteMarker(inner) => inner.span(),
            Self::EntityReference(inner) => inner.span(),
            Self::FencedCodeBlockDelimiter(inner) => inner.span(),
            Self::MinusMetadata(inner) => inner.span(),
            Self::NumericCharacterReference(inner) => inner.span(),
            Self::PipeTableAlignLeft(inner) => inner.span(),
            Self::PipeTableAlignRight(inner) => inner.span(),
            Self::PlusMetadata(inner) => inner.span(),
            Self::SetextH1Underline(inner) => inner.span(),
            Self::SetextH2Underline(inner) => inner.span(),
            Self::Unknown(node) => ::treesitter_types::Span::from(*node),
        }
    }
}
