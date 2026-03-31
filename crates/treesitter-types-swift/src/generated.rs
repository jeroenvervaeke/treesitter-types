#[derive(Debug, Clone)]
pub struct ArrayType<'tree> {
    pub span: ::treesitter_types::Span,
    pub children: ArrayTypeChildren<'tree>,
}
impl<'tree> ::treesitter_types::FromNode<'tree> for ArrayType<'tree> {
    #[allow(clippy::match_single_binding, clippy::suspicious_else_formatting)]
    fn from_node(
        node: ::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        debug_assert_eq!(node.kind(), "array_type");
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
                <ArrayTypeChildren as ::treesitter_types::FromNode>::from_node(child, src)?
            },
        })
    }
}
impl ::treesitter_types::Spanned for ArrayType<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        self.span
    }
}
#[derive(Debug, Clone)]
pub struct AsPattern<'tree> {
    pub span: ::treesitter_types::Span,
    pub children: ::std::vec::Vec<AsPatternChildren<'tree>>,
}
impl<'tree> ::treesitter_types::FromNode<'tree> for AsPattern<'tree> {
    #[allow(clippy::match_single_binding, clippy::suspicious_else_formatting)]
    fn from_node(
        node: ::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        debug_assert_eq!(node.kind(), "as_pattern");
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
                        <AsPatternChildren as ::treesitter_types::FromNode>::from_node(child, src)?,
                    );
                }
                items
            },
        })
    }
}
impl ::treesitter_types::Spanned for AsPattern<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        self.span
    }
}
#[derive(Debug, Clone)]
pub struct AssociatedtypeDeclaration<'tree> {
    pub span: ::treesitter_types::Span,
    pub children: ::std::vec::Vec<AssociatedtypeDeclarationChildren<'tree>>,
}
impl<'tree> ::treesitter_types::FromNode<'tree> for AssociatedtypeDeclaration<'tree> {
    #[allow(clippy::match_single_binding, clippy::suspicious_else_formatting)]
    fn from_node(
        node: ::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        debug_assert_eq!(node.kind(), "associatedtype_declaration");
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
                            <AssociatedtypeDeclarationChildren as ::treesitter_types::FromNode>::from_node(
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
impl ::treesitter_types::Spanned for AssociatedtypeDeclaration<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        self.span
    }
}
#[derive(Debug, Clone)]
pub struct AssociativityClause<'tree> {
    pub span: ::treesitter_types::Span,
    text: &'tree str,
}
impl<'tree> ::treesitter_types::FromNode<'tree> for AssociativityClause<'tree> {
    fn from_node(
        node: ::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        debug_assert_eq!(node.kind(), "associativity_clause");
        Ok(Self {
            span: ::treesitter_types::Span::from(node),
            text: node.utf8_text(src)?,
        })
    }
}
impl<'tree> ::treesitter_types::LeafNode<'tree> for AssociativityClause<'tree> {
    fn text(&self) -> &'tree str {
        self.text
    }
}
impl ::treesitter_types::Spanned for AssociativityClause<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        self.span
    }
}
#[derive(Debug, Clone)]
pub struct AvailabilityCondition<'tree> {
    pub span: ::treesitter_types::Span,
    text: &'tree str,
}
impl<'tree> ::treesitter_types::FromNode<'tree> for AvailabilityCondition<'tree> {
    fn from_node(
        node: ::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        debug_assert_eq!(node.kind(), "availability_condition");
        Ok(Self {
            span: ::treesitter_types::Span::from(node),
            text: node.utf8_text(src)?,
        })
    }
}
impl<'tree> ::treesitter_types::LeafNode<'tree> for AvailabilityCondition<'tree> {
    fn text(&self) -> &'tree str {
        self.text
    }
}
impl ::treesitter_types::Spanned for AvailabilityCondition<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        self.span
    }
}
#[derive(Debug, Clone)]
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
#[derive(Debug, Clone)]
pub struct BooleanLiteral<'tree> {
    pub span: ::treesitter_types::Span,
    text: &'tree str,
}
impl<'tree> ::treesitter_types::FromNode<'tree> for BooleanLiteral<'tree> {
    fn from_node(
        node: ::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        debug_assert_eq!(node.kind(), "boolean_literal");
        Ok(Self {
            span: ::treesitter_types::Span::from(node),
            text: node.utf8_text(src)?,
        })
    }
}
impl<'tree> ::treesitter_types::LeafNode<'tree> for BooleanLiteral<'tree> {
    fn text(&self) -> &'tree str {
        self.text
    }
}
impl ::treesitter_types::Spanned for BooleanLiteral<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        self.span
    }
}
#[derive(Debug, Clone)]
pub struct BreakStatement<'tree> {
    pub span: ::treesitter_types::Span,
    pub children: ::core::option::Option<Identifier<'tree>>,
}
impl<'tree> ::treesitter_types::FromNode<'tree> for BreakStatement<'tree> {
    #[allow(clippy::match_single_binding, clippy::suspicious_else_formatting)]
    fn from_node(
        node: ::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        debug_assert_eq!(node.kind(), "break_statement");
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
                    Some(&child) => Some(<Identifier as ::treesitter_types::FromNode>::from_node(
                        child, src,
                    )?),
                    None => None,
                }
            },
        })
    }
}
impl ::treesitter_types::Spanned for BreakStatement<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        self.span
    }
}
#[derive(Debug, Clone)]
pub struct BuildConfigurationStatement<'tree> {
    pub span: ::treesitter_types::Span,
    pub children: ::std::vec::Vec<BuildConfigurationStatementChildren<'tree>>,
}
impl<'tree> ::treesitter_types::FromNode<'tree> for BuildConfigurationStatement<'tree> {
    #[allow(clippy::match_single_binding, clippy::suspicious_else_formatting)]
    fn from_node(
        node: ::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        debug_assert_eq!(node.kind(), "build_configuration_statement");
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
                            <BuildConfigurationStatementChildren as ::treesitter_types::FromNode>::from_node(
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
impl ::treesitter_types::Spanned for BuildConfigurationStatement<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        self.span
    }
}
#[derive(Debug, Clone)]
pub struct CaseCondition<'tree> {
    pub span: ::treesitter_types::Span,
    pub children: ::std::vec::Vec<CaseConditionChildren<'tree>>,
}
impl<'tree> ::treesitter_types::FromNode<'tree> for CaseCondition<'tree> {
    #[allow(clippy::match_single_binding, clippy::suspicious_else_formatting)]
    fn from_node(
        node: ::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        debug_assert_eq!(node.kind(), "case_condition");
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
                        <CaseConditionChildren as ::treesitter_types::FromNode>::from_node(
                            child, src,
                        )?,
                    );
                }
                items
            },
        })
    }
}
impl ::treesitter_types::Spanned for CaseCondition<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        self.span
    }
}
#[derive(Debug, Clone)]
pub struct CaseDeclaration<'tree> {
    pub span: ::treesitter_types::Span,
    pub children: ::std::vec::Vec<CaseDeclarationChildren<'tree>>,
}
impl<'tree> ::treesitter_types::FromNode<'tree> for CaseDeclaration<'tree> {
    #[allow(clippy::match_single_binding, clippy::suspicious_else_formatting)]
    fn from_node(
        node: ::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        debug_assert_eq!(node.kind(), "case_declaration");
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
                        <CaseDeclarationChildren as ::treesitter_types::FromNode>::from_node(
                            child, src,
                        )?,
                    );
                }
                items
            },
        })
    }
}
impl ::treesitter_types::Spanned for CaseDeclaration<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        self.span
    }
}
#[derive(Debug, Clone)]
pub struct CaseStatement<'tree> {
    pub span: ::treesitter_types::Span,
    pub children: ::std::vec::Vec<CaseStatementChildren<'tree>>,
}
impl<'tree> ::treesitter_types::FromNode<'tree> for CaseStatement<'tree> {
    #[allow(clippy::match_single_binding, clippy::suspicious_else_formatting)]
    fn from_node(
        node: ::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        debug_assert_eq!(node.kind(), "case_statement");
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
                        <CaseStatementChildren as ::treesitter_types::FromNode>::from_node(
                            child, src,
                        )?,
                    );
                }
                items
            },
        })
    }
}
impl ::treesitter_types::Spanned for CaseStatement<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        self.span
    }
}
#[derive(Debug, Clone)]
pub struct CatchClause<'tree> {
    pub span: ::treesitter_types::Span,
    pub children: ::std::vec::Vec<CatchClauseChildren<'tree>>,
}
impl<'tree> ::treesitter_types::FromNode<'tree> for CatchClause<'tree> {
    #[allow(clippy::match_single_binding, clippy::suspicious_else_formatting)]
    fn from_node(
        node: ::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        debug_assert_eq!(node.kind(), "catch_clause");
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
                        <CatchClauseChildren as ::treesitter_types::FromNode>::from_node(
                            child, src,
                        )?,
                    );
                }
                items
            },
        })
    }
}
impl ::treesitter_types::Spanned for CatchClause<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        self.span
    }
}
#[derive(Debug, Clone)]
pub struct ClassDeclaration<'tree> {
    pub span: ::treesitter_types::Span,
    pub children: ::std::vec::Vec<ClassDeclarationChildren<'tree>>,
}
impl<'tree> ::treesitter_types::FromNode<'tree> for ClassDeclaration<'tree> {
    #[allow(clippy::match_single_binding, clippy::suspicious_else_formatting)]
    fn from_node(
        node: ::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        debug_assert_eq!(node.kind(), "class_declaration");
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
                        <ClassDeclarationChildren as ::treesitter_types::FromNode>::from_node(
                            child, src,
                        )?,
                    );
                }
                items
            },
        })
    }
}
impl ::treesitter_types::Spanned for ClassDeclaration<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        self.span
    }
}
#[derive(Debug, Clone)]
pub struct ConstantDeclaration<'tree> {
    pub span: ::treesitter_types::Span,
    pub name: ::std::vec::Vec<ConstantDeclarationName<'tree>>,
    pub r#type: ::std::vec::Vec<ConstantDeclarationType<'tree>>,
    pub value: ::std::vec::Vec<ConstantDeclarationValue<'tree>>,
    pub children: ::std::vec::Vec<Modifier<'tree>>,
}
impl<'tree> ::treesitter_types::FromNode<'tree> for ConstantDeclaration<'tree> {
    #[allow(clippy::match_single_binding, clippy::suspicious_else_formatting)]
    fn from_node(
        node: ::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        debug_assert_eq!(node.kind(), "constant_declaration");
        Ok(Self {
            span: ::treesitter_types::Span::from(node),
            name: {
                let mut cursor = node.walk();
                let mut items = ::std::vec::Vec::new();
                for child in node.children_by_field_name("name", &mut cursor) {
                    items.push(
                        <ConstantDeclarationName as ::treesitter_types::FromNode>::from_node(
                            child, src,
                        )?,
                    );
                }
                items
            },
            r#type: {
                let mut cursor = node.walk();
                let mut items = ::std::vec::Vec::new();
                for child in node.children_by_field_name("type", &mut cursor) {
                    items.push(
                        <ConstantDeclarationType as ::treesitter_types::FromNode>::from_node(
                            child, src,
                        )?,
                    );
                }
                items
            },
            value: {
                let mut cursor = node.walk();
                let mut items = ::std::vec::Vec::new();
                for child in node.children_by_field_name("value", &mut cursor) {
                    items.push(
                        <ConstantDeclarationValue as ::treesitter_types::FromNode>::from_node(
                            child, src,
                        )?,
                    );
                }
                items
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
                    items.push(<Modifier as ::treesitter_types::FromNode>::from_node(
                        child, src,
                    )?);
                }
                items
            },
        })
    }
}
impl ::treesitter_types::Spanned for ConstantDeclaration<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        self.span
    }
}
#[derive(Debug, Clone)]
pub struct ContinueStatement<'tree> {
    pub span: ::treesitter_types::Span,
    pub children: ::core::option::Option<Identifier<'tree>>,
}
impl<'tree> ::treesitter_types::FromNode<'tree> for ContinueStatement<'tree> {
    #[allow(clippy::match_single_binding, clippy::suspicious_else_formatting)]
    fn from_node(
        node: ::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        debug_assert_eq!(node.kind(), "continue_statement");
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
                    Some(&child) => Some(<Identifier as ::treesitter_types::FromNode>::from_node(
                        child, src,
                    )?),
                    None => None,
                }
            },
        })
    }
}
impl ::treesitter_types::Spanned for ContinueStatement<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        self.span
    }
}
#[derive(Debug, Clone)]
pub struct DeferStatement<'tree> {
    pub span: ::treesitter_types::Span,
    pub children: ::std::vec::Vec<DeferStatementChildren<'tree>>,
}
impl<'tree> ::treesitter_types::FromNode<'tree> for DeferStatement<'tree> {
    #[allow(clippy::match_single_binding, clippy::suspicious_else_formatting)]
    fn from_node(
        node: ::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        debug_assert_eq!(node.kind(), "defer_statement");
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
                        <DeferStatementChildren as ::treesitter_types::FromNode>::from_node(
                            child, src,
                        )?,
                    );
                }
                items
            },
        })
    }
}
impl ::treesitter_types::Spanned for DeferStatement<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        self.span
    }
}
#[derive(Debug, Clone)]
pub struct DeinitializerDeclaration<'tree> {
    pub span: ::treesitter_types::Span,
    pub children: ::std::vec::Vec<DeinitializerDeclarationChildren<'tree>>,
}
impl<'tree> ::treesitter_types::FromNode<'tree> for DeinitializerDeclaration<'tree> {
    #[allow(clippy::match_single_binding, clippy::suspicious_else_formatting)]
    fn from_node(
        node: ::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        debug_assert_eq!(node.kind(), "deinitializer_declaration");
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
                            <DeinitializerDeclarationChildren as ::treesitter_types::FromNode>::from_node(
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
impl ::treesitter_types::Spanned for DeinitializerDeclaration<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        self.span
    }
}
#[derive(Debug, Clone)]
pub struct DiagnosticStatement<'tree> {
    pub span: ::treesitter_types::Span,
    pub children: StaticStringLiteral<'tree>,
}
impl<'tree> ::treesitter_types::FromNode<'tree> for DiagnosticStatement<'tree> {
    #[allow(clippy::match_single_binding, clippy::suspicious_else_formatting)]
    fn from_node(
        node: ::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        debug_assert_eq!(node.kind(), "diagnostic_statement");
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
                <StaticStringLiteral as ::treesitter_types::FromNode>::from_node(child, src)?
            },
        })
    }
}
impl ::treesitter_types::Spanned for DiagnosticStatement<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        self.span
    }
}
#[derive(Debug, Clone)]
pub struct DictionaryType<'tree> {
    pub span: ::treesitter_types::Span,
    pub children: ::std::vec::Vec<DictionaryTypeChildren<'tree>>,
}
impl<'tree> ::treesitter_types::FromNode<'tree> for DictionaryType<'tree> {
    #[allow(clippy::match_single_binding, clippy::suspicious_else_formatting)]
    fn from_node(
        node: ::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        debug_assert_eq!(node.kind(), "dictionary_type");
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
                        <DictionaryTypeChildren as ::treesitter_types::FromNode>::from_node(
                            child, src,
                        )?,
                    );
                }
                items
            },
        })
    }
}
impl ::treesitter_types::Spanned for DictionaryType<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        self.span
    }
}
#[derive(Debug, Clone)]
pub struct DoStatement<'tree> {
    pub span: ::treesitter_types::Span,
    pub children: ::std::vec::Vec<DoStatementChildren<'tree>>,
}
impl<'tree> ::treesitter_types::FromNode<'tree> for DoStatement<'tree> {
    #[allow(clippy::match_single_binding, clippy::suspicious_else_formatting)]
    fn from_node(
        node: ::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        debug_assert_eq!(node.kind(), "do_statement");
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
                        <DoStatementChildren as ::treesitter_types::FromNode>::from_node(
                            child, src,
                        )?,
                    );
                }
                items
            },
        })
    }
}
impl ::treesitter_types::Spanned for DoStatement<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        self.span
    }
}
#[derive(Debug, Clone)]
pub struct EnumCasePattern<'tree> {
    pub span: ::treesitter_types::Span,
    pub children: ::std::vec::Vec<EnumCasePatternChildren<'tree>>,
}
impl<'tree> ::treesitter_types::FromNode<'tree> for EnumCasePattern<'tree> {
    #[allow(clippy::match_single_binding, clippy::suspicious_else_formatting)]
    fn from_node(
        node: ::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        debug_assert_eq!(node.kind(), "enum_case_pattern");
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
                        <EnumCasePatternChildren as ::treesitter_types::FromNode>::from_node(
                            child, src,
                        )?,
                    );
                }
                items
            },
        })
    }
}
impl ::treesitter_types::Spanned for EnumCasePattern<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        self.span
    }
}
#[derive(Debug, Clone)]
pub struct EnumDeclaration<'tree> {
    pub span: ::treesitter_types::Span,
    pub children: ::std::vec::Vec<EnumDeclarationChildren<'tree>>,
}
impl<'tree> ::treesitter_types::FromNode<'tree> for EnumDeclaration<'tree> {
    #[allow(clippy::match_single_binding, clippy::suspicious_else_formatting)]
    fn from_node(
        node: ::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        debug_assert_eq!(node.kind(), "enum_declaration");
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
                        <EnumDeclarationChildren as ::treesitter_types::FromNode>::from_node(
                            child, src,
                        )?,
                    );
                }
                items
            },
        })
    }
}
impl ::treesitter_types::Spanned for EnumDeclaration<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        self.span
    }
}
#[derive(Debug, Clone)]
pub struct ExtensionDeclaration<'tree> {
    pub span: ::treesitter_types::Span,
    pub children: ::std::vec::Vec<ExtensionDeclarationChildren<'tree>>,
}
impl<'tree> ::treesitter_types::FromNode<'tree> for ExtensionDeclaration<'tree> {
    #[allow(clippy::match_single_binding, clippy::suspicious_else_formatting)]
    fn from_node(
        node: ::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        debug_assert_eq!(node.kind(), "extension_declaration");
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
                        <ExtensionDeclarationChildren as ::treesitter_types::FromNode>::from_node(
                            child, src,
                        )?,
                    );
                }
                items
            },
        })
    }
}
impl ::treesitter_types::Spanned for ExtensionDeclaration<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        self.span
    }
}
#[derive(Debug, Clone)]
pub struct ForStatement<'tree> {
    pub span: ::treesitter_types::Span,
    pub children: ::std::vec::Vec<ForStatementChildren<'tree>>,
}
impl<'tree> ::treesitter_types::FromNode<'tree> for ForStatement<'tree> {
    #[allow(clippy::match_single_binding, clippy::suspicious_else_formatting)]
    fn from_node(
        node: ::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        debug_assert_eq!(node.kind(), "for_statement");
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
                        <ForStatementChildren as ::treesitter_types::FromNode>::from_node(
                            child, src,
                        )?,
                    );
                }
                items
            },
        })
    }
}
impl ::treesitter_types::Spanned for ForStatement<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        self.span
    }
}
#[derive(Debug, Clone)]
pub struct FunctionDeclaration<'tree> {
    pub span: ::treesitter_types::Span,
    pub name: FunctionDeclarationName<'tree>,
    pub r#return: ::std::vec::Vec<FunctionDeclarationReturn<'tree>>,
    pub children: ::std::vec::Vec<FunctionDeclarationChildren<'tree>>,
}
impl<'tree> ::treesitter_types::FromNode<'tree> for FunctionDeclaration<'tree> {
    #[allow(clippy::match_single_binding, clippy::suspicious_else_formatting)]
    fn from_node(
        node: ::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        debug_assert_eq!(node.kind(), "function_declaration");
        Ok(Self {
            span: ::treesitter_types::Span::from(node),
            name: {
                let child = node
                    .child_by_field_name("name")
                    .ok_or_else(|| ::treesitter_types::ParseError::missing_field("name", node))?;
                <FunctionDeclarationName as ::treesitter_types::FromNode>::from_node(child, src)?
            },
            r#return: {
                let mut cursor = node.walk();
                let mut items = ::std::vec::Vec::new();
                for child in node.children_by_field_name("return", &mut cursor) {
                    items.push(
                        <FunctionDeclarationReturn as ::treesitter_types::FromNode>::from_node(
                            child, src,
                        )?,
                    );
                }
                items
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
                        <FunctionDeclarationChildren as ::treesitter_types::FromNode>::from_node(
                            child, src,
                        )?,
                    );
                }
                items
            },
        })
    }
}
impl ::treesitter_types::Spanned for FunctionDeclaration<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        self.span
    }
}
#[derive(Debug, Clone)]
pub struct GenericClause<'tree> {
    pub span: ::treesitter_types::Span,
    pub name: ::std::vec::Vec<Identifier<'tree>>,
    pub r#type: ::std::vec::Vec<GenericClauseType<'tree>>,
}
impl<'tree> ::treesitter_types::FromNode<'tree> for GenericClause<'tree> {
    #[allow(clippy::match_single_binding, clippy::suspicious_else_formatting)]
    fn from_node(
        node: ::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        debug_assert_eq!(node.kind(), "generic_clause");
        Ok(Self {
            span: ::treesitter_types::Span::from(node),
            name: {
                let mut cursor = node.walk();
                let mut items = ::std::vec::Vec::new();
                for child in node.children_by_field_name("name", &mut cursor) {
                    items.push(<Identifier as ::treesitter_types::FromNode>::from_node(
                        child, src,
                    )?);
                }
                items
            },
            r#type: {
                let mut cursor = node.walk();
                let mut items = ::std::vec::Vec::new();
                for child in node.children_by_field_name("type", &mut cursor) {
                    items.push(
                        <GenericClauseType as ::treesitter_types::FromNode>::from_node(child, src)?,
                    );
                }
                items
            },
        })
    }
}
impl ::treesitter_types::Spanned for GenericClause<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        self.span
    }
}
#[derive(Debug, Clone)]
pub struct GuardStatement<'tree> {
    pub span: ::treesitter_types::Span,
    pub children: ::std::vec::Vec<GuardStatementChildren<'tree>>,
}
impl<'tree> ::treesitter_types::FromNode<'tree> for GuardStatement<'tree> {
    #[allow(clippy::match_single_binding, clippy::suspicious_else_formatting)]
    fn from_node(
        node: ::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        debug_assert_eq!(node.kind(), "guard_statement");
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
                        <GuardStatementChildren as ::treesitter_types::FromNode>::from_node(
                            child, src,
                        )?,
                    );
                }
                items
            },
        })
    }
}
impl ::treesitter_types::Spanned for GuardStatement<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        self.span
    }
}
#[derive(Debug, Clone)]
pub struct IfStatement<'tree> {
    pub span: ::treesitter_types::Span,
    pub children: ::std::vec::Vec<IfStatementChildren<'tree>>,
}
impl<'tree> ::treesitter_types::FromNode<'tree> for IfStatement<'tree> {
    #[allow(clippy::match_single_binding, clippy::suspicious_else_formatting)]
    fn from_node(
        node: ::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        debug_assert_eq!(node.kind(), "if_statement");
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
                        <IfStatementChildren as ::treesitter_types::FromNode>::from_node(
                            child, src,
                        )?,
                    );
                }
                items
            },
        })
    }
}
impl ::treesitter_types::Spanned for IfStatement<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        self.span
    }
}
#[derive(Debug, Clone)]
pub struct ImportDeclaration<'tree> {
    pub span: ::treesitter_types::Span,
    pub children: ::std::vec::Vec<ImportDeclarationChildren<'tree>>,
}
impl<'tree> ::treesitter_types::FromNode<'tree> for ImportDeclaration<'tree> {
    #[allow(clippy::match_single_binding, clippy::suspicious_else_formatting)]
    fn from_node(
        node: ::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        debug_assert_eq!(node.kind(), "import_declaration");
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
                        <ImportDeclarationChildren as ::treesitter_types::FromNode>::from_node(
                            child, src,
                        )?,
                    );
                }
                items
            },
        })
    }
}
impl ::treesitter_types::Spanned for ImportDeclaration<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        self.span
    }
}
#[derive(Debug, Clone)]
pub struct InitializerDeclaration<'tree> {
    pub span: ::treesitter_types::Span,
    pub children: ::std::vec::Vec<InitializerDeclarationChildren<'tree>>,
}
impl<'tree> ::treesitter_types::FromNode<'tree> for InitializerDeclaration<'tree> {
    #[allow(clippy::match_single_binding, clippy::suspicious_else_formatting)]
    fn from_node(
        node: ::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        debug_assert_eq!(node.kind(), "initializer_declaration");
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
                            <InitializerDeclarationChildren as ::treesitter_types::FromNode>::from_node(
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
impl ::treesitter_types::Spanned for InitializerDeclaration<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        self.span
    }
}
#[derive(Debug, Clone)]
pub struct IsPattern<'tree> {
    pub span: ::treesitter_types::Span,
    pub children: Type<'tree>,
}
impl<'tree> ::treesitter_types::FromNode<'tree> for IsPattern<'tree> {
    #[allow(clippy::match_single_binding, clippy::suspicious_else_formatting)]
    fn from_node(
        node: ::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        debug_assert_eq!(node.kind(), "is_pattern");
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
                <Type as ::treesitter_types::FromNode>::from_node(child, src)?
            },
        })
    }
}
impl ::treesitter_types::Spanned for IsPattern<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        self.span
    }
}
#[derive(Debug, Clone)]
pub struct LabeledStatement<'tree> {
    pub span: ::treesitter_types::Span,
    pub children: ::std::vec::Vec<LabeledStatementChildren<'tree>>,
}
impl<'tree> ::treesitter_types::FromNode<'tree> for LabeledStatement<'tree> {
    #[allow(clippy::match_single_binding, clippy::suspicious_else_formatting)]
    fn from_node(
        node: ::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        debug_assert_eq!(node.kind(), "labeled_statement");
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
                        <LabeledStatementChildren as ::treesitter_types::FromNode>::from_node(
                            child, src,
                        )?,
                    );
                }
                items
            },
        })
    }
}
impl ::treesitter_types::Spanned for LabeledStatement<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        self.span
    }
}
#[derive(Debug, Clone)]
pub struct LineControlStatement<'tree> {
    pub span: ::treesitter_types::Span,
    pub children: ::core::option::Option<StaticStringLiteral<'tree>>,
}
impl<'tree> ::treesitter_types::FromNode<'tree> for LineControlStatement<'tree> {
    #[allow(clippy::match_single_binding, clippy::suspicious_else_formatting)]
    fn from_node(
        node: ::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        debug_assert_eq!(node.kind(), "line_control_statement");
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
                        <StaticStringLiteral as ::treesitter_types::FromNode>::from_node(
                            child, src,
                        )?,
                    ),
                    None => None,
                }
            },
        })
    }
}
impl ::treesitter_types::Spanned for LineControlStatement<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        self.span
    }
}
#[derive(Debug, Clone)]
pub struct Modifier<'tree> {
    pub span: ::treesitter_types::Span,
    text: &'tree str,
}
impl<'tree> ::treesitter_types::FromNode<'tree> for Modifier<'tree> {
    fn from_node(
        node: ::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        debug_assert_eq!(node.kind(), "modifier");
        Ok(Self {
            span: ::treesitter_types::Span::from(node),
            text: node.utf8_text(src)?,
        })
    }
}
impl<'tree> ::treesitter_types::LeafNode<'tree> for Modifier<'tree> {
    fn text(&self) -> &'tree str {
        self.text
    }
}
impl ::treesitter_types::Spanned for Modifier<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        self.span
    }
}
#[derive(Debug, Clone)]
pub struct OperatorDeclaration<'tree> {
    pub span: ::treesitter_types::Span,
    pub children: ::std::vec::Vec<OperatorDeclarationChildren<'tree>>,
}
impl<'tree> ::treesitter_types::FromNode<'tree> for OperatorDeclaration<'tree> {
    #[allow(clippy::match_single_binding, clippy::suspicious_else_formatting)]
    fn from_node(
        node: ::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        debug_assert_eq!(node.kind(), "operator_declaration");
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
                        <OperatorDeclarationChildren as ::treesitter_types::FromNode>::from_node(
                            child, src,
                        )?,
                    );
                }
                items
            },
        })
    }
}
impl ::treesitter_types::Spanned for OperatorDeclaration<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        self.span
    }
}
#[derive(Debug, Clone)]
pub struct OptionalBinding<'tree> {
    pub span: ::treesitter_types::Span,
    pub children: ::std::vec::Vec<OptionalBindingChildren<'tree>>,
}
impl<'tree> ::treesitter_types::FromNode<'tree> for OptionalBinding<'tree> {
    #[allow(clippy::match_single_binding, clippy::suspicious_else_formatting)]
    fn from_node(
        node: ::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        debug_assert_eq!(node.kind(), "optional_binding");
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
                        <OptionalBindingChildren as ::treesitter_types::FromNode>::from_node(
                            child, src,
                        )?,
                    );
                }
                items
            },
        })
    }
}
impl ::treesitter_types::Spanned for OptionalBinding<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        self.span
    }
}
#[derive(Debug, Clone)]
pub struct OptionalBindingCondition<'tree> {
    pub span: ::treesitter_types::Span,
    pub children: ::std::vec::Vec<OptionalBinding<'tree>>,
}
impl<'tree> ::treesitter_types::FromNode<'tree> for OptionalBindingCondition<'tree> {
    #[allow(clippy::match_single_binding, clippy::suspicious_else_formatting)]
    fn from_node(
        node: ::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        debug_assert_eq!(node.kind(), "optional_binding_condition");
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
                        <OptionalBinding as ::treesitter_types::FromNode>::from_node(child, src)?,
                    );
                }
                items
            },
        })
    }
}
impl ::treesitter_types::Spanned for OptionalBindingCondition<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        self.span
    }
}
#[derive(Debug, Clone)]
pub struct OptionalPattern<'tree> {
    pub span: ::treesitter_types::Span,
    pub children: ::std::vec::Vec<OptionalPatternChildren<'tree>>,
}
impl<'tree> ::treesitter_types::FromNode<'tree> for OptionalPattern<'tree> {
    #[allow(clippy::match_single_binding, clippy::suspicious_else_formatting)]
    fn from_node(
        node: ::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        debug_assert_eq!(node.kind(), "optional_pattern");
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
                        <OptionalPatternChildren as ::treesitter_types::FromNode>::from_node(
                            child, src,
                        )?,
                    );
                }
                items
            },
        })
    }
}
impl ::treesitter_types::Spanned for OptionalPattern<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        self.span
    }
}
#[derive(Debug, Clone)]
pub struct ParameterList<'tree> {
    pub span: ::treesitter_types::Span,
    pub children: ::std::vec::Vec<ParameterDeclaration<'tree>>,
}
impl<'tree> ::treesitter_types::FromNode<'tree> for ParameterList<'tree> {
    #[allow(clippy::match_single_binding, clippy::suspicious_else_formatting)]
    fn from_node(
        node: ::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        debug_assert_eq!(node.kind(), "parameter_list");
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
                        <ParameterDeclaration as ::treesitter_types::FromNode>::from_node(
                            child, src,
                        )?,
                    );
                }
                items
            },
        })
    }
}
impl ::treesitter_types::Spanned for ParameterList<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        self.span
    }
}
#[derive(Debug, Clone)]
pub struct PrecedenceClause<'tree> {
    pub span: ::treesitter_types::Span,
    text: &'tree str,
}
impl<'tree> ::treesitter_types::FromNode<'tree> for PrecedenceClause<'tree> {
    fn from_node(
        node: ::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        debug_assert_eq!(node.kind(), "precedence_clause");
        Ok(Self {
            span: ::treesitter_types::Span::from(node),
            text: node.utf8_text(src)?,
        })
    }
}
impl<'tree> ::treesitter_types::LeafNode<'tree> for PrecedenceClause<'tree> {
    fn text(&self) -> &'tree str {
        self.text
    }
}
impl ::treesitter_types::Spanned for PrecedenceClause<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        self.span
    }
}
#[derive(Debug, Clone)]
pub struct Program<'tree> {
    pub span: ::treesitter_types::Span,
    pub children: ::std::vec::Vec<ProgramChildren<'tree>>,
}
impl<'tree> ::treesitter_types::FromNode<'tree> for Program<'tree> {
    #[allow(clippy::match_single_binding, clippy::suspicious_else_formatting)]
    fn from_node(
        node: ::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        debug_assert_eq!(node.kind(), "program");
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
                        <ProgramChildren as ::treesitter_types::FromNode>::from_node(child, src)?,
                    );
                }
                items
            },
        })
    }
}
impl ::treesitter_types::Spanned for Program<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        self.span
    }
}
#[derive(Debug, Clone)]
pub struct ProtocolDeclaration<'tree> {
    pub span: ::treesitter_types::Span,
    pub children: ::std::vec::Vec<ProtocolDeclarationChildren<'tree>>,
}
impl<'tree> ::treesitter_types::FromNode<'tree> for ProtocolDeclaration<'tree> {
    #[allow(clippy::match_single_binding, clippy::suspicious_else_formatting)]
    fn from_node(
        node: ::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        debug_assert_eq!(node.kind(), "protocol_declaration");
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
                        <ProtocolDeclarationChildren as ::treesitter_types::FromNode>::from_node(
                            child, src,
                        )?,
                    );
                }
                items
            },
        })
    }
}
impl ::treesitter_types::Spanned for ProtocolDeclaration<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        self.span
    }
}
#[derive(Debug, Clone)]
pub struct ProtocolInitializerDeclaration<'tree> {
    pub span: ::treesitter_types::Span,
    pub children: ::std::vec::Vec<ProtocolInitializerDeclarationChildren<'tree>>,
}
impl<'tree> ::treesitter_types::FromNode<'tree> for ProtocolInitializerDeclaration<'tree> {
    #[allow(clippy::match_single_binding, clippy::suspicious_else_formatting)]
    fn from_node(
        node: ::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        debug_assert_eq!(node.kind(), "protocol_initializer_declaration");
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
                            <ProtocolInitializerDeclarationChildren as ::treesitter_types::FromNode>::from_node(
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
impl ::treesitter_types::Spanned for ProtocolInitializerDeclaration<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        self.span
    }
}
#[derive(Debug, Clone)]
pub struct ProtocolMethodDeclaration<'tree> {
    pub span: ::treesitter_types::Span,
    pub name: ProtocolMethodDeclarationName<'tree>,
    pub r#return: ::std::vec::Vec<ProtocolMethodDeclarationReturn<'tree>>,
    pub children: ::std::vec::Vec<ProtocolMethodDeclarationChildren<'tree>>,
}
impl<'tree> ::treesitter_types::FromNode<'tree> for ProtocolMethodDeclaration<'tree> {
    #[allow(clippy::match_single_binding, clippy::suspicious_else_formatting)]
    fn from_node(
        node: ::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        debug_assert_eq!(node.kind(), "protocol_method_declaration");
        Ok(Self {
            span: ::treesitter_types::Span::from(node),
            name: {
                let child = node
                    .child_by_field_name("name")
                    .ok_or_else(|| ::treesitter_types::ParseError::missing_field("name", node))?;
                <ProtocolMethodDeclarationName as ::treesitter_types::FromNode>::from_node(
                    child, src,
                )?
            },
            r#return: {
                let mut cursor = node.walk();
                let mut items = ::std::vec::Vec::new();
                for child in node.children_by_field_name("return", &mut cursor) {
                    items
                        .push(
                            <ProtocolMethodDeclarationReturn as ::treesitter_types::FromNode>::from_node(
                                child,
                                src,
                            )?,
                        );
                }
                items
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
                    items
                        .push(
                            <ProtocolMethodDeclarationChildren as ::treesitter_types::FromNode>::from_node(
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
impl ::treesitter_types::Spanned for ProtocolMethodDeclaration<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        self.span
    }
}
#[derive(Debug, Clone)]
pub struct ProtocolSubscriptDeclaration<'tree> {
    pub span: ::treesitter_types::Span,
    pub children: ::std::vec::Vec<ProtocolSubscriptDeclarationChildren<'tree>>,
}
impl<'tree> ::treesitter_types::FromNode<'tree> for ProtocolSubscriptDeclaration<'tree> {
    #[allow(clippy::match_single_binding, clippy::suspicious_else_formatting)]
    fn from_node(
        node: ::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        debug_assert_eq!(node.kind(), "protocol_subscript_declaration");
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
                            <ProtocolSubscriptDeclarationChildren as ::treesitter_types::FromNode>::from_node(
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
impl ::treesitter_types::Spanned for ProtocolSubscriptDeclaration<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        self.span
    }
}
#[derive(Debug, Clone)]
pub struct ProtocolTypealiasDeclaration<'tree> {
    pub span: ::treesitter_types::Span,
    pub children: ::std::vec::Vec<ProtocolTypealiasDeclarationChildren<'tree>>,
}
impl<'tree> ::treesitter_types::FromNode<'tree> for ProtocolTypealiasDeclaration<'tree> {
    #[allow(clippy::match_single_binding, clippy::suspicious_else_formatting)]
    fn from_node(
        node: ::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        debug_assert_eq!(node.kind(), "protocol_typealias_declaration");
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
                            <ProtocolTypealiasDeclarationChildren as ::treesitter_types::FromNode>::from_node(
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
impl ::treesitter_types::Spanned for ProtocolTypealiasDeclaration<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        self.span
    }
}
#[derive(Debug, Clone)]
pub struct ProtocolVariableDeclaration<'tree> {
    pub span: ::treesitter_types::Span,
    pub children: ::std::vec::Vec<ProtocolVariableDeclarationChildren<'tree>>,
}
impl<'tree> ::treesitter_types::FromNode<'tree> for ProtocolVariableDeclaration<'tree> {
    #[allow(clippy::match_single_binding, clippy::suspicious_else_formatting)]
    fn from_node(
        node: ::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        debug_assert_eq!(node.kind(), "protocol_variable_declaration");
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
                            <ProtocolVariableDeclarationChildren as ::treesitter_types::FromNode>::from_node(
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
impl ::treesitter_types::Spanned for ProtocolVariableDeclaration<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        self.span
    }
}
#[derive(Debug, Clone)]
pub struct RepeatWhileStatement<'tree> {
    pub span: ::treesitter_types::Span,
    pub children: ::std::vec::Vec<RepeatWhileStatementChildren<'tree>>,
}
impl<'tree> ::treesitter_types::FromNode<'tree> for RepeatWhileStatement<'tree> {
    #[allow(clippy::match_single_binding, clippy::suspicious_else_formatting)]
    fn from_node(
        node: ::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        debug_assert_eq!(node.kind(), "repeat_while_statement");
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
                        <RepeatWhileStatementChildren as ::treesitter_types::FromNode>::from_node(
                            child, src,
                        )?,
                    );
                }
                items
            },
        })
    }
}
impl ::treesitter_types::Spanned for RepeatWhileStatement<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        self.span
    }
}
#[derive(Debug, Clone)]
pub struct ReturnStatement<'tree> {
    pub span: ::treesitter_types::Span,
    pub children: ::std::vec::Vec<ReturnStatementChildren<'tree>>,
}
impl<'tree> ::treesitter_types::FromNode<'tree> for ReturnStatement<'tree> {
    #[allow(clippy::match_single_binding, clippy::suspicious_else_formatting)]
    fn from_node(
        node: ::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        debug_assert_eq!(node.kind(), "return_statement");
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
                        <ReturnStatementChildren as ::treesitter_types::FromNode>::from_node(
                            child, src,
                        )?,
                    );
                }
                items
            },
        })
    }
}
impl ::treesitter_types::Spanned for ReturnStatement<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        self.span
    }
}
#[derive(Debug, Clone)]
pub struct StructDeclaration<'tree> {
    pub span: ::treesitter_types::Span,
    pub children: ::std::vec::Vec<StructDeclarationChildren<'tree>>,
}
impl<'tree> ::treesitter_types::FromNode<'tree> for StructDeclaration<'tree> {
    #[allow(clippy::match_single_binding, clippy::suspicious_else_formatting)]
    fn from_node(
        node: ::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        debug_assert_eq!(node.kind(), "struct_declaration");
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
                        <StructDeclarationChildren as ::treesitter_types::FromNode>::from_node(
                            child, src,
                        )?,
                    );
                }
                items
            },
        })
    }
}
impl ::treesitter_types::Spanned for StructDeclaration<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        self.span
    }
}
#[derive(Debug, Clone)]
pub struct SubscriptDeclaration<'tree> {
    pub span: ::treesitter_types::Span,
    pub children: ::std::vec::Vec<SubscriptDeclarationChildren<'tree>>,
}
impl<'tree> ::treesitter_types::FromNode<'tree> for SubscriptDeclaration<'tree> {
    #[allow(clippy::match_single_binding, clippy::suspicious_else_formatting)]
    fn from_node(
        node: ::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        debug_assert_eq!(node.kind(), "subscript_declaration");
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
                        <SubscriptDeclarationChildren as ::treesitter_types::FromNode>::from_node(
                            child, src,
                        )?,
                    );
                }
                items
            },
        })
    }
}
impl ::treesitter_types::Spanned for SubscriptDeclaration<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        self.span
    }
}
#[derive(Debug, Clone)]
pub struct SwitchStatement<'tree> {
    pub span: ::treesitter_types::Span,
    pub children: ::std::vec::Vec<SwitchStatementChildren<'tree>>,
}
impl<'tree> ::treesitter_types::FromNode<'tree> for SwitchStatement<'tree> {
    #[allow(clippy::match_single_binding, clippy::suspicious_else_formatting)]
    fn from_node(
        node: ::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        debug_assert_eq!(node.kind(), "switch_statement");
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
                        <SwitchStatementChildren as ::treesitter_types::FromNode>::from_node(
                            child, src,
                        )?,
                    );
                }
                items
            },
        })
    }
}
impl ::treesitter_types::Spanned for SwitchStatement<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        self.span
    }
}
#[derive(Debug, Clone)]
pub struct ThrowStatement<'tree> {
    pub span: ::treesitter_types::Span,
    pub children: ::std::vec::Vec<ThrowStatementChildren<'tree>>,
}
impl<'tree> ::treesitter_types::FromNode<'tree> for ThrowStatement<'tree> {
    #[allow(clippy::match_single_binding, clippy::suspicious_else_formatting)]
    fn from_node(
        node: ::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        debug_assert_eq!(node.kind(), "throw_statement");
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
                        <ThrowStatementChildren as ::treesitter_types::FromNode>::from_node(
                            child, src,
                        )?,
                    );
                }
                items
            },
        })
    }
}
impl ::treesitter_types::Spanned for ThrowStatement<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        self.span
    }
}
#[derive(Debug, Clone)]
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
#[derive(Debug, Clone)]
pub struct TupleType<'tree> {
    pub span: ::treesitter_types::Span,
    pub children: ::std::vec::Vec<TupleTypeChildren<'tree>>,
}
impl<'tree> ::treesitter_types::FromNode<'tree> for TupleType<'tree> {
    #[allow(clippy::match_single_binding, clippy::suspicious_else_formatting)]
    fn from_node(
        node: ::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        debug_assert_eq!(node.kind(), "tuple_type");
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
                        <TupleTypeChildren as ::treesitter_types::FromNode>::from_node(child, src)?,
                    );
                }
                items
            },
        })
    }
}
impl ::treesitter_types::Spanned for TupleType<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        self.span
    }
}
#[derive(Debug, Clone)]
pub struct Type<'tree> {
    pub span: ::treesitter_types::Span,
    pub children: ::std::vec::Vec<TypeChildren<'tree>>,
}
impl<'tree> ::treesitter_types::FromNode<'tree> for Type<'tree> {
    #[allow(clippy::match_single_binding, clippy::suspicious_else_formatting)]
    fn from_node(
        node: ::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        debug_assert_eq!(node.kind(), "type");
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
                    items.push(<TypeChildren as ::treesitter_types::FromNode>::from_node(
                        child, src,
                    )?);
                }
                items
            },
        })
    }
}
impl ::treesitter_types::Spanned for Type<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        self.span
    }
}
#[derive(Debug, Clone)]
pub struct TypealiasDeclaration<'tree> {
    pub span: ::treesitter_types::Span,
    pub children: ::std::vec::Vec<TypealiasDeclarationChildren<'tree>>,
}
impl<'tree> ::treesitter_types::FromNode<'tree> for TypealiasDeclaration<'tree> {
    #[allow(clippy::match_single_binding, clippy::suspicious_else_formatting)]
    fn from_node(
        node: ::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        debug_assert_eq!(node.kind(), "typealias_declaration");
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
                        <TypealiasDeclarationChildren as ::treesitter_types::FromNode>::from_node(
                            child, src,
                        )?,
                    );
                }
                items
            },
        })
    }
}
impl ::treesitter_types::Spanned for TypealiasDeclaration<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        self.span
    }
}
#[derive(Debug, Clone)]
pub struct ValueBindingPattern<'tree> {
    pub span: ::treesitter_types::Span,
    pub children: ::std::vec::Vec<ValueBindingPatternChildren<'tree>>,
}
impl<'tree> ::treesitter_types::FromNode<'tree> for ValueBindingPattern<'tree> {
    #[allow(clippy::match_single_binding, clippy::suspicious_else_formatting)]
    fn from_node(
        node: ::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        debug_assert_eq!(node.kind(), "value_binding_pattern");
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
                        <ValueBindingPatternChildren as ::treesitter_types::FromNode>::from_node(
                            child, src,
                        )?,
                    );
                }
                items
            },
        })
    }
}
impl ::treesitter_types::Spanned for ValueBindingPattern<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        self.span
    }
}
#[derive(Debug, Clone)]
pub struct VariableDeclaration<'tree> {
    pub span: ::treesitter_types::Span,
    pub name: ::std::vec::Vec<VariableDeclarationName<'tree>>,
    pub r#type: ::std::vec::Vec<VariableDeclarationType<'tree>>,
    pub value: ::std::vec::Vec<VariableDeclarationValue<'tree>>,
    pub children: ::std::vec::Vec<VariableDeclarationChildren<'tree>>,
}
impl<'tree> ::treesitter_types::FromNode<'tree> for VariableDeclaration<'tree> {
    #[allow(clippy::match_single_binding, clippy::suspicious_else_formatting)]
    fn from_node(
        node: ::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        debug_assert_eq!(node.kind(), "variable_declaration");
        Ok(Self {
            span: ::treesitter_types::Span::from(node),
            name: {
                let mut cursor = node.walk();
                let mut items = ::std::vec::Vec::new();
                for child in node.children_by_field_name("name", &mut cursor) {
                    items.push(
                        <VariableDeclarationName as ::treesitter_types::FromNode>::from_node(
                            child, src,
                        )?,
                    );
                }
                items
            },
            r#type: {
                let mut cursor = node.walk();
                let mut items = ::std::vec::Vec::new();
                for child in node.children_by_field_name("type", &mut cursor) {
                    items.push(
                        <VariableDeclarationType as ::treesitter_types::FromNode>::from_node(
                            child, src,
                        )?,
                    );
                }
                items
            },
            value: {
                let mut cursor = node.walk();
                let mut items = ::std::vec::Vec::new();
                for child in node.children_by_field_name("value", &mut cursor) {
                    items.push(
                        <VariableDeclarationValue as ::treesitter_types::FromNode>::from_node(
                            child, src,
                        )?,
                    );
                }
                items
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
                        <VariableDeclarationChildren as ::treesitter_types::FromNode>::from_node(
                            child, src,
                        )?,
                    );
                }
                items
            },
        })
    }
}
impl ::treesitter_types::Spanned for VariableDeclaration<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        self.span
    }
}
#[derive(Debug, Clone)]
pub struct WhileStatement<'tree> {
    pub span: ::treesitter_types::Span,
    pub children: ::std::vec::Vec<WhileStatementChildren<'tree>>,
}
impl<'tree> ::treesitter_types::FromNode<'tree> for WhileStatement<'tree> {
    #[allow(clippy::match_single_binding, clippy::suspicious_else_formatting)]
    fn from_node(
        node: ::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        debug_assert_eq!(node.kind(), "while_statement");
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
                        <WhileStatementChildren as ::treesitter_types::FromNode>::from_node(
                            child, src,
                        )?,
                    );
                }
                items
            },
        })
    }
}
impl ::treesitter_types::Spanned for WhileStatement<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        self.span
    }
}
#[derive(Debug, Clone)]
pub struct WildcardPattern<'tree> {
    pub span: ::treesitter_types::Span,
    text: &'tree str,
}
impl<'tree> ::treesitter_types::FromNode<'tree> for WildcardPattern<'tree> {
    fn from_node(
        node: ::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        debug_assert_eq!(node.kind(), "wildcard_pattern");
        Ok(Self {
            span: ::treesitter_types::Span::from(node),
            text: node.utf8_text(src)?,
        })
    }
}
impl<'tree> ::treesitter_types::LeafNode<'tree> for WildcardPattern<'tree> {
    fn text(&self) -> &'tree str {
        self.text
    }
}
impl ::treesitter_types::Spanned for WildcardPattern<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        self.span
    }
}
#[derive(Debug, Clone)]
pub struct FallthroughStatement<'tree> {
    pub span: ::treesitter_types::Span,
    text: &'tree str,
}
impl<'tree> ::treesitter_types::FromNode<'tree> for FallthroughStatement<'tree> {
    fn from_node(
        node: ::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        debug_assert_eq!(node.kind(), "fallthrough_statement");
        Ok(Self {
            span: ::treesitter_types::Span::from(node),
            text: node.utf8_text(src)?,
        })
    }
}
impl<'tree> ::treesitter_types::LeafNode<'tree> for FallthroughStatement<'tree> {
    fn text(&self) -> &'tree str {
        self.text
    }
}
impl ::treesitter_types::Spanned for FallthroughStatement<'_> {
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
#[derive(Debug, Clone)]
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
#[derive(Debug, Clone)]
pub struct Operator<'tree> {
    pub span: ::treesitter_types::Span,
    text: &'tree str,
}
impl<'tree> ::treesitter_types::FromNode<'tree> for Operator<'tree> {
    fn from_node(
        node: ::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        debug_assert_eq!(node.kind(), "operator");
        Ok(Self {
            span: ::treesitter_types::Span::from(node),
            text: node.utf8_text(src)?,
        })
    }
}
impl<'tree> ::treesitter_types::LeafNode<'tree> for Operator<'tree> {
    fn text(&self) -> &'tree str {
        self.text
    }
}
impl ::treesitter_types::Spanned for Operator<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        self.span
    }
}
#[derive(Debug, Clone)]
pub struct SemanticVersion<'tree> {
    pub span: ::treesitter_types::Span,
    text: &'tree str,
}
impl<'tree> ::treesitter_types::FromNode<'tree> for SemanticVersion<'tree> {
    fn from_node(
        node: ::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        debug_assert_eq!(node.kind(), "semantic_version");
        Ok(Self {
            span: ::treesitter_types::Span::from(node),
            text: node.utf8_text(src)?,
        })
    }
}
impl<'tree> ::treesitter_types::LeafNode<'tree> for SemanticVersion<'tree> {
    fn text(&self) -> &'tree str {
        self.text
    }
}
impl ::treesitter_types::Spanned for SemanticVersion<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        self.span
    }
}
#[derive(Debug, Clone)]
pub struct StandardType<'tree> {
    pub span: ::treesitter_types::Span,
    text: &'tree str,
}
impl<'tree> ::treesitter_types::FromNode<'tree> for StandardType<'tree> {
    fn from_node(
        node: ::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        debug_assert_eq!(node.kind(), "standard_type");
        Ok(Self {
            span: ::treesitter_types::Span::from(node),
            text: node.utf8_text(src)?,
        })
    }
}
impl<'tree> ::treesitter_types::LeafNode<'tree> for StandardType<'tree> {
    fn text(&self) -> &'tree str {
        self.text
    }
}
impl ::treesitter_types::Spanned for StandardType<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        self.span
    }
}
#[derive(Debug, Clone)]
pub struct StaticStringLiteral<'tree> {
    pub span: ::treesitter_types::Span,
    text: &'tree str,
}
impl<'tree> ::treesitter_types::FromNode<'tree> for StaticStringLiteral<'tree> {
    fn from_node(
        node: ::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        debug_assert_eq!(node.kind(), "static_string_literal");
        Ok(Self {
            span: ::treesitter_types::Span::from(node),
            text: node.utf8_text(src)?,
        })
    }
}
impl<'tree> ::treesitter_types::LeafNode<'tree> for StaticStringLiteral<'tree> {
    fn text(&self) -> &'tree str {
        self.text
    }
}
impl ::treesitter_types::Spanned for StaticStringLiteral<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        self.span
    }
}
#[derive(Debug, Clone)]
pub struct String<'tree> {
    pub span: ::treesitter_types::Span,
    text: &'tree str,
}
impl<'tree> ::treesitter_types::FromNode<'tree> for String<'tree> {
    fn from_node(
        node: ::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        debug_assert_eq!(node.kind(), "string");
        Ok(Self {
            span: ::treesitter_types::Span::from(node),
            text: node.utf8_text(src)?,
        })
    }
}
impl<'tree> ::treesitter_types::LeafNode<'tree> for String<'tree> {
    fn text(&self) -> &'tree str {
        self.text
    }
}
impl ::treesitter_types::Spanned for String<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        self.span
    }
}
#[derive(Debug, Clone)]
pub struct TypeIdentifier<'tree> {
    pub span: ::treesitter_types::Span,
    text: &'tree str,
}
impl<'tree> ::treesitter_types::FromNode<'tree> for TypeIdentifier<'tree> {
    fn from_node(
        node: ::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        debug_assert_eq!(node.kind(), "type_identifier");
        Ok(Self {
            span: ::treesitter_types::Span::from(node),
            text: node.utf8_text(src)?,
        })
    }
}
impl<'tree> ::treesitter_types::LeafNode<'tree> for TypeIdentifier<'tree> {
    fn text(&self) -> &'tree str {
        self.text
    }
}
impl ::treesitter_types::Spanned for TypeIdentifier<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        self.span
    }
}
#[derive(Debug, Clone)]
pub struct ParameterDeclaration<'tree> {
    pub span: ::treesitter_types::Span,
    text: &'tree str,
}
impl<'tree> ::treesitter_types::FromNode<'tree> for ParameterDeclaration<'tree> {
    fn from_node(
        node: ::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        debug_assert_eq!(node.kind(), "parameter_declaration");
        Ok(Self {
            span: ::treesitter_types::Span::from(node),
            text: node.utf8_text(src)?,
        })
    }
}
impl<'tree> ::treesitter_types::LeafNode<'tree> for ParameterDeclaration<'tree> {
    fn text(&self) -> &'tree str {
        self.text
    }
}
impl ::treesitter_types::Spanned for ParameterDeclaration<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        self.span
    }
}
#[derive(Debug, Clone)]
pub enum ArrayTypeChildren<'tree> {
    ArrayType(::std::boxed::Box<ArrayType<'tree>>),
    DictionaryType(::std::boxed::Box<DictionaryType<'tree>>),
    StandardType(::std::boxed::Box<StandardType<'tree>>),
    TupleType(::std::boxed::Box<TupleType<'tree>>),
    TypeIdentifier(::std::boxed::Box<TypeIdentifier<'tree>>),
}
impl<'tree> ::treesitter_types::FromNode<'tree> for ArrayTypeChildren<'tree> {
    #[allow(clippy::collapsible_else_if)]
    fn from_node(
        node: ::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        match node.kind() {
            "array_type" => Ok(Self::ArrayType(::std::boxed::Box::new(
                <ArrayType as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "dictionary_type" => Ok(Self::DictionaryType(::std::boxed::Box::new(
                <DictionaryType as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "standard_type" => Ok(Self::StandardType(::std::boxed::Box::new(
                <StandardType as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "tuple_type" => Ok(Self::TupleType(::std::boxed::Box::new(
                <TupleType as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "type_identifier" => Ok(Self::TypeIdentifier(::std::boxed::Box::new(
                <TypeIdentifier as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            other => Err(::treesitter_types::ParseError::unexpected_kind(other, node)),
        }
    }
}
impl ::treesitter_types::Spanned for ArrayTypeChildren<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        match self {
            Self::ArrayType(inner) => inner.span(),
            Self::DictionaryType(inner) => inner.span(),
            Self::StandardType(inner) => inner.span(),
            Self::TupleType(inner) => inner.span(),
            Self::TypeIdentifier(inner) => inner.span(),
        }
    }
}
#[derive(Debug, Clone)]
pub enum AsPatternChildren<'tree> {
    AsPattern(::std::boxed::Box<AsPattern<'tree>>),
    Boolean(::std::boxed::Box<Boolean<'tree>>),
    EnumCasePattern(::std::boxed::Box<EnumCasePattern<'tree>>),
    Identifier(::std::boxed::Box<Identifier<'tree>>),
    IsPattern(::std::boxed::Box<IsPattern<'tree>>),
    Nil(::std::boxed::Box<Nil<'tree>>),
    Number(::std::boxed::Box<Number<'tree>>),
    OptionalPattern(::std::boxed::Box<OptionalPattern<'tree>>),
    String(::std::boxed::Box<String<'tree>>),
    Type(::std::boxed::Box<Type<'tree>>),
    ValueBindingPattern(::std::boxed::Box<ValueBindingPattern<'tree>>),
    WildcardPattern(::std::boxed::Box<WildcardPattern<'tree>>),
}
impl<'tree> ::treesitter_types::FromNode<'tree> for AsPatternChildren<'tree> {
    #[allow(clippy::collapsible_else_if)]
    fn from_node(
        node: ::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        match node.kind() {
            "as_pattern" => Ok(Self::AsPattern(::std::boxed::Box::new(
                <AsPattern as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "boolean" => Ok(Self::Boolean(::std::boxed::Box::new(
                <Boolean as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "enum_case_pattern" => Ok(Self::EnumCasePattern(::std::boxed::Box::new(
                <EnumCasePattern as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "identifier" => Ok(Self::Identifier(::std::boxed::Box::new(
                <Identifier as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "is_pattern" => Ok(Self::IsPattern(::std::boxed::Box::new(
                <IsPattern as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "nil" => Ok(Self::Nil(::std::boxed::Box::new(
                <Nil as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "number" => Ok(Self::Number(::std::boxed::Box::new(
                <Number as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "optional_pattern" => Ok(Self::OptionalPattern(::std::boxed::Box::new(
                <OptionalPattern as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "string" => Ok(Self::String(::std::boxed::Box::new(
                <String as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "type" => Ok(Self::Type(::std::boxed::Box::new(
                <Type as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "value_binding_pattern" => Ok(Self::ValueBindingPattern(::std::boxed::Box::new(
                <ValueBindingPattern as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "wildcard_pattern" => Ok(Self::WildcardPattern(::std::boxed::Box::new(
                <WildcardPattern as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            other => Err(::treesitter_types::ParseError::unexpected_kind(other, node)),
        }
    }
}
impl ::treesitter_types::Spanned for AsPatternChildren<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        match self {
            Self::AsPattern(inner) => inner.span(),
            Self::Boolean(inner) => inner.span(),
            Self::EnumCasePattern(inner) => inner.span(),
            Self::Identifier(inner) => inner.span(),
            Self::IsPattern(inner) => inner.span(),
            Self::Nil(inner) => inner.span(),
            Self::Number(inner) => inner.span(),
            Self::OptionalPattern(inner) => inner.span(),
            Self::String(inner) => inner.span(),
            Self::Type(inner) => inner.span(),
            Self::ValueBindingPattern(inner) => inner.span(),
            Self::WildcardPattern(inner) => inner.span(),
        }
    }
}
#[derive(Debug, Clone)]
pub enum AssociatedtypeDeclarationChildren<'tree> {
    ArrayType(::std::boxed::Box<ArrayType<'tree>>),
    DictionaryType(::std::boxed::Box<DictionaryType<'tree>>),
    Identifier(::std::boxed::Box<Identifier<'tree>>),
    Type(::std::boxed::Box<Type<'tree>>),
}
impl<'tree> ::treesitter_types::FromNode<'tree> for AssociatedtypeDeclarationChildren<'tree> {
    #[allow(clippy::collapsible_else_if)]
    fn from_node(
        node: ::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        match node.kind() {
            "array_type" => Ok(Self::ArrayType(::std::boxed::Box::new(
                <ArrayType as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "dictionary_type" => Ok(Self::DictionaryType(::std::boxed::Box::new(
                <DictionaryType as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "identifier" => Ok(Self::Identifier(::std::boxed::Box::new(
                <Identifier as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "type" => Ok(Self::Type(::std::boxed::Box::new(
                <Type as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            other => Err(::treesitter_types::ParseError::unexpected_kind(other, node)),
        }
    }
}
impl ::treesitter_types::Spanned for AssociatedtypeDeclarationChildren<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        match self {
            Self::ArrayType(inner) => inner.span(),
            Self::DictionaryType(inner) => inner.span(),
            Self::Identifier(inner) => inner.span(),
            Self::Type(inner) => inner.span(),
        }
    }
}
#[derive(Debug, Clone)]
pub enum BuildConfigurationStatementChildren<'tree> {
    Boolean(::std::boxed::Box<Boolean<'tree>>),
    BooleanLiteral(::std::boxed::Box<BooleanLiteral<'tree>>),
    BreakStatement(::std::boxed::Box<BreakStatement<'tree>>),
    BuildConfigurationStatement(::std::boxed::Box<BuildConfigurationStatement<'tree>>),
    ClassDeclaration(::std::boxed::Box<ClassDeclaration<'tree>>),
    ConstantDeclaration(::std::boxed::Box<ConstantDeclaration<'tree>>),
    ContinueStatement(::std::boxed::Box<ContinueStatement<'tree>>),
    DeferStatement(::std::boxed::Box<DeferStatement<'tree>>),
    DeinitializerDeclaration(::std::boxed::Box<DeinitializerDeclaration<'tree>>),
    DiagnosticStatement(::std::boxed::Box<DiagnosticStatement<'tree>>),
    DoStatement(::std::boxed::Box<DoStatement<'tree>>),
    EnumDeclaration(::std::boxed::Box<EnumDeclaration<'tree>>),
    ExtensionDeclaration(::std::boxed::Box<ExtensionDeclaration<'tree>>),
    FallthroughStatement(::std::boxed::Box<FallthroughStatement<'tree>>),
    ForStatement(::std::boxed::Box<ForStatement<'tree>>),
    FunctionDeclaration(::std::boxed::Box<FunctionDeclaration<'tree>>),
    GuardStatement(::std::boxed::Box<GuardStatement<'tree>>),
    Identifier(::std::boxed::Box<Identifier<'tree>>),
    IfStatement(::std::boxed::Box<IfStatement<'tree>>),
    ImportDeclaration(::std::boxed::Box<ImportDeclaration<'tree>>),
    InitializerDeclaration(::std::boxed::Box<InitializerDeclaration<'tree>>),
    LabeledStatement(::std::boxed::Box<LabeledStatement<'tree>>),
    LineControlStatement(::std::boxed::Box<LineControlStatement<'tree>>),
    Nil(::std::boxed::Box<Nil<'tree>>),
    Number(::std::boxed::Box<Number<'tree>>),
    OperatorDeclaration(::std::boxed::Box<OperatorDeclaration<'tree>>),
    ProtocolDeclaration(::std::boxed::Box<ProtocolDeclaration<'tree>>),
    RepeatWhileStatement(::std::boxed::Box<RepeatWhileStatement<'tree>>),
    ReturnStatement(::std::boxed::Box<ReturnStatement<'tree>>),
    SemanticVersion(::std::boxed::Box<SemanticVersion<'tree>>),
    String(::std::boxed::Box<String<'tree>>),
    StructDeclaration(::std::boxed::Box<StructDeclaration<'tree>>),
    SubscriptDeclaration(::std::boxed::Box<SubscriptDeclaration<'tree>>),
    SwitchStatement(::std::boxed::Box<SwitchStatement<'tree>>),
    ThrowStatement(::std::boxed::Box<ThrowStatement<'tree>>),
    TypealiasDeclaration(::std::boxed::Box<TypealiasDeclaration<'tree>>),
    VariableDeclaration(::std::boxed::Box<VariableDeclaration<'tree>>),
    WhileStatement(::std::boxed::Box<WhileStatement<'tree>>),
}
impl<'tree> ::treesitter_types::FromNode<'tree> for BuildConfigurationStatementChildren<'tree> {
    #[allow(clippy::collapsible_else_if)]
    fn from_node(
        node: ::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        match node.kind() {
            "boolean" => Ok(Self::Boolean(::std::boxed::Box::new(
                <Boolean as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "boolean_literal" => Ok(Self::BooleanLiteral(::std::boxed::Box::new(
                <BooleanLiteral as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "break_statement" => Ok(Self::BreakStatement(::std::boxed::Box::new(
                <BreakStatement as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "build_configuration_statement" => {
                Ok(Self::BuildConfigurationStatement(::std::boxed::Box::new(
                    <BuildConfigurationStatement as ::treesitter_types::FromNode>::from_node(
                        node, src,
                    )?,
                )))
            }
            "class_declaration" => Ok(Self::ClassDeclaration(::std::boxed::Box::new(
                <ClassDeclaration as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "constant_declaration" => Ok(Self::ConstantDeclaration(::std::boxed::Box::new(
                <ConstantDeclaration as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "continue_statement" => Ok(Self::ContinueStatement(::std::boxed::Box::new(
                <ContinueStatement as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "defer_statement" => Ok(Self::DeferStatement(::std::boxed::Box::new(
                <DeferStatement as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "deinitializer_declaration" => {
                Ok(Self::DeinitializerDeclaration(::std::boxed::Box::new(
                    <DeinitializerDeclaration as ::treesitter_types::FromNode>::from_node(
                        node, src,
                    )?,
                )))
            }
            "diagnostic_statement" => Ok(Self::DiagnosticStatement(::std::boxed::Box::new(
                <DiagnosticStatement as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "do_statement" => Ok(Self::DoStatement(::std::boxed::Box::new(
                <DoStatement as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "enum_declaration" => Ok(Self::EnumDeclaration(::std::boxed::Box::new(
                <EnumDeclaration as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "extension_declaration" => Ok(Self::ExtensionDeclaration(::std::boxed::Box::new(
                <ExtensionDeclaration as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "fallthrough_statement" => Ok(Self::FallthroughStatement(::std::boxed::Box::new(
                <FallthroughStatement as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "for_statement" => Ok(Self::ForStatement(::std::boxed::Box::new(
                <ForStatement as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "function_declaration" => Ok(Self::FunctionDeclaration(::std::boxed::Box::new(
                <FunctionDeclaration as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "guard_statement" => Ok(Self::GuardStatement(::std::boxed::Box::new(
                <GuardStatement as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "identifier" => Ok(Self::Identifier(::std::boxed::Box::new(
                <Identifier as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "if_statement" => Ok(Self::IfStatement(::std::boxed::Box::new(
                <IfStatement as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "import_declaration" => Ok(Self::ImportDeclaration(::std::boxed::Box::new(
                <ImportDeclaration as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "initializer_declaration" => Ok(Self::InitializerDeclaration(::std::boxed::Box::new(
                <InitializerDeclaration as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "labeled_statement" => Ok(Self::LabeledStatement(::std::boxed::Box::new(
                <LabeledStatement as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "line_control_statement" => Ok(Self::LineControlStatement(::std::boxed::Box::new(
                <LineControlStatement as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "nil" => Ok(Self::Nil(::std::boxed::Box::new(
                <Nil as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "number" => Ok(Self::Number(::std::boxed::Box::new(
                <Number as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "operator_declaration" => Ok(Self::OperatorDeclaration(::std::boxed::Box::new(
                <OperatorDeclaration as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "protocol_declaration" => Ok(Self::ProtocolDeclaration(::std::boxed::Box::new(
                <ProtocolDeclaration as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "repeat_while_statement" => Ok(Self::RepeatWhileStatement(::std::boxed::Box::new(
                <RepeatWhileStatement as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "return_statement" => Ok(Self::ReturnStatement(::std::boxed::Box::new(
                <ReturnStatement as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "semantic_version" => Ok(Self::SemanticVersion(::std::boxed::Box::new(
                <SemanticVersion as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "string" => Ok(Self::String(::std::boxed::Box::new(
                <String as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "struct_declaration" => Ok(Self::StructDeclaration(::std::boxed::Box::new(
                <StructDeclaration as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "subscript_declaration" => Ok(Self::SubscriptDeclaration(::std::boxed::Box::new(
                <SubscriptDeclaration as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "switch_statement" => Ok(Self::SwitchStatement(::std::boxed::Box::new(
                <SwitchStatement as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "throw_statement" => Ok(Self::ThrowStatement(::std::boxed::Box::new(
                <ThrowStatement as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "typealias_declaration" => Ok(Self::TypealiasDeclaration(::std::boxed::Box::new(
                <TypealiasDeclaration as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "variable_declaration" => Ok(Self::VariableDeclaration(::std::boxed::Box::new(
                <VariableDeclaration as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "while_statement" => Ok(Self::WhileStatement(::std::boxed::Box::new(
                <WhileStatement as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            other => Err(::treesitter_types::ParseError::unexpected_kind(other, node)),
        }
    }
}
impl ::treesitter_types::Spanned for BuildConfigurationStatementChildren<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        match self {
            Self::Boolean(inner) => inner.span(),
            Self::BooleanLiteral(inner) => inner.span(),
            Self::BreakStatement(inner) => inner.span(),
            Self::BuildConfigurationStatement(inner) => inner.span(),
            Self::ClassDeclaration(inner) => inner.span(),
            Self::ConstantDeclaration(inner) => inner.span(),
            Self::ContinueStatement(inner) => inner.span(),
            Self::DeferStatement(inner) => inner.span(),
            Self::DeinitializerDeclaration(inner) => inner.span(),
            Self::DiagnosticStatement(inner) => inner.span(),
            Self::DoStatement(inner) => inner.span(),
            Self::EnumDeclaration(inner) => inner.span(),
            Self::ExtensionDeclaration(inner) => inner.span(),
            Self::FallthroughStatement(inner) => inner.span(),
            Self::ForStatement(inner) => inner.span(),
            Self::FunctionDeclaration(inner) => inner.span(),
            Self::GuardStatement(inner) => inner.span(),
            Self::Identifier(inner) => inner.span(),
            Self::IfStatement(inner) => inner.span(),
            Self::ImportDeclaration(inner) => inner.span(),
            Self::InitializerDeclaration(inner) => inner.span(),
            Self::LabeledStatement(inner) => inner.span(),
            Self::LineControlStatement(inner) => inner.span(),
            Self::Nil(inner) => inner.span(),
            Self::Number(inner) => inner.span(),
            Self::OperatorDeclaration(inner) => inner.span(),
            Self::ProtocolDeclaration(inner) => inner.span(),
            Self::RepeatWhileStatement(inner) => inner.span(),
            Self::ReturnStatement(inner) => inner.span(),
            Self::SemanticVersion(inner) => inner.span(),
            Self::String(inner) => inner.span(),
            Self::StructDeclaration(inner) => inner.span(),
            Self::SubscriptDeclaration(inner) => inner.span(),
            Self::SwitchStatement(inner) => inner.span(),
            Self::ThrowStatement(inner) => inner.span(),
            Self::TypealiasDeclaration(inner) => inner.span(),
            Self::VariableDeclaration(inner) => inner.span(),
            Self::WhileStatement(inner) => inner.span(),
        }
    }
}
#[derive(Debug, Clone)]
pub enum CaseConditionChildren<'tree> {
    AsPattern(::std::boxed::Box<AsPattern<'tree>>),
    Boolean(::std::boxed::Box<Boolean<'tree>>),
    EnumCasePattern(::std::boxed::Box<EnumCasePattern<'tree>>),
    Identifier(::std::boxed::Box<Identifier<'tree>>),
    IsPattern(::std::boxed::Box<IsPattern<'tree>>),
    Nil(::std::boxed::Box<Nil<'tree>>),
    Number(::std::boxed::Box<Number<'tree>>),
    OptionalPattern(::std::boxed::Box<OptionalPattern<'tree>>),
    String(::std::boxed::Box<String<'tree>>),
    ValueBindingPattern(::std::boxed::Box<ValueBindingPattern<'tree>>),
    WildcardPattern(::std::boxed::Box<WildcardPattern<'tree>>),
}
impl<'tree> ::treesitter_types::FromNode<'tree> for CaseConditionChildren<'tree> {
    #[allow(clippy::collapsible_else_if)]
    fn from_node(
        node: ::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        match node.kind() {
            "as_pattern" => Ok(Self::AsPattern(::std::boxed::Box::new(
                <AsPattern as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "boolean" => Ok(Self::Boolean(::std::boxed::Box::new(
                <Boolean as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "enum_case_pattern" => Ok(Self::EnumCasePattern(::std::boxed::Box::new(
                <EnumCasePattern as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "identifier" => Ok(Self::Identifier(::std::boxed::Box::new(
                <Identifier as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "is_pattern" => Ok(Self::IsPattern(::std::boxed::Box::new(
                <IsPattern as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "nil" => Ok(Self::Nil(::std::boxed::Box::new(
                <Nil as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "number" => Ok(Self::Number(::std::boxed::Box::new(
                <Number as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "optional_pattern" => Ok(Self::OptionalPattern(::std::boxed::Box::new(
                <OptionalPattern as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "string" => Ok(Self::String(::std::boxed::Box::new(
                <String as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "value_binding_pattern" => Ok(Self::ValueBindingPattern(::std::boxed::Box::new(
                <ValueBindingPattern as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "wildcard_pattern" => Ok(Self::WildcardPattern(::std::boxed::Box::new(
                <WildcardPattern as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            other => Err(::treesitter_types::ParseError::unexpected_kind(other, node)),
        }
    }
}
impl ::treesitter_types::Spanned for CaseConditionChildren<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        match self {
            Self::AsPattern(inner) => inner.span(),
            Self::Boolean(inner) => inner.span(),
            Self::EnumCasePattern(inner) => inner.span(),
            Self::Identifier(inner) => inner.span(),
            Self::IsPattern(inner) => inner.span(),
            Self::Nil(inner) => inner.span(),
            Self::Number(inner) => inner.span(),
            Self::OptionalPattern(inner) => inner.span(),
            Self::String(inner) => inner.span(),
            Self::ValueBindingPattern(inner) => inner.span(),
            Self::WildcardPattern(inner) => inner.span(),
        }
    }
}
#[derive(Debug, Clone)]
pub enum CaseDeclarationChildren<'tree> {
    BooleanLiteral(::std::boxed::Box<BooleanLiteral<'tree>>),
    Identifier(::std::boxed::Box<Identifier<'tree>>),
    StaticStringLiteral(::std::boxed::Box<StaticStringLiteral<'tree>>),
    TupleType(::std::boxed::Box<TupleType<'tree>>),
}
impl<'tree> ::treesitter_types::FromNode<'tree> for CaseDeclarationChildren<'tree> {
    #[allow(clippy::collapsible_else_if)]
    fn from_node(
        node: ::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        match node.kind() {
            "boolean_literal" => Ok(Self::BooleanLiteral(::std::boxed::Box::new(
                <BooleanLiteral as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "identifier" => Ok(Self::Identifier(::std::boxed::Box::new(
                <Identifier as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "static_string_literal" => Ok(Self::StaticStringLiteral(::std::boxed::Box::new(
                <StaticStringLiteral as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "tuple_type" => Ok(Self::TupleType(::std::boxed::Box::new(
                <TupleType as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            other => Err(::treesitter_types::ParseError::unexpected_kind(other, node)),
        }
    }
}
impl ::treesitter_types::Spanned for CaseDeclarationChildren<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        match self {
            Self::BooleanLiteral(inner) => inner.span(),
            Self::Identifier(inner) => inner.span(),
            Self::StaticStringLiteral(inner) => inner.span(),
            Self::TupleType(inner) => inner.span(),
        }
    }
}
#[derive(Debug, Clone)]
pub enum CaseStatementChildren<'tree> {
    AsPattern(::std::boxed::Box<AsPattern<'tree>>),
    Boolean(::std::boxed::Box<Boolean<'tree>>),
    BreakStatement(::std::boxed::Box<BreakStatement<'tree>>),
    BuildConfigurationStatement(::std::boxed::Box<BuildConfigurationStatement<'tree>>),
    ClassDeclaration(::std::boxed::Box<ClassDeclaration<'tree>>),
    ConstantDeclaration(::std::boxed::Box<ConstantDeclaration<'tree>>),
    ContinueStatement(::std::boxed::Box<ContinueStatement<'tree>>),
    DeferStatement(::std::boxed::Box<DeferStatement<'tree>>),
    DeinitializerDeclaration(::std::boxed::Box<DeinitializerDeclaration<'tree>>),
    DiagnosticStatement(::std::boxed::Box<DiagnosticStatement<'tree>>),
    DoStatement(::std::boxed::Box<DoStatement<'tree>>),
    EnumCasePattern(::std::boxed::Box<EnumCasePattern<'tree>>),
    EnumDeclaration(::std::boxed::Box<EnumDeclaration<'tree>>),
    ExtensionDeclaration(::std::boxed::Box<ExtensionDeclaration<'tree>>),
    FallthroughStatement(::std::boxed::Box<FallthroughStatement<'tree>>),
    ForStatement(::std::boxed::Box<ForStatement<'tree>>),
    FunctionDeclaration(::std::boxed::Box<FunctionDeclaration<'tree>>),
    GuardStatement(::std::boxed::Box<GuardStatement<'tree>>),
    Identifier(::std::boxed::Box<Identifier<'tree>>),
    IfStatement(::std::boxed::Box<IfStatement<'tree>>),
    ImportDeclaration(::std::boxed::Box<ImportDeclaration<'tree>>),
    InitializerDeclaration(::std::boxed::Box<InitializerDeclaration<'tree>>),
    IsPattern(::std::boxed::Box<IsPattern<'tree>>),
    LabeledStatement(::std::boxed::Box<LabeledStatement<'tree>>),
    LineControlStatement(::std::boxed::Box<LineControlStatement<'tree>>),
    Nil(::std::boxed::Box<Nil<'tree>>),
    Number(::std::boxed::Box<Number<'tree>>),
    OperatorDeclaration(::std::boxed::Box<OperatorDeclaration<'tree>>),
    OptionalPattern(::std::boxed::Box<OptionalPattern<'tree>>),
    ProtocolDeclaration(::std::boxed::Box<ProtocolDeclaration<'tree>>),
    RepeatWhileStatement(::std::boxed::Box<RepeatWhileStatement<'tree>>),
    ReturnStatement(::std::boxed::Box<ReturnStatement<'tree>>),
    String(::std::boxed::Box<String<'tree>>),
    StructDeclaration(::std::boxed::Box<StructDeclaration<'tree>>),
    SubscriptDeclaration(::std::boxed::Box<SubscriptDeclaration<'tree>>),
    SwitchStatement(::std::boxed::Box<SwitchStatement<'tree>>),
    ThrowStatement(::std::boxed::Box<ThrowStatement<'tree>>),
    TypealiasDeclaration(::std::boxed::Box<TypealiasDeclaration<'tree>>),
    ValueBindingPattern(::std::boxed::Box<ValueBindingPattern<'tree>>),
    VariableDeclaration(::std::boxed::Box<VariableDeclaration<'tree>>),
    WhileStatement(::std::boxed::Box<WhileStatement<'tree>>),
    WildcardPattern(::std::boxed::Box<WildcardPattern<'tree>>),
}
impl<'tree> ::treesitter_types::FromNode<'tree> for CaseStatementChildren<'tree> {
    #[allow(clippy::collapsible_else_if)]
    fn from_node(
        node: ::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        match node.kind() {
            "as_pattern" => Ok(Self::AsPattern(::std::boxed::Box::new(
                <AsPattern as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "boolean" => Ok(Self::Boolean(::std::boxed::Box::new(
                <Boolean as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "break_statement" => Ok(Self::BreakStatement(::std::boxed::Box::new(
                <BreakStatement as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "build_configuration_statement" => {
                Ok(Self::BuildConfigurationStatement(::std::boxed::Box::new(
                    <BuildConfigurationStatement as ::treesitter_types::FromNode>::from_node(
                        node, src,
                    )?,
                )))
            }
            "class_declaration" => Ok(Self::ClassDeclaration(::std::boxed::Box::new(
                <ClassDeclaration as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "constant_declaration" => Ok(Self::ConstantDeclaration(::std::boxed::Box::new(
                <ConstantDeclaration as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "continue_statement" => Ok(Self::ContinueStatement(::std::boxed::Box::new(
                <ContinueStatement as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "defer_statement" => Ok(Self::DeferStatement(::std::boxed::Box::new(
                <DeferStatement as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "deinitializer_declaration" => {
                Ok(Self::DeinitializerDeclaration(::std::boxed::Box::new(
                    <DeinitializerDeclaration as ::treesitter_types::FromNode>::from_node(
                        node, src,
                    )?,
                )))
            }
            "diagnostic_statement" => Ok(Self::DiagnosticStatement(::std::boxed::Box::new(
                <DiagnosticStatement as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "do_statement" => Ok(Self::DoStatement(::std::boxed::Box::new(
                <DoStatement as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "enum_case_pattern" => Ok(Self::EnumCasePattern(::std::boxed::Box::new(
                <EnumCasePattern as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "enum_declaration" => Ok(Self::EnumDeclaration(::std::boxed::Box::new(
                <EnumDeclaration as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "extension_declaration" => Ok(Self::ExtensionDeclaration(::std::boxed::Box::new(
                <ExtensionDeclaration as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "fallthrough_statement" => Ok(Self::FallthroughStatement(::std::boxed::Box::new(
                <FallthroughStatement as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "for_statement" => Ok(Self::ForStatement(::std::boxed::Box::new(
                <ForStatement as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "function_declaration" => Ok(Self::FunctionDeclaration(::std::boxed::Box::new(
                <FunctionDeclaration as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "guard_statement" => Ok(Self::GuardStatement(::std::boxed::Box::new(
                <GuardStatement as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "identifier" => Ok(Self::Identifier(::std::boxed::Box::new(
                <Identifier as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "if_statement" => Ok(Self::IfStatement(::std::boxed::Box::new(
                <IfStatement as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "import_declaration" => Ok(Self::ImportDeclaration(::std::boxed::Box::new(
                <ImportDeclaration as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "initializer_declaration" => Ok(Self::InitializerDeclaration(::std::boxed::Box::new(
                <InitializerDeclaration as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "is_pattern" => Ok(Self::IsPattern(::std::boxed::Box::new(
                <IsPattern as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "labeled_statement" => Ok(Self::LabeledStatement(::std::boxed::Box::new(
                <LabeledStatement as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "line_control_statement" => Ok(Self::LineControlStatement(::std::boxed::Box::new(
                <LineControlStatement as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "nil" => Ok(Self::Nil(::std::boxed::Box::new(
                <Nil as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "number" => Ok(Self::Number(::std::boxed::Box::new(
                <Number as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "operator_declaration" => Ok(Self::OperatorDeclaration(::std::boxed::Box::new(
                <OperatorDeclaration as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "optional_pattern" => Ok(Self::OptionalPattern(::std::boxed::Box::new(
                <OptionalPattern as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "protocol_declaration" => Ok(Self::ProtocolDeclaration(::std::boxed::Box::new(
                <ProtocolDeclaration as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "repeat_while_statement" => Ok(Self::RepeatWhileStatement(::std::boxed::Box::new(
                <RepeatWhileStatement as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "return_statement" => Ok(Self::ReturnStatement(::std::boxed::Box::new(
                <ReturnStatement as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "string" => Ok(Self::String(::std::boxed::Box::new(
                <String as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "struct_declaration" => Ok(Self::StructDeclaration(::std::boxed::Box::new(
                <StructDeclaration as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "subscript_declaration" => Ok(Self::SubscriptDeclaration(::std::boxed::Box::new(
                <SubscriptDeclaration as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "switch_statement" => Ok(Self::SwitchStatement(::std::boxed::Box::new(
                <SwitchStatement as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "throw_statement" => Ok(Self::ThrowStatement(::std::boxed::Box::new(
                <ThrowStatement as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "typealias_declaration" => Ok(Self::TypealiasDeclaration(::std::boxed::Box::new(
                <TypealiasDeclaration as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "value_binding_pattern" => Ok(Self::ValueBindingPattern(::std::boxed::Box::new(
                <ValueBindingPattern as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "variable_declaration" => Ok(Self::VariableDeclaration(::std::boxed::Box::new(
                <VariableDeclaration as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "while_statement" => Ok(Self::WhileStatement(::std::boxed::Box::new(
                <WhileStatement as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "wildcard_pattern" => Ok(Self::WildcardPattern(::std::boxed::Box::new(
                <WildcardPattern as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            other => Err(::treesitter_types::ParseError::unexpected_kind(other, node)),
        }
    }
}
impl ::treesitter_types::Spanned for CaseStatementChildren<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        match self {
            Self::AsPattern(inner) => inner.span(),
            Self::Boolean(inner) => inner.span(),
            Self::BreakStatement(inner) => inner.span(),
            Self::BuildConfigurationStatement(inner) => inner.span(),
            Self::ClassDeclaration(inner) => inner.span(),
            Self::ConstantDeclaration(inner) => inner.span(),
            Self::ContinueStatement(inner) => inner.span(),
            Self::DeferStatement(inner) => inner.span(),
            Self::DeinitializerDeclaration(inner) => inner.span(),
            Self::DiagnosticStatement(inner) => inner.span(),
            Self::DoStatement(inner) => inner.span(),
            Self::EnumCasePattern(inner) => inner.span(),
            Self::EnumDeclaration(inner) => inner.span(),
            Self::ExtensionDeclaration(inner) => inner.span(),
            Self::FallthroughStatement(inner) => inner.span(),
            Self::ForStatement(inner) => inner.span(),
            Self::FunctionDeclaration(inner) => inner.span(),
            Self::GuardStatement(inner) => inner.span(),
            Self::Identifier(inner) => inner.span(),
            Self::IfStatement(inner) => inner.span(),
            Self::ImportDeclaration(inner) => inner.span(),
            Self::InitializerDeclaration(inner) => inner.span(),
            Self::IsPattern(inner) => inner.span(),
            Self::LabeledStatement(inner) => inner.span(),
            Self::LineControlStatement(inner) => inner.span(),
            Self::Nil(inner) => inner.span(),
            Self::Number(inner) => inner.span(),
            Self::OperatorDeclaration(inner) => inner.span(),
            Self::OptionalPattern(inner) => inner.span(),
            Self::ProtocolDeclaration(inner) => inner.span(),
            Self::RepeatWhileStatement(inner) => inner.span(),
            Self::ReturnStatement(inner) => inner.span(),
            Self::String(inner) => inner.span(),
            Self::StructDeclaration(inner) => inner.span(),
            Self::SubscriptDeclaration(inner) => inner.span(),
            Self::SwitchStatement(inner) => inner.span(),
            Self::ThrowStatement(inner) => inner.span(),
            Self::TypealiasDeclaration(inner) => inner.span(),
            Self::ValueBindingPattern(inner) => inner.span(),
            Self::VariableDeclaration(inner) => inner.span(),
            Self::WhileStatement(inner) => inner.span(),
            Self::WildcardPattern(inner) => inner.span(),
        }
    }
}
#[derive(Debug, Clone)]
pub enum CatchClauseChildren<'tree> {
    AsPattern(::std::boxed::Box<AsPattern<'tree>>),
    Boolean(::std::boxed::Box<Boolean<'tree>>),
    BreakStatement(::std::boxed::Box<BreakStatement<'tree>>),
    BuildConfigurationStatement(::std::boxed::Box<BuildConfigurationStatement<'tree>>),
    ClassDeclaration(::std::boxed::Box<ClassDeclaration<'tree>>),
    ConstantDeclaration(::std::boxed::Box<ConstantDeclaration<'tree>>),
    ContinueStatement(::std::boxed::Box<ContinueStatement<'tree>>),
    DeferStatement(::std::boxed::Box<DeferStatement<'tree>>),
    DeinitializerDeclaration(::std::boxed::Box<DeinitializerDeclaration<'tree>>),
    DiagnosticStatement(::std::boxed::Box<DiagnosticStatement<'tree>>),
    DoStatement(::std::boxed::Box<DoStatement<'tree>>),
    EnumCasePattern(::std::boxed::Box<EnumCasePattern<'tree>>),
    EnumDeclaration(::std::boxed::Box<EnumDeclaration<'tree>>),
    ExtensionDeclaration(::std::boxed::Box<ExtensionDeclaration<'tree>>),
    FallthroughStatement(::std::boxed::Box<FallthroughStatement<'tree>>),
    ForStatement(::std::boxed::Box<ForStatement<'tree>>),
    FunctionDeclaration(::std::boxed::Box<FunctionDeclaration<'tree>>),
    GuardStatement(::std::boxed::Box<GuardStatement<'tree>>),
    Identifier(::std::boxed::Box<Identifier<'tree>>),
    IfStatement(::std::boxed::Box<IfStatement<'tree>>),
    ImportDeclaration(::std::boxed::Box<ImportDeclaration<'tree>>),
    InitializerDeclaration(::std::boxed::Box<InitializerDeclaration<'tree>>),
    IsPattern(::std::boxed::Box<IsPattern<'tree>>),
    LabeledStatement(::std::boxed::Box<LabeledStatement<'tree>>),
    LineControlStatement(::std::boxed::Box<LineControlStatement<'tree>>),
    Nil(::std::boxed::Box<Nil<'tree>>),
    Number(::std::boxed::Box<Number<'tree>>),
    OperatorDeclaration(::std::boxed::Box<OperatorDeclaration<'tree>>),
    OptionalPattern(::std::boxed::Box<OptionalPattern<'tree>>),
    ProtocolDeclaration(::std::boxed::Box<ProtocolDeclaration<'tree>>),
    RepeatWhileStatement(::std::boxed::Box<RepeatWhileStatement<'tree>>),
    ReturnStatement(::std::boxed::Box<ReturnStatement<'tree>>),
    String(::std::boxed::Box<String<'tree>>),
    StructDeclaration(::std::boxed::Box<StructDeclaration<'tree>>),
    SubscriptDeclaration(::std::boxed::Box<SubscriptDeclaration<'tree>>),
    SwitchStatement(::std::boxed::Box<SwitchStatement<'tree>>),
    ThrowStatement(::std::boxed::Box<ThrowStatement<'tree>>),
    TypealiasDeclaration(::std::boxed::Box<TypealiasDeclaration<'tree>>),
    ValueBindingPattern(::std::boxed::Box<ValueBindingPattern<'tree>>),
    VariableDeclaration(::std::boxed::Box<VariableDeclaration<'tree>>),
    WhileStatement(::std::boxed::Box<WhileStatement<'tree>>),
    WildcardPattern(::std::boxed::Box<WildcardPattern<'tree>>),
}
impl<'tree> ::treesitter_types::FromNode<'tree> for CatchClauseChildren<'tree> {
    #[allow(clippy::collapsible_else_if)]
    fn from_node(
        node: ::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        match node.kind() {
            "as_pattern" => Ok(Self::AsPattern(::std::boxed::Box::new(
                <AsPattern as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "boolean" => Ok(Self::Boolean(::std::boxed::Box::new(
                <Boolean as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "break_statement" => Ok(Self::BreakStatement(::std::boxed::Box::new(
                <BreakStatement as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "build_configuration_statement" => {
                Ok(Self::BuildConfigurationStatement(::std::boxed::Box::new(
                    <BuildConfigurationStatement as ::treesitter_types::FromNode>::from_node(
                        node, src,
                    )?,
                )))
            }
            "class_declaration" => Ok(Self::ClassDeclaration(::std::boxed::Box::new(
                <ClassDeclaration as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "constant_declaration" => Ok(Self::ConstantDeclaration(::std::boxed::Box::new(
                <ConstantDeclaration as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "continue_statement" => Ok(Self::ContinueStatement(::std::boxed::Box::new(
                <ContinueStatement as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "defer_statement" => Ok(Self::DeferStatement(::std::boxed::Box::new(
                <DeferStatement as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "deinitializer_declaration" => {
                Ok(Self::DeinitializerDeclaration(::std::boxed::Box::new(
                    <DeinitializerDeclaration as ::treesitter_types::FromNode>::from_node(
                        node, src,
                    )?,
                )))
            }
            "diagnostic_statement" => Ok(Self::DiagnosticStatement(::std::boxed::Box::new(
                <DiagnosticStatement as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "do_statement" => Ok(Self::DoStatement(::std::boxed::Box::new(
                <DoStatement as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "enum_case_pattern" => Ok(Self::EnumCasePattern(::std::boxed::Box::new(
                <EnumCasePattern as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "enum_declaration" => Ok(Self::EnumDeclaration(::std::boxed::Box::new(
                <EnumDeclaration as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "extension_declaration" => Ok(Self::ExtensionDeclaration(::std::boxed::Box::new(
                <ExtensionDeclaration as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "fallthrough_statement" => Ok(Self::FallthroughStatement(::std::boxed::Box::new(
                <FallthroughStatement as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "for_statement" => Ok(Self::ForStatement(::std::boxed::Box::new(
                <ForStatement as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "function_declaration" => Ok(Self::FunctionDeclaration(::std::boxed::Box::new(
                <FunctionDeclaration as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "guard_statement" => Ok(Self::GuardStatement(::std::boxed::Box::new(
                <GuardStatement as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "identifier" => Ok(Self::Identifier(::std::boxed::Box::new(
                <Identifier as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "if_statement" => Ok(Self::IfStatement(::std::boxed::Box::new(
                <IfStatement as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "import_declaration" => Ok(Self::ImportDeclaration(::std::boxed::Box::new(
                <ImportDeclaration as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "initializer_declaration" => Ok(Self::InitializerDeclaration(::std::boxed::Box::new(
                <InitializerDeclaration as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "is_pattern" => Ok(Self::IsPattern(::std::boxed::Box::new(
                <IsPattern as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "labeled_statement" => Ok(Self::LabeledStatement(::std::boxed::Box::new(
                <LabeledStatement as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "line_control_statement" => Ok(Self::LineControlStatement(::std::boxed::Box::new(
                <LineControlStatement as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "nil" => Ok(Self::Nil(::std::boxed::Box::new(
                <Nil as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "number" => Ok(Self::Number(::std::boxed::Box::new(
                <Number as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "operator_declaration" => Ok(Self::OperatorDeclaration(::std::boxed::Box::new(
                <OperatorDeclaration as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "optional_pattern" => Ok(Self::OptionalPattern(::std::boxed::Box::new(
                <OptionalPattern as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "protocol_declaration" => Ok(Self::ProtocolDeclaration(::std::boxed::Box::new(
                <ProtocolDeclaration as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "repeat_while_statement" => Ok(Self::RepeatWhileStatement(::std::boxed::Box::new(
                <RepeatWhileStatement as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "return_statement" => Ok(Self::ReturnStatement(::std::boxed::Box::new(
                <ReturnStatement as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "string" => Ok(Self::String(::std::boxed::Box::new(
                <String as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "struct_declaration" => Ok(Self::StructDeclaration(::std::boxed::Box::new(
                <StructDeclaration as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "subscript_declaration" => Ok(Self::SubscriptDeclaration(::std::boxed::Box::new(
                <SubscriptDeclaration as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "switch_statement" => Ok(Self::SwitchStatement(::std::boxed::Box::new(
                <SwitchStatement as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "throw_statement" => Ok(Self::ThrowStatement(::std::boxed::Box::new(
                <ThrowStatement as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "typealias_declaration" => Ok(Self::TypealiasDeclaration(::std::boxed::Box::new(
                <TypealiasDeclaration as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "value_binding_pattern" => Ok(Self::ValueBindingPattern(::std::boxed::Box::new(
                <ValueBindingPattern as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "variable_declaration" => Ok(Self::VariableDeclaration(::std::boxed::Box::new(
                <VariableDeclaration as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "while_statement" => Ok(Self::WhileStatement(::std::boxed::Box::new(
                <WhileStatement as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "wildcard_pattern" => Ok(Self::WildcardPattern(::std::boxed::Box::new(
                <WildcardPattern as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            other => Err(::treesitter_types::ParseError::unexpected_kind(other, node)),
        }
    }
}
impl ::treesitter_types::Spanned for CatchClauseChildren<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        match self {
            Self::AsPattern(inner) => inner.span(),
            Self::Boolean(inner) => inner.span(),
            Self::BreakStatement(inner) => inner.span(),
            Self::BuildConfigurationStatement(inner) => inner.span(),
            Self::ClassDeclaration(inner) => inner.span(),
            Self::ConstantDeclaration(inner) => inner.span(),
            Self::ContinueStatement(inner) => inner.span(),
            Self::DeferStatement(inner) => inner.span(),
            Self::DeinitializerDeclaration(inner) => inner.span(),
            Self::DiagnosticStatement(inner) => inner.span(),
            Self::DoStatement(inner) => inner.span(),
            Self::EnumCasePattern(inner) => inner.span(),
            Self::EnumDeclaration(inner) => inner.span(),
            Self::ExtensionDeclaration(inner) => inner.span(),
            Self::FallthroughStatement(inner) => inner.span(),
            Self::ForStatement(inner) => inner.span(),
            Self::FunctionDeclaration(inner) => inner.span(),
            Self::GuardStatement(inner) => inner.span(),
            Self::Identifier(inner) => inner.span(),
            Self::IfStatement(inner) => inner.span(),
            Self::ImportDeclaration(inner) => inner.span(),
            Self::InitializerDeclaration(inner) => inner.span(),
            Self::IsPattern(inner) => inner.span(),
            Self::LabeledStatement(inner) => inner.span(),
            Self::LineControlStatement(inner) => inner.span(),
            Self::Nil(inner) => inner.span(),
            Self::Number(inner) => inner.span(),
            Self::OperatorDeclaration(inner) => inner.span(),
            Self::OptionalPattern(inner) => inner.span(),
            Self::ProtocolDeclaration(inner) => inner.span(),
            Self::RepeatWhileStatement(inner) => inner.span(),
            Self::ReturnStatement(inner) => inner.span(),
            Self::String(inner) => inner.span(),
            Self::StructDeclaration(inner) => inner.span(),
            Self::SubscriptDeclaration(inner) => inner.span(),
            Self::SwitchStatement(inner) => inner.span(),
            Self::ThrowStatement(inner) => inner.span(),
            Self::TypealiasDeclaration(inner) => inner.span(),
            Self::ValueBindingPattern(inner) => inner.span(),
            Self::VariableDeclaration(inner) => inner.span(),
            Self::WhileStatement(inner) => inner.span(),
            Self::WildcardPattern(inner) => inner.span(),
        }
    }
}
#[derive(Debug, Clone)]
pub enum ClassDeclarationChildren<'tree> {
    ClassDeclaration(::std::boxed::Box<ClassDeclaration<'tree>>),
    ConstantDeclaration(::std::boxed::Box<ConstantDeclaration<'tree>>),
    DeinitializerDeclaration(::std::boxed::Box<DeinitializerDeclaration<'tree>>),
    EnumDeclaration(::std::boxed::Box<EnumDeclaration<'tree>>),
    ExtensionDeclaration(::std::boxed::Box<ExtensionDeclaration<'tree>>),
    FunctionDeclaration(::std::boxed::Box<FunctionDeclaration<'tree>>),
    Identifier(::std::boxed::Box<Identifier<'tree>>),
    ImportDeclaration(::std::boxed::Box<ImportDeclaration<'tree>>),
    InitializerDeclaration(::std::boxed::Box<InitializerDeclaration<'tree>>),
    Modifier(::std::boxed::Box<Modifier<'tree>>),
    OperatorDeclaration(::std::boxed::Box<OperatorDeclaration<'tree>>),
    ProtocolDeclaration(::std::boxed::Box<ProtocolDeclaration<'tree>>),
    StructDeclaration(::std::boxed::Box<StructDeclaration<'tree>>),
    SubscriptDeclaration(::std::boxed::Box<SubscriptDeclaration<'tree>>),
    TypealiasDeclaration(::std::boxed::Box<TypealiasDeclaration<'tree>>),
    VariableDeclaration(::std::boxed::Box<VariableDeclaration<'tree>>),
}
impl<'tree> ::treesitter_types::FromNode<'tree> for ClassDeclarationChildren<'tree> {
    #[allow(clippy::collapsible_else_if)]
    fn from_node(
        node: ::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        match node.kind() {
            "class_declaration" => Ok(Self::ClassDeclaration(::std::boxed::Box::new(
                <ClassDeclaration as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "constant_declaration" => Ok(Self::ConstantDeclaration(::std::boxed::Box::new(
                <ConstantDeclaration as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "deinitializer_declaration" => {
                Ok(Self::DeinitializerDeclaration(::std::boxed::Box::new(
                    <DeinitializerDeclaration as ::treesitter_types::FromNode>::from_node(
                        node, src,
                    )?,
                )))
            }
            "enum_declaration" => Ok(Self::EnumDeclaration(::std::boxed::Box::new(
                <EnumDeclaration as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "extension_declaration" => Ok(Self::ExtensionDeclaration(::std::boxed::Box::new(
                <ExtensionDeclaration as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "function_declaration" => Ok(Self::FunctionDeclaration(::std::boxed::Box::new(
                <FunctionDeclaration as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "identifier" => Ok(Self::Identifier(::std::boxed::Box::new(
                <Identifier as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "import_declaration" => Ok(Self::ImportDeclaration(::std::boxed::Box::new(
                <ImportDeclaration as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "initializer_declaration" => Ok(Self::InitializerDeclaration(::std::boxed::Box::new(
                <InitializerDeclaration as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "modifier" => Ok(Self::Modifier(::std::boxed::Box::new(
                <Modifier as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "operator_declaration" => Ok(Self::OperatorDeclaration(::std::boxed::Box::new(
                <OperatorDeclaration as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "protocol_declaration" => Ok(Self::ProtocolDeclaration(::std::boxed::Box::new(
                <ProtocolDeclaration as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "struct_declaration" => Ok(Self::StructDeclaration(::std::boxed::Box::new(
                <StructDeclaration as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "subscript_declaration" => Ok(Self::SubscriptDeclaration(::std::boxed::Box::new(
                <SubscriptDeclaration as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "typealias_declaration" => Ok(Self::TypealiasDeclaration(::std::boxed::Box::new(
                <TypealiasDeclaration as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "variable_declaration" => Ok(Self::VariableDeclaration(::std::boxed::Box::new(
                <VariableDeclaration as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            other => Err(::treesitter_types::ParseError::unexpected_kind(other, node)),
        }
    }
}
impl ::treesitter_types::Spanned for ClassDeclarationChildren<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        match self {
            Self::ClassDeclaration(inner) => inner.span(),
            Self::ConstantDeclaration(inner) => inner.span(),
            Self::DeinitializerDeclaration(inner) => inner.span(),
            Self::EnumDeclaration(inner) => inner.span(),
            Self::ExtensionDeclaration(inner) => inner.span(),
            Self::FunctionDeclaration(inner) => inner.span(),
            Self::Identifier(inner) => inner.span(),
            Self::ImportDeclaration(inner) => inner.span(),
            Self::InitializerDeclaration(inner) => inner.span(),
            Self::Modifier(inner) => inner.span(),
            Self::OperatorDeclaration(inner) => inner.span(),
            Self::ProtocolDeclaration(inner) => inner.span(),
            Self::StructDeclaration(inner) => inner.span(),
            Self::SubscriptDeclaration(inner) => inner.span(),
            Self::TypealiasDeclaration(inner) => inner.span(),
            Self::VariableDeclaration(inner) => inner.span(),
        }
    }
}
#[derive(Debug, Clone)]
pub enum ConstantDeclarationName<'tree> {
    LParen(::treesitter_types::Span),
    RParen(::treesitter_types::Span),
    Comma(::treesitter_types::Span),
    Colon(::treesitter_types::Span),
    LBracket(::treesitter_types::Span),
    RBracket(::treesitter_types::Span),
    AsPattern(::std::boxed::Box<AsPattern<'tree>>),
    Boolean(::std::boxed::Box<Boolean<'tree>>),
    EnumCasePattern(::std::boxed::Box<EnumCasePattern<'tree>>),
    Identifier(::std::boxed::Box<Identifier<'tree>>),
    IsPattern(::std::boxed::Box<IsPattern<'tree>>),
    Nil(::std::boxed::Box<Nil<'tree>>),
    Number(::std::boxed::Box<Number<'tree>>),
    OptionalPattern(::std::boxed::Box<OptionalPattern<'tree>>),
    String(::std::boxed::Box<String<'tree>>),
    ValueBindingPattern(::std::boxed::Box<ValueBindingPattern<'tree>>),
    WildcardPattern(::std::boxed::Box<WildcardPattern<'tree>>),
}
impl<'tree> ::treesitter_types::FromNode<'tree> for ConstantDeclarationName<'tree> {
    #[allow(clippy::collapsible_else_if)]
    fn from_node(
        node: ::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        match node.kind() {
            "(" => Ok(Self::LParen(::treesitter_types::Span::from(node))),
            ")" => Ok(Self::RParen(::treesitter_types::Span::from(node))),
            "," => Ok(Self::Comma(::treesitter_types::Span::from(node))),
            ":" => Ok(Self::Colon(::treesitter_types::Span::from(node))),
            "[" => Ok(Self::LBracket(::treesitter_types::Span::from(node))),
            "]" => Ok(Self::RBracket(::treesitter_types::Span::from(node))),
            "as_pattern" => Ok(Self::AsPattern(::std::boxed::Box::new(
                <AsPattern as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "boolean" => Ok(Self::Boolean(::std::boxed::Box::new(
                <Boolean as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "enum_case_pattern" => Ok(Self::EnumCasePattern(::std::boxed::Box::new(
                <EnumCasePattern as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "identifier" => Ok(Self::Identifier(::std::boxed::Box::new(
                <Identifier as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "is_pattern" => Ok(Self::IsPattern(::std::boxed::Box::new(
                <IsPattern as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "nil" => Ok(Self::Nil(::std::boxed::Box::new(
                <Nil as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "number" => Ok(Self::Number(::std::boxed::Box::new(
                <Number as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "optional_pattern" => Ok(Self::OptionalPattern(::std::boxed::Box::new(
                <OptionalPattern as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "string" => Ok(Self::String(::std::boxed::Box::new(
                <String as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "value_binding_pattern" => Ok(Self::ValueBindingPattern(::std::boxed::Box::new(
                <ValueBindingPattern as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "wildcard_pattern" => Ok(Self::WildcardPattern(::std::boxed::Box::new(
                <WildcardPattern as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            other => Err(::treesitter_types::ParseError::unexpected_kind(other, node)),
        }
    }
}
impl ::treesitter_types::Spanned for ConstantDeclarationName<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        match self {
            Self::LParen(span) => *span,
            Self::RParen(span) => *span,
            Self::Comma(span) => *span,
            Self::Colon(span) => *span,
            Self::LBracket(span) => *span,
            Self::RBracket(span) => *span,
            Self::AsPattern(inner) => inner.span(),
            Self::Boolean(inner) => inner.span(),
            Self::EnumCasePattern(inner) => inner.span(),
            Self::Identifier(inner) => inner.span(),
            Self::IsPattern(inner) => inner.span(),
            Self::Nil(inner) => inner.span(),
            Self::Number(inner) => inner.span(),
            Self::OptionalPattern(inner) => inner.span(),
            Self::String(inner) => inner.span(),
            Self::ValueBindingPattern(inner) => inner.span(),
            Self::WildcardPattern(inner) => inner.span(),
        }
    }
}
#[derive(Debug, Clone)]
pub enum ConstantDeclarationType<'tree> {
    Colon(::treesitter_types::Span),
    ArrayType(::std::boxed::Box<ArrayType<'tree>>),
    DictionaryType(::std::boxed::Box<DictionaryType<'tree>>),
    Type(::std::boxed::Box<Type<'tree>>),
}
impl<'tree> ::treesitter_types::FromNode<'tree> for ConstantDeclarationType<'tree> {
    #[allow(clippy::collapsible_else_if)]
    fn from_node(
        node: ::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        match node.kind() {
            ":" => Ok(Self::Colon(::treesitter_types::Span::from(node))),
            "array_type" => Ok(Self::ArrayType(::std::boxed::Box::new(
                <ArrayType as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "dictionary_type" => Ok(Self::DictionaryType(::std::boxed::Box::new(
                <DictionaryType as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "type" => Ok(Self::Type(::std::boxed::Box::new(
                <Type as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            other => Err(::treesitter_types::ParseError::unexpected_kind(other, node)),
        }
    }
}
impl ::treesitter_types::Spanned for ConstantDeclarationType<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        match self {
            Self::Colon(span) => *span,
            Self::ArrayType(inner) => inner.span(),
            Self::DictionaryType(inner) => inner.span(),
            Self::Type(inner) => inner.span(),
        }
    }
}
#[derive(Debug, Clone)]
pub enum ConstantDeclarationValue<'tree> {
    LParen(::treesitter_types::Span),
    RParen(::treesitter_types::Span),
    Comma(::treesitter_types::Span),
    Colon(::treesitter_types::Span),
    LBracket(::treesitter_types::Span),
    RBracket(::treesitter_types::Span),
    Boolean(::std::boxed::Box<Boolean<'tree>>),
    Identifier(::std::boxed::Box<Identifier<'tree>>),
    Nil(::std::boxed::Box<Nil<'tree>>),
    Number(::std::boxed::Box<Number<'tree>>),
    String(::std::boxed::Box<String<'tree>>),
}
impl<'tree> ::treesitter_types::FromNode<'tree> for ConstantDeclarationValue<'tree> {
    #[allow(clippy::collapsible_else_if)]
    fn from_node(
        node: ::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        match node.kind() {
            "(" => Ok(Self::LParen(::treesitter_types::Span::from(node))),
            ")" => Ok(Self::RParen(::treesitter_types::Span::from(node))),
            "," => Ok(Self::Comma(::treesitter_types::Span::from(node))),
            ":" => Ok(Self::Colon(::treesitter_types::Span::from(node))),
            "[" => Ok(Self::LBracket(::treesitter_types::Span::from(node))),
            "]" => Ok(Self::RBracket(::treesitter_types::Span::from(node))),
            "boolean" => Ok(Self::Boolean(::std::boxed::Box::new(
                <Boolean as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "identifier" => Ok(Self::Identifier(::std::boxed::Box::new(
                <Identifier as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "nil" => Ok(Self::Nil(::std::boxed::Box::new(
                <Nil as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "number" => Ok(Self::Number(::std::boxed::Box::new(
                <Number as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "string" => Ok(Self::String(::std::boxed::Box::new(
                <String as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            other => Err(::treesitter_types::ParseError::unexpected_kind(other, node)),
        }
    }
}
impl ::treesitter_types::Spanned for ConstantDeclarationValue<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        match self {
            Self::LParen(span) => *span,
            Self::RParen(span) => *span,
            Self::Comma(span) => *span,
            Self::Colon(span) => *span,
            Self::LBracket(span) => *span,
            Self::RBracket(span) => *span,
            Self::Boolean(inner) => inner.span(),
            Self::Identifier(inner) => inner.span(),
            Self::Nil(inner) => inner.span(),
            Self::Number(inner) => inner.span(),
            Self::String(inner) => inner.span(),
        }
    }
}
#[derive(Debug, Clone)]
pub enum DeferStatementChildren<'tree> {
    Boolean(::std::boxed::Box<Boolean<'tree>>),
    BreakStatement(::std::boxed::Box<BreakStatement<'tree>>),
    BuildConfigurationStatement(::std::boxed::Box<BuildConfigurationStatement<'tree>>),
    ClassDeclaration(::std::boxed::Box<ClassDeclaration<'tree>>),
    ConstantDeclaration(::std::boxed::Box<ConstantDeclaration<'tree>>),
    ContinueStatement(::std::boxed::Box<ContinueStatement<'tree>>),
    DeferStatement(::std::boxed::Box<DeferStatement<'tree>>),
    DeinitializerDeclaration(::std::boxed::Box<DeinitializerDeclaration<'tree>>),
    DiagnosticStatement(::std::boxed::Box<DiagnosticStatement<'tree>>),
    DoStatement(::std::boxed::Box<DoStatement<'tree>>),
    EnumDeclaration(::std::boxed::Box<EnumDeclaration<'tree>>),
    ExtensionDeclaration(::std::boxed::Box<ExtensionDeclaration<'tree>>),
    FallthroughStatement(::std::boxed::Box<FallthroughStatement<'tree>>),
    ForStatement(::std::boxed::Box<ForStatement<'tree>>),
    FunctionDeclaration(::std::boxed::Box<FunctionDeclaration<'tree>>),
    GuardStatement(::std::boxed::Box<GuardStatement<'tree>>),
    Identifier(::std::boxed::Box<Identifier<'tree>>),
    IfStatement(::std::boxed::Box<IfStatement<'tree>>),
    ImportDeclaration(::std::boxed::Box<ImportDeclaration<'tree>>),
    InitializerDeclaration(::std::boxed::Box<InitializerDeclaration<'tree>>),
    LabeledStatement(::std::boxed::Box<LabeledStatement<'tree>>),
    LineControlStatement(::std::boxed::Box<LineControlStatement<'tree>>),
    Nil(::std::boxed::Box<Nil<'tree>>),
    Number(::std::boxed::Box<Number<'tree>>),
    OperatorDeclaration(::std::boxed::Box<OperatorDeclaration<'tree>>),
    ProtocolDeclaration(::std::boxed::Box<ProtocolDeclaration<'tree>>),
    RepeatWhileStatement(::std::boxed::Box<RepeatWhileStatement<'tree>>),
    ReturnStatement(::std::boxed::Box<ReturnStatement<'tree>>),
    String(::std::boxed::Box<String<'tree>>),
    StructDeclaration(::std::boxed::Box<StructDeclaration<'tree>>),
    SubscriptDeclaration(::std::boxed::Box<SubscriptDeclaration<'tree>>),
    SwitchStatement(::std::boxed::Box<SwitchStatement<'tree>>),
    ThrowStatement(::std::boxed::Box<ThrowStatement<'tree>>),
    TypealiasDeclaration(::std::boxed::Box<TypealiasDeclaration<'tree>>),
    VariableDeclaration(::std::boxed::Box<VariableDeclaration<'tree>>),
    WhileStatement(::std::boxed::Box<WhileStatement<'tree>>),
}
impl<'tree> ::treesitter_types::FromNode<'tree> for DeferStatementChildren<'tree> {
    #[allow(clippy::collapsible_else_if)]
    fn from_node(
        node: ::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        match node.kind() {
            "boolean" => Ok(Self::Boolean(::std::boxed::Box::new(
                <Boolean as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "break_statement" => Ok(Self::BreakStatement(::std::boxed::Box::new(
                <BreakStatement as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "build_configuration_statement" => {
                Ok(Self::BuildConfigurationStatement(::std::boxed::Box::new(
                    <BuildConfigurationStatement as ::treesitter_types::FromNode>::from_node(
                        node, src,
                    )?,
                )))
            }
            "class_declaration" => Ok(Self::ClassDeclaration(::std::boxed::Box::new(
                <ClassDeclaration as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "constant_declaration" => Ok(Self::ConstantDeclaration(::std::boxed::Box::new(
                <ConstantDeclaration as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "continue_statement" => Ok(Self::ContinueStatement(::std::boxed::Box::new(
                <ContinueStatement as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "defer_statement" => Ok(Self::DeferStatement(::std::boxed::Box::new(
                <DeferStatement as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "deinitializer_declaration" => {
                Ok(Self::DeinitializerDeclaration(::std::boxed::Box::new(
                    <DeinitializerDeclaration as ::treesitter_types::FromNode>::from_node(
                        node, src,
                    )?,
                )))
            }
            "diagnostic_statement" => Ok(Self::DiagnosticStatement(::std::boxed::Box::new(
                <DiagnosticStatement as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "do_statement" => Ok(Self::DoStatement(::std::boxed::Box::new(
                <DoStatement as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "enum_declaration" => Ok(Self::EnumDeclaration(::std::boxed::Box::new(
                <EnumDeclaration as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "extension_declaration" => Ok(Self::ExtensionDeclaration(::std::boxed::Box::new(
                <ExtensionDeclaration as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "fallthrough_statement" => Ok(Self::FallthroughStatement(::std::boxed::Box::new(
                <FallthroughStatement as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "for_statement" => Ok(Self::ForStatement(::std::boxed::Box::new(
                <ForStatement as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "function_declaration" => Ok(Self::FunctionDeclaration(::std::boxed::Box::new(
                <FunctionDeclaration as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "guard_statement" => Ok(Self::GuardStatement(::std::boxed::Box::new(
                <GuardStatement as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "identifier" => Ok(Self::Identifier(::std::boxed::Box::new(
                <Identifier as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "if_statement" => Ok(Self::IfStatement(::std::boxed::Box::new(
                <IfStatement as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "import_declaration" => Ok(Self::ImportDeclaration(::std::boxed::Box::new(
                <ImportDeclaration as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "initializer_declaration" => Ok(Self::InitializerDeclaration(::std::boxed::Box::new(
                <InitializerDeclaration as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "labeled_statement" => Ok(Self::LabeledStatement(::std::boxed::Box::new(
                <LabeledStatement as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "line_control_statement" => Ok(Self::LineControlStatement(::std::boxed::Box::new(
                <LineControlStatement as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "nil" => Ok(Self::Nil(::std::boxed::Box::new(
                <Nil as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "number" => Ok(Self::Number(::std::boxed::Box::new(
                <Number as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "operator_declaration" => Ok(Self::OperatorDeclaration(::std::boxed::Box::new(
                <OperatorDeclaration as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "protocol_declaration" => Ok(Self::ProtocolDeclaration(::std::boxed::Box::new(
                <ProtocolDeclaration as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "repeat_while_statement" => Ok(Self::RepeatWhileStatement(::std::boxed::Box::new(
                <RepeatWhileStatement as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "return_statement" => Ok(Self::ReturnStatement(::std::boxed::Box::new(
                <ReturnStatement as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "string" => Ok(Self::String(::std::boxed::Box::new(
                <String as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "struct_declaration" => Ok(Self::StructDeclaration(::std::boxed::Box::new(
                <StructDeclaration as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "subscript_declaration" => Ok(Self::SubscriptDeclaration(::std::boxed::Box::new(
                <SubscriptDeclaration as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "switch_statement" => Ok(Self::SwitchStatement(::std::boxed::Box::new(
                <SwitchStatement as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "throw_statement" => Ok(Self::ThrowStatement(::std::boxed::Box::new(
                <ThrowStatement as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "typealias_declaration" => Ok(Self::TypealiasDeclaration(::std::boxed::Box::new(
                <TypealiasDeclaration as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "variable_declaration" => Ok(Self::VariableDeclaration(::std::boxed::Box::new(
                <VariableDeclaration as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "while_statement" => Ok(Self::WhileStatement(::std::boxed::Box::new(
                <WhileStatement as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            other => Err(::treesitter_types::ParseError::unexpected_kind(other, node)),
        }
    }
}
impl ::treesitter_types::Spanned for DeferStatementChildren<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        match self {
            Self::Boolean(inner) => inner.span(),
            Self::BreakStatement(inner) => inner.span(),
            Self::BuildConfigurationStatement(inner) => inner.span(),
            Self::ClassDeclaration(inner) => inner.span(),
            Self::ConstantDeclaration(inner) => inner.span(),
            Self::ContinueStatement(inner) => inner.span(),
            Self::DeferStatement(inner) => inner.span(),
            Self::DeinitializerDeclaration(inner) => inner.span(),
            Self::DiagnosticStatement(inner) => inner.span(),
            Self::DoStatement(inner) => inner.span(),
            Self::EnumDeclaration(inner) => inner.span(),
            Self::ExtensionDeclaration(inner) => inner.span(),
            Self::FallthroughStatement(inner) => inner.span(),
            Self::ForStatement(inner) => inner.span(),
            Self::FunctionDeclaration(inner) => inner.span(),
            Self::GuardStatement(inner) => inner.span(),
            Self::Identifier(inner) => inner.span(),
            Self::IfStatement(inner) => inner.span(),
            Self::ImportDeclaration(inner) => inner.span(),
            Self::InitializerDeclaration(inner) => inner.span(),
            Self::LabeledStatement(inner) => inner.span(),
            Self::LineControlStatement(inner) => inner.span(),
            Self::Nil(inner) => inner.span(),
            Self::Number(inner) => inner.span(),
            Self::OperatorDeclaration(inner) => inner.span(),
            Self::ProtocolDeclaration(inner) => inner.span(),
            Self::RepeatWhileStatement(inner) => inner.span(),
            Self::ReturnStatement(inner) => inner.span(),
            Self::String(inner) => inner.span(),
            Self::StructDeclaration(inner) => inner.span(),
            Self::SubscriptDeclaration(inner) => inner.span(),
            Self::SwitchStatement(inner) => inner.span(),
            Self::ThrowStatement(inner) => inner.span(),
            Self::TypealiasDeclaration(inner) => inner.span(),
            Self::VariableDeclaration(inner) => inner.span(),
            Self::WhileStatement(inner) => inner.span(),
        }
    }
}
#[derive(Debug, Clone)]
pub enum DeinitializerDeclarationChildren<'tree> {
    Boolean(::std::boxed::Box<Boolean<'tree>>),
    BreakStatement(::std::boxed::Box<BreakStatement<'tree>>),
    BuildConfigurationStatement(::std::boxed::Box<BuildConfigurationStatement<'tree>>),
    ClassDeclaration(::std::boxed::Box<ClassDeclaration<'tree>>),
    ConstantDeclaration(::std::boxed::Box<ConstantDeclaration<'tree>>),
    ContinueStatement(::std::boxed::Box<ContinueStatement<'tree>>),
    DeferStatement(::std::boxed::Box<DeferStatement<'tree>>),
    DeinitializerDeclaration(::std::boxed::Box<DeinitializerDeclaration<'tree>>),
    DiagnosticStatement(::std::boxed::Box<DiagnosticStatement<'tree>>),
    DoStatement(::std::boxed::Box<DoStatement<'tree>>),
    EnumDeclaration(::std::boxed::Box<EnumDeclaration<'tree>>),
    ExtensionDeclaration(::std::boxed::Box<ExtensionDeclaration<'tree>>),
    FallthroughStatement(::std::boxed::Box<FallthroughStatement<'tree>>),
    ForStatement(::std::boxed::Box<ForStatement<'tree>>),
    FunctionDeclaration(::std::boxed::Box<FunctionDeclaration<'tree>>),
    GuardStatement(::std::boxed::Box<GuardStatement<'tree>>),
    Identifier(::std::boxed::Box<Identifier<'tree>>),
    IfStatement(::std::boxed::Box<IfStatement<'tree>>),
    ImportDeclaration(::std::boxed::Box<ImportDeclaration<'tree>>),
    InitializerDeclaration(::std::boxed::Box<InitializerDeclaration<'tree>>),
    LabeledStatement(::std::boxed::Box<LabeledStatement<'tree>>),
    LineControlStatement(::std::boxed::Box<LineControlStatement<'tree>>),
    Nil(::std::boxed::Box<Nil<'tree>>),
    Number(::std::boxed::Box<Number<'tree>>),
    OperatorDeclaration(::std::boxed::Box<OperatorDeclaration<'tree>>),
    ProtocolDeclaration(::std::boxed::Box<ProtocolDeclaration<'tree>>),
    RepeatWhileStatement(::std::boxed::Box<RepeatWhileStatement<'tree>>),
    ReturnStatement(::std::boxed::Box<ReturnStatement<'tree>>),
    String(::std::boxed::Box<String<'tree>>),
    StructDeclaration(::std::boxed::Box<StructDeclaration<'tree>>),
    SubscriptDeclaration(::std::boxed::Box<SubscriptDeclaration<'tree>>),
    SwitchStatement(::std::boxed::Box<SwitchStatement<'tree>>),
    ThrowStatement(::std::boxed::Box<ThrowStatement<'tree>>),
    TypealiasDeclaration(::std::boxed::Box<TypealiasDeclaration<'tree>>),
    VariableDeclaration(::std::boxed::Box<VariableDeclaration<'tree>>),
    WhileStatement(::std::boxed::Box<WhileStatement<'tree>>),
}
impl<'tree> ::treesitter_types::FromNode<'tree> for DeinitializerDeclarationChildren<'tree> {
    #[allow(clippy::collapsible_else_if)]
    fn from_node(
        node: ::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        match node.kind() {
            "boolean" => Ok(Self::Boolean(::std::boxed::Box::new(
                <Boolean as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "break_statement" => Ok(Self::BreakStatement(::std::boxed::Box::new(
                <BreakStatement as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "build_configuration_statement" => {
                Ok(Self::BuildConfigurationStatement(::std::boxed::Box::new(
                    <BuildConfigurationStatement as ::treesitter_types::FromNode>::from_node(
                        node, src,
                    )?,
                )))
            }
            "class_declaration" => Ok(Self::ClassDeclaration(::std::boxed::Box::new(
                <ClassDeclaration as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "constant_declaration" => Ok(Self::ConstantDeclaration(::std::boxed::Box::new(
                <ConstantDeclaration as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "continue_statement" => Ok(Self::ContinueStatement(::std::boxed::Box::new(
                <ContinueStatement as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "defer_statement" => Ok(Self::DeferStatement(::std::boxed::Box::new(
                <DeferStatement as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "deinitializer_declaration" => {
                Ok(Self::DeinitializerDeclaration(::std::boxed::Box::new(
                    <DeinitializerDeclaration as ::treesitter_types::FromNode>::from_node(
                        node, src,
                    )?,
                )))
            }
            "diagnostic_statement" => Ok(Self::DiagnosticStatement(::std::boxed::Box::new(
                <DiagnosticStatement as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "do_statement" => Ok(Self::DoStatement(::std::boxed::Box::new(
                <DoStatement as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "enum_declaration" => Ok(Self::EnumDeclaration(::std::boxed::Box::new(
                <EnumDeclaration as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "extension_declaration" => Ok(Self::ExtensionDeclaration(::std::boxed::Box::new(
                <ExtensionDeclaration as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "fallthrough_statement" => Ok(Self::FallthroughStatement(::std::boxed::Box::new(
                <FallthroughStatement as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "for_statement" => Ok(Self::ForStatement(::std::boxed::Box::new(
                <ForStatement as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "function_declaration" => Ok(Self::FunctionDeclaration(::std::boxed::Box::new(
                <FunctionDeclaration as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "guard_statement" => Ok(Self::GuardStatement(::std::boxed::Box::new(
                <GuardStatement as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "identifier" => Ok(Self::Identifier(::std::boxed::Box::new(
                <Identifier as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "if_statement" => Ok(Self::IfStatement(::std::boxed::Box::new(
                <IfStatement as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "import_declaration" => Ok(Self::ImportDeclaration(::std::boxed::Box::new(
                <ImportDeclaration as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "initializer_declaration" => Ok(Self::InitializerDeclaration(::std::boxed::Box::new(
                <InitializerDeclaration as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "labeled_statement" => Ok(Self::LabeledStatement(::std::boxed::Box::new(
                <LabeledStatement as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "line_control_statement" => Ok(Self::LineControlStatement(::std::boxed::Box::new(
                <LineControlStatement as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "nil" => Ok(Self::Nil(::std::boxed::Box::new(
                <Nil as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "number" => Ok(Self::Number(::std::boxed::Box::new(
                <Number as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "operator_declaration" => Ok(Self::OperatorDeclaration(::std::boxed::Box::new(
                <OperatorDeclaration as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "protocol_declaration" => Ok(Self::ProtocolDeclaration(::std::boxed::Box::new(
                <ProtocolDeclaration as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "repeat_while_statement" => Ok(Self::RepeatWhileStatement(::std::boxed::Box::new(
                <RepeatWhileStatement as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "return_statement" => Ok(Self::ReturnStatement(::std::boxed::Box::new(
                <ReturnStatement as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "string" => Ok(Self::String(::std::boxed::Box::new(
                <String as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "struct_declaration" => Ok(Self::StructDeclaration(::std::boxed::Box::new(
                <StructDeclaration as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "subscript_declaration" => Ok(Self::SubscriptDeclaration(::std::boxed::Box::new(
                <SubscriptDeclaration as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "switch_statement" => Ok(Self::SwitchStatement(::std::boxed::Box::new(
                <SwitchStatement as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "throw_statement" => Ok(Self::ThrowStatement(::std::boxed::Box::new(
                <ThrowStatement as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "typealias_declaration" => Ok(Self::TypealiasDeclaration(::std::boxed::Box::new(
                <TypealiasDeclaration as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "variable_declaration" => Ok(Self::VariableDeclaration(::std::boxed::Box::new(
                <VariableDeclaration as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "while_statement" => Ok(Self::WhileStatement(::std::boxed::Box::new(
                <WhileStatement as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            other => Err(::treesitter_types::ParseError::unexpected_kind(other, node)),
        }
    }
}
impl ::treesitter_types::Spanned for DeinitializerDeclarationChildren<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        match self {
            Self::Boolean(inner) => inner.span(),
            Self::BreakStatement(inner) => inner.span(),
            Self::BuildConfigurationStatement(inner) => inner.span(),
            Self::ClassDeclaration(inner) => inner.span(),
            Self::ConstantDeclaration(inner) => inner.span(),
            Self::ContinueStatement(inner) => inner.span(),
            Self::DeferStatement(inner) => inner.span(),
            Self::DeinitializerDeclaration(inner) => inner.span(),
            Self::DiagnosticStatement(inner) => inner.span(),
            Self::DoStatement(inner) => inner.span(),
            Self::EnumDeclaration(inner) => inner.span(),
            Self::ExtensionDeclaration(inner) => inner.span(),
            Self::FallthroughStatement(inner) => inner.span(),
            Self::ForStatement(inner) => inner.span(),
            Self::FunctionDeclaration(inner) => inner.span(),
            Self::GuardStatement(inner) => inner.span(),
            Self::Identifier(inner) => inner.span(),
            Self::IfStatement(inner) => inner.span(),
            Self::ImportDeclaration(inner) => inner.span(),
            Self::InitializerDeclaration(inner) => inner.span(),
            Self::LabeledStatement(inner) => inner.span(),
            Self::LineControlStatement(inner) => inner.span(),
            Self::Nil(inner) => inner.span(),
            Self::Number(inner) => inner.span(),
            Self::OperatorDeclaration(inner) => inner.span(),
            Self::ProtocolDeclaration(inner) => inner.span(),
            Self::RepeatWhileStatement(inner) => inner.span(),
            Self::ReturnStatement(inner) => inner.span(),
            Self::String(inner) => inner.span(),
            Self::StructDeclaration(inner) => inner.span(),
            Self::SubscriptDeclaration(inner) => inner.span(),
            Self::SwitchStatement(inner) => inner.span(),
            Self::ThrowStatement(inner) => inner.span(),
            Self::TypealiasDeclaration(inner) => inner.span(),
            Self::VariableDeclaration(inner) => inner.span(),
            Self::WhileStatement(inner) => inner.span(),
        }
    }
}
#[derive(Debug, Clone)]
pub enum DictionaryTypeChildren<'tree> {
    ArrayType(::std::boxed::Box<ArrayType<'tree>>),
    DictionaryType(::std::boxed::Box<DictionaryType<'tree>>),
    StandardType(::std::boxed::Box<StandardType<'tree>>),
    TupleType(::std::boxed::Box<TupleType<'tree>>),
    TypeIdentifier(::std::boxed::Box<TypeIdentifier<'tree>>),
}
impl<'tree> ::treesitter_types::FromNode<'tree> for DictionaryTypeChildren<'tree> {
    #[allow(clippy::collapsible_else_if)]
    fn from_node(
        node: ::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        match node.kind() {
            "array_type" => Ok(Self::ArrayType(::std::boxed::Box::new(
                <ArrayType as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "dictionary_type" => Ok(Self::DictionaryType(::std::boxed::Box::new(
                <DictionaryType as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "standard_type" => Ok(Self::StandardType(::std::boxed::Box::new(
                <StandardType as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "tuple_type" => Ok(Self::TupleType(::std::boxed::Box::new(
                <TupleType as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "type_identifier" => Ok(Self::TypeIdentifier(::std::boxed::Box::new(
                <TypeIdentifier as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            other => Err(::treesitter_types::ParseError::unexpected_kind(other, node)),
        }
    }
}
impl ::treesitter_types::Spanned for DictionaryTypeChildren<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        match self {
            Self::ArrayType(inner) => inner.span(),
            Self::DictionaryType(inner) => inner.span(),
            Self::StandardType(inner) => inner.span(),
            Self::TupleType(inner) => inner.span(),
            Self::TypeIdentifier(inner) => inner.span(),
        }
    }
}
#[derive(Debug, Clone)]
pub enum DoStatementChildren<'tree> {
    Boolean(::std::boxed::Box<Boolean<'tree>>),
    BreakStatement(::std::boxed::Box<BreakStatement<'tree>>),
    BuildConfigurationStatement(::std::boxed::Box<BuildConfigurationStatement<'tree>>),
    CatchClause(::std::boxed::Box<CatchClause<'tree>>),
    ClassDeclaration(::std::boxed::Box<ClassDeclaration<'tree>>),
    ConstantDeclaration(::std::boxed::Box<ConstantDeclaration<'tree>>),
    ContinueStatement(::std::boxed::Box<ContinueStatement<'tree>>),
    DeferStatement(::std::boxed::Box<DeferStatement<'tree>>),
    DeinitializerDeclaration(::std::boxed::Box<DeinitializerDeclaration<'tree>>),
    DiagnosticStatement(::std::boxed::Box<DiagnosticStatement<'tree>>),
    DoStatement(::std::boxed::Box<DoStatement<'tree>>),
    EnumDeclaration(::std::boxed::Box<EnumDeclaration<'tree>>),
    ExtensionDeclaration(::std::boxed::Box<ExtensionDeclaration<'tree>>),
    FallthroughStatement(::std::boxed::Box<FallthroughStatement<'tree>>),
    ForStatement(::std::boxed::Box<ForStatement<'tree>>),
    FunctionDeclaration(::std::boxed::Box<FunctionDeclaration<'tree>>),
    GuardStatement(::std::boxed::Box<GuardStatement<'tree>>),
    Identifier(::std::boxed::Box<Identifier<'tree>>),
    IfStatement(::std::boxed::Box<IfStatement<'tree>>),
    ImportDeclaration(::std::boxed::Box<ImportDeclaration<'tree>>),
    InitializerDeclaration(::std::boxed::Box<InitializerDeclaration<'tree>>),
    LabeledStatement(::std::boxed::Box<LabeledStatement<'tree>>),
    LineControlStatement(::std::boxed::Box<LineControlStatement<'tree>>),
    Nil(::std::boxed::Box<Nil<'tree>>),
    Number(::std::boxed::Box<Number<'tree>>),
    OperatorDeclaration(::std::boxed::Box<OperatorDeclaration<'tree>>),
    ProtocolDeclaration(::std::boxed::Box<ProtocolDeclaration<'tree>>),
    RepeatWhileStatement(::std::boxed::Box<RepeatWhileStatement<'tree>>),
    ReturnStatement(::std::boxed::Box<ReturnStatement<'tree>>),
    String(::std::boxed::Box<String<'tree>>),
    StructDeclaration(::std::boxed::Box<StructDeclaration<'tree>>),
    SubscriptDeclaration(::std::boxed::Box<SubscriptDeclaration<'tree>>),
    SwitchStatement(::std::boxed::Box<SwitchStatement<'tree>>),
    ThrowStatement(::std::boxed::Box<ThrowStatement<'tree>>),
    TypealiasDeclaration(::std::boxed::Box<TypealiasDeclaration<'tree>>),
    VariableDeclaration(::std::boxed::Box<VariableDeclaration<'tree>>),
    WhileStatement(::std::boxed::Box<WhileStatement<'tree>>),
}
impl<'tree> ::treesitter_types::FromNode<'tree> for DoStatementChildren<'tree> {
    #[allow(clippy::collapsible_else_if)]
    fn from_node(
        node: ::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        match node.kind() {
            "boolean" => Ok(Self::Boolean(::std::boxed::Box::new(
                <Boolean as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "break_statement" => Ok(Self::BreakStatement(::std::boxed::Box::new(
                <BreakStatement as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "build_configuration_statement" => {
                Ok(Self::BuildConfigurationStatement(::std::boxed::Box::new(
                    <BuildConfigurationStatement as ::treesitter_types::FromNode>::from_node(
                        node, src,
                    )?,
                )))
            }
            "catch_clause" => Ok(Self::CatchClause(::std::boxed::Box::new(
                <CatchClause as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "class_declaration" => Ok(Self::ClassDeclaration(::std::boxed::Box::new(
                <ClassDeclaration as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "constant_declaration" => Ok(Self::ConstantDeclaration(::std::boxed::Box::new(
                <ConstantDeclaration as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "continue_statement" => Ok(Self::ContinueStatement(::std::boxed::Box::new(
                <ContinueStatement as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "defer_statement" => Ok(Self::DeferStatement(::std::boxed::Box::new(
                <DeferStatement as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "deinitializer_declaration" => {
                Ok(Self::DeinitializerDeclaration(::std::boxed::Box::new(
                    <DeinitializerDeclaration as ::treesitter_types::FromNode>::from_node(
                        node, src,
                    )?,
                )))
            }
            "diagnostic_statement" => Ok(Self::DiagnosticStatement(::std::boxed::Box::new(
                <DiagnosticStatement as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "do_statement" => Ok(Self::DoStatement(::std::boxed::Box::new(
                <DoStatement as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "enum_declaration" => Ok(Self::EnumDeclaration(::std::boxed::Box::new(
                <EnumDeclaration as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "extension_declaration" => Ok(Self::ExtensionDeclaration(::std::boxed::Box::new(
                <ExtensionDeclaration as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "fallthrough_statement" => Ok(Self::FallthroughStatement(::std::boxed::Box::new(
                <FallthroughStatement as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "for_statement" => Ok(Self::ForStatement(::std::boxed::Box::new(
                <ForStatement as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "function_declaration" => Ok(Self::FunctionDeclaration(::std::boxed::Box::new(
                <FunctionDeclaration as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "guard_statement" => Ok(Self::GuardStatement(::std::boxed::Box::new(
                <GuardStatement as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "identifier" => Ok(Self::Identifier(::std::boxed::Box::new(
                <Identifier as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "if_statement" => Ok(Self::IfStatement(::std::boxed::Box::new(
                <IfStatement as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "import_declaration" => Ok(Self::ImportDeclaration(::std::boxed::Box::new(
                <ImportDeclaration as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "initializer_declaration" => Ok(Self::InitializerDeclaration(::std::boxed::Box::new(
                <InitializerDeclaration as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "labeled_statement" => Ok(Self::LabeledStatement(::std::boxed::Box::new(
                <LabeledStatement as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "line_control_statement" => Ok(Self::LineControlStatement(::std::boxed::Box::new(
                <LineControlStatement as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "nil" => Ok(Self::Nil(::std::boxed::Box::new(
                <Nil as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "number" => Ok(Self::Number(::std::boxed::Box::new(
                <Number as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "operator_declaration" => Ok(Self::OperatorDeclaration(::std::boxed::Box::new(
                <OperatorDeclaration as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "protocol_declaration" => Ok(Self::ProtocolDeclaration(::std::boxed::Box::new(
                <ProtocolDeclaration as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "repeat_while_statement" => Ok(Self::RepeatWhileStatement(::std::boxed::Box::new(
                <RepeatWhileStatement as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "return_statement" => Ok(Self::ReturnStatement(::std::boxed::Box::new(
                <ReturnStatement as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "string" => Ok(Self::String(::std::boxed::Box::new(
                <String as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "struct_declaration" => Ok(Self::StructDeclaration(::std::boxed::Box::new(
                <StructDeclaration as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "subscript_declaration" => Ok(Self::SubscriptDeclaration(::std::boxed::Box::new(
                <SubscriptDeclaration as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "switch_statement" => Ok(Self::SwitchStatement(::std::boxed::Box::new(
                <SwitchStatement as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "throw_statement" => Ok(Self::ThrowStatement(::std::boxed::Box::new(
                <ThrowStatement as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "typealias_declaration" => Ok(Self::TypealiasDeclaration(::std::boxed::Box::new(
                <TypealiasDeclaration as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "variable_declaration" => Ok(Self::VariableDeclaration(::std::boxed::Box::new(
                <VariableDeclaration as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "while_statement" => Ok(Self::WhileStatement(::std::boxed::Box::new(
                <WhileStatement as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            other => Err(::treesitter_types::ParseError::unexpected_kind(other, node)),
        }
    }
}
impl ::treesitter_types::Spanned for DoStatementChildren<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        match self {
            Self::Boolean(inner) => inner.span(),
            Self::BreakStatement(inner) => inner.span(),
            Self::BuildConfigurationStatement(inner) => inner.span(),
            Self::CatchClause(inner) => inner.span(),
            Self::ClassDeclaration(inner) => inner.span(),
            Self::ConstantDeclaration(inner) => inner.span(),
            Self::ContinueStatement(inner) => inner.span(),
            Self::DeferStatement(inner) => inner.span(),
            Self::DeinitializerDeclaration(inner) => inner.span(),
            Self::DiagnosticStatement(inner) => inner.span(),
            Self::DoStatement(inner) => inner.span(),
            Self::EnumDeclaration(inner) => inner.span(),
            Self::ExtensionDeclaration(inner) => inner.span(),
            Self::FallthroughStatement(inner) => inner.span(),
            Self::ForStatement(inner) => inner.span(),
            Self::FunctionDeclaration(inner) => inner.span(),
            Self::GuardStatement(inner) => inner.span(),
            Self::Identifier(inner) => inner.span(),
            Self::IfStatement(inner) => inner.span(),
            Self::ImportDeclaration(inner) => inner.span(),
            Self::InitializerDeclaration(inner) => inner.span(),
            Self::LabeledStatement(inner) => inner.span(),
            Self::LineControlStatement(inner) => inner.span(),
            Self::Nil(inner) => inner.span(),
            Self::Number(inner) => inner.span(),
            Self::OperatorDeclaration(inner) => inner.span(),
            Self::ProtocolDeclaration(inner) => inner.span(),
            Self::RepeatWhileStatement(inner) => inner.span(),
            Self::ReturnStatement(inner) => inner.span(),
            Self::String(inner) => inner.span(),
            Self::StructDeclaration(inner) => inner.span(),
            Self::SubscriptDeclaration(inner) => inner.span(),
            Self::SwitchStatement(inner) => inner.span(),
            Self::ThrowStatement(inner) => inner.span(),
            Self::TypealiasDeclaration(inner) => inner.span(),
            Self::VariableDeclaration(inner) => inner.span(),
            Self::WhileStatement(inner) => inner.span(),
        }
    }
}
#[derive(Debug, Clone)]
pub enum EnumCasePatternChildren<'tree> {
    Boolean(::std::boxed::Box<Boolean<'tree>>),
    Identifier(::std::boxed::Box<Identifier<'tree>>),
    Nil(::std::boxed::Box<Nil<'tree>>),
    Number(::std::boxed::Box<Number<'tree>>),
    String(::std::boxed::Box<String<'tree>>),
}
impl<'tree> ::treesitter_types::FromNode<'tree> for EnumCasePatternChildren<'tree> {
    #[allow(clippy::collapsible_else_if)]
    fn from_node(
        node: ::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        match node.kind() {
            "boolean" => Ok(Self::Boolean(::std::boxed::Box::new(
                <Boolean as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "identifier" => Ok(Self::Identifier(::std::boxed::Box::new(
                <Identifier as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "nil" => Ok(Self::Nil(::std::boxed::Box::new(
                <Nil as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "number" => Ok(Self::Number(::std::boxed::Box::new(
                <Number as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "string" => Ok(Self::String(::std::boxed::Box::new(
                <String as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            other => Err(::treesitter_types::ParseError::unexpected_kind(other, node)),
        }
    }
}
impl ::treesitter_types::Spanned for EnumCasePatternChildren<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        match self {
            Self::Boolean(inner) => inner.span(),
            Self::Identifier(inner) => inner.span(),
            Self::Nil(inner) => inner.span(),
            Self::Number(inner) => inner.span(),
            Self::String(inner) => inner.span(),
        }
    }
}
#[derive(Debug, Clone)]
pub enum EnumDeclarationChildren<'tree> {
    CaseDeclaration(::std::boxed::Box<CaseDeclaration<'tree>>),
    ClassDeclaration(::std::boxed::Box<ClassDeclaration<'tree>>),
    ConstantDeclaration(::std::boxed::Box<ConstantDeclaration<'tree>>),
    DeinitializerDeclaration(::std::boxed::Box<DeinitializerDeclaration<'tree>>),
    EnumDeclaration(::std::boxed::Box<EnumDeclaration<'tree>>),
    ExtensionDeclaration(::std::boxed::Box<ExtensionDeclaration<'tree>>),
    FunctionDeclaration(::std::boxed::Box<FunctionDeclaration<'tree>>),
    Identifier(::std::boxed::Box<Identifier<'tree>>),
    ImportDeclaration(::std::boxed::Box<ImportDeclaration<'tree>>),
    InitializerDeclaration(::std::boxed::Box<InitializerDeclaration<'tree>>),
    Modifier(::std::boxed::Box<Modifier<'tree>>),
    OperatorDeclaration(::std::boxed::Box<OperatorDeclaration<'tree>>),
    ProtocolDeclaration(::std::boxed::Box<ProtocolDeclaration<'tree>>),
    StructDeclaration(::std::boxed::Box<StructDeclaration<'tree>>),
    SubscriptDeclaration(::std::boxed::Box<SubscriptDeclaration<'tree>>),
    TypealiasDeclaration(::std::boxed::Box<TypealiasDeclaration<'tree>>),
    VariableDeclaration(::std::boxed::Box<VariableDeclaration<'tree>>),
}
impl<'tree> ::treesitter_types::FromNode<'tree> for EnumDeclarationChildren<'tree> {
    #[allow(clippy::collapsible_else_if)]
    fn from_node(
        node: ::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        match node.kind() {
            "case_declaration" => Ok(Self::CaseDeclaration(::std::boxed::Box::new(
                <CaseDeclaration as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "class_declaration" => Ok(Self::ClassDeclaration(::std::boxed::Box::new(
                <ClassDeclaration as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "constant_declaration" => Ok(Self::ConstantDeclaration(::std::boxed::Box::new(
                <ConstantDeclaration as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "deinitializer_declaration" => {
                Ok(Self::DeinitializerDeclaration(::std::boxed::Box::new(
                    <DeinitializerDeclaration as ::treesitter_types::FromNode>::from_node(
                        node, src,
                    )?,
                )))
            }
            "enum_declaration" => Ok(Self::EnumDeclaration(::std::boxed::Box::new(
                <EnumDeclaration as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "extension_declaration" => Ok(Self::ExtensionDeclaration(::std::boxed::Box::new(
                <ExtensionDeclaration as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "function_declaration" => Ok(Self::FunctionDeclaration(::std::boxed::Box::new(
                <FunctionDeclaration as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "identifier" => Ok(Self::Identifier(::std::boxed::Box::new(
                <Identifier as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "import_declaration" => Ok(Self::ImportDeclaration(::std::boxed::Box::new(
                <ImportDeclaration as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "initializer_declaration" => Ok(Self::InitializerDeclaration(::std::boxed::Box::new(
                <InitializerDeclaration as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "modifier" => Ok(Self::Modifier(::std::boxed::Box::new(
                <Modifier as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "operator_declaration" => Ok(Self::OperatorDeclaration(::std::boxed::Box::new(
                <OperatorDeclaration as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "protocol_declaration" => Ok(Self::ProtocolDeclaration(::std::boxed::Box::new(
                <ProtocolDeclaration as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "struct_declaration" => Ok(Self::StructDeclaration(::std::boxed::Box::new(
                <StructDeclaration as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "subscript_declaration" => Ok(Self::SubscriptDeclaration(::std::boxed::Box::new(
                <SubscriptDeclaration as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "typealias_declaration" => Ok(Self::TypealiasDeclaration(::std::boxed::Box::new(
                <TypealiasDeclaration as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "variable_declaration" => Ok(Self::VariableDeclaration(::std::boxed::Box::new(
                <VariableDeclaration as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            other => Err(::treesitter_types::ParseError::unexpected_kind(other, node)),
        }
    }
}
impl ::treesitter_types::Spanned for EnumDeclarationChildren<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        match self {
            Self::CaseDeclaration(inner) => inner.span(),
            Self::ClassDeclaration(inner) => inner.span(),
            Self::ConstantDeclaration(inner) => inner.span(),
            Self::DeinitializerDeclaration(inner) => inner.span(),
            Self::EnumDeclaration(inner) => inner.span(),
            Self::ExtensionDeclaration(inner) => inner.span(),
            Self::FunctionDeclaration(inner) => inner.span(),
            Self::Identifier(inner) => inner.span(),
            Self::ImportDeclaration(inner) => inner.span(),
            Self::InitializerDeclaration(inner) => inner.span(),
            Self::Modifier(inner) => inner.span(),
            Self::OperatorDeclaration(inner) => inner.span(),
            Self::ProtocolDeclaration(inner) => inner.span(),
            Self::StructDeclaration(inner) => inner.span(),
            Self::SubscriptDeclaration(inner) => inner.span(),
            Self::TypealiasDeclaration(inner) => inner.span(),
            Self::VariableDeclaration(inner) => inner.span(),
        }
    }
}
#[derive(Debug, Clone)]
pub enum ExtensionDeclarationChildren<'tree> {
    ClassDeclaration(::std::boxed::Box<ClassDeclaration<'tree>>),
    ConstantDeclaration(::std::boxed::Box<ConstantDeclaration<'tree>>),
    DeinitializerDeclaration(::std::boxed::Box<DeinitializerDeclaration<'tree>>),
    EnumDeclaration(::std::boxed::Box<EnumDeclaration<'tree>>),
    ExtensionDeclaration(::std::boxed::Box<ExtensionDeclaration<'tree>>),
    FunctionDeclaration(::std::boxed::Box<FunctionDeclaration<'tree>>),
    Identifier(::std::boxed::Box<Identifier<'tree>>),
    ImportDeclaration(::std::boxed::Box<ImportDeclaration<'tree>>),
    InitializerDeclaration(::std::boxed::Box<InitializerDeclaration<'tree>>),
    Modifier(::std::boxed::Box<Modifier<'tree>>),
    OperatorDeclaration(::std::boxed::Box<OperatorDeclaration<'tree>>),
    ProtocolDeclaration(::std::boxed::Box<ProtocolDeclaration<'tree>>),
    StructDeclaration(::std::boxed::Box<StructDeclaration<'tree>>),
    SubscriptDeclaration(::std::boxed::Box<SubscriptDeclaration<'tree>>),
    TypealiasDeclaration(::std::boxed::Box<TypealiasDeclaration<'tree>>),
    VariableDeclaration(::std::boxed::Box<VariableDeclaration<'tree>>),
}
impl<'tree> ::treesitter_types::FromNode<'tree> for ExtensionDeclarationChildren<'tree> {
    #[allow(clippy::collapsible_else_if)]
    fn from_node(
        node: ::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        match node.kind() {
            "class_declaration" => Ok(Self::ClassDeclaration(::std::boxed::Box::new(
                <ClassDeclaration as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "constant_declaration" => Ok(Self::ConstantDeclaration(::std::boxed::Box::new(
                <ConstantDeclaration as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "deinitializer_declaration" => {
                Ok(Self::DeinitializerDeclaration(::std::boxed::Box::new(
                    <DeinitializerDeclaration as ::treesitter_types::FromNode>::from_node(
                        node, src,
                    )?,
                )))
            }
            "enum_declaration" => Ok(Self::EnumDeclaration(::std::boxed::Box::new(
                <EnumDeclaration as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "extension_declaration" => Ok(Self::ExtensionDeclaration(::std::boxed::Box::new(
                <ExtensionDeclaration as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "function_declaration" => Ok(Self::FunctionDeclaration(::std::boxed::Box::new(
                <FunctionDeclaration as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "identifier" => Ok(Self::Identifier(::std::boxed::Box::new(
                <Identifier as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "import_declaration" => Ok(Self::ImportDeclaration(::std::boxed::Box::new(
                <ImportDeclaration as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "initializer_declaration" => Ok(Self::InitializerDeclaration(::std::boxed::Box::new(
                <InitializerDeclaration as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "modifier" => Ok(Self::Modifier(::std::boxed::Box::new(
                <Modifier as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "operator_declaration" => Ok(Self::OperatorDeclaration(::std::boxed::Box::new(
                <OperatorDeclaration as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "protocol_declaration" => Ok(Self::ProtocolDeclaration(::std::boxed::Box::new(
                <ProtocolDeclaration as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "struct_declaration" => Ok(Self::StructDeclaration(::std::boxed::Box::new(
                <StructDeclaration as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "subscript_declaration" => Ok(Self::SubscriptDeclaration(::std::boxed::Box::new(
                <SubscriptDeclaration as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "typealias_declaration" => Ok(Self::TypealiasDeclaration(::std::boxed::Box::new(
                <TypealiasDeclaration as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "variable_declaration" => Ok(Self::VariableDeclaration(::std::boxed::Box::new(
                <VariableDeclaration as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            other => Err(::treesitter_types::ParseError::unexpected_kind(other, node)),
        }
    }
}
impl ::treesitter_types::Spanned for ExtensionDeclarationChildren<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        match self {
            Self::ClassDeclaration(inner) => inner.span(),
            Self::ConstantDeclaration(inner) => inner.span(),
            Self::DeinitializerDeclaration(inner) => inner.span(),
            Self::EnumDeclaration(inner) => inner.span(),
            Self::ExtensionDeclaration(inner) => inner.span(),
            Self::FunctionDeclaration(inner) => inner.span(),
            Self::Identifier(inner) => inner.span(),
            Self::ImportDeclaration(inner) => inner.span(),
            Self::InitializerDeclaration(inner) => inner.span(),
            Self::Modifier(inner) => inner.span(),
            Self::OperatorDeclaration(inner) => inner.span(),
            Self::ProtocolDeclaration(inner) => inner.span(),
            Self::StructDeclaration(inner) => inner.span(),
            Self::SubscriptDeclaration(inner) => inner.span(),
            Self::TypealiasDeclaration(inner) => inner.span(),
            Self::VariableDeclaration(inner) => inner.span(),
        }
    }
}
#[derive(Debug, Clone)]
pub enum ForStatementChildren<'tree> {
    ArrayType(::std::boxed::Box<ArrayType<'tree>>),
    AsPattern(::std::boxed::Box<AsPattern<'tree>>),
    Boolean(::std::boxed::Box<Boolean<'tree>>),
    BreakStatement(::std::boxed::Box<BreakStatement<'tree>>),
    BuildConfigurationStatement(::std::boxed::Box<BuildConfigurationStatement<'tree>>),
    ClassDeclaration(::std::boxed::Box<ClassDeclaration<'tree>>),
    ConstantDeclaration(::std::boxed::Box<ConstantDeclaration<'tree>>),
    ContinueStatement(::std::boxed::Box<ContinueStatement<'tree>>),
    DeferStatement(::std::boxed::Box<DeferStatement<'tree>>),
    DeinitializerDeclaration(::std::boxed::Box<DeinitializerDeclaration<'tree>>),
    DiagnosticStatement(::std::boxed::Box<DiagnosticStatement<'tree>>),
    DictionaryType(::std::boxed::Box<DictionaryType<'tree>>),
    DoStatement(::std::boxed::Box<DoStatement<'tree>>),
    EnumCasePattern(::std::boxed::Box<EnumCasePattern<'tree>>),
    EnumDeclaration(::std::boxed::Box<EnumDeclaration<'tree>>),
    ExtensionDeclaration(::std::boxed::Box<ExtensionDeclaration<'tree>>),
    FallthroughStatement(::std::boxed::Box<FallthroughStatement<'tree>>),
    ForStatement(::std::boxed::Box<ForStatement<'tree>>),
    FunctionDeclaration(::std::boxed::Box<FunctionDeclaration<'tree>>),
    GuardStatement(::std::boxed::Box<GuardStatement<'tree>>),
    Identifier(::std::boxed::Box<Identifier<'tree>>),
    IfStatement(::std::boxed::Box<IfStatement<'tree>>),
    ImportDeclaration(::std::boxed::Box<ImportDeclaration<'tree>>),
    InitializerDeclaration(::std::boxed::Box<InitializerDeclaration<'tree>>),
    IsPattern(::std::boxed::Box<IsPattern<'tree>>),
    LabeledStatement(::std::boxed::Box<LabeledStatement<'tree>>),
    LineControlStatement(::std::boxed::Box<LineControlStatement<'tree>>),
    Nil(::std::boxed::Box<Nil<'tree>>),
    Number(::std::boxed::Box<Number<'tree>>),
    OperatorDeclaration(::std::boxed::Box<OperatorDeclaration<'tree>>),
    OptionalPattern(::std::boxed::Box<OptionalPattern<'tree>>),
    ProtocolDeclaration(::std::boxed::Box<ProtocolDeclaration<'tree>>),
    RepeatWhileStatement(::std::boxed::Box<RepeatWhileStatement<'tree>>),
    ReturnStatement(::std::boxed::Box<ReturnStatement<'tree>>),
    String(::std::boxed::Box<String<'tree>>),
    StructDeclaration(::std::boxed::Box<StructDeclaration<'tree>>),
    SubscriptDeclaration(::std::boxed::Box<SubscriptDeclaration<'tree>>),
    SwitchStatement(::std::boxed::Box<SwitchStatement<'tree>>),
    ThrowStatement(::std::boxed::Box<ThrowStatement<'tree>>),
    Type(::std::boxed::Box<Type<'tree>>),
    TypealiasDeclaration(::std::boxed::Box<TypealiasDeclaration<'tree>>),
    ValueBindingPattern(::std::boxed::Box<ValueBindingPattern<'tree>>),
    VariableDeclaration(::std::boxed::Box<VariableDeclaration<'tree>>),
    WhileStatement(::std::boxed::Box<WhileStatement<'tree>>),
    WildcardPattern(::std::boxed::Box<WildcardPattern<'tree>>),
}
impl<'tree> ::treesitter_types::FromNode<'tree> for ForStatementChildren<'tree> {
    #[allow(clippy::collapsible_else_if)]
    fn from_node(
        node: ::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        match node.kind() {
            "array_type" => Ok(Self::ArrayType(::std::boxed::Box::new(
                <ArrayType as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "as_pattern" => Ok(Self::AsPattern(::std::boxed::Box::new(
                <AsPattern as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "boolean" => Ok(Self::Boolean(::std::boxed::Box::new(
                <Boolean as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "break_statement" => Ok(Self::BreakStatement(::std::boxed::Box::new(
                <BreakStatement as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "build_configuration_statement" => {
                Ok(Self::BuildConfigurationStatement(::std::boxed::Box::new(
                    <BuildConfigurationStatement as ::treesitter_types::FromNode>::from_node(
                        node, src,
                    )?,
                )))
            }
            "class_declaration" => Ok(Self::ClassDeclaration(::std::boxed::Box::new(
                <ClassDeclaration as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "constant_declaration" => Ok(Self::ConstantDeclaration(::std::boxed::Box::new(
                <ConstantDeclaration as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "continue_statement" => Ok(Self::ContinueStatement(::std::boxed::Box::new(
                <ContinueStatement as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "defer_statement" => Ok(Self::DeferStatement(::std::boxed::Box::new(
                <DeferStatement as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "deinitializer_declaration" => {
                Ok(Self::DeinitializerDeclaration(::std::boxed::Box::new(
                    <DeinitializerDeclaration as ::treesitter_types::FromNode>::from_node(
                        node, src,
                    )?,
                )))
            }
            "diagnostic_statement" => Ok(Self::DiagnosticStatement(::std::boxed::Box::new(
                <DiagnosticStatement as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "dictionary_type" => Ok(Self::DictionaryType(::std::boxed::Box::new(
                <DictionaryType as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "do_statement" => Ok(Self::DoStatement(::std::boxed::Box::new(
                <DoStatement as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "enum_case_pattern" => Ok(Self::EnumCasePattern(::std::boxed::Box::new(
                <EnumCasePattern as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "enum_declaration" => Ok(Self::EnumDeclaration(::std::boxed::Box::new(
                <EnumDeclaration as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "extension_declaration" => Ok(Self::ExtensionDeclaration(::std::boxed::Box::new(
                <ExtensionDeclaration as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "fallthrough_statement" => Ok(Self::FallthroughStatement(::std::boxed::Box::new(
                <FallthroughStatement as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "for_statement" => Ok(Self::ForStatement(::std::boxed::Box::new(
                <ForStatement as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "function_declaration" => Ok(Self::FunctionDeclaration(::std::boxed::Box::new(
                <FunctionDeclaration as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "guard_statement" => Ok(Self::GuardStatement(::std::boxed::Box::new(
                <GuardStatement as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "identifier" => Ok(Self::Identifier(::std::boxed::Box::new(
                <Identifier as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "if_statement" => Ok(Self::IfStatement(::std::boxed::Box::new(
                <IfStatement as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "import_declaration" => Ok(Self::ImportDeclaration(::std::boxed::Box::new(
                <ImportDeclaration as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "initializer_declaration" => Ok(Self::InitializerDeclaration(::std::boxed::Box::new(
                <InitializerDeclaration as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "is_pattern" => Ok(Self::IsPattern(::std::boxed::Box::new(
                <IsPattern as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "labeled_statement" => Ok(Self::LabeledStatement(::std::boxed::Box::new(
                <LabeledStatement as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "line_control_statement" => Ok(Self::LineControlStatement(::std::boxed::Box::new(
                <LineControlStatement as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "nil" => Ok(Self::Nil(::std::boxed::Box::new(
                <Nil as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "number" => Ok(Self::Number(::std::boxed::Box::new(
                <Number as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "operator_declaration" => Ok(Self::OperatorDeclaration(::std::boxed::Box::new(
                <OperatorDeclaration as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "optional_pattern" => Ok(Self::OptionalPattern(::std::boxed::Box::new(
                <OptionalPattern as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "protocol_declaration" => Ok(Self::ProtocolDeclaration(::std::boxed::Box::new(
                <ProtocolDeclaration as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "repeat_while_statement" => Ok(Self::RepeatWhileStatement(::std::boxed::Box::new(
                <RepeatWhileStatement as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "return_statement" => Ok(Self::ReturnStatement(::std::boxed::Box::new(
                <ReturnStatement as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "string" => Ok(Self::String(::std::boxed::Box::new(
                <String as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "struct_declaration" => Ok(Self::StructDeclaration(::std::boxed::Box::new(
                <StructDeclaration as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "subscript_declaration" => Ok(Self::SubscriptDeclaration(::std::boxed::Box::new(
                <SubscriptDeclaration as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "switch_statement" => Ok(Self::SwitchStatement(::std::boxed::Box::new(
                <SwitchStatement as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "throw_statement" => Ok(Self::ThrowStatement(::std::boxed::Box::new(
                <ThrowStatement as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "type" => Ok(Self::Type(::std::boxed::Box::new(
                <Type as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "typealias_declaration" => Ok(Self::TypealiasDeclaration(::std::boxed::Box::new(
                <TypealiasDeclaration as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "value_binding_pattern" => Ok(Self::ValueBindingPattern(::std::boxed::Box::new(
                <ValueBindingPattern as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "variable_declaration" => Ok(Self::VariableDeclaration(::std::boxed::Box::new(
                <VariableDeclaration as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "while_statement" => Ok(Self::WhileStatement(::std::boxed::Box::new(
                <WhileStatement as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "wildcard_pattern" => Ok(Self::WildcardPattern(::std::boxed::Box::new(
                <WildcardPattern as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            other => Err(::treesitter_types::ParseError::unexpected_kind(other, node)),
        }
    }
}
impl ::treesitter_types::Spanned for ForStatementChildren<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        match self {
            Self::ArrayType(inner) => inner.span(),
            Self::AsPattern(inner) => inner.span(),
            Self::Boolean(inner) => inner.span(),
            Self::BreakStatement(inner) => inner.span(),
            Self::BuildConfigurationStatement(inner) => inner.span(),
            Self::ClassDeclaration(inner) => inner.span(),
            Self::ConstantDeclaration(inner) => inner.span(),
            Self::ContinueStatement(inner) => inner.span(),
            Self::DeferStatement(inner) => inner.span(),
            Self::DeinitializerDeclaration(inner) => inner.span(),
            Self::DiagnosticStatement(inner) => inner.span(),
            Self::DictionaryType(inner) => inner.span(),
            Self::DoStatement(inner) => inner.span(),
            Self::EnumCasePattern(inner) => inner.span(),
            Self::EnumDeclaration(inner) => inner.span(),
            Self::ExtensionDeclaration(inner) => inner.span(),
            Self::FallthroughStatement(inner) => inner.span(),
            Self::ForStatement(inner) => inner.span(),
            Self::FunctionDeclaration(inner) => inner.span(),
            Self::GuardStatement(inner) => inner.span(),
            Self::Identifier(inner) => inner.span(),
            Self::IfStatement(inner) => inner.span(),
            Self::ImportDeclaration(inner) => inner.span(),
            Self::InitializerDeclaration(inner) => inner.span(),
            Self::IsPattern(inner) => inner.span(),
            Self::LabeledStatement(inner) => inner.span(),
            Self::LineControlStatement(inner) => inner.span(),
            Self::Nil(inner) => inner.span(),
            Self::Number(inner) => inner.span(),
            Self::OperatorDeclaration(inner) => inner.span(),
            Self::OptionalPattern(inner) => inner.span(),
            Self::ProtocolDeclaration(inner) => inner.span(),
            Self::RepeatWhileStatement(inner) => inner.span(),
            Self::ReturnStatement(inner) => inner.span(),
            Self::String(inner) => inner.span(),
            Self::StructDeclaration(inner) => inner.span(),
            Self::SubscriptDeclaration(inner) => inner.span(),
            Self::SwitchStatement(inner) => inner.span(),
            Self::ThrowStatement(inner) => inner.span(),
            Self::Type(inner) => inner.span(),
            Self::TypealiasDeclaration(inner) => inner.span(),
            Self::ValueBindingPattern(inner) => inner.span(),
            Self::VariableDeclaration(inner) => inner.span(),
            Self::WhileStatement(inner) => inner.span(),
            Self::WildcardPattern(inner) => inner.span(),
        }
    }
}
#[derive(Debug, Clone)]
pub enum FunctionDeclarationName<'tree> {
    Identifier(::std::boxed::Box<Identifier<'tree>>),
    Operator(::std::boxed::Box<Operator<'tree>>),
}
impl<'tree> ::treesitter_types::FromNode<'tree> for FunctionDeclarationName<'tree> {
    #[allow(clippy::collapsible_else_if)]
    fn from_node(
        node: ::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        match node.kind() {
            "identifier" => Ok(Self::Identifier(::std::boxed::Box::new(
                <Identifier as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "operator" => Ok(Self::Operator(::std::boxed::Box::new(
                <Operator as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            other => Err(::treesitter_types::ParseError::unexpected_kind(other, node)),
        }
    }
}
impl ::treesitter_types::Spanned for FunctionDeclarationName<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        match self {
            Self::Identifier(inner) => inner.span(),
            Self::Operator(inner) => inner.span(),
        }
    }
}
#[derive(Debug, Clone)]
pub enum FunctionDeclarationReturn<'tree> {
    Bang(::treesitter_types::Span),
    Question(::treesitter_types::Span),
    Type(::std::boxed::Box<Type<'tree>>),
}
impl<'tree> ::treesitter_types::FromNode<'tree> for FunctionDeclarationReturn<'tree> {
    #[allow(clippy::collapsible_else_if)]
    fn from_node(
        node: ::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        match node.kind() {
            "!" => Ok(Self::Bang(::treesitter_types::Span::from(node))),
            "?" => Ok(Self::Question(::treesitter_types::Span::from(node))),
            "type" => Ok(Self::Type(::std::boxed::Box::new(
                <Type as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            other => Err(::treesitter_types::ParseError::unexpected_kind(other, node)),
        }
    }
}
impl ::treesitter_types::Spanned for FunctionDeclarationReturn<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        match self {
            Self::Bang(span) => *span,
            Self::Question(span) => *span,
            Self::Type(inner) => inner.span(),
        }
    }
}
#[derive(Debug, Clone)]
pub enum FunctionDeclarationChildren<'tree> {
    Boolean(::std::boxed::Box<Boolean<'tree>>),
    BreakStatement(::std::boxed::Box<BreakStatement<'tree>>),
    BuildConfigurationStatement(::std::boxed::Box<BuildConfigurationStatement<'tree>>),
    ClassDeclaration(::std::boxed::Box<ClassDeclaration<'tree>>),
    ConstantDeclaration(::std::boxed::Box<ConstantDeclaration<'tree>>),
    ContinueStatement(::std::boxed::Box<ContinueStatement<'tree>>),
    DeferStatement(::std::boxed::Box<DeferStatement<'tree>>),
    DeinitializerDeclaration(::std::boxed::Box<DeinitializerDeclaration<'tree>>),
    DiagnosticStatement(::std::boxed::Box<DiagnosticStatement<'tree>>),
    DoStatement(::std::boxed::Box<DoStatement<'tree>>),
    EnumDeclaration(::std::boxed::Box<EnumDeclaration<'tree>>),
    ExtensionDeclaration(::std::boxed::Box<ExtensionDeclaration<'tree>>),
    FallthroughStatement(::std::boxed::Box<FallthroughStatement<'tree>>),
    ForStatement(::std::boxed::Box<ForStatement<'tree>>),
    FunctionDeclaration(::std::boxed::Box<FunctionDeclaration<'tree>>),
    GenericClause(::std::boxed::Box<GenericClause<'tree>>),
    GuardStatement(::std::boxed::Box<GuardStatement<'tree>>),
    Identifier(::std::boxed::Box<Identifier<'tree>>),
    IfStatement(::std::boxed::Box<IfStatement<'tree>>),
    ImportDeclaration(::std::boxed::Box<ImportDeclaration<'tree>>),
    InitializerDeclaration(::std::boxed::Box<InitializerDeclaration<'tree>>),
    LabeledStatement(::std::boxed::Box<LabeledStatement<'tree>>),
    LineControlStatement(::std::boxed::Box<LineControlStatement<'tree>>),
    Modifier(::std::boxed::Box<Modifier<'tree>>),
    Nil(::std::boxed::Box<Nil<'tree>>),
    Number(::std::boxed::Box<Number<'tree>>),
    OperatorDeclaration(::std::boxed::Box<OperatorDeclaration<'tree>>),
    ParameterList(::std::boxed::Box<ParameterList<'tree>>),
    ProtocolDeclaration(::std::boxed::Box<ProtocolDeclaration<'tree>>),
    RepeatWhileStatement(::std::boxed::Box<RepeatWhileStatement<'tree>>),
    ReturnStatement(::std::boxed::Box<ReturnStatement<'tree>>),
    String(::std::boxed::Box<String<'tree>>),
    StructDeclaration(::std::boxed::Box<StructDeclaration<'tree>>),
    SubscriptDeclaration(::std::boxed::Box<SubscriptDeclaration<'tree>>),
    SwitchStatement(::std::boxed::Box<SwitchStatement<'tree>>),
    ThrowStatement(::std::boxed::Box<ThrowStatement<'tree>>),
    TypealiasDeclaration(::std::boxed::Box<TypealiasDeclaration<'tree>>),
    VariableDeclaration(::std::boxed::Box<VariableDeclaration<'tree>>),
    WhileStatement(::std::boxed::Box<WhileStatement<'tree>>),
}
impl<'tree> ::treesitter_types::FromNode<'tree> for FunctionDeclarationChildren<'tree> {
    #[allow(clippy::collapsible_else_if)]
    fn from_node(
        node: ::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        match node.kind() {
            "boolean" => Ok(Self::Boolean(::std::boxed::Box::new(
                <Boolean as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "break_statement" => Ok(Self::BreakStatement(::std::boxed::Box::new(
                <BreakStatement as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "build_configuration_statement" => {
                Ok(Self::BuildConfigurationStatement(::std::boxed::Box::new(
                    <BuildConfigurationStatement as ::treesitter_types::FromNode>::from_node(
                        node, src,
                    )?,
                )))
            }
            "class_declaration" => Ok(Self::ClassDeclaration(::std::boxed::Box::new(
                <ClassDeclaration as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "constant_declaration" => Ok(Self::ConstantDeclaration(::std::boxed::Box::new(
                <ConstantDeclaration as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "continue_statement" => Ok(Self::ContinueStatement(::std::boxed::Box::new(
                <ContinueStatement as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "defer_statement" => Ok(Self::DeferStatement(::std::boxed::Box::new(
                <DeferStatement as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "deinitializer_declaration" => {
                Ok(Self::DeinitializerDeclaration(::std::boxed::Box::new(
                    <DeinitializerDeclaration as ::treesitter_types::FromNode>::from_node(
                        node, src,
                    )?,
                )))
            }
            "diagnostic_statement" => Ok(Self::DiagnosticStatement(::std::boxed::Box::new(
                <DiagnosticStatement as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "do_statement" => Ok(Self::DoStatement(::std::boxed::Box::new(
                <DoStatement as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "enum_declaration" => Ok(Self::EnumDeclaration(::std::boxed::Box::new(
                <EnumDeclaration as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "extension_declaration" => Ok(Self::ExtensionDeclaration(::std::boxed::Box::new(
                <ExtensionDeclaration as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "fallthrough_statement" => Ok(Self::FallthroughStatement(::std::boxed::Box::new(
                <FallthroughStatement as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "for_statement" => Ok(Self::ForStatement(::std::boxed::Box::new(
                <ForStatement as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "function_declaration" => Ok(Self::FunctionDeclaration(::std::boxed::Box::new(
                <FunctionDeclaration as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "generic_clause" => Ok(Self::GenericClause(::std::boxed::Box::new(
                <GenericClause as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "guard_statement" => Ok(Self::GuardStatement(::std::boxed::Box::new(
                <GuardStatement as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "identifier" => Ok(Self::Identifier(::std::boxed::Box::new(
                <Identifier as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "if_statement" => Ok(Self::IfStatement(::std::boxed::Box::new(
                <IfStatement as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "import_declaration" => Ok(Self::ImportDeclaration(::std::boxed::Box::new(
                <ImportDeclaration as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "initializer_declaration" => Ok(Self::InitializerDeclaration(::std::boxed::Box::new(
                <InitializerDeclaration as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "labeled_statement" => Ok(Self::LabeledStatement(::std::boxed::Box::new(
                <LabeledStatement as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "line_control_statement" => Ok(Self::LineControlStatement(::std::boxed::Box::new(
                <LineControlStatement as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "modifier" => Ok(Self::Modifier(::std::boxed::Box::new(
                <Modifier as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "nil" => Ok(Self::Nil(::std::boxed::Box::new(
                <Nil as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "number" => Ok(Self::Number(::std::boxed::Box::new(
                <Number as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "operator_declaration" => Ok(Self::OperatorDeclaration(::std::boxed::Box::new(
                <OperatorDeclaration as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "parameter_list" => Ok(Self::ParameterList(::std::boxed::Box::new(
                <ParameterList as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "protocol_declaration" => Ok(Self::ProtocolDeclaration(::std::boxed::Box::new(
                <ProtocolDeclaration as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "repeat_while_statement" => Ok(Self::RepeatWhileStatement(::std::boxed::Box::new(
                <RepeatWhileStatement as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "return_statement" => Ok(Self::ReturnStatement(::std::boxed::Box::new(
                <ReturnStatement as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "string" => Ok(Self::String(::std::boxed::Box::new(
                <String as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "struct_declaration" => Ok(Self::StructDeclaration(::std::boxed::Box::new(
                <StructDeclaration as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "subscript_declaration" => Ok(Self::SubscriptDeclaration(::std::boxed::Box::new(
                <SubscriptDeclaration as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "switch_statement" => Ok(Self::SwitchStatement(::std::boxed::Box::new(
                <SwitchStatement as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "throw_statement" => Ok(Self::ThrowStatement(::std::boxed::Box::new(
                <ThrowStatement as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "typealias_declaration" => Ok(Self::TypealiasDeclaration(::std::boxed::Box::new(
                <TypealiasDeclaration as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "variable_declaration" => Ok(Self::VariableDeclaration(::std::boxed::Box::new(
                <VariableDeclaration as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "while_statement" => Ok(Self::WhileStatement(::std::boxed::Box::new(
                <WhileStatement as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            other => Err(::treesitter_types::ParseError::unexpected_kind(other, node)),
        }
    }
}
impl ::treesitter_types::Spanned for FunctionDeclarationChildren<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        match self {
            Self::Boolean(inner) => inner.span(),
            Self::BreakStatement(inner) => inner.span(),
            Self::BuildConfigurationStatement(inner) => inner.span(),
            Self::ClassDeclaration(inner) => inner.span(),
            Self::ConstantDeclaration(inner) => inner.span(),
            Self::ContinueStatement(inner) => inner.span(),
            Self::DeferStatement(inner) => inner.span(),
            Self::DeinitializerDeclaration(inner) => inner.span(),
            Self::DiagnosticStatement(inner) => inner.span(),
            Self::DoStatement(inner) => inner.span(),
            Self::EnumDeclaration(inner) => inner.span(),
            Self::ExtensionDeclaration(inner) => inner.span(),
            Self::FallthroughStatement(inner) => inner.span(),
            Self::ForStatement(inner) => inner.span(),
            Self::FunctionDeclaration(inner) => inner.span(),
            Self::GenericClause(inner) => inner.span(),
            Self::GuardStatement(inner) => inner.span(),
            Self::Identifier(inner) => inner.span(),
            Self::IfStatement(inner) => inner.span(),
            Self::ImportDeclaration(inner) => inner.span(),
            Self::InitializerDeclaration(inner) => inner.span(),
            Self::LabeledStatement(inner) => inner.span(),
            Self::LineControlStatement(inner) => inner.span(),
            Self::Modifier(inner) => inner.span(),
            Self::Nil(inner) => inner.span(),
            Self::Number(inner) => inner.span(),
            Self::OperatorDeclaration(inner) => inner.span(),
            Self::ParameterList(inner) => inner.span(),
            Self::ProtocolDeclaration(inner) => inner.span(),
            Self::RepeatWhileStatement(inner) => inner.span(),
            Self::ReturnStatement(inner) => inner.span(),
            Self::String(inner) => inner.span(),
            Self::StructDeclaration(inner) => inner.span(),
            Self::SubscriptDeclaration(inner) => inner.span(),
            Self::SwitchStatement(inner) => inner.span(),
            Self::ThrowStatement(inner) => inner.span(),
            Self::TypealiasDeclaration(inner) => inner.span(),
            Self::VariableDeclaration(inner) => inner.span(),
            Self::WhileStatement(inner) => inner.span(),
        }
    }
}
#[derive(Debug, Clone)]
pub enum GenericClauseType<'tree> {
    ArrayType(::std::boxed::Box<ArrayType<'tree>>),
    DictionaryType(::std::boxed::Box<DictionaryType<'tree>>),
    StandardType(::std::boxed::Box<StandardType<'tree>>),
    TupleType(::std::boxed::Box<TupleType<'tree>>),
    TypeIdentifier(::std::boxed::Box<TypeIdentifier<'tree>>),
}
impl<'tree> ::treesitter_types::FromNode<'tree> for GenericClauseType<'tree> {
    #[allow(clippy::collapsible_else_if)]
    fn from_node(
        node: ::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        match node.kind() {
            "array_type" => Ok(Self::ArrayType(::std::boxed::Box::new(
                <ArrayType as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "dictionary_type" => Ok(Self::DictionaryType(::std::boxed::Box::new(
                <DictionaryType as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "standard_type" => Ok(Self::StandardType(::std::boxed::Box::new(
                <StandardType as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "tuple_type" => Ok(Self::TupleType(::std::boxed::Box::new(
                <TupleType as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "type_identifier" => Ok(Self::TypeIdentifier(::std::boxed::Box::new(
                <TypeIdentifier as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            other => Err(::treesitter_types::ParseError::unexpected_kind(other, node)),
        }
    }
}
impl ::treesitter_types::Spanned for GenericClauseType<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        match self {
            Self::ArrayType(inner) => inner.span(),
            Self::DictionaryType(inner) => inner.span(),
            Self::StandardType(inner) => inner.span(),
            Self::TupleType(inner) => inner.span(),
            Self::TypeIdentifier(inner) => inner.span(),
        }
    }
}
#[derive(Debug, Clone)]
pub enum GuardStatementChildren<'tree> {
    AvailabilityCondition(::std::boxed::Box<AvailabilityCondition<'tree>>),
    Boolean(::std::boxed::Box<Boolean<'tree>>),
    BreakStatement(::std::boxed::Box<BreakStatement<'tree>>),
    BuildConfigurationStatement(::std::boxed::Box<BuildConfigurationStatement<'tree>>),
    CaseCondition(::std::boxed::Box<CaseCondition<'tree>>),
    ClassDeclaration(::std::boxed::Box<ClassDeclaration<'tree>>),
    ConstantDeclaration(::std::boxed::Box<ConstantDeclaration<'tree>>),
    ContinueStatement(::std::boxed::Box<ContinueStatement<'tree>>),
    DeferStatement(::std::boxed::Box<DeferStatement<'tree>>),
    DeinitializerDeclaration(::std::boxed::Box<DeinitializerDeclaration<'tree>>),
    DiagnosticStatement(::std::boxed::Box<DiagnosticStatement<'tree>>),
    DoStatement(::std::boxed::Box<DoStatement<'tree>>),
    EnumDeclaration(::std::boxed::Box<EnumDeclaration<'tree>>),
    ExtensionDeclaration(::std::boxed::Box<ExtensionDeclaration<'tree>>),
    FallthroughStatement(::std::boxed::Box<FallthroughStatement<'tree>>),
    ForStatement(::std::boxed::Box<ForStatement<'tree>>),
    FunctionDeclaration(::std::boxed::Box<FunctionDeclaration<'tree>>),
    GuardStatement(::std::boxed::Box<GuardStatement<'tree>>),
    Identifier(::std::boxed::Box<Identifier<'tree>>),
    IfStatement(::std::boxed::Box<IfStatement<'tree>>),
    ImportDeclaration(::std::boxed::Box<ImportDeclaration<'tree>>),
    InitializerDeclaration(::std::boxed::Box<InitializerDeclaration<'tree>>),
    LabeledStatement(::std::boxed::Box<LabeledStatement<'tree>>),
    LineControlStatement(::std::boxed::Box<LineControlStatement<'tree>>),
    Nil(::std::boxed::Box<Nil<'tree>>),
    Number(::std::boxed::Box<Number<'tree>>),
    OperatorDeclaration(::std::boxed::Box<OperatorDeclaration<'tree>>),
    OptionalBindingCondition(::std::boxed::Box<OptionalBindingCondition<'tree>>),
    ProtocolDeclaration(::std::boxed::Box<ProtocolDeclaration<'tree>>),
    RepeatWhileStatement(::std::boxed::Box<RepeatWhileStatement<'tree>>),
    ReturnStatement(::std::boxed::Box<ReturnStatement<'tree>>),
    String(::std::boxed::Box<String<'tree>>),
    StructDeclaration(::std::boxed::Box<StructDeclaration<'tree>>),
    SubscriptDeclaration(::std::boxed::Box<SubscriptDeclaration<'tree>>),
    SwitchStatement(::std::boxed::Box<SwitchStatement<'tree>>),
    ThrowStatement(::std::boxed::Box<ThrowStatement<'tree>>),
    TypealiasDeclaration(::std::boxed::Box<TypealiasDeclaration<'tree>>),
    VariableDeclaration(::std::boxed::Box<VariableDeclaration<'tree>>),
    WhileStatement(::std::boxed::Box<WhileStatement<'tree>>),
}
impl<'tree> ::treesitter_types::FromNode<'tree> for GuardStatementChildren<'tree> {
    #[allow(clippy::collapsible_else_if)]
    fn from_node(
        node: ::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        match node.kind() {
            "availability_condition" => Ok(Self::AvailabilityCondition(::std::boxed::Box::new(
                <AvailabilityCondition as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "boolean" => Ok(Self::Boolean(::std::boxed::Box::new(
                <Boolean as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "break_statement" => Ok(Self::BreakStatement(::std::boxed::Box::new(
                <BreakStatement as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "build_configuration_statement" => {
                Ok(Self::BuildConfigurationStatement(::std::boxed::Box::new(
                    <BuildConfigurationStatement as ::treesitter_types::FromNode>::from_node(
                        node, src,
                    )?,
                )))
            }
            "case_condition" => Ok(Self::CaseCondition(::std::boxed::Box::new(
                <CaseCondition as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "class_declaration" => Ok(Self::ClassDeclaration(::std::boxed::Box::new(
                <ClassDeclaration as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "constant_declaration" => Ok(Self::ConstantDeclaration(::std::boxed::Box::new(
                <ConstantDeclaration as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "continue_statement" => Ok(Self::ContinueStatement(::std::boxed::Box::new(
                <ContinueStatement as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "defer_statement" => Ok(Self::DeferStatement(::std::boxed::Box::new(
                <DeferStatement as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "deinitializer_declaration" => {
                Ok(Self::DeinitializerDeclaration(::std::boxed::Box::new(
                    <DeinitializerDeclaration as ::treesitter_types::FromNode>::from_node(
                        node, src,
                    )?,
                )))
            }
            "diagnostic_statement" => Ok(Self::DiagnosticStatement(::std::boxed::Box::new(
                <DiagnosticStatement as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "do_statement" => Ok(Self::DoStatement(::std::boxed::Box::new(
                <DoStatement as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "enum_declaration" => Ok(Self::EnumDeclaration(::std::boxed::Box::new(
                <EnumDeclaration as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "extension_declaration" => Ok(Self::ExtensionDeclaration(::std::boxed::Box::new(
                <ExtensionDeclaration as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "fallthrough_statement" => Ok(Self::FallthroughStatement(::std::boxed::Box::new(
                <FallthroughStatement as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "for_statement" => Ok(Self::ForStatement(::std::boxed::Box::new(
                <ForStatement as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "function_declaration" => Ok(Self::FunctionDeclaration(::std::boxed::Box::new(
                <FunctionDeclaration as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "guard_statement" => Ok(Self::GuardStatement(::std::boxed::Box::new(
                <GuardStatement as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "identifier" => Ok(Self::Identifier(::std::boxed::Box::new(
                <Identifier as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "if_statement" => Ok(Self::IfStatement(::std::boxed::Box::new(
                <IfStatement as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "import_declaration" => Ok(Self::ImportDeclaration(::std::boxed::Box::new(
                <ImportDeclaration as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "initializer_declaration" => Ok(Self::InitializerDeclaration(::std::boxed::Box::new(
                <InitializerDeclaration as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "labeled_statement" => Ok(Self::LabeledStatement(::std::boxed::Box::new(
                <LabeledStatement as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "line_control_statement" => Ok(Self::LineControlStatement(::std::boxed::Box::new(
                <LineControlStatement as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "nil" => Ok(Self::Nil(::std::boxed::Box::new(
                <Nil as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "number" => Ok(Self::Number(::std::boxed::Box::new(
                <Number as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "operator_declaration" => Ok(Self::OperatorDeclaration(::std::boxed::Box::new(
                <OperatorDeclaration as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "optional_binding_condition" => {
                Ok(Self::OptionalBindingCondition(::std::boxed::Box::new(
                    <OptionalBindingCondition as ::treesitter_types::FromNode>::from_node(
                        node, src,
                    )?,
                )))
            }
            "protocol_declaration" => Ok(Self::ProtocolDeclaration(::std::boxed::Box::new(
                <ProtocolDeclaration as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "repeat_while_statement" => Ok(Self::RepeatWhileStatement(::std::boxed::Box::new(
                <RepeatWhileStatement as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "return_statement" => Ok(Self::ReturnStatement(::std::boxed::Box::new(
                <ReturnStatement as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "string" => Ok(Self::String(::std::boxed::Box::new(
                <String as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "struct_declaration" => Ok(Self::StructDeclaration(::std::boxed::Box::new(
                <StructDeclaration as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "subscript_declaration" => Ok(Self::SubscriptDeclaration(::std::boxed::Box::new(
                <SubscriptDeclaration as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "switch_statement" => Ok(Self::SwitchStatement(::std::boxed::Box::new(
                <SwitchStatement as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "throw_statement" => Ok(Self::ThrowStatement(::std::boxed::Box::new(
                <ThrowStatement as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "typealias_declaration" => Ok(Self::TypealiasDeclaration(::std::boxed::Box::new(
                <TypealiasDeclaration as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "variable_declaration" => Ok(Self::VariableDeclaration(::std::boxed::Box::new(
                <VariableDeclaration as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "while_statement" => Ok(Self::WhileStatement(::std::boxed::Box::new(
                <WhileStatement as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            other => Err(::treesitter_types::ParseError::unexpected_kind(other, node)),
        }
    }
}
impl ::treesitter_types::Spanned for GuardStatementChildren<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        match self {
            Self::AvailabilityCondition(inner) => inner.span(),
            Self::Boolean(inner) => inner.span(),
            Self::BreakStatement(inner) => inner.span(),
            Self::BuildConfigurationStatement(inner) => inner.span(),
            Self::CaseCondition(inner) => inner.span(),
            Self::ClassDeclaration(inner) => inner.span(),
            Self::ConstantDeclaration(inner) => inner.span(),
            Self::ContinueStatement(inner) => inner.span(),
            Self::DeferStatement(inner) => inner.span(),
            Self::DeinitializerDeclaration(inner) => inner.span(),
            Self::DiagnosticStatement(inner) => inner.span(),
            Self::DoStatement(inner) => inner.span(),
            Self::EnumDeclaration(inner) => inner.span(),
            Self::ExtensionDeclaration(inner) => inner.span(),
            Self::FallthroughStatement(inner) => inner.span(),
            Self::ForStatement(inner) => inner.span(),
            Self::FunctionDeclaration(inner) => inner.span(),
            Self::GuardStatement(inner) => inner.span(),
            Self::Identifier(inner) => inner.span(),
            Self::IfStatement(inner) => inner.span(),
            Self::ImportDeclaration(inner) => inner.span(),
            Self::InitializerDeclaration(inner) => inner.span(),
            Self::LabeledStatement(inner) => inner.span(),
            Self::LineControlStatement(inner) => inner.span(),
            Self::Nil(inner) => inner.span(),
            Self::Number(inner) => inner.span(),
            Self::OperatorDeclaration(inner) => inner.span(),
            Self::OptionalBindingCondition(inner) => inner.span(),
            Self::ProtocolDeclaration(inner) => inner.span(),
            Self::RepeatWhileStatement(inner) => inner.span(),
            Self::ReturnStatement(inner) => inner.span(),
            Self::String(inner) => inner.span(),
            Self::StructDeclaration(inner) => inner.span(),
            Self::SubscriptDeclaration(inner) => inner.span(),
            Self::SwitchStatement(inner) => inner.span(),
            Self::ThrowStatement(inner) => inner.span(),
            Self::TypealiasDeclaration(inner) => inner.span(),
            Self::VariableDeclaration(inner) => inner.span(),
            Self::WhileStatement(inner) => inner.span(),
        }
    }
}
#[derive(Debug, Clone)]
pub enum IfStatementChildren<'tree> {
    AvailabilityCondition(::std::boxed::Box<AvailabilityCondition<'tree>>),
    Boolean(::std::boxed::Box<Boolean<'tree>>),
    BreakStatement(::std::boxed::Box<BreakStatement<'tree>>),
    BuildConfigurationStatement(::std::boxed::Box<BuildConfigurationStatement<'tree>>),
    CaseCondition(::std::boxed::Box<CaseCondition<'tree>>),
    ClassDeclaration(::std::boxed::Box<ClassDeclaration<'tree>>),
    ConstantDeclaration(::std::boxed::Box<ConstantDeclaration<'tree>>),
    ContinueStatement(::std::boxed::Box<ContinueStatement<'tree>>),
    DeferStatement(::std::boxed::Box<DeferStatement<'tree>>),
    DeinitializerDeclaration(::std::boxed::Box<DeinitializerDeclaration<'tree>>),
    DiagnosticStatement(::std::boxed::Box<DiagnosticStatement<'tree>>),
    DoStatement(::std::boxed::Box<DoStatement<'tree>>),
    EnumDeclaration(::std::boxed::Box<EnumDeclaration<'tree>>),
    ExtensionDeclaration(::std::boxed::Box<ExtensionDeclaration<'tree>>),
    FallthroughStatement(::std::boxed::Box<FallthroughStatement<'tree>>),
    ForStatement(::std::boxed::Box<ForStatement<'tree>>),
    FunctionDeclaration(::std::boxed::Box<FunctionDeclaration<'tree>>),
    GuardStatement(::std::boxed::Box<GuardStatement<'tree>>),
    Identifier(::std::boxed::Box<Identifier<'tree>>),
    IfStatement(::std::boxed::Box<IfStatement<'tree>>),
    ImportDeclaration(::std::boxed::Box<ImportDeclaration<'tree>>),
    InitializerDeclaration(::std::boxed::Box<InitializerDeclaration<'tree>>),
    LabeledStatement(::std::boxed::Box<LabeledStatement<'tree>>),
    LineControlStatement(::std::boxed::Box<LineControlStatement<'tree>>),
    Nil(::std::boxed::Box<Nil<'tree>>),
    Number(::std::boxed::Box<Number<'tree>>),
    OperatorDeclaration(::std::boxed::Box<OperatorDeclaration<'tree>>),
    OptionalBindingCondition(::std::boxed::Box<OptionalBindingCondition<'tree>>),
    ProtocolDeclaration(::std::boxed::Box<ProtocolDeclaration<'tree>>),
    RepeatWhileStatement(::std::boxed::Box<RepeatWhileStatement<'tree>>),
    ReturnStatement(::std::boxed::Box<ReturnStatement<'tree>>),
    String(::std::boxed::Box<String<'tree>>),
    StructDeclaration(::std::boxed::Box<StructDeclaration<'tree>>),
    SubscriptDeclaration(::std::boxed::Box<SubscriptDeclaration<'tree>>),
    SwitchStatement(::std::boxed::Box<SwitchStatement<'tree>>),
    ThrowStatement(::std::boxed::Box<ThrowStatement<'tree>>),
    TypealiasDeclaration(::std::boxed::Box<TypealiasDeclaration<'tree>>),
    VariableDeclaration(::std::boxed::Box<VariableDeclaration<'tree>>),
    WhileStatement(::std::boxed::Box<WhileStatement<'tree>>),
}
impl<'tree> ::treesitter_types::FromNode<'tree> for IfStatementChildren<'tree> {
    #[allow(clippy::collapsible_else_if)]
    fn from_node(
        node: ::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        match node.kind() {
            "availability_condition" => Ok(Self::AvailabilityCondition(::std::boxed::Box::new(
                <AvailabilityCondition as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "boolean" => Ok(Self::Boolean(::std::boxed::Box::new(
                <Boolean as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "break_statement" => Ok(Self::BreakStatement(::std::boxed::Box::new(
                <BreakStatement as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "build_configuration_statement" => {
                Ok(Self::BuildConfigurationStatement(::std::boxed::Box::new(
                    <BuildConfigurationStatement as ::treesitter_types::FromNode>::from_node(
                        node, src,
                    )?,
                )))
            }
            "case_condition" => Ok(Self::CaseCondition(::std::boxed::Box::new(
                <CaseCondition as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "class_declaration" => Ok(Self::ClassDeclaration(::std::boxed::Box::new(
                <ClassDeclaration as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "constant_declaration" => Ok(Self::ConstantDeclaration(::std::boxed::Box::new(
                <ConstantDeclaration as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "continue_statement" => Ok(Self::ContinueStatement(::std::boxed::Box::new(
                <ContinueStatement as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "defer_statement" => Ok(Self::DeferStatement(::std::boxed::Box::new(
                <DeferStatement as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "deinitializer_declaration" => {
                Ok(Self::DeinitializerDeclaration(::std::boxed::Box::new(
                    <DeinitializerDeclaration as ::treesitter_types::FromNode>::from_node(
                        node, src,
                    )?,
                )))
            }
            "diagnostic_statement" => Ok(Self::DiagnosticStatement(::std::boxed::Box::new(
                <DiagnosticStatement as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "do_statement" => Ok(Self::DoStatement(::std::boxed::Box::new(
                <DoStatement as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "enum_declaration" => Ok(Self::EnumDeclaration(::std::boxed::Box::new(
                <EnumDeclaration as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "extension_declaration" => Ok(Self::ExtensionDeclaration(::std::boxed::Box::new(
                <ExtensionDeclaration as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "fallthrough_statement" => Ok(Self::FallthroughStatement(::std::boxed::Box::new(
                <FallthroughStatement as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "for_statement" => Ok(Self::ForStatement(::std::boxed::Box::new(
                <ForStatement as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "function_declaration" => Ok(Self::FunctionDeclaration(::std::boxed::Box::new(
                <FunctionDeclaration as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "guard_statement" => Ok(Self::GuardStatement(::std::boxed::Box::new(
                <GuardStatement as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "identifier" => Ok(Self::Identifier(::std::boxed::Box::new(
                <Identifier as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "if_statement" => Ok(Self::IfStatement(::std::boxed::Box::new(
                <IfStatement as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "import_declaration" => Ok(Self::ImportDeclaration(::std::boxed::Box::new(
                <ImportDeclaration as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "initializer_declaration" => Ok(Self::InitializerDeclaration(::std::boxed::Box::new(
                <InitializerDeclaration as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "labeled_statement" => Ok(Self::LabeledStatement(::std::boxed::Box::new(
                <LabeledStatement as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "line_control_statement" => Ok(Self::LineControlStatement(::std::boxed::Box::new(
                <LineControlStatement as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "nil" => Ok(Self::Nil(::std::boxed::Box::new(
                <Nil as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "number" => Ok(Self::Number(::std::boxed::Box::new(
                <Number as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "operator_declaration" => Ok(Self::OperatorDeclaration(::std::boxed::Box::new(
                <OperatorDeclaration as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "optional_binding_condition" => {
                Ok(Self::OptionalBindingCondition(::std::boxed::Box::new(
                    <OptionalBindingCondition as ::treesitter_types::FromNode>::from_node(
                        node, src,
                    )?,
                )))
            }
            "protocol_declaration" => Ok(Self::ProtocolDeclaration(::std::boxed::Box::new(
                <ProtocolDeclaration as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "repeat_while_statement" => Ok(Self::RepeatWhileStatement(::std::boxed::Box::new(
                <RepeatWhileStatement as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "return_statement" => Ok(Self::ReturnStatement(::std::boxed::Box::new(
                <ReturnStatement as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "string" => Ok(Self::String(::std::boxed::Box::new(
                <String as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "struct_declaration" => Ok(Self::StructDeclaration(::std::boxed::Box::new(
                <StructDeclaration as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "subscript_declaration" => Ok(Self::SubscriptDeclaration(::std::boxed::Box::new(
                <SubscriptDeclaration as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "switch_statement" => Ok(Self::SwitchStatement(::std::boxed::Box::new(
                <SwitchStatement as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "throw_statement" => Ok(Self::ThrowStatement(::std::boxed::Box::new(
                <ThrowStatement as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "typealias_declaration" => Ok(Self::TypealiasDeclaration(::std::boxed::Box::new(
                <TypealiasDeclaration as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "variable_declaration" => Ok(Self::VariableDeclaration(::std::boxed::Box::new(
                <VariableDeclaration as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "while_statement" => Ok(Self::WhileStatement(::std::boxed::Box::new(
                <WhileStatement as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            other => Err(::treesitter_types::ParseError::unexpected_kind(other, node)),
        }
    }
}
impl ::treesitter_types::Spanned for IfStatementChildren<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        match self {
            Self::AvailabilityCondition(inner) => inner.span(),
            Self::Boolean(inner) => inner.span(),
            Self::BreakStatement(inner) => inner.span(),
            Self::BuildConfigurationStatement(inner) => inner.span(),
            Self::CaseCondition(inner) => inner.span(),
            Self::ClassDeclaration(inner) => inner.span(),
            Self::ConstantDeclaration(inner) => inner.span(),
            Self::ContinueStatement(inner) => inner.span(),
            Self::DeferStatement(inner) => inner.span(),
            Self::DeinitializerDeclaration(inner) => inner.span(),
            Self::DiagnosticStatement(inner) => inner.span(),
            Self::DoStatement(inner) => inner.span(),
            Self::EnumDeclaration(inner) => inner.span(),
            Self::ExtensionDeclaration(inner) => inner.span(),
            Self::FallthroughStatement(inner) => inner.span(),
            Self::ForStatement(inner) => inner.span(),
            Self::FunctionDeclaration(inner) => inner.span(),
            Self::GuardStatement(inner) => inner.span(),
            Self::Identifier(inner) => inner.span(),
            Self::IfStatement(inner) => inner.span(),
            Self::ImportDeclaration(inner) => inner.span(),
            Self::InitializerDeclaration(inner) => inner.span(),
            Self::LabeledStatement(inner) => inner.span(),
            Self::LineControlStatement(inner) => inner.span(),
            Self::Nil(inner) => inner.span(),
            Self::Number(inner) => inner.span(),
            Self::OperatorDeclaration(inner) => inner.span(),
            Self::OptionalBindingCondition(inner) => inner.span(),
            Self::ProtocolDeclaration(inner) => inner.span(),
            Self::RepeatWhileStatement(inner) => inner.span(),
            Self::ReturnStatement(inner) => inner.span(),
            Self::String(inner) => inner.span(),
            Self::StructDeclaration(inner) => inner.span(),
            Self::SubscriptDeclaration(inner) => inner.span(),
            Self::SwitchStatement(inner) => inner.span(),
            Self::ThrowStatement(inner) => inner.span(),
            Self::TypealiasDeclaration(inner) => inner.span(),
            Self::VariableDeclaration(inner) => inner.span(),
            Self::WhileStatement(inner) => inner.span(),
        }
    }
}
#[derive(Debug, Clone)]
pub enum ImportDeclarationChildren<'tree> {
    Identifier(::std::boxed::Box<Identifier<'tree>>),
    Operator(::std::boxed::Box<Operator<'tree>>),
}
impl<'tree> ::treesitter_types::FromNode<'tree> for ImportDeclarationChildren<'tree> {
    #[allow(clippy::collapsible_else_if)]
    fn from_node(
        node: ::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        match node.kind() {
            "identifier" => Ok(Self::Identifier(::std::boxed::Box::new(
                <Identifier as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "operator" => Ok(Self::Operator(::std::boxed::Box::new(
                <Operator as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            other => Err(::treesitter_types::ParseError::unexpected_kind(other, node)),
        }
    }
}
impl ::treesitter_types::Spanned for ImportDeclarationChildren<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        match self {
            Self::Identifier(inner) => inner.span(),
            Self::Operator(inner) => inner.span(),
        }
    }
}
#[derive(Debug, Clone)]
pub enum InitializerDeclarationChildren<'tree> {
    ArrayType(::std::boxed::Box<ArrayType<'tree>>),
    Boolean(::std::boxed::Box<Boolean<'tree>>),
    BreakStatement(::std::boxed::Box<BreakStatement<'tree>>),
    BuildConfigurationStatement(::std::boxed::Box<BuildConfigurationStatement<'tree>>),
    ClassDeclaration(::std::boxed::Box<ClassDeclaration<'tree>>),
    ConstantDeclaration(::std::boxed::Box<ConstantDeclaration<'tree>>),
    ContinueStatement(::std::boxed::Box<ContinueStatement<'tree>>),
    DeferStatement(::std::boxed::Box<DeferStatement<'tree>>),
    DeinitializerDeclaration(::std::boxed::Box<DeinitializerDeclaration<'tree>>),
    DiagnosticStatement(::std::boxed::Box<DiagnosticStatement<'tree>>),
    DictionaryType(::std::boxed::Box<DictionaryType<'tree>>),
    DoStatement(::std::boxed::Box<DoStatement<'tree>>),
    EnumDeclaration(::std::boxed::Box<EnumDeclaration<'tree>>),
    ExtensionDeclaration(::std::boxed::Box<ExtensionDeclaration<'tree>>),
    FallthroughStatement(::std::boxed::Box<FallthroughStatement<'tree>>),
    ForStatement(::std::boxed::Box<ForStatement<'tree>>),
    FunctionDeclaration(::std::boxed::Box<FunctionDeclaration<'tree>>),
    GuardStatement(::std::boxed::Box<GuardStatement<'tree>>),
    Identifier(::std::boxed::Box<Identifier<'tree>>),
    IfStatement(::std::boxed::Box<IfStatement<'tree>>),
    ImportDeclaration(::std::boxed::Box<ImportDeclaration<'tree>>),
    InitializerDeclaration(::std::boxed::Box<InitializerDeclaration<'tree>>),
    LabeledStatement(::std::boxed::Box<LabeledStatement<'tree>>),
    LineControlStatement(::std::boxed::Box<LineControlStatement<'tree>>),
    Nil(::std::boxed::Box<Nil<'tree>>),
    Number(::std::boxed::Box<Number<'tree>>),
    OperatorDeclaration(::std::boxed::Box<OperatorDeclaration<'tree>>),
    ProtocolDeclaration(::std::boxed::Box<ProtocolDeclaration<'tree>>),
    RepeatWhileStatement(::std::boxed::Box<RepeatWhileStatement<'tree>>),
    ReturnStatement(::std::boxed::Box<ReturnStatement<'tree>>),
    String(::std::boxed::Box<String<'tree>>),
    StructDeclaration(::std::boxed::Box<StructDeclaration<'tree>>),
    SubscriptDeclaration(::std::boxed::Box<SubscriptDeclaration<'tree>>),
    SwitchStatement(::std::boxed::Box<SwitchStatement<'tree>>),
    ThrowStatement(::std::boxed::Box<ThrowStatement<'tree>>),
    Type(::std::boxed::Box<Type<'tree>>),
    TypealiasDeclaration(::std::boxed::Box<TypealiasDeclaration<'tree>>),
    VariableDeclaration(::std::boxed::Box<VariableDeclaration<'tree>>),
    WhileStatement(::std::boxed::Box<WhileStatement<'tree>>),
}
impl<'tree> ::treesitter_types::FromNode<'tree> for InitializerDeclarationChildren<'tree> {
    #[allow(clippy::collapsible_else_if)]
    fn from_node(
        node: ::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        match node.kind() {
            "array_type" => Ok(Self::ArrayType(::std::boxed::Box::new(
                <ArrayType as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "boolean" => Ok(Self::Boolean(::std::boxed::Box::new(
                <Boolean as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "break_statement" => Ok(Self::BreakStatement(::std::boxed::Box::new(
                <BreakStatement as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "build_configuration_statement" => {
                Ok(Self::BuildConfigurationStatement(::std::boxed::Box::new(
                    <BuildConfigurationStatement as ::treesitter_types::FromNode>::from_node(
                        node, src,
                    )?,
                )))
            }
            "class_declaration" => Ok(Self::ClassDeclaration(::std::boxed::Box::new(
                <ClassDeclaration as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "constant_declaration" => Ok(Self::ConstantDeclaration(::std::boxed::Box::new(
                <ConstantDeclaration as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "continue_statement" => Ok(Self::ContinueStatement(::std::boxed::Box::new(
                <ContinueStatement as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "defer_statement" => Ok(Self::DeferStatement(::std::boxed::Box::new(
                <DeferStatement as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "deinitializer_declaration" => {
                Ok(Self::DeinitializerDeclaration(::std::boxed::Box::new(
                    <DeinitializerDeclaration as ::treesitter_types::FromNode>::from_node(
                        node, src,
                    )?,
                )))
            }
            "diagnostic_statement" => Ok(Self::DiagnosticStatement(::std::boxed::Box::new(
                <DiagnosticStatement as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "dictionary_type" => Ok(Self::DictionaryType(::std::boxed::Box::new(
                <DictionaryType as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "do_statement" => Ok(Self::DoStatement(::std::boxed::Box::new(
                <DoStatement as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "enum_declaration" => Ok(Self::EnumDeclaration(::std::boxed::Box::new(
                <EnumDeclaration as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "extension_declaration" => Ok(Self::ExtensionDeclaration(::std::boxed::Box::new(
                <ExtensionDeclaration as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "fallthrough_statement" => Ok(Self::FallthroughStatement(::std::boxed::Box::new(
                <FallthroughStatement as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "for_statement" => Ok(Self::ForStatement(::std::boxed::Box::new(
                <ForStatement as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "function_declaration" => Ok(Self::FunctionDeclaration(::std::boxed::Box::new(
                <FunctionDeclaration as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "guard_statement" => Ok(Self::GuardStatement(::std::boxed::Box::new(
                <GuardStatement as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "identifier" => Ok(Self::Identifier(::std::boxed::Box::new(
                <Identifier as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "if_statement" => Ok(Self::IfStatement(::std::boxed::Box::new(
                <IfStatement as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "import_declaration" => Ok(Self::ImportDeclaration(::std::boxed::Box::new(
                <ImportDeclaration as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "initializer_declaration" => Ok(Self::InitializerDeclaration(::std::boxed::Box::new(
                <InitializerDeclaration as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "labeled_statement" => Ok(Self::LabeledStatement(::std::boxed::Box::new(
                <LabeledStatement as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "line_control_statement" => Ok(Self::LineControlStatement(::std::boxed::Box::new(
                <LineControlStatement as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "nil" => Ok(Self::Nil(::std::boxed::Box::new(
                <Nil as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "number" => Ok(Self::Number(::std::boxed::Box::new(
                <Number as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "operator_declaration" => Ok(Self::OperatorDeclaration(::std::boxed::Box::new(
                <OperatorDeclaration as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "protocol_declaration" => Ok(Self::ProtocolDeclaration(::std::boxed::Box::new(
                <ProtocolDeclaration as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "repeat_while_statement" => Ok(Self::RepeatWhileStatement(::std::boxed::Box::new(
                <RepeatWhileStatement as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "return_statement" => Ok(Self::ReturnStatement(::std::boxed::Box::new(
                <ReturnStatement as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "string" => Ok(Self::String(::std::boxed::Box::new(
                <String as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "struct_declaration" => Ok(Self::StructDeclaration(::std::boxed::Box::new(
                <StructDeclaration as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "subscript_declaration" => Ok(Self::SubscriptDeclaration(::std::boxed::Box::new(
                <SubscriptDeclaration as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "switch_statement" => Ok(Self::SwitchStatement(::std::boxed::Box::new(
                <SwitchStatement as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "throw_statement" => Ok(Self::ThrowStatement(::std::boxed::Box::new(
                <ThrowStatement as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "type" => Ok(Self::Type(::std::boxed::Box::new(
                <Type as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "typealias_declaration" => Ok(Self::TypealiasDeclaration(::std::boxed::Box::new(
                <TypealiasDeclaration as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "variable_declaration" => Ok(Self::VariableDeclaration(::std::boxed::Box::new(
                <VariableDeclaration as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "while_statement" => Ok(Self::WhileStatement(::std::boxed::Box::new(
                <WhileStatement as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            other => Err(::treesitter_types::ParseError::unexpected_kind(other, node)),
        }
    }
}
impl ::treesitter_types::Spanned for InitializerDeclarationChildren<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        match self {
            Self::ArrayType(inner) => inner.span(),
            Self::Boolean(inner) => inner.span(),
            Self::BreakStatement(inner) => inner.span(),
            Self::BuildConfigurationStatement(inner) => inner.span(),
            Self::ClassDeclaration(inner) => inner.span(),
            Self::ConstantDeclaration(inner) => inner.span(),
            Self::ContinueStatement(inner) => inner.span(),
            Self::DeferStatement(inner) => inner.span(),
            Self::DeinitializerDeclaration(inner) => inner.span(),
            Self::DiagnosticStatement(inner) => inner.span(),
            Self::DictionaryType(inner) => inner.span(),
            Self::DoStatement(inner) => inner.span(),
            Self::EnumDeclaration(inner) => inner.span(),
            Self::ExtensionDeclaration(inner) => inner.span(),
            Self::FallthroughStatement(inner) => inner.span(),
            Self::ForStatement(inner) => inner.span(),
            Self::FunctionDeclaration(inner) => inner.span(),
            Self::GuardStatement(inner) => inner.span(),
            Self::Identifier(inner) => inner.span(),
            Self::IfStatement(inner) => inner.span(),
            Self::ImportDeclaration(inner) => inner.span(),
            Self::InitializerDeclaration(inner) => inner.span(),
            Self::LabeledStatement(inner) => inner.span(),
            Self::LineControlStatement(inner) => inner.span(),
            Self::Nil(inner) => inner.span(),
            Self::Number(inner) => inner.span(),
            Self::OperatorDeclaration(inner) => inner.span(),
            Self::ProtocolDeclaration(inner) => inner.span(),
            Self::RepeatWhileStatement(inner) => inner.span(),
            Self::ReturnStatement(inner) => inner.span(),
            Self::String(inner) => inner.span(),
            Self::StructDeclaration(inner) => inner.span(),
            Self::SubscriptDeclaration(inner) => inner.span(),
            Self::SwitchStatement(inner) => inner.span(),
            Self::ThrowStatement(inner) => inner.span(),
            Self::Type(inner) => inner.span(),
            Self::TypealiasDeclaration(inner) => inner.span(),
            Self::VariableDeclaration(inner) => inner.span(),
            Self::WhileStatement(inner) => inner.span(),
        }
    }
}
#[derive(Debug, Clone)]
pub enum LabeledStatementChildren<'tree> {
    ForStatement(::std::boxed::Box<ForStatement<'tree>>),
    Identifier(::std::boxed::Box<Identifier<'tree>>),
    IfStatement(::std::boxed::Box<IfStatement<'tree>>),
    RepeatWhileStatement(::std::boxed::Box<RepeatWhileStatement<'tree>>),
    WhileStatement(::std::boxed::Box<WhileStatement<'tree>>),
}
impl<'tree> ::treesitter_types::FromNode<'tree> for LabeledStatementChildren<'tree> {
    #[allow(clippy::collapsible_else_if)]
    fn from_node(
        node: ::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        match node.kind() {
            "for_statement" => Ok(Self::ForStatement(::std::boxed::Box::new(
                <ForStatement as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "identifier" => Ok(Self::Identifier(::std::boxed::Box::new(
                <Identifier as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "if_statement" => Ok(Self::IfStatement(::std::boxed::Box::new(
                <IfStatement as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "repeat_while_statement" => Ok(Self::RepeatWhileStatement(::std::boxed::Box::new(
                <RepeatWhileStatement as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "while_statement" => Ok(Self::WhileStatement(::std::boxed::Box::new(
                <WhileStatement as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            other => Err(::treesitter_types::ParseError::unexpected_kind(other, node)),
        }
    }
}
impl ::treesitter_types::Spanned for LabeledStatementChildren<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        match self {
            Self::ForStatement(inner) => inner.span(),
            Self::Identifier(inner) => inner.span(),
            Self::IfStatement(inner) => inner.span(),
            Self::RepeatWhileStatement(inner) => inner.span(),
            Self::WhileStatement(inner) => inner.span(),
        }
    }
}
#[derive(Debug, Clone)]
pub enum OperatorDeclarationChildren<'tree> {
    AssociativityClause(::std::boxed::Box<AssociativityClause<'tree>>),
    Operator(::std::boxed::Box<Operator<'tree>>),
    PrecedenceClause(::std::boxed::Box<PrecedenceClause<'tree>>),
}
impl<'tree> ::treesitter_types::FromNode<'tree> for OperatorDeclarationChildren<'tree> {
    #[allow(clippy::collapsible_else_if)]
    fn from_node(
        node: ::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        match node.kind() {
            "associativity_clause" => Ok(Self::AssociativityClause(::std::boxed::Box::new(
                <AssociativityClause as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "operator" => Ok(Self::Operator(::std::boxed::Box::new(
                <Operator as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "precedence_clause" => Ok(Self::PrecedenceClause(::std::boxed::Box::new(
                <PrecedenceClause as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            other => Err(::treesitter_types::ParseError::unexpected_kind(other, node)),
        }
    }
}
impl ::treesitter_types::Spanned for OperatorDeclarationChildren<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        match self {
            Self::AssociativityClause(inner) => inner.span(),
            Self::Operator(inner) => inner.span(),
            Self::PrecedenceClause(inner) => inner.span(),
        }
    }
}
#[derive(Debug, Clone)]
pub enum OptionalBindingChildren<'tree> {
    AsPattern(::std::boxed::Box<AsPattern<'tree>>),
    Boolean(::std::boxed::Box<Boolean<'tree>>),
    EnumCasePattern(::std::boxed::Box<EnumCasePattern<'tree>>),
    Identifier(::std::boxed::Box<Identifier<'tree>>),
    IsPattern(::std::boxed::Box<IsPattern<'tree>>),
    Nil(::std::boxed::Box<Nil<'tree>>),
    Number(::std::boxed::Box<Number<'tree>>),
    OptionalPattern(::std::boxed::Box<OptionalPattern<'tree>>),
    String(::std::boxed::Box<String<'tree>>),
    ValueBindingPattern(::std::boxed::Box<ValueBindingPattern<'tree>>),
    WildcardPattern(::std::boxed::Box<WildcardPattern<'tree>>),
}
impl<'tree> ::treesitter_types::FromNode<'tree> for OptionalBindingChildren<'tree> {
    #[allow(clippy::collapsible_else_if)]
    fn from_node(
        node: ::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        match node.kind() {
            "as_pattern" => Ok(Self::AsPattern(::std::boxed::Box::new(
                <AsPattern as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "boolean" => Ok(Self::Boolean(::std::boxed::Box::new(
                <Boolean as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "enum_case_pattern" => Ok(Self::EnumCasePattern(::std::boxed::Box::new(
                <EnumCasePattern as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "identifier" => Ok(Self::Identifier(::std::boxed::Box::new(
                <Identifier as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "is_pattern" => Ok(Self::IsPattern(::std::boxed::Box::new(
                <IsPattern as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "nil" => Ok(Self::Nil(::std::boxed::Box::new(
                <Nil as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "number" => Ok(Self::Number(::std::boxed::Box::new(
                <Number as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "optional_pattern" => Ok(Self::OptionalPattern(::std::boxed::Box::new(
                <OptionalPattern as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "string" => Ok(Self::String(::std::boxed::Box::new(
                <String as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "value_binding_pattern" => Ok(Self::ValueBindingPattern(::std::boxed::Box::new(
                <ValueBindingPattern as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "wildcard_pattern" => Ok(Self::WildcardPattern(::std::boxed::Box::new(
                <WildcardPattern as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            other => Err(::treesitter_types::ParseError::unexpected_kind(other, node)),
        }
    }
}
impl ::treesitter_types::Spanned for OptionalBindingChildren<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        match self {
            Self::AsPattern(inner) => inner.span(),
            Self::Boolean(inner) => inner.span(),
            Self::EnumCasePattern(inner) => inner.span(),
            Self::Identifier(inner) => inner.span(),
            Self::IsPattern(inner) => inner.span(),
            Self::Nil(inner) => inner.span(),
            Self::Number(inner) => inner.span(),
            Self::OptionalPattern(inner) => inner.span(),
            Self::String(inner) => inner.span(),
            Self::ValueBindingPattern(inner) => inner.span(),
            Self::WildcardPattern(inner) => inner.span(),
        }
    }
}
#[derive(Debug, Clone)]
pub enum OptionalPatternChildren<'tree> {
    AsPattern(::std::boxed::Box<AsPattern<'tree>>),
    Boolean(::std::boxed::Box<Boolean<'tree>>),
    EnumCasePattern(::std::boxed::Box<EnumCasePattern<'tree>>),
    Identifier(::std::boxed::Box<Identifier<'tree>>),
    IsPattern(::std::boxed::Box<IsPattern<'tree>>),
    Nil(::std::boxed::Box<Nil<'tree>>),
    Number(::std::boxed::Box<Number<'tree>>),
    OptionalPattern(::std::boxed::Box<OptionalPattern<'tree>>),
    String(::std::boxed::Box<String<'tree>>),
    ValueBindingPattern(::std::boxed::Box<ValueBindingPattern<'tree>>),
    WildcardPattern(::std::boxed::Box<WildcardPattern<'tree>>),
}
impl<'tree> ::treesitter_types::FromNode<'tree> for OptionalPatternChildren<'tree> {
    #[allow(clippy::collapsible_else_if)]
    fn from_node(
        node: ::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        match node.kind() {
            "as_pattern" => Ok(Self::AsPattern(::std::boxed::Box::new(
                <AsPattern as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "boolean" => Ok(Self::Boolean(::std::boxed::Box::new(
                <Boolean as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "enum_case_pattern" => Ok(Self::EnumCasePattern(::std::boxed::Box::new(
                <EnumCasePattern as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "identifier" => Ok(Self::Identifier(::std::boxed::Box::new(
                <Identifier as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "is_pattern" => Ok(Self::IsPattern(::std::boxed::Box::new(
                <IsPattern as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "nil" => Ok(Self::Nil(::std::boxed::Box::new(
                <Nil as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "number" => Ok(Self::Number(::std::boxed::Box::new(
                <Number as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "optional_pattern" => Ok(Self::OptionalPattern(::std::boxed::Box::new(
                <OptionalPattern as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "string" => Ok(Self::String(::std::boxed::Box::new(
                <String as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "value_binding_pattern" => Ok(Self::ValueBindingPattern(::std::boxed::Box::new(
                <ValueBindingPattern as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "wildcard_pattern" => Ok(Self::WildcardPattern(::std::boxed::Box::new(
                <WildcardPattern as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            other => Err(::treesitter_types::ParseError::unexpected_kind(other, node)),
        }
    }
}
impl ::treesitter_types::Spanned for OptionalPatternChildren<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        match self {
            Self::AsPattern(inner) => inner.span(),
            Self::Boolean(inner) => inner.span(),
            Self::EnumCasePattern(inner) => inner.span(),
            Self::Identifier(inner) => inner.span(),
            Self::IsPattern(inner) => inner.span(),
            Self::Nil(inner) => inner.span(),
            Self::Number(inner) => inner.span(),
            Self::OptionalPattern(inner) => inner.span(),
            Self::String(inner) => inner.span(),
            Self::ValueBindingPattern(inner) => inner.span(),
            Self::WildcardPattern(inner) => inner.span(),
        }
    }
}
#[derive(Debug, Clone)]
pub enum ProgramChildren<'tree> {
    Boolean(::std::boxed::Box<Boolean<'tree>>),
    BreakStatement(::std::boxed::Box<BreakStatement<'tree>>),
    BuildConfigurationStatement(::std::boxed::Box<BuildConfigurationStatement<'tree>>),
    ClassDeclaration(::std::boxed::Box<ClassDeclaration<'tree>>),
    ConstantDeclaration(::std::boxed::Box<ConstantDeclaration<'tree>>),
    ContinueStatement(::std::boxed::Box<ContinueStatement<'tree>>),
    DeferStatement(::std::boxed::Box<DeferStatement<'tree>>),
    DeinitializerDeclaration(::std::boxed::Box<DeinitializerDeclaration<'tree>>),
    DiagnosticStatement(::std::boxed::Box<DiagnosticStatement<'tree>>),
    DoStatement(::std::boxed::Box<DoStatement<'tree>>),
    EnumDeclaration(::std::boxed::Box<EnumDeclaration<'tree>>),
    ExtensionDeclaration(::std::boxed::Box<ExtensionDeclaration<'tree>>),
    FallthroughStatement(::std::boxed::Box<FallthroughStatement<'tree>>),
    ForStatement(::std::boxed::Box<ForStatement<'tree>>),
    FunctionDeclaration(::std::boxed::Box<FunctionDeclaration<'tree>>),
    GuardStatement(::std::boxed::Box<GuardStatement<'tree>>),
    Identifier(::std::boxed::Box<Identifier<'tree>>),
    IfStatement(::std::boxed::Box<IfStatement<'tree>>),
    ImportDeclaration(::std::boxed::Box<ImportDeclaration<'tree>>),
    InitializerDeclaration(::std::boxed::Box<InitializerDeclaration<'tree>>),
    LabeledStatement(::std::boxed::Box<LabeledStatement<'tree>>),
    LineControlStatement(::std::boxed::Box<LineControlStatement<'tree>>),
    Nil(::std::boxed::Box<Nil<'tree>>),
    Number(::std::boxed::Box<Number<'tree>>),
    OperatorDeclaration(::std::boxed::Box<OperatorDeclaration<'tree>>),
    ProtocolDeclaration(::std::boxed::Box<ProtocolDeclaration<'tree>>),
    RepeatWhileStatement(::std::boxed::Box<RepeatWhileStatement<'tree>>),
    ReturnStatement(::std::boxed::Box<ReturnStatement<'tree>>),
    String(::std::boxed::Box<String<'tree>>),
    StructDeclaration(::std::boxed::Box<StructDeclaration<'tree>>),
    SubscriptDeclaration(::std::boxed::Box<SubscriptDeclaration<'tree>>),
    SwitchStatement(::std::boxed::Box<SwitchStatement<'tree>>),
    ThrowStatement(::std::boxed::Box<ThrowStatement<'tree>>),
    TypealiasDeclaration(::std::boxed::Box<TypealiasDeclaration<'tree>>),
    VariableDeclaration(::std::boxed::Box<VariableDeclaration<'tree>>),
    WhileStatement(::std::boxed::Box<WhileStatement<'tree>>),
}
impl<'tree> ::treesitter_types::FromNode<'tree> for ProgramChildren<'tree> {
    #[allow(clippy::collapsible_else_if)]
    fn from_node(
        node: ::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        match node.kind() {
            "boolean" => Ok(Self::Boolean(::std::boxed::Box::new(
                <Boolean as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "break_statement" => Ok(Self::BreakStatement(::std::boxed::Box::new(
                <BreakStatement as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "build_configuration_statement" => {
                Ok(Self::BuildConfigurationStatement(::std::boxed::Box::new(
                    <BuildConfigurationStatement as ::treesitter_types::FromNode>::from_node(
                        node, src,
                    )?,
                )))
            }
            "class_declaration" => Ok(Self::ClassDeclaration(::std::boxed::Box::new(
                <ClassDeclaration as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "constant_declaration" => Ok(Self::ConstantDeclaration(::std::boxed::Box::new(
                <ConstantDeclaration as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "continue_statement" => Ok(Self::ContinueStatement(::std::boxed::Box::new(
                <ContinueStatement as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "defer_statement" => Ok(Self::DeferStatement(::std::boxed::Box::new(
                <DeferStatement as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "deinitializer_declaration" => {
                Ok(Self::DeinitializerDeclaration(::std::boxed::Box::new(
                    <DeinitializerDeclaration as ::treesitter_types::FromNode>::from_node(
                        node, src,
                    )?,
                )))
            }
            "diagnostic_statement" => Ok(Self::DiagnosticStatement(::std::boxed::Box::new(
                <DiagnosticStatement as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "do_statement" => Ok(Self::DoStatement(::std::boxed::Box::new(
                <DoStatement as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "enum_declaration" => Ok(Self::EnumDeclaration(::std::boxed::Box::new(
                <EnumDeclaration as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "extension_declaration" => Ok(Self::ExtensionDeclaration(::std::boxed::Box::new(
                <ExtensionDeclaration as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "fallthrough_statement" => Ok(Self::FallthroughStatement(::std::boxed::Box::new(
                <FallthroughStatement as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "for_statement" => Ok(Self::ForStatement(::std::boxed::Box::new(
                <ForStatement as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "function_declaration" => Ok(Self::FunctionDeclaration(::std::boxed::Box::new(
                <FunctionDeclaration as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "guard_statement" => Ok(Self::GuardStatement(::std::boxed::Box::new(
                <GuardStatement as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "identifier" => Ok(Self::Identifier(::std::boxed::Box::new(
                <Identifier as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "if_statement" => Ok(Self::IfStatement(::std::boxed::Box::new(
                <IfStatement as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "import_declaration" => Ok(Self::ImportDeclaration(::std::boxed::Box::new(
                <ImportDeclaration as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "initializer_declaration" => Ok(Self::InitializerDeclaration(::std::boxed::Box::new(
                <InitializerDeclaration as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "labeled_statement" => Ok(Self::LabeledStatement(::std::boxed::Box::new(
                <LabeledStatement as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "line_control_statement" => Ok(Self::LineControlStatement(::std::boxed::Box::new(
                <LineControlStatement as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "nil" => Ok(Self::Nil(::std::boxed::Box::new(
                <Nil as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "number" => Ok(Self::Number(::std::boxed::Box::new(
                <Number as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "operator_declaration" => Ok(Self::OperatorDeclaration(::std::boxed::Box::new(
                <OperatorDeclaration as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "protocol_declaration" => Ok(Self::ProtocolDeclaration(::std::boxed::Box::new(
                <ProtocolDeclaration as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "repeat_while_statement" => Ok(Self::RepeatWhileStatement(::std::boxed::Box::new(
                <RepeatWhileStatement as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "return_statement" => Ok(Self::ReturnStatement(::std::boxed::Box::new(
                <ReturnStatement as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "string" => Ok(Self::String(::std::boxed::Box::new(
                <String as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "struct_declaration" => Ok(Self::StructDeclaration(::std::boxed::Box::new(
                <StructDeclaration as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "subscript_declaration" => Ok(Self::SubscriptDeclaration(::std::boxed::Box::new(
                <SubscriptDeclaration as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "switch_statement" => Ok(Self::SwitchStatement(::std::boxed::Box::new(
                <SwitchStatement as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "throw_statement" => Ok(Self::ThrowStatement(::std::boxed::Box::new(
                <ThrowStatement as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "typealias_declaration" => Ok(Self::TypealiasDeclaration(::std::boxed::Box::new(
                <TypealiasDeclaration as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "variable_declaration" => Ok(Self::VariableDeclaration(::std::boxed::Box::new(
                <VariableDeclaration as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "while_statement" => Ok(Self::WhileStatement(::std::boxed::Box::new(
                <WhileStatement as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            other => Err(::treesitter_types::ParseError::unexpected_kind(other, node)),
        }
    }
}
impl ::treesitter_types::Spanned for ProgramChildren<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        match self {
            Self::Boolean(inner) => inner.span(),
            Self::BreakStatement(inner) => inner.span(),
            Self::BuildConfigurationStatement(inner) => inner.span(),
            Self::ClassDeclaration(inner) => inner.span(),
            Self::ConstantDeclaration(inner) => inner.span(),
            Self::ContinueStatement(inner) => inner.span(),
            Self::DeferStatement(inner) => inner.span(),
            Self::DeinitializerDeclaration(inner) => inner.span(),
            Self::DiagnosticStatement(inner) => inner.span(),
            Self::DoStatement(inner) => inner.span(),
            Self::EnumDeclaration(inner) => inner.span(),
            Self::ExtensionDeclaration(inner) => inner.span(),
            Self::FallthroughStatement(inner) => inner.span(),
            Self::ForStatement(inner) => inner.span(),
            Self::FunctionDeclaration(inner) => inner.span(),
            Self::GuardStatement(inner) => inner.span(),
            Self::Identifier(inner) => inner.span(),
            Self::IfStatement(inner) => inner.span(),
            Self::ImportDeclaration(inner) => inner.span(),
            Self::InitializerDeclaration(inner) => inner.span(),
            Self::LabeledStatement(inner) => inner.span(),
            Self::LineControlStatement(inner) => inner.span(),
            Self::Nil(inner) => inner.span(),
            Self::Number(inner) => inner.span(),
            Self::OperatorDeclaration(inner) => inner.span(),
            Self::ProtocolDeclaration(inner) => inner.span(),
            Self::RepeatWhileStatement(inner) => inner.span(),
            Self::ReturnStatement(inner) => inner.span(),
            Self::String(inner) => inner.span(),
            Self::StructDeclaration(inner) => inner.span(),
            Self::SubscriptDeclaration(inner) => inner.span(),
            Self::SwitchStatement(inner) => inner.span(),
            Self::ThrowStatement(inner) => inner.span(),
            Self::TypealiasDeclaration(inner) => inner.span(),
            Self::VariableDeclaration(inner) => inner.span(),
            Self::WhileStatement(inner) => inner.span(),
        }
    }
}
#[derive(Debug, Clone)]
pub enum ProtocolDeclarationChildren<'tree> {
    AssociatedtypeDeclaration(::std::boxed::Box<AssociatedtypeDeclaration<'tree>>),
    Identifier(::std::boxed::Box<Identifier<'tree>>),
    Modifier(::std::boxed::Box<Modifier<'tree>>),
    ProtocolInitializerDeclaration(::std::boxed::Box<ProtocolInitializerDeclaration<'tree>>),
    ProtocolMethodDeclaration(::std::boxed::Box<ProtocolMethodDeclaration<'tree>>),
    ProtocolSubscriptDeclaration(::std::boxed::Box<ProtocolSubscriptDeclaration<'tree>>),
    ProtocolTypealiasDeclaration(::std::boxed::Box<ProtocolTypealiasDeclaration<'tree>>),
    ProtocolVariableDeclaration(::std::boxed::Box<ProtocolVariableDeclaration<'tree>>),
}
impl<'tree> ::treesitter_types::FromNode<'tree> for ProtocolDeclarationChildren<'tree> {
    #[allow(clippy::collapsible_else_if)]
    fn from_node(
        node: ::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        match node.kind() {
            "associatedtype_declaration" => {
                Ok(Self::AssociatedtypeDeclaration(::std::boxed::Box::new(
                    <AssociatedtypeDeclaration as ::treesitter_types::FromNode>::from_node(
                        node, src,
                    )?,
                )))
            }
            "identifier" => Ok(Self::Identifier(::std::boxed::Box::new(
                <Identifier as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "modifier" => Ok(Self::Modifier(::std::boxed::Box::new(
                <Modifier as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "protocol_initializer_declaration" => Ok(Self::ProtocolInitializerDeclaration(
                ::std::boxed::Box::new(
                    <ProtocolInitializerDeclaration as ::treesitter_types::FromNode>::from_node(
                        node, src,
                    )?,
                ),
            )),
            "protocol_method_declaration" => {
                Ok(Self::ProtocolMethodDeclaration(::std::boxed::Box::new(
                    <ProtocolMethodDeclaration as ::treesitter_types::FromNode>::from_node(
                        node, src,
                    )?,
                )))
            }
            "protocol_subscript_declaration" => {
                Ok(Self::ProtocolSubscriptDeclaration(::std::boxed::Box::new(
                    <ProtocolSubscriptDeclaration as ::treesitter_types::FromNode>::from_node(
                        node, src,
                    )?,
                )))
            }
            "protocol_typealias_declaration" => {
                Ok(Self::ProtocolTypealiasDeclaration(::std::boxed::Box::new(
                    <ProtocolTypealiasDeclaration as ::treesitter_types::FromNode>::from_node(
                        node, src,
                    )?,
                )))
            }
            "protocol_variable_declaration" => {
                Ok(Self::ProtocolVariableDeclaration(::std::boxed::Box::new(
                    <ProtocolVariableDeclaration as ::treesitter_types::FromNode>::from_node(
                        node, src,
                    )?,
                )))
            }
            other => Err(::treesitter_types::ParseError::unexpected_kind(other, node)),
        }
    }
}
impl ::treesitter_types::Spanned for ProtocolDeclarationChildren<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        match self {
            Self::AssociatedtypeDeclaration(inner) => inner.span(),
            Self::Identifier(inner) => inner.span(),
            Self::Modifier(inner) => inner.span(),
            Self::ProtocolInitializerDeclaration(inner) => inner.span(),
            Self::ProtocolMethodDeclaration(inner) => inner.span(),
            Self::ProtocolSubscriptDeclaration(inner) => inner.span(),
            Self::ProtocolTypealiasDeclaration(inner) => inner.span(),
            Self::ProtocolVariableDeclaration(inner) => inner.span(),
        }
    }
}
#[derive(Debug, Clone)]
pub enum ProtocolInitializerDeclarationChildren<'tree> {
    ArrayType(::std::boxed::Box<ArrayType<'tree>>),
    Boolean(::std::boxed::Box<Boolean<'tree>>),
    DictionaryType(::std::boxed::Box<DictionaryType<'tree>>),
    Identifier(::std::boxed::Box<Identifier<'tree>>),
    Nil(::std::boxed::Box<Nil<'tree>>),
    Number(::std::boxed::Box<Number<'tree>>),
    String(::std::boxed::Box<String<'tree>>),
    Type(::std::boxed::Box<Type<'tree>>),
}
impl<'tree> ::treesitter_types::FromNode<'tree> for ProtocolInitializerDeclarationChildren<'tree> {
    #[allow(clippy::collapsible_else_if)]
    fn from_node(
        node: ::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        match node.kind() {
            "array_type" => Ok(Self::ArrayType(::std::boxed::Box::new(
                <ArrayType as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "boolean" => Ok(Self::Boolean(::std::boxed::Box::new(
                <Boolean as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "dictionary_type" => Ok(Self::DictionaryType(::std::boxed::Box::new(
                <DictionaryType as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "identifier" => Ok(Self::Identifier(::std::boxed::Box::new(
                <Identifier as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "nil" => Ok(Self::Nil(::std::boxed::Box::new(
                <Nil as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "number" => Ok(Self::Number(::std::boxed::Box::new(
                <Number as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "string" => Ok(Self::String(::std::boxed::Box::new(
                <String as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "type" => Ok(Self::Type(::std::boxed::Box::new(
                <Type as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            other => Err(::treesitter_types::ParseError::unexpected_kind(other, node)),
        }
    }
}
impl ::treesitter_types::Spanned for ProtocolInitializerDeclarationChildren<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        match self {
            Self::ArrayType(inner) => inner.span(),
            Self::Boolean(inner) => inner.span(),
            Self::DictionaryType(inner) => inner.span(),
            Self::Identifier(inner) => inner.span(),
            Self::Nil(inner) => inner.span(),
            Self::Number(inner) => inner.span(),
            Self::String(inner) => inner.span(),
            Self::Type(inner) => inner.span(),
        }
    }
}
#[derive(Debug, Clone)]
pub enum ProtocolMethodDeclarationName<'tree> {
    Identifier(::std::boxed::Box<Identifier<'tree>>),
    Operator(::std::boxed::Box<Operator<'tree>>),
}
impl<'tree> ::treesitter_types::FromNode<'tree> for ProtocolMethodDeclarationName<'tree> {
    #[allow(clippy::collapsible_else_if)]
    fn from_node(
        node: ::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        match node.kind() {
            "identifier" => Ok(Self::Identifier(::std::boxed::Box::new(
                <Identifier as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "operator" => Ok(Self::Operator(::std::boxed::Box::new(
                <Operator as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            other => Err(::treesitter_types::ParseError::unexpected_kind(other, node)),
        }
    }
}
impl ::treesitter_types::Spanned for ProtocolMethodDeclarationName<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        match self {
            Self::Identifier(inner) => inner.span(),
            Self::Operator(inner) => inner.span(),
        }
    }
}
#[derive(Debug, Clone)]
pub enum ProtocolMethodDeclarationReturn<'tree> {
    Bang(::treesitter_types::Span),
    Question(::treesitter_types::Span),
    Type(::std::boxed::Box<Type<'tree>>),
}
impl<'tree> ::treesitter_types::FromNode<'tree> for ProtocolMethodDeclarationReturn<'tree> {
    #[allow(clippy::collapsible_else_if)]
    fn from_node(
        node: ::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        match node.kind() {
            "!" => Ok(Self::Bang(::treesitter_types::Span::from(node))),
            "?" => Ok(Self::Question(::treesitter_types::Span::from(node))),
            "type" => Ok(Self::Type(::std::boxed::Box::new(
                <Type as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            other => Err(::treesitter_types::ParseError::unexpected_kind(other, node)),
        }
    }
}
impl ::treesitter_types::Spanned for ProtocolMethodDeclarationReturn<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        match self {
            Self::Bang(span) => *span,
            Self::Question(span) => *span,
            Self::Type(inner) => inner.span(),
        }
    }
}
#[derive(Debug, Clone)]
pub enum ProtocolMethodDeclarationChildren<'tree> {
    GenericClause(::std::boxed::Box<GenericClause<'tree>>),
    Modifier(::std::boxed::Box<Modifier<'tree>>),
    ParameterList(::std::boxed::Box<ParameterList<'tree>>),
}
impl<'tree> ::treesitter_types::FromNode<'tree> for ProtocolMethodDeclarationChildren<'tree> {
    #[allow(clippy::collapsible_else_if)]
    fn from_node(
        node: ::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        match node.kind() {
            "generic_clause" => Ok(Self::GenericClause(::std::boxed::Box::new(
                <GenericClause as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "modifier" => Ok(Self::Modifier(::std::boxed::Box::new(
                <Modifier as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "parameter_list" => Ok(Self::ParameterList(::std::boxed::Box::new(
                <ParameterList as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            other => Err(::treesitter_types::ParseError::unexpected_kind(other, node)),
        }
    }
}
impl ::treesitter_types::Spanned for ProtocolMethodDeclarationChildren<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        match self {
            Self::GenericClause(inner) => inner.span(),
            Self::Modifier(inner) => inner.span(),
            Self::ParameterList(inner) => inner.span(),
        }
    }
}
#[derive(Debug, Clone)]
pub enum ProtocolSubscriptDeclarationChildren<'tree> {
    ArrayType(::std::boxed::Box<ArrayType<'tree>>),
    Boolean(::std::boxed::Box<Boolean<'tree>>),
    DictionaryType(::std::boxed::Box<DictionaryType<'tree>>),
    Identifier(::std::boxed::Box<Identifier<'tree>>),
    Nil(::std::boxed::Box<Nil<'tree>>),
    Number(::std::boxed::Box<Number<'tree>>),
    String(::std::boxed::Box<String<'tree>>),
    Type(::std::boxed::Box<Type<'tree>>),
}
impl<'tree> ::treesitter_types::FromNode<'tree> for ProtocolSubscriptDeclarationChildren<'tree> {
    #[allow(clippy::collapsible_else_if)]
    fn from_node(
        node: ::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        match node.kind() {
            "array_type" => Ok(Self::ArrayType(::std::boxed::Box::new(
                <ArrayType as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "boolean" => Ok(Self::Boolean(::std::boxed::Box::new(
                <Boolean as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "dictionary_type" => Ok(Self::DictionaryType(::std::boxed::Box::new(
                <DictionaryType as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "identifier" => Ok(Self::Identifier(::std::boxed::Box::new(
                <Identifier as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "nil" => Ok(Self::Nil(::std::boxed::Box::new(
                <Nil as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "number" => Ok(Self::Number(::std::boxed::Box::new(
                <Number as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "string" => Ok(Self::String(::std::boxed::Box::new(
                <String as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "type" => Ok(Self::Type(::std::boxed::Box::new(
                <Type as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            other => Err(::treesitter_types::ParseError::unexpected_kind(other, node)),
        }
    }
}
impl ::treesitter_types::Spanned for ProtocolSubscriptDeclarationChildren<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        match self {
            Self::ArrayType(inner) => inner.span(),
            Self::Boolean(inner) => inner.span(),
            Self::DictionaryType(inner) => inner.span(),
            Self::Identifier(inner) => inner.span(),
            Self::Nil(inner) => inner.span(),
            Self::Number(inner) => inner.span(),
            Self::String(inner) => inner.span(),
            Self::Type(inner) => inner.span(),
        }
    }
}
#[derive(Debug, Clone)]
pub enum ProtocolTypealiasDeclarationChildren<'tree> {
    Identifier(::std::boxed::Box<Identifier<'tree>>),
    Modifier(::std::boxed::Box<Modifier<'tree>>),
    Type(::std::boxed::Box<Type<'tree>>),
}
impl<'tree> ::treesitter_types::FromNode<'tree> for ProtocolTypealiasDeclarationChildren<'tree> {
    #[allow(clippy::collapsible_else_if)]
    fn from_node(
        node: ::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        match node.kind() {
            "identifier" => Ok(Self::Identifier(::std::boxed::Box::new(
                <Identifier as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "modifier" => Ok(Self::Modifier(::std::boxed::Box::new(
                <Modifier as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "type" => Ok(Self::Type(::std::boxed::Box::new(
                <Type as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            other => Err(::treesitter_types::ParseError::unexpected_kind(other, node)),
        }
    }
}
impl ::treesitter_types::Spanned for ProtocolTypealiasDeclarationChildren<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        match self {
            Self::Identifier(inner) => inner.span(),
            Self::Modifier(inner) => inner.span(),
            Self::Type(inner) => inner.span(),
        }
    }
}
#[derive(Debug, Clone)]
pub enum ProtocolVariableDeclarationChildren<'tree> {
    ArrayType(::std::boxed::Box<ArrayType<'tree>>),
    DictionaryType(::std::boxed::Box<DictionaryType<'tree>>),
    Identifier(::std::boxed::Box<Identifier<'tree>>),
    Modifier(::std::boxed::Box<Modifier<'tree>>),
    Type(::std::boxed::Box<Type<'tree>>),
}
impl<'tree> ::treesitter_types::FromNode<'tree> for ProtocolVariableDeclarationChildren<'tree> {
    #[allow(clippy::collapsible_else_if)]
    fn from_node(
        node: ::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        match node.kind() {
            "array_type" => Ok(Self::ArrayType(::std::boxed::Box::new(
                <ArrayType as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "dictionary_type" => Ok(Self::DictionaryType(::std::boxed::Box::new(
                <DictionaryType as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "identifier" => Ok(Self::Identifier(::std::boxed::Box::new(
                <Identifier as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "modifier" => Ok(Self::Modifier(::std::boxed::Box::new(
                <Modifier as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "type" => Ok(Self::Type(::std::boxed::Box::new(
                <Type as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            other => Err(::treesitter_types::ParseError::unexpected_kind(other, node)),
        }
    }
}
impl ::treesitter_types::Spanned for ProtocolVariableDeclarationChildren<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        match self {
            Self::ArrayType(inner) => inner.span(),
            Self::DictionaryType(inner) => inner.span(),
            Self::Identifier(inner) => inner.span(),
            Self::Modifier(inner) => inner.span(),
            Self::Type(inner) => inner.span(),
        }
    }
}
#[derive(Debug, Clone)]
pub enum RepeatWhileStatementChildren<'tree> {
    Boolean(::std::boxed::Box<Boolean<'tree>>),
    BreakStatement(::std::boxed::Box<BreakStatement<'tree>>),
    BuildConfigurationStatement(::std::boxed::Box<BuildConfigurationStatement<'tree>>),
    ClassDeclaration(::std::boxed::Box<ClassDeclaration<'tree>>),
    ConstantDeclaration(::std::boxed::Box<ConstantDeclaration<'tree>>),
    ContinueStatement(::std::boxed::Box<ContinueStatement<'tree>>),
    DeferStatement(::std::boxed::Box<DeferStatement<'tree>>),
    DeinitializerDeclaration(::std::boxed::Box<DeinitializerDeclaration<'tree>>),
    DiagnosticStatement(::std::boxed::Box<DiagnosticStatement<'tree>>),
    DoStatement(::std::boxed::Box<DoStatement<'tree>>),
    EnumDeclaration(::std::boxed::Box<EnumDeclaration<'tree>>),
    ExtensionDeclaration(::std::boxed::Box<ExtensionDeclaration<'tree>>),
    FallthroughStatement(::std::boxed::Box<FallthroughStatement<'tree>>),
    ForStatement(::std::boxed::Box<ForStatement<'tree>>),
    FunctionDeclaration(::std::boxed::Box<FunctionDeclaration<'tree>>),
    GuardStatement(::std::boxed::Box<GuardStatement<'tree>>),
    Identifier(::std::boxed::Box<Identifier<'tree>>),
    IfStatement(::std::boxed::Box<IfStatement<'tree>>),
    ImportDeclaration(::std::boxed::Box<ImportDeclaration<'tree>>),
    InitializerDeclaration(::std::boxed::Box<InitializerDeclaration<'tree>>),
    LabeledStatement(::std::boxed::Box<LabeledStatement<'tree>>),
    LineControlStatement(::std::boxed::Box<LineControlStatement<'tree>>),
    Nil(::std::boxed::Box<Nil<'tree>>),
    Number(::std::boxed::Box<Number<'tree>>),
    OperatorDeclaration(::std::boxed::Box<OperatorDeclaration<'tree>>),
    ProtocolDeclaration(::std::boxed::Box<ProtocolDeclaration<'tree>>),
    RepeatWhileStatement(::std::boxed::Box<RepeatWhileStatement<'tree>>),
    ReturnStatement(::std::boxed::Box<ReturnStatement<'tree>>),
    String(::std::boxed::Box<String<'tree>>),
    StructDeclaration(::std::boxed::Box<StructDeclaration<'tree>>),
    SubscriptDeclaration(::std::boxed::Box<SubscriptDeclaration<'tree>>),
    SwitchStatement(::std::boxed::Box<SwitchStatement<'tree>>),
    ThrowStatement(::std::boxed::Box<ThrowStatement<'tree>>),
    TypealiasDeclaration(::std::boxed::Box<TypealiasDeclaration<'tree>>),
    VariableDeclaration(::std::boxed::Box<VariableDeclaration<'tree>>),
    WhileStatement(::std::boxed::Box<WhileStatement<'tree>>),
}
impl<'tree> ::treesitter_types::FromNode<'tree> for RepeatWhileStatementChildren<'tree> {
    #[allow(clippy::collapsible_else_if)]
    fn from_node(
        node: ::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        match node.kind() {
            "boolean" => Ok(Self::Boolean(::std::boxed::Box::new(
                <Boolean as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "break_statement" => Ok(Self::BreakStatement(::std::boxed::Box::new(
                <BreakStatement as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "build_configuration_statement" => {
                Ok(Self::BuildConfigurationStatement(::std::boxed::Box::new(
                    <BuildConfigurationStatement as ::treesitter_types::FromNode>::from_node(
                        node, src,
                    )?,
                )))
            }
            "class_declaration" => Ok(Self::ClassDeclaration(::std::boxed::Box::new(
                <ClassDeclaration as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "constant_declaration" => Ok(Self::ConstantDeclaration(::std::boxed::Box::new(
                <ConstantDeclaration as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "continue_statement" => Ok(Self::ContinueStatement(::std::boxed::Box::new(
                <ContinueStatement as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "defer_statement" => Ok(Self::DeferStatement(::std::boxed::Box::new(
                <DeferStatement as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "deinitializer_declaration" => {
                Ok(Self::DeinitializerDeclaration(::std::boxed::Box::new(
                    <DeinitializerDeclaration as ::treesitter_types::FromNode>::from_node(
                        node, src,
                    )?,
                )))
            }
            "diagnostic_statement" => Ok(Self::DiagnosticStatement(::std::boxed::Box::new(
                <DiagnosticStatement as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "do_statement" => Ok(Self::DoStatement(::std::boxed::Box::new(
                <DoStatement as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "enum_declaration" => Ok(Self::EnumDeclaration(::std::boxed::Box::new(
                <EnumDeclaration as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "extension_declaration" => Ok(Self::ExtensionDeclaration(::std::boxed::Box::new(
                <ExtensionDeclaration as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "fallthrough_statement" => Ok(Self::FallthroughStatement(::std::boxed::Box::new(
                <FallthroughStatement as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "for_statement" => Ok(Self::ForStatement(::std::boxed::Box::new(
                <ForStatement as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "function_declaration" => Ok(Self::FunctionDeclaration(::std::boxed::Box::new(
                <FunctionDeclaration as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "guard_statement" => Ok(Self::GuardStatement(::std::boxed::Box::new(
                <GuardStatement as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "identifier" => Ok(Self::Identifier(::std::boxed::Box::new(
                <Identifier as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "if_statement" => Ok(Self::IfStatement(::std::boxed::Box::new(
                <IfStatement as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "import_declaration" => Ok(Self::ImportDeclaration(::std::boxed::Box::new(
                <ImportDeclaration as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "initializer_declaration" => Ok(Self::InitializerDeclaration(::std::boxed::Box::new(
                <InitializerDeclaration as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "labeled_statement" => Ok(Self::LabeledStatement(::std::boxed::Box::new(
                <LabeledStatement as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "line_control_statement" => Ok(Self::LineControlStatement(::std::boxed::Box::new(
                <LineControlStatement as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "nil" => Ok(Self::Nil(::std::boxed::Box::new(
                <Nil as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "number" => Ok(Self::Number(::std::boxed::Box::new(
                <Number as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "operator_declaration" => Ok(Self::OperatorDeclaration(::std::boxed::Box::new(
                <OperatorDeclaration as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "protocol_declaration" => Ok(Self::ProtocolDeclaration(::std::boxed::Box::new(
                <ProtocolDeclaration as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "repeat_while_statement" => Ok(Self::RepeatWhileStatement(::std::boxed::Box::new(
                <RepeatWhileStatement as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "return_statement" => Ok(Self::ReturnStatement(::std::boxed::Box::new(
                <ReturnStatement as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "string" => Ok(Self::String(::std::boxed::Box::new(
                <String as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "struct_declaration" => Ok(Self::StructDeclaration(::std::boxed::Box::new(
                <StructDeclaration as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "subscript_declaration" => Ok(Self::SubscriptDeclaration(::std::boxed::Box::new(
                <SubscriptDeclaration as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "switch_statement" => Ok(Self::SwitchStatement(::std::boxed::Box::new(
                <SwitchStatement as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "throw_statement" => Ok(Self::ThrowStatement(::std::boxed::Box::new(
                <ThrowStatement as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "typealias_declaration" => Ok(Self::TypealiasDeclaration(::std::boxed::Box::new(
                <TypealiasDeclaration as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "variable_declaration" => Ok(Self::VariableDeclaration(::std::boxed::Box::new(
                <VariableDeclaration as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "while_statement" => Ok(Self::WhileStatement(::std::boxed::Box::new(
                <WhileStatement as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            other => Err(::treesitter_types::ParseError::unexpected_kind(other, node)),
        }
    }
}
impl ::treesitter_types::Spanned for RepeatWhileStatementChildren<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        match self {
            Self::Boolean(inner) => inner.span(),
            Self::BreakStatement(inner) => inner.span(),
            Self::BuildConfigurationStatement(inner) => inner.span(),
            Self::ClassDeclaration(inner) => inner.span(),
            Self::ConstantDeclaration(inner) => inner.span(),
            Self::ContinueStatement(inner) => inner.span(),
            Self::DeferStatement(inner) => inner.span(),
            Self::DeinitializerDeclaration(inner) => inner.span(),
            Self::DiagnosticStatement(inner) => inner.span(),
            Self::DoStatement(inner) => inner.span(),
            Self::EnumDeclaration(inner) => inner.span(),
            Self::ExtensionDeclaration(inner) => inner.span(),
            Self::FallthroughStatement(inner) => inner.span(),
            Self::ForStatement(inner) => inner.span(),
            Self::FunctionDeclaration(inner) => inner.span(),
            Self::GuardStatement(inner) => inner.span(),
            Self::Identifier(inner) => inner.span(),
            Self::IfStatement(inner) => inner.span(),
            Self::ImportDeclaration(inner) => inner.span(),
            Self::InitializerDeclaration(inner) => inner.span(),
            Self::LabeledStatement(inner) => inner.span(),
            Self::LineControlStatement(inner) => inner.span(),
            Self::Nil(inner) => inner.span(),
            Self::Number(inner) => inner.span(),
            Self::OperatorDeclaration(inner) => inner.span(),
            Self::ProtocolDeclaration(inner) => inner.span(),
            Self::RepeatWhileStatement(inner) => inner.span(),
            Self::ReturnStatement(inner) => inner.span(),
            Self::String(inner) => inner.span(),
            Self::StructDeclaration(inner) => inner.span(),
            Self::SubscriptDeclaration(inner) => inner.span(),
            Self::SwitchStatement(inner) => inner.span(),
            Self::ThrowStatement(inner) => inner.span(),
            Self::TypealiasDeclaration(inner) => inner.span(),
            Self::VariableDeclaration(inner) => inner.span(),
            Self::WhileStatement(inner) => inner.span(),
        }
    }
}
#[derive(Debug, Clone)]
pub enum ReturnStatementChildren<'tree> {
    Boolean(::std::boxed::Box<Boolean<'tree>>),
    Identifier(::std::boxed::Box<Identifier<'tree>>),
    Nil(::std::boxed::Box<Nil<'tree>>),
    Number(::std::boxed::Box<Number<'tree>>),
    String(::std::boxed::Box<String<'tree>>),
}
impl<'tree> ::treesitter_types::FromNode<'tree> for ReturnStatementChildren<'tree> {
    #[allow(clippy::collapsible_else_if)]
    fn from_node(
        node: ::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        match node.kind() {
            "boolean" => Ok(Self::Boolean(::std::boxed::Box::new(
                <Boolean as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "identifier" => Ok(Self::Identifier(::std::boxed::Box::new(
                <Identifier as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "nil" => Ok(Self::Nil(::std::boxed::Box::new(
                <Nil as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "number" => Ok(Self::Number(::std::boxed::Box::new(
                <Number as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "string" => Ok(Self::String(::std::boxed::Box::new(
                <String as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            other => Err(::treesitter_types::ParseError::unexpected_kind(other, node)),
        }
    }
}
impl ::treesitter_types::Spanned for ReturnStatementChildren<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        match self {
            Self::Boolean(inner) => inner.span(),
            Self::Identifier(inner) => inner.span(),
            Self::Nil(inner) => inner.span(),
            Self::Number(inner) => inner.span(),
            Self::String(inner) => inner.span(),
        }
    }
}
#[derive(Debug, Clone)]
pub enum StructDeclarationChildren<'tree> {
    ClassDeclaration(::std::boxed::Box<ClassDeclaration<'tree>>),
    ConstantDeclaration(::std::boxed::Box<ConstantDeclaration<'tree>>),
    DeinitializerDeclaration(::std::boxed::Box<DeinitializerDeclaration<'tree>>),
    EnumDeclaration(::std::boxed::Box<EnumDeclaration<'tree>>),
    ExtensionDeclaration(::std::boxed::Box<ExtensionDeclaration<'tree>>),
    FunctionDeclaration(::std::boxed::Box<FunctionDeclaration<'tree>>),
    Identifier(::std::boxed::Box<Identifier<'tree>>),
    ImportDeclaration(::std::boxed::Box<ImportDeclaration<'tree>>),
    InitializerDeclaration(::std::boxed::Box<InitializerDeclaration<'tree>>),
    Modifier(::std::boxed::Box<Modifier<'tree>>),
    OperatorDeclaration(::std::boxed::Box<OperatorDeclaration<'tree>>),
    ProtocolDeclaration(::std::boxed::Box<ProtocolDeclaration<'tree>>),
    StructDeclaration(::std::boxed::Box<StructDeclaration<'tree>>),
    SubscriptDeclaration(::std::boxed::Box<SubscriptDeclaration<'tree>>),
    TypealiasDeclaration(::std::boxed::Box<TypealiasDeclaration<'tree>>),
    VariableDeclaration(::std::boxed::Box<VariableDeclaration<'tree>>),
}
impl<'tree> ::treesitter_types::FromNode<'tree> for StructDeclarationChildren<'tree> {
    #[allow(clippy::collapsible_else_if)]
    fn from_node(
        node: ::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        match node.kind() {
            "class_declaration" => Ok(Self::ClassDeclaration(::std::boxed::Box::new(
                <ClassDeclaration as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "constant_declaration" => Ok(Self::ConstantDeclaration(::std::boxed::Box::new(
                <ConstantDeclaration as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "deinitializer_declaration" => {
                Ok(Self::DeinitializerDeclaration(::std::boxed::Box::new(
                    <DeinitializerDeclaration as ::treesitter_types::FromNode>::from_node(
                        node, src,
                    )?,
                )))
            }
            "enum_declaration" => Ok(Self::EnumDeclaration(::std::boxed::Box::new(
                <EnumDeclaration as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "extension_declaration" => Ok(Self::ExtensionDeclaration(::std::boxed::Box::new(
                <ExtensionDeclaration as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "function_declaration" => Ok(Self::FunctionDeclaration(::std::boxed::Box::new(
                <FunctionDeclaration as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "identifier" => Ok(Self::Identifier(::std::boxed::Box::new(
                <Identifier as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "import_declaration" => Ok(Self::ImportDeclaration(::std::boxed::Box::new(
                <ImportDeclaration as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "initializer_declaration" => Ok(Self::InitializerDeclaration(::std::boxed::Box::new(
                <InitializerDeclaration as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "modifier" => Ok(Self::Modifier(::std::boxed::Box::new(
                <Modifier as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "operator_declaration" => Ok(Self::OperatorDeclaration(::std::boxed::Box::new(
                <OperatorDeclaration as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "protocol_declaration" => Ok(Self::ProtocolDeclaration(::std::boxed::Box::new(
                <ProtocolDeclaration as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "struct_declaration" => Ok(Self::StructDeclaration(::std::boxed::Box::new(
                <StructDeclaration as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "subscript_declaration" => Ok(Self::SubscriptDeclaration(::std::boxed::Box::new(
                <SubscriptDeclaration as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "typealias_declaration" => Ok(Self::TypealiasDeclaration(::std::boxed::Box::new(
                <TypealiasDeclaration as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "variable_declaration" => Ok(Self::VariableDeclaration(::std::boxed::Box::new(
                <VariableDeclaration as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            other => Err(::treesitter_types::ParseError::unexpected_kind(other, node)),
        }
    }
}
impl ::treesitter_types::Spanned for StructDeclarationChildren<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        match self {
            Self::ClassDeclaration(inner) => inner.span(),
            Self::ConstantDeclaration(inner) => inner.span(),
            Self::DeinitializerDeclaration(inner) => inner.span(),
            Self::EnumDeclaration(inner) => inner.span(),
            Self::ExtensionDeclaration(inner) => inner.span(),
            Self::FunctionDeclaration(inner) => inner.span(),
            Self::Identifier(inner) => inner.span(),
            Self::ImportDeclaration(inner) => inner.span(),
            Self::InitializerDeclaration(inner) => inner.span(),
            Self::Modifier(inner) => inner.span(),
            Self::OperatorDeclaration(inner) => inner.span(),
            Self::ProtocolDeclaration(inner) => inner.span(),
            Self::StructDeclaration(inner) => inner.span(),
            Self::SubscriptDeclaration(inner) => inner.span(),
            Self::TypealiasDeclaration(inner) => inner.span(),
            Self::VariableDeclaration(inner) => inner.span(),
        }
    }
}
#[derive(Debug, Clone)]
pub enum SubscriptDeclarationChildren<'tree> {
    ArrayType(::std::boxed::Box<ArrayType<'tree>>),
    Boolean(::std::boxed::Box<Boolean<'tree>>),
    BreakStatement(::std::boxed::Box<BreakStatement<'tree>>),
    BuildConfigurationStatement(::std::boxed::Box<BuildConfigurationStatement<'tree>>),
    ClassDeclaration(::std::boxed::Box<ClassDeclaration<'tree>>),
    ConstantDeclaration(::std::boxed::Box<ConstantDeclaration<'tree>>),
    ContinueStatement(::std::boxed::Box<ContinueStatement<'tree>>),
    DeferStatement(::std::boxed::Box<DeferStatement<'tree>>),
    DeinitializerDeclaration(::std::boxed::Box<DeinitializerDeclaration<'tree>>),
    DiagnosticStatement(::std::boxed::Box<DiagnosticStatement<'tree>>),
    DictionaryType(::std::boxed::Box<DictionaryType<'tree>>),
    DoStatement(::std::boxed::Box<DoStatement<'tree>>),
    EnumDeclaration(::std::boxed::Box<EnumDeclaration<'tree>>),
    ExtensionDeclaration(::std::boxed::Box<ExtensionDeclaration<'tree>>),
    FallthroughStatement(::std::boxed::Box<FallthroughStatement<'tree>>),
    ForStatement(::std::boxed::Box<ForStatement<'tree>>),
    FunctionDeclaration(::std::boxed::Box<FunctionDeclaration<'tree>>),
    GuardStatement(::std::boxed::Box<GuardStatement<'tree>>),
    Identifier(::std::boxed::Box<Identifier<'tree>>),
    IfStatement(::std::boxed::Box<IfStatement<'tree>>),
    ImportDeclaration(::std::boxed::Box<ImportDeclaration<'tree>>),
    InitializerDeclaration(::std::boxed::Box<InitializerDeclaration<'tree>>),
    LabeledStatement(::std::boxed::Box<LabeledStatement<'tree>>),
    LineControlStatement(::std::boxed::Box<LineControlStatement<'tree>>),
    Nil(::std::boxed::Box<Nil<'tree>>),
    Number(::std::boxed::Box<Number<'tree>>),
    OperatorDeclaration(::std::boxed::Box<OperatorDeclaration<'tree>>),
    ProtocolDeclaration(::std::boxed::Box<ProtocolDeclaration<'tree>>),
    RepeatWhileStatement(::std::boxed::Box<RepeatWhileStatement<'tree>>),
    ReturnStatement(::std::boxed::Box<ReturnStatement<'tree>>),
    String(::std::boxed::Box<String<'tree>>),
    StructDeclaration(::std::boxed::Box<StructDeclaration<'tree>>),
    SubscriptDeclaration(::std::boxed::Box<SubscriptDeclaration<'tree>>),
    SwitchStatement(::std::boxed::Box<SwitchStatement<'tree>>),
    ThrowStatement(::std::boxed::Box<ThrowStatement<'tree>>),
    Type(::std::boxed::Box<Type<'tree>>),
    TypealiasDeclaration(::std::boxed::Box<TypealiasDeclaration<'tree>>),
    VariableDeclaration(::std::boxed::Box<VariableDeclaration<'tree>>),
    WhileStatement(::std::boxed::Box<WhileStatement<'tree>>),
}
impl<'tree> ::treesitter_types::FromNode<'tree> for SubscriptDeclarationChildren<'tree> {
    #[allow(clippy::collapsible_else_if)]
    fn from_node(
        node: ::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        match node.kind() {
            "array_type" => Ok(Self::ArrayType(::std::boxed::Box::new(
                <ArrayType as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "boolean" => Ok(Self::Boolean(::std::boxed::Box::new(
                <Boolean as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "break_statement" => Ok(Self::BreakStatement(::std::boxed::Box::new(
                <BreakStatement as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "build_configuration_statement" => {
                Ok(Self::BuildConfigurationStatement(::std::boxed::Box::new(
                    <BuildConfigurationStatement as ::treesitter_types::FromNode>::from_node(
                        node, src,
                    )?,
                )))
            }
            "class_declaration" => Ok(Self::ClassDeclaration(::std::boxed::Box::new(
                <ClassDeclaration as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "constant_declaration" => Ok(Self::ConstantDeclaration(::std::boxed::Box::new(
                <ConstantDeclaration as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "continue_statement" => Ok(Self::ContinueStatement(::std::boxed::Box::new(
                <ContinueStatement as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "defer_statement" => Ok(Self::DeferStatement(::std::boxed::Box::new(
                <DeferStatement as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "deinitializer_declaration" => {
                Ok(Self::DeinitializerDeclaration(::std::boxed::Box::new(
                    <DeinitializerDeclaration as ::treesitter_types::FromNode>::from_node(
                        node, src,
                    )?,
                )))
            }
            "diagnostic_statement" => Ok(Self::DiagnosticStatement(::std::boxed::Box::new(
                <DiagnosticStatement as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "dictionary_type" => Ok(Self::DictionaryType(::std::boxed::Box::new(
                <DictionaryType as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "do_statement" => Ok(Self::DoStatement(::std::boxed::Box::new(
                <DoStatement as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "enum_declaration" => Ok(Self::EnumDeclaration(::std::boxed::Box::new(
                <EnumDeclaration as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "extension_declaration" => Ok(Self::ExtensionDeclaration(::std::boxed::Box::new(
                <ExtensionDeclaration as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "fallthrough_statement" => Ok(Self::FallthroughStatement(::std::boxed::Box::new(
                <FallthroughStatement as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "for_statement" => Ok(Self::ForStatement(::std::boxed::Box::new(
                <ForStatement as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "function_declaration" => Ok(Self::FunctionDeclaration(::std::boxed::Box::new(
                <FunctionDeclaration as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "guard_statement" => Ok(Self::GuardStatement(::std::boxed::Box::new(
                <GuardStatement as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "identifier" => Ok(Self::Identifier(::std::boxed::Box::new(
                <Identifier as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "if_statement" => Ok(Self::IfStatement(::std::boxed::Box::new(
                <IfStatement as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "import_declaration" => Ok(Self::ImportDeclaration(::std::boxed::Box::new(
                <ImportDeclaration as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "initializer_declaration" => Ok(Self::InitializerDeclaration(::std::boxed::Box::new(
                <InitializerDeclaration as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "labeled_statement" => Ok(Self::LabeledStatement(::std::boxed::Box::new(
                <LabeledStatement as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "line_control_statement" => Ok(Self::LineControlStatement(::std::boxed::Box::new(
                <LineControlStatement as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "nil" => Ok(Self::Nil(::std::boxed::Box::new(
                <Nil as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "number" => Ok(Self::Number(::std::boxed::Box::new(
                <Number as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "operator_declaration" => Ok(Self::OperatorDeclaration(::std::boxed::Box::new(
                <OperatorDeclaration as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "protocol_declaration" => Ok(Self::ProtocolDeclaration(::std::boxed::Box::new(
                <ProtocolDeclaration as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "repeat_while_statement" => Ok(Self::RepeatWhileStatement(::std::boxed::Box::new(
                <RepeatWhileStatement as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "return_statement" => Ok(Self::ReturnStatement(::std::boxed::Box::new(
                <ReturnStatement as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "string" => Ok(Self::String(::std::boxed::Box::new(
                <String as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "struct_declaration" => Ok(Self::StructDeclaration(::std::boxed::Box::new(
                <StructDeclaration as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "subscript_declaration" => Ok(Self::SubscriptDeclaration(::std::boxed::Box::new(
                <SubscriptDeclaration as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "switch_statement" => Ok(Self::SwitchStatement(::std::boxed::Box::new(
                <SwitchStatement as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "throw_statement" => Ok(Self::ThrowStatement(::std::boxed::Box::new(
                <ThrowStatement as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "type" => Ok(Self::Type(::std::boxed::Box::new(
                <Type as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "typealias_declaration" => Ok(Self::TypealiasDeclaration(::std::boxed::Box::new(
                <TypealiasDeclaration as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "variable_declaration" => Ok(Self::VariableDeclaration(::std::boxed::Box::new(
                <VariableDeclaration as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "while_statement" => Ok(Self::WhileStatement(::std::boxed::Box::new(
                <WhileStatement as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            other => Err(::treesitter_types::ParseError::unexpected_kind(other, node)),
        }
    }
}
impl ::treesitter_types::Spanned for SubscriptDeclarationChildren<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        match self {
            Self::ArrayType(inner) => inner.span(),
            Self::Boolean(inner) => inner.span(),
            Self::BreakStatement(inner) => inner.span(),
            Self::BuildConfigurationStatement(inner) => inner.span(),
            Self::ClassDeclaration(inner) => inner.span(),
            Self::ConstantDeclaration(inner) => inner.span(),
            Self::ContinueStatement(inner) => inner.span(),
            Self::DeferStatement(inner) => inner.span(),
            Self::DeinitializerDeclaration(inner) => inner.span(),
            Self::DiagnosticStatement(inner) => inner.span(),
            Self::DictionaryType(inner) => inner.span(),
            Self::DoStatement(inner) => inner.span(),
            Self::EnumDeclaration(inner) => inner.span(),
            Self::ExtensionDeclaration(inner) => inner.span(),
            Self::FallthroughStatement(inner) => inner.span(),
            Self::ForStatement(inner) => inner.span(),
            Self::FunctionDeclaration(inner) => inner.span(),
            Self::GuardStatement(inner) => inner.span(),
            Self::Identifier(inner) => inner.span(),
            Self::IfStatement(inner) => inner.span(),
            Self::ImportDeclaration(inner) => inner.span(),
            Self::InitializerDeclaration(inner) => inner.span(),
            Self::LabeledStatement(inner) => inner.span(),
            Self::LineControlStatement(inner) => inner.span(),
            Self::Nil(inner) => inner.span(),
            Self::Number(inner) => inner.span(),
            Self::OperatorDeclaration(inner) => inner.span(),
            Self::ProtocolDeclaration(inner) => inner.span(),
            Self::RepeatWhileStatement(inner) => inner.span(),
            Self::ReturnStatement(inner) => inner.span(),
            Self::String(inner) => inner.span(),
            Self::StructDeclaration(inner) => inner.span(),
            Self::SubscriptDeclaration(inner) => inner.span(),
            Self::SwitchStatement(inner) => inner.span(),
            Self::ThrowStatement(inner) => inner.span(),
            Self::Type(inner) => inner.span(),
            Self::TypealiasDeclaration(inner) => inner.span(),
            Self::VariableDeclaration(inner) => inner.span(),
            Self::WhileStatement(inner) => inner.span(),
        }
    }
}
#[derive(Debug, Clone)]
pub enum SwitchStatementChildren<'tree> {
    Boolean(::std::boxed::Box<Boolean<'tree>>),
    CaseStatement(::std::boxed::Box<CaseStatement<'tree>>),
    Identifier(::std::boxed::Box<Identifier<'tree>>),
    Nil(::std::boxed::Box<Nil<'tree>>),
    Number(::std::boxed::Box<Number<'tree>>),
    String(::std::boxed::Box<String<'tree>>),
}
impl<'tree> ::treesitter_types::FromNode<'tree> for SwitchStatementChildren<'tree> {
    #[allow(clippy::collapsible_else_if)]
    fn from_node(
        node: ::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        match node.kind() {
            "boolean" => Ok(Self::Boolean(::std::boxed::Box::new(
                <Boolean as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "case_statement" => Ok(Self::CaseStatement(::std::boxed::Box::new(
                <CaseStatement as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "identifier" => Ok(Self::Identifier(::std::boxed::Box::new(
                <Identifier as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "nil" => Ok(Self::Nil(::std::boxed::Box::new(
                <Nil as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "number" => Ok(Self::Number(::std::boxed::Box::new(
                <Number as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "string" => Ok(Self::String(::std::boxed::Box::new(
                <String as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            other => Err(::treesitter_types::ParseError::unexpected_kind(other, node)),
        }
    }
}
impl ::treesitter_types::Spanned for SwitchStatementChildren<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        match self {
            Self::Boolean(inner) => inner.span(),
            Self::CaseStatement(inner) => inner.span(),
            Self::Identifier(inner) => inner.span(),
            Self::Nil(inner) => inner.span(),
            Self::Number(inner) => inner.span(),
            Self::String(inner) => inner.span(),
        }
    }
}
#[derive(Debug, Clone)]
pub enum ThrowStatementChildren<'tree> {
    Boolean(::std::boxed::Box<Boolean<'tree>>),
    Identifier(::std::boxed::Box<Identifier<'tree>>),
    Nil(::std::boxed::Box<Nil<'tree>>),
    Number(::std::boxed::Box<Number<'tree>>),
    String(::std::boxed::Box<String<'tree>>),
}
impl<'tree> ::treesitter_types::FromNode<'tree> for ThrowStatementChildren<'tree> {
    #[allow(clippy::collapsible_else_if)]
    fn from_node(
        node: ::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        match node.kind() {
            "boolean" => Ok(Self::Boolean(::std::boxed::Box::new(
                <Boolean as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "identifier" => Ok(Self::Identifier(::std::boxed::Box::new(
                <Identifier as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "nil" => Ok(Self::Nil(::std::boxed::Box::new(
                <Nil as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "number" => Ok(Self::Number(::std::boxed::Box::new(
                <Number as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "string" => Ok(Self::String(::std::boxed::Box::new(
                <String as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            other => Err(::treesitter_types::ParseError::unexpected_kind(other, node)),
        }
    }
}
impl ::treesitter_types::Spanned for ThrowStatementChildren<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        match self {
            Self::Boolean(inner) => inner.span(),
            Self::Identifier(inner) => inner.span(),
            Self::Nil(inner) => inner.span(),
            Self::Number(inner) => inner.span(),
            Self::String(inner) => inner.span(),
        }
    }
}
#[derive(Debug, Clone)]
pub enum TupleChildren<'tree> {
    ArrayType(::std::boxed::Box<ArrayType<'tree>>),
    DictionaryType(::std::boxed::Box<DictionaryType<'tree>>),
    Identifier(::std::boxed::Box<Identifier<'tree>>),
    StandardType(::std::boxed::Box<StandardType<'tree>>),
    TupleType(::std::boxed::Box<TupleType<'tree>>),
    Type(::std::boxed::Box<Type<'tree>>),
    TypeIdentifier(::std::boxed::Box<TypeIdentifier<'tree>>),
}
impl<'tree> ::treesitter_types::FromNode<'tree> for TupleChildren<'tree> {
    #[allow(clippy::collapsible_else_if)]
    fn from_node(
        node: ::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        match node.kind() {
            "array_type" => Ok(Self::ArrayType(::std::boxed::Box::new(
                <ArrayType as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "dictionary_type" => Ok(Self::DictionaryType(::std::boxed::Box::new(
                <DictionaryType as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "identifier" => Ok(Self::Identifier(::std::boxed::Box::new(
                <Identifier as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "standard_type" => Ok(Self::StandardType(::std::boxed::Box::new(
                <StandardType as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "tuple_type" => Ok(Self::TupleType(::std::boxed::Box::new(
                <TupleType as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "type" => Ok(Self::Type(::std::boxed::Box::new(
                <Type as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "type_identifier" => Ok(Self::TypeIdentifier(::std::boxed::Box::new(
                <TypeIdentifier as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            other => Err(::treesitter_types::ParseError::unexpected_kind(other, node)),
        }
    }
}
impl ::treesitter_types::Spanned for TupleChildren<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        match self {
            Self::ArrayType(inner) => inner.span(),
            Self::DictionaryType(inner) => inner.span(),
            Self::Identifier(inner) => inner.span(),
            Self::StandardType(inner) => inner.span(),
            Self::TupleType(inner) => inner.span(),
            Self::Type(inner) => inner.span(),
            Self::TypeIdentifier(inner) => inner.span(),
        }
    }
}
#[derive(Debug, Clone)]
pub enum TupleTypeChildren<'tree> {
    ArrayType(::std::boxed::Box<ArrayType<'tree>>),
    DictionaryType(::std::boxed::Box<DictionaryType<'tree>>),
    Identifier(::std::boxed::Box<Identifier<'tree>>),
    StandardType(::std::boxed::Box<StandardType<'tree>>),
    TupleType(::std::boxed::Box<TupleType<'tree>>),
    Type(::std::boxed::Box<Type<'tree>>),
    TypeIdentifier(::std::boxed::Box<TypeIdentifier<'tree>>),
}
impl<'tree> ::treesitter_types::FromNode<'tree> for TupleTypeChildren<'tree> {
    #[allow(clippy::collapsible_else_if)]
    fn from_node(
        node: ::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        match node.kind() {
            "array_type" => Ok(Self::ArrayType(::std::boxed::Box::new(
                <ArrayType as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "dictionary_type" => Ok(Self::DictionaryType(::std::boxed::Box::new(
                <DictionaryType as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "identifier" => Ok(Self::Identifier(::std::boxed::Box::new(
                <Identifier as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "standard_type" => Ok(Self::StandardType(::std::boxed::Box::new(
                <StandardType as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "tuple_type" => Ok(Self::TupleType(::std::boxed::Box::new(
                <TupleType as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "type" => Ok(Self::Type(::std::boxed::Box::new(
                <Type as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "type_identifier" => Ok(Self::TypeIdentifier(::std::boxed::Box::new(
                <TypeIdentifier as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            other => Err(::treesitter_types::ParseError::unexpected_kind(other, node)),
        }
    }
}
impl ::treesitter_types::Spanned for TupleTypeChildren<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        match self {
            Self::ArrayType(inner) => inner.span(),
            Self::DictionaryType(inner) => inner.span(),
            Self::Identifier(inner) => inner.span(),
            Self::StandardType(inner) => inner.span(),
            Self::TupleType(inner) => inner.span(),
            Self::Type(inner) => inner.span(),
            Self::TypeIdentifier(inner) => inner.span(),
        }
    }
}
#[derive(Debug, Clone)]
pub enum TypeChildren<'tree> {
    Identifier(::std::boxed::Box<Identifier<'tree>>),
    StandardType(::std::boxed::Box<StandardType<'tree>>),
    Tuple(::std::boxed::Box<Tuple<'tree>>),
}
impl<'tree> ::treesitter_types::FromNode<'tree> for TypeChildren<'tree> {
    #[allow(clippy::collapsible_else_if)]
    fn from_node(
        node: ::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        match node.kind() {
            "identifier" => Ok(Self::Identifier(::std::boxed::Box::new(
                <Identifier as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "standard_type" => Ok(Self::StandardType(::std::boxed::Box::new(
                <StandardType as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "tuple" => Ok(Self::Tuple(::std::boxed::Box::new(
                <Tuple as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            other => Err(::treesitter_types::ParseError::unexpected_kind(other, node)),
        }
    }
}
impl ::treesitter_types::Spanned for TypeChildren<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        match self {
            Self::Identifier(inner) => inner.span(),
            Self::StandardType(inner) => inner.span(),
            Self::Tuple(inner) => inner.span(),
        }
    }
}
#[derive(Debug, Clone)]
pub enum TypealiasDeclarationChildren<'tree> {
    Identifier(::std::boxed::Box<Identifier<'tree>>),
    Modifier(::std::boxed::Box<Modifier<'tree>>),
    Type(::std::boxed::Box<Type<'tree>>),
}
impl<'tree> ::treesitter_types::FromNode<'tree> for TypealiasDeclarationChildren<'tree> {
    #[allow(clippy::collapsible_else_if)]
    fn from_node(
        node: ::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        match node.kind() {
            "identifier" => Ok(Self::Identifier(::std::boxed::Box::new(
                <Identifier as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "modifier" => Ok(Self::Modifier(::std::boxed::Box::new(
                <Modifier as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "type" => Ok(Self::Type(::std::boxed::Box::new(
                <Type as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            other => Err(::treesitter_types::ParseError::unexpected_kind(other, node)),
        }
    }
}
impl ::treesitter_types::Spanned for TypealiasDeclarationChildren<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        match self {
            Self::Identifier(inner) => inner.span(),
            Self::Modifier(inner) => inner.span(),
            Self::Type(inner) => inner.span(),
        }
    }
}
#[derive(Debug, Clone)]
pub enum ValueBindingPatternChildren<'tree> {
    AsPattern(::std::boxed::Box<AsPattern<'tree>>),
    Boolean(::std::boxed::Box<Boolean<'tree>>),
    EnumCasePattern(::std::boxed::Box<EnumCasePattern<'tree>>),
    Identifier(::std::boxed::Box<Identifier<'tree>>),
    IsPattern(::std::boxed::Box<IsPattern<'tree>>),
    Nil(::std::boxed::Box<Nil<'tree>>),
    Number(::std::boxed::Box<Number<'tree>>),
    OptionalPattern(::std::boxed::Box<OptionalPattern<'tree>>),
    String(::std::boxed::Box<String<'tree>>),
    ValueBindingPattern(::std::boxed::Box<ValueBindingPattern<'tree>>),
    WildcardPattern(::std::boxed::Box<WildcardPattern<'tree>>),
}
impl<'tree> ::treesitter_types::FromNode<'tree> for ValueBindingPatternChildren<'tree> {
    #[allow(clippy::collapsible_else_if)]
    fn from_node(
        node: ::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        match node.kind() {
            "as_pattern" => Ok(Self::AsPattern(::std::boxed::Box::new(
                <AsPattern as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "boolean" => Ok(Self::Boolean(::std::boxed::Box::new(
                <Boolean as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "enum_case_pattern" => Ok(Self::EnumCasePattern(::std::boxed::Box::new(
                <EnumCasePattern as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "identifier" => Ok(Self::Identifier(::std::boxed::Box::new(
                <Identifier as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "is_pattern" => Ok(Self::IsPattern(::std::boxed::Box::new(
                <IsPattern as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "nil" => Ok(Self::Nil(::std::boxed::Box::new(
                <Nil as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "number" => Ok(Self::Number(::std::boxed::Box::new(
                <Number as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "optional_pattern" => Ok(Self::OptionalPattern(::std::boxed::Box::new(
                <OptionalPattern as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "string" => Ok(Self::String(::std::boxed::Box::new(
                <String as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "value_binding_pattern" => Ok(Self::ValueBindingPattern(::std::boxed::Box::new(
                <ValueBindingPattern as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "wildcard_pattern" => Ok(Self::WildcardPattern(::std::boxed::Box::new(
                <WildcardPattern as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            other => Err(::treesitter_types::ParseError::unexpected_kind(other, node)),
        }
    }
}
impl ::treesitter_types::Spanned for ValueBindingPatternChildren<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        match self {
            Self::AsPattern(inner) => inner.span(),
            Self::Boolean(inner) => inner.span(),
            Self::EnumCasePattern(inner) => inner.span(),
            Self::Identifier(inner) => inner.span(),
            Self::IsPattern(inner) => inner.span(),
            Self::Nil(inner) => inner.span(),
            Self::Number(inner) => inner.span(),
            Self::OptionalPattern(inner) => inner.span(),
            Self::String(inner) => inner.span(),
            Self::ValueBindingPattern(inner) => inner.span(),
            Self::WildcardPattern(inner) => inner.span(),
        }
    }
}
#[derive(Debug, Clone)]
pub enum VariableDeclarationName<'tree> {
    LParen(::treesitter_types::Span),
    RParen(::treesitter_types::Span),
    Comma(::treesitter_types::Span),
    Colon(::treesitter_types::Span),
    LBracket(::treesitter_types::Span),
    RBracket(::treesitter_types::Span),
    AsPattern(::std::boxed::Box<AsPattern<'tree>>),
    Boolean(::std::boxed::Box<Boolean<'tree>>),
    EnumCasePattern(::std::boxed::Box<EnumCasePattern<'tree>>),
    Identifier(::std::boxed::Box<Identifier<'tree>>),
    IsPattern(::std::boxed::Box<IsPattern<'tree>>),
    Nil(::std::boxed::Box<Nil<'tree>>),
    Number(::std::boxed::Box<Number<'tree>>),
    OptionalPattern(::std::boxed::Box<OptionalPattern<'tree>>),
    String(::std::boxed::Box<String<'tree>>),
    ValueBindingPattern(::std::boxed::Box<ValueBindingPattern<'tree>>),
    WildcardPattern(::std::boxed::Box<WildcardPattern<'tree>>),
}
impl<'tree> ::treesitter_types::FromNode<'tree> for VariableDeclarationName<'tree> {
    #[allow(clippy::collapsible_else_if)]
    fn from_node(
        node: ::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        match node.kind() {
            "(" => Ok(Self::LParen(::treesitter_types::Span::from(node))),
            ")" => Ok(Self::RParen(::treesitter_types::Span::from(node))),
            "," => Ok(Self::Comma(::treesitter_types::Span::from(node))),
            ":" => Ok(Self::Colon(::treesitter_types::Span::from(node))),
            "[" => Ok(Self::LBracket(::treesitter_types::Span::from(node))),
            "]" => Ok(Self::RBracket(::treesitter_types::Span::from(node))),
            "as_pattern" => Ok(Self::AsPattern(::std::boxed::Box::new(
                <AsPattern as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "boolean" => Ok(Self::Boolean(::std::boxed::Box::new(
                <Boolean as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "enum_case_pattern" => Ok(Self::EnumCasePattern(::std::boxed::Box::new(
                <EnumCasePattern as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "identifier" => Ok(Self::Identifier(::std::boxed::Box::new(
                <Identifier as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "is_pattern" => Ok(Self::IsPattern(::std::boxed::Box::new(
                <IsPattern as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "nil" => Ok(Self::Nil(::std::boxed::Box::new(
                <Nil as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "number" => Ok(Self::Number(::std::boxed::Box::new(
                <Number as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "optional_pattern" => Ok(Self::OptionalPattern(::std::boxed::Box::new(
                <OptionalPattern as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "string" => Ok(Self::String(::std::boxed::Box::new(
                <String as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "value_binding_pattern" => Ok(Self::ValueBindingPattern(::std::boxed::Box::new(
                <ValueBindingPattern as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "wildcard_pattern" => Ok(Self::WildcardPattern(::std::boxed::Box::new(
                <WildcardPattern as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            other => Err(::treesitter_types::ParseError::unexpected_kind(other, node)),
        }
    }
}
impl ::treesitter_types::Spanned for VariableDeclarationName<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        match self {
            Self::LParen(span) => *span,
            Self::RParen(span) => *span,
            Self::Comma(span) => *span,
            Self::Colon(span) => *span,
            Self::LBracket(span) => *span,
            Self::RBracket(span) => *span,
            Self::AsPattern(inner) => inner.span(),
            Self::Boolean(inner) => inner.span(),
            Self::EnumCasePattern(inner) => inner.span(),
            Self::Identifier(inner) => inner.span(),
            Self::IsPattern(inner) => inner.span(),
            Self::Nil(inner) => inner.span(),
            Self::Number(inner) => inner.span(),
            Self::OptionalPattern(inner) => inner.span(),
            Self::String(inner) => inner.span(),
            Self::ValueBindingPattern(inner) => inner.span(),
            Self::WildcardPattern(inner) => inner.span(),
        }
    }
}
#[derive(Debug, Clone)]
pub enum VariableDeclarationType<'tree> {
    Colon(::treesitter_types::Span),
    ArrayType(::std::boxed::Box<ArrayType<'tree>>),
    DictionaryType(::std::boxed::Box<DictionaryType<'tree>>),
    Type(::std::boxed::Box<Type<'tree>>),
}
impl<'tree> ::treesitter_types::FromNode<'tree> for VariableDeclarationType<'tree> {
    #[allow(clippy::collapsible_else_if)]
    fn from_node(
        node: ::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        match node.kind() {
            ":" => Ok(Self::Colon(::treesitter_types::Span::from(node))),
            "array_type" => Ok(Self::ArrayType(::std::boxed::Box::new(
                <ArrayType as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "dictionary_type" => Ok(Self::DictionaryType(::std::boxed::Box::new(
                <DictionaryType as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "type" => Ok(Self::Type(::std::boxed::Box::new(
                <Type as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            other => Err(::treesitter_types::ParseError::unexpected_kind(other, node)),
        }
    }
}
impl ::treesitter_types::Spanned for VariableDeclarationType<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        match self {
            Self::Colon(span) => *span,
            Self::ArrayType(inner) => inner.span(),
            Self::DictionaryType(inner) => inner.span(),
            Self::Type(inner) => inner.span(),
        }
    }
}
#[derive(Debug, Clone)]
pub enum VariableDeclarationValue<'tree> {
    LParen(::treesitter_types::Span),
    RParen(::treesitter_types::Span),
    Comma(::treesitter_types::Span),
    Colon(::treesitter_types::Span),
    LBracket(::treesitter_types::Span),
    RBracket(::treesitter_types::Span),
    Boolean(::std::boxed::Box<Boolean<'tree>>),
    Identifier(::std::boxed::Box<Identifier<'tree>>),
    Nil(::std::boxed::Box<Nil<'tree>>),
    Number(::std::boxed::Box<Number<'tree>>),
    String(::std::boxed::Box<String<'tree>>),
}
impl<'tree> ::treesitter_types::FromNode<'tree> for VariableDeclarationValue<'tree> {
    #[allow(clippy::collapsible_else_if)]
    fn from_node(
        node: ::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        match node.kind() {
            "(" => Ok(Self::LParen(::treesitter_types::Span::from(node))),
            ")" => Ok(Self::RParen(::treesitter_types::Span::from(node))),
            "," => Ok(Self::Comma(::treesitter_types::Span::from(node))),
            ":" => Ok(Self::Colon(::treesitter_types::Span::from(node))),
            "[" => Ok(Self::LBracket(::treesitter_types::Span::from(node))),
            "]" => Ok(Self::RBracket(::treesitter_types::Span::from(node))),
            "boolean" => Ok(Self::Boolean(::std::boxed::Box::new(
                <Boolean as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "identifier" => Ok(Self::Identifier(::std::boxed::Box::new(
                <Identifier as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "nil" => Ok(Self::Nil(::std::boxed::Box::new(
                <Nil as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "number" => Ok(Self::Number(::std::boxed::Box::new(
                <Number as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "string" => Ok(Self::String(::std::boxed::Box::new(
                <String as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            other => Err(::treesitter_types::ParseError::unexpected_kind(other, node)),
        }
    }
}
impl ::treesitter_types::Spanned for VariableDeclarationValue<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        match self {
            Self::LParen(span) => *span,
            Self::RParen(span) => *span,
            Self::Comma(span) => *span,
            Self::Colon(span) => *span,
            Self::LBracket(span) => *span,
            Self::RBracket(span) => *span,
            Self::Boolean(inner) => inner.span(),
            Self::Identifier(inner) => inner.span(),
            Self::Nil(inner) => inner.span(),
            Self::Number(inner) => inner.span(),
            Self::String(inner) => inner.span(),
        }
    }
}
#[derive(Debug, Clone)]
pub enum VariableDeclarationChildren<'tree> {
    Boolean(::std::boxed::Box<Boolean<'tree>>),
    BreakStatement(::std::boxed::Box<BreakStatement<'tree>>),
    BuildConfigurationStatement(::std::boxed::Box<BuildConfigurationStatement<'tree>>),
    ClassDeclaration(::std::boxed::Box<ClassDeclaration<'tree>>),
    ConstantDeclaration(::std::boxed::Box<ConstantDeclaration<'tree>>),
    ContinueStatement(::std::boxed::Box<ContinueStatement<'tree>>),
    DeferStatement(::std::boxed::Box<DeferStatement<'tree>>),
    DeinitializerDeclaration(::std::boxed::Box<DeinitializerDeclaration<'tree>>),
    DiagnosticStatement(::std::boxed::Box<DiagnosticStatement<'tree>>),
    DoStatement(::std::boxed::Box<DoStatement<'tree>>),
    EnumDeclaration(::std::boxed::Box<EnumDeclaration<'tree>>),
    ExtensionDeclaration(::std::boxed::Box<ExtensionDeclaration<'tree>>),
    FallthroughStatement(::std::boxed::Box<FallthroughStatement<'tree>>),
    ForStatement(::std::boxed::Box<ForStatement<'tree>>),
    FunctionDeclaration(::std::boxed::Box<FunctionDeclaration<'tree>>),
    GuardStatement(::std::boxed::Box<GuardStatement<'tree>>),
    Identifier(::std::boxed::Box<Identifier<'tree>>),
    IfStatement(::std::boxed::Box<IfStatement<'tree>>),
    ImportDeclaration(::std::boxed::Box<ImportDeclaration<'tree>>),
    InitializerDeclaration(::std::boxed::Box<InitializerDeclaration<'tree>>),
    LabeledStatement(::std::boxed::Box<LabeledStatement<'tree>>),
    LineControlStatement(::std::boxed::Box<LineControlStatement<'tree>>),
    Modifier(::std::boxed::Box<Modifier<'tree>>),
    Nil(::std::boxed::Box<Nil<'tree>>),
    Number(::std::boxed::Box<Number<'tree>>),
    OperatorDeclaration(::std::boxed::Box<OperatorDeclaration<'tree>>),
    ProtocolDeclaration(::std::boxed::Box<ProtocolDeclaration<'tree>>),
    RepeatWhileStatement(::std::boxed::Box<RepeatWhileStatement<'tree>>),
    ReturnStatement(::std::boxed::Box<ReturnStatement<'tree>>),
    String(::std::boxed::Box<String<'tree>>),
    StructDeclaration(::std::boxed::Box<StructDeclaration<'tree>>),
    SubscriptDeclaration(::std::boxed::Box<SubscriptDeclaration<'tree>>),
    SwitchStatement(::std::boxed::Box<SwitchStatement<'tree>>),
    ThrowStatement(::std::boxed::Box<ThrowStatement<'tree>>),
    TypealiasDeclaration(::std::boxed::Box<TypealiasDeclaration<'tree>>),
    VariableDeclaration(::std::boxed::Box<VariableDeclaration<'tree>>),
    WhileStatement(::std::boxed::Box<WhileStatement<'tree>>),
}
impl<'tree> ::treesitter_types::FromNode<'tree> for VariableDeclarationChildren<'tree> {
    #[allow(clippy::collapsible_else_if)]
    fn from_node(
        node: ::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        match node.kind() {
            "boolean" => Ok(Self::Boolean(::std::boxed::Box::new(
                <Boolean as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "break_statement" => Ok(Self::BreakStatement(::std::boxed::Box::new(
                <BreakStatement as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "build_configuration_statement" => {
                Ok(Self::BuildConfigurationStatement(::std::boxed::Box::new(
                    <BuildConfigurationStatement as ::treesitter_types::FromNode>::from_node(
                        node, src,
                    )?,
                )))
            }
            "class_declaration" => Ok(Self::ClassDeclaration(::std::boxed::Box::new(
                <ClassDeclaration as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "constant_declaration" => Ok(Self::ConstantDeclaration(::std::boxed::Box::new(
                <ConstantDeclaration as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "continue_statement" => Ok(Self::ContinueStatement(::std::boxed::Box::new(
                <ContinueStatement as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "defer_statement" => Ok(Self::DeferStatement(::std::boxed::Box::new(
                <DeferStatement as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "deinitializer_declaration" => {
                Ok(Self::DeinitializerDeclaration(::std::boxed::Box::new(
                    <DeinitializerDeclaration as ::treesitter_types::FromNode>::from_node(
                        node, src,
                    )?,
                )))
            }
            "diagnostic_statement" => Ok(Self::DiagnosticStatement(::std::boxed::Box::new(
                <DiagnosticStatement as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "do_statement" => Ok(Self::DoStatement(::std::boxed::Box::new(
                <DoStatement as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "enum_declaration" => Ok(Self::EnumDeclaration(::std::boxed::Box::new(
                <EnumDeclaration as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "extension_declaration" => Ok(Self::ExtensionDeclaration(::std::boxed::Box::new(
                <ExtensionDeclaration as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "fallthrough_statement" => Ok(Self::FallthroughStatement(::std::boxed::Box::new(
                <FallthroughStatement as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "for_statement" => Ok(Self::ForStatement(::std::boxed::Box::new(
                <ForStatement as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "function_declaration" => Ok(Self::FunctionDeclaration(::std::boxed::Box::new(
                <FunctionDeclaration as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "guard_statement" => Ok(Self::GuardStatement(::std::boxed::Box::new(
                <GuardStatement as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "identifier" => Ok(Self::Identifier(::std::boxed::Box::new(
                <Identifier as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "if_statement" => Ok(Self::IfStatement(::std::boxed::Box::new(
                <IfStatement as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "import_declaration" => Ok(Self::ImportDeclaration(::std::boxed::Box::new(
                <ImportDeclaration as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "initializer_declaration" => Ok(Self::InitializerDeclaration(::std::boxed::Box::new(
                <InitializerDeclaration as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "labeled_statement" => Ok(Self::LabeledStatement(::std::boxed::Box::new(
                <LabeledStatement as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "line_control_statement" => Ok(Self::LineControlStatement(::std::boxed::Box::new(
                <LineControlStatement as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "modifier" => Ok(Self::Modifier(::std::boxed::Box::new(
                <Modifier as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "nil" => Ok(Self::Nil(::std::boxed::Box::new(
                <Nil as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "number" => Ok(Self::Number(::std::boxed::Box::new(
                <Number as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "operator_declaration" => Ok(Self::OperatorDeclaration(::std::boxed::Box::new(
                <OperatorDeclaration as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "protocol_declaration" => Ok(Self::ProtocolDeclaration(::std::boxed::Box::new(
                <ProtocolDeclaration as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "repeat_while_statement" => Ok(Self::RepeatWhileStatement(::std::boxed::Box::new(
                <RepeatWhileStatement as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "return_statement" => Ok(Self::ReturnStatement(::std::boxed::Box::new(
                <ReturnStatement as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "string" => Ok(Self::String(::std::boxed::Box::new(
                <String as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "struct_declaration" => Ok(Self::StructDeclaration(::std::boxed::Box::new(
                <StructDeclaration as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "subscript_declaration" => Ok(Self::SubscriptDeclaration(::std::boxed::Box::new(
                <SubscriptDeclaration as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "switch_statement" => Ok(Self::SwitchStatement(::std::boxed::Box::new(
                <SwitchStatement as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "throw_statement" => Ok(Self::ThrowStatement(::std::boxed::Box::new(
                <ThrowStatement as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "typealias_declaration" => Ok(Self::TypealiasDeclaration(::std::boxed::Box::new(
                <TypealiasDeclaration as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "variable_declaration" => Ok(Self::VariableDeclaration(::std::boxed::Box::new(
                <VariableDeclaration as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "while_statement" => Ok(Self::WhileStatement(::std::boxed::Box::new(
                <WhileStatement as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            other => Err(::treesitter_types::ParseError::unexpected_kind(other, node)),
        }
    }
}
impl ::treesitter_types::Spanned for VariableDeclarationChildren<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        match self {
            Self::Boolean(inner) => inner.span(),
            Self::BreakStatement(inner) => inner.span(),
            Self::BuildConfigurationStatement(inner) => inner.span(),
            Self::ClassDeclaration(inner) => inner.span(),
            Self::ConstantDeclaration(inner) => inner.span(),
            Self::ContinueStatement(inner) => inner.span(),
            Self::DeferStatement(inner) => inner.span(),
            Self::DeinitializerDeclaration(inner) => inner.span(),
            Self::DiagnosticStatement(inner) => inner.span(),
            Self::DoStatement(inner) => inner.span(),
            Self::EnumDeclaration(inner) => inner.span(),
            Self::ExtensionDeclaration(inner) => inner.span(),
            Self::FallthroughStatement(inner) => inner.span(),
            Self::ForStatement(inner) => inner.span(),
            Self::FunctionDeclaration(inner) => inner.span(),
            Self::GuardStatement(inner) => inner.span(),
            Self::Identifier(inner) => inner.span(),
            Self::IfStatement(inner) => inner.span(),
            Self::ImportDeclaration(inner) => inner.span(),
            Self::InitializerDeclaration(inner) => inner.span(),
            Self::LabeledStatement(inner) => inner.span(),
            Self::LineControlStatement(inner) => inner.span(),
            Self::Modifier(inner) => inner.span(),
            Self::Nil(inner) => inner.span(),
            Self::Number(inner) => inner.span(),
            Self::OperatorDeclaration(inner) => inner.span(),
            Self::ProtocolDeclaration(inner) => inner.span(),
            Self::RepeatWhileStatement(inner) => inner.span(),
            Self::ReturnStatement(inner) => inner.span(),
            Self::String(inner) => inner.span(),
            Self::StructDeclaration(inner) => inner.span(),
            Self::SubscriptDeclaration(inner) => inner.span(),
            Self::SwitchStatement(inner) => inner.span(),
            Self::ThrowStatement(inner) => inner.span(),
            Self::TypealiasDeclaration(inner) => inner.span(),
            Self::VariableDeclaration(inner) => inner.span(),
            Self::WhileStatement(inner) => inner.span(),
        }
    }
}
#[derive(Debug, Clone)]
pub enum WhileStatementChildren<'tree> {
    AvailabilityCondition(::std::boxed::Box<AvailabilityCondition<'tree>>),
    Boolean(::std::boxed::Box<Boolean<'tree>>),
    BreakStatement(::std::boxed::Box<BreakStatement<'tree>>),
    BuildConfigurationStatement(::std::boxed::Box<BuildConfigurationStatement<'tree>>),
    CaseCondition(::std::boxed::Box<CaseCondition<'tree>>),
    ClassDeclaration(::std::boxed::Box<ClassDeclaration<'tree>>),
    ConstantDeclaration(::std::boxed::Box<ConstantDeclaration<'tree>>),
    ContinueStatement(::std::boxed::Box<ContinueStatement<'tree>>),
    DeferStatement(::std::boxed::Box<DeferStatement<'tree>>),
    DeinitializerDeclaration(::std::boxed::Box<DeinitializerDeclaration<'tree>>),
    DiagnosticStatement(::std::boxed::Box<DiagnosticStatement<'tree>>),
    DoStatement(::std::boxed::Box<DoStatement<'tree>>),
    EnumDeclaration(::std::boxed::Box<EnumDeclaration<'tree>>),
    ExtensionDeclaration(::std::boxed::Box<ExtensionDeclaration<'tree>>),
    FallthroughStatement(::std::boxed::Box<FallthroughStatement<'tree>>),
    ForStatement(::std::boxed::Box<ForStatement<'tree>>),
    FunctionDeclaration(::std::boxed::Box<FunctionDeclaration<'tree>>),
    GuardStatement(::std::boxed::Box<GuardStatement<'tree>>),
    Identifier(::std::boxed::Box<Identifier<'tree>>),
    IfStatement(::std::boxed::Box<IfStatement<'tree>>),
    ImportDeclaration(::std::boxed::Box<ImportDeclaration<'tree>>),
    InitializerDeclaration(::std::boxed::Box<InitializerDeclaration<'tree>>),
    LabeledStatement(::std::boxed::Box<LabeledStatement<'tree>>),
    LineControlStatement(::std::boxed::Box<LineControlStatement<'tree>>),
    Nil(::std::boxed::Box<Nil<'tree>>),
    Number(::std::boxed::Box<Number<'tree>>),
    OperatorDeclaration(::std::boxed::Box<OperatorDeclaration<'tree>>),
    OptionalBindingCondition(::std::boxed::Box<OptionalBindingCondition<'tree>>),
    ProtocolDeclaration(::std::boxed::Box<ProtocolDeclaration<'tree>>),
    RepeatWhileStatement(::std::boxed::Box<RepeatWhileStatement<'tree>>),
    ReturnStatement(::std::boxed::Box<ReturnStatement<'tree>>),
    String(::std::boxed::Box<String<'tree>>),
    StructDeclaration(::std::boxed::Box<StructDeclaration<'tree>>),
    SubscriptDeclaration(::std::boxed::Box<SubscriptDeclaration<'tree>>),
    SwitchStatement(::std::boxed::Box<SwitchStatement<'tree>>),
    ThrowStatement(::std::boxed::Box<ThrowStatement<'tree>>),
    TypealiasDeclaration(::std::boxed::Box<TypealiasDeclaration<'tree>>),
    VariableDeclaration(::std::boxed::Box<VariableDeclaration<'tree>>),
    WhileStatement(::std::boxed::Box<WhileStatement<'tree>>),
}
impl<'tree> ::treesitter_types::FromNode<'tree> for WhileStatementChildren<'tree> {
    #[allow(clippy::collapsible_else_if)]
    fn from_node(
        node: ::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        match node.kind() {
            "availability_condition" => Ok(Self::AvailabilityCondition(::std::boxed::Box::new(
                <AvailabilityCondition as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "boolean" => Ok(Self::Boolean(::std::boxed::Box::new(
                <Boolean as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "break_statement" => Ok(Self::BreakStatement(::std::boxed::Box::new(
                <BreakStatement as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "build_configuration_statement" => {
                Ok(Self::BuildConfigurationStatement(::std::boxed::Box::new(
                    <BuildConfigurationStatement as ::treesitter_types::FromNode>::from_node(
                        node, src,
                    )?,
                )))
            }
            "case_condition" => Ok(Self::CaseCondition(::std::boxed::Box::new(
                <CaseCondition as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "class_declaration" => Ok(Self::ClassDeclaration(::std::boxed::Box::new(
                <ClassDeclaration as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "constant_declaration" => Ok(Self::ConstantDeclaration(::std::boxed::Box::new(
                <ConstantDeclaration as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "continue_statement" => Ok(Self::ContinueStatement(::std::boxed::Box::new(
                <ContinueStatement as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "defer_statement" => Ok(Self::DeferStatement(::std::boxed::Box::new(
                <DeferStatement as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "deinitializer_declaration" => {
                Ok(Self::DeinitializerDeclaration(::std::boxed::Box::new(
                    <DeinitializerDeclaration as ::treesitter_types::FromNode>::from_node(
                        node, src,
                    )?,
                )))
            }
            "diagnostic_statement" => Ok(Self::DiagnosticStatement(::std::boxed::Box::new(
                <DiagnosticStatement as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "do_statement" => Ok(Self::DoStatement(::std::boxed::Box::new(
                <DoStatement as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "enum_declaration" => Ok(Self::EnumDeclaration(::std::boxed::Box::new(
                <EnumDeclaration as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "extension_declaration" => Ok(Self::ExtensionDeclaration(::std::boxed::Box::new(
                <ExtensionDeclaration as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "fallthrough_statement" => Ok(Self::FallthroughStatement(::std::boxed::Box::new(
                <FallthroughStatement as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "for_statement" => Ok(Self::ForStatement(::std::boxed::Box::new(
                <ForStatement as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "function_declaration" => Ok(Self::FunctionDeclaration(::std::boxed::Box::new(
                <FunctionDeclaration as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "guard_statement" => Ok(Self::GuardStatement(::std::boxed::Box::new(
                <GuardStatement as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "identifier" => Ok(Self::Identifier(::std::boxed::Box::new(
                <Identifier as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "if_statement" => Ok(Self::IfStatement(::std::boxed::Box::new(
                <IfStatement as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "import_declaration" => Ok(Self::ImportDeclaration(::std::boxed::Box::new(
                <ImportDeclaration as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "initializer_declaration" => Ok(Self::InitializerDeclaration(::std::boxed::Box::new(
                <InitializerDeclaration as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "labeled_statement" => Ok(Self::LabeledStatement(::std::boxed::Box::new(
                <LabeledStatement as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "line_control_statement" => Ok(Self::LineControlStatement(::std::boxed::Box::new(
                <LineControlStatement as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "nil" => Ok(Self::Nil(::std::boxed::Box::new(
                <Nil as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "number" => Ok(Self::Number(::std::boxed::Box::new(
                <Number as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "operator_declaration" => Ok(Self::OperatorDeclaration(::std::boxed::Box::new(
                <OperatorDeclaration as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "optional_binding_condition" => {
                Ok(Self::OptionalBindingCondition(::std::boxed::Box::new(
                    <OptionalBindingCondition as ::treesitter_types::FromNode>::from_node(
                        node, src,
                    )?,
                )))
            }
            "protocol_declaration" => Ok(Self::ProtocolDeclaration(::std::boxed::Box::new(
                <ProtocolDeclaration as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "repeat_while_statement" => Ok(Self::RepeatWhileStatement(::std::boxed::Box::new(
                <RepeatWhileStatement as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "return_statement" => Ok(Self::ReturnStatement(::std::boxed::Box::new(
                <ReturnStatement as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "string" => Ok(Self::String(::std::boxed::Box::new(
                <String as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "struct_declaration" => Ok(Self::StructDeclaration(::std::boxed::Box::new(
                <StructDeclaration as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "subscript_declaration" => Ok(Self::SubscriptDeclaration(::std::boxed::Box::new(
                <SubscriptDeclaration as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "switch_statement" => Ok(Self::SwitchStatement(::std::boxed::Box::new(
                <SwitchStatement as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "throw_statement" => Ok(Self::ThrowStatement(::std::boxed::Box::new(
                <ThrowStatement as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "typealias_declaration" => Ok(Self::TypealiasDeclaration(::std::boxed::Box::new(
                <TypealiasDeclaration as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "variable_declaration" => Ok(Self::VariableDeclaration(::std::boxed::Box::new(
                <VariableDeclaration as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "while_statement" => Ok(Self::WhileStatement(::std::boxed::Box::new(
                <WhileStatement as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            other => Err(::treesitter_types::ParseError::unexpected_kind(other, node)),
        }
    }
}
impl ::treesitter_types::Spanned for WhileStatementChildren<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        match self {
            Self::AvailabilityCondition(inner) => inner.span(),
            Self::Boolean(inner) => inner.span(),
            Self::BreakStatement(inner) => inner.span(),
            Self::BuildConfigurationStatement(inner) => inner.span(),
            Self::CaseCondition(inner) => inner.span(),
            Self::ClassDeclaration(inner) => inner.span(),
            Self::ConstantDeclaration(inner) => inner.span(),
            Self::ContinueStatement(inner) => inner.span(),
            Self::DeferStatement(inner) => inner.span(),
            Self::DeinitializerDeclaration(inner) => inner.span(),
            Self::DiagnosticStatement(inner) => inner.span(),
            Self::DoStatement(inner) => inner.span(),
            Self::EnumDeclaration(inner) => inner.span(),
            Self::ExtensionDeclaration(inner) => inner.span(),
            Self::FallthroughStatement(inner) => inner.span(),
            Self::ForStatement(inner) => inner.span(),
            Self::FunctionDeclaration(inner) => inner.span(),
            Self::GuardStatement(inner) => inner.span(),
            Self::Identifier(inner) => inner.span(),
            Self::IfStatement(inner) => inner.span(),
            Self::ImportDeclaration(inner) => inner.span(),
            Self::InitializerDeclaration(inner) => inner.span(),
            Self::LabeledStatement(inner) => inner.span(),
            Self::LineControlStatement(inner) => inner.span(),
            Self::Nil(inner) => inner.span(),
            Self::Number(inner) => inner.span(),
            Self::OperatorDeclaration(inner) => inner.span(),
            Self::OptionalBindingCondition(inner) => inner.span(),
            Self::ProtocolDeclaration(inner) => inner.span(),
            Self::RepeatWhileStatement(inner) => inner.span(),
            Self::ReturnStatement(inner) => inner.span(),
            Self::String(inner) => inner.span(),
            Self::StructDeclaration(inner) => inner.span(),
            Self::SubscriptDeclaration(inner) => inner.span(),
            Self::SwitchStatement(inner) => inner.span(),
            Self::ThrowStatement(inner) => inner.span(),
            Self::TypealiasDeclaration(inner) => inner.span(),
            Self::VariableDeclaration(inner) => inner.span(),
            Self::WhileStatement(inner) => inner.span(),
        }
    }
}
#[derive(Debug, Clone)]
pub enum AnyNode<'tree> {
    ArrayType(ArrayType<'tree>),
    AsPattern(AsPattern<'tree>),
    AssociatedtypeDeclaration(AssociatedtypeDeclaration<'tree>),
    AssociativityClause(AssociativityClause<'tree>),
    AvailabilityCondition(AvailabilityCondition<'tree>),
    Boolean(Boolean<'tree>),
    BooleanLiteral(BooleanLiteral<'tree>),
    BreakStatement(BreakStatement<'tree>),
    BuildConfigurationStatement(BuildConfigurationStatement<'tree>),
    CaseCondition(CaseCondition<'tree>),
    CaseDeclaration(CaseDeclaration<'tree>),
    CaseStatement(CaseStatement<'tree>),
    CatchClause(CatchClause<'tree>),
    ClassDeclaration(ClassDeclaration<'tree>),
    ConstantDeclaration(ConstantDeclaration<'tree>),
    ContinueStatement(ContinueStatement<'tree>),
    DeferStatement(DeferStatement<'tree>),
    DeinitializerDeclaration(DeinitializerDeclaration<'tree>),
    DiagnosticStatement(DiagnosticStatement<'tree>),
    DictionaryType(DictionaryType<'tree>),
    DoStatement(DoStatement<'tree>),
    EnumCasePattern(EnumCasePattern<'tree>),
    EnumDeclaration(EnumDeclaration<'tree>),
    ExtensionDeclaration(ExtensionDeclaration<'tree>),
    ForStatement(ForStatement<'tree>),
    FunctionDeclaration(FunctionDeclaration<'tree>),
    GenericClause(GenericClause<'tree>),
    GuardStatement(GuardStatement<'tree>),
    IfStatement(IfStatement<'tree>),
    ImportDeclaration(ImportDeclaration<'tree>),
    InitializerDeclaration(InitializerDeclaration<'tree>),
    IsPattern(IsPattern<'tree>),
    LabeledStatement(LabeledStatement<'tree>),
    LineControlStatement(LineControlStatement<'tree>),
    Modifier(Modifier<'tree>),
    OperatorDeclaration(OperatorDeclaration<'tree>),
    OptionalBinding(OptionalBinding<'tree>),
    OptionalBindingCondition(OptionalBindingCondition<'tree>),
    OptionalPattern(OptionalPattern<'tree>),
    ParameterList(ParameterList<'tree>),
    PrecedenceClause(PrecedenceClause<'tree>),
    Program(Program<'tree>),
    ProtocolDeclaration(ProtocolDeclaration<'tree>),
    ProtocolInitializerDeclaration(ProtocolInitializerDeclaration<'tree>),
    ProtocolMethodDeclaration(ProtocolMethodDeclaration<'tree>),
    ProtocolSubscriptDeclaration(ProtocolSubscriptDeclaration<'tree>),
    ProtocolTypealiasDeclaration(ProtocolTypealiasDeclaration<'tree>),
    ProtocolVariableDeclaration(ProtocolVariableDeclaration<'tree>),
    RepeatWhileStatement(RepeatWhileStatement<'tree>),
    ReturnStatement(ReturnStatement<'tree>),
    StructDeclaration(StructDeclaration<'tree>),
    SubscriptDeclaration(SubscriptDeclaration<'tree>),
    SwitchStatement(SwitchStatement<'tree>),
    ThrowStatement(ThrowStatement<'tree>),
    Tuple(Tuple<'tree>),
    TupleType(TupleType<'tree>),
    Type(Type<'tree>),
    TypealiasDeclaration(TypealiasDeclaration<'tree>),
    ValueBindingPattern(ValueBindingPattern<'tree>),
    VariableDeclaration(VariableDeclaration<'tree>),
    WhileStatement(WhileStatement<'tree>),
    WildcardPattern(WildcardPattern<'tree>),
    FallthroughStatement(FallthroughStatement<'tree>),
    Identifier(Identifier<'tree>),
    Nil(Nil<'tree>),
    Number(Number<'tree>),
    Operator(Operator<'tree>),
    SemanticVersion(SemanticVersion<'tree>),
    StandardType(StandardType<'tree>),
    StaticStringLiteral(StaticStringLiteral<'tree>),
    String(String<'tree>),
    TypeIdentifier(TypeIdentifier<'tree>),
    ParameterDeclaration(ParameterDeclaration<'tree>),
    Unknown(::tree_sitter::Node<'tree>),
}
impl<'tree> AnyNode<'tree> {
    pub fn from_node(node: ::tree_sitter::Node<'tree>, src: &'tree [u8]) -> Self {
        match node.kind() {
            "array_type" => <ArrayType as ::treesitter_types::FromNode>::from_node(node, src)
                .map(Self::ArrayType)
                .unwrap_or(Self::Unknown(node)),
            "as_pattern" => <AsPattern as ::treesitter_types::FromNode>::from_node(node, src)
                .map(Self::AsPattern)
                .unwrap_or(Self::Unknown(node)),
            "associatedtype_declaration" => {
                <AssociatedtypeDeclaration as ::treesitter_types::FromNode>::from_node(node, src)
                    .map(Self::AssociatedtypeDeclaration)
                    .unwrap_or(Self::Unknown(node))
            }
            "associativity_clause" => {
                <AssociativityClause as ::treesitter_types::FromNode>::from_node(node, src)
                    .map(Self::AssociativityClause)
                    .unwrap_or(Self::Unknown(node))
            }
            "availability_condition" => {
                <AvailabilityCondition as ::treesitter_types::FromNode>::from_node(node, src)
                    .map(Self::AvailabilityCondition)
                    .unwrap_or(Self::Unknown(node))
            }
            "boolean" => <Boolean as ::treesitter_types::FromNode>::from_node(node, src)
                .map(Self::Boolean)
                .unwrap_or(Self::Unknown(node)),
            "boolean_literal" => {
                <BooleanLiteral as ::treesitter_types::FromNode>::from_node(node, src)
                    .map(Self::BooleanLiteral)
                    .unwrap_or(Self::Unknown(node))
            }
            "break_statement" => {
                <BreakStatement as ::treesitter_types::FromNode>::from_node(node, src)
                    .map(Self::BreakStatement)
                    .unwrap_or(Self::Unknown(node))
            }
            "build_configuration_statement" => {
                <BuildConfigurationStatement as ::treesitter_types::FromNode>::from_node(node, src)
                    .map(Self::BuildConfigurationStatement)
                    .unwrap_or(Self::Unknown(node))
            }
            "case_condition" => {
                <CaseCondition as ::treesitter_types::FromNode>::from_node(node, src)
                    .map(Self::CaseCondition)
                    .unwrap_or(Self::Unknown(node))
            }
            "case_declaration" => {
                <CaseDeclaration as ::treesitter_types::FromNode>::from_node(node, src)
                    .map(Self::CaseDeclaration)
                    .unwrap_or(Self::Unknown(node))
            }
            "case_statement" => {
                <CaseStatement as ::treesitter_types::FromNode>::from_node(node, src)
                    .map(Self::CaseStatement)
                    .unwrap_or(Self::Unknown(node))
            }
            "catch_clause" => <CatchClause as ::treesitter_types::FromNode>::from_node(node, src)
                .map(Self::CatchClause)
                .unwrap_or(Self::Unknown(node)),
            "class_declaration" => {
                <ClassDeclaration as ::treesitter_types::FromNode>::from_node(node, src)
                    .map(Self::ClassDeclaration)
                    .unwrap_or(Self::Unknown(node))
            }
            "constant_declaration" => {
                <ConstantDeclaration as ::treesitter_types::FromNode>::from_node(node, src)
                    .map(Self::ConstantDeclaration)
                    .unwrap_or(Self::Unknown(node))
            }
            "continue_statement" => {
                <ContinueStatement as ::treesitter_types::FromNode>::from_node(node, src)
                    .map(Self::ContinueStatement)
                    .unwrap_or(Self::Unknown(node))
            }
            "defer_statement" => {
                <DeferStatement as ::treesitter_types::FromNode>::from_node(node, src)
                    .map(Self::DeferStatement)
                    .unwrap_or(Self::Unknown(node))
            }
            "deinitializer_declaration" => {
                <DeinitializerDeclaration as ::treesitter_types::FromNode>::from_node(node, src)
                    .map(Self::DeinitializerDeclaration)
                    .unwrap_or(Self::Unknown(node))
            }
            "diagnostic_statement" => {
                <DiagnosticStatement as ::treesitter_types::FromNode>::from_node(node, src)
                    .map(Self::DiagnosticStatement)
                    .unwrap_or(Self::Unknown(node))
            }
            "dictionary_type" => {
                <DictionaryType as ::treesitter_types::FromNode>::from_node(node, src)
                    .map(Self::DictionaryType)
                    .unwrap_or(Self::Unknown(node))
            }
            "do_statement" => <DoStatement as ::treesitter_types::FromNode>::from_node(node, src)
                .map(Self::DoStatement)
                .unwrap_or(Self::Unknown(node)),
            "enum_case_pattern" => {
                <EnumCasePattern as ::treesitter_types::FromNode>::from_node(node, src)
                    .map(Self::EnumCasePattern)
                    .unwrap_or(Self::Unknown(node))
            }
            "enum_declaration" => {
                <EnumDeclaration as ::treesitter_types::FromNode>::from_node(node, src)
                    .map(Self::EnumDeclaration)
                    .unwrap_or(Self::Unknown(node))
            }
            "extension_declaration" => {
                <ExtensionDeclaration as ::treesitter_types::FromNode>::from_node(node, src)
                    .map(Self::ExtensionDeclaration)
                    .unwrap_or(Self::Unknown(node))
            }
            "for_statement" => <ForStatement as ::treesitter_types::FromNode>::from_node(node, src)
                .map(Self::ForStatement)
                .unwrap_or(Self::Unknown(node)),
            "function_declaration" => {
                <FunctionDeclaration as ::treesitter_types::FromNode>::from_node(node, src)
                    .map(Self::FunctionDeclaration)
                    .unwrap_or(Self::Unknown(node))
            }
            "generic_clause" => {
                <GenericClause as ::treesitter_types::FromNode>::from_node(node, src)
                    .map(Self::GenericClause)
                    .unwrap_or(Self::Unknown(node))
            }
            "guard_statement" => {
                <GuardStatement as ::treesitter_types::FromNode>::from_node(node, src)
                    .map(Self::GuardStatement)
                    .unwrap_or(Self::Unknown(node))
            }
            "if_statement" => <IfStatement as ::treesitter_types::FromNode>::from_node(node, src)
                .map(Self::IfStatement)
                .unwrap_or(Self::Unknown(node)),
            "import_declaration" => {
                <ImportDeclaration as ::treesitter_types::FromNode>::from_node(node, src)
                    .map(Self::ImportDeclaration)
                    .unwrap_or(Self::Unknown(node))
            }
            "initializer_declaration" => {
                <InitializerDeclaration as ::treesitter_types::FromNode>::from_node(node, src)
                    .map(Self::InitializerDeclaration)
                    .unwrap_or(Self::Unknown(node))
            }
            "is_pattern" => <IsPattern as ::treesitter_types::FromNode>::from_node(node, src)
                .map(Self::IsPattern)
                .unwrap_or(Self::Unknown(node)),
            "labeled_statement" => {
                <LabeledStatement as ::treesitter_types::FromNode>::from_node(node, src)
                    .map(Self::LabeledStatement)
                    .unwrap_or(Self::Unknown(node))
            }
            "line_control_statement" => {
                <LineControlStatement as ::treesitter_types::FromNode>::from_node(node, src)
                    .map(Self::LineControlStatement)
                    .unwrap_or(Self::Unknown(node))
            }
            "modifier" => <Modifier as ::treesitter_types::FromNode>::from_node(node, src)
                .map(Self::Modifier)
                .unwrap_or(Self::Unknown(node)),
            "operator_declaration" => {
                <OperatorDeclaration as ::treesitter_types::FromNode>::from_node(node, src)
                    .map(Self::OperatorDeclaration)
                    .unwrap_or(Self::Unknown(node))
            }
            "optional_binding" => {
                <OptionalBinding as ::treesitter_types::FromNode>::from_node(node, src)
                    .map(Self::OptionalBinding)
                    .unwrap_or(Self::Unknown(node))
            }
            "optional_binding_condition" => {
                <OptionalBindingCondition as ::treesitter_types::FromNode>::from_node(node, src)
                    .map(Self::OptionalBindingCondition)
                    .unwrap_or(Self::Unknown(node))
            }
            "optional_pattern" => {
                <OptionalPattern as ::treesitter_types::FromNode>::from_node(node, src)
                    .map(Self::OptionalPattern)
                    .unwrap_or(Self::Unknown(node))
            }
            "parameter_list" => {
                <ParameterList as ::treesitter_types::FromNode>::from_node(node, src)
                    .map(Self::ParameterList)
                    .unwrap_or(Self::Unknown(node))
            }
            "precedence_clause" => {
                <PrecedenceClause as ::treesitter_types::FromNode>::from_node(node, src)
                    .map(Self::PrecedenceClause)
                    .unwrap_or(Self::Unknown(node))
            }
            "program" => <Program as ::treesitter_types::FromNode>::from_node(node, src)
                .map(Self::Program)
                .unwrap_or(Self::Unknown(node)),
            "protocol_declaration" => {
                <ProtocolDeclaration as ::treesitter_types::FromNode>::from_node(node, src)
                    .map(Self::ProtocolDeclaration)
                    .unwrap_or(Self::Unknown(node))
            }
            "protocol_initializer_declaration" => {
                <ProtocolInitializerDeclaration as ::treesitter_types::FromNode>::from_node(
                    node, src,
                )
                .map(Self::ProtocolInitializerDeclaration)
                .unwrap_or(Self::Unknown(node))
            }
            "protocol_method_declaration" => {
                <ProtocolMethodDeclaration as ::treesitter_types::FromNode>::from_node(node, src)
                    .map(Self::ProtocolMethodDeclaration)
                    .unwrap_or(Self::Unknown(node))
            }
            "protocol_subscript_declaration" => {
                <ProtocolSubscriptDeclaration as ::treesitter_types::FromNode>::from_node(node, src)
                    .map(Self::ProtocolSubscriptDeclaration)
                    .unwrap_or(Self::Unknown(node))
            }
            "protocol_typealias_declaration" => {
                <ProtocolTypealiasDeclaration as ::treesitter_types::FromNode>::from_node(node, src)
                    .map(Self::ProtocolTypealiasDeclaration)
                    .unwrap_or(Self::Unknown(node))
            }
            "protocol_variable_declaration" => {
                <ProtocolVariableDeclaration as ::treesitter_types::FromNode>::from_node(node, src)
                    .map(Self::ProtocolVariableDeclaration)
                    .unwrap_or(Self::Unknown(node))
            }
            "repeat_while_statement" => {
                <RepeatWhileStatement as ::treesitter_types::FromNode>::from_node(node, src)
                    .map(Self::RepeatWhileStatement)
                    .unwrap_or(Self::Unknown(node))
            }
            "return_statement" => {
                <ReturnStatement as ::treesitter_types::FromNode>::from_node(node, src)
                    .map(Self::ReturnStatement)
                    .unwrap_or(Self::Unknown(node))
            }
            "struct_declaration" => {
                <StructDeclaration as ::treesitter_types::FromNode>::from_node(node, src)
                    .map(Self::StructDeclaration)
                    .unwrap_or(Self::Unknown(node))
            }
            "subscript_declaration" => {
                <SubscriptDeclaration as ::treesitter_types::FromNode>::from_node(node, src)
                    .map(Self::SubscriptDeclaration)
                    .unwrap_or(Self::Unknown(node))
            }
            "switch_statement" => {
                <SwitchStatement as ::treesitter_types::FromNode>::from_node(node, src)
                    .map(Self::SwitchStatement)
                    .unwrap_or(Self::Unknown(node))
            }
            "throw_statement" => {
                <ThrowStatement as ::treesitter_types::FromNode>::from_node(node, src)
                    .map(Self::ThrowStatement)
                    .unwrap_or(Self::Unknown(node))
            }
            "tuple" => <Tuple as ::treesitter_types::FromNode>::from_node(node, src)
                .map(Self::Tuple)
                .unwrap_or(Self::Unknown(node)),
            "tuple_type" => <TupleType as ::treesitter_types::FromNode>::from_node(node, src)
                .map(Self::TupleType)
                .unwrap_or(Self::Unknown(node)),
            "type" => <Type as ::treesitter_types::FromNode>::from_node(node, src)
                .map(Self::Type)
                .unwrap_or(Self::Unknown(node)),
            "typealias_declaration" => {
                <TypealiasDeclaration as ::treesitter_types::FromNode>::from_node(node, src)
                    .map(Self::TypealiasDeclaration)
                    .unwrap_or(Self::Unknown(node))
            }
            "value_binding_pattern" => {
                <ValueBindingPattern as ::treesitter_types::FromNode>::from_node(node, src)
                    .map(Self::ValueBindingPattern)
                    .unwrap_or(Self::Unknown(node))
            }
            "variable_declaration" => {
                <VariableDeclaration as ::treesitter_types::FromNode>::from_node(node, src)
                    .map(Self::VariableDeclaration)
                    .unwrap_or(Self::Unknown(node))
            }
            "while_statement" => {
                <WhileStatement as ::treesitter_types::FromNode>::from_node(node, src)
                    .map(Self::WhileStatement)
                    .unwrap_or(Self::Unknown(node))
            }
            "wildcard_pattern" => {
                <WildcardPattern as ::treesitter_types::FromNode>::from_node(node, src)
                    .map(Self::WildcardPattern)
                    .unwrap_or(Self::Unknown(node))
            }
            "fallthrough_statement" => {
                <FallthroughStatement as ::treesitter_types::FromNode>::from_node(node, src)
                    .map(Self::FallthroughStatement)
                    .unwrap_or(Self::Unknown(node))
            }
            "identifier" => <Identifier as ::treesitter_types::FromNode>::from_node(node, src)
                .map(Self::Identifier)
                .unwrap_or(Self::Unknown(node)),
            "nil" => <Nil as ::treesitter_types::FromNode>::from_node(node, src)
                .map(Self::Nil)
                .unwrap_or(Self::Unknown(node)),
            "number" => <Number as ::treesitter_types::FromNode>::from_node(node, src)
                .map(Self::Number)
                .unwrap_or(Self::Unknown(node)),
            "operator" => <Operator as ::treesitter_types::FromNode>::from_node(node, src)
                .map(Self::Operator)
                .unwrap_or(Self::Unknown(node)),
            "semantic_version" => {
                <SemanticVersion as ::treesitter_types::FromNode>::from_node(node, src)
                    .map(Self::SemanticVersion)
                    .unwrap_or(Self::Unknown(node))
            }
            "standard_type" => <StandardType as ::treesitter_types::FromNode>::from_node(node, src)
                .map(Self::StandardType)
                .unwrap_or(Self::Unknown(node)),
            "static_string_literal" => {
                <StaticStringLiteral as ::treesitter_types::FromNode>::from_node(node, src)
                    .map(Self::StaticStringLiteral)
                    .unwrap_or(Self::Unknown(node))
            }
            "string" => <String as ::treesitter_types::FromNode>::from_node(node, src)
                .map(Self::String)
                .unwrap_or(Self::Unknown(node)),
            "type_identifier" => {
                <TypeIdentifier as ::treesitter_types::FromNode>::from_node(node, src)
                    .map(Self::TypeIdentifier)
                    .unwrap_or(Self::Unknown(node))
            }
            "parameter_declaration" => {
                <ParameterDeclaration as ::treesitter_types::FromNode>::from_node(node, src)
                    .map(Self::ParameterDeclaration)
                    .unwrap_or(Self::Unknown(node))
            }
            _ => Self::Unknown(node),
        }
    }
}
impl ::treesitter_types::Spanned for AnyNode<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        match self {
            Self::ArrayType(inner) => inner.span(),
            Self::AsPattern(inner) => inner.span(),
            Self::AssociatedtypeDeclaration(inner) => inner.span(),
            Self::AssociativityClause(inner) => inner.span(),
            Self::AvailabilityCondition(inner) => inner.span(),
            Self::Boolean(inner) => inner.span(),
            Self::BooleanLiteral(inner) => inner.span(),
            Self::BreakStatement(inner) => inner.span(),
            Self::BuildConfigurationStatement(inner) => inner.span(),
            Self::CaseCondition(inner) => inner.span(),
            Self::CaseDeclaration(inner) => inner.span(),
            Self::CaseStatement(inner) => inner.span(),
            Self::CatchClause(inner) => inner.span(),
            Self::ClassDeclaration(inner) => inner.span(),
            Self::ConstantDeclaration(inner) => inner.span(),
            Self::ContinueStatement(inner) => inner.span(),
            Self::DeferStatement(inner) => inner.span(),
            Self::DeinitializerDeclaration(inner) => inner.span(),
            Self::DiagnosticStatement(inner) => inner.span(),
            Self::DictionaryType(inner) => inner.span(),
            Self::DoStatement(inner) => inner.span(),
            Self::EnumCasePattern(inner) => inner.span(),
            Self::EnumDeclaration(inner) => inner.span(),
            Self::ExtensionDeclaration(inner) => inner.span(),
            Self::ForStatement(inner) => inner.span(),
            Self::FunctionDeclaration(inner) => inner.span(),
            Self::GenericClause(inner) => inner.span(),
            Self::GuardStatement(inner) => inner.span(),
            Self::IfStatement(inner) => inner.span(),
            Self::ImportDeclaration(inner) => inner.span(),
            Self::InitializerDeclaration(inner) => inner.span(),
            Self::IsPattern(inner) => inner.span(),
            Self::LabeledStatement(inner) => inner.span(),
            Self::LineControlStatement(inner) => inner.span(),
            Self::Modifier(inner) => inner.span(),
            Self::OperatorDeclaration(inner) => inner.span(),
            Self::OptionalBinding(inner) => inner.span(),
            Self::OptionalBindingCondition(inner) => inner.span(),
            Self::OptionalPattern(inner) => inner.span(),
            Self::ParameterList(inner) => inner.span(),
            Self::PrecedenceClause(inner) => inner.span(),
            Self::Program(inner) => inner.span(),
            Self::ProtocolDeclaration(inner) => inner.span(),
            Self::ProtocolInitializerDeclaration(inner) => inner.span(),
            Self::ProtocolMethodDeclaration(inner) => inner.span(),
            Self::ProtocolSubscriptDeclaration(inner) => inner.span(),
            Self::ProtocolTypealiasDeclaration(inner) => inner.span(),
            Self::ProtocolVariableDeclaration(inner) => inner.span(),
            Self::RepeatWhileStatement(inner) => inner.span(),
            Self::ReturnStatement(inner) => inner.span(),
            Self::StructDeclaration(inner) => inner.span(),
            Self::SubscriptDeclaration(inner) => inner.span(),
            Self::SwitchStatement(inner) => inner.span(),
            Self::ThrowStatement(inner) => inner.span(),
            Self::Tuple(inner) => inner.span(),
            Self::TupleType(inner) => inner.span(),
            Self::Type(inner) => inner.span(),
            Self::TypealiasDeclaration(inner) => inner.span(),
            Self::ValueBindingPattern(inner) => inner.span(),
            Self::VariableDeclaration(inner) => inner.span(),
            Self::WhileStatement(inner) => inner.span(),
            Self::WildcardPattern(inner) => inner.span(),
            Self::FallthroughStatement(inner) => inner.span(),
            Self::Identifier(inner) => inner.span(),
            Self::Nil(inner) => inner.span(),
            Self::Number(inner) => inner.span(),
            Self::Operator(inner) => inner.span(),
            Self::SemanticVersion(inner) => inner.span(),
            Self::StandardType(inner) => inner.span(),
            Self::StaticStringLiteral(inner) => inner.span(),
            Self::String(inner) => inner.span(),
            Self::TypeIdentifier(inner) => inner.span(),
            Self::ParameterDeclaration(inner) => inner.span(),
            Self::Unknown(node) => ::treesitter_types::Span::from(*node),
        }
    }
}
