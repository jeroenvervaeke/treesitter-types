#[derive(Debug, Clone)]
pub enum Declaration<'tree> {
    ClassDeclaration(::std::boxed::Box<ClassDeclaration<'tree>>),
    ConstructorDeclaration(::std::boxed::Box<ConstructorDeclaration<'tree>>),
    ConversionOperatorDeclaration(::std::boxed::Box<ConversionOperatorDeclaration<'tree>>),
    DelegateDeclaration(::std::boxed::Box<DelegateDeclaration<'tree>>),
    DestructorDeclaration(::std::boxed::Box<DestructorDeclaration<'tree>>),
    EnumDeclaration(::std::boxed::Box<EnumDeclaration<'tree>>),
    EventDeclaration(::std::boxed::Box<EventDeclaration<'tree>>),
    EventFieldDeclaration(::std::boxed::Box<EventFieldDeclaration<'tree>>),
    FieldDeclaration(::std::boxed::Box<FieldDeclaration<'tree>>),
    IndexerDeclaration(::std::boxed::Box<IndexerDeclaration<'tree>>),
    InterfaceDeclaration(::std::boxed::Box<InterfaceDeclaration<'tree>>),
    MethodDeclaration(::std::boxed::Box<MethodDeclaration<'tree>>),
    NamespaceDeclaration(::std::boxed::Box<NamespaceDeclaration<'tree>>),
    OperatorDeclaration(::std::boxed::Box<OperatorDeclaration<'tree>>),
    PreprocIf(::std::boxed::Box<PreprocIf<'tree>>),
    PropertyDeclaration(::std::boxed::Box<PropertyDeclaration<'tree>>),
    RecordDeclaration(::std::boxed::Box<RecordDeclaration<'tree>>),
    StructDeclaration(::std::boxed::Box<StructDeclaration<'tree>>),
    UsingDirective(::std::boxed::Box<UsingDirective<'tree>>),
}
impl<'tree> ::treesitter_types::FromNode<'tree> for Declaration<'tree> {
    #[allow(clippy::collapsible_else_if)]
    fn from_node(
        node: ::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        match node.kind() {
            "class_declaration" => Ok(Self::ClassDeclaration(::std::boxed::Box::new(
                <ClassDeclaration as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "constructor_declaration" => Ok(Self::ConstructorDeclaration(::std::boxed::Box::new(
                <ConstructorDeclaration as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "conversion_operator_declaration" => {
                Ok(Self::ConversionOperatorDeclaration(::std::boxed::Box::new(
                    <ConversionOperatorDeclaration as ::treesitter_types::FromNode>::from_node(
                        node, src,
                    )?,
                )))
            }
            "delegate_declaration" => Ok(Self::DelegateDeclaration(::std::boxed::Box::new(
                <DelegateDeclaration as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "destructor_declaration" => Ok(Self::DestructorDeclaration(::std::boxed::Box::new(
                <DestructorDeclaration as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "enum_declaration" => Ok(Self::EnumDeclaration(::std::boxed::Box::new(
                <EnumDeclaration as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "event_declaration" => Ok(Self::EventDeclaration(::std::boxed::Box::new(
                <EventDeclaration as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "event_field_declaration" => Ok(Self::EventFieldDeclaration(::std::boxed::Box::new(
                <EventFieldDeclaration as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "field_declaration" => Ok(Self::FieldDeclaration(::std::boxed::Box::new(
                <FieldDeclaration as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "indexer_declaration" => Ok(Self::IndexerDeclaration(::std::boxed::Box::new(
                <IndexerDeclaration as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "interface_declaration" => Ok(Self::InterfaceDeclaration(::std::boxed::Box::new(
                <InterfaceDeclaration as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "method_declaration" => Ok(Self::MethodDeclaration(::std::boxed::Box::new(
                <MethodDeclaration as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "namespace_declaration" => Ok(Self::NamespaceDeclaration(::std::boxed::Box::new(
                <NamespaceDeclaration as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "operator_declaration" => Ok(Self::OperatorDeclaration(::std::boxed::Box::new(
                <OperatorDeclaration as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "preproc_if" => Ok(Self::PreprocIf(::std::boxed::Box::new(
                <PreprocIf as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "property_declaration" => Ok(Self::PropertyDeclaration(::std::boxed::Box::new(
                <PropertyDeclaration as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "record_declaration" => Ok(Self::RecordDeclaration(::std::boxed::Box::new(
                <RecordDeclaration as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "struct_declaration" => Ok(Self::StructDeclaration(::std::boxed::Box::new(
                <StructDeclaration as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "using_directive" => Ok(Self::UsingDirective(::std::boxed::Box::new(
                <UsingDirective as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            other => Err(::treesitter_types::ParseError::unexpected_kind(other, node)),
        }
    }
}
impl ::treesitter_types::Spanned for Declaration<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        match self {
            Self::ClassDeclaration(inner) => inner.span(),
            Self::ConstructorDeclaration(inner) => inner.span(),
            Self::ConversionOperatorDeclaration(inner) => inner.span(),
            Self::DelegateDeclaration(inner) => inner.span(),
            Self::DestructorDeclaration(inner) => inner.span(),
            Self::EnumDeclaration(inner) => inner.span(),
            Self::EventDeclaration(inner) => inner.span(),
            Self::EventFieldDeclaration(inner) => inner.span(),
            Self::FieldDeclaration(inner) => inner.span(),
            Self::IndexerDeclaration(inner) => inner.span(),
            Self::InterfaceDeclaration(inner) => inner.span(),
            Self::MethodDeclaration(inner) => inner.span(),
            Self::NamespaceDeclaration(inner) => inner.span(),
            Self::OperatorDeclaration(inner) => inner.span(),
            Self::PreprocIf(inner) => inner.span(),
            Self::PropertyDeclaration(inner) => inner.span(),
            Self::RecordDeclaration(inner) => inner.span(),
            Self::StructDeclaration(inner) => inner.span(),
            Self::UsingDirective(inner) => inner.span(),
        }
    }
}
#[derive(Debug, Clone)]
pub enum Expression<'tree> {
    LvalueExpression(::std::boxed::Box<LvalueExpression<'tree>>),
    NonLvalueExpression(::std::boxed::Box<NonLvalueExpression<'tree>>),
}
impl<'tree> ::treesitter_types::FromNode<'tree> for Expression<'tree> {
    #[allow(clippy::collapsible_else_if)]
    fn from_node(
        node: ::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        if let Ok(v) = <LvalueExpression as ::treesitter_types::FromNode>::from_node(node, src) {
            Ok(Self::LvalueExpression(::std::boxed::Box::new(v)))
        } else {
            if let Ok(v) =
                <NonLvalueExpression as ::treesitter_types::FromNode>::from_node(node, src)
            {
                Ok(Self::NonLvalueExpression(::std::boxed::Box::new(v)))
            } else {
                Err(::treesitter_types::ParseError::unexpected_kind(
                    node.kind(),
                    node,
                ))
            }
        }
    }
}
impl ::treesitter_types::Spanned for Expression<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        match self {
            Self::LvalueExpression(inner) => inner.span(),
            Self::NonLvalueExpression(inner) => inner.span(),
        }
    }
}
#[derive(Debug, Clone)]
pub enum Literal<'tree> {
    BooleanLiteral(::std::boxed::Box<BooleanLiteral<'tree>>),
    CharacterLiteral(::std::boxed::Box<CharacterLiteral<'tree>>),
    IntegerLiteral(::std::boxed::Box<IntegerLiteral<'tree>>),
    NullLiteral(::std::boxed::Box<NullLiteral<'tree>>),
    RawStringLiteral(::std::boxed::Box<RawStringLiteral<'tree>>),
    RealLiteral(::std::boxed::Box<RealLiteral<'tree>>),
    StringLiteral(::std::boxed::Box<StringLiteral<'tree>>),
    VerbatimStringLiteral(::std::boxed::Box<VerbatimStringLiteral<'tree>>),
}
impl<'tree> ::treesitter_types::FromNode<'tree> for Literal<'tree> {
    #[allow(clippy::collapsible_else_if)]
    fn from_node(
        node: ::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        match node.kind() {
            "boolean_literal" => Ok(Self::BooleanLiteral(::std::boxed::Box::new(
                <BooleanLiteral as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "character_literal" => Ok(Self::CharacterLiteral(::std::boxed::Box::new(
                <CharacterLiteral as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "integer_literal" => Ok(Self::IntegerLiteral(::std::boxed::Box::new(
                <IntegerLiteral as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "null_literal" => Ok(Self::NullLiteral(::std::boxed::Box::new(
                <NullLiteral as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "raw_string_literal" => Ok(Self::RawStringLiteral(::std::boxed::Box::new(
                <RawStringLiteral as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "real_literal" => Ok(Self::RealLiteral(::std::boxed::Box::new(
                <RealLiteral as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "string_literal" => Ok(Self::StringLiteral(::std::boxed::Box::new(
                <StringLiteral as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "verbatim_string_literal" => Ok(Self::VerbatimStringLiteral(::std::boxed::Box::new(
                <VerbatimStringLiteral as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            other => Err(::treesitter_types::ParseError::unexpected_kind(other, node)),
        }
    }
}
impl ::treesitter_types::Spanned for Literal<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        match self {
            Self::BooleanLiteral(inner) => inner.span(),
            Self::CharacterLiteral(inner) => inner.span(),
            Self::IntegerLiteral(inner) => inner.span(),
            Self::NullLiteral(inner) => inner.span(),
            Self::RawStringLiteral(inner) => inner.span(),
            Self::RealLiteral(inner) => inner.span(),
            Self::StringLiteral(inner) => inner.span(),
            Self::VerbatimStringLiteral(inner) => inner.span(),
        }
    }
}
#[derive(Debug, Clone)]
pub enum LvalueExpression<'tree> {
    ElementAccessExpression(::std::boxed::Box<ElementAccessExpression<'tree>>),
    ElementBindingExpression(::std::boxed::Box<ElementBindingExpression<'tree>>),
    GenericName(::std::boxed::Box<GenericName<'tree>>),
    Identifier(::std::boxed::Box<Identifier<'tree>>),
    MemberAccessExpression(::std::boxed::Box<MemberAccessExpression<'tree>>),
    ParenthesizedExpression(::std::boxed::Box<ParenthesizedExpression<'tree>>),
    PrefixUnaryExpression(::std::boxed::Box<PrefixUnaryExpression<'tree>>),
    This(::treesitter_types::Span),
    TupleExpression(::std::boxed::Box<TupleExpression<'tree>>),
}
impl<'tree> ::treesitter_types::FromNode<'tree> for LvalueExpression<'tree> {
    #[allow(clippy::collapsible_else_if)]
    fn from_node(
        node: ::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        match node.kind() {
            "element_access_expression" => {
                Ok(Self::ElementAccessExpression(::std::boxed::Box::new(
                    <ElementAccessExpression as ::treesitter_types::FromNode>::from_node(
                        node, src,
                    )?,
                )))
            }
            "element_binding_expression" => {
                Ok(Self::ElementBindingExpression(::std::boxed::Box::new(
                    <ElementBindingExpression as ::treesitter_types::FromNode>::from_node(
                        node, src,
                    )?,
                )))
            }
            "generic_name" => Ok(Self::GenericName(::std::boxed::Box::new(
                <GenericName as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "identifier" => Ok(Self::Identifier(::std::boxed::Box::new(
                <Identifier as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "member_access_expression" => Ok(Self::MemberAccessExpression(::std::boxed::Box::new(
                <MemberAccessExpression as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "parenthesized_expression" => {
                Ok(Self::ParenthesizedExpression(::std::boxed::Box::new(
                    <ParenthesizedExpression as ::treesitter_types::FromNode>::from_node(
                        node, src,
                    )?,
                )))
            }
            "prefix_unary_expression" => Ok(Self::PrefixUnaryExpression(::std::boxed::Box::new(
                <PrefixUnaryExpression as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "this" => Ok(Self::This(::treesitter_types::Span::from(node))),
            "tuple_expression" => Ok(Self::TupleExpression(::std::boxed::Box::new(
                <TupleExpression as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            other => Err(::treesitter_types::ParseError::unexpected_kind(other, node)),
        }
    }
}
impl ::treesitter_types::Spanned for LvalueExpression<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        match self {
            Self::ElementAccessExpression(inner) => inner.span(),
            Self::ElementBindingExpression(inner) => inner.span(),
            Self::GenericName(inner) => inner.span(),
            Self::Identifier(inner) => inner.span(),
            Self::MemberAccessExpression(inner) => inner.span(),
            Self::ParenthesizedExpression(inner) => inner.span(),
            Self::PrefixUnaryExpression(inner) => inner.span(),
            Self::This(span) => *span,
            Self::TupleExpression(inner) => inner.span(),
        }
    }
}
#[derive(Debug, Clone)]
pub enum NonLvalueExpression<'tree> {
    AnonymousMethodExpression(::std::boxed::Box<AnonymousMethodExpression<'tree>>),
    AnonymousObjectCreationExpression(::std::boxed::Box<AnonymousObjectCreationExpression<'tree>>),
    ArrayCreationExpression(::std::boxed::Box<ArrayCreationExpression<'tree>>),
    AsExpression(::std::boxed::Box<AsExpression<'tree>>),
    AssignmentExpression(::std::boxed::Box<AssignmentExpression<'tree>>),
    AwaitExpression(::std::boxed::Box<AwaitExpression<'tree>>),
    Base(::treesitter_types::Span),
    BinaryExpression(::std::boxed::Box<BinaryExpression<'tree>>),
    CastExpression(::std::boxed::Box<CastExpression<'tree>>),
    CheckedExpression(::std::boxed::Box<CheckedExpression<'tree>>),
    ConditionalAccessExpression(::std::boxed::Box<ConditionalAccessExpression<'tree>>),
    ConditionalExpression(::std::boxed::Box<ConditionalExpression<'tree>>),
    DefaultExpression(::std::boxed::Box<DefaultExpression<'tree>>),
    ImplicitArrayCreationExpression(::std::boxed::Box<ImplicitArrayCreationExpression<'tree>>),
    ImplicitObjectCreationExpression(::std::boxed::Box<ImplicitObjectCreationExpression<'tree>>),
    ImplicitStackallocExpression(::std::boxed::Box<ImplicitStackallocExpression<'tree>>),
    InitializerExpression(::std::boxed::Box<InitializerExpression<'tree>>),
    InterpolatedStringExpression(::std::boxed::Box<InterpolatedStringExpression<'tree>>),
    InvocationExpression(::std::boxed::Box<InvocationExpression<'tree>>),
    IsExpression(::std::boxed::Box<IsExpression<'tree>>),
    IsPatternExpression(::std::boxed::Box<IsPatternExpression<'tree>>),
    LambdaExpression(::std::boxed::Box<LambdaExpression<'tree>>),
    Literal(::std::boxed::Box<Literal<'tree>>),
    MakerefExpression(::std::boxed::Box<MakerefExpression<'tree>>),
    ObjectCreationExpression(::std::boxed::Box<ObjectCreationExpression<'tree>>),
    ParenthesizedExpression(::std::boxed::Box<ParenthesizedExpression<'tree>>),
    PostfixUnaryExpression(::std::boxed::Box<PostfixUnaryExpression<'tree>>),
    PrefixUnaryExpression(::std::boxed::Box<PrefixUnaryExpression<'tree>>),
    PreprocIf(::std::boxed::Box<PreprocIf<'tree>>),
    QueryExpression(::std::boxed::Box<QueryExpression<'tree>>),
    RangeExpression(::std::boxed::Box<RangeExpression<'tree>>),
    RefExpression(::std::boxed::Box<RefExpression<'tree>>),
    ReftypeExpression(::std::boxed::Box<ReftypeExpression<'tree>>),
    RefvalueExpression(::std::boxed::Box<RefvalueExpression<'tree>>),
    SizeofExpression(::std::boxed::Box<SizeofExpression<'tree>>),
    StackallocExpression(::std::boxed::Box<StackallocExpression<'tree>>),
    SwitchExpression(::std::boxed::Box<SwitchExpression<'tree>>),
    ThrowExpression(::std::boxed::Box<ThrowExpression<'tree>>),
    TypeofExpression(::std::boxed::Box<TypeofExpression<'tree>>),
    WithExpression(::std::boxed::Box<WithExpression<'tree>>),
}
impl<'tree> ::treesitter_types::FromNode<'tree> for NonLvalueExpression<'tree> {
    #[allow(clippy::collapsible_else_if)]
    fn from_node(
        node: ::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        match node.kind() {
            "anonymous_method_expression" => {
                Ok(Self::AnonymousMethodExpression(::std::boxed::Box::new(
                    <AnonymousMethodExpression as ::treesitter_types::FromNode>::from_node(
                        node, src,
                    )?,
                )))
            }
            "anonymous_object_creation_expression" => Ok(Self::AnonymousObjectCreationExpression(
                ::std::boxed::Box::new(
                    <AnonymousObjectCreationExpression as ::treesitter_types::FromNode>::from_node(
                        node, src,
                    )?,
                ),
            )),
            "array_creation_expression" => {
                Ok(Self::ArrayCreationExpression(::std::boxed::Box::new(
                    <ArrayCreationExpression as ::treesitter_types::FromNode>::from_node(
                        node, src,
                    )?,
                )))
            }
            "as_expression" => Ok(Self::AsExpression(::std::boxed::Box::new(
                <AsExpression as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "assignment_expression" => Ok(Self::AssignmentExpression(::std::boxed::Box::new(
                <AssignmentExpression as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "await_expression" => Ok(Self::AwaitExpression(::std::boxed::Box::new(
                <AwaitExpression as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "base" => Ok(Self::Base(::treesitter_types::Span::from(node))),
            "binary_expression" => Ok(Self::BinaryExpression(::std::boxed::Box::new(
                <BinaryExpression as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "cast_expression" => Ok(Self::CastExpression(::std::boxed::Box::new(
                <CastExpression as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "checked_expression" => Ok(Self::CheckedExpression(::std::boxed::Box::new(
                <CheckedExpression as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "conditional_access_expression" => {
                Ok(Self::ConditionalAccessExpression(::std::boxed::Box::new(
                    <ConditionalAccessExpression as ::treesitter_types::FromNode>::from_node(
                        node, src,
                    )?,
                )))
            }
            "conditional_expression" => Ok(Self::ConditionalExpression(::std::boxed::Box::new(
                <ConditionalExpression as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "default_expression" => Ok(Self::DefaultExpression(::std::boxed::Box::new(
                <DefaultExpression as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "implicit_array_creation_expression" => Ok(Self::ImplicitArrayCreationExpression(
                ::std::boxed::Box::new(
                    <ImplicitArrayCreationExpression as ::treesitter_types::FromNode>::from_node(
                        node, src,
                    )?,
                ),
            )),
            "implicit_object_creation_expression" => Ok(Self::ImplicitObjectCreationExpression(
                ::std::boxed::Box::new(
                    <ImplicitObjectCreationExpression as ::treesitter_types::FromNode>::from_node(
                        node, src,
                    )?,
                ),
            )),
            "implicit_stackalloc_expression" => {
                Ok(Self::ImplicitStackallocExpression(::std::boxed::Box::new(
                    <ImplicitStackallocExpression as ::treesitter_types::FromNode>::from_node(
                        node, src,
                    )?,
                )))
            }
            "initializer_expression" => Ok(Self::InitializerExpression(::std::boxed::Box::new(
                <InitializerExpression as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "interpolated_string_expression" => {
                Ok(Self::InterpolatedStringExpression(::std::boxed::Box::new(
                    <InterpolatedStringExpression as ::treesitter_types::FromNode>::from_node(
                        node, src,
                    )?,
                )))
            }
            "invocation_expression" => Ok(Self::InvocationExpression(::std::boxed::Box::new(
                <InvocationExpression as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "is_expression" => Ok(Self::IsExpression(::std::boxed::Box::new(
                <IsExpression as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "is_pattern_expression" => Ok(Self::IsPatternExpression(::std::boxed::Box::new(
                <IsPatternExpression as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "lambda_expression" => Ok(Self::LambdaExpression(::std::boxed::Box::new(
                <LambdaExpression as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "makeref_expression" => Ok(Self::MakerefExpression(::std::boxed::Box::new(
                <MakerefExpression as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "object_creation_expression" => {
                Ok(Self::ObjectCreationExpression(::std::boxed::Box::new(
                    <ObjectCreationExpression as ::treesitter_types::FromNode>::from_node(
                        node, src,
                    )?,
                )))
            }
            "parenthesized_expression" => {
                Ok(Self::ParenthesizedExpression(::std::boxed::Box::new(
                    <ParenthesizedExpression as ::treesitter_types::FromNode>::from_node(
                        node, src,
                    )?,
                )))
            }
            "postfix_unary_expression" => Ok(Self::PostfixUnaryExpression(::std::boxed::Box::new(
                <PostfixUnaryExpression as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "prefix_unary_expression" => Ok(Self::PrefixUnaryExpression(::std::boxed::Box::new(
                <PrefixUnaryExpression as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "preproc_if" => Ok(Self::PreprocIf(::std::boxed::Box::new(
                <PreprocIf as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "query_expression" => Ok(Self::QueryExpression(::std::boxed::Box::new(
                <QueryExpression as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "range_expression" => Ok(Self::RangeExpression(::std::boxed::Box::new(
                <RangeExpression as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "ref_expression" => Ok(Self::RefExpression(::std::boxed::Box::new(
                <RefExpression as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "reftype_expression" => Ok(Self::ReftypeExpression(::std::boxed::Box::new(
                <ReftypeExpression as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "refvalue_expression" => Ok(Self::RefvalueExpression(::std::boxed::Box::new(
                <RefvalueExpression as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "sizeof_expression" => Ok(Self::SizeofExpression(::std::boxed::Box::new(
                <SizeofExpression as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "stackalloc_expression" => Ok(Self::StackallocExpression(::std::boxed::Box::new(
                <StackallocExpression as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "switch_expression" => Ok(Self::SwitchExpression(::std::boxed::Box::new(
                <SwitchExpression as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "throw_expression" => Ok(Self::ThrowExpression(::std::boxed::Box::new(
                <ThrowExpression as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "typeof_expression" => Ok(Self::TypeofExpression(::std::boxed::Box::new(
                <TypeofExpression as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "with_expression" => Ok(Self::WithExpression(::std::boxed::Box::new(
                <WithExpression as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            _other => {
                if let Ok(v) = <Literal as ::treesitter_types::FromNode>::from_node(node, src) {
                    Ok(Self::Literal(::std::boxed::Box::new(v)))
                } else {
                    Err(::treesitter_types::ParseError::unexpected_kind(
                        _other, node,
                    ))
                }
            }
        }
    }
}
impl ::treesitter_types::Spanned for NonLvalueExpression<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        match self {
            Self::AnonymousMethodExpression(inner) => inner.span(),
            Self::AnonymousObjectCreationExpression(inner) => inner.span(),
            Self::ArrayCreationExpression(inner) => inner.span(),
            Self::AsExpression(inner) => inner.span(),
            Self::AssignmentExpression(inner) => inner.span(),
            Self::AwaitExpression(inner) => inner.span(),
            Self::Base(span) => *span,
            Self::BinaryExpression(inner) => inner.span(),
            Self::CastExpression(inner) => inner.span(),
            Self::CheckedExpression(inner) => inner.span(),
            Self::ConditionalAccessExpression(inner) => inner.span(),
            Self::ConditionalExpression(inner) => inner.span(),
            Self::DefaultExpression(inner) => inner.span(),
            Self::ImplicitArrayCreationExpression(inner) => inner.span(),
            Self::ImplicitObjectCreationExpression(inner) => inner.span(),
            Self::ImplicitStackallocExpression(inner) => inner.span(),
            Self::InitializerExpression(inner) => inner.span(),
            Self::InterpolatedStringExpression(inner) => inner.span(),
            Self::InvocationExpression(inner) => inner.span(),
            Self::IsExpression(inner) => inner.span(),
            Self::IsPatternExpression(inner) => inner.span(),
            Self::LambdaExpression(inner) => inner.span(),
            Self::Literal(inner) => inner.span(),
            Self::MakerefExpression(inner) => inner.span(),
            Self::ObjectCreationExpression(inner) => inner.span(),
            Self::ParenthesizedExpression(inner) => inner.span(),
            Self::PostfixUnaryExpression(inner) => inner.span(),
            Self::PrefixUnaryExpression(inner) => inner.span(),
            Self::PreprocIf(inner) => inner.span(),
            Self::QueryExpression(inner) => inner.span(),
            Self::RangeExpression(inner) => inner.span(),
            Self::RefExpression(inner) => inner.span(),
            Self::ReftypeExpression(inner) => inner.span(),
            Self::RefvalueExpression(inner) => inner.span(),
            Self::SizeofExpression(inner) => inner.span(),
            Self::StackallocExpression(inner) => inner.span(),
            Self::SwitchExpression(inner) => inner.span(),
            Self::ThrowExpression(inner) => inner.span(),
            Self::TypeofExpression(inner) => inner.span(),
            Self::WithExpression(inner) => inner.span(),
        }
    }
}
#[derive(Debug, Clone)]
pub enum Pattern<'tree> {
    AndPattern(::std::boxed::Box<AndPattern<'tree>>),
    ConstantPattern(::std::boxed::Box<ConstantPattern<'tree>>),
    DeclarationPattern(::std::boxed::Box<DeclarationPattern<'tree>>),
    Discard(::std::boxed::Box<Discard<'tree>>),
    ListPattern(::std::boxed::Box<ListPattern<'tree>>),
    NegatedPattern(::std::boxed::Box<NegatedPattern<'tree>>),
    OrPattern(::std::boxed::Box<OrPattern<'tree>>),
    ParenthesizedPattern(::std::boxed::Box<ParenthesizedPattern<'tree>>),
    RecursivePattern(::std::boxed::Box<RecursivePattern<'tree>>),
    RelationalPattern(::std::boxed::Box<RelationalPattern<'tree>>),
    TypePattern(::std::boxed::Box<TypePattern<'tree>>),
    VarPattern(::std::boxed::Box<VarPattern<'tree>>),
}
impl<'tree> ::treesitter_types::FromNode<'tree> for Pattern<'tree> {
    #[allow(clippy::collapsible_else_if)]
    fn from_node(
        node: ::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        match node.kind() {
            "and_pattern" => Ok(Self::AndPattern(::std::boxed::Box::new(
                <AndPattern as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "constant_pattern" => Ok(Self::ConstantPattern(::std::boxed::Box::new(
                <ConstantPattern as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "declaration_pattern" => Ok(Self::DeclarationPattern(::std::boxed::Box::new(
                <DeclarationPattern as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "discard" => Ok(Self::Discard(::std::boxed::Box::new(
                <Discard as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "list_pattern" => Ok(Self::ListPattern(::std::boxed::Box::new(
                <ListPattern as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "negated_pattern" => Ok(Self::NegatedPattern(::std::boxed::Box::new(
                <NegatedPattern as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "or_pattern" => Ok(Self::OrPattern(::std::boxed::Box::new(
                <OrPattern as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "parenthesized_pattern" => Ok(Self::ParenthesizedPattern(::std::boxed::Box::new(
                <ParenthesizedPattern as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "recursive_pattern" => Ok(Self::RecursivePattern(::std::boxed::Box::new(
                <RecursivePattern as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "relational_pattern" => Ok(Self::RelationalPattern(::std::boxed::Box::new(
                <RelationalPattern as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "type_pattern" => Ok(Self::TypePattern(::std::boxed::Box::new(
                <TypePattern as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "var_pattern" => Ok(Self::VarPattern(::std::boxed::Box::new(
                <VarPattern as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            other => Err(::treesitter_types::ParseError::unexpected_kind(other, node)),
        }
    }
}
impl ::treesitter_types::Spanned for Pattern<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        match self {
            Self::AndPattern(inner) => inner.span(),
            Self::ConstantPattern(inner) => inner.span(),
            Self::DeclarationPattern(inner) => inner.span(),
            Self::Discard(inner) => inner.span(),
            Self::ListPattern(inner) => inner.span(),
            Self::NegatedPattern(inner) => inner.span(),
            Self::OrPattern(inner) => inner.span(),
            Self::ParenthesizedPattern(inner) => inner.span(),
            Self::RecursivePattern(inner) => inner.span(),
            Self::RelationalPattern(inner) => inner.span(),
            Self::TypePattern(inner) => inner.span(),
            Self::VarPattern(inner) => inner.span(),
        }
    }
}
#[derive(Debug, Clone)]
pub enum Statement<'tree> {
    Block(::std::boxed::Box<Block<'tree>>),
    BreakStatement(::std::boxed::Box<BreakStatement<'tree>>),
    CheckedStatement(::std::boxed::Box<CheckedStatement<'tree>>),
    ContinueStatement(::std::boxed::Box<ContinueStatement<'tree>>),
    DoStatement(::std::boxed::Box<DoStatement<'tree>>),
    EmptyStatement(::std::boxed::Box<EmptyStatement<'tree>>),
    ExpressionStatement(::std::boxed::Box<ExpressionStatement<'tree>>),
    FixedStatement(::std::boxed::Box<FixedStatement<'tree>>),
    ForStatement(::std::boxed::Box<ForStatement<'tree>>),
    ForeachStatement(::std::boxed::Box<ForeachStatement<'tree>>),
    GotoStatement(::std::boxed::Box<GotoStatement<'tree>>),
    IfStatement(::std::boxed::Box<IfStatement<'tree>>),
    LabeledStatement(::std::boxed::Box<LabeledStatement<'tree>>),
    LocalDeclarationStatement(::std::boxed::Box<LocalDeclarationStatement<'tree>>),
    LocalFunctionStatement(::std::boxed::Box<LocalFunctionStatement<'tree>>),
    LockStatement(::std::boxed::Box<LockStatement<'tree>>),
    PreprocIf(::std::boxed::Box<PreprocIf<'tree>>),
    ReturnStatement(::std::boxed::Box<ReturnStatement<'tree>>),
    SwitchStatement(::std::boxed::Box<SwitchStatement<'tree>>),
    ThrowStatement(::std::boxed::Box<ThrowStatement<'tree>>),
    TryStatement(::std::boxed::Box<TryStatement<'tree>>),
    UnsafeStatement(::std::boxed::Box<UnsafeStatement<'tree>>),
    UsingStatement(::std::boxed::Box<UsingStatement<'tree>>),
    WhileStatement(::std::boxed::Box<WhileStatement<'tree>>),
    YieldStatement(::std::boxed::Box<YieldStatement<'tree>>),
}
impl<'tree> ::treesitter_types::FromNode<'tree> for Statement<'tree> {
    #[allow(clippy::collapsible_else_if)]
    fn from_node(
        node: ::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        match node.kind() {
            "block" => Ok(Self::Block(::std::boxed::Box::new(
                <Block as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "break_statement" => Ok(Self::BreakStatement(::std::boxed::Box::new(
                <BreakStatement as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "checked_statement" => Ok(Self::CheckedStatement(::std::boxed::Box::new(
                <CheckedStatement as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "continue_statement" => Ok(Self::ContinueStatement(::std::boxed::Box::new(
                <ContinueStatement as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "do_statement" => Ok(Self::DoStatement(::std::boxed::Box::new(
                <DoStatement as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "empty_statement" => Ok(Self::EmptyStatement(::std::boxed::Box::new(
                <EmptyStatement as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "expression_statement" => Ok(Self::ExpressionStatement(::std::boxed::Box::new(
                <ExpressionStatement as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "fixed_statement" => Ok(Self::FixedStatement(::std::boxed::Box::new(
                <FixedStatement as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "for_statement" => Ok(Self::ForStatement(::std::boxed::Box::new(
                <ForStatement as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "foreach_statement" => Ok(Self::ForeachStatement(::std::boxed::Box::new(
                <ForeachStatement as ::treesitter_types::FromNode>::from_node(node, src)?,
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
            "local_declaration_statement" => {
                Ok(Self::LocalDeclarationStatement(::std::boxed::Box::new(
                    <LocalDeclarationStatement as ::treesitter_types::FromNode>::from_node(
                        node, src,
                    )?,
                )))
            }
            "local_function_statement" => Ok(Self::LocalFunctionStatement(::std::boxed::Box::new(
                <LocalFunctionStatement as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "lock_statement" => Ok(Self::LockStatement(::std::boxed::Box::new(
                <LockStatement as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "preproc_if" => Ok(Self::PreprocIf(::std::boxed::Box::new(
                <PreprocIf as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "return_statement" => Ok(Self::ReturnStatement(::std::boxed::Box::new(
                <ReturnStatement as ::treesitter_types::FromNode>::from_node(node, src)?,
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
            "unsafe_statement" => Ok(Self::UnsafeStatement(::std::boxed::Box::new(
                <UnsafeStatement as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "using_statement" => Ok(Self::UsingStatement(::std::boxed::Box::new(
                <UsingStatement as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "while_statement" => Ok(Self::WhileStatement(::std::boxed::Box::new(
                <WhileStatement as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "yield_statement" => Ok(Self::YieldStatement(::std::boxed::Box::new(
                <YieldStatement as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            other => Err(::treesitter_types::ParseError::unexpected_kind(other, node)),
        }
    }
}
impl ::treesitter_types::Spanned for Statement<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        match self {
            Self::Block(inner) => inner.span(),
            Self::BreakStatement(inner) => inner.span(),
            Self::CheckedStatement(inner) => inner.span(),
            Self::ContinueStatement(inner) => inner.span(),
            Self::DoStatement(inner) => inner.span(),
            Self::EmptyStatement(inner) => inner.span(),
            Self::ExpressionStatement(inner) => inner.span(),
            Self::FixedStatement(inner) => inner.span(),
            Self::ForStatement(inner) => inner.span(),
            Self::ForeachStatement(inner) => inner.span(),
            Self::GotoStatement(inner) => inner.span(),
            Self::IfStatement(inner) => inner.span(),
            Self::LabeledStatement(inner) => inner.span(),
            Self::LocalDeclarationStatement(inner) => inner.span(),
            Self::LocalFunctionStatement(inner) => inner.span(),
            Self::LockStatement(inner) => inner.span(),
            Self::PreprocIf(inner) => inner.span(),
            Self::ReturnStatement(inner) => inner.span(),
            Self::SwitchStatement(inner) => inner.span(),
            Self::ThrowStatement(inner) => inner.span(),
            Self::TryStatement(inner) => inner.span(),
            Self::UnsafeStatement(inner) => inner.span(),
            Self::UsingStatement(inner) => inner.span(),
            Self::WhileStatement(inner) => inner.span(),
            Self::YieldStatement(inner) => inner.span(),
        }
    }
}
#[derive(Debug, Clone)]
pub enum Type<'tree> {
    AliasQualifiedName(::std::boxed::Box<AliasQualifiedName<'tree>>),
    ArrayType(::std::boxed::Box<ArrayType<'tree>>),
    FunctionPointerType(::std::boxed::Box<FunctionPointerType<'tree>>),
    GenericName(::std::boxed::Box<GenericName<'tree>>),
    Identifier(::std::boxed::Box<Identifier<'tree>>),
    ImplicitType(::std::boxed::Box<ImplicitType<'tree>>),
    NullableType(::std::boxed::Box<NullableType<'tree>>),
    PointerType(::std::boxed::Box<PointerType<'tree>>),
    PredefinedType(::std::boxed::Box<PredefinedType<'tree>>),
    QualifiedName(::std::boxed::Box<QualifiedName<'tree>>),
    RefType(::std::boxed::Box<RefType<'tree>>),
    ScopedType(::std::boxed::Box<ScopedType<'tree>>),
    TupleType(::std::boxed::Box<TupleType<'tree>>),
}
impl<'tree> ::treesitter_types::FromNode<'tree> for Type<'tree> {
    #[allow(clippy::collapsible_else_if)]
    fn from_node(
        node: ::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        match node.kind() {
            "alias_qualified_name" => Ok(Self::AliasQualifiedName(::std::boxed::Box::new(
                <AliasQualifiedName as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "array_type" => Ok(Self::ArrayType(::std::boxed::Box::new(
                <ArrayType as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "function_pointer_type" => Ok(Self::FunctionPointerType(::std::boxed::Box::new(
                <FunctionPointerType as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "generic_name" => Ok(Self::GenericName(::std::boxed::Box::new(
                <GenericName as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "identifier" => Ok(Self::Identifier(::std::boxed::Box::new(
                <Identifier as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "implicit_type" => Ok(Self::ImplicitType(::std::boxed::Box::new(
                <ImplicitType as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "nullable_type" => Ok(Self::NullableType(::std::boxed::Box::new(
                <NullableType as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "pointer_type" => Ok(Self::PointerType(::std::boxed::Box::new(
                <PointerType as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "predefined_type" => Ok(Self::PredefinedType(::std::boxed::Box::new(
                <PredefinedType as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "qualified_name" => Ok(Self::QualifiedName(::std::boxed::Box::new(
                <QualifiedName as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "ref_type" => Ok(Self::RefType(::std::boxed::Box::new(
                <RefType as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "scoped_type" => Ok(Self::ScopedType(::std::boxed::Box::new(
                <ScopedType as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "tuple_type" => Ok(Self::TupleType(::std::boxed::Box::new(
                <TupleType as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            other => Err(::treesitter_types::ParseError::unexpected_kind(other, node)),
        }
    }
}
impl ::treesitter_types::Spanned for Type<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        match self {
            Self::AliasQualifiedName(inner) => inner.span(),
            Self::ArrayType(inner) => inner.span(),
            Self::FunctionPointerType(inner) => inner.span(),
            Self::GenericName(inner) => inner.span(),
            Self::Identifier(inner) => inner.span(),
            Self::ImplicitType(inner) => inner.span(),
            Self::NullableType(inner) => inner.span(),
            Self::PointerType(inner) => inner.span(),
            Self::PredefinedType(inner) => inner.span(),
            Self::QualifiedName(inner) => inner.span(),
            Self::RefType(inner) => inner.span(),
            Self::ScopedType(inner) => inner.span(),
            Self::TupleType(inner) => inner.span(),
        }
    }
}
#[derive(Debug, Clone)]
pub enum TypeDeclaration<'tree> {
    ClassDeclaration(::std::boxed::Box<ClassDeclaration<'tree>>),
    DelegateDeclaration(::std::boxed::Box<DelegateDeclaration<'tree>>),
    EnumDeclaration(::std::boxed::Box<EnumDeclaration<'tree>>),
    InterfaceDeclaration(::std::boxed::Box<InterfaceDeclaration<'tree>>),
    RecordDeclaration(::std::boxed::Box<RecordDeclaration<'tree>>),
    StructDeclaration(::std::boxed::Box<StructDeclaration<'tree>>),
}
impl<'tree> ::treesitter_types::FromNode<'tree> for TypeDeclaration<'tree> {
    #[allow(clippy::collapsible_else_if)]
    fn from_node(
        node: ::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        match node.kind() {
            "class_declaration" => Ok(Self::ClassDeclaration(::std::boxed::Box::new(
                <ClassDeclaration as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "delegate_declaration" => Ok(Self::DelegateDeclaration(::std::boxed::Box::new(
                <DelegateDeclaration as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "enum_declaration" => Ok(Self::EnumDeclaration(::std::boxed::Box::new(
                <EnumDeclaration as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "interface_declaration" => Ok(Self::InterfaceDeclaration(::std::boxed::Box::new(
                <InterfaceDeclaration as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "record_declaration" => Ok(Self::RecordDeclaration(::std::boxed::Box::new(
                <RecordDeclaration as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "struct_declaration" => Ok(Self::StructDeclaration(::std::boxed::Box::new(
                <StructDeclaration as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            other => Err(::treesitter_types::ParseError::unexpected_kind(other, node)),
        }
    }
}
impl ::treesitter_types::Spanned for TypeDeclaration<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        match self {
            Self::ClassDeclaration(inner) => inner.span(),
            Self::DelegateDeclaration(inner) => inner.span(),
            Self::EnumDeclaration(inner) => inner.span(),
            Self::InterfaceDeclaration(inner) => inner.span(),
            Self::RecordDeclaration(inner) => inner.span(),
            Self::StructDeclaration(inner) => inner.span(),
        }
    }
}
#[derive(Debug, Clone)]
pub struct AccessorDeclaration<'tree> {
    pub span: ::treesitter_types::Span,
    pub body: ::core::option::Option<AccessorDeclarationBody<'tree>>,
    pub name: AccessorDeclarationName<'tree>,
    pub children: ::std::vec::Vec<AccessorDeclarationChildren<'tree>>,
}
impl<'tree> ::treesitter_types::FromNode<'tree> for AccessorDeclaration<'tree> {
    #[allow(clippy::match_single_binding, clippy::suspicious_else_formatting)]
    fn from_node(
        node: ::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        debug_assert_eq!(node.kind(), "accessor_declaration");
        Ok(Self {
            span: ::treesitter_types::Span::from(node),
            body: match node.child_by_field_name("body") {
                Some(child) => Some(
                    <AccessorDeclarationBody as ::treesitter_types::FromNode>::from_node(
                        child, src,
                    )?,
                ),
                None => None,
            },
            name: {
                let child = node
                    .child_by_field_name("name")
                    .ok_or_else(|| ::treesitter_types::ParseError::missing_field("name", node))?;
                <AccessorDeclarationName as ::treesitter_types::FromNode>::from_node(child, src)?
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
                        <AccessorDeclarationChildren as ::treesitter_types::FromNode>::from_node(
                            child, src,
                        )?,
                    );
                }
                items
            },
        })
    }
}
impl ::treesitter_types::Spanned for AccessorDeclaration<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        self.span
    }
}
#[derive(Debug, Clone)]
pub struct AccessorList<'tree> {
    pub span: ::treesitter_types::Span,
    pub children: ::std::vec::Vec<AccessorDeclaration<'tree>>,
}
impl<'tree> ::treesitter_types::FromNode<'tree> for AccessorList<'tree> {
    #[allow(clippy::match_single_binding, clippy::suspicious_else_formatting)]
    fn from_node(
        node: ::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        debug_assert_eq!(node.kind(), "accessor_list");
        Ok(Self {
            span: ::treesitter_types::Span::from(node),
            children: {
                #[allow(clippy::suspicious_else_formatting)]
                let non_field_children = {
                    let mut cursor = node.walk();
                    let mut result = ::std::vec::Vec::new();
                    if cursor.goto_first_child() {
                        loop {
                            if cursor.field_name().is_none()
                                && cursor.node().is_named()
                                && !cursor.node().is_extra()
                            {
                                result.push(cursor.node());
                            }
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
                        <AccessorDeclaration as ::treesitter_types::FromNode>::from_node(
                            child, src,
                        )?,
                    );
                }
                items
            },
        })
    }
}
impl ::treesitter_types::Spanned for AccessorList<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        self.span
    }
}
#[derive(Debug, Clone)]
pub struct AliasQualifiedName<'tree> {
    pub span: ::treesitter_types::Span,
    pub alias: Identifier<'tree>,
    pub name: AliasQualifiedNameName<'tree>,
}
impl<'tree> ::treesitter_types::FromNode<'tree> for AliasQualifiedName<'tree> {
    #[allow(clippy::match_single_binding, clippy::suspicious_else_formatting)]
    fn from_node(
        node: ::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        debug_assert_eq!(node.kind(), "alias_qualified_name");
        Ok(Self {
            span: ::treesitter_types::Span::from(node),
            alias: {
                let child = node
                    .child_by_field_name("alias")
                    .ok_or_else(|| ::treesitter_types::ParseError::missing_field("alias", node))?;
                <Identifier as ::treesitter_types::FromNode>::from_node(child, src)?
            },
            name: {
                let child = node
                    .child_by_field_name("name")
                    .ok_or_else(|| ::treesitter_types::ParseError::missing_field("name", node))?;
                <AliasQualifiedNameName as ::treesitter_types::FromNode>::from_node(child, src)?
            },
        })
    }
}
impl ::treesitter_types::Spanned for AliasQualifiedName<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        self.span
    }
}
#[derive(Debug, Clone)]
pub struct AndPattern<'tree> {
    pub span: ::treesitter_types::Span,
    pub left: Pattern<'tree>,
    pub operator: AndPatternOperator,
    pub right: Pattern<'tree>,
}
impl<'tree> ::treesitter_types::FromNode<'tree> for AndPattern<'tree> {
    #[allow(clippy::match_single_binding, clippy::suspicious_else_formatting)]
    fn from_node(
        node: ::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        debug_assert_eq!(node.kind(), "and_pattern");
        Ok(Self {
            span: ::treesitter_types::Span::from(node),
            left: {
                let child = node
                    .child_by_field_name("left")
                    .ok_or_else(|| ::treesitter_types::ParseError::missing_field("left", node))?;
                <Pattern as ::treesitter_types::FromNode>::from_node(child, src)?
            },
            operator: {
                let child = node.child_by_field_name("operator").ok_or_else(|| {
                    ::treesitter_types::ParseError::missing_field("operator", node)
                })?;
                <AndPatternOperator as ::treesitter_types::FromNode>::from_node(child, src)?
            },
            right: {
                let child = node
                    .child_by_field_name("right")
                    .ok_or_else(|| ::treesitter_types::ParseError::missing_field("right", node))?;
                <Pattern as ::treesitter_types::FromNode>::from_node(child, src)?
            },
        })
    }
}
impl ::treesitter_types::Spanned for AndPattern<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        self.span
    }
}
#[derive(Debug, Clone)]
pub struct AnonymousMethodExpression<'tree> {
    pub span: ::treesitter_types::Span,
    pub parameters: ::core::option::Option<ParameterList<'tree>>,
    pub children: ::std::vec::Vec<AnonymousMethodExpressionChildren<'tree>>,
}
impl<'tree> ::treesitter_types::FromNode<'tree> for AnonymousMethodExpression<'tree> {
    #[allow(clippy::match_single_binding, clippy::suspicious_else_formatting)]
    fn from_node(
        node: ::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        debug_assert_eq!(node.kind(), "anonymous_method_expression");
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
                    items
                        .push(
                            <AnonymousMethodExpressionChildren as ::treesitter_types::FromNode>::from_node(
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
impl ::treesitter_types::Spanned for AnonymousMethodExpression<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        self.span
    }
}
#[derive(Debug, Clone)]
pub struct AnonymousObjectCreationExpression<'tree> {
    pub span: ::treesitter_types::Span,
    pub children: ::std::vec::Vec<AnonymousObjectCreationExpressionChildren<'tree>>,
}
impl<'tree> ::treesitter_types::FromNode<'tree> for AnonymousObjectCreationExpression<'tree> {
    #[allow(clippy::match_single_binding, clippy::suspicious_else_formatting)]
    fn from_node(
        node: ::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        debug_assert_eq!(node.kind(), "anonymous_object_creation_expression");
        Ok(Self {
            span: ::treesitter_types::Span::from(node),
            children: {
                #[allow(clippy::suspicious_else_formatting)]
                let non_field_children = {
                    let mut cursor = node.walk();
                    let mut result = ::std::vec::Vec::new();
                    if cursor.goto_first_child() {
                        loop {
                            if cursor.field_name().is_none()
                                && cursor.node().is_named()
                                && !cursor.node().is_extra()
                            {
                                result.push(cursor.node());
                            }
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
                            <AnonymousObjectCreationExpressionChildren as ::treesitter_types::FromNode>::from_node(
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
impl ::treesitter_types::Spanned for AnonymousObjectCreationExpression<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        self.span
    }
}
#[derive(Debug, Clone)]
pub struct Argument<'tree> {
    pub span: ::treesitter_types::Span,
    pub name: ::core::option::Option<Identifier<'tree>>,
    pub children: ArgumentChildren<'tree>,
}
impl<'tree> ::treesitter_types::FromNode<'tree> for Argument<'tree> {
    #[allow(clippy::match_single_binding, clippy::suspicious_else_formatting)]
    fn from_node(
        node: ::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        debug_assert_eq!(node.kind(), "argument");
        Ok(Self {
            span: ::treesitter_types::Span::from(node),
            name: match node.child_by_field_name("name") {
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
                <ArgumentChildren as ::treesitter_types::FromNode>::from_node(child, src)?
            },
        })
    }
}
impl ::treesitter_types::Spanned for Argument<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        self.span
    }
}
#[derive(Debug, Clone)]
pub struct ArgumentList<'tree> {
    pub span: ::treesitter_types::Span,
    pub children: ::std::vec::Vec<Argument<'tree>>,
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
                    items.push(<Argument as ::treesitter_types::FromNode>::from_node(
                        child, src,
                    )?);
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
pub struct ArrayCreationExpression<'tree> {
    pub span: ::treesitter_types::Span,
    pub r#type: ArrayType<'tree>,
    pub children: ::core::option::Option<InitializerExpression<'tree>>,
}
impl<'tree> ::treesitter_types::FromNode<'tree> for ArrayCreationExpression<'tree> {
    #[allow(clippy::match_single_binding, clippy::suspicious_else_formatting)]
    fn from_node(
        node: ::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        debug_assert_eq!(node.kind(), "array_creation_expression");
        Ok(Self {
            span: ::treesitter_types::Span::from(node),
            r#type: {
                let child = node
                    .child_by_field_name("type")
                    .ok_or_else(|| ::treesitter_types::ParseError::missing_field("type", node))?;
                <ArrayType as ::treesitter_types::FromNode>::from_node(child, src)?
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
                        <InitializerExpression as ::treesitter_types::FromNode>::from_node(
                            child, src,
                        )?,
                    ),
                    None => None,
                }
            },
        })
    }
}
impl ::treesitter_types::Spanned for ArrayCreationExpression<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        self.span
    }
}
#[derive(Debug, Clone)]
pub struct ArrayRankSpecifier<'tree> {
    pub span: ::treesitter_types::Span,
    pub children: ::std::vec::Vec<Expression<'tree>>,
}
impl<'tree> ::treesitter_types::FromNode<'tree> for ArrayRankSpecifier<'tree> {
    #[allow(clippy::match_single_binding, clippy::suspicious_else_formatting)]
    fn from_node(
        node: ::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        debug_assert_eq!(node.kind(), "array_rank_specifier");
        Ok(Self {
            span: ::treesitter_types::Span::from(node),
            children: {
                #[allow(clippy::suspicious_else_formatting)]
                let non_field_children = {
                    let mut cursor = node.walk();
                    let mut result = ::std::vec::Vec::new();
                    if cursor.goto_first_child() {
                        loop {
                            if cursor.field_name().is_none()
                                && cursor.node().is_named()
                                && !cursor.node().is_extra()
                            {
                                result.push(cursor.node());
                            }
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
impl ::treesitter_types::Spanned for ArrayRankSpecifier<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        self.span
    }
}
#[derive(Debug, Clone)]
pub struct ArrayType<'tree> {
    pub span: ::treesitter_types::Span,
    pub rank: ArrayRankSpecifier<'tree>,
    pub r#type: ArrayTypeType<'tree>,
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
            rank: {
                let child = node
                    .child_by_field_name("rank")
                    .ok_or_else(|| ::treesitter_types::ParseError::missing_field("rank", node))?;
                <ArrayRankSpecifier as ::treesitter_types::FromNode>::from_node(child, src)?
            },
            r#type: {
                let child = node
                    .child_by_field_name("type")
                    .ok_or_else(|| ::treesitter_types::ParseError::missing_field("type", node))?;
                <ArrayTypeType as ::treesitter_types::FromNode>::from_node(child, src)?
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
pub struct ArrowExpressionClause<'tree> {
    pub span: ::treesitter_types::Span,
    pub children: Expression<'tree>,
}
impl<'tree> ::treesitter_types::FromNode<'tree> for ArrowExpressionClause<'tree> {
    #[allow(clippy::match_single_binding, clippy::suspicious_else_formatting)]
    fn from_node(
        node: ::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        debug_assert_eq!(node.kind(), "arrow_expression_clause");
        Ok(Self {
            span: ::treesitter_types::Span::from(node),
            children: {
                #[allow(clippy::suspicious_else_formatting)]
                let non_field_children = {
                    let mut cursor = node.walk();
                    let mut result = ::std::vec::Vec::new();
                    if cursor.goto_first_child() {
                        loop {
                            if cursor.field_name().is_none()
                                && cursor.node().is_named()
                                && !cursor.node().is_extra()
                            {
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
impl ::treesitter_types::Spanned for ArrowExpressionClause<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        self.span
    }
}
#[derive(Debug, Clone)]
pub struct AsExpression<'tree> {
    pub span: ::treesitter_types::Span,
    pub left: Expression<'tree>,
    pub operator: AsExpressionOperator,
    pub right: Type<'tree>,
}
impl<'tree> ::treesitter_types::FromNode<'tree> for AsExpression<'tree> {
    #[allow(clippy::match_single_binding, clippy::suspicious_else_formatting)]
    fn from_node(
        node: ::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        debug_assert_eq!(node.kind(), "as_expression");
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
                <AsExpressionOperator as ::treesitter_types::FromNode>::from_node(child, src)?
            },
            right: {
                let child = node
                    .child_by_field_name("right")
                    .ok_or_else(|| ::treesitter_types::ParseError::missing_field("right", node))?;
                <Type as ::treesitter_types::FromNode>::from_node(child, src)?
            },
        })
    }
}
impl ::treesitter_types::Spanned for AsExpression<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        self.span
    }
}
#[derive(Debug, Clone)]
pub struct AssignmentExpression<'tree> {
    pub span: ::treesitter_types::Span,
    pub left: LvalueExpression<'tree>,
    pub operator: AssignmentExpressionOperator,
    pub right: Expression<'tree>,
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
                <LvalueExpression as ::treesitter_types::FromNode>::from_node(child, src)?
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
                <Expression as ::treesitter_types::FromNode>::from_node(child, src)?
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
    pub name: AttributeName<'tree>,
    pub children: ::core::option::Option<AttributeArgumentList<'tree>>,
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
                <AttributeName as ::treesitter_types::FromNode>::from_node(child, src)?
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
                        <AttributeArgumentList as ::treesitter_types::FromNode>::from_node(
                            child, src,
                        )?,
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
pub struct AttributeArgument<'tree> {
    pub span: ::treesitter_types::Span,
    pub name: ::core::option::Option<Identifier<'tree>>,
    pub children: Expression<'tree>,
}
impl<'tree> ::treesitter_types::FromNode<'tree> for AttributeArgument<'tree> {
    #[allow(clippy::match_single_binding, clippy::suspicious_else_formatting)]
    fn from_node(
        node: ::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        debug_assert_eq!(node.kind(), "attribute_argument");
        Ok(Self {
            span: ::treesitter_types::Span::from(node),
            name: match node.child_by_field_name("name") {
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
impl ::treesitter_types::Spanned for AttributeArgument<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        self.span
    }
}
#[derive(Debug, Clone)]
pub struct AttributeArgumentList<'tree> {
    pub span: ::treesitter_types::Span,
    pub children: ::std::vec::Vec<AttributeArgument<'tree>>,
}
impl<'tree> ::treesitter_types::FromNode<'tree> for AttributeArgumentList<'tree> {
    #[allow(clippy::match_single_binding, clippy::suspicious_else_formatting)]
    fn from_node(
        node: ::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        debug_assert_eq!(node.kind(), "attribute_argument_list");
        Ok(Self {
            span: ::treesitter_types::Span::from(node),
            children: {
                #[allow(clippy::suspicious_else_formatting)]
                let non_field_children = {
                    let mut cursor = node.walk();
                    let mut result = ::std::vec::Vec::new();
                    if cursor.goto_first_child() {
                        loop {
                            if cursor.field_name().is_none()
                                && cursor.node().is_named()
                                && !cursor.node().is_extra()
                            {
                                result.push(cursor.node());
                            }
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
                        <AttributeArgument as ::treesitter_types::FromNode>::from_node(child, src)?,
                    );
                }
                items
            },
        })
    }
}
impl ::treesitter_types::Spanned for AttributeArgumentList<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        self.span
    }
}
#[derive(Debug, Clone)]
pub struct AttributeList<'tree> {
    pub span: ::treesitter_types::Span,
    pub children: ::std::vec::Vec<AttributeListChildren<'tree>>,
}
impl<'tree> ::treesitter_types::FromNode<'tree> for AttributeList<'tree> {
    #[allow(clippy::match_single_binding, clippy::suspicious_else_formatting)]
    fn from_node(
        node: ::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        debug_assert_eq!(node.kind(), "attribute_list");
        Ok(Self {
            span: ::treesitter_types::Span::from(node),
            children: {
                #[allow(clippy::suspicious_else_formatting)]
                let non_field_children = {
                    let mut cursor = node.walk();
                    let mut result = ::std::vec::Vec::new();
                    if cursor.goto_first_child() {
                        loop {
                            if cursor.field_name().is_none()
                                && cursor.node().is_named()
                                && !cursor.node().is_extra()
                            {
                                result.push(cursor.node());
                            }
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
                        <AttributeListChildren as ::treesitter_types::FromNode>::from_node(
                            child, src,
                        )?,
                    );
                }
                items
            },
        })
    }
}
impl ::treesitter_types::Spanned for AttributeList<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        self.span
    }
}
#[derive(Debug, Clone)]
pub struct AttributeTargetSpecifier<'tree> {
    pub span: ::treesitter_types::Span,
    text: &'tree str,
}
impl<'tree> ::treesitter_types::FromNode<'tree> for AttributeTargetSpecifier<'tree> {
    fn from_node(
        node: ::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        debug_assert_eq!(node.kind(), "attribute_target_specifier");
        Ok(Self {
            span: ::treesitter_types::Span::from(node),
            text: node.utf8_text(src)?,
        })
    }
}
impl<'tree> ::treesitter_types::LeafNode<'tree> for AttributeTargetSpecifier<'tree> {
    fn text(&self) -> &'tree str {
        self.text
    }
}
impl ::treesitter_types::Spanned for AttributeTargetSpecifier<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        self.span
    }
}
#[derive(Debug, Clone)]
pub struct AwaitExpression<'tree> {
    pub span: ::treesitter_types::Span,
    pub children: Expression<'tree>,
}
impl<'tree> ::treesitter_types::FromNode<'tree> for AwaitExpression<'tree> {
    #[allow(clippy::match_single_binding, clippy::suspicious_else_formatting)]
    fn from_node(
        node: ::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        debug_assert_eq!(node.kind(), "await_expression");
        Ok(Self {
            span: ::treesitter_types::Span::from(node),
            children: {
                #[allow(clippy::suspicious_else_formatting)]
                let non_field_children = {
                    let mut cursor = node.walk();
                    let mut result = ::std::vec::Vec::new();
                    if cursor.goto_first_child() {
                        loop {
                            if cursor.field_name().is_none()
                                && cursor.node().is_named()
                                && !cursor.node().is_extra()
                            {
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
impl ::treesitter_types::Spanned for AwaitExpression<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        self.span
    }
}
#[derive(Debug, Clone)]
pub struct BaseList<'tree> {
    pub span: ::treesitter_types::Span,
    pub children: ::std::vec::Vec<BaseListChildren<'tree>>,
}
impl<'tree> ::treesitter_types::FromNode<'tree> for BaseList<'tree> {
    #[allow(clippy::match_single_binding, clippy::suspicious_else_formatting)]
    fn from_node(
        node: ::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        debug_assert_eq!(node.kind(), "base_list");
        Ok(Self {
            span: ::treesitter_types::Span::from(node),
            children: {
                #[allow(clippy::suspicious_else_formatting)]
                let non_field_children = {
                    let mut cursor = node.walk();
                    let mut result = ::std::vec::Vec::new();
                    if cursor.goto_first_child() {
                        loop {
                            if cursor.field_name().is_none()
                                && cursor.node().is_named()
                                && !cursor.node().is_extra()
                            {
                                result.push(cursor.node());
                            }
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
                        <BaseListChildren as ::treesitter_types::FromNode>::from_node(child, src)?,
                    );
                }
                items
            },
        })
    }
}
impl ::treesitter_types::Spanned for BaseList<'_> {
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
pub struct Block<'tree> {
    pub span: ::treesitter_types::Span,
    pub children: ::std::vec::Vec<Statement<'tree>>,
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
                    items.push(<Statement as ::treesitter_types::FromNode>::from_node(
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
pub struct BracketedArgumentList<'tree> {
    pub span: ::treesitter_types::Span,
    pub children: ::std::vec::Vec<Argument<'tree>>,
}
impl<'tree> ::treesitter_types::FromNode<'tree> for BracketedArgumentList<'tree> {
    #[allow(clippy::match_single_binding, clippy::suspicious_else_formatting)]
    fn from_node(
        node: ::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        debug_assert_eq!(node.kind(), "bracketed_argument_list");
        Ok(Self {
            span: ::treesitter_types::Span::from(node),
            children: {
                #[allow(clippy::suspicious_else_formatting)]
                let non_field_children = {
                    let mut cursor = node.walk();
                    let mut result = ::std::vec::Vec::new();
                    if cursor.goto_first_child() {
                        loop {
                            if cursor.field_name().is_none()
                                && cursor.node().is_named()
                                && !cursor.node().is_extra()
                            {
                                result.push(cursor.node());
                            }
                            if !cursor.goto_next_sibling() {
                                break;
                            }
                        }
                    }
                    result
                };
                let mut items = ::std::vec::Vec::new();
                for child in non_field_children {
                    items.push(<Argument as ::treesitter_types::FromNode>::from_node(
                        child, src,
                    )?);
                }
                items
            },
        })
    }
}
impl ::treesitter_types::Spanned for BracketedArgumentList<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        self.span
    }
}
#[derive(Debug, Clone)]
pub struct BracketedParameterList<'tree> {
    pub span: ::treesitter_types::Span,
    pub name: ::std::vec::Vec<Identifier<'tree>>,
    pub r#type: ::std::vec::Vec<Type<'tree>>,
    pub children: ::std::vec::Vec<BracketedParameterListChildren<'tree>>,
}
impl<'tree> ::treesitter_types::FromNode<'tree> for BracketedParameterList<'tree> {
    #[allow(clippy::match_single_binding, clippy::suspicious_else_formatting)]
    fn from_node(
        node: ::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        debug_assert_eq!(node.kind(), "bracketed_parameter_list");
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
                    items.push(<Type as ::treesitter_types::FromNode>::from_node(
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
                    items
                        .push(
                            <BracketedParameterListChildren as ::treesitter_types::FromNode>::from_node(
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
impl ::treesitter_types::Spanned for BracketedParameterList<'_> {
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
pub struct CallingConvention<'tree> {
    pub span: ::treesitter_types::Span,
    pub children: ::std::vec::Vec<Identifier<'tree>>,
}
impl<'tree> ::treesitter_types::FromNode<'tree> for CallingConvention<'tree> {
    #[allow(clippy::match_single_binding, clippy::suspicious_else_formatting)]
    fn from_node(
        node: ::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        debug_assert_eq!(node.kind(), "calling_convention");
        Ok(Self {
            span: ::treesitter_types::Span::from(node),
            children: {
                #[allow(clippy::suspicious_else_formatting)]
                let non_field_children = {
                    let mut cursor = node.walk();
                    let mut result = ::std::vec::Vec::new();
                    if cursor.goto_first_child() {
                        loop {
                            if cursor.field_name().is_none()
                                && cursor.node().is_named()
                                && !cursor.node().is_extra()
                            {
                                result.push(cursor.node());
                            }
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
impl ::treesitter_types::Spanned for CallingConvention<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        self.span
    }
}
#[derive(Debug, Clone)]
pub struct CastExpression<'tree> {
    pub span: ::treesitter_types::Span,
    pub r#type: Type<'tree>,
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
                <Type as ::treesitter_types::FromNode>::from_node(child, src)?
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
    pub body: Block<'tree>,
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
            body: {
                let child = node
                    .child_by_field_name("body")
                    .ok_or_else(|| ::treesitter_types::ParseError::missing_field("body", node))?;
                <Block as ::treesitter_types::FromNode>::from_node(child, src)?
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
pub struct CatchDeclaration<'tree> {
    pub span: ::treesitter_types::Span,
    pub name: ::core::option::Option<Identifier<'tree>>,
    pub r#type: Type<'tree>,
}
impl<'tree> ::treesitter_types::FromNode<'tree> for CatchDeclaration<'tree> {
    #[allow(clippy::match_single_binding, clippy::suspicious_else_formatting)]
    fn from_node(
        node: ::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        debug_assert_eq!(node.kind(), "catch_declaration");
        Ok(Self {
            span: ::treesitter_types::Span::from(node),
            name: match node.child_by_field_name("name") {
                Some(child) => Some(<Identifier as ::treesitter_types::FromNode>::from_node(
                    child, src,
                )?),
                None => None,
            },
            r#type: {
                let child = node
                    .child_by_field_name("type")
                    .ok_or_else(|| ::treesitter_types::ParseError::missing_field("type", node))?;
                <Type as ::treesitter_types::FromNode>::from_node(child, src)?
            },
        })
    }
}
impl ::treesitter_types::Spanned for CatchDeclaration<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        self.span
    }
}
#[derive(Debug, Clone)]
pub struct CatchFilterClause<'tree> {
    pub span: ::treesitter_types::Span,
    pub children: Expression<'tree>,
}
impl<'tree> ::treesitter_types::FromNode<'tree> for CatchFilterClause<'tree> {
    #[allow(clippy::match_single_binding, clippy::suspicious_else_formatting)]
    fn from_node(
        node: ::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        debug_assert_eq!(node.kind(), "catch_filter_clause");
        Ok(Self {
            span: ::treesitter_types::Span::from(node),
            children: {
                #[allow(clippy::suspicious_else_formatting)]
                let non_field_children = {
                    let mut cursor = node.walk();
                    let mut result = ::std::vec::Vec::new();
                    if cursor.goto_first_child() {
                        loop {
                            if cursor.field_name().is_none()
                                && cursor.node().is_named()
                                && !cursor.node().is_extra()
                            {
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
impl ::treesitter_types::Spanned for CatchFilterClause<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        self.span
    }
}
#[derive(Debug, Clone)]
pub struct CharacterLiteral<'tree> {
    pub span: ::treesitter_types::Span,
    pub children: CharacterLiteralChildren<'tree>,
}
impl<'tree> ::treesitter_types::FromNode<'tree> for CharacterLiteral<'tree> {
    #[allow(clippy::match_single_binding, clippy::suspicious_else_formatting)]
    fn from_node(
        node: ::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        debug_assert_eq!(node.kind(), "character_literal");
        Ok(Self {
            span: ::treesitter_types::Span::from(node),
            children: {
                #[allow(clippy::suspicious_else_formatting)]
                let non_field_children = {
                    let mut cursor = node.walk();
                    let mut result = ::std::vec::Vec::new();
                    if cursor.goto_first_child() {
                        loop {
                            if cursor.field_name().is_none()
                                && cursor.node().is_named()
                                && !cursor.node().is_extra()
                            {
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
                <CharacterLiteralChildren as ::treesitter_types::FromNode>::from_node(child, src)?
            },
        })
    }
}
impl ::treesitter_types::Spanned for CharacterLiteral<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        self.span
    }
}
#[derive(Debug, Clone)]
pub struct CheckedExpression<'tree> {
    pub span: ::treesitter_types::Span,
    pub children: Expression<'tree>,
}
impl<'tree> ::treesitter_types::FromNode<'tree> for CheckedExpression<'tree> {
    #[allow(clippy::match_single_binding, clippy::suspicious_else_formatting)]
    fn from_node(
        node: ::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        debug_assert_eq!(node.kind(), "checked_expression");
        Ok(Self {
            span: ::treesitter_types::Span::from(node),
            children: {
                #[allow(clippy::suspicious_else_formatting)]
                let non_field_children = {
                    let mut cursor = node.walk();
                    let mut result = ::std::vec::Vec::new();
                    if cursor.goto_first_child() {
                        loop {
                            if cursor.field_name().is_none()
                                && cursor.node().is_named()
                                && !cursor.node().is_extra()
                            {
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
impl ::treesitter_types::Spanned for CheckedExpression<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        self.span
    }
}
#[derive(Debug, Clone)]
pub struct CheckedStatement<'tree> {
    pub span: ::treesitter_types::Span,
    pub children: Block<'tree>,
}
impl<'tree> ::treesitter_types::FromNode<'tree> for CheckedStatement<'tree> {
    #[allow(clippy::match_single_binding, clippy::suspicious_else_formatting)]
    fn from_node(
        node: ::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        debug_assert_eq!(node.kind(), "checked_statement");
        Ok(Self {
            span: ::treesitter_types::Span::from(node),
            children: {
                #[allow(clippy::suspicious_else_formatting)]
                let non_field_children = {
                    let mut cursor = node.walk();
                    let mut result = ::std::vec::Vec::new();
                    if cursor.goto_first_child() {
                        loop {
                            if cursor.field_name().is_none()
                                && cursor.node().is_named()
                                && !cursor.node().is_extra()
                            {
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
                <Block as ::treesitter_types::FromNode>::from_node(child, src)?
            },
        })
    }
}
impl ::treesitter_types::Spanned for CheckedStatement<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        self.span
    }
}
#[derive(Debug, Clone)]
pub struct ClassDeclaration<'tree> {
    pub span: ::treesitter_types::Span,
    pub body: ::core::option::Option<DeclarationList<'tree>>,
    pub name: Identifier<'tree>,
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
            body: match node.child_by_field_name("body") {
                Some(child) => {
                    Some(<DeclarationList as ::treesitter_types::FromNode>::from_node(child, src)?)
                }
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
pub struct CompilationUnit<'tree> {
    pub span: ::treesitter_types::Span,
    pub children: ::std::vec::Vec<CompilationUnitChildren<'tree>>,
}
impl<'tree> ::treesitter_types::FromNode<'tree> for CompilationUnit<'tree> {
    #[allow(clippy::match_single_binding, clippy::suspicious_else_formatting)]
    fn from_node(
        node: ::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        debug_assert_eq!(node.kind(), "compilation_unit");
        Ok(Self {
            span: ::treesitter_types::Span::from(node),
            children: {
                #[allow(clippy::suspicious_else_formatting)]
                let non_field_children = {
                    let mut cursor = node.walk();
                    let mut result = ::std::vec::Vec::new();
                    if cursor.goto_first_child() {
                        loop {
                            if cursor.field_name().is_none()
                                && cursor.node().is_named()
                                && !cursor.node().is_extra()
                            {
                                result.push(cursor.node());
                            }
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
                        <CompilationUnitChildren as ::treesitter_types::FromNode>::from_node(
                            child, src,
                        )?,
                    );
                }
                items
            },
        })
    }
}
impl ::treesitter_types::Spanned for CompilationUnit<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        self.span
    }
}
#[derive(Debug, Clone)]
pub struct ConditionalAccessExpression<'tree> {
    pub span: ::treesitter_types::Span,
    pub condition: Expression<'tree>,
    pub children: ConditionalAccessExpressionChildren<'tree>,
}
impl<'tree> ::treesitter_types::FromNode<'tree> for ConditionalAccessExpression<'tree> {
    #[allow(clippy::match_single_binding, clippy::suspicious_else_formatting)]
    fn from_node(
        node: ::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        debug_assert_eq!(node.kind(), "conditional_access_expression");
        Ok(Self {
            span: ::treesitter_types::Span::from(node),
            condition: {
                let child = node.child_by_field_name("condition").ok_or_else(|| {
                    ::treesitter_types::ParseError::missing_field("condition", node)
                })?;
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
                <ConditionalAccessExpressionChildren as ::treesitter_types::FromNode>::from_node(
                    child, src,
                )?
            },
        })
    }
}
impl ::treesitter_types::Spanned for ConditionalAccessExpression<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        self.span
    }
}
#[derive(Debug, Clone)]
pub struct ConditionalExpression<'tree> {
    pub span: ::treesitter_types::Span,
    pub alternative: Expression<'tree>,
    pub condition: Expression<'tree>,
    pub consequence: Expression<'tree>,
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
            consequence: {
                let child = node.child_by_field_name("consequence").ok_or_else(|| {
                    ::treesitter_types::ParseError::missing_field("consequence", node)
                })?;
                <Expression as ::treesitter_types::FromNode>::from_node(child, src)?
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
pub struct ConstantPattern<'tree> {
    pub span: ::treesitter_types::Span,
    pub children: ConstantPatternChildren<'tree>,
}
impl<'tree> ::treesitter_types::FromNode<'tree> for ConstantPattern<'tree> {
    #[allow(clippy::match_single_binding, clippy::suspicious_else_formatting)]
    fn from_node(
        node: ::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        debug_assert_eq!(node.kind(), "constant_pattern");
        Ok(Self {
            span: ::treesitter_types::Span::from(node),
            children: {
                #[allow(clippy::suspicious_else_formatting)]
                let non_field_children = {
                    let mut cursor = node.walk();
                    let mut result = ::std::vec::Vec::new();
                    if cursor.goto_first_child() {
                        loop {
                            if cursor.field_name().is_none()
                                && cursor.node().is_named()
                                && !cursor.node().is_extra()
                            {
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
                <ConstantPatternChildren as ::treesitter_types::FromNode>::from_node(child, src)?
            },
        })
    }
}
impl ::treesitter_types::Spanned for ConstantPattern<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        self.span
    }
}
#[derive(Debug, Clone)]
pub struct ConstructorConstraint<'tree> {
    pub span: ::treesitter_types::Span,
    text: &'tree str,
}
impl<'tree> ::treesitter_types::FromNode<'tree> for ConstructorConstraint<'tree> {
    fn from_node(
        node: ::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        debug_assert_eq!(node.kind(), "constructor_constraint");
        Ok(Self {
            span: ::treesitter_types::Span::from(node),
            text: node.utf8_text(src)?,
        })
    }
}
impl<'tree> ::treesitter_types::LeafNode<'tree> for ConstructorConstraint<'tree> {
    fn text(&self) -> &'tree str {
        self.text
    }
}
impl ::treesitter_types::Spanned for ConstructorConstraint<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        self.span
    }
}
#[derive(Debug, Clone)]
pub struct ConstructorDeclaration<'tree> {
    pub span: ::treesitter_types::Span,
    pub body: ::core::option::Option<ConstructorDeclarationBody<'tree>>,
    pub name: Identifier<'tree>,
    pub parameters: ParameterList<'tree>,
    pub children: ::std::vec::Vec<ConstructorDeclarationChildren<'tree>>,
}
impl<'tree> ::treesitter_types::FromNode<'tree> for ConstructorDeclaration<'tree> {
    #[allow(clippy::match_single_binding, clippy::suspicious_else_formatting)]
    fn from_node(
        node: ::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        debug_assert_eq!(node.kind(), "constructor_declaration");
        Ok(Self {
            span: ::treesitter_types::Span::from(node),
            body: match node.child_by_field_name("body") {
                Some(child) => Some(
                    <ConstructorDeclarationBody as ::treesitter_types::FromNode>::from_node(
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
                            <ConstructorDeclarationChildren as ::treesitter_types::FromNode>::from_node(
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
impl ::treesitter_types::Spanned for ConstructorDeclaration<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        self.span
    }
}
#[derive(Debug, Clone)]
pub struct ConstructorInitializer<'tree> {
    pub span: ::treesitter_types::Span,
    pub children: ArgumentList<'tree>,
}
impl<'tree> ::treesitter_types::FromNode<'tree> for ConstructorInitializer<'tree> {
    #[allow(clippy::match_single_binding, clippy::suspicious_else_formatting)]
    fn from_node(
        node: ::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        debug_assert_eq!(node.kind(), "constructor_initializer");
        Ok(Self {
            span: ::treesitter_types::Span::from(node),
            children: {
                #[allow(clippy::suspicious_else_formatting)]
                let non_field_children = {
                    let mut cursor = node.walk();
                    let mut result = ::std::vec::Vec::new();
                    if cursor.goto_first_child() {
                        loop {
                            if cursor.field_name().is_none()
                                && cursor.node().is_named()
                                && !cursor.node().is_extra()
                            {
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
impl ::treesitter_types::Spanned for ConstructorInitializer<'_> {
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
pub struct ConversionOperatorDeclaration<'tree> {
    pub span: ::treesitter_types::Span,
    pub body: ::core::option::Option<ConversionOperatorDeclarationBody<'tree>>,
    pub parameters: ParameterList<'tree>,
    pub r#type: Type<'tree>,
    pub children: ::std::vec::Vec<ConversionOperatorDeclarationChildren<'tree>>,
}
impl<'tree> ::treesitter_types::FromNode<'tree> for ConversionOperatorDeclaration<'tree> {
    #[allow(clippy::match_single_binding, clippy::suspicious_else_formatting)]
    fn from_node(
        node: ::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        debug_assert_eq!(node.kind(), "conversion_operator_declaration");
        Ok(Self {
            span: ::treesitter_types::Span::from(node),
            body: match node.child_by_field_name("body") {
                Some(child) => Some(
                    <ConversionOperatorDeclarationBody as ::treesitter_types::FromNode>::from_node(
                        child, src,
                    )?,
                ),
                None => None,
            },
            parameters: {
                let child = node.child_by_field_name("parameters").ok_or_else(|| {
                    ::treesitter_types::ParseError::missing_field("parameters", node)
                })?;
                <ParameterList as ::treesitter_types::FromNode>::from_node(child, src)?
            },
            r#type: {
                let child = node
                    .child_by_field_name("type")
                    .ok_or_else(|| ::treesitter_types::ParseError::missing_field("type", node))?;
                <Type as ::treesitter_types::FromNode>::from_node(child, src)?
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
                            <ConversionOperatorDeclarationChildren as ::treesitter_types::FromNode>::from_node(
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
impl ::treesitter_types::Spanned for ConversionOperatorDeclaration<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        self.span
    }
}
#[derive(Debug, Clone)]
pub struct DeclarationExpression<'tree> {
    pub span: ::treesitter_types::Span,
    pub name: Identifier<'tree>,
    pub r#type: Type<'tree>,
}
impl<'tree> ::treesitter_types::FromNode<'tree> for DeclarationExpression<'tree> {
    #[allow(clippy::match_single_binding, clippy::suspicious_else_formatting)]
    fn from_node(
        node: ::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        debug_assert_eq!(node.kind(), "declaration_expression");
        Ok(Self {
            span: ::treesitter_types::Span::from(node),
            name: {
                let child = node
                    .child_by_field_name("name")
                    .ok_or_else(|| ::treesitter_types::ParseError::missing_field("name", node))?;
                <Identifier as ::treesitter_types::FromNode>::from_node(child, src)?
            },
            r#type: {
                let child = node
                    .child_by_field_name("type")
                    .ok_or_else(|| ::treesitter_types::ParseError::missing_field("type", node))?;
                <Type as ::treesitter_types::FromNode>::from_node(child, src)?
            },
        })
    }
}
impl ::treesitter_types::Spanned for DeclarationExpression<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        self.span
    }
}
#[derive(Debug, Clone)]
pub struct DeclarationList<'tree> {
    pub span: ::treesitter_types::Span,
    pub children: ::std::vec::Vec<Declaration<'tree>>,
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
                    items.push(<Declaration as ::treesitter_types::FromNode>::from_node(
                        child, src,
                    )?);
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
pub struct DeclarationPattern<'tree> {
    pub span: ::treesitter_types::Span,
    pub name: ::core::option::Option<Identifier<'tree>>,
    pub r#type: Type<'tree>,
    pub children: ::core::option::Option<DeclarationPatternChildren<'tree>>,
}
impl<'tree> ::treesitter_types::FromNode<'tree> for DeclarationPattern<'tree> {
    #[allow(clippy::match_single_binding, clippy::suspicious_else_formatting)]
    fn from_node(
        node: ::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        debug_assert_eq!(node.kind(), "declaration_pattern");
        Ok(Self {
            span: ::treesitter_types::Span::from(node),
            name: match node.child_by_field_name("name") {
                Some(child) => Some(<Identifier as ::treesitter_types::FromNode>::from_node(
                    child, src,
                )?),
                None => None,
            },
            r#type: {
                let child = node
                    .child_by_field_name("type")
                    .ok_or_else(|| ::treesitter_types::ParseError::missing_field("type", node))?;
                <Type as ::treesitter_types::FromNode>::from_node(child, src)?
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
                        <DeclarationPatternChildren as ::treesitter_types::FromNode>::from_node(
                            child, src,
                        )?,
                    ),
                    None => None,
                }
            },
        })
    }
}
impl ::treesitter_types::Spanned for DeclarationPattern<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        self.span
    }
}
#[derive(Debug, Clone)]
pub struct DefaultExpression<'tree> {
    pub span: ::treesitter_types::Span,
    pub r#type: ::core::option::Option<Type<'tree>>,
}
impl<'tree> ::treesitter_types::FromNode<'tree> for DefaultExpression<'tree> {
    #[allow(clippy::match_single_binding, clippy::suspicious_else_formatting)]
    fn from_node(
        node: ::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        debug_assert_eq!(node.kind(), "default_expression");
        Ok(Self {
            span: ::treesitter_types::Span::from(node),
            r#type: match node.child_by_field_name("type") {
                Some(child) => Some(<Type as ::treesitter_types::FromNode>::from_node(
                    child, src,
                )?),
                None => None,
            },
        })
    }
}
impl ::treesitter_types::Spanned for DefaultExpression<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        self.span
    }
}
#[derive(Debug, Clone)]
pub struct DelegateDeclaration<'tree> {
    pub span: ::treesitter_types::Span,
    pub name: Identifier<'tree>,
    pub parameters: ParameterList<'tree>,
    pub r#type: Type<'tree>,
    pub type_parameters: ::core::option::Option<TypeParameterList<'tree>>,
    pub children: ::std::vec::Vec<DelegateDeclarationChildren<'tree>>,
}
impl<'tree> ::treesitter_types::FromNode<'tree> for DelegateDeclaration<'tree> {
    #[allow(clippy::match_single_binding, clippy::suspicious_else_formatting)]
    fn from_node(
        node: ::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        debug_assert_eq!(node.kind(), "delegate_declaration");
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
                <ParameterList as ::treesitter_types::FromNode>::from_node(child, src)?
            },
            r#type: {
                let child = node
                    .child_by_field_name("type")
                    .ok_or_else(|| ::treesitter_types::ParseError::missing_field("type", node))?;
                <Type as ::treesitter_types::FromNode>::from_node(child, src)?
            },
            type_parameters: match node.child_by_field_name("type_parameters") {
                Some(child) => Some(
                    <TypeParameterList as ::treesitter_types::FromNode>::from_node(child, src)?,
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
                        <DelegateDeclarationChildren as ::treesitter_types::FromNode>::from_node(
                            child, src,
                        )?,
                    );
                }
                items
            },
        })
    }
}
impl ::treesitter_types::Spanned for DelegateDeclaration<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        self.span
    }
}
#[derive(Debug, Clone)]
pub struct DestructorDeclaration<'tree> {
    pub span: ::treesitter_types::Span,
    pub body: ::core::option::Option<DestructorDeclarationBody<'tree>>,
    pub name: Identifier<'tree>,
    pub parameters: ParameterList<'tree>,
    pub children: ::std::vec::Vec<DestructorDeclarationChildren<'tree>>,
}
impl<'tree> ::treesitter_types::FromNode<'tree> for DestructorDeclaration<'tree> {
    #[allow(clippy::match_single_binding, clippy::suspicious_else_formatting)]
    fn from_node(
        node: ::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        debug_assert_eq!(node.kind(), "destructor_declaration");
        Ok(Self {
            span: ::treesitter_types::Span::from(node),
            body: match node.child_by_field_name("body") {
                Some(child) => Some(
                    <DestructorDeclarationBody as ::treesitter_types::FromNode>::from_node(
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
                        <DestructorDeclarationChildren as ::treesitter_types::FromNode>::from_node(
                            child, src,
                        )?,
                    );
                }
                items
            },
        })
    }
}
impl ::treesitter_types::Spanned for DestructorDeclaration<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        self.span
    }
}
#[derive(Debug, Clone)]
pub struct DoStatement<'tree> {
    pub span: ::treesitter_types::Span,
    pub body: Statement<'tree>,
    pub condition: Expression<'tree>,
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
                <Expression as ::treesitter_types::FromNode>::from_node(child, src)?
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
pub struct ElementAccessExpression<'tree> {
    pub span: ::treesitter_types::Span,
    pub expression: Expression<'tree>,
    pub subscript: BracketedArgumentList<'tree>,
}
impl<'tree> ::treesitter_types::FromNode<'tree> for ElementAccessExpression<'tree> {
    #[allow(clippy::match_single_binding, clippy::suspicious_else_formatting)]
    fn from_node(
        node: ::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        debug_assert_eq!(node.kind(), "element_access_expression");
        Ok(Self {
            span: ::treesitter_types::Span::from(node),
            expression: {
                let child = node.child_by_field_name("expression").ok_or_else(|| {
                    ::treesitter_types::ParseError::missing_field("expression", node)
                })?;
                <Expression as ::treesitter_types::FromNode>::from_node(child, src)?
            },
            subscript: {
                let child = node.child_by_field_name("subscript").ok_or_else(|| {
                    ::treesitter_types::ParseError::missing_field("subscript", node)
                })?;
                <BracketedArgumentList as ::treesitter_types::FromNode>::from_node(child, src)?
            },
        })
    }
}
impl ::treesitter_types::Spanned for ElementAccessExpression<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        self.span
    }
}
#[derive(Debug, Clone)]
pub struct ElementBindingExpression<'tree> {
    pub span: ::treesitter_types::Span,
    pub children: ::std::vec::Vec<Argument<'tree>>,
}
impl<'tree> ::treesitter_types::FromNode<'tree> for ElementBindingExpression<'tree> {
    #[allow(clippy::match_single_binding, clippy::suspicious_else_formatting)]
    fn from_node(
        node: ::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        debug_assert_eq!(node.kind(), "element_binding_expression");
        Ok(Self {
            span: ::treesitter_types::Span::from(node),
            children: {
                #[allow(clippy::suspicious_else_formatting)]
                let non_field_children = {
                    let mut cursor = node.walk();
                    let mut result = ::std::vec::Vec::new();
                    if cursor.goto_first_child() {
                        loop {
                            if cursor.field_name().is_none()
                                && cursor.node().is_named()
                                && !cursor.node().is_extra()
                            {
                                result.push(cursor.node());
                            }
                            if !cursor.goto_next_sibling() {
                                break;
                            }
                        }
                    }
                    result
                };
                let mut items = ::std::vec::Vec::new();
                for child in non_field_children {
                    items.push(<Argument as ::treesitter_types::FromNode>::from_node(
                        child, src,
                    )?);
                }
                items
            },
        })
    }
}
impl ::treesitter_types::Spanned for ElementBindingExpression<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        self.span
    }
}
#[derive(Debug, Clone)]
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
#[derive(Debug, Clone)]
pub struct EnumDeclaration<'tree> {
    pub span: ::treesitter_types::Span,
    pub body: ::core::option::Option<EnumMemberDeclarationList<'tree>>,
    pub name: Identifier<'tree>,
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
            body: match node.child_by_field_name("body") {
                Some(child) => Some(
                    <EnumMemberDeclarationList as ::treesitter_types::FromNode>::from_node(
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
pub struct EnumMemberDeclaration<'tree> {
    pub span: ::treesitter_types::Span,
    pub name: Identifier<'tree>,
    pub value: ::core::option::Option<Expression<'tree>>,
    pub children: ::std::vec::Vec<EnumMemberDeclarationChildren<'tree>>,
}
impl<'tree> ::treesitter_types::FromNode<'tree> for EnumMemberDeclaration<'tree> {
    #[allow(clippy::match_single_binding, clippy::suspicious_else_formatting)]
    fn from_node(
        node: ::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        debug_assert_eq!(node.kind(), "enum_member_declaration");
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
            children: {
                #[allow(clippy::suspicious_else_formatting)]
                let non_field_children = {
                    let mut cursor = node.walk();
                    let mut result = ::std::vec::Vec::new();
                    if cursor.goto_first_child() {
                        loop {
                            if cursor.field_name().is_none()
                                && cursor.node().is_named()
                                && !cursor.node().is_extra()
                            {
                                result.push(cursor.node());
                            }
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
                        <EnumMemberDeclarationChildren as ::treesitter_types::FromNode>::from_node(
                            child, src,
                        )?,
                    );
                }
                items
            },
        })
    }
}
impl ::treesitter_types::Spanned for EnumMemberDeclaration<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        self.span
    }
}
#[derive(Debug, Clone)]
pub struct EnumMemberDeclarationList<'tree> {
    pub span: ::treesitter_types::Span,
    pub children: ::std::vec::Vec<EnumMemberDeclarationListChildren<'tree>>,
}
impl<'tree> ::treesitter_types::FromNode<'tree> for EnumMemberDeclarationList<'tree> {
    #[allow(clippy::match_single_binding, clippy::suspicious_else_formatting)]
    fn from_node(
        node: ::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        debug_assert_eq!(node.kind(), "enum_member_declaration_list");
        Ok(Self {
            span: ::treesitter_types::Span::from(node),
            children: {
                #[allow(clippy::suspicious_else_formatting)]
                let non_field_children = {
                    let mut cursor = node.walk();
                    let mut result = ::std::vec::Vec::new();
                    if cursor.goto_first_child() {
                        loop {
                            if cursor.field_name().is_none()
                                && cursor.node().is_named()
                                && !cursor.node().is_extra()
                            {
                                result.push(cursor.node());
                            }
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
                            <EnumMemberDeclarationListChildren as ::treesitter_types::FromNode>::from_node(
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
impl ::treesitter_types::Spanned for EnumMemberDeclarationList<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        self.span
    }
}
#[derive(Debug, Clone)]
pub struct EventDeclaration<'tree> {
    pub span: ::treesitter_types::Span,
    pub accessors: ::core::option::Option<AccessorList<'tree>>,
    pub name: Identifier<'tree>,
    pub r#type: Type<'tree>,
    pub children: ::std::vec::Vec<EventDeclarationChildren<'tree>>,
}
impl<'tree> ::treesitter_types::FromNode<'tree> for EventDeclaration<'tree> {
    #[allow(clippy::match_single_binding, clippy::suspicious_else_formatting)]
    fn from_node(
        node: ::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        debug_assert_eq!(node.kind(), "event_declaration");
        Ok(Self {
            span: ::treesitter_types::Span::from(node),
            accessors: match node.child_by_field_name("accessors") {
                Some(child) => Some(<AccessorList as ::treesitter_types::FromNode>::from_node(
                    child, src,
                )?),
                None => None,
            },
            name: {
                let child = node
                    .child_by_field_name("name")
                    .ok_or_else(|| ::treesitter_types::ParseError::missing_field("name", node))?;
                <Identifier as ::treesitter_types::FromNode>::from_node(child, src)?
            },
            r#type: {
                let child = node
                    .child_by_field_name("type")
                    .ok_or_else(|| ::treesitter_types::ParseError::missing_field("type", node))?;
                <Type as ::treesitter_types::FromNode>::from_node(child, src)?
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
                        <EventDeclarationChildren as ::treesitter_types::FromNode>::from_node(
                            child, src,
                        )?,
                    );
                }
                items
            },
        })
    }
}
impl ::treesitter_types::Spanned for EventDeclaration<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        self.span
    }
}
#[derive(Debug, Clone)]
pub struct EventFieldDeclaration<'tree> {
    pub span: ::treesitter_types::Span,
    pub children: ::std::vec::Vec<EventFieldDeclarationChildren<'tree>>,
}
impl<'tree> ::treesitter_types::FromNode<'tree> for EventFieldDeclaration<'tree> {
    #[allow(clippy::match_single_binding, clippy::suspicious_else_formatting)]
    fn from_node(
        node: ::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        debug_assert_eq!(node.kind(), "event_field_declaration");
        Ok(Self {
            span: ::treesitter_types::Span::from(node),
            children: {
                #[allow(clippy::suspicious_else_formatting)]
                let non_field_children = {
                    let mut cursor = node.walk();
                    let mut result = ::std::vec::Vec::new();
                    if cursor.goto_first_child() {
                        loop {
                            if cursor.field_name().is_none()
                                && cursor.node().is_named()
                                && !cursor.node().is_extra()
                            {
                                result.push(cursor.node());
                            }
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
                        <EventFieldDeclarationChildren as ::treesitter_types::FromNode>::from_node(
                            child, src,
                        )?,
                    );
                }
                items
            },
        })
    }
}
impl ::treesitter_types::Spanned for EventFieldDeclaration<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        self.span
    }
}
#[derive(Debug, Clone)]
pub struct ExplicitInterfaceSpecifier<'tree> {
    pub span: ::treesitter_types::Span,
    pub children: ExplicitInterfaceSpecifierChildren<'tree>,
}
impl<'tree> ::treesitter_types::FromNode<'tree> for ExplicitInterfaceSpecifier<'tree> {
    #[allow(clippy::match_single_binding, clippy::suspicious_else_formatting)]
    fn from_node(
        node: ::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        debug_assert_eq!(node.kind(), "explicit_interface_specifier");
        Ok(Self {
            span: ::treesitter_types::Span::from(node),
            children: {
                #[allow(clippy::suspicious_else_formatting)]
                let non_field_children = {
                    let mut cursor = node.walk();
                    let mut result = ::std::vec::Vec::new();
                    if cursor.goto_first_child() {
                        loop {
                            if cursor.field_name().is_none()
                                && cursor.node().is_named()
                                && !cursor.node().is_extra()
                            {
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
                <ExplicitInterfaceSpecifierChildren as ::treesitter_types::FromNode>::from_node(
                    child, src,
                )?
            },
        })
    }
}
impl ::treesitter_types::Spanned for ExplicitInterfaceSpecifier<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        self.span
    }
}
#[derive(Debug, Clone)]
pub struct ExpressionStatement<'tree> {
    pub span: ::treesitter_types::Span,
    pub children: ExpressionStatementChildren<'tree>,
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
                <ExpressionStatementChildren as ::treesitter_types::FromNode>::from_node(
                    child, src,
                )?
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
pub struct ExternAliasDirective<'tree> {
    pub span: ::treesitter_types::Span,
    pub name: Identifier<'tree>,
}
impl<'tree> ::treesitter_types::FromNode<'tree> for ExternAliasDirective<'tree> {
    #[allow(clippy::match_single_binding, clippy::suspicious_else_formatting)]
    fn from_node(
        node: ::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        debug_assert_eq!(node.kind(), "extern_alias_directive");
        Ok(Self {
            span: ::treesitter_types::Span::from(node),
            name: {
                let child = node
                    .child_by_field_name("name")
                    .ok_or_else(|| ::treesitter_types::ParseError::missing_field("name", node))?;
                <Identifier as ::treesitter_types::FromNode>::from_node(child, src)?
            },
        })
    }
}
impl ::treesitter_types::Spanned for ExternAliasDirective<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        self.span
    }
}
#[derive(Debug, Clone)]
pub struct FieldDeclaration<'tree> {
    pub span: ::treesitter_types::Span,
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
            children: {
                #[allow(clippy::suspicious_else_formatting)]
                let non_field_children = {
                    let mut cursor = node.walk();
                    let mut result = ::std::vec::Vec::new();
                    if cursor.goto_first_child() {
                        loop {
                            if cursor.field_name().is_none()
                                && cursor.node().is_named()
                                && !cursor.node().is_extra()
                            {
                                result.push(cursor.node());
                            }
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
pub struct FileScopedNamespaceDeclaration<'tree> {
    pub span: ::treesitter_types::Span,
    pub name: FileScopedNamespaceDeclarationName<'tree>,
}
impl<'tree> ::treesitter_types::FromNode<'tree> for FileScopedNamespaceDeclaration<'tree> {
    #[allow(clippy::match_single_binding, clippy::suspicious_else_formatting)]
    fn from_node(
        node: ::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        debug_assert_eq!(node.kind(), "file_scoped_namespace_declaration");
        Ok(Self {
            span: ::treesitter_types::Span::from(node),
            name: {
                let child = node
                    .child_by_field_name("name")
                    .ok_or_else(|| ::treesitter_types::ParseError::missing_field("name", node))?;
                <FileScopedNamespaceDeclarationName as ::treesitter_types::FromNode>::from_node(
                    child, src,
                )?
            },
        })
    }
}
impl ::treesitter_types::Spanned for FileScopedNamespaceDeclaration<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        self.span
    }
}
#[derive(Debug, Clone)]
pub struct FinallyClause<'tree> {
    pub span: ::treesitter_types::Span,
    pub children: Block<'tree>,
}
impl<'tree> ::treesitter_types::FromNode<'tree> for FinallyClause<'tree> {
    #[allow(clippy::match_single_binding, clippy::suspicious_else_formatting)]
    fn from_node(
        node: ::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        debug_assert_eq!(node.kind(), "finally_clause");
        Ok(Self {
            span: ::treesitter_types::Span::from(node),
            children: {
                #[allow(clippy::suspicious_else_formatting)]
                let non_field_children = {
                    let mut cursor = node.walk();
                    let mut result = ::std::vec::Vec::new();
                    if cursor.goto_first_child() {
                        loop {
                            if cursor.field_name().is_none()
                                && cursor.node().is_named()
                                && !cursor.node().is_extra()
                            {
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
                <Block as ::treesitter_types::FromNode>::from_node(child, src)?
            },
        })
    }
}
impl ::treesitter_types::Spanned for FinallyClause<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        self.span
    }
}
#[derive(Debug, Clone)]
pub struct FixedStatement<'tree> {
    pub span: ::treesitter_types::Span,
    pub children: ::std::vec::Vec<FixedStatementChildren<'tree>>,
}
impl<'tree> ::treesitter_types::FromNode<'tree> for FixedStatement<'tree> {
    #[allow(clippy::match_single_binding, clippy::suspicious_else_formatting)]
    fn from_node(
        node: ::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        debug_assert_eq!(node.kind(), "fixed_statement");
        Ok(Self {
            span: ::treesitter_types::Span::from(node),
            children: {
                #[allow(clippy::suspicious_else_formatting)]
                let non_field_children = {
                    let mut cursor = node.walk();
                    let mut result = ::std::vec::Vec::new();
                    if cursor.goto_first_child() {
                        loop {
                            if cursor.field_name().is_none()
                                && cursor.node().is_named()
                                && !cursor.node().is_extra()
                            {
                                result.push(cursor.node());
                            }
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
                        <FixedStatementChildren as ::treesitter_types::FromNode>::from_node(
                            child, src,
                        )?,
                    );
                }
                items
            },
        })
    }
}
impl ::treesitter_types::Spanned for FixedStatement<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        self.span
    }
}
#[derive(Debug, Clone)]
pub struct ForStatement<'tree> {
    pub span: ::treesitter_types::Span,
    pub body: Statement<'tree>,
    pub condition: ::core::option::Option<Expression<'tree>>,
    pub initializer: ::std::vec::Vec<ForStatementInitializer<'tree>>,
    pub update: ::std::vec::Vec<ForStatementUpdate<'tree>>,
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
                Some(child) => Some(<Expression as ::treesitter_types::FromNode>::from_node(
                    child, src,
                )?),
                None => None,
            },
            initializer: {
                let mut cursor = node.walk();
                let mut items = ::std::vec::Vec::new();
                for child in node.children_by_field_name("initializer", &mut cursor) {
                    items.push(
                        <ForStatementInitializer as ::treesitter_types::FromNode>::from_node(
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
                        <ForStatementUpdate as ::treesitter_types::FromNode>::from_node(
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
pub struct ForeachStatement<'tree> {
    pub span: ::treesitter_types::Span,
    pub body: Statement<'tree>,
    pub left: ForeachStatementLeft<'tree>,
    pub right: Expression<'tree>,
    pub r#type: ::core::option::Option<Type<'tree>>,
}
impl<'tree> ::treesitter_types::FromNode<'tree> for ForeachStatement<'tree> {
    #[allow(clippy::match_single_binding, clippy::suspicious_else_formatting)]
    fn from_node(
        node: ::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        debug_assert_eq!(node.kind(), "foreach_statement");
        Ok(Self {
            span: ::treesitter_types::Span::from(node),
            body: {
                let child = node
                    .child_by_field_name("body")
                    .ok_or_else(|| ::treesitter_types::ParseError::missing_field("body", node))?;
                <Statement as ::treesitter_types::FromNode>::from_node(child, src)?
            },
            left: {
                let child = node
                    .child_by_field_name("left")
                    .ok_or_else(|| ::treesitter_types::ParseError::missing_field("left", node))?;
                <ForeachStatementLeft as ::treesitter_types::FromNode>::from_node(child, src)?
            },
            right: {
                let child = node
                    .child_by_field_name("right")
                    .ok_or_else(|| ::treesitter_types::ParseError::missing_field("right", node))?;
                <Expression as ::treesitter_types::FromNode>::from_node(child, src)?
            },
            r#type: match node.child_by_field_name("type") {
                Some(child) => Some(<Type as ::treesitter_types::FromNode>::from_node(
                    child, src,
                )?),
                None => None,
            },
        })
    }
}
impl ::treesitter_types::Spanned for ForeachStatement<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        self.span
    }
}
#[derive(Debug, Clone)]
pub struct FromClause<'tree> {
    pub span: ::treesitter_types::Span,
    pub name: Identifier<'tree>,
    pub r#type: ::core::option::Option<Type<'tree>>,
    pub children: Expression<'tree>,
}
impl<'tree> ::treesitter_types::FromNode<'tree> for FromClause<'tree> {
    #[allow(clippy::match_single_binding, clippy::suspicious_else_formatting)]
    fn from_node(
        node: ::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        debug_assert_eq!(node.kind(), "from_clause");
        Ok(Self {
            span: ::treesitter_types::Span::from(node),
            name: {
                let child = node
                    .child_by_field_name("name")
                    .ok_or_else(|| ::treesitter_types::ParseError::missing_field("name", node))?;
                <Identifier as ::treesitter_types::FromNode>::from_node(child, src)?
            },
            r#type: match node.child_by_field_name("type") {
                Some(child) => Some(<Type as ::treesitter_types::FromNode>::from_node(
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
impl ::treesitter_types::Spanned for FromClause<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        self.span
    }
}
#[derive(Debug, Clone)]
pub struct FunctionPointerParameter<'tree> {
    pub span: ::treesitter_types::Span,
    pub r#type: FunctionPointerParameterType<'tree>,
}
impl<'tree> ::treesitter_types::FromNode<'tree> for FunctionPointerParameter<'tree> {
    #[allow(clippy::match_single_binding, clippy::suspicious_else_formatting)]
    fn from_node(
        node: ::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        debug_assert_eq!(node.kind(), "function_pointer_parameter");
        Ok(Self {
            span: ::treesitter_types::Span::from(node),
            r#type: {
                let child = node
                    .child_by_field_name("type")
                    .ok_or_else(|| ::treesitter_types::ParseError::missing_field("type", node))?;
                <FunctionPointerParameterType as ::treesitter_types::FromNode>::from_node(
                    child, src,
                )?
            },
        })
    }
}
impl ::treesitter_types::Spanned for FunctionPointerParameter<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        self.span
    }
}
#[derive(Debug, Clone)]
pub struct FunctionPointerType<'tree> {
    pub span: ::treesitter_types::Span,
    pub returns: Type<'tree>,
    pub children: ::std::vec::Vec<FunctionPointerTypeChildren<'tree>>,
}
impl<'tree> ::treesitter_types::FromNode<'tree> for FunctionPointerType<'tree> {
    #[allow(clippy::match_single_binding, clippy::suspicious_else_formatting)]
    fn from_node(
        node: ::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        debug_assert_eq!(node.kind(), "function_pointer_type");
        Ok(Self {
            span: ::treesitter_types::Span::from(node),
            returns: {
                let child = node.child_by_field_name("returns").ok_or_else(|| {
                    ::treesitter_types::ParseError::missing_field("returns", node)
                })?;
                <Type as ::treesitter_types::FromNode>::from_node(child, src)?
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
                        <FunctionPointerTypeChildren as ::treesitter_types::FromNode>::from_node(
                            child, src,
                        )?,
                    );
                }
                items
            },
        })
    }
}
impl ::treesitter_types::Spanned for FunctionPointerType<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        self.span
    }
}
#[derive(Debug, Clone)]
pub struct GenericName<'tree> {
    pub span: ::treesitter_types::Span,
    pub children: ::std::vec::Vec<GenericNameChildren<'tree>>,
}
impl<'tree> ::treesitter_types::FromNode<'tree> for GenericName<'tree> {
    #[allow(clippy::match_single_binding, clippy::suspicious_else_formatting)]
    fn from_node(
        node: ::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        debug_assert_eq!(node.kind(), "generic_name");
        Ok(Self {
            span: ::treesitter_types::Span::from(node),
            children: {
                #[allow(clippy::suspicious_else_formatting)]
                let non_field_children = {
                    let mut cursor = node.walk();
                    let mut result = ::std::vec::Vec::new();
                    if cursor.goto_first_child() {
                        loop {
                            if cursor.field_name().is_none()
                                && cursor.node().is_named()
                                && !cursor.node().is_extra()
                            {
                                result.push(cursor.node());
                            }
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
                        <GenericNameChildren as ::treesitter_types::FromNode>::from_node(
                            child, src,
                        )?,
                    );
                }
                items
            },
        })
    }
}
impl ::treesitter_types::Spanned for GenericName<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        self.span
    }
}
#[derive(Debug, Clone)]
pub struct GlobalAttribute<'tree> {
    pub span: ::treesitter_types::Span,
    pub children: ::std::vec::Vec<Attribute<'tree>>,
}
impl<'tree> ::treesitter_types::FromNode<'tree> for GlobalAttribute<'tree> {
    #[allow(clippy::match_single_binding, clippy::suspicious_else_formatting)]
    fn from_node(
        node: ::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        debug_assert_eq!(node.kind(), "global_attribute");
        Ok(Self {
            span: ::treesitter_types::Span::from(node),
            children: {
                #[allow(clippy::suspicious_else_formatting)]
                let non_field_children = {
                    let mut cursor = node.walk();
                    let mut result = ::std::vec::Vec::new();
                    if cursor.goto_first_child() {
                        loop {
                            if cursor.field_name().is_none()
                                && cursor.node().is_named()
                                && !cursor.node().is_extra()
                            {
                                result.push(cursor.node());
                            }
                            if !cursor.goto_next_sibling() {
                                break;
                            }
                        }
                    }
                    result
                };
                let mut items = ::std::vec::Vec::new();
                for child in non_field_children {
                    items.push(<Attribute as ::treesitter_types::FromNode>::from_node(
                        child, src,
                    )?);
                }
                items
            },
        })
    }
}
impl ::treesitter_types::Spanned for GlobalAttribute<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        self.span
    }
}
#[derive(Debug, Clone)]
pub struct GlobalStatement<'tree> {
    pub span: ::treesitter_types::Span,
    pub children: Statement<'tree>,
}
impl<'tree> ::treesitter_types::FromNode<'tree> for GlobalStatement<'tree> {
    #[allow(clippy::match_single_binding, clippy::suspicious_else_formatting)]
    fn from_node(
        node: ::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        debug_assert_eq!(node.kind(), "global_statement");
        Ok(Self {
            span: ::treesitter_types::Span::from(node),
            children: {
                #[allow(clippy::suspicious_else_formatting)]
                let non_field_children = {
                    let mut cursor = node.walk();
                    let mut result = ::std::vec::Vec::new();
                    if cursor.goto_first_child() {
                        loop {
                            if cursor.field_name().is_none()
                                && cursor.node().is_named()
                                && !cursor.node().is_extra()
                            {
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
impl ::treesitter_types::Spanned for GlobalStatement<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        self.span
    }
}
#[derive(Debug, Clone)]
pub struct GotoStatement<'tree> {
    pub span: ::treesitter_types::Span,
    pub children: ::core::option::Option<Expression<'tree>>,
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
impl ::treesitter_types::Spanned for GotoStatement<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        self.span
    }
}
#[derive(Debug, Clone)]
pub struct GroupClause<'tree> {
    pub span: ::treesitter_types::Span,
    pub children: ::std::vec::Vec<Expression<'tree>>,
}
impl<'tree> ::treesitter_types::FromNode<'tree> for GroupClause<'tree> {
    #[allow(clippy::match_single_binding, clippy::suspicious_else_formatting)]
    fn from_node(
        node: ::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        debug_assert_eq!(node.kind(), "group_clause");
        Ok(Self {
            span: ::treesitter_types::Span::from(node),
            children: {
                #[allow(clippy::suspicious_else_formatting)]
                let non_field_children = {
                    let mut cursor = node.walk();
                    let mut result = ::std::vec::Vec::new();
                    if cursor.goto_first_child() {
                        loop {
                            if cursor.field_name().is_none()
                                && cursor.node().is_named()
                                && !cursor.node().is_extra()
                            {
                                result.push(cursor.node());
                            }
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
impl ::treesitter_types::Spanned for GroupClause<'_> {
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
pub struct IfStatement<'tree> {
    pub span: ::treesitter_types::Span,
    pub alternative: ::core::option::Option<Statement<'tree>>,
    pub condition: Expression<'tree>,
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
                Some(child) => Some(<Statement as ::treesitter_types::FromNode>::from_node(
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
pub struct ImplicitArrayCreationExpression<'tree> {
    pub span: ::treesitter_types::Span,
    pub children: InitializerExpression<'tree>,
}
impl<'tree> ::treesitter_types::FromNode<'tree> for ImplicitArrayCreationExpression<'tree> {
    #[allow(clippy::match_single_binding, clippy::suspicious_else_formatting)]
    fn from_node(
        node: ::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        debug_assert_eq!(node.kind(), "implicit_array_creation_expression");
        Ok(Self {
            span: ::treesitter_types::Span::from(node),
            children: {
                #[allow(clippy::suspicious_else_formatting)]
                let non_field_children = {
                    let mut cursor = node.walk();
                    let mut result = ::std::vec::Vec::new();
                    if cursor.goto_first_child() {
                        loop {
                            if cursor.field_name().is_none()
                                && cursor.node().is_named()
                                && !cursor.node().is_extra()
                            {
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
                <InitializerExpression as ::treesitter_types::FromNode>::from_node(child, src)?
            },
        })
    }
}
impl ::treesitter_types::Spanned for ImplicitArrayCreationExpression<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        self.span
    }
}
#[derive(Debug, Clone)]
pub struct ImplicitObjectCreationExpression<'tree> {
    pub span: ::treesitter_types::Span,
    pub children: ::std::vec::Vec<ImplicitObjectCreationExpressionChildren<'tree>>,
}
impl<'tree> ::treesitter_types::FromNode<'tree> for ImplicitObjectCreationExpression<'tree> {
    #[allow(clippy::match_single_binding, clippy::suspicious_else_formatting)]
    fn from_node(
        node: ::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        debug_assert_eq!(node.kind(), "implicit_object_creation_expression");
        Ok(Self {
            span: ::treesitter_types::Span::from(node),
            children: {
                #[allow(clippy::suspicious_else_formatting)]
                let non_field_children = {
                    let mut cursor = node.walk();
                    let mut result = ::std::vec::Vec::new();
                    if cursor.goto_first_child() {
                        loop {
                            if cursor.field_name().is_none()
                                && cursor.node().is_named()
                                && !cursor.node().is_extra()
                            {
                                result.push(cursor.node());
                            }
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
                            <ImplicitObjectCreationExpressionChildren as ::treesitter_types::FromNode>::from_node(
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
impl ::treesitter_types::Spanned for ImplicitObjectCreationExpression<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        self.span
    }
}
#[derive(Debug, Clone)]
pub struct ImplicitParameter<'tree> {
    pub span: ::treesitter_types::Span,
    text: &'tree str,
}
impl<'tree> ::treesitter_types::FromNode<'tree> for ImplicitParameter<'tree> {
    fn from_node(
        node: ::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        debug_assert_eq!(node.kind(), "implicit_parameter");
        Ok(Self {
            span: ::treesitter_types::Span::from(node),
            text: node.utf8_text(src)?,
        })
    }
}
impl<'tree> ::treesitter_types::LeafNode<'tree> for ImplicitParameter<'tree> {
    fn text(&self) -> &'tree str {
        self.text
    }
}
impl ::treesitter_types::Spanned for ImplicitParameter<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        self.span
    }
}
#[derive(Debug, Clone)]
pub struct ImplicitStackallocExpression<'tree> {
    pub span: ::treesitter_types::Span,
    pub children: InitializerExpression<'tree>,
}
impl<'tree> ::treesitter_types::FromNode<'tree> for ImplicitStackallocExpression<'tree> {
    #[allow(clippy::match_single_binding, clippy::suspicious_else_formatting)]
    fn from_node(
        node: ::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        debug_assert_eq!(node.kind(), "implicit_stackalloc_expression");
        Ok(Self {
            span: ::treesitter_types::Span::from(node),
            children: {
                #[allow(clippy::suspicious_else_formatting)]
                let non_field_children = {
                    let mut cursor = node.walk();
                    let mut result = ::std::vec::Vec::new();
                    if cursor.goto_first_child() {
                        loop {
                            if cursor.field_name().is_none()
                                && cursor.node().is_named()
                                && !cursor.node().is_extra()
                            {
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
                <InitializerExpression as ::treesitter_types::FromNode>::from_node(child, src)?
            },
        })
    }
}
impl ::treesitter_types::Spanned for ImplicitStackallocExpression<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        self.span
    }
}
#[derive(Debug, Clone)]
pub struct ImplicitType<'tree> {
    pub span: ::treesitter_types::Span,
    text: &'tree str,
}
impl<'tree> ::treesitter_types::FromNode<'tree> for ImplicitType<'tree> {
    fn from_node(
        node: ::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        debug_assert_eq!(node.kind(), "implicit_type");
        Ok(Self {
            span: ::treesitter_types::Span::from(node),
            text: node.utf8_text(src)?,
        })
    }
}
impl<'tree> ::treesitter_types::LeafNode<'tree> for ImplicitType<'tree> {
    fn text(&self) -> &'tree str {
        self.text
    }
}
impl ::treesitter_types::Spanned for ImplicitType<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        self.span
    }
}
#[derive(Debug, Clone)]
pub struct IndexerDeclaration<'tree> {
    pub span: ::treesitter_types::Span,
    pub accessors: ::core::option::Option<AccessorList<'tree>>,
    pub parameters: BracketedParameterList<'tree>,
    pub r#type: Type<'tree>,
    pub value: ::core::option::Option<ArrowExpressionClause<'tree>>,
    pub children: ::std::vec::Vec<IndexerDeclarationChildren<'tree>>,
}
impl<'tree> ::treesitter_types::FromNode<'tree> for IndexerDeclaration<'tree> {
    #[allow(clippy::match_single_binding, clippy::suspicious_else_formatting)]
    fn from_node(
        node: ::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        debug_assert_eq!(node.kind(), "indexer_declaration");
        Ok(Self {
            span: ::treesitter_types::Span::from(node),
            accessors: match node.child_by_field_name("accessors") {
                Some(child) => Some(<AccessorList as ::treesitter_types::FromNode>::from_node(
                    child, src,
                )?),
                None => None,
            },
            parameters: {
                let child = node.child_by_field_name("parameters").ok_or_else(|| {
                    ::treesitter_types::ParseError::missing_field("parameters", node)
                })?;
                <BracketedParameterList as ::treesitter_types::FromNode>::from_node(child, src)?
            },
            r#type: {
                let child = node
                    .child_by_field_name("type")
                    .ok_or_else(|| ::treesitter_types::ParseError::missing_field("type", node))?;
                <Type as ::treesitter_types::FromNode>::from_node(child, src)?
            },
            value: match node.child_by_field_name("value") {
                Some(child) => Some(
                    <ArrowExpressionClause as ::treesitter_types::FromNode>::from_node(child, src)?,
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
                        <IndexerDeclarationChildren as ::treesitter_types::FromNode>::from_node(
                            child, src,
                        )?,
                    );
                }
                items
            },
        })
    }
}
impl ::treesitter_types::Spanned for IndexerDeclaration<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        self.span
    }
}
#[derive(Debug, Clone)]
pub struct InitializerExpression<'tree> {
    pub span: ::treesitter_types::Span,
    pub children: ::std::vec::Vec<Expression<'tree>>,
}
impl<'tree> ::treesitter_types::FromNode<'tree> for InitializerExpression<'tree> {
    #[allow(clippy::match_single_binding, clippy::suspicious_else_formatting)]
    fn from_node(
        node: ::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        debug_assert_eq!(node.kind(), "initializer_expression");
        Ok(Self {
            span: ::treesitter_types::Span::from(node),
            children: {
                #[allow(clippy::suspicious_else_formatting)]
                let non_field_children = {
                    let mut cursor = node.walk();
                    let mut result = ::std::vec::Vec::new();
                    if cursor.goto_first_child() {
                        loop {
                            if cursor.field_name().is_none()
                                && cursor.node().is_named()
                                && !cursor.node().is_extra()
                            {
                                result.push(cursor.node());
                            }
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
impl ::treesitter_types::Spanned for InitializerExpression<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        self.span
    }
}
#[derive(Debug, Clone)]
pub struct InterfaceDeclaration<'tree> {
    pub span: ::treesitter_types::Span,
    pub body: ::core::option::Option<DeclarationList<'tree>>,
    pub name: Identifier<'tree>,
    pub type_parameters: ::core::option::Option<TypeParameterList<'tree>>,
    pub children: ::std::vec::Vec<InterfaceDeclarationChildren<'tree>>,
}
impl<'tree> ::treesitter_types::FromNode<'tree> for InterfaceDeclaration<'tree> {
    #[allow(clippy::match_single_binding, clippy::suspicious_else_formatting)]
    fn from_node(
        node: ::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        debug_assert_eq!(node.kind(), "interface_declaration");
        Ok(Self {
            span: ::treesitter_types::Span::from(node),
            body: match node.child_by_field_name("body") {
                Some(child) => {
                    Some(<DeclarationList as ::treesitter_types::FromNode>::from_node(child, src)?)
                }
                None => None,
            },
            name: {
                let child = node
                    .child_by_field_name("name")
                    .ok_or_else(|| ::treesitter_types::ParseError::missing_field("name", node))?;
                <Identifier as ::treesitter_types::FromNode>::from_node(child, src)?
            },
            type_parameters: match node.child_by_field_name("type_parameters") {
                Some(child) => Some(
                    <TypeParameterList as ::treesitter_types::FromNode>::from_node(child, src)?,
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
                        <InterfaceDeclarationChildren as ::treesitter_types::FromNode>::from_node(
                            child, src,
                        )?,
                    );
                }
                items
            },
        })
    }
}
impl ::treesitter_types::Spanned for InterfaceDeclaration<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        self.span
    }
}
#[derive(Debug, Clone)]
pub struct InterpolatedStringExpression<'tree> {
    pub span: ::treesitter_types::Span,
    pub children: ::std::vec::Vec<InterpolatedStringExpressionChildren<'tree>>,
}
impl<'tree> ::treesitter_types::FromNode<'tree> for InterpolatedStringExpression<'tree> {
    #[allow(clippy::match_single_binding, clippy::suspicious_else_formatting)]
    fn from_node(
        node: ::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        debug_assert_eq!(node.kind(), "interpolated_string_expression");
        Ok(Self {
            span: ::treesitter_types::Span::from(node),
            children: {
                #[allow(clippy::suspicious_else_formatting)]
                let non_field_children = {
                    let mut cursor = node.walk();
                    let mut result = ::std::vec::Vec::new();
                    if cursor.goto_first_child() {
                        loop {
                            if cursor.field_name().is_none()
                                && cursor.node().is_named()
                                && !cursor.node().is_extra()
                            {
                                result.push(cursor.node());
                            }
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
                            <InterpolatedStringExpressionChildren as ::treesitter_types::FromNode>::from_node(
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
impl ::treesitter_types::Spanned for InterpolatedStringExpression<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        self.span
    }
}
#[derive(Debug, Clone)]
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
                    items.push(
                        <InterpolationChildren as ::treesitter_types::FromNode>::from_node(
                            child, src,
                        )?,
                    );
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
#[derive(Debug, Clone)]
pub struct InterpolationAlignmentClause<'tree> {
    pub span: ::treesitter_types::Span,
    pub children: Expression<'tree>,
}
impl<'tree> ::treesitter_types::FromNode<'tree> for InterpolationAlignmentClause<'tree> {
    #[allow(clippy::match_single_binding, clippy::suspicious_else_formatting)]
    fn from_node(
        node: ::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        debug_assert_eq!(node.kind(), "interpolation_alignment_clause");
        Ok(Self {
            span: ::treesitter_types::Span::from(node),
            children: {
                #[allow(clippy::suspicious_else_formatting)]
                let non_field_children = {
                    let mut cursor = node.walk();
                    let mut result = ::std::vec::Vec::new();
                    if cursor.goto_first_child() {
                        loop {
                            if cursor.field_name().is_none()
                                && cursor.node().is_named()
                                && !cursor.node().is_extra()
                            {
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
impl ::treesitter_types::Spanned for InterpolationAlignmentClause<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        self.span
    }
}
#[derive(Debug, Clone)]
pub struct InterpolationFormatClause<'tree> {
    pub span: ::treesitter_types::Span,
    text: &'tree str,
}
impl<'tree> ::treesitter_types::FromNode<'tree> for InterpolationFormatClause<'tree> {
    fn from_node(
        node: ::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        debug_assert_eq!(node.kind(), "interpolation_format_clause");
        Ok(Self {
            span: ::treesitter_types::Span::from(node),
            text: node.utf8_text(src)?,
        })
    }
}
impl<'tree> ::treesitter_types::LeafNode<'tree> for InterpolationFormatClause<'tree> {
    fn text(&self) -> &'tree str {
        self.text
    }
}
impl ::treesitter_types::Spanned for InterpolationFormatClause<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        self.span
    }
}
#[derive(Debug, Clone)]
pub struct InvocationExpression<'tree> {
    pub span: ::treesitter_types::Span,
    pub arguments: ArgumentList<'tree>,
    pub function: InvocationExpressionFunction<'tree>,
}
impl<'tree> ::treesitter_types::FromNode<'tree> for InvocationExpression<'tree> {
    #[allow(clippy::match_single_binding, clippy::suspicious_else_formatting)]
    fn from_node(
        node: ::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        debug_assert_eq!(node.kind(), "invocation_expression");
        Ok(Self {
            span: ::treesitter_types::Span::from(node),
            arguments: {
                let child = node.child_by_field_name("arguments").ok_or_else(|| {
                    ::treesitter_types::ParseError::missing_field("arguments", node)
                })?;
                <ArgumentList as ::treesitter_types::FromNode>::from_node(child, src)?
            },
            function: {
                let child = node.child_by_field_name("function").ok_or_else(|| {
                    ::treesitter_types::ParseError::missing_field("function", node)
                })?;
                <InvocationExpressionFunction as ::treesitter_types::FromNode>::from_node(
                    child, src,
                )?
            },
        })
    }
}
impl ::treesitter_types::Spanned for InvocationExpression<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        self.span
    }
}
#[derive(Debug, Clone)]
pub struct IsExpression<'tree> {
    pub span: ::treesitter_types::Span,
    pub left: Expression<'tree>,
    pub operator: IsExpressionOperator,
    pub right: Type<'tree>,
}
impl<'tree> ::treesitter_types::FromNode<'tree> for IsExpression<'tree> {
    #[allow(clippy::match_single_binding, clippy::suspicious_else_formatting)]
    fn from_node(
        node: ::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        debug_assert_eq!(node.kind(), "is_expression");
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
                <IsExpressionOperator as ::treesitter_types::FromNode>::from_node(child, src)?
            },
            right: {
                let child = node
                    .child_by_field_name("right")
                    .ok_or_else(|| ::treesitter_types::ParseError::missing_field("right", node))?;
                <Type as ::treesitter_types::FromNode>::from_node(child, src)?
            },
        })
    }
}
impl ::treesitter_types::Spanned for IsExpression<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        self.span
    }
}
#[derive(Debug, Clone)]
pub struct IsPatternExpression<'tree> {
    pub span: ::treesitter_types::Span,
    pub expression: Expression<'tree>,
    pub pattern: Pattern<'tree>,
}
impl<'tree> ::treesitter_types::FromNode<'tree> for IsPatternExpression<'tree> {
    #[allow(clippy::match_single_binding, clippy::suspicious_else_formatting)]
    fn from_node(
        node: ::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        debug_assert_eq!(node.kind(), "is_pattern_expression");
        Ok(Self {
            span: ::treesitter_types::Span::from(node),
            expression: {
                let child = node.child_by_field_name("expression").ok_or_else(|| {
                    ::treesitter_types::ParseError::missing_field("expression", node)
                })?;
                <Expression as ::treesitter_types::FromNode>::from_node(child, src)?
            },
            pattern: {
                let child = node.child_by_field_name("pattern").ok_or_else(|| {
                    ::treesitter_types::ParseError::missing_field("pattern", node)
                })?;
                <Pattern as ::treesitter_types::FromNode>::from_node(child, src)?
            },
        })
    }
}
impl ::treesitter_types::Spanned for IsPatternExpression<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        self.span
    }
}
#[derive(Debug, Clone)]
pub struct JoinClause<'tree> {
    pub span: ::treesitter_types::Span,
    pub r#type: ::core::option::Option<Type<'tree>>,
    pub children: ::std::vec::Vec<JoinClauseChildren<'tree>>,
}
impl<'tree> ::treesitter_types::FromNode<'tree> for JoinClause<'tree> {
    #[allow(clippy::match_single_binding, clippy::suspicious_else_formatting)]
    fn from_node(
        node: ::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        debug_assert_eq!(node.kind(), "join_clause");
        Ok(Self {
            span: ::treesitter_types::Span::from(node),
            r#type: match node.child_by_field_name("type") {
                Some(child) => Some(<Type as ::treesitter_types::FromNode>::from_node(
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
                        <JoinClauseChildren as ::treesitter_types::FromNode>::from_node(
                            child, src,
                        )?,
                    );
                }
                items
            },
        })
    }
}
impl ::treesitter_types::Spanned for JoinClause<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        self.span
    }
}
#[derive(Debug, Clone)]
pub struct JoinIntoClause<'tree> {
    pub span: ::treesitter_types::Span,
    pub children: Identifier<'tree>,
}
impl<'tree> ::treesitter_types::FromNode<'tree> for JoinIntoClause<'tree> {
    #[allow(clippy::match_single_binding, clippy::suspicious_else_formatting)]
    fn from_node(
        node: ::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        debug_assert_eq!(node.kind(), "join_into_clause");
        Ok(Self {
            span: ::treesitter_types::Span::from(node),
            children: {
                #[allow(clippy::suspicious_else_formatting)]
                let non_field_children = {
                    let mut cursor = node.walk();
                    let mut result = ::std::vec::Vec::new();
                    if cursor.goto_first_child() {
                        loop {
                            if cursor.field_name().is_none()
                                && cursor.node().is_named()
                                && !cursor.node().is_extra()
                            {
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
impl ::treesitter_types::Spanned for JoinIntoClause<'_> {
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
pub struct LambdaExpression<'tree> {
    pub span: ::treesitter_types::Span,
    pub body: LambdaExpressionBody<'tree>,
    pub parameters: LambdaExpressionParameters<'tree>,
    pub r#type: ::core::option::Option<Type<'tree>>,
    pub children: ::std::vec::Vec<LambdaExpressionChildren<'tree>>,
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
                <LambdaExpressionBody as ::treesitter_types::FromNode>::from_node(child, src)?
            },
            parameters: {
                let child = node.child_by_field_name("parameters").ok_or_else(|| {
                    ::treesitter_types::ParseError::missing_field("parameters", node)
                })?;
                <LambdaExpressionParameters as ::treesitter_types::FromNode>::from_node(child, src)?
            },
            r#type: match node.child_by_field_name("type") {
                Some(child) => Some(<Type as ::treesitter_types::FromNode>::from_node(
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
                        <LambdaExpressionChildren as ::treesitter_types::FromNode>::from_node(
                            child, src,
                        )?,
                    );
                }
                items
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
pub struct LetClause<'tree> {
    pub span: ::treesitter_types::Span,
    pub children: ::std::vec::Vec<LetClauseChildren<'tree>>,
}
impl<'tree> ::treesitter_types::FromNode<'tree> for LetClause<'tree> {
    #[allow(clippy::match_single_binding, clippy::suspicious_else_formatting)]
    fn from_node(
        node: ::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        debug_assert_eq!(node.kind(), "let_clause");
        Ok(Self {
            span: ::treesitter_types::Span::from(node),
            children: {
                #[allow(clippy::suspicious_else_formatting)]
                let non_field_children = {
                    let mut cursor = node.walk();
                    let mut result = ::std::vec::Vec::new();
                    if cursor.goto_first_child() {
                        loop {
                            if cursor.field_name().is_none()
                                && cursor.node().is_named()
                                && !cursor.node().is_extra()
                            {
                                result.push(cursor.node());
                            }
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
                        <LetClauseChildren as ::treesitter_types::FromNode>::from_node(child, src)?,
                    );
                }
                items
            },
        })
    }
}
impl ::treesitter_types::Spanned for LetClause<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        self.span
    }
}
#[derive(Debug, Clone)]
pub struct ListPattern<'tree> {
    pub span: ::treesitter_types::Span,
    pub name: ::core::option::Option<Identifier<'tree>>,
    pub children: ::std::vec::Vec<ListPatternChildren<'tree>>,
}
impl<'tree> ::treesitter_types::FromNode<'tree> for ListPattern<'tree> {
    #[allow(clippy::match_single_binding, clippy::suspicious_else_formatting)]
    fn from_node(
        node: ::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        debug_assert_eq!(node.kind(), "list_pattern");
        Ok(Self {
            span: ::treesitter_types::Span::from(node),
            name: match node.child_by_field_name("name") {
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
                let mut items = ::std::vec::Vec::new();
                for child in non_field_children {
                    items.push(
                        <ListPatternChildren as ::treesitter_types::FromNode>::from_node(
                            child, src,
                        )?,
                    );
                }
                items
            },
        })
    }
}
impl ::treesitter_types::Spanned for ListPattern<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        self.span
    }
}
#[derive(Debug, Clone)]
pub struct LocalDeclarationStatement<'tree> {
    pub span: ::treesitter_types::Span,
    pub children: ::std::vec::Vec<LocalDeclarationStatementChildren<'tree>>,
}
impl<'tree> ::treesitter_types::FromNode<'tree> for LocalDeclarationStatement<'tree> {
    #[allow(clippy::match_single_binding, clippy::suspicious_else_formatting)]
    fn from_node(
        node: ::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        debug_assert_eq!(node.kind(), "local_declaration_statement");
        Ok(Self {
            span: ::treesitter_types::Span::from(node),
            children: {
                #[allow(clippy::suspicious_else_formatting)]
                let non_field_children = {
                    let mut cursor = node.walk();
                    let mut result = ::std::vec::Vec::new();
                    if cursor.goto_first_child() {
                        loop {
                            if cursor.field_name().is_none()
                                && cursor.node().is_named()
                                && !cursor.node().is_extra()
                            {
                                result.push(cursor.node());
                            }
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
                            <LocalDeclarationStatementChildren as ::treesitter_types::FromNode>::from_node(
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
impl ::treesitter_types::Spanned for LocalDeclarationStatement<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        self.span
    }
}
#[derive(Debug, Clone)]
pub struct LocalFunctionStatement<'tree> {
    pub span: ::treesitter_types::Span,
    pub body: ::core::option::Option<LocalFunctionStatementBody<'tree>>,
    pub name: Identifier<'tree>,
    pub parameters: ParameterList<'tree>,
    pub r#type: Type<'tree>,
    pub type_parameters: ::core::option::Option<TypeParameterList<'tree>>,
    pub children: ::std::vec::Vec<LocalFunctionStatementChildren<'tree>>,
}
impl<'tree> ::treesitter_types::FromNode<'tree> for LocalFunctionStatement<'tree> {
    #[allow(clippy::match_single_binding, clippy::suspicious_else_formatting)]
    fn from_node(
        node: ::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        debug_assert_eq!(node.kind(), "local_function_statement");
        Ok(Self {
            span: ::treesitter_types::Span::from(node),
            body: match node.child_by_field_name("body") {
                Some(child) => Some(
                    <LocalFunctionStatementBody as ::treesitter_types::FromNode>::from_node(
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
            parameters: {
                let child = node.child_by_field_name("parameters").ok_or_else(|| {
                    ::treesitter_types::ParseError::missing_field("parameters", node)
                })?;
                <ParameterList as ::treesitter_types::FromNode>::from_node(child, src)?
            },
            r#type: {
                let child = node
                    .child_by_field_name("type")
                    .ok_or_else(|| ::treesitter_types::ParseError::missing_field("type", node))?;
                <Type as ::treesitter_types::FromNode>::from_node(child, src)?
            },
            type_parameters: match node.child_by_field_name("type_parameters") {
                Some(child) => Some(
                    <TypeParameterList as ::treesitter_types::FromNode>::from_node(child, src)?,
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
                            <LocalFunctionStatementChildren as ::treesitter_types::FromNode>::from_node(
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
impl ::treesitter_types::Spanned for LocalFunctionStatement<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        self.span
    }
}
#[derive(Debug, Clone)]
pub struct LockStatement<'tree> {
    pub span: ::treesitter_types::Span,
    pub children: ::std::vec::Vec<LockStatementChildren<'tree>>,
}
impl<'tree> ::treesitter_types::FromNode<'tree> for LockStatement<'tree> {
    #[allow(clippy::match_single_binding, clippy::suspicious_else_formatting)]
    fn from_node(
        node: ::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        debug_assert_eq!(node.kind(), "lock_statement");
        Ok(Self {
            span: ::treesitter_types::Span::from(node),
            children: {
                #[allow(clippy::suspicious_else_formatting)]
                let non_field_children = {
                    let mut cursor = node.walk();
                    let mut result = ::std::vec::Vec::new();
                    if cursor.goto_first_child() {
                        loop {
                            if cursor.field_name().is_none()
                                && cursor.node().is_named()
                                && !cursor.node().is_extra()
                            {
                                result.push(cursor.node());
                            }
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
                        <LockStatementChildren as ::treesitter_types::FromNode>::from_node(
                            child, src,
                        )?,
                    );
                }
                items
            },
        })
    }
}
impl ::treesitter_types::Spanned for LockStatement<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        self.span
    }
}
#[derive(Debug, Clone)]
pub struct MakerefExpression<'tree> {
    pub span: ::treesitter_types::Span,
    pub children: Expression<'tree>,
}
impl<'tree> ::treesitter_types::FromNode<'tree> for MakerefExpression<'tree> {
    #[allow(clippy::match_single_binding, clippy::suspicious_else_formatting)]
    fn from_node(
        node: ::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        debug_assert_eq!(node.kind(), "makeref_expression");
        Ok(Self {
            span: ::treesitter_types::Span::from(node),
            children: {
                #[allow(clippy::suspicious_else_formatting)]
                let non_field_children = {
                    let mut cursor = node.walk();
                    let mut result = ::std::vec::Vec::new();
                    if cursor.goto_first_child() {
                        loop {
                            if cursor.field_name().is_none()
                                && cursor.node().is_named()
                                && !cursor.node().is_extra()
                            {
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
impl ::treesitter_types::Spanned for MakerefExpression<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        self.span
    }
}
#[derive(Debug, Clone)]
pub struct MemberAccessExpression<'tree> {
    pub span: ::treesitter_types::Span,
    pub expression: MemberAccessExpressionExpression<'tree>,
    pub name: MemberAccessExpressionName<'tree>,
}
impl<'tree> ::treesitter_types::FromNode<'tree> for MemberAccessExpression<'tree> {
    #[allow(clippy::match_single_binding, clippy::suspicious_else_formatting)]
    fn from_node(
        node: ::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        debug_assert_eq!(node.kind(), "member_access_expression");
        Ok(Self {
            span: ::treesitter_types::Span::from(node),
            expression: {
                let child = node.child_by_field_name("expression").ok_or_else(|| {
                    ::treesitter_types::ParseError::missing_field("expression", node)
                })?;
                <MemberAccessExpressionExpression as ::treesitter_types::FromNode>::from_node(
                    child, src,
                )?
            },
            name: {
                let child = node
                    .child_by_field_name("name")
                    .ok_or_else(|| ::treesitter_types::ParseError::missing_field("name", node))?;
                <MemberAccessExpressionName as ::treesitter_types::FromNode>::from_node(child, src)?
            },
        })
    }
}
impl ::treesitter_types::Spanned for MemberAccessExpression<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        self.span
    }
}
#[derive(Debug, Clone)]
pub struct MemberBindingExpression<'tree> {
    pub span: ::treesitter_types::Span,
    pub name: MemberBindingExpressionName<'tree>,
}
impl<'tree> ::treesitter_types::FromNode<'tree> for MemberBindingExpression<'tree> {
    #[allow(clippy::match_single_binding, clippy::suspicious_else_formatting)]
    fn from_node(
        node: ::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        debug_assert_eq!(node.kind(), "member_binding_expression");
        Ok(Self {
            span: ::treesitter_types::Span::from(node),
            name: {
                let child = node
                    .child_by_field_name("name")
                    .ok_or_else(|| ::treesitter_types::ParseError::missing_field("name", node))?;
                <MemberBindingExpressionName as ::treesitter_types::FromNode>::from_node(
                    child, src,
                )?
            },
        })
    }
}
impl ::treesitter_types::Spanned for MemberBindingExpression<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        self.span
    }
}
#[derive(Debug, Clone)]
pub struct MethodDeclaration<'tree> {
    pub span: ::treesitter_types::Span,
    pub body: ::core::option::Option<MethodDeclarationBody<'tree>>,
    pub name: Identifier<'tree>,
    pub parameters: ParameterList<'tree>,
    pub returns: Type<'tree>,
    pub type_parameters: ::core::option::Option<TypeParameterList<'tree>>,
    pub children: ::std::vec::Vec<MethodDeclarationChildren<'tree>>,
}
impl<'tree> ::treesitter_types::FromNode<'tree> for MethodDeclaration<'tree> {
    #[allow(clippy::match_single_binding, clippy::suspicious_else_formatting)]
    fn from_node(
        node: ::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        debug_assert_eq!(node.kind(), "method_declaration");
        Ok(Self {
            span: ::treesitter_types::Span::from(node),
            body: match node.child_by_field_name("body") {
                Some(child) => Some(
                    <MethodDeclarationBody as ::treesitter_types::FromNode>::from_node(child, src)?,
                ),
                None => None,
            },
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
                <ParameterList as ::treesitter_types::FromNode>::from_node(child, src)?
            },
            returns: {
                let child = node.child_by_field_name("returns").ok_or_else(|| {
                    ::treesitter_types::ParseError::missing_field("returns", node)
                })?;
                <Type as ::treesitter_types::FromNode>::from_node(child, src)?
            },
            type_parameters: match node.child_by_field_name("type_parameters") {
                Some(child) => Some(
                    <TypeParameterList as ::treesitter_types::FromNode>::from_node(child, src)?,
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
                        <MethodDeclarationChildren as ::treesitter_types::FromNode>::from_node(
                            child, src,
                        )?,
                    );
                }
                items
            },
        })
    }
}
impl ::treesitter_types::Spanned for MethodDeclaration<'_> {
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
pub struct NamespaceDeclaration<'tree> {
    pub span: ::treesitter_types::Span,
    pub body: DeclarationList<'tree>,
    pub name: NamespaceDeclarationName<'tree>,
}
impl<'tree> ::treesitter_types::FromNode<'tree> for NamespaceDeclaration<'tree> {
    #[allow(clippy::match_single_binding, clippy::suspicious_else_formatting)]
    fn from_node(
        node: ::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        debug_assert_eq!(node.kind(), "namespace_declaration");
        Ok(Self {
            span: ::treesitter_types::Span::from(node),
            body: {
                let child = node
                    .child_by_field_name("body")
                    .ok_or_else(|| ::treesitter_types::ParseError::missing_field("body", node))?;
                <DeclarationList as ::treesitter_types::FromNode>::from_node(child, src)?
            },
            name: {
                let child = node
                    .child_by_field_name("name")
                    .ok_or_else(|| ::treesitter_types::ParseError::missing_field("name", node))?;
                <NamespaceDeclarationName as ::treesitter_types::FromNode>::from_node(child, src)?
            },
        })
    }
}
impl ::treesitter_types::Spanned for NamespaceDeclaration<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        self.span
    }
}
#[derive(Debug, Clone)]
pub struct NegatedPattern<'tree> {
    pub span: ::treesitter_types::Span,
    pub children: Pattern<'tree>,
}
impl<'tree> ::treesitter_types::FromNode<'tree> for NegatedPattern<'tree> {
    #[allow(clippy::match_single_binding, clippy::suspicious_else_formatting)]
    fn from_node(
        node: ::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        debug_assert_eq!(node.kind(), "negated_pattern");
        Ok(Self {
            span: ::treesitter_types::Span::from(node),
            children: {
                #[allow(clippy::suspicious_else_formatting)]
                let non_field_children = {
                    let mut cursor = node.walk();
                    let mut result = ::std::vec::Vec::new();
                    if cursor.goto_first_child() {
                        loop {
                            if cursor.field_name().is_none()
                                && cursor.node().is_named()
                                && !cursor.node().is_extra()
                            {
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
                <Pattern as ::treesitter_types::FromNode>::from_node(child, src)?
            },
        })
    }
}
impl ::treesitter_types::Spanned for NegatedPattern<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        self.span
    }
}
#[derive(Debug, Clone)]
pub struct NullableType<'tree> {
    pub span: ::treesitter_types::Span,
    pub r#type: NullableTypeType<'tree>,
}
impl<'tree> ::treesitter_types::FromNode<'tree> for NullableType<'tree> {
    #[allow(clippy::match_single_binding, clippy::suspicious_else_formatting)]
    fn from_node(
        node: ::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        debug_assert_eq!(node.kind(), "nullable_type");
        Ok(Self {
            span: ::treesitter_types::Span::from(node),
            r#type: {
                let child = node
                    .child_by_field_name("type")
                    .ok_or_else(|| ::treesitter_types::ParseError::missing_field("type", node))?;
                <NullableTypeType as ::treesitter_types::FromNode>::from_node(child, src)?
            },
        })
    }
}
impl ::treesitter_types::Spanned for NullableType<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        self.span
    }
}
#[derive(Debug, Clone)]
pub struct ObjectCreationExpression<'tree> {
    pub span: ::treesitter_types::Span,
    pub arguments: ::core::option::Option<ArgumentList<'tree>>,
    pub initializer: ::core::option::Option<InitializerExpression<'tree>>,
    pub r#type: Type<'tree>,
}
impl<'tree> ::treesitter_types::FromNode<'tree> for ObjectCreationExpression<'tree> {
    #[allow(clippy::match_single_binding, clippy::suspicious_else_formatting)]
    fn from_node(
        node: ::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        debug_assert_eq!(node.kind(), "object_creation_expression");
        Ok(Self {
            span: ::treesitter_types::Span::from(node),
            arguments: match node.child_by_field_name("arguments") {
                Some(child) => Some(<ArgumentList as ::treesitter_types::FromNode>::from_node(
                    child, src,
                )?),
                None => None,
            },
            initializer: match node.child_by_field_name("initializer") {
                Some(child) => Some(
                    <InitializerExpression as ::treesitter_types::FromNode>::from_node(child, src)?,
                ),
                None => None,
            },
            r#type: {
                let child = node
                    .child_by_field_name("type")
                    .ok_or_else(|| ::treesitter_types::ParseError::missing_field("type", node))?;
                <Type as ::treesitter_types::FromNode>::from_node(child, src)?
            },
        })
    }
}
impl ::treesitter_types::Spanned for ObjectCreationExpression<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        self.span
    }
}
#[derive(Debug, Clone)]
pub struct OperatorDeclaration<'tree> {
    pub span: ::treesitter_types::Span,
    pub body: ::core::option::Option<OperatorDeclarationBody<'tree>>,
    pub operator: OperatorDeclarationOperator,
    pub parameters: ParameterList<'tree>,
    pub r#type: Type<'tree>,
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
            body: match node.child_by_field_name("body") {
                Some(child) => Some(
                    <OperatorDeclarationBody as ::treesitter_types::FromNode>::from_node(
                        child, src,
                    )?,
                ),
                None => None,
            },
            operator: {
                let child = node.child_by_field_name("operator").ok_or_else(|| {
                    ::treesitter_types::ParseError::missing_field("operator", node)
                })?;
                <OperatorDeclarationOperator as ::treesitter_types::FromNode>::from_node(
                    child, src,
                )?
            },
            parameters: {
                let child = node.child_by_field_name("parameters").ok_or_else(|| {
                    ::treesitter_types::ParseError::missing_field("parameters", node)
                })?;
                <ParameterList as ::treesitter_types::FromNode>::from_node(child, src)?
            },
            r#type: {
                let child = node
                    .child_by_field_name("type")
                    .ok_or_else(|| ::treesitter_types::ParseError::missing_field("type", node))?;
                <Type as ::treesitter_types::FromNode>::from_node(child, src)?
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
pub struct OrPattern<'tree> {
    pub span: ::treesitter_types::Span,
    pub left: Pattern<'tree>,
    pub operator: OrPatternOperator,
    pub right: Pattern<'tree>,
}
impl<'tree> ::treesitter_types::FromNode<'tree> for OrPattern<'tree> {
    #[allow(clippy::match_single_binding, clippy::suspicious_else_formatting)]
    fn from_node(
        node: ::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        debug_assert_eq!(node.kind(), "or_pattern");
        Ok(Self {
            span: ::treesitter_types::Span::from(node),
            left: {
                let child = node
                    .child_by_field_name("left")
                    .ok_or_else(|| ::treesitter_types::ParseError::missing_field("left", node))?;
                <Pattern as ::treesitter_types::FromNode>::from_node(child, src)?
            },
            operator: {
                let child = node.child_by_field_name("operator").ok_or_else(|| {
                    ::treesitter_types::ParseError::missing_field("operator", node)
                })?;
                <OrPatternOperator as ::treesitter_types::FromNode>::from_node(child, src)?
            },
            right: {
                let child = node
                    .child_by_field_name("right")
                    .ok_or_else(|| ::treesitter_types::ParseError::missing_field("right", node))?;
                <Pattern as ::treesitter_types::FromNode>::from_node(child, src)?
            },
        })
    }
}
impl ::treesitter_types::Spanned for OrPattern<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        self.span
    }
}
#[derive(Debug, Clone)]
pub struct OrderByClause<'tree> {
    pub span: ::treesitter_types::Span,
    pub children: ::std::vec::Vec<Expression<'tree>>,
}
impl<'tree> ::treesitter_types::FromNode<'tree> for OrderByClause<'tree> {
    #[allow(clippy::match_single_binding, clippy::suspicious_else_formatting)]
    fn from_node(
        node: ::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        debug_assert_eq!(node.kind(), "order_by_clause");
        Ok(Self {
            span: ::treesitter_types::Span::from(node),
            children: {
                #[allow(clippy::suspicious_else_formatting)]
                let non_field_children = {
                    let mut cursor = node.walk();
                    let mut result = ::std::vec::Vec::new();
                    if cursor.goto_first_child() {
                        loop {
                            if cursor.field_name().is_none()
                                && cursor.node().is_named()
                                && !cursor.node().is_extra()
                            {
                                result.push(cursor.node());
                            }
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
impl ::treesitter_types::Spanned for OrderByClause<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        self.span
    }
}
#[derive(Debug, Clone)]
pub struct Parameter<'tree> {
    pub span: ::treesitter_types::Span,
    pub name: Identifier<'tree>,
    pub r#type: ::core::option::Option<Type<'tree>>,
    pub children: ::std::vec::Vec<ParameterChildren<'tree>>,
}
impl<'tree> ::treesitter_types::FromNode<'tree> for Parameter<'tree> {
    #[allow(clippy::match_single_binding, clippy::suspicious_else_formatting)]
    fn from_node(
        node: ::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        debug_assert_eq!(node.kind(), "parameter");
        Ok(Self {
            span: ::treesitter_types::Span::from(node),
            name: {
                let child = node
                    .child_by_field_name("name")
                    .ok_or_else(|| ::treesitter_types::ParseError::missing_field("name", node))?;
                <Identifier as ::treesitter_types::FromNode>::from_node(child, src)?
            },
            r#type: match node.child_by_field_name("type") {
                Some(child) => Some(<Type as ::treesitter_types::FromNode>::from_node(
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
                        <ParameterChildren as ::treesitter_types::FromNode>::from_node(child, src)?,
                    );
                }
                items
            },
        })
    }
}
impl ::treesitter_types::Spanned for Parameter<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        self.span
    }
}
#[derive(Debug, Clone)]
pub struct ParameterList<'tree> {
    pub span: ::treesitter_types::Span,
    pub name: ::std::vec::Vec<Identifier<'tree>>,
    pub r#type: ::std::vec::Vec<Type<'tree>>,
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
                    items.push(<Type as ::treesitter_types::FromNode>::from_node(
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
pub struct ParenthesizedPattern<'tree> {
    pub span: ::treesitter_types::Span,
    pub children: Pattern<'tree>,
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
                <Pattern as ::treesitter_types::FromNode>::from_node(child, src)?
            },
        })
    }
}
impl ::treesitter_types::Spanned for ParenthesizedPattern<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        self.span
    }
}
#[derive(Debug, Clone)]
pub struct ParenthesizedVariableDesignation<'tree> {
    pub span: ::treesitter_types::Span,
    pub name: ::std::vec::Vec<Identifier<'tree>>,
    pub children: ::std::vec::Vec<ParenthesizedVariableDesignationChildren<'tree>>,
}
impl<'tree> ::treesitter_types::FromNode<'tree> for ParenthesizedVariableDesignation<'tree> {
    #[allow(clippy::match_single_binding, clippy::suspicious_else_formatting)]
    fn from_node(
        node: ::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        debug_assert_eq!(node.kind(), "parenthesized_variable_designation");
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
                let mut items = ::std::vec::Vec::new();
                for child in non_field_children {
                    items
                        .push(
                            <ParenthesizedVariableDesignationChildren as ::treesitter_types::FromNode>::from_node(
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
impl ::treesitter_types::Spanned for ParenthesizedVariableDesignation<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        self.span
    }
}
#[derive(Debug, Clone)]
pub struct PointerType<'tree> {
    pub span: ::treesitter_types::Span,
    pub r#type: PointerTypeType<'tree>,
}
impl<'tree> ::treesitter_types::FromNode<'tree> for PointerType<'tree> {
    #[allow(clippy::match_single_binding, clippy::suspicious_else_formatting)]
    fn from_node(
        node: ::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        debug_assert_eq!(node.kind(), "pointer_type");
        Ok(Self {
            span: ::treesitter_types::Span::from(node),
            r#type: {
                let child = node
                    .child_by_field_name("type")
                    .ok_or_else(|| ::treesitter_types::ParseError::missing_field("type", node))?;
                <PointerTypeType as ::treesitter_types::FromNode>::from_node(child, src)?
            },
        })
    }
}
impl ::treesitter_types::Spanned for PointerType<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        self.span
    }
}
#[derive(Debug, Clone)]
pub struct PositionalPatternClause<'tree> {
    pub span: ::treesitter_types::Span,
    pub children: ::std::vec::Vec<Subpattern<'tree>>,
}
impl<'tree> ::treesitter_types::FromNode<'tree> for PositionalPatternClause<'tree> {
    #[allow(clippy::match_single_binding, clippy::suspicious_else_formatting)]
    fn from_node(
        node: ::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        debug_assert_eq!(node.kind(), "positional_pattern_clause");
        Ok(Self {
            span: ::treesitter_types::Span::from(node),
            children: {
                #[allow(clippy::suspicious_else_formatting)]
                let non_field_children = {
                    let mut cursor = node.walk();
                    let mut result = ::std::vec::Vec::new();
                    if cursor.goto_first_child() {
                        loop {
                            if cursor.field_name().is_none()
                                && cursor.node().is_named()
                                && !cursor.node().is_extra()
                            {
                                result.push(cursor.node());
                            }
                            if !cursor.goto_next_sibling() {
                                break;
                            }
                        }
                    }
                    result
                };
                let mut items = ::std::vec::Vec::new();
                for child in non_field_children {
                    items.push(<Subpattern as ::treesitter_types::FromNode>::from_node(
                        child, src,
                    )?);
                }
                items
            },
        })
    }
}
impl ::treesitter_types::Spanned for PositionalPatternClause<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        self.span
    }
}
#[derive(Debug, Clone)]
pub struct PostfixUnaryExpression<'tree> {
    pub span: ::treesitter_types::Span,
    pub children: Expression<'tree>,
}
impl<'tree> ::treesitter_types::FromNode<'tree> for PostfixUnaryExpression<'tree> {
    #[allow(clippy::match_single_binding, clippy::suspicious_else_formatting)]
    fn from_node(
        node: ::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        debug_assert_eq!(node.kind(), "postfix_unary_expression");
        Ok(Self {
            span: ::treesitter_types::Span::from(node),
            children: {
                #[allow(clippy::suspicious_else_formatting)]
                let non_field_children = {
                    let mut cursor = node.walk();
                    let mut result = ::std::vec::Vec::new();
                    if cursor.goto_first_child() {
                        loop {
                            if cursor.field_name().is_none()
                                && cursor.node().is_named()
                                && !cursor.node().is_extra()
                            {
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
impl ::treesitter_types::Spanned for PostfixUnaryExpression<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        self.span
    }
}
#[derive(Debug, Clone)]
pub struct PrefixUnaryExpression<'tree> {
    pub span: ::treesitter_types::Span,
    pub children: Expression<'tree>,
}
impl<'tree> ::treesitter_types::FromNode<'tree> for PrefixUnaryExpression<'tree> {
    #[allow(clippy::match_single_binding, clippy::suspicious_else_formatting)]
    fn from_node(
        node: ::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        debug_assert_eq!(node.kind(), "prefix_unary_expression");
        Ok(Self {
            span: ::treesitter_types::Span::from(node),
            children: {
                #[allow(clippy::suspicious_else_formatting)]
                let non_field_children = {
                    let mut cursor = node.walk();
                    let mut result = ::std::vec::Vec::new();
                    if cursor.goto_first_child() {
                        loop {
                            if cursor.field_name().is_none()
                                && cursor.node().is_named()
                                && !cursor.node().is_extra()
                            {
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
impl ::treesitter_types::Spanned for PrefixUnaryExpression<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        self.span
    }
}
#[derive(Debug, Clone)]
pub struct PreprocDefine<'tree> {
    pub span: ::treesitter_types::Span,
    pub children: PreprocArg<'tree>,
}
impl<'tree> ::treesitter_types::FromNode<'tree> for PreprocDefine<'tree> {
    #[allow(clippy::match_single_binding, clippy::suspicious_else_formatting)]
    fn from_node(
        node: ::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        debug_assert_eq!(node.kind(), "preproc_define");
        Ok(Self {
            span: ::treesitter_types::Span::from(node),
            children: {
                #[allow(clippy::suspicious_else_formatting)]
                let non_field_children = {
                    let mut cursor = node.walk();
                    let mut result = ::std::vec::Vec::new();
                    if cursor.goto_first_child() {
                        loop {
                            if cursor.field_name().is_none()
                                && cursor.node().is_named()
                                && !cursor.node().is_extra()
                            {
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
                <PreprocArg as ::treesitter_types::FromNode>::from_node(child, src)?
            },
        })
    }
}
impl ::treesitter_types::Spanned for PreprocDefine<'_> {
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
pub struct PreprocEndregion<'tree> {
    pub span: ::treesitter_types::Span,
    pub content: ::core::option::Option<PreprocArg<'tree>>,
}
impl<'tree> ::treesitter_types::FromNode<'tree> for PreprocEndregion<'tree> {
    #[allow(clippy::match_single_binding, clippy::suspicious_else_formatting)]
    fn from_node(
        node: ::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        debug_assert_eq!(node.kind(), "preproc_endregion");
        Ok(Self {
            span: ::treesitter_types::Span::from(node),
            content: match node.child_by_field_name("content") {
                Some(child) => Some(<PreprocArg as ::treesitter_types::FromNode>::from_node(
                    child, src,
                )?),
                None => None,
            },
        })
    }
}
impl ::treesitter_types::Spanned for PreprocEndregion<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        self.span
    }
}
#[derive(Debug, Clone)]
pub struct PreprocError<'tree> {
    pub span: ::treesitter_types::Span,
    pub children: PreprocArg<'tree>,
}
impl<'tree> ::treesitter_types::FromNode<'tree> for PreprocError<'tree> {
    #[allow(clippy::match_single_binding, clippy::suspicious_else_formatting)]
    fn from_node(
        node: ::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        debug_assert_eq!(node.kind(), "preproc_error");
        Ok(Self {
            span: ::treesitter_types::Span::from(node),
            children: {
                #[allow(clippy::suspicious_else_formatting)]
                let non_field_children = {
                    let mut cursor = node.walk();
                    let mut result = ::std::vec::Vec::new();
                    if cursor.goto_first_child() {
                        loop {
                            if cursor.field_name().is_none()
                                && cursor.node().is_named()
                                && !cursor.node().is_extra()
                            {
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
                <PreprocArg as ::treesitter_types::FromNode>::from_node(child, src)?
            },
        })
    }
}
impl ::treesitter_types::Spanned for PreprocError<'_> {
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
pub struct PreprocIfInAttributeList<'tree> {
    pub span: ::treesitter_types::Span,
    pub alternative: ::core::option::Option<PreprocIfInAttributeListAlternative<'tree>>,
    pub condition: PreprocIfInAttributeListCondition<'tree>,
    pub children: ::core::option::Option<AttributeList<'tree>>,
}
impl<'tree> ::treesitter_types::FromNode<'tree> for PreprocIfInAttributeList<'tree> {
    #[allow(clippy::match_single_binding, clippy::suspicious_else_formatting)]
    fn from_node(
        node: ::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        debug_assert_eq!(node.kind(), "preproc_if_in_attribute_list");
        Ok(Self {
            span: ::treesitter_types::Span::from(node),
            alternative: match node.child_by_field_name("alternative") {
                Some(child) => {
                    Some(
                        <PreprocIfInAttributeListAlternative as ::treesitter_types::FromNode>::from_node(
                            child,
                            src,
                        )?,
                    )
                }
                None => None,
            },
            condition: {
                let child = node
                    .child_by_field_name("condition")
                    .ok_or_else(|| ::treesitter_types::ParseError::missing_field(
                        "condition",
                        node,
                    ))?;
                <PreprocIfInAttributeListCondition as ::treesitter_types::FromNode>::from_node(
                    child,
                    src,
                )?
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
                match non_field_children.first() {
                    Some(&child) => {
                        Some(
                            <AttributeList as ::treesitter_types::FromNode>::from_node(
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
impl ::treesitter_types::Spanned for PreprocIfInAttributeList<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        self.span
    }
}
#[derive(Debug, Clone)]
pub struct PreprocLine<'tree> {
    pub span: ::treesitter_types::Span,
    pub children: ::std::vec::Vec<PreprocLineChildren<'tree>>,
}
impl<'tree> ::treesitter_types::FromNode<'tree> for PreprocLine<'tree> {
    #[allow(clippy::match_single_binding, clippy::suspicious_else_formatting)]
    fn from_node(
        node: ::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        debug_assert_eq!(node.kind(), "preproc_line");
        Ok(Self {
            span: ::treesitter_types::Span::from(node),
            children: {
                #[allow(clippy::suspicious_else_formatting)]
                let non_field_children = {
                    let mut cursor = node.walk();
                    let mut result = ::std::vec::Vec::new();
                    if cursor.goto_first_child() {
                        loop {
                            if cursor.field_name().is_none()
                                && cursor.node().is_named()
                                && !cursor.node().is_extra()
                            {
                                result.push(cursor.node());
                            }
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
                        <PreprocLineChildren as ::treesitter_types::FromNode>::from_node(
                            child, src,
                        )?,
                    );
                }
                items
            },
        })
    }
}
impl ::treesitter_types::Spanned for PreprocLine<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        self.span
    }
}
#[derive(Debug, Clone)]
pub struct PreprocNullable<'tree> {
    pub span: ::treesitter_types::Span,
    text: &'tree str,
}
impl<'tree> ::treesitter_types::FromNode<'tree> for PreprocNullable<'tree> {
    fn from_node(
        node: ::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        debug_assert_eq!(node.kind(), "preproc_nullable");
        Ok(Self {
            span: ::treesitter_types::Span::from(node),
            text: node.utf8_text(src)?,
        })
    }
}
impl<'tree> ::treesitter_types::LeafNode<'tree> for PreprocNullable<'tree> {
    fn text(&self) -> &'tree str {
        self.text
    }
}
impl ::treesitter_types::Spanned for PreprocNullable<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        self.span
    }
}
#[derive(Debug, Clone)]
pub struct PreprocPragma<'tree> {
    pub span: ::treesitter_types::Span,
    pub children: ::std::vec::Vec<PreprocPragmaChildren<'tree>>,
}
impl<'tree> ::treesitter_types::FromNode<'tree> for PreprocPragma<'tree> {
    #[allow(clippy::match_single_binding, clippy::suspicious_else_formatting)]
    fn from_node(
        node: ::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        debug_assert_eq!(node.kind(), "preproc_pragma");
        Ok(Self {
            span: ::treesitter_types::Span::from(node),
            children: {
                #[allow(clippy::suspicious_else_formatting)]
                let non_field_children = {
                    let mut cursor = node.walk();
                    let mut result = ::std::vec::Vec::new();
                    if cursor.goto_first_child() {
                        loop {
                            if cursor.field_name().is_none()
                                && cursor.node().is_named()
                                && !cursor.node().is_extra()
                            {
                                result.push(cursor.node());
                            }
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
                        <PreprocPragmaChildren as ::treesitter_types::FromNode>::from_node(
                            child, src,
                        )?,
                    );
                }
                items
            },
        })
    }
}
impl ::treesitter_types::Spanned for PreprocPragma<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        self.span
    }
}
#[derive(Debug, Clone)]
pub struct PreprocRegion<'tree> {
    pub span: ::treesitter_types::Span,
    pub content: ::core::option::Option<PreprocArg<'tree>>,
}
impl<'tree> ::treesitter_types::FromNode<'tree> for PreprocRegion<'tree> {
    #[allow(clippy::match_single_binding, clippy::suspicious_else_formatting)]
    fn from_node(
        node: ::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        debug_assert_eq!(node.kind(), "preproc_region");
        Ok(Self {
            span: ::treesitter_types::Span::from(node),
            content: match node.child_by_field_name("content") {
                Some(child) => Some(<PreprocArg as ::treesitter_types::FromNode>::from_node(
                    child, src,
                )?),
                None => None,
            },
        })
    }
}
impl ::treesitter_types::Spanned for PreprocRegion<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        self.span
    }
}
#[derive(Debug, Clone)]
pub struct PreprocUndef<'tree> {
    pub span: ::treesitter_types::Span,
    pub children: PreprocArg<'tree>,
}
impl<'tree> ::treesitter_types::FromNode<'tree> for PreprocUndef<'tree> {
    #[allow(clippy::match_single_binding, clippy::suspicious_else_formatting)]
    fn from_node(
        node: ::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        debug_assert_eq!(node.kind(), "preproc_undef");
        Ok(Self {
            span: ::treesitter_types::Span::from(node),
            children: {
                #[allow(clippy::suspicious_else_formatting)]
                let non_field_children = {
                    let mut cursor = node.walk();
                    let mut result = ::std::vec::Vec::new();
                    if cursor.goto_first_child() {
                        loop {
                            if cursor.field_name().is_none()
                                && cursor.node().is_named()
                                && !cursor.node().is_extra()
                            {
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
                <PreprocArg as ::treesitter_types::FromNode>::from_node(child, src)?
            },
        })
    }
}
impl ::treesitter_types::Spanned for PreprocUndef<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        self.span
    }
}
#[derive(Debug, Clone)]
pub struct PreprocWarning<'tree> {
    pub span: ::treesitter_types::Span,
    pub children: PreprocArg<'tree>,
}
impl<'tree> ::treesitter_types::FromNode<'tree> for PreprocWarning<'tree> {
    #[allow(clippy::match_single_binding, clippy::suspicious_else_formatting)]
    fn from_node(
        node: ::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        debug_assert_eq!(node.kind(), "preproc_warning");
        Ok(Self {
            span: ::treesitter_types::Span::from(node),
            children: {
                #[allow(clippy::suspicious_else_formatting)]
                let non_field_children = {
                    let mut cursor = node.walk();
                    let mut result = ::std::vec::Vec::new();
                    if cursor.goto_first_child() {
                        loop {
                            if cursor.field_name().is_none()
                                && cursor.node().is_named()
                                && !cursor.node().is_extra()
                            {
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
                <PreprocArg as ::treesitter_types::FromNode>::from_node(child, src)?
            },
        })
    }
}
impl ::treesitter_types::Spanned for PreprocWarning<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        self.span
    }
}
#[derive(Debug, Clone)]
pub struct PrimaryConstructorBaseType<'tree> {
    pub span: ::treesitter_types::Span,
    pub r#type: PrimaryConstructorBaseTypeType<'tree>,
    pub children: ArgumentList<'tree>,
}
impl<'tree> ::treesitter_types::FromNode<'tree> for PrimaryConstructorBaseType<'tree> {
    #[allow(clippy::match_single_binding, clippy::suspicious_else_formatting)]
    fn from_node(
        node: ::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        debug_assert_eq!(node.kind(), "primary_constructor_base_type");
        Ok(Self {
            span: ::treesitter_types::Span::from(node),
            r#type: {
                let child = node
                    .child_by_field_name("type")
                    .ok_or_else(|| ::treesitter_types::ParseError::missing_field("type", node))?;
                <PrimaryConstructorBaseTypeType as ::treesitter_types::FromNode>::from_node(
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
impl ::treesitter_types::Spanned for PrimaryConstructorBaseType<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        self.span
    }
}
#[derive(Debug, Clone)]
pub struct PropertyDeclaration<'tree> {
    pub span: ::treesitter_types::Span,
    pub accessors: ::core::option::Option<AccessorList<'tree>>,
    pub name: Identifier<'tree>,
    pub r#type: Type<'tree>,
    pub value: ::core::option::Option<PropertyDeclarationValue<'tree>>,
    pub children: ::std::vec::Vec<PropertyDeclarationChildren<'tree>>,
}
impl<'tree> ::treesitter_types::FromNode<'tree> for PropertyDeclaration<'tree> {
    #[allow(clippy::match_single_binding, clippy::suspicious_else_formatting)]
    fn from_node(
        node: ::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        debug_assert_eq!(node.kind(), "property_declaration");
        Ok(Self {
            span: ::treesitter_types::Span::from(node),
            accessors: match node.child_by_field_name("accessors") {
                Some(child) => Some(<AccessorList as ::treesitter_types::FromNode>::from_node(
                    child, src,
                )?),
                None => None,
            },
            name: {
                let child = node
                    .child_by_field_name("name")
                    .ok_or_else(|| ::treesitter_types::ParseError::missing_field("name", node))?;
                <Identifier as ::treesitter_types::FromNode>::from_node(child, src)?
            },
            r#type: {
                let child = node
                    .child_by_field_name("type")
                    .ok_or_else(|| ::treesitter_types::ParseError::missing_field("type", node))?;
                <Type as ::treesitter_types::FromNode>::from_node(child, src)?
            },
            value: match node.child_by_field_name("value") {
                Some(child) => Some(
                    <PropertyDeclarationValue as ::treesitter_types::FromNode>::from_node(
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
                        <PropertyDeclarationChildren as ::treesitter_types::FromNode>::from_node(
                            child, src,
                        )?,
                    );
                }
                items
            },
        })
    }
}
impl ::treesitter_types::Spanned for PropertyDeclaration<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        self.span
    }
}
#[derive(Debug, Clone)]
pub struct PropertyPatternClause<'tree> {
    pub span: ::treesitter_types::Span,
    pub children: ::std::vec::Vec<Subpattern<'tree>>,
}
impl<'tree> ::treesitter_types::FromNode<'tree> for PropertyPatternClause<'tree> {
    #[allow(clippy::match_single_binding, clippy::suspicious_else_formatting)]
    fn from_node(
        node: ::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        debug_assert_eq!(node.kind(), "property_pattern_clause");
        Ok(Self {
            span: ::treesitter_types::Span::from(node),
            children: {
                #[allow(clippy::suspicious_else_formatting)]
                let non_field_children = {
                    let mut cursor = node.walk();
                    let mut result = ::std::vec::Vec::new();
                    if cursor.goto_first_child() {
                        loop {
                            if cursor.field_name().is_none()
                                && cursor.node().is_named()
                                && !cursor.node().is_extra()
                            {
                                result.push(cursor.node());
                            }
                            if !cursor.goto_next_sibling() {
                                break;
                            }
                        }
                    }
                    result
                };
                let mut items = ::std::vec::Vec::new();
                for child in non_field_children {
                    items.push(<Subpattern as ::treesitter_types::FromNode>::from_node(
                        child, src,
                    )?);
                }
                items
            },
        })
    }
}
impl ::treesitter_types::Spanned for PropertyPatternClause<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        self.span
    }
}
#[derive(Debug, Clone)]
pub struct QualifiedName<'tree> {
    pub span: ::treesitter_types::Span,
    pub name: QualifiedNameName<'tree>,
    pub qualifier: QualifiedNameQualifier<'tree>,
}
impl<'tree> ::treesitter_types::FromNode<'tree> for QualifiedName<'tree> {
    #[allow(clippy::match_single_binding, clippy::suspicious_else_formatting)]
    fn from_node(
        node: ::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        debug_assert_eq!(node.kind(), "qualified_name");
        Ok(Self {
            span: ::treesitter_types::Span::from(node),
            name: {
                let child = node
                    .child_by_field_name("name")
                    .ok_or_else(|| ::treesitter_types::ParseError::missing_field("name", node))?;
                <QualifiedNameName as ::treesitter_types::FromNode>::from_node(child, src)?
            },
            qualifier: {
                let child = node.child_by_field_name("qualifier").ok_or_else(|| {
                    ::treesitter_types::ParseError::missing_field("qualifier", node)
                })?;
                <QualifiedNameQualifier as ::treesitter_types::FromNode>::from_node(child, src)?
            },
        })
    }
}
impl ::treesitter_types::Spanned for QualifiedName<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        self.span
    }
}
#[derive(Debug, Clone)]
pub struct QueryExpression<'tree> {
    pub span: ::treesitter_types::Span,
    pub children: ::std::vec::Vec<QueryExpressionChildren<'tree>>,
}
impl<'tree> ::treesitter_types::FromNode<'tree> for QueryExpression<'tree> {
    #[allow(clippy::match_single_binding, clippy::suspicious_else_formatting)]
    fn from_node(
        node: ::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        debug_assert_eq!(node.kind(), "query_expression");
        Ok(Self {
            span: ::treesitter_types::Span::from(node),
            children: {
                #[allow(clippy::suspicious_else_formatting)]
                let non_field_children = {
                    let mut cursor = node.walk();
                    let mut result = ::std::vec::Vec::new();
                    if cursor.goto_first_child() {
                        loop {
                            if cursor.field_name().is_none()
                                && cursor.node().is_named()
                                && !cursor.node().is_extra()
                            {
                                result.push(cursor.node());
                            }
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
                        <QueryExpressionChildren as ::treesitter_types::FromNode>::from_node(
                            child, src,
                        )?,
                    );
                }
                items
            },
        })
    }
}
impl ::treesitter_types::Spanned for QueryExpression<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        self.span
    }
}
#[derive(Debug, Clone)]
pub struct RangeExpression<'tree> {
    pub span: ::treesitter_types::Span,
    pub children: ::std::vec::Vec<Expression<'tree>>,
}
impl<'tree> ::treesitter_types::FromNode<'tree> for RangeExpression<'tree> {
    #[allow(clippy::match_single_binding, clippy::suspicious_else_formatting)]
    fn from_node(
        node: ::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        debug_assert_eq!(node.kind(), "range_expression");
        Ok(Self {
            span: ::treesitter_types::Span::from(node),
            children: {
                #[allow(clippy::suspicious_else_formatting)]
                let non_field_children = {
                    let mut cursor = node.walk();
                    let mut result = ::std::vec::Vec::new();
                    if cursor.goto_first_child() {
                        loop {
                            if cursor.field_name().is_none()
                                && cursor.node().is_named()
                                && !cursor.node().is_extra()
                            {
                                result.push(cursor.node());
                            }
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
impl ::treesitter_types::Spanned for RangeExpression<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        self.span
    }
}
#[derive(Debug, Clone)]
pub struct RawStringLiteral<'tree> {
    pub span: ::treesitter_types::Span,
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
            children: {
                #[allow(clippy::suspicious_else_formatting)]
                let non_field_children = {
                    let mut cursor = node.walk();
                    let mut result = ::std::vec::Vec::new();
                    if cursor.goto_first_child() {
                        loop {
                            if cursor.field_name().is_none()
                                && cursor.node().is_named()
                                && !cursor.node().is_extra()
                            {
                                result.push(cursor.node());
                            }
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
pub struct RecordDeclaration<'tree> {
    pub span: ::treesitter_types::Span,
    pub body: ::core::option::Option<DeclarationList<'tree>>,
    pub name: Identifier<'tree>,
    pub children: ::std::vec::Vec<RecordDeclarationChildren<'tree>>,
}
impl<'tree> ::treesitter_types::FromNode<'tree> for RecordDeclaration<'tree> {
    #[allow(clippy::match_single_binding, clippy::suspicious_else_formatting)]
    fn from_node(
        node: ::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        debug_assert_eq!(node.kind(), "record_declaration");
        Ok(Self {
            span: ::treesitter_types::Span::from(node),
            body: match node.child_by_field_name("body") {
                Some(child) => {
                    Some(<DeclarationList as ::treesitter_types::FromNode>::from_node(child, src)?)
                }
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
                        <RecordDeclarationChildren as ::treesitter_types::FromNode>::from_node(
                            child, src,
                        )?,
                    );
                }
                items
            },
        })
    }
}
impl ::treesitter_types::Spanned for RecordDeclaration<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        self.span
    }
}
#[derive(Debug, Clone)]
pub struct RecursivePattern<'tree> {
    pub span: ::treesitter_types::Span,
    pub name: ::core::option::Option<Identifier<'tree>>,
    pub r#type: ::core::option::Option<Type<'tree>>,
    pub children: ::std::vec::Vec<RecursivePatternChildren<'tree>>,
}
impl<'tree> ::treesitter_types::FromNode<'tree> for RecursivePattern<'tree> {
    #[allow(clippy::match_single_binding, clippy::suspicious_else_formatting)]
    fn from_node(
        node: ::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        debug_assert_eq!(node.kind(), "recursive_pattern");
        Ok(Self {
            span: ::treesitter_types::Span::from(node),
            name: match node.child_by_field_name("name") {
                Some(child) => Some(<Identifier as ::treesitter_types::FromNode>::from_node(
                    child, src,
                )?),
                None => None,
            },
            r#type: match node.child_by_field_name("type") {
                Some(child) => Some(<Type as ::treesitter_types::FromNode>::from_node(
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
                        <RecursivePatternChildren as ::treesitter_types::FromNode>::from_node(
                            child, src,
                        )?,
                    );
                }
                items
            },
        })
    }
}
impl ::treesitter_types::Spanned for RecursivePattern<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        self.span
    }
}
#[derive(Debug, Clone)]
pub struct RefExpression<'tree> {
    pub span: ::treesitter_types::Span,
    pub children: Expression<'tree>,
}
impl<'tree> ::treesitter_types::FromNode<'tree> for RefExpression<'tree> {
    #[allow(clippy::match_single_binding, clippy::suspicious_else_formatting)]
    fn from_node(
        node: ::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        debug_assert_eq!(node.kind(), "ref_expression");
        Ok(Self {
            span: ::treesitter_types::Span::from(node),
            children: {
                #[allow(clippy::suspicious_else_formatting)]
                let non_field_children = {
                    let mut cursor = node.walk();
                    let mut result = ::std::vec::Vec::new();
                    if cursor.goto_first_child() {
                        loop {
                            if cursor.field_name().is_none()
                                && cursor.node().is_named()
                                && !cursor.node().is_extra()
                            {
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
impl ::treesitter_types::Spanned for RefExpression<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        self.span
    }
}
#[derive(Debug, Clone)]
pub struct RefType<'tree> {
    pub span: ::treesitter_types::Span,
    pub r#type: Type<'tree>,
}
impl<'tree> ::treesitter_types::FromNode<'tree> for RefType<'tree> {
    #[allow(clippy::match_single_binding, clippy::suspicious_else_formatting)]
    fn from_node(
        node: ::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        debug_assert_eq!(node.kind(), "ref_type");
        Ok(Self {
            span: ::treesitter_types::Span::from(node),
            r#type: {
                let child = node
                    .child_by_field_name("type")
                    .ok_or_else(|| ::treesitter_types::ParseError::missing_field("type", node))?;
                <Type as ::treesitter_types::FromNode>::from_node(child, src)?
            },
        })
    }
}
impl ::treesitter_types::Spanned for RefType<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        self.span
    }
}
#[derive(Debug, Clone)]
pub struct ReftypeExpression<'tree> {
    pub span: ::treesitter_types::Span,
    pub children: Expression<'tree>,
}
impl<'tree> ::treesitter_types::FromNode<'tree> for ReftypeExpression<'tree> {
    #[allow(clippy::match_single_binding, clippy::suspicious_else_formatting)]
    fn from_node(
        node: ::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        debug_assert_eq!(node.kind(), "reftype_expression");
        Ok(Self {
            span: ::treesitter_types::Span::from(node),
            children: {
                #[allow(clippy::suspicious_else_formatting)]
                let non_field_children = {
                    let mut cursor = node.walk();
                    let mut result = ::std::vec::Vec::new();
                    if cursor.goto_first_child() {
                        loop {
                            if cursor.field_name().is_none()
                                && cursor.node().is_named()
                                && !cursor.node().is_extra()
                            {
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
impl ::treesitter_types::Spanned for ReftypeExpression<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        self.span
    }
}
#[derive(Debug, Clone)]
pub struct RefvalueExpression<'tree> {
    pub span: ::treesitter_types::Span,
    pub r#type: Type<'tree>,
    pub value: Expression<'tree>,
}
impl<'tree> ::treesitter_types::FromNode<'tree> for RefvalueExpression<'tree> {
    #[allow(clippy::match_single_binding, clippy::suspicious_else_formatting)]
    fn from_node(
        node: ::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        debug_assert_eq!(node.kind(), "refvalue_expression");
        Ok(Self {
            span: ::treesitter_types::Span::from(node),
            r#type: {
                let child = node
                    .child_by_field_name("type")
                    .ok_or_else(|| ::treesitter_types::ParseError::missing_field("type", node))?;
                <Type as ::treesitter_types::FromNode>::from_node(child, src)?
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
impl ::treesitter_types::Spanned for RefvalueExpression<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        self.span
    }
}
#[derive(Debug, Clone)]
pub struct RelationalPattern<'tree> {
    pub span: ::treesitter_types::Span,
    pub children: Expression<'tree>,
}
impl<'tree> ::treesitter_types::FromNode<'tree> for RelationalPattern<'tree> {
    #[allow(clippy::match_single_binding, clippy::suspicious_else_formatting)]
    fn from_node(
        node: ::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        debug_assert_eq!(node.kind(), "relational_pattern");
        Ok(Self {
            span: ::treesitter_types::Span::from(node),
            children: {
                #[allow(clippy::suspicious_else_formatting)]
                let non_field_children = {
                    let mut cursor = node.walk();
                    let mut result = ::std::vec::Vec::new();
                    if cursor.goto_first_child() {
                        loop {
                            if cursor.field_name().is_none()
                                && cursor.node().is_named()
                                && !cursor.node().is_extra()
                            {
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
impl ::treesitter_types::Spanned for RelationalPattern<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        self.span
    }
}
#[derive(Debug, Clone)]
pub struct ReturnStatement<'tree> {
    pub span: ::treesitter_types::Span,
    pub children: ::core::option::Option<Expression<'tree>>,
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
                    Some(&child) => Some(<Expression as ::treesitter_types::FromNode>::from_node(
                        child, src,
                    )?),
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
pub struct ScopedType<'tree> {
    pub span: ::treesitter_types::Span,
    pub r#type: ScopedTypeType<'tree>,
}
impl<'tree> ::treesitter_types::FromNode<'tree> for ScopedType<'tree> {
    #[allow(clippy::match_single_binding, clippy::suspicious_else_formatting)]
    fn from_node(
        node: ::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        debug_assert_eq!(node.kind(), "scoped_type");
        Ok(Self {
            span: ::treesitter_types::Span::from(node),
            r#type: {
                let child = node
                    .child_by_field_name("type")
                    .ok_or_else(|| ::treesitter_types::ParseError::missing_field("type", node))?;
                <ScopedTypeType as ::treesitter_types::FromNode>::from_node(child, src)?
            },
        })
    }
}
impl ::treesitter_types::Spanned for ScopedType<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        self.span
    }
}
#[derive(Debug, Clone)]
pub struct SelectClause<'tree> {
    pub span: ::treesitter_types::Span,
    pub children: Expression<'tree>,
}
impl<'tree> ::treesitter_types::FromNode<'tree> for SelectClause<'tree> {
    #[allow(clippy::match_single_binding, clippy::suspicious_else_formatting)]
    fn from_node(
        node: ::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        debug_assert_eq!(node.kind(), "select_clause");
        Ok(Self {
            span: ::treesitter_types::Span::from(node),
            children: {
                #[allow(clippy::suspicious_else_formatting)]
                let non_field_children = {
                    let mut cursor = node.walk();
                    let mut result = ::std::vec::Vec::new();
                    if cursor.goto_first_child() {
                        loop {
                            if cursor.field_name().is_none()
                                && cursor.node().is_named()
                                && !cursor.node().is_extra()
                            {
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
impl ::treesitter_types::Spanned for SelectClause<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        self.span
    }
}
#[derive(Debug, Clone)]
pub struct SizeofExpression<'tree> {
    pub span: ::treesitter_types::Span,
    pub r#type: Type<'tree>,
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
            r#type: {
                let child = node
                    .child_by_field_name("type")
                    .ok_or_else(|| ::treesitter_types::ParseError::missing_field("type", node))?;
                <Type as ::treesitter_types::FromNode>::from_node(child, src)?
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
pub struct StackallocExpression<'tree> {
    pub span: ::treesitter_types::Span,
    pub r#type: ArrayType<'tree>,
    pub children: ::core::option::Option<InitializerExpression<'tree>>,
}
impl<'tree> ::treesitter_types::FromNode<'tree> for StackallocExpression<'tree> {
    #[allow(clippy::match_single_binding, clippy::suspicious_else_formatting)]
    fn from_node(
        node: ::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        debug_assert_eq!(node.kind(), "stackalloc_expression");
        Ok(Self {
            span: ::treesitter_types::Span::from(node),
            r#type: {
                let child = node
                    .child_by_field_name("type")
                    .ok_or_else(|| ::treesitter_types::ParseError::missing_field("type", node))?;
                <ArrayType as ::treesitter_types::FromNode>::from_node(child, src)?
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
                        <InitializerExpression as ::treesitter_types::FromNode>::from_node(
                            child, src,
                        )?,
                    ),
                    None => None,
                }
            },
        })
    }
}
impl ::treesitter_types::Spanned for StackallocExpression<'_> {
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
pub struct StructDeclaration<'tree> {
    pub span: ::treesitter_types::Span,
    pub body: ::core::option::Option<DeclarationList<'tree>>,
    pub name: Identifier<'tree>,
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
            body: match node.child_by_field_name("body") {
                Some(child) => {
                    Some(<DeclarationList as ::treesitter_types::FromNode>::from_node(child, src)?)
                }
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
pub struct Subpattern<'tree> {
    pub span: ::treesitter_types::Span,
    pub children: ::std::vec::Vec<SubpatternChildren<'tree>>,
}
impl<'tree> ::treesitter_types::FromNode<'tree> for Subpattern<'tree> {
    #[allow(clippy::match_single_binding, clippy::suspicious_else_formatting)]
    fn from_node(
        node: ::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        debug_assert_eq!(node.kind(), "subpattern");
        Ok(Self {
            span: ::treesitter_types::Span::from(node),
            children: {
                #[allow(clippy::suspicious_else_formatting)]
                let non_field_children = {
                    let mut cursor = node.walk();
                    let mut result = ::std::vec::Vec::new();
                    if cursor.goto_first_child() {
                        loop {
                            if cursor.field_name().is_none()
                                && cursor.node().is_named()
                                && !cursor.node().is_extra()
                            {
                                result.push(cursor.node());
                            }
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
                        <SubpatternChildren as ::treesitter_types::FromNode>::from_node(
                            child, src,
                        )?,
                    );
                }
                items
            },
        })
    }
}
impl ::treesitter_types::Spanned for Subpattern<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        self.span
    }
}
#[derive(Debug, Clone)]
pub struct SwitchBody<'tree> {
    pub span: ::treesitter_types::Span,
    pub children: ::std::vec::Vec<SwitchSection<'tree>>,
}
impl<'tree> ::treesitter_types::FromNode<'tree> for SwitchBody<'tree> {
    #[allow(clippy::match_single_binding, clippy::suspicious_else_formatting)]
    fn from_node(
        node: ::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        debug_assert_eq!(node.kind(), "switch_body");
        Ok(Self {
            span: ::treesitter_types::Span::from(node),
            children: {
                #[allow(clippy::suspicious_else_formatting)]
                let non_field_children = {
                    let mut cursor = node.walk();
                    let mut result = ::std::vec::Vec::new();
                    if cursor.goto_first_child() {
                        loop {
                            if cursor.field_name().is_none()
                                && cursor.node().is_named()
                                && !cursor.node().is_extra()
                            {
                                result.push(cursor.node());
                            }
                            if !cursor.goto_next_sibling() {
                                break;
                            }
                        }
                    }
                    result
                };
                let mut items = ::std::vec::Vec::new();
                for child in non_field_children {
                    items.push(<SwitchSection as ::treesitter_types::FromNode>::from_node(
                        child, src,
                    )?);
                }
                items
            },
        })
    }
}
impl ::treesitter_types::Spanned for SwitchBody<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        self.span
    }
}
#[derive(Debug, Clone)]
pub struct SwitchExpression<'tree> {
    pub span: ::treesitter_types::Span,
    pub children: ::std::vec::Vec<SwitchExpressionChildren<'tree>>,
}
impl<'tree> ::treesitter_types::FromNode<'tree> for SwitchExpression<'tree> {
    #[allow(clippy::match_single_binding, clippy::suspicious_else_formatting)]
    fn from_node(
        node: ::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        debug_assert_eq!(node.kind(), "switch_expression");
        Ok(Self {
            span: ::treesitter_types::Span::from(node),
            children: {
                #[allow(clippy::suspicious_else_formatting)]
                let non_field_children = {
                    let mut cursor = node.walk();
                    let mut result = ::std::vec::Vec::new();
                    if cursor.goto_first_child() {
                        loop {
                            if cursor.field_name().is_none()
                                && cursor.node().is_named()
                                && !cursor.node().is_extra()
                            {
                                result.push(cursor.node());
                            }
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
                        <SwitchExpressionChildren as ::treesitter_types::FromNode>::from_node(
                            child, src,
                        )?,
                    );
                }
                items
            },
        })
    }
}
impl ::treesitter_types::Spanned for SwitchExpression<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        self.span
    }
}
#[derive(Debug, Clone)]
pub struct SwitchExpressionArm<'tree> {
    pub span: ::treesitter_types::Span,
    pub children: ::std::vec::Vec<SwitchExpressionArmChildren<'tree>>,
}
impl<'tree> ::treesitter_types::FromNode<'tree> for SwitchExpressionArm<'tree> {
    #[allow(clippy::match_single_binding, clippy::suspicious_else_formatting)]
    fn from_node(
        node: ::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        debug_assert_eq!(node.kind(), "switch_expression_arm");
        Ok(Self {
            span: ::treesitter_types::Span::from(node),
            children: {
                #[allow(clippy::suspicious_else_formatting)]
                let non_field_children = {
                    let mut cursor = node.walk();
                    let mut result = ::std::vec::Vec::new();
                    if cursor.goto_first_child() {
                        loop {
                            if cursor.field_name().is_none()
                                && cursor.node().is_named()
                                && !cursor.node().is_extra()
                            {
                                result.push(cursor.node());
                            }
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
                        <SwitchExpressionArmChildren as ::treesitter_types::FromNode>::from_node(
                            child, src,
                        )?,
                    );
                }
                items
            },
        })
    }
}
impl ::treesitter_types::Spanned for SwitchExpressionArm<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        self.span
    }
}
#[derive(Debug, Clone)]
pub struct SwitchSection<'tree> {
    pub span: ::treesitter_types::Span,
    pub children: ::std::vec::Vec<SwitchSectionChildren<'tree>>,
}
impl<'tree> ::treesitter_types::FromNode<'tree> for SwitchSection<'tree> {
    #[allow(clippy::match_single_binding, clippy::suspicious_else_formatting)]
    fn from_node(
        node: ::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        debug_assert_eq!(node.kind(), "switch_section");
        Ok(Self {
            span: ::treesitter_types::Span::from(node),
            children: {
                #[allow(clippy::suspicious_else_formatting)]
                let non_field_children = {
                    let mut cursor = node.walk();
                    let mut result = ::std::vec::Vec::new();
                    if cursor.goto_first_child() {
                        loop {
                            if cursor.field_name().is_none()
                                && cursor.node().is_named()
                                && !cursor.node().is_extra()
                            {
                                result.push(cursor.node());
                            }
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
                        <SwitchSectionChildren as ::treesitter_types::FromNode>::from_node(
                            child, src,
                        )?,
                    );
                }
                items
            },
        })
    }
}
impl ::treesitter_types::Spanned for SwitchSection<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        self.span
    }
}
#[derive(Debug, Clone)]
pub struct SwitchStatement<'tree> {
    pub span: ::treesitter_types::Span,
    pub body: SwitchBody<'tree>,
    pub value: SwitchStatementValue<'tree>,
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
                <SwitchBody as ::treesitter_types::FromNode>::from_node(child, src)?
            },
            value: {
                let child = node
                    .child_by_field_name("value")
                    .ok_or_else(|| ::treesitter_types::ParseError::missing_field("value", node))?;
                <SwitchStatementValue as ::treesitter_types::FromNode>::from_node(child, src)?
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
pub struct ThrowExpression<'tree> {
    pub span: ::treesitter_types::Span,
    pub children: Expression<'tree>,
}
impl<'tree> ::treesitter_types::FromNode<'tree> for ThrowExpression<'tree> {
    #[allow(clippy::match_single_binding, clippy::suspicious_else_formatting)]
    fn from_node(
        node: ::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        debug_assert_eq!(node.kind(), "throw_expression");
        Ok(Self {
            span: ::treesitter_types::Span::from(node),
            children: {
                #[allow(clippy::suspicious_else_formatting)]
                let non_field_children = {
                    let mut cursor = node.walk();
                    let mut result = ::std::vec::Vec::new();
                    if cursor.goto_first_child() {
                        loop {
                            if cursor.field_name().is_none()
                                && cursor.node().is_named()
                                && !cursor.node().is_extra()
                            {
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
impl ::treesitter_types::Spanned for ThrowExpression<'_> {
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
pub struct TryStatement<'tree> {
    pub span: ::treesitter_types::Span,
    pub body: Block<'tree>,
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
                <Block as ::treesitter_types::FromNode>::from_node(child, src)?
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
pub struct TupleElement<'tree> {
    pub span: ::treesitter_types::Span,
    pub name: ::core::option::Option<Identifier<'tree>>,
    pub r#type: Type<'tree>,
}
impl<'tree> ::treesitter_types::FromNode<'tree> for TupleElement<'tree> {
    #[allow(clippy::match_single_binding, clippy::suspicious_else_formatting)]
    fn from_node(
        node: ::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        debug_assert_eq!(node.kind(), "tuple_element");
        Ok(Self {
            span: ::treesitter_types::Span::from(node),
            name: match node.child_by_field_name("name") {
                Some(child) => Some(<Identifier as ::treesitter_types::FromNode>::from_node(
                    child, src,
                )?),
                None => None,
            },
            r#type: {
                let child = node
                    .child_by_field_name("type")
                    .ok_or_else(|| ::treesitter_types::ParseError::missing_field("type", node))?;
                <Type as ::treesitter_types::FromNode>::from_node(child, src)?
            },
        })
    }
}
impl ::treesitter_types::Spanned for TupleElement<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        self.span
    }
}
#[derive(Debug, Clone)]
pub struct TupleExpression<'tree> {
    pub span: ::treesitter_types::Span,
    pub children: ::std::vec::Vec<Argument<'tree>>,
}
impl<'tree> ::treesitter_types::FromNode<'tree> for TupleExpression<'tree> {
    #[allow(clippy::match_single_binding, clippy::suspicious_else_formatting)]
    fn from_node(
        node: ::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        debug_assert_eq!(node.kind(), "tuple_expression");
        Ok(Self {
            span: ::treesitter_types::Span::from(node),
            children: {
                #[allow(clippy::suspicious_else_formatting)]
                let non_field_children = {
                    let mut cursor = node.walk();
                    let mut result = ::std::vec::Vec::new();
                    if cursor.goto_first_child() {
                        loop {
                            if cursor.field_name().is_none()
                                && cursor.node().is_named()
                                && !cursor.node().is_extra()
                            {
                                result.push(cursor.node());
                            }
                            if !cursor.goto_next_sibling() {
                                break;
                            }
                        }
                    }
                    result
                };
                let mut items = ::std::vec::Vec::new();
                for child in non_field_children {
                    items.push(<Argument as ::treesitter_types::FromNode>::from_node(
                        child, src,
                    )?);
                }
                items
            },
        })
    }
}
impl ::treesitter_types::Spanned for TupleExpression<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        self.span
    }
}
#[derive(Debug, Clone)]
pub struct TuplePattern<'tree> {
    pub span: ::treesitter_types::Span,
    pub name: ::std::vec::Vec<Identifier<'tree>>,
    pub children: ::std::vec::Vec<TuplePatternChildren<'tree>>,
}
impl<'tree> ::treesitter_types::FromNode<'tree> for TuplePattern<'tree> {
    #[allow(clippy::match_single_binding, clippy::suspicious_else_formatting)]
    fn from_node(
        node: ::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        debug_assert_eq!(node.kind(), "tuple_pattern");
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
                let mut items = ::std::vec::Vec::new();
                for child in non_field_children {
                    items.push(
                        <TuplePatternChildren as ::treesitter_types::FromNode>::from_node(
                            child, src,
                        )?,
                    );
                }
                items
            },
        })
    }
}
impl ::treesitter_types::Spanned for TuplePattern<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        self.span
    }
}
#[derive(Debug, Clone)]
pub struct TupleType<'tree> {
    pub span: ::treesitter_types::Span,
    pub children: ::std::vec::Vec<TupleElement<'tree>>,
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
                    items.push(<TupleElement as ::treesitter_types::FromNode>::from_node(
                        child, src,
                    )?);
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
pub struct TypeArgumentList<'tree> {
    pub span: ::treesitter_types::Span,
    pub children: ::std::vec::Vec<Type<'tree>>,
}
impl<'tree> ::treesitter_types::FromNode<'tree> for TypeArgumentList<'tree> {
    #[allow(clippy::match_single_binding, clippy::suspicious_else_formatting)]
    fn from_node(
        node: ::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        debug_assert_eq!(node.kind(), "type_argument_list");
        Ok(Self {
            span: ::treesitter_types::Span::from(node),
            children: {
                #[allow(clippy::suspicious_else_formatting)]
                let non_field_children = {
                    let mut cursor = node.walk();
                    let mut result = ::std::vec::Vec::new();
                    if cursor.goto_first_child() {
                        loop {
                            if cursor.field_name().is_none()
                                && cursor.node().is_named()
                                && !cursor.node().is_extra()
                            {
                                result.push(cursor.node());
                            }
                            if !cursor.goto_next_sibling() {
                                break;
                            }
                        }
                    }
                    result
                };
                let mut items = ::std::vec::Vec::new();
                for child in non_field_children {
                    items.push(<Type as ::treesitter_types::FromNode>::from_node(
                        child, src,
                    )?);
                }
                items
            },
        })
    }
}
impl ::treesitter_types::Spanned for TypeArgumentList<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        self.span
    }
}
#[derive(Debug, Clone)]
pub struct TypeParameter<'tree> {
    pub span: ::treesitter_types::Span,
    pub name: Identifier<'tree>,
    pub children: ::std::vec::Vec<TypeParameterChildren<'tree>>,
}
impl<'tree> ::treesitter_types::FromNode<'tree> for TypeParameter<'tree> {
    #[allow(clippy::match_single_binding, clippy::suspicious_else_formatting)]
    fn from_node(
        node: ::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        debug_assert_eq!(node.kind(), "type_parameter");
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
                let mut items = ::std::vec::Vec::new();
                for child in non_field_children {
                    items.push(
                        <TypeParameterChildren as ::treesitter_types::FromNode>::from_node(
                            child, src,
                        )?,
                    );
                }
                items
            },
        })
    }
}
impl ::treesitter_types::Spanned for TypeParameter<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        self.span
    }
}
#[derive(Debug, Clone)]
pub struct TypeParameterConstraint<'tree> {
    pub span: ::treesitter_types::Span,
    pub r#type: ::core::option::Option<Type<'tree>>,
    pub children: ::core::option::Option<ConstructorConstraint<'tree>>,
}
impl<'tree> ::treesitter_types::FromNode<'tree> for TypeParameterConstraint<'tree> {
    #[allow(clippy::match_single_binding, clippy::suspicious_else_formatting)]
    fn from_node(
        node: ::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        debug_assert_eq!(node.kind(), "type_parameter_constraint");
        Ok(Self {
            span: ::treesitter_types::Span::from(node),
            r#type: match node.child_by_field_name("type") {
                Some(child) => Some(<Type as ::treesitter_types::FromNode>::from_node(
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
                        <ConstructorConstraint as ::treesitter_types::FromNode>::from_node(
                            child, src,
                        )?,
                    ),
                    None => None,
                }
            },
        })
    }
}
impl ::treesitter_types::Spanned for TypeParameterConstraint<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        self.span
    }
}
#[derive(Debug, Clone)]
pub struct TypeParameterConstraintsClause<'tree> {
    pub span: ::treesitter_types::Span,
    pub children: ::std::vec::Vec<TypeParameterConstraintsClauseChildren<'tree>>,
}
impl<'tree> ::treesitter_types::FromNode<'tree> for TypeParameterConstraintsClause<'tree> {
    #[allow(clippy::match_single_binding, clippy::suspicious_else_formatting)]
    fn from_node(
        node: ::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        debug_assert_eq!(node.kind(), "type_parameter_constraints_clause");
        Ok(Self {
            span: ::treesitter_types::Span::from(node),
            children: {
                #[allow(clippy::suspicious_else_formatting)]
                let non_field_children = {
                    let mut cursor = node.walk();
                    let mut result = ::std::vec::Vec::new();
                    if cursor.goto_first_child() {
                        loop {
                            if cursor.field_name().is_none()
                                && cursor.node().is_named()
                                && !cursor.node().is_extra()
                            {
                                result.push(cursor.node());
                            }
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
                            <TypeParameterConstraintsClauseChildren as ::treesitter_types::FromNode>::from_node(
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
impl ::treesitter_types::Spanned for TypeParameterConstraintsClause<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        self.span
    }
}
#[derive(Debug, Clone)]
pub struct TypeParameterList<'tree> {
    pub span: ::treesitter_types::Span,
    pub children: ::std::vec::Vec<TypeParameter<'tree>>,
}
impl<'tree> ::treesitter_types::FromNode<'tree> for TypeParameterList<'tree> {
    #[allow(clippy::match_single_binding, clippy::suspicious_else_formatting)]
    fn from_node(
        node: ::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        debug_assert_eq!(node.kind(), "type_parameter_list");
        Ok(Self {
            span: ::treesitter_types::Span::from(node),
            children: {
                #[allow(clippy::suspicious_else_formatting)]
                let non_field_children = {
                    let mut cursor = node.walk();
                    let mut result = ::std::vec::Vec::new();
                    if cursor.goto_first_child() {
                        loop {
                            if cursor.field_name().is_none()
                                && cursor.node().is_named()
                                && !cursor.node().is_extra()
                            {
                                result.push(cursor.node());
                            }
                            if !cursor.goto_next_sibling() {
                                break;
                            }
                        }
                    }
                    result
                };
                let mut items = ::std::vec::Vec::new();
                for child in non_field_children {
                    items.push(<TypeParameter as ::treesitter_types::FromNode>::from_node(
                        child, src,
                    )?);
                }
                items
            },
        })
    }
}
impl ::treesitter_types::Spanned for TypeParameterList<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        self.span
    }
}
#[derive(Debug, Clone)]
pub struct TypePattern<'tree> {
    pub span: ::treesitter_types::Span,
    pub r#type: Type<'tree>,
}
impl<'tree> ::treesitter_types::FromNode<'tree> for TypePattern<'tree> {
    #[allow(clippy::match_single_binding, clippy::suspicious_else_formatting)]
    fn from_node(
        node: ::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        debug_assert_eq!(node.kind(), "type_pattern");
        Ok(Self {
            span: ::treesitter_types::Span::from(node),
            r#type: {
                let child = node
                    .child_by_field_name("type")
                    .ok_or_else(|| ::treesitter_types::ParseError::missing_field("type", node))?;
                <Type as ::treesitter_types::FromNode>::from_node(child, src)?
            },
        })
    }
}
impl ::treesitter_types::Spanned for TypePattern<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        self.span
    }
}
#[derive(Debug, Clone)]
pub struct TypeofExpression<'tree> {
    pub span: ::treesitter_types::Span,
    pub r#type: Type<'tree>,
}
impl<'tree> ::treesitter_types::FromNode<'tree> for TypeofExpression<'tree> {
    #[allow(clippy::match_single_binding, clippy::suspicious_else_formatting)]
    fn from_node(
        node: ::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        debug_assert_eq!(node.kind(), "typeof_expression");
        Ok(Self {
            span: ::treesitter_types::Span::from(node),
            r#type: {
                let child = node
                    .child_by_field_name("type")
                    .ok_or_else(|| ::treesitter_types::ParseError::missing_field("type", node))?;
                <Type as ::treesitter_types::FromNode>::from_node(child, src)?
            },
        })
    }
}
impl ::treesitter_types::Spanned for TypeofExpression<'_> {
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
pub struct UnsafeStatement<'tree> {
    pub span: ::treesitter_types::Span,
    pub children: Block<'tree>,
}
impl<'tree> ::treesitter_types::FromNode<'tree> for UnsafeStatement<'tree> {
    #[allow(clippy::match_single_binding, clippy::suspicious_else_formatting)]
    fn from_node(
        node: ::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        debug_assert_eq!(node.kind(), "unsafe_statement");
        Ok(Self {
            span: ::treesitter_types::Span::from(node),
            children: {
                #[allow(clippy::suspicious_else_formatting)]
                let non_field_children = {
                    let mut cursor = node.walk();
                    let mut result = ::std::vec::Vec::new();
                    if cursor.goto_first_child() {
                        loop {
                            if cursor.field_name().is_none()
                                && cursor.node().is_named()
                                && !cursor.node().is_extra()
                            {
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
                <Block as ::treesitter_types::FromNode>::from_node(child, src)?
            },
        })
    }
}
impl ::treesitter_types::Spanned for UnsafeStatement<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        self.span
    }
}
#[derive(Debug, Clone)]
pub struct UsingDirective<'tree> {
    pub span: ::treesitter_types::Span,
    pub name: ::core::option::Option<Identifier<'tree>>,
    pub children: Type<'tree>,
}
impl<'tree> ::treesitter_types::FromNode<'tree> for UsingDirective<'tree> {
    #[allow(clippy::match_single_binding, clippy::suspicious_else_formatting)]
    fn from_node(
        node: ::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        debug_assert_eq!(node.kind(), "using_directive");
        Ok(Self {
            span: ::treesitter_types::Span::from(node),
            name: match node.child_by_field_name("name") {
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
impl ::treesitter_types::Spanned for UsingDirective<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        self.span
    }
}
#[derive(Debug, Clone)]
pub struct UsingStatement<'tree> {
    pub span: ::treesitter_types::Span,
    pub body: Statement<'tree>,
    pub children: UsingStatementChildren<'tree>,
}
impl<'tree> ::treesitter_types::FromNode<'tree> for UsingStatement<'tree> {
    #[allow(clippy::match_single_binding, clippy::suspicious_else_formatting)]
    fn from_node(
        node: ::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        debug_assert_eq!(node.kind(), "using_statement");
        Ok(Self {
            span: ::treesitter_types::Span::from(node),
            body: {
                let child = node
                    .child_by_field_name("body")
                    .ok_or_else(|| ::treesitter_types::ParseError::missing_field("body", node))?;
                <Statement as ::treesitter_types::FromNode>::from_node(child, src)?
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
                <UsingStatementChildren as ::treesitter_types::FromNode>::from_node(child, src)?
            },
        })
    }
}
impl ::treesitter_types::Spanned for UsingStatement<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        self.span
    }
}
#[derive(Debug, Clone)]
pub struct VarPattern<'tree> {
    pub span: ::treesitter_types::Span,
    pub name: ::core::option::Option<Identifier<'tree>>,
    pub children: ::core::option::Option<VarPatternChildren<'tree>>,
}
impl<'tree> ::treesitter_types::FromNode<'tree> for VarPattern<'tree> {
    #[allow(clippy::match_single_binding, clippy::suspicious_else_formatting)]
    fn from_node(
        node: ::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        debug_assert_eq!(node.kind(), "var_pattern");
        Ok(Self {
            span: ::treesitter_types::Span::from(node),
            name: match node.child_by_field_name("name") {
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
                        <VarPatternChildren as ::treesitter_types::FromNode>::from_node(
                            child, src,
                        )?,
                    ),
                    None => None,
                }
            },
        })
    }
}
impl ::treesitter_types::Spanned for VarPattern<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        self.span
    }
}
#[derive(Debug, Clone)]
pub struct VariableDeclaration<'tree> {
    pub span: ::treesitter_types::Span,
    pub r#type: Type<'tree>,
    pub children: ::std::vec::Vec<VariableDeclarator<'tree>>,
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
            r#type: {
                let child = node
                    .child_by_field_name("type")
                    .ok_or_else(|| ::treesitter_types::ParseError::missing_field("type", node))?;
                <Type as ::treesitter_types::FromNode>::from_node(child, src)?
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
                        <VariableDeclarator as ::treesitter_types::FromNode>::from_node(
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
pub struct VariableDeclarator<'tree> {
    pub span: ::treesitter_types::Span,
    pub name: ::core::option::Option<Identifier<'tree>>,
    pub children: ::std::vec::Vec<VariableDeclaratorChildren<'tree>>,
}
impl<'tree> ::treesitter_types::FromNode<'tree> for VariableDeclarator<'tree> {
    #[allow(clippy::match_single_binding, clippy::suspicious_else_formatting)]
    fn from_node(
        node: ::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        debug_assert_eq!(node.kind(), "variable_declarator");
        Ok(Self {
            span: ::treesitter_types::Span::from(node),
            name: match node.child_by_field_name("name") {
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
                let mut items = ::std::vec::Vec::new();
                for child in non_field_children {
                    items.push(
                        <VariableDeclaratorChildren as ::treesitter_types::FromNode>::from_node(
                            child, src,
                        )?,
                    );
                }
                items
            },
        })
    }
}
impl ::treesitter_types::Spanned for VariableDeclarator<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        self.span
    }
}
#[derive(Debug, Clone)]
pub struct WhenClause<'tree> {
    pub span: ::treesitter_types::Span,
    pub children: Expression<'tree>,
}
impl<'tree> ::treesitter_types::FromNode<'tree> for WhenClause<'tree> {
    #[allow(clippy::match_single_binding, clippy::suspicious_else_formatting)]
    fn from_node(
        node: ::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        debug_assert_eq!(node.kind(), "when_clause");
        Ok(Self {
            span: ::treesitter_types::Span::from(node),
            children: {
                #[allow(clippy::suspicious_else_formatting)]
                let non_field_children = {
                    let mut cursor = node.walk();
                    let mut result = ::std::vec::Vec::new();
                    if cursor.goto_first_child() {
                        loop {
                            if cursor.field_name().is_none()
                                && cursor.node().is_named()
                                && !cursor.node().is_extra()
                            {
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
impl ::treesitter_types::Spanned for WhenClause<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        self.span
    }
}
#[derive(Debug, Clone)]
pub struct WhereClause<'tree> {
    pub span: ::treesitter_types::Span,
    pub children: Expression<'tree>,
}
impl<'tree> ::treesitter_types::FromNode<'tree> for WhereClause<'tree> {
    #[allow(clippy::match_single_binding, clippy::suspicious_else_formatting)]
    fn from_node(
        node: ::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        debug_assert_eq!(node.kind(), "where_clause");
        Ok(Self {
            span: ::treesitter_types::Span::from(node),
            children: {
                #[allow(clippy::suspicious_else_formatting)]
                let non_field_children = {
                    let mut cursor = node.walk();
                    let mut result = ::std::vec::Vec::new();
                    if cursor.goto_first_child() {
                        loop {
                            if cursor.field_name().is_none()
                                && cursor.node().is_named()
                                && !cursor.node().is_extra()
                            {
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
impl ::treesitter_types::Spanned for WhereClause<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        self.span
    }
}
#[derive(Debug, Clone)]
pub struct WhileStatement<'tree> {
    pub span: ::treesitter_types::Span,
    pub body: Statement<'tree>,
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
#[derive(Debug, Clone)]
pub struct WithExpression<'tree> {
    pub span: ::treesitter_types::Span,
    pub children: ::std::vec::Vec<WithExpressionChildren<'tree>>,
}
impl<'tree> ::treesitter_types::FromNode<'tree> for WithExpression<'tree> {
    #[allow(clippy::match_single_binding, clippy::suspicious_else_formatting)]
    fn from_node(
        node: ::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        debug_assert_eq!(node.kind(), "with_expression");
        Ok(Self {
            span: ::treesitter_types::Span::from(node),
            children: {
                #[allow(clippy::suspicious_else_formatting)]
                let non_field_children = {
                    let mut cursor = node.walk();
                    let mut result = ::std::vec::Vec::new();
                    if cursor.goto_first_child() {
                        loop {
                            if cursor.field_name().is_none()
                                && cursor.node().is_named()
                                && !cursor.node().is_extra()
                            {
                                result.push(cursor.node());
                            }
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
                        <WithExpressionChildren as ::treesitter_types::FromNode>::from_node(
                            child, src,
                        )?,
                    );
                }
                items
            },
        })
    }
}
impl ::treesitter_types::Spanned for WithExpression<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        self.span
    }
}
#[derive(Debug, Clone)]
pub struct WithInitializer<'tree> {
    pub span: ::treesitter_types::Span,
    pub children: ::std::vec::Vec<WithInitializerChildren<'tree>>,
}
impl<'tree> ::treesitter_types::FromNode<'tree> for WithInitializer<'tree> {
    #[allow(clippy::match_single_binding, clippy::suspicious_else_formatting)]
    fn from_node(
        node: ::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        debug_assert_eq!(node.kind(), "with_initializer");
        Ok(Self {
            span: ::treesitter_types::Span::from(node),
            children: {
                #[allow(clippy::suspicious_else_formatting)]
                let non_field_children = {
                    let mut cursor = node.walk();
                    let mut result = ::std::vec::Vec::new();
                    if cursor.goto_first_child() {
                        loop {
                            if cursor.field_name().is_none()
                                && cursor.node().is_named()
                                && !cursor.node().is_extra()
                            {
                                result.push(cursor.node());
                            }
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
                        <WithInitializerChildren as ::treesitter_types::FromNode>::from_node(
                            child, src,
                        )?,
                    );
                }
                items
            },
        })
    }
}
impl ::treesitter_types::Spanned for WithInitializer<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        self.span
    }
}
#[derive(Debug, Clone)]
pub struct YieldStatement<'tree> {
    pub span: ::treesitter_types::Span,
    pub children: ::core::option::Option<Expression<'tree>>,
}
impl<'tree> ::treesitter_types::FromNode<'tree> for YieldStatement<'tree> {
    #[allow(clippy::match_single_binding, clippy::suspicious_else_formatting)]
    fn from_node(
        node: ::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        debug_assert_eq!(node.kind(), "yield_statement");
        Ok(Self {
            span: ::treesitter_types::Span::from(node),
            children: {
                #[allow(clippy::suspicious_else_formatting)]
                let non_field_children = {
                    let mut cursor = node.walk();
                    let mut result = ::std::vec::Vec::new();
                    if cursor.goto_first_child() {
                        loop {
                            if cursor.field_name().is_none()
                                && cursor.node().is_named()
                                && !cursor.node().is_extra()
                            {
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
impl ::treesitter_types::Spanned for YieldStatement<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        self.span
    }
}
#[derive(Debug, Clone)]
pub struct CharacterLiteralContent<'tree> {
    pub span: ::treesitter_types::Span,
    text: &'tree str,
}
impl<'tree> ::treesitter_types::FromNode<'tree> for CharacterLiteralContent<'tree> {
    fn from_node(
        node: ::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        debug_assert_eq!(node.kind(), "character_literal_content");
        Ok(Self {
            span: ::treesitter_types::Span::from(node),
            text: node.utf8_text(src)?,
        })
    }
}
impl<'tree> ::treesitter_types::LeafNode<'tree> for CharacterLiteralContent<'tree> {
    fn text(&self) -> &'tree str {
        self.text
    }
}
impl ::treesitter_types::Spanned for CharacterLiteralContent<'_> {
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
pub struct Discard<'tree> {
    pub span: ::treesitter_types::Span,
    text: &'tree str,
}
impl<'tree> ::treesitter_types::FromNode<'tree> for Discard<'tree> {
    fn from_node(
        node: ::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        debug_assert_eq!(node.kind(), "discard");
        Ok(Self {
            span: ::treesitter_types::Span::from(node),
            text: node.utf8_text(src)?,
        })
    }
}
impl<'tree> ::treesitter_types::LeafNode<'tree> for Discard<'tree> {
    fn text(&self) -> &'tree str {
        self.text
    }
}
impl ::treesitter_types::Spanned for Discard<'_> {
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
pub struct IntegerLiteral<'tree> {
    pub span: ::treesitter_types::Span,
    text: &'tree str,
}
impl<'tree> ::treesitter_types::FromNode<'tree> for IntegerLiteral<'tree> {
    fn from_node(
        node: ::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        debug_assert_eq!(node.kind(), "integer_literal");
        Ok(Self {
            span: ::treesitter_types::Span::from(node),
            text: node.utf8_text(src)?,
        })
    }
}
impl<'tree> ::treesitter_types::LeafNode<'tree> for IntegerLiteral<'tree> {
    fn text(&self) -> &'tree str {
        self.text
    }
}
impl ::treesitter_types::Spanned for IntegerLiteral<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        self.span
    }
}
#[derive(Debug, Clone)]
pub struct InterpolationBrace<'tree> {
    pub span: ::treesitter_types::Span,
    text: &'tree str,
}
impl<'tree> ::treesitter_types::FromNode<'tree> for InterpolationBrace<'tree> {
    fn from_node(
        node: ::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        debug_assert_eq!(node.kind(), "interpolation_brace");
        Ok(Self {
            span: ::treesitter_types::Span::from(node),
            text: node.utf8_text(src)?,
        })
    }
}
impl<'tree> ::treesitter_types::LeafNode<'tree> for InterpolationBrace<'tree> {
    fn text(&self) -> &'tree str {
        self.text
    }
}
impl ::treesitter_types::Spanned for InterpolationBrace<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        self.span
    }
}
#[derive(Debug, Clone)]
pub struct InterpolationQuote<'tree> {
    pub span: ::treesitter_types::Span,
    text: &'tree str,
}
impl<'tree> ::treesitter_types::FromNode<'tree> for InterpolationQuote<'tree> {
    fn from_node(
        node: ::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        debug_assert_eq!(node.kind(), "interpolation_quote");
        Ok(Self {
            span: ::treesitter_types::Span::from(node),
            text: node.utf8_text(src)?,
        })
    }
}
impl<'tree> ::treesitter_types::LeafNode<'tree> for InterpolationQuote<'tree> {
    fn text(&self) -> &'tree str {
        self.text
    }
}
impl ::treesitter_types::Spanned for InterpolationQuote<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        self.span
    }
}
#[derive(Debug, Clone)]
pub struct InterpolationStart<'tree> {
    pub span: ::treesitter_types::Span,
    text: &'tree str,
}
impl<'tree> ::treesitter_types::FromNode<'tree> for InterpolationStart<'tree> {
    fn from_node(
        node: ::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        debug_assert_eq!(node.kind(), "interpolation_start");
        Ok(Self {
            span: ::treesitter_types::Span::from(node),
            text: node.utf8_text(src)?,
        })
    }
}
impl<'tree> ::treesitter_types::LeafNode<'tree> for InterpolationStart<'tree> {
    fn text(&self) -> &'tree str {
        self.text
    }
}
impl ::treesitter_types::Spanned for InterpolationStart<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        self.span
    }
}
#[derive(Debug, Clone)]
pub struct NullLiteral<'tree> {
    pub span: ::treesitter_types::Span,
    text: &'tree str,
}
impl<'tree> ::treesitter_types::FromNode<'tree> for NullLiteral<'tree> {
    fn from_node(
        node: ::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        debug_assert_eq!(node.kind(), "null_literal");
        Ok(Self {
            span: ::treesitter_types::Span::from(node),
            text: node.utf8_text(src)?,
        })
    }
}
impl<'tree> ::treesitter_types::LeafNode<'tree> for NullLiteral<'tree> {
    fn text(&self) -> &'tree str {
        self.text
    }
}
impl ::treesitter_types::Spanned for NullLiteral<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        self.span
    }
}
#[derive(Debug, Clone)]
pub struct PredefinedType<'tree> {
    pub span: ::treesitter_types::Span,
    text: &'tree str,
}
impl<'tree> ::treesitter_types::FromNode<'tree> for PredefinedType<'tree> {
    fn from_node(
        node: ::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        debug_assert_eq!(node.kind(), "predefined_type");
        Ok(Self {
            span: ::treesitter_types::Span::from(node),
            text: node.utf8_text(src)?,
        })
    }
}
impl<'tree> ::treesitter_types::LeafNode<'tree> for PredefinedType<'tree> {
    fn text(&self) -> &'tree str {
        self.text
    }
}
impl ::treesitter_types::Spanned for PredefinedType<'_> {
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
pub struct RawStringEnd<'tree> {
    pub span: ::treesitter_types::Span,
    text: &'tree str,
}
impl<'tree> ::treesitter_types::FromNode<'tree> for RawStringEnd<'tree> {
    fn from_node(
        node: ::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        debug_assert_eq!(node.kind(), "raw_string_end");
        Ok(Self {
            span: ::treesitter_types::Span::from(node),
            text: node.utf8_text(src)?,
        })
    }
}
impl<'tree> ::treesitter_types::LeafNode<'tree> for RawStringEnd<'tree> {
    fn text(&self) -> &'tree str {
        self.text
    }
}
impl ::treesitter_types::Spanned for RawStringEnd<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        self.span
    }
}
#[derive(Debug, Clone)]
pub struct RawStringStart<'tree> {
    pub span: ::treesitter_types::Span,
    text: &'tree str,
}
impl<'tree> ::treesitter_types::FromNode<'tree> for RawStringStart<'tree> {
    fn from_node(
        node: ::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        debug_assert_eq!(node.kind(), "raw_string_start");
        Ok(Self {
            span: ::treesitter_types::Span::from(node),
            text: node.utf8_text(src)?,
        })
    }
}
impl<'tree> ::treesitter_types::LeafNode<'tree> for RawStringStart<'tree> {
    fn text(&self) -> &'tree str {
        self.text
    }
}
impl ::treesitter_types::Spanned for RawStringStart<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        self.span
    }
}
#[derive(Debug, Clone)]
pub struct RealLiteral<'tree> {
    pub span: ::treesitter_types::Span,
    text: &'tree str,
}
impl<'tree> ::treesitter_types::FromNode<'tree> for RealLiteral<'tree> {
    fn from_node(
        node: ::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        debug_assert_eq!(node.kind(), "real_literal");
        Ok(Self {
            span: ::treesitter_types::Span::from(node),
            text: node.utf8_text(src)?,
        })
    }
}
impl<'tree> ::treesitter_types::LeafNode<'tree> for RealLiteral<'tree> {
    fn text(&self) -> &'tree str {
        self.text
    }
}
impl ::treesitter_types::Spanned for RealLiteral<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        self.span
    }
}
#[derive(Debug, Clone)]
pub struct ShebangDirective<'tree> {
    pub span: ::treesitter_types::Span,
    text: &'tree str,
}
impl<'tree> ::treesitter_types::FromNode<'tree> for ShebangDirective<'tree> {
    fn from_node(
        node: ::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        debug_assert_eq!(node.kind(), "shebang_directive");
        Ok(Self {
            span: ::treesitter_types::Span::from(node),
            text: node.utf8_text(src)?,
        })
    }
}
impl<'tree> ::treesitter_types::LeafNode<'tree> for ShebangDirective<'tree> {
    fn text(&self) -> &'tree str {
        self.text
    }
}
impl ::treesitter_types::Spanned for ShebangDirective<'_> {
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
pub struct StringLiteralContent<'tree> {
    pub span: ::treesitter_types::Span,
    text: &'tree str,
}
impl<'tree> ::treesitter_types::FromNode<'tree> for StringLiteralContent<'tree> {
    fn from_node(
        node: ::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        debug_assert_eq!(node.kind(), "string_literal_content");
        Ok(Self {
            span: ::treesitter_types::Span::from(node),
            text: node.utf8_text(src)?,
        })
    }
}
impl<'tree> ::treesitter_types::LeafNode<'tree> for StringLiteralContent<'tree> {
    fn text(&self) -> &'tree str {
        self.text
    }
}
impl ::treesitter_types::Spanned for StringLiteralContent<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        self.span
    }
}
#[derive(Debug, Clone)]
pub struct StringLiteralEncoding<'tree> {
    pub span: ::treesitter_types::Span,
    text: &'tree str,
}
impl<'tree> ::treesitter_types::FromNode<'tree> for StringLiteralEncoding<'tree> {
    fn from_node(
        node: ::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        debug_assert_eq!(node.kind(), "string_literal_encoding");
        Ok(Self {
            span: ::treesitter_types::Span::from(node),
            text: node.utf8_text(src)?,
        })
    }
}
impl<'tree> ::treesitter_types::LeafNode<'tree> for StringLiteralEncoding<'tree> {
    fn text(&self) -> &'tree str {
        self.text
    }
}
impl ::treesitter_types::Spanned for StringLiteralEncoding<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        self.span
    }
}
#[derive(Debug, Clone)]
pub struct VerbatimStringLiteral<'tree> {
    pub span: ::treesitter_types::Span,
    text: &'tree str,
}
impl<'tree> ::treesitter_types::FromNode<'tree> for VerbatimStringLiteral<'tree> {
    fn from_node(
        node: ::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        debug_assert_eq!(node.kind(), "verbatim_string_literal");
        Ok(Self {
            span: ::treesitter_types::Span::from(node),
            text: node.utf8_text(src)?,
        })
    }
}
impl<'tree> ::treesitter_types::LeafNode<'tree> for VerbatimStringLiteral<'tree> {
    fn text(&self) -> &'tree str {
        self.text
    }
}
impl ::treesitter_types::Spanned for VerbatimStringLiteral<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        self.span
    }
}
#[derive(Debug, Clone)]
pub enum AccessorDeclarationBody<'tree> {
    ArrowExpressionClause(::std::boxed::Box<ArrowExpressionClause<'tree>>),
    Block(::std::boxed::Box<Block<'tree>>),
}
impl<'tree> ::treesitter_types::FromNode<'tree> for AccessorDeclarationBody<'tree> {
    #[allow(clippy::collapsible_else_if)]
    fn from_node(
        node: ::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        match node.kind() {
            "arrow_expression_clause" => Ok(Self::ArrowExpressionClause(::std::boxed::Box::new(
                <ArrowExpressionClause as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "block" => Ok(Self::Block(::std::boxed::Box::new(
                <Block as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            other => Err(::treesitter_types::ParseError::unexpected_kind(other, node)),
        }
    }
}
impl ::treesitter_types::Spanned for AccessorDeclarationBody<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        match self {
            Self::ArrowExpressionClause(inner) => inner.span(),
            Self::Block(inner) => inner.span(),
        }
    }
}
#[derive(Debug, Clone)]
pub enum AccessorDeclarationName<'tree> {
    Add(::treesitter_types::Span),
    Get(::treesitter_types::Span),
    Identifier(::std::boxed::Box<Identifier<'tree>>),
    Init(::treesitter_types::Span),
    Remove(::treesitter_types::Span),
    Set(::treesitter_types::Span),
}
impl<'tree> ::treesitter_types::FromNode<'tree> for AccessorDeclarationName<'tree> {
    #[allow(clippy::collapsible_else_if)]
    fn from_node(
        node: ::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        match node.kind() {
            "add" => Ok(Self::Add(::treesitter_types::Span::from(node))),
            "get" => Ok(Self::Get(::treesitter_types::Span::from(node))),
            "identifier" => Ok(Self::Identifier(::std::boxed::Box::new(
                <Identifier as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "init" => Ok(Self::Init(::treesitter_types::Span::from(node))),
            "remove" => Ok(Self::Remove(::treesitter_types::Span::from(node))),
            "set" => Ok(Self::Set(::treesitter_types::Span::from(node))),
            other => Err(::treesitter_types::ParseError::unexpected_kind(other, node)),
        }
    }
}
impl ::treesitter_types::Spanned for AccessorDeclarationName<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        match self {
            Self::Add(span) => *span,
            Self::Get(span) => *span,
            Self::Identifier(inner) => inner.span(),
            Self::Init(span) => *span,
            Self::Remove(span) => *span,
            Self::Set(span) => *span,
        }
    }
}
#[derive(Debug, Clone)]
pub enum AccessorDeclarationChildren<'tree> {
    AttributeList(::std::boxed::Box<AttributeList<'tree>>),
    Modifier(::std::boxed::Box<Modifier<'tree>>),
    PreprocIfInAttributeList(::std::boxed::Box<PreprocIfInAttributeList<'tree>>),
}
impl<'tree> ::treesitter_types::FromNode<'tree> for AccessorDeclarationChildren<'tree> {
    #[allow(clippy::collapsible_else_if)]
    fn from_node(
        node: ::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        match node.kind() {
            "attribute_list" => Ok(Self::AttributeList(::std::boxed::Box::new(
                <AttributeList as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "modifier" => Ok(Self::Modifier(::std::boxed::Box::new(
                <Modifier as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "preproc_if_in_attribute_list" => {
                Ok(Self::PreprocIfInAttributeList(::std::boxed::Box::new(
                    <PreprocIfInAttributeList as ::treesitter_types::FromNode>::from_node(
                        node, src,
                    )?,
                )))
            }
            other => Err(::treesitter_types::ParseError::unexpected_kind(other, node)),
        }
    }
}
impl ::treesitter_types::Spanned for AccessorDeclarationChildren<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        match self {
            Self::AttributeList(inner) => inner.span(),
            Self::Modifier(inner) => inner.span(),
            Self::PreprocIfInAttributeList(inner) => inner.span(),
        }
    }
}
#[derive(Debug, Clone)]
pub enum AliasQualifiedNameName<'tree> {
    GenericName(::std::boxed::Box<GenericName<'tree>>),
    Identifier(::std::boxed::Box<Identifier<'tree>>),
}
impl<'tree> ::treesitter_types::FromNode<'tree> for AliasQualifiedNameName<'tree> {
    #[allow(clippy::collapsible_else_if)]
    fn from_node(
        node: ::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        match node.kind() {
            "generic_name" => Ok(Self::GenericName(::std::boxed::Box::new(
                <GenericName as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "identifier" => Ok(Self::Identifier(::std::boxed::Box::new(
                <Identifier as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            other => Err(::treesitter_types::ParseError::unexpected_kind(other, node)),
        }
    }
}
impl ::treesitter_types::Spanned for AliasQualifiedNameName<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        match self {
            Self::GenericName(inner) => inner.span(),
            Self::Identifier(inner) => inner.span(),
        }
    }
}
#[derive(Debug, Clone)]
pub enum AndPatternOperator {
    And(::treesitter_types::Span),
}
impl<'tree> ::treesitter_types::FromNode<'tree> for AndPatternOperator {
    #[allow(clippy::collapsible_else_if)]
    fn from_node(
        node: ::tree_sitter::Node<'tree>,
        _src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        match node.kind() {
            "and" => Ok(Self::And(::treesitter_types::Span::from(node))),
            other => Err(::treesitter_types::ParseError::unexpected_kind(other, node)),
        }
    }
}
impl ::treesitter_types::Spanned for AndPatternOperator {
    fn span(&self) -> ::treesitter_types::Span {
        match self {
            Self::And(span) => *span,
        }
    }
}
#[derive(Debug, Clone)]
pub enum AnonymousMethodExpressionChildren<'tree> {
    Block(::std::boxed::Box<Block<'tree>>),
    Modifier(::std::boxed::Box<Modifier<'tree>>),
}
impl<'tree> ::treesitter_types::FromNode<'tree> for AnonymousMethodExpressionChildren<'tree> {
    #[allow(clippy::collapsible_else_if)]
    fn from_node(
        node: ::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        match node.kind() {
            "block" => Ok(Self::Block(::std::boxed::Box::new(
                <Block as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "modifier" => Ok(Self::Modifier(::std::boxed::Box::new(
                <Modifier as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            other => Err(::treesitter_types::ParseError::unexpected_kind(other, node)),
        }
    }
}
impl ::treesitter_types::Spanned for AnonymousMethodExpressionChildren<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        match self {
            Self::Block(inner) => inner.span(),
            Self::Modifier(inner) => inner.span(),
        }
    }
}
#[derive(Debug, Clone)]
pub enum AnonymousObjectCreationExpressionChildren<'tree> {
    Expression(::std::boxed::Box<Expression<'tree>>),
    Identifier(::std::boxed::Box<Identifier<'tree>>),
}
impl<'tree> ::treesitter_types::FromNode<'tree>
    for AnonymousObjectCreationExpressionChildren<'tree>
{
    #[allow(clippy::collapsible_else_if)]
    fn from_node(
        node: ::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        match node.kind() {
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
impl ::treesitter_types::Spanned for AnonymousObjectCreationExpressionChildren<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        match self {
            Self::Expression(inner) => inner.span(),
            Self::Identifier(inner) => inner.span(),
        }
    }
}
#[derive(Debug, Clone)]
pub enum ArgumentChildren<'tree> {
    DeclarationExpression(::std::boxed::Box<DeclarationExpression<'tree>>),
    Expression(::std::boxed::Box<Expression<'tree>>),
}
impl<'tree> ::treesitter_types::FromNode<'tree> for ArgumentChildren<'tree> {
    #[allow(clippy::collapsible_else_if)]
    fn from_node(
        node: ::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        match node.kind() {
            "declaration_expression" => Ok(Self::DeclarationExpression(::std::boxed::Box::new(
                <DeclarationExpression as ::treesitter_types::FromNode>::from_node(node, src)?,
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
impl ::treesitter_types::Spanned for ArgumentChildren<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        match self {
            Self::DeclarationExpression(inner) => inner.span(),
            Self::Expression(inner) => inner.span(),
        }
    }
}
#[derive(Debug, Clone)]
pub enum ArrayTypeType<'tree> {
    AliasQualifiedName(::std::boxed::Box<AliasQualifiedName<'tree>>),
    ArrayType(::std::boxed::Box<ArrayType<'tree>>),
    FunctionPointerType(::std::boxed::Box<FunctionPointerType<'tree>>),
    GenericName(::std::boxed::Box<GenericName<'tree>>),
    Identifier(::std::boxed::Box<Identifier<'tree>>),
    NullableType(::std::boxed::Box<NullableType<'tree>>),
    PointerType(::std::boxed::Box<PointerType<'tree>>),
    PredefinedType(::std::boxed::Box<PredefinedType<'tree>>),
    QualifiedName(::std::boxed::Box<QualifiedName<'tree>>),
    TupleType(::std::boxed::Box<TupleType<'tree>>),
}
impl<'tree> ::treesitter_types::FromNode<'tree> for ArrayTypeType<'tree> {
    #[allow(clippy::collapsible_else_if)]
    fn from_node(
        node: ::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        match node.kind() {
            "alias_qualified_name" => Ok(Self::AliasQualifiedName(::std::boxed::Box::new(
                <AliasQualifiedName as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "array_type" => Ok(Self::ArrayType(::std::boxed::Box::new(
                <ArrayType as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "function_pointer_type" => Ok(Self::FunctionPointerType(::std::boxed::Box::new(
                <FunctionPointerType as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "generic_name" => Ok(Self::GenericName(::std::boxed::Box::new(
                <GenericName as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "identifier" => Ok(Self::Identifier(::std::boxed::Box::new(
                <Identifier as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "nullable_type" => Ok(Self::NullableType(::std::boxed::Box::new(
                <NullableType as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "pointer_type" => Ok(Self::PointerType(::std::boxed::Box::new(
                <PointerType as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "predefined_type" => Ok(Self::PredefinedType(::std::boxed::Box::new(
                <PredefinedType as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "qualified_name" => Ok(Self::QualifiedName(::std::boxed::Box::new(
                <QualifiedName as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "tuple_type" => Ok(Self::TupleType(::std::boxed::Box::new(
                <TupleType as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            other => Err(::treesitter_types::ParseError::unexpected_kind(other, node)),
        }
    }
}
impl ::treesitter_types::Spanned for ArrayTypeType<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        match self {
            Self::AliasQualifiedName(inner) => inner.span(),
            Self::ArrayType(inner) => inner.span(),
            Self::FunctionPointerType(inner) => inner.span(),
            Self::GenericName(inner) => inner.span(),
            Self::Identifier(inner) => inner.span(),
            Self::NullableType(inner) => inner.span(),
            Self::PointerType(inner) => inner.span(),
            Self::PredefinedType(inner) => inner.span(),
            Self::QualifiedName(inner) => inner.span(),
            Self::TupleType(inner) => inner.span(),
        }
    }
}
#[derive(Debug, Clone)]
pub enum AsExpressionOperator {
    As(::treesitter_types::Span),
}
impl<'tree> ::treesitter_types::FromNode<'tree> for AsExpressionOperator {
    #[allow(clippy::collapsible_else_if)]
    fn from_node(
        node: ::tree_sitter::Node<'tree>,
        _src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        match node.kind() {
            "as" => Ok(Self::As(::treesitter_types::Span::from(node))),
            other => Err(::treesitter_types::ParseError::unexpected_kind(other, node)),
        }
    }
}
impl ::treesitter_types::Spanned for AsExpressionOperator {
    fn span(&self) -> ::treesitter_types::Span {
        match self {
            Self::As(span) => *span,
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
    GtGtGtEq(::treesitter_types::Span),
    QuestionQuestionEq(::treesitter_types::Span),
    CaretEq(::treesitter_types::Span),
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
            ">>>=" => Ok(Self::GtGtGtEq(::treesitter_types::Span::from(node))),
            "??=" => Ok(Self::QuestionQuestionEq(::treesitter_types::Span::from(
                node,
            ))),
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
            Self::GtGtGtEq(span) => *span,
            Self::QuestionQuestionEq(span) => *span,
            Self::CaretEq(span) => *span,
            Self::PipeEq(span) => *span,
        }
    }
}
#[derive(Debug, Clone)]
pub enum AttributeName<'tree> {
    AliasQualifiedName(::std::boxed::Box<AliasQualifiedName<'tree>>),
    GenericName(::std::boxed::Box<GenericName<'tree>>),
    Identifier(::std::boxed::Box<Identifier<'tree>>),
    QualifiedName(::std::boxed::Box<QualifiedName<'tree>>),
}
impl<'tree> ::treesitter_types::FromNode<'tree> for AttributeName<'tree> {
    #[allow(clippy::collapsible_else_if)]
    fn from_node(
        node: ::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        match node.kind() {
            "alias_qualified_name" => Ok(Self::AliasQualifiedName(::std::boxed::Box::new(
                <AliasQualifiedName as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "generic_name" => Ok(Self::GenericName(::std::boxed::Box::new(
                <GenericName as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "identifier" => Ok(Self::Identifier(::std::boxed::Box::new(
                <Identifier as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "qualified_name" => Ok(Self::QualifiedName(::std::boxed::Box::new(
                <QualifiedName as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            other => Err(::treesitter_types::ParseError::unexpected_kind(other, node)),
        }
    }
}
impl ::treesitter_types::Spanned for AttributeName<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        match self {
            Self::AliasQualifiedName(inner) => inner.span(),
            Self::GenericName(inner) => inner.span(),
            Self::Identifier(inner) => inner.span(),
            Self::QualifiedName(inner) => inner.span(),
        }
    }
}
#[derive(Debug, Clone)]
pub enum AttributeListChildren<'tree> {
    Attribute(::std::boxed::Box<Attribute<'tree>>),
    AttributeTargetSpecifier(::std::boxed::Box<AttributeTargetSpecifier<'tree>>),
}
impl<'tree> ::treesitter_types::FromNode<'tree> for AttributeListChildren<'tree> {
    #[allow(clippy::collapsible_else_if)]
    fn from_node(
        node: ::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        match node.kind() {
            "attribute" => Ok(Self::Attribute(::std::boxed::Box::new(
                <Attribute as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "attribute_target_specifier" => {
                Ok(Self::AttributeTargetSpecifier(::std::boxed::Box::new(
                    <AttributeTargetSpecifier as ::treesitter_types::FromNode>::from_node(
                        node, src,
                    )?,
                )))
            }
            other => Err(::treesitter_types::ParseError::unexpected_kind(other, node)),
        }
    }
}
impl ::treesitter_types::Spanned for AttributeListChildren<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        match self {
            Self::Attribute(inner) => inner.span(),
            Self::AttributeTargetSpecifier(inner) => inner.span(),
        }
    }
}
#[derive(Debug, Clone)]
pub enum BaseListChildren<'tree> {
    ArgumentList(::std::boxed::Box<ArgumentList<'tree>>),
    PrimaryConstructorBaseType(::std::boxed::Box<PrimaryConstructorBaseType<'tree>>),
    Type(::std::boxed::Box<Type<'tree>>),
}
impl<'tree> ::treesitter_types::FromNode<'tree> for BaseListChildren<'tree> {
    #[allow(clippy::collapsible_else_if)]
    fn from_node(
        node: ::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        match node.kind() {
            "argument_list" => Ok(Self::ArgumentList(::std::boxed::Box::new(
                <ArgumentList as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "primary_constructor_base_type" => {
                Ok(Self::PrimaryConstructorBaseType(::std::boxed::Box::new(
                    <PrimaryConstructorBaseType as ::treesitter_types::FromNode>::from_node(
                        node, src,
                    )?,
                )))
            }
            _other => {
                if let Ok(v) = <Type as ::treesitter_types::FromNode>::from_node(node, src) {
                    Ok(Self::Type(::std::boxed::Box::new(v)))
                } else {
                    Err(::treesitter_types::ParseError::unexpected_kind(
                        _other, node,
                    ))
                }
            }
        }
    }
}
impl ::treesitter_types::Spanned for BaseListChildren<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        match self {
            Self::ArgumentList(inner) => inner.span(),
            Self::PrimaryConstructorBaseType(inner) => inner.span(),
            Self::Type(inner) => inner.span(),
        }
    }
}
#[derive(Debug, Clone)]
pub enum BinaryExpressionLeft<'tree> {
    BinaryExpression(::std::boxed::Box<BinaryExpression<'tree>>),
    BooleanLiteral(::std::boxed::Box<BooleanLiteral<'tree>>),
    CharacterLiteral(::std::boxed::Box<CharacterLiteral<'tree>>),
    Expression(::std::boxed::Box<Expression<'tree>>),
    Identifier(::std::boxed::Box<Identifier<'tree>>),
    IntegerLiteral(::std::boxed::Box<IntegerLiteral<'tree>>),
    ParenthesizedExpression(::std::boxed::Box<ParenthesizedExpression<'tree>>),
    UnaryExpression(::std::boxed::Box<UnaryExpression<'tree>>),
}
impl<'tree> ::treesitter_types::FromNode<'tree> for BinaryExpressionLeft<'tree> {
    #[allow(clippy::collapsible_else_if)]
    fn from_node(
        node: ::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        match node.kind() {
            "binary_expression" => Ok(Self::BinaryExpression(::std::boxed::Box::new(
                <BinaryExpression as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "boolean_literal" => Ok(Self::BooleanLiteral(::std::boxed::Box::new(
                <BooleanLiteral as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "character_literal" => Ok(Self::CharacterLiteral(::std::boxed::Box::new(
                <CharacterLiteral as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "identifier" => Ok(Self::Identifier(::std::boxed::Box::new(
                <Identifier as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "integer_literal" => Ok(Self::IntegerLiteral(::std::boxed::Box::new(
                <IntegerLiteral as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "parenthesized_expression" => {
                Ok(Self::ParenthesizedExpression(::std::boxed::Box::new(
                    <ParenthesizedExpression as ::treesitter_types::FromNode>::from_node(
                        node, src,
                    )?,
                )))
            }
            "unary_expression" => Ok(Self::UnaryExpression(::std::boxed::Box::new(
                <UnaryExpression as ::treesitter_types::FromNode>::from_node(node, src)?,
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
            Self::BinaryExpression(inner) => inner.span(),
            Self::BooleanLiteral(inner) => inner.span(),
            Self::CharacterLiteral(inner) => inner.span(),
            Self::Expression(inner) => inner.span(),
            Self::Identifier(inner) => inner.span(),
            Self::IntegerLiteral(inner) => inner.span(),
            Self::ParenthesizedExpression(inner) => inner.span(),
            Self::UnaryExpression(inner) => inner.span(),
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
    EqEq(::treesitter_types::Span),
    Gt(::treesitter_types::Span),
    GtEq(::treesitter_types::Span),
    Shr(::treesitter_types::Span),
    GtGtGt(::treesitter_types::Span),
    QuestionQuestion(::treesitter_types::Span),
    Caret(::treesitter_types::Span),
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
            ">>>" => Ok(Self::GtGtGt(::treesitter_types::Span::from(node))),
            "??" => Ok(Self::QuestionQuestion(::treesitter_types::Span::from(node))),
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
            Self::GtGtGt(span) => *span,
            Self::QuestionQuestion(span) => *span,
            Self::Caret(span) => *span,
            Self::Pipe(span) => *span,
            Self::PipePipe(span) => *span,
        }
    }
}
#[derive(Debug, Clone)]
pub enum BinaryExpressionRight<'tree> {
    BinaryExpression(::std::boxed::Box<BinaryExpression<'tree>>),
    BooleanLiteral(::std::boxed::Box<BooleanLiteral<'tree>>),
    CharacterLiteral(::std::boxed::Box<CharacterLiteral<'tree>>),
    Expression(::std::boxed::Box<Expression<'tree>>),
    Identifier(::std::boxed::Box<Identifier<'tree>>),
    IntegerLiteral(::std::boxed::Box<IntegerLiteral<'tree>>),
    ParenthesizedExpression(::std::boxed::Box<ParenthesizedExpression<'tree>>),
    UnaryExpression(::std::boxed::Box<UnaryExpression<'tree>>),
}
impl<'tree> ::treesitter_types::FromNode<'tree> for BinaryExpressionRight<'tree> {
    #[allow(clippy::collapsible_else_if)]
    fn from_node(
        node: ::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        match node.kind() {
            "binary_expression" => Ok(Self::BinaryExpression(::std::boxed::Box::new(
                <BinaryExpression as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "boolean_literal" => Ok(Self::BooleanLiteral(::std::boxed::Box::new(
                <BooleanLiteral as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "character_literal" => Ok(Self::CharacterLiteral(::std::boxed::Box::new(
                <CharacterLiteral as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "identifier" => Ok(Self::Identifier(::std::boxed::Box::new(
                <Identifier as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "integer_literal" => Ok(Self::IntegerLiteral(::std::boxed::Box::new(
                <IntegerLiteral as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "parenthesized_expression" => {
                Ok(Self::ParenthesizedExpression(::std::boxed::Box::new(
                    <ParenthesizedExpression as ::treesitter_types::FromNode>::from_node(
                        node, src,
                    )?,
                )))
            }
            "unary_expression" => Ok(Self::UnaryExpression(::std::boxed::Box::new(
                <UnaryExpression as ::treesitter_types::FromNode>::from_node(node, src)?,
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
            Self::BinaryExpression(inner) => inner.span(),
            Self::BooleanLiteral(inner) => inner.span(),
            Self::CharacterLiteral(inner) => inner.span(),
            Self::Expression(inner) => inner.span(),
            Self::Identifier(inner) => inner.span(),
            Self::IntegerLiteral(inner) => inner.span(),
            Self::ParenthesizedExpression(inner) => inner.span(),
            Self::UnaryExpression(inner) => inner.span(),
        }
    }
}
#[derive(Debug, Clone)]
pub enum BracketedParameterListChildren<'tree> {
    AttributeList(::std::boxed::Box<AttributeList<'tree>>),
    Parameter(::std::boxed::Box<Parameter<'tree>>),
    PreprocIfInAttributeList(::std::boxed::Box<PreprocIfInAttributeList<'tree>>),
}
impl<'tree> ::treesitter_types::FromNode<'tree> for BracketedParameterListChildren<'tree> {
    #[allow(clippy::collapsible_else_if)]
    fn from_node(
        node: ::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        match node.kind() {
            "attribute_list" => Ok(Self::AttributeList(::std::boxed::Box::new(
                <AttributeList as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "parameter" => Ok(Self::Parameter(::std::boxed::Box::new(
                <Parameter as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "preproc_if_in_attribute_list" => {
                Ok(Self::PreprocIfInAttributeList(::std::boxed::Box::new(
                    <PreprocIfInAttributeList as ::treesitter_types::FromNode>::from_node(
                        node, src,
                    )?,
                )))
            }
            other => Err(::treesitter_types::ParseError::unexpected_kind(other, node)),
        }
    }
}
impl ::treesitter_types::Spanned for BracketedParameterListChildren<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        match self {
            Self::AttributeList(inner) => inner.span(),
            Self::Parameter(inner) => inner.span(),
            Self::PreprocIfInAttributeList(inner) => inner.span(),
        }
    }
}
#[derive(Debug, Clone)]
pub enum CatchClauseChildren<'tree> {
    CatchDeclaration(::std::boxed::Box<CatchDeclaration<'tree>>),
    CatchFilterClause(::std::boxed::Box<CatchFilterClause<'tree>>),
}
impl<'tree> ::treesitter_types::FromNode<'tree> for CatchClauseChildren<'tree> {
    #[allow(clippy::collapsible_else_if)]
    fn from_node(
        node: ::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        match node.kind() {
            "catch_declaration" => Ok(Self::CatchDeclaration(::std::boxed::Box::new(
                <CatchDeclaration as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "catch_filter_clause" => Ok(Self::CatchFilterClause(::std::boxed::Box::new(
                <CatchFilterClause as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            other => Err(::treesitter_types::ParseError::unexpected_kind(other, node)),
        }
    }
}
impl ::treesitter_types::Spanned for CatchClauseChildren<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        match self {
            Self::CatchDeclaration(inner) => inner.span(),
            Self::CatchFilterClause(inner) => inner.span(),
        }
    }
}
#[derive(Debug, Clone)]
pub enum CharacterLiteralChildren<'tree> {
    CharacterLiteralContent(::std::boxed::Box<CharacterLiteralContent<'tree>>),
    EscapeSequence(::std::boxed::Box<EscapeSequence<'tree>>),
}
impl<'tree> ::treesitter_types::FromNode<'tree> for CharacterLiteralChildren<'tree> {
    #[allow(clippy::collapsible_else_if)]
    fn from_node(
        node: ::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        match node.kind() {
            "character_literal_content" => {
                Ok(Self::CharacterLiteralContent(::std::boxed::Box::new(
                    <CharacterLiteralContent as ::treesitter_types::FromNode>::from_node(
                        node, src,
                    )?,
                )))
            }
            "escape_sequence" => Ok(Self::EscapeSequence(::std::boxed::Box::new(
                <EscapeSequence as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            other => Err(::treesitter_types::ParseError::unexpected_kind(other, node)),
        }
    }
}
impl ::treesitter_types::Spanned for CharacterLiteralChildren<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        match self {
            Self::CharacterLiteralContent(inner) => inner.span(),
            Self::EscapeSequence(inner) => inner.span(),
        }
    }
}
#[derive(Debug, Clone)]
pub enum ClassDeclarationChildren<'tree> {
    AttributeList(::std::boxed::Box<AttributeList<'tree>>),
    BaseList(::std::boxed::Box<BaseList<'tree>>),
    Modifier(::std::boxed::Box<Modifier<'tree>>),
    ParameterList(::std::boxed::Box<ParameterList<'tree>>),
    PreprocIfInAttributeList(::std::boxed::Box<PreprocIfInAttributeList<'tree>>),
    TypeParameterConstraintsClause(::std::boxed::Box<TypeParameterConstraintsClause<'tree>>),
    TypeParameterList(::std::boxed::Box<TypeParameterList<'tree>>),
}
impl<'tree> ::treesitter_types::FromNode<'tree> for ClassDeclarationChildren<'tree> {
    #[allow(clippy::collapsible_else_if)]
    fn from_node(
        node: ::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        match node.kind() {
            "attribute_list" => Ok(Self::AttributeList(::std::boxed::Box::new(
                <AttributeList as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "base_list" => Ok(Self::BaseList(::std::boxed::Box::new(
                <BaseList as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "modifier" => Ok(Self::Modifier(::std::boxed::Box::new(
                <Modifier as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "parameter_list" => Ok(Self::ParameterList(::std::boxed::Box::new(
                <ParameterList as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "preproc_if_in_attribute_list" => {
                Ok(Self::PreprocIfInAttributeList(::std::boxed::Box::new(
                    <PreprocIfInAttributeList as ::treesitter_types::FromNode>::from_node(
                        node, src,
                    )?,
                )))
            }
            "type_parameter_constraints_clause" => Ok(Self::TypeParameterConstraintsClause(
                ::std::boxed::Box::new(
                    <TypeParameterConstraintsClause as ::treesitter_types::FromNode>::from_node(
                        node, src,
                    )?,
                ),
            )),
            "type_parameter_list" => Ok(Self::TypeParameterList(::std::boxed::Box::new(
                <TypeParameterList as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            other => Err(::treesitter_types::ParseError::unexpected_kind(other, node)),
        }
    }
}
impl ::treesitter_types::Spanned for ClassDeclarationChildren<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        match self {
            Self::AttributeList(inner) => inner.span(),
            Self::BaseList(inner) => inner.span(),
            Self::Modifier(inner) => inner.span(),
            Self::ParameterList(inner) => inner.span(),
            Self::PreprocIfInAttributeList(inner) => inner.span(),
            Self::TypeParameterConstraintsClause(inner) => inner.span(),
            Self::TypeParameterList(inner) => inner.span(),
        }
    }
}
#[derive(Debug, Clone)]
pub enum CompilationUnitChildren<'tree> {
    ExternAliasDirective(::std::boxed::Box<ExternAliasDirective<'tree>>),
    FileScopedNamespaceDeclaration(::std::boxed::Box<FileScopedNamespaceDeclaration<'tree>>),
    GlobalAttribute(::std::boxed::Box<GlobalAttribute<'tree>>),
    GlobalStatement(::std::boxed::Box<GlobalStatement<'tree>>),
    NamespaceDeclaration(::std::boxed::Box<NamespaceDeclaration<'tree>>),
    PreprocIf(::std::boxed::Box<PreprocIf<'tree>>),
    ShebangDirective(::std::boxed::Box<ShebangDirective<'tree>>),
    TypeDeclaration(::std::boxed::Box<TypeDeclaration<'tree>>),
    UsingDirective(::std::boxed::Box<UsingDirective<'tree>>),
}
impl<'tree> ::treesitter_types::FromNode<'tree> for CompilationUnitChildren<'tree> {
    #[allow(clippy::collapsible_else_if)]
    fn from_node(
        node: ::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        match node.kind() {
            "extern_alias_directive" => Ok(Self::ExternAliasDirective(::std::boxed::Box::new(
                <ExternAliasDirective as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "file_scoped_namespace_declaration" => Ok(Self::FileScopedNamespaceDeclaration(
                ::std::boxed::Box::new(
                    <FileScopedNamespaceDeclaration as ::treesitter_types::FromNode>::from_node(
                        node, src,
                    )?,
                ),
            )),
            "global_attribute" => Ok(Self::GlobalAttribute(::std::boxed::Box::new(
                <GlobalAttribute as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "global_statement" => Ok(Self::GlobalStatement(::std::boxed::Box::new(
                <GlobalStatement as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "namespace_declaration" => Ok(Self::NamespaceDeclaration(::std::boxed::Box::new(
                <NamespaceDeclaration as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "preproc_if" => Ok(Self::PreprocIf(::std::boxed::Box::new(
                <PreprocIf as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "shebang_directive" => Ok(Self::ShebangDirective(::std::boxed::Box::new(
                <ShebangDirective as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "using_directive" => Ok(Self::UsingDirective(::std::boxed::Box::new(
                <UsingDirective as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            _other => {
                if let Ok(v) =
                    <TypeDeclaration as ::treesitter_types::FromNode>::from_node(node, src)
                {
                    Ok(Self::TypeDeclaration(::std::boxed::Box::new(v)))
                } else {
                    Err(::treesitter_types::ParseError::unexpected_kind(
                        _other, node,
                    ))
                }
            }
        }
    }
}
impl ::treesitter_types::Spanned for CompilationUnitChildren<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        match self {
            Self::ExternAliasDirective(inner) => inner.span(),
            Self::FileScopedNamespaceDeclaration(inner) => inner.span(),
            Self::GlobalAttribute(inner) => inner.span(),
            Self::GlobalStatement(inner) => inner.span(),
            Self::NamespaceDeclaration(inner) => inner.span(),
            Self::PreprocIf(inner) => inner.span(),
            Self::ShebangDirective(inner) => inner.span(),
            Self::TypeDeclaration(inner) => inner.span(),
            Self::UsingDirective(inner) => inner.span(),
        }
    }
}
#[derive(Debug, Clone)]
pub enum ConditionalAccessExpressionChildren<'tree> {
    ElementBindingExpression(::std::boxed::Box<ElementBindingExpression<'tree>>),
    MemberBindingExpression(::std::boxed::Box<MemberBindingExpression<'tree>>),
}
impl<'tree> ::treesitter_types::FromNode<'tree> for ConditionalAccessExpressionChildren<'tree> {
    #[allow(clippy::collapsible_else_if)]
    fn from_node(
        node: ::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        match node.kind() {
            "element_binding_expression" => {
                Ok(Self::ElementBindingExpression(::std::boxed::Box::new(
                    <ElementBindingExpression as ::treesitter_types::FromNode>::from_node(
                        node, src,
                    )?,
                )))
            }
            "member_binding_expression" => {
                Ok(Self::MemberBindingExpression(::std::boxed::Box::new(
                    <MemberBindingExpression as ::treesitter_types::FromNode>::from_node(
                        node, src,
                    )?,
                )))
            }
            other => Err(::treesitter_types::ParseError::unexpected_kind(other, node)),
        }
    }
}
impl ::treesitter_types::Spanned for ConditionalAccessExpressionChildren<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        match self {
            Self::ElementBindingExpression(inner) => inner.span(),
            Self::MemberBindingExpression(inner) => inner.span(),
        }
    }
}
#[derive(Debug, Clone)]
pub enum ConstantPatternChildren<'tree> {
    BinaryExpression(::std::boxed::Box<BinaryExpression<'tree>>),
    CastExpression(::std::boxed::Box<CastExpression<'tree>>),
    DefaultExpression(::std::boxed::Box<DefaultExpression<'tree>>),
    GenericName(::std::boxed::Box<GenericName<'tree>>),
    Identifier(::std::boxed::Box<Identifier<'tree>>),
    InterpolatedStringExpression(::std::boxed::Box<InterpolatedStringExpression<'tree>>),
    InvocationExpression(::std::boxed::Box<InvocationExpression<'tree>>),
    Literal(::std::boxed::Box<Literal<'tree>>),
    MemberAccessExpression(::std::boxed::Box<MemberAccessExpression<'tree>>),
    ParenthesizedExpression(::std::boxed::Box<ParenthesizedExpression<'tree>>),
    PostfixUnaryExpression(::std::boxed::Box<PostfixUnaryExpression<'tree>>),
    PrefixUnaryExpression(::std::boxed::Box<PrefixUnaryExpression<'tree>>),
    SizeofExpression(::std::boxed::Box<SizeofExpression<'tree>>),
    TupleExpression(::std::boxed::Box<TupleExpression<'tree>>),
    TypeofExpression(::std::boxed::Box<TypeofExpression<'tree>>),
}
impl<'tree> ::treesitter_types::FromNode<'tree> for ConstantPatternChildren<'tree> {
    #[allow(clippy::collapsible_else_if)]
    fn from_node(
        node: ::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        match node.kind() {
            "binary_expression" => Ok(Self::BinaryExpression(::std::boxed::Box::new(
                <BinaryExpression as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "cast_expression" => Ok(Self::CastExpression(::std::boxed::Box::new(
                <CastExpression as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "default_expression" => Ok(Self::DefaultExpression(::std::boxed::Box::new(
                <DefaultExpression as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "generic_name" => Ok(Self::GenericName(::std::boxed::Box::new(
                <GenericName as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "identifier" => Ok(Self::Identifier(::std::boxed::Box::new(
                <Identifier as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "interpolated_string_expression" => {
                Ok(Self::InterpolatedStringExpression(::std::boxed::Box::new(
                    <InterpolatedStringExpression as ::treesitter_types::FromNode>::from_node(
                        node, src,
                    )?,
                )))
            }
            "invocation_expression" => Ok(Self::InvocationExpression(::std::boxed::Box::new(
                <InvocationExpression as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "member_access_expression" => Ok(Self::MemberAccessExpression(::std::boxed::Box::new(
                <MemberAccessExpression as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "parenthesized_expression" => {
                Ok(Self::ParenthesizedExpression(::std::boxed::Box::new(
                    <ParenthesizedExpression as ::treesitter_types::FromNode>::from_node(
                        node, src,
                    )?,
                )))
            }
            "postfix_unary_expression" => Ok(Self::PostfixUnaryExpression(::std::boxed::Box::new(
                <PostfixUnaryExpression as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "prefix_unary_expression" => Ok(Self::PrefixUnaryExpression(::std::boxed::Box::new(
                <PrefixUnaryExpression as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "sizeof_expression" => Ok(Self::SizeofExpression(::std::boxed::Box::new(
                <SizeofExpression as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "tuple_expression" => Ok(Self::TupleExpression(::std::boxed::Box::new(
                <TupleExpression as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "typeof_expression" => Ok(Self::TypeofExpression(::std::boxed::Box::new(
                <TypeofExpression as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            _other => {
                if let Ok(v) = <Literal as ::treesitter_types::FromNode>::from_node(node, src) {
                    Ok(Self::Literal(::std::boxed::Box::new(v)))
                } else {
                    Err(::treesitter_types::ParseError::unexpected_kind(
                        _other, node,
                    ))
                }
            }
        }
    }
}
impl ::treesitter_types::Spanned for ConstantPatternChildren<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        match self {
            Self::BinaryExpression(inner) => inner.span(),
            Self::CastExpression(inner) => inner.span(),
            Self::DefaultExpression(inner) => inner.span(),
            Self::GenericName(inner) => inner.span(),
            Self::Identifier(inner) => inner.span(),
            Self::InterpolatedStringExpression(inner) => inner.span(),
            Self::InvocationExpression(inner) => inner.span(),
            Self::Literal(inner) => inner.span(),
            Self::MemberAccessExpression(inner) => inner.span(),
            Self::ParenthesizedExpression(inner) => inner.span(),
            Self::PostfixUnaryExpression(inner) => inner.span(),
            Self::PrefixUnaryExpression(inner) => inner.span(),
            Self::SizeofExpression(inner) => inner.span(),
            Self::TupleExpression(inner) => inner.span(),
            Self::TypeofExpression(inner) => inner.span(),
        }
    }
}
#[derive(Debug, Clone)]
pub enum ConstructorDeclarationBody<'tree> {
    ArrowExpressionClause(::std::boxed::Box<ArrowExpressionClause<'tree>>),
    Block(::std::boxed::Box<Block<'tree>>),
}
impl<'tree> ::treesitter_types::FromNode<'tree> for ConstructorDeclarationBody<'tree> {
    #[allow(clippy::collapsible_else_if)]
    fn from_node(
        node: ::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        match node.kind() {
            "arrow_expression_clause" => Ok(Self::ArrowExpressionClause(::std::boxed::Box::new(
                <ArrowExpressionClause as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "block" => Ok(Self::Block(::std::boxed::Box::new(
                <Block as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            other => Err(::treesitter_types::ParseError::unexpected_kind(other, node)),
        }
    }
}
impl ::treesitter_types::Spanned for ConstructorDeclarationBody<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        match self {
            Self::ArrowExpressionClause(inner) => inner.span(),
            Self::Block(inner) => inner.span(),
        }
    }
}
#[derive(Debug, Clone)]
pub enum ConstructorDeclarationChildren<'tree> {
    AttributeList(::std::boxed::Box<AttributeList<'tree>>),
    ConstructorInitializer(::std::boxed::Box<ConstructorInitializer<'tree>>),
    Modifier(::std::boxed::Box<Modifier<'tree>>),
    PreprocIfInAttributeList(::std::boxed::Box<PreprocIfInAttributeList<'tree>>),
}
impl<'tree> ::treesitter_types::FromNode<'tree> for ConstructorDeclarationChildren<'tree> {
    #[allow(clippy::collapsible_else_if)]
    fn from_node(
        node: ::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        match node.kind() {
            "attribute_list" => Ok(Self::AttributeList(::std::boxed::Box::new(
                <AttributeList as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "constructor_initializer" => Ok(Self::ConstructorInitializer(::std::boxed::Box::new(
                <ConstructorInitializer as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "modifier" => Ok(Self::Modifier(::std::boxed::Box::new(
                <Modifier as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "preproc_if_in_attribute_list" => {
                Ok(Self::PreprocIfInAttributeList(::std::boxed::Box::new(
                    <PreprocIfInAttributeList as ::treesitter_types::FromNode>::from_node(
                        node, src,
                    )?,
                )))
            }
            other => Err(::treesitter_types::ParseError::unexpected_kind(other, node)),
        }
    }
}
impl ::treesitter_types::Spanned for ConstructorDeclarationChildren<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        match self {
            Self::AttributeList(inner) => inner.span(),
            Self::ConstructorInitializer(inner) => inner.span(),
            Self::Modifier(inner) => inner.span(),
            Self::PreprocIfInAttributeList(inner) => inner.span(),
        }
    }
}
#[derive(Debug, Clone)]
pub enum ConversionOperatorDeclarationBody<'tree> {
    ArrowExpressionClause(::std::boxed::Box<ArrowExpressionClause<'tree>>),
    Block(::std::boxed::Box<Block<'tree>>),
}
impl<'tree> ::treesitter_types::FromNode<'tree> for ConversionOperatorDeclarationBody<'tree> {
    #[allow(clippy::collapsible_else_if)]
    fn from_node(
        node: ::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        match node.kind() {
            "arrow_expression_clause" => Ok(Self::ArrowExpressionClause(::std::boxed::Box::new(
                <ArrowExpressionClause as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "block" => Ok(Self::Block(::std::boxed::Box::new(
                <Block as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            other => Err(::treesitter_types::ParseError::unexpected_kind(other, node)),
        }
    }
}
impl ::treesitter_types::Spanned for ConversionOperatorDeclarationBody<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        match self {
            Self::ArrowExpressionClause(inner) => inner.span(),
            Self::Block(inner) => inner.span(),
        }
    }
}
#[derive(Debug, Clone)]
pub enum ConversionOperatorDeclarationChildren<'tree> {
    AttributeList(::std::boxed::Box<AttributeList<'tree>>),
    ExplicitInterfaceSpecifier(::std::boxed::Box<ExplicitInterfaceSpecifier<'tree>>),
    Modifier(::std::boxed::Box<Modifier<'tree>>),
    PreprocIfInAttributeList(::std::boxed::Box<PreprocIfInAttributeList<'tree>>),
}
impl<'tree> ::treesitter_types::FromNode<'tree> for ConversionOperatorDeclarationChildren<'tree> {
    #[allow(clippy::collapsible_else_if)]
    fn from_node(
        node: ::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        match node.kind() {
            "attribute_list" => Ok(Self::AttributeList(::std::boxed::Box::new(
                <AttributeList as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "explicit_interface_specifier" => {
                Ok(Self::ExplicitInterfaceSpecifier(::std::boxed::Box::new(
                    <ExplicitInterfaceSpecifier as ::treesitter_types::FromNode>::from_node(
                        node, src,
                    )?,
                )))
            }
            "modifier" => Ok(Self::Modifier(::std::boxed::Box::new(
                <Modifier as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "preproc_if_in_attribute_list" => {
                Ok(Self::PreprocIfInAttributeList(::std::boxed::Box::new(
                    <PreprocIfInAttributeList as ::treesitter_types::FromNode>::from_node(
                        node, src,
                    )?,
                )))
            }
            other => Err(::treesitter_types::ParseError::unexpected_kind(other, node)),
        }
    }
}
impl ::treesitter_types::Spanned for ConversionOperatorDeclarationChildren<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        match self {
            Self::AttributeList(inner) => inner.span(),
            Self::ExplicitInterfaceSpecifier(inner) => inner.span(),
            Self::Modifier(inner) => inner.span(),
            Self::PreprocIfInAttributeList(inner) => inner.span(),
        }
    }
}
#[derive(Debug, Clone)]
pub enum DeclarationPatternChildren<'tree> {
    Discard(::std::boxed::Box<Discard<'tree>>),
    ParenthesizedVariableDesignation(::std::boxed::Box<ParenthesizedVariableDesignation<'tree>>),
}
impl<'tree> ::treesitter_types::FromNode<'tree> for DeclarationPatternChildren<'tree> {
    #[allow(clippy::collapsible_else_if)]
    fn from_node(
        node: ::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        match node.kind() {
            "discard" => Ok(Self::Discard(::std::boxed::Box::new(
                <Discard as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "parenthesized_variable_designation" => Ok(Self::ParenthesizedVariableDesignation(
                ::std::boxed::Box::new(
                    <ParenthesizedVariableDesignation as ::treesitter_types::FromNode>::from_node(
                        node, src,
                    )?,
                ),
            )),
            other => Err(::treesitter_types::ParseError::unexpected_kind(other, node)),
        }
    }
}
impl ::treesitter_types::Spanned for DeclarationPatternChildren<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        match self {
            Self::Discard(inner) => inner.span(),
            Self::ParenthesizedVariableDesignation(inner) => inner.span(),
        }
    }
}
#[derive(Debug, Clone)]
pub enum DelegateDeclarationChildren<'tree> {
    AttributeList(::std::boxed::Box<AttributeList<'tree>>),
    Modifier(::std::boxed::Box<Modifier<'tree>>),
    PreprocIfInAttributeList(::std::boxed::Box<PreprocIfInAttributeList<'tree>>),
    TypeParameterConstraintsClause(::std::boxed::Box<TypeParameterConstraintsClause<'tree>>),
}
impl<'tree> ::treesitter_types::FromNode<'tree> for DelegateDeclarationChildren<'tree> {
    #[allow(clippy::collapsible_else_if)]
    fn from_node(
        node: ::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        match node.kind() {
            "attribute_list" => Ok(Self::AttributeList(::std::boxed::Box::new(
                <AttributeList as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "modifier" => Ok(Self::Modifier(::std::boxed::Box::new(
                <Modifier as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "preproc_if_in_attribute_list" => {
                Ok(Self::PreprocIfInAttributeList(::std::boxed::Box::new(
                    <PreprocIfInAttributeList as ::treesitter_types::FromNode>::from_node(
                        node, src,
                    )?,
                )))
            }
            "type_parameter_constraints_clause" => Ok(Self::TypeParameterConstraintsClause(
                ::std::boxed::Box::new(
                    <TypeParameterConstraintsClause as ::treesitter_types::FromNode>::from_node(
                        node, src,
                    )?,
                ),
            )),
            other => Err(::treesitter_types::ParseError::unexpected_kind(other, node)),
        }
    }
}
impl ::treesitter_types::Spanned for DelegateDeclarationChildren<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        match self {
            Self::AttributeList(inner) => inner.span(),
            Self::Modifier(inner) => inner.span(),
            Self::PreprocIfInAttributeList(inner) => inner.span(),
            Self::TypeParameterConstraintsClause(inner) => inner.span(),
        }
    }
}
#[derive(Debug, Clone)]
pub enum DestructorDeclarationBody<'tree> {
    ArrowExpressionClause(::std::boxed::Box<ArrowExpressionClause<'tree>>),
    Block(::std::boxed::Box<Block<'tree>>),
}
impl<'tree> ::treesitter_types::FromNode<'tree> for DestructorDeclarationBody<'tree> {
    #[allow(clippy::collapsible_else_if)]
    fn from_node(
        node: ::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        match node.kind() {
            "arrow_expression_clause" => Ok(Self::ArrowExpressionClause(::std::boxed::Box::new(
                <ArrowExpressionClause as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "block" => Ok(Self::Block(::std::boxed::Box::new(
                <Block as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            other => Err(::treesitter_types::ParseError::unexpected_kind(other, node)),
        }
    }
}
impl ::treesitter_types::Spanned for DestructorDeclarationBody<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        match self {
            Self::ArrowExpressionClause(inner) => inner.span(),
            Self::Block(inner) => inner.span(),
        }
    }
}
#[derive(Debug, Clone)]
pub enum DestructorDeclarationChildren<'tree> {
    AttributeList(::std::boxed::Box<AttributeList<'tree>>),
    PreprocIfInAttributeList(::std::boxed::Box<PreprocIfInAttributeList<'tree>>),
}
impl<'tree> ::treesitter_types::FromNode<'tree> for DestructorDeclarationChildren<'tree> {
    #[allow(clippy::collapsible_else_if)]
    fn from_node(
        node: ::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        match node.kind() {
            "attribute_list" => Ok(Self::AttributeList(::std::boxed::Box::new(
                <AttributeList as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "preproc_if_in_attribute_list" => {
                Ok(Self::PreprocIfInAttributeList(::std::boxed::Box::new(
                    <PreprocIfInAttributeList as ::treesitter_types::FromNode>::from_node(
                        node, src,
                    )?,
                )))
            }
            other => Err(::treesitter_types::ParseError::unexpected_kind(other, node)),
        }
    }
}
impl ::treesitter_types::Spanned for DestructorDeclarationChildren<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        match self {
            Self::AttributeList(inner) => inner.span(),
            Self::PreprocIfInAttributeList(inner) => inner.span(),
        }
    }
}
#[derive(Debug, Clone)]
pub enum EnumDeclarationChildren<'tree> {
    AttributeList(::std::boxed::Box<AttributeList<'tree>>),
    BaseList(::std::boxed::Box<BaseList<'tree>>),
    Modifier(::std::boxed::Box<Modifier<'tree>>),
    PreprocIfInAttributeList(::std::boxed::Box<PreprocIfInAttributeList<'tree>>),
}
impl<'tree> ::treesitter_types::FromNode<'tree> for EnumDeclarationChildren<'tree> {
    #[allow(clippy::collapsible_else_if)]
    fn from_node(
        node: ::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        match node.kind() {
            "attribute_list" => Ok(Self::AttributeList(::std::boxed::Box::new(
                <AttributeList as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "base_list" => Ok(Self::BaseList(::std::boxed::Box::new(
                <BaseList as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "modifier" => Ok(Self::Modifier(::std::boxed::Box::new(
                <Modifier as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "preproc_if_in_attribute_list" => {
                Ok(Self::PreprocIfInAttributeList(::std::boxed::Box::new(
                    <PreprocIfInAttributeList as ::treesitter_types::FromNode>::from_node(
                        node, src,
                    )?,
                )))
            }
            other => Err(::treesitter_types::ParseError::unexpected_kind(other, node)),
        }
    }
}
impl ::treesitter_types::Spanned for EnumDeclarationChildren<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        match self {
            Self::AttributeList(inner) => inner.span(),
            Self::BaseList(inner) => inner.span(),
            Self::Modifier(inner) => inner.span(),
            Self::PreprocIfInAttributeList(inner) => inner.span(),
        }
    }
}
#[derive(Debug, Clone)]
pub enum EnumMemberDeclarationChildren<'tree> {
    AttributeList(::std::boxed::Box<AttributeList<'tree>>),
    PreprocIfInAttributeList(::std::boxed::Box<PreprocIfInAttributeList<'tree>>),
}
impl<'tree> ::treesitter_types::FromNode<'tree> for EnumMemberDeclarationChildren<'tree> {
    #[allow(clippy::collapsible_else_if)]
    fn from_node(
        node: ::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        match node.kind() {
            "attribute_list" => Ok(Self::AttributeList(::std::boxed::Box::new(
                <AttributeList as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "preproc_if_in_attribute_list" => {
                Ok(Self::PreprocIfInAttributeList(::std::boxed::Box::new(
                    <PreprocIfInAttributeList as ::treesitter_types::FromNode>::from_node(
                        node, src,
                    )?,
                )))
            }
            other => Err(::treesitter_types::ParseError::unexpected_kind(other, node)),
        }
    }
}
impl ::treesitter_types::Spanned for EnumMemberDeclarationChildren<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        match self {
            Self::AttributeList(inner) => inner.span(),
            Self::PreprocIfInAttributeList(inner) => inner.span(),
        }
    }
}
#[derive(Debug, Clone)]
pub enum EnumMemberDeclarationListChildren<'tree> {
    EnumMemberDeclaration(::std::boxed::Box<EnumMemberDeclaration<'tree>>),
    PreprocIf(::std::boxed::Box<PreprocIf<'tree>>),
}
impl<'tree> ::treesitter_types::FromNode<'tree> for EnumMemberDeclarationListChildren<'tree> {
    #[allow(clippy::collapsible_else_if)]
    fn from_node(
        node: ::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        match node.kind() {
            "enum_member_declaration" => Ok(Self::EnumMemberDeclaration(::std::boxed::Box::new(
                <EnumMemberDeclaration as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "preproc_if" => Ok(Self::PreprocIf(::std::boxed::Box::new(
                <PreprocIf as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            other => Err(::treesitter_types::ParseError::unexpected_kind(other, node)),
        }
    }
}
impl ::treesitter_types::Spanned for EnumMemberDeclarationListChildren<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        match self {
            Self::EnumMemberDeclaration(inner) => inner.span(),
            Self::PreprocIf(inner) => inner.span(),
        }
    }
}
#[derive(Debug, Clone)]
pub enum EventDeclarationChildren<'tree> {
    AttributeList(::std::boxed::Box<AttributeList<'tree>>),
    ExplicitInterfaceSpecifier(::std::boxed::Box<ExplicitInterfaceSpecifier<'tree>>),
    Modifier(::std::boxed::Box<Modifier<'tree>>),
    PreprocIfInAttributeList(::std::boxed::Box<PreprocIfInAttributeList<'tree>>),
}
impl<'tree> ::treesitter_types::FromNode<'tree> for EventDeclarationChildren<'tree> {
    #[allow(clippy::collapsible_else_if)]
    fn from_node(
        node: ::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        match node.kind() {
            "attribute_list" => Ok(Self::AttributeList(::std::boxed::Box::new(
                <AttributeList as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "explicit_interface_specifier" => {
                Ok(Self::ExplicitInterfaceSpecifier(::std::boxed::Box::new(
                    <ExplicitInterfaceSpecifier as ::treesitter_types::FromNode>::from_node(
                        node, src,
                    )?,
                )))
            }
            "modifier" => Ok(Self::Modifier(::std::boxed::Box::new(
                <Modifier as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "preproc_if_in_attribute_list" => {
                Ok(Self::PreprocIfInAttributeList(::std::boxed::Box::new(
                    <PreprocIfInAttributeList as ::treesitter_types::FromNode>::from_node(
                        node, src,
                    )?,
                )))
            }
            other => Err(::treesitter_types::ParseError::unexpected_kind(other, node)),
        }
    }
}
impl ::treesitter_types::Spanned for EventDeclarationChildren<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        match self {
            Self::AttributeList(inner) => inner.span(),
            Self::ExplicitInterfaceSpecifier(inner) => inner.span(),
            Self::Modifier(inner) => inner.span(),
            Self::PreprocIfInAttributeList(inner) => inner.span(),
        }
    }
}
#[derive(Debug, Clone)]
pub enum EventFieldDeclarationChildren<'tree> {
    AttributeList(::std::boxed::Box<AttributeList<'tree>>),
    Modifier(::std::boxed::Box<Modifier<'tree>>),
    PreprocIfInAttributeList(::std::boxed::Box<PreprocIfInAttributeList<'tree>>),
    VariableDeclaration(::std::boxed::Box<VariableDeclaration<'tree>>),
}
impl<'tree> ::treesitter_types::FromNode<'tree> for EventFieldDeclarationChildren<'tree> {
    #[allow(clippy::collapsible_else_if)]
    fn from_node(
        node: ::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        match node.kind() {
            "attribute_list" => Ok(Self::AttributeList(::std::boxed::Box::new(
                <AttributeList as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "modifier" => Ok(Self::Modifier(::std::boxed::Box::new(
                <Modifier as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "preproc_if_in_attribute_list" => {
                Ok(Self::PreprocIfInAttributeList(::std::boxed::Box::new(
                    <PreprocIfInAttributeList as ::treesitter_types::FromNode>::from_node(
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
impl ::treesitter_types::Spanned for EventFieldDeclarationChildren<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        match self {
            Self::AttributeList(inner) => inner.span(),
            Self::Modifier(inner) => inner.span(),
            Self::PreprocIfInAttributeList(inner) => inner.span(),
            Self::VariableDeclaration(inner) => inner.span(),
        }
    }
}
#[derive(Debug, Clone)]
pub enum ExplicitInterfaceSpecifierChildren<'tree> {
    AliasQualifiedName(::std::boxed::Box<AliasQualifiedName<'tree>>),
    GenericName(::std::boxed::Box<GenericName<'tree>>),
    Identifier(::std::boxed::Box<Identifier<'tree>>),
    QualifiedName(::std::boxed::Box<QualifiedName<'tree>>),
}
impl<'tree> ::treesitter_types::FromNode<'tree> for ExplicitInterfaceSpecifierChildren<'tree> {
    #[allow(clippy::collapsible_else_if)]
    fn from_node(
        node: ::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        match node.kind() {
            "alias_qualified_name" => Ok(Self::AliasQualifiedName(::std::boxed::Box::new(
                <AliasQualifiedName as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "generic_name" => Ok(Self::GenericName(::std::boxed::Box::new(
                <GenericName as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "identifier" => Ok(Self::Identifier(::std::boxed::Box::new(
                <Identifier as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "qualified_name" => Ok(Self::QualifiedName(::std::boxed::Box::new(
                <QualifiedName as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            other => Err(::treesitter_types::ParseError::unexpected_kind(other, node)),
        }
    }
}
impl ::treesitter_types::Spanned for ExplicitInterfaceSpecifierChildren<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        match self {
            Self::AliasQualifiedName(inner) => inner.span(),
            Self::GenericName(inner) => inner.span(),
            Self::Identifier(inner) => inner.span(),
            Self::QualifiedName(inner) => inner.span(),
        }
    }
}
#[derive(Debug, Clone)]
pub enum ExpressionStatementChildren<'tree> {
    AssignmentExpression(::std::boxed::Box<AssignmentExpression<'tree>>),
    AwaitExpression(::std::boxed::Box<AwaitExpression<'tree>>),
    InvocationExpression(::std::boxed::Box<InvocationExpression<'tree>>),
    ObjectCreationExpression(::std::boxed::Box<ObjectCreationExpression<'tree>>),
    ParenthesizedExpression(::std::boxed::Box<ParenthesizedExpression<'tree>>),
    PostfixUnaryExpression(::std::boxed::Box<PostfixUnaryExpression<'tree>>),
    PrefixUnaryExpression(::std::boxed::Box<PrefixUnaryExpression<'tree>>),
}
impl<'tree> ::treesitter_types::FromNode<'tree> for ExpressionStatementChildren<'tree> {
    #[allow(clippy::collapsible_else_if)]
    fn from_node(
        node: ::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        match node.kind() {
            "assignment_expression" => Ok(Self::AssignmentExpression(::std::boxed::Box::new(
                <AssignmentExpression as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "await_expression" => Ok(Self::AwaitExpression(::std::boxed::Box::new(
                <AwaitExpression as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "invocation_expression" => Ok(Self::InvocationExpression(::std::boxed::Box::new(
                <InvocationExpression as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "object_creation_expression" => {
                Ok(Self::ObjectCreationExpression(::std::boxed::Box::new(
                    <ObjectCreationExpression as ::treesitter_types::FromNode>::from_node(
                        node, src,
                    )?,
                )))
            }
            "parenthesized_expression" => {
                Ok(Self::ParenthesizedExpression(::std::boxed::Box::new(
                    <ParenthesizedExpression as ::treesitter_types::FromNode>::from_node(
                        node, src,
                    )?,
                )))
            }
            "postfix_unary_expression" => Ok(Self::PostfixUnaryExpression(::std::boxed::Box::new(
                <PostfixUnaryExpression as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "prefix_unary_expression" => Ok(Self::PrefixUnaryExpression(::std::boxed::Box::new(
                <PrefixUnaryExpression as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            other => Err(::treesitter_types::ParseError::unexpected_kind(other, node)),
        }
    }
}
impl ::treesitter_types::Spanned for ExpressionStatementChildren<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        match self {
            Self::AssignmentExpression(inner) => inner.span(),
            Self::AwaitExpression(inner) => inner.span(),
            Self::InvocationExpression(inner) => inner.span(),
            Self::ObjectCreationExpression(inner) => inner.span(),
            Self::ParenthesizedExpression(inner) => inner.span(),
            Self::PostfixUnaryExpression(inner) => inner.span(),
            Self::PrefixUnaryExpression(inner) => inner.span(),
        }
    }
}
#[derive(Debug, Clone)]
pub enum FieldDeclarationChildren<'tree> {
    AttributeList(::std::boxed::Box<AttributeList<'tree>>),
    Modifier(::std::boxed::Box<Modifier<'tree>>),
    PreprocIfInAttributeList(::std::boxed::Box<PreprocIfInAttributeList<'tree>>),
    VariableDeclaration(::std::boxed::Box<VariableDeclaration<'tree>>),
}
impl<'tree> ::treesitter_types::FromNode<'tree> for FieldDeclarationChildren<'tree> {
    #[allow(clippy::collapsible_else_if)]
    fn from_node(
        node: ::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        match node.kind() {
            "attribute_list" => Ok(Self::AttributeList(::std::boxed::Box::new(
                <AttributeList as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "modifier" => Ok(Self::Modifier(::std::boxed::Box::new(
                <Modifier as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "preproc_if_in_attribute_list" => {
                Ok(Self::PreprocIfInAttributeList(::std::boxed::Box::new(
                    <PreprocIfInAttributeList as ::treesitter_types::FromNode>::from_node(
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
impl ::treesitter_types::Spanned for FieldDeclarationChildren<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        match self {
            Self::AttributeList(inner) => inner.span(),
            Self::Modifier(inner) => inner.span(),
            Self::PreprocIfInAttributeList(inner) => inner.span(),
            Self::VariableDeclaration(inner) => inner.span(),
        }
    }
}
#[derive(Debug, Clone)]
pub enum FileScopedNamespaceDeclarationName<'tree> {
    AliasQualifiedName(::std::boxed::Box<AliasQualifiedName<'tree>>),
    GenericName(::std::boxed::Box<GenericName<'tree>>),
    Identifier(::std::boxed::Box<Identifier<'tree>>),
    QualifiedName(::std::boxed::Box<QualifiedName<'tree>>),
}
impl<'tree> ::treesitter_types::FromNode<'tree> for FileScopedNamespaceDeclarationName<'tree> {
    #[allow(clippy::collapsible_else_if)]
    fn from_node(
        node: ::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        match node.kind() {
            "alias_qualified_name" => Ok(Self::AliasQualifiedName(::std::boxed::Box::new(
                <AliasQualifiedName as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "generic_name" => Ok(Self::GenericName(::std::boxed::Box::new(
                <GenericName as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "identifier" => Ok(Self::Identifier(::std::boxed::Box::new(
                <Identifier as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "qualified_name" => Ok(Self::QualifiedName(::std::boxed::Box::new(
                <QualifiedName as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            other => Err(::treesitter_types::ParseError::unexpected_kind(other, node)),
        }
    }
}
impl ::treesitter_types::Spanned for FileScopedNamespaceDeclarationName<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        match self {
            Self::AliasQualifiedName(inner) => inner.span(),
            Self::GenericName(inner) => inner.span(),
            Self::Identifier(inner) => inner.span(),
            Self::QualifiedName(inner) => inner.span(),
        }
    }
}
#[derive(Debug, Clone)]
pub enum FixedStatementChildren<'tree> {
    Statement(::std::boxed::Box<Statement<'tree>>),
    VariableDeclaration(::std::boxed::Box<VariableDeclaration<'tree>>),
}
impl<'tree> ::treesitter_types::FromNode<'tree> for FixedStatementChildren<'tree> {
    #[allow(clippy::collapsible_else_if)]
    fn from_node(
        node: ::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        match node.kind() {
            "variable_declaration" => Ok(Self::VariableDeclaration(::std::boxed::Box::new(
                <VariableDeclaration as ::treesitter_types::FromNode>::from_node(node, src)?,
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
impl ::treesitter_types::Spanned for FixedStatementChildren<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        match self {
            Self::Statement(inner) => inner.span(),
            Self::VariableDeclaration(inner) => inner.span(),
        }
    }
}
#[derive(Debug, Clone)]
pub enum ForStatementInitializer<'tree> {
    Comma(::treesitter_types::Span),
    Expression(::std::boxed::Box<Expression<'tree>>),
    VariableDeclaration(::std::boxed::Box<VariableDeclaration<'tree>>),
}
impl<'tree> ::treesitter_types::FromNode<'tree> for ForStatementInitializer<'tree> {
    #[allow(clippy::collapsible_else_if)]
    fn from_node(
        node: ::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        match node.kind() {
            "," => Ok(Self::Comma(::treesitter_types::Span::from(node))),
            "variable_declaration" => Ok(Self::VariableDeclaration(::std::boxed::Box::new(
                <VariableDeclaration as ::treesitter_types::FromNode>::from_node(node, src)?,
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
            Self::Comma(span) => *span,
            Self::Expression(inner) => inner.span(),
            Self::VariableDeclaration(inner) => inner.span(),
        }
    }
}
#[derive(Debug, Clone)]
pub enum ForStatementUpdate<'tree> {
    Comma(::treesitter_types::Span),
    Expression(::std::boxed::Box<Expression<'tree>>),
}
impl<'tree> ::treesitter_types::FromNode<'tree> for ForStatementUpdate<'tree> {
    #[allow(clippy::collapsible_else_if)]
    fn from_node(
        node: ::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        match node.kind() {
            "," => Ok(Self::Comma(::treesitter_types::Span::from(node))),
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
            Self::Comma(span) => *span,
            Self::Expression(inner) => inner.span(),
        }
    }
}
#[derive(Debug, Clone)]
pub enum ForeachStatementLeft<'tree> {
    Expression(::std::boxed::Box<Expression<'tree>>),
    Identifier(::std::boxed::Box<Identifier<'tree>>),
    TuplePattern(::std::boxed::Box<TuplePattern<'tree>>),
}
impl<'tree> ::treesitter_types::FromNode<'tree> for ForeachStatementLeft<'tree> {
    #[allow(clippy::collapsible_else_if)]
    fn from_node(
        node: ::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        match node.kind() {
            "identifier" => Ok(Self::Identifier(::std::boxed::Box::new(
                <Identifier as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "tuple_pattern" => Ok(Self::TuplePattern(::std::boxed::Box::new(
                <TuplePattern as ::treesitter_types::FromNode>::from_node(node, src)?,
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
impl ::treesitter_types::Spanned for ForeachStatementLeft<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        match self {
            Self::Expression(inner) => inner.span(),
            Self::Identifier(inner) => inner.span(),
            Self::TuplePattern(inner) => inner.span(),
        }
    }
}
#[derive(Debug, Clone)]
pub enum FunctionPointerParameterType<'tree> {
    AliasQualifiedName(::std::boxed::Box<AliasQualifiedName<'tree>>),
    ArrayType(::std::boxed::Box<ArrayType<'tree>>),
    FunctionPointerType(::std::boxed::Box<FunctionPointerType<'tree>>),
    GenericName(::std::boxed::Box<GenericName<'tree>>),
    Identifier(::std::boxed::Box<Identifier<'tree>>),
    ImplicitType(::std::boxed::Box<ImplicitType<'tree>>),
    NullableType(::std::boxed::Box<NullableType<'tree>>),
    PointerType(::std::boxed::Box<PointerType<'tree>>),
    PredefinedType(::std::boxed::Box<PredefinedType<'tree>>),
    QualifiedName(::std::boxed::Box<QualifiedName<'tree>>),
    TupleType(::std::boxed::Box<TupleType<'tree>>),
}
impl<'tree> ::treesitter_types::FromNode<'tree> for FunctionPointerParameterType<'tree> {
    #[allow(clippy::collapsible_else_if)]
    fn from_node(
        node: ::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        match node.kind() {
            "alias_qualified_name" => Ok(Self::AliasQualifiedName(::std::boxed::Box::new(
                <AliasQualifiedName as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "array_type" => Ok(Self::ArrayType(::std::boxed::Box::new(
                <ArrayType as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "function_pointer_type" => Ok(Self::FunctionPointerType(::std::boxed::Box::new(
                <FunctionPointerType as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "generic_name" => Ok(Self::GenericName(::std::boxed::Box::new(
                <GenericName as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "identifier" => Ok(Self::Identifier(::std::boxed::Box::new(
                <Identifier as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "implicit_type" => Ok(Self::ImplicitType(::std::boxed::Box::new(
                <ImplicitType as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "nullable_type" => Ok(Self::NullableType(::std::boxed::Box::new(
                <NullableType as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "pointer_type" => Ok(Self::PointerType(::std::boxed::Box::new(
                <PointerType as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "predefined_type" => Ok(Self::PredefinedType(::std::boxed::Box::new(
                <PredefinedType as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "qualified_name" => Ok(Self::QualifiedName(::std::boxed::Box::new(
                <QualifiedName as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "tuple_type" => Ok(Self::TupleType(::std::boxed::Box::new(
                <TupleType as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            other => Err(::treesitter_types::ParseError::unexpected_kind(other, node)),
        }
    }
}
impl ::treesitter_types::Spanned for FunctionPointerParameterType<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        match self {
            Self::AliasQualifiedName(inner) => inner.span(),
            Self::ArrayType(inner) => inner.span(),
            Self::FunctionPointerType(inner) => inner.span(),
            Self::GenericName(inner) => inner.span(),
            Self::Identifier(inner) => inner.span(),
            Self::ImplicitType(inner) => inner.span(),
            Self::NullableType(inner) => inner.span(),
            Self::PointerType(inner) => inner.span(),
            Self::PredefinedType(inner) => inner.span(),
            Self::QualifiedName(inner) => inner.span(),
            Self::TupleType(inner) => inner.span(),
        }
    }
}
#[derive(Debug, Clone)]
pub enum FunctionPointerTypeChildren<'tree> {
    CallingConvention(::std::boxed::Box<CallingConvention<'tree>>),
    FunctionPointerParameter(::std::boxed::Box<FunctionPointerParameter<'tree>>),
}
impl<'tree> ::treesitter_types::FromNode<'tree> for FunctionPointerTypeChildren<'tree> {
    #[allow(clippy::collapsible_else_if)]
    fn from_node(
        node: ::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        match node.kind() {
            "calling_convention" => Ok(Self::CallingConvention(::std::boxed::Box::new(
                <CallingConvention as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "function_pointer_parameter" => {
                Ok(Self::FunctionPointerParameter(::std::boxed::Box::new(
                    <FunctionPointerParameter as ::treesitter_types::FromNode>::from_node(
                        node, src,
                    )?,
                )))
            }
            other => Err(::treesitter_types::ParseError::unexpected_kind(other, node)),
        }
    }
}
impl ::treesitter_types::Spanned for FunctionPointerTypeChildren<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        match self {
            Self::CallingConvention(inner) => inner.span(),
            Self::FunctionPointerParameter(inner) => inner.span(),
        }
    }
}
#[derive(Debug, Clone)]
pub enum GenericNameChildren<'tree> {
    Identifier(::std::boxed::Box<Identifier<'tree>>),
    TypeArgumentList(::std::boxed::Box<TypeArgumentList<'tree>>),
}
impl<'tree> ::treesitter_types::FromNode<'tree> for GenericNameChildren<'tree> {
    #[allow(clippy::collapsible_else_if)]
    fn from_node(
        node: ::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        match node.kind() {
            "identifier" => Ok(Self::Identifier(::std::boxed::Box::new(
                <Identifier as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "type_argument_list" => Ok(Self::TypeArgumentList(::std::boxed::Box::new(
                <TypeArgumentList as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            other => Err(::treesitter_types::ParseError::unexpected_kind(other, node)),
        }
    }
}
impl ::treesitter_types::Spanned for GenericNameChildren<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        match self {
            Self::Identifier(inner) => inner.span(),
            Self::TypeArgumentList(inner) => inner.span(),
        }
    }
}
#[derive(Debug, Clone)]
pub enum ImplicitObjectCreationExpressionChildren<'tree> {
    ArgumentList(::std::boxed::Box<ArgumentList<'tree>>),
    InitializerExpression(::std::boxed::Box<InitializerExpression<'tree>>),
}
impl<'tree> ::treesitter_types::FromNode<'tree>
    for ImplicitObjectCreationExpressionChildren<'tree>
{
    #[allow(clippy::collapsible_else_if)]
    fn from_node(
        node: ::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        match node.kind() {
            "argument_list" => Ok(Self::ArgumentList(::std::boxed::Box::new(
                <ArgumentList as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "initializer_expression" => Ok(Self::InitializerExpression(::std::boxed::Box::new(
                <InitializerExpression as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            other => Err(::treesitter_types::ParseError::unexpected_kind(other, node)),
        }
    }
}
impl ::treesitter_types::Spanned for ImplicitObjectCreationExpressionChildren<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        match self {
            Self::ArgumentList(inner) => inner.span(),
            Self::InitializerExpression(inner) => inner.span(),
        }
    }
}
#[derive(Debug, Clone)]
pub enum IndexerDeclarationChildren<'tree> {
    AttributeList(::std::boxed::Box<AttributeList<'tree>>),
    ExplicitInterfaceSpecifier(::std::boxed::Box<ExplicitInterfaceSpecifier<'tree>>),
    Modifier(::std::boxed::Box<Modifier<'tree>>),
    PreprocIfInAttributeList(::std::boxed::Box<PreprocIfInAttributeList<'tree>>),
}
impl<'tree> ::treesitter_types::FromNode<'tree> for IndexerDeclarationChildren<'tree> {
    #[allow(clippy::collapsible_else_if)]
    fn from_node(
        node: ::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        match node.kind() {
            "attribute_list" => Ok(Self::AttributeList(::std::boxed::Box::new(
                <AttributeList as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "explicit_interface_specifier" => {
                Ok(Self::ExplicitInterfaceSpecifier(::std::boxed::Box::new(
                    <ExplicitInterfaceSpecifier as ::treesitter_types::FromNode>::from_node(
                        node, src,
                    )?,
                )))
            }
            "modifier" => Ok(Self::Modifier(::std::boxed::Box::new(
                <Modifier as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "preproc_if_in_attribute_list" => {
                Ok(Self::PreprocIfInAttributeList(::std::boxed::Box::new(
                    <PreprocIfInAttributeList as ::treesitter_types::FromNode>::from_node(
                        node, src,
                    )?,
                )))
            }
            other => Err(::treesitter_types::ParseError::unexpected_kind(other, node)),
        }
    }
}
impl ::treesitter_types::Spanned for IndexerDeclarationChildren<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        match self {
            Self::AttributeList(inner) => inner.span(),
            Self::ExplicitInterfaceSpecifier(inner) => inner.span(),
            Self::Modifier(inner) => inner.span(),
            Self::PreprocIfInAttributeList(inner) => inner.span(),
        }
    }
}
#[derive(Debug, Clone)]
pub enum InterfaceDeclarationChildren<'tree> {
    AttributeList(::std::boxed::Box<AttributeList<'tree>>),
    BaseList(::std::boxed::Box<BaseList<'tree>>),
    Modifier(::std::boxed::Box<Modifier<'tree>>),
    PreprocIfInAttributeList(::std::boxed::Box<PreprocIfInAttributeList<'tree>>),
    TypeParameterConstraintsClause(::std::boxed::Box<TypeParameterConstraintsClause<'tree>>),
}
impl<'tree> ::treesitter_types::FromNode<'tree> for InterfaceDeclarationChildren<'tree> {
    #[allow(clippy::collapsible_else_if)]
    fn from_node(
        node: ::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        match node.kind() {
            "attribute_list" => Ok(Self::AttributeList(::std::boxed::Box::new(
                <AttributeList as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "base_list" => Ok(Self::BaseList(::std::boxed::Box::new(
                <BaseList as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "modifier" => Ok(Self::Modifier(::std::boxed::Box::new(
                <Modifier as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "preproc_if_in_attribute_list" => {
                Ok(Self::PreprocIfInAttributeList(::std::boxed::Box::new(
                    <PreprocIfInAttributeList as ::treesitter_types::FromNode>::from_node(
                        node, src,
                    )?,
                )))
            }
            "type_parameter_constraints_clause" => Ok(Self::TypeParameterConstraintsClause(
                ::std::boxed::Box::new(
                    <TypeParameterConstraintsClause as ::treesitter_types::FromNode>::from_node(
                        node, src,
                    )?,
                ),
            )),
            other => Err(::treesitter_types::ParseError::unexpected_kind(other, node)),
        }
    }
}
impl ::treesitter_types::Spanned for InterfaceDeclarationChildren<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        match self {
            Self::AttributeList(inner) => inner.span(),
            Self::BaseList(inner) => inner.span(),
            Self::Modifier(inner) => inner.span(),
            Self::PreprocIfInAttributeList(inner) => inner.span(),
            Self::TypeParameterConstraintsClause(inner) => inner.span(),
        }
    }
}
#[derive(Debug, Clone)]
pub enum InterpolatedStringExpressionChildren<'tree> {
    EscapeSequence(::std::boxed::Box<EscapeSequence<'tree>>),
    Interpolation(::std::boxed::Box<Interpolation<'tree>>),
    InterpolationQuote(::std::boxed::Box<InterpolationQuote<'tree>>),
    InterpolationStart(::std::boxed::Box<InterpolationStart<'tree>>),
    StringContent(::std::boxed::Box<StringContent<'tree>>),
}
impl<'tree> ::treesitter_types::FromNode<'tree> for InterpolatedStringExpressionChildren<'tree> {
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
            "interpolation_quote" => Ok(Self::InterpolationQuote(::std::boxed::Box::new(
                <InterpolationQuote as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "interpolation_start" => Ok(Self::InterpolationStart(::std::boxed::Box::new(
                <InterpolationStart as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "string_content" => Ok(Self::StringContent(::std::boxed::Box::new(
                <StringContent as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            other => Err(::treesitter_types::ParseError::unexpected_kind(other, node)),
        }
    }
}
impl ::treesitter_types::Spanned for InterpolatedStringExpressionChildren<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        match self {
            Self::EscapeSequence(inner) => inner.span(),
            Self::Interpolation(inner) => inner.span(),
            Self::InterpolationQuote(inner) => inner.span(),
            Self::InterpolationStart(inner) => inner.span(),
            Self::StringContent(inner) => inner.span(),
        }
    }
}
#[derive(Debug, Clone)]
pub enum InterpolationChildren<'tree> {
    Expression(::std::boxed::Box<Expression<'tree>>),
    InterpolationAlignmentClause(::std::boxed::Box<InterpolationAlignmentClause<'tree>>),
    InterpolationBrace(::std::boxed::Box<InterpolationBrace<'tree>>),
    InterpolationFormatClause(::std::boxed::Box<InterpolationFormatClause<'tree>>),
}
impl<'tree> ::treesitter_types::FromNode<'tree> for InterpolationChildren<'tree> {
    #[allow(clippy::collapsible_else_if)]
    fn from_node(
        node: ::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        match node.kind() {
            "interpolation_alignment_clause" => {
                Ok(Self::InterpolationAlignmentClause(::std::boxed::Box::new(
                    <InterpolationAlignmentClause as ::treesitter_types::FromNode>::from_node(
                        node, src,
                    )?,
                )))
            }
            "interpolation_brace" => Ok(Self::InterpolationBrace(::std::boxed::Box::new(
                <InterpolationBrace as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "interpolation_format_clause" => {
                Ok(Self::InterpolationFormatClause(::std::boxed::Box::new(
                    <InterpolationFormatClause as ::treesitter_types::FromNode>::from_node(
                        node, src,
                    )?,
                )))
            }
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
impl ::treesitter_types::Spanned for InterpolationChildren<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        match self {
            Self::Expression(inner) => inner.span(),
            Self::InterpolationAlignmentClause(inner) => inner.span(),
            Self::InterpolationBrace(inner) => inner.span(),
            Self::InterpolationFormatClause(inner) => inner.span(),
        }
    }
}
#[derive(Debug, Clone)]
pub enum InvocationExpressionFunction<'tree> {
    AliasQualifiedName(::std::boxed::Box<AliasQualifiedName<'tree>>),
    CastExpression(::std::boxed::Box<CastExpression<'tree>>),
    ConditionalAccessExpression(::std::boxed::Box<ConditionalAccessExpression<'tree>>),
    ElementAccessExpression(::std::boxed::Box<ElementAccessExpression<'tree>>),
    Expression(::std::boxed::Box<Expression<'tree>>),
    GenericName(::std::boxed::Box<GenericName<'tree>>),
    Identifier(::std::boxed::Box<Identifier<'tree>>),
    InvocationExpression(::std::boxed::Box<InvocationExpression<'tree>>),
    MemberAccessExpression(::std::boxed::Box<MemberAccessExpression<'tree>>),
    ParenthesizedExpression(::std::boxed::Box<ParenthesizedExpression<'tree>>),
    QualifiedName(::std::boxed::Box<QualifiedName<'tree>>),
}
impl<'tree> ::treesitter_types::FromNode<'tree> for InvocationExpressionFunction<'tree> {
    #[allow(clippy::collapsible_else_if)]
    fn from_node(
        node: ::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        match node.kind() {
            "alias_qualified_name" => Ok(Self::AliasQualifiedName(::std::boxed::Box::new(
                <AliasQualifiedName as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "cast_expression" => Ok(Self::CastExpression(::std::boxed::Box::new(
                <CastExpression as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "conditional_access_expression" => {
                Ok(Self::ConditionalAccessExpression(::std::boxed::Box::new(
                    <ConditionalAccessExpression as ::treesitter_types::FromNode>::from_node(
                        node, src,
                    )?,
                )))
            }
            "element_access_expression" => {
                Ok(Self::ElementAccessExpression(::std::boxed::Box::new(
                    <ElementAccessExpression as ::treesitter_types::FromNode>::from_node(
                        node, src,
                    )?,
                )))
            }
            "generic_name" => Ok(Self::GenericName(::std::boxed::Box::new(
                <GenericName as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "identifier" => Ok(Self::Identifier(::std::boxed::Box::new(
                <Identifier as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "invocation_expression" => Ok(Self::InvocationExpression(::std::boxed::Box::new(
                <InvocationExpression as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "member_access_expression" => Ok(Self::MemberAccessExpression(::std::boxed::Box::new(
                <MemberAccessExpression as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "parenthesized_expression" => {
                Ok(Self::ParenthesizedExpression(::std::boxed::Box::new(
                    <ParenthesizedExpression as ::treesitter_types::FromNode>::from_node(
                        node, src,
                    )?,
                )))
            }
            "qualified_name" => Ok(Self::QualifiedName(::std::boxed::Box::new(
                <QualifiedName as ::treesitter_types::FromNode>::from_node(node, src)?,
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
impl ::treesitter_types::Spanned for InvocationExpressionFunction<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        match self {
            Self::AliasQualifiedName(inner) => inner.span(),
            Self::CastExpression(inner) => inner.span(),
            Self::ConditionalAccessExpression(inner) => inner.span(),
            Self::ElementAccessExpression(inner) => inner.span(),
            Self::Expression(inner) => inner.span(),
            Self::GenericName(inner) => inner.span(),
            Self::Identifier(inner) => inner.span(),
            Self::InvocationExpression(inner) => inner.span(),
            Self::MemberAccessExpression(inner) => inner.span(),
            Self::ParenthesizedExpression(inner) => inner.span(),
            Self::QualifiedName(inner) => inner.span(),
        }
    }
}
#[derive(Debug, Clone)]
pub enum IsExpressionOperator {
    Is(::treesitter_types::Span),
}
impl<'tree> ::treesitter_types::FromNode<'tree> for IsExpressionOperator {
    #[allow(clippy::collapsible_else_if)]
    fn from_node(
        node: ::tree_sitter::Node<'tree>,
        _src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        match node.kind() {
            "is" => Ok(Self::Is(::treesitter_types::Span::from(node))),
            other => Err(::treesitter_types::ParseError::unexpected_kind(other, node)),
        }
    }
}
impl ::treesitter_types::Spanned for IsExpressionOperator {
    fn span(&self) -> ::treesitter_types::Span {
        match self {
            Self::Is(span) => *span,
        }
    }
}
#[derive(Debug, Clone)]
pub enum JoinClauseChildren<'tree> {
    Expression(::std::boxed::Box<Expression<'tree>>),
    Identifier(::std::boxed::Box<Identifier<'tree>>),
    JoinIntoClause(::std::boxed::Box<JoinIntoClause<'tree>>),
}
impl<'tree> ::treesitter_types::FromNode<'tree> for JoinClauseChildren<'tree> {
    #[allow(clippy::collapsible_else_if)]
    fn from_node(
        node: ::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        match node.kind() {
            "identifier" => Ok(Self::Identifier(::std::boxed::Box::new(
                <Identifier as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "join_into_clause" => Ok(Self::JoinIntoClause(::std::boxed::Box::new(
                <JoinIntoClause as ::treesitter_types::FromNode>::from_node(node, src)?,
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
impl ::treesitter_types::Spanned for JoinClauseChildren<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        match self {
            Self::Expression(inner) => inner.span(),
            Self::Identifier(inner) => inner.span(),
            Self::JoinIntoClause(inner) => inner.span(),
        }
    }
}
#[derive(Debug, Clone)]
pub enum LabeledStatementChildren<'tree> {
    Identifier(::std::boxed::Box<Identifier<'tree>>),
    Statement(::std::boxed::Box<Statement<'tree>>),
}
impl<'tree> ::treesitter_types::FromNode<'tree> for LabeledStatementChildren<'tree> {
    #[allow(clippy::collapsible_else_if)]
    fn from_node(
        node: ::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        match node.kind() {
            "identifier" => Ok(Self::Identifier(::std::boxed::Box::new(
                <Identifier as ::treesitter_types::FromNode>::from_node(node, src)?,
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
            Self::Identifier(inner) => inner.span(),
            Self::Statement(inner) => inner.span(),
        }
    }
}
#[derive(Debug, Clone)]
pub enum LambdaExpressionBody<'tree> {
    Block(::std::boxed::Box<Block<'tree>>),
    Expression(::std::boxed::Box<Expression<'tree>>),
}
impl<'tree> ::treesitter_types::FromNode<'tree> for LambdaExpressionBody<'tree> {
    #[allow(clippy::collapsible_else_if)]
    fn from_node(
        node: ::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        match node.kind() {
            "block" => Ok(Self::Block(::std::boxed::Box::new(
                <Block as ::treesitter_types::FromNode>::from_node(node, src)?,
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
impl ::treesitter_types::Spanned for LambdaExpressionBody<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        match self {
            Self::Block(inner) => inner.span(),
            Self::Expression(inner) => inner.span(),
        }
    }
}
#[derive(Debug, Clone)]
pub enum LambdaExpressionParameters<'tree> {
    ImplicitParameter(::std::boxed::Box<ImplicitParameter<'tree>>),
    ParameterList(::std::boxed::Box<ParameterList<'tree>>),
}
impl<'tree> ::treesitter_types::FromNode<'tree> for LambdaExpressionParameters<'tree> {
    #[allow(clippy::collapsible_else_if)]
    fn from_node(
        node: ::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        match node.kind() {
            "implicit_parameter" => Ok(Self::ImplicitParameter(::std::boxed::Box::new(
                <ImplicitParameter as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "parameter_list" => Ok(Self::ParameterList(::std::boxed::Box::new(
                <ParameterList as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            other => Err(::treesitter_types::ParseError::unexpected_kind(other, node)),
        }
    }
}
impl ::treesitter_types::Spanned for LambdaExpressionParameters<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        match self {
            Self::ImplicitParameter(inner) => inner.span(),
            Self::ParameterList(inner) => inner.span(),
        }
    }
}
#[derive(Debug, Clone)]
pub enum LambdaExpressionChildren<'tree> {
    AttributeList(::std::boxed::Box<AttributeList<'tree>>),
    Modifier(::std::boxed::Box<Modifier<'tree>>),
    PreprocIfInAttributeList(::std::boxed::Box<PreprocIfInAttributeList<'tree>>),
}
impl<'tree> ::treesitter_types::FromNode<'tree> for LambdaExpressionChildren<'tree> {
    #[allow(clippy::collapsible_else_if)]
    fn from_node(
        node: ::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        match node.kind() {
            "attribute_list" => Ok(Self::AttributeList(::std::boxed::Box::new(
                <AttributeList as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "modifier" => Ok(Self::Modifier(::std::boxed::Box::new(
                <Modifier as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "preproc_if_in_attribute_list" => {
                Ok(Self::PreprocIfInAttributeList(::std::boxed::Box::new(
                    <PreprocIfInAttributeList as ::treesitter_types::FromNode>::from_node(
                        node, src,
                    )?,
                )))
            }
            other => Err(::treesitter_types::ParseError::unexpected_kind(other, node)),
        }
    }
}
impl ::treesitter_types::Spanned for LambdaExpressionChildren<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        match self {
            Self::AttributeList(inner) => inner.span(),
            Self::Modifier(inner) => inner.span(),
            Self::PreprocIfInAttributeList(inner) => inner.span(),
        }
    }
}
#[derive(Debug, Clone)]
pub enum LetClauseChildren<'tree> {
    Expression(::std::boxed::Box<Expression<'tree>>),
    Identifier(::std::boxed::Box<Identifier<'tree>>),
}
impl<'tree> ::treesitter_types::FromNode<'tree> for LetClauseChildren<'tree> {
    #[allow(clippy::collapsible_else_if)]
    fn from_node(
        node: ::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        match node.kind() {
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
impl ::treesitter_types::Spanned for LetClauseChildren<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        match self {
            Self::Expression(inner) => inner.span(),
            Self::Identifier(inner) => inner.span(),
        }
    }
}
#[derive(Debug, Clone)]
pub enum ListPatternChildren<'tree> {
    ParenthesizedVariableDesignation(::std::boxed::Box<ParenthesizedVariableDesignation<'tree>>),
    Pattern(::std::boxed::Box<Pattern<'tree>>),
}
impl<'tree> ::treesitter_types::FromNode<'tree> for ListPatternChildren<'tree> {
    #[allow(clippy::collapsible_else_if)]
    fn from_node(
        node: ::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        match node.kind() {
            "parenthesized_variable_designation" => Ok(Self::ParenthesizedVariableDesignation(
                ::std::boxed::Box::new(
                    <ParenthesizedVariableDesignation as ::treesitter_types::FromNode>::from_node(
                        node, src,
                    )?,
                ),
            )),
            _other => {
                if let Ok(v) = <Pattern as ::treesitter_types::FromNode>::from_node(node, src) {
                    Ok(Self::Pattern(::std::boxed::Box::new(v)))
                } else {
                    Err(::treesitter_types::ParseError::unexpected_kind(
                        _other, node,
                    ))
                }
            }
        }
    }
}
impl ::treesitter_types::Spanned for ListPatternChildren<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        match self {
            Self::ParenthesizedVariableDesignation(inner) => inner.span(),
            Self::Pattern(inner) => inner.span(),
        }
    }
}
#[derive(Debug, Clone)]
pub enum LocalDeclarationStatementChildren<'tree> {
    Modifier(::std::boxed::Box<Modifier<'tree>>),
    VariableDeclaration(::std::boxed::Box<VariableDeclaration<'tree>>),
}
impl<'tree> ::treesitter_types::FromNode<'tree> for LocalDeclarationStatementChildren<'tree> {
    #[allow(clippy::collapsible_else_if)]
    fn from_node(
        node: ::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        match node.kind() {
            "modifier" => Ok(Self::Modifier(::std::boxed::Box::new(
                <Modifier as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "variable_declaration" => Ok(Self::VariableDeclaration(::std::boxed::Box::new(
                <VariableDeclaration as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            other => Err(::treesitter_types::ParseError::unexpected_kind(other, node)),
        }
    }
}
impl ::treesitter_types::Spanned for LocalDeclarationStatementChildren<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        match self {
            Self::Modifier(inner) => inner.span(),
            Self::VariableDeclaration(inner) => inner.span(),
        }
    }
}
#[derive(Debug, Clone)]
pub enum LocalFunctionStatementBody<'tree> {
    ArrowExpressionClause(::std::boxed::Box<ArrowExpressionClause<'tree>>),
    Block(::std::boxed::Box<Block<'tree>>),
}
impl<'tree> ::treesitter_types::FromNode<'tree> for LocalFunctionStatementBody<'tree> {
    #[allow(clippy::collapsible_else_if)]
    fn from_node(
        node: ::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        match node.kind() {
            "arrow_expression_clause" => Ok(Self::ArrowExpressionClause(::std::boxed::Box::new(
                <ArrowExpressionClause as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "block" => Ok(Self::Block(::std::boxed::Box::new(
                <Block as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            other => Err(::treesitter_types::ParseError::unexpected_kind(other, node)),
        }
    }
}
impl ::treesitter_types::Spanned for LocalFunctionStatementBody<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        match self {
            Self::ArrowExpressionClause(inner) => inner.span(),
            Self::Block(inner) => inner.span(),
        }
    }
}
#[derive(Debug, Clone)]
pub enum LocalFunctionStatementChildren<'tree> {
    AttributeList(::std::boxed::Box<AttributeList<'tree>>),
    Modifier(::std::boxed::Box<Modifier<'tree>>),
    PreprocIfInAttributeList(::std::boxed::Box<PreprocIfInAttributeList<'tree>>),
    TypeParameterConstraintsClause(::std::boxed::Box<TypeParameterConstraintsClause<'tree>>),
}
impl<'tree> ::treesitter_types::FromNode<'tree> for LocalFunctionStatementChildren<'tree> {
    #[allow(clippy::collapsible_else_if)]
    fn from_node(
        node: ::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        match node.kind() {
            "attribute_list" => Ok(Self::AttributeList(::std::boxed::Box::new(
                <AttributeList as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "modifier" => Ok(Self::Modifier(::std::boxed::Box::new(
                <Modifier as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "preproc_if_in_attribute_list" => {
                Ok(Self::PreprocIfInAttributeList(::std::boxed::Box::new(
                    <PreprocIfInAttributeList as ::treesitter_types::FromNode>::from_node(
                        node, src,
                    )?,
                )))
            }
            "type_parameter_constraints_clause" => Ok(Self::TypeParameterConstraintsClause(
                ::std::boxed::Box::new(
                    <TypeParameterConstraintsClause as ::treesitter_types::FromNode>::from_node(
                        node, src,
                    )?,
                ),
            )),
            other => Err(::treesitter_types::ParseError::unexpected_kind(other, node)),
        }
    }
}
impl ::treesitter_types::Spanned for LocalFunctionStatementChildren<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        match self {
            Self::AttributeList(inner) => inner.span(),
            Self::Modifier(inner) => inner.span(),
            Self::PreprocIfInAttributeList(inner) => inner.span(),
            Self::TypeParameterConstraintsClause(inner) => inner.span(),
        }
    }
}
#[derive(Debug, Clone)]
pub enum LockStatementChildren<'tree> {
    Expression(::std::boxed::Box<Expression<'tree>>),
    Statement(::std::boxed::Box<Statement<'tree>>),
}
impl<'tree> ::treesitter_types::FromNode<'tree> for LockStatementChildren<'tree> {
    #[allow(clippy::collapsible_else_if)]
    fn from_node(
        node: ::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        if let Ok(v) = <Expression as ::treesitter_types::FromNode>::from_node(node, src) {
            Ok(Self::Expression(::std::boxed::Box::new(v)))
        } else {
            if let Ok(v) = <Statement as ::treesitter_types::FromNode>::from_node(node, src) {
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
impl ::treesitter_types::Spanned for LockStatementChildren<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        match self {
            Self::Expression(inner) => inner.span(),
            Self::Statement(inner) => inner.span(),
        }
    }
}
#[derive(Debug, Clone)]
pub enum MemberAccessExpressionExpression<'tree> {
    AliasQualifiedName(::std::boxed::Box<AliasQualifiedName<'tree>>),
    Expression(::std::boxed::Box<Expression<'tree>>),
    GenericName(::std::boxed::Box<GenericName<'tree>>),
    Identifier(::std::boxed::Box<Identifier<'tree>>),
    PredefinedType(::std::boxed::Box<PredefinedType<'tree>>),
    QualifiedName(::std::boxed::Box<QualifiedName<'tree>>),
}
impl<'tree> ::treesitter_types::FromNode<'tree> for MemberAccessExpressionExpression<'tree> {
    #[allow(clippy::collapsible_else_if)]
    fn from_node(
        node: ::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        match node.kind() {
            "alias_qualified_name" => Ok(Self::AliasQualifiedName(::std::boxed::Box::new(
                <AliasQualifiedName as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "generic_name" => Ok(Self::GenericName(::std::boxed::Box::new(
                <GenericName as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "identifier" => Ok(Self::Identifier(::std::boxed::Box::new(
                <Identifier as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "predefined_type" => Ok(Self::PredefinedType(::std::boxed::Box::new(
                <PredefinedType as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "qualified_name" => Ok(Self::QualifiedName(::std::boxed::Box::new(
                <QualifiedName as ::treesitter_types::FromNode>::from_node(node, src)?,
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
impl ::treesitter_types::Spanned for MemberAccessExpressionExpression<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        match self {
            Self::AliasQualifiedName(inner) => inner.span(),
            Self::Expression(inner) => inner.span(),
            Self::GenericName(inner) => inner.span(),
            Self::Identifier(inner) => inner.span(),
            Self::PredefinedType(inner) => inner.span(),
            Self::QualifiedName(inner) => inner.span(),
        }
    }
}
#[derive(Debug, Clone)]
pub enum MemberAccessExpressionName<'tree> {
    GenericName(::std::boxed::Box<GenericName<'tree>>),
    Identifier(::std::boxed::Box<Identifier<'tree>>),
}
impl<'tree> ::treesitter_types::FromNode<'tree> for MemberAccessExpressionName<'tree> {
    #[allow(clippy::collapsible_else_if)]
    fn from_node(
        node: ::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        match node.kind() {
            "generic_name" => Ok(Self::GenericName(::std::boxed::Box::new(
                <GenericName as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "identifier" => Ok(Self::Identifier(::std::boxed::Box::new(
                <Identifier as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            other => Err(::treesitter_types::ParseError::unexpected_kind(other, node)),
        }
    }
}
impl ::treesitter_types::Spanned for MemberAccessExpressionName<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        match self {
            Self::GenericName(inner) => inner.span(),
            Self::Identifier(inner) => inner.span(),
        }
    }
}
#[derive(Debug, Clone)]
pub enum MemberBindingExpressionName<'tree> {
    GenericName(::std::boxed::Box<GenericName<'tree>>),
    Identifier(::std::boxed::Box<Identifier<'tree>>),
}
impl<'tree> ::treesitter_types::FromNode<'tree> for MemberBindingExpressionName<'tree> {
    #[allow(clippy::collapsible_else_if)]
    fn from_node(
        node: ::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        match node.kind() {
            "generic_name" => Ok(Self::GenericName(::std::boxed::Box::new(
                <GenericName as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "identifier" => Ok(Self::Identifier(::std::boxed::Box::new(
                <Identifier as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            other => Err(::treesitter_types::ParseError::unexpected_kind(other, node)),
        }
    }
}
impl ::treesitter_types::Spanned for MemberBindingExpressionName<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        match self {
            Self::GenericName(inner) => inner.span(),
            Self::Identifier(inner) => inner.span(),
        }
    }
}
#[derive(Debug, Clone)]
pub enum MethodDeclarationBody<'tree> {
    ArrowExpressionClause(::std::boxed::Box<ArrowExpressionClause<'tree>>),
    Block(::std::boxed::Box<Block<'tree>>),
}
impl<'tree> ::treesitter_types::FromNode<'tree> for MethodDeclarationBody<'tree> {
    #[allow(clippy::collapsible_else_if)]
    fn from_node(
        node: ::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        match node.kind() {
            "arrow_expression_clause" => Ok(Self::ArrowExpressionClause(::std::boxed::Box::new(
                <ArrowExpressionClause as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "block" => Ok(Self::Block(::std::boxed::Box::new(
                <Block as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            other => Err(::treesitter_types::ParseError::unexpected_kind(other, node)),
        }
    }
}
impl ::treesitter_types::Spanned for MethodDeclarationBody<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        match self {
            Self::ArrowExpressionClause(inner) => inner.span(),
            Self::Block(inner) => inner.span(),
        }
    }
}
#[derive(Debug, Clone)]
pub enum MethodDeclarationChildren<'tree> {
    AttributeList(::std::boxed::Box<AttributeList<'tree>>),
    ExplicitInterfaceSpecifier(::std::boxed::Box<ExplicitInterfaceSpecifier<'tree>>),
    Modifier(::std::boxed::Box<Modifier<'tree>>),
    PreprocIfInAttributeList(::std::boxed::Box<PreprocIfInAttributeList<'tree>>),
    TypeParameterConstraintsClause(::std::boxed::Box<TypeParameterConstraintsClause<'tree>>),
}
impl<'tree> ::treesitter_types::FromNode<'tree> for MethodDeclarationChildren<'tree> {
    #[allow(clippy::collapsible_else_if)]
    fn from_node(
        node: ::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        match node.kind() {
            "attribute_list" => Ok(Self::AttributeList(::std::boxed::Box::new(
                <AttributeList as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "explicit_interface_specifier" => {
                Ok(Self::ExplicitInterfaceSpecifier(::std::boxed::Box::new(
                    <ExplicitInterfaceSpecifier as ::treesitter_types::FromNode>::from_node(
                        node, src,
                    )?,
                )))
            }
            "modifier" => Ok(Self::Modifier(::std::boxed::Box::new(
                <Modifier as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "preproc_if_in_attribute_list" => {
                Ok(Self::PreprocIfInAttributeList(::std::boxed::Box::new(
                    <PreprocIfInAttributeList as ::treesitter_types::FromNode>::from_node(
                        node, src,
                    )?,
                )))
            }
            "type_parameter_constraints_clause" => Ok(Self::TypeParameterConstraintsClause(
                ::std::boxed::Box::new(
                    <TypeParameterConstraintsClause as ::treesitter_types::FromNode>::from_node(
                        node, src,
                    )?,
                ),
            )),
            other => Err(::treesitter_types::ParseError::unexpected_kind(other, node)),
        }
    }
}
impl ::treesitter_types::Spanned for MethodDeclarationChildren<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        match self {
            Self::AttributeList(inner) => inner.span(),
            Self::ExplicitInterfaceSpecifier(inner) => inner.span(),
            Self::Modifier(inner) => inner.span(),
            Self::PreprocIfInAttributeList(inner) => inner.span(),
            Self::TypeParameterConstraintsClause(inner) => inner.span(),
        }
    }
}
#[derive(Debug, Clone)]
pub enum NamespaceDeclarationName<'tree> {
    AliasQualifiedName(::std::boxed::Box<AliasQualifiedName<'tree>>),
    GenericName(::std::boxed::Box<GenericName<'tree>>),
    Identifier(::std::boxed::Box<Identifier<'tree>>),
    QualifiedName(::std::boxed::Box<QualifiedName<'tree>>),
}
impl<'tree> ::treesitter_types::FromNode<'tree> for NamespaceDeclarationName<'tree> {
    #[allow(clippy::collapsible_else_if)]
    fn from_node(
        node: ::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        match node.kind() {
            "alias_qualified_name" => Ok(Self::AliasQualifiedName(::std::boxed::Box::new(
                <AliasQualifiedName as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "generic_name" => Ok(Self::GenericName(::std::boxed::Box::new(
                <GenericName as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "identifier" => Ok(Self::Identifier(::std::boxed::Box::new(
                <Identifier as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "qualified_name" => Ok(Self::QualifiedName(::std::boxed::Box::new(
                <QualifiedName as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            other => Err(::treesitter_types::ParseError::unexpected_kind(other, node)),
        }
    }
}
impl ::treesitter_types::Spanned for NamespaceDeclarationName<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        match self {
            Self::AliasQualifiedName(inner) => inner.span(),
            Self::GenericName(inner) => inner.span(),
            Self::Identifier(inner) => inner.span(),
            Self::QualifiedName(inner) => inner.span(),
        }
    }
}
#[derive(Debug, Clone)]
pub enum NullableTypeType<'tree> {
    AliasQualifiedName(::std::boxed::Box<AliasQualifiedName<'tree>>),
    ArrayType(::std::boxed::Box<ArrayType<'tree>>),
    GenericName(::std::boxed::Box<GenericName<'tree>>),
    Identifier(::std::boxed::Box<Identifier<'tree>>),
    PredefinedType(::std::boxed::Box<PredefinedType<'tree>>),
    QualifiedName(::std::boxed::Box<QualifiedName<'tree>>),
    TupleType(::std::boxed::Box<TupleType<'tree>>),
}
impl<'tree> ::treesitter_types::FromNode<'tree> for NullableTypeType<'tree> {
    #[allow(clippy::collapsible_else_if)]
    fn from_node(
        node: ::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        match node.kind() {
            "alias_qualified_name" => Ok(Self::AliasQualifiedName(::std::boxed::Box::new(
                <AliasQualifiedName as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "array_type" => Ok(Self::ArrayType(::std::boxed::Box::new(
                <ArrayType as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "generic_name" => Ok(Self::GenericName(::std::boxed::Box::new(
                <GenericName as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "identifier" => Ok(Self::Identifier(::std::boxed::Box::new(
                <Identifier as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "predefined_type" => Ok(Self::PredefinedType(::std::boxed::Box::new(
                <PredefinedType as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "qualified_name" => Ok(Self::QualifiedName(::std::boxed::Box::new(
                <QualifiedName as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "tuple_type" => Ok(Self::TupleType(::std::boxed::Box::new(
                <TupleType as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            other => Err(::treesitter_types::ParseError::unexpected_kind(other, node)),
        }
    }
}
impl ::treesitter_types::Spanned for NullableTypeType<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        match self {
            Self::AliasQualifiedName(inner) => inner.span(),
            Self::ArrayType(inner) => inner.span(),
            Self::GenericName(inner) => inner.span(),
            Self::Identifier(inner) => inner.span(),
            Self::PredefinedType(inner) => inner.span(),
            Self::QualifiedName(inner) => inner.span(),
            Self::TupleType(inner) => inner.span(),
        }
    }
}
#[derive(Debug, Clone)]
pub enum OperatorDeclarationBody<'tree> {
    ArrowExpressionClause(::std::boxed::Box<ArrowExpressionClause<'tree>>),
    Block(::std::boxed::Box<Block<'tree>>),
}
impl<'tree> ::treesitter_types::FromNode<'tree> for OperatorDeclarationBody<'tree> {
    #[allow(clippy::collapsible_else_if)]
    fn from_node(
        node: ::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        match node.kind() {
            "arrow_expression_clause" => Ok(Self::ArrowExpressionClause(::std::boxed::Box::new(
                <ArrowExpressionClause as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "block" => Ok(Self::Block(::std::boxed::Box::new(
                <Block as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            other => Err(::treesitter_types::ParseError::unexpected_kind(other, node)),
        }
    }
}
impl ::treesitter_types::Spanned for OperatorDeclarationBody<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        match self {
            Self::ArrowExpressionClause(inner) => inner.span(),
            Self::Block(inner) => inner.span(),
        }
    }
}
#[derive(Debug, Clone)]
pub enum OperatorDeclarationOperator {
    Bang(::treesitter_types::Span),
    NotEq(::treesitter_types::Span),
    Percent(::treesitter_types::Span),
    Amp(::treesitter_types::Span),
    Star(::treesitter_types::Span),
    Plus(::treesitter_types::Span),
    PlusPlus(::treesitter_types::Span),
    Minus(::treesitter_types::Span),
    MinusMinus(::treesitter_types::Span),
    Slash(::treesitter_types::Span),
    Lt(::treesitter_types::Span),
    Shl(::treesitter_types::Span),
    LtEq(::treesitter_types::Span),
    EqEq(::treesitter_types::Span),
    Gt(::treesitter_types::Span),
    GtEq(::treesitter_types::Span),
    Shr(::treesitter_types::Span),
    GtGtGt(::treesitter_types::Span),
    Caret(::treesitter_types::Span),
    False(::treesitter_types::Span),
    True(::treesitter_types::Span),
    Pipe(::treesitter_types::Span),
    Tilde(::treesitter_types::Span),
}
impl<'tree> ::treesitter_types::FromNode<'tree> for OperatorDeclarationOperator {
    #[allow(clippy::collapsible_else_if)]
    fn from_node(
        node: ::tree_sitter::Node<'tree>,
        _src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        match node.kind() {
            "!" => Ok(Self::Bang(::treesitter_types::Span::from(node))),
            "!=" => Ok(Self::NotEq(::treesitter_types::Span::from(node))),
            "%" => Ok(Self::Percent(::treesitter_types::Span::from(node))),
            "&" => Ok(Self::Amp(::treesitter_types::Span::from(node))),
            "*" => Ok(Self::Star(::treesitter_types::Span::from(node))),
            "+" => Ok(Self::Plus(::treesitter_types::Span::from(node))),
            "++" => Ok(Self::PlusPlus(::treesitter_types::Span::from(node))),
            "-" => Ok(Self::Minus(::treesitter_types::Span::from(node))),
            "--" => Ok(Self::MinusMinus(::treesitter_types::Span::from(node))),
            "/" => Ok(Self::Slash(::treesitter_types::Span::from(node))),
            "<" => Ok(Self::Lt(::treesitter_types::Span::from(node))),
            "<<" => Ok(Self::Shl(::treesitter_types::Span::from(node))),
            "<=" => Ok(Self::LtEq(::treesitter_types::Span::from(node))),
            "==" => Ok(Self::EqEq(::treesitter_types::Span::from(node))),
            ">" => Ok(Self::Gt(::treesitter_types::Span::from(node))),
            ">=" => Ok(Self::GtEq(::treesitter_types::Span::from(node))),
            ">>" => Ok(Self::Shr(::treesitter_types::Span::from(node))),
            ">>>" => Ok(Self::GtGtGt(::treesitter_types::Span::from(node))),
            "^" => Ok(Self::Caret(::treesitter_types::Span::from(node))),
            "false" => Ok(Self::False(::treesitter_types::Span::from(node))),
            "true" => Ok(Self::True(::treesitter_types::Span::from(node))),
            "|" => Ok(Self::Pipe(::treesitter_types::Span::from(node))),
            "~" => Ok(Self::Tilde(::treesitter_types::Span::from(node))),
            other => Err(::treesitter_types::ParseError::unexpected_kind(other, node)),
        }
    }
}
impl ::treesitter_types::Spanned for OperatorDeclarationOperator {
    fn span(&self) -> ::treesitter_types::Span {
        match self {
            Self::Bang(span) => *span,
            Self::NotEq(span) => *span,
            Self::Percent(span) => *span,
            Self::Amp(span) => *span,
            Self::Star(span) => *span,
            Self::Plus(span) => *span,
            Self::PlusPlus(span) => *span,
            Self::Minus(span) => *span,
            Self::MinusMinus(span) => *span,
            Self::Slash(span) => *span,
            Self::Lt(span) => *span,
            Self::Shl(span) => *span,
            Self::LtEq(span) => *span,
            Self::EqEq(span) => *span,
            Self::Gt(span) => *span,
            Self::GtEq(span) => *span,
            Self::Shr(span) => *span,
            Self::GtGtGt(span) => *span,
            Self::Caret(span) => *span,
            Self::False(span) => *span,
            Self::True(span) => *span,
            Self::Pipe(span) => *span,
            Self::Tilde(span) => *span,
        }
    }
}
#[derive(Debug, Clone)]
pub enum OperatorDeclarationChildren<'tree> {
    AttributeList(::std::boxed::Box<AttributeList<'tree>>),
    ExplicitInterfaceSpecifier(::std::boxed::Box<ExplicitInterfaceSpecifier<'tree>>),
    Modifier(::std::boxed::Box<Modifier<'tree>>),
    PreprocIfInAttributeList(::std::boxed::Box<PreprocIfInAttributeList<'tree>>),
}
impl<'tree> ::treesitter_types::FromNode<'tree> for OperatorDeclarationChildren<'tree> {
    #[allow(clippy::collapsible_else_if)]
    fn from_node(
        node: ::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        match node.kind() {
            "attribute_list" => Ok(Self::AttributeList(::std::boxed::Box::new(
                <AttributeList as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "explicit_interface_specifier" => {
                Ok(Self::ExplicitInterfaceSpecifier(::std::boxed::Box::new(
                    <ExplicitInterfaceSpecifier as ::treesitter_types::FromNode>::from_node(
                        node, src,
                    )?,
                )))
            }
            "modifier" => Ok(Self::Modifier(::std::boxed::Box::new(
                <Modifier as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "preproc_if_in_attribute_list" => {
                Ok(Self::PreprocIfInAttributeList(::std::boxed::Box::new(
                    <PreprocIfInAttributeList as ::treesitter_types::FromNode>::from_node(
                        node, src,
                    )?,
                )))
            }
            other => Err(::treesitter_types::ParseError::unexpected_kind(other, node)),
        }
    }
}
impl ::treesitter_types::Spanned for OperatorDeclarationChildren<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        match self {
            Self::AttributeList(inner) => inner.span(),
            Self::ExplicitInterfaceSpecifier(inner) => inner.span(),
            Self::Modifier(inner) => inner.span(),
            Self::PreprocIfInAttributeList(inner) => inner.span(),
        }
    }
}
#[derive(Debug, Clone)]
pub enum OrPatternOperator {
    Or(::treesitter_types::Span),
}
impl<'tree> ::treesitter_types::FromNode<'tree> for OrPatternOperator {
    #[allow(clippy::collapsible_else_if)]
    fn from_node(
        node: ::tree_sitter::Node<'tree>,
        _src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        match node.kind() {
            "or" => Ok(Self::Or(::treesitter_types::Span::from(node))),
            other => Err(::treesitter_types::ParseError::unexpected_kind(other, node)),
        }
    }
}
impl ::treesitter_types::Spanned for OrPatternOperator {
    fn span(&self) -> ::treesitter_types::Span {
        match self {
            Self::Or(span) => *span,
        }
    }
}
#[derive(Debug, Clone)]
pub enum ParameterChildren<'tree> {
    AttributeList(::std::boxed::Box<AttributeList<'tree>>),
    Expression(::std::boxed::Box<Expression<'tree>>),
    Modifier(::std::boxed::Box<Modifier<'tree>>),
    PreprocIfInAttributeList(::std::boxed::Box<PreprocIfInAttributeList<'tree>>),
}
impl<'tree> ::treesitter_types::FromNode<'tree> for ParameterChildren<'tree> {
    #[allow(clippy::collapsible_else_if)]
    fn from_node(
        node: ::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        match node.kind() {
            "attribute_list" => Ok(Self::AttributeList(::std::boxed::Box::new(
                <AttributeList as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "modifier" => Ok(Self::Modifier(::std::boxed::Box::new(
                <Modifier as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "preproc_if_in_attribute_list" => {
                Ok(Self::PreprocIfInAttributeList(::std::boxed::Box::new(
                    <PreprocIfInAttributeList as ::treesitter_types::FromNode>::from_node(
                        node, src,
                    )?,
                )))
            }
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
impl ::treesitter_types::Spanned for ParameterChildren<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        match self {
            Self::AttributeList(inner) => inner.span(),
            Self::Expression(inner) => inner.span(),
            Self::Modifier(inner) => inner.span(),
            Self::PreprocIfInAttributeList(inner) => inner.span(),
        }
    }
}
#[derive(Debug, Clone)]
pub enum ParameterListChildren<'tree> {
    AttributeList(::std::boxed::Box<AttributeList<'tree>>),
    Parameter(::std::boxed::Box<Parameter<'tree>>),
    PreprocIfInAttributeList(::std::boxed::Box<PreprocIfInAttributeList<'tree>>),
}
impl<'tree> ::treesitter_types::FromNode<'tree> for ParameterListChildren<'tree> {
    #[allow(clippy::collapsible_else_if)]
    fn from_node(
        node: ::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        match node.kind() {
            "attribute_list" => Ok(Self::AttributeList(::std::boxed::Box::new(
                <AttributeList as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "parameter" => Ok(Self::Parameter(::std::boxed::Box::new(
                <Parameter as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "preproc_if_in_attribute_list" => {
                Ok(Self::PreprocIfInAttributeList(::std::boxed::Box::new(
                    <PreprocIfInAttributeList as ::treesitter_types::FromNode>::from_node(
                        node, src,
                    )?,
                )))
            }
            other => Err(::treesitter_types::ParseError::unexpected_kind(other, node)),
        }
    }
}
impl ::treesitter_types::Spanned for ParameterListChildren<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        match self {
            Self::AttributeList(inner) => inner.span(),
            Self::Parameter(inner) => inner.span(),
            Self::PreprocIfInAttributeList(inner) => inner.span(),
        }
    }
}
#[derive(Debug, Clone)]
pub enum ParenthesizedExpressionChildren<'tree> {
    BooleanLiteral(::std::boxed::Box<BooleanLiteral<'tree>>),
    CharacterLiteral(::std::boxed::Box<CharacterLiteral<'tree>>),
    IntegerLiteral(::std::boxed::Box<IntegerLiteral<'tree>>),
    LvalueExpression(::std::boxed::Box<LvalueExpression<'tree>>),
    NonLvalueExpression(::std::boxed::Box<NonLvalueExpression<'tree>>),
    UnaryExpression(::std::boxed::Box<UnaryExpression<'tree>>),
}
impl<'tree> ::treesitter_types::FromNode<'tree> for ParenthesizedExpressionChildren<'tree> {
    #[allow(clippy::collapsible_else_if)]
    fn from_node(
        node: ::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        match node.kind() {
            "boolean_literal" => Ok(Self::BooleanLiteral(::std::boxed::Box::new(
                <BooleanLiteral as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "character_literal" => Ok(Self::CharacterLiteral(::std::boxed::Box::new(
                <CharacterLiteral as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "integer_literal" => Ok(Self::IntegerLiteral(::std::boxed::Box::new(
                <IntegerLiteral as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "unary_expression" => Ok(Self::UnaryExpression(::std::boxed::Box::new(
                <UnaryExpression as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            _other => {
                if let Ok(v) =
                    <LvalueExpression as ::treesitter_types::FromNode>::from_node(node, src)
                {
                    Ok(Self::LvalueExpression(::std::boxed::Box::new(v)))
                } else {
                    if let Ok(v) =
                        <NonLvalueExpression as ::treesitter_types::FromNode>::from_node(node, src)
                    {
                        Ok(Self::NonLvalueExpression(::std::boxed::Box::new(v)))
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
impl ::treesitter_types::Spanned for ParenthesizedExpressionChildren<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        match self {
            Self::BooleanLiteral(inner) => inner.span(),
            Self::CharacterLiteral(inner) => inner.span(),
            Self::IntegerLiteral(inner) => inner.span(),
            Self::LvalueExpression(inner) => inner.span(),
            Self::NonLvalueExpression(inner) => inner.span(),
            Self::UnaryExpression(inner) => inner.span(),
        }
    }
}
#[derive(Debug, Clone)]
pub enum ParenthesizedVariableDesignationChildren<'tree> {
    Discard(::std::boxed::Box<Discard<'tree>>),
    ParenthesizedVariableDesignation(::std::boxed::Box<ParenthesizedVariableDesignation<'tree>>),
}
impl<'tree> ::treesitter_types::FromNode<'tree>
    for ParenthesizedVariableDesignationChildren<'tree>
{
    #[allow(clippy::collapsible_else_if)]
    fn from_node(
        node: ::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        match node.kind() {
            "discard" => Ok(Self::Discard(::std::boxed::Box::new(
                <Discard as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "parenthesized_variable_designation" => Ok(Self::ParenthesizedVariableDesignation(
                ::std::boxed::Box::new(
                    <ParenthesizedVariableDesignation as ::treesitter_types::FromNode>::from_node(
                        node, src,
                    )?,
                ),
            )),
            other => Err(::treesitter_types::ParseError::unexpected_kind(other, node)),
        }
    }
}
impl ::treesitter_types::Spanned for ParenthesizedVariableDesignationChildren<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        match self {
            Self::Discard(inner) => inner.span(),
            Self::ParenthesizedVariableDesignation(inner) => inner.span(),
        }
    }
}
#[derive(Debug, Clone)]
pub enum PointerTypeType<'tree> {
    AliasQualifiedName(::std::boxed::Box<AliasQualifiedName<'tree>>),
    FunctionPointerType(::std::boxed::Box<FunctionPointerType<'tree>>),
    GenericName(::std::boxed::Box<GenericName<'tree>>),
    Identifier(::std::boxed::Box<Identifier<'tree>>),
    NullableType(::std::boxed::Box<NullableType<'tree>>),
    PointerType(::std::boxed::Box<PointerType<'tree>>),
    PredefinedType(::std::boxed::Box<PredefinedType<'tree>>),
    QualifiedName(::std::boxed::Box<QualifiedName<'tree>>),
    TupleType(::std::boxed::Box<TupleType<'tree>>),
}
impl<'tree> ::treesitter_types::FromNode<'tree> for PointerTypeType<'tree> {
    #[allow(clippy::collapsible_else_if)]
    fn from_node(
        node: ::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        match node.kind() {
            "alias_qualified_name" => Ok(Self::AliasQualifiedName(::std::boxed::Box::new(
                <AliasQualifiedName as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "function_pointer_type" => Ok(Self::FunctionPointerType(::std::boxed::Box::new(
                <FunctionPointerType as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "generic_name" => Ok(Self::GenericName(::std::boxed::Box::new(
                <GenericName as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "identifier" => Ok(Self::Identifier(::std::boxed::Box::new(
                <Identifier as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "nullable_type" => Ok(Self::NullableType(::std::boxed::Box::new(
                <NullableType as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "pointer_type" => Ok(Self::PointerType(::std::boxed::Box::new(
                <PointerType as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "predefined_type" => Ok(Self::PredefinedType(::std::boxed::Box::new(
                <PredefinedType as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "qualified_name" => Ok(Self::QualifiedName(::std::boxed::Box::new(
                <QualifiedName as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "tuple_type" => Ok(Self::TupleType(::std::boxed::Box::new(
                <TupleType as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            other => Err(::treesitter_types::ParseError::unexpected_kind(other, node)),
        }
    }
}
impl ::treesitter_types::Spanned for PointerTypeType<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        match self {
            Self::AliasQualifiedName(inner) => inner.span(),
            Self::FunctionPointerType(inner) => inner.span(),
            Self::GenericName(inner) => inner.span(),
            Self::Identifier(inner) => inner.span(),
            Self::NullableType(inner) => inner.span(),
            Self::PointerType(inner) => inner.span(),
            Self::PredefinedType(inner) => inner.span(),
            Self::QualifiedName(inner) => inner.span(),
            Self::TupleType(inner) => inner.span(),
        }
    }
}
#[derive(Debug, Clone)]
pub enum PreprocElifAlternative<'tree> {
    PreprocElif(::std::boxed::Box<PreprocElif<'tree>>),
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
            Self::PreprocElse(inner) => inner.span(),
        }
    }
}
#[derive(Debug, Clone)]
pub enum PreprocElifCondition<'tree> {
    BinaryExpression(::std::boxed::Box<BinaryExpression<'tree>>),
    BooleanLiteral(::std::boxed::Box<BooleanLiteral<'tree>>),
    CharacterLiteral(::std::boxed::Box<CharacterLiteral<'tree>>),
    Identifier(::std::boxed::Box<Identifier<'tree>>),
    IntegerLiteral(::std::boxed::Box<IntegerLiteral<'tree>>),
    ParenthesizedExpression(::std::boxed::Box<ParenthesizedExpression<'tree>>),
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
            "boolean_literal" => Ok(Self::BooleanLiteral(::std::boxed::Box::new(
                <BooleanLiteral as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "character_literal" => Ok(Self::CharacterLiteral(::std::boxed::Box::new(
                <CharacterLiteral as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "identifier" => Ok(Self::Identifier(::std::boxed::Box::new(
                <Identifier as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "integer_literal" => Ok(Self::IntegerLiteral(::std::boxed::Box::new(
                <IntegerLiteral as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "parenthesized_expression" => {
                Ok(Self::ParenthesizedExpression(::std::boxed::Box::new(
                    <ParenthesizedExpression as ::treesitter_types::FromNode>::from_node(
                        node, src,
                    )?,
                )))
            }
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
            Self::BooleanLiteral(inner) => inner.span(),
            Self::CharacterLiteral(inner) => inner.span(),
            Self::Identifier(inner) => inner.span(),
            Self::IntegerLiteral(inner) => inner.span(),
            Self::ParenthesizedExpression(inner) => inner.span(),
            Self::UnaryExpression(inner) => inner.span(),
        }
    }
}
#[derive(Debug, Clone)]
pub enum PreprocElifChildren<'tree> {
    AttributeList(::std::boxed::Box<AttributeList<'tree>>),
    Declaration(::std::boxed::Box<Declaration<'tree>>),
    EnumMemberDeclaration(::std::boxed::Box<EnumMemberDeclaration<'tree>>),
    Expression(::std::boxed::Box<Expression<'tree>>),
    ExternAliasDirective(::std::boxed::Box<ExternAliasDirective<'tree>>),
    FileScopedNamespaceDeclaration(::std::boxed::Box<FileScopedNamespaceDeclaration<'tree>>),
    GlobalAttribute(::std::boxed::Box<GlobalAttribute<'tree>>),
    Statement(::std::boxed::Box<Statement<'tree>>),
    TypeDeclaration(::std::boxed::Box<TypeDeclaration<'tree>>),
}
impl<'tree> ::treesitter_types::FromNode<'tree> for PreprocElifChildren<'tree> {
    #[allow(clippy::collapsible_else_if)]
    fn from_node(
        node: ::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        match node.kind() {
            "attribute_list" => Ok(Self::AttributeList(::std::boxed::Box::new(
                <AttributeList as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "enum_member_declaration" => Ok(Self::EnumMemberDeclaration(::std::boxed::Box::new(
                <EnumMemberDeclaration as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "extern_alias_directive" => Ok(Self::ExternAliasDirective(::std::boxed::Box::new(
                <ExternAliasDirective as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "file_scoped_namespace_declaration" => Ok(Self::FileScopedNamespaceDeclaration(
                ::std::boxed::Box::new(
                    <FileScopedNamespaceDeclaration as ::treesitter_types::FromNode>::from_node(
                        node, src,
                    )?,
                ),
            )),
            "global_attribute" => Ok(Self::GlobalAttribute(::std::boxed::Box::new(
                <GlobalAttribute as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            _other => {
                if let Ok(v) = <Declaration as ::treesitter_types::FromNode>::from_node(node, src) {
                    Ok(Self::Declaration(::std::boxed::Box::new(v)))
                } else {
                    if let Ok(v) =
                        <Expression as ::treesitter_types::FromNode>::from_node(node, src)
                    {
                        Ok(Self::Expression(::std::boxed::Box::new(v)))
                    } else {
                        if let Ok(v) =
                            <Statement as ::treesitter_types::FromNode>::from_node(node, src)
                        {
                            Ok(Self::Statement(::std::boxed::Box::new(v)))
                        } else {
                            if let Ok(v) =
                                <TypeDeclaration as ::treesitter_types::FromNode>::from_node(
                                    node, src,
                                )
                            {
                                Ok(Self::TypeDeclaration(::std::boxed::Box::new(v)))
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
}
impl ::treesitter_types::Spanned for PreprocElifChildren<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        match self {
            Self::AttributeList(inner) => inner.span(),
            Self::Declaration(inner) => inner.span(),
            Self::EnumMemberDeclaration(inner) => inner.span(),
            Self::Expression(inner) => inner.span(),
            Self::ExternAliasDirective(inner) => inner.span(),
            Self::FileScopedNamespaceDeclaration(inner) => inner.span(),
            Self::GlobalAttribute(inner) => inner.span(),
            Self::Statement(inner) => inner.span(),
            Self::TypeDeclaration(inner) => inner.span(),
        }
    }
}
#[derive(Debug, Clone)]
pub enum PreprocElseChildren<'tree> {
    AttributeList(::std::boxed::Box<AttributeList<'tree>>),
    Declaration(::std::boxed::Box<Declaration<'tree>>),
    EnumMemberDeclaration(::std::boxed::Box<EnumMemberDeclaration<'tree>>),
    Expression(::std::boxed::Box<Expression<'tree>>),
    ExternAliasDirective(::std::boxed::Box<ExternAliasDirective<'tree>>),
    FileScopedNamespaceDeclaration(::std::boxed::Box<FileScopedNamespaceDeclaration<'tree>>),
    GlobalAttribute(::std::boxed::Box<GlobalAttribute<'tree>>),
    Statement(::std::boxed::Box<Statement<'tree>>),
    TypeDeclaration(::std::boxed::Box<TypeDeclaration<'tree>>),
}
impl<'tree> ::treesitter_types::FromNode<'tree> for PreprocElseChildren<'tree> {
    #[allow(clippy::collapsible_else_if)]
    fn from_node(
        node: ::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        match node.kind() {
            "attribute_list" => Ok(Self::AttributeList(::std::boxed::Box::new(
                <AttributeList as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "enum_member_declaration" => Ok(Self::EnumMemberDeclaration(::std::boxed::Box::new(
                <EnumMemberDeclaration as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "extern_alias_directive" => Ok(Self::ExternAliasDirective(::std::boxed::Box::new(
                <ExternAliasDirective as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "file_scoped_namespace_declaration" => Ok(Self::FileScopedNamespaceDeclaration(
                ::std::boxed::Box::new(
                    <FileScopedNamespaceDeclaration as ::treesitter_types::FromNode>::from_node(
                        node, src,
                    )?,
                ),
            )),
            "global_attribute" => Ok(Self::GlobalAttribute(::std::boxed::Box::new(
                <GlobalAttribute as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            _other => {
                if let Ok(v) = <Declaration as ::treesitter_types::FromNode>::from_node(node, src) {
                    Ok(Self::Declaration(::std::boxed::Box::new(v)))
                } else {
                    if let Ok(v) =
                        <Expression as ::treesitter_types::FromNode>::from_node(node, src)
                    {
                        Ok(Self::Expression(::std::boxed::Box::new(v)))
                    } else {
                        if let Ok(v) =
                            <Statement as ::treesitter_types::FromNode>::from_node(node, src)
                        {
                            Ok(Self::Statement(::std::boxed::Box::new(v)))
                        } else {
                            if let Ok(v) =
                                <TypeDeclaration as ::treesitter_types::FromNode>::from_node(
                                    node, src,
                                )
                            {
                                Ok(Self::TypeDeclaration(::std::boxed::Box::new(v)))
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
}
impl ::treesitter_types::Spanned for PreprocElseChildren<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        match self {
            Self::AttributeList(inner) => inner.span(),
            Self::Declaration(inner) => inner.span(),
            Self::EnumMemberDeclaration(inner) => inner.span(),
            Self::Expression(inner) => inner.span(),
            Self::ExternAliasDirective(inner) => inner.span(),
            Self::FileScopedNamespaceDeclaration(inner) => inner.span(),
            Self::GlobalAttribute(inner) => inner.span(),
            Self::Statement(inner) => inner.span(),
            Self::TypeDeclaration(inner) => inner.span(),
        }
    }
}
#[derive(Debug, Clone)]
pub enum PreprocIfAlternative<'tree> {
    PreprocElif(::std::boxed::Box<PreprocElif<'tree>>),
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
            Self::PreprocElse(inner) => inner.span(),
        }
    }
}
#[derive(Debug, Clone)]
pub enum PreprocIfCondition<'tree> {
    BinaryExpression(::std::boxed::Box<BinaryExpression<'tree>>),
    BooleanLiteral(::std::boxed::Box<BooleanLiteral<'tree>>),
    CharacterLiteral(::std::boxed::Box<CharacterLiteral<'tree>>),
    Identifier(::std::boxed::Box<Identifier<'tree>>),
    IntegerLiteral(::std::boxed::Box<IntegerLiteral<'tree>>),
    ParenthesizedExpression(::std::boxed::Box<ParenthesizedExpression<'tree>>),
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
            "boolean_literal" => Ok(Self::BooleanLiteral(::std::boxed::Box::new(
                <BooleanLiteral as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "character_literal" => Ok(Self::CharacterLiteral(::std::boxed::Box::new(
                <CharacterLiteral as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "identifier" => Ok(Self::Identifier(::std::boxed::Box::new(
                <Identifier as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "integer_literal" => Ok(Self::IntegerLiteral(::std::boxed::Box::new(
                <IntegerLiteral as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "parenthesized_expression" => {
                Ok(Self::ParenthesizedExpression(::std::boxed::Box::new(
                    <ParenthesizedExpression as ::treesitter_types::FromNode>::from_node(
                        node, src,
                    )?,
                )))
            }
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
            Self::BooleanLiteral(inner) => inner.span(),
            Self::CharacterLiteral(inner) => inner.span(),
            Self::Identifier(inner) => inner.span(),
            Self::IntegerLiteral(inner) => inner.span(),
            Self::ParenthesizedExpression(inner) => inner.span(),
            Self::UnaryExpression(inner) => inner.span(),
        }
    }
}
#[derive(Debug, Clone)]
pub enum PreprocIfChildren<'tree> {
    Declaration(::std::boxed::Box<Declaration<'tree>>),
    EnumMemberDeclaration(::std::boxed::Box<EnumMemberDeclaration<'tree>>),
    Expression(::std::boxed::Box<Expression<'tree>>),
    ExternAliasDirective(::std::boxed::Box<ExternAliasDirective<'tree>>),
    FileScopedNamespaceDeclaration(::std::boxed::Box<FileScopedNamespaceDeclaration<'tree>>),
    GlobalAttribute(::std::boxed::Box<GlobalAttribute<'tree>>),
    Statement(::std::boxed::Box<Statement<'tree>>),
    TypeDeclaration(::std::boxed::Box<TypeDeclaration<'tree>>),
}
impl<'tree> ::treesitter_types::FromNode<'tree> for PreprocIfChildren<'tree> {
    #[allow(clippy::collapsible_else_if)]
    fn from_node(
        node: ::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        match node.kind() {
            "enum_member_declaration" => Ok(Self::EnumMemberDeclaration(::std::boxed::Box::new(
                <EnumMemberDeclaration as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "extern_alias_directive" => Ok(Self::ExternAliasDirective(::std::boxed::Box::new(
                <ExternAliasDirective as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "file_scoped_namespace_declaration" => Ok(Self::FileScopedNamespaceDeclaration(
                ::std::boxed::Box::new(
                    <FileScopedNamespaceDeclaration as ::treesitter_types::FromNode>::from_node(
                        node, src,
                    )?,
                ),
            )),
            "global_attribute" => Ok(Self::GlobalAttribute(::std::boxed::Box::new(
                <GlobalAttribute as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            _other => {
                if let Ok(v) = <Declaration as ::treesitter_types::FromNode>::from_node(node, src) {
                    Ok(Self::Declaration(::std::boxed::Box::new(v)))
                } else {
                    if let Ok(v) =
                        <Expression as ::treesitter_types::FromNode>::from_node(node, src)
                    {
                        Ok(Self::Expression(::std::boxed::Box::new(v)))
                    } else {
                        if let Ok(v) =
                            <Statement as ::treesitter_types::FromNode>::from_node(node, src)
                        {
                            Ok(Self::Statement(::std::boxed::Box::new(v)))
                        } else {
                            if let Ok(v) =
                                <TypeDeclaration as ::treesitter_types::FromNode>::from_node(
                                    node, src,
                                )
                            {
                                Ok(Self::TypeDeclaration(::std::boxed::Box::new(v)))
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
}
impl ::treesitter_types::Spanned for PreprocIfChildren<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        match self {
            Self::Declaration(inner) => inner.span(),
            Self::EnumMemberDeclaration(inner) => inner.span(),
            Self::Expression(inner) => inner.span(),
            Self::ExternAliasDirective(inner) => inner.span(),
            Self::FileScopedNamespaceDeclaration(inner) => inner.span(),
            Self::GlobalAttribute(inner) => inner.span(),
            Self::Statement(inner) => inner.span(),
            Self::TypeDeclaration(inner) => inner.span(),
        }
    }
}
#[derive(Debug, Clone)]
pub enum PreprocIfInAttributeListAlternative<'tree> {
    PreprocElif(::std::boxed::Box<PreprocElif<'tree>>),
    PreprocElse(::std::boxed::Box<PreprocElse<'tree>>),
}
impl<'tree> ::treesitter_types::FromNode<'tree> for PreprocIfInAttributeListAlternative<'tree> {
    #[allow(clippy::collapsible_else_if)]
    fn from_node(
        node: ::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        match node.kind() {
            "preproc_elif" => Ok(Self::PreprocElif(::std::boxed::Box::new(
                <PreprocElif as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "preproc_else" => Ok(Self::PreprocElse(::std::boxed::Box::new(
                <PreprocElse as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            other => Err(::treesitter_types::ParseError::unexpected_kind(other, node)),
        }
    }
}
impl ::treesitter_types::Spanned for PreprocIfInAttributeListAlternative<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        match self {
            Self::PreprocElif(inner) => inner.span(),
            Self::PreprocElse(inner) => inner.span(),
        }
    }
}
#[derive(Debug, Clone)]
pub enum PreprocIfInAttributeListCondition<'tree> {
    BinaryExpression(::std::boxed::Box<BinaryExpression<'tree>>),
    BooleanLiteral(::std::boxed::Box<BooleanLiteral<'tree>>),
    CharacterLiteral(::std::boxed::Box<CharacterLiteral<'tree>>),
    Identifier(::std::boxed::Box<Identifier<'tree>>),
    IntegerLiteral(::std::boxed::Box<IntegerLiteral<'tree>>),
    ParenthesizedExpression(::std::boxed::Box<ParenthesizedExpression<'tree>>),
    UnaryExpression(::std::boxed::Box<UnaryExpression<'tree>>),
}
impl<'tree> ::treesitter_types::FromNode<'tree> for PreprocIfInAttributeListCondition<'tree> {
    #[allow(clippy::collapsible_else_if)]
    fn from_node(
        node: ::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        match node.kind() {
            "binary_expression" => Ok(Self::BinaryExpression(::std::boxed::Box::new(
                <BinaryExpression as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "boolean_literal" => Ok(Self::BooleanLiteral(::std::boxed::Box::new(
                <BooleanLiteral as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "character_literal" => Ok(Self::CharacterLiteral(::std::boxed::Box::new(
                <CharacterLiteral as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "identifier" => Ok(Self::Identifier(::std::boxed::Box::new(
                <Identifier as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "integer_literal" => Ok(Self::IntegerLiteral(::std::boxed::Box::new(
                <IntegerLiteral as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "parenthesized_expression" => {
                Ok(Self::ParenthesizedExpression(::std::boxed::Box::new(
                    <ParenthesizedExpression as ::treesitter_types::FromNode>::from_node(
                        node, src,
                    )?,
                )))
            }
            "unary_expression" => Ok(Self::UnaryExpression(::std::boxed::Box::new(
                <UnaryExpression as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            other => Err(::treesitter_types::ParseError::unexpected_kind(other, node)),
        }
    }
}
impl ::treesitter_types::Spanned for PreprocIfInAttributeListCondition<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        match self {
            Self::BinaryExpression(inner) => inner.span(),
            Self::BooleanLiteral(inner) => inner.span(),
            Self::CharacterLiteral(inner) => inner.span(),
            Self::Identifier(inner) => inner.span(),
            Self::IntegerLiteral(inner) => inner.span(),
            Self::ParenthesizedExpression(inner) => inner.span(),
            Self::UnaryExpression(inner) => inner.span(),
        }
    }
}
#[derive(Debug, Clone)]
pub enum PreprocLineChildren<'tree> {
    IntegerLiteral(::std::boxed::Box<IntegerLiteral<'tree>>),
    StringLiteral(::std::boxed::Box<StringLiteral<'tree>>),
}
impl<'tree> ::treesitter_types::FromNode<'tree> for PreprocLineChildren<'tree> {
    #[allow(clippy::collapsible_else_if)]
    fn from_node(
        node: ::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        match node.kind() {
            "integer_literal" => Ok(Self::IntegerLiteral(::std::boxed::Box::new(
                <IntegerLiteral as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "string_literal" => Ok(Self::StringLiteral(::std::boxed::Box::new(
                <StringLiteral as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            other => Err(::treesitter_types::ParseError::unexpected_kind(other, node)),
        }
    }
}
impl ::treesitter_types::Spanned for PreprocLineChildren<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        match self {
            Self::IntegerLiteral(inner) => inner.span(),
            Self::StringLiteral(inner) => inner.span(),
        }
    }
}
#[derive(Debug, Clone)]
pub enum PreprocPragmaChildren<'tree> {
    Identifier(::std::boxed::Box<Identifier<'tree>>),
    IntegerLiteral(::std::boxed::Box<IntegerLiteral<'tree>>),
    StringLiteral(::std::boxed::Box<StringLiteral<'tree>>),
}
impl<'tree> ::treesitter_types::FromNode<'tree> for PreprocPragmaChildren<'tree> {
    #[allow(clippy::collapsible_else_if)]
    fn from_node(
        node: ::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        match node.kind() {
            "identifier" => Ok(Self::Identifier(::std::boxed::Box::new(
                <Identifier as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "integer_literal" => Ok(Self::IntegerLiteral(::std::boxed::Box::new(
                <IntegerLiteral as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "string_literal" => Ok(Self::StringLiteral(::std::boxed::Box::new(
                <StringLiteral as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            other => Err(::treesitter_types::ParseError::unexpected_kind(other, node)),
        }
    }
}
impl ::treesitter_types::Spanned for PreprocPragmaChildren<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        match self {
            Self::Identifier(inner) => inner.span(),
            Self::IntegerLiteral(inner) => inner.span(),
            Self::StringLiteral(inner) => inner.span(),
        }
    }
}
#[derive(Debug, Clone)]
pub enum PrimaryConstructorBaseTypeType<'tree> {
    AliasQualifiedName(::std::boxed::Box<AliasQualifiedName<'tree>>),
    GenericName(::std::boxed::Box<GenericName<'tree>>),
    Identifier(::std::boxed::Box<Identifier<'tree>>),
    QualifiedName(::std::boxed::Box<QualifiedName<'tree>>),
}
impl<'tree> ::treesitter_types::FromNode<'tree> for PrimaryConstructorBaseTypeType<'tree> {
    #[allow(clippy::collapsible_else_if)]
    fn from_node(
        node: ::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        match node.kind() {
            "alias_qualified_name" => Ok(Self::AliasQualifiedName(::std::boxed::Box::new(
                <AliasQualifiedName as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "generic_name" => Ok(Self::GenericName(::std::boxed::Box::new(
                <GenericName as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "identifier" => Ok(Self::Identifier(::std::boxed::Box::new(
                <Identifier as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "qualified_name" => Ok(Self::QualifiedName(::std::boxed::Box::new(
                <QualifiedName as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            other => Err(::treesitter_types::ParseError::unexpected_kind(other, node)),
        }
    }
}
impl ::treesitter_types::Spanned for PrimaryConstructorBaseTypeType<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        match self {
            Self::AliasQualifiedName(inner) => inner.span(),
            Self::GenericName(inner) => inner.span(),
            Self::Identifier(inner) => inner.span(),
            Self::QualifiedName(inner) => inner.span(),
        }
    }
}
#[derive(Debug, Clone)]
pub enum PropertyDeclarationValue<'tree> {
    ArrowExpressionClause(::std::boxed::Box<ArrowExpressionClause<'tree>>),
    Expression(::std::boxed::Box<Expression<'tree>>),
}
impl<'tree> ::treesitter_types::FromNode<'tree> for PropertyDeclarationValue<'tree> {
    #[allow(clippy::collapsible_else_if)]
    fn from_node(
        node: ::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        match node.kind() {
            "arrow_expression_clause" => Ok(Self::ArrowExpressionClause(::std::boxed::Box::new(
                <ArrowExpressionClause as ::treesitter_types::FromNode>::from_node(node, src)?,
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
impl ::treesitter_types::Spanned for PropertyDeclarationValue<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        match self {
            Self::ArrowExpressionClause(inner) => inner.span(),
            Self::Expression(inner) => inner.span(),
        }
    }
}
#[derive(Debug, Clone)]
pub enum PropertyDeclarationChildren<'tree> {
    AttributeList(::std::boxed::Box<AttributeList<'tree>>),
    ExplicitInterfaceSpecifier(::std::boxed::Box<ExplicitInterfaceSpecifier<'tree>>),
    Modifier(::std::boxed::Box<Modifier<'tree>>),
    PreprocIfInAttributeList(::std::boxed::Box<PreprocIfInAttributeList<'tree>>),
}
impl<'tree> ::treesitter_types::FromNode<'tree> for PropertyDeclarationChildren<'tree> {
    #[allow(clippy::collapsible_else_if)]
    fn from_node(
        node: ::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        match node.kind() {
            "attribute_list" => Ok(Self::AttributeList(::std::boxed::Box::new(
                <AttributeList as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "explicit_interface_specifier" => {
                Ok(Self::ExplicitInterfaceSpecifier(::std::boxed::Box::new(
                    <ExplicitInterfaceSpecifier as ::treesitter_types::FromNode>::from_node(
                        node, src,
                    )?,
                )))
            }
            "modifier" => Ok(Self::Modifier(::std::boxed::Box::new(
                <Modifier as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "preproc_if_in_attribute_list" => {
                Ok(Self::PreprocIfInAttributeList(::std::boxed::Box::new(
                    <PreprocIfInAttributeList as ::treesitter_types::FromNode>::from_node(
                        node, src,
                    )?,
                )))
            }
            other => Err(::treesitter_types::ParseError::unexpected_kind(other, node)),
        }
    }
}
impl ::treesitter_types::Spanned for PropertyDeclarationChildren<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        match self {
            Self::AttributeList(inner) => inner.span(),
            Self::ExplicitInterfaceSpecifier(inner) => inner.span(),
            Self::Modifier(inner) => inner.span(),
            Self::PreprocIfInAttributeList(inner) => inner.span(),
        }
    }
}
#[derive(Debug, Clone)]
pub enum QualifiedNameName<'tree> {
    GenericName(::std::boxed::Box<GenericName<'tree>>),
    Identifier(::std::boxed::Box<Identifier<'tree>>),
}
impl<'tree> ::treesitter_types::FromNode<'tree> for QualifiedNameName<'tree> {
    #[allow(clippy::collapsible_else_if)]
    fn from_node(
        node: ::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        match node.kind() {
            "generic_name" => Ok(Self::GenericName(::std::boxed::Box::new(
                <GenericName as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "identifier" => Ok(Self::Identifier(::std::boxed::Box::new(
                <Identifier as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            other => Err(::treesitter_types::ParseError::unexpected_kind(other, node)),
        }
    }
}
impl ::treesitter_types::Spanned for QualifiedNameName<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        match self {
            Self::GenericName(inner) => inner.span(),
            Self::Identifier(inner) => inner.span(),
        }
    }
}
#[derive(Debug, Clone)]
pub enum QualifiedNameQualifier<'tree> {
    AliasQualifiedName(::std::boxed::Box<AliasQualifiedName<'tree>>),
    GenericName(::std::boxed::Box<GenericName<'tree>>),
    Identifier(::std::boxed::Box<Identifier<'tree>>),
    QualifiedName(::std::boxed::Box<QualifiedName<'tree>>),
}
impl<'tree> ::treesitter_types::FromNode<'tree> for QualifiedNameQualifier<'tree> {
    #[allow(clippy::collapsible_else_if)]
    fn from_node(
        node: ::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        match node.kind() {
            "alias_qualified_name" => Ok(Self::AliasQualifiedName(::std::boxed::Box::new(
                <AliasQualifiedName as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "generic_name" => Ok(Self::GenericName(::std::boxed::Box::new(
                <GenericName as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "identifier" => Ok(Self::Identifier(::std::boxed::Box::new(
                <Identifier as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "qualified_name" => Ok(Self::QualifiedName(::std::boxed::Box::new(
                <QualifiedName as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            other => Err(::treesitter_types::ParseError::unexpected_kind(other, node)),
        }
    }
}
impl ::treesitter_types::Spanned for QualifiedNameQualifier<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        match self {
            Self::AliasQualifiedName(inner) => inner.span(),
            Self::GenericName(inner) => inner.span(),
            Self::Identifier(inner) => inner.span(),
            Self::QualifiedName(inner) => inner.span(),
        }
    }
}
#[derive(Debug, Clone)]
pub enum QueryExpressionChildren<'tree> {
    FromClause(::std::boxed::Box<FromClause<'tree>>),
    GroupClause(::std::boxed::Box<GroupClause<'tree>>),
    Identifier(::std::boxed::Box<Identifier<'tree>>),
    JoinClause(::std::boxed::Box<JoinClause<'tree>>),
    LetClause(::std::boxed::Box<LetClause<'tree>>),
    OrderByClause(::std::boxed::Box<OrderByClause<'tree>>),
    SelectClause(::std::boxed::Box<SelectClause<'tree>>),
    WhereClause(::std::boxed::Box<WhereClause<'tree>>),
}
impl<'tree> ::treesitter_types::FromNode<'tree> for QueryExpressionChildren<'tree> {
    #[allow(clippy::collapsible_else_if)]
    fn from_node(
        node: ::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        match node.kind() {
            "from_clause" => Ok(Self::FromClause(::std::boxed::Box::new(
                <FromClause as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "group_clause" => Ok(Self::GroupClause(::std::boxed::Box::new(
                <GroupClause as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "identifier" => Ok(Self::Identifier(::std::boxed::Box::new(
                <Identifier as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "join_clause" => Ok(Self::JoinClause(::std::boxed::Box::new(
                <JoinClause as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "let_clause" => Ok(Self::LetClause(::std::boxed::Box::new(
                <LetClause as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "order_by_clause" => Ok(Self::OrderByClause(::std::boxed::Box::new(
                <OrderByClause as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "select_clause" => Ok(Self::SelectClause(::std::boxed::Box::new(
                <SelectClause as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "where_clause" => Ok(Self::WhereClause(::std::boxed::Box::new(
                <WhereClause as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            other => Err(::treesitter_types::ParseError::unexpected_kind(other, node)),
        }
    }
}
impl ::treesitter_types::Spanned for QueryExpressionChildren<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        match self {
            Self::FromClause(inner) => inner.span(),
            Self::GroupClause(inner) => inner.span(),
            Self::Identifier(inner) => inner.span(),
            Self::JoinClause(inner) => inner.span(),
            Self::LetClause(inner) => inner.span(),
            Self::OrderByClause(inner) => inner.span(),
            Self::SelectClause(inner) => inner.span(),
            Self::WhereClause(inner) => inner.span(),
        }
    }
}
#[derive(Debug, Clone)]
pub enum RawStringLiteralChildren<'tree> {
    RawStringContent(::std::boxed::Box<RawStringContent<'tree>>),
    RawStringEnd(::std::boxed::Box<RawStringEnd<'tree>>),
    RawStringStart(::std::boxed::Box<RawStringStart<'tree>>),
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
            "raw_string_end" => Ok(Self::RawStringEnd(::std::boxed::Box::new(
                <RawStringEnd as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "raw_string_start" => Ok(Self::RawStringStart(::std::boxed::Box::new(
                <RawStringStart as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            other => Err(::treesitter_types::ParseError::unexpected_kind(other, node)),
        }
    }
}
impl ::treesitter_types::Spanned for RawStringLiteralChildren<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        match self {
            Self::RawStringContent(inner) => inner.span(),
            Self::RawStringEnd(inner) => inner.span(),
            Self::RawStringStart(inner) => inner.span(),
        }
    }
}
#[derive(Debug, Clone)]
pub enum RecordDeclarationChildren<'tree> {
    AttributeList(::std::boxed::Box<AttributeList<'tree>>),
    BaseList(::std::boxed::Box<BaseList<'tree>>),
    Modifier(::std::boxed::Box<Modifier<'tree>>),
    ParameterList(::std::boxed::Box<ParameterList<'tree>>),
    PreprocIfInAttributeList(::std::boxed::Box<PreprocIfInAttributeList<'tree>>),
    TypeParameterConstraintsClause(::std::boxed::Box<TypeParameterConstraintsClause<'tree>>),
    TypeParameterList(::std::boxed::Box<TypeParameterList<'tree>>),
}
impl<'tree> ::treesitter_types::FromNode<'tree> for RecordDeclarationChildren<'tree> {
    #[allow(clippy::collapsible_else_if)]
    fn from_node(
        node: ::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        match node.kind() {
            "attribute_list" => Ok(Self::AttributeList(::std::boxed::Box::new(
                <AttributeList as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "base_list" => Ok(Self::BaseList(::std::boxed::Box::new(
                <BaseList as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "modifier" => Ok(Self::Modifier(::std::boxed::Box::new(
                <Modifier as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "parameter_list" => Ok(Self::ParameterList(::std::boxed::Box::new(
                <ParameterList as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "preproc_if_in_attribute_list" => {
                Ok(Self::PreprocIfInAttributeList(::std::boxed::Box::new(
                    <PreprocIfInAttributeList as ::treesitter_types::FromNode>::from_node(
                        node, src,
                    )?,
                )))
            }
            "type_parameter_constraints_clause" => Ok(Self::TypeParameterConstraintsClause(
                ::std::boxed::Box::new(
                    <TypeParameterConstraintsClause as ::treesitter_types::FromNode>::from_node(
                        node, src,
                    )?,
                ),
            )),
            "type_parameter_list" => Ok(Self::TypeParameterList(::std::boxed::Box::new(
                <TypeParameterList as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            other => Err(::treesitter_types::ParseError::unexpected_kind(other, node)),
        }
    }
}
impl ::treesitter_types::Spanned for RecordDeclarationChildren<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        match self {
            Self::AttributeList(inner) => inner.span(),
            Self::BaseList(inner) => inner.span(),
            Self::Modifier(inner) => inner.span(),
            Self::ParameterList(inner) => inner.span(),
            Self::PreprocIfInAttributeList(inner) => inner.span(),
            Self::TypeParameterConstraintsClause(inner) => inner.span(),
            Self::TypeParameterList(inner) => inner.span(),
        }
    }
}
#[derive(Debug, Clone)]
pub enum RecursivePatternChildren<'tree> {
    ParenthesizedVariableDesignation(::std::boxed::Box<ParenthesizedVariableDesignation<'tree>>),
    Pattern(::std::boxed::Box<Pattern<'tree>>),
    PositionalPatternClause(::std::boxed::Box<PositionalPatternClause<'tree>>),
    PropertyPatternClause(::std::boxed::Box<PropertyPatternClause<'tree>>),
}
impl<'tree> ::treesitter_types::FromNode<'tree> for RecursivePatternChildren<'tree> {
    #[allow(clippy::collapsible_else_if)]
    fn from_node(
        node: ::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        match node.kind() {
            "parenthesized_variable_designation" => Ok(Self::ParenthesizedVariableDesignation(
                ::std::boxed::Box::new(
                    <ParenthesizedVariableDesignation as ::treesitter_types::FromNode>::from_node(
                        node, src,
                    )?,
                ),
            )),
            "positional_pattern_clause" => {
                Ok(Self::PositionalPatternClause(::std::boxed::Box::new(
                    <PositionalPatternClause as ::treesitter_types::FromNode>::from_node(
                        node, src,
                    )?,
                )))
            }
            "property_pattern_clause" => Ok(Self::PropertyPatternClause(::std::boxed::Box::new(
                <PropertyPatternClause as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            _other => {
                if let Ok(v) = <Pattern as ::treesitter_types::FromNode>::from_node(node, src) {
                    Ok(Self::Pattern(::std::boxed::Box::new(v)))
                } else {
                    Err(::treesitter_types::ParseError::unexpected_kind(
                        _other, node,
                    ))
                }
            }
        }
    }
}
impl ::treesitter_types::Spanned for RecursivePatternChildren<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        match self {
            Self::ParenthesizedVariableDesignation(inner) => inner.span(),
            Self::Pattern(inner) => inner.span(),
            Self::PositionalPatternClause(inner) => inner.span(),
            Self::PropertyPatternClause(inner) => inner.span(),
        }
    }
}
#[derive(Debug, Clone)]
pub enum ScopedTypeType<'tree> {
    AliasQualifiedName(::std::boxed::Box<AliasQualifiedName<'tree>>),
    GenericName(::std::boxed::Box<GenericName<'tree>>),
    Identifier(::std::boxed::Box<Identifier<'tree>>),
    QualifiedName(::std::boxed::Box<QualifiedName<'tree>>),
    RefType(::std::boxed::Box<RefType<'tree>>),
}
impl<'tree> ::treesitter_types::FromNode<'tree> for ScopedTypeType<'tree> {
    #[allow(clippy::collapsible_else_if)]
    fn from_node(
        node: ::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        match node.kind() {
            "alias_qualified_name" => Ok(Self::AliasQualifiedName(::std::boxed::Box::new(
                <AliasQualifiedName as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "generic_name" => Ok(Self::GenericName(::std::boxed::Box::new(
                <GenericName as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "identifier" => Ok(Self::Identifier(::std::boxed::Box::new(
                <Identifier as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "qualified_name" => Ok(Self::QualifiedName(::std::boxed::Box::new(
                <QualifiedName as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "ref_type" => Ok(Self::RefType(::std::boxed::Box::new(
                <RefType as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            other => Err(::treesitter_types::ParseError::unexpected_kind(other, node)),
        }
    }
}
impl ::treesitter_types::Spanned for ScopedTypeType<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        match self {
            Self::AliasQualifiedName(inner) => inner.span(),
            Self::GenericName(inner) => inner.span(),
            Self::Identifier(inner) => inner.span(),
            Self::QualifiedName(inner) => inner.span(),
            Self::RefType(inner) => inner.span(),
        }
    }
}
#[derive(Debug, Clone)]
pub enum StringLiteralChildren<'tree> {
    EscapeSequence(::std::boxed::Box<EscapeSequence<'tree>>),
    StringLiteralContent(::std::boxed::Box<StringLiteralContent<'tree>>),
    StringLiteralEncoding(::std::boxed::Box<StringLiteralEncoding<'tree>>),
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
            "string_literal_content" => Ok(Self::StringLiteralContent(::std::boxed::Box::new(
                <StringLiteralContent as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "string_literal_encoding" => Ok(Self::StringLiteralEncoding(::std::boxed::Box::new(
                <StringLiteralEncoding as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            other => Err(::treesitter_types::ParseError::unexpected_kind(other, node)),
        }
    }
}
impl ::treesitter_types::Spanned for StringLiteralChildren<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        match self {
            Self::EscapeSequence(inner) => inner.span(),
            Self::StringLiteralContent(inner) => inner.span(),
            Self::StringLiteralEncoding(inner) => inner.span(),
        }
    }
}
#[derive(Debug, Clone)]
pub enum StructDeclarationChildren<'tree> {
    AttributeList(::std::boxed::Box<AttributeList<'tree>>),
    BaseList(::std::boxed::Box<BaseList<'tree>>),
    Modifier(::std::boxed::Box<Modifier<'tree>>),
    ParameterList(::std::boxed::Box<ParameterList<'tree>>),
    PreprocIfInAttributeList(::std::boxed::Box<PreprocIfInAttributeList<'tree>>),
    TypeParameterConstraintsClause(::std::boxed::Box<TypeParameterConstraintsClause<'tree>>),
    TypeParameterList(::std::boxed::Box<TypeParameterList<'tree>>),
}
impl<'tree> ::treesitter_types::FromNode<'tree> for StructDeclarationChildren<'tree> {
    #[allow(clippy::collapsible_else_if)]
    fn from_node(
        node: ::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        match node.kind() {
            "attribute_list" => Ok(Self::AttributeList(::std::boxed::Box::new(
                <AttributeList as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "base_list" => Ok(Self::BaseList(::std::boxed::Box::new(
                <BaseList as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "modifier" => Ok(Self::Modifier(::std::boxed::Box::new(
                <Modifier as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "parameter_list" => Ok(Self::ParameterList(::std::boxed::Box::new(
                <ParameterList as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "preproc_if_in_attribute_list" => {
                Ok(Self::PreprocIfInAttributeList(::std::boxed::Box::new(
                    <PreprocIfInAttributeList as ::treesitter_types::FromNode>::from_node(
                        node, src,
                    )?,
                )))
            }
            "type_parameter_constraints_clause" => Ok(Self::TypeParameterConstraintsClause(
                ::std::boxed::Box::new(
                    <TypeParameterConstraintsClause as ::treesitter_types::FromNode>::from_node(
                        node, src,
                    )?,
                ),
            )),
            "type_parameter_list" => Ok(Self::TypeParameterList(::std::boxed::Box::new(
                <TypeParameterList as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            other => Err(::treesitter_types::ParseError::unexpected_kind(other, node)),
        }
    }
}
impl ::treesitter_types::Spanned for StructDeclarationChildren<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        match self {
            Self::AttributeList(inner) => inner.span(),
            Self::BaseList(inner) => inner.span(),
            Self::Modifier(inner) => inner.span(),
            Self::ParameterList(inner) => inner.span(),
            Self::PreprocIfInAttributeList(inner) => inner.span(),
            Self::TypeParameterConstraintsClause(inner) => inner.span(),
            Self::TypeParameterList(inner) => inner.span(),
        }
    }
}
#[derive(Debug, Clone)]
pub enum SubpatternChildren<'tree> {
    Expression(::std::boxed::Box<Expression<'tree>>),
    Identifier(::std::boxed::Box<Identifier<'tree>>),
    Pattern(::std::boxed::Box<Pattern<'tree>>),
}
impl<'tree> ::treesitter_types::FromNode<'tree> for SubpatternChildren<'tree> {
    #[allow(clippy::collapsible_else_if)]
    fn from_node(
        node: ::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        match node.kind() {
            "identifier" => Ok(Self::Identifier(::std::boxed::Box::new(
                <Identifier as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            _other => {
                if let Ok(v) = <Expression as ::treesitter_types::FromNode>::from_node(node, src) {
                    Ok(Self::Expression(::std::boxed::Box::new(v)))
                } else {
                    if let Ok(v) = <Pattern as ::treesitter_types::FromNode>::from_node(node, src) {
                        Ok(Self::Pattern(::std::boxed::Box::new(v)))
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
impl ::treesitter_types::Spanned for SubpatternChildren<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        match self {
            Self::Expression(inner) => inner.span(),
            Self::Identifier(inner) => inner.span(),
            Self::Pattern(inner) => inner.span(),
        }
    }
}
#[derive(Debug, Clone)]
pub enum SwitchExpressionChildren<'tree> {
    Expression(::std::boxed::Box<Expression<'tree>>),
    SwitchExpressionArm(::std::boxed::Box<SwitchExpressionArm<'tree>>),
}
impl<'tree> ::treesitter_types::FromNode<'tree> for SwitchExpressionChildren<'tree> {
    #[allow(clippy::collapsible_else_if)]
    fn from_node(
        node: ::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        match node.kind() {
            "switch_expression_arm" => Ok(Self::SwitchExpressionArm(::std::boxed::Box::new(
                <SwitchExpressionArm as ::treesitter_types::FromNode>::from_node(node, src)?,
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
impl ::treesitter_types::Spanned for SwitchExpressionChildren<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        match self {
            Self::Expression(inner) => inner.span(),
            Self::SwitchExpressionArm(inner) => inner.span(),
        }
    }
}
#[derive(Debug, Clone)]
pub enum SwitchExpressionArmChildren<'tree> {
    Expression(::std::boxed::Box<Expression<'tree>>),
    Pattern(::std::boxed::Box<Pattern<'tree>>),
    WhenClause(::std::boxed::Box<WhenClause<'tree>>),
}
impl<'tree> ::treesitter_types::FromNode<'tree> for SwitchExpressionArmChildren<'tree> {
    #[allow(clippy::collapsible_else_if)]
    fn from_node(
        node: ::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        match node.kind() {
            "when_clause" => Ok(Self::WhenClause(::std::boxed::Box::new(
                <WhenClause as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            _other => {
                if let Ok(v) = <Expression as ::treesitter_types::FromNode>::from_node(node, src) {
                    Ok(Self::Expression(::std::boxed::Box::new(v)))
                } else {
                    if let Ok(v) = <Pattern as ::treesitter_types::FromNode>::from_node(node, src) {
                        Ok(Self::Pattern(::std::boxed::Box::new(v)))
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
impl ::treesitter_types::Spanned for SwitchExpressionArmChildren<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        match self {
            Self::Expression(inner) => inner.span(),
            Self::Pattern(inner) => inner.span(),
            Self::WhenClause(inner) => inner.span(),
        }
    }
}
#[derive(Debug, Clone)]
pub enum SwitchSectionChildren<'tree> {
    Expression(::std::boxed::Box<Expression<'tree>>),
    Pattern(::std::boxed::Box<Pattern<'tree>>),
    Statement(::std::boxed::Box<Statement<'tree>>),
    WhenClause(::std::boxed::Box<WhenClause<'tree>>),
}
impl<'tree> ::treesitter_types::FromNode<'tree> for SwitchSectionChildren<'tree> {
    #[allow(clippy::collapsible_else_if)]
    fn from_node(
        node: ::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        match node.kind() {
            "when_clause" => Ok(Self::WhenClause(::std::boxed::Box::new(
                <WhenClause as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            _other => {
                if let Ok(v) = <Expression as ::treesitter_types::FromNode>::from_node(node, src) {
                    Ok(Self::Expression(::std::boxed::Box::new(v)))
                } else {
                    if let Ok(v) = <Pattern as ::treesitter_types::FromNode>::from_node(node, src) {
                        Ok(Self::Pattern(::std::boxed::Box::new(v)))
                    } else {
                        if let Ok(v) =
                            <Statement as ::treesitter_types::FromNode>::from_node(node, src)
                        {
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
}
impl ::treesitter_types::Spanned for SwitchSectionChildren<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        match self {
            Self::Expression(inner) => inner.span(),
            Self::Pattern(inner) => inner.span(),
            Self::Statement(inner) => inner.span(),
            Self::WhenClause(inner) => inner.span(),
        }
    }
}
#[derive(Debug, Clone)]
pub enum SwitchStatementValue<'tree> {
    Expression(::std::boxed::Box<Expression<'tree>>),
    TupleExpression(::std::boxed::Box<TupleExpression<'tree>>),
}
impl<'tree> ::treesitter_types::FromNode<'tree> for SwitchStatementValue<'tree> {
    #[allow(clippy::collapsible_else_if)]
    fn from_node(
        node: ::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        match node.kind() {
            "tuple_expression" => Ok(Self::TupleExpression(::std::boxed::Box::new(
                <TupleExpression as ::treesitter_types::FromNode>::from_node(node, src)?,
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
impl ::treesitter_types::Spanned for SwitchStatementValue<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        match self {
            Self::Expression(inner) => inner.span(),
            Self::TupleExpression(inner) => inner.span(),
        }
    }
}
#[derive(Debug, Clone)]
pub enum TryStatementChildren<'tree> {
    CatchClause(::std::boxed::Box<CatchClause<'tree>>),
    FinallyClause(::std::boxed::Box<FinallyClause<'tree>>),
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
            "finally_clause" => Ok(Self::FinallyClause(::std::boxed::Box::new(
                <FinallyClause as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            other => Err(::treesitter_types::ParseError::unexpected_kind(other, node)),
        }
    }
}
impl ::treesitter_types::Spanned for TryStatementChildren<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        match self {
            Self::CatchClause(inner) => inner.span(),
            Self::FinallyClause(inner) => inner.span(),
        }
    }
}
#[derive(Debug, Clone)]
pub enum TuplePatternChildren<'tree> {
    Discard(::std::boxed::Box<Discard<'tree>>),
    TuplePattern(::std::boxed::Box<TuplePattern<'tree>>),
}
impl<'tree> ::treesitter_types::FromNode<'tree> for TuplePatternChildren<'tree> {
    #[allow(clippy::collapsible_else_if)]
    fn from_node(
        node: ::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        match node.kind() {
            "discard" => Ok(Self::Discard(::std::boxed::Box::new(
                <Discard as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "tuple_pattern" => Ok(Self::TuplePattern(::std::boxed::Box::new(
                <TuplePattern as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            other => Err(::treesitter_types::ParseError::unexpected_kind(other, node)),
        }
    }
}
impl ::treesitter_types::Spanned for TuplePatternChildren<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        match self {
            Self::Discard(inner) => inner.span(),
            Self::TuplePattern(inner) => inner.span(),
        }
    }
}
#[derive(Debug, Clone)]
pub enum TypeParameterChildren<'tree> {
    AttributeList(::std::boxed::Box<AttributeList<'tree>>),
    PreprocIfInAttributeList(::std::boxed::Box<PreprocIfInAttributeList<'tree>>),
}
impl<'tree> ::treesitter_types::FromNode<'tree> for TypeParameterChildren<'tree> {
    #[allow(clippy::collapsible_else_if)]
    fn from_node(
        node: ::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        match node.kind() {
            "attribute_list" => Ok(Self::AttributeList(::std::boxed::Box::new(
                <AttributeList as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "preproc_if_in_attribute_list" => {
                Ok(Self::PreprocIfInAttributeList(::std::boxed::Box::new(
                    <PreprocIfInAttributeList as ::treesitter_types::FromNode>::from_node(
                        node, src,
                    )?,
                )))
            }
            other => Err(::treesitter_types::ParseError::unexpected_kind(other, node)),
        }
    }
}
impl ::treesitter_types::Spanned for TypeParameterChildren<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        match self {
            Self::AttributeList(inner) => inner.span(),
            Self::PreprocIfInAttributeList(inner) => inner.span(),
        }
    }
}
#[derive(Debug, Clone)]
pub enum TypeParameterConstraintsClauseChildren<'tree> {
    Identifier(::std::boxed::Box<Identifier<'tree>>),
    TypeParameterConstraint(::std::boxed::Box<TypeParameterConstraint<'tree>>),
}
impl<'tree> ::treesitter_types::FromNode<'tree> for TypeParameterConstraintsClauseChildren<'tree> {
    #[allow(clippy::collapsible_else_if)]
    fn from_node(
        node: ::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        match node.kind() {
            "identifier" => Ok(Self::Identifier(::std::boxed::Box::new(
                <Identifier as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "type_parameter_constraint" => {
                Ok(Self::TypeParameterConstraint(::std::boxed::Box::new(
                    <TypeParameterConstraint as ::treesitter_types::FromNode>::from_node(
                        node, src,
                    )?,
                )))
            }
            other => Err(::treesitter_types::ParseError::unexpected_kind(other, node)),
        }
    }
}
impl ::treesitter_types::Spanned for TypeParameterConstraintsClauseChildren<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        match self {
            Self::Identifier(inner) => inner.span(),
            Self::TypeParameterConstraint(inner) => inner.span(),
        }
    }
}
#[derive(Debug, Clone)]
pub enum UnaryExpressionArgument<'tree> {
    BinaryExpression(::std::boxed::Box<BinaryExpression<'tree>>),
    BooleanLiteral(::std::boxed::Box<BooleanLiteral<'tree>>),
    CharacterLiteral(::std::boxed::Box<CharacterLiteral<'tree>>),
    Identifier(::std::boxed::Box<Identifier<'tree>>),
    IntegerLiteral(::std::boxed::Box<IntegerLiteral<'tree>>),
    ParenthesizedExpression(::std::boxed::Box<ParenthesizedExpression<'tree>>),
    UnaryExpression(::std::boxed::Box<UnaryExpression<'tree>>),
}
impl<'tree> ::treesitter_types::FromNode<'tree> for UnaryExpressionArgument<'tree> {
    #[allow(clippy::collapsible_else_if)]
    fn from_node(
        node: ::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        match node.kind() {
            "binary_expression" => Ok(Self::BinaryExpression(::std::boxed::Box::new(
                <BinaryExpression as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "boolean_literal" => Ok(Self::BooleanLiteral(::std::boxed::Box::new(
                <BooleanLiteral as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "character_literal" => Ok(Self::CharacterLiteral(::std::boxed::Box::new(
                <CharacterLiteral as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "identifier" => Ok(Self::Identifier(::std::boxed::Box::new(
                <Identifier as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "integer_literal" => Ok(Self::IntegerLiteral(::std::boxed::Box::new(
                <IntegerLiteral as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "parenthesized_expression" => {
                Ok(Self::ParenthesizedExpression(::std::boxed::Box::new(
                    <ParenthesizedExpression as ::treesitter_types::FromNode>::from_node(
                        node, src,
                    )?,
                )))
            }
            "unary_expression" => Ok(Self::UnaryExpression(::std::boxed::Box::new(
                <UnaryExpression as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            other => Err(::treesitter_types::ParseError::unexpected_kind(other, node)),
        }
    }
}
impl ::treesitter_types::Spanned for UnaryExpressionArgument<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        match self {
            Self::BinaryExpression(inner) => inner.span(),
            Self::BooleanLiteral(inner) => inner.span(),
            Self::CharacterLiteral(inner) => inner.span(),
            Self::Identifier(inner) => inner.span(),
            Self::IntegerLiteral(inner) => inner.span(),
            Self::ParenthesizedExpression(inner) => inner.span(),
            Self::UnaryExpression(inner) => inner.span(),
        }
    }
}
#[derive(Debug, Clone)]
pub enum UnaryExpressionOperator {
    Bang(::treesitter_types::Span),
}
impl<'tree> ::treesitter_types::FromNode<'tree> for UnaryExpressionOperator {
    #[allow(clippy::collapsible_else_if)]
    fn from_node(
        node: ::tree_sitter::Node<'tree>,
        _src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        match node.kind() {
            "!" => Ok(Self::Bang(::treesitter_types::Span::from(node))),
            other => Err(::treesitter_types::ParseError::unexpected_kind(other, node)),
        }
    }
}
impl ::treesitter_types::Spanned for UnaryExpressionOperator {
    fn span(&self) -> ::treesitter_types::Span {
        match self {
            Self::Bang(span) => *span,
        }
    }
}
#[derive(Debug, Clone)]
pub enum UsingStatementChildren<'tree> {
    Expression(::std::boxed::Box<Expression<'tree>>),
    VariableDeclaration(::std::boxed::Box<VariableDeclaration<'tree>>),
}
impl<'tree> ::treesitter_types::FromNode<'tree> for UsingStatementChildren<'tree> {
    #[allow(clippy::collapsible_else_if)]
    fn from_node(
        node: ::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        match node.kind() {
            "variable_declaration" => Ok(Self::VariableDeclaration(::std::boxed::Box::new(
                <VariableDeclaration as ::treesitter_types::FromNode>::from_node(node, src)?,
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
impl ::treesitter_types::Spanned for UsingStatementChildren<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        match self {
            Self::Expression(inner) => inner.span(),
            Self::VariableDeclaration(inner) => inner.span(),
        }
    }
}
#[derive(Debug, Clone)]
pub enum VarPatternChildren<'tree> {
    Discard(::std::boxed::Box<Discard<'tree>>),
    ParenthesizedVariableDesignation(::std::boxed::Box<ParenthesizedVariableDesignation<'tree>>),
}
impl<'tree> ::treesitter_types::FromNode<'tree> for VarPatternChildren<'tree> {
    #[allow(clippy::collapsible_else_if)]
    fn from_node(
        node: ::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        match node.kind() {
            "discard" => Ok(Self::Discard(::std::boxed::Box::new(
                <Discard as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "parenthesized_variable_designation" => Ok(Self::ParenthesizedVariableDesignation(
                ::std::boxed::Box::new(
                    <ParenthesizedVariableDesignation as ::treesitter_types::FromNode>::from_node(
                        node, src,
                    )?,
                ),
            )),
            other => Err(::treesitter_types::ParseError::unexpected_kind(other, node)),
        }
    }
}
impl ::treesitter_types::Spanned for VarPatternChildren<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        match self {
            Self::Discard(inner) => inner.span(),
            Self::ParenthesizedVariableDesignation(inner) => inner.span(),
        }
    }
}
#[derive(Debug, Clone)]
pub enum VariableDeclaratorChildren<'tree> {
    BracketedArgumentList(::std::boxed::Box<BracketedArgumentList<'tree>>),
    Expression(::std::boxed::Box<Expression<'tree>>),
    TuplePattern(::std::boxed::Box<TuplePattern<'tree>>),
}
impl<'tree> ::treesitter_types::FromNode<'tree> for VariableDeclaratorChildren<'tree> {
    #[allow(clippy::collapsible_else_if)]
    fn from_node(
        node: ::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        match node.kind() {
            "bracketed_argument_list" => Ok(Self::BracketedArgumentList(::std::boxed::Box::new(
                <BracketedArgumentList as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "tuple_pattern" => Ok(Self::TuplePattern(::std::boxed::Box::new(
                <TuplePattern as ::treesitter_types::FromNode>::from_node(node, src)?,
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
impl ::treesitter_types::Spanned for VariableDeclaratorChildren<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        match self {
            Self::BracketedArgumentList(inner) => inner.span(),
            Self::Expression(inner) => inner.span(),
            Self::TuplePattern(inner) => inner.span(),
        }
    }
}
#[derive(Debug, Clone)]
pub enum WithExpressionChildren<'tree> {
    Expression(::std::boxed::Box<Expression<'tree>>),
    WithInitializer(::std::boxed::Box<WithInitializer<'tree>>),
}
impl<'tree> ::treesitter_types::FromNode<'tree> for WithExpressionChildren<'tree> {
    #[allow(clippy::collapsible_else_if)]
    fn from_node(
        node: ::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        match node.kind() {
            "with_initializer" => Ok(Self::WithInitializer(::std::boxed::Box::new(
                <WithInitializer as ::treesitter_types::FromNode>::from_node(node, src)?,
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
impl ::treesitter_types::Spanned for WithExpressionChildren<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        match self {
            Self::Expression(inner) => inner.span(),
            Self::WithInitializer(inner) => inner.span(),
        }
    }
}
#[derive(Debug, Clone)]
pub enum WithInitializerChildren<'tree> {
    Expression(::std::boxed::Box<Expression<'tree>>),
    Identifier(::std::boxed::Box<Identifier<'tree>>),
}
impl<'tree> ::treesitter_types::FromNode<'tree> for WithInitializerChildren<'tree> {
    #[allow(clippy::collapsible_else_if)]
    fn from_node(
        node: ::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        match node.kind() {
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
impl ::treesitter_types::Spanned for WithInitializerChildren<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        match self {
            Self::Expression(inner) => inner.span(),
            Self::Identifier(inner) => inner.span(),
        }
    }
}
#[derive(Debug, Clone)]
pub enum AnyNode<'tree> {
    Declaration(Declaration<'tree>),
    Expression(Expression<'tree>),
    Literal(Literal<'tree>),
    LvalueExpression(LvalueExpression<'tree>),
    NonLvalueExpression(NonLvalueExpression<'tree>),
    Pattern(Pattern<'tree>),
    Statement(Statement<'tree>),
    Type(Type<'tree>),
    TypeDeclaration(TypeDeclaration<'tree>),
    AccessorDeclaration(AccessorDeclaration<'tree>),
    AccessorList(AccessorList<'tree>),
    AliasQualifiedName(AliasQualifiedName<'tree>),
    AndPattern(AndPattern<'tree>),
    AnonymousMethodExpression(AnonymousMethodExpression<'tree>),
    AnonymousObjectCreationExpression(AnonymousObjectCreationExpression<'tree>),
    Argument(Argument<'tree>),
    ArgumentList(ArgumentList<'tree>),
    ArrayCreationExpression(ArrayCreationExpression<'tree>),
    ArrayRankSpecifier(ArrayRankSpecifier<'tree>),
    ArrayType(ArrayType<'tree>),
    ArrowExpressionClause(ArrowExpressionClause<'tree>),
    AsExpression(AsExpression<'tree>),
    AssignmentExpression(AssignmentExpression<'tree>),
    Attribute(Attribute<'tree>),
    AttributeArgument(AttributeArgument<'tree>),
    AttributeArgumentList(AttributeArgumentList<'tree>),
    AttributeList(AttributeList<'tree>),
    AttributeTargetSpecifier(AttributeTargetSpecifier<'tree>),
    AwaitExpression(AwaitExpression<'tree>),
    BaseList(BaseList<'tree>),
    BinaryExpression(BinaryExpression<'tree>),
    Block(Block<'tree>),
    BooleanLiteral(BooleanLiteral<'tree>),
    BracketedArgumentList(BracketedArgumentList<'tree>),
    BracketedParameterList(BracketedParameterList<'tree>),
    BreakStatement(BreakStatement<'tree>),
    CallingConvention(CallingConvention<'tree>),
    CastExpression(CastExpression<'tree>),
    CatchClause(CatchClause<'tree>),
    CatchDeclaration(CatchDeclaration<'tree>),
    CatchFilterClause(CatchFilterClause<'tree>),
    CharacterLiteral(CharacterLiteral<'tree>),
    CheckedExpression(CheckedExpression<'tree>),
    CheckedStatement(CheckedStatement<'tree>),
    ClassDeclaration(ClassDeclaration<'tree>),
    CompilationUnit(CompilationUnit<'tree>),
    ConditionalAccessExpression(ConditionalAccessExpression<'tree>),
    ConditionalExpression(ConditionalExpression<'tree>),
    ConstantPattern(ConstantPattern<'tree>),
    ConstructorConstraint(ConstructorConstraint<'tree>),
    ConstructorDeclaration(ConstructorDeclaration<'tree>),
    ConstructorInitializer(ConstructorInitializer<'tree>),
    ContinueStatement(ContinueStatement<'tree>),
    ConversionOperatorDeclaration(ConversionOperatorDeclaration<'tree>),
    DeclarationExpression(DeclarationExpression<'tree>),
    DeclarationList(DeclarationList<'tree>),
    DeclarationPattern(DeclarationPattern<'tree>),
    DefaultExpression(DefaultExpression<'tree>),
    DelegateDeclaration(DelegateDeclaration<'tree>),
    DestructorDeclaration(DestructorDeclaration<'tree>),
    DoStatement(DoStatement<'tree>),
    ElementAccessExpression(ElementAccessExpression<'tree>),
    ElementBindingExpression(ElementBindingExpression<'tree>),
    EmptyStatement(EmptyStatement<'tree>),
    EnumDeclaration(EnumDeclaration<'tree>),
    EnumMemberDeclaration(EnumMemberDeclaration<'tree>),
    EnumMemberDeclarationList(EnumMemberDeclarationList<'tree>),
    EventDeclaration(EventDeclaration<'tree>),
    EventFieldDeclaration(EventFieldDeclaration<'tree>),
    ExplicitInterfaceSpecifier(ExplicitInterfaceSpecifier<'tree>),
    ExpressionStatement(ExpressionStatement<'tree>),
    ExternAliasDirective(ExternAliasDirective<'tree>),
    FieldDeclaration(FieldDeclaration<'tree>),
    FileScopedNamespaceDeclaration(FileScopedNamespaceDeclaration<'tree>),
    FinallyClause(FinallyClause<'tree>),
    FixedStatement(FixedStatement<'tree>),
    ForStatement(ForStatement<'tree>),
    ForeachStatement(ForeachStatement<'tree>),
    FromClause(FromClause<'tree>),
    FunctionPointerParameter(FunctionPointerParameter<'tree>),
    FunctionPointerType(FunctionPointerType<'tree>),
    GenericName(GenericName<'tree>),
    GlobalAttribute(GlobalAttribute<'tree>),
    GlobalStatement(GlobalStatement<'tree>),
    GotoStatement(GotoStatement<'tree>),
    GroupClause(GroupClause<'tree>),
    Identifier(Identifier<'tree>),
    IfStatement(IfStatement<'tree>),
    ImplicitArrayCreationExpression(ImplicitArrayCreationExpression<'tree>),
    ImplicitObjectCreationExpression(ImplicitObjectCreationExpression<'tree>),
    ImplicitParameter(ImplicitParameter<'tree>),
    ImplicitStackallocExpression(ImplicitStackallocExpression<'tree>),
    ImplicitType(ImplicitType<'tree>),
    IndexerDeclaration(IndexerDeclaration<'tree>),
    InitializerExpression(InitializerExpression<'tree>),
    InterfaceDeclaration(InterfaceDeclaration<'tree>),
    InterpolatedStringExpression(InterpolatedStringExpression<'tree>),
    Interpolation(Interpolation<'tree>),
    InterpolationAlignmentClause(InterpolationAlignmentClause<'tree>),
    InterpolationFormatClause(InterpolationFormatClause<'tree>),
    InvocationExpression(InvocationExpression<'tree>),
    IsExpression(IsExpression<'tree>),
    IsPatternExpression(IsPatternExpression<'tree>),
    JoinClause(JoinClause<'tree>),
    JoinIntoClause(JoinIntoClause<'tree>),
    LabeledStatement(LabeledStatement<'tree>),
    LambdaExpression(LambdaExpression<'tree>),
    LetClause(LetClause<'tree>),
    ListPattern(ListPattern<'tree>),
    LocalDeclarationStatement(LocalDeclarationStatement<'tree>),
    LocalFunctionStatement(LocalFunctionStatement<'tree>),
    LockStatement(LockStatement<'tree>),
    MakerefExpression(MakerefExpression<'tree>),
    MemberAccessExpression(MemberAccessExpression<'tree>),
    MemberBindingExpression(MemberBindingExpression<'tree>),
    MethodDeclaration(MethodDeclaration<'tree>),
    Modifier(Modifier<'tree>),
    NamespaceDeclaration(NamespaceDeclaration<'tree>),
    NegatedPattern(NegatedPattern<'tree>),
    NullableType(NullableType<'tree>),
    ObjectCreationExpression(ObjectCreationExpression<'tree>),
    OperatorDeclaration(OperatorDeclaration<'tree>),
    OrPattern(OrPattern<'tree>),
    OrderByClause(OrderByClause<'tree>),
    Parameter(Parameter<'tree>),
    ParameterList(ParameterList<'tree>),
    ParenthesizedExpression(ParenthesizedExpression<'tree>),
    ParenthesizedPattern(ParenthesizedPattern<'tree>),
    ParenthesizedVariableDesignation(ParenthesizedVariableDesignation<'tree>),
    PointerType(PointerType<'tree>),
    PositionalPatternClause(PositionalPatternClause<'tree>),
    PostfixUnaryExpression(PostfixUnaryExpression<'tree>),
    PrefixUnaryExpression(PrefixUnaryExpression<'tree>),
    PreprocDefine(PreprocDefine<'tree>),
    PreprocElif(PreprocElif<'tree>),
    PreprocElse(PreprocElse<'tree>),
    PreprocEndregion(PreprocEndregion<'tree>),
    PreprocError(PreprocError<'tree>),
    PreprocIf(PreprocIf<'tree>),
    PreprocIfInAttributeList(PreprocIfInAttributeList<'tree>),
    PreprocLine(PreprocLine<'tree>),
    PreprocNullable(PreprocNullable<'tree>),
    PreprocPragma(PreprocPragma<'tree>),
    PreprocRegion(PreprocRegion<'tree>),
    PreprocUndef(PreprocUndef<'tree>),
    PreprocWarning(PreprocWarning<'tree>),
    PrimaryConstructorBaseType(PrimaryConstructorBaseType<'tree>),
    PropertyDeclaration(PropertyDeclaration<'tree>),
    PropertyPatternClause(PropertyPatternClause<'tree>),
    QualifiedName(QualifiedName<'tree>),
    QueryExpression(QueryExpression<'tree>),
    RangeExpression(RangeExpression<'tree>),
    RawStringLiteral(RawStringLiteral<'tree>),
    RecordDeclaration(RecordDeclaration<'tree>),
    RecursivePattern(RecursivePattern<'tree>),
    RefExpression(RefExpression<'tree>),
    RefType(RefType<'tree>),
    ReftypeExpression(ReftypeExpression<'tree>),
    RefvalueExpression(RefvalueExpression<'tree>),
    RelationalPattern(RelationalPattern<'tree>),
    ReturnStatement(ReturnStatement<'tree>),
    ScopedType(ScopedType<'tree>),
    SelectClause(SelectClause<'tree>),
    SizeofExpression(SizeofExpression<'tree>),
    StackallocExpression(StackallocExpression<'tree>),
    StringLiteral(StringLiteral<'tree>),
    StructDeclaration(StructDeclaration<'tree>),
    Subpattern(Subpattern<'tree>),
    SwitchBody(SwitchBody<'tree>),
    SwitchExpression(SwitchExpression<'tree>),
    SwitchExpressionArm(SwitchExpressionArm<'tree>),
    SwitchSection(SwitchSection<'tree>),
    SwitchStatement(SwitchStatement<'tree>),
    ThrowExpression(ThrowExpression<'tree>),
    ThrowStatement(ThrowStatement<'tree>),
    TryStatement(TryStatement<'tree>),
    TupleElement(TupleElement<'tree>),
    TupleExpression(TupleExpression<'tree>),
    TuplePattern(TuplePattern<'tree>),
    TupleType(TupleType<'tree>),
    TypeArgumentList(TypeArgumentList<'tree>),
    TypeParameter(TypeParameter<'tree>),
    TypeParameterConstraint(TypeParameterConstraint<'tree>),
    TypeParameterConstraintsClause(TypeParameterConstraintsClause<'tree>),
    TypeParameterList(TypeParameterList<'tree>),
    TypePattern(TypePattern<'tree>),
    TypeofExpression(TypeofExpression<'tree>),
    UnaryExpression(UnaryExpression<'tree>),
    UnsafeStatement(UnsafeStatement<'tree>),
    UsingDirective(UsingDirective<'tree>),
    UsingStatement(UsingStatement<'tree>),
    VarPattern(VarPattern<'tree>),
    VariableDeclaration(VariableDeclaration<'tree>),
    VariableDeclarator(VariableDeclarator<'tree>),
    WhenClause(WhenClause<'tree>),
    WhereClause(WhereClause<'tree>),
    WhileStatement(WhileStatement<'tree>),
    WithExpression(WithExpression<'tree>),
    WithInitializer(WithInitializer<'tree>),
    YieldStatement(YieldStatement<'tree>),
    CharacterLiteralContent(CharacterLiteralContent<'tree>),
    Comment(Comment<'tree>),
    Discard(Discard<'tree>),
    EscapeSequence(EscapeSequence<'tree>),
    IntegerLiteral(IntegerLiteral<'tree>),
    InterpolationBrace(InterpolationBrace<'tree>),
    InterpolationQuote(InterpolationQuote<'tree>),
    InterpolationStart(InterpolationStart<'tree>),
    NullLiteral(NullLiteral<'tree>),
    PredefinedType(PredefinedType<'tree>),
    PreprocArg(PreprocArg<'tree>),
    RawStringContent(RawStringContent<'tree>),
    RawStringEnd(RawStringEnd<'tree>),
    RawStringStart(RawStringStart<'tree>),
    RealLiteral(RealLiteral<'tree>),
    ShebangDirective(ShebangDirective<'tree>),
    StringContent(StringContent<'tree>),
    StringLiteralContent(StringLiteralContent<'tree>),
    StringLiteralEncoding(StringLiteralEncoding<'tree>),
    VerbatimStringLiteral(VerbatimStringLiteral<'tree>),
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
            "literal" => <Literal as ::treesitter_types::FromNode>::from_node(node, src)
                .map(Self::Literal)
                .unwrap_or(Self::Unknown(node)),
            "lvalue_expression" => {
                <LvalueExpression as ::treesitter_types::FromNode>::from_node(node, src)
                    .map(Self::LvalueExpression)
                    .unwrap_or(Self::Unknown(node))
            }
            "non_lvalue_expression" => {
                <NonLvalueExpression as ::treesitter_types::FromNode>::from_node(node, src)
                    .map(Self::NonLvalueExpression)
                    .unwrap_or(Self::Unknown(node))
            }
            "pattern" => <Pattern as ::treesitter_types::FromNode>::from_node(node, src)
                .map(Self::Pattern)
                .unwrap_or(Self::Unknown(node)),
            "statement" => <Statement as ::treesitter_types::FromNode>::from_node(node, src)
                .map(Self::Statement)
                .unwrap_or(Self::Unknown(node)),
            "type" => <Type as ::treesitter_types::FromNode>::from_node(node, src)
                .map(Self::Type)
                .unwrap_or(Self::Unknown(node)),
            "type_declaration" => {
                <TypeDeclaration as ::treesitter_types::FromNode>::from_node(node, src)
                    .map(Self::TypeDeclaration)
                    .unwrap_or(Self::Unknown(node))
            }
            "accessor_declaration" => {
                <AccessorDeclaration as ::treesitter_types::FromNode>::from_node(node, src)
                    .map(Self::AccessorDeclaration)
                    .unwrap_or(Self::Unknown(node))
            }
            "accessor_list" => <AccessorList as ::treesitter_types::FromNode>::from_node(node, src)
                .map(Self::AccessorList)
                .unwrap_or(Self::Unknown(node)),
            "alias_qualified_name" => {
                <AliasQualifiedName as ::treesitter_types::FromNode>::from_node(node, src)
                    .map(Self::AliasQualifiedName)
                    .unwrap_or(Self::Unknown(node))
            }
            "and_pattern" => <AndPattern as ::treesitter_types::FromNode>::from_node(node, src)
                .map(Self::AndPattern)
                .unwrap_or(Self::Unknown(node)),
            "anonymous_method_expression" => {
                <AnonymousMethodExpression as ::treesitter_types::FromNode>::from_node(node, src)
                    .map(Self::AnonymousMethodExpression)
                    .unwrap_or(Self::Unknown(node))
            }
            "anonymous_object_creation_expression" => {
                <AnonymousObjectCreationExpression as ::treesitter_types::FromNode>::from_node(
                    node, src,
                )
                .map(Self::AnonymousObjectCreationExpression)
                .unwrap_or(Self::Unknown(node))
            }
            "argument" => <Argument as ::treesitter_types::FromNode>::from_node(node, src)
                .map(Self::Argument)
                .unwrap_or(Self::Unknown(node)),
            "argument_list" => <ArgumentList as ::treesitter_types::FromNode>::from_node(node, src)
                .map(Self::ArgumentList)
                .unwrap_or(Self::Unknown(node)),
            "array_creation_expression" => {
                <ArrayCreationExpression as ::treesitter_types::FromNode>::from_node(node, src)
                    .map(Self::ArrayCreationExpression)
                    .unwrap_or(Self::Unknown(node))
            }
            "array_rank_specifier" => {
                <ArrayRankSpecifier as ::treesitter_types::FromNode>::from_node(node, src)
                    .map(Self::ArrayRankSpecifier)
                    .unwrap_or(Self::Unknown(node))
            }
            "array_type" => <ArrayType as ::treesitter_types::FromNode>::from_node(node, src)
                .map(Self::ArrayType)
                .unwrap_or(Self::Unknown(node)),
            "arrow_expression_clause" => {
                <ArrowExpressionClause as ::treesitter_types::FromNode>::from_node(node, src)
                    .map(Self::ArrowExpressionClause)
                    .unwrap_or(Self::Unknown(node))
            }
            "as_expression" => <AsExpression as ::treesitter_types::FromNode>::from_node(node, src)
                .map(Self::AsExpression)
                .unwrap_or(Self::Unknown(node)),
            "assignment_expression" => {
                <AssignmentExpression as ::treesitter_types::FromNode>::from_node(node, src)
                    .map(Self::AssignmentExpression)
                    .unwrap_or(Self::Unknown(node))
            }
            "attribute" => <Attribute as ::treesitter_types::FromNode>::from_node(node, src)
                .map(Self::Attribute)
                .unwrap_or(Self::Unknown(node)),
            "attribute_argument" => {
                <AttributeArgument as ::treesitter_types::FromNode>::from_node(node, src)
                    .map(Self::AttributeArgument)
                    .unwrap_or(Self::Unknown(node))
            }
            "attribute_argument_list" => {
                <AttributeArgumentList as ::treesitter_types::FromNode>::from_node(node, src)
                    .map(Self::AttributeArgumentList)
                    .unwrap_or(Self::Unknown(node))
            }
            "attribute_list" => {
                <AttributeList as ::treesitter_types::FromNode>::from_node(node, src)
                    .map(Self::AttributeList)
                    .unwrap_or(Self::Unknown(node))
            }
            "attribute_target_specifier" => {
                <AttributeTargetSpecifier as ::treesitter_types::FromNode>::from_node(node, src)
                    .map(Self::AttributeTargetSpecifier)
                    .unwrap_or(Self::Unknown(node))
            }
            "await_expression" => {
                <AwaitExpression as ::treesitter_types::FromNode>::from_node(node, src)
                    .map(Self::AwaitExpression)
                    .unwrap_or(Self::Unknown(node))
            }
            "base_list" => <BaseList as ::treesitter_types::FromNode>::from_node(node, src)
                .map(Self::BaseList)
                .unwrap_or(Self::Unknown(node)),
            "binary_expression" => {
                <BinaryExpression as ::treesitter_types::FromNode>::from_node(node, src)
                    .map(Self::BinaryExpression)
                    .unwrap_or(Self::Unknown(node))
            }
            "block" => <Block as ::treesitter_types::FromNode>::from_node(node, src)
                .map(Self::Block)
                .unwrap_or(Self::Unknown(node)),
            "boolean_literal" => {
                <BooleanLiteral as ::treesitter_types::FromNode>::from_node(node, src)
                    .map(Self::BooleanLiteral)
                    .unwrap_or(Self::Unknown(node))
            }
            "bracketed_argument_list" => {
                <BracketedArgumentList as ::treesitter_types::FromNode>::from_node(node, src)
                    .map(Self::BracketedArgumentList)
                    .unwrap_or(Self::Unknown(node))
            }
            "bracketed_parameter_list" => {
                <BracketedParameterList as ::treesitter_types::FromNode>::from_node(node, src)
                    .map(Self::BracketedParameterList)
                    .unwrap_or(Self::Unknown(node))
            }
            "break_statement" => {
                <BreakStatement as ::treesitter_types::FromNode>::from_node(node, src)
                    .map(Self::BreakStatement)
                    .unwrap_or(Self::Unknown(node))
            }
            "calling_convention" => {
                <CallingConvention as ::treesitter_types::FromNode>::from_node(node, src)
                    .map(Self::CallingConvention)
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
            "catch_declaration" => {
                <CatchDeclaration as ::treesitter_types::FromNode>::from_node(node, src)
                    .map(Self::CatchDeclaration)
                    .unwrap_or(Self::Unknown(node))
            }
            "catch_filter_clause" => {
                <CatchFilterClause as ::treesitter_types::FromNode>::from_node(node, src)
                    .map(Self::CatchFilterClause)
                    .unwrap_or(Self::Unknown(node))
            }
            "character_literal" => {
                <CharacterLiteral as ::treesitter_types::FromNode>::from_node(node, src)
                    .map(Self::CharacterLiteral)
                    .unwrap_or(Self::Unknown(node))
            }
            "checked_expression" => {
                <CheckedExpression as ::treesitter_types::FromNode>::from_node(node, src)
                    .map(Self::CheckedExpression)
                    .unwrap_or(Self::Unknown(node))
            }
            "checked_statement" => {
                <CheckedStatement as ::treesitter_types::FromNode>::from_node(node, src)
                    .map(Self::CheckedStatement)
                    .unwrap_or(Self::Unknown(node))
            }
            "class_declaration" => {
                <ClassDeclaration as ::treesitter_types::FromNode>::from_node(node, src)
                    .map(Self::ClassDeclaration)
                    .unwrap_or(Self::Unknown(node))
            }
            "compilation_unit" => {
                <CompilationUnit as ::treesitter_types::FromNode>::from_node(node, src)
                    .map(Self::CompilationUnit)
                    .unwrap_or(Self::Unknown(node))
            }
            "conditional_access_expression" => {
                <ConditionalAccessExpression as ::treesitter_types::FromNode>::from_node(node, src)
                    .map(Self::ConditionalAccessExpression)
                    .unwrap_or(Self::Unknown(node))
            }
            "conditional_expression" => {
                <ConditionalExpression as ::treesitter_types::FromNode>::from_node(node, src)
                    .map(Self::ConditionalExpression)
                    .unwrap_or(Self::Unknown(node))
            }
            "constant_pattern" => {
                <ConstantPattern as ::treesitter_types::FromNode>::from_node(node, src)
                    .map(Self::ConstantPattern)
                    .unwrap_or(Self::Unknown(node))
            }
            "constructor_constraint" => {
                <ConstructorConstraint as ::treesitter_types::FromNode>::from_node(node, src)
                    .map(Self::ConstructorConstraint)
                    .unwrap_or(Self::Unknown(node))
            }
            "constructor_declaration" => {
                <ConstructorDeclaration as ::treesitter_types::FromNode>::from_node(node, src)
                    .map(Self::ConstructorDeclaration)
                    .unwrap_or(Self::Unknown(node))
            }
            "constructor_initializer" => {
                <ConstructorInitializer as ::treesitter_types::FromNode>::from_node(node, src)
                    .map(Self::ConstructorInitializer)
                    .unwrap_or(Self::Unknown(node))
            }
            "continue_statement" => {
                <ContinueStatement as ::treesitter_types::FromNode>::from_node(node, src)
                    .map(Self::ContinueStatement)
                    .unwrap_or(Self::Unknown(node))
            }
            "conversion_operator_declaration" => {
                <ConversionOperatorDeclaration as ::treesitter_types::FromNode>::from_node(
                    node, src,
                )
                .map(Self::ConversionOperatorDeclaration)
                .unwrap_or(Self::Unknown(node))
            }
            "declaration_expression" => {
                <DeclarationExpression as ::treesitter_types::FromNode>::from_node(node, src)
                    .map(Self::DeclarationExpression)
                    .unwrap_or(Self::Unknown(node))
            }
            "declaration_list" => {
                <DeclarationList as ::treesitter_types::FromNode>::from_node(node, src)
                    .map(Self::DeclarationList)
                    .unwrap_or(Self::Unknown(node))
            }
            "declaration_pattern" => {
                <DeclarationPattern as ::treesitter_types::FromNode>::from_node(node, src)
                    .map(Self::DeclarationPattern)
                    .unwrap_or(Self::Unknown(node))
            }
            "default_expression" => {
                <DefaultExpression as ::treesitter_types::FromNode>::from_node(node, src)
                    .map(Self::DefaultExpression)
                    .unwrap_or(Self::Unknown(node))
            }
            "delegate_declaration" => {
                <DelegateDeclaration as ::treesitter_types::FromNode>::from_node(node, src)
                    .map(Self::DelegateDeclaration)
                    .unwrap_or(Self::Unknown(node))
            }
            "destructor_declaration" => {
                <DestructorDeclaration as ::treesitter_types::FromNode>::from_node(node, src)
                    .map(Self::DestructorDeclaration)
                    .unwrap_or(Self::Unknown(node))
            }
            "do_statement" => <DoStatement as ::treesitter_types::FromNode>::from_node(node, src)
                .map(Self::DoStatement)
                .unwrap_or(Self::Unknown(node)),
            "element_access_expression" => {
                <ElementAccessExpression as ::treesitter_types::FromNode>::from_node(node, src)
                    .map(Self::ElementAccessExpression)
                    .unwrap_or(Self::Unknown(node))
            }
            "element_binding_expression" => {
                <ElementBindingExpression as ::treesitter_types::FromNode>::from_node(node, src)
                    .map(Self::ElementBindingExpression)
                    .unwrap_or(Self::Unknown(node))
            }
            "empty_statement" => {
                <EmptyStatement as ::treesitter_types::FromNode>::from_node(node, src)
                    .map(Self::EmptyStatement)
                    .unwrap_or(Self::Unknown(node))
            }
            "enum_declaration" => {
                <EnumDeclaration as ::treesitter_types::FromNode>::from_node(node, src)
                    .map(Self::EnumDeclaration)
                    .unwrap_or(Self::Unknown(node))
            }
            "enum_member_declaration" => {
                <EnumMemberDeclaration as ::treesitter_types::FromNode>::from_node(node, src)
                    .map(Self::EnumMemberDeclaration)
                    .unwrap_or(Self::Unknown(node))
            }
            "enum_member_declaration_list" => {
                <EnumMemberDeclarationList as ::treesitter_types::FromNode>::from_node(node, src)
                    .map(Self::EnumMemberDeclarationList)
                    .unwrap_or(Self::Unknown(node))
            }
            "event_declaration" => {
                <EventDeclaration as ::treesitter_types::FromNode>::from_node(node, src)
                    .map(Self::EventDeclaration)
                    .unwrap_or(Self::Unknown(node))
            }
            "event_field_declaration" => {
                <EventFieldDeclaration as ::treesitter_types::FromNode>::from_node(node, src)
                    .map(Self::EventFieldDeclaration)
                    .unwrap_or(Self::Unknown(node))
            }
            "explicit_interface_specifier" => {
                <ExplicitInterfaceSpecifier as ::treesitter_types::FromNode>::from_node(node, src)
                    .map(Self::ExplicitInterfaceSpecifier)
                    .unwrap_or(Self::Unknown(node))
            }
            "expression_statement" => {
                <ExpressionStatement as ::treesitter_types::FromNode>::from_node(node, src)
                    .map(Self::ExpressionStatement)
                    .unwrap_or(Self::Unknown(node))
            }
            "extern_alias_directive" => {
                <ExternAliasDirective as ::treesitter_types::FromNode>::from_node(node, src)
                    .map(Self::ExternAliasDirective)
                    .unwrap_or(Self::Unknown(node))
            }
            "field_declaration" => {
                <FieldDeclaration as ::treesitter_types::FromNode>::from_node(node, src)
                    .map(Self::FieldDeclaration)
                    .unwrap_or(Self::Unknown(node))
            }
            "file_scoped_namespace_declaration" => {
                <FileScopedNamespaceDeclaration as ::treesitter_types::FromNode>::from_node(
                    node, src,
                )
                .map(Self::FileScopedNamespaceDeclaration)
                .unwrap_or(Self::Unknown(node))
            }
            "finally_clause" => {
                <FinallyClause as ::treesitter_types::FromNode>::from_node(node, src)
                    .map(Self::FinallyClause)
                    .unwrap_or(Self::Unknown(node))
            }
            "fixed_statement" => {
                <FixedStatement as ::treesitter_types::FromNode>::from_node(node, src)
                    .map(Self::FixedStatement)
                    .unwrap_or(Self::Unknown(node))
            }
            "for_statement" => <ForStatement as ::treesitter_types::FromNode>::from_node(node, src)
                .map(Self::ForStatement)
                .unwrap_or(Self::Unknown(node)),
            "foreach_statement" => {
                <ForeachStatement as ::treesitter_types::FromNode>::from_node(node, src)
                    .map(Self::ForeachStatement)
                    .unwrap_or(Self::Unknown(node))
            }
            "from_clause" => <FromClause as ::treesitter_types::FromNode>::from_node(node, src)
                .map(Self::FromClause)
                .unwrap_or(Self::Unknown(node)),
            "function_pointer_parameter" => {
                <FunctionPointerParameter as ::treesitter_types::FromNode>::from_node(node, src)
                    .map(Self::FunctionPointerParameter)
                    .unwrap_or(Self::Unknown(node))
            }
            "function_pointer_type" => {
                <FunctionPointerType as ::treesitter_types::FromNode>::from_node(node, src)
                    .map(Self::FunctionPointerType)
                    .unwrap_or(Self::Unknown(node))
            }
            "generic_name" => <GenericName as ::treesitter_types::FromNode>::from_node(node, src)
                .map(Self::GenericName)
                .unwrap_or(Self::Unknown(node)),
            "global_attribute" => {
                <GlobalAttribute as ::treesitter_types::FromNode>::from_node(node, src)
                    .map(Self::GlobalAttribute)
                    .unwrap_or(Self::Unknown(node))
            }
            "global_statement" => {
                <GlobalStatement as ::treesitter_types::FromNode>::from_node(node, src)
                    .map(Self::GlobalStatement)
                    .unwrap_or(Self::Unknown(node))
            }
            "goto_statement" => {
                <GotoStatement as ::treesitter_types::FromNode>::from_node(node, src)
                    .map(Self::GotoStatement)
                    .unwrap_or(Self::Unknown(node))
            }
            "group_clause" => <GroupClause as ::treesitter_types::FromNode>::from_node(node, src)
                .map(Self::GroupClause)
                .unwrap_or(Self::Unknown(node)),
            "identifier" => <Identifier as ::treesitter_types::FromNode>::from_node(node, src)
                .map(Self::Identifier)
                .unwrap_or(Self::Unknown(node)),
            "if_statement" => <IfStatement as ::treesitter_types::FromNode>::from_node(node, src)
                .map(Self::IfStatement)
                .unwrap_or(Self::Unknown(node)),
            "implicit_array_creation_expression" => {
                <ImplicitArrayCreationExpression as ::treesitter_types::FromNode>::from_node(
                    node, src,
                )
                .map(Self::ImplicitArrayCreationExpression)
                .unwrap_or(Self::Unknown(node))
            }
            "implicit_object_creation_expression" => {
                <ImplicitObjectCreationExpression as ::treesitter_types::FromNode>::from_node(
                    node, src,
                )
                .map(Self::ImplicitObjectCreationExpression)
                .unwrap_or(Self::Unknown(node))
            }
            "implicit_parameter" => {
                <ImplicitParameter as ::treesitter_types::FromNode>::from_node(node, src)
                    .map(Self::ImplicitParameter)
                    .unwrap_or(Self::Unknown(node))
            }
            "implicit_stackalloc_expression" => {
                <ImplicitStackallocExpression as ::treesitter_types::FromNode>::from_node(node, src)
                    .map(Self::ImplicitStackallocExpression)
                    .unwrap_or(Self::Unknown(node))
            }
            "implicit_type" => <ImplicitType as ::treesitter_types::FromNode>::from_node(node, src)
                .map(Self::ImplicitType)
                .unwrap_or(Self::Unknown(node)),
            "indexer_declaration" => {
                <IndexerDeclaration as ::treesitter_types::FromNode>::from_node(node, src)
                    .map(Self::IndexerDeclaration)
                    .unwrap_or(Self::Unknown(node))
            }
            "initializer_expression" => {
                <InitializerExpression as ::treesitter_types::FromNode>::from_node(node, src)
                    .map(Self::InitializerExpression)
                    .unwrap_or(Self::Unknown(node))
            }
            "interface_declaration" => {
                <InterfaceDeclaration as ::treesitter_types::FromNode>::from_node(node, src)
                    .map(Self::InterfaceDeclaration)
                    .unwrap_or(Self::Unknown(node))
            }
            "interpolated_string_expression" => {
                <InterpolatedStringExpression as ::treesitter_types::FromNode>::from_node(node, src)
                    .map(Self::InterpolatedStringExpression)
                    .unwrap_or(Self::Unknown(node))
            }
            "interpolation" => {
                <Interpolation as ::treesitter_types::FromNode>::from_node(node, src)
                    .map(Self::Interpolation)
                    .unwrap_or(Self::Unknown(node))
            }
            "interpolation_alignment_clause" => {
                <InterpolationAlignmentClause as ::treesitter_types::FromNode>::from_node(node, src)
                    .map(Self::InterpolationAlignmentClause)
                    .unwrap_or(Self::Unknown(node))
            }
            "interpolation_format_clause" => {
                <InterpolationFormatClause as ::treesitter_types::FromNode>::from_node(node, src)
                    .map(Self::InterpolationFormatClause)
                    .unwrap_or(Self::Unknown(node))
            }
            "invocation_expression" => {
                <InvocationExpression as ::treesitter_types::FromNode>::from_node(node, src)
                    .map(Self::InvocationExpression)
                    .unwrap_or(Self::Unknown(node))
            }
            "is_expression" => <IsExpression as ::treesitter_types::FromNode>::from_node(node, src)
                .map(Self::IsExpression)
                .unwrap_or(Self::Unknown(node)),
            "is_pattern_expression" => {
                <IsPatternExpression as ::treesitter_types::FromNode>::from_node(node, src)
                    .map(Self::IsPatternExpression)
                    .unwrap_or(Self::Unknown(node))
            }
            "join_clause" => <JoinClause as ::treesitter_types::FromNode>::from_node(node, src)
                .map(Self::JoinClause)
                .unwrap_or(Self::Unknown(node)),
            "join_into_clause" => {
                <JoinIntoClause as ::treesitter_types::FromNode>::from_node(node, src)
                    .map(Self::JoinIntoClause)
                    .unwrap_or(Self::Unknown(node))
            }
            "labeled_statement" => {
                <LabeledStatement as ::treesitter_types::FromNode>::from_node(node, src)
                    .map(Self::LabeledStatement)
                    .unwrap_or(Self::Unknown(node))
            }
            "lambda_expression" => {
                <LambdaExpression as ::treesitter_types::FromNode>::from_node(node, src)
                    .map(Self::LambdaExpression)
                    .unwrap_or(Self::Unknown(node))
            }
            "let_clause" => <LetClause as ::treesitter_types::FromNode>::from_node(node, src)
                .map(Self::LetClause)
                .unwrap_or(Self::Unknown(node)),
            "list_pattern" => <ListPattern as ::treesitter_types::FromNode>::from_node(node, src)
                .map(Self::ListPattern)
                .unwrap_or(Self::Unknown(node)),
            "local_declaration_statement" => {
                <LocalDeclarationStatement as ::treesitter_types::FromNode>::from_node(node, src)
                    .map(Self::LocalDeclarationStatement)
                    .unwrap_or(Self::Unknown(node))
            }
            "local_function_statement" => {
                <LocalFunctionStatement as ::treesitter_types::FromNode>::from_node(node, src)
                    .map(Self::LocalFunctionStatement)
                    .unwrap_or(Self::Unknown(node))
            }
            "lock_statement" => {
                <LockStatement as ::treesitter_types::FromNode>::from_node(node, src)
                    .map(Self::LockStatement)
                    .unwrap_or(Self::Unknown(node))
            }
            "makeref_expression" => {
                <MakerefExpression as ::treesitter_types::FromNode>::from_node(node, src)
                    .map(Self::MakerefExpression)
                    .unwrap_or(Self::Unknown(node))
            }
            "member_access_expression" => {
                <MemberAccessExpression as ::treesitter_types::FromNode>::from_node(node, src)
                    .map(Self::MemberAccessExpression)
                    .unwrap_or(Self::Unknown(node))
            }
            "member_binding_expression" => {
                <MemberBindingExpression as ::treesitter_types::FromNode>::from_node(node, src)
                    .map(Self::MemberBindingExpression)
                    .unwrap_or(Self::Unknown(node))
            }
            "method_declaration" => {
                <MethodDeclaration as ::treesitter_types::FromNode>::from_node(node, src)
                    .map(Self::MethodDeclaration)
                    .unwrap_or(Self::Unknown(node))
            }
            "modifier" => <Modifier as ::treesitter_types::FromNode>::from_node(node, src)
                .map(Self::Modifier)
                .unwrap_or(Self::Unknown(node)),
            "namespace_declaration" => {
                <NamespaceDeclaration as ::treesitter_types::FromNode>::from_node(node, src)
                    .map(Self::NamespaceDeclaration)
                    .unwrap_or(Self::Unknown(node))
            }
            "negated_pattern" => {
                <NegatedPattern as ::treesitter_types::FromNode>::from_node(node, src)
                    .map(Self::NegatedPattern)
                    .unwrap_or(Self::Unknown(node))
            }
            "nullable_type" => <NullableType as ::treesitter_types::FromNode>::from_node(node, src)
                .map(Self::NullableType)
                .unwrap_or(Self::Unknown(node)),
            "object_creation_expression" => {
                <ObjectCreationExpression as ::treesitter_types::FromNode>::from_node(node, src)
                    .map(Self::ObjectCreationExpression)
                    .unwrap_or(Self::Unknown(node))
            }
            "operator_declaration" => {
                <OperatorDeclaration as ::treesitter_types::FromNode>::from_node(node, src)
                    .map(Self::OperatorDeclaration)
                    .unwrap_or(Self::Unknown(node))
            }
            "or_pattern" => <OrPattern as ::treesitter_types::FromNode>::from_node(node, src)
                .map(Self::OrPattern)
                .unwrap_or(Self::Unknown(node)),
            "order_by_clause" => {
                <OrderByClause as ::treesitter_types::FromNode>::from_node(node, src)
                    .map(Self::OrderByClause)
                    .unwrap_or(Self::Unknown(node))
            }
            "parameter" => <Parameter as ::treesitter_types::FromNode>::from_node(node, src)
                .map(Self::Parameter)
                .unwrap_or(Self::Unknown(node)),
            "parameter_list" => {
                <ParameterList as ::treesitter_types::FromNode>::from_node(node, src)
                    .map(Self::ParameterList)
                    .unwrap_or(Self::Unknown(node))
            }
            "parenthesized_expression" => {
                <ParenthesizedExpression as ::treesitter_types::FromNode>::from_node(node, src)
                    .map(Self::ParenthesizedExpression)
                    .unwrap_or(Self::Unknown(node))
            }
            "parenthesized_pattern" => {
                <ParenthesizedPattern as ::treesitter_types::FromNode>::from_node(node, src)
                    .map(Self::ParenthesizedPattern)
                    .unwrap_or(Self::Unknown(node))
            }
            "parenthesized_variable_designation" => {
                <ParenthesizedVariableDesignation as ::treesitter_types::FromNode>::from_node(
                    node, src,
                )
                .map(Self::ParenthesizedVariableDesignation)
                .unwrap_or(Self::Unknown(node))
            }
            "pointer_type" => <PointerType as ::treesitter_types::FromNode>::from_node(node, src)
                .map(Self::PointerType)
                .unwrap_or(Self::Unknown(node)),
            "positional_pattern_clause" => {
                <PositionalPatternClause as ::treesitter_types::FromNode>::from_node(node, src)
                    .map(Self::PositionalPatternClause)
                    .unwrap_or(Self::Unknown(node))
            }
            "postfix_unary_expression" => {
                <PostfixUnaryExpression as ::treesitter_types::FromNode>::from_node(node, src)
                    .map(Self::PostfixUnaryExpression)
                    .unwrap_or(Self::Unknown(node))
            }
            "prefix_unary_expression" => {
                <PrefixUnaryExpression as ::treesitter_types::FromNode>::from_node(node, src)
                    .map(Self::PrefixUnaryExpression)
                    .unwrap_or(Self::Unknown(node))
            }
            "preproc_define" => {
                <PreprocDefine as ::treesitter_types::FromNode>::from_node(node, src)
                    .map(Self::PreprocDefine)
                    .unwrap_or(Self::Unknown(node))
            }
            "preproc_elif" => <PreprocElif as ::treesitter_types::FromNode>::from_node(node, src)
                .map(Self::PreprocElif)
                .unwrap_or(Self::Unknown(node)),
            "preproc_else" => <PreprocElse as ::treesitter_types::FromNode>::from_node(node, src)
                .map(Self::PreprocElse)
                .unwrap_or(Self::Unknown(node)),
            "preproc_endregion" => {
                <PreprocEndregion as ::treesitter_types::FromNode>::from_node(node, src)
                    .map(Self::PreprocEndregion)
                    .unwrap_or(Self::Unknown(node))
            }
            "preproc_error" => <PreprocError as ::treesitter_types::FromNode>::from_node(node, src)
                .map(Self::PreprocError)
                .unwrap_or(Self::Unknown(node)),
            "preproc_if" => <PreprocIf as ::treesitter_types::FromNode>::from_node(node, src)
                .map(Self::PreprocIf)
                .unwrap_or(Self::Unknown(node)),
            "preproc_if_in_attribute_list" => {
                <PreprocIfInAttributeList as ::treesitter_types::FromNode>::from_node(node, src)
                    .map(Self::PreprocIfInAttributeList)
                    .unwrap_or(Self::Unknown(node))
            }
            "preproc_line" => <PreprocLine as ::treesitter_types::FromNode>::from_node(node, src)
                .map(Self::PreprocLine)
                .unwrap_or(Self::Unknown(node)),
            "preproc_nullable" => {
                <PreprocNullable as ::treesitter_types::FromNode>::from_node(node, src)
                    .map(Self::PreprocNullable)
                    .unwrap_or(Self::Unknown(node))
            }
            "preproc_pragma" => {
                <PreprocPragma as ::treesitter_types::FromNode>::from_node(node, src)
                    .map(Self::PreprocPragma)
                    .unwrap_or(Self::Unknown(node))
            }
            "preproc_region" => {
                <PreprocRegion as ::treesitter_types::FromNode>::from_node(node, src)
                    .map(Self::PreprocRegion)
                    .unwrap_or(Self::Unknown(node))
            }
            "preproc_undef" => <PreprocUndef as ::treesitter_types::FromNode>::from_node(node, src)
                .map(Self::PreprocUndef)
                .unwrap_or(Self::Unknown(node)),
            "preproc_warning" => {
                <PreprocWarning as ::treesitter_types::FromNode>::from_node(node, src)
                    .map(Self::PreprocWarning)
                    .unwrap_or(Self::Unknown(node))
            }
            "primary_constructor_base_type" => {
                <PrimaryConstructorBaseType as ::treesitter_types::FromNode>::from_node(node, src)
                    .map(Self::PrimaryConstructorBaseType)
                    .unwrap_or(Self::Unknown(node))
            }
            "property_declaration" => {
                <PropertyDeclaration as ::treesitter_types::FromNode>::from_node(node, src)
                    .map(Self::PropertyDeclaration)
                    .unwrap_or(Self::Unknown(node))
            }
            "property_pattern_clause" => {
                <PropertyPatternClause as ::treesitter_types::FromNode>::from_node(node, src)
                    .map(Self::PropertyPatternClause)
                    .unwrap_or(Self::Unknown(node))
            }
            "qualified_name" => {
                <QualifiedName as ::treesitter_types::FromNode>::from_node(node, src)
                    .map(Self::QualifiedName)
                    .unwrap_or(Self::Unknown(node))
            }
            "query_expression" => {
                <QueryExpression as ::treesitter_types::FromNode>::from_node(node, src)
                    .map(Self::QueryExpression)
                    .unwrap_or(Self::Unknown(node))
            }
            "range_expression" => {
                <RangeExpression as ::treesitter_types::FromNode>::from_node(node, src)
                    .map(Self::RangeExpression)
                    .unwrap_or(Self::Unknown(node))
            }
            "raw_string_literal" => {
                <RawStringLiteral as ::treesitter_types::FromNode>::from_node(node, src)
                    .map(Self::RawStringLiteral)
                    .unwrap_or(Self::Unknown(node))
            }
            "record_declaration" => {
                <RecordDeclaration as ::treesitter_types::FromNode>::from_node(node, src)
                    .map(Self::RecordDeclaration)
                    .unwrap_or(Self::Unknown(node))
            }
            "recursive_pattern" => {
                <RecursivePattern as ::treesitter_types::FromNode>::from_node(node, src)
                    .map(Self::RecursivePattern)
                    .unwrap_or(Self::Unknown(node))
            }
            "ref_expression" => {
                <RefExpression as ::treesitter_types::FromNode>::from_node(node, src)
                    .map(Self::RefExpression)
                    .unwrap_or(Self::Unknown(node))
            }
            "ref_type" => <RefType as ::treesitter_types::FromNode>::from_node(node, src)
                .map(Self::RefType)
                .unwrap_or(Self::Unknown(node)),
            "reftype_expression" => {
                <ReftypeExpression as ::treesitter_types::FromNode>::from_node(node, src)
                    .map(Self::ReftypeExpression)
                    .unwrap_or(Self::Unknown(node))
            }
            "refvalue_expression" => {
                <RefvalueExpression as ::treesitter_types::FromNode>::from_node(node, src)
                    .map(Self::RefvalueExpression)
                    .unwrap_or(Self::Unknown(node))
            }
            "relational_pattern" => {
                <RelationalPattern as ::treesitter_types::FromNode>::from_node(node, src)
                    .map(Self::RelationalPattern)
                    .unwrap_or(Self::Unknown(node))
            }
            "return_statement" => {
                <ReturnStatement as ::treesitter_types::FromNode>::from_node(node, src)
                    .map(Self::ReturnStatement)
                    .unwrap_or(Self::Unknown(node))
            }
            "scoped_type" => <ScopedType as ::treesitter_types::FromNode>::from_node(node, src)
                .map(Self::ScopedType)
                .unwrap_or(Self::Unknown(node)),
            "select_clause" => <SelectClause as ::treesitter_types::FromNode>::from_node(node, src)
                .map(Self::SelectClause)
                .unwrap_or(Self::Unknown(node)),
            "sizeof_expression" => {
                <SizeofExpression as ::treesitter_types::FromNode>::from_node(node, src)
                    .map(Self::SizeofExpression)
                    .unwrap_or(Self::Unknown(node))
            }
            "stackalloc_expression" => {
                <StackallocExpression as ::treesitter_types::FromNode>::from_node(node, src)
                    .map(Self::StackallocExpression)
                    .unwrap_or(Self::Unknown(node))
            }
            "string_literal" => {
                <StringLiteral as ::treesitter_types::FromNode>::from_node(node, src)
                    .map(Self::StringLiteral)
                    .unwrap_or(Self::Unknown(node))
            }
            "struct_declaration" => {
                <StructDeclaration as ::treesitter_types::FromNode>::from_node(node, src)
                    .map(Self::StructDeclaration)
                    .unwrap_or(Self::Unknown(node))
            }
            "subpattern" => <Subpattern as ::treesitter_types::FromNode>::from_node(node, src)
                .map(Self::Subpattern)
                .unwrap_or(Self::Unknown(node)),
            "switch_body" => <SwitchBody as ::treesitter_types::FromNode>::from_node(node, src)
                .map(Self::SwitchBody)
                .unwrap_or(Self::Unknown(node)),
            "switch_expression" => {
                <SwitchExpression as ::treesitter_types::FromNode>::from_node(node, src)
                    .map(Self::SwitchExpression)
                    .unwrap_or(Self::Unknown(node))
            }
            "switch_expression_arm" => {
                <SwitchExpressionArm as ::treesitter_types::FromNode>::from_node(node, src)
                    .map(Self::SwitchExpressionArm)
                    .unwrap_or(Self::Unknown(node))
            }
            "switch_section" => {
                <SwitchSection as ::treesitter_types::FromNode>::from_node(node, src)
                    .map(Self::SwitchSection)
                    .unwrap_or(Self::Unknown(node))
            }
            "switch_statement" => {
                <SwitchStatement as ::treesitter_types::FromNode>::from_node(node, src)
                    .map(Self::SwitchStatement)
                    .unwrap_or(Self::Unknown(node))
            }
            "throw_expression" => {
                <ThrowExpression as ::treesitter_types::FromNode>::from_node(node, src)
                    .map(Self::ThrowExpression)
                    .unwrap_or(Self::Unknown(node))
            }
            "throw_statement" => {
                <ThrowStatement as ::treesitter_types::FromNode>::from_node(node, src)
                    .map(Self::ThrowStatement)
                    .unwrap_or(Self::Unknown(node))
            }
            "try_statement" => <TryStatement as ::treesitter_types::FromNode>::from_node(node, src)
                .map(Self::TryStatement)
                .unwrap_or(Self::Unknown(node)),
            "tuple_element" => <TupleElement as ::treesitter_types::FromNode>::from_node(node, src)
                .map(Self::TupleElement)
                .unwrap_or(Self::Unknown(node)),
            "tuple_expression" => {
                <TupleExpression as ::treesitter_types::FromNode>::from_node(node, src)
                    .map(Self::TupleExpression)
                    .unwrap_or(Self::Unknown(node))
            }
            "tuple_pattern" => <TuplePattern as ::treesitter_types::FromNode>::from_node(node, src)
                .map(Self::TuplePattern)
                .unwrap_or(Self::Unknown(node)),
            "tuple_type" => <TupleType as ::treesitter_types::FromNode>::from_node(node, src)
                .map(Self::TupleType)
                .unwrap_or(Self::Unknown(node)),
            "type_argument_list" => {
                <TypeArgumentList as ::treesitter_types::FromNode>::from_node(node, src)
                    .map(Self::TypeArgumentList)
                    .unwrap_or(Self::Unknown(node))
            }
            "type_parameter" => {
                <TypeParameter as ::treesitter_types::FromNode>::from_node(node, src)
                    .map(Self::TypeParameter)
                    .unwrap_or(Self::Unknown(node))
            }
            "type_parameter_constraint" => {
                <TypeParameterConstraint as ::treesitter_types::FromNode>::from_node(node, src)
                    .map(Self::TypeParameterConstraint)
                    .unwrap_or(Self::Unknown(node))
            }
            "type_parameter_constraints_clause" => {
                <TypeParameterConstraintsClause as ::treesitter_types::FromNode>::from_node(
                    node, src,
                )
                .map(Self::TypeParameterConstraintsClause)
                .unwrap_or(Self::Unknown(node))
            }
            "type_parameter_list" => {
                <TypeParameterList as ::treesitter_types::FromNode>::from_node(node, src)
                    .map(Self::TypeParameterList)
                    .unwrap_or(Self::Unknown(node))
            }
            "type_pattern" => <TypePattern as ::treesitter_types::FromNode>::from_node(node, src)
                .map(Self::TypePattern)
                .unwrap_or(Self::Unknown(node)),
            "typeof_expression" => {
                <TypeofExpression as ::treesitter_types::FromNode>::from_node(node, src)
                    .map(Self::TypeofExpression)
                    .unwrap_or(Self::Unknown(node))
            }
            "unary_expression" => {
                <UnaryExpression as ::treesitter_types::FromNode>::from_node(node, src)
                    .map(Self::UnaryExpression)
                    .unwrap_or(Self::Unknown(node))
            }
            "unsafe_statement" => {
                <UnsafeStatement as ::treesitter_types::FromNode>::from_node(node, src)
                    .map(Self::UnsafeStatement)
                    .unwrap_or(Self::Unknown(node))
            }
            "using_directive" => {
                <UsingDirective as ::treesitter_types::FromNode>::from_node(node, src)
                    .map(Self::UsingDirective)
                    .unwrap_or(Self::Unknown(node))
            }
            "using_statement" => {
                <UsingStatement as ::treesitter_types::FromNode>::from_node(node, src)
                    .map(Self::UsingStatement)
                    .unwrap_or(Self::Unknown(node))
            }
            "var_pattern" => <VarPattern as ::treesitter_types::FromNode>::from_node(node, src)
                .map(Self::VarPattern)
                .unwrap_or(Self::Unknown(node)),
            "variable_declaration" => {
                <VariableDeclaration as ::treesitter_types::FromNode>::from_node(node, src)
                    .map(Self::VariableDeclaration)
                    .unwrap_or(Self::Unknown(node))
            }
            "variable_declarator" => {
                <VariableDeclarator as ::treesitter_types::FromNode>::from_node(node, src)
                    .map(Self::VariableDeclarator)
                    .unwrap_or(Self::Unknown(node))
            }
            "when_clause" => <WhenClause as ::treesitter_types::FromNode>::from_node(node, src)
                .map(Self::WhenClause)
                .unwrap_or(Self::Unknown(node)),
            "where_clause" => <WhereClause as ::treesitter_types::FromNode>::from_node(node, src)
                .map(Self::WhereClause)
                .unwrap_or(Self::Unknown(node)),
            "while_statement" => {
                <WhileStatement as ::treesitter_types::FromNode>::from_node(node, src)
                    .map(Self::WhileStatement)
                    .unwrap_or(Self::Unknown(node))
            }
            "with_expression" => {
                <WithExpression as ::treesitter_types::FromNode>::from_node(node, src)
                    .map(Self::WithExpression)
                    .unwrap_or(Self::Unknown(node))
            }
            "with_initializer" => {
                <WithInitializer as ::treesitter_types::FromNode>::from_node(node, src)
                    .map(Self::WithInitializer)
                    .unwrap_or(Self::Unknown(node))
            }
            "yield_statement" => {
                <YieldStatement as ::treesitter_types::FromNode>::from_node(node, src)
                    .map(Self::YieldStatement)
                    .unwrap_or(Self::Unknown(node))
            }
            "character_literal_content" => {
                <CharacterLiteralContent as ::treesitter_types::FromNode>::from_node(node, src)
                    .map(Self::CharacterLiteralContent)
                    .unwrap_or(Self::Unknown(node))
            }
            "comment" => <Comment as ::treesitter_types::FromNode>::from_node(node, src)
                .map(Self::Comment)
                .unwrap_or(Self::Unknown(node)),
            "discard" => <Discard as ::treesitter_types::FromNode>::from_node(node, src)
                .map(Self::Discard)
                .unwrap_or(Self::Unknown(node)),
            "escape_sequence" => {
                <EscapeSequence as ::treesitter_types::FromNode>::from_node(node, src)
                    .map(Self::EscapeSequence)
                    .unwrap_or(Self::Unknown(node))
            }
            "integer_literal" => {
                <IntegerLiteral as ::treesitter_types::FromNode>::from_node(node, src)
                    .map(Self::IntegerLiteral)
                    .unwrap_or(Self::Unknown(node))
            }
            "interpolation_brace" => {
                <InterpolationBrace as ::treesitter_types::FromNode>::from_node(node, src)
                    .map(Self::InterpolationBrace)
                    .unwrap_or(Self::Unknown(node))
            }
            "interpolation_quote" => {
                <InterpolationQuote as ::treesitter_types::FromNode>::from_node(node, src)
                    .map(Self::InterpolationQuote)
                    .unwrap_or(Self::Unknown(node))
            }
            "interpolation_start" => {
                <InterpolationStart as ::treesitter_types::FromNode>::from_node(node, src)
                    .map(Self::InterpolationStart)
                    .unwrap_or(Self::Unknown(node))
            }
            "null_literal" => <NullLiteral as ::treesitter_types::FromNode>::from_node(node, src)
                .map(Self::NullLiteral)
                .unwrap_or(Self::Unknown(node)),
            "predefined_type" => {
                <PredefinedType as ::treesitter_types::FromNode>::from_node(node, src)
                    .map(Self::PredefinedType)
                    .unwrap_or(Self::Unknown(node))
            }
            "preproc_arg" => <PreprocArg as ::treesitter_types::FromNode>::from_node(node, src)
                .map(Self::PreprocArg)
                .unwrap_or(Self::Unknown(node)),
            "raw_string_content" => {
                <RawStringContent as ::treesitter_types::FromNode>::from_node(node, src)
                    .map(Self::RawStringContent)
                    .unwrap_or(Self::Unknown(node))
            }
            "raw_string_end" => {
                <RawStringEnd as ::treesitter_types::FromNode>::from_node(node, src)
                    .map(Self::RawStringEnd)
                    .unwrap_or(Self::Unknown(node))
            }
            "raw_string_start" => {
                <RawStringStart as ::treesitter_types::FromNode>::from_node(node, src)
                    .map(Self::RawStringStart)
                    .unwrap_or(Self::Unknown(node))
            }
            "real_literal" => <RealLiteral as ::treesitter_types::FromNode>::from_node(node, src)
                .map(Self::RealLiteral)
                .unwrap_or(Self::Unknown(node)),
            "shebang_directive" => {
                <ShebangDirective as ::treesitter_types::FromNode>::from_node(node, src)
                    .map(Self::ShebangDirective)
                    .unwrap_or(Self::Unknown(node))
            }
            "string_content" => {
                <StringContent as ::treesitter_types::FromNode>::from_node(node, src)
                    .map(Self::StringContent)
                    .unwrap_or(Self::Unknown(node))
            }
            "string_literal_content" => {
                <StringLiteralContent as ::treesitter_types::FromNode>::from_node(node, src)
                    .map(Self::StringLiteralContent)
                    .unwrap_or(Self::Unknown(node))
            }
            "string_literal_encoding" => {
                <StringLiteralEncoding as ::treesitter_types::FromNode>::from_node(node, src)
                    .map(Self::StringLiteralEncoding)
                    .unwrap_or(Self::Unknown(node))
            }
            "verbatim_string_literal" => {
                <VerbatimStringLiteral as ::treesitter_types::FromNode>::from_node(node, src)
                    .map(Self::VerbatimStringLiteral)
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
            Self::Literal(inner) => inner.span(),
            Self::LvalueExpression(inner) => inner.span(),
            Self::NonLvalueExpression(inner) => inner.span(),
            Self::Pattern(inner) => inner.span(),
            Self::Statement(inner) => inner.span(),
            Self::Type(inner) => inner.span(),
            Self::TypeDeclaration(inner) => inner.span(),
            Self::AccessorDeclaration(inner) => inner.span(),
            Self::AccessorList(inner) => inner.span(),
            Self::AliasQualifiedName(inner) => inner.span(),
            Self::AndPattern(inner) => inner.span(),
            Self::AnonymousMethodExpression(inner) => inner.span(),
            Self::AnonymousObjectCreationExpression(inner) => inner.span(),
            Self::Argument(inner) => inner.span(),
            Self::ArgumentList(inner) => inner.span(),
            Self::ArrayCreationExpression(inner) => inner.span(),
            Self::ArrayRankSpecifier(inner) => inner.span(),
            Self::ArrayType(inner) => inner.span(),
            Self::ArrowExpressionClause(inner) => inner.span(),
            Self::AsExpression(inner) => inner.span(),
            Self::AssignmentExpression(inner) => inner.span(),
            Self::Attribute(inner) => inner.span(),
            Self::AttributeArgument(inner) => inner.span(),
            Self::AttributeArgumentList(inner) => inner.span(),
            Self::AttributeList(inner) => inner.span(),
            Self::AttributeTargetSpecifier(inner) => inner.span(),
            Self::AwaitExpression(inner) => inner.span(),
            Self::BaseList(inner) => inner.span(),
            Self::BinaryExpression(inner) => inner.span(),
            Self::Block(inner) => inner.span(),
            Self::BooleanLiteral(inner) => inner.span(),
            Self::BracketedArgumentList(inner) => inner.span(),
            Self::BracketedParameterList(inner) => inner.span(),
            Self::BreakStatement(inner) => inner.span(),
            Self::CallingConvention(inner) => inner.span(),
            Self::CastExpression(inner) => inner.span(),
            Self::CatchClause(inner) => inner.span(),
            Self::CatchDeclaration(inner) => inner.span(),
            Self::CatchFilterClause(inner) => inner.span(),
            Self::CharacterLiteral(inner) => inner.span(),
            Self::CheckedExpression(inner) => inner.span(),
            Self::CheckedStatement(inner) => inner.span(),
            Self::ClassDeclaration(inner) => inner.span(),
            Self::CompilationUnit(inner) => inner.span(),
            Self::ConditionalAccessExpression(inner) => inner.span(),
            Self::ConditionalExpression(inner) => inner.span(),
            Self::ConstantPattern(inner) => inner.span(),
            Self::ConstructorConstraint(inner) => inner.span(),
            Self::ConstructorDeclaration(inner) => inner.span(),
            Self::ConstructorInitializer(inner) => inner.span(),
            Self::ContinueStatement(inner) => inner.span(),
            Self::ConversionOperatorDeclaration(inner) => inner.span(),
            Self::DeclarationExpression(inner) => inner.span(),
            Self::DeclarationList(inner) => inner.span(),
            Self::DeclarationPattern(inner) => inner.span(),
            Self::DefaultExpression(inner) => inner.span(),
            Self::DelegateDeclaration(inner) => inner.span(),
            Self::DestructorDeclaration(inner) => inner.span(),
            Self::DoStatement(inner) => inner.span(),
            Self::ElementAccessExpression(inner) => inner.span(),
            Self::ElementBindingExpression(inner) => inner.span(),
            Self::EmptyStatement(inner) => inner.span(),
            Self::EnumDeclaration(inner) => inner.span(),
            Self::EnumMemberDeclaration(inner) => inner.span(),
            Self::EnumMemberDeclarationList(inner) => inner.span(),
            Self::EventDeclaration(inner) => inner.span(),
            Self::EventFieldDeclaration(inner) => inner.span(),
            Self::ExplicitInterfaceSpecifier(inner) => inner.span(),
            Self::ExpressionStatement(inner) => inner.span(),
            Self::ExternAliasDirective(inner) => inner.span(),
            Self::FieldDeclaration(inner) => inner.span(),
            Self::FileScopedNamespaceDeclaration(inner) => inner.span(),
            Self::FinallyClause(inner) => inner.span(),
            Self::FixedStatement(inner) => inner.span(),
            Self::ForStatement(inner) => inner.span(),
            Self::ForeachStatement(inner) => inner.span(),
            Self::FromClause(inner) => inner.span(),
            Self::FunctionPointerParameter(inner) => inner.span(),
            Self::FunctionPointerType(inner) => inner.span(),
            Self::GenericName(inner) => inner.span(),
            Self::GlobalAttribute(inner) => inner.span(),
            Self::GlobalStatement(inner) => inner.span(),
            Self::GotoStatement(inner) => inner.span(),
            Self::GroupClause(inner) => inner.span(),
            Self::Identifier(inner) => inner.span(),
            Self::IfStatement(inner) => inner.span(),
            Self::ImplicitArrayCreationExpression(inner) => inner.span(),
            Self::ImplicitObjectCreationExpression(inner) => inner.span(),
            Self::ImplicitParameter(inner) => inner.span(),
            Self::ImplicitStackallocExpression(inner) => inner.span(),
            Self::ImplicitType(inner) => inner.span(),
            Self::IndexerDeclaration(inner) => inner.span(),
            Self::InitializerExpression(inner) => inner.span(),
            Self::InterfaceDeclaration(inner) => inner.span(),
            Self::InterpolatedStringExpression(inner) => inner.span(),
            Self::Interpolation(inner) => inner.span(),
            Self::InterpolationAlignmentClause(inner) => inner.span(),
            Self::InterpolationFormatClause(inner) => inner.span(),
            Self::InvocationExpression(inner) => inner.span(),
            Self::IsExpression(inner) => inner.span(),
            Self::IsPatternExpression(inner) => inner.span(),
            Self::JoinClause(inner) => inner.span(),
            Self::JoinIntoClause(inner) => inner.span(),
            Self::LabeledStatement(inner) => inner.span(),
            Self::LambdaExpression(inner) => inner.span(),
            Self::LetClause(inner) => inner.span(),
            Self::ListPattern(inner) => inner.span(),
            Self::LocalDeclarationStatement(inner) => inner.span(),
            Self::LocalFunctionStatement(inner) => inner.span(),
            Self::LockStatement(inner) => inner.span(),
            Self::MakerefExpression(inner) => inner.span(),
            Self::MemberAccessExpression(inner) => inner.span(),
            Self::MemberBindingExpression(inner) => inner.span(),
            Self::MethodDeclaration(inner) => inner.span(),
            Self::Modifier(inner) => inner.span(),
            Self::NamespaceDeclaration(inner) => inner.span(),
            Self::NegatedPattern(inner) => inner.span(),
            Self::NullableType(inner) => inner.span(),
            Self::ObjectCreationExpression(inner) => inner.span(),
            Self::OperatorDeclaration(inner) => inner.span(),
            Self::OrPattern(inner) => inner.span(),
            Self::OrderByClause(inner) => inner.span(),
            Self::Parameter(inner) => inner.span(),
            Self::ParameterList(inner) => inner.span(),
            Self::ParenthesizedExpression(inner) => inner.span(),
            Self::ParenthesizedPattern(inner) => inner.span(),
            Self::ParenthesizedVariableDesignation(inner) => inner.span(),
            Self::PointerType(inner) => inner.span(),
            Self::PositionalPatternClause(inner) => inner.span(),
            Self::PostfixUnaryExpression(inner) => inner.span(),
            Self::PrefixUnaryExpression(inner) => inner.span(),
            Self::PreprocDefine(inner) => inner.span(),
            Self::PreprocElif(inner) => inner.span(),
            Self::PreprocElse(inner) => inner.span(),
            Self::PreprocEndregion(inner) => inner.span(),
            Self::PreprocError(inner) => inner.span(),
            Self::PreprocIf(inner) => inner.span(),
            Self::PreprocIfInAttributeList(inner) => inner.span(),
            Self::PreprocLine(inner) => inner.span(),
            Self::PreprocNullable(inner) => inner.span(),
            Self::PreprocPragma(inner) => inner.span(),
            Self::PreprocRegion(inner) => inner.span(),
            Self::PreprocUndef(inner) => inner.span(),
            Self::PreprocWarning(inner) => inner.span(),
            Self::PrimaryConstructorBaseType(inner) => inner.span(),
            Self::PropertyDeclaration(inner) => inner.span(),
            Self::PropertyPatternClause(inner) => inner.span(),
            Self::QualifiedName(inner) => inner.span(),
            Self::QueryExpression(inner) => inner.span(),
            Self::RangeExpression(inner) => inner.span(),
            Self::RawStringLiteral(inner) => inner.span(),
            Self::RecordDeclaration(inner) => inner.span(),
            Self::RecursivePattern(inner) => inner.span(),
            Self::RefExpression(inner) => inner.span(),
            Self::RefType(inner) => inner.span(),
            Self::ReftypeExpression(inner) => inner.span(),
            Self::RefvalueExpression(inner) => inner.span(),
            Self::RelationalPattern(inner) => inner.span(),
            Self::ReturnStatement(inner) => inner.span(),
            Self::ScopedType(inner) => inner.span(),
            Self::SelectClause(inner) => inner.span(),
            Self::SizeofExpression(inner) => inner.span(),
            Self::StackallocExpression(inner) => inner.span(),
            Self::StringLiteral(inner) => inner.span(),
            Self::StructDeclaration(inner) => inner.span(),
            Self::Subpattern(inner) => inner.span(),
            Self::SwitchBody(inner) => inner.span(),
            Self::SwitchExpression(inner) => inner.span(),
            Self::SwitchExpressionArm(inner) => inner.span(),
            Self::SwitchSection(inner) => inner.span(),
            Self::SwitchStatement(inner) => inner.span(),
            Self::ThrowExpression(inner) => inner.span(),
            Self::ThrowStatement(inner) => inner.span(),
            Self::TryStatement(inner) => inner.span(),
            Self::TupleElement(inner) => inner.span(),
            Self::TupleExpression(inner) => inner.span(),
            Self::TuplePattern(inner) => inner.span(),
            Self::TupleType(inner) => inner.span(),
            Self::TypeArgumentList(inner) => inner.span(),
            Self::TypeParameter(inner) => inner.span(),
            Self::TypeParameterConstraint(inner) => inner.span(),
            Self::TypeParameterConstraintsClause(inner) => inner.span(),
            Self::TypeParameterList(inner) => inner.span(),
            Self::TypePattern(inner) => inner.span(),
            Self::TypeofExpression(inner) => inner.span(),
            Self::UnaryExpression(inner) => inner.span(),
            Self::UnsafeStatement(inner) => inner.span(),
            Self::UsingDirective(inner) => inner.span(),
            Self::UsingStatement(inner) => inner.span(),
            Self::VarPattern(inner) => inner.span(),
            Self::VariableDeclaration(inner) => inner.span(),
            Self::VariableDeclarator(inner) => inner.span(),
            Self::WhenClause(inner) => inner.span(),
            Self::WhereClause(inner) => inner.span(),
            Self::WhileStatement(inner) => inner.span(),
            Self::WithExpression(inner) => inner.span(),
            Self::WithInitializer(inner) => inner.span(),
            Self::YieldStatement(inner) => inner.span(),
            Self::CharacterLiteralContent(inner) => inner.span(),
            Self::Comment(inner) => inner.span(),
            Self::Discard(inner) => inner.span(),
            Self::EscapeSequence(inner) => inner.span(),
            Self::IntegerLiteral(inner) => inner.span(),
            Self::InterpolationBrace(inner) => inner.span(),
            Self::InterpolationQuote(inner) => inner.span(),
            Self::InterpolationStart(inner) => inner.span(),
            Self::NullLiteral(inner) => inner.span(),
            Self::PredefinedType(inner) => inner.span(),
            Self::PreprocArg(inner) => inner.span(),
            Self::RawStringContent(inner) => inner.span(),
            Self::RawStringEnd(inner) => inner.span(),
            Self::RawStringStart(inner) => inner.span(),
            Self::RealLiteral(inner) => inner.span(),
            Self::ShebangDirective(inner) => inner.span(),
            Self::StringContent(inner) => inner.span(),
            Self::StringLiteralContent(inner) => inner.span(),
            Self::StringLiteralEncoding(inner) => inner.span(),
            Self::VerbatimStringLiteral(inner) => inner.span(),
            Self::Unknown(node) => ::treesitter_types::Span::from(*node),
        }
    }
}
