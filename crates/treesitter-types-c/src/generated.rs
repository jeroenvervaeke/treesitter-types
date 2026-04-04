#[derive(Debug, Clone, PartialEq, Eq)]
pub enum AbstractDeclarator<'tree> {
    AbstractArrayDeclarator(::std::boxed::Box<AbstractArrayDeclarator<'tree>>),
    AbstractFunctionDeclarator(::std::boxed::Box<AbstractFunctionDeclarator<'tree>>),
    AbstractParenthesizedDeclarator(::std::boxed::Box<AbstractParenthesizedDeclarator<'tree>>),
    AbstractPointerDeclarator(::std::boxed::Box<AbstractPointerDeclarator<'tree>>),
}
impl<'tree> ::treesitter_types::FromNode<'tree> for AbstractDeclarator<'tree> {
    #[allow(clippy::collapsible_else_if)]
    fn from_node(
        node: ::treesitter_types::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        match node.kind() {
            "abstract_array_declarator" => Ok(Self::AbstractArrayDeclarator(
                ::std::boxed::Box::new(::treesitter_types::runtime::maybe_grow_stack(|| {
                    <AbstractArrayDeclarator as ::treesitter_types::FromNode>::from_node(node, src)
                })?),
            )),
            "abstract_function_declarator" => Ok(Self::AbstractFunctionDeclarator(
                ::std::boxed::Box::new(::treesitter_types::runtime::maybe_grow_stack(|| {
                    <AbstractFunctionDeclarator as ::treesitter_types::FromNode>::from_node(
                        node, src,
                    )
                })?),
            )),
            "abstract_parenthesized_declarator" => Ok(Self::AbstractParenthesizedDeclarator(
                ::std::boxed::Box::new(::treesitter_types::runtime::maybe_grow_stack(|| {
                    <AbstractParenthesizedDeclarator as ::treesitter_types::FromNode>::from_node(
                        node, src,
                    )
                })?),
            )),
            "abstract_pointer_declarator" => Ok(Self::AbstractPointerDeclarator(
                ::std::boxed::Box::new(::treesitter_types::runtime::maybe_grow_stack(|| {
                    <AbstractPointerDeclarator as ::treesitter_types::FromNode>::from_node(
                        node, src,
                    )
                })?),
            )),
            other => Err(::treesitter_types::ParseError::unexpected_kind(other, node)),
        }
    }
}
impl ::treesitter_types::Spanned for AbstractDeclarator<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        match self {
            Self::AbstractArrayDeclarator(inner) => inner.span(),
            Self::AbstractFunctionDeclarator(inner) => inner.span(),
            Self::AbstractParenthesizedDeclarator(inner) => inner.span(),
            Self::AbstractPointerDeclarator(inner) => inner.span(),
        }
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Declarator<'tree> {
    ArrayDeclarator(::std::boxed::Box<ArrayDeclarator<'tree>>),
    AttributedDeclarator(::std::boxed::Box<AttributedDeclarator<'tree>>),
    FunctionDeclarator(::std::boxed::Box<FunctionDeclarator<'tree>>),
    Identifier(::std::boxed::Box<Identifier<'tree>>),
    ParenthesizedDeclarator(::std::boxed::Box<ParenthesizedDeclarator<'tree>>),
    PointerDeclarator(::std::boxed::Box<PointerDeclarator<'tree>>),
}
impl<'tree> ::treesitter_types::FromNode<'tree> for Declarator<'tree> {
    #[allow(clippy::collapsible_else_if)]
    fn from_node(
        node: ::treesitter_types::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        match node.kind() {
            "array_declarator" => Ok(Self::ArrayDeclarator(::std::boxed::Box::new(
                ::treesitter_types::runtime::maybe_grow_stack(|| {
                    <ArrayDeclarator as ::treesitter_types::FromNode>::from_node(node, src)
                })?,
            ))),
            "attributed_declarator" => Ok(Self::AttributedDeclarator(::std::boxed::Box::new(
                ::treesitter_types::runtime::maybe_grow_stack(|| {
                    <AttributedDeclarator as ::treesitter_types::FromNode>::from_node(node, src)
                })?,
            ))),
            "function_declarator" => Ok(Self::FunctionDeclarator(::std::boxed::Box::new(
                ::treesitter_types::runtime::maybe_grow_stack(|| {
                    <FunctionDeclarator as ::treesitter_types::FromNode>::from_node(node, src)
                })?,
            ))),
            "identifier" => Ok(Self::Identifier(::std::boxed::Box::new(
                ::treesitter_types::runtime::maybe_grow_stack(|| {
                    <Identifier as ::treesitter_types::FromNode>::from_node(node, src)
                })?,
            ))),
            "parenthesized_declarator" => Ok(Self::ParenthesizedDeclarator(
                ::std::boxed::Box::new(::treesitter_types::runtime::maybe_grow_stack(|| {
                    <ParenthesizedDeclarator as ::treesitter_types::FromNode>::from_node(node, src)
                })?),
            )),
            "pointer_declarator" => Ok(Self::PointerDeclarator(::std::boxed::Box::new(
                ::treesitter_types::runtime::maybe_grow_stack(|| {
                    <PointerDeclarator as ::treesitter_types::FromNode>::from_node(node, src)
                })?,
            ))),
            other => Err(::treesitter_types::ParseError::unexpected_kind(other, node)),
        }
    }
}
impl ::treesitter_types::Spanned for Declarator<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        match self {
            Self::ArrayDeclarator(inner) => inner.span(),
            Self::AttributedDeclarator(inner) => inner.span(),
            Self::FunctionDeclarator(inner) => inner.span(),
            Self::Identifier(inner) => inner.span(),
            Self::ParenthesizedDeclarator(inner) => inner.span(),
            Self::PointerDeclarator(inner) => inner.span(),
        }
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum FieldDeclarator<'tree> {
    ArrayDeclarator(::std::boxed::Box<ArrayDeclarator<'tree>>),
    AttributedDeclarator(::std::boxed::Box<AttributedDeclarator<'tree>>),
    FieldIdentifier(::std::boxed::Box<FieldIdentifier<'tree>>),
    FunctionDeclarator(::std::boxed::Box<FunctionDeclarator<'tree>>),
    ParenthesizedDeclarator(::std::boxed::Box<ParenthesizedDeclarator<'tree>>),
    PointerDeclarator(::std::boxed::Box<PointerDeclarator<'tree>>),
}
impl<'tree> ::treesitter_types::FromNode<'tree> for FieldDeclarator<'tree> {
    #[allow(clippy::collapsible_else_if)]
    fn from_node(
        node: ::treesitter_types::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        match node.kind() {
            "array_declarator" => Ok(Self::ArrayDeclarator(::std::boxed::Box::new(
                ::treesitter_types::runtime::maybe_grow_stack(|| {
                    <ArrayDeclarator as ::treesitter_types::FromNode>::from_node(node, src)
                })?,
            ))),
            "attributed_declarator" => Ok(Self::AttributedDeclarator(::std::boxed::Box::new(
                ::treesitter_types::runtime::maybe_grow_stack(|| {
                    <AttributedDeclarator as ::treesitter_types::FromNode>::from_node(node, src)
                })?,
            ))),
            "field_identifier" => Ok(Self::FieldIdentifier(::std::boxed::Box::new(
                ::treesitter_types::runtime::maybe_grow_stack(|| {
                    <FieldIdentifier as ::treesitter_types::FromNode>::from_node(node, src)
                })?,
            ))),
            "function_declarator" => Ok(Self::FunctionDeclarator(::std::boxed::Box::new(
                ::treesitter_types::runtime::maybe_grow_stack(|| {
                    <FunctionDeclarator as ::treesitter_types::FromNode>::from_node(node, src)
                })?,
            ))),
            "parenthesized_declarator" => Ok(Self::ParenthesizedDeclarator(
                ::std::boxed::Box::new(::treesitter_types::runtime::maybe_grow_stack(|| {
                    <ParenthesizedDeclarator as ::treesitter_types::FromNode>::from_node(node, src)
                })?),
            )),
            "pointer_declarator" => Ok(Self::PointerDeclarator(::std::boxed::Box::new(
                ::treesitter_types::runtime::maybe_grow_stack(|| {
                    <PointerDeclarator as ::treesitter_types::FromNode>::from_node(node, src)
                })?,
            ))),
            other => Err(::treesitter_types::ParseError::unexpected_kind(other, node)),
        }
    }
}
impl ::treesitter_types::Spanned for FieldDeclarator<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        match self {
            Self::ArrayDeclarator(inner) => inner.span(),
            Self::AttributedDeclarator(inner) => inner.span(),
            Self::FieldIdentifier(inner) => inner.span(),
            Self::FunctionDeclarator(inner) => inner.span(),
            Self::ParenthesizedDeclarator(inner) => inner.span(),
            Self::PointerDeclarator(inner) => inner.span(),
        }
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum TypeDeclarator<'tree> {
    ArrayDeclarator(::std::boxed::Box<ArrayDeclarator<'tree>>),
    AttributedDeclarator(::std::boxed::Box<AttributedDeclarator<'tree>>),
    FunctionDeclarator(::std::boxed::Box<FunctionDeclarator<'tree>>),
    ParenthesizedDeclarator(::std::boxed::Box<ParenthesizedDeclarator<'tree>>),
    PointerDeclarator(::std::boxed::Box<PointerDeclarator<'tree>>),
    PrimitiveType(::std::boxed::Box<PrimitiveType<'tree>>),
    TypeIdentifier(::std::boxed::Box<TypeIdentifier<'tree>>),
}
impl<'tree> ::treesitter_types::FromNode<'tree> for TypeDeclarator<'tree> {
    #[allow(clippy::collapsible_else_if)]
    fn from_node(
        node: ::treesitter_types::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        match node.kind() {
            "array_declarator" => Ok(Self::ArrayDeclarator(::std::boxed::Box::new(
                ::treesitter_types::runtime::maybe_grow_stack(|| {
                    <ArrayDeclarator as ::treesitter_types::FromNode>::from_node(node, src)
                })?,
            ))),
            "attributed_declarator" => Ok(Self::AttributedDeclarator(::std::boxed::Box::new(
                ::treesitter_types::runtime::maybe_grow_stack(|| {
                    <AttributedDeclarator as ::treesitter_types::FromNode>::from_node(node, src)
                })?,
            ))),
            "function_declarator" => Ok(Self::FunctionDeclarator(::std::boxed::Box::new(
                ::treesitter_types::runtime::maybe_grow_stack(|| {
                    <FunctionDeclarator as ::treesitter_types::FromNode>::from_node(node, src)
                })?,
            ))),
            "parenthesized_declarator" => Ok(Self::ParenthesizedDeclarator(
                ::std::boxed::Box::new(::treesitter_types::runtime::maybe_grow_stack(|| {
                    <ParenthesizedDeclarator as ::treesitter_types::FromNode>::from_node(node, src)
                })?),
            )),
            "pointer_declarator" => Ok(Self::PointerDeclarator(::std::boxed::Box::new(
                ::treesitter_types::runtime::maybe_grow_stack(|| {
                    <PointerDeclarator as ::treesitter_types::FromNode>::from_node(node, src)
                })?,
            ))),
            "primitive_type" => Ok(Self::PrimitiveType(::std::boxed::Box::new(
                ::treesitter_types::runtime::maybe_grow_stack(|| {
                    <PrimitiveType as ::treesitter_types::FromNode>::from_node(node, src)
                })?,
            ))),
            "type_identifier" => Ok(Self::TypeIdentifier(::std::boxed::Box::new(
                ::treesitter_types::runtime::maybe_grow_stack(|| {
                    <TypeIdentifier as ::treesitter_types::FromNode>::from_node(node, src)
                })?,
            ))),
            other => Err(::treesitter_types::ParseError::unexpected_kind(other, node)),
        }
    }
}
impl ::treesitter_types::Spanned for TypeDeclarator<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        match self {
            Self::ArrayDeclarator(inner) => inner.span(),
            Self::AttributedDeclarator(inner) => inner.span(),
            Self::FunctionDeclarator(inner) => inner.span(),
            Self::ParenthesizedDeclarator(inner) => inner.span(),
            Self::PointerDeclarator(inner) => inner.span(),
            Self::PrimitiveType(inner) => inner.span(),
            Self::TypeIdentifier(inner) => inner.span(),
        }
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Expression<'tree> {
    AlignofExpression(::std::boxed::Box<AlignofExpression<'tree>>),
    AssignmentExpression(::std::boxed::Box<AssignmentExpression<'tree>>),
    BinaryExpression(::std::boxed::Box<BinaryExpression<'tree>>),
    CallExpression(::std::boxed::Box<CallExpression<'tree>>),
    CastExpression(::std::boxed::Box<CastExpression<'tree>>),
    CharLiteral(::std::boxed::Box<CharLiteral<'tree>>),
    CompoundLiteralExpression(::std::boxed::Box<CompoundLiteralExpression<'tree>>),
    ConcatenatedString(::std::boxed::Box<ConcatenatedString<'tree>>),
    ConditionalExpression(::std::boxed::Box<ConditionalExpression<'tree>>),
    ExtensionExpression(::std::boxed::Box<ExtensionExpression<'tree>>),
    False(::std::boxed::Box<False<'tree>>),
    FieldExpression(::std::boxed::Box<FieldExpression<'tree>>),
    GenericExpression(::std::boxed::Box<GenericExpression<'tree>>),
    GnuAsmExpression(::std::boxed::Box<GnuAsmExpression<'tree>>),
    Identifier(::std::boxed::Box<Identifier<'tree>>),
    Null(::std::boxed::Box<Null<'tree>>),
    NumberLiteral(::std::boxed::Box<NumberLiteral<'tree>>),
    OffsetofExpression(::std::boxed::Box<OffsetofExpression<'tree>>),
    ParenthesizedExpression(::std::boxed::Box<ParenthesizedExpression<'tree>>),
    PointerExpression(::std::boxed::Box<PointerExpression<'tree>>),
    SizeofExpression(::std::boxed::Box<SizeofExpression<'tree>>),
    StringLiteral(::std::boxed::Box<StringLiteral<'tree>>),
    SubscriptExpression(::std::boxed::Box<SubscriptExpression<'tree>>),
    True(::std::boxed::Box<True<'tree>>),
    UnaryExpression(::std::boxed::Box<UnaryExpression<'tree>>),
    UpdateExpression(::std::boxed::Box<UpdateExpression<'tree>>),
}
impl<'tree> ::treesitter_types::FromNode<'tree> for Expression<'tree> {
    #[allow(clippy::collapsible_else_if)]
    fn from_node(
        node: ::treesitter_types::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        match node.kind() {
            "alignof_expression" => Ok(Self::AlignofExpression(::std::boxed::Box::new(
                ::treesitter_types::runtime::maybe_grow_stack(|| {
                    <AlignofExpression as ::treesitter_types::FromNode>::from_node(node, src)
                })?,
            ))),
            "assignment_expression" => Ok(Self::AssignmentExpression(::std::boxed::Box::new(
                ::treesitter_types::runtime::maybe_grow_stack(|| {
                    <AssignmentExpression as ::treesitter_types::FromNode>::from_node(node, src)
                })?,
            ))),
            "binary_expression" => Ok(Self::BinaryExpression(::std::boxed::Box::new(
                ::treesitter_types::runtime::maybe_grow_stack(|| {
                    <BinaryExpression as ::treesitter_types::FromNode>::from_node(node, src)
                })?,
            ))),
            "call_expression" => Ok(Self::CallExpression(::std::boxed::Box::new(
                ::treesitter_types::runtime::maybe_grow_stack(|| {
                    <CallExpression as ::treesitter_types::FromNode>::from_node(node, src)
                })?,
            ))),
            "cast_expression" => Ok(Self::CastExpression(::std::boxed::Box::new(
                ::treesitter_types::runtime::maybe_grow_stack(|| {
                    <CastExpression as ::treesitter_types::FromNode>::from_node(node, src)
                })?,
            ))),
            "char_literal" => Ok(Self::CharLiteral(::std::boxed::Box::new(
                ::treesitter_types::runtime::maybe_grow_stack(|| {
                    <CharLiteral as ::treesitter_types::FromNode>::from_node(node, src)
                })?,
            ))),
            "compound_literal_expression" => Ok(Self::CompoundLiteralExpression(
                ::std::boxed::Box::new(::treesitter_types::runtime::maybe_grow_stack(|| {
                    <CompoundLiteralExpression as ::treesitter_types::FromNode>::from_node(
                        node, src,
                    )
                })?),
            )),
            "concatenated_string" => Ok(Self::ConcatenatedString(::std::boxed::Box::new(
                ::treesitter_types::runtime::maybe_grow_stack(|| {
                    <ConcatenatedString as ::treesitter_types::FromNode>::from_node(node, src)
                })?,
            ))),
            "conditional_expression" => Ok(Self::ConditionalExpression(::std::boxed::Box::new(
                ::treesitter_types::runtime::maybe_grow_stack(|| {
                    <ConditionalExpression as ::treesitter_types::FromNode>::from_node(node, src)
                })?,
            ))),
            "extension_expression" => Ok(Self::ExtensionExpression(::std::boxed::Box::new(
                ::treesitter_types::runtime::maybe_grow_stack(|| {
                    <ExtensionExpression as ::treesitter_types::FromNode>::from_node(node, src)
                })?,
            ))),
            "false" => Ok(Self::False(::std::boxed::Box::new(
                ::treesitter_types::runtime::maybe_grow_stack(|| {
                    <False as ::treesitter_types::FromNode>::from_node(node, src)
                })?,
            ))),
            "field_expression" => Ok(Self::FieldExpression(::std::boxed::Box::new(
                ::treesitter_types::runtime::maybe_grow_stack(|| {
                    <FieldExpression as ::treesitter_types::FromNode>::from_node(node, src)
                })?,
            ))),
            "generic_expression" => Ok(Self::GenericExpression(::std::boxed::Box::new(
                ::treesitter_types::runtime::maybe_grow_stack(|| {
                    <GenericExpression as ::treesitter_types::FromNode>::from_node(node, src)
                })?,
            ))),
            "gnu_asm_expression" => Ok(Self::GnuAsmExpression(::std::boxed::Box::new(
                ::treesitter_types::runtime::maybe_grow_stack(|| {
                    <GnuAsmExpression as ::treesitter_types::FromNode>::from_node(node, src)
                })?,
            ))),
            "identifier" => Ok(Self::Identifier(::std::boxed::Box::new(
                ::treesitter_types::runtime::maybe_grow_stack(|| {
                    <Identifier as ::treesitter_types::FromNode>::from_node(node, src)
                })?,
            ))),
            "null" => Ok(Self::Null(::std::boxed::Box::new(
                ::treesitter_types::runtime::maybe_grow_stack(|| {
                    <Null as ::treesitter_types::FromNode>::from_node(node, src)
                })?,
            ))),
            "number_literal" => Ok(Self::NumberLiteral(::std::boxed::Box::new(
                ::treesitter_types::runtime::maybe_grow_stack(|| {
                    <NumberLiteral as ::treesitter_types::FromNode>::from_node(node, src)
                })?,
            ))),
            "offsetof_expression" => Ok(Self::OffsetofExpression(::std::boxed::Box::new(
                ::treesitter_types::runtime::maybe_grow_stack(|| {
                    <OffsetofExpression as ::treesitter_types::FromNode>::from_node(node, src)
                })?,
            ))),
            "parenthesized_expression" => Ok(Self::ParenthesizedExpression(
                ::std::boxed::Box::new(::treesitter_types::runtime::maybe_grow_stack(|| {
                    <ParenthesizedExpression as ::treesitter_types::FromNode>::from_node(node, src)
                })?),
            )),
            "pointer_expression" => Ok(Self::PointerExpression(::std::boxed::Box::new(
                ::treesitter_types::runtime::maybe_grow_stack(|| {
                    <PointerExpression as ::treesitter_types::FromNode>::from_node(node, src)
                })?,
            ))),
            "sizeof_expression" => Ok(Self::SizeofExpression(::std::boxed::Box::new(
                ::treesitter_types::runtime::maybe_grow_stack(|| {
                    <SizeofExpression as ::treesitter_types::FromNode>::from_node(node, src)
                })?,
            ))),
            "string_literal" => Ok(Self::StringLiteral(::std::boxed::Box::new(
                ::treesitter_types::runtime::maybe_grow_stack(|| {
                    <StringLiteral as ::treesitter_types::FromNode>::from_node(node, src)
                })?,
            ))),
            "subscript_expression" => Ok(Self::SubscriptExpression(::std::boxed::Box::new(
                ::treesitter_types::runtime::maybe_grow_stack(|| {
                    <SubscriptExpression as ::treesitter_types::FromNode>::from_node(node, src)
                })?,
            ))),
            "true" => Ok(Self::True(::std::boxed::Box::new(
                ::treesitter_types::runtime::maybe_grow_stack(|| {
                    <True as ::treesitter_types::FromNode>::from_node(node, src)
                })?,
            ))),
            "unary_expression" => Ok(Self::UnaryExpression(::std::boxed::Box::new(
                ::treesitter_types::runtime::maybe_grow_stack(|| {
                    <UnaryExpression as ::treesitter_types::FromNode>::from_node(node, src)
                })?,
            ))),
            "update_expression" => Ok(Self::UpdateExpression(::std::boxed::Box::new(
                ::treesitter_types::runtime::maybe_grow_stack(|| {
                    <UpdateExpression as ::treesitter_types::FromNode>::from_node(node, src)
                })?,
            ))),
            other => Err(::treesitter_types::ParseError::unexpected_kind(other, node)),
        }
    }
}
impl ::treesitter_types::Spanned for Expression<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        match self {
            Self::AlignofExpression(inner) => inner.span(),
            Self::AssignmentExpression(inner) => inner.span(),
            Self::BinaryExpression(inner) => inner.span(),
            Self::CallExpression(inner) => inner.span(),
            Self::CastExpression(inner) => inner.span(),
            Self::CharLiteral(inner) => inner.span(),
            Self::CompoundLiteralExpression(inner) => inner.span(),
            Self::ConcatenatedString(inner) => inner.span(),
            Self::ConditionalExpression(inner) => inner.span(),
            Self::ExtensionExpression(inner) => inner.span(),
            Self::False(inner) => inner.span(),
            Self::FieldExpression(inner) => inner.span(),
            Self::GenericExpression(inner) => inner.span(),
            Self::GnuAsmExpression(inner) => inner.span(),
            Self::Identifier(inner) => inner.span(),
            Self::Null(inner) => inner.span(),
            Self::NumberLiteral(inner) => inner.span(),
            Self::OffsetofExpression(inner) => inner.span(),
            Self::ParenthesizedExpression(inner) => inner.span(),
            Self::PointerExpression(inner) => inner.span(),
            Self::SizeofExpression(inner) => inner.span(),
            Self::StringLiteral(inner) => inner.span(),
            Self::SubscriptExpression(inner) => inner.span(),
            Self::True(inner) => inner.span(),
            Self::UnaryExpression(inner) => inner.span(),
            Self::UpdateExpression(inner) => inner.span(),
        }
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Statement<'tree> {
    AttributedStatement(::std::boxed::Box<AttributedStatement<'tree>>),
    BreakStatement(::std::boxed::Box<BreakStatement<'tree>>),
    CaseStatement(::std::boxed::Box<CaseStatement<'tree>>),
    CompoundStatement(::std::boxed::Box<CompoundStatement<'tree>>),
    ContinueStatement(::std::boxed::Box<ContinueStatement<'tree>>),
    DoStatement(::std::boxed::Box<DoStatement<'tree>>),
    ExpressionStatement(::std::boxed::Box<ExpressionStatement<'tree>>),
    ForStatement(::std::boxed::Box<ForStatement<'tree>>),
    GotoStatement(::std::boxed::Box<GotoStatement<'tree>>),
    IfStatement(::std::boxed::Box<IfStatement<'tree>>),
    LabeledStatement(::std::boxed::Box<LabeledStatement<'tree>>),
    ReturnStatement(::std::boxed::Box<ReturnStatement<'tree>>),
    SehLeaveStatement(::std::boxed::Box<SehLeaveStatement<'tree>>),
    SehTryStatement(::std::boxed::Box<SehTryStatement<'tree>>),
    SwitchStatement(::std::boxed::Box<SwitchStatement<'tree>>),
    WhileStatement(::std::boxed::Box<WhileStatement<'tree>>),
}
impl<'tree> ::treesitter_types::FromNode<'tree> for Statement<'tree> {
    #[allow(clippy::collapsible_else_if)]
    fn from_node(
        node: ::treesitter_types::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        match node.kind() {
            "attributed_statement" => Ok(Self::AttributedStatement(::std::boxed::Box::new(
                ::treesitter_types::runtime::maybe_grow_stack(|| {
                    <AttributedStatement as ::treesitter_types::FromNode>::from_node(node, src)
                })?,
            ))),
            "break_statement" => Ok(Self::BreakStatement(::std::boxed::Box::new(
                ::treesitter_types::runtime::maybe_grow_stack(|| {
                    <BreakStatement as ::treesitter_types::FromNode>::from_node(node, src)
                })?,
            ))),
            "case_statement" => Ok(Self::CaseStatement(::std::boxed::Box::new(
                ::treesitter_types::runtime::maybe_grow_stack(|| {
                    <CaseStatement as ::treesitter_types::FromNode>::from_node(node, src)
                })?,
            ))),
            "compound_statement" => Ok(Self::CompoundStatement(::std::boxed::Box::new(
                ::treesitter_types::runtime::maybe_grow_stack(|| {
                    <CompoundStatement as ::treesitter_types::FromNode>::from_node(node, src)
                })?,
            ))),
            "continue_statement" => Ok(Self::ContinueStatement(::std::boxed::Box::new(
                ::treesitter_types::runtime::maybe_grow_stack(|| {
                    <ContinueStatement as ::treesitter_types::FromNode>::from_node(node, src)
                })?,
            ))),
            "do_statement" => Ok(Self::DoStatement(::std::boxed::Box::new(
                ::treesitter_types::runtime::maybe_grow_stack(|| {
                    <DoStatement as ::treesitter_types::FromNode>::from_node(node, src)
                })?,
            ))),
            "expression_statement" => Ok(Self::ExpressionStatement(::std::boxed::Box::new(
                ::treesitter_types::runtime::maybe_grow_stack(|| {
                    <ExpressionStatement as ::treesitter_types::FromNode>::from_node(node, src)
                })?,
            ))),
            "for_statement" => Ok(Self::ForStatement(::std::boxed::Box::new(
                ::treesitter_types::runtime::maybe_grow_stack(|| {
                    <ForStatement as ::treesitter_types::FromNode>::from_node(node, src)
                })?,
            ))),
            "goto_statement" => Ok(Self::GotoStatement(::std::boxed::Box::new(
                ::treesitter_types::runtime::maybe_grow_stack(|| {
                    <GotoStatement as ::treesitter_types::FromNode>::from_node(node, src)
                })?,
            ))),
            "if_statement" => Ok(Self::IfStatement(::std::boxed::Box::new(
                ::treesitter_types::runtime::maybe_grow_stack(|| {
                    <IfStatement as ::treesitter_types::FromNode>::from_node(node, src)
                })?,
            ))),
            "labeled_statement" => Ok(Self::LabeledStatement(::std::boxed::Box::new(
                ::treesitter_types::runtime::maybe_grow_stack(|| {
                    <LabeledStatement as ::treesitter_types::FromNode>::from_node(node, src)
                })?,
            ))),
            "return_statement" => Ok(Self::ReturnStatement(::std::boxed::Box::new(
                ::treesitter_types::runtime::maybe_grow_stack(|| {
                    <ReturnStatement as ::treesitter_types::FromNode>::from_node(node, src)
                })?,
            ))),
            "seh_leave_statement" => Ok(Self::SehLeaveStatement(::std::boxed::Box::new(
                ::treesitter_types::runtime::maybe_grow_stack(|| {
                    <SehLeaveStatement as ::treesitter_types::FromNode>::from_node(node, src)
                })?,
            ))),
            "seh_try_statement" => Ok(Self::SehTryStatement(::std::boxed::Box::new(
                ::treesitter_types::runtime::maybe_grow_stack(|| {
                    <SehTryStatement as ::treesitter_types::FromNode>::from_node(node, src)
                })?,
            ))),
            "switch_statement" => Ok(Self::SwitchStatement(::std::boxed::Box::new(
                ::treesitter_types::runtime::maybe_grow_stack(|| {
                    <SwitchStatement as ::treesitter_types::FromNode>::from_node(node, src)
                })?,
            ))),
            "while_statement" => Ok(Self::WhileStatement(::std::boxed::Box::new(
                ::treesitter_types::runtime::maybe_grow_stack(|| {
                    <WhileStatement as ::treesitter_types::FromNode>::from_node(node, src)
                })?,
            ))),
            other => Err(::treesitter_types::ParseError::unexpected_kind(other, node)),
        }
    }
}
impl ::treesitter_types::Spanned for Statement<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        match self {
            Self::AttributedStatement(inner) => inner.span(),
            Self::BreakStatement(inner) => inner.span(),
            Self::CaseStatement(inner) => inner.span(),
            Self::CompoundStatement(inner) => inner.span(),
            Self::ContinueStatement(inner) => inner.span(),
            Self::DoStatement(inner) => inner.span(),
            Self::ExpressionStatement(inner) => inner.span(),
            Self::ForStatement(inner) => inner.span(),
            Self::GotoStatement(inner) => inner.span(),
            Self::IfStatement(inner) => inner.span(),
            Self::LabeledStatement(inner) => inner.span(),
            Self::ReturnStatement(inner) => inner.span(),
            Self::SehLeaveStatement(inner) => inner.span(),
            Self::SehTryStatement(inner) => inner.span(),
            Self::SwitchStatement(inner) => inner.span(),
            Self::WhileStatement(inner) => inner.span(),
        }
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum TypeSpecifier<'tree> {
    EnumSpecifier(::std::boxed::Box<EnumSpecifier<'tree>>),
    MacroTypeSpecifier(::std::boxed::Box<MacroTypeSpecifier<'tree>>),
    PrimitiveType(::std::boxed::Box<PrimitiveType<'tree>>),
    SizedTypeSpecifier(::std::boxed::Box<SizedTypeSpecifier<'tree>>),
    StructSpecifier(::std::boxed::Box<StructSpecifier<'tree>>),
    TypeIdentifier(::std::boxed::Box<TypeIdentifier<'tree>>),
    UnionSpecifier(::std::boxed::Box<UnionSpecifier<'tree>>),
}
impl<'tree> ::treesitter_types::FromNode<'tree> for TypeSpecifier<'tree> {
    #[allow(clippy::collapsible_else_if)]
    fn from_node(
        node: ::treesitter_types::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        match node.kind() {
            "enum_specifier" => Ok(Self::EnumSpecifier(::std::boxed::Box::new(
                ::treesitter_types::runtime::maybe_grow_stack(|| {
                    <EnumSpecifier as ::treesitter_types::FromNode>::from_node(node, src)
                })?,
            ))),
            "macro_type_specifier" => Ok(Self::MacroTypeSpecifier(::std::boxed::Box::new(
                ::treesitter_types::runtime::maybe_grow_stack(|| {
                    <MacroTypeSpecifier as ::treesitter_types::FromNode>::from_node(node, src)
                })?,
            ))),
            "primitive_type" => Ok(Self::PrimitiveType(::std::boxed::Box::new(
                ::treesitter_types::runtime::maybe_grow_stack(|| {
                    <PrimitiveType as ::treesitter_types::FromNode>::from_node(node, src)
                })?,
            ))),
            "sized_type_specifier" => Ok(Self::SizedTypeSpecifier(::std::boxed::Box::new(
                ::treesitter_types::runtime::maybe_grow_stack(|| {
                    <SizedTypeSpecifier as ::treesitter_types::FromNode>::from_node(node, src)
                })?,
            ))),
            "struct_specifier" => Ok(Self::StructSpecifier(::std::boxed::Box::new(
                ::treesitter_types::runtime::maybe_grow_stack(|| {
                    <StructSpecifier as ::treesitter_types::FromNode>::from_node(node, src)
                })?,
            ))),
            "type_identifier" => Ok(Self::TypeIdentifier(::std::boxed::Box::new(
                ::treesitter_types::runtime::maybe_grow_stack(|| {
                    <TypeIdentifier as ::treesitter_types::FromNode>::from_node(node, src)
                })?,
            ))),
            "union_specifier" => Ok(Self::UnionSpecifier(::std::boxed::Box::new(
                ::treesitter_types::runtime::maybe_grow_stack(|| {
                    <UnionSpecifier as ::treesitter_types::FromNode>::from_node(node, src)
                })?,
            ))),
            other => Err(::treesitter_types::ParseError::unexpected_kind(other, node)),
        }
    }
}
impl ::treesitter_types::Spanned for TypeSpecifier<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        match self {
            Self::EnumSpecifier(inner) => inner.span(),
            Self::MacroTypeSpecifier(inner) => inner.span(),
            Self::PrimitiveType(inner) => inner.span(),
            Self::SizedTypeSpecifier(inner) => inner.span(),
            Self::StructSpecifier(inner) => inner.span(),
            Self::TypeIdentifier(inner) => inner.span(),
            Self::UnionSpecifier(inner) => inner.span(),
        }
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct AbstractArrayDeclarator<'tree> {
    pub span: ::treesitter_types::Span,
    pub declarator: ::core::option::Option<AbstractDeclarator<'tree>>,
    pub size: ::core::option::Option<AbstractArrayDeclaratorSize<'tree>>,
    pub children: ::std::vec::Vec<TypeQualifier<'tree>>,
}
impl<'tree> ::treesitter_types::FromNode<'tree> for AbstractArrayDeclarator<'tree> {
    #[allow(clippy::match_single_binding, clippy::suspicious_else_formatting)]
    fn from_node(
        node: ::treesitter_types::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        debug_assert_eq!(node.kind(), "abstract_array_declarator");
        Ok(Self {
            span: ::treesitter_types::Span::from(node),
            declarator: match node.child_by_field_name("declarator") {
                Some(child) => Some(::treesitter_types::runtime::maybe_grow_stack(|| {
                    <AbstractDeclarator as ::treesitter_types::FromNode>::from_node(child, src)
                })?),
                None => None,
            },
            size: match node.child_by_field_name("size") {
                Some(child) => Some(::treesitter_types::runtime::maybe_grow_stack(|| {
                    <AbstractArrayDeclaratorSize as ::treesitter_types::FromNode>::from_node(
                        child, src,
                    )
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
                    items.push(::treesitter_types::runtime::maybe_grow_stack(|| {
                        <TypeQualifier as ::treesitter_types::FromNode>::from_node(child, src)
                    })?);
                }
                items
            },
        })
    }
}
impl ::treesitter_types::Spanned for AbstractArrayDeclarator<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        self.span
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct AbstractFunctionDeclarator<'tree> {
    pub span: ::treesitter_types::Span,
    pub declarator: ::core::option::Option<AbstractDeclarator<'tree>>,
    pub parameters: ParameterList<'tree>,
}
impl<'tree> ::treesitter_types::FromNode<'tree> for AbstractFunctionDeclarator<'tree> {
    #[allow(clippy::match_single_binding, clippy::suspicious_else_formatting)]
    fn from_node(
        node: ::treesitter_types::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        debug_assert_eq!(node.kind(), "abstract_function_declarator");
        Ok(Self {
            span: ::treesitter_types::Span::from(node),
            declarator: match node.child_by_field_name("declarator") {
                Some(child) => Some(::treesitter_types::runtime::maybe_grow_stack(|| {
                    <AbstractDeclarator as ::treesitter_types::FromNode>::from_node(child, src)
                })?),
                None => None,
            },
            parameters: {
                let child = node.child_by_field_name("parameters").ok_or_else(|| {
                    ::treesitter_types::ParseError::missing_field("parameters", node)
                })?;
                ::treesitter_types::runtime::maybe_grow_stack(|| {
                    <ParameterList as ::treesitter_types::FromNode>::from_node(child, src)
                })?
            },
        })
    }
}
impl ::treesitter_types::Spanned for AbstractFunctionDeclarator<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        self.span
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct AbstractParenthesizedDeclarator<'tree> {
    pub span: ::treesitter_types::Span,
    pub children: ::std::vec::Vec<AbstractParenthesizedDeclaratorChildren<'tree>>,
}
impl<'tree> ::treesitter_types::FromNode<'tree> for AbstractParenthesizedDeclarator<'tree> {
    #[allow(clippy::match_single_binding, clippy::suspicious_else_formatting)]
    fn from_node(
        node: ::treesitter_types::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        debug_assert_eq!(node.kind(), "abstract_parenthesized_declarator");
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
                            ::treesitter_types::runtime::maybe_grow_stack(|| <AbstractParenthesizedDeclaratorChildren as ::treesitter_types::FromNode>::from_node(
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
impl ::treesitter_types::Spanned for AbstractParenthesizedDeclarator<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        self.span
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct AbstractPointerDeclarator<'tree> {
    pub span: ::treesitter_types::Span,
    pub declarator: ::core::option::Option<AbstractDeclarator<'tree>>,
    pub children: ::std::vec::Vec<AbstractPointerDeclaratorChildren<'tree>>,
}
impl<'tree> ::treesitter_types::FromNode<'tree> for AbstractPointerDeclarator<'tree> {
    #[allow(clippy::match_single_binding, clippy::suspicious_else_formatting)]
    fn from_node(
        node: ::treesitter_types::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        debug_assert_eq!(node.kind(), "abstract_pointer_declarator");
        Ok(Self {
            span: ::treesitter_types::Span::from(node),
            declarator: match node.child_by_field_name("declarator") {
                Some(child) => Some(::treesitter_types::runtime::maybe_grow_stack(|| {
                    <AbstractDeclarator as ::treesitter_types::FromNode>::from_node(child, src)
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
                    items
                        .push(
                            ::treesitter_types::runtime::maybe_grow_stack(|| <AbstractPointerDeclaratorChildren as ::treesitter_types::FromNode>::from_node(
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
impl ::treesitter_types::Spanned for AbstractPointerDeclarator<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        self.span
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct AlignasQualifier<'tree> {
    pub span: ::treesitter_types::Span,
    pub children: AlignasQualifierChildren<'tree>,
}
impl<'tree> ::treesitter_types::FromNode<'tree> for AlignasQualifier<'tree> {
    #[allow(clippy::match_single_binding, clippy::suspicious_else_formatting)]
    fn from_node(
        node: ::treesitter_types::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        debug_assert_eq!(node.kind(), "alignas_qualifier");
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
                                        ::treesitter_types::runtime::maybe_grow_stack(|| <AlignasQualifierChildren as ::treesitter_types::FromNode>::from_node(
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
                                            ::treesitter_types::runtime::maybe_grow_stack(|| <AlignasQualifierChildren as ::treesitter_types::FromNode>::from_node(
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
                    <AlignasQualifierChildren as ::treesitter_types::FromNode>::from_node(
                        child, src,
                    )
                })?
            },
        })
    }
}
impl ::treesitter_types::Spanned for AlignasQualifier<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        self.span
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct AlignofExpression<'tree> {
    pub span: ::treesitter_types::Span,
    pub r#type: TypeDescriptor<'tree>,
}
impl<'tree> ::treesitter_types::FromNode<'tree> for AlignofExpression<'tree> {
    #[allow(clippy::match_single_binding, clippy::suspicious_else_formatting)]
    fn from_node(
        node: ::treesitter_types::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        debug_assert_eq!(node.kind(), "alignof_expression");
        Ok(Self {
            span: ::treesitter_types::Span::from(node),
            r#type: {
                let child = node
                    .child_by_field_name("type")
                    .ok_or_else(|| ::treesitter_types::ParseError::missing_field("type", node))?;
                ::treesitter_types::runtime::maybe_grow_stack(|| {
                    <TypeDescriptor as ::treesitter_types::FromNode>::from_node(child, src)
                })?
            },
        })
    }
}
impl ::treesitter_types::Spanned for AlignofExpression<'_> {
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
        node: ::treesitter_types::tree_sitter::Node<'tree>,
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
                    items.push(::treesitter_types::runtime::maybe_grow_stack(|| {
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
pub struct ArrayDeclarator<'tree> {
    pub span: ::treesitter_types::Span,
    pub declarator: ArrayDeclaratorDeclarator<'tree>,
    pub size: ::core::option::Option<ArrayDeclaratorSize<'tree>>,
    pub children: ::std::vec::Vec<TypeQualifier<'tree>>,
}
impl<'tree> ::treesitter_types::FromNode<'tree> for ArrayDeclarator<'tree> {
    #[allow(clippy::match_single_binding, clippy::suspicious_else_formatting)]
    fn from_node(
        node: ::treesitter_types::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        debug_assert_eq!(node.kind(), "array_declarator");
        Ok(Self {
            span: ::treesitter_types::Span::from(node),
            declarator: {
                let child = node.child_by_field_name("declarator").ok_or_else(|| {
                    ::treesitter_types::ParseError::missing_field("declarator", node)
                })?;
                ::treesitter_types::runtime::maybe_grow_stack(|| {
                    <ArrayDeclaratorDeclarator as ::treesitter_types::FromNode>::from_node(
                        child, src,
                    )
                })?
            },
            size: match node.child_by_field_name("size") {
                Some(child) => Some(::treesitter_types::runtime::maybe_grow_stack(|| {
                    <ArrayDeclaratorSize as ::treesitter_types::FromNode>::from_node(child, src)
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
                    items.push(::treesitter_types::runtime::maybe_grow_stack(|| {
                        <TypeQualifier as ::treesitter_types::FromNode>::from_node(child, src)
                    })?);
                }
                items
            },
        })
    }
}
impl ::treesitter_types::Spanned for ArrayDeclarator<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        self.span
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct AssignmentExpression<'tree> {
    pub span: ::treesitter_types::Span,
    pub left: AssignmentExpressionLeft<'tree>,
    pub operator: AssignmentExpressionOperator,
    pub right: Expression<'tree>,
}
impl<'tree> ::treesitter_types::FromNode<'tree> for AssignmentExpression<'tree> {
    #[allow(clippy::match_single_binding, clippy::suspicious_else_formatting)]
    fn from_node(
        node: ::treesitter_types::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        debug_assert_eq!(node.kind(), "assignment_expression");
        Ok(Self {
            span: ::treesitter_types::Span::from(node),
            left: {
                let child = node
                    .child_by_field_name("left")
                    .ok_or_else(|| ::treesitter_types::ParseError::missing_field("left", node))?;
                ::treesitter_types::runtime::maybe_grow_stack(|| {
                    <AssignmentExpressionLeft as ::treesitter_types::FromNode>::from_node(
                        child, src,
                    )
                })?
            },
            operator: {
                let child = node.child_by_field_name("operator").ok_or_else(|| {
                    ::treesitter_types::ParseError::missing_field("operator", node)
                })?;
                ::treesitter_types::runtime::maybe_grow_stack(|| {
                    <AssignmentExpressionOperator as ::treesitter_types::FromNode>::from_node(
                        child, src,
                    )
                })?
            },
            right: {
                let child = node
                    .child_by_field_name("right")
                    .ok_or_else(|| ::treesitter_types::ParseError::missing_field("right", node))?;
                ::treesitter_types::runtime::maybe_grow_stack(|| {
                    <Expression as ::treesitter_types::FromNode>::from_node(child, src)
                })?
            },
        })
    }
}
impl ::treesitter_types::Spanned for AssignmentExpression<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        self.span
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Attribute<'tree> {
    pub span: ::treesitter_types::Span,
    pub name: Identifier<'tree>,
    pub prefix: ::core::option::Option<Identifier<'tree>>,
    pub children: ::core::option::Option<ArgumentList<'tree>>,
}
impl<'tree> ::treesitter_types::FromNode<'tree> for Attribute<'tree> {
    #[allow(clippy::match_single_binding, clippy::suspicious_else_formatting)]
    fn from_node(
        node: ::treesitter_types::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        debug_assert_eq!(node.kind(), "attribute");
        Ok(Self {
            span: ::treesitter_types::Span::from(node),
            name: {
                let child = node
                    .child_by_field_name("name")
                    .ok_or_else(|| ::treesitter_types::ParseError::missing_field("name", node))?;
                ::treesitter_types::runtime::maybe_grow_stack(|| {
                    <Identifier as ::treesitter_types::FromNode>::from_node(child, src)
                })?
            },
            prefix: match node.child_by_field_name("prefix") {
                Some(child) => Some(::treesitter_types::runtime::maybe_grow_stack(|| {
                    <Identifier as ::treesitter_types::FromNode>::from_node(child, src)
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
                match non_field_children.first() {
                    Some(&child) => Some(::treesitter_types::runtime::maybe_grow_stack(|| {
                        <ArgumentList as ::treesitter_types::FromNode>::from_node(child, src)
                    })?),
                    None => None,
                }
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
pub struct AttributeDeclaration<'tree> {
    pub span: ::treesitter_types::Span,
    pub children: ::std::vec::Vec<Attribute<'tree>>,
}
impl<'tree> ::treesitter_types::FromNode<'tree> for AttributeDeclaration<'tree> {
    #[allow(clippy::match_single_binding, clippy::suspicious_else_formatting)]
    fn from_node(
        node: ::treesitter_types::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        debug_assert_eq!(node.kind(), "attribute_declaration");
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
                        <Attribute as ::treesitter_types::FromNode>::from_node(child, src)
                    })?);
                }
                items
            },
        })
    }
}
impl ::treesitter_types::Spanned for AttributeDeclaration<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        self.span
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct AttributeSpecifier<'tree> {
    pub span: ::treesitter_types::Span,
    pub children: ArgumentList<'tree>,
}
impl<'tree> ::treesitter_types::FromNode<'tree> for AttributeSpecifier<'tree> {
    #[allow(clippy::match_single_binding, clippy::suspicious_else_formatting)]
    fn from_node(
        node: ::treesitter_types::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        debug_assert_eq!(node.kind(), "attribute_specifier");
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
                                        ::treesitter_types::runtime::maybe_grow_stack(|| <ArgumentList as ::treesitter_types::FromNode>::from_node(
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
                                            ::treesitter_types::runtime::maybe_grow_stack(|| <ArgumentList as ::treesitter_types::FromNode>::from_node(
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
                    <ArgumentList as ::treesitter_types::FromNode>::from_node(child, src)
                })?
            },
        })
    }
}
impl ::treesitter_types::Spanned for AttributeSpecifier<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        self.span
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct AttributedDeclarator<'tree> {
    pub span: ::treesitter_types::Span,
    pub children: ::std::vec::Vec<AttributedDeclaratorChildren<'tree>>,
}
impl<'tree> ::treesitter_types::FromNode<'tree> for AttributedDeclarator<'tree> {
    #[allow(clippy::match_single_binding, clippy::suspicious_else_formatting)]
    fn from_node(
        node: ::treesitter_types::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        debug_assert_eq!(node.kind(), "attributed_declarator");
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
                        <AttributedDeclaratorChildren as ::treesitter_types::FromNode>::from_node(
                            child, src,
                        )
                    })?);
                }
                items
            },
        })
    }
}
impl ::treesitter_types::Spanned for AttributedDeclarator<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        self.span
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct AttributedStatement<'tree> {
    pub span: ::treesitter_types::Span,
    pub children: ::std::vec::Vec<AttributedStatementChildren<'tree>>,
}
impl<'tree> ::treesitter_types::FromNode<'tree> for AttributedStatement<'tree> {
    #[allow(clippy::match_single_binding, clippy::suspicious_else_formatting)]
    fn from_node(
        node: ::treesitter_types::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        debug_assert_eq!(node.kind(), "attributed_statement");
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
                        <AttributedStatementChildren as ::treesitter_types::FromNode>::from_node(
                            child, src,
                        )
                    })?);
                }
                items
            },
        })
    }
}
impl ::treesitter_types::Spanned for AttributedStatement<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        self.span
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct BinaryExpression<'tree> {
    pub span: ::treesitter_types::Span,
    pub left: BinaryExpressionLeft<'tree>,
    pub operator: BinaryExpressionOperator,
    pub right: BinaryExpressionRight<'tree>,
}
impl<'tree> ::treesitter_types::FromNode<'tree> for BinaryExpression<'tree> {
    #[allow(clippy::match_single_binding, clippy::suspicious_else_formatting)]
    fn from_node(
        node: ::treesitter_types::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        debug_assert_eq!(node.kind(), "binary_expression");
        Ok(Self {
            span: ::treesitter_types::Span::from(node),
            left: {
                let child = node
                    .child_by_field_name("left")
                    .ok_or_else(|| ::treesitter_types::ParseError::missing_field("left", node))?;
                ::treesitter_types::runtime::maybe_grow_stack(|| {
                    <BinaryExpressionLeft as ::treesitter_types::FromNode>::from_node(child, src)
                })?
            },
            operator: {
                let child = node.child_by_field_name("operator").ok_or_else(|| {
                    ::treesitter_types::ParseError::missing_field("operator", node)
                })?;
                ::treesitter_types::runtime::maybe_grow_stack(|| {
                    <BinaryExpressionOperator as ::treesitter_types::FromNode>::from_node(
                        child, src,
                    )
                })?
            },
            right: {
                let child = node
                    .child_by_field_name("right")
                    .ok_or_else(|| ::treesitter_types::ParseError::missing_field("right", node))?;
                ::treesitter_types::runtime::maybe_grow_stack(|| {
                    <BinaryExpressionRight as ::treesitter_types::FromNode>::from_node(child, src)
                })?
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
pub struct BitfieldClause<'tree> {
    pub span: ::treesitter_types::Span,
    pub children: Expression<'tree>,
}
impl<'tree> ::treesitter_types::FromNode<'tree> for BitfieldClause<'tree> {
    #[allow(clippy::match_single_binding, clippy::suspicious_else_formatting)]
    fn from_node(
        node: ::treesitter_types::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        debug_assert_eq!(node.kind(), "bitfield_clause");
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
                                        ::treesitter_types::runtime::maybe_grow_stack(|| <Expression as ::treesitter_types::FromNode>::from_node(
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
                                            ::treesitter_types::runtime::maybe_grow_stack(|| <Expression as ::treesitter_types::FromNode>::from_node(
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
                    <Expression as ::treesitter_types::FromNode>::from_node(child, src)
                })?
            },
        })
    }
}
impl ::treesitter_types::Spanned for BitfieldClause<'_> {
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
        node: ::treesitter_types::tree_sitter::Node<'tree>,
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
pub struct CallExpression<'tree> {
    pub span: ::treesitter_types::Span,
    pub arguments: ArgumentList<'tree>,
    pub function: Expression<'tree>,
}
impl<'tree> ::treesitter_types::FromNode<'tree> for CallExpression<'tree> {
    #[allow(clippy::match_single_binding, clippy::suspicious_else_formatting)]
    fn from_node(
        node: ::treesitter_types::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        debug_assert_eq!(node.kind(), "call_expression");
        Ok(Self {
            span: ::treesitter_types::Span::from(node),
            arguments: {
                let child = node.child_by_field_name("arguments").ok_or_else(|| {
                    ::treesitter_types::ParseError::missing_field("arguments", node)
                })?;
                ::treesitter_types::runtime::maybe_grow_stack(|| {
                    <ArgumentList as ::treesitter_types::FromNode>::from_node(child, src)
                })?
            },
            function: {
                let child = node.child_by_field_name("function").ok_or_else(|| {
                    ::treesitter_types::ParseError::missing_field("function", node)
                })?;
                ::treesitter_types::runtime::maybe_grow_stack(|| {
                    <Expression as ::treesitter_types::FromNode>::from_node(child, src)
                })?
            },
        })
    }
}
impl ::treesitter_types::Spanned for CallExpression<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        self.span
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CaseStatement<'tree> {
    pub span: ::treesitter_types::Span,
    pub value: ::core::option::Option<Expression<'tree>>,
    pub children: ::std::vec::Vec<CaseStatementChildren<'tree>>,
}
impl<'tree> ::treesitter_types::FromNode<'tree> for CaseStatement<'tree> {
    #[allow(clippy::match_single_binding, clippy::suspicious_else_formatting)]
    fn from_node(
        node: ::treesitter_types::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        debug_assert_eq!(node.kind(), "case_statement");
        Ok(Self {
            span: ::treesitter_types::Span::from(node),
            value: match node.child_by_field_name("value") {
                Some(child) => Some(::treesitter_types::runtime::maybe_grow_stack(|| {
                    <Expression as ::treesitter_types::FromNode>::from_node(child, src)
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
                    items.push(::treesitter_types::runtime::maybe_grow_stack(|| {
                        <CaseStatementChildren as ::treesitter_types::FromNode>::from_node(
                            child, src,
                        )
                    })?);
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
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CastExpression<'tree> {
    pub span: ::treesitter_types::Span,
    pub r#type: TypeDescriptor<'tree>,
    pub value: Expression<'tree>,
}
impl<'tree> ::treesitter_types::FromNode<'tree> for CastExpression<'tree> {
    #[allow(clippy::match_single_binding, clippy::suspicious_else_formatting)]
    fn from_node(
        node: ::treesitter_types::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        debug_assert_eq!(node.kind(), "cast_expression");
        Ok(Self {
            span: ::treesitter_types::Span::from(node),
            r#type: {
                let child = node
                    .child_by_field_name("type")
                    .ok_or_else(|| ::treesitter_types::ParseError::missing_field("type", node))?;
                ::treesitter_types::runtime::maybe_grow_stack(|| {
                    <TypeDescriptor as ::treesitter_types::FromNode>::from_node(child, src)
                })?
            },
            value: {
                let child = node
                    .child_by_field_name("value")
                    .ok_or_else(|| ::treesitter_types::ParseError::missing_field("value", node))?;
                ::treesitter_types::runtime::maybe_grow_stack(|| {
                    <Expression as ::treesitter_types::FromNode>::from_node(child, src)
                })?
            },
        })
    }
}
impl ::treesitter_types::Spanned for CastExpression<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        self.span
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CharLiteral<'tree> {
    pub span: ::treesitter_types::Span,
    pub children: ::std::vec::Vec<CharLiteralChildren<'tree>>,
}
impl<'tree> ::treesitter_types::FromNode<'tree> for CharLiteral<'tree> {
    #[allow(clippy::match_single_binding, clippy::suspicious_else_formatting)]
    fn from_node(
        node: ::treesitter_types::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        debug_assert_eq!(node.kind(), "char_literal");
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
                        <CharLiteralChildren as ::treesitter_types::FromNode>::from_node(child, src)
                    })?);
                }
                items
            },
        })
    }
}
impl ::treesitter_types::Spanned for CharLiteral<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        self.span
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CommaExpression<'tree> {
    pub span: ::treesitter_types::Span,
    pub left: Expression<'tree>,
    pub right: CommaExpressionRight<'tree>,
}
impl<'tree> ::treesitter_types::FromNode<'tree> for CommaExpression<'tree> {
    #[allow(clippy::match_single_binding, clippy::suspicious_else_formatting)]
    fn from_node(
        node: ::treesitter_types::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        debug_assert_eq!(node.kind(), "comma_expression");
        Ok(Self {
            span: ::treesitter_types::Span::from(node),
            left: {
                let child = node
                    .child_by_field_name("left")
                    .ok_or_else(|| ::treesitter_types::ParseError::missing_field("left", node))?;
                ::treesitter_types::runtime::maybe_grow_stack(|| {
                    <Expression as ::treesitter_types::FromNode>::from_node(child, src)
                })?
            },
            right: {
                let child = node
                    .child_by_field_name("right")
                    .ok_or_else(|| ::treesitter_types::ParseError::missing_field("right", node))?;
                ::treesitter_types::runtime::maybe_grow_stack(|| {
                    <CommaExpressionRight as ::treesitter_types::FromNode>::from_node(child, src)
                })?
            },
        })
    }
}
impl ::treesitter_types::Spanned for CommaExpression<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        self.span
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CompoundLiteralExpression<'tree> {
    pub span: ::treesitter_types::Span,
    pub r#type: TypeDescriptor<'tree>,
    pub value: InitializerList<'tree>,
}
impl<'tree> ::treesitter_types::FromNode<'tree> for CompoundLiteralExpression<'tree> {
    #[allow(clippy::match_single_binding, clippy::suspicious_else_formatting)]
    fn from_node(
        node: ::treesitter_types::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        debug_assert_eq!(node.kind(), "compound_literal_expression");
        Ok(Self {
            span: ::treesitter_types::Span::from(node),
            r#type: {
                let child = node
                    .child_by_field_name("type")
                    .ok_or_else(|| ::treesitter_types::ParseError::missing_field("type", node))?;
                ::treesitter_types::runtime::maybe_grow_stack(|| {
                    <TypeDescriptor as ::treesitter_types::FromNode>::from_node(child, src)
                })?
            },
            value: {
                let child = node
                    .child_by_field_name("value")
                    .ok_or_else(|| ::treesitter_types::ParseError::missing_field("value", node))?;
                ::treesitter_types::runtime::maybe_grow_stack(|| {
                    <InitializerList as ::treesitter_types::FromNode>::from_node(child, src)
                })?
            },
        })
    }
}
impl ::treesitter_types::Spanned for CompoundLiteralExpression<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        self.span
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CompoundStatement<'tree> {
    pub span: ::treesitter_types::Span,
    pub children: ::std::vec::Vec<CompoundStatementChildren<'tree>>,
}
impl<'tree> ::treesitter_types::FromNode<'tree> for CompoundStatement<'tree> {
    #[allow(clippy::match_single_binding, clippy::suspicious_else_formatting)]
    fn from_node(
        node: ::treesitter_types::tree_sitter::Node<'tree>,
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
                    items.push(::treesitter_types::runtime::maybe_grow_stack(|| {
                        <CompoundStatementChildren as ::treesitter_types::FromNode>::from_node(
                            child, src,
                        )
                    })?);
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
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ConcatenatedString<'tree> {
    pub span: ::treesitter_types::Span,
    pub children: ::std::vec::Vec<ConcatenatedStringChildren<'tree>>,
}
impl<'tree> ::treesitter_types::FromNode<'tree> for ConcatenatedString<'tree> {
    #[allow(clippy::match_single_binding, clippy::suspicious_else_formatting)]
    fn from_node(
        node: ::treesitter_types::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        debug_assert_eq!(node.kind(), "concatenated_string");
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
                        <ConcatenatedStringChildren as ::treesitter_types::FromNode>::from_node(
                            child, src,
                        )
                    })?);
                }
                items
            },
        })
    }
}
impl ::treesitter_types::Spanned for ConcatenatedString<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        self.span
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ConditionalExpression<'tree> {
    pub span: ::treesitter_types::Span,
    pub alternative: Expression<'tree>,
    pub condition: Expression<'tree>,
    pub consequence: ::core::option::Option<ConditionalExpressionConsequence<'tree>>,
}
impl<'tree> ::treesitter_types::FromNode<'tree> for ConditionalExpression<'tree> {
    #[allow(clippy::match_single_binding, clippy::suspicious_else_formatting)]
    fn from_node(
        node: ::treesitter_types::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        debug_assert_eq!(node.kind(), "conditional_expression");
        Ok(Self {
            span: ::treesitter_types::Span::from(node),
            alternative: {
                let child = node.child_by_field_name("alternative").ok_or_else(|| {
                    ::treesitter_types::ParseError::missing_field("alternative", node)
                })?;
                ::treesitter_types::runtime::maybe_grow_stack(|| {
                    <Expression as ::treesitter_types::FromNode>::from_node(child, src)
                })?
            },
            condition: {
                let child = node.child_by_field_name("condition").ok_or_else(|| {
                    ::treesitter_types::ParseError::missing_field("condition", node)
                })?;
                ::treesitter_types::runtime::maybe_grow_stack(|| {
                    <Expression as ::treesitter_types::FromNode>::from_node(child, src)
                })?
            },
            consequence: match node.child_by_field_name("consequence") {
                Some(child) => Some(::treesitter_types::runtime::maybe_grow_stack(|| {
                    <ConditionalExpressionConsequence as ::treesitter_types::FromNode>::from_node(
                        child, src,
                    )
                })?),
                None => None,
            },
        })
    }
}
impl ::treesitter_types::Spanned for ConditionalExpression<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        self.span
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ContinueStatement<'tree> {
    pub span: ::treesitter_types::Span,
    text: &'tree str,
}
impl<'tree> ::treesitter_types::FromNode<'tree> for ContinueStatement<'tree> {
    fn from_node(
        node: ::treesitter_types::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        debug_assert_eq!(node.kind(), "continue_statement");
        Ok(Self {
            span: ::treesitter_types::Span::from(node),
            text: node.utf8_text(src)?,
        })
    }
}
impl<'tree> ::treesitter_types::LeafNode<'tree> for ContinueStatement<'tree> {
    fn text(&self) -> &'tree str {
        self.text
    }
}
impl ::treesitter_types::Spanned for ContinueStatement<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        self.span
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Declaration<'tree> {
    pub span: ::treesitter_types::Span,
    pub declarator: ::std::vec::Vec<DeclarationDeclarator<'tree>>,
    pub r#type: TypeSpecifier<'tree>,
    pub children: ::std::vec::Vec<DeclarationChildren<'tree>>,
}
impl<'tree> ::treesitter_types::FromNode<'tree> for Declaration<'tree> {
    #[allow(clippy::match_single_binding, clippy::suspicious_else_formatting)]
    fn from_node(
        node: ::treesitter_types::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        debug_assert_eq!(node.kind(), "declaration");
        Ok(Self {
            span: ::treesitter_types::Span::from(node),
            declarator: {
                let mut cursor = node.walk();
                let mut items = ::std::vec::Vec::new();
                for child in node.children_by_field_name("declarator", &mut cursor) {
                    items.push(::treesitter_types::runtime::maybe_grow_stack(|| {
                        <DeclarationDeclarator as ::treesitter_types::FromNode>::from_node(
                            child, src,
                        )
                    })?);
                }
                items
            },
            r#type: {
                let child = node
                    .child_by_field_name("type")
                    .ok_or_else(|| ::treesitter_types::ParseError::missing_field("type", node))?;
                ::treesitter_types::runtime::maybe_grow_stack(|| {
                    <TypeSpecifier as ::treesitter_types::FromNode>::from_node(child, src)
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
                    items.push(::treesitter_types::runtime::maybe_grow_stack(|| {
                        <DeclarationChildren as ::treesitter_types::FromNode>::from_node(child, src)
                    })?);
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
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct DeclarationList<'tree> {
    pub span: ::treesitter_types::Span,
    pub children: ::std::vec::Vec<DeclarationListChildren<'tree>>,
}
impl<'tree> ::treesitter_types::FromNode<'tree> for DeclarationList<'tree> {
    #[allow(clippy::match_single_binding, clippy::suspicious_else_formatting)]
    fn from_node(
        node: ::treesitter_types::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        debug_assert_eq!(node.kind(), "declaration_list");
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
                        <DeclarationListChildren as ::treesitter_types::FromNode>::from_node(
                            child, src,
                        )
                    })?);
                }
                items
            },
        })
    }
}
impl ::treesitter_types::Spanned for DeclarationList<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        self.span
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct DoStatement<'tree> {
    pub span: ::treesitter_types::Span,
    pub body: Statement<'tree>,
    pub condition: ParenthesizedExpression<'tree>,
}
impl<'tree> ::treesitter_types::FromNode<'tree> for DoStatement<'tree> {
    #[allow(clippy::match_single_binding, clippy::suspicious_else_formatting)]
    fn from_node(
        node: ::treesitter_types::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        debug_assert_eq!(node.kind(), "do_statement");
        Ok(Self {
            span: ::treesitter_types::Span::from(node),
            body: {
                let child = node
                    .child_by_field_name("body")
                    .ok_or_else(|| ::treesitter_types::ParseError::missing_field("body", node))?;
                ::treesitter_types::runtime::maybe_grow_stack(|| {
                    <Statement as ::treesitter_types::FromNode>::from_node(child, src)
                })?
            },
            condition: {
                let child = node.child_by_field_name("condition").ok_or_else(|| {
                    ::treesitter_types::ParseError::missing_field("condition", node)
                })?;
                ::treesitter_types::runtime::maybe_grow_stack(|| {
                    <ParenthesizedExpression as ::treesitter_types::FromNode>::from_node(child, src)
                })?
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
pub struct ElseClause<'tree> {
    pub span: ::treesitter_types::Span,
    pub children: Statement<'tree>,
}
impl<'tree> ::treesitter_types::FromNode<'tree> for ElseClause<'tree> {
    #[allow(clippy::match_single_binding, clippy::suspicious_else_formatting)]
    fn from_node(
        node: ::treesitter_types::tree_sitter::Node<'tree>,
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
                                        ::treesitter_types::runtime::maybe_grow_stack(|| <Statement as ::treesitter_types::FromNode>::from_node(
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
                                            ::treesitter_types::runtime::maybe_grow_stack(|| <Statement as ::treesitter_types::FromNode>::from_node(
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
                    <Statement as ::treesitter_types::FromNode>::from_node(child, src)
                })?
            },
        })
    }
}
impl ::treesitter_types::Spanned for ElseClause<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        self.span
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct EnumSpecifier<'tree> {
    pub span: ::treesitter_types::Span,
    pub body: ::core::option::Option<EnumeratorList<'tree>>,
    pub name: ::core::option::Option<TypeIdentifier<'tree>>,
    pub underlying_type: ::core::option::Option<PrimitiveType<'tree>>,
    pub children: ::core::option::Option<AttributeSpecifier<'tree>>,
}
impl<'tree> ::treesitter_types::FromNode<'tree> for EnumSpecifier<'tree> {
    #[allow(clippy::match_single_binding, clippy::suspicious_else_formatting)]
    fn from_node(
        node: ::treesitter_types::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        debug_assert_eq!(node.kind(), "enum_specifier");
        Ok(Self {
            span: ::treesitter_types::Span::from(node),
            body: match node.child_by_field_name("body") {
                Some(child) => Some(::treesitter_types::runtime::maybe_grow_stack(|| {
                    <EnumeratorList as ::treesitter_types::FromNode>::from_node(child, src)
                })?),
                None => None,
            },
            name: match node.child_by_field_name("name") {
                Some(child) => Some(::treesitter_types::runtime::maybe_grow_stack(|| {
                    <TypeIdentifier as ::treesitter_types::FromNode>::from_node(child, src)
                })?),
                None => None,
            },
            underlying_type: match node.child_by_field_name("underlying_type") {
                Some(child) => Some(::treesitter_types::runtime::maybe_grow_stack(|| {
                    <PrimitiveType as ::treesitter_types::FromNode>::from_node(child, src)
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
                match non_field_children.first() {
                    Some(&child) => Some(::treesitter_types::runtime::maybe_grow_stack(|| {
                        <AttributeSpecifier as ::treesitter_types::FromNode>::from_node(child, src)
                    })?),
                    None => None,
                }
            },
        })
    }
}
impl ::treesitter_types::Spanned for EnumSpecifier<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        self.span
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Enumerator<'tree> {
    pub span: ::treesitter_types::Span,
    pub name: Identifier<'tree>,
    pub value: ::core::option::Option<Expression<'tree>>,
}
impl<'tree> ::treesitter_types::FromNode<'tree> for Enumerator<'tree> {
    #[allow(clippy::match_single_binding, clippy::suspicious_else_formatting)]
    fn from_node(
        node: ::treesitter_types::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        debug_assert_eq!(node.kind(), "enumerator");
        Ok(Self {
            span: ::treesitter_types::Span::from(node),
            name: {
                let child = node
                    .child_by_field_name("name")
                    .ok_or_else(|| ::treesitter_types::ParseError::missing_field("name", node))?;
                ::treesitter_types::runtime::maybe_grow_stack(|| {
                    <Identifier as ::treesitter_types::FromNode>::from_node(child, src)
                })?
            },
            value: match node.child_by_field_name("value") {
                Some(child) => Some(::treesitter_types::runtime::maybe_grow_stack(|| {
                    <Expression as ::treesitter_types::FromNode>::from_node(child, src)
                })?),
                None => None,
            },
        })
    }
}
impl ::treesitter_types::Spanned for Enumerator<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        self.span
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct EnumeratorList<'tree> {
    pub span: ::treesitter_types::Span,
    pub children: ::std::vec::Vec<EnumeratorListChildren<'tree>>,
}
impl<'tree> ::treesitter_types::FromNode<'tree> for EnumeratorList<'tree> {
    #[allow(clippy::match_single_binding, clippy::suspicious_else_formatting)]
    fn from_node(
        node: ::treesitter_types::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        debug_assert_eq!(node.kind(), "enumerator_list");
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
                        <EnumeratorListChildren as ::treesitter_types::FromNode>::from_node(
                            child, src,
                        )
                    })?);
                }
                items
            },
        })
    }
}
impl ::treesitter_types::Spanned for EnumeratorList<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        self.span
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ExpressionStatement<'tree> {
    pub span: ::treesitter_types::Span,
    pub children: ::core::option::Option<ExpressionStatementChildren<'tree>>,
}
impl<'tree> ::treesitter_types::FromNode<'tree> for ExpressionStatement<'tree> {
    #[allow(clippy::match_single_binding, clippy::suspicious_else_formatting)]
    fn from_node(
        node: ::treesitter_types::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        debug_assert_eq!(node.kind(), "expression_statement");
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
                        <ExpressionStatementChildren as ::treesitter_types::FromNode>::from_node(
                            child, src,
                        )
                    })?),
                    None => None,
                }
            },
        })
    }
}
impl ::treesitter_types::Spanned for ExpressionStatement<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        self.span
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ExtensionExpression<'tree> {
    pub span: ::treesitter_types::Span,
    pub children: Expression<'tree>,
}
impl<'tree> ::treesitter_types::FromNode<'tree> for ExtensionExpression<'tree> {
    #[allow(clippy::match_single_binding, clippy::suspicious_else_formatting)]
    fn from_node(
        node: ::treesitter_types::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        debug_assert_eq!(node.kind(), "extension_expression");
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
                                        ::treesitter_types::runtime::maybe_grow_stack(|| <Expression as ::treesitter_types::FromNode>::from_node(
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
                                            ::treesitter_types::runtime::maybe_grow_stack(|| <Expression as ::treesitter_types::FromNode>::from_node(
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
                    <Expression as ::treesitter_types::FromNode>::from_node(child, src)
                })?
            },
        })
    }
}
impl ::treesitter_types::Spanned for ExtensionExpression<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        self.span
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct FieldDeclaration<'tree> {
    pub span: ::treesitter_types::Span,
    pub declarator: ::std::vec::Vec<FieldDeclarator<'tree>>,
    pub r#type: TypeSpecifier<'tree>,
    pub children: ::std::vec::Vec<FieldDeclarationChildren<'tree>>,
}
impl<'tree> ::treesitter_types::FromNode<'tree> for FieldDeclaration<'tree> {
    #[allow(clippy::match_single_binding, clippy::suspicious_else_formatting)]
    fn from_node(
        node: ::treesitter_types::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        debug_assert_eq!(node.kind(), "field_declaration");
        Ok(Self {
            span: ::treesitter_types::Span::from(node),
            declarator: {
                let mut cursor = node.walk();
                let mut items = ::std::vec::Vec::new();
                for child in node.children_by_field_name("declarator", &mut cursor) {
                    items.push(::treesitter_types::runtime::maybe_grow_stack(|| {
                        <FieldDeclarator as ::treesitter_types::FromNode>::from_node(child, src)
                    })?);
                }
                items
            },
            r#type: {
                let child = node
                    .child_by_field_name("type")
                    .ok_or_else(|| ::treesitter_types::ParseError::missing_field("type", node))?;
                ::treesitter_types::runtime::maybe_grow_stack(|| {
                    <TypeSpecifier as ::treesitter_types::FromNode>::from_node(child, src)
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
                    items.push(::treesitter_types::runtime::maybe_grow_stack(|| {
                        <FieldDeclarationChildren as ::treesitter_types::FromNode>::from_node(
                            child, src,
                        )
                    })?);
                }
                items
            },
        })
    }
}
impl ::treesitter_types::Spanned for FieldDeclaration<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        self.span
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct FieldDeclarationList<'tree> {
    pub span: ::treesitter_types::Span,
    pub children: ::std::vec::Vec<FieldDeclarationListChildren<'tree>>,
}
impl<'tree> ::treesitter_types::FromNode<'tree> for FieldDeclarationList<'tree> {
    #[allow(clippy::match_single_binding, clippy::suspicious_else_formatting)]
    fn from_node(
        node: ::treesitter_types::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        debug_assert_eq!(node.kind(), "field_declaration_list");
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
                        <FieldDeclarationListChildren as ::treesitter_types::FromNode>::from_node(
                            child, src,
                        )
                    })?);
                }
                items
            },
        })
    }
}
impl ::treesitter_types::Spanned for FieldDeclarationList<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        self.span
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct FieldDesignator<'tree> {
    pub span: ::treesitter_types::Span,
    pub children: FieldIdentifier<'tree>,
}
impl<'tree> ::treesitter_types::FromNode<'tree> for FieldDesignator<'tree> {
    #[allow(clippy::match_single_binding, clippy::suspicious_else_formatting)]
    fn from_node(
        node: ::treesitter_types::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        debug_assert_eq!(node.kind(), "field_designator");
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
                                        ::treesitter_types::runtime::maybe_grow_stack(|| <FieldIdentifier as ::treesitter_types::FromNode>::from_node(
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
                                            ::treesitter_types::runtime::maybe_grow_stack(|| <FieldIdentifier as ::treesitter_types::FromNode>::from_node(
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
                    <FieldIdentifier as ::treesitter_types::FromNode>::from_node(child, src)
                })?
            },
        })
    }
}
impl ::treesitter_types::Spanned for FieldDesignator<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        self.span
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct FieldExpression<'tree> {
    pub span: ::treesitter_types::Span,
    pub argument: Expression<'tree>,
    pub field: FieldIdentifier<'tree>,
    pub operator: FieldExpressionOperator,
}
impl<'tree> ::treesitter_types::FromNode<'tree> for FieldExpression<'tree> {
    #[allow(clippy::match_single_binding, clippy::suspicious_else_formatting)]
    fn from_node(
        node: ::treesitter_types::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        debug_assert_eq!(node.kind(), "field_expression");
        Ok(Self {
            span: ::treesitter_types::Span::from(node),
            argument: {
                let child = node.child_by_field_name("argument").ok_or_else(|| {
                    ::treesitter_types::ParseError::missing_field("argument", node)
                })?;
                ::treesitter_types::runtime::maybe_grow_stack(|| {
                    <Expression as ::treesitter_types::FromNode>::from_node(child, src)
                })?
            },
            field: {
                let child = node
                    .child_by_field_name("field")
                    .ok_or_else(|| ::treesitter_types::ParseError::missing_field("field", node))?;
                ::treesitter_types::runtime::maybe_grow_stack(|| {
                    <FieldIdentifier as ::treesitter_types::FromNode>::from_node(child, src)
                })?
            },
            operator: {
                let child = node.child_by_field_name("operator").ok_or_else(|| {
                    ::treesitter_types::ParseError::missing_field("operator", node)
                })?;
                ::treesitter_types::runtime::maybe_grow_stack(|| {
                    <FieldExpressionOperator as ::treesitter_types::FromNode>::from_node(child, src)
                })?
            },
        })
    }
}
impl ::treesitter_types::Spanned for FieldExpression<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        self.span
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ForStatement<'tree> {
    pub span: ::treesitter_types::Span,
    pub body: Statement<'tree>,
    pub condition: ::core::option::Option<ForStatementCondition<'tree>>,
    pub initializer: ::core::option::Option<ForStatementInitializer<'tree>>,
    pub update: ::core::option::Option<ForStatementUpdate<'tree>>,
}
impl<'tree> ::treesitter_types::FromNode<'tree> for ForStatement<'tree> {
    #[allow(clippy::match_single_binding, clippy::suspicious_else_formatting)]
    fn from_node(
        node: ::treesitter_types::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        debug_assert_eq!(node.kind(), "for_statement");
        Ok(Self {
            span: ::treesitter_types::Span::from(node),
            body: {
                let child = node
                    .child_by_field_name("body")
                    .ok_or_else(|| ::treesitter_types::ParseError::missing_field("body", node))?;
                ::treesitter_types::runtime::maybe_grow_stack(|| {
                    <Statement as ::treesitter_types::FromNode>::from_node(child, src)
                })?
            },
            condition: match node.child_by_field_name("condition") {
                Some(child) => Some(::treesitter_types::runtime::maybe_grow_stack(|| {
                    <ForStatementCondition as ::treesitter_types::FromNode>::from_node(child, src)
                })?),
                None => None,
            },
            initializer: match node.child_by_field_name("initializer") {
                Some(child) => Some(::treesitter_types::runtime::maybe_grow_stack(|| {
                    <ForStatementInitializer as ::treesitter_types::FromNode>::from_node(child, src)
                })?),
                None => None,
            },
            update: match node.child_by_field_name("update") {
                Some(child) => Some(::treesitter_types::runtime::maybe_grow_stack(|| {
                    <ForStatementUpdate as ::treesitter_types::FromNode>::from_node(child, src)
                })?),
                None => None,
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
pub struct FunctionDeclarator<'tree> {
    pub span: ::treesitter_types::Span,
    pub declarator: FunctionDeclaratorDeclarator<'tree>,
    pub parameters: ParameterList<'tree>,
    pub children: ::std::vec::Vec<FunctionDeclaratorChildren<'tree>>,
}
impl<'tree> ::treesitter_types::FromNode<'tree> for FunctionDeclarator<'tree> {
    #[allow(clippy::match_single_binding, clippy::suspicious_else_formatting)]
    fn from_node(
        node: ::treesitter_types::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        debug_assert_eq!(node.kind(), "function_declarator");
        Ok(Self {
            span: ::treesitter_types::Span::from(node),
            declarator: {
                let child = node.child_by_field_name("declarator").ok_or_else(|| {
                    ::treesitter_types::ParseError::missing_field("declarator", node)
                })?;
                ::treesitter_types::runtime::maybe_grow_stack(|| {
                    <FunctionDeclaratorDeclarator as ::treesitter_types::FromNode>::from_node(
                        child, src,
                    )
                })?
            },
            parameters: {
                let child = node.child_by_field_name("parameters").ok_or_else(|| {
                    ::treesitter_types::ParseError::missing_field("parameters", node)
                })?;
                ::treesitter_types::runtime::maybe_grow_stack(|| {
                    <ParameterList as ::treesitter_types::FromNode>::from_node(child, src)
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
                    items.push(::treesitter_types::runtime::maybe_grow_stack(|| {
                        <FunctionDeclaratorChildren as ::treesitter_types::FromNode>::from_node(
                            child, src,
                        )
                    })?);
                }
                items
            },
        })
    }
}
impl ::treesitter_types::Spanned for FunctionDeclarator<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        self.span
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct FunctionDefinition<'tree> {
    pub span: ::treesitter_types::Span,
    pub body: CompoundStatement<'tree>,
    pub declarator: Declarator<'tree>,
    pub r#type: TypeSpecifier<'tree>,
    pub children: ::std::vec::Vec<FunctionDefinitionChildren<'tree>>,
}
impl<'tree> ::treesitter_types::FromNode<'tree> for FunctionDefinition<'tree> {
    #[allow(clippy::match_single_binding, clippy::suspicious_else_formatting)]
    fn from_node(
        node: ::treesitter_types::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        debug_assert_eq!(node.kind(), "function_definition");
        Ok(Self {
            span: ::treesitter_types::Span::from(node),
            body: {
                let child = node
                    .child_by_field_name("body")
                    .ok_or_else(|| ::treesitter_types::ParseError::missing_field("body", node))?;
                ::treesitter_types::runtime::maybe_grow_stack(|| {
                    <CompoundStatement as ::treesitter_types::FromNode>::from_node(child, src)
                })?
            },
            declarator: {
                let child = node.child_by_field_name("declarator").ok_or_else(|| {
                    ::treesitter_types::ParseError::missing_field("declarator", node)
                })?;
                ::treesitter_types::runtime::maybe_grow_stack(|| {
                    <Declarator as ::treesitter_types::FromNode>::from_node(child, src)
                })?
            },
            r#type: {
                let child = node
                    .child_by_field_name("type")
                    .ok_or_else(|| ::treesitter_types::ParseError::missing_field("type", node))?;
                ::treesitter_types::runtime::maybe_grow_stack(|| {
                    <TypeSpecifier as ::treesitter_types::FromNode>::from_node(child, src)
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
                    items.push(::treesitter_types::runtime::maybe_grow_stack(|| {
                        <FunctionDefinitionChildren as ::treesitter_types::FromNode>::from_node(
                            child, src,
                        )
                    })?);
                }
                items
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
pub struct GenericExpression<'tree> {
    pub span: ::treesitter_types::Span,
    pub children: ::std::vec::Vec<GenericExpressionChildren<'tree>>,
}
impl<'tree> ::treesitter_types::FromNode<'tree> for GenericExpression<'tree> {
    #[allow(clippy::match_single_binding, clippy::suspicious_else_formatting)]
    fn from_node(
        node: ::treesitter_types::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        debug_assert_eq!(node.kind(), "generic_expression");
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
                        <GenericExpressionChildren as ::treesitter_types::FromNode>::from_node(
                            child, src,
                        )
                    })?);
                }
                items
            },
        })
    }
}
impl ::treesitter_types::Spanned for GenericExpression<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        self.span
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct GnuAsmClobberList<'tree> {
    pub span: ::treesitter_types::Span,
    pub register: ::std::vec::Vec<GnuAsmClobberListRegister<'tree>>,
}
impl<'tree> ::treesitter_types::FromNode<'tree> for GnuAsmClobberList<'tree> {
    #[allow(clippy::match_single_binding, clippy::suspicious_else_formatting)]
    fn from_node(
        node: ::treesitter_types::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        debug_assert_eq!(node.kind(), "gnu_asm_clobber_list");
        Ok(Self {
            span: ::treesitter_types::Span::from(node),
            register: {
                let mut cursor = node.walk();
                let mut items = ::std::vec::Vec::new();
                for child in node.children_by_field_name("register", &mut cursor) {
                    items.push(::treesitter_types::runtime::maybe_grow_stack(|| {
                        <GnuAsmClobberListRegister as ::treesitter_types::FromNode>::from_node(
                            child, src,
                        )
                    })?);
                }
                items
            },
        })
    }
}
impl ::treesitter_types::Spanned for GnuAsmClobberList<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        self.span
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct GnuAsmExpression<'tree> {
    pub span: ::treesitter_types::Span,
    pub assembly_code: GnuAsmExpressionAssemblyCode<'tree>,
    pub clobbers: ::core::option::Option<GnuAsmClobberList<'tree>>,
    pub goto_labels: ::core::option::Option<GnuAsmGotoList<'tree>>,
    pub input_operands: ::core::option::Option<GnuAsmInputOperandList<'tree>>,
    pub output_operands: ::core::option::Option<GnuAsmOutputOperandList<'tree>>,
    pub children: ::std::vec::Vec<GnuAsmQualifier<'tree>>,
}
impl<'tree> ::treesitter_types::FromNode<'tree> for GnuAsmExpression<'tree> {
    #[allow(clippy::match_single_binding, clippy::suspicious_else_formatting)]
    fn from_node(
        node: ::treesitter_types::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        debug_assert_eq!(node.kind(), "gnu_asm_expression");
        Ok(Self {
            span: ::treesitter_types::Span::from(node),
            assembly_code: {
                let child = node.child_by_field_name("assembly_code").ok_or_else(|| {
                    ::treesitter_types::ParseError::missing_field("assembly_code", node)
                })?;
                ::treesitter_types::runtime::maybe_grow_stack(|| {
                    <GnuAsmExpressionAssemblyCode as ::treesitter_types::FromNode>::from_node(
                        child, src,
                    )
                })?
            },
            clobbers: match node.child_by_field_name("clobbers") {
                Some(child) => Some(::treesitter_types::runtime::maybe_grow_stack(|| {
                    <GnuAsmClobberList as ::treesitter_types::FromNode>::from_node(child, src)
                })?),
                None => None,
            },
            goto_labels: match node.child_by_field_name("goto_labels") {
                Some(child) => Some(::treesitter_types::runtime::maybe_grow_stack(|| {
                    <GnuAsmGotoList as ::treesitter_types::FromNode>::from_node(child, src)
                })?),
                None => None,
            },
            input_operands: match node.child_by_field_name("input_operands") {
                Some(child) => Some(::treesitter_types::runtime::maybe_grow_stack(|| {
                    <GnuAsmInputOperandList as ::treesitter_types::FromNode>::from_node(child, src)
                })?),
                None => None,
            },
            output_operands: match node.child_by_field_name("output_operands") {
                Some(child) => Some(::treesitter_types::runtime::maybe_grow_stack(|| {
                    <GnuAsmOutputOperandList as ::treesitter_types::FromNode>::from_node(child, src)
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
                    items.push(::treesitter_types::runtime::maybe_grow_stack(|| {
                        <GnuAsmQualifier as ::treesitter_types::FromNode>::from_node(child, src)
                    })?);
                }
                items
            },
        })
    }
}
impl ::treesitter_types::Spanned for GnuAsmExpression<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        self.span
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct GnuAsmGotoList<'tree> {
    pub span: ::treesitter_types::Span,
    pub label: ::std::vec::Vec<Identifier<'tree>>,
}
impl<'tree> ::treesitter_types::FromNode<'tree> for GnuAsmGotoList<'tree> {
    #[allow(clippy::match_single_binding, clippy::suspicious_else_formatting)]
    fn from_node(
        node: ::treesitter_types::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        debug_assert_eq!(node.kind(), "gnu_asm_goto_list");
        Ok(Self {
            span: ::treesitter_types::Span::from(node),
            label: {
                let mut cursor = node.walk();
                let mut items = ::std::vec::Vec::new();
                for child in node.children_by_field_name("label", &mut cursor) {
                    items.push(::treesitter_types::runtime::maybe_grow_stack(|| {
                        <Identifier as ::treesitter_types::FromNode>::from_node(child, src)
                    })?);
                }
                items
            },
        })
    }
}
impl ::treesitter_types::Spanned for GnuAsmGotoList<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        self.span
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct GnuAsmInputOperand<'tree> {
    pub span: ::treesitter_types::Span,
    pub constraint: StringLiteral<'tree>,
    pub symbol: ::core::option::Option<Identifier<'tree>>,
    pub value: Expression<'tree>,
}
impl<'tree> ::treesitter_types::FromNode<'tree> for GnuAsmInputOperand<'tree> {
    #[allow(clippy::match_single_binding, clippy::suspicious_else_formatting)]
    fn from_node(
        node: ::treesitter_types::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        debug_assert_eq!(node.kind(), "gnu_asm_input_operand");
        Ok(Self {
            span: ::treesitter_types::Span::from(node),
            constraint: {
                let child = node.child_by_field_name("constraint").ok_or_else(|| {
                    ::treesitter_types::ParseError::missing_field("constraint", node)
                })?;
                ::treesitter_types::runtime::maybe_grow_stack(|| {
                    <StringLiteral as ::treesitter_types::FromNode>::from_node(child, src)
                })?
            },
            symbol: match node.child_by_field_name("symbol") {
                Some(child) => Some(::treesitter_types::runtime::maybe_grow_stack(|| {
                    <Identifier as ::treesitter_types::FromNode>::from_node(child, src)
                })?),
                None => None,
            },
            value: {
                let child = node
                    .child_by_field_name("value")
                    .ok_or_else(|| ::treesitter_types::ParseError::missing_field("value", node))?;
                ::treesitter_types::runtime::maybe_grow_stack(|| {
                    <Expression as ::treesitter_types::FromNode>::from_node(child, src)
                })?
            },
        })
    }
}
impl ::treesitter_types::Spanned for GnuAsmInputOperand<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        self.span
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct GnuAsmInputOperandList<'tree> {
    pub span: ::treesitter_types::Span,
    pub operand: ::std::vec::Vec<GnuAsmInputOperand<'tree>>,
}
impl<'tree> ::treesitter_types::FromNode<'tree> for GnuAsmInputOperandList<'tree> {
    #[allow(clippy::match_single_binding, clippy::suspicious_else_formatting)]
    fn from_node(
        node: ::treesitter_types::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        debug_assert_eq!(node.kind(), "gnu_asm_input_operand_list");
        Ok(Self {
            span: ::treesitter_types::Span::from(node),
            operand: {
                let mut cursor = node.walk();
                let mut items = ::std::vec::Vec::new();
                for child in node.children_by_field_name("operand", &mut cursor) {
                    items.push(::treesitter_types::runtime::maybe_grow_stack(|| {
                        <GnuAsmInputOperand as ::treesitter_types::FromNode>::from_node(child, src)
                    })?);
                }
                items
            },
        })
    }
}
impl ::treesitter_types::Spanned for GnuAsmInputOperandList<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        self.span
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct GnuAsmOutputOperand<'tree> {
    pub span: ::treesitter_types::Span,
    pub constraint: StringLiteral<'tree>,
    pub symbol: ::core::option::Option<Identifier<'tree>>,
    pub value: Expression<'tree>,
}
impl<'tree> ::treesitter_types::FromNode<'tree> for GnuAsmOutputOperand<'tree> {
    #[allow(clippy::match_single_binding, clippy::suspicious_else_formatting)]
    fn from_node(
        node: ::treesitter_types::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        debug_assert_eq!(node.kind(), "gnu_asm_output_operand");
        Ok(Self {
            span: ::treesitter_types::Span::from(node),
            constraint: {
                let child = node.child_by_field_name("constraint").ok_or_else(|| {
                    ::treesitter_types::ParseError::missing_field("constraint", node)
                })?;
                ::treesitter_types::runtime::maybe_grow_stack(|| {
                    <StringLiteral as ::treesitter_types::FromNode>::from_node(child, src)
                })?
            },
            symbol: match node.child_by_field_name("symbol") {
                Some(child) => Some(::treesitter_types::runtime::maybe_grow_stack(|| {
                    <Identifier as ::treesitter_types::FromNode>::from_node(child, src)
                })?),
                None => None,
            },
            value: {
                let child = node
                    .child_by_field_name("value")
                    .ok_or_else(|| ::treesitter_types::ParseError::missing_field("value", node))?;
                ::treesitter_types::runtime::maybe_grow_stack(|| {
                    <Expression as ::treesitter_types::FromNode>::from_node(child, src)
                })?
            },
        })
    }
}
impl ::treesitter_types::Spanned for GnuAsmOutputOperand<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        self.span
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct GnuAsmOutputOperandList<'tree> {
    pub span: ::treesitter_types::Span,
    pub operand: ::std::vec::Vec<GnuAsmOutputOperand<'tree>>,
}
impl<'tree> ::treesitter_types::FromNode<'tree> for GnuAsmOutputOperandList<'tree> {
    #[allow(clippy::match_single_binding, clippy::suspicious_else_formatting)]
    fn from_node(
        node: ::treesitter_types::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        debug_assert_eq!(node.kind(), "gnu_asm_output_operand_list");
        Ok(Self {
            span: ::treesitter_types::Span::from(node),
            operand: {
                let mut cursor = node.walk();
                let mut items = ::std::vec::Vec::new();
                for child in node.children_by_field_name("operand", &mut cursor) {
                    items.push(::treesitter_types::runtime::maybe_grow_stack(|| {
                        <GnuAsmOutputOperand as ::treesitter_types::FromNode>::from_node(child, src)
                    })?);
                }
                items
            },
        })
    }
}
impl ::treesitter_types::Spanned for GnuAsmOutputOperandList<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        self.span
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct GnuAsmQualifier<'tree> {
    pub span: ::treesitter_types::Span,
    text: &'tree str,
}
impl<'tree> ::treesitter_types::FromNode<'tree> for GnuAsmQualifier<'tree> {
    fn from_node(
        node: ::treesitter_types::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        debug_assert_eq!(node.kind(), "gnu_asm_qualifier");
        Ok(Self {
            span: ::treesitter_types::Span::from(node),
            text: node.utf8_text(src)?,
        })
    }
}
impl<'tree> ::treesitter_types::LeafNode<'tree> for GnuAsmQualifier<'tree> {
    fn text(&self) -> &'tree str {
        self.text
    }
}
impl ::treesitter_types::Spanned for GnuAsmQualifier<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        self.span
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct GotoStatement<'tree> {
    pub span: ::treesitter_types::Span,
    pub label: StatementIdentifier<'tree>,
}
impl<'tree> ::treesitter_types::FromNode<'tree> for GotoStatement<'tree> {
    #[allow(clippy::match_single_binding, clippy::suspicious_else_formatting)]
    fn from_node(
        node: ::treesitter_types::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        debug_assert_eq!(node.kind(), "goto_statement");
        Ok(Self {
            span: ::treesitter_types::Span::from(node),
            label: {
                let child = node
                    .child_by_field_name("label")
                    .ok_or_else(|| ::treesitter_types::ParseError::missing_field("label", node))?;
                ::treesitter_types::runtime::maybe_grow_stack(|| {
                    <StatementIdentifier as ::treesitter_types::FromNode>::from_node(child, src)
                })?
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
    pub alternative: ::core::option::Option<ElseClause<'tree>>,
    pub condition: ParenthesizedExpression<'tree>,
    pub consequence: Statement<'tree>,
}
impl<'tree> ::treesitter_types::FromNode<'tree> for IfStatement<'tree> {
    #[allow(clippy::match_single_binding, clippy::suspicious_else_formatting)]
    fn from_node(
        node: ::treesitter_types::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        debug_assert_eq!(node.kind(), "if_statement");
        Ok(Self {
            span: ::treesitter_types::Span::from(node),
            alternative: match node.child_by_field_name("alternative") {
                Some(child) => Some(::treesitter_types::runtime::maybe_grow_stack(|| {
                    <ElseClause as ::treesitter_types::FromNode>::from_node(child, src)
                })?),
                None => None,
            },
            condition: {
                let child = node.child_by_field_name("condition").ok_or_else(|| {
                    ::treesitter_types::ParseError::missing_field("condition", node)
                })?;
                ::treesitter_types::runtime::maybe_grow_stack(|| {
                    <ParenthesizedExpression as ::treesitter_types::FromNode>::from_node(child, src)
                })?
            },
            consequence: {
                let child = node.child_by_field_name("consequence").ok_or_else(|| {
                    ::treesitter_types::ParseError::missing_field("consequence", node)
                })?;
                ::treesitter_types::runtime::maybe_grow_stack(|| {
                    <Statement as ::treesitter_types::FromNode>::from_node(child, src)
                })?
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
pub struct InitDeclarator<'tree> {
    pub span: ::treesitter_types::Span,
    pub declarator: Declarator<'tree>,
    pub value: InitDeclaratorValue<'tree>,
}
impl<'tree> ::treesitter_types::FromNode<'tree> for InitDeclarator<'tree> {
    #[allow(clippy::match_single_binding, clippy::suspicious_else_formatting)]
    fn from_node(
        node: ::treesitter_types::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        debug_assert_eq!(node.kind(), "init_declarator");
        Ok(Self {
            span: ::treesitter_types::Span::from(node),
            declarator: {
                let child = node.child_by_field_name("declarator").ok_or_else(|| {
                    ::treesitter_types::ParseError::missing_field("declarator", node)
                })?;
                ::treesitter_types::runtime::maybe_grow_stack(|| {
                    <Declarator as ::treesitter_types::FromNode>::from_node(child, src)
                })?
            },
            value: {
                let child = node
                    .child_by_field_name("value")
                    .ok_or_else(|| ::treesitter_types::ParseError::missing_field("value", node))?;
                ::treesitter_types::runtime::maybe_grow_stack(|| {
                    <InitDeclaratorValue as ::treesitter_types::FromNode>::from_node(child, src)
                })?
            },
        })
    }
}
impl ::treesitter_types::Spanned for InitDeclarator<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        self.span
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct InitializerList<'tree> {
    pub span: ::treesitter_types::Span,
    pub children: ::std::vec::Vec<InitializerListChildren<'tree>>,
}
impl<'tree> ::treesitter_types::FromNode<'tree> for InitializerList<'tree> {
    #[allow(clippy::match_single_binding, clippy::suspicious_else_formatting)]
    fn from_node(
        node: ::treesitter_types::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        debug_assert_eq!(node.kind(), "initializer_list");
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
                        <InitializerListChildren as ::treesitter_types::FromNode>::from_node(
                            child, src,
                        )
                    })?);
                }
                items
            },
        })
    }
}
impl ::treesitter_types::Spanned for InitializerList<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        self.span
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct InitializerPair<'tree> {
    pub span: ::treesitter_types::Span,
    pub designator: ::std::vec::Vec<InitializerPairDesignator<'tree>>,
    pub value: InitializerPairValue<'tree>,
}
impl<'tree> ::treesitter_types::FromNode<'tree> for InitializerPair<'tree> {
    #[allow(clippy::match_single_binding, clippy::suspicious_else_formatting)]
    fn from_node(
        node: ::treesitter_types::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        debug_assert_eq!(node.kind(), "initializer_pair");
        Ok(Self {
            span: ::treesitter_types::Span::from(node),
            designator: {
                let mut cursor = node.walk();
                let mut items = ::std::vec::Vec::new();
                for child in node.children_by_field_name("designator", &mut cursor) {
                    items.push(::treesitter_types::runtime::maybe_grow_stack(|| {
                        <InitializerPairDesignator as ::treesitter_types::FromNode>::from_node(
                            child, src,
                        )
                    })?);
                }
                items
            },
            value: {
                let child = node
                    .child_by_field_name("value")
                    .ok_or_else(|| ::treesitter_types::ParseError::missing_field("value", node))?;
                ::treesitter_types::runtime::maybe_grow_stack(|| {
                    <InitializerPairValue as ::treesitter_types::FromNode>::from_node(child, src)
                })?
            },
        })
    }
}
impl ::treesitter_types::Spanned for InitializerPair<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        self.span
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct LabeledStatement<'tree> {
    pub span: ::treesitter_types::Span,
    pub label: StatementIdentifier<'tree>,
    pub children: LabeledStatementChildren<'tree>,
}
impl<'tree> ::treesitter_types::FromNode<'tree> for LabeledStatement<'tree> {
    #[allow(clippy::match_single_binding, clippy::suspicious_else_formatting)]
    fn from_node(
        node: ::treesitter_types::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        debug_assert_eq!(node.kind(), "labeled_statement");
        Ok(Self {
            span: ::treesitter_types::Span::from(node),
            label: {
                let child = node
                    .child_by_field_name("label")
                    .ok_or_else(|| ::treesitter_types::ParseError::missing_field("label", node))?;
                ::treesitter_types::runtime::maybe_grow_stack(|| {
                    <StatementIdentifier as ::treesitter_types::FromNode>::from_node(child, src)
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
                                        ::treesitter_types::runtime::maybe_grow_stack(|| <LabeledStatementChildren as ::treesitter_types::FromNode>::from_node(
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
                                            ::treesitter_types::runtime::maybe_grow_stack(|| <LabeledStatementChildren as ::treesitter_types::FromNode>::from_node(
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
                    <LabeledStatementChildren as ::treesitter_types::FromNode>::from_node(
                        child, src,
                    )
                })?
            },
        })
    }
}
impl ::treesitter_types::Spanned for LabeledStatement<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        self.span
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct LinkageSpecification<'tree> {
    pub span: ::treesitter_types::Span,
    pub body: LinkageSpecificationBody<'tree>,
    pub value: StringLiteral<'tree>,
}
impl<'tree> ::treesitter_types::FromNode<'tree> for LinkageSpecification<'tree> {
    #[allow(clippy::match_single_binding, clippy::suspicious_else_formatting)]
    fn from_node(
        node: ::treesitter_types::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        debug_assert_eq!(node.kind(), "linkage_specification");
        Ok(Self {
            span: ::treesitter_types::Span::from(node),
            body: {
                let child = node
                    .child_by_field_name("body")
                    .ok_or_else(|| ::treesitter_types::ParseError::missing_field("body", node))?;
                ::treesitter_types::runtime::maybe_grow_stack(|| {
                    <LinkageSpecificationBody as ::treesitter_types::FromNode>::from_node(
                        child, src,
                    )
                })?
            },
            value: {
                let child = node
                    .child_by_field_name("value")
                    .ok_or_else(|| ::treesitter_types::ParseError::missing_field("value", node))?;
                ::treesitter_types::runtime::maybe_grow_stack(|| {
                    <StringLiteral as ::treesitter_types::FromNode>::from_node(child, src)
                })?
            },
        })
    }
}
impl ::treesitter_types::Spanned for LinkageSpecification<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        self.span
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct MacroTypeSpecifier<'tree> {
    pub span: ::treesitter_types::Span,
    pub name: Identifier<'tree>,
    pub r#type: TypeDescriptor<'tree>,
}
impl<'tree> ::treesitter_types::FromNode<'tree> for MacroTypeSpecifier<'tree> {
    #[allow(clippy::match_single_binding, clippy::suspicious_else_formatting)]
    fn from_node(
        node: ::treesitter_types::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        debug_assert_eq!(node.kind(), "macro_type_specifier");
        Ok(Self {
            span: ::treesitter_types::Span::from(node),
            name: {
                let child = node
                    .child_by_field_name("name")
                    .ok_or_else(|| ::treesitter_types::ParseError::missing_field("name", node))?;
                ::treesitter_types::runtime::maybe_grow_stack(|| {
                    <Identifier as ::treesitter_types::FromNode>::from_node(child, src)
                })?
            },
            r#type: {
                let child = node
                    .child_by_field_name("type")
                    .ok_or_else(|| ::treesitter_types::ParseError::missing_field("type", node))?;
                ::treesitter_types::runtime::maybe_grow_stack(|| {
                    <TypeDescriptor as ::treesitter_types::FromNode>::from_node(child, src)
                })?
            },
        })
    }
}
impl ::treesitter_types::Spanned for MacroTypeSpecifier<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        self.span
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct MsBasedModifier<'tree> {
    pub span: ::treesitter_types::Span,
    pub children: ArgumentList<'tree>,
}
impl<'tree> ::treesitter_types::FromNode<'tree> for MsBasedModifier<'tree> {
    #[allow(clippy::match_single_binding, clippy::suspicious_else_formatting)]
    fn from_node(
        node: ::treesitter_types::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        debug_assert_eq!(node.kind(), "ms_based_modifier");
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
                                        ::treesitter_types::runtime::maybe_grow_stack(|| <ArgumentList as ::treesitter_types::FromNode>::from_node(
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
                                            ::treesitter_types::runtime::maybe_grow_stack(|| <ArgumentList as ::treesitter_types::FromNode>::from_node(
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
                    <ArgumentList as ::treesitter_types::FromNode>::from_node(child, src)
                })?
            },
        })
    }
}
impl ::treesitter_types::Spanned for MsBasedModifier<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        self.span
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct MsCallModifier<'tree> {
    pub span: ::treesitter_types::Span,
    text: &'tree str,
}
impl<'tree> ::treesitter_types::FromNode<'tree> for MsCallModifier<'tree> {
    fn from_node(
        node: ::treesitter_types::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        debug_assert_eq!(node.kind(), "ms_call_modifier");
        Ok(Self {
            span: ::treesitter_types::Span::from(node),
            text: node.utf8_text(src)?,
        })
    }
}
impl<'tree> ::treesitter_types::LeafNode<'tree> for MsCallModifier<'tree> {
    fn text(&self) -> &'tree str {
        self.text
    }
}
impl ::treesitter_types::Spanned for MsCallModifier<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        self.span
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct MsDeclspecModifier<'tree> {
    pub span: ::treesitter_types::Span,
    pub children: Identifier<'tree>,
}
impl<'tree> ::treesitter_types::FromNode<'tree> for MsDeclspecModifier<'tree> {
    #[allow(clippy::match_single_binding, clippy::suspicious_else_formatting)]
    fn from_node(
        node: ::treesitter_types::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        debug_assert_eq!(node.kind(), "ms_declspec_modifier");
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
                                        ::treesitter_types::runtime::maybe_grow_stack(|| <Identifier as ::treesitter_types::FromNode>::from_node(
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
                                            ::treesitter_types::runtime::maybe_grow_stack(|| <Identifier as ::treesitter_types::FromNode>::from_node(
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
                    <Identifier as ::treesitter_types::FromNode>::from_node(child, src)
                })?
            },
        })
    }
}
impl ::treesitter_types::Spanned for MsDeclspecModifier<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        self.span
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct MsPointerModifier<'tree> {
    pub span: ::treesitter_types::Span,
    pub children: MsPointerModifierChildren<'tree>,
}
impl<'tree> ::treesitter_types::FromNode<'tree> for MsPointerModifier<'tree> {
    #[allow(clippy::match_single_binding, clippy::suspicious_else_formatting)]
    fn from_node(
        node: ::treesitter_types::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        debug_assert_eq!(node.kind(), "ms_pointer_modifier");
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
                                        ::treesitter_types::runtime::maybe_grow_stack(|| <MsPointerModifierChildren as ::treesitter_types::FromNode>::from_node(
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
                                            ::treesitter_types::runtime::maybe_grow_stack(|| <MsPointerModifierChildren as ::treesitter_types::FromNode>::from_node(
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
                    <MsPointerModifierChildren as ::treesitter_types::FromNode>::from_node(
                        child, src,
                    )
                })?
            },
        })
    }
}
impl ::treesitter_types::Spanned for MsPointerModifier<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        self.span
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct MsUnalignedPtrModifier<'tree> {
    pub span: ::treesitter_types::Span,
    text: &'tree str,
}
impl<'tree> ::treesitter_types::FromNode<'tree> for MsUnalignedPtrModifier<'tree> {
    fn from_node(
        node: ::treesitter_types::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        debug_assert_eq!(node.kind(), "ms_unaligned_ptr_modifier");
        Ok(Self {
            span: ::treesitter_types::Span::from(node),
            text: node.utf8_text(src)?,
        })
    }
}
impl<'tree> ::treesitter_types::LeafNode<'tree> for MsUnalignedPtrModifier<'tree> {
    fn text(&self) -> &'tree str {
        self.text
    }
}
impl ::treesitter_types::Spanned for MsUnalignedPtrModifier<'_> {
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
        node: ::treesitter_types::tree_sitter::Node<'tree>,
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
pub struct OffsetofExpression<'tree> {
    pub span: ::treesitter_types::Span,
    pub member: FieldIdentifier<'tree>,
    pub r#type: TypeDescriptor<'tree>,
}
impl<'tree> ::treesitter_types::FromNode<'tree> for OffsetofExpression<'tree> {
    #[allow(clippy::match_single_binding, clippy::suspicious_else_formatting)]
    fn from_node(
        node: ::treesitter_types::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        debug_assert_eq!(node.kind(), "offsetof_expression");
        Ok(Self {
            span: ::treesitter_types::Span::from(node),
            member: {
                let child = node
                    .child_by_field_name("member")
                    .ok_or_else(|| ::treesitter_types::ParseError::missing_field("member", node))?;
                ::treesitter_types::runtime::maybe_grow_stack(|| {
                    <FieldIdentifier as ::treesitter_types::FromNode>::from_node(child, src)
                })?
            },
            r#type: {
                let child = node
                    .child_by_field_name("type")
                    .ok_or_else(|| ::treesitter_types::ParseError::missing_field("type", node))?;
                ::treesitter_types::runtime::maybe_grow_stack(|| {
                    <TypeDescriptor as ::treesitter_types::FromNode>::from_node(child, src)
                })?
            },
        })
    }
}
impl ::treesitter_types::Spanned for OffsetofExpression<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        self.span
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ParameterDeclaration<'tree> {
    pub span: ::treesitter_types::Span,
    pub declarator: ::core::option::Option<ParameterDeclarationDeclarator<'tree>>,
    pub r#type: TypeSpecifier<'tree>,
    pub children: ::std::vec::Vec<ParameterDeclarationChildren<'tree>>,
}
impl<'tree> ::treesitter_types::FromNode<'tree> for ParameterDeclaration<'tree> {
    #[allow(clippy::match_single_binding, clippy::suspicious_else_formatting)]
    fn from_node(
        node: ::treesitter_types::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        debug_assert_eq!(node.kind(), "parameter_declaration");
        Ok(Self {
            span: ::treesitter_types::Span::from(node),
            declarator: match node.child_by_field_name("declarator") {
                Some(child) => Some(::treesitter_types::runtime::maybe_grow_stack(|| {
                    <ParameterDeclarationDeclarator as ::treesitter_types::FromNode>::from_node(
                        child, src,
                    )
                })?),
                None => None,
            },
            r#type: {
                let child = node
                    .child_by_field_name("type")
                    .ok_or_else(|| ::treesitter_types::ParseError::missing_field("type", node))?;
                ::treesitter_types::runtime::maybe_grow_stack(|| {
                    <TypeSpecifier as ::treesitter_types::FromNode>::from_node(child, src)
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
                    items.push(::treesitter_types::runtime::maybe_grow_stack(|| {
                        <ParameterDeclarationChildren as ::treesitter_types::FromNode>::from_node(
                            child, src,
                        )
                    })?);
                }
                items
            },
        })
    }
}
impl ::treesitter_types::Spanned for ParameterDeclaration<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        self.span
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ParameterList<'tree> {
    pub span: ::treesitter_types::Span,
    pub children: ::std::vec::Vec<ParameterListChildren<'tree>>,
}
impl<'tree> ::treesitter_types::FromNode<'tree> for ParameterList<'tree> {
    #[allow(clippy::match_single_binding, clippy::suspicious_else_formatting)]
    fn from_node(
        node: ::treesitter_types::tree_sitter::Node<'tree>,
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
                    items.push(::treesitter_types::runtime::maybe_grow_stack(|| {
                        <ParameterListChildren as ::treesitter_types::FromNode>::from_node(
                            child, src,
                        )
                    })?);
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
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ParenthesizedDeclarator<'tree> {
    pub span: ::treesitter_types::Span,
    pub children: ::std::vec::Vec<ParenthesizedDeclaratorChildren<'tree>>,
}
impl<'tree> ::treesitter_types::FromNode<'tree> for ParenthesizedDeclarator<'tree> {
    #[allow(clippy::match_single_binding, clippy::suspicious_else_formatting)]
    fn from_node(
        node: ::treesitter_types::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        debug_assert_eq!(node.kind(), "parenthesized_declarator");
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
                            ::treesitter_types::runtime::maybe_grow_stack(|| <ParenthesizedDeclaratorChildren as ::treesitter_types::FromNode>::from_node(
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
impl ::treesitter_types::Spanned for ParenthesizedDeclarator<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        self.span
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ParenthesizedExpression<'tree> {
    pub span: ::treesitter_types::Span,
    pub children: ParenthesizedExpressionChildren<'tree>,
}
impl<'tree> ::treesitter_types::FromNode<'tree> for ParenthesizedExpression<'tree> {
    #[allow(clippy::match_single_binding, clippy::suspicious_else_formatting)]
    fn from_node(
        node: ::treesitter_types::tree_sitter::Node<'tree>,
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
                                        ::treesitter_types::runtime::maybe_grow_stack(|| <ParenthesizedExpressionChildren as ::treesitter_types::FromNode>::from_node(
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
                                            ::treesitter_types::runtime::maybe_grow_stack(|| <ParenthesizedExpressionChildren as ::treesitter_types::FromNode>::from_node(
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
                    <ParenthesizedExpressionChildren as ::treesitter_types::FromNode>::from_node(
                        child, src,
                    )
                })?
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
pub struct PointerDeclarator<'tree> {
    pub span: ::treesitter_types::Span,
    pub declarator: PointerDeclaratorDeclarator<'tree>,
    pub children: ::std::vec::Vec<PointerDeclaratorChildren<'tree>>,
}
impl<'tree> ::treesitter_types::FromNode<'tree> for PointerDeclarator<'tree> {
    #[allow(clippy::match_single_binding, clippy::suspicious_else_formatting)]
    fn from_node(
        node: ::treesitter_types::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        debug_assert_eq!(node.kind(), "pointer_declarator");
        Ok(Self {
            span: ::treesitter_types::Span::from(node),
            declarator: {
                let child = node.child_by_field_name("declarator").ok_or_else(|| {
                    ::treesitter_types::ParseError::missing_field("declarator", node)
                })?;
                ::treesitter_types::runtime::maybe_grow_stack(|| {
                    <PointerDeclaratorDeclarator as ::treesitter_types::FromNode>::from_node(
                        child, src,
                    )
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
                    items.push(::treesitter_types::runtime::maybe_grow_stack(|| {
                        <PointerDeclaratorChildren as ::treesitter_types::FromNode>::from_node(
                            child, src,
                        )
                    })?);
                }
                items
            },
        })
    }
}
impl ::treesitter_types::Spanned for PointerDeclarator<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        self.span
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PointerExpression<'tree> {
    pub span: ::treesitter_types::Span,
    pub argument: Expression<'tree>,
    pub operator: PointerExpressionOperator,
}
impl<'tree> ::treesitter_types::FromNode<'tree> for PointerExpression<'tree> {
    #[allow(clippy::match_single_binding, clippy::suspicious_else_formatting)]
    fn from_node(
        node: ::treesitter_types::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        debug_assert_eq!(node.kind(), "pointer_expression");
        Ok(Self {
            span: ::treesitter_types::Span::from(node),
            argument: {
                let child = node.child_by_field_name("argument").ok_or_else(|| {
                    ::treesitter_types::ParseError::missing_field("argument", node)
                })?;
                ::treesitter_types::runtime::maybe_grow_stack(|| {
                    <Expression as ::treesitter_types::FromNode>::from_node(child, src)
                })?
            },
            operator: {
                let child = node.child_by_field_name("operator").ok_or_else(|| {
                    ::treesitter_types::ParseError::missing_field("operator", node)
                })?;
                ::treesitter_types::runtime::maybe_grow_stack(|| {
                    <PointerExpressionOperator as ::treesitter_types::FromNode>::from_node(
                        child, src,
                    )
                })?
            },
        })
    }
}
impl ::treesitter_types::Spanned for PointerExpression<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        self.span
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PreprocCall<'tree> {
    pub span: ::treesitter_types::Span,
    pub argument: ::core::option::Option<PreprocArg<'tree>>,
    pub directive: PreprocDirective<'tree>,
}
impl<'tree> ::treesitter_types::FromNode<'tree> for PreprocCall<'tree> {
    #[allow(clippy::match_single_binding, clippy::suspicious_else_formatting)]
    fn from_node(
        node: ::treesitter_types::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        debug_assert_eq!(node.kind(), "preproc_call");
        Ok(Self {
            span: ::treesitter_types::Span::from(node),
            argument: match node.child_by_field_name("argument") {
                Some(child) => Some(::treesitter_types::runtime::maybe_grow_stack(|| {
                    <PreprocArg as ::treesitter_types::FromNode>::from_node(child, src)
                })?),
                None => None,
            },
            directive: {
                let child = node.child_by_field_name("directive").ok_or_else(|| {
                    ::treesitter_types::ParseError::missing_field("directive", node)
                })?;
                ::treesitter_types::runtime::maybe_grow_stack(|| {
                    <PreprocDirective as ::treesitter_types::FromNode>::from_node(child, src)
                })?
            },
        })
    }
}
impl ::treesitter_types::Spanned for PreprocCall<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        self.span
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PreprocDef<'tree> {
    pub span: ::treesitter_types::Span,
    pub name: Identifier<'tree>,
    pub value: ::core::option::Option<PreprocArg<'tree>>,
}
impl<'tree> ::treesitter_types::FromNode<'tree> for PreprocDef<'tree> {
    #[allow(clippy::match_single_binding, clippy::suspicious_else_formatting)]
    fn from_node(
        node: ::treesitter_types::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        debug_assert_eq!(node.kind(), "preproc_def");
        Ok(Self {
            span: ::treesitter_types::Span::from(node),
            name: {
                let child = node
                    .child_by_field_name("name")
                    .ok_or_else(|| ::treesitter_types::ParseError::missing_field("name", node))?;
                ::treesitter_types::runtime::maybe_grow_stack(|| {
                    <Identifier as ::treesitter_types::FromNode>::from_node(child, src)
                })?
            },
            value: match node.child_by_field_name("value") {
                Some(child) => Some(::treesitter_types::runtime::maybe_grow_stack(|| {
                    <PreprocArg as ::treesitter_types::FromNode>::from_node(child, src)
                })?),
                None => None,
            },
        })
    }
}
impl ::treesitter_types::Spanned for PreprocDef<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        self.span
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PreprocDefined<'tree> {
    pub span: ::treesitter_types::Span,
    pub children: Identifier<'tree>,
}
impl<'tree> ::treesitter_types::FromNode<'tree> for PreprocDefined<'tree> {
    #[allow(clippy::match_single_binding, clippy::suspicious_else_formatting)]
    fn from_node(
        node: ::treesitter_types::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        debug_assert_eq!(node.kind(), "preproc_defined");
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
                                        ::treesitter_types::runtime::maybe_grow_stack(|| <Identifier as ::treesitter_types::FromNode>::from_node(
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
                                            ::treesitter_types::runtime::maybe_grow_stack(|| <Identifier as ::treesitter_types::FromNode>::from_node(
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
                    <Identifier as ::treesitter_types::FromNode>::from_node(child, src)
                })?
            },
        })
    }
}
impl ::treesitter_types::Spanned for PreprocDefined<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        self.span
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PreprocElif<'tree> {
    pub span: ::treesitter_types::Span,
    pub alternative: ::core::option::Option<PreprocElifAlternative<'tree>>,
    pub condition: PreprocElifCondition<'tree>,
    pub children: ::std::vec::Vec<PreprocElifChildren<'tree>>,
}
impl<'tree> ::treesitter_types::FromNode<'tree> for PreprocElif<'tree> {
    #[allow(clippy::match_single_binding, clippy::suspicious_else_formatting)]
    fn from_node(
        node: ::treesitter_types::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        debug_assert_eq!(node.kind(), "preproc_elif");
        Ok(Self {
            span: ::treesitter_types::Span::from(node),
            alternative: match node.child_by_field_name("alternative") {
                Some(child) => Some(::treesitter_types::runtime::maybe_grow_stack(|| {
                    <PreprocElifAlternative as ::treesitter_types::FromNode>::from_node(child, src)
                })?),
                None => None,
            },
            condition: {
                let child = node.child_by_field_name("condition").ok_or_else(|| {
                    ::treesitter_types::ParseError::missing_field("condition", node)
                })?;
                ::treesitter_types::runtime::maybe_grow_stack(|| {
                    <PreprocElifCondition as ::treesitter_types::FromNode>::from_node(child, src)
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
                    items.push(::treesitter_types::runtime::maybe_grow_stack(|| {
                        <PreprocElifChildren as ::treesitter_types::FromNode>::from_node(child, src)
                    })?);
                }
                items
            },
        })
    }
}
impl ::treesitter_types::Spanned for PreprocElif<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        self.span
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PreprocElifdef<'tree> {
    pub span: ::treesitter_types::Span,
    pub alternative: ::core::option::Option<PreprocElifdefAlternative<'tree>>,
    pub name: Identifier<'tree>,
    pub children: ::std::vec::Vec<PreprocElifdefChildren<'tree>>,
}
impl<'tree> ::treesitter_types::FromNode<'tree> for PreprocElifdef<'tree> {
    #[allow(clippy::match_single_binding, clippy::suspicious_else_formatting)]
    fn from_node(
        node: ::treesitter_types::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        debug_assert_eq!(node.kind(), "preproc_elifdef");
        Ok(Self {
            span: ::treesitter_types::Span::from(node),
            alternative: match node.child_by_field_name("alternative") {
                Some(child) => Some(::treesitter_types::runtime::maybe_grow_stack(|| {
                    <PreprocElifdefAlternative as ::treesitter_types::FromNode>::from_node(
                        child, src,
                    )
                })?),
                None => None,
            },
            name: {
                let child = node
                    .child_by_field_name("name")
                    .ok_or_else(|| ::treesitter_types::ParseError::missing_field("name", node))?;
                ::treesitter_types::runtime::maybe_grow_stack(|| {
                    <Identifier as ::treesitter_types::FromNode>::from_node(child, src)
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
                    items.push(::treesitter_types::runtime::maybe_grow_stack(|| {
                        <PreprocElifdefChildren as ::treesitter_types::FromNode>::from_node(
                            child, src,
                        )
                    })?);
                }
                items
            },
        })
    }
}
impl ::treesitter_types::Spanned for PreprocElifdef<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        self.span
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PreprocElse<'tree> {
    pub span: ::treesitter_types::Span,
    pub children: ::std::vec::Vec<PreprocElseChildren<'tree>>,
}
impl<'tree> ::treesitter_types::FromNode<'tree> for PreprocElse<'tree> {
    #[allow(clippy::match_single_binding, clippy::suspicious_else_formatting)]
    fn from_node(
        node: ::treesitter_types::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        debug_assert_eq!(node.kind(), "preproc_else");
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
                        <PreprocElseChildren as ::treesitter_types::FromNode>::from_node(child, src)
                    })?);
                }
                items
            },
        })
    }
}
impl ::treesitter_types::Spanned for PreprocElse<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        self.span
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PreprocFunctionDef<'tree> {
    pub span: ::treesitter_types::Span,
    pub name: Identifier<'tree>,
    pub parameters: PreprocParams<'tree>,
    pub value: ::core::option::Option<PreprocArg<'tree>>,
}
impl<'tree> ::treesitter_types::FromNode<'tree> for PreprocFunctionDef<'tree> {
    #[allow(clippy::match_single_binding, clippy::suspicious_else_formatting)]
    fn from_node(
        node: ::treesitter_types::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        debug_assert_eq!(node.kind(), "preproc_function_def");
        Ok(Self {
            span: ::treesitter_types::Span::from(node),
            name: {
                let child = node
                    .child_by_field_name("name")
                    .ok_or_else(|| ::treesitter_types::ParseError::missing_field("name", node))?;
                ::treesitter_types::runtime::maybe_grow_stack(|| {
                    <Identifier as ::treesitter_types::FromNode>::from_node(child, src)
                })?
            },
            parameters: {
                let child = node.child_by_field_name("parameters").ok_or_else(|| {
                    ::treesitter_types::ParseError::missing_field("parameters", node)
                })?;
                ::treesitter_types::runtime::maybe_grow_stack(|| {
                    <PreprocParams as ::treesitter_types::FromNode>::from_node(child, src)
                })?
            },
            value: match node.child_by_field_name("value") {
                Some(child) => Some(::treesitter_types::runtime::maybe_grow_stack(|| {
                    <PreprocArg as ::treesitter_types::FromNode>::from_node(child, src)
                })?),
                None => None,
            },
        })
    }
}
impl ::treesitter_types::Spanned for PreprocFunctionDef<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        self.span
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PreprocIf<'tree> {
    pub span: ::treesitter_types::Span,
    pub alternative: ::core::option::Option<PreprocIfAlternative<'tree>>,
    pub condition: PreprocIfCondition<'tree>,
    pub children: ::std::vec::Vec<PreprocIfChildren<'tree>>,
}
impl<'tree> ::treesitter_types::FromNode<'tree> for PreprocIf<'tree> {
    #[allow(clippy::match_single_binding, clippy::suspicious_else_formatting)]
    fn from_node(
        node: ::treesitter_types::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        debug_assert_eq!(node.kind(), "preproc_if");
        Ok(Self {
            span: ::treesitter_types::Span::from(node),
            alternative: match node.child_by_field_name("alternative") {
                Some(child) => Some(::treesitter_types::runtime::maybe_grow_stack(|| {
                    <PreprocIfAlternative as ::treesitter_types::FromNode>::from_node(child, src)
                })?),
                None => None,
            },
            condition: {
                let child = node.child_by_field_name("condition").ok_or_else(|| {
                    ::treesitter_types::ParseError::missing_field("condition", node)
                })?;
                ::treesitter_types::runtime::maybe_grow_stack(|| {
                    <PreprocIfCondition as ::treesitter_types::FromNode>::from_node(child, src)
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
                    items.push(::treesitter_types::runtime::maybe_grow_stack(|| {
                        <PreprocIfChildren as ::treesitter_types::FromNode>::from_node(child, src)
                    })?);
                }
                items
            },
        })
    }
}
impl ::treesitter_types::Spanned for PreprocIf<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        self.span
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PreprocIfdef<'tree> {
    pub span: ::treesitter_types::Span,
    pub alternative: ::core::option::Option<PreprocIfdefAlternative<'tree>>,
    pub name: Identifier<'tree>,
    pub children: ::std::vec::Vec<PreprocIfdefChildren<'tree>>,
}
impl<'tree> ::treesitter_types::FromNode<'tree> for PreprocIfdef<'tree> {
    #[allow(clippy::match_single_binding, clippy::suspicious_else_formatting)]
    fn from_node(
        node: ::treesitter_types::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        debug_assert_eq!(node.kind(), "preproc_ifdef");
        Ok(Self {
            span: ::treesitter_types::Span::from(node),
            alternative: match node.child_by_field_name("alternative") {
                Some(child) => Some(::treesitter_types::runtime::maybe_grow_stack(|| {
                    <PreprocIfdefAlternative as ::treesitter_types::FromNode>::from_node(child, src)
                })?),
                None => None,
            },
            name: {
                let child = node
                    .child_by_field_name("name")
                    .ok_or_else(|| ::treesitter_types::ParseError::missing_field("name", node))?;
                ::treesitter_types::runtime::maybe_grow_stack(|| {
                    <Identifier as ::treesitter_types::FromNode>::from_node(child, src)
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
                    items.push(::treesitter_types::runtime::maybe_grow_stack(|| {
                        <PreprocIfdefChildren as ::treesitter_types::FromNode>::from_node(
                            child, src,
                        )
                    })?);
                }
                items
            },
        })
    }
}
impl ::treesitter_types::Spanned for PreprocIfdef<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        self.span
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PreprocInclude<'tree> {
    pub span: ::treesitter_types::Span,
    pub path: PreprocIncludePath<'tree>,
}
impl<'tree> ::treesitter_types::FromNode<'tree> for PreprocInclude<'tree> {
    #[allow(clippy::match_single_binding, clippy::suspicious_else_formatting)]
    fn from_node(
        node: ::treesitter_types::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        debug_assert_eq!(node.kind(), "preproc_include");
        Ok(Self {
            span: ::treesitter_types::Span::from(node),
            path: {
                let child = node
                    .child_by_field_name("path")
                    .ok_or_else(|| ::treesitter_types::ParseError::missing_field("path", node))?;
                ::treesitter_types::runtime::maybe_grow_stack(|| {
                    <PreprocIncludePath as ::treesitter_types::FromNode>::from_node(child, src)
                })?
            },
        })
    }
}
impl ::treesitter_types::Spanned for PreprocInclude<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        self.span
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PreprocParams<'tree> {
    pub span: ::treesitter_types::Span,
    pub children: ::std::vec::Vec<Identifier<'tree>>,
}
impl<'tree> ::treesitter_types::FromNode<'tree> for PreprocParams<'tree> {
    #[allow(clippy::match_single_binding, clippy::suspicious_else_formatting)]
    fn from_node(
        node: ::treesitter_types::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        debug_assert_eq!(node.kind(), "preproc_params");
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
                        <Identifier as ::treesitter_types::FromNode>::from_node(child, src)
                    })?);
                }
                items
            },
        })
    }
}
impl ::treesitter_types::Spanned for PreprocParams<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        self.span
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ReturnStatement<'tree> {
    pub span: ::treesitter_types::Span,
    pub children: ::core::option::Option<ReturnStatementChildren<'tree>>,
}
impl<'tree> ::treesitter_types::FromNode<'tree> for ReturnStatement<'tree> {
    #[allow(clippy::match_single_binding, clippy::suspicious_else_formatting)]
    fn from_node(
        node: ::treesitter_types::tree_sitter::Node<'tree>,
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
                    Some(&child) => Some(::treesitter_types::runtime::maybe_grow_stack(|| {
                        <ReturnStatementChildren as ::treesitter_types::FromNode>::from_node(
                            child, src,
                        )
                    })?),
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
pub struct SehExceptClause<'tree> {
    pub span: ::treesitter_types::Span,
    pub body: CompoundStatement<'tree>,
    pub filter: ParenthesizedExpression<'tree>,
}
impl<'tree> ::treesitter_types::FromNode<'tree> for SehExceptClause<'tree> {
    #[allow(clippy::match_single_binding, clippy::suspicious_else_formatting)]
    fn from_node(
        node: ::treesitter_types::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        debug_assert_eq!(node.kind(), "seh_except_clause");
        Ok(Self {
            span: ::treesitter_types::Span::from(node),
            body: {
                let child = node
                    .child_by_field_name("body")
                    .ok_or_else(|| ::treesitter_types::ParseError::missing_field("body", node))?;
                ::treesitter_types::runtime::maybe_grow_stack(|| {
                    <CompoundStatement as ::treesitter_types::FromNode>::from_node(child, src)
                })?
            },
            filter: {
                let child = node
                    .child_by_field_name("filter")
                    .ok_or_else(|| ::treesitter_types::ParseError::missing_field("filter", node))?;
                ::treesitter_types::runtime::maybe_grow_stack(|| {
                    <ParenthesizedExpression as ::treesitter_types::FromNode>::from_node(child, src)
                })?
            },
        })
    }
}
impl ::treesitter_types::Spanned for SehExceptClause<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        self.span
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SehFinallyClause<'tree> {
    pub span: ::treesitter_types::Span,
    pub body: CompoundStatement<'tree>,
}
impl<'tree> ::treesitter_types::FromNode<'tree> for SehFinallyClause<'tree> {
    #[allow(clippy::match_single_binding, clippy::suspicious_else_formatting)]
    fn from_node(
        node: ::treesitter_types::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        debug_assert_eq!(node.kind(), "seh_finally_clause");
        Ok(Self {
            span: ::treesitter_types::Span::from(node),
            body: {
                let child = node
                    .child_by_field_name("body")
                    .ok_or_else(|| ::treesitter_types::ParseError::missing_field("body", node))?;
                ::treesitter_types::runtime::maybe_grow_stack(|| {
                    <CompoundStatement as ::treesitter_types::FromNode>::from_node(child, src)
                })?
            },
        })
    }
}
impl ::treesitter_types::Spanned for SehFinallyClause<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        self.span
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SehLeaveStatement<'tree> {
    pub span: ::treesitter_types::Span,
    text: &'tree str,
}
impl<'tree> ::treesitter_types::FromNode<'tree> for SehLeaveStatement<'tree> {
    fn from_node(
        node: ::treesitter_types::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        debug_assert_eq!(node.kind(), "seh_leave_statement");
        Ok(Self {
            span: ::treesitter_types::Span::from(node),
            text: node.utf8_text(src)?,
        })
    }
}
impl<'tree> ::treesitter_types::LeafNode<'tree> for SehLeaveStatement<'tree> {
    fn text(&self) -> &'tree str {
        self.text
    }
}
impl ::treesitter_types::Spanned for SehLeaveStatement<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        self.span
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SehTryStatement<'tree> {
    pub span: ::treesitter_types::Span,
    pub body: CompoundStatement<'tree>,
    pub children: SehTryStatementChildren<'tree>,
}
impl<'tree> ::treesitter_types::FromNode<'tree> for SehTryStatement<'tree> {
    #[allow(clippy::match_single_binding, clippy::suspicious_else_formatting)]
    fn from_node(
        node: ::treesitter_types::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        debug_assert_eq!(node.kind(), "seh_try_statement");
        Ok(Self {
            span: ::treesitter_types::Span::from(node),
            body: {
                let child = node
                    .child_by_field_name("body")
                    .ok_or_else(|| ::treesitter_types::ParseError::missing_field("body", node))?;
                ::treesitter_types::runtime::maybe_grow_stack(|| {
                    <CompoundStatement as ::treesitter_types::FromNode>::from_node(child, src)
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
                                        ::treesitter_types::runtime::maybe_grow_stack(|| <SehTryStatementChildren as ::treesitter_types::FromNode>::from_node(
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
                                            ::treesitter_types::runtime::maybe_grow_stack(|| <SehTryStatementChildren as ::treesitter_types::FromNode>::from_node(
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
                    <SehTryStatementChildren as ::treesitter_types::FromNode>::from_node(child, src)
                })?
            },
        })
    }
}
impl ::treesitter_types::Spanned for SehTryStatement<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        self.span
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SizedTypeSpecifier<'tree> {
    pub span: ::treesitter_types::Span,
    pub r#type: ::core::option::Option<SizedTypeSpecifierType<'tree>>,
    pub children: ::std::vec::Vec<TypeQualifier<'tree>>,
}
impl<'tree> ::treesitter_types::FromNode<'tree> for SizedTypeSpecifier<'tree> {
    #[allow(clippy::match_single_binding, clippy::suspicious_else_formatting)]
    fn from_node(
        node: ::treesitter_types::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        debug_assert_eq!(node.kind(), "sized_type_specifier");
        Ok(Self {
            span: ::treesitter_types::Span::from(node),
            r#type: match node.child_by_field_name("type") {
                Some(child) => Some(::treesitter_types::runtime::maybe_grow_stack(|| {
                    <SizedTypeSpecifierType as ::treesitter_types::FromNode>::from_node(child, src)
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
                    items.push(::treesitter_types::runtime::maybe_grow_stack(|| {
                        <TypeQualifier as ::treesitter_types::FromNode>::from_node(child, src)
                    })?);
                }
                items
            },
        })
    }
}
impl ::treesitter_types::Spanned for SizedTypeSpecifier<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        self.span
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SizeofExpression<'tree> {
    pub span: ::treesitter_types::Span,
    pub r#type: ::core::option::Option<TypeDescriptor<'tree>>,
    pub value: ::core::option::Option<Expression<'tree>>,
}
impl<'tree> ::treesitter_types::FromNode<'tree> for SizeofExpression<'tree> {
    #[allow(clippy::match_single_binding, clippy::suspicious_else_formatting)]
    fn from_node(
        node: ::treesitter_types::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        debug_assert_eq!(node.kind(), "sizeof_expression");
        Ok(Self {
            span: ::treesitter_types::Span::from(node),
            r#type: match node.child_by_field_name("type") {
                Some(child) => Some(::treesitter_types::runtime::maybe_grow_stack(|| {
                    <TypeDescriptor as ::treesitter_types::FromNode>::from_node(child, src)
                })?),
                None => None,
            },
            value: match node.child_by_field_name("value") {
                Some(child) => Some(::treesitter_types::runtime::maybe_grow_stack(|| {
                    <Expression as ::treesitter_types::FromNode>::from_node(child, src)
                })?),
                None => None,
            },
        })
    }
}
impl ::treesitter_types::Spanned for SizeofExpression<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        self.span
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct StorageClassSpecifier<'tree> {
    pub span: ::treesitter_types::Span,
    text: &'tree str,
}
impl<'tree> ::treesitter_types::FromNode<'tree> for StorageClassSpecifier<'tree> {
    fn from_node(
        node: ::treesitter_types::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        debug_assert_eq!(node.kind(), "storage_class_specifier");
        Ok(Self {
            span: ::treesitter_types::Span::from(node),
            text: node.utf8_text(src)?,
        })
    }
}
impl<'tree> ::treesitter_types::LeafNode<'tree> for StorageClassSpecifier<'tree> {
    fn text(&self) -> &'tree str {
        self.text
    }
}
impl ::treesitter_types::Spanned for StorageClassSpecifier<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        self.span
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct StringLiteral<'tree> {
    pub span: ::treesitter_types::Span,
    pub children: ::std::vec::Vec<StringLiteralChildren<'tree>>,
}
impl<'tree> ::treesitter_types::FromNode<'tree> for StringLiteral<'tree> {
    #[allow(clippy::match_single_binding, clippy::suspicious_else_formatting)]
    fn from_node(
        node: ::treesitter_types::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        debug_assert_eq!(node.kind(), "string_literal");
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
                        <StringLiteralChildren as ::treesitter_types::FromNode>::from_node(
                            child, src,
                        )
                    })?);
                }
                items
            },
        })
    }
}
impl ::treesitter_types::Spanned for StringLiteral<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        self.span
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct StructSpecifier<'tree> {
    pub span: ::treesitter_types::Span,
    pub body: ::core::option::Option<FieldDeclarationList<'tree>>,
    pub name: ::core::option::Option<TypeIdentifier<'tree>>,
    pub children: ::std::vec::Vec<StructSpecifierChildren<'tree>>,
}
impl<'tree> ::treesitter_types::FromNode<'tree> for StructSpecifier<'tree> {
    #[allow(clippy::match_single_binding, clippy::suspicious_else_formatting)]
    fn from_node(
        node: ::treesitter_types::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        debug_assert_eq!(node.kind(), "struct_specifier");
        Ok(Self {
            span: ::treesitter_types::Span::from(node),
            body: match node.child_by_field_name("body") {
                Some(child) => Some(::treesitter_types::runtime::maybe_grow_stack(|| {
                    <FieldDeclarationList as ::treesitter_types::FromNode>::from_node(child, src)
                })?),
                None => None,
            },
            name: match node.child_by_field_name("name") {
                Some(child) => Some(::treesitter_types::runtime::maybe_grow_stack(|| {
                    <TypeIdentifier as ::treesitter_types::FromNode>::from_node(child, src)
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
                    items.push(::treesitter_types::runtime::maybe_grow_stack(|| {
                        <StructSpecifierChildren as ::treesitter_types::FromNode>::from_node(
                            child, src,
                        )
                    })?);
                }
                items
            },
        })
    }
}
impl ::treesitter_types::Spanned for StructSpecifier<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        self.span
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SubscriptDesignator<'tree> {
    pub span: ::treesitter_types::Span,
    pub children: Expression<'tree>,
}
impl<'tree> ::treesitter_types::FromNode<'tree> for SubscriptDesignator<'tree> {
    #[allow(clippy::match_single_binding, clippy::suspicious_else_formatting)]
    fn from_node(
        node: ::treesitter_types::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        debug_assert_eq!(node.kind(), "subscript_designator");
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
                                        ::treesitter_types::runtime::maybe_grow_stack(|| <Expression as ::treesitter_types::FromNode>::from_node(
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
                                            ::treesitter_types::runtime::maybe_grow_stack(|| <Expression as ::treesitter_types::FromNode>::from_node(
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
                    <Expression as ::treesitter_types::FromNode>::from_node(child, src)
                })?
            },
        })
    }
}
impl ::treesitter_types::Spanned for SubscriptDesignator<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        self.span
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SubscriptExpression<'tree> {
    pub span: ::treesitter_types::Span,
    pub argument: Expression<'tree>,
    pub index: Expression<'tree>,
}
impl<'tree> ::treesitter_types::FromNode<'tree> for SubscriptExpression<'tree> {
    #[allow(clippy::match_single_binding, clippy::suspicious_else_formatting)]
    fn from_node(
        node: ::treesitter_types::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        debug_assert_eq!(node.kind(), "subscript_expression");
        Ok(Self {
            span: ::treesitter_types::Span::from(node),
            argument: {
                let child = node.child_by_field_name("argument").ok_or_else(|| {
                    ::treesitter_types::ParseError::missing_field("argument", node)
                })?;
                ::treesitter_types::runtime::maybe_grow_stack(|| {
                    <Expression as ::treesitter_types::FromNode>::from_node(child, src)
                })?
            },
            index: {
                let child = node
                    .child_by_field_name("index")
                    .ok_or_else(|| ::treesitter_types::ParseError::missing_field("index", node))?;
                ::treesitter_types::runtime::maybe_grow_stack(|| {
                    <Expression as ::treesitter_types::FromNode>::from_node(child, src)
                })?
            },
        })
    }
}
impl ::treesitter_types::Spanned for SubscriptExpression<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        self.span
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SubscriptRangeDesignator<'tree> {
    pub span: ::treesitter_types::Span,
    pub end: Expression<'tree>,
    pub start: Expression<'tree>,
}
impl<'tree> ::treesitter_types::FromNode<'tree> for SubscriptRangeDesignator<'tree> {
    #[allow(clippy::match_single_binding, clippy::suspicious_else_formatting)]
    fn from_node(
        node: ::treesitter_types::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        debug_assert_eq!(node.kind(), "subscript_range_designator");
        Ok(Self {
            span: ::treesitter_types::Span::from(node),
            end: {
                let child = node
                    .child_by_field_name("end")
                    .ok_or_else(|| ::treesitter_types::ParseError::missing_field("end", node))?;
                ::treesitter_types::runtime::maybe_grow_stack(|| {
                    <Expression as ::treesitter_types::FromNode>::from_node(child, src)
                })?
            },
            start: {
                let child = node
                    .child_by_field_name("start")
                    .ok_or_else(|| ::treesitter_types::ParseError::missing_field("start", node))?;
                ::treesitter_types::runtime::maybe_grow_stack(|| {
                    <Expression as ::treesitter_types::FromNode>::from_node(child, src)
                })?
            },
        })
    }
}
impl ::treesitter_types::Spanned for SubscriptRangeDesignator<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        self.span
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SwitchStatement<'tree> {
    pub span: ::treesitter_types::Span,
    pub body: CompoundStatement<'tree>,
    pub condition: ParenthesizedExpression<'tree>,
}
impl<'tree> ::treesitter_types::FromNode<'tree> for SwitchStatement<'tree> {
    #[allow(clippy::match_single_binding, clippy::suspicious_else_formatting)]
    fn from_node(
        node: ::treesitter_types::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        debug_assert_eq!(node.kind(), "switch_statement");
        Ok(Self {
            span: ::treesitter_types::Span::from(node),
            body: {
                let child = node
                    .child_by_field_name("body")
                    .ok_or_else(|| ::treesitter_types::ParseError::missing_field("body", node))?;
                ::treesitter_types::runtime::maybe_grow_stack(|| {
                    <CompoundStatement as ::treesitter_types::FromNode>::from_node(child, src)
                })?
            },
            condition: {
                let child = node.child_by_field_name("condition").ok_or_else(|| {
                    ::treesitter_types::ParseError::missing_field("condition", node)
                })?;
                ::treesitter_types::runtime::maybe_grow_stack(|| {
                    <ParenthesizedExpression as ::treesitter_types::FromNode>::from_node(child, src)
                })?
            },
        })
    }
}
impl ::treesitter_types::Spanned for SwitchStatement<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        self.span
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct TranslationUnit<'tree> {
    pub span: ::treesitter_types::Span,
    pub children: ::std::vec::Vec<TranslationUnitChildren<'tree>>,
}
impl<'tree> ::treesitter_types::FromNode<'tree> for TranslationUnit<'tree> {
    #[allow(clippy::match_single_binding, clippy::suspicious_else_formatting)]
    fn from_node(
        node: ::treesitter_types::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        debug_assert_eq!(node.kind(), "translation_unit");
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
                        <TranslationUnitChildren as ::treesitter_types::FromNode>::from_node(
                            child, src,
                        )
                    })?);
                }
                items
            },
        })
    }
}
impl ::treesitter_types::Spanned for TranslationUnit<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        self.span
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct TypeDefinition<'tree> {
    pub span: ::treesitter_types::Span,
    pub declarator: ::std::vec::Vec<TypeDeclarator<'tree>>,
    pub r#type: TypeSpecifier<'tree>,
    pub children: ::std::vec::Vec<TypeDefinitionChildren<'tree>>,
}
impl<'tree> ::treesitter_types::FromNode<'tree> for TypeDefinition<'tree> {
    #[allow(clippy::match_single_binding, clippy::suspicious_else_formatting)]
    fn from_node(
        node: ::treesitter_types::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        debug_assert_eq!(node.kind(), "type_definition");
        Ok(Self {
            span: ::treesitter_types::Span::from(node),
            declarator: {
                let mut cursor = node.walk();
                let mut items = ::std::vec::Vec::new();
                for child in node.children_by_field_name("declarator", &mut cursor) {
                    items.push(::treesitter_types::runtime::maybe_grow_stack(|| {
                        <TypeDeclarator as ::treesitter_types::FromNode>::from_node(child, src)
                    })?);
                }
                items
            },
            r#type: {
                let child = node
                    .child_by_field_name("type")
                    .ok_or_else(|| ::treesitter_types::ParseError::missing_field("type", node))?;
                ::treesitter_types::runtime::maybe_grow_stack(|| {
                    <TypeSpecifier as ::treesitter_types::FromNode>::from_node(child, src)
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
                    items.push(::treesitter_types::runtime::maybe_grow_stack(|| {
                        <TypeDefinitionChildren as ::treesitter_types::FromNode>::from_node(
                            child, src,
                        )
                    })?);
                }
                items
            },
        })
    }
}
impl ::treesitter_types::Spanned for TypeDefinition<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        self.span
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct TypeDescriptor<'tree> {
    pub span: ::treesitter_types::Span,
    pub declarator: ::core::option::Option<AbstractDeclarator<'tree>>,
    pub r#type: TypeSpecifier<'tree>,
    pub children: ::std::vec::Vec<TypeQualifier<'tree>>,
}
impl<'tree> ::treesitter_types::FromNode<'tree> for TypeDescriptor<'tree> {
    #[allow(clippy::match_single_binding, clippy::suspicious_else_formatting)]
    fn from_node(
        node: ::treesitter_types::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        debug_assert_eq!(node.kind(), "type_descriptor");
        Ok(Self {
            span: ::treesitter_types::Span::from(node),
            declarator: match node.child_by_field_name("declarator") {
                Some(child) => Some(::treesitter_types::runtime::maybe_grow_stack(|| {
                    <AbstractDeclarator as ::treesitter_types::FromNode>::from_node(child, src)
                })?),
                None => None,
            },
            r#type: {
                let child = node
                    .child_by_field_name("type")
                    .ok_or_else(|| ::treesitter_types::ParseError::missing_field("type", node))?;
                ::treesitter_types::runtime::maybe_grow_stack(|| {
                    <TypeSpecifier as ::treesitter_types::FromNode>::from_node(child, src)
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
                    items.push(::treesitter_types::runtime::maybe_grow_stack(|| {
                        <TypeQualifier as ::treesitter_types::FromNode>::from_node(child, src)
                    })?);
                }
                items
            },
        })
    }
}
impl ::treesitter_types::Spanned for TypeDescriptor<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        self.span
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct TypeQualifier<'tree> {
    pub span: ::treesitter_types::Span,
    pub children: ::core::option::Option<AlignasQualifier<'tree>>,
}
impl<'tree> ::treesitter_types::FromNode<'tree> for TypeQualifier<'tree> {
    #[allow(clippy::match_single_binding, clippy::suspicious_else_formatting)]
    fn from_node(
        node: ::treesitter_types::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        debug_assert_eq!(node.kind(), "type_qualifier");
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
                        <AlignasQualifier as ::treesitter_types::FromNode>::from_node(child, src)
                    })?),
                    None => None,
                }
            },
        })
    }
}
impl ::treesitter_types::Spanned for TypeQualifier<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        self.span
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct UnaryExpression<'tree> {
    pub span: ::treesitter_types::Span,
    pub argument: UnaryExpressionArgument<'tree>,
    pub operator: UnaryExpressionOperator,
}
impl<'tree> ::treesitter_types::FromNode<'tree> for UnaryExpression<'tree> {
    #[allow(clippy::match_single_binding, clippy::suspicious_else_formatting)]
    fn from_node(
        node: ::treesitter_types::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        debug_assert_eq!(node.kind(), "unary_expression");
        Ok(Self {
            span: ::treesitter_types::Span::from(node),
            argument: {
                let child = node.child_by_field_name("argument").ok_or_else(|| {
                    ::treesitter_types::ParseError::missing_field("argument", node)
                })?;
                ::treesitter_types::runtime::maybe_grow_stack(|| {
                    <UnaryExpressionArgument as ::treesitter_types::FromNode>::from_node(child, src)
                })?
            },
            operator: {
                let child = node.child_by_field_name("operator").ok_or_else(|| {
                    ::treesitter_types::ParseError::missing_field("operator", node)
                })?;
                ::treesitter_types::runtime::maybe_grow_stack(|| {
                    <UnaryExpressionOperator as ::treesitter_types::FromNode>::from_node(child, src)
                })?
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
pub struct UnionSpecifier<'tree> {
    pub span: ::treesitter_types::Span,
    pub body: ::core::option::Option<FieldDeclarationList<'tree>>,
    pub name: ::core::option::Option<TypeIdentifier<'tree>>,
    pub children: ::std::vec::Vec<UnionSpecifierChildren<'tree>>,
}
impl<'tree> ::treesitter_types::FromNode<'tree> for UnionSpecifier<'tree> {
    #[allow(clippy::match_single_binding, clippy::suspicious_else_formatting)]
    fn from_node(
        node: ::treesitter_types::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        debug_assert_eq!(node.kind(), "union_specifier");
        Ok(Self {
            span: ::treesitter_types::Span::from(node),
            body: match node.child_by_field_name("body") {
                Some(child) => Some(::treesitter_types::runtime::maybe_grow_stack(|| {
                    <FieldDeclarationList as ::treesitter_types::FromNode>::from_node(child, src)
                })?),
                None => None,
            },
            name: match node.child_by_field_name("name") {
                Some(child) => Some(::treesitter_types::runtime::maybe_grow_stack(|| {
                    <TypeIdentifier as ::treesitter_types::FromNode>::from_node(child, src)
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
                    items.push(::treesitter_types::runtime::maybe_grow_stack(|| {
                        <UnionSpecifierChildren as ::treesitter_types::FromNode>::from_node(
                            child, src,
                        )
                    })?);
                }
                items
            },
        })
    }
}
impl ::treesitter_types::Spanned for UnionSpecifier<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        self.span
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct UpdateExpression<'tree> {
    pub span: ::treesitter_types::Span,
    pub argument: Expression<'tree>,
    pub operator: UpdateExpressionOperator,
}
impl<'tree> ::treesitter_types::FromNode<'tree> for UpdateExpression<'tree> {
    #[allow(clippy::match_single_binding, clippy::suspicious_else_formatting)]
    fn from_node(
        node: ::treesitter_types::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        debug_assert_eq!(node.kind(), "update_expression");
        Ok(Self {
            span: ::treesitter_types::Span::from(node),
            argument: {
                let child = node.child_by_field_name("argument").ok_or_else(|| {
                    ::treesitter_types::ParseError::missing_field("argument", node)
                })?;
                ::treesitter_types::runtime::maybe_grow_stack(|| {
                    <Expression as ::treesitter_types::FromNode>::from_node(child, src)
                })?
            },
            operator: {
                let child = node.child_by_field_name("operator").ok_or_else(|| {
                    ::treesitter_types::ParseError::missing_field("operator", node)
                })?;
                ::treesitter_types::runtime::maybe_grow_stack(|| {
                    <UpdateExpressionOperator as ::treesitter_types::FromNode>::from_node(
                        child, src,
                    )
                })?
            },
        })
    }
}
impl ::treesitter_types::Spanned for UpdateExpression<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        self.span
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct VariadicParameter<'tree> {
    pub span: ::treesitter_types::Span,
    text: &'tree str,
}
impl<'tree> ::treesitter_types::FromNode<'tree> for VariadicParameter<'tree> {
    fn from_node(
        node: ::treesitter_types::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        debug_assert_eq!(node.kind(), "variadic_parameter");
        Ok(Self {
            span: ::treesitter_types::Span::from(node),
            text: node.utf8_text(src)?,
        })
    }
}
impl<'tree> ::treesitter_types::LeafNode<'tree> for VariadicParameter<'tree> {
    fn text(&self) -> &'tree str {
        self.text
    }
}
impl ::treesitter_types::Spanned for VariadicParameter<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        self.span
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct WhileStatement<'tree> {
    pub span: ::treesitter_types::Span,
    pub body: Statement<'tree>,
    pub condition: ParenthesizedExpression<'tree>,
}
impl<'tree> ::treesitter_types::FromNode<'tree> for WhileStatement<'tree> {
    #[allow(clippy::match_single_binding, clippy::suspicious_else_formatting)]
    fn from_node(
        node: ::treesitter_types::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        debug_assert_eq!(node.kind(), "while_statement");
        Ok(Self {
            span: ::treesitter_types::Span::from(node),
            body: {
                let child = node
                    .child_by_field_name("body")
                    .ok_or_else(|| ::treesitter_types::ParseError::missing_field("body", node))?;
                ::treesitter_types::runtime::maybe_grow_stack(|| {
                    <Statement as ::treesitter_types::FromNode>::from_node(child, src)
                })?
            },
            condition: {
                let child = node.child_by_field_name("condition").ok_or_else(|| {
                    ::treesitter_types::ParseError::missing_field("condition", node)
                })?;
                ::treesitter_types::runtime::maybe_grow_stack(|| {
                    <ParenthesizedExpression as ::treesitter_types::FromNode>::from_node(child, src)
                })?
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
pub struct Character<'tree> {
    pub span: ::treesitter_types::Span,
    text: &'tree str,
}
impl<'tree> ::treesitter_types::FromNode<'tree> for Character<'tree> {
    fn from_node(
        node: ::treesitter_types::tree_sitter::Node<'tree>,
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
pub struct Comment<'tree> {
    pub span: ::treesitter_types::Span,
    text: &'tree str,
}
impl<'tree> ::treesitter_types::FromNode<'tree> for Comment<'tree> {
    fn from_node(
        node: ::treesitter_types::tree_sitter::Node<'tree>,
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
        node: ::treesitter_types::tree_sitter::Node<'tree>,
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
        node: ::treesitter_types::tree_sitter::Node<'tree>,
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
pub struct FieldIdentifier<'tree> {
    pub span: ::treesitter_types::Span,
    text: &'tree str,
}
impl<'tree> ::treesitter_types::FromNode<'tree> for FieldIdentifier<'tree> {
    fn from_node(
        node: ::treesitter_types::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        debug_assert_eq!(node.kind(), "field_identifier");
        Ok(Self {
            span: ::treesitter_types::Span::from(node),
            text: node.utf8_text(src)?,
        })
    }
}
impl<'tree> ::treesitter_types::LeafNode<'tree> for FieldIdentifier<'tree> {
    fn text(&self) -> &'tree str {
        self.text
    }
}
impl ::treesitter_types::Spanned for FieldIdentifier<'_> {
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
        node: ::treesitter_types::tree_sitter::Node<'tree>,
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
pub struct MsRestrictModifier<'tree> {
    pub span: ::treesitter_types::Span,
    text: &'tree str,
}
impl<'tree> ::treesitter_types::FromNode<'tree> for MsRestrictModifier<'tree> {
    fn from_node(
        node: ::treesitter_types::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        debug_assert_eq!(node.kind(), "ms_restrict_modifier");
        Ok(Self {
            span: ::treesitter_types::Span::from(node),
            text: node.utf8_text(src)?,
        })
    }
}
impl<'tree> ::treesitter_types::LeafNode<'tree> for MsRestrictModifier<'tree> {
    fn text(&self) -> &'tree str {
        self.text
    }
}
impl ::treesitter_types::Spanned for MsRestrictModifier<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        self.span
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct MsSignedPtrModifier<'tree> {
    pub span: ::treesitter_types::Span,
    text: &'tree str,
}
impl<'tree> ::treesitter_types::FromNode<'tree> for MsSignedPtrModifier<'tree> {
    fn from_node(
        node: ::treesitter_types::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        debug_assert_eq!(node.kind(), "ms_signed_ptr_modifier");
        Ok(Self {
            span: ::treesitter_types::Span::from(node),
            text: node.utf8_text(src)?,
        })
    }
}
impl<'tree> ::treesitter_types::LeafNode<'tree> for MsSignedPtrModifier<'tree> {
    fn text(&self) -> &'tree str {
        self.text
    }
}
impl ::treesitter_types::Spanned for MsSignedPtrModifier<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        self.span
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct MsUnsignedPtrModifier<'tree> {
    pub span: ::treesitter_types::Span,
    text: &'tree str,
}
impl<'tree> ::treesitter_types::FromNode<'tree> for MsUnsignedPtrModifier<'tree> {
    fn from_node(
        node: ::treesitter_types::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        debug_assert_eq!(node.kind(), "ms_unsigned_ptr_modifier");
        Ok(Self {
            span: ::treesitter_types::Span::from(node),
            text: node.utf8_text(src)?,
        })
    }
}
impl<'tree> ::treesitter_types::LeafNode<'tree> for MsUnsignedPtrModifier<'tree> {
    fn text(&self) -> &'tree str {
        self.text
    }
}
impl ::treesitter_types::Spanned for MsUnsignedPtrModifier<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        self.span
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct NumberLiteral<'tree> {
    pub span: ::treesitter_types::Span,
    text: &'tree str,
}
impl<'tree> ::treesitter_types::FromNode<'tree> for NumberLiteral<'tree> {
    fn from_node(
        node: ::treesitter_types::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        debug_assert_eq!(node.kind(), "number_literal");
        Ok(Self {
            span: ::treesitter_types::Span::from(node),
            text: node.utf8_text(src)?,
        })
    }
}
impl<'tree> ::treesitter_types::LeafNode<'tree> for NumberLiteral<'tree> {
    fn text(&self) -> &'tree str {
        self.text
    }
}
impl ::treesitter_types::Spanned for NumberLiteral<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        self.span
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PreprocArg<'tree> {
    pub span: ::treesitter_types::Span,
    text: &'tree str,
}
impl<'tree> ::treesitter_types::FromNode<'tree> for PreprocArg<'tree> {
    fn from_node(
        node: ::treesitter_types::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        debug_assert_eq!(node.kind(), "preproc_arg");
        Ok(Self {
            span: ::treesitter_types::Span::from(node),
            text: node.utf8_text(src)?,
        })
    }
}
impl<'tree> ::treesitter_types::LeafNode<'tree> for PreprocArg<'tree> {
    fn text(&self) -> &'tree str {
        self.text
    }
}
impl ::treesitter_types::Spanned for PreprocArg<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        self.span
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PreprocDirective<'tree> {
    pub span: ::treesitter_types::Span,
    text: &'tree str,
}
impl<'tree> ::treesitter_types::FromNode<'tree> for PreprocDirective<'tree> {
    fn from_node(
        node: ::treesitter_types::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        debug_assert_eq!(node.kind(), "preproc_directive");
        Ok(Self {
            span: ::treesitter_types::Span::from(node),
            text: node.utf8_text(src)?,
        })
    }
}
impl<'tree> ::treesitter_types::LeafNode<'tree> for PreprocDirective<'tree> {
    fn text(&self) -> &'tree str {
        self.text
    }
}
impl ::treesitter_types::Spanned for PreprocDirective<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        self.span
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PrimitiveType<'tree> {
    pub span: ::treesitter_types::Span,
    text: &'tree str,
}
impl<'tree> ::treesitter_types::FromNode<'tree> for PrimitiveType<'tree> {
    fn from_node(
        node: ::treesitter_types::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        debug_assert_eq!(node.kind(), "primitive_type");
        Ok(Self {
            span: ::treesitter_types::Span::from(node),
            text: node.utf8_text(src)?,
        })
    }
}
impl<'tree> ::treesitter_types::LeafNode<'tree> for PrimitiveType<'tree> {
    fn text(&self) -> &'tree str {
        self.text
    }
}
impl ::treesitter_types::Spanned for PrimitiveType<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        self.span
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct StatementIdentifier<'tree> {
    pub span: ::treesitter_types::Span,
    text: &'tree str,
}
impl<'tree> ::treesitter_types::FromNode<'tree> for StatementIdentifier<'tree> {
    fn from_node(
        node: ::treesitter_types::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        debug_assert_eq!(node.kind(), "statement_identifier");
        Ok(Self {
            span: ::treesitter_types::Span::from(node),
            text: node.utf8_text(src)?,
        })
    }
}
impl<'tree> ::treesitter_types::LeafNode<'tree> for StatementIdentifier<'tree> {
    fn text(&self) -> &'tree str {
        self.text
    }
}
impl ::treesitter_types::Spanned for StatementIdentifier<'_> {
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
        node: ::treesitter_types::tree_sitter::Node<'tree>,
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
pub struct SystemLibString<'tree> {
    pub span: ::treesitter_types::Span,
    text: &'tree str,
}
impl<'tree> ::treesitter_types::FromNode<'tree> for SystemLibString<'tree> {
    fn from_node(
        node: ::treesitter_types::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        debug_assert_eq!(node.kind(), "system_lib_string");
        Ok(Self {
            span: ::treesitter_types::Span::from(node),
            text: node.utf8_text(src)?,
        })
    }
}
impl<'tree> ::treesitter_types::LeafNode<'tree> for SystemLibString<'tree> {
    fn text(&self) -> &'tree str {
        self.text
    }
}
impl ::treesitter_types::Spanned for SystemLibString<'_> {
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
        node: ::treesitter_types::tree_sitter::Node<'tree>,
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
pub struct TypeIdentifier<'tree> {
    pub span: ::treesitter_types::Span,
    text: &'tree str,
}
impl<'tree> ::treesitter_types::FromNode<'tree> for TypeIdentifier<'tree> {
    fn from_node(
        node: ::treesitter_types::tree_sitter::Node<'tree>,
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
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum AbstractArrayDeclaratorSize<'tree> {
    Star(::treesitter_types::Span),
    Expression(::std::boxed::Box<Expression<'tree>>),
}
impl<'tree> ::treesitter_types::FromNode<'tree> for AbstractArrayDeclaratorSize<'tree> {
    #[allow(clippy::collapsible_else_if)]
    fn from_node(
        node: ::treesitter_types::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        match node.kind() {
            "*" => Ok(Self::Star(::treesitter_types::Span::from(node))),
            _other => {
                if let Ok(v) = ::treesitter_types::runtime::maybe_grow_stack(|| {
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
impl ::treesitter_types::Spanned for AbstractArrayDeclaratorSize<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        match self {
            Self::Star(span) => *span,
            Self::Expression(inner) => inner.span(),
        }
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum AbstractParenthesizedDeclaratorChildren<'tree> {
    AbstractDeclarator(::std::boxed::Box<AbstractDeclarator<'tree>>),
    MsCallModifier(::std::boxed::Box<MsCallModifier<'tree>>),
}
impl<'tree> ::treesitter_types::FromNode<'tree> for AbstractParenthesizedDeclaratorChildren<'tree> {
    #[allow(clippy::collapsible_else_if)]
    fn from_node(
        node: ::treesitter_types::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        match node.kind() {
            "ms_call_modifier" => Ok(Self::MsCallModifier(::std::boxed::Box::new(
                ::treesitter_types::runtime::maybe_grow_stack(|| {
                    <MsCallModifier as ::treesitter_types::FromNode>::from_node(node, src)
                })?,
            ))),
            _other => {
                if let Ok(v) = ::treesitter_types::runtime::maybe_grow_stack(|| {
                    <AbstractDeclarator as ::treesitter_types::FromNode>::from_node(node, src)
                }) {
                    Ok(Self::AbstractDeclarator(::std::boxed::Box::new(v)))
                } else {
                    Err(::treesitter_types::ParseError::unexpected_kind(
                        _other, node,
                    ))
                }
            }
        }
    }
}
impl ::treesitter_types::Spanned for AbstractParenthesizedDeclaratorChildren<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        match self {
            Self::AbstractDeclarator(inner) => inner.span(),
            Self::MsCallModifier(inner) => inner.span(),
        }
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum AbstractPointerDeclaratorChildren<'tree> {
    MsPointerModifier(::std::boxed::Box<MsPointerModifier<'tree>>),
    TypeQualifier(::std::boxed::Box<TypeQualifier<'tree>>),
}
impl<'tree> ::treesitter_types::FromNode<'tree> for AbstractPointerDeclaratorChildren<'tree> {
    #[allow(clippy::collapsible_else_if)]
    fn from_node(
        node: ::treesitter_types::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        match node.kind() {
            "ms_pointer_modifier" => Ok(Self::MsPointerModifier(::std::boxed::Box::new(
                ::treesitter_types::runtime::maybe_grow_stack(|| {
                    <MsPointerModifier as ::treesitter_types::FromNode>::from_node(node, src)
                })?,
            ))),
            "type_qualifier" => Ok(Self::TypeQualifier(::std::boxed::Box::new(
                ::treesitter_types::runtime::maybe_grow_stack(|| {
                    <TypeQualifier as ::treesitter_types::FromNode>::from_node(node, src)
                })?,
            ))),
            other => Err(::treesitter_types::ParseError::unexpected_kind(other, node)),
        }
    }
}
impl ::treesitter_types::Spanned for AbstractPointerDeclaratorChildren<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        match self {
            Self::MsPointerModifier(inner) => inner.span(),
            Self::TypeQualifier(inner) => inner.span(),
        }
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum AlignasQualifierChildren<'tree> {
    Expression(::std::boxed::Box<Expression<'tree>>),
    TypeDescriptor(::std::boxed::Box<TypeDescriptor<'tree>>),
}
impl<'tree> ::treesitter_types::FromNode<'tree> for AlignasQualifierChildren<'tree> {
    #[allow(clippy::collapsible_else_if)]
    fn from_node(
        node: ::treesitter_types::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        match node.kind() {
            "type_descriptor" => Ok(Self::TypeDescriptor(::std::boxed::Box::new(
                ::treesitter_types::runtime::maybe_grow_stack(|| {
                    <TypeDescriptor as ::treesitter_types::FromNode>::from_node(node, src)
                })?,
            ))),
            _other => {
                if let Ok(v) = ::treesitter_types::runtime::maybe_grow_stack(|| {
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
impl ::treesitter_types::Spanned for AlignasQualifierChildren<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        match self {
            Self::Expression(inner) => inner.span(),
            Self::TypeDescriptor(inner) => inner.span(),
        }
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ArgumentListChildren<'tree> {
    CompoundStatement(::std::boxed::Box<CompoundStatement<'tree>>),
    Expression(::std::boxed::Box<Expression<'tree>>),
    PreprocDefined(::std::boxed::Box<PreprocDefined<'tree>>),
}
impl<'tree> ::treesitter_types::FromNode<'tree> for ArgumentListChildren<'tree> {
    #[allow(clippy::collapsible_else_if)]
    fn from_node(
        node: ::treesitter_types::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        match node.kind() {
            "compound_statement" => Ok(Self::CompoundStatement(::std::boxed::Box::new(
                ::treesitter_types::runtime::maybe_grow_stack(|| {
                    <CompoundStatement as ::treesitter_types::FromNode>::from_node(node, src)
                })?,
            ))),
            "preproc_defined" => Ok(Self::PreprocDefined(::std::boxed::Box::new(
                ::treesitter_types::runtime::maybe_grow_stack(|| {
                    <PreprocDefined as ::treesitter_types::FromNode>::from_node(node, src)
                })?,
            ))),
            _other => {
                if let Ok(v) = ::treesitter_types::runtime::maybe_grow_stack(|| {
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
            Self::CompoundStatement(inner) => inner.span(),
            Self::Expression(inner) => inner.span(),
            Self::PreprocDefined(inner) => inner.span(),
        }
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ArrayDeclaratorDeclarator<'tree> {
    Declarator(::std::boxed::Box<Declarator<'tree>>),
    FieldDeclarator(::std::boxed::Box<FieldDeclarator<'tree>>),
    TypeDeclarator(::std::boxed::Box<TypeDeclarator<'tree>>),
}
impl<'tree> ::treesitter_types::FromNode<'tree> for ArrayDeclaratorDeclarator<'tree> {
    #[allow(clippy::collapsible_else_if)]
    fn from_node(
        node: ::treesitter_types::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        if let Ok(v) = ::treesitter_types::runtime::maybe_grow_stack(|| {
            <Declarator as ::treesitter_types::FromNode>::from_node(node, src)
        }) {
            Ok(Self::Declarator(::std::boxed::Box::new(v)))
        } else {
            if let Ok(v) = ::treesitter_types::runtime::maybe_grow_stack(|| {
                <FieldDeclarator as ::treesitter_types::FromNode>::from_node(node, src)
            }) {
                Ok(Self::FieldDeclarator(::std::boxed::Box::new(v)))
            } else {
                if let Ok(v) = ::treesitter_types::runtime::maybe_grow_stack(|| {
                    <TypeDeclarator as ::treesitter_types::FromNode>::from_node(node, src)
                }) {
                    Ok(Self::TypeDeclarator(::std::boxed::Box::new(v)))
                } else {
                    Err(::treesitter_types::ParseError::unexpected_kind(
                        node.kind(),
                        node,
                    ))
                }
            }
        }
    }
}
impl ::treesitter_types::Spanned for ArrayDeclaratorDeclarator<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        match self {
            Self::Declarator(inner) => inner.span(),
            Self::FieldDeclarator(inner) => inner.span(),
            Self::TypeDeclarator(inner) => inner.span(),
        }
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ArrayDeclaratorSize<'tree> {
    Star(::treesitter_types::Span),
    Expression(::std::boxed::Box<Expression<'tree>>),
}
impl<'tree> ::treesitter_types::FromNode<'tree> for ArrayDeclaratorSize<'tree> {
    #[allow(clippy::collapsible_else_if)]
    fn from_node(
        node: ::treesitter_types::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        match node.kind() {
            "*" => Ok(Self::Star(::treesitter_types::Span::from(node))),
            _other => {
                if let Ok(v) = ::treesitter_types::runtime::maybe_grow_stack(|| {
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
impl ::treesitter_types::Spanned for ArrayDeclaratorSize<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        match self {
            Self::Star(span) => *span,
            Self::Expression(inner) => inner.span(),
        }
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum AssignmentExpressionLeft<'tree> {
    CallExpression(::std::boxed::Box<CallExpression<'tree>>),
    FieldExpression(::std::boxed::Box<FieldExpression<'tree>>),
    Identifier(::std::boxed::Box<Identifier<'tree>>),
    ParenthesizedExpression(::std::boxed::Box<ParenthesizedExpression<'tree>>),
    PointerExpression(::std::boxed::Box<PointerExpression<'tree>>),
    SubscriptExpression(::std::boxed::Box<SubscriptExpression<'tree>>),
}
impl<'tree> ::treesitter_types::FromNode<'tree> for AssignmentExpressionLeft<'tree> {
    #[allow(clippy::collapsible_else_if)]
    fn from_node(
        node: ::treesitter_types::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        match node.kind() {
            "call_expression" => Ok(Self::CallExpression(::std::boxed::Box::new(
                ::treesitter_types::runtime::maybe_grow_stack(|| {
                    <CallExpression as ::treesitter_types::FromNode>::from_node(node, src)
                })?,
            ))),
            "field_expression" => Ok(Self::FieldExpression(::std::boxed::Box::new(
                ::treesitter_types::runtime::maybe_grow_stack(|| {
                    <FieldExpression as ::treesitter_types::FromNode>::from_node(node, src)
                })?,
            ))),
            "identifier" => Ok(Self::Identifier(::std::boxed::Box::new(
                ::treesitter_types::runtime::maybe_grow_stack(|| {
                    <Identifier as ::treesitter_types::FromNode>::from_node(node, src)
                })?,
            ))),
            "parenthesized_expression" => Ok(Self::ParenthesizedExpression(
                ::std::boxed::Box::new(::treesitter_types::runtime::maybe_grow_stack(|| {
                    <ParenthesizedExpression as ::treesitter_types::FromNode>::from_node(node, src)
                })?),
            )),
            "pointer_expression" => Ok(Self::PointerExpression(::std::boxed::Box::new(
                ::treesitter_types::runtime::maybe_grow_stack(|| {
                    <PointerExpression as ::treesitter_types::FromNode>::from_node(node, src)
                })?,
            ))),
            "subscript_expression" => Ok(Self::SubscriptExpression(::std::boxed::Box::new(
                ::treesitter_types::runtime::maybe_grow_stack(|| {
                    <SubscriptExpression as ::treesitter_types::FromNode>::from_node(node, src)
                })?,
            ))),
            other => Err(::treesitter_types::ParseError::unexpected_kind(other, node)),
        }
    }
}
impl ::treesitter_types::Spanned for AssignmentExpressionLeft<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        match self {
            Self::CallExpression(inner) => inner.span(),
            Self::FieldExpression(inner) => inner.span(),
            Self::Identifier(inner) => inner.span(),
            Self::ParenthesizedExpression(inner) => inner.span(),
            Self::PointerExpression(inner) => inner.span(),
            Self::SubscriptExpression(inner) => inner.span(),
        }
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum AssignmentExpressionOperator {
    PercentEq(::treesitter_types::Span),
    AmpEq(::treesitter_types::Span),
    StarEq(::treesitter_types::Span),
    PlusEq(::treesitter_types::Span),
    MinusEq(::treesitter_types::Span),
    SlashEq(::treesitter_types::Span),
    ShlEq(::treesitter_types::Span),
    Eq(::treesitter_types::Span),
    ShrEq(::treesitter_types::Span),
    CaretEq(::treesitter_types::Span),
    PipeEq(::treesitter_types::Span),
}
impl<'tree> ::treesitter_types::FromNode<'tree> for AssignmentExpressionOperator {
    #[allow(clippy::collapsible_else_if)]
    fn from_node(
        node: ::treesitter_types::tree_sitter::Node<'tree>,
        _src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        match node.kind() {
            "%=" => Ok(Self::PercentEq(::treesitter_types::Span::from(node))),
            "&=" => Ok(Self::AmpEq(::treesitter_types::Span::from(node))),
            "*=" => Ok(Self::StarEq(::treesitter_types::Span::from(node))),
            "+=" => Ok(Self::PlusEq(::treesitter_types::Span::from(node))),
            "-=" => Ok(Self::MinusEq(::treesitter_types::Span::from(node))),
            "/=" => Ok(Self::SlashEq(::treesitter_types::Span::from(node))),
            "<<=" => Ok(Self::ShlEq(::treesitter_types::Span::from(node))),
            "=" => Ok(Self::Eq(::treesitter_types::Span::from(node))),
            ">>=" => Ok(Self::ShrEq(::treesitter_types::Span::from(node))),
            "^=" => Ok(Self::CaretEq(::treesitter_types::Span::from(node))),
            "|=" => Ok(Self::PipeEq(::treesitter_types::Span::from(node))),
            other => Err(::treesitter_types::ParseError::unexpected_kind(other, node)),
        }
    }
}
impl ::treesitter_types::Spanned for AssignmentExpressionOperator {
    fn span(&self) -> ::treesitter_types::Span {
        match self {
            Self::PercentEq(span) => *span,
            Self::AmpEq(span) => *span,
            Self::StarEq(span) => *span,
            Self::PlusEq(span) => *span,
            Self::MinusEq(span) => *span,
            Self::SlashEq(span) => *span,
            Self::ShlEq(span) => *span,
            Self::Eq(span) => *span,
            Self::ShrEq(span) => *span,
            Self::CaretEq(span) => *span,
            Self::PipeEq(span) => *span,
        }
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum AttributedDeclaratorChildren<'tree> {
    Declarator(::std::boxed::Box<Declarator<'tree>>),
    FieldDeclarator(::std::boxed::Box<FieldDeclarator<'tree>>),
    TypeDeclarator(::std::boxed::Box<TypeDeclarator<'tree>>),
    AttributeDeclaration(::std::boxed::Box<AttributeDeclaration<'tree>>),
}
impl<'tree> ::treesitter_types::FromNode<'tree> for AttributedDeclaratorChildren<'tree> {
    #[allow(clippy::collapsible_else_if)]
    fn from_node(
        node: ::treesitter_types::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        match node.kind() {
            "attribute_declaration" => Ok(Self::AttributeDeclaration(::std::boxed::Box::new(
                ::treesitter_types::runtime::maybe_grow_stack(|| {
                    <AttributeDeclaration as ::treesitter_types::FromNode>::from_node(node, src)
                })?,
            ))),
            _other => {
                if let Ok(v) = ::treesitter_types::runtime::maybe_grow_stack(|| {
                    <Declarator as ::treesitter_types::FromNode>::from_node(node, src)
                }) {
                    Ok(Self::Declarator(::std::boxed::Box::new(v)))
                } else {
                    if let Ok(v) = ::treesitter_types::runtime::maybe_grow_stack(|| {
                        <FieldDeclarator as ::treesitter_types::FromNode>::from_node(node, src)
                    }) {
                        Ok(Self::FieldDeclarator(::std::boxed::Box::new(v)))
                    } else {
                        if let Ok(v) = ::treesitter_types::runtime::maybe_grow_stack(|| {
                            <TypeDeclarator as ::treesitter_types::FromNode>::from_node(node, src)
                        }) {
                            Ok(Self::TypeDeclarator(::std::boxed::Box::new(v)))
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
}
impl ::treesitter_types::Spanned for AttributedDeclaratorChildren<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        match self {
            Self::Declarator(inner) => inner.span(),
            Self::FieldDeclarator(inner) => inner.span(),
            Self::TypeDeclarator(inner) => inner.span(),
            Self::AttributeDeclaration(inner) => inner.span(),
        }
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum AttributedStatementChildren<'tree> {
    AttributeDeclaration(::std::boxed::Box<AttributeDeclaration<'tree>>),
    Statement(::std::boxed::Box<Statement<'tree>>),
}
impl<'tree> ::treesitter_types::FromNode<'tree> for AttributedStatementChildren<'tree> {
    #[allow(clippy::collapsible_else_if)]
    fn from_node(
        node: ::treesitter_types::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        match node.kind() {
            "attribute_declaration" => Ok(Self::AttributeDeclaration(::std::boxed::Box::new(
                ::treesitter_types::runtime::maybe_grow_stack(|| {
                    <AttributeDeclaration as ::treesitter_types::FromNode>::from_node(node, src)
                })?,
            ))),
            _other => {
                if let Ok(v) = ::treesitter_types::runtime::maybe_grow_stack(|| {
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
impl ::treesitter_types::Spanned for AttributedStatementChildren<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        match self {
            Self::AttributeDeclaration(inner) => inner.span(),
            Self::Statement(inner) => inner.span(),
        }
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum BinaryExpressionLeft<'tree> {
    Expression(::std::boxed::Box<Expression<'tree>>),
    PreprocDefined(::std::boxed::Box<PreprocDefined<'tree>>),
}
impl<'tree> ::treesitter_types::FromNode<'tree> for BinaryExpressionLeft<'tree> {
    #[allow(clippy::collapsible_else_if)]
    fn from_node(
        node: ::treesitter_types::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        match node.kind() {
            "preproc_defined" => Ok(Self::PreprocDefined(::std::boxed::Box::new(
                ::treesitter_types::runtime::maybe_grow_stack(|| {
                    <PreprocDefined as ::treesitter_types::FromNode>::from_node(node, src)
                })?,
            ))),
            _other => {
                if let Ok(v) = ::treesitter_types::runtime::maybe_grow_stack(|| {
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
impl ::treesitter_types::Spanned for BinaryExpressionLeft<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        match self {
            Self::Expression(inner) => inner.span(),
            Self::PreprocDefined(inner) => inner.span(),
        }
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum BinaryExpressionOperator {
    NotEq(::treesitter_types::Span),
    Percent(::treesitter_types::Span),
    Amp(::treesitter_types::Span),
    AmpAmp(::treesitter_types::Span),
    Star(::treesitter_types::Span),
    Plus(::treesitter_types::Span),
    Minus(::treesitter_types::Span),
    Slash(::treesitter_types::Span),
    Lt(::treesitter_types::Span),
    Shl(::treesitter_types::Span),
    LtEq(::treesitter_types::Span),
    EqEq(::treesitter_types::Span),
    Gt(::treesitter_types::Span),
    GtEq(::treesitter_types::Span),
    Shr(::treesitter_types::Span),
    Caret(::treesitter_types::Span),
    Pipe(::treesitter_types::Span),
    PipePipe(::treesitter_types::Span),
}
impl<'tree> ::treesitter_types::FromNode<'tree> for BinaryExpressionOperator {
    #[allow(clippy::collapsible_else_if)]
    fn from_node(
        node: ::treesitter_types::tree_sitter::Node<'tree>,
        _src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        match node.kind() {
            "!=" => Ok(Self::NotEq(::treesitter_types::Span::from(node))),
            "%" => Ok(Self::Percent(::treesitter_types::Span::from(node))),
            "&" => Ok(Self::Amp(::treesitter_types::Span::from(node))),
            "&&" => Ok(Self::AmpAmp(::treesitter_types::Span::from(node))),
            "*" => Ok(Self::Star(::treesitter_types::Span::from(node))),
            "+" => Ok(Self::Plus(::treesitter_types::Span::from(node))),
            "-" => Ok(Self::Minus(::treesitter_types::Span::from(node))),
            "/" => Ok(Self::Slash(::treesitter_types::Span::from(node))),
            "<" => Ok(Self::Lt(::treesitter_types::Span::from(node))),
            "<<" => Ok(Self::Shl(::treesitter_types::Span::from(node))),
            "<=" => Ok(Self::LtEq(::treesitter_types::Span::from(node))),
            "==" => Ok(Self::EqEq(::treesitter_types::Span::from(node))),
            ">" => Ok(Self::Gt(::treesitter_types::Span::from(node))),
            ">=" => Ok(Self::GtEq(::treesitter_types::Span::from(node))),
            ">>" => Ok(Self::Shr(::treesitter_types::Span::from(node))),
            "^" => Ok(Self::Caret(::treesitter_types::Span::from(node))),
            "|" => Ok(Self::Pipe(::treesitter_types::Span::from(node))),
            "||" => Ok(Self::PipePipe(::treesitter_types::Span::from(node))),
            other => Err(::treesitter_types::ParseError::unexpected_kind(other, node)),
        }
    }
}
impl ::treesitter_types::Spanned for BinaryExpressionOperator {
    fn span(&self) -> ::treesitter_types::Span {
        match self {
            Self::NotEq(span) => *span,
            Self::Percent(span) => *span,
            Self::Amp(span) => *span,
            Self::AmpAmp(span) => *span,
            Self::Star(span) => *span,
            Self::Plus(span) => *span,
            Self::Minus(span) => *span,
            Self::Slash(span) => *span,
            Self::Lt(span) => *span,
            Self::Shl(span) => *span,
            Self::LtEq(span) => *span,
            Self::EqEq(span) => *span,
            Self::Gt(span) => *span,
            Self::GtEq(span) => *span,
            Self::Shr(span) => *span,
            Self::Caret(span) => *span,
            Self::Pipe(span) => *span,
            Self::PipePipe(span) => *span,
        }
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum BinaryExpressionRight<'tree> {
    Expression(::std::boxed::Box<Expression<'tree>>),
    PreprocDefined(::std::boxed::Box<PreprocDefined<'tree>>),
}
impl<'tree> ::treesitter_types::FromNode<'tree> for BinaryExpressionRight<'tree> {
    #[allow(clippy::collapsible_else_if)]
    fn from_node(
        node: ::treesitter_types::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        match node.kind() {
            "preproc_defined" => Ok(Self::PreprocDefined(::std::boxed::Box::new(
                ::treesitter_types::runtime::maybe_grow_stack(|| {
                    <PreprocDefined as ::treesitter_types::FromNode>::from_node(node, src)
                })?,
            ))),
            _other => {
                if let Ok(v) = ::treesitter_types::runtime::maybe_grow_stack(|| {
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
impl ::treesitter_types::Spanned for BinaryExpressionRight<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        match self {
            Self::Expression(inner) => inner.span(),
            Self::PreprocDefined(inner) => inner.span(),
        }
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum CaseStatementChildren<'tree> {
    AttributedStatement(::std::boxed::Box<AttributedStatement<'tree>>),
    BreakStatement(::std::boxed::Box<BreakStatement<'tree>>),
    CompoundStatement(::std::boxed::Box<CompoundStatement<'tree>>),
    ContinueStatement(::std::boxed::Box<ContinueStatement<'tree>>),
    Declaration(::std::boxed::Box<Declaration<'tree>>),
    DoStatement(::std::boxed::Box<DoStatement<'tree>>),
    ExpressionStatement(::std::boxed::Box<ExpressionStatement<'tree>>),
    ForStatement(::std::boxed::Box<ForStatement<'tree>>),
    GotoStatement(::std::boxed::Box<GotoStatement<'tree>>),
    IfStatement(::std::boxed::Box<IfStatement<'tree>>),
    LabeledStatement(::std::boxed::Box<LabeledStatement<'tree>>),
    ReturnStatement(::std::boxed::Box<ReturnStatement<'tree>>),
    SehLeaveStatement(::std::boxed::Box<SehLeaveStatement<'tree>>),
    SehTryStatement(::std::boxed::Box<SehTryStatement<'tree>>),
    SwitchStatement(::std::boxed::Box<SwitchStatement<'tree>>),
    TypeDefinition(::std::boxed::Box<TypeDefinition<'tree>>),
    WhileStatement(::std::boxed::Box<WhileStatement<'tree>>),
}
impl<'tree> ::treesitter_types::FromNode<'tree> for CaseStatementChildren<'tree> {
    #[allow(clippy::collapsible_else_if)]
    fn from_node(
        node: ::treesitter_types::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        match node.kind() {
            "attributed_statement" => Ok(Self::AttributedStatement(::std::boxed::Box::new(
                ::treesitter_types::runtime::maybe_grow_stack(|| {
                    <AttributedStatement as ::treesitter_types::FromNode>::from_node(node, src)
                })?,
            ))),
            "break_statement" => Ok(Self::BreakStatement(::std::boxed::Box::new(
                ::treesitter_types::runtime::maybe_grow_stack(|| {
                    <BreakStatement as ::treesitter_types::FromNode>::from_node(node, src)
                })?,
            ))),
            "compound_statement" => Ok(Self::CompoundStatement(::std::boxed::Box::new(
                ::treesitter_types::runtime::maybe_grow_stack(|| {
                    <CompoundStatement as ::treesitter_types::FromNode>::from_node(node, src)
                })?,
            ))),
            "continue_statement" => Ok(Self::ContinueStatement(::std::boxed::Box::new(
                ::treesitter_types::runtime::maybe_grow_stack(|| {
                    <ContinueStatement as ::treesitter_types::FromNode>::from_node(node, src)
                })?,
            ))),
            "declaration" => Ok(Self::Declaration(::std::boxed::Box::new(
                ::treesitter_types::runtime::maybe_grow_stack(|| {
                    <Declaration as ::treesitter_types::FromNode>::from_node(node, src)
                })?,
            ))),
            "do_statement" => Ok(Self::DoStatement(::std::boxed::Box::new(
                ::treesitter_types::runtime::maybe_grow_stack(|| {
                    <DoStatement as ::treesitter_types::FromNode>::from_node(node, src)
                })?,
            ))),
            "expression_statement" => Ok(Self::ExpressionStatement(::std::boxed::Box::new(
                ::treesitter_types::runtime::maybe_grow_stack(|| {
                    <ExpressionStatement as ::treesitter_types::FromNode>::from_node(node, src)
                })?,
            ))),
            "for_statement" => Ok(Self::ForStatement(::std::boxed::Box::new(
                ::treesitter_types::runtime::maybe_grow_stack(|| {
                    <ForStatement as ::treesitter_types::FromNode>::from_node(node, src)
                })?,
            ))),
            "goto_statement" => Ok(Self::GotoStatement(::std::boxed::Box::new(
                ::treesitter_types::runtime::maybe_grow_stack(|| {
                    <GotoStatement as ::treesitter_types::FromNode>::from_node(node, src)
                })?,
            ))),
            "if_statement" => Ok(Self::IfStatement(::std::boxed::Box::new(
                ::treesitter_types::runtime::maybe_grow_stack(|| {
                    <IfStatement as ::treesitter_types::FromNode>::from_node(node, src)
                })?,
            ))),
            "labeled_statement" => Ok(Self::LabeledStatement(::std::boxed::Box::new(
                ::treesitter_types::runtime::maybe_grow_stack(|| {
                    <LabeledStatement as ::treesitter_types::FromNode>::from_node(node, src)
                })?,
            ))),
            "return_statement" => Ok(Self::ReturnStatement(::std::boxed::Box::new(
                ::treesitter_types::runtime::maybe_grow_stack(|| {
                    <ReturnStatement as ::treesitter_types::FromNode>::from_node(node, src)
                })?,
            ))),
            "seh_leave_statement" => Ok(Self::SehLeaveStatement(::std::boxed::Box::new(
                ::treesitter_types::runtime::maybe_grow_stack(|| {
                    <SehLeaveStatement as ::treesitter_types::FromNode>::from_node(node, src)
                })?,
            ))),
            "seh_try_statement" => Ok(Self::SehTryStatement(::std::boxed::Box::new(
                ::treesitter_types::runtime::maybe_grow_stack(|| {
                    <SehTryStatement as ::treesitter_types::FromNode>::from_node(node, src)
                })?,
            ))),
            "switch_statement" => Ok(Self::SwitchStatement(::std::boxed::Box::new(
                ::treesitter_types::runtime::maybe_grow_stack(|| {
                    <SwitchStatement as ::treesitter_types::FromNode>::from_node(node, src)
                })?,
            ))),
            "type_definition" => Ok(Self::TypeDefinition(::std::boxed::Box::new(
                ::treesitter_types::runtime::maybe_grow_stack(|| {
                    <TypeDefinition as ::treesitter_types::FromNode>::from_node(node, src)
                })?,
            ))),
            "while_statement" => Ok(Self::WhileStatement(::std::boxed::Box::new(
                ::treesitter_types::runtime::maybe_grow_stack(|| {
                    <WhileStatement as ::treesitter_types::FromNode>::from_node(node, src)
                })?,
            ))),
            other => Err(::treesitter_types::ParseError::unexpected_kind(other, node)),
        }
    }
}
impl ::treesitter_types::Spanned for CaseStatementChildren<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        match self {
            Self::AttributedStatement(inner) => inner.span(),
            Self::BreakStatement(inner) => inner.span(),
            Self::CompoundStatement(inner) => inner.span(),
            Self::ContinueStatement(inner) => inner.span(),
            Self::Declaration(inner) => inner.span(),
            Self::DoStatement(inner) => inner.span(),
            Self::ExpressionStatement(inner) => inner.span(),
            Self::ForStatement(inner) => inner.span(),
            Self::GotoStatement(inner) => inner.span(),
            Self::IfStatement(inner) => inner.span(),
            Self::LabeledStatement(inner) => inner.span(),
            Self::ReturnStatement(inner) => inner.span(),
            Self::SehLeaveStatement(inner) => inner.span(),
            Self::SehTryStatement(inner) => inner.span(),
            Self::SwitchStatement(inner) => inner.span(),
            Self::TypeDefinition(inner) => inner.span(),
            Self::WhileStatement(inner) => inner.span(),
        }
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum CharLiteralChildren<'tree> {
    Character(::std::boxed::Box<Character<'tree>>),
    EscapeSequence(::std::boxed::Box<EscapeSequence<'tree>>),
}
impl<'tree> ::treesitter_types::FromNode<'tree> for CharLiteralChildren<'tree> {
    #[allow(clippy::collapsible_else_if)]
    fn from_node(
        node: ::treesitter_types::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        match node.kind() {
            "character" => Ok(Self::Character(::std::boxed::Box::new(
                ::treesitter_types::runtime::maybe_grow_stack(|| {
                    <Character as ::treesitter_types::FromNode>::from_node(node, src)
                })?,
            ))),
            "escape_sequence" => Ok(Self::EscapeSequence(::std::boxed::Box::new(
                ::treesitter_types::runtime::maybe_grow_stack(|| {
                    <EscapeSequence as ::treesitter_types::FromNode>::from_node(node, src)
                })?,
            ))),
            other => Err(::treesitter_types::ParseError::unexpected_kind(other, node)),
        }
    }
}
impl ::treesitter_types::Spanned for CharLiteralChildren<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        match self {
            Self::Character(inner) => inner.span(),
            Self::EscapeSequence(inner) => inner.span(),
        }
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum CommaExpressionRight<'tree> {
    CommaExpression(::std::boxed::Box<CommaExpression<'tree>>),
    Expression(::std::boxed::Box<Expression<'tree>>),
}
impl<'tree> ::treesitter_types::FromNode<'tree> for CommaExpressionRight<'tree> {
    #[allow(clippy::collapsible_else_if)]
    fn from_node(
        node: ::treesitter_types::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        match node.kind() {
            "comma_expression" => Ok(Self::CommaExpression(::std::boxed::Box::new(
                ::treesitter_types::runtime::maybe_grow_stack(|| {
                    <CommaExpression as ::treesitter_types::FromNode>::from_node(node, src)
                })?,
            ))),
            _other => {
                if let Ok(v) = ::treesitter_types::runtime::maybe_grow_stack(|| {
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
impl ::treesitter_types::Spanned for CommaExpressionRight<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        match self {
            Self::CommaExpression(inner) => inner.span(),
            Self::Expression(inner) => inner.span(),
        }
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum CompoundStatementChildren<'tree> {
    Declaration(::std::boxed::Box<Declaration<'tree>>),
    FunctionDefinition(::std::boxed::Box<FunctionDefinition<'tree>>),
    LinkageSpecification(::std::boxed::Box<LinkageSpecification<'tree>>),
    PreprocCall(::std::boxed::Box<PreprocCall<'tree>>),
    PreprocDef(::std::boxed::Box<PreprocDef<'tree>>),
    PreprocFunctionDef(::std::boxed::Box<PreprocFunctionDef<'tree>>),
    PreprocIf(::std::boxed::Box<PreprocIf<'tree>>),
    PreprocIfdef(::std::boxed::Box<PreprocIfdef<'tree>>),
    PreprocInclude(::std::boxed::Box<PreprocInclude<'tree>>),
    Statement(::std::boxed::Box<Statement<'tree>>),
    TypeDefinition(::std::boxed::Box<TypeDefinition<'tree>>),
    TypeSpecifier(::std::boxed::Box<TypeSpecifier<'tree>>),
}
impl<'tree> ::treesitter_types::FromNode<'tree> for CompoundStatementChildren<'tree> {
    #[allow(clippy::collapsible_else_if)]
    fn from_node(
        node: ::treesitter_types::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        match node.kind() {
            "declaration" => Ok(Self::Declaration(::std::boxed::Box::new(
                ::treesitter_types::runtime::maybe_grow_stack(|| {
                    <Declaration as ::treesitter_types::FromNode>::from_node(node, src)
                })?,
            ))),
            "function_definition" => Ok(Self::FunctionDefinition(::std::boxed::Box::new(
                ::treesitter_types::runtime::maybe_grow_stack(|| {
                    <FunctionDefinition as ::treesitter_types::FromNode>::from_node(node, src)
                })?,
            ))),
            "linkage_specification" => Ok(Self::LinkageSpecification(::std::boxed::Box::new(
                ::treesitter_types::runtime::maybe_grow_stack(|| {
                    <LinkageSpecification as ::treesitter_types::FromNode>::from_node(node, src)
                })?,
            ))),
            "preproc_call" => Ok(Self::PreprocCall(::std::boxed::Box::new(
                ::treesitter_types::runtime::maybe_grow_stack(|| {
                    <PreprocCall as ::treesitter_types::FromNode>::from_node(node, src)
                })?,
            ))),
            "preproc_def" => Ok(Self::PreprocDef(::std::boxed::Box::new(
                ::treesitter_types::runtime::maybe_grow_stack(|| {
                    <PreprocDef as ::treesitter_types::FromNode>::from_node(node, src)
                })?,
            ))),
            "preproc_function_def" => Ok(Self::PreprocFunctionDef(::std::boxed::Box::new(
                ::treesitter_types::runtime::maybe_grow_stack(|| {
                    <PreprocFunctionDef as ::treesitter_types::FromNode>::from_node(node, src)
                })?,
            ))),
            "preproc_if" => Ok(Self::PreprocIf(::std::boxed::Box::new(
                ::treesitter_types::runtime::maybe_grow_stack(|| {
                    <PreprocIf as ::treesitter_types::FromNode>::from_node(node, src)
                })?,
            ))),
            "preproc_ifdef" => Ok(Self::PreprocIfdef(::std::boxed::Box::new(
                ::treesitter_types::runtime::maybe_grow_stack(|| {
                    <PreprocIfdef as ::treesitter_types::FromNode>::from_node(node, src)
                })?,
            ))),
            "preproc_include" => Ok(Self::PreprocInclude(::std::boxed::Box::new(
                ::treesitter_types::runtime::maybe_grow_stack(|| {
                    <PreprocInclude as ::treesitter_types::FromNode>::from_node(node, src)
                })?,
            ))),
            "type_definition" => Ok(Self::TypeDefinition(::std::boxed::Box::new(
                ::treesitter_types::runtime::maybe_grow_stack(|| {
                    <TypeDefinition as ::treesitter_types::FromNode>::from_node(node, src)
                })?,
            ))),
            _other => {
                if let Ok(v) = ::treesitter_types::runtime::maybe_grow_stack(|| {
                    <Statement as ::treesitter_types::FromNode>::from_node(node, src)
                }) {
                    Ok(Self::Statement(::std::boxed::Box::new(v)))
                } else {
                    if let Ok(v) = ::treesitter_types::runtime::maybe_grow_stack(|| {
                        <TypeSpecifier as ::treesitter_types::FromNode>::from_node(node, src)
                    }) {
                        Ok(Self::TypeSpecifier(::std::boxed::Box::new(v)))
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
impl ::treesitter_types::Spanned for CompoundStatementChildren<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        match self {
            Self::Declaration(inner) => inner.span(),
            Self::FunctionDefinition(inner) => inner.span(),
            Self::LinkageSpecification(inner) => inner.span(),
            Self::PreprocCall(inner) => inner.span(),
            Self::PreprocDef(inner) => inner.span(),
            Self::PreprocFunctionDef(inner) => inner.span(),
            Self::PreprocIf(inner) => inner.span(),
            Self::PreprocIfdef(inner) => inner.span(),
            Self::PreprocInclude(inner) => inner.span(),
            Self::Statement(inner) => inner.span(),
            Self::TypeDefinition(inner) => inner.span(),
            Self::TypeSpecifier(inner) => inner.span(),
        }
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ConcatenatedStringChildren<'tree> {
    Identifier(::std::boxed::Box<Identifier<'tree>>),
    StringLiteral(::std::boxed::Box<StringLiteral<'tree>>),
}
impl<'tree> ::treesitter_types::FromNode<'tree> for ConcatenatedStringChildren<'tree> {
    #[allow(clippy::collapsible_else_if)]
    fn from_node(
        node: ::treesitter_types::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        match node.kind() {
            "identifier" => Ok(Self::Identifier(::std::boxed::Box::new(
                ::treesitter_types::runtime::maybe_grow_stack(|| {
                    <Identifier as ::treesitter_types::FromNode>::from_node(node, src)
                })?,
            ))),
            "string_literal" => Ok(Self::StringLiteral(::std::boxed::Box::new(
                ::treesitter_types::runtime::maybe_grow_stack(|| {
                    <StringLiteral as ::treesitter_types::FromNode>::from_node(node, src)
                })?,
            ))),
            other => Err(::treesitter_types::ParseError::unexpected_kind(other, node)),
        }
    }
}
impl ::treesitter_types::Spanned for ConcatenatedStringChildren<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        match self {
            Self::Identifier(inner) => inner.span(),
            Self::StringLiteral(inner) => inner.span(),
        }
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ConditionalExpressionConsequence<'tree> {
    CommaExpression(::std::boxed::Box<CommaExpression<'tree>>),
    Expression(::std::boxed::Box<Expression<'tree>>),
}
impl<'tree> ::treesitter_types::FromNode<'tree> for ConditionalExpressionConsequence<'tree> {
    #[allow(clippy::collapsible_else_if)]
    fn from_node(
        node: ::treesitter_types::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        match node.kind() {
            "comma_expression" => Ok(Self::CommaExpression(::std::boxed::Box::new(
                ::treesitter_types::runtime::maybe_grow_stack(|| {
                    <CommaExpression as ::treesitter_types::FromNode>::from_node(node, src)
                })?,
            ))),
            _other => {
                if let Ok(v) = ::treesitter_types::runtime::maybe_grow_stack(|| {
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
impl ::treesitter_types::Spanned for ConditionalExpressionConsequence<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        match self {
            Self::CommaExpression(inner) => inner.span(),
            Self::Expression(inner) => inner.span(),
        }
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum DeclarationDeclarator<'tree> {
    ArrayDeclarator(::std::boxed::Box<ArrayDeclarator<'tree>>),
    AttributedDeclarator(::std::boxed::Box<AttributedDeclarator<'tree>>),
    FunctionDeclarator(::std::boxed::Box<FunctionDeclarator<'tree>>),
    GnuAsmExpression(::std::boxed::Box<GnuAsmExpression<'tree>>),
    Identifier(::std::boxed::Box<Identifier<'tree>>),
    InitDeclarator(::std::boxed::Box<InitDeclarator<'tree>>),
    MsCallModifier(::std::boxed::Box<MsCallModifier<'tree>>),
    ParenthesizedDeclarator(::std::boxed::Box<ParenthesizedDeclarator<'tree>>),
    PointerDeclarator(::std::boxed::Box<PointerDeclarator<'tree>>),
}
impl<'tree> ::treesitter_types::FromNode<'tree> for DeclarationDeclarator<'tree> {
    #[allow(clippy::collapsible_else_if)]
    fn from_node(
        node: ::treesitter_types::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        match node.kind() {
            "array_declarator" => Ok(Self::ArrayDeclarator(::std::boxed::Box::new(
                ::treesitter_types::runtime::maybe_grow_stack(|| {
                    <ArrayDeclarator as ::treesitter_types::FromNode>::from_node(node, src)
                })?,
            ))),
            "attributed_declarator" => Ok(Self::AttributedDeclarator(::std::boxed::Box::new(
                ::treesitter_types::runtime::maybe_grow_stack(|| {
                    <AttributedDeclarator as ::treesitter_types::FromNode>::from_node(node, src)
                })?,
            ))),
            "function_declarator" => Ok(Self::FunctionDeclarator(::std::boxed::Box::new(
                ::treesitter_types::runtime::maybe_grow_stack(|| {
                    <FunctionDeclarator as ::treesitter_types::FromNode>::from_node(node, src)
                })?,
            ))),
            "gnu_asm_expression" => Ok(Self::GnuAsmExpression(::std::boxed::Box::new(
                ::treesitter_types::runtime::maybe_grow_stack(|| {
                    <GnuAsmExpression as ::treesitter_types::FromNode>::from_node(node, src)
                })?,
            ))),
            "identifier" => Ok(Self::Identifier(::std::boxed::Box::new(
                ::treesitter_types::runtime::maybe_grow_stack(|| {
                    <Identifier as ::treesitter_types::FromNode>::from_node(node, src)
                })?,
            ))),
            "init_declarator" => Ok(Self::InitDeclarator(::std::boxed::Box::new(
                ::treesitter_types::runtime::maybe_grow_stack(|| {
                    <InitDeclarator as ::treesitter_types::FromNode>::from_node(node, src)
                })?,
            ))),
            "ms_call_modifier" => Ok(Self::MsCallModifier(::std::boxed::Box::new(
                ::treesitter_types::runtime::maybe_grow_stack(|| {
                    <MsCallModifier as ::treesitter_types::FromNode>::from_node(node, src)
                })?,
            ))),
            "parenthesized_declarator" => Ok(Self::ParenthesizedDeclarator(
                ::std::boxed::Box::new(::treesitter_types::runtime::maybe_grow_stack(|| {
                    <ParenthesizedDeclarator as ::treesitter_types::FromNode>::from_node(node, src)
                })?),
            )),
            "pointer_declarator" => Ok(Self::PointerDeclarator(::std::boxed::Box::new(
                ::treesitter_types::runtime::maybe_grow_stack(|| {
                    <PointerDeclarator as ::treesitter_types::FromNode>::from_node(node, src)
                })?,
            ))),
            other => Err(::treesitter_types::ParseError::unexpected_kind(other, node)),
        }
    }
}
impl ::treesitter_types::Spanned for DeclarationDeclarator<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        match self {
            Self::ArrayDeclarator(inner) => inner.span(),
            Self::AttributedDeclarator(inner) => inner.span(),
            Self::FunctionDeclarator(inner) => inner.span(),
            Self::GnuAsmExpression(inner) => inner.span(),
            Self::Identifier(inner) => inner.span(),
            Self::InitDeclarator(inner) => inner.span(),
            Self::MsCallModifier(inner) => inner.span(),
            Self::ParenthesizedDeclarator(inner) => inner.span(),
            Self::PointerDeclarator(inner) => inner.span(),
        }
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum DeclarationChildren<'tree> {
    AttributeDeclaration(::std::boxed::Box<AttributeDeclaration<'tree>>),
    AttributeSpecifier(::std::boxed::Box<AttributeSpecifier<'tree>>),
    MsDeclspecModifier(::std::boxed::Box<MsDeclspecModifier<'tree>>),
    StorageClassSpecifier(::std::boxed::Box<StorageClassSpecifier<'tree>>),
    TypeQualifier(::std::boxed::Box<TypeQualifier<'tree>>),
}
impl<'tree> ::treesitter_types::FromNode<'tree> for DeclarationChildren<'tree> {
    #[allow(clippy::collapsible_else_if)]
    fn from_node(
        node: ::treesitter_types::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        match node.kind() {
            "attribute_declaration" => Ok(Self::AttributeDeclaration(::std::boxed::Box::new(
                ::treesitter_types::runtime::maybe_grow_stack(|| {
                    <AttributeDeclaration as ::treesitter_types::FromNode>::from_node(node, src)
                })?,
            ))),
            "attribute_specifier" => Ok(Self::AttributeSpecifier(::std::boxed::Box::new(
                ::treesitter_types::runtime::maybe_grow_stack(|| {
                    <AttributeSpecifier as ::treesitter_types::FromNode>::from_node(node, src)
                })?,
            ))),
            "ms_declspec_modifier" => Ok(Self::MsDeclspecModifier(::std::boxed::Box::new(
                ::treesitter_types::runtime::maybe_grow_stack(|| {
                    <MsDeclspecModifier as ::treesitter_types::FromNode>::from_node(node, src)
                })?,
            ))),
            "storage_class_specifier" => Ok(Self::StorageClassSpecifier(::std::boxed::Box::new(
                ::treesitter_types::runtime::maybe_grow_stack(|| {
                    <StorageClassSpecifier as ::treesitter_types::FromNode>::from_node(node, src)
                })?,
            ))),
            "type_qualifier" => Ok(Self::TypeQualifier(::std::boxed::Box::new(
                ::treesitter_types::runtime::maybe_grow_stack(|| {
                    <TypeQualifier as ::treesitter_types::FromNode>::from_node(node, src)
                })?,
            ))),
            other => Err(::treesitter_types::ParseError::unexpected_kind(other, node)),
        }
    }
}
impl ::treesitter_types::Spanned for DeclarationChildren<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        match self {
            Self::AttributeDeclaration(inner) => inner.span(),
            Self::AttributeSpecifier(inner) => inner.span(),
            Self::MsDeclspecModifier(inner) => inner.span(),
            Self::StorageClassSpecifier(inner) => inner.span(),
            Self::TypeQualifier(inner) => inner.span(),
        }
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum DeclarationListChildren<'tree> {
    Declaration(::std::boxed::Box<Declaration<'tree>>),
    FunctionDefinition(::std::boxed::Box<FunctionDefinition<'tree>>),
    LinkageSpecification(::std::boxed::Box<LinkageSpecification<'tree>>),
    PreprocCall(::std::boxed::Box<PreprocCall<'tree>>),
    PreprocDef(::std::boxed::Box<PreprocDef<'tree>>),
    PreprocFunctionDef(::std::boxed::Box<PreprocFunctionDef<'tree>>),
    PreprocIf(::std::boxed::Box<PreprocIf<'tree>>),
    PreprocIfdef(::std::boxed::Box<PreprocIfdef<'tree>>),
    PreprocInclude(::std::boxed::Box<PreprocInclude<'tree>>),
    Statement(::std::boxed::Box<Statement<'tree>>),
    TypeDefinition(::std::boxed::Box<TypeDefinition<'tree>>),
    TypeSpecifier(::std::boxed::Box<TypeSpecifier<'tree>>),
}
impl<'tree> ::treesitter_types::FromNode<'tree> for DeclarationListChildren<'tree> {
    #[allow(clippy::collapsible_else_if)]
    fn from_node(
        node: ::treesitter_types::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        match node.kind() {
            "declaration" => Ok(Self::Declaration(::std::boxed::Box::new(
                ::treesitter_types::runtime::maybe_grow_stack(|| {
                    <Declaration as ::treesitter_types::FromNode>::from_node(node, src)
                })?,
            ))),
            "function_definition" => Ok(Self::FunctionDefinition(::std::boxed::Box::new(
                ::treesitter_types::runtime::maybe_grow_stack(|| {
                    <FunctionDefinition as ::treesitter_types::FromNode>::from_node(node, src)
                })?,
            ))),
            "linkage_specification" => Ok(Self::LinkageSpecification(::std::boxed::Box::new(
                ::treesitter_types::runtime::maybe_grow_stack(|| {
                    <LinkageSpecification as ::treesitter_types::FromNode>::from_node(node, src)
                })?,
            ))),
            "preproc_call" => Ok(Self::PreprocCall(::std::boxed::Box::new(
                ::treesitter_types::runtime::maybe_grow_stack(|| {
                    <PreprocCall as ::treesitter_types::FromNode>::from_node(node, src)
                })?,
            ))),
            "preproc_def" => Ok(Self::PreprocDef(::std::boxed::Box::new(
                ::treesitter_types::runtime::maybe_grow_stack(|| {
                    <PreprocDef as ::treesitter_types::FromNode>::from_node(node, src)
                })?,
            ))),
            "preproc_function_def" => Ok(Self::PreprocFunctionDef(::std::boxed::Box::new(
                ::treesitter_types::runtime::maybe_grow_stack(|| {
                    <PreprocFunctionDef as ::treesitter_types::FromNode>::from_node(node, src)
                })?,
            ))),
            "preproc_if" => Ok(Self::PreprocIf(::std::boxed::Box::new(
                ::treesitter_types::runtime::maybe_grow_stack(|| {
                    <PreprocIf as ::treesitter_types::FromNode>::from_node(node, src)
                })?,
            ))),
            "preproc_ifdef" => Ok(Self::PreprocIfdef(::std::boxed::Box::new(
                ::treesitter_types::runtime::maybe_grow_stack(|| {
                    <PreprocIfdef as ::treesitter_types::FromNode>::from_node(node, src)
                })?,
            ))),
            "preproc_include" => Ok(Self::PreprocInclude(::std::boxed::Box::new(
                ::treesitter_types::runtime::maybe_grow_stack(|| {
                    <PreprocInclude as ::treesitter_types::FromNode>::from_node(node, src)
                })?,
            ))),
            "type_definition" => Ok(Self::TypeDefinition(::std::boxed::Box::new(
                ::treesitter_types::runtime::maybe_grow_stack(|| {
                    <TypeDefinition as ::treesitter_types::FromNode>::from_node(node, src)
                })?,
            ))),
            _other => {
                if let Ok(v) = ::treesitter_types::runtime::maybe_grow_stack(|| {
                    <Statement as ::treesitter_types::FromNode>::from_node(node, src)
                }) {
                    Ok(Self::Statement(::std::boxed::Box::new(v)))
                } else {
                    if let Ok(v) = ::treesitter_types::runtime::maybe_grow_stack(|| {
                        <TypeSpecifier as ::treesitter_types::FromNode>::from_node(node, src)
                    }) {
                        Ok(Self::TypeSpecifier(::std::boxed::Box::new(v)))
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
impl ::treesitter_types::Spanned for DeclarationListChildren<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        match self {
            Self::Declaration(inner) => inner.span(),
            Self::FunctionDefinition(inner) => inner.span(),
            Self::LinkageSpecification(inner) => inner.span(),
            Self::PreprocCall(inner) => inner.span(),
            Self::PreprocDef(inner) => inner.span(),
            Self::PreprocFunctionDef(inner) => inner.span(),
            Self::PreprocIf(inner) => inner.span(),
            Self::PreprocIfdef(inner) => inner.span(),
            Self::PreprocInclude(inner) => inner.span(),
            Self::Statement(inner) => inner.span(),
            Self::TypeDefinition(inner) => inner.span(),
            Self::TypeSpecifier(inner) => inner.span(),
        }
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum EnumeratorListChildren<'tree> {
    Enumerator(::std::boxed::Box<Enumerator<'tree>>),
    PreprocCall(::std::boxed::Box<PreprocCall<'tree>>),
    PreprocIf(::std::boxed::Box<PreprocIf<'tree>>),
    PreprocIfdef(::std::boxed::Box<PreprocIfdef<'tree>>),
}
impl<'tree> ::treesitter_types::FromNode<'tree> for EnumeratorListChildren<'tree> {
    #[allow(clippy::collapsible_else_if)]
    fn from_node(
        node: ::treesitter_types::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        match node.kind() {
            "enumerator" => Ok(Self::Enumerator(::std::boxed::Box::new(
                ::treesitter_types::runtime::maybe_grow_stack(|| {
                    <Enumerator as ::treesitter_types::FromNode>::from_node(node, src)
                })?,
            ))),
            "preproc_call" => Ok(Self::PreprocCall(::std::boxed::Box::new(
                ::treesitter_types::runtime::maybe_grow_stack(|| {
                    <PreprocCall as ::treesitter_types::FromNode>::from_node(node, src)
                })?,
            ))),
            "preproc_if" => Ok(Self::PreprocIf(::std::boxed::Box::new(
                ::treesitter_types::runtime::maybe_grow_stack(|| {
                    <PreprocIf as ::treesitter_types::FromNode>::from_node(node, src)
                })?,
            ))),
            "preproc_ifdef" => Ok(Self::PreprocIfdef(::std::boxed::Box::new(
                ::treesitter_types::runtime::maybe_grow_stack(|| {
                    <PreprocIfdef as ::treesitter_types::FromNode>::from_node(node, src)
                })?,
            ))),
            other => Err(::treesitter_types::ParseError::unexpected_kind(other, node)),
        }
    }
}
impl ::treesitter_types::Spanned for EnumeratorListChildren<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        match self {
            Self::Enumerator(inner) => inner.span(),
            Self::PreprocCall(inner) => inner.span(),
            Self::PreprocIf(inner) => inner.span(),
            Self::PreprocIfdef(inner) => inner.span(),
        }
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ExpressionStatementChildren<'tree> {
    CommaExpression(::std::boxed::Box<CommaExpression<'tree>>),
    Expression(::std::boxed::Box<Expression<'tree>>),
}
impl<'tree> ::treesitter_types::FromNode<'tree> for ExpressionStatementChildren<'tree> {
    #[allow(clippy::collapsible_else_if)]
    fn from_node(
        node: ::treesitter_types::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        match node.kind() {
            "comma_expression" => Ok(Self::CommaExpression(::std::boxed::Box::new(
                ::treesitter_types::runtime::maybe_grow_stack(|| {
                    <CommaExpression as ::treesitter_types::FromNode>::from_node(node, src)
                })?,
            ))),
            _other => {
                if let Ok(v) = ::treesitter_types::runtime::maybe_grow_stack(|| {
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
impl ::treesitter_types::Spanned for ExpressionStatementChildren<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        match self {
            Self::CommaExpression(inner) => inner.span(),
            Self::Expression(inner) => inner.span(),
        }
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum FieldDeclarationChildren<'tree> {
    AttributeDeclaration(::std::boxed::Box<AttributeDeclaration<'tree>>),
    AttributeSpecifier(::std::boxed::Box<AttributeSpecifier<'tree>>),
    BitfieldClause(::std::boxed::Box<BitfieldClause<'tree>>),
    MsDeclspecModifier(::std::boxed::Box<MsDeclspecModifier<'tree>>),
    StorageClassSpecifier(::std::boxed::Box<StorageClassSpecifier<'tree>>),
    TypeQualifier(::std::boxed::Box<TypeQualifier<'tree>>),
}
impl<'tree> ::treesitter_types::FromNode<'tree> for FieldDeclarationChildren<'tree> {
    #[allow(clippy::collapsible_else_if)]
    fn from_node(
        node: ::treesitter_types::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        match node.kind() {
            "attribute_declaration" => Ok(Self::AttributeDeclaration(::std::boxed::Box::new(
                ::treesitter_types::runtime::maybe_grow_stack(|| {
                    <AttributeDeclaration as ::treesitter_types::FromNode>::from_node(node, src)
                })?,
            ))),
            "attribute_specifier" => Ok(Self::AttributeSpecifier(::std::boxed::Box::new(
                ::treesitter_types::runtime::maybe_grow_stack(|| {
                    <AttributeSpecifier as ::treesitter_types::FromNode>::from_node(node, src)
                })?,
            ))),
            "bitfield_clause" => Ok(Self::BitfieldClause(::std::boxed::Box::new(
                ::treesitter_types::runtime::maybe_grow_stack(|| {
                    <BitfieldClause as ::treesitter_types::FromNode>::from_node(node, src)
                })?,
            ))),
            "ms_declspec_modifier" => Ok(Self::MsDeclspecModifier(::std::boxed::Box::new(
                ::treesitter_types::runtime::maybe_grow_stack(|| {
                    <MsDeclspecModifier as ::treesitter_types::FromNode>::from_node(node, src)
                })?,
            ))),
            "storage_class_specifier" => Ok(Self::StorageClassSpecifier(::std::boxed::Box::new(
                ::treesitter_types::runtime::maybe_grow_stack(|| {
                    <StorageClassSpecifier as ::treesitter_types::FromNode>::from_node(node, src)
                })?,
            ))),
            "type_qualifier" => Ok(Self::TypeQualifier(::std::boxed::Box::new(
                ::treesitter_types::runtime::maybe_grow_stack(|| {
                    <TypeQualifier as ::treesitter_types::FromNode>::from_node(node, src)
                })?,
            ))),
            other => Err(::treesitter_types::ParseError::unexpected_kind(other, node)),
        }
    }
}
impl ::treesitter_types::Spanned for FieldDeclarationChildren<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        match self {
            Self::AttributeDeclaration(inner) => inner.span(),
            Self::AttributeSpecifier(inner) => inner.span(),
            Self::BitfieldClause(inner) => inner.span(),
            Self::MsDeclspecModifier(inner) => inner.span(),
            Self::StorageClassSpecifier(inner) => inner.span(),
            Self::TypeQualifier(inner) => inner.span(),
        }
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum FieldDeclarationListChildren<'tree> {
    FieldDeclaration(::std::boxed::Box<FieldDeclaration<'tree>>),
    PreprocCall(::std::boxed::Box<PreprocCall<'tree>>),
    PreprocDef(::std::boxed::Box<PreprocDef<'tree>>),
    PreprocFunctionDef(::std::boxed::Box<PreprocFunctionDef<'tree>>),
    PreprocIf(::std::boxed::Box<PreprocIf<'tree>>),
    PreprocIfdef(::std::boxed::Box<PreprocIfdef<'tree>>),
}
impl<'tree> ::treesitter_types::FromNode<'tree> for FieldDeclarationListChildren<'tree> {
    #[allow(clippy::collapsible_else_if)]
    fn from_node(
        node: ::treesitter_types::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        match node.kind() {
            "field_declaration" => Ok(Self::FieldDeclaration(::std::boxed::Box::new(
                ::treesitter_types::runtime::maybe_grow_stack(|| {
                    <FieldDeclaration as ::treesitter_types::FromNode>::from_node(node, src)
                })?,
            ))),
            "preproc_call" => Ok(Self::PreprocCall(::std::boxed::Box::new(
                ::treesitter_types::runtime::maybe_grow_stack(|| {
                    <PreprocCall as ::treesitter_types::FromNode>::from_node(node, src)
                })?,
            ))),
            "preproc_def" => Ok(Self::PreprocDef(::std::boxed::Box::new(
                ::treesitter_types::runtime::maybe_grow_stack(|| {
                    <PreprocDef as ::treesitter_types::FromNode>::from_node(node, src)
                })?,
            ))),
            "preproc_function_def" => Ok(Self::PreprocFunctionDef(::std::boxed::Box::new(
                ::treesitter_types::runtime::maybe_grow_stack(|| {
                    <PreprocFunctionDef as ::treesitter_types::FromNode>::from_node(node, src)
                })?,
            ))),
            "preproc_if" => Ok(Self::PreprocIf(::std::boxed::Box::new(
                ::treesitter_types::runtime::maybe_grow_stack(|| {
                    <PreprocIf as ::treesitter_types::FromNode>::from_node(node, src)
                })?,
            ))),
            "preproc_ifdef" => Ok(Self::PreprocIfdef(::std::boxed::Box::new(
                ::treesitter_types::runtime::maybe_grow_stack(|| {
                    <PreprocIfdef as ::treesitter_types::FromNode>::from_node(node, src)
                })?,
            ))),
            other => Err(::treesitter_types::ParseError::unexpected_kind(other, node)),
        }
    }
}
impl ::treesitter_types::Spanned for FieldDeclarationListChildren<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        match self {
            Self::FieldDeclaration(inner) => inner.span(),
            Self::PreprocCall(inner) => inner.span(),
            Self::PreprocDef(inner) => inner.span(),
            Self::PreprocFunctionDef(inner) => inner.span(),
            Self::PreprocIf(inner) => inner.span(),
            Self::PreprocIfdef(inner) => inner.span(),
        }
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum FieldExpressionOperator {
    Arrow(::treesitter_types::Span),
    Dot(::treesitter_types::Span),
}
impl<'tree> ::treesitter_types::FromNode<'tree> for FieldExpressionOperator {
    #[allow(clippy::collapsible_else_if)]
    fn from_node(
        node: ::treesitter_types::tree_sitter::Node<'tree>,
        _src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        match node.kind() {
            "->" => Ok(Self::Arrow(::treesitter_types::Span::from(node))),
            "." => Ok(Self::Dot(::treesitter_types::Span::from(node))),
            other => Err(::treesitter_types::ParseError::unexpected_kind(other, node)),
        }
    }
}
impl ::treesitter_types::Spanned for FieldExpressionOperator {
    fn span(&self) -> ::treesitter_types::Span {
        match self {
            Self::Arrow(span) => *span,
            Self::Dot(span) => *span,
        }
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ForStatementCondition<'tree> {
    CommaExpression(::std::boxed::Box<CommaExpression<'tree>>),
    Expression(::std::boxed::Box<Expression<'tree>>),
}
impl<'tree> ::treesitter_types::FromNode<'tree> for ForStatementCondition<'tree> {
    #[allow(clippy::collapsible_else_if)]
    fn from_node(
        node: ::treesitter_types::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        match node.kind() {
            "comma_expression" => Ok(Self::CommaExpression(::std::boxed::Box::new(
                ::treesitter_types::runtime::maybe_grow_stack(|| {
                    <CommaExpression as ::treesitter_types::FromNode>::from_node(node, src)
                })?,
            ))),
            _other => {
                if let Ok(v) = ::treesitter_types::runtime::maybe_grow_stack(|| {
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
impl ::treesitter_types::Spanned for ForStatementCondition<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        match self {
            Self::CommaExpression(inner) => inner.span(),
            Self::Expression(inner) => inner.span(),
        }
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ForStatementInitializer<'tree> {
    CommaExpression(::std::boxed::Box<CommaExpression<'tree>>),
    Declaration(::std::boxed::Box<Declaration<'tree>>),
    Expression(::std::boxed::Box<Expression<'tree>>),
}
impl<'tree> ::treesitter_types::FromNode<'tree> for ForStatementInitializer<'tree> {
    #[allow(clippy::collapsible_else_if)]
    fn from_node(
        node: ::treesitter_types::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        match node.kind() {
            "comma_expression" => Ok(Self::CommaExpression(::std::boxed::Box::new(
                ::treesitter_types::runtime::maybe_grow_stack(|| {
                    <CommaExpression as ::treesitter_types::FromNode>::from_node(node, src)
                })?,
            ))),
            "declaration" => Ok(Self::Declaration(::std::boxed::Box::new(
                ::treesitter_types::runtime::maybe_grow_stack(|| {
                    <Declaration as ::treesitter_types::FromNode>::from_node(node, src)
                })?,
            ))),
            _other => {
                if let Ok(v) = ::treesitter_types::runtime::maybe_grow_stack(|| {
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
impl ::treesitter_types::Spanned for ForStatementInitializer<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        match self {
            Self::CommaExpression(inner) => inner.span(),
            Self::Declaration(inner) => inner.span(),
            Self::Expression(inner) => inner.span(),
        }
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ForStatementUpdate<'tree> {
    CommaExpression(::std::boxed::Box<CommaExpression<'tree>>),
    Expression(::std::boxed::Box<Expression<'tree>>),
}
impl<'tree> ::treesitter_types::FromNode<'tree> for ForStatementUpdate<'tree> {
    #[allow(clippy::collapsible_else_if)]
    fn from_node(
        node: ::treesitter_types::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        match node.kind() {
            "comma_expression" => Ok(Self::CommaExpression(::std::boxed::Box::new(
                ::treesitter_types::runtime::maybe_grow_stack(|| {
                    <CommaExpression as ::treesitter_types::FromNode>::from_node(node, src)
                })?,
            ))),
            _other => {
                if let Ok(v) = ::treesitter_types::runtime::maybe_grow_stack(|| {
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
impl ::treesitter_types::Spanned for ForStatementUpdate<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        match self {
            Self::CommaExpression(inner) => inner.span(),
            Self::Expression(inner) => inner.span(),
        }
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum FunctionDeclaratorDeclarator<'tree> {
    Declarator(::std::boxed::Box<Declarator<'tree>>),
    FieldDeclarator(::std::boxed::Box<FieldDeclarator<'tree>>),
    TypeDeclarator(::std::boxed::Box<TypeDeclarator<'tree>>),
}
impl<'tree> ::treesitter_types::FromNode<'tree> for FunctionDeclaratorDeclarator<'tree> {
    #[allow(clippy::collapsible_else_if)]
    fn from_node(
        node: ::treesitter_types::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        if let Ok(v) = ::treesitter_types::runtime::maybe_grow_stack(|| {
            <Declarator as ::treesitter_types::FromNode>::from_node(node, src)
        }) {
            Ok(Self::Declarator(::std::boxed::Box::new(v)))
        } else {
            if let Ok(v) = ::treesitter_types::runtime::maybe_grow_stack(|| {
                <FieldDeclarator as ::treesitter_types::FromNode>::from_node(node, src)
            }) {
                Ok(Self::FieldDeclarator(::std::boxed::Box::new(v)))
            } else {
                if let Ok(v) = ::treesitter_types::runtime::maybe_grow_stack(|| {
                    <TypeDeclarator as ::treesitter_types::FromNode>::from_node(node, src)
                }) {
                    Ok(Self::TypeDeclarator(::std::boxed::Box::new(v)))
                } else {
                    Err(::treesitter_types::ParseError::unexpected_kind(
                        node.kind(),
                        node,
                    ))
                }
            }
        }
    }
}
impl ::treesitter_types::Spanned for FunctionDeclaratorDeclarator<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        match self {
            Self::Declarator(inner) => inner.span(),
            Self::FieldDeclarator(inner) => inner.span(),
            Self::TypeDeclarator(inner) => inner.span(),
        }
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum FunctionDeclaratorChildren<'tree> {
    AttributeSpecifier(::std::boxed::Box<AttributeSpecifier<'tree>>),
    CallExpression(::std::boxed::Box<CallExpression<'tree>>),
    GnuAsmExpression(::std::boxed::Box<GnuAsmExpression<'tree>>),
    Identifier(::std::boxed::Box<Identifier<'tree>>),
}
impl<'tree> ::treesitter_types::FromNode<'tree> for FunctionDeclaratorChildren<'tree> {
    #[allow(clippy::collapsible_else_if)]
    fn from_node(
        node: ::treesitter_types::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        match node.kind() {
            "attribute_specifier" => Ok(Self::AttributeSpecifier(::std::boxed::Box::new(
                ::treesitter_types::runtime::maybe_grow_stack(|| {
                    <AttributeSpecifier as ::treesitter_types::FromNode>::from_node(node, src)
                })?,
            ))),
            "call_expression" => Ok(Self::CallExpression(::std::boxed::Box::new(
                ::treesitter_types::runtime::maybe_grow_stack(|| {
                    <CallExpression as ::treesitter_types::FromNode>::from_node(node, src)
                })?,
            ))),
            "gnu_asm_expression" => Ok(Self::GnuAsmExpression(::std::boxed::Box::new(
                ::treesitter_types::runtime::maybe_grow_stack(|| {
                    <GnuAsmExpression as ::treesitter_types::FromNode>::from_node(node, src)
                })?,
            ))),
            "identifier" => Ok(Self::Identifier(::std::boxed::Box::new(
                ::treesitter_types::runtime::maybe_grow_stack(|| {
                    <Identifier as ::treesitter_types::FromNode>::from_node(node, src)
                })?,
            ))),
            other => Err(::treesitter_types::ParseError::unexpected_kind(other, node)),
        }
    }
}
impl ::treesitter_types::Spanned for FunctionDeclaratorChildren<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        match self {
            Self::AttributeSpecifier(inner) => inner.span(),
            Self::CallExpression(inner) => inner.span(),
            Self::GnuAsmExpression(inner) => inner.span(),
            Self::Identifier(inner) => inner.span(),
        }
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum FunctionDefinitionChildren<'tree> {
    AttributeDeclaration(::std::boxed::Box<AttributeDeclaration<'tree>>),
    AttributeSpecifier(::std::boxed::Box<AttributeSpecifier<'tree>>),
    Declaration(::std::boxed::Box<Declaration<'tree>>),
    MsCallModifier(::std::boxed::Box<MsCallModifier<'tree>>),
    MsDeclspecModifier(::std::boxed::Box<MsDeclspecModifier<'tree>>),
    StorageClassSpecifier(::std::boxed::Box<StorageClassSpecifier<'tree>>),
    TypeQualifier(::std::boxed::Box<TypeQualifier<'tree>>),
}
impl<'tree> ::treesitter_types::FromNode<'tree> for FunctionDefinitionChildren<'tree> {
    #[allow(clippy::collapsible_else_if)]
    fn from_node(
        node: ::treesitter_types::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        match node.kind() {
            "attribute_declaration" => Ok(Self::AttributeDeclaration(::std::boxed::Box::new(
                ::treesitter_types::runtime::maybe_grow_stack(|| {
                    <AttributeDeclaration as ::treesitter_types::FromNode>::from_node(node, src)
                })?,
            ))),
            "attribute_specifier" => Ok(Self::AttributeSpecifier(::std::boxed::Box::new(
                ::treesitter_types::runtime::maybe_grow_stack(|| {
                    <AttributeSpecifier as ::treesitter_types::FromNode>::from_node(node, src)
                })?,
            ))),
            "declaration" => Ok(Self::Declaration(::std::boxed::Box::new(
                ::treesitter_types::runtime::maybe_grow_stack(|| {
                    <Declaration as ::treesitter_types::FromNode>::from_node(node, src)
                })?,
            ))),
            "ms_call_modifier" => Ok(Self::MsCallModifier(::std::boxed::Box::new(
                ::treesitter_types::runtime::maybe_grow_stack(|| {
                    <MsCallModifier as ::treesitter_types::FromNode>::from_node(node, src)
                })?,
            ))),
            "ms_declspec_modifier" => Ok(Self::MsDeclspecModifier(::std::boxed::Box::new(
                ::treesitter_types::runtime::maybe_grow_stack(|| {
                    <MsDeclspecModifier as ::treesitter_types::FromNode>::from_node(node, src)
                })?,
            ))),
            "storage_class_specifier" => Ok(Self::StorageClassSpecifier(::std::boxed::Box::new(
                ::treesitter_types::runtime::maybe_grow_stack(|| {
                    <StorageClassSpecifier as ::treesitter_types::FromNode>::from_node(node, src)
                })?,
            ))),
            "type_qualifier" => Ok(Self::TypeQualifier(::std::boxed::Box::new(
                ::treesitter_types::runtime::maybe_grow_stack(|| {
                    <TypeQualifier as ::treesitter_types::FromNode>::from_node(node, src)
                })?,
            ))),
            other => Err(::treesitter_types::ParseError::unexpected_kind(other, node)),
        }
    }
}
impl ::treesitter_types::Spanned for FunctionDefinitionChildren<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        match self {
            Self::AttributeDeclaration(inner) => inner.span(),
            Self::AttributeSpecifier(inner) => inner.span(),
            Self::Declaration(inner) => inner.span(),
            Self::MsCallModifier(inner) => inner.span(),
            Self::MsDeclspecModifier(inner) => inner.span(),
            Self::StorageClassSpecifier(inner) => inner.span(),
            Self::TypeQualifier(inner) => inner.span(),
        }
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum GenericExpressionChildren<'tree> {
    Expression(::std::boxed::Box<Expression<'tree>>),
    TypeDescriptor(::std::boxed::Box<TypeDescriptor<'tree>>),
}
impl<'tree> ::treesitter_types::FromNode<'tree> for GenericExpressionChildren<'tree> {
    #[allow(clippy::collapsible_else_if)]
    fn from_node(
        node: ::treesitter_types::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        match node.kind() {
            "type_descriptor" => Ok(Self::TypeDescriptor(::std::boxed::Box::new(
                ::treesitter_types::runtime::maybe_grow_stack(|| {
                    <TypeDescriptor as ::treesitter_types::FromNode>::from_node(node, src)
                })?,
            ))),
            _other => {
                if let Ok(v) = ::treesitter_types::runtime::maybe_grow_stack(|| {
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
impl ::treesitter_types::Spanned for GenericExpressionChildren<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        match self {
            Self::Expression(inner) => inner.span(),
            Self::TypeDescriptor(inner) => inner.span(),
        }
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum GnuAsmClobberListRegister<'tree> {
    ConcatenatedString(::std::boxed::Box<ConcatenatedString<'tree>>),
    StringLiteral(::std::boxed::Box<StringLiteral<'tree>>),
}
impl<'tree> ::treesitter_types::FromNode<'tree> for GnuAsmClobberListRegister<'tree> {
    #[allow(clippy::collapsible_else_if)]
    fn from_node(
        node: ::treesitter_types::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        match node.kind() {
            "concatenated_string" => Ok(Self::ConcatenatedString(::std::boxed::Box::new(
                ::treesitter_types::runtime::maybe_grow_stack(|| {
                    <ConcatenatedString as ::treesitter_types::FromNode>::from_node(node, src)
                })?,
            ))),
            "string_literal" => Ok(Self::StringLiteral(::std::boxed::Box::new(
                ::treesitter_types::runtime::maybe_grow_stack(|| {
                    <StringLiteral as ::treesitter_types::FromNode>::from_node(node, src)
                })?,
            ))),
            other => Err(::treesitter_types::ParseError::unexpected_kind(other, node)),
        }
    }
}
impl ::treesitter_types::Spanned for GnuAsmClobberListRegister<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        match self {
            Self::ConcatenatedString(inner) => inner.span(),
            Self::StringLiteral(inner) => inner.span(),
        }
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum GnuAsmExpressionAssemblyCode<'tree> {
    ConcatenatedString(::std::boxed::Box<ConcatenatedString<'tree>>),
    StringLiteral(::std::boxed::Box<StringLiteral<'tree>>),
}
impl<'tree> ::treesitter_types::FromNode<'tree> for GnuAsmExpressionAssemblyCode<'tree> {
    #[allow(clippy::collapsible_else_if)]
    fn from_node(
        node: ::treesitter_types::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        match node.kind() {
            "concatenated_string" => Ok(Self::ConcatenatedString(::std::boxed::Box::new(
                ::treesitter_types::runtime::maybe_grow_stack(|| {
                    <ConcatenatedString as ::treesitter_types::FromNode>::from_node(node, src)
                })?,
            ))),
            "string_literal" => Ok(Self::StringLiteral(::std::boxed::Box::new(
                ::treesitter_types::runtime::maybe_grow_stack(|| {
                    <StringLiteral as ::treesitter_types::FromNode>::from_node(node, src)
                })?,
            ))),
            other => Err(::treesitter_types::ParseError::unexpected_kind(other, node)),
        }
    }
}
impl ::treesitter_types::Spanned for GnuAsmExpressionAssemblyCode<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        match self {
            Self::ConcatenatedString(inner) => inner.span(),
            Self::StringLiteral(inner) => inner.span(),
        }
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum InitDeclaratorValue<'tree> {
    Expression(::std::boxed::Box<Expression<'tree>>),
    InitializerList(::std::boxed::Box<InitializerList<'tree>>),
}
impl<'tree> ::treesitter_types::FromNode<'tree> for InitDeclaratorValue<'tree> {
    #[allow(clippy::collapsible_else_if)]
    fn from_node(
        node: ::treesitter_types::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        match node.kind() {
            "initializer_list" => Ok(Self::InitializerList(::std::boxed::Box::new(
                ::treesitter_types::runtime::maybe_grow_stack(|| {
                    <InitializerList as ::treesitter_types::FromNode>::from_node(node, src)
                })?,
            ))),
            _other => {
                if let Ok(v) = ::treesitter_types::runtime::maybe_grow_stack(|| {
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
impl ::treesitter_types::Spanned for InitDeclaratorValue<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        match self {
            Self::Expression(inner) => inner.span(),
            Self::InitializerList(inner) => inner.span(),
        }
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum InitializerListChildren<'tree> {
    Expression(::std::boxed::Box<Expression<'tree>>),
    InitializerList(::std::boxed::Box<InitializerList<'tree>>),
    InitializerPair(::std::boxed::Box<InitializerPair<'tree>>),
}
impl<'tree> ::treesitter_types::FromNode<'tree> for InitializerListChildren<'tree> {
    #[allow(clippy::collapsible_else_if)]
    fn from_node(
        node: ::treesitter_types::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        match node.kind() {
            "initializer_list" => Ok(Self::InitializerList(::std::boxed::Box::new(
                ::treesitter_types::runtime::maybe_grow_stack(|| {
                    <InitializerList as ::treesitter_types::FromNode>::from_node(node, src)
                })?,
            ))),
            "initializer_pair" => Ok(Self::InitializerPair(::std::boxed::Box::new(
                ::treesitter_types::runtime::maybe_grow_stack(|| {
                    <InitializerPair as ::treesitter_types::FromNode>::from_node(node, src)
                })?,
            ))),
            _other => {
                if let Ok(v) = ::treesitter_types::runtime::maybe_grow_stack(|| {
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
impl ::treesitter_types::Spanned for InitializerListChildren<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        match self {
            Self::Expression(inner) => inner.span(),
            Self::InitializerList(inner) => inner.span(),
            Self::InitializerPair(inner) => inner.span(),
        }
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum InitializerPairDesignator<'tree> {
    FieldDesignator(::std::boxed::Box<FieldDesignator<'tree>>),
    FieldIdentifier(::std::boxed::Box<FieldIdentifier<'tree>>),
    SubscriptDesignator(::std::boxed::Box<SubscriptDesignator<'tree>>),
    SubscriptRangeDesignator(::std::boxed::Box<SubscriptRangeDesignator<'tree>>),
}
impl<'tree> ::treesitter_types::FromNode<'tree> for InitializerPairDesignator<'tree> {
    #[allow(clippy::collapsible_else_if)]
    fn from_node(
        node: ::treesitter_types::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        match node.kind() {
            "field_designator" => Ok(Self::FieldDesignator(::std::boxed::Box::new(
                ::treesitter_types::runtime::maybe_grow_stack(|| {
                    <FieldDesignator as ::treesitter_types::FromNode>::from_node(node, src)
                })?,
            ))),
            "field_identifier" => Ok(Self::FieldIdentifier(::std::boxed::Box::new(
                ::treesitter_types::runtime::maybe_grow_stack(|| {
                    <FieldIdentifier as ::treesitter_types::FromNode>::from_node(node, src)
                })?,
            ))),
            "subscript_designator" => Ok(Self::SubscriptDesignator(::std::boxed::Box::new(
                ::treesitter_types::runtime::maybe_grow_stack(|| {
                    <SubscriptDesignator as ::treesitter_types::FromNode>::from_node(node, src)
                })?,
            ))),
            "subscript_range_designator" => Ok(Self::SubscriptRangeDesignator(
                ::std::boxed::Box::new(::treesitter_types::runtime::maybe_grow_stack(|| {
                    <SubscriptRangeDesignator as ::treesitter_types::FromNode>::from_node(node, src)
                })?),
            )),
            other => Err(::treesitter_types::ParseError::unexpected_kind(other, node)),
        }
    }
}
impl ::treesitter_types::Spanned for InitializerPairDesignator<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        match self {
            Self::FieldDesignator(inner) => inner.span(),
            Self::FieldIdentifier(inner) => inner.span(),
            Self::SubscriptDesignator(inner) => inner.span(),
            Self::SubscriptRangeDesignator(inner) => inner.span(),
        }
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum InitializerPairValue<'tree> {
    Expression(::std::boxed::Box<Expression<'tree>>),
    InitializerList(::std::boxed::Box<InitializerList<'tree>>),
}
impl<'tree> ::treesitter_types::FromNode<'tree> for InitializerPairValue<'tree> {
    #[allow(clippy::collapsible_else_if)]
    fn from_node(
        node: ::treesitter_types::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        match node.kind() {
            "initializer_list" => Ok(Self::InitializerList(::std::boxed::Box::new(
                ::treesitter_types::runtime::maybe_grow_stack(|| {
                    <InitializerList as ::treesitter_types::FromNode>::from_node(node, src)
                })?,
            ))),
            _other => {
                if let Ok(v) = ::treesitter_types::runtime::maybe_grow_stack(|| {
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
impl ::treesitter_types::Spanned for InitializerPairValue<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        match self {
            Self::Expression(inner) => inner.span(),
            Self::InitializerList(inner) => inner.span(),
        }
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum LabeledStatementChildren<'tree> {
    Declaration(::std::boxed::Box<Declaration<'tree>>),
    Statement(::std::boxed::Box<Statement<'tree>>),
}
impl<'tree> ::treesitter_types::FromNode<'tree> for LabeledStatementChildren<'tree> {
    #[allow(clippy::collapsible_else_if)]
    fn from_node(
        node: ::treesitter_types::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        match node.kind() {
            "declaration" => Ok(Self::Declaration(::std::boxed::Box::new(
                ::treesitter_types::runtime::maybe_grow_stack(|| {
                    <Declaration as ::treesitter_types::FromNode>::from_node(node, src)
                })?,
            ))),
            _other => {
                if let Ok(v) = ::treesitter_types::runtime::maybe_grow_stack(|| {
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
impl ::treesitter_types::Spanned for LabeledStatementChildren<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        match self {
            Self::Declaration(inner) => inner.span(),
            Self::Statement(inner) => inner.span(),
        }
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum LinkageSpecificationBody<'tree> {
    Declaration(::std::boxed::Box<Declaration<'tree>>),
    DeclarationList(::std::boxed::Box<DeclarationList<'tree>>),
    FunctionDefinition(::std::boxed::Box<FunctionDefinition<'tree>>),
}
impl<'tree> ::treesitter_types::FromNode<'tree> for LinkageSpecificationBody<'tree> {
    #[allow(clippy::collapsible_else_if)]
    fn from_node(
        node: ::treesitter_types::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        match node.kind() {
            "declaration" => Ok(Self::Declaration(::std::boxed::Box::new(
                ::treesitter_types::runtime::maybe_grow_stack(|| {
                    <Declaration as ::treesitter_types::FromNode>::from_node(node, src)
                })?,
            ))),
            "declaration_list" => Ok(Self::DeclarationList(::std::boxed::Box::new(
                ::treesitter_types::runtime::maybe_grow_stack(|| {
                    <DeclarationList as ::treesitter_types::FromNode>::from_node(node, src)
                })?,
            ))),
            "function_definition" => Ok(Self::FunctionDefinition(::std::boxed::Box::new(
                ::treesitter_types::runtime::maybe_grow_stack(|| {
                    <FunctionDefinition as ::treesitter_types::FromNode>::from_node(node, src)
                })?,
            ))),
            other => Err(::treesitter_types::ParseError::unexpected_kind(other, node)),
        }
    }
}
impl ::treesitter_types::Spanned for LinkageSpecificationBody<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        match self {
            Self::Declaration(inner) => inner.span(),
            Self::DeclarationList(inner) => inner.span(),
            Self::FunctionDefinition(inner) => inner.span(),
        }
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum MsPointerModifierChildren<'tree> {
    MsRestrictModifier(::std::boxed::Box<MsRestrictModifier<'tree>>),
    MsSignedPtrModifier(::std::boxed::Box<MsSignedPtrModifier<'tree>>),
    MsUnalignedPtrModifier(::std::boxed::Box<MsUnalignedPtrModifier<'tree>>),
    MsUnsignedPtrModifier(::std::boxed::Box<MsUnsignedPtrModifier<'tree>>),
}
impl<'tree> ::treesitter_types::FromNode<'tree> for MsPointerModifierChildren<'tree> {
    #[allow(clippy::collapsible_else_if)]
    fn from_node(
        node: ::treesitter_types::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        match node.kind() {
            "ms_restrict_modifier" => Ok(Self::MsRestrictModifier(::std::boxed::Box::new(
                ::treesitter_types::runtime::maybe_grow_stack(|| {
                    <MsRestrictModifier as ::treesitter_types::FromNode>::from_node(node, src)
                })?,
            ))),
            "ms_signed_ptr_modifier" => Ok(Self::MsSignedPtrModifier(::std::boxed::Box::new(
                ::treesitter_types::runtime::maybe_grow_stack(|| {
                    <MsSignedPtrModifier as ::treesitter_types::FromNode>::from_node(node, src)
                })?,
            ))),
            "ms_unaligned_ptr_modifier" => Ok(Self::MsUnalignedPtrModifier(
                ::std::boxed::Box::new(::treesitter_types::runtime::maybe_grow_stack(|| {
                    <MsUnalignedPtrModifier as ::treesitter_types::FromNode>::from_node(node, src)
                })?),
            )),
            "ms_unsigned_ptr_modifier" => Ok(Self::MsUnsignedPtrModifier(::std::boxed::Box::new(
                ::treesitter_types::runtime::maybe_grow_stack(|| {
                    <MsUnsignedPtrModifier as ::treesitter_types::FromNode>::from_node(node, src)
                })?,
            ))),
            other => Err(::treesitter_types::ParseError::unexpected_kind(other, node)),
        }
    }
}
impl ::treesitter_types::Spanned for MsPointerModifierChildren<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        match self {
            Self::MsRestrictModifier(inner) => inner.span(),
            Self::MsSignedPtrModifier(inner) => inner.span(),
            Self::MsUnalignedPtrModifier(inner) => inner.span(),
            Self::MsUnsignedPtrModifier(inner) => inner.span(),
        }
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ParameterDeclarationDeclarator<'tree> {
    AbstractDeclarator(::std::boxed::Box<AbstractDeclarator<'tree>>),
    Declarator(::std::boxed::Box<Declarator<'tree>>),
}
impl<'tree> ::treesitter_types::FromNode<'tree> for ParameterDeclarationDeclarator<'tree> {
    #[allow(clippy::collapsible_else_if)]
    fn from_node(
        node: ::treesitter_types::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        if let Ok(v) = ::treesitter_types::runtime::maybe_grow_stack(|| {
            <AbstractDeclarator as ::treesitter_types::FromNode>::from_node(node, src)
        }) {
            Ok(Self::AbstractDeclarator(::std::boxed::Box::new(v)))
        } else {
            if let Ok(v) = ::treesitter_types::runtime::maybe_grow_stack(|| {
                <Declarator as ::treesitter_types::FromNode>::from_node(node, src)
            }) {
                Ok(Self::Declarator(::std::boxed::Box::new(v)))
            } else {
                Err(::treesitter_types::ParseError::unexpected_kind(
                    node.kind(),
                    node,
                ))
            }
        }
    }
}
impl ::treesitter_types::Spanned for ParameterDeclarationDeclarator<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        match self {
            Self::AbstractDeclarator(inner) => inner.span(),
            Self::Declarator(inner) => inner.span(),
        }
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ParameterDeclarationChildren<'tree> {
    AttributeDeclaration(::std::boxed::Box<AttributeDeclaration<'tree>>),
    AttributeSpecifier(::std::boxed::Box<AttributeSpecifier<'tree>>),
    MsDeclspecModifier(::std::boxed::Box<MsDeclspecModifier<'tree>>),
    StorageClassSpecifier(::std::boxed::Box<StorageClassSpecifier<'tree>>),
    TypeQualifier(::std::boxed::Box<TypeQualifier<'tree>>),
}
impl<'tree> ::treesitter_types::FromNode<'tree> for ParameterDeclarationChildren<'tree> {
    #[allow(clippy::collapsible_else_if)]
    fn from_node(
        node: ::treesitter_types::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        match node.kind() {
            "attribute_declaration" => Ok(Self::AttributeDeclaration(::std::boxed::Box::new(
                ::treesitter_types::runtime::maybe_grow_stack(|| {
                    <AttributeDeclaration as ::treesitter_types::FromNode>::from_node(node, src)
                })?,
            ))),
            "attribute_specifier" => Ok(Self::AttributeSpecifier(::std::boxed::Box::new(
                ::treesitter_types::runtime::maybe_grow_stack(|| {
                    <AttributeSpecifier as ::treesitter_types::FromNode>::from_node(node, src)
                })?,
            ))),
            "ms_declspec_modifier" => Ok(Self::MsDeclspecModifier(::std::boxed::Box::new(
                ::treesitter_types::runtime::maybe_grow_stack(|| {
                    <MsDeclspecModifier as ::treesitter_types::FromNode>::from_node(node, src)
                })?,
            ))),
            "storage_class_specifier" => Ok(Self::StorageClassSpecifier(::std::boxed::Box::new(
                ::treesitter_types::runtime::maybe_grow_stack(|| {
                    <StorageClassSpecifier as ::treesitter_types::FromNode>::from_node(node, src)
                })?,
            ))),
            "type_qualifier" => Ok(Self::TypeQualifier(::std::boxed::Box::new(
                ::treesitter_types::runtime::maybe_grow_stack(|| {
                    <TypeQualifier as ::treesitter_types::FromNode>::from_node(node, src)
                })?,
            ))),
            other => Err(::treesitter_types::ParseError::unexpected_kind(other, node)),
        }
    }
}
impl ::treesitter_types::Spanned for ParameterDeclarationChildren<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        match self {
            Self::AttributeDeclaration(inner) => inner.span(),
            Self::AttributeSpecifier(inner) => inner.span(),
            Self::MsDeclspecModifier(inner) => inner.span(),
            Self::StorageClassSpecifier(inner) => inner.span(),
            Self::TypeQualifier(inner) => inner.span(),
        }
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ParameterListChildren<'tree> {
    CompoundStatement(::std::boxed::Box<CompoundStatement<'tree>>),
    Identifier(::std::boxed::Box<Identifier<'tree>>),
    ParameterDeclaration(::std::boxed::Box<ParameterDeclaration<'tree>>),
    VariadicParameter(::std::boxed::Box<VariadicParameter<'tree>>),
}
impl<'tree> ::treesitter_types::FromNode<'tree> for ParameterListChildren<'tree> {
    #[allow(clippy::collapsible_else_if)]
    fn from_node(
        node: ::treesitter_types::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        match node.kind() {
            "compound_statement" => Ok(Self::CompoundStatement(::std::boxed::Box::new(
                ::treesitter_types::runtime::maybe_grow_stack(|| {
                    <CompoundStatement as ::treesitter_types::FromNode>::from_node(node, src)
                })?,
            ))),
            "identifier" => Ok(Self::Identifier(::std::boxed::Box::new(
                ::treesitter_types::runtime::maybe_grow_stack(|| {
                    <Identifier as ::treesitter_types::FromNode>::from_node(node, src)
                })?,
            ))),
            "parameter_declaration" => Ok(Self::ParameterDeclaration(::std::boxed::Box::new(
                ::treesitter_types::runtime::maybe_grow_stack(|| {
                    <ParameterDeclaration as ::treesitter_types::FromNode>::from_node(node, src)
                })?,
            ))),
            "variadic_parameter" => Ok(Self::VariadicParameter(::std::boxed::Box::new(
                ::treesitter_types::runtime::maybe_grow_stack(|| {
                    <VariadicParameter as ::treesitter_types::FromNode>::from_node(node, src)
                })?,
            ))),
            other => Err(::treesitter_types::ParseError::unexpected_kind(other, node)),
        }
    }
}
impl ::treesitter_types::Spanned for ParameterListChildren<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        match self {
            Self::CompoundStatement(inner) => inner.span(),
            Self::Identifier(inner) => inner.span(),
            Self::ParameterDeclaration(inner) => inner.span(),
            Self::VariadicParameter(inner) => inner.span(),
        }
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ParenthesizedDeclaratorChildren<'tree> {
    Declarator(::std::boxed::Box<Declarator<'tree>>),
    FieldDeclarator(::std::boxed::Box<FieldDeclarator<'tree>>),
    TypeDeclarator(::std::boxed::Box<TypeDeclarator<'tree>>),
    MsCallModifier(::std::boxed::Box<MsCallModifier<'tree>>),
}
impl<'tree> ::treesitter_types::FromNode<'tree> for ParenthesizedDeclaratorChildren<'tree> {
    #[allow(clippy::collapsible_else_if)]
    fn from_node(
        node: ::treesitter_types::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        match node.kind() {
            "ms_call_modifier" => Ok(Self::MsCallModifier(::std::boxed::Box::new(
                ::treesitter_types::runtime::maybe_grow_stack(|| {
                    <MsCallModifier as ::treesitter_types::FromNode>::from_node(node, src)
                })?,
            ))),
            _other => {
                if let Ok(v) = ::treesitter_types::runtime::maybe_grow_stack(|| {
                    <Declarator as ::treesitter_types::FromNode>::from_node(node, src)
                }) {
                    Ok(Self::Declarator(::std::boxed::Box::new(v)))
                } else {
                    if let Ok(v) = ::treesitter_types::runtime::maybe_grow_stack(|| {
                        <FieldDeclarator as ::treesitter_types::FromNode>::from_node(node, src)
                    }) {
                        Ok(Self::FieldDeclarator(::std::boxed::Box::new(v)))
                    } else {
                        if let Ok(v) = ::treesitter_types::runtime::maybe_grow_stack(|| {
                            <TypeDeclarator as ::treesitter_types::FromNode>::from_node(node, src)
                        }) {
                            Ok(Self::TypeDeclarator(::std::boxed::Box::new(v)))
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
}
impl ::treesitter_types::Spanned for ParenthesizedDeclaratorChildren<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        match self {
            Self::Declarator(inner) => inner.span(),
            Self::FieldDeclarator(inner) => inner.span(),
            Self::TypeDeclarator(inner) => inner.span(),
            Self::MsCallModifier(inner) => inner.span(),
        }
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ParenthesizedExpressionChildren<'tree> {
    CommaExpression(::std::boxed::Box<CommaExpression<'tree>>),
    CompoundStatement(::std::boxed::Box<CompoundStatement<'tree>>),
    Expression(::std::boxed::Box<Expression<'tree>>),
    PreprocDefined(::std::boxed::Box<PreprocDefined<'tree>>),
}
impl<'tree> ::treesitter_types::FromNode<'tree> for ParenthesizedExpressionChildren<'tree> {
    #[allow(clippy::collapsible_else_if)]
    fn from_node(
        node: ::treesitter_types::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        match node.kind() {
            "comma_expression" => Ok(Self::CommaExpression(::std::boxed::Box::new(
                ::treesitter_types::runtime::maybe_grow_stack(|| {
                    <CommaExpression as ::treesitter_types::FromNode>::from_node(node, src)
                })?,
            ))),
            "compound_statement" => Ok(Self::CompoundStatement(::std::boxed::Box::new(
                ::treesitter_types::runtime::maybe_grow_stack(|| {
                    <CompoundStatement as ::treesitter_types::FromNode>::from_node(node, src)
                })?,
            ))),
            "preproc_defined" => Ok(Self::PreprocDefined(::std::boxed::Box::new(
                ::treesitter_types::runtime::maybe_grow_stack(|| {
                    <PreprocDefined as ::treesitter_types::FromNode>::from_node(node, src)
                })?,
            ))),
            _other => {
                if let Ok(v) = ::treesitter_types::runtime::maybe_grow_stack(|| {
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
impl ::treesitter_types::Spanned for ParenthesizedExpressionChildren<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        match self {
            Self::CommaExpression(inner) => inner.span(),
            Self::CompoundStatement(inner) => inner.span(),
            Self::Expression(inner) => inner.span(),
            Self::PreprocDefined(inner) => inner.span(),
        }
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum PointerDeclaratorDeclarator<'tree> {
    Declarator(::std::boxed::Box<Declarator<'tree>>),
    FieldDeclarator(::std::boxed::Box<FieldDeclarator<'tree>>),
    TypeDeclarator(::std::boxed::Box<TypeDeclarator<'tree>>),
}
impl<'tree> ::treesitter_types::FromNode<'tree> for PointerDeclaratorDeclarator<'tree> {
    #[allow(clippy::collapsible_else_if)]
    fn from_node(
        node: ::treesitter_types::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        if let Ok(v) = ::treesitter_types::runtime::maybe_grow_stack(|| {
            <Declarator as ::treesitter_types::FromNode>::from_node(node, src)
        }) {
            Ok(Self::Declarator(::std::boxed::Box::new(v)))
        } else {
            if let Ok(v) = ::treesitter_types::runtime::maybe_grow_stack(|| {
                <FieldDeclarator as ::treesitter_types::FromNode>::from_node(node, src)
            }) {
                Ok(Self::FieldDeclarator(::std::boxed::Box::new(v)))
            } else {
                if let Ok(v) = ::treesitter_types::runtime::maybe_grow_stack(|| {
                    <TypeDeclarator as ::treesitter_types::FromNode>::from_node(node, src)
                }) {
                    Ok(Self::TypeDeclarator(::std::boxed::Box::new(v)))
                } else {
                    Err(::treesitter_types::ParseError::unexpected_kind(
                        node.kind(),
                        node,
                    ))
                }
            }
        }
    }
}
impl ::treesitter_types::Spanned for PointerDeclaratorDeclarator<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        match self {
            Self::Declarator(inner) => inner.span(),
            Self::FieldDeclarator(inner) => inner.span(),
            Self::TypeDeclarator(inner) => inner.span(),
        }
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum PointerDeclaratorChildren<'tree> {
    MsBasedModifier(::std::boxed::Box<MsBasedModifier<'tree>>),
    MsPointerModifier(::std::boxed::Box<MsPointerModifier<'tree>>),
    TypeQualifier(::std::boxed::Box<TypeQualifier<'tree>>),
}
impl<'tree> ::treesitter_types::FromNode<'tree> for PointerDeclaratorChildren<'tree> {
    #[allow(clippy::collapsible_else_if)]
    fn from_node(
        node: ::treesitter_types::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        match node.kind() {
            "ms_based_modifier" => Ok(Self::MsBasedModifier(::std::boxed::Box::new(
                ::treesitter_types::runtime::maybe_grow_stack(|| {
                    <MsBasedModifier as ::treesitter_types::FromNode>::from_node(node, src)
                })?,
            ))),
            "ms_pointer_modifier" => Ok(Self::MsPointerModifier(::std::boxed::Box::new(
                ::treesitter_types::runtime::maybe_grow_stack(|| {
                    <MsPointerModifier as ::treesitter_types::FromNode>::from_node(node, src)
                })?,
            ))),
            "type_qualifier" => Ok(Self::TypeQualifier(::std::boxed::Box::new(
                ::treesitter_types::runtime::maybe_grow_stack(|| {
                    <TypeQualifier as ::treesitter_types::FromNode>::from_node(node, src)
                })?,
            ))),
            other => Err(::treesitter_types::ParseError::unexpected_kind(other, node)),
        }
    }
}
impl ::treesitter_types::Spanned for PointerDeclaratorChildren<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        match self {
            Self::MsBasedModifier(inner) => inner.span(),
            Self::MsPointerModifier(inner) => inner.span(),
            Self::TypeQualifier(inner) => inner.span(),
        }
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum PointerExpressionOperator {
    Amp(::treesitter_types::Span),
    Star(::treesitter_types::Span),
}
impl<'tree> ::treesitter_types::FromNode<'tree> for PointerExpressionOperator {
    #[allow(clippy::collapsible_else_if)]
    fn from_node(
        node: ::treesitter_types::tree_sitter::Node<'tree>,
        _src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        match node.kind() {
            "&" => Ok(Self::Amp(::treesitter_types::Span::from(node))),
            "*" => Ok(Self::Star(::treesitter_types::Span::from(node))),
            other => Err(::treesitter_types::ParseError::unexpected_kind(other, node)),
        }
    }
}
impl ::treesitter_types::Spanned for PointerExpressionOperator {
    fn span(&self) -> ::treesitter_types::Span {
        match self {
            Self::Amp(span) => *span,
            Self::Star(span) => *span,
        }
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum PreprocElifAlternative<'tree> {
    PreprocElif(::std::boxed::Box<PreprocElif<'tree>>),
    PreprocElifdef(::std::boxed::Box<PreprocElifdef<'tree>>),
    PreprocElse(::std::boxed::Box<PreprocElse<'tree>>),
}
impl<'tree> ::treesitter_types::FromNode<'tree> for PreprocElifAlternative<'tree> {
    #[allow(clippy::collapsible_else_if)]
    fn from_node(
        node: ::treesitter_types::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        match node.kind() {
            "preproc_elif" => Ok(Self::PreprocElif(::std::boxed::Box::new(
                ::treesitter_types::runtime::maybe_grow_stack(|| {
                    <PreprocElif as ::treesitter_types::FromNode>::from_node(node, src)
                })?,
            ))),
            "preproc_elifdef" => Ok(Self::PreprocElifdef(::std::boxed::Box::new(
                ::treesitter_types::runtime::maybe_grow_stack(|| {
                    <PreprocElifdef as ::treesitter_types::FromNode>::from_node(node, src)
                })?,
            ))),
            "preproc_else" => Ok(Self::PreprocElse(::std::boxed::Box::new(
                ::treesitter_types::runtime::maybe_grow_stack(|| {
                    <PreprocElse as ::treesitter_types::FromNode>::from_node(node, src)
                })?,
            ))),
            other => Err(::treesitter_types::ParseError::unexpected_kind(other, node)),
        }
    }
}
impl ::treesitter_types::Spanned for PreprocElifAlternative<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        match self {
            Self::PreprocElif(inner) => inner.span(),
            Self::PreprocElifdef(inner) => inner.span(),
            Self::PreprocElse(inner) => inner.span(),
        }
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum PreprocElifCondition<'tree> {
    BinaryExpression(::std::boxed::Box<BinaryExpression<'tree>>),
    CallExpression(::std::boxed::Box<CallExpression<'tree>>),
    CharLiteral(::std::boxed::Box<CharLiteral<'tree>>),
    Identifier(::std::boxed::Box<Identifier<'tree>>),
    NumberLiteral(::std::boxed::Box<NumberLiteral<'tree>>),
    ParenthesizedExpression(::std::boxed::Box<ParenthesizedExpression<'tree>>),
    PreprocDefined(::std::boxed::Box<PreprocDefined<'tree>>),
    UnaryExpression(::std::boxed::Box<UnaryExpression<'tree>>),
}
impl<'tree> ::treesitter_types::FromNode<'tree> for PreprocElifCondition<'tree> {
    #[allow(clippy::collapsible_else_if)]
    fn from_node(
        node: ::treesitter_types::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        match node.kind() {
            "binary_expression" => Ok(Self::BinaryExpression(::std::boxed::Box::new(
                ::treesitter_types::runtime::maybe_grow_stack(|| {
                    <BinaryExpression as ::treesitter_types::FromNode>::from_node(node, src)
                })?,
            ))),
            "call_expression" => Ok(Self::CallExpression(::std::boxed::Box::new(
                ::treesitter_types::runtime::maybe_grow_stack(|| {
                    <CallExpression as ::treesitter_types::FromNode>::from_node(node, src)
                })?,
            ))),
            "char_literal" => Ok(Self::CharLiteral(::std::boxed::Box::new(
                ::treesitter_types::runtime::maybe_grow_stack(|| {
                    <CharLiteral as ::treesitter_types::FromNode>::from_node(node, src)
                })?,
            ))),
            "identifier" => Ok(Self::Identifier(::std::boxed::Box::new(
                ::treesitter_types::runtime::maybe_grow_stack(|| {
                    <Identifier as ::treesitter_types::FromNode>::from_node(node, src)
                })?,
            ))),
            "number_literal" => Ok(Self::NumberLiteral(::std::boxed::Box::new(
                ::treesitter_types::runtime::maybe_grow_stack(|| {
                    <NumberLiteral as ::treesitter_types::FromNode>::from_node(node, src)
                })?,
            ))),
            "parenthesized_expression" => Ok(Self::ParenthesizedExpression(
                ::std::boxed::Box::new(::treesitter_types::runtime::maybe_grow_stack(|| {
                    <ParenthesizedExpression as ::treesitter_types::FromNode>::from_node(node, src)
                })?),
            )),
            "preproc_defined" => Ok(Self::PreprocDefined(::std::boxed::Box::new(
                ::treesitter_types::runtime::maybe_grow_stack(|| {
                    <PreprocDefined as ::treesitter_types::FromNode>::from_node(node, src)
                })?,
            ))),
            "unary_expression" => Ok(Self::UnaryExpression(::std::boxed::Box::new(
                ::treesitter_types::runtime::maybe_grow_stack(|| {
                    <UnaryExpression as ::treesitter_types::FromNode>::from_node(node, src)
                })?,
            ))),
            other => Err(::treesitter_types::ParseError::unexpected_kind(other, node)),
        }
    }
}
impl ::treesitter_types::Spanned for PreprocElifCondition<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        match self {
            Self::BinaryExpression(inner) => inner.span(),
            Self::CallExpression(inner) => inner.span(),
            Self::CharLiteral(inner) => inner.span(),
            Self::Identifier(inner) => inner.span(),
            Self::NumberLiteral(inner) => inner.span(),
            Self::ParenthesizedExpression(inner) => inner.span(),
            Self::PreprocDefined(inner) => inner.span(),
            Self::UnaryExpression(inner) => inner.span(),
        }
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum PreprocElifChildren<'tree> {
    Declaration(::std::boxed::Box<Declaration<'tree>>),
    Enumerator(::std::boxed::Box<Enumerator<'tree>>),
    FieldDeclaration(::std::boxed::Box<FieldDeclaration<'tree>>),
    FunctionDefinition(::std::boxed::Box<FunctionDefinition<'tree>>),
    LinkageSpecification(::std::boxed::Box<LinkageSpecification<'tree>>),
    PreprocCall(::std::boxed::Box<PreprocCall<'tree>>),
    PreprocDef(::std::boxed::Box<PreprocDef<'tree>>),
    PreprocFunctionDef(::std::boxed::Box<PreprocFunctionDef<'tree>>),
    PreprocIf(::std::boxed::Box<PreprocIf<'tree>>),
    PreprocIfdef(::std::boxed::Box<PreprocIfdef<'tree>>),
    PreprocInclude(::std::boxed::Box<PreprocInclude<'tree>>),
    Statement(::std::boxed::Box<Statement<'tree>>),
    TypeDefinition(::std::boxed::Box<TypeDefinition<'tree>>),
    TypeSpecifier(::std::boxed::Box<TypeSpecifier<'tree>>),
}
impl<'tree> ::treesitter_types::FromNode<'tree> for PreprocElifChildren<'tree> {
    #[allow(clippy::collapsible_else_if)]
    fn from_node(
        node: ::treesitter_types::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        match node.kind() {
            "declaration" => Ok(Self::Declaration(::std::boxed::Box::new(
                ::treesitter_types::runtime::maybe_grow_stack(|| {
                    <Declaration as ::treesitter_types::FromNode>::from_node(node, src)
                })?,
            ))),
            "enumerator" => Ok(Self::Enumerator(::std::boxed::Box::new(
                ::treesitter_types::runtime::maybe_grow_stack(|| {
                    <Enumerator as ::treesitter_types::FromNode>::from_node(node, src)
                })?,
            ))),
            "field_declaration" => Ok(Self::FieldDeclaration(::std::boxed::Box::new(
                ::treesitter_types::runtime::maybe_grow_stack(|| {
                    <FieldDeclaration as ::treesitter_types::FromNode>::from_node(node, src)
                })?,
            ))),
            "function_definition" => Ok(Self::FunctionDefinition(::std::boxed::Box::new(
                ::treesitter_types::runtime::maybe_grow_stack(|| {
                    <FunctionDefinition as ::treesitter_types::FromNode>::from_node(node, src)
                })?,
            ))),
            "linkage_specification" => Ok(Self::LinkageSpecification(::std::boxed::Box::new(
                ::treesitter_types::runtime::maybe_grow_stack(|| {
                    <LinkageSpecification as ::treesitter_types::FromNode>::from_node(node, src)
                })?,
            ))),
            "preproc_call" => Ok(Self::PreprocCall(::std::boxed::Box::new(
                ::treesitter_types::runtime::maybe_grow_stack(|| {
                    <PreprocCall as ::treesitter_types::FromNode>::from_node(node, src)
                })?,
            ))),
            "preproc_def" => Ok(Self::PreprocDef(::std::boxed::Box::new(
                ::treesitter_types::runtime::maybe_grow_stack(|| {
                    <PreprocDef as ::treesitter_types::FromNode>::from_node(node, src)
                })?,
            ))),
            "preproc_function_def" => Ok(Self::PreprocFunctionDef(::std::boxed::Box::new(
                ::treesitter_types::runtime::maybe_grow_stack(|| {
                    <PreprocFunctionDef as ::treesitter_types::FromNode>::from_node(node, src)
                })?,
            ))),
            "preproc_if" => Ok(Self::PreprocIf(::std::boxed::Box::new(
                ::treesitter_types::runtime::maybe_grow_stack(|| {
                    <PreprocIf as ::treesitter_types::FromNode>::from_node(node, src)
                })?,
            ))),
            "preproc_ifdef" => Ok(Self::PreprocIfdef(::std::boxed::Box::new(
                ::treesitter_types::runtime::maybe_grow_stack(|| {
                    <PreprocIfdef as ::treesitter_types::FromNode>::from_node(node, src)
                })?,
            ))),
            "preproc_include" => Ok(Self::PreprocInclude(::std::boxed::Box::new(
                ::treesitter_types::runtime::maybe_grow_stack(|| {
                    <PreprocInclude as ::treesitter_types::FromNode>::from_node(node, src)
                })?,
            ))),
            "type_definition" => Ok(Self::TypeDefinition(::std::boxed::Box::new(
                ::treesitter_types::runtime::maybe_grow_stack(|| {
                    <TypeDefinition as ::treesitter_types::FromNode>::from_node(node, src)
                })?,
            ))),
            _other => {
                if let Ok(v) = ::treesitter_types::runtime::maybe_grow_stack(|| {
                    <Statement as ::treesitter_types::FromNode>::from_node(node, src)
                }) {
                    Ok(Self::Statement(::std::boxed::Box::new(v)))
                } else {
                    if let Ok(v) = ::treesitter_types::runtime::maybe_grow_stack(|| {
                        <TypeSpecifier as ::treesitter_types::FromNode>::from_node(node, src)
                    }) {
                        Ok(Self::TypeSpecifier(::std::boxed::Box::new(v)))
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
impl ::treesitter_types::Spanned for PreprocElifChildren<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        match self {
            Self::Declaration(inner) => inner.span(),
            Self::Enumerator(inner) => inner.span(),
            Self::FieldDeclaration(inner) => inner.span(),
            Self::FunctionDefinition(inner) => inner.span(),
            Self::LinkageSpecification(inner) => inner.span(),
            Self::PreprocCall(inner) => inner.span(),
            Self::PreprocDef(inner) => inner.span(),
            Self::PreprocFunctionDef(inner) => inner.span(),
            Self::PreprocIf(inner) => inner.span(),
            Self::PreprocIfdef(inner) => inner.span(),
            Self::PreprocInclude(inner) => inner.span(),
            Self::Statement(inner) => inner.span(),
            Self::TypeDefinition(inner) => inner.span(),
            Self::TypeSpecifier(inner) => inner.span(),
        }
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum PreprocElifdefAlternative<'tree> {
    PreprocElif(::std::boxed::Box<PreprocElif<'tree>>),
    PreprocElifdef(::std::boxed::Box<PreprocElifdef<'tree>>),
    PreprocElse(::std::boxed::Box<PreprocElse<'tree>>),
}
impl<'tree> ::treesitter_types::FromNode<'tree> for PreprocElifdefAlternative<'tree> {
    #[allow(clippy::collapsible_else_if)]
    fn from_node(
        node: ::treesitter_types::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        match node.kind() {
            "preproc_elif" => Ok(Self::PreprocElif(::std::boxed::Box::new(
                ::treesitter_types::runtime::maybe_grow_stack(|| {
                    <PreprocElif as ::treesitter_types::FromNode>::from_node(node, src)
                })?,
            ))),
            "preproc_elifdef" => Ok(Self::PreprocElifdef(::std::boxed::Box::new(
                ::treesitter_types::runtime::maybe_grow_stack(|| {
                    <PreprocElifdef as ::treesitter_types::FromNode>::from_node(node, src)
                })?,
            ))),
            "preproc_else" => Ok(Self::PreprocElse(::std::boxed::Box::new(
                ::treesitter_types::runtime::maybe_grow_stack(|| {
                    <PreprocElse as ::treesitter_types::FromNode>::from_node(node, src)
                })?,
            ))),
            other => Err(::treesitter_types::ParseError::unexpected_kind(other, node)),
        }
    }
}
impl ::treesitter_types::Spanned for PreprocElifdefAlternative<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        match self {
            Self::PreprocElif(inner) => inner.span(),
            Self::PreprocElifdef(inner) => inner.span(),
            Self::PreprocElse(inner) => inner.span(),
        }
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum PreprocElifdefChildren<'tree> {
    Declaration(::std::boxed::Box<Declaration<'tree>>),
    Enumerator(::std::boxed::Box<Enumerator<'tree>>),
    FieldDeclaration(::std::boxed::Box<FieldDeclaration<'tree>>),
    FunctionDefinition(::std::boxed::Box<FunctionDefinition<'tree>>),
    LinkageSpecification(::std::boxed::Box<LinkageSpecification<'tree>>),
    PreprocCall(::std::boxed::Box<PreprocCall<'tree>>),
    PreprocDef(::std::boxed::Box<PreprocDef<'tree>>),
    PreprocFunctionDef(::std::boxed::Box<PreprocFunctionDef<'tree>>),
    PreprocIf(::std::boxed::Box<PreprocIf<'tree>>),
    PreprocIfdef(::std::boxed::Box<PreprocIfdef<'tree>>),
    PreprocInclude(::std::boxed::Box<PreprocInclude<'tree>>),
    Statement(::std::boxed::Box<Statement<'tree>>),
    TypeDefinition(::std::boxed::Box<TypeDefinition<'tree>>),
    TypeSpecifier(::std::boxed::Box<TypeSpecifier<'tree>>),
}
impl<'tree> ::treesitter_types::FromNode<'tree> for PreprocElifdefChildren<'tree> {
    #[allow(clippy::collapsible_else_if)]
    fn from_node(
        node: ::treesitter_types::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        match node.kind() {
            "declaration" => Ok(Self::Declaration(::std::boxed::Box::new(
                ::treesitter_types::runtime::maybe_grow_stack(|| {
                    <Declaration as ::treesitter_types::FromNode>::from_node(node, src)
                })?,
            ))),
            "enumerator" => Ok(Self::Enumerator(::std::boxed::Box::new(
                ::treesitter_types::runtime::maybe_grow_stack(|| {
                    <Enumerator as ::treesitter_types::FromNode>::from_node(node, src)
                })?,
            ))),
            "field_declaration" => Ok(Self::FieldDeclaration(::std::boxed::Box::new(
                ::treesitter_types::runtime::maybe_grow_stack(|| {
                    <FieldDeclaration as ::treesitter_types::FromNode>::from_node(node, src)
                })?,
            ))),
            "function_definition" => Ok(Self::FunctionDefinition(::std::boxed::Box::new(
                ::treesitter_types::runtime::maybe_grow_stack(|| {
                    <FunctionDefinition as ::treesitter_types::FromNode>::from_node(node, src)
                })?,
            ))),
            "linkage_specification" => Ok(Self::LinkageSpecification(::std::boxed::Box::new(
                ::treesitter_types::runtime::maybe_grow_stack(|| {
                    <LinkageSpecification as ::treesitter_types::FromNode>::from_node(node, src)
                })?,
            ))),
            "preproc_call" => Ok(Self::PreprocCall(::std::boxed::Box::new(
                ::treesitter_types::runtime::maybe_grow_stack(|| {
                    <PreprocCall as ::treesitter_types::FromNode>::from_node(node, src)
                })?,
            ))),
            "preproc_def" => Ok(Self::PreprocDef(::std::boxed::Box::new(
                ::treesitter_types::runtime::maybe_grow_stack(|| {
                    <PreprocDef as ::treesitter_types::FromNode>::from_node(node, src)
                })?,
            ))),
            "preproc_function_def" => Ok(Self::PreprocFunctionDef(::std::boxed::Box::new(
                ::treesitter_types::runtime::maybe_grow_stack(|| {
                    <PreprocFunctionDef as ::treesitter_types::FromNode>::from_node(node, src)
                })?,
            ))),
            "preproc_if" => Ok(Self::PreprocIf(::std::boxed::Box::new(
                ::treesitter_types::runtime::maybe_grow_stack(|| {
                    <PreprocIf as ::treesitter_types::FromNode>::from_node(node, src)
                })?,
            ))),
            "preproc_ifdef" => Ok(Self::PreprocIfdef(::std::boxed::Box::new(
                ::treesitter_types::runtime::maybe_grow_stack(|| {
                    <PreprocIfdef as ::treesitter_types::FromNode>::from_node(node, src)
                })?,
            ))),
            "preproc_include" => Ok(Self::PreprocInclude(::std::boxed::Box::new(
                ::treesitter_types::runtime::maybe_grow_stack(|| {
                    <PreprocInclude as ::treesitter_types::FromNode>::from_node(node, src)
                })?,
            ))),
            "type_definition" => Ok(Self::TypeDefinition(::std::boxed::Box::new(
                ::treesitter_types::runtime::maybe_grow_stack(|| {
                    <TypeDefinition as ::treesitter_types::FromNode>::from_node(node, src)
                })?,
            ))),
            _other => {
                if let Ok(v) = ::treesitter_types::runtime::maybe_grow_stack(|| {
                    <Statement as ::treesitter_types::FromNode>::from_node(node, src)
                }) {
                    Ok(Self::Statement(::std::boxed::Box::new(v)))
                } else {
                    if let Ok(v) = ::treesitter_types::runtime::maybe_grow_stack(|| {
                        <TypeSpecifier as ::treesitter_types::FromNode>::from_node(node, src)
                    }) {
                        Ok(Self::TypeSpecifier(::std::boxed::Box::new(v)))
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
impl ::treesitter_types::Spanned for PreprocElifdefChildren<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        match self {
            Self::Declaration(inner) => inner.span(),
            Self::Enumerator(inner) => inner.span(),
            Self::FieldDeclaration(inner) => inner.span(),
            Self::FunctionDefinition(inner) => inner.span(),
            Self::LinkageSpecification(inner) => inner.span(),
            Self::PreprocCall(inner) => inner.span(),
            Self::PreprocDef(inner) => inner.span(),
            Self::PreprocFunctionDef(inner) => inner.span(),
            Self::PreprocIf(inner) => inner.span(),
            Self::PreprocIfdef(inner) => inner.span(),
            Self::PreprocInclude(inner) => inner.span(),
            Self::Statement(inner) => inner.span(),
            Self::TypeDefinition(inner) => inner.span(),
            Self::TypeSpecifier(inner) => inner.span(),
        }
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum PreprocElseChildren<'tree> {
    Declaration(::std::boxed::Box<Declaration<'tree>>),
    Enumerator(::std::boxed::Box<Enumerator<'tree>>),
    FieldDeclaration(::std::boxed::Box<FieldDeclaration<'tree>>),
    FunctionDefinition(::std::boxed::Box<FunctionDefinition<'tree>>),
    LinkageSpecification(::std::boxed::Box<LinkageSpecification<'tree>>),
    PreprocCall(::std::boxed::Box<PreprocCall<'tree>>),
    PreprocDef(::std::boxed::Box<PreprocDef<'tree>>),
    PreprocFunctionDef(::std::boxed::Box<PreprocFunctionDef<'tree>>),
    PreprocIf(::std::boxed::Box<PreprocIf<'tree>>),
    PreprocIfdef(::std::boxed::Box<PreprocIfdef<'tree>>),
    PreprocInclude(::std::boxed::Box<PreprocInclude<'tree>>),
    Statement(::std::boxed::Box<Statement<'tree>>),
    TypeDefinition(::std::boxed::Box<TypeDefinition<'tree>>),
    TypeSpecifier(::std::boxed::Box<TypeSpecifier<'tree>>),
}
impl<'tree> ::treesitter_types::FromNode<'tree> for PreprocElseChildren<'tree> {
    #[allow(clippy::collapsible_else_if)]
    fn from_node(
        node: ::treesitter_types::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        match node.kind() {
            "declaration" => Ok(Self::Declaration(::std::boxed::Box::new(
                ::treesitter_types::runtime::maybe_grow_stack(|| {
                    <Declaration as ::treesitter_types::FromNode>::from_node(node, src)
                })?,
            ))),
            "enumerator" => Ok(Self::Enumerator(::std::boxed::Box::new(
                ::treesitter_types::runtime::maybe_grow_stack(|| {
                    <Enumerator as ::treesitter_types::FromNode>::from_node(node, src)
                })?,
            ))),
            "field_declaration" => Ok(Self::FieldDeclaration(::std::boxed::Box::new(
                ::treesitter_types::runtime::maybe_grow_stack(|| {
                    <FieldDeclaration as ::treesitter_types::FromNode>::from_node(node, src)
                })?,
            ))),
            "function_definition" => Ok(Self::FunctionDefinition(::std::boxed::Box::new(
                ::treesitter_types::runtime::maybe_grow_stack(|| {
                    <FunctionDefinition as ::treesitter_types::FromNode>::from_node(node, src)
                })?,
            ))),
            "linkage_specification" => Ok(Self::LinkageSpecification(::std::boxed::Box::new(
                ::treesitter_types::runtime::maybe_grow_stack(|| {
                    <LinkageSpecification as ::treesitter_types::FromNode>::from_node(node, src)
                })?,
            ))),
            "preproc_call" => Ok(Self::PreprocCall(::std::boxed::Box::new(
                ::treesitter_types::runtime::maybe_grow_stack(|| {
                    <PreprocCall as ::treesitter_types::FromNode>::from_node(node, src)
                })?,
            ))),
            "preproc_def" => Ok(Self::PreprocDef(::std::boxed::Box::new(
                ::treesitter_types::runtime::maybe_grow_stack(|| {
                    <PreprocDef as ::treesitter_types::FromNode>::from_node(node, src)
                })?,
            ))),
            "preproc_function_def" => Ok(Self::PreprocFunctionDef(::std::boxed::Box::new(
                ::treesitter_types::runtime::maybe_grow_stack(|| {
                    <PreprocFunctionDef as ::treesitter_types::FromNode>::from_node(node, src)
                })?,
            ))),
            "preproc_if" => Ok(Self::PreprocIf(::std::boxed::Box::new(
                ::treesitter_types::runtime::maybe_grow_stack(|| {
                    <PreprocIf as ::treesitter_types::FromNode>::from_node(node, src)
                })?,
            ))),
            "preproc_ifdef" => Ok(Self::PreprocIfdef(::std::boxed::Box::new(
                ::treesitter_types::runtime::maybe_grow_stack(|| {
                    <PreprocIfdef as ::treesitter_types::FromNode>::from_node(node, src)
                })?,
            ))),
            "preproc_include" => Ok(Self::PreprocInclude(::std::boxed::Box::new(
                ::treesitter_types::runtime::maybe_grow_stack(|| {
                    <PreprocInclude as ::treesitter_types::FromNode>::from_node(node, src)
                })?,
            ))),
            "type_definition" => Ok(Self::TypeDefinition(::std::boxed::Box::new(
                ::treesitter_types::runtime::maybe_grow_stack(|| {
                    <TypeDefinition as ::treesitter_types::FromNode>::from_node(node, src)
                })?,
            ))),
            _other => {
                if let Ok(v) = ::treesitter_types::runtime::maybe_grow_stack(|| {
                    <Statement as ::treesitter_types::FromNode>::from_node(node, src)
                }) {
                    Ok(Self::Statement(::std::boxed::Box::new(v)))
                } else {
                    if let Ok(v) = ::treesitter_types::runtime::maybe_grow_stack(|| {
                        <TypeSpecifier as ::treesitter_types::FromNode>::from_node(node, src)
                    }) {
                        Ok(Self::TypeSpecifier(::std::boxed::Box::new(v)))
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
impl ::treesitter_types::Spanned for PreprocElseChildren<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        match self {
            Self::Declaration(inner) => inner.span(),
            Self::Enumerator(inner) => inner.span(),
            Self::FieldDeclaration(inner) => inner.span(),
            Self::FunctionDefinition(inner) => inner.span(),
            Self::LinkageSpecification(inner) => inner.span(),
            Self::PreprocCall(inner) => inner.span(),
            Self::PreprocDef(inner) => inner.span(),
            Self::PreprocFunctionDef(inner) => inner.span(),
            Self::PreprocIf(inner) => inner.span(),
            Self::PreprocIfdef(inner) => inner.span(),
            Self::PreprocInclude(inner) => inner.span(),
            Self::Statement(inner) => inner.span(),
            Self::TypeDefinition(inner) => inner.span(),
            Self::TypeSpecifier(inner) => inner.span(),
        }
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum PreprocIfAlternative<'tree> {
    PreprocElif(::std::boxed::Box<PreprocElif<'tree>>),
    PreprocElifdef(::std::boxed::Box<PreprocElifdef<'tree>>),
    PreprocElse(::std::boxed::Box<PreprocElse<'tree>>),
}
impl<'tree> ::treesitter_types::FromNode<'tree> for PreprocIfAlternative<'tree> {
    #[allow(clippy::collapsible_else_if)]
    fn from_node(
        node: ::treesitter_types::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        match node.kind() {
            "preproc_elif" => Ok(Self::PreprocElif(::std::boxed::Box::new(
                ::treesitter_types::runtime::maybe_grow_stack(|| {
                    <PreprocElif as ::treesitter_types::FromNode>::from_node(node, src)
                })?,
            ))),
            "preproc_elifdef" => Ok(Self::PreprocElifdef(::std::boxed::Box::new(
                ::treesitter_types::runtime::maybe_grow_stack(|| {
                    <PreprocElifdef as ::treesitter_types::FromNode>::from_node(node, src)
                })?,
            ))),
            "preproc_else" => Ok(Self::PreprocElse(::std::boxed::Box::new(
                ::treesitter_types::runtime::maybe_grow_stack(|| {
                    <PreprocElse as ::treesitter_types::FromNode>::from_node(node, src)
                })?,
            ))),
            other => Err(::treesitter_types::ParseError::unexpected_kind(other, node)),
        }
    }
}
impl ::treesitter_types::Spanned for PreprocIfAlternative<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        match self {
            Self::PreprocElif(inner) => inner.span(),
            Self::PreprocElifdef(inner) => inner.span(),
            Self::PreprocElse(inner) => inner.span(),
        }
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum PreprocIfCondition<'tree> {
    BinaryExpression(::std::boxed::Box<BinaryExpression<'tree>>),
    CallExpression(::std::boxed::Box<CallExpression<'tree>>),
    CharLiteral(::std::boxed::Box<CharLiteral<'tree>>),
    Identifier(::std::boxed::Box<Identifier<'tree>>),
    NumberLiteral(::std::boxed::Box<NumberLiteral<'tree>>),
    ParenthesizedExpression(::std::boxed::Box<ParenthesizedExpression<'tree>>),
    PreprocDefined(::std::boxed::Box<PreprocDefined<'tree>>),
    UnaryExpression(::std::boxed::Box<UnaryExpression<'tree>>),
}
impl<'tree> ::treesitter_types::FromNode<'tree> for PreprocIfCondition<'tree> {
    #[allow(clippy::collapsible_else_if)]
    fn from_node(
        node: ::treesitter_types::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        match node.kind() {
            "binary_expression" => Ok(Self::BinaryExpression(::std::boxed::Box::new(
                ::treesitter_types::runtime::maybe_grow_stack(|| {
                    <BinaryExpression as ::treesitter_types::FromNode>::from_node(node, src)
                })?,
            ))),
            "call_expression" => Ok(Self::CallExpression(::std::boxed::Box::new(
                ::treesitter_types::runtime::maybe_grow_stack(|| {
                    <CallExpression as ::treesitter_types::FromNode>::from_node(node, src)
                })?,
            ))),
            "char_literal" => Ok(Self::CharLiteral(::std::boxed::Box::new(
                ::treesitter_types::runtime::maybe_grow_stack(|| {
                    <CharLiteral as ::treesitter_types::FromNode>::from_node(node, src)
                })?,
            ))),
            "identifier" => Ok(Self::Identifier(::std::boxed::Box::new(
                ::treesitter_types::runtime::maybe_grow_stack(|| {
                    <Identifier as ::treesitter_types::FromNode>::from_node(node, src)
                })?,
            ))),
            "number_literal" => Ok(Self::NumberLiteral(::std::boxed::Box::new(
                ::treesitter_types::runtime::maybe_grow_stack(|| {
                    <NumberLiteral as ::treesitter_types::FromNode>::from_node(node, src)
                })?,
            ))),
            "parenthesized_expression" => Ok(Self::ParenthesizedExpression(
                ::std::boxed::Box::new(::treesitter_types::runtime::maybe_grow_stack(|| {
                    <ParenthesizedExpression as ::treesitter_types::FromNode>::from_node(node, src)
                })?),
            )),
            "preproc_defined" => Ok(Self::PreprocDefined(::std::boxed::Box::new(
                ::treesitter_types::runtime::maybe_grow_stack(|| {
                    <PreprocDefined as ::treesitter_types::FromNode>::from_node(node, src)
                })?,
            ))),
            "unary_expression" => Ok(Self::UnaryExpression(::std::boxed::Box::new(
                ::treesitter_types::runtime::maybe_grow_stack(|| {
                    <UnaryExpression as ::treesitter_types::FromNode>::from_node(node, src)
                })?,
            ))),
            other => Err(::treesitter_types::ParseError::unexpected_kind(other, node)),
        }
    }
}
impl ::treesitter_types::Spanned for PreprocIfCondition<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        match self {
            Self::BinaryExpression(inner) => inner.span(),
            Self::CallExpression(inner) => inner.span(),
            Self::CharLiteral(inner) => inner.span(),
            Self::Identifier(inner) => inner.span(),
            Self::NumberLiteral(inner) => inner.span(),
            Self::ParenthesizedExpression(inner) => inner.span(),
            Self::PreprocDefined(inner) => inner.span(),
            Self::UnaryExpression(inner) => inner.span(),
        }
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum PreprocIfChildren<'tree> {
    Declaration(::std::boxed::Box<Declaration<'tree>>),
    Enumerator(::std::boxed::Box<Enumerator<'tree>>),
    FieldDeclaration(::std::boxed::Box<FieldDeclaration<'tree>>),
    FunctionDefinition(::std::boxed::Box<FunctionDefinition<'tree>>),
    LinkageSpecification(::std::boxed::Box<LinkageSpecification<'tree>>),
    PreprocCall(::std::boxed::Box<PreprocCall<'tree>>),
    PreprocDef(::std::boxed::Box<PreprocDef<'tree>>),
    PreprocFunctionDef(::std::boxed::Box<PreprocFunctionDef<'tree>>),
    PreprocIf(::std::boxed::Box<PreprocIf<'tree>>),
    PreprocIfdef(::std::boxed::Box<PreprocIfdef<'tree>>),
    PreprocInclude(::std::boxed::Box<PreprocInclude<'tree>>),
    Statement(::std::boxed::Box<Statement<'tree>>),
    TypeDefinition(::std::boxed::Box<TypeDefinition<'tree>>),
    TypeSpecifier(::std::boxed::Box<TypeSpecifier<'tree>>),
}
impl<'tree> ::treesitter_types::FromNode<'tree> for PreprocIfChildren<'tree> {
    #[allow(clippy::collapsible_else_if)]
    fn from_node(
        node: ::treesitter_types::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        match node.kind() {
            "declaration" => Ok(Self::Declaration(::std::boxed::Box::new(
                ::treesitter_types::runtime::maybe_grow_stack(|| {
                    <Declaration as ::treesitter_types::FromNode>::from_node(node, src)
                })?,
            ))),
            "enumerator" => Ok(Self::Enumerator(::std::boxed::Box::new(
                ::treesitter_types::runtime::maybe_grow_stack(|| {
                    <Enumerator as ::treesitter_types::FromNode>::from_node(node, src)
                })?,
            ))),
            "field_declaration" => Ok(Self::FieldDeclaration(::std::boxed::Box::new(
                ::treesitter_types::runtime::maybe_grow_stack(|| {
                    <FieldDeclaration as ::treesitter_types::FromNode>::from_node(node, src)
                })?,
            ))),
            "function_definition" => Ok(Self::FunctionDefinition(::std::boxed::Box::new(
                ::treesitter_types::runtime::maybe_grow_stack(|| {
                    <FunctionDefinition as ::treesitter_types::FromNode>::from_node(node, src)
                })?,
            ))),
            "linkage_specification" => Ok(Self::LinkageSpecification(::std::boxed::Box::new(
                ::treesitter_types::runtime::maybe_grow_stack(|| {
                    <LinkageSpecification as ::treesitter_types::FromNode>::from_node(node, src)
                })?,
            ))),
            "preproc_call" => Ok(Self::PreprocCall(::std::boxed::Box::new(
                ::treesitter_types::runtime::maybe_grow_stack(|| {
                    <PreprocCall as ::treesitter_types::FromNode>::from_node(node, src)
                })?,
            ))),
            "preproc_def" => Ok(Self::PreprocDef(::std::boxed::Box::new(
                ::treesitter_types::runtime::maybe_grow_stack(|| {
                    <PreprocDef as ::treesitter_types::FromNode>::from_node(node, src)
                })?,
            ))),
            "preproc_function_def" => Ok(Self::PreprocFunctionDef(::std::boxed::Box::new(
                ::treesitter_types::runtime::maybe_grow_stack(|| {
                    <PreprocFunctionDef as ::treesitter_types::FromNode>::from_node(node, src)
                })?,
            ))),
            "preproc_if" => Ok(Self::PreprocIf(::std::boxed::Box::new(
                ::treesitter_types::runtime::maybe_grow_stack(|| {
                    <PreprocIf as ::treesitter_types::FromNode>::from_node(node, src)
                })?,
            ))),
            "preproc_ifdef" => Ok(Self::PreprocIfdef(::std::boxed::Box::new(
                ::treesitter_types::runtime::maybe_grow_stack(|| {
                    <PreprocIfdef as ::treesitter_types::FromNode>::from_node(node, src)
                })?,
            ))),
            "preproc_include" => Ok(Self::PreprocInclude(::std::boxed::Box::new(
                ::treesitter_types::runtime::maybe_grow_stack(|| {
                    <PreprocInclude as ::treesitter_types::FromNode>::from_node(node, src)
                })?,
            ))),
            "type_definition" => Ok(Self::TypeDefinition(::std::boxed::Box::new(
                ::treesitter_types::runtime::maybe_grow_stack(|| {
                    <TypeDefinition as ::treesitter_types::FromNode>::from_node(node, src)
                })?,
            ))),
            _other => {
                if let Ok(v) = ::treesitter_types::runtime::maybe_grow_stack(|| {
                    <Statement as ::treesitter_types::FromNode>::from_node(node, src)
                }) {
                    Ok(Self::Statement(::std::boxed::Box::new(v)))
                } else {
                    if let Ok(v) = ::treesitter_types::runtime::maybe_grow_stack(|| {
                        <TypeSpecifier as ::treesitter_types::FromNode>::from_node(node, src)
                    }) {
                        Ok(Self::TypeSpecifier(::std::boxed::Box::new(v)))
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
impl ::treesitter_types::Spanned for PreprocIfChildren<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        match self {
            Self::Declaration(inner) => inner.span(),
            Self::Enumerator(inner) => inner.span(),
            Self::FieldDeclaration(inner) => inner.span(),
            Self::FunctionDefinition(inner) => inner.span(),
            Self::LinkageSpecification(inner) => inner.span(),
            Self::PreprocCall(inner) => inner.span(),
            Self::PreprocDef(inner) => inner.span(),
            Self::PreprocFunctionDef(inner) => inner.span(),
            Self::PreprocIf(inner) => inner.span(),
            Self::PreprocIfdef(inner) => inner.span(),
            Self::PreprocInclude(inner) => inner.span(),
            Self::Statement(inner) => inner.span(),
            Self::TypeDefinition(inner) => inner.span(),
            Self::TypeSpecifier(inner) => inner.span(),
        }
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum PreprocIfdefAlternative<'tree> {
    PreprocElif(::std::boxed::Box<PreprocElif<'tree>>),
    PreprocElifdef(::std::boxed::Box<PreprocElifdef<'tree>>),
    PreprocElse(::std::boxed::Box<PreprocElse<'tree>>),
}
impl<'tree> ::treesitter_types::FromNode<'tree> for PreprocIfdefAlternative<'tree> {
    #[allow(clippy::collapsible_else_if)]
    fn from_node(
        node: ::treesitter_types::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        match node.kind() {
            "preproc_elif" => Ok(Self::PreprocElif(::std::boxed::Box::new(
                ::treesitter_types::runtime::maybe_grow_stack(|| {
                    <PreprocElif as ::treesitter_types::FromNode>::from_node(node, src)
                })?,
            ))),
            "preproc_elifdef" => Ok(Self::PreprocElifdef(::std::boxed::Box::new(
                ::treesitter_types::runtime::maybe_grow_stack(|| {
                    <PreprocElifdef as ::treesitter_types::FromNode>::from_node(node, src)
                })?,
            ))),
            "preproc_else" => Ok(Self::PreprocElse(::std::boxed::Box::new(
                ::treesitter_types::runtime::maybe_grow_stack(|| {
                    <PreprocElse as ::treesitter_types::FromNode>::from_node(node, src)
                })?,
            ))),
            other => Err(::treesitter_types::ParseError::unexpected_kind(other, node)),
        }
    }
}
impl ::treesitter_types::Spanned for PreprocIfdefAlternative<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        match self {
            Self::PreprocElif(inner) => inner.span(),
            Self::PreprocElifdef(inner) => inner.span(),
            Self::PreprocElse(inner) => inner.span(),
        }
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum PreprocIfdefChildren<'tree> {
    Declaration(::std::boxed::Box<Declaration<'tree>>),
    Enumerator(::std::boxed::Box<Enumerator<'tree>>),
    FieldDeclaration(::std::boxed::Box<FieldDeclaration<'tree>>),
    FunctionDefinition(::std::boxed::Box<FunctionDefinition<'tree>>),
    LinkageSpecification(::std::boxed::Box<LinkageSpecification<'tree>>),
    PreprocCall(::std::boxed::Box<PreprocCall<'tree>>),
    PreprocDef(::std::boxed::Box<PreprocDef<'tree>>),
    PreprocFunctionDef(::std::boxed::Box<PreprocFunctionDef<'tree>>),
    PreprocIf(::std::boxed::Box<PreprocIf<'tree>>),
    PreprocIfdef(::std::boxed::Box<PreprocIfdef<'tree>>),
    PreprocInclude(::std::boxed::Box<PreprocInclude<'tree>>),
    Statement(::std::boxed::Box<Statement<'tree>>),
    TypeDefinition(::std::boxed::Box<TypeDefinition<'tree>>),
    TypeSpecifier(::std::boxed::Box<TypeSpecifier<'tree>>),
}
impl<'tree> ::treesitter_types::FromNode<'tree> for PreprocIfdefChildren<'tree> {
    #[allow(clippy::collapsible_else_if)]
    fn from_node(
        node: ::treesitter_types::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        match node.kind() {
            "declaration" => Ok(Self::Declaration(::std::boxed::Box::new(
                ::treesitter_types::runtime::maybe_grow_stack(|| {
                    <Declaration as ::treesitter_types::FromNode>::from_node(node, src)
                })?,
            ))),
            "enumerator" => Ok(Self::Enumerator(::std::boxed::Box::new(
                ::treesitter_types::runtime::maybe_grow_stack(|| {
                    <Enumerator as ::treesitter_types::FromNode>::from_node(node, src)
                })?,
            ))),
            "field_declaration" => Ok(Self::FieldDeclaration(::std::boxed::Box::new(
                ::treesitter_types::runtime::maybe_grow_stack(|| {
                    <FieldDeclaration as ::treesitter_types::FromNode>::from_node(node, src)
                })?,
            ))),
            "function_definition" => Ok(Self::FunctionDefinition(::std::boxed::Box::new(
                ::treesitter_types::runtime::maybe_grow_stack(|| {
                    <FunctionDefinition as ::treesitter_types::FromNode>::from_node(node, src)
                })?,
            ))),
            "linkage_specification" => Ok(Self::LinkageSpecification(::std::boxed::Box::new(
                ::treesitter_types::runtime::maybe_grow_stack(|| {
                    <LinkageSpecification as ::treesitter_types::FromNode>::from_node(node, src)
                })?,
            ))),
            "preproc_call" => Ok(Self::PreprocCall(::std::boxed::Box::new(
                ::treesitter_types::runtime::maybe_grow_stack(|| {
                    <PreprocCall as ::treesitter_types::FromNode>::from_node(node, src)
                })?,
            ))),
            "preproc_def" => Ok(Self::PreprocDef(::std::boxed::Box::new(
                ::treesitter_types::runtime::maybe_grow_stack(|| {
                    <PreprocDef as ::treesitter_types::FromNode>::from_node(node, src)
                })?,
            ))),
            "preproc_function_def" => Ok(Self::PreprocFunctionDef(::std::boxed::Box::new(
                ::treesitter_types::runtime::maybe_grow_stack(|| {
                    <PreprocFunctionDef as ::treesitter_types::FromNode>::from_node(node, src)
                })?,
            ))),
            "preproc_if" => Ok(Self::PreprocIf(::std::boxed::Box::new(
                ::treesitter_types::runtime::maybe_grow_stack(|| {
                    <PreprocIf as ::treesitter_types::FromNode>::from_node(node, src)
                })?,
            ))),
            "preproc_ifdef" => Ok(Self::PreprocIfdef(::std::boxed::Box::new(
                ::treesitter_types::runtime::maybe_grow_stack(|| {
                    <PreprocIfdef as ::treesitter_types::FromNode>::from_node(node, src)
                })?,
            ))),
            "preproc_include" => Ok(Self::PreprocInclude(::std::boxed::Box::new(
                ::treesitter_types::runtime::maybe_grow_stack(|| {
                    <PreprocInclude as ::treesitter_types::FromNode>::from_node(node, src)
                })?,
            ))),
            "type_definition" => Ok(Self::TypeDefinition(::std::boxed::Box::new(
                ::treesitter_types::runtime::maybe_grow_stack(|| {
                    <TypeDefinition as ::treesitter_types::FromNode>::from_node(node, src)
                })?,
            ))),
            _other => {
                if let Ok(v) = ::treesitter_types::runtime::maybe_grow_stack(|| {
                    <Statement as ::treesitter_types::FromNode>::from_node(node, src)
                }) {
                    Ok(Self::Statement(::std::boxed::Box::new(v)))
                } else {
                    if let Ok(v) = ::treesitter_types::runtime::maybe_grow_stack(|| {
                        <TypeSpecifier as ::treesitter_types::FromNode>::from_node(node, src)
                    }) {
                        Ok(Self::TypeSpecifier(::std::boxed::Box::new(v)))
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
impl ::treesitter_types::Spanned for PreprocIfdefChildren<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        match self {
            Self::Declaration(inner) => inner.span(),
            Self::Enumerator(inner) => inner.span(),
            Self::FieldDeclaration(inner) => inner.span(),
            Self::FunctionDefinition(inner) => inner.span(),
            Self::LinkageSpecification(inner) => inner.span(),
            Self::PreprocCall(inner) => inner.span(),
            Self::PreprocDef(inner) => inner.span(),
            Self::PreprocFunctionDef(inner) => inner.span(),
            Self::PreprocIf(inner) => inner.span(),
            Self::PreprocIfdef(inner) => inner.span(),
            Self::PreprocInclude(inner) => inner.span(),
            Self::Statement(inner) => inner.span(),
            Self::TypeDefinition(inner) => inner.span(),
            Self::TypeSpecifier(inner) => inner.span(),
        }
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum PreprocIncludePath<'tree> {
    CallExpression(::std::boxed::Box<CallExpression<'tree>>),
    Identifier(::std::boxed::Box<Identifier<'tree>>),
    StringLiteral(::std::boxed::Box<StringLiteral<'tree>>),
    SystemLibString(::std::boxed::Box<SystemLibString<'tree>>),
}
impl<'tree> ::treesitter_types::FromNode<'tree> for PreprocIncludePath<'tree> {
    #[allow(clippy::collapsible_else_if)]
    fn from_node(
        node: ::treesitter_types::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        match node.kind() {
            "call_expression" => Ok(Self::CallExpression(::std::boxed::Box::new(
                ::treesitter_types::runtime::maybe_grow_stack(|| {
                    <CallExpression as ::treesitter_types::FromNode>::from_node(node, src)
                })?,
            ))),
            "identifier" => Ok(Self::Identifier(::std::boxed::Box::new(
                ::treesitter_types::runtime::maybe_grow_stack(|| {
                    <Identifier as ::treesitter_types::FromNode>::from_node(node, src)
                })?,
            ))),
            "string_literal" => Ok(Self::StringLiteral(::std::boxed::Box::new(
                ::treesitter_types::runtime::maybe_grow_stack(|| {
                    <StringLiteral as ::treesitter_types::FromNode>::from_node(node, src)
                })?,
            ))),
            "system_lib_string" => Ok(Self::SystemLibString(::std::boxed::Box::new(
                ::treesitter_types::runtime::maybe_grow_stack(|| {
                    <SystemLibString as ::treesitter_types::FromNode>::from_node(node, src)
                })?,
            ))),
            other => Err(::treesitter_types::ParseError::unexpected_kind(other, node)),
        }
    }
}
impl ::treesitter_types::Spanned for PreprocIncludePath<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        match self {
            Self::CallExpression(inner) => inner.span(),
            Self::Identifier(inner) => inner.span(),
            Self::StringLiteral(inner) => inner.span(),
            Self::SystemLibString(inner) => inner.span(),
        }
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ReturnStatementChildren<'tree> {
    CommaExpression(::std::boxed::Box<CommaExpression<'tree>>),
    Expression(::std::boxed::Box<Expression<'tree>>),
}
impl<'tree> ::treesitter_types::FromNode<'tree> for ReturnStatementChildren<'tree> {
    #[allow(clippy::collapsible_else_if)]
    fn from_node(
        node: ::treesitter_types::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        match node.kind() {
            "comma_expression" => Ok(Self::CommaExpression(::std::boxed::Box::new(
                ::treesitter_types::runtime::maybe_grow_stack(|| {
                    <CommaExpression as ::treesitter_types::FromNode>::from_node(node, src)
                })?,
            ))),
            _other => {
                if let Ok(v) = ::treesitter_types::runtime::maybe_grow_stack(|| {
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
impl ::treesitter_types::Spanned for ReturnStatementChildren<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        match self {
            Self::CommaExpression(inner) => inner.span(),
            Self::Expression(inner) => inner.span(),
        }
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum SehTryStatementChildren<'tree> {
    SehExceptClause(::std::boxed::Box<SehExceptClause<'tree>>),
    SehFinallyClause(::std::boxed::Box<SehFinallyClause<'tree>>),
}
impl<'tree> ::treesitter_types::FromNode<'tree> for SehTryStatementChildren<'tree> {
    #[allow(clippy::collapsible_else_if)]
    fn from_node(
        node: ::treesitter_types::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        match node.kind() {
            "seh_except_clause" => Ok(Self::SehExceptClause(::std::boxed::Box::new(
                ::treesitter_types::runtime::maybe_grow_stack(|| {
                    <SehExceptClause as ::treesitter_types::FromNode>::from_node(node, src)
                })?,
            ))),
            "seh_finally_clause" => Ok(Self::SehFinallyClause(::std::boxed::Box::new(
                ::treesitter_types::runtime::maybe_grow_stack(|| {
                    <SehFinallyClause as ::treesitter_types::FromNode>::from_node(node, src)
                })?,
            ))),
            other => Err(::treesitter_types::ParseError::unexpected_kind(other, node)),
        }
    }
}
impl ::treesitter_types::Spanned for SehTryStatementChildren<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        match self {
            Self::SehExceptClause(inner) => inner.span(),
            Self::SehFinallyClause(inner) => inner.span(),
        }
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum SizedTypeSpecifierType<'tree> {
    PrimitiveType(::std::boxed::Box<PrimitiveType<'tree>>),
    TypeIdentifier(::std::boxed::Box<TypeIdentifier<'tree>>),
}
impl<'tree> ::treesitter_types::FromNode<'tree> for SizedTypeSpecifierType<'tree> {
    #[allow(clippy::collapsible_else_if)]
    fn from_node(
        node: ::treesitter_types::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        match node.kind() {
            "primitive_type" => Ok(Self::PrimitiveType(::std::boxed::Box::new(
                ::treesitter_types::runtime::maybe_grow_stack(|| {
                    <PrimitiveType as ::treesitter_types::FromNode>::from_node(node, src)
                })?,
            ))),
            "type_identifier" => Ok(Self::TypeIdentifier(::std::boxed::Box::new(
                ::treesitter_types::runtime::maybe_grow_stack(|| {
                    <TypeIdentifier as ::treesitter_types::FromNode>::from_node(node, src)
                })?,
            ))),
            other => Err(::treesitter_types::ParseError::unexpected_kind(other, node)),
        }
    }
}
impl ::treesitter_types::Spanned for SizedTypeSpecifierType<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        match self {
            Self::PrimitiveType(inner) => inner.span(),
            Self::TypeIdentifier(inner) => inner.span(),
        }
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum StringLiteralChildren<'tree> {
    EscapeSequence(::std::boxed::Box<EscapeSequence<'tree>>),
    StringContent(::std::boxed::Box<StringContent<'tree>>),
}
impl<'tree> ::treesitter_types::FromNode<'tree> for StringLiteralChildren<'tree> {
    #[allow(clippy::collapsible_else_if)]
    fn from_node(
        node: ::treesitter_types::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        match node.kind() {
            "escape_sequence" => Ok(Self::EscapeSequence(::std::boxed::Box::new(
                ::treesitter_types::runtime::maybe_grow_stack(|| {
                    <EscapeSequence as ::treesitter_types::FromNode>::from_node(node, src)
                })?,
            ))),
            "string_content" => Ok(Self::StringContent(::std::boxed::Box::new(
                ::treesitter_types::runtime::maybe_grow_stack(|| {
                    <StringContent as ::treesitter_types::FromNode>::from_node(node, src)
                })?,
            ))),
            other => Err(::treesitter_types::ParseError::unexpected_kind(other, node)),
        }
    }
}
impl ::treesitter_types::Spanned for StringLiteralChildren<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        match self {
            Self::EscapeSequence(inner) => inner.span(),
            Self::StringContent(inner) => inner.span(),
        }
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum StructSpecifierChildren<'tree> {
    AttributeSpecifier(::std::boxed::Box<AttributeSpecifier<'tree>>),
    MsDeclspecModifier(::std::boxed::Box<MsDeclspecModifier<'tree>>),
}
impl<'tree> ::treesitter_types::FromNode<'tree> for StructSpecifierChildren<'tree> {
    #[allow(clippy::collapsible_else_if)]
    fn from_node(
        node: ::treesitter_types::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        match node.kind() {
            "attribute_specifier" => Ok(Self::AttributeSpecifier(::std::boxed::Box::new(
                ::treesitter_types::runtime::maybe_grow_stack(|| {
                    <AttributeSpecifier as ::treesitter_types::FromNode>::from_node(node, src)
                })?,
            ))),
            "ms_declspec_modifier" => Ok(Self::MsDeclspecModifier(::std::boxed::Box::new(
                ::treesitter_types::runtime::maybe_grow_stack(|| {
                    <MsDeclspecModifier as ::treesitter_types::FromNode>::from_node(node, src)
                })?,
            ))),
            other => Err(::treesitter_types::ParseError::unexpected_kind(other, node)),
        }
    }
}
impl ::treesitter_types::Spanned for StructSpecifierChildren<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        match self {
            Self::AttributeSpecifier(inner) => inner.span(),
            Self::MsDeclspecModifier(inner) => inner.span(),
        }
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum TranslationUnitChildren<'tree> {
    AttributedStatement(::std::boxed::Box<AttributedStatement<'tree>>),
    BreakStatement(::std::boxed::Box<BreakStatement<'tree>>),
    CaseStatement(::std::boxed::Box<CaseStatement<'tree>>),
    CompoundStatement(::std::boxed::Box<CompoundStatement<'tree>>),
    ContinueStatement(::std::boxed::Box<ContinueStatement<'tree>>),
    Declaration(::std::boxed::Box<Declaration<'tree>>),
    DoStatement(::std::boxed::Box<DoStatement<'tree>>),
    ExpressionStatement(::std::boxed::Box<ExpressionStatement<'tree>>),
    ForStatement(::std::boxed::Box<ForStatement<'tree>>),
    FunctionDefinition(::std::boxed::Box<FunctionDefinition<'tree>>),
    GotoStatement(::std::boxed::Box<GotoStatement<'tree>>),
    IfStatement(::std::boxed::Box<IfStatement<'tree>>),
    LabeledStatement(::std::boxed::Box<LabeledStatement<'tree>>),
    LinkageSpecification(::std::boxed::Box<LinkageSpecification<'tree>>),
    PreprocCall(::std::boxed::Box<PreprocCall<'tree>>),
    PreprocDef(::std::boxed::Box<PreprocDef<'tree>>),
    PreprocFunctionDef(::std::boxed::Box<PreprocFunctionDef<'tree>>),
    PreprocIf(::std::boxed::Box<PreprocIf<'tree>>),
    PreprocIfdef(::std::boxed::Box<PreprocIfdef<'tree>>),
    PreprocInclude(::std::boxed::Box<PreprocInclude<'tree>>),
    ReturnStatement(::std::boxed::Box<ReturnStatement<'tree>>),
    SwitchStatement(::std::boxed::Box<SwitchStatement<'tree>>),
    TypeDefinition(::std::boxed::Box<TypeDefinition<'tree>>),
    TypeSpecifier(::std::boxed::Box<TypeSpecifier<'tree>>),
    WhileStatement(::std::boxed::Box<WhileStatement<'tree>>),
}
impl<'tree> ::treesitter_types::FromNode<'tree> for TranslationUnitChildren<'tree> {
    #[allow(clippy::collapsible_else_if)]
    fn from_node(
        node: ::treesitter_types::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        match node.kind() {
            "attributed_statement" => Ok(Self::AttributedStatement(::std::boxed::Box::new(
                ::treesitter_types::runtime::maybe_grow_stack(|| {
                    <AttributedStatement as ::treesitter_types::FromNode>::from_node(node, src)
                })?,
            ))),
            "break_statement" => Ok(Self::BreakStatement(::std::boxed::Box::new(
                ::treesitter_types::runtime::maybe_grow_stack(|| {
                    <BreakStatement as ::treesitter_types::FromNode>::from_node(node, src)
                })?,
            ))),
            "case_statement" => Ok(Self::CaseStatement(::std::boxed::Box::new(
                ::treesitter_types::runtime::maybe_grow_stack(|| {
                    <CaseStatement as ::treesitter_types::FromNode>::from_node(node, src)
                })?,
            ))),
            "compound_statement" => Ok(Self::CompoundStatement(::std::boxed::Box::new(
                ::treesitter_types::runtime::maybe_grow_stack(|| {
                    <CompoundStatement as ::treesitter_types::FromNode>::from_node(node, src)
                })?,
            ))),
            "continue_statement" => Ok(Self::ContinueStatement(::std::boxed::Box::new(
                ::treesitter_types::runtime::maybe_grow_stack(|| {
                    <ContinueStatement as ::treesitter_types::FromNode>::from_node(node, src)
                })?,
            ))),
            "declaration" => Ok(Self::Declaration(::std::boxed::Box::new(
                ::treesitter_types::runtime::maybe_grow_stack(|| {
                    <Declaration as ::treesitter_types::FromNode>::from_node(node, src)
                })?,
            ))),
            "do_statement" => Ok(Self::DoStatement(::std::boxed::Box::new(
                ::treesitter_types::runtime::maybe_grow_stack(|| {
                    <DoStatement as ::treesitter_types::FromNode>::from_node(node, src)
                })?,
            ))),
            "expression_statement" => Ok(Self::ExpressionStatement(::std::boxed::Box::new(
                ::treesitter_types::runtime::maybe_grow_stack(|| {
                    <ExpressionStatement as ::treesitter_types::FromNode>::from_node(node, src)
                })?,
            ))),
            "for_statement" => Ok(Self::ForStatement(::std::boxed::Box::new(
                ::treesitter_types::runtime::maybe_grow_stack(|| {
                    <ForStatement as ::treesitter_types::FromNode>::from_node(node, src)
                })?,
            ))),
            "function_definition" => Ok(Self::FunctionDefinition(::std::boxed::Box::new(
                ::treesitter_types::runtime::maybe_grow_stack(|| {
                    <FunctionDefinition as ::treesitter_types::FromNode>::from_node(node, src)
                })?,
            ))),
            "goto_statement" => Ok(Self::GotoStatement(::std::boxed::Box::new(
                ::treesitter_types::runtime::maybe_grow_stack(|| {
                    <GotoStatement as ::treesitter_types::FromNode>::from_node(node, src)
                })?,
            ))),
            "if_statement" => Ok(Self::IfStatement(::std::boxed::Box::new(
                ::treesitter_types::runtime::maybe_grow_stack(|| {
                    <IfStatement as ::treesitter_types::FromNode>::from_node(node, src)
                })?,
            ))),
            "labeled_statement" => Ok(Self::LabeledStatement(::std::boxed::Box::new(
                ::treesitter_types::runtime::maybe_grow_stack(|| {
                    <LabeledStatement as ::treesitter_types::FromNode>::from_node(node, src)
                })?,
            ))),
            "linkage_specification" => Ok(Self::LinkageSpecification(::std::boxed::Box::new(
                ::treesitter_types::runtime::maybe_grow_stack(|| {
                    <LinkageSpecification as ::treesitter_types::FromNode>::from_node(node, src)
                })?,
            ))),
            "preproc_call" => Ok(Self::PreprocCall(::std::boxed::Box::new(
                ::treesitter_types::runtime::maybe_grow_stack(|| {
                    <PreprocCall as ::treesitter_types::FromNode>::from_node(node, src)
                })?,
            ))),
            "preproc_def" => Ok(Self::PreprocDef(::std::boxed::Box::new(
                ::treesitter_types::runtime::maybe_grow_stack(|| {
                    <PreprocDef as ::treesitter_types::FromNode>::from_node(node, src)
                })?,
            ))),
            "preproc_function_def" => Ok(Self::PreprocFunctionDef(::std::boxed::Box::new(
                ::treesitter_types::runtime::maybe_grow_stack(|| {
                    <PreprocFunctionDef as ::treesitter_types::FromNode>::from_node(node, src)
                })?,
            ))),
            "preproc_if" => Ok(Self::PreprocIf(::std::boxed::Box::new(
                ::treesitter_types::runtime::maybe_grow_stack(|| {
                    <PreprocIf as ::treesitter_types::FromNode>::from_node(node, src)
                })?,
            ))),
            "preproc_ifdef" => Ok(Self::PreprocIfdef(::std::boxed::Box::new(
                ::treesitter_types::runtime::maybe_grow_stack(|| {
                    <PreprocIfdef as ::treesitter_types::FromNode>::from_node(node, src)
                })?,
            ))),
            "preproc_include" => Ok(Self::PreprocInclude(::std::boxed::Box::new(
                ::treesitter_types::runtime::maybe_grow_stack(|| {
                    <PreprocInclude as ::treesitter_types::FromNode>::from_node(node, src)
                })?,
            ))),
            "return_statement" => Ok(Self::ReturnStatement(::std::boxed::Box::new(
                ::treesitter_types::runtime::maybe_grow_stack(|| {
                    <ReturnStatement as ::treesitter_types::FromNode>::from_node(node, src)
                })?,
            ))),
            "switch_statement" => Ok(Self::SwitchStatement(::std::boxed::Box::new(
                ::treesitter_types::runtime::maybe_grow_stack(|| {
                    <SwitchStatement as ::treesitter_types::FromNode>::from_node(node, src)
                })?,
            ))),
            "type_definition" => Ok(Self::TypeDefinition(::std::boxed::Box::new(
                ::treesitter_types::runtime::maybe_grow_stack(|| {
                    <TypeDefinition as ::treesitter_types::FromNode>::from_node(node, src)
                })?,
            ))),
            "while_statement" => Ok(Self::WhileStatement(::std::boxed::Box::new(
                ::treesitter_types::runtime::maybe_grow_stack(|| {
                    <WhileStatement as ::treesitter_types::FromNode>::from_node(node, src)
                })?,
            ))),
            _other => {
                if let Ok(v) = ::treesitter_types::runtime::maybe_grow_stack(|| {
                    <TypeSpecifier as ::treesitter_types::FromNode>::from_node(node, src)
                }) {
                    Ok(Self::TypeSpecifier(::std::boxed::Box::new(v)))
                } else {
                    Err(::treesitter_types::ParseError::unexpected_kind(
                        _other, node,
                    ))
                }
            }
        }
    }
}
impl ::treesitter_types::Spanned for TranslationUnitChildren<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        match self {
            Self::AttributedStatement(inner) => inner.span(),
            Self::BreakStatement(inner) => inner.span(),
            Self::CaseStatement(inner) => inner.span(),
            Self::CompoundStatement(inner) => inner.span(),
            Self::ContinueStatement(inner) => inner.span(),
            Self::Declaration(inner) => inner.span(),
            Self::DoStatement(inner) => inner.span(),
            Self::ExpressionStatement(inner) => inner.span(),
            Self::ForStatement(inner) => inner.span(),
            Self::FunctionDefinition(inner) => inner.span(),
            Self::GotoStatement(inner) => inner.span(),
            Self::IfStatement(inner) => inner.span(),
            Self::LabeledStatement(inner) => inner.span(),
            Self::LinkageSpecification(inner) => inner.span(),
            Self::PreprocCall(inner) => inner.span(),
            Self::PreprocDef(inner) => inner.span(),
            Self::PreprocFunctionDef(inner) => inner.span(),
            Self::PreprocIf(inner) => inner.span(),
            Self::PreprocIfdef(inner) => inner.span(),
            Self::PreprocInclude(inner) => inner.span(),
            Self::ReturnStatement(inner) => inner.span(),
            Self::SwitchStatement(inner) => inner.span(),
            Self::TypeDefinition(inner) => inner.span(),
            Self::TypeSpecifier(inner) => inner.span(),
            Self::WhileStatement(inner) => inner.span(),
        }
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum TypeDefinitionChildren<'tree> {
    AttributeSpecifier(::std::boxed::Box<AttributeSpecifier<'tree>>),
    TypeQualifier(::std::boxed::Box<TypeQualifier<'tree>>),
}
impl<'tree> ::treesitter_types::FromNode<'tree> for TypeDefinitionChildren<'tree> {
    #[allow(clippy::collapsible_else_if)]
    fn from_node(
        node: ::treesitter_types::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        match node.kind() {
            "attribute_specifier" => Ok(Self::AttributeSpecifier(::std::boxed::Box::new(
                ::treesitter_types::runtime::maybe_grow_stack(|| {
                    <AttributeSpecifier as ::treesitter_types::FromNode>::from_node(node, src)
                })?,
            ))),
            "type_qualifier" => Ok(Self::TypeQualifier(::std::boxed::Box::new(
                ::treesitter_types::runtime::maybe_grow_stack(|| {
                    <TypeQualifier as ::treesitter_types::FromNode>::from_node(node, src)
                })?,
            ))),
            other => Err(::treesitter_types::ParseError::unexpected_kind(other, node)),
        }
    }
}
impl ::treesitter_types::Spanned for TypeDefinitionChildren<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        match self {
            Self::AttributeSpecifier(inner) => inner.span(),
            Self::TypeQualifier(inner) => inner.span(),
        }
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum UnaryExpressionArgument<'tree> {
    Expression(::std::boxed::Box<Expression<'tree>>),
    PreprocDefined(::std::boxed::Box<PreprocDefined<'tree>>),
}
impl<'tree> ::treesitter_types::FromNode<'tree> for UnaryExpressionArgument<'tree> {
    #[allow(clippy::collapsible_else_if)]
    fn from_node(
        node: ::treesitter_types::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        match node.kind() {
            "preproc_defined" => Ok(Self::PreprocDefined(::std::boxed::Box::new(
                ::treesitter_types::runtime::maybe_grow_stack(|| {
                    <PreprocDefined as ::treesitter_types::FromNode>::from_node(node, src)
                })?,
            ))),
            _other => {
                if let Ok(v) = ::treesitter_types::runtime::maybe_grow_stack(|| {
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
impl ::treesitter_types::Spanned for UnaryExpressionArgument<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        match self {
            Self::Expression(inner) => inner.span(),
            Self::PreprocDefined(inner) => inner.span(),
        }
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum UnaryExpressionOperator {
    Bang(::treesitter_types::Span),
    Plus(::treesitter_types::Span),
    Minus(::treesitter_types::Span),
    Tilde(::treesitter_types::Span),
}
impl<'tree> ::treesitter_types::FromNode<'tree> for UnaryExpressionOperator {
    #[allow(clippy::collapsible_else_if)]
    fn from_node(
        node: ::treesitter_types::tree_sitter::Node<'tree>,
        _src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        match node.kind() {
            "!" => Ok(Self::Bang(::treesitter_types::Span::from(node))),
            "+" => Ok(Self::Plus(::treesitter_types::Span::from(node))),
            "-" => Ok(Self::Minus(::treesitter_types::Span::from(node))),
            "~" => Ok(Self::Tilde(::treesitter_types::Span::from(node))),
            other => Err(::treesitter_types::ParseError::unexpected_kind(other, node)),
        }
    }
}
impl ::treesitter_types::Spanned for UnaryExpressionOperator {
    fn span(&self) -> ::treesitter_types::Span {
        match self {
            Self::Bang(span) => *span,
            Self::Plus(span) => *span,
            Self::Minus(span) => *span,
            Self::Tilde(span) => *span,
        }
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum UnionSpecifierChildren<'tree> {
    AttributeSpecifier(::std::boxed::Box<AttributeSpecifier<'tree>>),
    MsDeclspecModifier(::std::boxed::Box<MsDeclspecModifier<'tree>>),
}
impl<'tree> ::treesitter_types::FromNode<'tree> for UnionSpecifierChildren<'tree> {
    #[allow(clippy::collapsible_else_if)]
    fn from_node(
        node: ::treesitter_types::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        match node.kind() {
            "attribute_specifier" => Ok(Self::AttributeSpecifier(::std::boxed::Box::new(
                ::treesitter_types::runtime::maybe_grow_stack(|| {
                    <AttributeSpecifier as ::treesitter_types::FromNode>::from_node(node, src)
                })?,
            ))),
            "ms_declspec_modifier" => Ok(Self::MsDeclspecModifier(::std::boxed::Box::new(
                ::treesitter_types::runtime::maybe_grow_stack(|| {
                    <MsDeclspecModifier as ::treesitter_types::FromNode>::from_node(node, src)
                })?,
            ))),
            other => Err(::treesitter_types::ParseError::unexpected_kind(other, node)),
        }
    }
}
impl ::treesitter_types::Spanned for UnionSpecifierChildren<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        match self {
            Self::AttributeSpecifier(inner) => inner.span(),
            Self::MsDeclspecModifier(inner) => inner.span(),
        }
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum UpdateExpressionOperator {
    PlusPlus(::treesitter_types::Span),
    MinusMinus(::treesitter_types::Span),
}
impl<'tree> ::treesitter_types::FromNode<'tree> for UpdateExpressionOperator {
    #[allow(clippy::collapsible_else_if)]
    fn from_node(
        node: ::treesitter_types::tree_sitter::Node<'tree>,
        _src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        match node.kind() {
            "++" => Ok(Self::PlusPlus(::treesitter_types::Span::from(node))),
            "--" => Ok(Self::MinusMinus(::treesitter_types::Span::from(node))),
            other => Err(::treesitter_types::ParseError::unexpected_kind(other, node)),
        }
    }
}
impl ::treesitter_types::Spanned for UpdateExpressionOperator {
    fn span(&self) -> ::treesitter_types::Span {
        match self {
            Self::PlusPlus(span) => *span,
            Self::MinusMinus(span) => *span,
        }
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum AnyNode<'tree> {
    AbstractDeclarator(AbstractDeclarator<'tree>),
    Declarator(Declarator<'tree>),
    FieldDeclarator(FieldDeclarator<'tree>),
    TypeDeclarator(TypeDeclarator<'tree>),
    Expression(Expression<'tree>),
    Statement(Statement<'tree>),
    TypeSpecifier(TypeSpecifier<'tree>),
    AbstractArrayDeclarator(AbstractArrayDeclarator<'tree>),
    AbstractFunctionDeclarator(AbstractFunctionDeclarator<'tree>),
    AbstractParenthesizedDeclarator(AbstractParenthesizedDeclarator<'tree>),
    AbstractPointerDeclarator(AbstractPointerDeclarator<'tree>),
    AlignasQualifier(AlignasQualifier<'tree>),
    AlignofExpression(AlignofExpression<'tree>),
    ArgumentList(ArgumentList<'tree>),
    ArrayDeclarator(ArrayDeclarator<'tree>),
    AssignmentExpression(AssignmentExpression<'tree>),
    Attribute(Attribute<'tree>),
    AttributeDeclaration(AttributeDeclaration<'tree>),
    AttributeSpecifier(AttributeSpecifier<'tree>),
    AttributedDeclarator(AttributedDeclarator<'tree>),
    AttributedStatement(AttributedStatement<'tree>),
    BinaryExpression(BinaryExpression<'tree>),
    BitfieldClause(BitfieldClause<'tree>),
    BreakStatement(BreakStatement<'tree>),
    CallExpression(CallExpression<'tree>),
    CaseStatement(CaseStatement<'tree>),
    CastExpression(CastExpression<'tree>),
    CharLiteral(CharLiteral<'tree>),
    CommaExpression(CommaExpression<'tree>),
    CompoundLiteralExpression(CompoundLiteralExpression<'tree>),
    CompoundStatement(CompoundStatement<'tree>),
    ConcatenatedString(ConcatenatedString<'tree>),
    ConditionalExpression(ConditionalExpression<'tree>),
    ContinueStatement(ContinueStatement<'tree>),
    Declaration(Declaration<'tree>),
    DeclarationList(DeclarationList<'tree>),
    DoStatement(DoStatement<'tree>),
    ElseClause(ElseClause<'tree>),
    EnumSpecifier(EnumSpecifier<'tree>),
    Enumerator(Enumerator<'tree>),
    EnumeratorList(EnumeratorList<'tree>),
    ExpressionStatement(ExpressionStatement<'tree>),
    ExtensionExpression(ExtensionExpression<'tree>),
    FieldDeclaration(FieldDeclaration<'tree>),
    FieldDeclarationList(FieldDeclarationList<'tree>),
    FieldDesignator(FieldDesignator<'tree>),
    FieldExpression(FieldExpression<'tree>),
    ForStatement(ForStatement<'tree>),
    FunctionDeclarator(FunctionDeclarator<'tree>),
    FunctionDefinition(FunctionDefinition<'tree>),
    GenericExpression(GenericExpression<'tree>),
    GnuAsmClobberList(GnuAsmClobberList<'tree>),
    GnuAsmExpression(GnuAsmExpression<'tree>),
    GnuAsmGotoList(GnuAsmGotoList<'tree>),
    GnuAsmInputOperand(GnuAsmInputOperand<'tree>),
    GnuAsmInputOperandList(GnuAsmInputOperandList<'tree>),
    GnuAsmOutputOperand(GnuAsmOutputOperand<'tree>),
    GnuAsmOutputOperandList(GnuAsmOutputOperandList<'tree>),
    GnuAsmQualifier(GnuAsmQualifier<'tree>),
    GotoStatement(GotoStatement<'tree>),
    IfStatement(IfStatement<'tree>),
    InitDeclarator(InitDeclarator<'tree>),
    InitializerList(InitializerList<'tree>),
    InitializerPair(InitializerPair<'tree>),
    LabeledStatement(LabeledStatement<'tree>),
    LinkageSpecification(LinkageSpecification<'tree>),
    MacroTypeSpecifier(MacroTypeSpecifier<'tree>),
    MsBasedModifier(MsBasedModifier<'tree>),
    MsCallModifier(MsCallModifier<'tree>),
    MsDeclspecModifier(MsDeclspecModifier<'tree>),
    MsPointerModifier(MsPointerModifier<'tree>),
    MsUnalignedPtrModifier(MsUnalignedPtrModifier<'tree>),
    Null(Null<'tree>),
    OffsetofExpression(OffsetofExpression<'tree>),
    ParameterDeclaration(ParameterDeclaration<'tree>),
    ParameterList(ParameterList<'tree>),
    ParenthesizedDeclarator(ParenthesizedDeclarator<'tree>),
    ParenthesizedExpression(ParenthesizedExpression<'tree>),
    PointerDeclarator(PointerDeclarator<'tree>),
    PointerExpression(PointerExpression<'tree>),
    PreprocCall(PreprocCall<'tree>),
    PreprocDef(PreprocDef<'tree>),
    PreprocDefined(PreprocDefined<'tree>),
    PreprocElif(PreprocElif<'tree>),
    PreprocElifdef(PreprocElifdef<'tree>),
    PreprocElse(PreprocElse<'tree>),
    PreprocFunctionDef(PreprocFunctionDef<'tree>),
    PreprocIf(PreprocIf<'tree>),
    PreprocIfdef(PreprocIfdef<'tree>),
    PreprocInclude(PreprocInclude<'tree>),
    PreprocParams(PreprocParams<'tree>),
    ReturnStatement(ReturnStatement<'tree>),
    SehExceptClause(SehExceptClause<'tree>),
    SehFinallyClause(SehFinallyClause<'tree>),
    SehLeaveStatement(SehLeaveStatement<'tree>),
    SehTryStatement(SehTryStatement<'tree>),
    SizedTypeSpecifier(SizedTypeSpecifier<'tree>),
    SizeofExpression(SizeofExpression<'tree>),
    StorageClassSpecifier(StorageClassSpecifier<'tree>),
    StringLiteral(StringLiteral<'tree>),
    StructSpecifier(StructSpecifier<'tree>),
    SubscriptDesignator(SubscriptDesignator<'tree>),
    SubscriptExpression(SubscriptExpression<'tree>),
    SubscriptRangeDesignator(SubscriptRangeDesignator<'tree>),
    SwitchStatement(SwitchStatement<'tree>),
    TranslationUnit(TranslationUnit<'tree>),
    TypeDefinition(TypeDefinition<'tree>),
    TypeDescriptor(TypeDescriptor<'tree>),
    TypeQualifier(TypeQualifier<'tree>),
    UnaryExpression(UnaryExpression<'tree>),
    UnionSpecifier(UnionSpecifier<'tree>),
    UpdateExpression(UpdateExpression<'tree>),
    VariadicParameter(VariadicParameter<'tree>),
    WhileStatement(WhileStatement<'tree>),
    Character(Character<'tree>),
    Comment(Comment<'tree>),
    EscapeSequence(EscapeSequence<'tree>),
    False(False<'tree>),
    FieldIdentifier(FieldIdentifier<'tree>),
    Identifier(Identifier<'tree>),
    MsRestrictModifier(MsRestrictModifier<'tree>),
    MsSignedPtrModifier(MsSignedPtrModifier<'tree>),
    MsUnsignedPtrModifier(MsUnsignedPtrModifier<'tree>),
    NumberLiteral(NumberLiteral<'tree>),
    PreprocArg(PreprocArg<'tree>),
    PreprocDirective(PreprocDirective<'tree>),
    PrimitiveType(PrimitiveType<'tree>),
    StatementIdentifier(StatementIdentifier<'tree>),
    StringContent(StringContent<'tree>),
    SystemLibString(SystemLibString<'tree>),
    True(True<'tree>),
    TypeIdentifier(TypeIdentifier<'tree>),
    Unknown(::treesitter_types::tree_sitter::Node<'tree>),
}
impl<'tree> AnyNode<'tree> {
    pub fn from_node(node: ::treesitter_types::tree_sitter::Node<'tree>, src: &'tree [u8]) -> Self {
        match node.kind() {
            "_abstract_declarator" => ::treesitter_types::runtime::maybe_grow_stack(|| {
                <AbstractDeclarator as ::treesitter_types::FromNode>::from_node(node, src)
            })
            .map(Self::AbstractDeclarator)
            .unwrap_or(Self::Unknown(node)),
            "_declarator" => ::treesitter_types::runtime::maybe_grow_stack(|| {
                <Declarator as ::treesitter_types::FromNode>::from_node(node, src)
            })
            .map(Self::Declarator)
            .unwrap_or(Self::Unknown(node)),
            "_field_declarator" => ::treesitter_types::runtime::maybe_grow_stack(|| {
                <FieldDeclarator as ::treesitter_types::FromNode>::from_node(node, src)
            })
            .map(Self::FieldDeclarator)
            .unwrap_or(Self::Unknown(node)),
            "_type_declarator" => ::treesitter_types::runtime::maybe_grow_stack(|| {
                <TypeDeclarator as ::treesitter_types::FromNode>::from_node(node, src)
            })
            .map(Self::TypeDeclarator)
            .unwrap_or(Self::Unknown(node)),
            "expression" => ::treesitter_types::runtime::maybe_grow_stack(|| {
                <Expression as ::treesitter_types::FromNode>::from_node(node, src)
            })
            .map(Self::Expression)
            .unwrap_or(Self::Unknown(node)),
            "statement" => ::treesitter_types::runtime::maybe_grow_stack(|| {
                <Statement as ::treesitter_types::FromNode>::from_node(node, src)
            })
            .map(Self::Statement)
            .unwrap_or(Self::Unknown(node)),
            "type_specifier" => ::treesitter_types::runtime::maybe_grow_stack(|| {
                <TypeSpecifier as ::treesitter_types::FromNode>::from_node(node, src)
            })
            .map(Self::TypeSpecifier)
            .unwrap_or(Self::Unknown(node)),
            "abstract_array_declarator" => ::treesitter_types::runtime::maybe_grow_stack(|| {
                <AbstractArrayDeclarator as ::treesitter_types::FromNode>::from_node(node, src)
            })
            .map(Self::AbstractArrayDeclarator)
            .unwrap_or(Self::Unknown(node)),
            "abstract_function_declarator" => ::treesitter_types::runtime::maybe_grow_stack(|| {
                <AbstractFunctionDeclarator as ::treesitter_types::FromNode>::from_node(node, src)
            })
            .map(Self::AbstractFunctionDeclarator)
            .unwrap_or(Self::Unknown(node)),
            "abstract_parenthesized_declarator" => {
                ::treesitter_types::runtime::maybe_grow_stack(|| {
                    <AbstractParenthesizedDeclarator as ::treesitter_types::FromNode>::from_node(
                        node, src,
                    )
                })
                .map(Self::AbstractParenthesizedDeclarator)
                .unwrap_or(Self::Unknown(node))
            }
            "abstract_pointer_declarator" => ::treesitter_types::runtime::maybe_grow_stack(|| {
                <AbstractPointerDeclarator as ::treesitter_types::FromNode>::from_node(node, src)
            })
            .map(Self::AbstractPointerDeclarator)
            .unwrap_or(Self::Unknown(node)),
            "alignas_qualifier" => ::treesitter_types::runtime::maybe_grow_stack(|| {
                <AlignasQualifier as ::treesitter_types::FromNode>::from_node(node, src)
            })
            .map(Self::AlignasQualifier)
            .unwrap_or(Self::Unknown(node)),
            "alignof_expression" => ::treesitter_types::runtime::maybe_grow_stack(|| {
                <AlignofExpression as ::treesitter_types::FromNode>::from_node(node, src)
            })
            .map(Self::AlignofExpression)
            .unwrap_or(Self::Unknown(node)),
            "argument_list" => ::treesitter_types::runtime::maybe_grow_stack(|| {
                <ArgumentList as ::treesitter_types::FromNode>::from_node(node, src)
            })
            .map(Self::ArgumentList)
            .unwrap_or(Self::Unknown(node)),
            "array_declarator" => ::treesitter_types::runtime::maybe_grow_stack(|| {
                <ArrayDeclarator as ::treesitter_types::FromNode>::from_node(node, src)
            })
            .map(Self::ArrayDeclarator)
            .unwrap_or(Self::Unknown(node)),
            "assignment_expression" => ::treesitter_types::runtime::maybe_grow_stack(|| {
                <AssignmentExpression as ::treesitter_types::FromNode>::from_node(node, src)
            })
            .map(Self::AssignmentExpression)
            .unwrap_or(Self::Unknown(node)),
            "attribute" => ::treesitter_types::runtime::maybe_grow_stack(|| {
                <Attribute as ::treesitter_types::FromNode>::from_node(node, src)
            })
            .map(Self::Attribute)
            .unwrap_or(Self::Unknown(node)),
            "attribute_declaration" => ::treesitter_types::runtime::maybe_grow_stack(|| {
                <AttributeDeclaration as ::treesitter_types::FromNode>::from_node(node, src)
            })
            .map(Self::AttributeDeclaration)
            .unwrap_or(Self::Unknown(node)),
            "attribute_specifier" => ::treesitter_types::runtime::maybe_grow_stack(|| {
                <AttributeSpecifier as ::treesitter_types::FromNode>::from_node(node, src)
            })
            .map(Self::AttributeSpecifier)
            .unwrap_or(Self::Unknown(node)),
            "attributed_declarator" => ::treesitter_types::runtime::maybe_grow_stack(|| {
                <AttributedDeclarator as ::treesitter_types::FromNode>::from_node(node, src)
            })
            .map(Self::AttributedDeclarator)
            .unwrap_or(Self::Unknown(node)),
            "attributed_statement" => ::treesitter_types::runtime::maybe_grow_stack(|| {
                <AttributedStatement as ::treesitter_types::FromNode>::from_node(node, src)
            })
            .map(Self::AttributedStatement)
            .unwrap_or(Self::Unknown(node)),
            "binary_expression" => ::treesitter_types::runtime::maybe_grow_stack(|| {
                <BinaryExpression as ::treesitter_types::FromNode>::from_node(node, src)
            })
            .map(Self::BinaryExpression)
            .unwrap_or(Self::Unknown(node)),
            "bitfield_clause" => ::treesitter_types::runtime::maybe_grow_stack(|| {
                <BitfieldClause as ::treesitter_types::FromNode>::from_node(node, src)
            })
            .map(Self::BitfieldClause)
            .unwrap_or(Self::Unknown(node)),
            "break_statement" => ::treesitter_types::runtime::maybe_grow_stack(|| {
                <BreakStatement as ::treesitter_types::FromNode>::from_node(node, src)
            })
            .map(Self::BreakStatement)
            .unwrap_or(Self::Unknown(node)),
            "call_expression" => ::treesitter_types::runtime::maybe_grow_stack(|| {
                <CallExpression as ::treesitter_types::FromNode>::from_node(node, src)
            })
            .map(Self::CallExpression)
            .unwrap_or(Self::Unknown(node)),
            "case_statement" => ::treesitter_types::runtime::maybe_grow_stack(|| {
                <CaseStatement as ::treesitter_types::FromNode>::from_node(node, src)
            })
            .map(Self::CaseStatement)
            .unwrap_or(Self::Unknown(node)),
            "cast_expression" => ::treesitter_types::runtime::maybe_grow_stack(|| {
                <CastExpression as ::treesitter_types::FromNode>::from_node(node, src)
            })
            .map(Self::CastExpression)
            .unwrap_or(Self::Unknown(node)),
            "char_literal" => ::treesitter_types::runtime::maybe_grow_stack(|| {
                <CharLiteral as ::treesitter_types::FromNode>::from_node(node, src)
            })
            .map(Self::CharLiteral)
            .unwrap_or(Self::Unknown(node)),
            "comma_expression" => ::treesitter_types::runtime::maybe_grow_stack(|| {
                <CommaExpression as ::treesitter_types::FromNode>::from_node(node, src)
            })
            .map(Self::CommaExpression)
            .unwrap_or(Self::Unknown(node)),
            "compound_literal_expression" => ::treesitter_types::runtime::maybe_grow_stack(|| {
                <CompoundLiteralExpression as ::treesitter_types::FromNode>::from_node(node, src)
            })
            .map(Self::CompoundLiteralExpression)
            .unwrap_or(Self::Unknown(node)),
            "compound_statement" => ::treesitter_types::runtime::maybe_grow_stack(|| {
                <CompoundStatement as ::treesitter_types::FromNode>::from_node(node, src)
            })
            .map(Self::CompoundStatement)
            .unwrap_or(Self::Unknown(node)),
            "concatenated_string" => ::treesitter_types::runtime::maybe_grow_stack(|| {
                <ConcatenatedString as ::treesitter_types::FromNode>::from_node(node, src)
            })
            .map(Self::ConcatenatedString)
            .unwrap_or(Self::Unknown(node)),
            "conditional_expression" => ::treesitter_types::runtime::maybe_grow_stack(|| {
                <ConditionalExpression as ::treesitter_types::FromNode>::from_node(node, src)
            })
            .map(Self::ConditionalExpression)
            .unwrap_or(Self::Unknown(node)),
            "continue_statement" => ::treesitter_types::runtime::maybe_grow_stack(|| {
                <ContinueStatement as ::treesitter_types::FromNode>::from_node(node, src)
            })
            .map(Self::ContinueStatement)
            .unwrap_or(Self::Unknown(node)),
            "declaration" => ::treesitter_types::runtime::maybe_grow_stack(|| {
                <Declaration as ::treesitter_types::FromNode>::from_node(node, src)
            })
            .map(Self::Declaration)
            .unwrap_or(Self::Unknown(node)),
            "declaration_list" => ::treesitter_types::runtime::maybe_grow_stack(|| {
                <DeclarationList as ::treesitter_types::FromNode>::from_node(node, src)
            })
            .map(Self::DeclarationList)
            .unwrap_or(Self::Unknown(node)),
            "do_statement" => ::treesitter_types::runtime::maybe_grow_stack(|| {
                <DoStatement as ::treesitter_types::FromNode>::from_node(node, src)
            })
            .map(Self::DoStatement)
            .unwrap_or(Self::Unknown(node)),
            "else_clause" => ::treesitter_types::runtime::maybe_grow_stack(|| {
                <ElseClause as ::treesitter_types::FromNode>::from_node(node, src)
            })
            .map(Self::ElseClause)
            .unwrap_or(Self::Unknown(node)),
            "enum_specifier" => ::treesitter_types::runtime::maybe_grow_stack(|| {
                <EnumSpecifier as ::treesitter_types::FromNode>::from_node(node, src)
            })
            .map(Self::EnumSpecifier)
            .unwrap_or(Self::Unknown(node)),
            "enumerator" => ::treesitter_types::runtime::maybe_grow_stack(|| {
                <Enumerator as ::treesitter_types::FromNode>::from_node(node, src)
            })
            .map(Self::Enumerator)
            .unwrap_or(Self::Unknown(node)),
            "enumerator_list" => ::treesitter_types::runtime::maybe_grow_stack(|| {
                <EnumeratorList as ::treesitter_types::FromNode>::from_node(node, src)
            })
            .map(Self::EnumeratorList)
            .unwrap_or(Self::Unknown(node)),
            "expression_statement" => ::treesitter_types::runtime::maybe_grow_stack(|| {
                <ExpressionStatement as ::treesitter_types::FromNode>::from_node(node, src)
            })
            .map(Self::ExpressionStatement)
            .unwrap_or(Self::Unknown(node)),
            "extension_expression" => ::treesitter_types::runtime::maybe_grow_stack(|| {
                <ExtensionExpression as ::treesitter_types::FromNode>::from_node(node, src)
            })
            .map(Self::ExtensionExpression)
            .unwrap_or(Self::Unknown(node)),
            "field_declaration" => ::treesitter_types::runtime::maybe_grow_stack(|| {
                <FieldDeclaration as ::treesitter_types::FromNode>::from_node(node, src)
            })
            .map(Self::FieldDeclaration)
            .unwrap_or(Self::Unknown(node)),
            "field_declaration_list" => ::treesitter_types::runtime::maybe_grow_stack(|| {
                <FieldDeclarationList as ::treesitter_types::FromNode>::from_node(node, src)
            })
            .map(Self::FieldDeclarationList)
            .unwrap_or(Self::Unknown(node)),
            "field_designator" => ::treesitter_types::runtime::maybe_grow_stack(|| {
                <FieldDesignator as ::treesitter_types::FromNode>::from_node(node, src)
            })
            .map(Self::FieldDesignator)
            .unwrap_or(Self::Unknown(node)),
            "field_expression" => ::treesitter_types::runtime::maybe_grow_stack(|| {
                <FieldExpression as ::treesitter_types::FromNode>::from_node(node, src)
            })
            .map(Self::FieldExpression)
            .unwrap_or(Self::Unknown(node)),
            "for_statement" => ::treesitter_types::runtime::maybe_grow_stack(|| {
                <ForStatement as ::treesitter_types::FromNode>::from_node(node, src)
            })
            .map(Self::ForStatement)
            .unwrap_or(Self::Unknown(node)),
            "function_declarator" => ::treesitter_types::runtime::maybe_grow_stack(|| {
                <FunctionDeclarator as ::treesitter_types::FromNode>::from_node(node, src)
            })
            .map(Self::FunctionDeclarator)
            .unwrap_or(Self::Unknown(node)),
            "function_definition" => ::treesitter_types::runtime::maybe_grow_stack(|| {
                <FunctionDefinition as ::treesitter_types::FromNode>::from_node(node, src)
            })
            .map(Self::FunctionDefinition)
            .unwrap_or(Self::Unknown(node)),
            "generic_expression" => ::treesitter_types::runtime::maybe_grow_stack(|| {
                <GenericExpression as ::treesitter_types::FromNode>::from_node(node, src)
            })
            .map(Self::GenericExpression)
            .unwrap_or(Self::Unknown(node)),
            "gnu_asm_clobber_list" => ::treesitter_types::runtime::maybe_grow_stack(|| {
                <GnuAsmClobberList as ::treesitter_types::FromNode>::from_node(node, src)
            })
            .map(Self::GnuAsmClobberList)
            .unwrap_or(Self::Unknown(node)),
            "gnu_asm_expression" => ::treesitter_types::runtime::maybe_grow_stack(|| {
                <GnuAsmExpression as ::treesitter_types::FromNode>::from_node(node, src)
            })
            .map(Self::GnuAsmExpression)
            .unwrap_or(Self::Unknown(node)),
            "gnu_asm_goto_list" => ::treesitter_types::runtime::maybe_grow_stack(|| {
                <GnuAsmGotoList as ::treesitter_types::FromNode>::from_node(node, src)
            })
            .map(Self::GnuAsmGotoList)
            .unwrap_or(Self::Unknown(node)),
            "gnu_asm_input_operand" => ::treesitter_types::runtime::maybe_grow_stack(|| {
                <GnuAsmInputOperand as ::treesitter_types::FromNode>::from_node(node, src)
            })
            .map(Self::GnuAsmInputOperand)
            .unwrap_or(Self::Unknown(node)),
            "gnu_asm_input_operand_list" => ::treesitter_types::runtime::maybe_grow_stack(|| {
                <GnuAsmInputOperandList as ::treesitter_types::FromNode>::from_node(node, src)
            })
            .map(Self::GnuAsmInputOperandList)
            .unwrap_or(Self::Unknown(node)),
            "gnu_asm_output_operand" => ::treesitter_types::runtime::maybe_grow_stack(|| {
                <GnuAsmOutputOperand as ::treesitter_types::FromNode>::from_node(node, src)
            })
            .map(Self::GnuAsmOutputOperand)
            .unwrap_or(Self::Unknown(node)),
            "gnu_asm_output_operand_list" => ::treesitter_types::runtime::maybe_grow_stack(|| {
                <GnuAsmOutputOperandList as ::treesitter_types::FromNode>::from_node(node, src)
            })
            .map(Self::GnuAsmOutputOperandList)
            .unwrap_or(Self::Unknown(node)),
            "gnu_asm_qualifier" => ::treesitter_types::runtime::maybe_grow_stack(|| {
                <GnuAsmQualifier as ::treesitter_types::FromNode>::from_node(node, src)
            })
            .map(Self::GnuAsmQualifier)
            .unwrap_or(Self::Unknown(node)),
            "goto_statement" => ::treesitter_types::runtime::maybe_grow_stack(|| {
                <GotoStatement as ::treesitter_types::FromNode>::from_node(node, src)
            })
            .map(Self::GotoStatement)
            .unwrap_or(Self::Unknown(node)),
            "if_statement" => ::treesitter_types::runtime::maybe_grow_stack(|| {
                <IfStatement as ::treesitter_types::FromNode>::from_node(node, src)
            })
            .map(Self::IfStatement)
            .unwrap_or(Self::Unknown(node)),
            "init_declarator" => ::treesitter_types::runtime::maybe_grow_stack(|| {
                <InitDeclarator as ::treesitter_types::FromNode>::from_node(node, src)
            })
            .map(Self::InitDeclarator)
            .unwrap_or(Self::Unknown(node)),
            "initializer_list" => ::treesitter_types::runtime::maybe_grow_stack(|| {
                <InitializerList as ::treesitter_types::FromNode>::from_node(node, src)
            })
            .map(Self::InitializerList)
            .unwrap_or(Self::Unknown(node)),
            "initializer_pair" => ::treesitter_types::runtime::maybe_grow_stack(|| {
                <InitializerPair as ::treesitter_types::FromNode>::from_node(node, src)
            })
            .map(Self::InitializerPair)
            .unwrap_or(Self::Unknown(node)),
            "labeled_statement" => ::treesitter_types::runtime::maybe_grow_stack(|| {
                <LabeledStatement as ::treesitter_types::FromNode>::from_node(node, src)
            })
            .map(Self::LabeledStatement)
            .unwrap_or(Self::Unknown(node)),
            "linkage_specification" => ::treesitter_types::runtime::maybe_grow_stack(|| {
                <LinkageSpecification as ::treesitter_types::FromNode>::from_node(node, src)
            })
            .map(Self::LinkageSpecification)
            .unwrap_or(Self::Unknown(node)),
            "macro_type_specifier" => ::treesitter_types::runtime::maybe_grow_stack(|| {
                <MacroTypeSpecifier as ::treesitter_types::FromNode>::from_node(node, src)
            })
            .map(Self::MacroTypeSpecifier)
            .unwrap_or(Self::Unknown(node)),
            "ms_based_modifier" => ::treesitter_types::runtime::maybe_grow_stack(|| {
                <MsBasedModifier as ::treesitter_types::FromNode>::from_node(node, src)
            })
            .map(Self::MsBasedModifier)
            .unwrap_or(Self::Unknown(node)),
            "ms_call_modifier" => ::treesitter_types::runtime::maybe_grow_stack(|| {
                <MsCallModifier as ::treesitter_types::FromNode>::from_node(node, src)
            })
            .map(Self::MsCallModifier)
            .unwrap_or(Self::Unknown(node)),
            "ms_declspec_modifier" => ::treesitter_types::runtime::maybe_grow_stack(|| {
                <MsDeclspecModifier as ::treesitter_types::FromNode>::from_node(node, src)
            })
            .map(Self::MsDeclspecModifier)
            .unwrap_or(Self::Unknown(node)),
            "ms_pointer_modifier" => ::treesitter_types::runtime::maybe_grow_stack(|| {
                <MsPointerModifier as ::treesitter_types::FromNode>::from_node(node, src)
            })
            .map(Self::MsPointerModifier)
            .unwrap_or(Self::Unknown(node)),
            "ms_unaligned_ptr_modifier" => ::treesitter_types::runtime::maybe_grow_stack(|| {
                <MsUnalignedPtrModifier as ::treesitter_types::FromNode>::from_node(node, src)
            })
            .map(Self::MsUnalignedPtrModifier)
            .unwrap_or(Self::Unknown(node)),
            "null" => ::treesitter_types::runtime::maybe_grow_stack(|| {
                <Null as ::treesitter_types::FromNode>::from_node(node, src)
            })
            .map(Self::Null)
            .unwrap_or(Self::Unknown(node)),
            "offsetof_expression" => ::treesitter_types::runtime::maybe_grow_stack(|| {
                <OffsetofExpression as ::treesitter_types::FromNode>::from_node(node, src)
            })
            .map(Self::OffsetofExpression)
            .unwrap_or(Self::Unknown(node)),
            "parameter_declaration" => ::treesitter_types::runtime::maybe_grow_stack(|| {
                <ParameterDeclaration as ::treesitter_types::FromNode>::from_node(node, src)
            })
            .map(Self::ParameterDeclaration)
            .unwrap_or(Self::Unknown(node)),
            "parameter_list" => ::treesitter_types::runtime::maybe_grow_stack(|| {
                <ParameterList as ::treesitter_types::FromNode>::from_node(node, src)
            })
            .map(Self::ParameterList)
            .unwrap_or(Self::Unknown(node)),
            "parenthesized_declarator" => ::treesitter_types::runtime::maybe_grow_stack(|| {
                <ParenthesizedDeclarator as ::treesitter_types::FromNode>::from_node(node, src)
            })
            .map(Self::ParenthesizedDeclarator)
            .unwrap_or(Self::Unknown(node)),
            "parenthesized_expression" => ::treesitter_types::runtime::maybe_grow_stack(|| {
                <ParenthesizedExpression as ::treesitter_types::FromNode>::from_node(node, src)
            })
            .map(Self::ParenthesizedExpression)
            .unwrap_or(Self::Unknown(node)),
            "pointer_declarator" => ::treesitter_types::runtime::maybe_grow_stack(|| {
                <PointerDeclarator as ::treesitter_types::FromNode>::from_node(node, src)
            })
            .map(Self::PointerDeclarator)
            .unwrap_or(Self::Unknown(node)),
            "pointer_expression" => ::treesitter_types::runtime::maybe_grow_stack(|| {
                <PointerExpression as ::treesitter_types::FromNode>::from_node(node, src)
            })
            .map(Self::PointerExpression)
            .unwrap_or(Self::Unknown(node)),
            "preproc_call" => ::treesitter_types::runtime::maybe_grow_stack(|| {
                <PreprocCall as ::treesitter_types::FromNode>::from_node(node, src)
            })
            .map(Self::PreprocCall)
            .unwrap_or(Self::Unknown(node)),
            "preproc_def" => ::treesitter_types::runtime::maybe_grow_stack(|| {
                <PreprocDef as ::treesitter_types::FromNode>::from_node(node, src)
            })
            .map(Self::PreprocDef)
            .unwrap_or(Self::Unknown(node)),
            "preproc_defined" => ::treesitter_types::runtime::maybe_grow_stack(|| {
                <PreprocDefined as ::treesitter_types::FromNode>::from_node(node, src)
            })
            .map(Self::PreprocDefined)
            .unwrap_or(Self::Unknown(node)),
            "preproc_elif" => ::treesitter_types::runtime::maybe_grow_stack(|| {
                <PreprocElif as ::treesitter_types::FromNode>::from_node(node, src)
            })
            .map(Self::PreprocElif)
            .unwrap_or(Self::Unknown(node)),
            "preproc_elifdef" => ::treesitter_types::runtime::maybe_grow_stack(|| {
                <PreprocElifdef as ::treesitter_types::FromNode>::from_node(node, src)
            })
            .map(Self::PreprocElifdef)
            .unwrap_or(Self::Unknown(node)),
            "preproc_else" => ::treesitter_types::runtime::maybe_grow_stack(|| {
                <PreprocElse as ::treesitter_types::FromNode>::from_node(node, src)
            })
            .map(Self::PreprocElse)
            .unwrap_or(Self::Unknown(node)),
            "preproc_function_def" => ::treesitter_types::runtime::maybe_grow_stack(|| {
                <PreprocFunctionDef as ::treesitter_types::FromNode>::from_node(node, src)
            })
            .map(Self::PreprocFunctionDef)
            .unwrap_or(Self::Unknown(node)),
            "preproc_if" => ::treesitter_types::runtime::maybe_grow_stack(|| {
                <PreprocIf as ::treesitter_types::FromNode>::from_node(node, src)
            })
            .map(Self::PreprocIf)
            .unwrap_or(Self::Unknown(node)),
            "preproc_ifdef" => ::treesitter_types::runtime::maybe_grow_stack(|| {
                <PreprocIfdef as ::treesitter_types::FromNode>::from_node(node, src)
            })
            .map(Self::PreprocIfdef)
            .unwrap_or(Self::Unknown(node)),
            "preproc_include" => ::treesitter_types::runtime::maybe_grow_stack(|| {
                <PreprocInclude as ::treesitter_types::FromNode>::from_node(node, src)
            })
            .map(Self::PreprocInclude)
            .unwrap_or(Self::Unknown(node)),
            "preproc_params" => ::treesitter_types::runtime::maybe_grow_stack(|| {
                <PreprocParams as ::treesitter_types::FromNode>::from_node(node, src)
            })
            .map(Self::PreprocParams)
            .unwrap_or(Self::Unknown(node)),
            "return_statement" => ::treesitter_types::runtime::maybe_grow_stack(|| {
                <ReturnStatement as ::treesitter_types::FromNode>::from_node(node, src)
            })
            .map(Self::ReturnStatement)
            .unwrap_or(Self::Unknown(node)),
            "seh_except_clause" => ::treesitter_types::runtime::maybe_grow_stack(|| {
                <SehExceptClause as ::treesitter_types::FromNode>::from_node(node, src)
            })
            .map(Self::SehExceptClause)
            .unwrap_or(Self::Unknown(node)),
            "seh_finally_clause" => ::treesitter_types::runtime::maybe_grow_stack(|| {
                <SehFinallyClause as ::treesitter_types::FromNode>::from_node(node, src)
            })
            .map(Self::SehFinallyClause)
            .unwrap_or(Self::Unknown(node)),
            "seh_leave_statement" => ::treesitter_types::runtime::maybe_grow_stack(|| {
                <SehLeaveStatement as ::treesitter_types::FromNode>::from_node(node, src)
            })
            .map(Self::SehLeaveStatement)
            .unwrap_or(Self::Unknown(node)),
            "seh_try_statement" => ::treesitter_types::runtime::maybe_grow_stack(|| {
                <SehTryStatement as ::treesitter_types::FromNode>::from_node(node, src)
            })
            .map(Self::SehTryStatement)
            .unwrap_or(Self::Unknown(node)),
            "sized_type_specifier" => ::treesitter_types::runtime::maybe_grow_stack(|| {
                <SizedTypeSpecifier as ::treesitter_types::FromNode>::from_node(node, src)
            })
            .map(Self::SizedTypeSpecifier)
            .unwrap_or(Self::Unknown(node)),
            "sizeof_expression" => ::treesitter_types::runtime::maybe_grow_stack(|| {
                <SizeofExpression as ::treesitter_types::FromNode>::from_node(node, src)
            })
            .map(Self::SizeofExpression)
            .unwrap_or(Self::Unknown(node)),
            "storage_class_specifier" => ::treesitter_types::runtime::maybe_grow_stack(|| {
                <StorageClassSpecifier as ::treesitter_types::FromNode>::from_node(node, src)
            })
            .map(Self::StorageClassSpecifier)
            .unwrap_or(Self::Unknown(node)),
            "string_literal" => ::treesitter_types::runtime::maybe_grow_stack(|| {
                <StringLiteral as ::treesitter_types::FromNode>::from_node(node, src)
            })
            .map(Self::StringLiteral)
            .unwrap_or(Self::Unknown(node)),
            "struct_specifier" => ::treesitter_types::runtime::maybe_grow_stack(|| {
                <StructSpecifier as ::treesitter_types::FromNode>::from_node(node, src)
            })
            .map(Self::StructSpecifier)
            .unwrap_or(Self::Unknown(node)),
            "subscript_designator" => ::treesitter_types::runtime::maybe_grow_stack(|| {
                <SubscriptDesignator as ::treesitter_types::FromNode>::from_node(node, src)
            })
            .map(Self::SubscriptDesignator)
            .unwrap_or(Self::Unknown(node)),
            "subscript_expression" => ::treesitter_types::runtime::maybe_grow_stack(|| {
                <SubscriptExpression as ::treesitter_types::FromNode>::from_node(node, src)
            })
            .map(Self::SubscriptExpression)
            .unwrap_or(Self::Unknown(node)),
            "subscript_range_designator" => ::treesitter_types::runtime::maybe_grow_stack(|| {
                <SubscriptRangeDesignator as ::treesitter_types::FromNode>::from_node(node, src)
            })
            .map(Self::SubscriptRangeDesignator)
            .unwrap_or(Self::Unknown(node)),
            "switch_statement" => ::treesitter_types::runtime::maybe_grow_stack(|| {
                <SwitchStatement as ::treesitter_types::FromNode>::from_node(node, src)
            })
            .map(Self::SwitchStatement)
            .unwrap_or(Self::Unknown(node)),
            "translation_unit" => ::treesitter_types::runtime::maybe_grow_stack(|| {
                <TranslationUnit as ::treesitter_types::FromNode>::from_node(node, src)
            })
            .map(Self::TranslationUnit)
            .unwrap_or(Self::Unknown(node)),
            "type_definition" => ::treesitter_types::runtime::maybe_grow_stack(|| {
                <TypeDefinition as ::treesitter_types::FromNode>::from_node(node, src)
            })
            .map(Self::TypeDefinition)
            .unwrap_or(Self::Unknown(node)),
            "type_descriptor" => ::treesitter_types::runtime::maybe_grow_stack(|| {
                <TypeDescriptor as ::treesitter_types::FromNode>::from_node(node, src)
            })
            .map(Self::TypeDescriptor)
            .unwrap_or(Self::Unknown(node)),
            "type_qualifier" => ::treesitter_types::runtime::maybe_grow_stack(|| {
                <TypeQualifier as ::treesitter_types::FromNode>::from_node(node, src)
            })
            .map(Self::TypeQualifier)
            .unwrap_or(Self::Unknown(node)),
            "unary_expression" => ::treesitter_types::runtime::maybe_grow_stack(|| {
                <UnaryExpression as ::treesitter_types::FromNode>::from_node(node, src)
            })
            .map(Self::UnaryExpression)
            .unwrap_or(Self::Unknown(node)),
            "union_specifier" => ::treesitter_types::runtime::maybe_grow_stack(|| {
                <UnionSpecifier as ::treesitter_types::FromNode>::from_node(node, src)
            })
            .map(Self::UnionSpecifier)
            .unwrap_or(Self::Unknown(node)),
            "update_expression" => ::treesitter_types::runtime::maybe_grow_stack(|| {
                <UpdateExpression as ::treesitter_types::FromNode>::from_node(node, src)
            })
            .map(Self::UpdateExpression)
            .unwrap_or(Self::Unknown(node)),
            "variadic_parameter" => ::treesitter_types::runtime::maybe_grow_stack(|| {
                <VariadicParameter as ::treesitter_types::FromNode>::from_node(node, src)
            })
            .map(Self::VariadicParameter)
            .unwrap_or(Self::Unknown(node)),
            "while_statement" => ::treesitter_types::runtime::maybe_grow_stack(|| {
                <WhileStatement as ::treesitter_types::FromNode>::from_node(node, src)
            })
            .map(Self::WhileStatement)
            .unwrap_or(Self::Unknown(node)),
            "character" => ::treesitter_types::runtime::maybe_grow_stack(|| {
                <Character as ::treesitter_types::FromNode>::from_node(node, src)
            })
            .map(Self::Character)
            .unwrap_or(Self::Unknown(node)),
            "comment" => ::treesitter_types::runtime::maybe_grow_stack(|| {
                <Comment as ::treesitter_types::FromNode>::from_node(node, src)
            })
            .map(Self::Comment)
            .unwrap_or(Self::Unknown(node)),
            "escape_sequence" => ::treesitter_types::runtime::maybe_grow_stack(|| {
                <EscapeSequence as ::treesitter_types::FromNode>::from_node(node, src)
            })
            .map(Self::EscapeSequence)
            .unwrap_or(Self::Unknown(node)),
            "false" => ::treesitter_types::runtime::maybe_grow_stack(|| {
                <False as ::treesitter_types::FromNode>::from_node(node, src)
            })
            .map(Self::False)
            .unwrap_or(Self::Unknown(node)),
            "field_identifier" => ::treesitter_types::runtime::maybe_grow_stack(|| {
                <FieldIdentifier as ::treesitter_types::FromNode>::from_node(node, src)
            })
            .map(Self::FieldIdentifier)
            .unwrap_or(Self::Unknown(node)),
            "identifier" => ::treesitter_types::runtime::maybe_grow_stack(|| {
                <Identifier as ::treesitter_types::FromNode>::from_node(node, src)
            })
            .map(Self::Identifier)
            .unwrap_or(Self::Unknown(node)),
            "ms_restrict_modifier" => ::treesitter_types::runtime::maybe_grow_stack(|| {
                <MsRestrictModifier as ::treesitter_types::FromNode>::from_node(node, src)
            })
            .map(Self::MsRestrictModifier)
            .unwrap_or(Self::Unknown(node)),
            "ms_signed_ptr_modifier" => ::treesitter_types::runtime::maybe_grow_stack(|| {
                <MsSignedPtrModifier as ::treesitter_types::FromNode>::from_node(node, src)
            })
            .map(Self::MsSignedPtrModifier)
            .unwrap_or(Self::Unknown(node)),
            "ms_unsigned_ptr_modifier" => ::treesitter_types::runtime::maybe_grow_stack(|| {
                <MsUnsignedPtrModifier as ::treesitter_types::FromNode>::from_node(node, src)
            })
            .map(Self::MsUnsignedPtrModifier)
            .unwrap_or(Self::Unknown(node)),
            "number_literal" => ::treesitter_types::runtime::maybe_grow_stack(|| {
                <NumberLiteral as ::treesitter_types::FromNode>::from_node(node, src)
            })
            .map(Self::NumberLiteral)
            .unwrap_or(Self::Unknown(node)),
            "preproc_arg" => ::treesitter_types::runtime::maybe_grow_stack(|| {
                <PreprocArg as ::treesitter_types::FromNode>::from_node(node, src)
            })
            .map(Self::PreprocArg)
            .unwrap_or(Self::Unknown(node)),
            "preproc_directive" => ::treesitter_types::runtime::maybe_grow_stack(|| {
                <PreprocDirective as ::treesitter_types::FromNode>::from_node(node, src)
            })
            .map(Self::PreprocDirective)
            .unwrap_or(Self::Unknown(node)),
            "primitive_type" => ::treesitter_types::runtime::maybe_grow_stack(|| {
                <PrimitiveType as ::treesitter_types::FromNode>::from_node(node, src)
            })
            .map(Self::PrimitiveType)
            .unwrap_or(Self::Unknown(node)),
            "statement_identifier" => ::treesitter_types::runtime::maybe_grow_stack(|| {
                <StatementIdentifier as ::treesitter_types::FromNode>::from_node(node, src)
            })
            .map(Self::StatementIdentifier)
            .unwrap_or(Self::Unknown(node)),
            "string_content" => ::treesitter_types::runtime::maybe_grow_stack(|| {
                <StringContent as ::treesitter_types::FromNode>::from_node(node, src)
            })
            .map(Self::StringContent)
            .unwrap_or(Self::Unknown(node)),
            "system_lib_string" => ::treesitter_types::runtime::maybe_grow_stack(|| {
                <SystemLibString as ::treesitter_types::FromNode>::from_node(node, src)
            })
            .map(Self::SystemLibString)
            .unwrap_or(Self::Unknown(node)),
            "true" => ::treesitter_types::runtime::maybe_grow_stack(|| {
                <True as ::treesitter_types::FromNode>::from_node(node, src)
            })
            .map(Self::True)
            .unwrap_or(Self::Unknown(node)),
            "type_identifier" => ::treesitter_types::runtime::maybe_grow_stack(|| {
                <TypeIdentifier as ::treesitter_types::FromNode>::from_node(node, src)
            })
            .map(Self::TypeIdentifier)
            .unwrap_or(Self::Unknown(node)),
            _ => Self::Unknown(node),
        }
    }
}
impl ::treesitter_types::Spanned for AnyNode<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        match self {
            Self::AbstractDeclarator(inner) => inner.span(),
            Self::Declarator(inner) => inner.span(),
            Self::FieldDeclarator(inner) => inner.span(),
            Self::TypeDeclarator(inner) => inner.span(),
            Self::Expression(inner) => inner.span(),
            Self::Statement(inner) => inner.span(),
            Self::TypeSpecifier(inner) => inner.span(),
            Self::AbstractArrayDeclarator(inner) => inner.span(),
            Self::AbstractFunctionDeclarator(inner) => inner.span(),
            Self::AbstractParenthesizedDeclarator(inner) => inner.span(),
            Self::AbstractPointerDeclarator(inner) => inner.span(),
            Self::AlignasQualifier(inner) => inner.span(),
            Self::AlignofExpression(inner) => inner.span(),
            Self::ArgumentList(inner) => inner.span(),
            Self::ArrayDeclarator(inner) => inner.span(),
            Self::AssignmentExpression(inner) => inner.span(),
            Self::Attribute(inner) => inner.span(),
            Self::AttributeDeclaration(inner) => inner.span(),
            Self::AttributeSpecifier(inner) => inner.span(),
            Self::AttributedDeclarator(inner) => inner.span(),
            Self::AttributedStatement(inner) => inner.span(),
            Self::BinaryExpression(inner) => inner.span(),
            Self::BitfieldClause(inner) => inner.span(),
            Self::BreakStatement(inner) => inner.span(),
            Self::CallExpression(inner) => inner.span(),
            Self::CaseStatement(inner) => inner.span(),
            Self::CastExpression(inner) => inner.span(),
            Self::CharLiteral(inner) => inner.span(),
            Self::CommaExpression(inner) => inner.span(),
            Self::CompoundLiteralExpression(inner) => inner.span(),
            Self::CompoundStatement(inner) => inner.span(),
            Self::ConcatenatedString(inner) => inner.span(),
            Self::ConditionalExpression(inner) => inner.span(),
            Self::ContinueStatement(inner) => inner.span(),
            Self::Declaration(inner) => inner.span(),
            Self::DeclarationList(inner) => inner.span(),
            Self::DoStatement(inner) => inner.span(),
            Self::ElseClause(inner) => inner.span(),
            Self::EnumSpecifier(inner) => inner.span(),
            Self::Enumerator(inner) => inner.span(),
            Self::EnumeratorList(inner) => inner.span(),
            Self::ExpressionStatement(inner) => inner.span(),
            Self::ExtensionExpression(inner) => inner.span(),
            Self::FieldDeclaration(inner) => inner.span(),
            Self::FieldDeclarationList(inner) => inner.span(),
            Self::FieldDesignator(inner) => inner.span(),
            Self::FieldExpression(inner) => inner.span(),
            Self::ForStatement(inner) => inner.span(),
            Self::FunctionDeclarator(inner) => inner.span(),
            Self::FunctionDefinition(inner) => inner.span(),
            Self::GenericExpression(inner) => inner.span(),
            Self::GnuAsmClobberList(inner) => inner.span(),
            Self::GnuAsmExpression(inner) => inner.span(),
            Self::GnuAsmGotoList(inner) => inner.span(),
            Self::GnuAsmInputOperand(inner) => inner.span(),
            Self::GnuAsmInputOperandList(inner) => inner.span(),
            Self::GnuAsmOutputOperand(inner) => inner.span(),
            Self::GnuAsmOutputOperandList(inner) => inner.span(),
            Self::GnuAsmQualifier(inner) => inner.span(),
            Self::GotoStatement(inner) => inner.span(),
            Self::IfStatement(inner) => inner.span(),
            Self::InitDeclarator(inner) => inner.span(),
            Self::InitializerList(inner) => inner.span(),
            Self::InitializerPair(inner) => inner.span(),
            Self::LabeledStatement(inner) => inner.span(),
            Self::LinkageSpecification(inner) => inner.span(),
            Self::MacroTypeSpecifier(inner) => inner.span(),
            Self::MsBasedModifier(inner) => inner.span(),
            Self::MsCallModifier(inner) => inner.span(),
            Self::MsDeclspecModifier(inner) => inner.span(),
            Self::MsPointerModifier(inner) => inner.span(),
            Self::MsUnalignedPtrModifier(inner) => inner.span(),
            Self::Null(inner) => inner.span(),
            Self::OffsetofExpression(inner) => inner.span(),
            Self::ParameterDeclaration(inner) => inner.span(),
            Self::ParameterList(inner) => inner.span(),
            Self::ParenthesizedDeclarator(inner) => inner.span(),
            Self::ParenthesizedExpression(inner) => inner.span(),
            Self::PointerDeclarator(inner) => inner.span(),
            Self::PointerExpression(inner) => inner.span(),
            Self::PreprocCall(inner) => inner.span(),
            Self::PreprocDef(inner) => inner.span(),
            Self::PreprocDefined(inner) => inner.span(),
            Self::PreprocElif(inner) => inner.span(),
            Self::PreprocElifdef(inner) => inner.span(),
            Self::PreprocElse(inner) => inner.span(),
            Self::PreprocFunctionDef(inner) => inner.span(),
            Self::PreprocIf(inner) => inner.span(),
            Self::PreprocIfdef(inner) => inner.span(),
            Self::PreprocInclude(inner) => inner.span(),
            Self::PreprocParams(inner) => inner.span(),
            Self::ReturnStatement(inner) => inner.span(),
            Self::SehExceptClause(inner) => inner.span(),
            Self::SehFinallyClause(inner) => inner.span(),
            Self::SehLeaveStatement(inner) => inner.span(),
            Self::SehTryStatement(inner) => inner.span(),
            Self::SizedTypeSpecifier(inner) => inner.span(),
            Self::SizeofExpression(inner) => inner.span(),
            Self::StorageClassSpecifier(inner) => inner.span(),
            Self::StringLiteral(inner) => inner.span(),
            Self::StructSpecifier(inner) => inner.span(),
            Self::SubscriptDesignator(inner) => inner.span(),
            Self::SubscriptExpression(inner) => inner.span(),
            Self::SubscriptRangeDesignator(inner) => inner.span(),
            Self::SwitchStatement(inner) => inner.span(),
            Self::TranslationUnit(inner) => inner.span(),
            Self::TypeDefinition(inner) => inner.span(),
            Self::TypeDescriptor(inner) => inner.span(),
            Self::TypeQualifier(inner) => inner.span(),
            Self::UnaryExpression(inner) => inner.span(),
            Self::UnionSpecifier(inner) => inner.span(),
            Self::UpdateExpression(inner) => inner.span(),
            Self::VariadicParameter(inner) => inner.span(),
            Self::WhileStatement(inner) => inner.span(),
            Self::Character(inner) => inner.span(),
            Self::Comment(inner) => inner.span(),
            Self::EscapeSequence(inner) => inner.span(),
            Self::False(inner) => inner.span(),
            Self::FieldIdentifier(inner) => inner.span(),
            Self::Identifier(inner) => inner.span(),
            Self::MsRestrictModifier(inner) => inner.span(),
            Self::MsSignedPtrModifier(inner) => inner.span(),
            Self::MsUnsignedPtrModifier(inner) => inner.span(),
            Self::NumberLiteral(inner) => inner.span(),
            Self::PreprocArg(inner) => inner.span(),
            Self::PreprocDirective(inner) => inner.span(),
            Self::PrimitiveType(inner) => inner.span(),
            Self::StatementIdentifier(inner) => inner.span(),
            Self::StringContent(inner) => inner.span(),
            Self::SystemLibString(inner) => inner.span(),
            Self::True(inner) => inner.span(),
            Self::TypeIdentifier(inner) => inner.span(),
            Self::Unknown(node) => ::treesitter_types::Span::from(*node),
        }
    }
}
