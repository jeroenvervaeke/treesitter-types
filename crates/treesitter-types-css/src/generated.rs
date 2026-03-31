#[derive(Debug, Clone)]
pub struct AdjacentSiblingSelector<'tree> {
    pub span: ::treesitter_types::Span,
    pub children: ::std::vec::Vec<AdjacentSiblingSelectorChildren<'tree>>,
}
impl<'tree> ::treesitter_types::FromNode<'tree> for AdjacentSiblingSelector<'tree> {
    #[allow(clippy::match_single_binding, clippy::suspicious_else_formatting)]
    fn from_node(
        node: ::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        debug_assert_eq!(node.kind(), "adjacent_sibling_selector");
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
                    items
                        .push(
                            <AdjacentSiblingSelectorChildren as ::treesitter_types::FromNode>::from_node(
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
impl ::treesitter_types::Spanned for AdjacentSiblingSelector<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        self.span
    }
}
#[derive(Debug, Clone)]
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
#[derive(Debug, Clone)]
pub struct AtRule<'tree> {
    pub span: ::treesitter_types::Span,
    pub children: ::std::vec::Vec<AtRuleChildren<'tree>>,
}
impl<'tree> ::treesitter_types::FromNode<'tree> for AtRule<'tree> {
    #[allow(clippy::match_single_binding, clippy::suspicious_else_formatting)]
    fn from_node(
        node: ::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        debug_assert_eq!(node.kind(), "at_rule");
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
                    items.push(<AtRuleChildren as ::treesitter_types::FromNode>::from_node(
                        child, src,
                    )?);
                }
                items
            },
        })
    }
}
impl ::treesitter_types::Spanned for AtRule<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        self.span
    }
}
#[derive(Debug, Clone)]
pub struct AttributeName<'tree> {
    pub span: ::treesitter_types::Span,
    pub children: ::std::vec::Vec<AttributeNameChildren<'tree>>,
}
impl<'tree> ::treesitter_types::FromNode<'tree> for AttributeName<'tree> {
    #[allow(clippy::match_single_binding, clippy::suspicious_else_formatting)]
    fn from_node(
        node: ::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        debug_assert_eq!(node.kind(), "attribute_name");
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
                        <AttributeNameChildren as ::treesitter_types::FromNode>::from_node(
                            child, src,
                        )?,
                    );
                }
                items
            },
        })
    }
}
impl ::treesitter_types::Spanned for AttributeName<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        self.span
    }
}
#[derive(Debug, Clone)]
pub struct AttributeSelector<'tree> {
    pub span: ::treesitter_types::Span,
    pub children: ::std::vec::Vec<AttributeSelectorChildren<'tree>>,
}
impl<'tree> ::treesitter_types::FromNode<'tree> for AttributeSelector<'tree> {
    #[allow(clippy::match_single_binding, clippy::suspicious_else_formatting)]
    fn from_node(
        node: ::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        debug_assert_eq!(node.kind(), "attribute_selector");
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
                        <AttributeSelectorChildren as ::treesitter_types::FromNode>::from_node(
                            child, src,
                        )?,
                    );
                }
                items
            },
        })
    }
}
impl ::treesitter_types::Spanned for AttributeSelector<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        self.span
    }
}
#[derive(Debug, Clone)]
pub struct BinaryExpression<'tree> {
    pub span: ::treesitter_types::Span,
    pub children: ::std::vec::Vec<BinaryExpressionChildren<'tree>>,
}
impl<'tree> ::treesitter_types::FromNode<'tree> for BinaryExpression<'tree> {
    #[allow(clippy::match_single_binding, clippy::suspicious_else_formatting)]
    fn from_node(
        node: ::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        debug_assert_eq!(node.kind(), "binary_expression");
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
                        <BinaryExpressionChildren as ::treesitter_types::FromNode>::from_node(
                            child, src,
                        )?,
                    );
                }
                items
            },
        })
    }
}
impl ::treesitter_types::Spanned for BinaryExpression<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        self.span
    }
}
#[derive(Debug, Clone)]
pub struct BinaryQuery<'tree> {
    pub span: ::treesitter_types::Span,
    pub children: ::std::vec::Vec<BinaryQueryChildren<'tree>>,
}
impl<'tree> ::treesitter_types::FromNode<'tree> for BinaryQuery<'tree> {
    #[allow(clippy::match_single_binding, clippy::suspicious_else_formatting)]
    fn from_node(
        node: ::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        debug_assert_eq!(node.kind(), "binary_query");
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
                        <BinaryQueryChildren as ::treesitter_types::FromNode>::from_node(
                            child, src,
                        )?,
                    );
                }
                items
            },
        })
    }
}
impl ::treesitter_types::Spanned for BinaryQuery<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        self.span
    }
}
#[derive(Debug, Clone)]
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
#[derive(Debug, Clone)]
pub struct CallExpression<'tree> {
    pub span: ::treesitter_types::Span,
    pub children: ::std::vec::Vec<CallExpressionChildren<'tree>>,
}
impl<'tree> ::treesitter_types::FromNode<'tree> for CallExpression<'tree> {
    #[allow(clippy::match_single_binding, clippy::suspicious_else_formatting)]
    fn from_node(
        node: ::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        debug_assert_eq!(node.kind(), "call_expression");
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
                        <CallExpressionChildren as ::treesitter_types::FromNode>::from_node(
                            child, src,
                        )?,
                    );
                }
                items
            },
        })
    }
}
impl ::treesitter_types::Spanned for CallExpression<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        self.span
    }
}
#[derive(Debug, Clone)]
pub struct CharsetStatement<'tree> {
    pub span: ::treesitter_types::Span,
    pub children: CharsetStatementChildren<'tree>,
}
impl<'tree> ::treesitter_types::FromNode<'tree> for CharsetStatement<'tree> {
    #[allow(clippy::match_single_binding, clippy::suspicious_else_formatting)]
    fn from_node(
        node: ::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        debug_assert_eq!(node.kind(), "charset_statement");
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
                                fallback_child = Some(fallback_cursor.node());
                                break;
                            }
                            if !fallback_cursor.goto_next_sibling() {
                                break;
                            }
                        }
                    }
                    fallback_child.ok_or_else(|| {
                        ::treesitter_types::ParseError::missing_field("children", node)
                    })?
                };
                <CharsetStatementChildren as ::treesitter_types::FromNode>::from_node(child, src)?
            },
        })
    }
}
impl ::treesitter_types::Spanned for CharsetStatement<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        self.span
    }
}
#[derive(Debug, Clone)]
pub struct ChildSelector<'tree> {
    pub span: ::treesitter_types::Span,
    pub children: ::std::vec::Vec<ChildSelectorChildren<'tree>>,
}
impl<'tree> ::treesitter_types::FromNode<'tree> for ChildSelector<'tree> {
    #[allow(clippy::match_single_binding, clippy::suspicious_else_formatting)]
    fn from_node(
        node: ::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        debug_assert_eq!(node.kind(), "child_selector");
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
                        <ChildSelectorChildren as ::treesitter_types::FromNode>::from_node(
                            child, src,
                        )?,
                    );
                }
                items
            },
        })
    }
}
impl ::treesitter_types::Spanned for ChildSelector<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        self.span
    }
}
#[derive(Debug, Clone)]
pub struct ClassName<'tree> {
    pub span: ::treesitter_types::Span,
    pub children: ::std::vec::Vec<ClassNameChildren<'tree>>,
}
impl<'tree> ::treesitter_types::FromNode<'tree> for ClassName<'tree> {
    #[allow(clippy::match_single_binding, clippy::suspicious_else_formatting)]
    fn from_node(
        node: ::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        debug_assert_eq!(node.kind(), "class_name");
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
                        <ClassNameChildren as ::treesitter_types::FromNode>::from_node(child, src)?,
                    );
                }
                items
            },
        })
    }
}
impl ::treesitter_types::Spanned for ClassName<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        self.span
    }
}
#[derive(Debug, Clone)]
pub struct ClassSelector<'tree> {
    pub span: ::treesitter_types::Span,
    pub children: ::std::vec::Vec<ClassSelectorChildren<'tree>>,
}
impl<'tree> ::treesitter_types::FromNode<'tree> for ClassSelector<'tree> {
    #[allow(clippy::match_single_binding, clippy::suspicious_else_formatting)]
    fn from_node(
        node: ::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        debug_assert_eq!(node.kind(), "class_selector");
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
                        <ClassSelectorChildren as ::treesitter_types::FromNode>::from_node(
                            child, src,
                        )?,
                    );
                }
                items
            },
        })
    }
}
impl ::treesitter_types::Spanned for ClassSelector<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        self.span
    }
}
#[derive(Debug, Clone)]
pub struct ColorValue<'tree> {
    pub span: ::treesitter_types::Span,
    text: &'tree str,
}
impl<'tree> ::treesitter_types::FromNode<'tree> for ColorValue<'tree> {
    fn from_node(
        node: ::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        debug_assert_eq!(node.kind(), "color_value");
        Ok(Self {
            span: ::treesitter_types::Span::from(node),
            text: node.utf8_text(src)?,
        })
    }
}
impl<'tree> ::treesitter_types::LeafNode<'tree> for ColorValue<'tree> {
    fn text(&self) -> &'tree str {
        self.text
    }
}
impl ::treesitter_types::Spanned for ColorValue<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        self.span
    }
}
#[derive(Debug, Clone)]
pub struct Declaration<'tree> {
    pub span: ::treesitter_types::Span,
    pub children: ::std::vec::Vec<DeclarationChildren<'tree>>,
}
impl<'tree> ::treesitter_types::FromNode<'tree> for Declaration<'tree> {
    #[allow(clippy::match_single_binding, clippy::suspicious_else_formatting)]
    fn from_node(
        node: ::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        debug_assert_eq!(node.kind(), "declaration");
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
                        <DeclarationChildren as ::treesitter_types::FromNode>::from_node(
                            child, src,
                        )?,
                    );
                }
                items
            },
        })
    }
}
impl ::treesitter_types::Spanned for Declaration<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        self.span
    }
}
#[derive(Debug, Clone)]
pub struct DescendantSelector<'tree> {
    pub span: ::treesitter_types::Span,
    pub children: ::std::vec::Vec<DescendantSelectorChildren<'tree>>,
}
impl<'tree> ::treesitter_types::FromNode<'tree> for DescendantSelector<'tree> {
    #[allow(clippy::match_single_binding, clippy::suspicious_else_formatting)]
    fn from_node(
        node: ::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        debug_assert_eq!(node.kind(), "descendant_selector");
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
                        <DescendantSelectorChildren as ::treesitter_types::FromNode>::from_node(
                            child, src,
                        )?,
                    );
                }
                items
            },
        })
    }
}
impl ::treesitter_types::Spanned for DescendantSelector<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        self.span
    }
}
#[derive(Debug, Clone)]
pub struct FeatureQuery<'tree> {
    pub span: ::treesitter_types::Span,
    pub children: ::std::vec::Vec<FeatureQueryChildren<'tree>>,
}
impl<'tree> ::treesitter_types::FromNode<'tree> for FeatureQuery<'tree> {
    #[allow(clippy::match_single_binding, clippy::suspicious_else_formatting)]
    fn from_node(
        node: ::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        debug_assert_eq!(node.kind(), "feature_query");
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
                        <FeatureQueryChildren as ::treesitter_types::FromNode>::from_node(
                            child, src,
                        )?,
                    );
                }
                items
            },
        })
    }
}
impl ::treesitter_types::Spanned for FeatureQuery<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        self.span
    }
}
#[derive(Debug, Clone)]
pub struct FloatValue<'tree> {
    pub span: ::treesitter_types::Span,
    pub children: ::core::option::Option<Unit<'tree>>,
}
impl<'tree> ::treesitter_types::FromNode<'tree> for FloatValue<'tree> {
    #[allow(clippy::match_single_binding, clippy::suspicious_else_formatting)]
    fn from_node(
        node: ::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        debug_assert_eq!(node.kind(), "float_value");
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
                    Some(&child) => Some(<Unit as ::treesitter_types::FromNode>::from_node(
                        child, src,
                    )?),
                    None => None,
                }
            },
        })
    }
}
impl ::treesitter_types::Spanned for FloatValue<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        self.span
    }
}
#[derive(Debug, Clone)]
pub struct GridValue<'tree> {
    pub span: ::treesitter_types::Span,
    pub children: ::std::vec::Vec<GridValueChildren<'tree>>,
}
impl<'tree> ::treesitter_types::FromNode<'tree> for GridValue<'tree> {
    #[allow(clippy::match_single_binding, clippy::suspicious_else_formatting)]
    fn from_node(
        node: ::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        debug_assert_eq!(node.kind(), "grid_value");
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
                        <GridValueChildren as ::treesitter_types::FromNode>::from_node(child, src)?,
                    );
                }
                items
            },
        })
    }
}
impl ::treesitter_types::Spanned for GridValue<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        self.span
    }
}
#[derive(Debug, Clone)]
pub struct IdSelector<'tree> {
    pub span: ::treesitter_types::Span,
    pub children: ::std::vec::Vec<IdSelectorChildren<'tree>>,
}
impl<'tree> ::treesitter_types::FromNode<'tree> for IdSelector<'tree> {
    #[allow(clippy::match_single_binding, clippy::suspicious_else_formatting)]
    fn from_node(
        node: ::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        debug_assert_eq!(node.kind(), "id_selector");
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
                        <IdSelectorChildren as ::treesitter_types::FromNode>::from_node(
                            child, src,
                        )?,
                    );
                }
                items
            },
        })
    }
}
impl ::treesitter_types::Spanned for IdSelector<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        self.span
    }
}
#[derive(Debug, Clone)]
pub struct ImportStatement<'tree> {
    pub span: ::treesitter_types::Span,
    pub children: ::std::vec::Vec<ImportStatementChildren<'tree>>,
}
impl<'tree> ::treesitter_types::FromNode<'tree> for ImportStatement<'tree> {
    #[allow(clippy::match_single_binding, clippy::suspicious_else_formatting)]
    fn from_node(
        node: ::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        debug_assert_eq!(node.kind(), "import_statement");
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
                        <ImportStatementChildren as ::treesitter_types::FromNode>::from_node(
                            child, src,
                        )?,
                    );
                }
                items
            },
        })
    }
}
impl ::treesitter_types::Spanned for ImportStatement<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        self.span
    }
}
#[derive(Debug, Clone)]
pub struct IntegerValue<'tree> {
    pub span: ::treesitter_types::Span,
    pub children: ::core::option::Option<Unit<'tree>>,
}
impl<'tree> ::treesitter_types::FromNode<'tree> for IntegerValue<'tree> {
    #[allow(clippy::match_single_binding, clippy::suspicious_else_formatting)]
    fn from_node(
        node: ::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        debug_assert_eq!(node.kind(), "integer_value");
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
                    Some(&child) => Some(<Unit as ::treesitter_types::FromNode>::from_node(
                        child, src,
                    )?),
                    None => None,
                }
            },
        })
    }
}
impl ::treesitter_types::Spanned for IntegerValue<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        self.span
    }
}
#[derive(Debug, Clone)]
pub struct KeyframeBlock<'tree> {
    pub span: ::treesitter_types::Span,
    pub children: ::std::vec::Vec<KeyframeBlockChildren<'tree>>,
}
impl<'tree> ::treesitter_types::FromNode<'tree> for KeyframeBlock<'tree> {
    #[allow(clippy::match_single_binding, clippy::suspicious_else_formatting)]
    fn from_node(
        node: ::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        debug_assert_eq!(node.kind(), "keyframe_block");
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
                        <KeyframeBlockChildren as ::treesitter_types::FromNode>::from_node(
                            child, src,
                        )?,
                    );
                }
                items
            },
        })
    }
}
impl ::treesitter_types::Spanned for KeyframeBlock<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        self.span
    }
}
#[derive(Debug, Clone)]
pub struct KeyframeBlockList<'tree> {
    pub span: ::treesitter_types::Span,
    pub children: ::std::vec::Vec<KeyframeBlock<'tree>>,
}
impl<'tree> ::treesitter_types::FromNode<'tree> for KeyframeBlockList<'tree> {
    #[allow(clippy::match_single_binding, clippy::suspicious_else_formatting)]
    fn from_node(
        node: ::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        debug_assert_eq!(node.kind(), "keyframe_block_list");
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
                    items.push(<KeyframeBlock as ::treesitter_types::FromNode>::from_node(
                        child, src,
                    )?);
                }
                items
            },
        })
    }
}
impl ::treesitter_types::Spanned for KeyframeBlockList<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        self.span
    }
}
#[derive(Debug, Clone)]
pub struct KeyframesStatement<'tree> {
    pub span: ::treesitter_types::Span,
    pub children: ::std::vec::Vec<KeyframesStatementChildren<'tree>>,
}
impl<'tree> ::treesitter_types::FromNode<'tree> for KeyframesStatement<'tree> {
    #[allow(clippy::match_single_binding, clippy::suspicious_else_formatting)]
    fn from_node(
        node: ::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        debug_assert_eq!(node.kind(), "keyframes_statement");
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
                        <KeyframesStatementChildren as ::treesitter_types::FromNode>::from_node(
                            child, src,
                        )?,
                    );
                }
                items
            },
        })
    }
}
impl ::treesitter_types::Spanned for KeyframesStatement<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        self.span
    }
}
#[derive(Debug, Clone)]
pub struct MediaStatement<'tree> {
    pub span: ::treesitter_types::Span,
    pub children: ::std::vec::Vec<MediaStatementChildren<'tree>>,
}
impl<'tree> ::treesitter_types::FromNode<'tree> for MediaStatement<'tree> {
    #[allow(clippy::match_single_binding, clippy::suspicious_else_formatting)]
    fn from_node(
        node: ::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        debug_assert_eq!(node.kind(), "media_statement");
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
                        <MediaStatementChildren as ::treesitter_types::FromNode>::from_node(
                            child, src,
                        )?,
                    );
                }
                items
            },
        })
    }
}
impl ::treesitter_types::Spanned for MediaStatement<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        self.span
    }
}
#[derive(Debug, Clone)]
pub struct NamespaceSelector<'tree> {
    pub span: ::treesitter_types::Span,
    pub children: ::std::vec::Vec<NamespaceSelectorChildren<'tree>>,
}
impl<'tree> ::treesitter_types::FromNode<'tree> for NamespaceSelector<'tree> {
    #[allow(clippy::match_single_binding, clippy::suspicious_else_formatting)]
    fn from_node(
        node: ::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        debug_assert_eq!(node.kind(), "namespace_selector");
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
                        <NamespaceSelectorChildren as ::treesitter_types::FromNode>::from_node(
                            child, src,
                        )?,
                    );
                }
                items
            },
        })
    }
}
impl ::treesitter_types::Spanned for NamespaceSelector<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        self.span
    }
}
#[derive(Debug, Clone)]
pub struct NamespaceStatement<'tree> {
    pub span: ::treesitter_types::Span,
    pub children: ::std::vec::Vec<NamespaceStatementChildren<'tree>>,
}
impl<'tree> ::treesitter_types::FromNode<'tree> for NamespaceStatement<'tree> {
    #[allow(clippy::match_single_binding, clippy::suspicious_else_formatting)]
    fn from_node(
        node: ::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        debug_assert_eq!(node.kind(), "namespace_statement");
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
                        <NamespaceStatementChildren as ::treesitter_types::FromNode>::from_node(
                            child, src,
                        )?,
                    );
                }
                items
            },
        })
    }
}
impl ::treesitter_types::Spanned for NamespaceStatement<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        self.span
    }
}
#[derive(Debug, Clone)]
pub struct ParenthesizedQuery<'tree> {
    pub span: ::treesitter_types::Span,
    pub children: ParenthesizedQueryChildren<'tree>,
}
impl<'tree> ::treesitter_types::FromNode<'tree> for ParenthesizedQuery<'tree> {
    #[allow(clippy::match_single_binding, clippy::suspicious_else_formatting)]
    fn from_node(
        node: ::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        debug_assert_eq!(node.kind(), "parenthesized_query");
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
                                fallback_child = Some(fallback_cursor.node());
                                break;
                            }
                            if !fallback_cursor.goto_next_sibling() {
                                break;
                            }
                        }
                    }
                    fallback_child.ok_or_else(|| {
                        ::treesitter_types::ParseError::missing_field("children", node)
                    })?
                };
                <ParenthesizedQueryChildren as ::treesitter_types::FromNode>::from_node(child, src)?
            },
        })
    }
}
impl ::treesitter_types::Spanned for ParenthesizedQuery<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        self.span
    }
}
#[derive(Debug, Clone)]
pub struct ParenthesizedValue<'tree> {
    pub span: ::treesitter_types::Span,
    pub children: ParenthesizedValueChildren<'tree>,
}
impl<'tree> ::treesitter_types::FromNode<'tree> for ParenthesizedValue<'tree> {
    #[allow(clippy::match_single_binding, clippy::suspicious_else_formatting)]
    fn from_node(
        node: ::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        debug_assert_eq!(node.kind(), "parenthesized_value");
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
                                fallback_child = Some(fallback_cursor.node());
                                break;
                            }
                            if !fallback_cursor.goto_next_sibling() {
                                break;
                            }
                        }
                    }
                    fallback_child.ok_or_else(|| {
                        ::treesitter_types::ParseError::missing_field("children", node)
                    })?
                };
                <ParenthesizedValueChildren as ::treesitter_types::FromNode>::from_node(child, src)?
            },
        })
    }
}
impl ::treesitter_types::Spanned for ParenthesizedValue<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        self.span
    }
}
#[derive(Debug, Clone)]
pub struct PostcssStatement<'tree> {
    pub span: ::treesitter_types::Span,
    pub children: ::std::vec::Vec<PostcssStatementChildren<'tree>>,
}
impl<'tree> ::treesitter_types::FromNode<'tree> for PostcssStatement<'tree> {
    #[allow(clippy::match_single_binding, clippy::suspicious_else_formatting)]
    fn from_node(
        node: ::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        debug_assert_eq!(node.kind(), "postcss_statement");
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
                        <PostcssStatementChildren as ::treesitter_types::FromNode>::from_node(
                            child, src,
                        )?,
                    );
                }
                items
            },
        })
    }
}
impl ::treesitter_types::Spanned for PostcssStatement<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        self.span
    }
}
#[derive(Debug, Clone)]
pub struct PseudoClassSelector<'tree> {
    pub span: ::treesitter_types::Span,
    pub children: ::std::vec::Vec<PseudoClassSelectorChildren<'tree>>,
}
impl<'tree> ::treesitter_types::FromNode<'tree> for PseudoClassSelector<'tree> {
    #[allow(clippy::match_single_binding, clippy::suspicious_else_formatting)]
    fn from_node(
        node: ::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        debug_assert_eq!(node.kind(), "pseudo_class_selector");
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
                        <PseudoClassSelectorChildren as ::treesitter_types::FromNode>::from_node(
                            child, src,
                        )?,
                    );
                }
                items
            },
        })
    }
}
impl ::treesitter_types::Spanned for PseudoClassSelector<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        self.span
    }
}
#[derive(Debug, Clone)]
pub struct PseudoElementSelector<'tree> {
    pub span: ::treesitter_types::Span,
    pub children: ::std::vec::Vec<PseudoElementSelectorChildren<'tree>>,
}
impl<'tree> ::treesitter_types::FromNode<'tree> for PseudoElementSelector<'tree> {
    #[allow(clippy::match_single_binding, clippy::suspicious_else_formatting)]
    fn from_node(
        node: ::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        debug_assert_eq!(node.kind(), "pseudo_element_selector");
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
                        <PseudoElementSelectorChildren as ::treesitter_types::FromNode>::from_node(
                            child, src,
                        )?,
                    );
                }
                items
            },
        })
    }
}
impl ::treesitter_types::Spanned for PseudoElementSelector<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        self.span
    }
}
#[derive(Debug, Clone)]
pub struct RuleSet<'tree> {
    pub span: ::treesitter_types::Span,
    pub children: ::std::vec::Vec<RuleSetChildren<'tree>>,
}
impl<'tree> ::treesitter_types::FromNode<'tree> for RuleSet<'tree> {
    #[allow(clippy::match_single_binding, clippy::suspicious_else_formatting)]
    fn from_node(
        node: ::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        debug_assert_eq!(node.kind(), "rule_set");
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
                        <RuleSetChildren as ::treesitter_types::FromNode>::from_node(child, src)?,
                    );
                }
                items
            },
        })
    }
}
impl ::treesitter_types::Spanned for RuleSet<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        self.span
    }
}
#[derive(Debug, Clone)]
pub struct ScopeStatement<'tree> {
    pub span: ::treesitter_types::Span,
    pub children: ::std::vec::Vec<ScopeStatementChildren<'tree>>,
}
impl<'tree> ::treesitter_types::FromNode<'tree> for ScopeStatement<'tree> {
    #[allow(clippy::match_single_binding, clippy::suspicious_else_formatting)]
    fn from_node(
        node: ::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        debug_assert_eq!(node.kind(), "scope_statement");
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
                        <ScopeStatementChildren as ::treesitter_types::FromNode>::from_node(
                            child, src,
                        )?,
                    );
                }
                items
            },
        })
    }
}
impl ::treesitter_types::Spanned for ScopeStatement<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        self.span
    }
}
#[derive(Debug, Clone)]
pub struct SelectorQuery<'tree> {
    pub span: ::treesitter_types::Span,
    pub children: SelectorQueryChildren<'tree>,
}
impl<'tree> ::treesitter_types::FromNode<'tree> for SelectorQuery<'tree> {
    #[allow(clippy::match_single_binding, clippy::suspicious_else_formatting)]
    fn from_node(
        node: ::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        debug_assert_eq!(node.kind(), "selector_query");
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
                                fallback_child = Some(fallback_cursor.node());
                                break;
                            }
                            if !fallback_cursor.goto_next_sibling() {
                                break;
                            }
                        }
                    }
                    fallback_child.ok_or_else(|| {
                        ::treesitter_types::ParseError::missing_field("children", node)
                    })?
                };
                <SelectorQueryChildren as ::treesitter_types::FromNode>::from_node(child, src)?
            },
        })
    }
}
impl ::treesitter_types::Spanned for SelectorQuery<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        self.span
    }
}
#[derive(Debug, Clone)]
pub struct Selectors<'tree> {
    pub span: ::treesitter_types::Span,
    pub children: ::std::vec::Vec<SelectorsChildren<'tree>>,
}
impl<'tree> ::treesitter_types::FromNode<'tree> for Selectors<'tree> {
    #[allow(clippy::match_single_binding, clippy::suspicious_else_formatting)]
    fn from_node(
        node: ::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        debug_assert_eq!(node.kind(), "selectors");
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
                        <SelectorsChildren as ::treesitter_types::FromNode>::from_node(child, src)?,
                    );
                }
                items
            },
        })
    }
}
impl ::treesitter_types::Spanned for Selectors<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        self.span
    }
}
#[derive(Debug, Clone)]
pub struct SiblingSelector<'tree> {
    pub span: ::treesitter_types::Span,
    pub children: ::std::vec::Vec<SiblingSelectorChildren<'tree>>,
}
impl<'tree> ::treesitter_types::FromNode<'tree> for SiblingSelector<'tree> {
    #[allow(clippy::match_single_binding, clippy::suspicious_else_formatting)]
    fn from_node(
        node: ::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        debug_assert_eq!(node.kind(), "sibling_selector");
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
                        <SiblingSelectorChildren as ::treesitter_types::FromNode>::from_node(
                            child, src,
                        )?,
                    );
                }
                items
            },
        })
    }
}
impl ::treesitter_types::Spanned for SiblingSelector<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        self.span
    }
}
#[derive(Debug, Clone)]
pub struct StringValue<'tree> {
    pub span: ::treesitter_types::Span,
    pub children: ::std::vec::Vec<StringValueChildren<'tree>>,
}
impl<'tree> ::treesitter_types::FromNode<'tree> for StringValue<'tree> {
    #[allow(clippy::match_single_binding, clippy::suspicious_else_formatting)]
    fn from_node(
        node: ::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        debug_assert_eq!(node.kind(), "string_value");
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
                        <StringValueChildren as ::treesitter_types::FromNode>::from_node(
                            child, src,
                        )?,
                    );
                }
                items
            },
        })
    }
}
impl ::treesitter_types::Spanned for StringValue<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        self.span
    }
}
#[derive(Debug, Clone)]
pub struct Stylesheet<'tree> {
    pub span: ::treesitter_types::Span,
    pub children: ::std::vec::Vec<StylesheetChildren<'tree>>,
}
impl<'tree> ::treesitter_types::FromNode<'tree> for Stylesheet<'tree> {
    #[allow(clippy::match_single_binding, clippy::suspicious_else_formatting)]
    fn from_node(
        node: ::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        debug_assert_eq!(node.kind(), "stylesheet");
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
                        <StylesheetChildren as ::treesitter_types::FromNode>::from_node(
                            child, src,
                        )?,
                    );
                }
                items
            },
        })
    }
}
impl ::treesitter_types::Spanned for Stylesheet<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        self.span
    }
}
#[derive(Debug, Clone)]
pub struct SupportsStatement<'tree> {
    pub span: ::treesitter_types::Span,
    pub children: ::std::vec::Vec<SupportsStatementChildren<'tree>>,
}
impl<'tree> ::treesitter_types::FromNode<'tree> for SupportsStatement<'tree> {
    #[allow(clippy::match_single_binding, clippy::suspicious_else_formatting)]
    fn from_node(
        node: ::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        debug_assert_eq!(node.kind(), "supports_statement");
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
                        <SupportsStatementChildren as ::treesitter_types::FromNode>::from_node(
                            child, src,
                        )?,
                    );
                }
                items
            },
        })
    }
}
impl ::treesitter_types::Spanned for SupportsStatement<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        self.span
    }
}
#[derive(Debug, Clone)]
pub struct To<'tree> {
    pub span: ::treesitter_types::Span,
    text: &'tree str,
}
impl<'tree> ::treesitter_types::FromNode<'tree> for To<'tree> {
    fn from_node(
        node: ::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        debug_assert_eq!(node.kind(), "to");
        Ok(Self {
            span: ::treesitter_types::Span::from(node),
            text: node.utf8_text(src)?,
        })
    }
}
impl<'tree> ::treesitter_types::LeafNode<'tree> for To<'tree> {
    fn text(&self) -> &'tree str {
        self.text
    }
}
impl ::treesitter_types::Spanned for To<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        self.span
    }
}
#[derive(Debug, Clone)]
pub struct UnaryQuery<'tree> {
    pub span: ::treesitter_types::Span,
    pub children: UnaryQueryChildren<'tree>,
}
impl<'tree> ::treesitter_types::FromNode<'tree> for UnaryQuery<'tree> {
    #[allow(clippy::match_single_binding, clippy::suspicious_else_formatting)]
    fn from_node(
        node: ::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        debug_assert_eq!(node.kind(), "unary_query");
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
                                fallback_child = Some(fallback_cursor.node());
                                break;
                            }
                            if !fallback_cursor.goto_next_sibling() {
                                break;
                            }
                        }
                    }
                    fallback_child.ok_or_else(|| {
                        ::treesitter_types::ParseError::missing_field("children", node)
                    })?
                };
                <UnaryQueryChildren as ::treesitter_types::FromNode>::from_node(child, src)?
            },
        })
    }
}
impl ::treesitter_types::Spanned for UnaryQuery<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        self.span
    }
}
#[derive(Debug, Clone)]
pub struct UniversalSelector<'tree> {
    pub span: ::treesitter_types::Span,
    text: &'tree str,
}
impl<'tree> ::treesitter_types::FromNode<'tree> for UniversalSelector<'tree> {
    fn from_node(
        node: ::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        debug_assert_eq!(node.kind(), "universal_selector");
        Ok(Self {
            span: ::treesitter_types::Span::from(node),
            text: node.utf8_text(src)?,
        })
    }
}
impl<'tree> ::treesitter_types::LeafNode<'tree> for UniversalSelector<'tree> {
    fn text(&self) -> &'tree str {
        self.text
    }
}
impl ::treesitter_types::Spanned for UniversalSelector<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        self.span
    }
}
#[derive(Debug, Clone)]
pub struct AtKeyword<'tree> {
    pub span: ::treesitter_types::Span,
    text: &'tree str,
}
impl<'tree> ::treesitter_types::FromNode<'tree> for AtKeyword<'tree> {
    fn from_node(
        node: ::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        debug_assert_eq!(node.kind(), "at_keyword");
        Ok(Self {
            span: ::treesitter_types::Span::from(node),
            text: node.utf8_text(src)?,
        })
    }
}
impl<'tree> ::treesitter_types::LeafNode<'tree> for AtKeyword<'tree> {
    fn text(&self) -> &'tree str {
        self.text
    }
}
impl ::treesitter_types::Spanned for AtKeyword<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        self.span
    }
}
#[derive(Debug, Clone)]
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
#[derive(Debug, Clone)]
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
#[derive(Debug, Clone)]
pub struct FeatureName<'tree> {
    pub span: ::treesitter_types::Span,
    text: &'tree str,
}
impl<'tree> ::treesitter_types::FromNode<'tree> for FeatureName<'tree> {
    fn from_node(
        node: ::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        debug_assert_eq!(node.kind(), "feature_name");
        Ok(Self {
            span: ::treesitter_types::Span::from(node),
            text: node.utf8_text(src)?,
        })
    }
}
impl<'tree> ::treesitter_types::LeafNode<'tree> for FeatureName<'tree> {
    fn text(&self) -> &'tree str {
        self.text
    }
}
impl ::treesitter_types::Spanned for FeatureName<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        self.span
    }
}
#[derive(Debug, Clone)]
pub struct From<'tree> {
    pub span: ::treesitter_types::Span,
    text: &'tree str,
}
impl<'tree> ::treesitter_types::FromNode<'tree> for From<'tree> {
    fn from_node(
        node: ::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        debug_assert_eq!(node.kind(), "from");
        Ok(Self {
            span: ::treesitter_types::Span::from(node),
            text: node.utf8_text(src)?,
        })
    }
}
impl<'tree> ::treesitter_types::LeafNode<'tree> for From<'tree> {
    fn text(&self) -> &'tree str {
        self.text
    }
}
impl ::treesitter_types::Spanned for From<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        self.span
    }
}
#[derive(Debug, Clone)]
pub struct FunctionName<'tree> {
    pub span: ::treesitter_types::Span,
    text: &'tree str,
}
impl<'tree> ::treesitter_types::FromNode<'tree> for FunctionName<'tree> {
    fn from_node(
        node: ::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        debug_assert_eq!(node.kind(), "function_name");
        Ok(Self {
            span: ::treesitter_types::Span::from(node),
            text: node.utf8_text(src)?,
        })
    }
}
impl<'tree> ::treesitter_types::LeafNode<'tree> for FunctionName<'tree> {
    fn text(&self) -> &'tree str {
        self.text
    }
}
impl ::treesitter_types::Spanned for FunctionName<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        self.span
    }
}
#[derive(Debug, Clone)]
pub struct IdName<'tree> {
    pub span: ::treesitter_types::Span,
    text: &'tree str,
}
impl<'tree> ::treesitter_types::FromNode<'tree> for IdName<'tree> {
    fn from_node(
        node: ::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        debug_assert_eq!(node.kind(), "id_name");
        Ok(Self {
            span: ::treesitter_types::Span::from(node),
            text: node.utf8_text(src)?,
        })
    }
}
impl<'tree> ::treesitter_types::LeafNode<'tree> for IdName<'tree> {
    fn text(&self) -> &'tree str {
        self.text
    }
}
impl ::treesitter_types::Spanned for IdName<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        self.span
    }
}
#[derive(Debug, Clone)]
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
#[derive(Debug, Clone)]
pub struct Important<'tree> {
    pub span: ::treesitter_types::Span,
    text: &'tree str,
}
impl<'tree> ::treesitter_types::FromNode<'tree> for Important<'tree> {
    fn from_node(
        node: ::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        debug_assert_eq!(node.kind(), "important");
        Ok(Self {
            span: ::treesitter_types::Span::from(node),
            text: node.utf8_text(src)?,
        })
    }
}
impl<'tree> ::treesitter_types::LeafNode<'tree> for Important<'tree> {
    fn text(&self) -> &'tree str {
        self.text
    }
}
impl ::treesitter_types::Spanned for Important<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        self.span
    }
}
#[derive(Debug, Clone)]
pub struct ImportantValue<'tree> {
    pub span: ::treesitter_types::Span,
    text: &'tree str,
}
impl<'tree> ::treesitter_types::FromNode<'tree> for ImportantValue<'tree> {
    fn from_node(
        node: ::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        debug_assert_eq!(node.kind(), "important_value");
        Ok(Self {
            span: ::treesitter_types::Span::from(node),
            text: node.utf8_text(src)?,
        })
    }
}
impl<'tree> ::treesitter_types::LeafNode<'tree> for ImportantValue<'tree> {
    fn text(&self) -> &'tree str {
        self.text
    }
}
impl ::treesitter_types::Spanned for ImportantValue<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        self.span
    }
}
#[derive(Debug, Clone)]
pub struct JsComment<'tree> {
    pub span: ::treesitter_types::Span,
    text: &'tree str,
}
impl<'tree> ::treesitter_types::FromNode<'tree> for JsComment<'tree> {
    fn from_node(
        node: ::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        debug_assert_eq!(node.kind(), "js_comment");
        Ok(Self {
            span: ::treesitter_types::Span::from(node),
            text: node.utf8_text(src)?,
        })
    }
}
impl<'tree> ::treesitter_types::LeafNode<'tree> for JsComment<'tree> {
    fn text(&self) -> &'tree str {
        self.text
    }
}
impl ::treesitter_types::Spanned for JsComment<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        self.span
    }
}
#[derive(Debug, Clone)]
pub struct KeyframesName<'tree> {
    pub span: ::treesitter_types::Span,
    text: &'tree str,
}
impl<'tree> ::treesitter_types::FromNode<'tree> for KeyframesName<'tree> {
    fn from_node(
        node: ::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        debug_assert_eq!(node.kind(), "keyframes_name");
        Ok(Self {
            span: ::treesitter_types::Span::from(node),
            text: node.utf8_text(src)?,
        })
    }
}
impl<'tree> ::treesitter_types::LeafNode<'tree> for KeyframesName<'tree> {
    fn text(&self) -> &'tree str {
        self.text
    }
}
impl ::treesitter_types::Spanned for KeyframesName<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        self.span
    }
}
#[derive(Debug, Clone)]
pub struct KeywordQuery<'tree> {
    pub span: ::treesitter_types::Span,
    text: &'tree str,
}
impl<'tree> ::treesitter_types::FromNode<'tree> for KeywordQuery<'tree> {
    fn from_node(
        node: ::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        debug_assert_eq!(node.kind(), "keyword_query");
        Ok(Self {
            span: ::treesitter_types::Span::from(node),
            text: node.utf8_text(src)?,
        })
    }
}
impl<'tree> ::treesitter_types::LeafNode<'tree> for KeywordQuery<'tree> {
    fn text(&self) -> &'tree str {
        self.text
    }
}
impl ::treesitter_types::Spanned for KeywordQuery<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        self.span
    }
}
#[derive(Debug, Clone)]
pub struct NamespaceName<'tree> {
    pub span: ::treesitter_types::Span,
    text: &'tree str,
}
impl<'tree> ::treesitter_types::FromNode<'tree> for NamespaceName<'tree> {
    fn from_node(
        node: ::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        debug_assert_eq!(node.kind(), "namespace_name");
        Ok(Self {
            span: ::treesitter_types::Span::from(node),
            text: node.utf8_text(src)?,
        })
    }
}
impl<'tree> ::treesitter_types::LeafNode<'tree> for NamespaceName<'tree> {
    fn text(&self) -> &'tree str {
        self.text
    }
}
impl ::treesitter_types::Spanned for NamespaceName<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        self.span
    }
}
#[derive(Debug, Clone)]
pub struct NestingSelector<'tree> {
    pub span: ::treesitter_types::Span,
    text: &'tree str,
}
impl<'tree> ::treesitter_types::FromNode<'tree> for NestingSelector<'tree> {
    fn from_node(
        node: ::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        debug_assert_eq!(node.kind(), "nesting_selector");
        Ok(Self {
            span: ::treesitter_types::Span::from(node),
            text: node.utf8_text(src)?,
        })
    }
}
impl<'tree> ::treesitter_types::LeafNode<'tree> for NestingSelector<'tree> {
    fn text(&self) -> &'tree str {
        self.text
    }
}
impl ::treesitter_types::Spanned for NestingSelector<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        self.span
    }
}
#[derive(Debug, Clone)]
pub struct PlainValue<'tree> {
    pub span: ::treesitter_types::Span,
    text: &'tree str,
}
impl<'tree> ::treesitter_types::FromNode<'tree> for PlainValue<'tree> {
    fn from_node(
        node: ::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        debug_assert_eq!(node.kind(), "plain_value");
        Ok(Self {
            span: ::treesitter_types::Span::from(node),
            text: node.utf8_text(src)?,
        })
    }
}
impl<'tree> ::treesitter_types::LeafNode<'tree> for PlainValue<'tree> {
    fn text(&self) -> &'tree str {
        self.text
    }
}
impl ::treesitter_types::Spanned for PlainValue<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        self.span
    }
}
#[derive(Debug, Clone)]
pub struct PropertyName<'tree> {
    pub span: ::treesitter_types::Span,
    text: &'tree str,
}
impl<'tree> ::treesitter_types::FromNode<'tree> for PropertyName<'tree> {
    fn from_node(
        node: ::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        debug_assert_eq!(node.kind(), "property_name");
        Ok(Self {
            span: ::treesitter_types::Span::from(node),
            text: node.utf8_text(src)?,
        })
    }
}
impl<'tree> ::treesitter_types::LeafNode<'tree> for PropertyName<'tree> {
    fn text(&self) -> &'tree str {
        self.text
    }
}
impl ::treesitter_types::Spanned for PropertyName<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        self.span
    }
}
#[derive(Debug, Clone)]
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
#[derive(Debug, Clone)]
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
#[derive(Debug, Clone)]
pub struct Unit<'tree> {
    pub span: ::treesitter_types::Span,
    text: &'tree str,
}
impl<'tree> ::treesitter_types::FromNode<'tree> for Unit<'tree> {
    fn from_node(
        node: ::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        debug_assert_eq!(node.kind(), "unit");
        Ok(Self {
            span: ::treesitter_types::Span::from(node),
            text: node.utf8_text(src)?,
        })
    }
}
impl<'tree> ::treesitter_types::LeafNode<'tree> for Unit<'tree> {
    fn text(&self) -> &'tree str {
        self.text
    }
}
impl ::treesitter_types::Spanned for Unit<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        self.span
    }
}
#[derive(Debug, Clone)]
pub enum AdjacentSiblingSelectorChildren<'tree> {
    AdjacentSiblingSelector(::std::boxed::Box<AdjacentSiblingSelector<'tree>>),
    AttributeSelector(::std::boxed::Box<AttributeSelector<'tree>>),
    ChildSelector(::std::boxed::Box<ChildSelector<'tree>>),
    ClassSelector(::std::boxed::Box<ClassSelector<'tree>>),
    DescendantSelector(::std::boxed::Box<DescendantSelector<'tree>>),
    IdSelector(::std::boxed::Box<IdSelector<'tree>>),
    NamespaceSelector(::std::boxed::Box<NamespaceSelector<'tree>>),
    NestingSelector(::std::boxed::Box<NestingSelector<'tree>>),
    PseudoClassSelector(::std::boxed::Box<PseudoClassSelector<'tree>>),
    PseudoElementSelector(::std::boxed::Box<PseudoElementSelector<'tree>>),
    SiblingSelector(::std::boxed::Box<SiblingSelector<'tree>>),
    StringValue(::std::boxed::Box<StringValue<'tree>>),
    TagName(::std::boxed::Box<TagName<'tree>>),
    UniversalSelector(::std::boxed::Box<UniversalSelector<'tree>>),
}
impl<'tree> ::treesitter_types::FromNode<'tree> for AdjacentSiblingSelectorChildren<'tree> {
    #[allow(clippy::collapsible_else_if)]
    fn from_node(
        node: ::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        match node.kind() {
            "adjacent_sibling_selector" => {
                Ok(Self::AdjacentSiblingSelector(::std::boxed::Box::new(
                    <AdjacentSiblingSelector as ::treesitter_types::FromNode>::from_node(
                        node, src,
                    )?,
                )))
            }
            "attribute_selector" => Ok(Self::AttributeSelector(::std::boxed::Box::new(
                <AttributeSelector as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "child_selector" => Ok(Self::ChildSelector(::std::boxed::Box::new(
                <ChildSelector as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "class_selector" => Ok(Self::ClassSelector(::std::boxed::Box::new(
                <ClassSelector as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "descendant_selector" => Ok(Self::DescendantSelector(::std::boxed::Box::new(
                <DescendantSelector as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "id_selector" => Ok(Self::IdSelector(::std::boxed::Box::new(
                <IdSelector as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "namespace_selector" => Ok(Self::NamespaceSelector(::std::boxed::Box::new(
                <NamespaceSelector as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "nesting_selector" => Ok(Self::NestingSelector(::std::boxed::Box::new(
                <NestingSelector as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "pseudo_class_selector" => Ok(Self::PseudoClassSelector(::std::boxed::Box::new(
                <PseudoClassSelector as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "pseudo_element_selector" => Ok(Self::PseudoElementSelector(::std::boxed::Box::new(
                <PseudoElementSelector as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "sibling_selector" => Ok(Self::SiblingSelector(::std::boxed::Box::new(
                <SiblingSelector as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "string_value" => Ok(Self::StringValue(::std::boxed::Box::new(
                <StringValue as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "tag_name" => Ok(Self::TagName(::std::boxed::Box::new(
                <TagName as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "universal_selector" => Ok(Self::UniversalSelector(::std::boxed::Box::new(
                <UniversalSelector as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            other => Err(::treesitter_types::ParseError::unexpected_kind(other, node)),
        }
    }
}
impl ::treesitter_types::Spanned for AdjacentSiblingSelectorChildren<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        match self {
            Self::AdjacentSiblingSelector(inner) => inner.span(),
            Self::AttributeSelector(inner) => inner.span(),
            Self::ChildSelector(inner) => inner.span(),
            Self::ClassSelector(inner) => inner.span(),
            Self::DescendantSelector(inner) => inner.span(),
            Self::IdSelector(inner) => inner.span(),
            Self::NamespaceSelector(inner) => inner.span(),
            Self::NestingSelector(inner) => inner.span(),
            Self::PseudoClassSelector(inner) => inner.span(),
            Self::PseudoElementSelector(inner) => inner.span(),
            Self::SiblingSelector(inner) => inner.span(),
            Self::StringValue(inner) => inner.span(),
            Self::TagName(inner) => inner.span(),
            Self::UniversalSelector(inner) => inner.span(),
        }
    }
}
#[derive(Debug, Clone)]
pub enum ArgumentsChildren<'tree> {
    AdjacentSiblingSelector(::std::boxed::Box<AdjacentSiblingSelector<'tree>>),
    AttributeSelector(::std::boxed::Box<AttributeSelector<'tree>>),
    BinaryExpression(::std::boxed::Box<BinaryExpression<'tree>>),
    CallExpression(::std::boxed::Box<CallExpression<'tree>>),
    ChildSelector(::std::boxed::Box<ChildSelector<'tree>>),
    ClassSelector(::std::boxed::Box<ClassSelector<'tree>>),
    ColorValue(::std::boxed::Box<ColorValue<'tree>>),
    DescendantSelector(::std::boxed::Box<DescendantSelector<'tree>>),
    FloatValue(::std::boxed::Box<FloatValue<'tree>>),
    GridValue(::std::boxed::Box<GridValue<'tree>>),
    IdSelector(::std::boxed::Box<IdSelector<'tree>>),
    Important(::std::boxed::Box<Important<'tree>>),
    IntegerValue(::std::boxed::Box<IntegerValue<'tree>>),
    NamespaceSelector(::std::boxed::Box<NamespaceSelector<'tree>>),
    NestingSelector(::std::boxed::Box<NestingSelector<'tree>>),
    ParenthesizedValue(::std::boxed::Box<ParenthesizedValue<'tree>>),
    PlainValue(::std::boxed::Box<PlainValue<'tree>>),
    PseudoClassSelector(::std::boxed::Box<PseudoClassSelector<'tree>>),
    PseudoElementSelector(::std::boxed::Box<PseudoElementSelector<'tree>>),
    SiblingSelector(::std::boxed::Box<SiblingSelector<'tree>>),
    StringValue(::std::boxed::Box<StringValue<'tree>>),
    TagName(::std::boxed::Box<TagName<'tree>>),
    UniversalSelector(::std::boxed::Box<UniversalSelector<'tree>>),
}
impl<'tree> ::treesitter_types::FromNode<'tree> for ArgumentsChildren<'tree> {
    #[allow(clippy::collapsible_else_if)]
    fn from_node(
        node: ::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        match node.kind() {
            "adjacent_sibling_selector" => {
                Ok(Self::AdjacentSiblingSelector(::std::boxed::Box::new(
                    <AdjacentSiblingSelector as ::treesitter_types::FromNode>::from_node(
                        node, src,
                    )?,
                )))
            }
            "attribute_selector" => Ok(Self::AttributeSelector(::std::boxed::Box::new(
                <AttributeSelector as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "binary_expression" => Ok(Self::BinaryExpression(::std::boxed::Box::new(
                <BinaryExpression as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "call_expression" => Ok(Self::CallExpression(::std::boxed::Box::new(
                <CallExpression as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "child_selector" => Ok(Self::ChildSelector(::std::boxed::Box::new(
                <ChildSelector as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "class_selector" => Ok(Self::ClassSelector(::std::boxed::Box::new(
                <ClassSelector as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "color_value" => Ok(Self::ColorValue(::std::boxed::Box::new(
                <ColorValue as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "descendant_selector" => Ok(Self::DescendantSelector(::std::boxed::Box::new(
                <DescendantSelector as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "float_value" => Ok(Self::FloatValue(::std::boxed::Box::new(
                <FloatValue as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "grid_value" => Ok(Self::GridValue(::std::boxed::Box::new(
                <GridValue as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "id_selector" => Ok(Self::IdSelector(::std::boxed::Box::new(
                <IdSelector as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "important" => Ok(Self::Important(::std::boxed::Box::new(
                <Important as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "integer_value" => Ok(Self::IntegerValue(::std::boxed::Box::new(
                <IntegerValue as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "namespace_selector" => Ok(Self::NamespaceSelector(::std::boxed::Box::new(
                <NamespaceSelector as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "nesting_selector" => Ok(Self::NestingSelector(::std::boxed::Box::new(
                <NestingSelector as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "parenthesized_value" => Ok(Self::ParenthesizedValue(::std::boxed::Box::new(
                <ParenthesizedValue as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "plain_value" => Ok(Self::PlainValue(::std::boxed::Box::new(
                <PlainValue as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "pseudo_class_selector" => Ok(Self::PseudoClassSelector(::std::boxed::Box::new(
                <PseudoClassSelector as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "pseudo_element_selector" => Ok(Self::PseudoElementSelector(::std::boxed::Box::new(
                <PseudoElementSelector as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "sibling_selector" => Ok(Self::SiblingSelector(::std::boxed::Box::new(
                <SiblingSelector as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "string_value" => Ok(Self::StringValue(::std::boxed::Box::new(
                <StringValue as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "tag_name" => Ok(Self::TagName(::std::boxed::Box::new(
                <TagName as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "universal_selector" => Ok(Self::UniversalSelector(::std::boxed::Box::new(
                <UniversalSelector as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            other => Err(::treesitter_types::ParseError::unexpected_kind(other, node)),
        }
    }
}
impl ::treesitter_types::Spanned for ArgumentsChildren<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        match self {
            Self::AdjacentSiblingSelector(inner) => inner.span(),
            Self::AttributeSelector(inner) => inner.span(),
            Self::BinaryExpression(inner) => inner.span(),
            Self::CallExpression(inner) => inner.span(),
            Self::ChildSelector(inner) => inner.span(),
            Self::ClassSelector(inner) => inner.span(),
            Self::ColorValue(inner) => inner.span(),
            Self::DescendantSelector(inner) => inner.span(),
            Self::FloatValue(inner) => inner.span(),
            Self::GridValue(inner) => inner.span(),
            Self::IdSelector(inner) => inner.span(),
            Self::Important(inner) => inner.span(),
            Self::IntegerValue(inner) => inner.span(),
            Self::NamespaceSelector(inner) => inner.span(),
            Self::NestingSelector(inner) => inner.span(),
            Self::ParenthesizedValue(inner) => inner.span(),
            Self::PlainValue(inner) => inner.span(),
            Self::PseudoClassSelector(inner) => inner.span(),
            Self::PseudoElementSelector(inner) => inner.span(),
            Self::SiblingSelector(inner) => inner.span(),
            Self::StringValue(inner) => inner.span(),
            Self::TagName(inner) => inner.span(),
            Self::UniversalSelector(inner) => inner.span(),
        }
    }
}
#[derive(Debug, Clone)]
pub enum AtRuleChildren<'tree> {
    AtKeyword(::std::boxed::Box<AtKeyword<'tree>>),
    BinaryQuery(::std::boxed::Box<BinaryQuery<'tree>>),
    Block(::std::boxed::Box<Block<'tree>>),
    FeatureQuery(::std::boxed::Box<FeatureQuery<'tree>>),
    KeywordQuery(::std::boxed::Box<KeywordQuery<'tree>>),
    ParenthesizedQuery(::std::boxed::Box<ParenthesizedQuery<'tree>>),
    SelectorQuery(::std::boxed::Box<SelectorQuery<'tree>>),
    UnaryQuery(::std::boxed::Box<UnaryQuery<'tree>>),
}
impl<'tree> ::treesitter_types::FromNode<'tree> for AtRuleChildren<'tree> {
    #[allow(clippy::collapsible_else_if)]
    fn from_node(
        node: ::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        match node.kind() {
            "at_keyword" => Ok(Self::AtKeyword(::std::boxed::Box::new(
                <AtKeyword as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "binary_query" => Ok(Self::BinaryQuery(::std::boxed::Box::new(
                <BinaryQuery as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "block" => Ok(Self::Block(::std::boxed::Box::new(
                <Block as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "feature_query" => Ok(Self::FeatureQuery(::std::boxed::Box::new(
                <FeatureQuery as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "keyword_query" => Ok(Self::KeywordQuery(::std::boxed::Box::new(
                <KeywordQuery as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "parenthesized_query" => Ok(Self::ParenthesizedQuery(::std::boxed::Box::new(
                <ParenthesizedQuery as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "selector_query" => Ok(Self::SelectorQuery(::std::boxed::Box::new(
                <SelectorQuery as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "unary_query" => Ok(Self::UnaryQuery(::std::boxed::Box::new(
                <UnaryQuery as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            other => Err(::treesitter_types::ParseError::unexpected_kind(other, node)),
        }
    }
}
impl ::treesitter_types::Spanned for AtRuleChildren<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        match self {
            Self::AtKeyword(inner) => inner.span(),
            Self::BinaryQuery(inner) => inner.span(),
            Self::Block(inner) => inner.span(),
            Self::FeatureQuery(inner) => inner.span(),
            Self::KeywordQuery(inner) => inner.span(),
            Self::ParenthesizedQuery(inner) => inner.span(),
            Self::SelectorQuery(inner) => inner.span(),
            Self::UnaryQuery(inner) => inner.span(),
        }
    }
}
#[derive(Debug, Clone)]
pub enum AttributeNameChildren<'tree> {
    AdjacentSiblingSelector(::std::boxed::Box<AdjacentSiblingSelector<'tree>>),
    AttributeSelector(::std::boxed::Box<AttributeSelector<'tree>>),
    ChildSelector(::std::boxed::Box<ChildSelector<'tree>>),
    ClassSelector(::std::boxed::Box<ClassSelector<'tree>>),
    DescendantSelector(::std::boxed::Box<DescendantSelector<'tree>>),
    IdSelector(::std::boxed::Box<IdSelector<'tree>>),
    NamespaceSelector(::std::boxed::Box<NamespaceSelector<'tree>>),
    NestingSelector(::std::boxed::Box<NestingSelector<'tree>>),
    PseudoClassSelector(::std::boxed::Box<PseudoClassSelector<'tree>>),
    PseudoElementSelector(::std::boxed::Box<PseudoElementSelector<'tree>>),
    SiblingSelector(::std::boxed::Box<SiblingSelector<'tree>>),
    StringValue(::std::boxed::Box<StringValue<'tree>>),
    TagName(::std::boxed::Box<TagName<'tree>>),
    UniversalSelector(::std::boxed::Box<UniversalSelector<'tree>>),
}
impl<'tree> ::treesitter_types::FromNode<'tree> for AttributeNameChildren<'tree> {
    #[allow(clippy::collapsible_else_if)]
    fn from_node(
        node: ::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        match node.kind() {
            "adjacent_sibling_selector" => {
                Ok(Self::AdjacentSiblingSelector(::std::boxed::Box::new(
                    <AdjacentSiblingSelector as ::treesitter_types::FromNode>::from_node(
                        node, src,
                    )?,
                )))
            }
            "attribute_selector" => Ok(Self::AttributeSelector(::std::boxed::Box::new(
                <AttributeSelector as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "child_selector" => Ok(Self::ChildSelector(::std::boxed::Box::new(
                <ChildSelector as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "class_selector" => Ok(Self::ClassSelector(::std::boxed::Box::new(
                <ClassSelector as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "descendant_selector" => Ok(Self::DescendantSelector(::std::boxed::Box::new(
                <DescendantSelector as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "id_selector" => Ok(Self::IdSelector(::std::boxed::Box::new(
                <IdSelector as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "namespace_selector" => Ok(Self::NamespaceSelector(::std::boxed::Box::new(
                <NamespaceSelector as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "nesting_selector" => Ok(Self::NestingSelector(::std::boxed::Box::new(
                <NestingSelector as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "pseudo_class_selector" => Ok(Self::PseudoClassSelector(::std::boxed::Box::new(
                <PseudoClassSelector as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "pseudo_element_selector" => Ok(Self::PseudoElementSelector(::std::boxed::Box::new(
                <PseudoElementSelector as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "sibling_selector" => Ok(Self::SiblingSelector(::std::boxed::Box::new(
                <SiblingSelector as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "string_value" => Ok(Self::StringValue(::std::boxed::Box::new(
                <StringValue as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "tag_name" => Ok(Self::TagName(::std::boxed::Box::new(
                <TagName as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "universal_selector" => Ok(Self::UniversalSelector(::std::boxed::Box::new(
                <UniversalSelector as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            other => Err(::treesitter_types::ParseError::unexpected_kind(other, node)),
        }
    }
}
impl ::treesitter_types::Spanned for AttributeNameChildren<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        match self {
            Self::AdjacentSiblingSelector(inner) => inner.span(),
            Self::AttributeSelector(inner) => inner.span(),
            Self::ChildSelector(inner) => inner.span(),
            Self::ClassSelector(inner) => inner.span(),
            Self::DescendantSelector(inner) => inner.span(),
            Self::IdSelector(inner) => inner.span(),
            Self::NamespaceSelector(inner) => inner.span(),
            Self::NestingSelector(inner) => inner.span(),
            Self::PseudoClassSelector(inner) => inner.span(),
            Self::PseudoElementSelector(inner) => inner.span(),
            Self::SiblingSelector(inner) => inner.span(),
            Self::StringValue(inner) => inner.span(),
            Self::TagName(inner) => inner.span(),
            Self::UniversalSelector(inner) => inner.span(),
        }
    }
}
#[derive(Debug, Clone)]
pub enum AttributeSelectorChildren<'tree> {
    AdjacentSiblingSelector(::std::boxed::Box<AdjacentSiblingSelector<'tree>>),
    AttributeName(::std::boxed::Box<AttributeName<'tree>>),
    AttributeSelector(::std::boxed::Box<AttributeSelector<'tree>>),
    BinaryExpression(::std::boxed::Box<BinaryExpression<'tree>>),
    CallExpression(::std::boxed::Box<CallExpression<'tree>>),
    ChildSelector(::std::boxed::Box<ChildSelector<'tree>>),
    ClassSelector(::std::boxed::Box<ClassSelector<'tree>>),
    ColorValue(::std::boxed::Box<ColorValue<'tree>>),
    DescendantSelector(::std::boxed::Box<DescendantSelector<'tree>>),
    FloatValue(::std::boxed::Box<FloatValue<'tree>>),
    GridValue(::std::boxed::Box<GridValue<'tree>>),
    IdSelector(::std::boxed::Box<IdSelector<'tree>>),
    Important(::std::boxed::Box<Important<'tree>>),
    IntegerValue(::std::boxed::Box<IntegerValue<'tree>>),
    NamespaceSelector(::std::boxed::Box<NamespaceSelector<'tree>>),
    NestingSelector(::std::boxed::Box<NestingSelector<'tree>>),
    ParenthesizedValue(::std::boxed::Box<ParenthesizedValue<'tree>>),
    PlainValue(::std::boxed::Box<PlainValue<'tree>>),
    PseudoClassSelector(::std::boxed::Box<PseudoClassSelector<'tree>>),
    PseudoElementSelector(::std::boxed::Box<PseudoElementSelector<'tree>>),
    SiblingSelector(::std::boxed::Box<SiblingSelector<'tree>>),
    StringValue(::std::boxed::Box<StringValue<'tree>>),
    TagName(::std::boxed::Box<TagName<'tree>>),
    UniversalSelector(::std::boxed::Box<UniversalSelector<'tree>>),
}
impl<'tree> ::treesitter_types::FromNode<'tree> for AttributeSelectorChildren<'tree> {
    #[allow(clippy::collapsible_else_if)]
    fn from_node(
        node: ::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        match node.kind() {
            "adjacent_sibling_selector" => {
                Ok(Self::AdjacentSiblingSelector(::std::boxed::Box::new(
                    <AdjacentSiblingSelector as ::treesitter_types::FromNode>::from_node(
                        node, src,
                    )?,
                )))
            }
            "attribute_name" => Ok(Self::AttributeName(::std::boxed::Box::new(
                <AttributeName as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "attribute_selector" => Ok(Self::AttributeSelector(::std::boxed::Box::new(
                <AttributeSelector as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "binary_expression" => Ok(Self::BinaryExpression(::std::boxed::Box::new(
                <BinaryExpression as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "call_expression" => Ok(Self::CallExpression(::std::boxed::Box::new(
                <CallExpression as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "child_selector" => Ok(Self::ChildSelector(::std::boxed::Box::new(
                <ChildSelector as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "class_selector" => Ok(Self::ClassSelector(::std::boxed::Box::new(
                <ClassSelector as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "color_value" => Ok(Self::ColorValue(::std::boxed::Box::new(
                <ColorValue as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "descendant_selector" => Ok(Self::DescendantSelector(::std::boxed::Box::new(
                <DescendantSelector as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "float_value" => Ok(Self::FloatValue(::std::boxed::Box::new(
                <FloatValue as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "grid_value" => Ok(Self::GridValue(::std::boxed::Box::new(
                <GridValue as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "id_selector" => Ok(Self::IdSelector(::std::boxed::Box::new(
                <IdSelector as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "important" => Ok(Self::Important(::std::boxed::Box::new(
                <Important as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "integer_value" => Ok(Self::IntegerValue(::std::boxed::Box::new(
                <IntegerValue as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "namespace_selector" => Ok(Self::NamespaceSelector(::std::boxed::Box::new(
                <NamespaceSelector as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "nesting_selector" => Ok(Self::NestingSelector(::std::boxed::Box::new(
                <NestingSelector as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "parenthesized_value" => Ok(Self::ParenthesizedValue(::std::boxed::Box::new(
                <ParenthesizedValue as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "plain_value" => Ok(Self::PlainValue(::std::boxed::Box::new(
                <PlainValue as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "pseudo_class_selector" => Ok(Self::PseudoClassSelector(::std::boxed::Box::new(
                <PseudoClassSelector as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "pseudo_element_selector" => Ok(Self::PseudoElementSelector(::std::boxed::Box::new(
                <PseudoElementSelector as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "sibling_selector" => Ok(Self::SiblingSelector(::std::boxed::Box::new(
                <SiblingSelector as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "string_value" => Ok(Self::StringValue(::std::boxed::Box::new(
                <StringValue as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "tag_name" => Ok(Self::TagName(::std::boxed::Box::new(
                <TagName as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "universal_selector" => Ok(Self::UniversalSelector(::std::boxed::Box::new(
                <UniversalSelector as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            other => Err(::treesitter_types::ParseError::unexpected_kind(other, node)),
        }
    }
}
impl ::treesitter_types::Spanned for AttributeSelectorChildren<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        match self {
            Self::AdjacentSiblingSelector(inner) => inner.span(),
            Self::AttributeName(inner) => inner.span(),
            Self::AttributeSelector(inner) => inner.span(),
            Self::BinaryExpression(inner) => inner.span(),
            Self::CallExpression(inner) => inner.span(),
            Self::ChildSelector(inner) => inner.span(),
            Self::ClassSelector(inner) => inner.span(),
            Self::ColorValue(inner) => inner.span(),
            Self::DescendantSelector(inner) => inner.span(),
            Self::FloatValue(inner) => inner.span(),
            Self::GridValue(inner) => inner.span(),
            Self::IdSelector(inner) => inner.span(),
            Self::Important(inner) => inner.span(),
            Self::IntegerValue(inner) => inner.span(),
            Self::NamespaceSelector(inner) => inner.span(),
            Self::NestingSelector(inner) => inner.span(),
            Self::ParenthesizedValue(inner) => inner.span(),
            Self::PlainValue(inner) => inner.span(),
            Self::PseudoClassSelector(inner) => inner.span(),
            Self::PseudoElementSelector(inner) => inner.span(),
            Self::SiblingSelector(inner) => inner.span(),
            Self::StringValue(inner) => inner.span(),
            Self::TagName(inner) => inner.span(),
            Self::UniversalSelector(inner) => inner.span(),
        }
    }
}
#[derive(Debug, Clone)]
pub enum BinaryExpressionChildren<'tree> {
    BinaryExpression(::std::boxed::Box<BinaryExpression<'tree>>),
    CallExpression(::std::boxed::Box<CallExpression<'tree>>),
    ColorValue(::std::boxed::Box<ColorValue<'tree>>),
    FloatValue(::std::boxed::Box<FloatValue<'tree>>),
    GridValue(::std::boxed::Box<GridValue<'tree>>),
    Important(::std::boxed::Box<Important<'tree>>),
    IntegerValue(::std::boxed::Box<IntegerValue<'tree>>),
    ParenthesizedValue(::std::boxed::Box<ParenthesizedValue<'tree>>),
    PlainValue(::std::boxed::Box<PlainValue<'tree>>),
    StringValue(::std::boxed::Box<StringValue<'tree>>),
}
impl<'tree> ::treesitter_types::FromNode<'tree> for BinaryExpressionChildren<'tree> {
    #[allow(clippy::collapsible_else_if)]
    fn from_node(
        node: ::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        match node.kind() {
            "binary_expression" => Ok(Self::BinaryExpression(::std::boxed::Box::new(
                <BinaryExpression as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "call_expression" => Ok(Self::CallExpression(::std::boxed::Box::new(
                <CallExpression as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "color_value" => Ok(Self::ColorValue(::std::boxed::Box::new(
                <ColorValue as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "float_value" => Ok(Self::FloatValue(::std::boxed::Box::new(
                <FloatValue as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "grid_value" => Ok(Self::GridValue(::std::boxed::Box::new(
                <GridValue as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "important" => Ok(Self::Important(::std::boxed::Box::new(
                <Important as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "integer_value" => Ok(Self::IntegerValue(::std::boxed::Box::new(
                <IntegerValue as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "parenthesized_value" => Ok(Self::ParenthesizedValue(::std::boxed::Box::new(
                <ParenthesizedValue as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "plain_value" => Ok(Self::PlainValue(::std::boxed::Box::new(
                <PlainValue as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "string_value" => Ok(Self::StringValue(::std::boxed::Box::new(
                <StringValue as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            other => Err(::treesitter_types::ParseError::unexpected_kind(other, node)),
        }
    }
}
impl ::treesitter_types::Spanned for BinaryExpressionChildren<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        match self {
            Self::BinaryExpression(inner) => inner.span(),
            Self::CallExpression(inner) => inner.span(),
            Self::ColorValue(inner) => inner.span(),
            Self::FloatValue(inner) => inner.span(),
            Self::GridValue(inner) => inner.span(),
            Self::Important(inner) => inner.span(),
            Self::IntegerValue(inner) => inner.span(),
            Self::ParenthesizedValue(inner) => inner.span(),
            Self::PlainValue(inner) => inner.span(),
            Self::StringValue(inner) => inner.span(),
        }
    }
}
#[derive(Debug, Clone)]
pub enum BinaryQueryChildren<'tree> {
    BinaryQuery(::std::boxed::Box<BinaryQuery<'tree>>),
    FeatureQuery(::std::boxed::Box<FeatureQuery<'tree>>),
    KeywordQuery(::std::boxed::Box<KeywordQuery<'tree>>),
    ParenthesizedQuery(::std::boxed::Box<ParenthesizedQuery<'tree>>),
    SelectorQuery(::std::boxed::Box<SelectorQuery<'tree>>),
    UnaryQuery(::std::boxed::Box<UnaryQuery<'tree>>),
}
impl<'tree> ::treesitter_types::FromNode<'tree> for BinaryQueryChildren<'tree> {
    #[allow(clippy::collapsible_else_if)]
    fn from_node(
        node: ::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        match node.kind() {
            "binary_query" => Ok(Self::BinaryQuery(::std::boxed::Box::new(
                <BinaryQuery as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "feature_query" => Ok(Self::FeatureQuery(::std::boxed::Box::new(
                <FeatureQuery as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "keyword_query" => Ok(Self::KeywordQuery(::std::boxed::Box::new(
                <KeywordQuery as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "parenthesized_query" => Ok(Self::ParenthesizedQuery(::std::boxed::Box::new(
                <ParenthesizedQuery as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "selector_query" => Ok(Self::SelectorQuery(::std::boxed::Box::new(
                <SelectorQuery as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "unary_query" => Ok(Self::UnaryQuery(::std::boxed::Box::new(
                <UnaryQuery as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            other => Err(::treesitter_types::ParseError::unexpected_kind(other, node)),
        }
    }
}
impl ::treesitter_types::Spanned for BinaryQueryChildren<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        match self {
            Self::BinaryQuery(inner) => inner.span(),
            Self::FeatureQuery(inner) => inner.span(),
            Self::KeywordQuery(inner) => inner.span(),
            Self::ParenthesizedQuery(inner) => inner.span(),
            Self::SelectorQuery(inner) => inner.span(),
            Self::UnaryQuery(inner) => inner.span(),
        }
    }
}
#[derive(Debug, Clone)]
pub enum BlockChildren<'tree> {
    AtRule(::std::boxed::Box<AtRule<'tree>>),
    CharsetStatement(::std::boxed::Box<CharsetStatement<'tree>>),
    Declaration(::std::boxed::Box<Declaration<'tree>>),
    ImportStatement(::std::boxed::Box<ImportStatement<'tree>>),
    KeyframesStatement(::std::boxed::Box<KeyframesStatement<'tree>>),
    MediaStatement(::std::boxed::Box<MediaStatement<'tree>>),
    NamespaceStatement(::std::boxed::Box<NamespaceStatement<'tree>>),
    PostcssStatement(::std::boxed::Box<PostcssStatement<'tree>>),
    RuleSet(::std::boxed::Box<RuleSet<'tree>>),
    ScopeStatement(::std::boxed::Box<ScopeStatement<'tree>>),
    SupportsStatement(::std::boxed::Box<SupportsStatement<'tree>>),
}
impl<'tree> ::treesitter_types::FromNode<'tree> for BlockChildren<'tree> {
    #[allow(clippy::collapsible_else_if)]
    fn from_node(
        node: ::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        match node.kind() {
            "at_rule" => Ok(Self::AtRule(::std::boxed::Box::new(
                <AtRule as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "charset_statement" => Ok(Self::CharsetStatement(::std::boxed::Box::new(
                <CharsetStatement as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "declaration" => Ok(Self::Declaration(::std::boxed::Box::new(
                <Declaration as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "import_statement" => Ok(Self::ImportStatement(::std::boxed::Box::new(
                <ImportStatement as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "keyframes_statement" => Ok(Self::KeyframesStatement(::std::boxed::Box::new(
                <KeyframesStatement as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "media_statement" => Ok(Self::MediaStatement(::std::boxed::Box::new(
                <MediaStatement as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "namespace_statement" => Ok(Self::NamespaceStatement(::std::boxed::Box::new(
                <NamespaceStatement as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "postcss_statement" => Ok(Self::PostcssStatement(::std::boxed::Box::new(
                <PostcssStatement as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "rule_set" => Ok(Self::RuleSet(::std::boxed::Box::new(
                <RuleSet as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "scope_statement" => Ok(Self::ScopeStatement(::std::boxed::Box::new(
                <ScopeStatement as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "supports_statement" => Ok(Self::SupportsStatement(::std::boxed::Box::new(
                <SupportsStatement as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            other => Err(::treesitter_types::ParseError::unexpected_kind(other, node)),
        }
    }
}
impl ::treesitter_types::Spanned for BlockChildren<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        match self {
            Self::AtRule(inner) => inner.span(),
            Self::CharsetStatement(inner) => inner.span(),
            Self::Declaration(inner) => inner.span(),
            Self::ImportStatement(inner) => inner.span(),
            Self::KeyframesStatement(inner) => inner.span(),
            Self::MediaStatement(inner) => inner.span(),
            Self::NamespaceStatement(inner) => inner.span(),
            Self::PostcssStatement(inner) => inner.span(),
            Self::RuleSet(inner) => inner.span(),
            Self::ScopeStatement(inner) => inner.span(),
            Self::SupportsStatement(inner) => inner.span(),
        }
    }
}
#[derive(Debug, Clone)]
pub enum CallExpressionChildren<'tree> {
    Arguments(::std::boxed::Box<Arguments<'tree>>),
    FunctionName(::std::boxed::Box<FunctionName<'tree>>),
}
impl<'tree> ::treesitter_types::FromNode<'tree> for CallExpressionChildren<'tree> {
    #[allow(clippy::collapsible_else_if)]
    fn from_node(
        node: ::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        match node.kind() {
            "arguments" => Ok(Self::Arguments(::std::boxed::Box::new(
                <Arguments as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "function_name" => Ok(Self::FunctionName(::std::boxed::Box::new(
                <FunctionName as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            other => Err(::treesitter_types::ParseError::unexpected_kind(other, node)),
        }
    }
}
impl ::treesitter_types::Spanned for CallExpressionChildren<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        match self {
            Self::Arguments(inner) => inner.span(),
            Self::FunctionName(inner) => inner.span(),
        }
    }
}
#[derive(Debug, Clone)]
pub enum CharsetStatementChildren<'tree> {
    BinaryExpression(::std::boxed::Box<BinaryExpression<'tree>>),
    CallExpression(::std::boxed::Box<CallExpression<'tree>>),
    ColorValue(::std::boxed::Box<ColorValue<'tree>>),
    FloatValue(::std::boxed::Box<FloatValue<'tree>>),
    GridValue(::std::boxed::Box<GridValue<'tree>>),
    Important(::std::boxed::Box<Important<'tree>>),
    IntegerValue(::std::boxed::Box<IntegerValue<'tree>>),
    ParenthesizedValue(::std::boxed::Box<ParenthesizedValue<'tree>>),
    PlainValue(::std::boxed::Box<PlainValue<'tree>>),
    StringValue(::std::boxed::Box<StringValue<'tree>>),
}
impl<'tree> ::treesitter_types::FromNode<'tree> for CharsetStatementChildren<'tree> {
    #[allow(clippy::collapsible_else_if)]
    fn from_node(
        node: ::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        match node.kind() {
            "binary_expression" => Ok(Self::BinaryExpression(::std::boxed::Box::new(
                <BinaryExpression as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "call_expression" => Ok(Self::CallExpression(::std::boxed::Box::new(
                <CallExpression as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "color_value" => Ok(Self::ColorValue(::std::boxed::Box::new(
                <ColorValue as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "float_value" => Ok(Self::FloatValue(::std::boxed::Box::new(
                <FloatValue as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "grid_value" => Ok(Self::GridValue(::std::boxed::Box::new(
                <GridValue as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "important" => Ok(Self::Important(::std::boxed::Box::new(
                <Important as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "integer_value" => Ok(Self::IntegerValue(::std::boxed::Box::new(
                <IntegerValue as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "parenthesized_value" => Ok(Self::ParenthesizedValue(::std::boxed::Box::new(
                <ParenthesizedValue as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "plain_value" => Ok(Self::PlainValue(::std::boxed::Box::new(
                <PlainValue as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "string_value" => Ok(Self::StringValue(::std::boxed::Box::new(
                <StringValue as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            other => Err(::treesitter_types::ParseError::unexpected_kind(other, node)),
        }
    }
}
impl ::treesitter_types::Spanned for CharsetStatementChildren<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        match self {
            Self::BinaryExpression(inner) => inner.span(),
            Self::CallExpression(inner) => inner.span(),
            Self::ColorValue(inner) => inner.span(),
            Self::FloatValue(inner) => inner.span(),
            Self::GridValue(inner) => inner.span(),
            Self::Important(inner) => inner.span(),
            Self::IntegerValue(inner) => inner.span(),
            Self::ParenthesizedValue(inner) => inner.span(),
            Self::PlainValue(inner) => inner.span(),
            Self::StringValue(inner) => inner.span(),
        }
    }
}
#[derive(Debug, Clone)]
pub enum ChildSelectorChildren<'tree> {
    AdjacentSiblingSelector(::std::boxed::Box<AdjacentSiblingSelector<'tree>>),
    AttributeSelector(::std::boxed::Box<AttributeSelector<'tree>>),
    ChildSelector(::std::boxed::Box<ChildSelector<'tree>>),
    ClassSelector(::std::boxed::Box<ClassSelector<'tree>>),
    DescendantSelector(::std::boxed::Box<DescendantSelector<'tree>>),
    IdSelector(::std::boxed::Box<IdSelector<'tree>>),
    NamespaceSelector(::std::boxed::Box<NamespaceSelector<'tree>>),
    NestingSelector(::std::boxed::Box<NestingSelector<'tree>>),
    PseudoClassSelector(::std::boxed::Box<PseudoClassSelector<'tree>>),
    PseudoElementSelector(::std::boxed::Box<PseudoElementSelector<'tree>>),
    SiblingSelector(::std::boxed::Box<SiblingSelector<'tree>>),
    StringValue(::std::boxed::Box<StringValue<'tree>>),
    TagName(::std::boxed::Box<TagName<'tree>>),
    UniversalSelector(::std::boxed::Box<UniversalSelector<'tree>>),
}
impl<'tree> ::treesitter_types::FromNode<'tree> for ChildSelectorChildren<'tree> {
    #[allow(clippy::collapsible_else_if)]
    fn from_node(
        node: ::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        match node.kind() {
            "adjacent_sibling_selector" => {
                Ok(Self::AdjacentSiblingSelector(::std::boxed::Box::new(
                    <AdjacentSiblingSelector as ::treesitter_types::FromNode>::from_node(
                        node, src,
                    )?,
                )))
            }
            "attribute_selector" => Ok(Self::AttributeSelector(::std::boxed::Box::new(
                <AttributeSelector as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "child_selector" => Ok(Self::ChildSelector(::std::boxed::Box::new(
                <ChildSelector as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "class_selector" => Ok(Self::ClassSelector(::std::boxed::Box::new(
                <ClassSelector as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "descendant_selector" => Ok(Self::DescendantSelector(::std::boxed::Box::new(
                <DescendantSelector as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "id_selector" => Ok(Self::IdSelector(::std::boxed::Box::new(
                <IdSelector as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "namespace_selector" => Ok(Self::NamespaceSelector(::std::boxed::Box::new(
                <NamespaceSelector as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "nesting_selector" => Ok(Self::NestingSelector(::std::boxed::Box::new(
                <NestingSelector as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "pseudo_class_selector" => Ok(Self::PseudoClassSelector(::std::boxed::Box::new(
                <PseudoClassSelector as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "pseudo_element_selector" => Ok(Self::PseudoElementSelector(::std::boxed::Box::new(
                <PseudoElementSelector as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "sibling_selector" => Ok(Self::SiblingSelector(::std::boxed::Box::new(
                <SiblingSelector as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "string_value" => Ok(Self::StringValue(::std::boxed::Box::new(
                <StringValue as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "tag_name" => Ok(Self::TagName(::std::boxed::Box::new(
                <TagName as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "universal_selector" => Ok(Self::UniversalSelector(::std::boxed::Box::new(
                <UniversalSelector as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            other => Err(::treesitter_types::ParseError::unexpected_kind(other, node)),
        }
    }
}
impl ::treesitter_types::Spanned for ChildSelectorChildren<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        match self {
            Self::AdjacentSiblingSelector(inner) => inner.span(),
            Self::AttributeSelector(inner) => inner.span(),
            Self::ChildSelector(inner) => inner.span(),
            Self::ClassSelector(inner) => inner.span(),
            Self::DescendantSelector(inner) => inner.span(),
            Self::IdSelector(inner) => inner.span(),
            Self::NamespaceSelector(inner) => inner.span(),
            Self::NestingSelector(inner) => inner.span(),
            Self::PseudoClassSelector(inner) => inner.span(),
            Self::PseudoElementSelector(inner) => inner.span(),
            Self::SiblingSelector(inner) => inner.span(),
            Self::StringValue(inner) => inner.span(),
            Self::TagName(inner) => inner.span(),
            Self::UniversalSelector(inner) => inner.span(),
        }
    }
}
#[derive(Debug, Clone)]
pub enum ClassNameChildren<'tree> {
    EscapeSequence(::std::boxed::Box<EscapeSequence<'tree>>),
    Identifier(::std::boxed::Box<Identifier<'tree>>),
}
impl<'tree> ::treesitter_types::FromNode<'tree> for ClassNameChildren<'tree> {
    #[allow(clippy::collapsible_else_if)]
    fn from_node(
        node: ::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        match node.kind() {
            "escape_sequence" => Ok(Self::EscapeSequence(::std::boxed::Box::new(
                <EscapeSequence as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "identifier" => Ok(Self::Identifier(::std::boxed::Box::new(
                <Identifier as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            other => Err(::treesitter_types::ParseError::unexpected_kind(other, node)),
        }
    }
}
impl ::treesitter_types::Spanned for ClassNameChildren<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        match self {
            Self::EscapeSequence(inner) => inner.span(),
            Self::Identifier(inner) => inner.span(),
        }
    }
}
#[derive(Debug, Clone)]
pub enum ClassSelectorChildren<'tree> {
    AdjacentSiblingSelector(::std::boxed::Box<AdjacentSiblingSelector<'tree>>),
    AttributeSelector(::std::boxed::Box<AttributeSelector<'tree>>),
    ChildSelector(::std::boxed::Box<ChildSelector<'tree>>),
    ClassName(::std::boxed::Box<ClassName<'tree>>),
    ClassSelector(::std::boxed::Box<ClassSelector<'tree>>),
    DescendantSelector(::std::boxed::Box<DescendantSelector<'tree>>),
    IdSelector(::std::boxed::Box<IdSelector<'tree>>),
    NamespaceSelector(::std::boxed::Box<NamespaceSelector<'tree>>),
    NestingSelector(::std::boxed::Box<NestingSelector<'tree>>),
    PseudoClassSelector(::std::boxed::Box<PseudoClassSelector<'tree>>),
    PseudoElementSelector(::std::boxed::Box<PseudoElementSelector<'tree>>),
    SiblingSelector(::std::boxed::Box<SiblingSelector<'tree>>),
    StringValue(::std::boxed::Box<StringValue<'tree>>),
    TagName(::std::boxed::Box<TagName<'tree>>),
    UniversalSelector(::std::boxed::Box<UniversalSelector<'tree>>),
}
impl<'tree> ::treesitter_types::FromNode<'tree> for ClassSelectorChildren<'tree> {
    #[allow(clippy::collapsible_else_if)]
    fn from_node(
        node: ::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        match node.kind() {
            "adjacent_sibling_selector" => {
                Ok(Self::AdjacentSiblingSelector(::std::boxed::Box::new(
                    <AdjacentSiblingSelector as ::treesitter_types::FromNode>::from_node(
                        node, src,
                    )?,
                )))
            }
            "attribute_selector" => Ok(Self::AttributeSelector(::std::boxed::Box::new(
                <AttributeSelector as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "child_selector" => Ok(Self::ChildSelector(::std::boxed::Box::new(
                <ChildSelector as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "class_name" => Ok(Self::ClassName(::std::boxed::Box::new(
                <ClassName as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "class_selector" => Ok(Self::ClassSelector(::std::boxed::Box::new(
                <ClassSelector as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "descendant_selector" => Ok(Self::DescendantSelector(::std::boxed::Box::new(
                <DescendantSelector as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "id_selector" => Ok(Self::IdSelector(::std::boxed::Box::new(
                <IdSelector as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "namespace_selector" => Ok(Self::NamespaceSelector(::std::boxed::Box::new(
                <NamespaceSelector as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "nesting_selector" => Ok(Self::NestingSelector(::std::boxed::Box::new(
                <NestingSelector as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "pseudo_class_selector" => Ok(Self::PseudoClassSelector(::std::boxed::Box::new(
                <PseudoClassSelector as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "pseudo_element_selector" => Ok(Self::PseudoElementSelector(::std::boxed::Box::new(
                <PseudoElementSelector as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "sibling_selector" => Ok(Self::SiblingSelector(::std::boxed::Box::new(
                <SiblingSelector as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "string_value" => Ok(Self::StringValue(::std::boxed::Box::new(
                <StringValue as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "tag_name" => Ok(Self::TagName(::std::boxed::Box::new(
                <TagName as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "universal_selector" => Ok(Self::UniversalSelector(::std::boxed::Box::new(
                <UniversalSelector as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            other => Err(::treesitter_types::ParseError::unexpected_kind(other, node)),
        }
    }
}
impl ::treesitter_types::Spanned for ClassSelectorChildren<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        match self {
            Self::AdjacentSiblingSelector(inner) => inner.span(),
            Self::AttributeSelector(inner) => inner.span(),
            Self::ChildSelector(inner) => inner.span(),
            Self::ClassName(inner) => inner.span(),
            Self::ClassSelector(inner) => inner.span(),
            Self::DescendantSelector(inner) => inner.span(),
            Self::IdSelector(inner) => inner.span(),
            Self::NamespaceSelector(inner) => inner.span(),
            Self::NestingSelector(inner) => inner.span(),
            Self::PseudoClassSelector(inner) => inner.span(),
            Self::PseudoElementSelector(inner) => inner.span(),
            Self::SiblingSelector(inner) => inner.span(),
            Self::StringValue(inner) => inner.span(),
            Self::TagName(inner) => inner.span(),
            Self::UniversalSelector(inner) => inner.span(),
        }
    }
}
#[derive(Debug, Clone)]
pub enum DeclarationChildren<'tree> {
    BinaryExpression(::std::boxed::Box<BinaryExpression<'tree>>),
    CallExpression(::std::boxed::Box<CallExpression<'tree>>),
    ColorValue(::std::boxed::Box<ColorValue<'tree>>),
    FloatValue(::std::boxed::Box<FloatValue<'tree>>),
    GridValue(::std::boxed::Box<GridValue<'tree>>),
    Important(::std::boxed::Box<Important<'tree>>),
    IntegerValue(::std::boxed::Box<IntegerValue<'tree>>),
    ParenthesizedValue(::std::boxed::Box<ParenthesizedValue<'tree>>),
    PlainValue(::std::boxed::Box<PlainValue<'tree>>),
    PropertyName(::std::boxed::Box<PropertyName<'tree>>),
    StringValue(::std::boxed::Box<StringValue<'tree>>),
}
impl<'tree> ::treesitter_types::FromNode<'tree> for DeclarationChildren<'tree> {
    #[allow(clippy::collapsible_else_if)]
    fn from_node(
        node: ::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        match node.kind() {
            "binary_expression" => Ok(Self::BinaryExpression(::std::boxed::Box::new(
                <BinaryExpression as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "call_expression" => Ok(Self::CallExpression(::std::boxed::Box::new(
                <CallExpression as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "color_value" => Ok(Self::ColorValue(::std::boxed::Box::new(
                <ColorValue as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "float_value" => Ok(Self::FloatValue(::std::boxed::Box::new(
                <FloatValue as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "grid_value" => Ok(Self::GridValue(::std::boxed::Box::new(
                <GridValue as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "important" => Ok(Self::Important(::std::boxed::Box::new(
                <Important as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "integer_value" => Ok(Self::IntegerValue(::std::boxed::Box::new(
                <IntegerValue as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "parenthesized_value" => Ok(Self::ParenthesizedValue(::std::boxed::Box::new(
                <ParenthesizedValue as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "plain_value" => Ok(Self::PlainValue(::std::boxed::Box::new(
                <PlainValue as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "property_name" => Ok(Self::PropertyName(::std::boxed::Box::new(
                <PropertyName as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "string_value" => Ok(Self::StringValue(::std::boxed::Box::new(
                <StringValue as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            other => Err(::treesitter_types::ParseError::unexpected_kind(other, node)),
        }
    }
}
impl ::treesitter_types::Spanned for DeclarationChildren<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        match self {
            Self::BinaryExpression(inner) => inner.span(),
            Self::CallExpression(inner) => inner.span(),
            Self::ColorValue(inner) => inner.span(),
            Self::FloatValue(inner) => inner.span(),
            Self::GridValue(inner) => inner.span(),
            Self::Important(inner) => inner.span(),
            Self::IntegerValue(inner) => inner.span(),
            Self::ParenthesizedValue(inner) => inner.span(),
            Self::PlainValue(inner) => inner.span(),
            Self::PropertyName(inner) => inner.span(),
            Self::StringValue(inner) => inner.span(),
        }
    }
}
#[derive(Debug, Clone)]
pub enum DescendantSelectorChildren<'tree> {
    AdjacentSiblingSelector(::std::boxed::Box<AdjacentSiblingSelector<'tree>>),
    AttributeSelector(::std::boxed::Box<AttributeSelector<'tree>>),
    ChildSelector(::std::boxed::Box<ChildSelector<'tree>>),
    ClassSelector(::std::boxed::Box<ClassSelector<'tree>>),
    DescendantSelector(::std::boxed::Box<DescendantSelector<'tree>>),
    IdSelector(::std::boxed::Box<IdSelector<'tree>>),
    NamespaceSelector(::std::boxed::Box<NamespaceSelector<'tree>>),
    NestingSelector(::std::boxed::Box<NestingSelector<'tree>>),
    PseudoClassSelector(::std::boxed::Box<PseudoClassSelector<'tree>>),
    PseudoElementSelector(::std::boxed::Box<PseudoElementSelector<'tree>>),
    SiblingSelector(::std::boxed::Box<SiblingSelector<'tree>>),
    StringValue(::std::boxed::Box<StringValue<'tree>>),
    TagName(::std::boxed::Box<TagName<'tree>>),
    UniversalSelector(::std::boxed::Box<UniversalSelector<'tree>>),
}
impl<'tree> ::treesitter_types::FromNode<'tree> for DescendantSelectorChildren<'tree> {
    #[allow(clippy::collapsible_else_if)]
    fn from_node(
        node: ::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        match node.kind() {
            "adjacent_sibling_selector" => {
                Ok(Self::AdjacentSiblingSelector(::std::boxed::Box::new(
                    <AdjacentSiblingSelector as ::treesitter_types::FromNode>::from_node(
                        node, src,
                    )?,
                )))
            }
            "attribute_selector" => Ok(Self::AttributeSelector(::std::boxed::Box::new(
                <AttributeSelector as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "child_selector" => Ok(Self::ChildSelector(::std::boxed::Box::new(
                <ChildSelector as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "class_selector" => Ok(Self::ClassSelector(::std::boxed::Box::new(
                <ClassSelector as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "descendant_selector" => Ok(Self::DescendantSelector(::std::boxed::Box::new(
                <DescendantSelector as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "id_selector" => Ok(Self::IdSelector(::std::boxed::Box::new(
                <IdSelector as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "namespace_selector" => Ok(Self::NamespaceSelector(::std::boxed::Box::new(
                <NamespaceSelector as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "nesting_selector" => Ok(Self::NestingSelector(::std::boxed::Box::new(
                <NestingSelector as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "pseudo_class_selector" => Ok(Self::PseudoClassSelector(::std::boxed::Box::new(
                <PseudoClassSelector as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "pseudo_element_selector" => Ok(Self::PseudoElementSelector(::std::boxed::Box::new(
                <PseudoElementSelector as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "sibling_selector" => Ok(Self::SiblingSelector(::std::boxed::Box::new(
                <SiblingSelector as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "string_value" => Ok(Self::StringValue(::std::boxed::Box::new(
                <StringValue as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "tag_name" => Ok(Self::TagName(::std::boxed::Box::new(
                <TagName as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "universal_selector" => Ok(Self::UniversalSelector(::std::boxed::Box::new(
                <UniversalSelector as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            other => Err(::treesitter_types::ParseError::unexpected_kind(other, node)),
        }
    }
}
impl ::treesitter_types::Spanned for DescendantSelectorChildren<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        match self {
            Self::AdjacentSiblingSelector(inner) => inner.span(),
            Self::AttributeSelector(inner) => inner.span(),
            Self::ChildSelector(inner) => inner.span(),
            Self::ClassSelector(inner) => inner.span(),
            Self::DescendantSelector(inner) => inner.span(),
            Self::IdSelector(inner) => inner.span(),
            Self::NamespaceSelector(inner) => inner.span(),
            Self::NestingSelector(inner) => inner.span(),
            Self::PseudoClassSelector(inner) => inner.span(),
            Self::PseudoElementSelector(inner) => inner.span(),
            Self::SiblingSelector(inner) => inner.span(),
            Self::StringValue(inner) => inner.span(),
            Self::TagName(inner) => inner.span(),
            Self::UniversalSelector(inner) => inner.span(),
        }
    }
}
#[derive(Debug, Clone)]
pub enum FeatureQueryChildren<'tree> {
    BinaryExpression(::std::boxed::Box<BinaryExpression<'tree>>),
    CallExpression(::std::boxed::Box<CallExpression<'tree>>),
    ColorValue(::std::boxed::Box<ColorValue<'tree>>),
    FeatureName(::std::boxed::Box<FeatureName<'tree>>),
    FloatValue(::std::boxed::Box<FloatValue<'tree>>),
    GridValue(::std::boxed::Box<GridValue<'tree>>),
    Important(::std::boxed::Box<Important<'tree>>),
    IntegerValue(::std::boxed::Box<IntegerValue<'tree>>),
    ParenthesizedValue(::std::boxed::Box<ParenthesizedValue<'tree>>),
    PlainValue(::std::boxed::Box<PlainValue<'tree>>),
    StringValue(::std::boxed::Box<StringValue<'tree>>),
}
impl<'tree> ::treesitter_types::FromNode<'tree> for FeatureQueryChildren<'tree> {
    #[allow(clippy::collapsible_else_if)]
    fn from_node(
        node: ::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        match node.kind() {
            "binary_expression" => Ok(Self::BinaryExpression(::std::boxed::Box::new(
                <BinaryExpression as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "call_expression" => Ok(Self::CallExpression(::std::boxed::Box::new(
                <CallExpression as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "color_value" => Ok(Self::ColorValue(::std::boxed::Box::new(
                <ColorValue as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "feature_name" => Ok(Self::FeatureName(::std::boxed::Box::new(
                <FeatureName as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "float_value" => Ok(Self::FloatValue(::std::boxed::Box::new(
                <FloatValue as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "grid_value" => Ok(Self::GridValue(::std::boxed::Box::new(
                <GridValue as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "important" => Ok(Self::Important(::std::boxed::Box::new(
                <Important as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "integer_value" => Ok(Self::IntegerValue(::std::boxed::Box::new(
                <IntegerValue as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "parenthesized_value" => Ok(Self::ParenthesizedValue(::std::boxed::Box::new(
                <ParenthesizedValue as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "plain_value" => Ok(Self::PlainValue(::std::boxed::Box::new(
                <PlainValue as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "string_value" => Ok(Self::StringValue(::std::boxed::Box::new(
                <StringValue as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            other => Err(::treesitter_types::ParseError::unexpected_kind(other, node)),
        }
    }
}
impl ::treesitter_types::Spanned for FeatureQueryChildren<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        match self {
            Self::BinaryExpression(inner) => inner.span(),
            Self::CallExpression(inner) => inner.span(),
            Self::ColorValue(inner) => inner.span(),
            Self::FeatureName(inner) => inner.span(),
            Self::FloatValue(inner) => inner.span(),
            Self::GridValue(inner) => inner.span(),
            Self::Important(inner) => inner.span(),
            Self::IntegerValue(inner) => inner.span(),
            Self::ParenthesizedValue(inner) => inner.span(),
            Self::PlainValue(inner) => inner.span(),
            Self::StringValue(inner) => inner.span(),
        }
    }
}
#[derive(Debug, Clone)]
pub enum GridValueChildren<'tree> {
    BinaryExpression(::std::boxed::Box<BinaryExpression<'tree>>),
    CallExpression(::std::boxed::Box<CallExpression<'tree>>),
    ColorValue(::std::boxed::Box<ColorValue<'tree>>),
    FloatValue(::std::boxed::Box<FloatValue<'tree>>),
    GridValue(::std::boxed::Box<GridValue<'tree>>),
    Important(::std::boxed::Box<Important<'tree>>),
    IntegerValue(::std::boxed::Box<IntegerValue<'tree>>),
    ParenthesizedValue(::std::boxed::Box<ParenthesizedValue<'tree>>),
    PlainValue(::std::boxed::Box<PlainValue<'tree>>),
    StringValue(::std::boxed::Box<StringValue<'tree>>),
}
impl<'tree> ::treesitter_types::FromNode<'tree> for GridValueChildren<'tree> {
    #[allow(clippy::collapsible_else_if)]
    fn from_node(
        node: ::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        match node.kind() {
            "binary_expression" => Ok(Self::BinaryExpression(::std::boxed::Box::new(
                <BinaryExpression as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "call_expression" => Ok(Self::CallExpression(::std::boxed::Box::new(
                <CallExpression as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "color_value" => Ok(Self::ColorValue(::std::boxed::Box::new(
                <ColorValue as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "float_value" => Ok(Self::FloatValue(::std::boxed::Box::new(
                <FloatValue as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "grid_value" => Ok(Self::GridValue(::std::boxed::Box::new(
                <GridValue as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "important" => Ok(Self::Important(::std::boxed::Box::new(
                <Important as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "integer_value" => Ok(Self::IntegerValue(::std::boxed::Box::new(
                <IntegerValue as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "parenthesized_value" => Ok(Self::ParenthesizedValue(::std::boxed::Box::new(
                <ParenthesizedValue as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "plain_value" => Ok(Self::PlainValue(::std::boxed::Box::new(
                <PlainValue as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "string_value" => Ok(Self::StringValue(::std::boxed::Box::new(
                <StringValue as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            other => Err(::treesitter_types::ParseError::unexpected_kind(other, node)),
        }
    }
}
impl ::treesitter_types::Spanned for GridValueChildren<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        match self {
            Self::BinaryExpression(inner) => inner.span(),
            Self::CallExpression(inner) => inner.span(),
            Self::ColorValue(inner) => inner.span(),
            Self::FloatValue(inner) => inner.span(),
            Self::GridValue(inner) => inner.span(),
            Self::Important(inner) => inner.span(),
            Self::IntegerValue(inner) => inner.span(),
            Self::ParenthesizedValue(inner) => inner.span(),
            Self::PlainValue(inner) => inner.span(),
            Self::StringValue(inner) => inner.span(),
        }
    }
}
#[derive(Debug, Clone)]
pub enum IdSelectorChildren<'tree> {
    AdjacentSiblingSelector(::std::boxed::Box<AdjacentSiblingSelector<'tree>>),
    AttributeSelector(::std::boxed::Box<AttributeSelector<'tree>>),
    ChildSelector(::std::boxed::Box<ChildSelector<'tree>>),
    ClassSelector(::std::boxed::Box<ClassSelector<'tree>>),
    DescendantSelector(::std::boxed::Box<DescendantSelector<'tree>>),
    IdName(::std::boxed::Box<IdName<'tree>>),
    IdSelector(::std::boxed::Box<IdSelector<'tree>>),
    NamespaceSelector(::std::boxed::Box<NamespaceSelector<'tree>>),
    NestingSelector(::std::boxed::Box<NestingSelector<'tree>>),
    PseudoClassSelector(::std::boxed::Box<PseudoClassSelector<'tree>>),
    PseudoElementSelector(::std::boxed::Box<PseudoElementSelector<'tree>>),
    SiblingSelector(::std::boxed::Box<SiblingSelector<'tree>>),
    StringValue(::std::boxed::Box<StringValue<'tree>>),
    TagName(::std::boxed::Box<TagName<'tree>>),
    UniversalSelector(::std::boxed::Box<UniversalSelector<'tree>>),
}
impl<'tree> ::treesitter_types::FromNode<'tree> for IdSelectorChildren<'tree> {
    #[allow(clippy::collapsible_else_if)]
    fn from_node(
        node: ::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        match node.kind() {
            "adjacent_sibling_selector" => {
                Ok(Self::AdjacentSiblingSelector(::std::boxed::Box::new(
                    <AdjacentSiblingSelector as ::treesitter_types::FromNode>::from_node(
                        node, src,
                    )?,
                )))
            }
            "attribute_selector" => Ok(Self::AttributeSelector(::std::boxed::Box::new(
                <AttributeSelector as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "child_selector" => Ok(Self::ChildSelector(::std::boxed::Box::new(
                <ChildSelector as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "class_selector" => Ok(Self::ClassSelector(::std::boxed::Box::new(
                <ClassSelector as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "descendant_selector" => Ok(Self::DescendantSelector(::std::boxed::Box::new(
                <DescendantSelector as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "id_name" => Ok(Self::IdName(::std::boxed::Box::new(
                <IdName as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "id_selector" => Ok(Self::IdSelector(::std::boxed::Box::new(
                <IdSelector as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "namespace_selector" => Ok(Self::NamespaceSelector(::std::boxed::Box::new(
                <NamespaceSelector as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "nesting_selector" => Ok(Self::NestingSelector(::std::boxed::Box::new(
                <NestingSelector as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "pseudo_class_selector" => Ok(Self::PseudoClassSelector(::std::boxed::Box::new(
                <PseudoClassSelector as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "pseudo_element_selector" => Ok(Self::PseudoElementSelector(::std::boxed::Box::new(
                <PseudoElementSelector as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "sibling_selector" => Ok(Self::SiblingSelector(::std::boxed::Box::new(
                <SiblingSelector as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "string_value" => Ok(Self::StringValue(::std::boxed::Box::new(
                <StringValue as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "tag_name" => Ok(Self::TagName(::std::boxed::Box::new(
                <TagName as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "universal_selector" => Ok(Self::UniversalSelector(::std::boxed::Box::new(
                <UniversalSelector as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            other => Err(::treesitter_types::ParseError::unexpected_kind(other, node)),
        }
    }
}
impl ::treesitter_types::Spanned for IdSelectorChildren<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        match self {
            Self::AdjacentSiblingSelector(inner) => inner.span(),
            Self::AttributeSelector(inner) => inner.span(),
            Self::ChildSelector(inner) => inner.span(),
            Self::ClassSelector(inner) => inner.span(),
            Self::DescendantSelector(inner) => inner.span(),
            Self::IdName(inner) => inner.span(),
            Self::IdSelector(inner) => inner.span(),
            Self::NamespaceSelector(inner) => inner.span(),
            Self::NestingSelector(inner) => inner.span(),
            Self::PseudoClassSelector(inner) => inner.span(),
            Self::PseudoElementSelector(inner) => inner.span(),
            Self::SiblingSelector(inner) => inner.span(),
            Self::StringValue(inner) => inner.span(),
            Self::TagName(inner) => inner.span(),
            Self::UniversalSelector(inner) => inner.span(),
        }
    }
}
#[derive(Debug, Clone)]
pub enum ImportStatementChildren<'tree> {
    BinaryExpression(::std::boxed::Box<BinaryExpression<'tree>>),
    BinaryQuery(::std::boxed::Box<BinaryQuery<'tree>>),
    CallExpression(::std::boxed::Box<CallExpression<'tree>>),
    ColorValue(::std::boxed::Box<ColorValue<'tree>>),
    FeatureQuery(::std::boxed::Box<FeatureQuery<'tree>>),
    FloatValue(::std::boxed::Box<FloatValue<'tree>>),
    GridValue(::std::boxed::Box<GridValue<'tree>>),
    Important(::std::boxed::Box<Important<'tree>>),
    IntegerValue(::std::boxed::Box<IntegerValue<'tree>>),
    KeywordQuery(::std::boxed::Box<KeywordQuery<'tree>>),
    ParenthesizedQuery(::std::boxed::Box<ParenthesizedQuery<'tree>>),
    ParenthesizedValue(::std::boxed::Box<ParenthesizedValue<'tree>>),
    PlainValue(::std::boxed::Box<PlainValue<'tree>>),
    SelectorQuery(::std::boxed::Box<SelectorQuery<'tree>>),
    StringValue(::std::boxed::Box<StringValue<'tree>>),
    UnaryQuery(::std::boxed::Box<UnaryQuery<'tree>>),
}
impl<'tree> ::treesitter_types::FromNode<'tree> for ImportStatementChildren<'tree> {
    #[allow(clippy::collapsible_else_if)]
    fn from_node(
        node: ::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        match node.kind() {
            "binary_expression" => Ok(Self::BinaryExpression(::std::boxed::Box::new(
                <BinaryExpression as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "binary_query" => Ok(Self::BinaryQuery(::std::boxed::Box::new(
                <BinaryQuery as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "call_expression" => Ok(Self::CallExpression(::std::boxed::Box::new(
                <CallExpression as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "color_value" => Ok(Self::ColorValue(::std::boxed::Box::new(
                <ColorValue as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "feature_query" => Ok(Self::FeatureQuery(::std::boxed::Box::new(
                <FeatureQuery as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "float_value" => Ok(Self::FloatValue(::std::boxed::Box::new(
                <FloatValue as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "grid_value" => Ok(Self::GridValue(::std::boxed::Box::new(
                <GridValue as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "important" => Ok(Self::Important(::std::boxed::Box::new(
                <Important as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "integer_value" => Ok(Self::IntegerValue(::std::boxed::Box::new(
                <IntegerValue as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "keyword_query" => Ok(Self::KeywordQuery(::std::boxed::Box::new(
                <KeywordQuery as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "parenthesized_query" => Ok(Self::ParenthesizedQuery(::std::boxed::Box::new(
                <ParenthesizedQuery as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "parenthesized_value" => Ok(Self::ParenthesizedValue(::std::boxed::Box::new(
                <ParenthesizedValue as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "plain_value" => Ok(Self::PlainValue(::std::boxed::Box::new(
                <PlainValue as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "selector_query" => Ok(Self::SelectorQuery(::std::boxed::Box::new(
                <SelectorQuery as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "string_value" => Ok(Self::StringValue(::std::boxed::Box::new(
                <StringValue as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "unary_query" => Ok(Self::UnaryQuery(::std::boxed::Box::new(
                <UnaryQuery as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            other => Err(::treesitter_types::ParseError::unexpected_kind(other, node)),
        }
    }
}
impl ::treesitter_types::Spanned for ImportStatementChildren<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        match self {
            Self::BinaryExpression(inner) => inner.span(),
            Self::BinaryQuery(inner) => inner.span(),
            Self::CallExpression(inner) => inner.span(),
            Self::ColorValue(inner) => inner.span(),
            Self::FeatureQuery(inner) => inner.span(),
            Self::FloatValue(inner) => inner.span(),
            Self::GridValue(inner) => inner.span(),
            Self::Important(inner) => inner.span(),
            Self::IntegerValue(inner) => inner.span(),
            Self::KeywordQuery(inner) => inner.span(),
            Self::ParenthesizedQuery(inner) => inner.span(),
            Self::ParenthesizedValue(inner) => inner.span(),
            Self::PlainValue(inner) => inner.span(),
            Self::SelectorQuery(inner) => inner.span(),
            Self::StringValue(inner) => inner.span(),
            Self::UnaryQuery(inner) => inner.span(),
        }
    }
}
#[derive(Debug, Clone)]
pub enum KeyframeBlockChildren<'tree> {
    Block(::std::boxed::Box<Block<'tree>>),
    From(::std::boxed::Box<From<'tree>>),
    IntegerValue(::std::boxed::Box<IntegerValue<'tree>>),
    To(::std::boxed::Box<To<'tree>>),
}
impl<'tree> ::treesitter_types::FromNode<'tree> for KeyframeBlockChildren<'tree> {
    #[allow(clippy::collapsible_else_if)]
    fn from_node(
        node: ::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        match node.kind() {
            "block" => Ok(Self::Block(::std::boxed::Box::new(
                <Block as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "from" => Ok(Self::From(::std::boxed::Box::new(
                <From as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "integer_value" => Ok(Self::IntegerValue(::std::boxed::Box::new(
                <IntegerValue as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "to" => Ok(Self::To(::std::boxed::Box::new(
                <To as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            other => Err(::treesitter_types::ParseError::unexpected_kind(other, node)),
        }
    }
}
impl ::treesitter_types::Spanned for KeyframeBlockChildren<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        match self {
            Self::Block(inner) => inner.span(),
            Self::From(inner) => inner.span(),
            Self::IntegerValue(inner) => inner.span(),
            Self::To(inner) => inner.span(),
        }
    }
}
#[derive(Debug, Clone)]
pub enum KeyframesStatementChildren<'tree> {
    AtKeyword(::std::boxed::Box<AtKeyword<'tree>>),
    KeyframeBlockList(::std::boxed::Box<KeyframeBlockList<'tree>>),
    KeyframesName(::std::boxed::Box<KeyframesName<'tree>>),
}
impl<'tree> ::treesitter_types::FromNode<'tree> for KeyframesStatementChildren<'tree> {
    #[allow(clippy::collapsible_else_if)]
    fn from_node(
        node: ::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        match node.kind() {
            "at_keyword" => Ok(Self::AtKeyword(::std::boxed::Box::new(
                <AtKeyword as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "keyframe_block_list" => Ok(Self::KeyframeBlockList(::std::boxed::Box::new(
                <KeyframeBlockList as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "keyframes_name" => Ok(Self::KeyframesName(::std::boxed::Box::new(
                <KeyframesName as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            other => Err(::treesitter_types::ParseError::unexpected_kind(other, node)),
        }
    }
}
impl ::treesitter_types::Spanned for KeyframesStatementChildren<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        match self {
            Self::AtKeyword(inner) => inner.span(),
            Self::KeyframeBlockList(inner) => inner.span(),
            Self::KeyframesName(inner) => inner.span(),
        }
    }
}
#[derive(Debug, Clone)]
pub enum MediaStatementChildren<'tree> {
    BinaryQuery(::std::boxed::Box<BinaryQuery<'tree>>),
    Block(::std::boxed::Box<Block<'tree>>),
    FeatureQuery(::std::boxed::Box<FeatureQuery<'tree>>),
    KeywordQuery(::std::boxed::Box<KeywordQuery<'tree>>),
    ParenthesizedQuery(::std::boxed::Box<ParenthesizedQuery<'tree>>),
    SelectorQuery(::std::boxed::Box<SelectorQuery<'tree>>),
    UnaryQuery(::std::boxed::Box<UnaryQuery<'tree>>),
}
impl<'tree> ::treesitter_types::FromNode<'tree> for MediaStatementChildren<'tree> {
    #[allow(clippy::collapsible_else_if)]
    fn from_node(
        node: ::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        match node.kind() {
            "binary_query" => Ok(Self::BinaryQuery(::std::boxed::Box::new(
                <BinaryQuery as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "block" => Ok(Self::Block(::std::boxed::Box::new(
                <Block as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "feature_query" => Ok(Self::FeatureQuery(::std::boxed::Box::new(
                <FeatureQuery as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "keyword_query" => Ok(Self::KeywordQuery(::std::boxed::Box::new(
                <KeywordQuery as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "parenthesized_query" => Ok(Self::ParenthesizedQuery(::std::boxed::Box::new(
                <ParenthesizedQuery as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "selector_query" => Ok(Self::SelectorQuery(::std::boxed::Box::new(
                <SelectorQuery as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "unary_query" => Ok(Self::UnaryQuery(::std::boxed::Box::new(
                <UnaryQuery as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            other => Err(::treesitter_types::ParseError::unexpected_kind(other, node)),
        }
    }
}
impl ::treesitter_types::Spanned for MediaStatementChildren<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        match self {
            Self::BinaryQuery(inner) => inner.span(),
            Self::Block(inner) => inner.span(),
            Self::FeatureQuery(inner) => inner.span(),
            Self::KeywordQuery(inner) => inner.span(),
            Self::ParenthesizedQuery(inner) => inner.span(),
            Self::SelectorQuery(inner) => inner.span(),
            Self::UnaryQuery(inner) => inner.span(),
        }
    }
}
#[derive(Debug, Clone)]
pub enum NamespaceSelectorChildren<'tree> {
    AdjacentSiblingSelector(::std::boxed::Box<AdjacentSiblingSelector<'tree>>),
    AttributeSelector(::std::boxed::Box<AttributeSelector<'tree>>),
    ChildSelector(::std::boxed::Box<ChildSelector<'tree>>),
    ClassSelector(::std::boxed::Box<ClassSelector<'tree>>),
    DescendantSelector(::std::boxed::Box<DescendantSelector<'tree>>),
    IdSelector(::std::boxed::Box<IdSelector<'tree>>),
    NamespaceSelector(::std::boxed::Box<NamespaceSelector<'tree>>),
    NestingSelector(::std::boxed::Box<NestingSelector<'tree>>),
    PseudoClassSelector(::std::boxed::Box<PseudoClassSelector<'tree>>),
    PseudoElementSelector(::std::boxed::Box<PseudoElementSelector<'tree>>),
    SiblingSelector(::std::boxed::Box<SiblingSelector<'tree>>),
    StringValue(::std::boxed::Box<StringValue<'tree>>),
    TagName(::std::boxed::Box<TagName<'tree>>),
    UniversalSelector(::std::boxed::Box<UniversalSelector<'tree>>),
}
impl<'tree> ::treesitter_types::FromNode<'tree> for NamespaceSelectorChildren<'tree> {
    #[allow(clippy::collapsible_else_if)]
    fn from_node(
        node: ::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        match node.kind() {
            "adjacent_sibling_selector" => {
                Ok(Self::AdjacentSiblingSelector(::std::boxed::Box::new(
                    <AdjacentSiblingSelector as ::treesitter_types::FromNode>::from_node(
                        node, src,
                    )?,
                )))
            }
            "attribute_selector" => Ok(Self::AttributeSelector(::std::boxed::Box::new(
                <AttributeSelector as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "child_selector" => Ok(Self::ChildSelector(::std::boxed::Box::new(
                <ChildSelector as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "class_selector" => Ok(Self::ClassSelector(::std::boxed::Box::new(
                <ClassSelector as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "descendant_selector" => Ok(Self::DescendantSelector(::std::boxed::Box::new(
                <DescendantSelector as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "id_selector" => Ok(Self::IdSelector(::std::boxed::Box::new(
                <IdSelector as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "namespace_selector" => Ok(Self::NamespaceSelector(::std::boxed::Box::new(
                <NamespaceSelector as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "nesting_selector" => Ok(Self::NestingSelector(::std::boxed::Box::new(
                <NestingSelector as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "pseudo_class_selector" => Ok(Self::PseudoClassSelector(::std::boxed::Box::new(
                <PseudoClassSelector as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "pseudo_element_selector" => Ok(Self::PseudoElementSelector(::std::boxed::Box::new(
                <PseudoElementSelector as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "sibling_selector" => Ok(Self::SiblingSelector(::std::boxed::Box::new(
                <SiblingSelector as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "string_value" => Ok(Self::StringValue(::std::boxed::Box::new(
                <StringValue as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "tag_name" => Ok(Self::TagName(::std::boxed::Box::new(
                <TagName as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "universal_selector" => Ok(Self::UniversalSelector(::std::boxed::Box::new(
                <UniversalSelector as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            other => Err(::treesitter_types::ParseError::unexpected_kind(other, node)),
        }
    }
}
impl ::treesitter_types::Spanned for NamespaceSelectorChildren<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        match self {
            Self::AdjacentSiblingSelector(inner) => inner.span(),
            Self::AttributeSelector(inner) => inner.span(),
            Self::ChildSelector(inner) => inner.span(),
            Self::ClassSelector(inner) => inner.span(),
            Self::DescendantSelector(inner) => inner.span(),
            Self::IdSelector(inner) => inner.span(),
            Self::NamespaceSelector(inner) => inner.span(),
            Self::NestingSelector(inner) => inner.span(),
            Self::PseudoClassSelector(inner) => inner.span(),
            Self::PseudoElementSelector(inner) => inner.span(),
            Self::SiblingSelector(inner) => inner.span(),
            Self::StringValue(inner) => inner.span(),
            Self::TagName(inner) => inner.span(),
            Self::UniversalSelector(inner) => inner.span(),
        }
    }
}
#[derive(Debug, Clone)]
pub enum NamespaceStatementChildren<'tree> {
    CallExpression(::std::boxed::Box<CallExpression<'tree>>),
    NamespaceName(::std::boxed::Box<NamespaceName<'tree>>),
    StringValue(::std::boxed::Box<StringValue<'tree>>),
}
impl<'tree> ::treesitter_types::FromNode<'tree> for NamespaceStatementChildren<'tree> {
    #[allow(clippy::collapsible_else_if)]
    fn from_node(
        node: ::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        match node.kind() {
            "call_expression" => Ok(Self::CallExpression(::std::boxed::Box::new(
                <CallExpression as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "namespace_name" => Ok(Self::NamespaceName(::std::boxed::Box::new(
                <NamespaceName as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "string_value" => Ok(Self::StringValue(::std::boxed::Box::new(
                <StringValue as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            other => Err(::treesitter_types::ParseError::unexpected_kind(other, node)),
        }
    }
}
impl ::treesitter_types::Spanned for NamespaceStatementChildren<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        match self {
            Self::CallExpression(inner) => inner.span(),
            Self::NamespaceName(inner) => inner.span(),
            Self::StringValue(inner) => inner.span(),
        }
    }
}
#[derive(Debug, Clone)]
pub enum ParenthesizedQueryChildren<'tree> {
    BinaryQuery(::std::boxed::Box<BinaryQuery<'tree>>),
    FeatureQuery(::std::boxed::Box<FeatureQuery<'tree>>),
    KeywordQuery(::std::boxed::Box<KeywordQuery<'tree>>),
    ParenthesizedQuery(::std::boxed::Box<ParenthesizedQuery<'tree>>),
    SelectorQuery(::std::boxed::Box<SelectorQuery<'tree>>),
    UnaryQuery(::std::boxed::Box<UnaryQuery<'tree>>),
}
impl<'tree> ::treesitter_types::FromNode<'tree> for ParenthesizedQueryChildren<'tree> {
    #[allow(clippy::collapsible_else_if)]
    fn from_node(
        node: ::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        match node.kind() {
            "binary_query" => Ok(Self::BinaryQuery(::std::boxed::Box::new(
                <BinaryQuery as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "feature_query" => Ok(Self::FeatureQuery(::std::boxed::Box::new(
                <FeatureQuery as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "keyword_query" => Ok(Self::KeywordQuery(::std::boxed::Box::new(
                <KeywordQuery as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "parenthesized_query" => Ok(Self::ParenthesizedQuery(::std::boxed::Box::new(
                <ParenthesizedQuery as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "selector_query" => Ok(Self::SelectorQuery(::std::boxed::Box::new(
                <SelectorQuery as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "unary_query" => Ok(Self::UnaryQuery(::std::boxed::Box::new(
                <UnaryQuery as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            other => Err(::treesitter_types::ParseError::unexpected_kind(other, node)),
        }
    }
}
impl ::treesitter_types::Spanned for ParenthesizedQueryChildren<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        match self {
            Self::BinaryQuery(inner) => inner.span(),
            Self::FeatureQuery(inner) => inner.span(),
            Self::KeywordQuery(inner) => inner.span(),
            Self::ParenthesizedQuery(inner) => inner.span(),
            Self::SelectorQuery(inner) => inner.span(),
            Self::UnaryQuery(inner) => inner.span(),
        }
    }
}
#[derive(Debug, Clone)]
pub enum ParenthesizedValueChildren<'tree> {
    BinaryExpression(::std::boxed::Box<BinaryExpression<'tree>>),
    CallExpression(::std::boxed::Box<CallExpression<'tree>>),
    ColorValue(::std::boxed::Box<ColorValue<'tree>>),
    FloatValue(::std::boxed::Box<FloatValue<'tree>>),
    GridValue(::std::boxed::Box<GridValue<'tree>>),
    Important(::std::boxed::Box<Important<'tree>>),
    IntegerValue(::std::boxed::Box<IntegerValue<'tree>>),
    ParenthesizedValue(::std::boxed::Box<ParenthesizedValue<'tree>>),
    PlainValue(::std::boxed::Box<PlainValue<'tree>>),
    StringValue(::std::boxed::Box<StringValue<'tree>>),
}
impl<'tree> ::treesitter_types::FromNode<'tree> for ParenthesizedValueChildren<'tree> {
    #[allow(clippy::collapsible_else_if)]
    fn from_node(
        node: ::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        match node.kind() {
            "binary_expression" => Ok(Self::BinaryExpression(::std::boxed::Box::new(
                <BinaryExpression as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "call_expression" => Ok(Self::CallExpression(::std::boxed::Box::new(
                <CallExpression as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "color_value" => Ok(Self::ColorValue(::std::boxed::Box::new(
                <ColorValue as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "float_value" => Ok(Self::FloatValue(::std::boxed::Box::new(
                <FloatValue as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "grid_value" => Ok(Self::GridValue(::std::boxed::Box::new(
                <GridValue as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "important" => Ok(Self::Important(::std::boxed::Box::new(
                <Important as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "integer_value" => Ok(Self::IntegerValue(::std::boxed::Box::new(
                <IntegerValue as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "parenthesized_value" => Ok(Self::ParenthesizedValue(::std::boxed::Box::new(
                <ParenthesizedValue as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "plain_value" => Ok(Self::PlainValue(::std::boxed::Box::new(
                <PlainValue as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "string_value" => Ok(Self::StringValue(::std::boxed::Box::new(
                <StringValue as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            other => Err(::treesitter_types::ParseError::unexpected_kind(other, node)),
        }
    }
}
impl ::treesitter_types::Spanned for ParenthesizedValueChildren<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        match self {
            Self::BinaryExpression(inner) => inner.span(),
            Self::CallExpression(inner) => inner.span(),
            Self::ColorValue(inner) => inner.span(),
            Self::FloatValue(inner) => inner.span(),
            Self::GridValue(inner) => inner.span(),
            Self::Important(inner) => inner.span(),
            Self::IntegerValue(inner) => inner.span(),
            Self::ParenthesizedValue(inner) => inner.span(),
            Self::PlainValue(inner) => inner.span(),
            Self::StringValue(inner) => inner.span(),
        }
    }
}
#[derive(Debug, Clone)]
pub enum PostcssStatementChildren<'tree> {
    AtKeyword(::std::boxed::Box<AtKeyword<'tree>>),
    BinaryExpression(::std::boxed::Box<BinaryExpression<'tree>>),
    CallExpression(::std::boxed::Box<CallExpression<'tree>>),
    ColorValue(::std::boxed::Box<ColorValue<'tree>>),
    FloatValue(::std::boxed::Box<FloatValue<'tree>>),
    GridValue(::std::boxed::Box<GridValue<'tree>>),
    Important(::std::boxed::Box<Important<'tree>>),
    ImportantValue(::std::boxed::Box<ImportantValue<'tree>>),
    IntegerValue(::std::boxed::Box<IntegerValue<'tree>>),
    ParenthesizedValue(::std::boxed::Box<ParenthesizedValue<'tree>>),
    PlainValue(::std::boxed::Box<PlainValue<'tree>>),
    StringValue(::std::boxed::Box<StringValue<'tree>>),
}
impl<'tree> ::treesitter_types::FromNode<'tree> for PostcssStatementChildren<'tree> {
    #[allow(clippy::collapsible_else_if)]
    fn from_node(
        node: ::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        match node.kind() {
            "at_keyword" => Ok(Self::AtKeyword(::std::boxed::Box::new(
                <AtKeyword as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "binary_expression" => Ok(Self::BinaryExpression(::std::boxed::Box::new(
                <BinaryExpression as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "call_expression" => Ok(Self::CallExpression(::std::boxed::Box::new(
                <CallExpression as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "color_value" => Ok(Self::ColorValue(::std::boxed::Box::new(
                <ColorValue as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "float_value" => Ok(Self::FloatValue(::std::boxed::Box::new(
                <FloatValue as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "grid_value" => Ok(Self::GridValue(::std::boxed::Box::new(
                <GridValue as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "important" => Ok(Self::Important(::std::boxed::Box::new(
                <Important as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "important_value" => Ok(Self::ImportantValue(::std::boxed::Box::new(
                <ImportantValue as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "integer_value" => Ok(Self::IntegerValue(::std::boxed::Box::new(
                <IntegerValue as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "parenthesized_value" => Ok(Self::ParenthesizedValue(::std::boxed::Box::new(
                <ParenthesizedValue as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "plain_value" => Ok(Self::PlainValue(::std::boxed::Box::new(
                <PlainValue as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "string_value" => Ok(Self::StringValue(::std::boxed::Box::new(
                <StringValue as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            other => Err(::treesitter_types::ParseError::unexpected_kind(other, node)),
        }
    }
}
impl ::treesitter_types::Spanned for PostcssStatementChildren<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        match self {
            Self::AtKeyword(inner) => inner.span(),
            Self::BinaryExpression(inner) => inner.span(),
            Self::CallExpression(inner) => inner.span(),
            Self::ColorValue(inner) => inner.span(),
            Self::FloatValue(inner) => inner.span(),
            Self::GridValue(inner) => inner.span(),
            Self::Important(inner) => inner.span(),
            Self::ImportantValue(inner) => inner.span(),
            Self::IntegerValue(inner) => inner.span(),
            Self::ParenthesizedValue(inner) => inner.span(),
            Self::PlainValue(inner) => inner.span(),
            Self::StringValue(inner) => inner.span(),
        }
    }
}
#[derive(Debug, Clone)]
pub enum PseudoClassSelectorChildren<'tree> {
    AdjacentSiblingSelector(::std::boxed::Box<AdjacentSiblingSelector<'tree>>),
    Arguments(::std::boxed::Box<Arguments<'tree>>),
    AttributeSelector(::std::boxed::Box<AttributeSelector<'tree>>),
    ChildSelector(::std::boxed::Box<ChildSelector<'tree>>),
    ClassName(::std::boxed::Box<ClassName<'tree>>),
    ClassSelector(::std::boxed::Box<ClassSelector<'tree>>),
    DescendantSelector(::std::boxed::Box<DescendantSelector<'tree>>),
    IdSelector(::std::boxed::Box<IdSelector<'tree>>),
    NamespaceSelector(::std::boxed::Box<NamespaceSelector<'tree>>),
    NestingSelector(::std::boxed::Box<NestingSelector<'tree>>),
    PseudoClassSelector(::std::boxed::Box<PseudoClassSelector<'tree>>),
    PseudoElementSelector(::std::boxed::Box<PseudoElementSelector<'tree>>),
    SiblingSelector(::std::boxed::Box<SiblingSelector<'tree>>),
    StringValue(::std::boxed::Box<StringValue<'tree>>),
    TagName(::std::boxed::Box<TagName<'tree>>),
    UniversalSelector(::std::boxed::Box<UniversalSelector<'tree>>),
}
impl<'tree> ::treesitter_types::FromNode<'tree> for PseudoClassSelectorChildren<'tree> {
    #[allow(clippy::collapsible_else_if)]
    fn from_node(
        node: ::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        match node.kind() {
            "adjacent_sibling_selector" => {
                Ok(Self::AdjacentSiblingSelector(::std::boxed::Box::new(
                    <AdjacentSiblingSelector as ::treesitter_types::FromNode>::from_node(
                        node, src,
                    )?,
                )))
            }
            "arguments" => Ok(Self::Arguments(::std::boxed::Box::new(
                <Arguments as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "attribute_selector" => Ok(Self::AttributeSelector(::std::boxed::Box::new(
                <AttributeSelector as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "child_selector" => Ok(Self::ChildSelector(::std::boxed::Box::new(
                <ChildSelector as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "class_name" => Ok(Self::ClassName(::std::boxed::Box::new(
                <ClassName as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "class_selector" => Ok(Self::ClassSelector(::std::boxed::Box::new(
                <ClassSelector as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "descendant_selector" => Ok(Self::DescendantSelector(::std::boxed::Box::new(
                <DescendantSelector as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "id_selector" => Ok(Self::IdSelector(::std::boxed::Box::new(
                <IdSelector as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "namespace_selector" => Ok(Self::NamespaceSelector(::std::boxed::Box::new(
                <NamespaceSelector as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "nesting_selector" => Ok(Self::NestingSelector(::std::boxed::Box::new(
                <NestingSelector as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "pseudo_class_selector" => Ok(Self::PseudoClassSelector(::std::boxed::Box::new(
                <PseudoClassSelector as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "pseudo_element_selector" => Ok(Self::PseudoElementSelector(::std::boxed::Box::new(
                <PseudoElementSelector as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "sibling_selector" => Ok(Self::SiblingSelector(::std::boxed::Box::new(
                <SiblingSelector as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "string_value" => Ok(Self::StringValue(::std::boxed::Box::new(
                <StringValue as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "tag_name" => Ok(Self::TagName(::std::boxed::Box::new(
                <TagName as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "universal_selector" => Ok(Self::UniversalSelector(::std::boxed::Box::new(
                <UniversalSelector as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            other => Err(::treesitter_types::ParseError::unexpected_kind(other, node)),
        }
    }
}
impl ::treesitter_types::Spanned for PseudoClassSelectorChildren<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        match self {
            Self::AdjacentSiblingSelector(inner) => inner.span(),
            Self::Arguments(inner) => inner.span(),
            Self::AttributeSelector(inner) => inner.span(),
            Self::ChildSelector(inner) => inner.span(),
            Self::ClassName(inner) => inner.span(),
            Self::ClassSelector(inner) => inner.span(),
            Self::DescendantSelector(inner) => inner.span(),
            Self::IdSelector(inner) => inner.span(),
            Self::NamespaceSelector(inner) => inner.span(),
            Self::NestingSelector(inner) => inner.span(),
            Self::PseudoClassSelector(inner) => inner.span(),
            Self::PseudoElementSelector(inner) => inner.span(),
            Self::SiblingSelector(inner) => inner.span(),
            Self::StringValue(inner) => inner.span(),
            Self::TagName(inner) => inner.span(),
            Self::UniversalSelector(inner) => inner.span(),
        }
    }
}
#[derive(Debug, Clone)]
pub enum PseudoElementSelectorChildren<'tree> {
    AdjacentSiblingSelector(::std::boxed::Box<AdjacentSiblingSelector<'tree>>),
    Arguments(::std::boxed::Box<Arguments<'tree>>),
    AttributeSelector(::std::boxed::Box<AttributeSelector<'tree>>),
    ChildSelector(::std::boxed::Box<ChildSelector<'tree>>),
    ClassSelector(::std::boxed::Box<ClassSelector<'tree>>),
    DescendantSelector(::std::boxed::Box<DescendantSelector<'tree>>),
    IdSelector(::std::boxed::Box<IdSelector<'tree>>),
    NamespaceSelector(::std::boxed::Box<NamespaceSelector<'tree>>),
    NestingSelector(::std::boxed::Box<NestingSelector<'tree>>),
    PseudoClassSelector(::std::boxed::Box<PseudoClassSelector<'tree>>),
    PseudoElementSelector(::std::boxed::Box<PseudoElementSelector<'tree>>),
    SiblingSelector(::std::boxed::Box<SiblingSelector<'tree>>),
    StringValue(::std::boxed::Box<StringValue<'tree>>),
    TagName(::std::boxed::Box<TagName<'tree>>),
    UniversalSelector(::std::boxed::Box<UniversalSelector<'tree>>),
}
impl<'tree> ::treesitter_types::FromNode<'tree> for PseudoElementSelectorChildren<'tree> {
    #[allow(clippy::collapsible_else_if)]
    fn from_node(
        node: ::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        match node.kind() {
            "adjacent_sibling_selector" => {
                Ok(Self::AdjacentSiblingSelector(::std::boxed::Box::new(
                    <AdjacentSiblingSelector as ::treesitter_types::FromNode>::from_node(
                        node, src,
                    )?,
                )))
            }
            "arguments" => Ok(Self::Arguments(::std::boxed::Box::new(
                <Arguments as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "attribute_selector" => Ok(Self::AttributeSelector(::std::boxed::Box::new(
                <AttributeSelector as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "child_selector" => Ok(Self::ChildSelector(::std::boxed::Box::new(
                <ChildSelector as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "class_selector" => Ok(Self::ClassSelector(::std::boxed::Box::new(
                <ClassSelector as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "descendant_selector" => Ok(Self::DescendantSelector(::std::boxed::Box::new(
                <DescendantSelector as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "id_selector" => Ok(Self::IdSelector(::std::boxed::Box::new(
                <IdSelector as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "namespace_selector" => Ok(Self::NamespaceSelector(::std::boxed::Box::new(
                <NamespaceSelector as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "nesting_selector" => Ok(Self::NestingSelector(::std::boxed::Box::new(
                <NestingSelector as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "pseudo_class_selector" => Ok(Self::PseudoClassSelector(::std::boxed::Box::new(
                <PseudoClassSelector as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "pseudo_element_selector" => Ok(Self::PseudoElementSelector(::std::boxed::Box::new(
                <PseudoElementSelector as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "sibling_selector" => Ok(Self::SiblingSelector(::std::boxed::Box::new(
                <SiblingSelector as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "string_value" => Ok(Self::StringValue(::std::boxed::Box::new(
                <StringValue as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "tag_name" => Ok(Self::TagName(::std::boxed::Box::new(
                <TagName as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "universal_selector" => Ok(Self::UniversalSelector(::std::boxed::Box::new(
                <UniversalSelector as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            other => Err(::treesitter_types::ParseError::unexpected_kind(other, node)),
        }
    }
}
impl ::treesitter_types::Spanned for PseudoElementSelectorChildren<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        match self {
            Self::AdjacentSiblingSelector(inner) => inner.span(),
            Self::Arguments(inner) => inner.span(),
            Self::AttributeSelector(inner) => inner.span(),
            Self::ChildSelector(inner) => inner.span(),
            Self::ClassSelector(inner) => inner.span(),
            Self::DescendantSelector(inner) => inner.span(),
            Self::IdSelector(inner) => inner.span(),
            Self::NamespaceSelector(inner) => inner.span(),
            Self::NestingSelector(inner) => inner.span(),
            Self::PseudoClassSelector(inner) => inner.span(),
            Self::PseudoElementSelector(inner) => inner.span(),
            Self::SiblingSelector(inner) => inner.span(),
            Self::StringValue(inner) => inner.span(),
            Self::TagName(inner) => inner.span(),
            Self::UniversalSelector(inner) => inner.span(),
        }
    }
}
#[derive(Debug, Clone)]
pub enum RuleSetChildren<'tree> {
    Block(::std::boxed::Box<Block<'tree>>),
    Selectors(::std::boxed::Box<Selectors<'tree>>),
}
impl<'tree> ::treesitter_types::FromNode<'tree> for RuleSetChildren<'tree> {
    #[allow(clippy::collapsible_else_if)]
    fn from_node(
        node: ::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        match node.kind() {
            "block" => Ok(Self::Block(::std::boxed::Box::new(
                <Block as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "selectors" => Ok(Self::Selectors(::std::boxed::Box::new(
                <Selectors as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            other => Err(::treesitter_types::ParseError::unexpected_kind(other, node)),
        }
    }
}
impl ::treesitter_types::Spanned for RuleSetChildren<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        match self {
            Self::Block(inner) => inner.span(),
            Self::Selectors(inner) => inner.span(),
        }
    }
}
#[derive(Debug, Clone)]
pub enum ScopeStatementChildren<'tree> {
    AdjacentSiblingSelector(::std::boxed::Box<AdjacentSiblingSelector<'tree>>),
    AttributeSelector(::std::boxed::Box<AttributeSelector<'tree>>),
    Block(::std::boxed::Box<Block<'tree>>),
    ChildSelector(::std::boxed::Box<ChildSelector<'tree>>),
    ClassSelector(::std::boxed::Box<ClassSelector<'tree>>),
    DescendantSelector(::std::boxed::Box<DescendantSelector<'tree>>),
    IdSelector(::std::boxed::Box<IdSelector<'tree>>),
    NamespaceSelector(::std::boxed::Box<NamespaceSelector<'tree>>),
    NestingSelector(::std::boxed::Box<NestingSelector<'tree>>),
    PseudoClassSelector(::std::boxed::Box<PseudoClassSelector<'tree>>),
    PseudoElementSelector(::std::boxed::Box<PseudoElementSelector<'tree>>),
    SiblingSelector(::std::boxed::Box<SiblingSelector<'tree>>),
    StringValue(::std::boxed::Box<StringValue<'tree>>),
    TagName(::std::boxed::Box<TagName<'tree>>),
    UniversalSelector(::std::boxed::Box<UniversalSelector<'tree>>),
}
impl<'tree> ::treesitter_types::FromNode<'tree> for ScopeStatementChildren<'tree> {
    #[allow(clippy::collapsible_else_if)]
    fn from_node(
        node: ::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        match node.kind() {
            "adjacent_sibling_selector" => {
                Ok(Self::AdjacentSiblingSelector(::std::boxed::Box::new(
                    <AdjacentSiblingSelector as ::treesitter_types::FromNode>::from_node(
                        node, src,
                    )?,
                )))
            }
            "attribute_selector" => Ok(Self::AttributeSelector(::std::boxed::Box::new(
                <AttributeSelector as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "block" => Ok(Self::Block(::std::boxed::Box::new(
                <Block as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "child_selector" => Ok(Self::ChildSelector(::std::boxed::Box::new(
                <ChildSelector as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "class_selector" => Ok(Self::ClassSelector(::std::boxed::Box::new(
                <ClassSelector as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "descendant_selector" => Ok(Self::DescendantSelector(::std::boxed::Box::new(
                <DescendantSelector as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "id_selector" => Ok(Self::IdSelector(::std::boxed::Box::new(
                <IdSelector as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "namespace_selector" => Ok(Self::NamespaceSelector(::std::boxed::Box::new(
                <NamespaceSelector as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "nesting_selector" => Ok(Self::NestingSelector(::std::boxed::Box::new(
                <NestingSelector as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "pseudo_class_selector" => Ok(Self::PseudoClassSelector(::std::boxed::Box::new(
                <PseudoClassSelector as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "pseudo_element_selector" => Ok(Self::PseudoElementSelector(::std::boxed::Box::new(
                <PseudoElementSelector as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "sibling_selector" => Ok(Self::SiblingSelector(::std::boxed::Box::new(
                <SiblingSelector as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "string_value" => Ok(Self::StringValue(::std::boxed::Box::new(
                <StringValue as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "tag_name" => Ok(Self::TagName(::std::boxed::Box::new(
                <TagName as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "universal_selector" => Ok(Self::UniversalSelector(::std::boxed::Box::new(
                <UniversalSelector as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            other => Err(::treesitter_types::ParseError::unexpected_kind(other, node)),
        }
    }
}
impl ::treesitter_types::Spanned for ScopeStatementChildren<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        match self {
            Self::AdjacentSiblingSelector(inner) => inner.span(),
            Self::AttributeSelector(inner) => inner.span(),
            Self::Block(inner) => inner.span(),
            Self::ChildSelector(inner) => inner.span(),
            Self::ClassSelector(inner) => inner.span(),
            Self::DescendantSelector(inner) => inner.span(),
            Self::IdSelector(inner) => inner.span(),
            Self::NamespaceSelector(inner) => inner.span(),
            Self::NestingSelector(inner) => inner.span(),
            Self::PseudoClassSelector(inner) => inner.span(),
            Self::PseudoElementSelector(inner) => inner.span(),
            Self::SiblingSelector(inner) => inner.span(),
            Self::StringValue(inner) => inner.span(),
            Self::TagName(inner) => inner.span(),
            Self::UniversalSelector(inner) => inner.span(),
        }
    }
}
#[derive(Debug, Clone)]
pub enum SelectorQueryChildren<'tree> {
    AdjacentSiblingSelector(::std::boxed::Box<AdjacentSiblingSelector<'tree>>),
    AttributeSelector(::std::boxed::Box<AttributeSelector<'tree>>),
    ChildSelector(::std::boxed::Box<ChildSelector<'tree>>),
    ClassSelector(::std::boxed::Box<ClassSelector<'tree>>),
    DescendantSelector(::std::boxed::Box<DescendantSelector<'tree>>),
    IdSelector(::std::boxed::Box<IdSelector<'tree>>),
    NamespaceSelector(::std::boxed::Box<NamespaceSelector<'tree>>),
    NestingSelector(::std::boxed::Box<NestingSelector<'tree>>),
    PseudoClassSelector(::std::boxed::Box<PseudoClassSelector<'tree>>),
    PseudoElementSelector(::std::boxed::Box<PseudoElementSelector<'tree>>),
    SiblingSelector(::std::boxed::Box<SiblingSelector<'tree>>),
    StringValue(::std::boxed::Box<StringValue<'tree>>),
    TagName(::std::boxed::Box<TagName<'tree>>),
    UniversalSelector(::std::boxed::Box<UniversalSelector<'tree>>),
}
impl<'tree> ::treesitter_types::FromNode<'tree> for SelectorQueryChildren<'tree> {
    #[allow(clippy::collapsible_else_if)]
    fn from_node(
        node: ::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        match node.kind() {
            "adjacent_sibling_selector" => {
                Ok(Self::AdjacentSiblingSelector(::std::boxed::Box::new(
                    <AdjacentSiblingSelector as ::treesitter_types::FromNode>::from_node(
                        node, src,
                    )?,
                )))
            }
            "attribute_selector" => Ok(Self::AttributeSelector(::std::boxed::Box::new(
                <AttributeSelector as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "child_selector" => Ok(Self::ChildSelector(::std::boxed::Box::new(
                <ChildSelector as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "class_selector" => Ok(Self::ClassSelector(::std::boxed::Box::new(
                <ClassSelector as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "descendant_selector" => Ok(Self::DescendantSelector(::std::boxed::Box::new(
                <DescendantSelector as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "id_selector" => Ok(Self::IdSelector(::std::boxed::Box::new(
                <IdSelector as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "namespace_selector" => Ok(Self::NamespaceSelector(::std::boxed::Box::new(
                <NamespaceSelector as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "nesting_selector" => Ok(Self::NestingSelector(::std::boxed::Box::new(
                <NestingSelector as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "pseudo_class_selector" => Ok(Self::PseudoClassSelector(::std::boxed::Box::new(
                <PseudoClassSelector as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "pseudo_element_selector" => Ok(Self::PseudoElementSelector(::std::boxed::Box::new(
                <PseudoElementSelector as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "sibling_selector" => Ok(Self::SiblingSelector(::std::boxed::Box::new(
                <SiblingSelector as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "string_value" => Ok(Self::StringValue(::std::boxed::Box::new(
                <StringValue as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "tag_name" => Ok(Self::TagName(::std::boxed::Box::new(
                <TagName as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "universal_selector" => Ok(Self::UniversalSelector(::std::boxed::Box::new(
                <UniversalSelector as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            other => Err(::treesitter_types::ParseError::unexpected_kind(other, node)),
        }
    }
}
impl ::treesitter_types::Spanned for SelectorQueryChildren<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        match self {
            Self::AdjacentSiblingSelector(inner) => inner.span(),
            Self::AttributeSelector(inner) => inner.span(),
            Self::ChildSelector(inner) => inner.span(),
            Self::ClassSelector(inner) => inner.span(),
            Self::DescendantSelector(inner) => inner.span(),
            Self::IdSelector(inner) => inner.span(),
            Self::NamespaceSelector(inner) => inner.span(),
            Self::NestingSelector(inner) => inner.span(),
            Self::PseudoClassSelector(inner) => inner.span(),
            Self::PseudoElementSelector(inner) => inner.span(),
            Self::SiblingSelector(inner) => inner.span(),
            Self::StringValue(inner) => inner.span(),
            Self::TagName(inner) => inner.span(),
            Self::UniversalSelector(inner) => inner.span(),
        }
    }
}
#[derive(Debug, Clone)]
pub enum SelectorsChildren<'tree> {
    AdjacentSiblingSelector(::std::boxed::Box<AdjacentSiblingSelector<'tree>>),
    AttributeSelector(::std::boxed::Box<AttributeSelector<'tree>>),
    ChildSelector(::std::boxed::Box<ChildSelector<'tree>>),
    ClassSelector(::std::boxed::Box<ClassSelector<'tree>>),
    DescendantSelector(::std::boxed::Box<DescendantSelector<'tree>>),
    IdSelector(::std::boxed::Box<IdSelector<'tree>>),
    NamespaceSelector(::std::boxed::Box<NamespaceSelector<'tree>>),
    NestingSelector(::std::boxed::Box<NestingSelector<'tree>>),
    PseudoClassSelector(::std::boxed::Box<PseudoClassSelector<'tree>>),
    PseudoElementSelector(::std::boxed::Box<PseudoElementSelector<'tree>>),
    SiblingSelector(::std::boxed::Box<SiblingSelector<'tree>>),
    StringValue(::std::boxed::Box<StringValue<'tree>>),
    TagName(::std::boxed::Box<TagName<'tree>>),
    UniversalSelector(::std::boxed::Box<UniversalSelector<'tree>>),
}
impl<'tree> ::treesitter_types::FromNode<'tree> for SelectorsChildren<'tree> {
    #[allow(clippy::collapsible_else_if)]
    fn from_node(
        node: ::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        match node.kind() {
            "adjacent_sibling_selector" => {
                Ok(Self::AdjacentSiblingSelector(::std::boxed::Box::new(
                    <AdjacentSiblingSelector as ::treesitter_types::FromNode>::from_node(
                        node, src,
                    )?,
                )))
            }
            "attribute_selector" => Ok(Self::AttributeSelector(::std::boxed::Box::new(
                <AttributeSelector as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "child_selector" => Ok(Self::ChildSelector(::std::boxed::Box::new(
                <ChildSelector as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "class_selector" => Ok(Self::ClassSelector(::std::boxed::Box::new(
                <ClassSelector as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "descendant_selector" => Ok(Self::DescendantSelector(::std::boxed::Box::new(
                <DescendantSelector as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "id_selector" => Ok(Self::IdSelector(::std::boxed::Box::new(
                <IdSelector as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "namespace_selector" => Ok(Self::NamespaceSelector(::std::boxed::Box::new(
                <NamespaceSelector as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "nesting_selector" => Ok(Self::NestingSelector(::std::boxed::Box::new(
                <NestingSelector as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "pseudo_class_selector" => Ok(Self::PseudoClassSelector(::std::boxed::Box::new(
                <PseudoClassSelector as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "pseudo_element_selector" => Ok(Self::PseudoElementSelector(::std::boxed::Box::new(
                <PseudoElementSelector as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "sibling_selector" => Ok(Self::SiblingSelector(::std::boxed::Box::new(
                <SiblingSelector as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "string_value" => Ok(Self::StringValue(::std::boxed::Box::new(
                <StringValue as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "tag_name" => Ok(Self::TagName(::std::boxed::Box::new(
                <TagName as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "universal_selector" => Ok(Self::UniversalSelector(::std::boxed::Box::new(
                <UniversalSelector as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            other => Err(::treesitter_types::ParseError::unexpected_kind(other, node)),
        }
    }
}
impl ::treesitter_types::Spanned for SelectorsChildren<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        match self {
            Self::AdjacentSiblingSelector(inner) => inner.span(),
            Self::AttributeSelector(inner) => inner.span(),
            Self::ChildSelector(inner) => inner.span(),
            Self::ClassSelector(inner) => inner.span(),
            Self::DescendantSelector(inner) => inner.span(),
            Self::IdSelector(inner) => inner.span(),
            Self::NamespaceSelector(inner) => inner.span(),
            Self::NestingSelector(inner) => inner.span(),
            Self::PseudoClassSelector(inner) => inner.span(),
            Self::PseudoElementSelector(inner) => inner.span(),
            Self::SiblingSelector(inner) => inner.span(),
            Self::StringValue(inner) => inner.span(),
            Self::TagName(inner) => inner.span(),
            Self::UniversalSelector(inner) => inner.span(),
        }
    }
}
#[derive(Debug, Clone)]
pub enum SiblingSelectorChildren<'tree> {
    AdjacentSiblingSelector(::std::boxed::Box<AdjacentSiblingSelector<'tree>>),
    AttributeSelector(::std::boxed::Box<AttributeSelector<'tree>>),
    ChildSelector(::std::boxed::Box<ChildSelector<'tree>>),
    ClassSelector(::std::boxed::Box<ClassSelector<'tree>>),
    DescendantSelector(::std::boxed::Box<DescendantSelector<'tree>>),
    IdSelector(::std::boxed::Box<IdSelector<'tree>>),
    NamespaceSelector(::std::boxed::Box<NamespaceSelector<'tree>>),
    NestingSelector(::std::boxed::Box<NestingSelector<'tree>>),
    PseudoClassSelector(::std::boxed::Box<PseudoClassSelector<'tree>>),
    PseudoElementSelector(::std::boxed::Box<PseudoElementSelector<'tree>>),
    SiblingSelector(::std::boxed::Box<SiblingSelector<'tree>>),
    StringValue(::std::boxed::Box<StringValue<'tree>>),
    TagName(::std::boxed::Box<TagName<'tree>>),
    UniversalSelector(::std::boxed::Box<UniversalSelector<'tree>>),
}
impl<'tree> ::treesitter_types::FromNode<'tree> for SiblingSelectorChildren<'tree> {
    #[allow(clippy::collapsible_else_if)]
    fn from_node(
        node: ::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        match node.kind() {
            "adjacent_sibling_selector" => {
                Ok(Self::AdjacentSiblingSelector(::std::boxed::Box::new(
                    <AdjacentSiblingSelector as ::treesitter_types::FromNode>::from_node(
                        node, src,
                    )?,
                )))
            }
            "attribute_selector" => Ok(Self::AttributeSelector(::std::boxed::Box::new(
                <AttributeSelector as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "child_selector" => Ok(Self::ChildSelector(::std::boxed::Box::new(
                <ChildSelector as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "class_selector" => Ok(Self::ClassSelector(::std::boxed::Box::new(
                <ClassSelector as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "descendant_selector" => Ok(Self::DescendantSelector(::std::boxed::Box::new(
                <DescendantSelector as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "id_selector" => Ok(Self::IdSelector(::std::boxed::Box::new(
                <IdSelector as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "namespace_selector" => Ok(Self::NamespaceSelector(::std::boxed::Box::new(
                <NamespaceSelector as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "nesting_selector" => Ok(Self::NestingSelector(::std::boxed::Box::new(
                <NestingSelector as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "pseudo_class_selector" => Ok(Self::PseudoClassSelector(::std::boxed::Box::new(
                <PseudoClassSelector as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "pseudo_element_selector" => Ok(Self::PseudoElementSelector(::std::boxed::Box::new(
                <PseudoElementSelector as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "sibling_selector" => Ok(Self::SiblingSelector(::std::boxed::Box::new(
                <SiblingSelector as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "string_value" => Ok(Self::StringValue(::std::boxed::Box::new(
                <StringValue as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "tag_name" => Ok(Self::TagName(::std::boxed::Box::new(
                <TagName as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "universal_selector" => Ok(Self::UniversalSelector(::std::boxed::Box::new(
                <UniversalSelector as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            other => Err(::treesitter_types::ParseError::unexpected_kind(other, node)),
        }
    }
}
impl ::treesitter_types::Spanned for SiblingSelectorChildren<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        match self {
            Self::AdjacentSiblingSelector(inner) => inner.span(),
            Self::AttributeSelector(inner) => inner.span(),
            Self::ChildSelector(inner) => inner.span(),
            Self::ClassSelector(inner) => inner.span(),
            Self::DescendantSelector(inner) => inner.span(),
            Self::IdSelector(inner) => inner.span(),
            Self::NamespaceSelector(inner) => inner.span(),
            Self::NestingSelector(inner) => inner.span(),
            Self::PseudoClassSelector(inner) => inner.span(),
            Self::PseudoElementSelector(inner) => inner.span(),
            Self::SiblingSelector(inner) => inner.span(),
            Self::StringValue(inner) => inner.span(),
            Self::TagName(inner) => inner.span(),
            Self::UniversalSelector(inner) => inner.span(),
        }
    }
}
#[derive(Debug, Clone)]
pub enum StringValueChildren<'tree> {
    EscapeSequence(::std::boxed::Box<EscapeSequence<'tree>>),
    StringContent(::std::boxed::Box<StringContent<'tree>>),
}
impl<'tree> ::treesitter_types::FromNode<'tree> for StringValueChildren<'tree> {
    #[allow(clippy::collapsible_else_if)]
    fn from_node(
        node: ::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        match node.kind() {
            "escape_sequence" => Ok(Self::EscapeSequence(::std::boxed::Box::new(
                <EscapeSequence as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "string_content" => Ok(Self::StringContent(::std::boxed::Box::new(
                <StringContent as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            other => Err(::treesitter_types::ParseError::unexpected_kind(other, node)),
        }
    }
}
impl ::treesitter_types::Spanned for StringValueChildren<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        match self {
            Self::EscapeSequence(inner) => inner.span(),
            Self::StringContent(inner) => inner.span(),
        }
    }
}
#[derive(Debug, Clone)]
pub enum StylesheetChildren<'tree> {
    AtRule(::std::boxed::Box<AtRule<'tree>>),
    CharsetStatement(::std::boxed::Box<CharsetStatement<'tree>>),
    Declaration(::std::boxed::Box<Declaration<'tree>>),
    ImportStatement(::std::boxed::Box<ImportStatement<'tree>>),
    KeyframesStatement(::std::boxed::Box<KeyframesStatement<'tree>>),
    MediaStatement(::std::boxed::Box<MediaStatement<'tree>>),
    NamespaceStatement(::std::boxed::Box<NamespaceStatement<'tree>>),
    RuleSet(::std::boxed::Box<RuleSet<'tree>>),
    ScopeStatement(::std::boxed::Box<ScopeStatement<'tree>>),
    SupportsStatement(::std::boxed::Box<SupportsStatement<'tree>>),
}
impl<'tree> ::treesitter_types::FromNode<'tree> for StylesheetChildren<'tree> {
    #[allow(clippy::collapsible_else_if)]
    fn from_node(
        node: ::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        match node.kind() {
            "at_rule" => Ok(Self::AtRule(::std::boxed::Box::new(
                <AtRule as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "charset_statement" => Ok(Self::CharsetStatement(::std::boxed::Box::new(
                <CharsetStatement as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "declaration" => Ok(Self::Declaration(::std::boxed::Box::new(
                <Declaration as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "import_statement" => Ok(Self::ImportStatement(::std::boxed::Box::new(
                <ImportStatement as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "keyframes_statement" => Ok(Self::KeyframesStatement(::std::boxed::Box::new(
                <KeyframesStatement as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "media_statement" => Ok(Self::MediaStatement(::std::boxed::Box::new(
                <MediaStatement as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "namespace_statement" => Ok(Self::NamespaceStatement(::std::boxed::Box::new(
                <NamespaceStatement as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "rule_set" => Ok(Self::RuleSet(::std::boxed::Box::new(
                <RuleSet as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "scope_statement" => Ok(Self::ScopeStatement(::std::boxed::Box::new(
                <ScopeStatement as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "supports_statement" => Ok(Self::SupportsStatement(::std::boxed::Box::new(
                <SupportsStatement as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            other => Err(::treesitter_types::ParseError::unexpected_kind(other, node)),
        }
    }
}
impl ::treesitter_types::Spanned for StylesheetChildren<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        match self {
            Self::AtRule(inner) => inner.span(),
            Self::CharsetStatement(inner) => inner.span(),
            Self::Declaration(inner) => inner.span(),
            Self::ImportStatement(inner) => inner.span(),
            Self::KeyframesStatement(inner) => inner.span(),
            Self::MediaStatement(inner) => inner.span(),
            Self::NamespaceStatement(inner) => inner.span(),
            Self::RuleSet(inner) => inner.span(),
            Self::ScopeStatement(inner) => inner.span(),
            Self::SupportsStatement(inner) => inner.span(),
        }
    }
}
#[derive(Debug, Clone)]
pub enum SupportsStatementChildren<'tree> {
    BinaryQuery(::std::boxed::Box<BinaryQuery<'tree>>),
    Block(::std::boxed::Box<Block<'tree>>),
    FeatureQuery(::std::boxed::Box<FeatureQuery<'tree>>),
    KeywordQuery(::std::boxed::Box<KeywordQuery<'tree>>),
    ParenthesizedQuery(::std::boxed::Box<ParenthesizedQuery<'tree>>),
    SelectorQuery(::std::boxed::Box<SelectorQuery<'tree>>),
    UnaryQuery(::std::boxed::Box<UnaryQuery<'tree>>),
}
impl<'tree> ::treesitter_types::FromNode<'tree> for SupportsStatementChildren<'tree> {
    #[allow(clippy::collapsible_else_if)]
    fn from_node(
        node: ::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        match node.kind() {
            "binary_query" => Ok(Self::BinaryQuery(::std::boxed::Box::new(
                <BinaryQuery as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "block" => Ok(Self::Block(::std::boxed::Box::new(
                <Block as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "feature_query" => Ok(Self::FeatureQuery(::std::boxed::Box::new(
                <FeatureQuery as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "keyword_query" => Ok(Self::KeywordQuery(::std::boxed::Box::new(
                <KeywordQuery as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "parenthesized_query" => Ok(Self::ParenthesizedQuery(::std::boxed::Box::new(
                <ParenthesizedQuery as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "selector_query" => Ok(Self::SelectorQuery(::std::boxed::Box::new(
                <SelectorQuery as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "unary_query" => Ok(Self::UnaryQuery(::std::boxed::Box::new(
                <UnaryQuery as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            other => Err(::treesitter_types::ParseError::unexpected_kind(other, node)),
        }
    }
}
impl ::treesitter_types::Spanned for SupportsStatementChildren<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        match self {
            Self::BinaryQuery(inner) => inner.span(),
            Self::Block(inner) => inner.span(),
            Self::FeatureQuery(inner) => inner.span(),
            Self::KeywordQuery(inner) => inner.span(),
            Self::ParenthesizedQuery(inner) => inner.span(),
            Self::SelectorQuery(inner) => inner.span(),
            Self::UnaryQuery(inner) => inner.span(),
        }
    }
}
#[derive(Debug, Clone)]
pub enum UnaryQueryChildren<'tree> {
    BinaryQuery(::std::boxed::Box<BinaryQuery<'tree>>),
    FeatureQuery(::std::boxed::Box<FeatureQuery<'tree>>),
    KeywordQuery(::std::boxed::Box<KeywordQuery<'tree>>),
    ParenthesizedQuery(::std::boxed::Box<ParenthesizedQuery<'tree>>),
    SelectorQuery(::std::boxed::Box<SelectorQuery<'tree>>),
    UnaryQuery(::std::boxed::Box<UnaryQuery<'tree>>),
}
impl<'tree> ::treesitter_types::FromNode<'tree> for UnaryQueryChildren<'tree> {
    #[allow(clippy::collapsible_else_if)]
    fn from_node(
        node: ::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        match node.kind() {
            "binary_query" => Ok(Self::BinaryQuery(::std::boxed::Box::new(
                <BinaryQuery as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "feature_query" => Ok(Self::FeatureQuery(::std::boxed::Box::new(
                <FeatureQuery as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "keyword_query" => Ok(Self::KeywordQuery(::std::boxed::Box::new(
                <KeywordQuery as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "parenthesized_query" => Ok(Self::ParenthesizedQuery(::std::boxed::Box::new(
                <ParenthesizedQuery as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "selector_query" => Ok(Self::SelectorQuery(::std::boxed::Box::new(
                <SelectorQuery as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "unary_query" => Ok(Self::UnaryQuery(::std::boxed::Box::new(
                <UnaryQuery as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            other => Err(::treesitter_types::ParseError::unexpected_kind(other, node)),
        }
    }
}
impl ::treesitter_types::Spanned for UnaryQueryChildren<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        match self {
            Self::BinaryQuery(inner) => inner.span(),
            Self::FeatureQuery(inner) => inner.span(),
            Self::KeywordQuery(inner) => inner.span(),
            Self::ParenthesizedQuery(inner) => inner.span(),
            Self::SelectorQuery(inner) => inner.span(),
            Self::UnaryQuery(inner) => inner.span(),
        }
    }
}
#[derive(Debug, Clone)]
pub enum AnyNode<'tree> {
    AdjacentSiblingSelector(AdjacentSiblingSelector<'tree>),
    Arguments(Arguments<'tree>),
    AtRule(AtRule<'tree>),
    AttributeName(AttributeName<'tree>),
    AttributeSelector(AttributeSelector<'tree>),
    BinaryExpression(BinaryExpression<'tree>),
    BinaryQuery(BinaryQuery<'tree>),
    Block(Block<'tree>),
    CallExpression(CallExpression<'tree>),
    CharsetStatement(CharsetStatement<'tree>),
    ChildSelector(ChildSelector<'tree>),
    ClassName(ClassName<'tree>),
    ClassSelector(ClassSelector<'tree>),
    ColorValue(ColorValue<'tree>),
    Declaration(Declaration<'tree>),
    DescendantSelector(DescendantSelector<'tree>),
    FeatureQuery(FeatureQuery<'tree>),
    FloatValue(FloatValue<'tree>),
    GridValue(GridValue<'tree>),
    IdSelector(IdSelector<'tree>),
    ImportStatement(ImportStatement<'tree>),
    IntegerValue(IntegerValue<'tree>),
    KeyframeBlock(KeyframeBlock<'tree>),
    KeyframeBlockList(KeyframeBlockList<'tree>),
    KeyframesStatement(KeyframesStatement<'tree>),
    MediaStatement(MediaStatement<'tree>),
    NamespaceSelector(NamespaceSelector<'tree>),
    NamespaceStatement(NamespaceStatement<'tree>),
    ParenthesizedQuery(ParenthesizedQuery<'tree>),
    ParenthesizedValue(ParenthesizedValue<'tree>),
    PostcssStatement(PostcssStatement<'tree>),
    PseudoClassSelector(PseudoClassSelector<'tree>),
    PseudoElementSelector(PseudoElementSelector<'tree>),
    RuleSet(RuleSet<'tree>),
    ScopeStatement(ScopeStatement<'tree>),
    SelectorQuery(SelectorQuery<'tree>),
    Selectors(Selectors<'tree>),
    SiblingSelector(SiblingSelector<'tree>),
    StringValue(StringValue<'tree>),
    Stylesheet(Stylesheet<'tree>),
    SupportsStatement(SupportsStatement<'tree>),
    To(To<'tree>),
    UnaryQuery(UnaryQuery<'tree>),
    UniversalSelector(UniversalSelector<'tree>),
    AtKeyword(AtKeyword<'tree>),
    Comment(Comment<'tree>),
    EscapeSequence(EscapeSequence<'tree>),
    FeatureName(FeatureName<'tree>),
    From(From<'tree>),
    FunctionName(FunctionName<'tree>),
    IdName(IdName<'tree>),
    Identifier(Identifier<'tree>),
    Important(Important<'tree>),
    ImportantValue(ImportantValue<'tree>),
    JsComment(JsComment<'tree>),
    KeyframesName(KeyframesName<'tree>),
    KeywordQuery(KeywordQuery<'tree>),
    NamespaceName(NamespaceName<'tree>),
    NestingSelector(NestingSelector<'tree>),
    PlainValue(PlainValue<'tree>),
    PropertyName(PropertyName<'tree>),
    StringContent(StringContent<'tree>),
    TagName(TagName<'tree>),
    Unit(Unit<'tree>),
    Unknown(::tree_sitter::Node<'tree>),
}
impl<'tree> AnyNode<'tree> {
    pub fn from_node(node: ::tree_sitter::Node<'tree>, src: &'tree [u8]) -> Self {
        match node.kind() {
            "adjacent_sibling_selector" => {
                <AdjacentSiblingSelector as ::treesitter_types::FromNode>::from_node(node, src)
                    .map(Self::AdjacentSiblingSelector)
                    .unwrap_or(Self::Unknown(node))
            }
            "arguments" => <Arguments as ::treesitter_types::FromNode>::from_node(node, src)
                .map(Self::Arguments)
                .unwrap_or(Self::Unknown(node)),
            "at_rule" => <AtRule as ::treesitter_types::FromNode>::from_node(node, src)
                .map(Self::AtRule)
                .unwrap_or(Self::Unknown(node)),
            "attribute_name" => {
                <AttributeName as ::treesitter_types::FromNode>::from_node(node, src)
                    .map(Self::AttributeName)
                    .unwrap_or(Self::Unknown(node))
            }
            "attribute_selector" => {
                <AttributeSelector as ::treesitter_types::FromNode>::from_node(node, src)
                    .map(Self::AttributeSelector)
                    .unwrap_or(Self::Unknown(node))
            }
            "binary_expression" => {
                <BinaryExpression as ::treesitter_types::FromNode>::from_node(node, src)
                    .map(Self::BinaryExpression)
                    .unwrap_or(Self::Unknown(node))
            }
            "binary_query" => <BinaryQuery as ::treesitter_types::FromNode>::from_node(node, src)
                .map(Self::BinaryQuery)
                .unwrap_or(Self::Unknown(node)),
            "block" => <Block as ::treesitter_types::FromNode>::from_node(node, src)
                .map(Self::Block)
                .unwrap_or(Self::Unknown(node)),
            "call_expression" => {
                <CallExpression as ::treesitter_types::FromNode>::from_node(node, src)
                    .map(Self::CallExpression)
                    .unwrap_or(Self::Unknown(node))
            }
            "charset_statement" => {
                <CharsetStatement as ::treesitter_types::FromNode>::from_node(node, src)
                    .map(Self::CharsetStatement)
                    .unwrap_or(Self::Unknown(node))
            }
            "child_selector" => {
                <ChildSelector as ::treesitter_types::FromNode>::from_node(node, src)
                    .map(Self::ChildSelector)
                    .unwrap_or(Self::Unknown(node))
            }
            "class_name" => <ClassName as ::treesitter_types::FromNode>::from_node(node, src)
                .map(Self::ClassName)
                .unwrap_or(Self::Unknown(node)),
            "class_selector" => {
                <ClassSelector as ::treesitter_types::FromNode>::from_node(node, src)
                    .map(Self::ClassSelector)
                    .unwrap_or(Self::Unknown(node))
            }
            "color_value" => <ColorValue as ::treesitter_types::FromNode>::from_node(node, src)
                .map(Self::ColorValue)
                .unwrap_or(Self::Unknown(node)),
            "declaration" => <Declaration as ::treesitter_types::FromNode>::from_node(node, src)
                .map(Self::Declaration)
                .unwrap_or(Self::Unknown(node)),
            "descendant_selector" => {
                <DescendantSelector as ::treesitter_types::FromNode>::from_node(node, src)
                    .map(Self::DescendantSelector)
                    .unwrap_or(Self::Unknown(node))
            }
            "feature_query" => <FeatureQuery as ::treesitter_types::FromNode>::from_node(node, src)
                .map(Self::FeatureQuery)
                .unwrap_or(Self::Unknown(node)),
            "float_value" => <FloatValue as ::treesitter_types::FromNode>::from_node(node, src)
                .map(Self::FloatValue)
                .unwrap_or(Self::Unknown(node)),
            "grid_value" => <GridValue as ::treesitter_types::FromNode>::from_node(node, src)
                .map(Self::GridValue)
                .unwrap_or(Self::Unknown(node)),
            "id_selector" => <IdSelector as ::treesitter_types::FromNode>::from_node(node, src)
                .map(Self::IdSelector)
                .unwrap_or(Self::Unknown(node)),
            "import_statement" => {
                <ImportStatement as ::treesitter_types::FromNode>::from_node(node, src)
                    .map(Self::ImportStatement)
                    .unwrap_or(Self::Unknown(node))
            }
            "integer_value" => <IntegerValue as ::treesitter_types::FromNode>::from_node(node, src)
                .map(Self::IntegerValue)
                .unwrap_or(Self::Unknown(node)),
            "keyframe_block" => {
                <KeyframeBlock as ::treesitter_types::FromNode>::from_node(node, src)
                    .map(Self::KeyframeBlock)
                    .unwrap_or(Self::Unknown(node))
            }
            "keyframe_block_list" => {
                <KeyframeBlockList as ::treesitter_types::FromNode>::from_node(node, src)
                    .map(Self::KeyframeBlockList)
                    .unwrap_or(Self::Unknown(node))
            }
            "keyframes_statement" => {
                <KeyframesStatement as ::treesitter_types::FromNode>::from_node(node, src)
                    .map(Self::KeyframesStatement)
                    .unwrap_or(Self::Unknown(node))
            }
            "media_statement" => {
                <MediaStatement as ::treesitter_types::FromNode>::from_node(node, src)
                    .map(Self::MediaStatement)
                    .unwrap_or(Self::Unknown(node))
            }
            "namespace_selector" => {
                <NamespaceSelector as ::treesitter_types::FromNode>::from_node(node, src)
                    .map(Self::NamespaceSelector)
                    .unwrap_or(Self::Unknown(node))
            }
            "namespace_statement" => {
                <NamespaceStatement as ::treesitter_types::FromNode>::from_node(node, src)
                    .map(Self::NamespaceStatement)
                    .unwrap_or(Self::Unknown(node))
            }
            "parenthesized_query" => {
                <ParenthesizedQuery as ::treesitter_types::FromNode>::from_node(node, src)
                    .map(Self::ParenthesizedQuery)
                    .unwrap_or(Self::Unknown(node))
            }
            "parenthesized_value" => {
                <ParenthesizedValue as ::treesitter_types::FromNode>::from_node(node, src)
                    .map(Self::ParenthesizedValue)
                    .unwrap_or(Self::Unknown(node))
            }
            "postcss_statement" => {
                <PostcssStatement as ::treesitter_types::FromNode>::from_node(node, src)
                    .map(Self::PostcssStatement)
                    .unwrap_or(Self::Unknown(node))
            }
            "pseudo_class_selector" => {
                <PseudoClassSelector as ::treesitter_types::FromNode>::from_node(node, src)
                    .map(Self::PseudoClassSelector)
                    .unwrap_or(Self::Unknown(node))
            }
            "pseudo_element_selector" => {
                <PseudoElementSelector as ::treesitter_types::FromNode>::from_node(node, src)
                    .map(Self::PseudoElementSelector)
                    .unwrap_or(Self::Unknown(node))
            }
            "rule_set" => <RuleSet as ::treesitter_types::FromNode>::from_node(node, src)
                .map(Self::RuleSet)
                .unwrap_or(Self::Unknown(node)),
            "scope_statement" => {
                <ScopeStatement as ::treesitter_types::FromNode>::from_node(node, src)
                    .map(Self::ScopeStatement)
                    .unwrap_or(Self::Unknown(node))
            }
            "selector_query" => {
                <SelectorQuery as ::treesitter_types::FromNode>::from_node(node, src)
                    .map(Self::SelectorQuery)
                    .unwrap_or(Self::Unknown(node))
            }
            "selectors" => <Selectors as ::treesitter_types::FromNode>::from_node(node, src)
                .map(Self::Selectors)
                .unwrap_or(Self::Unknown(node)),
            "sibling_selector" => {
                <SiblingSelector as ::treesitter_types::FromNode>::from_node(node, src)
                    .map(Self::SiblingSelector)
                    .unwrap_or(Self::Unknown(node))
            }
            "string_value" => <StringValue as ::treesitter_types::FromNode>::from_node(node, src)
                .map(Self::StringValue)
                .unwrap_or(Self::Unknown(node)),
            "stylesheet" => <Stylesheet as ::treesitter_types::FromNode>::from_node(node, src)
                .map(Self::Stylesheet)
                .unwrap_or(Self::Unknown(node)),
            "supports_statement" => {
                <SupportsStatement as ::treesitter_types::FromNode>::from_node(node, src)
                    .map(Self::SupportsStatement)
                    .unwrap_or(Self::Unknown(node))
            }
            "to" => <To as ::treesitter_types::FromNode>::from_node(node, src)
                .map(Self::To)
                .unwrap_or(Self::Unknown(node)),
            "unary_query" => <UnaryQuery as ::treesitter_types::FromNode>::from_node(node, src)
                .map(Self::UnaryQuery)
                .unwrap_or(Self::Unknown(node)),
            "universal_selector" => {
                <UniversalSelector as ::treesitter_types::FromNode>::from_node(node, src)
                    .map(Self::UniversalSelector)
                    .unwrap_or(Self::Unknown(node))
            }
            "at_keyword" => <AtKeyword as ::treesitter_types::FromNode>::from_node(node, src)
                .map(Self::AtKeyword)
                .unwrap_or(Self::Unknown(node)),
            "comment" => <Comment as ::treesitter_types::FromNode>::from_node(node, src)
                .map(Self::Comment)
                .unwrap_or(Self::Unknown(node)),
            "escape_sequence" => {
                <EscapeSequence as ::treesitter_types::FromNode>::from_node(node, src)
                    .map(Self::EscapeSequence)
                    .unwrap_or(Self::Unknown(node))
            }
            "feature_name" => <FeatureName as ::treesitter_types::FromNode>::from_node(node, src)
                .map(Self::FeatureName)
                .unwrap_or(Self::Unknown(node)),
            "from" => <From as ::treesitter_types::FromNode>::from_node(node, src)
                .map(Self::From)
                .unwrap_or(Self::Unknown(node)),
            "function_name" => <FunctionName as ::treesitter_types::FromNode>::from_node(node, src)
                .map(Self::FunctionName)
                .unwrap_or(Self::Unknown(node)),
            "id_name" => <IdName as ::treesitter_types::FromNode>::from_node(node, src)
                .map(Self::IdName)
                .unwrap_or(Self::Unknown(node)),
            "identifier" => <Identifier as ::treesitter_types::FromNode>::from_node(node, src)
                .map(Self::Identifier)
                .unwrap_or(Self::Unknown(node)),
            "important" => <Important as ::treesitter_types::FromNode>::from_node(node, src)
                .map(Self::Important)
                .unwrap_or(Self::Unknown(node)),
            "important_value" => {
                <ImportantValue as ::treesitter_types::FromNode>::from_node(node, src)
                    .map(Self::ImportantValue)
                    .unwrap_or(Self::Unknown(node))
            }
            "js_comment" => <JsComment as ::treesitter_types::FromNode>::from_node(node, src)
                .map(Self::JsComment)
                .unwrap_or(Self::Unknown(node)),
            "keyframes_name" => {
                <KeyframesName as ::treesitter_types::FromNode>::from_node(node, src)
                    .map(Self::KeyframesName)
                    .unwrap_or(Self::Unknown(node))
            }
            "keyword_query" => <KeywordQuery as ::treesitter_types::FromNode>::from_node(node, src)
                .map(Self::KeywordQuery)
                .unwrap_or(Self::Unknown(node)),
            "namespace_name" => {
                <NamespaceName as ::treesitter_types::FromNode>::from_node(node, src)
                    .map(Self::NamespaceName)
                    .unwrap_or(Self::Unknown(node))
            }
            "nesting_selector" => {
                <NestingSelector as ::treesitter_types::FromNode>::from_node(node, src)
                    .map(Self::NestingSelector)
                    .unwrap_or(Self::Unknown(node))
            }
            "plain_value" => <PlainValue as ::treesitter_types::FromNode>::from_node(node, src)
                .map(Self::PlainValue)
                .unwrap_or(Self::Unknown(node)),
            "property_name" => <PropertyName as ::treesitter_types::FromNode>::from_node(node, src)
                .map(Self::PropertyName)
                .unwrap_or(Self::Unknown(node)),
            "string_content" => {
                <StringContent as ::treesitter_types::FromNode>::from_node(node, src)
                    .map(Self::StringContent)
                    .unwrap_or(Self::Unknown(node))
            }
            "tag_name" => <TagName as ::treesitter_types::FromNode>::from_node(node, src)
                .map(Self::TagName)
                .unwrap_or(Self::Unknown(node)),
            "unit" => <Unit as ::treesitter_types::FromNode>::from_node(node, src)
                .map(Self::Unit)
                .unwrap_or(Self::Unknown(node)),
            _ => Self::Unknown(node),
        }
    }
}
impl ::treesitter_types::Spanned for AnyNode<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        match self {
            Self::AdjacentSiblingSelector(inner) => inner.span(),
            Self::Arguments(inner) => inner.span(),
            Self::AtRule(inner) => inner.span(),
            Self::AttributeName(inner) => inner.span(),
            Self::AttributeSelector(inner) => inner.span(),
            Self::BinaryExpression(inner) => inner.span(),
            Self::BinaryQuery(inner) => inner.span(),
            Self::Block(inner) => inner.span(),
            Self::CallExpression(inner) => inner.span(),
            Self::CharsetStatement(inner) => inner.span(),
            Self::ChildSelector(inner) => inner.span(),
            Self::ClassName(inner) => inner.span(),
            Self::ClassSelector(inner) => inner.span(),
            Self::ColorValue(inner) => inner.span(),
            Self::Declaration(inner) => inner.span(),
            Self::DescendantSelector(inner) => inner.span(),
            Self::FeatureQuery(inner) => inner.span(),
            Self::FloatValue(inner) => inner.span(),
            Self::GridValue(inner) => inner.span(),
            Self::IdSelector(inner) => inner.span(),
            Self::ImportStatement(inner) => inner.span(),
            Self::IntegerValue(inner) => inner.span(),
            Self::KeyframeBlock(inner) => inner.span(),
            Self::KeyframeBlockList(inner) => inner.span(),
            Self::KeyframesStatement(inner) => inner.span(),
            Self::MediaStatement(inner) => inner.span(),
            Self::NamespaceSelector(inner) => inner.span(),
            Self::NamespaceStatement(inner) => inner.span(),
            Self::ParenthesizedQuery(inner) => inner.span(),
            Self::ParenthesizedValue(inner) => inner.span(),
            Self::PostcssStatement(inner) => inner.span(),
            Self::PseudoClassSelector(inner) => inner.span(),
            Self::PseudoElementSelector(inner) => inner.span(),
            Self::RuleSet(inner) => inner.span(),
            Self::ScopeStatement(inner) => inner.span(),
            Self::SelectorQuery(inner) => inner.span(),
            Self::Selectors(inner) => inner.span(),
            Self::SiblingSelector(inner) => inner.span(),
            Self::StringValue(inner) => inner.span(),
            Self::Stylesheet(inner) => inner.span(),
            Self::SupportsStatement(inner) => inner.span(),
            Self::To(inner) => inner.span(),
            Self::UnaryQuery(inner) => inner.span(),
            Self::UniversalSelector(inner) => inner.span(),
            Self::AtKeyword(inner) => inner.span(),
            Self::Comment(inner) => inner.span(),
            Self::EscapeSequence(inner) => inner.span(),
            Self::FeatureName(inner) => inner.span(),
            Self::From(inner) => inner.span(),
            Self::FunctionName(inner) => inner.span(),
            Self::IdName(inner) => inner.span(),
            Self::Identifier(inner) => inner.span(),
            Self::Important(inner) => inner.span(),
            Self::ImportantValue(inner) => inner.span(),
            Self::JsComment(inner) => inner.span(),
            Self::KeyframesName(inner) => inner.span(),
            Self::KeywordQuery(inner) => inner.span(),
            Self::NamespaceName(inner) => inner.span(),
            Self::NestingSelector(inner) => inner.span(),
            Self::PlainValue(inner) => inner.span(),
            Self::PropertyName(inner) => inner.span(),
            Self::StringContent(inner) => inner.span(),
            Self::TagName(inner) => inner.span(),
            Self::Unit(inner) => inner.span(),
            Self::Unknown(node) => ::treesitter_types::Span::from(*node),
        }
    }
}
