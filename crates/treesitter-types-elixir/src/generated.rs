#[derive(Debug, Clone, PartialEq, Eq)]
pub struct AccessCall<'tree> {
    pub span: ::treesitter_types::Span,
    pub key: AccessCallKey<'tree>,
    pub target: AccessCallTarget<'tree>,
}
impl<'tree> ::treesitter_types::FromNode<'tree> for AccessCall<'tree> {
    #[allow(clippy::match_single_binding, clippy::suspicious_else_formatting)]
    fn from_node(
        node: ::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        debug_assert_eq!(node.kind(), "access_call");
        Ok(Self {
            span: ::treesitter_types::Span::from(node),
            key: {
                let child = node
                    .child_by_field_name("key")
                    .ok_or_else(|| ::treesitter_types::ParseError::missing_field("key", node))?;
                <AccessCallKey as ::treesitter_types::FromNode>::from_node(child, src)?
            },
            target: {
                let child = node
                    .child_by_field_name("target")
                    .ok_or_else(|| ::treesitter_types::ParseError::missing_field("target", node))?;
                <AccessCallTarget as ::treesitter_types::FromNode>::from_node(child, src)?
            },
        })
    }
}
impl ::treesitter_types::Spanned for AccessCall<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        self.span
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct AfterBlock<'tree> {
    pub span: ::treesitter_types::Span,
    pub children: ::std::vec::Vec<AfterBlockChildren<'tree>>,
}
impl<'tree> ::treesitter_types::FromNode<'tree> for AfterBlock<'tree> {
    #[allow(clippy::match_single_binding, clippy::suspicious_else_formatting)]
    fn from_node(
        node: ::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        debug_assert_eq!(node.kind(), "after_block");
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
                    items.push(
                        <AfterBlockChildren as ::treesitter_types::FromNode>::from_node(
                            child, src,
                        )?,
                    );
                }
                items
            },
        })
    }
}
impl ::treesitter_types::Spanned for AfterBlock<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        self.span
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct AnonymousFunction<'tree> {
    pub span: ::treesitter_types::Span,
    pub children: ::std::vec::Vec<StabClause<'tree>>,
}
impl<'tree> ::treesitter_types::FromNode<'tree> for AnonymousFunction<'tree> {
    #[allow(clippy::match_single_binding, clippy::suspicious_else_formatting)]
    fn from_node(
        node: ::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        debug_assert_eq!(node.kind(), "anonymous_function");
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
                    items.push(<StabClause as ::treesitter_types::FromNode>::from_node(
                        child, src,
                    )?);
                }
                items
            },
        })
    }
}
impl ::treesitter_types::Spanned for AnonymousFunction<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        self.span
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Arguments<'tree> {
    pub span: ::treesitter_types::Span,
    pub children: ::std::vec::Vec<ArgumentsChildren<'tree>>,
}
impl<'tree> ::treesitter_types::FromNode<'tree> for Arguments<'tree> {
    #[allow(clippy::match_single_binding, clippy::suspicious_else_formatting)]
    fn from_node(
        node: ::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        debug_assert_eq!(node.kind(), "arguments");
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
                    items.push(
                        <ArgumentsChildren as ::treesitter_types::FromNode>::from_node(child, src)?,
                    );
                }
                items
            },
        })
    }
}
impl ::treesitter_types::Spanned for Arguments<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        self.span
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct BinaryOperator<'tree> {
    pub span: ::treesitter_types::Span,
    pub left: BinaryOperatorLeft<'tree>,
    pub operator: BinaryOperatorOperator,
    pub right: BinaryOperatorRight<'tree>,
}
impl<'tree> ::treesitter_types::FromNode<'tree> for BinaryOperator<'tree> {
    #[allow(clippy::match_single_binding, clippy::suspicious_else_formatting)]
    fn from_node(
        node: ::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        debug_assert_eq!(node.kind(), "binary_operator");
        Ok(Self {
            span: ::treesitter_types::Span::from(node),
            left: {
                let child = node
                    .child_by_field_name("left")
                    .ok_or_else(|| ::treesitter_types::ParseError::missing_field("left", node))?;
                <BinaryOperatorLeft as ::treesitter_types::FromNode>::from_node(child, src)?
            },
            operator: {
                let child = node.child_by_field_name("operator").ok_or_else(|| {
                    ::treesitter_types::ParseError::missing_field("operator", node)
                })?;
                <BinaryOperatorOperator as ::treesitter_types::FromNode>::from_node(child, src)?
            },
            right: {
                let child = node
                    .child_by_field_name("right")
                    .ok_or_else(|| ::treesitter_types::ParseError::missing_field("right", node))?;
                <BinaryOperatorRight as ::treesitter_types::FromNode>::from_node(child, src)?
            },
        })
    }
}
impl ::treesitter_types::Spanned for BinaryOperator<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        self.span
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Bitstring<'tree> {
    pub span: ::treesitter_types::Span,
    pub children: ::std::vec::Vec<BitstringChildren<'tree>>,
}
impl<'tree> ::treesitter_types::FromNode<'tree> for Bitstring<'tree> {
    #[allow(clippy::match_single_binding, clippy::suspicious_else_formatting)]
    fn from_node(
        node: ::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        debug_assert_eq!(node.kind(), "bitstring");
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
                    items.push(
                        <BitstringChildren as ::treesitter_types::FromNode>::from_node(child, src)?,
                    );
                }
                items
            },
        })
    }
}
impl ::treesitter_types::Spanned for Bitstring<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        self.span
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Block<'tree> {
    pub span: ::treesitter_types::Span,
    pub children: ::std::vec::Vec<BlockChildren<'tree>>,
}
impl<'tree> ::treesitter_types::FromNode<'tree> for Block<'tree> {
    #[allow(clippy::match_single_binding, clippy::suspicious_else_formatting)]
    fn from_node(
        node: ::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        debug_assert_eq!(node.kind(), "block");
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
                    items.push(<BlockChildren as ::treesitter_types::FromNode>::from_node(
                        child, src,
                    )?);
                }
                items
            },
        })
    }
}
impl ::treesitter_types::Spanned for Block<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        self.span
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Body<'tree> {
    pub span: ::treesitter_types::Span,
    pub children: ::std::vec::Vec<BodyChildren<'tree>>,
}
impl<'tree> ::treesitter_types::FromNode<'tree> for Body<'tree> {
    #[allow(clippy::match_single_binding, clippy::suspicious_else_formatting)]
    fn from_node(
        node: ::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        debug_assert_eq!(node.kind(), "body");
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
                    items.push(<BodyChildren as ::treesitter_types::FromNode>::from_node(
                        child, src,
                    )?);
                }
                items
            },
        })
    }
}
impl ::treesitter_types::Spanned for Body<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        self.span
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Boolean<'tree> {
    pub span: ::treesitter_types::Span,
    text: &'tree str,
}
impl<'tree> ::treesitter_types::FromNode<'tree> for Boolean<'tree> {
    fn from_node(
        node: ::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        debug_assert_eq!(node.kind(), "boolean");
        Ok(Self {
            span: ::treesitter_types::Span::from(node),
            text: node.utf8_text(src)?,
        })
    }
}
impl<'tree> ::treesitter_types::LeafNode<'tree> for Boolean<'tree> {
    fn text(&self) -> &'tree str {
        self.text
    }
}
impl ::treesitter_types::Spanned for Boolean<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        self.span
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Call<'tree> {
    pub span: ::treesitter_types::Span,
    pub target: CallTarget<'tree>,
    pub children: ::std::vec::Vec<CallChildren<'tree>>,
}
impl<'tree> ::treesitter_types::FromNode<'tree> for Call<'tree> {
    #[allow(clippy::match_single_binding, clippy::suspicious_else_formatting)]
    fn from_node(
        node: ::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        debug_assert_eq!(node.kind(), "call");
        Ok(Self {
            span: ::treesitter_types::Span::from(node),
            target: {
                let child = node
                    .child_by_field_name("target")
                    .ok_or_else(|| ::treesitter_types::ParseError::missing_field("target", node))?;
                <CallTarget as ::treesitter_types::FromNode>::from_node(child, src)?
            },
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
                    items.push(<CallChildren as ::treesitter_types::FromNode>::from_node(
                        child, src,
                    )?);
                }
                items
            },
        })
    }
}
impl ::treesitter_types::Spanned for Call<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        self.span
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CatchBlock<'tree> {
    pub span: ::treesitter_types::Span,
    pub children: ::std::vec::Vec<CatchBlockChildren<'tree>>,
}
impl<'tree> ::treesitter_types::FromNode<'tree> for CatchBlock<'tree> {
    #[allow(clippy::match_single_binding, clippy::suspicious_else_formatting)]
    fn from_node(
        node: ::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        debug_assert_eq!(node.kind(), "catch_block");
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
                    items.push(
                        <CatchBlockChildren as ::treesitter_types::FromNode>::from_node(
                            child, src,
                        )?,
                    );
                }
                items
            },
        })
    }
}
impl ::treesitter_types::Spanned for CatchBlock<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        self.span
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Charlist<'tree> {
    pub span: ::treesitter_types::Span,
    pub quoted_end: CharlistQuotedEnd,
    pub quoted_start: CharlistQuotedStart,
    pub children: ::std::vec::Vec<CharlistChildren<'tree>>,
}
impl<'tree> ::treesitter_types::FromNode<'tree> for Charlist<'tree> {
    #[allow(clippy::match_single_binding, clippy::suspicious_else_formatting)]
    fn from_node(
        node: ::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        debug_assert_eq!(node.kind(), "charlist");
        Ok(Self {
            span: ::treesitter_types::Span::from(node),
            quoted_end: {
                let child = node.child_by_field_name("quoted_end").ok_or_else(|| {
                    ::treesitter_types::ParseError::missing_field("quoted_end", node)
                })?;
                <CharlistQuotedEnd as ::treesitter_types::FromNode>::from_node(child, src)?
            },
            quoted_start: {
                let child = node.child_by_field_name("quoted_start").ok_or_else(|| {
                    ::treesitter_types::ParseError::missing_field("quoted_start", node)
                })?;
                <CharlistQuotedStart as ::treesitter_types::FromNode>::from_node(child, src)?
            },
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
                    items.push(
                        <CharlistChildren as ::treesitter_types::FromNode>::from_node(child, src)?,
                    );
                }
                items
            },
        })
    }
}
impl ::treesitter_types::Spanned for Charlist<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        self.span
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct DoBlock<'tree> {
    pub span: ::treesitter_types::Span,
    pub children: ::std::vec::Vec<DoBlockChildren<'tree>>,
}
impl<'tree> ::treesitter_types::FromNode<'tree> for DoBlock<'tree> {
    #[allow(clippy::match_single_binding, clippy::suspicious_else_formatting)]
    fn from_node(
        node: ::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        debug_assert_eq!(node.kind(), "do_block");
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
                    items.push(
                        <DoBlockChildren as ::treesitter_types::FromNode>::from_node(child, src)?,
                    );
                }
                items
            },
        })
    }
}
impl ::treesitter_types::Spanned for DoBlock<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        self.span
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Dot<'tree> {
    pub span: ::treesitter_types::Span,
    pub left: DotLeft<'tree>,
    pub operator: DotOperator,
    pub right: ::core::option::Option<DotRight<'tree>>,
}
impl<'tree> ::treesitter_types::FromNode<'tree> for Dot<'tree> {
    #[allow(clippy::match_single_binding, clippy::suspicious_else_formatting)]
    fn from_node(
        node: ::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        debug_assert_eq!(node.kind(), "dot");
        Ok(Self {
            span: ::treesitter_types::Span::from(node),
            left: {
                let child = node
                    .child_by_field_name("left")
                    .ok_or_else(|| ::treesitter_types::ParseError::missing_field("left", node))?;
                <DotLeft as ::treesitter_types::FromNode>::from_node(child, src)?
            },
            operator: {
                let child = node.child_by_field_name("operator").ok_or_else(|| {
                    ::treesitter_types::ParseError::missing_field("operator", node)
                })?;
                <DotOperator as ::treesitter_types::FromNode>::from_node(child, src)?
            },
            right: match node.child_by_field_name("right") {
                Some(child) => Some(<DotRight as ::treesitter_types::FromNode>::from_node(
                    child, src,
                )?),
                None => None,
            },
        })
    }
}
impl ::treesitter_types::Spanned for Dot<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        self.span
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ElseBlock<'tree> {
    pub span: ::treesitter_types::Span,
    pub children: ::std::vec::Vec<ElseBlockChildren<'tree>>,
}
impl<'tree> ::treesitter_types::FromNode<'tree> for ElseBlock<'tree> {
    #[allow(clippy::match_single_binding, clippy::suspicious_else_formatting)]
    fn from_node(
        node: ::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        debug_assert_eq!(node.kind(), "else_block");
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
                    items.push(
                        <ElseBlockChildren as ::treesitter_types::FromNode>::from_node(child, src)?,
                    );
                }
                items
            },
        })
    }
}
impl ::treesitter_types::Spanned for ElseBlock<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        self.span
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Identifier<'tree> {
    pub span: ::treesitter_types::Span,
    text: &'tree str,
}
impl<'tree> ::treesitter_types::FromNode<'tree> for Identifier<'tree> {
    fn from_node(
        node: ::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        debug_assert_eq!(node.kind(), "identifier");
        Ok(Self {
            span: ::treesitter_types::Span::from(node),
            text: node.utf8_text(src)?,
        })
    }
}
impl<'tree> ::treesitter_types::LeafNode<'tree> for Identifier<'tree> {
    fn text(&self) -> &'tree str {
        self.text
    }
}
impl ::treesitter_types::Spanned for Identifier<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        self.span
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Interpolation<'tree> {
    pub span: ::treesitter_types::Span,
    pub children: ::core::option::Option<InterpolationChildren<'tree>>,
}
impl<'tree> ::treesitter_types::FromNode<'tree> for Interpolation<'tree> {
    #[allow(clippy::match_single_binding, clippy::suspicious_else_formatting)]
    fn from_node(
        node: ::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        debug_assert_eq!(node.kind(), "interpolation");
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
                    Some(&child) => Some(
                        <InterpolationChildren as ::treesitter_types::FromNode>::from_node(
                            child, src,
                        )?,
                    ),
                    None => None,
                }
            },
        })
    }
}
impl ::treesitter_types::Spanned for Interpolation<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        self.span
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Keywords<'tree> {
    pub span: ::treesitter_types::Span,
    pub children: ::std::vec::Vec<Pair<'tree>>,
}
impl<'tree> ::treesitter_types::FromNode<'tree> for Keywords<'tree> {
    #[allow(clippy::match_single_binding, clippy::suspicious_else_formatting)]
    fn from_node(
        node: ::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        debug_assert_eq!(node.kind(), "keywords");
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
                    items.push(<Pair as ::treesitter_types::FromNode>::from_node(
                        child, src,
                    )?);
                }
                items
            },
        })
    }
}
impl ::treesitter_types::Spanned for Keywords<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        self.span
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct List<'tree> {
    pub span: ::treesitter_types::Span,
    pub children: ::std::vec::Vec<ListChildren<'tree>>,
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
                    items.push(<ListChildren as ::treesitter_types::FromNode>::from_node(
                        child, src,
                    )?);
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
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Map<'tree> {
    pub span: ::treesitter_types::Span,
    pub children: ::std::vec::Vec<MapChildren<'tree>>,
}
impl<'tree> ::treesitter_types::FromNode<'tree> for Map<'tree> {
    #[allow(clippy::match_single_binding, clippy::suspicious_else_formatting)]
    fn from_node(
        node: ::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        debug_assert_eq!(node.kind(), "map");
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
                    items.push(<MapChildren as ::treesitter_types::FromNode>::from_node(
                        child, src,
                    )?);
                }
                items
            },
        })
    }
}
impl ::treesitter_types::Spanned for Map<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        self.span
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct MapContent<'tree> {
    pub span: ::treesitter_types::Span,
    pub children: ::std::vec::Vec<MapContentChildren<'tree>>,
}
impl<'tree> ::treesitter_types::FromNode<'tree> for MapContent<'tree> {
    #[allow(clippy::match_single_binding, clippy::suspicious_else_formatting)]
    fn from_node(
        node: ::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        debug_assert_eq!(node.kind(), "map_content");
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
                    items.push(
                        <MapContentChildren as ::treesitter_types::FromNode>::from_node(
                            child, src,
                        )?,
                    );
                }
                items
            },
        })
    }
}
impl ::treesitter_types::Spanned for MapContent<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        self.span
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Nil<'tree> {
    pub span: ::treesitter_types::Span,
    text: &'tree str,
}
impl<'tree> ::treesitter_types::FromNode<'tree> for Nil<'tree> {
    fn from_node(
        node: ::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        debug_assert_eq!(node.kind(), "nil");
        Ok(Self {
            span: ::treesitter_types::Span::from(node),
            text: node.utf8_text(src)?,
        })
    }
}
impl<'tree> ::treesitter_types::LeafNode<'tree> for Nil<'tree> {
    fn text(&self) -> &'tree str {
        self.text
    }
}
impl ::treesitter_types::Spanned for Nil<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        self.span
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct OperatorIdentifier<'tree> {
    pub span: ::treesitter_types::Span,
    text: &'tree str,
}
impl<'tree> ::treesitter_types::FromNode<'tree> for OperatorIdentifier<'tree> {
    fn from_node(
        node: ::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        debug_assert_eq!(node.kind(), "operator_identifier");
        Ok(Self {
            span: ::treesitter_types::Span::from(node),
            text: node.utf8_text(src)?,
        })
    }
}
impl<'tree> ::treesitter_types::LeafNode<'tree> for OperatorIdentifier<'tree> {
    fn text(&self) -> &'tree str {
        self.text
    }
}
impl ::treesitter_types::Spanned for OperatorIdentifier<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        self.span
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Pair<'tree> {
    pub span: ::treesitter_types::Span,
    pub key: PairKey<'tree>,
    pub value: PairValue<'tree>,
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
                <PairKey as ::treesitter_types::FromNode>::from_node(child, src)?
            },
            value: {
                let child = node
                    .child_by_field_name("value")
                    .ok_or_else(|| ::treesitter_types::ParseError::missing_field("value", node))?;
                <PairValue as ::treesitter_types::FromNode>::from_node(child, src)?
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
pub struct QuotedAtom<'tree> {
    pub span: ::treesitter_types::Span,
    pub quoted_end: QuotedAtomQuotedEnd,
    pub quoted_start: QuotedAtomQuotedStart,
    pub children: ::std::vec::Vec<QuotedAtomChildren<'tree>>,
}
impl<'tree> ::treesitter_types::FromNode<'tree> for QuotedAtom<'tree> {
    #[allow(clippy::match_single_binding, clippy::suspicious_else_formatting)]
    fn from_node(
        node: ::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        debug_assert_eq!(node.kind(), "quoted_atom");
        Ok(Self {
            span: ::treesitter_types::Span::from(node),
            quoted_end: {
                let child = node.child_by_field_name("quoted_end").ok_or_else(|| {
                    ::treesitter_types::ParseError::missing_field("quoted_end", node)
                })?;
                <QuotedAtomQuotedEnd as ::treesitter_types::FromNode>::from_node(child, src)?
            },
            quoted_start: {
                let child = node.child_by_field_name("quoted_start").ok_or_else(|| {
                    ::treesitter_types::ParseError::missing_field("quoted_start", node)
                })?;
                <QuotedAtomQuotedStart as ::treesitter_types::FromNode>::from_node(child, src)?
            },
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
                    items.push(
                        <QuotedAtomChildren as ::treesitter_types::FromNode>::from_node(
                            child, src,
                        )?,
                    );
                }
                items
            },
        })
    }
}
impl ::treesitter_types::Spanned for QuotedAtom<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        self.span
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct QuotedKeyword<'tree> {
    pub span: ::treesitter_types::Span,
    pub quoted_end: QuotedKeywordQuotedEnd,
    pub quoted_start: QuotedKeywordQuotedStart,
    pub children: ::std::vec::Vec<QuotedKeywordChildren<'tree>>,
}
impl<'tree> ::treesitter_types::FromNode<'tree> for QuotedKeyword<'tree> {
    #[allow(clippy::match_single_binding, clippy::suspicious_else_formatting)]
    fn from_node(
        node: ::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        debug_assert_eq!(node.kind(), "quoted_keyword");
        Ok(Self {
            span: ::treesitter_types::Span::from(node),
            quoted_end: {
                let child = node.child_by_field_name("quoted_end").ok_or_else(|| {
                    ::treesitter_types::ParseError::missing_field("quoted_end", node)
                })?;
                <QuotedKeywordQuotedEnd as ::treesitter_types::FromNode>::from_node(child, src)?
            },
            quoted_start: {
                let child = node.child_by_field_name("quoted_start").ok_or_else(|| {
                    ::treesitter_types::ParseError::missing_field("quoted_start", node)
                })?;
                <QuotedKeywordQuotedStart as ::treesitter_types::FromNode>::from_node(child, src)?
            },
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
                    items.push(
                        <QuotedKeywordChildren as ::treesitter_types::FromNode>::from_node(
                            child, src,
                        )?,
                    );
                }
                items
            },
        })
    }
}
impl ::treesitter_types::Spanned for QuotedKeyword<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        self.span
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct RescueBlock<'tree> {
    pub span: ::treesitter_types::Span,
    pub children: ::std::vec::Vec<RescueBlockChildren<'tree>>,
}
impl<'tree> ::treesitter_types::FromNode<'tree> for RescueBlock<'tree> {
    #[allow(clippy::match_single_binding, clippy::suspicious_else_formatting)]
    fn from_node(
        node: ::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        debug_assert_eq!(node.kind(), "rescue_block");
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
                    items.push(
                        <RescueBlockChildren as ::treesitter_types::FromNode>::from_node(
                            child, src,
                        )?,
                    );
                }
                items
            },
        })
    }
}
impl ::treesitter_types::Spanned for RescueBlock<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        self.span
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Sigil<'tree> {
    pub span: ::treesitter_types::Span,
    pub quoted_end: SigilQuotedEnd,
    pub quoted_start: SigilQuotedStart,
    pub children: ::std::vec::Vec<SigilChildren<'tree>>,
}
impl<'tree> ::treesitter_types::FromNode<'tree> for Sigil<'tree> {
    #[allow(clippy::match_single_binding, clippy::suspicious_else_formatting)]
    fn from_node(
        node: ::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        debug_assert_eq!(node.kind(), "sigil");
        Ok(Self {
            span: ::treesitter_types::Span::from(node),
            quoted_end: {
                let child = node.child_by_field_name("quoted_end").ok_or_else(|| {
                    ::treesitter_types::ParseError::missing_field("quoted_end", node)
                })?;
                <SigilQuotedEnd as ::treesitter_types::FromNode>::from_node(child, src)?
            },
            quoted_start: {
                let child = node.child_by_field_name("quoted_start").ok_or_else(|| {
                    ::treesitter_types::ParseError::missing_field("quoted_start", node)
                })?;
                <SigilQuotedStart as ::treesitter_types::FromNode>::from_node(child, src)?
            },
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
                    items.push(<SigilChildren as ::treesitter_types::FromNode>::from_node(
                        child, src,
                    )?);
                }
                items
            },
        })
    }
}
impl ::treesitter_types::Spanned for Sigil<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        self.span
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Source<'tree> {
    pub span: ::treesitter_types::Span,
    pub children: ::std::vec::Vec<SourceChildren<'tree>>,
}
impl<'tree> ::treesitter_types::FromNode<'tree> for Source<'tree> {
    #[allow(clippy::match_single_binding, clippy::suspicious_else_formatting)]
    fn from_node(
        node: ::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        debug_assert_eq!(node.kind(), "source");
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
                    items.push(<SourceChildren as ::treesitter_types::FromNode>::from_node(
                        child, src,
                    )?);
                }
                items
            },
        })
    }
}
impl ::treesitter_types::Spanned for Source<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        self.span
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct StabClause<'tree> {
    pub span: ::treesitter_types::Span,
    pub left: ::core::option::Option<StabClauseLeft<'tree>>,
    pub operator: StabClauseOperator,
    pub right: ::core::option::Option<Body<'tree>>,
}
impl<'tree> ::treesitter_types::FromNode<'tree> for StabClause<'tree> {
    #[allow(clippy::match_single_binding, clippy::suspicious_else_formatting)]
    fn from_node(
        node: ::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        debug_assert_eq!(node.kind(), "stab_clause");
        Ok(Self {
            span: ::treesitter_types::Span::from(node),
            left: match node.child_by_field_name("left") {
                Some(child) => Some(<StabClauseLeft as ::treesitter_types::FromNode>::from_node(
                    child, src,
                )?),
                None => None,
            },
            operator: {
                let child = node.child_by_field_name("operator").ok_or_else(|| {
                    ::treesitter_types::ParseError::missing_field("operator", node)
                })?;
                <StabClauseOperator as ::treesitter_types::FromNode>::from_node(child, src)?
            },
            right: match node.child_by_field_name("right") {
                Some(child) => Some(<Body as ::treesitter_types::FromNode>::from_node(
                    child, src,
                )?),
                None => None,
            },
        })
    }
}
impl ::treesitter_types::Spanned for StabClause<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        self.span
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct String<'tree> {
    pub span: ::treesitter_types::Span,
    pub quoted_end: StringQuotedEnd,
    pub quoted_start: StringQuotedStart,
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
            quoted_end: {
                let child = node.child_by_field_name("quoted_end").ok_or_else(|| {
                    ::treesitter_types::ParseError::missing_field("quoted_end", node)
                })?;
                <StringQuotedEnd as ::treesitter_types::FromNode>::from_node(child, src)?
            },
            quoted_start: {
                let child = node.child_by_field_name("quoted_start").ok_or_else(|| {
                    ::treesitter_types::ParseError::missing_field("quoted_start", node)
                })?;
                <StringQuotedStart as ::treesitter_types::FromNode>::from_node(child, src)?
            },
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
                    items.push(<StringChildren as ::treesitter_types::FromNode>::from_node(
                        child, src,
                    )?);
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
pub struct Struct<'tree> {
    pub span: ::treesitter_types::Span,
    pub children: StructChildren<'tree>,
}
impl<'tree> ::treesitter_types::FromNode<'tree> for Struct<'tree> {
    #[allow(clippy::match_single_binding, clippy::suspicious_else_formatting)]
    fn from_node(
        node: ::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        debug_assert_eq!(node.kind(), "struct");
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
                                        <StructChildren as ::treesitter_types::FromNode>::from_node(
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
                                            <StructChildren as ::treesitter_types::FromNode>::from_node(
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
                    fallback_child.ok_or_else(|| {
                        ::treesitter_types::ParseError::missing_field("children", node)
                    })?
                };
                <StructChildren as ::treesitter_types::FromNode>::from_node(child, src)?
            },
        })
    }
}
impl ::treesitter_types::Spanned for Struct<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        self.span
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Tuple<'tree> {
    pub span: ::treesitter_types::Span,
    pub children: ::std::vec::Vec<TupleChildren<'tree>>,
}
impl<'tree> ::treesitter_types::FromNode<'tree> for Tuple<'tree> {
    #[allow(clippy::match_single_binding, clippy::suspicious_else_formatting)]
    fn from_node(
        node: ::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        debug_assert_eq!(node.kind(), "tuple");
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
                    items.push(<TupleChildren as ::treesitter_types::FromNode>::from_node(
                        child, src,
                    )?);
                }
                items
            },
        })
    }
}
impl ::treesitter_types::Spanned for Tuple<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        self.span
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct UnaryOperator<'tree> {
    pub span: ::treesitter_types::Span,
    pub operand: ::std::vec::Vec<UnaryOperatorOperand<'tree>>,
    pub operator: UnaryOperatorOperator,
}
impl<'tree> ::treesitter_types::FromNode<'tree> for UnaryOperator<'tree> {
    #[allow(clippy::match_single_binding, clippy::suspicious_else_formatting)]
    fn from_node(
        node: ::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        debug_assert_eq!(node.kind(), "unary_operator");
        Ok(Self {
            span: ::treesitter_types::Span::from(node),
            operand: {
                let mut cursor = node.walk();
                let mut items = ::std::vec::Vec::new();
                for child in node.children_by_field_name("operand", &mut cursor) {
                    items.push(
                        <UnaryOperatorOperand as ::treesitter_types::FromNode>::from_node(
                            child, src,
                        )?,
                    );
                }
                items
            },
            operator: {
                let child = node.child_by_field_name("operator").ok_or_else(|| {
                    ::treesitter_types::ParseError::missing_field("operator", node)
                })?;
                <UnaryOperatorOperator as ::treesitter_types::FromNode>::from_node(child, src)?
            },
        })
    }
}
impl ::treesitter_types::Spanned for UnaryOperator<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        self.span
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Alias<'tree> {
    pub span: ::treesitter_types::Span,
    text: &'tree str,
}
impl<'tree> ::treesitter_types::FromNode<'tree> for Alias<'tree> {
    fn from_node(
        node: ::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        debug_assert_eq!(node.kind(), "alias");
        Ok(Self {
            span: ::treesitter_types::Span::from(node),
            text: node.utf8_text(src)?,
        })
    }
}
impl<'tree> ::treesitter_types::LeafNode<'tree> for Alias<'tree> {
    fn text(&self) -> &'tree str {
        self.text
    }
}
impl ::treesitter_types::Spanned for Alias<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        self.span
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Atom<'tree> {
    pub span: ::treesitter_types::Span,
    text: &'tree str,
}
impl<'tree> ::treesitter_types::FromNode<'tree> for Atom<'tree> {
    fn from_node(
        node: ::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        debug_assert_eq!(node.kind(), "atom");
        Ok(Self {
            span: ::treesitter_types::Span::from(node),
            text: node.utf8_text(src)?,
        })
    }
}
impl<'tree> ::treesitter_types::LeafNode<'tree> for Atom<'tree> {
    fn text(&self) -> &'tree str {
        self.text
    }
}
impl ::treesitter_types::Spanned for Atom<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        self.span
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Char<'tree> {
    pub span: ::treesitter_types::Span,
    text: &'tree str,
}
impl<'tree> ::treesitter_types::FromNode<'tree> for Char<'tree> {
    fn from_node(
        node: ::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        debug_assert_eq!(node.kind(), "char");
        Ok(Self {
            span: ::treesitter_types::Span::from(node),
            text: node.utf8_text(src)?,
        })
    }
}
impl<'tree> ::treesitter_types::LeafNode<'tree> for Char<'tree> {
    fn text(&self) -> &'tree str {
        self.text
    }
}
impl ::treesitter_types::Spanned for Char<'_> {
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
pub struct Float<'tree> {
    pub span: ::treesitter_types::Span,
    text: &'tree str,
}
impl<'tree> ::treesitter_types::FromNode<'tree> for Float<'tree> {
    fn from_node(
        node: ::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        debug_assert_eq!(node.kind(), "float");
        Ok(Self {
            span: ::treesitter_types::Span::from(node),
            text: node.utf8_text(src)?,
        })
    }
}
impl<'tree> ::treesitter_types::LeafNode<'tree> for Float<'tree> {
    fn text(&self) -> &'tree str {
        self.text
    }
}
impl ::treesitter_types::Spanned for Float<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        self.span
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Integer<'tree> {
    pub span: ::treesitter_types::Span,
    text: &'tree str,
}
impl<'tree> ::treesitter_types::FromNode<'tree> for Integer<'tree> {
    fn from_node(
        node: ::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        debug_assert_eq!(node.kind(), "integer");
        Ok(Self {
            span: ::treesitter_types::Span::from(node),
            text: node.utf8_text(src)?,
        })
    }
}
impl<'tree> ::treesitter_types::LeafNode<'tree> for Integer<'tree> {
    fn text(&self) -> &'tree str {
        self.text
    }
}
impl ::treesitter_types::Spanned for Integer<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        self.span
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Keyword<'tree> {
    pub span: ::treesitter_types::Span,
    text: &'tree str,
}
impl<'tree> ::treesitter_types::FromNode<'tree> for Keyword<'tree> {
    fn from_node(
        node: ::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        debug_assert_eq!(node.kind(), "keyword");
        Ok(Self {
            span: ::treesitter_types::Span::from(node),
            text: node.utf8_text(src)?,
        })
    }
}
impl<'tree> ::treesitter_types::LeafNode<'tree> for Keyword<'tree> {
    fn text(&self) -> &'tree str {
        self.text
    }
}
impl ::treesitter_types::Spanned for Keyword<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        self.span
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct QuotedContent<'tree> {
    pub span: ::treesitter_types::Span,
    text: &'tree str,
}
impl<'tree> ::treesitter_types::FromNode<'tree> for QuotedContent<'tree> {
    fn from_node(
        node: ::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        debug_assert_eq!(node.kind(), "quoted_content");
        Ok(Self {
            span: ::treesitter_types::Span::from(node),
            text: node.utf8_text(src)?,
        })
    }
}
impl<'tree> ::treesitter_types::LeafNode<'tree> for QuotedContent<'tree> {
    fn text(&self) -> &'tree str {
        self.text
    }
}
impl ::treesitter_types::Spanned for QuotedContent<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        self.span
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SigilModifiers<'tree> {
    pub span: ::treesitter_types::Span,
    text: &'tree str,
}
impl<'tree> ::treesitter_types::FromNode<'tree> for SigilModifiers<'tree> {
    fn from_node(
        node: ::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        debug_assert_eq!(node.kind(), "sigil_modifiers");
        Ok(Self {
            span: ::treesitter_types::Span::from(node),
            text: node.utf8_text(src)?,
        })
    }
}
impl<'tree> ::treesitter_types::LeafNode<'tree> for SigilModifiers<'tree> {
    fn text(&self) -> &'tree str {
        self.text
    }
}
impl ::treesitter_types::Spanned for SigilModifiers<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        self.span
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SigilName<'tree> {
    pub span: ::treesitter_types::Span,
    text: &'tree str,
}
impl<'tree> ::treesitter_types::FromNode<'tree> for SigilName<'tree> {
    fn from_node(
        node: ::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        debug_assert_eq!(node.kind(), "sigil_name");
        Ok(Self {
            span: ::treesitter_types::Span::from(node),
            text: node.utf8_text(src)?,
        })
    }
}
impl<'tree> ::treesitter_types::LeafNode<'tree> for SigilName<'tree> {
    fn text(&self) -> &'tree str {
        self.text
    }
}
impl ::treesitter_types::Spanned for SigilName<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        self.span
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum AccessCallKey<'tree> {
    AccessCall(::std::boxed::Box<AccessCall<'tree>>),
    Alias(::std::boxed::Box<Alias<'tree>>),
    AnonymousFunction(::std::boxed::Box<AnonymousFunction<'tree>>),
    Atom(::std::boxed::Box<Atom<'tree>>),
    BinaryOperator(::std::boxed::Box<BinaryOperator<'tree>>),
    Bitstring(::std::boxed::Box<Bitstring<'tree>>),
    Block(::std::boxed::Box<Block<'tree>>),
    Boolean(::std::boxed::Box<Boolean<'tree>>),
    Call(::std::boxed::Box<Call<'tree>>),
    Char(::std::boxed::Box<Char<'tree>>),
    Charlist(::std::boxed::Box<Charlist<'tree>>),
    Dot(::std::boxed::Box<Dot<'tree>>),
    Float(::std::boxed::Box<Float<'tree>>),
    Identifier(::std::boxed::Box<Identifier<'tree>>),
    Integer(::std::boxed::Box<Integer<'tree>>),
    List(::std::boxed::Box<List<'tree>>),
    Map(::std::boxed::Box<Map<'tree>>),
    Nil(::std::boxed::Box<Nil<'tree>>),
    OperatorIdentifier(::std::boxed::Box<OperatorIdentifier<'tree>>),
    QuotedAtom(::std::boxed::Box<QuotedAtom<'tree>>),
    Sigil(::std::boxed::Box<Sigil<'tree>>),
    String(::std::boxed::Box<String<'tree>>),
    Tuple(::std::boxed::Box<Tuple<'tree>>),
    UnaryOperator(::std::boxed::Box<UnaryOperator<'tree>>),
}
impl<'tree> ::treesitter_types::FromNode<'tree> for AccessCallKey<'tree> {
    #[allow(clippy::collapsible_else_if)]
    fn from_node(
        node: ::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        match node.kind() {
            "access_call" => Ok(Self::AccessCall(::std::boxed::Box::new(
                <AccessCall as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "alias" => Ok(Self::Alias(::std::boxed::Box::new(
                <Alias as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "anonymous_function" => Ok(Self::AnonymousFunction(::std::boxed::Box::new(
                <AnonymousFunction as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "atom" => Ok(Self::Atom(::std::boxed::Box::new(
                <Atom as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "binary_operator" => Ok(Self::BinaryOperator(::std::boxed::Box::new(
                <BinaryOperator as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "bitstring" => Ok(Self::Bitstring(::std::boxed::Box::new(
                <Bitstring as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "block" => Ok(Self::Block(::std::boxed::Box::new(
                <Block as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "boolean" => Ok(Self::Boolean(::std::boxed::Box::new(
                <Boolean as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "call" => Ok(Self::Call(::std::boxed::Box::new(
                <Call as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "char" => Ok(Self::Char(::std::boxed::Box::new(
                <Char as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "charlist" => Ok(Self::Charlist(::std::boxed::Box::new(
                <Charlist as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "dot" => Ok(Self::Dot(::std::boxed::Box::new(
                <Dot as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "float" => Ok(Self::Float(::std::boxed::Box::new(
                <Float as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "identifier" => Ok(Self::Identifier(::std::boxed::Box::new(
                <Identifier as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "integer" => Ok(Self::Integer(::std::boxed::Box::new(
                <Integer as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "list" => Ok(Self::List(::std::boxed::Box::new(
                <List as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "map" => Ok(Self::Map(::std::boxed::Box::new(
                <Map as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "nil" => Ok(Self::Nil(::std::boxed::Box::new(
                <Nil as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "operator_identifier" => Ok(Self::OperatorIdentifier(::std::boxed::Box::new(
                <OperatorIdentifier as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "quoted_atom" => Ok(Self::QuotedAtom(::std::boxed::Box::new(
                <QuotedAtom as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "sigil" => Ok(Self::Sigil(::std::boxed::Box::new(
                <Sigil as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "string" => Ok(Self::String(::std::boxed::Box::new(
                <String as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "tuple" => Ok(Self::Tuple(::std::boxed::Box::new(
                <Tuple as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "unary_operator" => Ok(Self::UnaryOperator(::std::boxed::Box::new(
                <UnaryOperator as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            other => Err(::treesitter_types::ParseError::unexpected_kind(other, node)),
        }
    }
}
impl ::treesitter_types::Spanned for AccessCallKey<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        match self {
            Self::AccessCall(inner) => inner.span(),
            Self::Alias(inner) => inner.span(),
            Self::AnonymousFunction(inner) => inner.span(),
            Self::Atom(inner) => inner.span(),
            Self::BinaryOperator(inner) => inner.span(),
            Self::Bitstring(inner) => inner.span(),
            Self::Block(inner) => inner.span(),
            Self::Boolean(inner) => inner.span(),
            Self::Call(inner) => inner.span(),
            Self::Char(inner) => inner.span(),
            Self::Charlist(inner) => inner.span(),
            Self::Dot(inner) => inner.span(),
            Self::Float(inner) => inner.span(),
            Self::Identifier(inner) => inner.span(),
            Self::Integer(inner) => inner.span(),
            Self::List(inner) => inner.span(),
            Self::Map(inner) => inner.span(),
            Self::Nil(inner) => inner.span(),
            Self::OperatorIdentifier(inner) => inner.span(),
            Self::QuotedAtom(inner) => inner.span(),
            Self::Sigil(inner) => inner.span(),
            Self::String(inner) => inner.span(),
            Self::Tuple(inner) => inner.span(),
            Self::UnaryOperator(inner) => inner.span(),
        }
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum AccessCallTarget<'tree> {
    AccessCall(::std::boxed::Box<AccessCall<'tree>>),
    Alias(::std::boxed::Box<Alias<'tree>>),
    AnonymousFunction(::std::boxed::Box<AnonymousFunction<'tree>>),
    Atom(::std::boxed::Box<Atom<'tree>>),
    BinaryOperator(::std::boxed::Box<BinaryOperator<'tree>>),
    Bitstring(::std::boxed::Box<Bitstring<'tree>>),
    Block(::std::boxed::Box<Block<'tree>>),
    Boolean(::std::boxed::Box<Boolean<'tree>>),
    Call(::std::boxed::Box<Call<'tree>>),
    Char(::std::boxed::Box<Char<'tree>>),
    Charlist(::std::boxed::Box<Charlist<'tree>>),
    Dot(::std::boxed::Box<Dot<'tree>>),
    Float(::std::boxed::Box<Float<'tree>>),
    Identifier(::std::boxed::Box<Identifier<'tree>>),
    Integer(::std::boxed::Box<Integer<'tree>>),
    List(::std::boxed::Box<List<'tree>>),
    Map(::std::boxed::Box<Map<'tree>>),
    Nil(::std::boxed::Box<Nil<'tree>>),
    OperatorIdentifier(::std::boxed::Box<OperatorIdentifier<'tree>>),
    QuotedAtom(::std::boxed::Box<QuotedAtom<'tree>>),
    Sigil(::std::boxed::Box<Sigil<'tree>>),
    String(::std::boxed::Box<String<'tree>>),
    Tuple(::std::boxed::Box<Tuple<'tree>>),
    UnaryOperator(::std::boxed::Box<UnaryOperator<'tree>>),
}
impl<'tree> ::treesitter_types::FromNode<'tree> for AccessCallTarget<'tree> {
    #[allow(clippy::collapsible_else_if)]
    fn from_node(
        node: ::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        match node.kind() {
            "access_call" => Ok(Self::AccessCall(::std::boxed::Box::new(
                <AccessCall as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "alias" => Ok(Self::Alias(::std::boxed::Box::new(
                <Alias as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "anonymous_function" => Ok(Self::AnonymousFunction(::std::boxed::Box::new(
                <AnonymousFunction as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "atom" => Ok(Self::Atom(::std::boxed::Box::new(
                <Atom as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "binary_operator" => Ok(Self::BinaryOperator(::std::boxed::Box::new(
                <BinaryOperator as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "bitstring" => Ok(Self::Bitstring(::std::boxed::Box::new(
                <Bitstring as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "block" => Ok(Self::Block(::std::boxed::Box::new(
                <Block as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "boolean" => Ok(Self::Boolean(::std::boxed::Box::new(
                <Boolean as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "call" => Ok(Self::Call(::std::boxed::Box::new(
                <Call as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "char" => Ok(Self::Char(::std::boxed::Box::new(
                <Char as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "charlist" => Ok(Self::Charlist(::std::boxed::Box::new(
                <Charlist as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "dot" => Ok(Self::Dot(::std::boxed::Box::new(
                <Dot as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "float" => Ok(Self::Float(::std::boxed::Box::new(
                <Float as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "identifier" => Ok(Self::Identifier(::std::boxed::Box::new(
                <Identifier as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "integer" => Ok(Self::Integer(::std::boxed::Box::new(
                <Integer as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "list" => Ok(Self::List(::std::boxed::Box::new(
                <List as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "map" => Ok(Self::Map(::std::boxed::Box::new(
                <Map as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "nil" => Ok(Self::Nil(::std::boxed::Box::new(
                <Nil as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "operator_identifier" => Ok(Self::OperatorIdentifier(::std::boxed::Box::new(
                <OperatorIdentifier as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "quoted_atom" => Ok(Self::QuotedAtom(::std::boxed::Box::new(
                <QuotedAtom as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "sigil" => Ok(Self::Sigil(::std::boxed::Box::new(
                <Sigil as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "string" => Ok(Self::String(::std::boxed::Box::new(
                <String as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "tuple" => Ok(Self::Tuple(::std::boxed::Box::new(
                <Tuple as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "unary_operator" => Ok(Self::UnaryOperator(::std::boxed::Box::new(
                <UnaryOperator as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            other => Err(::treesitter_types::ParseError::unexpected_kind(other, node)),
        }
    }
}
impl ::treesitter_types::Spanned for AccessCallTarget<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        match self {
            Self::AccessCall(inner) => inner.span(),
            Self::Alias(inner) => inner.span(),
            Self::AnonymousFunction(inner) => inner.span(),
            Self::Atom(inner) => inner.span(),
            Self::BinaryOperator(inner) => inner.span(),
            Self::Bitstring(inner) => inner.span(),
            Self::Block(inner) => inner.span(),
            Self::Boolean(inner) => inner.span(),
            Self::Call(inner) => inner.span(),
            Self::Char(inner) => inner.span(),
            Self::Charlist(inner) => inner.span(),
            Self::Dot(inner) => inner.span(),
            Self::Float(inner) => inner.span(),
            Self::Identifier(inner) => inner.span(),
            Self::Integer(inner) => inner.span(),
            Self::List(inner) => inner.span(),
            Self::Map(inner) => inner.span(),
            Self::Nil(inner) => inner.span(),
            Self::OperatorIdentifier(inner) => inner.span(),
            Self::QuotedAtom(inner) => inner.span(),
            Self::Sigil(inner) => inner.span(),
            Self::String(inner) => inner.span(),
            Self::Tuple(inner) => inner.span(),
            Self::UnaryOperator(inner) => inner.span(),
        }
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum AfterBlockChildren<'tree> {
    AccessCall(::std::boxed::Box<AccessCall<'tree>>),
    Alias(::std::boxed::Box<Alias<'tree>>),
    AnonymousFunction(::std::boxed::Box<AnonymousFunction<'tree>>),
    Atom(::std::boxed::Box<Atom<'tree>>),
    BinaryOperator(::std::boxed::Box<BinaryOperator<'tree>>),
    Bitstring(::std::boxed::Box<Bitstring<'tree>>),
    Block(::std::boxed::Box<Block<'tree>>),
    Boolean(::std::boxed::Box<Boolean<'tree>>),
    Call(::std::boxed::Box<Call<'tree>>),
    Char(::std::boxed::Box<Char<'tree>>),
    Charlist(::std::boxed::Box<Charlist<'tree>>),
    Dot(::std::boxed::Box<Dot<'tree>>),
    Float(::std::boxed::Box<Float<'tree>>),
    Identifier(::std::boxed::Box<Identifier<'tree>>),
    Integer(::std::boxed::Box<Integer<'tree>>),
    List(::std::boxed::Box<List<'tree>>),
    Map(::std::boxed::Box<Map<'tree>>),
    Nil(::std::boxed::Box<Nil<'tree>>),
    OperatorIdentifier(::std::boxed::Box<OperatorIdentifier<'tree>>),
    QuotedAtom(::std::boxed::Box<QuotedAtom<'tree>>),
    Sigil(::std::boxed::Box<Sigil<'tree>>),
    StabClause(::std::boxed::Box<StabClause<'tree>>),
    String(::std::boxed::Box<String<'tree>>),
    Tuple(::std::boxed::Box<Tuple<'tree>>),
    UnaryOperator(::std::boxed::Box<UnaryOperator<'tree>>),
}
impl<'tree> ::treesitter_types::FromNode<'tree> for AfterBlockChildren<'tree> {
    #[allow(clippy::collapsible_else_if)]
    fn from_node(
        node: ::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        match node.kind() {
            "access_call" => Ok(Self::AccessCall(::std::boxed::Box::new(
                <AccessCall as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "alias" => Ok(Self::Alias(::std::boxed::Box::new(
                <Alias as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "anonymous_function" => Ok(Self::AnonymousFunction(::std::boxed::Box::new(
                <AnonymousFunction as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "atom" => Ok(Self::Atom(::std::boxed::Box::new(
                <Atom as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "binary_operator" => Ok(Self::BinaryOperator(::std::boxed::Box::new(
                <BinaryOperator as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "bitstring" => Ok(Self::Bitstring(::std::boxed::Box::new(
                <Bitstring as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "block" => Ok(Self::Block(::std::boxed::Box::new(
                <Block as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "boolean" => Ok(Self::Boolean(::std::boxed::Box::new(
                <Boolean as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "call" => Ok(Self::Call(::std::boxed::Box::new(
                <Call as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "char" => Ok(Self::Char(::std::boxed::Box::new(
                <Char as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "charlist" => Ok(Self::Charlist(::std::boxed::Box::new(
                <Charlist as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "dot" => Ok(Self::Dot(::std::boxed::Box::new(
                <Dot as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "float" => Ok(Self::Float(::std::boxed::Box::new(
                <Float as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "identifier" => Ok(Self::Identifier(::std::boxed::Box::new(
                <Identifier as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "integer" => Ok(Self::Integer(::std::boxed::Box::new(
                <Integer as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "list" => Ok(Self::List(::std::boxed::Box::new(
                <List as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "map" => Ok(Self::Map(::std::boxed::Box::new(
                <Map as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "nil" => Ok(Self::Nil(::std::boxed::Box::new(
                <Nil as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "operator_identifier" => Ok(Self::OperatorIdentifier(::std::boxed::Box::new(
                <OperatorIdentifier as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "quoted_atom" => Ok(Self::QuotedAtom(::std::boxed::Box::new(
                <QuotedAtom as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "sigil" => Ok(Self::Sigil(::std::boxed::Box::new(
                <Sigil as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "stab_clause" => Ok(Self::StabClause(::std::boxed::Box::new(
                <StabClause as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "string" => Ok(Self::String(::std::boxed::Box::new(
                <String as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "tuple" => Ok(Self::Tuple(::std::boxed::Box::new(
                <Tuple as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "unary_operator" => Ok(Self::UnaryOperator(::std::boxed::Box::new(
                <UnaryOperator as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            other => Err(::treesitter_types::ParseError::unexpected_kind(other, node)),
        }
    }
}
impl ::treesitter_types::Spanned for AfterBlockChildren<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        match self {
            Self::AccessCall(inner) => inner.span(),
            Self::Alias(inner) => inner.span(),
            Self::AnonymousFunction(inner) => inner.span(),
            Self::Atom(inner) => inner.span(),
            Self::BinaryOperator(inner) => inner.span(),
            Self::Bitstring(inner) => inner.span(),
            Self::Block(inner) => inner.span(),
            Self::Boolean(inner) => inner.span(),
            Self::Call(inner) => inner.span(),
            Self::Char(inner) => inner.span(),
            Self::Charlist(inner) => inner.span(),
            Self::Dot(inner) => inner.span(),
            Self::Float(inner) => inner.span(),
            Self::Identifier(inner) => inner.span(),
            Self::Integer(inner) => inner.span(),
            Self::List(inner) => inner.span(),
            Self::Map(inner) => inner.span(),
            Self::Nil(inner) => inner.span(),
            Self::OperatorIdentifier(inner) => inner.span(),
            Self::QuotedAtom(inner) => inner.span(),
            Self::Sigil(inner) => inner.span(),
            Self::StabClause(inner) => inner.span(),
            Self::String(inner) => inner.span(),
            Self::Tuple(inner) => inner.span(),
            Self::UnaryOperator(inner) => inner.span(),
        }
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ArgumentsChildren<'tree> {
    AccessCall(::std::boxed::Box<AccessCall<'tree>>),
    Alias(::std::boxed::Box<Alias<'tree>>),
    AnonymousFunction(::std::boxed::Box<AnonymousFunction<'tree>>),
    Atom(::std::boxed::Box<Atom<'tree>>),
    BinaryOperator(::std::boxed::Box<BinaryOperator<'tree>>),
    Bitstring(::std::boxed::Box<Bitstring<'tree>>),
    Block(::std::boxed::Box<Block<'tree>>),
    Boolean(::std::boxed::Box<Boolean<'tree>>),
    Call(::std::boxed::Box<Call<'tree>>),
    Char(::std::boxed::Box<Char<'tree>>),
    Charlist(::std::boxed::Box<Charlist<'tree>>),
    Dot(::std::boxed::Box<Dot<'tree>>),
    Float(::std::boxed::Box<Float<'tree>>),
    Identifier(::std::boxed::Box<Identifier<'tree>>),
    Integer(::std::boxed::Box<Integer<'tree>>),
    Keywords(::std::boxed::Box<Keywords<'tree>>),
    List(::std::boxed::Box<List<'tree>>),
    Map(::std::boxed::Box<Map<'tree>>),
    Nil(::std::boxed::Box<Nil<'tree>>),
    OperatorIdentifier(::std::boxed::Box<OperatorIdentifier<'tree>>),
    QuotedAtom(::std::boxed::Box<QuotedAtom<'tree>>),
    Sigil(::std::boxed::Box<Sigil<'tree>>),
    String(::std::boxed::Box<String<'tree>>),
    Tuple(::std::boxed::Box<Tuple<'tree>>),
    UnaryOperator(::std::boxed::Box<UnaryOperator<'tree>>),
}
impl<'tree> ::treesitter_types::FromNode<'tree> for ArgumentsChildren<'tree> {
    #[allow(clippy::collapsible_else_if)]
    fn from_node(
        node: ::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        match node.kind() {
            "access_call" => Ok(Self::AccessCall(::std::boxed::Box::new(
                <AccessCall as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "alias" => Ok(Self::Alias(::std::boxed::Box::new(
                <Alias as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "anonymous_function" => Ok(Self::AnonymousFunction(::std::boxed::Box::new(
                <AnonymousFunction as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "atom" => Ok(Self::Atom(::std::boxed::Box::new(
                <Atom as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "binary_operator" => Ok(Self::BinaryOperator(::std::boxed::Box::new(
                <BinaryOperator as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "bitstring" => Ok(Self::Bitstring(::std::boxed::Box::new(
                <Bitstring as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "block" => Ok(Self::Block(::std::boxed::Box::new(
                <Block as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "boolean" => Ok(Self::Boolean(::std::boxed::Box::new(
                <Boolean as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "call" => Ok(Self::Call(::std::boxed::Box::new(
                <Call as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "char" => Ok(Self::Char(::std::boxed::Box::new(
                <Char as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "charlist" => Ok(Self::Charlist(::std::boxed::Box::new(
                <Charlist as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "dot" => Ok(Self::Dot(::std::boxed::Box::new(
                <Dot as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "float" => Ok(Self::Float(::std::boxed::Box::new(
                <Float as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "identifier" => Ok(Self::Identifier(::std::boxed::Box::new(
                <Identifier as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "integer" => Ok(Self::Integer(::std::boxed::Box::new(
                <Integer as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "keywords" => Ok(Self::Keywords(::std::boxed::Box::new(
                <Keywords as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "list" => Ok(Self::List(::std::boxed::Box::new(
                <List as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "map" => Ok(Self::Map(::std::boxed::Box::new(
                <Map as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "nil" => Ok(Self::Nil(::std::boxed::Box::new(
                <Nil as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "operator_identifier" => Ok(Self::OperatorIdentifier(::std::boxed::Box::new(
                <OperatorIdentifier as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "quoted_atom" => Ok(Self::QuotedAtom(::std::boxed::Box::new(
                <QuotedAtom as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "sigil" => Ok(Self::Sigil(::std::boxed::Box::new(
                <Sigil as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "string" => Ok(Self::String(::std::boxed::Box::new(
                <String as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "tuple" => Ok(Self::Tuple(::std::boxed::Box::new(
                <Tuple as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "unary_operator" => Ok(Self::UnaryOperator(::std::boxed::Box::new(
                <UnaryOperator as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            other => Err(::treesitter_types::ParseError::unexpected_kind(other, node)),
        }
    }
}
impl ::treesitter_types::Spanned for ArgumentsChildren<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        match self {
            Self::AccessCall(inner) => inner.span(),
            Self::Alias(inner) => inner.span(),
            Self::AnonymousFunction(inner) => inner.span(),
            Self::Atom(inner) => inner.span(),
            Self::BinaryOperator(inner) => inner.span(),
            Self::Bitstring(inner) => inner.span(),
            Self::Block(inner) => inner.span(),
            Self::Boolean(inner) => inner.span(),
            Self::Call(inner) => inner.span(),
            Self::Char(inner) => inner.span(),
            Self::Charlist(inner) => inner.span(),
            Self::Dot(inner) => inner.span(),
            Self::Float(inner) => inner.span(),
            Self::Identifier(inner) => inner.span(),
            Self::Integer(inner) => inner.span(),
            Self::Keywords(inner) => inner.span(),
            Self::List(inner) => inner.span(),
            Self::Map(inner) => inner.span(),
            Self::Nil(inner) => inner.span(),
            Self::OperatorIdentifier(inner) => inner.span(),
            Self::QuotedAtom(inner) => inner.span(),
            Self::Sigil(inner) => inner.span(),
            Self::String(inner) => inner.span(),
            Self::Tuple(inner) => inner.span(),
            Self::UnaryOperator(inner) => inner.span(),
        }
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum BinaryOperatorLeft<'tree> {
    AccessCall(::std::boxed::Box<AccessCall<'tree>>),
    Alias(::std::boxed::Box<Alias<'tree>>),
    AnonymousFunction(::std::boxed::Box<AnonymousFunction<'tree>>),
    Arguments(::std::boxed::Box<Arguments<'tree>>),
    Atom(::std::boxed::Box<Atom<'tree>>),
    BinaryOperator(::std::boxed::Box<BinaryOperator<'tree>>),
    Bitstring(::std::boxed::Box<Bitstring<'tree>>),
    Block(::std::boxed::Box<Block<'tree>>),
    Boolean(::std::boxed::Box<Boolean<'tree>>),
    Call(::std::boxed::Box<Call<'tree>>),
    Char(::std::boxed::Box<Char<'tree>>),
    Charlist(::std::boxed::Box<Charlist<'tree>>),
    Dot(::std::boxed::Box<Dot<'tree>>),
    Float(::std::boxed::Box<Float<'tree>>),
    Identifier(::std::boxed::Box<Identifier<'tree>>),
    Integer(::std::boxed::Box<Integer<'tree>>),
    List(::std::boxed::Box<List<'tree>>),
    Map(::std::boxed::Box<Map<'tree>>),
    Nil(::std::boxed::Box<Nil<'tree>>),
    OperatorIdentifier(::std::boxed::Box<OperatorIdentifier<'tree>>),
    QuotedAtom(::std::boxed::Box<QuotedAtom<'tree>>),
    Sigil(::std::boxed::Box<Sigil<'tree>>),
    String(::std::boxed::Box<String<'tree>>),
    Tuple(::std::boxed::Box<Tuple<'tree>>),
    UnaryOperator(::std::boxed::Box<UnaryOperator<'tree>>),
}
impl<'tree> ::treesitter_types::FromNode<'tree> for BinaryOperatorLeft<'tree> {
    #[allow(clippy::collapsible_else_if)]
    fn from_node(
        node: ::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        match node.kind() {
            "access_call" => Ok(Self::AccessCall(::std::boxed::Box::new(
                <AccessCall as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "alias" => Ok(Self::Alias(::std::boxed::Box::new(
                <Alias as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "anonymous_function" => Ok(Self::AnonymousFunction(::std::boxed::Box::new(
                <AnonymousFunction as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "arguments" => Ok(Self::Arguments(::std::boxed::Box::new(
                <Arguments as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "atom" => Ok(Self::Atom(::std::boxed::Box::new(
                <Atom as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "binary_operator" => Ok(Self::BinaryOperator(::std::boxed::Box::new(
                <BinaryOperator as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "bitstring" => Ok(Self::Bitstring(::std::boxed::Box::new(
                <Bitstring as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "block" => Ok(Self::Block(::std::boxed::Box::new(
                <Block as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "boolean" => Ok(Self::Boolean(::std::boxed::Box::new(
                <Boolean as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "call" => Ok(Self::Call(::std::boxed::Box::new(
                <Call as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "char" => Ok(Self::Char(::std::boxed::Box::new(
                <Char as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "charlist" => Ok(Self::Charlist(::std::boxed::Box::new(
                <Charlist as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "dot" => Ok(Self::Dot(::std::boxed::Box::new(
                <Dot as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "float" => Ok(Self::Float(::std::boxed::Box::new(
                <Float as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "identifier" => Ok(Self::Identifier(::std::boxed::Box::new(
                <Identifier as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "integer" => Ok(Self::Integer(::std::boxed::Box::new(
                <Integer as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "list" => Ok(Self::List(::std::boxed::Box::new(
                <List as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "map" => Ok(Self::Map(::std::boxed::Box::new(
                <Map as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "nil" => Ok(Self::Nil(::std::boxed::Box::new(
                <Nil as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "operator_identifier" => Ok(Self::OperatorIdentifier(::std::boxed::Box::new(
                <OperatorIdentifier as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "quoted_atom" => Ok(Self::QuotedAtom(::std::boxed::Box::new(
                <QuotedAtom as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "sigil" => Ok(Self::Sigil(::std::boxed::Box::new(
                <Sigil as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "string" => Ok(Self::String(::std::boxed::Box::new(
                <String as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "tuple" => Ok(Self::Tuple(::std::boxed::Box::new(
                <Tuple as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "unary_operator" => Ok(Self::UnaryOperator(::std::boxed::Box::new(
                <UnaryOperator as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            other => Err(::treesitter_types::ParseError::unexpected_kind(other, node)),
        }
    }
}
impl ::treesitter_types::Spanned for BinaryOperatorLeft<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        match self {
            Self::AccessCall(inner) => inner.span(),
            Self::Alias(inner) => inner.span(),
            Self::AnonymousFunction(inner) => inner.span(),
            Self::Arguments(inner) => inner.span(),
            Self::Atom(inner) => inner.span(),
            Self::BinaryOperator(inner) => inner.span(),
            Self::Bitstring(inner) => inner.span(),
            Self::Block(inner) => inner.span(),
            Self::Boolean(inner) => inner.span(),
            Self::Call(inner) => inner.span(),
            Self::Char(inner) => inner.span(),
            Self::Charlist(inner) => inner.span(),
            Self::Dot(inner) => inner.span(),
            Self::Float(inner) => inner.span(),
            Self::Identifier(inner) => inner.span(),
            Self::Integer(inner) => inner.span(),
            Self::List(inner) => inner.span(),
            Self::Map(inner) => inner.span(),
            Self::Nil(inner) => inner.span(),
            Self::OperatorIdentifier(inner) => inner.span(),
            Self::QuotedAtom(inner) => inner.span(),
            Self::Sigil(inner) => inner.span(),
            Self::String(inner) => inner.span(),
            Self::Tuple(inner) => inner.span(),
            Self::UnaryOperator(inner) => inner.span(),
        }
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum BinaryOperatorOperator {
    NotEq(::treesitter_types::Span),
    BangEqEq(::treesitter_types::Span),
    AmpAmp(::treesitter_types::Span),
    AmpAmpAmp(::treesitter_types::Span),
    Star(::treesitter_types::Span),
    StarStar(::treesitter_types::Span),
    Plus(::treesitter_types::Span),
    PlusPlus(::treesitter_types::Span),
    PlusPlusPlus(::treesitter_types::Span),
    Minus(::treesitter_types::Span),
    MinusMinus(::treesitter_types::Span),
    MinusMinusMinus(::treesitter_types::Span),
    DotDot(::treesitter_types::Span),
    Slash(::treesitter_types::Span),
    SlashSlash(::treesitter_types::Span),
    DoubleColon(::treesitter_types::Span),
    Lt(::treesitter_types::Span),
    LArrow(::treesitter_types::Span),
    LtLtLt(::treesitter_types::Span),
    LtLtTilde(::treesitter_types::Span),
    LtEq(::treesitter_types::Span),
    LtGt(::treesitter_types::Span),
    LtPipeGt(::treesitter_types::Span),
    LtTilde(::treesitter_types::Span),
    LtTildeGt(::treesitter_types::Span),
    Eq(::treesitter_types::Span),
    EqEq(::treesitter_types::Span),
    EqEqEq(::treesitter_types::Span),
    FatArrow(::treesitter_types::Span),
    EqTilde(::treesitter_types::Span),
    Gt(::treesitter_types::Span),
    GtEq(::treesitter_types::Span),
    GtGtGt(::treesitter_types::Span),
    BackslashBackslash(::treesitter_types::Span),
    CaretCaretCaret(::treesitter_types::Span),
    And(::treesitter_types::Span),
    In(::treesitter_types::Span),
    NOTXIN(::treesitter_types::Span),
    Or(::treesitter_types::Span),
    When(::treesitter_types::Span),
    Pipe(::treesitter_types::Span),
    PipeGt(::treesitter_types::Span),
    PipePipe(::treesitter_types::Span),
    PipePipePipe(::treesitter_types::Span),
    TildeGt(::treesitter_types::Span),
    TildeGtGt(::treesitter_types::Span),
}
impl<'tree> ::treesitter_types::FromNode<'tree> for BinaryOperatorOperator {
    #[allow(clippy::collapsible_else_if)]
    fn from_node(
        node: ::tree_sitter::Node<'tree>,
        _src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        match node.kind() {
            "!=" => Ok(Self::NotEq(::treesitter_types::Span::from(node))),
            "!==" => Ok(Self::BangEqEq(::treesitter_types::Span::from(node))),
            "&&" => Ok(Self::AmpAmp(::treesitter_types::Span::from(node))),
            "&&&" => Ok(Self::AmpAmpAmp(::treesitter_types::Span::from(node))),
            "*" => Ok(Self::Star(::treesitter_types::Span::from(node))),
            "**" => Ok(Self::StarStar(::treesitter_types::Span::from(node))),
            "+" => Ok(Self::Plus(::treesitter_types::Span::from(node))),
            "++" => Ok(Self::PlusPlus(::treesitter_types::Span::from(node))),
            "+++" => Ok(Self::PlusPlusPlus(::treesitter_types::Span::from(node))),
            "-" => Ok(Self::Minus(::treesitter_types::Span::from(node))),
            "--" => Ok(Self::MinusMinus(::treesitter_types::Span::from(node))),
            "---" => Ok(Self::MinusMinusMinus(::treesitter_types::Span::from(node))),
            ".." => Ok(Self::DotDot(::treesitter_types::Span::from(node))),
            "/" => Ok(Self::Slash(::treesitter_types::Span::from(node))),
            "//" => Ok(Self::SlashSlash(::treesitter_types::Span::from(node))),
            "::" => Ok(Self::DoubleColon(::treesitter_types::Span::from(node))),
            "<" => Ok(Self::Lt(::treesitter_types::Span::from(node))),
            "<-" => Ok(Self::LArrow(::treesitter_types::Span::from(node))),
            "<<<" => Ok(Self::LtLtLt(::treesitter_types::Span::from(node))),
            "<<~" => Ok(Self::LtLtTilde(::treesitter_types::Span::from(node))),
            "<=" => Ok(Self::LtEq(::treesitter_types::Span::from(node))),
            "<>" => Ok(Self::LtGt(::treesitter_types::Span::from(node))),
            "<|>" => Ok(Self::LtPipeGt(::treesitter_types::Span::from(node))),
            "<~" => Ok(Self::LtTilde(::treesitter_types::Span::from(node))),
            "<~>" => Ok(Self::LtTildeGt(::treesitter_types::Span::from(node))),
            "=" => Ok(Self::Eq(::treesitter_types::Span::from(node))),
            "==" => Ok(Self::EqEq(::treesitter_types::Span::from(node))),
            "===" => Ok(Self::EqEqEq(::treesitter_types::Span::from(node))),
            "=>" => Ok(Self::FatArrow(::treesitter_types::Span::from(node))),
            "=~" => Ok(Self::EqTilde(::treesitter_types::Span::from(node))),
            ">" => Ok(Self::Gt(::treesitter_types::Span::from(node))),
            ">=" => Ok(Self::GtEq(::treesitter_types::Span::from(node))),
            ">>>" => Ok(Self::GtGtGt(::treesitter_types::Span::from(node))),
            "\\\\" => Ok(Self::BackslashBackslash(::treesitter_types::Span::from(
                node,
            ))),
            "^^^" => Ok(Self::CaretCaretCaret(::treesitter_types::Span::from(node))),
            "and" => Ok(Self::And(::treesitter_types::Span::from(node))),
            "in" => Ok(Self::In(::treesitter_types::Span::from(node))),
            "not in" => Ok(Self::NOTXIN(::treesitter_types::Span::from(node))),
            "or" => Ok(Self::Or(::treesitter_types::Span::from(node))),
            "when" => Ok(Self::When(::treesitter_types::Span::from(node))),
            "|" => Ok(Self::Pipe(::treesitter_types::Span::from(node))),
            "|>" => Ok(Self::PipeGt(::treesitter_types::Span::from(node))),
            "||" => Ok(Self::PipePipe(::treesitter_types::Span::from(node))),
            "|||" => Ok(Self::PipePipePipe(::treesitter_types::Span::from(node))),
            "~>" => Ok(Self::TildeGt(::treesitter_types::Span::from(node))),
            "~>>" => Ok(Self::TildeGtGt(::treesitter_types::Span::from(node))),
            other => Err(::treesitter_types::ParseError::unexpected_kind(other, node)),
        }
    }
}
impl ::treesitter_types::Spanned for BinaryOperatorOperator {
    fn span(&self) -> ::treesitter_types::Span {
        match self {
            Self::NotEq(span) => *span,
            Self::BangEqEq(span) => *span,
            Self::AmpAmp(span) => *span,
            Self::AmpAmpAmp(span) => *span,
            Self::Star(span) => *span,
            Self::StarStar(span) => *span,
            Self::Plus(span) => *span,
            Self::PlusPlus(span) => *span,
            Self::PlusPlusPlus(span) => *span,
            Self::Minus(span) => *span,
            Self::MinusMinus(span) => *span,
            Self::MinusMinusMinus(span) => *span,
            Self::DotDot(span) => *span,
            Self::Slash(span) => *span,
            Self::SlashSlash(span) => *span,
            Self::DoubleColon(span) => *span,
            Self::Lt(span) => *span,
            Self::LArrow(span) => *span,
            Self::LtLtLt(span) => *span,
            Self::LtLtTilde(span) => *span,
            Self::LtEq(span) => *span,
            Self::LtGt(span) => *span,
            Self::LtPipeGt(span) => *span,
            Self::LtTilde(span) => *span,
            Self::LtTildeGt(span) => *span,
            Self::Eq(span) => *span,
            Self::EqEq(span) => *span,
            Self::EqEqEq(span) => *span,
            Self::FatArrow(span) => *span,
            Self::EqTilde(span) => *span,
            Self::Gt(span) => *span,
            Self::GtEq(span) => *span,
            Self::GtGtGt(span) => *span,
            Self::BackslashBackslash(span) => *span,
            Self::CaretCaretCaret(span) => *span,
            Self::And(span) => *span,
            Self::In(span) => *span,
            Self::NOTXIN(span) => *span,
            Self::Or(span) => *span,
            Self::When(span) => *span,
            Self::Pipe(span) => *span,
            Self::PipeGt(span) => *span,
            Self::PipePipe(span) => *span,
            Self::PipePipePipe(span) => *span,
            Self::TildeGt(span) => *span,
            Self::TildeGtGt(span) => *span,
        }
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum BinaryOperatorRight<'tree> {
    AccessCall(::std::boxed::Box<AccessCall<'tree>>),
    Alias(::std::boxed::Box<Alias<'tree>>),
    AnonymousFunction(::std::boxed::Box<AnonymousFunction<'tree>>),
    Atom(::std::boxed::Box<Atom<'tree>>),
    BinaryOperator(::std::boxed::Box<BinaryOperator<'tree>>),
    Bitstring(::std::boxed::Box<Bitstring<'tree>>),
    Block(::std::boxed::Box<Block<'tree>>),
    Boolean(::std::boxed::Box<Boolean<'tree>>),
    Call(::std::boxed::Box<Call<'tree>>),
    Char(::std::boxed::Box<Char<'tree>>),
    Charlist(::std::boxed::Box<Charlist<'tree>>),
    Dot(::std::boxed::Box<Dot<'tree>>),
    Float(::std::boxed::Box<Float<'tree>>),
    Identifier(::std::boxed::Box<Identifier<'tree>>),
    Integer(::std::boxed::Box<Integer<'tree>>),
    Keywords(::std::boxed::Box<Keywords<'tree>>),
    List(::std::boxed::Box<List<'tree>>),
    Map(::std::boxed::Box<Map<'tree>>),
    Nil(::std::boxed::Box<Nil<'tree>>),
    OperatorIdentifier(::std::boxed::Box<OperatorIdentifier<'tree>>),
    QuotedAtom(::std::boxed::Box<QuotedAtom<'tree>>),
    Sigil(::std::boxed::Box<Sigil<'tree>>),
    String(::std::boxed::Box<String<'tree>>),
    Tuple(::std::boxed::Box<Tuple<'tree>>),
    UnaryOperator(::std::boxed::Box<UnaryOperator<'tree>>),
}
impl<'tree> ::treesitter_types::FromNode<'tree> for BinaryOperatorRight<'tree> {
    #[allow(clippy::collapsible_else_if)]
    fn from_node(
        node: ::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        match node.kind() {
            "access_call" => Ok(Self::AccessCall(::std::boxed::Box::new(
                <AccessCall as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "alias" => Ok(Self::Alias(::std::boxed::Box::new(
                <Alias as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "anonymous_function" => Ok(Self::AnonymousFunction(::std::boxed::Box::new(
                <AnonymousFunction as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "atom" => Ok(Self::Atom(::std::boxed::Box::new(
                <Atom as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "binary_operator" => Ok(Self::BinaryOperator(::std::boxed::Box::new(
                <BinaryOperator as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "bitstring" => Ok(Self::Bitstring(::std::boxed::Box::new(
                <Bitstring as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "block" => Ok(Self::Block(::std::boxed::Box::new(
                <Block as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "boolean" => Ok(Self::Boolean(::std::boxed::Box::new(
                <Boolean as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "call" => Ok(Self::Call(::std::boxed::Box::new(
                <Call as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "char" => Ok(Self::Char(::std::boxed::Box::new(
                <Char as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "charlist" => Ok(Self::Charlist(::std::boxed::Box::new(
                <Charlist as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "dot" => Ok(Self::Dot(::std::boxed::Box::new(
                <Dot as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "float" => Ok(Self::Float(::std::boxed::Box::new(
                <Float as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "identifier" => Ok(Self::Identifier(::std::boxed::Box::new(
                <Identifier as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "integer" => Ok(Self::Integer(::std::boxed::Box::new(
                <Integer as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "keywords" => Ok(Self::Keywords(::std::boxed::Box::new(
                <Keywords as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "list" => Ok(Self::List(::std::boxed::Box::new(
                <List as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "map" => Ok(Self::Map(::std::boxed::Box::new(
                <Map as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "nil" => Ok(Self::Nil(::std::boxed::Box::new(
                <Nil as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "operator_identifier" => Ok(Self::OperatorIdentifier(::std::boxed::Box::new(
                <OperatorIdentifier as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "quoted_atom" => Ok(Self::QuotedAtom(::std::boxed::Box::new(
                <QuotedAtom as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "sigil" => Ok(Self::Sigil(::std::boxed::Box::new(
                <Sigil as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "string" => Ok(Self::String(::std::boxed::Box::new(
                <String as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "tuple" => Ok(Self::Tuple(::std::boxed::Box::new(
                <Tuple as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "unary_operator" => Ok(Self::UnaryOperator(::std::boxed::Box::new(
                <UnaryOperator as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            other => Err(::treesitter_types::ParseError::unexpected_kind(other, node)),
        }
    }
}
impl ::treesitter_types::Spanned for BinaryOperatorRight<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        match self {
            Self::AccessCall(inner) => inner.span(),
            Self::Alias(inner) => inner.span(),
            Self::AnonymousFunction(inner) => inner.span(),
            Self::Atom(inner) => inner.span(),
            Self::BinaryOperator(inner) => inner.span(),
            Self::Bitstring(inner) => inner.span(),
            Self::Block(inner) => inner.span(),
            Self::Boolean(inner) => inner.span(),
            Self::Call(inner) => inner.span(),
            Self::Char(inner) => inner.span(),
            Self::Charlist(inner) => inner.span(),
            Self::Dot(inner) => inner.span(),
            Self::Float(inner) => inner.span(),
            Self::Identifier(inner) => inner.span(),
            Self::Integer(inner) => inner.span(),
            Self::Keywords(inner) => inner.span(),
            Self::List(inner) => inner.span(),
            Self::Map(inner) => inner.span(),
            Self::Nil(inner) => inner.span(),
            Self::OperatorIdentifier(inner) => inner.span(),
            Self::QuotedAtom(inner) => inner.span(),
            Self::Sigil(inner) => inner.span(),
            Self::String(inner) => inner.span(),
            Self::Tuple(inner) => inner.span(),
            Self::UnaryOperator(inner) => inner.span(),
        }
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum BitstringChildren<'tree> {
    AccessCall(::std::boxed::Box<AccessCall<'tree>>),
    Alias(::std::boxed::Box<Alias<'tree>>),
    AnonymousFunction(::std::boxed::Box<AnonymousFunction<'tree>>),
    Atom(::std::boxed::Box<Atom<'tree>>),
    BinaryOperator(::std::boxed::Box<BinaryOperator<'tree>>),
    Bitstring(::std::boxed::Box<Bitstring<'tree>>),
    Block(::std::boxed::Box<Block<'tree>>),
    Boolean(::std::boxed::Box<Boolean<'tree>>),
    Call(::std::boxed::Box<Call<'tree>>),
    Char(::std::boxed::Box<Char<'tree>>),
    Charlist(::std::boxed::Box<Charlist<'tree>>),
    Dot(::std::boxed::Box<Dot<'tree>>),
    Float(::std::boxed::Box<Float<'tree>>),
    Identifier(::std::boxed::Box<Identifier<'tree>>),
    Integer(::std::boxed::Box<Integer<'tree>>),
    Keywords(::std::boxed::Box<Keywords<'tree>>),
    List(::std::boxed::Box<List<'tree>>),
    Map(::std::boxed::Box<Map<'tree>>),
    Nil(::std::boxed::Box<Nil<'tree>>),
    OperatorIdentifier(::std::boxed::Box<OperatorIdentifier<'tree>>),
    QuotedAtom(::std::boxed::Box<QuotedAtom<'tree>>),
    Sigil(::std::boxed::Box<Sigil<'tree>>),
    String(::std::boxed::Box<String<'tree>>),
    Tuple(::std::boxed::Box<Tuple<'tree>>),
    UnaryOperator(::std::boxed::Box<UnaryOperator<'tree>>),
}
impl<'tree> ::treesitter_types::FromNode<'tree> for BitstringChildren<'tree> {
    #[allow(clippy::collapsible_else_if)]
    fn from_node(
        node: ::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        match node.kind() {
            "access_call" => Ok(Self::AccessCall(::std::boxed::Box::new(
                <AccessCall as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "alias" => Ok(Self::Alias(::std::boxed::Box::new(
                <Alias as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "anonymous_function" => Ok(Self::AnonymousFunction(::std::boxed::Box::new(
                <AnonymousFunction as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "atom" => Ok(Self::Atom(::std::boxed::Box::new(
                <Atom as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "binary_operator" => Ok(Self::BinaryOperator(::std::boxed::Box::new(
                <BinaryOperator as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "bitstring" => Ok(Self::Bitstring(::std::boxed::Box::new(
                <Bitstring as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "block" => Ok(Self::Block(::std::boxed::Box::new(
                <Block as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "boolean" => Ok(Self::Boolean(::std::boxed::Box::new(
                <Boolean as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "call" => Ok(Self::Call(::std::boxed::Box::new(
                <Call as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "char" => Ok(Self::Char(::std::boxed::Box::new(
                <Char as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "charlist" => Ok(Self::Charlist(::std::boxed::Box::new(
                <Charlist as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "dot" => Ok(Self::Dot(::std::boxed::Box::new(
                <Dot as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "float" => Ok(Self::Float(::std::boxed::Box::new(
                <Float as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "identifier" => Ok(Self::Identifier(::std::boxed::Box::new(
                <Identifier as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "integer" => Ok(Self::Integer(::std::boxed::Box::new(
                <Integer as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "keywords" => Ok(Self::Keywords(::std::boxed::Box::new(
                <Keywords as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "list" => Ok(Self::List(::std::boxed::Box::new(
                <List as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "map" => Ok(Self::Map(::std::boxed::Box::new(
                <Map as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "nil" => Ok(Self::Nil(::std::boxed::Box::new(
                <Nil as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "operator_identifier" => Ok(Self::OperatorIdentifier(::std::boxed::Box::new(
                <OperatorIdentifier as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "quoted_atom" => Ok(Self::QuotedAtom(::std::boxed::Box::new(
                <QuotedAtom as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "sigil" => Ok(Self::Sigil(::std::boxed::Box::new(
                <Sigil as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "string" => Ok(Self::String(::std::boxed::Box::new(
                <String as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "tuple" => Ok(Self::Tuple(::std::boxed::Box::new(
                <Tuple as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "unary_operator" => Ok(Self::UnaryOperator(::std::boxed::Box::new(
                <UnaryOperator as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            other => Err(::treesitter_types::ParseError::unexpected_kind(other, node)),
        }
    }
}
impl ::treesitter_types::Spanned for BitstringChildren<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        match self {
            Self::AccessCall(inner) => inner.span(),
            Self::Alias(inner) => inner.span(),
            Self::AnonymousFunction(inner) => inner.span(),
            Self::Atom(inner) => inner.span(),
            Self::BinaryOperator(inner) => inner.span(),
            Self::Bitstring(inner) => inner.span(),
            Self::Block(inner) => inner.span(),
            Self::Boolean(inner) => inner.span(),
            Self::Call(inner) => inner.span(),
            Self::Char(inner) => inner.span(),
            Self::Charlist(inner) => inner.span(),
            Self::Dot(inner) => inner.span(),
            Self::Float(inner) => inner.span(),
            Self::Identifier(inner) => inner.span(),
            Self::Integer(inner) => inner.span(),
            Self::Keywords(inner) => inner.span(),
            Self::List(inner) => inner.span(),
            Self::Map(inner) => inner.span(),
            Self::Nil(inner) => inner.span(),
            Self::OperatorIdentifier(inner) => inner.span(),
            Self::QuotedAtom(inner) => inner.span(),
            Self::Sigil(inner) => inner.span(),
            Self::String(inner) => inner.span(),
            Self::Tuple(inner) => inner.span(),
            Self::UnaryOperator(inner) => inner.span(),
        }
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum BlockChildren<'tree> {
    AccessCall(::std::boxed::Box<AccessCall<'tree>>),
    Alias(::std::boxed::Box<Alias<'tree>>),
    AnonymousFunction(::std::boxed::Box<AnonymousFunction<'tree>>),
    Atom(::std::boxed::Box<Atom<'tree>>),
    BinaryOperator(::std::boxed::Box<BinaryOperator<'tree>>),
    Bitstring(::std::boxed::Box<Bitstring<'tree>>),
    Block(::std::boxed::Box<Block<'tree>>),
    Boolean(::std::boxed::Box<Boolean<'tree>>),
    Call(::std::boxed::Box<Call<'tree>>),
    Char(::std::boxed::Box<Char<'tree>>),
    Charlist(::std::boxed::Box<Charlist<'tree>>),
    Dot(::std::boxed::Box<Dot<'tree>>),
    Float(::std::boxed::Box<Float<'tree>>),
    Identifier(::std::boxed::Box<Identifier<'tree>>),
    Integer(::std::boxed::Box<Integer<'tree>>),
    List(::std::boxed::Box<List<'tree>>),
    Map(::std::boxed::Box<Map<'tree>>),
    Nil(::std::boxed::Box<Nil<'tree>>),
    OperatorIdentifier(::std::boxed::Box<OperatorIdentifier<'tree>>),
    QuotedAtom(::std::boxed::Box<QuotedAtom<'tree>>),
    Sigil(::std::boxed::Box<Sigil<'tree>>),
    StabClause(::std::boxed::Box<StabClause<'tree>>),
    String(::std::boxed::Box<String<'tree>>),
    Tuple(::std::boxed::Box<Tuple<'tree>>),
    UnaryOperator(::std::boxed::Box<UnaryOperator<'tree>>),
}
impl<'tree> ::treesitter_types::FromNode<'tree> for BlockChildren<'tree> {
    #[allow(clippy::collapsible_else_if)]
    fn from_node(
        node: ::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        match node.kind() {
            "access_call" => Ok(Self::AccessCall(::std::boxed::Box::new(
                <AccessCall as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "alias" => Ok(Self::Alias(::std::boxed::Box::new(
                <Alias as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "anonymous_function" => Ok(Self::AnonymousFunction(::std::boxed::Box::new(
                <AnonymousFunction as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "atom" => Ok(Self::Atom(::std::boxed::Box::new(
                <Atom as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "binary_operator" => Ok(Self::BinaryOperator(::std::boxed::Box::new(
                <BinaryOperator as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "bitstring" => Ok(Self::Bitstring(::std::boxed::Box::new(
                <Bitstring as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "block" => Ok(Self::Block(::std::boxed::Box::new(
                <Block as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "boolean" => Ok(Self::Boolean(::std::boxed::Box::new(
                <Boolean as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "call" => Ok(Self::Call(::std::boxed::Box::new(
                <Call as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "char" => Ok(Self::Char(::std::boxed::Box::new(
                <Char as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "charlist" => Ok(Self::Charlist(::std::boxed::Box::new(
                <Charlist as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "dot" => Ok(Self::Dot(::std::boxed::Box::new(
                <Dot as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "float" => Ok(Self::Float(::std::boxed::Box::new(
                <Float as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "identifier" => Ok(Self::Identifier(::std::boxed::Box::new(
                <Identifier as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "integer" => Ok(Self::Integer(::std::boxed::Box::new(
                <Integer as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "list" => Ok(Self::List(::std::boxed::Box::new(
                <List as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "map" => Ok(Self::Map(::std::boxed::Box::new(
                <Map as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "nil" => Ok(Self::Nil(::std::boxed::Box::new(
                <Nil as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "operator_identifier" => Ok(Self::OperatorIdentifier(::std::boxed::Box::new(
                <OperatorIdentifier as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "quoted_atom" => Ok(Self::QuotedAtom(::std::boxed::Box::new(
                <QuotedAtom as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "sigil" => Ok(Self::Sigil(::std::boxed::Box::new(
                <Sigil as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "stab_clause" => Ok(Self::StabClause(::std::boxed::Box::new(
                <StabClause as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "string" => Ok(Self::String(::std::boxed::Box::new(
                <String as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "tuple" => Ok(Self::Tuple(::std::boxed::Box::new(
                <Tuple as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "unary_operator" => Ok(Self::UnaryOperator(::std::boxed::Box::new(
                <UnaryOperator as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            other => Err(::treesitter_types::ParseError::unexpected_kind(other, node)),
        }
    }
}
impl ::treesitter_types::Spanned for BlockChildren<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        match self {
            Self::AccessCall(inner) => inner.span(),
            Self::Alias(inner) => inner.span(),
            Self::AnonymousFunction(inner) => inner.span(),
            Self::Atom(inner) => inner.span(),
            Self::BinaryOperator(inner) => inner.span(),
            Self::Bitstring(inner) => inner.span(),
            Self::Block(inner) => inner.span(),
            Self::Boolean(inner) => inner.span(),
            Self::Call(inner) => inner.span(),
            Self::Char(inner) => inner.span(),
            Self::Charlist(inner) => inner.span(),
            Self::Dot(inner) => inner.span(),
            Self::Float(inner) => inner.span(),
            Self::Identifier(inner) => inner.span(),
            Self::Integer(inner) => inner.span(),
            Self::List(inner) => inner.span(),
            Self::Map(inner) => inner.span(),
            Self::Nil(inner) => inner.span(),
            Self::OperatorIdentifier(inner) => inner.span(),
            Self::QuotedAtom(inner) => inner.span(),
            Self::Sigil(inner) => inner.span(),
            Self::StabClause(inner) => inner.span(),
            Self::String(inner) => inner.span(),
            Self::Tuple(inner) => inner.span(),
            Self::UnaryOperator(inner) => inner.span(),
        }
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum BodyChildren<'tree> {
    AccessCall(::std::boxed::Box<AccessCall<'tree>>),
    Alias(::std::boxed::Box<Alias<'tree>>),
    AnonymousFunction(::std::boxed::Box<AnonymousFunction<'tree>>),
    Atom(::std::boxed::Box<Atom<'tree>>),
    BinaryOperator(::std::boxed::Box<BinaryOperator<'tree>>),
    Bitstring(::std::boxed::Box<Bitstring<'tree>>),
    Block(::std::boxed::Box<Block<'tree>>),
    Boolean(::std::boxed::Box<Boolean<'tree>>),
    Call(::std::boxed::Box<Call<'tree>>),
    Char(::std::boxed::Box<Char<'tree>>),
    Charlist(::std::boxed::Box<Charlist<'tree>>),
    Dot(::std::boxed::Box<Dot<'tree>>),
    Float(::std::boxed::Box<Float<'tree>>),
    Identifier(::std::boxed::Box<Identifier<'tree>>),
    Integer(::std::boxed::Box<Integer<'tree>>),
    List(::std::boxed::Box<List<'tree>>),
    Map(::std::boxed::Box<Map<'tree>>),
    Nil(::std::boxed::Box<Nil<'tree>>),
    OperatorIdentifier(::std::boxed::Box<OperatorIdentifier<'tree>>),
    QuotedAtom(::std::boxed::Box<QuotedAtom<'tree>>),
    Sigil(::std::boxed::Box<Sigil<'tree>>),
    String(::std::boxed::Box<String<'tree>>),
    Tuple(::std::boxed::Box<Tuple<'tree>>),
    UnaryOperator(::std::boxed::Box<UnaryOperator<'tree>>),
}
impl<'tree> ::treesitter_types::FromNode<'tree> for BodyChildren<'tree> {
    #[allow(clippy::collapsible_else_if)]
    fn from_node(
        node: ::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        match node.kind() {
            "access_call" => Ok(Self::AccessCall(::std::boxed::Box::new(
                <AccessCall as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "alias" => Ok(Self::Alias(::std::boxed::Box::new(
                <Alias as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "anonymous_function" => Ok(Self::AnonymousFunction(::std::boxed::Box::new(
                <AnonymousFunction as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "atom" => Ok(Self::Atom(::std::boxed::Box::new(
                <Atom as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "binary_operator" => Ok(Self::BinaryOperator(::std::boxed::Box::new(
                <BinaryOperator as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "bitstring" => Ok(Self::Bitstring(::std::boxed::Box::new(
                <Bitstring as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "block" => Ok(Self::Block(::std::boxed::Box::new(
                <Block as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "boolean" => Ok(Self::Boolean(::std::boxed::Box::new(
                <Boolean as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "call" => Ok(Self::Call(::std::boxed::Box::new(
                <Call as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "char" => Ok(Self::Char(::std::boxed::Box::new(
                <Char as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "charlist" => Ok(Self::Charlist(::std::boxed::Box::new(
                <Charlist as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "dot" => Ok(Self::Dot(::std::boxed::Box::new(
                <Dot as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "float" => Ok(Self::Float(::std::boxed::Box::new(
                <Float as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "identifier" => Ok(Self::Identifier(::std::boxed::Box::new(
                <Identifier as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "integer" => Ok(Self::Integer(::std::boxed::Box::new(
                <Integer as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "list" => Ok(Self::List(::std::boxed::Box::new(
                <List as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "map" => Ok(Self::Map(::std::boxed::Box::new(
                <Map as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "nil" => Ok(Self::Nil(::std::boxed::Box::new(
                <Nil as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "operator_identifier" => Ok(Self::OperatorIdentifier(::std::boxed::Box::new(
                <OperatorIdentifier as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "quoted_atom" => Ok(Self::QuotedAtom(::std::boxed::Box::new(
                <QuotedAtom as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "sigil" => Ok(Self::Sigil(::std::boxed::Box::new(
                <Sigil as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "string" => Ok(Self::String(::std::boxed::Box::new(
                <String as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "tuple" => Ok(Self::Tuple(::std::boxed::Box::new(
                <Tuple as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "unary_operator" => Ok(Self::UnaryOperator(::std::boxed::Box::new(
                <UnaryOperator as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            other => Err(::treesitter_types::ParseError::unexpected_kind(other, node)),
        }
    }
}
impl ::treesitter_types::Spanned for BodyChildren<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        match self {
            Self::AccessCall(inner) => inner.span(),
            Self::Alias(inner) => inner.span(),
            Self::AnonymousFunction(inner) => inner.span(),
            Self::Atom(inner) => inner.span(),
            Self::BinaryOperator(inner) => inner.span(),
            Self::Bitstring(inner) => inner.span(),
            Self::Block(inner) => inner.span(),
            Self::Boolean(inner) => inner.span(),
            Self::Call(inner) => inner.span(),
            Self::Char(inner) => inner.span(),
            Self::Charlist(inner) => inner.span(),
            Self::Dot(inner) => inner.span(),
            Self::Float(inner) => inner.span(),
            Self::Identifier(inner) => inner.span(),
            Self::Integer(inner) => inner.span(),
            Self::List(inner) => inner.span(),
            Self::Map(inner) => inner.span(),
            Self::Nil(inner) => inner.span(),
            Self::OperatorIdentifier(inner) => inner.span(),
            Self::QuotedAtom(inner) => inner.span(),
            Self::Sigil(inner) => inner.span(),
            Self::String(inner) => inner.span(),
            Self::Tuple(inner) => inner.span(),
            Self::UnaryOperator(inner) => inner.span(),
        }
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum CallTarget<'tree> {
    Call(::std::boxed::Box<Call<'tree>>),
    Dot(::std::boxed::Box<Dot<'tree>>),
    Identifier(::std::boxed::Box<Identifier<'tree>>),
}
impl<'tree> ::treesitter_types::FromNode<'tree> for CallTarget<'tree> {
    #[allow(clippy::collapsible_else_if)]
    fn from_node(
        node: ::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        match node.kind() {
            "call" => Ok(Self::Call(::std::boxed::Box::new(
                <Call as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "dot" => Ok(Self::Dot(::std::boxed::Box::new(
                <Dot as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "identifier" => Ok(Self::Identifier(::std::boxed::Box::new(
                <Identifier as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            other => Err(::treesitter_types::ParseError::unexpected_kind(other, node)),
        }
    }
}
impl ::treesitter_types::Spanned for CallTarget<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        match self {
            Self::Call(inner) => inner.span(),
            Self::Dot(inner) => inner.span(),
            Self::Identifier(inner) => inner.span(),
        }
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum CallChildren<'tree> {
    Arguments(::std::boxed::Box<Arguments<'tree>>),
    DoBlock(::std::boxed::Box<DoBlock<'tree>>),
}
impl<'tree> ::treesitter_types::FromNode<'tree> for CallChildren<'tree> {
    #[allow(clippy::collapsible_else_if)]
    fn from_node(
        node: ::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        match node.kind() {
            "arguments" => Ok(Self::Arguments(::std::boxed::Box::new(
                <Arguments as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "do_block" => Ok(Self::DoBlock(::std::boxed::Box::new(
                <DoBlock as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            other => Err(::treesitter_types::ParseError::unexpected_kind(other, node)),
        }
    }
}
impl ::treesitter_types::Spanned for CallChildren<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        match self {
            Self::Arguments(inner) => inner.span(),
            Self::DoBlock(inner) => inner.span(),
        }
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum CatchBlockChildren<'tree> {
    AccessCall(::std::boxed::Box<AccessCall<'tree>>),
    Alias(::std::boxed::Box<Alias<'tree>>),
    AnonymousFunction(::std::boxed::Box<AnonymousFunction<'tree>>),
    Atom(::std::boxed::Box<Atom<'tree>>),
    BinaryOperator(::std::boxed::Box<BinaryOperator<'tree>>),
    Bitstring(::std::boxed::Box<Bitstring<'tree>>),
    Block(::std::boxed::Box<Block<'tree>>),
    Boolean(::std::boxed::Box<Boolean<'tree>>),
    Call(::std::boxed::Box<Call<'tree>>),
    Char(::std::boxed::Box<Char<'tree>>),
    Charlist(::std::boxed::Box<Charlist<'tree>>),
    Dot(::std::boxed::Box<Dot<'tree>>),
    Float(::std::boxed::Box<Float<'tree>>),
    Identifier(::std::boxed::Box<Identifier<'tree>>),
    Integer(::std::boxed::Box<Integer<'tree>>),
    List(::std::boxed::Box<List<'tree>>),
    Map(::std::boxed::Box<Map<'tree>>),
    Nil(::std::boxed::Box<Nil<'tree>>),
    OperatorIdentifier(::std::boxed::Box<OperatorIdentifier<'tree>>),
    QuotedAtom(::std::boxed::Box<QuotedAtom<'tree>>),
    Sigil(::std::boxed::Box<Sigil<'tree>>),
    StabClause(::std::boxed::Box<StabClause<'tree>>),
    String(::std::boxed::Box<String<'tree>>),
    Tuple(::std::boxed::Box<Tuple<'tree>>),
    UnaryOperator(::std::boxed::Box<UnaryOperator<'tree>>),
}
impl<'tree> ::treesitter_types::FromNode<'tree> for CatchBlockChildren<'tree> {
    #[allow(clippy::collapsible_else_if)]
    fn from_node(
        node: ::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        match node.kind() {
            "access_call" => Ok(Self::AccessCall(::std::boxed::Box::new(
                <AccessCall as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "alias" => Ok(Self::Alias(::std::boxed::Box::new(
                <Alias as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "anonymous_function" => Ok(Self::AnonymousFunction(::std::boxed::Box::new(
                <AnonymousFunction as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "atom" => Ok(Self::Atom(::std::boxed::Box::new(
                <Atom as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "binary_operator" => Ok(Self::BinaryOperator(::std::boxed::Box::new(
                <BinaryOperator as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "bitstring" => Ok(Self::Bitstring(::std::boxed::Box::new(
                <Bitstring as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "block" => Ok(Self::Block(::std::boxed::Box::new(
                <Block as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "boolean" => Ok(Self::Boolean(::std::boxed::Box::new(
                <Boolean as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "call" => Ok(Self::Call(::std::boxed::Box::new(
                <Call as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "char" => Ok(Self::Char(::std::boxed::Box::new(
                <Char as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "charlist" => Ok(Self::Charlist(::std::boxed::Box::new(
                <Charlist as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "dot" => Ok(Self::Dot(::std::boxed::Box::new(
                <Dot as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "float" => Ok(Self::Float(::std::boxed::Box::new(
                <Float as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "identifier" => Ok(Self::Identifier(::std::boxed::Box::new(
                <Identifier as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "integer" => Ok(Self::Integer(::std::boxed::Box::new(
                <Integer as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "list" => Ok(Self::List(::std::boxed::Box::new(
                <List as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "map" => Ok(Self::Map(::std::boxed::Box::new(
                <Map as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "nil" => Ok(Self::Nil(::std::boxed::Box::new(
                <Nil as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "operator_identifier" => Ok(Self::OperatorIdentifier(::std::boxed::Box::new(
                <OperatorIdentifier as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "quoted_atom" => Ok(Self::QuotedAtom(::std::boxed::Box::new(
                <QuotedAtom as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "sigil" => Ok(Self::Sigil(::std::boxed::Box::new(
                <Sigil as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "stab_clause" => Ok(Self::StabClause(::std::boxed::Box::new(
                <StabClause as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "string" => Ok(Self::String(::std::boxed::Box::new(
                <String as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "tuple" => Ok(Self::Tuple(::std::boxed::Box::new(
                <Tuple as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "unary_operator" => Ok(Self::UnaryOperator(::std::boxed::Box::new(
                <UnaryOperator as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            other => Err(::treesitter_types::ParseError::unexpected_kind(other, node)),
        }
    }
}
impl ::treesitter_types::Spanned for CatchBlockChildren<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        match self {
            Self::AccessCall(inner) => inner.span(),
            Self::Alias(inner) => inner.span(),
            Self::AnonymousFunction(inner) => inner.span(),
            Self::Atom(inner) => inner.span(),
            Self::BinaryOperator(inner) => inner.span(),
            Self::Bitstring(inner) => inner.span(),
            Self::Block(inner) => inner.span(),
            Self::Boolean(inner) => inner.span(),
            Self::Call(inner) => inner.span(),
            Self::Char(inner) => inner.span(),
            Self::Charlist(inner) => inner.span(),
            Self::Dot(inner) => inner.span(),
            Self::Float(inner) => inner.span(),
            Self::Identifier(inner) => inner.span(),
            Self::Integer(inner) => inner.span(),
            Self::List(inner) => inner.span(),
            Self::Map(inner) => inner.span(),
            Self::Nil(inner) => inner.span(),
            Self::OperatorIdentifier(inner) => inner.span(),
            Self::QuotedAtom(inner) => inner.span(),
            Self::Sigil(inner) => inner.span(),
            Self::StabClause(inner) => inner.span(),
            Self::String(inner) => inner.span(),
            Self::Tuple(inner) => inner.span(),
            Self::UnaryOperator(inner) => inner.span(),
        }
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum CharlistQuotedEnd {
    SingleQuote(::treesitter_types::Span),
    SQuoteSQuoteSQuote(::treesitter_types::Span),
}
impl<'tree> ::treesitter_types::FromNode<'tree> for CharlistQuotedEnd {
    #[allow(clippy::collapsible_else_if)]
    fn from_node(
        node: ::tree_sitter::Node<'tree>,
        _src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        match node.kind() {
            "'" => Ok(Self::SingleQuote(::treesitter_types::Span::from(node))),
            "'''" => Ok(Self::SQuoteSQuoteSQuote(::treesitter_types::Span::from(
                node,
            ))),
            other => Err(::treesitter_types::ParseError::unexpected_kind(other, node)),
        }
    }
}
impl ::treesitter_types::Spanned for CharlistQuotedEnd {
    fn span(&self) -> ::treesitter_types::Span {
        match self {
            Self::SingleQuote(span) => *span,
            Self::SQuoteSQuoteSQuote(span) => *span,
        }
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum CharlistQuotedStart {
    SingleQuote(::treesitter_types::Span),
    SQuoteSQuoteSQuote(::treesitter_types::Span),
}
impl<'tree> ::treesitter_types::FromNode<'tree> for CharlistQuotedStart {
    #[allow(clippy::collapsible_else_if)]
    fn from_node(
        node: ::tree_sitter::Node<'tree>,
        _src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        match node.kind() {
            "'" => Ok(Self::SingleQuote(::treesitter_types::Span::from(node))),
            "'''" => Ok(Self::SQuoteSQuoteSQuote(::treesitter_types::Span::from(
                node,
            ))),
            other => Err(::treesitter_types::ParseError::unexpected_kind(other, node)),
        }
    }
}
impl ::treesitter_types::Spanned for CharlistQuotedStart {
    fn span(&self) -> ::treesitter_types::Span {
        match self {
            Self::SingleQuote(span) => *span,
            Self::SQuoteSQuoteSQuote(span) => *span,
        }
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum CharlistChildren<'tree> {
    EscapeSequence(::std::boxed::Box<EscapeSequence<'tree>>),
    Interpolation(::std::boxed::Box<Interpolation<'tree>>),
    QuotedContent(::std::boxed::Box<QuotedContent<'tree>>),
}
impl<'tree> ::treesitter_types::FromNode<'tree> for CharlistChildren<'tree> {
    #[allow(clippy::collapsible_else_if)]
    fn from_node(
        node: ::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        match node.kind() {
            "escape_sequence" => Ok(Self::EscapeSequence(::std::boxed::Box::new(
                <EscapeSequence as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "interpolation" => Ok(Self::Interpolation(::std::boxed::Box::new(
                <Interpolation as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "quoted_content" => Ok(Self::QuotedContent(::std::boxed::Box::new(
                <QuotedContent as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            other => Err(::treesitter_types::ParseError::unexpected_kind(other, node)),
        }
    }
}
impl ::treesitter_types::Spanned for CharlistChildren<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        match self {
            Self::EscapeSequence(inner) => inner.span(),
            Self::Interpolation(inner) => inner.span(),
            Self::QuotedContent(inner) => inner.span(),
        }
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum DoBlockChildren<'tree> {
    AccessCall(::std::boxed::Box<AccessCall<'tree>>),
    AfterBlock(::std::boxed::Box<AfterBlock<'tree>>),
    Alias(::std::boxed::Box<Alias<'tree>>),
    AnonymousFunction(::std::boxed::Box<AnonymousFunction<'tree>>),
    Atom(::std::boxed::Box<Atom<'tree>>),
    BinaryOperator(::std::boxed::Box<BinaryOperator<'tree>>),
    Bitstring(::std::boxed::Box<Bitstring<'tree>>),
    Block(::std::boxed::Box<Block<'tree>>),
    Boolean(::std::boxed::Box<Boolean<'tree>>),
    Call(::std::boxed::Box<Call<'tree>>),
    CatchBlock(::std::boxed::Box<CatchBlock<'tree>>),
    Char(::std::boxed::Box<Char<'tree>>),
    Charlist(::std::boxed::Box<Charlist<'tree>>),
    Dot(::std::boxed::Box<Dot<'tree>>),
    ElseBlock(::std::boxed::Box<ElseBlock<'tree>>),
    Float(::std::boxed::Box<Float<'tree>>),
    Identifier(::std::boxed::Box<Identifier<'tree>>),
    Integer(::std::boxed::Box<Integer<'tree>>),
    List(::std::boxed::Box<List<'tree>>),
    Map(::std::boxed::Box<Map<'tree>>),
    Nil(::std::boxed::Box<Nil<'tree>>),
    OperatorIdentifier(::std::boxed::Box<OperatorIdentifier<'tree>>),
    QuotedAtom(::std::boxed::Box<QuotedAtom<'tree>>),
    RescueBlock(::std::boxed::Box<RescueBlock<'tree>>),
    Sigil(::std::boxed::Box<Sigil<'tree>>),
    StabClause(::std::boxed::Box<StabClause<'tree>>),
    String(::std::boxed::Box<String<'tree>>),
    Tuple(::std::boxed::Box<Tuple<'tree>>),
    UnaryOperator(::std::boxed::Box<UnaryOperator<'tree>>),
}
impl<'tree> ::treesitter_types::FromNode<'tree> for DoBlockChildren<'tree> {
    #[allow(clippy::collapsible_else_if)]
    fn from_node(
        node: ::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        match node.kind() {
            "access_call" => Ok(Self::AccessCall(::std::boxed::Box::new(
                <AccessCall as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "after_block" => Ok(Self::AfterBlock(::std::boxed::Box::new(
                <AfterBlock as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "alias" => Ok(Self::Alias(::std::boxed::Box::new(
                <Alias as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "anonymous_function" => Ok(Self::AnonymousFunction(::std::boxed::Box::new(
                <AnonymousFunction as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "atom" => Ok(Self::Atom(::std::boxed::Box::new(
                <Atom as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "binary_operator" => Ok(Self::BinaryOperator(::std::boxed::Box::new(
                <BinaryOperator as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "bitstring" => Ok(Self::Bitstring(::std::boxed::Box::new(
                <Bitstring as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "block" => Ok(Self::Block(::std::boxed::Box::new(
                <Block as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "boolean" => Ok(Self::Boolean(::std::boxed::Box::new(
                <Boolean as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "call" => Ok(Self::Call(::std::boxed::Box::new(
                <Call as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "catch_block" => Ok(Self::CatchBlock(::std::boxed::Box::new(
                <CatchBlock as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "char" => Ok(Self::Char(::std::boxed::Box::new(
                <Char as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "charlist" => Ok(Self::Charlist(::std::boxed::Box::new(
                <Charlist as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "dot" => Ok(Self::Dot(::std::boxed::Box::new(
                <Dot as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "else_block" => Ok(Self::ElseBlock(::std::boxed::Box::new(
                <ElseBlock as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "float" => Ok(Self::Float(::std::boxed::Box::new(
                <Float as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "identifier" => Ok(Self::Identifier(::std::boxed::Box::new(
                <Identifier as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "integer" => Ok(Self::Integer(::std::boxed::Box::new(
                <Integer as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "list" => Ok(Self::List(::std::boxed::Box::new(
                <List as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "map" => Ok(Self::Map(::std::boxed::Box::new(
                <Map as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "nil" => Ok(Self::Nil(::std::boxed::Box::new(
                <Nil as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "operator_identifier" => Ok(Self::OperatorIdentifier(::std::boxed::Box::new(
                <OperatorIdentifier as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "quoted_atom" => Ok(Self::QuotedAtom(::std::boxed::Box::new(
                <QuotedAtom as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "rescue_block" => Ok(Self::RescueBlock(::std::boxed::Box::new(
                <RescueBlock as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "sigil" => Ok(Self::Sigil(::std::boxed::Box::new(
                <Sigil as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "stab_clause" => Ok(Self::StabClause(::std::boxed::Box::new(
                <StabClause as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "string" => Ok(Self::String(::std::boxed::Box::new(
                <String as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "tuple" => Ok(Self::Tuple(::std::boxed::Box::new(
                <Tuple as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "unary_operator" => Ok(Self::UnaryOperator(::std::boxed::Box::new(
                <UnaryOperator as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            other => Err(::treesitter_types::ParseError::unexpected_kind(other, node)),
        }
    }
}
impl ::treesitter_types::Spanned for DoBlockChildren<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        match self {
            Self::AccessCall(inner) => inner.span(),
            Self::AfterBlock(inner) => inner.span(),
            Self::Alias(inner) => inner.span(),
            Self::AnonymousFunction(inner) => inner.span(),
            Self::Atom(inner) => inner.span(),
            Self::BinaryOperator(inner) => inner.span(),
            Self::Bitstring(inner) => inner.span(),
            Self::Block(inner) => inner.span(),
            Self::Boolean(inner) => inner.span(),
            Self::Call(inner) => inner.span(),
            Self::CatchBlock(inner) => inner.span(),
            Self::Char(inner) => inner.span(),
            Self::Charlist(inner) => inner.span(),
            Self::Dot(inner) => inner.span(),
            Self::ElseBlock(inner) => inner.span(),
            Self::Float(inner) => inner.span(),
            Self::Identifier(inner) => inner.span(),
            Self::Integer(inner) => inner.span(),
            Self::List(inner) => inner.span(),
            Self::Map(inner) => inner.span(),
            Self::Nil(inner) => inner.span(),
            Self::OperatorIdentifier(inner) => inner.span(),
            Self::QuotedAtom(inner) => inner.span(),
            Self::RescueBlock(inner) => inner.span(),
            Self::Sigil(inner) => inner.span(),
            Self::StabClause(inner) => inner.span(),
            Self::String(inner) => inner.span(),
            Self::Tuple(inner) => inner.span(),
            Self::UnaryOperator(inner) => inner.span(),
        }
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum DotLeft<'tree> {
    AccessCall(::std::boxed::Box<AccessCall<'tree>>),
    Alias(::std::boxed::Box<Alias<'tree>>),
    AnonymousFunction(::std::boxed::Box<AnonymousFunction<'tree>>),
    Atom(::std::boxed::Box<Atom<'tree>>),
    BinaryOperator(::std::boxed::Box<BinaryOperator<'tree>>),
    Bitstring(::std::boxed::Box<Bitstring<'tree>>),
    Block(::std::boxed::Box<Block<'tree>>),
    Boolean(::std::boxed::Box<Boolean<'tree>>),
    Call(::std::boxed::Box<Call<'tree>>),
    Char(::std::boxed::Box<Char<'tree>>),
    Charlist(::std::boxed::Box<Charlist<'tree>>),
    Dot(::std::boxed::Box<Dot<'tree>>),
    Float(::std::boxed::Box<Float<'tree>>),
    Identifier(::std::boxed::Box<Identifier<'tree>>),
    Integer(::std::boxed::Box<Integer<'tree>>),
    List(::std::boxed::Box<List<'tree>>),
    Map(::std::boxed::Box<Map<'tree>>),
    Nil(::std::boxed::Box<Nil<'tree>>),
    OperatorIdentifier(::std::boxed::Box<OperatorIdentifier<'tree>>),
    QuotedAtom(::std::boxed::Box<QuotedAtom<'tree>>),
    Sigil(::std::boxed::Box<Sigil<'tree>>),
    String(::std::boxed::Box<String<'tree>>),
    Tuple(::std::boxed::Box<Tuple<'tree>>),
    UnaryOperator(::std::boxed::Box<UnaryOperator<'tree>>),
}
impl<'tree> ::treesitter_types::FromNode<'tree> for DotLeft<'tree> {
    #[allow(clippy::collapsible_else_if)]
    fn from_node(
        node: ::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        match node.kind() {
            "access_call" => Ok(Self::AccessCall(::std::boxed::Box::new(
                <AccessCall as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "alias" => Ok(Self::Alias(::std::boxed::Box::new(
                <Alias as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "anonymous_function" => Ok(Self::AnonymousFunction(::std::boxed::Box::new(
                <AnonymousFunction as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "atom" => Ok(Self::Atom(::std::boxed::Box::new(
                <Atom as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "binary_operator" => Ok(Self::BinaryOperator(::std::boxed::Box::new(
                <BinaryOperator as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "bitstring" => Ok(Self::Bitstring(::std::boxed::Box::new(
                <Bitstring as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "block" => Ok(Self::Block(::std::boxed::Box::new(
                <Block as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "boolean" => Ok(Self::Boolean(::std::boxed::Box::new(
                <Boolean as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "call" => Ok(Self::Call(::std::boxed::Box::new(
                <Call as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "char" => Ok(Self::Char(::std::boxed::Box::new(
                <Char as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "charlist" => Ok(Self::Charlist(::std::boxed::Box::new(
                <Charlist as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "dot" => Ok(Self::Dot(::std::boxed::Box::new(
                <Dot as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "float" => Ok(Self::Float(::std::boxed::Box::new(
                <Float as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "identifier" => Ok(Self::Identifier(::std::boxed::Box::new(
                <Identifier as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "integer" => Ok(Self::Integer(::std::boxed::Box::new(
                <Integer as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "list" => Ok(Self::List(::std::boxed::Box::new(
                <List as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "map" => Ok(Self::Map(::std::boxed::Box::new(
                <Map as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "nil" => Ok(Self::Nil(::std::boxed::Box::new(
                <Nil as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "operator_identifier" => Ok(Self::OperatorIdentifier(::std::boxed::Box::new(
                <OperatorIdentifier as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "quoted_atom" => Ok(Self::QuotedAtom(::std::boxed::Box::new(
                <QuotedAtom as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "sigil" => Ok(Self::Sigil(::std::boxed::Box::new(
                <Sigil as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "string" => Ok(Self::String(::std::boxed::Box::new(
                <String as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "tuple" => Ok(Self::Tuple(::std::boxed::Box::new(
                <Tuple as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "unary_operator" => Ok(Self::UnaryOperator(::std::boxed::Box::new(
                <UnaryOperator as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            other => Err(::treesitter_types::ParseError::unexpected_kind(other, node)),
        }
    }
}
impl ::treesitter_types::Spanned for DotLeft<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        match self {
            Self::AccessCall(inner) => inner.span(),
            Self::Alias(inner) => inner.span(),
            Self::AnonymousFunction(inner) => inner.span(),
            Self::Atom(inner) => inner.span(),
            Self::BinaryOperator(inner) => inner.span(),
            Self::Bitstring(inner) => inner.span(),
            Self::Block(inner) => inner.span(),
            Self::Boolean(inner) => inner.span(),
            Self::Call(inner) => inner.span(),
            Self::Char(inner) => inner.span(),
            Self::Charlist(inner) => inner.span(),
            Self::Dot(inner) => inner.span(),
            Self::Float(inner) => inner.span(),
            Self::Identifier(inner) => inner.span(),
            Self::Integer(inner) => inner.span(),
            Self::List(inner) => inner.span(),
            Self::Map(inner) => inner.span(),
            Self::Nil(inner) => inner.span(),
            Self::OperatorIdentifier(inner) => inner.span(),
            Self::QuotedAtom(inner) => inner.span(),
            Self::Sigil(inner) => inner.span(),
            Self::String(inner) => inner.span(),
            Self::Tuple(inner) => inner.span(),
            Self::UnaryOperator(inner) => inner.span(),
        }
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum DotOperator {
    Dot(::treesitter_types::Span),
}
impl<'tree> ::treesitter_types::FromNode<'tree> for DotOperator {
    #[allow(clippy::collapsible_else_if)]
    fn from_node(
        node: ::tree_sitter::Node<'tree>,
        _src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        match node.kind() {
            "." => Ok(Self::Dot(::treesitter_types::Span::from(node))),
            other => Err(::treesitter_types::ParseError::unexpected_kind(other, node)),
        }
    }
}
impl ::treesitter_types::Spanned for DotOperator {
    fn span(&self) -> ::treesitter_types::Span {
        match self {
            Self::Dot(span) => *span,
        }
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum DotRight<'tree> {
    Alias(::std::boxed::Box<Alias<'tree>>),
    Charlist(::std::boxed::Box<Charlist<'tree>>),
    Identifier(::std::boxed::Box<Identifier<'tree>>),
    OperatorIdentifier(::std::boxed::Box<OperatorIdentifier<'tree>>),
    String(::std::boxed::Box<String<'tree>>),
    Tuple(::std::boxed::Box<Tuple<'tree>>),
}
impl<'tree> ::treesitter_types::FromNode<'tree> for DotRight<'tree> {
    #[allow(clippy::collapsible_else_if)]
    fn from_node(
        node: ::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        match node.kind() {
            "alias" => Ok(Self::Alias(::std::boxed::Box::new(
                <Alias as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "charlist" => Ok(Self::Charlist(::std::boxed::Box::new(
                <Charlist as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "identifier" => Ok(Self::Identifier(::std::boxed::Box::new(
                <Identifier as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "operator_identifier" => Ok(Self::OperatorIdentifier(::std::boxed::Box::new(
                <OperatorIdentifier as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "string" => Ok(Self::String(::std::boxed::Box::new(
                <String as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "tuple" => Ok(Self::Tuple(::std::boxed::Box::new(
                <Tuple as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            other => Err(::treesitter_types::ParseError::unexpected_kind(other, node)),
        }
    }
}
impl ::treesitter_types::Spanned for DotRight<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        match self {
            Self::Alias(inner) => inner.span(),
            Self::Charlist(inner) => inner.span(),
            Self::Identifier(inner) => inner.span(),
            Self::OperatorIdentifier(inner) => inner.span(),
            Self::String(inner) => inner.span(),
            Self::Tuple(inner) => inner.span(),
        }
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ElseBlockChildren<'tree> {
    AccessCall(::std::boxed::Box<AccessCall<'tree>>),
    Alias(::std::boxed::Box<Alias<'tree>>),
    AnonymousFunction(::std::boxed::Box<AnonymousFunction<'tree>>),
    Atom(::std::boxed::Box<Atom<'tree>>),
    BinaryOperator(::std::boxed::Box<BinaryOperator<'tree>>),
    Bitstring(::std::boxed::Box<Bitstring<'tree>>),
    Block(::std::boxed::Box<Block<'tree>>),
    Boolean(::std::boxed::Box<Boolean<'tree>>),
    Call(::std::boxed::Box<Call<'tree>>),
    Char(::std::boxed::Box<Char<'tree>>),
    Charlist(::std::boxed::Box<Charlist<'tree>>),
    Dot(::std::boxed::Box<Dot<'tree>>),
    Float(::std::boxed::Box<Float<'tree>>),
    Identifier(::std::boxed::Box<Identifier<'tree>>),
    Integer(::std::boxed::Box<Integer<'tree>>),
    List(::std::boxed::Box<List<'tree>>),
    Map(::std::boxed::Box<Map<'tree>>),
    Nil(::std::boxed::Box<Nil<'tree>>),
    OperatorIdentifier(::std::boxed::Box<OperatorIdentifier<'tree>>),
    QuotedAtom(::std::boxed::Box<QuotedAtom<'tree>>),
    Sigil(::std::boxed::Box<Sigil<'tree>>),
    StabClause(::std::boxed::Box<StabClause<'tree>>),
    String(::std::boxed::Box<String<'tree>>),
    Tuple(::std::boxed::Box<Tuple<'tree>>),
    UnaryOperator(::std::boxed::Box<UnaryOperator<'tree>>),
}
impl<'tree> ::treesitter_types::FromNode<'tree> for ElseBlockChildren<'tree> {
    #[allow(clippy::collapsible_else_if)]
    fn from_node(
        node: ::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        match node.kind() {
            "access_call" => Ok(Self::AccessCall(::std::boxed::Box::new(
                <AccessCall as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "alias" => Ok(Self::Alias(::std::boxed::Box::new(
                <Alias as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "anonymous_function" => Ok(Self::AnonymousFunction(::std::boxed::Box::new(
                <AnonymousFunction as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "atom" => Ok(Self::Atom(::std::boxed::Box::new(
                <Atom as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "binary_operator" => Ok(Self::BinaryOperator(::std::boxed::Box::new(
                <BinaryOperator as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "bitstring" => Ok(Self::Bitstring(::std::boxed::Box::new(
                <Bitstring as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "block" => Ok(Self::Block(::std::boxed::Box::new(
                <Block as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "boolean" => Ok(Self::Boolean(::std::boxed::Box::new(
                <Boolean as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "call" => Ok(Self::Call(::std::boxed::Box::new(
                <Call as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "char" => Ok(Self::Char(::std::boxed::Box::new(
                <Char as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "charlist" => Ok(Self::Charlist(::std::boxed::Box::new(
                <Charlist as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "dot" => Ok(Self::Dot(::std::boxed::Box::new(
                <Dot as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "float" => Ok(Self::Float(::std::boxed::Box::new(
                <Float as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "identifier" => Ok(Self::Identifier(::std::boxed::Box::new(
                <Identifier as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "integer" => Ok(Self::Integer(::std::boxed::Box::new(
                <Integer as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "list" => Ok(Self::List(::std::boxed::Box::new(
                <List as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "map" => Ok(Self::Map(::std::boxed::Box::new(
                <Map as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "nil" => Ok(Self::Nil(::std::boxed::Box::new(
                <Nil as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "operator_identifier" => Ok(Self::OperatorIdentifier(::std::boxed::Box::new(
                <OperatorIdentifier as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "quoted_atom" => Ok(Self::QuotedAtom(::std::boxed::Box::new(
                <QuotedAtom as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "sigil" => Ok(Self::Sigil(::std::boxed::Box::new(
                <Sigil as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "stab_clause" => Ok(Self::StabClause(::std::boxed::Box::new(
                <StabClause as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "string" => Ok(Self::String(::std::boxed::Box::new(
                <String as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "tuple" => Ok(Self::Tuple(::std::boxed::Box::new(
                <Tuple as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "unary_operator" => Ok(Self::UnaryOperator(::std::boxed::Box::new(
                <UnaryOperator as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            other => Err(::treesitter_types::ParseError::unexpected_kind(other, node)),
        }
    }
}
impl ::treesitter_types::Spanned for ElseBlockChildren<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        match self {
            Self::AccessCall(inner) => inner.span(),
            Self::Alias(inner) => inner.span(),
            Self::AnonymousFunction(inner) => inner.span(),
            Self::Atom(inner) => inner.span(),
            Self::BinaryOperator(inner) => inner.span(),
            Self::Bitstring(inner) => inner.span(),
            Self::Block(inner) => inner.span(),
            Self::Boolean(inner) => inner.span(),
            Self::Call(inner) => inner.span(),
            Self::Char(inner) => inner.span(),
            Self::Charlist(inner) => inner.span(),
            Self::Dot(inner) => inner.span(),
            Self::Float(inner) => inner.span(),
            Self::Identifier(inner) => inner.span(),
            Self::Integer(inner) => inner.span(),
            Self::List(inner) => inner.span(),
            Self::Map(inner) => inner.span(),
            Self::Nil(inner) => inner.span(),
            Self::OperatorIdentifier(inner) => inner.span(),
            Self::QuotedAtom(inner) => inner.span(),
            Self::Sigil(inner) => inner.span(),
            Self::StabClause(inner) => inner.span(),
            Self::String(inner) => inner.span(),
            Self::Tuple(inner) => inner.span(),
            Self::UnaryOperator(inner) => inner.span(),
        }
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum InterpolationChildren<'tree> {
    AccessCall(::std::boxed::Box<AccessCall<'tree>>),
    Alias(::std::boxed::Box<Alias<'tree>>),
    AnonymousFunction(::std::boxed::Box<AnonymousFunction<'tree>>),
    Atom(::std::boxed::Box<Atom<'tree>>),
    BinaryOperator(::std::boxed::Box<BinaryOperator<'tree>>),
    Bitstring(::std::boxed::Box<Bitstring<'tree>>),
    Block(::std::boxed::Box<Block<'tree>>),
    Boolean(::std::boxed::Box<Boolean<'tree>>),
    Call(::std::boxed::Box<Call<'tree>>),
    Char(::std::boxed::Box<Char<'tree>>),
    Charlist(::std::boxed::Box<Charlist<'tree>>),
    Dot(::std::boxed::Box<Dot<'tree>>),
    Float(::std::boxed::Box<Float<'tree>>),
    Identifier(::std::boxed::Box<Identifier<'tree>>),
    Integer(::std::boxed::Box<Integer<'tree>>),
    List(::std::boxed::Box<List<'tree>>),
    Map(::std::boxed::Box<Map<'tree>>),
    Nil(::std::boxed::Box<Nil<'tree>>),
    OperatorIdentifier(::std::boxed::Box<OperatorIdentifier<'tree>>),
    QuotedAtom(::std::boxed::Box<QuotedAtom<'tree>>),
    Sigil(::std::boxed::Box<Sigil<'tree>>),
    String(::std::boxed::Box<String<'tree>>),
    Tuple(::std::boxed::Box<Tuple<'tree>>),
    UnaryOperator(::std::boxed::Box<UnaryOperator<'tree>>),
}
impl<'tree> ::treesitter_types::FromNode<'tree> for InterpolationChildren<'tree> {
    #[allow(clippy::collapsible_else_if)]
    fn from_node(
        node: ::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        match node.kind() {
            "access_call" => Ok(Self::AccessCall(::std::boxed::Box::new(
                <AccessCall as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "alias" => Ok(Self::Alias(::std::boxed::Box::new(
                <Alias as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "anonymous_function" => Ok(Self::AnonymousFunction(::std::boxed::Box::new(
                <AnonymousFunction as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "atom" => Ok(Self::Atom(::std::boxed::Box::new(
                <Atom as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "binary_operator" => Ok(Self::BinaryOperator(::std::boxed::Box::new(
                <BinaryOperator as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "bitstring" => Ok(Self::Bitstring(::std::boxed::Box::new(
                <Bitstring as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "block" => Ok(Self::Block(::std::boxed::Box::new(
                <Block as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "boolean" => Ok(Self::Boolean(::std::boxed::Box::new(
                <Boolean as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "call" => Ok(Self::Call(::std::boxed::Box::new(
                <Call as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "char" => Ok(Self::Char(::std::boxed::Box::new(
                <Char as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "charlist" => Ok(Self::Charlist(::std::boxed::Box::new(
                <Charlist as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "dot" => Ok(Self::Dot(::std::boxed::Box::new(
                <Dot as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "float" => Ok(Self::Float(::std::boxed::Box::new(
                <Float as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "identifier" => Ok(Self::Identifier(::std::boxed::Box::new(
                <Identifier as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "integer" => Ok(Self::Integer(::std::boxed::Box::new(
                <Integer as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "list" => Ok(Self::List(::std::boxed::Box::new(
                <List as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "map" => Ok(Self::Map(::std::boxed::Box::new(
                <Map as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "nil" => Ok(Self::Nil(::std::boxed::Box::new(
                <Nil as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "operator_identifier" => Ok(Self::OperatorIdentifier(::std::boxed::Box::new(
                <OperatorIdentifier as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "quoted_atom" => Ok(Self::QuotedAtom(::std::boxed::Box::new(
                <QuotedAtom as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "sigil" => Ok(Self::Sigil(::std::boxed::Box::new(
                <Sigil as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "string" => Ok(Self::String(::std::boxed::Box::new(
                <String as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "tuple" => Ok(Self::Tuple(::std::boxed::Box::new(
                <Tuple as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "unary_operator" => Ok(Self::UnaryOperator(::std::boxed::Box::new(
                <UnaryOperator as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            other => Err(::treesitter_types::ParseError::unexpected_kind(other, node)),
        }
    }
}
impl ::treesitter_types::Spanned for InterpolationChildren<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        match self {
            Self::AccessCall(inner) => inner.span(),
            Self::Alias(inner) => inner.span(),
            Self::AnonymousFunction(inner) => inner.span(),
            Self::Atom(inner) => inner.span(),
            Self::BinaryOperator(inner) => inner.span(),
            Self::Bitstring(inner) => inner.span(),
            Self::Block(inner) => inner.span(),
            Self::Boolean(inner) => inner.span(),
            Self::Call(inner) => inner.span(),
            Self::Char(inner) => inner.span(),
            Self::Charlist(inner) => inner.span(),
            Self::Dot(inner) => inner.span(),
            Self::Float(inner) => inner.span(),
            Self::Identifier(inner) => inner.span(),
            Self::Integer(inner) => inner.span(),
            Self::List(inner) => inner.span(),
            Self::Map(inner) => inner.span(),
            Self::Nil(inner) => inner.span(),
            Self::OperatorIdentifier(inner) => inner.span(),
            Self::QuotedAtom(inner) => inner.span(),
            Self::Sigil(inner) => inner.span(),
            Self::String(inner) => inner.span(),
            Self::Tuple(inner) => inner.span(),
            Self::UnaryOperator(inner) => inner.span(),
        }
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ListChildren<'tree> {
    AccessCall(::std::boxed::Box<AccessCall<'tree>>),
    Alias(::std::boxed::Box<Alias<'tree>>),
    AnonymousFunction(::std::boxed::Box<AnonymousFunction<'tree>>),
    Atom(::std::boxed::Box<Atom<'tree>>),
    BinaryOperator(::std::boxed::Box<BinaryOperator<'tree>>),
    Bitstring(::std::boxed::Box<Bitstring<'tree>>),
    Block(::std::boxed::Box<Block<'tree>>),
    Boolean(::std::boxed::Box<Boolean<'tree>>),
    Call(::std::boxed::Box<Call<'tree>>),
    Char(::std::boxed::Box<Char<'tree>>),
    Charlist(::std::boxed::Box<Charlist<'tree>>),
    Dot(::std::boxed::Box<Dot<'tree>>),
    Float(::std::boxed::Box<Float<'tree>>),
    Identifier(::std::boxed::Box<Identifier<'tree>>),
    Integer(::std::boxed::Box<Integer<'tree>>),
    Keywords(::std::boxed::Box<Keywords<'tree>>),
    List(::std::boxed::Box<List<'tree>>),
    Map(::std::boxed::Box<Map<'tree>>),
    Nil(::std::boxed::Box<Nil<'tree>>),
    OperatorIdentifier(::std::boxed::Box<OperatorIdentifier<'tree>>),
    QuotedAtom(::std::boxed::Box<QuotedAtom<'tree>>),
    Sigil(::std::boxed::Box<Sigil<'tree>>),
    String(::std::boxed::Box<String<'tree>>),
    Tuple(::std::boxed::Box<Tuple<'tree>>),
    UnaryOperator(::std::boxed::Box<UnaryOperator<'tree>>),
}
impl<'tree> ::treesitter_types::FromNode<'tree> for ListChildren<'tree> {
    #[allow(clippy::collapsible_else_if)]
    fn from_node(
        node: ::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        match node.kind() {
            "access_call" => Ok(Self::AccessCall(::std::boxed::Box::new(
                <AccessCall as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "alias" => Ok(Self::Alias(::std::boxed::Box::new(
                <Alias as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "anonymous_function" => Ok(Self::AnonymousFunction(::std::boxed::Box::new(
                <AnonymousFunction as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "atom" => Ok(Self::Atom(::std::boxed::Box::new(
                <Atom as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "binary_operator" => Ok(Self::BinaryOperator(::std::boxed::Box::new(
                <BinaryOperator as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "bitstring" => Ok(Self::Bitstring(::std::boxed::Box::new(
                <Bitstring as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "block" => Ok(Self::Block(::std::boxed::Box::new(
                <Block as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "boolean" => Ok(Self::Boolean(::std::boxed::Box::new(
                <Boolean as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "call" => Ok(Self::Call(::std::boxed::Box::new(
                <Call as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "char" => Ok(Self::Char(::std::boxed::Box::new(
                <Char as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "charlist" => Ok(Self::Charlist(::std::boxed::Box::new(
                <Charlist as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "dot" => Ok(Self::Dot(::std::boxed::Box::new(
                <Dot as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "float" => Ok(Self::Float(::std::boxed::Box::new(
                <Float as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "identifier" => Ok(Self::Identifier(::std::boxed::Box::new(
                <Identifier as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "integer" => Ok(Self::Integer(::std::boxed::Box::new(
                <Integer as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "keywords" => Ok(Self::Keywords(::std::boxed::Box::new(
                <Keywords as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "list" => Ok(Self::List(::std::boxed::Box::new(
                <List as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "map" => Ok(Self::Map(::std::boxed::Box::new(
                <Map as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "nil" => Ok(Self::Nil(::std::boxed::Box::new(
                <Nil as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "operator_identifier" => Ok(Self::OperatorIdentifier(::std::boxed::Box::new(
                <OperatorIdentifier as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "quoted_atom" => Ok(Self::QuotedAtom(::std::boxed::Box::new(
                <QuotedAtom as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "sigil" => Ok(Self::Sigil(::std::boxed::Box::new(
                <Sigil as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "string" => Ok(Self::String(::std::boxed::Box::new(
                <String as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "tuple" => Ok(Self::Tuple(::std::boxed::Box::new(
                <Tuple as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "unary_operator" => Ok(Self::UnaryOperator(::std::boxed::Box::new(
                <UnaryOperator as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            other => Err(::treesitter_types::ParseError::unexpected_kind(other, node)),
        }
    }
}
impl ::treesitter_types::Spanned for ListChildren<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        match self {
            Self::AccessCall(inner) => inner.span(),
            Self::Alias(inner) => inner.span(),
            Self::AnonymousFunction(inner) => inner.span(),
            Self::Atom(inner) => inner.span(),
            Self::BinaryOperator(inner) => inner.span(),
            Self::Bitstring(inner) => inner.span(),
            Self::Block(inner) => inner.span(),
            Self::Boolean(inner) => inner.span(),
            Self::Call(inner) => inner.span(),
            Self::Char(inner) => inner.span(),
            Self::Charlist(inner) => inner.span(),
            Self::Dot(inner) => inner.span(),
            Self::Float(inner) => inner.span(),
            Self::Identifier(inner) => inner.span(),
            Self::Integer(inner) => inner.span(),
            Self::Keywords(inner) => inner.span(),
            Self::List(inner) => inner.span(),
            Self::Map(inner) => inner.span(),
            Self::Nil(inner) => inner.span(),
            Self::OperatorIdentifier(inner) => inner.span(),
            Self::QuotedAtom(inner) => inner.span(),
            Self::Sigil(inner) => inner.span(),
            Self::String(inner) => inner.span(),
            Self::Tuple(inner) => inner.span(),
            Self::UnaryOperator(inner) => inner.span(),
        }
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum MapChildren<'tree> {
    MapContent(::std::boxed::Box<MapContent<'tree>>),
    Struct(::std::boxed::Box<Struct<'tree>>),
}
impl<'tree> ::treesitter_types::FromNode<'tree> for MapChildren<'tree> {
    #[allow(clippy::collapsible_else_if)]
    fn from_node(
        node: ::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        match node.kind() {
            "map_content" => Ok(Self::MapContent(::std::boxed::Box::new(
                <MapContent as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "struct" => Ok(Self::Struct(::std::boxed::Box::new(
                <Struct as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            other => Err(::treesitter_types::ParseError::unexpected_kind(other, node)),
        }
    }
}
impl ::treesitter_types::Spanned for MapChildren<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        match self {
            Self::MapContent(inner) => inner.span(),
            Self::Struct(inner) => inner.span(),
        }
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum MapContentChildren<'tree> {
    AccessCall(::std::boxed::Box<AccessCall<'tree>>),
    Alias(::std::boxed::Box<Alias<'tree>>),
    AnonymousFunction(::std::boxed::Box<AnonymousFunction<'tree>>),
    Atom(::std::boxed::Box<Atom<'tree>>),
    BinaryOperator(::std::boxed::Box<BinaryOperator<'tree>>),
    Bitstring(::std::boxed::Box<Bitstring<'tree>>),
    Block(::std::boxed::Box<Block<'tree>>),
    Boolean(::std::boxed::Box<Boolean<'tree>>),
    Call(::std::boxed::Box<Call<'tree>>),
    Char(::std::boxed::Box<Char<'tree>>),
    Charlist(::std::boxed::Box<Charlist<'tree>>),
    Dot(::std::boxed::Box<Dot<'tree>>),
    Float(::std::boxed::Box<Float<'tree>>),
    Identifier(::std::boxed::Box<Identifier<'tree>>),
    Integer(::std::boxed::Box<Integer<'tree>>),
    Keywords(::std::boxed::Box<Keywords<'tree>>),
    List(::std::boxed::Box<List<'tree>>),
    Map(::std::boxed::Box<Map<'tree>>),
    Nil(::std::boxed::Box<Nil<'tree>>),
    OperatorIdentifier(::std::boxed::Box<OperatorIdentifier<'tree>>),
    QuotedAtom(::std::boxed::Box<QuotedAtom<'tree>>),
    Sigil(::std::boxed::Box<Sigil<'tree>>),
    String(::std::boxed::Box<String<'tree>>),
    Tuple(::std::boxed::Box<Tuple<'tree>>),
    UnaryOperator(::std::boxed::Box<UnaryOperator<'tree>>),
}
impl<'tree> ::treesitter_types::FromNode<'tree> for MapContentChildren<'tree> {
    #[allow(clippy::collapsible_else_if)]
    fn from_node(
        node: ::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        match node.kind() {
            "access_call" => Ok(Self::AccessCall(::std::boxed::Box::new(
                <AccessCall as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "alias" => Ok(Self::Alias(::std::boxed::Box::new(
                <Alias as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "anonymous_function" => Ok(Self::AnonymousFunction(::std::boxed::Box::new(
                <AnonymousFunction as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "atom" => Ok(Self::Atom(::std::boxed::Box::new(
                <Atom as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "binary_operator" => Ok(Self::BinaryOperator(::std::boxed::Box::new(
                <BinaryOperator as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "bitstring" => Ok(Self::Bitstring(::std::boxed::Box::new(
                <Bitstring as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "block" => Ok(Self::Block(::std::boxed::Box::new(
                <Block as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "boolean" => Ok(Self::Boolean(::std::boxed::Box::new(
                <Boolean as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "call" => Ok(Self::Call(::std::boxed::Box::new(
                <Call as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "char" => Ok(Self::Char(::std::boxed::Box::new(
                <Char as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "charlist" => Ok(Self::Charlist(::std::boxed::Box::new(
                <Charlist as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "dot" => Ok(Self::Dot(::std::boxed::Box::new(
                <Dot as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "float" => Ok(Self::Float(::std::boxed::Box::new(
                <Float as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "identifier" => Ok(Self::Identifier(::std::boxed::Box::new(
                <Identifier as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "integer" => Ok(Self::Integer(::std::boxed::Box::new(
                <Integer as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "keywords" => Ok(Self::Keywords(::std::boxed::Box::new(
                <Keywords as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "list" => Ok(Self::List(::std::boxed::Box::new(
                <List as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "map" => Ok(Self::Map(::std::boxed::Box::new(
                <Map as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "nil" => Ok(Self::Nil(::std::boxed::Box::new(
                <Nil as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "operator_identifier" => Ok(Self::OperatorIdentifier(::std::boxed::Box::new(
                <OperatorIdentifier as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "quoted_atom" => Ok(Self::QuotedAtom(::std::boxed::Box::new(
                <QuotedAtom as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "sigil" => Ok(Self::Sigil(::std::boxed::Box::new(
                <Sigil as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "string" => Ok(Self::String(::std::boxed::Box::new(
                <String as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "tuple" => Ok(Self::Tuple(::std::boxed::Box::new(
                <Tuple as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "unary_operator" => Ok(Self::UnaryOperator(::std::boxed::Box::new(
                <UnaryOperator as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            other => Err(::treesitter_types::ParseError::unexpected_kind(other, node)),
        }
    }
}
impl ::treesitter_types::Spanned for MapContentChildren<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        match self {
            Self::AccessCall(inner) => inner.span(),
            Self::Alias(inner) => inner.span(),
            Self::AnonymousFunction(inner) => inner.span(),
            Self::Atom(inner) => inner.span(),
            Self::BinaryOperator(inner) => inner.span(),
            Self::Bitstring(inner) => inner.span(),
            Self::Block(inner) => inner.span(),
            Self::Boolean(inner) => inner.span(),
            Self::Call(inner) => inner.span(),
            Self::Char(inner) => inner.span(),
            Self::Charlist(inner) => inner.span(),
            Self::Dot(inner) => inner.span(),
            Self::Float(inner) => inner.span(),
            Self::Identifier(inner) => inner.span(),
            Self::Integer(inner) => inner.span(),
            Self::Keywords(inner) => inner.span(),
            Self::List(inner) => inner.span(),
            Self::Map(inner) => inner.span(),
            Self::Nil(inner) => inner.span(),
            Self::OperatorIdentifier(inner) => inner.span(),
            Self::QuotedAtom(inner) => inner.span(),
            Self::Sigil(inner) => inner.span(),
            Self::String(inner) => inner.span(),
            Self::Tuple(inner) => inner.span(),
            Self::UnaryOperator(inner) => inner.span(),
        }
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum PairKey<'tree> {
    Keyword(::std::boxed::Box<Keyword<'tree>>),
    QuotedKeyword(::std::boxed::Box<QuotedKeyword<'tree>>),
}
impl<'tree> ::treesitter_types::FromNode<'tree> for PairKey<'tree> {
    #[allow(clippy::collapsible_else_if)]
    fn from_node(
        node: ::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        match node.kind() {
            "keyword" => Ok(Self::Keyword(::std::boxed::Box::new(
                <Keyword as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "quoted_keyword" => Ok(Self::QuotedKeyword(::std::boxed::Box::new(
                <QuotedKeyword as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            other => Err(::treesitter_types::ParseError::unexpected_kind(other, node)),
        }
    }
}
impl ::treesitter_types::Spanned for PairKey<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        match self {
            Self::Keyword(inner) => inner.span(),
            Self::QuotedKeyword(inner) => inner.span(),
        }
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum PairValue<'tree> {
    AccessCall(::std::boxed::Box<AccessCall<'tree>>),
    Alias(::std::boxed::Box<Alias<'tree>>),
    AnonymousFunction(::std::boxed::Box<AnonymousFunction<'tree>>),
    Atom(::std::boxed::Box<Atom<'tree>>),
    BinaryOperator(::std::boxed::Box<BinaryOperator<'tree>>),
    Bitstring(::std::boxed::Box<Bitstring<'tree>>),
    Block(::std::boxed::Box<Block<'tree>>),
    Boolean(::std::boxed::Box<Boolean<'tree>>),
    Call(::std::boxed::Box<Call<'tree>>),
    Char(::std::boxed::Box<Char<'tree>>),
    Charlist(::std::boxed::Box<Charlist<'tree>>),
    Dot(::std::boxed::Box<Dot<'tree>>),
    Float(::std::boxed::Box<Float<'tree>>),
    Identifier(::std::boxed::Box<Identifier<'tree>>),
    Integer(::std::boxed::Box<Integer<'tree>>),
    List(::std::boxed::Box<List<'tree>>),
    Map(::std::boxed::Box<Map<'tree>>),
    Nil(::std::boxed::Box<Nil<'tree>>),
    OperatorIdentifier(::std::boxed::Box<OperatorIdentifier<'tree>>),
    QuotedAtom(::std::boxed::Box<QuotedAtom<'tree>>),
    Sigil(::std::boxed::Box<Sigil<'tree>>),
    String(::std::boxed::Box<String<'tree>>),
    Tuple(::std::boxed::Box<Tuple<'tree>>),
    UnaryOperator(::std::boxed::Box<UnaryOperator<'tree>>),
}
impl<'tree> ::treesitter_types::FromNode<'tree> for PairValue<'tree> {
    #[allow(clippy::collapsible_else_if)]
    fn from_node(
        node: ::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        match node.kind() {
            "access_call" => Ok(Self::AccessCall(::std::boxed::Box::new(
                <AccessCall as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "alias" => Ok(Self::Alias(::std::boxed::Box::new(
                <Alias as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "anonymous_function" => Ok(Self::AnonymousFunction(::std::boxed::Box::new(
                <AnonymousFunction as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "atom" => Ok(Self::Atom(::std::boxed::Box::new(
                <Atom as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "binary_operator" => Ok(Self::BinaryOperator(::std::boxed::Box::new(
                <BinaryOperator as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "bitstring" => Ok(Self::Bitstring(::std::boxed::Box::new(
                <Bitstring as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "block" => Ok(Self::Block(::std::boxed::Box::new(
                <Block as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "boolean" => Ok(Self::Boolean(::std::boxed::Box::new(
                <Boolean as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "call" => Ok(Self::Call(::std::boxed::Box::new(
                <Call as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "char" => Ok(Self::Char(::std::boxed::Box::new(
                <Char as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "charlist" => Ok(Self::Charlist(::std::boxed::Box::new(
                <Charlist as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "dot" => Ok(Self::Dot(::std::boxed::Box::new(
                <Dot as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "float" => Ok(Self::Float(::std::boxed::Box::new(
                <Float as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "identifier" => Ok(Self::Identifier(::std::boxed::Box::new(
                <Identifier as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "integer" => Ok(Self::Integer(::std::boxed::Box::new(
                <Integer as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "list" => Ok(Self::List(::std::boxed::Box::new(
                <List as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "map" => Ok(Self::Map(::std::boxed::Box::new(
                <Map as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "nil" => Ok(Self::Nil(::std::boxed::Box::new(
                <Nil as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "operator_identifier" => Ok(Self::OperatorIdentifier(::std::boxed::Box::new(
                <OperatorIdentifier as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "quoted_atom" => Ok(Self::QuotedAtom(::std::boxed::Box::new(
                <QuotedAtom as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "sigil" => Ok(Self::Sigil(::std::boxed::Box::new(
                <Sigil as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "string" => Ok(Self::String(::std::boxed::Box::new(
                <String as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "tuple" => Ok(Self::Tuple(::std::boxed::Box::new(
                <Tuple as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "unary_operator" => Ok(Self::UnaryOperator(::std::boxed::Box::new(
                <UnaryOperator as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            other => Err(::treesitter_types::ParseError::unexpected_kind(other, node)),
        }
    }
}
impl ::treesitter_types::Spanned for PairValue<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        match self {
            Self::AccessCall(inner) => inner.span(),
            Self::Alias(inner) => inner.span(),
            Self::AnonymousFunction(inner) => inner.span(),
            Self::Atom(inner) => inner.span(),
            Self::BinaryOperator(inner) => inner.span(),
            Self::Bitstring(inner) => inner.span(),
            Self::Block(inner) => inner.span(),
            Self::Boolean(inner) => inner.span(),
            Self::Call(inner) => inner.span(),
            Self::Char(inner) => inner.span(),
            Self::Charlist(inner) => inner.span(),
            Self::Dot(inner) => inner.span(),
            Self::Float(inner) => inner.span(),
            Self::Identifier(inner) => inner.span(),
            Self::Integer(inner) => inner.span(),
            Self::List(inner) => inner.span(),
            Self::Map(inner) => inner.span(),
            Self::Nil(inner) => inner.span(),
            Self::OperatorIdentifier(inner) => inner.span(),
            Self::QuotedAtom(inner) => inner.span(),
            Self::Sigil(inner) => inner.span(),
            Self::String(inner) => inner.span(),
            Self::Tuple(inner) => inner.span(),
            Self::UnaryOperator(inner) => inner.span(),
        }
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum QuotedAtomQuotedEnd {
    DoubleQuote(::treesitter_types::Span),
    SingleQuote(::treesitter_types::Span),
}
impl<'tree> ::treesitter_types::FromNode<'tree> for QuotedAtomQuotedEnd {
    #[allow(clippy::collapsible_else_if)]
    fn from_node(
        node: ::tree_sitter::Node<'tree>,
        _src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        match node.kind() {
            "\"" => Ok(Self::DoubleQuote(::treesitter_types::Span::from(node))),
            "'" => Ok(Self::SingleQuote(::treesitter_types::Span::from(node))),
            other => Err(::treesitter_types::ParseError::unexpected_kind(other, node)),
        }
    }
}
impl ::treesitter_types::Spanned for QuotedAtomQuotedEnd {
    fn span(&self) -> ::treesitter_types::Span {
        match self {
            Self::DoubleQuote(span) => *span,
            Self::SingleQuote(span) => *span,
        }
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum QuotedAtomQuotedStart {
    DoubleQuote(::treesitter_types::Span),
    SingleQuote(::treesitter_types::Span),
}
impl<'tree> ::treesitter_types::FromNode<'tree> for QuotedAtomQuotedStart {
    #[allow(clippy::collapsible_else_if)]
    fn from_node(
        node: ::tree_sitter::Node<'tree>,
        _src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        match node.kind() {
            "\"" => Ok(Self::DoubleQuote(::treesitter_types::Span::from(node))),
            "'" => Ok(Self::SingleQuote(::treesitter_types::Span::from(node))),
            other => Err(::treesitter_types::ParseError::unexpected_kind(other, node)),
        }
    }
}
impl ::treesitter_types::Spanned for QuotedAtomQuotedStart {
    fn span(&self) -> ::treesitter_types::Span {
        match self {
            Self::DoubleQuote(span) => *span,
            Self::SingleQuote(span) => *span,
        }
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum QuotedAtomChildren<'tree> {
    EscapeSequence(::std::boxed::Box<EscapeSequence<'tree>>),
    Interpolation(::std::boxed::Box<Interpolation<'tree>>),
    QuotedContent(::std::boxed::Box<QuotedContent<'tree>>),
}
impl<'tree> ::treesitter_types::FromNode<'tree> for QuotedAtomChildren<'tree> {
    #[allow(clippy::collapsible_else_if)]
    fn from_node(
        node: ::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        match node.kind() {
            "escape_sequence" => Ok(Self::EscapeSequence(::std::boxed::Box::new(
                <EscapeSequence as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "interpolation" => Ok(Self::Interpolation(::std::boxed::Box::new(
                <Interpolation as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "quoted_content" => Ok(Self::QuotedContent(::std::boxed::Box::new(
                <QuotedContent as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            other => Err(::treesitter_types::ParseError::unexpected_kind(other, node)),
        }
    }
}
impl ::treesitter_types::Spanned for QuotedAtomChildren<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        match self {
            Self::EscapeSequence(inner) => inner.span(),
            Self::Interpolation(inner) => inner.span(),
            Self::QuotedContent(inner) => inner.span(),
        }
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum QuotedKeywordQuotedEnd {
    DoubleQuote(::treesitter_types::Span),
    SingleQuote(::treesitter_types::Span),
}
impl<'tree> ::treesitter_types::FromNode<'tree> for QuotedKeywordQuotedEnd {
    #[allow(clippy::collapsible_else_if)]
    fn from_node(
        node: ::tree_sitter::Node<'tree>,
        _src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        match node.kind() {
            "\"" => Ok(Self::DoubleQuote(::treesitter_types::Span::from(node))),
            "'" => Ok(Self::SingleQuote(::treesitter_types::Span::from(node))),
            other => Err(::treesitter_types::ParseError::unexpected_kind(other, node)),
        }
    }
}
impl ::treesitter_types::Spanned for QuotedKeywordQuotedEnd {
    fn span(&self) -> ::treesitter_types::Span {
        match self {
            Self::DoubleQuote(span) => *span,
            Self::SingleQuote(span) => *span,
        }
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum QuotedKeywordQuotedStart {
    DoubleQuote(::treesitter_types::Span),
    SingleQuote(::treesitter_types::Span),
}
impl<'tree> ::treesitter_types::FromNode<'tree> for QuotedKeywordQuotedStart {
    #[allow(clippy::collapsible_else_if)]
    fn from_node(
        node: ::tree_sitter::Node<'tree>,
        _src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        match node.kind() {
            "\"" => Ok(Self::DoubleQuote(::treesitter_types::Span::from(node))),
            "'" => Ok(Self::SingleQuote(::treesitter_types::Span::from(node))),
            other => Err(::treesitter_types::ParseError::unexpected_kind(other, node)),
        }
    }
}
impl ::treesitter_types::Spanned for QuotedKeywordQuotedStart {
    fn span(&self) -> ::treesitter_types::Span {
        match self {
            Self::DoubleQuote(span) => *span,
            Self::SingleQuote(span) => *span,
        }
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum QuotedKeywordChildren<'tree> {
    EscapeSequence(::std::boxed::Box<EscapeSequence<'tree>>),
    Interpolation(::std::boxed::Box<Interpolation<'tree>>),
    QuotedContent(::std::boxed::Box<QuotedContent<'tree>>),
}
impl<'tree> ::treesitter_types::FromNode<'tree> for QuotedKeywordChildren<'tree> {
    #[allow(clippy::collapsible_else_if)]
    fn from_node(
        node: ::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        match node.kind() {
            "escape_sequence" => Ok(Self::EscapeSequence(::std::boxed::Box::new(
                <EscapeSequence as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "interpolation" => Ok(Self::Interpolation(::std::boxed::Box::new(
                <Interpolation as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "quoted_content" => Ok(Self::QuotedContent(::std::boxed::Box::new(
                <QuotedContent as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            other => Err(::treesitter_types::ParseError::unexpected_kind(other, node)),
        }
    }
}
impl ::treesitter_types::Spanned for QuotedKeywordChildren<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        match self {
            Self::EscapeSequence(inner) => inner.span(),
            Self::Interpolation(inner) => inner.span(),
            Self::QuotedContent(inner) => inner.span(),
        }
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum RescueBlockChildren<'tree> {
    AccessCall(::std::boxed::Box<AccessCall<'tree>>),
    Alias(::std::boxed::Box<Alias<'tree>>),
    AnonymousFunction(::std::boxed::Box<AnonymousFunction<'tree>>),
    Atom(::std::boxed::Box<Atom<'tree>>),
    BinaryOperator(::std::boxed::Box<BinaryOperator<'tree>>),
    Bitstring(::std::boxed::Box<Bitstring<'tree>>),
    Block(::std::boxed::Box<Block<'tree>>),
    Boolean(::std::boxed::Box<Boolean<'tree>>),
    Call(::std::boxed::Box<Call<'tree>>),
    Char(::std::boxed::Box<Char<'tree>>),
    Charlist(::std::boxed::Box<Charlist<'tree>>),
    Dot(::std::boxed::Box<Dot<'tree>>),
    Float(::std::boxed::Box<Float<'tree>>),
    Identifier(::std::boxed::Box<Identifier<'tree>>),
    Integer(::std::boxed::Box<Integer<'tree>>),
    List(::std::boxed::Box<List<'tree>>),
    Map(::std::boxed::Box<Map<'tree>>),
    Nil(::std::boxed::Box<Nil<'tree>>),
    OperatorIdentifier(::std::boxed::Box<OperatorIdentifier<'tree>>),
    QuotedAtom(::std::boxed::Box<QuotedAtom<'tree>>),
    Sigil(::std::boxed::Box<Sigil<'tree>>),
    StabClause(::std::boxed::Box<StabClause<'tree>>),
    String(::std::boxed::Box<String<'tree>>),
    Tuple(::std::boxed::Box<Tuple<'tree>>),
    UnaryOperator(::std::boxed::Box<UnaryOperator<'tree>>),
}
impl<'tree> ::treesitter_types::FromNode<'tree> for RescueBlockChildren<'tree> {
    #[allow(clippy::collapsible_else_if)]
    fn from_node(
        node: ::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        match node.kind() {
            "access_call" => Ok(Self::AccessCall(::std::boxed::Box::new(
                <AccessCall as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "alias" => Ok(Self::Alias(::std::boxed::Box::new(
                <Alias as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "anonymous_function" => Ok(Self::AnonymousFunction(::std::boxed::Box::new(
                <AnonymousFunction as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "atom" => Ok(Self::Atom(::std::boxed::Box::new(
                <Atom as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "binary_operator" => Ok(Self::BinaryOperator(::std::boxed::Box::new(
                <BinaryOperator as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "bitstring" => Ok(Self::Bitstring(::std::boxed::Box::new(
                <Bitstring as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "block" => Ok(Self::Block(::std::boxed::Box::new(
                <Block as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "boolean" => Ok(Self::Boolean(::std::boxed::Box::new(
                <Boolean as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "call" => Ok(Self::Call(::std::boxed::Box::new(
                <Call as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "char" => Ok(Self::Char(::std::boxed::Box::new(
                <Char as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "charlist" => Ok(Self::Charlist(::std::boxed::Box::new(
                <Charlist as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "dot" => Ok(Self::Dot(::std::boxed::Box::new(
                <Dot as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "float" => Ok(Self::Float(::std::boxed::Box::new(
                <Float as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "identifier" => Ok(Self::Identifier(::std::boxed::Box::new(
                <Identifier as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "integer" => Ok(Self::Integer(::std::boxed::Box::new(
                <Integer as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "list" => Ok(Self::List(::std::boxed::Box::new(
                <List as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "map" => Ok(Self::Map(::std::boxed::Box::new(
                <Map as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "nil" => Ok(Self::Nil(::std::boxed::Box::new(
                <Nil as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "operator_identifier" => Ok(Self::OperatorIdentifier(::std::boxed::Box::new(
                <OperatorIdentifier as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "quoted_atom" => Ok(Self::QuotedAtom(::std::boxed::Box::new(
                <QuotedAtom as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "sigil" => Ok(Self::Sigil(::std::boxed::Box::new(
                <Sigil as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "stab_clause" => Ok(Self::StabClause(::std::boxed::Box::new(
                <StabClause as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "string" => Ok(Self::String(::std::boxed::Box::new(
                <String as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "tuple" => Ok(Self::Tuple(::std::boxed::Box::new(
                <Tuple as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "unary_operator" => Ok(Self::UnaryOperator(::std::boxed::Box::new(
                <UnaryOperator as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            other => Err(::treesitter_types::ParseError::unexpected_kind(other, node)),
        }
    }
}
impl ::treesitter_types::Spanned for RescueBlockChildren<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        match self {
            Self::AccessCall(inner) => inner.span(),
            Self::Alias(inner) => inner.span(),
            Self::AnonymousFunction(inner) => inner.span(),
            Self::Atom(inner) => inner.span(),
            Self::BinaryOperator(inner) => inner.span(),
            Self::Bitstring(inner) => inner.span(),
            Self::Block(inner) => inner.span(),
            Self::Boolean(inner) => inner.span(),
            Self::Call(inner) => inner.span(),
            Self::Char(inner) => inner.span(),
            Self::Charlist(inner) => inner.span(),
            Self::Dot(inner) => inner.span(),
            Self::Float(inner) => inner.span(),
            Self::Identifier(inner) => inner.span(),
            Self::Integer(inner) => inner.span(),
            Self::List(inner) => inner.span(),
            Self::Map(inner) => inner.span(),
            Self::Nil(inner) => inner.span(),
            Self::OperatorIdentifier(inner) => inner.span(),
            Self::QuotedAtom(inner) => inner.span(),
            Self::Sigil(inner) => inner.span(),
            Self::StabClause(inner) => inner.span(),
            Self::String(inner) => inner.span(),
            Self::Tuple(inner) => inner.span(),
            Self::UnaryOperator(inner) => inner.span(),
        }
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum SigilQuotedEnd {
    DoubleQuote(::treesitter_types::Span),
    DQuoteDQuoteDQuote(::treesitter_types::Span),
    SingleQuote(::treesitter_types::Span),
    SQuoteSQuoteSQuote(::treesitter_types::Span),
    RParen(::treesitter_types::Span),
    Slash(::treesitter_types::Span),
    Gt(::treesitter_types::Span),
    RBracket(::treesitter_types::Span),
    Pipe(::treesitter_types::Span),
    RBrace(::treesitter_types::Span),
}
impl<'tree> ::treesitter_types::FromNode<'tree> for SigilQuotedEnd {
    #[allow(clippy::collapsible_else_if)]
    fn from_node(
        node: ::tree_sitter::Node<'tree>,
        _src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        match node.kind() {
            "\"" => Ok(Self::DoubleQuote(::treesitter_types::Span::from(node))),
            "\"\"\"" => Ok(Self::DQuoteDQuoteDQuote(::treesitter_types::Span::from(
                node,
            ))),
            "'" => Ok(Self::SingleQuote(::treesitter_types::Span::from(node))),
            "'''" => Ok(Self::SQuoteSQuoteSQuote(::treesitter_types::Span::from(
                node,
            ))),
            ")" => Ok(Self::RParen(::treesitter_types::Span::from(node))),
            "/" => Ok(Self::Slash(::treesitter_types::Span::from(node))),
            ">" => Ok(Self::Gt(::treesitter_types::Span::from(node))),
            "]" => Ok(Self::RBracket(::treesitter_types::Span::from(node))),
            "|" => Ok(Self::Pipe(::treesitter_types::Span::from(node))),
            "}" => Ok(Self::RBrace(::treesitter_types::Span::from(node))),
            other => Err(::treesitter_types::ParseError::unexpected_kind(other, node)),
        }
    }
}
impl ::treesitter_types::Spanned for SigilQuotedEnd {
    fn span(&self) -> ::treesitter_types::Span {
        match self {
            Self::DoubleQuote(span) => *span,
            Self::DQuoteDQuoteDQuote(span) => *span,
            Self::SingleQuote(span) => *span,
            Self::SQuoteSQuoteSQuote(span) => *span,
            Self::RParen(span) => *span,
            Self::Slash(span) => *span,
            Self::Gt(span) => *span,
            Self::RBracket(span) => *span,
            Self::Pipe(span) => *span,
            Self::RBrace(span) => *span,
        }
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum SigilQuotedStart {
    DoubleQuote(::treesitter_types::Span),
    DQuoteDQuoteDQuote(::treesitter_types::Span),
    SingleQuote(::treesitter_types::Span),
    SQuoteSQuoteSQuote(::treesitter_types::Span),
    LParen(::treesitter_types::Span),
    Slash(::treesitter_types::Span),
    Lt(::treesitter_types::Span),
    LBracket(::treesitter_types::Span),
    LBrace(::treesitter_types::Span),
    Pipe(::treesitter_types::Span),
}
impl<'tree> ::treesitter_types::FromNode<'tree> for SigilQuotedStart {
    #[allow(clippy::collapsible_else_if)]
    fn from_node(
        node: ::tree_sitter::Node<'tree>,
        _src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        match node.kind() {
            "\"" => Ok(Self::DoubleQuote(::treesitter_types::Span::from(node))),
            "\"\"\"" => Ok(Self::DQuoteDQuoteDQuote(::treesitter_types::Span::from(
                node,
            ))),
            "'" => Ok(Self::SingleQuote(::treesitter_types::Span::from(node))),
            "'''" => Ok(Self::SQuoteSQuoteSQuote(::treesitter_types::Span::from(
                node,
            ))),
            "(" => Ok(Self::LParen(::treesitter_types::Span::from(node))),
            "/" => Ok(Self::Slash(::treesitter_types::Span::from(node))),
            "<" => Ok(Self::Lt(::treesitter_types::Span::from(node))),
            "[" => Ok(Self::LBracket(::treesitter_types::Span::from(node))),
            "{" => Ok(Self::LBrace(::treesitter_types::Span::from(node))),
            "|" => Ok(Self::Pipe(::treesitter_types::Span::from(node))),
            other => Err(::treesitter_types::ParseError::unexpected_kind(other, node)),
        }
    }
}
impl ::treesitter_types::Spanned for SigilQuotedStart {
    fn span(&self) -> ::treesitter_types::Span {
        match self {
            Self::DoubleQuote(span) => *span,
            Self::DQuoteDQuoteDQuote(span) => *span,
            Self::SingleQuote(span) => *span,
            Self::SQuoteSQuoteSQuote(span) => *span,
            Self::LParen(span) => *span,
            Self::Slash(span) => *span,
            Self::Lt(span) => *span,
            Self::LBracket(span) => *span,
            Self::LBrace(span) => *span,
            Self::Pipe(span) => *span,
        }
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum SigilChildren<'tree> {
    EscapeSequence(::std::boxed::Box<EscapeSequence<'tree>>),
    Interpolation(::std::boxed::Box<Interpolation<'tree>>),
    QuotedContent(::std::boxed::Box<QuotedContent<'tree>>),
    SigilModifiers(::std::boxed::Box<SigilModifiers<'tree>>),
    SigilName(::std::boxed::Box<SigilName<'tree>>),
}
impl<'tree> ::treesitter_types::FromNode<'tree> for SigilChildren<'tree> {
    #[allow(clippy::collapsible_else_if)]
    fn from_node(
        node: ::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        match node.kind() {
            "escape_sequence" => Ok(Self::EscapeSequence(::std::boxed::Box::new(
                <EscapeSequence as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "interpolation" => Ok(Self::Interpolation(::std::boxed::Box::new(
                <Interpolation as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "quoted_content" => Ok(Self::QuotedContent(::std::boxed::Box::new(
                <QuotedContent as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "sigil_modifiers" => Ok(Self::SigilModifiers(::std::boxed::Box::new(
                <SigilModifiers as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "sigil_name" => Ok(Self::SigilName(::std::boxed::Box::new(
                <SigilName as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            other => Err(::treesitter_types::ParseError::unexpected_kind(other, node)),
        }
    }
}
impl ::treesitter_types::Spanned for SigilChildren<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        match self {
            Self::EscapeSequence(inner) => inner.span(),
            Self::Interpolation(inner) => inner.span(),
            Self::QuotedContent(inner) => inner.span(),
            Self::SigilModifiers(inner) => inner.span(),
            Self::SigilName(inner) => inner.span(),
        }
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum SourceChildren<'tree> {
    AccessCall(::std::boxed::Box<AccessCall<'tree>>),
    Alias(::std::boxed::Box<Alias<'tree>>),
    AnonymousFunction(::std::boxed::Box<AnonymousFunction<'tree>>),
    Atom(::std::boxed::Box<Atom<'tree>>),
    BinaryOperator(::std::boxed::Box<BinaryOperator<'tree>>),
    Bitstring(::std::boxed::Box<Bitstring<'tree>>),
    Block(::std::boxed::Box<Block<'tree>>),
    Boolean(::std::boxed::Box<Boolean<'tree>>),
    Call(::std::boxed::Box<Call<'tree>>),
    Char(::std::boxed::Box<Char<'tree>>),
    Charlist(::std::boxed::Box<Charlist<'tree>>),
    Dot(::std::boxed::Box<Dot<'tree>>),
    Float(::std::boxed::Box<Float<'tree>>),
    Identifier(::std::boxed::Box<Identifier<'tree>>),
    Integer(::std::boxed::Box<Integer<'tree>>),
    List(::std::boxed::Box<List<'tree>>),
    Map(::std::boxed::Box<Map<'tree>>),
    Nil(::std::boxed::Box<Nil<'tree>>),
    OperatorIdentifier(::std::boxed::Box<OperatorIdentifier<'tree>>),
    QuotedAtom(::std::boxed::Box<QuotedAtom<'tree>>),
    Sigil(::std::boxed::Box<Sigil<'tree>>),
    String(::std::boxed::Box<String<'tree>>),
    Tuple(::std::boxed::Box<Tuple<'tree>>),
    UnaryOperator(::std::boxed::Box<UnaryOperator<'tree>>),
}
impl<'tree> ::treesitter_types::FromNode<'tree> for SourceChildren<'tree> {
    #[allow(clippy::collapsible_else_if)]
    fn from_node(
        node: ::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        match node.kind() {
            "access_call" => Ok(Self::AccessCall(::std::boxed::Box::new(
                <AccessCall as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "alias" => Ok(Self::Alias(::std::boxed::Box::new(
                <Alias as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "anonymous_function" => Ok(Self::AnonymousFunction(::std::boxed::Box::new(
                <AnonymousFunction as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "atom" => Ok(Self::Atom(::std::boxed::Box::new(
                <Atom as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "binary_operator" => Ok(Self::BinaryOperator(::std::boxed::Box::new(
                <BinaryOperator as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "bitstring" => Ok(Self::Bitstring(::std::boxed::Box::new(
                <Bitstring as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "block" => Ok(Self::Block(::std::boxed::Box::new(
                <Block as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "boolean" => Ok(Self::Boolean(::std::boxed::Box::new(
                <Boolean as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "call" => Ok(Self::Call(::std::boxed::Box::new(
                <Call as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "char" => Ok(Self::Char(::std::boxed::Box::new(
                <Char as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "charlist" => Ok(Self::Charlist(::std::boxed::Box::new(
                <Charlist as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "dot" => Ok(Self::Dot(::std::boxed::Box::new(
                <Dot as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "float" => Ok(Self::Float(::std::boxed::Box::new(
                <Float as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "identifier" => Ok(Self::Identifier(::std::boxed::Box::new(
                <Identifier as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "integer" => Ok(Self::Integer(::std::boxed::Box::new(
                <Integer as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "list" => Ok(Self::List(::std::boxed::Box::new(
                <List as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "map" => Ok(Self::Map(::std::boxed::Box::new(
                <Map as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "nil" => Ok(Self::Nil(::std::boxed::Box::new(
                <Nil as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "operator_identifier" => Ok(Self::OperatorIdentifier(::std::boxed::Box::new(
                <OperatorIdentifier as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "quoted_atom" => Ok(Self::QuotedAtom(::std::boxed::Box::new(
                <QuotedAtom as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "sigil" => Ok(Self::Sigil(::std::boxed::Box::new(
                <Sigil as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "string" => Ok(Self::String(::std::boxed::Box::new(
                <String as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "tuple" => Ok(Self::Tuple(::std::boxed::Box::new(
                <Tuple as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "unary_operator" => Ok(Self::UnaryOperator(::std::boxed::Box::new(
                <UnaryOperator as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            other => Err(::treesitter_types::ParseError::unexpected_kind(other, node)),
        }
    }
}
impl ::treesitter_types::Spanned for SourceChildren<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        match self {
            Self::AccessCall(inner) => inner.span(),
            Self::Alias(inner) => inner.span(),
            Self::AnonymousFunction(inner) => inner.span(),
            Self::Atom(inner) => inner.span(),
            Self::BinaryOperator(inner) => inner.span(),
            Self::Bitstring(inner) => inner.span(),
            Self::Block(inner) => inner.span(),
            Self::Boolean(inner) => inner.span(),
            Self::Call(inner) => inner.span(),
            Self::Char(inner) => inner.span(),
            Self::Charlist(inner) => inner.span(),
            Self::Dot(inner) => inner.span(),
            Self::Float(inner) => inner.span(),
            Self::Identifier(inner) => inner.span(),
            Self::Integer(inner) => inner.span(),
            Self::List(inner) => inner.span(),
            Self::Map(inner) => inner.span(),
            Self::Nil(inner) => inner.span(),
            Self::OperatorIdentifier(inner) => inner.span(),
            Self::QuotedAtom(inner) => inner.span(),
            Self::Sigil(inner) => inner.span(),
            Self::String(inner) => inner.span(),
            Self::Tuple(inner) => inner.span(),
            Self::UnaryOperator(inner) => inner.span(),
        }
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum StabClauseLeft<'tree> {
    Arguments(::std::boxed::Box<Arguments<'tree>>),
    BinaryOperator(::std::boxed::Box<BinaryOperator<'tree>>),
}
impl<'tree> ::treesitter_types::FromNode<'tree> for StabClauseLeft<'tree> {
    #[allow(clippy::collapsible_else_if)]
    fn from_node(
        node: ::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        match node.kind() {
            "arguments" => Ok(Self::Arguments(::std::boxed::Box::new(
                <Arguments as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "binary_operator" => Ok(Self::BinaryOperator(::std::boxed::Box::new(
                <BinaryOperator as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            other => Err(::treesitter_types::ParseError::unexpected_kind(other, node)),
        }
    }
}
impl ::treesitter_types::Spanned for StabClauseLeft<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        match self {
            Self::Arguments(inner) => inner.span(),
            Self::BinaryOperator(inner) => inner.span(),
        }
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum StabClauseOperator {
    Arrow(::treesitter_types::Span),
}
impl<'tree> ::treesitter_types::FromNode<'tree> for StabClauseOperator {
    #[allow(clippy::collapsible_else_if)]
    fn from_node(
        node: ::tree_sitter::Node<'tree>,
        _src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        match node.kind() {
            "->" => Ok(Self::Arrow(::treesitter_types::Span::from(node))),
            other => Err(::treesitter_types::ParseError::unexpected_kind(other, node)),
        }
    }
}
impl ::treesitter_types::Spanned for StabClauseOperator {
    fn span(&self) -> ::treesitter_types::Span {
        match self {
            Self::Arrow(span) => *span,
        }
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum StringQuotedEnd {
    DoubleQuote(::treesitter_types::Span),
    DQuoteDQuoteDQuote(::treesitter_types::Span),
}
impl<'tree> ::treesitter_types::FromNode<'tree> for StringQuotedEnd {
    #[allow(clippy::collapsible_else_if)]
    fn from_node(
        node: ::tree_sitter::Node<'tree>,
        _src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        match node.kind() {
            "\"" => Ok(Self::DoubleQuote(::treesitter_types::Span::from(node))),
            "\"\"\"" => Ok(Self::DQuoteDQuoteDQuote(::treesitter_types::Span::from(
                node,
            ))),
            other => Err(::treesitter_types::ParseError::unexpected_kind(other, node)),
        }
    }
}
impl ::treesitter_types::Spanned for StringQuotedEnd {
    fn span(&self) -> ::treesitter_types::Span {
        match self {
            Self::DoubleQuote(span) => *span,
            Self::DQuoteDQuoteDQuote(span) => *span,
        }
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum StringQuotedStart {
    DoubleQuote(::treesitter_types::Span),
    DQuoteDQuoteDQuote(::treesitter_types::Span),
}
impl<'tree> ::treesitter_types::FromNode<'tree> for StringQuotedStart {
    #[allow(clippy::collapsible_else_if)]
    fn from_node(
        node: ::tree_sitter::Node<'tree>,
        _src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        match node.kind() {
            "\"" => Ok(Self::DoubleQuote(::treesitter_types::Span::from(node))),
            "\"\"\"" => Ok(Self::DQuoteDQuoteDQuote(::treesitter_types::Span::from(
                node,
            ))),
            other => Err(::treesitter_types::ParseError::unexpected_kind(other, node)),
        }
    }
}
impl ::treesitter_types::Spanned for StringQuotedStart {
    fn span(&self) -> ::treesitter_types::Span {
        match self {
            Self::DoubleQuote(span) => *span,
            Self::DQuoteDQuoteDQuote(span) => *span,
        }
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum StringChildren<'tree> {
    EscapeSequence(::std::boxed::Box<EscapeSequence<'tree>>),
    Interpolation(::std::boxed::Box<Interpolation<'tree>>),
    QuotedContent(::std::boxed::Box<QuotedContent<'tree>>),
}
impl<'tree> ::treesitter_types::FromNode<'tree> for StringChildren<'tree> {
    #[allow(clippy::collapsible_else_if)]
    fn from_node(
        node: ::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        match node.kind() {
            "escape_sequence" => Ok(Self::EscapeSequence(::std::boxed::Box::new(
                <EscapeSequence as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "interpolation" => Ok(Self::Interpolation(::std::boxed::Box::new(
                <Interpolation as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "quoted_content" => Ok(Self::QuotedContent(::std::boxed::Box::new(
                <QuotedContent as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            other => Err(::treesitter_types::ParseError::unexpected_kind(other, node)),
        }
    }
}
impl ::treesitter_types::Spanned for StringChildren<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        match self {
            Self::EscapeSequence(inner) => inner.span(),
            Self::Interpolation(inner) => inner.span(),
            Self::QuotedContent(inner) => inner.span(),
        }
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum StructChildren<'tree> {
    Alias(::std::boxed::Box<Alias<'tree>>),
    Atom(::std::boxed::Box<Atom<'tree>>),
    Call(::std::boxed::Box<Call<'tree>>),
    Dot(::std::boxed::Box<Dot<'tree>>),
    Identifier(::std::boxed::Box<Identifier<'tree>>),
    QuotedAtom(::std::boxed::Box<QuotedAtom<'tree>>),
    UnaryOperator(::std::boxed::Box<UnaryOperator<'tree>>),
}
impl<'tree> ::treesitter_types::FromNode<'tree> for StructChildren<'tree> {
    #[allow(clippy::collapsible_else_if)]
    fn from_node(
        node: ::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        match node.kind() {
            "alias" => Ok(Self::Alias(::std::boxed::Box::new(
                <Alias as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "atom" => Ok(Self::Atom(::std::boxed::Box::new(
                <Atom as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "call" => Ok(Self::Call(::std::boxed::Box::new(
                <Call as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "dot" => Ok(Self::Dot(::std::boxed::Box::new(
                <Dot as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "identifier" => Ok(Self::Identifier(::std::boxed::Box::new(
                <Identifier as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "quoted_atom" => Ok(Self::QuotedAtom(::std::boxed::Box::new(
                <QuotedAtom as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "unary_operator" => Ok(Self::UnaryOperator(::std::boxed::Box::new(
                <UnaryOperator as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            other => Err(::treesitter_types::ParseError::unexpected_kind(other, node)),
        }
    }
}
impl ::treesitter_types::Spanned for StructChildren<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        match self {
            Self::Alias(inner) => inner.span(),
            Self::Atom(inner) => inner.span(),
            Self::Call(inner) => inner.span(),
            Self::Dot(inner) => inner.span(),
            Self::Identifier(inner) => inner.span(),
            Self::QuotedAtom(inner) => inner.span(),
            Self::UnaryOperator(inner) => inner.span(),
        }
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum TupleChildren<'tree> {
    AccessCall(::std::boxed::Box<AccessCall<'tree>>),
    Alias(::std::boxed::Box<Alias<'tree>>),
    AnonymousFunction(::std::boxed::Box<AnonymousFunction<'tree>>),
    Atom(::std::boxed::Box<Atom<'tree>>),
    BinaryOperator(::std::boxed::Box<BinaryOperator<'tree>>),
    Bitstring(::std::boxed::Box<Bitstring<'tree>>),
    Block(::std::boxed::Box<Block<'tree>>),
    Boolean(::std::boxed::Box<Boolean<'tree>>),
    Call(::std::boxed::Box<Call<'tree>>),
    Char(::std::boxed::Box<Char<'tree>>),
    Charlist(::std::boxed::Box<Charlist<'tree>>),
    Dot(::std::boxed::Box<Dot<'tree>>),
    Float(::std::boxed::Box<Float<'tree>>),
    Identifier(::std::boxed::Box<Identifier<'tree>>),
    Integer(::std::boxed::Box<Integer<'tree>>),
    Keywords(::std::boxed::Box<Keywords<'tree>>),
    List(::std::boxed::Box<List<'tree>>),
    Map(::std::boxed::Box<Map<'tree>>),
    Nil(::std::boxed::Box<Nil<'tree>>),
    OperatorIdentifier(::std::boxed::Box<OperatorIdentifier<'tree>>),
    QuotedAtom(::std::boxed::Box<QuotedAtom<'tree>>),
    Sigil(::std::boxed::Box<Sigil<'tree>>),
    String(::std::boxed::Box<String<'tree>>),
    Tuple(::std::boxed::Box<Tuple<'tree>>),
    UnaryOperator(::std::boxed::Box<UnaryOperator<'tree>>),
}
impl<'tree> ::treesitter_types::FromNode<'tree> for TupleChildren<'tree> {
    #[allow(clippy::collapsible_else_if)]
    fn from_node(
        node: ::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        match node.kind() {
            "access_call" => Ok(Self::AccessCall(::std::boxed::Box::new(
                <AccessCall as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "alias" => Ok(Self::Alias(::std::boxed::Box::new(
                <Alias as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "anonymous_function" => Ok(Self::AnonymousFunction(::std::boxed::Box::new(
                <AnonymousFunction as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "atom" => Ok(Self::Atom(::std::boxed::Box::new(
                <Atom as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "binary_operator" => Ok(Self::BinaryOperator(::std::boxed::Box::new(
                <BinaryOperator as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "bitstring" => Ok(Self::Bitstring(::std::boxed::Box::new(
                <Bitstring as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "block" => Ok(Self::Block(::std::boxed::Box::new(
                <Block as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "boolean" => Ok(Self::Boolean(::std::boxed::Box::new(
                <Boolean as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "call" => Ok(Self::Call(::std::boxed::Box::new(
                <Call as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "char" => Ok(Self::Char(::std::boxed::Box::new(
                <Char as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "charlist" => Ok(Self::Charlist(::std::boxed::Box::new(
                <Charlist as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "dot" => Ok(Self::Dot(::std::boxed::Box::new(
                <Dot as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "float" => Ok(Self::Float(::std::boxed::Box::new(
                <Float as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "identifier" => Ok(Self::Identifier(::std::boxed::Box::new(
                <Identifier as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "integer" => Ok(Self::Integer(::std::boxed::Box::new(
                <Integer as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "keywords" => Ok(Self::Keywords(::std::boxed::Box::new(
                <Keywords as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "list" => Ok(Self::List(::std::boxed::Box::new(
                <List as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "map" => Ok(Self::Map(::std::boxed::Box::new(
                <Map as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "nil" => Ok(Self::Nil(::std::boxed::Box::new(
                <Nil as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "operator_identifier" => Ok(Self::OperatorIdentifier(::std::boxed::Box::new(
                <OperatorIdentifier as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "quoted_atom" => Ok(Self::QuotedAtom(::std::boxed::Box::new(
                <QuotedAtom as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "sigil" => Ok(Self::Sigil(::std::boxed::Box::new(
                <Sigil as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "string" => Ok(Self::String(::std::boxed::Box::new(
                <String as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "tuple" => Ok(Self::Tuple(::std::boxed::Box::new(
                <Tuple as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "unary_operator" => Ok(Self::UnaryOperator(::std::boxed::Box::new(
                <UnaryOperator as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            other => Err(::treesitter_types::ParseError::unexpected_kind(other, node)),
        }
    }
}
impl ::treesitter_types::Spanned for TupleChildren<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        match self {
            Self::AccessCall(inner) => inner.span(),
            Self::Alias(inner) => inner.span(),
            Self::AnonymousFunction(inner) => inner.span(),
            Self::Atom(inner) => inner.span(),
            Self::BinaryOperator(inner) => inner.span(),
            Self::Bitstring(inner) => inner.span(),
            Self::Block(inner) => inner.span(),
            Self::Boolean(inner) => inner.span(),
            Self::Call(inner) => inner.span(),
            Self::Char(inner) => inner.span(),
            Self::Charlist(inner) => inner.span(),
            Self::Dot(inner) => inner.span(),
            Self::Float(inner) => inner.span(),
            Self::Identifier(inner) => inner.span(),
            Self::Integer(inner) => inner.span(),
            Self::Keywords(inner) => inner.span(),
            Self::List(inner) => inner.span(),
            Self::Map(inner) => inner.span(),
            Self::Nil(inner) => inner.span(),
            Self::OperatorIdentifier(inner) => inner.span(),
            Self::QuotedAtom(inner) => inner.span(),
            Self::Sigil(inner) => inner.span(),
            Self::String(inner) => inner.span(),
            Self::Tuple(inner) => inner.span(),
            Self::UnaryOperator(inner) => inner.span(),
        }
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum UnaryOperatorOperand<'tree> {
    LParen(::treesitter_types::Span),
    RParen(::treesitter_types::Span),
    AccessCall(::std::boxed::Box<AccessCall<'tree>>),
    Alias(::std::boxed::Box<Alias<'tree>>),
    AnonymousFunction(::std::boxed::Box<AnonymousFunction<'tree>>),
    Atom(::std::boxed::Box<Atom<'tree>>),
    BinaryOperator(::std::boxed::Box<BinaryOperator<'tree>>),
    Bitstring(::std::boxed::Box<Bitstring<'tree>>),
    Block(::std::boxed::Box<Block<'tree>>),
    Boolean(::std::boxed::Box<Boolean<'tree>>),
    Call(::std::boxed::Box<Call<'tree>>),
    Char(::std::boxed::Box<Char<'tree>>),
    Charlist(::std::boxed::Box<Charlist<'tree>>),
    Dot(::std::boxed::Box<Dot<'tree>>),
    Float(::std::boxed::Box<Float<'tree>>),
    Identifier(::std::boxed::Box<Identifier<'tree>>),
    Integer(::std::boxed::Box<Integer<'tree>>),
    List(::std::boxed::Box<List<'tree>>),
    Map(::std::boxed::Box<Map<'tree>>),
    Nil(::std::boxed::Box<Nil<'tree>>),
    OperatorIdentifier(::std::boxed::Box<OperatorIdentifier<'tree>>),
    QuotedAtom(::std::boxed::Box<QuotedAtom<'tree>>),
    Sigil(::std::boxed::Box<Sigil<'tree>>),
    String(::std::boxed::Box<String<'tree>>),
    Tuple(::std::boxed::Box<Tuple<'tree>>),
    UnaryOperator(::std::boxed::Box<UnaryOperator<'tree>>),
}
impl<'tree> ::treesitter_types::FromNode<'tree> for UnaryOperatorOperand<'tree> {
    #[allow(clippy::collapsible_else_if)]
    fn from_node(
        node: ::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        match node.kind() {
            "(" => Ok(Self::LParen(::treesitter_types::Span::from(node))),
            ")" => Ok(Self::RParen(::treesitter_types::Span::from(node))),
            "access_call" => Ok(Self::AccessCall(::std::boxed::Box::new(
                <AccessCall as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "alias" => Ok(Self::Alias(::std::boxed::Box::new(
                <Alias as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "anonymous_function" => Ok(Self::AnonymousFunction(::std::boxed::Box::new(
                <AnonymousFunction as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "atom" => Ok(Self::Atom(::std::boxed::Box::new(
                <Atom as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "binary_operator" => Ok(Self::BinaryOperator(::std::boxed::Box::new(
                <BinaryOperator as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "bitstring" => Ok(Self::Bitstring(::std::boxed::Box::new(
                <Bitstring as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "block" => Ok(Self::Block(::std::boxed::Box::new(
                <Block as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "boolean" => Ok(Self::Boolean(::std::boxed::Box::new(
                <Boolean as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "call" => Ok(Self::Call(::std::boxed::Box::new(
                <Call as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "char" => Ok(Self::Char(::std::boxed::Box::new(
                <Char as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "charlist" => Ok(Self::Charlist(::std::boxed::Box::new(
                <Charlist as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "dot" => Ok(Self::Dot(::std::boxed::Box::new(
                <Dot as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "float" => Ok(Self::Float(::std::boxed::Box::new(
                <Float as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "identifier" => Ok(Self::Identifier(::std::boxed::Box::new(
                <Identifier as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "integer" => Ok(Self::Integer(::std::boxed::Box::new(
                <Integer as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "list" => Ok(Self::List(::std::boxed::Box::new(
                <List as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "map" => Ok(Self::Map(::std::boxed::Box::new(
                <Map as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "nil" => Ok(Self::Nil(::std::boxed::Box::new(
                <Nil as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "operator_identifier" => Ok(Self::OperatorIdentifier(::std::boxed::Box::new(
                <OperatorIdentifier as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "quoted_atom" => Ok(Self::QuotedAtom(::std::boxed::Box::new(
                <QuotedAtom as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "sigil" => Ok(Self::Sigil(::std::boxed::Box::new(
                <Sigil as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "string" => Ok(Self::String(::std::boxed::Box::new(
                <String as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "tuple" => Ok(Self::Tuple(::std::boxed::Box::new(
                <Tuple as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "unary_operator" => Ok(Self::UnaryOperator(::std::boxed::Box::new(
                <UnaryOperator as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            other => Err(::treesitter_types::ParseError::unexpected_kind(other, node)),
        }
    }
}
impl ::treesitter_types::Spanned for UnaryOperatorOperand<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        match self {
            Self::LParen(span) => *span,
            Self::RParen(span) => *span,
            Self::AccessCall(inner) => inner.span(),
            Self::Alias(inner) => inner.span(),
            Self::AnonymousFunction(inner) => inner.span(),
            Self::Atom(inner) => inner.span(),
            Self::BinaryOperator(inner) => inner.span(),
            Self::Bitstring(inner) => inner.span(),
            Self::Block(inner) => inner.span(),
            Self::Boolean(inner) => inner.span(),
            Self::Call(inner) => inner.span(),
            Self::Char(inner) => inner.span(),
            Self::Charlist(inner) => inner.span(),
            Self::Dot(inner) => inner.span(),
            Self::Float(inner) => inner.span(),
            Self::Identifier(inner) => inner.span(),
            Self::Integer(inner) => inner.span(),
            Self::List(inner) => inner.span(),
            Self::Map(inner) => inner.span(),
            Self::Nil(inner) => inner.span(),
            Self::OperatorIdentifier(inner) => inner.span(),
            Self::QuotedAtom(inner) => inner.span(),
            Self::Sigil(inner) => inner.span(),
            Self::String(inner) => inner.span(),
            Self::Tuple(inner) => inner.span(),
            Self::UnaryOperator(inner) => inner.span(),
        }
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum UnaryOperatorOperator {
    Bang(::treesitter_types::Span),
    Amp(::treesitter_types::Span),
    Plus(::treesitter_types::Span),
    Minus(::treesitter_types::Span),
    At(::treesitter_types::Span),
    Caret(::treesitter_types::Span),
    Not(::treesitter_types::Span),
    TildeTildeTilde(::treesitter_types::Span),
}
impl<'tree> ::treesitter_types::FromNode<'tree> for UnaryOperatorOperator {
    #[allow(clippy::collapsible_else_if)]
    fn from_node(
        node: ::tree_sitter::Node<'tree>,
        _src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        match node.kind() {
            "!" => Ok(Self::Bang(::treesitter_types::Span::from(node))),
            "&" => Ok(Self::Amp(::treesitter_types::Span::from(node))),
            "+" => Ok(Self::Plus(::treesitter_types::Span::from(node))),
            "-" => Ok(Self::Minus(::treesitter_types::Span::from(node))),
            "@" => Ok(Self::At(::treesitter_types::Span::from(node))),
            "^" => Ok(Self::Caret(::treesitter_types::Span::from(node))),
            "not" => Ok(Self::Not(::treesitter_types::Span::from(node))),
            "~~~" => Ok(Self::TildeTildeTilde(::treesitter_types::Span::from(node))),
            other => Err(::treesitter_types::ParseError::unexpected_kind(other, node)),
        }
    }
}
impl ::treesitter_types::Spanned for UnaryOperatorOperator {
    fn span(&self) -> ::treesitter_types::Span {
        match self {
            Self::Bang(span) => *span,
            Self::Amp(span) => *span,
            Self::Plus(span) => *span,
            Self::Minus(span) => *span,
            Self::At(span) => *span,
            Self::Caret(span) => *span,
            Self::Not(span) => *span,
            Self::TildeTildeTilde(span) => *span,
        }
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum AnyNode<'tree> {
    AccessCall(AccessCall<'tree>),
    AfterBlock(AfterBlock<'tree>),
    AnonymousFunction(AnonymousFunction<'tree>),
    Arguments(Arguments<'tree>),
    BinaryOperator(BinaryOperator<'tree>),
    Bitstring(Bitstring<'tree>),
    Block(Block<'tree>),
    Body(Body<'tree>),
    Boolean(Boolean<'tree>),
    Call(Call<'tree>),
    CatchBlock(CatchBlock<'tree>),
    Charlist(Charlist<'tree>),
    DoBlock(DoBlock<'tree>),
    Dot(Dot<'tree>),
    ElseBlock(ElseBlock<'tree>),
    Identifier(Identifier<'tree>),
    Interpolation(Interpolation<'tree>),
    Keywords(Keywords<'tree>),
    List(List<'tree>),
    Map(Map<'tree>),
    MapContent(MapContent<'tree>),
    Nil(Nil<'tree>),
    OperatorIdentifier(OperatorIdentifier<'tree>),
    Pair(Pair<'tree>),
    QuotedAtom(QuotedAtom<'tree>),
    QuotedKeyword(QuotedKeyword<'tree>),
    RescueBlock(RescueBlock<'tree>),
    Sigil(Sigil<'tree>),
    Source(Source<'tree>),
    StabClause(StabClause<'tree>),
    String(String<'tree>),
    Struct(Struct<'tree>),
    Tuple(Tuple<'tree>),
    UnaryOperator(UnaryOperator<'tree>),
    Alias(Alias<'tree>),
    Atom(Atom<'tree>),
    Char(Char<'tree>),
    Comment(Comment<'tree>),
    EscapeSequence(EscapeSequence<'tree>),
    Float(Float<'tree>),
    Integer(Integer<'tree>),
    Keyword(Keyword<'tree>),
    QuotedContent(QuotedContent<'tree>),
    SigilModifiers(SigilModifiers<'tree>),
    SigilName(SigilName<'tree>),
    Unknown(::tree_sitter::Node<'tree>),
}
impl<'tree> AnyNode<'tree> {
    pub fn from_node(node: ::tree_sitter::Node<'tree>, src: &'tree [u8]) -> Self {
        match node.kind() {
            "access_call" => <AccessCall as ::treesitter_types::FromNode>::from_node(node, src)
                .map(Self::AccessCall)
                .unwrap_or(Self::Unknown(node)),
            "after_block" => <AfterBlock as ::treesitter_types::FromNode>::from_node(node, src)
                .map(Self::AfterBlock)
                .unwrap_or(Self::Unknown(node)),
            "anonymous_function" => {
                <AnonymousFunction as ::treesitter_types::FromNode>::from_node(node, src)
                    .map(Self::AnonymousFunction)
                    .unwrap_or(Self::Unknown(node))
            }
            "arguments" => <Arguments as ::treesitter_types::FromNode>::from_node(node, src)
                .map(Self::Arguments)
                .unwrap_or(Self::Unknown(node)),
            "binary_operator" => {
                <BinaryOperator as ::treesitter_types::FromNode>::from_node(node, src)
                    .map(Self::BinaryOperator)
                    .unwrap_or(Self::Unknown(node))
            }
            "bitstring" => <Bitstring as ::treesitter_types::FromNode>::from_node(node, src)
                .map(Self::Bitstring)
                .unwrap_or(Self::Unknown(node)),
            "block" => <Block as ::treesitter_types::FromNode>::from_node(node, src)
                .map(Self::Block)
                .unwrap_or(Self::Unknown(node)),
            "body" => <Body as ::treesitter_types::FromNode>::from_node(node, src)
                .map(Self::Body)
                .unwrap_or(Self::Unknown(node)),
            "boolean" => <Boolean as ::treesitter_types::FromNode>::from_node(node, src)
                .map(Self::Boolean)
                .unwrap_or(Self::Unknown(node)),
            "call" => <Call as ::treesitter_types::FromNode>::from_node(node, src)
                .map(Self::Call)
                .unwrap_or(Self::Unknown(node)),
            "catch_block" => <CatchBlock as ::treesitter_types::FromNode>::from_node(node, src)
                .map(Self::CatchBlock)
                .unwrap_or(Self::Unknown(node)),
            "charlist" => <Charlist as ::treesitter_types::FromNode>::from_node(node, src)
                .map(Self::Charlist)
                .unwrap_or(Self::Unknown(node)),
            "do_block" => <DoBlock as ::treesitter_types::FromNode>::from_node(node, src)
                .map(Self::DoBlock)
                .unwrap_or(Self::Unknown(node)),
            "dot" => <Dot as ::treesitter_types::FromNode>::from_node(node, src)
                .map(Self::Dot)
                .unwrap_or(Self::Unknown(node)),
            "else_block" => <ElseBlock as ::treesitter_types::FromNode>::from_node(node, src)
                .map(Self::ElseBlock)
                .unwrap_or(Self::Unknown(node)),
            "identifier" => <Identifier as ::treesitter_types::FromNode>::from_node(node, src)
                .map(Self::Identifier)
                .unwrap_or(Self::Unknown(node)),
            "interpolation" => {
                <Interpolation as ::treesitter_types::FromNode>::from_node(node, src)
                    .map(Self::Interpolation)
                    .unwrap_or(Self::Unknown(node))
            }
            "keywords" => <Keywords as ::treesitter_types::FromNode>::from_node(node, src)
                .map(Self::Keywords)
                .unwrap_or(Self::Unknown(node)),
            "list" => <List as ::treesitter_types::FromNode>::from_node(node, src)
                .map(Self::List)
                .unwrap_or(Self::Unknown(node)),
            "map" => <Map as ::treesitter_types::FromNode>::from_node(node, src)
                .map(Self::Map)
                .unwrap_or(Self::Unknown(node)),
            "map_content" => <MapContent as ::treesitter_types::FromNode>::from_node(node, src)
                .map(Self::MapContent)
                .unwrap_or(Self::Unknown(node)),
            "nil" => <Nil as ::treesitter_types::FromNode>::from_node(node, src)
                .map(Self::Nil)
                .unwrap_or(Self::Unknown(node)),
            "operator_identifier" => {
                <OperatorIdentifier as ::treesitter_types::FromNode>::from_node(node, src)
                    .map(Self::OperatorIdentifier)
                    .unwrap_or(Self::Unknown(node))
            }
            "pair" => <Pair as ::treesitter_types::FromNode>::from_node(node, src)
                .map(Self::Pair)
                .unwrap_or(Self::Unknown(node)),
            "quoted_atom" => <QuotedAtom as ::treesitter_types::FromNode>::from_node(node, src)
                .map(Self::QuotedAtom)
                .unwrap_or(Self::Unknown(node)),
            "quoted_keyword" => {
                <QuotedKeyword as ::treesitter_types::FromNode>::from_node(node, src)
                    .map(Self::QuotedKeyword)
                    .unwrap_or(Self::Unknown(node))
            }
            "rescue_block" => <RescueBlock as ::treesitter_types::FromNode>::from_node(node, src)
                .map(Self::RescueBlock)
                .unwrap_or(Self::Unknown(node)),
            "sigil" => <Sigil as ::treesitter_types::FromNode>::from_node(node, src)
                .map(Self::Sigil)
                .unwrap_or(Self::Unknown(node)),
            "source" => <Source as ::treesitter_types::FromNode>::from_node(node, src)
                .map(Self::Source)
                .unwrap_or(Self::Unknown(node)),
            "stab_clause" => <StabClause as ::treesitter_types::FromNode>::from_node(node, src)
                .map(Self::StabClause)
                .unwrap_or(Self::Unknown(node)),
            "string" => <String as ::treesitter_types::FromNode>::from_node(node, src)
                .map(Self::String)
                .unwrap_or(Self::Unknown(node)),
            "struct" => <Struct as ::treesitter_types::FromNode>::from_node(node, src)
                .map(Self::Struct)
                .unwrap_or(Self::Unknown(node)),
            "tuple" => <Tuple as ::treesitter_types::FromNode>::from_node(node, src)
                .map(Self::Tuple)
                .unwrap_or(Self::Unknown(node)),
            "unary_operator" => {
                <UnaryOperator as ::treesitter_types::FromNode>::from_node(node, src)
                    .map(Self::UnaryOperator)
                    .unwrap_or(Self::Unknown(node))
            }
            "alias" => <Alias as ::treesitter_types::FromNode>::from_node(node, src)
                .map(Self::Alias)
                .unwrap_or(Self::Unknown(node)),
            "atom" => <Atom as ::treesitter_types::FromNode>::from_node(node, src)
                .map(Self::Atom)
                .unwrap_or(Self::Unknown(node)),
            "char" => <Char as ::treesitter_types::FromNode>::from_node(node, src)
                .map(Self::Char)
                .unwrap_or(Self::Unknown(node)),
            "comment" => <Comment as ::treesitter_types::FromNode>::from_node(node, src)
                .map(Self::Comment)
                .unwrap_or(Self::Unknown(node)),
            "escape_sequence" => {
                <EscapeSequence as ::treesitter_types::FromNode>::from_node(node, src)
                    .map(Self::EscapeSequence)
                    .unwrap_or(Self::Unknown(node))
            }
            "float" => <Float as ::treesitter_types::FromNode>::from_node(node, src)
                .map(Self::Float)
                .unwrap_or(Self::Unknown(node)),
            "integer" => <Integer as ::treesitter_types::FromNode>::from_node(node, src)
                .map(Self::Integer)
                .unwrap_or(Self::Unknown(node)),
            "keyword" => <Keyword as ::treesitter_types::FromNode>::from_node(node, src)
                .map(Self::Keyword)
                .unwrap_or(Self::Unknown(node)),
            "quoted_content" => {
                <QuotedContent as ::treesitter_types::FromNode>::from_node(node, src)
                    .map(Self::QuotedContent)
                    .unwrap_or(Self::Unknown(node))
            }
            "sigil_modifiers" => {
                <SigilModifiers as ::treesitter_types::FromNode>::from_node(node, src)
                    .map(Self::SigilModifiers)
                    .unwrap_or(Self::Unknown(node))
            }
            "sigil_name" => <SigilName as ::treesitter_types::FromNode>::from_node(node, src)
                .map(Self::SigilName)
                .unwrap_or(Self::Unknown(node)),
            _ => Self::Unknown(node),
        }
    }
}
impl ::treesitter_types::Spanned for AnyNode<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        match self {
            Self::AccessCall(inner) => inner.span(),
            Self::AfterBlock(inner) => inner.span(),
            Self::AnonymousFunction(inner) => inner.span(),
            Self::Arguments(inner) => inner.span(),
            Self::BinaryOperator(inner) => inner.span(),
            Self::Bitstring(inner) => inner.span(),
            Self::Block(inner) => inner.span(),
            Self::Body(inner) => inner.span(),
            Self::Boolean(inner) => inner.span(),
            Self::Call(inner) => inner.span(),
            Self::CatchBlock(inner) => inner.span(),
            Self::Charlist(inner) => inner.span(),
            Self::DoBlock(inner) => inner.span(),
            Self::Dot(inner) => inner.span(),
            Self::ElseBlock(inner) => inner.span(),
            Self::Identifier(inner) => inner.span(),
            Self::Interpolation(inner) => inner.span(),
            Self::Keywords(inner) => inner.span(),
            Self::List(inner) => inner.span(),
            Self::Map(inner) => inner.span(),
            Self::MapContent(inner) => inner.span(),
            Self::Nil(inner) => inner.span(),
            Self::OperatorIdentifier(inner) => inner.span(),
            Self::Pair(inner) => inner.span(),
            Self::QuotedAtom(inner) => inner.span(),
            Self::QuotedKeyword(inner) => inner.span(),
            Self::RescueBlock(inner) => inner.span(),
            Self::Sigil(inner) => inner.span(),
            Self::Source(inner) => inner.span(),
            Self::StabClause(inner) => inner.span(),
            Self::String(inner) => inner.span(),
            Self::Struct(inner) => inner.span(),
            Self::Tuple(inner) => inner.span(),
            Self::UnaryOperator(inner) => inner.span(),
            Self::Alias(inner) => inner.span(),
            Self::Atom(inner) => inner.span(),
            Self::Char(inner) => inner.span(),
            Self::Comment(inner) => inner.span(),
            Self::EscapeSequence(inner) => inner.span(),
            Self::Float(inner) => inner.span(),
            Self::Integer(inner) => inner.span(),
            Self::Keyword(inner) => inner.span(),
            Self::QuotedContent(inner) => inner.span(),
            Self::SigilModifiers(inner) => inner.span(),
            Self::SigilName(inner) => inner.span(),
            Self::Unknown(node) => ::treesitter_types::Span::from(*node),
        }
    }
}
