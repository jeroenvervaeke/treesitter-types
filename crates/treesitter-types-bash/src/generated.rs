#[derive(Debug, Clone)]
pub enum Expression<'tree> {
    PrimaryExpression(::std::boxed::Box<PrimaryExpression<'tree>>),
    BinaryExpression(::std::boxed::Box<BinaryExpression<'tree>>),
    Concatenation(::std::boxed::Box<Concatenation<'tree>>),
    ParenthesizedExpression(::std::boxed::Box<ParenthesizedExpression<'tree>>),
    PostfixExpression(::std::boxed::Box<PostfixExpression<'tree>>),
    TernaryExpression(::std::boxed::Box<TernaryExpression<'tree>>),
    UnaryExpression(::std::boxed::Box<UnaryExpression<'tree>>),
    Word(::std::boxed::Box<Word<'tree>>),
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
            "concatenation" => Ok(Self::Concatenation(::std::boxed::Box::new(
                <Concatenation as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "parenthesized_expression" => {
                Ok(Self::ParenthesizedExpression(::std::boxed::Box::new(
                    <ParenthesizedExpression as ::treesitter_types::FromNode>::from_node(
                        node, src,
                    )?,
                )))
            }
            "postfix_expression" => Ok(Self::PostfixExpression(::std::boxed::Box::new(
                <PostfixExpression as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "ternary_expression" => Ok(Self::TernaryExpression(::std::boxed::Box::new(
                <TernaryExpression as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "unary_expression" => Ok(Self::UnaryExpression(::std::boxed::Box::new(
                <UnaryExpression as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "word" => Ok(Self::Word(::std::boxed::Box::new(
                <Word as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            _other => {
                if let Ok(v) =
                    <PrimaryExpression as ::treesitter_types::FromNode>::from_node(node, src)
                {
                    Ok(Self::PrimaryExpression(::std::boxed::Box::new(v)))
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
            Self::PrimaryExpression(inner) => inner.span(),
            Self::BinaryExpression(inner) => inner.span(),
            Self::Concatenation(inner) => inner.span(),
            Self::ParenthesizedExpression(inner) => inner.span(),
            Self::PostfixExpression(inner) => inner.span(),
            Self::TernaryExpression(inner) => inner.span(),
            Self::UnaryExpression(inner) => inner.span(),
            Self::Word(inner) => inner.span(),
        }
    }
}
#[derive(Debug, Clone)]
pub enum PrimaryExpression<'tree> {
    AnsiCString(::std::boxed::Box<AnsiCString<'tree>>),
    ArithmeticExpansion(::std::boxed::Box<ArithmeticExpansion<'tree>>),
    BraceExpression(::std::boxed::Box<BraceExpression<'tree>>),
    CommandSubstitution(::std::boxed::Box<CommandSubstitution<'tree>>),
    Expansion(::std::boxed::Box<Expansion<'tree>>),
    Number(::std::boxed::Box<Number<'tree>>),
    ProcessSubstitution(::std::boxed::Box<ProcessSubstitution<'tree>>),
    RawString(::std::boxed::Box<RawString<'tree>>),
    SimpleExpansion(::std::boxed::Box<SimpleExpansion<'tree>>),
    String(::std::boxed::Box<String<'tree>>),
    TranslatedString(::std::boxed::Box<TranslatedString<'tree>>),
    Word(::std::boxed::Box<Word<'tree>>),
}
impl<'tree> ::treesitter_types::FromNode<'tree> for PrimaryExpression<'tree> {
    #[allow(clippy::collapsible_else_if)]
    fn from_node(
        node: ::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        match node.kind() {
            "ansi_c_string" => Ok(Self::AnsiCString(::std::boxed::Box::new(
                <AnsiCString as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "arithmetic_expansion" => Ok(Self::ArithmeticExpansion(::std::boxed::Box::new(
                <ArithmeticExpansion as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "brace_expression" => Ok(Self::BraceExpression(::std::boxed::Box::new(
                <BraceExpression as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "command_substitution" => Ok(Self::CommandSubstitution(::std::boxed::Box::new(
                <CommandSubstitution as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "expansion" => Ok(Self::Expansion(::std::boxed::Box::new(
                <Expansion as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "number" => Ok(Self::Number(::std::boxed::Box::new(
                <Number as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "process_substitution" => Ok(Self::ProcessSubstitution(::std::boxed::Box::new(
                <ProcessSubstitution as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "raw_string" => Ok(Self::RawString(::std::boxed::Box::new(
                <RawString as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "simple_expansion" => Ok(Self::SimpleExpansion(::std::boxed::Box::new(
                <SimpleExpansion as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "string" => Ok(Self::String(::std::boxed::Box::new(
                <String as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "translated_string" => Ok(Self::TranslatedString(::std::boxed::Box::new(
                <TranslatedString as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "word" => Ok(Self::Word(::std::boxed::Box::new(
                <Word as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            other => Err(::treesitter_types::ParseError::unexpected_kind(other, node)),
        }
    }
}
impl ::treesitter_types::Spanned for PrimaryExpression<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        match self {
            Self::AnsiCString(inner) => inner.span(),
            Self::ArithmeticExpansion(inner) => inner.span(),
            Self::BraceExpression(inner) => inner.span(),
            Self::CommandSubstitution(inner) => inner.span(),
            Self::Expansion(inner) => inner.span(),
            Self::Number(inner) => inner.span(),
            Self::ProcessSubstitution(inner) => inner.span(),
            Self::RawString(inner) => inner.span(),
            Self::SimpleExpansion(inner) => inner.span(),
            Self::String(inner) => inner.span(),
            Self::TranslatedString(inner) => inner.span(),
            Self::Word(inner) => inner.span(),
        }
    }
}
#[derive(Debug, Clone)]
pub enum Statement<'tree> {
    CStyleForStatement(::std::boxed::Box<CStyleForStatement<'tree>>),
    CaseStatement(::std::boxed::Box<CaseStatement<'tree>>),
    Command(::std::boxed::Box<Command<'tree>>),
    CompoundStatement(::std::boxed::Box<CompoundStatement<'tree>>),
    DeclarationCommand(::std::boxed::Box<DeclarationCommand<'tree>>),
    ForStatement(::std::boxed::Box<ForStatement<'tree>>),
    FunctionDefinition(::std::boxed::Box<FunctionDefinition<'tree>>),
    IfStatement(::std::boxed::Box<IfStatement<'tree>>),
    List(::std::boxed::Box<List<'tree>>),
    NegatedCommand(::std::boxed::Box<NegatedCommand<'tree>>),
    Pipeline(::std::boxed::Box<Pipeline<'tree>>),
    RedirectedStatement(::std::boxed::Box<RedirectedStatement<'tree>>),
    Subshell(::std::boxed::Box<Subshell<'tree>>),
    TestCommand(::std::boxed::Box<TestCommand<'tree>>),
    UnsetCommand(::std::boxed::Box<UnsetCommand<'tree>>),
    VariableAssignment(::std::boxed::Box<VariableAssignment<'tree>>),
    VariableAssignments(::std::boxed::Box<VariableAssignments<'tree>>),
    WhileStatement(::std::boxed::Box<WhileStatement<'tree>>),
}
impl<'tree> ::treesitter_types::FromNode<'tree> for Statement<'tree> {
    #[allow(clippy::collapsible_else_if)]
    fn from_node(
        node: ::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        match node.kind() {
            "c_style_for_statement" => Ok(Self::CStyleForStatement(::std::boxed::Box::new(
                <CStyleForStatement as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "case_statement" => Ok(Self::CaseStatement(::std::boxed::Box::new(
                <CaseStatement as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "command" => Ok(Self::Command(::std::boxed::Box::new(
                <Command as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "compound_statement" => Ok(Self::CompoundStatement(::std::boxed::Box::new(
                <CompoundStatement as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "declaration_command" => Ok(Self::DeclarationCommand(::std::boxed::Box::new(
                <DeclarationCommand as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "for_statement" => Ok(Self::ForStatement(::std::boxed::Box::new(
                <ForStatement as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "function_definition" => Ok(Self::FunctionDefinition(::std::boxed::Box::new(
                <FunctionDefinition as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "if_statement" => Ok(Self::IfStatement(::std::boxed::Box::new(
                <IfStatement as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "list" => Ok(Self::List(::std::boxed::Box::new(
                <List as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "negated_command" => Ok(Self::NegatedCommand(::std::boxed::Box::new(
                <NegatedCommand as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "pipeline" => Ok(Self::Pipeline(::std::boxed::Box::new(
                <Pipeline as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "redirected_statement" => Ok(Self::RedirectedStatement(::std::boxed::Box::new(
                <RedirectedStatement as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "subshell" => Ok(Self::Subshell(::std::boxed::Box::new(
                <Subshell as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "test_command" => Ok(Self::TestCommand(::std::boxed::Box::new(
                <TestCommand as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "unset_command" => Ok(Self::UnsetCommand(::std::boxed::Box::new(
                <UnsetCommand as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "variable_assignment" => Ok(Self::VariableAssignment(::std::boxed::Box::new(
                <VariableAssignment as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "variable_assignments" => Ok(Self::VariableAssignments(::std::boxed::Box::new(
                <VariableAssignments as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "while_statement" => Ok(Self::WhileStatement(::std::boxed::Box::new(
                <WhileStatement as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            other => Err(::treesitter_types::ParseError::unexpected_kind(other, node)),
        }
    }
}
impl ::treesitter_types::Spanned for Statement<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        match self {
            Self::CStyleForStatement(inner) => inner.span(),
            Self::CaseStatement(inner) => inner.span(),
            Self::Command(inner) => inner.span(),
            Self::CompoundStatement(inner) => inner.span(),
            Self::DeclarationCommand(inner) => inner.span(),
            Self::ForStatement(inner) => inner.span(),
            Self::FunctionDefinition(inner) => inner.span(),
            Self::IfStatement(inner) => inner.span(),
            Self::List(inner) => inner.span(),
            Self::NegatedCommand(inner) => inner.span(),
            Self::Pipeline(inner) => inner.span(),
            Self::RedirectedStatement(inner) => inner.span(),
            Self::Subshell(inner) => inner.span(),
            Self::TestCommand(inner) => inner.span(),
            Self::UnsetCommand(inner) => inner.span(),
            Self::VariableAssignment(inner) => inner.span(),
            Self::VariableAssignments(inner) => inner.span(),
            Self::WhileStatement(inner) => inner.span(),
        }
    }
}
#[derive(Debug, Clone)]
pub struct ArithmeticExpansion<'tree> {
    pub span: ::treesitter_types::Span,
    pub children: ::std::vec::Vec<ArithmeticExpansionChildren<'tree>>,
}
impl<'tree> ::treesitter_types::FromNode<'tree> for ArithmeticExpansion<'tree> {
    #[allow(clippy::match_single_binding, clippy::suspicious_else_formatting)]
    fn from_node(
        node: ::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        debug_assert_eq!(node.kind(), "arithmetic_expansion");
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
                        <ArithmeticExpansionChildren as ::treesitter_types::FromNode>::from_node(
                            child, src,
                        )?,
                    );
                }
                items
            },
        })
    }
}
impl ::treesitter_types::Spanned for ArithmeticExpansion<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        self.span
    }
}
#[derive(Debug, Clone)]
pub struct Array<'tree> {
    pub span: ::treesitter_types::Span,
    pub children: ::std::vec::Vec<ArrayChildren<'tree>>,
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
                    items.push(<ArrayChildren as ::treesitter_types::FromNode>::from_node(
                        child, src,
                    )?);
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
#[derive(Debug, Clone)]
pub struct BinaryExpression<'tree> {
    pub span: ::treesitter_types::Span,
    pub left: ::core::option::Option<BinaryExpressionLeft<'tree>>,
    pub operator: BinaryExpressionOperator<'tree>,
    pub right: ::std::vec::Vec<BinaryExpressionRight<'tree>>,
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
            left: match node.child_by_field_name("left") {
                Some(child) => Some(
                    <BinaryExpressionLeft as ::treesitter_types::FromNode>::from_node(child, src)?,
                ),
                None => None,
            },
            operator: {
                let child = node.child_by_field_name("operator").ok_or_else(|| {
                    ::treesitter_types::ParseError::missing_field("operator", node)
                })?;
                <BinaryExpressionOperator as ::treesitter_types::FromNode>::from_node(child, src)?
            },
            right: {
                let mut cursor = node.walk();
                let mut items = ::std::vec::Vec::new();
                for child in node.children_by_field_name("right", &mut cursor) {
                    items.push(
                        <BinaryExpressionRight as ::treesitter_types::FromNode>::from_node(
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
pub struct BraceExpression<'tree> {
    pub span: ::treesitter_types::Span,
    pub children: ::std::vec::Vec<Number<'tree>>,
}
impl<'tree> ::treesitter_types::FromNode<'tree> for BraceExpression<'tree> {
    #[allow(clippy::match_single_binding, clippy::suspicious_else_formatting)]
    fn from_node(
        node: ::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        debug_assert_eq!(node.kind(), "brace_expression");
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
                    items.push(<Number as ::treesitter_types::FromNode>::from_node(
                        child, src,
                    )?);
                }
                items
            },
        })
    }
}
impl ::treesitter_types::Spanned for BraceExpression<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        self.span
    }
}
#[derive(Debug, Clone)]
pub struct CStyleForStatement<'tree> {
    pub span: ::treesitter_types::Span,
    pub body: CStyleForStatementBody<'tree>,
    pub condition: ::std::vec::Vec<CStyleForStatementCondition<'tree>>,
    pub initializer: ::std::vec::Vec<CStyleForStatementInitializer<'tree>>,
    pub update: ::std::vec::Vec<CStyleForStatementUpdate<'tree>>,
}
impl<'tree> ::treesitter_types::FromNode<'tree> for CStyleForStatement<'tree> {
    #[allow(clippy::match_single_binding, clippy::suspicious_else_formatting)]
    fn from_node(
        node: ::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        debug_assert_eq!(node.kind(), "c_style_for_statement");
        Ok(Self {
            span: ::treesitter_types::Span::from(node),
            body: {
                let child = node
                    .child_by_field_name("body")
                    .ok_or_else(|| ::treesitter_types::ParseError::missing_field("body", node))?;
                <CStyleForStatementBody as ::treesitter_types::FromNode>::from_node(child, src)?
            },
            condition: {
                let mut cursor = node.walk();
                let mut items = ::std::vec::Vec::new();
                for child in node.children_by_field_name("condition", &mut cursor) {
                    items.push(
                        <CStyleForStatementCondition as ::treesitter_types::FromNode>::from_node(
                            child, src,
                        )?,
                    );
                }
                items
            },
            initializer: {
                let mut cursor = node.walk();
                let mut items = ::std::vec::Vec::new();
                for child in node.children_by_field_name("initializer", &mut cursor) {
                    items.push(
                        <CStyleForStatementInitializer as ::treesitter_types::FromNode>::from_node(
                            child, src,
                        )?,
                    );
                }
                items
            },
            update: {
                let mut cursor = node.walk();
                let mut items = ::std::vec::Vec::new();
                for child in node.children_by_field_name("update", &mut cursor) {
                    items.push(
                        <CStyleForStatementUpdate as ::treesitter_types::FromNode>::from_node(
                            child, src,
                        )?,
                    );
                }
                items
            },
        })
    }
}
impl ::treesitter_types::Spanned for CStyleForStatement<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        self.span
    }
}
#[derive(Debug, Clone)]
pub struct CaseItem<'tree> {
    pub span: ::treesitter_types::Span,
    pub fallthrough: ::core::option::Option<CaseItemFallthrough>,
    pub termination: ::core::option::Option<CaseItemTermination>,
    pub value: ::std::vec::Vec<CaseItemValue<'tree>>,
    pub children: ::std::vec::Vec<Statement<'tree>>,
}
impl<'tree> ::treesitter_types::FromNode<'tree> for CaseItem<'tree> {
    #[allow(clippy::match_single_binding, clippy::suspicious_else_formatting)]
    fn from_node(
        node: ::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        debug_assert_eq!(node.kind(), "case_item");
        Ok(Self {
            span: ::treesitter_types::Span::from(node),
            fallthrough: match node.child_by_field_name("fallthrough") {
                Some(child) => Some(
                    <CaseItemFallthrough as ::treesitter_types::FromNode>::from_node(child, src)?,
                ),
                None => None,
            },
            termination: match node.child_by_field_name("termination") {
                Some(child) => Some(
                    <CaseItemTermination as ::treesitter_types::FromNode>::from_node(child, src)?,
                ),
                None => None,
            },
            value: {
                let mut cursor = node.walk();
                let mut items = ::std::vec::Vec::new();
                for child in node.children_by_field_name("value", &mut cursor) {
                    items.push(<CaseItemValue as ::treesitter_types::FromNode>::from_node(
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
                    items.push(<Statement as ::treesitter_types::FromNode>::from_node(
                        child, src,
                    )?);
                }
                items
            },
        })
    }
}
impl ::treesitter_types::Spanned for CaseItem<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        self.span
    }
}
#[derive(Debug, Clone)]
pub struct CaseStatement<'tree> {
    pub span: ::treesitter_types::Span,
    pub value: CaseStatementValue<'tree>,
    pub children: ::std::vec::Vec<CaseItem<'tree>>,
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
            value: {
                let child = node
                    .child_by_field_name("value")
                    .ok_or_else(|| ::treesitter_types::ParseError::missing_field("value", node))?;
                <CaseStatementValue as ::treesitter_types::FromNode>::from_node(child, src)?
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
                    items.push(<CaseItem as ::treesitter_types::FromNode>::from_node(
                        child, src,
                    )?);
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
pub struct Command<'tree> {
    pub span: ::treesitter_types::Span,
    pub argument: ::std::vec::Vec<CommandArgument<'tree>>,
    pub name: CommandName<'tree>,
    pub redirect: ::std::vec::Vec<CommandRedirect<'tree>>,
    pub children: ::std::vec::Vec<CommandChildren<'tree>>,
}
impl<'tree> ::treesitter_types::FromNode<'tree> for Command<'tree> {
    #[allow(clippy::match_single_binding, clippy::suspicious_else_formatting)]
    fn from_node(
        node: ::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        debug_assert_eq!(node.kind(), "command");
        Ok(Self {
            span: ::treesitter_types::Span::from(node),
            argument: {
                let mut cursor = node.walk();
                let mut items = ::std::vec::Vec::new();
                for child in node.children_by_field_name("argument", &mut cursor) {
                    items.push(
                        <CommandArgument as ::treesitter_types::FromNode>::from_node(child, src)?,
                    );
                }
                items
            },
            name: {
                let child = node
                    .child_by_field_name("name")
                    .ok_or_else(|| ::treesitter_types::ParseError::missing_field("name", node))?;
                <CommandName as ::treesitter_types::FromNode>::from_node(child, src)?
            },
            redirect: {
                let mut cursor = node.walk();
                let mut items = ::std::vec::Vec::new();
                for child in node.children_by_field_name("redirect", &mut cursor) {
                    items.push(
                        <CommandRedirect as ::treesitter_types::FromNode>::from_node(child, src)?,
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
                        <CommandChildren as ::treesitter_types::FromNode>::from_node(child, src)?,
                    );
                }
                items
            },
        })
    }
}
impl ::treesitter_types::Spanned for Command<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        self.span
    }
}
#[derive(Debug, Clone)]
pub struct CommandName<'tree> {
    pub span: ::treesitter_types::Span,
    pub children: CommandNameChildren<'tree>,
}
impl<'tree> ::treesitter_types::FromNode<'tree> for CommandName<'tree> {
    #[allow(clippy::match_single_binding, clippy::suspicious_else_formatting)]
    fn from_node(
        node: ::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        debug_assert_eq!(node.kind(), "command_name");
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
                                        <CommandNameChildren as ::treesitter_types::FromNode>::from_node(
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
                    fallback_child.ok_or_else(|| {
                        ::treesitter_types::ParseError::missing_field("children", node)
                    })?
                };
                <CommandNameChildren as ::treesitter_types::FromNode>::from_node(child, src)?
            },
        })
    }
}
impl ::treesitter_types::Spanned for CommandName<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        self.span
    }
}
#[derive(Debug, Clone)]
pub struct CommandSubstitution<'tree> {
    pub span: ::treesitter_types::Span,
    pub redirect: ::core::option::Option<FileRedirect<'tree>>,
    pub children: ::std::vec::Vec<Statement<'tree>>,
}
impl<'tree> ::treesitter_types::FromNode<'tree> for CommandSubstitution<'tree> {
    #[allow(clippy::match_single_binding, clippy::suspicious_else_formatting)]
    fn from_node(
        node: ::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        debug_assert_eq!(node.kind(), "command_substitution");
        Ok(Self {
            span: ::treesitter_types::Span::from(node),
            redirect: match node.child_by_field_name("redirect") {
                Some(child) => Some(<FileRedirect as ::treesitter_types::FromNode>::from_node(
                    child, src,
                )?),
                None => None,
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
                    items.push(<Statement as ::treesitter_types::FromNode>::from_node(
                        child, src,
                    )?);
                }
                items
            },
        })
    }
}
impl ::treesitter_types::Spanned for CommandSubstitution<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        self.span
    }
}
#[derive(Debug, Clone)]
pub struct CompoundStatement<'tree> {
    pub span: ::treesitter_types::Span,
    pub children: ::std::vec::Vec<CompoundStatementChildren<'tree>>,
}
impl<'tree> ::treesitter_types::FromNode<'tree> for CompoundStatement<'tree> {
    #[allow(clippy::match_single_binding, clippy::suspicious_else_formatting)]
    fn from_node(
        node: ::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        debug_assert_eq!(node.kind(), "compound_statement");
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
                        <CompoundStatementChildren as ::treesitter_types::FromNode>::from_node(
                            child, src,
                        )?,
                    );
                }
                items
            },
        })
    }
}
impl ::treesitter_types::Spanned for CompoundStatement<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        self.span
    }
}
#[derive(Debug, Clone)]
pub struct Concatenation<'tree> {
    pub span: ::treesitter_types::Span,
    pub children: ::std::vec::Vec<ConcatenationChildren<'tree>>,
}
impl<'tree> ::treesitter_types::FromNode<'tree> for Concatenation<'tree> {
    #[allow(clippy::match_single_binding, clippy::suspicious_else_formatting)]
    fn from_node(
        node: ::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        debug_assert_eq!(node.kind(), "concatenation");
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
                        <ConcatenationChildren as ::treesitter_types::FromNode>::from_node(
                            child, src,
                        )?,
                    );
                }
                items
            },
        })
    }
}
impl ::treesitter_types::Spanned for Concatenation<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        self.span
    }
}
#[derive(Debug, Clone)]
pub struct DeclarationCommand<'tree> {
    pub span: ::treesitter_types::Span,
    pub children: ::std::vec::Vec<DeclarationCommandChildren<'tree>>,
}
impl<'tree> ::treesitter_types::FromNode<'tree> for DeclarationCommand<'tree> {
    #[allow(clippy::match_single_binding, clippy::suspicious_else_formatting)]
    fn from_node(
        node: ::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        debug_assert_eq!(node.kind(), "declaration_command");
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
                        <DeclarationCommandChildren as ::treesitter_types::FromNode>::from_node(
                            child, src,
                        )?,
                    );
                }
                items
            },
        })
    }
}
impl ::treesitter_types::Spanned for DeclarationCommand<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        self.span
    }
}
#[derive(Debug, Clone)]
pub struct DoGroup<'tree> {
    pub span: ::treesitter_types::Span,
    pub children: ::std::vec::Vec<Statement<'tree>>,
}
impl<'tree> ::treesitter_types::FromNode<'tree> for DoGroup<'tree> {
    #[allow(clippy::match_single_binding, clippy::suspicious_else_formatting)]
    fn from_node(
        node: ::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        debug_assert_eq!(node.kind(), "do_group");
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
                    items.push(<Statement as ::treesitter_types::FromNode>::from_node(
                        child, src,
                    )?);
                }
                items
            },
        })
    }
}
impl ::treesitter_types::Spanned for DoGroup<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        self.span
    }
}
#[derive(Debug, Clone)]
pub struct ElifClause<'tree> {
    pub span: ::treesitter_types::Span,
    pub children: ::std::vec::Vec<Statement<'tree>>,
}
impl<'tree> ::treesitter_types::FromNode<'tree> for ElifClause<'tree> {
    #[allow(clippy::match_single_binding, clippy::suspicious_else_formatting)]
    fn from_node(
        node: ::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        debug_assert_eq!(node.kind(), "elif_clause");
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
                    items.push(<Statement as ::treesitter_types::FromNode>::from_node(
                        child, src,
                    )?);
                }
                items
            },
        })
    }
}
impl ::treesitter_types::Spanned for ElifClause<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        self.span
    }
}
#[derive(Debug, Clone)]
pub struct ElseClause<'tree> {
    pub span: ::treesitter_types::Span,
    pub children: ::std::vec::Vec<Statement<'tree>>,
}
impl<'tree> ::treesitter_types::FromNode<'tree> for ElseClause<'tree> {
    #[allow(clippy::match_single_binding, clippy::suspicious_else_formatting)]
    fn from_node(
        node: ::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        debug_assert_eq!(node.kind(), "else_clause");
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
                    items.push(<Statement as ::treesitter_types::FromNode>::from_node(
                        child, src,
                    )?);
                }
                items
            },
        })
    }
}
impl ::treesitter_types::Spanned for ElseClause<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        self.span
    }
}
#[derive(Debug, Clone)]
pub struct Expansion<'tree> {
    pub span: ::treesitter_types::Span,
    pub operator: ::std::vec::Vec<ExpansionOperator>,
    pub children: ::std::vec::Vec<ExpansionChildren<'tree>>,
}
impl<'tree> ::treesitter_types::FromNode<'tree> for Expansion<'tree> {
    #[allow(clippy::match_single_binding, clippy::suspicious_else_formatting)]
    fn from_node(
        node: ::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        debug_assert_eq!(node.kind(), "expansion");
        Ok(Self {
            span: ::treesitter_types::Span::from(node),
            operator: {
                let mut cursor = node.walk();
                let mut items = ::std::vec::Vec::new();
                for child in node.children_by_field_name("operator", &mut cursor) {
                    items.push(
                        <ExpansionOperator as ::treesitter_types::FromNode>::from_node(child, src)?,
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
                        <ExpansionChildren as ::treesitter_types::FromNode>::from_node(child, src)?,
                    );
                }
                items
            },
        })
    }
}
impl ::treesitter_types::Spanned for Expansion<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        self.span
    }
}
#[derive(Debug, Clone)]
pub struct FileRedirect<'tree> {
    pub span: ::treesitter_types::Span,
    pub descriptor: ::core::option::Option<FileDescriptor<'tree>>,
    pub destination: ::std::vec::Vec<FileRedirectDestination<'tree>>,
}
impl<'tree> ::treesitter_types::FromNode<'tree> for FileRedirect<'tree> {
    #[allow(clippy::match_single_binding, clippy::suspicious_else_formatting)]
    fn from_node(
        node: ::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        debug_assert_eq!(node.kind(), "file_redirect");
        Ok(Self {
            span: ::treesitter_types::Span::from(node),
            descriptor: match node.child_by_field_name("descriptor") {
                Some(child) => Some(<FileDescriptor as ::treesitter_types::FromNode>::from_node(
                    child, src,
                )?),
                None => None,
            },
            destination: {
                let mut cursor = node.walk();
                let mut items = ::std::vec::Vec::new();
                for child in node.children_by_field_name("destination", &mut cursor) {
                    items.push(
                        <FileRedirectDestination as ::treesitter_types::FromNode>::from_node(
                            child, src,
                        )?,
                    );
                }
                items
            },
        })
    }
}
impl ::treesitter_types::Spanned for FileRedirect<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        self.span
    }
}
#[derive(Debug, Clone)]
pub struct ForStatement<'tree> {
    pub span: ::treesitter_types::Span,
    pub body: DoGroup<'tree>,
    pub value: ::std::vec::Vec<ForStatementValue<'tree>>,
    pub variable: VariableName<'tree>,
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
            body: {
                let child = node
                    .child_by_field_name("body")
                    .ok_or_else(|| ::treesitter_types::ParseError::missing_field("body", node))?;
                <DoGroup as ::treesitter_types::FromNode>::from_node(child, src)?
            },
            value: {
                let mut cursor = node.walk();
                let mut items = ::std::vec::Vec::new();
                for child in node.children_by_field_name("value", &mut cursor) {
                    items.push(
                        <ForStatementValue as ::treesitter_types::FromNode>::from_node(child, src)?,
                    );
                }
                items
            },
            variable: {
                let child = node.child_by_field_name("variable").ok_or_else(|| {
                    ::treesitter_types::ParseError::missing_field("variable", node)
                })?;
                <VariableName as ::treesitter_types::FromNode>::from_node(child, src)?
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
pub struct FunctionDefinition<'tree> {
    pub span: ::treesitter_types::Span,
    pub body: FunctionDefinitionBody<'tree>,
    pub name: Word<'tree>,
    pub redirect: ::core::option::Option<FunctionDefinitionRedirect<'tree>>,
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
            body: {
                let child = node
                    .child_by_field_name("body")
                    .ok_or_else(|| ::treesitter_types::ParseError::missing_field("body", node))?;
                <FunctionDefinitionBody as ::treesitter_types::FromNode>::from_node(child, src)?
            },
            name: {
                let child = node
                    .child_by_field_name("name")
                    .ok_or_else(|| ::treesitter_types::ParseError::missing_field("name", node))?;
                <Word as ::treesitter_types::FromNode>::from_node(child, src)?
            },
            redirect: match node.child_by_field_name("redirect") {
                Some(child) => Some(
                    <FunctionDefinitionRedirect as ::treesitter_types::FromNode>::from_node(
                        child, src,
                    )?,
                ),
                None => None,
            },
        })
    }
}
impl ::treesitter_types::Spanned for FunctionDefinition<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        self.span
    }
}
#[derive(Debug, Clone)]
pub struct HeredocBody<'tree> {
    pub span: ::treesitter_types::Span,
    pub children: ::std::vec::Vec<HeredocBodyChildren<'tree>>,
}
impl<'tree> ::treesitter_types::FromNode<'tree> for HeredocBody<'tree> {
    #[allow(clippy::match_single_binding, clippy::suspicious_else_formatting)]
    fn from_node(
        node: ::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        debug_assert_eq!(node.kind(), "heredoc_body");
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
                        <HeredocBodyChildren as ::treesitter_types::FromNode>::from_node(
                            child, src,
                        )?,
                    );
                }
                items
            },
        })
    }
}
impl ::treesitter_types::Spanned for HeredocBody<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        self.span
    }
}
#[derive(Debug, Clone)]
pub struct HeredocRedirect<'tree> {
    pub span: ::treesitter_types::Span,
    pub argument: ::std::vec::Vec<HeredocRedirectArgument<'tree>>,
    pub descriptor: ::core::option::Option<FileDescriptor<'tree>>,
    pub operator: ::core::option::Option<HeredocRedirectOperator>,
    pub redirect: ::std::vec::Vec<HeredocRedirectRedirect<'tree>>,
    pub right: ::core::option::Option<Statement<'tree>>,
    pub children: ::std::vec::Vec<HeredocRedirectChildren<'tree>>,
}
impl<'tree> ::treesitter_types::FromNode<'tree> for HeredocRedirect<'tree> {
    #[allow(clippy::match_single_binding, clippy::suspicious_else_formatting)]
    fn from_node(
        node: ::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        debug_assert_eq!(node.kind(), "heredoc_redirect");
        Ok(Self {
            span: ::treesitter_types::Span::from(node),
            argument: {
                let mut cursor = node.walk();
                let mut items = ::std::vec::Vec::new();
                for child in node.children_by_field_name("argument", &mut cursor) {
                    items.push(
                        <HeredocRedirectArgument as ::treesitter_types::FromNode>::from_node(
                            child, src,
                        )?,
                    );
                }
                items
            },
            descriptor: match node.child_by_field_name("descriptor") {
                Some(child) => Some(<FileDescriptor as ::treesitter_types::FromNode>::from_node(
                    child, src,
                )?),
                None => None,
            },
            operator: match node.child_by_field_name("operator") {
                Some(child) => Some(
                    <HeredocRedirectOperator as ::treesitter_types::FromNode>::from_node(
                        child, src,
                    )?,
                ),
                None => None,
            },
            redirect: {
                let mut cursor = node.walk();
                let mut items = ::std::vec::Vec::new();
                for child in node.children_by_field_name("redirect", &mut cursor) {
                    items.push(
                        <HeredocRedirectRedirect as ::treesitter_types::FromNode>::from_node(
                            child, src,
                        )?,
                    );
                }
                items
            },
            right: match node.child_by_field_name("right") {
                Some(child) => Some(<Statement as ::treesitter_types::FromNode>::from_node(
                    child, src,
                )?),
                None => None,
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
                        <HeredocRedirectChildren as ::treesitter_types::FromNode>::from_node(
                            child, src,
                        )?,
                    );
                }
                items
            },
        })
    }
}
impl ::treesitter_types::Spanned for HeredocRedirect<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        self.span
    }
}
#[derive(Debug, Clone)]
pub struct HerestringRedirect<'tree> {
    pub span: ::treesitter_types::Span,
    pub descriptor: ::core::option::Option<FileDescriptor<'tree>>,
    pub children: HerestringRedirectChildren<'tree>,
}
impl<'tree> ::treesitter_types::FromNode<'tree> for HerestringRedirect<'tree> {
    #[allow(clippy::match_single_binding, clippy::suspicious_else_formatting)]
    fn from_node(
        node: ::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        debug_assert_eq!(node.kind(), "herestring_redirect");
        Ok(Self {
            span: ::treesitter_types::Span::from(node),
            descriptor: match node.child_by_field_name("descriptor") {
                Some(child) => Some(<FileDescriptor as ::treesitter_types::FromNode>::from_node(
                    child, src,
                )?),
                None => None,
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
                                        <HerestringRedirectChildren as ::treesitter_types::FromNode>::from_node(
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
                    fallback_child.ok_or_else(|| {
                        ::treesitter_types::ParseError::missing_field("children", node)
                    })?
                };
                <HerestringRedirectChildren as ::treesitter_types::FromNode>::from_node(child, src)?
            },
        })
    }
}
impl ::treesitter_types::Spanned for HerestringRedirect<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        self.span
    }
}
#[derive(Debug, Clone)]
pub struct IfStatement<'tree> {
    pub span: ::treesitter_types::Span,
    pub condition: ::std::vec::Vec<IfStatementCondition<'tree>>,
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
            condition: {
                let mut cursor = node.walk();
                let mut items = ::std::vec::Vec::new();
                for child in node.children_by_field_name("condition", &mut cursor) {
                    items.push(
                        <IfStatementCondition as ::treesitter_types::FromNode>::from_node(
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
pub struct List<'tree> {
    pub span: ::treesitter_types::Span,
    pub children: ::std::vec::Vec<Statement<'tree>>,
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
                    items.push(<Statement as ::treesitter_types::FromNode>::from_node(
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
#[derive(Debug, Clone)]
pub struct NegatedCommand<'tree> {
    pub span: ::treesitter_types::Span,
    pub children: NegatedCommandChildren<'tree>,
}
impl<'tree> ::treesitter_types::FromNode<'tree> for NegatedCommand<'tree> {
    #[allow(clippy::match_single_binding, clippy::suspicious_else_formatting)]
    fn from_node(
        node: ::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        debug_assert_eq!(node.kind(), "negated_command");
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
                                        <NegatedCommandChildren as ::treesitter_types::FromNode>::from_node(
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
                    fallback_child.ok_or_else(|| {
                        ::treesitter_types::ParseError::missing_field("children", node)
                    })?
                };
                <NegatedCommandChildren as ::treesitter_types::FromNode>::from_node(child, src)?
            },
        })
    }
}
impl ::treesitter_types::Spanned for NegatedCommand<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        self.span
    }
}
#[derive(Debug, Clone)]
pub struct Number<'tree> {
    pub span: ::treesitter_types::Span,
    pub children: ::core::option::Option<NumberChildren<'tree>>,
}
impl<'tree> ::treesitter_types::FromNode<'tree> for Number<'tree> {
    #[allow(clippy::match_single_binding, clippy::suspicious_else_formatting)]
    fn from_node(
        node: ::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        debug_assert_eq!(node.kind(), "number");
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
                        <NumberChildren as ::treesitter_types::FromNode>::from_node(child, src)?,
                    ),
                    None => None,
                }
            },
        })
    }
}
impl ::treesitter_types::Spanned for Number<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        self.span
    }
}
#[derive(Debug, Clone)]
pub struct ParenthesizedExpression<'tree> {
    pub span: ::treesitter_types::Span,
    pub children: ::std::vec::Vec<ParenthesizedExpressionChildren<'tree>>,
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
                let mut items = ::std::vec::Vec::new();
                for child in non_field_children {
                    items
                        .push(
                            <ParenthesizedExpressionChildren as ::treesitter_types::FromNode>::from_node(
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
impl ::treesitter_types::Spanned for ParenthesizedExpression<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        self.span
    }
}
#[derive(Debug, Clone)]
pub struct Pipeline<'tree> {
    pub span: ::treesitter_types::Span,
    pub children: ::std::vec::Vec<Statement<'tree>>,
}
impl<'tree> ::treesitter_types::FromNode<'tree> for Pipeline<'tree> {
    #[allow(clippy::match_single_binding, clippy::suspicious_else_formatting)]
    fn from_node(
        node: ::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        debug_assert_eq!(node.kind(), "pipeline");
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
                    items.push(<Statement as ::treesitter_types::FromNode>::from_node(
                        child, src,
                    )?);
                }
                items
            },
        })
    }
}
impl ::treesitter_types::Spanned for Pipeline<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        self.span
    }
}
#[derive(Debug, Clone)]
pub struct PostfixExpression<'tree> {
    pub span: ::treesitter_types::Span,
    pub operator: PostfixExpressionOperator,
    pub children: PostfixExpressionChildren<'tree>,
}
impl<'tree> ::treesitter_types::FromNode<'tree> for PostfixExpression<'tree> {
    #[allow(clippy::match_single_binding, clippy::suspicious_else_formatting)]
    fn from_node(
        node: ::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        debug_assert_eq!(node.kind(), "postfix_expression");
        Ok(Self {
            span: ::treesitter_types::Span::from(node),
            operator: {
                let child = node.child_by_field_name("operator").ok_or_else(|| {
                    ::treesitter_types::ParseError::missing_field("operator", node)
                })?;
                <PostfixExpressionOperator as ::treesitter_types::FromNode>::from_node(child, src)?
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
                                        <PostfixExpressionChildren as ::treesitter_types::FromNode>::from_node(
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
                    fallback_child.ok_or_else(|| {
                        ::treesitter_types::ParseError::missing_field("children", node)
                    })?
                };
                <PostfixExpressionChildren as ::treesitter_types::FromNode>::from_node(child, src)?
            },
        })
    }
}
impl ::treesitter_types::Spanned for PostfixExpression<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        self.span
    }
}
#[derive(Debug, Clone)]
pub struct ProcessSubstitution<'tree> {
    pub span: ::treesitter_types::Span,
    pub children: ::std::vec::Vec<Statement<'tree>>,
}
impl<'tree> ::treesitter_types::FromNode<'tree> for ProcessSubstitution<'tree> {
    #[allow(clippy::match_single_binding, clippy::suspicious_else_formatting)]
    fn from_node(
        node: ::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        debug_assert_eq!(node.kind(), "process_substitution");
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
                    items.push(<Statement as ::treesitter_types::FromNode>::from_node(
                        child, src,
                    )?);
                }
                items
            },
        })
    }
}
impl ::treesitter_types::Spanned for ProcessSubstitution<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        self.span
    }
}
#[derive(Debug, Clone)]
pub struct Program<'tree> {
    pub span: ::treesitter_types::Span,
    pub children: ::std::vec::Vec<Statement<'tree>>,
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
                    items.push(<Statement as ::treesitter_types::FromNode>::from_node(
                        child, src,
                    )?);
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
pub struct RedirectedStatement<'tree> {
    pub span: ::treesitter_types::Span,
    pub body: ::core::option::Option<Statement<'tree>>,
    pub redirect: ::std::vec::Vec<RedirectedStatementRedirect<'tree>>,
    pub children: ::core::option::Option<HerestringRedirect<'tree>>,
}
impl<'tree> ::treesitter_types::FromNode<'tree> for RedirectedStatement<'tree> {
    #[allow(clippy::match_single_binding, clippy::suspicious_else_formatting)]
    fn from_node(
        node: ::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        debug_assert_eq!(node.kind(), "redirected_statement");
        Ok(Self {
            span: ::treesitter_types::Span::from(node),
            body: match node.child_by_field_name("body") {
                Some(child) => Some(<Statement as ::treesitter_types::FromNode>::from_node(
                    child, src,
                )?),
                None => None,
            },
            redirect: {
                let mut cursor = node.walk();
                let mut items = ::std::vec::Vec::new();
                for child in node.children_by_field_name("redirect", &mut cursor) {
                    items.push(
                        <RedirectedStatementRedirect as ::treesitter_types::FromNode>::from_node(
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
                match non_field_children.first() {
                    Some(&child) => Some(
                        <HerestringRedirect as ::treesitter_types::FromNode>::from_node(
                            child, src,
                        )?,
                    ),
                    None => None,
                }
            },
        })
    }
}
impl ::treesitter_types::Spanned for RedirectedStatement<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        self.span
    }
}
#[derive(Debug, Clone)]
pub struct SimpleExpansion<'tree> {
    pub span: ::treesitter_types::Span,
    pub children: SimpleExpansionChildren<'tree>,
}
impl<'tree> ::treesitter_types::FromNode<'tree> for SimpleExpansion<'tree> {
    #[allow(clippy::match_single_binding, clippy::suspicious_else_formatting)]
    fn from_node(
        node: ::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        debug_assert_eq!(node.kind(), "simple_expansion");
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
                                        <SimpleExpansionChildren as ::treesitter_types::FromNode>::from_node(
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
                    fallback_child.ok_or_else(|| {
                        ::treesitter_types::ParseError::missing_field("children", node)
                    })?
                };
                <SimpleExpansionChildren as ::treesitter_types::FromNode>::from_node(child, src)?
            },
        })
    }
}
impl ::treesitter_types::Spanned for SimpleExpansion<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        self.span
    }
}
#[derive(Debug, Clone)]
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
#[derive(Debug, Clone)]
pub struct Subscript<'tree> {
    pub span: ::treesitter_types::Span,
    pub index: SubscriptIndex<'tree>,
    pub name: VariableName<'tree>,
}
impl<'tree> ::treesitter_types::FromNode<'tree> for Subscript<'tree> {
    #[allow(clippy::match_single_binding, clippy::suspicious_else_formatting)]
    fn from_node(
        node: ::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        debug_assert_eq!(node.kind(), "subscript");
        Ok(Self {
            span: ::treesitter_types::Span::from(node),
            index: {
                let child = node
                    .child_by_field_name("index")
                    .ok_or_else(|| ::treesitter_types::ParseError::missing_field("index", node))?;
                <SubscriptIndex as ::treesitter_types::FromNode>::from_node(child, src)?
            },
            name: {
                let child = node
                    .child_by_field_name("name")
                    .ok_or_else(|| ::treesitter_types::ParseError::missing_field("name", node))?;
                <VariableName as ::treesitter_types::FromNode>::from_node(child, src)?
            },
        })
    }
}
impl ::treesitter_types::Spanned for Subscript<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        self.span
    }
}
#[derive(Debug, Clone)]
pub struct Subshell<'tree> {
    pub span: ::treesitter_types::Span,
    pub children: ::std::vec::Vec<Statement<'tree>>,
}
impl<'tree> ::treesitter_types::FromNode<'tree> for Subshell<'tree> {
    #[allow(clippy::match_single_binding, clippy::suspicious_else_formatting)]
    fn from_node(
        node: ::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        debug_assert_eq!(node.kind(), "subshell");
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
                    items.push(<Statement as ::treesitter_types::FromNode>::from_node(
                        child, src,
                    )?);
                }
                items
            },
        })
    }
}
impl ::treesitter_types::Spanned for Subshell<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        self.span
    }
}
#[derive(Debug, Clone)]
pub struct TernaryExpression<'tree> {
    pub span: ::treesitter_types::Span,
    pub alternative: TernaryExpressionAlternative<'tree>,
    pub condition: TernaryExpressionCondition<'tree>,
    pub consequence: TernaryExpressionConsequence<'tree>,
}
impl<'tree> ::treesitter_types::FromNode<'tree> for TernaryExpression<'tree> {
    #[allow(clippy::match_single_binding, clippy::suspicious_else_formatting)]
    fn from_node(
        node: ::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        debug_assert_eq!(node.kind(), "ternary_expression");
        Ok(Self {
            span: ::treesitter_types::Span::from(node),
            alternative: {
                let child = node.child_by_field_name("alternative").ok_or_else(|| {
                    ::treesitter_types::ParseError::missing_field("alternative", node)
                })?;
                <TernaryExpressionAlternative as ::treesitter_types::FromNode>::from_node(
                    child, src,
                )?
            },
            condition: {
                let child = node.child_by_field_name("condition").ok_or_else(|| {
                    ::treesitter_types::ParseError::missing_field("condition", node)
                })?;
                <TernaryExpressionCondition as ::treesitter_types::FromNode>::from_node(child, src)?
            },
            consequence: {
                let child = node.child_by_field_name("consequence").ok_or_else(|| {
                    ::treesitter_types::ParseError::missing_field("consequence", node)
                })?;
                <TernaryExpressionConsequence as ::treesitter_types::FromNode>::from_node(
                    child, src,
                )?
            },
        })
    }
}
impl ::treesitter_types::Spanned for TernaryExpression<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        self.span
    }
}
#[derive(Debug, Clone)]
pub struct TestCommand<'tree> {
    pub span: ::treesitter_types::Span,
    pub children: ::core::option::Option<TestCommandChildren<'tree>>,
}
impl<'tree> ::treesitter_types::FromNode<'tree> for TestCommand<'tree> {
    #[allow(clippy::match_single_binding, clippy::suspicious_else_formatting)]
    fn from_node(
        node: ::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        debug_assert_eq!(node.kind(), "test_command");
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
                        <TestCommandChildren as ::treesitter_types::FromNode>::from_node(
                            child, src,
                        )?,
                    ),
                    None => None,
                }
            },
        })
    }
}
impl ::treesitter_types::Spanned for TestCommand<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        self.span
    }
}
#[derive(Debug, Clone)]
pub struct TranslatedString<'tree> {
    pub span: ::treesitter_types::Span,
    pub children: String<'tree>,
}
impl<'tree> ::treesitter_types::FromNode<'tree> for TranslatedString<'tree> {
    #[allow(clippy::match_single_binding, clippy::suspicious_else_formatting)]
    fn from_node(
        node: ::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        debug_assert_eq!(node.kind(), "translated_string");
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
                                        <String as ::treesitter_types::FromNode>::from_node(
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
                    fallback_child.ok_or_else(|| {
                        ::treesitter_types::ParseError::missing_field("children", node)
                    })?
                };
                <String as ::treesitter_types::FromNode>::from_node(child, src)?
            },
        })
    }
}
impl ::treesitter_types::Spanned for TranslatedString<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        self.span
    }
}
#[derive(Debug, Clone)]
pub struct UnaryExpression<'tree> {
    pub span: ::treesitter_types::Span,
    pub operator: UnaryExpressionOperator<'tree>,
    pub children: UnaryExpressionChildren<'tree>,
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
            operator: {
                let child = node.child_by_field_name("operator").ok_or_else(|| {
                    ::treesitter_types::ParseError::missing_field("operator", node)
                })?;
                <UnaryExpressionOperator as ::treesitter_types::FromNode>::from_node(child, src)?
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
                                        <UnaryExpressionChildren as ::treesitter_types::FromNode>::from_node(
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
                    fallback_child.ok_or_else(|| {
                        ::treesitter_types::ParseError::missing_field("children", node)
                    })?
                };
                <UnaryExpressionChildren as ::treesitter_types::FromNode>::from_node(child, src)?
            },
        })
    }
}
impl ::treesitter_types::Spanned for UnaryExpression<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        self.span
    }
}
#[derive(Debug, Clone)]
pub struct UnsetCommand<'tree> {
    pub span: ::treesitter_types::Span,
    pub children: ::std::vec::Vec<UnsetCommandChildren<'tree>>,
}
impl<'tree> ::treesitter_types::FromNode<'tree> for UnsetCommand<'tree> {
    #[allow(clippy::match_single_binding, clippy::suspicious_else_formatting)]
    fn from_node(
        node: ::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        debug_assert_eq!(node.kind(), "unset_command");
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
                        <UnsetCommandChildren as ::treesitter_types::FromNode>::from_node(
                            child, src,
                        )?,
                    );
                }
                items
            },
        })
    }
}
impl ::treesitter_types::Spanned for UnsetCommand<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        self.span
    }
}
#[derive(Debug, Clone)]
pub struct VariableAssignment<'tree> {
    pub span: ::treesitter_types::Span,
    pub name: VariableAssignmentName<'tree>,
    pub value: VariableAssignmentValue<'tree>,
}
impl<'tree> ::treesitter_types::FromNode<'tree> for VariableAssignment<'tree> {
    #[allow(clippy::match_single_binding, clippy::suspicious_else_formatting)]
    fn from_node(
        node: ::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        debug_assert_eq!(node.kind(), "variable_assignment");
        Ok(Self {
            span: ::treesitter_types::Span::from(node),
            name: {
                let child = node
                    .child_by_field_name("name")
                    .ok_or_else(|| ::treesitter_types::ParseError::missing_field("name", node))?;
                <VariableAssignmentName as ::treesitter_types::FromNode>::from_node(child, src)?
            },
            value: {
                let child = node
                    .child_by_field_name("value")
                    .ok_or_else(|| ::treesitter_types::ParseError::missing_field("value", node))?;
                <VariableAssignmentValue as ::treesitter_types::FromNode>::from_node(child, src)?
            },
        })
    }
}
impl ::treesitter_types::Spanned for VariableAssignment<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        self.span
    }
}
#[derive(Debug, Clone)]
pub struct VariableAssignments<'tree> {
    pub span: ::treesitter_types::Span,
    pub children: ::std::vec::Vec<VariableAssignment<'tree>>,
}
impl<'tree> ::treesitter_types::FromNode<'tree> for VariableAssignments<'tree> {
    #[allow(clippy::match_single_binding, clippy::suspicious_else_formatting)]
    fn from_node(
        node: ::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        debug_assert_eq!(node.kind(), "variable_assignments");
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
                        <VariableAssignment as ::treesitter_types::FromNode>::from_node(
                            child, src,
                        )?,
                    );
                }
                items
            },
        })
    }
}
impl ::treesitter_types::Spanned for VariableAssignments<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        self.span
    }
}
#[derive(Debug, Clone)]
pub struct WhileStatement<'tree> {
    pub span: ::treesitter_types::Span,
    pub body: DoGroup<'tree>,
    pub condition: ::std::vec::Vec<WhileStatementCondition<'tree>>,
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
            body: {
                let child = node
                    .child_by_field_name("body")
                    .ok_or_else(|| ::treesitter_types::ParseError::missing_field("body", node))?;
                <DoGroup as ::treesitter_types::FromNode>::from_node(child, src)?
            },
            condition: {
                let mut cursor = node.walk();
                let mut items = ::std::vec::Vec::new();
                for child in node.children_by_field_name("condition", &mut cursor) {
                    items.push(
                        <WhileStatementCondition as ::treesitter_types::FromNode>::from_node(
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
pub struct Word<'tree> {
    pub span: ::treesitter_types::Span,
    text: &'tree str,
}
impl<'tree> ::treesitter_types::FromNode<'tree> for Word<'tree> {
    fn from_node(
        node: ::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        debug_assert_eq!(node.kind(), "word");
        Ok(Self {
            span: ::treesitter_types::Span::from(node),
            text: node.utf8_text(src)?,
        })
    }
}
impl<'tree> ::treesitter_types::LeafNode<'tree> for Word<'tree> {
    fn text(&self) -> &'tree str {
        self.text
    }
}
impl ::treesitter_types::Spanned for Word<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        self.span
    }
}
#[derive(Debug, Clone)]
pub struct AnsiCString<'tree> {
    pub span: ::treesitter_types::Span,
    text: &'tree str,
}
impl<'tree> ::treesitter_types::FromNode<'tree> for AnsiCString<'tree> {
    fn from_node(
        node: ::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        debug_assert_eq!(node.kind(), "ansi_c_string");
        Ok(Self {
            span: ::treesitter_types::Span::from(node),
            text: node.utf8_text(src)?,
        })
    }
}
impl<'tree> ::treesitter_types::LeafNode<'tree> for AnsiCString<'tree> {
    fn text(&self) -> &'tree str {
        self.text
    }
}
impl ::treesitter_types::Spanned for AnsiCString<'_> {
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
pub struct ExtglobPattern<'tree> {
    pub span: ::treesitter_types::Span,
    text: &'tree str,
}
impl<'tree> ::treesitter_types::FromNode<'tree> for ExtglobPattern<'tree> {
    fn from_node(
        node: ::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        debug_assert_eq!(node.kind(), "extglob_pattern");
        Ok(Self {
            span: ::treesitter_types::Span::from(node),
            text: node.utf8_text(src)?,
        })
    }
}
impl<'tree> ::treesitter_types::LeafNode<'tree> for ExtglobPattern<'tree> {
    fn text(&self) -> &'tree str {
        self.text
    }
}
impl ::treesitter_types::Spanned for ExtglobPattern<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        self.span
    }
}
#[derive(Debug, Clone)]
pub struct FileDescriptor<'tree> {
    pub span: ::treesitter_types::Span,
    text: &'tree str,
}
impl<'tree> ::treesitter_types::FromNode<'tree> for FileDescriptor<'tree> {
    fn from_node(
        node: ::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        debug_assert_eq!(node.kind(), "file_descriptor");
        Ok(Self {
            span: ::treesitter_types::Span::from(node),
            text: node.utf8_text(src)?,
        })
    }
}
impl<'tree> ::treesitter_types::LeafNode<'tree> for FileDescriptor<'tree> {
    fn text(&self) -> &'tree str {
        self.text
    }
}
impl ::treesitter_types::Spanned for FileDescriptor<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        self.span
    }
}
#[derive(Debug, Clone)]
pub struct HeredocContent<'tree> {
    pub span: ::treesitter_types::Span,
    text: &'tree str,
}
impl<'tree> ::treesitter_types::FromNode<'tree> for HeredocContent<'tree> {
    fn from_node(
        node: ::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        debug_assert_eq!(node.kind(), "heredoc_content");
        Ok(Self {
            span: ::treesitter_types::Span::from(node),
            text: node.utf8_text(src)?,
        })
    }
}
impl<'tree> ::treesitter_types::LeafNode<'tree> for HeredocContent<'tree> {
    fn text(&self) -> &'tree str {
        self.text
    }
}
impl ::treesitter_types::Spanned for HeredocContent<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        self.span
    }
}
#[derive(Debug, Clone)]
pub struct HeredocEnd<'tree> {
    pub span: ::treesitter_types::Span,
    text: &'tree str,
}
impl<'tree> ::treesitter_types::FromNode<'tree> for HeredocEnd<'tree> {
    fn from_node(
        node: ::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        debug_assert_eq!(node.kind(), "heredoc_end");
        Ok(Self {
            span: ::treesitter_types::Span::from(node),
            text: node.utf8_text(src)?,
        })
    }
}
impl<'tree> ::treesitter_types::LeafNode<'tree> for HeredocEnd<'tree> {
    fn text(&self) -> &'tree str {
        self.text
    }
}
impl ::treesitter_types::Spanned for HeredocEnd<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        self.span
    }
}
#[derive(Debug, Clone)]
pub struct HeredocStart<'tree> {
    pub span: ::treesitter_types::Span,
    text: &'tree str,
}
impl<'tree> ::treesitter_types::FromNode<'tree> for HeredocStart<'tree> {
    fn from_node(
        node: ::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        debug_assert_eq!(node.kind(), "heredoc_start");
        Ok(Self {
            span: ::treesitter_types::Span::from(node),
            text: node.utf8_text(src)?,
        })
    }
}
impl<'tree> ::treesitter_types::LeafNode<'tree> for HeredocStart<'tree> {
    fn text(&self) -> &'tree str {
        self.text
    }
}
impl ::treesitter_types::Spanned for HeredocStart<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        self.span
    }
}
#[derive(Debug, Clone)]
pub struct RawString<'tree> {
    pub span: ::treesitter_types::Span,
    text: &'tree str,
}
impl<'tree> ::treesitter_types::FromNode<'tree> for RawString<'tree> {
    fn from_node(
        node: ::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        debug_assert_eq!(node.kind(), "raw_string");
        Ok(Self {
            span: ::treesitter_types::Span::from(node),
            text: node.utf8_text(src)?,
        })
    }
}
impl<'tree> ::treesitter_types::LeafNode<'tree> for RawString<'tree> {
    fn text(&self) -> &'tree str {
        self.text
    }
}
impl ::treesitter_types::Spanned for RawString<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        self.span
    }
}
#[derive(Debug, Clone)]
pub struct Regex<'tree> {
    pub span: ::treesitter_types::Span,
    text: &'tree str,
}
impl<'tree> ::treesitter_types::FromNode<'tree> for Regex<'tree> {
    fn from_node(
        node: ::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        debug_assert_eq!(node.kind(), "regex");
        Ok(Self {
            span: ::treesitter_types::Span::from(node),
            text: node.utf8_text(src)?,
        })
    }
}
impl<'tree> ::treesitter_types::LeafNode<'tree> for Regex<'tree> {
    fn text(&self) -> &'tree str {
        self.text
    }
}
impl ::treesitter_types::Spanned for Regex<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        self.span
    }
}
#[derive(Debug, Clone)]
pub struct SpecialVariableName<'tree> {
    pub span: ::treesitter_types::Span,
    text: &'tree str,
}
impl<'tree> ::treesitter_types::FromNode<'tree> for SpecialVariableName<'tree> {
    fn from_node(
        node: ::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        debug_assert_eq!(node.kind(), "special_variable_name");
        Ok(Self {
            span: ::treesitter_types::Span::from(node),
            text: node.utf8_text(src)?,
        })
    }
}
impl<'tree> ::treesitter_types::LeafNode<'tree> for SpecialVariableName<'tree> {
    fn text(&self) -> &'tree str {
        self.text
    }
}
impl ::treesitter_types::Spanned for SpecialVariableName<'_> {
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
pub struct TestOperator<'tree> {
    pub span: ::treesitter_types::Span,
    text: &'tree str,
}
impl<'tree> ::treesitter_types::FromNode<'tree> for TestOperator<'tree> {
    fn from_node(
        node: ::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        debug_assert_eq!(node.kind(), "test_operator");
        Ok(Self {
            span: ::treesitter_types::Span::from(node),
            text: node.utf8_text(src)?,
        })
    }
}
impl<'tree> ::treesitter_types::LeafNode<'tree> for TestOperator<'tree> {
    fn text(&self) -> &'tree str {
        self.text
    }
}
impl ::treesitter_types::Spanned for TestOperator<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        self.span
    }
}
#[derive(Debug, Clone)]
pub struct VariableName<'tree> {
    pub span: ::treesitter_types::Span,
    text: &'tree str,
}
impl<'tree> ::treesitter_types::FromNode<'tree> for VariableName<'tree> {
    fn from_node(
        node: ::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        debug_assert_eq!(node.kind(), "variable_name");
        Ok(Self {
            span: ::treesitter_types::Span::from(node),
            text: node.utf8_text(src)?,
        })
    }
}
impl<'tree> ::treesitter_types::LeafNode<'tree> for VariableName<'tree> {
    fn text(&self) -> &'tree str {
        self.text
    }
}
impl ::treesitter_types::Spanned for VariableName<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        self.span
    }
}
#[derive(Debug, Clone)]
pub enum ArithmeticExpansionChildren<'tree> {
    BinaryExpression(::std::boxed::Box<BinaryExpression<'tree>>),
    CommandSubstitution(::std::boxed::Box<CommandSubstitution<'tree>>),
    Expansion(::std::boxed::Box<Expansion<'tree>>),
    Number(::std::boxed::Box<Number<'tree>>),
    ParenthesizedExpression(::std::boxed::Box<ParenthesizedExpression<'tree>>),
    PostfixExpression(::std::boxed::Box<PostfixExpression<'tree>>),
    RawString(::std::boxed::Box<RawString<'tree>>),
    SimpleExpansion(::std::boxed::Box<SimpleExpansion<'tree>>),
    String(::std::boxed::Box<String<'tree>>),
    Subscript(::std::boxed::Box<Subscript<'tree>>),
    TernaryExpression(::std::boxed::Box<TernaryExpression<'tree>>),
    UnaryExpression(::std::boxed::Box<UnaryExpression<'tree>>),
    VariableName(::std::boxed::Box<VariableName<'tree>>),
}
impl<'tree> ::treesitter_types::FromNode<'tree> for ArithmeticExpansionChildren<'tree> {
    #[allow(clippy::collapsible_else_if)]
    fn from_node(
        node: ::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        match node.kind() {
            "binary_expression" => Ok(Self::BinaryExpression(::std::boxed::Box::new(
                <BinaryExpression as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "command_substitution" => Ok(Self::CommandSubstitution(::std::boxed::Box::new(
                <CommandSubstitution as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "expansion" => Ok(Self::Expansion(::std::boxed::Box::new(
                <Expansion as ::treesitter_types::FromNode>::from_node(node, src)?,
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
            "postfix_expression" => Ok(Self::PostfixExpression(::std::boxed::Box::new(
                <PostfixExpression as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "raw_string" => Ok(Self::RawString(::std::boxed::Box::new(
                <RawString as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "simple_expansion" => Ok(Self::SimpleExpansion(::std::boxed::Box::new(
                <SimpleExpansion as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "string" => Ok(Self::String(::std::boxed::Box::new(
                <String as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "subscript" => Ok(Self::Subscript(::std::boxed::Box::new(
                <Subscript as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "ternary_expression" => Ok(Self::TernaryExpression(::std::boxed::Box::new(
                <TernaryExpression as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "unary_expression" => Ok(Self::UnaryExpression(::std::boxed::Box::new(
                <UnaryExpression as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "variable_name" => Ok(Self::VariableName(::std::boxed::Box::new(
                <VariableName as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            other => Err(::treesitter_types::ParseError::unexpected_kind(other, node)),
        }
    }
}
impl ::treesitter_types::Spanned for ArithmeticExpansionChildren<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        match self {
            Self::BinaryExpression(inner) => inner.span(),
            Self::CommandSubstitution(inner) => inner.span(),
            Self::Expansion(inner) => inner.span(),
            Self::Number(inner) => inner.span(),
            Self::ParenthesizedExpression(inner) => inner.span(),
            Self::PostfixExpression(inner) => inner.span(),
            Self::RawString(inner) => inner.span(),
            Self::SimpleExpansion(inner) => inner.span(),
            Self::String(inner) => inner.span(),
            Self::Subscript(inner) => inner.span(),
            Self::TernaryExpression(inner) => inner.span(),
            Self::UnaryExpression(inner) => inner.span(),
            Self::VariableName(inner) => inner.span(),
        }
    }
}
#[derive(Debug, Clone)]
pub enum ArrayChildren<'tree> {
    PrimaryExpression(::std::boxed::Box<PrimaryExpression<'tree>>),
    Concatenation(::std::boxed::Box<Concatenation<'tree>>),
}
impl<'tree> ::treesitter_types::FromNode<'tree> for ArrayChildren<'tree> {
    #[allow(clippy::collapsible_else_if)]
    fn from_node(
        node: ::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        match node.kind() {
            "concatenation" => Ok(Self::Concatenation(::std::boxed::Box::new(
                <Concatenation as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            _other => {
                if let Ok(v) =
                    <PrimaryExpression as ::treesitter_types::FromNode>::from_node(node, src)
                {
                    Ok(Self::PrimaryExpression(::std::boxed::Box::new(v)))
                } else {
                    Err(::treesitter_types::ParseError::unexpected_kind(
                        _other, node,
                    ))
                }
            }
        }
    }
}
impl ::treesitter_types::Spanned for ArrayChildren<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        match self {
            Self::PrimaryExpression(inner) => inner.span(),
            Self::Concatenation(inner) => inner.span(),
        }
    }
}
#[derive(Debug, Clone)]
pub enum BinaryExpressionLeft<'tree> {
    Expression(::std::boxed::Box<Expression<'tree>>),
    CommandSubstitution(::std::boxed::Box<CommandSubstitution<'tree>>),
    Expansion(::std::boxed::Box<Expansion<'tree>>),
    Number(::std::boxed::Box<Number<'tree>>),
    RawString(::std::boxed::Box<RawString<'tree>>),
    SimpleExpansion(::std::boxed::Box<SimpleExpansion<'tree>>),
    String(::std::boxed::Box<String<'tree>>),
    Subscript(::std::boxed::Box<Subscript<'tree>>),
    VariableName(::std::boxed::Box<VariableName<'tree>>),
}
impl<'tree> ::treesitter_types::FromNode<'tree> for BinaryExpressionLeft<'tree> {
    #[allow(clippy::collapsible_else_if)]
    fn from_node(
        node: ::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        match node.kind() {
            "command_substitution" => Ok(Self::CommandSubstitution(::std::boxed::Box::new(
                <CommandSubstitution as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "expansion" => Ok(Self::Expansion(::std::boxed::Box::new(
                <Expansion as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "number" => Ok(Self::Number(::std::boxed::Box::new(
                <Number as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "raw_string" => Ok(Self::RawString(::std::boxed::Box::new(
                <RawString as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "simple_expansion" => Ok(Self::SimpleExpansion(::std::boxed::Box::new(
                <SimpleExpansion as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "string" => Ok(Self::String(::std::boxed::Box::new(
                <String as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "subscript" => Ok(Self::Subscript(::std::boxed::Box::new(
                <Subscript as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "variable_name" => Ok(Self::VariableName(::std::boxed::Box::new(
                <VariableName as ::treesitter_types::FromNode>::from_node(node, src)?,
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
impl ::treesitter_types::Spanned for BinaryExpressionLeft<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        match self {
            Self::Expression(inner) => inner.span(),
            Self::CommandSubstitution(inner) => inner.span(),
            Self::Expansion(inner) => inner.span(),
            Self::Number(inner) => inner.span(),
            Self::RawString(inner) => inner.span(),
            Self::SimpleExpansion(inner) => inner.span(),
            Self::String(inner) => inner.span(),
            Self::Subscript(inner) => inner.span(),
            Self::VariableName(inner) => inner.span(),
        }
    }
}
#[derive(Debug, Clone)]
pub enum BinaryExpressionOperator<'tree> {
    NotEq(::treesitter_types::Span),
    Percent(::treesitter_types::Span),
    PercentEq(::treesitter_types::Span),
    Amp(::treesitter_types::Span),
    AmpAmp(::treesitter_types::Span),
    AmpEq(::treesitter_types::Span),
    Star(::treesitter_types::Span),
    StarStar(::treesitter_types::Span),
    StarStarEq(::treesitter_types::Span),
    StarEq(::treesitter_types::Span),
    Plus(::treesitter_types::Span),
    PlusEq(::treesitter_types::Span),
    Minus(::treesitter_types::Span),
    MinusEq(::treesitter_types::Span),
    MinusA(::treesitter_types::Span),
    MinusO(::treesitter_types::Span),
    Slash(::treesitter_types::Span),
    SlashEq(::treesitter_types::Span),
    Lt(::treesitter_types::Span),
    Shl(::treesitter_types::Span),
    ShlEq(::treesitter_types::Span),
    LtEq(::treesitter_types::Span),
    Eq(::treesitter_types::Span),
    EqEq(::treesitter_types::Span),
    EqTilde(::treesitter_types::Span),
    Gt(::treesitter_types::Span),
    GtEq(::treesitter_types::Span),
    Shr(::treesitter_types::Span),
    ShrEq(::treesitter_types::Span),
    Caret(::treesitter_types::Span),
    CaretEq(::treesitter_types::Span),
    TestOperator(::std::boxed::Box<TestOperator<'tree>>),
    Pipe(::treesitter_types::Span),
    PipeEq(::treesitter_types::Span),
    PipePipe(::treesitter_types::Span),
}
impl<'tree> ::treesitter_types::FromNode<'tree> for BinaryExpressionOperator<'tree> {
    #[allow(clippy::collapsible_else_if)]
    fn from_node(
        node: ::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        match node.kind() {
            "!=" => Ok(Self::NotEq(::treesitter_types::Span::from(node))),
            "%" => Ok(Self::Percent(::treesitter_types::Span::from(node))),
            "%=" => Ok(Self::PercentEq(::treesitter_types::Span::from(node))),
            "&" => Ok(Self::Amp(::treesitter_types::Span::from(node))),
            "&&" => Ok(Self::AmpAmp(::treesitter_types::Span::from(node))),
            "&=" => Ok(Self::AmpEq(::treesitter_types::Span::from(node))),
            "*" => Ok(Self::Star(::treesitter_types::Span::from(node))),
            "**" => Ok(Self::StarStar(::treesitter_types::Span::from(node))),
            "**=" => Ok(Self::StarStarEq(::treesitter_types::Span::from(node))),
            "*=" => Ok(Self::StarEq(::treesitter_types::Span::from(node))),
            "+" => Ok(Self::Plus(::treesitter_types::Span::from(node))),
            "+=" => Ok(Self::PlusEq(::treesitter_types::Span::from(node))),
            "-" => Ok(Self::Minus(::treesitter_types::Span::from(node))),
            "-=" => Ok(Self::MinusEq(::treesitter_types::Span::from(node))),
            "-a" => Ok(Self::MinusA(::treesitter_types::Span::from(node))),
            "-o" => Ok(Self::MinusO(::treesitter_types::Span::from(node))),
            "/" => Ok(Self::Slash(::treesitter_types::Span::from(node))),
            "/=" => Ok(Self::SlashEq(::treesitter_types::Span::from(node))),
            "<" => Ok(Self::Lt(::treesitter_types::Span::from(node))),
            "<<" => Ok(Self::Shl(::treesitter_types::Span::from(node))),
            "<<=" => Ok(Self::ShlEq(::treesitter_types::Span::from(node))),
            "<=" => Ok(Self::LtEq(::treesitter_types::Span::from(node))),
            "=" => Ok(Self::Eq(::treesitter_types::Span::from(node))),
            "==" => Ok(Self::EqEq(::treesitter_types::Span::from(node))),
            "=~" => Ok(Self::EqTilde(::treesitter_types::Span::from(node))),
            ">" => Ok(Self::Gt(::treesitter_types::Span::from(node))),
            ">=" => Ok(Self::GtEq(::treesitter_types::Span::from(node))),
            ">>" => Ok(Self::Shr(::treesitter_types::Span::from(node))),
            ">>=" => Ok(Self::ShrEq(::treesitter_types::Span::from(node))),
            "^" => Ok(Self::Caret(::treesitter_types::Span::from(node))),
            "^=" => Ok(Self::CaretEq(::treesitter_types::Span::from(node))),
            "test_operator" => Ok(Self::TestOperator(::std::boxed::Box::new(
                <TestOperator as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "|" => Ok(Self::Pipe(::treesitter_types::Span::from(node))),
            "|=" => Ok(Self::PipeEq(::treesitter_types::Span::from(node))),
            "||" => Ok(Self::PipePipe(::treesitter_types::Span::from(node))),
            other => Err(::treesitter_types::ParseError::unexpected_kind(other, node)),
        }
    }
}
impl ::treesitter_types::Spanned for BinaryExpressionOperator<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        match self {
            Self::NotEq(span) => *span,
            Self::Percent(span) => *span,
            Self::PercentEq(span) => *span,
            Self::Amp(span) => *span,
            Self::AmpAmp(span) => *span,
            Self::AmpEq(span) => *span,
            Self::Star(span) => *span,
            Self::StarStar(span) => *span,
            Self::StarStarEq(span) => *span,
            Self::StarEq(span) => *span,
            Self::Plus(span) => *span,
            Self::PlusEq(span) => *span,
            Self::Minus(span) => *span,
            Self::MinusEq(span) => *span,
            Self::MinusA(span) => *span,
            Self::MinusO(span) => *span,
            Self::Slash(span) => *span,
            Self::SlashEq(span) => *span,
            Self::Lt(span) => *span,
            Self::Shl(span) => *span,
            Self::ShlEq(span) => *span,
            Self::LtEq(span) => *span,
            Self::Eq(span) => *span,
            Self::EqEq(span) => *span,
            Self::EqTilde(span) => *span,
            Self::Gt(span) => *span,
            Self::GtEq(span) => *span,
            Self::Shr(span) => *span,
            Self::ShrEq(span) => *span,
            Self::Caret(span) => *span,
            Self::CaretEq(span) => *span,
            Self::TestOperator(inner) => inner.span(),
            Self::Pipe(span) => *span,
            Self::PipeEq(span) => *span,
            Self::PipePipe(span) => *span,
        }
    }
}
#[derive(Debug, Clone)]
pub enum BinaryExpressionRight<'tree> {
    Expression(::std::boxed::Box<Expression<'tree>>),
    CommandSubstitution(::std::boxed::Box<CommandSubstitution<'tree>>),
    Expansion(::std::boxed::Box<Expansion<'tree>>),
    ExtglobPattern(::std::boxed::Box<ExtglobPattern<'tree>>),
    Number(::std::boxed::Box<Number<'tree>>),
    RawString(::std::boxed::Box<RawString<'tree>>),
    Regex(::std::boxed::Box<Regex<'tree>>),
    SimpleExpansion(::std::boxed::Box<SimpleExpansion<'tree>>),
    String(::std::boxed::Box<String<'tree>>),
    Subscript(::std::boxed::Box<Subscript<'tree>>),
    VariableName(::std::boxed::Box<VariableName<'tree>>),
}
impl<'tree> ::treesitter_types::FromNode<'tree> for BinaryExpressionRight<'tree> {
    #[allow(clippy::collapsible_else_if)]
    fn from_node(
        node: ::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        match node.kind() {
            "command_substitution" => Ok(Self::CommandSubstitution(::std::boxed::Box::new(
                <CommandSubstitution as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "expansion" => Ok(Self::Expansion(::std::boxed::Box::new(
                <Expansion as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "extglob_pattern" => Ok(Self::ExtglobPattern(::std::boxed::Box::new(
                <ExtglobPattern as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "number" => Ok(Self::Number(::std::boxed::Box::new(
                <Number as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "raw_string" => Ok(Self::RawString(::std::boxed::Box::new(
                <RawString as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "regex" => Ok(Self::Regex(::std::boxed::Box::new(
                <Regex as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "simple_expansion" => Ok(Self::SimpleExpansion(::std::boxed::Box::new(
                <SimpleExpansion as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "string" => Ok(Self::String(::std::boxed::Box::new(
                <String as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "subscript" => Ok(Self::Subscript(::std::boxed::Box::new(
                <Subscript as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "variable_name" => Ok(Self::VariableName(::std::boxed::Box::new(
                <VariableName as ::treesitter_types::FromNode>::from_node(node, src)?,
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
impl ::treesitter_types::Spanned for BinaryExpressionRight<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        match self {
            Self::Expression(inner) => inner.span(),
            Self::CommandSubstitution(inner) => inner.span(),
            Self::Expansion(inner) => inner.span(),
            Self::ExtglobPattern(inner) => inner.span(),
            Self::Number(inner) => inner.span(),
            Self::RawString(inner) => inner.span(),
            Self::Regex(inner) => inner.span(),
            Self::SimpleExpansion(inner) => inner.span(),
            Self::String(inner) => inner.span(),
            Self::Subscript(inner) => inner.span(),
            Self::VariableName(inner) => inner.span(),
        }
    }
}
#[derive(Debug, Clone)]
pub enum BinaryExpressionChildren<'tree> {
    BinaryExpression(::std::boxed::Box<BinaryExpression<'tree>>),
    Expansion(::std::boxed::Box<Expansion<'tree>>),
    Number(::std::boxed::Box<Number<'tree>>),
    VariableName(::std::boxed::Box<VariableName<'tree>>),
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
            "expansion" => Ok(Self::Expansion(::std::boxed::Box::new(
                <Expansion as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "number" => Ok(Self::Number(::std::boxed::Box::new(
                <Number as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "variable_name" => Ok(Self::VariableName(::std::boxed::Box::new(
                <VariableName as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            other => Err(::treesitter_types::ParseError::unexpected_kind(other, node)),
        }
    }
}
impl ::treesitter_types::Spanned for BinaryExpressionChildren<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        match self {
            Self::BinaryExpression(inner) => inner.span(),
            Self::Expansion(inner) => inner.span(),
            Self::Number(inner) => inner.span(),
            Self::VariableName(inner) => inner.span(),
        }
    }
}
#[derive(Debug, Clone)]
pub enum CStyleForStatementBody<'tree> {
    CompoundStatement(::std::boxed::Box<CompoundStatement<'tree>>),
    DoGroup(::std::boxed::Box<DoGroup<'tree>>),
}
impl<'tree> ::treesitter_types::FromNode<'tree> for CStyleForStatementBody<'tree> {
    #[allow(clippy::collapsible_else_if)]
    fn from_node(
        node: ::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        match node.kind() {
            "compound_statement" => Ok(Self::CompoundStatement(::std::boxed::Box::new(
                <CompoundStatement as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "do_group" => Ok(Self::DoGroup(::std::boxed::Box::new(
                <DoGroup as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            other => Err(::treesitter_types::ParseError::unexpected_kind(other, node)),
        }
    }
}
impl ::treesitter_types::Spanned for CStyleForStatementBody<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        match self {
            Self::CompoundStatement(inner) => inner.span(),
            Self::DoGroup(inner) => inner.span(),
        }
    }
}
#[derive(Debug, Clone)]
pub enum CStyleForStatementCondition<'tree> {
    Comma(::treesitter_types::Span),
    BinaryExpression(::std::boxed::Box<BinaryExpression<'tree>>),
    CommandSubstitution(::std::boxed::Box<CommandSubstitution<'tree>>),
    Expansion(::std::boxed::Box<Expansion<'tree>>),
    Number(::std::boxed::Box<Number<'tree>>),
    ParenthesizedExpression(::std::boxed::Box<ParenthesizedExpression<'tree>>),
    PostfixExpression(::std::boxed::Box<PostfixExpression<'tree>>),
    SimpleExpansion(::std::boxed::Box<SimpleExpansion<'tree>>),
    String(::std::boxed::Box<String<'tree>>),
    UnaryExpression(::std::boxed::Box<UnaryExpression<'tree>>),
    VariableAssignment(::std::boxed::Box<VariableAssignment<'tree>>),
    Word(::std::boxed::Box<Word<'tree>>),
}
impl<'tree> ::treesitter_types::FromNode<'tree> for CStyleForStatementCondition<'tree> {
    #[allow(clippy::collapsible_else_if)]
    fn from_node(
        node: ::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        match node.kind() {
            "," => Ok(Self::Comma(::treesitter_types::Span::from(node))),
            "binary_expression" => Ok(Self::BinaryExpression(::std::boxed::Box::new(
                <BinaryExpression as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "command_substitution" => Ok(Self::CommandSubstitution(::std::boxed::Box::new(
                <CommandSubstitution as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "expansion" => Ok(Self::Expansion(::std::boxed::Box::new(
                <Expansion as ::treesitter_types::FromNode>::from_node(node, src)?,
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
            "postfix_expression" => Ok(Self::PostfixExpression(::std::boxed::Box::new(
                <PostfixExpression as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "simple_expansion" => Ok(Self::SimpleExpansion(::std::boxed::Box::new(
                <SimpleExpansion as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "string" => Ok(Self::String(::std::boxed::Box::new(
                <String as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "unary_expression" => Ok(Self::UnaryExpression(::std::boxed::Box::new(
                <UnaryExpression as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "variable_assignment" => Ok(Self::VariableAssignment(::std::boxed::Box::new(
                <VariableAssignment as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "word" => Ok(Self::Word(::std::boxed::Box::new(
                <Word as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            other => Err(::treesitter_types::ParseError::unexpected_kind(other, node)),
        }
    }
}
impl ::treesitter_types::Spanned for CStyleForStatementCondition<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        match self {
            Self::Comma(span) => *span,
            Self::BinaryExpression(inner) => inner.span(),
            Self::CommandSubstitution(inner) => inner.span(),
            Self::Expansion(inner) => inner.span(),
            Self::Number(inner) => inner.span(),
            Self::ParenthesizedExpression(inner) => inner.span(),
            Self::PostfixExpression(inner) => inner.span(),
            Self::SimpleExpansion(inner) => inner.span(),
            Self::String(inner) => inner.span(),
            Self::UnaryExpression(inner) => inner.span(),
            Self::VariableAssignment(inner) => inner.span(),
            Self::Word(inner) => inner.span(),
        }
    }
}
#[derive(Debug, Clone)]
pub enum CStyleForStatementInitializer<'tree> {
    Comma(::treesitter_types::Span),
    BinaryExpression(::std::boxed::Box<BinaryExpression<'tree>>),
    CommandSubstitution(::std::boxed::Box<CommandSubstitution<'tree>>),
    Expansion(::std::boxed::Box<Expansion<'tree>>),
    Number(::std::boxed::Box<Number<'tree>>),
    ParenthesizedExpression(::std::boxed::Box<ParenthesizedExpression<'tree>>),
    PostfixExpression(::std::boxed::Box<PostfixExpression<'tree>>),
    SimpleExpansion(::std::boxed::Box<SimpleExpansion<'tree>>),
    String(::std::boxed::Box<String<'tree>>),
    UnaryExpression(::std::boxed::Box<UnaryExpression<'tree>>),
    VariableAssignment(::std::boxed::Box<VariableAssignment<'tree>>),
    Word(::std::boxed::Box<Word<'tree>>),
}
impl<'tree> ::treesitter_types::FromNode<'tree> for CStyleForStatementInitializer<'tree> {
    #[allow(clippy::collapsible_else_if)]
    fn from_node(
        node: ::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        match node.kind() {
            "," => Ok(Self::Comma(::treesitter_types::Span::from(node))),
            "binary_expression" => Ok(Self::BinaryExpression(::std::boxed::Box::new(
                <BinaryExpression as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "command_substitution" => Ok(Self::CommandSubstitution(::std::boxed::Box::new(
                <CommandSubstitution as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "expansion" => Ok(Self::Expansion(::std::boxed::Box::new(
                <Expansion as ::treesitter_types::FromNode>::from_node(node, src)?,
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
            "postfix_expression" => Ok(Self::PostfixExpression(::std::boxed::Box::new(
                <PostfixExpression as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "simple_expansion" => Ok(Self::SimpleExpansion(::std::boxed::Box::new(
                <SimpleExpansion as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "string" => Ok(Self::String(::std::boxed::Box::new(
                <String as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "unary_expression" => Ok(Self::UnaryExpression(::std::boxed::Box::new(
                <UnaryExpression as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "variable_assignment" => Ok(Self::VariableAssignment(::std::boxed::Box::new(
                <VariableAssignment as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "word" => Ok(Self::Word(::std::boxed::Box::new(
                <Word as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            other => Err(::treesitter_types::ParseError::unexpected_kind(other, node)),
        }
    }
}
impl ::treesitter_types::Spanned for CStyleForStatementInitializer<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        match self {
            Self::Comma(span) => *span,
            Self::BinaryExpression(inner) => inner.span(),
            Self::CommandSubstitution(inner) => inner.span(),
            Self::Expansion(inner) => inner.span(),
            Self::Number(inner) => inner.span(),
            Self::ParenthesizedExpression(inner) => inner.span(),
            Self::PostfixExpression(inner) => inner.span(),
            Self::SimpleExpansion(inner) => inner.span(),
            Self::String(inner) => inner.span(),
            Self::UnaryExpression(inner) => inner.span(),
            Self::VariableAssignment(inner) => inner.span(),
            Self::Word(inner) => inner.span(),
        }
    }
}
#[derive(Debug, Clone)]
pub enum CStyleForStatementUpdate<'tree> {
    Comma(::treesitter_types::Span),
    BinaryExpression(::std::boxed::Box<BinaryExpression<'tree>>),
    CommandSubstitution(::std::boxed::Box<CommandSubstitution<'tree>>),
    Expansion(::std::boxed::Box<Expansion<'tree>>),
    Number(::std::boxed::Box<Number<'tree>>),
    ParenthesizedExpression(::std::boxed::Box<ParenthesizedExpression<'tree>>),
    PostfixExpression(::std::boxed::Box<PostfixExpression<'tree>>),
    SimpleExpansion(::std::boxed::Box<SimpleExpansion<'tree>>),
    String(::std::boxed::Box<String<'tree>>),
    UnaryExpression(::std::boxed::Box<UnaryExpression<'tree>>),
    VariableAssignment(::std::boxed::Box<VariableAssignment<'tree>>),
    Word(::std::boxed::Box<Word<'tree>>),
}
impl<'tree> ::treesitter_types::FromNode<'tree> for CStyleForStatementUpdate<'tree> {
    #[allow(clippy::collapsible_else_if)]
    fn from_node(
        node: ::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        match node.kind() {
            "," => Ok(Self::Comma(::treesitter_types::Span::from(node))),
            "binary_expression" => Ok(Self::BinaryExpression(::std::boxed::Box::new(
                <BinaryExpression as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "command_substitution" => Ok(Self::CommandSubstitution(::std::boxed::Box::new(
                <CommandSubstitution as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "expansion" => Ok(Self::Expansion(::std::boxed::Box::new(
                <Expansion as ::treesitter_types::FromNode>::from_node(node, src)?,
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
            "postfix_expression" => Ok(Self::PostfixExpression(::std::boxed::Box::new(
                <PostfixExpression as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "simple_expansion" => Ok(Self::SimpleExpansion(::std::boxed::Box::new(
                <SimpleExpansion as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "string" => Ok(Self::String(::std::boxed::Box::new(
                <String as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "unary_expression" => Ok(Self::UnaryExpression(::std::boxed::Box::new(
                <UnaryExpression as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "variable_assignment" => Ok(Self::VariableAssignment(::std::boxed::Box::new(
                <VariableAssignment as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "word" => Ok(Self::Word(::std::boxed::Box::new(
                <Word as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            other => Err(::treesitter_types::ParseError::unexpected_kind(other, node)),
        }
    }
}
impl ::treesitter_types::Spanned for CStyleForStatementUpdate<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        match self {
            Self::Comma(span) => *span,
            Self::BinaryExpression(inner) => inner.span(),
            Self::CommandSubstitution(inner) => inner.span(),
            Self::Expansion(inner) => inner.span(),
            Self::Number(inner) => inner.span(),
            Self::ParenthesizedExpression(inner) => inner.span(),
            Self::PostfixExpression(inner) => inner.span(),
            Self::SimpleExpansion(inner) => inner.span(),
            Self::String(inner) => inner.span(),
            Self::UnaryExpression(inner) => inner.span(),
            Self::VariableAssignment(inner) => inner.span(),
            Self::Word(inner) => inner.span(),
        }
    }
}
#[derive(Debug, Clone)]
pub enum CaseItemFallthrough {
    SemiAmp(::treesitter_types::Span),
    SemiSemiAmp(::treesitter_types::Span),
}
impl<'tree> ::treesitter_types::FromNode<'tree> for CaseItemFallthrough {
    #[allow(clippy::collapsible_else_if)]
    fn from_node(
        node: ::tree_sitter::Node<'tree>,
        _src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        match node.kind() {
            ";&" => Ok(Self::SemiAmp(::treesitter_types::Span::from(node))),
            ";;&" => Ok(Self::SemiSemiAmp(::treesitter_types::Span::from(node))),
            other => Err(::treesitter_types::ParseError::unexpected_kind(other, node)),
        }
    }
}
impl ::treesitter_types::Spanned for CaseItemFallthrough {
    fn span(&self) -> ::treesitter_types::Span {
        match self {
            Self::SemiAmp(span) => *span,
            Self::SemiSemiAmp(span) => *span,
        }
    }
}
#[derive(Debug, Clone)]
pub enum CaseItemTermination {
    SemiSemi(::treesitter_types::Span),
}
impl<'tree> ::treesitter_types::FromNode<'tree> for CaseItemTermination {
    #[allow(clippy::collapsible_else_if)]
    fn from_node(
        node: ::tree_sitter::Node<'tree>,
        _src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        match node.kind() {
            ";;" => Ok(Self::SemiSemi(::treesitter_types::Span::from(node))),
            other => Err(::treesitter_types::ParseError::unexpected_kind(other, node)),
        }
    }
}
impl ::treesitter_types::Spanned for CaseItemTermination {
    fn span(&self) -> ::treesitter_types::Span {
        match self {
            Self::SemiSemi(span) => *span,
        }
    }
}
#[derive(Debug, Clone)]
pub enum CaseItemValue<'tree> {
    PrimaryExpression(::std::boxed::Box<PrimaryExpression<'tree>>),
    Concatenation(::std::boxed::Box<Concatenation<'tree>>),
    ExtglobPattern(::std::boxed::Box<ExtglobPattern<'tree>>),
}
impl<'tree> ::treesitter_types::FromNode<'tree> for CaseItemValue<'tree> {
    #[allow(clippy::collapsible_else_if)]
    fn from_node(
        node: ::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        match node.kind() {
            "concatenation" => Ok(Self::Concatenation(::std::boxed::Box::new(
                <Concatenation as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "extglob_pattern" => Ok(Self::ExtglobPattern(::std::boxed::Box::new(
                <ExtglobPattern as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            _other => {
                if let Ok(v) =
                    <PrimaryExpression as ::treesitter_types::FromNode>::from_node(node, src)
                {
                    Ok(Self::PrimaryExpression(::std::boxed::Box::new(v)))
                } else {
                    Err(::treesitter_types::ParseError::unexpected_kind(
                        _other, node,
                    ))
                }
            }
        }
    }
}
impl ::treesitter_types::Spanned for CaseItemValue<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        match self {
            Self::PrimaryExpression(inner) => inner.span(),
            Self::Concatenation(inner) => inner.span(),
            Self::ExtglobPattern(inner) => inner.span(),
        }
    }
}
#[derive(Debug, Clone)]
pub enum CaseStatementValue<'tree> {
    PrimaryExpression(::std::boxed::Box<PrimaryExpression<'tree>>),
    Concatenation(::std::boxed::Box<Concatenation<'tree>>),
}
impl<'tree> ::treesitter_types::FromNode<'tree> for CaseStatementValue<'tree> {
    #[allow(clippy::collapsible_else_if)]
    fn from_node(
        node: ::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        match node.kind() {
            "concatenation" => Ok(Self::Concatenation(::std::boxed::Box::new(
                <Concatenation as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            _other => {
                if let Ok(v) =
                    <PrimaryExpression as ::treesitter_types::FromNode>::from_node(node, src)
                {
                    Ok(Self::PrimaryExpression(::std::boxed::Box::new(v)))
                } else {
                    Err(::treesitter_types::ParseError::unexpected_kind(
                        _other, node,
                    ))
                }
            }
        }
    }
}
impl ::treesitter_types::Spanned for CaseStatementValue<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        match self {
            Self::PrimaryExpression(inner) => inner.span(),
            Self::Concatenation(inner) => inner.span(),
        }
    }
}
#[derive(Debug, Clone)]
pub enum CommandArgument<'tree> {
    Dollar(::treesitter_types::Span),
    EqEq(::treesitter_types::Span),
    EqTilde(::treesitter_types::Span),
    PrimaryExpression(::std::boxed::Box<PrimaryExpression<'tree>>),
    Concatenation(::std::boxed::Box<Concatenation<'tree>>),
    Regex(::std::boxed::Box<Regex<'tree>>),
}
impl<'tree> ::treesitter_types::FromNode<'tree> for CommandArgument<'tree> {
    #[allow(clippy::collapsible_else_if)]
    fn from_node(
        node: ::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        match node.kind() {
            "$" => Ok(Self::Dollar(::treesitter_types::Span::from(node))),
            "==" => Ok(Self::EqEq(::treesitter_types::Span::from(node))),
            "=~" => Ok(Self::EqTilde(::treesitter_types::Span::from(node))),
            "concatenation" => Ok(Self::Concatenation(::std::boxed::Box::new(
                <Concatenation as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "regex" => Ok(Self::Regex(::std::boxed::Box::new(
                <Regex as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            _other => {
                if let Ok(v) =
                    <PrimaryExpression as ::treesitter_types::FromNode>::from_node(node, src)
                {
                    Ok(Self::PrimaryExpression(::std::boxed::Box::new(v)))
                } else {
                    Err(::treesitter_types::ParseError::unexpected_kind(
                        _other, node,
                    ))
                }
            }
        }
    }
}
impl ::treesitter_types::Spanned for CommandArgument<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        match self {
            Self::Dollar(span) => *span,
            Self::EqEq(span) => *span,
            Self::EqTilde(span) => *span,
            Self::PrimaryExpression(inner) => inner.span(),
            Self::Concatenation(inner) => inner.span(),
            Self::Regex(inner) => inner.span(),
        }
    }
}
#[derive(Debug, Clone)]
pub enum CommandRedirect<'tree> {
    FileRedirect(::std::boxed::Box<FileRedirect<'tree>>),
    HerestringRedirect(::std::boxed::Box<HerestringRedirect<'tree>>),
}
impl<'tree> ::treesitter_types::FromNode<'tree> for CommandRedirect<'tree> {
    #[allow(clippy::collapsible_else_if)]
    fn from_node(
        node: ::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        match node.kind() {
            "file_redirect" => Ok(Self::FileRedirect(::std::boxed::Box::new(
                <FileRedirect as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "herestring_redirect" => Ok(Self::HerestringRedirect(::std::boxed::Box::new(
                <HerestringRedirect as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            other => Err(::treesitter_types::ParseError::unexpected_kind(other, node)),
        }
    }
}
impl ::treesitter_types::Spanned for CommandRedirect<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        match self {
            Self::FileRedirect(inner) => inner.span(),
            Self::HerestringRedirect(inner) => inner.span(),
        }
    }
}
#[derive(Debug, Clone)]
pub enum CommandChildren<'tree> {
    Subshell(::std::boxed::Box<Subshell<'tree>>),
    VariableAssignment(::std::boxed::Box<VariableAssignment<'tree>>),
}
impl<'tree> ::treesitter_types::FromNode<'tree> for CommandChildren<'tree> {
    #[allow(clippy::collapsible_else_if)]
    fn from_node(
        node: ::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        match node.kind() {
            "subshell" => Ok(Self::Subshell(::std::boxed::Box::new(
                <Subshell as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "variable_assignment" => Ok(Self::VariableAssignment(::std::boxed::Box::new(
                <VariableAssignment as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            other => Err(::treesitter_types::ParseError::unexpected_kind(other, node)),
        }
    }
}
impl ::treesitter_types::Spanned for CommandChildren<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        match self {
            Self::Subshell(inner) => inner.span(),
            Self::VariableAssignment(inner) => inner.span(),
        }
    }
}
#[derive(Debug, Clone)]
pub enum CommandNameChildren<'tree> {
    PrimaryExpression(::std::boxed::Box<PrimaryExpression<'tree>>),
    Concatenation(::std::boxed::Box<Concatenation<'tree>>),
}
impl<'tree> ::treesitter_types::FromNode<'tree> for CommandNameChildren<'tree> {
    #[allow(clippy::collapsible_else_if)]
    fn from_node(
        node: ::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        match node.kind() {
            "concatenation" => Ok(Self::Concatenation(::std::boxed::Box::new(
                <Concatenation as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            _other => {
                if let Ok(v) =
                    <PrimaryExpression as ::treesitter_types::FromNode>::from_node(node, src)
                {
                    Ok(Self::PrimaryExpression(::std::boxed::Box::new(v)))
                } else {
                    Err(::treesitter_types::ParseError::unexpected_kind(
                        _other, node,
                    ))
                }
            }
        }
    }
}
impl ::treesitter_types::Spanned for CommandNameChildren<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        match self {
            Self::PrimaryExpression(inner) => inner.span(),
            Self::Concatenation(inner) => inner.span(),
        }
    }
}
#[derive(Debug, Clone)]
pub enum CompoundStatementChildren<'tree> {
    Statement(::std::boxed::Box<Statement<'tree>>),
    BinaryExpression(::std::boxed::Box<BinaryExpression<'tree>>),
    CommandSubstitution(::std::boxed::Box<CommandSubstitution<'tree>>),
    Expansion(::std::boxed::Box<Expansion<'tree>>),
    Number(::std::boxed::Box<Number<'tree>>),
    ParenthesizedExpression(::std::boxed::Box<ParenthesizedExpression<'tree>>),
    PostfixExpression(::std::boxed::Box<PostfixExpression<'tree>>),
    RawString(::std::boxed::Box<RawString<'tree>>),
    SimpleExpansion(::std::boxed::Box<SimpleExpansion<'tree>>),
    String(::std::boxed::Box<String<'tree>>),
    Subscript(::std::boxed::Box<Subscript<'tree>>),
    TernaryExpression(::std::boxed::Box<TernaryExpression<'tree>>),
    UnaryExpression(::std::boxed::Box<UnaryExpression<'tree>>),
    VariableName(::std::boxed::Box<VariableName<'tree>>),
}
impl<'tree> ::treesitter_types::FromNode<'tree> for CompoundStatementChildren<'tree> {
    #[allow(clippy::collapsible_else_if)]
    fn from_node(
        node: ::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        match node.kind() {
            "binary_expression" => Ok(Self::BinaryExpression(::std::boxed::Box::new(
                <BinaryExpression as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "command_substitution" => Ok(Self::CommandSubstitution(::std::boxed::Box::new(
                <CommandSubstitution as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "expansion" => Ok(Self::Expansion(::std::boxed::Box::new(
                <Expansion as ::treesitter_types::FromNode>::from_node(node, src)?,
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
            "postfix_expression" => Ok(Self::PostfixExpression(::std::boxed::Box::new(
                <PostfixExpression as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "raw_string" => Ok(Self::RawString(::std::boxed::Box::new(
                <RawString as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "simple_expansion" => Ok(Self::SimpleExpansion(::std::boxed::Box::new(
                <SimpleExpansion as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "string" => Ok(Self::String(::std::boxed::Box::new(
                <String as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "subscript" => Ok(Self::Subscript(::std::boxed::Box::new(
                <Subscript as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "ternary_expression" => Ok(Self::TernaryExpression(::std::boxed::Box::new(
                <TernaryExpression as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "unary_expression" => Ok(Self::UnaryExpression(::std::boxed::Box::new(
                <UnaryExpression as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "variable_name" => Ok(Self::VariableName(::std::boxed::Box::new(
                <VariableName as ::treesitter_types::FromNode>::from_node(node, src)?,
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
impl ::treesitter_types::Spanned for CompoundStatementChildren<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        match self {
            Self::Statement(inner) => inner.span(),
            Self::BinaryExpression(inner) => inner.span(),
            Self::CommandSubstitution(inner) => inner.span(),
            Self::Expansion(inner) => inner.span(),
            Self::Number(inner) => inner.span(),
            Self::ParenthesizedExpression(inner) => inner.span(),
            Self::PostfixExpression(inner) => inner.span(),
            Self::RawString(inner) => inner.span(),
            Self::SimpleExpansion(inner) => inner.span(),
            Self::String(inner) => inner.span(),
            Self::Subscript(inner) => inner.span(),
            Self::TernaryExpression(inner) => inner.span(),
            Self::UnaryExpression(inner) => inner.span(),
            Self::VariableName(inner) => inner.span(),
        }
    }
}
#[derive(Debug, Clone)]
pub enum ConcatenationChildren<'tree> {
    PrimaryExpression(::std::boxed::Box<PrimaryExpression<'tree>>),
    Array(::std::boxed::Box<Array<'tree>>),
    VariableName(::std::boxed::Box<VariableName<'tree>>),
}
impl<'tree> ::treesitter_types::FromNode<'tree> for ConcatenationChildren<'tree> {
    #[allow(clippy::collapsible_else_if)]
    fn from_node(
        node: ::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        match node.kind() {
            "array" => Ok(Self::Array(::std::boxed::Box::new(
                <Array as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "variable_name" => Ok(Self::VariableName(::std::boxed::Box::new(
                <VariableName as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            _other => {
                if let Ok(v) =
                    <PrimaryExpression as ::treesitter_types::FromNode>::from_node(node, src)
                {
                    Ok(Self::PrimaryExpression(::std::boxed::Box::new(v)))
                } else {
                    Err(::treesitter_types::ParseError::unexpected_kind(
                        _other, node,
                    ))
                }
            }
        }
    }
}
impl ::treesitter_types::Spanned for ConcatenationChildren<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        match self {
            Self::PrimaryExpression(inner) => inner.span(),
            Self::Array(inner) => inner.span(),
            Self::VariableName(inner) => inner.span(),
        }
    }
}
#[derive(Debug, Clone)]
pub enum DeclarationCommandChildren<'tree> {
    PrimaryExpression(::std::boxed::Box<PrimaryExpression<'tree>>),
    Concatenation(::std::boxed::Box<Concatenation<'tree>>),
    VariableAssignment(::std::boxed::Box<VariableAssignment<'tree>>),
    VariableName(::std::boxed::Box<VariableName<'tree>>),
}
impl<'tree> ::treesitter_types::FromNode<'tree> for DeclarationCommandChildren<'tree> {
    #[allow(clippy::collapsible_else_if)]
    fn from_node(
        node: ::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        match node.kind() {
            "concatenation" => Ok(Self::Concatenation(::std::boxed::Box::new(
                <Concatenation as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "variable_assignment" => Ok(Self::VariableAssignment(::std::boxed::Box::new(
                <VariableAssignment as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "variable_name" => Ok(Self::VariableName(::std::boxed::Box::new(
                <VariableName as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            _other => {
                if let Ok(v) =
                    <PrimaryExpression as ::treesitter_types::FromNode>::from_node(node, src)
                {
                    Ok(Self::PrimaryExpression(::std::boxed::Box::new(v)))
                } else {
                    Err(::treesitter_types::ParseError::unexpected_kind(
                        _other, node,
                    ))
                }
            }
        }
    }
}
impl ::treesitter_types::Spanned for DeclarationCommandChildren<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        match self {
            Self::PrimaryExpression(inner) => inner.span(),
            Self::Concatenation(inner) => inner.span(),
            Self::VariableAssignment(inner) => inner.span(),
            Self::VariableName(inner) => inner.span(),
        }
    }
}
#[derive(Debug, Clone)]
pub enum ExpansionOperator {
    Bang(::treesitter_types::Span),
    Hash(::treesitter_types::Span),
    HashHash(::treesitter_types::Span),
    Percent(::treesitter_types::Span),
    PercentPercent(::treesitter_types::Span),
    Star(::treesitter_types::Span),
    Plus(::treesitter_types::Span),
    Comma(::treesitter_types::Span),
    CommaComma(::treesitter_types::Span),
    Minus(::treesitter_types::Span),
    Slash(::treesitter_types::Span),
    SlashHash(::treesitter_types::Span),
    SlashPercent(::treesitter_types::Span),
    SlashSlash(::treesitter_types::Span),
    Colon(::treesitter_types::Span),
    ColonPlus(::treesitter_types::Span),
    ColonMinus(::treesitter_types::Span),
    ColonEq(::treesitter_types::Span),
    ColonQuestion(::treesitter_types::Span),
    Eq(::treesitter_types::Span),
    Question(::treesitter_types::Span),
    At(::treesitter_types::Span),
    A(::treesitter_types::Span),
    E(::treesitter_types::Span),
    K(::treesitter_types::Span),
    L(::treesitter_types::Span),
    P(::treesitter_types::Span),
    Q(::treesitter_types::Span),
    U(::treesitter_types::Span),
    Caret(::treesitter_types::Span),
    CaretCaret(::treesitter_types::Span),
}
impl<'tree> ::treesitter_types::FromNode<'tree> for ExpansionOperator {
    #[allow(clippy::collapsible_else_if)]
    fn from_node(
        node: ::tree_sitter::Node<'tree>,
        _src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        match node.kind() {
            "!" => Ok(Self::Bang(::treesitter_types::Span::from(node))),
            "#" => Ok(Self::Hash(::treesitter_types::Span::from(node))),
            "##" => Ok(Self::HashHash(::treesitter_types::Span::from(node))),
            "%" => Ok(Self::Percent(::treesitter_types::Span::from(node))),
            "%%" => Ok(Self::PercentPercent(::treesitter_types::Span::from(node))),
            "*" => Ok(Self::Star(::treesitter_types::Span::from(node))),
            "+" => Ok(Self::Plus(::treesitter_types::Span::from(node))),
            "," => Ok(Self::Comma(::treesitter_types::Span::from(node))),
            ",," => Ok(Self::CommaComma(::treesitter_types::Span::from(node))),
            "-" => Ok(Self::Minus(::treesitter_types::Span::from(node))),
            "/" => Ok(Self::Slash(::treesitter_types::Span::from(node))),
            "/#" => Ok(Self::SlashHash(::treesitter_types::Span::from(node))),
            "/%" => Ok(Self::SlashPercent(::treesitter_types::Span::from(node))),
            "//" => Ok(Self::SlashSlash(::treesitter_types::Span::from(node))),
            ":" => Ok(Self::Colon(::treesitter_types::Span::from(node))),
            ":+" => Ok(Self::ColonPlus(::treesitter_types::Span::from(node))),
            ":-" => Ok(Self::ColonMinus(::treesitter_types::Span::from(node))),
            ":=" => Ok(Self::ColonEq(::treesitter_types::Span::from(node))),
            ":?" => Ok(Self::ColonQuestion(::treesitter_types::Span::from(node))),
            "=" => Ok(Self::Eq(::treesitter_types::Span::from(node))),
            "?" => Ok(Self::Question(::treesitter_types::Span::from(node))),
            "@" => Ok(Self::At(::treesitter_types::Span::from(node))),
            "A" | "a" => Ok(Self::A(::treesitter_types::Span::from(node))),
            "E" => Ok(Self::E(::treesitter_types::Span::from(node))),
            "K" | "k" => Ok(Self::K(::treesitter_types::Span::from(node))),
            "L" => Ok(Self::L(::treesitter_types::Span::from(node))),
            "P" => Ok(Self::P(::treesitter_types::Span::from(node))),
            "Q" => Ok(Self::Q(::treesitter_types::Span::from(node))),
            "U" | "u" => Ok(Self::U(::treesitter_types::Span::from(node))),
            "^" => Ok(Self::Caret(::treesitter_types::Span::from(node))),
            "^^" => Ok(Self::CaretCaret(::treesitter_types::Span::from(node))),
            other => Err(::treesitter_types::ParseError::unexpected_kind(other, node)),
        }
    }
}
impl ::treesitter_types::Spanned for ExpansionOperator {
    fn span(&self) -> ::treesitter_types::Span {
        match self {
            Self::Bang(span) => *span,
            Self::Hash(span) => *span,
            Self::HashHash(span) => *span,
            Self::Percent(span) => *span,
            Self::PercentPercent(span) => *span,
            Self::Star(span) => *span,
            Self::Plus(span) => *span,
            Self::Comma(span) => *span,
            Self::CommaComma(span) => *span,
            Self::Minus(span) => *span,
            Self::Slash(span) => *span,
            Self::SlashHash(span) => *span,
            Self::SlashPercent(span) => *span,
            Self::SlashSlash(span) => *span,
            Self::Colon(span) => *span,
            Self::ColonPlus(span) => *span,
            Self::ColonMinus(span) => *span,
            Self::ColonEq(span) => *span,
            Self::ColonQuestion(span) => *span,
            Self::Eq(span) => *span,
            Self::Question(span) => *span,
            Self::At(span) => *span,
            Self::A(span) => *span,
            Self::E(span) => *span,
            Self::K(span) => *span,
            Self::L(span) => *span,
            Self::P(span) => *span,
            Self::Q(span) => *span,
            Self::U(span) => *span,
            Self::Caret(span) => *span,
            Self::CaretCaret(span) => *span,
        }
    }
}
#[derive(Debug, Clone)]
pub enum ExpansionChildren<'tree> {
    PrimaryExpression(::std::boxed::Box<PrimaryExpression<'tree>>),
    Array(::std::boxed::Box<Array<'tree>>),
    BinaryExpression(::std::boxed::Box<BinaryExpression<'tree>>),
    Concatenation(::std::boxed::Box<Concatenation<'tree>>),
    ParenthesizedExpression(::std::boxed::Box<ParenthesizedExpression<'tree>>),
    Regex(::std::boxed::Box<Regex<'tree>>),
    SpecialVariableName(::std::boxed::Box<SpecialVariableName<'tree>>),
    Subscript(::std::boxed::Box<Subscript<'tree>>),
    VariableName(::std::boxed::Box<VariableName<'tree>>),
}
impl<'tree> ::treesitter_types::FromNode<'tree> for ExpansionChildren<'tree> {
    #[allow(clippy::collapsible_else_if)]
    fn from_node(
        node: ::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        match node.kind() {
            "array" => Ok(Self::Array(::std::boxed::Box::new(
                <Array as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "binary_expression" => Ok(Self::BinaryExpression(::std::boxed::Box::new(
                <BinaryExpression as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "concatenation" => Ok(Self::Concatenation(::std::boxed::Box::new(
                <Concatenation as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "parenthesized_expression" => {
                Ok(Self::ParenthesizedExpression(::std::boxed::Box::new(
                    <ParenthesizedExpression as ::treesitter_types::FromNode>::from_node(
                        node, src,
                    )?,
                )))
            }
            "regex" => Ok(Self::Regex(::std::boxed::Box::new(
                <Regex as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "special_variable_name" => Ok(Self::SpecialVariableName(::std::boxed::Box::new(
                <SpecialVariableName as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "subscript" => Ok(Self::Subscript(::std::boxed::Box::new(
                <Subscript as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "variable_name" => Ok(Self::VariableName(::std::boxed::Box::new(
                <VariableName as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            _other => {
                if let Ok(v) =
                    <PrimaryExpression as ::treesitter_types::FromNode>::from_node(node, src)
                {
                    Ok(Self::PrimaryExpression(::std::boxed::Box::new(v)))
                } else {
                    Err(::treesitter_types::ParseError::unexpected_kind(
                        _other, node,
                    ))
                }
            }
        }
    }
}
impl ::treesitter_types::Spanned for ExpansionChildren<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        match self {
            Self::PrimaryExpression(inner) => inner.span(),
            Self::Array(inner) => inner.span(),
            Self::BinaryExpression(inner) => inner.span(),
            Self::Concatenation(inner) => inner.span(),
            Self::ParenthesizedExpression(inner) => inner.span(),
            Self::Regex(inner) => inner.span(),
            Self::SpecialVariableName(inner) => inner.span(),
            Self::Subscript(inner) => inner.span(),
            Self::VariableName(inner) => inner.span(),
        }
    }
}
#[derive(Debug, Clone)]
pub enum FileRedirectDestination<'tree> {
    PrimaryExpression(::std::boxed::Box<PrimaryExpression<'tree>>),
    Concatenation(::std::boxed::Box<Concatenation<'tree>>),
}
impl<'tree> ::treesitter_types::FromNode<'tree> for FileRedirectDestination<'tree> {
    #[allow(clippy::collapsible_else_if)]
    fn from_node(
        node: ::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        match node.kind() {
            "concatenation" => Ok(Self::Concatenation(::std::boxed::Box::new(
                <Concatenation as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            _other => {
                if let Ok(v) =
                    <PrimaryExpression as ::treesitter_types::FromNode>::from_node(node, src)
                {
                    Ok(Self::PrimaryExpression(::std::boxed::Box::new(v)))
                } else {
                    Err(::treesitter_types::ParseError::unexpected_kind(
                        _other, node,
                    ))
                }
            }
        }
    }
}
impl ::treesitter_types::Spanned for FileRedirectDestination<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        match self {
            Self::PrimaryExpression(inner) => inner.span(),
            Self::Concatenation(inner) => inner.span(),
        }
    }
}
#[derive(Debug, Clone)]
pub enum ForStatementValue<'tree> {
    PrimaryExpression(::std::boxed::Box<PrimaryExpression<'tree>>),
    Concatenation(::std::boxed::Box<Concatenation<'tree>>),
}
impl<'tree> ::treesitter_types::FromNode<'tree> for ForStatementValue<'tree> {
    #[allow(clippy::collapsible_else_if)]
    fn from_node(
        node: ::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        match node.kind() {
            "concatenation" => Ok(Self::Concatenation(::std::boxed::Box::new(
                <Concatenation as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            _other => {
                if let Ok(v) =
                    <PrimaryExpression as ::treesitter_types::FromNode>::from_node(node, src)
                {
                    Ok(Self::PrimaryExpression(::std::boxed::Box::new(v)))
                } else {
                    Err(::treesitter_types::ParseError::unexpected_kind(
                        _other, node,
                    ))
                }
            }
        }
    }
}
impl ::treesitter_types::Spanned for ForStatementValue<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        match self {
            Self::PrimaryExpression(inner) => inner.span(),
            Self::Concatenation(inner) => inner.span(),
        }
    }
}
#[derive(Debug, Clone)]
pub enum FunctionDefinitionBody<'tree> {
    CompoundStatement(::std::boxed::Box<CompoundStatement<'tree>>),
    IfStatement(::std::boxed::Box<IfStatement<'tree>>),
    Subshell(::std::boxed::Box<Subshell<'tree>>),
    TestCommand(::std::boxed::Box<TestCommand<'tree>>),
}
impl<'tree> ::treesitter_types::FromNode<'tree> for FunctionDefinitionBody<'tree> {
    #[allow(clippy::collapsible_else_if)]
    fn from_node(
        node: ::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        match node.kind() {
            "compound_statement" => Ok(Self::CompoundStatement(::std::boxed::Box::new(
                <CompoundStatement as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "if_statement" => Ok(Self::IfStatement(::std::boxed::Box::new(
                <IfStatement as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "subshell" => Ok(Self::Subshell(::std::boxed::Box::new(
                <Subshell as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "test_command" => Ok(Self::TestCommand(::std::boxed::Box::new(
                <TestCommand as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            other => Err(::treesitter_types::ParseError::unexpected_kind(other, node)),
        }
    }
}
impl ::treesitter_types::Spanned for FunctionDefinitionBody<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        match self {
            Self::CompoundStatement(inner) => inner.span(),
            Self::IfStatement(inner) => inner.span(),
            Self::Subshell(inner) => inner.span(),
            Self::TestCommand(inner) => inner.span(),
        }
    }
}
#[derive(Debug, Clone)]
pub enum FunctionDefinitionRedirect<'tree> {
    FileRedirect(::std::boxed::Box<FileRedirect<'tree>>),
    HerestringRedirect(::std::boxed::Box<HerestringRedirect<'tree>>),
}
impl<'tree> ::treesitter_types::FromNode<'tree> for FunctionDefinitionRedirect<'tree> {
    #[allow(clippy::collapsible_else_if)]
    fn from_node(
        node: ::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        match node.kind() {
            "file_redirect" => Ok(Self::FileRedirect(::std::boxed::Box::new(
                <FileRedirect as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "herestring_redirect" => Ok(Self::HerestringRedirect(::std::boxed::Box::new(
                <HerestringRedirect as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            other => Err(::treesitter_types::ParseError::unexpected_kind(other, node)),
        }
    }
}
impl ::treesitter_types::Spanned for FunctionDefinitionRedirect<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        match self {
            Self::FileRedirect(inner) => inner.span(),
            Self::HerestringRedirect(inner) => inner.span(),
        }
    }
}
#[derive(Debug, Clone)]
pub enum HeredocBodyChildren<'tree> {
    CommandSubstitution(::std::boxed::Box<CommandSubstitution<'tree>>),
    Expansion(::std::boxed::Box<Expansion<'tree>>),
    HeredocContent(::std::boxed::Box<HeredocContent<'tree>>),
    SimpleExpansion(::std::boxed::Box<SimpleExpansion<'tree>>),
}
impl<'tree> ::treesitter_types::FromNode<'tree> for HeredocBodyChildren<'tree> {
    #[allow(clippy::collapsible_else_if)]
    fn from_node(
        node: ::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        match node.kind() {
            "command_substitution" => Ok(Self::CommandSubstitution(::std::boxed::Box::new(
                <CommandSubstitution as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "expansion" => Ok(Self::Expansion(::std::boxed::Box::new(
                <Expansion as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "heredoc_content" => Ok(Self::HeredocContent(::std::boxed::Box::new(
                <HeredocContent as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "simple_expansion" => Ok(Self::SimpleExpansion(::std::boxed::Box::new(
                <SimpleExpansion as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            other => Err(::treesitter_types::ParseError::unexpected_kind(other, node)),
        }
    }
}
impl ::treesitter_types::Spanned for HeredocBodyChildren<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        match self {
            Self::CommandSubstitution(inner) => inner.span(),
            Self::Expansion(inner) => inner.span(),
            Self::HeredocContent(inner) => inner.span(),
            Self::SimpleExpansion(inner) => inner.span(),
        }
    }
}
#[derive(Debug, Clone)]
pub enum HeredocRedirectArgument<'tree> {
    PrimaryExpression(::std::boxed::Box<PrimaryExpression<'tree>>),
    Concatenation(::std::boxed::Box<Concatenation<'tree>>),
}
impl<'tree> ::treesitter_types::FromNode<'tree> for HeredocRedirectArgument<'tree> {
    #[allow(clippy::collapsible_else_if)]
    fn from_node(
        node: ::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        match node.kind() {
            "concatenation" => Ok(Self::Concatenation(::std::boxed::Box::new(
                <Concatenation as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            _other => {
                if let Ok(v) =
                    <PrimaryExpression as ::treesitter_types::FromNode>::from_node(node, src)
                {
                    Ok(Self::PrimaryExpression(::std::boxed::Box::new(v)))
                } else {
                    Err(::treesitter_types::ParseError::unexpected_kind(
                        _other, node,
                    ))
                }
            }
        }
    }
}
impl ::treesitter_types::Spanned for HeredocRedirectArgument<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        match self {
            Self::PrimaryExpression(inner) => inner.span(),
            Self::Concatenation(inner) => inner.span(),
        }
    }
}
#[derive(Debug, Clone)]
pub enum HeredocRedirectOperator {
    AmpAmp(::treesitter_types::Span),
    PipePipe(::treesitter_types::Span),
}
impl<'tree> ::treesitter_types::FromNode<'tree> for HeredocRedirectOperator {
    #[allow(clippy::collapsible_else_if)]
    fn from_node(
        node: ::tree_sitter::Node<'tree>,
        _src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        match node.kind() {
            "&&" => Ok(Self::AmpAmp(::treesitter_types::Span::from(node))),
            "||" => Ok(Self::PipePipe(::treesitter_types::Span::from(node))),
            other => Err(::treesitter_types::ParseError::unexpected_kind(other, node)),
        }
    }
}
impl ::treesitter_types::Spanned for HeredocRedirectOperator {
    fn span(&self) -> ::treesitter_types::Span {
        match self {
            Self::AmpAmp(span) => *span,
            Self::PipePipe(span) => *span,
        }
    }
}
#[derive(Debug, Clone)]
pub enum HeredocRedirectRedirect<'tree> {
    FileRedirect(::std::boxed::Box<FileRedirect<'tree>>),
    HerestringRedirect(::std::boxed::Box<HerestringRedirect<'tree>>),
}
impl<'tree> ::treesitter_types::FromNode<'tree> for HeredocRedirectRedirect<'tree> {
    #[allow(clippy::collapsible_else_if)]
    fn from_node(
        node: ::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        match node.kind() {
            "file_redirect" => Ok(Self::FileRedirect(::std::boxed::Box::new(
                <FileRedirect as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "herestring_redirect" => Ok(Self::HerestringRedirect(::std::boxed::Box::new(
                <HerestringRedirect as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            other => Err(::treesitter_types::ParseError::unexpected_kind(other, node)),
        }
    }
}
impl ::treesitter_types::Spanned for HeredocRedirectRedirect<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        match self {
            Self::FileRedirect(inner) => inner.span(),
            Self::HerestringRedirect(inner) => inner.span(),
        }
    }
}
#[derive(Debug, Clone)]
pub enum HeredocRedirectChildren<'tree> {
    HeredocBody(::std::boxed::Box<HeredocBody<'tree>>),
    HeredocEnd(::std::boxed::Box<HeredocEnd<'tree>>),
    HeredocStart(::std::boxed::Box<HeredocStart<'tree>>),
    Pipeline(::std::boxed::Box<Pipeline<'tree>>),
}
impl<'tree> ::treesitter_types::FromNode<'tree> for HeredocRedirectChildren<'tree> {
    #[allow(clippy::collapsible_else_if)]
    fn from_node(
        node: ::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        match node.kind() {
            "heredoc_body" => Ok(Self::HeredocBody(::std::boxed::Box::new(
                <HeredocBody as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "heredoc_end" => Ok(Self::HeredocEnd(::std::boxed::Box::new(
                <HeredocEnd as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "heredoc_start" => Ok(Self::HeredocStart(::std::boxed::Box::new(
                <HeredocStart as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "pipeline" => Ok(Self::Pipeline(::std::boxed::Box::new(
                <Pipeline as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            other => Err(::treesitter_types::ParseError::unexpected_kind(other, node)),
        }
    }
}
impl ::treesitter_types::Spanned for HeredocRedirectChildren<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        match self {
            Self::HeredocBody(inner) => inner.span(),
            Self::HeredocEnd(inner) => inner.span(),
            Self::HeredocStart(inner) => inner.span(),
            Self::Pipeline(inner) => inner.span(),
        }
    }
}
#[derive(Debug, Clone)]
pub enum HerestringRedirectChildren<'tree> {
    PrimaryExpression(::std::boxed::Box<PrimaryExpression<'tree>>),
    Concatenation(::std::boxed::Box<Concatenation<'tree>>),
}
impl<'tree> ::treesitter_types::FromNode<'tree> for HerestringRedirectChildren<'tree> {
    #[allow(clippy::collapsible_else_if)]
    fn from_node(
        node: ::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        match node.kind() {
            "concatenation" => Ok(Self::Concatenation(::std::boxed::Box::new(
                <Concatenation as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            _other => {
                if let Ok(v) =
                    <PrimaryExpression as ::treesitter_types::FromNode>::from_node(node, src)
                {
                    Ok(Self::PrimaryExpression(::std::boxed::Box::new(v)))
                } else {
                    Err(::treesitter_types::ParseError::unexpected_kind(
                        _other, node,
                    ))
                }
            }
        }
    }
}
impl ::treesitter_types::Spanned for HerestringRedirectChildren<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        match self {
            Self::PrimaryExpression(inner) => inner.span(),
            Self::Concatenation(inner) => inner.span(),
        }
    }
}
#[derive(Debug, Clone)]
pub enum IfStatementCondition<'tree> {
    Amp(::treesitter_types::Span),
    Semicolon(::treesitter_types::Span),
    SemiSemi(::treesitter_types::Span),
    Statement(::std::boxed::Box<Statement<'tree>>),
}
impl<'tree> ::treesitter_types::FromNode<'tree> for IfStatementCondition<'tree> {
    #[allow(clippy::collapsible_else_if)]
    fn from_node(
        node: ::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        match node.kind() {
            "&" => Ok(Self::Amp(::treesitter_types::Span::from(node))),
            ";" => Ok(Self::Semicolon(::treesitter_types::Span::from(node))),
            ";;" => Ok(Self::SemiSemi(::treesitter_types::Span::from(node))),
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
impl ::treesitter_types::Spanned for IfStatementCondition<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        match self {
            Self::Amp(span) => *span,
            Self::Semicolon(span) => *span,
            Self::SemiSemi(span) => *span,
            Self::Statement(inner) => inner.span(),
        }
    }
}
#[derive(Debug, Clone)]
pub enum IfStatementChildren<'tree> {
    Statement(::std::boxed::Box<Statement<'tree>>),
    ElifClause(::std::boxed::Box<ElifClause<'tree>>),
    ElseClause(::std::boxed::Box<ElseClause<'tree>>),
}
impl<'tree> ::treesitter_types::FromNode<'tree> for IfStatementChildren<'tree> {
    #[allow(clippy::collapsible_else_if)]
    fn from_node(
        node: ::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        match node.kind() {
            "elif_clause" => Ok(Self::ElifClause(::std::boxed::Box::new(
                <ElifClause as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "else_clause" => Ok(Self::ElseClause(::std::boxed::Box::new(
                <ElseClause as ::treesitter_types::FromNode>::from_node(node, src)?,
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
impl ::treesitter_types::Spanned for IfStatementChildren<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        match self {
            Self::Statement(inner) => inner.span(),
            Self::ElifClause(inner) => inner.span(),
            Self::ElseClause(inner) => inner.span(),
        }
    }
}
#[derive(Debug, Clone)]
pub enum NegatedCommandChildren<'tree> {
    Command(::std::boxed::Box<Command<'tree>>),
    Subshell(::std::boxed::Box<Subshell<'tree>>),
    TestCommand(::std::boxed::Box<TestCommand<'tree>>),
    VariableAssignment(::std::boxed::Box<VariableAssignment<'tree>>),
}
impl<'tree> ::treesitter_types::FromNode<'tree> for NegatedCommandChildren<'tree> {
    #[allow(clippy::collapsible_else_if)]
    fn from_node(
        node: ::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        match node.kind() {
            "command" => Ok(Self::Command(::std::boxed::Box::new(
                <Command as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "subshell" => Ok(Self::Subshell(::std::boxed::Box::new(
                <Subshell as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "test_command" => Ok(Self::TestCommand(::std::boxed::Box::new(
                <TestCommand as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "variable_assignment" => Ok(Self::VariableAssignment(::std::boxed::Box::new(
                <VariableAssignment as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            other => Err(::treesitter_types::ParseError::unexpected_kind(other, node)),
        }
    }
}
impl ::treesitter_types::Spanned for NegatedCommandChildren<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        match self {
            Self::Command(inner) => inner.span(),
            Self::Subshell(inner) => inner.span(),
            Self::TestCommand(inner) => inner.span(),
            Self::VariableAssignment(inner) => inner.span(),
        }
    }
}
#[derive(Debug, Clone)]
pub enum NumberChildren<'tree> {
    CommandSubstitution(::std::boxed::Box<CommandSubstitution<'tree>>),
    Expansion(::std::boxed::Box<Expansion<'tree>>),
}
impl<'tree> ::treesitter_types::FromNode<'tree> for NumberChildren<'tree> {
    #[allow(clippy::collapsible_else_if)]
    fn from_node(
        node: ::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        match node.kind() {
            "command_substitution" => Ok(Self::CommandSubstitution(::std::boxed::Box::new(
                <CommandSubstitution as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "expansion" => Ok(Self::Expansion(::std::boxed::Box::new(
                <Expansion as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            other => Err(::treesitter_types::ParseError::unexpected_kind(other, node)),
        }
    }
}
impl ::treesitter_types::Spanned for NumberChildren<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        match self {
            Self::CommandSubstitution(inner) => inner.span(),
            Self::Expansion(inner) => inner.span(),
        }
    }
}
#[derive(Debug, Clone)]
pub enum ParenthesizedExpressionChildren<'tree> {
    Expression(::std::boxed::Box<Expression<'tree>>),
    CommandSubstitution(::std::boxed::Box<CommandSubstitution<'tree>>),
    Expansion(::std::boxed::Box<Expansion<'tree>>),
    Number(::std::boxed::Box<Number<'tree>>),
    RawString(::std::boxed::Box<RawString<'tree>>),
    SimpleExpansion(::std::boxed::Box<SimpleExpansion<'tree>>),
    String(::std::boxed::Box<String<'tree>>),
    Subscript(::std::boxed::Box<Subscript<'tree>>),
    VariableAssignment(::std::boxed::Box<VariableAssignment<'tree>>),
    VariableName(::std::boxed::Box<VariableName<'tree>>),
}
impl<'tree> ::treesitter_types::FromNode<'tree> for ParenthesizedExpressionChildren<'tree> {
    #[allow(clippy::collapsible_else_if)]
    fn from_node(
        node: ::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        match node.kind() {
            "command_substitution" => Ok(Self::CommandSubstitution(::std::boxed::Box::new(
                <CommandSubstitution as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "expansion" => Ok(Self::Expansion(::std::boxed::Box::new(
                <Expansion as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "number" => Ok(Self::Number(::std::boxed::Box::new(
                <Number as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "raw_string" => Ok(Self::RawString(::std::boxed::Box::new(
                <RawString as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "simple_expansion" => Ok(Self::SimpleExpansion(::std::boxed::Box::new(
                <SimpleExpansion as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "string" => Ok(Self::String(::std::boxed::Box::new(
                <String as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "subscript" => Ok(Self::Subscript(::std::boxed::Box::new(
                <Subscript as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "variable_assignment" => Ok(Self::VariableAssignment(::std::boxed::Box::new(
                <VariableAssignment as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "variable_name" => Ok(Self::VariableName(::std::boxed::Box::new(
                <VariableName as ::treesitter_types::FromNode>::from_node(node, src)?,
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
impl ::treesitter_types::Spanned for ParenthesizedExpressionChildren<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        match self {
            Self::Expression(inner) => inner.span(),
            Self::CommandSubstitution(inner) => inner.span(),
            Self::Expansion(inner) => inner.span(),
            Self::Number(inner) => inner.span(),
            Self::RawString(inner) => inner.span(),
            Self::SimpleExpansion(inner) => inner.span(),
            Self::String(inner) => inner.span(),
            Self::Subscript(inner) => inner.span(),
            Self::VariableAssignment(inner) => inner.span(),
            Self::VariableName(inner) => inner.span(),
        }
    }
}
#[derive(Debug, Clone)]
pub enum PostfixExpressionOperator {
    PlusPlus(::treesitter_types::Span),
    MinusMinus(::treesitter_types::Span),
}
impl<'tree> ::treesitter_types::FromNode<'tree> for PostfixExpressionOperator {
    #[allow(clippy::collapsible_else_if)]
    fn from_node(
        node: ::tree_sitter::Node<'tree>,
        _src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        match node.kind() {
            "++" => Ok(Self::PlusPlus(::treesitter_types::Span::from(node))),
            "--" => Ok(Self::MinusMinus(::treesitter_types::Span::from(node))),
            other => Err(::treesitter_types::ParseError::unexpected_kind(other, node)),
        }
    }
}
impl ::treesitter_types::Spanned for PostfixExpressionOperator {
    fn span(&self) -> ::treesitter_types::Span {
        match self {
            Self::PlusPlus(span) => *span,
            Self::MinusMinus(span) => *span,
        }
    }
}
#[derive(Debug, Clone)]
pub enum PostfixExpressionChildren<'tree> {
    Expression(::std::boxed::Box<Expression<'tree>>),
    CommandSubstitution(::std::boxed::Box<CommandSubstitution<'tree>>),
    Expansion(::std::boxed::Box<Expansion<'tree>>),
    Number(::std::boxed::Box<Number<'tree>>),
    RawString(::std::boxed::Box<RawString<'tree>>),
    SimpleExpansion(::std::boxed::Box<SimpleExpansion<'tree>>),
    String(::std::boxed::Box<String<'tree>>),
    Subscript(::std::boxed::Box<Subscript<'tree>>),
    VariableName(::std::boxed::Box<VariableName<'tree>>),
}
impl<'tree> ::treesitter_types::FromNode<'tree> for PostfixExpressionChildren<'tree> {
    #[allow(clippy::collapsible_else_if)]
    fn from_node(
        node: ::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        match node.kind() {
            "command_substitution" => Ok(Self::CommandSubstitution(::std::boxed::Box::new(
                <CommandSubstitution as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "expansion" => Ok(Self::Expansion(::std::boxed::Box::new(
                <Expansion as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "number" => Ok(Self::Number(::std::boxed::Box::new(
                <Number as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "raw_string" => Ok(Self::RawString(::std::boxed::Box::new(
                <RawString as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "simple_expansion" => Ok(Self::SimpleExpansion(::std::boxed::Box::new(
                <SimpleExpansion as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "string" => Ok(Self::String(::std::boxed::Box::new(
                <String as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "subscript" => Ok(Self::Subscript(::std::boxed::Box::new(
                <Subscript as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "variable_name" => Ok(Self::VariableName(::std::boxed::Box::new(
                <VariableName as ::treesitter_types::FromNode>::from_node(node, src)?,
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
impl ::treesitter_types::Spanned for PostfixExpressionChildren<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        match self {
            Self::Expression(inner) => inner.span(),
            Self::CommandSubstitution(inner) => inner.span(),
            Self::Expansion(inner) => inner.span(),
            Self::Number(inner) => inner.span(),
            Self::RawString(inner) => inner.span(),
            Self::SimpleExpansion(inner) => inner.span(),
            Self::String(inner) => inner.span(),
            Self::Subscript(inner) => inner.span(),
            Self::VariableName(inner) => inner.span(),
        }
    }
}
#[derive(Debug, Clone)]
pub enum RedirectedStatementRedirect<'tree> {
    FileRedirect(::std::boxed::Box<FileRedirect<'tree>>),
    HeredocRedirect(::std::boxed::Box<HeredocRedirect<'tree>>),
    HerestringRedirect(::std::boxed::Box<HerestringRedirect<'tree>>),
}
impl<'tree> ::treesitter_types::FromNode<'tree> for RedirectedStatementRedirect<'tree> {
    #[allow(clippy::collapsible_else_if)]
    fn from_node(
        node: ::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        match node.kind() {
            "file_redirect" => Ok(Self::FileRedirect(::std::boxed::Box::new(
                <FileRedirect as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "heredoc_redirect" => Ok(Self::HeredocRedirect(::std::boxed::Box::new(
                <HeredocRedirect as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "herestring_redirect" => Ok(Self::HerestringRedirect(::std::boxed::Box::new(
                <HerestringRedirect as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            other => Err(::treesitter_types::ParseError::unexpected_kind(other, node)),
        }
    }
}
impl ::treesitter_types::Spanned for RedirectedStatementRedirect<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        match self {
            Self::FileRedirect(inner) => inner.span(),
            Self::HeredocRedirect(inner) => inner.span(),
            Self::HerestringRedirect(inner) => inner.span(),
        }
    }
}
#[derive(Debug, Clone)]
pub enum SimpleExpansionChildren<'tree> {
    SpecialVariableName(::std::boxed::Box<SpecialVariableName<'tree>>),
    VariableName(::std::boxed::Box<VariableName<'tree>>),
}
impl<'tree> ::treesitter_types::FromNode<'tree> for SimpleExpansionChildren<'tree> {
    #[allow(clippy::collapsible_else_if)]
    fn from_node(
        node: ::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        match node.kind() {
            "special_variable_name" => Ok(Self::SpecialVariableName(::std::boxed::Box::new(
                <SpecialVariableName as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "variable_name" => Ok(Self::VariableName(::std::boxed::Box::new(
                <VariableName as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            other => Err(::treesitter_types::ParseError::unexpected_kind(other, node)),
        }
    }
}
impl ::treesitter_types::Spanned for SimpleExpansionChildren<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        match self {
            Self::SpecialVariableName(inner) => inner.span(),
            Self::VariableName(inner) => inner.span(),
        }
    }
}
#[derive(Debug, Clone)]
pub enum StringChildren<'tree> {
    ArithmeticExpansion(::std::boxed::Box<ArithmeticExpansion<'tree>>),
    CommandSubstitution(::std::boxed::Box<CommandSubstitution<'tree>>),
    Expansion(::std::boxed::Box<Expansion<'tree>>),
    SimpleExpansion(::std::boxed::Box<SimpleExpansion<'tree>>),
    StringContent(::std::boxed::Box<StringContent<'tree>>),
}
impl<'tree> ::treesitter_types::FromNode<'tree> for StringChildren<'tree> {
    #[allow(clippy::collapsible_else_if)]
    fn from_node(
        node: ::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        match node.kind() {
            "arithmetic_expansion" => Ok(Self::ArithmeticExpansion(::std::boxed::Box::new(
                <ArithmeticExpansion as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "command_substitution" => Ok(Self::CommandSubstitution(::std::boxed::Box::new(
                <CommandSubstitution as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "expansion" => Ok(Self::Expansion(::std::boxed::Box::new(
                <Expansion as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "simple_expansion" => Ok(Self::SimpleExpansion(::std::boxed::Box::new(
                <SimpleExpansion as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "string_content" => Ok(Self::StringContent(::std::boxed::Box::new(
                <StringContent as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            other => Err(::treesitter_types::ParseError::unexpected_kind(other, node)),
        }
    }
}
impl ::treesitter_types::Spanned for StringChildren<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        match self {
            Self::ArithmeticExpansion(inner) => inner.span(),
            Self::CommandSubstitution(inner) => inner.span(),
            Self::Expansion(inner) => inner.span(),
            Self::SimpleExpansion(inner) => inner.span(),
            Self::StringContent(inner) => inner.span(),
        }
    }
}
#[derive(Debug, Clone)]
pub enum SubscriptIndex<'tree> {
    PrimaryExpression(::std::boxed::Box<PrimaryExpression<'tree>>),
    BinaryExpression(::std::boxed::Box<BinaryExpression<'tree>>),
    CompoundStatement(::std::boxed::Box<CompoundStatement<'tree>>),
    Concatenation(::std::boxed::Box<Concatenation<'tree>>),
    Subshell(::std::boxed::Box<Subshell<'tree>>),
    UnaryExpression(::std::boxed::Box<UnaryExpression<'tree>>),
}
impl<'tree> ::treesitter_types::FromNode<'tree> for SubscriptIndex<'tree> {
    #[allow(clippy::collapsible_else_if)]
    fn from_node(
        node: ::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        match node.kind() {
            "binary_expression" => Ok(Self::BinaryExpression(::std::boxed::Box::new(
                <BinaryExpression as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "compound_statement" => Ok(Self::CompoundStatement(::std::boxed::Box::new(
                <CompoundStatement as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "concatenation" => Ok(Self::Concatenation(::std::boxed::Box::new(
                <Concatenation as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "subshell" => Ok(Self::Subshell(::std::boxed::Box::new(
                <Subshell as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "unary_expression" => Ok(Self::UnaryExpression(::std::boxed::Box::new(
                <UnaryExpression as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            _other => {
                if let Ok(v) =
                    <PrimaryExpression as ::treesitter_types::FromNode>::from_node(node, src)
                {
                    Ok(Self::PrimaryExpression(::std::boxed::Box::new(v)))
                } else {
                    Err(::treesitter_types::ParseError::unexpected_kind(
                        _other, node,
                    ))
                }
            }
        }
    }
}
impl ::treesitter_types::Spanned for SubscriptIndex<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        match self {
            Self::PrimaryExpression(inner) => inner.span(),
            Self::BinaryExpression(inner) => inner.span(),
            Self::CompoundStatement(inner) => inner.span(),
            Self::Concatenation(inner) => inner.span(),
            Self::Subshell(inner) => inner.span(),
            Self::UnaryExpression(inner) => inner.span(),
        }
    }
}
#[derive(Debug, Clone)]
pub enum TernaryExpressionAlternative<'tree> {
    Expression(::std::boxed::Box<Expression<'tree>>),
    CommandSubstitution(::std::boxed::Box<CommandSubstitution<'tree>>),
    Expansion(::std::boxed::Box<Expansion<'tree>>),
    Number(::std::boxed::Box<Number<'tree>>),
    RawString(::std::boxed::Box<RawString<'tree>>),
    SimpleExpansion(::std::boxed::Box<SimpleExpansion<'tree>>),
    String(::std::boxed::Box<String<'tree>>),
    Subscript(::std::boxed::Box<Subscript<'tree>>),
    VariableName(::std::boxed::Box<VariableName<'tree>>),
}
impl<'tree> ::treesitter_types::FromNode<'tree> for TernaryExpressionAlternative<'tree> {
    #[allow(clippy::collapsible_else_if)]
    fn from_node(
        node: ::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        match node.kind() {
            "command_substitution" => Ok(Self::CommandSubstitution(::std::boxed::Box::new(
                <CommandSubstitution as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "expansion" => Ok(Self::Expansion(::std::boxed::Box::new(
                <Expansion as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "number" => Ok(Self::Number(::std::boxed::Box::new(
                <Number as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "raw_string" => Ok(Self::RawString(::std::boxed::Box::new(
                <RawString as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "simple_expansion" => Ok(Self::SimpleExpansion(::std::boxed::Box::new(
                <SimpleExpansion as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "string" => Ok(Self::String(::std::boxed::Box::new(
                <String as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "subscript" => Ok(Self::Subscript(::std::boxed::Box::new(
                <Subscript as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "variable_name" => Ok(Self::VariableName(::std::boxed::Box::new(
                <VariableName as ::treesitter_types::FromNode>::from_node(node, src)?,
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
impl ::treesitter_types::Spanned for TernaryExpressionAlternative<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        match self {
            Self::Expression(inner) => inner.span(),
            Self::CommandSubstitution(inner) => inner.span(),
            Self::Expansion(inner) => inner.span(),
            Self::Number(inner) => inner.span(),
            Self::RawString(inner) => inner.span(),
            Self::SimpleExpansion(inner) => inner.span(),
            Self::String(inner) => inner.span(),
            Self::Subscript(inner) => inner.span(),
            Self::VariableName(inner) => inner.span(),
        }
    }
}
#[derive(Debug, Clone)]
pub enum TernaryExpressionCondition<'tree> {
    Expression(::std::boxed::Box<Expression<'tree>>),
    CommandSubstitution(::std::boxed::Box<CommandSubstitution<'tree>>),
    Expansion(::std::boxed::Box<Expansion<'tree>>),
    Number(::std::boxed::Box<Number<'tree>>),
    RawString(::std::boxed::Box<RawString<'tree>>),
    SimpleExpansion(::std::boxed::Box<SimpleExpansion<'tree>>),
    String(::std::boxed::Box<String<'tree>>),
    Subscript(::std::boxed::Box<Subscript<'tree>>),
    VariableName(::std::boxed::Box<VariableName<'tree>>),
}
impl<'tree> ::treesitter_types::FromNode<'tree> for TernaryExpressionCondition<'tree> {
    #[allow(clippy::collapsible_else_if)]
    fn from_node(
        node: ::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        match node.kind() {
            "command_substitution" => Ok(Self::CommandSubstitution(::std::boxed::Box::new(
                <CommandSubstitution as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "expansion" => Ok(Self::Expansion(::std::boxed::Box::new(
                <Expansion as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "number" => Ok(Self::Number(::std::boxed::Box::new(
                <Number as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "raw_string" => Ok(Self::RawString(::std::boxed::Box::new(
                <RawString as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "simple_expansion" => Ok(Self::SimpleExpansion(::std::boxed::Box::new(
                <SimpleExpansion as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "string" => Ok(Self::String(::std::boxed::Box::new(
                <String as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "subscript" => Ok(Self::Subscript(::std::boxed::Box::new(
                <Subscript as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "variable_name" => Ok(Self::VariableName(::std::boxed::Box::new(
                <VariableName as ::treesitter_types::FromNode>::from_node(node, src)?,
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
impl ::treesitter_types::Spanned for TernaryExpressionCondition<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        match self {
            Self::Expression(inner) => inner.span(),
            Self::CommandSubstitution(inner) => inner.span(),
            Self::Expansion(inner) => inner.span(),
            Self::Number(inner) => inner.span(),
            Self::RawString(inner) => inner.span(),
            Self::SimpleExpansion(inner) => inner.span(),
            Self::String(inner) => inner.span(),
            Self::Subscript(inner) => inner.span(),
            Self::VariableName(inner) => inner.span(),
        }
    }
}
#[derive(Debug, Clone)]
pub enum TernaryExpressionConsequence<'tree> {
    Expression(::std::boxed::Box<Expression<'tree>>),
    CommandSubstitution(::std::boxed::Box<CommandSubstitution<'tree>>),
    Expansion(::std::boxed::Box<Expansion<'tree>>),
    Number(::std::boxed::Box<Number<'tree>>),
    RawString(::std::boxed::Box<RawString<'tree>>),
    SimpleExpansion(::std::boxed::Box<SimpleExpansion<'tree>>),
    String(::std::boxed::Box<String<'tree>>),
    Subscript(::std::boxed::Box<Subscript<'tree>>),
    VariableName(::std::boxed::Box<VariableName<'tree>>),
}
impl<'tree> ::treesitter_types::FromNode<'tree> for TernaryExpressionConsequence<'tree> {
    #[allow(clippy::collapsible_else_if)]
    fn from_node(
        node: ::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        match node.kind() {
            "command_substitution" => Ok(Self::CommandSubstitution(::std::boxed::Box::new(
                <CommandSubstitution as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "expansion" => Ok(Self::Expansion(::std::boxed::Box::new(
                <Expansion as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "number" => Ok(Self::Number(::std::boxed::Box::new(
                <Number as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "raw_string" => Ok(Self::RawString(::std::boxed::Box::new(
                <RawString as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "simple_expansion" => Ok(Self::SimpleExpansion(::std::boxed::Box::new(
                <SimpleExpansion as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "string" => Ok(Self::String(::std::boxed::Box::new(
                <String as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "subscript" => Ok(Self::Subscript(::std::boxed::Box::new(
                <Subscript as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "variable_name" => Ok(Self::VariableName(::std::boxed::Box::new(
                <VariableName as ::treesitter_types::FromNode>::from_node(node, src)?,
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
impl ::treesitter_types::Spanned for TernaryExpressionConsequence<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        match self {
            Self::Expression(inner) => inner.span(),
            Self::CommandSubstitution(inner) => inner.span(),
            Self::Expansion(inner) => inner.span(),
            Self::Number(inner) => inner.span(),
            Self::RawString(inner) => inner.span(),
            Self::SimpleExpansion(inner) => inner.span(),
            Self::String(inner) => inner.span(),
            Self::Subscript(inner) => inner.span(),
            Self::VariableName(inner) => inner.span(),
        }
    }
}
#[derive(Debug, Clone)]
pub enum TestCommandChildren<'tree> {
    Expression(::std::boxed::Box<Expression<'tree>>),
    RedirectedStatement(::std::boxed::Box<RedirectedStatement<'tree>>),
}
impl<'tree> ::treesitter_types::FromNode<'tree> for TestCommandChildren<'tree> {
    #[allow(clippy::collapsible_else_if)]
    fn from_node(
        node: ::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        match node.kind() {
            "redirected_statement" => Ok(Self::RedirectedStatement(::std::boxed::Box::new(
                <RedirectedStatement as ::treesitter_types::FromNode>::from_node(node, src)?,
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
impl ::treesitter_types::Spanned for TestCommandChildren<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        match self {
            Self::Expression(inner) => inner.span(),
            Self::RedirectedStatement(inner) => inner.span(),
        }
    }
}
#[derive(Debug, Clone)]
pub enum UnaryExpressionOperator<'tree> {
    Bang(::treesitter_types::Span),
    Plus(::treesitter_types::Span),
    PlusPlus(::treesitter_types::Span),
    Minus(::treesitter_types::Span),
    MinusMinus(::treesitter_types::Span),
    TestOperator(::std::boxed::Box<TestOperator<'tree>>),
    Tilde(::treesitter_types::Span),
}
impl<'tree> ::treesitter_types::FromNode<'tree> for UnaryExpressionOperator<'tree> {
    #[allow(clippy::collapsible_else_if)]
    fn from_node(
        node: ::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        match node.kind() {
            "!" => Ok(Self::Bang(::treesitter_types::Span::from(node))),
            "+" => Ok(Self::Plus(::treesitter_types::Span::from(node))),
            "++" => Ok(Self::PlusPlus(::treesitter_types::Span::from(node))),
            "-" => Ok(Self::Minus(::treesitter_types::Span::from(node))),
            "--" => Ok(Self::MinusMinus(::treesitter_types::Span::from(node))),
            "test_operator" => Ok(Self::TestOperator(::std::boxed::Box::new(
                <TestOperator as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "~" => Ok(Self::Tilde(::treesitter_types::Span::from(node))),
            other => Err(::treesitter_types::ParseError::unexpected_kind(other, node)),
        }
    }
}
impl ::treesitter_types::Spanned for UnaryExpressionOperator<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        match self {
            Self::Bang(span) => *span,
            Self::Plus(span) => *span,
            Self::PlusPlus(span) => *span,
            Self::Minus(span) => *span,
            Self::MinusMinus(span) => *span,
            Self::TestOperator(inner) => inner.span(),
            Self::Tilde(span) => *span,
        }
    }
}
#[derive(Debug, Clone)]
pub enum UnaryExpressionChildren<'tree> {
    Expression(::std::boxed::Box<Expression<'tree>>),
    CommandSubstitution(::std::boxed::Box<CommandSubstitution<'tree>>),
    Expansion(::std::boxed::Box<Expansion<'tree>>),
    Number(::std::boxed::Box<Number<'tree>>),
    RawString(::std::boxed::Box<RawString<'tree>>),
    SimpleExpansion(::std::boxed::Box<SimpleExpansion<'tree>>),
    String(::std::boxed::Box<String<'tree>>),
    Subscript(::std::boxed::Box<Subscript<'tree>>),
    VariableName(::std::boxed::Box<VariableName<'tree>>),
}
impl<'tree> ::treesitter_types::FromNode<'tree> for UnaryExpressionChildren<'tree> {
    #[allow(clippy::collapsible_else_if)]
    fn from_node(
        node: ::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        match node.kind() {
            "command_substitution" => Ok(Self::CommandSubstitution(::std::boxed::Box::new(
                <CommandSubstitution as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "expansion" => Ok(Self::Expansion(::std::boxed::Box::new(
                <Expansion as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "number" => Ok(Self::Number(::std::boxed::Box::new(
                <Number as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "raw_string" => Ok(Self::RawString(::std::boxed::Box::new(
                <RawString as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "simple_expansion" => Ok(Self::SimpleExpansion(::std::boxed::Box::new(
                <SimpleExpansion as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "string" => Ok(Self::String(::std::boxed::Box::new(
                <String as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "subscript" => Ok(Self::Subscript(::std::boxed::Box::new(
                <Subscript as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "variable_name" => Ok(Self::VariableName(::std::boxed::Box::new(
                <VariableName as ::treesitter_types::FromNode>::from_node(node, src)?,
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
impl ::treesitter_types::Spanned for UnaryExpressionChildren<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        match self {
            Self::Expression(inner) => inner.span(),
            Self::CommandSubstitution(inner) => inner.span(),
            Self::Expansion(inner) => inner.span(),
            Self::Number(inner) => inner.span(),
            Self::RawString(inner) => inner.span(),
            Self::SimpleExpansion(inner) => inner.span(),
            Self::String(inner) => inner.span(),
            Self::Subscript(inner) => inner.span(),
            Self::VariableName(inner) => inner.span(),
        }
    }
}
#[derive(Debug, Clone)]
pub enum UnsetCommandChildren<'tree> {
    PrimaryExpression(::std::boxed::Box<PrimaryExpression<'tree>>),
    Concatenation(::std::boxed::Box<Concatenation<'tree>>),
    VariableName(::std::boxed::Box<VariableName<'tree>>),
}
impl<'tree> ::treesitter_types::FromNode<'tree> for UnsetCommandChildren<'tree> {
    #[allow(clippy::collapsible_else_if)]
    fn from_node(
        node: ::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        match node.kind() {
            "concatenation" => Ok(Self::Concatenation(::std::boxed::Box::new(
                <Concatenation as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "variable_name" => Ok(Self::VariableName(::std::boxed::Box::new(
                <VariableName as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            _other => {
                if let Ok(v) =
                    <PrimaryExpression as ::treesitter_types::FromNode>::from_node(node, src)
                {
                    Ok(Self::PrimaryExpression(::std::boxed::Box::new(v)))
                } else {
                    Err(::treesitter_types::ParseError::unexpected_kind(
                        _other, node,
                    ))
                }
            }
        }
    }
}
impl ::treesitter_types::Spanned for UnsetCommandChildren<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        match self {
            Self::PrimaryExpression(inner) => inner.span(),
            Self::Concatenation(inner) => inner.span(),
            Self::VariableName(inner) => inner.span(),
        }
    }
}
#[derive(Debug, Clone)]
pub enum VariableAssignmentName<'tree> {
    Subscript(::std::boxed::Box<Subscript<'tree>>),
    VariableName(::std::boxed::Box<VariableName<'tree>>),
}
impl<'tree> ::treesitter_types::FromNode<'tree> for VariableAssignmentName<'tree> {
    #[allow(clippy::collapsible_else_if)]
    fn from_node(
        node: ::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        match node.kind() {
            "subscript" => Ok(Self::Subscript(::std::boxed::Box::new(
                <Subscript as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "variable_name" => Ok(Self::VariableName(::std::boxed::Box::new(
                <VariableName as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            other => Err(::treesitter_types::ParseError::unexpected_kind(other, node)),
        }
    }
}
impl ::treesitter_types::Spanned for VariableAssignmentName<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        match self {
            Self::Subscript(inner) => inner.span(),
            Self::VariableName(inner) => inner.span(),
        }
    }
}
#[derive(Debug, Clone)]
pub enum VariableAssignmentValue<'tree> {
    PrimaryExpression(::std::boxed::Box<PrimaryExpression<'tree>>),
    Array(::std::boxed::Box<Array<'tree>>),
    BinaryExpression(::std::boxed::Box<BinaryExpression<'tree>>),
    Concatenation(::std::boxed::Box<Concatenation<'tree>>),
    ParenthesizedExpression(::std::boxed::Box<ParenthesizedExpression<'tree>>),
    PostfixExpression(::std::boxed::Box<PostfixExpression<'tree>>),
    UnaryExpression(::std::boxed::Box<UnaryExpression<'tree>>),
    VariableAssignment(::std::boxed::Box<VariableAssignment<'tree>>),
}
impl<'tree> ::treesitter_types::FromNode<'tree> for VariableAssignmentValue<'tree> {
    #[allow(clippy::collapsible_else_if)]
    fn from_node(
        node: ::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        match node.kind() {
            "array" => Ok(Self::Array(::std::boxed::Box::new(
                <Array as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "binary_expression" => Ok(Self::BinaryExpression(::std::boxed::Box::new(
                <BinaryExpression as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "concatenation" => Ok(Self::Concatenation(::std::boxed::Box::new(
                <Concatenation as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "parenthesized_expression" => {
                Ok(Self::ParenthesizedExpression(::std::boxed::Box::new(
                    <ParenthesizedExpression as ::treesitter_types::FromNode>::from_node(
                        node, src,
                    )?,
                )))
            }
            "postfix_expression" => Ok(Self::PostfixExpression(::std::boxed::Box::new(
                <PostfixExpression as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "unary_expression" => Ok(Self::UnaryExpression(::std::boxed::Box::new(
                <UnaryExpression as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "variable_assignment" => Ok(Self::VariableAssignment(::std::boxed::Box::new(
                <VariableAssignment as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            _other => {
                if let Ok(v) =
                    <PrimaryExpression as ::treesitter_types::FromNode>::from_node(node, src)
                {
                    Ok(Self::PrimaryExpression(::std::boxed::Box::new(v)))
                } else {
                    Err(::treesitter_types::ParseError::unexpected_kind(
                        _other, node,
                    ))
                }
            }
        }
    }
}
impl ::treesitter_types::Spanned for VariableAssignmentValue<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        match self {
            Self::PrimaryExpression(inner) => inner.span(),
            Self::Array(inner) => inner.span(),
            Self::BinaryExpression(inner) => inner.span(),
            Self::Concatenation(inner) => inner.span(),
            Self::ParenthesizedExpression(inner) => inner.span(),
            Self::PostfixExpression(inner) => inner.span(),
            Self::UnaryExpression(inner) => inner.span(),
            Self::VariableAssignment(inner) => inner.span(),
        }
    }
}
#[derive(Debug, Clone)]
pub enum WhileStatementCondition<'tree> {
    Amp(::treesitter_types::Span),
    Semicolon(::treesitter_types::Span),
    SemiSemi(::treesitter_types::Span),
    Statement(::std::boxed::Box<Statement<'tree>>),
}
impl<'tree> ::treesitter_types::FromNode<'tree> for WhileStatementCondition<'tree> {
    #[allow(clippy::collapsible_else_if)]
    fn from_node(
        node: ::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        match node.kind() {
            "&" => Ok(Self::Amp(::treesitter_types::Span::from(node))),
            ";" => Ok(Self::Semicolon(::treesitter_types::Span::from(node))),
            ";;" => Ok(Self::SemiSemi(::treesitter_types::Span::from(node))),
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
impl ::treesitter_types::Spanned for WhileStatementCondition<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        match self {
            Self::Amp(span) => *span,
            Self::Semicolon(span) => *span,
            Self::SemiSemi(span) => *span,
            Self::Statement(inner) => inner.span(),
        }
    }
}
#[derive(Debug, Clone)]
pub enum AnyNode<'tree> {
    Expression(Expression<'tree>),
    PrimaryExpression(PrimaryExpression<'tree>),
    Statement(Statement<'tree>),
    ArithmeticExpansion(ArithmeticExpansion<'tree>),
    Array(Array<'tree>),
    BinaryExpression(BinaryExpression<'tree>),
    BraceExpression(BraceExpression<'tree>),
    CStyleForStatement(CStyleForStatement<'tree>),
    CaseItem(CaseItem<'tree>),
    CaseStatement(CaseStatement<'tree>),
    Command(Command<'tree>),
    CommandName(CommandName<'tree>),
    CommandSubstitution(CommandSubstitution<'tree>),
    CompoundStatement(CompoundStatement<'tree>),
    Concatenation(Concatenation<'tree>),
    DeclarationCommand(DeclarationCommand<'tree>),
    DoGroup(DoGroup<'tree>),
    ElifClause(ElifClause<'tree>),
    ElseClause(ElseClause<'tree>),
    Expansion(Expansion<'tree>),
    FileRedirect(FileRedirect<'tree>),
    ForStatement(ForStatement<'tree>),
    FunctionDefinition(FunctionDefinition<'tree>),
    HeredocBody(HeredocBody<'tree>),
    HeredocRedirect(HeredocRedirect<'tree>),
    HerestringRedirect(HerestringRedirect<'tree>),
    IfStatement(IfStatement<'tree>),
    List(List<'tree>),
    NegatedCommand(NegatedCommand<'tree>),
    Number(Number<'tree>),
    ParenthesizedExpression(ParenthesizedExpression<'tree>),
    Pipeline(Pipeline<'tree>),
    PostfixExpression(PostfixExpression<'tree>),
    ProcessSubstitution(ProcessSubstitution<'tree>),
    Program(Program<'tree>),
    RedirectedStatement(RedirectedStatement<'tree>),
    SimpleExpansion(SimpleExpansion<'tree>),
    String(String<'tree>),
    Subscript(Subscript<'tree>),
    Subshell(Subshell<'tree>),
    TernaryExpression(TernaryExpression<'tree>),
    TestCommand(TestCommand<'tree>),
    TranslatedString(TranslatedString<'tree>),
    UnaryExpression(UnaryExpression<'tree>),
    UnsetCommand(UnsetCommand<'tree>),
    VariableAssignment(VariableAssignment<'tree>),
    VariableAssignments(VariableAssignments<'tree>),
    WhileStatement(WhileStatement<'tree>),
    Word(Word<'tree>),
    AnsiCString(AnsiCString<'tree>),
    Comment(Comment<'tree>),
    ExtglobPattern(ExtglobPattern<'tree>),
    FileDescriptor(FileDescriptor<'tree>),
    HeredocContent(HeredocContent<'tree>),
    HeredocEnd(HeredocEnd<'tree>),
    HeredocStart(HeredocStart<'tree>),
    RawString(RawString<'tree>),
    Regex(Regex<'tree>),
    SpecialVariableName(SpecialVariableName<'tree>),
    StringContent(StringContent<'tree>),
    TestOperator(TestOperator<'tree>),
    VariableName(VariableName<'tree>),
    Unknown(::tree_sitter::Node<'tree>),
}
impl<'tree> AnyNode<'tree> {
    pub fn from_node(node: ::tree_sitter::Node<'tree>, src: &'tree [u8]) -> Self {
        match node.kind() {
            "_expression" => <Expression as ::treesitter_types::FromNode>::from_node(node, src)
                .map(Self::Expression)
                .unwrap_or(Self::Unknown(node)),
            "_primary_expression" => {
                <PrimaryExpression as ::treesitter_types::FromNode>::from_node(node, src)
                    .map(Self::PrimaryExpression)
                    .unwrap_or(Self::Unknown(node))
            }
            "_statement" => <Statement as ::treesitter_types::FromNode>::from_node(node, src)
                .map(Self::Statement)
                .unwrap_or(Self::Unknown(node)),
            "arithmetic_expansion" => {
                <ArithmeticExpansion as ::treesitter_types::FromNode>::from_node(node, src)
                    .map(Self::ArithmeticExpansion)
                    .unwrap_or(Self::Unknown(node))
            }
            "array" => <Array as ::treesitter_types::FromNode>::from_node(node, src)
                .map(Self::Array)
                .unwrap_or(Self::Unknown(node)),
            "binary_expression" => {
                <BinaryExpression as ::treesitter_types::FromNode>::from_node(node, src)
                    .map(Self::BinaryExpression)
                    .unwrap_or(Self::Unknown(node))
            }
            "brace_expression" => {
                <BraceExpression as ::treesitter_types::FromNode>::from_node(node, src)
                    .map(Self::BraceExpression)
                    .unwrap_or(Self::Unknown(node))
            }
            "c_style_for_statement" => {
                <CStyleForStatement as ::treesitter_types::FromNode>::from_node(node, src)
                    .map(Self::CStyleForStatement)
                    .unwrap_or(Self::Unknown(node))
            }
            "case_item" => <CaseItem as ::treesitter_types::FromNode>::from_node(node, src)
                .map(Self::CaseItem)
                .unwrap_or(Self::Unknown(node)),
            "case_statement" => {
                <CaseStatement as ::treesitter_types::FromNode>::from_node(node, src)
                    .map(Self::CaseStatement)
                    .unwrap_or(Self::Unknown(node))
            }
            "command" => <Command as ::treesitter_types::FromNode>::from_node(node, src)
                .map(Self::Command)
                .unwrap_or(Self::Unknown(node)),
            "command_name" => <CommandName as ::treesitter_types::FromNode>::from_node(node, src)
                .map(Self::CommandName)
                .unwrap_or(Self::Unknown(node)),
            "command_substitution" => {
                <CommandSubstitution as ::treesitter_types::FromNode>::from_node(node, src)
                    .map(Self::CommandSubstitution)
                    .unwrap_or(Self::Unknown(node))
            }
            "compound_statement" => {
                <CompoundStatement as ::treesitter_types::FromNode>::from_node(node, src)
                    .map(Self::CompoundStatement)
                    .unwrap_or(Self::Unknown(node))
            }
            "concatenation" => {
                <Concatenation as ::treesitter_types::FromNode>::from_node(node, src)
                    .map(Self::Concatenation)
                    .unwrap_or(Self::Unknown(node))
            }
            "declaration_command" => {
                <DeclarationCommand as ::treesitter_types::FromNode>::from_node(node, src)
                    .map(Self::DeclarationCommand)
                    .unwrap_or(Self::Unknown(node))
            }
            "do_group" => <DoGroup as ::treesitter_types::FromNode>::from_node(node, src)
                .map(Self::DoGroup)
                .unwrap_or(Self::Unknown(node)),
            "elif_clause" => <ElifClause as ::treesitter_types::FromNode>::from_node(node, src)
                .map(Self::ElifClause)
                .unwrap_or(Self::Unknown(node)),
            "else_clause" => <ElseClause as ::treesitter_types::FromNode>::from_node(node, src)
                .map(Self::ElseClause)
                .unwrap_or(Self::Unknown(node)),
            "expansion" => <Expansion as ::treesitter_types::FromNode>::from_node(node, src)
                .map(Self::Expansion)
                .unwrap_or(Self::Unknown(node)),
            "file_redirect" => <FileRedirect as ::treesitter_types::FromNode>::from_node(node, src)
                .map(Self::FileRedirect)
                .unwrap_or(Self::Unknown(node)),
            "for_statement" => <ForStatement as ::treesitter_types::FromNode>::from_node(node, src)
                .map(Self::ForStatement)
                .unwrap_or(Self::Unknown(node)),
            "function_definition" => {
                <FunctionDefinition as ::treesitter_types::FromNode>::from_node(node, src)
                    .map(Self::FunctionDefinition)
                    .unwrap_or(Self::Unknown(node))
            }
            "heredoc_body" => <HeredocBody as ::treesitter_types::FromNode>::from_node(node, src)
                .map(Self::HeredocBody)
                .unwrap_or(Self::Unknown(node)),
            "heredoc_redirect" => {
                <HeredocRedirect as ::treesitter_types::FromNode>::from_node(node, src)
                    .map(Self::HeredocRedirect)
                    .unwrap_or(Self::Unknown(node))
            }
            "herestring_redirect" => {
                <HerestringRedirect as ::treesitter_types::FromNode>::from_node(node, src)
                    .map(Self::HerestringRedirect)
                    .unwrap_or(Self::Unknown(node))
            }
            "if_statement" => <IfStatement as ::treesitter_types::FromNode>::from_node(node, src)
                .map(Self::IfStatement)
                .unwrap_or(Self::Unknown(node)),
            "list" => <List as ::treesitter_types::FromNode>::from_node(node, src)
                .map(Self::List)
                .unwrap_or(Self::Unknown(node)),
            "negated_command" => {
                <NegatedCommand as ::treesitter_types::FromNode>::from_node(node, src)
                    .map(Self::NegatedCommand)
                    .unwrap_or(Self::Unknown(node))
            }
            "number" => <Number as ::treesitter_types::FromNode>::from_node(node, src)
                .map(Self::Number)
                .unwrap_or(Self::Unknown(node)),
            "parenthesized_expression" => {
                <ParenthesizedExpression as ::treesitter_types::FromNode>::from_node(node, src)
                    .map(Self::ParenthesizedExpression)
                    .unwrap_or(Self::Unknown(node))
            }
            "pipeline" => <Pipeline as ::treesitter_types::FromNode>::from_node(node, src)
                .map(Self::Pipeline)
                .unwrap_or(Self::Unknown(node)),
            "postfix_expression" => {
                <PostfixExpression as ::treesitter_types::FromNode>::from_node(node, src)
                    .map(Self::PostfixExpression)
                    .unwrap_or(Self::Unknown(node))
            }
            "process_substitution" => {
                <ProcessSubstitution as ::treesitter_types::FromNode>::from_node(node, src)
                    .map(Self::ProcessSubstitution)
                    .unwrap_or(Self::Unknown(node))
            }
            "program" => <Program as ::treesitter_types::FromNode>::from_node(node, src)
                .map(Self::Program)
                .unwrap_or(Self::Unknown(node)),
            "redirected_statement" => {
                <RedirectedStatement as ::treesitter_types::FromNode>::from_node(node, src)
                    .map(Self::RedirectedStatement)
                    .unwrap_or(Self::Unknown(node))
            }
            "simple_expansion" => {
                <SimpleExpansion as ::treesitter_types::FromNode>::from_node(node, src)
                    .map(Self::SimpleExpansion)
                    .unwrap_or(Self::Unknown(node))
            }
            "string" => <String as ::treesitter_types::FromNode>::from_node(node, src)
                .map(Self::String)
                .unwrap_or(Self::Unknown(node)),
            "subscript" => <Subscript as ::treesitter_types::FromNode>::from_node(node, src)
                .map(Self::Subscript)
                .unwrap_or(Self::Unknown(node)),
            "subshell" => <Subshell as ::treesitter_types::FromNode>::from_node(node, src)
                .map(Self::Subshell)
                .unwrap_or(Self::Unknown(node)),
            "ternary_expression" => {
                <TernaryExpression as ::treesitter_types::FromNode>::from_node(node, src)
                    .map(Self::TernaryExpression)
                    .unwrap_or(Self::Unknown(node))
            }
            "test_command" => <TestCommand as ::treesitter_types::FromNode>::from_node(node, src)
                .map(Self::TestCommand)
                .unwrap_or(Self::Unknown(node)),
            "translated_string" => {
                <TranslatedString as ::treesitter_types::FromNode>::from_node(node, src)
                    .map(Self::TranslatedString)
                    .unwrap_or(Self::Unknown(node))
            }
            "unary_expression" => {
                <UnaryExpression as ::treesitter_types::FromNode>::from_node(node, src)
                    .map(Self::UnaryExpression)
                    .unwrap_or(Self::Unknown(node))
            }
            "unset_command" => <UnsetCommand as ::treesitter_types::FromNode>::from_node(node, src)
                .map(Self::UnsetCommand)
                .unwrap_or(Self::Unknown(node)),
            "variable_assignment" => {
                <VariableAssignment as ::treesitter_types::FromNode>::from_node(node, src)
                    .map(Self::VariableAssignment)
                    .unwrap_or(Self::Unknown(node))
            }
            "variable_assignments" => {
                <VariableAssignments as ::treesitter_types::FromNode>::from_node(node, src)
                    .map(Self::VariableAssignments)
                    .unwrap_or(Self::Unknown(node))
            }
            "while_statement" => {
                <WhileStatement as ::treesitter_types::FromNode>::from_node(node, src)
                    .map(Self::WhileStatement)
                    .unwrap_or(Self::Unknown(node))
            }
            "word" => <Word as ::treesitter_types::FromNode>::from_node(node, src)
                .map(Self::Word)
                .unwrap_or(Self::Unknown(node)),
            "ansi_c_string" => <AnsiCString as ::treesitter_types::FromNode>::from_node(node, src)
                .map(Self::AnsiCString)
                .unwrap_or(Self::Unknown(node)),
            "comment" => <Comment as ::treesitter_types::FromNode>::from_node(node, src)
                .map(Self::Comment)
                .unwrap_or(Self::Unknown(node)),
            "extglob_pattern" => {
                <ExtglobPattern as ::treesitter_types::FromNode>::from_node(node, src)
                    .map(Self::ExtglobPattern)
                    .unwrap_or(Self::Unknown(node))
            }
            "file_descriptor" => {
                <FileDescriptor as ::treesitter_types::FromNode>::from_node(node, src)
                    .map(Self::FileDescriptor)
                    .unwrap_or(Self::Unknown(node))
            }
            "heredoc_content" => {
                <HeredocContent as ::treesitter_types::FromNode>::from_node(node, src)
                    .map(Self::HeredocContent)
                    .unwrap_or(Self::Unknown(node))
            }
            "heredoc_end" => <HeredocEnd as ::treesitter_types::FromNode>::from_node(node, src)
                .map(Self::HeredocEnd)
                .unwrap_or(Self::Unknown(node)),
            "heredoc_start" => <HeredocStart as ::treesitter_types::FromNode>::from_node(node, src)
                .map(Self::HeredocStart)
                .unwrap_or(Self::Unknown(node)),
            "raw_string" => <RawString as ::treesitter_types::FromNode>::from_node(node, src)
                .map(Self::RawString)
                .unwrap_or(Self::Unknown(node)),
            "regex" => <Regex as ::treesitter_types::FromNode>::from_node(node, src)
                .map(Self::Regex)
                .unwrap_or(Self::Unknown(node)),
            "special_variable_name" => {
                <SpecialVariableName as ::treesitter_types::FromNode>::from_node(node, src)
                    .map(Self::SpecialVariableName)
                    .unwrap_or(Self::Unknown(node))
            }
            "string_content" => {
                <StringContent as ::treesitter_types::FromNode>::from_node(node, src)
                    .map(Self::StringContent)
                    .unwrap_or(Self::Unknown(node))
            }
            "test_operator" => <TestOperator as ::treesitter_types::FromNode>::from_node(node, src)
                .map(Self::TestOperator)
                .unwrap_or(Self::Unknown(node)),
            "variable_name" => <VariableName as ::treesitter_types::FromNode>::from_node(node, src)
                .map(Self::VariableName)
                .unwrap_or(Self::Unknown(node)),
            _ => Self::Unknown(node),
        }
    }
}
impl ::treesitter_types::Spanned for AnyNode<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        match self {
            Self::Expression(inner) => inner.span(),
            Self::PrimaryExpression(inner) => inner.span(),
            Self::Statement(inner) => inner.span(),
            Self::ArithmeticExpansion(inner) => inner.span(),
            Self::Array(inner) => inner.span(),
            Self::BinaryExpression(inner) => inner.span(),
            Self::BraceExpression(inner) => inner.span(),
            Self::CStyleForStatement(inner) => inner.span(),
            Self::CaseItem(inner) => inner.span(),
            Self::CaseStatement(inner) => inner.span(),
            Self::Command(inner) => inner.span(),
            Self::CommandName(inner) => inner.span(),
            Self::CommandSubstitution(inner) => inner.span(),
            Self::CompoundStatement(inner) => inner.span(),
            Self::Concatenation(inner) => inner.span(),
            Self::DeclarationCommand(inner) => inner.span(),
            Self::DoGroup(inner) => inner.span(),
            Self::ElifClause(inner) => inner.span(),
            Self::ElseClause(inner) => inner.span(),
            Self::Expansion(inner) => inner.span(),
            Self::FileRedirect(inner) => inner.span(),
            Self::ForStatement(inner) => inner.span(),
            Self::FunctionDefinition(inner) => inner.span(),
            Self::HeredocBody(inner) => inner.span(),
            Self::HeredocRedirect(inner) => inner.span(),
            Self::HerestringRedirect(inner) => inner.span(),
            Self::IfStatement(inner) => inner.span(),
            Self::List(inner) => inner.span(),
            Self::NegatedCommand(inner) => inner.span(),
            Self::Number(inner) => inner.span(),
            Self::ParenthesizedExpression(inner) => inner.span(),
            Self::Pipeline(inner) => inner.span(),
            Self::PostfixExpression(inner) => inner.span(),
            Self::ProcessSubstitution(inner) => inner.span(),
            Self::Program(inner) => inner.span(),
            Self::RedirectedStatement(inner) => inner.span(),
            Self::SimpleExpansion(inner) => inner.span(),
            Self::String(inner) => inner.span(),
            Self::Subscript(inner) => inner.span(),
            Self::Subshell(inner) => inner.span(),
            Self::TernaryExpression(inner) => inner.span(),
            Self::TestCommand(inner) => inner.span(),
            Self::TranslatedString(inner) => inner.span(),
            Self::UnaryExpression(inner) => inner.span(),
            Self::UnsetCommand(inner) => inner.span(),
            Self::VariableAssignment(inner) => inner.span(),
            Self::VariableAssignments(inner) => inner.span(),
            Self::WhileStatement(inner) => inner.span(),
            Self::Word(inner) => inner.span(),
            Self::AnsiCString(inner) => inner.span(),
            Self::Comment(inner) => inner.span(),
            Self::ExtglobPattern(inner) => inner.span(),
            Self::FileDescriptor(inner) => inner.span(),
            Self::HeredocContent(inner) => inner.span(),
            Self::HeredocEnd(inner) => inner.span(),
            Self::HeredocStart(inner) => inner.span(),
            Self::RawString(inner) => inner.span(),
            Self::Regex(inner) => inner.span(),
            Self::SpecialVariableName(inner) => inner.span(),
            Self::StringContent(inner) => inner.span(),
            Self::TestOperator(inner) => inner.span(),
            Self::VariableName(inner) => inner.span(),
            Self::Unknown(node) => ::treesitter_types::Span::from(*node),
        }
    }
}
