#[derive(Debug, Clone)]
pub enum Definition<'tree> {
    ClassDefinition(::std::boxed::Box<ClassDefinition<'tree>>),
    EnumDefinition(::std::boxed::Box<EnumDefinition<'tree>>),
    ExportDeclaration(::std::boxed::Box<ExportDeclaration<'tree>>),
    ExtensionDefinition(::std::boxed::Box<ExtensionDefinition<'tree>>),
    FunctionDeclaration(::std::boxed::Box<FunctionDeclaration<'tree>>),
    FunctionDefinition(::std::boxed::Box<FunctionDefinition<'tree>>),
    GivenDefinition(::std::boxed::Box<GivenDefinition<'tree>>),
    ImportDeclaration(::std::boxed::Box<ImportDeclaration<'tree>>),
    ObjectDefinition(::std::boxed::Box<ObjectDefinition<'tree>>),
    PackageClause(::std::boxed::Box<PackageClause<'tree>>),
    PackageObject(::std::boxed::Box<PackageObject<'tree>>),
    TraitDefinition(::std::boxed::Box<TraitDefinition<'tree>>),
    TypeDefinition(::std::boxed::Box<TypeDefinition<'tree>>),
    ValDeclaration(::std::boxed::Box<ValDeclaration<'tree>>),
    ValDefinition(::std::boxed::Box<ValDefinition<'tree>>),
    VarDeclaration(::std::boxed::Box<VarDeclaration<'tree>>),
    VarDefinition(::std::boxed::Box<VarDefinition<'tree>>),
}
impl<'tree> ::treesitter_types::FromNode<'tree> for Definition<'tree> {
    #[allow(clippy::collapsible_else_if)]
    fn from_node(
        node: ::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        match node.kind() {
            "class_definition" => Ok(Self::ClassDefinition(::std::boxed::Box::new(
                <ClassDefinition as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "enum_definition" => Ok(Self::EnumDefinition(::std::boxed::Box::new(
                <EnumDefinition as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "export_declaration" => Ok(Self::ExportDeclaration(::std::boxed::Box::new(
                <ExportDeclaration as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "extension_definition" => Ok(Self::ExtensionDefinition(::std::boxed::Box::new(
                <ExtensionDefinition as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "function_declaration" => Ok(Self::FunctionDeclaration(::std::boxed::Box::new(
                <FunctionDeclaration as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "function_definition" => Ok(Self::FunctionDefinition(::std::boxed::Box::new(
                <FunctionDefinition as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "given_definition" => Ok(Self::GivenDefinition(::std::boxed::Box::new(
                <GivenDefinition as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "import_declaration" => Ok(Self::ImportDeclaration(::std::boxed::Box::new(
                <ImportDeclaration as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "object_definition" => Ok(Self::ObjectDefinition(::std::boxed::Box::new(
                <ObjectDefinition as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "package_clause" => Ok(Self::PackageClause(::std::boxed::Box::new(
                <PackageClause as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "package_object" => Ok(Self::PackageObject(::std::boxed::Box::new(
                <PackageObject as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "trait_definition" => Ok(Self::TraitDefinition(::std::boxed::Box::new(
                <TraitDefinition as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "type_definition" => Ok(Self::TypeDefinition(::std::boxed::Box::new(
                <TypeDefinition as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "val_declaration" => Ok(Self::ValDeclaration(::std::boxed::Box::new(
                <ValDeclaration as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "val_definition" => Ok(Self::ValDefinition(::std::boxed::Box::new(
                <ValDefinition as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "var_declaration" => Ok(Self::VarDeclaration(::std::boxed::Box::new(
                <VarDeclaration as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "var_definition" => Ok(Self::VarDefinition(::std::boxed::Box::new(
                <VarDefinition as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            other => Err(::treesitter_types::ParseError::unexpected_kind(other, node)),
        }
    }
}
impl ::treesitter_types::Spanned for Definition<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        match self {
            Self::ClassDefinition(inner) => inner.span(),
            Self::EnumDefinition(inner) => inner.span(),
            Self::ExportDeclaration(inner) => inner.span(),
            Self::ExtensionDefinition(inner) => inner.span(),
            Self::FunctionDeclaration(inner) => inner.span(),
            Self::FunctionDefinition(inner) => inner.span(),
            Self::GivenDefinition(inner) => inner.span(),
            Self::ImportDeclaration(inner) => inner.span(),
            Self::ObjectDefinition(inner) => inner.span(),
            Self::PackageClause(inner) => inner.span(),
            Self::PackageObject(inner) => inner.span(),
            Self::TraitDefinition(inner) => inner.span(),
            Self::TypeDefinition(inner) => inner.span(),
            Self::ValDeclaration(inner) => inner.span(),
            Self::ValDefinition(inner) => inner.span(),
            Self::VarDeclaration(inner) => inner.span(),
            Self::VarDefinition(inner) => inner.span(),
        }
    }
}
#[derive(Debug, Clone)]
pub enum Pattern<'tree> {
    AlternativePattern(::std::boxed::Box<AlternativePattern<'tree>>),
    BooleanLiteral(::std::boxed::Box<BooleanLiteral<'tree>>),
    CapturePattern(::std::boxed::Box<CapturePattern<'tree>>),
    CaseClassPattern(::std::boxed::Box<CaseClassPattern<'tree>>),
    CharacterLiteral(::std::boxed::Box<CharacterLiteral<'tree>>),
    FloatingPointLiteral(::std::boxed::Box<FloatingPointLiteral<'tree>>),
    GivenPattern(::std::boxed::Box<GivenPattern<'tree>>),
    Identifier(::std::boxed::Box<Identifier<'tree>>),
    InfixPattern(::std::boxed::Box<InfixPattern<'tree>>),
    IntegerLiteral(::std::boxed::Box<IntegerLiteral<'tree>>),
    InterpolatedStringExpression(::std::boxed::Box<InterpolatedStringExpression<'tree>>),
    NamedTuplePattern(::std::boxed::Box<NamedTuplePattern<'tree>>),
    NullLiteral(::std::boxed::Box<NullLiteral<'tree>>),
    OperatorIdentifier(::std::boxed::Box<OperatorIdentifier<'tree>>),
    QuoteExpression(::std::boxed::Box<QuoteExpression<'tree>>),
    RepeatPattern(::std::boxed::Box<RepeatPattern<'tree>>),
    StableIdentifier(::std::boxed::Box<StableIdentifier<'tree>>),
    String(::std::boxed::Box<String<'tree>>),
    TuplePattern(::std::boxed::Box<TuplePattern<'tree>>),
    TypedPattern(::std::boxed::Box<TypedPattern<'tree>>),
    Wildcard(::std::boxed::Box<Wildcard<'tree>>),
}
impl<'tree> ::treesitter_types::FromNode<'tree> for Pattern<'tree> {
    #[allow(clippy::collapsible_else_if)]
    fn from_node(
        node: ::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        match node.kind() {
            "alternative_pattern" => Ok(Self::AlternativePattern(::std::boxed::Box::new(
                <AlternativePattern as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "boolean_literal" => Ok(Self::BooleanLiteral(::std::boxed::Box::new(
                <BooleanLiteral as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "capture_pattern" => Ok(Self::CapturePattern(::std::boxed::Box::new(
                <CapturePattern as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "case_class_pattern" => Ok(Self::CaseClassPattern(::std::boxed::Box::new(
                <CaseClassPattern as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "character_literal" => Ok(Self::CharacterLiteral(::std::boxed::Box::new(
                <CharacterLiteral as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "floating_point_literal" => Ok(Self::FloatingPointLiteral(::std::boxed::Box::new(
                <FloatingPointLiteral as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "given_pattern" => Ok(Self::GivenPattern(::std::boxed::Box::new(
                <GivenPattern as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "identifier" => Ok(Self::Identifier(::std::boxed::Box::new(
                <Identifier as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "infix_pattern" => Ok(Self::InfixPattern(::std::boxed::Box::new(
                <InfixPattern as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "integer_literal" => Ok(Self::IntegerLiteral(::std::boxed::Box::new(
                <IntegerLiteral as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "interpolated_string_expression" => {
                Ok(Self::InterpolatedStringExpression(::std::boxed::Box::new(
                    <InterpolatedStringExpression as ::treesitter_types::FromNode>::from_node(
                        node, src,
                    )?,
                )))
            }
            "named_tuple_pattern" => Ok(Self::NamedTuplePattern(::std::boxed::Box::new(
                <NamedTuplePattern as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "null_literal" => Ok(Self::NullLiteral(::std::boxed::Box::new(
                <NullLiteral as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "operator_identifier" => Ok(Self::OperatorIdentifier(::std::boxed::Box::new(
                <OperatorIdentifier as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "quote_expression" => Ok(Self::QuoteExpression(::std::boxed::Box::new(
                <QuoteExpression as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "repeat_pattern" => Ok(Self::RepeatPattern(::std::boxed::Box::new(
                <RepeatPattern as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "stable_identifier" => Ok(Self::StableIdentifier(::std::boxed::Box::new(
                <StableIdentifier as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "string" => Ok(Self::String(::std::boxed::Box::new(
                <String as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "tuple_pattern" => Ok(Self::TuplePattern(::std::boxed::Box::new(
                <TuplePattern as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "typed_pattern" => Ok(Self::TypedPattern(::std::boxed::Box::new(
                <TypedPattern as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "wildcard" => Ok(Self::Wildcard(::std::boxed::Box::new(
                <Wildcard as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            other => Err(::treesitter_types::ParseError::unexpected_kind(other, node)),
        }
    }
}
impl ::treesitter_types::Spanned for Pattern<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        match self {
            Self::AlternativePattern(inner) => inner.span(),
            Self::BooleanLiteral(inner) => inner.span(),
            Self::CapturePattern(inner) => inner.span(),
            Self::CaseClassPattern(inner) => inner.span(),
            Self::CharacterLiteral(inner) => inner.span(),
            Self::FloatingPointLiteral(inner) => inner.span(),
            Self::GivenPattern(inner) => inner.span(),
            Self::Identifier(inner) => inner.span(),
            Self::InfixPattern(inner) => inner.span(),
            Self::IntegerLiteral(inner) => inner.span(),
            Self::InterpolatedStringExpression(inner) => inner.span(),
            Self::NamedTuplePattern(inner) => inner.span(),
            Self::NullLiteral(inner) => inner.span(),
            Self::OperatorIdentifier(inner) => inner.span(),
            Self::QuoteExpression(inner) => inner.span(),
            Self::RepeatPattern(inner) => inner.span(),
            Self::StableIdentifier(inner) => inner.span(),
            Self::String(inner) => inner.span(),
            Self::TuplePattern(inner) => inner.span(),
            Self::TypedPattern(inner) => inner.span(),
            Self::Wildcard(inner) => inner.span(),
        }
    }
}
#[derive(Debug, Clone)]
pub enum Expression<'tree> {
    AscriptionExpression(::std::boxed::Box<AscriptionExpression<'tree>>),
    AssignmentExpression(::std::boxed::Box<AssignmentExpression<'tree>>),
    Block(::std::boxed::Box<Block<'tree>>),
    BooleanLiteral(::std::boxed::Box<BooleanLiteral<'tree>>),
    CallExpression(::std::boxed::Box<CallExpression<'tree>>),
    CaseBlock(::std::boxed::Box<CaseBlock<'tree>>),
    CharacterLiteral(::std::boxed::Box<CharacterLiteral<'tree>>),
    DoWhileExpression(::std::boxed::Box<DoWhileExpression<'tree>>),
    FieldExpression(::std::boxed::Box<FieldExpression<'tree>>),
    FloatingPointLiteral(::std::boxed::Box<FloatingPointLiteral<'tree>>),
    ForExpression(::std::boxed::Box<ForExpression<'tree>>),
    GenericFunction(::std::boxed::Box<GenericFunction<'tree>>),
    Identifier(::std::boxed::Box<Identifier<'tree>>),
    IfExpression(::std::boxed::Box<IfExpression<'tree>>),
    InfixExpression(::std::boxed::Box<InfixExpression<'tree>>),
    InstanceExpression(::std::boxed::Box<InstanceExpression<'tree>>),
    IntegerLiteral(::std::boxed::Box<IntegerLiteral<'tree>>),
    InterpolatedStringExpression(::std::boxed::Box<InterpolatedStringExpression<'tree>>),
    LambdaExpression(::std::boxed::Box<LambdaExpression<'tree>>),
    MacroBody(::std::boxed::Box<MacroBody<'tree>>),
    MatchExpression(::std::boxed::Box<MatchExpression<'tree>>),
    NullLiteral(::std::boxed::Box<NullLiteral<'tree>>),
    OperatorIdentifier(::std::boxed::Box<OperatorIdentifier<'tree>>),
    ParenthesizedExpression(::std::boxed::Box<ParenthesizedExpression<'tree>>),
    PostfixExpression(::std::boxed::Box<PostfixExpression<'tree>>),
    PrefixExpression(::std::boxed::Box<PrefixExpression<'tree>>),
    QuoteExpression(::std::boxed::Box<QuoteExpression<'tree>>),
    ReturnExpression(::std::boxed::Box<ReturnExpression<'tree>>),
    SpliceExpression(::std::boxed::Box<SpliceExpression<'tree>>),
    String(::std::boxed::Box<String<'tree>>),
    ThrowExpression(::std::boxed::Box<ThrowExpression<'tree>>),
    TryExpression(::std::boxed::Box<TryExpression<'tree>>),
    TupleExpression(::std::boxed::Box<TupleExpression<'tree>>),
    Unit(::std::boxed::Box<Unit<'tree>>),
    WhileExpression(::std::boxed::Box<WhileExpression<'tree>>),
    Wildcard(::std::boxed::Box<Wildcard<'tree>>),
}
impl<'tree> ::treesitter_types::FromNode<'tree> for Expression<'tree> {
    #[allow(clippy::collapsible_else_if)]
    fn from_node(
        node: ::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        match node.kind() {
            "ascription_expression" => Ok(Self::AscriptionExpression(::std::boxed::Box::new(
                <AscriptionExpression as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "assignment_expression" => Ok(Self::AssignmentExpression(::std::boxed::Box::new(
                <AssignmentExpression as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "block" => Ok(Self::Block(::std::boxed::Box::new(
                <Block as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "boolean_literal" => Ok(Self::BooleanLiteral(::std::boxed::Box::new(
                <BooleanLiteral as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "call_expression" => Ok(Self::CallExpression(::std::boxed::Box::new(
                <CallExpression as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "case_block" => Ok(Self::CaseBlock(::std::boxed::Box::new(
                <CaseBlock as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "character_literal" => Ok(Self::CharacterLiteral(::std::boxed::Box::new(
                <CharacterLiteral as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "do_while_expression" => Ok(Self::DoWhileExpression(::std::boxed::Box::new(
                <DoWhileExpression as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "field_expression" => Ok(Self::FieldExpression(::std::boxed::Box::new(
                <FieldExpression as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "floating_point_literal" => Ok(Self::FloatingPointLiteral(::std::boxed::Box::new(
                <FloatingPointLiteral as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "for_expression" => Ok(Self::ForExpression(::std::boxed::Box::new(
                <ForExpression as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "generic_function" => Ok(Self::GenericFunction(::std::boxed::Box::new(
                <GenericFunction as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "identifier" => Ok(Self::Identifier(::std::boxed::Box::new(
                <Identifier as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "if_expression" => Ok(Self::IfExpression(::std::boxed::Box::new(
                <IfExpression as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "infix_expression" => Ok(Self::InfixExpression(::std::boxed::Box::new(
                <InfixExpression as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "instance_expression" => Ok(Self::InstanceExpression(::std::boxed::Box::new(
                <InstanceExpression as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "integer_literal" => Ok(Self::IntegerLiteral(::std::boxed::Box::new(
                <IntegerLiteral as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "interpolated_string_expression" => {
                Ok(Self::InterpolatedStringExpression(::std::boxed::Box::new(
                    <InterpolatedStringExpression as ::treesitter_types::FromNode>::from_node(
                        node, src,
                    )?,
                )))
            }
            "lambda_expression" => Ok(Self::LambdaExpression(::std::boxed::Box::new(
                <LambdaExpression as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "macro_body" => Ok(Self::MacroBody(::std::boxed::Box::new(
                <MacroBody as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "match_expression" => Ok(Self::MatchExpression(::std::boxed::Box::new(
                <MatchExpression as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "null_literal" => Ok(Self::NullLiteral(::std::boxed::Box::new(
                <NullLiteral as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "operator_identifier" => Ok(Self::OperatorIdentifier(::std::boxed::Box::new(
                <OperatorIdentifier as ::treesitter_types::FromNode>::from_node(node, src)?,
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
            "prefix_expression" => Ok(Self::PrefixExpression(::std::boxed::Box::new(
                <PrefixExpression as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "quote_expression" => Ok(Self::QuoteExpression(::std::boxed::Box::new(
                <QuoteExpression as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "return_expression" => Ok(Self::ReturnExpression(::std::boxed::Box::new(
                <ReturnExpression as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "splice_expression" => Ok(Self::SpliceExpression(::std::boxed::Box::new(
                <SpliceExpression as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "string" => Ok(Self::String(::std::boxed::Box::new(
                <String as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "throw_expression" => Ok(Self::ThrowExpression(::std::boxed::Box::new(
                <ThrowExpression as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "try_expression" => Ok(Self::TryExpression(::std::boxed::Box::new(
                <TryExpression as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "tuple_expression" => Ok(Self::TupleExpression(::std::boxed::Box::new(
                <TupleExpression as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "unit" => Ok(Self::Unit(::std::boxed::Box::new(
                <Unit as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "while_expression" => Ok(Self::WhileExpression(::std::boxed::Box::new(
                <WhileExpression as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "wildcard" => Ok(Self::Wildcard(::std::boxed::Box::new(
                <Wildcard as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            other => Err(::treesitter_types::ParseError::unexpected_kind(other, node)),
        }
    }
}
impl ::treesitter_types::Spanned for Expression<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        match self {
            Self::AscriptionExpression(inner) => inner.span(),
            Self::AssignmentExpression(inner) => inner.span(),
            Self::Block(inner) => inner.span(),
            Self::BooleanLiteral(inner) => inner.span(),
            Self::CallExpression(inner) => inner.span(),
            Self::CaseBlock(inner) => inner.span(),
            Self::CharacterLiteral(inner) => inner.span(),
            Self::DoWhileExpression(inner) => inner.span(),
            Self::FieldExpression(inner) => inner.span(),
            Self::FloatingPointLiteral(inner) => inner.span(),
            Self::ForExpression(inner) => inner.span(),
            Self::GenericFunction(inner) => inner.span(),
            Self::Identifier(inner) => inner.span(),
            Self::IfExpression(inner) => inner.span(),
            Self::InfixExpression(inner) => inner.span(),
            Self::InstanceExpression(inner) => inner.span(),
            Self::IntegerLiteral(inner) => inner.span(),
            Self::InterpolatedStringExpression(inner) => inner.span(),
            Self::LambdaExpression(inner) => inner.span(),
            Self::MacroBody(inner) => inner.span(),
            Self::MatchExpression(inner) => inner.span(),
            Self::NullLiteral(inner) => inner.span(),
            Self::OperatorIdentifier(inner) => inner.span(),
            Self::ParenthesizedExpression(inner) => inner.span(),
            Self::PostfixExpression(inner) => inner.span(),
            Self::PrefixExpression(inner) => inner.span(),
            Self::QuoteExpression(inner) => inner.span(),
            Self::ReturnExpression(inner) => inner.span(),
            Self::SpliceExpression(inner) => inner.span(),
            Self::String(inner) => inner.span(),
            Self::ThrowExpression(inner) => inner.span(),
            Self::TryExpression(inner) => inner.span(),
            Self::TupleExpression(inner) => inner.span(),
            Self::Unit(inner) => inner.span(),
            Self::WhileExpression(inner) => inner.span(),
            Self::Wildcard(inner) => inner.span(),
        }
    }
}
#[derive(Debug, Clone)]
pub struct AccessModifier<'tree> {
    pub span: ::treesitter_types::Span,
    pub children: ::core::option::Option<AccessQualifier<'tree>>,
}
impl<'tree> ::treesitter_types::FromNode<'tree> for AccessModifier<'tree> {
    #[allow(clippy::match_single_binding, clippy::suspicious_else_formatting)]
    fn from_node(
        node: ::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        debug_assert_eq!(node.kind(), "access_modifier");
        Ok(Self {
            span: ::treesitter_types::Span::from(node),
            children: {
                #[allow(clippy::suspicious_else_formatting)]
                let non_field_children = {
                    let mut cursor = node.walk();
                    let mut result = ::std::vec::Vec::new();
                    if cursor.goto_first_child() {
                        loop {
                            if cursor.field_name().is_none()
                                && cursor.node().is_named()
                                && !cursor.node().is_extra()
                            {
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
                        <AccessQualifier as ::treesitter_types::FromNode>::from_node(child, src)?,
                    ),
                    None => None,
                }
            },
        })
    }
}
impl ::treesitter_types::Spanned for AccessModifier<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        self.span
    }
}
#[derive(Debug, Clone)]
pub struct AccessQualifier<'tree> {
    pub span: ::treesitter_types::Span,
    pub children: AccessQualifierChildren<'tree>,
}
impl<'tree> ::treesitter_types::FromNode<'tree> for AccessQualifier<'tree> {
    #[allow(clippy::match_single_binding, clippy::suspicious_else_formatting)]
    fn from_node(
        node: ::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        debug_assert_eq!(node.kind(), "access_qualifier");
        Ok(Self {
            span: ::treesitter_types::Span::from(node),
            children: {
                #[allow(clippy::suspicious_else_formatting)]
                let non_field_children = {
                    let mut cursor = node.walk();
                    let mut result = ::std::vec::Vec::new();
                    if cursor.goto_first_child() {
                        loop {
                            if cursor.field_name().is_none()
                                && cursor.node().is_named()
                                && !cursor.node().is_extra()
                            {
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
                                        <AccessQualifierChildren as ::treesitter_types::FromNode>::from_node(
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
                                            <AccessQualifierChildren as ::treesitter_types::FromNode>::from_node(
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
                <AccessQualifierChildren as ::treesitter_types::FromNode>::from_node(child, src)?
            },
        })
    }
}
impl ::treesitter_types::Spanned for AccessQualifier<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        self.span
    }
}
#[derive(Debug, Clone)]
pub struct AlternativePattern<'tree> {
    pub span: ::treesitter_types::Span,
    pub children: ::std::vec::Vec<Pattern<'tree>>,
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
            children: {
                #[allow(clippy::suspicious_else_formatting)]
                let non_field_children = {
                    let mut cursor = node.walk();
                    let mut result = ::std::vec::Vec::new();
                    if cursor.goto_first_child() {
                        loop {
                            if cursor.field_name().is_none()
                                && cursor.node().is_named()
                                && !cursor.node().is_extra()
                            {
                                result.push(cursor.node());
                            }
                            if !cursor.goto_next_sibling() {
                                break;
                            }
                        }
                    }
                    result
                };
                let mut items = ::std::vec::Vec::new();
                for child in non_field_children {
                    items.push(<Pattern as ::treesitter_types::FromNode>::from_node(
                        child, src,
                    )?);
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
#[derive(Debug, Clone)]
pub struct AnnotatedType<'tree> {
    pub span: ::treesitter_types::Span,
    pub children: ::std::vec::Vec<AnnotatedTypeChildren<'tree>>,
}
impl<'tree> ::treesitter_types::FromNode<'tree> for AnnotatedType<'tree> {
    #[allow(clippy::match_single_binding, clippy::suspicious_else_formatting)]
    fn from_node(
        node: ::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        debug_assert_eq!(node.kind(), "annotated_type");
        Ok(Self {
            span: ::treesitter_types::Span::from(node),
            children: {
                #[allow(clippy::suspicious_else_formatting)]
                let non_field_children = {
                    let mut cursor = node.walk();
                    let mut result = ::std::vec::Vec::new();
                    if cursor.goto_first_child() {
                        loop {
                            if cursor.field_name().is_none()
                                && cursor.node().is_named()
                                && !cursor.node().is_extra()
                            {
                                result.push(cursor.node());
                            }
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
                        <AnnotatedTypeChildren as ::treesitter_types::FromNode>::from_node(
                            child, src,
                        )?,
                    );
                }
                items
            },
        })
    }
}
impl ::treesitter_types::Spanned for AnnotatedType<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        self.span
    }
}
#[derive(Debug, Clone)]
pub struct Annotation<'tree> {
    pub span: ::treesitter_types::Span,
    pub arguments: ::std::vec::Vec<Arguments<'tree>>,
    pub name: AnnotationName<'tree>,
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
            arguments: {
                let mut cursor = node.walk();
                let mut items = ::std::vec::Vec::new();
                for child in node.children_by_field_name("arguments", &mut cursor) {
                    items.push(<Arguments as ::treesitter_types::FromNode>::from_node(
                        child, src,
                    )?);
                }
                items
            },
            name: {
                let child = node
                    .child_by_field_name("name")
                    .ok_or_else(|| ::treesitter_types::ParseError::missing_field("name", node))?;
                <AnnotationName as ::treesitter_types::FromNode>::from_node(child, src)?
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
pub struct AppliedConstructorType<'tree> {
    pub span: ::treesitter_types::Span,
    pub children: ::std::vec::Vec<AppliedConstructorTypeChildren<'tree>>,
}
impl<'tree> ::treesitter_types::FromNode<'tree> for AppliedConstructorType<'tree> {
    #[allow(clippy::match_single_binding, clippy::suspicious_else_formatting)]
    fn from_node(
        node: ::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        debug_assert_eq!(node.kind(), "applied_constructor_type");
        Ok(Self {
            span: ::treesitter_types::Span::from(node),
            children: {
                #[allow(clippy::suspicious_else_formatting)]
                let non_field_children = {
                    let mut cursor = node.walk();
                    let mut result = ::std::vec::Vec::new();
                    if cursor.goto_first_child() {
                        loop {
                            if cursor.field_name().is_none()
                                && cursor.node().is_named()
                                && !cursor.node().is_extra()
                            {
                                result.push(cursor.node());
                            }
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
                            <AppliedConstructorTypeChildren as ::treesitter_types::FromNode>::from_node(
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
impl ::treesitter_types::Spanned for AppliedConstructorType<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        self.span
    }
}
#[derive(Debug, Clone)]
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
#[derive(Debug, Clone)]
pub struct ArrowRenamedIdentifier<'tree> {
    pub span: ::treesitter_types::Span,
    pub alias: ArrowRenamedIdentifierAlias<'tree>,
    pub name: ArrowRenamedIdentifierName<'tree>,
}
impl<'tree> ::treesitter_types::FromNode<'tree> for ArrowRenamedIdentifier<'tree> {
    #[allow(clippy::match_single_binding, clippy::suspicious_else_formatting)]
    fn from_node(
        node: ::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        debug_assert_eq!(node.kind(), "arrow_renamed_identifier");
        Ok(Self {
            span: ::treesitter_types::Span::from(node),
            alias: {
                let child = node
                    .child_by_field_name("alias")
                    .ok_or_else(|| ::treesitter_types::ParseError::missing_field("alias", node))?;
                <ArrowRenamedIdentifierAlias as ::treesitter_types::FromNode>::from_node(
                    child, src,
                )?
            },
            name: {
                let child = node
                    .child_by_field_name("name")
                    .ok_or_else(|| ::treesitter_types::ParseError::missing_field("name", node))?;
                <ArrowRenamedIdentifierName as ::treesitter_types::FromNode>::from_node(child, src)?
            },
        })
    }
}
impl ::treesitter_types::Spanned for ArrowRenamedIdentifier<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        self.span
    }
}
#[derive(Debug, Clone)]
pub struct AsRenamedIdentifier<'tree> {
    pub span: ::treesitter_types::Span,
    pub alias: AsRenamedIdentifierAlias<'tree>,
    pub name: AsRenamedIdentifierName<'tree>,
}
impl<'tree> ::treesitter_types::FromNode<'tree> for AsRenamedIdentifier<'tree> {
    #[allow(clippy::match_single_binding, clippy::suspicious_else_formatting)]
    fn from_node(
        node: ::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        debug_assert_eq!(node.kind(), "as_renamed_identifier");
        Ok(Self {
            span: ::treesitter_types::Span::from(node),
            alias: {
                let child = node
                    .child_by_field_name("alias")
                    .ok_or_else(|| ::treesitter_types::ParseError::missing_field("alias", node))?;
                <AsRenamedIdentifierAlias as ::treesitter_types::FromNode>::from_node(child, src)?
            },
            name: {
                let child = node
                    .child_by_field_name("name")
                    .ok_or_else(|| ::treesitter_types::ParseError::missing_field("name", node))?;
                <AsRenamedIdentifierName as ::treesitter_types::FromNode>::from_node(child, src)?
            },
        })
    }
}
impl ::treesitter_types::Spanned for AsRenamedIdentifier<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        self.span
    }
}
#[derive(Debug, Clone)]
pub struct AscriptionExpression<'tree> {
    pub span: ::treesitter_types::Span,
    pub children: ::std::vec::Vec<AscriptionExpressionChildren<'tree>>,
}
impl<'tree> ::treesitter_types::FromNode<'tree> for AscriptionExpression<'tree> {
    #[allow(clippy::match_single_binding, clippy::suspicious_else_formatting)]
    fn from_node(
        node: ::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        debug_assert_eq!(node.kind(), "ascription_expression");
        Ok(Self {
            span: ::treesitter_types::Span::from(node),
            children: {
                #[allow(clippy::suspicious_else_formatting)]
                let non_field_children = {
                    let mut cursor = node.walk();
                    let mut result = ::std::vec::Vec::new();
                    if cursor.goto_first_child() {
                        loop {
                            if cursor.field_name().is_none()
                                && cursor.node().is_named()
                                && !cursor.node().is_extra()
                            {
                                result.push(cursor.node());
                            }
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
                        <AscriptionExpressionChildren as ::treesitter_types::FromNode>::from_node(
                            child, src,
                        )?,
                    );
                }
                items
            },
        })
    }
}
impl ::treesitter_types::Spanned for AscriptionExpression<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        self.span
    }
}
#[derive(Debug, Clone)]
pub struct AssignmentExpression<'tree> {
    pub span: ::treesitter_types::Span,
    pub left: AssignmentExpressionLeft<'tree>,
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
                <AssignmentExpressionLeft as ::treesitter_types::FromNode>::from_node(child, src)?
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
pub struct Binding<'tree> {
    pub span: ::treesitter_types::Span,
    pub name: ::core::option::Option<BindingName<'tree>>,
    pub r#type: ::core::option::Option<BindingType<'tree>>,
    pub children: ::core::option::Option<Wildcard<'tree>>,
}
impl<'tree> ::treesitter_types::FromNode<'tree> for Binding<'tree> {
    #[allow(clippy::match_single_binding, clippy::suspicious_else_formatting)]
    fn from_node(
        node: ::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        debug_assert_eq!(node.kind(), "binding");
        Ok(Self {
            span: ::treesitter_types::Span::from(node),
            name: match node.child_by_field_name("name") {
                Some(child) => Some(<BindingName as ::treesitter_types::FromNode>::from_node(
                    child, src,
                )?),
                None => None,
            },
            r#type: match node.child_by_field_name("type") {
                Some(child) => Some(<BindingType as ::treesitter_types::FromNode>::from_node(
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
                    Some(&child) => Some(<Wildcard as ::treesitter_types::FromNode>::from_node(
                        child, src,
                    )?),
                    None => None,
                }
            },
        })
    }
}
impl ::treesitter_types::Spanned for Binding<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        self.span
    }
}
#[derive(Debug, Clone)]
pub struct Bindings<'tree> {
    pub span: ::treesitter_types::Span,
    pub children: ::std::vec::Vec<Binding<'tree>>,
}
impl<'tree> ::treesitter_types::FromNode<'tree> for Bindings<'tree> {
    #[allow(clippy::match_single_binding, clippy::suspicious_else_formatting)]
    fn from_node(
        node: ::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        debug_assert_eq!(node.kind(), "bindings");
        Ok(Self {
            span: ::treesitter_types::Span::from(node),
            children: {
                #[allow(clippy::suspicious_else_formatting)]
                let non_field_children = {
                    let mut cursor = node.walk();
                    let mut result = ::std::vec::Vec::new();
                    if cursor.goto_first_child() {
                        loop {
                            if cursor.field_name().is_none()
                                && cursor.node().is_named()
                                && !cursor.node().is_extra()
                            {
                                result.push(cursor.node());
                            }
                            if !cursor.goto_next_sibling() {
                                break;
                            }
                        }
                    }
                    result
                };
                let mut items = ::std::vec::Vec::new();
                for child in non_field_children {
                    items.push(<Binding as ::treesitter_types::FromNode>::from_node(
                        child, src,
                    )?);
                }
                items
            },
        })
    }
}
impl ::treesitter_types::Spanned for Bindings<'_> {
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
pub struct BlockComment<'tree> {
    pub span: ::treesitter_types::Span,
    text: &'tree str,
}
impl<'tree> ::treesitter_types::FromNode<'tree> for BlockComment<'tree> {
    fn from_node(
        node: ::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        debug_assert_eq!(node.kind(), "block_comment");
        Ok(Self {
            span: ::treesitter_types::Span::from(node),
            text: node.utf8_text(src)?,
        })
    }
}
impl<'tree> ::treesitter_types::LeafNode<'tree> for BlockComment<'tree> {
    fn text(&self) -> &'tree str {
        self.text
    }
}
impl ::treesitter_types::Spanned for BlockComment<'_> {
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
pub struct CallExpression<'tree> {
    pub span: ::treesitter_types::Span,
    pub arguments: CallExpressionArguments<'tree>,
    pub function: CallExpressionFunction<'tree>,
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
                <CallExpressionArguments as ::treesitter_types::FromNode>::from_node(child, src)?
            },
            function: {
                let child = node.child_by_field_name("function").ok_or_else(|| {
                    ::treesitter_types::ParseError::missing_field("function", node)
                })?;
                <CallExpressionFunction as ::treesitter_types::FromNode>::from_node(child, src)?
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
pub struct CapturePattern<'tree> {
    pub span: ::treesitter_types::Span,
    pub name: CapturePatternName<'tree>,
    pub pattern: Pattern<'tree>,
}
impl<'tree> ::treesitter_types::FromNode<'tree> for CapturePattern<'tree> {
    #[allow(clippy::match_single_binding, clippy::suspicious_else_formatting)]
    fn from_node(
        node: ::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        debug_assert_eq!(node.kind(), "capture_pattern");
        Ok(Self {
            span: ::treesitter_types::Span::from(node),
            name: {
                let child = node
                    .child_by_field_name("name")
                    .ok_or_else(|| ::treesitter_types::ParseError::missing_field("name", node))?;
                <CapturePatternName as ::treesitter_types::FromNode>::from_node(child, src)?
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
impl ::treesitter_types::Spanned for CapturePattern<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        self.span
    }
}
#[derive(Debug, Clone)]
pub struct CaseBlock<'tree> {
    pub span: ::treesitter_types::Span,
    pub children: ::std::vec::Vec<CaseClause<'tree>>,
}
impl<'tree> ::treesitter_types::FromNode<'tree> for CaseBlock<'tree> {
    #[allow(clippy::match_single_binding, clippy::suspicious_else_formatting)]
    fn from_node(
        node: ::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        debug_assert_eq!(node.kind(), "case_block");
        Ok(Self {
            span: ::treesitter_types::Span::from(node),
            children: {
                #[allow(clippy::suspicious_else_formatting)]
                let non_field_children = {
                    let mut cursor = node.walk();
                    let mut result = ::std::vec::Vec::new();
                    if cursor.goto_first_child() {
                        loop {
                            if cursor.field_name().is_none()
                                && cursor.node().is_named()
                                && !cursor.node().is_extra()
                            {
                                result.push(cursor.node());
                            }
                            if !cursor.goto_next_sibling() {
                                break;
                            }
                        }
                    }
                    result
                };
                let mut items = ::std::vec::Vec::new();
                for child in non_field_children {
                    items.push(<CaseClause as ::treesitter_types::FromNode>::from_node(
                        child, src,
                    )?);
                }
                items
            },
        })
    }
}
impl ::treesitter_types::Spanned for CaseBlock<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        self.span
    }
}
#[derive(Debug, Clone)]
pub struct CaseClassPattern<'tree> {
    pub span: ::treesitter_types::Span,
    pub pattern: ::std::vec::Vec<CaseClassPatternPattern<'tree>>,
    pub r#type: CaseClassPatternType<'tree>,
}
impl<'tree> ::treesitter_types::FromNode<'tree> for CaseClassPattern<'tree> {
    #[allow(clippy::match_single_binding, clippy::suspicious_else_formatting)]
    fn from_node(
        node: ::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        debug_assert_eq!(node.kind(), "case_class_pattern");
        Ok(Self {
            span: ::treesitter_types::Span::from(node),
            pattern: {
                let mut cursor = node.walk();
                let mut items = ::std::vec::Vec::new();
                for child in node.children_by_field_name("pattern", &mut cursor) {
                    items.push(
                        <CaseClassPatternPattern as ::treesitter_types::FromNode>::from_node(
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
                <CaseClassPatternType as ::treesitter_types::FromNode>::from_node(child, src)?
            },
        })
    }
}
impl ::treesitter_types::Spanned for CaseClassPattern<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        self.span
    }
}
#[derive(Debug, Clone)]
pub struct CaseClause<'tree> {
    pub span: ::treesitter_types::Span,
    pub body: ::std::vec::Vec<CaseClauseBody<'tree>>,
    pub pattern: Pattern<'tree>,
    pub children: ::core::option::Option<Guard<'tree>>,
}
impl<'tree> ::treesitter_types::FromNode<'tree> for CaseClause<'tree> {
    #[allow(clippy::match_single_binding, clippy::suspicious_else_formatting)]
    fn from_node(
        node: ::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        debug_assert_eq!(node.kind(), "case_clause");
        Ok(Self {
            span: ::treesitter_types::Span::from(node),
            body: {
                let mut cursor = node.walk();
                let mut items = ::std::vec::Vec::new();
                for child in node.children_by_field_name("body", &mut cursor) {
                    items.push(<CaseClauseBody as ::treesitter_types::FromNode>::from_node(
                        child, src,
                    )?);
                }
                items
            },
            pattern: {
                let child = node.child_by_field_name("pattern").ok_or_else(|| {
                    ::treesitter_types::ParseError::missing_field("pattern", node)
                })?;
                <Pattern as ::treesitter_types::FromNode>::from_node(child, src)?
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
                    Some(&child) => Some(<Guard as ::treesitter_types::FromNode>::from_node(
                        child, src,
                    )?),
                    None => None,
                }
            },
        })
    }
}
impl ::treesitter_types::Spanned for CaseClause<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        self.span
    }
}
#[derive(Debug, Clone)]
pub struct CatchClause<'tree> {
    pub span: ::treesitter_types::Span,
    pub body: ::core::option::Option<Expression<'tree>>,
    pub pattern: ::core::option::Option<Pattern<'tree>>,
    pub children: ::core::option::Option<CatchClauseChildren<'tree>>,
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
            body: match node.child_by_field_name("body") {
                Some(child) => Some(<Expression as ::treesitter_types::FromNode>::from_node(
                    child, src,
                )?),
                None => None,
            },
            pattern: match node.child_by_field_name("pattern") {
                Some(child) => Some(<Pattern as ::treesitter_types::FromNode>::from_node(
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
                        <CatchClauseChildren as ::treesitter_types::FromNode>::from_node(
                            child, src,
                        )?,
                    ),
                    None => None,
                }
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
pub struct ClassDefinition<'tree> {
    pub span: ::treesitter_types::Span,
    pub body: ::std::vec::Vec<TemplateBody<'tree>>,
    pub class_parameters: ::std::vec::Vec<ClassParameters<'tree>>,
    pub derive: ::core::option::Option<DerivesClause<'tree>>,
    pub extend: ::core::option::Option<ExtendsClause<'tree>>,
    pub name: ClassDefinitionName<'tree>,
    pub type_parameters: ::core::option::Option<TypeParameters<'tree>>,
    pub children: ::std::vec::Vec<ClassDefinitionChildren<'tree>>,
}
impl<'tree> ::treesitter_types::FromNode<'tree> for ClassDefinition<'tree> {
    #[allow(clippy::match_single_binding, clippy::suspicious_else_formatting)]
    fn from_node(
        node: ::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        debug_assert_eq!(node.kind(), "class_definition");
        Ok(Self {
            span: ::treesitter_types::Span::from(node),
            body: {
                let mut cursor = node.walk();
                let mut items = ::std::vec::Vec::new();
                for child in node.children_by_field_name("body", &mut cursor) {
                    items.push(<TemplateBody as ::treesitter_types::FromNode>::from_node(
                        child, src,
                    )?);
                }
                items
            },
            class_parameters: {
                let mut cursor = node.walk();
                let mut items = ::std::vec::Vec::new();
                for child in node.children_by_field_name("class_parameters", &mut cursor) {
                    items.push(
                        <ClassParameters as ::treesitter_types::FromNode>::from_node(child, src)?,
                    );
                }
                items
            },
            derive: match node.child_by_field_name("derive") {
                Some(child) => Some(<DerivesClause as ::treesitter_types::FromNode>::from_node(
                    child, src,
                )?),
                None => None,
            },
            extend: match node.child_by_field_name("extend") {
                Some(child) => Some(<ExtendsClause as ::treesitter_types::FromNode>::from_node(
                    child, src,
                )?),
                None => None,
            },
            name: {
                let child = node
                    .child_by_field_name("name")
                    .ok_or_else(|| ::treesitter_types::ParseError::missing_field("name", node))?;
                <ClassDefinitionName as ::treesitter_types::FromNode>::from_node(child, src)?
            },
            type_parameters: match node.child_by_field_name("type_parameters") {
                Some(child) => Some(<TypeParameters as ::treesitter_types::FromNode>::from_node(
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
                        <ClassDefinitionChildren as ::treesitter_types::FromNode>::from_node(
                            child, src,
                        )?,
                    );
                }
                items
            },
        })
    }
}
impl ::treesitter_types::Spanned for ClassDefinition<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        self.span
    }
}
#[derive(Debug, Clone)]
pub struct ClassParameter<'tree> {
    pub span: ::treesitter_types::Span,
    pub default_value: ::core::option::Option<Expression<'tree>>,
    pub name: ClassParameterName<'tree>,
    pub r#type: ::core::option::Option<ClassParameterType<'tree>>,
    pub children: ::std::vec::Vec<ClassParameterChildren<'tree>>,
}
impl<'tree> ::treesitter_types::FromNode<'tree> for ClassParameter<'tree> {
    #[allow(clippy::match_single_binding, clippy::suspicious_else_formatting)]
    fn from_node(
        node: ::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        debug_assert_eq!(node.kind(), "class_parameter");
        Ok(Self {
            span: ::treesitter_types::Span::from(node),
            default_value: match node.child_by_field_name("default_value") {
                Some(child) => Some(<Expression as ::treesitter_types::FromNode>::from_node(
                    child, src,
                )?),
                None => None,
            },
            name: {
                let child = node
                    .child_by_field_name("name")
                    .ok_or_else(|| ::treesitter_types::ParseError::missing_field("name", node))?;
                <ClassParameterName as ::treesitter_types::FromNode>::from_node(child, src)?
            },
            r#type: match node.child_by_field_name("type") {
                Some(child) => Some(
                    <ClassParameterType as ::treesitter_types::FromNode>::from_node(child, src)?,
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
                        <ClassParameterChildren as ::treesitter_types::FromNode>::from_node(
                            child, src,
                        )?,
                    );
                }
                items
            },
        })
    }
}
impl ::treesitter_types::Spanned for ClassParameter<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        self.span
    }
}
#[derive(Debug, Clone)]
pub struct ClassParameters<'tree> {
    pub span: ::treesitter_types::Span,
    pub children: ::std::vec::Vec<ClassParametersChildren<'tree>>,
}
impl<'tree> ::treesitter_types::FromNode<'tree> for ClassParameters<'tree> {
    #[allow(clippy::match_single_binding, clippy::suspicious_else_formatting)]
    fn from_node(
        node: ::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        debug_assert_eq!(node.kind(), "class_parameters");
        Ok(Self {
            span: ::treesitter_types::Span::from(node),
            children: {
                #[allow(clippy::suspicious_else_formatting)]
                let non_field_children = {
                    let mut cursor = node.walk();
                    let mut result = ::std::vec::Vec::new();
                    if cursor.goto_first_child() {
                        loop {
                            if cursor.field_name().is_none()
                                && cursor.node().is_named()
                                && !cursor.node().is_extra()
                            {
                                result.push(cursor.node());
                            }
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
                        <ClassParametersChildren as ::treesitter_types::FromNode>::from_node(
                            child, src,
                        )?,
                    );
                }
                items
            },
        })
    }
}
impl ::treesitter_types::Spanned for ClassParameters<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        self.span
    }
}
#[derive(Debug, Clone)]
pub struct ColonArgument<'tree> {
    pub span: ::treesitter_types::Span,
    pub lambda_start: ::std::vec::Vec<ColonArgumentLambdaStart<'tree>>,
    pub children: ColonArgumentChildren<'tree>,
}
impl<'tree> ::treesitter_types::FromNode<'tree> for ColonArgument<'tree> {
    #[allow(clippy::match_single_binding, clippy::suspicious_else_formatting)]
    fn from_node(
        node: ::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        debug_assert_eq!(node.kind(), "colon_argument");
        Ok(Self {
            span: ::treesitter_types::Span::from(node),
            lambda_start: {
                let mut cursor = node.walk();
                let mut items = ::std::vec::Vec::new();
                for child in node.children_by_field_name("lambda_start", &mut cursor) {
                    items.push(
                        <ColonArgumentLambdaStart as ::treesitter_types::FromNode>::from_node(
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
                                        <ColonArgumentChildren as ::treesitter_types::FromNode>::from_node(
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
                                            <ColonArgumentChildren as ::treesitter_types::FromNode>::from_node(
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
                <ColonArgumentChildren as ::treesitter_types::FromNode>::from_node(child, src)?
            },
        })
    }
}
impl ::treesitter_types::Spanned for ColonArgument<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        self.span
    }
}
#[derive(Debug, Clone)]
pub struct Comment<'tree> {
    pub span: ::treesitter_types::Span,
    pub children: ::core::option::Option<UsingDirective<'tree>>,
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
            children: {
                #[allow(clippy::suspicious_else_formatting)]
                let non_field_children = {
                    let mut cursor = node.walk();
                    let mut result = ::std::vec::Vec::new();
                    if cursor.goto_first_child() {
                        loop {
                            if cursor.field_name().is_none()
                                && cursor.node().is_named()
                                && !cursor.node().is_extra()
                            {
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
                        <UsingDirective as ::treesitter_types::FromNode>::from_node(child, src)?,
                    ),
                    None => None,
                }
            },
        })
    }
}
impl ::treesitter_types::Spanned for Comment<'_> {
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
pub struct CompoundType<'tree> {
    pub span: ::treesitter_types::Span,
    pub base: CompoundTypeBase<'tree>,
    pub extra: ::std::vec::Vec<CompoundTypeExtra<'tree>>,
    pub children: ::core::option::Option<Refinement<'tree>>,
}
impl<'tree> ::treesitter_types::FromNode<'tree> for CompoundType<'tree> {
    #[allow(clippy::match_single_binding, clippy::suspicious_else_formatting)]
    fn from_node(
        node: ::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        debug_assert_eq!(node.kind(), "compound_type");
        Ok(Self {
            span: ::treesitter_types::Span::from(node),
            base: {
                let child = node
                    .child_by_field_name("base")
                    .ok_or_else(|| ::treesitter_types::ParseError::missing_field("base", node))?;
                <CompoundTypeBase as ::treesitter_types::FromNode>::from_node(child, src)?
            },
            extra: {
                let mut cursor = node.walk();
                let mut items = ::std::vec::Vec::new();
                for child in node.children_by_field_name("extra", &mut cursor) {
                    items.push(
                        <CompoundTypeExtra as ::treesitter_types::FromNode>::from_node(child, src)?,
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
                    Some(&child) => Some(<Refinement as ::treesitter_types::FromNode>::from_node(
                        child, src,
                    )?),
                    None => None,
                }
            },
        })
    }
}
impl ::treesitter_types::Spanned for CompoundType<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        self.span
    }
}
#[derive(Debug, Clone)]
pub struct ContextBound<'tree> {
    pub span: ::treesitter_types::Span,
    pub name: ::core::option::Option<ContextBoundName<'tree>>,
    pub r#type: ContextBoundType<'tree>,
}
impl<'tree> ::treesitter_types::FromNode<'tree> for ContextBound<'tree> {
    #[allow(clippy::match_single_binding, clippy::suspicious_else_formatting)]
    fn from_node(
        node: ::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        debug_assert_eq!(node.kind(), "context_bound");
        Ok(Self {
            span: ::treesitter_types::Span::from(node),
            name: match node.child_by_field_name("name") {
                Some(child) => {
                    Some(<ContextBoundName as ::treesitter_types::FromNode>::from_node(child, src)?)
                }
                None => None,
            },
            r#type: {
                let child = node
                    .child_by_field_name("type")
                    .ok_or_else(|| ::treesitter_types::ParseError::missing_field("type", node))?;
                <ContextBoundType as ::treesitter_types::FromNode>::from_node(child, src)?
            },
        })
    }
}
impl ::treesitter_types::Spanned for ContextBound<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        self.span
    }
}
#[derive(Debug, Clone)]
pub struct ContravariantTypeParameter<'tree> {
    pub span: ::treesitter_types::Span,
    pub bound: ::std::vec::Vec<ContravariantTypeParameterBound<'tree>>,
    pub name: ContravariantTypeParameterName<'tree>,
    pub type_parameters: ::core::option::Option<TypeParameters<'tree>>,
}
impl<'tree> ::treesitter_types::FromNode<'tree> for ContravariantTypeParameter<'tree> {
    #[allow(clippy::match_single_binding, clippy::suspicious_else_formatting)]
    fn from_node(
        node: ::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        debug_assert_eq!(node.kind(), "contravariant_type_parameter");
        Ok(Self {
            span: ::treesitter_types::Span::from(node),
            bound: {
                let mut cursor = node.walk();
                let mut items = ::std::vec::Vec::new();
                for child in node.children_by_field_name("bound", &mut cursor) {
                    items
                        .push(
                            <ContravariantTypeParameterBound as ::treesitter_types::FromNode>::from_node(
                                child,
                                src,
                            )?,
                        );
                }
                items
            },
            name: {
                let child = node
                    .child_by_field_name("name")
                    .ok_or_else(|| ::treesitter_types::ParseError::missing_field("name", node))?;
                <ContravariantTypeParameterName as ::treesitter_types::FromNode>::from_node(
                    child, src,
                )?
            },
            type_parameters: match node.child_by_field_name("type_parameters") {
                Some(child) => Some(<TypeParameters as ::treesitter_types::FromNode>::from_node(
                    child, src,
                )?),
                None => None,
            },
        })
    }
}
impl ::treesitter_types::Spanned for ContravariantTypeParameter<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        self.span
    }
}
#[derive(Debug, Clone)]
pub struct CovariantTypeParameter<'tree> {
    pub span: ::treesitter_types::Span,
    pub bound: ::std::vec::Vec<CovariantTypeParameterBound<'tree>>,
    pub name: CovariantTypeParameterName<'tree>,
    pub type_parameters: ::core::option::Option<TypeParameters<'tree>>,
}
impl<'tree> ::treesitter_types::FromNode<'tree> for CovariantTypeParameter<'tree> {
    #[allow(clippy::match_single_binding, clippy::suspicious_else_formatting)]
    fn from_node(
        node: ::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        debug_assert_eq!(node.kind(), "covariant_type_parameter");
        Ok(Self {
            span: ::treesitter_types::Span::from(node),
            bound: {
                let mut cursor = node.walk();
                let mut items = ::std::vec::Vec::new();
                for child in node.children_by_field_name("bound", &mut cursor) {
                    items.push(
                        <CovariantTypeParameterBound as ::treesitter_types::FromNode>::from_node(
                            child, src,
                        )?,
                    );
                }
                items
            },
            name: {
                let child = node
                    .child_by_field_name("name")
                    .ok_or_else(|| ::treesitter_types::ParseError::missing_field("name", node))?;
                <CovariantTypeParameterName as ::treesitter_types::FromNode>::from_node(child, src)?
            },
            type_parameters: match node.child_by_field_name("type_parameters") {
                Some(child) => Some(<TypeParameters as ::treesitter_types::FromNode>::from_node(
                    child, src,
                )?),
                None => None,
            },
        })
    }
}
impl ::treesitter_types::Spanned for CovariantTypeParameter<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        self.span
    }
}
#[derive(Debug, Clone)]
pub struct DerivesClause<'tree> {
    pub span: ::treesitter_types::Span,
    pub r#type: ::std::vec::Vec<DerivesClauseType<'tree>>,
}
impl<'tree> ::treesitter_types::FromNode<'tree> for DerivesClause<'tree> {
    #[allow(clippy::match_single_binding, clippy::suspicious_else_formatting)]
    fn from_node(
        node: ::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        debug_assert_eq!(node.kind(), "derives_clause");
        Ok(Self {
            span: ::treesitter_types::Span::from(node),
            r#type: {
                let mut cursor = node.walk();
                let mut items = ::std::vec::Vec::new();
                for child in node.children_by_field_name("type", &mut cursor) {
                    items.push(
                        <DerivesClauseType as ::treesitter_types::FromNode>::from_node(child, src)?,
                    );
                }
                items
            },
        })
    }
}
impl ::treesitter_types::Spanned for DerivesClause<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        self.span
    }
}
#[derive(Debug, Clone)]
pub struct DoWhileExpression<'tree> {
    pub span: ::treesitter_types::Span,
    pub body: Expression<'tree>,
    pub condition: ParenthesizedExpression<'tree>,
}
impl<'tree> ::treesitter_types::FromNode<'tree> for DoWhileExpression<'tree> {
    #[allow(clippy::match_single_binding, clippy::suspicious_else_formatting)]
    fn from_node(
        node: ::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        debug_assert_eq!(node.kind(), "do_while_expression");
        Ok(Self {
            span: ::treesitter_types::Span::from(node),
            body: {
                let child = node
                    .child_by_field_name("body")
                    .ok_or_else(|| ::treesitter_types::ParseError::missing_field("body", node))?;
                <Expression as ::treesitter_types::FromNode>::from_node(child, src)?
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
impl ::treesitter_types::Spanned for DoWhileExpression<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        self.span
    }
}
#[derive(Debug, Clone)]
pub struct EnumBody<'tree> {
    pub span: ::treesitter_types::Span,
    pub children: ::std::vec::Vec<EnumBodyChildren<'tree>>,
}
impl<'tree> ::treesitter_types::FromNode<'tree> for EnumBody<'tree> {
    #[allow(clippy::match_single_binding, clippy::suspicious_else_formatting)]
    fn from_node(
        node: ::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        debug_assert_eq!(node.kind(), "enum_body");
        Ok(Self {
            span: ::treesitter_types::Span::from(node),
            children: {
                #[allow(clippy::suspicious_else_formatting)]
                let non_field_children = {
                    let mut cursor = node.walk();
                    let mut result = ::std::vec::Vec::new();
                    if cursor.goto_first_child() {
                        loop {
                            if cursor.field_name().is_none()
                                && cursor.node().is_named()
                                && !cursor.node().is_extra()
                            {
                                result.push(cursor.node());
                            }
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
                        <EnumBodyChildren as ::treesitter_types::FromNode>::from_node(child, src)?,
                    );
                }
                items
            },
        })
    }
}
impl ::treesitter_types::Spanned for EnumBody<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        self.span
    }
}
#[derive(Debug, Clone)]
pub struct EnumCaseDefinitions<'tree> {
    pub span: ::treesitter_types::Span,
    pub children: ::std::vec::Vec<EnumCaseDefinitionsChildren<'tree>>,
}
impl<'tree> ::treesitter_types::FromNode<'tree> for EnumCaseDefinitions<'tree> {
    #[allow(clippy::match_single_binding, clippy::suspicious_else_formatting)]
    fn from_node(
        node: ::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        debug_assert_eq!(node.kind(), "enum_case_definitions");
        Ok(Self {
            span: ::treesitter_types::Span::from(node),
            children: {
                #[allow(clippy::suspicious_else_formatting)]
                let non_field_children = {
                    let mut cursor = node.walk();
                    let mut result = ::std::vec::Vec::new();
                    if cursor.goto_first_child() {
                        loop {
                            if cursor.field_name().is_none()
                                && cursor.node().is_named()
                                && !cursor.node().is_extra()
                            {
                                result.push(cursor.node());
                            }
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
                        <EnumCaseDefinitionsChildren as ::treesitter_types::FromNode>::from_node(
                            child, src,
                        )?,
                    );
                }
                items
            },
        })
    }
}
impl ::treesitter_types::Spanned for EnumCaseDefinitions<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        self.span
    }
}
#[derive(Debug, Clone)]
pub struct EnumDefinition<'tree> {
    pub span: ::treesitter_types::Span,
    pub body: EnumBody<'tree>,
    pub class_parameters: ::std::vec::Vec<ClassParameters<'tree>>,
    pub derive: ::core::option::Option<DerivesClause<'tree>>,
    pub extend: ::core::option::Option<ExtendsClause<'tree>>,
    pub name: EnumDefinitionName<'tree>,
    pub type_parameters: ::core::option::Option<TypeParameters<'tree>>,
    pub children: ::std::vec::Vec<EnumDefinitionChildren<'tree>>,
}
impl<'tree> ::treesitter_types::FromNode<'tree> for EnumDefinition<'tree> {
    #[allow(clippy::match_single_binding, clippy::suspicious_else_formatting)]
    fn from_node(
        node: ::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        debug_assert_eq!(node.kind(), "enum_definition");
        Ok(Self {
            span: ::treesitter_types::Span::from(node),
            body: {
                let child = node
                    .child_by_field_name("body")
                    .ok_or_else(|| ::treesitter_types::ParseError::missing_field("body", node))?;
                <EnumBody as ::treesitter_types::FromNode>::from_node(child, src)?
            },
            class_parameters: {
                let mut cursor = node.walk();
                let mut items = ::std::vec::Vec::new();
                for child in node.children_by_field_name("class_parameters", &mut cursor) {
                    items.push(
                        <ClassParameters as ::treesitter_types::FromNode>::from_node(child, src)?,
                    );
                }
                items
            },
            derive: match node.child_by_field_name("derive") {
                Some(child) => Some(<DerivesClause as ::treesitter_types::FromNode>::from_node(
                    child, src,
                )?),
                None => None,
            },
            extend: match node.child_by_field_name("extend") {
                Some(child) => Some(<ExtendsClause as ::treesitter_types::FromNode>::from_node(
                    child, src,
                )?),
                None => None,
            },
            name: {
                let child = node
                    .child_by_field_name("name")
                    .ok_or_else(|| ::treesitter_types::ParseError::missing_field("name", node))?;
                <EnumDefinitionName as ::treesitter_types::FromNode>::from_node(child, src)?
            },
            type_parameters: match node.child_by_field_name("type_parameters") {
                Some(child) => Some(<TypeParameters as ::treesitter_types::FromNode>::from_node(
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
                        <EnumDefinitionChildren as ::treesitter_types::FromNode>::from_node(
                            child, src,
                        )?,
                    );
                }
                items
            },
        })
    }
}
impl ::treesitter_types::Spanned for EnumDefinition<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        self.span
    }
}
#[derive(Debug, Clone)]
pub struct Enumerator<'tree> {
    pub span: ::treesitter_types::Span,
    pub children: ::std::vec::Vec<EnumeratorChildren<'tree>>,
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
            children: {
                #[allow(clippy::suspicious_else_formatting)]
                let non_field_children = {
                    let mut cursor = node.walk();
                    let mut result = ::std::vec::Vec::new();
                    if cursor.goto_first_child() {
                        loop {
                            if cursor.field_name().is_none()
                                && cursor.node().is_named()
                                && !cursor.node().is_extra()
                            {
                                result.push(cursor.node());
                            }
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
                        <EnumeratorChildren as ::treesitter_types::FromNode>::from_node(
                            child, src,
                        )?,
                    );
                }
                items
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
pub struct Enumerators<'tree> {
    pub span: ::treesitter_types::Span,
    pub children: ::std::vec::Vec<Enumerator<'tree>>,
}
impl<'tree> ::treesitter_types::FromNode<'tree> for Enumerators<'tree> {
    #[allow(clippy::match_single_binding, clippy::suspicious_else_formatting)]
    fn from_node(
        node: ::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        debug_assert_eq!(node.kind(), "enumerators");
        Ok(Self {
            span: ::treesitter_types::Span::from(node),
            children: {
                #[allow(clippy::suspicious_else_formatting)]
                let non_field_children = {
                    let mut cursor = node.walk();
                    let mut result = ::std::vec::Vec::new();
                    if cursor.goto_first_child() {
                        loop {
                            if cursor.field_name().is_none()
                                && cursor.node().is_named()
                                && !cursor.node().is_extra()
                            {
                                result.push(cursor.node());
                            }
                            if !cursor.goto_next_sibling() {
                                break;
                            }
                        }
                    }
                    result
                };
                let mut items = ::std::vec::Vec::new();
                for child in non_field_children {
                    items.push(<Enumerator as ::treesitter_types::FromNode>::from_node(
                        child, src,
                    )?);
                }
                items
            },
        })
    }
}
impl ::treesitter_types::Spanned for Enumerators<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        self.span
    }
}
#[derive(Debug, Clone)]
pub struct ExportDeclaration<'tree> {
    pub span: ::treesitter_types::Span,
    pub path: ::std::vec::Vec<ExportDeclarationPath<'tree>>,
    pub children: ::std::vec::Vec<ExportDeclarationChildren<'tree>>,
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
            path: {
                let mut cursor = node.walk();
                let mut items = ::std::vec::Vec::new();
                for child in node.children_by_field_name("path", &mut cursor) {
                    items.push(
                        <ExportDeclarationPath as ::treesitter_types::FromNode>::from_node(
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
                        <ExportDeclarationChildren as ::treesitter_types::FromNode>::from_node(
                            child, src,
                        )?,
                    );
                }
                items
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
pub struct ExtendsClause<'tree> {
    pub span: ::treesitter_types::Span,
    pub arguments: ::std::vec::Vec<Arguments<'tree>>,
    pub r#type: ::std::vec::Vec<ExtendsClauseType<'tree>>,
    pub children: ::core::option::Option<Arguments<'tree>>,
}
impl<'tree> ::treesitter_types::FromNode<'tree> for ExtendsClause<'tree> {
    #[allow(clippy::match_single_binding, clippy::suspicious_else_formatting)]
    fn from_node(
        node: ::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        debug_assert_eq!(node.kind(), "extends_clause");
        Ok(Self {
            span: ::treesitter_types::Span::from(node),
            arguments: {
                let mut cursor = node.walk();
                let mut items = ::std::vec::Vec::new();
                for child in node.children_by_field_name("arguments", &mut cursor) {
                    items.push(<Arguments as ::treesitter_types::FromNode>::from_node(
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
                        <ExtendsClauseType as ::treesitter_types::FromNode>::from_node(child, src)?,
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
                    Some(&child) => Some(<Arguments as ::treesitter_types::FromNode>::from_node(
                        child, src,
                    )?),
                    None => None,
                }
            },
        })
    }
}
impl ::treesitter_types::Spanned for ExtendsClause<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        self.span
    }
}
#[derive(Debug, Clone)]
pub struct ExtensionDefinition<'tree> {
    pub span: ::treesitter_types::Span,
    pub body: ::std::vec::Vec<ExtensionDefinitionBody<'tree>>,
    pub parameters: ::std::vec::Vec<Parameters<'tree>>,
    pub type_parameters: ::core::option::Option<TypeParameters<'tree>>,
}
impl<'tree> ::treesitter_types::FromNode<'tree> for ExtensionDefinition<'tree> {
    #[allow(clippy::match_single_binding, clippy::suspicious_else_formatting)]
    fn from_node(
        node: ::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        debug_assert_eq!(node.kind(), "extension_definition");
        Ok(Self {
            span: ::treesitter_types::Span::from(node),
            body: {
                let mut cursor = node.walk();
                let mut items = ::std::vec::Vec::new();
                for child in node.children_by_field_name("body", &mut cursor) {
                    items.push(
                        <ExtensionDefinitionBody as ::treesitter_types::FromNode>::from_node(
                            child, src,
                        )?,
                    );
                }
                items
            },
            parameters: {
                let mut cursor = node.walk();
                let mut items = ::std::vec::Vec::new();
                for child in node.children_by_field_name("parameters", &mut cursor) {
                    items.push(<Parameters as ::treesitter_types::FromNode>::from_node(
                        child, src,
                    )?);
                }
                items
            },
            type_parameters: match node.child_by_field_name("type_parameters") {
                Some(child) => Some(<TypeParameters as ::treesitter_types::FromNode>::from_node(
                    child, src,
                )?),
                None => None,
            },
        })
    }
}
impl ::treesitter_types::Spanned for ExtensionDefinition<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        self.span
    }
}
#[derive(Debug, Clone)]
pub struct FieldExpression<'tree> {
    pub span: ::treesitter_types::Span,
    pub field: FieldExpressionField<'tree>,
    pub value: FieldExpressionValue<'tree>,
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
            field: {
                let child = node
                    .child_by_field_name("field")
                    .ok_or_else(|| ::treesitter_types::ParseError::missing_field("field", node))?;
                <FieldExpressionField as ::treesitter_types::FromNode>::from_node(child, src)?
            },
            value: {
                let child = node
                    .child_by_field_name("value")
                    .ok_or_else(|| ::treesitter_types::ParseError::missing_field("value", node))?;
                <FieldExpressionValue as ::treesitter_types::FromNode>::from_node(child, src)?
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
pub struct FinallyClause<'tree> {
    pub span: ::treesitter_types::Span,
    pub children: FinallyClauseChildren<'tree>,
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
                                let candidate = fallback_cursor.node();
                                #[allow(clippy::needless_question_mark)]
                                if (|| -> ::core::result::Result<
                                    _,
                                    ::treesitter_types::ParseError,
                                > {
                                    let child = candidate;
                                    Ok(
                                        <FinallyClauseChildren as ::treesitter_types::FromNode>::from_node(
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
                                            <FinallyClauseChildren as ::treesitter_types::FromNode>::from_node(
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
                <FinallyClauseChildren as ::treesitter_types::FromNode>::from_node(child, src)?
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
pub struct ForExpression<'tree> {
    pub span: ::treesitter_types::Span,
    pub body: ForExpressionBody<'tree>,
    pub enumerators: ::std::vec::Vec<ForExpressionEnumerators<'tree>>,
}
impl<'tree> ::treesitter_types::FromNode<'tree> for ForExpression<'tree> {
    #[allow(clippy::match_single_binding, clippy::suspicious_else_formatting)]
    fn from_node(
        node: ::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        debug_assert_eq!(node.kind(), "for_expression");
        Ok(Self {
            span: ::treesitter_types::Span::from(node),
            body: {
                let child = node
                    .child_by_field_name("body")
                    .ok_or_else(|| ::treesitter_types::ParseError::missing_field("body", node))?;
                <ForExpressionBody as ::treesitter_types::FromNode>::from_node(child, src)?
            },
            enumerators: {
                let mut cursor = node.walk();
                let mut items = ::std::vec::Vec::new();
                for child in node.children_by_field_name("enumerators", &mut cursor) {
                    items.push(
                        <ForExpressionEnumerators as ::treesitter_types::FromNode>::from_node(
                            child, src,
                        )?,
                    );
                }
                items
            },
        })
    }
}
impl ::treesitter_types::Spanned for ForExpression<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        self.span
    }
}
#[derive(Debug, Clone)]
pub struct FullEnumCase<'tree> {
    pub span: ::treesitter_types::Span,
    pub class_parameters: ::std::vec::Vec<ClassParameters<'tree>>,
    pub extend: ::core::option::Option<ExtendsClause<'tree>>,
    pub name: FullEnumCaseName<'tree>,
    pub type_parameters: ::core::option::Option<TypeParameters<'tree>>,
}
impl<'tree> ::treesitter_types::FromNode<'tree> for FullEnumCase<'tree> {
    #[allow(clippy::match_single_binding, clippy::suspicious_else_formatting)]
    fn from_node(
        node: ::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        debug_assert_eq!(node.kind(), "full_enum_case");
        Ok(Self {
            span: ::treesitter_types::Span::from(node),
            class_parameters: {
                let mut cursor = node.walk();
                let mut items = ::std::vec::Vec::new();
                for child in node.children_by_field_name("class_parameters", &mut cursor) {
                    items.push(
                        <ClassParameters as ::treesitter_types::FromNode>::from_node(child, src)?,
                    );
                }
                items
            },
            extend: match node.child_by_field_name("extend") {
                Some(child) => Some(<ExtendsClause as ::treesitter_types::FromNode>::from_node(
                    child, src,
                )?),
                None => None,
            },
            name: {
                let child = node
                    .child_by_field_name("name")
                    .ok_or_else(|| ::treesitter_types::ParseError::missing_field("name", node))?;
                <FullEnumCaseName as ::treesitter_types::FromNode>::from_node(child, src)?
            },
            type_parameters: match node.child_by_field_name("type_parameters") {
                Some(child) => Some(<TypeParameters as ::treesitter_types::FromNode>::from_node(
                    child, src,
                )?),
                None => None,
            },
        })
    }
}
impl ::treesitter_types::Spanned for FullEnumCase<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        self.span
    }
}
#[derive(Debug, Clone)]
pub struct FunctionDeclaration<'tree> {
    pub span: ::treesitter_types::Span,
    pub name: FunctionDeclarationName<'tree>,
    pub parameters: ::std::vec::Vec<FunctionDeclarationParameters<'tree>>,
    pub return_type: ::core::option::Option<FunctionDeclarationReturnType<'tree>>,
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
            parameters: {
                let mut cursor = node.walk();
                let mut items = ::std::vec::Vec::new();
                for child in node.children_by_field_name("parameters", &mut cursor) {
                    items.push(
                        <FunctionDeclarationParameters as ::treesitter_types::FromNode>::from_node(
                            child, src,
                        )?,
                    );
                }
                items
            },
            return_type: match node.child_by_field_name("return_type") {
                Some(child) => Some(
                    <FunctionDeclarationReturnType as ::treesitter_types::FromNode>::from_node(
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
pub struct FunctionDefinition<'tree> {
    pub span: ::treesitter_types::Span,
    pub body: FunctionDefinitionBody<'tree>,
    pub name: FunctionDefinitionName<'tree>,
    pub parameters: ::std::vec::Vec<FunctionDefinitionParameters<'tree>>,
    pub return_type: ::core::option::Option<FunctionDefinitionReturnType<'tree>>,
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
                <FunctionDefinitionName as ::treesitter_types::FromNode>::from_node(child, src)?
            },
            parameters: {
                let mut cursor = node.walk();
                let mut items = ::std::vec::Vec::new();
                for child in node.children_by_field_name("parameters", &mut cursor) {
                    items.push(
                        <FunctionDefinitionParameters as ::treesitter_types::FromNode>::from_node(
                            child, src,
                        )?,
                    );
                }
                items
            },
            return_type: match node.child_by_field_name("return_type") {
                Some(child) => Some(
                    <FunctionDefinitionReturnType as ::treesitter_types::FromNode>::from_node(
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
pub struct FunctionType<'tree> {
    pub span: ::treesitter_types::Span,
    pub parameter_types: ::core::option::Option<ParameterTypes<'tree>>,
    pub return_type: FunctionTypeReturnType<'tree>,
    pub type_parameters: ::core::option::Option<TypeParameters<'tree>>,
}
impl<'tree> ::treesitter_types::FromNode<'tree> for FunctionType<'tree> {
    #[allow(clippy::match_single_binding, clippy::suspicious_else_formatting)]
    fn from_node(
        node: ::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        debug_assert_eq!(node.kind(), "function_type");
        Ok(Self {
            span: ::treesitter_types::Span::from(node),
            parameter_types: match node.child_by_field_name("parameter_types") {
                Some(child) => Some(<ParameterTypes as ::treesitter_types::FromNode>::from_node(
                    child, src,
                )?),
                None => None,
            },
            return_type: {
                let child = node.child_by_field_name("return_type").ok_or_else(|| {
                    ::treesitter_types::ParseError::missing_field("return_type", node)
                })?;
                <FunctionTypeReturnType as ::treesitter_types::FromNode>::from_node(child, src)?
            },
            type_parameters: match node.child_by_field_name("type_parameters") {
                Some(child) => Some(<TypeParameters as ::treesitter_types::FromNode>::from_node(
                    child, src,
                )?),
                None => None,
            },
        })
    }
}
impl ::treesitter_types::Spanned for FunctionType<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        self.span
    }
}
#[derive(Debug, Clone)]
pub struct GenericFunction<'tree> {
    pub span: ::treesitter_types::Span,
    pub function: Expression<'tree>,
    pub type_arguments: TypeArguments<'tree>,
}
impl<'tree> ::treesitter_types::FromNode<'tree> for GenericFunction<'tree> {
    #[allow(clippy::match_single_binding, clippy::suspicious_else_formatting)]
    fn from_node(
        node: ::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        debug_assert_eq!(node.kind(), "generic_function");
        Ok(Self {
            span: ::treesitter_types::Span::from(node),
            function: {
                let child = node.child_by_field_name("function").ok_or_else(|| {
                    ::treesitter_types::ParseError::missing_field("function", node)
                })?;
                <Expression as ::treesitter_types::FromNode>::from_node(child, src)?
            },
            type_arguments: {
                let child = node.child_by_field_name("type_arguments").ok_or_else(|| {
                    ::treesitter_types::ParseError::missing_field("type_arguments", node)
                })?;
                <TypeArguments as ::treesitter_types::FromNode>::from_node(child, src)?
            },
        })
    }
}
impl ::treesitter_types::Spanned for GenericFunction<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        self.span
    }
}
#[derive(Debug, Clone)]
pub struct GenericType<'tree> {
    pub span: ::treesitter_types::Span,
    pub r#type: GenericTypeType<'tree>,
    pub type_arguments: TypeArguments<'tree>,
}
impl<'tree> ::treesitter_types::FromNode<'tree> for GenericType<'tree> {
    #[allow(clippy::match_single_binding, clippy::suspicious_else_formatting)]
    fn from_node(
        node: ::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        debug_assert_eq!(node.kind(), "generic_type");
        Ok(Self {
            span: ::treesitter_types::Span::from(node),
            r#type: {
                let child = node
                    .child_by_field_name("type")
                    .ok_or_else(|| ::treesitter_types::ParseError::missing_field("type", node))?;
                <GenericTypeType as ::treesitter_types::FromNode>::from_node(child, src)?
            },
            type_arguments: {
                let child = node.child_by_field_name("type_arguments").ok_or_else(|| {
                    ::treesitter_types::ParseError::missing_field("type_arguments", node)
                })?;
                <TypeArguments as ::treesitter_types::FromNode>::from_node(child, src)?
            },
        })
    }
}
impl ::treesitter_types::Spanned for GenericType<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        self.span
    }
}
#[derive(Debug, Clone)]
pub struct GivenConditional<'tree> {
    pub span: ::treesitter_types::Span,
    pub children: ::std::vec::Vec<GivenConditionalChildren<'tree>>,
}
impl<'tree> ::treesitter_types::FromNode<'tree> for GivenConditional<'tree> {
    #[allow(clippy::match_single_binding, clippy::suspicious_else_formatting)]
    fn from_node(
        node: ::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        debug_assert_eq!(node.kind(), "given_conditional");
        Ok(Self {
            span: ::treesitter_types::Span::from(node),
            children: {
                #[allow(clippy::suspicious_else_formatting)]
                let non_field_children = {
                    let mut cursor = node.walk();
                    let mut result = ::std::vec::Vec::new();
                    if cursor.goto_first_child() {
                        loop {
                            if cursor.field_name().is_none()
                                && cursor.node().is_named()
                                && !cursor.node().is_extra()
                            {
                                result.push(cursor.node());
                            }
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
                        <GivenConditionalChildren as ::treesitter_types::FromNode>::from_node(
                            child, src,
                        )?,
                    );
                }
                items
            },
        })
    }
}
impl ::treesitter_types::Spanned for GivenConditional<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        self.span
    }
}
#[derive(Debug, Clone)]
pub struct GivenDefinition<'tree> {
    pub span: ::treesitter_types::Span,
    pub arguments: ::core::option::Option<Arguments<'tree>>,
    pub body: ::core::option::Option<GivenDefinitionBody<'tree>>,
    pub name: ::core::option::Option<GivenDefinitionName<'tree>>,
    pub parameters: ::std::vec::Vec<Parameters<'tree>>,
    pub return_type: ::std::vec::Vec<GivenDefinitionReturnType<'tree>>,
    pub type_parameters: ::core::option::Option<TypeParameters<'tree>>,
    pub children: ::std::vec::Vec<GivenDefinitionChildren<'tree>>,
}
impl<'tree> ::treesitter_types::FromNode<'tree> for GivenDefinition<'tree> {
    #[allow(clippy::match_single_binding, clippy::suspicious_else_formatting)]
    fn from_node(
        node: ::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        debug_assert_eq!(node.kind(), "given_definition");
        Ok(Self {
            span: ::treesitter_types::Span::from(node),
            arguments: match node.child_by_field_name("arguments") {
                Some(child) => Some(<Arguments as ::treesitter_types::FromNode>::from_node(
                    child, src,
                )?),
                None => None,
            },
            body: match node.child_by_field_name("body") {
                Some(child) => Some(
                    <GivenDefinitionBody as ::treesitter_types::FromNode>::from_node(child, src)?,
                ),
                None => None,
            },
            name: match node.child_by_field_name("name") {
                Some(child) => Some(
                    <GivenDefinitionName as ::treesitter_types::FromNode>::from_node(child, src)?,
                ),
                None => None,
            },
            parameters: {
                let mut cursor = node.walk();
                let mut items = ::std::vec::Vec::new();
                for child in node.children_by_field_name("parameters", &mut cursor) {
                    items.push(<Parameters as ::treesitter_types::FromNode>::from_node(
                        child, src,
                    )?);
                }
                items
            },
            return_type: {
                let mut cursor = node.walk();
                let mut items = ::std::vec::Vec::new();
                for child in node.children_by_field_name("return_type", &mut cursor) {
                    items.push(
                        <GivenDefinitionReturnType as ::treesitter_types::FromNode>::from_node(
                            child, src,
                        )?,
                    );
                }
                items
            },
            type_parameters: match node.child_by_field_name("type_parameters") {
                Some(child) => Some(<TypeParameters as ::treesitter_types::FromNode>::from_node(
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
                        <GivenDefinitionChildren as ::treesitter_types::FromNode>::from_node(
                            child, src,
                        )?,
                    );
                }
                items
            },
        })
    }
}
impl ::treesitter_types::Spanned for GivenDefinition<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        self.span
    }
}
#[derive(Debug, Clone)]
pub struct GivenPattern<'tree> {
    pub span: ::treesitter_types::Span,
    pub r#type: GivenPatternType<'tree>,
}
impl<'tree> ::treesitter_types::FromNode<'tree> for GivenPattern<'tree> {
    #[allow(clippy::match_single_binding, clippy::suspicious_else_formatting)]
    fn from_node(
        node: ::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        debug_assert_eq!(node.kind(), "given_pattern");
        Ok(Self {
            span: ::treesitter_types::Span::from(node),
            r#type: {
                let child = node
                    .child_by_field_name("type")
                    .ok_or_else(|| ::treesitter_types::ParseError::missing_field("type", node))?;
                <GivenPatternType as ::treesitter_types::FromNode>::from_node(child, src)?
            },
        })
    }
}
impl ::treesitter_types::Spanned for GivenPattern<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        self.span
    }
}
#[derive(Debug, Clone)]
pub struct Guard<'tree> {
    pub span: ::treesitter_types::Span,
    pub condition: GuardCondition<'tree>,
}
impl<'tree> ::treesitter_types::FromNode<'tree> for Guard<'tree> {
    #[allow(clippy::match_single_binding, clippy::suspicious_else_formatting)]
    fn from_node(
        node: ::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        debug_assert_eq!(node.kind(), "guard");
        Ok(Self {
            span: ::treesitter_types::Span::from(node),
            condition: {
                let child = node.child_by_field_name("condition").ok_or_else(|| {
                    ::treesitter_types::ParseError::missing_field("condition", node)
                })?;
                <GuardCondition as ::treesitter_types::FromNode>::from_node(child, src)?
            },
        })
    }
}
impl ::treesitter_types::Spanned for Guard<'_> {
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
pub struct Identifiers<'tree> {
    pub span: ::treesitter_types::Span,
    pub children: ::std::vec::Vec<Identifier<'tree>>,
}
impl<'tree> ::treesitter_types::FromNode<'tree> for Identifiers<'tree> {
    #[allow(clippy::match_single_binding, clippy::suspicious_else_formatting)]
    fn from_node(
        node: ::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        debug_assert_eq!(node.kind(), "identifiers");
        Ok(Self {
            span: ::treesitter_types::Span::from(node),
            children: {
                #[allow(clippy::suspicious_else_formatting)]
                let non_field_children = {
                    let mut cursor = node.walk();
                    let mut result = ::std::vec::Vec::new();
                    if cursor.goto_first_child() {
                        loop {
                            if cursor.field_name().is_none()
                                && cursor.node().is_named()
                                && !cursor.node().is_extra()
                            {
                                result.push(cursor.node());
                            }
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
impl ::treesitter_types::Spanned for Identifiers<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        self.span
    }
}
#[derive(Debug, Clone)]
pub struct IfExpression<'tree> {
    pub span: ::treesitter_types::Span,
    pub alternative: ::core::option::Option<IfExpressionAlternative<'tree>>,
    pub condition: ::std::vec::Vec<IfExpressionCondition<'tree>>,
    pub consequence: IfExpressionConsequence<'tree>,
    pub children: ::core::option::Option<InlineModifier<'tree>>,
}
impl<'tree> ::treesitter_types::FromNode<'tree> for IfExpression<'tree> {
    #[allow(clippy::match_single_binding, clippy::suspicious_else_formatting)]
    fn from_node(
        node: ::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        debug_assert_eq!(node.kind(), "if_expression");
        Ok(Self {
            span: ::treesitter_types::Span::from(node),
            alternative: match node.child_by_field_name("alternative") {
                Some(child) => Some(
                    <IfExpressionAlternative as ::treesitter_types::FromNode>::from_node(
                        child, src,
                    )?,
                ),
                None => None,
            },
            condition: {
                let mut cursor = node.walk();
                let mut items = ::std::vec::Vec::new();
                for child in node.children_by_field_name("condition", &mut cursor) {
                    items.push(
                        <IfExpressionCondition as ::treesitter_types::FromNode>::from_node(
                            child, src,
                        )?,
                    );
                }
                items
            },
            consequence: {
                let child = node.child_by_field_name("consequence").ok_or_else(|| {
                    ::treesitter_types::ParseError::missing_field("consequence", node)
                })?;
                <IfExpressionConsequence as ::treesitter_types::FromNode>::from_node(child, src)?
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
                        <InlineModifier as ::treesitter_types::FromNode>::from_node(child, src)?,
                    ),
                    None => None,
                }
            },
        })
    }
}
impl ::treesitter_types::Spanned for IfExpression<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        self.span
    }
}
#[derive(Debug, Clone)]
pub struct ImportDeclaration<'tree> {
    pub span: ::treesitter_types::Span,
    pub path: ::std::vec::Vec<ImportDeclarationPath<'tree>>,
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
            path: {
                let mut cursor = node.walk();
                let mut items = ::std::vec::Vec::new();
                for child in node.children_by_field_name("path", &mut cursor) {
                    items.push(
                        <ImportDeclarationPath as ::treesitter_types::FromNode>::from_node(
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
pub struct IndentedBlock<'tree> {
    pub span: ::treesitter_types::Span,
    pub children: ::std::vec::Vec<IndentedBlockChildren<'tree>>,
}
impl<'tree> ::treesitter_types::FromNode<'tree> for IndentedBlock<'tree> {
    #[allow(clippy::match_single_binding, clippy::suspicious_else_formatting)]
    fn from_node(
        node: ::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        debug_assert_eq!(node.kind(), "indented_block");
        Ok(Self {
            span: ::treesitter_types::Span::from(node),
            children: {
                #[allow(clippy::suspicious_else_formatting)]
                let non_field_children = {
                    let mut cursor = node.walk();
                    let mut result = ::std::vec::Vec::new();
                    if cursor.goto_first_child() {
                        loop {
                            if cursor.field_name().is_none()
                                && cursor.node().is_named()
                                && !cursor.node().is_extra()
                            {
                                result.push(cursor.node());
                            }
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
                        <IndentedBlockChildren as ::treesitter_types::FromNode>::from_node(
                            child, src,
                        )?,
                    );
                }
                items
            },
        })
    }
}
impl ::treesitter_types::Spanned for IndentedBlock<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        self.span
    }
}
#[derive(Debug, Clone)]
pub struct IndentedCases<'tree> {
    pub span: ::treesitter_types::Span,
    pub children: ::std::vec::Vec<CaseClause<'tree>>,
}
impl<'tree> ::treesitter_types::FromNode<'tree> for IndentedCases<'tree> {
    #[allow(clippy::match_single_binding, clippy::suspicious_else_formatting)]
    fn from_node(
        node: ::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        debug_assert_eq!(node.kind(), "indented_cases");
        Ok(Self {
            span: ::treesitter_types::Span::from(node),
            children: {
                #[allow(clippy::suspicious_else_formatting)]
                let non_field_children = {
                    let mut cursor = node.walk();
                    let mut result = ::std::vec::Vec::new();
                    if cursor.goto_first_child() {
                        loop {
                            if cursor.field_name().is_none()
                                && cursor.node().is_named()
                                && !cursor.node().is_extra()
                            {
                                result.push(cursor.node());
                            }
                            if !cursor.goto_next_sibling() {
                                break;
                            }
                        }
                    }
                    result
                };
                let mut items = ::std::vec::Vec::new();
                for child in non_field_children {
                    items.push(<CaseClause as ::treesitter_types::FromNode>::from_node(
                        child, src,
                    )?);
                }
                items
            },
        })
    }
}
impl ::treesitter_types::Spanned for IndentedCases<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        self.span
    }
}
#[derive(Debug, Clone)]
pub struct InfixExpression<'tree> {
    pub span: ::treesitter_types::Span,
    pub left: InfixExpressionLeft<'tree>,
    pub operator: InfixExpressionOperator<'tree>,
    pub right: ::std::vec::Vec<InfixExpressionRight<'tree>>,
}
impl<'tree> ::treesitter_types::FromNode<'tree> for InfixExpression<'tree> {
    #[allow(clippy::match_single_binding, clippy::suspicious_else_formatting)]
    fn from_node(
        node: ::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        debug_assert_eq!(node.kind(), "infix_expression");
        Ok(Self {
            span: ::treesitter_types::Span::from(node),
            left: {
                let child = node
                    .child_by_field_name("left")
                    .ok_or_else(|| ::treesitter_types::ParseError::missing_field("left", node))?;
                <InfixExpressionLeft as ::treesitter_types::FromNode>::from_node(child, src)?
            },
            operator: {
                let child = node.child_by_field_name("operator").ok_or_else(|| {
                    ::treesitter_types::ParseError::missing_field("operator", node)
                })?;
                <InfixExpressionOperator as ::treesitter_types::FromNode>::from_node(child, src)?
            },
            right: {
                let mut cursor = node.walk();
                let mut items = ::std::vec::Vec::new();
                for child in node.children_by_field_name("right", &mut cursor) {
                    items.push(
                        <InfixExpressionRight as ::treesitter_types::FromNode>::from_node(
                            child, src,
                        )?,
                    );
                }
                items
            },
        })
    }
}
impl ::treesitter_types::Spanned for InfixExpression<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        self.span
    }
}
#[derive(Debug, Clone)]
pub struct InfixModifier<'tree> {
    pub span: ::treesitter_types::Span,
    text: &'tree str,
}
impl<'tree> ::treesitter_types::FromNode<'tree> for InfixModifier<'tree> {
    fn from_node(
        node: ::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        debug_assert_eq!(node.kind(), "infix_modifier");
        Ok(Self {
            span: ::treesitter_types::Span::from(node),
            text: node.utf8_text(src)?,
        })
    }
}
impl<'tree> ::treesitter_types::LeafNode<'tree> for InfixModifier<'tree> {
    fn text(&self) -> &'tree str {
        self.text
    }
}
impl ::treesitter_types::Spanned for InfixModifier<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        self.span
    }
}
#[derive(Debug, Clone)]
pub struct InfixPattern<'tree> {
    pub span: ::treesitter_types::Span,
    pub left: Pattern<'tree>,
    pub operator: InfixPatternOperator<'tree>,
    pub right: Pattern<'tree>,
}
impl<'tree> ::treesitter_types::FromNode<'tree> for InfixPattern<'tree> {
    #[allow(clippy::match_single_binding, clippy::suspicious_else_formatting)]
    fn from_node(
        node: ::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        debug_assert_eq!(node.kind(), "infix_pattern");
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
                <InfixPatternOperator as ::treesitter_types::FromNode>::from_node(child, src)?
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
impl ::treesitter_types::Spanned for InfixPattern<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        self.span
    }
}
#[derive(Debug, Clone)]
pub struct InfixType<'tree> {
    pub span: ::treesitter_types::Span,
    pub left: InfixTypeLeft<'tree>,
    pub operator: InfixTypeOperator<'tree>,
    pub right: InfixTypeRight<'tree>,
}
impl<'tree> ::treesitter_types::FromNode<'tree> for InfixType<'tree> {
    #[allow(clippy::match_single_binding, clippy::suspicious_else_formatting)]
    fn from_node(
        node: ::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        debug_assert_eq!(node.kind(), "infix_type");
        Ok(Self {
            span: ::treesitter_types::Span::from(node),
            left: {
                let child = node
                    .child_by_field_name("left")
                    .ok_or_else(|| ::treesitter_types::ParseError::missing_field("left", node))?;
                <InfixTypeLeft as ::treesitter_types::FromNode>::from_node(child, src)?
            },
            operator: {
                let child = node.child_by_field_name("operator").ok_or_else(|| {
                    ::treesitter_types::ParseError::missing_field("operator", node)
                })?;
                <InfixTypeOperator as ::treesitter_types::FromNode>::from_node(child, src)?
            },
            right: {
                let child = node
                    .child_by_field_name("right")
                    .ok_or_else(|| ::treesitter_types::ParseError::missing_field("right", node))?;
                <InfixTypeRight as ::treesitter_types::FromNode>::from_node(child, src)?
            },
        })
    }
}
impl ::treesitter_types::Spanned for InfixType<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        self.span
    }
}
#[derive(Debug, Clone)]
pub struct InlineModifier<'tree> {
    pub span: ::treesitter_types::Span,
    text: &'tree str,
}
impl<'tree> ::treesitter_types::FromNode<'tree> for InlineModifier<'tree> {
    fn from_node(
        node: ::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        debug_assert_eq!(node.kind(), "inline_modifier");
        Ok(Self {
            span: ::treesitter_types::Span::from(node),
            text: node.utf8_text(src)?,
        })
    }
}
impl<'tree> ::treesitter_types::LeafNode<'tree> for InlineModifier<'tree> {
    fn text(&self) -> &'tree str {
        self.text
    }
}
impl ::treesitter_types::Spanned for InlineModifier<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        self.span
    }
}
#[derive(Debug, Clone)]
pub struct InstanceExpression<'tree> {
    pub span: ::treesitter_types::Span,
    pub arguments: ::core::option::Option<Arguments<'tree>>,
    pub children: ::std::vec::Vec<InstanceExpressionChildren<'tree>>,
}
impl<'tree> ::treesitter_types::FromNode<'tree> for InstanceExpression<'tree> {
    #[allow(clippy::match_single_binding, clippy::suspicious_else_formatting)]
    fn from_node(
        node: ::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        debug_assert_eq!(node.kind(), "instance_expression");
        Ok(Self {
            span: ::treesitter_types::Span::from(node),
            arguments: match node.child_by_field_name("arguments") {
                Some(child) => Some(<Arguments as ::treesitter_types::FromNode>::from_node(
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
                        <InstanceExpressionChildren as ::treesitter_types::FromNode>::from_node(
                            child, src,
                        )?,
                    );
                }
                items
            },
        })
    }
}
impl ::treesitter_types::Spanned for InstanceExpression<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        self.span
    }
}
#[derive(Debug, Clone)]
pub struct InterpolatedString<'tree> {
    pub span: ::treesitter_types::Span,
    pub children: ::std::vec::Vec<InterpolatedStringChildren<'tree>>,
}
impl<'tree> ::treesitter_types::FromNode<'tree> for InterpolatedString<'tree> {
    #[allow(clippy::match_single_binding, clippy::suspicious_else_formatting)]
    fn from_node(
        node: ::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        debug_assert_eq!(node.kind(), "interpolated_string");
        Ok(Self {
            span: ::treesitter_types::Span::from(node),
            children: {
                #[allow(clippy::suspicious_else_formatting)]
                let non_field_children = {
                    let mut cursor = node.walk();
                    let mut result = ::std::vec::Vec::new();
                    if cursor.goto_first_child() {
                        loop {
                            if cursor.field_name().is_none()
                                && cursor.node().is_named()
                                && !cursor.node().is_extra()
                            {
                                result.push(cursor.node());
                            }
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
                        <InterpolatedStringChildren as ::treesitter_types::FromNode>::from_node(
                            child, src,
                        )?,
                    );
                }
                items
            },
        })
    }
}
impl ::treesitter_types::Spanned for InterpolatedString<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        self.span
    }
}
#[derive(Debug, Clone)]
pub struct InterpolatedStringExpression<'tree> {
    pub span: ::treesitter_types::Span,
    pub interpolator: Identifier<'tree>,
    pub children: InterpolatedString<'tree>,
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
            interpolator: {
                let child = node.child_by_field_name("interpolator").ok_or_else(|| {
                    ::treesitter_types::ParseError::missing_field("interpolator", node)
                })?;
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
                                let candidate = fallback_cursor.node();
                                #[allow(clippy::needless_question_mark)]
                                if (|| -> ::core::result::Result<
                                    _,
                                    ::treesitter_types::ParseError,
                                > {
                                    let child = candidate;
                                    Ok(
                                        <InterpolatedString as ::treesitter_types::FromNode>::from_node(
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
                                            <InterpolatedString as ::treesitter_types::FromNode>::from_node(
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
                <InterpolatedString as ::treesitter_types::FromNode>::from_node(child, src)?
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
    pub children: InterpolationChildren<'tree>,
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
                                        <InterpolationChildren as ::treesitter_types::FromNode>::from_node(
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
                                            <InterpolationChildren as ::treesitter_types::FromNode>::from_node(
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
                <InterpolationChildren as ::treesitter_types::FromNode>::from_node(child, src)?
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
pub struct IntoModifier<'tree> {
    pub span: ::treesitter_types::Span,
    text: &'tree str,
}
impl<'tree> ::treesitter_types::FromNode<'tree> for IntoModifier<'tree> {
    fn from_node(
        node: ::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        debug_assert_eq!(node.kind(), "into_modifier");
        Ok(Self {
            span: ::treesitter_types::Span::from(node),
            text: node.utf8_text(src)?,
        })
    }
}
impl<'tree> ::treesitter_types::LeafNode<'tree> for IntoModifier<'tree> {
    fn text(&self) -> &'tree str {
        self.text
    }
}
impl ::treesitter_types::Spanned for IntoModifier<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        self.span
    }
}
#[derive(Debug, Clone)]
pub struct LambdaExpression<'tree> {
    pub span: ::treesitter_types::Span,
    pub parameters: ::std::vec::Vec<LambdaExpressionParameters<'tree>>,
    pub type_parameters: ::core::option::Option<TypeParameters<'tree>>,
    pub children: LambdaExpressionChildren<'tree>,
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
            parameters: {
                let mut cursor = node.walk();
                let mut items = ::std::vec::Vec::new();
                for child in node.children_by_field_name("parameters", &mut cursor) {
                    items.push(
                        <LambdaExpressionParameters as ::treesitter_types::FromNode>::from_node(
                            child, src,
                        )?,
                    );
                }
                items
            },
            type_parameters: match node.child_by_field_name("type_parameters") {
                Some(child) => Some(<TypeParameters as ::treesitter_types::FromNode>::from_node(
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
                                        <LambdaExpressionChildren as ::treesitter_types::FromNode>::from_node(
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
                                            <LambdaExpressionChildren as ::treesitter_types::FromNode>::from_node(
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
                <LambdaExpressionChildren as ::treesitter_types::FromNode>::from_node(child, src)?
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
pub struct LazyParameterType<'tree> {
    pub span: ::treesitter_types::Span,
    pub r#type: LazyParameterTypeType<'tree>,
}
impl<'tree> ::treesitter_types::FromNode<'tree> for LazyParameterType<'tree> {
    #[allow(clippy::match_single_binding, clippy::suspicious_else_formatting)]
    fn from_node(
        node: ::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        debug_assert_eq!(node.kind(), "lazy_parameter_type");
        Ok(Self {
            span: ::treesitter_types::Span::from(node),
            r#type: {
                let child = node
                    .child_by_field_name("type")
                    .ok_or_else(|| ::treesitter_types::ParseError::missing_field("type", node))?;
                <LazyParameterTypeType as ::treesitter_types::FromNode>::from_node(child, src)?
            },
        })
    }
}
impl ::treesitter_types::Spanned for LazyParameterType<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        self.span
    }
}
#[derive(Debug, Clone)]
pub struct LiteralType<'tree> {
    pub span: ::treesitter_types::Span,
    pub children: LiteralTypeChildren<'tree>,
}
impl<'tree> ::treesitter_types::FromNode<'tree> for LiteralType<'tree> {
    #[allow(clippy::match_single_binding, clippy::suspicious_else_formatting)]
    fn from_node(
        node: ::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        debug_assert_eq!(node.kind(), "literal_type");
        Ok(Self {
            span: ::treesitter_types::Span::from(node),
            children: {
                #[allow(clippy::suspicious_else_formatting)]
                let non_field_children = {
                    let mut cursor = node.walk();
                    let mut result = ::std::vec::Vec::new();
                    if cursor.goto_first_child() {
                        loop {
                            if cursor.field_name().is_none()
                                && cursor.node().is_named()
                                && !cursor.node().is_extra()
                            {
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
                                        <LiteralTypeChildren as ::treesitter_types::FromNode>::from_node(
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
                                            <LiteralTypeChildren as ::treesitter_types::FromNode>::from_node(
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
                <LiteralTypeChildren as ::treesitter_types::FromNode>::from_node(child, src)?
            },
        })
    }
}
impl ::treesitter_types::Spanned for LiteralType<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        self.span
    }
}
#[derive(Debug, Clone)]
pub struct LowerBound<'tree> {
    pub span: ::treesitter_types::Span,
    pub r#type: LowerBoundType<'tree>,
}
impl<'tree> ::treesitter_types::FromNode<'tree> for LowerBound<'tree> {
    #[allow(clippy::match_single_binding, clippy::suspicious_else_formatting)]
    fn from_node(
        node: ::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        debug_assert_eq!(node.kind(), "lower_bound");
        Ok(Self {
            span: ::treesitter_types::Span::from(node),
            r#type: {
                let child = node
                    .child_by_field_name("type")
                    .ok_or_else(|| ::treesitter_types::ParseError::missing_field("type", node))?;
                <LowerBoundType as ::treesitter_types::FromNode>::from_node(child, src)?
            },
        })
    }
}
impl ::treesitter_types::Spanned for LowerBound<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        self.span
    }
}
#[derive(Debug, Clone)]
pub struct MacroBody<'tree> {
    pub span: ::treesitter_types::Span,
    pub children: MacroBodyChildren<'tree>,
}
impl<'tree> ::treesitter_types::FromNode<'tree> for MacroBody<'tree> {
    #[allow(clippy::match_single_binding, clippy::suspicious_else_formatting)]
    fn from_node(
        node: ::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        debug_assert_eq!(node.kind(), "macro_body");
        Ok(Self {
            span: ::treesitter_types::Span::from(node),
            children: {
                #[allow(clippy::suspicious_else_formatting)]
                let non_field_children = {
                    let mut cursor = node.walk();
                    let mut result = ::std::vec::Vec::new();
                    if cursor.goto_first_child() {
                        loop {
                            if cursor.field_name().is_none()
                                && cursor.node().is_named()
                                && !cursor.node().is_extra()
                            {
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
                                        <MacroBodyChildren as ::treesitter_types::FromNode>::from_node(
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
                                            <MacroBodyChildren as ::treesitter_types::FromNode>::from_node(
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
                <MacroBodyChildren as ::treesitter_types::FromNode>::from_node(child, src)?
            },
        })
    }
}
impl ::treesitter_types::Spanned for MacroBody<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        self.span
    }
}
#[derive(Debug, Clone)]
pub struct MatchExpression<'tree> {
    pub span: ::treesitter_types::Span,
    pub body: MatchExpressionBody<'tree>,
    pub value: Expression<'tree>,
    pub children: ::core::option::Option<InlineModifier<'tree>>,
}
impl<'tree> ::treesitter_types::FromNode<'tree> for MatchExpression<'tree> {
    #[allow(clippy::match_single_binding, clippy::suspicious_else_formatting)]
    fn from_node(
        node: ::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        debug_assert_eq!(node.kind(), "match_expression");
        Ok(Self {
            span: ::treesitter_types::Span::from(node),
            body: {
                let child = node
                    .child_by_field_name("body")
                    .ok_or_else(|| ::treesitter_types::ParseError::missing_field("body", node))?;
                <MatchExpressionBody as ::treesitter_types::FromNode>::from_node(child, src)?
            },
            value: {
                let child = node
                    .child_by_field_name("value")
                    .ok_or_else(|| ::treesitter_types::ParseError::missing_field("value", node))?;
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
                    Some(&child) => Some(
                        <InlineModifier as ::treesitter_types::FromNode>::from_node(child, src)?,
                    ),
                    None => None,
                }
            },
        })
    }
}
impl ::treesitter_types::Spanned for MatchExpression<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        self.span
    }
}
#[derive(Debug, Clone)]
pub struct MatchType<'tree> {
    pub span: ::treesitter_types::Span,
    pub children: ::std::vec::Vec<MatchTypeChildren<'tree>>,
}
impl<'tree> ::treesitter_types::FromNode<'tree> for MatchType<'tree> {
    #[allow(clippy::match_single_binding, clippy::suspicious_else_formatting)]
    fn from_node(
        node: ::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        debug_assert_eq!(node.kind(), "match_type");
        Ok(Self {
            span: ::treesitter_types::Span::from(node),
            children: {
                #[allow(clippy::suspicious_else_formatting)]
                let non_field_children = {
                    let mut cursor = node.walk();
                    let mut result = ::std::vec::Vec::new();
                    if cursor.goto_first_child() {
                        loop {
                            if cursor.field_name().is_none()
                                && cursor.node().is_named()
                                && !cursor.node().is_extra()
                            {
                                result.push(cursor.node());
                            }
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
                        <MatchTypeChildren as ::treesitter_types::FromNode>::from_node(child, src)?,
                    );
                }
                items
            },
        })
    }
}
impl ::treesitter_types::Spanned for MatchType<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        self.span
    }
}
#[derive(Debug, Clone)]
pub struct Modifiers<'tree> {
    pub span: ::treesitter_types::Span,
    pub children: ::std::vec::Vec<ModifiersChildren<'tree>>,
}
impl<'tree> ::treesitter_types::FromNode<'tree> for Modifiers<'tree> {
    #[allow(clippy::match_single_binding, clippy::suspicious_else_formatting)]
    fn from_node(
        node: ::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        debug_assert_eq!(node.kind(), "modifiers");
        Ok(Self {
            span: ::treesitter_types::Span::from(node),
            children: {
                #[allow(clippy::suspicious_else_formatting)]
                let non_field_children = {
                    let mut cursor = node.walk();
                    let mut result = ::std::vec::Vec::new();
                    if cursor.goto_first_child() {
                        loop {
                            if cursor.field_name().is_none()
                                && cursor.node().is_named()
                                && !cursor.node().is_extra()
                            {
                                result.push(cursor.node());
                            }
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
                        <ModifiersChildren as ::treesitter_types::FromNode>::from_node(child, src)?,
                    );
                }
                items
            },
        })
    }
}
impl ::treesitter_types::Spanned for Modifiers<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        self.span
    }
}
#[derive(Debug, Clone)]
pub struct NameAndType<'tree> {
    pub span: ::treesitter_types::Span,
    pub name: NameAndTypeName<'tree>,
    pub r#type: NameAndTypeType<'tree>,
}
impl<'tree> ::treesitter_types::FromNode<'tree> for NameAndType<'tree> {
    #[allow(clippy::match_single_binding, clippy::suspicious_else_formatting)]
    fn from_node(
        node: ::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        debug_assert_eq!(node.kind(), "name_and_type");
        Ok(Self {
            span: ::treesitter_types::Span::from(node),
            name: {
                let child = node
                    .child_by_field_name("name")
                    .ok_or_else(|| ::treesitter_types::ParseError::missing_field("name", node))?;
                <NameAndTypeName as ::treesitter_types::FromNode>::from_node(child, src)?
            },
            r#type: {
                let child = node
                    .child_by_field_name("type")
                    .ok_or_else(|| ::treesitter_types::ParseError::missing_field("type", node))?;
                <NameAndTypeType as ::treesitter_types::FromNode>::from_node(child, src)?
            },
        })
    }
}
impl ::treesitter_types::Spanned for NameAndType<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        self.span
    }
}
#[derive(Debug, Clone)]
pub struct NamedPattern<'tree> {
    pub span: ::treesitter_types::Span,
    pub children: ::std::vec::Vec<Pattern<'tree>>,
}
impl<'tree> ::treesitter_types::FromNode<'tree> for NamedPattern<'tree> {
    #[allow(clippy::match_single_binding, clippy::suspicious_else_formatting)]
    fn from_node(
        node: ::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        debug_assert_eq!(node.kind(), "named_pattern");
        Ok(Self {
            span: ::treesitter_types::Span::from(node),
            children: {
                #[allow(clippy::suspicious_else_formatting)]
                let non_field_children = {
                    let mut cursor = node.walk();
                    let mut result = ::std::vec::Vec::new();
                    if cursor.goto_first_child() {
                        loop {
                            if cursor.field_name().is_none()
                                && cursor.node().is_named()
                                && !cursor.node().is_extra()
                            {
                                result.push(cursor.node());
                            }
                            if !cursor.goto_next_sibling() {
                                break;
                            }
                        }
                    }
                    result
                };
                let mut items = ::std::vec::Vec::new();
                for child in non_field_children {
                    items.push(<Pattern as ::treesitter_types::FromNode>::from_node(
                        child, src,
                    )?);
                }
                items
            },
        })
    }
}
impl ::treesitter_types::Spanned for NamedPattern<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        self.span
    }
}
#[derive(Debug, Clone)]
pub struct NamedTuplePattern<'tree> {
    pub span: ::treesitter_types::Span,
    pub children: ::std::vec::Vec<NamedPattern<'tree>>,
}
impl<'tree> ::treesitter_types::FromNode<'tree> for NamedTuplePattern<'tree> {
    #[allow(clippy::match_single_binding, clippy::suspicious_else_formatting)]
    fn from_node(
        node: ::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        debug_assert_eq!(node.kind(), "named_tuple_pattern");
        Ok(Self {
            span: ::treesitter_types::Span::from(node),
            children: {
                #[allow(clippy::suspicious_else_formatting)]
                let non_field_children = {
                    let mut cursor = node.walk();
                    let mut result = ::std::vec::Vec::new();
                    if cursor.goto_first_child() {
                        loop {
                            if cursor.field_name().is_none()
                                && cursor.node().is_named()
                                && !cursor.node().is_extra()
                            {
                                result.push(cursor.node());
                            }
                            if !cursor.goto_next_sibling() {
                                break;
                            }
                        }
                    }
                    result
                };
                let mut items = ::std::vec::Vec::new();
                for child in non_field_children {
                    items.push(<NamedPattern as ::treesitter_types::FromNode>::from_node(
                        child, src,
                    )?);
                }
                items
            },
        })
    }
}
impl ::treesitter_types::Spanned for NamedTuplePattern<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        self.span
    }
}
#[derive(Debug, Clone)]
pub struct NamedTupleType<'tree> {
    pub span: ::treesitter_types::Span,
    pub children: ::std::vec::Vec<NameAndType<'tree>>,
}
impl<'tree> ::treesitter_types::FromNode<'tree> for NamedTupleType<'tree> {
    #[allow(clippy::match_single_binding, clippy::suspicious_else_formatting)]
    fn from_node(
        node: ::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        debug_assert_eq!(node.kind(), "named_tuple_type");
        Ok(Self {
            span: ::treesitter_types::Span::from(node),
            children: {
                #[allow(clippy::suspicious_else_formatting)]
                let non_field_children = {
                    let mut cursor = node.walk();
                    let mut result = ::std::vec::Vec::new();
                    if cursor.goto_first_child() {
                        loop {
                            if cursor.field_name().is_none()
                                && cursor.node().is_named()
                                && !cursor.node().is_extra()
                            {
                                result.push(cursor.node());
                            }
                            if !cursor.goto_next_sibling() {
                                break;
                            }
                        }
                    }
                    result
                };
                let mut items = ::std::vec::Vec::new();
                for child in non_field_children {
                    items.push(<NameAndType as ::treesitter_types::FromNode>::from_node(
                        child, src,
                    )?);
                }
                items
            },
        })
    }
}
impl ::treesitter_types::Spanned for NamedTupleType<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        self.span
    }
}
#[derive(Debug, Clone)]
pub struct NamespaceSelectors<'tree> {
    pub span: ::treesitter_types::Span,
    pub children: ::std::vec::Vec<NamespaceSelectorsChildren<'tree>>,
}
impl<'tree> ::treesitter_types::FromNode<'tree> for NamespaceSelectors<'tree> {
    #[allow(clippy::match_single_binding, clippy::suspicious_else_formatting)]
    fn from_node(
        node: ::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        debug_assert_eq!(node.kind(), "namespace_selectors");
        Ok(Self {
            span: ::treesitter_types::Span::from(node),
            children: {
                #[allow(clippy::suspicious_else_formatting)]
                let non_field_children = {
                    let mut cursor = node.walk();
                    let mut result = ::std::vec::Vec::new();
                    if cursor.goto_first_child() {
                        loop {
                            if cursor.field_name().is_none()
                                && cursor.node().is_named()
                                && !cursor.node().is_extra()
                            {
                                result.push(cursor.node());
                            }
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
                        <NamespaceSelectorsChildren as ::treesitter_types::FromNode>::from_node(
                            child, src,
                        )?,
                    );
                }
                items
            },
        })
    }
}
impl ::treesitter_types::Spanned for NamespaceSelectors<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        self.span
    }
}
#[derive(Debug, Clone)]
pub struct NamespaceWildcard<'tree> {
    pub span: ::treesitter_types::Span,
    text: &'tree str,
}
impl<'tree> ::treesitter_types::FromNode<'tree> for NamespaceWildcard<'tree> {
    fn from_node(
        node: ::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        debug_assert_eq!(node.kind(), "namespace_wildcard");
        Ok(Self {
            span: ::treesitter_types::Span::from(node),
            text: node.utf8_text(src)?,
        })
    }
}
impl<'tree> ::treesitter_types::LeafNode<'tree> for NamespaceWildcard<'tree> {
    fn text(&self) -> &'tree str {
        self.text
    }
}
impl ::treesitter_types::Spanned for NamespaceWildcard<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        self.span
    }
}
#[derive(Debug, Clone)]
pub struct ObjectDefinition<'tree> {
    pub span: ::treesitter_types::Span,
    pub body: ::std::vec::Vec<TemplateBody<'tree>>,
    pub derive: ::core::option::Option<DerivesClause<'tree>>,
    pub extend: ::core::option::Option<ExtendsClause<'tree>>,
    pub name: ObjectDefinitionName<'tree>,
    pub children: ::std::vec::Vec<ObjectDefinitionChildren<'tree>>,
}
impl<'tree> ::treesitter_types::FromNode<'tree> for ObjectDefinition<'tree> {
    #[allow(clippy::match_single_binding, clippy::suspicious_else_formatting)]
    fn from_node(
        node: ::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        debug_assert_eq!(node.kind(), "object_definition");
        Ok(Self {
            span: ::treesitter_types::Span::from(node),
            body: {
                let mut cursor = node.walk();
                let mut items = ::std::vec::Vec::new();
                for child in node.children_by_field_name("body", &mut cursor) {
                    items.push(<TemplateBody as ::treesitter_types::FromNode>::from_node(
                        child, src,
                    )?);
                }
                items
            },
            derive: match node.child_by_field_name("derive") {
                Some(child) => Some(<DerivesClause as ::treesitter_types::FromNode>::from_node(
                    child, src,
                )?),
                None => None,
            },
            extend: match node.child_by_field_name("extend") {
                Some(child) => Some(<ExtendsClause as ::treesitter_types::FromNode>::from_node(
                    child, src,
                )?),
                None => None,
            },
            name: {
                let child = node
                    .child_by_field_name("name")
                    .ok_or_else(|| ::treesitter_types::ParseError::missing_field("name", node))?;
                <ObjectDefinitionName as ::treesitter_types::FromNode>::from_node(child, src)?
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
                        <ObjectDefinitionChildren as ::treesitter_types::FromNode>::from_node(
                            child, src,
                        )?,
                    );
                }
                items
            },
        })
    }
}
impl ::treesitter_types::Spanned for ObjectDefinition<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        self.span
    }
}
#[derive(Debug, Clone)]
pub struct OpaqueModifier<'tree> {
    pub span: ::treesitter_types::Span,
    text: &'tree str,
}
impl<'tree> ::treesitter_types::FromNode<'tree> for OpaqueModifier<'tree> {
    fn from_node(
        node: ::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        debug_assert_eq!(node.kind(), "opaque_modifier");
        Ok(Self {
            span: ::treesitter_types::Span::from(node),
            text: node.utf8_text(src)?,
        })
    }
}
impl<'tree> ::treesitter_types::LeafNode<'tree> for OpaqueModifier<'tree> {
    fn text(&self) -> &'tree str {
        self.text
    }
}
impl ::treesitter_types::Spanned for OpaqueModifier<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        self.span
    }
}
#[derive(Debug, Clone)]
pub struct OpenModifier<'tree> {
    pub span: ::treesitter_types::Span,
    text: &'tree str,
}
impl<'tree> ::treesitter_types::FromNode<'tree> for OpenModifier<'tree> {
    fn from_node(
        node: ::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        debug_assert_eq!(node.kind(), "open_modifier");
        Ok(Self {
            span: ::treesitter_types::Span::from(node),
            text: node.utf8_text(src)?,
        })
    }
}
impl<'tree> ::treesitter_types::LeafNode<'tree> for OpenModifier<'tree> {
    fn text(&self) -> &'tree str {
        self.text
    }
}
impl ::treesitter_types::Spanned for OpenModifier<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        self.span
    }
}
#[derive(Debug, Clone)]
pub struct PackageClause<'tree> {
    pub span: ::treesitter_types::Span,
    pub body: ::core::option::Option<TemplateBody<'tree>>,
    pub name: PackageIdentifier<'tree>,
}
impl<'tree> ::treesitter_types::FromNode<'tree> for PackageClause<'tree> {
    #[allow(clippy::match_single_binding, clippy::suspicious_else_formatting)]
    fn from_node(
        node: ::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        debug_assert_eq!(node.kind(), "package_clause");
        Ok(Self {
            span: ::treesitter_types::Span::from(node),
            body: match node.child_by_field_name("body") {
                Some(child) => Some(<TemplateBody as ::treesitter_types::FromNode>::from_node(
                    child, src,
                )?),
                None => None,
            },
            name: {
                let child = node
                    .child_by_field_name("name")
                    .ok_or_else(|| ::treesitter_types::ParseError::missing_field("name", node))?;
                <PackageIdentifier as ::treesitter_types::FromNode>::from_node(child, src)?
            },
        })
    }
}
impl ::treesitter_types::Spanned for PackageClause<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        self.span
    }
}
#[derive(Debug, Clone)]
pub struct PackageIdentifier<'tree> {
    pub span: ::treesitter_types::Span,
    pub children: ::std::vec::Vec<PackageIdentifierChildren<'tree>>,
}
impl<'tree> ::treesitter_types::FromNode<'tree> for PackageIdentifier<'tree> {
    #[allow(clippy::match_single_binding, clippy::suspicious_else_formatting)]
    fn from_node(
        node: ::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        debug_assert_eq!(node.kind(), "package_identifier");
        Ok(Self {
            span: ::treesitter_types::Span::from(node),
            children: {
                #[allow(clippy::suspicious_else_formatting)]
                let non_field_children = {
                    let mut cursor = node.walk();
                    let mut result = ::std::vec::Vec::new();
                    if cursor.goto_first_child() {
                        loop {
                            if cursor.field_name().is_none()
                                && cursor.node().is_named()
                                && !cursor.node().is_extra()
                            {
                                result.push(cursor.node());
                            }
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
                        <PackageIdentifierChildren as ::treesitter_types::FromNode>::from_node(
                            child, src,
                        )?,
                    );
                }
                items
            },
        })
    }
}
impl ::treesitter_types::Spanned for PackageIdentifier<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        self.span
    }
}
#[derive(Debug, Clone)]
pub struct PackageObject<'tree> {
    pub span: ::treesitter_types::Span,
    pub body: ::std::vec::Vec<TemplateBody<'tree>>,
    pub derive: ::core::option::Option<DerivesClause<'tree>>,
    pub extend: ::core::option::Option<ExtendsClause<'tree>>,
    pub name: PackageObjectName<'tree>,
}
impl<'tree> ::treesitter_types::FromNode<'tree> for PackageObject<'tree> {
    #[allow(clippy::match_single_binding, clippy::suspicious_else_formatting)]
    fn from_node(
        node: ::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        debug_assert_eq!(node.kind(), "package_object");
        Ok(Self {
            span: ::treesitter_types::Span::from(node),
            body: {
                let mut cursor = node.walk();
                let mut items = ::std::vec::Vec::new();
                for child in node.children_by_field_name("body", &mut cursor) {
                    items.push(<TemplateBody as ::treesitter_types::FromNode>::from_node(
                        child, src,
                    )?);
                }
                items
            },
            derive: match node.child_by_field_name("derive") {
                Some(child) => Some(<DerivesClause as ::treesitter_types::FromNode>::from_node(
                    child, src,
                )?),
                None => None,
            },
            extend: match node.child_by_field_name("extend") {
                Some(child) => Some(<ExtendsClause as ::treesitter_types::FromNode>::from_node(
                    child, src,
                )?),
                None => None,
            },
            name: {
                let child = node
                    .child_by_field_name("name")
                    .ok_or_else(|| ::treesitter_types::ParseError::missing_field("name", node))?;
                <PackageObjectName as ::treesitter_types::FromNode>::from_node(child, src)?
            },
        })
    }
}
impl ::treesitter_types::Spanned for PackageObject<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        self.span
    }
}
#[derive(Debug, Clone)]
pub struct Parameter<'tree> {
    pub span: ::treesitter_types::Span,
    pub default_value: ::core::option::Option<Expression<'tree>>,
    pub name: ParameterName<'tree>,
    pub r#type: ParameterType<'tree>,
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
            default_value: match node.child_by_field_name("default_value") {
                Some(child) => Some(<Expression as ::treesitter_types::FromNode>::from_node(
                    child, src,
                )?),
                None => None,
            },
            name: {
                let child = node
                    .child_by_field_name("name")
                    .ok_or_else(|| ::treesitter_types::ParseError::missing_field("name", node))?;
                <ParameterName as ::treesitter_types::FromNode>::from_node(child, src)?
            },
            r#type: {
                let child = node
                    .child_by_field_name("type")
                    .ok_or_else(|| ::treesitter_types::ParseError::missing_field("type", node))?;
                <ParameterType as ::treesitter_types::FromNode>::from_node(child, src)?
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
pub struct ParameterTypes<'tree> {
    pub span: ::treesitter_types::Span,
    pub children: ::std::vec::Vec<ParameterTypesChildren<'tree>>,
}
impl<'tree> ::treesitter_types::FromNode<'tree> for ParameterTypes<'tree> {
    #[allow(clippy::match_single_binding, clippy::suspicious_else_formatting)]
    fn from_node(
        node: ::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        debug_assert_eq!(node.kind(), "parameter_types");
        Ok(Self {
            span: ::treesitter_types::Span::from(node),
            children: {
                #[allow(clippy::suspicious_else_formatting)]
                let non_field_children = {
                    let mut cursor = node.walk();
                    let mut result = ::std::vec::Vec::new();
                    if cursor.goto_first_child() {
                        loop {
                            if cursor.field_name().is_none()
                                && cursor.node().is_named()
                                && !cursor.node().is_extra()
                            {
                                result.push(cursor.node());
                            }
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
                        <ParameterTypesChildren as ::treesitter_types::FromNode>::from_node(
                            child, src,
                        )?,
                    );
                }
                items
            },
        })
    }
}
impl ::treesitter_types::Spanned for ParameterTypes<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        self.span
    }
}
#[derive(Debug, Clone)]
pub struct Parameters<'tree> {
    pub span: ::treesitter_types::Span,
    pub children: ::std::vec::Vec<ParametersChildren<'tree>>,
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
            children: {
                #[allow(clippy::suspicious_else_formatting)]
                let non_field_children = {
                    let mut cursor = node.walk();
                    let mut result = ::std::vec::Vec::new();
                    if cursor.goto_first_child() {
                        loop {
                            if cursor.field_name().is_none()
                                && cursor.node().is_named()
                                && !cursor.node().is_extra()
                            {
                                result.push(cursor.node());
                            }
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
                        <ParametersChildren as ::treesitter_types::FromNode>::from_node(
                            child, src,
                        )?,
                    );
                }
                items
            },
        })
    }
}
impl ::treesitter_types::Spanned for Parameters<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        self.span
    }
}
#[derive(Debug, Clone)]
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
#[derive(Debug, Clone)]
pub struct PostfixExpression<'tree> {
    pub span: ::treesitter_types::Span,
    pub children: ::std::vec::Vec<PostfixExpressionChildren<'tree>>,
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
            children: {
                #[allow(clippy::suspicious_else_formatting)]
                let non_field_children = {
                    let mut cursor = node.walk();
                    let mut result = ::std::vec::Vec::new();
                    if cursor.goto_first_child() {
                        loop {
                            if cursor.field_name().is_none()
                                && cursor.node().is_named()
                                && !cursor.node().is_extra()
                            {
                                result.push(cursor.node());
                            }
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
                        <PostfixExpressionChildren as ::treesitter_types::FromNode>::from_node(
                            child, src,
                        )?,
                    );
                }
                items
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
pub struct PrefixExpression<'tree> {
    pub span: ::treesitter_types::Span,
    pub children: PrefixExpressionChildren<'tree>,
}
impl<'tree> ::treesitter_types::FromNode<'tree> for PrefixExpression<'tree> {
    #[allow(clippy::match_single_binding, clippy::suspicious_else_formatting)]
    fn from_node(
        node: ::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        debug_assert_eq!(node.kind(), "prefix_expression");
        Ok(Self {
            span: ::treesitter_types::Span::from(node),
            children: {
                #[allow(clippy::suspicious_else_formatting)]
                let non_field_children = {
                    let mut cursor = node.walk();
                    let mut result = ::std::vec::Vec::new();
                    if cursor.goto_first_child() {
                        loop {
                            if cursor.field_name().is_none()
                                && cursor.node().is_named()
                                && !cursor.node().is_extra()
                            {
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
                                        <PrefixExpressionChildren as ::treesitter_types::FromNode>::from_node(
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
                                            <PrefixExpressionChildren as ::treesitter_types::FromNode>::from_node(
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
                <PrefixExpressionChildren as ::treesitter_types::FromNode>::from_node(child, src)?
            },
        })
    }
}
impl ::treesitter_types::Spanned for PrefixExpression<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        self.span
    }
}
#[derive(Debug, Clone)]
pub struct ProjectedType<'tree> {
    pub span: ::treesitter_types::Span,
    pub selector: TypeIdentifier<'tree>,
    pub r#type: ProjectedTypeType<'tree>,
}
impl<'tree> ::treesitter_types::FromNode<'tree> for ProjectedType<'tree> {
    #[allow(clippy::match_single_binding, clippy::suspicious_else_formatting)]
    fn from_node(
        node: ::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        debug_assert_eq!(node.kind(), "projected_type");
        Ok(Self {
            span: ::treesitter_types::Span::from(node),
            selector: {
                let child = node.child_by_field_name("selector").ok_or_else(|| {
                    ::treesitter_types::ParseError::missing_field("selector", node)
                })?;
                <TypeIdentifier as ::treesitter_types::FromNode>::from_node(child, src)?
            },
            r#type: {
                let child = node
                    .child_by_field_name("type")
                    .ok_or_else(|| ::treesitter_types::ParseError::missing_field("type", node))?;
                <ProjectedTypeType as ::treesitter_types::FromNode>::from_node(child, src)?
            },
        })
    }
}
impl ::treesitter_types::Spanned for ProjectedType<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        self.span
    }
}
#[derive(Debug, Clone)]
pub struct QuoteExpression<'tree> {
    pub span: ::treesitter_types::Span,
    pub children: ::std::vec::Vec<QuoteExpressionChildren<'tree>>,
}
impl<'tree> ::treesitter_types::FromNode<'tree> for QuoteExpression<'tree> {
    #[allow(clippy::match_single_binding, clippy::suspicious_else_formatting)]
    fn from_node(
        node: ::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        debug_assert_eq!(node.kind(), "quote_expression");
        Ok(Self {
            span: ::treesitter_types::Span::from(node),
            children: {
                #[allow(clippy::suspicious_else_formatting)]
                let non_field_children = {
                    let mut cursor = node.walk();
                    let mut result = ::std::vec::Vec::new();
                    if cursor.goto_first_child() {
                        loop {
                            if cursor.field_name().is_none()
                                && cursor.node().is_named()
                                && !cursor.node().is_extra()
                            {
                                result.push(cursor.node());
                            }
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
                        <QuoteExpressionChildren as ::treesitter_types::FromNode>::from_node(
                            child, src,
                        )?,
                    );
                }
                items
            },
        })
    }
}
impl ::treesitter_types::Spanned for QuoteExpression<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        self.span
    }
}
#[derive(Debug, Clone)]
pub struct Refinement<'tree> {
    pub span: ::treesitter_types::Span,
    pub children: ::std::vec::Vec<RefinementChildren<'tree>>,
}
impl<'tree> ::treesitter_types::FromNode<'tree> for Refinement<'tree> {
    #[allow(clippy::match_single_binding, clippy::suspicious_else_formatting)]
    fn from_node(
        node: ::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        debug_assert_eq!(node.kind(), "refinement");
        Ok(Self {
            span: ::treesitter_types::Span::from(node),
            children: {
                #[allow(clippy::suspicious_else_formatting)]
                let non_field_children = {
                    let mut cursor = node.walk();
                    let mut result = ::std::vec::Vec::new();
                    if cursor.goto_first_child() {
                        loop {
                            if cursor.field_name().is_none()
                                && cursor.node().is_named()
                                && !cursor.node().is_extra()
                            {
                                result.push(cursor.node());
                            }
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
                        <RefinementChildren as ::treesitter_types::FromNode>::from_node(
                            child, src,
                        )?,
                    );
                }
                items
            },
        })
    }
}
impl ::treesitter_types::Spanned for Refinement<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        self.span
    }
}
#[derive(Debug, Clone)]
pub struct RepeatPattern<'tree> {
    pub span: ::treesitter_types::Span,
    pub pattern: Pattern<'tree>,
}
impl<'tree> ::treesitter_types::FromNode<'tree> for RepeatPattern<'tree> {
    #[allow(clippy::match_single_binding, clippy::suspicious_else_formatting)]
    fn from_node(
        node: ::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        debug_assert_eq!(node.kind(), "repeat_pattern");
        Ok(Self {
            span: ::treesitter_types::Span::from(node),
            pattern: {
                let child = node.child_by_field_name("pattern").ok_or_else(|| {
                    ::treesitter_types::ParseError::missing_field("pattern", node)
                })?;
                <Pattern as ::treesitter_types::FromNode>::from_node(child, src)?
            },
        })
    }
}
impl ::treesitter_types::Spanned for RepeatPattern<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        self.span
    }
}
#[derive(Debug, Clone)]
pub struct RepeatedParameterType<'tree> {
    pub span: ::treesitter_types::Span,
    pub r#type: RepeatedParameterTypeType<'tree>,
}
impl<'tree> ::treesitter_types::FromNode<'tree> for RepeatedParameterType<'tree> {
    #[allow(clippy::match_single_binding, clippy::suspicious_else_formatting)]
    fn from_node(
        node: ::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        debug_assert_eq!(node.kind(), "repeated_parameter_type");
        Ok(Self {
            span: ::treesitter_types::Span::from(node),
            r#type: {
                let child = node
                    .child_by_field_name("type")
                    .ok_or_else(|| ::treesitter_types::ParseError::missing_field("type", node))?;
                <RepeatedParameterTypeType as ::treesitter_types::FromNode>::from_node(child, src)?
            },
        })
    }
}
impl ::treesitter_types::Spanned for RepeatedParameterType<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        self.span
    }
}
#[derive(Debug, Clone)]
pub struct ReturnExpression<'tree> {
    pub span: ::treesitter_types::Span,
    pub children: ::core::option::Option<Expression<'tree>>,
}
impl<'tree> ::treesitter_types::FromNode<'tree> for ReturnExpression<'tree> {
    #[allow(clippy::match_single_binding, clippy::suspicious_else_formatting)]
    fn from_node(
        node: ::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        debug_assert_eq!(node.kind(), "return_expression");
        Ok(Self {
            span: ::treesitter_types::Span::from(node),
            children: {
                #[allow(clippy::suspicious_else_formatting)]
                let non_field_children = {
                    let mut cursor = node.walk();
                    let mut result = ::std::vec::Vec::new();
                    if cursor.goto_first_child() {
                        loop {
                            if cursor.field_name().is_none()
                                && cursor.node().is_named()
                                && !cursor.node().is_extra()
                            {
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
impl ::treesitter_types::Spanned for ReturnExpression<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        self.span
    }
}
#[derive(Debug, Clone)]
pub struct SelfType<'tree> {
    pub span: ::treesitter_types::Span,
    pub children: ::std::vec::Vec<SelfTypeChildren<'tree>>,
}
impl<'tree> ::treesitter_types::FromNode<'tree> for SelfType<'tree> {
    #[allow(clippy::match_single_binding, clippy::suspicious_else_formatting)]
    fn from_node(
        node: ::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        debug_assert_eq!(node.kind(), "self_type");
        Ok(Self {
            span: ::treesitter_types::Span::from(node),
            children: {
                #[allow(clippy::suspicious_else_formatting)]
                let non_field_children = {
                    let mut cursor = node.walk();
                    let mut result = ::std::vec::Vec::new();
                    if cursor.goto_first_child() {
                        loop {
                            if cursor.field_name().is_none()
                                && cursor.node().is_named()
                                && !cursor.node().is_extra()
                            {
                                result.push(cursor.node());
                            }
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
                        <SelfTypeChildren as ::treesitter_types::FromNode>::from_node(child, src)?,
                    );
                }
                items
            },
        })
    }
}
impl ::treesitter_types::Spanned for SelfType<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        self.span
    }
}
#[derive(Debug, Clone)]
pub struct SimpleEnumCase<'tree> {
    pub span: ::treesitter_types::Span,
    pub extend: ::core::option::Option<ExtendsClause<'tree>>,
    pub name: SimpleEnumCaseName<'tree>,
}
impl<'tree> ::treesitter_types::FromNode<'tree> for SimpleEnumCase<'tree> {
    #[allow(clippy::match_single_binding, clippy::suspicious_else_formatting)]
    fn from_node(
        node: ::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        debug_assert_eq!(node.kind(), "simple_enum_case");
        Ok(Self {
            span: ::treesitter_types::Span::from(node),
            extend: match node.child_by_field_name("extend") {
                Some(child) => Some(<ExtendsClause as ::treesitter_types::FromNode>::from_node(
                    child, src,
                )?),
                None => None,
            },
            name: {
                let child = node
                    .child_by_field_name("name")
                    .ok_or_else(|| ::treesitter_types::ParseError::missing_field("name", node))?;
                <SimpleEnumCaseName as ::treesitter_types::FromNode>::from_node(child, src)?
            },
        })
    }
}
impl ::treesitter_types::Spanned for SimpleEnumCase<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        self.span
    }
}
#[derive(Debug, Clone)]
pub struct SingletonType<'tree> {
    pub span: ::treesitter_types::Span,
    pub children: SingletonTypeChildren<'tree>,
}
impl<'tree> ::treesitter_types::FromNode<'tree> for SingletonType<'tree> {
    #[allow(clippy::match_single_binding, clippy::suspicious_else_formatting)]
    fn from_node(
        node: ::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        debug_assert_eq!(node.kind(), "singleton_type");
        Ok(Self {
            span: ::treesitter_types::Span::from(node),
            children: {
                #[allow(clippy::suspicious_else_formatting)]
                let non_field_children = {
                    let mut cursor = node.walk();
                    let mut result = ::std::vec::Vec::new();
                    if cursor.goto_first_child() {
                        loop {
                            if cursor.field_name().is_none()
                                && cursor.node().is_named()
                                && !cursor.node().is_extra()
                            {
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
                                        <SingletonTypeChildren as ::treesitter_types::FromNode>::from_node(
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
                                            <SingletonTypeChildren as ::treesitter_types::FromNode>::from_node(
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
                <SingletonTypeChildren as ::treesitter_types::FromNode>::from_node(child, src)?
            },
        })
    }
}
impl ::treesitter_types::Spanned for SingletonType<'_> {
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
pub struct StableIdentifier<'tree> {
    pub span: ::treesitter_types::Span,
    pub children: ::std::vec::Vec<StableIdentifierChildren<'tree>>,
}
impl<'tree> ::treesitter_types::FromNode<'tree> for StableIdentifier<'tree> {
    #[allow(clippy::match_single_binding, clippy::suspicious_else_formatting)]
    fn from_node(
        node: ::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        debug_assert_eq!(node.kind(), "stable_identifier");
        Ok(Self {
            span: ::treesitter_types::Span::from(node),
            children: {
                #[allow(clippy::suspicious_else_formatting)]
                let non_field_children = {
                    let mut cursor = node.walk();
                    let mut result = ::std::vec::Vec::new();
                    if cursor.goto_first_child() {
                        loop {
                            if cursor.field_name().is_none()
                                && cursor.node().is_named()
                                && !cursor.node().is_extra()
                            {
                                result.push(cursor.node());
                            }
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
                        <StableIdentifierChildren as ::treesitter_types::FromNode>::from_node(
                            child, src,
                        )?,
                    );
                }
                items
            },
        })
    }
}
impl ::treesitter_types::Spanned for StableIdentifier<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        self.span
    }
}
#[derive(Debug, Clone)]
pub struct StableTypeIdentifier<'tree> {
    pub span: ::treesitter_types::Span,
    pub children: ::std::vec::Vec<StableTypeIdentifierChildren<'tree>>,
}
impl<'tree> ::treesitter_types::FromNode<'tree> for StableTypeIdentifier<'tree> {
    #[allow(clippy::match_single_binding, clippy::suspicious_else_formatting)]
    fn from_node(
        node: ::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        debug_assert_eq!(node.kind(), "stable_type_identifier");
        Ok(Self {
            span: ::treesitter_types::Span::from(node),
            children: {
                #[allow(clippy::suspicious_else_formatting)]
                let non_field_children = {
                    let mut cursor = node.walk();
                    let mut result = ::std::vec::Vec::new();
                    if cursor.goto_first_child() {
                        loop {
                            if cursor.field_name().is_none()
                                && cursor.node().is_named()
                                && !cursor.node().is_extra()
                            {
                                result.push(cursor.node());
                            }
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
                        <StableTypeIdentifierChildren as ::treesitter_types::FromNode>::from_node(
                            child, src,
                        )?,
                    );
                }
                items
            },
        })
    }
}
impl ::treesitter_types::Spanned for StableTypeIdentifier<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        self.span
    }
}
#[derive(Debug, Clone)]
pub struct String<'tree> {
    pub span: ::treesitter_types::Span,
    pub children: ::std::vec::Vec<EscapeSequence<'tree>>,
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
                    items.push(<EscapeSequence as ::treesitter_types::FromNode>::from_node(
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
pub struct StructuralType<'tree> {
    pub span: ::treesitter_types::Span,
    pub children: ::std::vec::Vec<StructuralTypeChildren<'tree>>,
}
impl<'tree> ::treesitter_types::FromNode<'tree> for StructuralType<'tree> {
    #[allow(clippy::match_single_binding, clippy::suspicious_else_formatting)]
    fn from_node(
        node: ::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        debug_assert_eq!(node.kind(), "structural_type");
        Ok(Self {
            span: ::treesitter_types::Span::from(node),
            children: {
                #[allow(clippy::suspicious_else_formatting)]
                let non_field_children = {
                    let mut cursor = node.walk();
                    let mut result = ::std::vec::Vec::new();
                    if cursor.goto_first_child() {
                        loop {
                            if cursor.field_name().is_none()
                                && cursor.node().is_named()
                                && !cursor.node().is_extra()
                            {
                                result.push(cursor.node());
                            }
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
                        <StructuralTypeChildren as ::treesitter_types::FromNode>::from_node(
                            child, src,
                        )?,
                    );
                }
                items
            },
        })
    }
}
impl ::treesitter_types::Spanned for StructuralType<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        self.span
    }
}
#[derive(Debug, Clone)]
pub struct TemplateBody<'tree> {
    pub span: ::treesitter_types::Span,
    pub children: ::std::vec::Vec<TemplateBodyChildren<'tree>>,
}
impl<'tree> ::treesitter_types::FromNode<'tree> for TemplateBody<'tree> {
    #[allow(clippy::match_single_binding, clippy::suspicious_else_formatting)]
    fn from_node(
        node: ::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        debug_assert_eq!(node.kind(), "template_body");
        Ok(Self {
            span: ::treesitter_types::Span::from(node),
            children: {
                #[allow(clippy::suspicious_else_formatting)]
                let non_field_children = {
                    let mut cursor = node.walk();
                    let mut result = ::std::vec::Vec::new();
                    if cursor.goto_first_child() {
                        loop {
                            if cursor.field_name().is_none()
                                && cursor.node().is_named()
                                && !cursor.node().is_extra()
                            {
                                result.push(cursor.node());
                            }
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
                        <TemplateBodyChildren as ::treesitter_types::FromNode>::from_node(
                            child, src,
                        )?,
                    );
                }
                items
            },
        })
    }
}
impl ::treesitter_types::Spanned for TemplateBody<'_> {
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
impl ::treesitter_types::Spanned for ThrowExpression<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        self.span
    }
}
#[derive(Debug, Clone)]
pub struct TrackedModifier<'tree> {
    pub span: ::treesitter_types::Span,
    text: &'tree str,
}
impl<'tree> ::treesitter_types::FromNode<'tree> for TrackedModifier<'tree> {
    fn from_node(
        node: ::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        debug_assert_eq!(node.kind(), "tracked_modifier");
        Ok(Self {
            span: ::treesitter_types::Span::from(node),
            text: node.utf8_text(src)?,
        })
    }
}
impl<'tree> ::treesitter_types::LeafNode<'tree> for TrackedModifier<'tree> {
    fn text(&self) -> &'tree str {
        self.text
    }
}
impl ::treesitter_types::Spanned for TrackedModifier<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        self.span
    }
}
#[derive(Debug, Clone)]
pub struct TraitDefinition<'tree> {
    pub span: ::treesitter_types::Span,
    pub body: ::std::vec::Vec<TemplateBody<'tree>>,
    pub class_parameters: ::std::vec::Vec<ClassParameters<'tree>>,
    pub derive: ::core::option::Option<DerivesClause<'tree>>,
    pub extend: ::core::option::Option<ExtendsClause<'tree>>,
    pub name: TraitDefinitionName<'tree>,
    pub type_parameters: ::core::option::Option<TypeParameters<'tree>>,
    pub children: ::std::vec::Vec<TraitDefinitionChildren<'tree>>,
}
impl<'tree> ::treesitter_types::FromNode<'tree> for TraitDefinition<'tree> {
    #[allow(clippy::match_single_binding, clippy::suspicious_else_formatting)]
    fn from_node(
        node: ::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        debug_assert_eq!(node.kind(), "trait_definition");
        Ok(Self {
            span: ::treesitter_types::Span::from(node),
            body: {
                let mut cursor = node.walk();
                let mut items = ::std::vec::Vec::new();
                for child in node.children_by_field_name("body", &mut cursor) {
                    items.push(<TemplateBody as ::treesitter_types::FromNode>::from_node(
                        child, src,
                    )?);
                }
                items
            },
            class_parameters: {
                let mut cursor = node.walk();
                let mut items = ::std::vec::Vec::new();
                for child in node.children_by_field_name("class_parameters", &mut cursor) {
                    items.push(
                        <ClassParameters as ::treesitter_types::FromNode>::from_node(child, src)?,
                    );
                }
                items
            },
            derive: match node.child_by_field_name("derive") {
                Some(child) => Some(<DerivesClause as ::treesitter_types::FromNode>::from_node(
                    child, src,
                )?),
                None => None,
            },
            extend: match node.child_by_field_name("extend") {
                Some(child) => Some(<ExtendsClause as ::treesitter_types::FromNode>::from_node(
                    child, src,
                )?),
                None => None,
            },
            name: {
                let child = node
                    .child_by_field_name("name")
                    .ok_or_else(|| ::treesitter_types::ParseError::missing_field("name", node))?;
                <TraitDefinitionName as ::treesitter_types::FromNode>::from_node(child, src)?
            },
            type_parameters: match node.child_by_field_name("type_parameters") {
                Some(child) => Some(<TypeParameters as ::treesitter_types::FromNode>::from_node(
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
                        <TraitDefinitionChildren as ::treesitter_types::FromNode>::from_node(
                            child, src,
                        )?,
                    );
                }
                items
            },
        })
    }
}
impl ::treesitter_types::Spanned for TraitDefinition<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        self.span
    }
}
#[derive(Debug, Clone)]
pub struct TransparentModifier<'tree> {
    pub span: ::treesitter_types::Span,
    text: &'tree str,
}
impl<'tree> ::treesitter_types::FromNode<'tree> for TransparentModifier<'tree> {
    fn from_node(
        node: ::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        debug_assert_eq!(node.kind(), "transparent_modifier");
        Ok(Self {
            span: ::treesitter_types::Span::from(node),
            text: node.utf8_text(src)?,
        })
    }
}
impl<'tree> ::treesitter_types::LeafNode<'tree> for TransparentModifier<'tree> {
    fn text(&self) -> &'tree str {
        self.text
    }
}
impl ::treesitter_types::Spanned for TransparentModifier<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        self.span
    }
}
#[derive(Debug, Clone)]
pub struct TryExpression<'tree> {
    pub span: ::treesitter_types::Span,
    pub body: TryExpressionBody<'tree>,
    pub children: ::std::vec::Vec<TryExpressionChildren<'tree>>,
}
impl<'tree> ::treesitter_types::FromNode<'tree> for TryExpression<'tree> {
    #[allow(clippy::match_single_binding, clippy::suspicious_else_formatting)]
    fn from_node(
        node: ::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        debug_assert_eq!(node.kind(), "try_expression");
        Ok(Self {
            span: ::treesitter_types::Span::from(node),
            body: {
                let child = node
                    .child_by_field_name("body")
                    .ok_or_else(|| ::treesitter_types::ParseError::missing_field("body", node))?;
                <TryExpressionBody as ::treesitter_types::FromNode>::from_node(child, src)?
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
                        <TryExpressionChildren as ::treesitter_types::FromNode>::from_node(
                            child, src,
                        )?,
                    );
                }
                items
            },
        })
    }
}
impl ::treesitter_types::Spanned for TryExpression<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        self.span
    }
}
#[derive(Debug, Clone)]
pub struct TupleExpression<'tree> {
    pub span: ::treesitter_types::Span,
    pub children: ::std::vec::Vec<Expression<'tree>>,
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
                    items.push(<Expression as ::treesitter_types::FromNode>::from_node(
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
    pub children: ::std::vec::Vec<Pattern<'tree>>,
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
            children: {
                #[allow(clippy::suspicious_else_formatting)]
                let non_field_children = {
                    let mut cursor = node.walk();
                    let mut result = ::std::vec::Vec::new();
                    if cursor.goto_first_child() {
                        loop {
                            if cursor.field_name().is_none()
                                && cursor.node().is_named()
                                && !cursor.node().is_extra()
                            {
                                result.push(cursor.node());
                            }
                            if !cursor.goto_next_sibling() {
                                break;
                            }
                        }
                    }
                    result
                };
                let mut items = ::std::vec::Vec::new();
                for child in non_field_children {
                    items.push(<Pattern as ::treesitter_types::FromNode>::from_node(
                        child, src,
                    )?);
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
pub struct TypeArguments<'tree> {
    pub span: ::treesitter_types::Span,
    pub children: ::std::vec::Vec<TypeArgumentsChildren<'tree>>,
}
impl<'tree> ::treesitter_types::FromNode<'tree> for TypeArguments<'tree> {
    #[allow(clippy::match_single_binding, clippy::suspicious_else_formatting)]
    fn from_node(
        node: ::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        debug_assert_eq!(node.kind(), "type_arguments");
        Ok(Self {
            span: ::treesitter_types::Span::from(node),
            children: {
                #[allow(clippy::suspicious_else_formatting)]
                let non_field_children = {
                    let mut cursor = node.walk();
                    let mut result = ::std::vec::Vec::new();
                    if cursor.goto_first_child() {
                        loop {
                            if cursor.field_name().is_none()
                                && cursor.node().is_named()
                                && !cursor.node().is_extra()
                            {
                                result.push(cursor.node());
                            }
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
                        <TypeArgumentsChildren as ::treesitter_types::FromNode>::from_node(
                            child, src,
                        )?,
                    );
                }
                items
            },
        })
    }
}
impl ::treesitter_types::Spanned for TypeArguments<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        self.span
    }
}
#[derive(Debug, Clone)]
pub struct TypeCaseClause<'tree> {
    pub span: ::treesitter_types::Span,
    pub body: ::std::vec::Vec<TypeCaseClauseBody<'tree>>,
    pub return_type: TypeCaseClauseReturnType<'tree>,
    pub children: TypeCaseClauseChildren<'tree>,
}
impl<'tree> ::treesitter_types::FromNode<'tree> for TypeCaseClause<'tree> {
    #[allow(clippy::match_single_binding, clippy::suspicious_else_formatting)]
    fn from_node(
        node: ::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        debug_assert_eq!(node.kind(), "type_case_clause");
        Ok(Self {
            span: ::treesitter_types::Span::from(node),
            body: {
                let mut cursor = node.walk();
                let mut items = ::std::vec::Vec::new();
                for child in node.children_by_field_name("body", &mut cursor) {
                    items.push(
                        <TypeCaseClauseBody as ::treesitter_types::FromNode>::from_node(
                            child, src,
                        )?,
                    );
                }
                items
            },
            return_type: {
                let child = node.child_by_field_name("return_type").ok_or_else(|| {
                    ::treesitter_types::ParseError::missing_field("return_type", node)
                })?;
                <TypeCaseClauseReturnType as ::treesitter_types::FromNode>::from_node(child, src)?
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
                                        <TypeCaseClauseChildren as ::treesitter_types::FromNode>::from_node(
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
                                            <TypeCaseClauseChildren as ::treesitter_types::FromNode>::from_node(
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
                <TypeCaseClauseChildren as ::treesitter_types::FromNode>::from_node(child, src)?
            },
        })
    }
}
impl ::treesitter_types::Spanned for TypeCaseClause<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        self.span
    }
}
#[derive(Debug, Clone)]
pub struct TypeDefinition<'tree> {
    pub span: ::treesitter_types::Span,
    pub bound: ::std::vec::Vec<TypeDefinitionBound<'tree>>,
    pub name: TypeIdentifier<'tree>,
    pub r#type: ::core::option::Option<TypeDefinitionType<'tree>>,
    pub type_parameters: ::core::option::Option<TypeParameters<'tree>>,
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
            bound: {
                let mut cursor = node.walk();
                let mut items = ::std::vec::Vec::new();
                for child in node.children_by_field_name("bound", &mut cursor) {
                    items.push(
                        <TypeDefinitionBound as ::treesitter_types::FromNode>::from_node(
                            child, src,
                        )?,
                    );
                }
                items
            },
            name: {
                let child = node
                    .child_by_field_name("name")
                    .ok_or_else(|| ::treesitter_types::ParseError::missing_field("name", node))?;
                <TypeIdentifier as ::treesitter_types::FromNode>::from_node(child, src)?
            },
            r#type: match node.child_by_field_name("type") {
                Some(child) => Some(
                    <TypeDefinitionType as ::treesitter_types::FromNode>::from_node(child, src)?,
                ),
                None => None,
            },
            type_parameters: match node.child_by_field_name("type_parameters") {
                Some(child) => Some(<TypeParameters as ::treesitter_types::FromNode>::from_node(
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
pub struct TypeLambda<'tree> {
    pub span: ::treesitter_types::Span,
    pub bound: ::std::vec::Vec<TypeLambdaBound<'tree>>,
    pub name: ::std::vec::Vec<TypeLambdaName<'tree>>,
    pub return_type: TypeLambdaReturnType<'tree>,
    pub type_parameters: ::std::vec::Vec<TypeParameters<'tree>>,
}
impl<'tree> ::treesitter_types::FromNode<'tree> for TypeLambda<'tree> {
    #[allow(clippy::match_single_binding, clippy::suspicious_else_formatting)]
    fn from_node(
        node: ::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        debug_assert_eq!(node.kind(), "type_lambda");
        Ok(Self {
            span: ::treesitter_types::Span::from(node),
            bound: {
                let mut cursor = node.walk();
                let mut items = ::std::vec::Vec::new();
                for child in node.children_by_field_name("bound", &mut cursor) {
                    items.push(
                        <TypeLambdaBound as ::treesitter_types::FromNode>::from_node(child, src)?,
                    );
                }
                items
            },
            name: {
                let mut cursor = node.walk();
                let mut items = ::std::vec::Vec::new();
                for child in node.children_by_field_name("name", &mut cursor) {
                    items.push(<TypeLambdaName as ::treesitter_types::FromNode>::from_node(
                        child, src,
                    )?);
                }
                items
            },
            return_type: {
                let child = node.child_by_field_name("return_type").ok_or_else(|| {
                    ::treesitter_types::ParseError::missing_field("return_type", node)
                })?;
                <TypeLambdaReturnType as ::treesitter_types::FromNode>::from_node(child, src)?
            },
            type_parameters: {
                let mut cursor = node.walk();
                let mut items = ::std::vec::Vec::new();
                for child in node.children_by_field_name("type_parameters", &mut cursor) {
                    items.push(<TypeParameters as ::treesitter_types::FromNode>::from_node(
                        child, src,
                    )?);
                }
                items
            },
        })
    }
}
impl ::treesitter_types::Spanned for TypeLambda<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        self.span
    }
}
#[derive(Debug, Clone)]
pub struct TypeParameters<'tree> {
    pub span: ::treesitter_types::Span,
    pub bound: ::std::vec::Vec<TypeParametersBound<'tree>>,
    pub name: ::std::vec::Vec<TypeParametersName<'tree>>,
    pub type_parameters: ::std::vec::Vec<TypeParameters<'tree>>,
    pub children: ::std::vec::Vec<TypeParametersChildren<'tree>>,
}
impl<'tree> ::treesitter_types::FromNode<'tree> for TypeParameters<'tree> {
    #[allow(clippy::match_single_binding, clippy::suspicious_else_formatting)]
    fn from_node(
        node: ::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        debug_assert_eq!(node.kind(), "type_parameters");
        Ok(Self {
            span: ::treesitter_types::Span::from(node),
            bound: {
                let mut cursor = node.walk();
                let mut items = ::std::vec::Vec::new();
                for child in node.children_by_field_name("bound", &mut cursor) {
                    items.push(
                        <TypeParametersBound as ::treesitter_types::FromNode>::from_node(
                            child, src,
                        )?,
                    );
                }
                items
            },
            name: {
                let mut cursor = node.walk();
                let mut items = ::std::vec::Vec::new();
                for child in node.children_by_field_name("name", &mut cursor) {
                    items.push(
                        <TypeParametersName as ::treesitter_types::FromNode>::from_node(
                            child, src,
                        )?,
                    );
                }
                items
            },
            type_parameters: {
                let mut cursor = node.walk();
                let mut items = ::std::vec::Vec::new();
                for child in node.children_by_field_name("type_parameters", &mut cursor) {
                    items.push(<TypeParameters as ::treesitter_types::FromNode>::from_node(
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
                        <TypeParametersChildren as ::treesitter_types::FromNode>::from_node(
                            child, src,
                        )?,
                    );
                }
                items
            },
        })
    }
}
impl ::treesitter_types::Spanned for TypeParameters<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        self.span
    }
}
#[derive(Debug, Clone)]
pub struct TypedPattern<'tree> {
    pub span: ::treesitter_types::Span,
    pub pattern: Pattern<'tree>,
    pub r#type: TypedPatternType<'tree>,
}
impl<'tree> ::treesitter_types::FromNode<'tree> for TypedPattern<'tree> {
    #[allow(clippy::match_single_binding, clippy::suspicious_else_formatting)]
    fn from_node(
        node: ::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        debug_assert_eq!(node.kind(), "typed_pattern");
        Ok(Self {
            span: ::treesitter_types::Span::from(node),
            pattern: {
                let child = node.child_by_field_name("pattern").ok_or_else(|| {
                    ::treesitter_types::ParseError::missing_field("pattern", node)
                })?;
                <Pattern as ::treesitter_types::FromNode>::from_node(child, src)?
            },
            r#type: {
                let child = node
                    .child_by_field_name("type")
                    .ok_or_else(|| ::treesitter_types::ParseError::missing_field("type", node))?;
                <TypedPatternType as ::treesitter_types::FromNode>::from_node(child, src)?
            },
        })
    }
}
impl ::treesitter_types::Spanned for TypedPattern<'_> {
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
pub struct UpperBound<'tree> {
    pub span: ::treesitter_types::Span,
    pub r#type: UpperBoundType<'tree>,
}
impl<'tree> ::treesitter_types::FromNode<'tree> for UpperBound<'tree> {
    #[allow(clippy::match_single_binding, clippy::suspicious_else_formatting)]
    fn from_node(
        node: ::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        debug_assert_eq!(node.kind(), "upper_bound");
        Ok(Self {
            span: ::treesitter_types::Span::from(node),
            r#type: {
                let child = node
                    .child_by_field_name("type")
                    .ok_or_else(|| ::treesitter_types::ParseError::missing_field("type", node))?;
                <UpperBoundType as ::treesitter_types::FromNode>::from_node(child, src)?
            },
        })
    }
}
impl ::treesitter_types::Spanned for UpperBound<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        self.span
    }
}
#[derive(Debug, Clone)]
pub struct UsingDirective<'tree> {
    pub span: ::treesitter_types::Span,
    pub children: ::std::vec::Vec<UsingDirectiveChildren<'tree>>,
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
            children: {
                #[allow(clippy::suspicious_else_formatting)]
                let non_field_children = {
                    let mut cursor = node.walk();
                    let mut result = ::std::vec::Vec::new();
                    if cursor.goto_first_child() {
                        loop {
                            if cursor.field_name().is_none()
                                && cursor.node().is_named()
                                && !cursor.node().is_extra()
                            {
                                result.push(cursor.node());
                            }
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
                        <UsingDirectiveChildren as ::treesitter_types::FromNode>::from_node(
                            child, src,
                        )?,
                    );
                }
                items
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
pub struct ValDeclaration<'tree> {
    pub span: ::treesitter_types::Span,
    pub name: ::std::vec::Vec<ValDeclarationName<'tree>>,
    pub r#type: ValDeclarationType<'tree>,
    pub children: ::std::vec::Vec<ValDeclarationChildren<'tree>>,
}
impl<'tree> ::treesitter_types::FromNode<'tree> for ValDeclaration<'tree> {
    #[allow(clippy::match_single_binding, clippy::suspicious_else_formatting)]
    fn from_node(
        node: ::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        debug_assert_eq!(node.kind(), "val_declaration");
        Ok(Self {
            span: ::treesitter_types::Span::from(node),
            name: {
                let mut cursor = node.walk();
                let mut items = ::std::vec::Vec::new();
                for child in node.children_by_field_name("name", &mut cursor) {
                    items.push(
                        <ValDeclarationName as ::treesitter_types::FromNode>::from_node(
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
                <ValDeclarationType as ::treesitter_types::FromNode>::from_node(child, src)?
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
                        <ValDeclarationChildren as ::treesitter_types::FromNode>::from_node(
                            child, src,
                        )?,
                    );
                }
                items
            },
        })
    }
}
impl ::treesitter_types::Spanned for ValDeclaration<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        self.span
    }
}
#[derive(Debug, Clone)]
pub struct ValDefinition<'tree> {
    pub span: ::treesitter_types::Span,
    pub pattern: ValDefinitionPattern<'tree>,
    pub r#type: ::core::option::Option<ValDefinitionType<'tree>>,
    pub value: ValDefinitionValue<'tree>,
    pub children: ::std::vec::Vec<ValDefinitionChildren<'tree>>,
}
impl<'tree> ::treesitter_types::FromNode<'tree> for ValDefinition<'tree> {
    #[allow(clippy::match_single_binding, clippy::suspicious_else_formatting)]
    fn from_node(
        node: ::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        debug_assert_eq!(node.kind(), "val_definition");
        Ok(Self {
            span: ::treesitter_types::Span::from(node),
            pattern: {
                let child = node.child_by_field_name("pattern").ok_or_else(|| {
                    ::treesitter_types::ParseError::missing_field("pattern", node)
                })?;
                <ValDefinitionPattern as ::treesitter_types::FromNode>::from_node(child, src)?
            },
            r#type: match node.child_by_field_name("type") {
                Some(child) => Some(
                    <ValDefinitionType as ::treesitter_types::FromNode>::from_node(child, src)?,
                ),
                None => None,
            },
            value: {
                let child = node
                    .child_by_field_name("value")
                    .ok_or_else(|| ::treesitter_types::ParseError::missing_field("value", node))?;
                <ValDefinitionValue as ::treesitter_types::FromNode>::from_node(child, src)?
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
                        <ValDefinitionChildren as ::treesitter_types::FromNode>::from_node(
                            child, src,
                        )?,
                    );
                }
                items
            },
        })
    }
}
impl ::treesitter_types::Spanned for ValDefinition<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        self.span
    }
}
#[derive(Debug, Clone)]
pub struct VarDeclaration<'tree> {
    pub span: ::treesitter_types::Span,
    pub name: ::std::vec::Vec<VarDeclarationName<'tree>>,
    pub r#type: VarDeclarationType<'tree>,
    pub children: ::std::vec::Vec<VarDeclarationChildren<'tree>>,
}
impl<'tree> ::treesitter_types::FromNode<'tree> for VarDeclaration<'tree> {
    #[allow(clippy::match_single_binding, clippy::suspicious_else_formatting)]
    fn from_node(
        node: ::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        debug_assert_eq!(node.kind(), "var_declaration");
        Ok(Self {
            span: ::treesitter_types::Span::from(node),
            name: {
                let mut cursor = node.walk();
                let mut items = ::std::vec::Vec::new();
                for child in node.children_by_field_name("name", &mut cursor) {
                    items.push(
                        <VarDeclarationName as ::treesitter_types::FromNode>::from_node(
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
                <VarDeclarationType as ::treesitter_types::FromNode>::from_node(child, src)?
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
                        <VarDeclarationChildren as ::treesitter_types::FromNode>::from_node(
                            child, src,
                        )?,
                    );
                }
                items
            },
        })
    }
}
impl ::treesitter_types::Spanned for VarDeclaration<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        self.span
    }
}
#[derive(Debug, Clone)]
pub struct VarDefinition<'tree> {
    pub span: ::treesitter_types::Span,
    pub pattern: VarDefinitionPattern<'tree>,
    pub r#type: ::core::option::Option<VarDefinitionType<'tree>>,
    pub value: VarDefinitionValue<'tree>,
    pub children: ::std::vec::Vec<VarDefinitionChildren<'tree>>,
}
impl<'tree> ::treesitter_types::FromNode<'tree> for VarDefinition<'tree> {
    #[allow(clippy::match_single_binding, clippy::suspicious_else_formatting)]
    fn from_node(
        node: ::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        debug_assert_eq!(node.kind(), "var_definition");
        Ok(Self {
            span: ::treesitter_types::Span::from(node),
            pattern: {
                let child = node.child_by_field_name("pattern").ok_or_else(|| {
                    ::treesitter_types::ParseError::missing_field("pattern", node)
                })?;
                <VarDefinitionPattern as ::treesitter_types::FromNode>::from_node(child, src)?
            },
            r#type: match node.child_by_field_name("type") {
                Some(child) => Some(
                    <VarDefinitionType as ::treesitter_types::FromNode>::from_node(child, src)?,
                ),
                None => None,
            },
            value: {
                let child = node
                    .child_by_field_name("value")
                    .ok_or_else(|| ::treesitter_types::ParseError::missing_field("value", node))?;
                <VarDefinitionValue as ::treesitter_types::FromNode>::from_node(child, src)?
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
                        <VarDefinitionChildren as ::treesitter_types::FromNode>::from_node(
                            child, src,
                        )?,
                    );
                }
                items
            },
        })
    }
}
impl ::treesitter_types::Spanned for VarDefinition<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        self.span
    }
}
#[derive(Debug, Clone)]
pub struct ViewBound<'tree> {
    pub span: ::treesitter_types::Span,
    pub r#type: ViewBoundType<'tree>,
}
impl<'tree> ::treesitter_types::FromNode<'tree> for ViewBound<'tree> {
    #[allow(clippy::match_single_binding, clippy::suspicious_else_formatting)]
    fn from_node(
        node: ::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        debug_assert_eq!(node.kind(), "view_bound");
        Ok(Self {
            span: ::treesitter_types::Span::from(node),
            r#type: {
                let child = node
                    .child_by_field_name("type")
                    .ok_or_else(|| ::treesitter_types::ParseError::missing_field("type", node))?;
                <ViewBoundType as ::treesitter_types::FromNode>::from_node(child, src)?
            },
        })
    }
}
impl ::treesitter_types::Spanned for ViewBound<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        self.span
    }
}
#[derive(Debug, Clone)]
pub struct WhileExpression<'tree> {
    pub span: ::treesitter_types::Span,
    pub body: WhileExpressionBody<'tree>,
    pub condition: ::std::vec::Vec<WhileExpressionCondition<'tree>>,
}
impl<'tree> ::treesitter_types::FromNode<'tree> for WhileExpression<'tree> {
    #[allow(clippy::match_single_binding, clippy::suspicious_else_formatting)]
    fn from_node(
        node: ::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        debug_assert_eq!(node.kind(), "while_expression");
        Ok(Self {
            span: ::treesitter_types::Span::from(node),
            body: {
                let child = node
                    .child_by_field_name("body")
                    .ok_or_else(|| ::treesitter_types::ParseError::missing_field("body", node))?;
                <WhileExpressionBody as ::treesitter_types::FromNode>::from_node(child, src)?
            },
            condition: {
                let mut cursor = node.walk();
                let mut items = ::std::vec::Vec::new();
                for child in node.children_by_field_name("condition", &mut cursor) {
                    items.push(
                        <WhileExpressionCondition as ::treesitter_types::FromNode>::from_node(
                            child, src,
                        )?,
                    );
                }
                items
            },
        })
    }
}
impl ::treesitter_types::Spanned for WhileExpression<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        self.span
    }
}
#[derive(Debug, Clone)]
pub struct Wildcard<'tree> {
    pub span: ::treesitter_types::Span,
    text: &'tree str,
}
impl<'tree> ::treesitter_types::FromNode<'tree> for Wildcard<'tree> {
    fn from_node(
        node: ::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        debug_assert_eq!(node.kind(), "wildcard");
        Ok(Self {
            span: ::treesitter_types::Span::from(node),
            text: node.utf8_text(src)?,
        })
    }
}
impl<'tree> ::treesitter_types::LeafNode<'tree> for Wildcard<'tree> {
    fn text(&self) -> &'tree str {
        self.text
    }
}
impl ::treesitter_types::Spanned for Wildcard<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        self.span
    }
}
#[derive(Debug, Clone)]
pub struct WithTemplateBody<'tree> {
    pub span: ::treesitter_types::Span,
    pub children: ::std::vec::Vec<WithTemplateBodyChildren<'tree>>,
}
impl<'tree> ::treesitter_types::FromNode<'tree> for WithTemplateBody<'tree> {
    #[allow(clippy::match_single_binding, clippy::suspicious_else_formatting)]
    fn from_node(
        node: ::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        debug_assert_eq!(node.kind(), "with_template_body");
        Ok(Self {
            span: ::treesitter_types::Span::from(node),
            children: {
                #[allow(clippy::suspicious_else_formatting)]
                let non_field_children = {
                    let mut cursor = node.walk();
                    let mut result = ::std::vec::Vec::new();
                    if cursor.goto_first_child() {
                        loop {
                            if cursor.field_name().is_none()
                                && cursor.node().is_named()
                                && !cursor.node().is_extra()
                            {
                                result.push(cursor.node());
                            }
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
                        <WithTemplateBodyChildren as ::treesitter_types::FromNode>::from_node(
                            child, src,
                        )?,
                    );
                }
                items
            },
        })
    }
}
impl ::treesitter_types::Spanned for WithTemplateBody<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        self.span
    }
}
#[derive(Debug, Clone)]
pub struct CharacterLiteral<'tree> {
    pub span: ::treesitter_types::Span,
    text: &'tree str,
}
impl<'tree> ::treesitter_types::FromNode<'tree> for CharacterLiteral<'tree> {
    fn from_node(
        node: ::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        debug_assert_eq!(node.kind(), "character_literal");
        Ok(Self {
            span: ::treesitter_types::Span::from(node),
            text: node.utf8_text(src)?,
        })
    }
}
impl<'tree> ::treesitter_types::LeafNode<'tree> for CharacterLiteral<'tree> {
    fn text(&self) -> &'tree str {
        self.text
    }
}
impl ::treesitter_types::Spanned for CharacterLiteral<'_> {
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
pub struct FloatingPointLiteral<'tree> {
    pub span: ::treesitter_types::Span,
    text: &'tree str,
}
impl<'tree> ::treesitter_types::FromNode<'tree> for FloatingPointLiteral<'tree> {
    fn from_node(
        node: ::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        debug_assert_eq!(node.kind(), "floating_point_literal");
        Ok(Self {
            span: ::treesitter_types::Span::from(node),
            text: node.utf8_text(src)?,
        })
    }
}
impl<'tree> ::treesitter_types::LeafNode<'tree> for FloatingPointLiteral<'tree> {
    fn text(&self) -> &'tree str {
        self.text
    }
}
impl ::treesitter_types::Spanned for FloatingPointLiteral<'_> {
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
#[derive(Debug, Clone)]
pub struct UsingDirectiveKey<'tree> {
    pub span: ::treesitter_types::Span,
    text: &'tree str,
}
impl<'tree> ::treesitter_types::FromNode<'tree> for UsingDirectiveKey<'tree> {
    fn from_node(
        node: ::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        debug_assert_eq!(node.kind(), "using_directive_key");
        Ok(Self {
            span: ::treesitter_types::Span::from(node),
            text: node.utf8_text(src)?,
        })
    }
}
impl<'tree> ::treesitter_types::LeafNode<'tree> for UsingDirectiveKey<'tree> {
    fn text(&self) -> &'tree str {
        self.text
    }
}
impl ::treesitter_types::Spanned for UsingDirectiveKey<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        self.span
    }
}
#[derive(Debug, Clone)]
pub struct UsingDirectiveValue<'tree> {
    pub span: ::treesitter_types::Span,
    text: &'tree str,
}
impl<'tree> ::treesitter_types::FromNode<'tree> for UsingDirectiveValue<'tree> {
    fn from_node(
        node: ::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        debug_assert_eq!(node.kind(), "using_directive_value");
        Ok(Self {
            span: ::treesitter_types::Span::from(node),
            text: node.utf8_text(src)?,
        })
    }
}
impl<'tree> ::treesitter_types::LeafNode<'tree> for UsingDirectiveValue<'tree> {
    fn text(&self) -> &'tree str {
        self.text
    }
}
impl ::treesitter_types::Spanned for UsingDirectiveValue<'_> {
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
pub enum AccessQualifierChildren<'tree> {
    Identifier(::std::boxed::Box<Identifier<'tree>>),
    OperatorIdentifier(::std::boxed::Box<OperatorIdentifier<'tree>>),
}
impl<'tree> ::treesitter_types::FromNode<'tree> for AccessQualifierChildren<'tree> {
    #[allow(clippy::collapsible_else_if)]
    fn from_node(
        node: ::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        match node.kind() {
            "identifier" => Ok(Self::Identifier(::std::boxed::Box::new(
                <Identifier as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "operator_identifier" => Ok(Self::OperatorIdentifier(::std::boxed::Box::new(
                <OperatorIdentifier as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            other => Err(::treesitter_types::ParseError::unexpected_kind(other, node)),
        }
    }
}
impl ::treesitter_types::Spanned for AccessQualifierChildren<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        match self {
            Self::Identifier(inner) => inner.span(),
            Self::OperatorIdentifier(inner) => inner.span(),
        }
    }
}
#[derive(Debug, Clone)]
pub enum AnnotatedTypeChildren<'tree> {
    Annotation(::std::boxed::Box<Annotation<'tree>>),
    AppliedConstructorType(::std::boxed::Box<AppliedConstructorType<'tree>>),
    GenericType(::std::boxed::Box<GenericType<'tree>>),
    NamedTupleType(::std::boxed::Box<NamedTupleType<'tree>>),
    ProjectedType(::std::boxed::Box<ProjectedType<'tree>>),
    SingletonType(::std::boxed::Box<SingletonType<'tree>>),
    StableTypeIdentifier(::std::boxed::Box<StableTypeIdentifier<'tree>>),
    TupleType(::std::boxed::Box<TupleType<'tree>>),
    TypeIdentifier(::std::boxed::Box<TypeIdentifier<'tree>>),
    Wildcard(::std::boxed::Box<Wildcard<'tree>>),
}
impl<'tree> ::treesitter_types::FromNode<'tree> for AnnotatedTypeChildren<'tree> {
    #[allow(clippy::collapsible_else_if)]
    fn from_node(
        node: ::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        match node.kind() {
            "annotation" => Ok(Self::Annotation(::std::boxed::Box::new(
                <Annotation as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "applied_constructor_type" => Ok(Self::AppliedConstructorType(::std::boxed::Box::new(
                <AppliedConstructorType as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "generic_type" => Ok(Self::GenericType(::std::boxed::Box::new(
                <GenericType as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "named_tuple_type" => Ok(Self::NamedTupleType(::std::boxed::Box::new(
                <NamedTupleType as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "projected_type" => Ok(Self::ProjectedType(::std::boxed::Box::new(
                <ProjectedType as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "singleton_type" => Ok(Self::SingletonType(::std::boxed::Box::new(
                <SingletonType as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "stable_type_identifier" => Ok(Self::StableTypeIdentifier(::std::boxed::Box::new(
                <StableTypeIdentifier as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "tuple_type" => Ok(Self::TupleType(::std::boxed::Box::new(
                <TupleType as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "type_identifier" => Ok(Self::TypeIdentifier(::std::boxed::Box::new(
                <TypeIdentifier as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "wildcard" => Ok(Self::Wildcard(::std::boxed::Box::new(
                <Wildcard as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            other => Err(::treesitter_types::ParseError::unexpected_kind(other, node)),
        }
    }
}
impl ::treesitter_types::Spanned for AnnotatedTypeChildren<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        match self {
            Self::Annotation(inner) => inner.span(),
            Self::AppliedConstructorType(inner) => inner.span(),
            Self::GenericType(inner) => inner.span(),
            Self::NamedTupleType(inner) => inner.span(),
            Self::ProjectedType(inner) => inner.span(),
            Self::SingletonType(inner) => inner.span(),
            Self::StableTypeIdentifier(inner) => inner.span(),
            Self::TupleType(inner) => inner.span(),
            Self::TypeIdentifier(inner) => inner.span(),
            Self::Wildcard(inner) => inner.span(),
        }
    }
}
#[derive(Debug, Clone)]
pub enum AnnotationName<'tree> {
    AppliedConstructorType(::std::boxed::Box<AppliedConstructorType<'tree>>),
    GenericType(::std::boxed::Box<GenericType<'tree>>),
    NamedTupleType(::std::boxed::Box<NamedTupleType<'tree>>),
    ProjectedType(::std::boxed::Box<ProjectedType<'tree>>),
    SingletonType(::std::boxed::Box<SingletonType<'tree>>),
    StableTypeIdentifier(::std::boxed::Box<StableTypeIdentifier<'tree>>),
    TupleType(::std::boxed::Box<TupleType<'tree>>),
    TypeIdentifier(::std::boxed::Box<TypeIdentifier<'tree>>),
    Wildcard(::std::boxed::Box<Wildcard<'tree>>),
}
impl<'tree> ::treesitter_types::FromNode<'tree> for AnnotationName<'tree> {
    #[allow(clippy::collapsible_else_if)]
    fn from_node(
        node: ::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        match node.kind() {
            "applied_constructor_type" => Ok(Self::AppliedConstructorType(::std::boxed::Box::new(
                <AppliedConstructorType as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "generic_type" => Ok(Self::GenericType(::std::boxed::Box::new(
                <GenericType as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "named_tuple_type" => Ok(Self::NamedTupleType(::std::boxed::Box::new(
                <NamedTupleType as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "projected_type" => Ok(Self::ProjectedType(::std::boxed::Box::new(
                <ProjectedType as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "singleton_type" => Ok(Self::SingletonType(::std::boxed::Box::new(
                <SingletonType as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "stable_type_identifier" => Ok(Self::StableTypeIdentifier(::std::boxed::Box::new(
                <StableTypeIdentifier as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "tuple_type" => Ok(Self::TupleType(::std::boxed::Box::new(
                <TupleType as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "type_identifier" => Ok(Self::TypeIdentifier(::std::boxed::Box::new(
                <TypeIdentifier as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "wildcard" => Ok(Self::Wildcard(::std::boxed::Box::new(
                <Wildcard as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            other => Err(::treesitter_types::ParseError::unexpected_kind(other, node)),
        }
    }
}
impl ::treesitter_types::Spanned for AnnotationName<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        match self {
            Self::AppliedConstructorType(inner) => inner.span(),
            Self::GenericType(inner) => inner.span(),
            Self::NamedTupleType(inner) => inner.span(),
            Self::ProjectedType(inner) => inner.span(),
            Self::SingletonType(inner) => inner.span(),
            Self::StableTypeIdentifier(inner) => inner.span(),
            Self::TupleType(inner) => inner.span(),
            Self::TypeIdentifier(inner) => inner.span(),
            Self::Wildcard(inner) => inner.span(),
        }
    }
}
#[derive(Debug, Clone)]
pub enum AppliedConstructorTypeChildren<'tree> {
    Arguments(::std::boxed::Box<Arguments<'tree>>),
    TypeIdentifier(::std::boxed::Box<TypeIdentifier<'tree>>),
}
impl<'tree> ::treesitter_types::FromNode<'tree> for AppliedConstructorTypeChildren<'tree> {
    #[allow(clippy::collapsible_else_if)]
    fn from_node(
        node: ::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        match node.kind() {
            "arguments" => Ok(Self::Arguments(::std::boxed::Box::new(
                <Arguments as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "type_identifier" => Ok(Self::TypeIdentifier(::std::boxed::Box::new(
                <TypeIdentifier as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            other => Err(::treesitter_types::ParseError::unexpected_kind(other, node)),
        }
    }
}
impl ::treesitter_types::Spanned for AppliedConstructorTypeChildren<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        match self {
            Self::Arguments(inner) => inner.span(),
            Self::TypeIdentifier(inner) => inner.span(),
        }
    }
}
#[derive(Debug, Clone)]
pub enum ArrowRenamedIdentifierAlias<'tree> {
    Identifier(::std::boxed::Box<Identifier<'tree>>),
    OperatorIdentifier(::std::boxed::Box<OperatorIdentifier<'tree>>),
    Wildcard(::std::boxed::Box<Wildcard<'tree>>),
}
impl<'tree> ::treesitter_types::FromNode<'tree> for ArrowRenamedIdentifierAlias<'tree> {
    #[allow(clippy::collapsible_else_if)]
    fn from_node(
        node: ::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        match node.kind() {
            "identifier" => Ok(Self::Identifier(::std::boxed::Box::new(
                <Identifier as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "operator_identifier" => Ok(Self::OperatorIdentifier(::std::boxed::Box::new(
                <OperatorIdentifier as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "wildcard" => Ok(Self::Wildcard(::std::boxed::Box::new(
                <Wildcard as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            other => Err(::treesitter_types::ParseError::unexpected_kind(other, node)),
        }
    }
}
impl ::treesitter_types::Spanned for ArrowRenamedIdentifierAlias<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        match self {
            Self::Identifier(inner) => inner.span(),
            Self::OperatorIdentifier(inner) => inner.span(),
            Self::Wildcard(inner) => inner.span(),
        }
    }
}
#[derive(Debug, Clone)]
pub enum ArrowRenamedIdentifierName<'tree> {
    Identifier(::std::boxed::Box<Identifier<'tree>>),
    OperatorIdentifier(::std::boxed::Box<OperatorIdentifier<'tree>>),
}
impl<'tree> ::treesitter_types::FromNode<'tree> for ArrowRenamedIdentifierName<'tree> {
    #[allow(clippy::collapsible_else_if)]
    fn from_node(
        node: ::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        match node.kind() {
            "identifier" => Ok(Self::Identifier(::std::boxed::Box::new(
                <Identifier as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "operator_identifier" => Ok(Self::OperatorIdentifier(::std::boxed::Box::new(
                <OperatorIdentifier as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            other => Err(::treesitter_types::ParseError::unexpected_kind(other, node)),
        }
    }
}
impl ::treesitter_types::Spanned for ArrowRenamedIdentifierName<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        match self {
            Self::Identifier(inner) => inner.span(),
            Self::OperatorIdentifier(inner) => inner.span(),
        }
    }
}
#[derive(Debug, Clone)]
pub enum AsRenamedIdentifierAlias<'tree> {
    Identifier(::std::boxed::Box<Identifier<'tree>>),
    OperatorIdentifier(::std::boxed::Box<OperatorIdentifier<'tree>>),
    Wildcard(::std::boxed::Box<Wildcard<'tree>>),
}
impl<'tree> ::treesitter_types::FromNode<'tree> for AsRenamedIdentifierAlias<'tree> {
    #[allow(clippy::collapsible_else_if)]
    fn from_node(
        node: ::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        match node.kind() {
            "identifier" => Ok(Self::Identifier(::std::boxed::Box::new(
                <Identifier as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "operator_identifier" => Ok(Self::OperatorIdentifier(::std::boxed::Box::new(
                <OperatorIdentifier as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "wildcard" => Ok(Self::Wildcard(::std::boxed::Box::new(
                <Wildcard as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            other => Err(::treesitter_types::ParseError::unexpected_kind(other, node)),
        }
    }
}
impl ::treesitter_types::Spanned for AsRenamedIdentifierAlias<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        match self {
            Self::Identifier(inner) => inner.span(),
            Self::OperatorIdentifier(inner) => inner.span(),
            Self::Wildcard(inner) => inner.span(),
        }
    }
}
#[derive(Debug, Clone)]
pub enum AsRenamedIdentifierName<'tree> {
    Identifier(::std::boxed::Box<Identifier<'tree>>),
    OperatorIdentifier(::std::boxed::Box<OperatorIdentifier<'tree>>),
}
impl<'tree> ::treesitter_types::FromNode<'tree> for AsRenamedIdentifierName<'tree> {
    #[allow(clippy::collapsible_else_if)]
    fn from_node(
        node: ::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        match node.kind() {
            "identifier" => Ok(Self::Identifier(::std::boxed::Box::new(
                <Identifier as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "operator_identifier" => Ok(Self::OperatorIdentifier(::std::boxed::Box::new(
                <OperatorIdentifier as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            other => Err(::treesitter_types::ParseError::unexpected_kind(other, node)),
        }
    }
}
impl ::treesitter_types::Spanned for AsRenamedIdentifierName<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        match self {
            Self::Identifier(inner) => inner.span(),
            Self::OperatorIdentifier(inner) => inner.span(),
        }
    }
}
#[derive(Debug, Clone)]
pub enum AscriptionExpressionChildren<'tree> {
    AnnotatedType(::std::boxed::Box<AnnotatedType<'tree>>),
    Annotation(::std::boxed::Box<Annotation<'tree>>),
    AppliedConstructorType(::std::boxed::Box<AppliedConstructorType<'tree>>),
    Block(::std::boxed::Box<Block<'tree>>),
    BooleanLiteral(::std::boxed::Box<BooleanLiteral<'tree>>),
    CallExpression(::std::boxed::Box<CallExpression<'tree>>),
    CaseBlock(::std::boxed::Box<CaseBlock<'tree>>),
    CharacterLiteral(::std::boxed::Box<CharacterLiteral<'tree>>),
    CompoundType(::std::boxed::Box<CompoundType<'tree>>),
    FieldExpression(::std::boxed::Box<FieldExpression<'tree>>),
    FloatingPointLiteral(::std::boxed::Box<FloatingPointLiteral<'tree>>),
    FunctionType(::std::boxed::Box<FunctionType<'tree>>),
    GenericFunction(::std::boxed::Box<GenericFunction<'tree>>),
    GenericType(::std::boxed::Box<GenericType<'tree>>),
    Identifier(::std::boxed::Box<Identifier<'tree>>),
    InfixExpression(::std::boxed::Box<InfixExpression<'tree>>),
    InfixType(::std::boxed::Box<InfixType<'tree>>),
    InstanceExpression(::std::boxed::Box<InstanceExpression<'tree>>),
    IntegerLiteral(::std::boxed::Box<IntegerLiteral<'tree>>),
    InterpolatedStringExpression(::std::boxed::Box<InterpolatedStringExpression<'tree>>),
    LazyParameterType(::std::boxed::Box<LazyParameterType<'tree>>),
    LiteralType(::std::boxed::Box<LiteralType<'tree>>),
    MatchType(::std::boxed::Box<MatchType<'tree>>),
    NamedTupleType(::std::boxed::Box<NamedTupleType<'tree>>),
    NullLiteral(::std::boxed::Box<NullLiteral<'tree>>),
    OperatorIdentifier(::std::boxed::Box<OperatorIdentifier<'tree>>),
    ParenthesizedExpression(::std::boxed::Box<ParenthesizedExpression<'tree>>),
    PostfixExpression(::std::boxed::Box<PostfixExpression<'tree>>),
    PrefixExpression(::std::boxed::Box<PrefixExpression<'tree>>),
    ProjectedType(::std::boxed::Box<ProjectedType<'tree>>),
    QuoteExpression(::std::boxed::Box<QuoteExpression<'tree>>),
    RepeatedParameterType(::std::boxed::Box<RepeatedParameterType<'tree>>),
    SingletonType(::std::boxed::Box<SingletonType<'tree>>),
    SpliceExpression(::std::boxed::Box<SpliceExpression<'tree>>),
    StableTypeIdentifier(::std::boxed::Box<StableTypeIdentifier<'tree>>),
    String(::std::boxed::Box<String<'tree>>),
    StructuralType(::std::boxed::Box<StructuralType<'tree>>),
    TupleExpression(::std::boxed::Box<TupleExpression<'tree>>),
    TupleType(::std::boxed::Box<TupleType<'tree>>),
    TypeIdentifier(::std::boxed::Box<TypeIdentifier<'tree>>),
    TypeLambda(::std::boxed::Box<TypeLambda<'tree>>),
    Unit(::std::boxed::Box<Unit<'tree>>),
    Wildcard(::std::boxed::Box<Wildcard<'tree>>),
}
impl<'tree> ::treesitter_types::FromNode<'tree> for AscriptionExpressionChildren<'tree> {
    #[allow(clippy::collapsible_else_if)]
    fn from_node(
        node: ::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        match node.kind() {
            "annotated_type" => Ok(Self::AnnotatedType(::std::boxed::Box::new(
                <AnnotatedType as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "annotation" => Ok(Self::Annotation(::std::boxed::Box::new(
                <Annotation as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "applied_constructor_type" => Ok(Self::AppliedConstructorType(::std::boxed::Box::new(
                <AppliedConstructorType as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "block" => Ok(Self::Block(::std::boxed::Box::new(
                <Block as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "boolean_literal" => Ok(Self::BooleanLiteral(::std::boxed::Box::new(
                <BooleanLiteral as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "call_expression" => Ok(Self::CallExpression(::std::boxed::Box::new(
                <CallExpression as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "case_block" => Ok(Self::CaseBlock(::std::boxed::Box::new(
                <CaseBlock as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "character_literal" => Ok(Self::CharacterLiteral(::std::boxed::Box::new(
                <CharacterLiteral as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "compound_type" => Ok(Self::CompoundType(::std::boxed::Box::new(
                <CompoundType as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "field_expression" => Ok(Self::FieldExpression(::std::boxed::Box::new(
                <FieldExpression as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "floating_point_literal" => Ok(Self::FloatingPointLiteral(::std::boxed::Box::new(
                <FloatingPointLiteral as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "function_type" => Ok(Self::FunctionType(::std::boxed::Box::new(
                <FunctionType as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "generic_function" => Ok(Self::GenericFunction(::std::boxed::Box::new(
                <GenericFunction as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "generic_type" => Ok(Self::GenericType(::std::boxed::Box::new(
                <GenericType as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "identifier" => Ok(Self::Identifier(::std::boxed::Box::new(
                <Identifier as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "infix_expression" => Ok(Self::InfixExpression(::std::boxed::Box::new(
                <InfixExpression as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "infix_type" => Ok(Self::InfixType(::std::boxed::Box::new(
                <InfixType as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "instance_expression" => Ok(Self::InstanceExpression(::std::boxed::Box::new(
                <InstanceExpression as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "integer_literal" => Ok(Self::IntegerLiteral(::std::boxed::Box::new(
                <IntegerLiteral as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "interpolated_string_expression" => {
                Ok(Self::InterpolatedStringExpression(::std::boxed::Box::new(
                    <InterpolatedStringExpression as ::treesitter_types::FromNode>::from_node(
                        node, src,
                    )?,
                )))
            }
            "lazy_parameter_type" => Ok(Self::LazyParameterType(::std::boxed::Box::new(
                <LazyParameterType as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "literal_type" => Ok(Self::LiteralType(::std::boxed::Box::new(
                <LiteralType as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "match_type" => Ok(Self::MatchType(::std::boxed::Box::new(
                <MatchType as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "named_tuple_type" => Ok(Self::NamedTupleType(::std::boxed::Box::new(
                <NamedTupleType as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "null_literal" => Ok(Self::NullLiteral(::std::boxed::Box::new(
                <NullLiteral as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "operator_identifier" => Ok(Self::OperatorIdentifier(::std::boxed::Box::new(
                <OperatorIdentifier as ::treesitter_types::FromNode>::from_node(node, src)?,
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
            "prefix_expression" => Ok(Self::PrefixExpression(::std::boxed::Box::new(
                <PrefixExpression as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "projected_type" => Ok(Self::ProjectedType(::std::boxed::Box::new(
                <ProjectedType as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "quote_expression" => Ok(Self::QuoteExpression(::std::boxed::Box::new(
                <QuoteExpression as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "repeated_parameter_type" => Ok(Self::RepeatedParameterType(::std::boxed::Box::new(
                <RepeatedParameterType as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "singleton_type" => Ok(Self::SingletonType(::std::boxed::Box::new(
                <SingletonType as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "splice_expression" => Ok(Self::SpliceExpression(::std::boxed::Box::new(
                <SpliceExpression as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "stable_type_identifier" => Ok(Self::StableTypeIdentifier(::std::boxed::Box::new(
                <StableTypeIdentifier as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "string" => Ok(Self::String(::std::boxed::Box::new(
                <String as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "structural_type" => Ok(Self::StructuralType(::std::boxed::Box::new(
                <StructuralType as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "tuple_expression" => Ok(Self::TupleExpression(::std::boxed::Box::new(
                <TupleExpression as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "tuple_type" => Ok(Self::TupleType(::std::boxed::Box::new(
                <TupleType as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "type_identifier" => Ok(Self::TypeIdentifier(::std::boxed::Box::new(
                <TypeIdentifier as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "type_lambda" => Ok(Self::TypeLambda(::std::boxed::Box::new(
                <TypeLambda as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "unit" => Ok(Self::Unit(::std::boxed::Box::new(
                <Unit as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "wildcard" => Ok(Self::Wildcard(::std::boxed::Box::new(
                <Wildcard as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            other => Err(::treesitter_types::ParseError::unexpected_kind(other, node)),
        }
    }
}
impl ::treesitter_types::Spanned for AscriptionExpressionChildren<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        match self {
            Self::AnnotatedType(inner) => inner.span(),
            Self::Annotation(inner) => inner.span(),
            Self::AppliedConstructorType(inner) => inner.span(),
            Self::Block(inner) => inner.span(),
            Self::BooleanLiteral(inner) => inner.span(),
            Self::CallExpression(inner) => inner.span(),
            Self::CaseBlock(inner) => inner.span(),
            Self::CharacterLiteral(inner) => inner.span(),
            Self::CompoundType(inner) => inner.span(),
            Self::FieldExpression(inner) => inner.span(),
            Self::FloatingPointLiteral(inner) => inner.span(),
            Self::FunctionType(inner) => inner.span(),
            Self::GenericFunction(inner) => inner.span(),
            Self::GenericType(inner) => inner.span(),
            Self::Identifier(inner) => inner.span(),
            Self::InfixExpression(inner) => inner.span(),
            Self::InfixType(inner) => inner.span(),
            Self::InstanceExpression(inner) => inner.span(),
            Self::IntegerLiteral(inner) => inner.span(),
            Self::InterpolatedStringExpression(inner) => inner.span(),
            Self::LazyParameterType(inner) => inner.span(),
            Self::LiteralType(inner) => inner.span(),
            Self::MatchType(inner) => inner.span(),
            Self::NamedTupleType(inner) => inner.span(),
            Self::NullLiteral(inner) => inner.span(),
            Self::OperatorIdentifier(inner) => inner.span(),
            Self::ParenthesizedExpression(inner) => inner.span(),
            Self::PostfixExpression(inner) => inner.span(),
            Self::PrefixExpression(inner) => inner.span(),
            Self::ProjectedType(inner) => inner.span(),
            Self::QuoteExpression(inner) => inner.span(),
            Self::RepeatedParameterType(inner) => inner.span(),
            Self::SingletonType(inner) => inner.span(),
            Self::SpliceExpression(inner) => inner.span(),
            Self::StableTypeIdentifier(inner) => inner.span(),
            Self::String(inner) => inner.span(),
            Self::StructuralType(inner) => inner.span(),
            Self::TupleExpression(inner) => inner.span(),
            Self::TupleType(inner) => inner.span(),
            Self::TypeIdentifier(inner) => inner.span(),
            Self::TypeLambda(inner) => inner.span(),
            Self::Unit(inner) => inner.span(),
            Self::Wildcard(inner) => inner.span(),
        }
    }
}
#[derive(Debug, Clone)]
pub enum AssignmentExpressionLeft<'tree> {
    Block(::std::boxed::Box<Block<'tree>>),
    BooleanLiteral(::std::boxed::Box<BooleanLiteral<'tree>>),
    CallExpression(::std::boxed::Box<CallExpression<'tree>>),
    CaseBlock(::std::boxed::Box<CaseBlock<'tree>>),
    CharacterLiteral(::std::boxed::Box<CharacterLiteral<'tree>>),
    FieldExpression(::std::boxed::Box<FieldExpression<'tree>>),
    FloatingPointLiteral(::std::boxed::Box<FloatingPointLiteral<'tree>>),
    GenericFunction(::std::boxed::Box<GenericFunction<'tree>>),
    Identifier(::std::boxed::Box<Identifier<'tree>>),
    InstanceExpression(::std::boxed::Box<InstanceExpression<'tree>>),
    IntegerLiteral(::std::boxed::Box<IntegerLiteral<'tree>>),
    InterpolatedStringExpression(::std::boxed::Box<InterpolatedStringExpression<'tree>>),
    NullLiteral(::std::boxed::Box<NullLiteral<'tree>>),
    OperatorIdentifier(::std::boxed::Box<OperatorIdentifier<'tree>>),
    ParenthesizedExpression(::std::boxed::Box<ParenthesizedExpression<'tree>>),
    PrefixExpression(::std::boxed::Box<PrefixExpression<'tree>>),
    QuoteExpression(::std::boxed::Box<QuoteExpression<'tree>>),
    SpliceExpression(::std::boxed::Box<SpliceExpression<'tree>>),
    String(::std::boxed::Box<String<'tree>>),
    TupleExpression(::std::boxed::Box<TupleExpression<'tree>>),
    Unit(::std::boxed::Box<Unit<'tree>>),
    Wildcard(::std::boxed::Box<Wildcard<'tree>>),
}
impl<'tree> ::treesitter_types::FromNode<'tree> for AssignmentExpressionLeft<'tree> {
    #[allow(clippy::collapsible_else_if)]
    fn from_node(
        node: ::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        match node.kind() {
            "block" => Ok(Self::Block(::std::boxed::Box::new(
                <Block as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "boolean_literal" => Ok(Self::BooleanLiteral(::std::boxed::Box::new(
                <BooleanLiteral as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "call_expression" => Ok(Self::CallExpression(::std::boxed::Box::new(
                <CallExpression as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "case_block" => Ok(Self::CaseBlock(::std::boxed::Box::new(
                <CaseBlock as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "character_literal" => Ok(Self::CharacterLiteral(::std::boxed::Box::new(
                <CharacterLiteral as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "field_expression" => Ok(Self::FieldExpression(::std::boxed::Box::new(
                <FieldExpression as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "floating_point_literal" => Ok(Self::FloatingPointLiteral(::std::boxed::Box::new(
                <FloatingPointLiteral as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "generic_function" => Ok(Self::GenericFunction(::std::boxed::Box::new(
                <GenericFunction as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "identifier" => Ok(Self::Identifier(::std::boxed::Box::new(
                <Identifier as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "instance_expression" => Ok(Self::InstanceExpression(::std::boxed::Box::new(
                <InstanceExpression as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "integer_literal" => Ok(Self::IntegerLiteral(::std::boxed::Box::new(
                <IntegerLiteral as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "interpolated_string_expression" => {
                Ok(Self::InterpolatedStringExpression(::std::boxed::Box::new(
                    <InterpolatedStringExpression as ::treesitter_types::FromNode>::from_node(
                        node, src,
                    )?,
                )))
            }
            "null_literal" => Ok(Self::NullLiteral(::std::boxed::Box::new(
                <NullLiteral as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "operator_identifier" => Ok(Self::OperatorIdentifier(::std::boxed::Box::new(
                <OperatorIdentifier as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "parenthesized_expression" => {
                Ok(Self::ParenthesizedExpression(::std::boxed::Box::new(
                    <ParenthesizedExpression as ::treesitter_types::FromNode>::from_node(
                        node, src,
                    )?,
                )))
            }
            "prefix_expression" => Ok(Self::PrefixExpression(::std::boxed::Box::new(
                <PrefixExpression as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "quote_expression" => Ok(Self::QuoteExpression(::std::boxed::Box::new(
                <QuoteExpression as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "splice_expression" => Ok(Self::SpliceExpression(::std::boxed::Box::new(
                <SpliceExpression as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "string" => Ok(Self::String(::std::boxed::Box::new(
                <String as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "tuple_expression" => Ok(Self::TupleExpression(::std::boxed::Box::new(
                <TupleExpression as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "unit" => Ok(Self::Unit(::std::boxed::Box::new(
                <Unit as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "wildcard" => Ok(Self::Wildcard(::std::boxed::Box::new(
                <Wildcard as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            other => Err(::treesitter_types::ParseError::unexpected_kind(other, node)),
        }
    }
}
impl ::treesitter_types::Spanned for AssignmentExpressionLeft<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        match self {
            Self::Block(inner) => inner.span(),
            Self::BooleanLiteral(inner) => inner.span(),
            Self::CallExpression(inner) => inner.span(),
            Self::CaseBlock(inner) => inner.span(),
            Self::CharacterLiteral(inner) => inner.span(),
            Self::FieldExpression(inner) => inner.span(),
            Self::FloatingPointLiteral(inner) => inner.span(),
            Self::GenericFunction(inner) => inner.span(),
            Self::Identifier(inner) => inner.span(),
            Self::InstanceExpression(inner) => inner.span(),
            Self::IntegerLiteral(inner) => inner.span(),
            Self::InterpolatedStringExpression(inner) => inner.span(),
            Self::NullLiteral(inner) => inner.span(),
            Self::OperatorIdentifier(inner) => inner.span(),
            Self::ParenthesizedExpression(inner) => inner.span(),
            Self::PrefixExpression(inner) => inner.span(),
            Self::QuoteExpression(inner) => inner.span(),
            Self::SpliceExpression(inner) => inner.span(),
            Self::String(inner) => inner.span(),
            Self::TupleExpression(inner) => inner.span(),
            Self::Unit(inner) => inner.span(),
            Self::Wildcard(inner) => inner.span(),
        }
    }
}
#[derive(Debug, Clone)]
pub enum BindingName<'tree> {
    Identifier(::std::boxed::Box<Identifier<'tree>>),
    OperatorIdentifier(::std::boxed::Box<OperatorIdentifier<'tree>>),
}
impl<'tree> ::treesitter_types::FromNode<'tree> for BindingName<'tree> {
    #[allow(clippy::collapsible_else_if)]
    fn from_node(
        node: ::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        match node.kind() {
            "identifier" => Ok(Self::Identifier(::std::boxed::Box::new(
                <Identifier as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "operator_identifier" => Ok(Self::OperatorIdentifier(::std::boxed::Box::new(
                <OperatorIdentifier as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            other => Err(::treesitter_types::ParseError::unexpected_kind(other, node)),
        }
    }
}
impl ::treesitter_types::Spanned for BindingName<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        match self {
            Self::Identifier(inner) => inner.span(),
            Self::OperatorIdentifier(inner) => inner.span(),
        }
    }
}
#[derive(Debug, Clone)]
pub enum BindingType<'tree> {
    AnnotatedType(::std::boxed::Box<AnnotatedType<'tree>>),
    AppliedConstructorType(::std::boxed::Box<AppliedConstructorType<'tree>>),
    CompoundType(::std::boxed::Box<CompoundType<'tree>>),
    FunctionType(::std::boxed::Box<FunctionType<'tree>>),
    GenericType(::std::boxed::Box<GenericType<'tree>>),
    InfixType(::std::boxed::Box<InfixType<'tree>>),
    LazyParameterType(::std::boxed::Box<LazyParameterType<'tree>>),
    LiteralType(::std::boxed::Box<LiteralType<'tree>>),
    MatchType(::std::boxed::Box<MatchType<'tree>>),
    NamedTupleType(::std::boxed::Box<NamedTupleType<'tree>>),
    ProjectedType(::std::boxed::Box<ProjectedType<'tree>>),
    RepeatedParameterType(::std::boxed::Box<RepeatedParameterType<'tree>>),
    SingletonType(::std::boxed::Box<SingletonType<'tree>>),
    StableTypeIdentifier(::std::boxed::Box<StableTypeIdentifier<'tree>>),
    StructuralType(::std::boxed::Box<StructuralType<'tree>>),
    TupleType(::std::boxed::Box<TupleType<'tree>>),
    TypeIdentifier(::std::boxed::Box<TypeIdentifier<'tree>>),
    TypeLambda(::std::boxed::Box<TypeLambda<'tree>>),
    Wildcard(::std::boxed::Box<Wildcard<'tree>>),
}
impl<'tree> ::treesitter_types::FromNode<'tree> for BindingType<'tree> {
    #[allow(clippy::collapsible_else_if)]
    fn from_node(
        node: ::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        match node.kind() {
            "annotated_type" => Ok(Self::AnnotatedType(::std::boxed::Box::new(
                <AnnotatedType as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "applied_constructor_type" => Ok(Self::AppliedConstructorType(::std::boxed::Box::new(
                <AppliedConstructorType as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "compound_type" => Ok(Self::CompoundType(::std::boxed::Box::new(
                <CompoundType as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "function_type" => Ok(Self::FunctionType(::std::boxed::Box::new(
                <FunctionType as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "generic_type" => Ok(Self::GenericType(::std::boxed::Box::new(
                <GenericType as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "infix_type" => Ok(Self::InfixType(::std::boxed::Box::new(
                <InfixType as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "lazy_parameter_type" => Ok(Self::LazyParameterType(::std::boxed::Box::new(
                <LazyParameterType as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "literal_type" => Ok(Self::LiteralType(::std::boxed::Box::new(
                <LiteralType as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "match_type" => Ok(Self::MatchType(::std::boxed::Box::new(
                <MatchType as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "named_tuple_type" => Ok(Self::NamedTupleType(::std::boxed::Box::new(
                <NamedTupleType as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "projected_type" => Ok(Self::ProjectedType(::std::boxed::Box::new(
                <ProjectedType as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "repeated_parameter_type" => Ok(Self::RepeatedParameterType(::std::boxed::Box::new(
                <RepeatedParameterType as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "singleton_type" => Ok(Self::SingletonType(::std::boxed::Box::new(
                <SingletonType as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "stable_type_identifier" => Ok(Self::StableTypeIdentifier(::std::boxed::Box::new(
                <StableTypeIdentifier as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "structural_type" => Ok(Self::StructuralType(::std::boxed::Box::new(
                <StructuralType as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "tuple_type" => Ok(Self::TupleType(::std::boxed::Box::new(
                <TupleType as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "type_identifier" => Ok(Self::TypeIdentifier(::std::boxed::Box::new(
                <TypeIdentifier as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "type_lambda" => Ok(Self::TypeLambda(::std::boxed::Box::new(
                <TypeLambda as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "wildcard" => Ok(Self::Wildcard(::std::boxed::Box::new(
                <Wildcard as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            other => Err(::treesitter_types::ParseError::unexpected_kind(other, node)),
        }
    }
}
impl ::treesitter_types::Spanned for BindingType<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        match self {
            Self::AnnotatedType(inner) => inner.span(),
            Self::AppliedConstructorType(inner) => inner.span(),
            Self::CompoundType(inner) => inner.span(),
            Self::FunctionType(inner) => inner.span(),
            Self::GenericType(inner) => inner.span(),
            Self::InfixType(inner) => inner.span(),
            Self::LazyParameterType(inner) => inner.span(),
            Self::LiteralType(inner) => inner.span(),
            Self::MatchType(inner) => inner.span(),
            Self::NamedTupleType(inner) => inner.span(),
            Self::ProjectedType(inner) => inner.span(),
            Self::RepeatedParameterType(inner) => inner.span(),
            Self::SingletonType(inner) => inner.span(),
            Self::StableTypeIdentifier(inner) => inner.span(),
            Self::StructuralType(inner) => inner.span(),
            Self::TupleType(inner) => inner.span(),
            Self::TypeIdentifier(inner) => inner.span(),
            Self::TypeLambda(inner) => inner.span(),
            Self::Wildcard(inner) => inner.span(),
        }
    }
}
#[derive(Debug, Clone)]
pub enum BlockChildren<'tree> {
    Definition(::std::boxed::Box<Definition<'tree>>),
    Expression(::std::boxed::Box<Expression<'tree>>),
}
impl<'tree> ::treesitter_types::FromNode<'tree> for BlockChildren<'tree> {
    #[allow(clippy::collapsible_else_if)]
    fn from_node(
        node: ::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        if let Ok(v) = <Definition as ::treesitter_types::FromNode>::from_node(node, src) {
            Ok(Self::Definition(::std::boxed::Box::new(v)))
        } else {
            if let Ok(v) = <Expression as ::treesitter_types::FromNode>::from_node(node, src) {
                Ok(Self::Expression(::std::boxed::Box::new(v)))
            } else {
                Err(::treesitter_types::ParseError::unexpected_kind(
                    node.kind(),
                    node,
                ))
            }
        }
    }
}
impl ::treesitter_types::Spanned for BlockChildren<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        match self {
            Self::Definition(inner) => inner.span(),
            Self::Expression(inner) => inner.span(),
        }
    }
}
#[derive(Debug, Clone)]
pub enum CallExpressionArguments<'tree> {
    Arguments(::std::boxed::Box<Arguments<'tree>>),
    Block(::std::boxed::Box<Block<'tree>>),
    CaseBlock(::std::boxed::Box<CaseBlock<'tree>>),
    ColonArgument(::std::boxed::Box<ColonArgument<'tree>>),
}
impl<'tree> ::treesitter_types::FromNode<'tree> for CallExpressionArguments<'tree> {
    #[allow(clippy::collapsible_else_if)]
    fn from_node(
        node: ::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        match node.kind() {
            "arguments" => Ok(Self::Arguments(::std::boxed::Box::new(
                <Arguments as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "block" => Ok(Self::Block(::std::boxed::Box::new(
                <Block as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "case_block" => Ok(Self::CaseBlock(::std::boxed::Box::new(
                <CaseBlock as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "colon_argument" => Ok(Self::ColonArgument(::std::boxed::Box::new(
                <ColonArgument as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            other => Err(::treesitter_types::ParseError::unexpected_kind(other, node)),
        }
    }
}
impl ::treesitter_types::Spanned for CallExpressionArguments<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        match self {
            Self::Arguments(inner) => inner.span(),
            Self::Block(inner) => inner.span(),
            Self::CaseBlock(inner) => inner.span(),
            Self::ColonArgument(inner) => inner.span(),
        }
    }
}
#[derive(Debug, Clone)]
pub enum CallExpressionFunction<'tree> {
    Block(::std::boxed::Box<Block<'tree>>),
    BooleanLiteral(::std::boxed::Box<BooleanLiteral<'tree>>),
    CallExpression(::std::boxed::Box<CallExpression<'tree>>),
    CaseBlock(::std::boxed::Box<CaseBlock<'tree>>),
    CharacterLiteral(::std::boxed::Box<CharacterLiteral<'tree>>),
    FieldExpression(::std::boxed::Box<FieldExpression<'tree>>),
    FloatingPointLiteral(::std::boxed::Box<FloatingPointLiteral<'tree>>),
    GenericFunction(::std::boxed::Box<GenericFunction<'tree>>),
    Identifier(::std::boxed::Box<Identifier<'tree>>),
    InfixExpression(::std::boxed::Box<InfixExpression<'tree>>),
    InstanceExpression(::std::boxed::Box<InstanceExpression<'tree>>),
    IntegerLiteral(::std::boxed::Box<IntegerLiteral<'tree>>),
    InterpolatedStringExpression(::std::boxed::Box<InterpolatedStringExpression<'tree>>),
    NullLiteral(::std::boxed::Box<NullLiteral<'tree>>),
    OperatorIdentifier(::std::boxed::Box<OperatorIdentifier<'tree>>),
    ParenthesizedExpression(::std::boxed::Box<ParenthesizedExpression<'tree>>),
    PostfixExpression(::std::boxed::Box<PostfixExpression<'tree>>),
    PrefixExpression(::std::boxed::Box<PrefixExpression<'tree>>),
    QuoteExpression(::std::boxed::Box<QuoteExpression<'tree>>),
    SpliceExpression(::std::boxed::Box<SpliceExpression<'tree>>),
    String(::std::boxed::Box<String<'tree>>),
    TupleExpression(::std::boxed::Box<TupleExpression<'tree>>),
    Unit(::std::boxed::Box<Unit<'tree>>),
    Wildcard(::std::boxed::Box<Wildcard<'tree>>),
}
impl<'tree> ::treesitter_types::FromNode<'tree> for CallExpressionFunction<'tree> {
    #[allow(clippy::collapsible_else_if)]
    fn from_node(
        node: ::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        match node.kind() {
            "block" => Ok(Self::Block(::std::boxed::Box::new(
                <Block as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "boolean_literal" => Ok(Self::BooleanLiteral(::std::boxed::Box::new(
                <BooleanLiteral as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "call_expression" => Ok(Self::CallExpression(::std::boxed::Box::new(
                <CallExpression as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "case_block" => Ok(Self::CaseBlock(::std::boxed::Box::new(
                <CaseBlock as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "character_literal" => Ok(Self::CharacterLiteral(::std::boxed::Box::new(
                <CharacterLiteral as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "field_expression" => Ok(Self::FieldExpression(::std::boxed::Box::new(
                <FieldExpression as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "floating_point_literal" => Ok(Self::FloatingPointLiteral(::std::boxed::Box::new(
                <FloatingPointLiteral as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "generic_function" => Ok(Self::GenericFunction(::std::boxed::Box::new(
                <GenericFunction as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "identifier" => Ok(Self::Identifier(::std::boxed::Box::new(
                <Identifier as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "infix_expression" => Ok(Self::InfixExpression(::std::boxed::Box::new(
                <InfixExpression as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "instance_expression" => Ok(Self::InstanceExpression(::std::boxed::Box::new(
                <InstanceExpression as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "integer_literal" => Ok(Self::IntegerLiteral(::std::boxed::Box::new(
                <IntegerLiteral as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "interpolated_string_expression" => {
                Ok(Self::InterpolatedStringExpression(::std::boxed::Box::new(
                    <InterpolatedStringExpression as ::treesitter_types::FromNode>::from_node(
                        node, src,
                    )?,
                )))
            }
            "null_literal" => Ok(Self::NullLiteral(::std::boxed::Box::new(
                <NullLiteral as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "operator_identifier" => Ok(Self::OperatorIdentifier(::std::boxed::Box::new(
                <OperatorIdentifier as ::treesitter_types::FromNode>::from_node(node, src)?,
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
            "prefix_expression" => Ok(Self::PrefixExpression(::std::boxed::Box::new(
                <PrefixExpression as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "quote_expression" => Ok(Self::QuoteExpression(::std::boxed::Box::new(
                <QuoteExpression as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "splice_expression" => Ok(Self::SpliceExpression(::std::boxed::Box::new(
                <SpliceExpression as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "string" => Ok(Self::String(::std::boxed::Box::new(
                <String as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "tuple_expression" => Ok(Self::TupleExpression(::std::boxed::Box::new(
                <TupleExpression as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "unit" => Ok(Self::Unit(::std::boxed::Box::new(
                <Unit as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "wildcard" => Ok(Self::Wildcard(::std::boxed::Box::new(
                <Wildcard as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            other => Err(::treesitter_types::ParseError::unexpected_kind(other, node)),
        }
    }
}
impl ::treesitter_types::Spanned for CallExpressionFunction<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        match self {
            Self::Block(inner) => inner.span(),
            Self::BooleanLiteral(inner) => inner.span(),
            Self::CallExpression(inner) => inner.span(),
            Self::CaseBlock(inner) => inner.span(),
            Self::CharacterLiteral(inner) => inner.span(),
            Self::FieldExpression(inner) => inner.span(),
            Self::FloatingPointLiteral(inner) => inner.span(),
            Self::GenericFunction(inner) => inner.span(),
            Self::Identifier(inner) => inner.span(),
            Self::InfixExpression(inner) => inner.span(),
            Self::InstanceExpression(inner) => inner.span(),
            Self::IntegerLiteral(inner) => inner.span(),
            Self::InterpolatedStringExpression(inner) => inner.span(),
            Self::NullLiteral(inner) => inner.span(),
            Self::OperatorIdentifier(inner) => inner.span(),
            Self::ParenthesizedExpression(inner) => inner.span(),
            Self::PostfixExpression(inner) => inner.span(),
            Self::PrefixExpression(inner) => inner.span(),
            Self::QuoteExpression(inner) => inner.span(),
            Self::SpliceExpression(inner) => inner.span(),
            Self::String(inner) => inner.span(),
            Self::TupleExpression(inner) => inner.span(),
            Self::Unit(inner) => inner.span(),
            Self::Wildcard(inner) => inner.span(),
        }
    }
}
#[derive(Debug, Clone)]
pub enum CapturePatternName<'tree> {
    Identifier(::std::boxed::Box<Identifier<'tree>>),
    OperatorIdentifier(::std::boxed::Box<OperatorIdentifier<'tree>>),
    Wildcard(::std::boxed::Box<Wildcard<'tree>>),
}
impl<'tree> ::treesitter_types::FromNode<'tree> for CapturePatternName<'tree> {
    #[allow(clippy::collapsible_else_if)]
    fn from_node(
        node: ::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        match node.kind() {
            "identifier" => Ok(Self::Identifier(::std::boxed::Box::new(
                <Identifier as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "operator_identifier" => Ok(Self::OperatorIdentifier(::std::boxed::Box::new(
                <OperatorIdentifier as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "wildcard" => Ok(Self::Wildcard(::std::boxed::Box::new(
                <Wildcard as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            other => Err(::treesitter_types::ParseError::unexpected_kind(other, node)),
        }
    }
}
impl ::treesitter_types::Spanned for CapturePatternName<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        match self {
            Self::Identifier(inner) => inner.span(),
            Self::OperatorIdentifier(inner) => inner.span(),
            Self::Wildcard(inner) => inner.span(),
        }
    }
}
#[derive(Debug, Clone)]
pub enum CaseClassPatternPattern<'tree> {
    Comma(::treesitter_types::Span),
    Pattern(::std::boxed::Box<Pattern<'tree>>),
    NamedPattern(::std::boxed::Box<NamedPattern<'tree>>),
}
impl<'tree> ::treesitter_types::FromNode<'tree> for CaseClassPatternPattern<'tree> {
    #[allow(clippy::collapsible_else_if)]
    fn from_node(
        node: ::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        match node.kind() {
            "," => Ok(Self::Comma(::treesitter_types::Span::from(node))),
            "named_pattern" => Ok(Self::NamedPattern(::std::boxed::Box::new(
                <NamedPattern as ::treesitter_types::FromNode>::from_node(node, src)?,
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
impl ::treesitter_types::Spanned for CaseClassPatternPattern<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        match self {
            Self::Comma(span) => *span,
            Self::Pattern(inner) => inner.span(),
            Self::NamedPattern(inner) => inner.span(),
        }
    }
}
#[derive(Debug, Clone)]
pub enum CaseClassPatternType<'tree> {
    StableTypeIdentifier(::std::boxed::Box<StableTypeIdentifier<'tree>>),
    TypeIdentifier(::std::boxed::Box<TypeIdentifier<'tree>>),
}
impl<'tree> ::treesitter_types::FromNode<'tree> for CaseClassPatternType<'tree> {
    #[allow(clippy::collapsible_else_if)]
    fn from_node(
        node: ::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        match node.kind() {
            "stable_type_identifier" => Ok(Self::StableTypeIdentifier(::std::boxed::Box::new(
                <StableTypeIdentifier as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "type_identifier" => Ok(Self::TypeIdentifier(::std::boxed::Box::new(
                <TypeIdentifier as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            other => Err(::treesitter_types::ParseError::unexpected_kind(other, node)),
        }
    }
}
impl ::treesitter_types::Spanned for CaseClassPatternType<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        match self {
            Self::StableTypeIdentifier(inner) => inner.span(),
            Self::TypeIdentifier(inner) => inner.span(),
        }
    }
}
#[derive(Debug, Clone)]
pub enum CaseClauseBody<'tree> {
    Semicolon(::treesitter_types::Span),
    Definition(::std::boxed::Box<Definition<'tree>>),
    EndIdent(::treesitter_types::Span),
    End(::treesitter_types::Span),
    Expression(::std::boxed::Box<Expression<'tree>>),
    Extension(::treesitter_types::Span),
    For(::treesitter_types::Span),
    Given(::treesitter_types::Span),
    If(::treesitter_types::Span),
    Match(::treesitter_types::Span),
    New(::treesitter_types::Span),
    This(::treesitter_types::Span),
    Try(::treesitter_types::Span),
    Val(::treesitter_types::Span),
    While(::treesitter_types::Span),
}
impl<'tree> ::treesitter_types::FromNode<'tree> for CaseClauseBody<'tree> {
    #[allow(clippy::collapsible_else_if)]
    fn from_node(
        node: ::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        match node.kind() {
            ";" => Ok(Self::Semicolon(::treesitter_types::Span::from(node))),
            "_end_ident" => Ok(Self::EndIdent(::treesitter_types::Span::from(node))),
            "end" => Ok(Self::End(::treesitter_types::Span::from(node))),
            "extension" => Ok(Self::Extension(::treesitter_types::Span::from(node))),
            "for" => Ok(Self::For(::treesitter_types::Span::from(node))),
            "given" => Ok(Self::Given(::treesitter_types::Span::from(node))),
            "if" => Ok(Self::If(::treesitter_types::Span::from(node))),
            "match" => Ok(Self::Match(::treesitter_types::Span::from(node))),
            "new" => Ok(Self::New(::treesitter_types::Span::from(node))),
            "this" => Ok(Self::This(::treesitter_types::Span::from(node))),
            "try" => Ok(Self::Try(::treesitter_types::Span::from(node))),
            "val" => Ok(Self::Val(::treesitter_types::Span::from(node))),
            "while" => Ok(Self::While(::treesitter_types::Span::from(node))),
            _other => {
                if let Ok(v) = <Definition as ::treesitter_types::FromNode>::from_node(node, src) {
                    Ok(Self::Definition(::std::boxed::Box::new(v)))
                } else {
                    if let Ok(v) =
                        <Expression as ::treesitter_types::FromNode>::from_node(node, src)
                    {
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
}
impl ::treesitter_types::Spanned for CaseClauseBody<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        match self {
            Self::Semicolon(span) => *span,
            Self::Definition(inner) => inner.span(),
            Self::EndIdent(span) => *span,
            Self::End(span) => *span,
            Self::Expression(inner) => inner.span(),
            Self::Extension(span) => *span,
            Self::For(span) => *span,
            Self::Given(span) => *span,
            Self::If(span) => *span,
            Self::Match(span) => *span,
            Self::New(span) => *span,
            Self::This(span) => *span,
            Self::Try(span) => *span,
            Self::Val(span) => *span,
            Self::While(span) => *span,
        }
    }
}
#[derive(Debug, Clone)]
pub enum CatchClauseChildren<'tree> {
    Expression(::std::boxed::Box<Expression<'tree>>),
    Guard(::std::boxed::Box<Guard<'tree>>),
    IndentedBlock(::std::boxed::Box<IndentedBlock<'tree>>),
    IndentedCases(::std::boxed::Box<IndentedCases<'tree>>),
}
impl<'tree> ::treesitter_types::FromNode<'tree> for CatchClauseChildren<'tree> {
    #[allow(clippy::collapsible_else_if)]
    fn from_node(
        node: ::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        match node.kind() {
            "guard" => Ok(Self::Guard(::std::boxed::Box::new(
                <Guard as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "indented_block" => Ok(Self::IndentedBlock(::std::boxed::Box::new(
                <IndentedBlock as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "indented_cases" => Ok(Self::IndentedCases(::std::boxed::Box::new(
                <IndentedCases as ::treesitter_types::FromNode>::from_node(node, src)?,
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
impl ::treesitter_types::Spanned for CatchClauseChildren<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        match self {
            Self::Expression(inner) => inner.span(),
            Self::Guard(inner) => inner.span(),
            Self::IndentedBlock(inner) => inner.span(),
            Self::IndentedCases(inner) => inner.span(),
        }
    }
}
#[derive(Debug, Clone)]
pub enum ClassDefinitionName<'tree> {
    Identifier(::std::boxed::Box<Identifier<'tree>>),
    OperatorIdentifier(::std::boxed::Box<OperatorIdentifier<'tree>>),
}
impl<'tree> ::treesitter_types::FromNode<'tree> for ClassDefinitionName<'tree> {
    #[allow(clippy::collapsible_else_if)]
    fn from_node(
        node: ::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        match node.kind() {
            "identifier" => Ok(Self::Identifier(::std::boxed::Box::new(
                <Identifier as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "operator_identifier" => Ok(Self::OperatorIdentifier(::std::boxed::Box::new(
                <OperatorIdentifier as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            other => Err(::treesitter_types::ParseError::unexpected_kind(other, node)),
        }
    }
}
impl ::treesitter_types::Spanned for ClassDefinitionName<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        match self {
            Self::Identifier(inner) => inner.span(),
            Self::OperatorIdentifier(inner) => inner.span(),
        }
    }
}
#[derive(Debug, Clone)]
pub enum ClassDefinitionChildren<'tree> {
    AccessModifier(::std::boxed::Box<AccessModifier<'tree>>),
    Annotation(::std::boxed::Box<Annotation<'tree>>),
    Modifiers(::std::boxed::Box<Modifiers<'tree>>),
}
impl<'tree> ::treesitter_types::FromNode<'tree> for ClassDefinitionChildren<'tree> {
    #[allow(clippy::collapsible_else_if)]
    fn from_node(
        node: ::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        match node.kind() {
            "access_modifier" => Ok(Self::AccessModifier(::std::boxed::Box::new(
                <AccessModifier as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "annotation" => Ok(Self::Annotation(::std::boxed::Box::new(
                <Annotation as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "modifiers" => Ok(Self::Modifiers(::std::boxed::Box::new(
                <Modifiers as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            other => Err(::treesitter_types::ParseError::unexpected_kind(other, node)),
        }
    }
}
impl ::treesitter_types::Spanned for ClassDefinitionChildren<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        match self {
            Self::AccessModifier(inner) => inner.span(),
            Self::Annotation(inner) => inner.span(),
            Self::Modifiers(inner) => inner.span(),
        }
    }
}
#[derive(Debug, Clone)]
pub enum ClassParameterName<'tree> {
    Identifier(::std::boxed::Box<Identifier<'tree>>),
    OperatorIdentifier(::std::boxed::Box<OperatorIdentifier<'tree>>),
}
impl<'tree> ::treesitter_types::FromNode<'tree> for ClassParameterName<'tree> {
    #[allow(clippy::collapsible_else_if)]
    fn from_node(
        node: ::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        match node.kind() {
            "identifier" => Ok(Self::Identifier(::std::boxed::Box::new(
                <Identifier as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "operator_identifier" => Ok(Self::OperatorIdentifier(::std::boxed::Box::new(
                <OperatorIdentifier as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            other => Err(::treesitter_types::ParseError::unexpected_kind(other, node)),
        }
    }
}
impl ::treesitter_types::Spanned for ClassParameterName<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        match self {
            Self::Identifier(inner) => inner.span(),
            Self::OperatorIdentifier(inner) => inner.span(),
        }
    }
}
#[derive(Debug, Clone)]
pub enum ClassParameterType<'tree> {
    AnnotatedType(::std::boxed::Box<AnnotatedType<'tree>>),
    AppliedConstructorType(::std::boxed::Box<AppliedConstructorType<'tree>>),
    CompoundType(::std::boxed::Box<CompoundType<'tree>>),
    FunctionType(::std::boxed::Box<FunctionType<'tree>>),
    GenericType(::std::boxed::Box<GenericType<'tree>>),
    InfixType(::std::boxed::Box<InfixType<'tree>>),
    LazyParameterType(::std::boxed::Box<LazyParameterType<'tree>>),
    LiteralType(::std::boxed::Box<LiteralType<'tree>>),
    MatchType(::std::boxed::Box<MatchType<'tree>>),
    NamedTupleType(::std::boxed::Box<NamedTupleType<'tree>>),
    ProjectedType(::std::boxed::Box<ProjectedType<'tree>>),
    RepeatedParameterType(::std::boxed::Box<RepeatedParameterType<'tree>>),
    SingletonType(::std::boxed::Box<SingletonType<'tree>>),
    StableTypeIdentifier(::std::boxed::Box<StableTypeIdentifier<'tree>>),
    StructuralType(::std::boxed::Box<StructuralType<'tree>>),
    TupleType(::std::boxed::Box<TupleType<'tree>>),
    TypeIdentifier(::std::boxed::Box<TypeIdentifier<'tree>>),
    TypeLambda(::std::boxed::Box<TypeLambda<'tree>>),
    Wildcard(::std::boxed::Box<Wildcard<'tree>>),
}
impl<'tree> ::treesitter_types::FromNode<'tree> for ClassParameterType<'tree> {
    #[allow(clippy::collapsible_else_if)]
    fn from_node(
        node: ::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        match node.kind() {
            "annotated_type" => Ok(Self::AnnotatedType(::std::boxed::Box::new(
                <AnnotatedType as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "applied_constructor_type" => Ok(Self::AppliedConstructorType(::std::boxed::Box::new(
                <AppliedConstructorType as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "compound_type" => Ok(Self::CompoundType(::std::boxed::Box::new(
                <CompoundType as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "function_type" => Ok(Self::FunctionType(::std::boxed::Box::new(
                <FunctionType as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "generic_type" => Ok(Self::GenericType(::std::boxed::Box::new(
                <GenericType as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "infix_type" => Ok(Self::InfixType(::std::boxed::Box::new(
                <InfixType as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "lazy_parameter_type" => Ok(Self::LazyParameterType(::std::boxed::Box::new(
                <LazyParameterType as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "literal_type" => Ok(Self::LiteralType(::std::boxed::Box::new(
                <LiteralType as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "match_type" => Ok(Self::MatchType(::std::boxed::Box::new(
                <MatchType as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "named_tuple_type" => Ok(Self::NamedTupleType(::std::boxed::Box::new(
                <NamedTupleType as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "projected_type" => Ok(Self::ProjectedType(::std::boxed::Box::new(
                <ProjectedType as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "repeated_parameter_type" => Ok(Self::RepeatedParameterType(::std::boxed::Box::new(
                <RepeatedParameterType as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "singleton_type" => Ok(Self::SingletonType(::std::boxed::Box::new(
                <SingletonType as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "stable_type_identifier" => Ok(Self::StableTypeIdentifier(::std::boxed::Box::new(
                <StableTypeIdentifier as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "structural_type" => Ok(Self::StructuralType(::std::boxed::Box::new(
                <StructuralType as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "tuple_type" => Ok(Self::TupleType(::std::boxed::Box::new(
                <TupleType as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "type_identifier" => Ok(Self::TypeIdentifier(::std::boxed::Box::new(
                <TypeIdentifier as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "type_lambda" => Ok(Self::TypeLambda(::std::boxed::Box::new(
                <TypeLambda as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "wildcard" => Ok(Self::Wildcard(::std::boxed::Box::new(
                <Wildcard as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            other => Err(::treesitter_types::ParseError::unexpected_kind(other, node)),
        }
    }
}
impl ::treesitter_types::Spanned for ClassParameterType<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        match self {
            Self::AnnotatedType(inner) => inner.span(),
            Self::AppliedConstructorType(inner) => inner.span(),
            Self::CompoundType(inner) => inner.span(),
            Self::FunctionType(inner) => inner.span(),
            Self::GenericType(inner) => inner.span(),
            Self::InfixType(inner) => inner.span(),
            Self::LazyParameterType(inner) => inner.span(),
            Self::LiteralType(inner) => inner.span(),
            Self::MatchType(inner) => inner.span(),
            Self::NamedTupleType(inner) => inner.span(),
            Self::ProjectedType(inner) => inner.span(),
            Self::RepeatedParameterType(inner) => inner.span(),
            Self::SingletonType(inner) => inner.span(),
            Self::StableTypeIdentifier(inner) => inner.span(),
            Self::StructuralType(inner) => inner.span(),
            Self::TupleType(inner) => inner.span(),
            Self::TypeIdentifier(inner) => inner.span(),
            Self::TypeLambda(inner) => inner.span(),
            Self::Wildcard(inner) => inner.span(),
        }
    }
}
#[derive(Debug, Clone)]
pub enum ClassParameterChildren<'tree> {
    Annotation(::std::boxed::Box<Annotation<'tree>>),
    Modifiers(::std::boxed::Box<Modifiers<'tree>>),
}
impl<'tree> ::treesitter_types::FromNode<'tree> for ClassParameterChildren<'tree> {
    #[allow(clippy::collapsible_else_if)]
    fn from_node(
        node: ::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        match node.kind() {
            "annotation" => Ok(Self::Annotation(::std::boxed::Box::new(
                <Annotation as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "modifiers" => Ok(Self::Modifiers(::std::boxed::Box::new(
                <Modifiers as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            other => Err(::treesitter_types::ParseError::unexpected_kind(other, node)),
        }
    }
}
impl ::treesitter_types::Spanned for ClassParameterChildren<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        match self {
            Self::Annotation(inner) => inner.span(),
            Self::Modifiers(inner) => inner.span(),
        }
    }
}
#[derive(Debug, Clone)]
pub enum ClassParametersChildren<'tree> {
    AnnotatedType(::std::boxed::Box<AnnotatedType<'tree>>),
    AppliedConstructorType(::std::boxed::Box<AppliedConstructorType<'tree>>),
    ClassParameter(::std::boxed::Box<ClassParameter<'tree>>),
    CompoundType(::std::boxed::Box<CompoundType<'tree>>),
    FunctionType(::std::boxed::Box<FunctionType<'tree>>),
    GenericType(::std::boxed::Box<GenericType<'tree>>),
    InfixType(::std::boxed::Box<InfixType<'tree>>),
    LazyParameterType(::std::boxed::Box<LazyParameterType<'tree>>),
    LiteralType(::std::boxed::Box<LiteralType<'tree>>),
    MatchType(::std::boxed::Box<MatchType<'tree>>),
    NamedTupleType(::std::boxed::Box<NamedTupleType<'tree>>),
    ProjectedType(::std::boxed::Box<ProjectedType<'tree>>),
    RepeatedParameterType(::std::boxed::Box<RepeatedParameterType<'tree>>),
    SingletonType(::std::boxed::Box<SingletonType<'tree>>),
    StableTypeIdentifier(::std::boxed::Box<StableTypeIdentifier<'tree>>),
    StructuralType(::std::boxed::Box<StructuralType<'tree>>),
    TupleType(::std::boxed::Box<TupleType<'tree>>),
    TypeIdentifier(::std::boxed::Box<TypeIdentifier<'tree>>),
    TypeLambda(::std::boxed::Box<TypeLambda<'tree>>),
    Wildcard(::std::boxed::Box<Wildcard<'tree>>),
}
impl<'tree> ::treesitter_types::FromNode<'tree> for ClassParametersChildren<'tree> {
    #[allow(clippy::collapsible_else_if)]
    fn from_node(
        node: ::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        match node.kind() {
            "annotated_type" => Ok(Self::AnnotatedType(::std::boxed::Box::new(
                <AnnotatedType as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "applied_constructor_type" => Ok(Self::AppliedConstructorType(::std::boxed::Box::new(
                <AppliedConstructorType as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "class_parameter" => Ok(Self::ClassParameter(::std::boxed::Box::new(
                <ClassParameter as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "compound_type" => Ok(Self::CompoundType(::std::boxed::Box::new(
                <CompoundType as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "function_type" => Ok(Self::FunctionType(::std::boxed::Box::new(
                <FunctionType as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "generic_type" => Ok(Self::GenericType(::std::boxed::Box::new(
                <GenericType as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "infix_type" => Ok(Self::InfixType(::std::boxed::Box::new(
                <InfixType as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "lazy_parameter_type" => Ok(Self::LazyParameterType(::std::boxed::Box::new(
                <LazyParameterType as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "literal_type" => Ok(Self::LiteralType(::std::boxed::Box::new(
                <LiteralType as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "match_type" => Ok(Self::MatchType(::std::boxed::Box::new(
                <MatchType as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "named_tuple_type" => Ok(Self::NamedTupleType(::std::boxed::Box::new(
                <NamedTupleType as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "projected_type" => Ok(Self::ProjectedType(::std::boxed::Box::new(
                <ProjectedType as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "repeated_parameter_type" => Ok(Self::RepeatedParameterType(::std::boxed::Box::new(
                <RepeatedParameterType as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "singleton_type" => Ok(Self::SingletonType(::std::boxed::Box::new(
                <SingletonType as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "stable_type_identifier" => Ok(Self::StableTypeIdentifier(::std::boxed::Box::new(
                <StableTypeIdentifier as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "structural_type" => Ok(Self::StructuralType(::std::boxed::Box::new(
                <StructuralType as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "tuple_type" => Ok(Self::TupleType(::std::boxed::Box::new(
                <TupleType as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "type_identifier" => Ok(Self::TypeIdentifier(::std::boxed::Box::new(
                <TypeIdentifier as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "type_lambda" => Ok(Self::TypeLambda(::std::boxed::Box::new(
                <TypeLambda as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "wildcard" => Ok(Self::Wildcard(::std::boxed::Box::new(
                <Wildcard as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            other => Err(::treesitter_types::ParseError::unexpected_kind(other, node)),
        }
    }
}
impl ::treesitter_types::Spanned for ClassParametersChildren<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        match self {
            Self::AnnotatedType(inner) => inner.span(),
            Self::AppliedConstructorType(inner) => inner.span(),
            Self::ClassParameter(inner) => inner.span(),
            Self::CompoundType(inner) => inner.span(),
            Self::FunctionType(inner) => inner.span(),
            Self::GenericType(inner) => inner.span(),
            Self::InfixType(inner) => inner.span(),
            Self::LazyParameterType(inner) => inner.span(),
            Self::LiteralType(inner) => inner.span(),
            Self::MatchType(inner) => inner.span(),
            Self::NamedTupleType(inner) => inner.span(),
            Self::ProjectedType(inner) => inner.span(),
            Self::RepeatedParameterType(inner) => inner.span(),
            Self::SingletonType(inner) => inner.span(),
            Self::StableTypeIdentifier(inner) => inner.span(),
            Self::StructuralType(inner) => inner.span(),
            Self::TupleType(inner) => inner.span(),
            Self::TypeIdentifier(inner) => inner.span(),
            Self::TypeLambda(inner) => inner.span(),
            Self::Wildcard(inner) => inner.span(),
        }
    }
}
#[derive(Debug, Clone)]
pub enum ColonArgumentLambdaStart<'tree> {
    FatArrow(::treesitter_types::Span),
    Bindings(::std::boxed::Box<Bindings<'tree>>),
    Identifier(::std::boxed::Box<Identifier<'tree>>),
    OperatorIdentifier(::std::boxed::Box<OperatorIdentifier<'tree>>),
    Wildcard(::std::boxed::Box<Wildcard<'tree>>),
}
impl<'tree> ::treesitter_types::FromNode<'tree> for ColonArgumentLambdaStart<'tree> {
    #[allow(clippy::collapsible_else_if)]
    fn from_node(
        node: ::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        match node.kind() {
            "=>" => Ok(Self::FatArrow(::treesitter_types::Span::from(node))),
            "bindings" => Ok(Self::Bindings(::std::boxed::Box::new(
                <Bindings as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "identifier" => Ok(Self::Identifier(::std::boxed::Box::new(
                <Identifier as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "operator_identifier" => Ok(Self::OperatorIdentifier(::std::boxed::Box::new(
                <OperatorIdentifier as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "wildcard" => Ok(Self::Wildcard(::std::boxed::Box::new(
                <Wildcard as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            other => Err(::treesitter_types::ParseError::unexpected_kind(other, node)),
        }
    }
}
impl ::treesitter_types::Spanned for ColonArgumentLambdaStart<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        match self {
            Self::FatArrow(span) => *span,
            Self::Bindings(inner) => inner.span(),
            Self::Identifier(inner) => inner.span(),
            Self::OperatorIdentifier(inner) => inner.span(),
            Self::Wildcard(inner) => inner.span(),
        }
    }
}
#[derive(Debug, Clone)]
pub enum ColonArgumentChildren<'tree> {
    IndentedBlock(::std::boxed::Box<IndentedBlock<'tree>>),
    IndentedCases(::std::boxed::Box<IndentedCases<'tree>>),
}
impl<'tree> ::treesitter_types::FromNode<'tree> for ColonArgumentChildren<'tree> {
    #[allow(clippy::collapsible_else_if)]
    fn from_node(
        node: ::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        match node.kind() {
            "indented_block" => Ok(Self::IndentedBlock(::std::boxed::Box::new(
                <IndentedBlock as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "indented_cases" => Ok(Self::IndentedCases(::std::boxed::Box::new(
                <IndentedCases as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            other => Err(::treesitter_types::ParseError::unexpected_kind(other, node)),
        }
    }
}
impl ::treesitter_types::Spanned for ColonArgumentChildren<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        match self {
            Self::IndentedBlock(inner) => inner.span(),
            Self::IndentedCases(inner) => inner.span(),
        }
    }
}
#[derive(Debug, Clone)]
pub enum CompilationUnitChildren<'tree> {
    Definition(::std::boxed::Box<Definition<'tree>>),
    Comment(::std::boxed::Box<Comment<'tree>>),
    Expression(::std::boxed::Box<Expression<'tree>>),
}
impl<'tree> ::treesitter_types::FromNode<'tree> for CompilationUnitChildren<'tree> {
    #[allow(clippy::collapsible_else_if)]
    fn from_node(
        node: ::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        match node.kind() {
            "comment" => Ok(Self::Comment(::std::boxed::Box::new(
                <Comment as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            _other => {
                if let Ok(v) = <Definition as ::treesitter_types::FromNode>::from_node(node, src) {
                    Ok(Self::Definition(::std::boxed::Box::new(v)))
                } else {
                    if let Ok(v) =
                        <Expression as ::treesitter_types::FromNode>::from_node(node, src)
                    {
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
}
impl ::treesitter_types::Spanned for CompilationUnitChildren<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        match self {
            Self::Definition(inner) => inner.span(),
            Self::Comment(inner) => inner.span(),
            Self::Expression(inner) => inner.span(),
        }
    }
}
#[derive(Debug, Clone)]
pub enum CompoundTypeBase<'tree> {
    AnnotatedType(::std::boxed::Box<AnnotatedType<'tree>>),
    AppliedConstructorType(::std::boxed::Box<AppliedConstructorType<'tree>>),
    GenericType(::std::boxed::Box<GenericType<'tree>>),
    NamedTupleType(::std::boxed::Box<NamedTupleType<'tree>>),
    ProjectedType(::std::boxed::Box<ProjectedType<'tree>>),
    SingletonType(::std::boxed::Box<SingletonType<'tree>>),
    StableTypeIdentifier(::std::boxed::Box<StableTypeIdentifier<'tree>>),
    TupleType(::std::boxed::Box<TupleType<'tree>>),
    TypeIdentifier(::std::boxed::Box<TypeIdentifier<'tree>>),
    Wildcard(::std::boxed::Box<Wildcard<'tree>>),
}
impl<'tree> ::treesitter_types::FromNode<'tree> for CompoundTypeBase<'tree> {
    #[allow(clippy::collapsible_else_if)]
    fn from_node(
        node: ::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        match node.kind() {
            "annotated_type" => Ok(Self::AnnotatedType(::std::boxed::Box::new(
                <AnnotatedType as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "applied_constructor_type" => Ok(Self::AppliedConstructorType(::std::boxed::Box::new(
                <AppliedConstructorType as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "generic_type" => Ok(Self::GenericType(::std::boxed::Box::new(
                <GenericType as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "named_tuple_type" => Ok(Self::NamedTupleType(::std::boxed::Box::new(
                <NamedTupleType as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "projected_type" => Ok(Self::ProjectedType(::std::boxed::Box::new(
                <ProjectedType as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "singleton_type" => Ok(Self::SingletonType(::std::boxed::Box::new(
                <SingletonType as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "stable_type_identifier" => Ok(Self::StableTypeIdentifier(::std::boxed::Box::new(
                <StableTypeIdentifier as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "tuple_type" => Ok(Self::TupleType(::std::boxed::Box::new(
                <TupleType as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "type_identifier" => Ok(Self::TypeIdentifier(::std::boxed::Box::new(
                <TypeIdentifier as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "wildcard" => Ok(Self::Wildcard(::std::boxed::Box::new(
                <Wildcard as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            other => Err(::treesitter_types::ParseError::unexpected_kind(other, node)),
        }
    }
}
impl ::treesitter_types::Spanned for CompoundTypeBase<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        match self {
            Self::AnnotatedType(inner) => inner.span(),
            Self::AppliedConstructorType(inner) => inner.span(),
            Self::GenericType(inner) => inner.span(),
            Self::NamedTupleType(inner) => inner.span(),
            Self::ProjectedType(inner) => inner.span(),
            Self::SingletonType(inner) => inner.span(),
            Self::StableTypeIdentifier(inner) => inner.span(),
            Self::TupleType(inner) => inner.span(),
            Self::TypeIdentifier(inner) => inner.span(),
            Self::Wildcard(inner) => inner.span(),
        }
    }
}
#[derive(Debug, Clone)]
pub enum CompoundTypeExtra<'tree> {
    AnnotatedType(::std::boxed::Box<AnnotatedType<'tree>>),
    AppliedConstructorType(::std::boxed::Box<AppliedConstructorType<'tree>>),
    GenericType(::std::boxed::Box<GenericType<'tree>>),
    NamedTupleType(::std::boxed::Box<NamedTupleType<'tree>>),
    ProjectedType(::std::boxed::Box<ProjectedType<'tree>>),
    SingletonType(::std::boxed::Box<SingletonType<'tree>>),
    StableTypeIdentifier(::std::boxed::Box<StableTypeIdentifier<'tree>>),
    TupleType(::std::boxed::Box<TupleType<'tree>>),
    TypeIdentifier(::std::boxed::Box<TypeIdentifier<'tree>>),
    Wildcard(::std::boxed::Box<Wildcard<'tree>>),
}
impl<'tree> ::treesitter_types::FromNode<'tree> for CompoundTypeExtra<'tree> {
    #[allow(clippy::collapsible_else_if)]
    fn from_node(
        node: ::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        match node.kind() {
            "annotated_type" => Ok(Self::AnnotatedType(::std::boxed::Box::new(
                <AnnotatedType as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "applied_constructor_type" => Ok(Self::AppliedConstructorType(::std::boxed::Box::new(
                <AppliedConstructorType as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "generic_type" => Ok(Self::GenericType(::std::boxed::Box::new(
                <GenericType as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "named_tuple_type" => Ok(Self::NamedTupleType(::std::boxed::Box::new(
                <NamedTupleType as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "projected_type" => Ok(Self::ProjectedType(::std::boxed::Box::new(
                <ProjectedType as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "singleton_type" => Ok(Self::SingletonType(::std::boxed::Box::new(
                <SingletonType as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "stable_type_identifier" => Ok(Self::StableTypeIdentifier(::std::boxed::Box::new(
                <StableTypeIdentifier as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "tuple_type" => Ok(Self::TupleType(::std::boxed::Box::new(
                <TupleType as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "type_identifier" => Ok(Self::TypeIdentifier(::std::boxed::Box::new(
                <TypeIdentifier as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "wildcard" => Ok(Self::Wildcard(::std::boxed::Box::new(
                <Wildcard as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            other => Err(::treesitter_types::ParseError::unexpected_kind(other, node)),
        }
    }
}
impl ::treesitter_types::Spanned for CompoundTypeExtra<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        match self {
            Self::AnnotatedType(inner) => inner.span(),
            Self::AppliedConstructorType(inner) => inner.span(),
            Self::GenericType(inner) => inner.span(),
            Self::NamedTupleType(inner) => inner.span(),
            Self::ProjectedType(inner) => inner.span(),
            Self::SingletonType(inner) => inner.span(),
            Self::StableTypeIdentifier(inner) => inner.span(),
            Self::TupleType(inner) => inner.span(),
            Self::TypeIdentifier(inner) => inner.span(),
            Self::Wildcard(inner) => inner.span(),
        }
    }
}
#[derive(Debug, Clone)]
pub enum ContextBoundName<'tree> {
    Identifier(::std::boxed::Box<Identifier<'tree>>),
    OperatorIdentifier(::std::boxed::Box<OperatorIdentifier<'tree>>),
}
impl<'tree> ::treesitter_types::FromNode<'tree> for ContextBoundName<'tree> {
    #[allow(clippy::collapsible_else_if)]
    fn from_node(
        node: ::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        match node.kind() {
            "identifier" => Ok(Self::Identifier(::std::boxed::Box::new(
                <Identifier as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "operator_identifier" => Ok(Self::OperatorIdentifier(::std::boxed::Box::new(
                <OperatorIdentifier as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            other => Err(::treesitter_types::ParseError::unexpected_kind(other, node)),
        }
    }
}
impl ::treesitter_types::Spanned for ContextBoundName<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        match self {
            Self::Identifier(inner) => inner.span(),
            Self::OperatorIdentifier(inner) => inner.span(),
        }
    }
}
#[derive(Debug, Clone)]
pub enum ContextBoundType<'tree> {
    AnnotatedType(::std::boxed::Box<AnnotatedType<'tree>>),
    AppliedConstructorType(::std::boxed::Box<AppliedConstructorType<'tree>>),
    CompoundType(::std::boxed::Box<CompoundType<'tree>>),
    FunctionType(::std::boxed::Box<FunctionType<'tree>>),
    GenericType(::std::boxed::Box<GenericType<'tree>>),
    InfixType(::std::boxed::Box<InfixType<'tree>>),
    LiteralType(::std::boxed::Box<LiteralType<'tree>>),
    MatchType(::std::boxed::Box<MatchType<'tree>>),
    NamedTupleType(::std::boxed::Box<NamedTupleType<'tree>>),
    ProjectedType(::std::boxed::Box<ProjectedType<'tree>>),
    SingletonType(::std::boxed::Box<SingletonType<'tree>>),
    StableTypeIdentifier(::std::boxed::Box<StableTypeIdentifier<'tree>>),
    StructuralType(::std::boxed::Box<StructuralType<'tree>>),
    TupleType(::std::boxed::Box<TupleType<'tree>>),
    TypeIdentifier(::std::boxed::Box<TypeIdentifier<'tree>>),
    TypeLambda(::std::boxed::Box<TypeLambda<'tree>>),
    Wildcard(::std::boxed::Box<Wildcard<'tree>>),
}
impl<'tree> ::treesitter_types::FromNode<'tree> for ContextBoundType<'tree> {
    #[allow(clippy::collapsible_else_if)]
    fn from_node(
        node: ::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        match node.kind() {
            "annotated_type" => Ok(Self::AnnotatedType(::std::boxed::Box::new(
                <AnnotatedType as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "applied_constructor_type" => Ok(Self::AppliedConstructorType(::std::boxed::Box::new(
                <AppliedConstructorType as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "compound_type" => Ok(Self::CompoundType(::std::boxed::Box::new(
                <CompoundType as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "function_type" => Ok(Self::FunctionType(::std::boxed::Box::new(
                <FunctionType as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "generic_type" => Ok(Self::GenericType(::std::boxed::Box::new(
                <GenericType as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "infix_type" => Ok(Self::InfixType(::std::boxed::Box::new(
                <InfixType as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "literal_type" => Ok(Self::LiteralType(::std::boxed::Box::new(
                <LiteralType as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "match_type" => Ok(Self::MatchType(::std::boxed::Box::new(
                <MatchType as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "named_tuple_type" => Ok(Self::NamedTupleType(::std::boxed::Box::new(
                <NamedTupleType as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "projected_type" => Ok(Self::ProjectedType(::std::boxed::Box::new(
                <ProjectedType as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "singleton_type" => Ok(Self::SingletonType(::std::boxed::Box::new(
                <SingletonType as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "stable_type_identifier" => Ok(Self::StableTypeIdentifier(::std::boxed::Box::new(
                <StableTypeIdentifier as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "structural_type" => Ok(Self::StructuralType(::std::boxed::Box::new(
                <StructuralType as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "tuple_type" => Ok(Self::TupleType(::std::boxed::Box::new(
                <TupleType as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "type_identifier" => Ok(Self::TypeIdentifier(::std::boxed::Box::new(
                <TypeIdentifier as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "type_lambda" => Ok(Self::TypeLambda(::std::boxed::Box::new(
                <TypeLambda as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "wildcard" => Ok(Self::Wildcard(::std::boxed::Box::new(
                <Wildcard as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            other => Err(::treesitter_types::ParseError::unexpected_kind(other, node)),
        }
    }
}
impl ::treesitter_types::Spanned for ContextBoundType<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        match self {
            Self::AnnotatedType(inner) => inner.span(),
            Self::AppliedConstructorType(inner) => inner.span(),
            Self::CompoundType(inner) => inner.span(),
            Self::FunctionType(inner) => inner.span(),
            Self::GenericType(inner) => inner.span(),
            Self::InfixType(inner) => inner.span(),
            Self::LiteralType(inner) => inner.span(),
            Self::MatchType(inner) => inner.span(),
            Self::NamedTupleType(inner) => inner.span(),
            Self::ProjectedType(inner) => inner.span(),
            Self::SingletonType(inner) => inner.span(),
            Self::StableTypeIdentifier(inner) => inner.span(),
            Self::StructuralType(inner) => inner.span(),
            Self::TupleType(inner) => inner.span(),
            Self::TypeIdentifier(inner) => inner.span(),
            Self::TypeLambda(inner) => inner.span(),
            Self::Wildcard(inner) => inner.span(),
        }
    }
}
#[derive(Debug, Clone)]
pub enum ContravariantTypeParameterBound<'tree> {
    Comma(::treesitter_types::Span),
    Colon(::treesitter_types::Span),
    ContextBound(::std::boxed::Box<ContextBound<'tree>>),
    LowerBound(::std::boxed::Box<LowerBound<'tree>>),
    UpperBound(::std::boxed::Box<UpperBound<'tree>>),
    ViewBound(::std::boxed::Box<ViewBound<'tree>>),
    LBrace(::treesitter_types::Span),
    RBrace(::treesitter_types::Span),
}
impl<'tree> ::treesitter_types::FromNode<'tree> for ContravariantTypeParameterBound<'tree> {
    #[allow(clippy::collapsible_else_if)]
    fn from_node(
        node: ::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        match node.kind() {
            "," => Ok(Self::Comma(::treesitter_types::Span::from(node))),
            ":" => Ok(Self::Colon(::treesitter_types::Span::from(node))),
            "context_bound" => Ok(Self::ContextBound(::std::boxed::Box::new(
                <ContextBound as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "lower_bound" => Ok(Self::LowerBound(::std::boxed::Box::new(
                <LowerBound as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "upper_bound" => Ok(Self::UpperBound(::std::boxed::Box::new(
                <UpperBound as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "view_bound" => Ok(Self::ViewBound(::std::boxed::Box::new(
                <ViewBound as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "{" => Ok(Self::LBrace(::treesitter_types::Span::from(node))),
            "}" => Ok(Self::RBrace(::treesitter_types::Span::from(node))),
            other => Err(::treesitter_types::ParseError::unexpected_kind(other, node)),
        }
    }
}
impl ::treesitter_types::Spanned for ContravariantTypeParameterBound<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        match self {
            Self::Comma(span) => *span,
            Self::Colon(span) => *span,
            Self::ContextBound(inner) => inner.span(),
            Self::LowerBound(inner) => inner.span(),
            Self::UpperBound(inner) => inner.span(),
            Self::ViewBound(inner) => inner.span(),
            Self::LBrace(span) => *span,
            Self::RBrace(span) => *span,
        }
    }
}
#[derive(Debug, Clone)]
pub enum ContravariantTypeParameterName<'tree> {
    Identifier(::std::boxed::Box<Identifier<'tree>>),
    OperatorIdentifier(::std::boxed::Box<OperatorIdentifier<'tree>>),
    Wildcard(::std::boxed::Box<Wildcard<'tree>>),
}
impl<'tree> ::treesitter_types::FromNode<'tree> for ContravariantTypeParameterName<'tree> {
    #[allow(clippy::collapsible_else_if)]
    fn from_node(
        node: ::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        match node.kind() {
            "identifier" => Ok(Self::Identifier(::std::boxed::Box::new(
                <Identifier as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "operator_identifier" => Ok(Self::OperatorIdentifier(::std::boxed::Box::new(
                <OperatorIdentifier as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "wildcard" => Ok(Self::Wildcard(::std::boxed::Box::new(
                <Wildcard as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            other => Err(::treesitter_types::ParseError::unexpected_kind(other, node)),
        }
    }
}
impl ::treesitter_types::Spanned for ContravariantTypeParameterName<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        match self {
            Self::Identifier(inner) => inner.span(),
            Self::OperatorIdentifier(inner) => inner.span(),
            Self::Wildcard(inner) => inner.span(),
        }
    }
}
#[derive(Debug, Clone)]
pub enum CovariantTypeParameterBound<'tree> {
    Comma(::treesitter_types::Span),
    Colon(::treesitter_types::Span),
    ContextBound(::std::boxed::Box<ContextBound<'tree>>),
    LowerBound(::std::boxed::Box<LowerBound<'tree>>),
    UpperBound(::std::boxed::Box<UpperBound<'tree>>),
    ViewBound(::std::boxed::Box<ViewBound<'tree>>),
    LBrace(::treesitter_types::Span),
    RBrace(::treesitter_types::Span),
}
impl<'tree> ::treesitter_types::FromNode<'tree> for CovariantTypeParameterBound<'tree> {
    #[allow(clippy::collapsible_else_if)]
    fn from_node(
        node: ::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        match node.kind() {
            "," => Ok(Self::Comma(::treesitter_types::Span::from(node))),
            ":" => Ok(Self::Colon(::treesitter_types::Span::from(node))),
            "context_bound" => Ok(Self::ContextBound(::std::boxed::Box::new(
                <ContextBound as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "lower_bound" => Ok(Self::LowerBound(::std::boxed::Box::new(
                <LowerBound as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "upper_bound" => Ok(Self::UpperBound(::std::boxed::Box::new(
                <UpperBound as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "view_bound" => Ok(Self::ViewBound(::std::boxed::Box::new(
                <ViewBound as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "{" => Ok(Self::LBrace(::treesitter_types::Span::from(node))),
            "}" => Ok(Self::RBrace(::treesitter_types::Span::from(node))),
            other => Err(::treesitter_types::ParseError::unexpected_kind(other, node)),
        }
    }
}
impl ::treesitter_types::Spanned for CovariantTypeParameterBound<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        match self {
            Self::Comma(span) => *span,
            Self::Colon(span) => *span,
            Self::ContextBound(inner) => inner.span(),
            Self::LowerBound(inner) => inner.span(),
            Self::UpperBound(inner) => inner.span(),
            Self::ViewBound(inner) => inner.span(),
            Self::LBrace(span) => *span,
            Self::RBrace(span) => *span,
        }
    }
}
#[derive(Debug, Clone)]
pub enum CovariantTypeParameterName<'tree> {
    Identifier(::std::boxed::Box<Identifier<'tree>>),
    OperatorIdentifier(::std::boxed::Box<OperatorIdentifier<'tree>>),
    Wildcard(::std::boxed::Box<Wildcard<'tree>>),
}
impl<'tree> ::treesitter_types::FromNode<'tree> for CovariantTypeParameterName<'tree> {
    #[allow(clippy::collapsible_else_if)]
    fn from_node(
        node: ::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        match node.kind() {
            "identifier" => Ok(Self::Identifier(::std::boxed::Box::new(
                <Identifier as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "operator_identifier" => Ok(Self::OperatorIdentifier(::std::boxed::Box::new(
                <OperatorIdentifier as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "wildcard" => Ok(Self::Wildcard(::std::boxed::Box::new(
                <Wildcard as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            other => Err(::treesitter_types::ParseError::unexpected_kind(other, node)),
        }
    }
}
impl ::treesitter_types::Spanned for CovariantTypeParameterName<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        match self {
            Self::Identifier(inner) => inner.span(),
            Self::OperatorIdentifier(inner) => inner.span(),
            Self::Wildcard(inner) => inner.span(),
        }
    }
}
#[derive(Debug, Clone)]
pub enum DerivesClauseType<'tree> {
    StableTypeIdentifier(::std::boxed::Box<StableTypeIdentifier<'tree>>),
    TypeIdentifier(::std::boxed::Box<TypeIdentifier<'tree>>),
}
impl<'tree> ::treesitter_types::FromNode<'tree> for DerivesClauseType<'tree> {
    #[allow(clippy::collapsible_else_if)]
    fn from_node(
        node: ::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        match node.kind() {
            "stable_type_identifier" => Ok(Self::StableTypeIdentifier(::std::boxed::Box::new(
                <StableTypeIdentifier as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "type_identifier" => Ok(Self::TypeIdentifier(::std::boxed::Box::new(
                <TypeIdentifier as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            other => Err(::treesitter_types::ParseError::unexpected_kind(other, node)),
        }
    }
}
impl ::treesitter_types::Spanned for DerivesClauseType<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        match self {
            Self::StableTypeIdentifier(inner) => inner.span(),
            Self::TypeIdentifier(inner) => inner.span(),
        }
    }
}
#[derive(Debug, Clone)]
pub enum EnumBodyChildren<'tree> {
    Definition(::std::boxed::Box<Definition<'tree>>),
    EnumCaseDefinitions(::std::boxed::Box<EnumCaseDefinitions<'tree>>),
    Expression(::std::boxed::Box<Expression<'tree>>),
}
impl<'tree> ::treesitter_types::FromNode<'tree> for EnumBodyChildren<'tree> {
    #[allow(clippy::collapsible_else_if)]
    fn from_node(
        node: ::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        match node.kind() {
            "enum_case_definitions" => Ok(Self::EnumCaseDefinitions(::std::boxed::Box::new(
                <EnumCaseDefinitions as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            _other => {
                if let Ok(v) = <Definition as ::treesitter_types::FromNode>::from_node(node, src) {
                    Ok(Self::Definition(::std::boxed::Box::new(v)))
                } else {
                    if let Ok(v) =
                        <Expression as ::treesitter_types::FromNode>::from_node(node, src)
                    {
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
}
impl ::treesitter_types::Spanned for EnumBodyChildren<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        match self {
            Self::Definition(inner) => inner.span(),
            Self::EnumCaseDefinitions(inner) => inner.span(),
            Self::Expression(inner) => inner.span(),
        }
    }
}
#[derive(Debug, Clone)]
pub enum EnumCaseDefinitionsChildren<'tree> {
    Annotation(::std::boxed::Box<Annotation<'tree>>),
    FullEnumCase(::std::boxed::Box<FullEnumCase<'tree>>),
    SimpleEnumCase(::std::boxed::Box<SimpleEnumCase<'tree>>),
}
impl<'tree> ::treesitter_types::FromNode<'tree> for EnumCaseDefinitionsChildren<'tree> {
    #[allow(clippy::collapsible_else_if)]
    fn from_node(
        node: ::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        match node.kind() {
            "annotation" => Ok(Self::Annotation(::std::boxed::Box::new(
                <Annotation as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "full_enum_case" => Ok(Self::FullEnumCase(::std::boxed::Box::new(
                <FullEnumCase as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "simple_enum_case" => Ok(Self::SimpleEnumCase(::std::boxed::Box::new(
                <SimpleEnumCase as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            other => Err(::treesitter_types::ParseError::unexpected_kind(other, node)),
        }
    }
}
impl ::treesitter_types::Spanned for EnumCaseDefinitionsChildren<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        match self {
            Self::Annotation(inner) => inner.span(),
            Self::FullEnumCase(inner) => inner.span(),
            Self::SimpleEnumCase(inner) => inner.span(),
        }
    }
}
#[derive(Debug, Clone)]
pub enum EnumDefinitionName<'tree> {
    Identifier(::std::boxed::Box<Identifier<'tree>>),
    OperatorIdentifier(::std::boxed::Box<OperatorIdentifier<'tree>>),
}
impl<'tree> ::treesitter_types::FromNode<'tree> for EnumDefinitionName<'tree> {
    #[allow(clippy::collapsible_else_if)]
    fn from_node(
        node: ::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        match node.kind() {
            "identifier" => Ok(Self::Identifier(::std::boxed::Box::new(
                <Identifier as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "operator_identifier" => Ok(Self::OperatorIdentifier(::std::boxed::Box::new(
                <OperatorIdentifier as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            other => Err(::treesitter_types::ParseError::unexpected_kind(other, node)),
        }
    }
}
impl ::treesitter_types::Spanned for EnumDefinitionName<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        match self {
            Self::Identifier(inner) => inner.span(),
            Self::OperatorIdentifier(inner) => inner.span(),
        }
    }
}
#[derive(Debug, Clone)]
pub enum EnumDefinitionChildren<'tree> {
    AccessModifier(::std::boxed::Box<AccessModifier<'tree>>),
    Annotation(::std::boxed::Box<Annotation<'tree>>),
}
impl<'tree> ::treesitter_types::FromNode<'tree> for EnumDefinitionChildren<'tree> {
    #[allow(clippy::collapsible_else_if)]
    fn from_node(
        node: ::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        match node.kind() {
            "access_modifier" => Ok(Self::AccessModifier(::std::boxed::Box::new(
                <AccessModifier as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "annotation" => Ok(Self::Annotation(::std::boxed::Box::new(
                <Annotation as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            other => Err(::treesitter_types::ParseError::unexpected_kind(other, node)),
        }
    }
}
impl ::treesitter_types::Spanned for EnumDefinitionChildren<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        match self {
            Self::AccessModifier(inner) => inner.span(),
            Self::Annotation(inner) => inner.span(),
        }
    }
}
#[derive(Debug, Clone)]
pub enum EnumeratorChildren<'tree> {
    Pattern(::std::boxed::Box<Pattern<'tree>>),
    Expression(::std::boxed::Box<Expression<'tree>>),
    Guard(::std::boxed::Box<Guard<'tree>>),
}
impl<'tree> ::treesitter_types::FromNode<'tree> for EnumeratorChildren<'tree> {
    #[allow(clippy::collapsible_else_if)]
    fn from_node(
        node: ::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        match node.kind() {
            "guard" => Ok(Self::Guard(::std::boxed::Box::new(
                <Guard as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            _other => {
                if let Ok(v) = <Pattern as ::treesitter_types::FromNode>::from_node(node, src) {
                    Ok(Self::Pattern(::std::boxed::Box::new(v)))
                } else {
                    if let Ok(v) =
                        <Expression as ::treesitter_types::FromNode>::from_node(node, src)
                    {
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
}
impl ::treesitter_types::Spanned for EnumeratorChildren<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        match self {
            Self::Pattern(inner) => inner.span(),
            Self::Expression(inner) => inner.span(),
            Self::Guard(inner) => inner.span(),
        }
    }
}
#[derive(Debug, Clone)]
pub enum ExportDeclarationPath<'tree> {
    Dot(::treesitter_types::Span),
    Identifier(::std::boxed::Box<Identifier<'tree>>),
    OperatorIdentifier(::std::boxed::Box<OperatorIdentifier<'tree>>),
}
impl<'tree> ::treesitter_types::FromNode<'tree> for ExportDeclarationPath<'tree> {
    #[allow(clippy::collapsible_else_if)]
    fn from_node(
        node: ::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        match node.kind() {
            "." => Ok(Self::Dot(::treesitter_types::Span::from(node))),
            "identifier" => Ok(Self::Identifier(::std::boxed::Box::new(
                <Identifier as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "operator_identifier" => Ok(Self::OperatorIdentifier(::std::boxed::Box::new(
                <OperatorIdentifier as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            other => Err(::treesitter_types::ParseError::unexpected_kind(other, node)),
        }
    }
}
impl ::treesitter_types::Spanned for ExportDeclarationPath<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        match self {
            Self::Dot(span) => *span,
            Self::Identifier(inner) => inner.span(),
            Self::OperatorIdentifier(inner) => inner.span(),
        }
    }
}
#[derive(Debug, Clone)]
pub enum ExportDeclarationChildren<'tree> {
    AsRenamedIdentifier(::std::boxed::Box<AsRenamedIdentifier<'tree>>),
    NamespaceSelectors(::std::boxed::Box<NamespaceSelectors<'tree>>),
    NamespaceWildcard(::std::boxed::Box<NamespaceWildcard<'tree>>),
}
impl<'tree> ::treesitter_types::FromNode<'tree> for ExportDeclarationChildren<'tree> {
    #[allow(clippy::collapsible_else_if)]
    fn from_node(
        node: ::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        match node.kind() {
            "as_renamed_identifier" => Ok(Self::AsRenamedIdentifier(::std::boxed::Box::new(
                <AsRenamedIdentifier as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "namespace_selectors" => Ok(Self::NamespaceSelectors(::std::boxed::Box::new(
                <NamespaceSelectors as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "namespace_wildcard" => Ok(Self::NamespaceWildcard(::std::boxed::Box::new(
                <NamespaceWildcard as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            other => Err(::treesitter_types::ParseError::unexpected_kind(other, node)),
        }
    }
}
impl ::treesitter_types::Spanned for ExportDeclarationChildren<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        match self {
            Self::AsRenamedIdentifier(inner) => inner.span(),
            Self::NamespaceSelectors(inner) => inner.span(),
            Self::NamespaceWildcard(inner) => inner.span(),
        }
    }
}
#[derive(Debug, Clone)]
pub enum ExtendsClauseType<'tree> {
    Comma(::treesitter_types::Span),
    AnnotatedType(::std::boxed::Box<AnnotatedType<'tree>>),
    AppliedConstructorType(::std::boxed::Box<AppliedConstructorType<'tree>>),
    Arguments(::std::boxed::Box<Arguments<'tree>>),
    CompoundType(::std::boxed::Box<CompoundType<'tree>>),
    GenericType(::std::boxed::Box<GenericType<'tree>>),
    NamedTupleType(::std::boxed::Box<NamedTupleType<'tree>>),
    ProjectedType(::std::boxed::Box<ProjectedType<'tree>>),
    SingletonType(::std::boxed::Box<SingletonType<'tree>>),
    StableTypeIdentifier(::std::boxed::Box<StableTypeIdentifier<'tree>>),
    StructuralType(::std::boxed::Box<StructuralType<'tree>>),
    TupleType(::std::boxed::Box<TupleType<'tree>>),
    TypeIdentifier(::std::boxed::Box<TypeIdentifier<'tree>>),
    Wildcard(::std::boxed::Box<Wildcard<'tree>>),
    With(::treesitter_types::Span),
}
impl<'tree> ::treesitter_types::FromNode<'tree> for ExtendsClauseType<'tree> {
    #[allow(clippy::collapsible_else_if)]
    fn from_node(
        node: ::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        match node.kind() {
            "," => Ok(Self::Comma(::treesitter_types::Span::from(node))),
            "annotated_type" => Ok(Self::AnnotatedType(::std::boxed::Box::new(
                <AnnotatedType as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "applied_constructor_type" => Ok(Self::AppliedConstructorType(::std::boxed::Box::new(
                <AppliedConstructorType as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "arguments" => Ok(Self::Arguments(::std::boxed::Box::new(
                <Arguments as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "compound_type" => Ok(Self::CompoundType(::std::boxed::Box::new(
                <CompoundType as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "generic_type" => Ok(Self::GenericType(::std::boxed::Box::new(
                <GenericType as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "named_tuple_type" => Ok(Self::NamedTupleType(::std::boxed::Box::new(
                <NamedTupleType as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "projected_type" => Ok(Self::ProjectedType(::std::boxed::Box::new(
                <ProjectedType as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "singleton_type" => Ok(Self::SingletonType(::std::boxed::Box::new(
                <SingletonType as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "stable_type_identifier" => Ok(Self::StableTypeIdentifier(::std::boxed::Box::new(
                <StableTypeIdentifier as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "structural_type" => Ok(Self::StructuralType(::std::boxed::Box::new(
                <StructuralType as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "tuple_type" => Ok(Self::TupleType(::std::boxed::Box::new(
                <TupleType as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "type_identifier" => Ok(Self::TypeIdentifier(::std::boxed::Box::new(
                <TypeIdentifier as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "wildcard" => Ok(Self::Wildcard(::std::boxed::Box::new(
                <Wildcard as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "with" => Ok(Self::With(::treesitter_types::Span::from(node))),
            other => Err(::treesitter_types::ParseError::unexpected_kind(other, node)),
        }
    }
}
impl ::treesitter_types::Spanned for ExtendsClauseType<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        match self {
            Self::Comma(span) => *span,
            Self::AnnotatedType(inner) => inner.span(),
            Self::AppliedConstructorType(inner) => inner.span(),
            Self::Arguments(inner) => inner.span(),
            Self::CompoundType(inner) => inner.span(),
            Self::GenericType(inner) => inner.span(),
            Self::NamedTupleType(inner) => inner.span(),
            Self::ProjectedType(inner) => inner.span(),
            Self::SingletonType(inner) => inner.span(),
            Self::StableTypeIdentifier(inner) => inner.span(),
            Self::StructuralType(inner) => inner.span(),
            Self::TupleType(inner) => inner.span(),
            Self::TypeIdentifier(inner) => inner.span(),
            Self::Wildcard(inner) => inner.span(),
            Self::With(span) => *span,
        }
    }
}
#[derive(Debug, Clone)]
pub enum ExtensionDefinitionBody<'tree> {
    Semicolon(::treesitter_types::Span),
    Definition(::std::boxed::Box<Definition<'tree>>),
    EndIdent(::treesitter_types::Span),
    End(::treesitter_types::Span),
    Expression(::std::boxed::Box<Expression<'tree>>),
    Extension(::treesitter_types::Span),
    For(::treesitter_types::Span),
    Given(::treesitter_types::Span),
    If(::treesitter_types::Span),
    Match(::treesitter_types::Span),
    New(::treesitter_types::Span),
    This(::treesitter_types::Span),
    Try(::treesitter_types::Span),
    Val(::treesitter_types::Span),
    While(::treesitter_types::Span),
    LBrace(::treesitter_types::Span),
    RBrace(::treesitter_types::Span),
}
impl<'tree> ::treesitter_types::FromNode<'tree> for ExtensionDefinitionBody<'tree> {
    #[allow(clippy::collapsible_else_if)]
    fn from_node(
        node: ::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        match node.kind() {
            ";" => Ok(Self::Semicolon(::treesitter_types::Span::from(node))),
            "_end_ident" => Ok(Self::EndIdent(::treesitter_types::Span::from(node))),
            "end" => Ok(Self::End(::treesitter_types::Span::from(node))),
            "extension" => Ok(Self::Extension(::treesitter_types::Span::from(node))),
            "for" => Ok(Self::For(::treesitter_types::Span::from(node))),
            "given" => Ok(Self::Given(::treesitter_types::Span::from(node))),
            "if" => Ok(Self::If(::treesitter_types::Span::from(node))),
            "match" => Ok(Self::Match(::treesitter_types::Span::from(node))),
            "new" => Ok(Self::New(::treesitter_types::Span::from(node))),
            "this" => Ok(Self::This(::treesitter_types::Span::from(node))),
            "try" => Ok(Self::Try(::treesitter_types::Span::from(node))),
            "val" => Ok(Self::Val(::treesitter_types::Span::from(node))),
            "while" => Ok(Self::While(::treesitter_types::Span::from(node))),
            "{" => Ok(Self::LBrace(::treesitter_types::Span::from(node))),
            "}" => Ok(Self::RBrace(::treesitter_types::Span::from(node))),
            _other => {
                if let Ok(v) = <Definition as ::treesitter_types::FromNode>::from_node(node, src) {
                    Ok(Self::Definition(::std::boxed::Box::new(v)))
                } else {
                    if let Ok(v) =
                        <Expression as ::treesitter_types::FromNode>::from_node(node, src)
                    {
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
}
impl ::treesitter_types::Spanned for ExtensionDefinitionBody<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        match self {
            Self::Semicolon(span) => *span,
            Self::Definition(inner) => inner.span(),
            Self::EndIdent(span) => *span,
            Self::End(span) => *span,
            Self::Expression(inner) => inner.span(),
            Self::Extension(span) => *span,
            Self::For(span) => *span,
            Self::Given(span) => *span,
            Self::If(span) => *span,
            Self::Match(span) => *span,
            Self::New(span) => *span,
            Self::This(span) => *span,
            Self::Try(span) => *span,
            Self::Val(span) => *span,
            Self::While(span) => *span,
            Self::LBrace(span) => *span,
            Self::RBrace(span) => *span,
        }
    }
}
#[derive(Debug, Clone)]
pub enum FieldExpressionField<'tree> {
    Identifier(::std::boxed::Box<Identifier<'tree>>),
    OperatorIdentifier(::std::boxed::Box<OperatorIdentifier<'tree>>),
}
impl<'tree> ::treesitter_types::FromNode<'tree> for FieldExpressionField<'tree> {
    #[allow(clippy::collapsible_else_if)]
    fn from_node(
        node: ::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        match node.kind() {
            "identifier" => Ok(Self::Identifier(::std::boxed::Box::new(
                <Identifier as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "operator_identifier" => Ok(Self::OperatorIdentifier(::std::boxed::Box::new(
                <OperatorIdentifier as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            other => Err(::treesitter_types::ParseError::unexpected_kind(other, node)),
        }
    }
}
impl ::treesitter_types::Spanned for FieldExpressionField<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        match self {
            Self::Identifier(inner) => inner.span(),
            Self::OperatorIdentifier(inner) => inner.span(),
        }
    }
}
#[derive(Debug, Clone)]
pub enum FieldExpressionValue<'tree> {
    Block(::std::boxed::Box<Block<'tree>>),
    BooleanLiteral(::std::boxed::Box<BooleanLiteral<'tree>>),
    CallExpression(::std::boxed::Box<CallExpression<'tree>>),
    CaseBlock(::std::boxed::Box<CaseBlock<'tree>>),
    CharacterLiteral(::std::boxed::Box<CharacterLiteral<'tree>>),
    FieldExpression(::std::boxed::Box<FieldExpression<'tree>>),
    FloatingPointLiteral(::std::boxed::Box<FloatingPointLiteral<'tree>>),
    GenericFunction(::std::boxed::Box<GenericFunction<'tree>>),
    Identifier(::std::boxed::Box<Identifier<'tree>>),
    InstanceExpression(::std::boxed::Box<InstanceExpression<'tree>>),
    IntegerLiteral(::std::boxed::Box<IntegerLiteral<'tree>>),
    InterpolatedStringExpression(::std::boxed::Box<InterpolatedStringExpression<'tree>>),
    NullLiteral(::std::boxed::Box<NullLiteral<'tree>>),
    OperatorIdentifier(::std::boxed::Box<OperatorIdentifier<'tree>>),
    ParenthesizedExpression(::std::boxed::Box<ParenthesizedExpression<'tree>>),
    QuoteExpression(::std::boxed::Box<QuoteExpression<'tree>>),
    SpliceExpression(::std::boxed::Box<SpliceExpression<'tree>>),
    String(::std::boxed::Box<String<'tree>>),
    TupleExpression(::std::boxed::Box<TupleExpression<'tree>>),
    Unit(::std::boxed::Box<Unit<'tree>>),
    Wildcard(::std::boxed::Box<Wildcard<'tree>>),
}
impl<'tree> ::treesitter_types::FromNode<'tree> for FieldExpressionValue<'tree> {
    #[allow(clippy::collapsible_else_if)]
    fn from_node(
        node: ::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        match node.kind() {
            "block" => Ok(Self::Block(::std::boxed::Box::new(
                <Block as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "boolean_literal" => Ok(Self::BooleanLiteral(::std::boxed::Box::new(
                <BooleanLiteral as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "call_expression" => Ok(Self::CallExpression(::std::boxed::Box::new(
                <CallExpression as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "case_block" => Ok(Self::CaseBlock(::std::boxed::Box::new(
                <CaseBlock as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "character_literal" => Ok(Self::CharacterLiteral(::std::boxed::Box::new(
                <CharacterLiteral as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "field_expression" => Ok(Self::FieldExpression(::std::boxed::Box::new(
                <FieldExpression as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "floating_point_literal" => Ok(Self::FloatingPointLiteral(::std::boxed::Box::new(
                <FloatingPointLiteral as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "generic_function" => Ok(Self::GenericFunction(::std::boxed::Box::new(
                <GenericFunction as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "identifier" => Ok(Self::Identifier(::std::boxed::Box::new(
                <Identifier as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "instance_expression" => Ok(Self::InstanceExpression(::std::boxed::Box::new(
                <InstanceExpression as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "integer_literal" => Ok(Self::IntegerLiteral(::std::boxed::Box::new(
                <IntegerLiteral as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "interpolated_string_expression" => {
                Ok(Self::InterpolatedStringExpression(::std::boxed::Box::new(
                    <InterpolatedStringExpression as ::treesitter_types::FromNode>::from_node(
                        node, src,
                    )?,
                )))
            }
            "null_literal" => Ok(Self::NullLiteral(::std::boxed::Box::new(
                <NullLiteral as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "operator_identifier" => Ok(Self::OperatorIdentifier(::std::boxed::Box::new(
                <OperatorIdentifier as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "parenthesized_expression" => {
                Ok(Self::ParenthesizedExpression(::std::boxed::Box::new(
                    <ParenthesizedExpression as ::treesitter_types::FromNode>::from_node(
                        node, src,
                    )?,
                )))
            }
            "quote_expression" => Ok(Self::QuoteExpression(::std::boxed::Box::new(
                <QuoteExpression as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "splice_expression" => Ok(Self::SpliceExpression(::std::boxed::Box::new(
                <SpliceExpression as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "string" => Ok(Self::String(::std::boxed::Box::new(
                <String as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "tuple_expression" => Ok(Self::TupleExpression(::std::boxed::Box::new(
                <TupleExpression as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "unit" => Ok(Self::Unit(::std::boxed::Box::new(
                <Unit as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "wildcard" => Ok(Self::Wildcard(::std::boxed::Box::new(
                <Wildcard as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            other => Err(::treesitter_types::ParseError::unexpected_kind(other, node)),
        }
    }
}
impl ::treesitter_types::Spanned for FieldExpressionValue<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        match self {
            Self::Block(inner) => inner.span(),
            Self::BooleanLiteral(inner) => inner.span(),
            Self::CallExpression(inner) => inner.span(),
            Self::CaseBlock(inner) => inner.span(),
            Self::CharacterLiteral(inner) => inner.span(),
            Self::FieldExpression(inner) => inner.span(),
            Self::FloatingPointLiteral(inner) => inner.span(),
            Self::GenericFunction(inner) => inner.span(),
            Self::Identifier(inner) => inner.span(),
            Self::InstanceExpression(inner) => inner.span(),
            Self::IntegerLiteral(inner) => inner.span(),
            Self::InterpolatedStringExpression(inner) => inner.span(),
            Self::NullLiteral(inner) => inner.span(),
            Self::OperatorIdentifier(inner) => inner.span(),
            Self::ParenthesizedExpression(inner) => inner.span(),
            Self::QuoteExpression(inner) => inner.span(),
            Self::SpliceExpression(inner) => inner.span(),
            Self::String(inner) => inner.span(),
            Self::TupleExpression(inner) => inner.span(),
            Self::Unit(inner) => inner.span(),
            Self::Wildcard(inner) => inner.span(),
        }
    }
}
#[derive(Debug, Clone)]
pub enum FinallyClauseChildren<'tree> {
    Expression(::std::boxed::Box<Expression<'tree>>),
    IndentedBlock(::std::boxed::Box<IndentedBlock<'tree>>),
    IndentedCases(::std::boxed::Box<IndentedCases<'tree>>),
}
impl<'tree> ::treesitter_types::FromNode<'tree> for FinallyClauseChildren<'tree> {
    #[allow(clippy::collapsible_else_if)]
    fn from_node(
        node: ::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        match node.kind() {
            "indented_block" => Ok(Self::IndentedBlock(::std::boxed::Box::new(
                <IndentedBlock as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "indented_cases" => Ok(Self::IndentedCases(::std::boxed::Box::new(
                <IndentedCases as ::treesitter_types::FromNode>::from_node(node, src)?,
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
impl ::treesitter_types::Spanned for FinallyClauseChildren<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        match self {
            Self::Expression(inner) => inner.span(),
            Self::IndentedBlock(inner) => inner.span(),
            Self::IndentedCases(inner) => inner.span(),
        }
    }
}
#[derive(Debug, Clone)]
pub enum ForExpressionBody<'tree> {
    Expression(::std::boxed::Box<Expression<'tree>>),
    IndentedBlock(::std::boxed::Box<IndentedBlock<'tree>>),
    IndentedCases(::std::boxed::Box<IndentedCases<'tree>>),
}
impl<'tree> ::treesitter_types::FromNode<'tree> for ForExpressionBody<'tree> {
    #[allow(clippy::collapsible_else_if)]
    fn from_node(
        node: ::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        match node.kind() {
            "indented_block" => Ok(Self::IndentedBlock(::std::boxed::Box::new(
                <IndentedBlock as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "indented_cases" => Ok(Self::IndentedCases(::std::boxed::Box::new(
                <IndentedCases as ::treesitter_types::FromNode>::from_node(node, src)?,
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
impl ::treesitter_types::Spanned for ForExpressionBody<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        match self {
            Self::Expression(inner) => inner.span(),
            Self::IndentedBlock(inner) => inner.span(),
            Self::IndentedCases(inner) => inner.span(),
        }
    }
}
#[derive(Debug, Clone)]
pub enum ForExpressionEnumerators<'tree> {
    LParen(::treesitter_types::Span),
    RParen(::treesitter_types::Span),
    Enumerators(::std::boxed::Box<Enumerators<'tree>>),
    LBrace(::treesitter_types::Span),
    RBrace(::treesitter_types::Span),
}
impl<'tree> ::treesitter_types::FromNode<'tree> for ForExpressionEnumerators<'tree> {
    #[allow(clippy::collapsible_else_if)]
    fn from_node(
        node: ::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        match node.kind() {
            "(" => Ok(Self::LParen(::treesitter_types::Span::from(node))),
            ")" => Ok(Self::RParen(::treesitter_types::Span::from(node))),
            "enumerators" => Ok(Self::Enumerators(::std::boxed::Box::new(
                <Enumerators as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "{" => Ok(Self::LBrace(::treesitter_types::Span::from(node))),
            "}" => Ok(Self::RBrace(::treesitter_types::Span::from(node))),
            other => Err(::treesitter_types::ParseError::unexpected_kind(other, node)),
        }
    }
}
impl ::treesitter_types::Spanned for ForExpressionEnumerators<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        match self {
            Self::LParen(span) => *span,
            Self::RParen(span) => *span,
            Self::Enumerators(inner) => inner.span(),
            Self::LBrace(span) => *span,
            Self::RBrace(span) => *span,
        }
    }
}
#[derive(Debug, Clone)]
pub enum FullEnumCaseName<'tree> {
    Identifier(::std::boxed::Box<Identifier<'tree>>),
    OperatorIdentifier(::std::boxed::Box<OperatorIdentifier<'tree>>),
}
impl<'tree> ::treesitter_types::FromNode<'tree> for FullEnumCaseName<'tree> {
    #[allow(clippy::collapsible_else_if)]
    fn from_node(
        node: ::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        match node.kind() {
            "identifier" => Ok(Self::Identifier(::std::boxed::Box::new(
                <Identifier as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "operator_identifier" => Ok(Self::OperatorIdentifier(::std::boxed::Box::new(
                <OperatorIdentifier as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            other => Err(::treesitter_types::ParseError::unexpected_kind(other, node)),
        }
    }
}
impl ::treesitter_types::Spanned for FullEnumCaseName<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        match self {
            Self::Identifier(inner) => inner.span(),
            Self::OperatorIdentifier(inner) => inner.span(),
        }
    }
}
#[derive(Debug, Clone)]
pub enum FunctionDeclarationName<'tree> {
    Identifier(::std::boxed::Box<Identifier<'tree>>),
    OperatorIdentifier(::std::boxed::Box<OperatorIdentifier<'tree>>),
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
            "operator_identifier" => Ok(Self::OperatorIdentifier(::std::boxed::Box::new(
                <OperatorIdentifier as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            other => Err(::treesitter_types::ParseError::unexpected_kind(other, node)),
        }
    }
}
impl ::treesitter_types::Spanned for FunctionDeclarationName<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        match self {
            Self::Identifier(inner) => inner.span(),
            Self::OperatorIdentifier(inner) => inner.span(),
        }
    }
}
#[derive(Debug, Clone)]
pub enum FunctionDeclarationParameters<'tree> {
    Parameters(::std::boxed::Box<Parameters<'tree>>),
    TypeParameters(::std::boxed::Box<TypeParameters<'tree>>),
}
impl<'tree> ::treesitter_types::FromNode<'tree> for FunctionDeclarationParameters<'tree> {
    #[allow(clippy::collapsible_else_if)]
    fn from_node(
        node: ::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        match node.kind() {
            "parameters" => Ok(Self::Parameters(::std::boxed::Box::new(
                <Parameters as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "type_parameters" => Ok(Self::TypeParameters(::std::boxed::Box::new(
                <TypeParameters as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            other => Err(::treesitter_types::ParseError::unexpected_kind(other, node)),
        }
    }
}
impl ::treesitter_types::Spanned for FunctionDeclarationParameters<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        match self {
            Self::Parameters(inner) => inner.span(),
            Self::TypeParameters(inner) => inner.span(),
        }
    }
}
#[derive(Debug, Clone)]
pub enum FunctionDeclarationReturnType<'tree> {
    AnnotatedType(::std::boxed::Box<AnnotatedType<'tree>>),
    AppliedConstructorType(::std::boxed::Box<AppliedConstructorType<'tree>>),
    CompoundType(::std::boxed::Box<CompoundType<'tree>>),
    FunctionType(::std::boxed::Box<FunctionType<'tree>>),
    GenericType(::std::boxed::Box<GenericType<'tree>>),
    InfixType(::std::boxed::Box<InfixType<'tree>>),
    LiteralType(::std::boxed::Box<LiteralType<'tree>>),
    MatchType(::std::boxed::Box<MatchType<'tree>>),
    NamedTupleType(::std::boxed::Box<NamedTupleType<'tree>>),
    ProjectedType(::std::boxed::Box<ProjectedType<'tree>>),
    SingletonType(::std::boxed::Box<SingletonType<'tree>>),
    StableTypeIdentifier(::std::boxed::Box<StableTypeIdentifier<'tree>>),
    StructuralType(::std::boxed::Box<StructuralType<'tree>>),
    TupleType(::std::boxed::Box<TupleType<'tree>>),
    TypeIdentifier(::std::boxed::Box<TypeIdentifier<'tree>>),
    TypeLambda(::std::boxed::Box<TypeLambda<'tree>>),
    Wildcard(::std::boxed::Box<Wildcard<'tree>>),
}
impl<'tree> ::treesitter_types::FromNode<'tree> for FunctionDeclarationReturnType<'tree> {
    #[allow(clippy::collapsible_else_if)]
    fn from_node(
        node: ::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        match node.kind() {
            "annotated_type" => Ok(Self::AnnotatedType(::std::boxed::Box::new(
                <AnnotatedType as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "applied_constructor_type" => Ok(Self::AppliedConstructorType(::std::boxed::Box::new(
                <AppliedConstructorType as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "compound_type" => Ok(Self::CompoundType(::std::boxed::Box::new(
                <CompoundType as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "function_type" => Ok(Self::FunctionType(::std::boxed::Box::new(
                <FunctionType as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "generic_type" => Ok(Self::GenericType(::std::boxed::Box::new(
                <GenericType as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "infix_type" => Ok(Self::InfixType(::std::boxed::Box::new(
                <InfixType as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "literal_type" => Ok(Self::LiteralType(::std::boxed::Box::new(
                <LiteralType as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "match_type" => Ok(Self::MatchType(::std::boxed::Box::new(
                <MatchType as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "named_tuple_type" => Ok(Self::NamedTupleType(::std::boxed::Box::new(
                <NamedTupleType as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "projected_type" => Ok(Self::ProjectedType(::std::boxed::Box::new(
                <ProjectedType as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "singleton_type" => Ok(Self::SingletonType(::std::boxed::Box::new(
                <SingletonType as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "stable_type_identifier" => Ok(Self::StableTypeIdentifier(::std::boxed::Box::new(
                <StableTypeIdentifier as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "structural_type" => Ok(Self::StructuralType(::std::boxed::Box::new(
                <StructuralType as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "tuple_type" => Ok(Self::TupleType(::std::boxed::Box::new(
                <TupleType as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "type_identifier" => Ok(Self::TypeIdentifier(::std::boxed::Box::new(
                <TypeIdentifier as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "type_lambda" => Ok(Self::TypeLambda(::std::boxed::Box::new(
                <TypeLambda as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "wildcard" => Ok(Self::Wildcard(::std::boxed::Box::new(
                <Wildcard as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            other => Err(::treesitter_types::ParseError::unexpected_kind(other, node)),
        }
    }
}
impl ::treesitter_types::Spanned for FunctionDeclarationReturnType<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        match self {
            Self::AnnotatedType(inner) => inner.span(),
            Self::AppliedConstructorType(inner) => inner.span(),
            Self::CompoundType(inner) => inner.span(),
            Self::FunctionType(inner) => inner.span(),
            Self::GenericType(inner) => inner.span(),
            Self::InfixType(inner) => inner.span(),
            Self::LiteralType(inner) => inner.span(),
            Self::MatchType(inner) => inner.span(),
            Self::NamedTupleType(inner) => inner.span(),
            Self::ProjectedType(inner) => inner.span(),
            Self::SingletonType(inner) => inner.span(),
            Self::StableTypeIdentifier(inner) => inner.span(),
            Self::StructuralType(inner) => inner.span(),
            Self::TupleType(inner) => inner.span(),
            Self::TypeIdentifier(inner) => inner.span(),
            Self::TypeLambda(inner) => inner.span(),
            Self::Wildcard(inner) => inner.span(),
        }
    }
}
#[derive(Debug, Clone)]
pub enum FunctionDeclarationChildren<'tree> {
    Annotation(::std::boxed::Box<Annotation<'tree>>),
    Modifiers(::std::boxed::Box<Modifiers<'tree>>),
}
impl<'tree> ::treesitter_types::FromNode<'tree> for FunctionDeclarationChildren<'tree> {
    #[allow(clippy::collapsible_else_if)]
    fn from_node(
        node: ::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        match node.kind() {
            "annotation" => Ok(Self::Annotation(::std::boxed::Box::new(
                <Annotation as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "modifiers" => Ok(Self::Modifiers(::std::boxed::Box::new(
                <Modifiers as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            other => Err(::treesitter_types::ParseError::unexpected_kind(other, node)),
        }
    }
}
impl ::treesitter_types::Spanned for FunctionDeclarationChildren<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        match self {
            Self::Annotation(inner) => inner.span(),
            Self::Modifiers(inner) => inner.span(),
        }
    }
}
#[derive(Debug, Clone)]
pub enum FunctionDefinitionBody<'tree> {
    Expression(::std::boxed::Box<Expression<'tree>>),
    IndentedBlock(::std::boxed::Box<IndentedBlock<'tree>>),
    IndentedCases(::std::boxed::Box<IndentedCases<'tree>>),
}
impl<'tree> ::treesitter_types::FromNode<'tree> for FunctionDefinitionBody<'tree> {
    #[allow(clippy::collapsible_else_if)]
    fn from_node(
        node: ::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        match node.kind() {
            "indented_block" => Ok(Self::IndentedBlock(::std::boxed::Box::new(
                <IndentedBlock as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "indented_cases" => Ok(Self::IndentedCases(::std::boxed::Box::new(
                <IndentedCases as ::treesitter_types::FromNode>::from_node(node, src)?,
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
impl ::treesitter_types::Spanned for FunctionDefinitionBody<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        match self {
            Self::Expression(inner) => inner.span(),
            Self::IndentedBlock(inner) => inner.span(),
            Self::IndentedCases(inner) => inner.span(),
        }
    }
}
#[derive(Debug, Clone)]
pub enum FunctionDefinitionName<'tree> {
    Identifier(::std::boxed::Box<Identifier<'tree>>),
    OperatorIdentifier(::std::boxed::Box<OperatorIdentifier<'tree>>),
}
impl<'tree> ::treesitter_types::FromNode<'tree> for FunctionDefinitionName<'tree> {
    #[allow(clippy::collapsible_else_if)]
    fn from_node(
        node: ::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        match node.kind() {
            "identifier" => Ok(Self::Identifier(::std::boxed::Box::new(
                <Identifier as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "operator_identifier" => Ok(Self::OperatorIdentifier(::std::boxed::Box::new(
                <OperatorIdentifier as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            other => Err(::treesitter_types::ParseError::unexpected_kind(other, node)),
        }
    }
}
impl ::treesitter_types::Spanned for FunctionDefinitionName<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        match self {
            Self::Identifier(inner) => inner.span(),
            Self::OperatorIdentifier(inner) => inner.span(),
        }
    }
}
#[derive(Debug, Clone)]
pub enum FunctionDefinitionParameters<'tree> {
    Parameters(::std::boxed::Box<Parameters<'tree>>),
    TypeParameters(::std::boxed::Box<TypeParameters<'tree>>),
}
impl<'tree> ::treesitter_types::FromNode<'tree> for FunctionDefinitionParameters<'tree> {
    #[allow(clippy::collapsible_else_if)]
    fn from_node(
        node: ::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        match node.kind() {
            "parameters" => Ok(Self::Parameters(::std::boxed::Box::new(
                <Parameters as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "type_parameters" => Ok(Self::TypeParameters(::std::boxed::Box::new(
                <TypeParameters as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            other => Err(::treesitter_types::ParseError::unexpected_kind(other, node)),
        }
    }
}
impl ::treesitter_types::Spanned for FunctionDefinitionParameters<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        match self {
            Self::Parameters(inner) => inner.span(),
            Self::TypeParameters(inner) => inner.span(),
        }
    }
}
#[derive(Debug, Clone)]
pub enum FunctionDefinitionReturnType<'tree> {
    AnnotatedType(::std::boxed::Box<AnnotatedType<'tree>>),
    AppliedConstructorType(::std::boxed::Box<AppliedConstructorType<'tree>>),
    CompoundType(::std::boxed::Box<CompoundType<'tree>>),
    FunctionType(::std::boxed::Box<FunctionType<'tree>>),
    GenericType(::std::boxed::Box<GenericType<'tree>>),
    InfixType(::std::boxed::Box<InfixType<'tree>>),
    LiteralType(::std::boxed::Box<LiteralType<'tree>>),
    MatchType(::std::boxed::Box<MatchType<'tree>>),
    NamedTupleType(::std::boxed::Box<NamedTupleType<'tree>>),
    ProjectedType(::std::boxed::Box<ProjectedType<'tree>>),
    SingletonType(::std::boxed::Box<SingletonType<'tree>>),
    StableTypeIdentifier(::std::boxed::Box<StableTypeIdentifier<'tree>>),
    StructuralType(::std::boxed::Box<StructuralType<'tree>>),
    TupleType(::std::boxed::Box<TupleType<'tree>>),
    TypeIdentifier(::std::boxed::Box<TypeIdentifier<'tree>>),
    TypeLambda(::std::boxed::Box<TypeLambda<'tree>>),
    Wildcard(::std::boxed::Box<Wildcard<'tree>>),
}
impl<'tree> ::treesitter_types::FromNode<'tree> for FunctionDefinitionReturnType<'tree> {
    #[allow(clippy::collapsible_else_if)]
    fn from_node(
        node: ::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        match node.kind() {
            "annotated_type" => Ok(Self::AnnotatedType(::std::boxed::Box::new(
                <AnnotatedType as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "applied_constructor_type" => Ok(Self::AppliedConstructorType(::std::boxed::Box::new(
                <AppliedConstructorType as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "compound_type" => Ok(Self::CompoundType(::std::boxed::Box::new(
                <CompoundType as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "function_type" => Ok(Self::FunctionType(::std::boxed::Box::new(
                <FunctionType as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "generic_type" => Ok(Self::GenericType(::std::boxed::Box::new(
                <GenericType as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "infix_type" => Ok(Self::InfixType(::std::boxed::Box::new(
                <InfixType as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "literal_type" => Ok(Self::LiteralType(::std::boxed::Box::new(
                <LiteralType as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "match_type" => Ok(Self::MatchType(::std::boxed::Box::new(
                <MatchType as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "named_tuple_type" => Ok(Self::NamedTupleType(::std::boxed::Box::new(
                <NamedTupleType as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "projected_type" => Ok(Self::ProjectedType(::std::boxed::Box::new(
                <ProjectedType as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "singleton_type" => Ok(Self::SingletonType(::std::boxed::Box::new(
                <SingletonType as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "stable_type_identifier" => Ok(Self::StableTypeIdentifier(::std::boxed::Box::new(
                <StableTypeIdentifier as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "structural_type" => Ok(Self::StructuralType(::std::boxed::Box::new(
                <StructuralType as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "tuple_type" => Ok(Self::TupleType(::std::boxed::Box::new(
                <TupleType as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "type_identifier" => Ok(Self::TypeIdentifier(::std::boxed::Box::new(
                <TypeIdentifier as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "type_lambda" => Ok(Self::TypeLambda(::std::boxed::Box::new(
                <TypeLambda as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "wildcard" => Ok(Self::Wildcard(::std::boxed::Box::new(
                <Wildcard as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            other => Err(::treesitter_types::ParseError::unexpected_kind(other, node)),
        }
    }
}
impl ::treesitter_types::Spanned for FunctionDefinitionReturnType<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        match self {
            Self::AnnotatedType(inner) => inner.span(),
            Self::AppliedConstructorType(inner) => inner.span(),
            Self::CompoundType(inner) => inner.span(),
            Self::FunctionType(inner) => inner.span(),
            Self::GenericType(inner) => inner.span(),
            Self::InfixType(inner) => inner.span(),
            Self::LiteralType(inner) => inner.span(),
            Self::MatchType(inner) => inner.span(),
            Self::NamedTupleType(inner) => inner.span(),
            Self::ProjectedType(inner) => inner.span(),
            Self::SingletonType(inner) => inner.span(),
            Self::StableTypeIdentifier(inner) => inner.span(),
            Self::StructuralType(inner) => inner.span(),
            Self::TupleType(inner) => inner.span(),
            Self::TypeIdentifier(inner) => inner.span(),
            Self::TypeLambda(inner) => inner.span(),
            Self::Wildcard(inner) => inner.span(),
        }
    }
}
#[derive(Debug, Clone)]
pub enum FunctionDefinitionChildren<'tree> {
    Annotation(::std::boxed::Box<Annotation<'tree>>),
    Modifiers(::std::boxed::Box<Modifiers<'tree>>),
}
impl<'tree> ::treesitter_types::FromNode<'tree> for FunctionDefinitionChildren<'tree> {
    #[allow(clippy::collapsible_else_if)]
    fn from_node(
        node: ::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        match node.kind() {
            "annotation" => Ok(Self::Annotation(::std::boxed::Box::new(
                <Annotation as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "modifiers" => Ok(Self::Modifiers(::std::boxed::Box::new(
                <Modifiers as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            other => Err(::treesitter_types::ParseError::unexpected_kind(other, node)),
        }
    }
}
impl ::treesitter_types::Spanned for FunctionDefinitionChildren<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        match self {
            Self::Annotation(inner) => inner.span(),
            Self::Modifiers(inner) => inner.span(),
        }
    }
}
#[derive(Debug, Clone)]
pub enum FunctionTypeReturnType<'tree> {
    AnnotatedType(::std::boxed::Box<AnnotatedType<'tree>>),
    AppliedConstructorType(::std::boxed::Box<AppliedConstructorType<'tree>>),
    CompoundType(::std::boxed::Box<CompoundType<'tree>>),
    FunctionType(::std::boxed::Box<FunctionType<'tree>>),
    GenericType(::std::boxed::Box<GenericType<'tree>>),
    InfixType(::std::boxed::Box<InfixType<'tree>>),
    LiteralType(::std::boxed::Box<LiteralType<'tree>>),
    MatchType(::std::boxed::Box<MatchType<'tree>>),
    NamedTupleType(::std::boxed::Box<NamedTupleType<'tree>>),
    ProjectedType(::std::boxed::Box<ProjectedType<'tree>>),
    SingletonType(::std::boxed::Box<SingletonType<'tree>>),
    StableTypeIdentifier(::std::boxed::Box<StableTypeIdentifier<'tree>>),
    StructuralType(::std::boxed::Box<StructuralType<'tree>>),
    TupleType(::std::boxed::Box<TupleType<'tree>>),
    TypeIdentifier(::std::boxed::Box<TypeIdentifier<'tree>>),
    TypeLambda(::std::boxed::Box<TypeLambda<'tree>>),
    Wildcard(::std::boxed::Box<Wildcard<'tree>>),
}
impl<'tree> ::treesitter_types::FromNode<'tree> for FunctionTypeReturnType<'tree> {
    #[allow(clippy::collapsible_else_if)]
    fn from_node(
        node: ::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        match node.kind() {
            "annotated_type" => Ok(Self::AnnotatedType(::std::boxed::Box::new(
                <AnnotatedType as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "applied_constructor_type" => Ok(Self::AppliedConstructorType(::std::boxed::Box::new(
                <AppliedConstructorType as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "compound_type" => Ok(Self::CompoundType(::std::boxed::Box::new(
                <CompoundType as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "function_type" => Ok(Self::FunctionType(::std::boxed::Box::new(
                <FunctionType as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "generic_type" => Ok(Self::GenericType(::std::boxed::Box::new(
                <GenericType as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "infix_type" => Ok(Self::InfixType(::std::boxed::Box::new(
                <InfixType as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "literal_type" => Ok(Self::LiteralType(::std::boxed::Box::new(
                <LiteralType as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "match_type" => Ok(Self::MatchType(::std::boxed::Box::new(
                <MatchType as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "named_tuple_type" => Ok(Self::NamedTupleType(::std::boxed::Box::new(
                <NamedTupleType as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "projected_type" => Ok(Self::ProjectedType(::std::boxed::Box::new(
                <ProjectedType as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "singleton_type" => Ok(Self::SingletonType(::std::boxed::Box::new(
                <SingletonType as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "stable_type_identifier" => Ok(Self::StableTypeIdentifier(::std::boxed::Box::new(
                <StableTypeIdentifier as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "structural_type" => Ok(Self::StructuralType(::std::boxed::Box::new(
                <StructuralType as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "tuple_type" => Ok(Self::TupleType(::std::boxed::Box::new(
                <TupleType as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "type_identifier" => Ok(Self::TypeIdentifier(::std::boxed::Box::new(
                <TypeIdentifier as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "type_lambda" => Ok(Self::TypeLambda(::std::boxed::Box::new(
                <TypeLambda as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "wildcard" => Ok(Self::Wildcard(::std::boxed::Box::new(
                <Wildcard as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            other => Err(::treesitter_types::ParseError::unexpected_kind(other, node)),
        }
    }
}
impl ::treesitter_types::Spanned for FunctionTypeReturnType<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        match self {
            Self::AnnotatedType(inner) => inner.span(),
            Self::AppliedConstructorType(inner) => inner.span(),
            Self::CompoundType(inner) => inner.span(),
            Self::FunctionType(inner) => inner.span(),
            Self::GenericType(inner) => inner.span(),
            Self::InfixType(inner) => inner.span(),
            Self::LiteralType(inner) => inner.span(),
            Self::MatchType(inner) => inner.span(),
            Self::NamedTupleType(inner) => inner.span(),
            Self::ProjectedType(inner) => inner.span(),
            Self::SingletonType(inner) => inner.span(),
            Self::StableTypeIdentifier(inner) => inner.span(),
            Self::StructuralType(inner) => inner.span(),
            Self::TupleType(inner) => inner.span(),
            Self::TypeIdentifier(inner) => inner.span(),
            Self::TypeLambda(inner) => inner.span(),
            Self::Wildcard(inner) => inner.span(),
        }
    }
}
#[derive(Debug, Clone)]
pub enum GenericTypeType<'tree> {
    AppliedConstructorType(::std::boxed::Box<AppliedConstructorType<'tree>>),
    GenericType(::std::boxed::Box<GenericType<'tree>>),
    NamedTupleType(::std::boxed::Box<NamedTupleType<'tree>>),
    ProjectedType(::std::boxed::Box<ProjectedType<'tree>>),
    SingletonType(::std::boxed::Box<SingletonType<'tree>>),
    StableTypeIdentifier(::std::boxed::Box<StableTypeIdentifier<'tree>>),
    TupleType(::std::boxed::Box<TupleType<'tree>>),
    TypeIdentifier(::std::boxed::Box<TypeIdentifier<'tree>>),
    Wildcard(::std::boxed::Box<Wildcard<'tree>>),
}
impl<'tree> ::treesitter_types::FromNode<'tree> for GenericTypeType<'tree> {
    #[allow(clippy::collapsible_else_if)]
    fn from_node(
        node: ::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        match node.kind() {
            "applied_constructor_type" => Ok(Self::AppliedConstructorType(::std::boxed::Box::new(
                <AppliedConstructorType as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "generic_type" => Ok(Self::GenericType(::std::boxed::Box::new(
                <GenericType as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "named_tuple_type" => Ok(Self::NamedTupleType(::std::boxed::Box::new(
                <NamedTupleType as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "projected_type" => Ok(Self::ProjectedType(::std::boxed::Box::new(
                <ProjectedType as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "singleton_type" => Ok(Self::SingletonType(::std::boxed::Box::new(
                <SingletonType as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "stable_type_identifier" => Ok(Self::StableTypeIdentifier(::std::boxed::Box::new(
                <StableTypeIdentifier as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "tuple_type" => Ok(Self::TupleType(::std::boxed::Box::new(
                <TupleType as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "type_identifier" => Ok(Self::TypeIdentifier(::std::boxed::Box::new(
                <TypeIdentifier as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "wildcard" => Ok(Self::Wildcard(::std::boxed::Box::new(
                <Wildcard as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            other => Err(::treesitter_types::ParseError::unexpected_kind(other, node)),
        }
    }
}
impl ::treesitter_types::Spanned for GenericTypeType<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        match self {
            Self::AppliedConstructorType(inner) => inner.span(),
            Self::GenericType(inner) => inner.span(),
            Self::NamedTupleType(inner) => inner.span(),
            Self::ProjectedType(inner) => inner.span(),
            Self::SingletonType(inner) => inner.span(),
            Self::StableTypeIdentifier(inner) => inner.span(),
            Self::TupleType(inner) => inner.span(),
            Self::TypeIdentifier(inner) => inner.span(),
            Self::Wildcard(inner) => inner.span(),
        }
    }
}
#[derive(Debug, Clone)]
pub enum GivenConditionalChildren<'tree> {
    AnnotatedType(::std::boxed::Box<AnnotatedType<'tree>>),
    AppliedConstructorType(::std::boxed::Box<AppliedConstructorType<'tree>>),
    CompoundType(::std::boxed::Box<CompoundType<'tree>>),
    FunctionType(::std::boxed::Box<FunctionType<'tree>>),
    GenericType(::std::boxed::Box<GenericType<'tree>>),
    InfixType(::std::boxed::Box<InfixType<'tree>>),
    LazyParameterType(::std::boxed::Box<LazyParameterType<'tree>>),
    LiteralType(::std::boxed::Box<LiteralType<'tree>>),
    MatchType(::std::boxed::Box<MatchType<'tree>>),
    NamedTupleType(::std::boxed::Box<NamedTupleType<'tree>>),
    Parameter(::std::boxed::Box<Parameter<'tree>>),
    ProjectedType(::std::boxed::Box<ProjectedType<'tree>>),
    RepeatedParameterType(::std::boxed::Box<RepeatedParameterType<'tree>>),
    SingletonType(::std::boxed::Box<SingletonType<'tree>>),
    StableTypeIdentifier(::std::boxed::Box<StableTypeIdentifier<'tree>>),
    StructuralType(::std::boxed::Box<StructuralType<'tree>>),
    TupleType(::std::boxed::Box<TupleType<'tree>>),
    TypeIdentifier(::std::boxed::Box<TypeIdentifier<'tree>>),
    TypeLambda(::std::boxed::Box<TypeLambda<'tree>>),
    Wildcard(::std::boxed::Box<Wildcard<'tree>>),
}
impl<'tree> ::treesitter_types::FromNode<'tree> for GivenConditionalChildren<'tree> {
    #[allow(clippy::collapsible_else_if)]
    fn from_node(
        node: ::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        match node.kind() {
            "annotated_type" => Ok(Self::AnnotatedType(::std::boxed::Box::new(
                <AnnotatedType as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "applied_constructor_type" => Ok(Self::AppliedConstructorType(::std::boxed::Box::new(
                <AppliedConstructorType as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "compound_type" => Ok(Self::CompoundType(::std::boxed::Box::new(
                <CompoundType as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "function_type" => Ok(Self::FunctionType(::std::boxed::Box::new(
                <FunctionType as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "generic_type" => Ok(Self::GenericType(::std::boxed::Box::new(
                <GenericType as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "infix_type" => Ok(Self::InfixType(::std::boxed::Box::new(
                <InfixType as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "lazy_parameter_type" => Ok(Self::LazyParameterType(::std::boxed::Box::new(
                <LazyParameterType as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "literal_type" => Ok(Self::LiteralType(::std::boxed::Box::new(
                <LiteralType as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "match_type" => Ok(Self::MatchType(::std::boxed::Box::new(
                <MatchType as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "named_tuple_type" => Ok(Self::NamedTupleType(::std::boxed::Box::new(
                <NamedTupleType as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "parameter" => Ok(Self::Parameter(::std::boxed::Box::new(
                <Parameter as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "projected_type" => Ok(Self::ProjectedType(::std::boxed::Box::new(
                <ProjectedType as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "repeated_parameter_type" => Ok(Self::RepeatedParameterType(::std::boxed::Box::new(
                <RepeatedParameterType as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "singleton_type" => Ok(Self::SingletonType(::std::boxed::Box::new(
                <SingletonType as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "stable_type_identifier" => Ok(Self::StableTypeIdentifier(::std::boxed::Box::new(
                <StableTypeIdentifier as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "structural_type" => Ok(Self::StructuralType(::std::boxed::Box::new(
                <StructuralType as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "tuple_type" => Ok(Self::TupleType(::std::boxed::Box::new(
                <TupleType as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "type_identifier" => Ok(Self::TypeIdentifier(::std::boxed::Box::new(
                <TypeIdentifier as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "type_lambda" => Ok(Self::TypeLambda(::std::boxed::Box::new(
                <TypeLambda as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "wildcard" => Ok(Self::Wildcard(::std::boxed::Box::new(
                <Wildcard as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            other => Err(::treesitter_types::ParseError::unexpected_kind(other, node)),
        }
    }
}
impl ::treesitter_types::Spanned for GivenConditionalChildren<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        match self {
            Self::AnnotatedType(inner) => inner.span(),
            Self::AppliedConstructorType(inner) => inner.span(),
            Self::CompoundType(inner) => inner.span(),
            Self::FunctionType(inner) => inner.span(),
            Self::GenericType(inner) => inner.span(),
            Self::InfixType(inner) => inner.span(),
            Self::LazyParameterType(inner) => inner.span(),
            Self::LiteralType(inner) => inner.span(),
            Self::MatchType(inner) => inner.span(),
            Self::NamedTupleType(inner) => inner.span(),
            Self::Parameter(inner) => inner.span(),
            Self::ProjectedType(inner) => inner.span(),
            Self::RepeatedParameterType(inner) => inner.span(),
            Self::SingletonType(inner) => inner.span(),
            Self::StableTypeIdentifier(inner) => inner.span(),
            Self::StructuralType(inner) => inner.span(),
            Self::TupleType(inner) => inner.span(),
            Self::TypeIdentifier(inner) => inner.span(),
            Self::TypeLambda(inner) => inner.span(),
            Self::Wildcard(inner) => inner.span(),
        }
    }
}
#[derive(Debug, Clone)]
pub enum GivenDefinitionBody<'tree> {
    Expression(::std::boxed::Box<Expression<'tree>>),
    IndentedBlock(::std::boxed::Box<IndentedBlock<'tree>>),
    IndentedCases(::std::boxed::Box<IndentedCases<'tree>>),
    WithTemplateBody(::std::boxed::Box<WithTemplateBody<'tree>>),
}
impl<'tree> ::treesitter_types::FromNode<'tree> for GivenDefinitionBody<'tree> {
    #[allow(clippy::collapsible_else_if)]
    fn from_node(
        node: ::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        match node.kind() {
            "indented_block" => Ok(Self::IndentedBlock(::std::boxed::Box::new(
                <IndentedBlock as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "indented_cases" => Ok(Self::IndentedCases(::std::boxed::Box::new(
                <IndentedCases as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "with_template_body" => Ok(Self::WithTemplateBody(::std::boxed::Box::new(
                <WithTemplateBody as ::treesitter_types::FromNode>::from_node(node, src)?,
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
impl ::treesitter_types::Spanned for GivenDefinitionBody<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        match self {
            Self::Expression(inner) => inner.span(),
            Self::IndentedBlock(inner) => inner.span(),
            Self::IndentedCases(inner) => inner.span(),
            Self::WithTemplateBody(inner) => inner.span(),
        }
    }
}
#[derive(Debug, Clone)]
pub enum GivenDefinitionName<'tree> {
    Identifier(::std::boxed::Box<Identifier<'tree>>),
    OperatorIdentifier(::std::boxed::Box<OperatorIdentifier<'tree>>),
}
impl<'tree> ::treesitter_types::FromNode<'tree> for GivenDefinitionName<'tree> {
    #[allow(clippy::collapsible_else_if)]
    fn from_node(
        node: ::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        match node.kind() {
            "identifier" => Ok(Self::Identifier(::std::boxed::Box::new(
                <Identifier as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "operator_identifier" => Ok(Self::OperatorIdentifier(::std::boxed::Box::new(
                <OperatorIdentifier as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            other => Err(::treesitter_types::ParseError::unexpected_kind(other, node)),
        }
    }
}
impl ::treesitter_types::Spanned for GivenDefinitionName<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        match self {
            Self::Identifier(inner) => inner.span(),
            Self::OperatorIdentifier(inner) => inner.span(),
        }
    }
}
#[derive(Debug, Clone)]
pub enum GivenDefinitionReturnType<'tree> {
    Colon(::treesitter_types::Span),
    AnnotatedType(::std::boxed::Box<AnnotatedType<'tree>>),
    AppliedConstructorType(::std::boxed::Box<AppliedConstructorType<'tree>>),
    Arguments(::std::boxed::Box<Arguments<'tree>>),
    CompoundType(::std::boxed::Box<CompoundType<'tree>>),
    GenericType(::std::boxed::Box<GenericType<'tree>>),
    NamedTupleType(::std::boxed::Box<NamedTupleType<'tree>>),
    ProjectedType(::std::boxed::Box<ProjectedType<'tree>>),
    SingletonType(::std::boxed::Box<SingletonType<'tree>>),
    StableTypeIdentifier(::std::boxed::Box<StableTypeIdentifier<'tree>>),
    StructuralType(::std::boxed::Box<StructuralType<'tree>>),
    TupleType(::std::boxed::Box<TupleType<'tree>>),
    TypeIdentifier(::std::boxed::Box<TypeIdentifier<'tree>>),
    Wildcard(::std::boxed::Box<Wildcard<'tree>>),
    With(::treesitter_types::Span),
    WithTemplateBody(::std::boxed::Box<WithTemplateBody<'tree>>),
}
impl<'tree> ::treesitter_types::FromNode<'tree> for GivenDefinitionReturnType<'tree> {
    #[allow(clippy::collapsible_else_if)]
    fn from_node(
        node: ::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        match node.kind() {
            ":" => Ok(Self::Colon(::treesitter_types::Span::from(node))),
            "annotated_type" => Ok(Self::AnnotatedType(::std::boxed::Box::new(
                <AnnotatedType as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "applied_constructor_type" => Ok(Self::AppliedConstructorType(::std::boxed::Box::new(
                <AppliedConstructorType as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "arguments" => Ok(Self::Arguments(::std::boxed::Box::new(
                <Arguments as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "compound_type" => Ok(Self::CompoundType(::std::boxed::Box::new(
                <CompoundType as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "generic_type" => Ok(Self::GenericType(::std::boxed::Box::new(
                <GenericType as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "named_tuple_type" => Ok(Self::NamedTupleType(::std::boxed::Box::new(
                <NamedTupleType as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "projected_type" => Ok(Self::ProjectedType(::std::boxed::Box::new(
                <ProjectedType as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "singleton_type" => Ok(Self::SingletonType(::std::boxed::Box::new(
                <SingletonType as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "stable_type_identifier" => Ok(Self::StableTypeIdentifier(::std::boxed::Box::new(
                <StableTypeIdentifier as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "structural_type" => Ok(Self::StructuralType(::std::boxed::Box::new(
                <StructuralType as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "tuple_type" => Ok(Self::TupleType(::std::boxed::Box::new(
                <TupleType as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "type_identifier" => Ok(Self::TypeIdentifier(::std::boxed::Box::new(
                <TypeIdentifier as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "wildcard" => Ok(Self::Wildcard(::std::boxed::Box::new(
                <Wildcard as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "with" => Ok(Self::With(::treesitter_types::Span::from(node))),
            "with_template_body" => Ok(Self::WithTemplateBody(::std::boxed::Box::new(
                <WithTemplateBody as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            other => Err(::treesitter_types::ParseError::unexpected_kind(other, node)),
        }
    }
}
impl ::treesitter_types::Spanned for GivenDefinitionReturnType<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        match self {
            Self::Colon(span) => *span,
            Self::AnnotatedType(inner) => inner.span(),
            Self::AppliedConstructorType(inner) => inner.span(),
            Self::Arguments(inner) => inner.span(),
            Self::CompoundType(inner) => inner.span(),
            Self::GenericType(inner) => inner.span(),
            Self::NamedTupleType(inner) => inner.span(),
            Self::ProjectedType(inner) => inner.span(),
            Self::SingletonType(inner) => inner.span(),
            Self::StableTypeIdentifier(inner) => inner.span(),
            Self::StructuralType(inner) => inner.span(),
            Self::TupleType(inner) => inner.span(),
            Self::TypeIdentifier(inner) => inner.span(),
            Self::Wildcard(inner) => inner.span(),
            Self::With(span) => *span,
            Self::WithTemplateBody(inner) => inner.span(),
        }
    }
}
#[derive(Debug, Clone)]
pub enum GivenDefinitionChildren<'tree> {
    Annotation(::std::boxed::Box<Annotation<'tree>>),
    GivenConditional(::std::boxed::Box<GivenConditional<'tree>>),
    Modifiers(::std::boxed::Box<Modifiers<'tree>>),
    TypeParameters(::std::boxed::Box<TypeParameters<'tree>>),
}
impl<'tree> ::treesitter_types::FromNode<'tree> for GivenDefinitionChildren<'tree> {
    #[allow(clippy::collapsible_else_if)]
    fn from_node(
        node: ::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        match node.kind() {
            "annotation" => Ok(Self::Annotation(::std::boxed::Box::new(
                <Annotation as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "given_conditional" => Ok(Self::GivenConditional(::std::boxed::Box::new(
                <GivenConditional as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "modifiers" => Ok(Self::Modifiers(::std::boxed::Box::new(
                <Modifiers as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "type_parameters" => Ok(Self::TypeParameters(::std::boxed::Box::new(
                <TypeParameters as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            other => Err(::treesitter_types::ParseError::unexpected_kind(other, node)),
        }
    }
}
impl ::treesitter_types::Spanned for GivenDefinitionChildren<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        match self {
            Self::Annotation(inner) => inner.span(),
            Self::GivenConditional(inner) => inner.span(),
            Self::Modifiers(inner) => inner.span(),
            Self::TypeParameters(inner) => inner.span(),
        }
    }
}
#[derive(Debug, Clone)]
pub enum GivenPatternType<'tree> {
    AnnotatedType(::std::boxed::Box<AnnotatedType<'tree>>),
    AppliedConstructorType(::std::boxed::Box<AppliedConstructorType<'tree>>),
    CompoundType(::std::boxed::Box<CompoundType<'tree>>),
    FunctionType(::std::boxed::Box<FunctionType<'tree>>),
    GenericType(::std::boxed::Box<GenericType<'tree>>),
    InfixType(::std::boxed::Box<InfixType<'tree>>),
    LiteralType(::std::boxed::Box<LiteralType<'tree>>),
    MatchType(::std::boxed::Box<MatchType<'tree>>),
    NamedTupleType(::std::boxed::Box<NamedTupleType<'tree>>),
    ProjectedType(::std::boxed::Box<ProjectedType<'tree>>),
    SingletonType(::std::boxed::Box<SingletonType<'tree>>),
    StableTypeIdentifier(::std::boxed::Box<StableTypeIdentifier<'tree>>),
    StructuralType(::std::boxed::Box<StructuralType<'tree>>),
    TupleType(::std::boxed::Box<TupleType<'tree>>),
    TypeIdentifier(::std::boxed::Box<TypeIdentifier<'tree>>),
    TypeLambda(::std::boxed::Box<TypeLambda<'tree>>),
    Wildcard(::std::boxed::Box<Wildcard<'tree>>),
}
impl<'tree> ::treesitter_types::FromNode<'tree> for GivenPatternType<'tree> {
    #[allow(clippy::collapsible_else_if)]
    fn from_node(
        node: ::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        match node.kind() {
            "annotated_type" => Ok(Self::AnnotatedType(::std::boxed::Box::new(
                <AnnotatedType as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "applied_constructor_type" => Ok(Self::AppliedConstructorType(::std::boxed::Box::new(
                <AppliedConstructorType as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "compound_type" => Ok(Self::CompoundType(::std::boxed::Box::new(
                <CompoundType as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "function_type" => Ok(Self::FunctionType(::std::boxed::Box::new(
                <FunctionType as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "generic_type" => Ok(Self::GenericType(::std::boxed::Box::new(
                <GenericType as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "infix_type" => Ok(Self::InfixType(::std::boxed::Box::new(
                <InfixType as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "literal_type" => Ok(Self::LiteralType(::std::boxed::Box::new(
                <LiteralType as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "match_type" => Ok(Self::MatchType(::std::boxed::Box::new(
                <MatchType as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "named_tuple_type" => Ok(Self::NamedTupleType(::std::boxed::Box::new(
                <NamedTupleType as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "projected_type" => Ok(Self::ProjectedType(::std::boxed::Box::new(
                <ProjectedType as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "singleton_type" => Ok(Self::SingletonType(::std::boxed::Box::new(
                <SingletonType as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "stable_type_identifier" => Ok(Self::StableTypeIdentifier(::std::boxed::Box::new(
                <StableTypeIdentifier as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "structural_type" => Ok(Self::StructuralType(::std::boxed::Box::new(
                <StructuralType as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "tuple_type" => Ok(Self::TupleType(::std::boxed::Box::new(
                <TupleType as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "type_identifier" => Ok(Self::TypeIdentifier(::std::boxed::Box::new(
                <TypeIdentifier as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "type_lambda" => Ok(Self::TypeLambda(::std::boxed::Box::new(
                <TypeLambda as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "wildcard" => Ok(Self::Wildcard(::std::boxed::Box::new(
                <Wildcard as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            other => Err(::treesitter_types::ParseError::unexpected_kind(other, node)),
        }
    }
}
impl ::treesitter_types::Spanned for GivenPatternType<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        match self {
            Self::AnnotatedType(inner) => inner.span(),
            Self::AppliedConstructorType(inner) => inner.span(),
            Self::CompoundType(inner) => inner.span(),
            Self::FunctionType(inner) => inner.span(),
            Self::GenericType(inner) => inner.span(),
            Self::InfixType(inner) => inner.span(),
            Self::LiteralType(inner) => inner.span(),
            Self::MatchType(inner) => inner.span(),
            Self::NamedTupleType(inner) => inner.span(),
            Self::ProjectedType(inner) => inner.span(),
            Self::SingletonType(inner) => inner.span(),
            Self::StableTypeIdentifier(inner) => inner.span(),
            Self::StructuralType(inner) => inner.span(),
            Self::TupleType(inner) => inner.span(),
            Self::TypeIdentifier(inner) => inner.span(),
            Self::TypeLambda(inner) => inner.span(),
            Self::Wildcard(inner) => inner.span(),
        }
    }
}
#[derive(Debug, Clone)]
pub enum GuardCondition<'tree> {
    Block(::std::boxed::Box<Block<'tree>>),
    BooleanLiteral(::std::boxed::Box<BooleanLiteral<'tree>>),
    CallExpression(::std::boxed::Box<CallExpression<'tree>>),
    CaseBlock(::std::boxed::Box<CaseBlock<'tree>>),
    CharacterLiteral(::std::boxed::Box<CharacterLiteral<'tree>>),
    FieldExpression(::std::boxed::Box<FieldExpression<'tree>>),
    FloatingPointLiteral(::std::boxed::Box<FloatingPointLiteral<'tree>>),
    GenericFunction(::std::boxed::Box<GenericFunction<'tree>>),
    Identifier(::std::boxed::Box<Identifier<'tree>>),
    InfixExpression(::std::boxed::Box<InfixExpression<'tree>>),
    InstanceExpression(::std::boxed::Box<InstanceExpression<'tree>>),
    IntegerLiteral(::std::boxed::Box<IntegerLiteral<'tree>>),
    InterpolatedStringExpression(::std::boxed::Box<InterpolatedStringExpression<'tree>>),
    NullLiteral(::std::boxed::Box<NullLiteral<'tree>>),
    OperatorIdentifier(::std::boxed::Box<OperatorIdentifier<'tree>>),
    ParenthesizedExpression(::std::boxed::Box<ParenthesizedExpression<'tree>>),
    PostfixExpression(::std::boxed::Box<PostfixExpression<'tree>>),
    PrefixExpression(::std::boxed::Box<PrefixExpression<'tree>>),
    QuoteExpression(::std::boxed::Box<QuoteExpression<'tree>>),
    SpliceExpression(::std::boxed::Box<SpliceExpression<'tree>>),
    String(::std::boxed::Box<String<'tree>>),
    TupleExpression(::std::boxed::Box<TupleExpression<'tree>>),
    Unit(::std::boxed::Box<Unit<'tree>>),
    Wildcard(::std::boxed::Box<Wildcard<'tree>>),
}
impl<'tree> ::treesitter_types::FromNode<'tree> for GuardCondition<'tree> {
    #[allow(clippy::collapsible_else_if)]
    fn from_node(
        node: ::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        match node.kind() {
            "block" => Ok(Self::Block(::std::boxed::Box::new(
                <Block as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "boolean_literal" => Ok(Self::BooleanLiteral(::std::boxed::Box::new(
                <BooleanLiteral as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "call_expression" => Ok(Self::CallExpression(::std::boxed::Box::new(
                <CallExpression as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "case_block" => Ok(Self::CaseBlock(::std::boxed::Box::new(
                <CaseBlock as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "character_literal" => Ok(Self::CharacterLiteral(::std::boxed::Box::new(
                <CharacterLiteral as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "field_expression" => Ok(Self::FieldExpression(::std::boxed::Box::new(
                <FieldExpression as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "floating_point_literal" => Ok(Self::FloatingPointLiteral(::std::boxed::Box::new(
                <FloatingPointLiteral as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "generic_function" => Ok(Self::GenericFunction(::std::boxed::Box::new(
                <GenericFunction as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "identifier" => Ok(Self::Identifier(::std::boxed::Box::new(
                <Identifier as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "infix_expression" => Ok(Self::InfixExpression(::std::boxed::Box::new(
                <InfixExpression as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "instance_expression" => Ok(Self::InstanceExpression(::std::boxed::Box::new(
                <InstanceExpression as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "integer_literal" => Ok(Self::IntegerLiteral(::std::boxed::Box::new(
                <IntegerLiteral as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "interpolated_string_expression" => {
                Ok(Self::InterpolatedStringExpression(::std::boxed::Box::new(
                    <InterpolatedStringExpression as ::treesitter_types::FromNode>::from_node(
                        node, src,
                    )?,
                )))
            }
            "null_literal" => Ok(Self::NullLiteral(::std::boxed::Box::new(
                <NullLiteral as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "operator_identifier" => Ok(Self::OperatorIdentifier(::std::boxed::Box::new(
                <OperatorIdentifier as ::treesitter_types::FromNode>::from_node(node, src)?,
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
            "prefix_expression" => Ok(Self::PrefixExpression(::std::boxed::Box::new(
                <PrefixExpression as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "quote_expression" => Ok(Self::QuoteExpression(::std::boxed::Box::new(
                <QuoteExpression as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "splice_expression" => Ok(Self::SpliceExpression(::std::boxed::Box::new(
                <SpliceExpression as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "string" => Ok(Self::String(::std::boxed::Box::new(
                <String as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "tuple_expression" => Ok(Self::TupleExpression(::std::boxed::Box::new(
                <TupleExpression as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "unit" => Ok(Self::Unit(::std::boxed::Box::new(
                <Unit as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "wildcard" => Ok(Self::Wildcard(::std::boxed::Box::new(
                <Wildcard as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            other => Err(::treesitter_types::ParseError::unexpected_kind(other, node)),
        }
    }
}
impl ::treesitter_types::Spanned for GuardCondition<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        match self {
            Self::Block(inner) => inner.span(),
            Self::BooleanLiteral(inner) => inner.span(),
            Self::CallExpression(inner) => inner.span(),
            Self::CaseBlock(inner) => inner.span(),
            Self::CharacterLiteral(inner) => inner.span(),
            Self::FieldExpression(inner) => inner.span(),
            Self::FloatingPointLiteral(inner) => inner.span(),
            Self::GenericFunction(inner) => inner.span(),
            Self::Identifier(inner) => inner.span(),
            Self::InfixExpression(inner) => inner.span(),
            Self::InstanceExpression(inner) => inner.span(),
            Self::IntegerLiteral(inner) => inner.span(),
            Self::InterpolatedStringExpression(inner) => inner.span(),
            Self::NullLiteral(inner) => inner.span(),
            Self::OperatorIdentifier(inner) => inner.span(),
            Self::ParenthesizedExpression(inner) => inner.span(),
            Self::PostfixExpression(inner) => inner.span(),
            Self::PrefixExpression(inner) => inner.span(),
            Self::QuoteExpression(inner) => inner.span(),
            Self::SpliceExpression(inner) => inner.span(),
            Self::String(inner) => inner.span(),
            Self::TupleExpression(inner) => inner.span(),
            Self::Unit(inner) => inner.span(),
            Self::Wildcard(inner) => inner.span(),
        }
    }
}
#[derive(Debug, Clone)]
pub enum IfExpressionAlternative<'tree> {
    Expression(::std::boxed::Box<Expression<'tree>>),
    IndentedBlock(::std::boxed::Box<IndentedBlock<'tree>>),
    IndentedCases(::std::boxed::Box<IndentedCases<'tree>>),
}
impl<'tree> ::treesitter_types::FromNode<'tree> for IfExpressionAlternative<'tree> {
    #[allow(clippy::collapsible_else_if)]
    fn from_node(
        node: ::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        match node.kind() {
            "indented_block" => Ok(Self::IndentedBlock(::std::boxed::Box::new(
                <IndentedBlock as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "indented_cases" => Ok(Self::IndentedCases(::std::boxed::Box::new(
                <IndentedCases as ::treesitter_types::FromNode>::from_node(node, src)?,
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
impl ::treesitter_types::Spanned for IfExpressionAlternative<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        match self {
            Self::Expression(inner) => inner.span(),
            Self::IndentedBlock(inner) => inner.span(),
            Self::IndentedCases(inner) => inner.span(),
        }
    }
}
#[derive(Debug, Clone)]
pub enum IfExpressionCondition<'tree> {
    Expression(::std::boxed::Box<Expression<'tree>>),
    IndentedBlock(::std::boxed::Box<IndentedBlock<'tree>>),
    IndentedCases(::std::boxed::Box<IndentedCases<'tree>>),
    Then(::treesitter_types::Span),
}
impl<'tree> ::treesitter_types::FromNode<'tree> for IfExpressionCondition<'tree> {
    #[allow(clippy::collapsible_else_if)]
    fn from_node(
        node: ::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        match node.kind() {
            "indented_block" => Ok(Self::IndentedBlock(::std::boxed::Box::new(
                <IndentedBlock as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "indented_cases" => Ok(Self::IndentedCases(::std::boxed::Box::new(
                <IndentedCases as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "then" => Ok(Self::Then(::treesitter_types::Span::from(node))),
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
impl ::treesitter_types::Spanned for IfExpressionCondition<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        match self {
            Self::Expression(inner) => inner.span(),
            Self::IndentedBlock(inner) => inner.span(),
            Self::IndentedCases(inner) => inner.span(),
            Self::Then(span) => *span,
        }
    }
}
#[derive(Debug, Clone)]
pub enum IfExpressionConsequence<'tree> {
    Expression(::std::boxed::Box<Expression<'tree>>),
    IndentedBlock(::std::boxed::Box<IndentedBlock<'tree>>),
    IndentedCases(::std::boxed::Box<IndentedCases<'tree>>),
}
impl<'tree> ::treesitter_types::FromNode<'tree> for IfExpressionConsequence<'tree> {
    #[allow(clippy::collapsible_else_if)]
    fn from_node(
        node: ::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        match node.kind() {
            "indented_block" => Ok(Self::IndentedBlock(::std::boxed::Box::new(
                <IndentedBlock as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "indented_cases" => Ok(Self::IndentedCases(::std::boxed::Box::new(
                <IndentedCases as ::treesitter_types::FromNode>::from_node(node, src)?,
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
impl ::treesitter_types::Spanned for IfExpressionConsequence<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        match self {
            Self::Expression(inner) => inner.span(),
            Self::IndentedBlock(inner) => inner.span(),
            Self::IndentedCases(inner) => inner.span(),
        }
    }
}
#[derive(Debug, Clone)]
pub enum ImportDeclarationPath<'tree> {
    Dot(::treesitter_types::Span),
    Identifier(::std::boxed::Box<Identifier<'tree>>),
    OperatorIdentifier(::std::boxed::Box<OperatorIdentifier<'tree>>),
}
impl<'tree> ::treesitter_types::FromNode<'tree> for ImportDeclarationPath<'tree> {
    #[allow(clippy::collapsible_else_if)]
    fn from_node(
        node: ::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        match node.kind() {
            "." => Ok(Self::Dot(::treesitter_types::Span::from(node))),
            "identifier" => Ok(Self::Identifier(::std::boxed::Box::new(
                <Identifier as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "operator_identifier" => Ok(Self::OperatorIdentifier(::std::boxed::Box::new(
                <OperatorIdentifier as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            other => Err(::treesitter_types::ParseError::unexpected_kind(other, node)),
        }
    }
}
impl ::treesitter_types::Spanned for ImportDeclarationPath<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        match self {
            Self::Dot(span) => *span,
            Self::Identifier(inner) => inner.span(),
            Self::OperatorIdentifier(inner) => inner.span(),
        }
    }
}
#[derive(Debug, Clone)]
pub enum ImportDeclarationChildren<'tree> {
    AsRenamedIdentifier(::std::boxed::Box<AsRenamedIdentifier<'tree>>),
    NamespaceSelectors(::std::boxed::Box<NamespaceSelectors<'tree>>),
    NamespaceWildcard(::std::boxed::Box<NamespaceWildcard<'tree>>),
}
impl<'tree> ::treesitter_types::FromNode<'tree> for ImportDeclarationChildren<'tree> {
    #[allow(clippy::collapsible_else_if)]
    fn from_node(
        node: ::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        match node.kind() {
            "as_renamed_identifier" => Ok(Self::AsRenamedIdentifier(::std::boxed::Box::new(
                <AsRenamedIdentifier as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "namespace_selectors" => Ok(Self::NamespaceSelectors(::std::boxed::Box::new(
                <NamespaceSelectors as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "namespace_wildcard" => Ok(Self::NamespaceWildcard(::std::boxed::Box::new(
                <NamespaceWildcard as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            other => Err(::treesitter_types::ParseError::unexpected_kind(other, node)),
        }
    }
}
impl ::treesitter_types::Spanned for ImportDeclarationChildren<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        match self {
            Self::AsRenamedIdentifier(inner) => inner.span(),
            Self::NamespaceSelectors(inner) => inner.span(),
            Self::NamespaceWildcard(inner) => inner.span(),
        }
    }
}
#[derive(Debug, Clone)]
pub enum IndentedBlockChildren<'tree> {
    Definition(::std::boxed::Box<Definition<'tree>>),
    Expression(::std::boxed::Box<Expression<'tree>>),
}
impl<'tree> ::treesitter_types::FromNode<'tree> for IndentedBlockChildren<'tree> {
    #[allow(clippy::collapsible_else_if)]
    fn from_node(
        node: ::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        if let Ok(v) = <Definition as ::treesitter_types::FromNode>::from_node(node, src) {
            Ok(Self::Definition(::std::boxed::Box::new(v)))
        } else {
            if let Ok(v) = <Expression as ::treesitter_types::FromNode>::from_node(node, src) {
                Ok(Self::Expression(::std::boxed::Box::new(v)))
            } else {
                Err(::treesitter_types::ParseError::unexpected_kind(
                    node.kind(),
                    node,
                ))
            }
        }
    }
}
impl ::treesitter_types::Spanned for IndentedBlockChildren<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        match self {
            Self::Definition(inner) => inner.span(),
            Self::Expression(inner) => inner.span(),
        }
    }
}
#[derive(Debug, Clone)]
pub enum InfixExpressionLeft<'tree> {
    Block(::std::boxed::Box<Block<'tree>>),
    BooleanLiteral(::std::boxed::Box<BooleanLiteral<'tree>>),
    CallExpression(::std::boxed::Box<CallExpression<'tree>>),
    CaseBlock(::std::boxed::Box<CaseBlock<'tree>>),
    CharacterLiteral(::std::boxed::Box<CharacterLiteral<'tree>>),
    FieldExpression(::std::boxed::Box<FieldExpression<'tree>>),
    FloatingPointLiteral(::std::boxed::Box<FloatingPointLiteral<'tree>>),
    GenericFunction(::std::boxed::Box<GenericFunction<'tree>>),
    Identifier(::std::boxed::Box<Identifier<'tree>>),
    InfixExpression(::std::boxed::Box<InfixExpression<'tree>>),
    InstanceExpression(::std::boxed::Box<InstanceExpression<'tree>>),
    IntegerLiteral(::std::boxed::Box<IntegerLiteral<'tree>>),
    InterpolatedStringExpression(::std::boxed::Box<InterpolatedStringExpression<'tree>>),
    NullLiteral(::std::boxed::Box<NullLiteral<'tree>>),
    OperatorIdentifier(::std::boxed::Box<OperatorIdentifier<'tree>>),
    ParenthesizedExpression(::std::boxed::Box<ParenthesizedExpression<'tree>>),
    PrefixExpression(::std::boxed::Box<PrefixExpression<'tree>>),
    QuoteExpression(::std::boxed::Box<QuoteExpression<'tree>>),
    SpliceExpression(::std::boxed::Box<SpliceExpression<'tree>>),
    String(::std::boxed::Box<String<'tree>>),
    TupleExpression(::std::boxed::Box<TupleExpression<'tree>>),
    Unit(::std::boxed::Box<Unit<'tree>>),
    Wildcard(::std::boxed::Box<Wildcard<'tree>>),
}
impl<'tree> ::treesitter_types::FromNode<'tree> for InfixExpressionLeft<'tree> {
    #[allow(clippy::collapsible_else_if)]
    fn from_node(
        node: ::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        match node.kind() {
            "block" => Ok(Self::Block(::std::boxed::Box::new(
                <Block as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "boolean_literal" => Ok(Self::BooleanLiteral(::std::boxed::Box::new(
                <BooleanLiteral as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "call_expression" => Ok(Self::CallExpression(::std::boxed::Box::new(
                <CallExpression as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "case_block" => Ok(Self::CaseBlock(::std::boxed::Box::new(
                <CaseBlock as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "character_literal" => Ok(Self::CharacterLiteral(::std::boxed::Box::new(
                <CharacterLiteral as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "field_expression" => Ok(Self::FieldExpression(::std::boxed::Box::new(
                <FieldExpression as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "floating_point_literal" => Ok(Self::FloatingPointLiteral(::std::boxed::Box::new(
                <FloatingPointLiteral as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "generic_function" => Ok(Self::GenericFunction(::std::boxed::Box::new(
                <GenericFunction as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "identifier" => Ok(Self::Identifier(::std::boxed::Box::new(
                <Identifier as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "infix_expression" => Ok(Self::InfixExpression(::std::boxed::Box::new(
                <InfixExpression as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "instance_expression" => Ok(Self::InstanceExpression(::std::boxed::Box::new(
                <InstanceExpression as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "integer_literal" => Ok(Self::IntegerLiteral(::std::boxed::Box::new(
                <IntegerLiteral as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "interpolated_string_expression" => {
                Ok(Self::InterpolatedStringExpression(::std::boxed::Box::new(
                    <InterpolatedStringExpression as ::treesitter_types::FromNode>::from_node(
                        node, src,
                    )?,
                )))
            }
            "null_literal" => Ok(Self::NullLiteral(::std::boxed::Box::new(
                <NullLiteral as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "operator_identifier" => Ok(Self::OperatorIdentifier(::std::boxed::Box::new(
                <OperatorIdentifier as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "parenthesized_expression" => {
                Ok(Self::ParenthesizedExpression(::std::boxed::Box::new(
                    <ParenthesizedExpression as ::treesitter_types::FromNode>::from_node(
                        node, src,
                    )?,
                )))
            }
            "prefix_expression" => Ok(Self::PrefixExpression(::std::boxed::Box::new(
                <PrefixExpression as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "quote_expression" => Ok(Self::QuoteExpression(::std::boxed::Box::new(
                <QuoteExpression as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "splice_expression" => Ok(Self::SpliceExpression(::std::boxed::Box::new(
                <SpliceExpression as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "string" => Ok(Self::String(::std::boxed::Box::new(
                <String as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "tuple_expression" => Ok(Self::TupleExpression(::std::boxed::Box::new(
                <TupleExpression as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "unit" => Ok(Self::Unit(::std::boxed::Box::new(
                <Unit as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "wildcard" => Ok(Self::Wildcard(::std::boxed::Box::new(
                <Wildcard as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            other => Err(::treesitter_types::ParseError::unexpected_kind(other, node)),
        }
    }
}
impl ::treesitter_types::Spanned for InfixExpressionLeft<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        match self {
            Self::Block(inner) => inner.span(),
            Self::BooleanLiteral(inner) => inner.span(),
            Self::CallExpression(inner) => inner.span(),
            Self::CaseBlock(inner) => inner.span(),
            Self::CharacterLiteral(inner) => inner.span(),
            Self::FieldExpression(inner) => inner.span(),
            Self::FloatingPointLiteral(inner) => inner.span(),
            Self::GenericFunction(inner) => inner.span(),
            Self::Identifier(inner) => inner.span(),
            Self::InfixExpression(inner) => inner.span(),
            Self::InstanceExpression(inner) => inner.span(),
            Self::IntegerLiteral(inner) => inner.span(),
            Self::InterpolatedStringExpression(inner) => inner.span(),
            Self::NullLiteral(inner) => inner.span(),
            Self::OperatorIdentifier(inner) => inner.span(),
            Self::ParenthesizedExpression(inner) => inner.span(),
            Self::PrefixExpression(inner) => inner.span(),
            Self::QuoteExpression(inner) => inner.span(),
            Self::SpliceExpression(inner) => inner.span(),
            Self::String(inner) => inner.span(),
            Self::TupleExpression(inner) => inner.span(),
            Self::Unit(inner) => inner.span(),
            Self::Wildcard(inner) => inner.span(),
        }
    }
}
#[derive(Debug, Clone)]
pub enum InfixExpressionOperator<'tree> {
    Identifier(::std::boxed::Box<Identifier<'tree>>),
    OperatorIdentifier(::std::boxed::Box<OperatorIdentifier<'tree>>),
}
impl<'tree> ::treesitter_types::FromNode<'tree> for InfixExpressionOperator<'tree> {
    #[allow(clippy::collapsible_else_if)]
    fn from_node(
        node: ::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        match node.kind() {
            "identifier" => Ok(Self::Identifier(::std::boxed::Box::new(
                <Identifier as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "operator_identifier" => Ok(Self::OperatorIdentifier(::std::boxed::Box::new(
                <OperatorIdentifier as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            other => Err(::treesitter_types::ParseError::unexpected_kind(other, node)),
        }
    }
}
impl ::treesitter_types::Spanned for InfixExpressionOperator<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        match self {
            Self::Identifier(inner) => inner.span(),
            Self::OperatorIdentifier(inner) => inner.span(),
        }
    }
}
#[derive(Debug, Clone)]
pub enum InfixExpressionRight<'tree> {
    Colon(::treesitter_types::Span),
    Block(::std::boxed::Box<Block<'tree>>),
    BooleanLiteral(::std::boxed::Box<BooleanLiteral<'tree>>),
    CallExpression(::std::boxed::Box<CallExpression<'tree>>),
    CaseBlock(::std::boxed::Box<CaseBlock<'tree>>),
    CharacterLiteral(::std::boxed::Box<CharacterLiteral<'tree>>),
    ColonArgument(::std::boxed::Box<ColonArgument<'tree>>),
    FieldExpression(::std::boxed::Box<FieldExpression<'tree>>),
    FloatingPointLiteral(::std::boxed::Box<FloatingPointLiteral<'tree>>),
    GenericFunction(::std::boxed::Box<GenericFunction<'tree>>),
    Identifier(::std::boxed::Box<Identifier<'tree>>),
    InstanceExpression(::std::boxed::Box<InstanceExpression<'tree>>),
    IntegerLiteral(::std::boxed::Box<IntegerLiteral<'tree>>),
    InterpolatedStringExpression(::std::boxed::Box<InterpolatedStringExpression<'tree>>),
    NullLiteral(::std::boxed::Box<NullLiteral<'tree>>),
    OperatorIdentifier(::std::boxed::Box<OperatorIdentifier<'tree>>),
    ParenthesizedExpression(::std::boxed::Box<ParenthesizedExpression<'tree>>),
    PrefixExpression(::std::boxed::Box<PrefixExpression<'tree>>),
    QuoteExpression(::std::boxed::Box<QuoteExpression<'tree>>),
    SpliceExpression(::std::boxed::Box<SpliceExpression<'tree>>),
    String(::std::boxed::Box<String<'tree>>),
    TupleExpression(::std::boxed::Box<TupleExpression<'tree>>),
    Unit(::std::boxed::Box<Unit<'tree>>),
    Wildcard(::std::boxed::Box<Wildcard<'tree>>),
}
impl<'tree> ::treesitter_types::FromNode<'tree> for InfixExpressionRight<'tree> {
    #[allow(clippy::collapsible_else_if)]
    fn from_node(
        node: ::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        match node.kind() {
            ":" => Ok(Self::Colon(::treesitter_types::Span::from(node))),
            "block" => Ok(Self::Block(::std::boxed::Box::new(
                <Block as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "boolean_literal" => Ok(Self::BooleanLiteral(::std::boxed::Box::new(
                <BooleanLiteral as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "call_expression" => Ok(Self::CallExpression(::std::boxed::Box::new(
                <CallExpression as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "case_block" => Ok(Self::CaseBlock(::std::boxed::Box::new(
                <CaseBlock as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "character_literal" => Ok(Self::CharacterLiteral(::std::boxed::Box::new(
                <CharacterLiteral as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "colon_argument" => Ok(Self::ColonArgument(::std::boxed::Box::new(
                <ColonArgument as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "field_expression" => Ok(Self::FieldExpression(::std::boxed::Box::new(
                <FieldExpression as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "floating_point_literal" => Ok(Self::FloatingPointLiteral(::std::boxed::Box::new(
                <FloatingPointLiteral as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "generic_function" => Ok(Self::GenericFunction(::std::boxed::Box::new(
                <GenericFunction as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "identifier" => Ok(Self::Identifier(::std::boxed::Box::new(
                <Identifier as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "instance_expression" => Ok(Self::InstanceExpression(::std::boxed::Box::new(
                <InstanceExpression as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "integer_literal" => Ok(Self::IntegerLiteral(::std::boxed::Box::new(
                <IntegerLiteral as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "interpolated_string_expression" => {
                Ok(Self::InterpolatedStringExpression(::std::boxed::Box::new(
                    <InterpolatedStringExpression as ::treesitter_types::FromNode>::from_node(
                        node, src,
                    )?,
                )))
            }
            "null_literal" => Ok(Self::NullLiteral(::std::boxed::Box::new(
                <NullLiteral as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "operator_identifier" => Ok(Self::OperatorIdentifier(::std::boxed::Box::new(
                <OperatorIdentifier as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "parenthesized_expression" => {
                Ok(Self::ParenthesizedExpression(::std::boxed::Box::new(
                    <ParenthesizedExpression as ::treesitter_types::FromNode>::from_node(
                        node, src,
                    )?,
                )))
            }
            "prefix_expression" => Ok(Self::PrefixExpression(::std::boxed::Box::new(
                <PrefixExpression as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "quote_expression" => Ok(Self::QuoteExpression(::std::boxed::Box::new(
                <QuoteExpression as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "splice_expression" => Ok(Self::SpliceExpression(::std::boxed::Box::new(
                <SpliceExpression as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "string" => Ok(Self::String(::std::boxed::Box::new(
                <String as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "tuple_expression" => Ok(Self::TupleExpression(::std::boxed::Box::new(
                <TupleExpression as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "unit" => Ok(Self::Unit(::std::boxed::Box::new(
                <Unit as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "wildcard" => Ok(Self::Wildcard(::std::boxed::Box::new(
                <Wildcard as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            other => Err(::treesitter_types::ParseError::unexpected_kind(other, node)),
        }
    }
}
impl ::treesitter_types::Spanned for InfixExpressionRight<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        match self {
            Self::Colon(span) => *span,
            Self::Block(inner) => inner.span(),
            Self::BooleanLiteral(inner) => inner.span(),
            Self::CallExpression(inner) => inner.span(),
            Self::CaseBlock(inner) => inner.span(),
            Self::CharacterLiteral(inner) => inner.span(),
            Self::ColonArgument(inner) => inner.span(),
            Self::FieldExpression(inner) => inner.span(),
            Self::FloatingPointLiteral(inner) => inner.span(),
            Self::GenericFunction(inner) => inner.span(),
            Self::Identifier(inner) => inner.span(),
            Self::InstanceExpression(inner) => inner.span(),
            Self::IntegerLiteral(inner) => inner.span(),
            Self::InterpolatedStringExpression(inner) => inner.span(),
            Self::NullLiteral(inner) => inner.span(),
            Self::OperatorIdentifier(inner) => inner.span(),
            Self::ParenthesizedExpression(inner) => inner.span(),
            Self::PrefixExpression(inner) => inner.span(),
            Self::QuoteExpression(inner) => inner.span(),
            Self::SpliceExpression(inner) => inner.span(),
            Self::String(inner) => inner.span(),
            Self::TupleExpression(inner) => inner.span(),
            Self::Unit(inner) => inner.span(),
            Self::Wildcard(inner) => inner.span(),
        }
    }
}
#[derive(Debug, Clone)]
pub enum InfixPatternOperator<'tree> {
    Identifier(::std::boxed::Box<Identifier<'tree>>),
    OperatorIdentifier(::std::boxed::Box<OperatorIdentifier<'tree>>),
}
impl<'tree> ::treesitter_types::FromNode<'tree> for InfixPatternOperator<'tree> {
    #[allow(clippy::collapsible_else_if)]
    fn from_node(
        node: ::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        match node.kind() {
            "identifier" => Ok(Self::Identifier(::std::boxed::Box::new(
                <Identifier as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "operator_identifier" => Ok(Self::OperatorIdentifier(::std::boxed::Box::new(
                <OperatorIdentifier as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            other => Err(::treesitter_types::ParseError::unexpected_kind(other, node)),
        }
    }
}
impl ::treesitter_types::Spanned for InfixPatternOperator<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        match self {
            Self::Identifier(inner) => inner.span(),
            Self::OperatorIdentifier(inner) => inner.span(),
        }
    }
}
#[derive(Debug, Clone)]
pub enum InfixTypeLeft<'tree> {
    AnnotatedType(::std::boxed::Box<AnnotatedType<'tree>>),
    AppliedConstructorType(::std::boxed::Box<AppliedConstructorType<'tree>>),
    CompoundType(::std::boxed::Box<CompoundType<'tree>>),
    GenericType(::std::boxed::Box<GenericType<'tree>>),
    InfixType(::std::boxed::Box<InfixType<'tree>>),
    LiteralType(::std::boxed::Box<LiteralType<'tree>>),
    NamedTupleType(::std::boxed::Box<NamedTupleType<'tree>>),
    ProjectedType(::std::boxed::Box<ProjectedType<'tree>>),
    SingletonType(::std::boxed::Box<SingletonType<'tree>>),
    StableTypeIdentifier(::std::boxed::Box<StableTypeIdentifier<'tree>>),
    TupleType(::std::boxed::Box<TupleType<'tree>>),
    TypeIdentifier(::std::boxed::Box<TypeIdentifier<'tree>>),
    Wildcard(::std::boxed::Box<Wildcard<'tree>>),
}
impl<'tree> ::treesitter_types::FromNode<'tree> for InfixTypeLeft<'tree> {
    #[allow(clippy::collapsible_else_if)]
    fn from_node(
        node: ::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        match node.kind() {
            "annotated_type" => Ok(Self::AnnotatedType(::std::boxed::Box::new(
                <AnnotatedType as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "applied_constructor_type" => Ok(Self::AppliedConstructorType(::std::boxed::Box::new(
                <AppliedConstructorType as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "compound_type" => Ok(Self::CompoundType(::std::boxed::Box::new(
                <CompoundType as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "generic_type" => Ok(Self::GenericType(::std::boxed::Box::new(
                <GenericType as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "infix_type" => Ok(Self::InfixType(::std::boxed::Box::new(
                <InfixType as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "literal_type" => Ok(Self::LiteralType(::std::boxed::Box::new(
                <LiteralType as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "named_tuple_type" => Ok(Self::NamedTupleType(::std::boxed::Box::new(
                <NamedTupleType as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "projected_type" => Ok(Self::ProjectedType(::std::boxed::Box::new(
                <ProjectedType as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "singleton_type" => Ok(Self::SingletonType(::std::boxed::Box::new(
                <SingletonType as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "stable_type_identifier" => Ok(Self::StableTypeIdentifier(::std::boxed::Box::new(
                <StableTypeIdentifier as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "tuple_type" => Ok(Self::TupleType(::std::boxed::Box::new(
                <TupleType as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "type_identifier" => Ok(Self::TypeIdentifier(::std::boxed::Box::new(
                <TypeIdentifier as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "wildcard" => Ok(Self::Wildcard(::std::boxed::Box::new(
                <Wildcard as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            other => Err(::treesitter_types::ParseError::unexpected_kind(other, node)),
        }
    }
}
impl ::treesitter_types::Spanned for InfixTypeLeft<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        match self {
            Self::AnnotatedType(inner) => inner.span(),
            Self::AppliedConstructorType(inner) => inner.span(),
            Self::CompoundType(inner) => inner.span(),
            Self::GenericType(inner) => inner.span(),
            Self::InfixType(inner) => inner.span(),
            Self::LiteralType(inner) => inner.span(),
            Self::NamedTupleType(inner) => inner.span(),
            Self::ProjectedType(inner) => inner.span(),
            Self::SingletonType(inner) => inner.span(),
            Self::StableTypeIdentifier(inner) => inner.span(),
            Self::TupleType(inner) => inner.span(),
            Self::TypeIdentifier(inner) => inner.span(),
            Self::Wildcard(inner) => inner.span(),
        }
    }
}
#[derive(Debug, Clone)]
pub enum InfixTypeOperator<'tree> {
    Identifier(::std::boxed::Box<Identifier<'tree>>),
    OperatorIdentifier(::std::boxed::Box<OperatorIdentifier<'tree>>),
}
impl<'tree> ::treesitter_types::FromNode<'tree> for InfixTypeOperator<'tree> {
    #[allow(clippy::collapsible_else_if)]
    fn from_node(
        node: ::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        match node.kind() {
            "identifier" => Ok(Self::Identifier(::std::boxed::Box::new(
                <Identifier as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "operator_identifier" => Ok(Self::OperatorIdentifier(::std::boxed::Box::new(
                <OperatorIdentifier as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            other => Err(::treesitter_types::ParseError::unexpected_kind(other, node)),
        }
    }
}
impl ::treesitter_types::Spanned for InfixTypeOperator<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        match self {
            Self::Identifier(inner) => inner.span(),
            Self::OperatorIdentifier(inner) => inner.span(),
        }
    }
}
#[derive(Debug, Clone)]
pub enum InfixTypeRight<'tree> {
    AnnotatedType(::std::boxed::Box<AnnotatedType<'tree>>),
    AppliedConstructorType(::std::boxed::Box<AppliedConstructorType<'tree>>),
    CompoundType(::std::boxed::Box<CompoundType<'tree>>),
    GenericType(::std::boxed::Box<GenericType<'tree>>),
    InfixType(::std::boxed::Box<InfixType<'tree>>),
    LiteralType(::std::boxed::Box<LiteralType<'tree>>),
    NamedTupleType(::std::boxed::Box<NamedTupleType<'tree>>),
    ProjectedType(::std::boxed::Box<ProjectedType<'tree>>),
    SingletonType(::std::boxed::Box<SingletonType<'tree>>),
    StableTypeIdentifier(::std::boxed::Box<StableTypeIdentifier<'tree>>),
    TupleType(::std::boxed::Box<TupleType<'tree>>),
    TypeIdentifier(::std::boxed::Box<TypeIdentifier<'tree>>),
    Wildcard(::std::boxed::Box<Wildcard<'tree>>),
}
impl<'tree> ::treesitter_types::FromNode<'tree> for InfixTypeRight<'tree> {
    #[allow(clippy::collapsible_else_if)]
    fn from_node(
        node: ::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        match node.kind() {
            "annotated_type" => Ok(Self::AnnotatedType(::std::boxed::Box::new(
                <AnnotatedType as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "applied_constructor_type" => Ok(Self::AppliedConstructorType(::std::boxed::Box::new(
                <AppliedConstructorType as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "compound_type" => Ok(Self::CompoundType(::std::boxed::Box::new(
                <CompoundType as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "generic_type" => Ok(Self::GenericType(::std::boxed::Box::new(
                <GenericType as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "infix_type" => Ok(Self::InfixType(::std::boxed::Box::new(
                <InfixType as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "literal_type" => Ok(Self::LiteralType(::std::boxed::Box::new(
                <LiteralType as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "named_tuple_type" => Ok(Self::NamedTupleType(::std::boxed::Box::new(
                <NamedTupleType as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "projected_type" => Ok(Self::ProjectedType(::std::boxed::Box::new(
                <ProjectedType as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "singleton_type" => Ok(Self::SingletonType(::std::boxed::Box::new(
                <SingletonType as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "stable_type_identifier" => Ok(Self::StableTypeIdentifier(::std::boxed::Box::new(
                <StableTypeIdentifier as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "tuple_type" => Ok(Self::TupleType(::std::boxed::Box::new(
                <TupleType as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "type_identifier" => Ok(Self::TypeIdentifier(::std::boxed::Box::new(
                <TypeIdentifier as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "wildcard" => Ok(Self::Wildcard(::std::boxed::Box::new(
                <Wildcard as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            other => Err(::treesitter_types::ParseError::unexpected_kind(other, node)),
        }
    }
}
impl ::treesitter_types::Spanned for InfixTypeRight<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        match self {
            Self::AnnotatedType(inner) => inner.span(),
            Self::AppliedConstructorType(inner) => inner.span(),
            Self::CompoundType(inner) => inner.span(),
            Self::GenericType(inner) => inner.span(),
            Self::InfixType(inner) => inner.span(),
            Self::LiteralType(inner) => inner.span(),
            Self::NamedTupleType(inner) => inner.span(),
            Self::ProjectedType(inner) => inner.span(),
            Self::SingletonType(inner) => inner.span(),
            Self::StableTypeIdentifier(inner) => inner.span(),
            Self::TupleType(inner) => inner.span(),
            Self::TypeIdentifier(inner) => inner.span(),
            Self::Wildcard(inner) => inner.span(),
        }
    }
}
#[derive(Debug, Clone)]
pub enum InstanceExpressionChildren<'tree> {
    AnnotatedType(::std::boxed::Box<AnnotatedType<'tree>>),
    AppliedConstructorType(::std::boxed::Box<AppliedConstructorType<'tree>>),
    CompoundType(::std::boxed::Box<CompoundType<'tree>>),
    GenericType(::std::boxed::Box<GenericType<'tree>>),
    NamedTupleType(::std::boxed::Box<NamedTupleType<'tree>>),
    ProjectedType(::std::boxed::Box<ProjectedType<'tree>>),
    SingletonType(::std::boxed::Box<SingletonType<'tree>>),
    StableTypeIdentifier(::std::boxed::Box<StableTypeIdentifier<'tree>>),
    StructuralType(::std::boxed::Box<StructuralType<'tree>>),
    TemplateBody(::std::boxed::Box<TemplateBody<'tree>>),
    TupleType(::std::boxed::Box<TupleType<'tree>>),
    TypeIdentifier(::std::boxed::Box<TypeIdentifier<'tree>>),
    Wildcard(::std::boxed::Box<Wildcard<'tree>>),
}
impl<'tree> ::treesitter_types::FromNode<'tree> for InstanceExpressionChildren<'tree> {
    #[allow(clippy::collapsible_else_if)]
    fn from_node(
        node: ::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        match node.kind() {
            "annotated_type" => Ok(Self::AnnotatedType(::std::boxed::Box::new(
                <AnnotatedType as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "applied_constructor_type" => Ok(Self::AppliedConstructorType(::std::boxed::Box::new(
                <AppliedConstructorType as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "compound_type" => Ok(Self::CompoundType(::std::boxed::Box::new(
                <CompoundType as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "generic_type" => Ok(Self::GenericType(::std::boxed::Box::new(
                <GenericType as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "named_tuple_type" => Ok(Self::NamedTupleType(::std::boxed::Box::new(
                <NamedTupleType as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "projected_type" => Ok(Self::ProjectedType(::std::boxed::Box::new(
                <ProjectedType as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "singleton_type" => Ok(Self::SingletonType(::std::boxed::Box::new(
                <SingletonType as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "stable_type_identifier" => Ok(Self::StableTypeIdentifier(::std::boxed::Box::new(
                <StableTypeIdentifier as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "structural_type" => Ok(Self::StructuralType(::std::boxed::Box::new(
                <StructuralType as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "template_body" => Ok(Self::TemplateBody(::std::boxed::Box::new(
                <TemplateBody as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "tuple_type" => Ok(Self::TupleType(::std::boxed::Box::new(
                <TupleType as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "type_identifier" => Ok(Self::TypeIdentifier(::std::boxed::Box::new(
                <TypeIdentifier as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "wildcard" => Ok(Self::Wildcard(::std::boxed::Box::new(
                <Wildcard as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            other => Err(::treesitter_types::ParseError::unexpected_kind(other, node)),
        }
    }
}
impl ::treesitter_types::Spanned for InstanceExpressionChildren<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        match self {
            Self::AnnotatedType(inner) => inner.span(),
            Self::AppliedConstructorType(inner) => inner.span(),
            Self::CompoundType(inner) => inner.span(),
            Self::GenericType(inner) => inner.span(),
            Self::NamedTupleType(inner) => inner.span(),
            Self::ProjectedType(inner) => inner.span(),
            Self::SingletonType(inner) => inner.span(),
            Self::StableTypeIdentifier(inner) => inner.span(),
            Self::StructuralType(inner) => inner.span(),
            Self::TemplateBody(inner) => inner.span(),
            Self::TupleType(inner) => inner.span(),
            Self::TypeIdentifier(inner) => inner.span(),
            Self::Wildcard(inner) => inner.span(),
        }
    }
}
#[derive(Debug, Clone)]
pub enum InterpolatedStringChildren<'tree> {
    EscapeSequence(::std::boxed::Box<EscapeSequence<'tree>>),
    Interpolation(::std::boxed::Box<Interpolation<'tree>>),
}
impl<'tree> ::treesitter_types::FromNode<'tree> for InterpolatedStringChildren<'tree> {
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
            other => Err(::treesitter_types::ParseError::unexpected_kind(other, node)),
        }
    }
}
impl ::treesitter_types::Spanned for InterpolatedStringChildren<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        match self {
            Self::EscapeSequence(inner) => inner.span(),
            Self::Interpolation(inner) => inner.span(),
        }
    }
}
#[derive(Debug, Clone)]
pub enum InterpolationChildren<'tree> {
    Block(::std::boxed::Box<Block<'tree>>),
    Identifier(::std::boxed::Box<Identifier<'tree>>),
}
impl<'tree> ::treesitter_types::FromNode<'tree> for InterpolationChildren<'tree> {
    #[allow(clippy::collapsible_else_if)]
    fn from_node(
        node: ::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        match node.kind() {
            "block" => Ok(Self::Block(::std::boxed::Box::new(
                <Block as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "identifier" => Ok(Self::Identifier(::std::boxed::Box::new(
                <Identifier as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            other => Err(::treesitter_types::ParseError::unexpected_kind(other, node)),
        }
    }
}
impl ::treesitter_types::Spanned for InterpolationChildren<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        match self {
            Self::Block(inner) => inner.span(),
            Self::Identifier(inner) => inner.span(),
        }
    }
}
#[derive(Debug, Clone)]
pub enum LambdaExpressionParameters<'tree> {
    Bindings(::std::boxed::Box<Bindings<'tree>>),
    Identifier(::std::boxed::Box<Identifier<'tree>>),
    Implicit(::treesitter_types::Span),
    OperatorIdentifier(::std::boxed::Box<OperatorIdentifier<'tree>>),
    Wildcard(::std::boxed::Box<Wildcard<'tree>>),
}
impl<'tree> ::treesitter_types::FromNode<'tree> for LambdaExpressionParameters<'tree> {
    #[allow(clippy::collapsible_else_if)]
    fn from_node(
        node: ::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        match node.kind() {
            "bindings" => Ok(Self::Bindings(::std::boxed::Box::new(
                <Bindings as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "identifier" => Ok(Self::Identifier(::std::boxed::Box::new(
                <Identifier as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "implicit" => Ok(Self::Implicit(::treesitter_types::Span::from(node))),
            "operator_identifier" => Ok(Self::OperatorIdentifier(::std::boxed::Box::new(
                <OperatorIdentifier as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "wildcard" => Ok(Self::Wildcard(::std::boxed::Box::new(
                <Wildcard as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            other => Err(::treesitter_types::ParseError::unexpected_kind(other, node)),
        }
    }
}
impl ::treesitter_types::Spanned for LambdaExpressionParameters<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        match self {
            Self::Bindings(inner) => inner.span(),
            Self::Identifier(inner) => inner.span(),
            Self::Implicit(span) => *span,
            Self::OperatorIdentifier(inner) => inner.span(),
            Self::Wildcard(inner) => inner.span(),
        }
    }
}
#[derive(Debug, Clone)]
pub enum LambdaExpressionChildren<'tree> {
    Expression(::std::boxed::Box<Expression<'tree>>),
    IndentedBlock(::std::boxed::Box<IndentedBlock<'tree>>),
    IndentedCases(::std::boxed::Box<IndentedCases<'tree>>),
}
impl<'tree> ::treesitter_types::FromNode<'tree> for LambdaExpressionChildren<'tree> {
    #[allow(clippy::collapsible_else_if)]
    fn from_node(
        node: ::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        match node.kind() {
            "indented_block" => Ok(Self::IndentedBlock(::std::boxed::Box::new(
                <IndentedBlock as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "indented_cases" => Ok(Self::IndentedCases(::std::boxed::Box::new(
                <IndentedCases as ::treesitter_types::FromNode>::from_node(node, src)?,
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
impl ::treesitter_types::Spanned for LambdaExpressionChildren<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        match self {
            Self::Expression(inner) => inner.span(),
            Self::IndentedBlock(inner) => inner.span(),
            Self::IndentedCases(inner) => inner.span(),
        }
    }
}
#[derive(Debug, Clone)]
pub enum LazyParameterTypeType<'tree> {
    AnnotatedType(::std::boxed::Box<AnnotatedType<'tree>>),
    AppliedConstructorType(::std::boxed::Box<AppliedConstructorType<'tree>>),
    CompoundType(::std::boxed::Box<CompoundType<'tree>>),
    FunctionType(::std::boxed::Box<FunctionType<'tree>>),
    GenericType(::std::boxed::Box<GenericType<'tree>>),
    InfixType(::std::boxed::Box<InfixType<'tree>>),
    LiteralType(::std::boxed::Box<LiteralType<'tree>>),
    MatchType(::std::boxed::Box<MatchType<'tree>>),
    NamedTupleType(::std::boxed::Box<NamedTupleType<'tree>>),
    ProjectedType(::std::boxed::Box<ProjectedType<'tree>>),
    SingletonType(::std::boxed::Box<SingletonType<'tree>>),
    StableTypeIdentifier(::std::boxed::Box<StableTypeIdentifier<'tree>>),
    StructuralType(::std::boxed::Box<StructuralType<'tree>>),
    TupleType(::std::boxed::Box<TupleType<'tree>>),
    TypeIdentifier(::std::boxed::Box<TypeIdentifier<'tree>>),
    TypeLambda(::std::boxed::Box<TypeLambda<'tree>>),
    Wildcard(::std::boxed::Box<Wildcard<'tree>>),
}
impl<'tree> ::treesitter_types::FromNode<'tree> for LazyParameterTypeType<'tree> {
    #[allow(clippy::collapsible_else_if)]
    fn from_node(
        node: ::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        match node.kind() {
            "annotated_type" => Ok(Self::AnnotatedType(::std::boxed::Box::new(
                <AnnotatedType as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "applied_constructor_type" => Ok(Self::AppliedConstructorType(::std::boxed::Box::new(
                <AppliedConstructorType as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "compound_type" => Ok(Self::CompoundType(::std::boxed::Box::new(
                <CompoundType as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "function_type" => Ok(Self::FunctionType(::std::boxed::Box::new(
                <FunctionType as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "generic_type" => Ok(Self::GenericType(::std::boxed::Box::new(
                <GenericType as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "infix_type" => Ok(Self::InfixType(::std::boxed::Box::new(
                <InfixType as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "literal_type" => Ok(Self::LiteralType(::std::boxed::Box::new(
                <LiteralType as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "match_type" => Ok(Self::MatchType(::std::boxed::Box::new(
                <MatchType as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "named_tuple_type" => Ok(Self::NamedTupleType(::std::boxed::Box::new(
                <NamedTupleType as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "projected_type" => Ok(Self::ProjectedType(::std::boxed::Box::new(
                <ProjectedType as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "singleton_type" => Ok(Self::SingletonType(::std::boxed::Box::new(
                <SingletonType as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "stable_type_identifier" => Ok(Self::StableTypeIdentifier(::std::boxed::Box::new(
                <StableTypeIdentifier as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "structural_type" => Ok(Self::StructuralType(::std::boxed::Box::new(
                <StructuralType as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "tuple_type" => Ok(Self::TupleType(::std::boxed::Box::new(
                <TupleType as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "type_identifier" => Ok(Self::TypeIdentifier(::std::boxed::Box::new(
                <TypeIdentifier as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "type_lambda" => Ok(Self::TypeLambda(::std::boxed::Box::new(
                <TypeLambda as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "wildcard" => Ok(Self::Wildcard(::std::boxed::Box::new(
                <Wildcard as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            other => Err(::treesitter_types::ParseError::unexpected_kind(other, node)),
        }
    }
}
impl ::treesitter_types::Spanned for LazyParameterTypeType<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        match self {
            Self::AnnotatedType(inner) => inner.span(),
            Self::AppliedConstructorType(inner) => inner.span(),
            Self::CompoundType(inner) => inner.span(),
            Self::FunctionType(inner) => inner.span(),
            Self::GenericType(inner) => inner.span(),
            Self::InfixType(inner) => inner.span(),
            Self::LiteralType(inner) => inner.span(),
            Self::MatchType(inner) => inner.span(),
            Self::NamedTupleType(inner) => inner.span(),
            Self::ProjectedType(inner) => inner.span(),
            Self::SingletonType(inner) => inner.span(),
            Self::StableTypeIdentifier(inner) => inner.span(),
            Self::StructuralType(inner) => inner.span(),
            Self::TupleType(inner) => inner.span(),
            Self::TypeIdentifier(inner) => inner.span(),
            Self::TypeLambda(inner) => inner.span(),
            Self::Wildcard(inner) => inner.span(),
        }
    }
}
#[derive(Debug, Clone)]
pub enum LiteralTypeChildren<'tree> {
    BooleanLiteral(::std::boxed::Box<BooleanLiteral<'tree>>),
    CharacterLiteral(::std::boxed::Box<CharacterLiteral<'tree>>),
    FloatingPointLiteral(::std::boxed::Box<FloatingPointLiteral<'tree>>),
    IntegerLiteral(::std::boxed::Box<IntegerLiteral<'tree>>),
    String(::std::boxed::Box<String<'tree>>),
}
impl<'tree> ::treesitter_types::FromNode<'tree> for LiteralTypeChildren<'tree> {
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
            "floating_point_literal" => Ok(Self::FloatingPointLiteral(::std::boxed::Box::new(
                <FloatingPointLiteral as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "integer_literal" => Ok(Self::IntegerLiteral(::std::boxed::Box::new(
                <IntegerLiteral as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "string" => Ok(Self::String(::std::boxed::Box::new(
                <String as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            other => Err(::treesitter_types::ParseError::unexpected_kind(other, node)),
        }
    }
}
impl ::treesitter_types::Spanned for LiteralTypeChildren<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        match self {
            Self::BooleanLiteral(inner) => inner.span(),
            Self::CharacterLiteral(inner) => inner.span(),
            Self::FloatingPointLiteral(inner) => inner.span(),
            Self::IntegerLiteral(inner) => inner.span(),
            Self::String(inner) => inner.span(),
        }
    }
}
#[derive(Debug, Clone)]
pub enum LowerBoundType<'tree> {
    AnnotatedType(::std::boxed::Box<AnnotatedType<'tree>>),
    AppliedConstructorType(::std::boxed::Box<AppliedConstructorType<'tree>>),
    CompoundType(::std::boxed::Box<CompoundType<'tree>>),
    FunctionType(::std::boxed::Box<FunctionType<'tree>>),
    GenericType(::std::boxed::Box<GenericType<'tree>>),
    InfixType(::std::boxed::Box<InfixType<'tree>>),
    LiteralType(::std::boxed::Box<LiteralType<'tree>>),
    MatchType(::std::boxed::Box<MatchType<'tree>>),
    NamedTupleType(::std::boxed::Box<NamedTupleType<'tree>>),
    ProjectedType(::std::boxed::Box<ProjectedType<'tree>>),
    SingletonType(::std::boxed::Box<SingletonType<'tree>>),
    StableTypeIdentifier(::std::boxed::Box<StableTypeIdentifier<'tree>>),
    StructuralType(::std::boxed::Box<StructuralType<'tree>>),
    TupleType(::std::boxed::Box<TupleType<'tree>>),
    TypeIdentifier(::std::boxed::Box<TypeIdentifier<'tree>>),
    TypeLambda(::std::boxed::Box<TypeLambda<'tree>>),
    Wildcard(::std::boxed::Box<Wildcard<'tree>>),
}
impl<'tree> ::treesitter_types::FromNode<'tree> for LowerBoundType<'tree> {
    #[allow(clippy::collapsible_else_if)]
    fn from_node(
        node: ::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        match node.kind() {
            "annotated_type" => Ok(Self::AnnotatedType(::std::boxed::Box::new(
                <AnnotatedType as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "applied_constructor_type" => Ok(Self::AppliedConstructorType(::std::boxed::Box::new(
                <AppliedConstructorType as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "compound_type" => Ok(Self::CompoundType(::std::boxed::Box::new(
                <CompoundType as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "function_type" => Ok(Self::FunctionType(::std::boxed::Box::new(
                <FunctionType as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "generic_type" => Ok(Self::GenericType(::std::boxed::Box::new(
                <GenericType as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "infix_type" => Ok(Self::InfixType(::std::boxed::Box::new(
                <InfixType as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "literal_type" => Ok(Self::LiteralType(::std::boxed::Box::new(
                <LiteralType as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "match_type" => Ok(Self::MatchType(::std::boxed::Box::new(
                <MatchType as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "named_tuple_type" => Ok(Self::NamedTupleType(::std::boxed::Box::new(
                <NamedTupleType as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "projected_type" => Ok(Self::ProjectedType(::std::boxed::Box::new(
                <ProjectedType as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "singleton_type" => Ok(Self::SingletonType(::std::boxed::Box::new(
                <SingletonType as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "stable_type_identifier" => Ok(Self::StableTypeIdentifier(::std::boxed::Box::new(
                <StableTypeIdentifier as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "structural_type" => Ok(Self::StructuralType(::std::boxed::Box::new(
                <StructuralType as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "tuple_type" => Ok(Self::TupleType(::std::boxed::Box::new(
                <TupleType as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "type_identifier" => Ok(Self::TypeIdentifier(::std::boxed::Box::new(
                <TypeIdentifier as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "type_lambda" => Ok(Self::TypeLambda(::std::boxed::Box::new(
                <TypeLambda as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "wildcard" => Ok(Self::Wildcard(::std::boxed::Box::new(
                <Wildcard as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            other => Err(::treesitter_types::ParseError::unexpected_kind(other, node)),
        }
    }
}
impl ::treesitter_types::Spanned for LowerBoundType<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        match self {
            Self::AnnotatedType(inner) => inner.span(),
            Self::AppliedConstructorType(inner) => inner.span(),
            Self::CompoundType(inner) => inner.span(),
            Self::FunctionType(inner) => inner.span(),
            Self::GenericType(inner) => inner.span(),
            Self::InfixType(inner) => inner.span(),
            Self::LiteralType(inner) => inner.span(),
            Self::MatchType(inner) => inner.span(),
            Self::NamedTupleType(inner) => inner.span(),
            Self::ProjectedType(inner) => inner.span(),
            Self::SingletonType(inner) => inner.span(),
            Self::StableTypeIdentifier(inner) => inner.span(),
            Self::StructuralType(inner) => inner.span(),
            Self::TupleType(inner) => inner.span(),
            Self::TypeIdentifier(inner) => inner.span(),
            Self::TypeLambda(inner) => inner.span(),
            Self::Wildcard(inner) => inner.span(),
        }
    }
}
#[derive(Debug, Clone)]
pub enum MacroBodyChildren<'tree> {
    Block(::std::boxed::Box<Block<'tree>>),
    BooleanLiteral(::std::boxed::Box<BooleanLiteral<'tree>>),
    CallExpression(::std::boxed::Box<CallExpression<'tree>>),
    CaseBlock(::std::boxed::Box<CaseBlock<'tree>>),
    CharacterLiteral(::std::boxed::Box<CharacterLiteral<'tree>>),
    FieldExpression(::std::boxed::Box<FieldExpression<'tree>>),
    FloatingPointLiteral(::std::boxed::Box<FloatingPointLiteral<'tree>>),
    GenericFunction(::std::boxed::Box<GenericFunction<'tree>>),
    Identifier(::std::boxed::Box<Identifier<'tree>>),
    InfixExpression(::std::boxed::Box<InfixExpression<'tree>>),
    InstanceExpression(::std::boxed::Box<InstanceExpression<'tree>>),
    IntegerLiteral(::std::boxed::Box<IntegerLiteral<'tree>>),
    InterpolatedStringExpression(::std::boxed::Box<InterpolatedStringExpression<'tree>>),
    NullLiteral(::std::boxed::Box<NullLiteral<'tree>>),
    OperatorIdentifier(::std::boxed::Box<OperatorIdentifier<'tree>>),
    ParenthesizedExpression(::std::boxed::Box<ParenthesizedExpression<'tree>>),
    PrefixExpression(::std::boxed::Box<PrefixExpression<'tree>>),
    QuoteExpression(::std::boxed::Box<QuoteExpression<'tree>>),
    SpliceExpression(::std::boxed::Box<SpliceExpression<'tree>>),
    String(::std::boxed::Box<String<'tree>>),
    TupleExpression(::std::boxed::Box<TupleExpression<'tree>>),
    Unit(::std::boxed::Box<Unit<'tree>>),
    Wildcard(::std::boxed::Box<Wildcard<'tree>>),
}
impl<'tree> ::treesitter_types::FromNode<'tree> for MacroBodyChildren<'tree> {
    #[allow(clippy::collapsible_else_if)]
    fn from_node(
        node: ::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        match node.kind() {
            "block" => Ok(Self::Block(::std::boxed::Box::new(
                <Block as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "boolean_literal" => Ok(Self::BooleanLiteral(::std::boxed::Box::new(
                <BooleanLiteral as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "call_expression" => Ok(Self::CallExpression(::std::boxed::Box::new(
                <CallExpression as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "case_block" => Ok(Self::CaseBlock(::std::boxed::Box::new(
                <CaseBlock as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "character_literal" => Ok(Self::CharacterLiteral(::std::boxed::Box::new(
                <CharacterLiteral as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "field_expression" => Ok(Self::FieldExpression(::std::boxed::Box::new(
                <FieldExpression as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "floating_point_literal" => Ok(Self::FloatingPointLiteral(::std::boxed::Box::new(
                <FloatingPointLiteral as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "generic_function" => Ok(Self::GenericFunction(::std::boxed::Box::new(
                <GenericFunction as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "identifier" => Ok(Self::Identifier(::std::boxed::Box::new(
                <Identifier as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "infix_expression" => Ok(Self::InfixExpression(::std::boxed::Box::new(
                <InfixExpression as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "instance_expression" => Ok(Self::InstanceExpression(::std::boxed::Box::new(
                <InstanceExpression as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "integer_literal" => Ok(Self::IntegerLiteral(::std::boxed::Box::new(
                <IntegerLiteral as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "interpolated_string_expression" => {
                Ok(Self::InterpolatedStringExpression(::std::boxed::Box::new(
                    <InterpolatedStringExpression as ::treesitter_types::FromNode>::from_node(
                        node, src,
                    )?,
                )))
            }
            "null_literal" => Ok(Self::NullLiteral(::std::boxed::Box::new(
                <NullLiteral as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "operator_identifier" => Ok(Self::OperatorIdentifier(::std::boxed::Box::new(
                <OperatorIdentifier as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "parenthesized_expression" => {
                Ok(Self::ParenthesizedExpression(::std::boxed::Box::new(
                    <ParenthesizedExpression as ::treesitter_types::FromNode>::from_node(
                        node, src,
                    )?,
                )))
            }
            "prefix_expression" => Ok(Self::PrefixExpression(::std::boxed::Box::new(
                <PrefixExpression as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "quote_expression" => Ok(Self::QuoteExpression(::std::boxed::Box::new(
                <QuoteExpression as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "splice_expression" => Ok(Self::SpliceExpression(::std::boxed::Box::new(
                <SpliceExpression as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "string" => Ok(Self::String(::std::boxed::Box::new(
                <String as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "tuple_expression" => Ok(Self::TupleExpression(::std::boxed::Box::new(
                <TupleExpression as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "unit" => Ok(Self::Unit(::std::boxed::Box::new(
                <Unit as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "wildcard" => Ok(Self::Wildcard(::std::boxed::Box::new(
                <Wildcard as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            other => Err(::treesitter_types::ParseError::unexpected_kind(other, node)),
        }
    }
}
impl ::treesitter_types::Spanned for MacroBodyChildren<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        match self {
            Self::Block(inner) => inner.span(),
            Self::BooleanLiteral(inner) => inner.span(),
            Self::CallExpression(inner) => inner.span(),
            Self::CaseBlock(inner) => inner.span(),
            Self::CharacterLiteral(inner) => inner.span(),
            Self::FieldExpression(inner) => inner.span(),
            Self::FloatingPointLiteral(inner) => inner.span(),
            Self::GenericFunction(inner) => inner.span(),
            Self::Identifier(inner) => inner.span(),
            Self::InfixExpression(inner) => inner.span(),
            Self::InstanceExpression(inner) => inner.span(),
            Self::IntegerLiteral(inner) => inner.span(),
            Self::InterpolatedStringExpression(inner) => inner.span(),
            Self::NullLiteral(inner) => inner.span(),
            Self::OperatorIdentifier(inner) => inner.span(),
            Self::ParenthesizedExpression(inner) => inner.span(),
            Self::PrefixExpression(inner) => inner.span(),
            Self::QuoteExpression(inner) => inner.span(),
            Self::SpliceExpression(inner) => inner.span(),
            Self::String(inner) => inner.span(),
            Self::TupleExpression(inner) => inner.span(),
            Self::Unit(inner) => inner.span(),
            Self::Wildcard(inner) => inner.span(),
        }
    }
}
#[derive(Debug, Clone)]
pub enum MatchExpressionBody<'tree> {
    CaseBlock(::std::boxed::Box<CaseBlock<'tree>>),
    IndentedCases(::std::boxed::Box<IndentedCases<'tree>>),
}
impl<'tree> ::treesitter_types::FromNode<'tree> for MatchExpressionBody<'tree> {
    #[allow(clippy::collapsible_else_if)]
    fn from_node(
        node: ::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        match node.kind() {
            "case_block" => Ok(Self::CaseBlock(::std::boxed::Box::new(
                <CaseBlock as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "indented_cases" => Ok(Self::IndentedCases(::std::boxed::Box::new(
                <IndentedCases as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            other => Err(::treesitter_types::ParseError::unexpected_kind(other, node)),
        }
    }
}
impl ::treesitter_types::Spanned for MatchExpressionBody<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        match self {
            Self::CaseBlock(inner) => inner.span(),
            Self::IndentedCases(inner) => inner.span(),
        }
    }
}
#[derive(Debug, Clone)]
pub enum MatchTypeChildren<'tree> {
    AnnotatedType(::std::boxed::Box<AnnotatedType<'tree>>),
    AppliedConstructorType(::std::boxed::Box<AppliedConstructorType<'tree>>),
    CompoundType(::std::boxed::Box<CompoundType<'tree>>),
    GenericType(::std::boxed::Box<GenericType<'tree>>),
    InfixType(::std::boxed::Box<InfixType<'tree>>),
    LiteralType(::std::boxed::Box<LiteralType<'tree>>),
    NamedTupleType(::std::boxed::Box<NamedTupleType<'tree>>),
    ProjectedType(::std::boxed::Box<ProjectedType<'tree>>),
    SingletonType(::std::boxed::Box<SingletonType<'tree>>),
    StableTypeIdentifier(::std::boxed::Box<StableTypeIdentifier<'tree>>),
    TupleType(::std::boxed::Box<TupleType<'tree>>),
    TypeCaseClause(::std::boxed::Box<TypeCaseClause<'tree>>),
    TypeIdentifier(::std::boxed::Box<TypeIdentifier<'tree>>),
    Wildcard(::std::boxed::Box<Wildcard<'tree>>),
}
impl<'tree> ::treesitter_types::FromNode<'tree> for MatchTypeChildren<'tree> {
    #[allow(clippy::collapsible_else_if)]
    fn from_node(
        node: ::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        match node.kind() {
            "annotated_type" => Ok(Self::AnnotatedType(::std::boxed::Box::new(
                <AnnotatedType as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "applied_constructor_type" => Ok(Self::AppliedConstructorType(::std::boxed::Box::new(
                <AppliedConstructorType as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "compound_type" => Ok(Self::CompoundType(::std::boxed::Box::new(
                <CompoundType as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "generic_type" => Ok(Self::GenericType(::std::boxed::Box::new(
                <GenericType as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "infix_type" => Ok(Self::InfixType(::std::boxed::Box::new(
                <InfixType as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "literal_type" => Ok(Self::LiteralType(::std::boxed::Box::new(
                <LiteralType as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "named_tuple_type" => Ok(Self::NamedTupleType(::std::boxed::Box::new(
                <NamedTupleType as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "projected_type" => Ok(Self::ProjectedType(::std::boxed::Box::new(
                <ProjectedType as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "singleton_type" => Ok(Self::SingletonType(::std::boxed::Box::new(
                <SingletonType as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "stable_type_identifier" => Ok(Self::StableTypeIdentifier(::std::boxed::Box::new(
                <StableTypeIdentifier as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "tuple_type" => Ok(Self::TupleType(::std::boxed::Box::new(
                <TupleType as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "type_case_clause" => Ok(Self::TypeCaseClause(::std::boxed::Box::new(
                <TypeCaseClause as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "type_identifier" => Ok(Self::TypeIdentifier(::std::boxed::Box::new(
                <TypeIdentifier as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "wildcard" => Ok(Self::Wildcard(::std::boxed::Box::new(
                <Wildcard as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            other => Err(::treesitter_types::ParseError::unexpected_kind(other, node)),
        }
    }
}
impl ::treesitter_types::Spanned for MatchTypeChildren<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        match self {
            Self::AnnotatedType(inner) => inner.span(),
            Self::AppliedConstructorType(inner) => inner.span(),
            Self::CompoundType(inner) => inner.span(),
            Self::GenericType(inner) => inner.span(),
            Self::InfixType(inner) => inner.span(),
            Self::LiteralType(inner) => inner.span(),
            Self::NamedTupleType(inner) => inner.span(),
            Self::ProjectedType(inner) => inner.span(),
            Self::SingletonType(inner) => inner.span(),
            Self::StableTypeIdentifier(inner) => inner.span(),
            Self::TupleType(inner) => inner.span(),
            Self::TypeCaseClause(inner) => inner.span(),
            Self::TypeIdentifier(inner) => inner.span(),
            Self::Wildcard(inner) => inner.span(),
        }
    }
}
#[derive(Debug, Clone)]
pub enum ModifiersChildren<'tree> {
    AccessModifier(::std::boxed::Box<AccessModifier<'tree>>),
    InfixModifier(::std::boxed::Box<InfixModifier<'tree>>),
    InlineModifier(::std::boxed::Box<InlineModifier<'tree>>),
    IntoModifier(::std::boxed::Box<IntoModifier<'tree>>),
    OpenModifier(::std::boxed::Box<OpenModifier<'tree>>),
    TrackedModifier(::std::boxed::Box<TrackedModifier<'tree>>),
    TransparentModifier(::std::boxed::Box<TransparentModifier<'tree>>),
}
impl<'tree> ::treesitter_types::FromNode<'tree> for ModifiersChildren<'tree> {
    #[allow(clippy::collapsible_else_if)]
    fn from_node(
        node: ::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        match node.kind() {
            "access_modifier" => Ok(Self::AccessModifier(::std::boxed::Box::new(
                <AccessModifier as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "infix_modifier" => Ok(Self::InfixModifier(::std::boxed::Box::new(
                <InfixModifier as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "inline_modifier" => Ok(Self::InlineModifier(::std::boxed::Box::new(
                <InlineModifier as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "into_modifier" => Ok(Self::IntoModifier(::std::boxed::Box::new(
                <IntoModifier as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "open_modifier" => Ok(Self::OpenModifier(::std::boxed::Box::new(
                <OpenModifier as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "tracked_modifier" => Ok(Self::TrackedModifier(::std::boxed::Box::new(
                <TrackedModifier as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "transparent_modifier" => Ok(Self::TransparentModifier(::std::boxed::Box::new(
                <TransparentModifier as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            other => Err(::treesitter_types::ParseError::unexpected_kind(other, node)),
        }
    }
}
impl ::treesitter_types::Spanned for ModifiersChildren<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        match self {
            Self::AccessModifier(inner) => inner.span(),
            Self::InfixModifier(inner) => inner.span(),
            Self::InlineModifier(inner) => inner.span(),
            Self::IntoModifier(inner) => inner.span(),
            Self::OpenModifier(inner) => inner.span(),
            Self::TrackedModifier(inner) => inner.span(),
            Self::TransparentModifier(inner) => inner.span(),
        }
    }
}
#[derive(Debug, Clone)]
pub enum NameAndTypeName<'tree> {
    Identifier(::std::boxed::Box<Identifier<'tree>>),
    OperatorIdentifier(::std::boxed::Box<OperatorIdentifier<'tree>>),
}
impl<'tree> ::treesitter_types::FromNode<'tree> for NameAndTypeName<'tree> {
    #[allow(clippy::collapsible_else_if)]
    fn from_node(
        node: ::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        match node.kind() {
            "identifier" => Ok(Self::Identifier(::std::boxed::Box::new(
                <Identifier as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "operator_identifier" => Ok(Self::OperatorIdentifier(::std::boxed::Box::new(
                <OperatorIdentifier as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            other => Err(::treesitter_types::ParseError::unexpected_kind(other, node)),
        }
    }
}
impl ::treesitter_types::Spanned for NameAndTypeName<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        match self {
            Self::Identifier(inner) => inner.span(),
            Self::OperatorIdentifier(inner) => inner.span(),
        }
    }
}
#[derive(Debug, Clone)]
pub enum NameAndTypeType<'tree> {
    AnnotatedType(::std::boxed::Box<AnnotatedType<'tree>>),
    AppliedConstructorType(::std::boxed::Box<AppliedConstructorType<'tree>>),
    CompoundType(::std::boxed::Box<CompoundType<'tree>>),
    FunctionType(::std::boxed::Box<FunctionType<'tree>>),
    GenericType(::std::boxed::Box<GenericType<'tree>>),
    InfixType(::std::boxed::Box<InfixType<'tree>>),
    LazyParameterType(::std::boxed::Box<LazyParameterType<'tree>>),
    LiteralType(::std::boxed::Box<LiteralType<'tree>>),
    MatchType(::std::boxed::Box<MatchType<'tree>>),
    NamedTupleType(::std::boxed::Box<NamedTupleType<'tree>>),
    ProjectedType(::std::boxed::Box<ProjectedType<'tree>>),
    RepeatedParameterType(::std::boxed::Box<RepeatedParameterType<'tree>>),
    SingletonType(::std::boxed::Box<SingletonType<'tree>>),
    StableTypeIdentifier(::std::boxed::Box<StableTypeIdentifier<'tree>>),
    StructuralType(::std::boxed::Box<StructuralType<'tree>>),
    TupleType(::std::boxed::Box<TupleType<'tree>>),
    TypeIdentifier(::std::boxed::Box<TypeIdentifier<'tree>>),
    TypeLambda(::std::boxed::Box<TypeLambda<'tree>>),
    Wildcard(::std::boxed::Box<Wildcard<'tree>>),
}
impl<'tree> ::treesitter_types::FromNode<'tree> for NameAndTypeType<'tree> {
    #[allow(clippy::collapsible_else_if)]
    fn from_node(
        node: ::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        match node.kind() {
            "annotated_type" => Ok(Self::AnnotatedType(::std::boxed::Box::new(
                <AnnotatedType as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "applied_constructor_type" => Ok(Self::AppliedConstructorType(::std::boxed::Box::new(
                <AppliedConstructorType as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "compound_type" => Ok(Self::CompoundType(::std::boxed::Box::new(
                <CompoundType as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "function_type" => Ok(Self::FunctionType(::std::boxed::Box::new(
                <FunctionType as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "generic_type" => Ok(Self::GenericType(::std::boxed::Box::new(
                <GenericType as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "infix_type" => Ok(Self::InfixType(::std::boxed::Box::new(
                <InfixType as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "lazy_parameter_type" => Ok(Self::LazyParameterType(::std::boxed::Box::new(
                <LazyParameterType as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "literal_type" => Ok(Self::LiteralType(::std::boxed::Box::new(
                <LiteralType as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "match_type" => Ok(Self::MatchType(::std::boxed::Box::new(
                <MatchType as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "named_tuple_type" => Ok(Self::NamedTupleType(::std::boxed::Box::new(
                <NamedTupleType as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "projected_type" => Ok(Self::ProjectedType(::std::boxed::Box::new(
                <ProjectedType as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "repeated_parameter_type" => Ok(Self::RepeatedParameterType(::std::boxed::Box::new(
                <RepeatedParameterType as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "singleton_type" => Ok(Self::SingletonType(::std::boxed::Box::new(
                <SingletonType as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "stable_type_identifier" => Ok(Self::StableTypeIdentifier(::std::boxed::Box::new(
                <StableTypeIdentifier as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "structural_type" => Ok(Self::StructuralType(::std::boxed::Box::new(
                <StructuralType as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "tuple_type" => Ok(Self::TupleType(::std::boxed::Box::new(
                <TupleType as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "type_identifier" => Ok(Self::TypeIdentifier(::std::boxed::Box::new(
                <TypeIdentifier as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "type_lambda" => Ok(Self::TypeLambda(::std::boxed::Box::new(
                <TypeLambda as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "wildcard" => Ok(Self::Wildcard(::std::boxed::Box::new(
                <Wildcard as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            other => Err(::treesitter_types::ParseError::unexpected_kind(other, node)),
        }
    }
}
impl ::treesitter_types::Spanned for NameAndTypeType<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        match self {
            Self::AnnotatedType(inner) => inner.span(),
            Self::AppliedConstructorType(inner) => inner.span(),
            Self::CompoundType(inner) => inner.span(),
            Self::FunctionType(inner) => inner.span(),
            Self::GenericType(inner) => inner.span(),
            Self::InfixType(inner) => inner.span(),
            Self::LazyParameterType(inner) => inner.span(),
            Self::LiteralType(inner) => inner.span(),
            Self::MatchType(inner) => inner.span(),
            Self::NamedTupleType(inner) => inner.span(),
            Self::ProjectedType(inner) => inner.span(),
            Self::RepeatedParameterType(inner) => inner.span(),
            Self::SingletonType(inner) => inner.span(),
            Self::StableTypeIdentifier(inner) => inner.span(),
            Self::StructuralType(inner) => inner.span(),
            Self::TupleType(inner) => inner.span(),
            Self::TypeIdentifier(inner) => inner.span(),
            Self::TypeLambda(inner) => inner.span(),
            Self::Wildcard(inner) => inner.span(),
        }
    }
}
#[derive(Debug, Clone)]
pub enum NamespaceSelectorsChildren<'tree> {
    AnnotatedType(::std::boxed::Box<AnnotatedType<'tree>>),
    AppliedConstructorType(::std::boxed::Box<AppliedConstructorType<'tree>>),
    ArrowRenamedIdentifier(::std::boxed::Box<ArrowRenamedIdentifier<'tree>>),
    AsRenamedIdentifier(::std::boxed::Box<AsRenamedIdentifier<'tree>>),
    CompoundType(::std::boxed::Box<CompoundType<'tree>>),
    FunctionType(::std::boxed::Box<FunctionType<'tree>>),
    GenericType(::std::boxed::Box<GenericType<'tree>>),
    Identifier(::std::boxed::Box<Identifier<'tree>>),
    InfixType(::std::boxed::Box<InfixType<'tree>>),
    LiteralType(::std::boxed::Box<LiteralType<'tree>>),
    MatchType(::std::boxed::Box<MatchType<'tree>>),
    NamedTupleType(::std::boxed::Box<NamedTupleType<'tree>>),
    NamespaceWildcard(::std::boxed::Box<NamespaceWildcard<'tree>>),
    OperatorIdentifier(::std::boxed::Box<OperatorIdentifier<'tree>>),
    ProjectedType(::std::boxed::Box<ProjectedType<'tree>>),
    SingletonType(::std::boxed::Box<SingletonType<'tree>>),
    StableTypeIdentifier(::std::boxed::Box<StableTypeIdentifier<'tree>>),
    StructuralType(::std::boxed::Box<StructuralType<'tree>>),
    TupleType(::std::boxed::Box<TupleType<'tree>>),
    TypeIdentifier(::std::boxed::Box<TypeIdentifier<'tree>>),
    TypeLambda(::std::boxed::Box<TypeLambda<'tree>>),
    Wildcard(::std::boxed::Box<Wildcard<'tree>>),
}
impl<'tree> ::treesitter_types::FromNode<'tree> for NamespaceSelectorsChildren<'tree> {
    #[allow(clippy::collapsible_else_if)]
    fn from_node(
        node: ::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        match node.kind() {
            "annotated_type" => Ok(Self::AnnotatedType(::std::boxed::Box::new(
                <AnnotatedType as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "applied_constructor_type" => Ok(Self::AppliedConstructorType(::std::boxed::Box::new(
                <AppliedConstructorType as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "arrow_renamed_identifier" => Ok(Self::ArrowRenamedIdentifier(::std::boxed::Box::new(
                <ArrowRenamedIdentifier as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "as_renamed_identifier" => Ok(Self::AsRenamedIdentifier(::std::boxed::Box::new(
                <AsRenamedIdentifier as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "compound_type" => Ok(Self::CompoundType(::std::boxed::Box::new(
                <CompoundType as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "function_type" => Ok(Self::FunctionType(::std::boxed::Box::new(
                <FunctionType as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "generic_type" => Ok(Self::GenericType(::std::boxed::Box::new(
                <GenericType as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "identifier" => Ok(Self::Identifier(::std::boxed::Box::new(
                <Identifier as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "infix_type" => Ok(Self::InfixType(::std::boxed::Box::new(
                <InfixType as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "literal_type" => Ok(Self::LiteralType(::std::boxed::Box::new(
                <LiteralType as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "match_type" => Ok(Self::MatchType(::std::boxed::Box::new(
                <MatchType as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "named_tuple_type" => Ok(Self::NamedTupleType(::std::boxed::Box::new(
                <NamedTupleType as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "namespace_wildcard" => Ok(Self::NamespaceWildcard(::std::boxed::Box::new(
                <NamespaceWildcard as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "operator_identifier" => Ok(Self::OperatorIdentifier(::std::boxed::Box::new(
                <OperatorIdentifier as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "projected_type" => Ok(Self::ProjectedType(::std::boxed::Box::new(
                <ProjectedType as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "singleton_type" => Ok(Self::SingletonType(::std::boxed::Box::new(
                <SingletonType as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "stable_type_identifier" => Ok(Self::StableTypeIdentifier(::std::boxed::Box::new(
                <StableTypeIdentifier as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "structural_type" => Ok(Self::StructuralType(::std::boxed::Box::new(
                <StructuralType as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "tuple_type" => Ok(Self::TupleType(::std::boxed::Box::new(
                <TupleType as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "type_identifier" => Ok(Self::TypeIdentifier(::std::boxed::Box::new(
                <TypeIdentifier as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "type_lambda" => Ok(Self::TypeLambda(::std::boxed::Box::new(
                <TypeLambda as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "wildcard" => Ok(Self::Wildcard(::std::boxed::Box::new(
                <Wildcard as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            other => Err(::treesitter_types::ParseError::unexpected_kind(other, node)),
        }
    }
}
impl ::treesitter_types::Spanned for NamespaceSelectorsChildren<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        match self {
            Self::AnnotatedType(inner) => inner.span(),
            Self::AppliedConstructorType(inner) => inner.span(),
            Self::ArrowRenamedIdentifier(inner) => inner.span(),
            Self::AsRenamedIdentifier(inner) => inner.span(),
            Self::CompoundType(inner) => inner.span(),
            Self::FunctionType(inner) => inner.span(),
            Self::GenericType(inner) => inner.span(),
            Self::Identifier(inner) => inner.span(),
            Self::InfixType(inner) => inner.span(),
            Self::LiteralType(inner) => inner.span(),
            Self::MatchType(inner) => inner.span(),
            Self::NamedTupleType(inner) => inner.span(),
            Self::NamespaceWildcard(inner) => inner.span(),
            Self::OperatorIdentifier(inner) => inner.span(),
            Self::ProjectedType(inner) => inner.span(),
            Self::SingletonType(inner) => inner.span(),
            Self::StableTypeIdentifier(inner) => inner.span(),
            Self::StructuralType(inner) => inner.span(),
            Self::TupleType(inner) => inner.span(),
            Self::TypeIdentifier(inner) => inner.span(),
            Self::TypeLambda(inner) => inner.span(),
            Self::Wildcard(inner) => inner.span(),
        }
    }
}
#[derive(Debug, Clone)]
pub enum ObjectDefinitionName<'tree> {
    Identifier(::std::boxed::Box<Identifier<'tree>>),
    OperatorIdentifier(::std::boxed::Box<OperatorIdentifier<'tree>>),
}
impl<'tree> ::treesitter_types::FromNode<'tree> for ObjectDefinitionName<'tree> {
    #[allow(clippy::collapsible_else_if)]
    fn from_node(
        node: ::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        match node.kind() {
            "identifier" => Ok(Self::Identifier(::std::boxed::Box::new(
                <Identifier as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "operator_identifier" => Ok(Self::OperatorIdentifier(::std::boxed::Box::new(
                <OperatorIdentifier as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            other => Err(::treesitter_types::ParseError::unexpected_kind(other, node)),
        }
    }
}
impl ::treesitter_types::Spanned for ObjectDefinitionName<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        match self {
            Self::Identifier(inner) => inner.span(),
            Self::OperatorIdentifier(inner) => inner.span(),
        }
    }
}
#[derive(Debug, Clone)]
pub enum ObjectDefinitionChildren<'tree> {
    Annotation(::std::boxed::Box<Annotation<'tree>>),
    Modifiers(::std::boxed::Box<Modifiers<'tree>>),
}
impl<'tree> ::treesitter_types::FromNode<'tree> for ObjectDefinitionChildren<'tree> {
    #[allow(clippy::collapsible_else_if)]
    fn from_node(
        node: ::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        match node.kind() {
            "annotation" => Ok(Self::Annotation(::std::boxed::Box::new(
                <Annotation as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "modifiers" => Ok(Self::Modifiers(::std::boxed::Box::new(
                <Modifiers as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            other => Err(::treesitter_types::ParseError::unexpected_kind(other, node)),
        }
    }
}
impl ::treesitter_types::Spanned for ObjectDefinitionChildren<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        match self {
            Self::Annotation(inner) => inner.span(),
            Self::Modifiers(inner) => inner.span(),
        }
    }
}
#[derive(Debug, Clone)]
pub enum PackageIdentifierChildren<'tree> {
    Identifier(::std::boxed::Box<Identifier<'tree>>),
    OperatorIdentifier(::std::boxed::Box<OperatorIdentifier<'tree>>),
}
impl<'tree> ::treesitter_types::FromNode<'tree> for PackageIdentifierChildren<'tree> {
    #[allow(clippy::collapsible_else_if)]
    fn from_node(
        node: ::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        match node.kind() {
            "identifier" => Ok(Self::Identifier(::std::boxed::Box::new(
                <Identifier as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "operator_identifier" => Ok(Self::OperatorIdentifier(::std::boxed::Box::new(
                <OperatorIdentifier as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            other => Err(::treesitter_types::ParseError::unexpected_kind(other, node)),
        }
    }
}
impl ::treesitter_types::Spanned for PackageIdentifierChildren<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        match self {
            Self::Identifier(inner) => inner.span(),
            Self::OperatorIdentifier(inner) => inner.span(),
        }
    }
}
#[derive(Debug, Clone)]
pub enum PackageObjectName<'tree> {
    Identifier(::std::boxed::Box<Identifier<'tree>>),
    OperatorIdentifier(::std::boxed::Box<OperatorIdentifier<'tree>>),
}
impl<'tree> ::treesitter_types::FromNode<'tree> for PackageObjectName<'tree> {
    #[allow(clippy::collapsible_else_if)]
    fn from_node(
        node: ::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        match node.kind() {
            "identifier" => Ok(Self::Identifier(::std::boxed::Box::new(
                <Identifier as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "operator_identifier" => Ok(Self::OperatorIdentifier(::std::boxed::Box::new(
                <OperatorIdentifier as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            other => Err(::treesitter_types::ParseError::unexpected_kind(other, node)),
        }
    }
}
impl ::treesitter_types::Spanned for PackageObjectName<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        match self {
            Self::Identifier(inner) => inner.span(),
            Self::OperatorIdentifier(inner) => inner.span(),
        }
    }
}
#[derive(Debug, Clone)]
pub enum ParameterName<'tree> {
    Identifier(::std::boxed::Box<Identifier<'tree>>),
    OperatorIdentifier(::std::boxed::Box<OperatorIdentifier<'tree>>),
}
impl<'tree> ::treesitter_types::FromNode<'tree> for ParameterName<'tree> {
    #[allow(clippy::collapsible_else_if)]
    fn from_node(
        node: ::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        match node.kind() {
            "identifier" => Ok(Self::Identifier(::std::boxed::Box::new(
                <Identifier as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "operator_identifier" => Ok(Self::OperatorIdentifier(::std::boxed::Box::new(
                <OperatorIdentifier as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            other => Err(::treesitter_types::ParseError::unexpected_kind(other, node)),
        }
    }
}
impl ::treesitter_types::Spanned for ParameterName<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        match self {
            Self::Identifier(inner) => inner.span(),
            Self::OperatorIdentifier(inner) => inner.span(),
        }
    }
}
#[derive(Debug, Clone)]
pub enum ParameterType<'tree> {
    AnnotatedType(::std::boxed::Box<AnnotatedType<'tree>>),
    AppliedConstructorType(::std::boxed::Box<AppliedConstructorType<'tree>>),
    CompoundType(::std::boxed::Box<CompoundType<'tree>>),
    FunctionType(::std::boxed::Box<FunctionType<'tree>>),
    GenericType(::std::boxed::Box<GenericType<'tree>>),
    InfixType(::std::boxed::Box<InfixType<'tree>>),
    LazyParameterType(::std::boxed::Box<LazyParameterType<'tree>>),
    LiteralType(::std::boxed::Box<LiteralType<'tree>>),
    MatchType(::std::boxed::Box<MatchType<'tree>>),
    NamedTupleType(::std::boxed::Box<NamedTupleType<'tree>>),
    ProjectedType(::std::boxed::Box<ProjectedType<'tree>>),
    RepeatedParameterType(::std::boxed::Box<RepeatedParameterType<'tree>>),
    SingletonType(::std::boxed::Box<SingletonType<'tree>>),
    StableTypeIdentifier(::std::boxed::Box<StableTypeIdentifier<'tree>>),
    StructuralType(::std::boxed::Box<StructuralType<'tree>>),
    TupleType(::std::boxed::Box<TupleType<'tree>>),
    TypeIdentifier(::std::boxed::Box<TypeIdentifier<'tree>>),
    TypeLambda(::std::boxed::Box<TypeLambda<'tree>>),
    Wildcard(::std::boxed::Box<Wildcard<'tree>>),
}
impl<'tree> ::treesitter_types::FromNode<'tree> for ParameterType<'tree> {
    #[allow(clippy::collapsible_else_if)]
    fn from_node(
        node: ::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        match node.kind() {
            "annotated_type" => Ok(Self::AnnotatedType(::std::boxed::Box::new(
                <AnnotatedType as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "applied_constructor_type" => Ok(Self::AppliedConstructorType(::std::boxed::Box::new(
                <AppliedConstructorType as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "compound_type" => Ok(Self::CompoundType(::std::boxed::Box::new(
                <CompoundType as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "function_type" => Ok(Self::FunctionType(::std::boxed::Box::new(
                <FunctionType as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "generic_type" => Ok(Self::GenericType(::std::boxed::Box::new(
                <GenericType as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "infix_type" => Ok(Self::InfixType(::std::boxed::Box::new(
                <InfixType as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "lazy_parameter_type" => Ok(Self::LazyParameterType(::std::boxed::Box::new(
                <LazyParameterType as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "literal_type" => Ok(Self::LiteralType(::std::boxed::Box::new(
                <LiteralType as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "match_type" => Ok(Self::MatchType(::std::boxed::Box::new(
                <MatchType as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "named_tuple_type" => Ok(Self::NamedTupleType(::std::boxed::Box::new(
                <NamedTupleType as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "projected_type" => Ok(Self::ProjectedType(::std::boxed::Box::new(
                <ProjectedType as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "repeated_parameter_type" => Ok(Self::RepeatedParameterType(::std::boxed::Box::new(
                <RepeatedParameterType as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "singleton_type" => Ok(Self::SingletonType(::std::boxed::Box::new(
                <SingletonType as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "stable_type_identifier" => Ok(Self::StableTypeIdentifier(::std::boxed::Box::new(
                <StableTypeIdentifier as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "structural_type" => Ok(Self::StructuralType(::std::boxed::Box::new(
                <StructuralType as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "tuple_type" => Ok(Self::TupleType(::std::boxed::Box::new(
                <TupleType as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "type_identifier" => Ok(Self::TypeIdentifier(::std::boxed::Box::new(
                <TypeIdentifier as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "type_lambda" => Ok(Self::TypeLambda(::std::boxed::Box::new(
                <TypeLambda as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "wildcard" => Ok(Self::Wildcard(::std::boxed::Box::new(
                <Wildcard as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            other => Err(::treesitter_types::ParseError::unexpected_kind(other, node)),
        }
    }
}
impl ::treesitter_types::Spanned for ParameterType<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        match self {
            Self::AnnotatedType(inner) => inner.span(),
            Self::AppliedConstructorType(inner) => inner.span(),
            Self::CompoundType(inner) => inner.span(),
            Self::FunctionType(inner) => inner.span(),
            Self::GenericType(inner) => inner.span(),
            Self::InfixType(inner) => inner.span(),
            Self::LazyParameterType(inner) => inner.span(),
            Self::LiteralType(inner) => inner.span(),
            Self::MatchType(inner) => inner.span(),
            Self::NamedTupleType(inner) => inner.span(),
            Self::ProjectedType(inner) => inner.span(),
            Self::RepeatedParameterType(inner) => inner.span(),
            Self::SingletonType(inner) => inner.span(),
            Self::StableTypeIdentifier(inner) => inner.span(),
            Self::StructuralType(inner) => inner.span(),
            Self::TupleType(inner) => inner.span(),
            Self::TypeIdentifier(inner) => inner.span(),
            Self::TypeLambda(inner) => inner.span(),
            Self::Wildcard(inner) => inner.span(),
        }
    }
}
#[derive(Debug, Clone)]
pub enum ParameterChildren<'tree> {
    Annotation(::std::boxed::Box<Annotation<'tree>>),
    InlineModifier(::std::boxed::Box<InlineModifier<'tree>>),
}
impl<'tree> ::treesitter_types::FromNode<'tree> for ParameterChildren<'tree> {
    #[allow(clippy::collapsible_else_if)]
    fn from_node(
        node: ::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        match node.kind() {
            "annotation" => Ok(Self::Annotation(::std::boxed::Box::new(
                <Annotation as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "inline_modifier" => Ok(Self::InlineModifier(::std::boxed::Box::new(
                <InlineModifier as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            other => Err(::treesitter_types::ParseError::unexpected_kind(other, node)),
        }
    }
}
impl ::treesitter_types::Spanned for ParameterChildren<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        match self {
            Self::Annotation(inner) => inner.span(),
            Self::InlineModifier(inner) => inner.span(),
        }
    }
}
#[derive(Debug, Clone)]
pub enum ParameterTypesChildren<'tree> {
    AnnotatedType(::std::boxed::Box<AnnotatedType<'tree>>),
    AppliedConstructorType(::std::boxed::Box<AppliedConstructorType<'tree>>),
    CompoundType(::std::boxed::Box<CompoundType<'tree>>),
    FunctionType(::std::boxed::Box<FunctionType<'tree>>),
    GenericType(::std::boxed::Box<GenericType<'tree>>),
    InfixType(::std::boxed::Box<InfixType<'tree>>),
    LazyParameterType(::std::boxed::Box<LazyParameterType<'tree>>),
    LiteralType(::std::boxed::Box<LiteralType<'tree>>),
    MatchType(::std::boxed::Box<MatchType<'tree>>),
    NamedTupleType(::std::boxed::Box<NamedTupleType<'tree>>),
    ProjectedType(::std::boxed::Box<ProjectedType<'tree>>),
    RepeatedParameterType(::std::boxed::Box<RepeatedParameterType<'tree>>),
    SingletonType(::std::boxed::Box<SingletonType<'tree>>),
    StableTypeIdentifier(::std::boxed::Box<StableTypeIdentifier<'tree>>),
    StructuralType(::std::boxed::Box<StructuralType<'tree>>),
    TupleType(::std::boxed::Box<TupleType<'tree>>),
    TypeIdentifier(::std::boxed::Box<TypeIdentifier<'tree>>),
    TypeLambda(::std::boxed::Box<TypeLambda<'tree>>),
    Wildcard(::std::boxed::Box<Wildcard<'tree>>),
}
impl<'tree> ::treesitter_types::FromNode<'tree> for ParameterTypesChildren<'tree> {
    #[allow(clippy::collapsible_else_if)]
    fn from_node(
        node: ::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        match node.kind() {
            "annotated_type" => Ok(Self::AnnotatedType(::std::boxed::Box::new(
                <AnnotatedType as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "applied_constructor_type" => Ok(Self::AppliedConstructorType(::std::boxed::Box::new(
                <AppliedConstructorType as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "compound_type" => Ok(Self::CompoundType(::std::boxed::Box::new(
                <CompoundType as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "function_type" => Ok(Self::FunctionType(::std::boxed::Box::new(
                <FunctionType as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "generic_type" => Ok(Self::GenericType(::std::boxed::Box::new(
                <GenericType as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "infix_type" => Ok(Self::InfixType(::std::boxed::Box::new(
                <InfixType as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "lazy_parameter_type" => Ok(Self::LazyParameterType(::std::boxed::Box::new(
                <LazyParameterType as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "literal_type" => Ok(Self::LiteralType(::std::boxed::Box::new(
                <LiteralType as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "match_type" => Ok(Self::MatchType(::std::boxed::Box::new(
                <MatchType as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "named_tuple_type" => Ok(Self::NamedTupleType(::std::boxed::Box::new(
                <NamedTupleType as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "projected_type" => Ok(Self::ProjectedType(::std::boxed::Box::new(
                <ProjectedType as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "repeated_parameter_type" => Ok(Self::RepeatedParameterType(::std::boxed::Box::new(
                <RepeatedParameterType as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "singleton_type" => Ok(Self::SingletonType(::std::boxed::Box::new(
                <SingletonType as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "stable_type_identifier" => Ok(Self::StableTypeIdentifier(::std::boxed::Box::new(
                <StableTypeIdentifier as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "structural_type" => Ok(Self::StructuralType(::std::boxed::Box::new(
                <StructuralType as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "tuple_type" => Ok(Self::TupleType(::std::boxed::Box::new(
                <TupleType as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "type_identifier" => Ok(Self::TypeIdentifier(::std::boxed::Box::new(
                <TypeIdentifier as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "type_lambda" => Ok(Self::TypeLambda(::std::boxed::Box::new(
                <TypeLambda as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "wildcard" => Ok(Self::Wildcard(::std::boxed::Box::new(
                <Wildcard as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            other => Err(::treesitter_types::ParseError::unexpected_kind(other, node)),
        }
    }
}
impl ::treesitter_types::Spanned for ParameterTypesChildren<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        match self {
            Self::AnnotatedType(inner) => inner.span(),
            Self::AppliedConstructorType(inner) => inner.span(),
            Self::CompoundType(inner) => inner.span(),
            Self::FunctionType(inner) => inner.span(),
            Self::GenericType(inner) => inner.span(),
            Self::InfixType(inner) => inner.span(),
            Self::LazyParameterType(inner) => inner.span(),
            Self::LiteralType(inner) => inner.span(),
            Self::MatchType(inner) => inner.span(),
            Self::NamedTupleType(inner) => inner.span(),
            Self::ProjectedType(inner) => inner.span(),
            Self::RepeatedParameterType(inner) => inner.span(),
            Self::SingletonType(inner) => inner.span(),
            Self::StableTypeIdentifier(inner) => inner.span(),
            Self::StructuralType(inner) => inner.span(),
            Self::TupleType(inner) => inner.span(),
            Self::TypeIdentifier(inner) => inner.span(),
            Self::TypeLambda(inner) => inner.span(),
            Self::Wildcard(inner) => inner.span(),
        }
    }
}
#[derive(Debug, Clone)]
pub enum ParametersChildren<'tree> {
    AnnotatedType(::std::boxed::Box<AnnotatedType<'tree>>),
    AppliedConstructorType(::std::boxed::Box<AppliedConstructorType<'tree>>),
    CompoundType(::std::boxed::Box<CompoundType<'tree>>),
    FunctionType(::std::boxed::Box<FunctionType<'tree>>),
    GenericType(::std::boxed::Box<GenericType<'tree>>),
    InfixType(::std::boxed::Box<InfixType<'tree>>),
    LazyParameterType(::std::boxed::Box<LazyParameterType<'tree>>),
    LiteralType(::std::boxed::Box<LiteralType<'tree>>),
    MatchType(::std::boxed::Box<MatchType<'tree>>),
    NamedTupleType(::std::boxed::Box<NamedTupleType<'tree>>),
    Parameter(::std::boxed::Box<Parameter<'tree>>),
    ProjectedType(::std::boxed::Box<ProjectedType<'tree>>),
    RepeatedParameterType(::std::boxed::Box<RepeatedParameterType<'tree>>),
    SingletonType(::std::boxed::Box<SingletonType<'tree>>),
    StableTypeIdentifier(::std::boxed::Box<StableTypeIdentifier<'tree>>),
    StructuralType(::std::boxed::Box<StructuralType<'tree>>),
    TupleType(::std::boxed::Box<TupleType<'tree>>),
    TypeIdentifier(::std::boxed::Box<TypeIdentifier<'tree>>),
    TypeLambda(::std::boxed::Box<TypeLambda<'tree>>),
    Wildcard(::std::boxed::Box<Wildcard<'tree>>),
}
impl<'tree> ::treesitter_types::FromNode<'tree> for ParametersChildren<'tree> {
    #[allow(clippy::collapsible_else_if)]
    fn from_node(
        node: ::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        match node.kind() {
            "annotated_type" => Ok(Self::AnnotatedType(::std::boxed::Box::new(
                <AnnotatedType as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "applied_constructor_type" => Ok(Self::AppliedConstructorType(::std::boxed::Box::new(
                <AppliedConstructorType as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "compound_type" => Ok(Self::CompoundType(::std::boxed::Box::new(
                <CompoundType as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "function_type" => Ok(Self::FunctionType(::std::boxed::Box::new(
                <FunctionType as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "generic_type" => Ok(Self::GenericType(::std::boxed::Box::new(
                <GenericType as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "infix_type" => Ok(Self::InfixType(::std::boxed::Box::new(
                <InfixType as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "lazy_parameter_type" => Ok(Self::LazyParameterType(::std::boxed::Box::new(
                <LazyParameterType as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "literal_type" => Ok(Self::LiteralType(::std::boxed::Box::new(
                <LiteralType as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "match_type" => Ok(Self::MatchType(::std::boxed::Box::new(
                <MatchType as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "named_tuple_type" => Ok(Self::NamedTupleType(::std::boxed::Box::new(
                <NamedTupleType as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "parameter" => Ok(Self::Parameter(::std::boxed::Box::new(
                <Parameter as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "projected_type" => Ok(Self::ProjectedType(::std::boxed::Box::new(
                <ProjectedType as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "repeated_parameter_type" => Ok(Self::RepeatedParameterType(::std::boxed::Box::new(
                <RepeatedParameterType as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "singleton_type" => Ok(Self::SingletonType(::std::boxed::Box::new(
                <SingletonType as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "stable_type_identifier" => Ok(Self::StableTypeIdentifier(::std::boxed::Box::new(
                <StableTypeIdentifier as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "structural_type" => Ok(Self::StructuralType(::std::boxed::Box::new(
                <StructuralType as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "tuple_type" => Ok(Self::TupleType(::std::boxed::Box::new(
                <TupleType as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "type_identifier" => Ok(Self::TypeIdentifier(::std::boxed::Box::new(
                <TypeIdentifier as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "type_lambda" => Ok(Self::TypeLambda(::std::boxed::Box::new(
                <TypeLambda as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "wildcard" => Ok(Self::Wildcard(::std::boxed::Box::new(
                <Wildcard as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            other => Err(::treesitter_types::ParseError::unexpected_kind(other, node)),
        }
    }
}
impl ::treesitter_types::Spanned for ParametersChildren<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        match self {
            Self::AnnotatedType(inner) => inner.span(),
            Self::AppliedConstructorType(inner) => inner.span(),
            Self::CompoundType(inner) => inner.span(),
            Self::FunctionType(inner) => inner.span(),
            Self::GenericType(inner) => inner.span(),
            Self::InfixType(inner) => inner.span(),
            Self::LazyParameterType(inner) => inner.span(),
            Self::LiteralType(inner) => inner.span(),
            Self::MatchType(inner) => inner.span(),
            Self::NamedTupleType(inner) => inner.span(),
            Self::Parameter(inner) => inner.span(),
            Self::ProjectedType(inner) => inner.span(),
            Self::RepeatedParameterType(inner) => inner.span(),
            Self::SingletonType(inner) => inner.span(),
            Self::StableTypeIdentifier(inner) => inner.span(),
            Self::StructuralType(inner) => inner.span(),
            Self::TupleType(inner) => inner.span(),
            Self::TypeIdentifier(inner) => inner.span(),
            Self::TypeLambda(inner) => inner.span(),
            Self::Wildcard(inner) => inner.span(),
        }
    }
}
#[derive(Debug, Clone)]
pub enum PostfixExpressionChildren<'tree> {
    Block(::std::boxed::Box<Block<'tree>>),
    BooleanLiteral(::std::boxed::Box<BooleanLiteral<'tree>>),
    CallExpression(::std::boxed::Box<CallExpression<'tree>>),
    CaseBlock(::std::boxed::Box<CaseBlock<'tree>>),
    CharacterLiteral(::std::boxed::Box<CharacterLiteral<'tree>>),
    FieldExpression(::std::boxed::Box<FieldExpression<'tree>>),
    FloatingPointLiteral(::std::boxed::Box<FloatingPointLiteral<'tree>>),
    GenericFunction(::std::boxed::Box<GenericFunction<'tree>>),
    Identifier(::std::boxed::Box<Identifier<'tree>>),
    InfixExpression(::std::boxed::Box<InfixExpression<'tree>>),
    InstanceExpression(::std::boxed::Box<InstanceExpression<'tree>>),
    IntegerLiteral(::std::boxed::Box<IntegerLiteral<'tree>>),
    InterpolatedStringExpression(::std::boxed::Box<InterpolatedStringExpression<'tree>>),
    NullLiteral(::std::boxed::Box<NullLiteral<'tree>>),
    OperatorIdentifier(::std::boxed::Box<OperatorIdentifier<'tree>>),
    ParenthesizedExpression(::std::boxed::Box<ParenthesizedExpression<'tree>>),
    PrefixExpression(::std::boxed::Box<PrefixExpression<'tree>>),
    QuoteExpression(::std::boxed::Box<QuoteExpression<'tree>>),
    SpliceExpression(::std::boxed::Box<SpliceExpression<'tree>>),
    String(::std::boxed::Box<String<'tree>>),
    TupleExpression(::std::boxed::Box<TupleExpression<'tree>>),
    Unit(::std::boxed::Box<Unit<'tree>>),
    Wildcard(::std::boxed::Box<Wildcard<'tree>>),
}
impl<'tree> ::treesitter_types::FromNode<'tree> for PostfixExpressionChildren<'tree> {
    #[allow(clippy::collapsible_else_if)]
    fn from_node(
        node: ::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        match node.kind() {
            "block" => Ok(Self::Block(::std::boxed::Box::new(
                <Block as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "boolean_literal" => Ok(Self::BooleanLiteral(::std::boxed::Box::new(
                <BooleanLiteral as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "call_expression" => Ok(Self::CallExpression(::std::boxed::Box::new(
                <CallExpression as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "case_block" => Ok(Self::CaseBlock(::std::boxed::Box::new(
                <CaseBlock as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "character_literal" => Ok(Self::CharacterLiteral(::std::boxed::Box::new(
                <CharacterLiteral as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "field_expression" => Ok(Self::FieldExpression(::std::boxed::Box::new(
                <FieldExpression as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "floating_point_literal" => Ok(Self::FloatingPointLiteral(::std::boxed::Box::new(
                <FloatingPointLiteral as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "generic_function" => Ok(Self::GenericFunction(::std::boxed::Box::new(
                <GenericFunction as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "identifier" => Ok(Self::Identifier(::std::boxed::Box::new(
                <Identifier as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "infix_expression" => Ok(Self::InfixExpression(::std::boxed::Box::new(
                <InfixExpression as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "instance_expression" => Ok(Self::InstanceExpression(::std::boxed::Box::new(
                <InstanceExpression as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "integer_literal" => Ok(Self::IntegerLiteral(::std::boxed::Box::new(
                <IntegerLiteral as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "interpolated_string_expression" => {
                Ok(Self::InterpolatedStringExpression(::std::boxed::Box::new(
                    <InterpolatedStringExpression as ::treesitter_types::FromNode>::from_node(
                        node, src,
                    )?,
                )))
            }
            "null_literal" => Ok(Self::NullLiteral(::std::boxed::Box::new(
                <NullLiteral as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "operator_identifier" => Ok(Self::OperatorIdentifier(::std::boxed::Box::new(
                <OperatorIdentifier as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "parenthesized_expression" => {
                Ok(Self::ParenthesizedExpression(::std::boxed::Box::new(
                    <ParenthesizedExpression as ::treesitter_types::FromNode>::from_node(
                        node, src,
                    )?,
                )))
            }
            "prefix_expression" => Ok(Self::PrefixExpression(::std::boxed::Box::new(
                <PrefixExpression as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "quote_expression" => Ok(Self::QuoteExpression(::std::boxed::Box::new(
                <QuoteExpression as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "splice_expression" => Ok(Self::SpliceExpression(::std::boxed::Box::new(
                <SpliceExpression as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "string" => Ok(Self::String(::std::boxed::Box::new(
                <String as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "tuple_expression" => Ok(Self::TupleExpression(::std::boxed::Box::new(
                <TupleExpression as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "unit" => Ok(Self::Unit(::std::boxed::Box::new(
                <Unit as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "wildcard" => Ok(Self::Wildcard(::std::boxed::Box::new(
                <Wildcard as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            other => Err(::treesitter_types::ParseError::unexpected_kind(other, node)),
        }
    }
}
impl ::treesitter_types::Spanned for PostfixExpressionChildren<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        match self {
            Self::Block(inner) => inner.span(),
            Self::BooleanLiteral(inner) => inner.span(),
            Self::CallExpression(inner) => inner.span(),
            Self::CaseBlock(inner) => inner.span(),
            Self::CharacterLiteral(inner) => inner.span(),
            Self::FieldExpression(inner) => inner.span(),
            Self::FloatingPointLiteral(inner) => inner.span(),
            Self::GenericFunction(inner) => inner.span(),
            Self::Identifier(inner) => inner.span(),
            Self::InfixExpression(inner) => inner.span(),
            Self::InstanceExpression(inner) => inner.span(),
            Self::IntegerLiteral(inner) => inner.span(),
            Self::InterpolatedStringExpression(inner) => inner.span(),
            Self::NullLiteral(inner) => inner.span(),
            Self::OperatorIdentifier(inner) => inner.span(),
            Self::ParenthesizedExpression(inner) => inner.span(),
            Self::PrefixExpression(inner) => inner.span(),
            Self::QuoteExpression(inner) => inner.span(),
            Self::SpliceExpression(inner) => inner.span(),
            Self::String(inner) => inner.span(),
            Self::TupleExpression(inner) => inner.span(),
            Self::Unit(inner) => inner.span(),
            Self::Wildcard(inner) => inner.span(),
        }
    }
}
#[derive(Debug, Clone)]
pub enum PrefixExpressionChildren<'tree> {
    Block(::std::boxed::Box<Block<'tree>>),
    BooleanLiteral(::std::boxed::Box<BooleanLiteral<'tree>>),
    CallExpression(::std::boxed::Box<CallExpression<'tree>>),
    CaseBlock(::std::boxed::Box<CaseBlock<'tree>>),
    CharacterLiteral(::std::boxed::Box<CharacterLiteral<'tree>>),
    FieldExpression(::std::boxed::Box<FieldExpression<'tree>>),
    FloatingPointLiteral(::std::boxed::Box<FloatingPointLiteral<'tree>>),
    GenericFunction(::std::boxed::Box<GenericFunction<'tree>>),
    Identifier(::std::boxed::Box<Identifier<'tree>>),
    InstanceExpression(::std::boxed::Box<InstanceExpression<'tree>>),
    IntegerLiteral(::std::boxed::Box<IntegerLiteral<'tree>>),
    InterpolatedStringExpression(::std::boxed::Box<InterpolatedStringExpression<'tree>>),
    NullLiteral(::std::boxed::Box<NullLiteral<'tree>>),
    OperatorIdentifier(::std::boxed::Box<OperatorIdentifier<'tree>>),
    ParenthesizedExpression(::std::boxed::Box<ParenthesizedExpression<'tree>>),
    QuoteExpression(::std::boxed::Box<QuoteExpression<'tree>>),
    SpliceExpression(::std::boxed::Box<SpliceExpression<'tree>>),
    String(::std::boxed::Box<String<'tree>>),
    TupleExpression(::std::boxed::Box<TupleExpression<'tree>>),
    Unit(::std::boxed::Box<Unit<'tree>>),
    Wildcard(::std::boxed::Box<Wildcard<'tree>>),
}
impl<'tree> ::treesitter_types::FromNode<'tree> for PrefixExpressionChildren<'tree> {
    #[allow(clippy::collapsible_else_if)]
    fn from_node(
        node: ::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        match node.kind() {
            "block" => Ok(Self::Block(::std::boxed::Box::new(
                <Block as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "boolean_literal" => Ok(Self::BooleanLiteral(::std::boxed::Box::new(
                <BooleanLiteral as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "call_expression" => Ok(Self::CallExpression(::std::boxed::Box::new(
                <CallExpression as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "case_block" => Ok(Self::CaseBlock(::std::boxed::Box::new(
                <CaseBlock as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "character_literal" => Ok(Self::CharacterLiteral(::std::boxed::Box::new(
                <CharacterLiteral as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "field_expression" => Ok(Self::FieldExpression(::std::boxed::Box::new(
                <FieldExpression as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "floating_point_literal" => Ok(Self::FloatingPointLiteral(::std::boxed::Box::new(
                <FloatingPointLiteral as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "generic_function" => Ok(Self::GenericFunction(::std::boxed::Box::new(
                <GenericFunction as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "identifier" => Ok(Self::Identifier(::std::boxed::Box::new(
                <Identifier as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "instance_expression" => Ok(Self::InstanceExpression(::std::boxed::Box::new(
                <InstanceExpression as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "integer_literal" => Ok(Self::IntegerLiteral(::std::boxed::Box::new(
                <IntegerLiteral as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "interpolated_string_expression" => {
                Ok(Self::InterpolatedStringExpression(::std::boxed::Box::new(
                    <InterpolatedStringExpression as ::treesitter_types::FromNode>::from_node(
                        node, src,
                    )?,
                )))
            }
            "null_literal" => Ok(Self::NullLiteral(::std::boxed::Box::new(
                <NullLiteral as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "operator_identifier" => Ok(Self::OperatorIdentifier(::std::boxed::Box::new(
                <OperatorIdentifier as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "parenthesized_expression" => {
                Ok(Self::ParenthesizedExpression(::std::boxed::Box::new(
                    <ParenthesizedExpression as ::treesitter_types::FromNode>::from_node(
                        node, src,
                    )?,
                )))
            }
            "quote_expression" => Ok(Self::QuoteExpression(::std::boxed::Box::new(
                <QuoteExpression as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "splice_expression" => Ok(Self::SpliceExpression(::std::boxed::Box::new(
                <SpliceExpression as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "string" => Ok(Self::String(::std::boxed::Box::new(
                <String as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "tuple_expression" => Ok(Self::TupleExpression(::std::boxed::Box::new(
                <TupleExpression as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "unit" => Ok(Self::Unit(::std::boxed::Box::new(
                <Unit as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "wildcard" => Ok(Self::Wildcard(::std::boxed::Box::new(
                <Wildcard as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            other => Err(::treesitter_types::ParseError::unexpected_kind(other, node)),
        }
    }
}
impl ::treesitter_types::Spanned for PrefixExpressionChildren<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        match self {
            Self::Block(inner) => inner.span(),
            Self::BooleanLiteral(inner) => inner.span(),
            Self::CallExpression(inner) => inner.span(),
            Self::CaseBlock(inner) => inner.span(),
            Self::CharacterLiteral(inner) => inner.span(),
            Self::FieldExpression(inner) => inner.span(),
            Self::FloatingPointLiteral(inner) => inner.span(),
            Self::GenericFunction(inner) => inner.span(),
            Self::Identifier(inner) => inner.span(),
            Self::InstanceExpression(inner) => inner.span(),
            Self::IntegerLiteral(inner) => inner.span(),
            Self::InterpolatedStringExpression(inner) => inner.span(),
            Self::NullLiteral(inner) => inner.span(),
            Self::OperatorIdentifier(inner) => inner.span(),
            Self::ParenthesizedExpression(inner) => inner.span(),
            Self::QuoteExpression(inner) => inner.span(),
            Self::SpliceExpression(inner) => inner.span(),
            Self::String(inner) => inner.span(),
            Self::TupleExpression(inner) => inner.span(),
            Self::Unit(inner) => inner.span(),
            Self::Wildcard(inner) => inner.span(),
        }
    }
}
#[derive(Debug, Clone)]
pub enum ProjectedTypeType<'tree> {
    AppliedConstructorType(::std::boxed::Box<AppliedConstructorType<'tree>>),
    GenericType(::std::boxed::Box<GenericType<'tree>>),
    NamedTupleType(::std::boxed::Box<NamedTupleType<'tree>>),
    ProjectedType(::std::boxed::Box<ProjectedType<'tree>>),
    SingletonType(::std::boxed::Box<SingletonType<'tree>>),
    StableTypeIdentifier(::std::boxed::Box<StableTypeIdentifier<'tree>>),
    TupleType(::std::boxed::Box<TupleType<'tree>>),
    TypeIdentifier(::std::boxed::Box<TypeIdentifier<'tree>>),
    Wildcard(::std::boxed::Box<Wildcard<'tree>>),
}
impl<'tree> ::treesitter_types::FromNode<'tree> for ProjectedTypeType<'tree> {
    #[allow(clippy::collapsible_else_if)]
    fn from_node(
        node: ::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        match node.kind() {
            "applied_constructor_type" => Ok(Self::AppliedConstructorType(::std::boxed::Box::new(
                <AppliedConstructorType as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "generic_type" => Ok(Self::GenericType(::std::boxed::Box::new(
                <GenericType as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "named_tuple_type" => Ok(Self::NamedTupleType(::std::boxed::Box::new(
                <NamedTupleType as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "projected_type" => Ok(Self::ProjectedType(::std::boxed::Box::new(
                <ProjectedType as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "singleton_type" => Ok(Self::SingletonType(::std::boxed::Box::new(
                <SingletonType as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "stable_type_identifier" => Ok(Self::StableTypeIdentifier(::std::boxed::Box::new(
                <StableTypeIdentifier as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "tuple_type" => Ok(Self::TupleType(::std::boxed::Box::new(
                <TupleType as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "type_identifier" => Ok(Self::TypeIdentifier(::std::boxed::Box::new(
                <TypeIdentifier as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "wildcard" => Ok(Self::Wildcard(::std::boxed::Box::new(
                <Wildcard as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            other => Err(::treesitter_types::ParseError::unexpected_kind(other, node)),
        }
    }
}
impl ::treesitter_types::Spanned for ProjectedTypeType<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        match self {
            Self::AppliedConstructorType(inner) => inner.span(),
            Self::GenericType(inner) => inner.span(),
            Self::NamedTupleType(inner) => inner.span(),
            Self::ProjectedType(inner) => inner.span(),
            Self::SingletonType(inner) => inner.span(),
            Self::StableTypeIdentifier(inner) => inner.span(),
            Self::TupleType(inner) => inner.span(),
            Self::TypeIdentifier(inner) => inner.span(),
            Self::Wildcard(inner) => inner.span(),
        }
    }
}
#[derive(Debug, Clone)]
pub enum QuoteExpressionChildren<'tree> {
    Definition(::std::boxed::Box<Definition<'tree>>),
    AnnotatedType(::std::boxed::Box<AnnotatedType<'tree>>),
    AppliedConstructorType(::std::boxed::Box<AppliedConstructorType<'tree>>),
    CompoundType(::std::boxed::Box<CompoundType<'tree>>),
    Expression(::std::boxed::Box<Expression<'tree>>),
    FunctionType(::std::boxed::Box<FunctionType<'tree>>),
    GenericType(::std::boxed::Box<GenericType<'tree>>),
    InfixType(::std::boxed::Box<InfixType<'tree>>),
    LiteralType(::std::boxed::Box<LiteralType<'tree>>),
    MatchType(::std::boxed::Box<MatchType<'tree>>),
    NamedTupleType(::std::boxed::Box<NamedTupleType<'tree>>),
    ProjectedType(::std::boxed::Box<ProjectedType<'tree>>),
    SingletonType(::std::boxed::Box<SingletonType<'tree>>),
    StableTypeIdentifier(::std::boxed::Box<StableTypeIdentifier<'tree>>),
    StructuralType(::std::boxed::Box<StructuralType<'tree>>),
    TupleType(::std::boxed::Box<TupleType<'tree>>),
    TypeIdentifier(::std::boxed::Box<TypeIdentifier<'tree>>),
    TypeLambda(::std::boxed::Box<TypeLambda<'tree>>),
}
impl<'tree> ::treesitter_types::FromNode<'tree> for QuoteExpressionChildren<'tree> {
    #[allow(clippy::collapsible_else_if)]
    fn from_node(
        node: ::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        match node.kind() {
            "annotated_type" => Ok(Self::AnnotatedType(::std::boxed::Box::new(
                <AnnotatedType as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "applied_constructor_type" => Ok(Self::AppliedConstructorType(::std::boxed::Box::new(
                <AppliedConstructorType as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "compound_type" => Ok(Self::CompoundType(::std::boxed::Box::new(
                <CompoundType as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "function_type" => Ok(Self::FunctionType(::std::boxed::Box::new(
                <FunctionType as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "generic_type" => Ok(Self::GenericType(::std::boxed::Box::new(
                <GenericType as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "infix_type" => Ok(Self::InfixType(::std::boxed::Box::new(
                <InfixType as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "literal_type" => Ok(Self::LiteralType(::std::boxed::Box::new(
                <LiteralType as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "match_type" => Ok(Self::MatchType(::std::boxed::Box::new(
                <MatchType as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "named_tuple_type" => Ok(Self::NamedTupleType(::std::boxed::Box::new(
                <NamedTupleType as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "projected_type" => Ok(Self::ProjectedType(::std::boxed::Box::new(
                <ProjectedType as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "singleton_type" => Ok(Self::SingletonType(::std::boxed::Box::new(
                <SingletonType as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "stable_type_identifier" => Ok(Self::StableTypeIdentifier(::std::boxed::Box::new(
                <StableTypeIdentifier as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "structural_type" => Ok(Self::StructuralType(::std::boxed::Box::new(
                <StructuralType as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "tuple_type" => Ok(Self::TupleType(::std::boxed::Box::new(
                <TupleType as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "type_identifier" => Ok(Self::TypeIdentifier(::std::boxed::Box::new(
                <TypeIdentifier as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "type_lambda" => Ok(Self::TypeLambda(::std::boxed::Box::new(
                <TypeLambda as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            _other => {
                if let Ok(v) = <Definition as ::treesitter_types::FromNode>::from_node(node, src) {
                    Ok(Self::Definition(::std::boxed::Box::new(v)))
                } else {
                    if let Ok(v) =
                        <Expression as ::treesitter_types::FromNode>::from_node(node, src)
                    {
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
}
impl ::treesitter_types::Spanned for QuoteExpressionChildren<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        match self {
            Self::Definition(inner) => inner.span(),
            Self::AnnotatedType(inner) => inner.span(),
            Self::AppliedConstructorType(inner) => inner.span(),
            Self::CompoundType(inner) => inner.span(),
            Self::Expression(inner) => inner.span(),
            Self::FunctionType(inner) => inner.span(),
            Self::GenericType(inner) => inner.span(),
            Self::InfixType(inner) => inner.span(),
            Self::LiteralType(inner) => inner.span(),
            Self::MatchType(inner) => inner.span(),
            Self::NamedTupleType(inner) => inner.span(),
            Self::ProjectedType(inner) => inner.span(),
            Self::SingletonType(inner) => inner.span(),
            Self::StableTypeIdentifier(inner) => inner.span(),
            Self::StructuralType(inner) => inner.span(),
            Self::TupleType(inner) => inner.span(),
            Self::TypeIdentifier(inner) => inner.span(),
            Self::TypeLambda(inner) => inner.span(),
        }
    }
}
#[derive(Debug, Clone)]
pub enum RefinementChildren<'tree> {
    Definition(::std::boxed::Box<Definition<'tree>>),
    Expression(::std::boxed::Box<Expression<'tree>>),
    SelfType(::std::boxed::Box<SelfType<'tree>>),
}
impl<'tree> ::treesitter_types::FromNode<'tree> for RefinementChildren<'tree> {
    #[allow(clippy::collapsible_else_if)]
    fn from_node(
        node: ::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        match node.kind() {
            "self_type" => Ok(Self::SelfType(::std::boxed::Box::new(
                <SelfType as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            _other => {
                if let Ok(v) = <Definition as ::treesitter_types::FromNode>::from_node(node, src) {
                    Ok(Self::Definition(::std::boxed::Box::new(v)))
                } else {
                    if let Ok(v) =
                        <Expression as ::treesitter_types::FromNode>::from_node(node, src)
                    {
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
}
impl ::treesitter_types::Spanned for RefinementChildren<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        match self {
            Self::Definition(inner) => inner.span(),
            Self::Expression(inner) => inner.span(),
            Self::SelfType(inner) => inner.span(),
        }
    }
}
#[derive(Debug, Clone)]
pub enum RepeatedParameterTypeType<'tree> {
    AnnotatedType(::std::boxed::Box<AnnotatedType<'tree>>),
    AppliedConstructorType(::std::boxed::Box<AppliedConstructorType<'tree>>),
    CompoundType(::std::boxed::Box<CompoundType<'tree>>),
    FunctionType(::std::boxed::Box<FunctionType<'tree>>),
    GenericType(::std::boxed::Box<GenericType<'tree>>),
    InfixType(::std::boxed::Box<InfixType<'tree>>),
    LiteralType(::std::boxed::Box<LiteralType<'tree>>),
    MatchType(::std::boxed::Box<MatchType<'tree>>),
    NamedTupleType(::std::boxed::Box<NamedTupleType<'tree>>),
    ProjectedType(::std::boxed::Box<ProjectedType<'tree>>),
    SingletonType(::std::boxed::Box<SingletonType<'tree>>),
    StableTypeIdentifier(::std::boxed::Box<StableTypeIdentifier<'tree>>),
    StructuralType(::std::boxed::Box<StructuralType<'tree>>),
    TupleType(::std::boxed::Box<TupleType<'tree>>),
    TypeIdentifier(::std::boxed::Box<TypeIdentifier<'tree>>),
    TypeLambda(::std::boxed::Box<TypeLambda<'tree>>),
    Wildcard(::std::boxed::Box<Wildcard<'tree>>),
}
impl<'tree> ::treesitter_types::FromNode<'tree> for RepeatedParameterTypeType<'tree> {
    #[allow(clippy::collapsible_else_if)]
    fn from_node(
        node: ::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        match node.kind() {
            "annotated_type" => Ok(Self::AnnotatedType(::std::boxed::Box::new(
                <AnnotatedType as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "applied_constructor_type" => Ok(Self::AppliedConstructorType(::std::boxed::Box::new(
                <AppliedConstructorType as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "compound_type" => Ok(Self::CompoundType(::std::boxed::Box::new(
                <CompoundType as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "function_type" => Ok(Self::FunctionType(::std::boxed::Box::new(
                <FunctionType as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "generic_type" => Ok(Self::GenericType(::std::boxed::Box::new(
                <GenericType as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "infix_type" => Ok(Self::InfixType(::std::boxed::Box::new(
                <InfixType as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "literal_type" => Ok(Self::LiteralType(::std::boxed::Box::new(
                <LiteralType as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "match_type" => Ok(Self::MatchType(::std::boxed::Box::new(
                <MatchType as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "named_tuple_type" => Ok(Self::NamedTupleType(::std::boxed::Box::new(
                <NamedTupleType as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "projected_type" => Ok(Self::ProjectedType(::std::boxed::Box::new(
                <ProjectedType as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "singleton_type" => Ok(Self::SingletonType(::std::boxed::Box::new(
                <SingletonType as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "stable_type_identifier" => Ok(Self::StableTypeIdentifier(::std::boxed::Box::new(
                <StableTypeIdentifier as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "structural_type" => Ok(Self::StructuralType(::std::boxed::Box::new(
                <StructuralType as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "tuple_type" => Ok(Self::TupleType(::std::boxed::Box::new(
                <TupleType as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "type_identifier" => Ok(Self::TypeIdentifier(::std::boxed::Box::new(
                <TypeIdentifier as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "type_lambda" => Ok(Self::TypeLambda(::std::boxed::Box::new(
                <TypeLambda as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "wildcard" => Ok(Self::Wildcard(::std::boxed::Box::new(
                <Wildcard as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            other => Err(::treesitter_types::ParseError::unexpected_kind(other, node)),
        }
    }
}
impl ::treesitter_types::Spanned for RepeatedParameterTypeType<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        match self {
            Self::AnnotatedType(inner) => inner.span(),
            Self::AppliedConstructorType(inner) => inner.span(),
            Self::CompoundType(inner) => inner.span(),
            Self::FunctionType(inner) => inner.span(),
            Self::GenericType(inner) => inner.span(),
            Self::InfixType(inner) => inner.span(),
            Self::LiteralType(inner) => inner.span(),
            Self::MatchType(inner) => inner.span(),
            Self::NamedTupleType(inner) => inner.span(),
            Self::ProjectedType(inner) => inner.span(),
            Self::SingletonType(inner) => inner.span(),
            Self::StableTypeIdentifier(inner) => inner.span(),
            Self::StructuralType(inner) => inner.span(),
            Self::TupleType(inner) => inner.span(),
            Self::TypeIdentifier(inner) => inner.span(),
            Self::TypeLambda(inner) => inner.span(),
            Self::Wildcard(inner) => inner.span(),
        }
    }
}
#[derive(Debug, Clone)]
pub enum SelfTypeChildren<'tree> {
    AnnotatedType(::std::boxed::Box<AnnotatedType<'tree>>),
    AppliedConstructorType(::std::boxed::Box<AppliedConstructorType<'tree>>),
    CompoundType(::std::boxed::Box<CompoundType<'tree>>),
    FunctionType(::std::boxed::Box<FunctionType<'tree>>),
    GenericType(::std::boxed::Box<GenericType<'tree>>),
    Identifier(::std::boxed::Box<Identifier<'tree>>),
    InfixType(::std::boxed::Box<InfixType<'tree>>),
    LiteralType(::std::boxed::Box<LiteralType<'tree>>),
    MatchType(::std::boxed::Box<MatchType<'tree>>),
    NamedTupleType(::std::boxed::Box<NamedTupleType<'tree>>),
    OperatorIdentifier(::std::boxed::Box<OperatorIdentifier<'tree>>),
    ProjectedType(::std::boxed::Box<ProjectedType<'tree>>),
    SingletonType(::std::boxed::Box<SingletonType<'tree>>),
    StableTypeIdentifier(::std::boxed::Box<StableTypeIdentifier<'tree>>),
    StructuralType(::std::boxed::Box<StructuralType<'tree>>),
    TupleType(::std::boxed::Box<TupleType<'tree>>),
    TypeIdentifier(::std::boxed::Box<TypeIdentifier<'tree>>),
    TypeLambda(::std::boxed::Box<TypeLambda<'tree>>),
    Wildcard(::std::boxed::Box<Wildcard<'tree>>),
}
impl<'tree> ::treesitter_types::FromNode<'tree> for SelfTypeChildren<'tree> {
    #[allow(clippy::collapsible_else_if)]
    fn from_node(
        node: ::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        match node.kind() {
            "annotated_type" => Ok(Self::AnnotatedType(::std::boxed::Box::new(
                <AnnotatedType as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "applied_constructor_type" => Ok(Self::AppliedConstructorType(::std::boxed::Box::new(
                <AppliedConstructorType as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "compound_type" => Ok(Self::CompoundType(::std::boxed::Box::new(
                <CompoundType as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "function_type" => Ok(Self::FunctionType(::std::boxed::Box::new(
                <FunctionType as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "generic_type" => Ok(Self::GenericType(::std::boxed::Box::new(
                <GenericType as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "identifier" => Ok(Self::Identifier(::std::boxed::Box::new(
                <Identifier as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "infix_type" => Ok(Self::InfixType(::std::boxed::Box::new(
                <InfixType as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "literal_type" => Ok(Self::LiteralType(::std::boxed::Box::new(
                <LiteralType as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "match_type" => Ok(Self::MatchType(::std::boxed::Box::new(
                <MatchType as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "named_tuple_type" => Ok(Self::NamedTupleType(::std::boxed::Box::new(
                <NamedTupleType as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "operator_identifier" => Ok(Self::OperatorIdentifier(::std::boxed::Box::new(
                <OperatorIdentifier as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "projected_type" => Ok(Self::ProjectedType(::std::boxed::Box::new(
                <ProjectedType as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "singleton_type" => Ok(Self::SingletonType(::std::boxed::Box::new(
                <SingletonType as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "stable_type_identifier" => Ok(Self::StableTypeIdentifier(::std::boxed::Box::new(
                <StableTypeIdentifier as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "structural_type" => Ok(Self::StructuralType(::std::boxed::Box::new(
                <StructuralType as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "tuple_type" => Ok(Self::TupleType(::std::boxed::Box::new(
                <TupleType as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "type_identifier" => Ok(Self::TypeIdentifier(::std::boxed::Box::new(
                <TypeIdentifier as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "type_lambda" => Ok(Self::TypeLambda(::std::boxed::Box::new(
                <TypeLambda as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "wildcard" => Ok(Self::Wildcard(::std::boxed::Box::new(
                <Wildcard as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            other => Err(::treesitter_types::ParseError::unexpected_kind(other, node)),
        }
    }
}
impl ::treesitter_types::Spanned for SelfTypeChildren<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        match self {
            Self::AnnotatedType(inner) => inner.span(),
            Self::AppliedConstructorType(inner) => inner.span(),
            Self::CompoundType(inner) => inner.span(),
            Self::FunctionType(inner) => inner.span(),
            Self::GenericType(inner) => inner.span(),
            Self::Identifier(inner) => inner.span(),
            Self::InfixType(inner) => inner.span(),
            Self::LiteralType(inner) => inner.span(),
            Self::MatchType(inner) => inner.span(),
            Self::NamedTupleType(inner) => inner.span(),
            Self::OperatorIdentifier(inner) => inner.span(),
            Self::ProjectedType(inner) => inner.span(),
            Self::SingletonType(inner) => inner.span(),
            Self::StableTypeIdentifier(inner) => inner.span(),
            Self::StructuralType(inner) => inner.span(),
            Self::TupleType(inner) => inner.span(),
            Self::TypeIdentifier(inner) => inner.span(),
            Self::TypeLambda(inner) => inner.span(),
            Self::Wildcard(inner) => inner.span(),
        }
    }
}
#[derive(Debug, Clone)]
pub enum SimpleEnumCaseName<'tree> {
    Identifier(::std::boxed::Box<Identifier<'tree>>),
    OperatorIdentifier(::std::boxed::Box<OperatorIdentifier<'tree>>),
}
impl<'tree> ::treesitter_types::FromNode<'tree> for SimpleEnumCaseName<'tree> {
    #[allow(clippy::collapsible_else_if)]
    fn from_node(
        node: ::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        match node.kind() {
            "identifier" => Ok(Self::Identifier(::std::boxed::Box::new(
                <Identifier as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "operator_identifier" => Ok(Self::OperatorIdentifier(::std::boxed::Box::new(
                <OperatorIdentifier as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            other => Err(::treesitter_types::ParseError::unexpected_kind(other, node)),
        }
    }
}
impl ::treesitter_types::Spanned for SimpleEnumCaseName<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        match self {
            Self::Identifier(inner) => inner.span(),
            Self::OperatorIdentifier(inner) => inner.span(),
        }
    }
}
#[derive(Debug, Clone)]
pub enum SingletonTypeChildren<'tree> {
    Identifier(::std::boxed::Box<Identifier<'tree>>),
    OperatorIdentifier(::std::boxed::Box<OperatorIdentifier<'tree>>),
    StableIdentifier(::std::boxed::Box<StableIdentifier<'tree>>),
}
impl<'tree> ::treesitter_types::FromNode<'tree> for SingletonTypeChildren<'tree> {
    #[allow(clippy::collapsible_else_if)]
    fn from_node(
        node: ::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        match node.kind() {
            "identifier" => Ok(Self::Identifier(::std::boxed::Box::new(
                <Identifier as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "operator_identifier" => Ok(Self::OperatorIdentifier(::std::boxed::Box::new(
                <OperatorIdentifier as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "stable_identifier" => Ok(Self::StableIdentifier(::std::boxed::Box::new(
                <StableIdentifier as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            other => Err(::treesitter_types::ParseError::unexpected_kind(other, node)),
        }
    }
}
impl ::treesitter_types::Spanned for SingletonTypeChildren<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        match self {
            Self::Identifier(inner) => inner.span(),
            Self::OperatorIdentifier(inner) => inner.span(),
            Self::StableIdentifier(inner) => inner.span(),
        }
    }
}
#[derive(Debug, Clone)]
pub enum SpliceExpressionChildren<'tree> {
    Definition(::std::boxed::Box<Definition<'tree>>),
    AnnotatedType(::std::boxed::Box<AnnotatedType<'tree>>),
    AppliedConstructorType(::std::boxed::Box<AppliedConstructorType<'tree>>),
    CompoundType(::std::boxed::Box<CompoundType<'tree>>),
    Expression(::std::boxed::Box<Expression<'tree>>),
    FunctionType(::std::boxed::Box<FunctionType<'tree>>),
    GenericType(::std::boxed::Box<GenericType<'tree>>),
    InfixType(::std::boxed::Box<InfixType<'tree>>),
    LiteralType(::std::boxed::Box<LiteralType<'tree>>),
    MatchType(::std::boxed::Box<MatchType<'tree>>),
    NamedTupleType(::std::boxed::Box<NamedTupleType<'tree>>),
    ProjectedType(::std::boxed::Box<ProjectedType<'tree>>),
    SingletonType(::std::boxed::Box<SingletonType<'tree>>),
    StableTypeIdentifier(::std::boxed::Box<StableTypeIdentifier<'tree>>),
    StructuralType(::std::boxed::Box<StructuralType<'tree>>),
    TupleType(::std::boxed::Box<TupleType<'tree>>),
    TypeIdentifier(::std::boxed::Box<TypeIdentifier<'tree>>),
    TypeLambda(::std::boxed::Box<TypeLambda<'tree>>),
}
impl<'tree> ::treesitter_types::FromNode<'tree> for SpliceExpressionChildren<'tree> {
    #[allow(clippy::collapsible_else_if)]
    fn from_node(
        node: ::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        match node.kind() {
            "annotated_type" => Ok(Self::AnnotatedType(::std::boxed::Box::new(
                <AnnotatedType as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "applied_constructor_type" => Ok(Self::AppliedConstructorType(::std::boxed::Box::new(
                <AppliedConstructorType as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "compound_type" => Ok(Self::CompoundType(::std::boxed::Box::new(
                <CompoundType as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "function_type" => Ok(Self::FunctionType(::std::boxed::Box::new(
                <FunctionType as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "generic_type" => Ok(Self::GenericType(::std::boxed::Box::new(
                <GenericType as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "infix_type" => Ok(Self::InfixType(::std::boxed::Box::new(
                <InfixType as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "literal_type" => Ok(Self::LiteralType(::std::boxed::Box::new(
                <LiteralType as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "match_type" => Ok(Self::MatchType(::std::boxed::Box::new(
                <MatchType as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "named_tuple_type" => Ok(Self::NamedTupleType(::std::boxed::Box::new(
                <NamedTupleType as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "projected_type" => Ok(Self::ProjectedType(::std::boxed::Box::new(
                <ProjectedType as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "singleton_type" => Ok(Self::SingletonType(::std::boxed::Box::new(
                <SingletonType as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "stable_type_identifier" => Ok(Self::StableTypeIdentifier(::std::boxed::Box::new(
                <StableTypeIdentifier as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "structural_type" => Ok(Self::StructuralType(::std::boxed::Box::new(
                <StructuralType as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "tuple_type" => Ok(Self::TupleType(::std::boxed::Box::new(
                <TupleType as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "type_identifier" => Ok(Self::TypeIdentifier(::std::boxed::Box::new(
                <TypeIdentifier as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "type_lambda" => Ok(Self::TypeLambda(::std::boxed::Box::new(
                <TypeLambda as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            _other => {
                if let Ok(v) = <Definition as ::treesitter_types::FromNode>::from_node(node, src) {
                    Ok(Self::Definition(::std::boxed::Box::new(v)))
                } else {
                    if let Ok(v) =
                        <Expression as ::treesitter_types::FromNode>::from_node(node, src)
                    {
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
}
impl ::treesitter_types::Spanned for SpliceExpressionChildren<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        match self {
            Self::Definition(inner) => inner.span(),
            Self::AnnotatedType(inner) => inner.span(),
            Self::AppliedConstructorType(inner) => inner.span(),
            Self::CompoundType(inner) => inner.span(),
            Self::Expression(inner) => inner.span(),
            Self::FunctionType(inner) => inner.span(),
            Self::GenericType(inner) => inner.span(),
            Self::InfixType(inner) => inner.span(),
            Self::LiteralType(inner) => inner.span(),
            Self::MatchType(inner) => inner.span(),
            Self::NamedTupleType(inner) => inner.span(),
            Self::ProjectedType(inner) => inner.span(),
            Self::SingletonType(inner) => inner.span(),
            Self::StableTypeIdentifier(inner) => inner.span(),
            Self::StructuralType(inner) => inner.span(),
            Self::TupleType(inner) => inner.span(),
            Self::TypeIdentifier(inner) => inner.span(),
            Self::TypeLambda(inner) => inner.span(),
        }
    }
}
#[derive(Debug, Clone)]
pub enum StableIdentifierChildren<'tree> {
    Identifier(::std::boxed::Box<Identifier<'tree>>),
    OperatorIdentifier(::std::boxed::Box<OperatorIdentifier<'tree>>),
    StableIdentifier(::std::boxed::Box<StableIdentifier<'tree>>),
}
impl<'tree> ::treesitter_types::FromNode<'tree> for StableIdentifierChildren<'tree> {
    #[allow(clippy::collapsible_else_if)]
    fn from_node(
        node: ::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        match node.kind() {
            "identifier" => Ok(Self::Identifier(::std::boxed::Box::new(
                <Identifier as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "operator_identifier" => Ok(Self::OperatorIdentifier(::std::boxed::Box::new(
                <OperatorIdentifier as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "stable_identifier" => Ok(Self::StableIdentifier(::std::boxed::Box::new(
                <StableIdentifier as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            other => Err(::treesitter_types::ParseError::unexpected_kind(other, node)),
        }
    }
}
impl ::treesitter_types::Spanned for StableIdentifierChildren<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        match self {
            Self::Identifier(inner) => inner.span(),
            Self::OperatorIdentifier(inner) => inner.span(),
            Self::StableIdentifier(inner) => inner.span(),
        }
    }
}
#[derive(Debug, Clone)]
pub enum StableTypeIdentifierChildren<'tree> {
    Identifier(::std::boxed::Box<Identifier<'tree>>),
    OperatorIdentifier(::std::boxed::Box<OperatorIdentifier<'tree>>),
    StableIdentifier(::std::boxed::Box<StableIdentifier<'tree>>),
    TypeIdentifier(::std::boxed::Box<TypeIdentifier<'tree>>),
}
impl<'tree> ::treesitter_types::FromNode<'tree> for StableTypeIdentifierChildren<'tree> {
    #[allow(clippy::collapsible_else_if)]
    fn from_node(
        node: ::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        match node.kind() {
            "identifier" => Ok(Self::Identifier(::std::boxed::Box::new(
                <Identifier as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "operator_identifier" => Ok(Self::OperatorIdentifier(::std::boxed::Box::new(
                <OperatorIdentifier as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "stable_identifier" => Ok(Self::StableIdentifier(::std::boxed::Box::new(
                <StableIdentifier as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "type_identifier" => Ok(Self::TypeIdentifier(::std::boxed::Box::new(
                <TypeIdentifier as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            other => Err(::treesitter_types::ParseError::unexpected_kind(other, node)),
        }
    }
}
impl ::treesitter_types::Spanned for StableTypeIdentifierChildren<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        match self {
            Self::Identifier(inner) => inner.span(),
            Self::OperatorIdentifier(inner) => inner.span(),
            Self::StableIdentifier(inner) => inner.span(),
            Self::TypeIdentifier(inner) => inner.span(),
        }
    }
}
#[derive(Debug, Clone)]
pub enum StructuralTypeChildren<'tree> {
    Definition(::std::boxed::Box<Definition<'tree>>),
    Expression(::std::boxed::Box<Expression<'tree>>),
    SelfType(::std::boxed::Box<SelfType<'tree>>),
}
impl<'tree> ::treesitter_types::FromNode<'tree> for StructuralTypeChildren<'tree> {
    #[allow(clippy::collapsible_else_if)]
    fn from_node(
        node: ::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        match node.kind() {
            "self_type" => Ok(Self::SelfType(::std::boxed::Box::new(
                <SelfType as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            _other => {
                if let Ok(v) = <Definition as ::treesitter_types::FromNode>::from_node(node, src) {
                    Ok(Self::Definition(::std::boxed::Box::new(v)))
                } else {
                    if let Ok(v) =
                        <Expression as ::treesitter_types::FromNode>::from_node(node, src)
                    {
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
}
impl ::treesitter_types::Spanned for StructuralTypeChildren<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        match self {
            Self::Definition(inner) => inner.span(),
            Self::Expression(inner) => inner.span(),
            Self::SelfType(inner) => inner.span(),
        }
    }
}
#[derive(Debug, Clone)]
pub enum TemplateBodyChildren<'tree> {
    Definition(::std::boxed::Box<Definition<'tree>>),
    Expression(::std::boxed::Box<Expression<'tree>>),
    SelfType(::std::boxed::Box<SelfType<'tree>>),
}
impl<'tree> ::treesitter_types::FromNode<'tree> for TemplateBodyChildren<'tree> {
    #[allow(clippy::collapsible_else_if)]
    fn from_node(
        node: ::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        match node.kind() {
            "self_type" => Ok(Self::SelfType(::std::boxed::Box::new(
                <SelfType as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            _other => {
                if let Ok(v) = <Definition as ::treesitter_types::FromNode>::from_node(node, src) {
                    Ok(Self::Definition(::std::boxed::Box::new(v)))
                } else {
                    if let Ok(v) =
                        <Expression as ::treesitter_types::FromNode>::from_node(node, src)
                    {
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
}
impl ::treesitter_types::Spanned for TemplateBodyChildren<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        match self {
            Self::Definition(inner) => inner.span(),
            Self::Expression(inner) => inner.span(),
            Self::SelfType(inner) => inner.span(),
        }
    }
}
#[derive(Debug, Clone)]
pub enum TraitDefinitionName<'tree> {
    Identifier(::std::boxed::Box<Identifier<'tree>>),
    OperatorIdentifier(::std::boxed::Box<OperatorIdentifier<'tree>>),
}
impl<'tree> ::treesitter_types::FromNode<'tree> for TraitDefinitionName<'tree> {
    #[allow(clippy::collapsible_else_if)]
    fn from_node(
        node: ::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        match node.kind() {
            "identifier" => Ok(Self::Identifier(::std::boxed::Box::new(
                <Identifier as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "operator_identifier" => Ok(Self::OperatorIdentifier(::std::boxed::Box::new(
                <OperatorIdentifier as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            other => Err(::treesitter_types::ParseError::unexpected_kind(other, node)),
        }
    }
}
impl ::treesitter_types::Spanned for TraitDefinitionName<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        match self {
            Self::Identifier(inner) => inner.span(),
            Self::OperatorIdentifier(inner) => inner.span(),
        }
    }
}
#[derive(Debug, Clone)]
pub enum TraitDefinitionChildren<'tree> {
    AccessModifier(::std::boxed::Box<AccessModifier<'tree>>),
    Annotation(::std::boxed::Box<Annotation<'tree>>),
    Modifiers(::std::boxed::Box<Modifiers<'tree>>),
}
impl<'tree> ::treesitter_types::FromNode<'tree> for TraitDefinitionChildren<'tree> {
    #[allow(clippy::collapsible_else_if)]
    fn from_node(
        node: ::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        match node.kind() {
            "access_modifier" => Ok(Self::AccessModifier(::std::boxed::Box::new(
                <AccessModifier as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "annotation" => Ok(Self::Annotation(::std::boxed::Box::new(
                <Annotation as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "modifiers" => Ok(Self::Modifiers(::std::boxed::Box::new(
                <Modifiers as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            other => Err(::treesitter_types::ParseError::unexpected_kind(other, node)),
        }
    }
}
impl ::treesitter_types::Spanned for TraitDefinitionChildren<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        match self {
            Self::AccessModifier(inner) => inner.span(),
            Self::Annotation(inner) => inner.span(),
            Self::Modifiers(inner) => inner.span(),
        }
    }
}
#[derive(Debug, Clone)]
pub enum TryExpressionBody<'tree> {
    Expression(::std::boxed::Box<Expression<'tree>>),
    IndentedBlock(::std::boxed::Box<IndentedBlock<'tree>>),
    IndentedCases(::std::boxed::Box<IndentedCases<'tree>>),
}
impl<'tree> ::treesitter_types::FromNode<'tree> for TryExpressionBody<'tree> {
    #[allow(clippy::collapsible_else_if)]
    fn from_node(
        node: ::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        match node.kind() {
            "indented_block" => Ok(Self::IndentedBlock(::std::boxed::Box::new(
                <IndentedBlock as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "indented_cases" => Ok(Self::IndentedCases(::std::boxed::Box::new(
                <IndentedCases as ::treesitter_types::FromNode>::from_node(node, src)?,
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
impl ::treesitter_types::Spanned for TryExpressionBody<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        match self {
            Self::Expression(inner) => inner.span(),
            Self::IndentedBlock(inner) => inner.span(),
            Self::IndentedCases(inner) => inner.span(),
        }
    }
}
#[derive(Debug, Clone)]
pub enum TryExpressionChildren<'tree> {
    CatchClause(::std::boxed::Box<CatchClause<'tree>>),
    FinallyClause(::std::boxed::Box<FinallyClause<'tree>>),
}
impl<'tree> ::treesitter_types::FromNode<'tree> for TryExpressionChildren<'tree> {
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
impl ::treesitter_types::Spanned for TryExpressionChildren<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        match self {
            Self::CatchClause(inner) => inner.span(),
            Self::FinallyClause(inner) => inner.span(),
        }
    }
}
#[derive(Debug, Clone)]
pub enum TupleTypeChildren<'tree> {
    AnnotatedType(::std::boxed::Box<AnnotatedType<'tree>>),
    AppliedConstructorType(::std::boxed::Box<AppliedConstructorType<'tree>>),
    CompoundType(::std::boxed::Box<CompoundType<'tree>>),
    FunctionType(::std::boxed::Box<FunctionType<'tree>>),
    GenericType(::std::boxed::Box<GenericType<'tree>>),
    InfixType(::std::boxed::Box<InfixType<'tree>>),
    LiteralType(::std::boxed::Box<LiteralType<'tree>>),
    MatchType(::std::boxed::Box<MatchType<'tree>>),
    NamedTupleType(::std::boxed::Box<NamedTupleType<'tree>>),
    ProjectedType(::std::boxed::Box<ProjectedType<'tree>>),
    SingletonType(::std::boxed::Box<SingletonType<'tree>>),
    StableTypeIdentifier(::std::boxed::Box<StableTypeIdentifier<'tree>>),
    StructuralType(::std::boxed::Box<StructuralType<'tree>>),
    TupleType(::std::boxed::Box<TupleType<'tree>>),
    TypeIdentifier(::std::boxed::Box<TypeIdentifier<'tree>>),
    TypeLambda(::std::boxed::Box<TypeLambda<'tree>>),
    Wildcard(::std::boxed::Box<Wildcard<'tree>>),
}
impl<'tree> ::treesitter_types::FromNode<'tree> for TupleTypeChildren<'tree> {
    #[allow(clippy::collapsible_else_if)]
    fn from_node(
        node: ::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        match node.kind() {
            "annotated_type" => Ok(Self::AnnotatedType(::std::boxed::Box::new(
                <AnnotatedType as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "applied_constructor_type" => Ok(Self::AppliedConstructorType(::std::boxed::Box::new(
                <AppliedConstructorType as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "compound_type" => Ok(Self::CompoundType(::std::boxed::Box::new(
                <CompoundType as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "function_type" => Ok(Self::FunctionType(::std::boxed::Box::new(
                <FunctionType as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "generic_type" => Ok(Self::GenericType(::std::boxed::Box::new(
                <GenericType as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "infix_type" => Ok(Self::InfixType(::std::boxed::Box::new(
                <InfixType as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "literal_type" => Ok(Self::LiteralType(::std::boxed::Box::new(
                <LiteralType as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "match_type" => Ok(Self::MatchType(::std::boxed::Box::new(
                <MatchType as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "named_tuple_type" => Ok(Self::NamedTupleType(::std::boxed::Box::new(
                <NamedTupleType as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "projected_type" => Ok(Self::ProjectedType(::std::boxed::Box::new(
                <ProjectedType as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "singleton_type" => Ok(Self::SingletonType(::std::boxed::Box::new(
                <SingletonType as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "stable_type_identifier" => Ok(Self::StableTypeIdentifier(::std::boxed::Box::new(
                <StableTypeIdentifier as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "structural_type" => Ok(Self::StructuralType(::std::boxed::Box::new(
                <StructuralType as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "tuple_type" => Ok(Self::TupleType(::std::boxed::Box::new(
                <TupleType as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "type_identifier" => Ok(Self::TypeIdentifier(::std::boxed::Box::new(
                <TypeIdentifier as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "type_lambda" => Ok(Self::TypeLambda(::std::boxed::Box::new(
                <TypeLambda as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "wildcard" => Ok(Self::Wildcard(::std::boxed::Box::new(
                <Wildcard as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            other => Err(::treesitter_types::ParseError::unexpected_kind(other, node)),
        }
    }
}
impl ::treesitter_types::Spanned for TupleTypeChildren<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        match self {
            Self::AnnotatedType(inner) => inner.span(),
            Self::AppliedConstructorType(inner) => inner.span(),
            Self::CompoundType(inner) => inner.span(),
            Self::FunctionType(inner) => inner.span(),
            Self::GenericType(inner) => inner.span(),
            Self::InfixType(inner) => inner.span(),
            Self::LiteralType(inner) => inner.span(),
            Self::MatchType(inner) => inner.span(),
            Self::NamedTupleType(inner) => inner.span(),
            Self::ProjectedType(inner) => inner.span(),
            Self::SingletonType(inner) => inner.span(),
            Self::StableTypeIdentifier(inner) => inner.span(),
            Self::StructuralType(inner) => inner.span(),
            Self::TupleType(inner) => inner.span(),
            Self::TypeIdentifier(inner) => inner.span(),
            Self::TypeLambda(inner) => inner.span(),
            Self::Wildcard(inner) => inner.span(),
        }
    }
}
#[derive(Debug, Clone)]
pub enum TypeArgumentsChildren<'tree> {
    AnnotatedType(::std::boxed::Box<AnnotatedType<'tree>>),
    AppliedConstructorType(::std::boxed::Box<AppliedConstructorType<'tree>>),
    CompoundType(::std::boxed::Box<CompoundType<'tree>>),
    FunctionType(::std::boxed::Box<FunctionType<'tree>>),
    GenericType(::std::boxed::Box<GenericType<'tree>>),
    InfixType(::std::boxed::Box<InfixType<'tree>>),
    LiteralType(::std::boxed::Box<LiteralType<'tree>>),
    MatchType(::std::boxed::Box<MatchType<'tree>>),
    NamedTupleType(::std::boxed::Box<NamedTupleType<'tree>>),
    ProjectedType(::std::boxed::Box<ProjectedType<'tree>>),
    SingletonType(::std::boxed::Box<SingletonType<'tree>>),
    StableTypeIdentifier(::std::boxed::Box<StableTypeIdentifier<'tree>>),
    StructuralType(::std::boxed::Box<StructuralType<'tree>>),
    TupleType(::std::boxed::Box<TupleType<'tree>>),
    TypeIdentifier(::std::boxed::Box<TypeIdentifier<'tree>>),
    TypeLambda(::std::boxed::Box<TypeLambda<'tree>>),
    Wildcard(::std::boxed::Box<Wildcard<'tree>>),
}
impl<'tree> ::treesitter_types::FromNode<'tree> for TypeArgumentsChildren<'tree> {
    #[allow(clippy::collapsible_else_if)]
    fn from_node(
        node: ::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        match node.kind() {
            "annotated_type" => Ok(Self::AnnotatedType(::std::boxed::Box::new(
                <AnnotatedType as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "applied_constructor_type" => Ok(Self::AppliedConstructorType(::std::boxed::Box::new(
                <AppliedConstructorType as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "compound_type" => Ok(Self::CompoundType(::std::boxed::Box::new(
                <CompoundType as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "function_type" => Ok(Self::FunctionType(::std::boxed::Box::new(
                <FunctionType as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "generic_type" => Ok(Self::GenericType(::std::boxed::Box::new(
                <GenericType as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "infix_type" => Ok(Self::InfixType(::std::boxed::Box::new(
                <InfixType as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "literal_type" => Ok(Self::LiteralType(::std::boxed::Box::new(
                <LiteralType as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "match_type" => Ok(Self::MatchType(::std::boxed::Box::new(
                <MatchType as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "named_tuple_type" => Ok(Self::NamedTupleType(::std::boxed::Box::new(
                <NamedTupleType as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "projected_type" => Ok(Self::ProjectedType(::std::boxed::Box::new(
                <ProjectedType as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "singleton_type" => Ok(Self::SingletonType(::std::boxed::Box::new(
                <SingletonType as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "stable_type_identifier" => Ok(Self::StableTypeIdentifier(::std::boxed::Box::new(
                <StableTypeIdentifier as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "structural_type" => Ok(Self::StructuralType(::std::boxed::Box::new(
                <StructuralType as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "tuple_type" => Ok(Self::TupleType(::std::boxed::Box::new(
                <TupleType as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "type_identifier" => Ok(Self::TypeIdentifier(::std::boxed::Box::new(
                <TypeIdentifier as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "type_lambda" => Ok(Self::TypeLambda(::std::boxed::Box::new(
                <TypeLambda as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "wildcard" => Ok(Self::Wildcard(::std::boxed::Box::new(
                <Wildcard as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            other => Err(::treesitter_types::ParseError::unexpected_kind(other, node)),
        }
    }
}
impl ::treesitter_types::Spanned for TypeArgumentsChildren<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        match self {
            Self::AnnotatedType(inner) => inner.span(),
            Self::AppliedConstructorType(inner) => inner.span(),
            Self::CompoundType(inner) => inner.span(),
            Self::FunctionType(inner) => inner.span(),
            Self::GenericType(inner) => inner.span(),
            Self::InfixType(inner) => inner.span(),
            Self::LiteralType(inner) => inner.span(),
            Self::MatchType(inner) => inner.span(),
            Self::NamedTupleType(inner) => inner.span(),
            Self::ProjectedType(inner) => inner.span(),
            Self::SingletonType(inner) => inner.span(),
            Self::StableTypeIdentifier(inner) => inner.span(),
            Self::StructuralType(inner) => inner.span(),
            Self::TupleType(inner) => inner.span(),
            Self::TypeIdentifier(inner) => inner.span(),
            Self::TypeLambda(inner) => inner.span(),
            Self::Wildcard(inner) => inner.span(),
        }
    }
}
#[derive(Debug, Clone)]
pub enum TypeCaseClauseBody<'tree> {
    FatArrow(::treesitter_types::Span),
    QuestionEqGt(::treesitter_types::Span),
    AnnotatedType(::std::boxed::Box<AnnotatedType<'tree>>),
    AppliedConstructorType(::std::boxed::Box<AppliedConstructorType<'tree>>),
    CompoundType(::std::boxed::Box<CompoundType<'tree>>),
    FunctionType(::std::boxed::Box<FunctionType<'tree>>),
    GenericType(::std::boxed::Box<GenericType<'tree>>),
    InfixType(::std::boxed::Box<InfixType<'tree>>),
    LiteralType(::std::boxed::Box<LiteralType<'tree>>),
    MatchType(::std::boxed::Box<MatchType<'tree>>),
    NamedTupleType(::std::boxed::Box<NamedTupleType<'tree>>),
    ProjectedType(::std::boxed::Box<ProjectedType<'tree>>),
    SingletonType(::std::boxed::Box<SingletonType<'tree>>),
    StableTypeIdentifier(::std::boxed::Box<StableTypeIdentifier<'tree>>),
    StructuralType(::std::boxed::Box<StructuralType<'tree>>),
    TupleType(::std::boxed::Box<TupleType<'tree>>),
    TypeIdentifier(::std::boxed::Box<TypeIdentifier<'tree>>),
    TypeLambda(::std::boxed::Box<TypeLambda<'tree>>),
    Wildcard(::std::boxed::Box<Wildcard<'tree>>),
}
impl<'tree> ::treesitter_types::FromNode<'tree> for TypeCaseClauseBody<'tree> {
    #[allow(clippy::collapsible_else_if)]
    fn from_node(
        node: ::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        match node.kind() {
            "=>" => Ok(Self::FatArrow(::treesitter_types::Span::from(node))),
            "?=>" => Ok(Self::QuestionEqGt(::treesitter_types::Span::from(node))),
            "annotated_type" => Ok(Self::AnnotatedType(::std::boxed::Box::new(
                <AnnotatedType as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "applied_constructor_type" => Ok(Self::AppliedConstructorType(::std::boxed::Box::new(
                <AppliedConstructorType as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "compound_type" => Ok(Self::CompoundType(::std::boxed::Box::new(
                <CompoundType as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "function_type" => Ok(Self::FunctionType(::std::boxed::Box::new(
                <FunctionType as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "generic_type" => Ok(Self::GenericType(::std::boxed::Box::new(
                <GenericType as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "infix_type" => Ok(Self::InfixType(::std::boxed::Box::new(
                <InfixType as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "literal_type" => Ok(Self::LiteralType(::std::boxed::Box::new(
                <LiteralType as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "match_type" => Ok(Self::MatchType(::std::boxed::Box::new(
                <MatchType as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "named_tuple_type" => Ok(Self::NamedTupleType(::std::boxed::Box::new(
                <NamedTupleType as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "projected_type" => Ok(Self::ProjectedType(::std::boxed::Box::new(
                <ProjectedType as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "singleton_type" => Ok(Self::SingletonType(::std::boxed::Box::new(
                <SingletonType as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "stable_type_identifier" => Ok(Self::StableTypeIdentifier(::std::boxed::Box::new(
                <StableTypeIdentifier as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "structural_type" => Ok(Self::StructuralType(::std::boxed::Box::new(
                <StructuralType as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "tuple_type" => Ok(Self::TupleType(::std::boxed::Box::new(
                <TupleType as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "type_identifier" => Ok(Self::TypeIdentifier(::std::boxed::Box::new(
                <TypeIdentifier as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "type_lambda" => Ok(Self::TypeLambda(::std::boxed::Box::new(
                <TypeLambda as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "wildcard" => Ok(Self::Wildcard(::std::boxed::Box::new(
                <Wildcard as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            other => Err(::treesitter_types::ParseError::unexpected_kind(other, node)),
        }
    }
}
impl ::treesitter_types::Spanned for TypeCaseClauseBody<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        match self {
            Self::FatArrow(span) => *span,
            Self::QuestionEqGt(span) => *span,
            Self::AnnotatedType(inner) => inner.span(),
            Self::AppliedConstructorType(inner) => inner.span(),
            Self::CompoundType(inner) => inner.span(),
            Self::FunctionType(inner) => inner.span(),
            Self::GenericType(inner) => inner.span(),
            Self::InfixType(inner) => inner.span(),
            Self::LiteralType(inner) => inner.span(),
            Self::MatchType(inner) => inner.span(),
            Self::NamedTupleType(inner) => inner.span(),
            Self::ProjectedType(inner) => inner.span(),
            Self::SingletonType(inner) => inner.span(),
            Self::StableTypeIdentifier(inner) => inner.span(),
            Self::StructuralType(inner) => inner.span(),
            Self::TupleType(inner) => inner.span(),
            Self::TypeIdentifier(inner) => inner.span(),
            Self::TypeLambda(inner) => inner.span(),
            Self::Wildcard(inner) => inner.span(),
        }
    }
}
#[derive(Debug, Clone)]
pub enum TypeCaseClauseReturnType<'tree> {
    AnnotatedType(::std::boxed::Box<AnnotatedType<'tree>>),
    AppliedConstructorType(::std::boxed::Box<AppliedConstructorType<'tree>>),
    CompoundType(::std::boxed::Box<CompoundType<'tree>>),
    FunctionType(::std::boxed::Box<FunctionType<'tree>>),
    GenericType(::std::boxed::Box<GenericType<'tree>>),
    InfixType(::std::boxed::Box<InfixType<'tree>>),
    LiteralType(::std::boxed::Box<LiteralType<'tree>>),
    MatchType(::std::boxed::Box<MatchType<'tree>>),
    NamedTupleType(::std::boxed::Box<NamedTupleType<'tree>>),
    ProjectedType(::std::boxed::Box<ProjectedType<'tree>>),
    SingletonType(::std::boxed::Box<SingletonType<'tree>>),
    StableTypeIdentifier(::std::boxed::Box<StableTypeIdentifier<'tree>>),
    StructuralType(::std::boxed::Box<StructuralType<'tree>>),
    TupleType(::std::boxed::Box<TupleType<'tree>>),
    TypeIdentifier(::std::boxed::Box<TypeIdentifier<'tree>>),
    TypeLambda(::std::boxed::Box<TypeLambda<'tree>>),
    Wildcard(::std::boxed::Box<Wildcard<'tree>>),
}
impl<'tree> ::treesitter_types::FromNode<'tree> for TypeCaseClauseReturnType<'tree> {
    #[allow(clippy::collapsible_else_if)]
    fn from_node(
        node: ::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        match node.kind() {
            "annotated_type" => Ok(Self::AnnotatedType(::std::boxed::Box::new(
                <AnnotatedType as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "applied_constructor_type" => Ok(Self::AppliedConstructorType(::std::boxed::Box::new(
                <AppliedConstructorType as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "compound_type" => Ok(Self::CompoundType(::std::boxed::Box::new(
                <CompoundType as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "function_type" => Ok(Self::FunctionType(::std::boxed::Box::new(
                <FunctionType as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "generic_type" => Ok(Self::GenericType(::std::boxed::Box::new(
                <GenericType as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "infix_type" => Ok(Self::InfixType(::std::boxed::Box::new(
                <InfixType as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "literal_type" => Ok(Self::LiteralType(::std::boxed::Box::new(
                <LiteralType as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "match_type" => Ok(Self::MatchType(::std::boxed::Box::new(
                <MatchType as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "named_tuple_type" => Ok(Self::NamedTupleType(::std::boxed::Box::new(
                <NamedTupleType as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "projected_type" => Ok(Self::ProjectedType(::std::boxed::Box::new(
                <ProjectedType as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "singleton_type" => Ok(Self::SingletonType(::std::boxed::Box::new(
                <SingletonType as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "stable_type_identifier" => Ok(Self::StableTypeIdentifier(::std::boxed::Box::new(
                <StableTypeIdentifier as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "structural_type" => Ok(Self::StructuralType(::std::boxed::Box::new(
                <StructuralType as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "tuple_type" => Ok(Self::TupleType(::std::boxed::Box::new(
                <TupleType as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "type_identifier" => Ok(Self::TypeIdentifier(::std::boxed::Box::new(
                <TypeIdentifier as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "type_lambda" => Ok(Self::TypeLambda(::std::boxed::Box::new(
                <TypeLambda as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "wildcard" => Ok(Self::Wildcard(::std::boxed::Box::new(
                <Wildcard as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            other => Err(::treesitter_types::ParseError::unexpected_kind(other, node)),
        }
    }
}
impl ::treesitter_types::Spanned for TypeCaseClauseReturnType<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        match self {
            Self::AnnotatedType(inner) => inner.span(),
            Self::AppliedConstructorType(inner) => inner.span(),
            Self::CompoundType(inner) => inner.span(),
            Self::FunctionType(inner) => inner.span(),
            Self::GenericType(inner) => inner.span(),
            Self::InfixType(inner) => inner.span(),
            Self::LiteralType(inner) => inner.span(),
            Self::MatchType(inner) => inner.span(),
            Self::NamedTupleType(inner) => inner.span(),
            Self::ProjectedType(inner) => inner.span(),
            Self::SingletonType(inner) => inner.span(),
            Self::StableTypeIdentifier(inner) => inner.span(),
            Self::StructuralType(inner) => inner.span(),
            Self::TupleType(inner) => inner.span(),
            Self::TypeIdentifier(inner) => inner.span(),
            Self::TypeLambda(inner) => inner.span(),
            Self::Wildcard(inner) => inner.span(),
        }
    }
}
#[derive(Debug, Clone)]
pub enum TypeCaseClauseChildren<'tree> {
    AnnotatedType(::std::boxed::Box<AnnotatedType<'tree>>),
    AppliedConstructorType(::std::boxed::Box<AppliedConstructorType<'tree>>),
    CompoundType(::std::boxed::Box<CompoundType<'tree>>),
    GenericType(::std::boxed::Box<GenericType<'tree>>),
    InfixType(::std::boxed::Box<InfixType<'tree>>),
    LiteralType(::std::boxed::Box<LiteralType<'tree>>),
    NamedTupleType(::std::boxed::Box<NamedTupleType<'tree>>),
    ProjectedType(::std::boxed::Box<ProjectedType<'tree>>),
    SingletonType(::std::boxed::Box<SingletonType<'tree>>),
    StableTypeIdentifier(::std::boxed::Box<StableTypeIdentifier<'tree>>),
    TupleType(::std::boxed::Box<TupleType<'tree>>),
    TypeIdentifier(::std::boxed::Box<TypeIdentifier<'tree>>),
    Wildcard(::std::boxed::Box<Wildcard<'tree>>),
}
impl<'tree> ::treesitter_types::FromNode<'tree> for TypeCaseClauseChildren<'tree> {
    #[allow(clippy::collapsible_else_if)]
    fn from_node(
        node: ::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        match node.kind() {
            "annotated_type" => Ok(Self::AnnotatedType(::std::boxed::Box::new(
                <AnnotatedType as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "applied_constructor_type" => Ok(Self::AppliedConstructorType(::std::boxed::Box::new(
                <AppliedConstructorType as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "compound_type" => Ok(Self::CompoundType(::std::boxed::Box::new(
                <CompoundType as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "generic_type" => Ok(Self::GenericType(::std::boxed::Box::new(
                <GenericType as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "infix_type" => Ok(Self::InfixType(::std::boxed::Box::new(
                <InfixType as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "literal_type" => Ok(Self::LiteralType(::std::boxed::Box::new(
                <LiteralType as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "named_tuple_type" => Ok(Self::NamedTupleType(::std::boxed::Box::new(
                <NamedTupleType as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "projected_type" => Ok(Self::ProjectedType(::std::boxed::Box::new(
                <ProjectedType as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "singleton_type" => Ok(Self::SingletonType(::std::boxed::Box::new(
                <SingletonType as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "stable_type_identifier" => Ok(Self::StableTypeIdentifier(::std::boxed::Box::new(
                <StableTypeIdentifier as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "tuple_type" => Ok(Self::TupleType(::std::boxed::Box::new(
                <TupleType as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "type_identifier" => Ok(Self::TypeIdentifier(::std::boxed::Box::new(
                <TypeIdentifier as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "wildcard" => Ok(Self::Wildcard(::std::boxed::Box::new(
                <Wildcard as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            other => Err(::treesitter_types::ParseError::unexpected_kind(other, node)),
        }
    }
}
impl ::treesitter_types::Spanned for TypeCaseClauseChildren<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        match self {
            Self::AnnotatedType(inner) => inner.span(),
            Self::AppliedConstructorType(inner) => inner.span(),
            Self::CompoundType(inner) => inner.span(),
            Self::GenericType(inner) => inner.span(),
            Self::InfixType(inner) => inner.span(),
            Self::LiteralType(inner) => inner.span(),
            Self::NamedTupleType(inner) => inner.span(),
            Self::ProjectedType(inner) => inner.span(),
            Self::SingletonType(inner) => inner.span(),
            Self::StableTypeIdentifier(inner) => inner.span(),
            Self::TupleType(inner) => inner.span(),
            Self::TypeIdentifier(inner) => inner.span(),
            Self::Wildcard(inner) => inner.span(),
        }
    }
}
#[derive(Debug, Clone)]
pub enum TypeDefinitionBound<'tree> {
    Comma(::treesitter_types::Span),
    Colon(::treesitter_types::Span),
    ContextBound(::std::boxed::Box<ContextBound<'tree>>),
    LowerBound(::std::boxed::Box<LowerBound<'tree>>),
    UpperBound(::std::boxed::Box<UpperBound<'tree>>),
    LBrace(::treesitter_types::Span),
    RBrace(::treesitter_types::Span),
}
impl<'tree> ::treesitter_types::FromNode<'tree> for TypeDefinitionBound<'tree> {
    #[allow(clippy::collapsible_else_if)]
    fn from_node(
        node: ::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        match node.kind() {
            "," => Ok(Self::Comma(::treesitter_types::Span::from(node))),
            ":" => Ok(Self::Colon(::treesitter_types::Span::from(node))),
            "context_bound" => Ok(Self::ContextBound(::std::boxed::Box::new(
                <ContextBound as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "lower_bound" => Ok(Self::LowerBound(::std::boxed::Box::new(
                <LowerBound as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "upper_bound" => Ok(Self::UpperBound(::std::boxed::Box::new(
                <UpperBound as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "{" => Ok(Self::LBrace(::treesitter_types::Span::from(node))),
            "}" => Ok(Self::RBrace(::treesitter_types::Span::from(node))),
            other => Err(::treesitter_types::ParseError::unexpected_kind(other, node)),
        }
    }
}
impl ::treesitter_types::Spanned for TypeDefinitionBound<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        match self {
            Self::Comma(span) => *span,
            Self::Colon(span) => *span,
            Self::ContextBound(inner) => inner.span(),
            Self::LowerBound(inner) => inner.span(),
            Self::UpperBound(inner) => inner.span(),
            Self::LBrace(span) => *span,
            Self::RBrace(span) => *span,
        }
    }
}
#[derive(Debug, Clone)]
pub enum TypeDefinitionType<'tree> {
    AnnotatedType(::std::boxed::Box<AnnotatedType<'tree>>),
    AppliedConstructorType(::std::boxed::Box<AppliedConstructorType<'tree>>),
    CompoundType(::std::boxed::Box<CompoundType<'tree>>),
    FunctionType(::std::boxed::Box<FunctionType<'tree>>),
    GenericType(::std::boxed::Box<GenericType<'tree>>),
    InfixType(::std::boxed::Box<InfixType<'tree>>),
    LiteralType(::std::boxed::Box<LiteralType<'tree>>),
    MatchType(::std::boxed::Box<MatchType<'tree>>),
    NamedTupleType(::std::boxed::Box<NamedTupleType<'tree>>),
    ProjectedType(::std::boxed::Box<ProjectedType<'tree>>),
    SingletonType(::std::boxed::Box<SingletonType<'tree>>),
    StableTypeIdentifier(::std::boxed::Box<StableTypeIdentifier<'tree>>),
    StructuralType(::std::boxed::Box<StructuralType<'tree>>),
    TupleType(::std::boxed::Box<TupleType<'tree>>),
    TypeIdentifier(::std::boxed::Box<TypeIdentifier<'tree>>),
    TypeLambda(::std::boxed::Box<TypeLambda<'tree>>),
    Wildcard(::std::boxed::Box<Wildcard<'tree>>),
}
impl<'tree> ::treesitter_types::FromNode<'tree> for TypeDefinitionType<'tree> {
    #[allow(clippy::collapsible_else_if)]
    fn from_node(
        node: ::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        match node.kind() {
            "annotated_type" => Ok(Self::AnnotatedType(::std::boxed::Box::new(
                <AnnotatedType as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "applied_constructor_type" => Ok(Self::AppliedConstructorType(::std::boxed::Box::new(
                <AppliedConstructorType as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "compound_type" => Ok(Self::CompoundType(::std::boxed::Box::new(
                <CompoundType as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "function_type" => Ok(Self::FunctionType(::std::boxed::Box::new(
                <FunctionType as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "generic_type" => Ok(Self::GenericType(::std::boxed::Box::new(
                <GenericType as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "infix_type" => Ok(Self::InfixType(::std::boxed::Box::new(
                <InfixType as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "literal_type" => Ok(Self::LiteralType(::std::boxed::Box::new(
                <LiteralType as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "match_type" => Ok(Self::MatchType(::std::boxed::Box::new(
                <MatchType as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "named_tuple_type" => Ok(Self::NamedTupleType(::std::boxed::Box::new(
                <NamedTupleType as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "projected_type" => Ok(Self::ProjectedType(::std::boxed::Box::new(
                <ProjectedType as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "singleton_type" => Ok(Self::SingletonType(::std::boxed::Box::new(
                <SingletonType as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "stable_type_identifier" => Ok(Self::StableTypeIdentifier(::std::boxed::Box::new(
                <StableTypeIdentifier as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "structural_type" => Ok(Self::StructuralType(::std::boxed::Box::new(
                <StructuralType as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "tuple_type" => Ok(Self::TupleType(::std::boxed::Box::new(
                <TupleType as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "type_identifier" => Ok(Self::TypeIdentifier(::std::boxed::Box::new(
                <TypeIdentifier as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "type_lambda" => Ok(Self::TypeLambda(::std::boxed::Box::new(
                <TypeLambda as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "wildcard" => Ok(Self::Wildcard(::std::boxed::Box::new(
                <Wildcard as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            other => Err(::treesitter_types::ParseError::unexpected_kind(other, node)),
        }
    }
}
impl ::treesitter_types::Spanned for TypeDefinitionType<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        match self {
            Self::AnnotatedType(inner) => inner.span(),
            Self::AppliedConstructorType(inner) => inner.span(),
            Self::CompoundType(inner) => inner.span(),
            Self::FunctionType(inner) => inner.span(),
            Self::GenericType(inner) => inner.span(),
            Self::InfixType(inner) => inner.span(),
            Self::LiteralType(inner) => inner.span(),
            Self::MatchType(inner) => inner.span(),
            Self::NamedTupleType(inner) => inner.span(),
            Self::ProjectedType(inner) => inner.span(),
            Self::SingletonType(inner) => inner.span(),
            Self::StableTypeIdentifier(inner) => inner.span(),
            Self::StructuralType(inner) => inner.span(),
            Self::TupleType(inner) => inner.span(),
            Self::TypeIdentifier(inner) => inner.span(),
            Self::TypeLambda(inner) => inner.span(),
            Self::Wildcard(inner) => inner.span(),
        }
    }
}
#[derive(Debug, Clone)]
pub enum TypeDefinitionChildren<'tree> {
    Annotation(::std::boxed::Box<Annotation<'tree>>),
    Modifiers(::std::boxed::Box<Modifiers<'tree>>),
    OpaqueModifier(::std::boxed::Box<OpaqueModifier<'tree>>),
}
impl<'tree> ::treesitter_types::FromNode<'tree> for TypeDefinitionChildren<'tree> {
    #[allow(clippy::collapsible_else_if)]
    fn from_node(
        node: ::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        match node.kind() {
            "annotation" => Ok(Self::Annotation(::std::boxed::Box::new(
                <Annotation as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "modifiers" => Ok(Self::Modifiers(::std::boxed::Box::new(
                <Modifiers as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "opaque_modifier" => Ok(Self::OpaqueModifier(::std::boxed::Box::new(
                <OpaqueModifier as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            other => Err(::treesitter_types::ParseError::unexpected_kind(other, node)),
        }
    }
}
impl ::treesitter_types::Spanned for TypeDefinitionChildren<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        match self {
            Self::Annotation(inner) => inner.span(),
            Self::Modifiers(inner) => inner.span(),
            Self::OpaqueModifier(inner) => inner.span(),
        }
    }
}
#[derive(Debug, Clone)]
pub enum TypeLambdaBound<'tree> {
    Comma(::treesitter_types::Span),
    Colon(::treesitter_types::Span),
    ContextBound(::std::boxed::Box<ContextBound<'tree>>),
    LowerBound(::std::boxed::Box<LowerBound<'tree>>),
    UpperBound(::std::boxed::Box<UpperBound<'tree>>),
    ViewBound(::std::boxed::Box<ViewBound<'tree>>),
    LBrace(::treesitter_types::Span),
    RBrace(::treesitter_types::Span),
}
impl<'tree> ::treesitter_types::FromNode<'tree> for TypeLambdaBound<'tree> {
    #[allow(clippy::collapsible_else_if)]
    fn from_node(
        node: ::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        match node.kind() {
            "," => Ok(Self::Comma(::treesitter_types::Span::from(node))),
            ":" => Ok(Self::Colon(::treesitter_types::Span::from(node))),
            "context_bound" => Ok(Self::ContextBound(::std::boxed::Box::new(
                <ContextBound as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "lower_bound" => Ok(Self::LowerBound(::std::boxed::Box::new(
                <LowerBound as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "upper_bound" => Ok(Self::UpperBound(::std::boxed::Box::new(
                <UpperBound as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "view_bound" => Ok(Self::ViewBound(::std::boxed::Box::new(
                <ViewBound as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "{" => Ok(Self::LBrace(::treesitter_types::Span::from(node))),
            "}" => Ok(Self::RBrace(::treesitter_types::Span::from(node))),
            other => Err(::treesitter_types::ParseError::unexpected_kind(other, node)),
        }
    }
}
impl ::treesitter_types::Spanned for TypeLambdaBound<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        match self {
            Self::Comma(span) => *span,
            Self::Colon(span) => *span,
            Self::ContextBound(inner) => inner.span(),
            Self::LowerBound(inner) => inner.span(),
            Self::UpperBound(inner) => inner.span(),
            Self::ViewBound(inner) => inner.span(),
            Self::LBrace(span) => *span,
            Self::RBrace(span) => *span,
        }
    }
}
#[derive(Debug, Clone)]
pub enum TypeLambdaName<'tree> {
    Identifier(::std::boxed::Box<Identifier<'tree>>),
    OperatorIdentifier(::std::boxed::Box<OperatorIdentifier<'tree>>),
    Wildcard(::std::boxed::Box<Wildcard<'tree>>),
}
impl<'tree> ::treesitter_types::FromNode<'tree> for TypeLambdaName<'tree> {
    #[allow(clippy::collapsible_else_if)]
    fn from_node(
        node: ::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        match node.kind() {
            "identifier" => Ok(Self::Identifier(::std::boxed::Box::new(
                <Identifier as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "operator_identifier" => Ok(Self::OperatorIdentifier(::std::boxed::Box::new(
                <OperatorIdentifier as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "wildcard" => Ok(Self::Wildcard(::std::boxed::Box::new(
                <Wildcard as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            other => Err(::treesitter_types::ParseError::unexpected_kind(other, node)),
        }
    }
}
impl ::treesitter_types::Spanned for TypeLambdaName<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        match self {
            Self::Identifier(inner) => inner.span(),
            Self::OperatorIdentifier(inner) => inner.span(),
            Self::Wildcard(inner) => inner.span(),
        }
    }
}
#[derive(Debug, Clone)]
pub enum TypeLambdaReturnType<'tree> {
    AnnotatedType(::std::boxed::Box<AnnotatedType<'tree>>),
    AppliedConstructorType(::std::boxed::Box<AppliedConstructorType<'tree>>),
    CompoundType(::std::boxed::Box<CompoundType<'tree>>),
    FunctionType(::std::boxed::Box<FunctionType<'tree>>),
    GenericType(::std::boxed::Box<GenericType<'tree>>),
    InfixType(::std::boxed::Box<InfixType<'tree>>),
    LiteralType(::std::boxed::Box<LiteralType<'tree>>),
    MatchType(::std::boxed::Box<MatchType<'tree>>),
    NamedTupleType(::std::boxed::Box<NamedTupleType<'tree>>),
    ProjectedType(::std::boxed::Box<ProjectedType<'tree>>),
    SingletonType(::std::boxed::Box<SingletonType<'tree>>),
    StableTypeIdentifier(::std::boxed::Box<StableTypeIdentifier<'tree>>),
    StructuralType(::std::boxed::Box<StructuralType<'tree>>),
    TupleType(::std::boxed::Box<TupleType<'tree>>),
    TypeIdentifier(::std::boxed::Box<TypeIdentifier<'tree>>),
    TypeLambda(::std::boxed::Box<TypeLambda<'tree>>),
    Wildcard(::std::boxed::Box<Wildcard<'tree>>),
}
impl<'tree> ::treesitter_types::FromNode<'tree> for TypeLambdaReturnType<'tree> {
    #[allow(clippy::collapsible_else_if)]
    fn from_node(
        node: ::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        match node.kind() {
            "annotated_type" => Ok(Self::AnnotatedType(::std::boxed::Box::new(
                <AnnotatedType as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "applied_constructor_type" => Ok(Self::AppliedConstructorType(::std::boxed::Box::new(
                <AppliedConstructorType as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "compound_type" => Ok(Self::CompoundType(::std::boxed::Box::new(
                <CompoundType as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "function_type" => Ok(Self::FunctionType(::std::boxed::Box::new(
                <FunctionType as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "generic_type" => Ok(Self::GenericType(::std::boxed::Box::new(
                <GenericType as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "infix_type" => Ok(Self::InfixType(::std::boxed::Box::new(
                <InfixType as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "literal_type" => Ok(Self::LiteralType(::std::boxed::Box::new(
                <LiteralType as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "match_type" => Ok(Self::MatchType(::std::boxed::Box::new(
                <MatchType as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "named_tuple_type" => Ok(Self::NamedTupleType(::std::boxed::Box::new(
                <NamedTupleType as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "projected_type" => Ok(Self::ProjectedType(::std::boxed::Box::new(
                <ProjectedType as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "singleton_type" => Ok(Self::SingletonType(::std::boxed::Box::new(
                <SingletonType as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "stable_type_identifier" => Ok(Self::StableTypeIdentifier(::std::boxed::Box::new(
                <StableTypeIdentifier as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "structural_type" => Ok(Self::StructuralType(::std::boxed::Box::new(
                <StructuralType as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "tuple_type" => Ok(Self::TupleType(::std::boxed::Box::new(
                <TupleType as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "type_identifier" => Ok(Self::TypeIdentifier(::std::boxed::Box::new(
                <TypeIdentifier as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "type_lambda" => Ok(Self::TypeLambda(::std::boxed::Box::new(
                <TypeLambda as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "wildcard" => Ok(Self::Wildcard(::std::boxed::Box::new(
                <Wildcard as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            other => Err(::treesitter_types::ParseError::unexpected_kind(other, node)),
        }
    }
}
impl ::treesitter_types::Spanned for TypeLambdaReturnType<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        match self {
            Self::AnnotatedType(inner) => inner.span(),
            Self::AppliedConstructorType(inner) => inner.span(),
            Self::CompoundType(inner) => inner.span(),
            Self::FunctionType(inner) => inner.span(),
            Self::GenericType(inner) => inner.span(),
            Self::InfixType(inner) => inner.span(),
            Self::LiteralType(inner) => inner.span(),
            Self::MatchType(inner) => inner.span(),
            Self::NamedTupleType(inner) => inner.span(),
            Self::ProjectedType(inner) => inner.span(),
            Self::SingletonType(inner) => inner.span(),
            Self::StableTypeIdentifier(inner) => inner.span(),
            Self::StructuralType(inner) => inner.span(),
            Self::TupleType(inner) => inner.span(),
            Self::TypeIdentifier(inner) => inner.span(),
            Self::TypeLambda(inner) => inner.span(),
            Self::Wildcard(inner) => inner.span(),
        }
    }
}
#[derive(Debug, Clone)]
pub enum TypeParametersBound<'tree> {
    Comma(::treesitter_types::Span),
    Colon(::treesitter_types::Span),
    ContextBound(::std::boxed::Box<ContextBound<'tree>>),
    LowerBound(::std::boxed::Box<LowerBound<'tree>>),
    UpperBound(::std::boxed::Box<UpperBound<'tree>>),
    ViewBound(::std::boxed::Box<ViewBound<'tree>>),
    LBrace(::treesitter_types::Span),
    RBrace(::treesitter_types::Span),
}
impl<'tree> ::treesitter_types::FromNode<'tree> for TypeParametersBound<'tree> {
    #[allow(clippy::collapsible_else_if)]
    fn from_node(
        node: ::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        match node.kind() {
            "," => Ok(Self::Comma(::treesitter_types::Span::from(node))),
            ":" => Ok(Self::Colon(::treesitter_types::Span::from(node))),
            "context_bound" => Ok(Self::ContextBound(::std::boxed::Box::new(
                <ContextBound as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "lower_bound" => Ok(Self::LowerBound(::std::boxed::Box::new(
                <LowerBound as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "upper_bound" => Ok(Self::UpperBound(::std::boxed::Box::new(
                <UpperBound as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "view_bound" => Ok(Self::ViewBound(::std::boxed::Box::new(
                <ViewBound as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "{" => Ok(Self::LBrace(::treesitter_types::Span::from(node))),
            "}" => Ok(Self::RBrace(::treesitter_types::Span::from(node))),
            other => Err(::treesitter_types::ParseError::unexpected_kind(other, node)),
        }
    }
}
impl ::treesitter_types::Spanned for TypeParametersBound<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        match self {
            Self::Comma(span) => *span,
            Self::Colon(span) => *span,
            Self::ContextBound(inner) => inner.span(),
            Self::LowerBound(inner) => inner.span(),
            Self::UpperBound(inner) => inner.span(),
            Self::ViewBound(inner) => inner.span(),
            Self::LBrace(span) => *span,
            Self::RBrace(span) => *span,
        }
    }
}
#[derive(Debug, Clone)]
pub enum TypeParametersName<'tree> {
    Identifier(::std::boxed::Box<Identifier<'tree>>),
    OperatorIdentifier(::std::boxed::Box<OperatorIdentifier<'tree>>),
    Wildcard(::std::boxed::Box<Wildcard<'tree>>),
}
impl<'tree> ::treesitter_types::FromNode<'tree> for TypeParametersName<'tree> {
    #[allow(clippy::collapsible_else_if)]
    fn from_node(
        node: ::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        match node.kind() {
            "identifier" => Ok(Self::Identifier(::std::boxed::Box::new(
                <Identifier as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "operator_identifier" => Ok(Self::OperatorIdentifier(::std::boxed::Box::new(
                <OperatorIdentifier as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "wildcard" => Ok(Self::Wildcard(::std::boxed::Box::new(
                <Wildcard as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            other => Err(::treesitter_types::ParseError::unexpected_kind(other, node)),
        }
    }
}
impl ::treesitter_types::Spanned for TypeParametersName<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        match self {
            Self::Identifier(inner) => inner.span(),
            Self::OperatorIdentifier(inner) => inner.span(),
            Self::Wildcard(inner) => inner.span(),
        }
    }
}
#[derive(Debug, Clone)]
pub enum TypeParametersChildren<'tree> {
    Annotation(::std::boxed::Box<Annotation<'tree>>),
    ContravariantTypeParameter(::std::boxed::Box<ContravariantTypeParameter<'tree>>),
    CovariantTypeParameter(::std::boxed::Box<CovariantTypeParameter<'tree>>),
    TypeLambda(::std::boxed::Box<TypeLambda<'tree>>),
}
impl<'tree> ::treesitter_types::FromNode<'tree> for TypeParametersChildren<'tree> {
    #[allow(clippy::collapsible_else_if)]
    fn from_node(
        node: ::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        match node.kind() {
            "annotation" => Ok(Self::Annotation(::std::boxed::Box::new(
                <Annotation as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "contravariant_type_parameter" => {
                Ok(Self::ContravariantTypeParameter(::std::boxed::Box::new(
                    <ContravariantTypeParameter as ::treesitter_types::FromNode>::from_node(
                        node, src,
                    )?,
                )))
            }
            "covariant_type_parameter" => Ok(Self::CovariantTypeParameter(::std::boxed::Box::new(
                <CovariantTypeParameter as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "type_lambda" => Ok(Self::TypeLambda(::std::boxed::Box::new(
                <TypeLambda as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            other => Err(::treesitter_types::ParseError::unexpected_kind(other, node)),
        }
    }
}
impl ::treesitter_types::Spanned for TypeParametersChildren<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        match self {
            Self::Annotation(inner) => inner.span(),
            Self::ContravariantTypeParameter(inner) => inner.span(),
            Self::CovariantTypeParameter(inner) => inner.span(),
            Self::TypeLambda(inner) => inner.span(),
        }
    }
}
#[derive(Debug, Clone)]
pub enum TypedPatternType<'tree> {
    AnnotatedType(::std::boxed::Box<AnnotatedType<'tree>>),
    AppliedConstructorType(::std::boxed::Box<AppliedConstructorType<'tree>>),
    CompoundType(::std::boxed::Box<CompoundType<'tree>>),
    FunctionType(::std::boxed::Box<FunctionType<'tree>>),
    GenericType(::std::boxed::Box<GenericType<'tree>>),
    InfixType(::std::boxed::Box<InfixType<'tree>>),
    LiteralType(::std::boxed::Box<LiteralType<'tree>>),
    MatchType(::std::boxed::Box<MatchType<'tree>>),
    NamedTupleType(::std::boxed::Box<NamedTupleType<'tree>>),
    ProjectedType(::std::boxed::Box<ProjectedType<'tree>>),
    SingletonType(::std::boxed::Box<SingletonType<'tree>>),
    StableTypeIdentifier(::std::boxed::Box<StableTypeIdentifier<'tree>>),
    StructuralType(::std::boxed::Box<StructuralType<'tree>>),
    TupleType(::std::boxed::Box<TupleType<'tree>>),
    TypeIdentifier(::std::boxed::Box<TypeIdentifier<'tree>>),
    TypeLambda(::std::boxed::Box<TypeLambda<'tree>>),
    Wildcard(::std::boxed::Box<Wildcard<'tree>>),
}
impl<'tree> ::treesitter_types::FromNode<'tree> for TypedPatternType<'tree> {
    #[allow(clippy::collapsible_else_if)]
    fn from_node(
        node: ::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        match node.kind() {
            "annotated_type" => Ok(Self::AnnotatedType(::std::boxed::Box::new(
                <AnnotatedType as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "applied_constructor_type" => Ok(Self::AppliedConstructorType(::std::boxed::Box::new(
                <AppliedConstructorType as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "compound_type" => Ok(Self::CompoundType(::std::boxed::Box::new(
                <CompoundType as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "function_type" => Ok(Self::FunctionType(::std::boxed::Box::new(
                <FunctionType as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "generic_type" => Ok(Self::GenericType(::std::boxed::Box::new(
                <GenericType as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "infix_type" => Ok(Self::InfixType(::std::boxed::Box::new(
                <InfixType as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "literal_type" => Ok(Self::LiteralType(::std::boxed::Box::new(
                <LiteralType as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "match_type" => Ok(Self::MatchType(::std::boxed::Box::new(
                <MatchType as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "named_tuple_type" => Ok(Self::NamedTupleType(::std::boxed::Box::new(
                <NamedTupleType as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "projected_type" => Ok(Self::ProjectedType(::std::boxed::Box::new(
                <ProjectedType as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "singleton_type" => Ok(Self::SingletonType(::std::boxed::Box::new(
                <SingletonType as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "stable_type_identifier" => Ok(Self::StableTypeIdentifier(::std::boxed::Box::new(
                <StableTypeIdentifier as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "structural_type" => Ok(Self::StructuralType(::std::boxed::Box::new(
                <StructuralType as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "tuple_type" => Ok(Self::TupleType(::std::boxed::Box::new(
                <TupleType as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "type_identifier" => Ok(Self::TypeIdentifier(::std::boxed::Box::new(
                <TypeIdentifier as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "type_lambda" => Ok(Self::TypeLambda(::std::boxed::Box::new(
                <TypeLambda as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "wildcard" => Ok(Self::Wildcard(::std::boxed::Box::new(
                <Wildcard as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            other => Err(::treesitter_types::ParseError::unexpected_kind(other, node)),
        }
    }
}
impl ::treesitter_types::Spanned for TypedPatternType<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        match self {
            Self::AnnotatedType(inner) => inner.span(),
            Self::AppliedConstructorType(inner) => inner.span(),
            Self::CompoundType(inner) => inner.span(),
            Self::FunctionType(inner) => inner.span(),
            Self::GenericType(inner) => inner.span(),
            Self::InfixType(inner) => inner.span(),
            Self::LiteralType(inner) => inner.span(),
            Self::MatchType(inner) => inner.span(),
            Self::NamedTupleType(inner) => inner.span(),
            Self::ProjectedType(inner) => inner.span(),
            Self::SingletonType(inner) => inner.span(),
            Self::StableTypeIdentifier(inner) => inner.span(),
            Self::StructuralType(inner) => inner.span(),
            Self::TupleType(inner) => inner.span(),
            Self::TypeIdentifier(inner) => inner.span(),
            Self::TypeLambda(inner) => inner.span(),
            Self::Wildcard(inner) => inner.span(),
        }
    }
}
#[derive(Debug, Clone)]
pub enum UpperBoundType<'tree> {
    AnnotatedType(::std::boxed::Box<AnnotatedType<'tree>>),
    AppliedConstructorType(::std::boxed::Box<AppliedConstructorType<'tree>>),
    CompoundType(::std::boxed::Box<CompoundType<'tree>>),
    FunctionType(::std::boxed::Box<FunctionType<'tree>>),
    GenericType(::std::boxed::Box<GenericType<'tree>>),
    InfixType(::std::boxed::Box<InfixType<'tree>>),
    LiteralType(::std::boxed::Box<LiteralType<'tree>>),
    MatchType(::std::boxed::Box<MatchType<'tree>>),
    NamedTupleType(::std::boxed::Box<NamedTupleType<'tree>>),
    ProjectedType(::std::boxed::Box<ProjectedType<'tree>>),
    SingletonType(::std::boxed::Box<SingletonType<'tree>>),
    StableTypeIdentifier(::std::boxed::Box<StableTypeIdentifier<'tree>>),
    StructuralType(::std::boxed::Box<StructuralType<'tree>>),
    TupleType(::std::boxed::Box<TupleType<'tree>>),
    TypeIdentifier(::std::boxed::Box<TypeIdentifier<'tree>>),
    TypeLambda(::std::boxed::Box<TypeLambda<'tree>>),
    Wildcard(::std::boxed::Box<Wildcard<'tree>>),
}
impl<'tree> ::treesitter_types::FromNode<'tree> for UpperBoundType<'tree> {
    #[allow(clippy::collapsible_else_if)]
    fn from_node(
        node: ::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        match node.kind() {
            "annotated_type" => Ok(Self::AnnotatedType(::std::boxed::Box::new(
                <AnnotatedType as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "applied_constructor_type" => Ok(Self::AppliedConstructorType(::std::boxed::Box::new(
                <AppliedConstructorType as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "compound_type" => Ok(Self::CompoundType(::std::boxed::Box::new(
                <CompoundType as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "function_type" => Ok(Self::FunctionType(::std::boxed::Box::new(
                <FunctionType as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "generic_type" => Ok(Self::GenericType(::std::boxed::Box::new(
                <GenericType as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "infix_type" => Ok(Self::InfixType(::std::boxed::Box::new(
                <InfixType as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "literal_type" => Ok(Self::LiteralType(::std::boxed::Box::new(
                <LiteralType as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "match_type" => Ok(Self::MatchType(::std::boxed::Box::new(
                <MatchType as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "named_tuple_type" => Ok(Self::NamedTupleType(::std::boxed::Box::new(
                <NamedTupleType as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "projected_type" => Ok(Self::ProjectedType(::std::boxed::Box::new(
                <ProjectedType as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "singleton_type" => Ok(Self::SingletonType(::std::boxed::Box::new(
                <SingletonType as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "stable_type_identifier" => Ok(Self::StableTypeIdentifier(::std::boxed::Box::new(
                <StableTypeIdentifier as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "structural_type" => Ok(Self::StructuralType(::std::boxed::Box::new(
                <StructuralType as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "tuple_type" => Ok(Self::TupleType(::std::boxed::Box::new(
                <TupleType as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "type_identifier" => Ok(Self::TypeIdentifier(::std::boxed::Box::new(
                <TypeIdentifier as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "type_lambda" => Ok(Self::TypeLambda(::std::boxed::Box::new(
                <TypeLambda as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "wildcard" => Ok(Self::Wildcard(::std::boxed::Box::new(
                <Wildcard as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            other => Err(::treesitter_types::ParseError::unexpected_kind(other, node)),
        }
    }
}
impl ::treesitter_types::Spanned for UpperBoundType<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        match self {
            Self::AnnotatedType(inner) => inner.span(),
            Self::AppliedConstructorType(inner) => inner.span(),
            Self::CompoundType(inner) => inner.span(),
            Self::FunctionType(inner) => inner.span(),
            Self::GenericType(inner) => inner.span(),
            Self::InfixType(inner) => inner.span(),
            Self::LiteralType(inner) => inner.span(),
            Self::MatchType(inner) => inner.span(),
            Self::NamedTupleType(inner) => inner.span(),
            Self::ProjectedType(inner) => inner.span(),
            Self::SingletonType(inner) => inner.span(),
            Self::StableTypeIdentifier(inner) => inner.span(),
            Self::StructuralType(inner) => inner.span(),
            Self::TupleType(inner) => inner.span(),
            Self::TypeIdentifier(inner) => inner.span(),
            Self::TypeLambda(inner) => inner.span(),
            Self::Wildcard(inner) => inner.span(),
        }
    }
}
#[derive(Debug, Clone)]
pub enum UsingDirectiveChildren<'tree> {
    UsingDirectiveKey(::std::boxed::Box<UsingDirectiveKey<'tree>>),
    UsingDirectiveValue(::std::boxed::Box<UsingDirectiveValue<'tree>>),
}
impl<'tree> ::treesitter_types::FromNode<'tree> for UsingDirectiveChildren<'tree> {
    #[allow(clippy::collapsible_else_if)]
    fn from_node(
        node: ::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        match node.kind() {
            "using_directive_key" => Ok(Self::UsingDirectiveKey(::std::boxed::Box::new(
                <UsingDirectiveKey as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "using_directive_value" => Ok(Self::UsingDirectiveValue(::std::boxed::Box::new(
                <UsingDirectiveValue as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            other => Err(::treesitter_types::ParseError::unexpected_kind(other, node)),
        }
    }
}
impl ::treesitter_types::Spanned for UsingDirectiveChildren<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        match self {
            Self::UsingDirectiveKey(inner) => inner.span(),
            Self::UsingDirectiveValue(inner) => inner.span(),
        }
    }
}
#[derive(Debug, Clone)]
pub enum ValDeclarationName<'tree> {
    Identifier(::std::boxed::Box<Identifier<'tree>>),
    OperatorIdentifier(::std::boxed::Box<OperatorIdentifier<'tree>>),
}
impl<'tree> ::treesitter_types::FromNode<'tree> for ValDeclarationName<'tree> {
    #[allow(clippy::collapsible_else_if)]
    fn from_node(
        node: ::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        match node.kind() {
            "identifier" => Ok(Self::Identifier(::std::boxed::Box::new(
                <Identifier as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "operator_identifier" => Ok(Self::OperatorIdentifier(::std::boxed::Box::new(
                <OperatorIdentifier as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            other => Err(::treesitter_types::ParseError::unexpected_kind(other, node)),
        }
    }
}
impl ::treesitter_types::Spanned for ValDeclarationName<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        match self {
            Self::Identifier(inner) => inner.span(),
            Self::OperatorIdentifier(inner) => inner.span(),
        }
    }
}
#[derive(Debug, Clone)]
pub enum ValDeclarationType<'tree> {
    AnnotatedType(::std::boxed::Box<AnnotatedType<'tree>>),
    AppliedConstructorType(::std::boxed::Box<AppliedConstructorType<'tree>>),
    CompoundType(::std::boxed::Box<CompoundType<'tree>>),
    FunctionType(::std::boxed::Box<FunctionType<'tree>>),
    GenericType(::std::boxed::Box<GenericType<'tree>>),
    InfixType(::std::boxed::Box<InfixType<'tree>>),
    LiteralType(::std::boxed::Box<LiteralType<'tree>>),
    MatchType(::std::boxed::Box<MatchType<'tree>>),
    NamedTupleType(::std::boxed::Box<NamedTupleType<'tree>>),
    ProjectedType(::std::boxed::Box<ProjectedType<'tree>>),
    SingletonType(::std::boxed::Box<SingletonType<'tree>>),
    StableTypeIdentifier(::std::boxed::Box<StableTypeIdentifier<'tree>>),
    StructuralType(::std::boxed::Box<StructuralType<'tree>>),
    TupleType(::std::boxed::Box<TupleType<'tree>>),
    TypeIdentifier(::std::boxed::Box<TypeIdentifier<'tree>>),
    TypeLambda(::std::boxed::Box<TypeLambda<'tree>>),
    Wildcard(::std::boxed::Box<Wildcard<'tree>>),
}
impl<'tree> ::treesitter_types::FromNode<'tree> for ValDeclarationType<'tree> {
    #[allow(clippy::collapsible_else_if)]
    fn from_node(
        node: ::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        match node.kind() {
            "annotated_type" => Ok(Self::AnnotatedType(::std::boxed::Box::new(
                <AnnotatedType as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "applied_constructor_type" => Ok(Self::AppliedConstructorType(::std::boxed::Box::new(
                <AppliedConstructorType as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "compound_type" => Ok(Self::CompoundType(::std::boxed::Box::new(
                <CompoundType as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "function_type" => Ok(Self::FunctionType(::std::boxed::Box::new(
                <FunctionType as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "generic_type" => Ok(Self::GenericType(::std::boxed::Box::new(
                <GenericType as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "infix_type" => Ok(Self::InfixType(::std::boxed::Box::new(
                <InfixType as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "literal_type" => Ok(Self::LiteralType(::std::boxed::Box::new(
                <LiteralType as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "match_type" => Ok(Self::MatchType(::std::boxed::Box::new(
                <MatchType as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "named_tuple_type" => Ok(Self::NamedTupleType(::std::boxed::Box::new(
                <NamedTupleType as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "projected_type" => Ok(Self::ProjectedType(::std::boxed::Box::new(
                <ProjectedType as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "singleton_type" => Ok(Self::SingletonType(::std::boxed::Box::new(
                <SingletonType as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "stable_type_identifier" => Ok(Self::StableTypeIdentifier(::std::boxed::Box::new(
                <StableTypeIdentifier as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "structural_type" => Ok(Self::StructuralType(::std::boxed::Box::new(
                <StructuralType as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "tuple_type" => Ok(Self::TupleType(::std::boxed::Box::new(
                <TupleType as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "type_identifier" => Ok(Self::TypeIdentifier(::std::boxed::Box::new(
                <TypeIdentifier as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "type_lambda" => Ok(Self::TypeLambda(::std::boxed::Box::new(
                <TypeLambda as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "wildcard" => Ok(Self::Wildcard(::std::boxed::Box::new(
                <Wildcard as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            other => Err(::treesitter_types::ParseError::unexpected_kind(other, node)),
        }
    }
}
impl ::treesitter_types::Spanned for ValDeclarationType<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        match self {
            Self::AnnotatedType(inner) => inner.span(),
            Self::AppliedConstructorType(inner) => inner.span(),
            Self::CompoundType(inner) => inner.span(),
            Self::FunctionType(inner) => inner.span(),
            Self::GenericType(inner) => inner.span(),
            Self::InfixType(inner) => inner.span(),
            Self::LiteralType(inner) => inner.span(),
            Self::MatchType(inner) => inner.span(),
            Self::NamedTupleType(inner) => inner.span(),
            Self::ProjectedType(inner) => inner.span(),
            Self::SingletonType(inner) => inner.span(),
            Self::StableTypeIdentifier(inner) => inner.span(),
            Self::StructuralType(inner) => inner.span(),
            Self::TupleType(inner) => inner.span(),
            Self::TypeIdentifier(inner) => inner.span(),
            Self::TypeLambda(inner) => inner.span(),
            Self::Wildcard(inner) => inner.span(),
        }
    }
}
#[derive(Debug, Clone)]
pub enum ValDeclarationChildren<'tree> {
    Annotation(::std::boxed::Box<Annotation<'tree>>),
    Modifiers(::std::boxed::Box<Modifiers<'tree>>),
}
impl<'tree> ::treesitter_types::FromNode<'tree> for ValDeclarationChildren<'tree> {
    #[allow(clippy::collapsible_else_if)]
    fn from_node(
        node: ::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        match node.kind() {
            "annotation" => Ok(Self::Annotation(::std::boxed::Box::new(
                <Annotation as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "modifiers" => Ok(Self::Modifiers(::std::boxed::Box::new(
                <Modifiers as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            other => Err(::treesitter_types::ParseError::unexpected_kind(other, node)),
        }
    }
}
impl ::treesitter_types::Spanned for ValDeclarationChildren<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        match self {
            Self::Annotation(inner) => inner.span(),
            Self::Modifiers(inner) => inner.span(),
        }
    }
}
#[derive(Debug, Clone)]
pub enum ValDefinitionPattern<'tree> {
    Pattern(::std::boxed::Box<Pattern<'tree>>),
    Identifiers(::std::boxed::Box<Identifiers<'tree>>),
}
impl<'tree> ::treesitter_types::FromNode<'tree> for ValDefinitionPattern<'tree> {
    #[allow(clippy::collapsible_else_if)]
    fn from_node(
        node: ::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        match node.kind() {
            "identifiers" => Ok(Self::Identifiers(::std::boxed::Box::new(
                <Identifiers as ::treesitter_types::FromNode>::from_node(node, src)?,
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
impl ::treesitter_types::Spanned for ValDefinitionPattern<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        match self {
            Self::Pattern(inner) => inner.span(),
            Self::Identifiers(inner) => inner.span(),
        }
    }
}
#[derive(Debug, Clone)]
pub enum ValDefinitionType<'tree> {
    AnnotatedType(::std::boxed::Box<AnnotatedType<'tree>>),
    AppliedConstructorType(::std::boxed::Box<AppliedConstructorType<'tree>>),
    CompoundType(::std::boxed::Box<CompoundType<'tree>>),
    FunctionType(::std::boxed::Box<FunctionType<'tree>>),
    GenericType(::std::boxed::Box<GenericType<'tree>>),
    InfixType(::std::boxed::Box<InfixType<'tree>>),
    LiteralType(::std::boxed::Box<LiteralType<'tree>>),
    MatchType(::std::boxed::Box<MatchType<'tree>>),
    NamedTupleType(::std::boxed::Box<NamedTupleType<'tree>>),
    ProjectedType(::std::boxed::Box<ProjectedType<'tree>>),
    SingletonType(::std::boxed::Box<SingletonType<'tree>>),
    StableTypeIdentifier(::std::boxed::Box<StableTypeIdentifier<'tree>>),
    StructuralType(::std::boxed::Box<StructuralType<'tree>>),
    TupleType(::std::boxed::Box<TupleType<'tree>>),
    TypeIdentifier(::std::boxed::Box<TypeIdentifier<'tree>>),
    TypeLambda(::std::boxed::Box<TypeLambda<'tree>>),
    Wildcard(::std::boxed::Box<Wildcard<'tree>>),
}
impl<'tree> ::treesitter_types::FromNode<'tree> for ValDefinitionType<'tree> {
    #[allow(clippy::collapsible_else_if)]
    fn from_node(
        node: ::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        match node.kind() {
            "annotated_type" => Ok(Self::AnnotatedType(::std::boxed::Box::new(
                <AnnotatedType as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "applied_constructor_type" => Ok(Self::AppliedConstructorType(::std::boxed::Box::new(
                <AppliedConstructorType as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "compound_type" => Ok(Self::CompoundType(::std::boxed::Box::new(
                <CompoundType as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "function_type" => Ok(Self::FunctionType(::std::boxed::Box::new(
                <FunctionType as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "generic_type" => Ok(Self::GenericType(::std::boxed::Box::new(
                <GenericType as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "infix_type" => Ok(Self::InfixType(::std::boxed::Box::new(
                <InfixType as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "literal_type" => Ok(Self::LiteralType(::std::boxed::Box::new(
                <LiteralType as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "match_type" => Ok(Self::MatchType(::std::boxed::Box::new(
                <MatchType as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "named_tuple_type" => Ok(Self::NamedTupleType(::std::boxed::Box::new(
                <NamedTupleType as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "projected_type" => Ok(Self::ProjectedType(::std::boxed::Box::new(
                <ProjectedType as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "singleton_type" => Ok(Self::SingletonType(::std::boxed::Box::new(
                <SingletonType as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "stable_type_identifier" => Ok(Self::StableTypeIdentifier(::std::boxed::Box::new(
                <StableTypeIdentifier as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "structural_type" => Ok(Self::StructuralType(::std::boxed::Box::new(
                <StructuralType as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "tuple_type" => Ok(Self::TupleType(::std::boxed::Box::new(
                <TupleType as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "type_identifier" => Ok(Self::TypeIdentifier(::std::boxed::Box::new(
                <TypeIdentifier as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "type_lambda" => Ok(Self::TypeLambda(::std::boxed::Box::new(
                <TypeLambda as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "wildcard" => Ok(Self::Wildcard(::std::boxed::Box::new(
                <Wildcard as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            other => Err(::treesitter_types::ParseError::unexpected_kind(other, node)),
        }
    }
}
impl ::treesitter_types::Spanned for ValDefinitionType<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        match self {
            Self::AnnotatedType(inner) => inner.span(),
            Self::AppliedConstructorType(inner) => inner.span(),
            Self::CompoundType(inner) => inner.span(),
            Self::FunctionType(inner) => inner.span(),
            Self::GenericType(inner) => inner.span(),
            Self::InfixType(inner) => inner.span(),
            Self::LiteralType(inner) => inner.span(),
            Self::MatchType(inner) => inner.span(),
            Self::NamedTupleType(inner) => inner.span(),
            Self::ProjectedType(inner) => inner.span(),
            Self::SingletonType(inner) => inner.span(),
            Self::StableTypeIdentifier(inner) => inner.span(),
            Self::StructuralType(inner) => inner.span(),
            Self::TupleType(inner) => inner.span(),
            Self::TypeIdentifier(inner) => inner.span(),
            Self::TypeLambda(inner) => inner.span(),
            Self::Wildcard(inner) => inner.span(),
        }
    }
}
#[derive(Debug, Clone)]
pub enum ValDefinitionValue<'tree> {
    Expression(::std::boxed::Box<Expression<'tree>>),
    IndentedBlock(::std::boxed::Box<IndentedBlock<'tree>>),
    IndentedCases(::std::boxed::Box<IndentedCases<'tree>>),
}
impl<'tree> ::treesitter_types::FromNode<'tree> for ValDefinitionValue<'tree> {
    #[allow(clippy::collapsible_else_if)]
    fn from_node(
        node: ::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        match node.kind() {
            "indented_block" => Ok(Self::IndentedBlock(::std::boxed::Box::new(
                <IndentedBlock as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "indented_cases" => Ok(Self::IndentedCases(::std::boxed::Box::new(
                <IndentedCases as ::treesitter_types::FromNode>::from_node(node, src)?,
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
impl ::treesitter_types::Spanned for ValDefinitionValue<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        match self {
            Self::Expression(inner) => inner.span(),
            Self::IndentedBlock(inner) => inner.span(),
            Self::IndentedCases(inner) => inner.span(),
        }
    }
}
#[derive(Debug, Clone)]
pub enum ValDefinitionChildren<'tree> {
    Annotation(::std::boxed::Box<Annotation<'tree>>),
    Modifiers(::std::boxed::Box<Modifiers<'tree>>),
}
impl<'tree> ::treesitter_types::FromNode<'tree> for ValDefinitionChildren<'tree> {
    #[allow(clippy::collapsible_else_if)]
    fn from_node(
        node: ::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        match node.kind() {
            "annotation" => Ok(Self::Annotation(::std::boxed::Box::new(
                <Annotation as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "modifiers" => Ok(Self::Modifiers(::std::boxed::Box::new(
                <Modifiers as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            other => Err(::treesitter_types::ParseError::unexpected_kind(other, node)),
        }
    }
}
impl ::treesitter_types::Spanned for ValDefinitionChildren<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        match self {
            Self::Annotation(inner) => inner.span(),
            Self::Modifiers(inner) => inner.span(),
        }
    }
}
#[derive(Debug, Clone)]
pub enum VarDeclarationName<'tree> {
    Identifier(::std::boxed::Box<Identifier<'tree>>),
    OperatorIdentifier(::std::boxed::Box<OperatorIdentifier<'tree>>),
}
impl<'tree> ::treesitter_types::FromNode<'tree> for VarDeclarationName<'tree> {
    #[allow(clippy::collapsible_else_if)]
    fn from_node(
        node: ::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        match node.kind() {
            "identifier" => Ok(Self::Identifier(::std::boxed::Box::new(
                <Identifier as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "operator_identifier" => Ok(Self::OperatorIdentifier(::std::boxed::Box::new(
                <OperatorIdentifier as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            other => Err(::treesitter_types::ParseError::unexpected_kind(other, node)),
        }
    }
}
impl ::treesitter_types::Spanned for VarDeclarationName<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        match self {
            Self::Identifier(inner) => inner.span(),
            Self::OperatorIdentifier(inner) => inner.span(),
        }
    }
}
#[derive(Debug, Clone)]
pub enum VarDeclarationType<'tree> {
    AnnotatedType(::std::boxed::Box<AnnotatedType<'tree>>),
    AppliedConstructorType(::std::boxed::Box<AppliedConstructorType<'tree>>),
    CompoundType(::std::boxed::Box<CompoundType<'tree>>),
    FunctionType(::std::boxed::Box<FunctionType<'tree>>),
    GenericType(::std::boxed::Box<GenericType<'tree>>),
    InfixType(::std::boxed::Box<InfixType<'tree>>),
    LiteralType(::std::boxed::Box<LiteralType<'tree>>),
    MatchType(::std::boxed::Box<MatchType<'tree>>),
    NamedTupleType(::std::boxed::Box<NamedTupleType<'tree>>),
    ProjectedType(::std::boxed::Box<ProjectedType<'tree>>),
    SingletonType(::std::boxed::Box<SingletonType<'tree>>),
    StableTypeIdentifier(::std::boxed::Box<StableTypeIdentifier<'tree>>),
    StructuralType(::std::boxed::Box<StructuralType<'tree>>),
    TupleType(::std::boxed::Box<TupleType<'tree>>),
    TypeIdentifier(::std::boxed::Box<TypeIdentifier<'tree>>),
    TypeLambda(::std::boxed::Box<TypeLambda<'tree>>),
    Wildcard(::std::boxed::Box<Wildcard<'tree>>),
}
impl<'tree> ::treesitter_types::FromNode<'tree> for VarDeclarationType<'tree> {
    #[allow(clippy::collapsible_else_if)]
    fn from_node(
        node: ::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        match node.kind() {
            "annotated_type" => Ok(Self::AnnotatedType(::std::boxed::Box::new(
                <AnnotatedType as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "applied_constructor_type" => Ok(Self::AppliedConstructorType(::std::boxed::Box::new(
                <AppliedConstructorType as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "compound_type" => Ok(Self::CompoundType(::std::boxed::Box::new(
                <CompoundType as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "function_type" => Ok(Self::FunctionType(::std::boxed::Box::new(
                <FunctionType as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "generic_type" => Ok(Self::GenericType(::std::boxed::Box::new(
                <GenericType as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "infix_type" => Ok(Self::InfixType(::std::boxed::Box::new(
                <InfixType as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "literal_type" => Ok(Self::LiteralType(::std::boxed::Box::new(
                <LiteralType as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "match_type" => Ok(Self::MatchType(::std::boxed::Box::new(
                <MatchType as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "named_tuple_type" => Ok(Self::NamedTupleType(::std::boxed::Box::new(
                <NamedTupleType as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "projected_type" => Ok(Self::ProjectedType(::std::boxed::Box::new(
                <ProjectedType as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "singleton_type" => Ok(Self::SingletonType(::std::boxed::Box::new(
                <SingletonType as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "stable_type_identifier" => Ok(Self::StableTypeIdentifier(::std::boxed::Box::new(
                <StableTypeIdentifier as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "structural_type" => Ok(Self::StructuralType(::std::boxed::Box::new(
                <StructuralType as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "tuple_type" => Ok(Self::TupleType(::std::boxed::Box::new(
                <TupleType as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "type_identifier" => Ok(Self::TypeIdentifier(::std::boxed::Box::new(
                <TypeIdentifier as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "type_lambda" => Ok(Self::TypeLambda(::std::boxed::Box::new(
                <TypeLambda as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "wildcard" => Ok(Self::Wildcard(::std::boxed::Box::new(
                <Wildcard as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            other => Err(::treesitter_types::ParseError::unexpected_kind(other, node)),
        }
    }
}
impl ::treesitter_types::Spanned for VarDeclarationType<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        match self {
            Self::AnnotatedType(inner) => inner.span(),
            Self::AppliedConstructorType(inner) => inner.span(),
            Self::CompoundType(inner) => inner.span(),
            Self::FunctionType(inner) => inner.span(),
            Self::GenericType(inner) => inner.span(),
            Self::InfixType(inner) => inner.span(),
            Self::LiteralType(inner) => inner.span(),
            Self::MatchType(inner) => inner.span(),
            Self::NamedTupleType(inner) => inner.span(),
            Self::ProjectedType(inner) => inner.span(),
            Self::SingletonType(inner) => inner.span(),
            Self::StableTypeIdentifier(inner) => inner.span(),
            Self::StructuralType(inner) => inner.span(),
            Self::TupleType(inner) => inner.span(),
            Self::TypeIdentifier(inner) => inner.span(),
            Self::TypeLambda(inner) => inner.span(),
            Self::Wildcard(inner) => inner.span(),
        }
    }
}
#[derive(Debug, Clone)]
pub enum VarDeclarationChildren<'tree> {
    Annotation(::std::boxed::Box<Annotation<'tree>>),
    Modifiers(::std::boxed::Box<Modifiers<'tree>>),
}
impl<'tree> ::treesitter_types::FromNode<'tree> for VarDeclarationChildren<'tree> {
    #[allow(clippy::collapsible_else_if)]
    fn from_node(
        node: ::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        match node.kind() {
            "annotation" => Ok(Self::Annotation(::std::boxed::Box::new(
                <Annotation as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "modifiers" => Ok(Self::Modifiers(::std::boxed::Box::new(
                <Modifiers as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            other => Err(::treesitter_types::ParseError::unexpected_kind(other, node)),
        }
    }
}
impl ::treesitter_types::Spanned for VarDeclarationChildren<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        match self {
            Self::Annotation(inner) => inner.span(),
            Self::Modifiers(inner) => inner.span(),
        }
    }
}
#[derive(Debug, Clone)]
pub enum VarDefinitionPattern<'tree> {
    Pattern(::std::boxed::Box<Pattern<'tree>>),
    Identifiers(::std::boxed::Box<Identifiers<'tree>>),
}
impl<'tree> ::treesitter_types::FromNode<'tree> for VarDefinitionPattern<'tree> {
    #[allow(clippy::collapsible_else_if)]
    fn from_node(
        node: ::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        match node.kind() {
            "identifiers" => Ok(Self::Identifiers(::std::boxed::Box::new(
                <Identifiers as ::treesitter_types::FromNode>::from_node(node, src)?,
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
impl ::treesitter_types::Spanned for VarDefinitionPattern<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        match self {
            Self::Pattern(inner) => inner.span(),
            Self::Identifiers(inner) => inner.span(),
        }
    }
}
#[derive(Debug, Clone)]
pub enum VarDefinitionType<'tree> {
    AnnotatedType(::std::boxed::Box<AnnotatedType<'tree>>),
    AppliedConstructorType(::std::boxed::Box<AppliedConstructorType<'tree>>),
    CompoundType(::std::boxed::Box<CompoundType<'tree>>),
    FunctionType(::std::boxed::Box<FunctionType<'tree>>),
    GenericType(::std::boxed::Box<GenericType<'tree>>),
    InfixType(::std::boxed::Box<InfixType<'tree>>),
    LiteralType(::std::boxed::Box<LiteralType<'tree>>),
    MatchType(::std::boxed::Box<MatchType<'tree>>),
    NamedTupleType(::std::boxed::Box<NamedTupleType<'tree>>),
    ProjectedType(::std::boxed::Box<ProjectedType<'tree>>),
    SingletonType(::std::boxed::Box<SingletonType<'tree>>),
    StableTypeIdentifier(::std::boxed::Box<StableTypeIdentifier<'tree>>),
    StructuralType(::std::boxed::Box<StructuralType<'tree>>),
    TupleType(::std::boxed::Box<TupleType<'tree>>),
    TypeIdentifier(::std::boxed::Box<TypeIdentifier<'tree>>),
    TypeLambda(::std::boxed::Box<TypeLambda<'tree>>),
    Wildcard(::std::boxed::Box<Wildcard<'tree>>),
}
impl<'tree> ::treesitter_types::FromNode<'tree> for VarDefinitionType<'tree> {
    #[allow(clippy::collapsible_else_if)]
    fn from_node(
        node: ::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        match node.kind() {
            "annotated_type" => Ok(Self::AnnotatedType(::std::boxed::Box::new(
                <AnnotatedType as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "applied_constructor_type" => Ok(Self::AppliedConstructorType(::std::boxed::Box::new(
                <AppliedConstructorType as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "compound_type" => Ok(Self::CompoundType(::std::boxed::Box::new(
                <CompoundType as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "function_type" => Ok(Self::FunctionType(::std::boxed::Box::new(
                <FunctionType as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "generic_type" => Ok(Self::GenericType(::std::boxed::Box::new(
                <GenericType as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "infix_type" => Ok(Self::InfixType(::std::boxed::Box::new(
                <InfixType as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "literal_type" => Ok(Self::LiteralType(::std::boxed::Box::new(
                <LiteralType as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "match_type" => Ok(Self::MatchType(::std::boxed::Box::new(
                <MatchType as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "named_tuple_type" => Ok(Self::NamedTupleType(::std::boxed::Box::new(
                <NamedTupleType as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "projected_type" => Ok(Self::ProjectedType(::std::boxed::Box::new(
                <ProjectedType as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "singleton_type" => Ok(Self::SingletonType(::std::boxed::Box::new(
                <SingletonType as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "stable_type_identifier" => Ok(Self::StableTypeIdentifier(::std::boxed::Box::new(
                <StableTypeIdentifier as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "structural_type" => Ok(Self::StructuralType(::std::boxed::Box::new(
                <StructuralType as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "tuple_type" => Ok(Self::TupleType(::std::boxed::Box::new(
                <TupleType as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "type_identifier" => Ok(Self::TypeIdentifier(::std::boxed::Box::new(
                <TypeIdentifier as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "type_lambda" => Ok(Self::TypeLambda(::std::boxed::Box::new(
                <TypeLambda as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "wildcard" => Ok(Self::Wildcard(::std::boxed::Box::new(
                <Wildcard as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            other => Err(::treesitter_types::ParseError::unexpected_kind(other, node)),
        }
    }
}
impl ::treesitter_types::Spanned for VarDefinitionType<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        match self {
            Self::AnnotatedType(inner) => inner.span(),
            Self::AppliedConstructorType(inner) => inner.span(),
            Self::CompoundType(inner) => inner.span(),
            Self::FunctionType(inner) => inner.span(),
            Self::GenericType(inner) => inner.span(),
            Self::InfixType(inner) => inner.span(),
            Self::LiteralType(inner) => inner.span(),
            Self::MatchType(inner) => inner.span(),
            Self::NamedTupleType(inner) => inner.span(),
            Self::ProjectedType(inner) => inner.span(),
            Self::SingletonType(inner) => inner.span(),
            Self::StableTypeIdentifier(inner) => inner.span(),
            Self::StructuralType(inner) => inner.span(),
            Self::TupleType(inner) => inner.span(),
            Self::TypeIdentifier(inner) => inner.span(),
            Self::TypeLambda(inner) => inner.span(),
            Self::Wildcard(inner) => inner.span(),
        }
    }
}
#[derive(Debug, Clone)]
pub enum VarDefinitionValue<'tree> {
    Expression(::std::boxed::Box<Expression<'tree>>),
    IndentedBlock(::std::boxed::Box<IndentedBlock<'tree>>),
    IndentedCases(::std::boxed::Box<IndentedCases<'tree>>),
}
impl<'tree> ::treesitter_types::FromNode<'tree> for VarDefinitionValue<'tree> {
    #[allow(clippy::collapsible_else_if)]
    fn from_node(
        node: ::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        match node.kind() {
            "indented_block" => Ok(Self::IndentedBlock(::std::boxed::Box::new(
                <IndentedBlock as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "indented_cases" => Ok(Self::IndentedCases(::std::boxed::Box::new(
                <IndentedCases as ::treesitter_types::FromNode>::from_node(node, src)?,
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
impl ::treesitter_types::Spanned for VarDefinitionValue<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        match self {
            Self::Expression(inner) => inner.span(),
            Self::IndentedBlock(inner) => inner.span(),
            Self::IndentedCases(inner) => inner.span(),
        }
    }
}
#[derive(Debug, Clone)]
pub enum VarDefinitionChildren<'tree> {
    Annotation(::std::boxed::Box<Annotation<'tree>>),
    Modifiers(::std::boxed::Box<Modifiers<'tree>>),
}
impl<'tree> ::treesitter_types::FromNode<'tree> for VarDefinitionChildren<'tree> {
    #[allow(clippy::collapsible_else_if)]
    fn from_node(
        node: ::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        match node.kind() {
            "annotation" => Ok(Self::Annotation(::std::boxed::Box::new(
                <Annotation as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "modifiers" => Ok(Self::Modifiers(::std::boxed::Box::new(
                <Modifiers as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            other => Err(::treesitter_types::ParseError::unexpected_kind(other, node)),
        }
    }
}
impl ::treesitter_types::Spanned for VarDefinitionChildren<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        match self {
            Self::Annotation(inner) => inner.span(),
            Self::Modifiers(inner) => inner.span(),
        }
    }
}
#[derive(Debug, Clone)]
pub enum ViewBoundType<'tree> {
    AnnotatedType(::std::boxed::Box<AnnotatedType<'tree>>),
    AppliedConstructorType(::std::boxed::Box<AppliedConstructorType<'tree>>),
    CompoundType(::std::boxed::Box<CompoundType<'tree>>),
    FunctionType(::std::boxed::Box<FunctionType<'tree>>),
    GenericType(::std::boxed::Box<GenericType<'tree>>),
    InfixType(::std::boxed::Box<InfixType<'tree>>),
    LiteralType(::std::boxed::Box<LiteralType<'tree>>),
    MatchType(::std::boxed::Box<MatchType<'tree>>),
    NamedTupleType(::std::boxed::Box<NamedTupleType<'tree>>),
    ProjectedType(::std::boxed::Box<ProjectedType<'tree>>),
    SingletonType(::std::boxed::Box<SingletonType<'tree>>),
    StableTypeIdentifier(::std::boxed::Box<StableTypeIdentifier<'tree>>),
    StructuralType(::std::boxed::Box<StructuralType<'tree>>),
    TupleType(::std::boxed::Box<TupleType<'tree>>),
    TypeIdentifier(::std::boxed::Box<TypeIdentifier<'tree>>),
    TypeLambda(::std::boxed::Box<TypeLambda<'tree>>),
    Wildcard(::std::boxed::Box<Wildcard<'tree>>),
}
impl<'tree> ::treesitter_types::FromNode<'tree> for ViewBoundType<'tree> {
    #[allow(clippy::collapsible_else_if)]
    fn from_node(
        node: ::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        match node.kind() {
            "annotated_type" => Ok(Self::AnnotatedType(::std::boxed::Box::new(
                <AnnotatedType as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "applied_constructor_type" => Ok(Self::AppliedConstructorType(::std::boxed::Box::new(
                <AppliedConstructorType as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "compound_type" => Ok(Self::CompoundType(::std::boxed::Box::new(
                <CompoundType as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "function_type" => Ok(Self::FunctionType(::std::boxed::Box::new(
                <FunctionType as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "generic_type" => Ok(Self::GenericType(::std::boxed::Box::new(
                <GenericType as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "infix_type" => Ok(Self::InfixType(::std::boxed::Box::new(
                <InfixType as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "literal_type" => Ok(Self::LiteralType(::std::boxed::Box::new(
                <LiteralType as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "match_type" => Ok(Self::MatchType(::std::boxed::Box::new(
                <MatchType as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "named_tuple_type" => Ok(Self::NamedTupleType(::std::boxed::Box::new(
                <NamedTupleType as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "projected_type" => Ok(Self::ProjectedType(::std::boxed::Box::new(
                <ProjectedType as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "singleton_type" => Ok(Self::SingletonType(::std::boxed::Box::new(
                <SingletonType as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "stable_type_identifier" => Ok(Self::StableTypeIdentifier(::std::boxed::Box::new(
                <StableTypeIdentifier as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "structural_type" => Ok(Self::StructuralType(::std::boxed::Box::new(
                <StructuralType as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "tuple_type" => Ok(Self::TupleType(::std::boxed::Box::new(
                <TupleType as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "type_identifier" => Ok(Self::TypeIdentifier(::std::boxed::Box::new(
                <TypeIdentifier as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "type_lambda" => Ok(Self::TypeLambda(::std::boxed::Box::new(
                <TypeLambda as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "wildcard" => Ok(Self::Wildcard(::std::boxed::Box::new(
                <Wildcard as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            other => Err(::treesitter_types::ParseError::unexpected_kind(other, node)),
        }
    }
}
impl ::treesitter_types::Spanned for ViewBoundType<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        match self {
            Self::AnnotatedType(inner) => inner.span(),
            Self::AppliedConstructorType(inner) => inner.span(),
            Self::CompoundType(inner) => inner.span(),
            Self::FunctionType(inner) => inner.span(),
            Self::GenericType(inner) => inner.span(),
            Self::InfixType(inner) => inner.span(),
            Self::LiteralType(inner) => inner.span(),
            Self::MatchType(inner) => inner.span(),
            Self::NamedTupleType(inner) => inner.span(),
            Self::ProjectedType(inner) => inner.span(),
            Self::SingletonType(inner) => inner.span(),
            Self::StableTypeIdentifier(inner) => inner.span(),
            Self::StructuralType(inner) => inner.span(),
            Self::TupleType(inner) => inner.span(),
            Self::TypeIdentifier(inner) => inner.span(),
            Self::TypeLambda(inner) => inner.span(),
            Self::Wildcard(inner) => inner.span(),
        }
    }
}
#[derive(Debug, Clone)]
pub enum WhileExpressionBody<'tree> {
    Expression(::std::boxed::Box<Expression<'tree>>),
    IndentedBlock(::std::boxed::Box<IndentedBlock<'tree>>),
    IndentedCases(::std::boxed::Box<IndentedCases<'tree>>),
}
impl<'tree> ::treesitter_types::FromNode<'tree> for WhileExpressionBody<'tree> {
    #[allow(clippy::collapsible_else_if)]
    fn from_node(
        node: ::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        match node.kind() {
            "indented_block" => Ok(Self::IndentedBlock(::std::boxed::Box::new(
                <IndentedBlock as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "indented_cases" => Ok(Self::IndentedCases(::std::boxed::Box::new(
                <IndentedCases as ::treesitter_types::FromNode>::from_node(node, src)?,
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
impl ::treesitter_types::Spanned for WhileExpressionBody<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        match self {
            Self::Expression(inner) => inner.span(),
            Self::IndentedBlock(inner) => inner.span(),
            Self::IndentedCases(inner) => inner.span(),
        }
    }
}
#[derive(Debug, Clone)]
pub enum WhileExpressionCondition<'tree> {
    Do(::treesitter_types::Span),
    Expression(::std::boxed::Box<Expression<'tree>>),
    IndentedBlock(::std::boxed::Box<IndentedBlock<'tree>>),
    IndentedCases(::std::boxed::Box<IndentedCases<'tree>>),
}
impl<'tree> ::treesitter_types::FromNode<'tree> for WhileExpressionCondition<'tree> {
    #[allow(clippy::collapsible_else_if)]
    fn from_node(
        node: ::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        match node.kind() {
            "do" => Ok(Self::Do(::treesitter_types::Span::from(node))),
            "indented_block" => Ok(Self::IndentedBlock(::std::boxed::Box::new(
                <IndentedBlock as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "indented_cases" => Ok(Self::IndentedCases(::std::boxed::Box::new(
                <IndentedCases as ::treesitter_types::FromNode>::from_node(node, src)?,
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
impl ::treesitter_types::Spanned for WhileExpressionCondition<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        match self {
            Self::Do(span) => *span,
            Self::Expression(inner) => inner.span(),
            Self::IndentedBlock(inner) => inner.span(),
            Self::IndentedCases(inner) => inner.span(),
        }
    }
}
#[derive(Debug, Clone)]
pub enum WithTemplateBodyChildren<'tree> {
    Definition(::std::boxed::Box<Definition<'tree>>),
    Expression(::std::boxed::Box<Expression<'tree>>),
    SelfType(::std::boxed::Box<SelfType<'tree>>),
}
impl<'tree> ::treesitter_types::FromNode<'tree> for WithTemplateBodyChildren<'tree> {
    #[allow(clippy::collapsible_else_if)]
    fn from_node(
        node: ::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        match node.kind() {
            "self_type" => Ok(Self::SelfType(::std::boxed::Box::new(
                <SelfType as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            _other => {
                if let Ok(v) = <Definition as ::treesitter_types::FromNode>::from_node(node, src) {
                    Ok(Self::Definition(::std::boxed::Box::new(v)))
                } else {
                    if let Ok(v) =
                        <Expression as ::treesitter_types::FromNode>::from_node(node, src)
                    {
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
}
impl ::treesitter_types::Spanned for WithTemplateBodyChildren<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        match self {
            Self::Definition(inner) => inner.span(),
            Self::Expression(inner) => inner.span(),
            Self::SelfType(inner) => inner.span(),
        }
    }
}
#[derive(Debug, Clone)]
pub enum AnyNode<'tree> {
    Definition(Definition<'tree>),
    Pattern(Pattern<'tree>),
    Expression(Expression<'tree>),
    AccessModifier(AccessModifier<'tree>),
    AccessQualifier(AccessQualifier<'tree>),
    AlternativePattern(AlternativePattern<'tree>),
    AnnotatedType(AnnotatedType<'tree>),
    Annotation(Annotation<'tree>),
    AppliedConstructorType(AppliedConstructorType<'tree>),
    Arguments(Arguments<'tree>),
    ArrowRenamedIdentifier(ArrowRenamedIdentifier<'tree>),
    AsRenamedIdentifier(AsRenamedIdentifier<'tree>),
    AscriptionExpression(AscriptionExpression<'tree>),
    AssignmentExpression(AssignmentExpression<'tree>),
    Binding(Binding<'tree>),
    Bindings(Bindings<'tree>),
    Block(Block<'tree>),
    BlockComment(BlockComment<'tree>),
    BooleanLiteral(BooleanLiteral<'tree>),
    CallExpression(CallExpression<'tree>),
    CapturePattern(CapturePattern<'tree>),
    CaseBlock(CaseBlock<'tree>),
    CaseClassPattern(CaseClassPattern<'tree>),
    CaseClause(CaseClause<'tree>),
    CatchClause(CatchClause<'tree>),
    ClassDefinition(ClassDefinition<'tree>),
    ClassParameter(ClassParameter<'tree>),
    ClassParameters(ClassParameters<'tree>),
    ColonArgument(ColonArgument<'tree>),
    Comment(Comment<'tree>),
    CompilationUnit(CompilationUnit<'tree>),
    CompoundType(CompoundType<'tree>),
    ContextBound(ContextBound<'tree>),
    ContravariantTypeParameter(ContravariantTypeParameter<'tree>),
    CovariantTypeParameter(CovariantTypeParameter<'tree>),
    DerivesClause(DerivesClause<'tree>),
    DoWhileExpression(DoWhileExpression<'tree>),
    EnumBody(EnumBody<'tree>),
    EnumCaseDefinitions(EnumCaseDefinitions<'tree>),
    EnumDefinition(EnumDefinition<'tree>),
    Enumerator(Enumerator<'tree>),
    Enumerators(Enumerators<'tree>),
    ExportDeclaration(ExportDeclaration<'tree>),
    ExtendsClause(ExtendsClause<'tree>),
    ExtensionDefinition(ExtensionDefinition<'tree>),
    FieldExpression(FieldExpression<'tree>),
    FinallyClause(FinallyClause<'tree>),
    ForExpression(ForExpression<'tree>),
    FullEnumCase(FullEnumCase<'tree>),
    FunctionDeclaration(FunctionDeclaration<'tree>),
    FunctionDefinition(FunctionDefinition<'tree>),
    FunctionType(FunctionType<'tree>),
    GenericFunction(GenericFunction<'tree>),
    GenericType(GenericType<'tree>),
    GivenConditional(GivenConditional<'tree>),
    GivenDefinition(GivenDefinition<'tree>),
    GivenPattern(GivenPattern<'tree>),
    Guard(Guard<'tree>),
    Identifier(Identifier<'tree>),
    Identifiers(Identifiers<'tree>),
    IfExpression(IfExpression<'tree>),
    ImportDeclaration(ImportDeclaration<'tree>),
    IndentedBlock(IndentedBlock<'tree>),
    IndentedCases(IndentedCases<'tree>),
    InfixExpression(InfixExpression<'tree>),
    InfixModifier(InfixModifier<'tree>),
    InfixPattern(InfixPattern<'tree>),
    InfixType(InfixType<'tree>),
    InlineModifier(InlineModifier<'tree>),
    InstanceExpression(InstanceExpression<'tree>),
    InterpolatedString(InterpolatedString<'tree>),
    InterpolatedStringExpression(InterpolatedStringExpression<'tree>),
    Interpolation(Interpolation<'tree>),
    IntoModifier(IntoModifier<'tree>),
    LambdaExpression(LambdaExpression<'tree>),
    LazyParameterType(LazyParameterType<'tree>),
    LiteralType(LiteralType<'tree>),
    LowerBound(LowerBound<'tree>),
    MacroBody(MacroBody<'tree>),
    MatchExpression(MatchExpression<'tree>),
    MatchType(MatchType<'tree>),
    Modifiers(Modifiers<'tree>),
    NameAndType(NameAndType<'tree>),
    NamedPattern(NamedPattern<'tree>),
    NamedTuplePattern(NamedTuplePattern<'tree>),
    NamedTupleType(NamedTupleType<'tree>),
    NamespaceSelectors(NamespaceSelectors<'tree>),
    NamespaceWildcard(NamespaceWildcard<'tree>),
    ObjectDefinition(ObjectDefinition<'tree>),
    OpaqueModifier(OpaqueModifier<'tree>),
    OpenModifier(OpenModifier<'tree>),
    PackageClause(PackageClause<'tree>),
    PackageIdentifier(PackageIdentifier<'tree>),
    PackageObject(PackageObject<'tree>),
    Parameter(Parameter<'tree>),
    ParameterTypes(ParameterTypes<'tree>),
    Parameters(Parameters<'tree>),
    ParenthesizedExpression(ParenthesizedExpression<'tree>),
    PostfixExpression(PostfixExpression<'tree>),
    PrefixExpression(PrefixExpression<'tree>),
    ProjectedType(ProjectedType<'tree>),
    QuoteExpression(QuoteExpression<'tree>),
    Refinement(Refinement<'tree>),
    RepeatPattern(RepeatPattern<'tree>),
    RepeatedParameterType(RepeatedParameterType<'tree>),
    ReturnExpression(ReturnExpression<'tree>),
    SelfType(SelfType<'tree>),
    SimpleEnumCase(SimpleEnumCase<'tree>),
    SingletonType(SingletonType<'tree>),
    SpliceExpression(SpliceExpression<'tree>),
    StableIdentifier(StableIdentifier<'tree>),
    StableTypeIdentifier(StableTypeIdentifier<'tree>),
    String(String<'tree>),
    StructuralType(StructuralType<'tree>),
    TemplateBody(TemplateBody<'tree>),
    ThrowExpression(ThrowExpression<'tree>),
    TrackedModifier(TrackedModifier<'tree>),
    TraitDefinition(TraitDefinition<'tree>),
    TransparentModifier(TransparentModifier<'tree>),
    TryExpression(TryExpression<'tree>),
    TupleExpression(TupleExpression<'tree>),
    TuplePattern(TuplePattern<'tree>),
    TupleType(TupleType<'tree>),
    TypeArguments(TypeArguments<'tree>),
    TypeCaseClause(TypeCaseClause<'tree>),
    TypeDefinition(TypeDefinition<'tree>),
    TypeLambda(TypeLambda<'tree>),
    TypeParameters(TypeParameters<'tree>),
    TypedPattern(TypedPattern<'tree>),
    Unit(Unit<'tree>),
    UpperBound(UpperBound<'tree>),
    UsingDirective(UsingDirective<'tree>),
    ValDeclaration(ValDeclaration<'tree>),
    ValDefinition(ValDefinition<'tree>),
    VarDeclaration(VarDeclaration<'tree>),
    VarDefinition(VarDefinition<'tree>),
    ViewBound(ViewBound<'tree>),
    WhileExpression(WhileExpression<'tree>),
    Wildcard(Wildcard<'tree>),
    WithTemplateBody(WithTemplateBody<'tree>),
    CharacterLiteral(CharacterLiteral<'tree>),
    EscapeSequence(EscapeSequence<'tree>),
    FloatingPointLiteral(FloatingPointLiteral<'tree>),
    IntegerLiteral(IntegerLiteral<'tree>),
    NullLiteral(NullLiteral<'tree>),
    OperatorIdentifier(OperatorIdentifier<'tree>),
    UsingDirectiveKey(UsingDirectiveKey<'tree>),
    UsingDirectiveValue(UsingDirectiveValue<'tree>),
    TypeIdentifier(TypeIdentifier<'tree>),
    Unknown(::tree_sitter::Node<'tree>),
}
impl<'tree> AnyNode<'tree> {
    pub fn from_node(node: ::tree_sitter::Node<'tree>, src: &'tree [u8]) -> Self {
        match node.kind() {
            "_definition" => <Definition as ::treesitter_types::FromNode>::from_node(node, src)
                .map(Self::Definition)
                .unwrap_or(Self::Unknown(node)),
            "_pattern" => <Pattern as ::treesitter_types::FromNode>::from_node(node, src)
                .map(Self::Pattern)
                .unwrap_or(Self::Unknown(node)),
            "expression" => <Expression as ::treesitter_types::FromNode>::from_node(node, src)
                .map(Self::Expression)
                .unwrap_or(Self::Unknown(node)),
            "access_modifier" => {
                <AccessModifier as ::treesitter_types::FromNode>::from_node(node, src)
                    .map(Self::AccessModifier)
                    .unwrap_or(Self::Unknown(node))
            }
            "access_qualifier" => {
                <AccessQualifier as ::treesitter_types::FromNode>::from_node(node, src)
                    .map(Self::AccessQualifier)
                    .unwrap_or(Self::Unknown(node))
            }
            "alternative_pattern" => {
                <AlternativePattern as ::treesitter_types::FromNode>::from_node(node, src)
                    .map(Self::AlternativePattern)
                    .unwrap_or(Self::Unknown(node))
            }
            "annotated_type" => {
                <AnnotatedType as ::treesitter_types::FromNode>::from_node(node, src)
                    .map(Self::AnnotatedType)
                    .unwrap_or(Self::Unknown(node))
            }
            "annotation" => <Annotation as ::treesitter_types::FromNode>::from_node(node, src)
                .map(Self::Annotation)
                .unwrap_or(Self::Unknown(node)),
            "applied_constructor_type" => {
                <AppliedConstructorType as ::treesitter_types::FromNode>::from_node(node, src)
                    .map(Self::AppliedConstructorType)
                    .unwrap_or(Self::Unknown(node))
            }
            "arguments" => <Arguments as ::treesitter_types::FromNode>::from_node(node, src)
                .map(Self::Arguments)
                .unwrap_or(Self::Unknown(node)),
            "arrow_renamed_identifier" => {
                <ArrowRenamedIdentifier as ::treesitter_types::FromNode>::from_node(node, src)
                    .map(Self::ArrowRenamedIdentifier)
                    .unwrap_or(Self::Unknown(node))
            }
            "as_renamed_identifier" => {
                <AsRenamedIdentifier as ::treesitter_types::FromNode>::from_node(node, src)
                    .map(Self::AsRenamedIdentifier)
                    .unwrap_or(Self::Unknown(node))
            }
            "ascription_expression" => {
                <AscriptionExpression as ::treesitter_types::FromNode>::from_node(node, src)
                    .map(Self::AscriptionExpression)
                    .unwrap_or(Self::Unknown(node))
            }
            "assignment_expression" => {
                <AssignmentExpression as ::treesitter_types::FromNode>::from_node(node, src)
                    .map(Self::AssignmentExpression)
                    .unwrap_or(Self::Unknown(node))
            }
            "binding" => <Binding as ::treesitter_types::FromNode>::from_node(node, src)
                .map(Self::Binding)
                .unwrap_or(Self::Unknown(node)),
            "bindings" => <Bindings as ::treesitter_types::FromNode>::from_node(node, src)
                .map(Self::Bindings)
                .unwrap_or(Self::Unknown(node)),
            "block" => <Block as ::treesitter_types::FromNode>::from_node(node, src)
                .map(Self::Block)
                .unwrap_or(Self::Unknown(node)),
            "block_comment" => <BlockComment as ::treesitter_types::FromNode>::from_node(node, src)
                .map(Self::BlockComment)
                .unwrap_or(Self::Unknown(node)),
            "boolean_literal" => {
                <BooleanLiteral as ::treesitter_types::FromNode>::from_node(node, src)
                    .map(Self::BooleanLiteral)
                    .unwrap_or(Self::Unknown(node))
            }
            "call_expression" => {
                <CallExpression as ::treesitter_types::FromNode>::from_node(node, src)
                    .map(Self::CallExpression)
                    .unwrap_or(Self::Unknown(node))
            }
            "capture_pattern" => {
                <CapturePattern as ::treesitter_types::FromNode>::from_node(node, src)
                    .map(Self::CapturePattern)
                    .unwrap_or(Self::Unknown(node))
            }
            "case_block" => <CaseBlock as ::treesitter_types::FromNode>::from_node(node, src)
                .map(Self::CaseBlock)
                .unwrap_or(Self::Unknown(node)),
            "case_class_pattern" => {
                <CaseClassPattern as ::treesitter_types::FromNode>::from_node(node, src)
                    .map(Self::CaseClassPattern)
                    .unwrap_or(Self::Unknown(node))
            }
            "case_clause" => <CaseClause as ::treesitter_types::FromNode>::from_node(node, src)
                .map(Self::CaseClause)
                .unwrap_or(Self::Unknown(node)),
            "catch_clause" => <CatchClause as ::treesitter_types::FromNode>::from_node(node, src)
                .map(Self::CatchClause)
                .unwrap_or(Self::Unknown(node)),
            "class_definition" => {
                <ClassDefinition as ::treesitter_types::FromNode>::from_node(node, src)
                    .map(Self::ClassDefinition)
                    .unwrap_or(Self::Unknown(node))
            }
            "class_parameter" => {
                <ClassParameter as ::treesitter_types::FromNode>::from_node(node, src)
                    .map(Self::ClassParameter)
                    .unwrap_or(Self::Unknown(node))
            }
            "class_parameters" => {
                <ClassParameters as ::treesitter_types::FromNode>::from_node(node, src)
                    .map(Self::ClassParameters)
                    .unwrap_or(Self::Unknown(node))
            }
            "colon_argument" => {
                <ColonArgument as ::treesitter_types::FromNode>::from_node(node, src)
                    .map(Self::ColonArgument)
                    .unwrap_or(Self::Unknown(node))
            }
            "comment" => <Comment as ::treesitter_types::FromNode>::from_node(node, src)
                .map(Self::Comment)
                .unwrap_or(Self::Unknown(node)),
            "compilation_unit" => {
                <CompilationUnit as ::treesitter_types::FromNode>::from_node(node, src)
                    .map(Self::CompilationUnit)
                    .unwrap_or(Self::Unknown(node))
            }
            "compound_type" => <CompoundType as ::treesitter_types::FromNode>::from_node(node, src)
                .map(Self::CompoundType)
                .unwrap_or(Self::Unknown(node)),
            "context_bound" => <ContextBound as ::treesitter_types::FromNode>::from_node(node, src)
                .map(Self::ContextBound)
                .unwrap_or(Self::Unknown(node)),
            "contravariant_type_parameter" => {
                <ContravariantTypeParameter as ::treesitter_types::FromNode>::from_node(node, src)
                    .map(Self::ContravariantTypeParameter)
                    .unwrap_or(Self::Unknown(node))
            }
            "covariant_type_parameter" => {
                <CovariantTypeParameter as ::treesitter_types::FromNode>::from_node(node, src)
                    .map(Self::CovariantTypeParameter)
                    .unwrap_or(Self::Unknown(node))
            }
            "derives_clause" => {
                <DerivesClause as ::treesitter_types::FromNode>::from_node(node, src)
                    .map(Self::DerivesClause)
                    .unwrap_or(Self::Unknown(node))
            }
            "do_while_expression" => {
                <DoWhileExpression as ::treesitter_types::FromNode>::from_node(node, src)
                    .map(Self::DoWhileExpression)
                    .unwrap_or(Self::Unknown(node))
            }
            "enum_body" => <EnumBody as ::treesitter_types::FromNode>::from_node(node, src)
                .map(Self::EnumBody)
                .unwrap_or(Self::Unknown(node)),
            "enum_case_definitions" => {
                <EnumCaseDefinitions as ::treesitter_types::FromNode>::from_node(node, src)
                    .map(Self::EnumCaseDefinitions)
                    .unwrap_or(Self::Unknown(node))
            }
            "enum_definition" => {
                <EnumDefinition as ::treesitter_types::FromNode>::from_node(node, src)
                    .map(Self::EnumDefinition)
                    .unwrap_or(Self::Unknown(node))
            }
            "enumerator" => <Enumerator as ::treesitter_types::FromNode>::from_node(node, src)
                .map(Self::Enumerator)
                .unwrap_or(Self::Unknown(node)),
            "enumerators" => <Enumerators as ::treesitter_types::FromNode>::from_node(node, src)
                .map(Self::Enumerators)
                .unwrap_or(Self::Unknown(node)),
            "export_declaration" => {
                <ExportDeclaration as ::treesitter_types::FromNode>::from_node(node, src)
                    .map(Self::ExportDeclaration)
                    .unwrap_or(Self::Unknown(node))
            }
            "extends_clause" => {
                <ExtendsClause as ::treesitter_types::FromNode>::from_node(node, src)
                    .map(Self::ExtendsClause)
                    .unwrap_or(Self::Unknown(node))
            }
            "extension_definition" => {
                <ExtensionDefinition as ::treesitter_types::FromNode>::from_node(node, src)
                    .map(Self::ExtensionDefinition)
                    .unwrap_or(Self::Unknown(node))
            }
            "field_expression" => {
                <FieldExpression as ::treesitter_types::FromNode>::from_node(node, src)
                    .map(Self::FieldExpression)
                    .unwrap_or(Self::Unknown(node))
            }
            "finally_clause" => {
                <FinallyClause as ::treesitter_types::FromNode>::from_node(node, src)
                    .map(Self::FinallyClause)
                    .unwrap_or(Self::Unknown(node))
            }
            "for_expression" => {
                <ForExpression as ::treesitter_types::FromNode>::from_node(node, src)
                    .map(Self::ForExpression)
                    .unwrap_or(Self::Unknown(node))
            }
            "full_enum_case" => {
                <FullEnumCase as ::treesitter_types::FromNode>::from_node(node, src)
                    .map(Self::FullEnumCase)
                    .unwrap_or(Self::Unknown(node))
            }
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
            "function_type" => <FunctionType as ::treesitter_types::FromNode>::from_node(node, src)
                .map(Self::FunctionType)
                .unwrap_or(Self::Unknown(node)),
            "generic_function" => {
                <GenericFunction as ::treesitter_types::FromNode>::from_node(node, src)
                    .map(Self::GenericFunction)
                    .unwrap_or(Self::Unknown(node))
            }
            "generic_type" => <GenericType as ::treesitter_types::FromNode>::from_node(node, src)
                .map(Self::GenericType)
                .unwrap_or(Self::Unknown(node)),
            "given_conditional" => {
                <GivenConditional as ::treesitter_types::FromNode>::from_node(node, src)
                    .map(Self::GivenConditional)
                    .unwrap_or(Self::Unknown(node))
            }
            "given_definition" => {
                <GivenDefinition as ::treesitter_types::FromNode>::from_node(node, src)
                    .map(Self::GivenDefinition)
                    .unwrap_or(Self::Unknown(node))
            }
            "given_pattern" => <GivenPattern as ::treesitter_types::FromNode>::from_node(node, src)
                .map(Self::GivenPattern)
                .unwrap_or(Self::Unknown(node)),
            "guard" => <Guard as ::treesitter_types::FromNode>::from_node(node, src)
                .map(Self::Guard)
                .unwrap_or(Self::Unknown(node)),
            "identifier" => <Identifier as ::treesitter_types::FromNode>::from_node(node, src)
                .map(Self::Identifier)
                .unwrap_or(Self::Unknown(node)),
            "identifiers" => <Identifiers as ::treesitter_types::FromNode>::from_node(node, src)
                .map(Self::Identifiers)
                .unwrap_or(Self::Unknown(node)),
            "if_expression" => <IfExpression as ::treesitter_types::FromNode>::from_node(node, src)
                .map(Self::IfExpression)
                .unwrap_or(Self::Unknown(node)),
            "import_declaration" => {
                <ImportDeclaration as ::treesitter_types::FromNode>::from_node(node, src)
                    .map(Self::ImportDeclaration)
                    .unwrap_or(Self::Unknown(node))
            }
            "indented_block" => {
                <IndentedBlock as ::treesitter_types::FromNode>::from_node(node, src)
                    .map(Self::IndentedBlock)
                    .unwrap_or(Self::Unknown(node))
            }
            "indented_cases" => {
                <IndentedCases as ::treesitter_types::FromNode>::from_node(node, src)
                    .map(Self::IndentedCases)
                    .unwrap_or(Self::Unknown(node))
            }
            "infix_expression" => {
                <InfixExpression as ::treesitter_types::FromNode>::from_node(node, src)
                    .map(Self::InfixExpression)
                    .unwrap_or(Self::Unknown(node))
            }
            "infix_modifier" => {
                <InfixModifier as ::treesitter_types::FromNode>::from_node(node, src)
                    .map(Self::InfixModifier)
                    .unwrap_or(Self::Unknown(node))
            }
            "infix_pattern" => <InfixPattern as ::treesitter_types::FromNode>::from_node(node, src)
                .map(Self::InfixPattern)
                .unwrap_or(Self::Unknown(node)),
            "infix_type" => <InfixType as ::treesitter_types::FromNode>::from_node(node, src)
                .map(Self::InfixType)
                .unwrap_or(Self::Unknown(node)),
            "inline_modifier" => {
                <InlineModifier as ::treesitter_types::FromNode>::from_node(node, src)
                    .map(Self::InlineModifier)
                    .unwrap_or(Self::Unknown(node))
            }
            "instance_expression" => {
                <InstanceExpression as ::treesitter_types::FromNode>::from_node(node, src)
                    .map(Self::InstanceExpression)
                    .unwrap_or(Self::Unknown(node))
            }
            "interpolated_string" => {
                <InterpolatedString as ::treesitter_types::FromNode>::from_node(node, src)
                    .map(Self::InterpolatedString)
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
            "into_modifier" => <IntoModifier as ::treesitter_types::FromNode>::from_node(node, src)
                .map(Self::IntoModifier)
                .unwrap_or(Self::Unknown(node)),
            "lambda_expression" => {
                <LambdaExpression as ::treesitter_types::FromNode>::from_node(node, src)
                    .map(Self::LambdaExpression)
                    .unwrap_or(Self::Unknown(node))
            }
            "lazy_parameter_type" => {
                <LazyParameterType as ::treesitter_types::FromNode>::from_node(node, src)
                    .map(Self::LazyParameterType)
                    .unwrap_or(Self::Unknown(node))
            }
            "literal_type" => <LiteralType as ::treesitter_types::FromNode>::from_node(node, src)
                .map(Self::LiteralType)
                .unwrap_or(Self::Unknown(node)),
            "lower_bound" => <LowerBound as ::treesitter_types::FromNode>::from_node(node, src)
                .map(Self::LowerBound)
                .unwrap_or(Self::Unknown(node)),
            "macro_body" => <MacroBody as ::treesitter_types::FromNode>::from_node(node, src)
                .map(Self::MacroBody)
                .unwrap_or(Self::Unknown(node)),
            "match_expression" => {
                <MatchExpression as ::treesitter_types::FromNode>::from_node(node, src)
                    .map(Self::MatchExpression)
                    .unwrap_or(Self::Unknown(node))
            }
            "match_type" => <MatchType as ::treesitter_types::FromNode>::from_node(node, src)
                .map(Self::MatchType)
                .unwrap_or(Self::Unknown(node)),
            "modifiers" => <Modifiers as ::treesitter_types::FromNode>::from_node(node, src)
                .map(Self::Modifiers)
                .unwrap_or(Self::Unknown(node)),
            "name_and_type" => <NameAndType as ::treesitter_types::FromNode>::from_node(node, src)
                .map(Self::NameAndType)
                .unwrap_or(Self::Unknown(node)),
            "named_pattern" => <NamedPattern as ::treesitter_types::FromNode>::from_node(node, src)
                .map(Self::NamedPattern)
                .unwrap_or(Self::Unknown(node)),
            "named_tuple_pattern" => {
                <NamedTuplePattern as ::treesitter_types::FromNode>::from_node(node, src)
                    .map(Self::NamedTuplePattern)
                    .unwrap_or(Self::Unknown(node))
            }
            "named_tuple_type" => {
                <NamedTupleType as ::treesitter_types::FromNode>::from_node(node, src)
                    .map(Self::NamedTupleType)
                    .unwrap_or(Self::Unknown(node))
            }
            "namespace_selectors" => {
                <NamespaceSelectors as ::treesitter_types::FromNode>::from_node(node, src)
                    .map(Self::NamespaceSelectors)
                    .unwrap_or(Self::Unknown(node))
            }
            "namespace_wildcard" => {
                <NamespaceWildcard as ::treesitter_types::FromNode>::from_node(node, src)
                    .map(Self::NamespaceWildcard)
                    .unwrap_or(Self::Unknown(node))
            }
            "object_definition" => {
                <ObjectDefinition as ::treesitter_types::FromNode>::from_node(node, src)
                    .map(Self::ObjectDefinition)
                    .unwrap_or(Self::Unknown(node))
            }
            "opaque_modifier" => {
                <OpaqueModifier as ::treesitter_types::FromNode>::from_node(node, src)
                    .map(Self::OpaqueModifier)
                    .unwrap_or(Self::Unknown(node))
            }
            "open_modifier" => <OpenModifier as ::treesitter_types::FromNode>::from_node(node, src)
                .map(Self::OpenModifier)
                .unwrap_or(Self::Unknown(node)),
            "package_clause" => {
                <PackageClause as ::treesitter_types::FromNode>::from_node(node, src)
                    .map(Self::PackageClause)
                    .unwrap_or(Self::Unknown(node))
            }
            "package_identifier" => {
                <PackageIdentifier as ::treesitter_types::FromNode>::from_node(node, src)
                    .map(Self::PackageIdentifier)
                    .unwrap_or(Self::Unknown(node))
            }
            "package_object" => {
                <PackageObject as ::treesitter_types::FromNode>::from_node(node, src)
                    .map(Self::PackageObject)
                    .unwrap_or(Self::Unknown(node))
            }
            "parameter" => <Parameter as ::treesitter_types::FromNode>::from_node(node, src)
                .map(Self::Parameter)
                .unwrap_or(Self::Unknown(node)),
            "parameter_types" => {
                <ParameterTypes as ::treesitter_types::FromNode>::from_node(node, src)
                    .map(Self::ParameterTypes)
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
            "postfix_expression" => {
                <PostfixExpression as ::treesitter_types::FromNode>::from_node(node, src)
                    .map(Self::PostfixExpression)
                    .unwrap_or(Self::Unknown(node))
            }
            "prefix_expression" => {
                <PrefixExpression as ::treesitter_types::FromNode>::from_node(node, src)
                    .map(Self::PrefixExpression)
                    .unwrap_or(Self::Unknown(node))
            }
            "projected_type" => {
                <ProjectedType as ::treesitter_types::FromNode>::from_node(node, src)
                    .map(Self::ProjectedType)
                    .unwrap_or(Self::Unknown(node))
            }
            "quote_expression" => {
                <QuoteExpression as ::treesitter_types::FromNode>::from_node(node, src)
                    .map(Self::QuoteExpression)
                    .unwrap_or(Self::Unknown(node))
            }
            "refinement" => <Refinement as ::treesitter_types::FromNode>::from_node(node, src)
                .map(Self::Refinement)
                .unwrap_or(Self::Unknown(node)),
            "repeat_pattern" => {
                <RepeatPattern as ::treesitter_types::FromNode>::from_node(node, src)
                    .map(Self::RepeatPattern)
                    .unwrap_or(Self::Unknown(node))
            }
            "repeated_parameter_type" => {
                <RepeatedParameterType as ::treesitter_types::FromNode>::from_node(node, src)
                    .map(Self::RepeatedParameterType)
                    .unwrap_or(Self::Unknown(node))
            }
            "return_expression" => {
                <ReturnExpression as ::treesitter_types::FromNode>::from_node(node, src)
                    .map(Self::ReturnExpression)
                    .unwrap_or(Self::Unknown(node))
            }
            "self_type" => <SelfType as ::treesitter_types::FromNode>::from_node(node, src)
                .map(Self::SelfType)
                .unwrap_or(Self::Unknown(node)),
            "simple_enum_case" => {
                <SimpleEnumCase as ::treesitter_types::FromNode>::from_node(node, src)
                    .map(Self::SimpleEnumCase)
                    .unwrap_or(Self::Unknown(node))
            }
            "singleton_type" => {
                <SingletonType as ::treesitter_types::FromNode>::from_node(node, src)
                    .map(Self::SingletonType)
                    .unwrap_or(Self::Unknown(node))
            }
            "splice_expression" => {
                <SpliceExpression as ::treesitter_types::FromNode>::from_node(node, src)
                    .map(Self::SpliceExpression)
                    .unwrap_or(Self::Unknown(node))
            }
            "stable_identifier" => {
                <StableIdentifier as ::treesitter_types::FromNode>::from_node(node, src)
                    .map(Self::StableIdentifier)
                    .unwrap_or(Self::Unknown(node))
            }
            "stable_type_identifier" => {
                <StableTypeIdentifier as ::treesitter_types::FromNode>::from_node(node, src)
                    .map(Self::StableTypeIdentifier)
                    .unwrap_or(Self::Unknown(node))
            }
            "string" => <String as ::treesitter_types::FromNode>::from_node(node, src)
                .map(Self::String)
                .unwrap_or(Self::Unknown(node)),
            "structural_type" => {
                <StructuralType as ::treesitter_types::FromNode>::from_node(node, src)
                    .map(Self::StructuralType)
                    .unwrap_or(Self::Unknown(node))
            }
            "template_body" => <TemplateBody as ::treesitter_types::FromNode>::from_node(node, src)
                .map(Self::TemplateBody)
                .unwrap_or(Self::Unknown(node)),
            "throw_expression" => {
                <ThrowExpression as ::treesitter_types::FromNode>::from_node(node, src)
                    .map(Self::ThrowExpression)
                    .unwrap_or(Self::Unknown(node))
            }
            "tracked_modifier" => {
                <TrackedModifier as ::treesitter_types::FromNode>::from_node(node, src)
                    .map(Self::TrackedModifier)
                    .unwrap_or(Self::Unknown(node))
            }
            "trait_definition" => {
                <TraitDefinition as ::treesitter_types::FromNode>::from_node(node, src)
                    .map(Self::TraitDefinition)
                    .unwrap_or(Self::Unknown(node))
            }
            "transparent_modifier" => {
                <TransparentModifier as ::treesitter_types::FromNode>::from_node(node, src)
                    .map(Self::TransparentModifier)
                    .unwrap_or(Self::Unknown(node))
            }
            "try_expression" => {
                <TryExpression as ::treesitter_types::FromNode>::from_node(node, src)
                    .map(Self::TryExpression)
                    .unwrap_or(Self::Unknown(node))
            }
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
            "type_arguments" => {
                <TypeArguments as ::treesitter_types::FromNode>::from_node(node, src)
                    .map(Self::TypeArguments)
                    .unwrap_or(Self::Unknown(node))
            }
            "type_case_clause" => {
                <TypeCaseClause as ::treesitter_types::FromNode>::from_node(node, src)
                    .map(Self::TypeCaseClause)
                    .unwrap_or(Self::Unknown(node))
            }
            "type_definition" => {
                <TypeDefinition as ::treesitter_types::FromNode>::from_node(node, src)
                    .map(Self::TypeDefinition)
                    .unwrap_or(Self::Unknown(node))
            }
            "type_lambda" => <TypeLambda as ::treesitter_types::FromNode>::from_node(node, src)
                .map(Self::TypeLambda)
                .unwrap_or(Self::Unknown(node)),
            "type_parameters" => {
                <TypeParameters as ::treesitter_types::FromNode>::from_node(node, src)
                    .map(Self::TypeParameters)
                    .unwrap_or(Self::Unknown(node))
            }
            "typed_pattern" => <TypedPattern as ::treesitter_types::FromNode>::from_node(node, src)
                .map(Self::TypedPattern)
                .unwrap_or(Self::Unknown(node)),
            "unit" => <Unit as ::treesitter_types::FromNode>::from_node(node, src)
                .map(Self::Unit)
                .unwrap_or(Self::Unknown(node)),
            "upper_bound" => <UpperBound as ::treesitter_types::FromNode>::from_node(node, src)
                .map(Self::UpperBound)
                .unwrap_or(Self::Unknown(node)),
            "using_directive" => {
                <UsingDirective as ::treesitter_types::FromNode>::from_node(node, src)
                    .map(Self::UsingDirective)
                    .unwrap_or(Self::Unknown(node))
            }
            "val_declaration" => {
                <ValDeclaration as ::treesitter_types::FromNode>::from_node(node, src)
                    .map(Self::ValDeclaration)
                    .unwrap_or(Self::Unknown(node))
            }
            "val_definition" => {
                <ValDefinition as ::treesitter_types::FromNode>::from_node(node, src)
                    .map(Self::ValDefinition)
                    .unwrap_or(Self::Unknown(node))
            }
            "var_declaration" => {
                <VarDeclaration as ::treesitter_types::FromNode>::from_node(node, src)
                    .map(Self::VarDeclaration)
                    .unwrap_or(Self::Unknown(node))
            }
            "var_definition" => {
                <VarDefinition as ::treesitter_types::FromNode>::from_node(node, src)
                    .map(Self::VarDefinition)
                    .unwrap_or(Self::Unknown(node))
            }
            "view_bound" => <ViewBound as ::treesitter_types::FromNode>::from_node(node, src)
                .map(Self::ViewBound)
                .unwrap_or(Self::Unknown(node)),
            "while_expression" => {
                <WhileExpression as ::treesitter_types::FromNode>::from_node(node, src)
                    .map(Self::WhileExpression)
                    .unwrap_or(Self::Unknown(node))
            }
            "wildcard" => <Wildcard as ::treesitter_types::FromNode>::from_node(node, src)
                .map(Self::Wildcard)
                .unwrap_or(Self::Unknown(node)),
            "with_template_body" => {
                <WithTemplateBody as ::treesitter_types::FromNode>::from_node(node, src)
                    .map(Self::WithTemplateBody)
                    .unwrap_or(Self::Unknown(node))
            }
            "character_literal" => {
                <CharacterLiteral as ::treesitter_types::FromNode>::from_node(node, src)
                    .map(Self::CharacterLiteral)
                    .unwrap_or(Self::Unknown(node))
            }
            "escape_sequence" => {
                <EscapeSequence as ::treesitter_types::FromNode>::from_node(node, src)
                    .map(Self::EscapeSequence)
                    .unwrap_or(Self::Unknown(node))
            }
            "floating_point_literal" => {
                <FloatingPointLiteral as ::treesitter_types::FromNode>::from_node(node, src)
                    .map(Self::FloatingPointLiteral)
                    .unwrap_or(Self::Unknown(node))
            }
            "integer_literal" => {
                <IntegerLiteral as ::treesitter_types::FromNode>::from_node(node, src)
                    .map(Self::IntegerLiteral)
                    .unwrap_or(Self::Unknown(node))
            }
            "null_literal" => <NullLiteral as ::treesitter_types::FromNode>::from_node(node, src)
                .map(Self::NullLiteral)
                .unwrap_or(Self::Unknown(node)),
            "operator_identifier" => {
                <OperatorIdentifier as ::treesitter_types::FromNode>::from_node(node, src)
                    .map(Self::OperatorIdentifier)
                    .unwrap_or(Self::Unknown(node))
            }
            "using_directive_key" => {
                <UsingDirectiveKey as ::treesitter_types::FromNode>::from_node(node, src)
                    .map(Self::UsingDirectiveKey)
                    .unwrap_or(Self::Unknown(node))
            }
            "using_directive_value" => {
                <UsingDirectiveValue as ::treesitter_types::FromNode>::from_node(node, src)
                    .map(Self::UsingDirectiveValue)
                    .unwrap_or(Self::Unknown(node))
            }
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
            Self::Definition(inner) => inner.span(),
            Self::Pattern(inner) => inner.span(),
            Self::Expression(inner) => inner.span(),
            Self::AccessModifier(inner) => inner.span(),
            Self::AccessQualifier(inner) => inner.span(),
            Self::AlternativePattern(inner) => inner.span(),
            Self::AnnotatedType(inner) => inner.span(),
            Self::Annotation(inner) => inner.span(),
            Self::AppliedConstructorType(inner) => inner.span(),
            Self::Arguments(inner) => inner.span(),
            Self::ArrowRenamedIdentifier(inner) => inner.span(),
            Self::AsRenamedIdentifier(inner) => inner.span(),
            Self::AscriptionExpression(inner) => inner.span(),
            Self::AssignmentExpression(inner) => inner.span(),
            Self::Binding(inner) => inner.span(),
            Self::Bindings(inner) => inner.span(),
            Self::Block(inner) => inner.span(),
            Self::BlockComment(inner) => inner.span(),
            Self::BooleanLiteral(inner) => inner.span(),
            Self::CallExpression(inner) => inner.span(),
            Self::CapturePattern(inner) => inner.span(),
            Self::CaseBlock(inner) => inner.span(),
            Self::CaseClassPattern(inner) => inner.span(),
            Self::CaseClause(inner) => inner.span(),
            Self::CatchClause(inner) => inner.span(),
            Self::ClassDefinition(inner) => inner.span(),
            Self::ClassParameter(inner) => inner.span(),
            Self::ClassParameters(inner) => inner.span(),
            Self::ColonArgument(inner) => inner.span(),
            Self::Comment(inner) => inner.span(),
            Self::CompilationUnit(inner) => inner.span(),
            Self::CompoundType(inner) => inner.span(),
            Self::ContextBound(inner) => inner.span(),
            Self::ContravariantTypeParameter(inner) => inner.span(),
            Self::CovariantTypeParameter(inner) => inner.span(),
            Self::DerivesClause(inner) => inner.span(),
            Self::DoWhileExpression(inner) => inner.span(),
            Self::EnumBody(inner) => inner.span(),
            Self::EnumCaseDefinitions(inner) => inner.span(),
            Self::EnumDefinition(inner) => inner.span(),
            Self::Enumerator(inner) => inner.span(),
            Self::Enumerators(inner) => inner.span(),
            Self::ExportDeclaration(inner) => inner.span(),
            Self::ExtendsClause(inner) => inner.span(),
            Self::ExtensionDefinition(inner) => inner.span(),
            Self::FieldExpression(inner) => inner.span(),
            Self::FinallyClause(inner) => inner.span(),
            Self::ForExpression(inner) => inner.span(),
            Self::FullEnumCase(inner) => inner.span(),
            Self::FunctionDeclaration(inner) => inner.span(),
            Self::FunctionDefinition(inner) => inner.span(),
            Self::FunctionType(inner) => inner.span(),
            Self::GenericFunction(inner) => inner.span(),
            Self::GenericType(inner) => inner.span(),
            Self::GivenConditional(inner) => inner.span(),
            Self::GivenDefinition(inner) => inner.span(),
            Self::GivenPattern(inner) => inner.span(),
            Self::Guard(inner) => inner.span(),
            Self::Identifier(inner) => inner.span(),
            Self::Identifiers(inner) => inner.span(),
            Self::IfExpression(inner) => inner.span(),
            Self::ImportDeclaration(inner) => inner.span(),
            Self::IndentedBlock(inner) => inner.span(),
            Self::IndentedCases(inner) => inner.span(),
            Self::InfixExpression(inner) => inner.span(),
            Self::InfixModifier(inner) => inner.span(),
            Self::InfixPattern(inner) => inner.span(),
            Self::InfixType(inner) => inner.span(),
            Self::InlineModifier(inner) => inner.span(),
            Self::InstanceExpression(inner) => inner.span(),
            Self::InterpolatedString(inner) => inner.span(),
            Self::InterpolatedStringExpression(inner) => inner.span(),
            Self::Interpolation(inner) => inner.span(),
            Self::IntoModifier(inner) => inner.span(),
            Self::LambdaExpression(inner) => inner.span(),
            Self::LazyParameterType(inner) => inner.span(),
            Self::LiteralType(inner) => inner.span(),
            Self::LowerBound(inner) => inner.span(),
            Self::MacroBody(inner) => inner.span(),
            Self::MatchExpression(inner) => inner.span(),
            Self::MatchType(inner) => inner.span(),
            Self::Modifiers(inner) => inner.span(),
            Self::NameAndType(inner) => inner.span(),
            Self::NamedPattern(inner) => inner.span(),
            Self::NamedTuplePattern(inner) => inner.span(),
            Self::NamedTupleType(inner) => inner.span(),
            Self::NamespaceSelectors(inner) => inner.span(),
            Self::NamespaceWildcard(inner) => inner.span(),
            Self::ObjectDefinition(inner) => inner.span(),
            Self::OpaqueModifier(inner) => inner.span(),
            Self::OpenModifier(inner) => inner.span(),
            Self::PackageClause(inner) => inner.span(),
            Self::PackageIdentifier(inner) => inner.span(),
            Self::PackageObject(inner) => inner.span(),
            Self::Parameter(inner) => inner.span(),
            Self::ParameterTypes(inner) => inner.span(),
            Self::Parameters(inner) => inner.span(),
            Self::ParenthesizedExpression(inner) => inner.span(),
            Self::PostfixExpression(inner) => inner.span(),
            Self::PrefixExpression(inner) => inner.span(),
            Self::ProjectedType(inner) => inner.span(),
            Self::QuoteExpression(inner) => inner.span(),
            Self::Refinement(inner) => inner.span(),
            Self::RepeatPattern(inner) => inner.span(),
            Self::RepeatedParameterType(inner) => inner.span(),
            Self::ReturnExpression(inner) => inner.span(),
            Self::SelfType(inner) => inner.span(),
            Self::SimpleEnumCase(inner) => inner.span(),
            Self::SingletonType(inner) => inner.span(),
            Self::SpliceExpression(inner) => inner.span(),
            Self::StableIdentifier(inner) => inner.span(),
            Self::StableTypeIdentifier(inner) => inner.span(),
            Self::String(inner) => inner.span(),
            Self::StructuralType(inner) => inner.span(),
            Self::TemplateBody(inner) => inner.span(),
            Self::ThrowExpression(inner) => inner.span(),
            Self::TrackedModifier(inner) => inner.span(),
            Self::TraitDefinition(inner) => inner.span(),
            Self::TransparentModifier(inner) => inner.span(),
            Self::TryExpression(inner) => inner.span(),
            Self::TupleExpression(inner) => inner.span(),
            Self::TuplePattern(inner) => inner.span(),
            Self::TupleType(inner) => inner.span(),
            Self::TypeArguments(inner) => inner.span(),
            Self::TypeCaseClause(inner) => inner.span(),
            Self::TypeDefinition(inner) => inner.span(),
            Self::TypeLambda(inner) => inner.span(),
            Self::TypeParameters(inner) => inner.span(),
            Self::TypedPattern(inner) => inner.span(),
            Self::Unit(inner) => inner.span(),
            Self::UpperBound(inner) => inner.span(),
            Self::UsingDirective(inner) => inner.span(),
            Self::ValDeclaration(inner) => inner.span(),
            Self::ValDefinition(inner) => inner.span(),
            Self::VarDeclaration(inner) => inner.span(),
            Self::VarDefinition(inner) => inner.span(),
            Self::ViewBound(inner) => inner.span(),
            Self::WhileExpression(inner) => inner.span(),
            Self::Wildcard(inner) => inner.span(),
            Self::WithTemplateBody(inner) => inner.span(),
            Self::CharacterLiteral(inner) => inner.span(),
            Self::EscapeSequence(inner) => inner.span(),
            Self::FloatingPointLiteral(inner) => inner.span(),
            Self::IntegerLiteral(inner) => inner.span(),
            Self::NullLiteral(inner) => inner.span(),
            Self::OperatorIdentifier(inner) => inner.span(),
            Self::UsingDirectiveKey(inner) => inner.span(),
            Self::UsingDirectiveValue(inner) => inner.span(),
            Self::TypeIdentifier(inner) => inner.span(),
            Self::Unknown(node) => ::treesitter_types::Span::from(*node),
        }
    }
}
