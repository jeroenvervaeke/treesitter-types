#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Arg<'tree> {
    Primary(::std::boxed::Box<Primary<'tree>>),
    Assignment(::std::boxed::Box<Assignment<'tree>>),
    Binary(::std::boxed::Box<Binary<'tree>>),
    Conditional(::std::boxed::Box<Conditional<'tree>>),
    OperatorAssignment(::std::boxed::Box<OperatorAssignment<'tree>>),
    Range(::std::boxed::Box<Range<'tree>>),
    Unary(::std::boxed::Box<Unary<'tree>>),
}
impl<'tree> ::treesitter_types::FromNode<'tree> for Arg<'tree> {
    #[allow(clippy::collapsible_else_if)]
    fn from_node(
        node: ::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        match node.kind() {
            "assignment" => Ok(Self::Assignment(::std::boxed::Box::new(
                ::treesitter_types::maybe_grow_stack(|| {
                    <Assignment as ::treesitter_types::FromNode>::from_node(node, src)
                })?,
            ))),
            "binary" => Ok(Self::Binary(::std::boxed::Box::new(
                ::treesitter_types::maybe_grow_stack(|| {
                    <Binary as ::treesitter_types::FromNode>::from_node(node, src)
                })?,
            ))),
            "conditional" => Ok(Self::Conditional(::std::boxed::Box::new(
                ::treesitter_types::maybe_grow_stack(|| {
                    <Conditional as ::treesitter_types::FromNode>::from_node(node, src)
                })?,
            ))),
            "operator_assignment" => Ok(Self::OperatorAssignment(::std::boxed::Box::new(
                ::treesitter_types::maybe_grow_stack(|| {
                    <OperatorAssignment as ::treesitter_types::FromNode>::from_node(node, src)
                })?,
            ))),
            "range" => Ok(Self::Range(::std::boxed::Box::new(
                ::treesitter_types::maybe_grow_stack(|| {
                    <Range as ::treesitter_types::FromNode>::from_node(node, src)
                })?,
            ))),
            "unary" => Ok(Self::Unary(::std::boxed::Box::new(
                ::treesitter_types::maybe_grow_stack(|| {
                    <Unary as ::treesitter_types::FromNode>::from_node(node, src)
                })?,
            ))),
            _other => {
                if let Ok(v) = ::treesitter_types::maybe_grow_stack(|| {
                    <Primary as ::treesitter_types::FromNode>::from_node(node, src)
                }) {
                    Ok(Self::Primary(::std::boxed::Box::new(v)))
                } else {
                    Err(::treesitter_types::ParseError::unexpected_kind(
                        _other, node,
                    ))
                }
            }
        }
    }
}
impl ::treesitter_types::Spanned for Arg<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        match self {
            Self::Primary(inner) => inner.span(),
            Self::Assignment(inner) => inner.span(),
            Self::Binary(inner) => inner.span(),
            Self::Conditional(inner) => inner.span(),
            Self::OperatorAssignment(inner) => inner.span(),
            Self::Range(inner) => inner.span(),
            Self::Unary(inner) => inner.span(),
        }
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum CallOperator {
    AmpDot(::treesitter_types::Span),
    Dot(::treesitter_types::Span),
    DoubleColon(::treesitter_types::Span),
}
impl<'tree> ::treesitter_types::FromNode<'tree> for CallOperator {
    #[allow(clippy::collapsible_else_if)]
    fn from_node(
        node: ::tree_sitter::Node<'tree>,
        _src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        match node.kind() {
            "&." => Ok(Self::AmpDot(::treesitter_types::Span::from(node))),
            "." => Ok(Self::Dot(::treesitter_types::Span::from(node))),
            "::" => Ok(Self::DoubleColon(::treesitter_types::Span::from(node))),
            other => Err(::treesitter_types::ParseError::unexpected_kind(other, node)),
        }
    }
}
impl ::treesitter_types::Spanned for CallOperator {
    fn span(&self) -> ::treesitter_types::Span {
        match self {
            Self::AmpDot(span) => *span,
            Self::Dot(span) => *span,
            Self::DoubleColon(span) => *span,
        }
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Expression<'tree> {
    Arg(::std::boxed::Box<Arg<'tree>>),
    Assignment(::std::boxed::Box<Assignment<'tree>>),
    Binary(::std::boxed::Box<Binary<'tree>>),
    Break(::std::boxed::Box<Break<'tree>>),
    Call(::std::boxed::Box<Call<'tree>>),
    MatchPattern(::std::boxed::Box<MatchPattern<'tree>>),
    Next(::std::boxed::Box<Next<'tree>>),
    OperatorAssignment(::std::boxed::Box<OperatorAssignment<'tree>>),
    Return(::std::boxed::Box<Return<'tree>>),
    TestPattern(::std::boxed::Box<TestPattern<'tree>>),
    Unary(::std::boxed::Box<Unary<'tree>>),
    Yield(::std::boxed::Box<Yield<'tree>>),
}
impl<'tree> ::treesitter_types::FromNode<'tree> for Expression<'tree> {
    #[allow(clippy::collapsible_else_if)]
    fn from_node(
        node: ::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        match node.kind() {
            "assignment" => Ok(Self::Assignment(::std::boxed::Box::new(
                ::treesitter_types::maybe_grow_stack(|| {
                    <Assignment as ::treesitter_types::FromNode>::from_node(node, src)
                })?,
            ))),
            "binary" => Ok(Self::Binary(::std::boxed::Box::new(
                ::treesitter_types::maybe_grow_stack(|| {
                    <Binary as ::treesitter_types::FromNode>::from_node(node, src)
                })?,
            ))),
            "break" => Ok(Self::Break(::std::boxed::Box::new(
                ::treesitter_types::maybe_grow_stack(|| {
                    <Break as ::treesitter_types::FromNode>::from_node(node, src)
                })?,
            ))),
            "call" => Ok(Self::Call(::std::boxed::Box::new(
                ::treesitter_types::maybe_grow_stack(|| {
                    <Call as ::treesitter_types::FromNode>::from_node(node, src)
                })?,
            ))),
            "match_pattern" => Ok(Self::MatchPattern(::std::boxed::Box::new(
                ::treesitter_types::maybe_grow_stack(|| {
                    <MatchPattern as ::treesitter_types::FromNode>::from_node(node, src)
                })?,
            ))),
            "next" => Ok(Self::Next(::std::boxed::Box::new(
                ::treesitter_types::maybe_grow_stack(|| {
                    <Next as ::treesitter_types::FromNode>::from_node(node, src)
                })?,
            ))),
            "operator_assignment" => Ok(Self::OperatorAssignment(::std::boxed::Box::new(
                ::treesitter_types::maybe_grow_stack(|| {
                    <OperatorAssignment as ::treesitter_types::FromNode>::from_node(node, src)
                })?,
            ))),
            "return" => Ok(Self::Return(::std::boxed::Box::new(
                ::treesitter_types::maybe_grow_stack(|| {
                    <Return as ::treesitter_types::FromNode>::from_node(node, src)
                })?,
            ))),
            "test_pattern" => Ok(Self::TestPattern(::std::boxed::Box::new(
                ::treesitter_types::maybe_grow_stack(|| {
                    <TestPattern as ::treesitter_types::FromNode>::from_node(node, src)
                })?,
            ))),
            "unary" => Ok(Self::Unary(::std::boxed::Box::new(
                ::treesitter_types::maybe_grow_stack(|| {
                    <Unary as ::treesitter_types::FromNode>::from_node(node, src)
                })?,
            ))),
            "yield" => Ok(Self::Yield(::std::boxed::Box::new(
                ::treesitter_types::maybe_grow_stack(|| {
                    <Yield as ::treesitter_types::FromNode>::from_node(node, src)
                })?,
            ))),
            _other => {
                if let Ok(v) = ::treesitter_types::maybe_grow_stack(|| {
                    <Arg as ::treesitter_types::FromNode>::from_node(node, src)
                }) {
                    Ok(Self::Arg(::std::boxed::Box::new(v)))
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
            Self::Arg(inner) => inner.span(),
            Self::Assignment(inner) => inner.span(),
            Self::Binary(inner) => inner.span(),
            Self::Break(inner) => inner.span(),
            Self::Call(inner) => inner.span(),
            Self::MatchPattern(inner) => inner.span(),
            Self::Next(inner) => inner.span(),
            Self::OperatorAssignment(inner) => inner.span(),
            Self::Return(inner) => inner.span(),
            Self::TestPattern(inner) => inner.span(),
            Self::Unary(inner) => inner.span(),
            Self::Yield(inner) => inner.span(),
        }
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Lhs<'tree> {
    Variable(::std::boxed::Box<Variable<'tree>>),
    Call(::std::boxed::Box<Call<'tree>>),
    ElementReference(::std::boxed::Box<ElementReference<'tree>>),
    False(::std::boxed::Box<False<'tree>>),
    Nil(::std::boxed::Box<Nil<'tree>>),
    ScopeResolution(::std::boxed::Box<ScopeResolution<'tree>>),
    True(::std::boxed::Box<True<'tree>>),
}
impl<'tree> ::treesitter_types::FromNode<'tree> for Lhs<'tree> {
    #[allow(clippy::collapsible_else_if)]
    fn from_node(
        node: ::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        match node.kind() {
            "call" => Ok(Self::Call(::std::boxed::Box::new(
                ::treesitter_types::maybe_grow_stack(|| {
                    <Call as ::treesitter_types::FromNode>::from_node(node, src)
                })?,
            ))),
            "element_reference" => Ok(Self::ElementReference(::std::boxed::Box::new(
                ::treesitter_types::maybe_grow_stack(|| {
                    <ElementReference as ::treesitter_types::FromNode>::from_node(node, src)
                })?,
            ))),
            "false" => Ok(Self::False(::std::boxed::Box::new(
                ::treesitter_types::maybe_grow_stack(|| {
                    <False as ::treesitter_types::FromNode>::from_node(node, src)
                })?,
            ))),
            "nil" => Ok(Self::Nil(::std::boxed::Box::new(
                ::treesitter_types::maybe_grow_stack(|| {
                    <Nil as ::treesitter_types::FromNode>::from_node(node, src)
                })?,
            ))),
            "scope_resolution" => Ok(Self::ScopeResolution(::std::boxed::Box::new(
                ::treesitter_types::maybe_grow_stack(|| {
                    <ScopeResolution as ::treesitter_types::FromNode>::from_node(node, src)
                })?,
            ))),
            "true" => Ok(Self::True(::std::boxed::Box::new(
                ::treesitter_types::maybe_grow_stack(|| {
                    <True as ::treesitter_types::FromNode>::from_node(node, src)
                })?,
            ))),
            _other => {
                if let Ok(v) = ::treesitter_types::maybe_grow_stack(|| {
                    <Variable as ::treesitter_types::FromNode>::from_node(node, src)
                }) {
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
impl ::treesitter_types::Spanned for Lhs<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        match self {
            Self::Variable(inner) => inner.span(),
            Self::Call(inner) => inner.span(),
            Self::ElementReference(inner) => inner.span(),
            Self::False(inner) => inner.span(),
            Self::Nil(inner) => inner.span(),
            Self::ScopeResolution(inner) => inner.span(),
            Self::True(inner) => inner.span(),
        }
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum MethodName<'tree> {
    NonlocalVariable(::std::boxed::Box<NonlocalVariable<'tree>>),
    Constant(::std::boxed::Box<Constant<'tree>>),
    DelimitedSymbol(::std::boxed::Box<DelimitedSymbol<'tree>>),
    Identifier(::std::boxed::Box<Identifier<'tree>>),
    Operator(::std::boxed::Box<Operator<'tree>>),
    Setter(::std::boxed::Box<Setter<'tree>>),
    SimpleSymbol(::std::boxed::Box<SimpleSymbol<'tree>>),
}
impl<'tree> ::treesitter_types::FromNode<'tree> for MethodName<'tree> {
    #[allow(clippy::collapsible_else_if)]
    fn from_node(
        node: ::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        match node.kind() {
            "constant" => Ok(Self::Constant(::std::boxed::Box::new(
                ::treesitter_types::maybe_grow_stack(|| {
                    <Constant as ::treesitter_types::FromNode>::from_node(node, src)
                })?,
            ))),
            "delimited_symbol" => Ok(Self::DelimitedSymbol(::std::boxed::Box::new(
                ::treesitter_types::maybe_grow_stack(|| {
                    <DelimitedSymbol as ::treesitter_types::FromNode>::from_node(node, src)
                })?,
            ))),
            "identifier" => Ok(Self::Identifier(::std::boxed::Box::new(
                ::treesitter_types::maybe_grow_stack(|| {
                    <Identifier as ::treesitter_types::FromNode>::from_node(node, src)
                })?,
            ))),
            "operator" => Ok(Self::Operator(::std::boxed::Box::new(
                ::treesitter_types::maybe_grow_stack(|| {
                    <Operator as ::treesitter_types::FromNode>::from_node(node, src)
                })?,
            ))),
            "setter" => Ok(Self::Setter(::std::boxed::Box::new(
                ::treesitter_types::maybe_grow_stack(|| {
                    <Setter as ::treesitter_types::FromNode>::from_node(node, src)
                })?,
            ))),
            "simple_symbol" => Ok(Self::SimpleSymbol(::std::boxed::Box::new(
                ::treesitter_types::maybe_grow_stack(|| {
                    <SimpleSymbol as ::treesitter_types::FromNode>::from_node(node, src)
                })?,
            ))),
            _other => {
                if let Ok(v) = ::treesitter_types::maybe_grow_stack(|| {
                    <NonlocalVariable as ::treesitter_types::FromNode>::from_node(node, src)
                }) {
                    Ok(Self::NonlocalVariable(::std::boxed::Box::new(v)))
                } else {
                    Err(::treesitter_types::ParseError::unexpected_kind(
                        _other, node,
                    ))
                }
            }
        }
    }
}
impl ::treesitter_types::Spanned for MethodName<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        match self {
            Self::NonlocalVariable(inner) => inner.span(),
            Self::Constant(inner) => inner.span(),
            Self::DelimitedSymbol(inner) => inner.span(),
            Self::Identifier(inner) => inner.span(),
            Self::Operator(inner) => inner.span(),
            Self::Setter(inner) => inner.span(),
            Self::SimpleSymbol(inner) => inner.span(),
        }
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum NonlocalVariable<'tree> {
    ClassVariable(::std::boxed::Box<ClassVariable<'tree>>),
    GlobalVariable(::std::boxed::Box<GlobalVariable<'tree>>),
    InstanceVariable(::std::boxed::Box<InstanceVariable<'tree>>),
}
impl<'tree> ::treesitter_types::FromNode<'tree> for NonlocalVariable<'tree> {
    #[allow(clippy::collapsible_else_if)]
    fn from_node(
        node: ::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        match node.kind() {
            "class_variable" => Ok(Self::ClassVariable(::std::boxed::Box::new(
                ::treesitter_types::maybe_grow_stack(|| {
                    <ClassVariable as ::treesitter_types::FromNode>::from_node(node, src)
                })?,
            ))),
            "global_variable" => Ok(Self::GlobalVariable(::std::boxed::Box::new(
                ::treesitter_types::maybe_grow_stack(|| {
                    <GlobalVariable as ::treesitter_types::FromNode>::from_node(node, src)
                })?,
            ))),
            "instance_variable" => Ok(Self::InstanceVariable(::std::boxed::Box::new(
                ::treesitter_types::maybe_grow_stack(|| {
                    <InstanceVariable as ::treesitter_types::FromNode>::from_node(node, src)
                })?,
            ))),
            other => Err(::treesitter_types::ParseError::unexpected_kind(other, node)),
        }
    }
}
impl ::treesitter_types::Spanned for NonlocalVariable<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        match self {
            Self::ClassVariable(inner) => inner.span(),
            Self::GlobalVariable(inner) => inner.span(),
            Self::InstanceVariable(inner) => inner.span(),
        }
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum PatternConstant<'tree> {
    Constant(::std::boxed::Box<Constant<'tree>>),
    ScopeResolution(::std::boxed::Box<ScopeResolution<'tree>>),
}
impl<'tree> ::treesitter_types::FromNode<'tree> for PatternConstant<'tree> {
    #[allow(clippy::collapsible_else_if)]
    fn from_node(
        node: ::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        match node.kind() {
            "constant" => Ok(Self::Constant(::std::boxed::Box::new(
                ::treesitter_types::maybe_grow_stack(|| {
                    <Constant as ::treesitter_types::FromNode>::from_node(node, src)
                })?,
            ))),
            "scope_resolution" => Ok(Self::ScopeResolution(::std::boxed::Box::new(
                ::treesitter_types::maybe_grow_stack(|| {
                    <ScopeResolution as ::treesitter_types::FromNode>::from_node(node, src)
                })?,
            ))),
            other => Err(::treesitter_types::ParseError::unexpected_kind(other, node)),
        }
    }
}
impl ::treesitter_types::Spanned for PatternConstant<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        match self {
            Self::Constant(inner) => inner.span(),
            Self::ScopeResolution(inner) => inner.span(),
        }
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum PatternExpr<'tree> {
    PatternExprBasic(::std::boxed::Box<PatternExprBasic<'tree>>),
    AlternativePattern(::std::boxed::Box<AlternativePattern<'tree>>),
    AsPattern(::std::boxed::Box<AsPattern<'tree>>),
}
impl<'tree> ::treesitter_types::FromNode<'tree> for PatternExpr<'tree> {
    #[allow(clippy::collapsible_else_if)]
    fn from_node(
        node: ::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        match node.kind() {
            "alternative_pattern" => Ok(Self::AlternativePattern(::std::boxed::Box::new(
                ::treesitter_types::maybe_grow_stack(|| {
                    <AlternativePattern as ::treesitter_types::FromNode>::from_node(node, src)
                })?,
            ))),
            "as_pattern" => Ok(Self::AsPattern(::std::boxed::Box::new(
                ::treesitter_types::maybe_grow_stack(|| {
                    <AsPattern as ::treesitter_types::FromNode>::from_node(node, src)
                })?,
            ))),
            _other => {
                if let Ok(v) = ::treesitter_types::maybe_grow_stack(|| {
                    <PatternExprBasic as ::treesitter_types::FromNode>::from_node(node, src)
                }) {
                    Ok(Self::PatternExprBasic(::std::boxed::Box::new(v)))
                } else {
                    Err(::treesitter_types::ParseError::unexpected_kind(
                        _other, node,
                    ))
                }
            }
        }
    }
}
impl ::treesitter_types::Spanned for PatternExpr<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        match self {
            Self::PatternExprBasic(inner) => inner.span(),
            Self::AlternativePattern(inner) => inner.span(),
            Self::AsPattern(inner) => inner.span(),
        }
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum PatternExprBasic<'tree> {
    PatternConstant(::std::boxed::Box<PatternConstant<'tree>>),
    PatternPrimitive(::std::boxed::Box<PatternPrimitive<'tree>>),
    ArrayPattern(::std::boxed::Box<ArrayPattern<'tree>>),
    ExpressionReferencePattern(::std::boxed::Box<ExpressionReferencePattern<'tree>>),
    FindPattern(::std::boxed::Box<FindPattern<'tree>>),
    HashPattern(::std::boxed::Box<HashPattern<'tree>>),
    Identifier(::std::boxed::Box<Identifier<'tree>>),
    ParenthesizedPattern(::std::boxed::Box<ParenthesizedPattern<'tree>>),
    Range(::std::boxed::Box<Range<'tree>>),
    VariableReferencePattern(::std::boxed::Box<VariableReferencePattern<'tree>>),
}
impl<'tree> ::treesitter_types::FromNode<'tree> for PatternExprBasic<'tree> {
    #[allow(clippy::collapsible_else_if)]
    fn from_node(
        node: ::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        match node.kind() {
            "array_pattern" => Ok(Self::ArrayPattern(::std::boxed::Box::new(
                ::treesitter_types::maybe_grow_stack(|| {
                    <ArrayPattern as ::treesitter_types::FromNode>::from_node(node, src)
                })?,
            ))),
            "expression_reference_pattern" => Ok(Self::ExpressionReferencePattern(
                ::std::boxed::Box::new(::treesitter_types::maybe_grow_stack(|| {
                    <ExpressionReferencePattern as ::treesitter_types::FromNode>::from_node(
                        node, src,
                    )
                })?),
            )),
            "find_pattern" => Ok(Self::FindPattern(::std::boxed::Box::new(
                ::treesitter_types::maybe_grow_stack(|| {
                    <FindPattern as ::treesitter_types::FromNode>::from_node(node, src)
                })?,
            ))),
            "hash_pattern" => Ok(Self::HashPattern(::std::boxed::Box::new(
                ::treesitter_types::maybe_grow_stack(|| {
                    <HashPattern as ::treesitter_types::FromNode>::from_node(node, src)
                })?,
            ))),
            "identifier" => Ok(Self::Identifier(::std::boxed::Box::new(
                ::treesitter_types::maybe_grow_stack(|| {
                    <Identifier as ::treesitter_types::FromNode>::from_node(node, src)
                })?,
            ))),
            "parenthesized_pattern" => Ok(Self::ParenthesizedPattern(::std::boxed::Box::new(
                ::treesitter_types::maybe_grow_stack(|| {
                    <ParenthesizedPattern as ::treesitter_types::FromNode>::from_node(node, src)
                })?,
            ))),
            "range" => Ok(Self::Range(::std::boxed::Box::new(
                ::treesitter_types::maybe_grow_stack(|| {
                    <Range as ::treesitter_types::FromNode>::from_node(node, src)
                })?,
            ))),
            "variable_reference_pattern" => Ok(Self::VariableReferencePattern(
                ::std::boxed::Box::new(::treesitter_types::maybe_grow_stack(|| {
                    <VariableReferencePattern as ::treesitter_types::FromNode>::from_node(node, src)
                })?),
            )),
            _other => {
                if let Ok(v) = ::treesitter_types::maybe_grow_stack(|| {
                    <PatternConstant as ::treesitter_types::FromNode>::from_node(node, src)
                }) {
                    Ok(Self::PatternConstant(::std::boxed::Box::new(v)))
                } else {
                    if let Ok(v) = ::treesitter_types::maybe_grow_stack(|| {
                        <PatternPrimitive as ::treesitter_types::FromNode>::from_node(node, src)
                    }) {
                        Ok(Self::PatternPrimitive(::std::boxed::Box::new(v)))
                    } else {
                        Err(::treesitter_types::ParseError::unexpected_kind(
                            _other, node,
                        ))
                    }
                }
            }
        }
    }
}
impl ::treesitter_types::Spanned for PatternExprBasic<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        match self {
            Self::PatternConstant(inner) => inner.span(),
            Self::PatternPrimitive(inner) => inner.span(),
            Self::ArrayPattern(inner) => inner.span(),
            Self::ExpressionReferencePattern(inner) => inner.span(),
            Self::FindPattern(inner) => inner.span(),
            Self::HashPattern(inner) => inner.span(),
            Self::Identifier(inner) => inner.span(),
            Self::ParenthesizedPattern(inner) => inner.span(),
            Self::Range(inner) => inner.span(),
            Self::VariableReferencePattern(inner) => inner.span(),
        }
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum PatternPrimitive<'tree> {
    SimpleNumeric(::std::boxed::Box<SimpleNumeric<'tree>>),
    DelimitedSymbol(::std::boxed::Box<DelimitedSymbol<'tree>>),
    Encoding(::std::boxed::Box<Encoding<'tree>>),
    False(::std::boxed::Box<False<'tree>>),
    File(::std::boxed::Box<File<'tree>>),
    HeredocBeginning(::std::boxed::Box<HeredocBeginning<'tree>>),
    Lambda(::std::boxed::Box<Lambda<'tree>>),
    Line(::std::boxed::Box<Line<'tree>>),
    Nil(::std::boxed::Box<Nil<'tree>>),
    Regex(::std::boxed::Box<Regex<'tree>>),
    SelfType(::std::boxed::Box<SelfType<'tree>>),
    SimpleSymbol(::std::boxed::Box<SimpleSymbol<'tree>>),
    String(::std::boxed::Box<String<'tree>>),
    StringArray(::std::boxed::Box<StringArray<'tree>>),
    Subshell(::std::boxed::Box<Subshell<'tree>>),
    SymbolArray(::std::boxed::Box<SymbolArray<'tree>>),
    True(::std::boxed::Box<True<'tree>>),
    Unary(::std::boxed::Box<Unary<'tree>>),
}
impl<'tree> ::treesitter_types::FromNode<'tree> for PatternPrimitive<'tree> {
    #[allow(clippy::collapsible_else_if)]
    fn from_node(
        node: ::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        match node.kind() {
            "delimited_symbol" => Ok(Self::DelimitedSymbol(::std::boxed::Box::new(
                ::treesitter_types::maybe_grow_stack(|| {
                    <DelimitedSymbol as ::treesitter_types::FromNode>::from_node(node, src)
                })?,
            ))),
            "encoding" => Ok(Self::Encoding(::std::boxed::Box::new(
                ::treesitter_types::maybe_grow_stack(|| {
                    <Encoding as ::treesitter_types::FromNode>::from_node(node, src)
                })?,
            ))),
            "false" => Ok(Self::False(::std::boxed::Box::new(
                ::treesitter_types::maybe_grow_stack(|| {
                    <False as ::treesitter_types::FromNode>::from_node(node, src)
                })?,
            ))),
            "file" => Ok(Self::File(::std::boxed::Box::new(
                ::treesitter_types::maybe_grow_stack(|| {
                    <File as ::treesitter_types::FromNode>::from_node(node, src)
                })?,
            ))),
            "heredoc_beginning" => Ok(Self::HeredocBeginning(::std::boxed::Box::new(
                ::treesitter_types::maybe_grow_stack(|| {
                    <HeredocBeginning as ::treesitter_types::FromNode>::from_node(node, src)
                })?,
            ))),
            "lambda" => Ok(Self::Lambda(::std::boxed::Box::new(
                ::treesitter_types::maybe_grow_stack(|| {
                    <Lambda as ::treesitter_types::FromNode>::from_node(node, src)
                })?,
            ))),
            "line" => Ok(Self::Line(::std::boxed::Box::new(
                ::treesitter_types::maybe_grow_stack(|| {
                    <Line as ::treesitter_types::FromNode>::from_node(node, src)
                })?,
            ))),
            "nil" => Ok(Self::Nil(::std::boxed::Box::new(
                ::treesitter_types::maybe_grow_stack(|| {
                    <Nil as ::treesitter_types::FromNode>::from_node(node, src)
                })?,
            ))),
            "regex" => Ok(Self::Regex(::std::boxed::Box::new(
                ::treesitter_types::maybe_grow_stack(|| {
                    <Regex as ::treesitter_types::FromNode>::from_node(node, src)
                })?,
            ))),
            "self" => Ok(Self::SelfType(::std::boxed::Box::new(
                ::treesitter_types::maybe_grow_stack(|| {
                    <SelfType as ::treesitter_types::FromNode>::from_node(node, src)
                })?,
            ))),
            "simple_symbol" => Ok(Self::SimpleSymbol(::std::boxed::Box::new(
                ::treesitter_types::maybe_grow_stack(|| {
                    <SimpleSymbol as ::treesitter_types::FromNode>::from_node(node, src)
                })?,
            ))),
            "string" => Ok(Self::String(::std::boxed::Box::new(
                ::treesitter_types::maybe_grow_stack(|| {
                    <String as ::treesitter_types::FromNode>::from_node(node, src)
                })?,
            ))),
            "string_array" => Ok(Self::StringArray(::std::boxed::Box::new(
                ::treesitter_types::maybe_grow_stack(|| {
                    <StringArray as ::treesitter_types::FromNode>::from_node(node, src)
                })?,
            ))),
            "subshell" => Ok(Self::Subshell(::std::boxed::Box::new(
                ::treesitter_types::maybe_grow_stack(|| {
                    <Subshell as ::treesitter_types::FromNode>::from_node(node, src)
                })?,
            ))),
            "symbol_array" => Ok(Self::SymbolArray(::std::boxed::Box::new(
                ::treesitter_types::maybe_grow_stack(|| {
                    <SymbolArray as ::treesitter_types::FromNode>::from_node(node, src)
                })?,
            ))),
            "true" => Ok(Self::True(::std::boxed::Box::new(
                ::treesitter_types::maybe_grow_stack(|| {
                    <True as ::treesitter_types::FromNode>::from_node(node, src)
                })?,
            ))),
            "unary" => Ok(Self::Unary(::std::boxed::Box::new(
                ::treesitter_types::maybe_grow_stack(|| {
                    <Unary as ::treesitter_types::FromNode>::from_node(node, src)
                })?,
            ))),
            _other => {
                if let Ok(v) = ::treesitter_types::maybe_grow_stack(|| {
                    <SimpleNumeric as ::treesitter_types::FromNode>::from_node(node, src)
                }) {
                    Ok(Self::SimpleNumeric(::std::boxed::Box::new(v)))
                } else {
                    Err(::treesitter_types::ParseError::unexpected_kind(
                        _other, node,
                    ))
                }
            }
        }
    }
}
impl ::treesitter_types::Spanned for PatternPrimitive<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        match self {
            Self::SimpleNumeric(inner) => inner.span(),
            Self::DelimitedSymbol(inner) => inner.span(),
            Self::Encoding(inner) => inner.span(),
            Self::False(inner) => inner.span(),
            Self::File(inner) => inner.span(),
            Self::HeredocBeginning(inner) => inner.span(),
            Self::Lambda(inner) => inner.span(),
            Self::Line(inner) => inner.span(),
            Self::Nil(inner) => inner.span(),
            Self::Regex(inner) => inner.span(),
            Self::SelfType(inner) => inner.span(),
            Self::SimpleSymbol(inner) => inner.span(),
            Self::String(inner) => inner.span(),
            Self::StringArray(inner) => inner.span(),
            Self::Subshell(inner) => inner.span(),
            Self::SymbolArray(inner) => inner.span(),
            Self::True(inner) => inner.span(),
            Self::Unary(inner) => inner.span(),
        }
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum PatternTopExprBody<'tree> {
    PatternExpr(::std::boxed::Box<PatternExpr<'tree>>),
    ArrayPattern(::std::boxed::Box<ArrayPattern<'tree>>),
    FindPattern(::std::boxed::Box<FindPattern<'tree>>),
    HashPattern(::std::boxed::Box<HashPattern<'tree>>),
}
impl<'tree> ::treesitter_types::FromNode<'tree> for PatternTopExprBody<'tree> {
    #[allow(clippy::collapsible_else_if)]
    fn from_node(
        node: ::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        match node.kind() {
            "array_pattern" => Ok(Self::ArrayPattern(::std::boxed::Box::new(
                ::treesitter_types::maybe_grow_stack(|| {
                    <ArrayPattern as ::treesitter_types::FromNode>::from_node(node, src)
                })?,
            ))),
            "find_pattern" => Ok(Self::FindPattern(::std::boxed::Box::new(
                ::treesitter_types::maybe_grow_stack(|| {
                    <FindPattern as ::treesitter_types::FromNode>::from_node(node, src)
                })?,
            ))),
            "hash_pattern" => Ok(Self::HashPattern(::std::boxed::Box::new(
                ::treesitter_types::maybe_grow_stack(|| {
                    <HashPattern as ::treesitter_types::FromNode>::from_node(node, src)
                })?,
            ))),
            _other => {
                if let Ok(v) = ::treesitter_types::maybe_grow_stack(|| {
                    <PatternExpr as ::treesitter_types::FromNode>::from_node(node, src)
                }) {
                    Ok(Self::PatternExpr(::std::boxed::Box::new(v)))
                } else {
                    Err(::treesitter_types::ParseError::unexpected_kind(
                        _other, node,
                    ))
                }
            }
        }
    }
}
impl ::treesitter_types::Spanned for PatternTopExprBody<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        match self {
            Self::PatternExpr(inner) => inner.span(),
            Self::ArrayPattern(inner) => inner.span(),
            Self::FindPattern(inner) => inner.span(),
            Self::HashPattern(inner) => inner.span(),
        }
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Primary<'tree> {
    Lhs(::std::boxed::Box<Lhs<'tree>>),
    SimpleNumeric(::std::boxed::Box<SimpleNumeric<'tree>>),
    Array(::std::boxed::Box<Array<'tree>>),
    Begin(::std::boxed::Box<Begin<'tree>>),
    Break(::std::boxed::Box<Break<'tree>>),
    Call(::std::boxed::Box<Call<'tree>>),
    Case(::std::boxed::Box<Case<'tree>>),
    CaseMatch(::std::boxed::Box<CaseMatch<'tree>>),
    ChainedString(::std::boxed::Box<ChainedString<'tree>>),
    Character(::std::boxed::Box<Character<'tree>>),
    Class(::std::boxed::Box<Class<'tree>>),
    DelimitedSymbol(::std::boxed::Box<DelimitedSymbol<'tree>>),
    For(::std::boxed::Box<For<'tree>>),
    Hash(::std::boxed::Box<Hash<'tree>>),
    HeredocBeginning(::std::boxed::Box<HeredocBeginning<'tree>>),
    If(::std::boxed::Box<If<'tree>>),
    Lambda(::std::boxed::Box<Lambda<'tree>>),
    Method(::std::boxed::Box<Method<'tree>>),
    Module(::std::boxed::Box<Module<'tree>>),
    Next(::std::boxed::Box<Next<'tree>>),
    ParenthesizedStatements(::std::boxed::Box<ParenthesizedStatements<'tree>>),
    Redo(::std::boxed::Box<Redo<'tree>>),
    Regex(::std::boxed::Box<Regex<'tree>>),
    Retry(::std::boxed::Box<Retry<'tree>>),
    Return(::std::boxed::Box<Return<'tree>>),
    SimpleSymbol(::std::boxed::Box<SimpleSymbol<'tree>>),
    SingletonClass(::std::boxed::Box<SingletonClass<'tree>>),
    SingletonMethod(::std::boxed::Box<SingletonMethod<'tree>>),
    String(::std::boxed::Box<String<'tree>>),
    StringArray(::std::boxed::Box<StringArray<'tree>>),
    Subshell(::std::boxed::Box<Subshell<'tree>>),
    SymbolArray(::std::boxed::Box<SymbolArray<'tree>>),
    Unary(::std::boxed::Box<Unary<'tree>>),
    Unless(::std::boxed::Box<Unless<'tree>>),
    Until(::std::boxed::Box<Until<'tree>>),
    While(::std::boxed::Box<While<'tree>>),
    Yield(::std::boxed::Box<Yield<'tree>>),
}
impl<'tree> ::treesitter_types::FromNode<'tree> for Primary<'tree> {
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
            "begin" => Ok(Self::Begin(::std::boxed::Box::new(
                ::treesitter_types::maybe_grow_stack(|| {
                    <Begin as ::treesitter_types::FromNode>::from_node(node, src)
                })?,
            ))),
            "break" => Ok(Self::Break(::std::boxed::Box::new(
                ::treesitter_types::maybe_grow_stack(|| {
                    <Break as ::treesitter_types::FromNode>::from_node(node, src)
                })?,
            ))),
            "call" => Ok(Self::Call(::std::boxed::Box::new(
                ::treesitter_types::maybe_grow_stack(|| {
                    <Call as ::treesitter_types::FromNode>::from_node(node, src)
                })?,
            ))),
            "case" => Ok(Self::Case(::std::boxed::Box::new(
                ::treesitter_types::maybe_grow_stack(|| {
                    <Case as ::treesitter_types::FromNode>::from_node(node, src)
                })?,
            ))),
            "case_match" => Ok(Self::CaseMatch(::std::boxed::Box::new(
                ::treesitter_types::maybe_grow_stack(|| {
                    <CaseMatch as ::treesitter_types::FromNode>::from_node(node, src)
                })?,
            ))),
            "chained_string" => Ok(Self::ChainedString(::std::boxed::Box::new(
                ::treesitter_types::maybe_grow_stack(|| {
                    <ChainedString as ::treesitter_types::FromNode>::from_node(node, src)
                })?,
            ))),
            "character" => Ok(Self::Character(::std::boxed::Box::new(
                ::treesitter_types::maybe_grow_stack(|| {
                    <Character as ::treesitter_types::FromNode>::from_node(node, src)
                })?,
            ))),
            "class" => Ok(Self::Class(::std::boxed::Box::new(
                ::treesitter_types::maybe_grow_stack(|| {
                    <Class as ::treesitter_types::FromNode>::from_node(node, src)
                })?,
            ))),
            "delimited_symbol" => Ok(Self::DelimitedSymbol(::std::boxed::Box::new(
                ::treesitter_types::maybe_grow_stack(|| {
                    <DelimitedSymbol as ::treesitter_types::FromNode>::from_node(node, src)
                })?,
            ))),
            "for" => Ok(Self::For(::std::boxed::Box::new(
                ::treesitter_types::maybe_grow_stack(|| {
                    <For as ::treesitter_types::FromNode>::from_node(node, src)
                })?,
            ))),
            "hash" => Ok(Self::Hash(::std::boxed::Box::new(
                ::treesitter_types::maybe_grow_stack(|| {
                    <Hash as ::treesitter_types::FromNode>::from_node(node, src)
                })?,
            ))),
            "heredoc_beginning" => Ok(Self::HeredocBeginning(::std::boxed::Box::new(
                ::treesitter_types::maybe_grow_stack(|| {
                    <HeredocBeginning as ::treesitter_types::FromNode>::from_node(node, src)
                })?,
            ))),
            "if" => Ok(Self::If(::std::boxed::Box::new(
                ::treesitter_types::maybe_grow_stack(|| {
                    <If as ::treesitter_types::FromNode>::from_node(node, src)
                })?,
            ))),
            "lambda" => Ok(Self::Lambda(::std::boxed::Box::new(
                ::treesitter_types::maybe_grow_stack(|| {
                    <Lambda as ::treesitter_types::FromNode>::from_node(node, src)
                })?,
            ))),
            "method" => Ok(Self::Method(::std::boxed::Box::new(
                ::treesitter_types::maybe_grow_stack(|| {
                    <Method as ::treesitter_types::FromNode>::from_node(node, src)
                })?,
            ))),
            "module" => Ok(Self::Module(::std::boxed::Box::new(
                ::treesitter_types::maybe_grow_stack(|| {
                    <Module as ::treesitter_types::FromNode>::from_node(node, src)
                })?,
            ))),
            "next" => Ok(Self::Next(::std::boxed::Box::new(
                ::treesitter_types::maybe_grow_stack(|| {
                    <Next as ::treesitter_types::FromNode>::from_node(node, src)
                })?,
            ))),
            "parenthesized_statements" => Ok(Self::ParenthesizedStatements(
                ::std::boxed::Box::new(::treesitter_types::maybe_grow_stack(|| {
                    <ParenthesizedStatements as ::treesitter_types::FromNode>::from_node(node, src)
                })?),
            )),
            "redo" => Ok(Self::Redo(::std::boxed::Box::new(
                ::treesitter_types::maybe_grow_stack(|| {
                    <Redo as ::treesitter_types::FromNode>::from_node(node, src)
                })?,
            ))),
            "regex" => Ok(Self::Regex(::std::boxed::Box::new(
                ::treesitter_types::maybe_grow_stack(|| {
                    <Regex as ::treesitter_types::FromNode>::from_node(node, src)
                })?,
            ))),
            "retry" => Ok(Self::Retry(::std::boxed::Box::new(
                ::treesitter_types::maybe_grow_stack(|| {
                    <Retry as ::treesitter_types::FromNode>::from_node(node, src)
                })?,
            ))),
            "return" => Ok(Self::Return(::std::boxed::Box::new(
                ::treesitter_types::maybe_grow_stack(|| {
                    <Return as ::treesitter_types::FromNode>::from_node(node, src)
                })?,
            ))),
            "simple_symbol" => Ok(Self::SimpleSymbol(::std::boxed::Box::new(
                ::treesitter_types::maybe_grow_stack(|| {
                    <SimpleSymbol as ::treesitter_types::FromNode>::from_node(node, src)
                })?,
            ))),
            "singleton_class" => Ok(Self::SingletonClass(::std::boxed::Box::new(
                ::treesitter_types::maybe_grow_stack(|| {
                    <SingletonClass as ::treesitter_types::FromNode>::from_node(node, src)
                })?,
            ))),
            "singleton_method" => Ok(Self::SingletonMethod(::std::boxed::Box::new(
                ::treesitter_types::maybe_grow_stack(|| {
                    <SingletonMethod as ::treesitter_types::FromNode>::from_node(node, src)
                })?,
            ))),
            "string" => Ok(Self::String(::std::boxed::Box::new(
                ::treesitter_types::maybe_grow_stack(|| {
                    <String as ::treesitter_types::FromNode>::from_node(node, src)
                })?,
            ))),
            "string_array" => Ok(Self::StringArray(::std::boxed::Box::new(
                ::treesitter_types::maybe_grow_stack(|| {
                    <StringArray as ::treesitter_types::FromNode>::from_node(node, src)
                })?,
            ))),
            "subshell" => Ok(Self::Subshell(::std::boxed::Box::new(
                ::treesitter_types::maybe_grow_stack(|| {
                    <Subshell as ::treesitter_types::FromNode>::from_node(node, src)
                })?,
            ))),
            "symbol_array" => Ok(Self::SymbolArray(::std::boxed::Box::new(
                ::treesitter_types::maybe_grow_stack(|| {
                    <SymbolArray as ::treesitter_types::FromNode>::from_node(node, src)
                })?,
            ))),
            "unary" => Ok(Self::Unary(::std::boxed::Box::new(
                ::treesitter_types::maybe_grow_stack(|| {
                    <Unary as ::treesitter_types::FromNode>::from_node(node, src)
                })?,
            ))),
            "unless" => Ok(Self::Unless(::std::boxed::Box::new(
                ::treesitter_types::maybe_grow_stack(|| {
                    <Unless as ::treesitter_types::FromNode>::from_node(node, src)
                })?,
            ))),
            "until" => Ok(Self::Until(::std::boxed::Box::new(
                ::treesitter_types::maybe_grow_stack(|| {
                    <Until as ::treesitter_types::FromNode>::from_node(node, src)
                })?,
            ))),
            "while" => Ok(Self::While(::std::boxed::Box::new(
                ::treesitter_types::maybe_grow_stack(|| {
                    <While as ::treesitter_types::FromNode>::from_node(node, src)
                })?,
            ))),
            "yield" => Ok(Self::Yield(::std::boxed::Box::new(
                ::treesitter_types::maybe_grow_stack(|| {
                    <Yield as ::treesitter_types::FromNode>::from_node(node, src)
                })?,
            ))),
            _other => {
                if let Ok(v) = ::treesitter_types::maybe_grow_stack(|| {
                    <Lhs as ::treesitter_types::FromNode>::from_node(node, src)
                }) {
                    Ok(Self::Lhs(::std::boxed::Box::new(v)))
                } else {
                    if let Ok(v) = ::treesitter_types::maybe_grow_stack(|| {
                        <SimpleNumeric as ::treesitter_types::FromNode>::from_node(node, src)
                    }) {
                        Ok(Self::SimpleNumeric(::std::boxed::Box::new(v)))
                    } else {
                        Err(::treesitter_types::ParseError::unexpected_kind(
                            _other, node,
                        ))
                    }
                }
            }
        }
    }
}
impl ::treesitter_types::Spanned for Primary<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        match self {
            Self::Lhs(inner) => inner.span(),
            Self::SimpleNumeric(inner) => inner.span(),
            Self::Array(inner) => inner.span(),
            Self::Begin(inner) => inner.span(),
            Self::Break(inner) => inner.span(),
            Self::Call(inner) => inner.span(),
            Self::Case(inner) => inner.span(),
            Self::CaseMatch(inner) => inner.span(),
            Self::ChainedString(inner) => inner.span(),
            Self::Character(inner) => inner.span(),
            Self::Class(inner) => inner.span(),
            Self::DelimitedSymbol(inner) => inner.span(),
            Self::For(inner) => inner.span(),
            Self::Hash(inner) => inner.span(),
            Self::HeredocBeginning(inner) => inner.span(),
            Self::If(inner) => inner.span(),
            Self::Lambda(inner) => inner.span(),
            Self::Method(inner) => inner.span(),
            Self::Module(inner) => inner.span(),
            Self::Next(inner) => inner.span(),
            Self::ParenthesizedStatements(inner) => inner.span(),
            Self::Redo(inner) => inner.span(),
            Self::Regex(inner) => inner.span(),
            Self::Retry(inner) => inner.span(),
            Self::Return(inner) => inner.span(),
            Self::SimpleSymbol(inner) => inner.span(),
            Self::SingletonClass(inner) => inner.span(),
            Self::SingletonMethod(inner) => inner.span(),
            Self::String(inner) => inner.span(),
            Self::StringArray(inner) => inner.span(),
            Self::Subshell(inner) => inner.span(),
            Self::SymbolArray(inner) => inner.span(),
            Self::Unary(inner) => inner.span(),
            Self::Unless(inner) => inner.span(),
            Self::Until(inner) => inner.span(),
            Self::While(inner) => inner.span(),
            Self::Yield(inner) => inner.span(),
        }
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum SimpleNumeric<'tree> {
    Complex(::std::boxed::Box<Complex<'tree>>),
    Float(::std::boxed::Box<Float<'tree>>),
    Integer(::std::boxed::Box<Integer<'tree>>),
    Rational(::std::boxed::Box<Rational<'tree>>),
}
impl<'tree> ::treesitter_types::FromNode<'tree> for SimpleNumeric<'tree> {
    #[allow(clippy::collapsible_else_if)]
    fn from_node(
        node: ::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        match node.kind() {
            "complex" => Ok(Self::Complex(::std::boxed::Box::new(
                ::treesitter_types::maybe_grow_stack(|| {
                    <Complex as ::treesitter_types::FromNode>::from_node(node, src)
                })?,
            ))),
            "float" => Ok(Self::Float(::std::boxed::Box::new(
                ::treesitter_types::maybe_grow_stack(|| {
                    <Float as ::treesitter_types::FromNode>::from_node(node, src)
                })?,
            ))),
            "integer" => Ok(Self::Integer(::std::boxed::Box::new(
                ::treesitter_types::maybe_grow_stack(|| {
                    <Integer as ::treesitter_types::FromNode>::from_node(node, src)
                })?,
            ))),
            "rational" => Ok(Self::Rational(::std::boxed::Box::new(
                ::treesitter_types::maybe_grow_stack(|| {
                    <Rational as ::treesitter_types::FromNode>::from_node(node, src)
                })?,
            ))),
            other => Err(::treesitter_types::ParseError::unexpected_kind(other, node)),
        }
    }
}
impl ::treesitter_types::Spanned for SimpleNumeric<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        match self {
            Self::Complex(inner) => inner.span(),
            Self::Float(inner) => inner.span(),
            Self::Integer(inner) => inner.span(),
            Self::Rational(inner) => inner.span(),
        }
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Statement<'tree> {
    Expression(::std::boxed::Box<Expression<'tree>>),
    Alias(::std::boxed::Box<Alias<'tree>>),
    BeginBlock(::std::boxed::Box<BeginBlock<'tree>>),
    EndBlock(::std::boxed::Box<EndBlock<'tree>>),
    IfModifier(::std::boxed::Box<IfModifier<'tree>>),
    RescueModifier(::std::boxed::Box<RescueModifier<'tree>>),
    Undef(::std::boxed::Box<Undef<'tree>>),
    UnlessModifier(::std::boxed::Box<UnlessModifier<'tree>>),
    UntilModifier(::std::boxed::Box<UntilModifier<'tree>>),
    WhileModifier(::std::boxed::Box<WhileModifier<'tree>>),
}
impl<'tree> ::treesitter_types::FromNode<'tree> for Statement<'tree> {
    #[allow(clippy::collapsible_else_if)]
    fn from_node(
        node: ::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        match node.kind() {
            "alias" => Ok(Self::Alias(::std::boxed::Box::new(
                ::treesitter_types::maybe_grow_stack(|| {
                    <Alias as ::treesitter_types::FromNode>::from_node(node, src)
                })?,
            ))),
            "begin_block" => Ok(Self::BeginBlock(::std::boxed::Box::new(
                ::treesitter_types::maybe_grow_stack(|| {
                    <BeginBlock as ::treesitter_types::FromNode>::from_node(node, src)
                })?,
            ))),
            "end_block" => Ok(Self::EndBlock(::std::boxed::Box::new(
                ::treesitter_types::maybe_grow_stack(|| {
                    <EndBlock as ::treesitter_types::FromNode>::from_node(node, src)
                })?,
            ))),
            "if_modifier" => Ok(Self::IfModifier(::std::boxed::Box::new(
                ::treesitter_types::maybe_grow_stack(|| {
                    <IfModifier as ::treesitter_types::FromNode>::from_node(node, src)
                })?,
            ))),
            "rescue_modifier" => Ok(Self::RescueModifier(::std::boxed::Box::new(
                ::treesitter_types::maybe_grow_stack(|| {
                    <RescueModifier as ::treesitter_types::FromNode>::from_node(node, src)
                })?,
            ))),
            "undef" => Ok(Self::Undef(::std::boxed::Box::new(
                ::treesitter_types::maybe_grow_stack(|| {
                    <Undef as ::treesitter_types::FromNode>::from_node(node, src)
                })?,
            ))),
            "unless_modifier" => Ok(Self::UnlessModifier(::std::boxed::Box::new(
                ::treesitter_types::maybe_grow_stack(|| {
                    <UnlessModifier as ::treesitter_types::FromNode>::from_node(node, src)
                })?,
            ))),
            "until_modifier" => Ok(Self::UntilModifier(::std::boxed::Box::new(
                ::treesitter_types::maybe_grow_stack(|| {
                    <UntilModifier as ::treesitter_types::FromNode>::from_node(node, src)
                })?,
            ))),
            "while_modifier" => Ok(Self::WhileModifier(::std::boxed::Box::new(
                ::treesitter_types::maybe_grow_stack(|| {
                    <WhileModifier as ::treesitter_types::FromNode>::from_node(node, src)
                })?,
            ))),
            _other => {
                if let Ok(v) = ::treesitter_types::maybe_grow_stack(|| {
                    <Expression as ::treesitter_types::FromNode>::from_node(node, src)
                }) {
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
impl ::treesitter_types::Spanned for Statement<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        match self {
            Self::Expression(inner) => inner.span(),
            Self::Alias(inner) => inner.span(),
            Self::BeginBlock(inner) => inner.span(),
            Self::EndBlock(inner) => inner.span(),
            Self::IfModifier(inner) => inner.span(),
            Self::RescueModifier(inner) => inner.span(),
            Self::Undef(inner) => inner.span(),
            Self::UnlessModifier(inner) => inner.span(),
            Self::UntilModifier(inner) => inner.span(),
            Self::WhileModifier(inner) => inner.span(),
        }
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Variable<'tree> {
    NonlocalVariable(::std::boxed::Box<NonlocalVariable<'tree>>),
    Constant(::std::boxed::Box<Constant<'tree>>),
    Identifier(::std::boxed::Box<Identifier<'tree>>),
    SelfType(::std::boxed::Box<SelfType<'tree>>),
    Super(::std::boxed::Box<Super<'tree>>),
}
impl<'tree> ::treesitter_types::FromNode<'tree> for Variable<'tree> {
    #[allow(clippy::collapsible_else_if)]
    fn from_node(
        node: ::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        match node.kind() {
            "constant" => Ok(Self::Constant(::std::boxed::Box::new(
                ::treesitter_types::maybe_grow_stack(|| {
                    <Constant as ::treesitter_types::FromNode>::from_node(node, src)
                })?,
            ))),
            "identifier" => Ok(Self::Identifier(::std::boxed::Box::new(
                ::treesitter_types::maybe_grow_stack(|| {
                    <Identifier as ::treesitter_types::FromNode>::from_node(node, src)
                })?,
            ))),
            "self" => Ok(Self::SelfType(::std::boxed::Box::new(
                ::treesitter_types::maybe_grow_stack(|| {
                    <SelfType as ::treesitter_types::FromNode>::from_node(node, src)
                })?,
            ))),
            "super" => Ok(Self::Super(::std::boxed::Box::new(
                ::treesitter_types::maybe_grow_stack(|| {
                    <Super as ::treesitter_types::FromNode>::from_node(node, src)
                })?,
            ))),
            _other => {
                if let Ok(v) = ::treesitter_types::maybe_grow_stack(|| {
                    <NonlocalVariable as ::treesitter_types::FromNode>::from_node(node, src)
                }) {
                    Ok(Self::NonlocalVariable(::std::boxed::Box::new(v)))
                } else {
                    Err(::treesitter_types::ParseError::unexpected_kind(
                        _other, node,
                    ))
                }
            }
        }
    }
}
impl ::treesitter_types::Spanned for Variable<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        match self {
            Self::NonlocalVariable(inner) => inner.span(),
            Self::Constant(inner) => inner.span(),
            Self::Identifier(inner) => inner.span(),
            Self::SelfType(inner) => inner.span(),
            Self::Super(inner) => inner.span(),
        }
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Alias<'tree> {
    pub span: ::treesitter_types::Span,
    pub alias: MethodName<'tree>,
    pub name: MethodName<'tree>,
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
            alias: {
                let child = node
                    .child_by_field_name("alias")
                    .ok_or_else(|| ::treesitter_types::ParseError::missing_field("alias", node))?;
                ::treesitter_types::maybe_grow_stack(|| {
                    <MethodName as ::treesitter_types::FromNode>::from_node(child, src)
                })?
            },
            name: {
                let child = node
                    .child_by_field_name("name")
                    .ok_or_else(|| ::treesitter_types::ParseError::missing_field("name", node))?;
                ::treesitter_types::maybe_grow_stack(|| {
                    <MethodName as ::treesitter_types::FromNode>::from_node(child, src)
                })?
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
pub struct AlternativePattern<'tree> {
    pub span: ::treesitter_types::Span,
    pub alternatives: ::std::vec::Vec<PatternExprBasic<'tree>>,
}
impl<'tree> ::treesitter_types::FromNode<'tree> for AlternativePattern<'tree> {
    #[allow(clippy::match_single_binding, clippy::suspicious_else_formatting)]
    fn from_node(
        node: ::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        debug_assert_eq!(node.kind(), "alternative_pattern");
        Ok(Self {
            span: ::treesitter_types::Span::from(node),
            alternatives: {
                let mut cursor = node.walk();
                let mut items = ::std::vec::Vec::new();
                for child in node.children_by_field_name("alternatives", &mut cursor) {
                    items.push(::treesitter_types::maybe_grow_stack(|| {
                        <PatternExprBasic as ::treesitter_types::FromNode>::from_node(child, src)
                    })?);
                }
                items
            },
        })
    }
}
impl ::treesitter_types::Spanned for AlternativePattern<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        self.span
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ArgumentList<'tree> {
    pub span: ::treesitter_types::Span,
    pub children: ::std::vec::Vec<ArgumentListChildren<'tree>>,
}
impl<'tree> ::treesitter_types::FromNode<'tree> for ArgumentList<'tree> {
    #[allow(clippy::match_single_binding, clippy::suspicious_else_formatting)]
    fn from_node(
        node: ::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        debug_assert_eq!(node.kind(), "argument_list");
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
                        <ArgumentListChildren as ::treesitter_types::FromNode>::from_node(
                            child, src,
                        )
                    })?);
                }
                items
            },
        })
    }
}
impl ::treesitter_types::Spanned for ArgumentList<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        self.span
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
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
                    items.push(::treesitter_types::maybe_grow_stack(|| {
                        <ArrayChildren as ::treesitter_types::FromNode>::from_node(child, src)
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
pub struct ArrayPattern<'tree> {
    pub span: ::treesitter_types::Span,
    pub class: ::core::option::Option<PatternConstant<'tree>>,
    pub children: ::std::vec::Vec<ArrayPatternChildren<'tree>>,
}
impl<'tree> ::treesitter_types::FromNode<'tree> for ArrayPattern<'tree> {
    #[allow(clippy::match_single_binding, clippy::suspicious_else_formatting)]
    fn from_node(
        node: ::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        debug_assert_eq!(node.kind(), "array_pattern");
        Ok(Self {
            span: ::treesitter_types::Span::from(node),
            class: match node.child_by_field_name("class") {
                Some(child) => Some(::treesitter_types::maybe_grow_stack(|| {
                    <PatternConstant as ::treesitter_types::FromNode>::from_node(child, src)
                })?),
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
                    items.push(::treesitter_types::maybe_grow_stack(|| {
                        <ArrayPatternChildren as ::treesitter_types::FromNode>::from_node(
                            child, src,
                        )
                    })?);
                }
                items
            },
        })
    }
}
impl ::treesitter_types::Spanned for ArrayPattern<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        self.span
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct AsPattern<'tree> {
    pub span: ::treesitter_types::Span,
    pub name: Identifier<'tree>,
    pub value: PatternExpr<'tree>,
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
            name: {
                let child = node
                    .child_by_field_name("name")
                    .ok_or_else(|| ::treesitter_types::ParseError::missing_field("name", node))?;
                ::treesitter_types::maybe_grow_stack(|| {
                    <Identifier as ::treesitter_types::FromNode>::from_node(child, src)
                })?
            },
            value: {
                let child = node
                    .child_by_field_name("value")
                    .ok_or_else(|| ::treesitter_types::ParseError::missing_field("value", node))?;
                ::treesitter_types::maybe_grow_stack(|| {
                    <PatternExpr as ::treesitter_types::FromNode>::from_node(child, src)
                })?
            },
        })
    }
}
impl ::treesitter_types::Spanned for AsPattern<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        self.span
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Assignment<'tree> {
    pub span: ::treesitter_types::Span,
    pub left: AssignmentLeft<'tree>,
    pub right: AssignmentRight<'tree>,
}
impl<'tree> ::treesitter_types::FromNode<'tree> for Assignment<'tree> {
    #[allow(clippy::match_single_binding, clippy::suspicious_else_formatting)]
    fn from_node(
        node: ::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        debug_assert_eq!(node.kind(), "assignment");
        Ok(Self {
            span: ::treesitter_types::Span::from(node),
            left: {
                let child = node
                    .child_by_field_name("left")
                    .ok_or_else(|| ::treesitter_types::ParseError::missing_field("left", node))?;
                ::treesitter_types::maybe_grow_stack(|| {
                    <AssignmentLeft as ::treesitter_types::FromNode>::from_node(child, src)
                })?
            },
            right: {
                let child = node
                    .child_by_field_name("right")
                    .ok_or_else(|| ::treesitter_types::ParseError::missing_field("right", node))?;
                ::treesitter_types::maybe_grow_stack(|| {
                    <AssignmentRight as ::treesitter_types::FromNode>::from_node(child, src)
                })?
            },
        })
    }
}
impl ::treesitter_types::Spanned for Assignment<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        self.span
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct BareString<'tree> {
    pub span: ::treesitter_types::Span,
    pub children: ::std::vec::Vec<BareStringChildren<'tree>>,
}
impl<'tree> ::treesitter_types::FromNode<'tree> for BareString<'tree> {
    #[allow(clippy::match_single_binding, clippy::suspicious_else_formatting)]
    fn from_node(
        node: ::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        debug_assert_eq!(node.kind(), "bare_string");
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
                        <BareStringChildren as ::treesitter_types::FromNode>::from_node(child, src)
                    })?);
                }
                items
            },
        })
    }
}
impl ::treesitter_types::Spanned for BareString<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        self.span
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct BareSymbol<'tree> {
    pub span: ::treesitter_types::Span,
    pub children: ::std::vec::Vec<BareSymbolChildren<'tree>>,
}
impl<'tree> ::treesitter_types::FromNode<'tree> for BareSymbol<'tree> {
    #[allow(clippy::match_single_binding, clippy::suspicious_else_formatting)]
    fn from_node(
        node: ::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        debug_assert_eq!(node.kind(), "bare_symbol");
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
                        <BareSymbolChildren as ::treesitter_types::FromNode>::from_node(child, src)
                    })?);
                }
                items
            },
        })
    }
}
impl ::treesitter_types::Spanned for BareSymbol<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        self.span
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Begin<'tree> {
    pub span: ::treesitter_types::Span,
    pub children: ::std::vec::Vec<BeginChildren<'tree>>,
}
impl<'tree> ::treesitter_types::FromNode<'tree> for Begin<'tree> {
    #[allow(clippy::match_single_binding, clippy::suspicious_else_formatting)]
    fn from_node(
        node: ::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        debug_assert_eq!(node.kind(), "begin");
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
                        <BeginChildren as ::treesitter_types::FromNode>::from_node(child, src)
                    })?);
                }
                items
            },
        })
    }
}
impl ::treesitter_types::Spanned for Begin<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        self.span
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct BeginBlock<'tree> {
    pub span: ::treesitter_types::Span,
    pub children: ::std::vec::Vec<BeginBlockChildren<'tree>>,
}
impl<'tree> ::treesitter_types::FromNode<'tree> for BeginBlock<'tree> {
    #[allow(clippy::match_single_binding, clippy::suspicious_else_formatting)]
    fn from_node(
        node: ::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        debug_assert_eq!(node.kind(), "begin_block");
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
                        <BeginBlockChildren as ::treesitter_types::FromNode>::from_node(child, src)
                    })?);
                }
                items
            },
        })
    }
}
impl ::treesitter_types::Spanned for BeginBlock<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        self.span
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Binary<'tree> {
    pub span: ::treesitter_types::Span,
    pub left: BinaryLeft<'tree>,
    pub operator: BinaryOperator,
    pub right: Expression<'tree>,
}
impl<'tree> ::treesitter_types::FromNode<'tree> for Binary<'tree> {
    #[allow(clippy::match_single_binding, clippy::suspicious_else_formatting)]
    fn from_node(
        node: ::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        debug_assert_eq!(node.kind(), "binary");
        Ok(Self {
            span: ::treesitter_types::Span::from(node),
            left: {
                let child = node
                    .child_by_field_name("left")
                    .ok_or_else(|| ::treesitter_types::ParseError::missing_field("left", node))?;
                ::treesitter_types::maybe_grow_stack(|| {
                    <BinaryLeft as ::treesitter_types::FromNode>::from_node(child, src)
                })?
            },
            operator: {
                let child = node.child_by_field_name("operator").ok_or_else(|| {
                    ::treesitter_types::ParseError::missing_field("operator", node)
                })?;
                ::treesitter_types::maybe_grow_stack(|| {
                    <BinaryOperator as ::treesitter_types::FromNode>::from_node(child, src)
                })?
            },
            right: {
                let child = node
                    .child_by_field_name("right")
                    .ok_or_else(|| ::treesitter_types::ParseError::missing_field("right", node))?;
                ::treesitter_types::maybe_grow_stack(|| {
                    <Expression as ::treesitter_types::FromNode>::from_node(child, src)
                })?
            },
        })
    }
}
impl ::treesitter_types::Spanned for Binary<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        self.span
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Block<'tree> {
    pub span: ::treesitter_types::Span,
    pub body: ::core::option::Option<BlockBody<'tree>>,
    pub parameters: ::core::option::Option<BlockParameters<'tree>>,
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
            body: match node.child_by_field_name("body") {
                Some(child) => Some(::treesitter_types::maybe_grow_stack(|| {
                    <BlockBody as ::treesitter_types::FromNode>::from_node(child, src)
                })?),
                None => None,
            },
            parameters: match node.child_by_field_name("parameters") {
                Some(child) => Some(::treesitter_types::maybe_grow_stack(|| {
                    <BlockParameters as ::treesitter_types::FromNode>::from_node(child, src)
                })?),
                None => None,
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
pub struct BlockArgument<'tree> {
    pub span: ::treesitter_types::Span,
    pub children: ::core::option::Option<Arg<'tree>>,
}
impl<'tree> ::treesitter_types::FromNode<'tree> for BlockArgument<'tree> {
    #[allow(clippy::match_single_binding, clippy::suspicious_else_formatting)]
    fn from_node(
        node: ::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        debug_assert_eq!(node.kind(), "block_argument");
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
                    Some(&child) => Some(::treesitter_types::maybe_grow_stack(|| {
                        <Arg as ::treesitter_types::FromNode>::from_node(child, src)
                    })?),
                    None => None,
                }
            },
        })
    }
}
impl ::treesitter_types::Spanned for BlockArgument<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        self.span
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct BlockBody<'tree> {
    pub span: ::treesitter_types::Span,
    pub children: ::std::vec::Vec<BlockBodyChildren<'tree>>,
}
impl<'tree> ::treesitter_types::FromNode<'tree> for BlockBody<'tree> {
    #[allow(clippy::match_single_binding, clippy::suspicious_else_formatting)]
    fn from_node(
        node: ::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        debug_assert_eq!(node.kind(), "block_body");
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
                        <BlockBodyChildren as ::treesitter_types::FromNode>::from_node(child, src)
                    })?);
                }
                items
            },
        })
    }
}
impl ::treesitter_types::Spanned for BlockBody<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        self.span
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct BlockParameter<'tree> {
    pub span: ::treesitter_types::Span,
    pub name: ::core::option::Option<Identifier<'tree>>,
}
impl<'tree> ::treesitter_types::FromNode<'tree> for BlockParameter<'tree> {
    #[allow(clippy::match_single_binding, clippy::suspicious_else_formatting)]
    fn from_node(
        node: ::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        debug_assert_eq!(node.kind(), "block_parameter");
        Ok(Self {
            span: ::treesitter_types::Span::from(node),
            name: match node.child_by_field_name("name") {
                Some(child) => Some(::treesitter_types::maybe_grow_stack(|| {
                    <Identifier as ::treesitter_types::FromNode>::from_node(child, src)
                })?),
                None => None,
            },
        })
    }
}
impl ::treesitter_types::Spanned for BlockParameter<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        self.span
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct BlockParameters<'tree> {
    pub span: ::treesitter_types::Span,
    pub locals: ::std::vec::Vec<Identifier<'tree>>,
    pub children: ::std::vec::Vec<BlockParametersChildren<'tree>>,
}
impl<'tree> ::treesitter_types::FromNode<'tree> for BlockParameters<'tree> {
    #[allow(clippy::match_single_binding, clippy::suspicious_else_formatting)]
    fn from_node(
        node: ::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        debug_assert_eq!(node.kind(), "block_parameters");
        Ok(Self {
            span: ::treesitter_types::Span::from(node),
            locals: {
                let mut cursor = node.walk();
                let mut items = ::std::vec::Vec::new();
                for child in node.children_by_field_name("locals", &mut cursor) {
                    items.push(::treesitter_types::maybe_grow_stack(|| {
                        <Identifier as ::treesitter_types::FromNode>::from_node(child, src)
                    })?);
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
                    items.push(::treesitter_types::maybe_grow_stack(|| {
                        <BlockParametersChildren as ::treesitter_types::FromNode>::from_node(
                            child, src,
                        )
                    })?);
                }
                items
            },
        })
    }
}
impl ::treesitter_types::Spanned for BlockParameters<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        self.span
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct BodyStatement<'tree> {
    pub span: ::treesitter_types::Span,
    pub children: ::std::vec::Vec<BodyStatementChildren<'tree>>,
}
impl<'tree> ::treesitter_types::FromNode<'tree> for BodyStatement<'tree> {
    #[allow(clippy::match_single_binding, clippy::suspicious_else_formatting)]
    fn from_node(
        node: ::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        debug_assert_eq!(node.kind(), "body_statement");
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
                        <BodyStatementChildren as ::treesitter_types::FromNode>::from_node(
                            child, src,
                        )
                    })?);
                }
                items
            },
        })
    }
}
impl ::treesitter_types::Spanned for BodyStatement<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        self.span
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Break<'tree> {
    pub span: ::treesitter_types::Span,
    pub children: ::core::option::Option<ArgumentList<'tree>>,
}
impl<'tree> ::treesitter_types::FromNode<'tree> for Break<'tree> {
    #[allow(clippy::match_single_binding, clippy::suspicious_else_formatting)]
    fn from_node(
        node: ::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        debug_assert_eq!(node.kind(), "break");
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
                    Some(&child) => Some(::treesitter_types::maybe_grow_stack(|| {
                        <ArgumentList as ::treesitter_types::FromNode>::from_node(child, src)
                    })?),
                    None => None,
                }
            },
        })
    }
}
impl ::treesitter_types::Spanned for Break<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        self.span
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Call<'tree> {
    pub span: ::treesitter_types::Span,
    pub arguments: ::core::option::Option<ArgumentList<'tree>>,
    pub block: ::core::option::Option<CallBlock<'tree>>,
    pub method: ::core::option::Option<CallMethod<'tree>>,
    pub operator: ::core::option::Option<CallOperator>,
    pub receiver: ::core::option::Option<Primary<'tree>>,
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
            arguments: match node.child_by_field_name("arguments") {
                Some(child) => Some(::treesitter_types::maybe_grow_stack(|| {
                    <ArgumentList as ::treesitter_types::FromNode>::from_node(child, src)
                })?),
                None => None,
            },
            block: match node.child_by_field_name("block") {
                Some(child) => Some(::treesitter_types::maybe_grow_stack(|| {
                    <CallBlock as ::treesitter_types::FromNode>::from_node(child, src)
                })?),
                None => None,
            },
            method: match node.child_by_field_name("method") {
                Some(child) => Some(::treesitter_types::maybe_grow_stack(|| {
                    <CallMethod as ::treesitter_types::FromNode>::from_node(child, src)
                })?),
                None => None,
            },
            operator: match node.child_by_field_name("operator") {
                Some(child) => Some(::treesitter_types::maybe_grow_stack(|| {
                    <CallOperator as ::treesitter_types::FromNode>::from_node(child, src)
                })?),
                None => None,
            },
            receiver: match node.child_by_field_name("receiver") {
                Some(child) => Some(::treesitter_types::maybe_grow_stack(|| {
                    <Primary as ::treesitter_types::FromNode>::from_node(child, src)
                })?),
                None => None,
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
pub struct Case<'tree> {
    pub span: ::treesitter_types::Span,
    pub value: ::core::option::Option<Statement<'tree>>,
    pub children: ::std::vec::Vec<CaseChildren<'tree>>,
}
impl<'tree> ::treesitter_types::FromNode<'tree> for Case<'tree> {
    #[allow(clippy::match_single_binding, clippy::suspicious_else_formatting)]
    fn from_node(
        node: ::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        debug_assert_eq!(node.kind(), "case");
        Ok(Self {
            span: ::treesitter_types::Span::from(node),
            value: match node.child_by_field_name("value") {
                Some(child) => Some(::treesitter_types::maybe_grow_stack(|| {
                    <Statement as ::treesitter_types::FromNode>::from_node(child, src)
                })?),
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
                    items.push(::treesitter_types::maybe_grow_stack(|| {
                        <CaseChildren as ::treesitter_types::FromNode>::from_node(child, src)
                    })?);
                }
                items
            },
        })
    }
}
impl ::treesitter_types::Spanned for Case<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        self.span
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CaseMatch<'tree> {
    pub span: ::treesitter_types::Span,
    pub clauses: ::std::vec::Vec<InClause<'tree>>,
    pub r#else: ::core::option::Option<Else<'tree>>,
    pub value: Statement<'tree>,
}
impl<'tree> ::treesitter_types::FromNode<'tree> for CaseMatch<'tree> {
    #[allow(clippy::match_single_binding, clippy::suspicious_else_formatting)]
    fn from_node(
        node: ::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        debug_assert_eq!(node.kind(), "case_match");
        Ok(Self {
            span: ::treesitter_types::Span::from(node),
            clauses: {
                let mut cursor = node.walk();
                let mut items = ::std::vec::Vec::new();
                for child in node.children_by_field_name("clauses", &mut cursor) {
                    items.push(::treesitter_types::maybe_grow_stack(|| {
                        <InClause as ::treesitter_types::FromNode>::from_node(child, src)
                    })?);
                }
                items
            },
            r#else: match node.child_by_field_name("else") {
                Some(child) => Some(::treesitter_types::maybe_grow_stack(|| {
                    <Else as ::treesitter_types::FromNode>::from_node(child, src)
                })?),
                None => None,
            },
            value: {
                let child = node
                    .child_by_field_name("value")
                    .ok_or_else(|| ::treesitter_types::ParseError::missing_field("value", node))?;
                ::treesitter_types::maybe_grow_stack(|| {
                    <Statement as ::treesitter_types::FromNode>::from_node(child, src)
                })?
            },
        })
    }
}
impl ::treesitter_types::Spanned for CaseMatch<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        self.span
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ChainedString<'tree> {
    pub span: ::treesitter_types::Span,
    pub children: ::std::vec::Vec<String<'tree>>,
}
impl<'tree> ::treesitter_types::FromNode<'tree> for ChainedString<'tree> {
    #[allow(clippy::match_single_binding, clippy::suspicious_else_formatting)]
    fn from_node(
        node: ::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        debug_assert_eq!(node.kind(), "chained_string");
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
                        <String as ::treesitter_types::FromNode>::from_node(child, src)
                    })?);
                }
                items
            },
        })
    }
}
impl ::treesitter_types::Spanned for ChainedString<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        self.span
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Class<'tree> {
    pub span: ::treesitter_types::Span,
    pub body: ::core::option::Option<BodyStatement<'tree>>,
    pub name: ClassName<'tree>,
    pub superclass: ::core::option::Option<Superclass<'tree>>,
}
impl<'tree> ::treesitter_types::FromNode<'tree> for Class<'tree> {
    #[allow(clippy::match_single_binding, clippy::suspicious_else_formatting)]
    fn from_node(
        node: ::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        debug_assert_eq!(node.kind(), "class");
        Ok(Self {
            span: ::treesitter_types::Span::from(node),
            body: match node.child_by_field_name("body") {
                Some(child) => Some(::treesitter_types::maybe_grow_stack(|| {
                    <BodyStatement as ::treesitter_types::FromNode>::from_node(child, src)
                })?),
                None => None,
            },
            name: {
                let child = node
                    .child_by_field_name("name")
                    .ok_or_else(|| ::treesitter_types::ParseError::missing_field("name", node))?;
                ::treesitter_types::maybe_grow_stack(|| {
                    <ClassName as ::treesitter_types::FromNode>::from_node(child, src)
                })?
            },
            superclass: match node.child_by_field_name("superclass") {
                Some(child) => Some(::treesitter_types::maybe_grow_stack(|| {
                    <Superclass as ::treesitter_types::FromNode>::from_node(child, src)
                })?),
                None => None,
            },
        })
    }
}
impl ::treesitter_types::Spanned for Class<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        self.span
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Complex<'tree> {
    pub span: ::treesitter_types::Span,
    pub children: ComplexChildren<'tree>,
}
impl<'tree> ::treesitter_types::FromNode<'tree> for Complex<'tree> {
    #[allow(clippy::match_single_binding, clippy::suspicious_else_formatting)]
    fn from_node(
        node: ::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        debug_assert_eq!(node.kind(), "complex");
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
                                        ::treesitter_types::maybe_grow_stack(|| <ComplexChildren as ::treesitter_types::FromNode>::from_node(
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
                                            ::treesitter_types::maybe_grow_stack(|| <ComplexChildren as ::treesitter_types::FromNode>::from_node(
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
                ::treesitter_types::maybe_grow_stack(|| {
                    <ComplexChildren as ::treesitter_types::FromNode>::from_node(child, src)
                })?
            },
        })
    }
}
impl ::treesitter_types::Spanned for Complex<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        self.span
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Conditional<'tree> {
    pub span: ::treesitter_types::Span,
    pub alternative: Arg<'tree>,
    pub condition: Arg<'tree>,
    pub consequence: Arg<'tree>,
}
impl<'tree> ::treesitter_types::FromNode<'tree> for Conditional<'tree> {
    #[allow(clippy::match_single_binding, clippy::suspicious_else_formatting)]
    fn from_node(
        node: ::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        debug_assert_eq!(node.kind(), "conditional");
        Ok(Self {
            span: ::treesitter_types::Span::from(node),
            alternative: {
                let child = node.child_by_field_name("alternative").ok_or_else(|| {
                    ::treesitter_types::ParseError::missing_field("alternative", node)
                })?;
                ::treesitter_types::maybe_grow_stack(|| {
                    <Arg as ::treesitter_types::FromNode>::from_node(child, src)
                })?
            },
            condition: {
                let child = node.child_by_field_name("condition").ok_or_else(|| {
                    ::treesitter_types::ParseError::missing_field("condition", node)
                })?;
                ::treesitter_types::maybe_grow_stack(|| {
                    <Arg as ::treesitter_types::FromNode>::from_node(child, src)
                })?
            },
            consequence: {
                let child = node.child_by_field_name("consequence").ok_or_else(|| {
                    ::treesitter_types::ParseError::missing_field("consequence", node)
                })?;
                ::treesitter_types::maybe_grow_stack(|| {
                    <Arg as ::treesitter_types::FromNode>::from_node(child, src)
                })?
            },
        })
    }
}
impl ::treesitter_types::Spanned for Conditional<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        self.span
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Constant<'tree> {
    pub span: ::treesitter_types::Span,
    text: &'tree str,
}
impl<'tree> ::treesitter_types::FromNode<'tree> for Constant<'tree> {
    fn from_node(
        node: ::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        debug_assert_eq!(node.kind(), "constant");
        Ok(Self {
            span: ::treesitter_types::Span::from(node),
            text: node.utf8_text(src)?,
        })
    }
}
impl<'tree> ::treesitter_types::LeafNode<'tree> for Constant<'tree> {
    fn text(&self) -> &'tree str {
        self.text
    }
}
impl ::treesitter_types::Spanned for Constant<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        self.span
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct DelimitedSymbol<'tree> {
    pub span: ::treesitter_types::Span,
    pub children: ::std::vec::Vec<DelimitedSymbolChildren<'tree>>,
}
impl<'tree> ::treesitter_types::FromNode<'tree> for DelimitedSymbol<'tree> {
    #[allow(clippy::match_single_binding, clippy::suspicious_else_formatting)]
    fn from_node(
        node: ::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        debug_assert_eq!(node.kind(), "delimited_symbol");
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
                        <DelimitedSymbolChildren as ::treesitter_types::FromNode>::from_node(
                            child, src,
                        )
                    })?);
                }
                items
            },
        })
    }
}
impl ::treesitter_types::Spanned for DelimitedSymbol<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        self.span
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct DestructuredLeftAssignment<'tree> {
    pub span: ::treesitter_types::Span,
    pub children: ::std::vec::Vec<DestructuredLeftAssignmentChildren<'tree>>,
}
impl<'tree> ::treesitter_types::FromNode<'tree> for DestructuredLeftAssignment<'tree> {
    #[allow(clippy::match_single_binding, clippy::suspicious_else_formatting)]
    fn from_node(
        node: ::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        debug_assert_eq!(node.kind(), "destructured_left_assignment");
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
                            ::treesitter_types::maybe_grow_stack(|| <DestructuredLeftAssignmentChildren as ::treesitter_types::FromNode>::from_node(
                                child,
                                src,
                            ))?,
                        );
                }
                items
            },
        })
    }
}
impl ::treesitter_types::Spanned for DestructuredLeftAssignment<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        self.span
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct DestructuredParameter<'tree> {
    pub span: ::treesitter_types::Span,
    pub children: ::std::vec::Vec<DestructuredParameterChildren<'tree>>,
}
impl<'tree> ::treesitter_types::FromNode<'tree> for DestructuredParameter<'tree> {
    #[allow(clippy::match_single_binding, clippy::suspicious_else_formatting)]
    fn from_node(
        node: ::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        debug_assert_eq!(node.kind(), "destructured_parameter");
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
                        <DestructuredParameterChildren as ::treesitter_types::FromNode>::from_node(
                            child, src,
                        )
                    })?);
                }
                items
            },
        })
    }
}
impl ::treesitter_types::Spanned for DestructuredParameter<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        self.span
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Do<'tree> {
    pub span: ::treesitter_types::Span,
    pub children: ::std::vec::Vec<DoChildren<'tree>>,
}
impl<'tree> ::treesitter_types::FromNode<'tree> for Do<'tree> {
    #[allow(clippy::match_single_binding, clippy::suspicious_else_formatting)]
    fn from_node(
        node: ::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        debug_assert_eq!(node.kind(), "do");
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
                        <DoChildren as ::treesitter_types::FromNode>::from_node(child, src)
                    })?);
                }
                items
            },
        })
    }
}
impl ::treesitter_types::Spanned for Do<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        self.span
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct DoBlock<'tree> {
    pub span: ::treesitter_types::Span,
    pub body: ::core::option::Option<BodyStatement<'tree>>,
    pub parameters: ::core::option::Option<BlockParameters<'tree>>,
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
            body: match node.child_by_field_name("body") {
                Some(child) => Some(::treesitter_types::maybe_grow_stack(|| {
                    <BodyStatement as ::treesitter_types::FromNode>::from_node(child, src)
                })?),
                None => None,
            },
            parameters: match node.child_by_field_name("parameters") {
                Some(child) => Some(::treesitter_types::maybe_grow_stack(|| {
                    <BlockParameters as ::treesitter_types::FromNode>::from_node(child, src)
                })?),
                None => None,
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
pub struct ElementReference<'tree> {
    pub span: ::treesitter_types::Span,
    pub block: ::core::option::Option<ElementReferenceBlock<'tree>>,
    pub object: Primary<'tree>,
    pub children: ::std::vec::Vec<ElementReferenceChildren<'tree>>,
}
impl<'tree> ::treesitter_types::FromNode<'tree> for ElementReference<'tree> {
    #[allow(clippy::match_single_binding, clippy::suspicious_else_formatting)]
    fn from_node(
        node: ::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        debug_assert_eq!(node.kind(), "element_reference");
        Ok(Self {
            span: ::treesitter_types::Span::from(node),
            block: match node.child_by_field_name("block") {
                Some(child) => Some(::treesitter_types::maybe_grow_stack(|| {
                    <ElementReferenceBlock as ::treesitter_types::FromNode>::from_node(child, src)
                })?),
                None => None,
            },
            object: {
                let child = node
                    .child_by_field_name("object")
                    .ok_or_else(|| ::treesitter_types::ParseError::missing_field("object", node))?;
                ::treesitter_types::maybe_grow_stack(|| {
                    <Primary as ::treesitter_types::FromNode>::from_node(child, src)
                })?
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
                    items.push(::treesitter_types::maybe_grow_stack(|| {
                        <ElementReferenceChildren as ::treesitter_types::FromNode>::from_node(
                            child, src,
                        )
                    })?);
                }
                items
            },
        })
    }
}
impl ::treesitter_types::Spanned for ElementReference<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        self.span
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Else<'tree> {
    pub span: ::treesitter_types::Span,
    pub children: ::std::vec::Vec<ElseChildren<'tree>>,
}
impl<'tree> ::treesitter_types::FromNode<'tree> for Else<'tree> {
    #[allow(clippy::match_single_binding, clippy::suspicious_else_formatting)]
    fn from_node(
        node: ::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        debug_assert_eq!(node.kind(), "else");
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
                        <ElseChildren as ::treesitter_types::FromNode>::from_node(child, src)
                    })?);
                }
                items
            },
        })
    }
}
impl ::treesitter_types::Spanned for Else<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        self.span
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Elsif<'tree> {
    pub span: ::treesitter_types::Span,
    pub alternative: ::core::option::Option<ElsifAlternative<'tree>>,
    pub condition: Statement<'tree>,
    pub consequence: ::core::option::Option<Then<'tree>>,
}
impl<'tree> ::treesitter_types::FromNode<'tree> for Elsif<'tree> {
    #[allow(clippy::match_single_binding, clippy::suspicious_else_formatting)]
    fn from_node(
        node: ::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        debug_assert_eq!(node.kind(), "elsif");
        Ok(Self {
            span: ::treesitter_types::Span::from(node),
            alternative: match node.child_by_field_name("alternative") {
                Some(child) => Some(::treesitter_types::maybe_grow_stack(|| {
                    <ElsifAlternative as ::treesitter_types::FromNode>::from_node(child, src)
                })?),
                None => None,
            },
            condition: {
                let child = node.child_by_field_name("condition").ok_or_else(|| {
                    ::treesitter_types::ParseError::missing_field("condition", node)
                })?;
                ::treesitter_types::maybe_grow_stack(|| {
                    <Statement as ::treesitter_types::FromNode>::from_node(child, src)
                })?
            },
            consequence: match node.child_by_field_name("consequence") {
                Some(child) => Some(::treesitter_types::maybe_grow_stack(|| {
                    <Then as ::treesitter_types::FromNode>::from_node(child, src)
                })?),
                None => None,
            },
        })
    }
}
impl ::treesitter_types::Spanned for Elsif<'_> {
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
pub struct EndBlock<'tree> {
    pub span: ::treesitter_types::Span,
    pub children: ::std::vec::Vec<EndBlockChildren<'tree>>,
}
impl<'tree> ::treesitter_types::FromNode<'tree> for EndBlock<'tree> {
    #[allow(clippy::match_single_binding, clippy::suspicious_else_formatting)]
    fn from_node(
        node: ::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        debug_assert_eq!(node.kind(), "end_block");
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
                        <EndBlockChildren as ::treesitter_types::FromNode>::from_node(child, src)
                    })?);
                }
                items
            },
        })
    }
}
impl ::treesitter_types::Spanned for EndBlock<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        self.span
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Ensure<'tree> {
    pub span: ::treesitter_types::Span,
    pub children: ::std::vec::Vec<EnsureChildren<'tree>>,
}
impl<'tree> ::treesitter_types::FromNode<'tree> for Ensure<'tree> {
    #[allow(clippy::match_single_binding, clippy::suspicious_else_formatting)]
    fn from_node(
        node: ::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        debug_assert_eq!(node.kind(), "ensure");
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
                        <EnsureChildren as ::treesitter_types::FromNode>::from_node(child, src)
                    })?);
                }
                items
            },
        })
    }
}
impl ::treesitter_types::Spanned for Ensure<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        self.span
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ExceptionVariable<'tree> {
    pub span: ::treesitter_types::Span,
    pub children: Lhs<'tree>,
}
impl<'tree> ::treesitter_types::FromNode<'tree> for ExceptionVariable<'tree> {
    #[allow(clippy::match_single_binding, clippy::suspicious_else_formatting)]
    fn from_node(
        node: ::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        debug_assert_eq!(node.kind(), "exception_variable");
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
                                        ::treesitter_types::maybe_grow_stack(|| <Lhs as ::treesitter_types::FromNode>::from_node(
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
                                            ::treesitter_types::maybe_grow_stack(|| <Lhs as ::treesitter_types::FromNode>::from_node(
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
                ::treesitter_types::maybe_grow_stack(|| {
                    <Lhs as ::treesitter_types::FromNode>::from_node(child, src)
                })?
            },
        })
    }
}
impl ::treesitter_types::Spanned for ExceptionVariable<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        self.span
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Exceptions<'tree> {
    pub span: ::treesitter_types::Span,
    pub children: ::std::vec::Vec<ExceptionsChildren<'tree>>,
}
impl<'tree> ::treesitter_types::FromNode<'tree> for Exceptions<'tree> {
    #[allow(clippy::match_single_binding, clippy::suspicious_else_formatting)]
    fn from_node(
        node: ::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        debug_assert_eq!(node.kind(), "exceptions");
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
                        <ExceptionsChildren as ::treesitter_types::FromNode>::from_node(child, src)
                    })?);
                }
                items
            },
        })
    }
}
impl ::treesitter_types::Spanned for Exceptions<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        self.span
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ExpressionReferencePattern<'tree> {
    pub span: ::treesitter_types::Span,
    pub value: Expression<'tree>,
}
impl<'tree> ::treesitter_types::FromNode<'tree> for ExpressionReferencePattern<'tree> {
    #[allow(clippy::match_single_binding, clippy::suspicious_else_formatting)]
    fn from_node(
        node: ::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        debug_assert_eq!(node.kind(), "expression_reference_pattern");
        Ok(Self {
            span: ::treesitter_types::Span::from(node),
            value: {
                let child = node
                    .child_by_field_name("value")
                    .ok_or_else(|| ::treesitter_types::ParseError::missing_field("value", node))?;
                ::treesitter_types::maybe_grow_stack(|| {
                    <Expression as ::treesitter_types::FromNode>::from_node(child, src)
                })?
            },
        })
    }
}
impl ::treesitter_types::Spanned for ExpressionReferencePattern<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        self.span
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct FindPattern<'tree> {
    pub span: ::treesitter_types::Span,
    pub class: ::core::option::Option<PatternConstant<'tree>>,
    pub children: ::std::vec::Vec<FindPatternChildren<'tree>>,
}
impl<'tree> ::treesitter_types::FromNode<'tree> for FindPattern<'tree> {
    #[allow(clippy::match_single_binding, clippy::suspicious_else_formatting)]
    fn from_node(
        node: ::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        debug_assert_eq!(node.kind(), "find_pattern");
        Ok(Self {
            span: ::treesitter_types::Span::from(node),
            class: match node.child_by_field_name("class") {
                Some(child) => Some(::treesitter_types::maybe_grow_stack(|| {
                    <PatternConstant as ::treesitter_types::FromNode>::from_node(child, src)
                })?),
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
                    items.push(::treesitter_types::maybe_grow_stack(|| {
                        <FindPatternChildren as ::treesitter_types::FromNode>::from_node(child, src)
                    })?);
                }
                items
            },
        })
    }
}
impl ::treesitter_types::Spanned for FindPattern<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        self.span
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct For<'tree> {
    pub span: ::treesitter_types::Span,
    pub body: Do<'tree>,
    pub pattern: ForPattern<'tree>,
    pub value: In<'tree>,
}
impl<'tree> ::treesitter_types::FromNode<'tree> for For<'tree> {
    #[allow(clippy::match_single_binding, clippy::suspicious_else_formatting)]
    fn from_node(
        node: ::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        debug_assert_eq!(node.kind(), "for");
        Ok(Self {
            span: ::treesitter_types::Span::from(node),
            body: {
                let child = node
                    .child_by_field_name("body")
                    .ok_or_else(|| ::treesitter_types::ParseError::missing_field("body", node))?;
                ::treesitter_types::maybe_grow_stack(|| {
                    <Do as ::treesitter_types::FromNode>::from_node(child, src)
                })?
            },
            pattern: {
                let child = node.child_by_field_name("pattern").ok_or_else(|| {
                    ::treesitter_types::ParseError::missing_field("pattern", node)
                })?;
                ::treesitter_types::maybe_grow_stack(|| {
                    <ForPattern as ::treesitter_types::FromNode>::from_node(child, src)
                })?
            },
            value: {
                let child = node
                    .child_by_field_name("value")
                    .ok_or_else(|| ::treesitter_types::ParseError::missing_field("value", node))?;
                ::treesitter_types::maybe_grow_stack(|| {
                    <In as ::treesitter_types::FromNode>::from_node(child, src)
                })?
            },
        })
    }
}
impl ::treesitter_types::Spanned for For<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        self.span
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ForwardArgument<'tree> {
    pub span: ::treesitter_types::Span,
    text: &'tree str,
}
impl<'tree> ::treesitter_types::FromNode<'tree> for ForwardArgument<'tree> {
    fn from_node(
        node: ::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        debug_assert_eq!(node.kind(), "forward_argument");
        Ok(Self {
            span: ::treesitter_types::Span::from(node),
            text: node.utf8_text(src)?,
        })
    }
}
impl<'tree> ::treesitter_types::LeafNode<'tree> for ForwardArgument<'tree> {
    fn text(&self) -> &'tree str {
        self.text
    }
}
impl ::treesitter_types::Spanned for ForwardArgument<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        self.span
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ForwardParameter<'tree> {
    pub span: ::treesitter_types::Span,
    text: &'tree str,
}
impl<'tree> ::treesitter_types::FromNode<'tree> for ForwardParameter<'tree> {
    fn from_node(
        node: ::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        debug_assert_eq!(node.kind(), "forward_parameter");
        Ok(Self {
            span: ::treesitter_types::Span::from(node),
            text: node.utf8_text(src)?,
        })
    }
}
impl<'tree> ::treesitter_types::LeafNode<'tree> for ForwardParameter<'tree> {
    fn text(&self) -> &'tree str {
        self.text
    }
}
impl ::treesitter_types::Spanned for ForwardParameter<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        self.span
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Hash<'tree> {
    pub span: ::treesitter_types::Span,
    pub children: ::std::vec::Vec<HashChildren<'tree>>,
}
impl<'tree> ::treesitter_types::FromNode<'tree> for Hash<'tree> {
    #[allow(clippy::match_single_binding, clippy::suspicious_else_formatting)]
    fn from_node(
        node: ::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        debug_assert_eq!(node.kind(), "hash");
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
                        <HashChildren as ::treesitter_types::FromNode>::from_node(child, src)
                    })?);
                }
                items
            },
        })
    }
}
impl ::treesitter_types::Spanned for Hash<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        self.span
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct HashKeySymbol<'tree> {
    pub span: ::treesitter_types::Span,
    text: &'tree str,
}
impl<'tree> ::treesitter_types::FromNode<'tree> for HashKeySymbol<'tree> {
    fn from_node(
        node: ::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        debug_assert_eq!(node.kind(), "hash_key_symbol");
        Ok(Self {
            span: ::treesitter_types::Span::from(node),
            text: node.utf8_text(src)?,
        })
    }
}
impl<'tree> ::treesitter_types::LeafNode<'tree> for HashKeySymbol<'tree> {
    fn text(&self) -> &'tree str {
        self.text
    }
}
impl ::treesitter_types::Spanned for HashKeySymbol<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        self.span
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct HashPattern<'tree> {
    pub span: ::treesitter_types::Span,
    pub class: ::core::option::Option<PatternConstant<'tree>>,
    pub children: ::std::vec::Vec<HashPatternChildren<'tree>>,
}
impl<'tree> ::treesitter_types::FromNode<'tree> for HashPattern<'tree> {
    #[allow(clippy::match_single_binding, clippy::suspicious_else_formatting)]
    fn from_node(
        node: ::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        debug_assert_eq!(node.kind(), "hash_pattern");
        Ok(Self {
            span: ::treesitter_types::Span::from(node),
            class: match node.child_by_field_name("class") {
                Some(child) => Some(::treesitter_types::maybe_grow_stack(|| {
                    <PatternConstant as ::treesitter_types::FromNode>::from_node(child, src)
                })?),
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
                    items.push(::treesitter_types::maybe_grow_stack(|| {
                        <HashPatternChildren as ::treesitter_types::FromNode>::from_node(child, src)
                    })?);
                }
                items
            },
        })
    }
}
impl ::treesitter_types::Spanned for HashPattern<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        self.span
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct HashSplatArgument<'tree> {
    pub span: ::treesitter_types::Span,
    pub children: ::core::option::Option<Arg<'tree>>,
}
impl<'tree> ::treesitter_types::FromNode<'tree> for HashSplatArgument<'tree> {
    #[allow(clippy::match_single_binding, clippy::suspicious_else_formatting)]
    fn from_node(
        node: ::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        debug_assert_eq!(node.kind(), "hash_splat_argument");
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
                    Some(&child) => Some(::treesitter_types::maybe_grow_stack(|| {
                        <Arg as ::treesitter_types::FromNode>::from_node(child, src)
                    })?),
                    None => None,
                }
            },
        })
    }
}
impl ::treesitter_types::Spanned for HashSplatArgument<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        self.span
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct HashSplatNil<'tree> {
    pub span: ::treesitter_types::Span,
    text: &'tree str,
}
impl<'tree> ::treesitter_types::FromNode<'tree> for HashSplatNil<'tree> {
    fn from_node(
        node: ::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        debug_assert_eq!(node.kind(), "hash_splat_nil");
        Ok(Self {
            span: ::treesitter_types::Span::from(node),
            text: node.utf8_text(src)?,
        })
    }
}
impl<'tree> ::treesitter_types::LeafNode<'tree> for HashSplatNil<'tree> {
    fn text(&self) -> &'tree str {
        self.text
    }
}
impl ::treesitter_types::Spanned for HashSplatNil<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        self.span
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct HashSplatParameter<'tree> {
    pub span: ::treesitter_types::Span,
    pub name: ::core::option::Option<Identifier<'tree>>,
}
impl<'tree> ::treesitter_types::FromNode<'tree> for HashSplatParameter<'tree> {
    #[allow(clippy::match_single_binding, clippy::suspicious_else_formatting)]
    fn from_node(
        node: ::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        debug_assert_eq!(node.kind(), "hash_splat_parameter");
        Ok(Self {
            span: ::treesitter_types::Span::from(node),
            name: match node.child_by_field_name("name") {
                Some(child) => Some(::treesitter_types::maybe_grow_stack(|| {
                    <Identifier as ::treesitter_types::FromNode>::from_node(child, src)
                })?),
                None => None,
            },
        })
    }
}
impl ::treesitter_types::Spanned for HashSplatParameter<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        self.span
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
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
                    items.push(::treesitter_types::maybe_grow_stack(|| {
                        <HeredocBodyChildren as ::treesitter_types::FromNode>::from_node(child, src)
                    })?);
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
pub struct If<'tree> {
    pub span: ::treesitter_types::Span,
    pub alternative: ::core::option::Option<IfAlternative<'tree>>,
    pub condition: Statement<'tree>,
    pub consequence: ::core::option::Option<Then<'tree>>,
}
impl<'tree> ::treesitter_types::FromNode<'tree> for If<'tree> {
    #[allow(clippy::match_single_binding, clippy::suspicious_else_formatting)]
    fn from_node(
        node: ::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        debug_assert_eq!(node.kind(), "if");
        Ok(Self {
            span: ::treesitter_types::Span::from(node),
            alternative: match node.child_by_field_name("alternative") {
                Some(child) => Some(::treesitter_types::maybe_grow_stack(|| {
                    <IfAlternative as ::treesitter_types::FromNode>::from_node(child, src)
                })?),
                None => None,
            },
            condition: {
                let child = node.child_by_field_name("condition").ok_or_else(|| {
                    ::treesitter_types::ParseError::missing_field("condition", node)
                })?;
                ::treesitter_types::maybe_grow_stack(|| {
                    <Statement as ::treesitter_types::FromNode>::from_node(child, src)
                })?
            },
            consequence: match node.child_by_field_name("consequence") {
                Some(child) => Some(::treesitter_types::maybe_grow_stack(|| {
                    <Then as ::treesitter_types::FromNode>::from_node(child, src)
                })?),
                None => None,
            },
        })
    }
}
impl ::treesitter_types::Spanned for If<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        self.span
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct IfGuard<'tree> {
    pub span: ::treesitter_types::Span,
    pub condition: Expression<'tree>,
}
impl<'tree> ::treesitter_types::FromNode<'tree> for IfGuard<'tree> {
    #[allow(clippy::match_single_binding, clippy::suspicious_else_formatting)]
    fn from_node(
        node: ::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        debug_assert_eq!(node.kind(), "if_guard");
        Ok(Self {
            span: ::treesitter_types::Span::from(node),
            condition: {
                let child = node.child_by_field_name("condition").ok_or_else(|| {
                    ::treesitter_types::ParseError::missing_field("condition", node)
                })?;
                ::treesitter_types::maybe_grow_stack(|| {
                    <Expression as ::treesitter_types::FromNode>::from_node(child, src)
                })?
            },
        })
    }
}
impl ::treesitter_types::Spanned for IfGuard<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        self.span
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct IfModifier<'tree> {
    pub span: ::treesitter_types::Span,
    pub body: Statement<'tree>,
    pub condition: Expression<'tree>,
}
impl<'tree> ::treesitter_types::FromNode<'tree> for IfModifier<'tree> {
    #[allow(clippy::match_single_binding, clippy::suspicious_else_formatting)]
    fn from_node(
        node: ::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        debug_assert_eq!(node.kind(), "if_modifier");
        Ok(Self {
            span: ::treesitter_types::Span::from(node),
            body: {
                let child = node
                    .child_by_field_name("body")
                    .ok_or_else(|| ::treesitter_types::ParseError::missing_field("body", node))?;
                ::treesitter_types::maybe_grow_stack(|| {
                    <Statement as ::treesitter_types::FromNode>::from_node(child, src)
                })?
            },
            condition: {
                let child = node.child_by_field_name("condition").ok_or_else(|| {
                    ::treesitter_types::ParseError::missing_field("condition", node)
                })?;
                ::treesitter_types::maybe_grow_stack(|| {
                    <Expression as ::treesitter_types::FromNode>::from_node(child, src)
                })?
            },
        })
    }
}
impl ::treesitter_types::Spanned for IfModifier<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        self.span
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct In<'tree> {
    pub span: ::treesitter_types::Span,
    pub children: Arg<'tree>,
}
impl<'tree> ::treesitter_types::FromNode<'tree> for In<'tree> {
    #[allow(clippy::match_single_binding, clippy::suspicious_else_formatting)]
    fn from_node(
        node: ::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        debug_assert_eq!(node.kind(), "in");
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
                                        ::treesitter_types::maybe_grow_stack(|| <Arg as ::treesitter_types::FromNode>::from_node(
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
                                            ::treesitter_types::maybe_grow_stack(|| <Arg as ::treesitter_types::FromNode>::from_node(
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
                ::treesitter_types::maybe_grow_stack(|| {
                    <Arg as ::treesitter_types::FromNode>::from_node(child, src)
                })?
            },
        })
    }
}
impl ::treesitter_types::Spanned for In<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        self.span
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct InClause<'tree> {
    pub span: ::treesitter_types::Span,
    pub body: ::core::option::Option<Then<'tree>>,
    pub guard: ::core::option::Option<InClauseGuard<'tree>>,
    pub pattern: PatternTopExprBody<'tree>,
}
impl<'tree> ::treesitter_types::FromNode<'tree> for InClause<'tree> {
    #[allow(clippy::match_single_binding, clippy::suspicious_else_formatting)]
    fn from_node(
        node: ::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        debug_assert_eq!(node.kind(), "in_clause");
        Ok(Self {
            span: ::treesitter_types::Span::from(node),
            body: match node.child_by_field_name("body") {
                Some(child) => Some(::treesitter_types::maybe_grow_stack(|| {
                    <Then as ::treesitter_types::FromNode>::from_node(child, src)
                })?),
                None => None,
            },
            guard: match node.child_by_field_name("guard") {
                Some(child) => Some(::treesitter_types::maybe_grow_stack(|| {
                    <InClauseGuard as ::treesitter_types::FromNode>::from_node(child, src)
                })?),
                None => None,
            },
            pattern: {
                let child = node.child_by_field_name("pattern").ok_or_else(|| {
                    ::treesitter_types::ParseError::missing_field("pattern", node)
                })?;
                ::treesitter_types::maybe_grow_stack(|| {
                    <PatternTopExprBody as ::treesitter_types::FromNode>::from_node(child, src)
                })?
            },
        })
    }
}
impl ::treesitter_types::Spanned for InClause<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        self.span
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Interpolation<'tree> {
    pub span: ::treesitter_types::Span,
    pub children: ::std::vec::Vec<InterpolationChildren<'tree>>,
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
                let mut items = ::std::vec::Vec::new();
                for child in non_field_children {
                    items.push(::treesitter_types::maybe_grow_stack(|| {
                        <InterpolationChildren as ::treesitter_types::FromNode>::from_node(
                            child, src,
                        )
                    })?);
                }
                items
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
pub struct KeywordParameter<'tree> {
    pub span: ::treesitter_types::Span,
    pub name: Identifier<'tree>,
    pub value: ::core::option::Option<Arg<'tree>>,
}
impl<'tree> ::treesitter_types::FromNode<'tree> for KeywordParameter<'tree> {
    #[allow(clippy::match_single_binding, clippy::suspicious_else_formatting)]
    fn from_node(
        node: ::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        debug_assert_eq!(node.kind(), "keyword_parameter");
        Ok(Self {
            span: ::treesitter_types::Span::from(node),
            name: {
                let child = node
                    .child_by_field_name("name")
                    .ok_or_else(|| ::treesitter_types::ParseError::missing_field("name", node))?;
                ::treesitter_types::maybe_grow_stack(|| {
                    <Identifier as ::treesitter_types::FromNode>::from_node(child, src)
                })?
            },
            value: match node.child_by_field_name("value") {
                Some(child) => Some(::treesitter_types::maybe_grow_stack(|| {
                    <Arg as ::treesitter_types::FromNode>::from_node(child, src)
                })?),
                None => None,
            },
        })
    }
}
impl ::treesitter_types::Spanned for KeywordParameter<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        self.span
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct KeywordPattern<'tree> {
    pub span: ::treesitter_types::Span,
    pub key: KeywordPatternKey<'tree>,
    pub value: ::core::option::Option<PatternExpr<'tree>>,
}
impl<'tree> ::treesitter_types::FromNode<'tree> for KeywordPattern<'tree> {
    #[allow(clippy::match_single_binding, clippy::suspicious_else_formatting)]
    fn from_node(
        node: ::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        debug_assert_eq!(node.kind(), "keyword_pattern");
        Ok(Self {
            span: ::treesitter_types::Span::from(node),
            key: {
                let child = node
                    .child_by_field_name("key")
                    .ok_or_else(|| ::treesitter_types::ParseError::missing_field("key", node))?;
                ::treesitter_types::maybe_grow_stack(|| {
                    <KeywordPatternKey as ::treesitter_types::FromNode>::from_node(child, src)
                })?
            },
            value: match node.child_by_field_name("value") {
                Some(child) => Some(::treesitter_types::maybe_grow_stack(|| {
                    <PatternExpr as ::treesitter_types::FromNode>::from_node(child, src)
                })?),
                None => None,
            },
        })
    }
}
impl ::treesitter_types::Spanned for KeywordPattern<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        self.span
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Lambda<'tree> {
    pub span: ::treesitter_types::Span,
    pub body: LambdaBody<'tree>,
    pub parameters: ::core::option::Option<LambdaParameters<'tree>>,
}
impl<'tree> ::treesitter_types::FromNode<'tree> for Lambda<'tree> {
    #[allow(clippy::match_single_binding, clippy::suspicious_else_formatting)]
    fn from_node(
        node: ::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        debug_assert_eq!(node.kind(), "lambda");
        Ok(Self {
            span: ::treesitter_types::Span::from(node),
            body: {
                let child = node
                    .child_by_field_name("body")
                    .ok_or_else(|| ::treesitter_types::ParseError::missing_field("body", node))?;
                ::treesitter_types::maybe_grow_stack(|| {
                    <LambdaBody as ::treesitter_types::FromNode>::from_node(child, src)
                })?
            },
            parameters: match node.child_by_field_name("parameters") {
                Some(child) => Some(::treesitter_types::maybe_grow_stack(|| {
                    <LambdaParameters as ::treesitter_types::FromNode>::from_node(child, src)
                })?),
                None => None,
            },
        })
    }
}
impl ::treesitter_types::Spanned for Lambda<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        self.span
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct LambdaParameters<'tree> {
    pub span: ::treesitter_types::Span,
    pub children: ::std::vec::Vec<LambdaParametersChildren<'tree>>,
}
impl<'tree> ::treesitter_types::FromNode<'tree> for LambdaParameters<'tree> {
    #[allow(clippy::match_single_binding, clippy::suspicious_else_formatting)]
    fn from_node(
        node: ::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        debug_assert_eq!(node.kind(), "lambda_parameters");
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
                        <LambdaParametersChildren as ::treesitter_types::FromNode>::from_node(
                            child, src,
                        )
                    })?);
                }
                items
            },
        })
    }
}
impl ::treesitter_types::Spanned for LambdaParameters<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        self.span
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct LeftAssignmentList<'tree> {
    pub span: ::treesitter_types::Span,
    pub children: ::std::vec::Vec<LeftAssignmentListChildren<'tree>>,
}
impl<'tree> ::treesitter_types::FromNode<'tree> for LeftAssignmentList<'tree> {
    #[allow(clippy::match_single_binding, clippy::suspicious_else_formatting)]
    fn from_node(
        node: ::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        debug_assert_eq!(node.kind(), "left_assignment_list");
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
                        <LeftAssignmentListChildren as ::treesitter_types::FromNode>::from_node(
                            child, src,
                        )
                    })?);
                }
                items
            },
        })
    }
}
impl ::treesitter_types::Spanned for LeftAssignmentList<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        self.span
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct MatchPattern<'tree> {
    pub span: ::treesitter_types::Span,
    pub pattern: PatternTopExprBody<'tree>,
    pub value: Arg<'tree>,
}
impl<'tree> ::treesitter_types::FromNode<'tree> for MatchPattern<'tree> {
    #[allow(clippy::match_single_binding, clippy::suspicious_else_formatting)]
    fn from_node(
        node: ::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        debug_assert_eq!(node.kind(), "match_pattern");
        Ok(Self {
            span: ::treesitter_types::Span::from(node),
            pattern: {
                let child = node.child_by_field_name("pattern").ok_or_else(|| {
                    ::treesitter_types::ParseError::missing_field("pattern", node)
                })?;
                ::treesitter_types::maybe_grow_stack(|| {
                    <PatternTopExprBody as ::treesitter_types::FromNode>::from_node(child, src)
                })?
            },
            value: {
                let child = node
                    .child_by_field_name("value")
                    .ok_or_else(|| ::treesitter_types::ParseError::missing_field("value", node))?;
                ::treesitter_types::maybe_grow_stack(|| {
                    <Arg as ::treesitter_types::FromNode>::from_node(child, src)
                })?
            },
        })
    }
}
impl ::treesitter_types::Spanned for MatchPattern<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        self.span
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Method<'tree> {
    pub span: ::treesitter_types::Span,
    pub body: ::core::option::Option<MethodBody<'tree>>,
    pub name: MethodName<'tree>,
    pub parameters: ::core::option::Option<MethodParameters<'tree>>,
}
impl<'tree> ::treesitter_types::FromNode<'tree> for Method<'tree> {
    #[allow(clippy::match_single_binding, clippy::suspicious_else_formatting)]
    fn from_node(
        node: ::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        debug_assert_eq!(node.kind(), "method");
        Ok(Self {
            span: ::treesitter_types::Span::from(node),
            body: match node.child_by_field_name("body") {
                Some(child) => Some(::treesitter_types::maybe_grow_stack(|| {
                    <MethodBody as ::treesitter_types::FromNode>::from_node(child, src)
                })?),
                None => None,
            },
            name: {
                let child = node
                    .child_by_field_name("name")
                    .ok_or_else(|| ::treesitter_types::ParseError::missing_field("name", node))?;
                ::treesitter_types::maybe_grow_stack(|| {
                    <MethodName as ::treesitter_types::FromNode>::from_node(child, src)
                })?
            },
            parameters: match node.child_by_field_name("parameters") {
                Some(child) => Some(::treesitter_types::maybe_grow_stack(|| {
                    <MethodParameters as ::treesitter_types::FromNode>::from_node(child, src)
                })?),
                None => None,
            },
        })
    }
}
impl ::treesitter_types::Spanned for Method<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        self.span
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct MethodParameters<'tree> {
    pub span: ::treesitter_types::Span,
    pub children: ::std::vec::Vec<MethodParametersChildren<'tree>>,
}
impl<'tree> ::treesitter_types::FromNode<'tree> for MethodParameters<'tree> {
    #[allow(clippy::match_single_binding, clippy::suspicious_else_formatting)]
    fn from_node(
        node: ::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        debug_assert_eq!(node.kind(), "method_parameters");
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
                        <MethodParametersChildren as ::treesitter_types::FromNode>::from_node(
                            child, src,
                        )
                    })?);
                }
                items
            },
        })
    }
}
impl ::treesitter_types::Spanned for MethodParameters<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        self.span
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Module<'tree> {
    pub span: ::treesitter_types::Span,
    pub body: ::core::option::Option<BodyStatement<'tree>>,
    pub name: ModuleName<'tree>,
}
impl<'tree> ::treesitter_types::FromNode<'tree> for Module<'tree> {
    #[allow(clippy::match_single_binding, clippy::suspicious_else_formatting)]
    fn from_node(
        node: ::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        debug_assert_eq!(node.kind(), "module");
        Ok(Self {
            span: ::treesitter_types::Span::from(node),
            body: match node.child_by_field_name("body") {
                Some(child) => Some(::treesitter_types::maybe_grow_stack(|| {
                    <BodyStatement as ::treesitter_types::FromNode>::from_node(child, src)
                })?),
                None => None,
            },
            name: {
                let child = node
                    .child_by_field_name("name")
                    .ok_or_else(|| ::treesitter_types::ParseError::missing_field("name", node))?;
                ::treesitter_types::maybe_grow_stack(|| {
                    <ModuleName as ::treesitter_types::FromNode>::from_node(child, src)
                })?
            },
        })
    }
}
impl ::treesitter_types::Spanned for Module<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        self.span
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Next<'tree> {
    pub span: ::treesitter_types::Span,
    pub children: ::core::option::Option<ArgumentList<'tree>>,
}
impl<'tree> ::treesitter_types::FromNode<'tree> for Next<'tree> {
    #[allow(clippy::match_single_binding, clippy::suspicious_else_formatting)]
    fn from_node(
        node: ::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        debug_assert_eq!(node.kind(), "next");
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
                    Some(&child) => Some(::treesitter_types::maybe_grow_stack(|| {
                        <ArgumentList as ::treesitter_types::FromNode>::from_node(child, src)
                    })?),
                    None => None,
                }
            },
        })
    }
}
impl ::treesitter_types::Spanned for Next<'_> {
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
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct OperatorAssignment<'tree> {
    pub span: ::treesitter_types::Span,
    pub left: Lhs<'tree>,
    pub operator: OperatorAssignmentOperator,
    pub right: OperatorAssignmentRight<'tree>,
}
impl<'tree> ::treesitter_types::FromNode<'tree> for OperatorAssignment<'tree> {
    #[allow(clippy::match_single_binding, clippy::suspicious_else_formatting)]
    fn from_node(
        node: ::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        debug_assert_eq!(node.kind(), "operator_assignment");
        Ok(Self {
            span: ::treesitter_types::Span::from(node),
            left: {
                let child = node
                    .child_by_field_name("left")
                    .ok_or_else(|| ::treesitter_types::ParseError::missing_field("left", node))?;
                ::treesitter_types::maybe_grow_stack(|| {
                    <Lhs as ::treesitter_types::FromNode>::from_node(child, src)
                })?
            },
            operator: {
                let child = node.child_by_field_name("operator").ok_or_else(|| {
                    ::treesitter_types::ParseError::missing_field("operator", node)
                })?;
                ::treesitter_types::maybe_grow_stack(|| {
                    <OperatorAssignmentOperator as ::treesitter_types::FromNode>::from_node(
                        child, src,
                    )
                })?
            },
            right: {
                let child = node
                    .child_by_field_name("right")
                    .ok_or_else(|| ::treesitter_types::ParseError::missing_field("right", node))?;
                ::treesitter_types::maybe_grow_stack(|| {
                    <OperatorAssignmentRight as ::treesitter_types::FromNode>::from_node(child, src)
                })?
            },
        })
    }
}
impl ::treesitter_types::Spanned for OperatorAssignment<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        self.span
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct OptionalParameter<'tree> {
    pub span: ::treesitter_types::Span,
    pub name: Identifier<'tree>,
    pub value: Arg<'tree>,
}
impl<'tree> ::treesitter_types::FromNode<'tree> for OptionalParameter<'tree> {
    #[allow(clippy::match_single_binding, clippy::suspicious_else_formatting)]
    fn from_node(
        node: ::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        debug_assert_eq!(node.kind(), "optional_parameter");
        Ok(Self {
            span: ::treesitter_types::Span::from(node),
            name: {
                let child = node
                    .child_by_field_name("name")
                    .ok_or_else(|| ::treesitter_types::ParseError::missing_field("name", node))?;
                ::treesitter_types::maybe_grow_stack(|| {
                    <Identifier as ::treesitter_types::FromNode>::from_node(child, src)
                })?
            },
            value: {
                let child = node
                    .child_by_field_name("value")
                    .ok_or_else(|| ::treesitter_types::ParseError::missing_field("value", node))?;
                ::treesitter_types::maybe_grow_stack(|| {
                    <Arg as ::treesitter_types::FromNode>::from_node(child, src)
                })?
            },
        })
    }
}
impl ::treesitter_types::Spanned for OptionalParameter<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        self.span
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Pair<'tree> {
    pub span: ::treesitter_types::Span,
    pub key: PairKey<'tree>,
    pub value: ::core::option::Option<Arg<'tree>>,
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
                    <PairKey as ::treesitter_types::FromNode>::from_node(child, src)
                })?
            },
            value: match node.child_by_field_name("value") {
                Some(child) => Some(::treesitter_types::maybe_grow_stack(|| {
                    <Arg as ::treesitter_types::FromNode>::from_node(child, src)
                })?),
                None => None,
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
pub struct ParenthesizedPattern<'tree> {
    pub span: ::treesitter_types::Span,
    pub children: PatternExpr<'tree>,
}
impl<'tree> ::treesitter_types::FromNode<'tree> for ParenthesizedPattern<'tree> {
    #[allow(clippy::match_single_binding, clippy::suspicious_else_formatting)]
    fn from_node(
        node: ::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        debug_assert_eq!(node.kind(), "parenthesized_pattern");
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
                                        ::treesitter_types::maybe_grow_stack(|| <PatternExpr as ::treesitter_types::FromNode>::from_node(
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
                                            ::treesitter_types::maybe_grow_stack(|| <PatternExpr as ::treesitter_types::FromNode>::from_node(
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
                ::treesitter_types::maybe_grow_stack(|| {
                    <PatternExpr as ::treesitter_types::FromNode>::from_node(child, src)
                })?
            },
        })
    }
}
impl ::treesitter_types::Spanned for ParenthesizedPattern<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        self.span
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ParenthesizedStatements<'tree> {
    pub span: ::treesitter_types::Span,
    pub children: ::std::vec::Vec<ParenthesizedStatementsChildren<'tree>>,
}
impl<'tree> ::treesitter_types::FromNode<'tree> for ParenthesizedStatements<'tree> {
    #[allow(clippy::match_single_binding, clippy::suspicious_else_formatting)]
    fn from_node(
        node: ::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        debug_assert_eq!(node.kind(), "parenthesized_statements");
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
                            ::treesitter_types::maybe_grow_stack(|| <ParenthesizedStatementsChildren as ::treesitter_types::FromNode>::from_node(
                                child,
                                src,
                            ))?,
                        );
                }
                items
            },
        })
    }
}
impl ::treesitter_types::Spanned for ParenthesizedStatements<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        self.span
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Pattern<'tree> {
    pub span: ::treesitter_types::Span,
    pub children: PatternChildren<'tree>,
}
impl<'tree> ::treesitter_types::FromNode<'tree> for Pattern<'tree> {
    #[allow(clippy::match_single_binding, clippy::suspicious_else_formatting)]
    fn from_node(
        node: ::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        debug_assert_eq!(node.kind(), "pattern");
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
                                        ::treesitter_types::maybe_grow_stack(|| <PatternChildren as ::treesitter_types::FromNode>::from_node(
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
                                            ::treesitter_types::maybe_grow_stack(|| <PatternChildren as ::treesitter_types::FromNode>::from_node(
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
                ::treesitter_types::maybe_grow_stack(|| {
                    <PatternChildren as ::treesitter_types::FromNode>::from_node(child, src)
                })?
            },
        })
    }
}
impl ::treesitter_types::Spanned for Pattern<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        self.span
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
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
                    items.push(::treesitter_types::maybe_grow_stack(|| {
                        <ProgramChildren as ::treesitter_types::FromNode>::from_node(child, src)
                    })?);
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
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Range<'tree> {
    pub span: ::treesitter_types::Span,
    pub begin: ::core::option::Option<RangeBegin<'tree>>,
    pub end: ::core::option::Option<RangeEnd<'tree>>,
    pub operator: RangeOperator,
}
impl<'tree> ::treesitter_types::FromNode<'tree> for Range<'tree> {
    #[allow(clippy::match_single_binding, clippy::suspicious_else_formatting)]
    fn from_node(
        node: ::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        debug_assert_eq!(node.kind(), "range");
        Ok(Self {
            span: ::treesitter_types::Span::from(node),
            begin: match node.child_by_field_name("begin") {
                Some(child) => Some(::treesitter_types::maybe_grow_stack(|| {
                    <RangeBegin as ::treesitter_types::FromNode>::from_node(child, src)
                })?),
                None => None,
            },
            end: match node.child_by_field_name("end") {
                Some(child) => Some(::treesitter_types::maybe_grow_stack(|| {
                    <RangeEnd as ::treesitter_types::FromNode>::from_node(child, src)
                })?),
                None => None,
            },
            operator: {
                let child = node.child_by_field_name("operator").ok_or_else(|| {
                    ::treesitter_types::ParseError::missing_field("operator", node)
                })?;
                ::treesitter_types::maybe_grow_stack(|| {
                    <RangeOperator as ::treesitter_types::FromNode>::from_node(child, src)
                })?
            },
        })
    }
}
impl ::treesitter_types::Spanned for Range<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        self.span
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Rational<'tree> {
    pub span: ::treesitter_types::Span,
    pub children: RationalChildren<'tree>,
}
impl<'tree> ::treesitter_types::FromNode<'tree> for Rational<'tree> {
    #[allow(clippy::match_single_binding, clippy::suspicious_else_formatting)]
    fn from_node(
        node: ::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        debug_assert_eq!(node.kind(), "rational");
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
                                        ::treesitter_types::maybe_grow_stack(|| <RationalChildren as ::treesitter_types::FromNode>::from_node(
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
                                            ::treesitter_types::maybe_grow_stack(|| <RationalChildren as ::treesitter_types::FromNode>::from_node(
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
                ::treesitter_types::maybe_grow_stack(|| {
                    <RationalChildren as ::treesitter_types::FromNode>::from_node(child, src)
                })?
            },
        })
    }
}
impl ::treesitter_types::Spanned for Rational<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        self.span
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Redo<'tree> {
    pub span: ::treesitter_types::Span,
    pub children: ::core::option::Option<ArgumentList<'tree>>,
}
impl<'tree> ::treesitter_types::FromNode<'tree> for Redo<'tree> {
    #[allow(clippy::match_single_binding, clippy::suspicious_else_formatting)]
    fn from_node(
        node: ::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        debug_assert_eq!(node.kind(), "redo");
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
                    Some(&child) => Some(::treesitter_types::maybe_grow_stack(|| {
                        <ArgumentList as ::treesitter_types::FromNode>::from_node(child, src)
                    })?),
                    None => None,
                }
            },
        })
    }
}
impl ::treesitter_types::Spanned for Redo<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        self.span
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Regex<'tree> {
    pub span: ::treesitter_types::Span,
    pub children: ::std::vec::Vec<RegexChildren<'tree>>,
}
impl<'tree> ::treesitter_types::FromNode<'tree> for Regex<'tree> {
    #[allow(clippy::match_single_binding, clippy::suspicious_else_formatting)]
    fn from_node(
        node: ::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        debug_assert_eq!(node.kind(), "regex");
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
                        <RegexChildren as ::treesitter_types::FromNode>::from_node(child, src)
                    })?);
                }
                items
            },
        })
    }
}
impl ::treesitter_types::Spanned for Regex<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        self.span
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Rescue<'tree> {
    pub span: ::treesitter_types::Span,
    pub body: ::core::option::Option<Then<'tree>>,
    pub exceptions: ::core::option::Option<Exceptions<'tree>>,
    pub variable: ::core::option::Option<ExceptionVariable<'tree>>,
}
impl<'tree> ::treesitter_types::FromNode<'tree> for Rescue<'tree> {
    #[allow(clippy::match_single_binding, clippy::suspicious_else_formatting)]
    fn from_node(
        node: ::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        debug_assert_eq!(node.kind(), "rescue");
        Ok(Self {
            span: ::treesitter_types::Span::from(node),
            body: match node.child_by_field_name("body") {
                Some(child) => Some(::treesitter_types::maybe_grow_stack(|| {
                    <Then as ::treesitter_types::FromNode>::from_node(child, src)
                })?),
                None => None,
            },
            exceptions: match node.child_by_field_name("exceptions") {
                Some(child) => Some(::treesitter_types::maybe_grow_stack(|| {
                    <Exceptions as ::treesitter_types::FromNode>::from_node(child, src)
                })?),
                None => None,
            },
            variable: match node.child_by_field_name("variable") {
                Some(child) => Some(::treesitter_types::maybe_grow_stack(|| {
                    <ExceptionVariable as ::treesitter_types::FromNode>::from_node(child, src)
                })?),
                None => None,
            },
        })
    }
}
impl ::treesitter_types::Spanned for Rescue<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        self.span
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct RescueModifier<'tree> {
    pub span: ::treesitter_types::Span,
    pub body: RescueModifierBody<'tree>,
    pub handler: Expression<'tree>,
}
impl<'tree> ::treesitter_types::FromNode<'tree> for RescueModifier<'tree> {
    #[allow(clippy::match_single_binding, clippy::suspicious_else_formatting)]
    fn from_node(
        node: ::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        debug_assert_eq!(node.kind(), "rescue_modifier");
        Ok(Self {
            span: ::treesitter_types::Span::from(node),
            body: {
                let child = node
                    .child_by_field_name("body")
                    .ok_or_else(|| ::treesitter_types::ParseError::missing_field("body", node))?;
                ::treesitter_types::maybe_grow_stack(|| {
                    <RescueModifierBody as ::treesitter_types::FromNode>::from_node(child, src)
                })?
            },
            handler: {
                let child = node.child_by_field_name("handler").ok_or_else(|| {
                    ::treesitter_types::ParseError::missing_field("handler", node)
                })?;
                ::treesitter_types::maybe_grow_stack(|| {
                    <Expression as ::treesitter_types::FromNode>::from_node(child, src)
                })?
            },
        })
    }
}
impl ::treesitter_types::Spanned for RescueModifier<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        self.span
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct RestAssignment<'tree> {
    pub span: ::treesitter_types::Span,
    pub children: ::core::option::Option<Lhs<'tree>>,
}
impl<'tree> ::treesitter_types::FromNode<'tree> for RestAssignment<'tree> {
    #[allow(clippy::match_single_binding, clippy::suspicious_else_formatting)]
    fn from_node(
        node: ::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        debug_assert_eq!(node.kind(), "rest_assignment");
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
                    Some(&child) => Some(::treesitter_types::maybe_grow_stack(|| {
                        <Lhs as ::treesitter_types::FromNode>::from_node(child, src)
                    })?),
                    None => None,
                }
            },
        })
    }
}
impl ::treesitter_types::Spanned for RestAssignment<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        self.span
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Retry<'tree> {
    pub span: ::treesitter_types::Span,
    pub children: ::core::option::Option<ArgumentList<'tree>>,
}
impl<'tree> ::treesitter_types::FromNode<'tree> for Retry<'tree> {
    #[allow(clippy::match_single_binding, clippy::suspicious_else_formatting)]
    fn from_node(
        node: ::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        debug_assert_eq!(node.kind(), "retry");
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
                    Some(&child) => Some(::treesitter_types::maybe_grow_stack(|| {
                        <ArgumentList as ::treesitter_types::FromNode>::from_node(child, src)
                    })?),
                    None => None,
                }
            },
        })
    }
}
impl ::treesitter_types::Spanned for Retry<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        self.span
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Return<'tree> {
    pub span: ::treesitter_types::Span,
    pub children: ::core::option::Option<ArgumentList<'tree>>,
}
impl<'tree> ::treesitter_types::FromNode<'tree> for Return<'tree> {
    #[allow(clippy::match_single_binding, clippy::suspicious_else_formatting)]
    fn from_node(
        node: ::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        debug_assert_eq!(node.kind(), "return");
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
                    Some(&child) => Some(::treesitter_types::maybe_grow_stack(|| {
                        <ArgumentList as ::treesitter_types::FromNode>::from_node(child, src)
                    })?),
                    None => None,
                }
            },
        })
    }
}
impl ::treesitter_types::Spanned for Return<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        self.span
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct RightAssignmentList<'tree> {
    pub span: ::treesitter_types::Span,
    pub children: ::std::vec::Vec<RightAssignmentListChildren<'tree>>,
}
impl<'tree> ::treesitter_types::FromNode<'tree> for RightAssignmentList<'tree> {
    #[allow(clippy::match_single_binding, clippy::suspicious_else_formatting)]
    fn from_node(
        node: ::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        debug_assert_eq!(node.kind(), "right_assignment_list");
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
                        <RightAssignmentListChildren as ::treesitter_types::FromNode>::from_node(
                            child, src,
                        )
                    })?);
                }
                items
            },
        })
    }
}
impl ::treesitter_types::Spanned for RightAssignmentList<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        self.span
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ScopeResolution<'tree> {
    pub span: ::treesitter_types::Span,
    pub name: Constant<'tree>,
    pub scope: ::core::option::Option<ScopeResolutionScope<'tree>>,
}
impl<'tree> ::treesitter_types::FromNode<'tree> for ScopeResolution<'tree> {
    #[allow(clippy::match_single_binding, clippy::suspicious_else_formatting)]
    fn from_node(
        node: ::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        debug_assert_eq!(node.kind(), "scope_resolution");
        Ok(Self {
            span: ::treesitter_types::Span::from(node),
            name: {
                let child = node
                    .child_by_field_name("name")
                    .ok_or_else(|| ::treesitter_types::ParseError::missing_field("name", node))?;
                ::treesitter_types::maybe_grow_stack(|| {
                    <Constant as ::treesitter_types::FromNode>::from_node(child, src)
                })?
            },
            scope: match node.child_by_field_name("scope") {
                Some(child) => Some(::treesitter_types::maybe_grow_stack(|| {
                    <ScopeResolutionScope as ::treesitter_types::FromNode>::from_node(child, src)
                })?),
                None => None,
            },
        })
    }
}
impl ::treesitter_types::Spanned for ScopeResolution<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        self.span
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Setter<'tree> {
    pub span: ::treesitter_types::Span,
    pub name: Identifier<'tree>,
}
impl<'tree> ::treesitter_types::FromNode<'tree> for Setter<'tree> {
    #[allow(clippy::match_single_binding, clippy::suspicious_else_formatting)]
    fn from_node(
        node: ::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        debug_assert_eq!(node.kind(), "setter");
        Ok(Self {
            span: ::treesitter_types::Span::from(node),
            name: {
                let child = node
                    .child_by_field_name("name")
                    .ok_or_else(|| ::treesitter_types::ParseError::missing_field("name", node))?;
                ::treesitter_types::maybe_grow_stack(|| {
                    <Identifier as ::treesitter_types::FromNode>::from_node(child, src)
                })?
            },
        })
    }
}
impl ::treesitter_types::Spanned for Setter<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        self.span
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SingletonClass<'tree> {
    pub span: ::treesitter_types::Span,
    pub body: ::core::option::Option<BodyStatement<'tree>>,
    pub value: Arg<'tree>,
}
impl<'tree> ::treesitter_types::FromNode<'tree> for SingletonClass<'tree> {
    #[allow(clippy::match_single_binding, clippy::suspicious_else_formatting)]
    fn from_node(
        node: ::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        debug_assert_eq!(node.kind(), "singleton_class");
        Ok(Self {
            span: ::treesitter_types::Span::from(node),
            body: match node.child_by_field_name("body") {
                Some(child) => Some(::treesitter_types::maybe_grow_stack(|| {
                    <BodyStatement as ::treesitter_types::FromNode>::from_node(child, src)
                })?),
                None => None,
            },
            value: {
                let child = node
                    .child_by_field_name("value")
                    .ok_or_else(|| ::treesitter_types::ParseError::missing_field("value", node))?;
                ::treesitter_types::maybe_grow_stack(|| {
                    <Arg as ::treesitter_types::FromNode>::from_node(child, src)
                })?
            },
        })
    }
}
impl ::treesitter_types::Spanned for SingletonClass<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        self.span
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SingletonMethod<'tree> {
    pub span: ::treesitter_types::Span,
    pub body: ::core::option::Option<SingletonMethodBody<'tree>>,
    pub name: MethodName<'tree>,
    pub object: SingletonMethodObject<'tree>,
    pub parameters: ::core::option::Option<MethodParameters<'tree>>,
}
impl<'tree> ::treesitter_types::FromNode<'tree> for SingletonMethod<'tree> {
    #[allow(clippy::match_single_binding, clippy::suspicious_else_formatting)]
    fn from_node(
        node: ::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        debug_assert_eq!(node.kind(), "singleton_method");
        Ok(Self {
            span: ::treesitter_types::Span::from(node),
            body: match node.child_by_field_name("body") {
                Some(child) => Some(::treesitter_types::maybe_grow_stack(|| {
                    <SingletonMethodBody as ::treesitter_types::FromNode>::from_node(child, src)
                })?),
                None => None,
            },
            name: {
                let child = node
                    .child_by_field_name("name")
                    .ok_or_else(|| ::treesitter_types::ParseError::missing_field("name", node))?;
                ::treesitter_types::maybe_grow_stack(|| {
                    <MethodName as ::treesitter_types::FromNode>::from_node(child, src)
                })?
            },
            object: {
                let child = node
                    .child_by_field_name("object")
                    .ok_or_else(|| ::treesitter_types::ParseError::missing_field("object", node))?;
                ::treesitter_types::maybe_grow_stack(|| {
                    <SingletonMethodObject as ::treesitter_types::FromNode>::from_node(child, src)
                })?
            },
            parameters: match node.child_by_field_name("parameters") {
                Some(child) => Some(::treesitter_types::maybe_grow_stack(|| {
                    <MethodParameters as ::treesitter_types::FromNode>::from_node(child, src)
                })?),
                None => None,
            },
        })
    }
}
impl ::treesitter_types::Spanned for SingletonMethod<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        self.span
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SplatArgument<'tree> {
    pub span: ::treesitter_types::Span,
    pub children: ::core::option::Option<Arg<'tree>>,
}
impl<'tree> ::treesitter_types::FromNode<'tree> for SplatArgument<'tree> {
    #[allow(clippy::match_single_binding, clippy::suspicious_else_formatting)]
    fn from_node(
        node: ::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        debug_assert_eq!(node.kind(), "splat_argument");
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
                    Some(&child) => Some(::treesitter_types::maybe_grow_stack(|| {
                        <Arg as ::treesitter_types::FromNode>::from_node(child, src)
                    })?),
                    None => None,
                }
            },
        })
    }
}
impl ::treesitter_types::Spanned for SplatArgument<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        self.span
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SplatParameter<'tree> {
    pub span: ::treesitter_types::Span,
    pub name: ::core::option::Option<Identifier<'tree>>,
}
impl<'tree> ::treesitter_types::FromNode<'tree> for SplatParameter<'tree> {
    #[allow(clippy::match_single_binding, clippy::suspicious_else_formatting)]
    fn from_node(
        node: ::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        debug_assert_eq!(node.kind(), "splat_parameter");
        Ok(Self {
            span: ::treesitter_types::Span::from(node),
            name: match node.child_by_field_name("name") {
                Some(child) => Some(::treesitter_types::maybe_grow_stack(|| {
                    <Identifier as ::treesitter_types::FromNode>::from_node(child, src)
                })?),
                None => None,
            },
        })
    }
}
impl ::treesitter_types::Spanned for SplatParameter<'_> {
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
pub struct StringArray<'tree> {
    pub span: ::treesitter_types::Span,
    pub children: ::std::vec::Vec<BareString<'tree>>,
}
impl<'tree> ::treesitter_types::FromNode<'tree> for StringArray<'tree> {
    #[allow(clippy::match_single_binding, clippy::suspicious_else_formatting)]
    fn from_node(
        node: ::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        debug_assert_eq!(node.kind(), "string_array");
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
                        <BareString as ::treesitter_types::FromNode>::from_node(child, src)
                    })?);
                }
                items
            },
        })
    }
}
impl ::treesitter_types::Spanned for StringArray<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        self.span
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Subshell<'tree> {
    pub span: ::treesitter_types::Span,
    pub children: ::std::vec::Vec<SubshellChildren<'tree>>,
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
                    items.push(::treesitter_types::maybe_grow_stack(|| {
                        <SubshellChildren as ::treesitter_types::FromNode>::from_node(child, src)
                    })?);
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
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Superclass<'tree> {
    pub span: ::treesitter_types::Span,
    pub children: Expression<'tree>,
}
impl<'tree> ::treesitter_types::FromNode<'tree> for Superclass<'tree> {
    #[allow(clippy::match_single_binding, clippy::suspicious_else_formatting)]
    fn from_node(
        node: ::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        debug_assert_eq!(node.kind(), "superclass");
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
                                        ::treesitter_types::maybe_grow_stack(|| <Expression as ::treesitter_types::FromNode>::from_node(
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
                                            ::treesitter_types::maybe_grow_stack(|| <Expression as ::treesitter_types::FromNode>::from_node(
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
                ::treesitter_types::maybe_grow_stack(|| {
                    <Expression as ::treesitter_types::FromNode>::from_node(child, src)
                })?
            },
        })
    }
}
impl ::treesitter_types::Spanned for Superclass<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        self.span
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SymbolArray<'tree> {
    pub span: ::treesitter_types::Span,
    pub children: ::std::vec::Vec<BareSymbol<'tree>>,
}
impl<'tree> ::treesitter_types::FromNode<'tree> for SymbolArray<'tree> {
    #[allow(clippy::match_single_binding, clippy::suspicious_else_formatting)]
    fn from_node(
        node: ::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        debug_assert_eq!(node.kind(), "symbol_array");
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
                        <BareSymbol as ::treesitter_types::FromNode>::from_node(child, src)
                    })?);
                }
                items
            },
        })
    }
}
impl ::treesitter_types::Spanned for SymbolArray<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        self.span
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct TestPattern<'tree> {
    pub span: ::treesitter_types::Span,
    pub pattern: PatternTopExprBody<'tree>,
    pub value: Arg<'tree>,
}
impl<'tree> ::treesitter_types::FromNode<'tree> for TestPattern<'tree> {
    #[allow(clippy::match_single_binding, clippy::suspicious_else_formatting)]
    fn from_node(
        node: ::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        debug_assert_eq!(node.kind(), "test_pattern");
        Ok(Self {
            span: ::treesitter_types::Span::from(node),
            pattern: {
                let child = node.child_by_field_name("pattern").ok_or_else(|| {
                    ::treesitter_types::ParseError::missing_field("pattern", node)
                })?;
                ::treesitter_types::maybe_grow_stack(|| {
                    <PatternTopExprBody as ::treesitter_types::FromNode>::from_node(child, src)
                })?
            },
            value: {
                let child = node
                    .child_by_field_name("value")
                    .ok_or_else(|| ::treesitter_types::ParseError::missing_field("value", node))?;
                ::treesitter_types::maybe_grow_stack(|| {
                    <Arg as ::treesitter_types::FromNode>::from_node(child, src)
                })?
            },
        })
    }
}
impl ::treesitter_types::Spanned for TestPattern<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        self.span
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Then<'tree> {
    pub span: ::treesitter_types::Span,
    pub children: ::std::vec::Vec<ThenChildren<'tree>>,
}
impl<'tree> ::treesitter_types::FromNode<'tree> for Then<'tree> {
    #[allow(clippy::match_single_binding, clippy::suspicious_else_formatting)]
    fn from_node(
        node: ::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        debug_assert_eq!(node.kind(), "then");
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
                        <ThenChildren as ::treesitter_types::FromNode>::from_node(child, src)
                    })?);
                }
                items
            },
        })
    }
}
impl ::treesitter_types::Spanned for Then<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        self.span
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Unary<'tree> {
    pub span: ::treesitter_types::Span,
    pub operand: UnaryOperand<'tree>,
    pub operator: UnaryOperator,
}
impl<'tree> ::treesitter_types::FromNode<'tree> for Unary<'tree> {
    #[allow(clippy::match_single_binding, clippy::suspicious_else_formatting)]
    fn from_node(
        node: ::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        debug_assert_eq!(node.kind(), "unary");
        Ok(Self {
            span: ::treesitter_types::Span::from(node),
            operand: {
                let child = node.child_by_field_name("operand").ok_or_else(|| {
                    ::treesitter_types::ParseError::missing_field("operand", node)
                })?;
                ::treesitter_types::maybe_grow_stack(|| {
                    <UnaryOperand as ::treesitter_types::FromNode>::from_node(child, src)
                })?
            },
            operator: {
                let child = node.child_by_field_name("operator").ok_or_else(|| {
                    ::treesitter_types::ParseError::missing_field("operator", node)
                })?;
                ::treesitter_types::maybe_grow_stack(|| {
                    <UnaryOperator as ::treesitter_types::FromNode>::from_node(child, src)
                })?
            },
        })
    }
}
impl ::treesitter_types::Spanned for Unary<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        self.span
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Undef<'tree> {
    pub span: ::treesitter_types::Span,
    pub children: ::std::vec::Vec<MethodName<'tree>>,
}
impl<'tree> ::treesitter_types::FromNode<'tree> for Undef<'tree> {
    #[allow(clippy::match_single_binding, clippy::suspicious_else_formatting)]
    fn from_node(
        node: ::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        debug_assert_eq!(node.kind(), "undef");
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
                        <MethodName as ::treesitter_types::FromNode>::from_node(child, src)
                    })?);
                }
                items
            },
        })
    }
}
impl ::treesitter_types::Spanned for Undef<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        self.span
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Unless<'tree> {
    pub span: ::treesitter_types::Span,
    pub alternative: ::core::option::Option<UnlessAlternative<'tree>>,
    pub condition: Statement<'tree>,
    pub consequence: ::core::option::Option<Then<'tree>>,
}
impl<'tree> ::treesitter_types::FromNode<'tree> for Unless<'tree> {
    #[allow(clippy::match_single_binding, clippy::suspicious_else_formatting)]
    fn from_node(
        node: ::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        debug_assert_eq!(node.kind(), "unless");
        Ok(Self {
            span: ::treesitter_types::Span::from(node),
            alternative: match node.child_by_field_name("alternative") {
                Some(child) => Some(::treesitter_types::maybe_grow_stack(|| {
                    <UnlessAlternative as ::treesitter_types::FromNode>::from_node(child, src)
                })?),
                None => None,
            },
            condition: {
                let child = node.child_by_field_name("condition").ok_or_else(|| {
                    ::treesitter_types::ParseError::missing_field("condition", node)
                })?;
                ::treesitter_types::maybe_grow_stack(|| {
                    <Statement as ::treesitter_types::FromNode>::from_node(child, src)
                })?
            },
            consequence: match node.child_by_field_name("consequence") {
                Some(child) => Some(::treesitter_types::maybe_grow_stack(|| {
                    <Then as ::treesitter_types::FromNode>::from_node(child, src)
                })?),
                None => None,
            },
        })
    }
}
impl ::treesitter_types::Spanned for Unless<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        self.span
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct UnlessGuard<'tree> {
    pub span: ::treesitter_types::Span,
    pub condition: Expression<'tree>,
}
impl<'tree> ::treesitter_types::FromNode<'tree> for UnlessGuard<'tree> {
    #[allow(clippy::match_single_binding, clippy::suspicious_else_formatting)]
    fn from_node(
        node: ::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        debug_assert_eq!(node.kind(), "unless_guard");
        Ok(Self {
            span: ::treesitter_types::Span::from(node),
            condition: {
                let child = node.child_by_field_name("condition").ok_or_else(|| {
                    ::treesitter_types::ParseError::missing_field("condition", node)
                })?;
                ::treesitter_types::maybe_grow_stack(|| {
                    <Expression as ::treesitter_types::FromNode>::from_node(child, src)
                })?
            },
        })
    }
}
impl ::treesitter_types::Spanned for UnlessGuard<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        self.span
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct UnlessModifier<'tree> {
    pub span: ::treesitter_types::Span,
    pub body: Statement<'tree>,
    pub condition: Expression<'tree>,
}
impl<'tree> ::treesitter_types::FromNode<'tree> for UnlessModifier<'tree> {
    #[allow(clippy::match_single_binding, clippy::suspicious_else_formatting)]
    fn from_node(
        node: ::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        debug_assert_eq!(node.kind(), "unless_modifier");
        Ok(Self {
            span: ::treesitter_types::Span::from(node),
            body: {
                let child = node
                    .child_by_field_name("body")
                    .ok_or_else(|| ::treesitter_types::ParseError::missing_field("body", node))?;
                ::treesitter_types::maybe_grow_stack(|| {
                    <Statement as ::treesitter_types::FromNode>::from_node(child, src)
                })?
            },
            condition: {
                let child = node.child_by_field_name("condition").ok_or_else(|| {
                    ::treesitter_types::ParseError::missing_field("condition", node)
                })?;
                ::treesitter_types::maybe_grow_stack(|| {
                    <Expression as ::treesitter_types::FromNode>::from_node(child, src)
                })?
            },
        })
    }
}
impl ::treesitter_types::Spanned for UnlessModifier<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        self.span
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Until<'tree> {
    pub span: ::treesitter_types::Span,
    pub body: Do<'tree>,
    pub condition: Statement<'tree>,
}
impl<'tree> ::treesitter_types::FromNode<'tree> for Until<'tree> {
    #[allow(clippy::match_single_binding, clippy::suspicious_else_formatting)]
    fn from_node(
        node: ::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        debug_assert_eq!(node.kind(), "until");
        Ok(Self {
            span: ::treesitter_types::Span::from(node),
            body: {
                let child = node
                    .child_by_field_name("body")
                    .ok_or_else(|| ::treesitter_types::ParseError::missing_field("body", node))?;
                ::treesitter_types::maybe_grow_stack(|| {
                    <Do as ::treesitter_types::FromNode>::from_node(child, src)
                })?
            },
            condition: {
                let child = node.child_by_field_name("condition").ok_or_else(|| {
                    ::treesitter_types::ParseError::missing_field("condition", node)
                })?;
                ::treesitter_types::maybe_grow_stack(|| {
                    <Statement as ::treesitter_types::FromNode>::from_node(child, src)
                })?
            },
        })
    }
}
impl ::treesitter_types::Spanned for Until<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        self.span
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct UntilModifier<'tree> {
    pub span: ::treesitter_types::Span,
    pub body: Statement<'tree>,
    pub condition: Expression<'tree>,
}
impl<'tree> ::treesitter_types::FromNode<'tree> for UntilModifier<'tree> {
    #[allow(clippy::match_single_binding, clippy::suspicious_else_formatting)]
    fn from_node(
        node: ::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        debug_assert_eq!(node.kind(), "until_modifier");
        Ok(Self {
            span: ::treesitter_types::Span::from(node),
            body: {
                let child = node
                    .child_by_field_name("body")
                    .ok_or_else(|| ::treesitter_types::ParseError::missing_field("body", node))?;
                ::treesitter_types::maybe_grow_stack(|| {
                    <Statement as ::treesitter_types::FromNode>::from_node(child, src)
                })?
            },
            condition: {
                let child = node.child_by_field_name("condition").ok_or_else(|| {
                    ::treesitter_types::ParseError::missing_field("condition", node)
                })?;
                ::treesitter_types::maybe_grow_stack(|| {
                    <Expression as ::treesitter_types::FromNode>::from_node(child, src)
                })?
            },
        })
    }
}
impl ::treesitter_types::Spanned for UntilModifier<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        self.span
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct VariableReferencePattern<'tree> {
    pub span: ::treesitter_types::Span,
    pub name: VariableReferencePatternName<'tree>,
}
impl<'tree> ::treesitter_types::FromNode<'tree> for VariableReferencePattern<'tree> {
    #[allow(clippy::match_single_binding, clippy::suspicious_else_formatting)]
    fn from_node(
        node: ::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        debug_assert_eq!(node.kind(), "variable_reference_pattern");
        Ok(Self {
            span: ::treesitter_types::Span::from(node),
            name: {
                let child = node
                    .child_by_field_name("name")
                    .ok_or_else(|| ::treesitter_types::ParseError::missing_field("name", node))?;
                ::treesitter_types::maybe_grow_stack(|| {
                    <VariableReferencePatternName as ::treesitter_types::FromNode>::from_node(
                        child, src,
                    )
                })?
            },
        })
    }
}
impl ::treesitter_types::Spanned for VariableReferencePattern<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        self.span
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct When<'tree> {
    pub span: ::treesitter_types::Span,
    pub body: ::core::option::Option<Then<'tree>>,
    pub pattern: ::std::vec::Vec<Pattern<'tree>>,
}
impl<'tree> ::treesitter_types::FromNode<'tree> for When<'tree> {
    #[allow(clippy::match_single_binding, clippy::suspicious_else_formatting)]
    fn from_node(
        node: ::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        debug_assert_eq!(node.kind(), "when");
        Ok(Self {
            span: ::treesitter_types::Span::from(node),
            body: match node.child_by_field_name("body") {
                Some(child) => Some(::treesitter_types::maybe_grow_stack(|| {
                    <Then as ::treesitter_types::FromNode>::from_node(child, src)
                })?),
                None => None,
            },
            pattern: {
                let mut cursor = node.walk();
                let mut items = ::std::vec::Vec::new();
                for child in node.children_by_field_name("pattern", &mut cursor) {
                    items.push(::treesitter_types::maybe_grow_stack(|| {
                        <Pattern as ::treesitter_types::FromNode>::from_node(child, src)
                    })?);
                }
                items
            },
        })
    }
}
impl ::treesitter_types::Spanned for When<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        self.span
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct While<'tree> {
    pub span: ::treesitter_types::Span,
    pub body: Do<'tree>,
    pub condition: Statement<'tree>,
}
impl<'tree> ::treesitter_types::FromNode<'tree> for While<'tree> {
    #[allow(clippy::match_single_binding, clippy::suspicious_else_formatting)]
    fn from_node(
        node: ::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        debug_assert_eq!(node.kind(), "while");
        Ok(Self {
            span: ::treesitter_types::Span::from(node),
            body: {
                let child = node
                    .child_by_field_name("body")
                    .ok_or_else(|| ::treesitter_types::ParseError::missing_field("body", node))?;
                ::treesitter_types::maybe_grow_stack(|| {
                    <Do as ::treesitter_types::FromNode>::from_node(child, src)
                })?
            },
            condition: {
                let child = node.child_by_field_name("condition").ok_or_else(|| {
                    ::treesitter_types::ParseError::missing_field("condition", node)
                })?;
                ::treesitter_types::maybe_grow_stack(|| {
                    <Statement as ::treesitter_types::FromNode>::from_node(child, src)
                })?
            },
        })
    }
}
impl ::treesitter_types::Spanned for While<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        self.span
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct WhileModifier<'tree> {
    pub span: ::treesitter_types::Span,
    pub body: Statement<'tree>,
    pub condition: Expression<'tree>,
}
impl<'tree> ::treesitter_types::FromNode<'tree> for WhileModifier<'tree> {
    #[allow(clippy::match_single_binding, clippy::suspicious_else_formatting)]
    fn from_node(
        node: ::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        debug_assert_eq!(node.kind(), "while_modifier");
        Ok(Self {
            span: ::treesitter_types::Span::from(node),
            body: {
                let child = node
                    .child_by_field_name("body")
                    .ok_or_else(|| ::treesitter_types::ParseError::missing_field("body", node))?;
                ::treesitter_types::maybe_grow_stack(|| {
                    <Statement as ::treesitter_types::FromNode>::from_node(child, src)
                })?
            },
            condition: {
                let child = node.child_by_field_name("condition").ok_or_else(|| {
                    ::treesitter_types::ParseError::missing_field("condition", node)
                })?;
                ::treesitter_types::maybe_grow_stack(|| {
                    <Expression as ::treesitter_types::FromNode>::from_node(child, src)
                })?
            },
        })
    }
}
impl ::treesitter_types::Spanned for WhileModifier<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        self.span
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Yield<'tree> {
    pub span: ::treesitter_types::Span,
    pub children: ::core::option::Option<ArgumentList<'tree>>,
}
impl<'tree> ::treesitter_types::FromNode<'tree> for Yield<'tree> {
    #[allow(clippy::match_single_binding, clippy::suspicious_else_formatting)]
    fn from_node(
        node: ::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        debug_assert_eq!(node.kind(), "yield");
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
                    Some(&child) => Some(::treesitter_types::maybe_grow_stack(|| {
                        <ArgumentList as ::treesitter_types::FromNode>::from_node(child, src)
                    })?),
                    None => None,
                }
            },
        })
    }
}
impl ::treesitter_types::Spanned for Yield<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        self.span
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Character<'tree> {
    pub span: ::treesitter_types::Span,
    text: &'tree str,
}
impl<'tree> ::treesitter_types::FromNode<'tree> for Character<'tree> {
    fn from_node(
        node: ::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        debug_assert_eq!(node.kind(), "character");
        Ok(Self {
            span: ::treesitter_types::Span::from(node),
            text: node.utf8_text(src)?,
        })
    }
}
impl<'tree> ::treesitter_types::LeafNode<'tree> for Character<'tree> {
    fn text(&self) -> &'tree str {
        self.text
    }
}
impl ::treesitter_types::Spanned for Character<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        self.span
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ClassVariable<'tree> {
    pub span: ::treesitter_types::Span,
    text: &'tree str,
}
impl<'tree> ::treesitter_types::FromNode<'tree> for ClassVariable<'tree> {
    fn from_node(
        node: ::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        debug_assert_eq!(node.kind(), "class_variable");
        Ok(Self {
            span: ::treesitter_types::Span::from(node),
            text: node.utf8_text(src)?,
        })
    }
}
impl<'tree> ::treesitter_types::LeafNode<'tree> for ClassVariable<'tree> {
    fn text(&self) -> &'tree str {
        self.text
    }
}
impl ::treesitter_types::Spanned for ClassVariable<'_> {
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
pub struct Encoding<'tree> {
    pub span: ::treesitter_types::Span,
    text: &'tree str,
}
impl<'tree> ::treesitter_types::FromNode<'tree> for Encoding<'tree> {
    fn from_node(
        node: ::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        debug_assert_eq!(node.kind(), "encoding");
        Ok(Self {
            span: ::treesitter_types::Span::from(node),
            text: node.utf8_text(src)?,
        })
    }
}
impl<'tree> ::treesitter_types::LeafNode<'tree> for Encoding<'tree> {
    fn text(&self) -> &'tree str {
        self.text
    }
}
impl ::treesitter_types::Spanned for Encoding<'_> {
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
pub struct File<'tree> {
    pub span: ::treesitter_types::Span,
    text: &'tree str,
}
impl<'tree> ::treesitter_types::FromNode<'tree> for File<'tree> {
    fn from_node(
        node: ::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        debug_assert_eq!(node.kind(), "file");
        Ok(Self {
            span: ::treesitter_types::Span::from(node),
            text: node.utf8_text(src)?,
        })
    }
}
impl<'tree> ::treesitter_types::LeafNode<'tree> for File<'tree> {
    fn text(&self) -> &'tree str {
        self.text
    }
}
impl ::treesitter_types::Spanned for File<'_> {
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
pub struct GlobalVariable<'tree> {
    pub span: ::treesitter_types::Span,
    text: &'tree str,
}
impl<'tree> ::treesitter_types::FromNode<'tree> for GlobalVariable<'tree> {
    fn from_node(
        node: ::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        debug_assert_eq!(node.kind(), "global_variable");
        Ok(Self {
            span: ::treesitter_types::Span::from(node),
            text: node.utf8_text(src)?,
        })
    }
}
impl<'tree> ::treesitter_types::LeafNode<'tree> for GlobalVariable<'tree> {
    fn text(&self) -> &'tree str {
        self.text
    }
}
impl ::treesitter_types::Spanned for GlobalVariable<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        self.span
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct HeredocBeginning<'tree> {
    pub span: ::treesitter_types::Span,
    text: &'tree str,
}
impl<'tree> ::treesitter_types::FromNode<'tree> for HeredocBeginning<'tree> {
    fn from_node(
        node: ::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        debug_assert_eq!(node.kind(), "heredoc_beginning");
        Ok(Self {
            span: ::treesitter_types::Span::from(node),
            text: node.utf8_text(src)?,
        })
    }
}
impl<'tree> ::treesitter_types::LeafNode<'tree> for HeredocBeginning<'tree> {
    fn text(&self) -> &'tree str {
        self.text
    }
}
impl ::treesitter_types::Spanned for HeredocBeginning<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        self.span
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
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
#[derive(Debug, Clone, PartialEq, Eq)]
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
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct InstanceVariable<'tree> {
    pub span: ::treesitter_types::Span,
    text: &'tree str,
}
impl<'tree> ::treesitter_types::FromNode<'tree> for InstanceVariable<'tree> {
    fn from_node(
        node: ::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        debug_assert_eq!(node.kind(), "instance_variable");
        Ok(Self {
            span: ::treesitter_types::Span::from(node),
            text: node.utf8_text(src)?,
        })
    }
}
impl<'tree> ::treesitter_types::LeafNode<'tree> for InstanceVariable<'tree> {
    fn text(&self) -> &'tree str {
        self.text
    }
}
impl ::treesitter_types::Spanned for InstanceVariable<'_> {
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
pub struct Line<'tree> {
    pub span: ::treesitter_types::Span,
    text: &'tree str,
}
impl<'tree> ::treesitter_types::FromNode<'tree> for Line<'tree> {
    fn from_node(
        node: ::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        debug_assert_eq!(node.kind(), "line");
        Ok(Self {
            span: ::treesitter_types::Span::from(node),
            text: node.utf8_text(src)?,
        })
    }
}
impl<'tree> ::treesitter_types::LeafNode<'tree> for Line<'tree> {
    fn text(&self) -> &'tree str {
        self.text
    }
}
impl ::treesitter_types::Spanned for Line<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        self.span
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SelfType<'tree> {
    pub span: ::treesitter_types::Span,
    text: &'tree str,
}
impl<'tree> ::treesitter_types::FromNode<'tree> for SelfType<'tree> {
    fn from_node(
        node: ::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        debug_assert_eq!(node.kind(), "self");
        Ok(Self {
            span: ::treesitter_types::Span::from(node),
            text: node.utf8_text(src)?,
        })
    }
}
impl<'tree> ::treesitter_types::LeafNode<'tree> for SelfType<'tree> {
    fn text(&self) -> &'tree str {
        self.text
    }
}
impl ::treesitter_types::Spanned for SelfType<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        self.span
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SimpleSymbol<'tree> {
    pub span: ::treesitter_types::Span,
    text: &'tree str,
}
impl<'tree> ::treesitter_types::FromNode<'tree> for SimpleSymbol<'tree> {
    fn from_node(
        node: ::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        debug_assert_eq!(node.kind(), "simple_symbol");
        Ok(Self {
            span: ::treesitter_types::Span::from(node),
            text: node.utf8_text(src)?,
        })
    }
}
impl<'tree> ::treesitter_types::LeafNode<'tree> for SimpleSymbol<'tree> {
    fn text(&self) -> &'tree str {
        self.text
    }
}
impl ::treesitter_types::Spanned for SimpleSymbol<'_> {
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
pub struct Super<'tree> {
    pub span: ::treesitter_types::Span,
    text: &'tree str,
}
impl<'tree> ::treesitter_types::FromNode<'tree> for Super<'tree> {
    fn from_node(
        node: ::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        debug_assert_eq!(node.kind(), "super");
        Ok(Self {
            span: ::treesitter_types::Span::from(node),
            text: node.utf8_text(src)?,
        })
    }
}
impl<'tree> ::treesitter_types::LeafNode<'tree> for Super<'tree> {
    fn text(&self) -> &'tree str {
        self.text
    }
}
impl ::treesitter_types::Spanned for Super<'_> {
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
pub struct Uninterpreted<'tree> {
    pub span: ::treesitter_types::Span,
    text: &'tree str,
}
impl<'tree> ::treesitter_types::FromNode<'tree> for Uninterpreted<'tree> {
    fn from_node(
        node: ::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        debug_assert_eq!(node.kind(), "uninterpreted");
        Ok(Self {
            span: ::treesitter_types::Span::from(node),
            text: node.utf8_text(src)?,
        })
    }
}
impl<'tree> ::treesitter_types::LeafNode<'tree> for Uninterpreted<'tree> {
    fn text(&self) -> &'tree str {
        self.text
    }
}
impl ::treesitter_types::Spanned for Uninterpreted<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        self.span
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ArgumentListChildren<'tree> {
    Expression(::std::boxed::Box<Expression<'tree>>),
    BlockArgument(::std::boxed::Box<BlockArgument<'tree>>),
    ForwardArgument(::std::boxed::Box<ForwardArgument<'tree>>),
    HashSplatArgument(::std::boxed::Box<HashSplatArgument<'tree>>),
    Pair(::std::boxed::Box<Pair<'tree>>),
    SplatArgument(::std::boxed::Box<SplatArgument<'tree>>),
}
impl<'tree> ::treesitter_types::FromNode<'tree> for ArgumentListChildren<'tree> {
    #[allow(clippy::collapsible_else_if)]
    fn from_node(
        node: ::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        match node.kind() {
            "block_argument" => Ok(Self::BlockArgument(::std::boxed::Box::new(
                ::treesitter_types::maybe_grow_stack(|| {
                    <BlockArgument as ::treesitter_types::FromNode>::from_node(node, src)
                })?,
            ))),
            "forward_argument" => Ok(Self::ForwardArgument(::std::boxed::Box::new(
                ::treesitter_types::maybe_grow_stack(|| {
                    <ForwardArgument as ::treesitter_types::FromNode>::from_node(node, src)
                })?,
            ))),
            "hash_splat_argument" => Ok(Self::HashSplatArgument(::std::boxed::Box::new(
                ::treesitter_types::maybe_grow_stack(|| {
                    <HashSplatArgument as ::treesitter_types::FromNode>::from_node(node, src)
                })?,
            ))),
            "pair" => Ok(Self::Pair(::std::boxed::Box::new(
                ::treesitter_types::maybe_grow_stack(|| {
                    <Pair as ::treesitter_types::FromNode>::from_node(node, src)
                })?,
            ))),
            "splat_argument" => Ok(Self::SplatArgument(::std::boxed::Box::new(
                ::treesitter_types::maybe_grow_stack(|| {
                    <SplatArgument as ::treesitter_types::FromNode>::from_node(node, src)
                })?,
            ))),
            _other => {
                if let Ok(v) = ::treesitter_types::maybe_grow_stack(|| {
                    <Expression as ::treesitter_types::FromNode>::from_node(node, src)
                }) {
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
impl ::treesitter_types::Spanned for ArgumentListChildren<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        match self {
            Self::Expression(inner) => inner.span(),
            Self::BlockArgument(inner) => inner.span(),
            Self::ForwardArgument(inner) => inner.span(),
            Self::HashSplatArgument(inner) => inner.span(),
            Self::Pair(inner) => inner.span(),
            Self::SplatArgument(inner) => inner.span(),
        }
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ArrayChildren<'tree> {
    Expression(::std::boxed::Box<Expression<'tree>>),
    BlockArgument(::std::boxed::Box<BlockArgument<'tree>>),
    ForwardArgument(::std::boxed::Box<ForwardArgument<'tree>>),
    HashSplatArgument(::std::boxed::Box<HashSplatArgument<'tree>>),
    Pair(::std::boxed::Box<Pair<'tree>>),
    SplatArgument(::std::boxed::Box<SplatArgument<'tree>>),
}
impl<'tree> ::treesitter_types::FromNode<'tree> for ArrayChildren<'tree> {
    #[allow(clippy::collapsible_else_if)]
    fn from_node(
        node: ::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        match node.kind() {
            "block_argument" => Ok(Self::BlockArgument(::std::boxed::Box::new(
                ::treesitter_types::maybe_grow_stack(|| {
                    <BlockArgument as ::treesitter_types::FromNode>::from_node(node, src)
                })?,
            ))),
            "forward_argument" => Ok(Self::ForwardArgument(::std::boxed::Box::new(
                ::treesitter_types::maybe_grow_stack(|| {
                    <ForwardArgument as ::treesitter_types::FromNode>::from_node(node, src)
                })?,
            ))),
            "hash_splat_argument" => Ok(Self::HashSplatArgument(::std::boxed::Box::new(
                ::treesitter_types::maybe_grow_stack(|| {
                    <HashSplatArgument as ::treesitter_types::FromNode>::from_node(node, src)
                })?,
            ))),
            "pair" => Ok(Self::Pair(::std::boxed::Box::new(
                ::treesitter_types::maybe_grow_stack(|| {
                    <Pair as ::treesitter_types::FromNode>::from_node(node, src)
                })?,
            ))),
            "splat_argument" => Ok(Self::SplatArgument(::std::boxed::Box::new(
                ::treesitter_types::maybe_grow_stack(|| {
                    <SplatArgument as ::treesitter_types::FromNode>::from_node(node, src)
                })?,
            ))),
            _other => {
                if let Ok(v) = ::treesitter_types::maybe_grow_stack(|| {
                    <Expression as ::treesitter_types::FromNode>::from_node(node, src)
                }) {
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
impl ::treesitter_types::Spanned for ArrayChildren<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        match self {
            Self::Expression(inner) => inner.span(),
            Self::BlockArgument(inner) => inner.span(),
            Self::ForwardArgument(inner) => inner.span(),
            Self::HashSplatArgument(inner) => inner.span(),
            Self::Pair(inner) => inner.span(),
            Self::SplatArgument(inner) => inner.span(),
        }
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ArrayPatternChildren<'tree> {
    PatternExpr(::std::boxed::Box<PatternExpr<'tree>>),
    SplatParameter(::std::boxed::Box<SplatParameter<'tree>>),
}
impl<'tree> ::treesitter_types::FromNode<'tree> for ArrayPatternChildren<'tree> {
    #[allow(clippy::collapsible_else_if)]
    fn from_node(
        node: ::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        match node.kind() {
            "splat_parameter" => Ok(Self::SplatParameter(::std::boxed::Box::new(
                ::treesitter_types::maybe_grow_stack(|| {
                    <SplatParameter as ::treesitter_types::FromNode>::from_node(node, src)
                })?,
            ))),
            _other => {
                if let Ok(v) = ::treesitter_types::maybe_grow_stack(|| {
                    <PatternExpr as ::treesitter_types::FromNode>::from_node(node, src)
                }) {
                    Ok(Self::PatternExpr(::std::boxed::Box::new(v)))
                } else {
                    Err(::treesitter_types::ParseError::unexpected_kind(
                        _other, node,
                    ))
                }
            }
        }
    }
}
impl ::treesitter_types::Spanned for ArrayPatternChildren<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        match self {
            Self::PatternExpr(inner) => inner.span(),
            Self::SplatParameter(inner) => inner.span(),
        }
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum AssignmentLeft<'tree> {
    Lhs(::std::boxed::Box<Lhs<'tree>>),
    LeftAssignmentList(::std::boxed::Box<LeftAssignmentList<'tree>>),
}
impl<'tree> ::treesitter_types::FromNode<'tree> for AssignmentLeft<'tree> {
    #[allow(clippy::collapsible_else_if)]
    fn from_node(
        node: ::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        match node.kind() {
            "left_assignment_list" => Ok(Self::LeftAssignmentList(::std::boxed::Box::new(
                ::treesitter_types::maybe_grow_stack(|| {
                    <LeftAssignmentList as ::treesitter_types::FromNode>::from_node(node, src)
                })?,
            ))),
            _other => {
                if let Ok(v) = ::treesitter_types::maybe_grow_stack(|| {
                    <Lhs as ::treesitter_types::FromNode>::from_node(node, src)
                }) {
                    Ok(Self::Lhs(::std::boxed::Box::new(v)))
                } else {
                    Err(::treesitter_types::ParseError::unexpected_kind(
                        _other, node,
                    ))
                }
            }
        }
    }
}
impl ::treesitter_types::Spanned for AssignmentLeft<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        match self {
            Self::Lhs(inner) => inner.span(),
            Self::LeftAssignmentList(inner) => inner.span(),
        }
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum AssignmentRight<'tree> {
    Expression(::std::boxed::Box<Expression<'tree>>),
    RescueModifier(::std::boxed::Box<RescueModifier<'tree>>),
    RightAssignmentList(::std::boxed::Box<RightAssignmentList<'tree>>),
    SplatArgument(::std::boxed::Box<SplatArgument<'tree>>),
}
impl<'tree> ::treesitter_types::FromNode<'tree> for AssignmentRight<'tree> {
    #[allow(clippy::collapsible_else_if)]
    fn from_node(
        node: ::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        match node.kind() {
            "rescue_modifier" => Ok(Self::RescueModifier(::std::boxed::Box::new(
                ::treesitter_types::maybe_grow_stack(|| {
                    <RescueModifier as ::treesitter_types::FromNode>::from_node(node, src)
                })?,
            ))),
            "right_assignment_list" => Ok(Self::RightAssignmentList(::std::boxed::Box::new(
                ::treesitter_types::maybe_grow_stack(|| {
                    <RightAssignmentList as ::treesitter_types::FromNode>::from_node(node, src)
                })?,
            ))),
            "splat_argument" => Ok(Self::SplatArgument(::std::boxed::Box::new(
                ::treesitter_types::maybe_grow_stack(|| {
                    <SplatArgument as ::treesitter_types::FromNode>::from_node(node, src)
                })?,
            ))),
            _other => {
                if let Ok(v) = ::treesitter_types::maybe_grow_stack(|| {
                    <Expression as ::treesitter_types::FromNode>::from_node(node, src)
                }) {
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
impl ::treesitter_types::Spanned for AssignmentRight<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        match self {
            Self::Expression(inner) => inner.span(),
            Self::RescueModifier(inner) => inner.span(),
            Self::RightAssignmentList(inner) => inner.span(),
            Self::SplatArgument(inner) => inner.span(),
        }
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum BareStringChildren<'tree> {
    EscapeSequence(::std::boxed::Box<EscapeSequence<'tree>>),
    Interpolation(::std::boxed::Box<Interpolation<'tree>>),
    StringContent(::std::boxed::Box<StringContent<'tree>>),
}
impl<'tree> ::treesitter_types::FromNode<'tree> for BareStringChildren<'tree> {
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
            "interpolation" => Ok(Self::Interpolation(::std::boxed::Box::new(
                ::treesitter_types::maybe_grow_stack(|| {
                    <Interpolation as ::treesitter_types::FromNode>::from_node(node, src)
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
impl ::treesitter_types::Spanned for BareStringChildren<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        match self {
            Self::EscapeSequence(inner) => inner.span(),
            Self::Interpolation(inner) => inner.span(),
            Self::StringContent(inner) => inner.span(),
        }
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum BareSymbolChildren<'tree> {
    EscapeSequence(::std::boxed::Box<EscapeSequence<'tree>>),
    Interpolation(::std::boxed::Box<Interpolation<'tree>>),
    StringContent(::std::boxed::Box<StringContent<'tree>>),
}
impl<'tree> ::treesitter_types::FromNode<'tree> for BareSymbolChildren<'tree> {
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
            "interpolation" => Ok(Self::Interpolation(::std::boxed::Box::new(
                ::treesitter_types::maybe_grow_stack(|| {
                    <Interpolation as ::treesitter_types::FromNode>::from_node(node, src)
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
impl ::treesitter_types::Spanned for BareSymbolChildren<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        match self {
            Self::EscapeSequence(inner) => inner.span(),
            Self::Interpolation(inner) => inner.span(),
            Self::StringContent(inner) => inner.span(),
        }
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum BeginChildren<'tree> {
    Statement(::std::boxed::Box<Statement<'tree>>),
    Else(::std::boxed::Box<Else<'tree>>),
    EmptyStatement(::std::boxed::Box<EmptyStatement<'tree>>),
    Ensure(::std::boxed::Box<Ensure<'tree>>),
    Rescue(::std::boxed::Box<Rescue<'tree>>),
}
impl<'tree> ::treesitter_types::FromNode<'tree> for BeginChildren<'tree> {
    #[allow(clippy::collapsible_else_if)]
    fn from_node(
        node: ::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        match node.kind() {
            "else" => Ok(Self::Else(::std::boxed::Box::new(
                ::treesitter_types::maybe_grow_stack(|| {
                    <Else as ::treesitter_types::FromNode>::from_node(node, src)
                })?,
            ))),
            "empty_statement" => Ok(Self::EmptyStatement(::std::boxed::Box::new(
                ::treesitter_types::maybe_grow_stack(|| {
                    <EmptyStatement as ::treesitter_types::FromNode>::from_node(node, src)
                })?,
            ))),
            "ensure" => Ok(Self::Ensure(::std::boxed::Box::new(
                ::treesitter_types::maybe_grow_stack(|| {
                    <Ensure as ::treesitter_types::FromNode>::from_node(node, src)
                })?,
            ))),
            "rescue" => Ok(Self::Rescue(::std::boxed::Box::new(
                ::treesitter_types::maybe_grow_stack(|| {
                    <Rescue as ::treesitter_types::FromNode>::from_node(node, src)
                })?,
            ))),
            _other => {
                if let Ok(v) = ::treesitter_types::maybe_grow_stack(|| {
                    <Statement as ::treesitter_types::FromNode>::from_node(node, src)
                }) {
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
impl ::treesitter_types::Spanned for BeginChildren<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        match self {
            Self::Statement(inner) => inner.span(),
            Self::Else(inner) => inner.span(),
            Self::EmptyStatement(inner) => inner.span(),
            Self::Ensure(inner) => inner.span(),
            Self::Rescue(inner) => inner.span(),
        }
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum BeginBlockChildren<'tree> {
    Statement(::std::boxed::Box<Statement<'tree>>),
    EmptyStatement(::std::boxed::Box<EmptyStatement<'tree>>),
}
impl<'tree> ::treesitter_types::FromNode<'tree> for BeginBlockChildren<'tree> {
    #[allow(clippy::collapsible_else_if)]
    fn from_node(
        node: ::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        match node.kind() {
            "empty_statement" => Ok(Self::EmptyStatement(::std::boxed::Box::new(
                ::treesitter_types::maybe_grow_stack(|| {
                    <EmptyStatement as ::treesitter_types::FromNode>::from_node(node, src)
                })?,
            ))),
            _other => {
                if let Ok(v) = ::treesitter_types::maybe_grow_stack(|| {
                    <Statement as ::treesitter_types::FromNode>::from_node(node, src)
                }) {
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
impl ::treesitter_types::Spanned for BeginBlockChildren<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        match self {
            Self::Statement(inner) => inner.span(),
            Self::EmptyStatement(inner) => inner.span(),
        }
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum BinaryLeft<'tree> {
    Expression(::std::boxed::Box<Expression<'tree>>),
    SimpleNumeric(::std::boxed::Box<SimpleNumeric<'tree>>),
}
impl<'tree> ::treesitter_types::FromNode<'tree> for BinaryLeft<'tree> {
    #[allow(clippy::collapsible_else_if)]
    fn from_node(
        node: ::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        if let Ok(v) = ::treesitter_types::maybe_grow_stack(|| {
            <Expression as ::treesitter_types::FromNode>::from_node(node, src)
        }) {
            Ok(Self::Expression(::std::boxed::Box::new(v)))
        } else {
            if let Ok(v) = ::treesitter_types::maybe_grow_stack(|| {
                <SimpleNumeric as ::treesitter_types::FromNode>::from_node(node, src)
            }) {
                Ok(Self::SimpleNumeric(::std::boxed::Box::new(v)))
            } else {
                Err(::treesitter_types::ParseError::unexpected_kind(
                    node.kind(),
                    node,
                ))
            }
        }
    }
}
impl ::treesitter_types::Spanned for BinaryLeft<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        match self {
            Self::Expression(inner) => inner.span(),
            Self::SimpleNumeric(inner) => inner.span(),
        }
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum BinaryOperator {
    NotEq(::treesitter_types::Span),
    BangTilde(::treesitter_types::Span),
    Percent(::treesitter_types::Span),
    Amp(::treesitter_types::Span),
    AmpAmp(::treesitter_types::Span),
    Star(::treesitter_types::Span),
    StarStar(::treesitter_types::Span),
    Plus(::treesitter_types::Span),
    Minus(::treesitter_types::Span),
    Slash(::treesitter_types::Span),
    Lt(::treesitter_types::Span),
    Shl(::treesitter_types::Span),
    LtEq(::treesitter_types::Span),
    LtEqGt(::treesitter_types::Span),
    EqEq(::treesitter_types::Span),
    EqEqEq(::treesitter_types::Span),
    EqTilde(::treesitter_types::Span),
    Gt(::treesitter_types::Span),
    GtEq(::treesitter_types::Span),
    Shr(::treesitter_types::Span),
    Caret(::treesitter_types::Span),
    And(::treesitter_types::Span),
    Or(::treesitter_types::Span),
    Pipe(::treesitter_types::Span),
    PipePipe(::treesitter_types::Span),
}
impl<'tree> ::treesitter_types::FromNode<'tree> for BinaryOperator {
    #[allow(clippy::collapsible_else_if)]
    fn from_node(
        node: ::tree_sitter::Node<'tree>,
        _src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        match node.kind() {
            "!=" => Ok(Self::NotEq(::treesitter_types::Span::from(node))),
            "!~" => Ok(Self::BangTilde(::treesitter_types::Span::from(node))),
            "%" => Ok(Self::Percent(::treesitter_types::Span::from(node))),
            "&" => Ok(Self::Amp(::treesitter_types::Span::from(node))),
            "&&" => Ok(Self::AmpAmp(::treesitter_types::Span::from(node))),
            "*" => Ok(Self::Star(::treesitter_types::Span::from(node))),
            "**" => Ok(Self::StarStar(::treesitter_types::Span::from(node))),
            "+" => Ok(Self::Plus(::treesitter_types::Span::from(node))),
            "-" => Ok(Self::Minus(::treesitter_types::Span::from(node))),
            "/" => Ok(Self::Slash(::treesitter_types::Span::from(node))),
            "<" => Ok(Self::Lt(::treesitter_types::Span::from(node))),
            "<<" => Ok(Self::Shl(::treesitter_types::Span::from(node))),
            "<=" => Ok(Self::LtEq(::treesitter_types::Span::from(node))),
            "<=>" => Ok(Self::LtEqGt(::treesitter_types::Span::from(node))),
            "==" => Ok(Self::EqEq(::treesitter_types::Span::from(node))),
            "===" => Ok(Self::EqEqEq(::treesitter_types::Span::from(node))),
            "=~" => Ok(Self::EqTilde(::treesitter_types::Span::from(node))),
            ">" => Ok(Self::Gt(::treesitter_types::Span::from(node))),
            ">=" => Ok(Self::GtEq(::treesitter_types::Span::from(node))),
            ">>" => Ok(Self::Shr(::treesitter_types::Span::from(node))),
            "^" => Ok(Self::Caret(::treesitter_types::Span::from(node))),
            "and" => Ok(Self::And(::treesitter_types::Span::from(node))),
            "or" => Ok(Self::Or(::treesitter_types::Span::from(node))),
            "|" => Ok(Self::Pipe(::treesitter_types::Span::from(node))),
            "||" => Ok(Self::PipePipe(::treesitter_types::Span::from(node))),
            other => Err(::treesitter_types::ParseError::unexpected_kind(other, node)),
        }
    }
}
impl ::treesitter_types::Spanned for BinaryOperator {
    fn span(&self) -> ::treesitter_types::Span {
        match self {
            Self::NotEq(span) => *span,
            Self::BangTilde(span) => *span,
            Self::Percent(span) => *span,
            Self::Amp(span) => *span,
            Self::AmpAmp(span) => *span,
            Self::Star(span) => *span,
            Self::StarStar(span) => *span,
            Self::Plus(span) => *span,
            Self::Minus(span) => *span,
            Self::Slash(span) => *span,
            Self::Lt(span) => *span,
            Self::Shl(span) => *span,
            Self::LtEq(span) => *span,
            Self::LtEqGt(span) => *span,
            Self::EqEq(span) => *span,
            Self::EqEqEq(span) => *span,
            Self::EqTilde(span) => *span,
            Self::Gt(span) => *span,
            Self::GtEq(span) => *span,
            Self::Shr(span) => *span,
            Self::Caret(span) => *span,
            Self::And(span) => *span,
            Self::Or(span) => *span,
            Self::Pipe(span) => *span,
            Self::PipePipe(span) => *span,
        }
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum BlockBodyChildren<'tree> {
    Statement(::std::boxed::Box<Statement<'tree>>),
    EmptyStatement(::std::boxed::Box<EmptyStatement<'tree>>),
}
impl<'tree> ::treesitter_types::FromNode<'tree> for BlockBodyChildren<'tree> {
    #[allow(clippy::collapsible_else_if)]
    fn from_node(
        node: ::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        match node.kind() {
            "empty_statement" => Ok(Self::EmptyStatement(::std::boxed::Box::new(
                ::treesitter_types::maybe_grow_stack(|| {
                    <EmptyStatement as ::treesitter_types::FromNode>::from_node(node, src)
                })?,
            ))),
            _other => {
                if let Ok(v) = ::treesitter_types::maybe_grow_stack(|| {
                    <Statement as ::treesitter_types::FromNode>::from_node(node, src)
                }) {
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
impl ::treesitter_types::Spanned for BlockBodyChildren<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        match self {
            Self::Statement(inner) => inner.span(),
            Self::EmptyStatement(inner) => inner.span(),
        }
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum BlockParametersChildren<'tree> {
    BlockParameter(::std::boxed::Box<BlockParameter<'tree>>),
    DestructuredParameter(::std::boxed::Box<DestructuredParameter<'tree>>),
    ForwardParameter(::std::boxed::Box<ForwardParameter<'tree>>),
    HashSplatNil(::std::boxed::Box<HashSplatNil<'tree>>),
    HashSplatParameter(::std::boxed::Box<HashSplatParameter<'tree>>),
    Identifier(::std::boxed::Box<Identifier<'tree>>),
    KeywordParameter(::std::boxed::Box<KeywordParameter<'tree>>),
    OptionalParameter(::std::boxed::Box<OptionalParameter<'tree>>),
    SplatParameter(::std::boxed::Box<SplatParameter<'tree>>),
}
impl<'tree> ::treesitter_types::FromNode<'tree> for BlockParametersChildren<'tree> {
    #[allow(clippy::collapsible_else_if)]
    fn from_node(
        node: ::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        match node.kind() {
            "block_parameter" => Ok(Self::BlockParameter(::std::boxed::Box::new(
                ::treesitter_types::maybe_grow_stack(|| {
                    <BlockParameter as ::treesitter_types::FromNode>::from_node(node, src)
                })?,
            ))),
            "destructured_parameter" => Ok(Self::DestructuredParameter(::std::boxed::Box::new(
                ::treesitter_types::maybe_grow_stack(|| {
                    <DestructuredParameter as ::treesitter_types::FromNode>::from_node(node, src)
                })?,
            ))),
            "forward_parameter" => Ok(Self::ForwardParameter(::std::boxed::Box::new(
                ::treesitter_types::maybe_grow_stack(|| {
                    <ForwardParameter as ::treesitter_types::FromNode>::from_node(node, src)
                })?,
            ))),
            "hash_splat_nil" => Ok(Self::HashSplatNil(::std::boxed::Box::new(
                ::treesitter_types::maybe_grow_stack(|| {
                    <HashSplatNil as ::treesitter_types::FromNode>::from_node(node, src)
                })?,
            ))),
            "hash_splat_parameter" => Ok(Self::HashSplatParameter(::std::boxed::Box::new(
                ::treesitter_types::maybe_grow_stack(|| {
                    <HashSplatParameter as ::treesitter_types::FromNode>::from_node(node, src)
                })?,
            ))),
            "identifier" => Ok(Self::Identifier(::std::boxed::Box::new(
                ::treesitter_types::maybe_grow_stack(|| {
                    <Identifier as ::treesitter_types::FromNode>::from_node(node, src)
                })?,
            ))),
            "keyword_parameter" => Ok(Self::KeywordParameter(::std::boxed::Box::new(
                ::treesitter_types::maybe_grow_stack(|| {
                    <KeywordParameter as ::treesitter_types::FromNode>::from_node(node, src)
                })?,
            ))),
            "optional_parameter" => Ok(Self::OptionalParameter(::std::boxed::Box::new(
                ::treesitter_types::maybe_grow_stack(|| {
                    <OptionalParameter as ::treesitter_types::FromNode>::from_node(node, src)
                })?,
            ))),
            "splat_parameter" => Ok(Self::SplatParameter(::std::boxed::Box::new(
                ::treesitter_types::maybe_grow_stack(|| {
                    <SplatParameter as ::treesitter_types::FromNode>::from_node(node, src)
                })?,
            ))),
            other => Err(::treesitter_types::ParseError::unexpected_kind(other, node)),
        }
    }
}
impl ::treesitter_types::Spanned for BlockParametersChildren<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        match self {
            Self::BlockParameter(inner) => inner.span(),
            Self::DestructuredParameter(inner) => inner.span(),
            Self::ForwardParameter(inner) => inner.span(),
            Self::HashSplatNil(inner) => inner.span(),
            Self::HashSplatParameter(inner) => inner.span(),
            Self::Identifier(inner) => inner.span(),
            Self::KeywordParameter(inner) => inner.span(),
            Self::OptionalParameter(inner) => inner.span(),
            Self::SplatParameter(inner) => inner.span(),
        }
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum BodyStatementChildren<'tree> {
    Statement(::std::boxed::Box<Statement<'tree>>),
    Else(::std::boxed::Box<Else<'tree>>),
    EmptyStatement(::std::boxed::Box<EmptyStatement<'tree>>),
    Ensure(::std::boxed::Box<Ensure<'tree>>),
    Rescue(::std::boxed::Box<Rescue<'tree>>),
}
impl<'tree> ::treesitter_types::FromNode<'tree> for BodyStatementChildren<'tree> {
    #[allow(clippy::collapsible_else_if)]
    fn from_node(
        node: ::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        match node.kind() {
            "else" => Ok(Self::Else(::std::boxed::Box::new(
                ::treesitter_types::maybe_grow_stack(|| {
                    <Else as ::treesitter_types::FromNode>::from_node(node, src)
                })?,
            ))),
            "empty_statement" => Ok(Self::EmptyStatement(::std::boxed::Box::new(
                ::treesitter_types::maybe_grow_stack(|| {
                    <EmptyStatement as ::treesitter_types::FromNode>::from_node(node, src)
                })?,
            ))),
            "ensure" => Ok(Self::Ensure(::std::boxed::Box::new(
                ::treesitter_types::maybe_grow_stack(|| {
                    <Ensure as ::treesitter_types::FromNode>::from_node(node, src)
                })?,
            ))),
            "rescue" => Ok(Self::Rescue(::std::boxed::Box::new(
                ::treesitter_types::maybe_grow_stack(|| {
                    <Rescue as ::treesitter_types::FromNode>::from_node(node, src)
                })?,
            ))),
            _other => {
                if let Ok(v) = ::treesitter_types::maybe_grow_stack(|| {
                    <Statement as ::treesitter_types::FromNode>::from_node(node, src)
                }) {
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
impl ::treesitter_types::Spanned for BodyStatementChildren<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        match self {
            Self::Statement(inner) => inner.span(),
            Self::Else(inner) => inner.span(),
            Self::EmptyStatement(inner) => inner.span(),
            Self::Ensure(inner) => inner.span(),
            Self::Rescue(inner) => inner.span(),
        }
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum CallBlock<'tree> {
    Block(::std::boxed::Box<Block<'tree>>),
    DoBlock(::std::boxed::Box<DoBlock<'tree>>),
}
impl<'tree> ::treesitter_types::FromNode<'tree> for CallBlock<'tree> {
    #[allow(clippy::collapsible_else_if)]
    fn from_node(
        node: ::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        match node.kind() {
            "block" => Ok(Self::Block(::std::boxed::Box::new(
                ::treesitter_types::maybe_grow_stack(|| {
                    <Block as ::treesitter_types::FromNode>::from_node(node, src)
                })?,
            ))),
            "do_block" => Ok(Self::DoBlock(::std::boxed::Box::new(
                ::treesitter_types::maybe_grow_stack(|| {
                    <DoBlock as ::treesitter_types::FromNode>::from_node(node, src)
                })?,
            ))),
            other => Err(::treesitter_types::ParseError::unexpected_kind(other, node)),
        }
    }
}
impl ::treesitter_types::Spanned for CallBlock<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        match self {
            Self::Block(inner) => inner.span(),
            Self::DoBlock(inner) => inner.span(),
        }
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum CallMethod<'tree> {
    Variable(::std::boxed::Box<Variable<'tree>>),
    Operator(::std::boxed::Box<Operator<'tree>>),
}
impl<'tree> ::treesitter_types::FromNode<'tree> for CallMethod<'tree> {
    #[allow(clippy::collapsible_else_if)]
    fn from_node(
        node: ::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        match node.kind() {
            "operator" => Ok(Self::Operator(::std::boxed::Box::new(
                ::treesitter_types::maybe_grow_stack(|| {
                    <Operator as ::treesitter_types::FromNode>::from_node(node, src)
                })?,
            ))),
            _other => {
                if let Ok(v) = ::treesitter_types::maybe_grow_stack(|| {
                    <Variable as ::treesitter_types::FromNode>::from_node(node, src)
                }) {
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
impl ::treesitter_types::Spanned for CallMethod<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        match self {
            Self::Variable(inner) => inner.span(),
            Self::Operator(inner) => inner.span(),
        }
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum CaseChildren<'tree> {
    Else(::std::boxed::Box<Else<'tree>>),
    When(::std::boxed::Box<When<'tree>>),
}
impl<'tree> ::treesitter_types::FromNode<'tree> for CaseChildren<'tree> {
    #[allow(clippy::collapsible_else_if)]
    fn from_node(
        node: ::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        match node.kind() {
            "else" => Ok(Self::Else(::std::boxed::Box::new(
                ::treesitter_types::maybe_grow_stack(|| {
                    <Else as ::treesitter_types::FromNode>::from_node(node, src)
                })?,
            ))),
            "when" => Ok(Self::When(::std::boxed::Box::new(
                ::treesitter_types::maybe_grow_stack(|| {
                    <When as ::treesitter_types::FromNode>::from_node(node, src)
                })?,
            ))),
            other => Err(::treesitter_types::ParseError::unexpected_kind(other, node)),
        }
    }
}
impl ::treesitter_types::Spanned for CaseChildren<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        match self {
            Self::Else(inner) => inner.span(),
            Self::When(inner) => inner.span(),
        }
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ClassName<'tree> {
    Constant(::std::boxed::Box<Constant<'tree>>),
    ScopeResolution(::std::boxed::Box<ScopeResolution<'tree>>),
}
impl<'tree> ::treesitter_types::FromNode<'tree> for ClassName<'tree> {
    #[allow(clippy::collapsible_else_if)]
    fn from_node(
        node: ::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        match node.kind() {
            "constant" => Ok(Self::Constant(::std::boxed::Box::new(
                ::treesitter_types::maybe_grow_stack(|| {
                    <Constant as ::treesitter_types::FromNode>::from_node(node, src)
                })?,
            ))),
            "scope_resolution" => Ok(Self::ScopeResolution(::std::boxed::Box::new(
                ::treesitter_types::maybe_grow_stack(|| {
                    <ScopeResolution as ::treesitter_types::FromNode>::from_node(node, src)
                })?,
            ))),
            other => Err(::treesitter_types::ParseError::unexpected_kind(other, node)),
        }
    }
}
impl ::treesitter_types::Spanned for ClassName<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        match self {
            Self::Constant(inner) => inner.span(),
            Self::ScopeResolution(inner) => inner.span(),
        }
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ComplexChildren<'tree> {
    Float(::std::boxed::Box<Float<'tree>>),
    Integer(::std::boxed::Box<Integer<'tree>>),
    Rational(::std::boxed::Box<Rational<'tree>>),
}
impl<'tree> ::treesitter_types::FromNode<'tree> for ComplexChildren<'tree> {
    #[allow(clippy::collapsible_else_if)]
    fn from_node(
        node: ::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        match node.kind() {
            "float" => Ok(Self::Float(::std::boxed::Box::new(
                ::treesitter_types::maybe_grow_stack(|| {
                    <Float as ::treesitter_types::FromNode>::from_node(node, src)
                })?,
            ))),
            "integer" => Ok(Self::Integer(::std::boxed::Box::new(
                ::treesitter_types::maybe_grow_stack(|| {
                    <Integer as ::treesitter_types::FromNode>::from_node(node, src)
                })?,
            ))),
            "rational" => Ok(Self::Rational(::std::boxed::Box::new(
                ::treesitter_types::maybe_grow_stack(|| {
                    <Rational as ::treesitter_types::FromNode>::from_node(node, src)
                })?,
            ))),
            other => Err(::treesitter_types::ParseError::unexpected_kind(other, node)),
        }
    }
}
impl ::treesitter_types::Spanned for ComplexChildren<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        match self {
            Self::Float(inner) => inner.span(),
            Self::Integer(inner) => inner.span(),
            Self::Rational(inner) => inner.span(),
        }
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum DelimitedSymbolChildren<'tree> {
    EscapeSequence(::std::boxed::Box<EscapeSequence<'tree>>),
    Interpolation(::std::boxed::Box<Interpolation<'tree>>),
    StringContent(::std::boxed::Box<StringContent<'tree>>),
}
impl<'tree> ::treesitter_types::FromNode<'tree> for DelimitedSymbolChildren<'tree> {
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
            "interpolation" => Ok(Self::Interpolation(::std::boxed::Box::new(
                ::treesitter_types::maybe_grow_stack(|| {
                    <Interpolation as ::treesitter_types::FromNode>::from_node(node, src)
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
impl ::treesitter_types::Spanned for DelimitedSymbolChildren<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        match self {
            Self::EscapeSequence(inner) => inner.span(),
            Self::Interpolation(inner) => inner.span(),
            Self::StringContent(inner) => inner.span(),
        }
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum DestructuredLeftAssignmentChildren<'tree> {
    Lhs(::std::boxed::Box<Lhs<'tree>>),
    DestructuredLeftAssignment(::std::boxed::Box<DestructuredLeftAssignment<'tree>>),
    RestAssignment(::std::boxed::Box<RestAssignment<'tree>>),
}
impl<'tree> ::treesitter_types::FromNode<'tree> for DestructuredLeftAssignmentChildren<'tree> {
    #[allow(clippy::collapsible_else_if)]
    fn from_node(
        node: ::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        match node.kind() {
            "destructured_left_assignment" => Ok(Self::DestructuredLeftAssignment(
                ::std::boxed::Box::new(::treesitter_types::maybe_grow_stack(|| {
                    <DestructuredLeftAssignment as ::treesitter_types::FromNode>::from_node(
                        node, src,
                    )
                })?),
            )),
            "rest_assignment" => Ok(Self::RestAssignment(::std::boxed::Box::new(
                ::treesitter_types::maybe_grow_stack(|| {
                    <RestAssignment as ::treesitter_types::FromNode>::from_node(node, src)
                })?,
            ))),
            _other => {
                if let Ok(v) = ::treesitter_types::maybe_grow_stack(|| {
                    <Lhs as ::treesitter_types::FromNode>::from_node(node, src)
                }) {
                    Ok(Self::Lhs(::std::boxed::Box::new(v)))
                } else {
                    Err(::treesitter_types::ParseError::unexpected_kind(
                        _other, node,
                    ))
                }
            }
        }
    }
}
impl ::treesitter_types::Spanned for DestructuredLeftAssignmentChildren<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        match self {
            Self::Lhs(inner) => inner.span(),
            Self::DestructuredLeftAssignment(inner) => inner.span(),
            Self::RestAssignment(inner) => inner.span(),
        }
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum DestructuredParameterChildren<'tree> {
    BlockParameter(::std::boxed::Box<BlockParameter<'tree>>),
    DestructuredParameter(::std::boxed::Box<DestructuredParameter<'tree>>),
    ForwardParameter(::std::boxed::Box<ForwardParameter<'tree>>),
    HashSplatNil(::std::boxed::Box<HashSplatNil<'tree>>),
    HashSplatParameter(::std::boxed::Box<HashSplatParameter<'tree>>),
    Identifier(::std::boxed::Box<Identifier<'tree>>),
    KeywordParameter(::std::boxed::Box<KeywordParameter<'tree>>),
    OptionalParameter(::std::boxed::Box<OptionalParameter<'tree>>),
    SplatParameter(::std::boxed::Box<SplatParameter<'tree>>),
}
impl<'tree> ::treesitter_types::FromNode<'tree> for DestructuredParameterChildren<'tree> {
    #[allow(clippy::collapsible_else_if)]
    fn from_node(
        node: ::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        match node.kind() {
            "block_parameter" => Ok(Self::BlockParameter(::std::boxed::Box::new(
                ::treesitter_types::maybe_grow_stack(|| {
                    <BlockParameter as ::treesitter_types::FromNode>::from_node(node, src)
                })?,
            ))),
            "destructured_parameter" => Ok(Self::DestructuredParameter(::std::boxed::Box::new(
                ::treesitter_types::maybe_grow_stack(|| {
                    <DestructuredParameter as ::treesitter_types::FromNode>::from_node(node, src)
                })?,
            ))),
            "forward_parameter" => Ok(Self::ForwardParameter(::std::boxed::Box::new(
                ::treesitter_types::maybe_grow_stack(|| {
                    <ForwardParameter as ::treesitter_types::FromNode>::from_node(node, src)
                })?,
            ))),
            "hash_splat_nil" => Ok(Self::HashSplatNil(::std::boxed::Box::new(
                ::treesitter_types::maybe_grow_stack(|| {
                    <HashSplatNil as ::treesitter_types::FromNode>::from_node(node, src)
                })?,
            ))),
            "hash_splat_parameter" => Ok(Self::HashSplatParameter(::std::boxed::Box::new(
                ::treesitter_types::maybe_grow_stack(|| {
                    <HashSplatParameter as ::treesitter_types::FromNode>::from_node(node, src)
                })?,
            ))),
            "identifier" => Ok(Self::Identifier(::std::boxed::Box::new(
                ::treesitter_types::maybe_grow_stack(|| {
                    <Identifier as ::treesitter_types::FromNode>::from_node(node, src)
                })?,
            ))),
            "keyword_parameter" => Ok(Self::KeywordParameter(::std::boxed::Box::new(
                ::treesitter_types::maybe_grow_stack(|| {
                    <KeywordParameter as ::treesitter_types::FromNode>::from_node(node, src)
                })?,
            ))),
            "optional_parameter" => Ok(Self::OptionalParameter(::std::boxed::Box::new(
                ::treesitter_types::maybe_grow_stack(|| {
                    <OptionalParameter as ::treesitter_types::FromNode>::from_node(node, src)
                })?,
            ))),
            "splat_parameter" => Ok(Self::SplatParameter(::std::boxed::Box::new(
                ::treesitter_types::maybe_grow_stack(|| {
                    <SplatParameter as ::treesitter_types::FromNode>::from_node(node, src)
                })?,
            ))),
            other => Err(::treesitter_types::ParseError::unexpected_kind(other, node)),
        }
    }
}
impl ::treesitter_types::Spanned for DestructuredParameterChildren<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        match self {
            Self::BlockParameter(inner) => inner.span(),
            Self::DestructuredParameter(inner) => inner.span(),
            Self::ForwardParameter(inner) => inner.span(),
            Self::HashSplatNil(inner) => inner.span(),
            Self::HashSplatParameter(inner) => inner.span(),
            Self::Identifier(inner) => inner.span(),
            Self::KeywordParameter(inner) => inner.span(),
            Self::OptionalParameter(inner) => inner.span(),
            Self::SplatParameter(inner) => inner.span(),
        }
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum DoChildren<'tree> {
    Statement(::std::boxed::Box<Statement<'tree>>),
    EmptyStatement(::std::boxed::Box<EmptyStatement<'tree>>),
}
impl<'tree> ::treesitter_types::FromNode<'tree> for DoChildren<'tree> {
    #[allow(clippy::collapsible_else_if)]
    fn from_node(
        node: ::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        match node.kind() {
            "empty_statement" => Ok(Self::EmptyStatement(::std::boxed::Box::new(
                ::treesitter_types::maybe_grow_stack(|| {
                    <EmptyStatement as ::treesitter_types::FromNode>::from_node(node, src)
                })?,
            ))),
            _other => {
                if let Ok(v) = ::treesitter_types::maybe_grow_stack(|| {
                    <Statement as ::treesitter_types::FromNode>::from_node(node, src)
                }) {
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
impl ::treesitter_types::Spanned for DoChildren<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        match self {
            Self::Statement(inner) => inner.span(),
            Self::EmptyStatement(inner) => inner.span(),
        }
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ElementReferenceBlock<'tree> {
    Block(::std::boxed::Box<Block<'tree>>),
    DoBlock(::std::boxed::Box<DoBlock<'tree>>),
}
impl<'tree> ::treesitter_types::FromNode<'tree> for ElementReferenceBlock<'tree> {
    #[allow(clippy::collapsible_else_if)]
    fn from_node(
        node: ::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        match node.kind() {
            "block" => Ok(Self::Block(::std::boxed::Box::new(
                ::treesitter_types::maybe_grow_stack(|| {
                    <Block as ::treesitter_types::FromNode>::from_node(node, src)
                })?,
            ))),
            "do_block" => Ok(Self::DoBlock(::std::boxed::Box::new(
                ::treesitter_types::maybe_grow_stack(|| {
                    <DoBlock as ::treesitter_types::FromNode>::from_node(node, src)
                })?,
            ))),
            other => Err(::treesitter_types::ParseError::unexpected_kind(other, node)),
        }
    }
}
impl ::treesitter_types::Spanned for ElementReferenceBlock<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        match self {
            Self::Block(inner) => inner.span(),
            Self::DoBlock(inner) => inner.span(),
        }
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ElementReferenceChildren<'tree> {
    Expression(::std::boxed::Box<Expression<'tree>>),
    BlockArgument(::std::boxed::Box<BlockArgument<'tree>>),
    ForwardArgument(::std::boxed::Box<ForwardArgument<'tree>>),
    HashSplatArgument(::std::boxed::Box<HashSplatArgument<'tree>>),
    Pair(::std::boxed::Box<Pair<'tree>>),
    SplatArgument(::std::boxed::Box<SplatArgument<'tree>>),
}
impl<'tree> ::treesitter_types::FromNode<'tree> for ElementReferenceChildren<'tree> {
    #[allow(clippy::collapsible_else_if)]
    fn from_node(
        node: ::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        match node.kind() {
            "block_argument" => Ok(Self::BlockArgument(::std::boxed::Box::new(
                ::treesitter_types::maybe_grow_stack(|| {
                    <BlockArgument as ::treesitter_types::FromNode>::from_node(node, src)
                })?,
            ))),
            "forward_argument" => Ok(Self::ForwardArgument(::std::boxed::Box::new(
                ::treesitter_types::maybe_grow_stack(|| {
                    <ForwardArgument as ::treesitter_types::FromNode>::from_node(node, src)
                })?,
            ))),
            "hash_splat_argument" => Ok(Self::HashSplatArgument(::std::boxed::Box::new(
                ::treesitter_types::maybe_grow_stack(|| {
                    <HashSplatArgument as ::treesitter_types::FromNode>::from_node(node, src)
                })?,
            ))),
            "pair" => Ok(Self::Pair(::std::boxed::Box::new(
                ::treesitter_types::maybe_grow_stack(|| {
                    <Pair as ::treesitter_types::FromNode>::from_node(node, src)
                })?,
            ))),
            "splat_argument" => Ok(Self::SplatArgument(::std::boxed::Box::new(
                ::treesitter_types::maybe_grow_stack(|| {
                    <SplatArgument as ::treesitter_types::FromNode>::from_node(node, src)
                })?,
            ))),
            _other => {
                if let Ok(v) = ::treesitter_types::maybe_grow_stack(|| {
                    <Expression as ::treesitter_types::FromNode>::from_node(node, src)
                }) {
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
impl ::treesitter_types::Spanned for ElementReferenceChildren<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        match self {
            Self::Expression(inner) => inner.span(),
            Self::BlockArgument(inner) => inner.span(),
            Self::ForwardArgument(inner) => inner.span(),
            Self::HashSplatArgument(inner) => inner.span(),
            Self::Pair(inner) => inner.span(),
            Self::SplatArgument(inner) => inner.span(),
        }
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ElseChildren<'tree> {
    Statement(::std::boxed::Box<Statement<'tree>>),
    EmptyStatement(::std::boxed::Box<EmptyStatement<'tree>>),
}
impl<'tree> ::treesitter_types::FromNode<'tree> for ElseChildren<'tree> {
    #[allow(clippy::collapsible_else_if)]
    fn from_node(
        node: ::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        match node.kind() {
            "empty_statement" => Ok(Self::EmptyStatement(::std::boxed::Box::new(
                ::treesitter_types::maybe_grow_stack(|| {
                    <EmptyStatement as ::treesitter_types::FromNode>::from_node(node, src)
                })?,
            ))),
            _other => {
                if let Ok(v) = ::treesitter_types::maybe_grow_stack(|| {
                    <Statement as ::treesitter_types::FromNode>::from_node(node, src)
                }) {
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
impl ::treesitter_types::Spanned for ElseChildren<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        match self {
            Self::Statement(inner) => inner.span(),
            Self::EmptyStatement(inner) => inner.span(),
        }
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ElsifAlternative<'tree> {
    Else(::std::boxed::Box<Else<'tree>>),
    Elsif(::std::boxed::Box<Elsif<'tree>>),
}
impl<'tree> ::treesitter_types::FromNode<'tree> for ElsifAlternative<'tree> {
    #[allow(clippy::collapsible_else_if)]
    fn from_node(
        node: ::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        match node.kind() {
            "else" => Ok(Self::Else(::std::boxed::Box::new(
                ::treesitter_types::maybe_grow_stack(|| {
                    <Else as ::treesitter_types::FromNode>::from_node(node, src)
                })?,
            ))),
            "elsif" => Ok(Self::Elsif(::std::boxed::Box::new(
                ::treesitter_types::maybe_grow_stack(|| {
                    <Elsif as ::treesitter_types::FromNode>::from_node(node, src)
                })?,
            ))),
            other => Err(::treesitter_types::ParseError::unexpected_kind(other, node)),
        }
    }
}
impl ::treesitter_types::Spanned for ElsifAlternative<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        match self {
            Self::Else(inner) => inner.span(),
            Self::Elsif(inner) => inner.span(),
        }
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum EndBlockChildren<'tree> {
    Statement(::std::boxed::Box<Statement<'tree>>),
    EmptyStatement(::std::boxed::Box<EmptyStatement<'tree>>),
}
impl<'tree> ::treesitter_types::FromNode<'tree> for EndBlockChildren<'tree> {
    #[allow(clippy::collapsible_else_if)]
    fn from_node(
        node: ::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        match node.kind() {
            "empty_statement" => Ok(Self::EmptyStatement(::std::boxed::Box::new(
                ::treesitter_types::maybe_grow_stack(|| {
                    <EmptyStatement as ::treesitter_types::FromNode>::from_node(node, src)
                })?,
            ))),
            _other => {
                if let Ok(v) = ::treesitter_types::maybe_grow_stack(|| {
                    <Statement as ::treesitter_types::FromNode>::from_node(node, src)
                }) {
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
impl ::treesitter_types::Spanned for EndBlockChildren<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        match self {
            Self::Statement(inner) => inner.span(),
            Self::EmptyStatement(inner) => inner.span(),
        }
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum EnsureChildren<'tree> {
    Statement(::std::boxed::Box<Statement<'tree>>),
    EmptyStatement(::std::boxed::Box<EmptyStatement<'tree>>),
}
impl<'tree> ::treesitter_types::FromNode<'tree> for EnsureChildren<'tree> {
    #[allow(clippy::collapsible_else_if)]
    fn from_node(
        node: ::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        match node.kind() {
            "empty_statement" => Ok(Self::EmptyStatement(::std::boxed::Box::new(
                ::treesitter_types::maybe_grow_stack(|| {
                    <EmptyStatement as ::treesitter_types::FromNode>::from_node(node, src)
                })?,
            ))),
            _other => {
                if let Ok(v) = ::treesitter_types::maybe_grow_stack(|| {
                    <Statement as ::treesitter_types::FromNode>::from_node(node, src)
                }) {
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
impl ::treesitter_types::Spanned for EnsureChildren<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        match self {
            Self::Statement(inner) => inner.span(),
            Self::EmptyStatement(inner) => inner.span(),
        }
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ExceptionsChildren<'tree> {
    Arg(::std::boxed::Box<Arg<'tree>>),
    SplatArgument(::std::boxed::Box<SplatArgument<'tree>>),
}
impl<'tree> ::treesitter_types::FromNode<'tree> for ExceptionsChildren<'tree> {
    #[allow(clippy::collapsible_else_if)]
    fn from_node(
        node: ::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        match node.kind() {
            "splat_argument" => Ok(Self::SplatArgument(::std::boxed::Box::new(
                ::treesitter_types::maybe_grow_stack(|| {
                    <SplatArgument as ::treesitter_types::FromNode>::from_node(node, src)
                })?,
            ))),
            _other => {
                if let Ok(v) = ::treesitter_types::maybe_grow_stack(|| {
                    <Arg as ::treesitter_types::FromNode>::from_node(node, src)
                }) {
                    Ok(Self::Arg(::std::boxed::Box::new(v)))
                } else {
                    Err(::treesitter_types::ParseError::unexpected_kind(
                        _other, node,
                    ))
                }
            }
        }
    }
}
impl ::treesitter_types::Spanned for ExceptionsChildren<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        match self {
            Self::Arg(inner) => inner.span(),
            Self::SplatArgument(inner) => inner.span(),
        }
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum FindPatternChildren<'tree> {
    PatternExpr(::std::boxed::Box<PatternExpr<'tree>>),
    SplatParameter(::std::boxed::Box<SplatParameter<'tree>>),
}
impl<'tree> ::treesitter_types::FromNode<'tree> for FindPatternChildren<'tree> {
    #[allow(clippy::collapsible_else_if)]
    fn from_node(
        node: ::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        match node.kind() {
            "splat_parameter" => Ok(Self::SplatParameter(::std::boxed::Box::new(
                ::treesitter_types::maybe_grow_stack(|| {
                    <SplatParameter as ::treesitter_types::FromNode>::from_node(node, src)
                })?,
            ))),
            _other => {
                if let Ok(v) = ::treesitter_types::maybe_grow_stack(|| {
                    <PatternExpr as ::treesitter_types::FromNode>::from_node(node, src)
                }) {
                    Ok(Self::PatternExpr(::std::boxed::Box::new(v)))
                } else {
                    Err(::treesitter_types::ParseError::unexpected_kind(
                        _other, node,
                    ))
                }
            }
        }
    }
}
impl ::treesitter_types::Spanned for FindPatternChildren<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        match self {
            Self::PatternExpr(inner) => inner.span(),
            Self::SplatParameter(inner) => inner.span(),
        }
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ForPattern<'tree> {
    Lhs(::std::boxed::Box<Lhs<'tree>>),
    LeftAssignmentList(::std::boxed::Box<LeftAssignmentList<'tree>>),
}
impl<'tree> ::treesitter_types::FromNode<'tree> for ForPattern<'tree> {
    #[allow(clippy::collapsible_else_if)]
    fn from_node(
        node: ::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        match node.kind() {
            "left_assignment_list" => Ok(Self::LeftAssignmentList(::std::boxed::Box::new(
                ::treesitter_types::maybe_grow_stack(|| {
                    <LeftAssignmentList as ::treesitter_types::FromNode>::from_node(node, src)
                })?,
            ))),
            _other => {
                if let Ok(v) = ::treesitter_types::maybe_grow_stack(|| {
                    <Lhs as ::treesitter_types::FromNode>::from_node(node, src)
                }) {
                    Ok(Self::Lhs(::std::boxed::Box::new(v)))
                } else {
                    Err(::treesitter_types::ParseError::unexpected_kind(
                        _other, node,
                    ))
                }
            }
        }
    }
}
impl ::treesitter_types::Spanned for ForPattern<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        match self {
            Self::Lhs(inner) => inner.span(),
            Self::LeftAssignmentList(inner) => inner.span(),
        }
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum HashChildren<'tree> {
    HashSplatArgument(::std::boxed::Box<HashSplatArgument<'tree>>),
    Pair(::std::boxed::Box<Pair<'tree>>),
}
impl<'tree> ::treesitter_types::FromNode<'tree> for HashChildren<'tree> {
    #[allow(clippy::collapsible_else_if)]
    fn from_node(
        node: ::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        match node.kind() {
            "hash_splat_argument" => Ok(Self::HashSplatArgument(::std::boxed::Box::new(
                ::treesitter_types::maybe_grow_stack(|| {
                    <HashSplatArgument as ::treesitter_types::FromNode>::from_node(node, src)
                })?,
            ))),
            "pair" => Ok(Self::Pair(::std::boxed::Box::new(
                ::treesitter_types::maybe_grow_stack(|| {
                    <Pair as ::treesitter_types::FromNode>::from_node(node, src)
                })?,
            ))),
            other => Err(::treesitter_types::ParseError::unexpected_kind(other, node)),
        }
    }
}
impl ::treesitter_types::Spanned for HashChildren<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        match self {
            Self::HashSplatArgument(inner) => inner.span(),
            Self::Pair(inner) => inner.span(),
        }
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum HashPatternChildren<'tree> {
    HashSplatNil(::std::boxed::Box<HashSplatNil<'tree>>),
    HashSplatParameter(::std::boxed::Box<HashSplatParameter<'tree>>),
    KeywordPattern(::std::boxed::Box<KeywordPattern<'tree>>),
}
impl<'tree> ::treesitter_types::FromNode<'tree> for HashPatternChildren<'tree> {
    #[allow(clippy::collapsible_else_if)]
    fn from_node(
        node: ::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        match node.kind() {
            "hash_splat_nil" => Ok(Self::HashSplatNil(::std::boxed::Box::new(
                ::treesitter_types::maybe_grow_stack(|| {
                    <HashSplatNil as ::treesitter_types::FromNode>::from_node(node, src)
                })?,
            ))),
            "hash_splat_parameter" => Ok(Self::HashSplatParameter(::std::boxed::Box::new(
                ::treesitter_types::maybe_grow_stack(|| {
                    <HashSplatParameter as ::treesitter_types::FromNode>::from_node(node, src)
                })?,
            ))),
            "keyword_pattern" => Ok(Self::KeywordPattern(::std::boxed::Box::new(
                ::treesitter_types::maybe_grow_stack(|| {
                    <KeywordPattern as ::treesitter_types::FromNode>::from_node(node, src)
                })?,
            ))),
            other => Err(::treesitter_types::ParseError::unexpected_kind(other, node)),
        }
    }
}
impl ::treesitter_types::Spanned for HashPatternChildren<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        match self {
            Self::HashSplatNil(inner) => inner.span(),
            Self::HashSplatParameter(inner) => inner.span(),
            Self::KeywordPattern(inner) => inner.span(),
        }
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum HeredocBodyChildren<'tree> {
    EscapeSequence(::std::boxed::Box<EscapeSequence<'tree>>),
    HeredocContent(::std::boxed::Box<HeredocContent<'tree>>),
    HeredocEnd(::std::boxed::Box<HeredocEnd<'tree>>),
    Interpolation(::std::boxed::Box<Interpolation<'tree>>),
}
impl<'tree> ::treesitter_types::FromNode<'tree> for HeredocBodyChildren<'tree> {
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
            "heredoc_content" => Ok(Self::HeredocContent(::std::boxed::Box::new(
                ::treesitter_types::maybe_grow_stack(|| {
                    <HeredocContent as ::treesitter_types::FromNode>::from_node(node, src)
                })?,
            ))),
            "heredoc_end" => Ok(Self::HeredocEnd(::std::boxed::Box::new(
                ::treesitter_types::maybe_grow_stack(|| {
                    <HeredocEnd as ::treesitter_types::FromNode>::from_node(node, src)
                })?,
            ))),
            "interpolation" => Ok(Self::Interpolation(::std::boxed::Box::new(
                ::treesitter_types::maybe_grow_stack(|| {
                    <Interpolation as ::treesitter_types::FromNode>::from_node(node, src)
                })?,
            ))),
            other => Err(::treesitter_types::ParseError::unexpected_kind(other, node)),
        }
    }
}
impl ::treesitter_types::Spanned for HeredocBodyChildren<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        match self {
            Self::EscapeSequence(inner) => inner.span(),
            Self::HeredocContent(inner) => inner.span(),
            Self::HeredocEnd(inner) => inner.span(),
            Self::Interpolation(inner) => inner.span(),
        }
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum IfAlternative<'tree> {
    Else(::std::boxed::Box<Else<'tree>>),
    Elsif(::std::boxed::Box<Elsif<'tree>>),
}
impl<'tree> ::treesitter_types::FromNode<'tree> for IfAlternative<'tree> {
    #[allow(clippy::collapsible_else_if)]
    fn from_node(
        node: ::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        match node.kind() {
            "else" => Ok(Self::Else(::std::boxed::Box::new(
                ::treesitter_types::maybe_grow_stack(|| {
                    <Else as ::treesitter_types::FromNode>::from_node(node, src)
                })?,
            ))),
            "elsif" => Ok(Self::Elsif(::std::boxed::Box::new(
                ::treesitter_types::maybe_grow_stack(|| {
                    <Elsif as ::treesitter_types::FromNode>::from_node(node, src)
                })?,
            ))),
            other => Err(::treesitter_types::ParseError::unexpected_kind(other, node)),
        }
    }
}
impl ::treesitter_types::Spanned for IfAlternative<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        match self {
            Self::Else(inner) => inner.span(),
            Self::Elsif(inner) => inner.span(),
        }
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum InClauseGuard<'tree> {
    IfGuard(::std::boxed::Box<IfGuard<'tree>>),
    UnlessGuard(::std::boxed::Box<UnlessGuard<'tree>>),
}
impl<'tree> ::treesitter_types::FromNode<'tree> for InClauseGuard<'tree> {
    #[allow(clippy::collapsible_else_if)]
    fn from_node(
        node: ::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        match node.kind() {
            "if_guard" => Ok(Self::IfGuard(::std::boxed::Box::new(
                ::treesitter_types::maybe_grow_stack(|| {
                    <IfGuard as ::treesitter_types::FromNode>::from_node(node, src)
                })?,
            ))),
            "unless_guard" => Ok(Self::UnlessGuard(::std::boxed::Box::new(
                ::treesitter_types::maybe_grow_stack(|| {
                    <UnlessGuard as ::treesitter_types::FromNode>::from_node(node, src)
                })?,
            ))),
            other => Err(::treesitter_types::ParseError::unexpected_kind(other, node)),
        }
    }
}
impl ::treesitter_types::Spanned for InClauseGuard<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        match self {
            Self::IfGuard(inner) => inner.span(),
            Self::UnlessGuard(inner) => inner.span(),
        }
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum InterpolationChildren<'tree> {
    NonlocalVariable(::std::boxed::Box<NonlocalVariable<'tree>>),
    Statement(::std::boxed::Box<Statement<'tree>>),
    EmptyStatement(::std::boxed::Box<EmptyStatement<'tree>>),
}
impl<'tree> ::treesitter_types::FromNode<'tree> for InterpolationChildren<'tree> {
    #[allow(clippy::collapsible_else_if)]
    fn from_node(
        node: ::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        match node.kind() {
            "empty_statement" => Ok(Self::EmptyStatement(::std::boxed::Box::new(
                ::treesitter_types::maybe_grow_stack(|| {
                    <EmptyStatement as ::treesitter_types::FromNode>::from_node(node, src)
                })?,
            ))),
            _other => {
                if let Ok(v) = ::treesitter_types::maybe_grow_stack(|| {
                    <NonlocalVariable as ::treesitter_types::FromNode>::from_node(node, src)
                }) {
                    Ok(Self::NonlocalVariable(::std::boxed::Box::new(v)))
                } else {
                    if let Ok(v) = ::treesitter_types::maybe_grow_stack(|| {
                        <Statement as ::treesitter_types::FromNode>::from_node(node, src)
                    }) {
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
}
impl ::treesitter_types::Spanned for InterpolationChildren<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        match self {
            Self::NonlocalVariable(inner) => inner.span(),
            Self::Statement(inner) => inner.span(),
            Self::EmptyStatement(inner) => inner.span(),
        }
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum KeywordPatternKey<'tree> {
    HashKeySymbol(::std::boxed::Box<HashKeySymbol<'tree>>),
    String(::std::boxed::Box<String<'tree>>),
}
impl<'tree> ::treesitter_types::FromNode<'tree> for KeywordPatternKey<'tree> {
    #[allow(clippy::collapsible_else_if)]
    fn from_node(
        node: ::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        match node.kind() {
            "hash_key_symbol" => Ok(Self::HashKeySymbol(::std::boxed::Box::new(
                ::treesitter_types::maybe_grow_stack(|| {
                    <HashKeySymbol as ::treesitter_types::FromNode>::from_node(node, src)
                })?,
            ))),
            "string" => Ok(Self::String(::std::boxed::Box::new(
                ::treesitter_types::maybe_grow_stack(|| {
                    <String as ::treesitter_types::FromNode>::from_node(node, src)
                })?,
            ))),
            other => Err(::treesitter_types::ParseError::unexpected_kind(other, node)),
        }
    }
}
impl ::treesitter_types::Spanned for KeywordPatternKey<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        match self {
            Self::HashKeySymbol(inner) => inner.span(),
            Self::String(inner) => inner.span(),
        }
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum LambdaBody<'tree> {
    Block(::std::boxed::Box<Block<'tree>>),
    DoBlock(::std::boxed::Box<DoBlock<'tree>>),
}
impl<'tree> ::treesitter_types::FromNode<'tree> for LambdaBody<'tree> {
    #[allow(clippy::collapsible_else_if)]
    fn from_node(
        node: ::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        match node.kind() {
            "block" => Ok(Self::Block(::std::boxed::Box::new(
                ::treesitter_types::maybe_grow_stack(|| {
                    <Block as ::treesitter_types::FromNode>::from_node(node, src)
                })?,
            ))),
            "do_block" => Ok(Self::DoBlock(::std::boxed::Box::new(
                ::treesitter_types::maybe_grow_stack(|| {
                    <DoBlock as ::treesitter_types::FromNode>::from_node(node, src)
                })?,
            ))),
            other => Err(::treesitter_types::ParseError::unexpected_kind(other, node)),
        }
    }
}
impl ::treesitter_types::Spanned for LambdaBody<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        match self {
            Self::Block(inner) => inner.span(),
            Self::DoBlock(inner) => inner.span(),
        }
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum LambdaParametersChildren<'tree> {
    BlockParameter(::std::boxed::Box<BlockParameter<'tree>>),
    DestructuredParameter(::std::boxed::Box<DestructuredParameter<'tree>>),
    ForwardParameter(::std::boxed::Box<ForwardParameter<'tree>>),
    HashSplatNil(::std::boxed::Box<HashSplatNil<'tree>>),
    HashSplatParameter(::std::boxed::Box<HashSplatParameter<'tree>>),
    Identifier(::std::boxed::Box<Identifier<'tree>>),
    KeywordParameter(::std::boxed::Box<KeywordParameter<'tree>>),
    OptionalParameter(::std::boxed::Box<OptionalParameter<'tree>>),
    SplatParameter(::std::boxed::Box<SplatParameter<'tree>>),
}
impl<'tree> ::treesitter_types::FromNode<'tree> for LambdaParametersChildren<'tree> {
    #[allow(clippy::collapsible_else_if)]
    fn from_node(
        node: ::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        match node.kind() {
            "block_parameter" => Ok(Self::BlockParameter(::std::boxed::Box::new(
                ::treesitter_types::maybe_grow_stack(|| {
                    <BlockParameter as ::treesitter_types::FromNode>::from_node(node, src)
                })?,
            ))),
            "destructured_parameter" => Ok(Self::DestructuredParameter(::std::boxed::Box::new(
                ::treesitter_types::maybe_grow_stack(|| {
                    <DestructuredParameter as ::treesitter_types::FromNode>::from_node(node, src)
                })?,
            ))),
            "forward_parameter" => Ok(Self::ForwardParameter(::std::boxed::Box::new(
                ::treesitter_types::maybe_grow_stack(|| {
                    <ForwardParameter as ::treesitter_types::FromNode>::from_node(node, src)
                })?,
            ))),
            "hash_splat_nil" => Ok(Self::HashSplatNil(::std::boxed::Box::new(
                ::treesitter_types::maybe_grow_stack(|| {
                    <HashSplatNil as ::treesitter_types::FromNode>::from_node(node, src)
                })?,
            ))),
            "hash_splat_parameter" => Ok(Self::HashSplatParameter(::std::boxed::Box::new(
                ::treesitter_types::maybe_grow_stack(|| {
                    <HashSplatParameter as ::treesitter_types::FromNode>::from_node(node, src)
                })?,
            ))),
            "identifier" => Ok(Self::Identifier(::std::boxed::Box::new(
                ::treesitter_types::maybe_grow_stack(|| {
                    <Identifier as ::treesitter_types::FromNode>::from_node(node, src)
                })?,
            ))),
            "keyword_parameter" => Ok(Self::KeywordParameter(::std::boxed::Box::new(
                ::treesitter_types::maybe_grow_stack(|| {
                    <KeywordParameter as ::treesitter_types::FromNode>::from_node(node, src)
                })?,
            ))),
            "optional_parameter" => Ok(Self::OptionalParameter(::std::boxed::Box::new(
                ::treesitter_types::maybe_grow_stack(|| {
                    <OptionalParameter as ::treesitter_types::FromNode>::from_node(node, src)
                })?,
            ))),
            "splat_parameter" => Ok(Self::SplatParameter(::std::boxed::Box::new(
                ::treesitter_types::maybe_grow_stack(|| {
                    <SplatParameter as ::treesitter_types::FromNode>::from_node(node, src)
                })?,
            ))),
            other => Err(::treesitter_types::ParseError::unexpected_kind(other, node)),
        }
    }
}
impl ::treesitter_types::Spanned for LambdaParametersChildren<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        match self {
            Self::BlockParameter(inner) => inner.span(),
            Self::DestructuredParameter(inner) => inner.span(),
            Self::ForwardParameter(inner) => inner.span(),
            Self::HashSplatNil(inner) => inner.span(),
            Self::HashSplatParameter(inner) => inner.span(),
            Self::Identifier(inner) => inner.span(),
            Self::KeywordParameter(inner) => inner.span(),
            Self::OptionalParameter(inner) => inner.span(),
            Self::SplatParameter(inner) => inner.span(),
        }
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum LeftAssignmentListChildren<'tree> {
    Lhs(::std::boxed::Box<Lhs<'tree>>),
    DestructuredLeftAssignment(::std::boxed::Box<DestructuredLeftAssignment<'tree>>),
    RestAssignment(::std::boxed::Box<RestAssignment<'tree>>),
}
impl<'tree> ::treesitter_types::FromNode<'tree> for LeftAssignmentListChildren<'tree> {
    #[allow(clippy::collapsible_else_if)]
    fn from_node(
        node: ::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        match node.kind() {
            "destructured_left_assignment" => Ok(Self::DestructuredLeftAssignment(
                ::std::boxed::Box::new(::treesitter_types::maybe_grow_stack(|| {
                    <DestructuredLeftAssignment as ::treesitter_types::FromNode>::from_node(
                        node, src,
                    )
                })?),
            )),
            "rest_assignment" => Ok(Self::RestAssignment(::std::boxed::Box::new(
                ::treesitter_types::maybe_grow_stack(|| {
                    <RestAssignment as ::treesitter_types::FromNode>::from_node(node, src)
                })?,
            ))),
            _other => {
                if let Ok(v) = ::treesitter_types::maybe_grow_stack(|| {
                    <Lhs as ::treesitter_types::FromNode>::from_node(node, src)
                }) {
                    Ok(Self::Lhs(::std::boxed::Box::new(v)))
                } else {
                    Err(::treesitter_types::ParseError::unexpected_kind(
                        _other, node,
                    ))
                }
            }
        }
    }
}
impl ::treesitter_types::Spanned for LeftAssignmentListChildren<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        match self {
            Self::Lhs(inner) => inner.span(),
            Self::DestructuredLeftAssignment(inner) => inner.span(),
            Self::RestAssignment(inner) => inner.span(),
        }
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum MethodBody<'tree> {
    Arg(::std::boxed::Box<Arg<'tree>>),
    BodyStatement(::std::boxed::Box<BodyStatement<'tree>>),
    RescueModifier(::std::boxed::Box<RescueModifier<'tree>>),
}
impl<'tree> ::treesitter_types::FromNode<'tree> for MethodBody<'tree> {
    #[allow(clippy::collapsible_else_if)]
    fn from_node(
        node: ::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        match node.kind() {
            "body_statement" => Ok(Self::BodyStatement(::std::boxed::Box::new(
                ::treesitter_types::maybe_grow_stack(|| {
                    <BodyStatement as ::treesitter_types::FromNode>::from_node(node, src)
                })?,
            ))),
            "rescue_modifier" => Ok(Self::RescueModifier(::std::boxed::Box::new(
                ::treesitter_types::maybe_grow_stack(|| {
                    <RescueModifier as ::treesitter_types::FromNode>::from_node(node, src)
                })?,
            ))),
            _other => {
                if let Ok(v) = ::treesitter_types::maybe_grow_stack(|| {
                    <Arg as ::treesitter_types::FromNode>::from_node(node, src)
                }) {
                    Ok(Self::Arg(::std::boxed::Box::new(v)))
                } else {
                    Err(::treesitter_types::ParseError::unexpected_kind(
                        _other, node,
                    ))
                }
            }
        }
    }
}
impl ::treesitter_types::Spanned for MethodBody<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        match self {
            Self::Arg(inner) => inner.span(),
            Self::BodyStatement(inner) => inner.span(),
            Self::RescueModifier(inner) => inner.span(),
        }
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum MethodParametersChildren<'tree> {
    BlockParameter(::std::boxed::Box<BlockParameter<'tree>>),
    DestructuredParameter(::std::boxed::Box<DestructuredParameter<'tree>>),
    ForwardParameter(::std::boxed::Box<ForwardParameter<'tree>>),
    HashSplatNil(::std::boxed::Box<HashSplatNil<'tree>>),
    HashSplatParameter(::std::boxed::Box<HashSplatParameter<'tree>>),
    Identifier(::std::boxed::Box<Identifier<'tree>>),
    KeywordParameter(::std::boxed::Box<KeywordParameter<'tree>>),
    OptionalParameter(::std::boxed::Box<OptionalParameter<'tree>>),
    SplatParameter(::std::boxed::Box<SplatParameter<'tree>>),
}
impl<'tree> ::treesitter_types::FromNode<'tree> for MethodParametersChildren<'tree> {
    #[allow(clippy::collapsible_else_if)]
    fn from_node(
        node: ::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        match node.kind() {
            "block_parameter" => Ok(Self::BlockParameter(::std::boxed::Box::new(
                ::treesitter_types::maybe_grow_stack(|| {
                    <BlockParameter as ::treesitter_types::FromNode>::from_node(node, src)
                })?,
            ))),
            "destructured_parameter" => Ok(Self::DestructuredParameter(::std::boxed::Box::new(
                ::treesitter_types::maybe_grow_stack(|| {
                    <DestructuredParameter as ::treesitter_types::FromNode>::from_node(node, src)
                })?,
            ))),
            "forward_parameter" => Ok(Self::ForwardParameter(::std::boxed::Box::new(
                ::treesitter_types::maybe_grow_stack(|| {
                    <ForwardParameter as ::treesitter_types::FromNode>::from_node(node, src)
                })?,
            ))),
            "hash_splat_nil" => Ok(Self::HashSplatNil(::std::boxed::Box::new(
                ::treesitter_types::maybe_grow_stack(|| {
                    <HashSplatNil as ::treesitter_types::FromNode>::from_node(node, src)
                })?,
            ))),
            "hash_splat_parameter" => Ok(Self::HashSplatParameter(::std::boxed::Box::new(
                ::treesitter_types::maybe_grow_stack(|| {
                    <HashSplatParameter as ::treesitter_types::FromNode>::from_node(node, src)
                })?,
            ))),
            "identifier" => Ok(Self::Identifier(::std::boxed::Box::new(
                ::treesitter_types::maybe_grow_stack(|| {
                    <Identifier as ::treesitter_types::FromNode>::from_node(node, src)
                })?,
            ))),
            "keyword_parameter" => Ok(Self::KeywordParameter(::std::boxed::Box::new(
                ::treesitter_types::maybe_grow_stack(|| {
                    <KeywordParameter as ::treesitter_types::FromNode>::from_node(node, src)
                })?,
            ))),
            "optional_parameter" => Ok(Self::OptionalParameter(::std::boxed::Box::new(
                ::treesitter_types::maybe_grow_stack(|| {
                    <OptionalParameter as ::treesitter_types::FromNode>::from_node(node, src)
                })?,
            ))),
            "splat_parameter" => Ok(Self::SplatParameter(::std::boxed::Box::new(
                ::treesitter_types::maybe_grow_stack(|| {
                    <SplatParameter as ::treesitter_types::FromNode>::from_node(node, src)
                })?,
            ))),
            other => Err(::treesitter_types::ParseError::unexpected_kind(other, node)),
        }
    }
}
impl ::treesitter_types::Spanned for MethodParametersChildren<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        match self {
            Self::BlockParameter(inner) => inner.span(),
            Self::DestructuredParameter(inner) => inner.span(),
            Self::ForwardParameter(inner) => inner.span(),
            Self::HashSplatNil(inner) => inner.span(),
            Self::HashSplatParameter(inner) => inner.span(),
            Self::Identifier(inner) => inner.span(),
            Self::KeywordParameter(inner) => inner.span(),
            Self::OptionalParameter(inner) => inner.span(),
            Self::SplatParameter(inner) => inner.span(),
        }
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ModuleName<'tree> {
    Constant(::std::boxed::Box<Constant<'tree>>),
    ScopeResolution(::std::boxed::Box<ScopeResolution<'tree>>),
}
impl<'tree> ::treesitter_types::FromNode<'tree> for ModuleName<'tree> {
    #[allow(clippy::collapsible_else_if)]
    fn from_node(
        node: ::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        match node.kind() {
            "constant" => Ok(Self::Constant(::std::boxed::Box::new(
                ::treesitter_types::maybe_grow_stack(|| {
                    <Constant as ::treesitter_types::FromNode>::from_node(node, src)
                })?,
            ))),
            "scope_resolution" => Ok(Self::ScopeResolution(::std::boxed::Box::new(
                ::treesitter_types::maybe_grow_stack(|| {
                    <ScopeResolution as ::treesitter_types::FromNode>::from_node(node, src)
                })?,
            ))),
            other => Err(::treesitter_types::ParseError::unexpected_kind(other, node)),
        }
    }
}
impl ::treesitter_types::Spanned for ModuleName<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        match self {
            Self::Constant(inner) => inner.span(),
            Self::ScopeResolution(inner) => inner.span(),
        }
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum OperatorAssignmentOperator {
    PercentEq(::treesitter_types::Span),
    AmpAmpEq(::treesitter_types::Span),
    AmpEq(::treesitter_types::Span),
    StarStarEq(::treesitter_types::Span),
    StarEq(::treesitter_types::Span),
    PlusEq(::treesitter_types::Span),
    MinusEq(::treesitter_types::Span),
    SlashEq(::treesitter_types::Span),
    ShlEq(::treesitter_types::Span),
    ShrEq(::treesitter_types::Span),
    CaretEq(::treesitter_types::Span),
    PipeEq(::treesitter_types::Span),
    PipePipeEq(::treesitter_types::Span),
}
impl<'tree> ::treesitter_types::FromNode<'tree> for OperatorAssignmentOperator {
    #[allow(clippy::collapsible_else_if)]
    fn from_node(
        node: ::tree_sitter::Node<'tree>,
        _src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        match node.kind() {
            "%=" => Ok(Self::PercentEq(::treesitter_types::Span::from(node))),
            "&&=" => Ok(Self::AmpAmpEq(::treesitter_types::Span::from(node))),
            "&=" => Ok(Self::AmpEq(::treesitter_types::Span::from(node))),
            "**=" => Ok(Self::StarStarEq(::treesitter_types::Span::from(node))),
            "*=" => Ok(Self::StarEq(::treesitter_types::Span::from(node))),
            "+=" => Ok(Self::PlusEq(::treesitter_types::Span::from(node))),
            "-=" => Ok(Self::MinusEq(::treesitter_types::Span::from(node))),
            "/=" => Ok(Self::SlashEq(::treesitter_types::Span::from(node))),
            "<<=" => Ok(Self::ShlEq(::treesitter_types::Span::from(node))),
            ">>=" => Ok(Self::ShrEq(::treesitter_types::Span::from(node))),
            "^=" => Ok(Self::CaretEq(::treesitter_types::Span::from(node))),
            "|=" => Ok(Self::PipeEq(::treesitter_types::Span::from(node))),
            "||=" => Ok(Self::PipePipeEq(::treesitter_types::Span::from(node))),
            other => Err(::treesitter_types::ParseError::unexpected_kind(other, node)),
        }
    }
}
impl ::treesitter_types::Spanned for OperatorAssignmentOperator {
    fn span(&self) -> ::treesitter_types::Span {
        match self {
            Self::PercentEq(span) => *span,
            Self::AmpAmpEq(span) => *span,
            Self::AmpEq(span) => *span,
            Self::StarStarEq(span) => *span,
            Self::StarEq(span) => *span,
            Self::PlusEq(span) => *span,
            Self::MinusEq(span) => *span,
            Self::SlashEq(span) => *span,
            Self::ShlEq(span) => *span,
            Self::ShrEq(span) => *span,
            Self::CaretEq(span) => *span,
            Self::PipeEq(span) => *span,
            Self::PipePipeEq(span) => *span,
        }
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum OperatorAssignmentRight<'tree> {
    Expression(::std::boxed::Box<Expression<'tree>>),
    RescueModifier(::std::boxed::Box<RescueModifier<'tree>>),
}
impl<'tree> ::treesitter_types::FromNode<'tree> for OperatorAssignmentRight<'tree> {
    #[allow(clippy::collapsible_else_if)]
    fn from_node(
        node: ::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        match node.kind() {
            "rescue_modifier" => Ok(Self::RescueModifier(::std::boxed::Box::new(
                ::treesitter_types::maybe_grow_stack(|| {
                    <RescueModifier as ::treesitter_types::FromNode>::from_node(node, src)
                })?,
            ))),
            _other => {
                if let Ok(v) = ::treesitter_types::maybe_grow_stack(|| {
                    <Expression as ::treesitter_types::FromNode>::from_node(node, src)
                }) {
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
impl ::treesitter_types::Spanned for OperatorAssignmentRight<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        match self {
            Self::Expression(inner) => inner.span(),
            Self::RescueModifier(inner) => inner.span(),
        }
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum PairKey<'tree> {
    Arg(::std::boxed::Box<Arg<'tree>>),
    HashKeySymbol(::std::boxed::Box<HashKeySymbol<'tree>>),
    String(::std::boxed::Box<String<'tree>>),
}
impl<'tree> ::treesitter_types::FromNode<'tree> for PairKey<'tree> {
    #[allow(clippy::collapsible_else_if)]
    fn from_node(
        node: ::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        match node.kind() {
            "hash_key_symbol" => Ok(Self::HashKeySymbol(::std::boxed::Box::new(
                ::treesitter_types::maybe_grow_stack(|| {
                    <HashKeySymbol as ::treesitter_types::FromNode>::from_node(node, src)
                })?,
            ))),
            "string" => Ok(Self::String(::std::boxed::Box::new(
                ::treesitter_types::maybe_grow_stack(|| {
                    <String as ::treesitter_types::FromNode>::from_node(node, src)
                })?,
            ))),
            _other => {
                if let Ok(v) = ::treesitter_types::maybe_grow_stack(|| {
                    <Arg as ::treesitter_types::FromNode>::from_node(node, src)
                }) {
                    Ok(Self::Arg(::std::boxed::Box::new(v)))
                } else {
                    Err(::treesitter_types::ParseError::unexpected_kind(
                        _other, node,
                    ))
                }
            }
        }
    }
}
impl ::treesitter_types::Spanned for PairKey<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        match self {
            Self::Arg(inner) => inner.span(),
            Self::HashKeySymbol(inner) => inner.span(),
            Self::String(inner) => inner.span(),
        }
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ParenthesizedStatementsChildren<'tree> {
    Statement(::std::boxed::Box<Statement<'tree>>),
    EmptyStatement(::std::boxed::Box<EmptyStatement<'tree>>),
}
impl<'tree> ::treesitter_types::FromNode<'tree> for ParenthesizedStatementsChildren<'tree> {
    #[allow(clippy::collapsible_else_if)]
    fn from_node(
        node: ::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        match node.kind() {
            "empty_statement" => Ok(Self::EmptyStatement(::std::boxed::Box::new(
                ::treesitter_types::maybe_grow_stack(|| {
                    <EmptyStatement as ::treesitter_types::FromNode>::from_node(node, src)
                })?,
            ))),
            _other => {
                if let Ok(v) = ::treesitter_types::maybe_grow_stack(|| {
                    <Statement as ::treesitter_types::FromNode>::from_node(node, src)
                }) {
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
impl ::treesitter_types::Spanned for ParenthesizedStatementsChildren<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        match self {
            Self::Statement(inner) => inner.span(),
            Self::EmptyStatement(inner) => inner.span(),
        }
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum PatternChildren<'tree> {
    Arg(::std::boxed::Box<Arg<'tree>>),
    SplatArgument(::std::boxed::Box<SplatArgument<'tree>>),
}
impl<'tree> ::treesitter_types::FromNode<'tree> for PatternChildren<'tree> {
    #[allow(clippy::collapsible_else_if)]
    fn from_node(
        node: ::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        match node.kind() {
            "splat_argument" => Ok(Self::SplatArgument(::std::boxed::Box::new(
                ::treesitter_types::maybe_grow_stack(|| {
                    <SplatArgument as ::treesitter_types::FromNode>::from_node(node, src)
                })?,
            ))),
            _other => {
                if let Ok(v) = ::treesitter_types::maybe_grow_stack(|| {
                    <Arg as ::treesitter_types::FromNode>::from_node(node, src)
                }) {
                    Ok(Self::Arg(::std::boxed::Box::new(v)))
                } else {
                    Err(::treesitter_types::ParseError::unexpected_kind(
                        _other, node,
                    ))
                }
            }
        }
    }
}
impl ::treesitter_types::Spanned for PatternChildren<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        match self {
            Self::Arg(inner) => inner.span(),
            Self::SplatArgument(inner) => inner.span(),
        }
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ProgramChildren<'tree> {
    Statement(::std::boxed::Box<Statement<'tree>>),
    EmptyStatement(::std::boxed::Box<EmptyStatement<'tree>>),
    Uninterpreted(::std::boxed::Box<Uninterpreted<'tree>>),
}
impl<'tree> ::treesitter_types::FromNode<'tree> for ProgramChildren<'tree> {
    #[allow(clippy::collapsible_else_if)]
    fn from_node(
        node: ::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        match node.kind() {
            "empty_statement" => Ok(Self::EmptyStatement(::std::boxed::Box::new(
                ::treesitter_types::maybe_grow_stack(|| {
                    <EmptyStatement as ::treesitter_types::FromNode>::from_node(node, src)
                })?,
            ))),
            "uninterpreted" => Ok(Self::Uninterpreted(::std::boxed::Box::new(
                ::treesitter_types::maybe_grow_stack(|| {
                    <Uninterpreted as ::treesitter_types::FromNode>::from_node(node, src)
                })?,
            ))),
            _other => {
                if let Ok(v) = ::treesitter_types::maybe_grow_stack(|| {
                    <Statement as ::treesitter_types::FromNode>::from_node(node, src)
                }) {
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
impl ::treesitter_types::Spanned for ProgramChildren<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        match self {
            Self::Statement(inner) => inner.span(),
            Self::EmptyStatement(inner) => inner.span(),
            Self::Uninterpreted(inner) => inner.span(),
        }
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum RangeBegin<'tree> {
    Arg(::std::boxed::Box<Arg<'tree>>),
    PatternPrimitive(::std::boxed::Box<PatternPrimitive<'tree>>),
}
impl<'tree> ::treesitter_types::FromNode<'tree> for RangeBegin<'tree> {
    #[allow(clippy::collapsible_else_if)]
    fn from_node(
        node: ::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        if let Ok(v) = ::treesitter_types::maybe_grow_stack(|| {
            <Arg as ::treesitter_types::FromNode>::from_node(node, src)
        }) {
            Ok(Self::Arg(::std::boxed::Box::new(v)))
        } else {
            if let Ok(v) = ::treesitter_types::maybe_grow_stack(|| {
                <PatternPrimitive as ::treesitter_types::FromNode>::from_node(node, src)
            }) {
                Ok(Self::PatternPrimitive(::std::boxed::Box::new(v)))
            } else {
                Err(::treesitter_types::ParseError::unexpected_kind(
                    node.kind(),
                    node,
                ))
            }
        }
    }
}
impl ::treesitter_types::Spanned for RangeBegin<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        match self {
            Self::Arg(inner) => inner.span(),
            Self::PatternPrimitive(inner) => inner.span(),
        }
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum RangeEnd<'tree> {
    Arg(::std::boxed::Box<Arg<'tree>>),
    PatternPrimitive(::std::boxed::Box<PatternPrimitive<'tree>>),
}
impl<'tree> ::treesitter_types::FromNode<'tree> for RangeEnd<'tree> {
    #[allow(clippy::collapsible_else_if)]
    fn from_node(
        node: ::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        if let Ok(v) = ::treesitter_types::maybe_grow_stack(|| {
            <Arg as ::treesitter_types::FromNode>::from_node(node, src)
        }) {
            Ok(Self::Arg(::std::boxed::Box::new(v)))
        } else {
            if let Ok(v) = ::treesitter_types::maybe_grow_stack(|| {
                <PatternPrimitive as ::treesitter_types::FromNode>::from_node(node, src)
            }) {
                Ok(Self::PatternPrimitive(::std::boxed::Box::new(v)))
            } else {
                Err(::treesitter_types::ParseError::unexpected_kind(
                    node.kind(),
                    node,
                ))
            }
        }
    }
}
impl ::treesitter_types::Spanned for RangeEnd<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        match self {
            Self::Arg(inner) => inner.span(),
            Self::PatternPrimitive(inner) => inner.span(),
        }
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum RangeOperator {
    DotDot(::treesitter_types::Span),
    Ellipsis(::treesitter_types::Span),
}
impl<'tree> ::treesitter_types::FromNode<'tree> for RangeOperator {
    #[allow(clippy::collapsible_else_if)]
    fn from_node(
        node: ::tree_sitter::Node<'tree>,
        _src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        match node.kind() {
            ".." => Ok(Self::DotDot(::treesitter_types::Span::from(node))),
            "..." => Ok(Self::Ellipsis(::treesitter_types::Span::from(node))),
            other => Err(::treesitter_types::ParseError::unexpected_kind(other, node)),
        }
    }
}
impl ::treesitter_types::Spanned for RangeOperator {
    fn span(&self) -> ::treesitter_types::Span {
        match self {
            Self::DotDot(span) => *span,
            Self::Ellipsis(span) => *span,
        }
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum RationalChildren<'tree> {
    Float(::std::boxed::Box<Float<'tree>>),
    Integer(::std::boxed::Box<Integer<'tree>>),
}
impl<'tree> ::treesitter_types::FromNode<'tree> for RationalChildren<'tree> {
    #[allow(clippy::collapsible_else_if)]
    fn from_node(
        node: ::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        match node.kind() {
            "float" => Ok(Self::Float(::std::boxed::Box::new(
                ::treesitter_types::maybe_grow_stack(|| {
                    <Float as ::treesitter_types::FromNode>::from_node(node, src)
                })?,
            ))),
            "integer" => Ok(Self::Integer(::std::boxed::Box::new(
                ::treesitter_types::maybe_grow_stack(|| {
                    <Integer as ::treesitter_types::FromNode>::from_node(node, src)
                })?,
            ))),
            other => Err(::treesitter_types::ParseError::unexpected_kind(other, node)),
        }
    }
}
impl ::treesitter_types::Spanned for RationalChildren<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        match self {
            Self::Float(inner) => inner.span(),
            Self::Integer(inner) => inner.span(),
        }
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum RegexChildren<'tree> {
    EscapeSequence(::std::boxed::Box<EscapeSequence<'tree>>),
    Interpolation(::std::boxed::Box<Interpolation<'tree>>),
    StringContent(::std::boxed::Box<StringContent<'tree>>),
}
impl<'tree> ::treesitter_types::FromNode<'tree> for RegexChildren<'tree> {
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
            "interpolation" => Ok(Self::Interpolation(::std::boxed::Box::new(
                ::treesitter_types::maybe_grow_stack(|| {
                    <Interpolation as ::treesitter_types::FromNode>::from_node(node, src)
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
impl ::treesitter_types::Spanned for RegexChildren<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        match self {
            Self::EscapeSequence(inner) => inner.span(),
            Self::Interpolation(inner) => inner.span(),
            Self::StringContent(inner) => inner.span(),
        }
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum RescueModifierBody<'tree> {
    Arg(::std::boxed::Box<Arg<'tree>>),
    Statement(::std::boxed::Box<Statement<'tree>>),
}
impl<'tree> ::treesitter_types::FromNode<'tree> for RescueModifierBody<'tree> {
    #[allow(clippy::collapsible_else_if)]
    fn from_node(
        node: ::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        if let Ok(v) = ::treesitter_types::maybe_grow_stack(|| {
            <Arg as ::treesitter_types::FromNode>::from_node(node, src)
        }) {
            Ok(Self::Arg(::std::boxed::Box::new(v)))
        } else {
            if let Ok(v) = ::treesitter_types::maybe_grow_stack(|| {
                <Statement as ::treesitter_types::FromNode>::from_node(node, src)
            }) {
                Ok(Self::Statement(::std::boxed::Box::new(v)))
            } else {
                Err(::treesitter_types::ParseError::unexpected_kind(
                    node.kind(),
                    node,
                ))
            }
        }
    }
}
impl ::treesitter_types::Spanned for RescueModifierBody<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        match self {
            Self::Arg(inner) => inner.span(),
            Self::Statement(inner) => inner.span(),
        }
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum RightAssignmentListChildren<'tree> {
    Arg(::std::boxed::Box<Arg<'tree>>),
    SplatArgument(::std::boxed::Box<SplatArgument<'tree>>),
}
impl<'tree> ::treesitter_types::FromNode<'tree> for RightAssignmentListChildren<'tree> {
    #[allow(clippy::collapsible_else_if)]
    fn from_node(
        node: ::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        match node.kind() {
            "splat_argument" => Ok(Self::SplatArgument(::std::boxed::Box::new(
                ::treesitter_types::maybe_grow_stack(|| {
                    <SplatArgument as ::treesitter_types::FromNode>::from_node(node, src)
                })?,
            ))),
            _other => {
                if let Ok(v) = ::treesitter_types::maybe_grow_stack(|| {
                    <Arg as ::treesitter_types::FromNode>::from_node(node, src)
                }) {
                    Ok(Self::Arg(::std::boxed::Box::new(v)))
                } else {
                    Err(::treesitter_types::ParseError::unexpected_kind(
                        _other, node,
                    ))
                }
            }
        }
    }
}
impl ::treesitter_types::Spanned for RightAssignmentListChildren<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        match self {
            Self::Arg(inner) => inner.span(),
            Self::SplatArgument(inner) => inner.span(),
        }
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ScopeResolutionScope<'tree> {
    PatternConstant(::std::boxed::Box<PatternConstant<'tree>>),
    Primary(::std::boxed::Box<Primary<'tree>>),
}
impl<'tree> ::treesitter_types::FromNode<'tree> for ScopeResolutionScope<'tree> {
    #[allow(clippy::collapsible_else_if)]
    fn from_node(
        node: ::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        if let Ok(v) = ::treesitter_types::maybe_grow_stack(|| {
            <PatternConstant as ::treesitter_types::FromNode>::from_node(node, src)
        }) {
            Ok(Self::PatternConstant(::std::boxed::Box::new(v)))
        } else {
            if let Ok(v) = ::treesitter_types::maybe_grow_stack(|| {
                <Primary as ::treesitter_types::FromNode>::from_node(node, src)
            }) {
                Ok(Self::Primary(::std::boxed::Box::new(v)))
            } else {
                Err(::treesitter_types::ParseError::unexpected_kind(
                    node.kind(),
                    node,
                ))
            }
        }
    }
}
impl ::treesitter_types::Spanned for ScopeResolutionScope<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        match self {
            Self::PatternConstant(inner) => inner.span(),
            Self::Primary(inner) => inner.span(),
        }
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum SingletonMethodBody<'tree> {
    Arg(::std::boxed::Box<Arg<'tree>>),
    BodyStatement(::std::boxed::Box<BodyStatement<'tree>>),
    RescueModifier(::std::boxed::Box<RescueModifier<'tree>>),
}
impl<'tree> ::treesitter_types::FromNode<'tree> for SingletonMethodBody<'tree> {
    #[allow(clippy::collapsible_else_if)]
    fn from_node(
        node: ::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        match node.kind() {
            "body_statement" => Ok(Self::BodyStatement(::std::boxed::Box::new(
                ::treesitter_types::maybe_grow_stack(|| {
                    <BodyStatement as ::treesitter_types::FromNode>::from_node(node, src)
                })?,
            ))),
            "rescue_modifier" => Ok(Self::RescueModifier(::std::boxed::Box::new(
                ::treesitter_types::maybe_grow_stack(|| {
                    <RescueModifier as ::treesitter_types::FromNode>::from_node(node, src)
                })?,
            ))),
            _other => {
                if let Ok(v) = ::treesitter_types::maybe_grow_stack(|| {
                    <Arg as ::treesitter_types::FromNode>::from_node(node, src)
                }) {
                    Ok(Self::Arg(::std::boxed::Box::new(v)))
                } else {
                    Err(::treesitter_types::ParseError::unexpected_kind(
                        _other, node,
                    ))
                }
            }
        }
    }
}
impl ::treesitter_types::Spanned for SingletonMethodBody<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        match self {
            Self::Arg(inner) => inner.span(),
            Self::BodyStatement(inner) => inner.span(),
            Self::RescueModifier(inner) => inner.span(),
        }
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum SingletonMethodObject<'tree> {
    Arg(::std::boxed::Box<Arg<'tree>>),
    Variable(::std::boxed::Box<Variable<'tree>>),
}
impl<'tree> ::treesitter_types::FromNode<'tree> for SingletonMethodObject<'tree> {
    #[allow(clippy::collapsible_else_if)]
    fn from_node(
        node: ::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        if let Ok(v) = ::treesitter_types::maybe_grow_stack(|| {
            <Arg as ::treesitter_types::FromNode>::from_node(node, src)
        }) {
            Ok(Self::Arg(::std::boxed::Box::new(v)))
        } else {
            if let Ok(v) = ::treesitter_types::maybe_grow_stack(|| {
                <Variable as ::treesitter_types::FromNode>::from_node(node, src)
            }) {
                Ok(Self::Variable(::std::boxed::Box::new(v)))
            } else {
                Err(::treesitter_types::ParseError::unexpected_kind(
                    node.kind(),
                    node,
                ))
            }
        }
    }
}
impl ::treesitter_types::Spanned for SingletonMethodObject<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        match self {
            Self::Arg(inner) => inner.span(),
            Self::Variable(inner) => inner.span(),
        }
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum StringChildren<'tree> {
    EscapeSequence(::std::boxed::Box<EscapeSequence<'tree>>),
    Interpolation(::std::boxed::Box<Interpolation<'tree>>),
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
            "interpolation" => Ok(Self::Interpolation(::std::boxed::Box::new(
                ::treesitter_types::maybe_grow_stack(|| {
                    <Interpolation as ::treesitter_types::FromNode>::from_node(node, src)
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
            Self::Interpolation(inner) => inner.span(),
            Self::StringContent(inner) => inner.span(),
        }
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum SubshellChildren<'tree> {
    EscapeSequence(::std::boxed::Box<EscapeSequence<'tree>>),
    Interpolation(::std::boxed::Box<Interpolation<'tree>>),
    StringContent(::std::boxed::Box<StringContent<'tree>>),
}
impl<'tree> ::treesitter_types::FromNode<'tree> for SubshellChildren<'tree> {
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
            "interpolation" => Ok(Self::Interpolation(::std::boxed::Box::new(
                ::treesitter_types::maybe_grow_stack(|| {
                    <Interpolation as ::treesitter_types::FromNode>::from_node(node, src)
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
impl ::treesitter_types::Spanned for SubshellChildren<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        match self {
            Self::EscapeSequence(inner) => inner.span(),
            Self::Interpolation(inner) => inner.span(),
            Self::StringContent(inner) => inner.span(),
        }
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ThenChildren<'tree> {
    Statement(::std::boxed::Box<Statement<'tree>>),
    EmptyStatement(::std::boxed::Box<EmptyStatement<'tree>>),
}
impl<'tree> ::treesitter_types::FromNode<'tree> for ThenChildren<'tree> {
    #[allow(clippy::collapsible_else_if)]
    fn from_node(
        node: ::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        match node.kind() {
            "empty_statement" => Ok(Self::EmptyStatement(::std::boxed::Box::new(
                ::treesitter_types::maybe_grow_stack(|| {
                    <EmptyStatement as ::treesitter_types::FromNode>::from_node(node, src)
                })?,
            ))),
            _other => {
                if let Ok(v) = ::treesitter_types::maybe_grow_stack(|| {
                    <Statement as ::treesitter_types::FromNode>::from_node(node, src)
                }) {
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
impl ::treesitter_types::Spanned for ThenChildren<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        match self {
            Self::Statement(inner) => inner.span(),
            Self::EmptyStatement(inner) => inner.span(),
        }
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum UnaryOperand<'tree> {
    Expression(::std::boxed::Box<Expression<'tree>>),
    SimpleNumeric(::std::boxed::Box<SimpleNumeric<'tree>>),
    ParenthesizedStatements(::std::boxed::Box<ParenthesizedStatements<'tree>>),
}
impl<'tree> ::treesitter_types::FromNode<'tree> for UnaryOperand<'tree> {
    #[allow(clippy::collapsible_else_if)]
    fn from_node(
        node: ::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        match node.kind() {
            "parenthesized_statements" => Ok(Self::ParenthesizedStatements(
                ::std::boxed::Box::new(::treesitter_types::maybe_grow_stack(|| {
                    <ParenthesizedStatements as ::treesitter_types::FromNode>::from_node(node, src)
                })?),
            )),
            _other => {
                if let Ok(v) = ::treesitter_types::maybe_grow_stack(|| {
                    <Expression as ::treesitter_types::FromNode>::from_node(node, src)
                }) {
                    Ok(Self::Expression(::std::boxed::Box::new(v)))
                } else {
                    if let Ok(v) = ::treesitter_types::maybe_grow_stack(|| {
                        <SimpleNumeric as ::treesitter_types::FromNode>::from_node(node, src)
                    }) {
                        Ok(Self::SimpleNumeric(::std::boxed::Box::new(v)))
                    } else {
                        Err(::treesitter_types::ParseError::unexpected_kind(
                            _other, node,
                        ))
                    }
                }
            }
        }
    }
}
impl ::treesitter_types::Spanned for UnaryOperand<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        match self {
            Self::Expression(inner) => inner.span(),
            Self::SimpleNumeric(inner) => inner.span(),
            Self::ParenthesizedStatements(inner) => inner.span(),
        }
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum UnaryOperator {
    Bang(::treesitter_types::Span),
    Plus(::treesitter_types::Span),
    Minus(::treesitter_types::Span),
    DEFINEDQuestion(::treesitter_types::Span),
    Not(::treesitter_types::Span),
    Tilde(::treesitter_types::Span),
}
impl<'tree> ::treesitter_types::FromNode<'tree> for UnaryOperator {
    #[allow(clippy::collapsible_else_if)]
    fn from_node(
        node: ::tree_sitter::Node<'tree>,
        _src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        match node.kind() {
            "!" => Ok(Self::Bang(::treesitter_types::Span::from(node))),
            "+" => Ok(Self::Plus(::treesitter_types::Span::from(node))),
            "-" => Ok(Self::Minus(::treesitter_types::Span::from(node))),
            "defined?" => Ok(Self::DEFINEDQuestion(::treesitter_types::Span::from(node))),
            "not" => Ok(Self::Not(::treesitter_types::Span::from(node))),
            "~" => Ok(Self::Tilde(::treesitter_types::Span::from(node))),
            other => Err(::treesitter_types::ParseError::unexpected_kind(other, node)),
        }
    }
}
impl ::treesitter_types::Spanned for UnaryOperator {
    fn span(&self) -> ::treesitter_types::Span {
        match self {
            Self::Bang(span) => *span,
            Self::Plus(span) => *span,
            Self::Minus(span) => *span,
            Self::DEFINEDQuestion(span) => *span,
            Self::Not(span) => *span,
            Self::Tilde(span) => *span,
        }
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum UnlessAlternative<'tree> {
    Else(::std::boxed::Box<Else<'tree>>),
    Elsif(::std::boxed::Box<Elsif<'tree>>),
}
impl<'tree> ::treesitter_types::FromNode<'tree> for UnlessAlternative<'tree> {
    #[allow(clippy::collapsible_else_if)]
    fn from_node(
        node: ::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        match node.kind() {
            "else" => Ok(Self::Else(::std::boxed::Box::new(
                ::treesitter_types::maybe_grow_stack(|| {
                    <Else as ::treesitter_types::FromNode>::from_node(node, src)
                })?,
            ))),
            "elsif" => Ok(Self::Elsif(::std::boxed::Box::new(
                ::treesitter_types::maybe_grow_stack(|| {
                    <Elsif as ::treesitter_types::FromNode>::from_node(node, src)
                })?,
            ))),
            other => Err(::treesitter_types::ParseError::unexpected_kind(other, node)),
        }
    }
}
impl ::treesitter_types::Spanned for UnlessAlternative<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        match self {
            Self::Else(inner) => inner.span(),
            Self::Elsif(inner) => inner.span(),
        }
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum VariableReferencePatternName<'tree> {
    NonlocalVariable(::std::boxed::Box<NonlocalVariable<'tree>>),
    Identifier(::std::boxed::Box<Identifier<'tree>>),
}
impl<'tree> ::treesitter_types::FromNode<'tree> for VariableReferencePatternName<'tree> {
    #[allow(clippy::collapsible_else_if)]
    fn from_node(
        node: ::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        match node.kind() {
            "identifier" => Ok(Self::Identifier(::std::boxed::Box::new(
                ::treesitter_types::maybe_grow_stack(|| {
                    <Identifier as ::treesitter_types::FromNode>::from_node(node, src)
                })?,
            ))),
            _other => {
                if let Ok(v) = ::treesitter_types::maybe_grow_stack(|| {
                    <NonlocalVariable as ::treesitter_types::FromNode>::from_node(node, src)
                }) {
                    Ok(Self::NonlocalVariable(::std::boxed::Box::new(v)))
                } else {
                    Err(::treesitter_types::ParseError::unexpected_kind(
                        _other, node,
                    ))
                }
            }
        }
    }
}
impl ::treesitter_types::Spanned for VariableReferencePatternName<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        match self {
            Self::NonlocalVariable(inner) => inner.span(),
            Self::Identifier(inner) => inner.span(),
        }
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum AnyNode<'tree> {
    Arg(Arg<'tree>),
    CallOperator(CallOperator),
    Expression(Expression<'tree>),
    Lhs(Lhs<'tree>),
    MethodName(MethodName<'tree>),
    NonlocalVariable(NonlocalVariable<'tree>),
    PatternConstant(PatternConstant<'tree>),
    PatternExpr(PatternExpr<'tree>),
    PatternExprBasic(PatternExprBasic<'tree>),
    PatternPrimitive(PatternPrimitive<'tree>),
    PatternTopExprBody(PatternTopExprBody<'tree>),
    Primary(Primary<'tree>),
    SimpleNumeric(SimpleNumeric<'tree>),
    Statement(Statement<'tree>),
    Variable(Variable<'tree>),
    Alias(Alias<'tree>),
    AlternativePattern(AlternativePattern<'tree>),
    ArgumentList(ArgumentList<'tree>),
    Array(Array<'tree>),
    ArrayPattern(ArrayPattern<'tree>),
    AsPattern(AsPattern<'tree>),
    Assignment(Assignment<'tree>),
    BareString(BareString<'tree>),
    BareSymbol(BareSymbol<'tree>),
    Begin(Begin<'tree>),
    BeginBlock(BeginBlock<'tree>),
    Binary(Binary<'tree>),
    Block(Block<'tree>),
    BlockArgument(BlockArgument<'tree>),
    BlockBody(BlockBody<'tree>),
    BlockParameter(BlockParameter<'tree>),
    BlockParameters(BlockParameters<'tree>),
    BodyStatement(BodyStatement<'tree>),
    Break(Break<'tree>),
    Call(Call<'tree>),
    Case(Case<'tree>),
    CaseMatch(CaseMatch<'tree>),
    ChainedString(ChainedString<'tree>),
    Class(Class<'tree>),
    Complex(Complex<'tree>),
    Conditional(Conditional<'tree>),
    Constant(Constant<'tree>),
    DelimitedSymbol(DelimitedSymbol<'tree>),
    DestructuredLeftAssignment(DestructuredLeftAssignment<'tree>),
    DestructuredParameter(DestructuredParameter<'tree>),
    Do(Do<'tree>),
    DoBlock(DoBlock<'tree>),
    ElementReference(ElementReference<'tree>),
    Else(Else<'tree>),
    Elsif(Elsif<'tree>),
    EmptyStatement(EmptyStatement<'tree>),
    EndBlock(EndBlock<'tree>),
    Ensure(Ensure<'tree>),
    ExceptionVariable(ExceptionVariable<'tree>),
    Exceptions(Exceptions<'tree>),
    ExpressionReferencePattern(ExpressionReferencePattern<'tree>),
    FindPattern(FindPattern<'tree>),
    For(For<'tree>),
    ForwardArgument(ForwardArgument<'tree>),
    ForwardParameter(ForwardParameter<'tree>),
    Hash(Hash<'tree>),
    HashKeySymbol(HashKeySymbol<'tree>),
    HashPattern(HashPattern<'tree>),
    HashSplatArgument(HashSplatArgument<'tree>),
    HashSplatNil(HashSplatNil<'tree>),
    HashSplatParameter(HashSplatParameter<'tree>),
    HeredocBody(HeredocBody<'tree>),
    Identifier(Identifier<'tree>),
    If(If<'tree>),
    IfGuard(IfGuard<'tree>),
    IfModifier(IfModifier<'tree>),
    In(In<'tree>),
    InClause(InClause<'tree>),
    Interpolation(Interpolation<'tree>),
    KeywordParameter(KeywordParameter<'tree>),
    KeywordPattern(KeywordPattern<'tree>),
    Lambda(Lambda<'tree>),
    LambdaParameters(LambdaParameters<'tree>),
    LeftAssignmentList(LeftAssignmentList<'tree>),
    MatchPattern(MatchPattern<'tree>),
    Method(Method<'tree>),
    MethodParameters(MethodParameters<'tree>),
    Module(Module<'tree>),
    Next(Next<'tree>),
    Nil(Nil<'tree>),
    Operator(Operator<'tree>),
    OperatorAssignment(OperatorAssignment<'tree>),
    OptionalParameter(OptionalParameter<'tree>),
    Pair(Pair<'tree>),
    ParenthesizedPattern(ParenthesizedPattern<'tree>),
    ParenthesizedStatements(ParenthesizedStatements<'tree>),
    Pattern(Pattern<'tree>),
    Program(Program<'tree>),
    Range(Range<'tree>),
    Rational(Rational<'tree>),
    Redo(Redo<'tree>),
    Regex(Regex<'tree>),
    Rescue(Rescue<'tree>),
    RescueModifier(RescueModifier<'tree>),
    RestAssignment(RestAssignment<'tree>),
    Retry(Retry<'tree>),
    Return(Return<'tree>),
    RightAssignmentList(RightAssignmentList<'tree>),
    ScopeResolution(ScopeResolution<'tree>),
    Setter(Setter<'tree>),
    SingletonClass(SingletonClass<'tree>),
    SingletonMethod(SingletonMethod<'tree>),
    SplatArgument(SplatArgument<'tree>),
    SplatParameter(SplatParameter<'tree>),
    String(String<'tree>),
    StringArray(StringArray<'tree>),
    Subshell(Subshell<'tree>),
    Superclass(Superclass<'tree>),
    SymbolArray(SymbolArray<'tree>),
    TestPattern(TestPattern<'tree>),
    Then(Then<'tree>),
    Unary(Unary<'tree>),
    Undef(Undef<'tree>),
    Unless(Unless<'tree>),
    UnlessGuard(UnlessGuard<'tree>),
    UnlessModifier(UnlessModifier<'tree>),
    Until(Until<'tree>),
    UntilModifier(UntilModifier<'tree>),
    VariableReferencePattern(VariableReferencePattern<'tree>),
    When(When<'tree>),
    While(While<'tree>),
    WhileModifier(WhileModifier<'tree>),
    Yield(Yield<'tree>),
    Character(Character<'tree>),
    ClassVariable(ClassVariable<'tree>),
    Comment(Comment<'tree>),
    Encoding(Encoding<'tree>),
    EscapeSequence(EscapeSequence<'tree>),
    False(False<'tree>),
    File(File<'tree>),
    Float(Float<'tree>),
    GlobalVariable(GlobalVariable<'tree>),
    HeredocBeginning(HeredocBeginning<'tree>),
    HeredocContent(HeredocContent<'tree>),
    HeredocEnd(HeredocEnd<'tree>),
    InstanceVariable(InstanceVariable<'tree>),
    Integer(Integer<'tree>),
    Line(Line<'tree>),
    SelfType(SelfType<'tree>),
    SimpleSymbol(SimpleSymbol<'tree>),
    StringContent(StringContent<'tree>),
    Super(Super<'tree>),
    True(True<'tree>),
    Uninterpreted(Uninterpreted<'tree>),
    Unknown(::tree_sitter::Node<'tree>),
}
impl<'tree> AnyNode<'tree> {
    pub fn from_node(node: ::tree_sitter::Node<'tree>, src: &'tree [u8]) -> Self {
        match node.kind() {
            "_arg" => ::treesitter_types::maybe_grow_stack(|| {
                <Arg as ::treesitter_types::FromNode>::from_node(node, src)
            })
            .map(Self::Arg)
            .unwrap_or(Self::Unknown(node)),
            "_call_operator" => ::treesitter_types::maybe_grow_stack(|| {
                <CallOperator as ::treesitter_types::FromNode>::from_node(node, src)
            })
            .map(Self::CallOperator)
            .unwrap_or(Self::Unknown(node)),
            "_expression" => ::treesitter_types::maybe_grow_stack(|| {
                <Expression as ::treesitter_types::FromNode>::from_node(node, src)
            })
            .map(Self::Expression)
            .unwrap_or(Self::Unknown(node)),
            "_lhs" => ::treesitter_types::maybe_grow_stack(|| {
                <Lhs as ::treesitter_types::FromNode>::from_node(node, src)
            })
            .map(Self::Lhs)
            .unwrap_or(Self::Unknown(node)),
            "_method_name" => ::treesitter_types::maybe_grow_stack(|| {
                <MethodName as ::treesitter_types::FromNode>::from_node(node, src)
            })
            .map(Self::MethodName)
            .unwrap_or(Self::Unknown(node)),
            "_nonlocal_variable" => ::treesitter_types::maybe_grow_stack(|| {
                <NonlocalVariable as ::treesitter_types::FromNode>::from_node(node, src)
            })
            .map(Self::NonlocalVariable)
            .unwrap_or(Self::Unknown(node)),
            "_pattern_constant" => ::treesitter_types::maybe_grow_stack(|| {
                <PatternConstant as ::treesitter_types::FromNode>::from_node(node, src)
            })
            .map(Self::PatternConstant)
            .unwrap_or(Self::Unknown(node)),
            "_pattern_expr" => ::treesitter_types::maybe_grow_stack(|| {
                <PatternExpr as ::treesitter_types::FromNode>::from_node(node, src)
            })
            .map(Self::PatternExpr)
            .unwrap_or(Self::Unknown(node)),
            "_pattern_expr_basic" => ::treesitter_types::maybe_grow_stack(|| {
                <PatternExprBasic as ::treesitter_types::FromNode>::from_node(node, src)
            })
            .map(Self::PatternExprBasic)
            .unwrap_or(Self::Unknown(node)),
            "_pattern_primitive" => ::treesitter_types::maybe_grow_stack(|| {
                <PatternPrimitive as ::treesitter_types::FromNode>::from_node(node, src)
            })
            .map(Self::PatternPrimitive)
            .unwrap_or(Self::Unknown(node)),
            "_pattern_top_expr_body" => ::treesitter_types::maybe_grow_stack(|| {
                <PatternTopExprBody as ::treesitter_types::FromNode>::from_node(node, src)
            })
            .map(Self::PatternTopExprBody)
            .unwrap_or(Self::Unknown(node)),
            "_primary" => ::treesitter_types::maybe_grow_stack(|| {
                <Primary as ::treesitter_types::FromNode>::from_node(node, src)
            })
            .map(Self::Primary)
            .unwrap_or(Self::Unknown(node)),
            "_simple_numeric" => ::treesitter_types::maybe_grow_stack(|| {
                <SimpleNumeric as ::treesitter_types::FromNode>::from_node(node, src)
            })
            .map(Self::SimpleNumeric)
            .unwrap_or(Self::Unknown(node)),
            "_statement" => ::treesitter_types::maybe_grow_stack(|| {
                <Statement as ::treesitter_types::FromNode>::from_node(node, src)
            })
            .map(Self::Statement)
            .unwrap_or(Self::Unknown(node)),
            "_variable" => ::treesitter_types::maybe_grow_stack(|| {
                <Variable as ::treesitter_types::FromNode>::from_node(node, src)
            })
            .map(Self::Variable)
            .unwrap_or(Self::Unknown(node)),
            "alias" => ::treesitter_types::maybe_grow_stack(|| {
                <Alias as ::treesitter_types::FromNode>::from_node(node, src)
            })
            .map(Self::Alias)
            .unwrap_or(Self::Unknown(node)),
            "alternative_pattern" => ::treesitter_types::maybe_grow_stack(|| {
                <AlternativePattern as ::treesitter_types::FromNode>::from_node(node, src)
            })
            .map(Self::AlternativePattern)
            .unwrap_or(Self::Unknown(node)),
            "argument_list" => ::treesitter_types::maybe_grow_stack(|| {
                <ArgumentList as ::treesitter_types::FromNode>::from_node(node, src)
            })
            .map(Self::ArgumentList)
            .unwrap_or(Self::Unknown(node)),
            "array" => ::treesitter_types::maybe_grow_stack(|| {
                <Array as ::treesitter_types::FromNode>::from_node(node, src)
            })
            .map(Self::Array)
            .unwrap_or(Self::Unknown(node)),
            "array_pattern" => ::treesitter_types::maybe_grow_stack(|| {
                <ArrayPattern as ::treesitter_types::FromNode>::from_node(node, src)
            })
            .map(Self::ArrayPattern)
            .unwrap_or(Self::Unknown(node)),
            "as_pattern" => ::treesitter_types::maybe_grow_stack(|| {
                <AsPattern as ::treesitter_types::FromNode>::from_node(node, src)
            })
            .map(Self::AsPattern)
            .unwrap_or(Self::Unknown(node)),
            "assignment" => ::treesitter_types::maybe_grow_stack(|| {
                <Assignment as ::treesitter_types::FromNode>::from_node(node, src)
            })
            .map(Self::Assignment)
            .unwrap_or(Self::Unknown(node)),
            "bare_string" => ::treesitter_types::maybe_grow_stack(|| {
                <BareString as ::treesitter_types::FromNode>::from_node(node, src)
            })
            .map(Self::BareString)
            .unwrap_or(Self::Unknown(node)),
            "bare_symbol" => ::treesitter_types::maybe_grow_stack(|| {
                <BareSymbol as ::treesitter_types::FromNode>::from_node(node, src)
            })
            .map(Self::BareSymbol)
            .unwrap_or(Self::Unknown(node)),
            "begin" => ::treesitter_types::maybe_grow_stack(|| {
                <Begin as ::treesitter_types::FromNode>::from_node(node, src)
            })
            .map(Self::Begin)
            .unwrap_or(Self::Unknown(node)),
            "begin_block" => ::treesitter_types::maybe_grow_stack(|| {
                <BeginBlock as ::treesitter_types::FromNode>::from_node(node, src)
            })
            .map(Self::BeginBlock)
            .unwrap_or(Self::Unknown(node)),
            "binary" => ::treesitter_types::maybe_grow_stack(|| {
                <Binary as ::treesitter_types::FromNode>::from_node(node, src)
            })
            .map(Self::Binary)
            .unwrap_or(Self::Unknown(node)),
            "block" => ::treesitter_types::maybe_grow_stack(|| {
                <Block as ::treesitter_types::FromNode>::from_node(node, src)
            })
            .map(Self::Block)
            .unwrap_or(Self::Unknown(node)),
            "block_argument" => ::treesitter_types::maybe_grow_stack(|| {
                <BlockArgument as ::treesitter_types::FromNode>::from_node(node, src)
            })
            .map(Self::BlockArgument)
            .unwrap_or(Self::Unknown(node)),
            "block_body" => ::treesitter_types::maybe_grow_stack(|| {
                <BlockBody as ::treesitter_types::FromNode>::from_node(node, src)
            })
            .map(Self::BlockBody)
            .unwrap_or(Self::Unknown(node)),
            "block_parameter" => ::treesitter_types::maybe_grow_stack(|| {
                <BlockParameter as ::treesitter_types::FromNode>::from_node(node, src)
            })
            .map(Self::BlockParameter)
            .unwrap_or(Self::Unknown(node)),
            "block_parameters" => ::treesitter_types::maybe_grow_stack(|| {
                <BlockParameters as ::treesitter_types::FromNode>::from_node(node, src)
            })
            .map(Self::BlockParameters)
            .unwrap_or(Self::Unknown(node)),
            "body_statement" => ::treesitter_types::maybe_grow_stack(|| {
                <BodyStatement as ::treesitter_types::FromNode>::from_node(node, src)
            })
            .map(Self::BodyStatement)
            .unwrap_or(Self::Unknown(node)),
            "break" => ::treesitter_types::maybe_grow_stack(|| {
                <Break as ::treesitter_types::FromNode>::from_node(node, src)
            })
            .map(Self::Break)
            .unwrap_or(Self::Unknown(node)),
            "call" => ::treesitter_types::maybe_grow_stack(|| {
                <Call as ::treesitter_types::FromNode>::from_node(node, src)
            })
            .map(Self::Call)
            .unwrap_or(Self::Unknown(node)),
            "case" => ::treesitter_types::maybe_grow_stack(|| {
                <Case as ::treesitter_types::FromNode>::from_node(node, src)
            })
            .map(Self::Case)
            .unwrap_or(Self::Unknown(node)),
            "case_match" => ::treesitter_types::maybe_grow_stack(|| {
                <CaseMatch as ::treesitter_types::FromNode>::from_node(node, src)
            })
            .map(Self::CaseMatch)
            .unwrap_or(Self::Unknown(node)),
            "chained_string" => ::treesitter_types::maybe_grow_stack(|| {
                <ChainedString as ::treesitter_types::FromNode>::from_node(node, src)
            })
            .map(Self::ChainedString)
            .unwrap_or(Self::Unknown(node)),
            "class" => ::treesitter_types::maybe_grow_stack(|| {
                <Class as ::treesitter_types::FromNode>::from_node(node, src)
            })
            .map(Self::Class)
            .unwrap_or(Self::Unknown(node)),
            "complex" => ::treesitter_types::maybe_grow_stack(|| {
                <Complex as ::treesitter_types::FromNode>::from_node(node, src)
            })
            .map(Self::Complex)
            .unwrap_or(Self::Unknown(node)),
            "conditional" => ::treesitter_types::maybe_grow_stack(|| {
                <Conditional as ::treesitter_types::FromNode>::from_node(node, src)
            })
            .map(Self::Conditional)
            .unwrap_or(Self::Unknown(node)),
            "constant" => ::treesitter_types::maybe_grow_stack(|| {
                <Constant as ::treesitter_types::FromNode>::from_node(node, src)
            })
            .map(Self::Constant)
            .unwrap_or(Self::Unknown(node)),
            "delimited_symbol" => ::treesitter_types::maybe_grow_stack(|| {
                <DelimitedSymbol as ::treesitter_types::FromNode>::from_node(node, src)
            })
            .map(Self::DelimitedSymbol)
            .unwrap_or(Self::Unknown(node)),
            "destructured_left_assignment" => ::treesitter_types::maybe_grow_stack(|| {
                <DestructuredLeftAssignment as ::treesitter_types::FromNode>::from_node(node, src)
            })
            .map(Self::DestructuredLeftAssignment)
            .unwrap_or(Self::Unknown(node)),
            "destructured_parameter" => ::treesitter_types::maybe_grow_stack(|| {
                <DestructuredParameter as ::treesitter_types::FromNode>::from_node(node, src)
            })
            .map(Self::DestructuredParameter)
            .unwrap_or(Self::Unknown(node)),
            "do" => ::treesitter_types::maybe_grow_stack(|| {
                <Do as ::treesitter_types::FromNode>::from_node(node, src)
            })
            .map(Self::Do)
            .unwrap_or(Self::Unknown(node)),
            "do_block" => ::treesitter_types::maybe_grow_stack(|| {
                <DoBlock as ::treesitter_types::FromNode>::from_node(node, src)
            })
            .map(Self::DoBlock)
            .unwrap_or(Self::Unknown(node)),
            "element_reference" => ::treesitter_types::maybe_grow_stack(|| {
                <ElementReference as ::treesitter_types::FromNode>::from_node(node, src)
            })
            .map(Self::ElementReference)
            .unwrap_or(Self::Unknown(node)),
            "else" => ::treesitter_types::maybe_grow_stack(|| {
                <Else as ::treesitter_types::FromNode>::from_node(node, src)
            })
            .map(Self::Else)
            .unwrap_or(Self::Unknown(node)),
            "elsif" => ::treesitter_types::maybe_grow_stack(|| {
                <Elsif as ::treesitter_types::FromNode>::from_node(node, src)
            })
            .map(Self::Elsif)
            .unwrap_or(Self::Unknown(node)),
            "empty_statement" => ::treesitter_types::maybe_grow_stack(|| {
                <EmptyStatement as ::treesitter_types::FromNode>::from_node(node, src)
            })
            .map(Self::EmptyStatement)
            .unwrap_or(Self::Unknown(node)),
            "end_block" => ::treesitter_types::maybe_grow_stack(|| {
                <EndBlock as ::treesitter_types::FromNode>::from_node(node, src)
            })
            .map(Self::EndBlock)
            .unwrap_or(Self::Unknown(node)),
            "ensure" => ::treesitter_types::maybe_grow_stack(|| {
                <Ensure as ::treesitter_types::FromNode>::from_node(node, src)
            })
            .map(Self::Ensure)
            .unwrap_or(Self::Unknown(node)),
            "exception_variable" => ::treesitter_types::maybe_grow_stack(|| {
                <ExceptionVariable as ::treesitter_types::FromNode>::from_node(node, src)
            })
            .map(Self::ExceptionVariable)
            .unwrap_or(Self::Unknown(node)),
            "exceptions" => ::treesitter_types::maybe_grow_stack(|| {
                <Exceptions as ::treesitter_types::FromNode>::from_node(node, src)
            })
            .map(Self::Exceptions)
            .unwrap_or(Self::Unknown(node)),
            "expression_reference_pattern" => ::treesitter_types::maybe_grow_stack(|| {
                <ExpressionReferencePattern as ::treesitter_types::FromNode>::from_node(node, src)
            })
            .map(Self::ExpressionReferencePattern)
            .unwrap_or(Self::Unknown(node)),
            "find_pattern" => ::treesitter_types::maybe_grow_stack(|| {
                <FindPattern as ::treesitter_types::FromNode>::from_node(node, src)
            })
            .map(Self::FindPattern)
            .unwrap_or(Self::Unknown(node)),
            "for" => ::treesitter_types::maybe_grow_stack(|| {
                <For as ::treesitter_types::FromNode>::from_node(node, src)
            })
            .map(Self::For)
            .unwrap_or(Self::Unknown(node)),
            "forward_argument" => ::treesitter_types::maybe_grow_stack(|| {
                <ForwardArgument as ::treesitter_types::FromNode>::from_node(node, src)
            })
            .map(Self::ForwardArgument)
            .unwrap_or(Self::Unknown(node)),
            "forward_parameter" => ::treesitter_types::maybe_grow_stack(|| {
                <ForwardParameter as ::treesitter_types::FromNode>::from_node(node, src)
            })
            .map(Self::ForwardParameter)
            .unwrap_or(Self::Unknown(node)),
            "hash" => ::treesitter_types::maybe_grow_stack(|| {
                <Hash as ::treesitter_types::FromNode>::from_node(node, src)
            })
            .map(Self::Hash)
            .unwrap_or(Self::Unknown(node)),
            "hash_key_symbol" => ::treesitter_types::maybe_grow_stack(|| {
                <HashKeySymbol as ::treesitter_types::FromNode>::from_node(node, src)
            })
            .map(Self::HashKeySymbol)
            .unwrap_or(Self::Unknown(node)),
            "hash_pattern" => ::treesitter_types::maybe_grow_stack(|| {
                <HashPattern as ::treesitter_types::FromNode>::from_node(node, src)
            })
            .map(Self::HashPattern)
            .unwrap_or(Self::Unknown(node)),
            "hash_splat_argument" => ::treesitter_types::maybe_grow_stack(|| {
                <HashSplatArgument as ::treesitter_types::FromNode>::from_node(node, src)
            })
            .map(Self::HashSplatArgument)
            .unwrap_or(Self::Unknown(node)),
            "hash_splat_nil" => ::treesitter_types::maybe_grow_stack(|| {
                <HashSplatNil as ::treesitter_types::FromNode>::from_node(node, src)
            })
            .map(Self::HashSplatNil)
            .unwrap_or(Self::Unknown(node)),
            "hash_splat_parameter" => ::treesitter_types::maybe_grow_stack(|| {
                <HashSplatParameter as ::treesitter_types::FromNode>::from_node(node, src)
            })
            .map(Self::HashSplatParameter)
            .unwrap_or(Self::Unknown(node)),
            "heredoc_body" => ::treesitter_types::maybe_grow_stack(|| {
                <HeredocBody as ::treesitter_types::FromNode>::from_node(node, src)
            })
            .map(Self::HeredocBody)
            .unwrap_or(Self::Unknown(node)),
            "identifier" => ::treesitter_types::maybe_grow_stack(|| {
                <Identifier as ::treesitter_types::FromNode>::from_node(node, src)
            })
            .map(Self::Identifier)
            .unwrap_or(Self::Unknown(node)),
            "if" => ::treesitter_types::maybe_grow_stack(|| {
                <If as ::treesitter_types::FromNode>::from_node(node, src)
            })
            .map(Self::If)
            .unwrap_or(Self::Unknown(node)),
            "if_guard" => ::treesitter_types::maybe_grow_stack(|| {
                <IfGuard as ::treesitter_types::FromNode>::from_node(node, src)
            })
            .map(Self::IfGuard)
            .unwrap_or(Self::Unknown(node)),
            "if_modifier" => ::treesitter_types::maybe_grow_stack(|| {
                <IfModifier as ::treesitter_types::FromNode>::from_node(node, src)
            })
            .map(Self::IfModifier)
            .unwrap_or(Self::Unknown(node)),
            "in" => ::treesitter_types::maybe_grow_stack(|| {
                <In as ::treesitter_types::FromNode>::from_node(node, src)
            })
            .map(Self::In)
            .unwrap_or(Self::Unknown(node)),
            "in_clause" => ::treesitter_types::maybe_grow_stack(|| {
                <InClause as ::treesitter_types::FromNode>::from_node(node, src)
            })
            .map(Self::InClause)
            .unwrap_or(Self::Unknown(node)),
            "interpolation" => ::treesitter_types::maybe_grow_stack(|| {
                <Interpolation as ::treesitter_types::FromNode>::from_node(node, src)
            })
            .map(Self::Interpolation)
            .unwrap_or(Self::Unknown(node)),
            "keyword_parameter" => ::treesitter_types::maybe_grow_stack(|| {
                <KeywordParameter as ::treesitter_types::FromNode>::from_node(node, src)
            })
            .map(Self::KeywordParameter)
            .unwrap_or(Self::Unknown(node)),
            "keyword_pattern" => ::treesitter_types::maybe_grow_stack(|| {
                <KeywordPattern as ::treesitter_types::FromNode>::from_node(node, src)
            })
            .map(Self::KeywordPattern)
            .unwrap_or(Self::Unknown(node)),
            "lambda" => ::treesitter_types::maybe_grow_stack(|| {
                <Lambda as ::treesitter_types::FromNode>::from_node(node, src)
            })
            .map(Self::Lambda)
            .unwrap_or(Self::Unknown(node)),
            "lambda_parameters" => ::treesitter_types::maybe_grow_stack(|| {
                <LambdaParameters as ::treesitter_types::FromNode>::from_node(node, src)
            })
            .map(Self::LambdaParameters)
            .unwrap_or(Self::Unknown(node)),
            "left_assignment_list" => ::treesitter_types::maybe_grow_stack(|| {
                <LeftAssignmentList as ::treesitter_types::FromNode>::from_node(node, src)
            })
            .map(Self::LeftAssignmentList)
            .unwrap_or(Self::Unknown(node)),
            "match_pattern" => ::treesitter_types::maybe_grow_stack(|| {
                <MatchPattern as ::treesitter_types::FromNode>::from_node(node, src)
            })
            .map(Self::MatchPattern)
            .unwrap_or(Self::Unknown(node)),
            "method" => ::treesitter_types::maybe_grow_stack(|| {
                <Method as ::treesitter_types::FromNode>::from_node(node, src)
            })
            .map(Self::Method)
            .unwrap_or(Self::Unknown(node)),
            "method_parameters" => ::treesitter_types::maybe_grow_stack(|| {
                <MethodParameters as ::treesitter_types::FromNode>::from_node(node, src)
            })
            .map(Self::MethodParameters)
            .unwrap_or(Self::Unknown(node)),
            "module" => ::treesitter_types::maybe_grow_stack(|| {
                <Module as ::treesitter_types::FromNode>::from_node(node, src)
            })
            .map(Self::Module)
            .unwrap_or(Self::Unknown(node)),
            "next" => ::treesitter_types::maybe_grow_stack(|| {
                <Next as ::treesitter_types::FromNode>::from_node(node, src)
            })
            .map(Self::Next)
            .unwrap_or(Self::Unknown(node)),
            "nil" => ::treesitter_types::maybe_grow_stack(|| {
                <Nil as ::treesitter_types::FromNode>::from_node(node, src)
            })
            .map(Self::Nil)
            .unwrap_or(Self::Unknown(node)),
            "operator" => ::treesitter_types::maybe_grow_stack(|| {
                <Operator as ::treesitter_types::FromNode>::from_node(node, src)
            })
            .map(Self::Operator)
            .unwrap_or(Self::Unknown(node)),
            "operator_assignment" => ::treesitter_types::maybe_grow_stack(|| {
                <OperatorAssignment as ::treesitter_types::FromNode>::from_node(node, src)
            })
            .map(Self::OperatorAssignment)
            .unwrap_or(Self::Unknown(node)),
            "optional_parameter" => ::treesitter_types::maybe_grow_stack(|| {
                <OptionalParameter as ::treesitter_types::FromNode>::from_node(node, src)
            })
            .map(Self::OptionalParameter)
            .unwrap_or(Self::Unknown(node)),
            "pair" => ::treesitter_types::maybe_grow_stack(|| {
                <Pair as ::treesitter_types::FromNode>::from_node(node, src)
            })
            .map(Self::Pair)
            .unwrap_or(Self::Unknown(node)),
            "parenthesized_pattern" => ::treesitter_types::maybe_grow_stack(|| {
                <ParenthesizedPattern as ::treesitter_types::FromNode>::from_node(node, src)
            })
            .map(Self::ParenthesizedPattern)
            .unwrap_or(Self::Unknown(node)),
            "parenthesized_statements" => ::treesitter_types::maybe_grow_stack(|| {
                <ParenthesizedStatements as ::treesitter_types::FromNode>::from_node(node, src)
            })
            .map(Self::ParenthesizedStatements)
            .unwrap_or(Self::Unknown(node)),
            "pattern" => ::treesitter_types::maybe_grow_stack(|| {
                <Pattern as ::treesitter_types::FromNode>::from_node(node, src)
            })
            .map(Self::Pattern)
            .unwrap_or(Self::Unknown(node)),
            "program" => ::treesitter_types::maybe_grow_stack(|| {
                <Program as ::treesitter_types::FromNode>::from_node(node, src)
            })
            .map(Self::Program)
            .unwrap_or(Self::Unknown(node)),
            "range" => ::treesitter_types::maybe_grow_stack(|| {
                <Range as ::treesitter_types::FromNode>::from_node(node, src)
            })
            .map(Self::Range)
            .unwrap_or(Self::Unknown(node)),
            "rational" => ::treesitter_types::maybe_grow_stack(|| {
                <Rational as ::treesitter_types::FromNode>::from_node(node, src)
            })
            .map(Self::Rational)
            .unwrap_or(Self::Unknown(node)),
            "redo" => ::treesitter_types::maybe_grow_stack(|| {
                <Redo as ::treesitter_types::FromNode>::from_node(node, src)
            })
            .map(Self::Redo)
            .unwrap_or(Self::Unknown(node)),
            "regex" => ::treesitter_types::maybe_grow_stack(|| {
                <Regex as ::treesitter_types::FromNode>::from_node(node, src)
            })
            .map(Self::Regex)
            .unwrap_or(Self::Unknown(node)),
            "rescue" => ::treesitter_types::maybe_grow_stack(|| {
                <Rescue as ::treesitter_types::FromNode>::from_node(node, src)
            })
            .map(Self::Rescue)
            .unwrap_or(Self::Unknown(node)),
            "rescue_modifier" => ::treesitter_types::maybe_grow_stack(|| {
                <RescueModifier as ::treesitter_types::FromNode>::from_node(node, src)
            })
            .map(Self::RescueModifier)
            .unwrap_or(Self::Unknown(node)),
            "rest_assignment" => ::treesitter_types::maybe_grow_stack(|| {
                <RestAssignment as ::treesitter_types::FromNode>::from_node(node, src)
            })
            .map(Self::RestAssignment)
            .unwrap_or(Self::Unknown(node)),
            "retry" => ::treesitter_types::maybe_grow_stack(|| {
                <Retry as ::treesitter_types::FromNode>::from_node(node, src)
            })
            .map(Self::Retry)
            .unwrap_or(Self::Unknown(node)),
            "return" => ::treesitter_types::maybe_grow_stack(|| {
                <Return as ::treesitter_types::FromNode>::from_node(node, src)
            })
            .map(Self::Return)
            .unwrap_or(Self::Unknown(node)),
            "right_assignment_list" => ::treesitter_types::maybe_grow_stack(|| {
                <RightAssignmentList as ::treesitter_types::FromNode>::from_node(node, src)
            })
            .map(Self::RightAssignmentList)
            .unwrap_or(Self::Unknown(node)),
            "scope_resolution" => ::treesitter_types::maybe_grow_stack(|| {
                <ScopeResolution as ::treesitter_types::FromNode>::from_node(node, src)
            })
            .map(Self::ScopeResolution)
            .unwrap_or(Self::Unknown(node)),
            "setter" => ::treesitter_types::maybe_grow_stack(|| {
                <Setter as ::treesitter_types::FromNode>::from_node(node, src)
            })
            .map(Self::Setter)
            .unwrap_or(Self::Unknown(node)),
            "singleton_class" => ::treesitter_types::maybe_grow_stack(|| {
                <SingletonClass as ::treesitter_types::FromNode>::from_node(node, src)
            })
            .map(Self::SingletonClass)
            .unwrap_or(Self::Unknown(node)),
            "singleton_method" => ::treesitter_types::maybe_grow_stack(|| {
                <SingletonMethod as ::treesitter_types::FromNode>::from_node(node, src)
            })
            .map(Self::SingletonMethod)
            .unwrap_or(Self::Unknown(node)),
            "splat_argument" => ::treesitter_types::maybe_grow_stack(|| {
                <SplatArgument as ::treesitter_types::FromNode>::from_node(node, src)
            })
            .map(Self::SplatArgument)
            .unwrap_or(Self::Unknown(node)),
            "splat_parameter" => ::treesitter_types::maybe_grow_stack(|| {
                <SplatParameter as ::treesitter_types::FromNode>::from_node(node, src)
            })
            .map(Self::SplatParameter)
            .unwrap_or(Self::Unknown(node)),
            "string" => ::treesitter_types::maybe_grow_stack(|| {
                <String as ::treesitter_types::FromNode>::from_node(node, src)
            })
            .map(Self::String)
            .unwrap_or(Self::Unknown(node)),
            "string_array" => ::treesitter_types::maybe_grow_stack(|| {
                <StringArray as ::treesitter_types::FromNode>::from_node(node, src)
            })
            .map(Self::StringArray)
            .unwrap_or(Self::Unknown(node)),
            "subshell" => ::treesitter_types::maybe_grow_stack(|| {
                <Subshell as ::treesitter_types::FromNode>::from_node(node, src)
            })
            .map(Self::Subshell)
            .unwrap_or(Self::Unknown(node)),
            "superclass" => ::treesitter_types::maybe_grow_stack(|| {
                <Superclass as ::treesitter_types::FromNode>::from_node(node, src)
            })
            .map(Self::Superclass)
            .unwrap_or(Self::Unknown(node)),
            "symbol_array" => ::treesitter_types::maybe_grow_stack(|| {
                <SymbolArray as ::treesitter_types::FromNode>::from_node(node, src)
            })
            .map(Self::SymbolArray)
            .unwrap_or(Self::Unknown(node)),
            "test_pattern" => ::treesitter_types::maybe_grow_stack(|| {
                <TestPattern as ::treesitter_types::FromNode>::from_node(node, src)
            })
            .map(Self::TestPattern)
            .unwrap_or(Self::Unknown(node)),
            "then" => ::treesitter_types::maybe_grow_stack(|| {
                <Then as ::treesitter_types::FromNode>::from_node(node, src)
            })
            .map(Self::Then)
            .unwrap_or(Self::Unknown(node)),
            "unary" => ::treesitter_types::maybe_grow_stack(|| {
                <Unary as ::treesitter_types::FromNode>::from_node(node, src)
            })
            .map(Self::Unary)
            .unwrap_or(Self::Unknown(node)),
            "undef" => ::treesitter_types::maybe_grow_stack(|| {
                <Undef as ::treesitter_types::FromNode>::from_node(node, src)
            })
            .map(Self::Undef)
            .unwrap_or(Self::Unknown(node)),
            "unless" => ::treesitter_types::maybe_grow_stack(|| {
                <Unless as ::treesitter_types::FromNode>::from_node(node, src)
            })
            .map(Self::Unless)
            .unwrap_or(Self::Unknown(node)),
            "unless_guard" => ::treesitter_types::maybe_grow_stack(|| {
                <UnlessGuard as ::treesitter_types::FromNode>::from_node(node, src)
            })
            .map(Self::UnlessGuard)
            .unwrap_or(Self::Unknown(node)),
            "unless_modifier" => ::treesitter_types::maybe_grow_stack(|| {
                <UnlessModifier as ::treesitter_types::FromNode>::from_node(node, src)
            })
            .map(Self::UnlessModifier)
            .unwrap_or(Self::Unknown(node)),
            "until" => ::treesitter_types::maybe_grow_stack(|| {
                <Until as ::treesitter_types::FromNode>::from_node(node, src)
            })
            .map(Self::Until)
            .unwrap_or(Self::Unknown(node)),
            "until_modifier" => ::treesitter_types::maybe_grow_stack(|| {
                <UntilModifier as ::treesitter_types::FromNode>::from_node(node, src)
            })
            .map(Self::UntilModifier)
            .unwrap_or(Self::Unknown(node)),
            "variable_reference_pattern" => ::treesitter_types::maybe_grow_stack(|| {
                <VariableReferencePattern as ::treesitter_types::FromNode>::from_node(node, src)
            })
            .map(Self::VariableReferencePattern)
            .unwrap_or(Self::Unknown(node)),
            "when" => ::treesitter_types::maybe_grow_stack(|| {
                <When as ::treesitter_types::FromNode>::from_node(node, src)
            })
            .map(Self::When)
            .unwrap_or(Self::Unknown(node)),
            "while" => ::treesitter_types::maybe_grow_stack(|| {
                <While as ::treesitter_types::FromNode>::from_node(node, src)
            })
            .map(Self::While)
            .unwrap_or(Self::Unknown(node)),
            "while_modifier" => ::treesitter_types::maybe_grow_stack(|| {
                <WhileModifier as ::treesitter_types::FromNode>::from_node(node, src)
            })
            .map(Self::WhileModifier)
            .unwrap_or(Self::Unknown(node)),
            "yield" => ::treesitter_types::maybe_grow_stack(|| {
                <Yield as ::treesitter_types::FromNode>::from_node(node, src)
            })
            .map(Self::Yield)
            .unwrap_or(Self::Unknown(node)),
            "character" => ::treesitter_types::maybe_grow_stack(|| {
                <Character as ::treesitter_types::FromNode>::from_node(node, src)
            })
            .map(Self::Character)
            .unwrap_or(Self::Unknown(node)),
            "class_variable" => ::treesitter_types::maybe_grow_stack(|| {
                <ClassVariable as ::treesitter_types::FromNode>::from_node(node, src)
            })
            .map(Self::ClassVariable)
            .unwrap_or(Self::Unknown(node)),
            "comment" => ::treesitter_types::maybe_grow_stack(|| {
                <Comment as ::treesitter_types::FromNode>::from_node(node, src)
            })
            .map(Self::Comment)
            .unwrap_or(Self::Unknown(node)),
            "encoding" => ::treesitter_types::maybe_grow_stack(|| {
                <Encoding as ::treesitter_types::FromNode>::from_node(node, src)
            })
            .map(Self::Encoding)
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
            "file" => ::treesitter_types::maybe_grow_stack(|| {
                <File as ::treesitter_types::FromNode>::from_node(node, src)
            })
            .map(Self::File)
            .unwrap_or(Self::Unknown(node)),
            "float" => ::treesitter_types::maybe_grow_stack(|| {
                <Float as ::treesitter_types::FromNode>::from_node(node, src)
            })
            .map(Self::Float)
            .unwrap_or(Self::Unknown(node)),
            "global_variable" => ::treesitter_types::maybe_grow_stack(|| {
                <GlobalVariable as ::treesitter_types::FromNode>::from_node(node, src)
            })
            .map(Self::GlobalVariable)
            .unwrap_or(Self::Unknown(node)),
            "heredoc_beginning" => ::treesitter_types::maybe_grow_stack(|| {
                <HeredocBeginning as ::treesitter_types::FromNode>::from_node(node, src)
            })
            .map(Self::HeredocBeginning)
            .unwrap_or(Self::Unknown(node)),
            "heredoc_content" => ::treesitter_types::maybe_grow_stack(|| {
                <HeredocContent as ::treesitter_types::FromNode>::from_node(node, src)
            })
            .map(Self::HeredocContent)
            .unwrap_or(Self::Unknown(node)),
            "heredoc_end" => ::treesitter_types::maybe_grow_stack(|| {
                <HeredocEnd as ::treesitter_types::FromNode>::from_node(node, src)
            })
            .map(Self::HeredocEnd)
            .unwrap_or(Self::Unknown(node)),
            "instance_variable" => ::treesitter_types::maybe_grow_stack(|| {
                <InstanceVariable as ::treesitter_types::FromNode>::from_node(node, src)
            })
            .map(Self::InstanceVariable)
            .unwrap_or(Self::Unknown(node)),
            "integer" => ::treesitter_types::maybe_grow_stack(|| {
                <Integer as ::treesitter_types::FromNode>::from_node(node, src)
            })
            .map(Self::Integer)
            .unwrap_or(Self::Unknown(node)),
            "line" => ::treesitter_types::maybe_grow_stack(|| {
                <Line as ::treesitter_types::FromNode>::from_node(node, src)
            })
            .map(Self::Line)
            .unwrap_or(Self::Unknown(node)),
            "self" => ::treesitter_types::maybe_grow_stack(|| {
                <SelfType as ::treesitter_types::FromNode>::from_node(node, src)
            })
            .map(Self::SelfType)
            .unwrap_or(Self::Unknown(node)),
            "simple_symbol" => ::treesitter_types::maybe_grow_stack(|| {
                <SimpleSymbol as ::treesitter_types::FromNode>::from_node(node, src)
            })
            .map(Self::SimpleSymbol)
            .unwrap_or(Self::Unknown(node)),
            "string_content" => ::treesitter_types::maybe_grow_stack(|| {
                <StringContent as ::treesitter_types::FromNode>::from_node(node, src)
            })
            .map(Self::StringContent)
            .unwrap_or(Self::Unknown(node)),
            "super" => ::treesitter_types::maybe_grow_stack(|| {
                <Super as ::treesitter_types::FromNode>::from_node(node, src)
            })
            .map(Self::Super)
            .unwrap_or(Self::Unknown(node)),
            "true" => ::treesitter_types::maybe_grow_stack(|| {
                <True as ::treesitter_types::FromNode>::from_node(node, src)
            })
            .map(Self::True)
            .unwrap_or(Self::Unknown(node)),
            "uninterpreted" => ::treesitter_types::maybe_grow_stack(|| {
                <Uninterpreted as ::treesitter_types::FromNode>::from_node(node, src)
            })
            .map(Self::Uninterpreted)
            .unwrap_or(Self::Unknown(node)),
            _ => Self::Unknown(node),
        }
    }
}
impl ::treesitter_types::Spanned for AnyNode<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        match self {
            Self::Arg(inner) => inner.span(),
            Self::CallOperator(inner) => inner.span(),
            Self::Expression(inner) => inner.span(),
            Self::Lhs(inner) => inner.span(),
            Self::MethodName(inner) => inner.span(),
            Self::NonlocalVariable(inner) => inner.span(),
            Self::PatternConstant(inner) => inner.span(),
            Self::PatternExpr(inner) => inner.span(),
            Self::PatternExprBasic(inner) => inner.span(),
            Self::PatternPrimitive(inner) => inner.span(),
            Self::PatternTopExprBody(inner) => inner.span(),
            Self::Primary(inner) => inner.span(),
            Self::SimpleNumeric(inner) => inner.span(),
            Self::Statement(inner) => inner.span(),
            Self::Variable(inner) => inner.span(),
            Self::Alias(inner) => inner.span(),
            Self::AlternativePattern(inner) => inner.span(),
            Self::ArgumentList(inner) => inner.span(),
            Self::Array(inner) => inner.span(),
            Self::ArrayPattern(inner) => inner.span(),
            Self::AsPattern(inner) => inner.span(),
            Self::Assignment(inner) => inner.span(),
            Self::BareString(inner) => inner.span(),
            Self::BareSymbol(inner) => inner.span(),
            Self::Begin(inner) => inner.span(),
            Self::BeginBlock(inner) => inner.span(),
            Self::Binary(inner) => inner.span(),
            Self::Block(inner) => inner.span(),
            Self::BlockArgument(inner) => inner.span(),
            Self::BlockBody(inner) => inner.span(),
            Self::BlockParameter(inner) => inner.span(),
            Self::BlockParameters(inner) => inner.span(),
            Self::BodyStatement(inner) => inner.span(),
            Self::Break(inner) => inner.span(),
            Self::Call(inner) => inner.span(),
            Self::Case(inner) => inner.span(),
            Self::CaseMatch(inner) => inner.span(),
            Self::ChainedString(inner) => inner.span(),
            Self::Class(inner) => inner.span(),
            Self::Complex(inner) => inner.span(),
            Self::Conditional(inner) => inner.span(),
            Self::Constant(inner) => inner.span(),
            Self::DelimitedSymbol(inner) => inner.span(),
            Self::DestructuredLeftAssignment(inner) => inner.span(),
            Self::DestructuredParameter(inner) => inner.span(),
            Self::Do(inner) => inner.span(),
            Self::DoBlock(inner) => inner.span(),
            Self::ElementReference(inner) => inner.span(),
            Self::Else(inner) => inner.span(),
            Self::Elsif(inner) => inner.span(),
            Self::EmptyStatement(inner) => inner.span(),
            Self::EndBlock(inner) => inner.span(),
            Self::Ensure(inner) => inner.span(),
            Self::ExceptionVariable(inner) => inner.span(),
            Self::Exceptions(inner) => inner.span(),
            Self::ExpressionReferencePattern(inner) => inner.span(),
            Self::FindPattern(inner) => inner.span(),
            Self::For(inner) => inner.span(),
            Self::ForwardArgument(inner) => inner.span(),
            Self::ForwardParameter(inner) => inner.span(),
            Self::Hash(inner) => inner.span(),
            Self::HashKeySymbol(inner) => inner.span(),
            Self::HashPattern(inner) => inner.span(),
            Self::HashSplatArgument(inner) => inner.span(),
            Self::HashSplatNil(inner) => inner.span(),
            Self::HashSplatParameter(inner) => inner.span(),
            Self::HeredocBody(inner) => inner.span(),
            Self::Identifier(inner) => inner.span(),
            Self::If(inner) => inner.span(),
            Self::IfGuard(inner) => inner.span(),
            Self::IfModifier(inner) => inner.span(),
            Self::In(inner) => inner.span(),
            Self::InClause(inner) => inner.span(),
            Self::Interpolation(inner) => inner.span(),
            Self::KeywordParameter(inner) => inner.span(),
            Self::KeywordPattern(inner) => inner.span(),
            Self::Lambda(inner) => inner.span(),
            Self::LambdaParameters(inner) => inner.span(),
            Self::LeftAssignmentList(inner) => inner.span(),
            Self::MatchPattern(inner) => inner.span(),
            Self::Method(inner) => inner.span(),
            Self::MethodParameters(inner) => inner.span(),
            Self::Module(inner) => inner.span(),
            Self::Next(inner) => inner.span(),
            Self::Nil(inner) => inner.span(),
            Self::Operator(inner) => inner.span(),
            Self::OperatorAssignment(inner) => inner.span(),
            Self::OptionalParameter(inner) => inner.span(),
            Self::Pair(inner) => inner.span(),
            Self::ParenthesizedPattern(inner) => inner.span(),
            Self::ParenthesizedStatements(inner) => inner.span(),
            Self::Pattern(inner) => inner.span(),
            Self::Program(inner) => inner.span(),
            Self::Range(inner) => inner.span(),
            Self::Rational(inner) => inner.span(),
            Self::Redo(inner) => inner.span(),
            Self::Regex(inner) => inner.span(),
            Self::Rescue(inner) => inner.span(),
            Self::RescueModifier(inner) => inner.span(),
            Self::RestAssignment(inner) => inner.span(),
            Self::Retry(inner) => inner.span(),
            Self::Return(inner) => inner.span(),
            Self::RightAssignmentList(inner) => inner.span(),
            Self::ScopeResolution(inner) => inner.span(),
            Self::Setter(inner) => inner.span(),
            Self::SingletonClass(inner) => inner.span(),
            Self::SingletonMethod(inner) => inner.span(),
            Self::SplatArgument(inner) => inner.span(),
            Self::SplatParameter(inner) => inner.span(),
            Self::String(inner) => inner.span(),
            Self::StringArray(inner) => inner.span(),
            Self::Subshell(inner) => inner.span(),
            Self::Superclass(inner) => inner.span(),
            Self::SymbolArray(inner) => inner.span(),
            Self::TestPattern(inner) => inner.span(),
            Self::Then(inner) => inner.span(),
            Self::Unary(inner) => inner.span(),
            Self::Undef(inner) => inner.span(),
            Self::Unless(inner) => inner.span(),
            Self::UnlessGuard(inner) => inner.span(),
            Self::UnlessModifier(inner) => inner.span(),
            Self::Until(inner) => inner.span(),
            Self::UntilModifier(inner) => inner.span(),
            Self::VariableReferencePattern(inner) => inner.span(),
            Self::When(inner) => inner.span(),
            Self::While(inner) => inner.span(),
            Self::WhileModifier(inner) => inner.span(),
            Self::Yield(inner) => inner.span(),
            Self::Character(inner) => inner.span(),
            Self::ClassVariable(inner) => inner.span(),
            Self::Comment(inner) => inner.span(),
            Self::Encoding(inner) => inner.span(),
            Self::EscapeSequence(inner) => inner.span(),
            Self::False(inner) => inner.span(),
            Self::File(inner) => inner.span(),
            Self::Float(inner) => inner.span(),
            Self::GlobalVariable(inner) => inner.span(),
            Self::HeredocBeginning(inner) => inner.span(),
            Self::HeredocContent(inner) => inner.span(),
            Self::HeredocEnd(inner) => inner.span(),
            Self::InstanceVariable(inner) => inner.span(),
            Self::Integer(inner) => inner.span(),
            Self::Line(inner) => inner.span(),
            Self::SelfType(inner) => inner.span(),
            Self::SimpleSymbol(inner) => inner.span(),
            Self::StringContent(inner) => inner.span(),
            Self::Super(inner) => inner.span(),
            Self::True(inner) => inner.span(),
            Self::Uninterpreted(inner) => inner.span(),
            Self::Unknown(node) => ::treesitter_types::Span::from(*node),
        }
    }
}
