#[derive(Debug, Clone)]
pub enum AbstractDeclarator<'tree> {
    AbstractArrayDeclarator(::std::boxed::Box<AbstractArrayDeclarator<'tree>>),
    AbstractFunctionDeclarator(::std::boxed::Box<AbstractFunctionDeclarator<'tree>>),
    AbstractParenthesizedDeclarator(::std::boxed::Box<AbstractParenthesizedDeclarator<'tree>>),
    AbstractPointerDeclarator(::std::boxed::Box<AbstractPointerDeclarator<'tree>>),
    AbstractReferenceDeclarator(::std::boxed::Box<AbstractReferenceDeclarator<'tree>>),
}
impl<'tree> ::treesitter_types::FromNode<'tree> for AbstractDeclarator<'tree> {
    #[allow(clippy::collapsible_else_if)]
    fn from_node(
        node: ::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        match node.kind() {
            "abstract_array_declarator" => {
                Ok(Self::AbstractArrayDeclarator(::std::boxed::Box::new(
                    <AbstractArrayDeclarator as ::treesitter_types::FromNode>::from_node(
                        node, src,
                    )?,
                )))
            }
            "abstract_function_declarator" => {
                Ok(Self::AbstractFunctionDeclarator(::std::boxed::Box::new(
                    <AbstractFunctionDeclarator as ::treesitter_types::FromNode>::from_node(
                        node, src,
                    )?,
                )))
            }
            "abstract_parenthesized_declarator" => Ok(Self::AbstractParenthesizedDeclarator(
                ::std::boxed::Box::new(
                    <AbstractParenthesizedDeclarator as ::treesitter_types::FromNode>::from_node(
                        node, src,
                    )?,
                ),
            )),
            "abstract_pointer_declarator" => {
                Ok(Self::AbstractPointerDeclarator(::std::boxed::Box::new(
                    <AbstractPointerDeclarator as ::treesitter_types::FromNode>::from_node(
                        node, src,
                    )?,
                )))
            }
            "abstract_reference_declarator" => {
                Ok(Self::AbstractReferenceDeclarator(::std::boxed::Box::new(
                    <AbstractReferenceDeclarator as ::treesitter_types::FromNode>::from_node(
                        node, src,
                    )?,
                )))
            }
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
            Self::AbstractReferenceDeclarator(inner) => inner.span(),
        }
    }
}
#[derive(Debug, Clone)]
pub enum Declarator<'tree> {
    ArrayDeclarator(::std::boxed::Box<ArrayDeclarator<'tree>>),
    AttributedDeclarator(::std::boxed::Box<AttributedDeclarator<'tree>>),
    DestructorName(::std::boxed::Box<DestructorName<'tree>>),
    FunctionDeclarator(::std::boxed::Box<FunctionDeclarator<'tree>>),
    Identifier(::std::boxed::Box<Identifier<'tree>>),
    OperatorName(::std::boxed::Box<OperatorName<'tree>>),
    ParenthesizedDeclarator(::std::boxed::Box<ParenthesizedDeclarator<'tree>>),
    PointerDeclarator(::std::boxed::Box<PointerDeclarator<'tree>>),
    QualifiedIdentifier(::std::boxed::Box<QualifiedIdentifier<'tree>>),
    ReferenceDeclarator(::std::boxed::Box<ReferenceDeclarator<'tree>>),
    StructuredBindingDeclarator(::std::boxed::Box<StructuredBindingDeclarator<'tree>>),
    TemplateFunction(::std::boxed::Box<TemplateFunction<'tree>>),
}
impl<'tree> ::treesitter_types::FromNode<'tree> for Declarator<'tree> {
    #[allow(clippy::collapsible_else_if)]
    fn from_node(
        node: ::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        match node.kind() {
            "array_declarator" => Ok(Self::ArrayDeclarator(::std::boxed::Box::new(
                <ArrayDeclarator as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "attributed_declarator" => Ok(Self::AttributedDeclarator(::std::boxed::Box::new(
                <AttributedDeclarator as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "destructor_name" => Ok(Self::DestructorName(::std::boxed::Box::new(
                <DestructorName as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "function_declarator" => Ok(Self::FunctionDeclarator(::std::boxed::Box::new(
                <FunctionDeclarator as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "identifier" => Ok(Self::Identifier(::std::boxed::Box::new(
                <Identifier as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "operator_name" => Ok(Self::OperatorName(::std::boxed::Box::new(
                <OperatorName as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "parenthesized_declarator" => {
                Ok(Self::ParenthesizedDeclarator(::std::boxed::Box::new(
                    <ParenthesizedDeclarator as ::treesitter_types::FromNode>::from_node(
                        node, src,
                    )?,
                )))
            }
            "pointer_declarator" => Ok(Self::PointerDeclarator(::std::boxed::Box::new(
                <PointerDeclarator as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "qualified_identifier" => Ok(Self::QualifiedIdentifier(::std::boxed::Box::new(
                <QualifiedIdentifier as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "reference_declarator" => Ok(Self::ReferenceDeclarator(::std::boxed::Box::new(
                <ReferenceDeclarator as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "structured_binding_declarator" => {
                Ok(Self::StructuredBindingDeclarator(::std::boxed::Box::new(
                    <StructuredBindingDeclarator as ::treesitter_types::FromNode>::from_node(
                        node, src,
                    )?,
                )))
            }
            "template_function" => Ok(Self::TemplateFunction(::std::boxed::Box::new(
                <TemplateFunction as ::treesitter_types::FromNode>::from_node(node, src)?,
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
            Self::DestructorName(inner) => inner.span(),
            Self::FunctionDeclarator(inner) => inner.span(),
            Self::Identifier(inner) => inner.span(),
            Self::OperatorName(inner) => inner.span(),
            Self::ParenthesizedDeclarator(inner) => inner.span(),
            Self::PointerDeclarator(inner) => inner.span(),
            Self::QualifiedIdentifier(inner) => inner.span(),
            Self::ReferenceDeclarator(inner) => inner.span(),
            Self::StructuredBindingDeclarator(inner) => inner.span(),
            Self::TemplateFunction(inner) => inner.span(),
        }
    }
}
#[derive(Debug, Clone)]
pub enum FieldDeclarator<'tree> {
    ArrayDeclarator(::std::boxed::Box<ArrayDeclarator<'tree>>),
    AttributedDeclarator(::std::boxed::Box<AttributedDeclarator<'tree>>),
    FieldIdentifier(::std::boxed::Box<FieldIdentifier<'tree>>),
    FunctionDeclarator(::std::boxed::Box<FunctionDeclarator<'tree>>),
    OperatorName(::std::boxed::Box<OperatorName<'tree>>),
    ParenthesizedDeclarator(::std::boxed::Box<ParenthesizedDeclarator<'tree>>),
    PointerDeclarator(::std::boxed::Box<PointerDeclarator<'tree>>),
    ReferenceDeclarator(::std::boxed::Box<ReferenceDeclarator<'tree>>),
    TemplateMethod(::std::boxed::Box<TemplateMethod<'tree>>),
}
impl<'tree> ::treesitter_types::FromNode<'tree> for FieldDeclarator<'tree> {
    #[allow(clippy::collapsible_else_if)]
    fn from_node(
        node: ::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        match node.kind() {
            "array_declarator" => Ok(Self::ArrayDeclarator(::std::boxed::Box::new(
                <ArrayDeclarator as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "attributed_declarator" => Ok(Self::AttributedDeclarator(::std::boxed::Box::new(
                <AttributedDeclarator as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "field_identifier" => Ok(Self::FieldIdentifier(::std::boxed::Box::new(
                <FieldIdentifier as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "function_declarator" => Ok(Self::FunctionDeclarator(::std::boxed::Box::new(
                <FunctionDeclarator as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "operator_name" => Ok(Self::OperatorName(::std::boxed::Box::new(
                <OperatorName as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "parenthesized_declarator" => {
                Ok(Self::ParenthesizedDeclarator(::std::boxed::Box::new(
                    <ParenthesizedDeclarator as ::treesitter_types::FromNode>::from_node(
                        node, src,
                    )?,
                )))
            }
            "pointer_declarator" => Ok(Self::PointerDeclarator(::std::boxed::Box::new(
                <PointerDeclarator as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "reference_declarator" => Ok(Self::ReferenceDeclarator(::std::boxed::Box::new(
                <ReferenceDeclarator as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "template_method" => Ok(Self::TemplateMethod(::std::boxed::Box::new(
                <TemplateMethod as ::treesitter_types::FromNode>::from_node(node, src)?,
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
            Self::OperatorName(inner) => inner.span(),
            Self::ParenthesizedDeclarator(inner) => inner.span(),
            Self::PointerDeclarator(inner) => inner.span(),
            Self::ReferenceDeclarator(inner) => inner.span(),
            Self::TemplateMethod(inner) => inner.span(),
        }
    }
}
#[derive(Debug, Clone)]
pub enum TypeDeclarator<'tree> {
    ArrayDeclarator(::std::boxed::Box<ArrayDeclarator<'tree>>),
    AttributedDeclarator(::std::boxed::Box<AttributedDeclarator<'tree>>),
    FunctionDeclarator(::std::boxed::Box<FunctionDeclarator<'tree>>),
    ParenthesizedDeclarator(::std::boxed::Box<ParenthesizedDeclarator<'tree>>),
    PointerDeclarator(::std::boxed::Box<PointerDeclarator<'tree>>),
    PrimitiveType(::std::boxed::Box<PrimitiveType<'tree>>),
    ReferenceDeclarator(::std::boxed::Box<ReferenceDeclarator<'tree>>),
    TypeIdentifier(::std::boxed::Box<TypeIdentifier<'tree>>),
}
impl<'tree> ::treesitter_types::FromNode<'tree> for TypeDeclarator<'tree> {
    #[allow(clippy::collapsible_else_if)]
    fn from_node(
        node: ::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        match node.kind() {
            "array_declarator" => Ok(Self::ArrayDeclarator(::std::boxed::Box::new(
                <ArrayDeclarator as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "attributed_declarator" => Ok(Self::AttributedDeclarator(::std::boxed::Box::new(
                <AttributedDeclarator as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "function_declarator" => Ok(Self::FunctionDeclarator(::std::boxed::Box::new(
                <FunctionDeclarator as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "parenthesized_declarator" => {
                Ok(Self::ParenthesizedDeclarator(::std::boxed::Box::new(
                    <ParenthesizedDeclarator as ::treesitter_types::FromNode>::from_node(
                        node, src,
                    )?,
                )))
            }
            "pointer_declarator" => Ok(Self::PointerDeclarator(::std::boxed::Box::new(
                <PointerDeclarator as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "primitive_type" => Ok(Self::PrimitiveType(::std::boxed::Box::new(
                <PrimitiveType as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "reference_declarator" => Ok(Self::ReferenceDeclarator(::std::boxed::Box::new(
                <ReferenceDeclarator as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "type_identifier" => Ok(Self::TypeIdentifier(::std::boxed::Box::new(
                <TypeIdentifier as ::treesitter_types::FromNode>::from_node(node, src)?,
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
            Self::ReferenceDeclarator(inner) => inner.span(),
            Self::TypeIdentifier(inner) => inner.span(),
        }
    }
}
#[derive(Debug, Clone)]
pub enum Expression<'tree> {
    AlignofExpression(::std::boxed::Box<AlignofExpression<'tree>>),
    AssignmentExpression(::std::boxed::Box<AssignmentExpression<'tree>>),
    BinaryExpression(::std::boxed::Box<BinaryExpression<'tree>>),
    CallExpression(::std::boxed::Box<CallExpression<'tree>>),
    CastExpression(::std::boxed::Box<CastExpression<'tree>>),
    CharLiteral(::std::boxed::Box<CharLiteral<'tree>>),
    CoAwaitExpression(::std::boxed::Box<CoAwaitExpression<'tree>>),
    CompoundLiteralExpression(::std::boxed::Box<CompoundLiteralExpression<'tree>>),
    ConcatenatedString(::std::boxed::Box<ConcatenatedString<'tree>>),
    ConditionalExpression(::std::boxed::Box<ConditionalExpression<'tree>>),
    DeleteExpression(::std::boxed::Box<DeleteExpression<'tree>>),
    ExtensionExpression(::std::boxed::Box<ExtensionExpression<'tree>>),
    False(::std::boxed::Box<False<'tree>>),
    FieldExpression(::std::boxed::Box<FieldExpression<'tree>>),
    FoldExpression(::std::boxed::Box<FoldExpression<'tree>>),
    GenericExpression(::std::boxed::Box<GenericExpression<'tree>>),
    GnuAsmExpression(::std::boxed::Box<GnuAsmExpression<'tree>>),
    Identifier(::std::boxed::Box<Identifier<'tree>>),
    LambdaExpression(::std::boxed::Box<LambdaExpression<'tree>>),
    NewExpression(::std::boxed::Box<NewExpression<'tree>>),
    Null(::std::boxed::Box<Null<'tree>>),
    NumberLiteral(::std::boxed::Box<NumberLiteral<'tree>>),
    OffsetofExpression(::std::boxed::Box<OffsetofExpression<'tree>>),
    ParameterPackExpansion(::std::boxed::Box<ParameterPackExpansion<'tree>>),
    ParenthesizedExpression(::std::boxed::Box<ParenthesizedExpression<'tree>>),
    PointerExpression(::std::boxed::Box<PointerExpression<'tree>>),
    QualifiedIdentifier(::std::boxed::Box<QualifiedIdentifier<'tree>>),
    RawStringLiteral(::std::boxed::Box<RawStringLiteral<'tree>>),
    ReflectExpression(::std::boxed::Box<ReflectExpression<'tree>>),
    RequiresClause(::std::boxed::Box<RequiresClause<'tree>>),
    RequiresExpression(::std::boxed::Box<RequiresExpression<'tree>>),
    SizeofExpression(::std::boxed::Box<SizeofExpression<'tree>>),
    SpliceExpression(::std::boxed::Box<SpliceExpression<'tree>>),
    StringLiteral(::std::boxed::Box<StringLiteral<'tree>>),
    SubscriptExpression(::std::boxed::Box<SubscriptExpression<'tree>>),
    TemplateFunction(::std::boxed::Box<TemplateFunction<'tree>>),
    This(::std::boxed::Box<This<'tree>>),
    True(::std::boxed::Box<True<'tree>>),
    UnaryExpression(::std::boxed::Box<UnaryExpression<'tree>>),
    UpdateExpression(::std::boxed::Box<UpdateExpression<'tree>>),
    UserDefinedLiteral(::std::boxed::Box<UserDefinedLiteral<'tree>>),
}
impl<'tree> ::treesitter_types::FromNode<'tree> for Expression<'tree> {
    #[allow(clippy::collapsible_else_if)]
    fn from_node(
        node: ::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        match node.kind() {
            "alignof_expression" => Ok(Self::AlignofExpression(::std::boxed::Box::new(
                <AlignofExpression as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "assignment_expression" => Ok(Self::AssignmentExpression(::std::boxed::Box::new(
                <AssignmentExpression as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "binary_expression" => Ok(Self::BinaryExpression(::std::boxed::Box::new(
                <BinaryExpression as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "call_expression" => Ok(Self::CallExpression(::std::boxed::Box::new(
                <CallExpression as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "cast_expression" => Ok(Self::CastExpression(::std::boxed::Box::new(
                <CastExpression as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "char_literal" => Ok(Self::CharLiteral(::std::boxed::Box::new(
                <CharLiteral as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "co_await_expression" => Ok(Self::CoAwaitExpression(::std::boxed::Box::new(
                <CoAwaitExpression as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "compound_literal_expression" => {
                Ok(Self::CompoundLiteralExpression(::std::boxed::Box::new(
                    <CompoundLiteralExpression as ::treesitter_types::FromNode>::from_node(
                        node, src,
                    )?,
                )))
            }
            "concatenated_string" => Ok(Self::ConcatenatedString(::std::boxed::Box::new(
                <ConcatenatedString as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "conditional_expression" => Ok(Self::ConditionalExpression(::std::boxed::Box::new(
                <ConditionalExpression as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "delete_expression" => Ok(Self::DeleteExpression(::std::boxed::Box::new(
                <DeleteExpression as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "extension_expression" => Ok(Self::ExtensionExpression(::std::boxed::Box::new(
                <ExtensionExpression as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "false" => Ok(Self::False(::std::boxed::Box::new(
                <False as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "field_expression" => Ok(Self::FieldExpression(::std::boxed::Box::new(
                <FieldExpression as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "fold_expression" => Ok(Self::FoldExpression(::std::boxed::Box::new(
                <FoldExpression as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "generic_expression" => Ok(Self::GenericExpression(::std::boxed::Box::new(
                <GenericExpression as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "gnu_asm_expression" => Ok(Self::GnuAsmExpression(::std::boxed::Box::new(
                <GnuAsmExpression as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "identifier" => Ok(Self::Identifier(::std::boxed::Box::new(
                <Identifier as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "lambda_expression" => Ok(Self::LambdaExpression(::std::boxed::Box::new(
                <LambdaExpression as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "new_expression" => Ok(Self::NewExpression(::std::boxed::Box::new(
                <NewExpression as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "null" => Ok(Self::Null(::std::boxed::Box::new(
                <Null as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "number_literal" => Ok(Self::NumberLiteral(::std::boxed::Box::new(
                <NumberLiteral as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "offsetof_expression" => Ok(Self::OffsetofExpression(::std::boxed::Box::new(
                <OffsetofExpression as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "parameter_pack_expansion" => Ok(Self::ParameterPackExpansion(::std::boxed::Box::new(
                <ParameterPackExpansion as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "parenthesized_expression" => {
                Ok(Self::ParenthesizedExpression(::std::boxed::Box::new(
                    <ParenthesizedExpression as ::treesitter_types::FromNode>::from_node(
                        node, src,
                    )?,
                )))
            }
            "pointer_expression" => Ok(Self::PointerExpression(::std::boxed::Box::new(
                <PointerExpression as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "qualified_identifier" => Ok(Self::QualifiedIdentifier(::std::boxed::Box::new(
                <QualifiedIdentifier as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "raw_string_literal" => Ok(Self::RawStringLiteral(::std::boxed::Box::new(
                <RawStringLiteral as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "reflect_expression" => Ok(Self::ReflectExpression(::std::boxed::Box::new(
                <ReflectExpression as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "requires_clause" => Ok(Self::RequiresClause(::std::boxed::Box::new(
                <RequiresClause as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "requires_expression" => Ok(Self::RequiresExpression(::std::boxed::Box::new(
                <RequiresExpression as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "sizeof_expression" => Ok(Self::SizeofExpression(::std::boxed::Box::new(
                <SizeofExpression as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "splice_expression" => Ok(Self::SpliceExpression(::std::boxed::Box::new(
                <SpliceExpression as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "string_literal" => Ok(Self::StringLiteral(::std::boxed::Box::new(
                <StringLiteral as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "subscript_expression" => Ok(Self::SubscriptExpression(::std::boxed::Box::new(
                <SubscriptExpression as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "template_function" => Ok(Self::TemplateFunction(::std::boxed::Box::new(
                <TemplateFunction as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "this" => Ok(Self::This(::std::boxed::Box::new(
                <This as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "true" => Ok(Self::True(::std::boxed::Box::new(
                <True as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "unary_expression" => Ok(Self::UnaryExpression(::std::boxed::Box::new(
                <UnaryExpression as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "update_expression" => Ok(Self::UpdateExpression(::std::boxed::Box::new(
                <UpdateExpression as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "user_defined_literal" => Ok(Self::UserDefinedLiteral(::std::boxed::Box::new(
                <UserDefinedLiteral as ::treesitter_types::FromNode>::from_node(node, src)?,
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
            Self::CoAwaitExpression(inner) => inner.span(),
            Self::CompoundLiteralExpression(inner) => inner.span(),
            Self::ConcatenatedString(inner) => inner.span(),
            Self::ConditionalExpression(inner) => inner.span(),
            Self::DeleteExpression(inner) => inner.span(),
            Self::ExtensionExpression(inner) => inner.span(),
            Self::False(inner) => inner.span(),
            Self::FieldExpression(inner) => inner.span(),
            Self::FoldExpression(inner) => inner.span(),
            Self::GenericExpression(inner) => inner.span(),
            Self::GnuAsmExpression(inner) => inner.span(),
            Self::Identifier(inner) => inner.span(),
            Self::LambdaExpression(inner) => inner.span(),
            Self::NewExpression(inner) => inner.span(),
            Self::Null(inner) => inner.span(),
            Self::NumberLiteral(inner) => inner.span(),
            Self::OffsetofExpression(inner) => inner.span(),
            Self::ParameterPackExpansion(inner) => inner.span(),
            Self::ParenthesizedExpression(inner) => inner.span(),
            Self::PointerExpression(inner) => inner.span(),
            Self::QualifiedIdentifier(inner) => inner.span(),
            Self::RawStringLiteral(inner) => inner.span(),
            Self::ReflectExpression(inner) => inner.span(),
            Self::RequiresClause(inner) => inner.span(),
            Self::RequiresExpression(inner) => inner.span(),
            Self::SizeofExpression(inner) => inner.span(),
            Self::SpliceExpression(inner) => inner.span(),
            Self::StringLiteral(inner) => inner.span(),
            Self::SubscriptExpression(inner) => inner.span(),
            Self::TemplateFunction(inner) => inner.span(),
            Self::This(inner) => inner.span(),
            Self::True(inner) => inner.span(),
            Self::UnaryExpression(inner) => inner.span(),
            Self::UpdateExpression(inner) => inner.span(),
            Self::UserDefinedLiteral(inner) => inner.span(),
        }
    }
}
#[derive(Debug, Clone)]
pub enum Statement<'tree> {
    AttributedStatement(::std::boxed::Box<AttributedStatement<'tree>>),
    BreakStatement(::std::boxed::Box<BreakStatement<'tree>>),
    CaseStatement(::std::boxed::Box<CaseStatement<'tree>>),
    CoReturnStatement(::std::boxed::Box<CoReturnStatement<'tree>>),
    CoYieldStatement(::std::boxed::Box<CoYieldStatement<'tree>>),
    CompoundStatement(::std::boxed::Box<CompoundStatement<'tree>>),
    ContinueStatement(::std::boxed::Box<ContinueStatement<'tree>>),
    DoStatement(::std::boxed::Box<DoStatement<'tree>>),
    ExpansionStatement(::std::boxed::Box<ExpansionStatement<'tree>>),
    ExpressionStatement(::std::boxed::Box<ExpressionStatement<'tree>>),
    ForRangeLoop(::std::boxed::Box<ForRangeLoop<'tree>>),
    ForStatement(::std::boxed::Box<ForStatement<'tree>>),
    GotoStatement(::std::boxed::Box<GotoStatement<'tree>>),
    IfStatement(::std::boxed::Box<IfStatement<'tree>>),
    LabeledStatement(::std::boxed::Box<LabeledStatement<'tree>>),
    ReturnStatement(::std::boxed::Box<ReturnStatement<'tree>>),
    SehLeaveStatement(::std::boxed::Box<SehLeaveStatement<'tree>>),
    SehTryStatement(::std::boxed::Box<SehTryStatement<'tree>>),
    SwitchStatement(::std::boxed::Box<SwitchStatement<'tree>>),
    ThrowStatement(::std::boxed::Box<ThrowStatement<'tree>>),
    TryStatement(::std::boxed::Box<TryStatement<'tree>>),
    WhileStatement(::std::boxed::Box<WhileStatement<'tree>>),
}
impl<'tree> ::treesitter_types::FromNode<'tree> for Statement<'tree> {
    #[allow(clippy::collapsible_else_if)]
    fn from_node(
        node: ::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        match node.kind() {
            "attributed_statement" => Ok(Self::AttributedStatement(::std::boxed::Box::new(
                <AttributedStatement as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "break_statement" => Ok(Self::BreakStatement(::std::boxed::Box::new(
                <BreakStatement as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "case_statement" => Ok(Self::CaseStatement(::std::boxed::Box::new(
                <CaseStatement as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "co_return_statement" => Ok(Self::CoReturnStatement(::std::boxed::Box::new(
                <CoReturnStatement as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "co_yield_statement" => Ok(Self::CoYieldStatement(::std::boxed::Box::new(
                <CoYieldStatement as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "compound_statement" => Ok(Self::CompoundStatement(::std::boxed::Box::new(
                <CompoundStatement as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "continue_statement" => Ok(Self::ContinueStatement(::std::boxed::Box::new(
                <ContinueStatement as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "do_statement" => Ok(Self::DoStatement(::std::boxed::Box::new(
                <DoStatement as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "expansion_statement" => Ok(Self::ExpansionStatement(::std::boxed::Box::new(
                <ExpansionStatement as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "expression_statement" => Ok(Self::ExpressionStatement(::std::boxed::Box::new(
                <ExpressionStatement as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "for_range_loop" => Ok(Self::ForRangeLoop(::std::boxed::Box::new(
                <ForRangeLoop as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "for_statement" => Ok(Self::ForStatement(::std::boxed::Box::new(
                <ForStatement as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "goto_statement" => Ok(Self::GotoStatement(::std::boxed::Box::new(
                <GotoStatement as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "if_statement" => Ok(Self::IfStatement(::std::boxed::Box::new(
                <IfStatement as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "labeled_statement" => Ok(Self::LabeledStatement(::std::boxed::Box::new(
                <LabeledStatement as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "return_statement" => Ok(Self::ReturnStatement(::std::boxed::Box::new(
                <ReturnStatement as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "seh_leave_statement" => Ok(Self::SehLeaveStatement(::std::boxed::Box::new(
                <SehLeaveStatement as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "seh_try_statement" => Ok(Self::SehTryStatement(::std::boxed::Box::new(
                <SehTryStatement as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "switch_statement" => Ok(Self::SwitchStatement(::std::boxed::Box::new(
                <SwitchStatement as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "throw_statement" => Ok(Self::ThrowStatement(::std::boxed::Box::new(
                <ThrowStatement as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "try_statement" => Ok(Self::TryStatement(::std::boxed::Box::new(
                <TryStatement as ::treesitter_types::FromNode>::from_node(node, src)?,
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
            Self::AttributedStatement(inner) => inner.span(),
            Self::BreakStatement(inner) => inner.span(),
            Self::CaseStatement(inner) => inner.span(),
            Self::CoReturnStatement(inner) => inner.span(),
            Self::CoYieldStatement(inner) => inner.span(),
            Self::CompoundStatement(inner) => inner.span(),
            Self::ContinueStatement(inner) => inner.span(),
            Self::DoStatement(inner) => inner.span(),
            Self::ExpansionStatement(inner) => inner.span(),
            Self::ExpressionStatement(inner) => inner.span(),
            Self::ForRangeLoop(inner) => inner.span(),
            Self::ForStatement(inner) => inner.span(),
            Self::GotoStatement(inner) => inner.span(),
            Self::IfStatement(inner) => inner.span(),
            Self::LabeledStatement(inner) => inner.span(),
            Self::ReturnStatement(inner) => inner.span(),
            Self::SehLeaveStatement(inner) => inner.span(),
            Self::SehTryStatement(inner) => inner.span(),
            Self::SwitchStatement(inner) => inner.span(),
            Self::ThrowStatement(inner) => inner.span(),
            Self::TryStatement(inner) => inner.span(),
            Self::WhileStatement(inner) => inner.span(),
        }
    }
}
#[derive(Debug, Clone)]
pub enum TypeSpecifier<'tree> {
    ClassSpecifier(::std::boxed::Box<ClassSpecifier<'tree>>),
    Decltype(::std::boxed::Box<Decltype<'tree>>),
    DependentType(::std::boxed::Box<DependentType<'tree>>),
    EnumSpecifier(::std::boxed::Box<EnumSpecifier<'tree>>),
    PlaceholderTypeSpecifier(::std::boxed::Box<PlaceholderTypeSpecifier<'tree>>),
    PrimitiveType(::std::boxed::Box<PrimitiveType<'tree>>),
    QualifiedIdentifier(::std::boxed::Box<QualifiedIdentifier<'tree>>),
    SizedTypeSpecifier(::std::boxed::Box<SizedTypeSpecifier<'tree>>),
    SpliceTypeSpecifier(::std::boxed::Box<SpliceTypeSpecifier<'tree>>),
    StructSpecifier(::std::boxed::Box<StructSpecifier<'tree>>),
    TemplateType(::std::boxed::Box<TemplateType<'tree>>),
    TypeIdentifier(::std::boxed::Box<TypeIdentifier<'tree>>),
    UnionSpecifier(::std::boxed::Box<UnionSpecifier<'tree>>),
}
impl<'tree> ::treesitter_types::FromNode<'tree> for TypeSpecifier<'tree> {
    #[allow(clippy::collapsible_else_if)]
    fn from_node(
        node: ::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        match node.kind() {
            "class_specifier" => Ok(Self::ClassSpecifier(::std::boxed::Box::new(
                <ClassSpecifier as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "decltype" => Ok(Self::Decltype(::std::boxed::Box::new(
                <Decltype as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "dependent_type" => Ok(Self::DependentType(::std::boxed::Box::new(
                <DependentType as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "enum_specifier" => Ok(Self::EnumSpecifier(::std::boxed::Box::new(
                <EnumSpecifier as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "placeholder_type_specifier" => {
                Ok(Self::PlaceholderTypeSpecifier(::std::boxed::Box::new(
                    <PlaceholderTypeSpecifier as ::treesitter_types::FromNode>::from_node(
                        node, src,
                    )?,
                )))
            }
            "primitive_type" => Ok(Self::PrimitiveType(::std::boxed::Box::new(
                <PrimitiveType as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "qualified_identifier" => Ok(Self::QualifiedIdentifier(::std::boxed::Box::new(
                <QualifiedIdentifier as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "sized_type_specifier" => Ok(Self::SizedTypeSpecifier(::std::boxed::Box::new(
                <SizedTypeSpecifier as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "splice_type_specifier" => Ok(Self::SpliceTypeSpecifier(::std::boxed::Box::new(
                <SpliceTypeSpecifier as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "struct_specifier" => Ok(Self::StructSpecifier(::std::boxed::Box::new(
                <StructSpecifier as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "template_type" => Ok(Self::TemplateType(::std::boxed::Box::new(
                <TemplateType as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "type_identifier" => Ok(Self::TypeIdentifier(::std::boxed::Box::new(
                <TypeIdentifier as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "union_specifier" => Ok(Self::UnionSpecifier(::std::boxed::Box::new(
                <UnionSpecifier as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            other => Err(::treesitter_types::ParseError::unexpected_kind(other, node)),
        }
    }
}
impl ::treesitter_types::Spanned for TypeSpecifier<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        match self {
            Self::ClassSpecifier(inner) => inner.span(),
            Self::Decltype(inner) => inner.span(),
            Self::DependentType(inner) => inner.span(),
            Self::EnumSpecifier(inner) => inner.span(),
            Self::PlaceholderTypeSpecifier(inner) => inner.span(),
            Self::PrimitiveType(inner) => inner.span(),
            Self::QualifiedIdentifier(inner) => inner.span(),
            Self::SizedTypeSpecifier(inner) => inner.span(),
            Self::SpliceTypeSpecifier(inner) => inner.span(),
            Self::StructSpecifier(inner) => inner.span(),
            Self::TemplateType(inner) => inner.span(),
            Self::TypeIdentifier(inner) => inner.span(),
            Self::UnionSpecifier(inner) => inner.span(),
        }
    }
}
#[derive(Debug, Clone)]
pub struct AbstractArrayDeclarator<'tree> {
    pub span: ::treesitter_types::Span,
    pub declarator: ::core::option::Option<AbstractDeclarator<'tree>>,
    pub size: ::core::option::Option<AbstractArrayDeclaratorSize<'tree>>,
    pub children: ::std::vec::Vec<TypeQualifier<'tree>>,
}
impl<'tree> ::treesitter_types::FromNode<'tree> for AbstractArrayDeclarator<'tree> {
    #[allow(clippy::match_single_binding, clippy::suspicious_else_formatting)]
    fn from_node(
        node: ::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        debug_assert_eq!(node.kind(), "abstract_array_declarator");
        Ok(Self {
            span: ::treesitter_types::Span::from(node),
            declarator: match node.child_by_field_name("declarator") {
                Some(child) => Some(
                    <AbstractDeclarator as ::treesitter_types::FromNode>::from_node(child, src)?,
                ),
                None => None,
            },
            size: match node.child_by_field_name("size") {
                Some(child) => Some(
                    <AbstractArrayDeclaratorSize as ::treesitter_types::FromNode>::from_node(
                        child, src,
                    )?,
                ),
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
                    items.push(<TypeQualifier as ::treesitter_types::FromNode>::from_node(
                        child, src,
                    )?);
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
#[derive(Debug, Clone)]
pub struct AbstractFunctionDeclarator<'tree> {
    pub span: ::treesitter_types::Span,
    pub declarator: ::core::option::Option<AbstractDeclarator<'tree>>,
    pub parameters: ParameterList<'tree>,
    pub children: ::std::vec::Vec<AbstractFunctionDeclaratorChildren<'tree>>,
}
impl<'tree> ::treesitter_types::FromNode<'tree> for AbstractFunctionDeclarator<'tree> {
    #[allow(clippy::match_single_binding, clippy::suspicious_else_formatting)]
    fn from_node(
        node: ::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        debug_assert_eq!(node.kind(), "abstract_function_declarator");
        Ok(Self {
            span: ::treesitter_types::Span::from(node),
            declarator: match node.child_by_field_name("declarator") {
                Some(child) => Some(
                    <AbstractDeclarator as ::treesitter_types::FromNode>::from_node(child, src)?,
                ),
                None => None,
            },
            parameters: {
                let child = node.child_by_field_name("parameters").ok_or_else(|| {
                    ::treesitter_types::ParseError::missing_field("parameters", node)
                })?;
                <ParameterList as ::treesitter_types::FromNode>::from_node(child, src)?
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
                            <AbstractFunctionDeclaratorChildren as ::treesitter_types::FromNode>::from_node(
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
impl ::treesitter_types::Spanned for AbstractFunctionDeclarator<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        self.span
    }
}
#[derive(Debug, Clone)]
pub struct AbstractParenthesizedDeclarator<'tree> {
    pub span: ::treesitter_types::Span,
    pub children: ::std::vec::Vec<AbstractParenthesizedDeclaratorChildren<'tree>>,
}
impl<'tree> ::treesitter_types::FromNode<'tree> for AbstractParenthesizedDeclarator<'tree> {
    #[allow(clippy::match_single_binding, clippy::suspicious_else_formatting)]
    fn from_node(
        node: ::tree_sitter::Node<'tree>,
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
                            <AbstractParenthesizedDeclaratorChildren as ::treesitter_types::FromNode>::from_node(
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
impl ::treesitter_types::Spanned for AbstractParenthesizedDeclarator<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        self.span
    }
}
#[derive(Debug, Clone)]
pub struct AbstractPointerDeclarator<'tree> {
    pub span: ::treesitter_types::Span,
    pub declarator: ::core::option::Option<AbstractDeclarator<'tree>>,
    pub children: ::std::vec::Vec<AbstractPointerDeclaratorChildren<'tree>>,
}
impl<'tree> ::treesitter_types::FromNode<'tree> for AbstractPointerDeclarator<'tree> {
    #[allow(clippy::match_single_binding, clippy::suspicious_else_formatting)]
    fn from_node(
        node: ::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        debug_assert_eq!(node.kind(), "abstract_pointer_declarator");
        Ok(Self {
            span: ::treesitter_types::Span::from(node),
            declarator: match node.child_by_field_name("declarator") {
                Some(child) => Some(
                    <AbstractDeclarator as ::treesitter_types::FromNode>::from_node(child, src)?,
                ),
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
                            <AbstractPointerDeclaratorChildren as ::treesitter_types::FromNode>::from_node(
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
impl ::treesitter_types::Spanned for AbstractPointerDeclarator<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        self.span
    }
}
#[derive(Debug, Clone)]
pub struct AbstractReferenceDeclarator<'tree> {
    pub span: ::treesitter_types::Span,
    pub children: ::core::option::Option<AbstractDeclarator<'tree>>,
}
impl<'tree> ::treesitter_types::FromNode<'tree> for AbstractReferenceDeclarator<'tree> {
    #[allow(clippy::match_single_binding, clippy::suspicious_else_formatting)]
    fn from_node(
        node: ::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        debug_assert_eq!(node.kind(), "abstract_reference_declarator");
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
                        <AbstractDeclarator as ::treesitter_types::FromNode>::from_node(
                            child, src,
                        )?,
                    ),
                    None => None,
                }
            },
        })
    }
}
impl ::treesitter_types::Spanned for AbstractReferenceDeclarator<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        self.span
    }
}
#[derive(Debug, Clone)]
pub struct AccessSpecifier<'tree> {
    pub span: ::treesitter_types::Span,
    text: &'tree str,
}
impl<'tree> ::treesitter_types::FromNode<'tree> for AccessSpecifier<'tree> {
    fn from_node(
        node: ::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        debug_assert_eq!(node.kind(), "access_specifier");
        Ok(Self {
            span: ::treesitter_types::Span::from(node),
            text: node.utf8_text(src)?,
        })
    }
}
impl<'tree> ::treesitter_types::LeafNode<'tree> for AccessSpecifier<'tree> {
    fn text(&self) -> &'tree str {
        self.text
    }
}
impl ::treesitter_types::Spanned for AccessSpecifier<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        self.span
    }
}
#[derive(Debug, Clone)]
pub struct AliasDeclaration<'tree> {
    pub span: ::treesitter_types::Span,
    pub name: TypeIdentifier<'tree>,
    pub r#type: TypeDescriptor<'tree>,
    pub children: ::std::vec::Vec<AttributeDeclaration<'tree>>,
}
impl<'tree> ::treesitter_types::FromNode<'tree> for AliasDeclaration<'tree> {
    #[allow(clippy::match_single_binding, clippy::suspicious_else_formatting)]
    fn from_node(
        node: ::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        debug_assert_eq!(node.kind(), "alias_declaration");
        Ok(Self {
            span: ::treesitter_types::Span::from(node),
            name: {
                let child = node
                    .child_by_field_name("name")
                    .ok_or_else(|| ::treesitter_types::ParseError::missing_field("name", node))?;
                <TypeIdentifier as ::treesitter_types::FromNode>::from_node(child, src)?
            },
            r#type: {
                let child = node
                    .child_by_field_name("type")
                    .ok_or_else(|| ::treesitter_types::ParseError::missing_field("type", node))?;
                <TypeDescriptor as ::treesitter_types::FromNode>::from_node(child, src)?
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
                        <AttributeDeclaration as ::treesitter_types::FromNode>::from_node(
                            child, src,
                        )?,
                    );
                }
                items
            },
        })
    }
}
impl ::treesitter_types::Spanned for AliasDeclaration<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        self.span
    }
}
#[derive(Debug, Clone)]
pub struct AlignasQualifier<'tree> {
    pub span: ::treesitter_types::Span,
    pub children: AlignasQualifierChildren<'tree>,
}
impl<'tree> ::treesitter_types::FromNode<'tree> for AlignasQualifier<'tree> {
    #[allow(clippy::match_single_binding, clippy::suspicious_else_formatting)]
    fn from_node(
        node: ::tree_sitter::Node<'tree>,
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
                <AlignasQualifierChildren as ::treesitter_types::FromNode>::from_node(child, src)?
            },
        })
    }
}
impl ::treesitter_types::Spanned for AlignasQualifier<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        self.span
    }
}
#[derive(Debug, Clone)]
pub struct AlignofExpression<'tree> {
    pub span: ::treesitter_types::Span,
    pub r#type: TypeDescriptor<'tree>,
}
impl<'tree> ::treesitter_types::FromNode<'tree> for AlignofExpression<'tree> {
    #[allow(clippy::match_single_binding, clippy::suspicious_else_formatting)]
    fn from_node(
        node: ::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        debug_assert_eq!(node.kind(), "alignof_expression");
        Ok(Self {
            span: ::treesitter_types::Span::from(node),
            r#type: {
                let child = node
                    .child_by_field_name("type")
                    .ok_or_else(|| ::treesitter_types::ParseError::missing_field("type", node))?;
                <TypeDescriptor as ::treesitter_types::FromNode>::from_node(child, src)?
            },
        })
    }
}
impl ::treesitter_types::Spanned for AlignofExpression<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        self.span
    }
}
#[derive(Debug, Clone)]
pub struct Annotation<'tree> {
    pub span: ::treesitter_types::Span,
    pub children: Expression<'tree>,
}
impl<'tree> ::treesitter_types::FromNode<'tree> for Annotation<'tree> {
    #[allow(clippy::match_single_binding, clippy::suspicious_else_formatting)]
    fn from_node(
        node: ::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        debug_assert_eq!(node.kind(), "annotation");
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
                <Expression as ::treesitter_types::FromNode>::from_node(child, src)?
            },
        })
    }
}
impl ::treesitter_types::Spanned for Annotation<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        self.span
    }
}
#[derive(Debug, Clone)]
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
                    items.push(
                        <ArgumentListChildren as ::treesitter_types::FromNode>::from_node(
                            child, src,
                        )?,
                    );
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
#[derive(Debug, Clone)]
pub struct ArrayDeclarator<'tree> {
    pub span: ::treesitter_types::Span,
    pub declarator: ArrayDeclaratorDeclarator<'tree>,
    pub size: ::core::option::Option<ArrayDeclaratorSize<'tree>>,
    pub children: ::std::vec::Vec<TypeQualifier<'tree>>,
}
impl<'tree> ::treesitter_types::FromNode<'tree> for ArrayDeclarator<'tree> {
    #[allow(clippy::match_single_binding, clippy::suspicious_else_formatting)]
    fn from_node(
        node: ::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        debug_assert_eq!(node.kind(), "array_declarator");
        Ok(Self {
            span: ::treesitter_types::Span::from(node),
            declarator: {
                let child = node.child_by_field_name("declarator").ok_or_else(|| {
                    ::treesitter_types::ParseError::missing_field("declarator", node)
                })?;
                <ArrayDeclaratorDeclarator as ::treesitter_types::FromNode>::from_node(child, src)?
            },
            size: match node.child_by_field_name("size") {
                Some(child) => Some(
                    <ArrayDeclaratorSize as ::treesitter_types::FromNode>::from_node(child, src)?,
                ),
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
                    items.push(<TypeQualifier as ::treesitter_types::FromNode>::from_node(
                        child, src,
                    )?);
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
#[derive(Debug, Clone)]
pub struct AssignmentExpression<'tree> {
    pub span: ::treesitter_types::Span,
    pub left: Expression<'tree>,
    pub operator: AssignmentExpressionOperator,
    pub right: AssignmentExpressionRight<'tree>,
}
impl<'tree> ::treesitter_types::FromNode<'tree> for AssignmentExpression<'tree> {
    #[allow(clippy::match_single_binding, clippy::suspicious_else_formatting)]
    fn from_node(
        node: ::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        debug_assert_eq!(node.kind(), "assignment_expression");
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
                <AssignmentExpressionOperator as ::treesitter_types::FromNode>::from_node(
                    child, src,
                )?
            },
            right: {
                let child = node
                    .child_by_field_name("right")
                    .ok_or_else(|| ::treesitter_types::ParseError::missing_field("right", node))?;
                <AssignmentExpressionRight as ::treesitter_types::FromNode>::from_node(child, src)?
            },
        })
    }
}
impl ::treesitter_types::Spanned for AssignmentExpression<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        self.span
    }
}
#[derive(Debug, Clone)]
pub struct Attribute<'tree> {
    pub span: ::treesitter_types::Span,
    pub name: Identifier<'tree>,
    pub namespace: ::core::option::Option<Identifier<'tree>>,
    pub prefix: ::core::option::Option<Identifier<'tree>>,
    pub children: ::core::option::Option<ArgumentList<'tree>>,
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
            name: {
                let child = node
                    .child_by_field_name("name")
                    .ok_or_else(|| ::treesitter_types::ParseError::missing_field("name", node))?;
                <Identifier as ::treesitter_types::FromNode>::from_node(child, src)?
            },
            namespace: match node.child_by_field_name("namespace") {
                Some(child) => Some(<Identifier as ::treesitter_types::FromNode>::from_node(
                    child, src,
                )?),
                None => None,
            },
            prefix: match node.child_by_field_name("prefix") {
                Some(child) => Some(<Identifier as ::treesitter_types::FromNode>::from_node(
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
                match non_field_children.first() {
                    Some(&child) => Some(
                        <ArgumentList as ::treesitter_types::FromNode>::from_node(child, src)?,
                    ),
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
#[derive(Debug, Clone)]
pub struct AttributeDeclaration<'tree> {
    pub span: ::treesitter_types::Span,
    pub children: ::std::vec::Vec<AttributeDeclarationChildren<'tree>>,
}
impl<'tree> ::treesitter_types::FromNode<'tree> for AttributeDeclaration<'tree> {
    #[allow(clippy::match_single_binding, clippy::suspicious_else_formatting)]
    fn from_node(
        node: ::tree_sitter::Node<'tree>,
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
                    items.push(
                        <AttributeDeclarationChildren as ::treesitter_types::FromNode>::from_node(
                            child, src,
                        )?,
                    );
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
#[derive(Debug, Clone)]
pub struct AttributeSpecifier<'tree> {
    pub span: ::treesitter_types::Span,
    pub children: ArgumentList<'tree>,
}
impl<'tree> ::treesitter_types::FromNode<'tree> for AttributeSpecifier<'tree> {
    #[allow(clippy::match_single_binding, clippy::suspicious_else_formatting)]
    fn from_node(
        node: ::tree_sitter::Node<'tree>,
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
                <ArgumentList as ::treesitter_types::FromNode>::from_node(child, src)?
            },
        })
    }
}
impl ::treesitter_types::Spanned for AttributeSpecifier<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        self.span
    }
}
#[derive(Debug, Clone)]
pub struct AttributedDeclarator<'tree> {
    pub span: ::treesitter_types::Span,
    pub children: ::std::vec::Vec<AttributedDeclaratorChildren<'tree>>,
}
impl<'tree> ::treesitter_types::FromNode<'tree> for AttributedDeclarator<'tree> {
    #[allow(clippy::match_single_binding, clippy::suspicious_else_formatting)]
    fn from_node(
        node: ::tree_sitter::Node<'tree>,
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
                    items.push(
                        <AttributedDeclaratorChildren as ::treesitter_types::FromNode>::from_node(
                            child, src,
                        )?,
                    );
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
#[derive(Debug, Clone)]
pub struct AttributedStatement<'tree> {
    pub span: ::treesitter_types::Span,
    pub children: ::std::vec::Vec<AttributedStatementChildren<'tree>>,
}
impl<'tree> ::treesitter_types::FromNode<'tree> for AttributedStatement<'tree> {
    #[allow(clippy::match_single_binding, clippy::suspicious_else_formatting)]
    fn from_node(
        node: ::tree_sitter::Node<'tree>,
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
                    items.push(
                        <AttributedStatementChildren as ::treesitter_types::FromNode>::from_node(
                            child, src,
                        )?,
                    );
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
#[derive(Debug, Clone)]
pub struct BaseClassClause<'tree> {
    pub span: ::treesitter_types::Span,
    pub children: ::std::vec::Vec<BaseClassClauseChildren<'tree>>,
}
impl<'tree> ::treesitter_types::FromNode<'tree> for BaseClassClause<'tree> {
    #[allow(clippy::match_single_binding, clippy::suspicious_else_formatting)]
    fn from_node(
        node: ::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        debug_assert_eq!(node.kind(), "base_class_clause");
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
                        <BaseClassClauseChildren as ::treesitter_types::FromNode>::from_node(
                            child, src,
                        )?,
                    );
                }
                items
            },
        })
    }
}
impl ::treesitter_types::Spanned for BaseClassClause<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        self.span
    }
}
#[derive(Debug, Clone)]
pub struct BinaryExpression<'tree> {
    pub span: ::treesitter_types::Span,
    pub left: BinaryExpressionLeft<'tree>,
    pub operator: BinaryExpressionOperator,
    pub right: BinaryExpressionRight<'tree>,
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
                <BinaryExpressionLeft as ::treesitter_types::FromNode>::from_node(child, src)?
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
                <BinaryExpressionRight as ::treesitter_types::FromNode>::from_node(child, src)?
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
pub struct BitfieldClause<'tree> {
    pub span: ::treesitter_types::Span,
    pub children: Expression<'tree>,
}
impl<'tree> ::treesitter_types::FromNode<'tree> for BitfieldClause<'tree> {
    #[allow(clippy::match_single_binding, clippy::suspicious_else_formatting)]
    fn from_node(
        node: ::tree_sitter::Node<'tree>,
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
                <Expression as ::treesitter_types::FromNode>::from_node(child, src)?
            },
        })
    }
}
impl ::treesitter_types::Spanned for BitfieldClause<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        self.span
    }
}
#[derive(Debug, Clone)]
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
#[derive(Debug, Clone)]
pub struct CallExpression<'tree> {
    pub span: ::treesitter_types::Span,
    pub arguments: ArgumentList<'tree>,
    pub function: ::std::vec::Vec<CallExpressionFunction<'tree>>,
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
            arguments: {
                let child = node.child_by_field_name("arguments").ok_or_else(|| {
                    ::treesitter_types::ParseError::missing_field("arguments", node)
                })?;
                <ArgumentList as ::treesitter_types::FromNode>::from_node(child, src)?
            },
            function: {
                let mut cursor = node.walk();
                let mut items = ::std::vec::Vec::new();
                for child in node.children_by_field_name("function", &mut cursor) {
                    items.push(
                        <CallExpressionFunction as ::treesitter_types::FromNode>::from_node(
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
pub struct CaseStatement<'tree> {
    pub span: ::treesitter_types::Span,
    pub value: ::core::option::Option<Expression<'tree>>,
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
            value: match node.child_by_field_name("value") {
                Some(child) => Some(<Expression as ::treesitter_types::FromNode>::from_node(
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
pub struct CastExpression<'tree> {
    pub span: ::treesitter_types::Span,
    pub r#type: TypeDescriptor<'tree>,
    pub value: Expression<'tree>,
}
impl<'tree> ::treesitter_types::FromNode<'tree> for CastExpression<'tree> {
    #[allow(clippy::match_single_binding, clippy::suspicious_else_formatting)]
    fn from_node(
        node: ::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        debug_assert_eq!(node.kind(), "cast_expression");
        Ok(Self {
            span: ::treesitter_types::Span::from(node),
            r#type: {
                let child = node
                    .child_by_field_name("type")
                    .ok_or_else(|| ::treesitter_types::ParseError::missing_field("type", node))?;
                <TypeDescriptor as ::treesitter_types::FromNode>::from_node(child, src)?
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
impl ::treesitter_types::Spanned for CastExpression<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        self.span
    }
}
#[derive(Debug, Clone)]
pub struct CatchClause<'tree> {
    pub span: ::treesitter_types::Span,
    pub body: CompoundStatement<'tree>,
    pub parameters: ParameterList<'tree>,
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
            body: {
                let child = node
                    .child_by_field_name("body")
                    .ok_or_else(|| ::treesitter_types::ParseError::missing_field("body", node))?;
                <CompoundStatement as ::treesitter_types::FromNode>::from_node(child, src)?
            },
            parameters: {
                let child = node.child_by_field_name("parameters").ok_or_else(|| {
                    ::treesitter_types::ParseError::missing_field("parameters", node)
                })?;
                <ParameterList as ::treesitter_types::FromNode>::from_node(child, src)?
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
pub struct CharLiteral<'tree> {
    pub span: ::treesitter_types::Span,
    pub children: ::std::vec::Vec<CharLiteralChildren<'tree>>,
}
impl<'tree> ::treesitter_types::FromNode<'tree> for CharLiteral<'tree> {
    #[allow(clippy::match_single_binding, clippy::suspicious_else_formatting)]
    fn from_node(
        node: ::tree_sitter::Node<'tree>,
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
                    items.push(
                        <CharLiteralChildren as ::treesitter_types::FromNode>::from_node(
                            child, src,
                        )?,
                    );
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
#[derive(Debug, Clone)]
pub struct ClassSpecifier<'tree> {
    pub span: ::treesitter_types::Span,
    pub body: ::core::option::Option<FieldDeclarationList<'tree>>,
    pub name: ::core::option::Option<ClassSpecifierName<'tree>>,
    pub children: ::std::vec::Vec<ClassSpecifierChildren<'tree>>,
}
impl<'tree> ::treesitter_types::FromNode<'tree> for ClassSpecifier<'tree> {
    #[allow(clippy::match_single_binding, clippy::suspicious_else_formatting)]
    fn from_node(
        node: ::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        debug_assert_eq!(node.kind(), "class_specifier");
        Ok(Self {
            span: ::treesitter_types::Span::from(node),
            body: match node.child_by_field_name("body") {
                Some(child) => Some(
                    <FieldDeclarationList as ::treesitter_types::FromNode>::from_node(child, src)?,
                ),
                None => None,
            },
            name: match node.child_by_field_name("name") {
                Some(child) => Some(
                    <ClassSpecifierName as ::treesitter_types::FromNode>::from_node(child, src)?,
                ),
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
                        <ClassSpecifierChildren as ::treesitter_types::FromNode>::from_node(
                            child, src,
                        )?,
                    );
                }
                items
            },
        })
    }
}
impl ::treesitter_types::Spanned for ClassSpecifier<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        self.span
    }
}
#[derive(Debug, Clone)]
pub struct CoAwaitExpression<'tree> {
    pub span: ::treesitter_types::Span,
    pub argument: Expression<'tree>,
    pub operator: CoAwaitExpressionOperator,
}
impl<'tree> ::treesitter_types::FromNode<'tree> for CoAwaitExpression<'tree> {
    #[allow(clippy::match_single_binding, clippy::suspicious_else_formatting)]
    fn from_node(
        node: ::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        debug_assert_eq!(node.kind(), "co_await_expression");
        Ok(Self {
            span: ::treesitter_types::Span::from(node),
            argument: {
                let child = node.child_by_field_name("argument").ok_or_else(|| {
                    ::treesitter_types::ParseError::missing_field("argument", node)
                })?;
                <Expression as ::treesitter_types::FromNode>::from_node(child, src)?
            },
            operator: {
                let child = node.child_by_field_name("operator").ok_or_else(|| {
                    ::treesitter_types::ParseError::missing_field("operator", node)
                })?;
                <CoAwaitExpressionOperator as ::treesitter_types::FromNode>::from_node(child, src)?
            },
        })
    }
}
impl ::treesitter_types::Spanned for CoAwaitExpression<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        self.span
    }
}
#[derive(Debug, Clone)]
pub struct CoReturnStatement<'tree> {
    pub span: ::treesitter_types::Span,
    pub children: ::core::option::Option<Expression<'tree>>,
}
impl<'tree> ::treesitter_types::FromNode<'tree> for CoReturnStatement<'tree> {
    #[allow(clippy::match_single_binding, clippy::suspicious_else_formatting)]
    fn from_node(
        node: ::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        debug_assert_eq!(node.kind(), "co_return_statement");
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
                    Some(&child) => Some(<Expression as ::treesitter_types::FromNode>::from_node(
                        child, src,
                    )?),
                    None => None,
                }
            },
        })
    }
}
impl ::treesitter_types::Spanned for CoReturnStatement<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        self.span
    }
}
#[derive(Debug, Clone)]
pub struct CoYieldStatement<'tree> {
    pub span: ::treesitter_types::Span,
    pub children: Expression<'tree>,
}
impl<'tree> ::treesitter_types::FromNode<'tree> for CoYieldStatement<'tree> {
    #[allow(clippy::match_single_binding, clippy::suspicious_else_formatting)]
    fn from_node(
        node: ::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        debug_assert_eq!(node.kind(), "co_yield_statement");
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
                <Expression as ::treesitter_types::FromNode>::from_node(child, src)?
            },
        })
    }
}
impl ::treesitter_types::Spanned for CoYieldStatement<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        self.span
    }
}
#[derive(Debug, Clone)]
pub struct CommaExpression<'tree> {
    pub span: ::treesitter_types::Span,
    pub left: Expression<'tree>,
    pub right: CommaExpressionRight<'tree>,
}
impl<'tree> ::treesitter_types::FromNode<'tree> for CommaExpression<'tree> {
    #[allow(clippy::match_single_binding, clippy::suspicious_else_formatting)]
    fn from_node(
        node: ::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        debug_assert_eq!(node.kind(), "comma_expression");
        Ok(Self {
            span: ::treesitter_types::Span::from(node),
            left: {
                let child = node
                    .child_by_field_name("left")
                    .ok_or_else(|| ::treesitter_types::ParseError::missing_field("left", node))?;
                <Expression as ::treesitter_types::FromNode>::from_node(child, src)?
            },
            right: {
                let child = node
                    .child_by_field_name("right")
                    .ok_or_else(|| ::treesitter_types::ParseError::missing_field("right", node))?;
                <CommaExpressionRight as ::treesitter_types::FromNode>::from_node(child, src)?
            },
        })
    }
}
impl ::treesitter_types::Spanned for CommaExpression<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        self.span
    }
}
#[derive(Debug, Clone)]
pub struct CompoundLiteralExpression<'tree> {
    pub span: ::treesitter_types::Span,
    pub r#type: ::std::vec::Vec<CompoundLiteralExpressionType<'tree>>,
    pub value: InitializerList<'tree>,
}
impl<'tree> ::treesitter_types::FromNode<'tree> for CompoundLiteralExpression<'tree> {
    #[allow(clippy::match_single_binding, clippy::suspicious_else_formatting)]
    fn from_node(
        node: ::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        debug_assert_eq!(node.kind(), "compound_literal_expression");
        Ok(Self {
            span: ::treesitter_types::Span::from(node),
            r#type: {
                let mut cursor = node.walk();
                let mut items = ::std::vec::Vec::new();
                for child in node.children_by_field_name("type", &mut cursor) {
                    items.push(
                        <CompoundLiteralExpressionType as ::treesitter_types::FromNode>::from_node(
                            child, src,
                        )?,
                    );
                }
                items
            },
            value: {
                let child = node
                    .child_by_field_name("value")
                    .ok_or_else(|| ::treesitter_types::ParseError::missing_field("value", node))?;
                <InitializerList as ::treesitter_types::FromNode>::from_node(child, src)?
            },
        })
    }
}
impl ::treesitter_types::Spanned for CompoundLiteralExpression<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        self.span
    }
}
#[derive(Debug, Clone)]
pub struct CompoundRequirement<'tree> {
    pub span: ::treesitter_types::Span,
    pub children: ::std::vec::Vec<CompoundRequirementChildren<'tree>>,
}
impl<'tree> ::treesitter_types::FromNode<'tree> for CompoundRequirement<'tree> {
    #[allow(clippy::match_single_binding, clippy::suspicious_else_formatting)]
    fn from_node(
        node: ::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        debug_assert_eq!(node.kind(), "compound_requirement");
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
                        <CompoundRequirementChildren as ::treesitter_types::FromNode>::from_node(
                            child, src,
                        )?,
                    );
                }
                items
            },
        })
    }
}
impl ::treesitter_types::Spanned for CompoundRequirement<'_> {
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
pub struct ConcatenatedString<'tree> {
    pub span: ::treesitter_types::Span,
    pub children: ::std::vec::Vec<ConcatenatedStringChildren<'tree>>,
}
impl<'tree> ::treesitter_types::FromNode<'tree> for ConcatenatedString<'tree> {
    #[allow(clippy::match_single_binding, clippy::suspicious_else_formatting)]
    fn from_node(
        node: ::tree_sitter::Node<'tree>,
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
                    items.push(
                        <ConcatenatedStringChildren as ::treesitter_types::FromNode>::from_node(
                            child, src,
                        )?,
                    );
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
#[derive(Debug, Clone)]
pub struct ConceptDefinition<'tree> {
    pub span: ::treesitter_types::Span,
    pub name: Identifier<'tree>,
    pub children: Expression<'tree>,
}
impl<'tree> ::treesitter_types::FromNode<'tree> for ConceptDefinition<'tree> {
    #[allow(clippy::match_single_binding, clippy::suspicious_else_formatting)]
    fn from_node(
        node: ::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        debug_assert_eq!(node.kind(), "concept_definition");
        Ok(Self {
            span: ::treesitter_types::Span::from(node),
            name: {
                let child = node
                    .child_by_field_name("name")
                    .ok_or_else(|| ::treesitter_types::ParseError::missing_field("name", node))?;
                <Identifier as ::treesitter_types::FromNode>::from_node(child, src)?
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
                <Expression as ::treesitter_types::FromNode>::from_node(child, src)?
            },
        })
    }
}
impl ::treesitter_types::Spanned for ConceptDefinition<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        self.span
    }
}
#[derive(Debug, Clone)]
pub struct ConditionClause<'tree> {
    pub span: ::treesitter_types::Span,
    pub initializer: ::core::option::Option<InitStatement<'tree>>,
    pub value: ConditionClauseValue<'tree>,
}
impl<'tree> ::treesitter_types::FromNode<'tree> for ConditionClause<'tree> {
    #[allow(clippy::match_single_binding, clippy::suspicious_else_formatting)]
    fn from_node(
        node: ::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        debug_assert_eq!(node.kind(), "condition_clause");
        Ok(Self {
            span: ::treesitter_types::Span::from(node),
            initializer: match node.child_by_field_name("initializer") {
                Some(child) => Some(<InitStatement as ::treesitter_types::FromNode>::from_node(
                    child, src,
                )?),
                None => None,
            },
            value: {
                let child = node
                    .child_by_field_name("value")
                    .ok_or_else(|| ::treesitter_types::ParseError::missing_field("value", node))?;
                <ConditionClauseValue as ::treesitter_types::FromNode>::from_node(child, src)?
            },
        })
    }
}
impl ::treesitter_types::Spanned for ConditionClause<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        self.span
    }
}
#[derive(Debug, Clone)]
pub struct ConditionalExpression<'tree> {
    pub span: ::treesitter_types::Span,
    pub alternative: Expression<'tree>,
    pub condition: Expression<'tree>,
    pub consequence: ::core::option::Option<ConditionalExpressionConsequence<'tree>>,
}
impl<'tree> ::treesitter_types::FromNode<'tree> for ConditionalExpression<'tree> {
    #[allow(clippy::match_single_binding, clippy::suspicious_else_formatting)]
    fn from_node(
        node: ::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        debug_assert_eq!(node.kind(), "conditional_expression");
        Ok(Self {
            span: ::treesitter_types::Span::from(node),
            alternative: {
                let child = node.child_by_field_name("alternative").ok_or_else(|| {
                    ::treesitter_types::ParseError::missing_field("alternative", node)
                })?;
                <Expression as ::treesitter_types::FromNode>::from_node(child, src)?
            },
            condition: {
                let child = node.child_by_field_name("condition").ok_or_else(|| {
                    ::treesitter_types::ParseError::missing_field("condition", node)
                })?;
                <Expression as ::treesitter_types::FromNode>::from_node(child, src)?
            },
            consequence: match node.child_by_field_name("consequence") {
                Some(child) => Some(
                    <ConditionalExpressionConsequence as ::treesitter_types::FromNode>::from_node(
                        child, src,
                    )?,
                ),
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
#[derive(Debug, Clone)]
pub struct ConstevalBlockDeclaration<'tree> {
    pub span: ::treesitter_types::Span,
    pub body: CompoundStatement<'tree>,
}
impl<'tree> ::treesitter_types::FromNode<'tree> for ConstevalBlockDeclaration<'tree> {
    #[allow(clippy::match_single_binding, clippy::suspicious_else_formatting)]
    fn from_node(
        node: ::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        debug_assert_eq!(node.kind(), "consteval_block_declaration");
        Ok(Self {
            span: ::treesitter_types::Span::from(node),
            body: {
                let child = node
                    .child_by_field_name("body")
                    .ok_or_else(|| ::treesitter_types::ParseError::missing_field("body", node))?;
                <CompoundStatement as ::treesitter_types::FromNode>::from_node(child, src)?
            },
        })
    }
}
impl ::treesitter_types::Spanned for ConstevalBlockDeclaration<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        self.span
    }
}
#[derive(Debug, Clone)]
pub struct ConstraintConjunction<'tree> {
    pub span: ::treesitter_types::Span,
    pub left: ::std::vec::Vec<ConstraintConjunctionLeft<'tree>>,
    pub operator: ConstraintConjunctionOperator,
    pub right: ::std::vec::Vec<ConstraintConjunctionRight<'tree>>,
}
impl<'tree> ::treesitter_types::FromNode<'tree> for ConstraintConjunction<'tree> {
    #[allow(clippy::match_single_binding, clippy::suspicious_else_formatting)]
    fn from_node(
        node: ::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        debug_assert_eq!(node.kind(), "constraint_conjunction");
        Ok(Self {
            span: ::treesitter_types::Span::from(node),
            left: {
                let mut cursor = node.walk();
                let mut items = ::std::vec::Vec::new();
                for child in node.children_by_field_name("left", &mut cursor) {
                    items.push(
                        <ConstraintConjunctionLeft as ::treesitter_types::FromNode>::from_node(
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
                <ConstraintConjunctionOperator as ::treesitter_types::FromNode>::from_node(
                    child, src,
                )?
            },
            right: {
                let mut cursor = node.walk();
                let mut items = ::std::vec::Vec::new();
                for child in node.children_by_field_name("right", &mut cursor) {
                    items.push(
                        <ConstraintConjunctionRight as ::treesitter_types::FromNode>::from_node(
                            child, src,
                        )?,
                    );
                }
                items
            },
        })
    }
}
impl ::treesitter_types::Spanned for ConstraintConjunction<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        self.span
    }
}
#[derive(Debug, Clone)]
pub struct ConstraintDisjunction<'tree> {
    pub span: ::treesitter_types::Span,
    pub left: ::std::vec::Vec<ConstraintDisjunctionLeft<'tree>>,
    pub operator: ConstraintDisjunctionOperator,
    pub right: ::std::vec::Vec<ConstraintDisjunctionRight<'tree>>,
}
impl<'tree> ::treesitter_types::FromNode<'tree> for ConstraintDisjunction<'tree> {
    #[allow(clippy::match_single_binding, clippy::suspicious_else_formatting)]
    fn from_node(
        node: ::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        debug_assert_eq!(node.kind(), "constraint_disjunction");
        Ok(Self {
            span: ::treesitter_types::Span::from(node),
            left: {
                let mut cursor = node.walk();
                let mut items = ::std::vec::Vec::new();
                for child in node.children_by_field_name("left", &mut cursor) {
                    items.push(
                        <ConstraintDisjunctionLeft as ::treesitter_types::FromNode>::from_node(
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
                <ConstraintDisjunctionOperator as ::treesitter_types::FromNode>::from_node(
                    child, src,
                )?
            },
            right: {
                let mut cursor = node.walk();
                let mut items = ::std::vec::Vec::new();
                for child in node.children_by_field_name("right", &mut cursor) {
                    items.push(
                        <ConstraintDisjunctionRight as ::treesitter_types::FromNode>::from_node(
                            child, src,
                        )?,
                    );
                }
                items
            },
        })
    }
}
impl ::treesitter_types::Spanned for ConstraintDisjunction<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        self.span
    }
}
#[derive(Debug, Clone)]
pub struct ContinueStatement<'tree> {
    pub span: ::treesitter_types::Span,
    text: &'tree str,
}
impl<'tree> ::treesitter_types::FromNode<'tree> for ContinueStatement<'tree> {
    fn from_node(
        node: ::tree_sitter::Node<'tree>,
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
#[derive(Debug, Clone)]
pub struct Declaration<'tree> {
    pub span: ::treesitter_types::Span,
    pub declarator: ::std::vec::Vec<DeclarationDeclarator<'tree>>,
    pub default_value: ::core::option::Option<Expression<'tree>>,
    pub r#type: ::core::option::Option<TypeSpecifier<'tree>>,
    pub value: ::core::option::Option<DeclarationValue<'tree>>,
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
            declarator: {
                let mut cursor = node.walk();
                let mut items = ::std::vec::Vec::new();
                for child in node.children_by_field_name("declarator", &mut cursor) {
                    items.push(
                        <DeclarationDeclarator as ::treesitter_types::FromNode>::from_node(
                            child, src,
                        )?,
                    );
                }
                items
            },
            default_value: match node.child_by_field_name("default_value") {
                Some(child) => Some(<Expression as ::treesitter_types::FromNode>::from_node(
                    child, src,
                )?),
                None => None,
            },
            r#type: match node.child_by_field_name("type") {
                Some(child) => Some(<TypeSpecifier as ::treesitter_types::FromNode>::from_node(
                    child, src,
                )?),
                None => None,
            },
            value: match node.child_by_field_name("value") {
                Some(child) => {
                    Some(<DeclarationValue as ::treesitter_types::FromNode>::from_node(child, src)?)
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
                            if cursor.field_name().is_none()
                                && cursor.node().is_named()
                                && !cursor.node().is_extra()
                            {
                                result.push(cursor.node());
                            }
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
pub struct DeclarationList<'tree> {
    pub span: ::treesitter_types::Span,
    pub children: ::std::vec::Vec<DeclarationListChildren<'tree>>,
}
impl<'tree> ::treesitter_types::FromNode<'tree> for DeclarationList<'tree> {
    #[allow(clippy::match_single_binding, clippy::suspicious_else_formatting)]
    fn from_node(
        node: ::tree_sitter::Node<'tree>,
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
                    items.push(
                        <DeclarationListChildren as ::treesitter_types::FromNode>::from_node(
                            child, src,
                        )?,
                    );
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
#[derive(Debug, Clone)]
pub struct Decltype<'tree> {
    pub span: ::treesitter_types::Span,
    pub children: DecltypeChildren<'tree>,
}
impl<'tree> ::treesitter_types::FromNode<'tree> for Decltype<'tree> {
    #[allow(clippy::match_single_binding, clippy::suspicious_else_formatting)]
    fn from_node(
        node: ::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        debug_assert_eq!(node.kind(), "decltype");
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
                <DecltypeChildren as ::treesitter_types::FromNode>::from_node(child, src)?
            },
        })
    }
}
impl ::treesitter_types::Spanned for Decltype<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        self.span
    }
}
#[derive(Debug, Clone)]
pub struct DefaultMethodClause<'tree> {
    pub span: ::treesitter_types::Span,
    text: &'tree str,
}
impl<'tree> ::treesitter_types::FromNode<'tree> for DefaultMethodClause<'tree> {
    fn from_node(
        node: ::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        debug_assert_eq!(node.kind(), "default_method_clause");
        Ok(Self {
            span: ::treesitter_types::Span::from(node),
            text: node.utf8_text(src)?,
        })
    }
}
impl<'tree> ::treesitter_types::LeafNode<'tree> for DefaultMethodClause<'tree> {
    fn text(&self) -> &'tree str {
        self.text
    }
}
impl ::treesitter_types::Spanned for DefaultMethodClause<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        self.span
    }
}
#[derive(Debug, Clone)]
pub struct DeleteExpression<'tree> {
    pub span: ::treesitter_types::Span,
    pub children: Expression<'tree>,
}
impl<'tree> ::treesitter_types::FromNode<'tree> for DeleteExpression<'tree> {
    #[allow(clippy::match_single_binding, clippy::suspicious_else_formatting)]
    fn from_node(
        node: ::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        debug_assert_eq!(node.kind(), "delete_expression");
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
                <Expression as ::treesitter_types::FromNode>::from_node(child, src)?
            },
        })
    }
}
impl ::treesitter_types::Spanned for DeleteExpression<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        self.span
    }
}
#[derive(Debug, Clone)]
pub struct DeleteMethodClause<'tree> {
    pub span: ::treesitter_types::Span,
    text: &'tree str,
}
impl<'tree> ::treesitter_types::FromNode<'tree> for DeleteMethodClause<'tree> {
    fn from_node(
        node: ::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        debug_assert_eq!(node.kind(), "delete_method_clause");
        Ok(Self {
            span: ::treesitter_types::Span::from(node),
            text: node.utf8_text(src)?,
        })
    }
}
impl<'tree> ::treesitter_types::LeafNode<'tree> for DeleteMethodClause<'tree> {
    fn text(&self) -> &'tree str {
        self.text
    }
}
impl ::treesitter_types::Spanned for DeleteMethodClause<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        self.span
    }
}
#[derive(Debug, Clone)]
pub struct DependentName<'tree> {
    pub span: ::treesitter_types::Span,
    pub children: DependentNameChildren<'tree>,
}
impl<'tree> ::treesitter_types::FromNode<'tree> for DependentName<'tree> {
    #[allow(clippy::match_single_binding, clippy::suspicious_else_formatting)]
    fn from_node(
        node: ::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        debug_assert_eq!(node.kind(), "dependent_name");
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
                <DependentNameChildren as ::treesitter_types::FromNode>::from_node(child, src)?
            },
        })
    }
}
impl ::treesitter_types::Spanned for DependentName<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        self.span
    }
}
#[derive(Debug, Clone)]
pub struct DependentType<'tree> {
    pub span: ::treesitter_types::Span,
    pub children: TypeSpecifier<'tree>,
}
impl<'tree> ::treesitter_types::FromNode<'tree> for DependentType<'tree> {
    #[allow(clippy::match_single_binding, clippy::suspicious_else_formatting)]
    fn from_node(
        node: ::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        debug_assert_eq!(node.kind(), "dependent_type");
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
                <TypeSpecifier as ::treesitter_types::FromNode>::from_node(child, src)?
            },
        })
    }
}
impl ::treesitter_types::Spanned for DependentType<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        self.span
    }
}
#[derive(Debug, Clone)]
pub struct DestructorName<'tree> {
    pub span: ::treesitter_types::Span,
    pub children: Identifier<'tree>,
}
impl<'tree> ::treesitter_types::FromNode<'tree> for DestructorName<'tree> {
    #[allow(clippy::match_single_binding, clippy::suspicious_else_formatting)]
    fn from_node(
        node: ::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        debug_assert_eq!(node.kind(), "destructor_name");
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
                <Identifier as ::treesitter_types::FromNode>::from_node(child, src)?
            },
        })
    }
}
impl ::treesitter_types::Spanned for DestructorName<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        self.span
    }
}
#[derive(Debug, Clone)]
pub struct DoStatement<'tree> {
    pub span: ::treesitter_types::Span,
    pub body: Statement<'tree>,
    pub condition: ParenthesizedExpression<'tree>,
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
            body: {
                let child = node
                    .child_by_field_name("body")
                    .ok_or_else(|| ::treesitter_types::ParseError::missing_field("body", node))?;
                <Statement as ::treesitter_types::FromNode>::from_node(child, src)?
            },
            condition: {
                let child = node.child_by_field_name("condition").ok_or_else(|| {
                    ::treesitter_types::ParseError::missing_field("condition", node)
                })?;
                <ParenthesizedExpression as ::treesitter_types::FromNode>::from_node(child, src)?
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
pub struct ElseClause<'tree> {
    pub span: ::treesitter_types::Span,
    pub children: Statement<'tree>,
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
                <Statement as ::treesitter_types::FromNode>::from_node(child, src)?
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
pub struct EnumSpecifier<'tree> {
    pub span: ::treesitter_types::Span,
    pub base: ::core::option::Option<EnumSpecifierBase<'tree>>,
    pub body: ::core::option::Option<EnumeratorList<'tree>>,
    pub name: ::core::option::Option<EnumSpecifierName<'tree>>,
    pub children: ::core::option::Option<AttributeSpecifier<'tree>>,
}
impl<'tree> ::treesitter_types::FromNode<'tree> for EnumSpecifier<'tree> {
    #[allow(clippy::match_single_binding, clippy::suspicious_else_formatting)]
    fn from_node(
        node: ::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        debug_assert_eq!(node.kind(), "enum_specifier");
        Ok(Self {
            span: ::treesitter_types::Span::from(node),
            base: match node.child_by_field_name("base") {
                Some(child) => Some(
                    <EnumSpecifierBase as ::treesitter_types::FromNode>::from_node(child, src)?,
                ),
                None => None,
            },
            body: match node.child_by_field_name("body") {
                Some(child) => Some(<EnumeratorList as ::treesitter_types::FromNode>::from_node(
                    child, src,
                )?),
                None => None,
            },
            name: match node.child_by_field_name("name") {
                Some(child) => Some(
                    <EnumSpecifierName as ::treesitter_types::FromNode>::from_node(child, src)?,
                ),
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
                    Some(&child) => Some(
                        <AttributeSpecifier as ::treesitter_types::FromNode>::from_node(
                            child, src,
                        )?,
                    ),
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
#[derive(Debug, Clone)]
pub struct Enumerator<'tree> {
    pub span: ::treesitter_types::Span,
    pub name: Identifier<'tree>,
    pub value: ::core::option::Option<Expression<'tree>>,
}
impl<'tree> ::treesitter_types::FromNode<'tree> for Enumerator<'tree> {
    #[allow(clippy::match_single_binding, clippy::suspicious_else_formatting)]
    fn from_node(
        node: ::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        debug_assert_eq!(node.kind(), "enumerator");
        Ok(Self {
            span: ::treesitter_types::Span::from(node),
            name: {
                let child = node
                    .child_by_field_name("name")
                    .ok_or_else(|| ::treesitter_types::ParseError::missing_field("name", node))?;
                <Identifier as ::treesitter_types::FromNode>::from_node(child, src)?
            },
            value: match node.child_by_field_name("value") {
                Some(child) => Some(<Expression as ::treesitter_types::FromNode>::from_node(
                    child, src,
                )?),
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
#[derive(Debug, Clone)]
pub struct EnumeratorList<'tree> {
    pub span: ::treesitter_types::Span,
    pub children: ::std::vec::Vec<EnumeratorListChildren<'tree>>,
}
impl<'tree> ::treesitter_types::FromNode<'tree> for EnumeratorList<'tree> {
    #[allow(clippy::match_single_binding, clippy::suspicious_else_formatting)]
    fn from_node(
        node: ::tree_sitter::Node<'tree>,
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
                    items.push(
                        <EnumeratorListChildren as ::treesitter_types::FromNode>::from_node(
                            child, src,
                        )?,
                    );
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
#[derive(Debug, Clone)]
pub struct ExpansionStatement<'tree> {
    pub span: ::treesitter_types::Span,
    pub body: Statement<'tree>,
    pub declarator: Declarator<'tree>,
    pub initializer: ::core::option::Option<InitStatement<'tree>>,
    pub right: ExpansionStatementRight<'tree>,
    pub r#type: TypeSpecifier<'tree>,
    pub children: ::std::vec::Vec<ExpansionStatementChildren<'tree>>,
}
impl<'tree> ::treesitter_types::FromNode<'tree> for ExpansionStatement<'tree> {
    #[allow(clippy::match_single_binding, clippy::suspicious_else_formatting)]
    fn from_node(
        node: ::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        debug_assert_eq!(node.kind(), "expansion_statement");
        Ok(Self {
            span: ::treesitter_types::Span::from(node),
            body: {
                let child = node
                    .child_by_field_name("body")
                    .ok_or_else(|| ::treesitter_types::ParseError::missing_field("body", node))?;
                <Statement as ::treesitter_types::FromNode>::from_node(child, src)?
            },
            declarator: {
                let child = node.child_by_field_name("declarator").ok_or_else(|| {
                    ::treesitter_types::ParseError::missing_field("declarator", node)
                })?;
                <Declarator as ::treesitter_types::FromNode>::from_node(child, src)?
            },
            initializer: match node.child_by_field_name("initializer") {
                Some(child) => Some(<InitStatement as ::treesitter_types::FromNode>::from_node(
                    child, src,
                )?),
                None => None,
            },
            right: {
                let child = node
                    .child_by_field_name("right")
                    .ok_or_else(|| ::treesitter_types::ParseError::missing_field("right", node))?;
                <ExpansionStatementRight as ::treesitter_types::FromNode>::from_node(child, src)?
            },
            r#type: {
                let child = node
                    .child_by_field_name("type")
                    .ok_or_else(|| ::treesitter_types::ParseError::missing_field("type", node))?;
                <TypeSpecifier as ::treesitter_types::FromNode>::from_node(child, src)?
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
                        <ExpansionStatementChildren as ::treesitter_types::FromNode>::from_node(
                            child, src,
                        )?,
                    );
                }
                items
            },
        })
    }
}
impl ::treesitter_types::Spanned for ExpansionStatement<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        self.span
    }
}
#[derive(Debug, Clone)]
pub struct ExplicitFunctionSpecifier<'tree> {
    pub span: ::treesitter_types::Span,
    pub children: ::core::option::Option<Expression<'tree>>,
}
impl<'tree> ::treesitter_types::FromNode<'tree> for ExplicitFunctionSpecifier<'tree> {
    #[allow(clippy::match_single_binding, clippy::suspicious_else_formatting)]
    fn from_node(
        node: ::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        debug_assert_eq!(node.kind(), "explicit_function_specifier");
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
                    Some(&child) => Some(<Expression as ::treesitter_types::FromNode>::from_node(
                        child, src,
                    )?),
                    None => None,
                }
            },
        })
    }
}
impl ::treesitter_types::Spanned for ExplicitFunctionSpecifier<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        self.span
    }
}
#[derive(Debug, Clone)]
pub struct ExplicitObjectParameterDeclaration<'tree> {
    pub span: ::treesitter_types::Span,
    pub children: ::std::vec::Vec<ExplicitObjectParameterDeclarationChildren<'tree>>,
}
impl<'tree> ::treesitter_types::FromNode<'tree> for ExplicitObjectParameterDeclaration<'tree> {
    #[allow(clippy::match_single_binding, clippy::suspicious_else_formatting)]
    fn from_node(
        node: ::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        debug_assert_eq!(node.kind(), "explicit_object_parameter_declaration");
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
                            <ExplicitObjectParameterDeclarationChildren as ::treesitter_types::FromNode>::from_node(
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
impl ::treesitter_types::Spanned for ExplicitObjectParameterDeclaration<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        self.span
    }
}
#[derive(Debug, Clone)]
pub struct ExportDeclaration<'tree> {
    pub span: ::treesitter_types::Span,
    pub children: ExportDeclarationChildren<'tree>,
}
impl<'tree> ::treesitter_types::FromNode<'tree> for ExportDeclaration<'tree> {
    #[allow(clippy::match_single_binding, clippy::suspicious_else_formatting)]
    fn from_node(
        node: ::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        debug_assert_eq!(node.kind(), "export_declaration");
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
                <ExportDeclarationChildren as ::treesitter_types::FromNode>::from_node(child, src)?
            },
        })
    }
}
impl ::treesitter_types::Spanned for ExportDeclaration<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        self.span
    }
}
#[derive(Debug, Clone)]
pub struct ExpressionStatement<'tree> {
    pub span: ::treesitter_types::Span,
    pub children: ::core::option::Option<ExpressionStatementChildren<'tree>>,
}
impl<'tree> ::treesitter_types::FromNode<'tree> for ExpressionStatement<'tree> {
    #[allow(clippy::match_single_binding, clippy::suspicious_else_formatting)]
    fn from_node(
        node: ::tree_sitter::Node<'tree>,
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
                    Some(&child) => Some(
                        <ExpressionStatementChildren as ::treesitter_types::FromNode>::from_node(
                            child, src,
                        )?,
                    ),
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
#[derive(Debug, Clone)]
pub struct ExtensionExpression<'tree> {
    pub span: ::treesitter_types::Span,
    pub children: Expression<'tree>,
}
impl<'tree> ::treesitter_types::FromNode<'tree> for ExtensionExpression<'tree> {
    #[allow(clippy::match_single_binding, clippy::suspicious_else_formatting)]
    fn from_node(
        node: ::tree_sitter::Node<'tree>,
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
                <Expression as ::treesitter_types::FromNode>::from_node(child, src)?
            },
        })
    }
}
impl ::treesitter_types::Spanned for ExtensionExpression<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        self.span
    }
}
#[derive(Debug, Clone)]
pub struct FieldDeclaration<'tree> {
    pub span: ::treesitter_types::Span,
    pub declarator: ::std::vec::Vec<FieldDeclarator<'tree>>,
    pub default_value: ::std::vec::Vec<FieldDeclarationDefaultValue<'tree>>,
    pub r#type: TypeSpecifier<'tree>,
    pub children: ::std::vec::Vec<FieldDeclarationChildren<'tree>>,
}
impl<'tree> ::treesitter_types::FromNode<'tree> for FieldDeclaration<'tree> {
    #[allow(clippy::match_single_binding, clippy::suspicious_else_formatting)]
    fn from_node(
        node: ::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        debug_assert_eq!(node.kind(), "field_declaration");
        Ok(Self {
            span: ::treesitter_types::Span::from(node),
            declarator: {
                let mut cursor = node.walk();
                let mut items = ::std::vec::Vec::new();
                for child in node.children_by_field_name("declarator", &mut cursor) {
                    items.push(
                        <FieldDeclarator as ::treesitter_types::FromNode>::from_node(child, src)?,
                    );
                }
                items
            },
            default_value: {
                let mut cursor = node.walk();
                let mut items = ::std::vec::Vec::new();
                for child in node.children_by_field_name("default_value", &mut cursor) {
                    items.push(
                        <FieldDeclarationDefaultValue as ::treesitter_types::FromNode>::from_node(
                            child, src,
                        )?,
                    );
                }
                items
            },
            r#type: {
                let child = node
                    .child_by_field_name("type")
                    .ok_or_else(|| ::treesitter_types::ParseError::missing_field("type", node))?;
                <TypeSpecifier as ::treesitter_types::FromNode>::from_node(child, src)?
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
                        <FieldDeclarationChildren as ::treesitter_types::FromNode>::from_node(
                            child, src,
                        )?,
                    );
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
#[derive(Debug, Clone)]
pub struct FieldDeclarationList<'tree> {
    pub span: ::treesitter_types::Span,
    pub children: ::std::vec::Vec<FieldDeclarationListChildren<'tree>>,
}
impl<'tree> ::treesitter_types::FromNode<'tree> for FieldDeclarationList<'tree> {
    #[allow(clippy::match_single_binding, clippy::suspicious_else_formatting)]
    fn from_node(
        node: ::tree_sitter::Node<'tree>,
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
                    items.push(
                        <FieldDeclarationListChildren as ::treesitter_types::FromNode>::from_node(
                            child, src,
                        )?,
                    );
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
#[derive(Debug, Clone)]
pub struct FieldDesignator<'tree> {
    pub span: ::treesitter_types::Span,
    pub children: FieldIdentifier<'tree>,
}
impl<'tree> ::treesitter_types::FromNode<'tree> for FieldDesignator<'tree> {
    #[allow(clippy::match_single_binding, clippy::suspicious_else_formatting)]
    fn from_node(
        node: ::tree_sitter::Node<'tree>,
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
                <FieldIdentifier as ::treesitter_types::FromNode>::from_node(child, src)?
            },
        })
    }
}
impl ::treesitter_types::Spanned for FieldDesignator<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        self.span
    }
}
#[derive(Debug, Clone)]
pub struct FieldExpression<'tree> {
    pub span: ::treesitter_types::Span,
    pub argument: Expression<'tree>,
    pub field: FieldExpressionField<'tree>,
    pub operator: FieldExpressionOperator,
}
impl<'tree> ::treesitter_types::FromNode<'tree> for FieldExpression<'tree> {
    #[allow(clippy::match_single_binding, clippy::suspicious_else_formatting)]
    fn from_node(
        node: ::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        debug_assert_eq!(node.kind(), "field_expression");
        Ok(Self {
            span: ::treesitter_types::Span::from(node),
            argument: {
                let child = node.child_by_field_name("argument").ok_or_else(|| {
                    ::treesitter_types::ParseError::missing_field("argument", node)
                })?;
                <Expression as ::treesitter_types::FromNode>::from_node(child, src)?
            },
            field: {
                let child = node
                    .child_by_field_name("field")
                    .ok_or_else(|| ::treesitter_types::ParseError::missing_field("field", node))?;
                <FieldExpressionField as ::treesitter_types::FromNode>::from_node(child, src)?
            },
            operator: {
                let child = node.child_by_field_name("operator").ok_or_else(|| {
                    ::treesitter_types::ParseError::missing_field("operator", node)
                })?;
                <FieldExpressionOperator as ::treesitter_types::FromNode>::from_node(child, src)?
            },
        })
    }
}
impl ::treesitter_types::Spanned for FieldExpression<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        self.span
    }
}
#[derive(Debug, Clone)]
pub struct FieldInitializer<'tree> {
    pub span: ::treesitter_types::Span,
    pub children: ::std::vec::Vec<FieldInitializerChildren<'tree>>,
}
impl<'tree> ::treesitter_types::FromNode<'tree> for FieldInitializer<'tree> {
    #[allow(clippy::match_single_binding, clippy::suspicious_else_formatting)]
    fn from_node(
        node: ::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        debug_assert_eq!(node.kind(), "field_initializer");
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
                        <FieldInitializerChildren as ::treesitter_types::FromNode>::from_node(
                            child, src,
                        )?,
                    );
                }
                items
            },
        })
    }
}
impl ::treesitter_types::Spanned for FieldInitializer<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        self.span
    }
}
#[derive(Debug, Clone)]
pub struct FieldInitializerList<'tree> {
    pub span: ::treesitter_types::Span,
    pub children: ::std::vec::Vec<FieldInitializer<'tree>>,
}
impl<'tree> ::treesitter_types::FromNode<'tree> for FieldInitializerList<'tree> {
    #[allow(clippy::match_single_binding, clippy::suspicious_else_formatting)]
    fn from_node(
        node: ::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        debug_assert_eq!(node.kind(), "field_initializer_list");
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
                        <FieldInitializer as ::treesitter_types::FromNode>::from_node(child, src)?,
                    );
                }
                items
            },
        })
    }
}
impl ::treesitter_types::Spanned for FieldInitializerList<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        self.span
    }
}
#[derive(Debug, Clone)]
pub struct FoldExpression<'tree> {
    pub span: ::treesitter_types::Span,
    pub left: FoldExpressionLeft<'tree>,
    pub operator: FoldExpressionOperator,
    pub right: FoldExpressionRight<'tree>,
}
impl<'tree> ::treesitter_types::FromNode<'tree> for FoldExpression<'tree> {
    #[allow(clippy::match_single_binding, clippy::suspicious_else_formatting)]
    fn from_node(
        node: ::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        debug_assert_eq!(node.kind(), "fold_expression");
        Ok(Self {
            span: ::treesitter_types::Span::from(node),
            left: {
                let child = node
                    .child_by_field_name("left")
                    .ok_or_else(|| ::treesitter_types::ParseError::missing_field("left", node))?;
                <FoldExpressionLeft as ::treesitter_types::FromNode>::from_node(child, src)?
            },
            operator: {
                let child = node.child_by_field_name("operator").ok_or_else(|| {
                    ::treesitter_types::ParseError::missing_field("operator", node)
                })?;
                <FoldExpressionOperator as ::treesitter_types::FromNode>::from_node(child, src)?
            },
            right: {
                let child = node
                    .child_by_field_name("right")
                    .ok_or_else(|| ::treesitter_types::ParseError::missing_field("right", node))?;
                <FoldExpressionRight as ::treesitter_types::FromNode>::from_node(child, src)?
            },
        })
    }
}
impl ::treesitter_types::Spanned for FoldExpression<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        self.span
    }
}
#[derive(Debug, Clone)]
pub struct ForRangeLoop<'tree> {
    pub span: ::treesitter_types::Span,
    pub body: Statement<'tree>,
    pub declarator: Declarator<'tree>,
    pub initializer: ::core::option::Option<InitStatement<'tree>>,
    pub right: ForRangeLoopRight<'tree>,
    pub r#type: TypeSpecifier<'tree>,
    pub children: ::std::vec::Vec<ForRangeLoopChildren<'tree>>,
}
impl<'tree> ::treesitter_types::FromNode<'tree> for ForRangeLoop<'tree> {
    #[allow(clippy::match_single_binding, clippy::suspicious_else_formatting)]
    fn from_node(
        node: ::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        debug_assert_eq!(node.kind(), "for_range_loop");
        Ok(Self {
            span: ::treesitter_types::Span::from(node),
            body: {
                let child = node
                    .child_by_field_name("body")
                    .ok_or_else(|| ::treesitter_types::ParseError::missing_field("body", node))?;
                <Statement as ::treesitter_types::FromNode>::from_node(child, src)?
            },
            declarator: {
                let child = node.child_by_field_name("declarator").ok_or_else(|| {
                    ::treesitter_types::ParseError::missing_field("declarator", node)
                })?;
                <Declarator as ::treesitter_types::FromNode>::from_node(child, src)?
            },
            initializer: match node.child_by_field_name("initializer") {
                Some(child) => Some(<InitStatement as ::treesitter_types::FromNode>::from_node(
                    child, src,
                )?),
                None => None,
            },
            right: {
                let child = node
                    .child_by_field_name("right")
                    .ok_or_else(|| ::treesitter_types::ParseError::missing_field("right", node))?;
                <ForRangeLoopRight as ::treesitter_types::FromNode>::from_node(child, src)?
            },
            r#type: {
                let child = node
                    .child_by_field_name("type")
                    .ok_or_else(|| ::treesitter_types::ParseError::missing_field("type", node))?;
                <TypeSpecifier as ::treesitter_types::FromNode>::from_node(child, src)?
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
                        <ForRangeLoopChildren as ::treesitter_types::FromNode>::from_node(
                            child, src,
                        )?,
                    );
                }
                items
            },
        })
    }
}
impl ::treesitter_types::Spanned for ForRangeLoop<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        self.span
    }
}
#[derive(Debug, Clone)]
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
                <Statement as ::treesitter_types::FromNode>::from_node(child, src)?
            },
            condition: match node.child_by_field_name("condition") {
                Some(child) => Some(
                    <ForStatementCondition as ::treesitter_types::FromNode>::from_node(child, src)?,
                ),
                None => None,
            },
            initializer: match node.child_by_field_name("initializer") {
                Some(child) => Some(
                    <ForStatementInitializer as ::treesitter_types::FromNode>::from_node(
                        child, src,
                    )?,
                ),
                None => None,
            },
            update: match node.child_by_field_name("update") {
                Some(child) => Some(
                    <ForStatementUpdate as ::treesitter_types::FromNode>::from_node(child, src)?,
                ),
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
#[derive(Debug, Clone)]
pub struct FriendDeclaration<'tree> {
    pub span: ::treesitter_types::Span,
    pub children: FriendDeclarationChildren<'tree>,
}
impl<'tree> ::treesitter_types::FromNode<'tree> for FriendDeclaration<'tree> {
    #[allow(clippy::match_single_binding, clippy::suspicious_else_formatting)]
    fn from_node(
        node: ::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        debug_assert_eq!(node.kind(), "friend_declaration");
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
                <FriendDeclarationChildren as ::treesitter_types::FromNode>::from_node(child, src)?
            },
        })
    }
}
impl ::treesitter_types::Spanned for FriendDeclaration<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        self.span
    }
}
#[derive(Debug, Clone)]
pub struct FunctionDeclarator<'tree> {
    pub span: ::treesitter_types::Span,
    pub declarator: FunctionDeclaratorDeclarator<'tree>,
    pub parameters: ParameterList<'tree>,
    pub children: ::std::vec::Vec<FunctionDeclaratorChildren<'tree>>,
}
impl<'tree> ::treesitter_types::FromNode<'tree> for FunctionDeclarator<'tree> {
    #[allow(clippy::match_single_binding, clippy::suspicious_else_formatting)]
    fn from_node(
        node: ::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        debug_assert_eq!(node.kind(), "function_declarator");
        Ok(Self {
            span: ::treesitter_types::Span::from(node),
            declarator: {
                let child = node.child_by_field_name("declarator").ok_or_else(|| {
                    ::treesitter_types::ParseError::missing_field("declarator", node)
                })?;
                <FunctionDeclaratorDeclarator as ::treesitter_types::FromNode>::from_node(
                    child, src,
                )?
            },
            parameters: {
                let child = node.child_by_field_name("parameters").ok_or_else(|| {
                    ::treesitter_types::ParseError::missing_field("parameters", node)
                })?;
                <ParameterList as ::treesitter_types::FromNode>::from_node(child, src)?
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
                        <FunctionDeclaratorChildren as ::treesitter_types::FromNode>::from_node(
                            child, src,
                        )?,
                    );
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
#[derive(Debug, Clone)]
pub struct FunctionDefinition<'tree> {
    pub span: ::treesitter_types::Span,
    pub body: ::core::option::Option<FunctionDefinitionBody<'tree>>,
    pub declarator: FunctionDefinitionDeclarator<'tree>,
    pub r#type: ::core::option::Option<TypeSpecifier<'tree>>,
    pub children: ::std::vec::Vec<FunctionDefinitionChildren<'tree>>,
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
                Some(child) => Some(
                    <FunctionDefinitionBody as ::treesitter_types::FromNode>::from_node(
                        child, src,
                    )?,
                ),
                None => None,
            },
            declarator: {
                let child = node.child_by_field_name("declarator").ok_or_else(|| {
                    ::treesitter_types::ParseError::missing_field("declarator", node)
                })?;
                <FunctionDefinitionDeclarator as ::treesitter_types::FromNode>::from_node(
                    child, src,
                )?
            },
            r#type: match node.child_by_field_name("type") {
                Some(child) => Some(<TypeSpecifier as ::treesitter_types::FromNode>::from_node(
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
                        <FunctionDefinitionChildren as ::treesitter_types::FromNode>::from_node(
                            child, src,
                        )?,
                    );
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
#[derive(Debug, Clone)]
pub struct GenericExpression<'tree> {
    pub span: ::treesitter_types::Span,
    pub children: ::std::vec::Vec<GenericExpressionChildren<'tree>>,
}
impl<'tree> ::treesitter_types::FromNode<'tree> for GenericExpression<'tree> {
    #[allow(clippy::match_single_binding, clippy::suspicious_else_formatting)]
    fn from_node(
        node: ::tree_sitter::Node<'tree>,
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
                    items.push(
                        <GenericExpressionChildren as ::treesitter_types::FromNode>::from_node(
                            child, src,
                        )?,
                    );
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
#[derive(Debug, Clone)]
pub struct GlobalModuleFragmentDeclaration<'tree> {
    pub span: ::treesitter_types::Span,
    text: &'tree str,
}
impl<'tree> ::treesitter_types::FromNode<'tree> for GlobalModuleFragmentDeclaration<'tree> {
    fn from_node(
        node: ::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        debug_assert_eq!(node.kind(), "global_module_fragment_declaration");
        Ok(Self {
            span: ::treesitter_types::Span::from(node),
            text: node.utf8_text(src)?,
        })
    }
}
impl<'tree> ::treesitter_types::LeafNode<'tree> for GlobalModuleFragmentDeclaration<'tree> {
    fn text(&self) -> &'tree str {
        self.text
    }
}
impl ::treesitter_types::Spanned for GlobalModuleFragmentDeclaration<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        self.span
    }
}
#[derive(Debug, Clone)]
pub struct GnuAsmClobberList<'tree> {
    pub span: ::treesitter_types::Span,
    pub register: ::std::vec::Vec<GnuAsmClobberListRegister<'tree>>,
}
impl<'tree> ::treesitter_types::FromNode<'tree> for GnuAsmClobberList<'tree> {
    #[allow(clippy::match_single_binding, clippy::suspicious_else_formatting)]
    fn from_node(
        node: ::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        debug_assert_eq!(node.kind(), "gnu_asm_clobber_list");
        Ok(Self {
            span: ::treesitter_types::Span::from(node),
            register: {
                let mut cursor = node.walk();
                let mut items = ::std::vec::Vec::new();
                for child in node.children_by_field_name("register", &mut cursor) {
                    items.push(
                        <GnuAsmClobberListRegister as ::treesitter_types::FromNode>::from_node(
                            child, src,
                        )?,
                    );
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
#[derive(Debug, Clone)]
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
        node: ::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        debug_assert_eq!(node.kind(), "gnu_asm_expression");
        Ok(Self {
            span: ::treesitter_types::Span::from(node),
            assembly_code: {
                let child = node.child_by_field_name("assembly_code").ok_or_else(|| {
                    ::treesitter_types::ParseError::missing_field("assembly_code", node)
                })?;
                <GnuAsmExpressionAssemblyCode as ::treesitter_types::FromNode>::from_node(
                    child, src,
                )?
            },
            clobbers: match node.child_by_field_name("clobbers") {
                Some(child) => Some(
                    <GnuAsmClobberList as ::treesitter_types::FromNode>::from_node(child, src)?,
                ),
                None => None,
            },
            goto_labels: match node.child_by_field_name("goto_labels") {
                Some(child) => Some(<GnuAsmGotoList as ::treesitter_types::FromNode>::from_node(
                    child, src,
                )?),
                None => None,
            },
            input_operands: match node.child_by_field_name("input_operands") {
                Some(child) => Some(
                    <GnuAsmInputOperandList as ::treesitter_types::FromNode>::from_node(
                        child, src,
                    )?,
                ),
                None => None,
            },
            output_operands: match node.child_by_field_name("output_operands") {
                Some(child) => Some(
                    <GnuAsmOutputOperandList as ::treesitter_types::FromNode>::from_node(
                        child, src,
                    )?,
                ),
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
                        <GnuAsmQualifier as ::treesitter_types::FromNode>::from_node(child, src)?,
                    );
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
#[derive(Debug, Clone)]
pub struct GnuAsmGotoList<'tree> {
    pub span: ::treesitter_types::Span,
    pub label: ::std::vec::Vec<Identifier<'tree>>,
}
impl<'tree> ::treesitter_types::FromNode<'tree> for GnuAsmGotoList<'tree> {
    #[allow(clippy::match_single_binding, clippy::suspicious_else_formatting)]
    fn from_node(
        node: ::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        debug_assert_eq!(node.kind(), "gnu_asm_goto_list");
        Ok(Self {
            span: ::treesitter_types::Span::from(node),
            label: {
                let mut cursor = node.walk();
                let mut items = ::std::vec::Vec::new();
                for child in node.children_by_field_name("label", &mut cursor) {
                    items.push(<Identifier as ::treesitter_types::FromNode>::from_node(
                        child, src,
                    )?);
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
#[derive(Debug, Clone)]
pub struct GnuAsmInputOperand<'tree> {
    pub span: ::treesitter_types::Span,
    pub constraint: StringLiteral<'tree>,
    pub symbol: ::core::option::Option<Identifier<'tree>>,
    pub value: Expression<'tree>,
}
impl<'tree> ::treesitter_types::FromNode<'tree> for GnuAsmInputOperand<'tree> {
    #[allow(clippy::match_single_binding, clippy::suspicious_else_formatting)]
    fn from_node(
        node: ::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        debug_assert_eq!(node.kind(), "gnu_asm_input_operand");
        Ok(Self {
            span: ::treesitter_types::Span::from(node),
            constraint: {
                let child = node.child_by_field_name("constraint").ok_or_else(|| {
                    ::treesitter_types::ParseError::missing_field("constraint", node)
                })?;
                <StringLiteral as ::treesitter_types::FromNode>::from_node(child, src)?
            },
            symbol: match node.child_by_field_name("symbol") {
                Some(child) => Some(<Identifier as ::treesitter_types::FromNode>::from_node(
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
impl ::treesitter_types::Spanned for GnuAsmInputOperand<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        self.span
    }
}
#[derive(Debug, Clone)]
pub struct GnuAsmInputOperandList<'tree> {
    pub span: ::treesitter_types::Span,
    pub operand: ::std::vec::Vec<GnuAsmInputOperand<'tree>>,
}
impl<'tree> ::treesitter_types::FromNode<'tree> for GnuAsmInputOperandList<'tree> {
    #[allow(clippy::match_single_binding, clippy::suspicious_else_formatting)]
    fn from_node(
        node: ::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        debug_assert_eq!(node.kind(), "gnu_asm_input_operand_list");
        Ok(Self {
            span: ::treesitter_types::Span::from(node),
            operand: {
                let mut cursor = node.walk();
                let mut items = ::std::vec::Vec::new();
                for child in node.children_by_field_name("operand", &mut cursor) {
                    items.push(
                        <GnuAsmInputOperand as ::treesitter_types::FromNode>::from_node(
                            child, src,
                        )?,
                    );
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
#[derive(Debug, Clone)]
pub struct GnuAsmOutputOperand<'tree> {
    pub span: ::treesitter_types::Span,
    pub constraint: StringLiteral<'tree>,
    pub symbol: ::core::option::Option<Identifier<'tree>>,
    pub value: Expression<'tree>,
}
impl<'tree> ::treesitter_types::FromNode<'tree> for GnuAsmOutputOperand<'tree> {
    #[allow(clippy::match_single_binding, clippy::suspicious_else_formatting)]
    fn from_node(
        node: ::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        debug_assert_eq!(node.kind(), "gnu_asm_output_operand");
        Ok(Self {
            span: ::treesitter_types::Span::from(node),
            constraint: {
                let child = node.child_by_field_name("constraint").ok_or_else(|| {
                    ::treesitter_types::ParseError::missing_field("constraint", node)
                })?;
                <StringLiteral as ::treesitter_types::FromNode>::from_node(child, src)?
            },
            symbol: match node.child_by_field_name("symbol") {
                Some(child) => Some(<Identifier as ::treesitter_types::FromNode>::from_node(
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
impl ::treesitter_types::Spanned for GnuAsmOutputOperand<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        self.span
    }
}
#[derive(Debug, Clone)]
pub struct GnuAsmOutputOperandList<'tree> {
    pub span: ::treesitter_types::Span,
    pub operand: ::std::vec::Vec<GnuAsmOutputOperand<'tree>>,
}
impl<'tree> ::treesitter_types::FromNode<'tree> for GnuAsmOutputOperandList<'tree> {
    #[allow(clippy::match_single_binding, clippy::suspicious_else_formatting)]
    fn from_node(
        node: ::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        debug_assert_eq!(node.kind(), "gnu_asm_output_operand_list");
        Ok(Self {
            span: ::treesitter_types::Span::from(node),
            operand: {
                let mut cursor = node.walk();
                let mut items = ::std::vec::Vec::new();
                for child in node.children_by_field_name("operand", &mut cursor) {
                    items.push(
                        <GnuAsmOutputOperand as ::treesitter_types::FromNode>::from_node(
                            child, src,
                        )?,
                    );
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
#[derive(Debug, Clone)]
pub struct GnuAsmQualifier<'tree> {
    pub span: ::treesitter_types::Span,
    text: &'tree str,
}
impl<'tree> ::treesitter_types::FromNode<'tree> for GnuAsmQualifier<'tree> {
    fn from_node(
        node: ::tree_sitter::Node<'tree>,
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
#[derive(Debug, Clone)]
pub struct GotoStatement<'tree> {
    pub span: ::treesitter_types::Span,
    pub label: StatementIdentifier<'tree>,
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
            label: {
                let child = node
                    .child_by_field_name("label")
                    .ok_or_else(|| ::treesitter_types::ParseError::missing_field("label", node))?;
                <StatementIdentifier as ::treesitter_types::FromNode>::from_node(child, src)?
            },
        })
    }
}
impl ::treesitter_types::Spanned for GotoStatement<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        self.span
    }
}
#[derive(Debug, Clone)]
pub struct IfStatement<'tree> {
    pub span: ::treesitter_types::Span,
    pub alternative: ::core::option::Option<ElseClause<'tree>>,
    pub condition: ConditionClause<'tree>,
    pub consequence: Statement<'tree>,
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
            alternative: match node.child_by_field_name("alternative") {
                Some(child) => Some(<ElseClause as ::treesitter_types::FromNode>::from_node(
                    child, src,
                )?),
                None => None,
            },
            condition: {
                let child = node.child_by_field_name("condition").ok_or_else(|| {
                    ::treesitter_types::ParseError::missing_field("condition", node)
                })?;
                <ConditionClause as ::treesitter_types::FromNode>::from_node(child, src)?
            },
            consequence: {
                let child = node.child_by_field_name("consequence").ok_or_else(|| {
                    ::treesitter_types::ParseError::missing_field("consequence", node)
                })?;
                <Statement as ::treesitter_types::FromNode>::from_node(child, src)?
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
    pub header: ::core::option::Option<ImportDeclarationHeader<'tree>>,
    pub name: ::core::option::Option<ModuleName<'tree>>,
    pub partition: ::core::option::Option<ModulePartition<'tree>>,
    pub children: ::core::option::Option<AttributeDeclaration<'tree>>,
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
            header: match node.child_by_field_name("header") {
                Some(child) => Some(
                    <ImportDeclarationHeader as ::treesitter_types::FromNode>::from_node(
                        child, src,
                    )?,
                ),
                None => None,
            },
            name: match node.child_by_field_name("name") {
                Some(child) => Some(<ModuleName as ::treesitter_types::FromNode>::from_node(
                    child, src,
                )?),
                None => None,
            },
            partition: match node.child_by_field_name("partition") {
                Some(child) => {
                    Some(<ModulePartition as ::treesitter_types::FromNode>::from_node(child, src)?)
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
                            if cursor.field_name().is_none()
                                && cursor.node().is_named()
                                && !cursor.node().is_extra()
                            {
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
                        <AttributeDeclaration as ::treesitter_types::FromNode>::from_node(
                            child, src,
                        )?,
                    ),
                    None => None,
                }
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
pub struct InitDeclarator<'tree> {
    pub span: ::treesitter_types::Span,
    pub declarator: Declarator<'tree>,
    pub value: InitDeclaratorValue<'tree>,
}
impl<'tree> ::treesitter_types::FromNode<'tree> for InitDeclarator<'tree> {
    #[allow(clippy::match_single_binding, clippy::suspicious_else_formatting)]
    fn from_node(
        node: ::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        debug_assert_eq!(node.kind(), "init_declarator");
        Ok(Self {
            span: ::treesitter_types::Span::from(node),
            declarator: {
                let child = node.child_by_field_name("declarator").ok_or_else(|| {
                    ::treesitter_types::ParseError::missing_field("declarator", node)
                })?;
                <Declarator as ::treesitter_types::FromNode>::from_node(child, src)?
            },
            value: {
                let child = node
                    .child_by_field_name("value")
                    .ok_or_else(|| ::treesitter_types::ParseError::missing_field("value", node))?;
                <InitDeclaratorValue as ::treesitter_types::FromNode>::from_node(child, src)?
            },
        })
    }
}
impl ::treesitter_types::Spanned for InitDeclarator<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        self.span
    }
}
#[derive(Debug, Clone)]
pub struct InitStatement<'tree> {
    pub span: ::treesitter_types::Span,
    pub children: InitStatementChildren<'tree>,
}
impl<'tree> ::treesitter_types::FromNode<'tree> for InitStatement<'tree> {
    #[allow(clippy::match_single_binding, clippy::suspicious_else_formatting)]
    fn from_node(
        node: ::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        debug_assert_eq!(node.kind(), "init_statement");
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
                <InitStatementChildren as ::treesitter_types::FromNode>::from_node(child, src)?
            },
        })
    }
}
impl ::treesitter_types::Spanned for InitStatement<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        self.span
    }
}
#[derive(Debug, Clone)]
pub struct InitializerList<'tree> {
    pub span: ::treesitter_types::Span,
    pub children: ::std::vec::Vec<InitializerListChildren<'tree>>,
}
impl<'tree> ::treesitter_types::FromNode<'tree> for InitializerList<'tree> {
    #[allow(clippy::match_single_binding, clippy::suspicious_else_formatting)]
    fn from_node(
        node: ::tree_sitter::Node<'tree>,
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
                    items.push(
                        <InitializerListChildren as ::treesitter_types::FromNode>::from_node(
                            child, src,
                        )?,
                    );
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
#[derive(Debug, Clone)]
pub struct InitializerPair<'tree> {
    pub span: ::treesitter_types::Span,
    pub designator: ::std::vec::Vec<InitializerPairDesignator<'tree>>,
    pub value: InitializerPairValue<'tree>,
}
impl<'tree> ::treesitter_types::FromNode<'tree> for InitializerPair<'tree> {
    #[allow(clippy::match_single_binding, clippy::suspicious_else_formatting)]
    fn from_node(
        node: ::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        debug_assert_eq!(node.kind(), "initializer_pair");
        Ok(Self {
            span: ::treesitter_types::Span::from(node),
            designator: {
                let mut cursor = node.walk();
                let mut items = ::std::vec::Vec::new();
                for child in node.children_by_field_name("designator", &mut cursor) {
                    items.push(
                        <InitializerPairDesignator as ::treesitter_types::FromNode>::from_node(
                            child, src,
                        )?,
                    );
                }
                items
            },
            value: {
                let child = node
                    .child_by_field_name("value")
                    .ok_or_else(|| ::treesitter_types::ParseError::missing_field("value", node))?;
                <InitializerPairValue as ::treesitter_types::FromNode>::from_node(child, src)?
            },
        })
    }
}
impl ::treesitter_types::Spanned for InitializerPair<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        self.span
    }
}
#[derive(Debug, Clone)]
pub struct LabeledStatement<'tree> {
    pub span: ::treesitter_types::Span,
    pub label: StatementIdentifier<'tree>,
    pub children: LabeledStatementChildren<'tree>,
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
            label: {
                let child = node
                    .child_by_field_name("label")
                    .ok_or_else(|| ::treesitter_types::ParseError::missing_field("label", node))?;
                <StatementIdentifier as ::treesitter_types::FromNode>::from_node(child, src)?
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
                <LabeledStatementChildren as ::treesitter_types::FromNode>::from_node(child, src)?
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
pub struct LambdaCaptureInitializer<'tree> {
    pub span: ::treesitter_types::Span,
    pub left: Identifier<'tree>,
    pub right: Expression<'tree>,
}
impl<'tree> ::treesitter_types::FromNode<'tree> for LambdaCaptureInitializer<'tree> {
    #[allow(clippy::match_single_binding, clippy::suspicious_else_formatting)]
    fn from_node(
        node: ::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        debug_assert_eq!(node.kind(), "lambda_capture_initializer");
        Ok(Self {
            span: ::treesitter_types::Span::from(node),
            left: {
                let child = node
                    .child_by_field_name("left")
                    .ok_or_else(|| ::treesitter_types::ParseError::missing_field("left", node))?;
                <Identifier as ::treesitter_types::FromNode>::from_node(child, src)?
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
impl ::treesitter_types::Spanned for LambdaCaptureInitializer<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        self.span
    }
}
#[derive(Debug, Clone)]
pub struct LambdaCaptureSpecifier<'tree> {
    pub span: ::treesitter_types::Span,
    pub children: ::std::vec::Vec<LambdaCaptureSpecifierChildren<'tree>>,
}
impl<'tree> ::treesitter_types::FromNode<'tree> for LambdaCaptureSpecifier<'tree> {
    #[allow(clippy::match_single_binding, clippy::suspicious_else_formatting)]
    fn from_node(
        node: ::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        debug_assert_eq!(node.kind(), "lambda_capture_specifier");
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
                            <LambdaCaptureSpecifierChildren as ::treesitter_types::FromNode>::from_node(
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
impl ::treesitter_types::Spanned for LambdaCaptureSpecifier<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        self.span
    }
}
#[derive(Debug, Clone)]
pub struct LambdaDeclarator<'tree> {
    pub span: ::treesitter_types::Span,
    pub parameters: ::core::option::Option<ParameterList<'tree>>,
    pub children: ::std::vec::Vec<LambdaDeclaratorChildren<'tree>>,
}
impl<'tree> ::treesitter_types::FromNode<'tree> for LambdaDeclarator<'tree> {
    #[allow(clippy::match_single_binding, clippy::suspicious_else_formatting)]
    fn from_node(
        node: ::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        debug_assert_eq!(node.kind(), "lambda_declarator");
        Ok(Self {
            span: ::treesitter_types::Span::from(node),
            parameters: match node.child_by_field_name("parameters") {
                Some(child) => Some(<ParameterList as ::treesitter_types::FromNode>::from_node(
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
                        <LambdaDeclaratorChildren as ::treesitter_types::FromNode>::from_node(
                            child, src,
                        )?,
                    );
                }
                items
            },
        })
    }
}
impl ::treesitter_types::Spanned for LambdaDeclarator<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        self.span
    }
}
#[derive(Debug, Clone)]
pub struct LambdaDefaultCapture<'tree> {
    pub span: ::treesitter_types::Span,
    text: &'tree str,
}
impl<'tree> ::treesitter_types::FromNode<'tree> for LambdaDefaultCapture<'tree> {
    fn from_node(
        node: ::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        debug_assert_eq!(node.kind(), "lambda_default_capture");
        Ok(Self {
            span: ::treesitter_types::Span::from(node),
            text: node.utf8_text(src)?,
        })
    }
}
impl<'tree> ::treesitter_types::LeafNode<'tree> for LambdaDefaultCapture<'tree> {
    fn text(&self) -> &'tree str {
        self.text
    }
}
impl ::treesitter_types::Spanned for LambdaDefaultCapture<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        self.span
    }
}
#[derive(Debug, Clone)]
pub struct LambdaExpression<'tree> {
    pub span: ::treesitter_types::Span,
    pub body: CompoundStatement<'tree>,
    pub captures: LambdaCaptureSpecifier<'tree>,
    pub constraint: ::core::option::Option<RequiresClause<'tree>>,
    pub declarator: ::core::option::Option<LambdaDeclarator<'tree>>,
    pub template_parameters: ::core::option::Option<TemplateParameterList<'tree>>,
}
impl<'tree> ::treesitter_types::FromNode<'tree> for LambdaExpression<'tree> {
    #[allow(clippy::match_single_binding, clippy::suspicious_else_formatting)]
    fn from_node(
        node: ::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        debug_assert_eq!(node.kind(), "lambda_expression");
        Ok(Self {
            span: ::treesitter_types::Span::from(node),
            body: {
                let child = node
                    .child_by_field_name("body")
                    .ok_or_else(|| ::treesitter_types::ParseError::missing_field("body", node))?;
                <CompoundStatement as ::treesitter_types::FromNode>::from_node(child, src)?
            },
            captures: {
                let child = node.child_by_field_name("captures").ok_or_else(|| {
                    ::treesitter_types::ParseError::missing_field("captures", node)
                })?;
                <LambdaCaptureSpecifier as ::treesitter_types::FromNode>::from_node(child, src)?
            },
            constraint: match node.child_by_field_name("constraint") {
                Some(child) => Some(<RequiresClause as ::treesitter_types::FromNode>::from_node(
                    child, src,
                )?),
                None => None,
            },
            declarator: match node.child_by_field_name("declarator") {
                Some(child) => {
                    Some(<LambdaDeclarator as ::treesitter_types::FromNode>::from_node(child, src)?)
                }
                None => None,
            },
            template_parameters: match node.child_by_field_name("template_parameters") {
                Some(child) => Some(
                    <TemplateParameterList as ::treesitter_types::FromNode>::from_node(child, src)?,
                ),
                None => None,
            },
        })
    }
}
impl ::treesitter_types::Spanned for LambdaExpression<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        self.span
    }
}
#[derive(Debug, Clone)]
pub struct LambdaSpecifier<'tree> {
    pub span: ::treesitter_types::Span,
    text: &'tree str,
}
impl<'tree> ::treesitter_types::FromNode<'tree> for LambdaSpecifier<'tree> {
    fn from_node(
        node: ::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        debug_assert_eq!(node.kind(), "lambda_specifier");
        Ok(Self {
            span: ::treesitter_types::Span::from(node),
            text: node.utf8_text(src)?,
        })
    }
}
impl<'tree> ::treesitter_types::LeafNode<'tree> for LambdaSpecifier<'tree> {
    fn text(&self) -> &'tree str {
        self.text
    }
}
impl ::treesitter_types::Spanned for LambdaSpecifier<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        self.span
    }
}
#[derive(Debug, Clone)]
pub struct LinkageSpecification<'tree> {
    pub span: ::treesitter_types::Span,
    pub body: LinkageSpecificationBody<'tree>,
    pub value: StringLiteral<'tree>,
}
impl<'tree> ::treesitter_types::FromNode<'tree> for LinkageSpecification<'tree> {
    #[allow(clippy::match_single_binding, clippy::suspicious_else_formatting)]
    fn from_node(
        node: ::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        debug_assert_eq!(node.kind(), "linkage_specification");
        Ok(Self {
            span: ::treesitter_types::Span::from(node),
            body: {
                let child = node
                    .child_by_field_name("body")
                    .ok_or_else(|| ::treesitter_types::ParseError::missing_field("body", node))?;
                <LinkageSpecificationBody as ::treesitter_types::FromNode>::from_node(child, src)?
            },
            value: {
                let child = node
                    .child_by_field_name("value")
                    .ok_or_else(|| ::treesitter_types::ParseError::missing_field("value", node))?;
                <StringLiteral as ::treesitter_types::FromNode>::from_node(child, src)?
            },
        })
    }
}
impl ::treesitter_types::Spanned for LinkageSpecification<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        self.span
    }
}
#[derive(Debug, Clone)]
pub struct ModuleDeclaration<'tree> {
    pub span: ::treesitter_types::Span,
    pub name: ModuleName<'tree>,
    pub partition: ::core::option::Option<ModulePartition<'tree>>,
    pub children: ::core::option::Option<AttributeDeclaration<'tree>>,
}
impl<'tree> ::treesitter_types::FromNode<'tree> for ModuleDeclaration<'tree> {
    #[allow(clippy::match_single_binding, clippy::suspicious_else_formatting)]
    fn from_node(
        node: ::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        debug_assert_eq!(node.kind(), "module_declaration");
        Ok(Self {
            span: ::treesitter_types::Span::from(node),
            name: {
                let child = node
                    .child_by_field_name("name")
                    .ok_or_else(|| ::treesitter_types::ParseError::missing_field("name", node))?;
                <ModuleName as ::treesitter_types::FromNode>::from_node(child, src)?
            },
            partition: match node.child_by_field_name("partition") {
                Some(child) => {
                    Some(<ModulePartition as ::treesitter_types::FromNode>::from_node(child, src)?)
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
                            if cursor.field_name().is_none()
                                && cursor.node().is_named()
                                && !cursor.node().is_extra()
                            {
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
                        <AttributeDeclaration as ::treesitter_types::FromNode>::from_node(
                            child, src,
                        )?,
                    ),
                    None => None,
                }
            },
        })
    }
}
impl ::treesitter_types::Spanned for ModuleDeclaration<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        self.span
    }
}
#[derive(Debug, Clone)]
pub struct ModuleName<'tree> {
    pub span: ::treesitter_types::Span,
    pub children: ::std::vec::Vec<Identifier<'tree>>,
}
impl<'tree> ::treesitter_types::FromNode<'tree> for ModuleName<'tree> {
    #[allow(clippy::match_single_binding, clippy::suspicious_else_formatting)]
    fn from_node(
        node: ::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        debug_assert_eq!(node.kind(), "module_name");
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
                    items.push(<Identifier as ::treesitter_types::FromNode>::from_node(
                        child, src,
                    )?);
                }
                items
            },
        })
    }
}
impl ::treesitter_types::Spanned for ModuleName<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        self.span
    }
}
#[derive(Debug, Clone)]
pub struct ModulePartition<'tree> {
    pub span: ::treesitter_types::Span,
    pub children: ModuleName<'tree>,
}
impl<'tree> ::treesitter_types::FromNode<'tree> for ModulePartition<'tree> {
    #[allow(clippy::match_single_binding, clippy::suspicious_else_formatting)]
    fn from_node(
        node: ::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        debug_assert_eq!(node.kind(), "module_partition");
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
                <ModuleName as ::treesitter_types::FromNode>::from_node(child, src)?
            },
        })
    }
}
impl ::treesitter_types::Spanned for ModulePartition<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        self.span
    }
}
#[derive(Debug, Clone)]
pub struct MsBasedModifier<'tree> {
    pub span: ::treesitter_types::Span,
    pub children: ArgumentList<'tree>,
}
impl<'tree> ::treesitter_types::FromNode<'tree> for MsBasedModifier<'tree> {
    #[allow(clippy::match_single_binding, clippy::suspicious_else_formatting)]
    fn from_node(
        node: ::tree_sitter::Node<'tree>,
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
                <ArgumentList as ::treesitter_types::FromNode>::from_node(child, src)?
            },
        })
    }
}
impl ::treesitter_types::Spanned for MsBasedModifier<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        self.span
    }
}
#[derive(Debug, Clone)]
pub struct MsCallModifier<'tree> {
    pub span: ::treesitter_types::Span,
    text: &'tree str,
}
impl<'tree> ::treesitter_types::FromNode<'tree> for MsCallModifier<'tree> {
    fn from_node(
        node: ::tree_sitter::Node<'tree>,
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
#[derive(Debug, Clone)]
pub struct MsDeclspecModifier<'tree> {
    pub span: ::treesitter_types::Span,
    pub children: Identifier<'tree>,
}
impl<'tree> ::treesitter_types::FromNode<'tree> for MsDeclspecModifier<'tree> {
    #[allow(clippy::match_single_binding, clippy::suspicious_else_formatting)]
    fn from_node(
        node: ::tree_sitter::Node<'tree>,
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
                <Identifier as ::treesitter_types::FromNode>::from_node(child, src)?
            },
        })
    }
}
impl ::treesitter_types::Spanned for MsDeclspecModifier<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        self.span
    }
}
#[derive(Debug, Clone)]
pub struct MsPointerModifier<'tree> {
    pub span: ::treesitter_types::Span,
    pub children: MsPointerModifierChildren<'tree>,
}
impl<'tree> ::treesitter_types::FromNode<'tree> for MsPointerModifier<'tree> {
    #[allow(clippy::match_single_binding, clippy::suspicious_else_formatting)]
    fn from_node(
        node: ::tree_sitter::Node<'tree>,
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
                <MsPointerModifierChildren as ::treesitter_types::FromNode>::from_node(child, src)?
            },
        })
    }
}
impl ::treesitter_types::Spanned for MsPointerModifier<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        self.span
    }
}
#[derive(Debug, Clone)]
pub struct MsUnalignedPtrModifier<'tree> {
    pub span: ::treesitter_types::Span,
    text: &'tree str,
}
impl<'tree> ::treesitter_types::FromNode<'tree> for MsUnalignedPtrModifier<'tree> {
    fn from_node(
        node: ::tree_sitter::Node<'tree>,
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
#[derive(Debug, Clone)]
pub struct NamespaceAliasDefinition<'tree> {
    pub span: ::treesitter_types::Span,
    pub name: NamespaceIdentifier<'tree>,
    pub children: NamespaceAliasDefinitionChildren<'tree>,
}
impl<'tree> ::treesitter_types::FromNode<'tree> for NamespaceAliasDefinition<'tree> {
    #[allow(clippy::match_single_binding, clippy::suspicious_else_formatting)]
    fn from_node(
        node: ::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        debug_assert_eq!(node.kind(), "namespace_alias_definition");
        Ok(Self {
            span: ::treesitter_types::Span::from(node),
            name: {
                let child = node
                    .child_by_field_name("name")
                    .ok_or_else(|| ::treesitter_types::ParseError::missing_field("name", node))?;
                <NamespaceIdentifier as ::treesitter_types::FromNode>::from_node(child, src)?
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
                <NamespaceAliasDefinitionChildren as ::treesitter_types::FromNode>::from_node(
                    child, src,
                )?
            },
        })
    }
}
impl ::treesitter_types::Spanned for NamespaceAliasDefinition<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        self.span
    }
}
#[derive(Debug, Clone)]
pub struct NamespaceDefinition<'tree> {
    pub span: ::treesitter_types::Span,
    pub body: DeclarationList<'tree>,
    pub name: ::core::option::Option<NamespaceDefinitionName<'tree>>,
    pub children: ::core::option::Option<AttributeDeclaration<'tree>>,
}
impl<'tree> ::treesitter_types::FromNode<'tree> for NamespaceDefinition<'tree> {
    #[allow(clippy::match_single_binding, clippy::suspicious_else_formatting)]
    fn from_node(
        node: ::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        debug_assert_eq!(node.kind(), "namespace_definition");
        Ok(Self {
            span: ::treesitter_types::Span::from(node),
            body: {
                let child = node
                    .child_by_field_name("body")
                    .ok_or_else(|| ::treesitter_types::ParseError::missing_field("body", node))?;
                <DeclarationList as ::treesitter_types::FromNode>::from_node(child, src)?
            },
            name: match node.child_by_field_name("name") {
                Some(child) => Some(
                    <NamespaceDefinitionName as ::treesitter_types::FromNode>::from_node(
                        child, src,
                    )?,
                ),
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
                    Some(&child) => Some(
                        <AttributeDeclaration as ::treesitter_types::FromNode>::from_node(
                            child, src,
                        )?,
                    ),
                    None => None,
                }
            },
        })
    }
}
impl ::treesitter_types::Spanned for NamespaceDefinition<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        self.span
    }
}
#[derive(Debug, Clone)]
pub struct NestedNamespaceSpecifier<'tree> {
    pub span: ::treesitter_types::Span,
    pub children: ::std::vec::Vec<NestedNamespaceSpecifierChildren<'tree>>,
}
impl<'tree> ::treesitter_types::FromNode<'tree> for NestedNamespaceSpecifier<'tree> {
    #[allow(clippy::match_single_binding, clippy::suspicious_else_formatting)]
    fn from_node(
        node: ::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        debug_assert_eq!(node.kind(), "nested_namespace_specifier");
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
                            <NestedNamespaceSpecifierChildren as ::treesitter_types::FromNode>::from_node(
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
impl ::treesitter_types::Spanned for NestedNamespaceSpecifier<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        self.span
    }
}
#[derive(Debug, Clone)]
pub struct NewDeclarator<'tree> {
    pub span: ::treesitter_types::Span,
    pub length: Expression<'tree>,
    pub children: ::core::option::Option<::std::boxed::Box<NewDeclarator<'tree>>>,
}
impl<'tree> ::treesitter_types::FromNode<'tree> for NewDeclarator<'tree> {
    #[allow(clippy::match_single_binding, clippy::suspicious_else_formatting)]
    fn from_node(
        node: ::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        debug_assert_eq!(node.kind(), "new_declarator");
        Ok(Self {
            span: ::treesitter_types::Span::from(node),
            length: {
                let child = node
                    .child_by_field_name("length")
                    .ok_or_else(|| ::treesitter_types::ParseError::missing_field("length", node))?;
                <Expression as ::treesitter_types::FromNode>::from_node(child, src)?
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
                    Some(&child) => Some(::std::boxed::Box::new(
                        <NewDeclarator as ::treesitter_types::FromNode>::from_node(child, src)?,
                    )),
                    None => None,
                }
            },
        })
    }
}
impl ::treesitter_types::Spanned for NewDeclarator<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        self.span
    }
}
#[derive(Debug, Clone)]
pub struct NewExpression<'tree> {
    pub span: ::treesitter_types::Span,
    pub arguments: ::core::option::Option<NewExpressionArguments<'tree>>,
    pub declarator: ::core::option::Option<NewDeclarator<'tree>>,
    pub placement: ::core::option::Option<ArgumentList<'tree>>,
    pub r#type: TypeSpecifier<'tree>,
}
impl<'tree> ::treesitter_types::FromNode<'tree> for NewExpression<'tree> {
    #[allow(clippy::match_single_binding, clippy::suspicious_else_formatting)]
    fn from_node(
        node: ::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        debug_assert_eq!(node.kind(), "new_expression");
        Ok(Self {
            span: ::treesitter_types::Span::from(node),
            arguments: match node.child_by_field_name("arguments") {
                Some(child) => Some(
                    <NewExpressionArguments as ::treesitter_types::FromNode>::from_node(
                        child, src,
                    )?,
                ),
                None => None,
            },
            declarator: match node.child_by_field_name("declarator") {
                Some(child) => Some(<NewDeclarator as ::treesitter_types::FromNode>::from_node(
                    child, src,
                )?),
                None => None,
            },
            placement: match node.child_by_field_name("placement") {
                Some(child) => Some(<ArgumentList as ::treesitter_types::FromNode>::from_node(
                    child, src,
                )?),
                None => None,
            },
            r#type: {
                let child = node
                    .child_by_field_name("type")
                    .ok_or_else(|| ::treesitter_types::ParseError::missing_field("type", node))?;
                <TypeSpecifier as ::treesitter_types::FromNode>::from_node(child, src)?
            },
        })
    }
}
impl ::treesitter_types::Spanned for NewExpression<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        self.span
    }
}
#[derive(Debug, Clone)]
pub struct Noexcept<'tree> {
    pub span: ::treesitter_types::Span,
    pub children: ::core::option::Option<Expression<'tree>>,
}
impl<'tree> ::treesitter_types::FromNode<'tree> for Noexcept<'tree> {
    #[allow(clippy::match_single_binding, clippy::suspicious_else_formatting)]
    fn from_node(
        node: ::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        debug_assert_eq!(node.kind(), "noexcept");
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
                    Some(&child) => Some(<Expression as ::treesitter_types::FromNode>::from_node(
                        child, src,
                    )?),
                    None => None,
                }
            },
        })
    }
}
impl ::treesitter_types::Spanned for Noexcept<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        self.span
    }
}
#[derive(Debug, Clone)]
pub struct Null<'tree> {
    pub span: ::treesitter_types::Span,
    text: &'tree str,
}
impl<'tree> ::treesitter_types::FromNode<'tree> for Null<'tree> {
    fn from_node(
        node: ::tree_sitter::Node<'tree>,
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
#[derive(Debug, Clone)]
pub struct OffsetofExpression<'tree> {
    pub span: ::treesitter_types::Span,
    pub member: FieldIdentifier<'tree>,
    pub r#type: TypeDescriptor<'tree>,
}
impl<'tree> ::treesitter_types::FromNode<'tree> for OffsetofExpression<'tree> {
    #[allow(clippy::match_single_binding, clippy::suspicious_else_formatting)]
    fn from_node(
        node: ::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        debug_assert_eq!(node.kind(), "offsetof_expression");
        Ok(Self {
            span: ::treesitter_types::Span::from(node),
            member: {
                let child = node
                    .child_by_field_name("member")
                    .ok_or_else(|| ::treesitter_types::ParseError::missing_field("member", node))?;
                <FieldIdentifier as ::treesitter_types::FromNode>::from_node(child, src)?
            },
            r#type: {
                let child = node
                    .child_by_field_name("type")
                    .ok_or_else(|| ::treesitter_types::ParseError::missing_field("type", node))?;
                <TypeDescriptor as ::treesitter_types::FromNode>::from_node(child, src)?
            },
        })
    }
}
impl ::treesitter_types::Spanned for OffsetofExpression<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        self.span
    }
}
#[derive(Debug, Clone)]
pub struct OperatorCast<'tree> {
    pub span: ::treesitter_types::Span,
    pub declarator: AbstractDeclarator<'tree>,
    pub r#type: TypeSpecifier<'tree>,
    pub children: ::std::vec::Vec<OperatorCastChildren<'tree>>,
}
impl<'tree> ::treesitter_types::FromNode<'tree> for OperatorCast<'tree> {
    #[allow(clippy::match_single_binding, clippy::suspicious_else_formatting)]
    fn from_node(
        node: ::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        debug_assert_eq!(node.kind(), "operator_cast");
        Ok(Self {
            span: ::treesitter_types::Span::from(node),
            declarator: {
                let child = node.child_by_field_name("declarator").ok_or_else(|| {
                    ::treesitter_types::ParseError::missing_field("declarator", node)
                })?;
                <AbstractDeclarator as ::treesitter_types::FromNode>::from_node(child, src)?
            },
            r#type: {
                let child = node
                    .child_by_field_name("type")
                    .ok_or_else(|| ::treesitter_types::ParseError::missing_field("type", node))?;
                <TypeSpecifier as ::treesitter_types::FromNode>::from_node(child, src)?
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
                        <OperatorCastChildren as ::treesitter_types::FromNode>::from_node(
                            child, src,
                        )?,
                    );
                }
                items
            },
        })
    }
}
impl ::treesitter_types::Spanned for OperatorCast<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        self.span
    }
}
#[derive(Debug, Clone)]
pub struct OperatorName<'tree> {
    pub span: ::treesitter_types::Span,
    pub children: ::core::option::Option<Identifier<'tree>>,
}
impl<'tree> ::treesitter_types::FromNode<'tree> for OperatorName<'tree> {
    #[allow(clippy::match_single_binding, clippy::suspicious_else_formatting)]
    fn from_node(
        node: ::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        debug_assert_eq!(node.kind(), "operator_name");
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
impl ::treesitter_types::Spanned for OperatorName<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        self.span
    }
}
#[derive(Debug, Clone)]
pub struct OptionalParameterDeclaration<'tree> {
    pub span: ::treesitter_types::Span,
    pub declarator: ::core::option::Option<OptionalParameterDeclarationDeclarator<'tree>>,
    pub default_value: Expression<'tree>,
    pub r#type: TypeSpecifier<'tree>,
    pub children: ::std::vec::Vec<OptionalParameterDeclarationChildren<'tree>>,
}
impl<'tree> ::treesitter_types::FromNode<'tree> for OptionalParameterDeclaration<'tree> {
    #[allow(clippy::match_single_binding, clippy::suspicious_else_formatting)]
    fn from_node(
        node: ::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        debug_assert_eq!(node.kind(), "optional_parameter_declaration");
        Ok(Self {
            span: ::treesitter_types::Span::from(node),
            declarator: match node.child_by_field_name("declarator") {
                Some(child) => {
                    Some(
                        <OptionalParameterDeclarationDeclarator as ::treesitter_types::FromNode>::from_node(
                            child,
                            src,
                        )?,
                    )
                }
                None => None,
            },
            default_value: {
                let child = node
                    .child_by_field_name("default_value")
                    .ok_or_else(|| ::treesitter_types::ParseError::missing_field(
                        "default_value",
                        node,
                    ))?;
                <Expression as ::treesitter_types::FromNode>::from_node(child, src)?
            },
            r#type: {
                let child = node
                    .child_by_field_name("type")
                    .ok_or_else(|| ::treesitter_types::ParseError::missing_field(
                        "type",
                        node,
                    ))?;
                <TypeSpecifier as ::treesitter_types::FromNode>::from_node(child, src)?
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
                            <OptionalParameterDeclarationChildren as ::treesitter_types::FromNode>::from_node(
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
impl ::treesitter_types::Spanned for OptionalParameterDeclaration<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        self.span
    }
}
#[derive(Debug, Clone)]
pub struct OptionalTypeParameterDeclaration<'tree> {
    pub span: ::treesitter_types::Span,
    pub default_type: TypeSpecifier<'tree>,
    pub name: ::core::option::Option<TypeIdentifier<'tree>>,
}
impl<'tree> ::treesitter_types::FromNode<'tree> for OptionalTypeParameterDeclaration<'tree> {
    #[allow(clippy::match_single_binding, clippy::suspicious_else_formatting)]
    fn from_node(
        node: ::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        debug_assert_eq!(node.kind(), "optional_type_parameter_declaration");
        Ok(Self {
            span: ::treesitter_types::Span::from(node),
            default_type: {
                let child = node.child_by_field_name("default_type").ok_or_else(|| {
                    ::treesitter_types::ParseError::missing_field("default_type", node)
                })?;
                <TypeSpecifier as ::treesitter_types::FromNode>::from_node(child, src)?
            },
            name: match node.child_by_field_name("name") {
                Some(child) => Some(<TypeIdentifier as ::treesitter_types::FromNode>::from_node(
                    child, src,
                )?),
                None => None,
            },
        })
    }
}
impl ::treesitter_types::Spanned for OptionalTypeParameterDeclaration<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        self.span
    }
}
#[derive(Debug, Clone)]
pub struct ParameterDeclaration<'tree> {
    pub span: ::treesitter_types::Span,
    pub declarator: ::core::option::Option<ParameterDeclarationDeclarator<'tree>>,
    pub r#type: TypeSpecifier<'tree>,
    pub children: ::std::vec::Vec<ParameterDeclarationChildren<'tree>>,
}
impl<'tree> ::treesitter_types::FromNode<'tree> for ParameterDeclaration<'tree> {
    #[allow(clippy::match_single_binding, clippy::suspicious_else_formatting)]
    fn from_node(
        node: ::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        debug_assert_eq!(node.kind(), "parameter_declaration");
        Ok(Self {
            span: ::treesitter_types::Span::from(node),
            declarator: match node.child_by_field_name("declarator") {
                Some(child) => Some(
                    <ParameterDeclarationDeclarator as ::treesitter_types::FromNode>::from_node(
                        child, src,
                    )?,
                ),
                None => None,
            },
            r#type: {
                let child = node
                    .child_by_field_name("type")
                    .ok_or_else(|| ::treesitter_types::ParseError::missing_field("type", node))?;
                <TypeSpecifier as ::treesitter_types::FromNode>::from_node(child, src)?
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
                        <ParameterDeclarationChildren as ::treesitter_types::FromNode>::from_node(
                            child, src,
                        )?,
                    );
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
#[derive(Debug, Clone)]
pub struct ParameterList<'tree> {
    pub span: ::treesitter_types::Span,
    pub children: ::std::vec::Vec<ParameterListChildren<'tree>>,
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
                        <ParameterListChildren as ::treesitter_types::FromNode>::from_node(
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
pub struct ParameterPackExpansion<'tree> {
    pub span: ::treesitter_types::Span,
    pub pattern: ParameterPackExpansionPattern<'tree>,
}
impl<'tree> ::treesitter_types::FromNode<'tree> for ParameterPackExpansion<'tree> {
    #[allow(clippy::match_single_binding, clippy::suspicious_else_formatting)]
    fn from_node(
        node: ::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        debug_assert_eq!(node.kind(), "parameter_pack_expansion");
        Ok(Self {
            span: ::treesitter_types::Span::from(node),
            pattern: {
                let child = node.child_by_field_name("pattern").ok_or_else(|| {
                    ::treesitter_types::ParseError::missing_field("pattern", node)
                })?;
                <ParameterPackExpansionPattern as ::treesitter_types::FromNode>::from_node(
                    child, src,
                )?
            },
        })
    }
}
impl ::treesitter_types::Spanned for ParameterPackExpansion<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        self.span
    }
}
#[derive(Debug, Clone)]
pub struct ParenthesizedDeclarator<'tree> {
    pub span: ::treesitter_types::Span,
    pub children: ::std::vec::Vec<ParenthesizedDeclaratorChildren<'tree>>,
}
impl<'tree> ::treesitter_types::FromNode<'tree> for ParenthesizedDeclarator<'tree> {
    #[allow(clippy::match_single_binding, clippy::suspicious_else_formatting)]
    fn from_node(
        node: ::tree_sitter::Node<'tree>,
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
                            <ParenthesizedDeclaratorChildren as ::treesitter_types::FromNode>::from_node(
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
impl ::treesitter_types::Spanned for ParenthesizedDeclarator<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        self.span
    }
}
#[derive(Debug, Clone)]
pub struct ParenthesizedExpression<'tree> {
    pub span: ::treesitter_types::Span,
    pub children: ParenthesizedExpressionChildren<'tree>,
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
                <ParenthesizedExpressionChildren as ::treesitter_types::FromNode>::from_node(
                    child, src,
                )?
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
pub struct PlaceholderTypeSpecifier<'tree> {
    pub span: ::treesitter_types::Span,
    pub constraint: ::core::option::Option<PlaceholderTypeSpecifierConstraint<'tree>>,
    pub children: PlaceholderTypeSpecifierChildren<'tree>,
}
impl<'tree> ::treesitter_types::FromNode<'tree> for PlaceholderTypeSpecifier<'tree> {
    #[allow(clippy::match_single_binding, clippy::suspicious_else_formatting)]
    fn from_node(
        node: ::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        debug_assert_eq!(node.kind(), "placeholder_type_specifier");
        Ok(Self {
            span: ::treesitter_types::Span::from(node),
            constraint: match node.child_by_field_name("constraint") {
                Some(child) => {
                    Some(
                        <PlaceholderTypeSpecifierConstraint as ::treesitter_types::FromNode>::from_node(
                            child,
                            src,
                        )?,
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
                    fallback_child
                        .ok_or_else(|| ::treesitter_types::ParseError::missing_field(
                            "children",
                            node,
                        ))?
                };
                <PlaceholderTypeSpecifierChildren as ::treesitter_types::FromNode>::from_node(
                    child,
                    src,
                )?
            },
        })
    }
}
impl ::treesitter_types::Spanned for PlaceholderTypeSpecifier<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        self.span
    }
}
#[derive(Debug, Clone)]
pub struct PointerDeclarator<'tree> {
    pub span: ::treesitter_types::Span,
    pub declarator: PointerDeclaratorDeclarator<'tree>,
    pub children: ::std::vec::Vec<PointerDeclaratorChildren<'tree>>,
}
impl<'tree> ::treesitter_types::FromNode<'tree> for PointerDeclarator<'tree> {
    #[allow(clippy::match_single_binding, clippy::suspicious_else_formatting)]
    fn from_node(
        node: ::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        debug_assert_eq!(node.kind(), "pointer_declarator");
        Ok(Self {
            span: ::treesitter_types::Span::from(node),
            declarator: {
                let child = node.child_by_field_name("declarator").ok_or_else(|| {
                    ::treesitter_types::ParseError::missing_field("declarator", node)
                })?;
                <PointerDeclaratorDeclarator as ::treesitter_types::FromNode>::from_node(
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
                        <PointerDeclaratorChildren as ::treesitter_types::FromNode>::from_node(
                            child, src,
                        )?,
                    );
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
#[derive(Debug, Clone)]
pub struct PointerExpression<'tree> {
    pub span: ::treesitter_types::Span,
    pub argument: Expression<'tree>,
    pub operator: PointerExpressionOperator,
}
impl<'tree> ::treesitter_types::FromNode<'tree> for PointerExpression<'tree> {
    #[allow(clippy::match_single_binding, clippy::suspicious_else_formatting)]
    fn from_node(
        node: ::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        debug_assert_eq!(node.kind(), "pointer_expression");
        Ok(Self {
            span: ::treesitter_types::Span::from(node),
            argument: {
                let child = node.child_by_field_name("argument").ok_or_else(|| {
                    ::treesitter_types::ParseError::missing_field("argument", node)
                })?;
                <Expression as ::treesitter_types::FromNode>::from_node(child, src)?
            },
            operator: {
                let child = node.child_by_field_name("operator").ok_or_else(|| {
                    ::treesitter_types::ParseError::missing_field("operator", node)
                })?;
                <PointerExpressionOperator as ::treesitter_types::FromNode>::from_node(child, src)?
            },
        })
    }
}
impl ::treesitter_types::Spanned for PointerExpression<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        self.span
    }
}
#[derive(Debug, Clone)]
pub struct PointerTypeDeclarator<'tree> {
    pub span: ::treesitter_types::Span,
    pub declarator: TypeDeclarator<'tree>,
    pub children: ::std::vec::Vec<PointerTypeDeclaratorChildren<'tree>>,
}
impl<'tree> ::treesitter_types::FromNode<'tree> for PointerTypeDeclarator<'tree> {
    #[allow(clippy::match_single_binding, clippy::suspicious_else_formatting)]
    fn from_node(
        node: ::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        debug_assert_eq!(node.kind(), "pointer_type_declarator");
        Ok(Self {
            span: ::treesitter_types::Span::from(node),
            declarator: {
                let child = node.child_by_field_name("declarator").ok_or_else(|| {
                    ::treesitter_types::ParseError::missing_field("declarator", node)
                })?;
                <TypeDeclarator as ::treesitter_types::FromNode>::from_node(child, src)?
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
                        <PointerTypeDeclaratorChildren as ::treesitter_types::FromNode>::from_node(
                            child, src,
                        )?,
                    );
                }
                items
            },
        })
    }
}
impl ::treesitter_types::Spanned for PointerTypeDeclarator<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        self.span
    }
}
#[derive(Debug, Clone)]
pub struct PreprocCall<'tree> {
    pub span: ::treesitter_types::Span,
    pub argument: ::core::option::Option<PreprocArg<'tree>>,
    pub directive: PreprocDirective<'tree>,
}
impl<'tree> ::treesitter_types::FromNode<'tree> for PreprocCall<'tree> {
    #[allow(clippy::match_single_binding, clippy::suspicious_else_formatting)]
    fn from_node(
        node: ::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        debug_assert_eq!(node.kind(), "preproc_call");
        Ok(Self {
            span: ::treesitter_types::Span::from(node),
            argument: match node.child_by_field_name("argument") {
                Some(child) => Some(<PreprocArg as ::treesitter_types::FromNode>::from_node(
                    child, src,
                )?),
                None => None,
            },
            directive: {
                let child = node.child_by_field_name("directive").ok_or_else(|| {
                    ::treesitter_types::ParseError::missing_field("directive", node)
                })?;
                <PreprocDirective as ::treesitter_types::FromNode>::from_node(child, src)?
            },
        })
    }
}
impl ::treesitter_types::Spanned for PreprocCall<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        self.span
    }
}
#[derive(Debug, Clone)]
pub struct PreprocDef<'tree> {
    pub span: ::treesitter_types::Span,
    pub name: Identifier<'tree>,
    pub value: ::core::option::Option<PreprocArg<'tree>>,
}
impl<'tree> ::treesitter_types::FromNode<'tree> for PreprocDef<'tree> {
    #[allow(clippy::match_single_binding, clippy::suspicious_else_formatting)]
    fn from_node(
        node: ::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        debug_assert_eq!(node.kind(), "preproc_def");
        Ok(Self {
            span: ::treesitter_types::Span::from(node),
            name: {
                let child = node
                    .child_by_field_name("name")
                    .ok_or_else(|| ::treesitter_types::ParseError::missing_field("name", node))?;
                <Identifier as ::treesitter_types::FromNode>::from_node(child, src)?
            },
            value: match node.child_by_field_name("value") {
                Some(child) => Some(<PreprocArg as ::treesitter_types::FromNode>::from_node(
                    child, src,
                )?),
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
#[derive(Debug, Clone)]
pub struct PreprocDefined<'tree> {
    pub span: ::treesitter_types::Span,
    pub children: Identifier<'tree>,
}
impl<'tree> ::treesitter_types::FromNode<'tree> for PreprocDefined<'tree> {
    #[allow(clippy::match_single_binding, clippy::suspicious_else_formatting)]
    fn from_node(
        node: ::tree_sitter::Node<'tree>,
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
                <Identifier as ::treesitter_types::FromNode>::from_node(child, src)?
            },
        })
    }
}
impl ::treesitter_types::Spanned for PreprocDefined<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        self.span
    }
}
#[derive(Debug, Clone)]
pub struct PreprocElif<'tree> {
    pub span: ::treesitter_types::Span,
    pub alternative: ::core::option::Option<PreprocElifAlternative<'tree>>,
    pub condition: PreprocElifCondition<'tree>,
    pub children: ::std::vec::Vec<PreprocElifChildren<'tree>>,
}
impl<'tree> ::treesitter_types::FromNode<'tree> for PreprocElif<'tree> {
    #[allow(clippy::match_single_binding, clippy::suspicious_else_formatting)]
    fn from_node(
        node: ::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        debug_assert_eq!(node.kind(), "preproc_elif");
        Ok(Self {
            span: ::treesitter_types::Span::from(node),
            alternative: match node.child_by_field_name("alternative") {
                Some(child) => Some(
                    <PreprocElifAlternative as ::treesitter_types::FromNode>::from_node(
                        child, src,
                    )?,
                ),
                None => None,
            },
            condition: {
                let child = node.child_by_field_name("condition").ok_or_else(|| {
                    ::treesitter_types::ParseError::missing_field("condition", node)
                })?;
                <PreprocElifCondition as ::treesitter_types::FromNode>::from_node(child, src)?
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
                        <PreprocElifChildren as ::treesitter_types::FromNode>::from_node(
                            child, src,
                        )?,
                    );
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
#[derive(Debug, Clone)]
pub struct PreprocElifdef<'tree> {
    pub span: ::treesitter_types::Span,
    pub alternative: ::core::option::Option<PreprocElifdefAlternative<'tree>>,
    pub name: Identifier<'tree>,
    pub children: ::std::vec::Vec<PreprocElifdefChildren<'tree>>,
}
impl<'tree> ::treesitter_types::FromNode<'tree> for PreprocElifdef<'tree> {
    #[allow(clippy::match_single_binding, clippy::suspicious_else_formatting)]
    fn from_node(
        node: ::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        debug_assert_eq!(node.kind(), "preproc_elifdef");
        Ok(Self {
            span: ::treesitter_types::Span::from(node),
            alternative: match node.child_by_field_name("alternative") {
                Some(child) => Some(
                    <PreprocElifdefAlternative as ::treesitter_types::FromNode>::from_node(
                        child, src,
                    )?,
                ),
                None => None,
            },
            name: {
                let child = node
                    .child_by_field_name("name")
                    .ok_or_else(|| ::treesitter_types::ParseError::missing_field("name", node))?;
                <Identifier as ::treesitter_types::FromNode>::from_node(child, src)?
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
                        <PreprocElifdefChildren as ::treesitter_types::FromNode>::from_node(
                            child, src,
                        )?,
                    );
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
#[derive(Debug, Clone)]
pub struct PreprocElse<'tree> {
    pub span: ::treesitter_types::Span,
    pub children: ::std::vec::Vec<PreprocElseChildren<'tree>>,
}
impl<'tree> ::treesitter_types::FromNode<'tree> for PreprocElse<'tree> {
    #[allow(clippy::match_single_binding, clippy::suspicious_else_formatting)]
    fn from_node(
        node: ::tree_sitter::Node<'tree>,
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
                    items.push(
                        <PreprocElseChildren as ::treesitter_types::FromNode>::from_node(
                            child, src,
                        )?,
                    );
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
#[derive(Debug, Clone)]
pub struct PreprocFunctionDef<'tree> {
    pub span: ::treesitter_types::Span,
    pub name: Identifier<'tree>,
    pub parameters: PreprocParams<'tree>,
    pub value: ::core::option::Option<PreprocArg<'tree>>,
}
impl<'tree> ::treesitter_types::FromNode<'tree> for PreprocFunctionDef<'tree> {
    #[allow(clippy::match_single_binding, clippy::suspicious_else_formatting)]
    fn from_node(
        node: ::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        debug_assert_eq!(node.kind(), "preproc_function_def");
        Ok(Self {
            span: ::treesitter_types::Span::from(node),
            name: {
                let child = node
                    .child_by_field_name("name")
                    .ok_or_else(|| ::treesitter_types::ParseError::missing_field("name", node))?;
                <Identifier as ::treesitter_types::FromNode>::from_node(child, src)?
            },
            parameters: {
                let child = node.child_by_field_name("parameters").ok_or_else(|| {
                    ::treesitter_types::ParseError::missing_field("parameters", node)
                })?;
                <PreprocParams as ::treesitter_types::FromNode>::from_node(child, src)?
            },
            value: match node.child_by_field_name("value") {
                Some(child) => Some(<PreprocArg as ::treesitter_types::FromNode>::from_node(
                    child, src,
                )?),
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
#[derive(Debug, Clone)]
pub struct PreprocIf<'tree> {
    pub span: ::treesitter_types::Span,
    pub alternative: ::core::option::Option<PreprocIfAlternative<'tree>>,
    pub condition: PreprocIfCondition<'tree>,
    pub children: ::std::vec::Vec<PreprocIfChildren<'tree>>,
}
impl<'tree> ::treesitter_types::FromNode<'tree> for PreprocIf<'tree> {
    #[allow(clippy::match_single_binding, clippy::suspicious_else_formatting)]
    fn from_node(
        node: ::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        debug_assert_eq!(node.kind(), "preproc_if");
        Ok(Self {
            span: ::treesitter_types::Span::from(node),
            alternative: match node.child_by_field_name("alternative") {
                Some(child) => Some(
                    <PreprocIfAlternative as ::treesitter_types::FromNode>::from_node(child, src)?,
                ),
                None => None,
            },
            condition: {
                let child = node.child_by_field_name("condition").ok_or_else(|| {
                    ::treesitter_types::ParseError::missing_field("condition", node)
                })?;
                <PreprocIfCondition as ::treesitter_types::FromNode>::from_node(child, src)?
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
                        <PreprocIfChildren as ::treesitter_types::FromNode>::from_node(child, src)?,
                    );
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
#[derive(Debug, Clone)]
pub struct PreprocIfdef<'tree> {
    pub span: ::treesitter_types::Span,
    pub alternative: ::core::option::Option<PreprocIfdefAlternative<'tree>>,
    pub name: Identifier<'tree>,
    pub children: ::std::vec::Vec<PreprocIfdefChildren<'tree>>,
}
impl<'tree> ::treesitter_types::FromNode<'tree> for PreprocIfdef<'tree> {
    #[allow(clippy::match_single_binding, clippy::suspicious_else_formatting)]
    fn from_node(
        node: ::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        debug_assert_eq!(node.kind(), "preproc_ifdef");
        Ok(Self {
            span: ::treesitter_types::Span::from(node),
            alternative: match node.child_by_field_name("alternative") {
                Some(child) => Some(
                    <PreprocIfdefAlternative as ::treesitter_types::FromNode>::from_node(
                        child, src,
                    )?,
                ),
                None => None,
            },
            name: {
                let child = node
                    .child_by_field_name("name")
                    .ok_or_else(|| ::treesitter_types::ParseError::missing_field("name", node))?;
                <Identifier as ::treesitter_types::FromNode>::from_node(child, src)?
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
                        <PreprocIfdefChildren as ::treesitter_types::FromNode>::from_node(
                            child, src,
                        )?,
                    );
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
#[derive(Debug, Clone)]
pub struct PreprocInclude<'tree> {
    pub span: ::treesitter_types::Span,
    pub path: PreprocIncludePath<'tree>,
}
impl<'tree> ::treesitter_types::FromNode<'tree> for PreprocInclude<'tree> {
    #[allow(clippy::match_single_binding, clippy::suspicious_else_formatting)]
    fn from_node(
        node: ::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        debug_assert_eq!(node.kind(), "preproc_include");
        Ok(Self {
            span: ::treesitter_types::Span::from(node),
            path: {
                let child = node
                    .child_by_field_name("path")
                    .ok_or_else(|| ::treesitter_types::ParseError::missing_field("path", node))?;
                <PreprocIncludePath as ::treesitter_types::FromNode>::from_node(child, src)?
            },
        })
    }
}
impl ::treesitter_types::Spanned for PreprocInclude<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        self.span
    }
}
#[derive(Debug, Clone)]
pub struct PreprocParams<'tree> {
    pub span: ::treesitter_types::Span,
    pub children: ::std::vec::Vec<Identifier<'tree>>,
}
impl<'tree> ::treesitter_types::FromNode<'tree> for PreprocParams<'tree> {
    #[allow(clippy::match_single_binding, clippy::suspicious_else_formatting)]
    fn from_node(
        node: ::tree_sitter::Node<'tree>,
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
                    items.push(<Identifier as ::treesitter_types::FromNode>::from_node(
                        child, src,
                    )?);
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
#[derive(Debug, Clone)]
pub struct PrivateModuleFragmentDeclaration<'tree> {
    pub span: ::treesitter_types::Span,
    text: &'tree str,
}
impl<'tree> ::treesitter_types::FromNode<'tree> for PrivateModuleFragmentDeclaration<'tree> {
    fn from_node(
        node: ::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        debug_assert_eq!(node.kind(), "private_module_fragment_declaration");
        Ok(Self {
            span: ::treesitter_types::Span::from(node),
            text: node.utf8_text(src)?,
        })
    }
}
impl<'tree> ::treesitter_types::LeafNode<'tree> for PrivateModuleFragmentDeclaration<'tree> {
    fn text(&self) -> &'tree str {
        self.text
    }
}
impl ::treesitter_types::Spanned for PrivateModuleFragmentDeclaration<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        self.span
    }
}
#[derive(Debug, Clone)]
pub struct PureVirtualClause<'tree> {
    pub span: ::treesitter_types::Span,
    text: &'tree str,
}
impl<'tree> ::treesitter_types::FromNode<'tree> for PureVirtualClause<'tree> {
    fn from_node(
        node: ::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        debug_assert_eq!(node.kind(), "pure_virtual_clause");
        Ok(Self {
            span: ::treesitter_types::Span::from(node),
            text: node.utf8_text(src)?,
        })
    }
}
impl<'tree> ::treesitter_types::LeafNode<'tree> for PureVirtualClause<'tree> {
    fn text(&self) -> &'tree str {
        self.text
    }
}
impl ::treesitter_types::Spanned for PureVirtualClause<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        self.span
    }
}
#[derive(Debug, Clone)]
pub struct QualifiedIdentifier<'tree> {
    pub span: ::treesitter_types::Span,
    pub name: ::std::vec::Vec<QualifiedIdentifierName<'tree>>,
    pub scope: ::core::option::Option<QualifiedIdentifierScope<'tree>>,
}
impl<'tree> ::treesitter_types::FromNode<'tree> for QualifiedIdentifier<'tree> {
    #[allow(clippy::match_single_binding, clippy::suspicious_else_formatting)]
    fn from_node(
        node: ::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        debug_assert_eq!(node.kind(), "qualified_identifier");
        Ok(Self {
            span: ::treesitter_types::Span::from(node),
            name: {
                let mut cursor = node.walk();
                let mut items = ::std::vec::Vec::new();
                for child in node.children_by_field_name("name", &mut cursor) {
                    items.push(
                        <QualifiedIdentifierName as ::treesitter_types::FromNode>::from_node(
                            child, src,
                        )?,
                    );
                }
                items
            },
            scope: match node.child_by_field_name("scope") {
                Some(child) => Some(
                    <QualifiedIdentifierScope as ::treesitter_types::FromNode>::from_node(
                        child, src,
                    )?,
                ),
                None => None,
            },
        })
    }
}
impl ::treesitter_types::Spanned for QualifiedIdentifier<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        self.span
    }
}
#[derive(Debug, Clone)]
pub struct RawStringLiteral<'tree> {
    pub span: ::treesitter_types::Span,
    pub delimiter: ::core::option::Option<RawStringDelimiter<'tree>>,
    pub children: ::std::vec::Vec<RawStringLiteralChildren<'tree>>,
}
impl<'tree> ::treesitter_types::FromNode<'tree> for RawStringLiteral<'tree> {
    #[allow(clippy::match_single_binding, clippy::suspicious_else_formatting)]
    fn from_node(
        node: ::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        debug_assert_eq!(node.kind(), "raw_string_literal");
        Ok(Self {
            span: ::treesitter_types::Span::from(node),
            delimiter: match node.child_by_field_name("delimiter") {
                Some(child) => Some(
                    <RawStringDelimiter as ::treesitter_types::FromNode>::from_node(child, src)?,
                ),
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
                        <RawStringLiteralChildren as ::treesitter_types::FromNode>::from_node(
                            child, src,
                        )?,
                    );
                }
                items
            },
        })
    }
}
impl ::treesitter_types::Spanned for RawStringLiteral<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        self.span
    }
}
#[derive(Debug, Clone)]
pub struct RefQualifier<'tree> {
    pub span: ::treesitter_types::Span,
    text: &'tree str,
}
impl<'tree> ::treesitter_types::FromNode<'tree> for RefQualifier<'tree> {
    fn from_node(
        node: ::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        debug_assert_eq!(node.kind(), "ref_qualifier");
        Ok(Self {
            span: ::treesitter_types::Span::from(node),
            text: node.utf8_text(src)?,
        })
    }
}
impl<'tree> ::treesitter_types::LeafNode<'tree> for RefQualifier<'tree> {
    fn text(&self) -> &'tree str {
        self.text
    }
}
impl ::treesitter_types::Spanned for RefQualifier<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        self.span
    }
}
#[derive(Debug, Clone)]
pub struct ReferenceDeclarator<'tree> {
    pub span: ::treesitter_types::Span,
    pub children: ReferenceDeclaratorChildren<'tree>,
}
impl<'tree> ::treesitter_types::FromNode<'tree> for ReferenceDeclarator<'tree> {
    #[allow(clippy::match_single_binding, clippy::suspicious_else_formatting)]
    fn from_node(
        node: ::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        debug_assert_eq!(node.kind(), "reference_declarator");
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
                <ReferenceDeclaratorChildren as ::treesitter_types::FromNode>::from_node(
                    child, src,
                )?
            },
        })
    }
}
impl ::treesitter_types::Spanned for ReferenceDeclarator<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        self.span
    }
}
#[derive(Debug, Clone)]
pub struct ReflectExpression<'tree> {
    pub span: ::treesitter_types::Span,
    pub children: ::core::option::Option<ReflectExpressionChildren<'tree>>,
}
impl<'tree> ::treesitter_types::FromNode<'tree> for ReflectExpression<'tree> {
    #[allow(clippy::match_single_binding, clippy::suspicious_else_formatting)]
    fn from_node(
        node: ::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        debug_assert_eq!(node.kind(), "reflect_expression");
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
                        <ReflectExpressionChildren as ::treesitter_types::FromNode>::from_node(
                            child, src,
                        )?,
                    ),
                    None => None,
                }
            },
        })
    }
}
impl ::treesitter_types::Spanned for ReflectExpression<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        self.span
    }
}
#[derive(Debug, Clone)]
pub struct RequirementSeq<'tree> {
    pub span: ::treesitter_types::Span,
    pub children: ::std::vec::Vec<RequirementSeqChildren<'tree>>,
}
impl<'tree> ::treesitter_types::FromNode<'tree> for RequirementSeq<'tree> {
    #[allow(clippy::match_single_binding, clippy::suspicious_else_formatting)]
    fn from_node(
        node: ::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        debug_assert_eq!(node.kind(), "requirement_seq");
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
                        <RequirementSeqChildren as ::treesitter_types::FromNode>::from_node(
                            child, src,
                        )?,
                    );
                }
                items
            },
        })
    }
}
impl ::treesitter_types::Spanned for RequirementSeq<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        self.span
    }
}
#[derive(Debug, Clone)]
pub struct RequiresClause<'tree> {
    pub span: ::treesitter_types::Span,
    pub constraint: ::std::vec::Vec<RequiresClauseConstraint<'tree>>,
}
impl<'tree> ::treesitter_types::FromNode<'tree> for RequiresClause<'tree> {
    #[allow(clippy::match_single_binding, clippy::suspicious_else_formatting)]
    fn from_node(
        node: ::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        debug_assert_eq!(node.kind(), "requires_clause");
        Ok(Self {
            span: ::treesitter_types::Span::from(node),
            constraint: {
                let mut cursor = node.walk();
                let mut items = ::std::vec::Vec::new();
                for child in node.children_by_field_name("constraint", &mut cursor) {
                    items.push(
                        <RequiresClauseConstraint as ::treesitter_types::FromNode>::from_node(
                            child, src,
                        )?,
                    );
                }
                items
            },
        })
    }
}
impl ::treesitter_types::Spanned for RequiresClause<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        self.span
    }
}
#[derive(Debug, Clone)]
pub struct RequiresExpression<'tree> {
    pub span: ::treesitter_types::Span,
    pub parameters: ::core::option::Option<ParameterList<'tree>>,
    pub requirements: RequirementSeq<'tree>,
}
impl<'tree> ::treesitter_types::FromNode<'tree> for RequiresExpression<'tree> {
    #[allow(clippy::match_single_binding, clippy::suspicious_else_formatting)]
    fn from_node(
        node: ::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        debug_assert_eq!(node.kind(), "requires_expression");
        Ok(Self {
            span: ::treesitter_types::Span::from(node),
            parameters: match node.child_by_field_name("parameters") {
                Some(child) => Some(<ParameterList as ::treesitter_types::FromNode>::from_node(
                    child, src,
                )?),
                None => None,
            },
            requirements: {
                let child = node.child_by_field_name("requirements").ok_or_else(|| {
                    ::treesitter_types::ParseError::missing_field("requirements", node)
                })?;
                <RequirementSeq as ::treesitter_types::FromNode>::from_node(child, src)?
            },
        })
    }
}
impl ::treesitter_types::Spanned for RequiresExpression<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        self.span
    }
}
#[derive(Debug, Clone)]
pub struct ReturnStatement<'tree> {
    pub span: ::treesitter_types::Span,
    pub children: ::core::option::Option<ReturnStatementChildren<'tree>>,
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
                        <ReturnStatementChildren as ::treesitter_types::FromNode>::from_node(
                            child, src,
                        )?,
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
#[derive(Debug, Clone)]
pub struct SehExceptClause<'tree> {
    pub span: ::treesitter_types::Span,
    pub body: CompoundStatement<'tree>,
    pub filter: ParenthesizedExpression<'tree>,
}
impl<'tree> ::treesitter_types::FromNode<'tree> for SehExceptClause<'tree> {
    #[allow(clippy::match_single_binding, clippy::suspicious_else_formatting)]
    fn from_node(
        node: ::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        debug_assert_eq!(node.kind(), "seh_except_clause");
        Ok(Self {
            span: ::treesitter_types::Span::from(node),
            body: {
                let child = node
                    .child_by_field_name("body")
                    .ok_or_else(|| ::treesitter_types::ParseError::missing_field("body", node))?;
                <CompoundStatement as ::treesitter_types::FromNode>::from_node(child, src)?
            },
            filter: {
                let child = node
                    .child_by_field_name("filter")
                    .ok_or_else(|| ::treesitter_types::ParseError::missing_field("filter", node))?;
                <ParenthesizedExpression as ::treesitter_types::FromNode>::from_node(child, src)?
            },
        })
    }
}
impl ::treesitter_types::Spanned for SehExceptClause<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        self.span
    }
}
#[derive(Debug, Clone)]
pub struct SehFinallyClause<'tree> {
    pub span: ::treesitter_types::Span,
    pub body: CompoundStatement<'tree>,
}
impl<'tree> ::treesitter_types::FromNode<'tree> for SehFinallyClause<'tree> {
    #[allow(clippy::match_single_binding, clippy::suspicious_else_formatting)]
    fn from_node(
        node: ::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        debug_assert_eq!(node.kind(), "seh_finally_clause");
        Ok(Self {
            span: ::treesitter_types::Span::from(node),
            body: {
                let child = node
                    .child_by_field_name("body")
                    .ok_or_else(|| ::treesitter_types::ParseError::missing_field("body", node))?;
                <CompoundStatement as ::treesitter_types::FromNode>::from_node(child, src)?
            },
        })
    }
}
impl ::treesitter_types::Spanned for SehFinallyClause<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        self.span
    }
}
#[derive(Debug, Clone)]
pub struct SehLeaveStatement<'tree> {
    pub span: ::treesitter_types::Span,
    text: &'tree str,
}
impl<'tree> ::treesitter_types::FromNode<'tree> for SehLeaveStatement<'tree> {
    fn from_node(
        node: ::tree_sitter::Node<'tree>,
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
#[derive(Debug, Clone)]
pub struct SehTryStatement<'tree> {
    pub span: ::treesitter_types::Span,
    pub body: CompoundStatement<'tree>,
    pub children: SehTryStatementChildren<'tree>,
}
impl<'tree> ::treesitter_types::FromNode<'tree> for SehTryStatement<'tree> {
    #[allow(clippy::match_single_binding, clippy::suspicious_else_formatting)]
    fn from_node(
        node: ::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        debug_assert_eq!(node.kind(), "seh_try_statement");
        Ok(Self {
            span: ::treesitter_types::Span::from(node),
            body: {
                let child = node
                    .child_by_field_name("body")
                    .ok_or_else(|| ::treesitter_types::ParseError::missing_field("body", node))?;
                <CompoundStatement as ::treesitter_types::FromNode>::from_node(child, src)?
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
                <SehTryStatementChildren as ::treesitter_types::FromNode>::from_node(child, src)?
            },
        })
    }
}
impl ::treesitter_types::Spanned for SehTryStatement<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        self.span
    }
}
#[derive(Debug, Clone)]
pub struct SimpleRequirement<'tree> {
    pub span: ::treesitter_types::Span,
    pub children: ::core::option::Option<SimpleRequirementChildren<'tree>>,
}
impl<'tree> ::treesitter_types::FromNode<'tree> for SimpleRequirement<'tree> {
    #[allow(clippy::match_single_binding, clippy::suspicious_else_formatting)]
    fn from_node(
        node: ::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        debug_assert_eq!(node.kind(), "simple_requirement");
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
                        <SimpleRequirementChildren as ::treesitter_types::FromNode>::from_node(
                            child, src,
                        )?,
                    ),
                    None => None,
                }
            },
        })
    }
}
impl ::treesitter_types::Spanned for SimpleRequirement<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        self.span
    }
}
#[derive(Debug, Clone)]
pub struct SizedTypeSpecifier<'tree> {
    pub span: ::treesitter_types::Span,
    pub r#type: ::core::option::Option<SizedTypeSpecifierType<'tree>>,
    pub children: ::std::vec::Vec<TypeQualifier<'tree>>,
}
impl<'tree> ::treesitter_types::FromNode<'tree> for SizedTypeSpecifier<'tree> {
    #[allow(clippy::match_single_binding, clippy::suspicious_else_formatting)]
    fn from_node(
        node: ::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        debug_assert_eq!(node.kind(), "sized_type_specifier");
        Ok(Self {
            span: ::treesitter_types::Span::from(node),
            r#type: match node.child_by_field_name("type") {
                Some(child) => Some(
                    <SizedTypeSpecifierType as ::treesitter_types::FromNode>::from_node(
                        child, src,
                    )?,
                ),
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
                    items.push(<TypeQualifier as ::treesitter_types::FromNode>::from_node(
                        child, src,
                    )?);
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
#[derive(Debug, Clone)]
pub struct SizeofExpression<'tree> {
    pub span: ::treesitter_types::Span,
    pub r#type: ::core::option::Option<TypeDescriptor<'tree>>,
    pub value: ::core::option::Option<Expression<'tree>>,
}
impl<'tree> ::treesitter_types::FromNode<'tree> for SizeofExpression<'tree> {
    #[allow(clippy::match_single_binding, clippy::suspicious_else_formatting)]
    fn from_node(
        node: ::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        debug_assert_eq!(node.kind(), "sizeof_expression");
        Ok(Self {
            span: ::treesitter_types::Span::from(node),
            r#type: match node.child_by_field_name("type") {
                Some(child) => Some(<TypeDescriptor as ::treesitter_types::FromNode>::from_node(
                    child, src,
                )?),
                None => None,
            },
            value: match node.child_by_field_name("value") {
                Some(child) => Some(<Expression as ::treesitter_types::FromNode>::from_node(
                    child, src,
                )?),
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
#[derive(Debug, Clone)]
pub struct SpliceExpression<'tree> {
    pub span: ::treesitter_types::Span,
    pub children: ::std::vec::Vec<SpliceExpressionChildren<'tree>>,
}
impl<'tree> ::treesitter_types::FromNode<'tree> for SpliceExpression<'tree> {
    #[allow(clippy::match_single_binding, clippy::suspicious_else_formatting)]
    fn from_node(
        node: ::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        debug_assert_eq!(node.kind(), "splice_expression");
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
                        <SpliceExpressionChildren as ::treesitter_types::FromNode>::from_node(
                            child, src,
                        )?,
                    );
                }
                items
            },
        })
    }
}
impl ::treesitter_types::Spanned for SpliceExpression<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        self.span
    }
}
#[derive(Debug, Clone)]
pub struct SpliceSpecifier<'tree> {
    pub span: ::treesitter_types::Span,
    pub children: Expression<'tree>,
}
impl<'tree> ::treesitter_types::FromNode<'tree> for SpliceSpecifier<'tree> {
    #[allow(clippy::match_single_binding, clippy::suspicious_else_formatting)]
    fn from_node(
        node: ::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        debug_assert_eq!(node.kind(), "splice_specifier");
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
                <Expression as ::treesitter_types::FromNode>::from_node(child, src)?
            },
        })
    }
}
impl ::treesitter_types::Spanned for SpliceSpecifier<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        self.span
    }
}
#[derive(Debug, Clone)]
pub struct SpliceTypeSpecifier<'tree> {
    pub span: ::treesitter_types::Span,
    pub children: ::std::vec::Vec<SpliceTypeSpecifierChildren<'tree>>,
}
impl<'tree> ::treesitter_types::FromNode<'tree> for SpliceTypeSpecifier<'tree> {
    #[allow(clippy::match_single_binding, clippy::suspicious_else_formatting)]
    fn from_node(
        node: ::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        debug_assert_eq!(node.kind(), "splice_type_specifier");
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
                        <SpliceTypeSpecifierChildren as ::treesitter_types::FromNode>::from_node(
                            child, src,
                        )?,
                    );
                }
                items
            },
        })
    }
}
impl ::treesitter_types::Spanned for SpliceTypeSpecifier<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        self.span
    }
}
#[derive(Debug, Clone)]
pub struct StaticAssertDeclaration<'tree> {
    pub span: ::treesitter_types::Span,
    pub condition: Expression<'tree>,
    pub message: ::core::option::Option<StaticAssertDeclarationMessage<'tree>>,
}
impl<'tree> ::treesitter_types::FromNode<'tree> for StaticAssertDeclaration<'tree> {
    #[allow(clippy::match_single_binding, clippy::suspicious_else_formatting)]
    fn from_node(
        node: ::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        debug_assert_eq!(node.kind(), "static_assert_declaration");
        Ok(Self {
            span: ::treesitter_types::Span::from(node),
            condition: {
                let child = node.child_by_field_name("condition").ok_or_else(|| {
                    ::treesitter_types::ParseError::missing_field("condition", node)
                })?;
                <Expression as ::treesitter_types::FromNode>::from_node(child, src)?
            },
            message: match node.child_by_field_name("message") {
                Some(child) => Some(
                    <StaticAssertDeclarationMessage as ::treesitter_types::FromNode>::from_node(
                        child, src,
                    )?,
                ),
                None => None,
            },
        })
    }
}
impl ::treesitter_types::Spanned for StaticAssertDeclaration<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        self.span
    }
}
#[derive(Debug, Clone)]
pub struct StorageClassSpecifier<'tree> {
    pub span: ::treesitter_types::Span,
    text: &'tree str,
}
impl<'tree> ::treesitter_types::FromNode<'tree> for StorageClassSpecifier<'tree> {
    fn from_node(
        node: ::tree_sitter::Node<'tree>,
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
#[derive(Debug, Clone)]
pub struct StringLiteral<'tree> {
    pub span: ::treesitter_types::Span,
    pub children: ::std::vec::Vec<StringLiteralChildren<'tree>>,
}
impl<'tree> ::treesitter_types::FromNode<'tree> for StringLiteral<'tree> {
    #[allow(clippy::match_single_binding, clippy::suspicious_else_formatting)]
    fn from_node(
        node: ::tree_sitter::Node<'tree>,
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
                    items.push(
                        <StringLiteralChildren as ::treesitter_types::FromNode>::from_node(
                            child, src,
                        )?,
                    );
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
#[derive(Debug, Clone)]
pub struct StructSpecifier<'tree> {
    pub span: ::treesitter_types::Span,
    pub body: ::core::option::Option<FieldDeclarationList<'tree>>,
    pub name: ::core::option::Option<StructSpecifierName<'tree>>,
    pub children: ::std::vec::Vec<StructSpecifierChildren<'tree>>,
}
impl<'tree> ::treesitter_types::FromNode<'tree> for StructSpecifier<'tree> {
    #[allow(clippy::match_single_binding, clippy::suspicious_else_formatting)]
    fn from_node(
        node: ::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        debug_assert_eq!(node.kind(), "struct_specifier");
        Ok(Self {
            span: ::treesitter_types::Span::from(node),
            body: match node.child_by_field_name("body") {
                Some(child) => Some(
                    <FieldDeclarationList as ::treesitter_types::FromNode>::from_node(child, src)?,
                ),
                None => None,
            },
            name: match node.child_by_field_name("name") {
                Some(child) => Some(
                    <StructSpecifierName as ::treesitter_types::FromNode>::from_node(child, src)?,
                ),
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
                        <StructSpecifierChildren as ::treesitter_types::FromNode>::from_node(
                            child, src,
                        )?,
                    );
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
#[derive(Debug, Clone)]
pub struct StructuredBindingDeclarator<'tree> {
    pub span: ::treesitter_types::Span,
    pub children: ::std::vec::Vec<Identifier<'tree>>,
}
impl<'tree> ::treesitter_types::FromNode<'tree> for StructuredBindingDeclarator<'tree> {
    #[allow(clippy::match_single_binding, clippy::suspicious_else_formatting)]
    fn from_node(
        node: ::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        debug_assert_eq!(node.kind(), "structured_binding_declarator");
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
                    items.push(<Identifier as ::treesitter_types::FromNode>::from_node(
                        child, src,
                    )?);
                }
                items
            },
        })
    }
}
impl ::treesitter_types::Spanned for StructuredBindingDeclarator<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        self.span
    }
}
#[derive(Debug, Clone)]
pub struct SubscriptArgumentList<'tree> {
    pub span: ::treesitter_types::Span,
    pub children: ::std::vec::Vec<SubscriptArgumentListChildren<'tree>>,
}
impl<'tree> ::treesitter_types::FromNode<'tree> for SubscriptArgumentList<'tree> {
    #[allow(clippy::match_single_binding, clippy::suspicious_else_formatting)]
    fn from_node(
        node: ::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        debug_assert_eq!(node.kind(), "subscript_argument_list");
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
                        <SubscriptArgumentListChildren as ::treesitter_types::FromNode>::from_node(
                            child, src,
                        )?,
                    );
                }
                items
            },
        })
    }
}
impl ::treesitter_types::Spanned for SubscriptArgumentList<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        self.span
    }
}
#[derive(Debug, Clone)]
pub struct SubscriptDesignator<'tree> {
    pub span: ::treesitter_types::Span,
    pub children: Expression<'tree>,
}
impl<'tree> ::treesitter_types::FromNode<'tree> for SubscriptDesignator<'tree> {
    #[allow(clippy::match_single_binding, clippy::suspicious_else_formatting)]
    fn from_node(
        node: ::tree_sitter::Node<'tree>,
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
                <Expression as ::treesitter_types::FromNode>::from_node(child, src)?
            },
        })
    }
}
impl ::treesitter_types::Spanned for SubscriptDesignator<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        self.span
    }
}
#[derive(Debug, Clone)]
pub struct SubscriptExpression<'tree> {
    pub span: ::treesitter_types::Span,
    pub argument: Expression<'tree>,
    pub indices: SubscriptArgumentList<'tree>,
}
impl<'tree> ::treesitter_types::FromNode<'tree> for SubscriptExpression<'tree> {
    #[allow(clippy::match_single_binding, clippy::suspicious_else_formatting)]
    fn from_node(
        node: ::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        debug_assert_eq!(node.kind(), "subscript_expression");
        Ok(Self {
            span: ::treesitter_types::Span::from(node),
            argument: {
                let child = node.child_by_field_name("argument").ok_or_else(|| {
                    ::treesitter_types::ParseError::missing_field("argument", node)
                })?;
                <Expression as ::treesitter_types::FromNode>::from_node(child, src)?
            },
            indices: {
                let child = node.child_by_field_name("indices").ok_or_else(|| {
                    ::treesitter_types::ParseError::missing_field("indices", node)
                })?;
                <SubscriptArgumentList as ::treesitter_types::FromNode>::from_node(child, src)?
            },
        })
    }
}
impl ::treesitter_types::Spanned for SubscriptExpression<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        self.span
    }
}
#[derive(Debug, Clone)]
pub struct SubscriptRangeDesignator<'tree> {
    pub span: ::treesitter_types::Span,
    pub end: Expression<'tree>,
    pub start: Expression<'tree>,
}
impl<'tree> ::treesitter_types::FromNode<'tree> for SubscriptRangeDesignator<'tree> {
    #[allow(clippy::match_single_binding, clippy::suspicious_else_formatting)]
    fn from_node(
        node: ::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        debug_assert_eq!(node.kind(), "subscript_range_designator");
        Ok(Self {
            span: ::treesitter_types::Span::from(node),
            end: {
                let child = node
                    .child_by_field_name("end")
                    .ok_or_else(|| ::treesitter_types::ParseError::missing_field("end", node))?;
                <Expression as ::treesitter_types::FromNode>::from_node(child, src)?
            },
            start: {
                let child = node
                    .child_by_field_name("start")
                    .ok_or_else(|| ::treesitter_types::ParseError::missing_field("start", node))?;
                <Expression as ::treesitter_types::FromNode>::from_node(child, src)?
            },
        })
    }
}
impl ::treesitter_types::Spanned for SubscriptRangeDesignator<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        self.span
    }
}
#[derive(Debug, Clone)]
pub struct SwitchStatement<'tree> {
    pub span: ::treesitter_types::Span,
    pub body: CompoundStatement<'tree>,
    pub condition: ConditionClause<'tree>,
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
            body: {
                let child = node
                    .child_by_field_name("body")
                    .ok_or_else(|| ::treesitter_types::ParseError::missing_field("body", node))?;
                <CompoundStatement as ::treesitter_types::FromNode>::from_node(child, src)?
            },
            condition: {
                let child = node.child_by_field_name("condition").ok_or_else(|| {
                    ::treesitter_types::ParseError::missing_field("condition", node)
                })?;
                <ConditionClause as ::treesitter_types::FromNode>::from_node(child, src)?
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
pub struct TemplateArgumentList<'tree> {
    pub span: ::treesitter_types::Span,
    pub children: ::std::vec::Vec<TemplateArgumentListChildren<'tree>>,
}
impl<'tree> ::treesitter_types::FromNode<'tree> for TemplateArgumentList<'tree> {
    #[allow(clippy::match_single_binding, clippy::suspicious_else_formatting)]
    fn from_node(
        node: ::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        debug_assert_eq!(node.kind(), "template_argument_list");
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
                        <TemplateArgumentListChildren as ::treesitter_types::FromNode>::from_node(
                            child, src,
                        )?,
                    );
                }
                items
            },
        })
    }
}
impl ::treesitter_types::Spanned for TemplateArgumentList<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        self.span
    }
}
#[derive(Debug, Clone)]
pub struct TemplateDeclaration<'tree> {
    pub span: ::treesitter_types::Span,
    pub parameters: TemplateParameterList<'tree>,
    pub children: ::std::vec::Vec<TemplateDeclarationChildren<'tree>>,
}
impl<'tree> ::treesitter_types::FromNode<'tree> for TemplateDeclaration<'tree> {
    #[allow(clippy::match_single_binding, clippy::suspicious_else_formatting)]
    fn from_node(
        node: ::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        debug_assert_eq!(node.kind(), "template_declaration");
        Ok(Self {
            span: ::treesitter_types::Span::from(node),
            parameters: {
                let child = node.child_by_field_name("parameters").ok_or_else(|| {
                    ::treesitter_types::ParseError::missing_field("parameters", node)
                })?;
                <TemplateParameterList as ::treesitter_types::FromNode>::from_node(child, src)?
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
                        <TemplateDeclarationChildren as ::treesitter_types::FromNode>::from_node(
                            child, src,
                        )?,
                    );
                }
                items
            },
        })
    }
}
impl ::treesitter_types::Spanned for TemplateDeclaration<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        self.span
    }
}
#[derive(Debug, Clone)]
pub struct TemplateFunction<'tree> {
    pub span: ::treesitter_types::Span,
    pub arguments: TemplateArgumentList<'tree>,
    pub name: Identifier<'tree>,
}
impl<'tree> ::treesitter_types::FromNode<'tree> for TemplateFunction<'tree> {
    #[allow(clippy::match_single_binding, clippy::suspicious_else_formatting)]
    fn from_node(
        node: ::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        debug_assert_eq!(node.kind(), "template_function");
        Ok(Self {
            span: ::treesitter_types::Span::from(node),
            arguments: {
                let child = node.child_by_field_name("arguments").ok_or_else(|| {
                    ::treesitter_types::ParseError::missing_field("arguments", node)
                })?;
                <TemplateArgumentList as ::treesitter_types::FromNode>::from_node(child, src)?
            },
            name: {
                let child = node
                    .child_by_field_name("name")
                    .ok_or_else(|| ::treesitter_types::ParseError::missing_field("name", node))?;
                <Identifier as ::treesitter_types::FromNode>::from_node(child, src)?
            },
        })
    }
}
impl ::treesitter_types::Spanned for TemplateFunction<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        self.span
    }
}
#[derive(Debug, Clone)]
pub struct TemplateInstantiation<'tree> {
    pub span: ::treesitter_types::Span,
    pub declarator: Declarator<'tree>,
    pub r#type: ::core::option::Option<TypeSpecifier<'tree>>,
    pub children: ::std::vec::Vec<TemplateInstantiationChildren<'tree>>,
}
impl<'tree> ::treesitter_types::FromNode<'tree> for TemplateInstantiation<'tree> {
    #[allow(clippy::match_single_binding, clippy::suspicious_else_formatting)]
    fn from_node(
        node: ::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        debug_assert_eq!(node.kind(), "template_instantiation");
        Ok(Self {
            span: ::treesitter_types::Span::from(node),
            declarator: {
                let child = node.child_by_field_name("declarator").ok_or_else(|| {
                    ::treesitter_types::ParseError::missing_field("declarator", node)
                })?;
                <Declarator as ::treesitter_types::FromNode>::from_node(child, src)?
            },
            r#type: match node.child_by_field_name("type") {
                Some(child) => Some(<TypeSpecifier as ::treesitter_types::FromNode>::from_node(
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
                        <TemplateInstantiationChildren as ::treesitter_types::FromNode>::from_node(
                            child, src,
                        )?,
                    );
                }
                items
            },
        })
    }
}
impl ::treesitter_types::Spanned for TemplateInstantiation<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        self.span
    }
}
#[derive(Debug, Clone)]
pub struct TemplateMethod<'tree> {
    pub span: ::treesitter_types::Span,
    pub arguments: TemplateArgumentList<'tree>,
    pub name: TemplateMethodName<'tree>,
}
impl<'tree> ::treesitter_types::FromNode<'tree> for TemplateMethod<'tree> {
    #[allow(clippy::match_single_binding, clippy::suspicious_else_formatting)]
    fn from_node(
        node: ::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        debug_assert_eq!(node.kind(), "template_method");
        Ok(Self {
            span: ::treesitter_types::Span::from(node),
            arguments: {
                let child = node.child_by_field_name("arguments").ok_or_else(|| {
                    ::treesitter_types::ParseError::missing_field("arguments", node)
                })?;
                <TemplateArgumentList as ::treesitter_types::FromNode>::from_node(child, src)?
            },
            name: {
                let child = node
                    .child_by_field_name("name")
                    .ok_or_else(|| ::treesitter_types::ParseError::missing_field("name", node))?;
                <TemplateMethodName as ::treesitter_types::FromNode>::from_node(child, src)?
            },
        })
    }
}
impl ::treesitter_types::Spanned for TemplateMethod<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        self.span
    }
}
#[derive(Debug, Clone)]
pub struct TemplateParameterList<'tree> {
    pub span: ::treesitter_types::Span,
    pub children: ::std::vec::Vec<TemplateParameterListChildren<'tree>>,
}
impl<'tree> ::treesitter_types::FromNode<'tree> for TemplateParameterList<'tree> {
    #[allow(clippy::match_single_binding, clippy::suspicious_else_formatting)]
    fn from_node(
        node: ::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        debug_assert_eq!(node.kind(), "template_parameter_list");
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
                        <TemplateParameterListChildren as ::treesitter_types::FromNode>::from_node(
                            child, src,
                        )?,
                    );
                }
                items
            },
        })
    }
}
impl ::treesitter_types::Spanned for TemplateParameterList<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        self.span
    }
}
#[derive(Debug, Clone)]
pub struct TemplateTemplateParameterDeclaration<'tree> {
    pub span: ::treesitter_types::Span,
    pub parameters: TemplateParameterList<'tree>,
    pub children: TemplateTemplateParameterDeclarationChildren<'tree>,
}
impl<'tree> ::treesitter_types::FromNode<'tree> for TemplateTemplateParameterDeclaration<'tree> {
    #[allow(clippy::match_single_binding, clippy::suspicious_else_formatting)]
    fn from_node(
        node: ::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        debug_assert_eq!(node.kind(), "template_template_parameter_declaration");
        Ok(Self {
            span: ::treesitter_types::Span::from(node),
            parameters: {
                let child = node.child_by_field_name("parameters").ok_or_else(|| {
                    ::treesitter_types::ParseError::missing_field("parameters", node)
                })?;
                <TemplateParameterList as ::treesitter_types::FromNode>::from_node(child, src)?
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
                <TemplateTemplateParameterDeclarationChildren as ::treesitter_types::FromNode>::from_node(
                    child,
                    src,
                )?
            },
        })
    }
}
impl ::treesitter_types::Spanned for TemplateTemplateParameterDeclaration<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        self.span
    }
}
#[derive(Debug, Clone)]
pub struct TemplateType<'tree> {
    pub span: ::treesitter_types::Span,
    pub arguments: TemplateArgumentList<'tree>,
    pub name: TypeIdentifier<'tree>,
}
impl<'tree> ::treesitter_types::FromNode<'tree> for TemplateType<'tree> {
    #[allow(clippy::match_single_binding, clippy::suspicious_else_formatting)]
    fn from_node(
        node: ::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        debug_assert_eq!(node.kind(), "template_type");
        Ok(Self {
            span: ::treesitter_types::Span::from(node),
            arguments: {
                let child = node.child_by_field_name("arguments").ok_or_else(|| {
                    ::treesitter_types::ParseError::missing_field("arguments", node)
                })?;
                <TemplateArgumentList as ::treesitter_types::FromNode>::from_node(child, src)?
            },
            name: {
                let child = node
                    .child_by_field_name("name")
                    .ok_or_else(|| ::treesitter_types::ParseError::missing_field("name", node))?;
                <TypeIdentifier as ::treesitter_types::FromNode>::from_node(child, src)?
            },
        })
    }
}
impl ::treesitter_types::Spanned for TemplateType<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        self.span
    }
}
#[derive(Debug, Clone)]
pub struct ThrowSpecifier<'tree> {
    pub span: ::treesitter_types::Span,
    pub children: ::std::vec::Vec<TypeDescriptor<'tree>>,
}
impl<'tree> ::treesitter_types::FromNode<'tree> for ThrowSpecifier<'tree> {
    #[allow(clippy::match_single_binding, clippy::suspicious_else_formatting)]
    fn from_node(
        node: ::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        debug_assert_eq!(node.kind(), "throw_specifier");
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
                    items.push(<TypeDescriptor as ::treesitter_types::FromNode>::from_node(
                        child, src,
                    )?);
                }
                items
            },
        })
    }
}
impl ::treesitter_types::Spanned for ThrowSpecifier<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        self.span
    }
}
#[derive(Debug, Clone)]
pub struct ThrowStatement<'tree> {
    pub span: ::treesitter_types::Span,
    pub children: ::core::option::Option<Expression<'tree>>,
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
                match non_field_children.first() {
                    Some(&child) => Some(<Expression as ::treesitter_types::FromNode>::from_node(
                        child, src,
                    )?),
                    None => None,
                }
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
pub struct TrailingReturnType<'tree> {
    pub span: ::treesitter_types::Span,
    pub children: TypeDescriptor<'tree>,
}
impl<'tree> ::treesitter_types::FromNode<'tree> for TrailingReturnType<'tree> {
    #[allow(clippy::match_single_binding, clippy::suspicious_else_formatting)]
    fn from_node(
        node: ::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        debug_assert_eq!(node.kind(), "trailing_return_type");
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
                <TypeDescriptor as ::treesitter_types::FromNode>::from_node(child, src)?
            },
        })
    }
}
impl ::treesitter_types::Spanned for TrailingReturnType<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        self.span
    }
}
#[derive(Debug, Clone)]
pub struct TranslationUnit<'tree> {
    pub span: ::treesitter_types::Span,
    pub children: ::std::vec::Vec<TranslationUnitChildren<'tree>>,
}
impl<'tree> ::treesitter_types::FromNode<'tree> for TranslationUnit<'tree> {
    #[allow(clippy::match_single_binding, clippy::suspicious_else_formatting)]
    fn from_node(
        node: ::tree_sitter::Node<'tree>,
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
                    items.push(
                        <TranslationUnitChildren as ::treesitter_types::FromNode>::from_node(
                            child, src,
                        )?,
                    );
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
#[derive(Debug, Clone)]
pub struct TryStatement<'tree> {
    pub span: ::treesitter_types::Span,
    pub body: CompoundStatement<'tree>,
    pub children: ::std::vec::Vec<TryStatementChildren<'tree>>,
}
impl<'tree> ::treesitter_types::FromNode<'tree> for TryStatement<'tree> {
    #[allow(clippy::match_single_binding, clippy::suspicious_else_formatting)]
    fn from_node(
        node: ::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        debug_assert_eq!(node.kind(), "try_statement");
        Ok(Self {
            span: ::treesitter_types::Span::from(node),
            body: {
                let child = node
                    .child_by_field_name("body")
                    .ok_or_else(|| ::treesitter_types::ParseError::missing_field("body", node))?;
                <CompoundStatement as ::treesitter_types::FromNode>::from_node(child, src)?
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
                        <TryStatementChildren as ::treesitter_types::FromNode>::from_node(
                            child, src,
                        )?,
                    );
                }
                items
            },
        })
    }
}
impl ::treesitter_types::Spanned for TryStatement<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        self.span
    }
}
#[derive(Debug, Clone)]
pub struct TypeDefinition<'tree> {
    pub span: ::treesitter_types::Span,
    pub declarator: ::std::vec::Vec<TypeDeclarator<'tree>>,
    pub r#type: TypeSpecifier<'tree>,
    pub children: ::std::vec::Vec<TypeDefinitionChildren<'tree>>,
}
impl<'tree> ::treesitter_types::FromNode<'tree> for TypeDefinition<'tree> {
    #[allow(clippy::match_single_binding, clippy::suspicious_else_formatting)]
    fn from_node(
        node: ::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        debug_assert_eq!(node.kind(), "type_definition");
        Ok(Self {
            span: ::treesitter_types::Span::from(node),
            declarator: {
                let mut cursor = node.walk();
                let mut items = ::std::vec::Vec::new();
                for child in node.children_by_field_name("declarator", &mut cursor) {
                    items.push(<TypeDeclarator as ::treesitter_types::FromNode>::from_node(
                        child, src,
                    )?);
                }
                items
            },
            r#type: {
                let child = node
                    .child_by_field_name("type")
                    .ok_or_else(|| ::treesitter_types::ParseError::missing_field("type", node))?;
                <TypeSpecifier as ::treesitter_types::FromNode>::from_node(child, src)?
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
                        <TypeDefinitionChildren as ::treesitter_types::FromNode>::from_node(
                            child, src,
                        )?,
                    );
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
#[derive(Debug, Clone)]
pub struct TypeDescriptor<'tree> {
    pub span: ::treesitter_types::Span,
    pub declarator: ::core::option::Option<AbstractDeclarator<'tree>>,
    pub r#type: TypeSpecifier<'tree>,
    pub children: ::std::vec::Vec<TypeQualifier<'tree>>,
}
impl<'tree> ::treesitter_types::FromNode<'tree> for TypeDescriptor<'tree> {
    #[allow(clippy::match_single_binding, clippy::suspicious_else_formatting)]
    fn from_node(
        node: ::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        debug_assert_eq!(node.kind(), "type_descriptor");
        Ok(Self {
            span: ::treesitter_types::Span::from(node),
            declarator: match node.child_by_field_name("declarator") {
                Some(child) => Some(
                    <AbstractDeclarator as ::treesitter_types::FromNode>::from_node(child, src)?,
                ),
                None => None,
            },
            r#type: {
                let child = node
                    .child_by_field_name("type")
                    .ok_or_else(|| ::treesitter_types::ParseError::missing_field("type", node))?;
                <TypeSpecifier as ::treesitter_types::FromNode>::from_node(child, src)?
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
                    items.push(<TypeQualifier as ::treesitter_types::FromNode>::from_node(
                        child, src,
                    )?);
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
#[derive(Debug, Clone)]
pub struct TypeParameterDeclaration<'tree> {
    pub span: ::treesitter_types::Span,
    pub children: ::core::option::Option<TypeIdentifier<'tree>>,
}
impl<'tree> ::treesitter_types::FromNode<'tree> for TypeParameterDeclaration<'tree> {
    #[allow(clippy::match_single_binding, clippy::suspicious_else_formatting)]
    fn from_node(
        node: ::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        debug_assert_eq!(node.kind(), "type_parameter_declaration");
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
                        <TypeIdentifier as ::treesitter_types::FromNode>::from_node(child, src)?,
                    ),
                    None => None,
                }
            },
        })
    }
}
impl ::treesitter_types::Spanned for TypeParameterDeclaration<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        self.span
    }
}
#[derive(Debug, Clone)]
pub struct TypeQualifier<'tree> {
    pub span: ::treesitter_types::Span,
    pub children: ::core::option::Option<AlignasQualifier<'tree>>,
}
impl<'tree> ::treesitter_types::FromNode<'tree> for TypeQualifier<'tree> {
    #[allow(clippy::match_single_binding, clippy::suspicious_else_formatting)]
    fn from_node(
        node: ::tree_sitter::Node<'tree>,
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
                    Some(&child) => Some(
                        <AlignasQualifier as ::treesitter_types::FromNode>::from_node(child, src)?,
                    ),
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
#[derive(Debug, Clone)]
pub struct TypeRequirement<'tree> {
    pub span: ::treesitter_types::Span,
    pub children: TypeRequirementChildren<'tree>,
}
impl<'tree> ::treesitter_types::FromNode<'tree> for TypeRequirement<'tree> {
    #[allow(clippy::match_single_binding, clippy::suspicious_else_formatting)]
    fn from_node(
        node: ::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        debug_assert_eq!(node.kind(), "type_requirement");
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
                <TypeRequirementChildren as ::treesitter_types::FromNode>::from_node(child, src)?
            },
        })
    }
}
impl ::treesitter_types::Spanned for TypeRequirement<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        self.span
    }
}
#[derive(Debug, Clone)]
pub struct UnaryExpression<'tree> {
    pub span: ::treesitter_types::Span,
    pub argument: UnaryExpressionArgument<'tree>,
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
            argument: {
                let child = node.child_by_field_name("argument").ok_or_else(|| {
                    ::treesitter_types::ParseError::missing_field("argument", node)
                })?;
                <UnaryExpressionArgument as ::treesitter_types::FromNode>::from_node(child, src)?
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
#[derive(Debug, Clone)]
pub struct UnionSpecifier<'tree> {
    pub span: ::treesitter_types::Span,
    pub body: ::core::option::Option<FieldDeclarationList<'tree>>,
    pub name: ::core::option::Option<UnionSpecifierName<'tree>>,
    pub children: ::std::vec::Vec<UnionSpecifierChildren<'tree>>,
}
impl<'tree> ::treesitter_types::FromNode<'tree> for UnionSpecifier<'tree> {
    #[allow(clippy::match_single_binding, clippy::suspicious_else_formatting)]
    fn from_node(
        node: ::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        debug_assert_eq!(node.kind(), "union_specifier");
        Ok(Self {
            span: ::treesitter_types::Span::from(node),
            body: match node.child_by_field_name("body") {
                Some(child) => Some(
                    <FieldDeclarationList as ::treesitter_types::FromNode>::from_node(child, src)?,
                ),
                None => None,
            },
            name: match node.child_by_field_name("name") {
                Some(child) => Some(
                    <UnionSpecifierName as ::treesitter_types::FromNode>::from_node(child, src)?,
                ),
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
                        <UnionSpecifierChildren as ::treesitter_types::FromNode>::from_node(
                            child, src,
                        )?,
                    );
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
#[derive(Debug, Clone)]
pub struct UpdateExpression<'tree> {
    pub span: ::treesitter_types::Span,
    pub argument: Expression<'tree>,
    pub operator: UpdateExpressionOperator,
}
impl<'tree> ::treesitter_types::FromNode<'tree> for UpdateExpression<'tree> {
    #[allow(clippy::match_single_binding, clippy::suspicious_else_formatting)]
    fn from_node(
        node: ::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        debug_assert_eq!(node.kind(), "update_expression");
        Ok(Self {
            span: ::treesitter_types::Span::from(node),
            argument: {
                let child = node.child_by_field_name("argument").ok_or_else(|| {
                    ::treesitter_types::ParseError::missing_field("argument", node)
                })?;
                <Expression as ::treesitter_types::FromNode>::from_node(child, src)?
            },
            operator: {
                let child = node.child_by_field_name("operator").ok_or_else(|| {
                    ::treesitter_types::ParseError::missing_field("operator", node)
                })?;
                <UpdateExpressionOperator as ::treesitter_types::FromNode>::from_node(child, src)?
            },
        })
    }
}
impl ::treesitter_types::Spanned for UpdateExpression<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        self.span
    }
}
#[derive(Debug, Clone)]
pub struct UserDefinedLiteral<'tree> {
    pub span: ::treesitter_types::Span,
    pub children: ::std::vec::Vec<UserDefinedLiteralChildren<'tree>>,
}
impl<'tree> ::treesitter_types::FromNode<'tree> for UserDefinedLiteral<'tree> {
    #[allow(clippy::match_single_binding, clippy::suspicious_else_formatting)]
    fn from_node(
        node: ::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        debug_assert_eq!(node.kind(), "user_defined_literal");
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
                        <UserDefinedLiteralChildren as ::treesitter_types::FromNode>::from_node(
                            child, src,
                        )?,
                    );
                }
                items
            },
        })
    }
}
impl ::treesitter_types::Spanned for UserDefinedLiteral<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        self.span
    }
}
#[derive(Debug, Clone)]
pub struct UsingDeclaration<'tree> {
    pub span: ::treesitter_types::Span,
    pub children: ::std::vec::Vec<UsingDeclarationChildren<'tree>>,
}
impl<'tree> ::treesitter_types::FromNode<'tree> for UsingDeclaration<'tree> {
    #[allow(clippy::match_single_binding, clippy::suspicious_else_formatting)]
    fn from_node(
        node: ::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        debug_assert_eq!(node.kind(), "using_declaration");
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
                        <UsingDeclarationChildren as ::treesitter_types::FromNode>::from_node(
                            child, src,
                        )?,
                    );
                }
                items
            },
        })
    }
}
impl ::treesitter_types::Spanned for UsingDeclaration<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        self.span
    }
}
#[derive(Debug, Clone)]
pub struct VariadicDeclarator<'tree> {
    pub span: ::treesitter_types::Span,
    pub children: ::core::option::Option<Identifier<'tree>>,
}
impl<'tree> ::treesitter_types::FromNode<'tree> for VariadicDeclarator<'tree> {
    #[allow(clippy::match_single_binding, clippy::suspicious_else_formatting)]
    fn from_node(
        node: ::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        debug_assert_eq!(node.kind(), "variadic_declarator");
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
impl ::treesitter_types::Spanned for VariadicDeclarator<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        self.span
    }
}
#[derive(Debug, Clone)]
pub struct VariadicParameterDeclaration<'tree> {
    pub span: ::treesitter_types::Span,
    pub declarator: VariadicParameterDeclarationDeclarator<'tree>,
    pub r#type: TypeSpecifier<'tree>,
    pub children: ::std::vec::Vec<VariadicParameterDeclarationChildren<'tree>>,
}
impl<'tree> ::treesitter_types::FromNode<'tree> for VariadicParameterDeclaration<'tree> {
    #[allow(clippy::match_single_binding, clippy::suspicious_else_formatting)]
    fn from_node(
        node: ::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        debug_assert_eq!(node.kind(), "variadic_parameter_declaration");
        Ok(Self {
            span: ::treesitter_types::Span::from(node),
            declarator: {
                let child = node.child_by_field_name("declarator").ok_or_else(|| {
                    ::treesitter_types::ParseError::missing_field("declarator", node)
                })?;
                <VariadicParameterDeclarationDeclarator as ::treesitter_types::FromNode>::from_node(
                    child, src,
                )?
            },
            r#type: {
                let child = node
                    .child_by_field_name("type")
                    .ok_or_else(|| ::treesitter_types::ParseError::missing_field("type", node))?;
                <TypeSpecifier as ::treesitter_types::FromNode>::from_node(child, src)?
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
                            <VariadicParameterDeclarationChildren as ::treesitter_types::FromNode>::from_node(
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
impl ::treesitter_types::Spanned for VariadicParameterDeclaration<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        self.span
    }
}
#[derive(Debug, Clone)]
pub struct VariadicTypeParameterDeclaration<'tree> {
    pub span: ::treesitter_types::Span,
    pub children: ::core::option::Option<TypeIdentifier<'tree>>,
}
impl<'tree> ::treesitter_types::FromNode<'tree> for VariadicTypeParameterDeclaration<'tree> {
    #[allow(clippy::match_single_binding, clippy::suspicious_else_formatting)]
    fn from_node(
        node: ::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        debug_assert_eq!(node.kind(), "variadic_type_parameter_declaration");
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
                        <TypeIdentifier as ::treesitter_types::FromNode>::from_node(child, src)?,
                    ),
                    None => None,
                }
            },
        })
    }
}
impl ::treesitter_types::Spanned for VariadicTypeParameterDeclaration<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        self.span
    }
}
#[derive(Debug, Clone)]
pub struct VirtualSpecifier<'tree> {
    pub span: ::treesitter_types::Span,
    text: &'tree str,
}
impl<'tree> ::treesitter_types::FromNode<'tree> for VirtualSpecifier<'tree> {
    fn from_node(
        node: ::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        debug_assert_eq!(node.kind(), "virtual_specifier");
        Ok(Self {
            span: ::treesitter_types::Span::from(node),
            text: node.utf8_text(src)?,
        })
    }
}
impl<'tree> ::treesitter_types::LeafNode<'tree> for VirtualSpecifier<'tree> {
    fn text(&self) -> &'tree str {
        self.text
    }
}
impl ::treesitter_types::Spanned for VirtualSpecifier<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        self.span
    }
}
#[derive(Debug, Clone)]
pub struct WhileStatement<'tree> {
    pub span: ::treesitter_types::Span,
    pub body: Statement<'tree>,
    pub condition: ConditionClause<'tree>,
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
                <Statement as ::treesitter_types::FromNode>::from_node(child, src)?
            },
            condition: {
                let child = node.child_by_field_name("condition").ok_or_else(|| {
                    ::treesitter_types::ParseError::missing_field("condition", node)
                })?;
                <ConditionClause as ::treesitter_types::FromNode>::from_node(child, src)?
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
pub struct Auto<'tree> {
    pub span: ::treesitter_types::Span,
    text: &'tree str,
}
impl<'tree> ::treesitter_types::FromNode<'tree> for Auto<'tree> {
    fn from_node(
        node: ::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        debug_assert_eq!(node.kind(), "auto");
        Ok(Self {
            span: ::treesitter_types::Span::from(node),
            text: node.utf8_text(src)?,
        })
    }
}
impl<'tree> ::treesitter_types::LeafNode<'tree> for Auto<'tree> {
    fn text(&self) -> &'tree str {
        self.text
    }
}
impl ::treesitter_types::Spanned for Auto<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        self.span
    }
}
#[derive(Debug, Clone)]
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
#[derive(Debug, Clone)]
pub struct FieldIdentifier<'tree> {
    pub span: ::treesitter_types::Span,
    text: &'tree str,
}
impl<'tree> ::treesitter_types::FromNode<'tree> for FieldIdentifier<'tree> {
    fn from_node(
        node: ::tree_sitter::Node<'tree>,
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
pub struct LiteralSuffix<'tree> {
    pub span: ::treesitter_types::Span,
    text: &'tree str,
}
impl<'tree> ::treesitter_types::FromNode<'tree> for LiteralSuffix<'tree> {
    fn from_node(
        node: ::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        debug_assert_eq!(node.kind(), "literal_suffix");
        Ok(Self {
            span: ::treesitter_types::Span::from(node),
            text: node.utf8_text(src)?,
        })
    }
}
impl<'tree> ::treesitter_types::LeafNode<'tree> for LiteralSuffix<'tree> {
    fn text(&self) -> &'tree str {
        self.text
    }
}
impl ::treesitter_types::Spanned for LiteralSuffix<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        self.span
    }
}
#[derive(Debug, Clone)]
pub struct MsRestrictModifier<'tree> {
    pub span: ::treesitter_types::Span,
    text: &'tree str,
}
impl<'tree> ::treesitter_types::FromNode<'tree> for MsRestrictModifier<'tree> {
    fn from_node(
        node: ::tree_sitter::Node<'tree>,
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
#[derive(Debug, Clone)]
pub struct MsSignedPtrModifier<'tree> {
    pub span: ::treesitter_types::Span,
    text: &'tree str,
}
impl<'tree> ::treesitter_types::FromNode<'tree> for MsSignedPtrModifier<'tree> {
    fn from_node(
        node: ::tree_sitter::Node<'tree>,
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
#[derive(Debug, Clone)]
pub struct MsUnsignedPtrModifier<'tree> {
    pub span: ::treesitter_types::Span,
    text: &'tree str,
}
impl<'tree> ::treesitter_types::FromNode<'tree> for MsUnsignedPtrModifier<'tree> {
    fn from_node(
        node: ::tree_sitter::Node<'tree>,
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
#[derive(Debug, Clone)]
pub struct NamespaceIdentifier<'tree> {
    pub span: ::treesitter_types::Span,
    text: &'tree str,
}
impl<'tree> ::treesitter_types::FromNode<'tree> for NamespaceIdentifier<'tree> {
    fn from_node(
        node: ::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        debug_assert_eq!(node.kind(), "namespace_identifier");
        Ok(Self {
            span: ::treesitter_types::Span::from(node),
            text: node.utf8_text(src)?,
        })
    }
}
impl<'tree> ::treesitter_types::LeafNode<'tree> for NamespaceIdentifier<'tree> {
    fn text(&self) -> &'tree str {
        self.text
    }
}
impl ::treesitter_types::Spanned for NamespaceIdentifier<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        self.span
    }
}
#[derive(Debug, Clone)]
pub struct NumberLiteral<'tree> {
    pub span: ::treesitter_types::Span,
    text: &'tree str,
}
impl<'tree> ::treesitter_types::FromNode<'tree> for NumberLiteral<'tree> {
    fn from_node(
        node: ::tree_sitter::Node<'tree>,
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
#[derive(Debug, Clone)]
pub struct PreprocArg<'tree> {
    pub span: ::treesitter_types::Span,
    text: &'tree str,
}
impl<'tree> ::treesitter_types::FromNode<'tree> for PreprocArg<'tree> {
    fn from_node(
        node: ::tree_sitter::Node<'tree>,
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
#[derive(Debug, Clone)]
pub struct PreprocDirective<'tree> {
    pub span: ::treesitter_types::Span,
    text: &'tree str,
}
impl<'tree> ::treesitter_types::FromNode<'tree> for PreprocDirective<'tree> {
    fn from_node(
        node: ::tree_sitter::Node<'tree>,
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
#[derive(Debug, Clone)]
pub struct PrimitiveType<'tree> {
    pub span: ::treesitter_types::Span,
    text: &'tree str,
}
impl<'tree> ::treesitter_types::FromNode<'tree> for PrimitiveType<'tree> {
    fn from_node(
        node: ::tree_sitter::Node<'tree>,
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
#[derive(Debug, Clone)]
pub struct RawStringContent<'tree> {
    pub span: ::treesitter_types::Span,
    text: &'tree str,
}
impl<'tree> ::treesitter_types::FromNode<'tree> for RawStringContent<'tree> {
    fn from_node(
        node: ::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        debug_assert_eq!(node.kind(), "raw_string_content");
        Ok(Self {
            span: ::treesitter_types::Span::from(node),
            text: node.utf8_text(src)?,
        })
    }
}
impl<'tree> ::treesitter_types::LeafNode<'tree> for RawStringContent<'tree> {
    fn text(&self) -> &'tree str {
        self.text
    }
}
impl ::treesitter_types::Spanned for RawStringContent<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        self.span
    }
}
#[derive(Debug, Clone)]
pub struct RawStringDelimiter<'tree> {
    pub span: ::treesitter_types::Span,
    text: &'tree str,
}
impl<'tree> ::treesitter_types::FromNode<'tree> for RawStringDelimiter<'tree> {
    fn from_node(
        node: ::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        debug_assert_eq!(node.kind(), "raw_string_delimiter");
        Ok(Self {
            span: ::treesitter_types::Span::from(node),
            text: node.utf8_text(src)?,
        })
    }
}
impl<'tree> ::treesitter_types::LeafNode<'tree> for RawStringDelimiter<'tree> {
    fn text(&self) -> &'tree str {
        self.text
    }
}
impl ::treesitter_types::Spanned for RawStringDelimiter<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        self.span
    }
}
#[derive(Debug, Clone)]
pub struct StatementIdentifier<'tree> {
    pub span: ::treesitter_types::Span,
    text: &'tree str,
}
impl<'tree> ::treesitter_types::FromNode<'tree> for StatementIdentifier<'tree> {
    fn from_node(
        node: ::tree_sitter::Node<'tree>,
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
pub struct SystemLibString<'tree> {
    pub span: ::treesitter_types::Span,
    text: &'tree str,
}
impl<'tree> ::treesitter_types::FromNode<'tree> for SystemLibString<'tree> {
    fn from_node(
        node: ::tree_sitter::Node<'tree>,
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
#[derive(Debug, Clone)]
pub struct This<'tree> {
    pub span: ::treesitter_types::Span,
    text: &'tree str,
}
impl<'tree> ::treesitter_types::FromNode<'tree> for This<'tree> {
    fn from_node(
        node: ::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        debug_assert_eq!(node.kind(), "this");
        Ok(Self {
            span: ::treesitter_types::Span::from(node),
            text: node.utf8_text(src)?,
        })
    }
}
impl<'tree> ::treesitter_types::LeafNode<'tree> for This<'tree> {
    fn text(&self) -> &'tree str {
        self.text
    }
}
impl ::treesitter_types::Spanned for This<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        self.span
    }
}
#[derive(Debug, Clone)]
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
pub enum AbstractArrayDeclaratorSize<'tree> {
    Star(::treesitter_types::Span),
    Expression(::std::boxed::Box<Expression<'tree>>),
}
impl<'tree> ::treesitter_types::FromNode<'tree> for AbstractArrayDeclaratorSize<'tree> {
    #[allow(clippy::collapsible_else_if)]
    fn from_node(
        node: ::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        match node.kind() {
            "*" => Ok(Self::Star(::treesitter_types::Span::from(node))),
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
impl ::treesitter_types::Spanned for AbstractArrayDeclaratorSize<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        match self {
            Self::Star(span) => *span,
            Self::Expression(inner) => inner.span(),
        }
    }
}
#[derive(Debug, Clone)]
pub enum AbstractFunctionDeclaratorChildren<'tree> {
    AttributeDeclaration(::std::boxed::Box<AttributeDeclaration<'tree>>),
    AttributeSpecifier(::std::boxed::Box<AttributeSpecifier<'tree>>),
    GnuAsmExpression(::std::boxed::Box<GnuAsmExpression<'tree>>),
    Noexcept(::std::boxed::Box<Noexcept<'tree>>),
    RefQualifier(::std::boxed::Box<RefQualifier<'tree>>),
    RequiresClause(::std::boxed::Box<RequiresClause<'tree>>),
    ThrowSpecifier(::std::boxed::Box<ThrowSpecifier<'tree>>),
    TrailingReturnType(::std::boxed::Box<TrailingReturnType<'tree>>),
    TypeQualifier(::std::boxed::Box<TypeQualifier<'tree>>),
    VirtualSpecifier(::std::boxed::Box<VirtualSpecifier<'tree>>),
}
impl<'tree> ::treesitter_types::FromNode<'tree> for AbstractFunctionDeclaratorChildren<'tree> {
    #[allow(clippy::collapsible_else_if)]
    fn from_node(
        node: ::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        match node.kind() {
            "attribute_declaration" => Ok(Self::AttributeDeclaration(::std::boxed::Box::new(
                <AttributeDeclaration as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "attribute_specifier" => Ok(Self::AttributeSpecifier(::std::boxed::Box::new(
                <AttributeSpecifier as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "gnu_asm_expression" => Ok(Self::GnuAsmExpression(::std::boxed::Box::new(
                <GnuAsmExpression as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "noexcept" => Ok(Self::Noexcept(::std::boxed::Box::new(
                <Noexcept as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "ref_qualifier" => Ok(Self::RefQualifier(::std::boxed::Box::new(
                <RefQualifier as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "requires_clause" => Ok(Self::RequiresClause(::std::boxed::Box::new(
                <RequiresClause as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "throw_specifier" => Ok(Self::ThrowSpecifier(::std::boxed::Box::new(
                <ThrowSpecifier as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "trailing_return_type" => Ok(Self::TrailingReturnType(::std::boxed::Box::new(
                <TrailingReturnType as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "type_qualifier" => Ok(Self::TypeQualifier(::std::boxed::Box::new(
                <TypeQualifier as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "virtual_specifier" => Ok(Self::VirtualSpecifier(::std::boxed::Box::new(
                <VirtualSpecifier as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            other => Err(::treesitter_types::ParseError::unexpected_kind(other, node)),
        }
    }
}
impl ::treesitter_types::Spanned for AbstractFunctionDeclaratorChildren<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        match self {
            Self::AttributeDeclaration(inner) => inner.span(),
            Self::AttributeSpecifier(inner) => inner.span(),
            Self::GnuAsmExpression(inner) => inner.span(),
            Self::Noexcept(inner) => inner.span(),
            Self::RefQualifier(inner) => inner.span(),
            Self::RequiresClause(inner) => inner.span(),
            Self::ThrowSpecifier(inner) => inner.span(),
            Self::TrailingReturnType(inner) => inner.span(),
            Self::TypeQualifier(inner) => inner.span(),
            Self::VirtualSpecifier(inner) => inner.span(),
        }
    }
}
#[derive(Debug, Clone)]
pub enum AbstractParenthesizedDeclaratorChildren<'tree> {
    AbstractDeclarator(::std::boxed::Box<AbstractDeclarator<'tree>>),
    MsCallModifier(::std::boxed::Box<MsCallModifier<'tree>>),
}
impl<'tree> ::treesitter_types::FromNode<'tree> for AbstractParenthesizedDeclaratorChildren<'tree> {
    #[allow(clippy::collapsible_else_if)]
    fn from_node(
        node: ::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        match node.kind() {
            "ms_call_modifier" => Ok(Self::MsCallModifier(::std::boxed::Box::new(
                <MsCallModifier as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            _other => {
                if let Ok(v) =
                    <AbstractDeclarator as ::treesitter_types::FromNode>::from_node(node, src)
                {
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
#[derive(Debug, Clone)]
pub enum AbstractPointerDeclaratorChildren<'tree> {
    MsPointerModifier(::std::boxed::Box<MsPointerModifier<'tree>>),
    TypeQualifier(::std::boxed::Box<TypeQualifier<'tree>>),
}
impl<'tree> ::treesitter_types::FromNode<'tree> for AbstractPointerDeclaratorChildren<'tree> {
    #[allow(clippy::collapsible_else_if)]
    fn from_node(
        node: ::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        match node.kind() {
            "ms_pointer_modifier" => Ok(Self::MsPointerModifier(::std::boxed::Box::new(
                <MsPointerModifier as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "type_qualifier" => Ok(Self::TypeQualifier(::std::boxed::Box::new(
                <TypeQualifier as ::treesitter_types::FromNode>::from_node(node, src)?,
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
#[derive(Debug, Clone)]
pub enum AlignasQualifierChildren<'tree> {
    Expression(::std::boxed::Box<Expression<'tree>>),
    TypeDescriptor(::std::boxed::Box<TypeDescriptor<'tree>>),
}
impl<'tree> ::treesitter_types::FromNode<'tree> for AlignasQualifierChildren<'tree> {
    #[allow(clippy::collapsible_else_if)]
    fn from_node(
        node: ::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        match node.kind() {
            "type_descriptor" => Ok(Self::TypeDescriptor(::std::boxed::Box::new(
                <TypeDescriptor as ::treesitter_types::FromNode>::from_node(node, src)?,
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
impl ::treesitter_types::Spanned for AlignasQualifierChildren<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        match self {
            Self::Expression(inner) => inner.span(),
            Self::TypeDescriptor(inner) => inner.span(),
        }
    }
}
#[derive(Debug, Clone)]
pub enum ArgumentListChildren<'tree> {
    CompoundStatement(::std::boxed::Box<CompoundStatement<'tree>>),
    Expression(::std::boxed::Box<Expression<'tree>>),
    InitializerList(::std::boxed::Box<InitializerList<'tree>>),
    PreprocDefined(::std::boxed::Box<PreprocDefined<'tree>>),
}
impl<'tree> ::treesitter_types::FromNode<'tree> for ArgumentListChildren<'tree> {
    #[allow(clippy::collapsible_else_if)]
    fn from_node(
        node: ::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        match node.kind() {
            "compound_statement" => Ok(Self::CompoundStatement(::std::boxed::Box::new(
                <CompoundStatement as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "initializer_list" => Ok(Self::InitializerList(::std::boxed::Box::new(
                <InitializerList as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "preproc_defined" => Ok(Self::PreprocDefined(::std::boxed::Box::new(
                <PreprocDefined as ::treesitter_types::FromNode>::from_node(node, src)?,
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
impl ::treesitter_types::Spanned for ArgumentListChildren<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        match self {
            Self::CompoundStatement(inner) => inner.span(),
            Self::Expression(inner) => inner.span(),
            Self::InitializerList(inner) => inner.span(),
            Self::PreprocDefined(inner) => inner.span(),
        }
    }
}
#[derive(Debug, Clone)]
pub enum ArrayDeclaratorDeclarator<'tree> {
    Declarator(::std::boxed::Box<Declarator<'tree>>),
    FieldDeclarator(::std::boxed::Box<FieldDeclarator<'tree>>),
    TypeDeclarator(::std::boxed::Box<TypeDeclarator<'tree>>),
}
impl<'tree> ::treesitter_types::FromNode<'tree> for ArrayDeclaratorDeclarator<'tree> {
    #[allow(clippy::collapsible_else_if)]
    fn from_node(
        node: ::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        if let Ok(v) = <Declarator as ::treesitter_types::FromNode>::from_node(node, src) {
            Ok(Self::Declarator(::std::boxed::Box::new(v)))
        } else {
            if let Ok(v) = <FieldDeclarator as ::treesitter_types::FromNode>::from_node(node, src) {
                Ok(Self::FieldDeclarator(::std::boxed::Box::new(v)))
            } else {
                if let Ok(v) =
                    <TypeDeclarator as ::treesitter_types::FromNode>::from_node(node, src)
                {
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
#[derive(Debug, Clone)]
pub enum ArrayDeclaratorSize<'tree> {
    Star(::treesitter_types::Span),
    Expression(::std::boxed::Box<Expression<'tree>>),
}
impl<'tree> ::treesitter_types::FromNode<'tree> for ArrayDeclaratorSize<'tree> {
    #[allow(clippy::collapsible_else_if)]
    fn from_node(
        node: ::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        match node.kind() {
            "*" => Ok(Self::Star(::treesitter_types::Span::from(node))),
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
impl ::treesitter_types::Spanned for ArrayDeclaratorSize<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        match self {
            Self::Star(span) => *span,
            Self::Expression(inner) => inner.span(),
        }
    }
}
#[derive(Debug, Clone)]
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
    AndEq(::treesitter_types::Span),
    OrEq(::treesitter_types::Span),
    XorEq(::treesitter_types::Span),
    PipeEq(::treesitter_types::Span),
}
impl<'tree> ::treesitter_types::FromNode<'tree> for AssignmentExpressionOperator {
    #[allow(clippy::collapsible_else_if)]
    fn from_node(
        node: ::tree_sitter::Node<'tree>,
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
            "and_eq" => Ok(Self::AndEq(::treesitter_types::Span::from(node))),
            "or_eq" => Ok(Self::OrEq(::treesitter_types::Span::from(node))),
            "xor_eq" => Ok(Self::XorEq(::treesitter_types::Span::from(node))),
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
            Self::AndEq(span) => *span,
            Self::OrEq(span) => *span,
            Self::XorEq(span) => *span,
            Self::PipeEq(span) => *span,
        }
    }
}
#[derive(Debug, Clone)]
pub enum AssignmentExpressionRight<'tree> {
    Expression(::std::boxed::Box<Expression<'tree>>),
    InitializerList(::std::boxed::Box<InitializerList<'tree>>),
}
impl<'tree> ::treesitter_types::FromNode<'tree> for AssignmentExpressionRight<'tree> {
    #[allow(clippy::collapsible_else_if)]
    fn from_node(
        node: ::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        match node.kind() {
            "initializer_list" => Ok(Self::InitializerList(::std::boxed::Box::new(
                <InitializerList as ::treesitter_types::FromNode>::from_node(node, src)?,
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
impl ::treesitter_types::Spanned for AssignmentExpressionRight<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        match self {
            Self::Expression(inner) => inner.span(),
            Self::InitializerList(inner) => inner.span(),
        }
    }
}
#[derive(Debug, Clone)]
pub enum AttributeDeclarationChildren<'tree> {
    Annotation(::std::boxed::Box<Annotation<'tree>>),
    Attribute(::std::boxed::Box<Attribute<'tree>>),
}
impl<'tree> ::treesitter_types::FromNode<'tree> for AttributeDeclarationChildren<'tree> {
    #[allow(clippy::collapsible_else_if)]
    fn from_node(
        node: ::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        match node.kind() {
            "annotation" => Ok(Self::Annotation(::std::boxed::Box::new(
                <Annotation as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "attribute" => Ok(Self::Attribute(::std::boxed::Box::new(
                <Attribute as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            other => Err(::treesitter_types::ParseError::unexpected_kind(other, node)),
        }
    }
}
impl ::treesitter_types::Spanned for AttributeDeclarationChildren<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        match self {
            Self::Annotation(inner) => inner.span(),
            Self::Attribute(inner) => inner.span(),
        }
    }
}
#[derive(Debug, Clone)]
pub enum AttributedDeclaratorChildren<'tree> {
    Declarator(::std::boxed::Box<Declarator<'tree>>),
    FieldDeclarator(::std::boxed::Box<FieldDeclarator<'tree>>),
    TypeDeclarator(::std::boxed::Box<TypeDeclarator<'tree>>),
    AttributeDeclaration(::std::boxed::Box<AttributeDeclaration<'tree>>),
}
impl<'tree> ::treesitter_types::FromNode<'tree> for AttributedDeclaratorChildren<'tree> {
    #[allow(clippy::collapsible_else_if)]
    fn from_node(
        node: ::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        match node.kind() {
            "attribute_declaration" => Ok(Self::AttributeDeclaration(::std::boxed::Box::new(
                <AttributeDeclaration as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            _other => {
                if let Ok(v) = <Declarator as ::treesitter_types::FromNode>::from_node(node, src) {
                    Ok(Self::Declarator(::std::boxed::Box::new(v)))
                } else {
                    if let Ok(v) =
                        <FieldDeclarator as ::treesitter_types::FromNode>::from_node(node, src)
                    {
                        Ok(Self::FieldDeclarator(::std::boxed::Box::new(v)))
                    } else {
                        if let Ok(v) =
                            <TypeDeclarator as ::treesitter_types::FromNode>::from_node(node, src)
                        {
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
#[derive(Debug, Clone)]
pub enum AttributedStatementChildren<'tree> {
    AttributeDeclaration(::std::boxed::Box<AttributeDeclaration<'tree>>),
    Statement(::std::boxed::Box<Statement<'tree>>),
}
impl<'tree> ::treesitter_types::FromNode<'tree> for AttributedStatementChildren<'tree> {
    #[allow(clippy::collapsible_else_if)]
    fn from_node(
        node: ::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        match node.kind() {
            "attribute_declaration" => Ok(Self::AttributeDeclaration(::std::boxed::Box::new(
                <AttributeDeclaration as ::treesitter_types::FromNode>::from_node(node, src)?,
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
impl ::treesitter_types::Spanned for AttributedStatementChildren<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        match self {
            Self::AttributeDeclaration(inner) => inner.span(),
            Self::Statement(inner) => inner.span(),
        }
    }
}
#[derive(Debug, Clone)]
pub enum BaseClassClauseChildren<'tree> {
    AccessSpecifier(::std::boxed::Box<AccessSpecifier<'tree>>),
    AttributeDeclaration(::std::boxed::Box<AttributeDeclaration<'tree>>),
    QualifiedIdentifier(::std::boxed::Box<QualifiedIdentifier<'tree>>),
    SpliceTypeSpecifier(::std::boxed::Box<SpliceTypeSpecifier<'tree>>),
    TemplateType(::std::boxed::Box<TemplateType<'tree>>),
    TypeIdentifier(::std::boxed::Box<TypeIdentifier<'tree>>),
}
impl<'tree> ::treesitter_types::FromNode<'tree> for BaseClassClauseChildren<'tree> {
    #[allow(clippy::collapsible_else_if)]
    fn from_node(
        node: ::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        match node.kind() {
            "access_specifier" => Ok(Self::AccessSpecifier(::std::boxed::Box::new(
                <AccessSpecifier as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "attribute_declaration" => Ok(Self::AttributeDeclaration(::std::boxed::Box::new(
                <AttributeDeclaration as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "qualified_identifier" => Ok(Self::QualifiedIdentifier(::std::boxed::Box::new(
                <QualifiedIdentifier as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "splice_type_specifier" => Ok(Self::SpliceTypeSpecifier(::std::boxed::Box::new(
                <SpliceTypeSpecifier as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "template_type" => Ok(Self::TemplateType(::std::boxed::Box::new(
                <TemplateType as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "type_identifier" => Ok(Self::TypeIdentifier(::std::boxed::Box::new(
                <TypeIdentifier as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            other => Err(::treesitter_types::ParseError::unexpected_kind(other, node)),
        }
    }
}
impl ::treesitter_types::Spanned for BaseClassClauseChildren<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        match self {
            Self::AccessSpecifier(inner) => inner.span(),
            Self::AttributeDeclaration(inner) => inner.span(),
            Self::QualifiedIdentifier(inner) => inner.span(),
            Self::SpliceTypeSpecifier(inner) => inner.span(),
            Self::TemplateType(inner) => inner.span(),
            Self::TypeIdentifier(inner) => inner.span(),
        }
    }
}
#[derive(Debug, Clone)]
pub enum BinaryExpressionLeft<'tree> {
    Expression(::std::boxed::Box<Expression<'tree>>),
    PreprocDefined(::std::boxed::Box<PreprocDefined<'tree>>),
}
impl<'tree> ::treesitter_types::FromNode<'tree> for BinaryExpressionLeft<'tree> {
    #[allow(clippy::collapsible_else_if)]
    fn from_node(
        node: ::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        match node.kind() {
            "preproc_defined" => Ok(Self::PreprocDefined(::std::boxed::Box::new(
                <PreprocDefined as ::treesitter_types::FromNode>::from_node(node, src)?,
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
            Self::PreprocDefined(inner) => inner.span(),
        }
    }
}
#[derive(Debug, Clone)]
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
    LtEqGt(::treesitter_types::Span),
    EqEq(::treesitter_types::Span),
    Gt(::treesitter_types::Span),
    GtEq(::treesitter_types::Span),
    Shr(::treesitter_types::Span),
    Caret(::treesitter_types::Span),
    And(::treesitter_types::Span),
    Bitand(::treesitter_types::Span),
    Bitor(::treesitter_types::Span),
    Or(::treesitter_types::Span),
    Xor(::treesitter_types::Span),
    Pipe(::treesitter_types::Span),
    PipePipe(::treesitter_types::Span),
}
impl<'tree> ::treesitter_types::FromNode<'tree> for BinaryExpressionOperator {
    #[allow(clippy::collapsible_else_if)]
    fn from_node(
        node: ::tree_sitter::Node<'tree>,
        _src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        match node.kind() {
            "!=" | "not_eq" => Ok(Self::NotEq(::treesitter_types::Span::from(node))),
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
            "<=>" => Ok(Self::LtEqGt(::treesitter_types::Span::from(node))),
            "==" => Ok(Self::EqEq(::treesitter_types::Span::from(node))),
            ">" => Ok(Self::Gt(::treesitter_types::Span::from(node))),
            ">=" => Ok(Self::GtEq(::treesitter_types::Span::from(node))),
            ">>" => Ok(Self::Shr(::treesitter_types::Span::from(node))),
            "^" => Ok(Self::Caret(::treesitter_types::Span::from(node))),
            "and" => Ok(Self::And(::treesitter_types::Span::from(node))),
            "bitand" => Ok(Self::Bitand(::treesitter_types::Span::from(node))),
            "bitor" => Ok(Self::Bitor(::treesitter_types::Span::from(node))),
            "or" => Ok(Self::Or(::treesitter_types::Span::from(node))),
            "xor" => Ok(Self::Xor(::treesitter_types::Span::from(node))),
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
            Self::LtEqGt(span) => *span,
            Self::EqEq(span) => *span,
            Self::Gt(span) => *span,
            Self::GtEq(span) => *span,
            Self::Shr(span) => *span,
            Self::Caret(span) => *span,
            Self::And(span) => *span,
            Self::Bitand(span) => *span,
            Self::Bitor(span) => *span,
            Self::Or(span) => *span,
            Self::Xor(span) => *span,
            Self::Pipe(span) => *span,
            Self::PipePipe(span) => *span,
        }
    }
}
#[derive(Debug, Clone)]
pub enum BinaryExpressionRight<'tree> {
    Expression(::std::boxed::Box<Expression<'tree>>),
    PreprocDefined(::std::boxed::Box<PreprocDefined<'tree>>),
}
impl<'tree> ::treesitter_types::FromNode<'tree> for BinaryExpressionRight<'tree> {
    #[allow(clippy::collapsible_else_if)]
    fn from_node(
        node: ::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        match node.kind() {
            "preproc_defined" => Ok(Self::PreprocDefined(::std::boxed::Box::new(
                <PreprocDefined as ::treesitter_types::FromNode>::from_node(node, src)?,
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
            Self::PreprocDefined(inner) => inner.span(),
        }
    }
}
#[derive(Debug, Clone)]
pub enum CallExpressionFunction<'tree> {
    Expression(::std::boxed::Box<Expression<'tree>>),
    PrimitiveType(::std::boxed::Box<PrimitiveType<'tree>>),
    SpliceTypeSpecifier(::std::boxed::Box<SpliceTypeSpecifier<'tree>>),
    Typename(::treesitter_types::Span),
}
impl<'tree> ::treesitter_types::FromNode<'tree> for CallExpressionFunction<'tree> {
    #[allow(clippy::collapsible_else_if)]
    fn from_node(
        node: ::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        match node.kind() {
            "primitive_type" => Ok(Self::PrimitiveType(::std::boxed::Box::new(
                <PrimitiveType as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "splice_type_specifier" => Ok(Self::SpliceTypeSpecifier(::std::boxed::Box::new(
                <SpliceTypeSpecifier as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "typename" => Ok(Self::Typename(::treesitter_types::Span::from(node))),
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
impl ::treesitter_types::Spanned for CallExpressionFunction<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        match self {
            Self::Expression(inner) => inner.span(),
            Self::PrimitiveType(inner) => inner.span(),
            Self::SpliceTypeSpecifier(inner) => inner.span(),
            Self::Typename(span) => *span,
        }
    }
}
#[derive(Debug, Clone)]
pub enum CaseStatementChildren<'tree> {
    AttributedStatement(::std::boxed::Box<AttributedStatement<'tree>>),
    BreakStatement(::std::boxed::Box<BreakStatement<'tree>>),
    CoReturnStatement(::std::boxed::Box<CoReturnStatement<'tree>>),
    CoYieldStatement(::std::boxed::Box<CoYieldStatement<'tree>>),
    CompoundStatement(::std::boxed::Box<CompoundStatement<'tree>>),
    ContinueStatement(::std::boxed::Box<ContinueStatement<'tree>>),
    Declaration(::std::boxed::Box<Declaration<'tree>>),
    DoStatement(::std::boxed::Box<DoStatement<'tree>>),
    ExpansionStatement(::std::boxed::Box<ExpansionStatement<'tree>>),
    ExpressionStatement(::std::boxed::Box<ExpressionStatement<'tree>>),
    ForRangeLoop(::std::boxed::Box<ForRangeLoop<'tree>>),
    ForStatement(::std::boxed::Box<ForStatement<'tree>>),
    GotoStatement(::std::boxed::Box<GotoStatement<'tree>>),
    IfStatement(::std::boxed::Box<IfStatement<'tree>>),
    LabeledStatement(::std::boxed::Box<LabeledStatement<'tree>>),
    ReturnStatement(::std::boxed::Box<ReturnStatement<'tree>>),
    SehLeaveStatement(::std::boxed::Box<SehLeaveStatement<'tree>>),
    SehTryStatement(::std::boxed::Box<SehTryStatement<'tree>>),
    SwitchStatement(::std::boxed::Box<SwitchStatement<'tree>>),
    ThrowStatement(::std::boxed::Box<ThrowStatement<'tree>>),
    TryStatement(::std::boxed::Box<TryStatement<'tree>>),
    TypeDefinition(::std::boxed::Box<TypeDefinition<'tree>>),
    WhileStatement(::std::boxed::Box<WhileStatement<'tree>>),
}
impl<'tree> ::treesitter_types::FromNode<'tree> for CaseStatementChildren<'tree> {
    #[allow(clippy::collapsible_else_if)]
    fn from_node(
        node: ::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        match node.kind() {
            "attributed_statement" => Ok(Self::AttributedStatement(::std::boxed::Box::new(
                <AttributedStatement as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "break_statement" => Ok(Self::BreakStatement(::std::boxed::Box::new(
                <BreakStatement as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "co_return_statement" => Ok(Self::CoReturnStatement(::std::boxed::Box::new(
                <CoReturnStatement as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "co_yield_statement" => Ok(Self::CoYieldStatement(::std::boxed::Box::new(
                <CoYieldStatement as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "compound_statement" => Ok(Self::CompoundStatement(::std::boxed::Box::new(
                <CompoundStatement as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "continue_statement" => Ok(Self::ContinueStatement(::std::boxed::Box::new(
                <ContinueStatement as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "declaration" => Ok(Self::Declaration(::std::boxed::Box::new(
                <Declaration as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "do_statement" => Ok(Self::DoStatement(::std::boxed::Box::new(
                <DoStatement as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "expansion_statement" => Ok(Self::ExpansionStatement(::std::boxed::Box::new(
                <ExpansionStatement as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "expression_statement" => Ok(Self::ExpressionStatement(::std::boxed::Box::new(
                <ExpressionStatement as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "for_range_loop" => Ok(Self::ForRangeLoop(::std::boxed::Box::new(
                <ForRangeLoop as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "for_statement" => Ok(Self::ForStatement(::std::boxed::Box::new(
                <ForStatement as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "goto_statement" => Ok(Self::GotoStatement(::std::boxed::Box::new(
                <GotoStatement as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "if_statement" => Ok(Self::IfStatement(::std::boxed::Box::new(
                <IfStatement as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "labeled_statement" => Ok(Self::LabeledStatement(::std::boxed::Box::new(
                <LabeledStatement as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "return_statement" => Ok(Self::ReturnStatement(::std::boxed::Box::new(
                <ReturnStatement as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "seh_leave_statement" => Ok(Self::SehLeaveStatement(::std::boxed::Box::new(
                <SehLeaveStatement as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "seh_try_statement" => Ok(Self::SehTryStatement(::std::boxed::Box::new(
                <SehTryStatement as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "switch_statement" => Ok(Self::SwitchStatement(::std::boxed::Box::new(
                <SwitchStatement as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "throw_statement" => Ok(Self::ThrowStatement(::std::boxed::Box::new(
                <ThrowStatement as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "try_statement" => Ok(Self::TryStatement(::std::boxed::Box::new(
                <TryStatement as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "type_definition" => Ok(Self::TypeDefinition(::std::boxed::Box::new(
                <TypeDefinition as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "while_statement" => Ok(Self::WhileStatement(::std::boxed::Box::new(
                <WhileStatement as ::treesitter_types::FromNode>::from_node(node, src)?,
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
            Self::CoReturnStatement(inner) => inner.span(),
            Self::CoYieldStatement(inner) => inner.span(),
            Self::CompoundStatement(inner) => inner.span(),
            Self::ContinueStatement(inner) => inner.span(),
            Self::Declaration(inner) => inner.span(),
            Self::DoStatement(inner) => inner.span(),
            Self::ExpansionStatement(inner) => inner.span(),
            Self::ExpressionStatement(inner) => inner.span(),
            Self::ForRangeLoop(inner) => inner.span(),
            Self::ForStatement(inner) => inner.span(),
            Self::GotoStatement(inner) => inner.span(),
            Self::IfStatement(inner) => inner.span(),
            Self::LabeledStatement(inner) => inner.span(),
            Self::ReturnStatement(inner) => inner.span(),
            Self::SehLeaveStatement(inner) => inner.span(),
            Self::SehTryStatement(inner) => inner.span(),
            Self::SwitchStatement(inner) => inner.span(),
            Self::ThrowStatement(inner) => inner.span(),
            Self::TryStatement(inner) => inner.span(),
            Self::TypeDefinition(inner) => inner.span(),
            Self::WhileStatement(inner) => inner.span(),
        }
    }
}
#[derive(Debug, Clone)]
pub enum CharLiteralChildren<'tree> {
    Character(::std::boxed::Box<Character<'tree>>),
    EscapeSequence(::std::boxed::Box<EscapeSequence<'tree>>),
}
impl<'tree> ::treesitter_types::FromNode<'tree> for CharLiteralChildren<'tree> {
    #[allow(clippy::collapsible_else_if)]
    fn from_node(
        node: ::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        match node.kind() {
            "character" => Ok(Self::Character(::std::boxed::Box::new(
                <Character as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "escape_sequence" => Ok(Self::EscapeSequence(::std::boxed::Box::new(
                <EscapeSequence as ::treesitter_types::FromNode>::from_node(node, src)?,
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
#[derive(Debug, Clone)]
pub enum ClassSpecifierName<'tree> {
    QualifiedIdentifier(::std::boxed::Box<QualifiedIdentifier<'tree>>),
    SpliceTypeSpecifier(::std::boxed::Box<SpliceTypeSpecifier<'tree>>),
    TemplateType(::std::boxed::Box<TemplateType<'tree>>),
    TypeIdentifier(::std::boxed::Box<TypeIdentifier<'tree>>),
}
impl<'tree> ::treesitter_types::FromNode<'tree> for ClassSpecifierName<'tree> {
    #[allow(clippy::collapsible_else_if)]
    fn from_node(
        node: ::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        match node.kind() {
            "qualified_identifier" => Ok(Self::QualifiedIdentifier(::std::boxed::Box::new(
                <QualifiedIdentifier as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "splice_type_specifier" => Ok(Self::SpliceTypeSpecifier(::std::boxed::Box::new(
                <SpliceTypeSpecifier as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "template_type" => Ok(Self::TemplateType(::std::boxed::Box::new(
                <TemplateType as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "type_identifier" => Ok(Self::TypeIdentifier(::std::boxed::Box::new(
                <TypeIdentifier as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            other => Err(::treesitter_types::ParseError::unexpected_kind(other, node)),
        }
    }
}
impl ::treesitter_types::Spanned for ClassSpecifierName<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        match self {
            Self::QualifiedIdentifier(inner) => inner.span(),
            Self::SpliceTypeSpecifier(inner) => inner.span(),
            Self::TemplateType(inner) => inner.span(),
            Self::TypeIdentifier(inner) => inner.span(),
        }
    }
}
#[derive(Debug, Clone)]
pub enum ClassSpecifierChildren<'tree> {
    AlignasQualifier(::std::boxed::Box<AlignasQualifier<'tree>>),
    AttributeDeclaration(::std::boxed::Box<AttributeDeclaration<'tree>>),
    AttributeSpecifier(::std::boxed::Box<AttributeSpecifier<'tree>>),
    BaseClassClause(::std::boxed::Box<BaseClassClause<'tree>>),
    MsDeclspecModifier(::std::boxed::Box<MsDeclspecModifier<'tree>>),
    VirtualSpecifier(::std::boxed::Box<VirtualSpecifier<'tree>>),
}
impl<'tree> ::treesitter_types::FromNode<'tree> for ClassSpecifierChildren<'tree> {
    #[allow(clippy::collapsible_else_if)]
    fn from_node(
        node: ::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        match node.kind() {
            "alignas_qualifier" => Ok(Self::AlignasQualifier(::std::boxed::Box::new(
                <AlignasQualifier as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "attribute_declaration" => Ok(Self::AttributeDeclaration(::std::boxed::Box::new(
                <AttributeDeclaration as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "attribute_specifier" => Ok(Self::AttributeSpecifier(::std::boxed::Box::new(
                <AttributeSpecifier as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "base_class_clause" => Ok(Self::BaseClassClause(::std::boxed::Box::new(
                <BaseClassClause as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "ms_declspec_modifier" => Ok(Self::MsDeclspecModifier(::std::boxed::Box::new(
                <MsDeclspecModifier as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "virtual_specifier" => Ok(Self::VirtualSpecifier(::std::boxed::Box::new(
                <VirtualSpecifier as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            other => Err(::treesitter_types::ParseError::unexpected_kind(other, node)),
        }
    }
}
impl ::treesitter_types::Spanned for ClassSpecifierChildren<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        match self {
            Self::AlignasQualifier(inner) => inner.span(),
            Self::AttributeDeclaration(inner) => inner.span(),
            Self::AttributeSpecifier(inner) => inner.span(),
            Self::BaseClassClause(inner) => inner.span(),
            Self::MsDeclspecModifier(inner) => inner.span(),
            Self::VirtualSpecifier(inner) => inner.span(),
        }
    }
}
#[derive(Debug, Clone)]
pub enum CoAwaitExpressionOperator {
    CoAwait(::treesitter_types::Span),
}
impl<'tree> ::treesitter_types::FromNode<'tree> for CoAwaitExpressionOperator {
    #[allow(clippy::collapsible_else_if)]
    fn from_node(
        node: ::tree_sitter::Node<'tree>,
        _src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        match node.kind() {
            "co_await" => Ok(Self::CoAwait(::treesitter_types::Span::from(node))),
            other => Err(::treesitter_types::ParseError::unexpected_kind(other, node)),
        }
    }
}
impl ::treesitter_types::Spanned for CoAwaitExpressionOperator {
    fn span(&self) -> ::treesitter_types::Span {
        match self {
            Self::CoAwait(span) => *span,
        }
    }
}
#[derive(Debug, Clone)]
pub enum CommaExpressionRight<'tree> {
    CommaExpression(::std::boxed::Box<CommaExpression<'tree>>),
    Expression(::std::boxed::Box<Expression<'tree>>),
}
impl<'tree> ::treesitter_types::FromNode<'tree> for CommaExpressionRight<'tree> {
    #[allow(clippy::collapsible_else_if)]
    fn from_node(
        node: ::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        match node.kind() {
            "comma_expression" => Ok(Self::CommaExpression(::std::boxed::Box::new(
                <CommaExpression as ::treesitter_types::FromNode>::from_node(node, src)?,
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
impl ::treesitter_types::Spanned for CommaExpressionRight<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        match self {
            Self::CommaExpression(inner) => inner.span(),
            Self::Expression(inner) => inner.span(),
        }
    }
}
#[derive(Debug, Clone)]
pub enum CompoundLiteralExpressionType<'tree> {
    PrimitiveType(::std::boxed::Box<PrimitiveType<'tree>>),
    QualifiedIdentifier(::std::boxed::Box<QualifiedIdentifier<'tree>>),
    SpliceTypeSpecifier(::std::boxed::Box<SpliceTypeSpecifier<'tree>>),
    TemplateType(::std::boxed::Box<TemplateType<'tree>>),
    TypeDescriptor(::std::boxed::Box<TypeDescriptor<'tree>>),
    TypeIdentifier(::std::boxed::Box<TypeIdentifier<'tree>>),
    Typename(::treesitter_types::Span),
}
impl<'tree> ::treesitter_types::FromNode<'tree> for CompoundLiteralExpressionType<'tree> {
    #[allow(clippy::collapsible_else_if)]
    fn from_node(
        node: ::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        match node.kind() {
            "primitive_type" => Ok(Self::PrimitiveType(::std::boxed::Box::new(
                <PrimitiveType as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "qualified_identifier" => Ok(Self::QualifiedIdentifier(::std::boxed::Box::new(
                <QualifiedIdentifier as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "splice_type_specifier" => Ok(Self::SpliceTypeSpecifier(::std::boxed::Box::new(
                <SpliceTypeSpecifier as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "template_type" => Ok(Self::TemplateType(::std::boxed::Box::new(
                <TemplateType as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "type_descriptor" => Ok(Self::TypeDescriptor(::std::boxed::Box::new(
                <TypeDescriptor as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "type_identifier" => Ok(Self::TypeIdentifier(::std::boxed::Box::new(
                <TypeIdentifier as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "typename" => Ok(Self::Typename(::treesitter_types::Span::from(node))),
            other => Err(::treesitter_types::ParseError::unexpected_kind(other, node)),
        }
    }
}
impl ::treesitter_types::Spanned for CompoundLiteralExpressionType<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        match self {
            Self::PrimitiveType(inner) => inner.span(),
            Self::QualifiedIdentifier(inner) => inner.span(),
            Self::SpliceTypeSpecifier(inner) => inner.span(),
            Self::TemplateType(inner) => inner.span(),
            Self::TypeDescriptor(inner) => inner.span(),
            Self::TypeIdentifier(inner) => inner.span(),
            Self::Typename(span) => *span,
        }
    }
}
#[derive(Debug, Clone)]
pub enum CompoundRequirementChildren<'tree> {
    Expression(::std::boxed::Box<Expression<'tree>>),
    TrailingReturnType(::std::boxed::Box<TrailingReturnType<'tree>>),
}
impl<'tree> ::treesitter_types::FromNode<'tree> for CompoundRequirementChildren<'tree> {
    #[allow(clippy::collapsible_else_if)]
    fn from_node(
        node: ::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        match node.kind() {
            "trailing_return_type" => Ok(Self::TrailingReturnType(::std::boxed::Box::new(
                <TrailingReturnType as ::treesitter_types::FromNode>::from_node(node, src)?,
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
impl ::treesitter_types::Spanned for CompoundRequirementChildren<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        match self {
            Self::Expression(inner) => inner.span(),
            Self::TrailingReturnType(inner) => inner.span(),
        }
    }
}
#[derive(Debug, Clone)]
pub enum CompoundStatementChildren<'tree> {
    AliasDeclaration(::std::boxed::Box<AliasDeclaration<'tree>>),
    ConceptDefinition(::std::boxed::Box<ConceptDefinition<'tree>>),
    ConstevalBlockDeclaration(::std::boxed::Box<ConstevalBlockDeclaration<'tree>>),
    Declaration(::std::boxed::Box<Declaration<'tree>>),
    ExportDeclaration(::std::boxed::Box<ExportDeclaration<'tree>>),
    FunctionDefinition(::std::boxed::Box<FunctionDefinition<'tree>>),
    ImportDeclaration(::std::boxed::Box<ImportDeclaration<'tree>>),
    LinkageSpecification(::std::boxed::Box<LinkageSpecification<'tree>>),
    NamespaceAliasDefinition(::std::boxed::Box<NamespaceAliasDefinition<'tree>>),
    NamespaceDefinition(::std::boxed::Box<NamespaceDefinition<'tree>>),
    PreprocCall(::std::boxed::Box<PreprocCall<'tree>>),
    PreprocDef(::std::boxed::Box<PreprocDef<'tree>>),
    PreprocFunctionDef(::std::boxed::Box<PreprocFunctionDef<'tree>>),
    PreprocIf(::std::boxed::Box<PreprocIf<'tree>>),
    PreprocIfdef(::std::boxed::Box<PreprocIfdef<'tree>>),
    PreprocInclude(::std::boxed::Box<PreprocInclude<'tree>>),
    Statement(::std::boxed::Box<Statement<'tree>>),
    StaticAssertDeclaration(::std::boxed::Box<StaticAssertDeclaration<'tree>>),
    TemplateDeclaration(::std::boxed::Box<TemplateDeclaration<'tree>>),
    TemplateInstantiation(::std::boxed::Box<TemplateInstantiation<'tree>>),
    TypeDefinition(::std::boxed::Box<TypeDefinition<'tree>>),
    TypeSpecifier(::std::boxed::Box<TypeSpecifier<'tree>>),
    UsingDeclaration(::std::boxed::Box<UsingDeclaration<'tree>>),
}
impl<'tree> ::treesitter_types::FromNode<'tree> for CompoundStatementChildren<'tree> {
    #[allow(clippy::collapsible_else_if)]
    fn from_node(
        node: ::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        match node.kind() {
            "alias_declaration" => Ok(Self::AliasDeclaration(::std::boxed::Box::new(
                <AliasDeclaration as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "concept_definition" => Ok(Self::ConceptDefinition(::std::boxed::Box::new(
                <ConceptDefinition as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "consteval_block_declaration" => {
                Ok(Self::ConstevalBlockDeclaration(::std::boxed::Box::new(
                    <ConstevalBlockDeclaration as ::treesitter_types::FromNode>::from_node(
                        node, src,
                    )?,
                )))
            }
            "declaration" => Ok(Self::Declaration(::std::boxed::Box::new(
                <Declaration as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "export_declaration" => Ok(Self::ExportDeclaration(::std::boxed::Box::new(
                <ExportDeclaration as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "function_definition" => Ok(Self::FunctionDefinition(::std::boxed::Box::new(
                <FunctionDefinition as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "import_declaration" => Ok(Self::ImportDeclaration(::std::boxed::Box::new(
                <ImportDeclaration as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "linkage_specification" => Ok(Self::LinkageSpecification(::std::boxed::Box::new(
                <LinkageSpecification as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "namespace_alias_definition" => {
                Ok(Self::NamespaceAliasDefinition(::std::boxed::Box::new(
                    <NamespaceAliasDefinition as ::treesitter_types::FromNode>::from_node(
                        node, src,
                    )?,
                )))
            }
            "namespace_definition" => Ok(Self::NamespaceDefinition(::std::boxed::Box::new(
                <NamespaceDefinition as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "preproc_call" => Ok(Self::PreprocCall(::std::boxed::Box::new(
                <PreprocCall as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "preproc_def" => Ok(Self::PreprocDef(::std::boxed::Box::new(
                <PreprocDef as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "preproc_function_def" => Ok(Self::PreprocFunctionDef(::std::boxed::Box::new(
                <PreprocFunctionDef as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "preproc_if" => Ok(Self::PreprocIf(::std::boxed::Box::new(
                <PreprocIf as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "preproc_ifdef" => Ok(Self::PreprocIfdef(::std::boxed::Box::new(
                <PreprocIfdef as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "preproc_include" => Ok(Self::PreprocInclude(::std::boxed::Box::new(
                <PreprocInclude as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "static_assert_declaration" => {
                Ok(Self::StaticAssertDeclaration(::std::boxed::Box::new(
                    <StaticAssertDeclaration as ::treesitter_types::FromNode>::from_node(
                        node, src,
                    )?,
                )))
            }
            "template_declaration" => Ok(Self::TemplateDeclaration(::std::boxed::Box::new(
                <TemplateDeclaration as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "template_instantiation" => Ok(Self::TemplateInstantiation(::std::boxed::Box::new(
                <TemplateInstantiation as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "type_definition" => Ok(Self::TypeDefinition(::std::boxed::Box::new(
                <TypeDefinition as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "using_declaration" => Ok(Self::UsingDeclaration(::std::boxed::Box::new(
                <UsingDeclaration as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            _other => {
                if let Ok(v) = <Statement as ::treesitter_types::FromNode>::from_node(node, src) {
                    Ok(Self::Statement(::std::boxed::Box::new(v)))
                } else {
                    if let Ok(v) =
                        <TypeSpecifier as ::treesitter_types::FromNode>::from_node(node, src)
                    {
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
            Self::AliasDeclaration(inner) => inner.span(),
            Self::ConceptDefinition(inner) => inner.span(),
            Self::ConstevalBlockDeclaration(inner) => inner.span(),
            Self::Declaration(inner) => inner.span(),
            Self::ExportDeclaration(inner) => inner.span(),
            Self::FunctionDefinition(inner) => inner.span(),
            Self::ImportDeclaration(inner) => inner.span(),
            Self::LinkageSpecification(inner) => inner.span(),
            Self::NamespaceAliasDefinition(inner) => inner.span(),
            Self::NamespaceDefinition(inner) => inner.span(),
            Self::PreprocCall(inner) => inner.span(),
            Self::PreprocDef(inner) => inner.span(),
            Self::PreprocFunctionDef(inner) => inner.span(),
            Self::PreprocIf(inner) => inner.span(),
            Self::PreprocIfdef(inner) => inner.span(),
            Self::PreprocInclude(inner) => inner.span(),
            Self::Statement(inner) => inner.span(),
            Self::StaticAssertDeclaration(inner) => inner.span(),
            Self::TemplateDeclaration(inner) => inner.span(),
            Self::TemplateInstantiation(inner) => inner.span(),
            Self::TypeDefinition(inner) => inner.span(),
            Self::TypeSpecifier(inner) => inner.span(),
            Self::UsingDeclaration(inner) => inner.span(),
        }
    }
}
#[derive(Debug, Clone)]
pub enum ConcatenatedStringChildren<'tree> {
    Identifier(::std::boxed::Box<Identifier<'tree>>),
    RawStringLiteral(::std::boxed::Box<RawStringLiteral<'tree>>),
    StringLiteral(::std::boxed::Box<StringLiteral<'tree>>),
}
impl<'tree> ::treesitter_types::FromNode<'tree> for ConcatenatedStringChildren<'tree> {
    #[allow(clippy::collapsible_else_if)]
    fn from_node(
        node: ::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        match node.kind() {
            "identifier" => Ok(Self::Identifier(::std::boxed::Box::new(
                <Identifier as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "raw_string_literal" => Ok(Self::RawStringLiteral(::std::boxed::Box::new(
                <RawStringLiteral as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "string_literal" => Ok(Self::StringLiteral(::std::boxed::Box::new(
                <StringLiteral as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            other => Err(::treesitter_types::ParseError::unexpected_kind(other, node)),
        }
    }
}
impl ::treesitter_types::Spanned for ConcatenatedStringChildren<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        match self {
            Self::Identifier(inner) => inner.span(),
            Self::RawStringLiteral(inner) => inner.span(),
            Self::StringLiteral(inner) => inner.span(),
        }
    }
}
#[derive(Debug, Clone)]
pub enum ConditionClauseValue<'tree> {
    CommaExpression(::std::boxed::Box<CommaExpression<'tree>>),
    Declaration(::std::boxed::Box<Declaration<'tree>>),
    Expression(::std::boxed::Box<Expression<'tree>>),
}
impl<'tree> ::treesitter_types::FromNode<'tree> for ConditionClauseValue<'tree> {
    #[allow(clippy::collapsible_else_if)]
    fn from_node(
        node: ::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        match node.kind() {
            "comma_expression" => Ok(Self::CommaExpression(::std::boxed::Box::new(
                <CommaExpression as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "declaration" => Ok(Self::Declaration(::std::boxed::Box::new(
                <Declaration as ::treesitter_types::FromNode>::from_node(node, src)?,
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
impl ::treesitter_types::Spanned for ConditionClauseValue<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        match self {
            Self::CommaExpression(inner) => inner.span(),
            Self::Declaration(inner) => inner.span(),
            Self::Expression(inner) => inner.span(),
        }
    }
}
#[derive(Debug, Clone)]
pub enum ConditionalExpressionConsequence<'tree> {
    CommaExpression(::std::boxed::Box<CommaExpression<'tree>>),
    Expression(::std::boxed::Box<Expression<'tree>>),
}
impl<'tree> ::treesitter_types::FromNode<'tree> for ConditionalExpressionConsequence<'tree> {
    #[allow(clippy::collapsible_else_if)]
    fn from_node(
        node: ::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        match node.kind() {
            "comma_expression" => Ok(Self::CommaExpression(::std::boxed::Box::new(
                <CommaExpression as ::treesitter_types::FromNode>::from_node(node, src)?,
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
impl ::treesitter_types::Spanned for ConditionalExpressionConsequence<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        match self {
            Self::CommaExpression(inner) => inner.span(),
            Self::Expression(inner) => inner.span(),
        }
    }
}
#[derive(Debug, Clone)]
pub enum ConstraintConjunctionLeft<'tree> {
    LParen(::treesitter_types::Span),
    RParen(::treesitter_types::Span),
    ConstraintConjunction(::std::boxed::Box<ConstraintConjunction<'tree>>),
    ConstraintDisjunction(::std::boxed::Box<ConstraintDisjunction<'tree>>),
    Expression(::std::boxed::Box<Expression<'tree>>),
    SpliceTypeSpecifier(::std::boxed::Box<SpliceTypeSpecifier<'tree>>),
    TemplateType(::std::boxed::Box<TemplateType<'tree>>),
    TypeIdentifier(::std::boxed::Box<TypeIdentifier<'tree>>),
}
impl<'tree> ::treesitter_types::FromNode<'tree> for ConstraintConjunctionLeft<'tree> {
    #[allow(clippy::collapsible_else_if)]
    fn from_node(
        node: ::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        match node.kind() {
            "(" => Ok(Self::LParen(::treesitter_types::Span::from(node))),
            ")" => Ok(Self::RParen(::treesitter_types::Span::from(node))),
            "constraint_conjunction" => Ok(Self::ConstraintConjunction(::std::boxed::Box::new(
                <ConstraintConjunction as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "constraint_disjunction" => Ok(Self::ConstraintDisjunction(::std::boxed::Box::new(
                <ConstraintDisjunction as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "splice_type_specifier" => Ok(Self::SpliceTypeSpecifier(::std::boxed::Box::new(
                <SpliceTypeSpecifier as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "template_type" => Ok(Self::TemplateType(::std::boxed::Box::new(
                <TemplateType as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "type_identifier" => Ok(Self::TypeIdentifier(::std::boxed::Box::new(
                <TypeIdentifier as ::treesitter_types::FromNode>::from_node(node, src)?,
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
impl ::treesitter_types::Spanned for ConstraintConjunctionLeft<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        match self {
            Self::LParen(span) => *span,
            Self::RParen(span) => *span,
            Self::ConstraintConjunction(inner) => inner.span(),
            Self::ConstraintDisjunction(inner) => inner.span(),
            Self::Expression(inner) => inner.span(),
            Self::SpliceTypeSpecifier(inner) => inner.span(),
            Self::TemplateType(inner) => inner.span(),
            Self::TypeIdentifier(inner) => inner.span(),
        }
    }
}
#[derive(Debug, Clone)]
pub enum ConstraintConjunctionOperator {
    AmpAmp(::treesitter_types::Span),
    And(::treesitter_types::Span),
}
impl<'tree> ::treesitter_types::FromNode<'tree> for ConstraintConjunctionOperator {
    #[allow(clippy::collapsible_else_if)]
    fn from_node(
        node: ::tree_sitter::Node<'tree>,
        _src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        match node.kind() {
            "&&" => Ok(Self::AmpAmp(::treesitter_types::Span::from(node))),
            "and" => Ok(Self::And(::treesitter_types::Span::from(node))),
            other => Err(::treesitter_types::ParseError::unexpected_kind(other, node)),
        }
    }
}
impl ::treesitter_types::Spanned for ConstraintConjunctionOperator {
    fn span(&self) -> ::treesitter_types::Span {
        match self {
            Self::AmpAmp(span) => *span,
            Self::And(span) => *span,
        }
    }
}
#[derive(Debug, Clone)]
pub enum ConstraintConjunctionRight<'tree> {
    LParen(::treesitter_types::Span),
    RParen(::treesitter_types::Span),
    ConstraintConjunction(::std::boxed::Box<ConstraintConjunction<'tree>>),
    ConstraintDisjunction(::std::boxed::Box<ConstraintDisjunction<'tree>>),
    Expression(::std::boxed::Box<Expression<'tree>>),
    SpliceTypeSpecifier(::std::boxed::Box<SpliceTypeSpecifier<'tree>>),
    TemplateType(::std::boxed::Box<TemplateType<'tree>>),
    TypeIdentifier(::std::boxed::Box<TypeIdentifier<'tree>>),
}
impl<'tree> ::treesitter_types::FromNode<'tree> for ConstraintConjunctionRight<'tree> {
    #[allow(clippy::collapsible_else_if)]
    fn from_node(
        node: ::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        match node.kind() {
            "(" => Ok(Self::LParen(::treesitter_types::Span::from(node))),
            ")" => Ok(Self::RParen(::treesitter_types::Span::from(node))),
            "constraint_conjunction" => Ok(Self::ConstraintConjunction(::std::boxed::Box::new(
                <ConstraintConjunction as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "constraint_disjunction" => Ok(Self::ConstraintDisjunction(::std::boxed::Box::new(
                <ConstraintDisjunction as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "splice_type_specifier" => Ok(Self::SpliceTypeSpecifier(::std::boxed::Box::new(
                <SpliceTypeSpecifier as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "template_type" => Ok(Self::TemplateType(::std::boxed::Box::new(
                <TemplateType as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "type_identifier" => Ok(Self::TypeIdentifier(::std::boxed::Box::new(
                <TypeIdentifier as ::treesitter_types::FromNode>::from_node(node, src)?,
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
impl ::treesitter_types::Spanned for ConstraintConjunctionRight<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        match self {
            Self::LParen(span) => *span,
            Self::RParen(span) => *span,
            Self::ConstraintConjunction(inner) => inner.span(),
            Self::ConstraintDisjunction(inner) => inner.span(),
            Self::Expression(inner) => inner.span(),
            Self::SpliceTypeSpecifier(inner) => inner.span(),
            Self::TemplateType(inner) => inner.span(),
            Self::TypeIdentifier(inner) => inner.span(),
        }
    }
}
#[derive(Debug, Clone)]
pub enum ConstraintDisjunctionLeft<'tree> {
    LParen(::treesitter_types::Span),
    RParen(::treesitter_types::Span),
    ConstraintConjunction(::std::boxed::Box<ConstraintConjunction<'tree>>),
    ConstraintDisjunction(::std::boxed::Box<ConstraintDisjunction<'tree>>),
    Expression(::std::boxed::Box<Expression<'tree>>),
    SpliceTypeSpecifier(::std::boxed::Box<SpliceTypeSpecifier<'tree>>),
    TemplateType(::std::boxed::Box<TemplateType<'tree>>),
    TypeIdentifier(::std::boxed::Box<TypeIdentifier<'tree>>),
}
impl<'tree> ::treesitter_types::FromNode<'tree> for ConstraintDisjunctionLeft<'tree> {
    #[allow(clippy::collapsible_else_if)]
    fn from_node(
        node: ::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        match node.kind() {
            "(" => Ok(Self::LParen(::treesitter_types::Span::from(node))),
            ")" => Ok(Self::RParen(::treesitter_types::Span::from(node))),
            "constraint_conjunction" => Ok(Self::ConstraintConjunction(::std::boxed::Box::new(
                <ConstraintConjunction as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "constraint_disjunction" => Ok(Self::ConstraintDisjunction(::std::boxed::Box::new(
                <ConstraintDisjunction as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "splice_type_specifier" => Ok(Self::SpliceTypeSpecifier(::std::boxed::Box::new(
                <SpliceTypeSpecifier as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "template_type" => Ok(Self::TemplateType(::std::boxed::Box::new(
                <TemplateType as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "type_identifier" => Ok(Self::TypeIdentifier(::std::boxed::Box::new(
                <TypeIdentifier as ::treesitter_types::FromNode>::from_node(node, src)?,
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
impl ::treesitter_types::Spanned for ConstraintDisjunctionLeft<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        match self {
            Self::LParen(span) => *span,
            Self::RParen(span) => *span,
            Self::ConstraintConjunction(inner) => inner.span(),
            Self::ConstraintDisjunction(inner) => inner.span(),
            Self::Expression(inner) => inner.span(),
            Self::SpliceTypeSpecifier(inner) => inner.span(),
            Self::TemplateType(inner) => inner.span(),
            Self::TypeIdentifier(inner) => inner.span(),
        }
    }
}
#[derive(Debug, Clone)]
pub enum ConstraintDisjunctionOperator {
    Or(::treesitter_types::Span),
    PipePipe(::treesitter_types::Span),
}
impl<'tree> ::treesitter_types::FromNode<'tree> for ConstraintDisjunctionOperator {
    #[allow(clippy::collapsible_else_if)]
    fn from_node(
        node: ::tree_sitter::Node<'tree>,
        _src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        match node.kind() {
            "or" => Ok(Self::Or(::treesitter_types::Span::from(node))),
            "||" => Ok(Self::PipePipe(::treesitter_types::Span::from(node))),
            other => Err(::treesitter_types::ParseError::unexpected_kind(other, node)),
        }
    }
}
impl ::treesitter_types::Spanned for ConstraintDisjunctionOperator {
    fn span(&self) -> ::treesitter_types::Span {
        match self {
            Self::Or(span) => *span,
            Self::PipePipe(span) => *span,
        }
    }
}
#[derive(Debug, Clone)]
pub enum ConstraintDisjunctionRight<'tree> {
    LParen(::treesitter_types::Span),
    RParen(::treesitter_types::Span),
    ConstraintConjunction(::std::boxed::Box<ConstraintConjunction<'tree>>),
    ConstraintDisjunction(::std::boxed::Box<ConstraintDisjunction<'tree>>),
    Expression(::std::boxed::Box<Expression<'tree>>),
    SpliceTypeSpecifier(::std::boxed::Box<SpliceTypeSpecifier<'tree>>),
    TemplateType(::std::boxed::Box<TemplateType<'tree>>),
    TypeIdentifier(::std::boxed::Box<TypeIdentifier<'tree>>),
}
impl<'tree> ::treesitter_types::FromNode<'tree> for ConstraintDisjunctionRight<'tree> {
    #[allow(clippy::collapsible_else_if)]
    fn from_node(
        node: ::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        match node.kind() {
            "(" => Ok(Self::LParen(::treesitter_types::Span::from(node))),
            ")" => Ok(Self::RParen(::treesitter_types::Span::from(node))),
            "constraint_conjunction" => Ok(Self::ConstraintConjunction(::std::boxed::Box::new(
                <ConstraintConjunction as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "constraint_disjunction" => Ok(Self::ConstraintDisjunction(::std::boxed::Box::new(
                <ConstraintDisjunction as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "splice_type_specifier" => Ok(Self::SpliceTypeSpecifier(::std::boxed::Box::new(
                <SpliceTypeSpecifier as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "template_type" => Ok(Self::TemplateType(::std::boxed::Box::new(
                <TemplateType as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "type_identifier" => Ok(Self::TypeIdentifier(::std::boxed::Box::new(
                <TypeIdentifier as ::treesitter_types::FromNode>::from_node(node, src)?,
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
impl ::treesitter_types::Spanned for ConstraintDisjunctionRight<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        match self {
            Self::LParen(span) => *span,
            Self::RParen(span) => *span,
            Self::ConstraintConjunction(inner) => inner.span(),
            Self::ConstraintDisjunction(inner) => inner.span(),
            Self::Expression(inner) => inner.span(),
            Self::SpliceTypeSpecifier(inner) => inner.span(),
            Self::TemplateType(inner) => inner.span(),
            Self::TypeIdentifier(inner) => inner.span(),
        }
    }
}
#[derive(Debug, Clone)]
pub enum DeclarationDeclarator<'tree> {
    Declarator(::std::boxed::Box<Declarator<'tree>>),
    GnuAsmExpression(::std::boxed::Box<GnuAsmExpression<'tree>>),
    InitDeclarator(::std::boxed::Box<InitDeclarator<'tree>>),
    MsCallModifier(::std::boxed::Box<MsCallModifier<'tree>>),
    OperatorCast(::std::boxed::Box<OperatorCast<'tree>>),
}
impl<'tree> ::treesitter_types::FromNode<'tree> for DeclarationDeclarator<'tree> {
    #[allow(clippy::collapsible_else_if)]
    fn from_node(
        node: ::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        match node.kind() {
            "gnu_asm_expression" => Ok(Self::GnuAsmExpression(::std::boxed::Box::new(
                <GnuAsmExpression as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "init_declarator" => Ok(Self::InitDeclarator(::std::boxed::Box::new(
                <InitDeclarator as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "ms_call_modifier" => Ok(Self::MsCallModifier(::std::boxed::Box::new(
                <MsCallModifier as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "operator_cast" => Ok(Self::OperatorCast(::std::boxed::Box::new(
                <OperatorCast as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            _other => {
                if let Ok(v) = <Declarator as ::treesitter_types::FromNode>::from_node(node, src) {
                    Ok(Self::Declarator(::std::boxed::Box::new(v)))
                } else {
                    Err(::treesitter_types::ParseError::unexpected_kind(
                        _other, node,
                    ))
                }
            }
        }
    }
}
impl ::treesitter_types::Spanned for DeclarationDeclarator<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        match self {
            Self::Declarator(inner) => inner.span(),
            Self::GnuAsmExpression(inner) => inner.span(),
            Self::InitDeclarator(inner) => inner.span(),
            Self::MsCallModifier(inner) => inner.span(),
            Self::OperatorCast(inner) => inner.span(),
        }
    }
}
#[derive(Debug, Clone)]
pub enum DeclarationValue<'tree> {
    Expression(::std::boxed::Box<Expression<'tree>>),
    InitializerList(::std::boxed::Box<InitializerList<'tree>>),
}
impl<'tree> ::treesitter_types::FromNode<'tree> for DeclarationValue<'tree> {
    #[allow(clippy::collapsible_else_if)]
    fn from_node(
        node: ::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        match node.kind() {
            "initializer_list" => Ok(Self::InitializerList(::std::boxed::Box::new(
                <InitializerList as ::treesitter_types::FromNode>::from_node(node, src)?,
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
impl ::treesitter_types::Spanned for DeclarationValue<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        match self {
            Self::Expression(inner) => inner.span(),
            Self::InitializerList(inner) => inner.span(),
        }
    }
}
#[derive(Debug, Clone)]
pub enum DeclarationChildren<'tree> {
    AttributeDeclaration(::std::boxed::Box<AttributeDeclaration<'tree>>),
    AttributeSpecifier(::std::boxed::Box<AttributeSpecifier<'tree>>),
    ExplicitFunctionSpecifier(::std::boxed::Box<ExplicitFunctionSpecifier<'tree>>),
    MsDeclspecModifier(::std::boxed::Box<MsDeclspecModifier<'tree>>),
    StorageClassSpecifier(::std::boxed::Box<StorageClassSpecifier<'tree>>),
    TypeQualifier(::std::boxed::Box<TypeQualifier<'tree>>),
}
impl<'tree> ::treesitter_types::FromNode<'tree> for DeclarationChildren<'tree> {
    #[allow(clippy::collapsible_else_if)]
    fn from_node(
        node: ::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        match node.kind() {
            "attribute_declaration" => Ok(Self::AttributeDeclaration(::std::boxed::Box::new(
                <AttributeDeclaration as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "attribute_specifier" => Ok(Self::AttributeSpecifier(::std::boxed::Box::new(
                <AttributeSpecifier as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "explicit_function_specifier" => {
                Ok(Self::ExplicitFunctionSpecifier(::std::boxed::Box::new(
                    <ExplicitFunctionSpecifier as ::treesitter_types::FromNode>::from_node(
                        node, src,
                    )?,
                )))
            }
            "ms_declspec_modifier" => Ok(Self::MsDeclspecModifier(::std::boxed::Box::new(
                <MsDeclspecModifier as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "storage_class_specifier" => Ok(Self::StorageClassSpecifier(::std::boxed::Box::new(
                <StorageClassSpecifier as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "type_qualifier" => Ok(Self::TypeQualifier(::std::boxed::Box::new(
                <TypeQualifier as ::treesitter_types::FromNode>::from_node(node, src)?,
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
            Self::ExplicitFunctionSpecifier(inner) => inner.span(),
            Self::MsDeclspecModifier(inner) => inner.span(),
            Self::StorageClassSpecifier(inner) => inner.span(),
            Self::TypeQualifier(inner) => inner.span(),
        }
    }
}
#[derive(Debug, Clone)]
pub enum DeclarationListChildren<'tree> {
    AliasDeclaration(::std::boxed::Box<AliasDeclaration<'tree>>),
    ConceptDefinition(::std::boxed::Box<ConceptDefinition<'tree>>),
    ConstevalBlockDeclaration(::std::boxed::Box<ConstevalBlockDeclaration<'tree>>),
    Declaration(::std::boxed::Box<Declaration<'tree>>),
    ExportDeclaration(::std::boxed::Box<ExportDeclaration<'tree>>),
    FunctionDefinition(::std::boxed::Box<FunctionDefinition<'tree>>),
    ImportDeclaration(::std::boxed::Box<ImportDeclaration<'tree>>),
    LinkageSpecification(::std::boxed::Box<LinkageSpecification<'tree>>),
    NamespaceAliasDefinition(::std::boxed::Box<NamespaceAliasDefinition<'tree>>),
    NamespaceDefinition(::std::boxed::Box<NamespaceDefinition<'tree>>),
    PreprocCall(::std::boxed::Box<PreprocCall<'tree>>),
    PreprocDef(::std::boxed::Box<PreprocDef<'tree>>),
    PreprocFunctionDef(::std::boxed::Box<PreprocFunctionDef<'tree>>),
    PreprocIf(::std::boxed::Box<PreprocIf<'tree>>),
    PreprocIfdef(::std::boxed::Box<PreprocIfdef<'tree>>),
    PreprocInclude(::std::boxed::Box<PreprocInclude<'tree>>),
    Statement(::std::boxed::Box<Statement<'tree>>),
    StaticAssertDeclaration(::std::boxed::Box<StaticAssertDeclaration<'tree>>),
    TemplateDeclaration(::std::boxed::Box<TemplateDeclaration<'tree>>),
    TemplateInstantiation(::std::boxed::Box<TemplateInstantiation<'tree>>),
    TypeDefinition(::std::boxed::Box<TypeDefinition<'tree>>),
    TypeSpecifier(::std::boxed::Box<TypeSpecifier<'tree>>),
    UsingDeclaration(::std::boxed::Box<UsingDeclaration<'tree>>),
}
impl<'tree> ::treesitter_types::FromNode<'tree> for DeclarationListChildren<'tree> {
    #[allow(clippy::collapsible_else_if)]
    fn from_node(
        node: ::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        match node.kind() {
            "alias_declaration" => Ok(Self::AliasDeclaration(::std::boxed::Box::new(
                <AliasDeclaration as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "concept_definition" => Ok(Self::ConceptDefinition(::std::boxed::Box::new(
                <ConceptDefinition as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "consteval_block_declaration" => {
                Ok(Self::ConstevalBlockDeclaration(::std::boxed::Box::new(
                    <ConstevalBlockDeclaration as ::treesitter_types::FromNode>::from_node(
                        node, src,
                    )?,
                )))
            }
            "declaration" => Ok(Self::Declaration(::std::boxed::Box::new(
                <Declaration as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "export_declaration" => Ok(Self::ExportDeclaration(::std::boxed::Box::new(
                <ExportDeclaration as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "function_definition" => Ok(Self::FunctionDefinition(::std::boxed::Box::new(
                <FunctionDefinition as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "import_declaration" => Ok(Self::ImportDeclaration(::std::boxed::Box::new(
                <ImportDeclaration as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "linkage_specification" => Ok(Self::LinkageSpecification(::std::boxed::Box::new(
                <LinkageSpecification as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "namespace_alias_definition" => {
                Ok(Self::NamespaceAliasDefinition(::std::boxed::Box::new(
                    <NamespaceAliasDefinition as ::treesitter_types::FromNode>::from_node(
                        node, src,
                    )?,
                )))
            }
            "namespace_definition" => Ok(Self::NamespaceDefinition(::std::boxed::Box::new(
                <NamespaceDefinition as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "preproc_call" => Ok(Self::PreprocCall(::std::boxed::Box::new(
                <PreprocCall as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "preproc_def" => Ok(Self::PreprocDef(::std::boxed::Box::new(
                <PreprocDef as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "preproc_function_def" => Ok(Self::PreprocFunctionDef(::std::boxed::Box::new(
                <PreprocFunctionDef as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "preproc_if" => Ok(Self::PreprocIf(::std::boxed::Box::new(
                <PreprocIf as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "preproc_ifdef" => Ok(Self::PreprocIfdef(::std::boxed::Box::new(
                <PreprocIfdef as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "preproc_include" => Ok(Self::PreprocInclude(::std::boxed::Box::new(
                <PreprocInclude as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "static_assert_declaration" => {
                Ok(Self::StaticAssertDeclaration(::std::boxed::Box::new(
                    <StaticAssertDeclaration as ::treesitter_types::FromNode>::from_node(
                        node, src,
                    )?,
                )))
            }
            "template_declaration" => Ok(Self::TemplateDeclaration(::std::boxed::Box::new(
                <TemplateDeclaration as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "template_instantiation" => Ok(Self::TemplateInstantiation(::std::boxed::Box::new(
                <TemplateInstantiation as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "type_definition" => Ok(Self::TypeDefinition(::std::boxed::Box::new(
                <TypeDefinition as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "using_declaration" => Ok(Self::UsingDeclaration(::std::boxed::Box::new(
                <UsingDeclaration as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            _other => {
                if let Ok(v) = <Statement as ::treesitter_types::FromNode>::from_node(node, src) {
                    Ok(Self::Statement(::std::boxed::Box::new(v)))
                } else {
                    if let Ok(v) =
                        <TypeSpecifier as ::treesitter_types::FromNode>::from_node(node, src)
                    {
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
            Self::AliasDeclaration(inner) => inner.span(),
            Self::ConceptDefinition(inner) => inner.span(),
            Self::ConstevalBlockDeclaration(inner) => inner.span(),
            Self::Declaration(inner) => inner.span(),
            Self::ExportDeclaration(inner) => inner.span(),
            Self::FunctionDefinition(inner) => inner.span(),
            Self::ImportDeclaration(inner) => inner.span(),
            Self::LinkageSpecification(inner) => inner.span(),
            Self::NamespaceAliasDefinition(inner) => inner.span(),
            Self::NamespaceDefinition(inner) => inner.span(),
            Self::PreprocCall(inner) => inner.span(),
            Self::PreprocDef(inner) => inner.span(),
            Self::PreprocFunctionDef(inner) => inner.span(),
            Self::PreprocIf(inner) => inner.span(),
            Self::PreprocIfdef(inner) => inner.span(),
            Self::PreprocInclude(inner) => inner.span(),
            Self::Statement(inner) => inner.span(),
            Self::StaticAssertDeclaration(inner) => inner.span(),
            Self::TemplateDeclaration(inner) => inner.span(),
            Self::TemplateInstantiation(inner) => inner.span(),
            Self::TypeDefinition(inner) => inner.span(),
            Self::TypeSpecifier(inner) => inner.span(),
            Self::UsingDeclaration(inner) => inner.span(),
        }
    }
}
#[derive(Debug, Clone)]
pub enum DecltypeChildren<'tree> {
    Auto(::std::boxed::Box<Auto<'tree>>),
    Expression(::std::boxed::Box<Expression<'tree>>),
}
impl<'tree> ::treesitter_types::FromNode<'tree> for DecltypeChildren<'tree> {
    #[allow(clippy::collapsible_else_if)]
    fn from_node(
        node: ::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        match node.kind() {
            "auto" => Ok(Self::Auto(::std::boxed::Box::new(
                <Auto as ::treesitter_types::FromNode>::from_node(node, src)?,
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
impl ::treesitter_types::Spanned for DecltypeChildren<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        match self {
            Self::Auto(inner) => inner.span(),
            Self::Expression(inner) => inner.span(),
        }
    }
}
#[derive(Debug, Clone)]
pub enum DependentNameChildren<'tree> {
    TemplateFunction(::std::boxed::Box<TemplateFunction<'tree>>),
    TemplateMethod(::std::boxed::Box<TemplateMethod<'tree>>),
    TemplateType(::std::boxed::Box<TemplateType<'tree>>),
}
impl<'tree> ::treesitter_types::FromNode<'tree> for DependentNameChildren<'tree> {
    #[allow(clippy::collapsible_else_if)]
    fn from_node(
        node: ::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        match node.kind() {
            "template_function" => Ok(Self::TemplateFunction(::std::boxed::Box::new(
                <TemplateFunction as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "template_method" => Ok(Self::TemplateMethod(::std::boxed::Box::new(
                <TemplateMethod as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "template_type" => Ok(Self::TemplateType(::std::boxed::Box::new(
                <TemplateType as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            other => Err(::treesitter_types::ParseError::unexpected_kind(other, node)),
        }
    }
}
impl ::treesitter_types::Spanned for DependentNameChildren<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        match self {
            Self::TemplateFunction(inner) => inner.span(),
            Self::TemplateMethod(inner) => inner.span(),
            Self::TemplateType(inner) => inner.span(),
        }
    }
}
#[derive(Debug, Clone)]
pub enum EnumSpecifierBase<'tree> {
    PrimitiveType(::std::boxed::Box<PrimitiveType<'tree>>),
    QualifiedIdentifier(::std::boxed::Box<QualifiedIdentifier<'tree>>),
    SizedTypeSpecifier(::std::boxed::Box<SizedTypeSpecifier<'tree>>),
    TypeIdentifier(::std::boxed::Box<TypeIdentifier<'tree>>),
}
impl<'tree> ::treesitter_types::FromNode<'tree> for EnumSpecifierBase<'tree> {
    #[allow(clippy::collapsible_else_if)]
    fn from_node(
        node: ::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        match node.kind() {
            "primitive_type" => Ok(Self::PrimitiveType(::std::boxed::Box::new(
                <PrimitiveType as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "qualified_identifier" => Ok(Self::QualifiedIdentifier(::std::boxed::Box::new(
                <QualifiedIdentifier as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "sized_type_specifier" => Ok(Self::SizedTypeSpecifier(::std::boxed::Box::new(
                <SizedTypeSpecifier as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "type_identifier" => Ok(Self::TypeIdentifier(::std::boxed::Box::new(
                <TypeIdentifier as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            other => Err(::treesitter_types::ParseError::unexpected_kind(other, node)),
        }
    }
}
impl ::treesitter_types::Spanned for EnumSpecifierBase<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        match self {
            Self::PrimitiveType(inner) => inner.span(),
            Self::QualifiedIdentifier(inner) => inner.span(),
            Self::SizedTypeSpecifier(inner) => inner.span(),
            Self::TypeIdentifier(inner) => inner.span(),
        }
    }
}
#[derive(Debug, Clone)]
pub enum EnumSpecifierName<'tree> {
    QualifiedIdentifier(::std::boxed::Box<QualifiedIdentifier<'tree>>),
    SpliceTypeSpecifier(::std::boxed::Box<SpliceTypeSpecifier<'tree>>),
    TemplateType(::std::boxed::Box<TemplateType<'tree>>),
    TypeIdentifier(::std::boxed::Box<TypeIdentifier<'tree>>),
}
impl<'tree> ::treesitter_types::FromNode<'tree> for EnumSpecifierName<'tree> {
    #[allow(clippy::collapsible_else_if)]
    fn from_node(
        node: ::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        match node.kind() {
            "qualified_identifier" => Ok(Self::QualifiedIdentifier(::std::boxed::Box::new(
                <QualifiedIdentifier as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "splice_type_specifier" => Ok(Self::SpliceTypeSpecifier(::std::boxed::Box::new(
                <SpliceTypeSpecifier as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "template_type" => Ok(Self::TemplateType(::std::boxed::Box::new(
                <TemplateType as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "type_identifier" => Ok(Self::TypeIdentifier(::std::boxed::Box::new(
                <TypeIdentifier as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            other => Err(::treesitter_types::ParseError::unexpected_kind(other, node)),
        }
    }
}
impl ::treesitter_types::Spanned for EnumSpecifierName<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        match self {
            Self::QualifiedIdentifier(inner) => inner.span(),
            Self::SpliceTypeSpecifier(inner) => inner.span(),
            Self::TemplateType(inner) => inner.span(),
            Self::TypeIdentifier(inner) => inner.span(),
        }
    }
}
#[derive(Debug, Clone)]
pub enum EnumeratorListChildren<'tree> {
    Enumerator(::std::boxed::Box<Enumerator<'tree>>),
    PreprocCall(::std::boxed::Box<PreprocCall<'tree>>),
    PreprocIf(::std::boxed::Box<PreprocIf<'tree>>),
    PreprocIfdef(::std::boxed::Box<PreprocIfdef<'tree>>),
}
impl<'tree> ::treesitter_types::FromNode<'tree> for EnumeratorListChildren<'tree> {
    #[allow(clippy::collapsible_else_if)]
    fn from_node(
        node: ::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        match node.kind() {
            "enumerator" => Ok(Self::Enumerator(::std::boxed::Box::new(
                <Enumerator as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "preproc_call" => Ok(Self::PreprocCall(::std::boxed::Box::new(
                <PreprocCall as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "preproc_if" => Ok(Self::PreprocIf(::std::boxed::Box::new(
                <PreprocIf as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "preproc_ifdef" => Ok(Self::PreprocIfdef(::std::boxed::Box::new(
                <PreprocIfdef as ::treesitter_types::FromNode>::from_node(node, src)?,
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
#[derive(Debug, Clone)]
pub enum ExpansionStatementRight<'tree> {
    Expression(::std::boxed::Box<Expression<'tree>>),
    InitializerList(::std::boxed::Box<InitializerList<'tree>>),
}
impl<'tree> ::treesitter_types::FromNode<'tree> for ExpansionStatementRight<'tree> {
    #[allow(clippy::collapsible_else_if)]
    fn from_node(
        node: ::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        match node.kind() {
            "initializer_list" => Ok(Self::InitializerList(::std::boxed::Box::new(
                <InitializerList as ::treesitter_types::FromNode>::from_node(node, src)?,
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
impl ::treesitter_types::Spanned for ExpansionStatementRight<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        match self {
            Self::Expression(inner) => inner.span(),
            Self::InitializerList(inner) => inner.span(),
        }
    }
}
#[derive(Debug, Clone)]
pub enum ExpansionStatementChildren<'tree> {
    AttributeDeclaration(::std::boxed::Box<AttributeDeclaration<'tree>>),
    AttributeSpecifier(::std::boxed::Box<AttributeSpecifier<'tree>>),
    MsDeclspecModifier(::std::boxed::Box<MsDeclspecModifier<'tree>>),
    StorageClassSpecifier(::std::boxed::Box<StorageClassSpecifier<'tree>>),
    TypeQualifier(::std::boxed::Box<TypeQualifier<'tree>>),
}
impl<'tree> ::treesitter_types::FromNode<'tree> for ExpansionStatementChildren<'tree> {
    #[allow(clippy::collapsible_else_if)]
    fn from_node(
        node: ::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        match node.kind() {
            "attribute_declaration" => Ok(Self::AttributeDeclaration(::std::boxed::Box::new(
                <AttributeDeclaration as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "attribute_specifier" => Ok(Self::AttributeSpecifier(::std::boxed::Box::new(
                <AttributeSpecifier as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "ms_declspec_modifier" => Ok(Self::MsDeclspecModifier(::std::boxed::Box::new(
                <MsDeclspecModifier as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "storage_class_specifier" => Ok(Self::StorageClassSpecifier(::std::boxed::Box::new(
                <StorageClassSpecifier as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "type_qualifier" => Ok(Self::TypeQualifier(::std::boxed::Box::new(
                <TypeQualifier as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            other => Err(::treesitter_types::ParseError::unexpected_kind(other, node)),
        }
    }
}
impl ::treesitter_types::Spanned for ExpansionStatementChildren<'_> {
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
#[derive(Debug, Clone)]
pub enum ExplicitObjectParameterDeclarationChildren<'tree> {
    ParameterDeclaration(::std::boxed::Box<ParameterDeclaration<'tree>>),
    This(::std::boxed::Box<This<'tree>>),
}
impl<'tree> ::treesitter_types::FromNode<'tree>
    for ExplicitObjectParameterDeclarationChildren<'tree>
{
    #[allow(clippy::collapsible_else_if)]
    fn from_node(
        node: ::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        match node.kind() {
            "parameter_declaration" => Ok(Self::ParameterDeclaration(::std::boxed::Box::new(
                <ParameterDeclaration as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "this" => Ok(Self::This(::std::boxed::Box::new(
                <This as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            other => Err(::treesitter_types::ParseError::unexpected_kind(other, node)),
        }
    }
}
impl ::treesitter_types::Spanned for ExplicitObjectParameterDeclarationChildren<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        match self {
            Self::ParameterDeclaration(inner) => inner.span(),
            Self::This(inner) => inner.span(),
        }
    }
}
#[derive(Debug, Clone)]
pub enum ExportDeclarationChildren<'tree> {
    AliasDeclaration(::std::boxed::Box<AliasDeclaration<'tree>>),
    ConceptDefinition(::std::boxed::Box<ConceptDefinition<'tree>>),
    ConstevalBlockDeclaration(::std::boxed::Box<ConstevalBlockDeclaration<'tree>>),
    Declaration(::std::boxed::Box<Declaration<'tree>>),
    DeclarationList(::std::boxed::Box<DeclarationList<'tree>>),
    ExportDeclaration(::std::boxed::Box<ExportDeclaration<'tree>>),
    FunctionDefinition(::std::boxed::Box<FunctionDefinition<'tree>>),
    ImportDeclaration(::std::boxed::Box<ImportDeclaration<'tree>>),
    LinkageSpecification(::std::boxed::Box<LinkageSpecification<'tree>>),
    NamespaceAliasDefinition(::std::boxed::Box<NamespaceAliasDefinition<'tree>>),
    NamespaceDefinition(::std::boxed::Box<NamespaceDefinition<'tree>>),
    PreprocCall(::std::boxed::Box<PreprocCall<'tree>>),
    PreprocDef(::std::boxed::Box<PreprocDef<'tree>>),
    PreprocFunctionDef(::std::boxed::Box<PreprocFunctionDef<'tree>>),
    PreprocIf(::std::boxed::Box<PreprocIf<'tree>>),
    PreprocIfdef(::std::boxed::Box<PreprocIfdef<'tree>>),
    PreprocInclude(::std::boxed::Box<PreprocInclude<'tree>>),
    Statement(::std::boxed::Box<Statement<'tree>>),
    StaticAssertDeclaration(::std::boxed::Box<StaticAssertDeclaration<'tree>>),
    TemplateDeclaration(::std::boxed::Box<TemplateDeclaration<'tree>>),
    TemplateInstantiation(::std::boxed::Box<TemplateInstantiation<'tree>>),
    TypeDefinition(::std::boxed::Box<TypeDefinition<'tree>>),
    TypeSpecifier(::std::boxed::Box<TypeSpecifier<'tree>>),
    UsingDeclaration(::std::boxed::Box<UsingDeclaration<'tree>>),
}
impl<'tree> ::treesitter_types::FromNode<'tree> for ExportDeclarationChildren<'tree> {
    #[allow(clippy::collapsible_else_if)]
    fn from_node(
        node: ::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        match node.kind() {
            "alias_declaration" => Ok(Self::AliasDeclaration(::std::boxed::Box::new(
                <AliasDeclaration as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "concept_definition" => Ok(Self::ConceptDefinition(::std::boxed::Box::new(
                <ConceptDefinition as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "consteval_block_declaration" => {
                Ok(Self::ConstevalBlockDeclaration(::std::boxed::Box::new(
                    <ConstevalBlockDeclaration as ::treesitter_types::FromNode>::from_node(
                        node, src,
                    )?,
                )))
            }
            "declaration" => Ok(Self::Declaration(::std::boxed::Box::new(
                <Declaration as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "declaration_list" => Ok(Self::DeclarationList(::std::boxed::Box::new(
                <DeclarationList as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "export_declaration" => Ok(Self::ExportDeclaration(::std::boxed::Box::new(
                <ExportDeclaration as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "function_definition" => Ok(Self::FunctionDefinition(::std::boxed::Box::new(
                <FunctionDefinition as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "import_declaration" => Ok(Self::ImportDeclaration(::std::boxed::Box::new(
                <ImportDeclaration as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "linkage_specification" => Ok(Self::LinkageSpecification(::std::boxed::Box::new(
                <LinkageSpecification as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "namespace_alias_definition" => {
                Ok(Self::NamespaceAliasDefinition(::std::boxed::Box::new(
                    <NamespaceAliasDefinition as ::treesitter_types::FromNode>::from_node(
                        node, src,
                    )?,
                )))
            }
            "namespace_definition" => Ok(Self::NamespaceDefinition(::std::boxed::Box::new(
                <NamespaceDefinition as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "preproc_call" => Ok(Self::PreprocCall(::std::boxed::Box::new(
                <PreprocCall as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "preproc_def" => Ok(Self::PreprocDef(::std::boxed::Box::new(
                <PreprocDef as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "preproc_function_def" => Ok(Self::PreprocFunctionDef(::std::boxed::Box::new(
                <PreprocFunctionDef as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "preproc_if" => Ok(Self::PreprocIf(::std::boxed::Box::new(
                <PreprocIf as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "preproc_ifdef" => Ok(Self::PreprocIfdef(::std::boxed::Box::new(
                <PreprocIfdef as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "preproc_include" => Ok(Self::PreprocInclude(::std::boxed::Box::new(
                <PreprocInclude as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "static_assert_declaration" => {
                Ok(Self::StaticAssertDeclaration(::std::boxed::Box::new(
                    <StaticAssertDeclaration as ::treesitter_types::FromNode>::from_node(
                        node, src,
                    )?,
                )))
            }
            "template_declaration" => Ok(Self::TemplateDeclaration(::std::boxed::Box::new(
                <TemplateDeclaration as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "template_instantiation" => Ok(Self::TemplateInstantiation(::std::boxed::Box::new(
                <TemplateInstantiation as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "type_definition" => Ok(Self::TypeDefinition(::std::boxed::Box::new(
                <TypeDefinition as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "using_declaration" => Ok(Self::UsingDeclaration(::std::boxed::Box::new(
                <UsingDeclaration as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            _other => {
                if let Ok(v) = <Statement as ::treesitter_types::FromNode>::from_node(node, src) {
                    Ok(Self::Statement(::std::boxed::Box::new(v)))
                } else {
                    if let Ok(v) =
                        <TypeSpecifier as ::treesitter_types::FromNode>::from_node(node, src)
                    {
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
impl ::treesitter_types::Spanned for ExportDeclarationChildren<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        match self {
            Self::AliasDeclaration(inner) => inner.span(),
            Self::ConceptDefinition(inner) => inner.span(),
            Self::ConstevalBlockDeclaration(inner) => inner.span(),
            Self::Declaration(inner) => inner.span(),
            Self::DeclarationList(inner) => inner.span(),
            Self::ExportDeclaration(inner) => inner.span(),
            Self::FunctionDefinition(inner) => inner.span(),
            Self::ImportDeclaration(inner) => inner.span(),
            Self::LinkageSpecification(inner) => inner.span(),
            Self::NamespaceAliasDefinition(inner) => inner.span(),
            Self::NamespaceDefinition(inner) => inner.span(),
            Self::PreprocCall(inner) => inner.span(),
            Self::PreprocDef(inner) => inner.span(),
            Self::PreprocFunctionDef(inner) => inner.span(),
            Self::PreprocIf(inner) => inner.span(),
            Self::PreprocIfdef(inner) => inner.span(),
            Self::PreprocInclude(inner) => inner.span(),
            Self::Statement(inner) => inner.span(),
            Self::StaticAssertDeclaration(inner) => inner.span(),
            Self::TemplateDeclaration(inner) => inner.span(),
            Self::TemplateInstantiation(inner) => inner.span(),
            Self::TypeDefinition(inner) => inner.span(),
            Self::TypeSpecifier(inner) => inner.span(),
            Self::UsingDeclaration(inner) => inner.span(),
        }
    }
}
#[derive(Debug, Clone)]
pub enum ExpressionStatementChildren<'tree> {
    CommaExpression(::std::boxed::Box<CommaExpression<'tree>>),
    Expression(::std::boxed::Box<Expression<'tree>>),
}
impl<'tree> ::treesitter_types::FromNode<'tree> for ExpressionStatementChildren<'tree> {
    #[allow(clippy::collapsible_else_if)]
    fn from_node(
        node: ::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        match node.kind() {
            "comma_expression" => Ok(Self::CommaExpression(::std::boxed::Box::new(
                <CommaExpression as ::treesitter_types::FromNode>::from_node(node, src)?,
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
impl ::treesitter_types::Spanned for ExpressionStatementChildren<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        match self {
            Self::CommaExpression(inner) => inner.span(),
            Self::Expression(inner) => inner.span(),
        }
    }
}
#[derive(Debug, Clone)]
pub enum FieldDeclarationDefaultValue<'tree> {
    Expression(::std::boxed::Box<Expression<'tree>>),
    InitializerList(::std::boxed::Box<InitializerList<'tree>>),
}
impl<'tree> ::treesitter_types::FromNode<'tree> for FieldDeclarationDefaultValue<'tree> {
    #[allow(clippy::collapsible_else_if)]
    fn from_node(
        node: ::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        match node.kind() {
            "initializer_list" => Ok(Self::InitializerList(::std::boxed::Box::new(
                <InitializerList as ::treesitter_types::FromNode>::from_node(node, src)?,
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
impl ::treesitter_types::Spanned for FieldDeclarationDefaultValue<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        match self {
            Self::Expression(inner) => inner.span(),
            Self::InitializerList(inner) => inner.span(),
        }
    }
}
#[derive(Debug, Clone)]
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
        node: ::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        match node.kind() {
            "attribute_declaration" => Ok(Self::AttributeDeclaration(::std::boxed::Box::new(
                <AttributeDeclaration as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "attribute_specifier" => Ok(Self::AttributeSpecifier(::std::boxed::Box::new(
                <AttributeSpecifier as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "bitfield_clause" => Ok(Self::BitfieldClause(::std::boxed::Box::new(
                <BitfieldClause as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "ms_declspec_modifier" => Ok(Self::MsDeclspecModifier(::std::boxed::Box::new(
                <MsDeclspecModifier as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "storage_class_specifier" => Ok(Self::StorageClassSpecifier(::std::boxed::Box::new(
                <StorageClassSpecifier as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "type_qualifier" => Ok(Self::TypeQualifier(::std::boxed::Box::new(
                <TypeQualifier as ::treesitter_types::FromNode>::from_node(node, src)?,
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
#[derive(Debug, Clone)]
pub enum FieldDeclarationListChildren<'tree> {
    AccessSpecifier(::std::boxed::Box<AccessSpecifier<'tree>>),
    AliasDeclaration(::std::boxed::Box<AliasDeclaration<'tree>>),
    ConstevalBlockDeclaration(::std::boxed::Box<ConstevalBlockDeclaration<'tree>>),
    Declaration(::std::boxed::Box<Declaration<'tree>>),
    FieldDeclaration(::std::boxed::Box<FieldDeclaration<'tree>>),
    FriendDeclaration(::std::boxed::Box<FriendDeclaration<'tree>>),
    FunctionDefinition(::std::boxed::Box<FunctionDefinition<'tree>>),
    PreprocCall(::std::boxed::Box<PreprocCall<'tree>>),
    PreprocDef(::std::boxed::Box<PreprocDef<'tree>>),
    PreprocFunctionDef(::std::boxed::Box<PreprocFunctionDef<'tree>>),
    PreprocIf(::std::boxed::Box<PreprocIf<'tree>>),
    PreprocIfdef(::std::boxed::Box<PreprocIfdef<'tree>>),
    StaticAssertDeclaration(::std::boxed::Box<StaticAssertDeclaration<'tree>>),
    TemplateDeclaration(::std::boxed::Box<TemplateDeclaration<'tree>>),
    TypeDefinition(::std::boxed::Box<TypeDefinition<'tree>>),
    UsingDeclaration(::std::boxed::Box<UsingDeclaration<'tree>>),
}
impl<'tree> ::treesitter_types::FromNode<'tree> for FieldDeclarationListChildren<'tree> {
    #[allow(clippy::collapsible_else_if)]
    fn from_node(
        node: ::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        match node.kind() {
            "access_specifier" => Ok(Self::AccessSpecifier(::std::boxed::Box::new(
                <AccessSpecifier as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "alias_declaration" => Ok(Self::AliasDeclaration(::std::boxed::Box::new(
                <AliasDeclaration as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "consteval_block_declaration" => {
                Ok(Self::ConstevalBlockDeclaration(::std::boxed::Box::new(
                    <ConstevalBlockDeclaration as ::treesitter_types::FromNode>::from_node(
                        node, src,
                    )?,
                )))
            }
            "declaration" => Ok(Self::Declaration(::std::boxed::Box::new(
                <Declaration as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "field_declaration" => Ok(Self::FieldDeclaration(::std::boxed::Box::new(
                <FieldDeclaration as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "friend_declaration" => Ok(Self::FriendDeclaration(::std::boxed::Box::new(
                <FriendDeclaration as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "function_definition" => Ok(Self::FunctionDefinition(::std::boxed::Box::new(
                <FunctionDefinition as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "preproc_call" => Ok(Self::PreprocCall(::std::boxed::Box::new(
                <PreprocCall as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "preproc_def" => Ok(Self::PreprocDef(::std::boxed::Box::new(
                <PreprocDef as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "preproc_function_def" => Ok(Self::PreprocFunctionDef(::std::boxed::Box::new(
                <PreprocFunctionDef as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "preproc_if" => Ok(Self::PreprocIf(::std::boxed::Box::new(
                <PreprocIf as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "preproc_ifdef" => Ok(Self::PreprocIfdef(::std::boxed::Box::new(
                <PreprocIfdef as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "static_assert_declaration" => {
                Ok(Self::StaticAssertDeclaration(::std::boxed::Box::new(
                    <StaticAssertDeclaration as ::treesitter_types::FromNode>::from_node(
                        node, src,
                    )?,
                )))
            }
            "template_declaration" => Ok(Self::TemplateDeclaration(::std::boxed::Box::new(
                <TemplateDeclaration as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "type_definition" => Ok(Self::TypeDefinition(::std::boxed::Box::new(
                <TypeDefinition as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "using_declaration" => Ok(Self::UsingDeclaration(::std::boxed::Box::new(
                <UsingDeclaration as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            other => Err(::treesitter_types::ParseError::unexpected_kind(other, node)),
        }
    }
}
impl ::treesitter_types::Spanned for FieldDeclarationListChildren<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        match self {
            Self::AccessSpecifier(inner) => inner.span(),
            Self::AliasDeclaration(inner) => inner.span(),
            Self::ConstevalBlockDeclaration(inner) => inner.span(),
            Self::Declaration(inner) => inner.span(),
            Self::FieldDeclaration(inner) => inner.span(),
            Self::FriendDeclaration(inner) => inner.span(),
            Self::FunctionDefinition(inner) => inner.span(),
            Self::PreprocCall(inner) => inner.span(),
            Self::PreprocDef(inner) => inner.span(),
            Self::PreprocFunctionDef(inner) => inner.span(),
            Self::PreprocIf(inner) => inner.span(),
            Self::PreprocIfdef(inner) => inner.span(),
            Self::StaticAssertDeclaration(inner) => inner.span(),
            Self::TemplateDeclaration(inner) => inner.span(),
            Self::TypeDefinition(inner) => inner.span(),
            Self::UsingDeclaration(inner) => inner.span(),
        }
    }
}
#[derive(Debug, Clone)]
pub enum FieldExpressionField<'tree> {
    DependentName(::std::boxed::Box<DependentName<'tree>>),
    DestructorName(::std::boxed::Box<DestructorName<'tree>>),
    FieldIdentifier(::std::boxed::Box<FieldIdentifier<'tree>>),
    OperatorName(::std::boxed::Box<OperatorName<'tree>>),
    QualifiedIdentifier(::std::boxed::Box<QualifiedIdentifier<'tree>>),
    SpliceExpression(::std::boxed::Box<SpliceExpression<'tree>>),
    TemplateMethod(::std::boxed::Box<TemplateMethod<'tree>>),
}
impl<'tree> ::treesitter_types::FromNode<'tree> for FieldExpressionField<'tree> {
    #[allow(clippy::collapsible_else_if)]
    fn from_node(
        node: ::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        match node.kind() {
            "dependent_name" => Ok(Self::DependentName(::std::boxed::Box::new(
                <DependentName as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "destructor_name" => Ok(Self::DestructorName(::std::boxed::Box::new(
                <DestructorName as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "field_identifier" => Ok(Self::FieldIdentifier(::std::boxed::Box::new(
                <FieldIdentifier as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "operator_name" => Ok(Self::OperatorName(::std::boxed::Box::new(
                <OperatorName as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "qualified_identifier" => Ok(Self::QualifiedIdentifier(::std::boxed::Box::new(
                <QualifiedIdentifier as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "splice_expression" => Ok(Self::SpliceExpression(::std::boxed::Box::new(
                <SpliceExpression as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "template_method" => Ok(Self::TemplateMethod(::std::boxed::Box::new(
                <TemplateMethod as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            other => Err(::treesitter_types::ParseError::unexpected_kind(other, node)),
        }
    }
}
impl ::treesitter_types::Spanned for FieldExpressionField<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        match self {
            Self::DependentName(inner) => inner.span(),
            Self::DestructorName(inner) => inner.span(),
            Self::FieldIdentifier(inner) => inner.span(),
            Self::OperatorName(inner) => inner.span(),
            Self::QualifiedIdentifier(inner) => inner.span(),
            Self::SpliceExpression(inner) => inner.span(),
            Self::TemplateMethod(inner) => inner.span(),
        }
    }
}
#[derive(Debug, Clone)]
pub enum FieldExpressionOperator {
    Arrow(::treesitter_types::Span),
    Dot(::treesitter_types::Span),
    DotStar(::treesitter_types::Span),
}
impl<'tree> ::treesitter_types::FromNode<'tree> for FieldExpressionOperator {
    #[allow(clippy::collapsible_else_if)]
    fn from_node(
        node: ::tree_sitter::Node<'tree>,
        _src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        match node.kind() {
            "->" => Ok(Self::Arrow(::treesitter_types::Span::from(node))),
            "." => Ok(Self::Dot(::treesitter_types::Span::from(node))),
            ".*" => Ok(Self::DotStar(::treesitter_types::Span::from(node))),
            other => Err(::treesitter_types::ParseError::unexpected_kind(other, node)),
        }
    }
}
impl ::treesitter_types::Spanned for FieldExpressionOperator {
    fn span(&self) -> ::treesitter_types::Span {
        match self {
            Self::Arrow(span) => *span,
            Self::Dot(span) => *span,
            Self::DotStar(span) => *span,
        }
    }
}
#[derive(Debug, Clone)]
pub enum FieldInitializerChildren<'tree> {
    ArgumentList(::std::boxed::Box<ArgumentList<'tree>>),
    FieldIdentifier(::std::boxed::Box<FieldIdentifier<'tree>>),
    InitializerList(::std::boxed::Box<InitializerList<'tree>>),
    QualifiedIdentifier(::std::boxed::Box<QualifiedIdentifier<'tree>>),
    TemplateMethod(::std::boxed::Box<TemplateMethod<'tree>>),
}
impl<'tree> ::treesitter_types::FromNode<'tree> for FieldInitializerChildren<'tree> {
    #[allow(clippy::collapsible_else_if)]
    fn from_node(
        node: ::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        match node.kind() {
            "argument_list" => Ok(Self::ArgumentList(::std::boxed::Box::new(
                <ArgumentList as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "field_identifier" => Ok(Self::FieldIdentifier(::std::boxed::Box::new(
                <FieldIdentifier as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "initializer_list" => Ok(Self::InitializerList(::std::boxed::Box::new(
                <InitializerList as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "qualified_identifier" => Ok(Self::QualifiedIdentifier(::std::boxed::Box::new(
                <QualifiedIdentifier as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "template_method" => Ok(Self::TemplateMethod(::std::boxed::Box::new(
                <TemplateMethod as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            other => Err(::treesitter_types::ParseError::unexpected_kind(other, node)),
        }
    }
}
impl ::treesitter_types::Spanned for FieldInitializerChildren<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        match self {
            Self::ArgumentList(inner) => inner.span(),
            Self::FieldIdentifier(inner) => inner.span(),
            Self::InitializerList(inner) => inner.span(),
            Self::QualifiedIdentifier(inner) => inner.span(),
            Self::TemplateMethod(inner) => inner.span(),
        }
    }
}
#[derive(Debug, Clone)]
pub enum FoldExpressionLeft<'tree> {
    Ellipsis(::treesitter_types::Span),
    Expression(::std::boxed::Box<Expression<'tree>>),
}
impl<'tree> ::treesitter_types::FromNode<'tree> for FoldExpressionLeft<'tree> {
    #[allow(clippy::collapsible_else_if)]
    fn from_node(
        node: ::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        match node.kind() {
            "..." => Ok(Self::Ellipsis(::treesitter_types::Span::from(node))),
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
impl ::treesitter_types::Spanned for FoldExpressionLeft<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        match self {
            Self::Ellipsis(span) => *span,
            Self::Expression(inner) => inner.span(),
        }
    }
}
#[derive(Debug, Clone)]
pub enum FoldExpressionOperator {
    NotEq(::treesitter_types::Span),
    Percent(::treesitter_types::Span),
    PercentEq(::treesitter_types::Span),
    Amp(::treesitter_types::Span),
    AmpAmp(::treesitter_types::Span),
    AmpEq(::treesitter_types::Span),
    Star(::treesitter_types::Span),
    StarEq(::treesitter_types::Span),
    Plus(::treesitter_types::Span),
    PlusEq(::treesitter_types::Span),
    Comma(::treesitter_types::Span),
    Minus(::treesitter_types::Span),
    MinusEq(::treesitter_types::Span),
    MinusGtStar(::treesitter_types::Span),
    DotStar(::treesitter_types::Span),
    Slash(::treesitter_types::Span),
    SlashEq(::treesitter_types::Span),
    Lt(::treesitter_types::Span),
    Shl(::treesitter_types::Span),
    ShlEq(::treesitter_types::Span),
    LtEq(::treesitter_types::Span),
    Eq(::treesitter_types::Span),
    EqEq(::treesitter_types::Span),
    Gt(::treesitter_types::Span),
    GtEq(::treesitter_types::Span),
    Shr(::treesitter_types::Span),
    ShrEq(::treesitter_types::Span),
    Caret(::treesitter_types::Span),
    CaretEq(::treesitter_types::Span),
    And(::treesitter_types::Span),
    Bitand(::treesitter_types::Span),
    Bitor(::treesitter_types::Span),
    Or(::treesitter_types::Span),
    Xor(::treesitter_types::Span),
    Pipe(::treesitter_types::Span),
    PipeEq(::treesitter_types::Span),
    PipePipe(::treesitter_types::Span),
}
impl<'tree> ::treesitter_types::FromNode<'tree> for FoldExpressionOperator {
    #[allow(clippy::collapsible_else_if)]
    fn from_node(
        node: ::tree_sitter::Node<'tree>,
        _src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        match node.kind() {
            "!=" | "not_eq" => Ok(Self::NotEq(::treesitter_types::Span::from(node))),
            "%" => Ok(Self::Percent(::treesitter_types::Span::from(node))),
            "%=" => Ok(Self::PercentEq(::treesitter_types::Span::from(node))),
            "&" => Ok(Self::Amp(::treesitter_types::Span::from(node))),
            "&&" => Ok(Self::AmpAmp(::treesitter_types::Span::from(node))),
            "&=" => Ok(Self::AmpEq(::treesitter_types::Span::from(node))),
            "*" => Ok(Self::Star(::treesitter_types::Span::from(node))),
            "*=" => Ok(Self::StarEq(::treesitter_types::Span::from(node))),
            "+" => Ok(Self::Plus(::treesitter_types::Span::from(node))),
            "+=" => Ok(Self::PlusEq(::treesitter_types::Span::from(node))),
            "," => Ok(Self::Comma(::treesitter_types::Span::from(node))),
            "-" => Ok(Self::Minus(::treesitter_types::Span::from(node))),
            "-=" => Ok(Self::MinusEq(::treesitter_types::Span::from(node))),
            "->*" => Ok(Self::MinusGtStar(::treesitter_types::Span::from(node))),
            ".*" => Ok(Self::DotStar(::treesitter_types::Span::from(node))),
            "/" => Ok(Self::Slash(::treesitter_types::Span::from(node))),
            "/=" => Ok(Self::SlashEq(::treesitter_types::Span::from(node))),
            "<" => Ok(Self::Lt(::treesitter_types::Span::from(node))),
            "<<" => Ok(Self::Shl(::treesitter_types::Span::from(node))),
            "<<=" => Ok(Self::ShlEq(::treesitter_types::Span::from(node))),
            "<=" => Ok(Self::LtEq(::treesitter_types::Span::from(node))),
            "=" => Ok(Self::Eq(::treesitter_types::Span::from(node))),
            "==" => Ok(Self::EqEq(::treesitter_types::Span::from(node))),
            ">" => Ok(Self::Gt(::treesitter_types::Span::from(node))),
            ">=" => Ok(Self::GtEq(::treesitter_types::Span::from(node))),
            ">>" => Ok(Self::Shr(::treesitter_types::Span::from(node))),
            ">>=" => Ok(Self::ShrEq(::treesitter_types::Span::from(node))),
            "^" => Ok(Self::Caret(::treesitter_types::Span::from(node))),
            "^=" => Ok(Self::CaretEq(::treesitter_types::Span::from(node))),
            "and" => Ok(Self::And(::treesitter_types::Span::from(node))),
            "bitand" => Ok(Self::Bitand(::treesitter_types::Span::from(node))),
            "bitor" => Ok(Self::Bitor(::treesitter_types::Span::from(node))),
            "or" => Ok(Self::Or(::treesitter_types::Span::from(node))),
            "xor" => Ok(Self::Xor(::treesitter_types::Span::from(node))),
            "|" => Ok(Self::Pipe(::treesitter_types::Span::from(node))),
            "|=" => Ok(Self::PipeEq(::treesitter_types::Span::from(node))),
            "||" => Ok(Self::PipePipe(::treesitter_types::Span::from(node))),
            other => Err(::treesitter_types::ParseError::unexpected_kind(other, node)),
        }
    }
}
impl ::treesitter_types::Spanned for FoldExpressionOperator {
    fn span(&self) -> ::treesitter_types::Span {
        match self {
            Self::NotEq(span) => *span,
            Self::Percent(span) => *span,
            Self::PercentEq(span) => *span,
            Self::Amp(span) => *span,
            Self::AmpAmp(span) => *span,
            Self::AmpEq(span) => *span,
            Self::Star(span) => *span,
            Self::StarEq(span) => *span,
            Self::Plus(span) => *span,
            Self::PlusEq(span) => *span,
            Self::Comma(span) => *span,
            Self::Minus(span) => *span,
            Self::MinusEq(span) => *span,
            Self::MinusGtStar(span) => *span,
            Self::DotStar(span) => *span,
            Self::Slash(span) => *span,
            Self::SlashEq(span) => *span,
            Self::Lt(span) => *span,
            Self::Shl(span) => *span,
            Self::ShlEq(span) => *span,
            Self::LtEq(span) => *span,
            Self::Eq(span) => *span,
            Self::EqEq(span) => *span,
            Self::Gt(span) => *span,
            Self::GtEq(span) => *span,
            Self::Shr(span) => *span,
            Self::ShrEq(span) => *span,
            Self::Caret(span) => *span,
            Self::CaretEq(span) => *span,
            Self::And(span) => *span,
            Self::Bitand(span) => *span,
            Self::Bitor(span) => *span,
            Self::Or(span) => *span,
            Self::Xor(span) => *span,
            Self::Pipe(span) => *span,
            Self::PipeEq(span) => *span,
            Self::PipePipe(span) => *span,
        }
    }
}
#[derive(Debug, Clone)]
pub enum FoldExpressionRight<'tree> {
    Ellipsis(::treesitter_types::Span),
    Expression(::std::boxed::Box<Expression<'tree>>),
}
impl<'tree> ::treesitter_types::FromNode<'tree> for FoldExpressionRight<'tree> {
    #[allow(clippy::collapsible_else_if)]
    fn from_node(
        node: ::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        match node.kind() {
            "..." => Ok(Self::Ellipsis(::treesitter_types::Span::from(node))),
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
impl ::treesitter_types::Spanned for FoldExpressionRight<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        match self {
            Self::Ellipsis(span) => *span,
            Self::Expression(inner) => inner.span(),
        }
    }
}
#[derive(Debug, Clone)]
pub enum ForRangeLoopRight<'tree> {
    Expression(::std::boxed::Box<Expression<'tree>>),
    InitializerList(::std::boxed::Box<InitializerList<'tree>>),
}
impl<'tree> ::treesitter_types::FromNode<'tree> for ForRangeLoopRight<'tree> {
    #[allow(clippy::collapsible_else_if)]
    fn from_node(
        node: ::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        match node.kind() {
            "initializer_list" => Ok(Self::InitializerList(::std::boxed::Box::new(
                <InitializerList as ::treesitter_types::FromNode>::from_node(node, src)?,
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
impl ::treesitter_types::Spanned for ForRangeLoopRight<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        match self {
            Self::Expression(inner) => inner.span(),
            Self::InitializerList(inner) => inner.span(),
        }
    }
}
#[derive(Debug, Clone)]
pub enum ForRangeLoopChildren<'tree> {
    AttributeDeclaration(::std::boxed::Box<AttributeDeclaration<'tree>>),
    AttributeSpecifier(::std::boxed::Box<AttributeSpecifier<'tree>>),
    MsDeclspecModifier(::std::boxed::Box<MsDeclspecModifier<'tree>>),
    StorageClassSpecifier(::std::boxed::Box<StorageClassSpecifier<'tree>>),
    TypeQualifier(::std::boxed::Box<TypeQualifier<'tree>>),
}
impl<'tree> ::treesitter_types::FromNode<'tree> for ForRangeLoopChildren<'tree> {
    #[allow(clippy::collapsible_else_if)]
    fn from_node(
        node: ::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        match node.kind() {
            "attribute_declaration" => Ok(Self::AttributeDeclaration(::std::boxed::Box::new(
                <AttributeDeclaration as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "attribute_specifier" => Ok(Self::AttributeSpecifier(::std::boxed::Box::new(
                <AttributeSpecifier as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "ms_declspec_modifier" => Ok(Self::MsDeclspecModifier(::std::boxed::Box::new(
                <MsDeclspecModifier as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "storage_class_specifier" => Ok(Self::StorageClassSpecifier(::std::boxed::Box::new(
                <StorageClassSpecifier as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "type_qualifier" => Ok(Self::TypeQualifier(::std::boxed::Box::new(
                <TypeQualifier as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            other => Err(::treesitter_types::ParseError::unexpected_kind(other, node)),
        }
    }
}
impl ::treesitter_types::Spanned for ForRangeLoopChildren<'_> {
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
#[derive(Debug, Clone)]
pub enum ForStatementCondition<'tree> {
    CommaExpression(::std::boxed::Box<CommaExpression<'tree>>),
    Expression(::std::boxed::Box<Expression<'tree>>),
}
impl<'tree> ::treesitter_types::FromNode<'tree> for ForStatementCondition<'tree> {
    #[allow(clippy::collapsible_else_if)]
    fn from_node(
        node: ::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        match node.kind() {
            "comma_expression" => Ok(Self::CommaExpression(::std::boxed::Box::new(
                <CommaExpression as ::treesitter_types::FromNode>::from_node(node, src)?,
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
impl ::treesitter_types::Spanned for ForStatementCondition<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        match self {
            Self::CommaExpression(inner) => inner.span(),
            Self::Expression(inner) => inner.span(),
        }
    }
}
#[derive(Debug, Clone)]
pub enum ForStatementInitializer<'tree> {
    CommaExpression(::std::boxed::Box<CommaExpression<'tree>>),
    Declaration(::std::boxed::Box<Declaration<'tree>>),
    Expression(::std::boxed::Box<Expression<'tree>>),
}
impl<'tree> ::treesitter_types::FromNode<'tree> for ForStatementInitializer<'tree> {
    #[allow(clippy::collapsible_else_if)]
    fn from_node(
        node: ::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        match node.kind() {
            "comma_expression" => Ok(Self::CommaExpression(::std::boxed::Box::new(
                <CommaExpression as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "declaration" => Ok(Self::Declaration(::std::boxed::Box::new(
                <Declaration as ::treesitter_types::FromNode>::from_node(node, src)?,
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
impl ::treesitter_types::Spanned for ForStatementInitializer<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        match self {
            Self::CommaExpression(inner) => inner.span(),
            Self::Declaration(inner) => inner.span(),
            Self::Expression(inner) => inner.span(),
        }
    }
}
#[derive(Debug, Clone)]
pub enum ForStatementUpdate<'tree> {
    CommaExpression(::std::boxed::Box<CommaExpression<'tree>>),
    Expression(::std::boxed::Box<Expression<'tree>>),
}
impl<'tree> ::treesitter_types::FromNode<'tree> for ForStatementUpdate<'tree> {
    #[allow(clippy::collapsible_else_if)]
    fn from_node(
        node: ::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        match node.kind() {
            "comma_expression" => Ok(Self::CommaExpression(::std::boxed::Box::new(
                <CommaExpression as ::treesitter_types::FromNode>::from_node(node, src)?,
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
impl ::treesitter_types::Spanned for ForStatementUpdate<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        match self {
            Self::CommaExpression(inner) => inner.span(),
            Self::Expression(inner) => inner.span(),
        }
    }
}
#[derive(Debug, Clone)]
pub enum FriendDeclarationChildren<'tree> {
    Declaration(::std::boxed::Box<Declaration<'tree>>),
    FunctionDefinition(::std::boxed::Box<FunctionDefinition<'tree>>),
    QualifiedIdentifier(::std::boxed::Box<QualifiedIdentifier<'tree>>),
    SpliceTypeSpecifier(::std::boxed::Box<SpliceTypeSpecifier<'tree>>),
    TemplateType(::std::boxed::Box<TemplateType<'tree>>),
    TypeIdentifier(::std::boxed::Box<TypeIdentifier<'tree>>),
}
impl<'tree> ::treesitter_types::FromNode<'tree> for FriendDeclarationChildren<'tree> {
    #[allow(clippy::collapsible_else_if)]
    fn from_node(
        node: ::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        match node.kind() {
            "declaration" => Ok(Self::Declaration(::std::boxed::Box::new(
                <Declaration as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "function_definition" => Ok(Self::FunctionDefinition(::std::boxed::Box::new(
                <FunctionDefinition as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "qualified_identifier" => Ok(Self::QualifiedIdentifier(::std::boxed::Box::new(
                <QualifiedIdentifier as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "splice_type_specifier" => Ok(Self::SpliceTypeSpecifier(::std::boxed::Box::new(
                <SpliceTypeSpecifier as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "template_type" => Ok(Self::TemplateType(::std::boxed::Box::new(
                <TemplateType as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "type_identifier" => Ok(Self::TypeIdentifier(::std::boxed::Box::new(
                <TypeIdentifier as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            other => Err(::treesitter_types::ParseError::unexpected_kind(other, node)),
        }
    }
}
impl ::treesitter_types::Spanned for FriendDeclarationChildren<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        match self {
            Self::Declaration(inner) => inner.span(),
            Self::FunctionDefinition(inner) => inner.span(),
            Self::QualifiedIdentifier(inner) => inner.span(),
            Self::SpliceTypeSpecifier(inner) => inner.span(),
            Self::TemplateType(inner) => inner.span(),
            Self::TypeIdentifier(inner) => inner.span(),
        }
    }
}
#[derive(Debug, Clone)]
pub enum FunctionDeclaratorDeclarator<'tree> {
    Declarator(::std::boxed::Box<Declarator<'tree>>),
    FieldDeclarator(::std::boxed::Box<FieldDeclarator<'tree>>),
    TypeDeclarator(::std::boxed::Box<TypeDeclarator<'tree>>),
}
impl<'tree> ::treesitter_types::FromNode<'tree> for FunctionDeclaratorDeclarator<'tree> {
    #[allow(clippy::collapsible_else_if)]
    fn from_node(
        node: ::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        if let Ok(v) = <Declarator as ::treesitter_types::FromNode>::from_node(node, src) {
            Ok(Self::Declarator(::std::boxed::Box::new(v)))
        } else {
            if let Ok(v) = <FieldDeclarator as ::treesitter_types::FromNode>::from_node(node, src) {
                Ok(Self::FieldDeclarator(::std::boxed::Box::new(v)))
            } else {
                if let Ok(v) =
                    <TypeDeclarator as ::treesitter_types::FromNode>::from_node(node, src)
                {
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
#[derive(Debug, Clone)]
pub enum FunctionDeclaratorChildren<'tree> {
    AttributeDeclaration(::std::boxed::Box<AttributeDeclaration<'tree>>),
    AttributeSpecifier(::std::boxed::Box<AttributeSpecifier<'tree>>),
    GnuAsmExpression(::std::boxed::Box<GnuAsmExpression<'tree>>),
    Noexcept(::std::boxed::Box<Noexcept<'tree>>),
    RefQualifier(::std::boxed::Box<RefQualifier<'tree>>),
    RequiresClause(::std::boxed::Box<RequiresClause<'tree>>),
    ThrowSpecifier(::std::boxed::Box<ThrowSpecifier<'tree>>),
    TrailingReturnType(::std::boxed::Box<TrailingReturnType<'tree>>),
    TypeQualifier(::std::boxed::Box<TypeQualifier<'tree>>),
    VirtualSpecifier(::std::boxed::Box<VirtualSpecifier<'tree>>),
}
impl<'tree> ::treesitter_types::FromNode<'tree> for FunctionDeclaratorChildren<'tree> {
    #[allow(clippy::collapsible_else_if)]
    fn from_node(
        node: ::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        match node.kind() {
            "attribute_declaration" => Ok(Self::AttributeDeclaration(::std::boxed::Box::new(
                <AttributeDeclaration as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "attribute_specifier" => Ok(Self::AttributeSpecifier(::std::boxed::Box::new(
                <AttributeSpecifier as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "gnu_asm_expression" => Ok(Self::GnuAsmExpression(::std::boxed::Box::new(
                <GnuAsmExpression as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "noexcept" => Ok(Self::Noexcept(::std::boxed::Box::new(
                <Noexcept as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "ref_qualifier" => Ok(Self::RefQualifier(::std::boxed::Box::new(
                <RefQualifier as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "requires_clause" => Ok(Self::RequiresClause(::std::boxed::Box::new(
                <RequiresClause as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "throw_specifier" => Ok(Self::ThrowSpecifier(::std::boxed::Box::new(
                <ThrowSpecifier as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "trailing_return_type" => Ok(Self::TrailingReturnType(::std::boxed::Box::new(
                <TrailingReturnType as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "type_qualifier" => Ok(Self::TypeQualifier(::std::boxed::Box::new(
                <TypeQualifier as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "virtual_specifier" => Ok(Self::VirtualSpecifier(::std::boxed::Box::new(
                <VirtualSpecifier as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            other => Err(::treesitter_types::ParseError::unexpected_kind(other, node)),
        }
    }
}
impl ::treesitter_types::Spanned for FunctionDeclaratorChildren<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        match self {
            Self::AttributeDeclaration(inner) => inner.span(),
            Self::AttributeSpecifier(inner) => inner.span(),
            Self::GnuAsmExpression(inner) => inner.span(),
            Self::Noexcept(inner) => inner.span(),
            Self::RefQualifier(inner) => inner.span(),
            Self::RequiresClause(inner) => inner.span(),
            Self::ThrowSpecifier(inner) => inner.span(),
            Self::TrailingReturnType(inner) => inner.span(),
            Self::TypeQualifier(inner) => inner.span(),
            Self::VirtualSpecifier(inner) => inner.span(),
        }
    }
}
#[derive(Debug, Clone)]
pub enum FunctionDefinitionBody<'tree> {
    CompoundStatement(::std::boxed::Box<CompoundStatement<'tree>>),
    TryStatement(::std::boxed::Box<TryStatement<'tree>>),
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
            "try_statement" => Ok(Self::TryStatement(::std::boxed::Box::new(
                <TryStatement as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            other => Err(::treesitter_types::ParseError::unexpected_kind(other, node)),
        }
    }
}
impl ::treesitter_types::Spanned for FunctionDefinitionBody<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        match self {
            Self::CompoundStatement(inner) => inner.span(),
            Self::TryStatement(inner) => inner.span(),
        }
    }
}
#[derive(Debug, Clone)]
pub enum FunctionDefinitionDeclarator<'tree> {
    Declarator(::std::boxed::Box<Declarator<'tree>>),
    FieldDeclarator(::std::boxed::Box<FieldDeclarator<'tree>>),
    OperatorCast(::std::boxed::Box<OperatorCast<'tree>>),
}
impl<'tree> ::treesitter_types::FromNode<'tree> for FunctionDefinitionDeclarator<'tree> {
    #[allow(clippy::collapsible_else_if)]
    fn from_node(
        node: ::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        match node.kind() {
            "operator_cast" => Ok(Self::OperatorCast(::std::boxed::Box::new(
                <OperatorCast as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            _other => {
                if let Ok(v) = <Declarator as ::treesitter_types::FromNode>::from_node(node, src) {
                    Ok(Self::Declarator(::std::boxed::Box::new(v)))
                } else {
                    if let Ok(v) =
                        <FieldDeclarator as ::treesitter_types::FromNode>::from_node(node, src)
                    {
                        Ok(Self::FieldDeclarator(::std::boxed::Box::new(v)))
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
impl ::treesitter_types::Spanned for FunctionDefinitionDeclarator<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        match self {
            Self::Declarator(inner) => inner.span(),
            Self::FieldDeclarator(inner) => inner.span(),
            Self::OperatorCast(inner) => inner.span(),
        }
    }
}
#[derive(Debug, Clone)]
pub enum FunctionDefinitionChildren<'tree> {
    AttributeDeclaration(::std::boxed::Box<AttributeDeclaration<'tree>>),
    AttributeSpecifier(::std::boxed::Box<AttributeSpecifier<'tree>>),
    DefaultMethodClause(::std::boxed::Box<DefaultMethodClause<'tree>>),
    DeleteMethodClause(::std::boxed::Box<DeleteMethodClause<'tree>>),
    ExplicitFunctionSpecifier(::std::boxed::Box<ExplicitFunctionSpecifier<'tree>>),
    FieldInitializerList(::std::boxed::Box<FieldInitializerList<'tree>>),
    MsCallModifier(::std::boxed::Box<MsCallModifier<'tree>>),
    MsDeclspecModifier(::std::boxed::Box<MsDeclspecModifier<'tree>>),
    PureVirtualClause(::std::boxed::Box<PureVirtualClause<'tree>>),
    StorageClassSpecifier(::std::boxed::Box<StorageClassSpecifier<'tree>>),
    TryStatement(::std::boxed::Box<TryStatement<'tree>>),
    TypeQualifier(::std::boxed::Box<TypeQualifier<'tree>>),
}
impl<'tree> ::treesitter_types::FromNode<'tree> for FunctionDefinitionChildren<'tree> {
    #[allow(clippy::collapsible_else_if)]
    fn from_node(
        node: ::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        match node.kind() {
            "attribute_declaration" => Ok(Self::AttributeDeclaration(::std::boxed::Box::new(
                <AttributeDeclaration as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "attribute_specifier" => Ok(Self::AttributeSpecifier(::std::boxed::Box::new(
                <AttributeSpecifier as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "default_method_clause" => Ok(Self::DefaultMethodClause(::std::boxed::Box::new(
                <DefaultMethodClause as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "delete_method_clause" => Ok(Self::DeleteMethodClause(::std::boxed::Box::new(
                <DeleteMethodClause as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "explicit_function_specifier" => {
                Ok(Self::ExplicitFunctionSpecifier(::std::boxed::Box::new(
                    <ExplicitFunctionSpecifier as ::treesitter_types::FromNode>::from_node(
                        node, src,
                    )?,
                )))
            }
            "field_initializer_list" => Ok(Self::FieldInitializerList(::std::boxed::Box::new(
                <FieldInitializerList as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "ms_call_modifier" => Ok(Self::MsCallModifier(::std::boxed::Box::new(
                <MsCallModifier as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "ms_declspec_modifier" => Ok(Self::MsDeclspecModifier(::std::boxed::Box::new(
                <MsDeclspecModifier as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "pure_virtual_clause" => Ok(Self::PureVirtualClause(::std::boxed::Box::new(
                <PureVirtualClause as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "storage_class_specifier" => Ok(Self::StorageClassSpecifier(::std::boxed::Box::new(
                <StorageClassSpecifier as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "try_statement" => Ok(Self::TryStatement(::std::boxed::Box::new(
                <TryStatement as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "type_qualifier" => Ok(Self::TypeQualifier(::std::boxed::Box::new(
                <TypeQualifier as ::treesitter_types::FromNode>::from_node(node, src)?,
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
            Self::DefaultMethodClause(inner) => inner.span(),
            Self::DeleteMethodClause(inner) => inner.span(),
            Self::ExplicitFunctionSpecifier(inner) => inner.span(),
            Self::FieldInitializerList(inner) => inner.span(),
            Self::MsCallModifier(inner) => inner.span(),
            Self::MsDeclspecModifier(inner) => inner.span(),
            Self::PureVirtualClause(inner) => inner.span(),
            Self::StorageClassSpecifier(inner) => inner.span(),
            Self::TryStatement(inner) => inner.span(),
            Self::TypeQualifier(inner) => inner.span(),
        }
    }
}
#[derive(Debug, Clone)]
pub enum GenericExpressionChildren<'tree> {
    Expression(::std::boxed::Box<Expression<'tree>>),
    TypeDescriptor(::std::boxed::Box<TypeDescriptor<'tree>>),
}
impl<'tree> ::treesitter_types::FromNode<'tree> for GenericExpressionChildren<'tree> {
    #[allow(clippy::collapsible_else_if)]
    fn from_node(
        node: ::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        match node.kind() {
            "type_descriptor" => Ok(Self::TypeDescriptor(::std::boxed::Box::new(
                <TypeDescriptor as ::treesitter_types::FromNode>::from_node(node, src)?,
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
impl ::treesitter_types::Spanned for GenericExpressionChildren<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        match self {
            Self::Expression(inner) => inner.span(),
            Self::TypeDescriptor(inner) => inner.span(),
        }
    }
}
#[derive(Debug, Clone)]
pub enum GnuAsmClobberListRegister<'tree> {
    ConcatenatedString(::std::boxed::Box<ConcatenatedString<'tree>>),
    RawStringLiteral(::std::boxed::Box<RawStringLiteral<'tree>>),
    StringLiteral(::std::boxed::Box<StringLiteral<'tree>>),
}
impl<'tree> ::treesitter_types::FromNode<'tree> for GnuAsmClobberListRegister<'tree> {
    #[allow(clippy::collapsible_else_if)]
    fn from_node(
        node: ::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        match node.kind() {
            "concatenated_string" => Ok(Self::ConcatenatedString(::std::boxed::Box::new(
                <ConcatenatedString as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "raw_string_literal" => Ok(Self::RawStringLiteral(::std::boxed::Box::new(
                <RawStringLiteral as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "string_literal" => Ok(Self::StringLiteral(::std::boxed::Box::new(
                <StringLiteral as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            other => Err(::treesitter_types::ParseError::unexpected_kind(other, node)),
        }
    }
}
impl ::treesitter_types::Spanned for GnuAsmClobberListRegister<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        match self {
            Self::ConcatenatedString(inner) => inner.span(),
            Self::RawStringLiteral(inner) => inner.span(),
            Self::StringLiteral(inner) => inner.span(),
        }
    }
}
#[derive(Debug, Clone)]
pub enum GnuAsmExpressionAssemblyCode<'tree> {
    ConcatenatedString(::std::boxed::Box<ConcatenatedString<'tree>>),
    RawStringLiteral(::std::boxed::Box<RawStringLiteral<'tree>>),
    StringLiteral(::std::boxed::Box<StringLiteral<'tree>>),
}
impl<'tree> ::treesitter_types::FromNode<'tree> for GnuAsmExpressionAssemblyCode<'tree> {
    #[allow(clippy::collapsible_else_if)]
    fn from_node(
        node: ::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        match node.kind() {
            "concatenated_string" => Ok(Self::ConcatenatedString(::std::boxed::Box::new(
                <ConcatenatedString as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "raw_string_literal" => Ok(Self::RawStringLiteral(::std::boxed::Box::new(
                <RawStringLiteral as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "string_literal" => Ok(Self::StringLiteral(::std::boxed::Box::new(
                <StringLiteral as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            other => Err(::treesitter_types::ParseError::unexpected_kind(other, node)),
        }
    }
}
impl ::treesitter_types::Spanned for GnuAsmExpressionAssemblyCode<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        match self {
            Self::ConcatenatedString(inner) => inner.span(),
            Self::RawStringLiteral(inner) => inner.span(),
            Self::StringLiteral(inner) => inner.span(),
        }
    }
}
#[derive(Debug, Clone)]
pub enum ImportDeclarationHeader<'tree> {
    StringLiteral(::std::boxed::Box<StringLiteral<'tree>>),
    SystemLibString(::std::boxed::Box<SystemLibString<'tree>>),
}
impl<'tree> ::treesitter_types::FromNode<'tree> for ImportDeclarationHeader<'tree> {
    #[allow(clippy::collapsible_else_if)]
    fn from_node(
        node: ::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        match node.kind() {
            "string_literal" => Ok(Self::StringLiteral(::std::boxed::Box::new(
                <StringLiteral as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "system_lib_string" => Ok(Self::SystemLibString(::std::boxed::Box::new(
                <SystemLibString as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            other => Err(::treesitter_types::ParseError::unexpected_kind(other, node)),
        }
    }
}
impl ::treesitter_types::Spanned for ImportDeclarationHeader<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        match self {
            Self::StringLiteral(inner) => inner.span(),
            Self::SystemLibString(inner) => inner.span(),
        }
    }
}
#[derive(Debug, Clone)]
pub enum InitDeclaratorValue<'tree> {
    ArgumentList(::std::boxed::Box<ArgumentList<'tree>>),
    Expression(::std::boxed::Box<Expression<'tree>>),
    InitializerList(::std::boxed::Box<InitializerList<'tree>>),
}
impl<'tree> ::treesitter_types::FromNode<'tree> for InitDeclaratorValue<'tree> {
    #[allow(clippy::collapsible_else_if)]
    fn from_node(
        node: ::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        match node.kind() {
            "argument_list" => Ok(Self::ArgumentList(::std::boxed::Box::new(
                <ArgumentList as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "initializer_list" => Ok(Self::InitializerList(::std::boxed::Box::new(
                <InitializerList as ::treesitter_types::FromNode>::from_node(node, src)?,
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
impl ::treesitter_types::Spanned for InitDeclaratorValue<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        match self {
            Self::ArgumentList(inner) => inner.span(),
            Self::Expression(inner) => inner.span(),
            Self::InitializerList(inner) => inner.span(),
        }
    }
}
#[derive(Debug, Clone)]
pub enum InitStatementChildren<'tree> {
    AliasDeclaration(::std::boxed::Box<AliasDeclaration<'tree>>),
    Declaration(::std::boxed::Box<Declaration<'tree>>),
    ExpressionStatement(::std::boxed::Box<ExpressionStatement<'tree>>),
    TypeDefinition(::std::boxed::Box<TypeDefinition<'tree>>),
}
impl<'tree> ::treesitter_types::FromNode<'tree> for InitStatementChildren<'tree> {
    #[allow(clippy::collapsible_else_if)]
    fn from_node(
        node: ::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        match node.kind() {
            "alias_declaration" => Ok(Self::AliasDeclaration(::std::boxed::Box::new(
                <AliasDeclaration as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "declaration" => Ok(Self::Declaration(::std::boxed::Box::new(
                <Declaration as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "expression_statement" => Ok(Self::ExpressionStatement(::std::boxed::Box::new(
                <ExpressionStatement as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "type_definition" => Ok(Self::TypeDefinition(::std::boxed::Box::new(
                <TypeDefinition as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            other => Err(::treesitter_types::ParseError::unexpected_kind(other, node)),
        }
    }
}
impl ::treesitter_types::Spanned for InitStatementChildren<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        match self {
            Self::AliasDeclaration(inner) => inner.span(),
            Self::Declaration(inner) => inner.span(),
            Self::ExpressionStatement(inner) => inner.span(),
            Self::TypeDefinition(inner) => inner.span(),
        }
    }
}
#[derive(Debug, Clone)]
pub enum InitializerListChildren<'tree> {
    Expression(::std::boxed::Box<Expression<'tree>>),
    InitializerList(::std::boxed::Box<InitializerList<'tree>>),
    InitializerPair(::std::boxed::Box<InitializerPair<'tree>>),
}
impl<'tree> ::treesitter_types::FromNode<'tree> for InitializerListChildren<'tree> {
    #[allow(clippy::collapsible_else_if)]
    fn from_node(
        node: ::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        match node.kind() {
            "initializer_list" => Ok(Self::InitializerList(::std::boxed::Box::new(
                <InitializerList as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "initializer_pair" => Ok(Self::InitializerPair(::std::boxed::Box::new(
                <InitializerPair as ::treesitter_types::FromNode>::from_node(node, src)?,
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
impl ::treesitter_types::Spanned for InitializerListChildren<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        match self {
            Self::Expression(inner) => inner.span(),
            Self::InitializerList(inner) => inner.span(),
            Self::InitializerPair(inner) => inner.span(),
        }
    }
}
#[derive(Debug, Clone)]
pub enum InitializerPairDesignator<'tree> {
    FieldDesignator(::std::boxed::Box<FieldDesignator<'tree>>),
    FieldIdentifier(::std::boxed::Box<FieldIdentifier<'tree>>),
    SubscriptDesignator(::std::boxed::Box<SubscriptDesignator<'tree>>),
    SubscriptRangeDesignator(::std::boxed::Box<SubscriptRangeDesignator<'tree>>),
}
impl<'tree> ::treesitter_types::FromNode<'tree> for InitializerPairDesignator<'tree> {
    #[allow(clippy::collapsible_else_if)]
    fn from_node(
        node: ::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        match node.kind() {
            "field_designator" => Ok(Self::FieldDesignator(::std::boxed::Box::new(
                <FieldDesignator as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "field_identifier" => Ok(Self::FieldIdentifier(::std::boxed::Box::new(
                <FieldIdentifier as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "subscript_designator" => Ok(Self::SubscriptDesignator(::std::boxed::Box::new(
                <SubscriptDesignator as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "subscript_range_designator" => {
                Ok(Self::SubscriptRangeDesignator(::std::boxed::Box::new(
                    <SubscriptRangeDesignator as ::treesitter_types::FromNode>::from_node(
                        node, src,
                    )?,
                )))
            }
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
#[derive(Debug, Clone)]
pub enum InitializerPairValue<'tree> {
    Expression(::std::boxed::Box<Expression<'tree>>),
    InitializerList(::std::boxed::Box<InitializerList<'tree>>),
}
impl<'tree> ::treesitter_types::FromNode<'tree> for InitializerPairValue<'tree> {
    #[allow(clippy::collapsible_else_if)]
    fn from_node(
        node: ::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        match node.kind() {
            "initializer_list" => Ok(Self::InitializerList(::std::boxed::Box::new(
                <InitializerList as ::treesitter_types::FromNode>::from_node(node, src)?,
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
impl ::treesitter_types::Spanned for InitializerPairValue<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        match self {
            Self::Expression(inner) => inner.span(),
            Self::InitializerList(inner) => inner.span(),
        }
    }
}
#[derive(Debug, Clone)]
pub enum LabeledStatementChildren<'tree> {
    Declaration(::std::boxed::Box<Declaration<'tree>>),
    Statement(::std::boxed::Box<Statement<'tree>>),
}
impl<'tree> ::treesitter_types::FromNode<'tree> for LabeledStatementChildren<'tree> {
    #[allow(clippy::collapsible_else_if)]
    fn from_node(
        node: ::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        match node.kind() {
            "declaration" => Ok(Self::Declaration(::std::boxed::Box::new(
                <Declaration as ::treesitter_types::FromNode>::from_node(node, src)?,
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
impl ::treesitter_types::Spanned for LabeledStatementChildren<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        match self {
            Self::Declaration(inner) => inner.span(),
            Self::Statement(inner) => inner.span(),
        }
    }
}
#[derive(Debug, Clone)]
pub enum LambdaCaptureSpecifierChildren<'tree> {
    Identifier(::std::boxed::Box<Identifier<'tree>>),
    LambdaCaptureInitializer(::std::boxed::Box<LambdaCaptureInitializer<'tree>>),
    LambdaDefaultCapture(::std::boxed::Box<LambdaDefaultCapture<'tree>>),
    ParameterPackExpansion(::std::boxed::Box<ParameterPackExpansion<'tree>>),
    QualifiedIdentifier(::std::boxed::Box<QualifiedIdentifier<'tree>>),
    This(::std::boxed::Box<This<'tree>>),
}
impl<'tree> ::treesitter_types::FromNode<'tree> for LambdaCaptureSpecifierChildren<'tree> {
    #[allow(clippy::collapsible_else_if)]
    fn from_node(
        node: ::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        match node.kind() {
            "identifier" => Ok(Self::Identifier(::std::boxed::Box::new(
                <Identifier as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "lambda_capture_initializer" => {
                Ok(Self::LambdaCaptureInitializer(::std::boxed::Box::new(
                    <LambdaCaptureInitializer as ::treesitter_types::FromNode>::from_node(
                        node, src,
                    )?,
                )))
            }
            "lambda_default_capture" => Ok(Self::LambdaDefaultCapture(::std::boxed::Box::new(
                <LambdaDefaultCapture as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "parameter_pack_expansion" => Ok(Self::ParameterPackExpansion(::std::boxed::Box::new(
                <ParameterPackExpansion as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "qualified_identifier" => Ok(Self::QualifiedIdentifier(::std::boxed::Box::new(
                <QualifiedIdentifier as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "this" => Ok(Self::This(::std::boxed::Box::new(
                <This as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            other => Err(::treesitter_types::ParseError::unexpected_kind(other, node)),
        }
    }
}
impl ::treesitter_types::Spanned for LambdaCaptureSpecifierChildren<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        match self {
            Self::Identifier(inner) => inner.span(),
            Self::LambdaCaptureInitializer(inner) => inner.span(),
            Self::LambdaDefaultCapture(inner) => inner.span(),
            Self::ParameterPackExpansion(inner) => inner.span(),
            Self::QualifiedIdentifier(inner) => inner.span(),
            Self::This(inner) => inner.span(),
        }
    }
}
#[derive(Debug, Clone)]
pub enum LambdaDeclaratorChildren<'tree> {
    AttributeDeclaration(::std::boxed::Box<AttributeDeclaration<'tree>>),
    LambdaSpecifier(::std::boxed::Box<LambdaSpecifier<'tree>>),
    Noexcept(::std::boxed::Box<Noexcept<'tree>>),
    RequiresClause(::std::boxed::Box<RequiresClause<'tree>>),
    ThrowSpecifier(::std::boxed::Box<ThrowSpecifier<'tree>>),
    TrailingReturnType(::std::boxed::Box<TrailingReturnType<'tree>>),
}
impl<'tree> ::treesitter_types::FromNode<'tree> for LambdaDeclaratorChildren<'tree> {
    #[allow(clippy::collapsible_else_if)]
    fn from_node(
        node: ::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        match node.kind() {
            "attribute_declaration" => Ok(Self::AttributeDeclaration(::std::boxed::Box::new(
                <AttributeDeclaration as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "lambda_specifier" => Ok(Self::LambdaSpecifier(::std::boxed::Box::new(
                <LambdaSpecifier as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "noexcept" => Ok(Self::Noexcept(::std::boxed::Box::new(
                <Noexcept as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "requires_clause" => Ok(Self::RequiresClause(::std::boxed::Box::new(
                <RequiresClause as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "throw_specifier" => Ok(Self::ThrowSpecifier(::std::boxed::Box::new(
                <ThrowSpecifier as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "trailing_return_type" => Ok(Self::TrailingReturnType(::std::boxed::Box::new(
                <TrailingReturnType as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            other => Err(::treesitter_types::ParseError::unexpected_kind(other, node)),
        }
    }
}
impl ::treesitter_types::Spanned for LambdaDeclaratorChildren<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        match self {
            Self::AttributeDeclaration(inner) => inner.span(),
            Self::LambdaSpecifier(inner) => inner.span(),
            Self::Noexcept(inner) => inner.span(),
            Self::RequiresClause(inner) => inner.span(),
            Self::ThrowSpecifier(inner) => inner.span(),
            Self::TrailingReturnType(inner) => inner.span(),
        }
    }
}
#[derive(Debug, Clone)]
pub enum LinkageSpecificationBody<'tree> {
    Declaration(::std::boxed::Box<Declaration<'tree>>),
    DeclarationList(::std::boxed::Box<DeclarationList<'tree>>),
    FunctionDefinition(::std::boxed::Box<FunctionDefinition<'tree>>),
}
impl<'tree> ::treesitter_types::FromNode<'tree> for LinkageSpecificationBody<'tree> {
    #[allow(clippy::collapsible_else_if)]
    fn from_node(
        node: ::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        match node.kind() {
            "declaration" => Ok(Self::Declaration(::std::boxed::Box::new(
                <Declaration as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "declaration_list" => Ok(Self::DeclarationList(::std::boxed::Box::new(
                <DeclarationList as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "function_definition" => Ok(Self::FunctionDefinition(::std::boxed::Box::new(
                <FunctionDefinition as ::treesitter_types::FromNode>::from_node(node, src)?,
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
#[derive(Debug, Clone)]
pub enum MsPointerModifierChildren<'tree> {
    MsRestrictModifier(::std::boxed::Box<MsRestrictModifier<'tree>>),
    MsSignedPtrModifier(::std::boxed::Box<MsSignedPtrModifier<'tree>>),
    MsUnalignedPtrModifier(::std::boxed::Box<MsUnalignedPtrModifier<'tree>>),
    MsUnsignedPtrModifier(::std::boxed::Box<MsUnsignedPtrModifier<'tree>>),
}
impl<'tree> ::treesitter_types::FromNode<'tree> for MsPointerModifierChildren<'tree> {
    #[allow(clippy::collapsible_else_if)]
    fn from_node(
        node: ::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        match node.kind() {
            "ms_restrict_modifier" => Ok(Self::MsRestrictModifier(::std::boxed::Box::new(
                <MsRestrictModifier as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "ms_signed_ptr_modifier" => Ok(Self::MsSignedPtrModifier(::std::boxed::Box::new(
                <MsSignedPtrModifier as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "ms_unaligned_ptr_modifier" => {
                Ok(Self::MsUnalignedPtrModifier(::std::boxed::Box::new(
                    <MsUnalignedPtrModifier as ::treesitter_types::FromNode>::from_node(node, src)?,
                )))
            }
            "ms_unsigned_ptr_modifier" => Ok(Self::MsUnsignedPtrModifier(::std::boxed::Box::new(
                <MsUnsignedPtrModifier as ::treesitter_types::FromNode>::from_node(node, src)?,
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
#[derive(Debug, Clone)]
pub enum NamespaceAliasDefinitionChildren<'tree> {
    NamespaceIdentifier(::std::boxed::Box<NamespaceIdentifier<'tree>>),
    NestedNamespaceSpecifier(::std::boxed::Box<NestedNamespaceSpecifier<'tree>>),
    SpliceSpecifier(::std::boxed::Box<SpliceSpecifier<'tree>>),
}
impl<'tree> ::treesitter_types::FromNode<'tree> for NamespaceAliasDefinitionChildren<'tree> {
    #[allow(clippy::collapsible_else_if)]
    fn from_node(
        node: ::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        match node.kind() {
            "namespace_identifier" => Ok(Self::NamespaceIdentifier(::std::boxed::Box::new(
                <NamespaceIdentifier as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "nested_namespace_specifier" => {
                Ok(Self::NestedNamespaceSpecifier(::std::boxed::Box::new(
                    <NestedNamespaceSpecifier as ::treesitter_types::FromNode>::from_node(
                        node, src,
                    )?,
                )))
            }
            "splice_specifier" => Ok(Self::SpliceSpecifier(::std::boxed::Box::new(
                <SpliceSpecifier as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            other => Err(::treesitter_types::ParseError::unexpected_kind(other, node)),
        }
    }
}
impl ::treesitter_types::Spanned for NamespaceAliasDefinitionChildren<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        match self {
            Self::NamespaceIdentifier(inner) => inner.span(),
            Self::NestedNamespaceSpecifier(inner) => inner.span(),
            Self::SpliceSpecifier(inner) => inner.span(),
        }
    }
}
#[derive(Debug, Clone)]
pub enum NamespaceDefinitionName<'tree> {
    NamespaceIdentifier(::std::boxed::Box<NamespaceIdentifier<'tree>>),
    NestedNamespaceSpecifier(::std::boxed::Box<NestedNamespaceSpecifier<'tree>>),
}
impl<'tree> ::treesitter_types::FromNode<'tree> for NamespaceDefinitionName<'tree> {
    #[allow(clippy::collapsible_else_if)]
    fn from_node(
        node: ::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        match node.kind() {
            "namespace_identifier" => Ok(Self::NamespaceIdentifier(::std::boxed::Box::new(
                <NamespaceIdentifier as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "nested_namespace_specifier" => {
                Ok(Self::NestedNamespaceSpecifier(::std::boxed::Box::new(
                    <NestedNamespaceSpecifier as ::treesitter_types::FromNode>::from_node(
                        node, src,
                    )?,
                )))
            }
            other => Err(::treesitter_types::ParseError::unexpected_kind(other, node)),
        }
    }
}
impl ::treesitter_types::Spanned for NamespaceDefinitionName<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        match self {
            Self::NamespaceIdentifier(inner) => inner.span(),
            Self::NestedNamespaceSpecifier(inner) => inner.span(),
        }
    }
}
#[derive(Debug, Clone)]
pub enum NestedNamespaceSpecifierChildren<'tree> {
    NamespaceIdentifier(::std::boxed::Box<NamespaceIdentifier<'tree>>),
    NestedNamespaceSpecifier(::std::boxed::Box<NestedNamespaceSpecifier<'tree>>),
}
impl<'tree> ::treesitter_types::FromNode<'tree> for NestedNamespaceSpecifierChildren<'tree> {
    #[allow(clippy::collapsible_else_if)]
    fn from_node(
        node: ::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        match node.kind() {
            "namespace_identifier" => Ok(Self::NamespaceIdentifier(::std::boxed::Box::new(
                <NamespaceIdentifier as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "nested_namespace_specifier" => {
                Ok(Self::NestedNamespaceSpecifier(::std::boxed::Box::new(
                    <NestedNamespaceSpecifier as ::treesitter_types::FromNode>::from_node(
                        node, src,
                    )?,
                )))
            }
            other => Err(::treesitter_types::ParseError::unexpected_kind(other, node)),
        }
    }
}
impl ::treesitter_types::Spanned for NestedNamespaceSpecifierChildren<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        match self {
            Self::NamespaceIdentifier(inner) => inner.span(),
            Self::NestedNamespaceSpecifier(inner) => inner.span(),
        }
    }
}
#[derive(Debug, Clone)]
pub enum NewExpressionArguments<'tree> {
    ArgumentList(::std::boxed::Box<ArgumentList<'tree>>),
    InitializerList(::std::boxed::Box<InitializerList<'tree>>),
}
impl<'tree> ::treesitter_types::FromNode<'tree> for NewExpressionArguments<'tree> {
    #[allow(clippy::collapsible_else_if)]
    fn from_node(
        node: ::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        match node.kind() {
            "argument_list" => Ok(Self::ArgumentList(::std::boxed::Box::new(
                <ArgumentList as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "initializer_list" => Ok(Self::InitializerList(::std::boxed::Box::new(
                <InitializerList as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            other => Err(::treesitter_types::ParseError::unexpected_kind(other, node)),
        }
    }
}
impl ::treesitter_types::Spanned for NewExpressionArguments<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        match self {
            Self::ArgumentList(inner) => inner.span(),
            Self::InitializerList(inner) => inner.span(),
        }
    }
}
#[derive(Debug, Clone)]
pub enum OperatorCastChildren<'tree> {
    AttributeDeclaration(::std::boxed::Box<AttributeDeclaration<'tree>>),
    AttributeSpecifier(::std::boxed::Box<AttributeSpecifier<'tree>>),
    MsDeclspecModifier(::std::boxed::Box<MsDeclspecModifier<'tree>>),
    StorageClassSpecifier(::std::boxed::Box<StorageClassSpecifier<'tree>>),
    TypeQualifier(::std::boxed::Box<TypeQualifier<'tree>>),
}
impl<'tree> ::treesitter_types::FromNode<'tree> for OperatorCastChildren<'tree> {
    #[allow(clippy::collapsible_else_if)]
    fn from_node(
        node: ::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        match node.kind() {
            "attribute_declaration" => Ok(Self::AttributeDeclaration(::std::boxed::Box::new(
                <AttributeDeclaration as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "attribute_specifier" => Ok(Self::AttributeSpecifier(::std::boxed::Box::new(
                <AttributeSpecifier as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "ms_declspec_modifier" => Ok(Self::MsDeclspecModifier(::std::boxed::Box::new(
                <MsDeclspecModifier as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "storage_class_specifier" => Ok(Self::StorageClassSpecifier(::std::boxed::Box::new(
                <StorageClassSpecifier as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "type_qualifier" => Ok(Self::TypeQualifier(::std::boxed::Box::new(
                <TypeQualifier as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            other => Err(::treesitter_types::ParseError::unexpected_kind(other, node)),
        }
    }
}
impl ::treesitter_types::Spanned for OperatorCastChildren<'_> {
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
#[derive(Debug, Clone)]
pub enum OptionalParameterDeclarationDeclarator<'tree> {
    Declarator(::std::boxed::Box<Declarator<'tree>>),
    AbstractReferenceDeclarator(::std::boxed::Box<AbstractReferenceDeclarator<'tree>>),
}
impl<'tree> ::treesitter_types::FromNode<'tree> for OptionalParameterDeclarationDeclarator<'tree> {
    #[allow(clippy::collapsible_else_if)]
    fn from_node(
        node: ::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        match node.kind() {
            "abstract_reference_declarator" => {
                Ok(Self::AbstractReferenceDeclarator(::std::boxed::Box::new(
                    <AbstractReferenceDeclarator as ::treesitter_types::FromNode>::from_node(
                        node, src,
                    )?,
                )))
            }
            _other => {
                if let Ok(v) = <Declarator as ::treesitter_types::FromNode>::from_node(node, src) {
                    Ok(Self::Declarator(::std::boxed::Box::new(v)))
                } else {
                    Err(::treesitter_types::ParseError::unexpected_kind(
                        _other, node,
                    ))
                }
            }
        }
    }
}
impl ::treesitter_types::Spanned for OptionalParameterDeclarationDeclarator<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        match self {
            Self::Declarator(inner) => inner.span(),
            Self::AbstractReferenceDeclarator(inner) => inner.span(),
        }
    }
}
#[derive(Debug, Clone)]
pub enum OptionalParameterDeclarationChildren<'tree> {
    AttributeDeclaration(::std::boxed::Box<AttributeDeclaration<'tree>>),
    AttributeSpecifier(::std::boxed::Box<AttributeSpecifier<'tree>>),
    MsDeclspecModifier(::std::boxed::Box<MsDeclspecModifier<'tree>>),
    StorageClassSpecifier(::std::boxed::Box<StorageClassSpecifier<'tree>>),
    TypeQualifier(::std::boxed::Box<TypeQualifier<'tree>>),
}
impl<'tree> ::treesitter_types::FromNode<'tree> for OptionalParameterDeclarationChildren<'tree> {
    #[allow(clippy::collapsible_else_if)]
    fn from_node(
        node: ::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        match node.kind() {
            "attribute_declaration" => Ok(Self::AttributeDeclaration(::std::boxed::Box::new(
                <AttributeDeclaration as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "attribute_specifier" => Ok(Self::AttributeSpecifier(::std::boxed::Box::new(
                <AttributeSpecifier as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "ms_declspec_modifier" => Ok(Self::MsDeclspecModifier(::std::boxed::Box::new(
                <MsDeclspecModifier as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "storage_class_specifier" => Ok(Self::StorageClassSpecifier(::std::boxed::Box::new(
                <StorageClassSpecifier as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "type_qualifier" => Ok(Self::TypeQualifier(::std::boxed::Box::new(
                <TypeQualifier as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            other => Err(::treesitter_types::ParseError::unexpected_kind(other, node)),
        }
    }
}
impl ::treesitter_types::Spanned for OptionalParameterDeclarationChildren<'_> {
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
#[derive(Debug, Clone)]
pub enum ParameterDeclarationDeclarator<'tree> {
    AbstractDeclarator(::std::boxed::Box<AbstractDeclarator<'tree>>),
    Declarator(::std::boxed::Box<Declarator<'tree>>),
}
impl<'tree> ::treesitter_types::FromNode<'tree> for ParameterDeclarationDeclarator<'tree> {
    #[allow(clippy::collapsible_else_if)]
    fn from_node(
        node: ::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        if let Ok(v) = <AbstractDeclarator as ::treesitter_types::FromNode>::from_node(node, src) {
            Ok(Self::AbstractDeclarator(::std::boxed::Box::new(v)))
        } else {
            if let Ok(v) = <Declarator as ::treesitter_types::FromNode>::from_node(node, src) {
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
#[derive(Debug, Clone)]
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
        node: ::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        match node.kind() {
            "attribute_declaration" => Ok(Self::AttributeDeclaration(::std::boxed::Box::new(
                <AttributeDeclaration as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "attribute_specifier" => Ok(Self::AttributeSpecifier(::std::boxed::Box::new(
                <AttributeSpecifier as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "ms_declspec_modifier" => Ok(Self::MsDeclspecModifier(::std::boxed::Box::new(
                <MsDeclspecModifier as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "storage_class_specifier" => Ok(Self::StorageClassSpecifier(::std::boxed::Box::new(
                <StorageClassSpecifier as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "type_qualifier" => Ok(Self::TypeQualifier(::std::boxed::Box::new(
                <TypeQualifier as ::treesitter_types::FromNode>::from_node(node, src)?,
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
#[derive(Debug, Clone)]
pub enum ParameterListChildren<'tree> {
    ExplicitObjectParameterDeclaration(
        ::std::boxed::Box<ExplicitObjectParameterDeclaration<'tree>>,
    ),
    OptionalParameterDeclaration(::std::boxed::Box<OptionalParameterDeclaration<'tree>>),
    ParameterDeclaration(::std::boxed::Box<ParameterDeclaration<'tree>>),
    VariadicParameterDeclaration(::std::boxed::Box<VariadicParameterDeclaration<'tree>>),
}
impl<'tree> ::treesitter_types::FromNode<'tree> for ParameterListChildren<'tree> {
    #[allow(clippy::collapsible_else_if)]
    fn from_node(
        node: ::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        match node.kind() {
            "explicit_object_parameter_declaration" => {
                Ok(
                    Self::ExplicitObjectParameterDeclaration(
                        ::std::boxed::Box::new(
                            <ExplicitObjectParameterDeclaration as ::treesitter_types::FromNode>::from_node(
                                node,
                                src,
                            )?,
                        ),
                    ),
                )
            }
            "optional_parameter_declaration" => {
                Ok(
                    Self::OptionalParameterDeclaration(
                        ::std::boxed::Box::new(
                            <OptionalParameterDeclaration as ::treesitter_types::FromNode>::from_node(
                                node,
                                src,
                            )?,
                        ),
                    ),
                )
            }
            "parameter_declaration" => {
                Ok(
                    Self::ParameterDeclaration(
                        ::std::boxed::Box::new(
                            <ParameterDeclaration as ::treesitter_types::FromNode>::from_node(
                                node,
                                src,
                            )?,
                        ),
                    ),
                )
            }
            "variadic_parameter_declaration" => {
                Ok(
                    Self::VariadicParameterDeclaration(
                        ::std::boxed::Box::new(
                            <VariadicParameterDeclaration as ::treesitter_types::FromNode>::from_node(
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
impl ::treesitter_types::Spanned for ParameterListChildren<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        match self {
            Self::ExplicitObjectParameterDeclaration(inner) => inner.span(),
            Self::OptionalParameterDeclaration(inner) => inner.span(),
            Self::ParameterDeclaration(inner) => inner.span(),
            Self::VariadicParameterDeclaration(inner) => inner.span(),
        }
    }
}
#[derive(Debug, Clone)]
pub enum ParameterPackExpansionPattern<'tree> {
    Expression(::std::boxed::Box<Expression<'tree>>),
    TypeDescriptor(::std::boxed::Box<TypeDescriptor<'tree>>),
}
impl<'tree> ::treesitter_types::FromNode<'tree> for ParameterPackExpansionPattern<'tree> {
    #[allow(clippy::collapsible_else_if)]
    fn from_node(
        node: ::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        match node.kind() {
            "type_descriptor" => Ok(Self::TypeDescriptor(::std::boxed::Box::new(
                <TypeDescriptor as ::treesitter_types::FromNode>::from_node(node, src)?,
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
impl ::treesitter_types::Spanned for ParameterPackExpansionPattern<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        match self {
            Self::Expression(inner) => inner.span(),
            Self::TypeDescriptor(inner) => inner.span(),
        }
    }
}
#[derive(Debug, Clone)]
pub enum ParenthesizedDeclaratorChildren<'tree> {
    Declarator(::std::boxed::Box<Declarator<'tree>>),
    FieldDeclarator(::std::boxed::Box<FieldDeclarator<'tree>>),
    TypeDeclarator(::std::boxed::Box<TypeDeclarator<'tree>>),
    MsCallModifier(::std::boxed::Box<MsCallModifier<'tree>>),
}
impl<'tree> ::treesitter_types::FromNode<'tree> for ParenthesizedDeclaratorChildren<'tree> {
    #[allow(clippy::collapsible_else_if)]
    fn from_node(
        node: ::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        match node.kind() {
            "ms_call_modifier" => Ok(Self::MsCallModifier(::std::boxed::Box::new(
                <MsCallModifier as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            _other => {
                if let Ok(v) = <Declarator as ::treesitter_types::FromNode>::from_node(node, src) {
                    Ok(Self::Declarator(::std::boxed::Box::new(v)))
                } else {
                    if let Ok(v) =
                        <FieldDeclarator as ::treesitter_types::FromNode>::from_node(node, src)
                    {
                        Ok(Self::FieldDeclarator(::std::boxed::Box::new(v)))
                    } else {
                        if let Ok(v) =
                            <TypeDeclarator as ::treesitter_types::FromNode>::from_node(node, src)
                        {
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
#[derive(Debug, Clone)]
pub enum ParenthesizedExpressionChildren<'tree> {
    CommaExpression(::std::boxed::Box<CommaExpression<'tree>>),
    CompoundStatement(::std::boxed::Box<CompoundStatement<'tree>>),
    Expression(::std::boxed::Box<Expression<'tree>>),
    PreprocDefined(::std::boxed::Box<PreprocDefined<'tree>>),
}
impl<'tree> ::treesitter_types::FromNode<'tree> for ParenthesizedExpressionChildren<'tree> {
    #[allow(clippy::collapsible_else_if)]
    fn from_node(
        node: ::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        match node.kind() {
            "comma_expression" => Ok(Self::CommaExpression(::std::boxed::Box::new(
                <CommaExpression as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "compound_statement" => Ok(Self::CompoundStatement(::std::boxed::Box::new(
                <CompoundStatement as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "preproc_defined" => Ok(Self::PreprocDefined(::std::boxed::Box::new(
                <PreprocDefined as ::treesitter_types::FromNode>::from_node(node, src)?,
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
            Self::CommaExpression(inner) => inner.span(),
            Self::CompoundStatement(inner) => inner.span(),
            Self::Expression(inner) => inner.span(),
            Self::PreprocDefined(inner) => inner.span(),
        }
    }
}
#[derive(Debug, Clone)]
pub enum PlaceholderTypeSpecifierConstraint<'tree> {
    QualifiedIdentifier(::std::boxed::Box<QualifiedIdentifier<'tree>>),
    TemplateType(::std::boxed::Box<TemplateType<'tree>>),
    TypeIdentifier(::std::boxed::Box<TypeIdentifier<'tree>>),
}
impl<'tree> ::treesitter_types::FromNode<'tree> for PlaceholderTypeSpecifierConstraint<'tree> {
    #[allow(clippy::collapsible_else_if)]
    fn from_node(
        node: ::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        match node.kind() {
            "qualified_identifier" => Ok(Self::QualifiedIdentifier(::std::boxed::Box::new(
                <QualifiedIdentifier as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "template_type" => Ok(Self::TemplateType(::std::boxed::Box::new(
                <TemplateType as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "type_identifier" => Ok(Self::TypeIdentifier(::std::boxed::Box::new(
                <TypeIdentifier as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            other => Err(::treesitter_types::ParseError::unexpected_kind(other, node)),
        }
    }
}
impl ::treesitter_types::Spanned for PlaceholderTypeSpecifierConstraint<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        match self {
            Self::QualifiedIdentifier(inner) => inner.span(),
            Self::TemplateType(inner) => inner.span(),
            Self::TypeIdentifier(inner) => inner.span(),
        }
    }
}
#[derive(Debug, Clone)]
pub enum PlaceholderTypeSpecifierChildren<'tree> {
    Auto(::std::boxed::Box<Auto<'tree>>),
    Decltype(::std::boxed::Box<Decltype<'tree>>),
}
impl<'tree> ::treesitter_types::FromNode<'tree> for PlaceholderTypeSpecifierChildren<'tree> {
    #[allow(clippy::collapsible_else_if)]
    fn from_node(
        node: ::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        match node.kind() {
            "auto" => Ok(Self::Auto(::std::boxed::Box::new(
                <Auto as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "decltype" => Ok(Self::Decltype(::std::boxed::Box::new(
                <Decltype as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            other => Err(::treesitter_types::ParseError::unexpected_kind(other, node)),
        }
    }
}
impl ::treesitter_types::Spanned for PlaceholderTypeSpecifierChildren<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        match self {
            Self::Auto(inner) => inner.span(),
            Self::Decltype(inner) => inner.span(),
        }
    }
}
#[derive(Debug, Clone)]
pub enum PointerDeclaratorDeclarator<'tree> {
    Declarator(::std::boxed::Box<Declarator<'tree>>),
    FieldDeclarator(::std::boxed::Box<FieldDeclarator<'tree>>),
    TypeDeclarator(::std::boxed::Box<TypeDeclarator<'tree>>),
}
impl<'tree> ::treesitter_types::FromNode<'tree> for PointerDeclaratorDeclarator<'tree> {
    #[allow(clippy::collapsible_else_if)]
    fn from_node(
        node: ::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        if let Ok(v) = <Declarator as ::treesitter_types::FromNode>::from_node(node, src) {
            Ok(Self::Declarator(::std::boxed::Box::new(v)))
        } else {
            if let Ok(v) = <FieldDeclarator as ::treesitter_types::FromNode>::from_node(node, src) {
                Ok(Self::FieldDeclarator(::std::boxed::Box::new(v)))
            } else {
                if let Ok(v) =
                    <TypeDeclarator as ::treesitter_types::FromNode>::from_node(node, src)
                {
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
#[derive(Debug, Clone)]
pub enum PointerDeclaratorChildren<'tree> {
    MsBasedModifier(::std::boxed::Box<MsBasedModifier<'tree>>),
    MsPointerModifier(::std::boxed::Box<MsPointerModifier<'tree>>),
    TypeQualifier(::std::boxed::Box<TypeQualifier<'tree>>),
}
impl<'tree> ::treesitter_types::FromNode<'tree> for PointerDeclaratorChildren<'tree> {
    #[allow(clippy::collapsible_else_if)]
    fn from_node(
        node: ::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        match node.kind() {
            "ms_based_modifier" => Ok(Self::MsBasedModifier(::std::boxed::Box::new(
                <MsBasedModifier as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "ms_pointer_modifier" => Ok(Self::MsPointerModifier(::std::boxed::Box::new(
                <MsPointerModifier as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "type_qualifier" => Ok(Self::TypeQualifier(::std::boxed::Box::new(
                <TypeQualifier as ::treesitter_types::FromNode>::from_node(node, src)?,
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
#[derive(Debug, Clone)]
pub enum PointerExpressionOperator {
    Amp(::treesitter_types::Span),
    Star(::treesitter_types::Span),
}
impl<'tree> ::treesitter_types::FromNode<'tree> for PointerExpressionOperator {
    #[allow(clippy::collapsible_else_if)]
    fn from_node(
        node: ::tree_sitter::Node<'tree>,
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
#[derive(Debug, Clone)]
pub enum PointerTypeDeclaratorChildren<'tree> {
    MsBasedModifier(::std::boxed::Box<MsBasedModifier<'tree>>),
    MsPointerModifier(::std::boxed::Box<MsPointerModifier<'tree>>),
    TypeQualifier(::std::boxed::Box<TypeQualifier<'tree>>),
}
impl<'tree> ::treesitter_types::FromNode<'tree> for PointerTypeDeclaratorChildren<'tree> {
    #[allow(clippy::collapsible_else_if)]
    fn from_node(
        node: ::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        match node.kind() {
            "ms_based_modifier" => Ok(Self::MsBasedModifier(::std::boxed::Box::new(
                <MsBasedModifier as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "ms_pointer_modifier" => Ok(Self::MsPointerModifier(::std::boxed::Box::new(
                <MsPointerModifier as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "type_qualifier" => Ok(Self::TypeQualifier(::std::boxed::Box::new(
                <TypeQualifier as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            other => Err(::treesitter_types::ParseError::unexpected_kind(other, node)),
        }
    }
}
impl ::treesitter_types::Spanned for PointerTypeDeclaratorChildren<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        match self {
            Self::MsBasedModifier(inner) => inner.span(),
            Self::MsPointerModifier(inner) => inner.span(),
            Self::TypeQualifier(inner) => inner.span(),
        }
    }
}
#[derive(Debug, Clone)]
pub enum PreprocElifAlternative<'tree> {
    PreprocElif(::std::boxed::Box<PreprocElif<'tree>>),
    PreprocElifdef(::std::boxed::Box<PreprocElifdef<'tree>>),
    PreprocElse(::std::boxed::Box<PreprocElse<'tree>>),
}
impl<'tree> ::treesitter_types::FromNode<'tree> for PreprocElifAlternative<'tree> {
    #[allow(clippy::collapsible_else_if)]
    fn from_node(
        node: ::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        match node.kind() {
            "preproc_elif" => Ok(Self::PreprocElif(::std::boxed::Box::new(
                <PreprocElif as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "preproc_elifdef" => Ok(Self::PreprocElifdef(::std::boxed::Box::new(
                <PreprocElifdef as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "preproc_else" => Ok(Self::PreprocElse(::std::boxed::Box::new(
                <PreprocElse as ::treesitter_types::FromNode>::from_node(node, src)?,
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
#[derive(Debug, Clone)]
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
            "char_literal" => Ok(Self::CharLiteral(::std::boxed::Box::new(
                <CharLiteral as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "identifier" => Ok(Self::Identifier(::std::boxed::Box::new(
                <Identifier as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "number_literal" => Ok(Self::NumberLiteral(::std::boxed::Box::new(
                <NumberLiteral as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "parenthesized_expression" => {
                Ok(Self::ParenthesizedExpression(::std::boxed::Box::new(
                    <ParenthesizedExpression as ::treesitter_types::FromNode>::from_node(
                        node, src,
                    )?,
                )))
            }
            "preproc_defined" => Ok(Self::PreprocDefined(::std::boxed::Box::new(
                <PreprocDefined as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "unary_expression" => Ok(Self::UnaryExpression(::std::boxed::Box::new(
                <UnaryExpression as ::treesitter_types::FromNode>::from_node(node, src)?,
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
#[derive(Debug, Clone)]
pub enum PreprocElifChildren<'tree> {
    AccessSpecifier(::std::boxed::Box<AccessSpecifier<'tree>>),
    AliasDeclaration(::std::boxed::Box<AliasDeclaration<'tree>>),
    ConceptDefinition(::std::boxed::Box<ConceptDefinition<'tree>>),
    ConstevalBlockDeclaration(::std::boxed::Box<ConstevalBlockDeclaration<'tree>>),
    Declaration(::std::boxed::Box<Declaration<'tree>>),
    Enumerator(::std::boxed::Box<Enumerator<'tree>>),
    ExportDeclaration(::std::boxed::Box<ExportDeclaration<'tree>>),
    FieldDeclaration(::std::boxed::Box<FieldDeclaration<'tree>>),
    FriendDeclaration(::std::boxed::Box<FriendDeclaration<'tree>>),
    FunctionDefinition(::std::boxed::Box<FunctionDefinition<'tree>>),
    GlobalModuleFragmentDeclaration(::std::boxed::Box<GlobalModuleFragmentDeclaration<'tree>>),
    ImportDeclaration(::std::boxed::Box<ImportDeclaration<'tree>>),
    LinkageSpecification(::std::boxed::Box<LinkageSpecification<'tree>>),
    ModuleDeclaration(::std::boxed::Box<ModuleDeclaration<'tree>>),
    NamespaceAliasDefinition(::std::boxed::Box<NamespaceAliasDefinition<'tree>>),
    NamespaceDefinition(::std::boxed::Box<NamespaceDefinition<'tree>>),
    PreprocCall(::std::boxed::Box<PreprocCall<'tree>>),
    PreprocDef(::std::boxed::Box<PreprocDef<'tree>>),
    PreprocFunctionDef(::std::boxed::Box<PreprocFunctionDef<'tree>>),
    PreprocIf(::std::boxed::Box<PreprocIf<'tree>>),
    PreprocIfdef(::std::boxed::Box<PreprocIfdef<'tree>>),
    PreprocInclude(::std::boxed::Box<PreprocInclude<'tree>>),
    PrivateModuleFragmentDeclaration(::std::boxed::Box<PrivateModuleFragmentDeclaration<'tree>>),
    Statement(::std::boxed::Box<Statement<'tree>>),
    StaticAssertDeclaration(::std::boxed::Box<StaticAssertDeclaration<'tree>>),
    TemplateDeclaration(::std::boxed::Box<TemplateDeclaration<'tree>>),
    TemplateInstantiation(::std::boxed::Box<TemplateInstantiation<'tree>>),
    TypeDefinition(::std::boxed::Box<TypeDefinition<'tree>>),
    TypeSpecifier(::std::boxed::Box<TypeSpecifier<'tree>>),
    UsingDeclaration(::std::boxed::Box<UsingDeclaration<'tree>>),
}
impl<'tree> ::treesitter_types::FromNode<'tree> for PreprocElifChildren<'tree> {
    #[allow(clippy::collapsible_else_if)]
    fn from_node(
        node: ::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        match node.kind() {
            "access_specifier" => Ok(Self::AccessSpecifier(::std::boxed::Box::new(
                <AccessSpecifier as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "alias_declaration" => Ok(Self::AliasDeclaration(::std::boxed::Box::new(
                <AliasDeclaration as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "concept_definition" => Ok(Self::ConceptDefinition(::std::boxed::Box::new(
                <ConceptDefinition as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "consteval_block_declaration" => {
                Ok(Self::ConstevalBlockDeclaration(::std::boxed::Box::new(
                    <ConstevalBlockDeclaration as ::treesitter_types::FromNode>::from_node(
                        node, src,
                    )?,
                )))
            }
            "declaration" => Ok(Self::Declaration(::std::boxed::Box::new(
                <Declaration as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "enumerator" => Ok(Self::Enumerator(::std::boxed::Box::new(
                <Enumerator as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "export_declaration" => Ok(Self::ExportDeclaration(::std::boxed::Box::new(
                <ExportDeclaration as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "field_declaration" => Ok(Self::FieldDeclaration(::std::boxed::Box::new(
                <FieldDeclaration as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "friend_declaration" => Ok(Self::FriendDeclaration(::std::boxed::Box::new(
                <FriendDeclaration as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "function_definition" => Ok(Self::FunctionDefinition(::std::boxed::Box::new(
                <FunctionDefinition as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "global_module_fragment_declaration" => Ok(Self::GlobalModuleFragmentDeclaration(
                ::std::boxed::Box::new(
                    <GlobalModuleFragmentDeclaration as ::treesitter_types::FromNode>::from_node(
                        node, src,
                    )?,
                ),
            )),
            "import_declaration" => Ok(Self::ImportDeclaration(::std::boxed::Box::new(
                <ImportDeclaration as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "linkage_specification" => Ok(Self::LinkageSpecification(::std::boxed::Box::new(
                <LinkageSpecification as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "module_declaration" => Ok(Self::ModuleDeclaration(::std::boxed::Box::new(
                <ModuleDeclaration as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "namespace_alias_definition" => {
                Ok(Self::NamespaceAliasDefinition(::std::boxed::Box::new(
                    <NamespaceAliasDefinition as ::treesitter_types::FromNode>::from_node(
                        node, src,
                    )?,
                )))
            }
            "namespace_definition" => Ok(Self::NamespaceDefinition(::std::boxed::Box::new(
                <NamespaceDefinition as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "preproc_call" => Ok(Self::PreprocCall(::std::boxed::Box::new(
                <PreprocCall as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "preproc_def" => Ok(Self::PreprocDef(::std::boxed::Box::new(
                <PreprocDef as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "preproc_function_def" => Ok(Self::PreprocFunctionDef(::std::boxed::Box::new(
                <PreprocFunctionDef as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "preproc_if" => Ok(Self::PreprocIf(::std::boxed::Box::new(
                <PreprocIf as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "preproc_ifdef" => Ok(Self::PreprocIfdef(::std::boxed::Box::new(
                <PreprocIfdef as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "preproc_include" => Ok(Self::PreprocInclude(::std::boxed::Box::new(
                <PreprocInclude as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "private_module_fragment_declaration" => Ok(Self::PrivateModuleFragmentDeclaration(
                ::std::boxed::Box::new(
                    <PrivateModuleFragmentDeclaration as ::treesitter_types::FromNode>::from_node(
                        node, src,
                    )?,
                ),
            )),
            "static_assert_declaration" => {
                Ok(Self::StaticAssertDeclaration(::std::boxed::Box::new(
                    <StaticAssertDeclaration as ::treesitter_types::FromNode>::from_node(
                        node, src,
                    )?,
                )))
            }
            "template_declaration" => Ok(Self::TemplateDeclaration(::std::boxed::Box::new(
                <TemplateDeclaration as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "template_instantiation" => Ok(Self::TemplateInstantiation(::std::boxed::Box::new(
                <TemplateInstantiation as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "type_definition" => Ok(Self::TypeDefinition(::std::boxed::Box::new(
                <TypeDefinition as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "using_declaration" => Ok(Self::UsingDeclaration(::std::boxed::Box::new(
                <UsingDeclaration as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            _other => {
                if let Ok(v) = <Statement as ::treesitter_types::FromNode>::from_node(node, src) {
                    Ok(Self::Statement(::std::boxed::Box::new(v)))
                } else {
                    if let Ok(v) =
                        <TypeSpecifier as ::treesitter_types::FromNode>::from_node(node, src)
                    {
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
            Self::AccessSpecifier(inner) => inner.span(),
            Self::AliasDeclaration(inner) => inner.span(),
            Self::ConceptDefinition(inner) => inner.span(),
            Self::ConstevalBlockDeclaration(inner) => inner.span(),
            Self::Declaration(inner) => inner.span(),
            Self::Enumerator(inner) => inner.span(),
            Self::ExportDeclaration(inner) => inner.span(),
            Self::FieldDeclaration(inner) => inner.span(),
            Self::FriendDeclaration(inner) => inner.span(),
            Self::FunctionDefinition(inner) => inner.span(),
            Self::GlobalModuleFragmentDeclaration(inner) => inner.span(),
            Self::ImportDeclaration(inner) => inner.span(),
            Self::LinkageSpecification(inner) => inner.span(),
            Self::ModuleDeclaration(inner) => inner.span(),
            Self::NamespaceAliasDefinition(inner) => inner.span(),
            Self::NamespaceDefinition(inner) => inner.span(),
            Self::PreprocCall(inner) => inner.span(),
            Self::PreprocDef(inner) => inner.span(),
            Self::PreprocFunctionDef(inner) => inner.span(),
            Self::PreprocIf(inner) => inner.span(),
            Self::PreprocIfdef(inner) => inner.span(),
            Self::PreprocInclude(inner) => inner.span(),
            Self::PrivateModuleFragmentDeclaration(inner) => inner.span(),
            Self::Statement(inner) => inner.span(),
            Self::StaticAssertDeclaration(inner) => inner.span(),
            Self::TemplateDeclaration(inner) => inner.span(),
            Self::TemplateInstantiation(inner) => inner.span(),
            Self::TypeDefinition(inner) => inner.span(),
            Self::TypeSpecifier(inner) => inner.span(),
            Self::UsingDeclaration(inner) => inner.span(),
        }
    }
}
#[derive(Debug, Clone)]
pub enum PreprocElifdefAlternative<'tree> {
    PreprocElif(::std::boxed::Box<PreprocElif<'tree>>),
    PreprocElifdef(::std::boxed::Box<PreprocElifdef<'tree>>),
    PreprocElse(::std::boxed::Box<PreprocElse<'tree>>),
}
impl<'tree> ::treesitter_types::FromNode<'tree> for PreprocElifdefAlternative<'tree> {
    #[allow(clippy::collapsible_else_if)]
    fn from_node(
        node: ::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        match node.kind() {
            "preproc_elif" => Ok(Self::PreprocElif(::std::boxed::Box::new(
                <PreprocElif as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "preproc_elifdef" => Ok(Self::PreprocElifdef(::std::boxed::Box::new(
                <PreprocElifdef as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "preproc_else" => Ok(Self::PreprocElse(::std::boxed::Box::new(
                <PreprocElse as ::treesitter_types::FromNode>::from_node(node, src)?,
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
#[derive(Debug, Clone)]
pub enum PreprocElifdefChildren<'tree> {
    AccessSpecifier(::std::boxed::Box<AccessSpecifier<'tree>>),
    AliasDeclaration(::std::boxed::Box<AliasDeclaration<'tree>>),
    ConceptDefinition(::std::boxed::Box<ConceptDefinition<'tree>>),
    ConstevalBlockDeclaration(::std::boxed::Box<ConstevalBlockDeclaration<'tree>>),
    Declaration(::std::boxed::Box<Declaration<'tree>>),
    Enumerator(::std::boxed::Box<Enumerator<'tree>>),
    ExportDeclaration(::std::boxed::Box<ExportDeclaration<'tree>>),
    FieldDeclaration(::std::boxed::Box<FieldDeclaration<'tree>>),
    FriendDeclaration(::std::boxed::Box<FriendDeclaration<'tree>>),
    FunctionDefinition(::std::boxed::Box<FunctionDefinition<'tree>>),
    GlobalModuleFragmentDeclaration(::std::boxed::Box<GlobalModuleFragmentDeclaration<'tree>>),
    ImportDeclaration(::std::boxed::Box<ImportDeclaration<'tree>>),
    LinkageSpecification(::std::boxed::Box<LinkageSpecification<'tree>>),
    ModuleDeclaration(::std::boxed::Box<ModuleDeclaration<'tree>>),
    NamespaceAliasDefinition(::std::boxed::Box<NamespaceAliasDefinition<'tree>>),
    NamespaceDefinition(::std::boxed::Box<NamespaceDefinition<'tree>>),
    PreprocCall(::std::boxed::Box<PreprocCall<'tree>>),
    PreprocDef(::std::boxed::Box<PreprocDef<'tree>>),
    PreprocFunctionDef(::std::boxed::Box<PreprocFunctionDef<'tree>>),
    PreprocIf(::std::boxed::Box<PreprocIf<'tree>>),
    PreprocIfdef(::std::boxed::Box<PreprocIfdef<'tree>>),
    PreprocInclude(::std::boxed::Box<PreprocInclude<'tree>>),
    PrivateModuleFragmentDeclaration(::std::boxed::Box<PrivateModuleFragmentDeclaration<'tree>>),
    Statement(::std::boxed::Box<Statement<'tree>>),
    StaticAssertDeclaration(::std::boxed::Box<StaticAssertDeclaration<'tree>>),
    TemplateDeclaration(::std::boxed::Box<TemplateDeclaration<'tree>>),
    TemplateInstantiation(::std::boxed::Box<TemplateInstantiation<'tree>>),
    TypeDefinition(::std::boxed::Box<TypeDefinition<'tree>>),
    TypeSpecifier(::std::boxed::Box<TypeSpecifier<'tree>>),
    UsingDeclaration(::std::boxed::Box<UsingDeclaration<'tree>>),
}
impl<'tree> ::treesitter_types::FromNode<'tree> for PreprocElifdefChildren<'tree> {
    #[allow(clippy::collapsible_else_if)]
    fn from_node(
        node: ::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        match node.kind() {
            "access_specifier" => Ok(Self::AccessSpecifier(::std::boxed::Box::new(
                <AccessSpecifier as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "alias_declaration" => Ok(Self::AliasDeclaration(::std::boxed::Box::new(
                <AliasDeclaration as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "concept_definition" => Ok(Self::ConceptDefinition(::std::boxed::Box::new(
                <ConceptDefinition as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "consteval_block_declaration" => {
                Ok(Self::ConstevalBlockDeclaration(::std::boxed::Box::new(
                    <ConstevalBlockDeclaration as ::treesitter_types::FromNode>::from_node(
                        node, src,
                    )?,
                )))
            }
            "declaration" => Ok(Self::Declaration(::std::boxed::Box::new(
                <Declaration as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "enumerator" => Ok(Self::Enumerator(::std::boxed::Box::new(
                <Enumerator as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "export_declaration" => Ok(Self::ExportDeclaration(::std::boxed::Box::new(
                <ExportDeclaration as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "field_declaration" => Ok(Self::FieldDeclaration(::std::boxed::Box::new(
                <FieldDeclaration as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "friend_declaration" => Ok(Self::FriendDeclaration(::std::boxed::Box::new(
                <FriendDeclaration as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "function_definition" => Ok(Self::FunctionDefinition(::std::boxed::Box::new(
                <FunctionDefinition as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "global_module_fragment_declaration" => Ok(Self::GlobalModuleFragmentDeclaration(
                ::std::boxed::Box::new(
                    <GlobalModuleFragmentDeclaration as ::treesitter_types::FromNode>::from_node(
                        node, src,
                    )?,
                ),
            )),
            "import_declaration" => Ok(Self::ImportDeclaration(::std::boxed::Box::new(
                <ImportDeclaration as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "linkage_specification" => Ok(Self::LinkageSpecification(::std::boxed::Box::new(
                <LinkageSpecification as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "module_declaration" => Ok(Self::ModuleDeclaration(::std::boxed::Box::new(
                <ModuleDeclaration as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "namespace_alias_definition" => {
                Ok(Self::NamespaceAliasDefinition(::std::boxed::Box::new(
                    <NamespaceAliasDefinition as ::treesitter_types::FromNode>::from_node(
                        node, src,
                    )?,
                )))
            }
            "namespace_definition" => Ok(Self::NamespaceDefinition(::std::boxed::Box::new(
                <NamespaceDefinition as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "preproc_call" => Ok(Self::PreprocCall(::std::boxed::Box::new(
                <PreprocCall as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "preproc_def" => Ok(Self::PreprocDef(::std::boxed::Box::new(
                <PreprocDef as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "preproc_function_def" => Ok(Self::PreprocFunctionDef(::std::boxed::Box::new(
                <PreprocFunctionDef as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "preproc_if" => Ok(Self::PreprocIf(::std::boxed::Box::new(
                <PreprocIf as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "preproc_ifdef" => Ok(Self::PreprocIfdef(::std::boxed::Box::new(
                <PreprocIfdef as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "preproc_include" => Ok(Self::PreprocInclude(::std::boxed::Box::new(
                <PreprocInclude as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "private_module_fragment_declaration" => Ok(Self::PrivateModuleFragmentDeclaration(
                ::std::boxed::Box::new(
                    <PrivateModuleFragmentDeclaration as ::treesitter_types::FromNode>::from_node(
                        node, src,
                    )?,
                ),
            )),
            "static_assert_declaration" => {
                Ok(Self::StaticAssertDeclaration(::std::boxed::Box::new(
                    <StaticAssertDeclaration as ::treesitter_types::FromNode>::from_node(
                        node, src,
                    )?,
                )))
            }
            "template_declaration" => Ok(Self::TemplateDeclaration(::std::boxed::Box::new(
                <TemplateDeclaration as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "template_instantiation" => Ok(Self::TemplateInstantiation(::std::boxed::Box::new(
                <TemplateInstantiation as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "type_definition" => Ok(Self::TypeDefinition(::std::boxed::Box::new(
                <TypeDefinition as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "using_declaration" => Ok(Self::UsingDeclaration(::std::boxed::Box::new(
                <UsingDeclaration as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            _other => {
                if let Ok(v) = <Statement as ::treesitter_types::FromNode>::from_node(node, src) {
                    Ok(Self::Statement(::std::boxed::Box::new(v)))
                } else {
                    if let Ok(v) =
                        <TypeSpecifier as ::treesitter_types::FromNode>::from_node(node, src)
                    {
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
            Self::AccessSpecifier(inner) => inner.span(),
            Self::AliasDeclaration(inner) => inner.span(),
            Self::ConceptDefinition(inner) => inner.span(),
            Self::ConstevalBlockDeclaration(inner) => inner.span(),
            Self::Declaration(inner) => inner.span(),
            Self::Enumerator(inner) => inner.span(),
            Self::ExportDeclaration(inner) => inner.span(),
            Self::FieldDeclaration(inner) => inner.span(),
            Self::FriendDeclaration(inner) => inner.span(),
            Self::FunctionDefinition(inner) => inner.span(),
            Self::GlobalModuleFragmentDeclaration(inner) => inner.span(),
            Self::ImportDeclaration(inner) => inner.span(),
            Self::LinkageSpecification(inner) => inner.span(),
            Self::ModuleDeclaration(inner) => inner.span(),
            Self::NamespaceAliasDefinition(inner) => inner.span(),
            Self::NamespaceDefinition(inner) => inner.span(),
            Self::PreprocCall(inner) => inner.span(),
            Self::PreprocDef(inner) => inner.span(),
            Self::PreprocFunctionDef(inner) => inner.span(),
            Self::PreprocIf(inner) => inner.span(),
            Self::PreprocIfdef(inner) => inner.span(),
            Self::PreprocInclude(inner) => inner.span(),
            Self::PrivateModuleFragmentDeclaration(inner) => inner.span(),
            Self::Statement(inner) => inner.span(),
            Self::StaticAssertDeclaration(inner) => inner.span(),
            Self::TemplateDeclaration(inner) => inner.span(),
            Self::TemplateInstantiation(inner) => inner.span(),
            Self::TypeDefinition(inner) => inner.span(),
            Self::TypeSpecifier(inner) => inner.span(),
            Self::UsingDeclaration(inner) => inner.span(),
        }
    }
}
#[derive(Debug, Clone)]
pub enum PreprocElseChildren<'tree> {
    AccessSpecifier(::std::boxed::Box<AccessSpecifier<'tree>>),
    AliasDeclaration(::std::boxed::Box<AliasDeclaration<'tree>>),
    ConceptDefinition(::std::boxed::Box<ConceptDefinition<'tree>>),
    ConstevalBlockDeclaration(::std::boxed::Box<ConstevalBlockDeclaration<'tree>>),
    Declaration(::std::boxed::Box<Declaration<'tree>>),
    Enumerator(::std::boxed::Box<Enumerator<'tree>>),
    ExportDeclaration(::std::boxed::Box<ExportDeclaration<'tree>>),
    FieldDeclaration(::std::boxed::Box<FieldDeclaration<'tree>>),
    FriendDeclaration(::std::boxed::Box<FriendDeclaration<'tree>>),
    FunctionDefinition(::std::boxed::Box<FunctionDefinition<'tree>>),
    GlobalModuleFragmentDeclaration(::std::boxed::Box<GlobalModuleFragmentDeclaration<'tree>>),
    ImportDeclaration(::std::boxed::Box<ImportDeclaration<'tree>>),
    LinkageSpecification(::std::boxed::Box<LinkageSpecification<'tree>>),
    ModuleDeclaration(::std::boxed::Box<ModuleDeclaration<'tree>>),
    NamespaceAliasDefinition(::std::boxed::Box<NamespaceAliasDefinition<'tree>>),
    NamespaceDefinition(::std::boxed::Box<NamespaceDefinition<'tree>>),
    PreprocCall(::std::boxed::Box<PreprocCall<'tree>>),
    PreprocDef(::std::boxed::Box<PreprocDef<'tree>>),
    PreprocFunctionDef(::std::boxed::Box<PreprocFunctionDef<'tree>>),
    PreprocIf(::std::boxed::Box<PreprocIf<'tree>>),
    PreprocIfdef(::std::boxed::Box<PreprocIfdef<'tree>>),
    PreprocInclude(::std::boxed::Box<PreprocInclude<'tree>>),
    PrivateModuleFragmentDeclaration(::std::boxed::Box<PrivateModuleFragmentDeclaration<'tree>>),
    Statement(::std::boxed::Box<Statement<'tree>>),
    StaticAssertDeclaration(::std::boxed::Box<StaticAssertDeclaration<'tree>>),
    TemplateDeclaration(::std::boxed::Box<TemplateDeclaration<'tree>>),
    TemplateInstantiation(::std::boxed::Box<TemplateInstantiation<'tree>>),
    TypeDefinition(::std::boxed::Box<TypeDefinition<'tree>>),
    TypeSpecifier(::std::boxed::Box<TypeSpecifier<'tree>>),
    UsingDeclaration(::std::boxed::Box<UsingDeclaration<'tree>>),
}
impl<'tree> ::treesitter_types::FromNode<'tree> for PreprocElseChildren<'tree> {
    #[allow(clippy::collapsible_else_if)]
    fn from_node(
        node: ::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        match node.kind() {
            "access_specifier" => Ok(Self::AccessSpecifier(::std::boxed::Box::new(
                <AccessSpecifier as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "alias_declaration" => Ok(Self::AliasDeclaration(::std::boxed::Box::new(
                <AliasDeclaration as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "concept_definition" => Ok(Self::ConceptDefinition(::std::boxed::Box::new(
                <ConceptDefinition as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "consteval_block_declaration" => {
                Ok(Self::ConstevalBlockDeclaration(::std::boxed::Box::new(
                    <ConstevalBlockDeclaration as ::treesitter_types::FromNode>::from_node(
                        node, src,
                    )?,
                )))
            }
            "declaration" => Ok(Self::Declaration(::std::boxed::Box::new(
                <Declaration as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "enumerator" => Ok(Self::Enumerator(::std::boxed::Box::new(
                <Enumerator as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "export_declaration" => Ok(Self::ExportDeclaration(::std::boxed::Box::new(
                <ExportDeclaration as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "field_declaration" => Ok(Self::FieldDeclaration(::std::boxed::Box::new(
                <FieldDeclaration as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "friend_declaration" => Ok(Self::FriendDeclaration(::std::boxed::Box::new(
                <FriendDeclaration as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "function_definition" => Ok(Self::FunctionDefinition(::std::boxed::Box::new(
                <FunctionDefinition as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "global_module_fragment_declaration" => Ok(Self::GlobalModuleFragmentDeclaration(
                ::std::boxed::Box::new(
                    <GlobalModuleFragmentDeclaration as ::treesitter_types::FromNode>::from_node(
                        node, src,
                    )?,
                ),
            )),
            "import_declaration" => Ok(Self::ImportDeclaration(::std::boxed::Box::new(
                <ImportDeclaration as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "linkage_specification" => Ok(Self::LinkageSpecification(::std::boxed::Box::new(
                <LinkageSpecification as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "module_declaration" => Ok(Self::ModuleDeclaration(::std::boxed::Box::new(
                <ModuleDeclaration as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "namespace_alias_definition" => {
                Ok(Self::NamespaceAliasDefinition(::std::boxed::Box::new(
                    <NamespaceAliasDefinition as ::treesitter_types::FromNode>::from_node(
                        node, src,
                    )?,
                )))
            }
            "namespace_definition" => Ok(Self::NamespaceDefinition(::std::boxed::Box::new(
                <NamespaceDefinition as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "preproc_call" => Ok(Self::PreprocCall(::std::boxed::Box::new(
                <PreprocCall as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "preproc_def" => Ok(Self::PreprocDef(::std::boxed::Box::new(
                <PreprocDef as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "preproc_function_def" => Ok(Self::PreprocFunctionDef(::std::boxed::Box::new(
                <PreprocFunctionDef as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "preproc_if" => Ok(Self::PreprocIf(::std::boxed::Box::new(
                <PreprocIf as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "preproc_ifdef" => Ok(Self::PreprocIfdef(::std::boxed::Box::new(
                <PreprocIfdef as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "preproc_include" => Ok(Self::PreprocInclude(::std::boxed::Box::new(
                <PreprocInclude as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "private_module_fragment_declaration" => Ok(Self::PrivateModuleFragmentDeclaration(
                ::std::boxed::Box::new(
                    <PrivateModuleFragmentDeclaration as ::treesitter_types::FromNode>::from_node(
                        node, src,
                    )?,
                ),
            )),
            "static_assert_declaration" => {
                Ok(Self::StaticAssertDeclaration(::std::boxed::Box::new(
                    <StaticAssertDeclaration as ::treesitter_types::FromNode>::from_node(
                        node, src,
                    )?,
                )))
            }
            "template_declaration" => Ok(Self::TemplateDeclaration(::std::boxed::Box::new(
                <TemplateDeclaration as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "template_instantiation" => Ok(Self::TemplateInstantiation(::std::boxed::Box::new(
                <TemplateInstantiation as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "type_definition" => Ok(Self::TypeDefinition(::std::boxed::Box::new(
                <TypeDefinition as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "using_declaration" => Ok(Self::UsingDeclaration(::std::boxed::Box::new(
                <UsingDeclaration as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            _other => {
                if let Ok(v) = <Statement as ::treesitter_types::FromNode>::from_node(node, src) {
                    Ok(Self::Statement(::std::boxed::Box::new(v)))
                } else {
                    if let Ok(v) =
                        <TypeSpecifier as ::treesitter_types::FromNode>::from_node(node, src)
                    {
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
            Self::AccessSpecifier(inner) => inner.span(),
            Self::AliasDeclaration(inner) => inner.span(),
            Self::ConceptDefinition(inner) => inner.span(),
            Self::ConstevalBlockDeclaration(inner) => inner.span(),
            Self::Declaration(inner) => inner.span(),
            Self::Enumerator(inner) => inner.span(),
            Self::ExportDeclaration(inner) => inner.span(),
            Self::FieldDeclaration(inner) => inner.span(),
            Self::FriendDeclaration(inner) => inner.span(),
            Self::FunctionDefinition(inner) => inner.span(),
            Self::GlobalModuleFragmentDeclaration(inner) => inner.span(),
            Self::ImportDeclaration(inner) => inner.span(),
            Self::LinkageSpecification(inner) => inner.span(),
            Self::ModuleDeclaration(inner) => inner.span(),
            Self::NamespaceAliasDefinition(inner) => inner.span(),
            Self::NamespaceDefinition(inner) => inner.span(),
            Self::PreprocCall(inner) => inner.span(),
            Self::PreprocDef(inner) => inner.span(),
            Self::PreprocFunctionDef(inner) => inner.span(),
            Self::PreprocIf(inner) => inner.span(),
            Self::PreprocIfdef(inner) => inner.span(),
            Self::PreprocInclude(inner) => inner.span(),
            Self::PrivateModuleFragmentDeclaration(inner) => inner.span(),
            Self::Statement(inner) => inner.span(),
            Self::StaticAssertDeclaration(inner) => inner.span(),
            Self::TemplateDeclaration(inner) => inner.span(),
            Self::TemplateInstantiation(inner) => inner.span(),
            Self::TypeDefinition(inner) => inner.span(),
            Self::TypeSpecifier(inner) => inner.span(),
            Self::UsingDeclaration(inner) => inner.span(),
        }
    }
}
#[derive(Debug, Clone)]
pub enum PreprocIfAlternative<'tree> {
    PreprocElif(::std::boxed::Box<PreprocElif<'tree>>),
    PreprocElifdef(::std::boxed::Box<PreprocElifdef<'tree>>),
    PreprocElse(::std::boxed::Box<PreprocElse<'tree>>),
}
impl<'tree> ::treesitter_types::FromNode<'tree> for PreprocIfAlternative<'tree> {
    #[allow(clippy::collapsible_else_if)]
    fn from_node(
        node: ::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        match node.kind() {
            "preproc_elif" => Ok(Self::PreprocElif(::std::boxed::Box::new(
                <PreprocElif as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "preproc_elifdef" => Ok(Self::PreprocElifdef(::std::boxed::Box::new(
                <PreprocElifdef as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "preproc_else" => Ok(Self::PreprocElse(::std::boxed::Box::new(
                <PreprocElse as ::treesitter_types::FromNode>::from_node(node, src)?,
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
#[derive(Debug, Clone)]
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
            "char_literal" => Ok(Self::CharLiteral(::std::boxed::Box::new(
                <CharLiteral as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "identifier" => Ok(Self::Identifier(::std::boxed::Box::new(
                <Identifier as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "number_literal" => Ok(Self::NumberLiteral(::std::boxed::Box::new(
                <NumberLiteral as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "parenthesized_expression" => {
                Ok(Self::ParenthesizedExpression(::std::boxed::Box::new(
                    <ParenthesizedExpression as ::treesitter_types::FromNode>::from_node(
                        node, src,
                    )?,
                )))
            }
            "preproc_defined" => Ok(Self::PreprocDefined(::std::boxed::Box::new(
                <PreprocDefined as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "unary_expression" => Ok(Self::UnaryExpression(::std::boxed::Box::new(
                <UnaryExpression as ::treesitter_types::FromNode>::from_node(node, src)?,
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
#[derive(Debug, Clone)]
pub enum PreprocIfChildren<'tree> {
    AccessSpecifier(::std::boxed::Box<AccessSpecifier<'tree>>),
    AliasDeclaration(::std::boxed::Box<AliasDeclaration<'tree>>),
    ConceptDefinition(::std::boxed::Box<ConceptDefinition<'tree>>),
    ConstevalBlockDeclaration(::std::boxed::Box<ConstevalBlockDeclaration<'tree>>),
    Declaration(::std::boxed::Box<Declaration<'tree>>),
    Enumerator(::std::boxed::Box<Enumerator<'tree>>),
    ExportDeclaration(::std::boxed::Box<ExportDeclaration<'tree>>),
    FieldDeclaration(::std::boxed::Box<FieldDeclaration<'tree>>),
    FriendDeclaration(::std::boxed::Box<FriendDeclaration<'tree>>),
    FunctionDefinition(::std::boxed::Box<FunctionDefinition<'tree>>),
    GlobalModuleFragmentDeclaration(::std::boxed::Box<GlobalModuleFragmentDeclaration<'tree>>),
    ImportDeclaration(::std::boxed::Box<ImportDeclaration<'tree>>),
    LinkageSpecification(::std::boxed::Box<LinkageSpecification<'tree>>),
    ModuleDeclaration(::std::boxed::Box<ModuleDeclaration<'tree>>),
    NamespaceAliasDefinition(::std::boxed::Box<NamespaceAliasDefinition<'tree>>),
    NamespaceDefinition(::std::boxed::Box<NamespaceDefinition<'tree>>),
    PreprocCall(::std::boxed::Box<PreprocCall<'tree>>),
    PreprocDef(::std::boxed::Box<PreprocDef<'tree>>),
    PreprocFunctionDef(::std::boxed::Box<PreprocFunctionDef<'tree>>),
    PreprocIf(::std::boxed::Box<PreprocIf<'tree>>),
    PreprocIfdef(::std::boxed::Box<PreprocIfdef<'tree>>),
    PreprocInclude(::std::boxed::Box<PreprocInclude<'tree>>),
    PrivateModuleFragmentDeclaration(::std::boxed::Box<PrivateModuleFragmentDeclaration<'tree>>),
    Statement(::std::boxed::Box<Statement<'tree>>),
    StaticAssertDeclaration(::std::boxed::Box<StaticAssertDeclaration<'tree>>),
    TemplateDeclaration(::std::boxed::Box<TemplateDeclaration<'tree>>),
    TemplateInstantiation(::std::boxed::Box<TemplateInstantiation<'tree>>),
    TypeDefinition(::std::boxed::Box<TypeDefinition<'tree>>),
    TypeSpecifier(::std::boxed::Box<TypeSpecifier<'tree>>),
    UsingDeclaration(::std::boxed::Box<UsingDeclaration<'tree>>),
}
impl<'tree> ::treesitter_types::FromNode<'tree> for PreprocIfChildren<'tree> {
    #[allow(clippy::collapsible_else_if)]
    fn from_node(
        node: ::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        match node.kind() {
            "access_specifier" => Ok(Self::AccessSpecifier(::std::boxed::Box::new(
                <AccessSpecifier as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "alias_declaration" => Ok(Self::AliasDeclaration(::std::boxed::Box::new(
                <AliasDeclaration as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "concept_definition" => Ok(Self::ConceptDefinition(::std::boxed::Box::new(
                <ConceptDefinition as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "consteval_block_declaration" => {
                Ok(Self::ConstevalBlockDeclaration(::std::boxed::Box::new(
                    <ConstevalBlockDeclaration as ::treesitter_types::FromNode>::from_node(
                        node, src,
                    )?,
                )))
            }
            "declaration" => Ok(Self::Declaration(::std::boxed::Box::new(
                <Declaration as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "enumerator" => Ok(Self::Enumerator(::std::boxed::Box::new(
                <Enumerator as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "export_declaration" => Ok(Self::ExportDeclaration(::std::boxed::Box::new(
                <ExportDeclaration as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "field_declaration" => Ok(Self::FieldDeclaration(::std::boxed::Box::new(
                <FieldDeclaration as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "friend_declaration" => Ok(Self::FriendDeclaration(::std::boxed::Box::new(
                <FriendDeclaration as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "function_definition" => Ok(Self::FunctionDefinition(::std::boxed::Box::new(
                <FunctionDefinition as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "global_module_fragment_declaration" => Ok(Self::GlobalModuleFragmentDeclaration(
                ::std::boxed::Box::new(
                    <GlobalModuleFragmentDeclaration as ::treesitter_types::FromNode>::from_node(
                        node, src,
                    )?,
                ),
            )),
            "import_declaration" => Ok(Self::ImportDeclaration(::std::boxed::Box::new(
                <ImportDeclaration as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "linkage_specification" => Ok(Self::LinkageSpecification(::std::boxed::Box::new(
                <LinkageSpecification as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "module_declaration" => Ok(Self::ModuleDeclaration(::std::boxed::Box::new(
                <ModuleDeclaration as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "namespace_alias_definition" => {
                Ok(Self::NamespaceAliasDefinition(::std::boxed::Box::new(
                    <NamespaceAliasDefinition as ::treesitter_types::FromNode>::from_node(
                        node, src,
                    )?,
                )))
            }
            "namespace_definition" => Ok(Self::NamespaceDefinition(::std::boxed::Box::new(
                <NamespaceDefinition as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "preproc_call" => Ok(Self::PreprocCall(::std::boxed::Box::new(
                <PreprocCall as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "preproc_def" => Ok(Self::PreprocDef(::std::boxed::Box::new(
                <PreprocDef as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "preproc_function_def" => Ok(Self::PreprocFunctionDef(::std::boxed::Box::new(
                <PreprocFunctionDef as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "preproc_if" => Ok(Self::PreprocIf(::std::boxed::Box::new(
                <PreprocIf as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "preproc_ifdef" => Ok(Self::PreprocIfdef(::std::boxed::Box::new(
                <PreprocIfdef as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "preproc_include" => Ok(Self::PreprocInclude(::std::boxed::Box::new(
                <PreprocInclude as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "private_module_fragment_declaration" => Ok(Self::PrivateModuleFragmentDeclaration(
                ::std::boxed::Box::new(
                    <PrivateModuleFragmentDeclaration as ::treesitter_types::FromNode>::from_node(
                        node, src,
                    )?,
                ),
            )),
            "static_assert_declaration" => {
                Ok(Self::StaticAssertDeclaration(::std::boxed::Box::new(
                    <StaticAssertDeclaration as ::treesitter_types::FromNode>::from_node(
                        node, src,
                    )?,
                )))
            }
            "template_declaration" => Ok(Self::TemplateDeclaration(::std::boxed::Box::new(
                <TemplateDeclaration as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "template_instantiation" => Ok(Self::TemplateInstantiation(::std::boxed::Box::new(
                <TemplateInstantiation as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "type_definition" => Ok(Self::TypeDefinition(::std::boxed::Box::new(
                <TypeDefinition as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "using_declaration" => Ok(Self::UsingDeclaration(::std::boxed::Box::new(
                <UsingDeclaration as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            _other => {
                if let Ok(v) = <Statement as ::treesitter_types::FromNode>::from_node(node, src) {
                    Ok(Self::Statement(::std::boxed::Box::new(v)))
                } else {
                    if let Ok(v) =
                        <TypeSpecifier as ::treesitter_types::FromNode>::from_node(node, src)
                    {
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
            Self::AccessSpecifier(inner) => inner.span(),
            Self::AliasDeclaration(inner) => inner.span(),
            Self::ConceptDefinition(inner) => inner.span(),
            Self::ConstevalBlockDeclaration(inner) => inner.span(),
            Self::Declaration(inner) => inner.span(),
            Self::Enumerator(inner) => inner.span(),
            Self::ExportDeclaration(inner) => inner.span(),
            Self::FieldDeclaration(inner) => inner.span(),
            Self::FriendDeclaration(inner) => inner.span(),
            Self::FunctionDefinition(inner) => inner.span(),
            Self::GlobalModuleFragmentDeclaration(inner) => inner.span(),
            Self::ImportDeclaration(inner) => inner.span(),
            Self::LinkageSpecification(inner) => inner.span(),
            Self::ModuleDeclaration(inner) => inner.span(),
            Self::NamespaceAliasDefinition(inner) => inner.span(),
            Self::NamespaceDefinition(inner) => inner.span(),
            Self::PreprocCall(inner) => inner.span(),
            Self::PreprocDef(inner) => inner.span(),
            Self::PreprocFunctionDef(inner) => inner.span(),
            Self::PreprocIf(inner) => inner.span(),
            Self::PreprocIfdef(inner) => inner.span(),
            Self::PreprocInclude(inner) => inner.span(),
            Self::PrivateModuleFragmentDeclaration(inner) => inner.span(),
            Self::Statement(inner) => inner.span(),
            Self::StaticAssertDeclaration(inner) => inner.span(),
            Self::TemplateDeclaration(inner) => inner.span(),
            Self::TemplateInstantiation(inner) => inner.span(),
            Self::TypeDefinition(inner) => inner.span(),
            Self::TypeSpecifier(inner) => inner.span(),
            Self::UsingDeclaration(inner) => inner.span(),
        }
    }
}
#[derive(Debug, Clone)]
pub enum PreprocIfdefAlternative<'tree> {
    PreprocElif(::std::boxed::Box<PreprocElif<'tree>>),
    PreprocElifdef(::std::boxed::Box<PreprocElifdef<'tree>>),
    PreprocElse(::std::boxed::Box<PreprocElse<'tree>>),
}
impl<'tree> ::treesitter_types::FromNode<'tree> for PreprocIfdefAlternative<'tree> {
    #[allow(clippy::collapsible_else_if)]
    fn from_node(
        node: ::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        match node.kind() {
            "preproc_elif" => Ok(Self::PreprocElif(::std::boxed::Box::new(
                <PreprocElif as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "preproc_elifdef" => Ok(Self::PreprocElifdef(::std::boxed::Box::new(
                <PreprocElifdef as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "preproc_else" => Ok(Self::PreprocElse(::std::boxed::Box::new(
                <PreprocElse as ::treesitter_types::FromNode>::from_node(node, src)?,
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
#[derive(Debug, Clone)]
pub enum PreprocIfdefChildren<'tree> {
    AccessSpecifier(::std::boxed::Box<AccessSpecifier<'tree>>),
    AliasDeclaration(::std::boxed::Box<AliasDeclaration<'tree>>),
    ConceptDefinition(::std::boxed::Box<ConceptDefinition<'tree>>),
    ConstevalBlockDeclaration(::std::boxed::Box<ConstevalBlockDeclaration<'tree>>),
    Declaration(::std::boxed::Box<Declaration<'tree>>),
    Enumerator(::std::boxed::Box<Enumerator<'tree>>),
    ExportDeclaration(::std::boxed::Box<ExportDeclaration<'tree>>),
    FieldDeclaration(::std::boxed::Box<FieldDeclaration<'tree>>),
    FriendDeclaration(::std::boxed::Box<FriendDeclaration<'tree>>),
    FunctionDefinition(::std::boxed::Box<FunctionDefinition<'tree>>),
    GlobalModuleFragmentDeclaration(::std::boxed::Box<GlobalModuleFragmentDeclaration<'tree>>),
    ImportDeclaration(::std::boxed::Box<ImportDeclaration<'tree>>),
    LinkageSpecification(::std::boxed::Box<LinkageSpecification<'tree>>),
    ModuleDeclaration(::std::boxed::Box<ModuleDeclaration<'tree>>),
    NamespaceAliasDefinition(::std::boxed::Box<NamespaceAliasDefinition<'tree>>),
    NamespaceDefinition(::std::boxed::Box<NamespaceDefinition<'tree>>),
    PreprocCall(::std::boxed::Box<PreprocCall<'tree>>),
    PreprocDef(::std::boxed::Box<PreprocDef<'tree>>),
    PreprocFunctionDef(::std::boxed::Box<PreprocFunctionDef<'tree>>),
    PreprocIf(::std::boxed::Box<PreprocIf<'tree>>),
    PreprocIfdef(::std::boxed::Box<PreprocIfdef<'tree>>),
    PreprocInclude(::std::boxed::Box<PreprocInclude<'tree>>),
    PrivateModuleFragmentDeclaration(::std::boxed::Box<PrivateModuleFragmentDeclaration<'tree>>),
    Statement(::std::boxed::Box<Statement<'tree>>),
    StaticAssertDeclaration(::std::boxed::Box<StaticAssertDeclaration<'tree>>),
    TemplateDeclaration(::std::boxed::Box<TemplateDeclaration<'tree>>),
    TemplateInstantiation(::std::boxed::Box<TemplateInstantiation<'tree>>),
    TypeDefinition(::std::boxed::Box<TypeDefinition<'tree>>),
    TypeSpecifier(::std::boxed::Box<TypeSpecifier<'tree>>),
    UsingDeclaration(::std::boxed::Box<UsingDeclaration<'tree>>),
}
impl<'tree> ::treesitter_types::FromNode<'tree> for PreprocIfdefChildren<'tree> {
    #[allow(clippy::collapsible_else_if)]
    fn from_node(
        node: ::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        match node.kind() {
            "access_specifier" => Ok(Self::AccessSpecifier(::std::boxed::Box::new(
                <AccessSpecifier as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "alias_declaration" => Ok(Self::AliasDeclaration(::std::boxed::Box::new(
                <AliasDeclaration as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "concept_definition" => Ok(Self::ConceptDefinition(::std::boxed::Box::new(
                <ConceptDefinition as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "consteval_block_declaration" => {
                Ok(Self::ConstevalBlockDeclaration(::std::boxed::Box::new(
                    <ConstevalBlockDeclaration as ::treesitter_types::FromNode>::from_node(
                        node, src,
                    )?,
                )))
            }
            "declaration" => Ok(Self::Declaration(::std::boxed::Box::new(
                <Declaration as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "enumerator" => Ok(Self::Enumerator(::std::boxed::Box::new(
                <Enumerator as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "export_declaration" => Ok(Self::ExportDeclaration(::std::boxed::Box::new(
                <ExportDeclaration as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "field_declaration" => Ok(Self::FieldDeclaration(::std::boxed::Box::new(
                <FieldDeclaration as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "friend_declaration" => Ok(Self::FriendDeclaration(::std::boxed::Box::new(
                <FriendDeclaration as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "function_definition" => Ok(Self::FunctionDefinition(::std::boxed::Box::new(
                <FunctionDefinition as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "global_module_fragment_declaration" => Ok(Self::GlobalModuleFragmentDeclaration(
                ::std::boxed::Box::new(
                    <GlobalModuleFragmentDeclaration as ::treesitter_types::FromNode>::from_node(
                        node, src,
                    )?,
                ),
            )),
            "import_declaration" => Ok(Self::ImportDeclaration(::std::boxed::Box::new(
                <ImportDeclaration as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "linkage_specification" => Ok(Self::LinkageSpecification(::std::boxed::Box::new(
                <LinkageSpecification as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "module_declaration" => Ok(Self::ModuleDeclaration(::std::boxed::Box::new(
                <ModuleDeclaration as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "namespace_alias_definition" => {
                Ok(Self::NamespaceAliasDefinition(::std::boxed::Box::new(
                    <NamespaceAliasDefinition as ::treesitter_types::FromNode>::from_node(
                        node, src,
                    )?,
                )))
            }
            "namespace_definition" => Ok(Self::NamespaceDefinition(::std::boxed::Box::new(
                <NamespaceDefinition as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "preproc_call" => Ok(Self::PreprocCall(::std::boxed::Box::new(
                <PreprocCall as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "preproc_def" => Ok(Self::PreprocDef(::std::boxed::Box::new(
                <PreprocDef as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "preproc_function_def" => Ok(Self::PreprocFunctionDef(::std::boxed::Box::new(
                <PreprocFunctionDef as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "preproc_if" => Ok(Self::PreprocIf(::std::boxed::Box::new(
                <PreprocIf as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "preproc_ifdef" => Ok(Self::PreprocIfdef(::std::boxed::Box::new(
                <PreprocIfdef as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "preproc_include" => Ok(Self::PreprocInclude(::std::boxed::Box::new(
                <PreprocInclude as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "private_module_fragment_declaration" => Ok(Self::PrivateModuleFragmentDeclaration(
                ::std::boxed::Box::new(
                    <PrivateModuleFragmentDeclaration as ::treesitter_types::FromNode>::from_node(
                        node, src,
                    )?,
                ),
            )),
            "static_assert_declaration" => {
                Ok(Self::StaticAssertDeclaration(::std::boxed::Box::new(
                    <StaticAssertDeclaration as ::treesitter_types::FromNode>::from_node(
                        node, src,
                    )?,
                )))
            }
            "template_declaration" => Ok(Self::TemplateDeclaration(::std::boxed::Box::new(
                <TemplateDeclaration as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "template_instantiation" => Ok(Self::TemplateInstantiation(::std::boxed::Box::new(
                <TemplateInstantiation as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "type_definition" => Ok(Self::TypeDefinition(::std::boxed::Box::new(
                <TypeDefinition as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "using_declaration" => Ok(Self::UsingDeclaration(::std::boxed::Box::new(
                <UsingDeclaration as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            _other => {
                if let Ok(v) = <Statement as ::treesitter_types::FromNode>::from_node(node, src) {
                    Ok(Self::Statement(::std::boxed::Box::new(v)))
                } else {
                    if let Ok(v) =
                        <TypeSpecifier as ::treesitter_types::FromNode>::from_node(node, src)
                    {
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
            Self::AccessSpecifier(inner) => inner.span(),
            Self::AliasDeclaration(inner) => inner.span(),
            Self::ConceptDefinition(inner) => inner.span(),
            Self::ConstevalBlockDeclaration(inner) => inner.span(),
            Self::Declaration(inner) => inner.span(),
            Self::Enumerator(inner) => inner.span(),
            Self::ExportDeclaration(inner) => inner.span(),
            Self::FieldDeclaration(inner) => inner.span(),
            Self::FriendDeclaration(inner) => inner.span(),
            Self::FunctionDefinition(inner) => inner.span(),
            Self::GlobalModuleFragmentDeclaration(inner) => inner.span(),
            Self::ImportDeclaration(inner) => inner.span(),
            Self::LinkageSpecification(inner) => inner.span(),
            Self::ModuleDeclaration(inner) => inner.span(),
            Self::NamespaceAliasDefinition(inner) => inner.span(),
            Self::NamespaceDefinition(inner) => inner.span(),
            Self::PreprocCall(inner) => inner.span(),
            Self::PreprocDef(inner) => inner.span(),
            Self::PreprocFunctionDef(inner) => inner.span(),
            Self::PreprocIf(inner) => inner.span(),
            Self::PreprocIfdef(inner) => inner.span(),
            Self::PreprocInclude(inner) => inner.span(),
            Self::PrivateModuleFragmentDeclaration(inner) => inner.span(),
            Self::Statement(inner) => inner.span(),
            Self::StaticAssertDeclaration(inner) => inner.span(),
            Self::TemplateDeclaration(inner) => inner.span(),
            Self::TemplateInstantiation(inner) => inner.span(),
            Self::TypeDefinition(inner) => inner.span(),
            Self::TypeSpecifier(inner) => inner.span(),
            Self::UsingDeclaration(inner) => inner.span(),
        }
    }
}
#[derive(Debug, Clone)]
pub enum PreprocIncludePath<'tree> {
    CallExpression(::std::boxed::Box<CallExpression<'tree>>),
    Identifier(::std::boxed::Box<Identifier<'tree>>),
    StringLiteral(::std::boxed::Box<StringLiteral<'tree>>),
    SystemLibString(::std::boxed::Box<SystemLibString<'tree>>),
}
impl<'tree> ::treesitter_types::FromNode<'tree> for PreprocIncludePath<'tree> {
    #[allow(clippy::collapsible_else_if)]
    fn from_node(
        node: ::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        match node.kind() {
            "call_expression" => Ok(Self::CallExpression(::std::boxed::Box::new(
                <CallExpression as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "identifier" => Ok(Self::Identifier(::std::boxed::Box::new(
                <Identifier as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "string_literal" => Ok(Self::StringLiteral(::std::boxed::Box::new(
                <StringLiteral as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "system_lib_string" => Ok(Self::SystemLibString(::std::boxed::Box::new(
                <SystemLibString as ::treesitter_types::FromNode>::from_node(node, src)?,
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
#[derive(Debug, Clone)]
pub enum QualifiedIdentifierName<'tree> {
    DependentName(::std::boxed::Box<DependentName<'tree>>),
    DestructorName(::std::boxed::Box<DestructorName<'tree>>),
    FieldIdentifier(::std::boxed::Box<FieldIdentifier<'tree>>),
    Identifier(::std::boxed::Box<Identifier<'tree>>),
    OperatorCast(::std::boxed::Box<OperatorCast<'tree>>),
    OperatorName(::std::boxed::Box<OperatorName<'tree>>),
    PointerTypeDeclarator(::std::boxed::Box<PointerTypeDeclarator<'tree>>),
    QualifiedIdentifier(::std::boxed::Box<QualifiedIdentifier<'tree>>),
    Template(::treesitter_types::Span),
    TemplateFunction(::std::boxed::Box<TemplateFunction<'tree>>),
    TemplateMethod(::std::boxed::Box<TemplateMethod<'tree>>),
    TemplateType(::std::boxed::Box<TemplateType<'tree>>),
    TypeIdentifier(::std::boxed::Box<TypeIdentifier<'tree>>),
}
impl<'tree> ::treesitter_types::FromNode<'tree> for QualifiedIdentifierName<'tree> {
    #[allow(clippy::collapsible_else_if)]
    fn from_node(
        node: ::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        match node.kind() {
            "dependent_name" => Ok(Self::DependentName(::std::boxed::Box::new(
                <DependentName as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "destructor_name" => Ok(Self::DestructorName(::std::boxed::Box::new(
                <DestructorName as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "field_identifier" => Ok(Self::FieldIdentifier(::std::boxed::Box::new(
                <FieldIdentifier as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "identifier" => Ok(Self::Identifier(::std::boxed::Box::new(
                <Identifier as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "operator_cast" => Ok(Self::OperatorCast(::std::boxed::Box::new(
                <OperatorCast as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "operator_name" => Ok(Self::OperatorName(::std::boxed::Box::new(
                <OperatorName as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "pointer_type_declarator" => Ok(Self::PointerTypeDeclarator(::std::boxed::Box::new(
                <PointerTypeDeclarator as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "qualified_identifier" => Ok(Self::QualifiedIdentifier(::std::boxed::Box::new(
                <QualifiedIdentifier as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "template" => Ok(Self::Template(::treesitter_types::Span::from(node))),
            "template_function" => Ok(Self::TemplateFunction(::std::boxed::Box::new(
                <TemplateFunction as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "template_method" => Ok(Self::TemplateMethod(::std::boxed::Box::new(
                <TemplateMethod as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "template_type" => Ok(Self::TemplateType(::std::boxed::Box::new(
                <TemplateType as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "type_identifier" => Ok(Self::TypeIdentifier(::std::boxed::Box::new(
                <TypeIdentifier as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            other => Err(::treesitter_types::ParseError::unexpected_kind(other, node)),
        }
    }
}
impl ::treesitter_types::Spanned for QualifiedIdentifierName<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        match self {
            Self::DependentName(inner) => inner.span(),
            Self::DestructorName(inner) => inner.span(),
            Self::FieldIdentifier(inner) => inner.span(),
            Self::Identifier(inner) => inner.span(),
            Self::OperatorCast(inner) => inner.span(),
            Self::OperatorName(inner) => inner.span(),
            Self::PointerTypeDeclarator(inner) => inner.span(),
            Self::QualifiedIdentifier(inner) => inner.span(),
            Self::Template(span) => *span,
            Self::TemplateFunction(inner) => inner.span(),
            Self::TemplateMethod(inner) => inner.span(),
            Self::TemplateType(inner) => inner.span(),
            Self::TypeIdentifier(inner) => inner.span(),
        }
    }
}
#[derive(Debug, Clone)]
pub enum QualifiedIdentifierScope<'tree> {
    Decltype(::std::boxed::Box<Decltype<'tree>>),
    DependentName(::std::boxed::Box<DependentName<'tree>>),
    NamespaceIdentifier(::std::boxed::Box<NamespaceIdentifier<'tree>>),
    SpliceExpression(::std::boxed::Box<SpliceExpression<'tree>>),
    SpliceTypeSpecifier(::std::boxed::Box<SpliceTypeSpecifier<'tree>>),
    TemplateType(::std::boxed::Box<TemplateType<'tree>>),
}
impl<'tree> ::treesitter_types::FromNode<'tree> for QualifiedIdentifierScope<'tree> {
    #[allow(clippy::collapsible_else_if)]
    fn from_node(
        node: ::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        match node.kind() {
            "decltype" => Ok(Self::Decltype(::std::boxed::Box::new(
                <Decltype as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "dependent_name" => Ok(Self::DependentName(::std::boxed::Box::new(
                <DependentName as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "namespace_identifier" => Ok(Self::NamespaceIdentifier(::std::boxed::Box::new(
                <NamespaceIdentifier as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "splice_expression" => Ok(Self::SpliceExpression(::std::boxed::Box::new(
                <SpliceExpression as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "splice_type_specifier" => Ok(Self::SpliceTypeSpecifier(::std::boxed::Box::new(
                <SpliceTypeSpecifier as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "template_type" => Ok(Self::TemplateType(::std::boxed::Box::new(
                <TemplateType as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            other => Err(::treesitter_types::ParseError::unexpected_kind(other, node)),
        }
    }
}
impl ::treesitter_types::Spanned for QualifiedIdentifierScope<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        match self {
            Self::Decltype(inner) => inner.span(),
            Self::DependentName(inner) => inner.span(),
            Self::NamespaceIdentifier(inner) => inner.span(),
            Self::SpliceExpression(inner) => inner.span(),
            Self::SpliceTypeSpecifier(inner) => inner.span(),
            Self::TemplateType(inner) => inner.span(),
        }
    }
}
#[derive(Debug, Clone)]
pub enum RawStringLiteralChildren<'tree> {
    RawStringContent(::std::boxed::Box<RawStringContent<'tree>>),
    RawStringDelimiter(::std::boxed::Box<RawStringDelimiter<'tree>>),
}
impl<'tree> ::treesitter_types::FromNode<'tree> for RawStringLiteralChildren<'tree> {
    #[allow(clippy::collapsible_else_if)]
    fn from_node(
        node: ::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        match node.kind() {
            "raw_string_content" => Ok(Self::RawStringContent(::std::boxed::Box::new(
                <RawStringContent as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "raw_string_delimiter" => Ok(Self::RawStringDelimiter(::std::boxed::Box::new(
                <RawStringDelimiter as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            other => Err(::treesitter_types::ParseError::unexpected_kind(other, node)),
        }
    }
}
impl ::treesitter_types::Spanned for RawStringLiteralChildren<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        match self {
            Self::RawStringContent(inner) => inner.span(),
            Self::RawStringDelimiter(inner) => inner.span(),
        }
    }
}
#[derive(Debug, Clone)]
pub enum ReferenceDeclaratorChildren<'tree> {
    Declarator(::std::boxed::Box<Declarator<'tree>>),
    FieldDeclarator(::std::boxed::Box<FieldDeclarator<'tree>>),
    TypeDeclarator(::std::boxed::Box<TypeDeclarator<'tree>>),
    VariadicDeclarator(::std::boxed::Box<VariadicDeclarator<'tree>>),
}
impl<'tree> ::treesitter_types::FromNode<'tree> for ReferenceDeclaratorChildren<'tree> {
    #[allow(clippy::collapsible_else_if)]
    fn from_node(
        node: ::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        match node.kind() {
            "variadic_declarator" => Ok(Self::VariadicDeclarator(::std::boxed::Box::new(
                <VariadicDeclarator as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            _other => {
                if let Ok(v) = <Declarator as ::treesitter_types::FromNode>::from_node(node, src) {
                    Ok(Self::Declarator(::std::boxed::Box::new(v)))
                } else {
                    if let Ok(v) =
                        <FieldDeclarator as ::treesitter_types::FromNode>::from_node(node, src)
                    {
                        Ok(Self::FieldDeclarator(::std::boxed::Box::new(v)))
                    } else {
                        if let Ok(v) =
                            <TypeDeclarator as ::treesitter_types::FromNode>::from_node(node, src)
                        {
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
impl ::treesitter_types::Spanned for ReferenceDeclaratorChildren<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        match self {
            Self::Declarator(inner) => inner.span(),
            Self::FieldDeclarator(inner) => inner.span(),
            Self::TypeDeclarator(inner) => inner.span(),
            Self::VariadicDeclarator(inner) => inner.span(),
        }
    }
}
#[derive(Debug, Clone)]
pub enum ReflectExpressionChildren<'tree> {
    Expression(::std::boxed::Box<Expression<'tree>>),
    TypeDescriptor(::std::boxed::Box<TypeDescriptor<'tree>>),
}
impl<'tree> ::treesitter_types::FromNode<'tree> for ReflectExpressionChildren<'tree> {
    #[allow(clippy::collapsible_else_if)]
    fn from_node(
        node: ::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        match node.kind() {
            "type_descriptor" => Ok(Self::TypeDescriptor(::std::boxed::Box::new(
                <TypeDescriptor as ::treesitter_types::FromNode>::from_node(node, src)?,
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
impl ::treesitter_types::Spanned for ReflectExpressionChildren<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        match self {
            Self::Expression(inner) => inner.span(),
            Self::TypeDescriptor(inner) => inner.span(),
        }
    }
}
#[derive(Debug, Clone)]
pub enum RequirementSeqChildren<'tree> {
    CompoundRequirement(::std::boxed::Box<CompoundRequirement<'tree>>),
    SimpleRequirement(::std::boxed::Box<SimpleRequirement<'tree>>),
    TypeRequirement(::std::boxed::Box<TypeRequirement<'tree>>),
}
impl<'tree> ::treesitter_types::FromNode<'tree> for RequirementSeqChildren<'tree> {
    #[allow(clippy::collapsible_else_if)]
    fn from_node(
        node: ::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        match node.kind() {
            "compound_requirement" => Ok(Self::CompoundRequirement(::std::boxed::Box::new(
                <CompoundRequirement as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "simple_requirement" => Ok(Self::SimpleRequirement(::std::boxed::Box::new(
                <SimpleRequirement as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "type_requirement" => Ok(Self::TypeRequirement(::std::boxed::Box::new(
                <TypeRequirement as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            other => Err(::treesitter_types::ParseError::unexpected_kind(other, node)),
        }
    }
}
impl ::treesitter_types::Spanned for RequirementSeqChildren<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        match self {
            Self::CompoundRequirement(inner) => inner.span(),
            Self::SimpleRequirement(inner) => inner.span(),
            Self::TypeRequirement(inner) => inner.span(),
        }
    }
}
#[derive(Debug, Clone)]
pub enum RequiresClauseConstraint<'tree> {
    LParen(::treesitter_types::Span),
    RParen(::treesitter_types::Span),
    ConstraintConjunction(::std::boxed::Box<ConstraintConjunction<'tree>>),
    ConstraintDisjunction(::std::boxed::Box<ConstraintDisjunction<'tree>>),
    Expression(::std::boxed::Box<Expression<'tree>>),
    SpliceTypeSpecifier(::std::boxed::Box<SpliceTypeSpecifier<'tree>>),
    TemplateType(::std::boxed::Box<TemplateType<'tree>>),
    TypeIdentifier(::std::boxed::Box<TypeIdentifier<'tree>>),
}
impl<'tree> ::treesitter_types::FromNode<'tree> for RequiresClauseConstraint<'tree> {
    #[allow(clippy::collapsible_else_if)]
    fn from_node(
        node: ::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        match node.kind() {
            "(" => Ok(Self::LParen(::treesitter_types::Span::from(node))),
            ")" => Ok(Self::RParen(::treesitter_types::Span::from(node))),
            "constraint_conjunction" => Ok(Self::ConstraintConjunction(::std::boxed::Box::new(
                <ConstraintConjunction as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "constraint_disjunction" => Ok(Self::ConstraintDisjunction(::std::boxed::Box::new(
                <ConstraintDisjunction as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "splice_type_specifier" => Ok(Self::SpliceTypeSpecifier(::std::boxed::Box::new(
                <SpliceTypeSpecifier as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "template_type" => Ok(Self::TemplateType(::std::boxed::Box::new(
                <TemplateType as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "type_identifier" => Ok(Self::TypeIdentifier(::std::boxed::Box::new(
                <TypeIdentifier as ::treesitter_types::FromNode>::from_node(node, src)?,
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
impl ::treesitter_types::Spanned for RequiresClauseConstraint<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        match self {
            Self::LParen(span) => *span,
            Self::RParen(span) => *span,
            Self::ConstraintConjunction(inner) => inner.span(),
            Self::ConstraintDisjunction(inner) => inner.span(),
            Self::Expression(inner) => inner.span(),
            Self::SpliceTypeSpecifier(inner) => inner.span(),
            Self::TemplateType(inner) => inner.span(),
            Self::TypeIdentifier(inner) => inner.span(),
        }
    }
}
#[derive(Debug, Clone)]
pub enum ReturnStatementChildren<'tree> {
    CommaExpression(::std::boxed::Box<CommaExpression<'tree>>),
    Expression(::std::boxed::Box<Expression<'tree>>),
    InitializerList(::std::boxed::Box<InitializerList<'tree>>),
}
impl<'tree> ::treesitter_types::FromNode<'tree> for ReturnStatementChildren<'tree> {
    #[allow(clippy::collapsible_else_if)]
    fn from_node(
        node: ::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        match node.kind() {
            "comma_expression" => Ok(Self::CommaExpression(::std::boxed::Box::new(
                <CommaExpression as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "initializer_list" => Ok(Self::InitializerList(::std::boxed::Box::new(
                <InitializerList as ::treesitter_types::FromNode>::from_node(node, src)?,
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
impl ::treesitter_types::Spanned for ReturnStatementChildren<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        match self {
            Self::CommaExpression(inner) => inner.span(),
            Self::Expression(inner) => inner.span(),
            Self::InitializerList(inner) => inner.span(),
        }
    }
}
#[derive(Debug, Clone)]
pub enum SehTryStatementChildren<'tree> {
    SehExceptClause(::std::boxed::Box<SehExceptClause<'tree>>),
    SehFinallyClause(::std::boxed::Box<SehFinallyClause<'tree>>),
}
impl<'tree> ::treesitter_types::FromNode<'tree> for SehTryStatementChildren<'tree> {
    #[allow(clippy::collapsible_else_if)]
    fn from_node(
        node: ::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        match node.kind() {
            "seh_except_clause" => Ok(Self::SehExceptClause(::std::boxed::Box::new(
                <SehExceptClause as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "seh_finally_clause" => Ok(Self::SehFinallyClause(::std::boxed::Box::new(
                <SehFinallyClause as ::treesitter_types::FromNode>::from_node(node, src)?,
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
#[derive(Debug, Clone)]
pub enum SimpleRequirementChildren<'tree> {
    CommaExpression(::std::boxed::Box<CommaExpression<'tree>>),
    Expression(::std::boxed::Box<Expression<'tree>>),
}
impl<'tree> ::treesitter_types::FromNode<'tree> for SimpleRequirementChildren<'tree> {
    #[allow(clippy::collapsible_else_if)]
    fn from_node(
        node: ::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        match node.kind() {
            "comma_expression" => Ok(Self::CommaExpression(::std::boxed::Box::new(
                <CommaExpression as ::treesitter_types::FromNode>::from_node(node, src)?,
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
impl ::treesitter_types::Spanned for SimpleRequirementChildren<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        match self {
            Self::CommaExpression(inner) => inner.span(),
            Self::Expression(inner) => inner.span(),
        }
    }
}
#[derive(Debug, Clone)]
pub enum SizedTypeSpecifierType<'tree> {
    PrimitiveType(::std::boxed::Box<PrimitiveType<'tree>>),
    TypeIdentifier(::std::boxed::Box<TypeIdentifier<'tree>>),
}
impl<'tree> ::treesitter_types::FromNode<'tree> for SizedTypeSpecifierType<'tree> {
    #[allow(clippy::collapsible_else_if)]
    fn from_node(
        node: ::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        match node.kind() {
            "primitive_type" => Ok(Self::PrimitiveType(::std::boxed::Box::new(
                <PrimitiveType as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "type_identifier" => Ok(Self::TypeIdentifier(::std::boxed::Box::new(
                <TypeIdentifier as ::treesitter_types::FromNode>::from_node(node, src)?,
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
#[derive(Debug, Clone)]
pub enum SpliceExpressionChildren<'tree> {
    SpliceSpecifier(::std::boxed::Box<SpliceSpecifier<'tree>>),
    TemplateArgumentList(::std::boxed::Box<TemplateArgumentList<'tree>>),
}
impl<'tree> ::treesitter_types::FromNode<'tree> for SpliceExpressionChildren<'tree> {
    #[allow(clippy::collapsible_else_if)]
    fn from_node(
        node: ::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        match node.kind() {
            "splice_specifier" => Ok(Self::SpliceSpecifier(::std::boxed::Box::new(
                <SpliceSpecifier as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "template_argument_list" => Ok(Self::TemplateArgumentList(::std::boxed::Box::new(
                <TemplateArgumentList as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            other => Err(::treesitter_types::ParseError::unexpected_kind(other, node)),
        }
    }
}
impl ::treesitter_types::Spanned for SpliceExpressionChildren<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        match self {
            Self::SpliceSpecifier(inner) => inner.span(),
            Self::TemplateArgumentList(inner) => inner.span(),
        }
    }
}
#[derive(Debug, Clone)]
pub enum SpliceTypeSpecifierChildren<'tree> {
    SpliceSpecifier(::std::boxed::Box<SpliceSpecifier<'tree>>),
    TemplateArgumentList(::std::boxed::Box<TemplateArgumentList<'tree>>),
}
impl<'tree> ::treesitter_types::FromNode<'tree> for SpliceTypeSpecifierChildren<'tree> {
    #[allow(clippy::collapsible_else_if)]
    fn from_node(
        node: ::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        match node.kind() {
            "splice_specifier" => Ok(Self::SpliceSpecifier(::std::boxed::Box::new(
                <SpliceSpecifier as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "template_argument_list" => Ok(Self::TemplateArgumentList(::std::boxed::Box::new(
                <TemplateArgumentList as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            other => Err(::treesitter_types::ParseError::unexpected_kind(other, node)),
        }
    }
}
impl ::treesitter_types::Spanned for SpliceTypeSpecifierChildren<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        match self {
            Self::SpliceSpecifier(inner) => inner.span(),
            Self::TemplateArgumentList(inner) => inner.span(),
        }
    }
}
#[derive(Debug, Clone)]
pub enum StaticAssertDeclarationMessage<'tree> {
    ConcatenatedString(::std::boxed::Box<ConcatenatedString<'tree>>),
    RawStringLiteral(::std::boxed::Box<RawStringLiteral<'tree>>),
    StringLiteral(::std::boxed::Box<StringLiteral<'tree>>),
}
impl<'tree> ::treesitter_types::FromNode<'tree> for StaticAssertDeclarationMessage<'tree> {
    #[allow(clippy::collapsible_else_if)]
    fn from_node(
        node: ::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        match node.kind() {
            "concatenated_string" => Ok(Self::ConcatenatedString(::std::boxed::Box::new(
                <ConcatenatedString as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "raw_string_literal" => Ok(Self::RawStringLiteral(::std::boxed::Box::new(
                <RawStringLiteral as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "string_literal" => Ok(Self::StringLiteral(::std::boxed::Box::new(
                <StringLiteral as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            other => Err(::treesitter_types::ParseError::unexpected_kind(other, node)),
        }
    }
}
impl ::treesitter_types::Spanned for StaticAssertDeclarationMessage<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        match self {
            Self::ConcatenatedString(inner) => inner.span(),
            Self::RawStringLiteral(inner) => inner.span(),
            Self::StringLiteral(inner) => inner.span(),
        }
    }
}
#[derive(Debug, Clone)]
pub enum StringLiteralChildren<'tree> {
    EscapeSequence(::std::boxed::Box<EscapeSequence<'tree>>),
    StringContent(::std::boxed::Box<StringContent<'tree>>),
}
impl<'tree> ::treesitter_types::FromNode<'tree> for StringLiteralChildren<'tree> {
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
impl ::treesitter_types::Spanned for StringLiteralChildren<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        match self {
            Self::EscapeSequence(inner) => inner.span(),
            Self::StringContent(inner) => inner.span(),
        }
    }
}
#[derive(Debug, Clone)]
pub enum StructSpecifierName<'tree> {
    QualifiedIdentifier(::std::boxed::Box<QualifiedIdentifier<'tree>>),
    SpliceTypeSpecifier(::std::boxed::Box<SpliceTypeSpecifier<'tree>>),
    TemplateType(::std::boxed::Box<TemplateType<'tree>>),
    TypeIdentifier(::std::boxed::Box<TypeIdentifier<'tree>>),
}
impl<'tree> ::treesitter_types::FromNode<'tree> for StructSpecifierName<'tree> {
    #[allow(clippy::collapsible_else_if)]
    fn from_node(
        node: ::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        match node.kind() {
            "qualified_identifier" => Ok(Self::QualifiedIdentifier(::std::boxed::Box::new(
                <QualifiedIdentifier as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "splice_type_specifier" => Ok(Self::SpliceTypeSpecifier(::std::boxed::Box::new(
                <SpliceTypeSpecifier as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "template_type" => Ok(Self::TemplateType(::std::boxed::Box::new(
                <TemplateType as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "type_identifier" => Ok(Self::TypeIdentifier(::std::boxed::Box::new(
                <TypeIdentifier as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            other => Err(::treesitter_types::ParseError::unexpected_kind(other, node)),
        }
    }
}
impl ::treesitter_types::Spanned for StructSpecifierName<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        match self {
            Self::QualifiedIdentifier(inner) => inner.span(),
            Self::SpliceTypeSpecifier(inner) => inner.span(),
            Self::TemplateType(inner) => inner.span(),
            Self::TypeIdentifier(inner) => inner.span(),
        }
    }
}
#[derive(Debug, Clone)]
pub enum StructSpecifierChildren<'tree> {
    AlignasQualifier(::std::boxed::Box<AlignasQualifier<'tree>>),
    AttributeDeclaration(::std::boxed::Box<AttributeDeclaration<'tree>>),
    AttributeSpecifier(::std::boxed::Box<AttributeSpecifier<'tree>>),
    BaseClassClause(::std::boxed::Box<BaseClassClause<'tree>>),
    MsDeclspecModifier(::std::boxed::Box<MsDeclspecModifier<'tree>>),
    VirtualSpecifier(::std::boxed::Box<VirtualSpecifier<'tree>>),
}
impl<'tree> ::treesitter_types::FromNode<'tree> for StructSpecifierChildren<'tree> {
    #[allow(clippy::collapsible_else_if)]
    fn from_node(
        node: ::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        match node.kind() {
            "alignas_qualifier" => Ok(Self::AlignasQualifier(::std::boxed::Box::new(
                <AlignasQualifier as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "attribute_declaration" => Ok(Self::AttributeDeclaration(::std::boxed::Box::new(
                <AttributeDeclaration as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "attribute_specifier" => Ok(Self::AttributeSpecifier(::std::boxed::Box::new(
                <AttributeSpecifier as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "base_class_clause" => Ok(Self::BaseClassClause(::std::boxed::Box::new(
                <BaseClassClause as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "ms_declspec_modifier" => Ok(Self::MsDeclspecModifier(::std::boxed::Box::new(
                <MsDeclspecModifier as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "virtual_specifier" => Ok(Self::VirtualSpecifier(::std::boxed::Box::new(
                <VirtualSpecifier as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            other => Err(::treesitter_types::ParseError::unexpected_kind(other, node)),
        }
    }
}
impl ::treesitter_types::Spanned for StructSpecifierChildren<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        match self {
            Self::AlignasQualifier(inner) => inner.span(),
            Self::AttributeDeclaration(inner) => inner.span(),
            Self::AttributeSpecifier(inner) => inner.span(),
            Self::BaseClassClause(inner) => inner.span(),
            Self::MsDeclspecModifier(inner) => inner.span(),
            Self::VirtualSpecifier(inner) => inner.span(),
        }
    }
}
#[derive(Debug, Clone)]
pub enum SubscriptArgumentListChildren<'tree> {
    Expression(::std::boxed::Box<Expression<'tree>>),
    InitializerList(::std::boxed::Box<InitializerList<'tree>>),
}
impl<'tree> ::treesitter_types::FromNode<'tree> for SubscriptArgumentListChildren<'tree> {
    #[allow(clippy::collapsible_else_if)]
    fn from_node(
        node: ::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        match node.kind() {
            "initializer_list" => Ok(Self::InitializerList(::std::boxed::Box::new(
                <InitializerList as ::treesitter_types::FromNode>::from_node(node, src)?,
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
impl ::treesitter_types::Spanned for SubscriptArgumentListChildren<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        match self {
            Self::Expression(inner) => inner.span(),
            Self::InitializerList(inner) => inner.span(),
        }
    }
}
#[derive(Debug, Clone)]
pub enum TemplateArgumentListChildren<'tree> {
    Expression(::std::boxed::Box<Expression<'tree>>),
    TypeDescriptor(::std::boxed::Box<TypeDescriptor<'tree>>),
}
impl<'tree> ::treesitter_types::FromNode<'tree> for TemplateArgumentListChildren<'tree> {
    #[allow(clippy::collapsible_else_if)]
    fn from_node(
        node: ::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        match node.kind() {
            "type_descriptor" => Ok(Self::TypeDescriptor(::std::boxed::Box::new(
                <TypeDescriptor as ::treesitter_types::FromNode>::from_node(node, src)?,
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
impl ::treesitter_types::Spanned for TemplateArgumentListChildren<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        match self {
            Self::Expression(inner) => inner.span(),
            Self::TypeDescriptor(inner) => inner.span(),
        }
    }
}
#[derive(Debug, Clone)]
pub enum TemplateDeclarationChildren<'tree> {
    AliasDeclaration(::std::boxed::Box<AliasDeclaration<'tree>>),
    ConceptDefinition(::std::boxed::Box<ConceptDefinition<'tree>>),
    Declaration(::std::boxed::Box<Declaration<'tree>>),
    FriendDeclaration(::std::boxed::Box<FriendDeclaration<'tree>>),
    FunctionDefinition(::std::boxed::Box<FunctionDefinition<'tree>>),
    RequiresClause(::std::boxed::Box<RequiresClause<'tree>>),
    TemplateDeclaration(::std::boxed::Box<TemplateDeclaration<'tree>>),
    TypeSpecifier(::std::boxed::Box<TypeSpecifier<'tree>>),
}
impl<'tree> ::treesitter_types::FromNode<'tree> for TemplateDeclarationChildren<'tree> {
    #[allow(clippy::collapsible_else_if)]
    fn from_node(
        node: ::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        match node.kind() {
            "alias_declaration" => Ok(Self::AliasDeclaration(::std::boxed::Box::new(
                <AliasDeclaration as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "concept_definition" => Ok(Self::ConceptDefinition(::std::boxed::Box::new(
                <ConceptDefinition as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "declaration" => Ok(Self::Declaration(::std::boxed::Box::new(
                <Declaration as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "friend_declaration" => Ok(Self::FriendDeclaration(::std::boxed::Box::new(
                <FriendDeclaration as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "function_definition" => Ok(Self::FunctionDefinition(::std::boxed::Box::new(
                <FunctionDefinition as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "requires_clause" => Ok(Self::RequiresClause(::std::boxed::Box::new(
                <RequiresClause as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "template_declaration" => Ok(Self::TemplateDeclaration(::std::boxed::Box::new(
                <TemplateDeclaration as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            _other => {
                if let Ok(v) = <TypeSpecifier as ::treesitter_types::FromNode>::from_node(node, src)
                {
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
impl ::treesitter_types::Spanned for TemplateDeclarationChildren<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        match self {
            Self::AliasDeclaration(inner) => inner.span(),
            Self::ConceptDefinition(inner) => inner.span(),
            Self::Declaration(inner) => inner.span(),
            Self::FriendDeclaration(inner) => inner.span(),
            Self::FunctionDefinition(inner) => inner.span(),
            Self::RequiresClause(inner) => inner.span(),
            Self::TemplateDeclaration(inner) => inner.span(),
            Self::TypeSpecifier(inner) => inner.span(),
        }
    }
}
#[derive(Debug, Clone)]
pub enum TemplateInstantiationChildren<'tree> {
    AttributeDeclaration(::std::boxed::Box<AttributeDeclaration<'tree>>),
    AttributeSpecifier(::std::boxed::Box<AttributeSpecifier<'tree>>),
    MsDeclspecModifier(::std::boxed::Box<MsDeclspecModifier<'tree>>),
    StorageClassSpecifier(::std::boxed::Box<StorageClassSpecifier<'tree>>),
    TypeQualifier(::std::boxed::Box<TypeQualifier<'tree>>),
}
impl<'tree> ::treesitter_types::FromNode<'tree> for TemplateInstantiationChildren<'tree> {
    #[allow(clippy::collapsible_else_if)]
    fn from_node(
        node: ::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        match node.kind() {
            "attribute_declaration" => Ok(Self::AttributeDeclaration(::std::boxed::Box::new(
                <AttributeDeclaration as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "attribute_specifier" => Ok(Self::AttributeSpecifier(::std::boxed::Box::new(
                <AttributeSpecifier as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "ms_declspec_modifier" => Ok(Self::MsDeclspecModifier(::std::boxed::Box::new(
                <MsDeclspecModifier as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "storage_class_specifier" => Ok(Self::StorageClassSpecifier(::std::boxed::Box::new(
                <StorageClassSpecifier as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "type_qualifier" => Ok(Self::TypeQualifier(::std::boxed::Box::new(
                <TypeQualifier as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            other => Err(::treesitter_types::ParseError::unexpected_kind(other, node)),
        }
    }
}
impl ::treesitter_types::Spanned for TemplateInstantiationChildren<'_> {
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
#[derive(Debug, Clone)]
pub enum TemplateMethodName<'tree> {
    FieldIdentifier(::std::boxed::Box<FieldIdentifier<'tree>>),
    OperatorName(::std::boxed::Box<OperatorName<'tree>>),
}
impl<'tree> ::treesitter_types::FromNode<'tree> for TemplateMethodName<'tree> {
    #[allow(clippy::collapsible_else_if)]
    fn from_node(
        node: ::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        match node.kind() {
            "field_identifier" => Ok(Self::FieldIdentifier(::std::boxed::Box::new(
                <FieldIdentifier as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "operator_name" => Ok(Self::OperatorName(::std::boxed::Box::new(
                <OperatorName as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            other => Err(::treesitter_types::ParseError::unexpected_kind(other, node)),
        }
    }
}
impl ::treesitter_types::Spanned for TemplateMethodName<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        match self {
            Self::FieldIdentifier(inner) => inner.span(),
            Self::OperatorName(inner) => inner.span(),
        }
    }
}
#[derive(Debug, Clone)]
pub enum TemplateParameterListChildren<'tree> {
    OptionalParameterDeclaration(::std::boxed::Box<OptionalParameterDeclaration<'tree>>),
    OptionalTypeParameterDeclaration(::std::boxed::Box<OptionalTypeParameterDeclaration<'tree>>),
    ParameterDeclaration(::std::boxed::Box<ParameterDeclaration<'tree>>),
    TemplateTemplateParameterDeclaration(
        ::std::boxed::Box<TemplateTemplateParameterDeclaration<'tree>>,
    ),
    TypeParameterDeclaration(::std::boxed::Box<TypeParameterDeclaration<'tree>>),
    VariadicParameterDeclaration(::std::boxed::Box<VariadicParameterDeclaration<'tree>>),
    VariadicTypeParameterDeclaration(::std::boxed::Box<VariadicTypeParameterDeclaration<'tree>>),
}
impl<'tree> ::treesitter_types::FromNode<'tree> for TemplateParameterListChildren<'tree> {
    #[allow(clippy::collapsible_else_if)]
    fn from_node(
        node: ::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        match node.kind() {
            "optional_parameter_declaration" => {
                Ok(
                    Self::OptionalParameterDeclaration(
                        ::std::boxed::Box::new(
                            <OptionalParameterDeclaration as ::treesitter_types::FromNode>::from_node(
                                node,
                                src,
                            )?,
                        ),
                    ),
                )
            }
            "optional_type_parameter_declaration" => {
                Ok(
                    Self::OptionalTypeParameterDeclaration(
                        ::std::boxed::Box::new(
                            <OptionalTypeParameterDeclaration as ::treesitter_types::FromNode>::from_node(
                                node,
                                src,
                            )?,
                        ),
                    ),
                )
            }
            "parameter_declaration" => {
                Ok(
                    Self::ParameterDeclaration(
                        ::std::boxed::Box::new(
                            <ParameterDeclaration as ::treesitter_types::FromNode>::from_node(
                                node,
                                src,
                            )?,
                        ),
                    ),
                )
            }
            "template_template_parameter_declaration" => {
                Ok(
                    Self::TemplateTemplateParameterDeclaration(
                        ::std::boxed::Box::new(
                            <TemplateTemplateParameterDeclaration as ::treesitter_types::FromNode>::from_node(
                                node,
                                src,
                            )?,
                        ),
                    ),
                )
            }
            "type_parameter_declaration" => {
                Ok(
                    Self::TypeParameterDeclaration(
                        ::std::boxed::Box::new(
                            <TypeParameterDeclaration as ::treesitter_types::FromNode>::from_node(
                                node,
                                src,
                            )?,
                        ),
                    ),
                )
            }
            "variadic_parameter_declaration" => {
                Ok(
                    Self::VariadicParameterDeclaration(
                        ::std::boxed::Box::new(
                            <VariadicParameterDeclaration as ::treesitter_types::FromNode>::from_node(
                                node,
                                src,
                            )?,
                        ),
                    ),
                )
            }
            "variadic_type_parameter_declaration" => {
                Ok(
                    Self::VariadicTypeParameterDeclaration(
                        ::std::boxed::Box::new(
                            <VariadicTypeParameterDeclaration as ::treesitter_types::FromNode>::from_node(
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
impl ::treesitter_types::Spanned for TemplateParameterListChildren<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        match self {
            Self::OptionalParameterDeclaration(inner) => inner.span(),
            Self::OptionalTypeParameterDeclaration(inner) => inner.span(),
            Self::ParameterDeclaration(inner) => inner.span(),
            Self::TemplateTemplateParameterDeclaration(inner) => inner.span(),
            Self::TypeParameterDeclaration(inner) => inner.span(),
            Self::VariadicParameterDeclaration(inner) => inner.span(),
            Self::VariadicTypeParameterDeclaration(inner) => inner.span(),
        }
    }
}
#[derive(Debug, Clone)]
pub enum TemplateTemplateParameterDeclarationChildren<'tree> {
    OptionalTypeParameterDeclaration(::std::boxed::Box<OptionalTypeParameterDeclaration<'tree>>),
    TypeParameterDeclaration(::std::boxed::Box<TypeParameterDeclaration<'tree>>),
    VariadicTypeParameterDeclaration(::std::boxed::Box<VariadicTypeParameterDeclaration<'tree>>),
}
impl<'tree> ::treesitter_types::FromNode<'tree>
    for TemplateTemplateParameterDeclarationChildren<'tree>
{
    #[allow(clippy::collapsible_else_if)]
    fn from_node(
        node: ::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        match node.kind() {
            "optional_type_parameter_declaration" => Ok(Self::OptionalTypeParameterDeclaration(
                ::std::boxed::Box::new(
                    <OptionalTypeParameterDeclaration as ::treesitter_types::FromNode>::from_node(
                        node, src,
                    )?,
                ),
            )),
            "type_parameter_declaration" => {
                Ok(Self::TypeParameterDeclaration(::std::boxed::Box::new(
                    <TypeParameterDeclaration as ::treesitter_types::FromNode>::from_node(
                        node, src,
                    )?,
                )))
            }
            "variadic_type_parameter_declaration" => Ok(Self::VariadicTypeParameterDeclaration(
                ::std::boxed::Box::new(
                    <VariadicTypeParameterDeclaration as ::treesitter_types::FromNode>::from_node(
                        node, src,
                    )?,
                ),
            )),
            other => Err(::treesitter_types::ParseError::unexpected_kind(other, node)),
        }
    }
}
impl ::treesitter_types::Spanned for TemplateTemplateParameterDeclarationChildren<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        match self {
            Self::OptionalTypeParameterDeclaration(inner) => inner.span(),
            Self::TypeParameterDeclaration(inner) => inner.span(),
            Self::VariadicTypeParameterDeclaration(inner) => inner.span(),
        }
    }
}
#[derive(Debug, Clone)]
pub enum TranslationUnitChildren<'tree> {
    AliasDeclaration(::std::boxed::Box<AliasDeclaration<'tree>>),
    AttributedStatement(::std::boxed::Box<AttributedStatement<'tree>>),
    BreakStatement(::std::boxed::Box<BreakStatement<'tree>>),
    CaseStatement(::std::boxed::Box<CaseStatement<'tree>>),
    CoReturnStatement(::std::boxed::Box<CoReturnStatement<'tree>>),
    CoYieldStatement(::std::boxed::Box<CoYieldStatement<'tree>>),
    CompoundStatement(::std::boxed::Box<CompoundStatement<'tree>>),
    ConceptDefinition(::std::boxed::Box<ConceptDefinition<'tree>>),
    ConstevalBlockDeclaration(::std::boxed::Box<ConstevalBlockDeclaration<'tree>>),
    ContinueStatement(::std::boxed::Box<ContinueStatement<'tree>>),
    Declaration(::std::boxed::Box<Declaration<'tree>>),
    DoStatement(::std::boxed::Box<DoStatement<'tree>>),
    ExpansionStatement(::std::boxed::Box<ExpansionStatement<'tree>>),
    ExportDeclaration(::std::boxed::Box<ExportDeclaration<'tree>>),
    ExpressionStatement(::std::boxed::Box<ExpressionStatement<'tree>>),
    ForRangeLoop(::std::boxed::Box<ForRangeLoop<'tree>>),
    ForStatement(::std::boxed::Box<ForStatement<'tree>>),
    FunctionDefinition(::std::boxed::Box<FunctionDefinition<'tree>>),
    GlobalModuleFragmentDeclaration(::std::boxed::Box<GlobalModuleFragmentDeclaration<'tree>>),
    GotoStatement(::std::boxed::Box<GotoStatement<'tree>>),
    IfStatement(::std::boxed::Box<IfStatement<'tree>>),
    ImportDeclaration(::std::boxed::Box<ImportDeclaration<'tree>>),
    LabeledStatement(::std::boxed::Box<LabeledStatement<'tree>>),
    LinkageSpecification(::std::boxed::Box<LinkageSpecification<'tree>>),
    ModuleDeclaration(::std::boxed::Box<ModuleDeclaration<'tree>>),
    NamespaceAliasDefinition(::std::boxed::Box<NamespaceAliasDefinition<'tree>>),
    NamespaceDefinition(::std::boxed::Box<NamespaceDefinition<'tree>>),
    PreprocCall(::std::boxed::Box<PreprocCall<'tree>>),
    PreprocDef(::std::boxed::Box<PreprocDef<'tree>>),
    PreprocFunctionDef(::std::boxed::Box<PreprocFunctionDef<'tree>>),
    PreprocIf(::std::boxed::Box<PreprocIf<'tree>>),
    PreprocIfdef(::std::boxed::Box<PreprocIfdef<'tree>>),
    PreprocInclude(::std::boxed::Box<PreprocInclude<'tree>>),
    PrivateModuleFragmentDeclaration(::std::boxed::Box<PrivateModuleFragmentDeclaration<'tree>>),
    ReturnStatement(::std::boxed::Box<ReturnStatement<'tree>>),
    StaticAssertDeclaration(::std::boxed::Box<StaticAssertDeclaration<'tree>>),
    SwitchStatement(::std::boxed::Box<SwitchStatement<'tree>>),
    TemplateDeclaration(::std::boxed::Box<TemplateDeclaration<'tree>>),
    TemplateInstantiation(::std::boxed::Box<TemplateInstantiation<'tree>>),
    ThrowStatement(::std::boxed::Box<ThrowStatement<'tree>>),
    TryStatement(::std::boxed::Box<TryStatement<'tree>>),
    TypeDefinition(::std::boxed::Box<TypeDefinition<'tree>>),
    TypeSpecifier(::std::boxed::Box<TypeSpecifier<'tree>>),
    UsingDeclaration(::std::boxed::Box<UsingDeclaration<'tree>>),
    WhileStatement(::std::boxed::Box<WhileStatement<'tree>>),
}
impl<'tree> ::treesitter_types::FromNode<'tree> for TranslationUnitChildren<'tree> {
    #[allow(clippy::collapsible_else_if)]
    fn from_node(
        node: ::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        match node.kind() {
            "alias_declaration" => Ok(Self::AliasDeclaration(::std::boxed::Box::new(
                <AliasDeclaration as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "attributed_statement" => Ok(Self::AttributedStatement(::std::boxed::Box::new(
                <AttributedStatement as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "break_statement" => Ok(Self::BreakStatement(::std::boxed::Box::new(
                <BreakStatement as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "case_statement" => Ok(Self::CaseStatement(::std::boxed::Box::new(
                <CaseStatement as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "co_return_statement" => Ok(Self::CoReturnStatement(::std::boxed::Box::new(
                <CoReturnStatement as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "co_yield_statement" => Ok(Self::CoYieldStatement(::std::boxed::Box::new(
                <CoYieldStatement as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "compound_statement" => Ok(Self::CompoundStatement(::std::boxed::Box::new(
                <CompoundStatement as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "concept_definition" => Ok(Self::ConceptDefinition(::std::boxed::Box::new(
                <ConceptDefinition as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "consteval_block_declaration" => {
                Ok(Self::ConstevalBlockDeclaration(::std::boxed::Box::new(
                    <ConstevalBlockDeclaration as ::treesitter_types::FromNode>::from_node(
                        node, src,
                    )?,
                )))
            }
            "continue_statement" => Ok(Self::ContinueStatement(::std::boxed::Box::new(
                <ContinueStatement as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "declaration" => Ok(Self::Declaration(::std::boxed::Box::new(
                <Declaration as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "do_statement" => Ok(Self::DoStatement(::std::boxed::Box::new(
                <DoStatement as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "expansion_statement" => Ok(Self::ExpansionStatement(::std::boxed::Box::new(
                <ExpansionStatement as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "export_declaration" => Ok(Self::ExportDeclaration(::std::boxed::Box::new(
                <ExportDeclaration as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "expression_statement" => Ok(Self::ExpressionStatement(::std::boxed::Box::new(
                <ExpressionStatement as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "for_range_loop" => Ok(Self::ForRangeLoop(::std::boxed::Box::new(
                <ForRangeLoop as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "for_statement" => Ok(Self::ForStatement(::std::boxed::Box::new(
                <ForStatement as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "function_definition" => Ok(Self::FunctionDefinition(::std::boxed::Box::new(
                <FunctionDefinition as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "global_module_fragment_declaration" => Ok(Self::GlobalModuleFragmentDeclaration(
                ::std::boxed::Box::new(
                    <GlobalModuleFragmentDeclaration as ::treesitter_types::FromNode>::from_node(
                        node, src,
                    )?,
                ),
            )),
            "goto_statement" => Ok(Self::GotoStatement(::std::boxed::Box::new(
                <GotoStatement as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "if_statement" => Ok(Self::IfStatement(::std::boxed::Box::new(
                <IfStatement as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "import_declaration" => Ok(Self::ImportDeclaration(::std::boxed::Box::new(
                <ImportDeclaration as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "labeled_statement" => Ok(Self::LabeledStatement(::std::boxed::Box::new(
                <LabeledStatement as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "linkage_specification" => Ok(Self::LinkageSpecification(::std::boxed::Box::new(
                <LinkageSpecification as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "module_declaration" => Ok(Self::ModuleDeclaration(::std::boxed::Box::new(
                <ModuleDeclaration as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "namespace_alias_definition" => {
                Ok(Self::NamespaceAliasDefinition(::std::boxed::Box::new(
                    <NamespaceAliasDefinition as ::treesitter_types::FromNode>::from_node(
                        node, src,
                    )?,
                )))
            }
            "namespace_definition" => Ok(Self::NamespaceDefinition(::std::boxed::Box::new(
                <NamespaceDefinition as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "preproc_call" => Ok(Self::PreprocCall(::std::boxed::Box::new(
                <PreprocCall as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "preproc_def" => Ok(Self::PreprocDef(::std::boxed::Box::new(
                <PreprocDef as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "preproc_function_def" => Ok(Self::PreprocFunctionDef(::std::boxed::Box::new(
                <PreprocFunctionDef as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "preproc_if" => Ok(Self::PreprocIf(::std::boxed::Box::new(
                <PreprocIf as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "preproc_ifdef" => Ok(Self::PreprocIfdef(::std::boxed::Box::new(
                <PreprocIfdef as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "preproc_include" => Ok(Self::PreprocInclude(::std::boxed::Box::new(
                <PreprocInclude as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "private_module_fragment_declaration" => Ok(Self::PrivateModuleFragmentDeclaration(
                ::std::boxed::Box::new(
                    <PrivateModuleFragmentDeclaration as ::treesitter_types::FromNode>::from_node(
                        node, src,
                    )?,
                ),
            )),
            "return_statement" => Ok(Self::ReturnStatement(::std::boxed::Box::new(
                <ReturnStatement as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "static_assert_declaration" => {
                Ok(Self::StaticAssertDeclaration(::std::boxed::Box::new(
                    <StaticAssertDeclaration as ::treesitter_types::FromNode>::from_node(
                        node, src,
                    )?,
                )))
            }
            "switch_statement" => Ok(Self::SwitchStatement(::std::boxed::Box::new(
                <SwitchStatement as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "template_declaration" => Ok(Self::TemplateDeclaration(::std::boxed::Box::new(
                <TemplateDeclaration as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "template_instantiation" => Ok(Self::TemplateInstantiation(::std::boxed::Box::new(
                <TemplateInstantiation as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "throw_statement" => Ok(Self::ThrowStatement(::std::boxed::Box::new(
                <ThrowStatement as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "try_statement" => Ok(Self::TryStatement(::std::boxed::Box::new(
                <TryStatement as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "type_definition" => Ok(Self::TypeDefinition(::std::boxed::Box::new(
                <TypeDefinition as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "using_declaration" => Ok(Self::UsingDeclaration(::std::boxed::Box::new(
                <UsingDeclaration as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "while_statement" => Ok(Self::WhileStatement(::std::boxed::Box::new(
                <WhileStatement as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            _other => {
                if let Ok(v) = <TypeSpecifier as ::treesitter_types::FromNode>::from_node(node, src)
                {
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
            Self::AliasDeclaration(inner) => inner.span(),
            Self::AttributedStatement(inner) => inner.span(),
            Self::BreakStatement(inner) => inner.span(),
            Self::CaseStatement(inner) => inner.span(),
            Self::CoReturnStatement(inner) => inner.span(),
            Self::CoYieldStatement(inner) => inner.span(),
            Self::CompoundStatement(inner) => inner.span(),
            Self::ConceptDefinition(inner) => inner.span(),
            Self::ConstevalBlockDeclaration(inner) => inner.span(),
            Self::ContinueStatement(inner) => inner.span(),
            Self::Declaration(inner) => inner.span(),
            Self::DoStatement(inner) => inner.span(),
            Self::ExpansionStatement(inner) => inner.span(),
            Self::ExportDeclaration(inner) => inner.span(),
            Self::ExpressionStatement(inner) => inner.span(),
            Self::ForRangeLoop(inner) => inner.span(),
            Self::ForStatement(inner) => inner.span(),
            Self::FunctionDefinition(inner) => inner.span(),
            Self::GlobalModuleFragmentDeclaration(inner) => inner.span(),
            Self::GotoStatement(inner) => inner.span(),
            Self::IfStatement(inner) => inner.span(),
            Self::ImportDeclaration(inner) => inner.span(),
            Self::LabeledStatement(inner) => inner.span(),
            Self::LinkageSpecification(inner) => inner.span(),
            Self::ModuleDeclaration(inner) => inner.span(),
            Self::NamespaceAliasDefinition(inner) => inner.span(),
            Self::NamespaceDefinition(inner) => inner.span(),
            Self::PreprocCall(inner) => inner.span(),
            Self::PreprocDef(inner) => inner.span(),
            Self::PreprocFunctionDef(inner) => inner.span(),
            Self::PreprocIf(inner) => inner.span(),
            Self::PreprocIfdef(inner) => inner.span(),
            Self::PreprocInclude(inner) => inner.span(),
            Self::PrivateModuleFragmentDeclaration(inner) => inner.span(),
            Self::ReturnStatement(inner) => inner.span(),
            Self::StaticAssertDeclaration(inner) => inner.span(),
            Self::SwitchStatement(inner) => inner.span(),
            Self::TemplateDeclaration(inner) => inner.span(),
            Self::TemplateInstantiation(inner) => inner.span(),
            Self::ThrowStatement(inner) => inner.span(),
            Self::TryStatement(inner) => inner.span(),
            Self::TypeDefinition(inner) => inner.span(),
            Self::TypeSpecifier(inner) => inner.span(),
            Self::UsingDeclaration(inner) => inner.span(),
            Self::WhileStatement(inner) => inner.span(),
        }
    }
}
#[derive(Debug, Clone)]
pub enum TryStatementChildren<'tree> {
    CatchClause(::std::boxed::Box<CatchClause<'tree>>),
    FieldInitializerList(::std::boxed::Box<FieldInitializerList<'tree>>),
}
impl<'tree> ::treesitter_types::FromNode<'tree> for TryStatementChildren<'tree> {
    #[allow(clippy::collapsible_else_if)]
    fn from_node(
        node: ::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        match node.kind() {
            "catch_clause" => Ok(Self::CatchClause(::std::boxed::Box::new(
                <CatchClause as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "field_initializer_list" => Ok(Self::FieldInitializerList(::std::boxed::Box::new(
                <FieldInitializerList as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            other => Err(::treesitter_types::ParseError::unexpected_kind(other, node)),
        }
    }
}
impl ::treesitter_types::Spanned for TryStatementChildren<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        match self {
            Self::CatchClause(inner) => inner.span(),
            Self::FieldInitializerList(inner) => inner.span(),
        }
    }
}
#[derive(Debug, Clone)]
pub enum TypeDefinitionChildren<'tree> {
    AttributeSpecifier(::std::boxed::Box<AttributeSpecifier<'tree>>),
    TypeQualifier(::std::boxed::Box<TypeQualifier<'tree>>),
}
impl<'tree> ::treesitter_types::FromNode<'tree> for TypeDefinitionChildren<'tree> {
    #[allow(clippy::collapsible_else_if)]
    fn from_node(
        node: ::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        match node.kind() {
            "attribute_specifier" => Ok(Self::AttributeSpecifier(::std::boxed::Box::new(
                <AttributeSpecifier as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "type_qualifier" => Ok(Self::TypeQualifier(::std::boxed::Box::new(
                <TypeQualifier as ::treesitter_types::FromNode>::from_node(node, src)?,
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
#[derive(Debug, Clone)]
pub enum TypeRequirementChildren<'tree> {
    QualifiedIdentifier(::std::boxed::Box<QualifiedIdentifier<'tree>>),
    SpliceTypeSpecifier(::std::boxed::Box<SpliceTypeSpecifier<'tree>>),
    TemplateType(::std::boxed::Box<TemplateType<'tree>>),
    TypeIdentifier(::std::boxed::Box<TypeIdentifier<'tree>>),
}
impl<'tree> ::treesitter_types::FromNode<'tree> for TypeRequirementChildren<'tree> {
    #[allow(clippy::collapsible_else_if)]
    fn from_node(
        node: ::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        match node.kind() {
            "qualified_identifier" => Ok(Self::QualifiedIdentifier(::std::boxed::Box::new(
                <QualifiedIdentifier as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "splice_type_specifier" => Ok(Self::SpliceTypeSpecifier(::std::boxed::Box::new(
                <SpliceTypeSpecifier as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "template_type" => Ok(Self::TemplateType(::std::boxed::Box::new(
                <TemplateType as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "type_identifier" => Ok(Self::TypeIdentifier(::std::boxed::Box::new(
                <TypeIdentifier as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            other => Err(::treesitter_types::ParseError::unexpected_kind(other, node)),
        }
    }
}
impl ::treesitter_types::Spanned for TypeRequirementChildren<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        match self {
            Self::QualifiedIdentifier(inner) => inner.span(),
            Self::SpliceTypeSpecifier(inner) => inner.span(),
            Self::TemplateType(inner) => inner.span(),
            Self::TypeIdentifier(inner) => inner.span(),
        }
    }
}
#[derive(Debug, Clone)]
pub enum UnaryExpressionArgument<'tree> {
    Expression(::std::boxed::Box<Expression<'tree>>),
    PreprocDefined(::std::boxed::Box<PreprocDefined<'tree>>),
}
impl<'tree> ::treesitter_types::FromNode<'tree> for UnaryExpressionArgument<'tree> {
    #[allow(clippy::collapsible_else_if)]
    fn from_node(
        node: ::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        match node.kind() {
            "preproc_defined" => Ok(Self::PreprocDefined(::std::boxed::Box::new(
                <PreprocDefined as ::treesitter_types::FromNode>::from_node(node, src)?,
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
impl ::treesitter_types::Spanned for UnaryExpressionArgument<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        match self {
            Self::Expression(inner) => inner.span(),
            Self::PreprocDefined(inner) => inner.span(),
        }
    }
}
#[derive(Debug, Clone)]
pub enum UnaryExpressionOperator {
    Bang(::treesitter_types::Span),
    Plus(::treesitter_types::Span),
    Minus(::treesitter_types::Span),
    Compl(::treesitter_types::Span),
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
            "!" => Ok(Self::Bang(::treesitter_types::Span::from(node))),
            "+" => Ok(Self::Plus(::treesitter_types::Span::from(node))),
            "-" => Ok(Self::Minus(::treesitter_types::Span::from(node))),
            "compl" => Ok(Self::Compl(::treesitter_types::Span::from(node))),
            "not" => Ok(Self::Not(::treesitter_types::Span::from(node))),
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
            Self::Compl(span) => *span,
            Self::Not(span) => *span,
            Self::Tilde(span) => *span,
        }
    }
}
#[derive(Debug, Clone)]
pub enum UnionSpecifierName<'tree> {
    QualifiedIdentifier(::std::boxed::Box<QualifiedIdentifier<'tree>>),
    SpliceTypeSpecifier(::std::boxed::Box<SpliceTypeSpecifier<'tree>>),
    TemplateType(::std::boxed::Box<TemplateType<'tree>>),
    TypeIdentifier(::std::boxed::Box<TypeIdentifier<'tree>>),
}
impl<'tree> ::treesitter_types::FromNode<'tree> for UnionSpecifierName<'tree> {
    #[allow(clippy::collapsible_else_if)]
    fn from_node(
        node: ::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        match node.kind() {
            "qualified_identifier" => Ok(Self::QualifiedIdentifier(::std::boxed::Box::new(
                <QualifiedIdentifier as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "splice_type_specifier" => Ok(Self::SpliceTypeSpecifier(::std::boxed::Box::new(
                <SpliceTypeSpecifier as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "template_type" => Ok(Self::TemplateType(::std::boxed::Box::new(
                <TemplateType as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "type_identifier" => Ok(Self::TypeIdentifier(::std::boxed::Box::new(
                <TypeIdentifier as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            other => Err(::treesitter_types::ParseError::unexpected_kind(other, node)),
        }
    }
}
impl ::treesitter_types::Spanned for UnionSpecifierName<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        match self {
            Self::QualifiedIdentifier(inner) => inner.span(),
            Self::SpliceTypeSpecifier(inner) => inner.span(),
            Self::TemplateType(inner) => inner.span(),
            Self::TypeIdentifier(inner) => inner.span(),
        }
    }
}
#[derive(Debug, Clone)]
pub enum UnionSpecifierChildren<'tree> {
    AlignasQualifier(::std::boxed::Box<AlignasQualifier<'tree>>),
    AttributeDeclaration(::std::boxed::Box<AttributeDeclaration<'tree>>),
    AttributeSpecifier(::std::boxed::Box<AttributeSpecifier<'tree>>),
    BaseClassClause(::std::boxed::Box<BaseClassClause<'tree>>),
    MsDeclspecModifier(::std::boxed::Box<MsDeclspecModifier<'tree>>),
    VirtualSpecifier(::std::boxed::Box<VirtualSpecifier<'tree>>),
}
impl<'tree> ::treesitter_types::FromNode<'tree> for UnionSpecifierChildren<'tree> {
    #[allow(clippy::collapsible_else_if)]
    fn from_node(
        node: ::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        match node.kind() {
            "alignas_qualifier" => Ok(Self::AlignasQualifier(::std::boxed::Box::new(
                <AlignasQualifier as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "attribute_declaration" => Ok(Self::AttributeDeclaration(::std::boxed::Box::new(
                <AttributeDeclaration as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "attribute_specifier" => Ok(Self::AttributeSpecifier(::std::boxed::Box::new(
                <AttributeSpecifier as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "base_class_clause" => Ok(Self::BaseClassClause(::std::boxed::Box::new(
                <BaseClassClause as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "ms_declspec_modifier" => Ok(Self::MsDeclspecModifier(::std::boxed::Box::new(
                <MsDeclspecModifier as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "virtual_specifier" => Ok(Self::VirtualSpecifier(::std::boxed::Box::new(
                <VirtualSpecifier as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            other => Err(::treesitter_types::ParseError::unexpected_kind(other, node)),
        }
    }
}
impl ::treesitter_types::Spanned for UnionSpecifierChildren<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        match self {
            Self::AlignasQualifier(inner) => inner.span(),
            Self::AttributeDeclaration(inner) => inner.span(),
            Self::AttributeSpecifier(inner) => inner.span(),
            Self::BaseClassClause(inner) => inner.span(),
            Self::MsDeclspecModifier(inner) => inner.span(),
            Self::VirtualSpecifier(inner) => inner.span(),
        }
    }
}
#[derive(Debug, Clone)]
pub enum UpdateExpressionOperator {
    PlusPlus(::treesitter_types::Span),
    MinusMinus(::treesitter_types::Span),
}
impl<'tree> ::treesitter_types::FromNode<'tree> for UpdateExpressionOperator {
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
impl ::treesitter_types::Spanned for UpdateExpressionOperator {
    fn span(&self) -> ::treesitter_types::Span {
        match self {
            Self::PlusPlus(span) => *span,
            Self::MinusMinus(span) => *span,
        }
    }
}
#[derive(Debug, Clone)]
pub enum UserDefinedLiteralChildren<'tree> {
    CharLiteral(::std::boxed::Box<CharLiteral<'tree>>),
    ConcatenatedString(::std::boxed::Box<ConcatenatedString<'tree>>),
    LiteralSuffix(::std::boxed::Box<LiteralSuffix<'tree>>),
    NumberLiteral(::std::boxed::Box<NumberLiteral<'tree>>),
    RawStringLiteral(::std::boxed::Box<RawStringLiteral<'tree>>),
    StringLiteral(::std::boxed::Box<StringLiteral<'tree>>),
}
impl<'tree> ::treesitter_types::FromNode<'tree> for UserDefinedLiteralChildren<'tree> {
    #[allow(clippy::collapsible_else_if)]
    fn from_node(
        node: ::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        match node.kind() {
            "char_literal" => Ok(Self::CharLiteral(::std::boxed::Box::new(
                <CharLiteral as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "concatenated_string" => Ok(Self::ConcatenatedString(::std::boxed::Box::new(
                <ConcatenatedString as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "literal_suffix" => Ok(Self::LiteralSuffix(::std::boxed::Box::new(
                <LiteralSuffix as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "number_literal" => Ok(Self::NumberLiteral(::std::boxed::Box::new(
                <NumberLiteral as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "raw_string_literal" => Ok(Self::RawStringLiteral(::std::boxed::Box::new(
                <RawStringLiteral as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "string_literal" => Ok(Self::StringLiteral(::std::boxed::Box::new(
                <StringLiteral as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            other => Err(::treesitter_types::ParseError::unexpected_kind(other, node)),
        }
    }
}
impl ::treesitter_types::Spanned for UserDefinedLiteralChildren<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        match self {
            Self::CharLiteral(inner) => inner.span(),
            Self::ConcatenatedString(inner) => inner.span(),
            Self::LiteralSuffix(inner) => inner.span(),
            Self::NumberLiteral(inner) => inner.span(),
            Self::RawStringLiteral(inner) => inner.span(),
            Self::StringLiteral(inner) => inner.span(),
        }
    }
}
#[derive(Debug, Clone)]
pub enum UsingDeclarationChildren<'tree> {
    AttributeDeclaration(::std::boxed::Box<AttributeDeclaration<'tree>>),
    Identifier(::std::boxed::Box<Identifier<'tree>>),
    QualifiedIdentifier(::std::boxed::Box<QualifiedIdentifier<'tree>>),
    SpliceTypeSpecifier(::std::boxed::Box<SpliceTypeSpecifier<'tree>>),
}
impl<'tree> ::treesitter_types::FromNode<'tree> for UsingDeclarationChildren<'tree> {
    #[allow(clippy::collapsible_else_if)]
    fn from_node(
        node: ::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        match node.kind() {
            "attribute_declaration" => Ok(Self::AttributeDeclaration(::std::boxed::Box::new(
                <AttributeDeclaration as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "identifier" => Ok(Self::Identifier(::std::boxed::Box::new(
                <Identifier as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "qualified_identifier" => Ok(Self::QualifiedIdentifier(::std::boxed::Box::new(
                <QualifiedIdentifier as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "splice_type_specifier" => Ok(Self::SpliceTypeSpecifier(::std::boxed::Box::new(
                <SpliceTypeSpecifier as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            other => Err(::treesitter_types::ParseError::unexpected_kind(other, node)),
        }
    }
}
impl ::treesitter_types::Spanned for UsingDeclarationChildren<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        match self {
            Self::AttributeDeclaration(inner) => inner.span(),
            Self::Identifier(inner) => inner.span(),
            Self::QualifiedIdentifier(inner) => inner.span(),
            Self::SpliceTypeSpecifier(inner) => inner.span(),
        }
    }
}
#[derive(Debug, Clone)]
pub enum VariadicParameterDeclarationDeclarator<'tree> {
    ReferenceDeclarator(::std::boxed::Box<ReferenceDeclarator<'tree>>),
    VariadicDeclarator(::std::boxed::Box<VariadicDeclarator<'tree>>),
}
impl<'tree> ::treesitter_types::FromNode<'tree> for VariadicParameterDeclarationDeclarator<'tree> {
    #[allow(clippy::collapsible_else_if)]
    fn from_node(
        node: ::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        match node.kind() {
            "reference_declarator" => Ok(Self::ReferenceDeclarator(::std::boxed::Box::new(
                <ReferenceDeclarator as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "variadic_declarator" => Ok(Self::VariadicDeclarator(::std::boxed::Box::new(
                <VariadicDeclarator as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            other => Err(::treesitter_types::ParseError::unexpected_kind(other, node)),
        }
    }
}
impl ::treesitter_types::Spanned for VariadicParameterDeclarationDeclarator<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        match self {
            Self::ReferenceDeclarator(inner) => inner.span(),
            Self::VariadicDeclarator(inner) => inner.span(),
        }
    }
}
#[derive(Debug, Clone)]
pub enum VariadicParameterDeclarationChildren<'tree> {
    AttributeDeclaration(::std::boxed::Box<AttributeDeclaration<'tree>>),
    AttributeSpecifier(::std::boxed::Box<AttributeSpecifier<'tree>>),
    MsDeclspecModifier(::std::boxed::Box<MsDeclspecModifier<'tree>>),
    StorageClassSpecifier(::std::boxed::Box<StorageClassSpecifier<'tree>>),
    TypeQualifier(::std::boxed::Box<TypeQualifier<'tree>>),
}
impl<'tree> ::treesitter_types::FromNode<'tree> for VariadicParameterDeclarationChildren<'tree> {
    #[allow(clippy::collapsible_else_if)]
    fn from_node(
        node: ::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        match node.kind() {
            "attribute_declaration" => Ok(Self::AttributeDeclaration(::std::boxed::Box::new(
                <AttributeDeclaration as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "attribute_specifier" => Ok(Self::AttributeSpecifier(::std::boxed::Box::new(
                <AttributeSpecifier as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "ms_declspec_modifier" => Ok(Self::MsDeclspecModifier(::std::boxed::Box::new(
                <MsDeclspecModifier as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "storage_class_specifier" => Ok(Self::StorageClassSpecifier(::std::boxed::Box::new(
                <StorageClassSpecifier as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "type_qualifier" => Ok(Self::TypeQualifier(::std::boxed::Box::new(
                <TypeQualifier as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            other => Err(::treesitter_types::ParseError::unexpected_kind(other, node)),
        }
    }
}
impl ::treesitter_types::Spanned for VariadicParameterDeclarationChildren<'_> {
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
#[derive(Debug, Clone)]
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
    AbstractReferenceDeclarator(AbstractReferenceDeclarator<'tree>),
    AccessSpecifier(AccessSpecifier<'tree>),
    AliasDeclaration(AliasDeclaration<'tree>),
    AlignasQualifier(AlignasQualifier<'tree>),
    AlignofExpression(AlignofExpression<'tree>),
    Annotation(Annotation<'tree>),
    ArgumentList(ArgumentList<'tree>),
    ArrayDeclarator(ArrayDeclarator<'tree>),
    AssignmentExpression(AssignmentExpression<'tree>),
    Attribute(Attribute<'tree>),
    AttributeDeclaration(AttributeDeclaration<'tree>),
    AttributeSpecifier(AttributeSpecifier<'tree>),
    AttributedDeclarator(AttributedDeclarator<'tree>),
    AttributedStatement(AttributedStatement<'tree>),
    BaseClassClause(BaseClassClause<'tree>),
    BinaryExpression(BinaryExpression<'tree>),
    BitfieldClause(BitfieldClause<'tree>),
    BreakStatement(BreakStatement<'tree>),
    CallExpression(CallExpression<'tree>),
    CaseStatement(CaseStatement<'tree>),
    CastExpression(CastExpression<'tree>),
    CatchClause(CatchClause<'tree>),
    CharLiteral(CharLiteral<'tree>),
    ClassSpecifier(ClassSpecifier<'tree>),
    CoAwaitExpression(CoAwaitExpression<'tree>),
    CoReturnStatement(CoReturnStatement<'tree>),
    CoYieldStatement(CoYieldStatement<'tree>),
    CommaExpression(CommaExpression<'tree>),
    CompoundLiteralExpression(CompoundLiteralExpression<'tree>),
    CompoundRequirement(CompoundRequirement<'tree>),
    CompoundStatement(CompoundStatement<'tree>),
    ConcatenatedString(ConcatenatedString<'tree>),
    ConceptDefinition(ConceptDefinition<'tree>),
    ConditionClause(ConditionClause<'tree>),
    ConditionalExpression(ConditionalExpression<'tree>),
    ConstevalBlockDeclaration(ConstevalBlockDeclaration<'tree>),
    ConstraintConjunction(ConstraintConjunction<'tree>),
    ConstraintDisjunction(ConstraintDisjunction<'tree>),
    ContinueStatement(ContinueStatement<'tree>),
    Declaration(Declaration<'tree>),
    DeclarationList(DeclarationList<'tree>),
    Decltype(Decltype<'tree>),
    DefaultMethodClause(DefaultMethodClause<'tree>),
    DeleteExpression(DeleteExpression<'tree>),
    DeleteMethodClause(DeleteMethodClause<'tree>),
    DependentName(DependentName<'tree>),
    DependentType(DependentType<'tree>),
    DestructorName(DestructorName<'tree>),
    DoStatement(DoStatement<'tree>),
    ElseClause(ElseClause<'tree>),
    EnumSpecifier(EnumSpecifier<'tree>),
    Enumerator(Enumerator<'tree>),
    EnumeratorList(EnumeratorList<'tree>),
    ExpansionStatement(ExpansionStatement<'tree>),
    ExplicitFunctionSpecifier(ExplicitFunctionSpecifier<'tree>),
    ExplicitObjectParameterDeclaration(ExplicitObjectParameterDeclaration<'tree>),
    ExportDeclaration(ExportDeclaration<'tree>),
    ExpressionStatement(ExpressionStatement<'tree>),
    ExtensionExpression(ExtensionExpression<'tree>),
    FieldDeclaration(FieldDeclaration<'tree>),
    FieldDeclarationList(FieldDeclarationList<'tree>),
    FieldDesignator(FieldDesignator<'tree>),
    FieldExpression(FieldExpression<'tree>),
    FieldInitializer(FieldInitializer<'tree>),
    FieldInitializerList(FieldInitializerList<'tree>),
    FoldExpression(FoldExpression<'tree>),
    ForRangeLoop(ForRangeLoop<'tree>),
    ForStatement(ForStatement<'tree>),
    FriendDeclaration(FriendDeclaration<'tree>),
    FunctionDeclarator(FunctionDeclarator<'tree>),
    FunctionDefinition(FunctionDefinition<'tree>),
    GenericExpression(GenericExpression<'tree>),
    GlobalModuleFragmentDeclaration(GlobalModuleFragmentDeclaration<'tree>),
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
    ImportDeclaration(ImportDeclaration<'tree>),
    InitDeclarator(InitDeclarator<'tree>),
    InitStatement(InitStatement<'tree>),
    InitializerList(InitializerList<'tree>),
    InitializerPair(InitializerPair<'tree>),
    LabeledStatement(LabeledStatement<'tree>),
    LambdaCaptureInitializer(LambdaCaptureInitializer<'tree>),
    LambdaCaptureSpecifier(LambdaCaptureSpecifier<'tree>),
    LambdaDeclarator(LambdaDeclarator<'tree>),
    LambdaDefaultCapture(LambdaDefaultCapture<'tree>),
    LambdaExpression(LambdaExpression<'tree>),
    LambdaSpecifier(LambdaSpecifier<'tree>),
    LinkageSpecification(LinkageSpecification<'tree>),
    ModuleDeclaration(ModuleDeclaration<'tree>),
    ModuleName(ModuleName<'tree>),
    ModulePartition(ModulePartition<'tree>),
    MsBasedModifier(MsBasedModifier<'tree>),
    MsCallModifier(MsCallModifier<'tree>),
    MsDeclspecModifier(MsDeclspecModifier<'tree>),
    MsPointerModifier(MsPointerModifier<'tree>),
    MsUnalignedPtrModifier(MsUnalignedPtrModifier<'tree>),
    NamespaceAliasDefinition(NamespaceAliasDefinition<'tree>),
    NamespaceDefinition(NamespaceDefinition<'tree>),
    NestedNamespaceSpecifier(NestedNamespaceSpecifier<'tree>),
    NewDeclarator(NewDeclarator<'tree>),
    NewExpression(NewExpression<'tree>),
    Noexcept(Noexcept<'tree>),
    Null(Null<'tree>),
    OffsetofExpression(OffsetofExpression<'tree>),
    OperatorCast(OperatorCast<'tree>),
    OperatorName(OperatorName<'tree>),
    OptionalParameterDeclaration(OptionalParameterDeclaration<'tree>),
    OptionalTypeParameterDeclaration(OptionalTypeParameterDeclaration<'tree>),
    ParameterDeclaration(ParameterDeclaration<'tree>),
    ParameterList(ParameterList<'tree>),
    ParameterPackExpansion(ParameterPackExpansion<'tree>),
    ParenthesizedDeclarator(ParenthesizedDeclarator<'tree>),
    ParenthesizedExpression(ParenthesizedExpression<'tree>),
    PlaceholderTypeSpecifier(PlaceholderTypeSpecifier<'tree>),
    PointerDeclarator(PointerDeclarator<'tree>),
    PointerExpression(PointerExpression<'tree>),
    PointerTypeDeclarator(PointerTypeDeclarator<'tree>),
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
    PrivateModuleFragmentDeclaration(PrivateModuleFragmentDeclaration<'tree>),
    PureVirtualClause(PureVirtualClause<'tree>),
    QualifiedIdentifier(QualifiedIdentifier<'tree>),
    RawStringLiteral(RawStringLiteral<'tree>),
    RefQualifier(RefQualifier<'tree>),
    ReferenceDeclarator(ReferenceDeclarator<'tree>),
    ReflectExpression(ReflectExpression<'tree>),
    RequirementSeq(RequirementSeq<'tree>),
    RequiresClause(RequiresClause<'tree>),
    RequiresExpression(RequiresExpression<'tree>),
    ReturnStatement(ReturnStatement<'tree>),
    SehExceptClause(SehExceptClause<'tree>),
    SehFinallyClause(SehFinallyClause<'tree>),
    SehLeaveStatement(SehLeaveStatement<'tree>),
    SehTryStatement(SehTryStatement<'tree>),
    SimpleRequirement(SimpleRequirement<'tree>),
    SizedTypeSpecifier(SizedTypeSpecifier<'tree>),
    SizeofExpression(SizeofExpression<'tree>),
    SpliceExpression(SpliceExpression<'tree>),
    SpliceSpecifier(SpliceSpecifier<'tree>),
    SpliceTypeSpecifier(SpliceTypeSpecifier<'tree>),
    StaticAssertDeclaration(StaticAssertDeclaration<'tree>),
    StorageClassSpecifier(StorageClassSpecifier<'tree>),
    StringLiteral(StringLiteral<'tree>),
    StructSpecifier(StructSpecifier<'tree>),
    StructuredBindingDeclarator(StructuredBindingDeclarator<'tree>),
    SubscriptArgumentList(SubscriptArgumentList<'tree>),
    SubscriptDesignator(SubscriptDesignator<'tree>),
    SubscriptExpression(SubscriptExpression<'tree>),
    SubscriptRangeDesignator(SubscriptRangeDesignator<'tree>),
    SwitchStatement(SwitchStatement<'tree>),
    TemplateArgumentList(TemplateArgumentList<'tree>),
    TemplateDeclaration(TemplateDeclaration<'tree>),
    TemplateFunction(TemplateFunction<'tree>),
    TemplateInstantiation(TemplateInstantiation<'tree>),
    TemplateMethod(TemplateMethod<'tree>),
    TemplateParameterList(TemplateParameterList<'tree>),
    TemplateTemplateParameterDeclaration(TemplateTemplateParameterDeclaration<'tree>),
    TemplateType(TemplateType<'tree>),
    ThrowSpecifier(ThrowSpecifier<'tree>),
    ThrowStatement(ThrowStatement<'tree>),
    TrailingReturnType(TrailingReturnType<'tree>),
    TranslationUnit(TranslationUnit<'tree>),
    TryStatement(TryStatement<'tree>),
    TypeDefinition(TypeDefinition<'tree>),
    TypeDescriptor(TypeDescriptor<'tree>),
    TypeParameterDeclaration(TypeParameterDeclaration<'tree>),
    TypeQualifier(TypeQualifier<'tree>),
    TypeRequirement(TypeRequirement<'tree>),
    UnaryExpression(UnaryExpression<'tree>),
    UnionSpecifier(UnionSpecifier<'tree>),
    UpdateExpression(UpdateExpression<'tree>),
    UserDefinedLiteral(UserDefinedLiteral<'tree>),
    UsingDeclaration(UsingDeclaration<'tree>),
    VariadicDeclarator(VariadicDeclarator<'tree>),
    VariadicParameterDeclaration(VariadicParameterDeclaration<'tree>),
    VariadicTypeParameterDeclaration(VariadicTypeParameterDeclaration<'tree>),
    VirtualSpecifier(VirtualSpecifier<'tree>),
    WhileStatement(WhileStatement<'tree>),
    Auto(Auto<'tree>),
    Character(Character<'tree>),
    Comment(Comment<'tree>),
    EscapeSequence(EscapeSequence<'tree>),
    False(False<'tree>),
    FieldIdentifier(FieldIdentifier<'tree>),
    Identifier(Identifier<'tree>),
    LiteralSuffix(LiteralSuffix<'tree>),
    MsRestrictModifier(MsRestrictModifier<'tree>),
    MsSignedPtrModifier(MsSignedPtrModifier<'tree>),
    MsUnsignedPtrModifier(MsUnsignedPtrModifier<'tree>),
    NamespaceIdentifier(NamespaceIdentifier<'tree>),
    NumberLiteral(NumberLiteral<'tree>),
    PreprocArg(PreprocArg<'tree>),
    PreprocDirective(PreprocDirective<'tree>),
    PrimitiveType(PrimitiveType<'tree>),
    RawStringContent(RawStringContent<'tree>),
    RawStringDelimiter(RawStringDelimiter<'tree>),
    StatementIdentifier(StatementIdentifier<'tree>),
    StringContent(StringContent<'tree>),
    SystemLibString(SystemLibString<'tree>),
    This(This<'tree>),
    True(True<'tree>),
    TypeIdentifier(TypeIdentifier<'tree>),
    Unknown(::tree_sitter::Node<'tree>),
}
impl<'tree> AnyNode<'tree> {
    pub fn from_node(node: ::tree_sitter::Node<'tree>, src: &'tree [u8]) -> Self {
        match node.kind() {
            "_abstract_declarator" => {
                <AbstractDeclarator as ::treesitter_types::FromNode>::from_node(node, src)
                    .map(Self::AbstractDeclarator)
                    .unwrap_or(Self::Unknown(node))
            }
            "_declarator" => <Declarator as ::treesitter_types::FromNode>::from_node(node, src)
                .map(Self::Declarator)
                .unwrap_or(Self::Unknown(node)),
            "_field_declarator" => {
                <FieldDeclarator as ::treesitter_types::FromNode>::from_node(node, src)
                    .map(Self::FieldDeclarator)
                    .unwrap_or(Self::Unknown(node))
            }
            "_type_declarator" => {
                <TypeDeclarator as ::treesitter_types::FromNode>::from_node(node, src)
                    .map(Self::TypeDeclarator)
                    .unwrap_or(Self::Unknown(node))
            }
            "expression" => <Expression as ::treesitter_types::FromNode>::from_node(node, src)
                .map(Self::Expression)
                .unwrap_or(Self::Unknown(node)),
            "statement" => <Statement as ::treesitter_types::FromNode>::from_node(node, src)
                .map(Self::Statement)
                .unwrap_or(Self::Unknown(node)),
            "type_specifier" => {
                <TypeSpecifier as ::treesitter_types::FromNode>::from_node(node, src)
                    .map(Self::TypeSpecifier)
                    .unwrap_or(Self::Unknown(node))
            }
            "abstract_array_declarator" => {
                <AbstractArrayDeclarator as ::treesitter_types::FromNode>::from_node(node, src)
                    .map(Self::AbstractArrayDeclarator)
                    .unwrap_or(Self::Unknown(node))
            }
            "abstract_function_declarator" => {
                <AbstractFunctionDeclarator as ::treesitter_types::FromNode>::from_node(node, src)
                    .map(Self::AbstractFunctionDeclarator)
                    .unwrap_or(Self::Unknown(node))
            }
            "abstract_parenthesized_declarator" => {
                <AbstractParenthesizedDeclarator as ::treesitter_types::FromNode>::from_node(
                    node, src,
                )
                .map(Self::AbstractParenthesizedDeclarator)
                .unwrap_or(Self::Unknown(node))
            }
            "abstract_pointer_declarator" => {
                <AbstractPointerDeclarator as ::treesitter_types::FromNode>::from_node(node, src)
                    .map(Self::AbstractPointerDeclarator)
                    .unwrap_or(Self::Unknown(node))
            }
            "abstract_reference_declarator" => {
                <AbstractReferenceDeclarator as ::treesitter_types::FromNode>::from_node(node, src)
                    .map(Self::AbstractReferenceDeclarator)
                    .unwrap_or(Self::Unknown(node))
            }
            "access_specifier" => {
                <AccessSpecifier as ::treesitter_types::FromNode>::from_node(node, src)
                    .map(Self::AccessSpecifier)
                    .unwrap_or(Self::Unknown(node))
            }
            "alias_declaration" => {
                <AliasDeclaration as ::treesitter_types::FromNode>::from_node(node, src)
                    .map(Self::AliasDeclaration)
                    .unwrap_or(Self::Unknown(node))
            }
            "alignas_qualifier" => {
                <AlignasQualifier as ::treesitter_types::FromNode>::from_node(node, src)
                    .map(Self::AlignasQualifier)
                    .unwrap_or(Self::Unknown(node))
            }
            "alignof_expression" => {
                <AlignofExpression as ::treesitter_types::FromNode>::from_node(node, src)
                    .map(Self::AlignofExpression)
                    .unwrap_or(Self::Unknown(node))
            }
            "annotation" => <Annotation as ::treesitter_types::FromNode>::from_node(node, src)
                .map(Self::Annotation)
                .unwrap_or(Self::Unknown(node)),
            "argument_list" => <ArgumentList as ::treesitter_types::FromNode>::from_node(node, src)
                .map(Self::ArgumentList)
                .unwrap_or(Self::Unknown(node)),
            "array_declarator" => {
                <ArrayDeclarator as ::treesitter_types::FromNode>::from_node(node, src)
                    .map(Self::ArrayDeclarator)
                    .unwrap_or(Self::Unknown(node))
            }
            "assignment_expression" => {
                <AssignmentExpression as ::treesitter_types::FromNode>::from_node(node, src)
                    .map(Self::AssignmentExpression)
                    .unwrap_or(Self::Unknown(node))
            }
            "attribute" => <Attribute as ::treesitter_types::FromNode>::from_node(node, src)
                .map(Self::Attribute)
                .unwrap_or(Self::Unknown(node)),
            "attribute_declaration" => {
                <AttributeDeclaration as ::treesitter_types::FromNode>::from_node(node, src)
                    .map(Self::AttributeDeclaration)
                    .unwrap_or(Self::Unknown(node))
            }
            "attribute_specifier" => {
                <AttributeSpecifier as ::treesitter_types::FromNode>::from_node(node, src)
                    .map(Self::AttributeSpecifier)
                    .unwrap_or(Self::Unknown(node))
            }
            "attributed_declarator" => {
                <AttributedDeclarator as ::treesitter_types::FromNode>::from_node(node, src)
                    .map(Self::AttributedDeclarator)
                    .unwrap_or(Self::Unknown(node))
            }
            "attributed_statement" => {
                <AttributedStatement as ::treesitter_types::FromNode>::from_node(node, src)
                    .map(Self::AttributedStatement)
                    .unwrap_or(Self::Unknown(node))
            }
            "base_class_clause" => {
                <BaseClassClause as ::treesitter_types::FromNode>::from_node(node, src)
                    .map(Self::BaseClassClause)
                    .unwrap_or(Self::Unknown(node))
            }
            "binary_expression" => {
                <BinaryExpression as ::treesitter_types::FromNode>::from_node(node, src)
                    .map(Self::BinaryExpression)
                    .unwrap_or(Self::Unknown(node))
            }
            "bitfield_clause" => {
                <BitfieldClause as ::treesitter_types::FromNode>::from_node(node, src)
                    .map(Self::BitfieldClause)
                    .unwrap_or(Self::Unknown(node))
            }
            "break_statement" => {
                <BreakStatement as ::treesitter_types::FromNode>::from_node(node, src)
                    .map(Self::BreakStatement)
                    .unwrap_or(Self::Unknown(node))
            }
            "call_expression" => {
                <CallExpression as ::treesitter_types::FromNode>::from_node(node, src)
                    .map(Self::CallExpression)
                    .unwrap_or(Self::Unknown(node))
            }
            "case_statement" => {
                <CaseStatement as ::treesitter_types::FromNode>::from_node(node, src)
                    .map(Self::CaseStatement)
                    .unwrap_or(Self::Unknown(node))
            }
            "cast_expression" => {
                <CastExpression as ::treesitter_types::FromNode>::from_node(node, src)
                    .map(Self::CastExpression)
                    .unwrap_or(Self::Unknown(node))
            }
            "catch_clause" => <CatchClause as ::treesitter_types::FromNode>::from_node(node, src)
                .map(Self::CatchClause)
                .unwrap_or(Self::Unknown(node)),
            "char_literal" => <CharLiteral as ::treesitter_types::FromNode>::from_node(node, src)
                .map(Self::CharLiteral)
                .unwrap_or(Self::Unknown(node)),
            "class_specifier" => {
                <ClassSpecifier as ::treesitter_types::FromNode>::from_node(node, src)
                    .map(Self::ClassSpecifier)
                    .unwrap_or(Self::Unknown(node))
            }
            "co_await_expression" => {
                <CoAwaitExpression as ::treesitter_types::FromNode>::from_node(node, src)
                    .map(Self::CoAwaitExpression)
                    .unwrap_or(Self::Unknown(node))
            }
            "co_return_statement" => {
                <CoReturnStatement as ::treesitter_types::FromNode>::from_node(node, src)
                    .map(Self::CoReturnStatement)
                    .unwrap_or(Self::Unknown(node))
            }
            "co_yield_statement" => {
                <CoYieldStatement as ::treesitter_types::FromNode>::from_node(node, src)
                    .map(Self::CoYieldStatement)
                    .unwrap_or(Self::Unknown(node))
            }
            "comma_expression" => {
                <CommaExpression as ::treesitter_types::FromNode>::from_node(node, src)
                    .map(Self::CommaExpression)
                    .unwrap_or(Self::Unknown(node))
            }
            "compound_literal_expression" => {
                <CompoundLiteralExpression as ::treesitter_types::FromNode>::from_node(node, src)
                    .map(Self::CompoundLiteralExpression)
                    .unwrap_or(Self::Unknown(node))
            }
            "compound_requirement" => {
                <CompoundRequirement as ::treesitter_types::FromNode>::from_node(node, src)
                    .map(Self::CompoundRequirement)
                    .unwrap_or(Self::Unknown(node))
            }
            "compound_statement" => {
                <CompoundStatement as ::treesitter_types::FromNode>::from_node(node, src)
                    .map(Self::CompoundStatement)
                    .unwrap_or(Self::Unknown(node))
            }
            "concatenated_string" => {
                <ConcatenatedString as ::treesitter_types::FromNode>::from_node(node, src)
                    .map(Self::ConcatenatedString)
                    .unwrap_or(Self::Unknown(node))
            }
            "concept_definition" => {
                <ConceptDefinition as ::treesitter_types::FromNode>::from_node(node, src)
                    .map(Self::ConceptDefinition)
                    .unwrap_or(Self::Unknown(node))
            }
            "condition_clause" => {
                <ConditionClause as ::treesitter_types::FromNode>::from_node(node, src)
                    .map(Self::ConditionClause)
                    .unwrap_or(Self::Unknown(node))
            }
            "conditional_expression" => {
                <ConditionalExpression as ::treesitter_types::FromNode>::from_node(node, src)
                    .map(Self::ConditionalExpression)
                    .unwrap_or(Self::Unknown(node))
            }
            "consteval_block_declaration" => {
                <ConstevalBlockDeclaration as ::treesitter_types::FromNode>::from_node(node, src)
                    .map(Self::ConstevalBlockDeclaration)
                    .unwrap_or(Self::Unknown(node))
            }
            "constraint_conjunction" => {
                <ConstraintConjunction as ::treesitter_types::FromNode>::from_node(node, src)
                    .map(Self::ConstraintConjunction)
                    .unwrap_or(Self::Unknown(node))
            }
            "constraint_disjunction" => {
                <ConstraintDisjunction as ::treesitter_types::FromNode>::from_node(node, src)
                    .map(Self::ConstraintDisjunction)
                    .unwrap_or(Self::Unknown(node))
            }
            "continue_statement" => {
                <ContinueStatement as ::treesitter_types::FromNode>::from_node(node, src)
                    .map(Self::ContinueStatement)
                    .unwrap_or(Self::Unknown(node))
            }
            "declaration" => <Declaration as ::treesitter_types::FromNode>::from_node(node, src)
                .map(Self::Declaration)
                .unwrap_or(Self::Unknown(node)),
            "declaration_list" => {
                <DeclarationList as ::treesitter_types::FromNode>::from_node(node, src)
                    .map(Self::DeclarationList)
                    .unwrap_or(Self::Unknown(node))
            }
            "decltype" => <Decltype as ::treesitter_types::FromNode>::from_node(node, src)
                .map(Self::Decltype)
                .unwrap_or(Self::Unknown(node)),
            "default_method_clause" => {
                <DefaultMethodClause as ::treesitter_types::FromNode>::from_node(node, src)
                    .map(Self::DefaultMethodClause)
                    .unwrap_or(Self::Unknown(node))
            }
            "delete_expression" => {
                <DeleteExpression as ::treesitter_types::FromNode>::from_node(node, src)
                    .map(Self::DeleteExpression)
                    .unwrap_or(Self::Unknown(node))
            }
            "delete_method_clause" => {
                <DeleteMethodClause as ::treesitter_types::FromNode>::from_node(node, src)
                    .map(Self::DeleteMethodClause)
                    .unwrap_or(Self::Unknown(node))
            }
            "dependent_name" => {
                <DependentName as ::treesitter_types::FromNode>::from_node(node, src)
                    .map(Self::DependentName)
                    .unwrap_or(Self::Unknown(node))
            }
            "dependent_type" => {
                <DependentType as ::treesitter_types::FromNode>::from_node(node, src)
                    .map(Self::DependentType)
                    .unwrap_or(Self::Unknown(node))
            }
            "destructor_name" => {
                <DestructorName as ::treesitter_types::FromNode>::from_node(node, src)
                    .map(Self::DestructorName)
                    .unwrap_or(Self::Unknown(node))
            }
            "do_statement" => <DoStatement as ::treesitter_types::FromNode>::from_node(node, src)
                .map(Self::DoStatement)
                .unwrap_or(Self::Unknown(node)),
            "else_clause" => <ElseClause as ::treesitter_types::FromNode>::from_node(node, src)
                .map(Self::ElseClause)
                .unwrap_or(Self::Unknown(node)),
            "enum_specifier" => {
                <EnumSpecifier as ::treesitter_types::FromNode>::from_node(node, src)
                    .map(Self::EnumSpecifier)
                    .unwrap_or(Self::Unknown(node))
            }
            "enumerator" => <Enumerator as ::treesitter_types::FromNode>::from_node(node, src)
                .map(Self::Enumerator)
                .unwrap_or(Self::Unknown(node)),
            "enumerator_list" => {
                <EnumeratorList as ::treesitter_types::FromNode>::from_node(node, src)
                    .map(Self::EnumeratorList)
                    .unwrap_or(Self::Unknown(node))
            }
            "expansion_statement" => {
                <ExpansionStatement as ::treesitter_types::FromNode>::from_node(node, src)
                    .map(Self::ExpansionStatement)
                    .unwrap_or(Self::Unknown(node))
            }
            "explicit_function_specifier" => {
                <ExplicitFunctionSpecifier as ::treesitter_types::FromNode>::from_node(node, src)
                    .map(Self::ExplicitFunctionSpecifier)
                    .unwrap_or(Self::Unknown(node))
            }
            "explicit_object_parameter_declaration" => {
                <ExplicitObjectParameterDeclaration as ::treesitter_types::FromNode>::from_node(
                    node, src,
                )
                .map(Self::ExplicitObjectParameterDeclaration)
                .unwrap_or(Self::Unknown(node))
            }
            "export_declaration" => {
                <ExportDeclaration as ::treesitter_types::FromNode>::from_node(node, src)
                    .map(Self::ExportDeclaration)
                    .unwrap_or(Self::Unknown(node))
            }
            "expression_statement" => {
                <ExpressionStatement as ::treesitter_types::FromNode>::from_node(node, src)
                    .map(Self::ExpressionStatement)
                    .unwrap_or(Self::Unknown(node))
            }
            "extension_expression" => {
                <ExtensionExpression as ::treesitter_types::FromNode>::from_node(node, src)
                    .map(Self::ExtensionExpression)
                    .unwrap_or(Self::Unknown(node))
            }
            "field_declaration" => {
                <FieldDeclaration as ::treesitter_types::FromNode>::from_node(node, src)
                    .map(Self::FieldDeclaration)
                    .unwrap_or(Self::Unknown(node))
            }
            "field_declaration_list" => {
                <FieldDeclarationList as ::treesitter_types::FromNode>::from_node(node, src)
                    .map(Self::FieldDeclarationList)
                    .unwrap_or(Self::Unknown(node))
            }
            "field_designator" => {
                <FieldDesignator as ::treesitter_types::FromNode>::from_node(node, src)
                    .map(Self::FieldDesignator)
                    .unwrap_or(Self::Unknown(node))
            }
            "field_expression" => {
                <FieldExpression as ::treesitter_types::FromNode>::from_node(node, src)
                    .map(Self::FieldExpression)
                    .unwrap_or(Self::Unknown(node))
            }
            "field_initializer" => {
                <FieldInitializer as ::treesitter_types::FromNode>::from_node(node, src)
                    .map(Self::FieldInitializer)
                    .unwrap_or(Self::Unknown(node))
            }
            "field_initializer_list" => {
                <FieldInitializerList as ::treesitter_types::FromNode>::from_node(node, src)
                    .map(Self::FieldInitializerList)
                    .unwrap_or(Self::Unknown(node))
            }
            "fold_expression" => {
                <FoldExpression as ::treesitter_types::FromNode>::from_node(node, src)
                    .map(Self::FoldExpression)
                    .unwrap_or(Self::Unknown(node))
            }
            "for_range_loop" => {
                <ForRangeLoop as ::treesitter_types::FromNode>::from_node(node, src)
                    .map(Self::ForRangeLoop)
                    .unwrap_or(Self::Unknown(node))
            }
            "for_statement" => <ForStatement as ::treesitter_types::FromNode>::from_node(node, src)
                .map(Self::ForStatement)
                .unwrap_or(Self::Unknown(node)),
            "friend_declaration" => {
                <FriendDeclaration as ::treesitter_types::FromNode>::from_node(node, src)
                    .map(Self::FriendDeclaration)
                    .unwrap_or(Self::Unknown(node))
            }
            "function_declarator" => {
                <FunctionDeclarator as ::treesitter_types::FromNode>::from_node(node, src)
                    .map(Self::FunctionDeclarator)
                    .unwrap_or(Self::Unknown(node))
            }
            "function_definition" => {
                <FunctionDefinition as ::treesitter_types::FromNode>::from_node(node, src)
                    .map(Self::FunctionDefinition)
                    .unwrap_or(Self::Unknown(node))
            }
            "generic_expression" => {
                <GenericExpression as ::treesitter_types::FromNode>::from_node(node, src)
                    .map(Self::GenericExpression)
                    .unwrap_or(Self::Unknown(node))
            }
            "global_module_fragment_declaration" => {
                <GlobalModuleFragmentDeclaration as ::treesitter_types::FromNode>::from_node(
                    node, src,
                )
                .map(Self::GlobalModuleFragmentDeclaration)
                .unwrap_or(Self::Unknown(node))
            }
            "gnu_asm_clobber_list" => {
                <GnuAsmClobberList as ::treesitter_types::FromNode>::from_node(node, src)
                    .map(Self::GnuAsmClobberList)
                    .unwrap_or(Self::Unknown(node))
            }
            "gnu_asm_expression" => {
                <GnuAsmExpression as ::treesitter_types::FromNode>::from_node(node, src)
                    .map(Self::GnuAsmExpression)
                    .unwrap_or(Self::Unknown(node))
            }
            "gnu_asm_goto_list" => {
                <GnuAsmGotoList as ::treesitter_types::FromNode>::from_node(node, src)
                    .map(Self::GnuAsmGotoList)
                    .unwrap_or(Self::Unknown(node))
            }
            "gnu_asm_input_operand" => {
                <GnuAsmInputOperand as ::treesitter_types::FromNode>::from_node(node, src)
                    .map(Self::GnuAsmInputOperand)
                    .unwrap_or(Self::Unknown(node))
            }
            "gnu_asm_input_operand_list" => {
                <GnuAsmInputOperandList as ::treesitter_types::FromNode>::from_node(node, src)
                    .map(Self::GnuAsmInputOperandList)
                    .unwrap_or(Self::Unknown(node))
            }
            "gnu_asm_output_operand" => {
                <GnuAsmOutputOperand as ::treesitter_types::FromNode>::from_node(node, src)
                    .map(Self::GnuAsmOutputOperand)
                    .unwrap_or(Self::Unknown(node))
            }
            "gnu_asm_output_operand_list" => {
                <GnuAsmOutputOperandList as ::treesitter_types::FromNode>::from_node(node, src)
                    .map(Self::GnuAsmOutputOperandList)
                    .unwrap_or(Self::Unknown(node))
            }
            "gnu_asm_qualifier" => {
                <GnuAsmQualifier as ::treesitter_types::FromNode>::from_node(node, src)
                    .map(Self::GnuAsmQualifier)
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
            "import_declaration" => {
                <ImportDeclaration as ::treesitter_types::FromNode>::from_node(node, src)
                    .map(Self::ImportDeclaration)
                    .unwrap_or(Self::Unknown(node))
            }
            "init_declarator" => {
                <InitDeclarator as ::treesitter_types::FromNode>::from_node(node, src)
                    .map(Self::InitDeclarator)
                    .unwrap_or(Self::Unknown(node))
            }
            "init_statement" => {
                <InitStatement as ::treesitter_types::FromNode>::from_node(node, src)
                    .map(Self::InitStatement)
                    .unwrap_or(Self::Unknown(node))
            }
            "initializer_list" => {
                <InitializerList as ::treesitter_types::FromNode>::from_node(node, src)
                    .map(Self::InitializerList)
                    .unwrap_or(Self::Unknown(node))
            }
            "initializer_pair" => {
                <InitializerPair as ::treesitter_types::FromNode>::from_node(node, src)
                    .map(Self::InitializerPair)
                    .unwrap_or(Self::Unknown(node))
            }
            "labeled_statement" => {
                <LabeledStatement as ::treesitter_types::FromNode>::from_node(node, src)
                    .map(Self::LabeledStatement)
                    .unwrap_or(Self::Unknown(node))
            }
            "lambda_capture_initializer" => {
                <LambdaCaptureInitializer as ::treesitter_types::FromNode>::from_node(node, src)
                    .map(Self::LambdaCaptureInitializer)
                    .unwrap_or(Self::Unknown(node))
            }
            "lambda_capture_specifier" => {
                <LambdaCaptureSpecifier as ::treesitter_types::FromNode>::from_node(node, src)
                    .map(Self::LambdaCaptureSpecifier)
                    .unwrap_or(Self::Unknown(node))
            }
            "lambda_declarator" => {
                <LambdaDeclarator as ::treesitter_types::FromNode>::from_node(node, src)
                    .map(Self::LambdaDeclarator)
                    .unwrap_or(Self::Unknown(node))
            }
            "lambda_default_capture" => {
                <LambdaDefaultCapture as ::treesitter_types::FromNode>::from_node(node, src)
                    .map(Self::LambdaDefaultCapture)
                    .unwrap_or(Self::Unknown(node))
            }
            "lambda_expression" => {
                <LambdaExpression as ::treesitter_types::FromNode>::from_node(node, src)
                    .map(Self::LambdaExpression)
                    .unwrap_or(Self::Unknown(node))
            }
            "lambda_specifier" => {
                <LambdaSpecifier as ::treesitter_types::FromNode>::from_node(node, src)
                    .map(Self::LambdaSpecifier)
                    .unwrap_or(Self::Unknown(node))
            }
            "linkage_specification" => {
                <LinkageSpecification as ::treesitter_types::FromNode>::from_node(node, src)
                    .map(Self::LinkageSpecification)
                    .unwrap_or(Self::Unknown(node))
            }
            "module_declaration" => {
                <ModuleDeclaration as ::treesitter_types::FromNode>::from_node(node, src)
                    .map(Self::ModuleDeclaration)
                    .unwrap_or(Self::Unknown(node))
            }
            "module_name" => <ModuleName as ::treesitter_types::FromNode>::from_node(node, src)
                .map(Self::ModuleName)
                .unwrap_or(Self::Unknown(node)),
            "module_partition" => {
                <ModulePartition as ::treesitter_types::FromNode>::from_node(node, src)
                    .map(Self::ModulePartition)
                    .unwrap_or(Self::Unknown(node))
            }
            "ms_based_modifier" => {
                <MsBasedModifier as ::treesitter_types::FromNode>::from_node(node, src)
                    .map(Self::MsBasedModifier)
                    .unwrap_or(Self::Unknown(node))
            }
            "ms_call_modifier" => {
                <MsCallModifier as ::treesitter_types::FromNode>::from_node(node, src)
                    .map(Self::MsCallModifier)
                    .unwrap_or(Self::Unknown(node))
            }
            "ms_declspec_modifier" => {
                <MsDeclspecModifier as ::treesitter_types::FromNode>::from_node(node, src)
                    .map(Self::MsDeclspecModifier)
                    .unwrap_or(Self::Unknown(node))
            }
            "ms_pointer_modifier" => {
                <MsPointerModifier as ::treesitter_types::FromNode>::from_node(node, src)
                    .map(Self::MsPointerModifier)
                    .unwrap_or(Self::Unknown(node))
            }
            "ms_unaligned_ptr_modifier" => {
                <MsUnalignedPtrModifier as ::treesitter_types::FromNode>::from_node(node, src)
                    .map(Self::MsUnalignedPtrModifier)
                    .unwrap_or(Self::Unknown(node))
            }
            "namespace_alias_definition" => {
                <NamespaceAliasDefinition as ::treesitter_types::FromNode>::from_node(node, src)
                    .map(Self::NamespaceAliasDefinition)
                    .unwrap_or(Self::Unknown(node))
            }
            "namespace_definition" => {
                <NamespaceDefinition as ::treesitter_types::FromNode>::from_node(node, src)
                    .map(Self::NamespaceDefinition)
                    .unwrap_or(Self::Unknown(node))
            }
            "nested_namespace_specifier" => {
                <NestedNamespaceSpecifier as ::treesitter_types::FromNode>::from_node(node, src)
                    .map(Self::NestedNamespaceSpecifier)
                    .unwrap_or(Self::Unknown(node))
            }
            "new_declarator" => {
                <NewDeclarator as ::treesitter_types::FromNode>::from_node(node, src)
                    .map(Self::NewDeclarator)
                    .unwrap_or(Self::Unknown(node))
            }
            "new_expression" => {
                <NewExpression as ::treesitter_types::FromNode>::from_node(node, src)
                    .map(Self::NewExpression)
                    .unwrap_or(Self::Unknown(node))
            }
            "noexcept" => <Noexcept as ::treesitter_types::FromNode>::from_node(node, src)
                .map(Self::Noexcept)
                .unwrap_or(Self::Unknown(node)),
            "null" => <Null as ::treesitter_types::FromNode>::from_node(node, src)
                .map(Self::Null)
                .unwrap_or(Self::Unknown(node)),
            "offsetof_expression" => {
                <OffsetofExpression as ::treesitter_types::FromNode>::from_node(node, src)
                    .map(Self::OffsetofExpression)
                    .unwrap_or(Self::Unknown(node))
            }
            "operator_cast" => <OperatorCast as ::treesitter_types::FromNode>::from_node(node, src)
                .map(Self::OperatorCast)
                .unwrap_or(Self::Unknown(node)),
            "operator_name" => <OperatorName as ::treesitter_types::FromNode>::from_node(node, src)
                .map(Self::OperatorName)
                .unwrap_or(Self::Unknown(node)),
            "optional_parameter_declaration" => {
                <OptionalParameterDeclaration as ::treesitter_types::FromNode>::from_node(node, src)
                    .map(Self::OptionalParameterDeclaration)
                    .unwrap_or(Self::Unknown(node))
            }
            "optional_type_parameter_declaration" => {
                <OptionalTypeParameterDeclaration as ::treesitter_types::FromNode>::from_node(
                    node, src,
                )
                .map(Self::OptionalTypeParameterDeclaration)
                .unwrap_or(Self::Unknown(node))
            }
            "parameter_declaration" => {
                <ParameterDeclaration as ::treesitter_types::FromNode>::from_node(node, src)
                    .map(Self::ParameterDeclaration)
                    .unwrap_or(Self::Unknown(node))
            }
            "parameter_list" => {
                <ParameterList as ::treesitter_types::FromNode>::from_node(node, src)
                    .map(Self::ParameterList)
                    .unwrap_or(Self::Unknown(node))
            }
            "parameter_pack_expansion" => {
                <ParameterPackExpansion as ::treesitter_types::FromNode>::from_node(node, src)
                    .map(Self::ParameterPackExpansion)
                    .unwrap_or(Self::Unknown(node))
            }
            "parenthesized_declarator" => {
                <ParenthesizedDeclarator as ::treesitter_types::FromNode>::from_node(node, src)
                    .map(Self::ParenthesizedDeclarator)
                    .unwrap_or(Self::Unknown(node))
            }
            "parenthesized_expression" => {
                <ParenthesizedExpression as ::treesitter_types::FromNode>::from_node(node, src)
                    .map(Self::ParenthesizedExpression)
                    .unwrap_or(Self::Unknown(node))
            }
            "placeholder_type_specifier" => {
                <PlaceholderTypeSpecifier as ::treesitter_types::FromNode>::from_node(node, src)
                    .map(Self::PlaceholderTypeSpecifier)
                    .unwrap_or(Self::Unknown(node))
            }
            "pointer_declarator" => {
                <PointerDeclarator as ::treesitter_types::FromNode>::from_node(node, src)
                    .map(Self::PointerDeclarator)
                    .unwrap_or(Self::Unknown(node))
            }
            "pointer_expression" => {
                <PointerExpression as ::treesitter_types::FromNode>::from_node(node, src)
                    .map(Self::PointerExpression)
                    .unwrap_or(Self::Unknown(node))
            }
            "pointer_type_declarator" => {
                <PointerTypeDeclarator as ::treesitter_types::FromNode>::from_node(node, src)
                    .map(Self::PointerTypeDeclarator)
                    .unwrap_or(Self::Unknown(node))
            }
            "preproc_call" => <PreprocCall as ::treesitter_types::FromNode>::from_node(node, src)
                .map(Self::PreprocCall)
                .unwrap_or(Self::Unknown(node)),
            "preproc_def" => <PreprocDef as ::treesitter_types::FromNode>::from_node(node, src)
                .map(Self::PreprocDef)
                .unwrap_or(Self::Unknown(node)),
            "preproc_defined" => {
                <PreprocDefined as ::treesitter_types::FromNode>::from_node(node, src)
                    .map(Self::PreprocDefined)
                    .unwrap_or(Self::Unknown(node))
            }
            "preproc_elif" => <PreprocElif as ::treesitter_types::FromNode>::from_node(node, src)
                .map(Self::PreprocElif)
                .unwrap_or(Self::Unknown(node)),
            "preproc_elifdef" => {
                <PreprocElifdef as ::treesitter_types::FromNode>::from_node(node, src)
                    .map(Self::PreprocElifdef)
                    .unwrap_or(Self::Unknown(node))
            }
            "preproc_else" => <PreprocElse as ::treesitter_types::FromNode>::from_node(node, src)
                .map(Self::PreprocElse)
                .unwrap_or(Self::Unknown(node)),
            "preproc_function_def" => {
                <PreprocFunctionDef as ::treesitter_types::FromNode>::from_node(node, src)
                    .map(Self::PreprocFunctionDef)
                    .unwrap_or(Self::Unknown(node))
            }
            "preproc_if" => <PreprocIf as ::treesitter_types::FromNode>::from_node(node, src)
                .map(Self::PreprocIf)
                .unwrap_or(Self::Unknown(node)),
            "preproc_ifdef" => <PreprocIfdef as ::treesitter_types::FromNode>::from_node(node, src)
                .map(Self::PreprocIfdef)
                .unwrap_or(Self::Unknown(node)),
            "preproc_include" => {
                <PreprocInclude as ::treesitter_types::FromNode>::from_node(node, src)
                    .map(Self::PreprocInclude)
                    .unwrap_or(Self::Unknown(node))
            }
            "preproc_params" => {
                <PreprocParams as ::treesitter_types::FromNode>::from_node(node, src)
                    .map(Self::PreprocParams)
                    .unwrap_or(Self::Unknown(node))
            }
            "private_module_fragment_declaration" => {
                <PrivateModuleFragmentDeclaration as ::treesitter_types::FromNode>::from_node(
                    node, src,
                )
                .map(Self::PrivateModuleFragmentDeclaration)
                .unwrap_or(Self::Unknown(node))
            }
            "pure_virtual_clause" => {
                <PureVirtualClause as ::treesitter_types::FromNode>::from_node(node, src)
                    .map(Self::PureVirtualClause)
                    .unwrap_or(Self::Unknown(node))
            }
            "qualified_identifier" => {
                <QualifiedIdentifier as ::treesitter_types::FromNode>::from_node(node, src)
                    .map(Self::QualifiedIdentifier)
                    .unwrap_or(Self::Unknown(node))
            }
            "raw_string_literal" => {
                <RawStringLiteral as ::treesitter_types::FromNode>::from_node(node, src)
                    .map(Self::RawStringLiteral)
                    .unwrap_or(Self::Unknown(node))
            }
            "ref_qualifier" => <RefQualifier as ::treesitter_types::FromNode>::from_node(node, src)
                .map(Self::RefQualifier)
                .unwrap_or(Self::Unknown(node)),
            "reference_declarator" => {
                <ReferenceDeclarator as ::treesitter_types::FromNode>::from_node(node, src)
                    .map(Self::ReferenceDeclarator)
                    .unwrap_or(Self::Unknown(node))
            }
            "reflect_expression" => {
                <ReflectExpression as ::treesitter_types::FromNode>::from_node(node, src)
                    .map(Self::ReflectExpression)
                    .unwrap_or(Self::Unknown(node))
            }
            "requirement_seq" => {
                <RequirementSeq as ::treesitter_types::FromNode>::from_node(node, src)
                    .map(Self::RequirementSeq)
                    .unwrap_or(Self::Unknown(node))
            }
            "requires_clause" => {
                <RequiresClause as ::treesitter_types::FromNode>::from_node(node, src)
                    .map(Self::RequiresClause)
                    .unwrap_or(Self::Unknown(node))
            }
            "requires_expression" => {
                <RequiresExpression as ::treesitter_types::FromNode>::from_node(node, src)
                    .map(Self::RequiresExpression)
                    .unwrap_or(Self::Unknown(node))
            }
            "return_statement" => {
                <ReturnStatement as ::treesitter_types::FromNode>::from_node(node, src)
                    .map(Self::ReturnStatement)
                    .unwrap_or(Self::Unknown(node))
            }
            "seh_except_clause" => {
                <SehExceptClause as ::treesitter_types::FromNode>::from_node(node, src)
                    .map(Self::SehExceptClause)
                    .unwrap_or(Self::Unknown(node))
            }
            "seh_finally_clause" => {
                <SehFinallyClause as ::treesitter_types::FromNode>::from_node(node, src)
                    .map(Self::SehFinallyClause)
                    .unwrap_or(Self::Unknown(node))
            }
            "seh_leave_statement" => {
                <SehLeaveStatement as ::treesitter_types::FromNode>::from_node(node, src)
                    .map(Self::SehLeaveStatement)
                    .unwrap_or(Self::Unknown(node))
            }
            "seh_try_statement" => {
                <SehTryStatement as ::treesitter_types::FromNode>::from_node(node, src)
                    .map(Self::SehTryStatement)
                    .unwrap_or(Self::Unknown(node))
            }
            "simple_requirement" => {
                <SimpleRequirement as ::treesitter_types::FromNode>::from_node(node, src)
                    .map(Self::SimpleRequirement)
                    .unwrap_or(Self::Unknown(node))
            }
            "sized_type_specifier" => {
                <SizedTypeSpecifier as ::treesitter_types::FromNode>::from_node(node, src)
                    .map(Self::SizedTypeSpecifier)
                    .unwrap_or(Self::Unknown(node))
            }
            "sizeof_expression" => {
                <SizeofExpression as ::treesitter_types::FromNode>::from_node(node, src)
                    .map(Self::SizeofExpression)
                    .unwrap_or(Self::Unknown(node))
            }
            "splice_expression" => {
                <SpliceExpression as ::treesitter_types::FromNode>::from_node(node, src)
                    .map(Self::SpliceExpression)
                    .unwrap_or(Self::Unknown(node))
            }
            "splice_specifier" => {
                <SpliceSpecifier as ::treesitter_types::FromNode>::from_node(node, src)
                    .map(Self::SpliceSpecifier)
                    .unwrap_or(Self::Unknown(node))
            }
            "splice_type_specifier" => {
                <SpliceTypeSpecifier as ::treesitter_types::FromNode>::from_node(node, src)
                    .map(Self::SpliceTypeSpecifier)
                    .unwrap_or(Self::Unknown(node))
            }
            "static_assert_declaration" => {
                <StaticAssertDeclaration as ::treesitter_types::FromNode>::from_node(node, src)
                    .map(Self::StaticAssertDeclaration)
                    .unwrap_or(Self::Unknown(node))
            }
            "storage_class_specifier" => {
                <StorageClassSpecifier as ::treesitter_types::FromNode>::from_node(node, src)
                    .map(Self::StorageClassSpecifier)
                    .unwrap_or(Self::Unknown(node))
            }
            "string_literal" => {
                <StringLiteral as ::treesitter_types::FromNode>::from_node(node, src)
                    .map(Self::StringLiteral)
                    .unwrap_or(Self::Unknown(node))
            }
            "struct_specifier" => {
                <StructSpecifier as ::treesitter_types::FromNode>::from_node(node, src)
                    .map(Self::StructSpecifier)
                    .unwrap_or(Self::Unknown(node))
            }
            "structured_binding_declarator" => {
                <StructuredBindingDeclarator as ::treesitter_types::FromNode>::from_node(node, src)
                    .map(Self::StructuredBindingDeclarator)
                    .unwrap_or(Self::Unknown(node))
            }
            "subscript_argument_list" => {
                <SubscriptArgumentList as ::treesitter_types::FromNode>::from_node(node, src)
                    .map(Self::SubscriptArgumentList)
                    .unwrap_or(Self::Unknown(node))
            }
            "subscript_designator" => {
                <SubscriptDesignator as ::treesitter_types::FromNode>::from_node(node, src)
                    .map(Self::SubscriptDesignator)
                    .unwrap_or(Self::Unknown(node))
            }
            "subscript_expression" => {
                <SubscriptExpression as ::treesitter_types::FromNode>::from_node(node, src)
                    .map(Self::SubscriptExpression)
                    .unwrap_or(Self::Unknown(node))
            }
            "subscript_range_designator" => {
                <SubscriptRangeDesignator as ::treesitter_types::FromNode>::from_node(node, src)
                    .map(Self::SubscriptRangeDesignator)
                    .unwrap_or(Self::Unknown(node))
            }
            "switch_statement" => {
                <SwitchStatement as ::treesitter_types::FromNode>::from_node(node, src)
                    .map(Self::SwitchStatement)
                    .unwrap_or(Self::Unknown(node))
            }
            "template_argument_list" => {
                <TemplateArgumentList as ::treesitter_types::FromNode>::from_node(node, src)
                    .map(Self::TemplateArgumentList)
                    .unwrap_or(Self::Unknown(node))
            }
            "template_declaration" => {
                <TemplateDeclaration as ::treesitter_types::FromNode>::from_node(node, src)
                    .map(Self::TemplateDeclaration)
                    .unwrap_or(Self::Unknown(node))
            }
            "template_function" => {
                <TemplateFunction as ::treesitter_types::FromNode>::from_node(node, src)
                    .map(Self::TemplateFunction)
                    .unwrap_or(Self::Unknown(node))
            }
            "template_instantiation" => {
                <TemplateInstantiation as ::treesitter_types::FromNode>::from_node(node, src)
                    .map(Self::TemplateInstantiation)
                    .unwrap_or(Self::Unknown(node))
            }
            "template_method" => {
                <TemplateMethod as ::treesitter_types::FromNode>::from_node(node, src)
                    .map(Self::TemplateMethod)
                    .unwrap_or(Self::Unknown(node))
            }
            "template_parameter_list" => {
                <TemplateParameterList as ::treesitter_types::FromNode>::from_node(node, src)
                    .map(Self::TemplateParameterList)
                    .unwrap_or(Self::Unknown(node))
            }
            "template_template_parameter_declaration" => {
                <TemplateTemplateParameterDeclaration as ::treesitter_types::FromNode>::from_node(
                    node, src,
                )
                .map(Self::TemplateTemplateParameterDeclaration)
                .unwrap_or(Self::Unknown(node))
            }
            "template_type" => <TemplateType as ::treesitter_types::FromNode>::from_node(node, src)
                .map(Self::TemplateType)
                .unwrap_or(Self::Unknown(node)),
            "throw_specifier" => {
                <ThrowSpecifier as ::treesitter_types::FromNode>::from_node(node, src)
                    .map(Self::ThrowSpecifier)
                    .unwrap_or(Self::Unknown(node))
            }
            "throw_statement" => {
                <ThrowStatement as ::treesitter_types::FromNode>::from_node(node, src)
                    .map(Self::ThrowStatement)
                    .unwrap_or(Self::Unknown(node))
            }
            "trailing_return_type" => {
                <TrailingReturnType as ::treesitter_types::FromNode>::from_node(node, src)
                    .map(Self::TrailingReturnType)
                    .unwrap_or(Self::Unknown(node))
            }
            "translation_unit" => {
                <TranslationUnit as ::treesitter_types::FromNode>::from_node(node, src)
                    .map(Self::TranslationUnit)
                    .unwrap_or(Self::Unknown(node))
            }
            "try_statement" => <TryStatement as ::treesitter_types::FromNode>::from_node(node, src)
                .map(Self::TryStatement)
                .unwrap_or(Self::Unknown(node)),
            "type_definition" => {
                <TypeDefinition as ::treesitter_types::FromNode>::from_node(node, src)
                    .map(Self::TypeDefinition)
                    .unwrap_or(Self::Unknown(node))
            }
            "type_descriptor" => {
                <TypeDescriptor as ::treesitter_types::FromNode>::from_node(node, src)
                    .map(Self::TypeDescriptor)
                    .unwrap_or(Self::Unknown(node))
            }
            "type_parameter_declaration" => {
                <TypeParameterDeclaration as ::treesitter_types::FromNode>::from_node(node, src)
                    .map(Self::TypeParameterDeclaration)
                    .unwrap_or(Self::Unknown(node))
            }
            "type_qualifier" => {
                <TypeQualifier as ::treesitter_types::FromNode>::from_node(node, src)
                    .map(Self::TypeQualifier)
                    .unwrap_or(Self::Unknown(node))
            }
            "type_requirement" => {
                <TypeRequirement as ::treesitter_types::FromNode>::from_node(node, src)
                    .map(Self::TypeRequirement)
                    .unwrap_or(Self::Unknown(node))
            }
            "unary_expression" => {
                <UnaryExpression as ::treesitter_types::FromNode>::from_node(node, src)
                    .map(Self::UnaryExpression)
                    .unwrap_or(Self::Unknown(node))
            }
            "union_specifier" => {
                <UnionSpecifier as ::treesitter_types::FromNode>::from_node(node, src)
                    .map(Self::UnionSpecifier)
                    .unwrap_or(Self::Unknown(node))
            }
            "update_expression" => {
                <UpdateExpression as ::treesitter_types::FromNode>::from_node(node, src)
                    .map(Self::UpdateExpression)
                    .unwrap_or(Self::Unknown(node))
            }
            "user_defined_literal" => {
                <UserDefinedLiteral as ::treesitter_types::FromNode>::from_node(node, src)
                    .map(Self::UserDefinedLiteral)
                    .unwrap_or(Self::Unknown(node))
            }
            "using_declaration" => {
                <UsingDeclaration as ::treesitter_types::FromNode>::from_node(node, src)
                    .map(Self::UsingDeclaration)
                    .unwrap_or(Self::Unknown(node))
            }
            "variadic_declarator" => {
                <VariadicDeclarator as ::treesitter_types::FromNode>::from_node(node, src)
                    .map(Self::VariadicDeclarator)
                    .unwrap_or(Self::Unknown(node))
            }
            "variadic_parameter_declaration" => {
                <VariadicParameterDeclaration as ::treesitter_types::FromNode>::from_node(node, src)
                    .map(Self::VariadicParameterDeclaration)
                    .unwrap_or(Self::Unknown(node))
            }
            "variadic_type_parameter_declaration" => {
                <VariadicTypeParameterDeclaration as ::treesitter_types::FromNode>::from_node(
                    node, src,
                )
                .map(Self::VariadicTypeParameterDeclaration)
                .unwrap_or(Self::Unknown(node))
            }
            "virtual_specifier" => {
                <VirtualSpecifier as ::treesitter_types::FromNode>::from_node(node, src)
                    .map(Self::VirtualSpecifier)
                    .unwrap_or(Self::Unknown(node))
            }
            "while_statement" => {
                <WhileStatement as ::treesitter_types::FromNode>::from_node(node, src)
                    .map(Self::WhileStatement)
                    .unwrap_or(Self::Unknown(node))
            }
            "auto" => <Auto as ::treesitter_types::FromNode>::from_node(node, src)
                .map(Self::Auto)
                .unwrap_or(Self::Unknown(node)),
            "character" => <Character as ::treesitter_types::FromNode>::from_node(node, src)
                .map(Self::Character)
                .unwrap_or(Self::Unknown(node)),
            "comment" => <Comment as ::treesitter_types::FromNode>::from_node(node, src)
                .map(Self::Comment)
                .unwrap_or(Self::Unknown(node)),
            "escape_sequence" => {
                <EscapeSequence as ::treesitter_types::FromNode>::from_node(node, src)
                    .map(Self::EscapeSequence)
                    .unwrap_or(Self::Unknown(node))
            }
            "false" => <False as ::treesitter_types::FromNode>::from_node(node, src)
                .map(Self::False)
                .unwrap_or(Self::Unknown(node)),
            "field_identifier" => {
                <FieldIdentifier as ::treesitter_types::FromNode>::from_node(node, src)
                    .map(Self::FieldIdentifier)
                    .unwrap_or(Self::Unknown(node))
            }
            "identifier" => <Identifier as ::treesitter_types::FromNode>::from_node(node, src)
                .map(Self::Identifier)
                .unwrap_or(Self::Unknown(node)),
            "literal_suffix" => {
                <LiteralSuffix as ::treesitter_types::FromNode>::from_node(node, src)
                    .map(Self::LiteralSuffix)
                    .unwrap_or(Self::Unknown(node))
            }
            "ms_restrict_modifier" => {
                <MsRestrictModifier as ::treesitter_types::FromNode>::from_node(node, src)
                    .map(Self::MsRestrictModifier)
                    .unwrap_or(Self::Unknown(node))
            }
            "ms_signed_ptr_modifier" => {
                <MsSignedPtrModifier as ::treesitter_types::FromNode>::from_node(node, src)
                    .map(Self::MsSignedPtrModifier)
                    .unwrap_or(Self::Unknown(node))
            }
            "ms_unsigned_ptr_modifier" => {
                <MsUnsignedPtrModifier as ::treesitter_types::FromNode>::from_node(node, src)
                    .map(Self::MsUnsignedPtrModifier)
                    .unwrap_or(Self::Unknown(node))
            }
            "namespace_identifier" => {
                <NamespaceIdentifier as ::treesitter_types::FromNode>::from_node(node, src)
                    .map(Self::NamespaceIdentifier)
                    .unwrap_or(Self::Unknown(node))
            }
            "number_literal" => {
                <NumberLiteral as ::treesitter_types::FromNode>::from_node(node, src)
                    .map(Self::NumberLiteral)
                    .unwrap_or(Self::Unknown(node))
            }
            "preproc_arg" => <PreprocArg as ::treesitter_types::FromNode>::from_node(node, src)
                .map(Self::PreprocArg)
                .unwrap_or(Self::Unknown(node)),
            "preproc_directive" => {
                <PreprocDirective as ::treesitter_types::FromNode>::from_node(node, src)
                    .map(Self::PreprocDirective)
                    .unwrap_or(Self::Unknown(node))
            }
            "primitive_type" => {
                <PrimitiveType as ::treesitter_types::FromNode>::from_node(node, src)
                    .map(Self::PrimitiveType)
                    .unwrap_or(Self::Unknown(node))
            }
            "raw_string_content" => {
                <RawStringContent as ::treesitter_types::FromNode>::from_node(node, src)
                    .map(Self::RawStringContent)
                    .unwrap_or(Self::Unknown(node))
            }
            "raw_string_delimiter" => {
                <RawStringDelimiter as ::treesitter_types::FromNode>::from_node(node, src)
                    .map(Self::RawStringDelimiter)
                    .unwrap_or(Self::Unknown(node))
            }
            "statement_identifier" => {
                <StatementIdentifier as ::treesitter_types::FromNode>::from_node(node, src)
                    .map(Self::StatementIdentifier)
                    .unwrap_or(Self::Unknown(node))
            }
            "string_content" => {
                <StringContent as ::treesitter_types::FromNode>::from_node(node, src)
                    .map(Self::StringContent)
                    .unwrap_or(Self::Unknown(node))
            }
            "system_lib_string" => {
                <SystemLibString as ::treesitter_types::FromNode>::from_node(node, src)
                    .map(Self::SystemLibString)
                    .unwrap_or(Self::Unknown(node))
            }
            "this" => <This as ::treesitter_types::FromNode>::from_node(node, src)
                .map(Self::This)
                .unwrap_or(Self::Unknown(node)),
            "true" => <True as ::treesitter_types::FromNode>::from_node(node, src)
                .map(Self::True)
                .unwrap_or(Self::Unknown(node)),
            "type_identifier" => {
                <TypeIdentifier as ::treesitter_types::FromNode>::from_node(node, src)
                    .map(Self::TypeIdentifier)
                    .unwrap_or(Self::Unknown(node))
            }
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
            Self::AbstractReferenceDeclarator(inner) => inner.span(),
            Self::AccessSpecifier(inner) => inner.span(),
            Self::AliasDeclaration(inner) => inner.span(),
            Self::AlignasQualifier(inner) => inner.span(),
            Self::AlignofExpression(inner) => inner.span(),
            Self::Annotation(inner) => inner.span(),
            Self::ArgumentList(inner) => inner.span(),
            Self::ArrayDeclarator(inner) => inner.span(),
            Self::AssignmentExpression(inner) => inner.span(),
            Self::Attribute(inner) => inner.span(),
            Self::AttributeDeclaration(inner) => inner.span(),
            Self::AttributeSpecifier(inner) => inner.span(),
            Self::AttributedDeclarator(inner) => inner.span(),
            Self::AttributedStatement(inner) => inner.span(),
            Self::BaseClassClause(inner) => inner.span(),
            Self::BinaryExpression(inner) => inner.span(),
            Self::BitfieldClause(inner) => inner.span(),
            Self::BreakStatement(inner) => inner.span(),
            Self::CallExpression(inner) => inner.span(),
            Self::CaseStatement(inner) => inner.span(),
            Self::CastExpression(inner) => inner.span(),
            Self::CatchClause(inner) => inner.span(),
            Self::CharLiteral(inner) => inner.span(),
            Self::ClassSpecifier(inner) => inner.span(),
            Self::CoAwaitExpression(inner) => inner.span(),
            Self::CoReturnStatement(inner) => inner.span(),
            Self::CoYieldStatement(inner) => inner.span(),
            Self::CommaExpression(inner) => inner.span(),
            Self::CompoundLiteralExpression(inner) => inner.span(),
            Self::CompoundRequirement(inner) => inner.span(),
            Self::CompoundStatement(inner) => inner.span(),
            Self::ConcatenatedString(inner) => inner.span(),
            Self::ConceptDefinition(inner) => inner.span(),
            Self::ConditionClause(inner) => inner.span(),
            Self::ConditionalExpression(inner) => inner.span(),
            Self::ConstevalBlockDeclaration(inner) => inner.span(),
            Self::ConstraintConjunction(inner) => inner.span(),
            Self::ConstraintDisjunction(inner) => inner.span(),
            Self::ContinueStatement(inner) => inner.span(),
            Self::Declaration(inner) => inner.span(),
            Self::DeclarationList(inner) => inner.span(),
            Self::Decltype(inner) => inner.span(),
            Self::DefaultMethodClause(inner) => inner.span(),
            Self::DeleteExpression(inner) => inner.span(),
            Self::DeleteMethodClause(inner) => inner.span(),
            Self::DependentName(inner) => inner.span(),
            Self::DependentType(inner) => inner.span(),
            Self::DestructorName(inner) => inner.span(),
            Self::DoStatement(inner) => inner.span(),
            Self::ElseClause(inner) => inner.span(),
            Self::EnumSpecifier(inner) => inner.span(),
            Self::Enumerator(inner) => inner.span(),
            Self::EnumeratorList(inner) => inner.span(),
            Self::ExpansionStatement(inner) => inner.span(),
            Self::ExplicitFunctionSpecifier(inner) => inner.span(),
            Self::ExplicitObjectParameterDeclaration(inner) => inner.span(),
            Self::ExportDeclaration(inner) => inner.span(),
            Self::ExpressionStatement(inner) => inner.span(),
            Self::ExtensionExpression(inner) => inner.span(),
            Self::FieldDeclaration(inner) => inner.span(),
            Self::FieldDeclarationList(inner) => inner.span(),
            Self::FieldDesignator(inner) => inner.span(),
            Self::FieldExpression(inner) => inner.span(),
            Self::FieldInitializer(inner) => inner.span(),
            Self::FieldInitializerList(inner) => inner.span(),
            Self::FoldExpression(inner) => inner.span(),
            Self::ForRangeLoop(inner) => inner.span(),
            Self::ForStatement(inner) => inner.span(),
            Self::FriendDeclaration(inner) => inner.span(),
            Self::FunctionDeclarator(inner) => inner.span(),
            Self::FunctionDefinition(inner) => inner.span(),
            Self::GenericExpression(inner) => inner.span(),
            Self::GlobalModuleFragmentDeclaration(inner) => inner.span(),
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
            Self::ImportDeclaration(inner) => inner.span(),
            Self::InitDeclarator(inner) => inner.span(),
            Self::InitStatement(inner) => inner.span(),
            Self::InitializerList(inner) => inner.span(),
            Self::InitializerPair(inner) => inner.span(),
            Self::LabeledStatement(inner) => inner.span(),
            Self::LambdaCaptureInitializer(inner) => inner.span(),
            Self::LambdaCaptureSpecifier(inner) => inner.span(),
            Self::LambdaDeclarator(inner) => inner.span(),
            Self::LambdaDefaultCapture(inner) => inner.span(),
            Self::LambdaExpression(inner) => inner.span(),
            Self::LambdaSpecifier(inner) => inner.span(),
            Self::LinkageSpecification(inner) => inner.span(),
            Self::ModuleDeclaration(inner) => inner.span(),
            Self::ModuleName(inner) => inner.span(),
            Self::ModulePartition(inner) => inner.span(),
            Self::MsBasedModifier(inner) => inner.span(),
            Self::MsCallModifier(inner) => inner.span(),
            Self::MsDeclspecModifier(inner) => inner.span(),
            Self::MsPointerModifier(inner) => inner.span(),
            Self::MsUnalignedPtrModifier(inner) => inner.span(),
            Self::NamespaceAliasDefinition(inner) => inner.span(),
            Self::NamespaceDefinition(inner) => inner.span(),
            Self::NestedNamespaceSpecifier(inner) => inner.span(),
            Self::NewDeclarator(inner) => inner.span(),
            Self::NewExpression(inner) => inner.span(),
            Self::Noexcept(inner) => inner.span(),
            Self::Null(inner) => inner.span(),
            Self::OffsetofExpression(inner) => inner.span(),
            Self::OperatorCast(inner) => inner.span(),
            Self::OperatorName(inner) => inner.span(),
            Self::OptionalParameterDeclaration(inner) => inner.span(),
            Self::OptionalTypeParameterDeclaration(inner) => inner.span(),
            Self::ParameterDeclaration(inner) => inner.span(),
            Self::ParameterList(inner) => inner.span(),
            Self::ParameterPackExpansion(inner) => inner.span(),
            Self::ParenthesizedDeclarator(inner) => inner.span(),
            Self::ParenthesizedExpression(inner) => inner.span(),
            Self::PlaceholderTypeSpecifier(inner) => inner.span(),
            Self::PointerDeclarator(inner) => inner.span(),
            Self::PointerExpression(inner) => inner.span(),
            Self::PointerTypeDeclarator(inner) => inner.span(),
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
            Self::PrivateModuleFragmentDeclaration(inner) => inner.span(),
            Self::PureVirtualClause(inner) => inner.span(),
            Self::QualifiedIdentifier(inner) => inner.span(),
            Self::RawStringLiteral(inner) => inner.span(),
            Self::RefQualifier(inner) => inner.span(),
            Self::ReferenceDeclarator(inner) => inner.span(),
            Self::ReflectExpression(inner) => inner.span(),
            Self::RequirementSeq(inner) => inner.span(),
            Self::RequiresClause(inner) => inner.span(),
            Self::RequiresExpression(inner) => inner.span(),
            Self::ReturnStatement(inner) => inner.span(),
            Self::SehExceptClause(inner) => inner.span(),
            Self::SehFinallyClause(inner) => inner.span(),
            Self::SehLeaveStatement(inner) => inner.span(),
            Self::SehTryStatement(inner) => inner.span(),
            Self::SimpleRequirement(inner) => inner.span(),
            Self::SizedTypeSpecifier(inner) => inner.span(),
            Self::SizeofExpression(inner) => inner.span(),
            Self::SpliceExpression(inner) => inner.span(),
            Self::SpliceSpecifier(inner) => inner.span(),
            Self::SpliceTypeSpecifier(inner) => inner.span(),
            Self::StaticAssertDeclaration(inner) => inner.span(),
            Self::StorageClassSpecifier(inner) => inner.span(),
            Self::StringLiteral(inner) => inner.span(),
            Self::StructSpecifier(inner) => inner.span(),
            Self::StructuredBindingDeclarator(inner) => inner.span(),
            Self::SubscriptArgumentList(inner) => inner.span(),
            Self::SubscriptDesignator(inner) => inner.span(),
            Self::SubscriptExpression(inner) => inner.span(),
            Self::SubscriptRangeDesignator(inner) => inner.span(),
            Self::SwitchStatement(inner) => inner.span(),
            Self::TemplateArgumentList(inner) => inner.span(),
            Self::TemplateDeclaration(inner) => inner.span(),
            Self::TemplateFunction(inner) => inner.span(),
            Self::TemplateInstantiation(inner) => inner.span(),
            Self::TemplateMethod(inner) => inner.span(),
            Self::TemplateParameterList(inner) => inner.span(),
            Self::TemplateTemplateParameterDeclaration(inner) => inner.span(),
            Self::TemplateType(inner) => inner.span(),
            Self::ThrowSpecifier(inner) => inner.span(),
            Self::ThrowStatement(inner) => inner.span(),
            Self::TrailingReturnType(inner) => inner.span(),
            Self::TranslationUnit(inner) => inner.span(),
            Self::TryStatement(inner) => inner.span(),
            Self::TypeDefinition(inner) => inner.span(),
            Self::TypeDescriptor(inner) => inner.span(),
            Self::TypeParameterDeclaration(inner) => inner.span(),
            Self::TypeQualifier(inner) => inner.span(),
            Self::TypeRequirement(inner) => inner.span(),
            Self::UnaryExpression(inner) => inner.span(),
            Self::UnionSpecifier(inner) => inner.span(),
            Self::UpdateExpression(inner) => inner.span(),
            Self::UserDefinedLiteral(inner) => inner.span(),
            Self::UsingDeclaration(inner) => inner.span(),
            Self::VariadicDeclarator(inner) => inner.span(),
            Self::VariadicParameterDeclaration(inner) => inner.span(),
            Self::VariadicTypeParameterDeclaration(inner) => inner.span(),
            Self::VirtualSpecifier(inner) => inner.span(),
            Self::WhileStatement(inner) => inner.span(),
            Self::Auto(inner) => inner.span(),
            Self::Character(inner) => inner.span(),
            Self::Comment(inner) => inner.span(),
            Self::EscapeSequence(inner) => inner.span(),
            Self::False(inner) => inner.span(),
            Self::FieldIdentifier(inner) => inner.span(),
            Self::Identifier(inner) => inner.span(),
            Self::LiteralSuffix(inner) => inner.span(),
            Self::MsRestrictModifier(inner) => inner.span(),
            Self::MsSignedPtrModifier(inner) => inner.span(),
            Self::MsUnsignedPtrModifier(inner) => inner.span(),
            Self::NamespaceIdentifier(inner) => inner.span(),
            Self::NumberLiteral(inner) => inner.span(),
            Self::PreprocArg(inner) => inner.span(),
            Self::PreprocDirective(inner) => inner.span(),
            Self::PrimitiveType(inner) => inner.span(),
            Self::RawStringContent(inner) => inner.span(),
            Self::RawStringDelimiter(inner) => inner.span(),
            Self::StatementIdentifier(inner) => inner.span(),
            Self::StringContent(inner) => inner.span(),
            Self::SystemLibString(inner) => inner.span(),
            Self::This(inner) => inner.span(),
            Self::True(inner) => inner.span(),
            Self::TypeIdentifier(inner) => inner.span(),
            Self::Unknown(node) => ::treesitter_types::Span::from(*node),
        }
    }
}
