#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Declaration<'tree> {
    FunctionDeclaration(::std::boxed::Box<FunctionDeclaration<'tree>>),
    ImplicitVariableDeclaration(::std::boxed::Box<ImplicitVariableDeclaration<'tree>>),
    VariableDeclaration(::std::boxed::Box<VariableDeclaration<'tree>>),
}
impl<'tree> ::treesitter_types::FromNode<'tree> for Declaration<'tree> {
    #[allow(clippy::collapsible_else_if)]
    fn from_node(
        node: ::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        match node.kind() {
            "function_declaration" => Ok(Self::FunctionDeclaration(::std::boxed::Box::new(
                <FunctionDeclaration as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "implicit_variable_declaration" => {
                Ok(Self::ImplicitVariableDeclaration(::std::boxed::Box::new(
                    <ImplicitVariableDeclaration as ::treesitter_types::FromNode>::from_node(
                        node, src,
                    )?,
                )))
            }
            "variable_declaration" => Ok(Self::VariableDeclaration(::std::boxed::Box::new(
                <VariableDeclaration as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            other => Err(::treesitter_types::ParseError::unexpected_kind(other, node)),
        }
    }
}
impl ::treesitter_types::Spanned for Declaration<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        match self {
            Self::FunctionDeclaration(inner) => inner.span(),
            Self::ImplicitVariableDeclaration(inner) => inner.span(),
            Self::VariableDeclaration(inner) => inner.span(),
        }
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Expression<'tree> {
    BinaryExpression(::std::boxed::Box<BinaryExpression<'tree>>),
    False(::std::boxed::Box<False<'tree>>),
    FunctionCall(::std::boxed::Box<FunctionCall<'tree>>),
    FunctionDefinition(::std::boxed::Box<FunctionDefinition<'tree>>),
    Nil(::std::boxed::Box<Nil<'tree>>),
    Number(::std::boxed::Box<Number<'tree>>),
    ParenthesizedExpression(::std::boxed::Box<ParenthesizedExpression<'tree>>),
    String(::std::boxed::Box<String<'tree>>),
    TableConstructor(::std::boxed::Box<TableConstructor<'tree>>),
    True(::std::boxed::Box<True<'tree>>),
    UnaryExpression(::std::boxed::Box<UnaryExpression<'tree>>),
    VarargExpression(::std::boxed::Box<VarargExpression<'tree>>),
    Variable(::std::boxed::Box<Variable<'tree>>),
}
impl<'tree> ::treesitter_types::FromNode<'tree> for Expression<'tree> {
    #[allow(clippy::collapsible_else_if)]
    fn from_node(
        node: ::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        match node.kind() {
            "binary_expression" => Ok(Self::BinaryExpression(::std::boxed::Box::new(
                <BinaryExpression as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "false" => Ok(Self::False(::std::boxed::Box::new(
                <False as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "function_call" => Ok(Self::FunctionCall(::std::boxed::Box::new(
                <FunctionCall as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "function_definition" => Ok(Self::FunctionDefinition(::std::boxed::Box::new(
                <FunctionDefinition as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "nil" => Ok(Self::Nil(::std::boxed::Box::new(
                <Nil as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "number" => Ok(Self::Number(::std::boxed::Box::new(
                <Number as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "parenthesized_expression" => {
                Ok(Self::ParenthesizedExpression(::std::boxed::Box::new(
                    <ParenthesizedExpression as ::treesitter_types::FromNode>::from_node(
                        node, src,
                    )?,
                )))
            }
            "string" => Ok(Self::String(::std::boxed::Box::new(
                <String as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "table_constructor" => Ok(Self::TableConstructor(::std::boxed::Box::new(
                <TableConstructor as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "true" => Ok(Self::True(::std::boxed::Box::new(
                <True as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "unary_expression" => Ok(Self::UnaryExpression(::std::boxed::Box::new(
                <UnaryExpression as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "vararg_expression" => Ok(Self::VarargExpression(::std::boxed::Box::new(
                <VarargExpression as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            _other => {
                if let Ok(v) = <Variable as ::treesitter_types::FromNode>::from_node(node, src) {
                    Ok(Self::Variable(::std::boxed::Box::new(v)))
                } else {
                    Err(::treesitter_types::ParseError::unexpected_kind(
                        _other, node,
                    ))
                }
            }
        }
    }
}
impl ::treesitter_types::Spanned for Expression<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        match self {
            Self::BinaryExpression(inner) => inner.span(),
            Self::False(inner) => inner.span(),
            Self::FunctionCall(inner) => inner.span(),
            Self::FunctionDefinition(inner) => inner.span(),
            Self::Nil(inner) => inner.span(),
            Self::Number(inner) => inner.span(),
            Self::ParenthesizedExpression(inner) => inner.span(),
            Self::String(inner) => inner.span(),
            Self::TableConstructor(inner) => inner.span(),
            Self::True(inner) => inner.span(),
            Self::UnaryExpression(inner) => inner.span(),
            Self::VarargExpression(inner) => inner.span(),
            Self::Variable(inner) => inner.span(),
        }
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Statement<'tree> {
    AssignmentStatement(::std::boxed::Box<AssignmentStatement<'tree>>),
    BreakStatement(::std::boxed::Box<BreakStatement<'tree>>),
    Declaration(::std::boxed::Box<Declaration<'tree>>),
    DoStatement(::std::boxed::Box<DoStatement<'tree>>),
    EmptyStatement(::std::boxed::Box<EmptyStatement<'tree>>),
    ForStatement(::std::boxed::Box<ForStatement<'tree>>),
    FunctionCall(::std::boxed::Box<FunctionCall<'tree>>),
    GotoStatement(::std::boxed::Box<GotoStatement<'tree>>),
    IfStatement(::std::boxed::Box<IfStatement<'tree>>),
    LabelStatement(::std::boxed::Box<LabelStatement<'tree>>),
    RepeatStatement(::std::boxed::Box<RepeatStatement<'tree>>),
    WhileStatement(::std::boxed::Box<WhileStatement<'tree>>),
}
impl<'tree> ::treesitter_types::FromNode<'tree> for Statement<'tree> {
    #[allow(clippy::collapsible_else_if)]
    fn from_node(
        node: ::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        match node.kind() {
            "assignment_statement" => Ok(Self::AssignmentStatement(::std::boxed::Box::new(
                <AssignmentStatement as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "break_statement" => Ok(Self::BreakStatement(::std::boxed::Box::new(
                <BreakStatement as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "do_statement" => Ok(Self::DoStatement(::std::boxed::Box::new(
                <DoStatement as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "empty_statement" => Ok(Self::EmptyStatement(::std::boxed::Box::new(
                <EmptyStatement as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "for_statement" => Ok(Self::ForStatement(::std::boxed::Box::new(
                <ForStatement as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "function_call" => Ok(Self::FunctionCall(::std::boxed::Box::new(
                <FunctionCall as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "goto_statement" => Ok(Self::GotoStatement(::std::boxed::Box::new(
                <GotoStatement as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "if_statement" => Ok(Self::IfStatement(::std::boxed::Box::new(
                <IfStatement as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "label_statement" => Ok(Self::LabelStatement(::std::boxed::Box::new(
                <LabelStatement as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "repeat_statement" => Ok(Self::RepeatStatement(::std::boxed::Box::new(
                <RepeatStatement as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "while_statement" => Ok(Self::WhileStatement(::std::boxed::Box::new(
                <WhileStatement as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            _other => {
                if let Ok(v) = <Declaration as ::treesitter_types::FromNode>::from_node(node, src) {
                    Ok(Self::Declaration(::std::boxed::Box::new(v)))
                } else {
                    Err(::treesitter_types::ParseError::unexpected_kind(
                        _other, node,
                    ))
                }
            }
        }
    }
}
impl ::treesitter_types::Spanned for Statement<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        match self {
            Self::AssignmentStatement(inner) => inner.span(),
            Self::BreakStatement(inner) => inner.span(),
            Self::Declaration(inner) => inner.span(),
            Self::DoStatement(inner) => inner.span(),
            Self::EmptyStatement(inner) => inner.span(),
            Self::ForStatement(inner) => inner.span(),
            Self::FunctionCall(inner) => inner.span(),
            Self::GotoStatement(inner) => inner.span(),
            Self::IfStatement(inner) => inner.span(),
            Self::LabelStatement(inner) => inner.span(),
            Self::RepeatStatement(inner) => inner.span(),
            Self::WhileStatement(inner) => inner.span(),
        }
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Variable<'tree> {
    BracketIndexExpression(::std::boxed::Box<BracketIndexExpression<'tree>>),
    DotIndexExpression(::std::boxed::Box<DotIndexExpression<'tree>>),
    Global(::treesitter_types::Span),
    Identifier(::std::boxed::Box<Identifier<'tree>>),
}
impl<'tree> ::treesitter_types::FromNode<'tree> for Variable<'tree> {
    #[allow(clippy::collapsible_else_if)]
    fn from_node(
        node: ::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        match node.kind() {
            "bracket_index_expression" => Ok(Self::BracketIndexExpression(::std::boxed::Box::new(
                <BracketIndexExpression as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "dot_index_expression" => Ok(Self::DotIndexExpression(::std::boxed::Box::new(
                <DotIndexExpression as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "global" => Ok(Self::Global(::treesitter_types::Span::from(node))),
            "identifier" => Ok(Self::Identifier(::std::boxed::Box::new(
                <Identifier as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            other => Err(::treesitter_types::ParseError::unexpected_kind(other, node)),
        }
    }
}
impl ::treesitter_types::Spanned for Variable<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        match self {
            Self::BracketIndexExpression(inner) => inner.span(),
            Self::DotIndexExpression(inner) => inner.span(),
            Self::Global(span) => *span,
            Self::Identifier(inner) => inner.span(),
        }
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Arguments<'tree> {
    pub span: ::treesitter_types::Span,
    pub children: ::std::vec::Vec<Expression<'tree>>,
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
                    items.push(<Expression as ::treesitter_types::FromNode>::from_node(
                        child, src,
                    )?);
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
pub struct AssignmentStatement<'tree> {
    pub span: ::treesitter_types::Span,
    pub operator: AssignmentStatementOperator,
    pub children: ::std::vec::Vec<AssignmentStatementChildren<'tree>>,
}
impl<'tree> ::treesitter_types::FromNode<'tree> for AssignmentStatement<'tree> {
    #[allow(clippy::match_single_binding, clippy::suspicious_else_formatting)]
    fn from_node(
        node: ::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        debug_assert_eq!(node.kind(), "assignment_statement");
        Ok(Self {
            span: ::treesitter_types::Span::from(node),
            operator: {
                let child = node.child_by_field_name("operator").ok_or_else(|| {
                    ::treesitter_types::ParseError::missing_field("operator", node)
                })?;
                <AssignmentStatementOperator as ::treesitter_types::FromNode>::from_node(
                    child, src,
                )?
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
                        <AssignmentStatementChildren as ::treesitter_types::FromNode>::from_node(
                            child, src,
                        )?,
                    );
                }
                items
            },
        })
    }
}
impl ::treesitter_types::Spanned for AssignmentStatement<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        self.span
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Attribute<'tree> {
    pub span: ::treesitter_types::Span,
    pub children: Identifier<'tree>,
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
                                        <Identifier as ::treesitter_types::FromNode>::from_node(
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
                                            <Identifier as ::treesitter_types::FromNode>::from_node(
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
                <Identifier as ::treesitter_types::FromNode>::from_node(child, src)?
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
pub struct BinaryExpression<'tree> {
    pub span: ::treesitter_types::Span,
    pub left: Expression<'tree>,
    pub operator: BinaryExpressionOperator,
    pub right: Expression<'tree>,
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
            left: {
                let child = node
                    .child_by_field_name("left")
                    .ok_or_else(|| ::treesitter_types::ParseError::missing_field("left", node))?;
                <Expression as ::treesitter_types::FromNode>::from_node(child, src)?
            },
            operator: {
                let child = node.child_by_field_name("operator").ok_or_else(|| {
                    ::treesitter_types::ParseError::missing_field("operator", node)
                })?;
                <BinaryExpressionOperator as ::treesitter_types::FromNode>::from_node(child, src)?
            },
            right: {
                let child = node
                    .child_by_field_name("right")
                    .ok_or_else(|| ::treesitter_types::ParseError::missing_field("right", node))?;
                <Expression as ::treesitter_types::FromNode>::from_node(child, src)?
            },
        })
    }
}
impl ::treesitter_types::Spanned for BinaryExpression<'_> {
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
pub struct BracketIndexExpression<'tree> {
    pub span: ::treesitter_types::Span,
    pub field: Expression<'tree>,
    pub table: BracketIndexExpressionTable<'tree>,
}
impl<'tree> ::treesitter_types::FromNode<'tree> for BracketIndexExpression<'tree> {
    #[allow(clippy::match_single_binding, clippy::suspicious_else_formatting)]
    fn from_node(
        node: ::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        debug_assert_eq!(node.kind(), "bracket_index_expression");
        Ok(Self {
            span: ::treesitter_types::Span::from(node),
            field: {
                let child = node
                    .child_by_field_name("field")
                    .ok_or_else(|| ::treesitter_types::ParseError::missing_field("field", node))?;
                <Expression as ::treesitter_types::FromNode>::from_node(child, src)?
            },
            table: {
                let child = node
                    .child_by_field_name("table")
                    .ok_or_else(|| ::treesitter_types::ParseError::missing_field("table", node))?;
                <BracketIndexExpressionTable as ::treesitter_types::FromNode>::from_node(
                    child, src,
                )?
            },
        })
    }
}
impl ::treesitter_types::Spanned for BracketIndexExpression<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        self.span
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Chunk<'tree> {
    pub span: ::treesitter_types::Span,
    pub children: ::std::vec::Vec<ChunkChildren<'tree>>,
}
impl<'tree> ::treesitter_types::FromNode<'tree> for Chunk<'tree> {
    #[allow(clippy::match_single_binding, clippy::suspicious_else_formatting)]
    fn from_node(
        node: ::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        debug_assert_eq!(node.kind(), "chunk");
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
                    items.push(<ChunkChildren as ::treesitter_types::FromNode>::from_node(
                        child, src,
                    )?);
                }
                items
            },
        })
    }
}
impl ::treesitter_types::Spanned for Chunk<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        self.span
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Comment<'tree> {
    pub span: ::treesitter_types::Span,
    pub content: CommentContent<'tree>,
    pub end: ::core::option::Option<CommentEnd>,
    pub start: CommentStart,
}
impl<'tree> ::treesitter_types::FromNode<'tree> for Comment<'tree> {
    #[allow(clippy::match_single_binding, clippy::suspicious_else_formatting)]
    fn from_node(
        node: ::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        debug_assert_eq!(node.kind(), "comment");
        Ok(Self {
            span: ::treesitter_types::Span::from(node),
            content: {
                let child = node.child_by_field_name("content").ok_or_else(|| {
                    ::treesitter_types::ParseError::missing_field("content", node)
                })?;
                <CommentContent as ::treesitter_types::FromNode>::from_node(child, src)?
            },
            end: match node.child_by_field_name("end") {
                Some(child) => Some(<CommentEnd as ::treesitter_types::FromNode>::from_node(
                    child, src,
                )?),
                None => None,
            },
            start: {
                let child = node
                    .child_by_field_name("start")
                    .ok_or_else(|| ::treesitter_types::ParseError::missing_field("start", node))?;
                <CommentStart as ::treesitter_types::FromNode>::from_node(child, src)?
            },
        })
    }
}
impl ::treesitter_types::Spanned for Comment<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        self.span
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct DoStatement<'tree> {
    pub span: ::treesitter_types::Span,
    pub body: ::core::option::Option<Block<'tree>>,
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
            body: match node.child_by_field_name("body") {
                Some(child) => Some(<Block as ::treesitter_types::FromNode>::from_node(
                    child, src,
                )?),
                None => None,
            },
        })
    }
}
impl ::treesitter_types::Spanned for DoStatement<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        self.span
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct DotIndexExpression<'tree> {
    pub span: ::treesitter_types::Span,
    pub field: Identifier<'tree>,
    pub table: DotIndexExpressionTable<'tree>,
}
impl<'tree> ::treesitter_types::FromNode<'tree> for DotIndexExpression<'tree> {
    #[allow(clippy::match_single_binding, clippy::suspicious_else_formatting)]
    fn from_node(
        node: ::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        debug_assert_eq!(node.kind(), "dot_index_expression");
        Ok(Self {
            span: ::treesitter_types::Span::from(node),
            field: {
                let child = node
                    .child_by_field_name("field")
                    .ok_or_else(|| ::treesitter_types::ParseError::missing_field("field", node))?;
                <Identifier as ::treesitter_types::FromNode>::from_node(child, src)?
            },
            table: {
                let child = node
                    .child_by_field_name("table")
                    .ok_or_else(|| ::treesitter_types::ParseError::missing_field("table", node))?;
                <DotIndexExpressionTable as ::treesitter_types::FromNode>::from_node(child, src)?
            },
        })
    }
}
impl ::treesitter_types::Spanned for DotIndexExpression<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        self.span
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ElseStatement<'tree> {
    pub span: ::treesitter_types::Span,
    pub body: ::core::option::Option<Block<'tree>>,
}
impl<'tree> ::treesitter_types::FromNode<'tree> for ElseStatement<'tree> {
    #[allow(clippy::match_single_binding, clippy::suspicious_else_formatting)]
    fn from_node(
        node: ::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        debug_assert_eq!(node.kind(), "else_statement");
        Ok(Self {
            span: ::treesitter_types::Span::from(node),
            body: match node.child_by_field_name("body") {
                Some(child) => Some(<Block as ::treesitter_types::FromNode>::from_node(
                    child, src,
                )?),
                None => None,
            },
        })
    }
}
impl ::treesitter_types::Spanned for ElseStatement<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        self.span
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ElseifStatement<'tree> {
    pub span: ::treesitter_types::Span,
    pub condition: Expression<'tree>,
    pub consequence: ::core::option::Option<Block<'tree>>,
}
impl<'tree> ::treesitter_types::FromNode<'tree> for ElseifStatement<'tree> {
    #[allow(clippy::match_single_binding, clippy::suspicious_else_formatting)]
    fn from_node(
        node: ::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        debug_assert_eq!(node.kind(), "elseif_statement");
        Ok(Self {
            span: ::treesitter_types::Span::from(node),
            condition: {
                let child = node.child_by_field_name("condition").ok_or_else(|| {
                    ::treesitter_types::ParseError::missing_field("condition", node)
                })?;
                <Expression as ::treesitter_types::FromNode>::from_node(child, src)?
            },
            consequence: match node.child_by_field_name("consequence") {
                Some(child) => Some(<Block as ::treesitter_types::FromNode>::from_node(
                    child, src,
                )?),
                None => None,
            },
        })
    }
}
impl ::treesitter_types::Spanned for ElseifStatement<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        self.span
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct EmptyStatement<'tree> {
    pub span: ::treesitter_types::Span,
    text: &'tree str,
}
impl<'tree> ::treesitter_types::FromNode<'tree> for EmptyStatement<'tree> {
    fn from_node(
        node: ::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        debug_assert_eq!(node.kind(), "empty_statement");
        Ok(Self {
            span: ::treesitter_types::Span::from(node),
            text: node.utf8_text(src)?,
        })
    }
}
impl<'tree> ::treesitter_types::LeafNode<'tree> for EmptyStatement<'tree> {
    fn text(&self) -> &'tree str {
        self.text
    }
}
impl ::treesitter_types::Spanned for EmptyStatement<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        self.span
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ExpressionList<'tree> {
    pub span: ::treesitter_types::Span,
    pub value: ::std::vec::Vec<Expression<'tree>>,
    pub children: ::std::vec::Vec<Expression<'tree>>,
}
impl<'tree> ::treesitter_types::FromNode<'tree> for ExpressionList<'tree> {
    #[allow(clippy::match_single_binding, clippy::suspicious_else_formatting)]
    fn from_node(
        node: ::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        debug_assert_eq!(node.kind(), "expression_list");
        Ok(Self {
            span: ::treesitter_types::Span::from(node),
            value: {
                let mut cursor = node.walk();
                let mut items = ::std::vec::Vec::new();
                for child in node.children_by_field_name("value", &mut cursor) {
                    items.push(<Expression as ::treesitter_types::FromNode>::from_node(
                        child, src,
                    )?);
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
                    items.push(<Expression as ::treesitter_types::FromNode>::from_node(
                        child, src,
                    )?);
                }
                items
            },
        })
    }
}
impl ::treesitter_types::Spanned for ExpressionList<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        self.span
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Field<'tree> {
    pub span: ::treesitter_types::Span,
    pub name: ::core::option::Option<FieldName<'tree>>,
    pub operator: ::core::option::Option<FieldOperator>,
    pub value: Expression<'tree>,
}
impl<'tree> ::treesitter_types::FromNode<'tree> for Field<'tree> {
    #[allow(clippy::match_single_binding, clippy::suspicious_else_formatting)]
    fn from_node(
        node: ::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        debug_assert_eq!(node.kind(), "field");
        Ok(Self {
            span: ::treesitter_types::Span::from(node),
            name: match node.child_by_field_name("name") {
                Some(child) => Some(<FieldName as ::treesitter_types::FromNode>::from_node(
                    child, src,
                )?),
                None => None,
            },
            operator: match node.child_by_field_name("operator") {
                Some(child) => Some(<FieldOperator as ::treesitter_types::FromNode>::from_node(
                    child, src,
                )?),
                None => None,
            },
            value: {
                let child = node
                    .child_by_field_name("value")
                    .ok_or_else(|| ::treesitter_types::ParseError::missing_field("value", node))?;
                <Expression as ::treesitter_types::FromNode>::from_node(child, src)?
            },
        })
    }
}
impl ::treesitter_types::Spanned for Field<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        self.span
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ForGenericClause<'tree> {
    pub span: ::treesitter_types::Span,
    pub children: ::std::vec::Vec<ForGenericClauseChildren<'tree>>,
}
impl<'tree> ::treesitter_types::FromNode<'tree> for ForGenericClause<'tree> {
    #[allow(clippy::match_single_binding, clippy::suspicious_else_formatting)]
    fn from_node(
        node: ::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        debug_assert_eq!(node.kind(), "for_generic_clause");
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
                        <ForGenericClauseChildren as ::treesitter_types::FromNode>::from_node(
                            child, src,
                        )?,
                    );
                }
                items
            },
        })
    }
}
impl ::treesitter_types::Spanned for ForGenericClause<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        self.span
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ForNumericClause<'tree> {
    pub span: ::treesitter_types::Span,
    pub end: Expression<'tree>,
    pub name: Identifier<'tree>,
    pub operator: ForNumericClauseOperator,
    pub start: Expression<'tree>,
    pub step: ::core::option::Option<Expression<'tree>>,
}
impl<'tree> ::treesitter_types::FromNode<'tree> for ForNumericClause<'tree> {
    #[allow(clippy::match_single_binding, clippy::suspicious_else_formatting)]
    fn from_node(
        node: ::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        debug_assert_eq!(node.kind(), "for_numeric_clause");
        Ok(Self {
            span: ::treesitter_types::Span::from(node),
            end: {
                let child = node
                    .child_by_field_name("end")
                    .ok_or_else(|| ::treesitter_types::ParseError::missing_field("end", node))?;
                <Expression as ::treesitter_types::FromNode>::from_node(child, src)?
            },
            name: {
                let child = node
                    .child_by_field_name("name")
                    .ok_or_else(|| ::treesitter_types::ParseError::missing_field("name", node))?;
                <Identifier as ::treesitter_types::FromNode>::from_node(child, src)?
            },
            operator: {
                let child = node.child_by_field_name("operator").ok_or_else(|| {
                    ::treesitter_types::ParseError::missing_field("operator", node)
                })?;
                <ForNumericClauseOperator as ::treesitter_types::FromNode>::from_node(child, src)?
            },
            start: {
                let child = node
                    .child_by_field_name("start")
                    .ok_or_else(|| ::treesitter_types::ParseError::missing_field("start", node))?;
                <Expression as ::treesitter_types::FromNode>::from_node(child, src)?
            },
            step: match node.child_by_field_name("step") {
                Some(child) => Some(<Expression as ::treesitter_types::FromNode>::from_node(
                    child, src,
                )?),
                None => None,
            },
        })
    }
}
impl ::treesitter_types::Spanned for ForNumericClause<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        self.span
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ForStatement<'tree> {
    pub span: ::treesitter_types::Span,
    pub body: ::core::option::Option<Block<'tree>>,
    pub clause: ForStatementClause<'tree>,
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
            body: match node.child_by_field_name("body") {
                Some(child) => Some(<Block as ::treesitter_types::FromNode>::from_node(
                    child, src,
                )?),
                None => None,
            },
            clause: {
                let child = node
                    .child_by_field_name("clause")
                    .ok_or_else(|| ::treesitter_types::ParseError::missing_field("clause", node))?;
                <ForStatementClause as ::treesitter_types::FromNode>::from_node(child, src)?
            },
        })
    }
}
impl ::treesitter_types::Spanned for ForStatement<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        self.span
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct FunctionCall<'tree> {
    pub span: ::treesitter_types::Span,
    pub arguments: Arguments<'tree>,
    pub name: FunctionCallName<'tree>,
}
impl<'tree> ::treesitter_types::FromNode<'tree> for FunctionCall<'tree> {
    #[allow(clippy::match_single_binding, clippy::suspicious_else_formatting)]
    fn from_node(
        node: ::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        debug_assert_eq!(node.kind(), "function_call");
        Ok(Self {
            span: ::treesitter_types::Span::from(node),
            arguments: {
                let child = node.child_by_field_name("arguments").ok_or_else(|| {
                    ::treesitter_types::ParseError::missing_field("arguments", node)
                })?;
                <Arguments as ::treesitter_types::FromNode>::from_node(child, src)?
            },
            name: {
                let child = node
                    .child_by_field_name("name")
                    .ok_or_else(|| ::treesitter_types::ParseError::missing_field("name", node))?;
                <FunctionCallName as ::treesitter_types::FromNode>::from_node(child, src)?
            },
        })
    }
}
impl ::treesitter_types::Spanned for FunctionCall<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        self.span
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct FunctionDeclaration<'tree> {
    pub span: ::treesitter_types::Span,
    pub body: ::core::option::Option<Block<'tree>>,
    pub name: FunctionDeclarationName<'tree>,
    pub parameters: Parameters<'tree>,
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
            body: match node.child_by_field_name("body") {
                Some(child) => Some(<Block as ::treesitter_types::FromNode>::from_node(
                    child, src,
                )?),
                None => None,
            },
            name: {
                let child = node
                    .child_by_field_name("name")
                    .ok_or_else(|| ::treesitter_types::ParseError::missing_field("name", node))?;
                <FunctionDeclarationName as ::treesitter_types::FromNode>::from_node(child, src)?
            },
            parameters: {
                let child = node.child_by_field_name("parameters").ok_or_else(|| {
                    ::treesitter_types::ParseError::missing_field("parameters", node)
                })?;
                <Parameters as ::treesitter_types::FromNode>::from_node(child, src)?
            },
        })
    }
}
impl ::treesitter_types::Spanned for FunctionDeclaration<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        self.span
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct FunctionDefinition<'tree> {
    pub span: ::treesitter_types::Span,
    pub body: ::core::option::Option<Block<'tree>>,
    pub parameters: Parameters<'tree>,
}
impl<'tree> ::treesitter_types::FromNode<'tree> for FunctionDefinition<'tree> {
    #[allow(clippy::match_single_binding, clippy::suspicious_else_formatting)]
    fn from_node(
        node: ::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        debug_assert_eq!(node.kind(), "function_definition");
        Ok(Self {
            span: ::treesitter_types::Span::from(node),
            body: match node.child_by_field_name("body") {
                Some(child) => Some(<Block as ::treesitter_types::FromNode>::from_node(
                    child, src,
                )?),
                None => None,
            },
            parameters: {
                let child = node.child_by_field_name("parameters").ok_or_else(|| {
                    ::treesitter_types::ParseError::missing_field("parameters", node)
                })?;
                <Parameters as ::treesitter_types::FromNode>::from_node(child, src)?
            },
        })
    }
}
impl ::treesitter_types::Spanned for FunctionDefinition<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        self.span
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct GotoStatement<'tree> {
    pub span: ::treesitter_types::Span,
    pub children: Identifier<'tree>,
}
impl<'tree> ::treesitter_types::FromNode<'tree> for GotoStatement<'tree> {
    #[allow(clippy::match_single_binding, clippy::suspicious_else_formatting)]
    fn from_node(
        node: ::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        debug_assert_eq!(node.kind(), "goto_statement");
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
                                        <Identifier as ::treesitter_types::FromNode>::from_node(
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
                                            <Identifier as ::treesitter_types::FromNode>::from_node(
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
                <Identifier as ::treesitter_types::FromNode>::from_node(child, src)?
            },
        })
    }
}
impl ::treesitter_types::Spanned for GotoStatement<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        self.span
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct IfStatement<'tree> {
    pub span: ::treesitter_types::Span,
    pub alternative: ::std::vec::Vec<IfStatementAlternative<'tree>>,
    pub condition: Expression<'tree>,
    pub consequence: ::core::option::Option<Block<'tree>>,
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
            alternative: {
                let mut cursor = node.walk();
                let mut items = ::std::vec::Vec::new();
                for child in node.children_by_field_name("alternative", &mut cursor) {
                    items.push(
                        <IfStatementAlternative as ::treesitter_types::FromNode>::from_node(
                            child, src,
                        )?,
                    );
                }
                items
            },
            condition: {
                let child = node.child_by_field_name("condition").ok_or_else(|| {
                    ::treesitter_types::ParseError::missing_field("condition", node)
                })?;
                <Expression as ::treesitter_types::FromNode>::from_node(child, src)?
            },
            consequence: match node.child_by_field_name("consequence") {
                Some(child) => Some(<Block as ::treesitter_types::FromNode>::from_node(
                    child, src,
                )?),
                None => None,
            },
        })
    }
}
impl ::treesitter_types::Spanned for IfStatement<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        self.span
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ImplicitVariableDeclaration<'tree> {
    pub span: ::treesitter_types::Span,
    pub attribute: ::core::option::Option<Attribute<'tree>>,
}
impl<'tree> ::treesitter_types::FromNode<'tree> for ImplicitVariableDeclaration<'tree> {
    #[allow(clippy::match_single_binding, clippy::suspicious_else_formatting)]
    fn from_node(
        node: ::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        debug_assert_eq!(node.kind(), "implicit_variable_declaration");
        Ok(Self {
            span: ::treesitter_types::Span::from(node),
            attribute: match node.child_by_field_name("attribute") {
                Some(child) => Some(<Attribute as ::treesitter_types::FromNode>::from_node(
                    child, src,
                )?),
                None => None,
            },
        })
    }
}
impl ::treesitter_types::Spanned for ImplicitVariableDeclaration<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        self.span
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct LabelStatement<'tree> {
    pub span: ::treesitter_types::Span,
    pub children: Identifier<'tree>,
}
impl<'tree> ::treesitter_types::FromNode<'tree> for LabelStatement<'tree> {
    #[allow(clippy::match_single_binding, clippy::suspicious_else_formatting)]
    fn from_node(
        node: ::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        debug_assert_eq!(node.kind(), "label_statement");
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
                                        <Identifier as ::treesitter_types::FromNode>::from_node(
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
                                            <Identifier as ::treesitter_types::FromNode>::from_node(
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
                <Identifier as ::treesitter_types::FromNode>::from_node(child, src)?
            },
        })
    }
}
impl ::treesitter_types::Spanned for LabelStatement<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        self.span
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct MethodIndexExpression<'tree> {
    pub span: ::treesitter_types::Span,
    pub method: Identifier<'tree>,
    pub table: MethodIndexExpressionTable<'tree>,
}
impl<'tree> ::treesitter_types::FromNode<'tree> for MethodIndexExpression<'tree> {
    #[allow(clippy::match_single_binding, clippy::suspicious_else_formatting)]
    fn from_node(
        node: ::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        debug_assert_eq!(node.kind(), "method_index_expression");
        Ok(Self {
            span: ::treesitter_types::Span::from(node),
            method: {
                let child = node
                    .child_by_field_name("method")
                    .ok_or_else(|| ::treesitter_types::ParseError::missing_field("method", node))?;
                <Identifier as ::treesitter_types::FromNode>::from_node(child, src)?
            },
            table: {
                let child = node
                    .child_by_field_name("table")
                    .ok_or_else(|| ::treesitter_types::ParseError::missing_field("table", node))?;
                <MethodIndexExpressionTable as ::treesitter_types::FromNode>::from_node(child, src)?
            },
        })
    }
}
impl ::treesitter_types::Spanned for MethodIndexExpression<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        self.span
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Parameters<'tree> {
    pub span: ::treesitter_types::Span,
    pub name: ::std::vec::Vec<Identifier<'tree>>,
    pub children: ::core::option::Option<VarargExpression<'tree>>,
}
impl<'tree> ::treesitter_types::FromNode<'tree> for Parameters<'tree> {
    #[allow(clippy::match_single_binding, clippy::suspicious_else_formatting)]
    fn from_node(
        node: ::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        debug_assert_eq!(node.kind(), "parameters");
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
                        <VarargExpression as ::treesitter_types::FromNode>::from_node(child, src)?,
                    ),
                    None => None,
                }
            },
        })
    }
}
impl ::treesitter_types::Spanned for Parameters<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        self.span
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ParenthesizedExpression<'tree> {
    pub span: ::treesitter_types::Span,
    pub children: Expression<'tree>,
}
impl<'tree> ::treesitter_types::FromNode<'tree> for ParenthesizedExpression<'tree> {
    #[allow(clippy::match_single_binding, clippy::suspicious_else_formatting)]
    fn from_node(
        node: ::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        debug_assert_eq!(node.kind(), "parenthesized_expression");
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
                                        <Expression as ::treesitter_types::FromNode>::from_node(
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
                                            <Expression as ::treesitter_types::FromNode>::from_node(
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
                <Expression as ::treesitter_types::FromNode>::from_node(child, src)?
            },
        })
    }
}
impl ::treesitter_types::Spanned for ParenthesizedExpression<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        self.span
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct RepeatStatement<'tree> {
    pub span: ::treesitter_types::Span,
    pub body: ::core::option::Option<Block<'tree>>,
    pub condition: Expression<'tree>,
}
impl<'tree> ::treesitter_types::FromNode<'tree> for RepeatStatement<'tree> {
    #[allow(clippy::match_single_binding, clippy::suspicious_else_formatting)]
    fn from_node(
        node: ::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        debug_assert_eq!(node.kind(), "repeat_statement");
        Ok(Self {
            span: ::treesitter_types::Span::from(node),
            body: match node.child_by_field_name("body") {
                Some(child) => Some(<Block as ::treesitter_types::FromNode>::from_node(
                    child, src,
                )?),
                None => None,
            },
            condition: {
                let child = node.child_by_field_name("condition").ok_or_else(|| {
                    ::treesitter_types::ParseError::missing_field("condition", node)
                })?;
                <Expression as ::treesitter_types::FromNode>::from_node(child, src)?
            },
        })
    }
}
impl ::treesitter_types::Spanned for RepeatStatement<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        self.span
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ReturnStatement<'tree> {
    pub span: ::treesitter_types::Span,
    pub children: ::core::option::Option<ExpressionList<'tree>>,
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
                match non_field_children.first() {
                    Some(&child) => Some(
                        <ExpressionList as ::treesitter_types::FromNode>::from_node(child, src)?,
                    ),
                    None => None,
                }
            },
        })
    }
}
impl ::treesitter_types::Spanned for ReturnStatement<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        self.span
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct String<'tree> {
    pub span: ::treesitter_types::Span,
    pub content: ::core::option::Option<StringContent<'tree>>,
    pub end: StringEnd,
    pub start: StringStart,
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
            content: match node.child_by_field_name("content") {
                Some(child) => Some(<StringContent as ::treesitter_types::FromNode>::from_node(
                    child, src,
                )?),
                None => None,
            },
            end: {
                let child = node
                    .child_by_field_name("end")
                    .ok_or_else(|| ::treesitter_types::ParseError::missing_field("end", node))?;
                <StringEnd as ::treesitter_types::FromNode>::from_node(child, src)?
            },
            start: {
                let child = node
                    .child_by_field_name("start")
                    .ok_or_else(|| ::treesitter_types::ParseError::missing_field("start", node))?;
                <StringStart as ::treesitter_types::FromNode>::from_node(child, src)?
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
pub struct StringContent<'tree> {
    pub span: ::treesitter_types::Span,
    pub children: ::std::vec::Vec<EscapeSequence<'tree>>,
}
impl<'tree> ::treesitter_types::FromNode<'tree> for StringContent<'tree> {
    #[allow(clippy::match_single_binding, clippy::suspicious_else_formatting)]
    fn from_node(
        node: ::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        debug_assert_eq!(node.kind(), "string_content");
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
                    items.push(<EscapeSequence as ::treesitter_types::FromNode>::from_node(
                        child, src,
                    )?);
                }
                items
            },
        })
    }
}
impl ::treesitter_types::Spanned for StringContent<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        self.span
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct TableConstructor<'tree> {
    pub span: ::treesitter_types::Span,
    pub children: ::std::vec::Vec<Field<'tree>>,
}
impl<'tree> ::treesitter_types::FromNode<'tree> for TableConstructor<'tree> {
    #[allow(clippy::match_single_binding, clippy::suspicious_else_formatting)]
    fn from_node(
        node: ::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        debug_assert_eq!(node.kind(), "table_constructor");
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
                    items.push(<Field as ::treesitter_types::FromNode>::from_node(
                        child, src,
                    )?);
                }
                items
            },
        })
    }
}
impl ::treesitter_types::Spanned for TableConstructor<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        self.span
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct UnaryExpression<'tree> {
    pub span: ::treesitter_types::Span,
    pub operand: Expression<'tree>,
    pub operator: UnaryExpressionOperator,
}
impl<'tree> ::treesitter_types::FromNode<'tree> for UnaryExpression<'tree> {
    #[allow(clippy::match_single_binding, clippy::suspicious_else_formatting)]
    fn from_node(
        node: ::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        debug_assert_eq!(node.kind(), "unary_expression");
        Ok(Self {
            span: ::treesitter_types::Span::from(node),
            operand: {
                let child = node.child_by_field_name("operand").ok_or_else(|| {
                    ::treesitter_types::ParseError::missing_field("operand", node)
                })?;
                <Expression as ::treesitter_types::FromNode>::from_node(child, src)?
            },
            operator: {
                let child = node.child_by_field_name("operator").ok_or_else(|| {
                    ::treesitter_types::ParseError::missing_field("operator", node)
                })?;
                <UnaryExpressionOperator as ::treesitter_types::FromNode>::from_node(child, src)?
            },
        })
    }
}
impl ::treesitter_types::Spanned for UnaryExpression<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        self.span
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct VariableDeclaration<'tree> {
    pub span: ::treesitter_types::Span,
    pub children: VariableDeclarationChildren<'tree>,
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
                                        <VariableDeclarationChildren as ::treesitter_types::FromNode>::from_node(
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
                                            <VariableDeclarationChildren as ::treesitter_types::FromNode>::from_node(
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
                <VariableDeclarationChildren as ::treesitter_types::FromNode>::from_node(
                    child, src,
                )?
            },
        })
    }
}
impl ::treesitter_types::Spanned for VariableDeclaration<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        self.span
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct VariableList<'tree> {
    pub span: ::treesitter_types::Span,
    pub attribute: ::std::vec::Vec<Attribute<'tree>>,
    pub name: ::std::vec::Vec<Variable<'tree>>,
}
impl<'tree> ::treesitter_types::FromNode<'tree> for VariableList<'tree> {
    #[allow(clippy::match_single_binding, clippy::suspicious_else_formatting)]
    fn from_node(
        node: ::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        debug_assert_eq!(node.kind(), "variable_list");
        Ok(Self {
            span: ::treesitter_types::Span::from(node),
            attribute: {
                let mut cursor = node.walk();
                let mut items = ::std::vec::Vec::new();
                for child in node.children_by_field_name("attribute", &mut cursor) {
                    items.push(<Attribute as ::treesitter_types::FromNode>::from_node(
                        child, src,
                    )?);
                }
                items
            },
            name: {
                let mut cursor = node.walk();
                let mut items = ::std::vec::Vec::new();
                for child in node.children_by_field_name("name", &mut cursor) {
                    items.push(<Variable as ::treesitter_types::FromNode>::from_node(
                        child, src,
                    )?);
                }
                items
            },
        })
    }
}
impl ::treesitter_types::Spanned for VariableList<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        self.span
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct WhileStatement<'tree> {
    pub span: ::treesitter_types::Span,
    pub body: ::core::option::Option<Block<'tree>>,
    pub condition: Expression<'tree>,
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
            body: match node.child_by_field_name("body") {
                Some(child) => Some(<Block as ::treesitter_types::FromNode>::from_node(
                    child, src,
                )?),
                None => None,
            },
            condition: {
                let child = node.child_by_field_name("condition").ok_or_else(|| {
                    ::treesitter_types::ParseError::missing_field("condition", node)
                })?;
                <Expression as ::treesitter_types::FromNode>::from_node(child, src)?
            },
        })
    }
}
impl ::treesitter_types::Spanned for WhileStatement<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        self.span
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct BreakStatement<'tree> {
    pub span: ::treesitter_types::Span,
    text: &'tree str,
}
impl<'tree> ::treesitter_types::FromNode<'tree> for BreakStatement<'tree> {
    fn from_node(
        node: ::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        debug_assert_eq!(node.kind(), "break_statement");
        Ok(Self {
            span: ::treesitter_types::Span::from(node),
            text: node.utf8_text(src)?,
        })
    }
}
impl<'tree> ::treesitter_types::LeafNode<'tree> for BreakStatement<'tree> {
    fn text(&self) -> &'tree str {
        self.text
    }
}
impl ::treesitter_types::Spanned for BreakStatement<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        self.span
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CommentContent<'tree> {
    pub span: ::treesitter_types::Span,
    text: &'tree str,
}
impl<'tree> ::treesitter_types::FromNode<'tree> for CommentContent<'tree> {
    fn from_node(
        node: ::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        debug_assert_eq!(node.kind(), "comment_content");
        Ok(Self {
            span: ::treesitter_types::Span::from(node),
            text: node.utf8_text(src)?,
        })
    }
}
impl<'tree> ::treesitter_types::LeafNode<'tree> for CommentContent<'tree> {
    fn text(&self) -> &'tree str {
        self.text
    }
}
impl ::treesitter_types::Spanned for CommentContent<'_> {
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
pub struct HashBangLine<'tree> {
    pub span: ::treesitter_types::Span,
    text: &'tree str,
}
impl<'tree> ::treesitter_types::FromNode<'tree> for HashBangLine<'tree> {
    fn from_node(
        node: ::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        debug_assert_eq!(node.kind(), "hash_bang_line");
        Ok(Self {
            span: ::treesitter_types::Span::from(node),
            text: node.utf8_text(src)?,
        })
    }
}
impl<'tree> ::treesitter_types::LeafNode<'tree> for HashBangLine<'tree> {
    fn text(&self) -> &'tree str {
        self.text
    }
}
impl ::treesitter_types::Spanned for HashBangLine<'_> {
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
pub struct VarargExpression<'tree> {
    pub span: ::treesitter_types::Span,
    text: &'tree str,
}
impl<'tree> ::treesitter_types::FromNode<'tree> for VarargExpression<'tree> {
    fn from_node(
        node: ::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        debug_assert_eq!(node.kind(), "vararg_expression");
        Ok(Self {
            span: ::treesitter_types::Span::from(node),
            text: node.utf8_text(src)?,
        })
    }
}
impl<'tree> ::treesitter_types::LeafNode<'tree> for VarargExpression<'tree> {
    fn text(&self) -> &'tree str {
        self.text
    }
}
impl ::treesitter_types::Spanned for VarargExpression<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        self.span
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum AssignmentStatementOperator {
    Eq(::treesitter_types::Span),
}
impl<'tree> ::treesitter_types::FromNode<'tree> for AssignmentStatementOperator {
    #[allow(clippy::collapsible_else_if)]
    fn from_node(
        node: ::tree_sitter::Node<'tree>,
        _src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        match node.kind() {
            "=" => Ok(Self::Eq(::treesitter_types::Span::from(node))),
            other => Err(::treesitter_types::ParseError::unexpected_kind(other, node)),
        }
    }
}
impl ::treesitter_types::Spanned for AssignmentStatementOperator {
    fn span(&self) -> ::treesitter_types::Span {
        match self {
            Self::Eq(span) => *span,
        }
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum AssignmentStatementChildren<'tree> {
    ExpressionList(::std::boxed::Box<ExpressionList<'tree>>),
    VariableList(::std::boxed::Box<VariableList<'tree>>),
}
impl<'tree> ::treesitter_types::FromNode<'tree> for AssignmentStatementChildren<'tree> {
    #[allow(clippy::collapsible_else_if)]
    fn from_node(
        node: ::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        match node.kind() {
            "expression_list" => Ok(Self::ExpressionList(::std::boxed::Box::new(
                <ExpressionList as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "variable_list" => Ok(Self::VariableList(::std::boxed::Box::new(
                <VariableList as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            other => Err(::treesitter_types::ParseError::unexpected_kind(other, node)),
        }
    }
}
impl ::treesitter_types::Spanned for AssignmentStatementChildren<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        match self {
            Self::ExpressionList(inner) => inner.span(),
            Self::VariableList(inner) => inner.span(),
        }
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum BinaryExpressionOperator {
    Percent(::treesitter_types::Span),
    Amp(::treesitter_types::Span),
    Star(::treesitter_types::Span),
    Plus(::treesitter_types::Span),
    Minus(::treesitter_types::Span),
    DotDot(::treesitter_types::Span),
    Slash(::treesitter_types::Span),
    SlashSlash(::treesitter_types::Span),
    Lt(::treesitter_types::Span),
    Shl(::treesitter_types::Span),
    LtEq(::treesitter_types::Span),
    EqEq(::treesitter_types::Span),
    Gt(::treesitter_types::Span),
    GtEq(::treesitter_types::Span),
    Shr(::treesitter_types::Span),
    Caret(::treesitter_types::Span),
    And(::treesitter_types::Span),
    Or(::treesitter_types::Span),
    Pipe(::treesitter_types::Span),
    Tilde(::treesitter_types::Span),
    TildeEq(::treesitter_types::Span),
}
impl<'tree> ::treesitter_types::FromNode<'tree> for BinaryExpressionOperator {
    #[allow(clippy::collapsible_else_if)]
    fn from_node(
        node: ::tree_sitter::Node<'tree>,
        _src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        match node.kind() {
            "%" => Ok(Self::Percent(::treesitter_types::Span::from(node))),
            "&" => Ok(Self::Amp(::treesitter_types::Span::from(node))),
            "*" => Ok(Self::Star(::treesitter_types::Span::from(node))),
            "+" => Ok(Self::Plus(::treesitter_types::Span::from(node))),
            "-" => Ok(Self::Minus(::treesitter_types::Span::from(node))),
            ".." => Ok(Self::DotDot(::treesitter_types::Span::from(node))),
            "/" => Ok(Self::Slash(::treesitter_types::Span::from(node))),
            "//" => Ok(Self::SlashSlash(::treesitter_types::Span::from(node))),
            "<" => Ok(Self::Lt(::treesitter_types::Span::from(node))),
            "<<" => Ok(Self::Shl(::treesitter_types::Span::from(node))),
            "<=" => Ok(Self::LtEq(::treesitter_types::Span::from(node))),
            "==" => Ok(Self::EqEq(::treesitter_types::Span::from(node))),
            ">" => Ok(Self::Gt(::treesitter_types::Span::from(node))),
            ">=" => Ok(Self::GtEq(::treesitter_types::Span::from(node))),
            ">>" => Ok(Self::Shr(::treesitter_types::Span::from(node))),
            "^" => Ok(Self::Caret(::treesitter_types::Span::from(node))),
            "and" => Ok(Self::And(::treesitter_types::Span::from(node))),
            "or" => Ok(Self::Or(::treesitter_types::Span::from(node))),
            "|" => Ok(Self::Pipe(::treesitter_types::Span::from(node))),
            "~" => Ok(Self::Tilde(::treesitter_types::Span::from(node))),
            "~=" => Ok(Self::TildeEq(::treesitter_types::Span::from(node))),
            other => Err(::treesitter_types::ParseError::unexpected_kind(other, node)),
        }
    }
}
impl ::treesitter_types::Spanned for BinaryExpressionOperator {
    fn span(&self) -> ::treesitter_types::Span {
        match self {
            Self::Percent(span) => *span,
            Self::Amp(span) => *span,
            Self::Star(span) => *span,
            Self::Plus(span) => *span,
            Self::Minus(span) => *span,
            Self::DotDot(span) => *span,
            Self::Slash(span) => *span,
            Self::SlashSlash(span) => *span,
            Self::Lt(span) => *span,
            Self::Shl(span) => *span,
            Self::LtEq(span) => *span,
            Self::EqEq(span) => *span,
            Self::Gt(span) => *span,
            Self::GtEq(span) => *span,
            Self::Shr(span) => *span,
            Self::Caret(span) => *span,
            Self::And(span) => *span,
            Self::Or(span) => *span,
            Self::Pipe(span) => *span,
            Self::Tilde(span) => *span,
            Self::TildeEq(span) => *span,
        }
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum BlockChildren<'tree> {
    ReturnStatement(::std::boxed::Box<ReturnStatement<'tree>>),
    Statement(::std::boxed::Box<Statement<'tree>>),
}
impl<'tree> ::treesitter_types::FromNode<'tree> for BlockChildren<'tree> {
    #[allow(clippy::collapsible_else_if)]
    fn from_node(
        node: ::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        match node.kind() {
            "return_statement" => Ok(Self::ReturnStatement(::std::boxed::Box::new(
                <ReturnStatement as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            _other => {
                if let Ok(v) = <Statement as ::treesitter_types::FromNode>::from_node(node, src) {
                    Ok(Self::Statement(::std::boxed::Box::new(v)))
                } else {
                    Err(::treesitter_types::ParseError::unexpected_kind(
                        _other, node,
                    ))
                }
            }
        }
    }
}
impl ::treesitter_types::Spanned for BlockChildren<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        match self {
            Self::ReturnStatement(inner) => inner.span(),
            Self::Statement(inner) => inner.span(),
        }
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum BracketIndexExpressionTable<'tree> {
    FunctionCall(::std::boxed::Box<FunctionCall<'tree>>),
    ParenthesizedExpression(::std::boxed::Box<ParenthesizedExpression<'tree>>),
    Variable(::std::boxed::Box<Variable<'tree>>),
}
impl<'tree> ::treesitter_types::FromNode<'tree> for BracketIndexExpressionTable<'tree> {
    #[allow(clippy::collapsible_else_if)]
    fn from_node(
        node: ::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        match node.kind() {
            "function_call" => Ok(Self::FunctionCall(::std::boxed::Box::new(
                <FunctionCall as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "parenthesized_expression" => {
                Ok(Self::ParenthesizedExpression(::std::boxed::Box::new(
                    <ParenthesizedExpression as ::treesitter_types::FromNode>::from_node(
                        node, src,
                    )?,
                )))
            }
            _other => {
                if let Ok(v) = <Variable as ::treesitter_types::FromNode>::from_node(node, src) {
                    Ok(Self::Variable(::std::boxed::Box::new(v)))
                } else {
                    Err(::treesitter_types::ParseError::unexpected_kind(
                        _other, node,
                    ))
                }
            }
        }
    }
}
impl ::treesitter_types::Spanned for BracketIndexExpressionTable<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        match self {
            Self::FunctionCall(inner) => inner.span(),
            Self::ParenthesizedExpression(inner) => inner.span(),
            Self::Variable(inner) => inner.span(),
        }
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ChunkChildren<'tree> {
    HashBangLine(::std::boxed::Box<HashBangLine<'tree>>),
    ReturnStatement(::std::boxed::Box<ReturnStatement<'tree>>),
    Statement(::std::boxed::Box<Statement<'tree>>),
}
impl<'tree> ::treesitter_types::FromNode<'tree> for ChunkChildren<'tree> {
    #[allow(clippy::collapsible_else_if)]
    fn from_node(
        node: ::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        match node.kind() {
            "hash_bang_line" => Ok(Self::HashBangLine(::std::boxed::Box::new(
                <HashBangLine as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "return_statement" => Ok(Self::ReturnStatement(::std::boxed::Box::new(
                <ReturnStatement as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            _other => {
                if let Ok(v) = <Statement as ::treesitter_types::FromNode>::from_node(node, src) {
                    Ok(Self::Statement(::std::boxed::Box::new(v)))
                } else {
                    Err(::treesitter_types::ParseError::unexpected_kind(
                        _other, node,
                    ))
                }
            }
        }
    }
}
impl ::treesitter_types::Spanned for ChunkChildren<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        match self {
            Self::HashBangLine(inner) => inner.span(),
            Self::ReturnStatement(inner) => inner.span(),
            Self::Statement(inner) => inner.span(),
        }
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum CommentEnd {
    RBracketRBracket(::treesitter_types::Span),
}
impl<'tree> ::treesitter_types::FromNode<'tree> for CommentEnd {
    #[allow(clippy::collapsible_else_if)]
    fn from_node(
        node: ::tree_sitter::Node<'tree>,
        _src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        match node.kind() {
            "]]" => Ok(Self::RBracketRBracket(::treesitter_types::Span::from(node))),
            other => Err(::treesitter_types::ParseError::unexpected_kind(other, node)),
        }
    }
}
impl ::treesitter_types::Spanned for CommentEnd {
    fn span(&self) -> ::treesitter_types::Span {
        match self {
            Self::RBracketRBracket(span) => *span,
        }
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum CommentStart {
    MinusMinus(::treesitter_types::Span),
    LBracketLBracket(::treesitter_types::Span),
}
impl<'tree> ::treesitter_types::FromNode<'tree> for CommentStart {
    #[allow(clippy::collapsible_else_if)]
    fn from_node(
        node: ::tree_sitter::Node<'tree>,
        _src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        match node.kind() {
            "--" => Ok(Self::MinusMinus(::treesitter_types::Span::from(node))),
            "[[" => Ok(Self::LBracketLBracket(::treesitter_types::Span::from(node))),
            other => Err(::treesitter_types::ParseError::unexpected_kind(other, node)),
        }
    }
}
impl ::treesitter_types::Spanned for CommentStart {
    fn span(&self) -> ::treesitter_types::Span {
        match self {
            Self::MinusMinus(span) => *span,
            Self::LBracketLBracket(span) => *span,
        }
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum DotIndexExpressionTable<'tree> {
    FunctionCall(::std::boxed::Box<FunctionCall<'tree>>),
    ParenthesizedExpression(::std::boxed::Box<ParenthesizedExpression<'tree>>),
    Variable(::std::boxed::Box<Variable<'tree>>),
}
impl<'tree> ::treesitter_types::FromNode<'tree> for DotIndexExpressionTable<'tree> {
    #[allow(clippy::collapsible_else_if)]
    fn from_node(
        node: ::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        match node.kind() {
            "function_call" => Ok(Self::FunctionCall(::std::boxed::Box::new(
                <FunctionCall as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "parenthesized_expression" => {
                Ok(Self::ParenthesizedExpression(::std::boxed::Box::new(
                    <ParenthesizedExpression as ::treesitter_types::FromNode>::from_node(
                        node, src,
                    )?,
                )))
            }
            _other => {
                if let Ok(v) = <Variable as ::treesitter_types::FromNode>::from_node(node, src) {
                    Ok(Self::Variable(::std::boxed::Box::new(v)))
                } else {
                    Err(::treesitter_types::ParseError::unexpected_kind(
                        _other, node,
                    ))
                }
            }
        }
    }
}
impl ::treesitter_types::Spanned for DotIndexExpressionTable<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        match self {
            Self::FunctionCall(inner) => inner.span(),
            Self::ParenthesizedExpression(inner) => inner.span(),
            Self::Variable(inner) => inner.span(),
        }
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum FieldName<'tree> {
    Expression(::std::boxed::Box<Expression<'tree>>),
    Global(::treesitter_types::Span),
    Identifier(::std::boxed::Box<Identifier<'tree>>),
}
impl<'tree> ::treesitter_types::FromNode<'tree> for FieldName<'tree> {
    #[allow(clippy::collapsible_else_if)]
    fn from_node(
        node: ::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        match node.kind() {
            "global" => Ok(Self::Global(::treesitter_types::Span::from(node))),
            "identifier" => Ok(Self::Identifier(::std::boxed::Box::new(
                <Identifier as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            _other => {
                if let Ok(v) = <Expression as ::treesitter_types::FromNode>::from_node(node, src) {
                    Ok(Self::Expression(::std::boxed::Box::new(v)))
                } else {
                    Err(::treesitter_types::ParseError::unexpected_kind(
                        _other, node,
                    ))
                }
            }
        }
    }
}
impl ::treesitter_types::Spanned for FieldName<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        match self {
            Self::Expression(inner) => inner.span(),
            Self::Global(span) => *span,
            Self::Identifier(inner) => inner.span(),
        }
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum FieldOperator {
    Eq(::treesitter_types::Span),
}
impl<'tree> ::treesitter_types::FromNode<'tree> for FieldOperator {
    #[allow(clippy::collapsible_else_if)]
    fn from_node(
        node: ::tree_sitter::Node<'tree>,
        _src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        match node.kind() {
            "=" => Ok(Self::Eq(::treesitter_types::Span::from(node))),
            other => Err(::treesitter_types::ParseError::unexpected_kind(other, node)),
        }
    }
}
impl ::treesitter_types::Spanned for FieldOperator {
    fn span(&self) -> ::treesitter_types::Span {
        match self {
            Self::Eq(span) => *span,
        }
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ForGenericClauseChildren<'tree> {
    ExpressionList(::std::boxed::Box<ExpressionList<'tree>>),
    VariableList(::std::boxed::Box<VariableList<'tree>>),
}
impl<'tree> ::treesitter_types::FromNode<'tree> for ForGenericClauseChildren<'tree> {
    #[allow(clippy::collapsible_else_if)]
    fn from_node(
        node: ::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        match node.kind() {
            "expression_list" => Ok(Self::ExpressionList(::std::boxed::Box::new(
                <ExpressionList as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "variable_list" => Ok(Self::VariableList(::std::boxed::Box::new(
                <VariableList as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            other => Err(::treesitter_types::ParseError::unexpected_kind(other, node)),
        }
    }
}
impl ::treesitter_types::Spanned for ForGenericClauseChildren<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        match self {
            Self::ExpressionList(inner) => inner.span(),
            Self::VariableList(inner) => inner.span(),
        }
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ForNumericClauseOperator {
    Eq(::treesitter_types::Span),
}
impl<'tree> ::treesitter_types::FromNode<'tree> for ForNumericClauseOperator {
    #[allow(clippy::collapsible_else_if)]
    fn from_node(
        node: ::tree_sitter::Node<'tree>,
        _src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        match node.kind() {
            "=" => Ok(Self::Eq(::treesitter_types::Span::from(node))),
            other => Err(::treesitter_types::ParseError::unexpected_kind(other, node)),
        }
    }
}
impl ::treesitter_types::Spanned for ForNumericClauseOperator {
    fn span(&self) -> ::treesitter_types::Span {
        match self {
            Self::Eq(span) => *span,
        }
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ForStatementClause<'tree> {
    ForGenericClause(::std::boxed::Box<ForGenericClause<'tree>>),
    ForNumericClause(::std::boxed::Box<ForNumericClause<'tree>>),
}
impl<'tree> ::treesitter_types::FromNode<'tree> for ForStatementClause<'tree> {
    #[allow(clippy::collapsible_else_if)]
    fn from_node(
        node: ::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        match node.kind() {
            "for_generic_clause" => Ok(Self::ForGenericClause(::std::boxed::Box::new(
                <ForGenericClause as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "for_numeric_clause" => Ok(Self::ForNumericClause(::std::boxed::Box::new(
                <ForNumericClause as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            other => Err(::treesitter_types::ParseError::unexpected_kind(other, node)),
        }
    }
}
impl ::treesitter_types::Spanned for ForStatementClause<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        match self {
            Self::ForGenericClause(inner) => inner.span(),
            Self::ForNumericClause(inner) => inner.span(),
        }
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum FunctionCallName<'tree> {
    FunctionCall(::std::boxed::Box<FunctionCall<'tree>>),
    MethodIndexExpression(::std::boxed::Box<MethodIndexExpression<'tree>>),
    ParenthesizedExpression(::std::boxed::Box<ParenthesizedExpression<'tree>>),
    Variable(::std::boxed::Box<Variable<'tree>>),
}
impl<'tree> ::treesitter_types::FromNode<'tree> for FunctionCallName<'tree> {
    #[allow(clippy::collapsible_else_if)]
    fn from_node(
        node: ::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        match node.kind() {
            "function_call" => Ok(Self::FunctionCall(::std::boxed::Box::new(
                <FunctionCall as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "method_index_expression" => Ok(Self::MethodIndexExpression(::std::boxed::Box::new(
                <MethodIndexExpression as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "parenthesized_expression" => {
                Ok(Self::ParenthesizedExpression(::std::boxed::Box::new(
                    <ParenthesizedExpression as ::treesitter_types::FromNode>::from_node(
                        node, src,
                    )?,
                )))
            }
            _other => {
                if let Ok(v) = <Variable as ::treesitter_types::FromNode>::from_node(node, src) {
                    Ok(Self::Variable(::std::boxed::Box::new(v)))
                } else {
                    Err(::treesitter_types::ParseError::unexpected_kind(
                        _other, node,
                    ))
                }
            }
        }
    }
}
impl ::treesitter_types::Spanned for FunctionCallName<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        match self {
            Self::FunctionCall(inner) => inner.span(),
            Self::MethodIndexExpression(inner) => inner.span(),
            Self::ParenthesizedExpression(inner) => inner.span(),
            Self::Variable(inner) => inner.span(),
        }
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum FunctionDeclarationName<'tree> {
    DotIndexExpression(::std::boxed::Box<DotIndexExpression<'tree>>),
    Identifier(::std::boxed::Box<Identifier<'tree>>),
    MethodIndexExpression(::std::boxed::Box<MethodIndexExpression<'tree>>),
}
impl<'tree> ::treesitter_types::FromNode<'tree> for FunctionDeclarationName<'tree> {
    #[allow(clippy::collapsible_else_if)]
    fn from_node(
        node: ::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        match node.kind() {
            "dot_index_expression" => Ok(Self::DotIndexExpression(::std::boxed::Box::new(
                <DotIndexExpression as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "identifier" => Ok(Self::Identifier(::std::boxed::Box::new(
                <Identifier as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "method_index_expression" => Ok(Self::MethodIndexExpression(::std::boxed::Box::new(
                <MethodIndexExpression as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            other => Err(::treesitter_types::ParseError::unexpected_kind(other, node)),
        }
    }
}
impl ::treesitter_types::Spanned for FunctionDeclarationName<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        match self {
            Self::DotIndexExpression(inner) => inner.span(),
            Self::Identifier(inner) => inner.span(),
            Self::MethodIndexExpression(inner) => inner.span(),
        }
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum IfStatementAlternative<'tree> {
    ElseStatement(::std::boxed::Box<ElseStatement<'tree>>),
    ElseifStatement(::std::boxed::Box<ElseifStatement<'tree>>),
}
impl<'tree> ::treesitter_types::FromNode<'tree> for IfStatementAlternative<'tree> {
    #[allow(clippy::collapsible_else_if)]
    fn from_node(
        node: ::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        match node.kind() {
            "else_statement" => Ok(Self::ElseStatement(::std::boxed::Box::new(
                <ElseStatement as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "elseif_statement" => Ok(Self::ElseifStatement(::std::boxed::Box::new(
                <ElseifStatement as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            other => Err(::treesitter_types::ParseError::unexpected_kind(other, node)),
        }
    }
}
impl ::treesitter_types::Spanned for IfStatementAlternative<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        match self {
            Self::ElseStatement(inner) => inner.span(),
            Self::ElseifStatement(inner) => inner.span(),
        }
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum MethodIndexExpressionTable<'tree> {
    FunctionCall(::std::boxed::Box<FunctionCall<'tree>>),
    ParenthesizedExpression(::std::boxed::Box<ParenthesizedExpression<'tree>>),
    Variable(::std::boxed::Box<Variable<'tree>>),
}
impl<'tree> ::treesitter_types::FromNode<'tree> for MethodIndexExpressionTable<'tree> {
    #[allow(clippy::collapsible_else_if)]
    fn from_node(
        node: ::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        match node.kind() {
            "function_call" => Ok(Self::FunctionCall(::std::boxed::Box::new(
                <FunctionCall as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "parenthesized_expression" => {
                Ok(Self::ParenthesizedExpression(::std::boxed::Box::new(
                    <ParenthesizedExpression as ::treesitter_types::FromNode>::from_node(
                        node, src,
                    )?,
                )))
            }
            _other => {
                if let Ok(v) = <Variable as ::treesitter_types::FromNode>::from_node(node, src) {
                    Ok(Self::Variable(::std::boxed::Box::new(v)))
                } else {
                    Err(::treesitter_types::ParseError::unexpected_kind(
                        _other, node,
                    ))
                }
            }
        }
    }
}
impl ::treesitter_types::Spanned for MethodIndexExpressionTable<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        match self {
            Self::FunctionCall(inner) => inner.span(),
            Self::ParenthesizedExpression(inner) => inner.span(),
            Self::Variable(inner) => inner.span(),
        }
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum StringEnd {
    DoubleQuote(::treesitter_types::Span),
    SingleQuote(::treesitter_types::Span),
    RBracketRBracket(::treesitter_types::Span),
}
impl<'tree> ::treesitter_types::FromNode<'tree> for StringEnd {
    #[allow(clippy::collapsible_else_if)]
    fn from_node(
        node: ::tree_sitter::Node<'tree>,
        _src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        match node.kind() {
            "\"" => Ok(Self::DoubleQuote(::treesitter_types::Span::from(node))),
            "'" => Ok(Self::SingleQuote(::treesitter_types::Span::from(node))),
            "]]" => Ok(Self::RBracketRBracket(::treesitter_types::Span::from(node))),
            other => Err(::treesitter_types::ParseError::unexpected_kind(other, node)),
        }
    }
}
impl ::treesitter_types::Spanned for StringEnd {
    fn span(&self) -> ::treesitter_types::Span {
        match self {
            Self::DoubleQuote(span) => *span,
            Self::SingleQuote(span) => *span,
            Self::RBracketRBracket(span) => *span,
        }
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum StringStart {
    DoubleQuote(::treesitter_types::Span),
    SingleQuote(::treesitter_types::Span),
    LBracketLBracket(::treesitter_types::Span),
}
impl<'tree> ::treesitter_types::FromNode<'tree> for StringStart {
    #[allow(clippy::collapsible_else_if)]
    fn from_node(
        node: ::tree_sitter::Node<'tree>,
        _src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        match node.kind() {
            "\"" => Ok(Self::DoubleQuote(::treesitter_types::Span::from(node))),
            "'" => Ok(Self::SingleQuote(::treesitter_types::Span::from(node))),
            "[[" => Ok(Self::LBracketLBracket(::treesitter_types::Span::from(node))),
            other => Err(::treesitter_types::ParseError::unexpected_kind(other, node)),
        }
    }
}
impl ::treesitter_types::Spanned for StringStart {
    fn span(&self) -> ::treesitter_types::Span {
        match self {
            Self::DoubleQuote(span) => *span,
            Self::SingleQuote(span) => *span,
            Self::LBracketLBracket(span) => *span,
        }
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum UnaryExpressionOperator {
    Hash(::treesitter_types::Span),
    Minus(::treesitter_types::Span),
    Not(::treesitter_types::Span),
    Tilde(::treesitter_types::Span),
}
impl<'tree> ::treesitter_types::FromNode<'tree> for UnaryExpressionOperator {
    #[allow(clippy::collapsible_else_if)]
    fn from_node(
        node: ::tree_sitter::Node<'tree>,
        _src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        match node.kind() {
            "#" => Ok(Self::Hash(::treesitter_types::Span::from(node))),
            "-" => Ok(Self::Minus(::treesitter_types::Span::from(node))),
            "not" => Ok(Self::Not(::treesitter_types::Span::from(node))),
            "~" => Ok(Self::Tilde(::treesitter_types::Span::from(node))),
            other => Err(::treesitter_types::ParseError::unexpected_kind(other, node)),
        }
    }
}
impl ::treesitter_types::Spanned for UnaryExpressionOperator {
    fn span(&self) -> ::treesitter_types::Span {
        match self {
            Self::Hash(span) => *span,
            Self::Minus(span) => *span,
            Self::Not(span) => *span,
            Self::Tilde(span) => *span,
        }
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum VariableDeclarationChildren<'tree> {
    AssignmentStatement(::std::boxed::Box<AssignmentStatement<'tree>>),
    VariableList(::std::boxed::Box<VariableList<'tree>>),
}
impl<'tree> ::treesitter_types::FromNode<'tree> for VariableDeclarationChildren<'tree> {
    #[allow(clippy::collapsible_else_if)]
    fn from_node(
        node: ::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        match node.kind() {
            "assignment_statement" => Ok(Self::AssignmentStatement(::std::boxed::Box::new(
                <AssignmentStatement as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "variable_list" => Ok(Self::VariableList(::std::boxed::Box::new(
                <VariableList as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            other => Err(::treesitter_types::ParseError::unexpected_kind(other, node)),
        }
    }
}
impl ::treesitter_types::Spanned for VariableDeclarationChildren<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        match self {
            Self::AssignmentStatement(inner) => inner.span(),
            Self::VariableList(inner) => inner.span(),
        }
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum AnyNode<'tree> {
    Declaration(Declaration<'tree>),
    Expression(Expression<'tree>),
    Statement(Statement<'tree>),
    Variable(Variable<'tree>),
    Arguments(Arguments<'tree>),
    AssignmentStatement(AssignmentStatement<'tree>),
    Attribute(Attribute<'tree>),
    BinaryExpression(BinaryExpression<'tree>),
    Block(Block<'tree>),
    BracketIndexExpression(BracketIndexExpression<'tree>),
    Chunk(Chunk<'tree>),
    Comment(Comment<'tree>),
    DoStatement(DoStatement<'tree>),
    DotIndexExpression(DotIndexExpression<'tree>),
    ElseStatement(ElseStatement<'tree>),
    ElseifStatement(ElseifStatement<'tree>),
    EmptyStatement(EmptyStatement<'tree>),
    ExpressionList(ExpressionList<'tree>),
    Field(Field<'tree>),
    ForGenericClause(ForGenericClause<'tree>),
    ForNumericClause(ForNumericClause<'tree>),
    ForStatement(ForStatement<'tree>),
    FunctionCall(FunctionCall<'tree>),
    FunctionDeclaration(FunctionDeclaration<'tree>),
    FunctionDefinition(FunctionDefinition<'tree>),
    GotoStatement(GotoStatement<'tree>),
    IfStatement(IfStatement<'tree>),
    ImplicitVariableDeclaration(ImplicitVariableDeclaration<'tree>),
    LabelStatement(LabelStatement<'tree>),
    MethodIndexExpression(MethodIndexExpression<'tree>),
    Parameters(Parameters<'tree>),
    ParenthesizedExpression(ParenthesizedExpression<'tree>),
    RepeatStatement(RepeatStatement<'tree>),
    ReturnStatement(ReturnStatement<'tree>),
    String(String<'tree>),
    StringContent(StringContent<'tree>),
    TableConstructor(TableConstructor<'tree>),
    UnaryExpression(UnaryExpression<'tree>),
    VariableDeclaration(VariableDeclaration<'tree>),
    VariableList(VariableList<'tree>),
    WhileStatement(WhileStatement<'tree>),
    BreakStatement(BreakStatement<'tree>),
    CommentContent(CommentContent<'tree>),
    EscapeSequence(EscapeSequence<'tree>),
    False(False<'tree>),
    HashBangLine(HashBangLine<'tree>),
    Identifier(Identifier<'tree>),
    Nil(Nil<'tree>),
    Number(Number<'tree>),
    True(True<'tree>),
    VarargExpression(VarargExpression<'tree>),
    Unknown(::tree_sitter::Node<'tree>),
}
impl<'tree> AnyNode<'tree> {
    pub fn from_node(node: ::tree_sitter::Node<'tree>, src: &'tree [u8]) -> Self {
        match node.kind() {
            "declaration" => <Declaration as ::treesitter_types::FromNode>::from_node(node, src)
                .map(Self::Declaration)
                .unwrap_or(Self::Unknown(node)),
            "expression" => <Expression as ::treesitter_types::FromNode>::from_node(node, src)
                .map(Self::Expression)
                .unwrap_or(Self::Unknown(node)),
            "statement" => <Statement as ::treesitter_types::FromNode>::from_node(node, src)
                .map(Self::Statement)
                .unwrap_or(Self::Unknown(node)),
            "variable" => <Variable as ::treesitter_types::FromNode>::from_node(node, src)
                .map(Self::Variable)
                .unwrap_or(Self::Unknown(node)),
            "arguments" => <Arguments as ::treesitter_types::FromNode>::from_node(node, src)
                .map(Self::Arguments)
                .unwrap_or(Self::Unknown(node)),
            "assignment_statement" => {
                <AssignmentStatement as ::treesitter_types::FromNode>::from_node(node, src)
                    .map(Self::AssignmentStatement)
                    .unwrap_or(Self::Unknown(node))
            }
            "attribute" => <Attribute as ::treesitter_types::FromNode>::from_node(node, src)
                .map(Self::Attribute)
                .unwrap_or(Self::Unknown(node)),
            "binary_expression" => {
                <BinaryExpression as ::treesitter_types::FromNode>::from_node(node, src)
                    .map(Self::BinaryExpression)
                    .unwrap_or(Self::Unknown(node))
            }
            "block" => <Block as ::treesitter_types::FromNode>::from_node(node, src)
                .map(Self::Block)
                .unwrap_or(Self::Unknown(node)),
            "bracket_index_expression" => {
                <BracketIndexExpression as ::treesitter_types::FromNode>::from_node(node, src)
                    .map(Self::BracketIndexExpression)
                    .unwrap_or(Self::Unknown(node))
            }
            "chunk" => <Chunk as ::treesitter_types::FromNode>::from_node(node, src)
                .map(Self::Chunk)
                .unwrap_or(Self::Unknown(node)),
            "comment" => <Comment as ::treesitter_types::FromNode>::from_node(node, src)
                .map(Self::Comment)
                .unwrap_or(Self::Unknown(node)),
            "do_statement" => <DoStatement as ::treesitter_types::FromNode>::from_node(node, src)
                .map(Self::DoStatement)
                .unwrap_or(Self::Unknown(node)),
            "dot_index_expression" => {
                <DotIndexExpression as ::treesitter_types::FromNode>::from_node(node, src)
                    .map(Self::DotIndexExpression)
                    .unwrap_or(Self::Unknown(node))
            }
            "else_statement" => {
                <ElseStatement as ::treesitter_types::FromNode>::from_node(node, src)
                    .map(Self::ElseStatement)
                    .unwrap_or(Self::Unknown(node))
            }
            "elseif_statement" => {
                <ElseifStatement as ::treesitter_types::FromNode>::from_node(node, src)
                    .map(Self::ElseifStatement)
                    .unwrap_or(Self::Unknown(node))
            }
            "empty_statement" => {
                <EmptyStatement as ::treesitter_types::FromNode>::from_node(node, src)
                    .map(Self::EmptyStatement)
                    .unwrap_or(Self::Unknown(node))
            }
            "expression_list" => {
                <ExpressionList as ::treesitter_types::FromNode>::from_node(node, src)
                    .map(Self::ExpressionList)
                    .unwrap_or(Self::Unknown(node))
            }
            "field" => <Field as ::treesitter_types::FromNode>::from_node(node, src)
                .map(Self::Field)
                .unwrap_or(Self::Unknown(node)),
            "for_generic_clause" => {
                <ForGenericClause as ::treesitter_types::FromNode>::from_node(node, src)
                    .map(Self::ForGenericClause)
                    .unwrap_or(Self::Unknown(node))
            }
            "for_numeric_clause" => {
                <ForNumericClause as ::treesitter_types::FromNode>::from_node(node, src)
                    .map(Self::ForNumericClause)
                    .unwrap_or(Self::Unknown(node))
            }
            "for_statement" => <ForStatement as ::treesitter_types::FromNode>::from_node(node, src)
                .map(Self::ForStatement)
                .unwrap_or(Self::Unknown(node)),
            "function_call" => <FunctionCall as ::treesitter_types::FromNode>::from_node(node, src)
                .map(Self::FunctionCall)
                .unwrap_or(Self::Unknown(node)),
            "function_declaration" => {
                <FunctionDeclaration as ::treesitter_types::FromNode>::from_node(node, src)
                    .map(Self::FunctionDeclaration)
                    .unwrap_or(Self::Unknown(node))
            }
            "function_definition" => {
                <FunctionDefinition as ::treesitter_types::FromNode>::from_node(node, src)
                    .map(Self::FunctionDefinition)
                    .unwrap_or(Self::Unknown(node))
            }
            "goto_statement" => {
                <GotoStatement as ::treesitter_types::FromNode>::from_node(node, src)
                    .map(Self::GotoStatement)
                    .unwrap_or(Self::Unknown(node))
            }
            "if_statement" => <IfStatement as ::treesitter_types::FromNode>::from_node(node, src)
                .map(Self::IfStatement)
                .unwrap_or(Self::Unknown(node)),
            "implicit_variable_declaration" => {
                <ImplicitVariableDeclaration as ::treesitter_types::FromNode>::from_node(node, src)
                    .map(Self::ImplicitVariableDeclaration)
                    .unwrap_or(Self::Unknown(node))
            }
            "label_statement" => {
                <LabelStatement as ::treesitter_types::FromNode>::from_node(node, src)
                    .map(Self::LabelStatement)
                    .unwrap_or(Self::Unknown(node))
            }
            "method_index_expression" => {
                <MethodIndexExpression as ::treesitter_types::FromNode>::from_node(node, src)
                    .map(Self::MethodIndexExpression)
                    .unwrap_or(Self::Unknown(node))
            }
            "parameters" => <Parameters as ::treesitter_types::FromNode>::from_node(node, src)
                .map(Self::Parameters)
                .unwrap_or(Self::Unknown(node)),
            "parenthesized_expression" => {
                <ParenthesizedExpression as ::treesitter_types::FromNode>::from_node(node, src)
                    .map(Self::ParenthesizedExpression)
                    .unwrap_or(Self::Unknown(node))
            }
            "repeat_statement" => {
                <RepeatStatement as ::treesitter_types::FromNode>::from_node(node, src)
                    .map(Self::RepeatStatement)
                    .unwrap_or(Self::Unknown(node))
            }
            "return_statement" => {
                <ReturnStatement as ::treesitter_types::FromNode>::from_node(node, src)
                    .map(Self::ReturnStatement)
                    .unwrap_or(Self::Unknown(node))
            }
            "string" => <String as ::treesitter_types::FromNode>::from_node(node, src)
                .map(Self::String)
                .unwrap_or(Self::Unknown(node)),
            "string_content" => {
                <StringContent as ::treesitter_types::FromNode>::from_node(node, src)
                    .map(Self::StringContent)
                    .unwrap_or(Self::Unknown(node))
            }
            "table_constructor" => {
                <TableConstructor as ::treesitter_types::FromNode>::from_node(node, src)
                    .map(Self::TableConstructor)
                    .unwrap_or(Self::Unknown(node))
            }
            "unary_expression" => {
                <UnaryExpression as ::treesitter_types::FromNode>::from_node(node, src)
                    .map(Self::UnaryExpression)
                    .unwrap_or(Self::Unknown(node))
            }
            "variable_declaration" => {
                <VariableDeclaration as ::treesitter_types::FromNode>::from_node(node, src)
                    .map(Self::VariableDeclaration)
                    .unwrap_or(Self::Unknown(node))
            }
            "variable_list" => <VariableList as ::treesitter_types::FromNode>::from_node(node, src)
                .map(Self::VariableList)
                .unwrap_or(Self::Unknown(node)),
            "while_statement" => {
                <WhileStatement as ::treesitter_types::FromNode>::from_node(node, src)
                    .map(Self::WhileStatement)
                    .unwrap_or(Self::Unknown(node))
            }
            "break_statement" => {
                <BreakStatement as ::treesitter_types::FromNode>::from_node(node, src)
                    .map(Self::BreakStatement)
                    .unwrap_or(Self::Unknown(node))
            }
            "comment_content" => {
                <CommentContent as ::treesitter_types::FromNode>::from_node(node, src)
                    .map(Self::CommentContent)
                    .unwrap_or(Self::Unknown(node))
            }
            "escape_sequence" => {
                <EscapeSequence as ::treesitter_types::FromNode>::from_node(node, src)
                    .map(Self::EscapeSequence)
                    .unwrap_or(Self::Unknown(node))
            }
            "false" => <False as ::treesitter_types::FromNode>::from_node(node, src)
                .map(Self::False)
                .unwrap_or(Self::Unknown(node)),
            "hash_bang_line" => {
                <HashBangLine as ::treesitter_types::FromNode>::from_node(node, src)
                    .map(Self::HashBangLine)
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
            "true" => <True as ::treesitter_types::FromNode>::from_node(node, src)
                .map(Self::True)
                .unwrap_or(Self::Unknown(node)),
            "vararg_expression" => {
                <VarargExpression as ::treesitter_types::FromNode>::from_node(node, src)
                    .map(Self::VarargExpression)
                    .unwrap_or(Self::Unknown(node))
            }
            _ => Self::Unknown(node),
        }
    }
}
impl ::treesitter_types::Spanned for AnyNode<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        match self {
            Self::Declaration(inner) => inner.span(),
            Self::Expression(inner) => inner.span(),
            Self::Statement(inner) => inner.span(),
            Self::Variable(inner) => inner.span(),
            Self::Arguments(inner) => inner.span(),
            Self::AssignmentStatement(inner) => inner.span(),
            Self::Attribute(inner) => inner.span(),
            Self::BinaryExpression(inner) => inner.span(),
            Self::Block(inner) => inner.span(),
            Self::BracketIndexExpression(inner) => inner.span(),
            Self::Chunk(inner) => inner.span(),
            Self::Comment(inner) => inner.span(),
            Self::DoStatement(inner) => inner.span(),
            Self::DotIndexExpression(inner) => inner.span(),
            Self::ElseStatement(inner) => inner.span(),
            Self::ElseifStatement(inner) => inner.span(),
            Self::EmptyStatement(inner) => inner.span(),
            Self::ExpressionList(inner) => inner.span(),
            Self::Field(inner) => inner.span(),
            Self::ForGenericClause(inner) => inner.span(),
            Self::ForNumericClause(inner) => inner.span(),
            Self::ForStatement(inner) => inner.span(),
            Self::FunctionCall(inner) => inner.span(),
            Self::FunctionDeclaration(inner) => inner.span(),
            Self::FunctionDefinition(inner) => inner.span(),
            Self::GotoStatement(inner) => inner.span(),
            Self::IfStatement(inner) => inner.span(),
            Self::ImplicitVariableDeclaration(inner) => inner.span(),
            Self::LabelStatement(inner) => inner.span(),
            Self::MethodIndexExpression(inner) => inner.span(),
            Self::Parameters(inner) => inner.span(),
            Self::ParenthesizedExpression(inner) => inner.span(),
            Self::RepeatStatement(inner) => inner.span(),
            Self::ReturnStatement(inner) => inner.span(),
            Self::String(inner) => inner.span(),
            Self::StringContent(inner) => inner.span(),
            Self::TableConstructor(inner) => inner.span(),
            Self::UnaryExpression(inner) => inner.span(),
            Self::VariableDeclaration(inner) => inner.span(),
            Self::VariableList(inner) => inner.span(),
            Self::WhileStatement(inner) => inner.span(),
            Self::BreakStatement(inner) => inner.span(),
            Self::CommentContent(inner) => inner.span(),
            Self::EscapeSequence(inner) => inner.span(),
            Self::False(inner) => inner.span(),
            Self::HashBangLine(inner) => inner.span(),
            Self::Identifier(inner) => inner.span(),
            Self::Nil(inner) => inner.span(),
            Self::Number(inner) => inner.span(),
            Self::True(inner) => inner.span(),
            Self::VarargExpression(inner) => inner.span(),
            Self::Unknown(node) => ::treesitter_types::Span::from(*node),
        }
    }
}
