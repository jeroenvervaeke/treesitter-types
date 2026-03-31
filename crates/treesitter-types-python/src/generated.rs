#[derive(Debug, Clone)]
pub enum CompoundStatement<'tree> {
    ClassDefinition(::std::boxed::Box<ClassDefinition<'tree>>),
    DecoratedDefinition(::std::boxed::Box<DecoratedDefinition<'tree>>),
    ForStatement(::std::boxed::Box<ForStatement<'tree>>),
    FunctionDefinition(::std::boxed::Box<FunctionDefinition<'tree>>),
    IfStatement(::std::boxed::Box<IfStatement<'tree>>),
    MatchStatement(::std::boxed::Box<MatchStatement<'tree>>),
    TryStatement(::std::boxed::Box<TryStatement<'tree>>),
    WhileStatement(::std::boxed::Box<WhileStatement<'tree>>),
    WithStatement(::std::boxed::Box<WithStatement<'tree>>),
}
impl<'tree> ::treesitter_types::FromNode<'tree> for CompoundStatement<'tree> {
    #[allow(clippy::collapsible_else_if)]
    fn from_node(
        node: ::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        match node.kind() {
            "class_definition" => Ok(Self::ClassDefinition(::std::boxed::Box::new(
                <ClassDefinition as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "decorated_definition" => Ok(Self::DecoratedDefinition(::std::boxed::Box::new(
                <DecoratedDefinition as ::treesitter_types::FromNode>::from_node(node, src)?,
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
            "match_statement" => Ok(Self::MatchStatement(::std::boxed::Box::new(
                <MatchStatement as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "try_statement" => Ok(Self::TryStatement(::std::boxed::Box::new(
                <TryStatement as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "while_statement" => Ok(Self::WhileStatement(::std::boxed::Box::new(
                <WhileStatement as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "with_statement" => Ok(Self::WithStatement(::std::boxed::Box::new(
                <WithStatement as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            other => Err(::treesitter_types::ParseError::unexpected_kind(other, node)),
        }
    }
}
impl ::treesitter_types::Spanned for CompoundStatement<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        match self {
            Self::ClassDefinition(inner) => inner.span(),
            Self::DecoratedDefinition(inner) => inner.span(),
            Self::ForStatement(inner) => inner.span(),
            Self::FunctionDefinition(inner) => inner.span(),
            Self::IfStatement(inner) => inner.span(),
            Self::MatchStatement(inner) => inner.span(),
            Self::TryStatement(inner) => inner.span(),
            Self::WhileStatement(inner) => inner.span(),
            Self::WithStatement(inner) => inner.span(),
        }
    }
}
#[derive(Debug, Clone)]
pub enum SimpleStatement<'tree> {
    AssertStatement(::std::boxed::Box<AssertStatement<'tree>>),
    BreakStatement(::std::boxed::Box<BreakStatement<'tree>>),
    ContinueStatement(::std::boxed::Box<ContinueStatement<'tree>>),
    DeleteStatement(::std::boxed::Box<DeleteStatement<'tree>>),
    ExecStatement(::std::boxed::Box<ExecStatement<'tree>>),
    ExpressionStatement(::std::boxed::Box<ExpressionStatement<'tree>>),
    FutureImportStatement(::std::boxed::Box<FutureImportStatement<'tree>>),
    GlobalStatement(::std::boxed::Box<GlobalStatement<'tree>>),
    ImportFromStatement(::std::boxed::Box<ImportFromStatement<'tree>>),
    ImportStatement(::std::boxed::Box<ImportStatement<'tree>>),
    NonlocalStatement(::std::boxed::Box<NonlocalStatement<'tree>>),
    PassStatement(::std::boxed::Box<PassStatement<'tree>>),
    PrintStatement(::std::boxed::Box<PrintStatement<'tree>>),
    RaiseStatement(::std::boxed::Box<RaiseStatement<'tree>>),
    ReturnStatement(::std::boxed::Box<ReturnStatement<'tree>>),
    TypeAliasStatement(::std::boxed::Box<TypeAliasStatement<'tree>>),
}
impl<'tree> ::treesitter_types::FromNode<'tree> for SimpleStatement<'tree> {
    #[allow(clippy::collapsible_else_if)]
    fn from_node(
        node: ::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        match node.kind() {
            "assert_statement" => Ok(Self::AssertStatement(::std::boxed::Box::new(
                <AssertStatement as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "break_statement" => Ok(Self::BreakStatement(::std::boxed::Box::new(
                <BreakStatement as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "continue_statement" => Ok(Self::ContinueStatement(::std::boxed::Box::new(
                <ContinueStatement as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "delete_statement" => Ok(Self::DeleteStatement(::std::boxed::Box::new(
                <DeleteStatement as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "exec_statement" => Ok(Self::ExecStatement(::std::boxed::Box::new(
                <ExecStatement as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "expression_statement" => Ok(Self::ExpressionStatement(::std::boxed::Box::new(
                <ExpressionStatement as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "future_import_statement" => Ok(Self::FutureImportStatement(::std::boxed::Box::new(
                <FutureImportStatement as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "global_statement" => Ok(Self::GlobalStatement(::std::boxed::Box::new(
                <GlobalStatement as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "import_from_statement" => Ok(Self::ImportFromStatement(::std::boxed::Box::new(
                <ImportFromStatement as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "import_statement" => Ok(Self::ImportStatement(::std::boxed::Box::new(
                <ImportStatement as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "nonlocal_statement" => Ok(Self::NonlocalStatement(::std::boxed::Box::new(
                <NonlocalStatement as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "pass_statement" => Ok(Self::PassStatement(::std::boxed::Box::new(
                <PassStatement as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "print_statement" => Ok(Self::PrintStatement(::std::boxed::Box::new(
                <PrintStatement as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "raise_statement" => Ok(Self::RaiseStatement(::std::boxed::Box::new(
                <RaiseStatement as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "return_statement" => Ok(Self::ReturnStatement(::std::boxed::Box::new(
                <ReturnStatement as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "type_alias_statement" => Ok(Self::TypeAliasStatement(::std::boxed::Box::new(
                <TypeAliasStatement as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            other => Err(::treesitter_types::ParseError::unexpected_kind(other, node)),
        }
    }
}
impl ::treesitter_types::Spanned for SimpleStatement<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        match self {
            Self::AssertStatement(inner) => inner.span(),
            Self::BreakStatement(inner) => inner.span(),
            Self::ContinueStatement(inner) => inner.span(),
            Self::DeleteStatement(inner) => inner.span(),
            Self::ExecStatement(inner) => inner.span(),
            Self::ExpressionStatement(inner) => inner.span(),
            Self::FutureImportStatement(inner) => inner.span(),
            Self::GlobalStatement(inner) => inner.span(),
            Self::ImportFromStatement(inner) => inner.span(),
            Self::ImportStatement(inner) => inner.span(),
            Self::NonlocalStatement(inner) => inner.span(),
            Self::PassStatement(inner) => inner.span(),
            Self::PrintStatement(inner) => inner.span(),
            Self::RaiseStatement(inner) => inner.span(),
            Self::ReturnStatement(inner) => inner.span(),
            Self::TypeAliasStatement(inner) => inner.span(),
        }
    }
}
#[derive(Debug, Clone)]
pub enum Expression<'tree> {
    AsPattern(::std::boxed::Box<AsPattern<'tree>>),
    BooleanOperator(::std::boxed::Box<BooleanOperator<'tree>>),
    ComparisonOperator(::std::boxed::Box<ComparisonOperator<'tree>>),
    ConditionalExpression(::std::boxed::Box<ConditionalExpression<'tree>>),
    Lambda(::std::boxed::Box<Lambda<'tree>>),
    NamedExpression(::std::boxed::Box<NamedExpression<'tree>>),
    NotOperator(::std::boxed::Box<NotOperator<'tree>>),
    PrimaryExpression(::std::boxed::Box<PrimaryExpression<'tree>>),
}
impl<'tree> ::treesitter_types::FromNode<'tree> for Expression<'tree> {
    #[allow(clippy::collapsible_else_if)]
    fn from_node(
        node: ::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        match node.kind() {
            "as_pattern" => Ok(Self::AsPattern(::std::boxed::Box::new(
                <AsPattern as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "boolean_operator" => Ok(Self::BooleanOperator(::std::boxed::Box::new(
                <BooleanOperator as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "comparison_operator" => Ok(Self::ComparisonOperator(::std::boxed::Box::new(
                <ComparisonOperator as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "conditional_expression" => Ok(Self::ConditionalExpression(::std::boxed::Box::new(
                <ConditionalExpression as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "lambda" => Ok(Self::Lambda(::std::boxed::Box::new(
                <Lambda as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "named_expression" => Ok(Self::NamedExpression(::std::boxed::Box::new(
                <NamedExpression as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "not_operator" => Ok(Self::NotOperator(::std::boxed::Box::new(
                <NotOperator as ::treesitter_types::FromNode>::from_node(node, src)?,
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
            Self::AsPattern(inner) => inner.span(),
            Self::BooleanOperator(inner) => inner.span(),
            Self::ComparisonOperator(inner) => inner.span(),
            Self::ConditionalExpression(inner) => inner.span(),
            Self::Lambda(inner) => inner.span(),
            Self::NamedExpression(inner) => inner.span(),
            Self::NotOperator(inner) => inner.span(),
            Self::PrimaryExpression(inner) => inner.span(),
        }
    }
}
#[derive(Debug, Clone)]
pub enum Parameter<'tree> {
    DefaultParameter(::std::boxed::Box<DefaultParameter<'tree>>),
    DictionarySplatPattern(::std::boxed::Box<DictionarySplatPattern<'tree>>),
    Identifier(::std::boxed::Box<Identifier<'tree>>),
    KeywordSeparator(::std::boxed::Box<KeywordSeparator<'tree>>),
    ListSplatPattern(::std::boxed::Box<ListSplatPattern<'tree>>),
    PositionalSeparator(::std::boxed::Box<PositionalSeparator<'tree>>),
    TuplePattern(::std::boxed::Box<TuplePattern<'tree>>),
    TypedDefaultParameter(::std::boxed::Box<TypedDefaultParameter<'tree>>),
    TypedParameter(::std::boxed::Box<TypedParameter<'tree>>),
}
impl<'tree> ::treesitter_types::FromNode<'tree> for Parameter<'tree> {
    #[allow(clippy::collapsible_else_if)]
    fn from_node(
        node: ::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        match node.kind() {
            "default_parameter" => Ok(Self::DefaultParameter(::std::boxed::Box::new(
                <DefaultParameter as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "dictionary_splat_pattern" => Ok(Self::DictionarySplatPattern(::std::boxed::Box::new(
                <DictionarySplatPattern as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "identifier" => Ok(Self::Identifier(::std::boxed::Box::new(
                <Identifier as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "keyword_separator" => Ok(Self::KeywordSeparator(::std::boxed::Box::new(
                <KeywordSeparator as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "list_splat_pattern" => Ok(Self::ListSplatPattern(::std::boxed::Box::new(
                <ListSplatPattern as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "positional_separator" => Ok(Self::PositionalSeparator(::std::boxed::Box::new(
                <PositionalSeparator as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "tuple_pattern" => Ok(Self::TuplePattern(::std::boxed::Box::new(
                <TuplePattern as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "typed_default_parameter" => Ok(Self::TypedDefaultParameter(::std::boxed::Box::new(
                <TypedDefaultParameter as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "typed_parameter" => Ok(Self::TypedParameter(::std::boxed::Box::new(
                <TypedParameter as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            other => Err(::treesitter_types::ParseError::unexpected_kind(other, node)),
        }
    }
}
impl ::treesitter_types::Spanned for Parameter<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        match self {
            Self::DefaultParameter(inner) => inner.span(),
            Self::DictionarySplatPattern(inner) => inner.span(),
            Self::Identifier(inner) => inner.span(),
            Self::KeywordSeparator(inner) => inner.span(),
            Self::ListSplatPattern(inner) => inner.span(),
            Self::PositionalSeparator(inner) => inner.span(),
            Self::TuplePattern(inner) => inner.span(),
            Self::TypedDefaultParameter(inner) => inner.span(),
            Self::TypedParameter(inner) => inner.span(),
        }
    }
}
#[derive(Debug, Clone)]
pub enum Pattern<'tree> {
    Attribute(::std::boxed::Box<Attribute<'tree>>),
    Identifier(::std::boxed::Box<Identifier<'tree>>),
    ListPattern(::std::boxed::Box<ListPattern<'tree>>),
    ListSplatPattern(::std::boxed::Box<ListSplatPattern<'tree>>),
    Subscript(::std::boxed::Box<Subscript<'tree>>),
    TuplePattern(::std::boxed::Box<TuplePattern<'tree>>),
}
impl<'tree> ::treesitter_types::FromNode<'tree> for Pattern<'tree> {
    #[allow(clippy::collapsible_else_if)]
    fn from_node(
        node: ::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        match node.kind() {
            "attribute" => Ok(Self::Attribute(::std::boxed::Box::new(
                <Attribute as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "identifier" => Ok(Self::Identifier(::std::boxed::Box::new(
                <Identifier as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "list_pattern" => Ok(Self::ListPattern(::std::boxed::Box::new(
                <ListPattern as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "list_splat_pattern" => Ok(Self::ListSplatPattern(::std::boxed::Box::new(
                <ListSplatPattern as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "subscript" => Ok(Self::Subscript(::std::boxed::Box::new(
                <Subscript as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "tuple_pattern" => Ok(Self::TuplePattern(::std::boxed::Box::new(
                <TuplePattern as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            other => Err(::treesitter_types::ParseError::unexpected_kind(other, node)),
        }
    }
}
impl ::treesitter_types::Spanned for Pattern<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        match self {
            Self::Attribute(inner) => inner.span(),
            Self::Identifier(inner) => inner.span(),
            Self::ListPattern(inner) => inner.span(),
            Self::ListSplatPattern(inner) => inner.span(),
            Self::Subscript(inner) => inner.span(),
            Self::TuplePattern(inner) => inner.span(),
        }
    }
}
#[derive(Debug, Clone)]
pub enum PrimaryExpression<'tree> {
    Attribute(::std::boxed::Box<Attribute<'tree>>),
    Await(::std::boxed::Box<Await<'tree>>),
    BinaryOperator(::std::boxed::Box<BinaryOperator<'tree>>),
    Call(::std::boxed::Box<Call<'tree>>),
    ConcatenatedString(::std::boxed::Box<ConcatenatedString<'tree>>),
    Dictionary(::std::boxed::Box<Dictionary<'tree>>),
    DictionaryComprehension(::std::boxed::Box<DictionaryComprehension<'tree>>),
    Ellipsis(::std::boxed::Box<Ellipsis<'tree>>),
    False(::std::boxed::Box<False<'tree>>),
    Float(::std::boxed::Box<Float<'tree>>),
    GeneratorExpression(::std::boxed::Box<GeneratorExpression<'tree>>),
    Identifier(::std::boxed::Box<Identifier<'tree>>),
    Integer(::std::boxed::Box<Integer<'tree>>),
    List(::std::boxed::Box<List<'tree>>),
    ListComprehension(::std::boxed::Box<ListComprehension<'tree>>),
    ListSplat(::std::boxed::Box<ListSplat<'tree>>),
    None(::std::boxed::Box<None<'tree>>),
    ParenthesizedExpression(::std::boxed::Box<ParenthesizedExpression<'tree>>),
    Set(::std::boxed::Box<Set<'tree>>),
    SetComprehension(::std::boxed::Box<SetComprehension<'tree>>),
    String(::std::boxed::Box<String<'tree>>),
    Subscript(::std::boxed::Box<Subscript<'tree>>),
    True(::std::boxed::Box<True<'tree>>),
    Tuple(::std::boxed::Box<Tuple<'tree>>),
    UnaryOperator(::std::boxed::Box<UnaryOperator<'tree>>),
}
impl<'tree> ::treesitter_types::FromNode<'tree> for PrimaryExpression<'tree> {
    #[allow(clippy::collapsible_else_if)]
    fn from_node(
        node: ::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        match node.kind() {
            "attribute" => Ok(Self::Attribute(::std::boxed::Box::new(
                <Attribute as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "await" => Ok(Self::Await(::std::boxed::Box::new(
                <Await as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "binary_operator" => Ok(Self::BinaryOperator(::std::boxed::Box::new(
                <BinaryOperator as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "call" => Ok(Self::Call(::std::boxed::Box::new(
                <Call as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "concatenated_string" => Ok(Self::ConcatenatedString(::std::boxed::Box::new(
                <ConcatenatedString as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "dictionary" => Ok(Self::Dictionary(::std::boxed::Box::new(
                <Dictionary as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "dictionary_comprehension" => {
                Ok(Self::DictionaryComprehension(::std::boxed::Box::new(
                    <DictionaryComprehension as ::treesitter_types::FromNode>::from_node(
                        node, src,
                    )?,
                )))
            }
            "ellipsis" => Ok(Self::Ellipsis(::std::boxed::Box::new(
                <Ellipsis as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "false" => Ok(Self::False(::std::boxed::Box::new(
                <False as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "float" => Ok(Self::Float(::std::boxed::Box::new(
                <Float as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "generator_expression" => Ok(Self::GeneratorExpression(::std::boxed::Box::new(
                <GeneratorExpression as ::treesitter_types::FromNode>::from_node(node, src)?,
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
            "list_comprehension" => Ok(Self::ListComprehension(::std::boxed::Box::new(
                <ListComprehension as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "list_splat" => Ok(Self::ListSplat(::std::boxed::Box::new(
                <ListSplat as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "none" => Ok(Self::None(::std::boxed::Box::new(
                <None as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "parenthesized_expression" => {
                Ok(Self::ParenthesizedExpression(::std::boxed::Box::new(
                    <ParenthesizedExpression as ::treesitter_types::FromNode>::from_node(
                        node, src,
                    )?,
                )))
            }
            "set" => Ok(Self::Set(::std::boxed::Box::new(
                <Set as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "set_comprehension" => Ok(Self::SetComprehension(::std::boxed::Box::new(
                <SetComprehension as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "string" => Ok(Self::String(::std::boxed::Box::new(
                <String as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "subscript" => Ok(Self::Subscript(::std::boxed::Box::new(
                <Subscript as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "true" => Ok(Self::True(::std::boxed::Box::new(
                <True as ::treesitter_types::FromNode>::from_node(node, src)?,
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
impl ::treesitter_types::Spanned for PrimaryExpression<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        match self {
            Self::Attribute(inner) => inner.span(),
            Self::Await(inner) => inner.span(),
            Self::BinaryOperator(inner) => inner.span(),
            Self::Call(inner) => inner.span(),
            Self::ConcatenatedString(inner) => inner.span(),
            Self::Dictionary(inner) => inner.span(),
            Self::DictionaryComprehension(inner) => inner.span(),
            Self::Ellipsis(inner) => inner.span(),
            Self::False(inner) => inner.span(),
            Self::Float(inner) => inner.span(),
            Self::GeneratorExpression(inner) => inner.span(),
            Self::Identifier(inner) => inner.span(),
            Self::Integer(inner) => inner.span(),
            Self::List(inner) => inner.span(),
            Self::ListComprehension(inner) => inner.span(),
            Self::ListSplat(inner) => inner.span(),
            Self::None(inner) => inner.span(),
            Self::ParenthesizedExpression(inner) => inner.span(),
            Self::Set(inner) => inner.span(),
            Self::SetComprehension(inner) => inner.span(),
            Self::String(inner) => inner.span(),
            Self::Subscript(inner) => inner.span(),
            Self::True(inner) => inner.span(),
            Self::Tuple(inner) => inner.span(),
            Self::UnaryOperator(inner) => inner.span(),
        }
    }
}
#[derive(Debug, Clone)]
pub struct AliasedImport<'tree> {
    pub span: ::treesitter_types::Span,
    pub alias: Identifier<'tree>,
    pub name: DottedName<'tree>,
}
impl<'tree> ::treesitter_types::FromNode<'tree> for AliasedImport<'tree> {
    #[allow(clippy::match_single_binding, clippy::suspicious_else_formatting)]
    fn from_node(
        node: ::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        debug_assert_eq!(node.kind(), "aliased_import");
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
                <DottedName as ::treesitter_types::FromNode>::from_node(child, src)?
            },
        })
    }
}
impl ::treesitter_types::Spanned for AliasedImport<'_> {
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
pub struct AsPattern<'tree> {
    pub span: ::treesitter_types::Span,
    pub alias: ::core::option::Option<AsPatternTarget<'tree>>,
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
            alias: match node.child_by_field_name("alias") {
                Some(child) => {
                    Some(<AsPatternTarget as ::treesitter_types::FromNode>::from_node(child, src)?)
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
pub struct AssertStatement<'tree> {
    pub span: ::treesitter_types::Span,
    pub children: ::std::vec::Vec<Expression<'tree>>,
}
impl<'tree> ::treesitter_types::FromNode<'tree> for AssertStatement<'tree> {
    #[allow(clippy::match_single_binding, clippy::suspicious_else_formatting)]
    fn from_node(
        node: ::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        debug_assert_eq!(node.kind(), "assert_statement");
        Ok(Self {
            span: ::treesitter_types::Span::from(node),
            children: {
                #[allow(clippy::suspicious_else_formatting)]
                let non_field_children = {
                    let mut cursor = node.walk();
                    let mut result = ::std::vec::Vec::new();
                    if cursor.goto_first_child() {
                        loop {
                            if cursor.field_name().is_none()
                                && cursor.node().is_named()
                                && !cursor.node().is_extra()
                            {
                                result.push(cursor.node());
                            }
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
impl ::treesitter_types::Spanned for AssertStatement<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        self.span
    }
}
#[derive(Debug, Clone)]
pub struct Assignment<'tree> {
    pub span: ::treesitter_types::Span,
    pub left: AssignmentLeft<'tree>,
    pub right: ::core::option::Option<AssignmentRight<'tree>>,
    pub r#type: ::core::option::Option<Type<'tree>>,
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
                <AssignmentLeft as ::treesitter_types::FromNode>::from_node(child, src)?
            },
            right: match node.child_by_field_name("right") {
                Some(child) => {
                    Some(<AssignmentRight as ::treesitter_types::FromNode>::from_node(child, src)?)
                }
                None => None,
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
impl ::treesitter_types::Spanned for Assignment<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        self.span
    }
}
#[derive(Debug, Clone)]
pub struct Attribute<'tree> {
    pub span: ::treesitter_types::Span,
    pub attribute: Identifier<'tree>,
    pub object: PrimaryExpression<'tree>,
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
            attribute: {
                let child = node.child_by_field_name("attribute").ok_or_else(|| {
                    ::treesitter_types::ParseError::missing_field("attribute", node)
                })?;
                <Identifier as ::treesitter_types::FromNode>::from_node(child, src)?
            },
            object: {
                let child = node
                    .child_by_field_name("object")
                    .ok_or_else(|| ::treesitter_types::ParseError::missing_field("object", node))?;
                <PrimaryExpression as ::treesitter_types::FromNode>::from_node(child, src)?
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
pub struct AugmentedAssignment<'tree> {
    pub span: ::treesitter_types::Span,
    pub left: AugmentedAssignmentLeft<'tree>,
    pub operator: AugmentedAssignmentOperator,
    pub right: AugmentedAssignmentRight<'tree>,
}
impl<'tree> ::treesitter_types::FromNode<'tree> for AugmentedAssignment<'tree> {
    #[allow(clippy::match_single_binding, clippy::suspicious_else_formatting)]
    fn from_node(
        node: ::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        debug_assert_eq!(node.kind(), "augmented_assignment");
        Ok(Self {
            span: ::treesitter_types::Span::from(node),
            left: {
                let child = node
                    .child_by_field_name("left")
                    .ok_or_else(|| ::treesitter_types::ParseError::missing_field("left", node))?;
                <AugmentedAssignmentLeft as ::treesitter_types::FromNode>::from_node(child, src)?
            },
            operator: {
                let child = node.child_by_field_name("operator").ok_or_else(|| {
                    ::treesitter_types::ParseError::missing_field("operator", node)
                })?;
                <AugmentedAssignmentOperator as ::treesitter_types::FromNode>::from_node(
                    child, src,
                )?
            },
            right: {
                let child = node
                    .child_by_field_name("right")
                    .ok_or_else(|| ::treesitter_types::ParseError::missing_field("right", node))?;
                <AugmentedAssignmentRight as ::treesitter_types::FromNode>::from_node(child, src)?
            },
        })
    }
}
impl ::treesitter_types::Spanned for AugmentedAssignment<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        self.span
    }
}
#[derive(Debug, Clone)]
pub struct Await<'tree> {
    pub span: ::treesitter_types::Span,
    pub children: PrimaryExpression<'tree>,
}
impl<'tree> ::treesitter_types::FromNode<'tree> for Await<'tree> {
    #[allow(clippy::match_single_binding, clippy::suspicious_else_formatting)]
    fn from_node(
        node: ::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        debug_assert_eq!(node.kind(), "await");
        Ok(Self {
            span: ::treesitter_types::Span::from(node),
            children: {
                #[allow(clippy::suspicious_else_formatting)]
                let non_field_children = {
                    let mut cursor = node.walk();
                    let mut result = ::std::vec::Vec::new();
                    if cursor.goto_first_child() {
                        loop {
                            if cursor.field_name().is_none()
                                && cursor.node().is_named()
                                && !cursor.node().is_extra()
                            {
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
                                        <PrimaryExpression as ::treesitter_types::FromNode>::from_node(
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
                <PrimaryExpression as ::treesitter_types::FromNode>::from_node(child, src)?
            },
        })
    }
}
impl ::treesitter_types::Spanned for Await<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        self.span
    }
}
#[derive(Debug, Clone)]
pub struct BinaryOperator<'tree> {
    pub span: ::treesitter_types::Span,
    pub left: PrimaryExpression<'tree>,
    pub operator: BinaryOperatorOperator,
    pub right: PrimaryExpression<'tree>,
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
                <PrimaryExpression as ::treesitter_types::FromNode>::from_node(child, src)?
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
                <PrimaryExpression as ::treesitter_types::FromNode>::from_node(child, src)?
            },
        })
    }
}
impl ::treesitter_types::Spanned for BinaryOperator<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        self.span
    }
}
#[derive(Debug, Clone)]
pub struct Block<'tree> {
    pub span: ::treesitter_types::Span,
    pub alternative: ::std::vec::Vec<CaseClause<'tree>>,
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
            alternative: {
                let mut cursor = node.walk();
                let mut items = ::std::vec::Vec::new();
                for child in node.children_by_field_name("alternative", &mut cursor) {
                    items.push(<CaseClause as ::treesitter_types::FromNode>::from_node(
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
pub struct BooleanOperator<'tree> {
    pub span: ::treesitter_types::Span,
    pub left: Expression<'tree>,
    pub operator: BooleanOperatorOperator,
    pub right: Expression<'tree>,
}
impl<'tree> ::treesitter_types::FromNode<'tree> for BooleanOperator<'tree> {
    #[allow(clippy::match_single_binding, clippy::suspicious_else_formatting)]
    fn from_node(
        node: ::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        debug_assert_eq!(node.kind(), "boolean_operator");
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
                <BooleanOperatorOperator as ::treesitter_types::FromNode>::from_node(child, src)?
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
impl ::treesitter_types::Spanned for BooleanOperator<'_> {
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
pub struct Call<'tree> {
    pub span: ::treesitter_types::Span,
    pub arguments: CallArguments<'tree>,
    pub function: PrimaryExpression<'tree>,
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
            arguments: {
                let child = node.child_by_field_name("arguments").ok_or_else(|| {
                    ::treesitter_types::ParseError::missing_field("arguments", node)
                })?;
                <CallArguments as ::treesitter_types::FromNode>::from_node(child, src)?
            },
            function: {
                let child = node.child_by_field_name("function").ok_or_else(|| {
                    ::treesitter_types::ParseError::missing_field("function", node)
                })?;
                <PrimaryExpression as ::treesitter_types::FromNode>::from_node(child, src)?
            },
        })
    }
}
impl ::treesitter_types::Spanned for Call<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        self.span
    }
}
#[derive(Debug, Clone)]
pub struct CaseClause<'tree> {
    pub span: ::treesitter_types::Span,
    pub consequence: Block<'tree>,
    pub guard: ::core::option::Option<IfClause<'tree>>,
    pub children: ::std::vec::Vec<CasePattern<'tree>>,
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
            consequence: {
                let child = node.child_by_field_name("consequence").ok_or_else(|| {
                    ::treesitter_types::ParseError::missing_field("consequence", node)
                })?;
                <Block as ::treesitter_types::FromNode>::from_node(child, src)?
            },
            guard: match node.child_by_field_name("guard") {
                Some(child) => Some(<IfClause as ::treesitter_types::FromNode>::from_node(
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
                    items.push(<CasePattern as ::treesitter_types::FromNode>::from_node(
                        child, src,
                    )?);
                }
                items
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
pub struct CasePattern<'tree> {
    pub span: ::treesitter_types::Span,
    pub children: ::core::option::Option<CasePatternChildren<'tree>>,
}
impl<'tree> ::treesitter_types::FromNode<'tree> for CasePattern<'tree> {
    #[allow(clippy::match_single_binding, clippy::suspicious_else_formatting)]
    fn from_node(
        node: ::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        debug_assert_eq!(node.kind(), "case_pattern");
        Ok(Self {
            span: ::treesitter_types::Span::from(node),
            children: {
                #[allow(clippy::suspicious_else_formatting)]
                let non_field_children = {
                    let mut cursor = node.walk();
                    let mut result = ::std::vec::Vec::new();
                    if cursor.goto_first_child() {
                        loop {
                            if cursor.field_name().is_none()
                                && cursor.node().is_named()
                                && !cursor.node().is_extra()
                            {
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
                        <CasePatternChildren as ::treesitter_types::FromNode>::from_node(
                            child, src,
                        )?,
                    ),
                    None => None,
                }
            },
        })
    }
}
impl ::treesitter_types::Spanned for CasePattern<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        self.span
    }
}
#[derive(Debug, Clone)]
pub struct Chevron<'tree> {
    pub span: ::treesitter_types::Span,
    pub children: Expression<'tree>,
}
impl<'tree> ::treesitter_types::FromNode<'tree> for Chevron<'tree> {
    #[allow(clippy::match_single_binding, clippy::suspicious_else_formatting)]
    fn from_node(
        node: ::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        debug_assert_eq!(node.kind(), "chevron");
        Ok(Self {
            span: ::treesitter_types::Span::from(node),
            children: {
                #[allow(clippy::suspicious_else_formatting)]
                let non_field_children = {
                    let mut cursor = node.walk();
                    let mut result = ::std::vec::Vec::new();
                    if cursor.goto_first_child() {
                        loop {
                            if cursor.field_name().is_none()
                                && cursor.node().is_named()
                                && !cursor.node().is_extra()
                            {
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
                    fallback_child.ok_or_else(|| {
                        ::treesitter_types::ParseError::missing_field("children", node)
                    })?
                };
                <Expression as ::treesitter_types::FromNode>::from_node(child, src)?
            },
        })
    }
}
impl ::treesitter_types::Spanned for Chevron<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        self.span
    }
}
#[derive(Debug, Clone)]
pub struct ClassDefinition<'tree> {
    pub span: ::treesitter_types::Span,
    pub body: Block<'tree>,
    pub name: Identifier<'tree>,
    pub superclasses: ::core::option::Option<ArgumentList<'tree>>,
    pub type_parameters: ::core::option::Option<TypeParameter<'tree>>,
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
                let child = node
                    .child_by_field_name("body")
                    .ok_or_else(|| ::treesitter_types::ParseError::missing_field("body", node))?;
                <Block as ::treesitter_types::FromNode>::from_node(child, src)?
            },
            name: {
                let child = node
                    .child_by_field_name("name")
                    .ok_or_else(|| ::treesitter_types::ParseError::missing_field("name", node))?;
                <Identifier as ::treesitter_types::FromNode>::from_node(child, src)?
            },
            superclasses: match node.child_by_field_name("superclasses") {
                Some(child) => Some(<ArgumentList as ::treesitter_types::FromNode>::from_node(
                    child, src,
                )?),
                None => None,
            },
            type_parameters: match node.child_by_field_name("type_parameters") {
                Some(child) => Some(<TypeParameter as ::treesitter_types::FromNode>::from_node(
                    child, src,
                )?),
                None => None,
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
pub struct ClassPattern<'tree> {
    pub span: ::treesitter_types::Span,
    pub children: ::std::vec::Vec<ClassPatternChildren<'tree>>,
}
impl<'tree> ::treesitter_types::FromNode<'tree> for ClassPattern<'tree> {
    #[allow(clippy::match_single_binding, clippy::suspicious_else_formatting)]
    fn from_node(
        node: ::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        debug_assert_eq!(node.kind(), "class_pattern");
        Ok(Self {
            span: ::treesitter_types::Span::from(node),
            children: {
                #[allow(clippy::suspicious_else_formatting)]
                let non_field_children = {
                    let mut cursor = node.walk();
                    let mut result = ::std::vec::Vec::new();
                    if cursor.goto_first_child() {
                        loop {
                            if cursor.field_name().is_none()
                                && cursor.node().is_named()
                                && !cursor.node().is_extra()
                            {
                                result.push(cursor.node());
                            }
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
                        <ClassPatternChildren as ::treesitter_types::FromNode>::from_node(
                            child, src,
                        )?,
                    );
                }
                items
            },
        })
    }
}
impl ::treesitter_types::Spanned for ClassPattern<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        self.span
    }
}
#[derive(Debug, Clone)]
pub struct ComparisonOperator<'tree> {
    pub span: ::treesitter_types::Span,
    pub operators: ::std::vec::Vec<ComparisonOperatorOperators>,
    pub children: ::std::vec::Vec<PrimaryExpression<'tree>>,
}
impl<'tree> ::treesitter_types::FromNode<'tree> for ComparisonOperator<'tree> {
    #[allow(clippy::match_single_binding, clippy::suspicious_else_formatting)]
    fn from_node(
        node: ::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        debug_assert_eq!(node.kind(), "comparison_operator");
        Ok(Self {
            span: ::treesitter_types::Span::from(node),
            operators: {
                let mut cursor = node.walk();
                let mut items = ::std::vec::Vec::new();
                for child in node.children_by_field_name("operators", &mut cursor) {
                    items.push(
                        <ComparisonOperatorOperators as ::treesitter_types::FromNode>::from_node(
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
                        <PrimaryExpression as ::treesitter_types::FromNode>::from_node(child, src)?,
                    );
                }
                items
            },
        })
    }
}
impl ::treesitter_types::Spanned for ComparisonOperator<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        self.span
    }
}
#[derive(Debug, Clone)]
pub struct ComplexPattern<'tree> {
    pub span: ::treesitter_types::Span,
    pub children: ::std::vec::Vec<ComplexPatternChildren<'tree>>,
}
impl<'tree> ::treesitter_types::FromNode<'tree> for ComplexPattern<'tree> {
    #[allow(clippy::match_single_binding, clippy::suspicious_else_formatting)]
    fn from_node(
        node: ::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        debug_assert_eq!(node.kind(), "complex_pattern");
        Ok(Self {
            span: ::treesitter_types::Span::from(node),
            children: {
                #[allow(clippy::suspicious_else_formatting)]
                let non_field_children = {
                    let mut cursor = node.walk();
                    let mut result = ::std::vec::Vec::new();
                    if cursor.goto_first_child() {
                        loop {
                            if cursor.field_name().is_none()
                                && cursor.node().is_named()
                                && !cursor.node().is_extra()
                            {
                                result.push(cursor.node());
                            }
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
                        <ComplexPatternChildren as ::treesitter_types::FromNode>::from_node(
                            child, src,
                        )?,
                    );
                }
                items
            },
        })
    }
}
impl ::treesitter_types::Spanned for ComplexPattern<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        self.span
    }
}
#[derive(Debug, Clone)]
pub struct ConcatenatedString<'tree> {
    pub span: ::treesitter_types::Span,
    pub children: ::std::vec::Vec<String<'tree>>,
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
                    items.push(<String as ::treesitter_types::FromNode>::from_node(
                        child, src,
                    )?);
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
pub struct ConditionalExpression<'tree> {
    pub span: ::treesitter_types::Span,
    pub children: ::std::vec::Vec<Expression<'tree>>,
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
            children: {
                #[allow(clippy::suspicious_else_formatting)]
                let non_field_children = {
                    let mut cursor = node.walk();
                    let mut result = ::std::vec::Vec::new();
                    if cursor.goto_first_child() {
                        loop {
                            if cursor.field_name().is_none()
                                && cursor.node().is_named()
                                && !cursor.node().is_extra()
                            {
                                result.push(cursor.node());
                            }
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
impl ::treesitter_types::Spanned for ConditionalExpression<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        self.span
    }
}
#[derive(Debug, Clone)]
pub struct ConstrainedType<'tree> {
    pub span: ::treesitter_types::Span,
    pub children: ::std::vec::Vec<Type<'tree>>,
}
impl<'tree> ::treesitter_types::FromNode<'tree> for ConstrainedType<'tree> {
    #[allow(clippy::match_single_binding, clippy::suspicious_else_formatting)]
    fn from_node(
        node: ::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        debug_assert_eq!(node.kind(), "constrained_type");
        Ok(Self {
            span: ::treesitter_types::Span::from(node),
            children: {
                #[allow(clippy::suspicious_else_formatting)]
                let non_field_children = {
                    let mut cursor = node.walk();
                    let mut result = ::std::vec::Vec::new();
                    if cursor.goto_first_child() {
                        loop {
                            if cursor.field_name().is_none()
                                && cursor.node().is_named()
                                && !cursor.node().is_extra()
                            {
                                result.push(cursor.node());
                            }
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
impl ::treesitter_types::Spanned for ConstrainedType<'_> {
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
pub struct DecoratedDefinition<'tree> {
    pub span: ::treesitter_types::Span,
    pub definition: DecoratedDefinitionDefinition<'tree>,
    pub children: ::std::vec::Vec<Decorator<'tree>>,
}
impl<'tree> ::treesitter_types::FromNode<'tree> for DecoratedDefinition<'tree> {
    #[allow(clippy::match_single_binding, clippy::suspicious_else_formatting)]
    fn from_node(
        node: ::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        debug_assert_eq!(node.kind(), "decorated_definition");
        Ok(Self {
            span: ::treesitter_types::Span::from(node),
            definition: {
                let child = node.child_by_field_name("definition").ok_or_else(|| {
                    ::treesitter_types::ParseError::missing_field("definition", node)
                })?;
                <DecoratedDefinitionDefinition as ::treesitter_types::FromNode>::from_node(
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
                    items.push(<Decorator as ::treesitter_types::FromNode>::from_node(
                        child, src,
                    )?);
                }
                items
            },
        })
    }
}
impl ::treesitter_types::Spanned for DecoratedDefinition<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        self.span
    }
}
#[derive(Debug, Clone)]
pub struct Decorator<'tree> {
    pub span: ::treesitter_types::Span,
    pub children: Expression<'tree>,
}
impl<'tree> ::treesitter_types::FromNode<'tree> for Decorator<'tree> {
    #[allow(clippy::match_single_binding, clippy::suspicious_else_formatting)]
    fn from_node(
        node: ::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        debug_assert_eq!(node.kind(), "decorator");
        Ok(Self {
            span: ::treesitter_types::Span::from(node),
            children: {
                #[allow(clippy::suspicious_else_formatting)]
                let non_field_children = {
                    let mut cursor = node.walk();
                    let mut result = ::std::vec::Vec::new();
                    if cursor.goto_first_child() {
                        loop {
                            if cursor.field_name().is_none()
                                && cursor.node().is_named()
                                && !cursor.node().is_extra()
                            {
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
                    fallback_child.ok_or_else(|| {
                        ::treesitter_types::ParseError::missing_field("children", node)
                    })?
                };
                <Expression as ::treesitter_types::FromNode>::from_node(child, src)?
            },
        })
    }
}
impl ::treesitter_types::Spanned for Decorator<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        self.span
    }
}
#[derive(Debug, Clone)]
pub struct DefaultParameter<'tree> {
    pub span: ::treesitter_types::Span,
    pub name: DefaultParameterName<'tree>,
    pub value: Expression<'tree>,
}
impl<'tree> ::treesitter_types::FromNode<'tree> for DefaultParameter<'tree> {
    #[allow(clippy::match_single_binding, clippy::suspicious_else_formatting)]
    fn from_node(
        node: ::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        debug_assert_eq!(node.kind(), "default_parameter");
        Ok(Self {
            span: ::treesitter_types::Span::from(node),
            name: {
                let child = node
                    .child_by_field_name("name")
                    .ok_or_else(|| ::treesitter_types::ParseError::missing_field("name", node))?;
                <DefaultParameterName as ::treesitter_types::FromNode>::from_node(child, src)?
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
impl ::treesitter_types::Spanned for DefaultParameter<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        self.span
    }
}
#[derive(Debug, Clone)]
pub struct DeleteStatement<'tree> {
    pub span: ::treesitter_types::Span,
    pub children: DeleteStatementChildren<'tree>,
}
impl<'tree> ::treesitter_types::FromNode<'tree> for DeleteStatement<'tree> {
    #[allow(clippy::match_single_binding, clippy::suspicious_else_formatting)]
    fn from_node(
        node: ::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        debug_assert_eq!(node.kind(), "delete_statement");
        Ok(Self {
            span: ::treesitter_types::Span::from(node),
            children: {
                #[allow(clippy::suspicious_else_formatting)]
                let non_field_children = {
                    let mut cursor = node.walk();
                    let mut result = ::std::vec::Vec::new();
                    if cursor.goto_first_child() {
                        loop {
                            if cursor.field_name().is_none()
                                && cursor.node().is_named()
                                && !cursor.node().is_extra()
                            {
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
                                        <DeleteStatementChildren as ::treesitter_types::FromNode>::from_node(
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
                <DeleteStatementChildren as ::treesitter_types::FromNode>::from_node(child, src)?
            },
        })
    }
}
impl ::treesitter_types::Spanned for DeleteStatement<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        self.span
    }
}
#[derive(Debug, Clone)]
pub struct DictPattern<'tree> {
    pub span: ::treesitter_types::Span,
    pub key: ::std::vec::Vec<DictPatternKey<'tree>>,
    pub value: ::std::vec::Vec<CasePattern<'tree>>,
    pub children: ::std::vec::Vec<SplatPattern<'tree>>,
}
impl<'tree> ::treesitter_types::FromNode<'tree> for DictPattern<'tree> {
    #[allow(clippy::match_single_binding, clippy::suspicious_else_formatting)]
    fn from_node(
        node: ::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        debug_assert_eq!(node.kind(), "dict_pattern");
        Ok(Self {
            span: ::treesitter_types::Span::from(node),
            key: {
                let mut cursor = node.walk();
                let mut items = ::std::vec::Vec::new();
                for child in node.children_by_field_name("key", &mut cursor) {
                    items.push(<DictPatternKey as ::treesitter_types::FromNode>::from_node(
                        child, src,
                    )?);
                }
                items
            },
            value: {
                let mut cursor = node.walk();
                let mut items = ::std::vec::Vec::new();
                for child in node.children_by_field_name("value", &mut cursor) {
                    items.push(<CasePattern as ::treesitter_types::FromNode>::from_node(
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
                    items.push(<SplatPattern as ::treesitter_types::FromNode>::from_node(
                        child, src,
                    )?);
                }
                items
            },
        })
    }
}
impl ::treesitter_types::Spanned for DictPattern<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        self.span
    }
}
#[derive(Debug, Clone)]
pub struct Dictionary<'tree> {
    pub span: ::treesitter_types::Span,
    pub children: ::std::vec::Vec<DictionaryChildren<'tree>>,
}
impl<'tree> ::treesitter_types::FromNode<'tree> for Dictionary<'tree> {
    #[allow(clippy::match_single_binding, clippy::suspicious_else_formatting)]
    fn from_node(
        node: ::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        debug_assert_eq!(node.kind(), "dictionary");
        Ok(Self {
            span: ::treesitter_types::Span::from(node),
            children: {
                #[allow(clippy::suspicious_else_formatting)]
                let non_field_children = {
                    let mut cursor = node.walk();
                    let mut result = ::std::vec::Vec::new();
                    if cursor.goto_first_child() {
                        loop {
                            if cursor.field_name().is_none()
                                && cursor.node().is_named()
                                && !cursor.node().is_extra()
                            {
                                result.push(cursor.node());
                            }
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
                        <DictionaryChildren as ::treesitter_types::FromNode>::from_node(
                            child, src,
                        )?,
                    );
                }
                items
            },
        })
    }
}
impl ::treesitter_types::Spanned for Dictionary<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        self.span
    }
}
#[derive(Debug, Clone)]
pub struct DictionaryComprehension<'tree> {
    pub span: ::treesitter_types::Span,
    pub body: Pair<'tree>,
    pub children: ::std::vec::Vec<DictionaryComprehensionChildren<'tree>>,
}
impl<'tree> ::treesitter_types::FromNode<'tree> for DictionaryComprehension<'tree> {
    #[allow(clippy::match_single_binding, clippy::suspicious_else_formatting)]
    fn from_node(
        node: ::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        debug_assert_eq!(node.kind(), "dictionary_comprehension");
        Ok(Self {
            span: ::treesitter_types::Span::from(node),
            body: {
                let child = node
                    .child_by_field_name("body")
                    .ok_or_else(|| ::treesitter_types::ParseError::missing_field("body", node))?;
                <Pair as ::treesitter_types::FromNode>::from_node(child, src)?
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
                            <DictionaryComprehensionChildren as ::treesitter_types::FromNode>::from_node(
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
impl ::treesitter_types::Spanned for DictionaryComprehension<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        self.span
    }
}
#[derive(Debug, Clone)]
pub struct DictionarySplat<'tree> {
    pub span: ::treesitter_types::Span,
    pub children: Expression<'tree>,
}
impl<'tree> ::treesitter_types::FromNode<'tree> for DictionarySplat<'tree> {
    #[allow(clippy::match_single_binding, clippy::suspicious_else_formatting)]
    fn from_node(
        node: ::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        debug_assert_eq!(node.kind(), "dictionary_splat");
        Ok(Self {
            span: ::treesitter_types::Span::from(node),
            children: {
                #[allow(clippy::suspicious_else_formatting)]
                let non_field_children = {
                    let mut cursor = node.walk();
                    let mut result = ::std::vec::Vec::new();
                    if cursor.goto_first_child() {
                        loop {
                            if cursor.field_name().is_none()
                                && cursor.node().is_named()
                                && !cursor.node().is_extra()
                            {
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
                    fallback_child.ok_or_else(|| {
                        ::treesitter_types::ParseError::missing_field("children", node)
                    })?
                };
                <Expression as ::treesitter_types::FromNode>::from_node(child, src)?
            },
        })
    }
}
impl ::treesitter_types::Spanned for DictionarySplat<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        self.span
    }
}
#[derive(Debug, Clone)]
pub struct DictionarySplatPattern<'tree> {
    pub span: ::treesitter_types::Span,
    pub children: DictionarySplatPatternChildren<'tree>,
}
impl<'tree> ::treesitter_types::FromNode<'tree> for DictionarySplatPattern<'tree> {
    #[allow(clippy::match_single_binding, clippy::suspicious_else_formatting)]
    fn from_node(
        node: ::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        debug_assert_eq!(node.kind(), "dictionary_splat_pattern");
        Ok(Self {
            span: ::treesitter_types::Span::from(node),
            children: {
                #[allow(clippy::suspicious_else_formatting)]
                let non_field_children = {
                    let mut cursor = node.walk();
                    let mut result = ::std::vec::Vec::new();
                    if cursor.goto_first_child() {
                        loop {
                            if cursor.field_name().is_none()
                                && cursor.node().is_named()
                                && !cursor.node().is_extra()
                            {
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
                                        <DictionarySplatPatternChildren as ::treesitter_types::FromNode>::from_node(
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
                <DictionarySplatPatternChildren as ::treesitter_types::FromNode>::from_node(
                    child, src,
                )?
            },
        })
    }
}
impl ::treesitter_types::Spanned for DictionarySplatPattern<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        self.span
    }
}
#[derive(Debug, Clone)]
pub struct DottedName<'tree> {
    pub span: ::treesitter_types::Span,
    pub children: ::std::vec::Vec<Identifier<'tree>>,
}
impl<'tree> ::treesitter_types::FromNode<'tree> for DottedName<'tree> {
    #[allow(clippy::match_single_binding, clippy::suspicious_else_formatting)]
    fn from_node(
        node: ::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        debug_assert_eq!(node.kind(), "dotted_name");
        Ok(Self {
            span: ::treesitter_types::Span::from(node),
            children: {
                #[allow(clippy::suspicious_else_formatting)]
                let non_field_children = {
                    let mut cursor = node.walk();
                    let mut result = ::std::vec::Vec::new();
                    if cursor.goto_first_child() {
                        loop {
                            if cursor.field_name().is_none()
                                && cursor.node().is_named()
                                && !cursor.node().is_extra()
                            {
                                result.push(cursor.node());
                            }
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
impl ::treesitter_types::Spanned for DottedName<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        self.span
    }
}
#[derive(Debug, Clone)]
pub struct ElifClause<'tree> {
    pub span: ::treesitter_types::Span,
    pub condition: Expression<'tree>,
    pub consequence: Block<'tree>,
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
                <Block as ::treesitter_types::FromNode>::from_node(child, src)?
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
    pub body: Block<'tree>,
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
            body: {
                let child = node
                    .child_by_field_name("body")
                    .ok_or_else(|| ::treesitter_types::ParseError::missing_field("body", node))?;
                <Block as ::treesitter_types::FromNode>::from_node(child, src)?
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
pub struct ExceptClause<'tree> {
    pub span: ::treesitter_types::Span,
    pub alias: ::core::option::Option<Expression<'tree>>,
    pub value: ::std::vec::Vec<Expression<'tree>>,
    pub children: Block<'tree>,
}
impl<'tree> ::treesitter_types::FromNode<'tree> for ExceptClause<'tree> {
    #[allow(clippy::match_single_binding, clippy::suspicious_else_formatting)]
    fn from_node(
        node: ::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        debug_assert_eq!(node.kind(), "except_clause");
        Ok(Self {
            span: ::treesitter_types::Span::from(node),
            alias: match node.child_by_field_name("alias") {
                Some(child) => Some(<Expression as ::treesitter_types::FromNode>::from_node(
                    child, src,
                )?),
                None => None,
            },
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
                                        <Block as ::treesitter_types::FromNode>::from_node(
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
                <Block as ::treesitter_types::FromNode>::from_node(child, src)?
            },
        })
    }
}
impl ::treesitter_types::Spanned for ExceptClause<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        self.span
    }
}
#[derive(Debug, Clone)]
pub struct ExecStatement<'tree> {
    pub span: ::treesitter_types::Span,
    pub code: ExecStatementCode<'tree>,
    pub children: ::std::vec::Vec<Expression<'tree>>,
}
impl<'tree> ::treesitter_types::FromNode<'tree> for ExecStatement<'tree> {
    #[allow(clippy::match_single_binding, clippy::suspicious_else_formatting)]
    fn from_node(
        node: ::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        debug_assert_eq!(node.kind(), "exec_statement");
        Ok(Self {
            span: ::treesitter_types::Span::from(node),
            code: {
                let child = node
                    .child_by_field_name("code")
                    .ok_or_else(|| ::treesitter_types::ParseError::missing_field("code", node))?;
                <ExecStatementCode as ::treesitter_types::FromNode>::from_node(child, src)?
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
impl ::treesitter_types::Spanned for ExecStatement<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        self.span
    }
}
#[derive(Debug, Clone)]
pub struct ExpressionList<'tree> {
    pub span: ::treesitter_types::Span,
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
            children: {
                #[allow(clippy::suspicious_else_formatting)]
                let non_field_children = {
                    let mut cursor = node.walk();
                    let mut result = ::std::vec::Vec::new();
                    if cursor.goto_first_child() {
                        loop {
                            if cursor.field_name().is_none()
                                && cursor.node().is_named()
                                && !cursor.node().is_extra()
                            {
                                result.push(cursor.node());
                            }
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
#[derive(Debug, Clone)]
pub struct ExpressionStatement<'tree> {
    pub span: ::treesitter_types::Span,
    pub children: ::std::vec::Vec<ExpressionStatementChildren<'tree>>,
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
                let mut items = ::std::vec::Vec::new();
                for child in non_field_children {
                    items.push(
                        <ExpressionStatementChildren as ::treesitter_types::FromNode>::from_node(
                            child, src,
                        )?,
                    );
                }
                items
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
                                let candidate = fallback_cursor.node();
                                #[allow(clippy::needless_question_mark)]
                                if (|| -> ::core::result::Result<
                                    _,
                                    ::treesitter_types::ParseError,
                                > {
                                    let child = candidate;
                                    Ok(
                                        <Block as ::treesitter_types::FromNode>::from_node(
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
pub struct ForInClause<'tree> {
    pub span: ::treesitter_types::Span,
    pub left: ForInClauseLeft<'tree>,
    pub right: ::std::vec::Vec<ForInClauseRight<'tree>>,
}
impl<'tree> ::treesitter_types::FromNode<'tree> for ForInClause<'tree> {
    #[allow(clippy::match_single_binding, clippy::suspicious_else_formatting)]
    fn from_node(
        node: ::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        debug_assert_eq!(node.kind(), "for_in_clause");
        Ok(Self {
            span: ::treesitter_types::Span::from(node),
            left: {
                let child = node
                    .child_by_field_name("left")
                    .ok_or_else(|| ::treesitter_types::ParseError::missing_field("left", node))?;
                <ForInClauseLeft as ::treesitter_types::FromNode>::from_node(child, src)?
            },
            right: {
                let mut cursor = node.walk();
                let mut items = ::std::vec::Vec::new();
                for child in node.children_by_field_name("right", &mut cursor) {
                    items.push(
                        <ForInClauseRight as ::treesitter_types::FromNode>::from_node(child, src)?,
                    );
                }
                items
            },
        })
    }
}
impl ::treesitter_types::Spanned for ForInClause<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        self.span
    }
}
#[derive(Debug, Clone)]
pub struct ForStatement<'tree> {
    pub span: ::treesitter_types::Span,
    pub alternative: ::core::option::Option<ElseClause<'tree>>,
    pub body: Block<'tree>,
    pub left: ForStatementLeft<'tree>,
    pub right: ForStatementRight<'tree>,
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
            alternative: match node.child_by_field_name("alternative") {
                Some(child) => Some(<ElseClause as ::treesitter_types::FromNode>::from_node(
                    child, src,
                )?),
                None => None,
            },
            body: {
                let child = node
                    .child_by_field_name("body")
                    .ok_or_else(|| ::treesitter_types::ParseError::missing_field("body", node))?;
                <Block as ::treesitter_types::FromNode>::from_node(child, src)?
            },
            left: {
                let child = node
                    .child_by_field_name("left")
                    .ok_or_else(|| ::treesitter_types::ParseError::missing_field("left", node))?;
                <ForStatementLeft as ::treesitter_types::FromNode>::from_node(child, src)?
            },
            right: {
                let child = node
                    .child_by_field_name("right")
                    .ok_or_else(|| ::treesitter_types::ParseError::missing_field("right", node))?;
                <ForStatementRight as ::treesitter_types::FromNode>::from_node(child, src)?
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
pub struct FormatExpression<'tree> {
    pub span: ::treesitter_types::Span,
    pub expression: FormatExpressionExpression<'tree>,
    pub format_specifier: ::core::option::Option<FormatSpecifier<'tree>>,
    pub type_conversion: ::core::option::Option<TypeConversion<'tree>>,
}
impl<'tree> ::treesitter_types::FromNode<'tree> for FormatExpression<'tree> {
    #[allow(clippy::match_single_binding, clippy::suspicious_else_formatting)]
    fn from_node(
        node: ::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        debug_assert_eq!(node.kind(), "format_expression");
        Ok(Self {
            span: ::treesitter_types::Span::from(node),
            expression: {
                let child = node.child_by_field_name("expression").ok_or_else(|| {
                    ::treesitter_types::ParseError::missing_field("expression", node)
                })?;
                <FormatExpressionExpression as ::treesitter_types::FromNode>::from_node(child, src)?
            },
            format_specifier: match node.child_by_field_name("format_specifier") {
                Some(child) => {
                    Some(<FormatSpecifier as ::treesitter_types::FromNode>::from_node(child, src)?)
                }
                None => None,
            },
            type_conversion: match node.child_by_field_name("type_conversion") {
                Some(child) => Some(<TypeConversion as ::treesitter_types::FromNode>::from_node(
                    child, src,
                )?),
                None => None,
            },
        })
    }
}
impl ::treesitter_types::Spanned for FormatExpression<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        self.span
    }
}
#[derive(Debug, Clone)]
pub struct FormatSpecifier<'tree> {
    pub span: ::treesitter_types::Span,
    pub children: ::std::vec::Vec<FormatExpression<'tree>>,
}
impl<'tree> ::treesitter_types::FromNode<'tree> for FormatSpecifier<'tree> {
    #[allow(clippy::match_single_binding, clippy::suspicious_else_formatting)]
    fn from_node(
        node: ::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        debug_assert_eq!(node.kind(), "format_specifier");
        Ok(Self {
            span: ::treesitter_types::Span::from(node),
            children: {
                #[allow(clippy::suspicious_else_formatting)]
                let non_field_children = {
                    let mut cursor = node.walk();
                    let mut result = ::std::vec::Vec::new();
                    if cursor.goto_first_child() {
                        loop {
                            if cursor.field_name().is_none()
                                && cursor.node().is_named()
                                && !cursor.node().is_extra()
                            {
                                result.push(cursor.node());
                            }
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
                        <FormatExpression as ::treesitter_types::FromNode>::from_node(child, src)?,
                    );
                }
                items
            },
        })
    }
}
impl ::treesitter_types::Spanned for FormatSpecifier<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        self.span
    }
}
#[derive(Debug, Clone)]
pub struct FunctionDefinition<'tree> {
    pub span: ::treesitter_types::Span,
    pub body: Block<'tree>,
    pub name: Identifier<'tree>,
    pub parameters: Parameters<'tree>,
    pub return_type: ::core::option::Option<Type<'tree>>,
    pub type_parameters: ::core::option::Option<TypeParameter<'tree>>,
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
                <Block as ::treesitter_types::FromNode>::from_node(child, src)?
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
                <Parameters as ::treesitter_types::FromNode>::from_node(child, src)?
            },
            return_type: match node.child_by_field_name("return_type") {
                Some(child) => Some(<Type as ::treesitter_types::FromNode>::from_node(
                    child, src,
                )?),
                None => None,
            },
            type_parameters: match node.child_by_field_name("type_parameters") {
                Some(child) => Some(<TypeParameter as ::treesitter_types::FromNode>::from_node(
                    child, src,
                )?),
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
pub struct FutureImportStatement<'tree> {
    pub span: ::treesitter_types::Span,
    pub name: ::std::vec::Vec<FutureImportStatementName<'tree>>,
}
impl<'tree> ::treesitter_types::FromNode<'tree> for FutureImportStatement<'tree> {
    #[allow(clippy::match_single_binding, clippy::suspicious_else_formatting)]
    fn from_node(
        node: ::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        debug_assert_eq!(node.kind(), "future_import_statement");
        Ok(Self {
            span: ::treesitter_types::Span::from(node),
            name: {
                let mut cursor = node.walk();
                let mut items = ::std::vec::Vec::new();
                for child in node.children_by_field_name("name", &mut cursor) {
                    items.push(
                        <FutureImportStatementName as ::treesitter_types::FromNode>::from_node(
                            child, src,
                        )?,
                    );
                }
                items
            },
        })
    }
}
impl ::treesitter_types::Spanned for FutureImportStatement<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        self.span
    }
}
#[derive(Debug, Clone)]
pub struct GeneratorExpression<'tree> {
    pub span: ::treesitter_types::Span,
    pub body: Expression<'tree>,
    pub children: ::std::vec::Vec<GeneratorExpressionChildren<'tree>>,
}
impl<'tree> ::treesitter_types::FromNode<'tree> for GeneratorExpression<'tree> {
    #[allow(clippy::match_single_binding, clippy::suspicious_else_formatting)]
    fn from_node(
        node: ::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        debug_assert_eq!(node.kind(), "generator_expression");
        Ok(Self {
            span: ::treesitter_types::Span::from(node),
            body: {
                let child = node
                    .child_by_field_name("body")
                    .ok_or_else(|| ::treesitter_types::ParseError::missing_field("body", node))?;
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
                let mut items = ::std::vec::Vec::new();
                for child in non_field_children {
                    items.push(
                        <GeneratorExpressionChildren as ::treesitter_types::FromNode>::from_node(
                            child, src,
                        )?,
                    );
                }
                items
            },
        })
    }
}
impl ::treesitter_types::Spanned for GeneratorExpression<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        self.span
    }
}
#[derive(Debug, Clone)]
pub struct GenericType<'tree> {
    pub span: ::treesitter_types::Span,
    pub children: ::std::vec::Vec<GenericTypeChildren<'tree>>,
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
            children: {
                #[allow(clippy::suspicious_else_formatting)]
                let non_field_children = {
                    let mut cursor = node.walk();
                    let mut result = ::std::vec::Vec::new();
                    if cursor.goto_first_child() {
                        loop {
                            if cursor.field_name().is_none()
                                && cursor.node().is_named()
                                && !cursor.node().is_extra()
                            {
                                result.push(cursor.node());
                            }
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
                        <GenericTypeChildren as ::treesitter_types::FromNode>::from_node(
                            child, src,
                        )?,
                    );
                }
                items
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
pub struct GlobalStatement<'tree> {
    pub span: ::treesitter_types::Span,
    pub children: ::std::vec::Vec<Identifier<'tree>>,
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
impl ::treesitter_types::Spanned for GlobalStatement<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        self.span
    }
}
#[derive(Debug, Clone)]
pub struct IfClause<'tree> {
    pub span: ::treesitter_types::Span,
    pub children: Expression<'tree>,
}
impl<'tree> ::treesitter_types::FromNode<'tree> for IfClause<'tree> {
    #[allow(clippy::match_single_binding, clippy::suspicious_else_formatting)]
    fn from_node(
        node: ::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        debug_assert_eq!(node.kind(), "if_clause");
        Ok(Self {
            span: ::treesitter_types::Span::from(node),
            children: {
                #[allow(clippy::suspicious_else_formatting)]
                let non_field_children = {
                    let mut cursor = node.walk();
                    let mut result = ::std::vec::Vec::new();
                    if cursor.goto_first_child() {
                        loop {
                            if cursor.field_name().is_none()
                                && cursor.node().is_named()
                                && !cursor.node().is_extra()
                            {
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
                    fallback_child.ok_or_else(|| {
                        ::treesitter_types::ParseError::missing_field("children", node)
                    })?
                };
                <Expression as ::treesitter_types::FromNode>::from_node(child, src)?
            },
        })
    }
}
impl ::treesitter_types::Spanned for IfClause<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        self.span
    }
}
#[derive(Debug, Clone)]
pub struct IfStatement<'tree> {
    pub span: ::treesitter_types::Span,
    pub alternative: ::std::vec::Vec<IfStatementAlternative<'tree>>,
    pub condition: Expression<'tree>,
    pub consequence: Block<'tree>,
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
            consequence: {
                let child = node.child_by_field_name("consequence").ok_or_else(|| {
                    ::treesitter_types::ParseError::missing_field("consequence", node)
                })?;
                <Block as ::treesitter_types::FromNode>::from_node(child, src)?
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
pub struct ImportFromStatement<'tree> {
    pub span: ::treesitter_types::Span,
    pub module_name: ImportFromStatementModuleName<'tree>,
    pub name: ::std::vec::Vec<ImportFromStatementName<'tree>>,
    pub children: ::core::option::Option<WildcardImport<'tree>>,
}
impl<'tree> ::treesitter_types::FromNode<'tree> for ImportFromStatement<'tree> {
    #[allow(clippy::match_single_binding, clippy::suspicious_else_formatting)]
    fn from_node(
        node: ::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        debug_assert_eq!(node.kind(), "import_from_statement");
        Ok(Self {
            span: ::treesitter_types::Span::from(node),
            module_name: {
                let child = node.child_by_field_name("module_name").ok_or_else(|| {
                    ::treesitter_types::ParseError::missing_field("module_name", node)
                })?;
                <ImportFromStatementModuleName as ::treesitter_types::FromNode>::from_node(
                    child, src,
                )?
            },
            name: {
                let mut cursor = node.walk();
                let mut items = ::std::vec::Vec::new();
                for child in node.children_by_field_name("name", &mut cursor) {
                    items.push(
                        <ImportFromStatementName as ::treesitter_types::FromNode>::from_node(
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
                        <WildcardImport as ::treesitter_types::FromNode>::from_node(child, src)?,
                    ),
                    None => None,
                }
            },
        })
    }
}
impl ::treesitter_types::Spanned for ImportFromStatement<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        self.span
    }
}
#[derive(Debug, Clone)]
pub struct ImportPrefix<'tree> {
    pub span: ::treesitter_types::Span,
    text: &'tree str,
}
impl<'tree> ::treesitter_types::FromNode<'tree> for ImportPrefix<'tree> {
    fn from_node(
        node: ::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        debug_assert_eq!(node.kind(), "import_prefix");
        Ok(Self {
            span: ::treesitter_types::Span::from(node),
            text: node.utf8_text(src)?,
        })
    }
}
impl<'tree> ::treesitter_types::LeafNode<'tree> for ImportPrefix<'tree> {
    fn text(&self) -> &'tree str {
        self.text
    }
}
impl ::treesitter_types::Spanned for ImportPrefix<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        self.span
    }
}
#[derive(Debug, Clone)]
pub struct ImportStatement<'tree> {
    pub span: ::treesitter_types::Span,
    pub name: ::std::vec::Vec<ImportStatementName<'tree>>,
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
            name: {
                let mut cursor = node.walk();
                let mut items = ::std::vec::Vec::new();
                for child in node.children_by_field_name("name", &mut cursor) {
                    items.push(
                        <ImportStatementName as ::treesitter_types::FromNode>::from_node(
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
pub struct Interpolation<'tree> {
    pub span: ::treesitter_types::Span,
    pub expression: InterpolationExpression<'tree>,
    pub format_specifier: ::core::option::Option<FormatSpecifier<'tree>>,
    pub type_conversion: ::core::option::Option<TypeConversion<'tree>>,
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
            expression: {
                let child = node.child_by_field_name("expression").ok_or_else(|| {
                    ::treesitter_types::ParseError::missing_field("expression", node)
                })?;
                <InterpolationExpression as ::treesitter_types::FromNode>::from_node(child, src)?
            },
            format_specifier: match node.child_by_field_name("format_specifier") {
                Some(child) => {
                    Some(<FormatSpecifier as ::treesitter_types::FromNode>::from_node(child, src)?)
                }
                None => None,
            },
            type_conversion: match node.child_by_field_name("type_conversion") {
                Some(child) => Some(<TypeConversion as ::treesitter_types::FromNode>::from_node(
                    child, src,
                )?),
                None => None,
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
pub struct KeywordArgument<'tree> {
    pub span: ::treesitter_types::Span,
    pub name: Identifier<'tree>,
    pub value: Expression<'tree>,
}
impl<'tree> ::treesitter_types::FromNode<'tree> for KeywordArgument<'tree> {
    #[allow(clippy::match_single_binding, clippy::suspicious_else_formatting)]
    fn from_node(
        node: ::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        debug_assert_eq!(node.kind(), "keyword_argument");
        Ok(Self {
            span: ::treesitter_types::Span::from(node),
            name: {
                let child = node
                    .child_by_field_name("name")
                    .ok_or_else(|| ::treesitter_types::ParseError::missing_field("name", node))?;
                <Identifier as ::treesitter_types::FromNode>::from_node(child, src)?
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
impl ::treesitter_types::Spanned for KeywordArgument<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        self.span
    }
}
#[derive(Debug, Clone)]
pub struct KeywordPattern<'tree> {
    pub span: ::treesitter_types::Span,
    pub children: ::std::vec::Vec<KeywordPatternChildren<'tree>>,
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
            children: {
                #[allow(clippy::suspicious_else_formatting)]
                let non_field_children = {
                    let mut cursor = node.walk();
                    let mut result = ::std::vec::Vec::new();
                    if cursor.goto_first_child() {
                        loop {
                            if cursor.field_name().is_none()
                                && cursor.node().is_named()
                                && !cursor.node().is_extra()
                            {
                                result.push(cursor.node());
                            }
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
                        <KeywordPatternChildren as ::treesitter_types::FromNode>::from_node(
                            child, src,
                        )?,
                    );
                }
                items
            },
        })
    }
}
impl ::treesitter_types::Spanned for KeywordPattern<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        self.span
    }
}
#[derive(Debug, Clone)]
pub struct KeywordSeparator<'tree> {
    pub span: ::treesitter_types::Span,
    text: &'tree str,
}
impl<'tree> ::treesitter_types::FromNode<'tree> for KeywordSeparator<'tree> {
    fn from_node(
        node: ::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        debug_assert_eq!(node.kind(), "keyword_separator");
        Ok(Self {
            span: ::treesitter_types::Span::from(node),
            text: node.utf8_text(src)?,
        })
    }
}
impl<'tree> ::treesitter_types::LeafNode<'tree> for KeywordSeparator<'tree> {
    fn text(&self) -> &'tree str {
        self.text
    }
}
impl ::treesitter_types::Spanned for KeywordSeparator<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        self.span
    }
}
#[derive(Debug, Clone)]
pub struct Lambda<'tree> {
    pub span: ::treesitter_types::Span,
    pub body: Expression<'tree>,
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
                <Expression as ::treesitter_types::FromNode>::from_node(child, src)?
            },
            parameters: match node.child_by_field_name("parameters") {
                Some(child) => {
                    Some(<LambdaParameters as ::treesitter_types::FromNode>::from_node(child, src)?)
                }
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
#[derive(Debug, Clone)]
pub struct LambdaParameters<'tree> {
    pub span: ::treesitter_types::Span,
    pub children: ::std::vec::Vec<Parameter<'tree>>,
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
                    items.push(<Parameter as ::treesitter_types::FromNode>::from_node(
                        child, src,
                    )?);
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
#[derive(Debug, Clone)]
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
#[derive(Debug, Clone)]
pub struct ListComprehension<'tree> {
    pub span: ::treesitter_types::Span,
    pub body: Expression<'tree>,
    pub children: ::std::vec::Vec<ListComprehensionChildren<'tree>>,
}
impl<'tree> ::treesitter_types::FromNode<'tree> for ListComprehension<'tree> {
    #[allow(clippy::match_single_binding, clippy::suspicious_else_formatting)]
    fn from_node(
        node: ::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        debug_assert_eq!(node.kind(), "list_comprehension");
        Ok(Self {
            span: ::treesitter_types::Span::from(node),
            body: {
                let child = node
                    .child_by_field_name("body")
                    .ok_or_else(|| ::treesitter_types::ParseError::missing_field("body", node))?;
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
                let mut items = ::std::vec::Vec::new();
                for child in non_field_children {
                    items.push(
                        <ListComprehensionChildren as ::treesitter_types::FromNode>::from_node(
                            child, src,
                        )?,
                    );
                }
                items
            },
        })
    }
}
impl ::treesitter_types::Spanned for ListComprehension<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        self.span
    }
}
#[derive(Debug, Clone)]
pub struct ListPattern<'tree> {
    pub span: ::treesitter_types::Span,
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
            children: {
                #[allow(clippy::suspicious_else_formatting)]
                let non_field_children = {
                    let mut cursor = node.walk();
                    let mut result = ::std::vec::Vec::new();
                    if cursor.goto_first_child() {
                        loop {
                            if cursor.field_name().is_none()
                                && cursor.node().is_named()
                                && !cursor.node().is_extra()
                            {
                                result.push(cursor.node());
                            }
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
pub struct ListSplat<'tree> {
    pub span: ::treesitter_types::Span,
    pub children: ListSplatChildren<'tree>,
}
impl<'tree> ::treesitter_types::FromNode<'tree> for ListSplat<'tree> {
    #[allow(clippy::match_single_binding, clippy::suspicious_else_formatting)]
    fn from_node(
        node: ::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        debug_assert_eq!(node.kind(), "list_splat");
        Ok(Self {
            span: ::treesitter_types::Span::from(node),
            children: {
                #[allow(clippy::suspicious_else_formatting)]
                let non_field_children = {
                    let mut cursor = node.walk();
                    let mut result = ::std::vec::Vec::new();
                    if cursor.goto_first_child() {
                        loop {
                            if cursor.field_name().is_none()
                                && cursor.node().is_named()
                                && !cursor.node().is_extra()
                            {
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
                                        <ListSplatChildren as ::treesitter_types::FromNode>::from_node(
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
                <ListSplatChildren as ::treesitter_types::FromNode>::from_node(child, src)?
            },
        })
    }
}
impl ::treesitter_types::Spanned for ListSplat<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        self.span
    }
}
#[derive(Debug, Clone)]
pub struct ListSplatPattern<'tree> {
    pub span: ::treesitter_types::Span,
    pub children: ListSplatPatternChildren<'tree>,
}
impl<'tree> ::treesitter_types::FromNode<'tree> for ListSplatPattern<'tree> {
    #[allow(clippy::match_single_binding, clippy::suspicious_else_formatting)]
    fn from_node(
        node: ::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        debug_assert_eq!(node.kind(), "list_splat_pattern");
        Ok(Self {
            span: ::treesitter_types::Span::from(node),
            children: {
                #[allow(clippy::suspicious_else_formatting)]
                let non_field_children = {
                    let mut cursor = node.walk();
                    let mut result = ::std::vec::Vec::new();
                    if cursor.goto_first_child() {
                        loop {
                            if cursor.field_name().is_none()
                                && cursor.node().is_named()
                                && !cursor.node().is_extra()
                            {
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
                                        <ListSplatPatternChildren as ::treesitter_types::FromNode>::from_node(
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
                <ListSplatPatternChildren as ::treesitter_types::FromNode>::from_node(child, src)?
            },
        })
    }
}
impl ::treesitter_types::Spanned for ListSplatPattern<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        self.span
    }
}
#[derive(Debug, Clone)]
pub struct MatchStatement<'tree> {
    pub span: ::treesitter_types::Span,
    pub body: Block<'tree>,
    pub subject: ::std::vec::Vec<Expression<'tree>>,
}
impl<'tree> ::treesitter_types::FromNode<'tree> for MatchStatement<'tree> {
    #[allow(clippy::match_single_binding, clippy::suspicious_else_formatting)]
    fn from_node(
        node: ::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        debug_assert_eq!(node.kind(), "match_statement");
        Ok(Self {
            span: ::treesitter_types::Span::from(node),
            body: {
                let child = node
                    .child_by_field_name("body")
                    .ok_or_else(|| ::treesitter_types::ParseError::missing_field("body", node))?;
                <Block as ::treesitter_types::FromNode>::from_node(child, src)?
            },
            subject: {
                let mut cursor = node.walk();
                let mut items = ::std::vec::Vec::new();
                for child in node.children_by_field_name("subject", &mut cursor) {
                    items.push(<Expression as ::treesitter_types::FromNode>::from_node(
                        child, src,
                    )?);
                }
                items
            },
        })
    }
}
impl ::treesitter_types::Spanned for MatchStatement<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        self.span
    }
}
#[derive(Debug, Clone)]
pub struct MemberType<'tree> {
    pub span: ::treesitter_types::Span,
    pub children: ::std::vec::Vec<MemberTypeChildren<'tree>>,
}
impl<'tree> ::treesitter_types::FromNode<'tree> for MemberType<'tree> {
    #[allow(clippy::match_single_binding, clippy::suspicious_else_formatting)]
    fn from_node(
        node: ::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        debug_assert_eq!(node.kind(), "member_type");
        Ok(Self {
            span: ::treesitter_types::Span::from(node),
            children: {
                #[allow(clippy::suspicious_else_formatting)]
                let non_field_children = {
                    let mut cursor = node.walk();
                    let mut result = ::std::vec::Vec::new();
                    if cursor.goto_first_child() {
                        loop {
                            if cursor.field_name().is_none()
                                && cursor.node().is_named()
                                && !cursor.node().is_extra()
                            {
                                result.push(cursor.node());
                            }
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
                        <MemberTypeChildren as ::treesitter_types::FromNode>::from_node(
                            child, src,
                        )?,
                    );
                }
                items
            },
        })
    }
}
impl ::treesitter_types::Spanned for MemberType<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        self.span
    }
}
#[derive(Debug, Clone)]
pub struct Module<'tree> {
    pub span: ::treesitter_types::Span,
    pub children: ::std::vec::Vec<ModuleChildren<'tree>>,
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
            children: {
                #[allow(clippy::suspicious_else_formatting)]
                let non_field_children = {
                    let mut cursor = node.walk();
                    let mut result = ::std::vec::Vec::new();
                    if cursor.goto_first_child() {
                        loop {
                            if cursor.field_name().is_none()
                                && cursor.node().is_named()
                                && !cursor.node().is_extra()
                            {
                                result.push(cursor.node());
                            }
                            if !cursor.goto_next_sibling() {
                                break;
                            }
                        }
                    }
                    result
                };
                let mut items = ::std::vec::Vec::new();
                for child in non_field_children {
                    items.push(<ModuleChildren as ::treesitter_types::FromNode>::from_node(
                        child, src,
                    )?);
                }
                items
            },
        })
    }
}
impl ::treesitter_types::Spanned for Module<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        self.span
    }
}
#[derive(Debug, Clone)]
pub struct NamedExpression<'tree> {
    pub span: ::treesitter_types::Span,
    pub name: Identifier<'tree>,
    pub value: Expression<'tree>,
}
impl<'tree> ::treesitter_types::FromNode<'tree> for NamedExpression<'tree> {
    #[allow(clippy::match_single_binding, clippy::suspicious_else_formatting)]
    fn from_node(
        node: ::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        debug_assert_eq!(node.kind(), "named_expression");
        Ok(Self {
            span: ::treesitter_types::Span::from(node),
            name: {
                let child = node
                    .child_by_field_name("name")
                    .ok_or_else(|| ::treesitter_types::ParseError::missing_field("name", node))?;
                <Identifier as ::treesitter_types::FromNode>::from_node(child, src)?
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
impl ::treesitter_types::Spanned for NamedExpression<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        self.span
    }
}
#[derive(Debug, Clone)]
pub struct NonlocalStatement<'tree> {
    pub span: ::treesitter_types::Span,
    pub children: ::std::vec::Vec<Identifier<'tree>>,
}
impl<'tree> ::treesitter_types::FromNode<'tree> for NonlocalStatement<'tree> {
    #[allow(clippy::match_single_binding, clippy::suspicious_else_formatting)]
    fn from_node(
        node: ::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        debug_assert_eq!(node.kind(), "nonlocal_statement");
        Ok(Self {
            span: ::treesitter_types::Span::from(node),
            children: {
                #[allow(clippy::suspicious_else_formatting)]
                let non_field_children = {
                    let mut cursor = node.walk();
                    let mut result = ::std::vec::Vec::new();
                    if cursor.goto_first_child() {
                        loop {
                            if cursor.field_name().is_none()
                                && cursor.node().is_named()
                                && !cursor.node().is_extra()
                            {
                                result.push(cursor.node());
                            }
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
impl ::treesitter_types::Spanned for NonlocalStatement<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        self.span
    }
}
#[derive(Debug, Clone)]
pub struct NotOperator<'tree> {
    pub span: ::treesitter_types::Span,
    pub argument: Expression<'tree>,
}
impl<'tree> ::treesitter_types::FromNode<'tree> for NotOperator<'tree> {
    #[allow(clippy::match_single_binding, clippy::suspicious_else_formatting)]
    fn from_node(
        node: ::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        debug_assert_eq!(node.kind(), "not_operator");
        Ok(Self {
            span: ::treesitter_types::Span::from(node),
            argument: {
                let child = node.child_by_field_name("argument").ok_or_else(|| {
                    ::treesitter_types::ParseError::missing_field("argument", node)
                })?;
                <Expression as ::treesitter_types::FromNode>::from_node(child, src)?
            },
        })
    }
}
impl ::treesitter_types::Spanned for NotOperator<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        self.span
    }
}
#[derive(Debug, Clone)]
pub struct Pair<'tree> {
    pub span: ::treesitter_types::Span,
    pub key: Expression<'tree>,
    pub value: Expression<'tree>,
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
                <Expression as ::treesitter_types::FromNode>::from_node(child, src)?
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
impl ::treesitter_types::Spanned for Pair<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        self.span
    }
}
#[derive(Debug, Clone)]
pub struct Parameters<'tree> {
    pub span: ::treesitter_types::Span,
    pub children: ::std::vec::Vec<Parameter<'tree>>,
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
                    items.push(<Parameter as ::treesitter_types::FromNode>::from_node(
                        child, src,
                    )?);
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
                                let candidate = fallback_cursor.node();
                                #[allow(clippy::needless_question_mark)]
                                if (|| -> ::core::result::Result<
                                    _,
                                    ::treesitter_types::ParseError,
                                > {
                                    let child = candidate;
                                    Ok(
                                        <ParenthesizedExpressionChildren as ::treesitter_types::FromNode>::from_node(
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
pub struct ParenthesizedListSplat<'tree> {
    pub span: ::treesitter_types::Span,
    pub children: ParenthesizedListSplatChildren<'tree>,
}
impl<'tree> ::treesitter_types::FromNode<'tree> for ParenthesizedListSplat<'tree> {
    #[allow(clippy::match_single_binding, clippy::suspicious_else_formatting)]
    fn from_node(
        node: ::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        debug_assert_eq!(node.kind(), "parenthesized_list_splat");
        Ok(Self {
            span: ::treesitter_types::Span::from(node),
            children: {
                #[allow(clippy::suspicious_else_formatting)]
                let non_field_children = {
                    let mut cursor = node.walk();
                    let mut result = ::std::vec::Vec::new();
                    if cursor.goto_first_child() {
                        loop {
                            if cursor.field_name().is_none()
                                && cursor.node().is_named()
                                && !cursor.node().is_extra()
                            {
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
                                        <ParenthesizedListSplatChildren as ::treesitter_types::FromNode>::from_node(
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
                <ParenthesizedListSplatChildren as ::treesitter_types::FromNode>::from_node(
                    child, src,
                )?
            },
        })
    }
}
impl ::treesitter_types::Spanned for ParenthesizedListSplat<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        self.span
    }
}
#[derive(Debug, Clone)]
pub struct PassStatement<'tree> {
    pub span: ::treesitter_types::Span,
    text: &'tree str,
}
impl<'tree> ::treesitter_types::FromNode<'tree> for PassStatement<'tree> {
    fn from_node(
        node: ::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        debug_assert_eq!(node.kind(), "pass_statement");
        Ok(Self {
            span: ::treesitter_types::Span::from(node),
            text: node.utf8_text(src)?,
        })
    }
}
impl<'tree> ::treesitter_types::LeafNode<'tree> for PassStatement<'tree> {
    fn text(&self) -> &'tree str {
        self.text
    }
}
impl ::treesitter_types::Spanned for PassStatement<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        self.span
    }
}
#[derive(Debug, Clone)]
pub struct PatternList<'tree> {
    pub span: ::treesitter_types::Span,
    pub children: ::std::vec::Vec<Pattern<'tree>>,
}
impl<'tree> ::treesitter_types::FromNode<'tree> for PatternList<'tree> {
    #[allow(clippy::match_single_binding, clippy::suspicious_else_formatting)]
    fn from_node(
        node: ::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        debug_assert_eq!(node.kind(), "pattern_list");
        Ok(Self {
            span: ::treesitter_types::Span::from(node),
            children: {
                #[allow(clippy::suspicious_else_formatting)]
                let non_field_children = {
                    let mut cursor = node.walk();
                    let mut result = ::std::vec::Vec::new();
                    if cursor.goto_first_child() {
                        loop {
                            if cursor.field_name().is_none()
                                && cursor.node().is_named()
                                && !cursor.node().is_extra()
                            {
                                result.push(cursor.node());
                            }
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
impl ::treesitter_types::Spanned for PatternList<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        self.span
    }
}
#[derive(Debug, Clone)]
pub struct PositionalSeparator<'tree> {
    pub span: ::treesitter_types::Span,
    text: &'tree str,
}
impl<'tree> ::treesitter_types::FromNode<'tree> for PositionalSeparator<'tree> {
    fn from_node(
        node: ::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        debug_assert_eq!(node.kind(), "positional_separator");
        Ok(Self {
            span: ::treesitter_types::Span::from(node),
            text: node.utf8_text(src)?,
        })
    }
}
impl<'tree> ::treesitter_types::LeafNode<'tree> for PositionalSeparator<'tree> {
    fn text(&self) -> &'tree str {
        self.text
    }
}
impl ::treesitter_types::Spanned for PositionalSeparator<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        self.span
    }
}
#[derive(Debug, Clone)]
pub struct PrintStatement<'tree> {
    pub span: ::treesitter_types::Span,
    pub argument: ::std::vec::Vec<Expression<'tree>>,
    pub children: ::core::option::Option<Chevron<'tree>>,
}
impl<'tree> ::treesitter_types::FromNode<'tree> for PrintStatement<'tree> {
    #[allow(clippy::match_single_binding, clippy::suspicious_else_formatting)]
    fn from_node(
        node: ::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        debug_assert_eq!(node.kind(), "print_statement");
        Ok(Self {
            span: ::treesitter_types::Span::from(node),
            argument: {
                let mut cursor = node.walk();
                let mut items = ::std::vec::Vec::new();
                for child in node.children_by_field_name("argument", &mut cursor) {
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
                match non_field_children.first() {
                    Some(&child) => Some(<Chevron as ::treesitter_types::FromNode>::from_node(
                        child, src,
                    )?),
                    None => None,
                }
            },
        })
    }
}
impl ::treesitter_types::Spanned for PrintStatement<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        self.span
    }
}
#[derive(Debug, Clone)]
pub struct RaiseStatement<'tree> {
    pub span: ::treesitter_types::Span,
    pub cause: ::core::option::Option<Expression<'tree>>,
    pub children: ::core::option::Option<RaiseStatementChildren<'tree>>,
}
impl<'tree> ::treesitter_types::FromNode<'tree> for RaiseStatement<'tree> {
    #[allow(clippy::match_single_binding, clippy::suspicious_else_formatting)]
    fn from_node(
        node: ::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        debug_assert_eq!(node.kind(), "raise_statement");
        Ok(Self {
            span: ::treesitter_types::Span::from(node),
            cause: match node.child_by_field_name("cause") {
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
                match non_field_children.first() {
                    Some(&child) => Some(
                        <RaiseStatementChildren as ::treesitter_types::FromNode>::from_node(
                            child, src,
                        )?,
                    ),
                    None => None,
                }
            },
        })
    }
}
impl ::treesitter_types::Spanned for RaiseStatement<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        self.span
    }
}
#[derive(Debug, Clone)]
pub struct RelativeImport<'tree> {
    pub span: ::treesitter_types::Span,
    pub children: ::std::vec::Vec<RelativeImportChildren<'tree>>,
}
impl<'tree> ::treesitter_types::FromNode<'tree> for RelativeImport<'tree> {
    #[allow(clippy::match_single_binding, clippy::suspicious_else_formatting)]
    fn from_node(
        node: ::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        debug_assert_eq!(node.kind(), "relative_import");
        Ok(Self {
            span: ::treesitter_types::Span::from(node),
            children: {
                #[allow(clippy::suspicious_else_formatting)]
                let non_field_children = {
                    let mut cursor = node.walk();
                    let mut result = ::std::vec::Vec::new();
                    if cursor.goto_first_child() {
                        loop {
                            if cursor.field_name().is_none()
                                && cursor.node().is_named()
                                && !cursor.node().is_extra()
                            {
                                result.push(cursor.node());
                            }
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
                        <RelativeImportChildren as ::treesitter_types::FromNode>::from_node(
                            child, src,
                        )?,
                    );
                }
                items
            },
        })
    }
}
impl ::treesitter_types::Spanned for RelativeImport<'_> {
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
pub struct Set<'tree> {
    pub span: ::treesitter_types::Span,
    pub children: ::std::vec::Vec<SetChildren<'tree>>,
}
impl<'tree> ::treesitter_types::FromNode<'tree> for Set<'tree> {
    #[allow(clippy::match_single_binding, clippy::suspicious_else_formatting)]
    fn from_node(
        node: ::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        debug_assert_eq!(node.kind(), "set");
        Ok(Self {
            span: ::treesitter_types::Span::from(node),
            children: {
                #[allow(clippy::suspicious_else_formatting)]
                let non_field_children = {
                    let mut cursor = node.walk();
                    let mut result = ::std::vec::Vec::new();
                    if cursor.goto_first_child() {
                        loop {
                            if cursor.field_name().is_none()
                                && cursor.node().is_named()
                                && !cursor.node().is_extra()
                            {
                                result.push(cursor.node());
                            }
                            if !cursor.goto_next_sibling() {
                                break;
                            }
                        }
                    }
                    result
                };
                let mut items = ::std::vec::Vec::new();
                for child in non_field_children {
                    items.push(<SetChildren as ::treesitter_types::FromNode>::from_node(
                        child, src,
                    )?);
                }
                items
            },
        })
    }
}
impl ::treesitter_types::Spanned for Set<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        self.span
    }
}
#[derive(Debug, Clone)]
pub struct SetComprehension<'tree> {
    pub span: ::treesitter_types::Span,
    pub body: Expression<'tree>,
    pub children: ::std::vec::Vec<SetComprehensionChildren<'tree>>,
}
impl<'tree> ::treesitter_types::FromNode<'tree> for SetComprehension<'tree> {
    #[allow(clippy::match_single_binding, clippy::suspicious_else_formatting)]
    fn from_node(
        node: ::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        debug_assert_eq!(node.kind(), "set_comprehension");
        Ok(Self {
            span: ::treesitter_types::Span::from(node),
            body: {
                let child = node
                    .child_by_field_name("body")
                    .ok_or_else(|| ::treesitter_types::ParseError::missing_field("body", node))?;
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
                let mut items = ::std::vec::Vec::new();
                for child in non_field_children {
                    items.push(
                        <SetComprehensionChildren as ::treesitter_types::FromNode>::from_node(
                            child, src,
                        )?,
                    );
                }
                items
            },
        })
    }
}
impl ::treesitter_types::Spanned for SetComprehension<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        self.span
    }
}
#[derive(Debug, Clone)]
pub struct Slice<'tree> {
    pub span: ::treesitter_types::Span,
    pub children: ::std::vec::Vec<Expression<'tree>>,
}
impl<'tree> ::treesitter_types::FromNode<'tree> for Slice<'tree> {
    #[allow(clippy::match_single_binding, clippy::suspicious_else_formatting)]
    fn from_node(
        node: ::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        debug_assert_eq!(node.kind(), "slice");
        Ok(Self {
            span: ::treesitter_types::Span::from(node),
            children: {
                #[allow(clippy::suspicious_else_formatting)]
                let non_field_children = {
                    let mut cursor = node.walk();
                    let mut result = ::std::vec::Vec::new();
                    if cursor.goto_first_child() {
                        loop {
                            if cursor.field_name().is_none()
                                && cursor.node().is_named()
                                && !cursor.node().is_extra()
                            {
                                result.push(cursor.node());
                            }
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
impl ::treesitter_types::Spanned for Slice<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        self.span
    }
}
#[derive(Debug, Clone)]
pub struct SplatPattern<'tree> {
    pub span: ::treesitter_types::Span,
    pub children: ::core::option::Option<Identifier<'tree>>,
}
impl<'tree> ::treesitter_types::FromNode<'tree> for SplatPattern<'tree> {
    #[allow(clippy::match_single_binding, clippy::suspicious_else_formatting)]
    fn from_node(
        node: ::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        debug_assert_eq!(node.kind(), "splat_pattern");
        Ok(Self {
            span: ::treesitter_types::Span::from(node),
            children: {
                #[allow(clippy::suspicious_else_formatting)]
                let non_field_children = {
                    let mut cursor = node.walk();
                    let mut result = ::std::vec::Vec::new();
                    if cursor.goto_first_child() {
                        loop {
                            if cursor.field_name().is_none()
                                && cursor.node().is_named()
                                && !cursor.node().is_extra()
                            {
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
impl ::treesitter_types::Spanned for SplatPattern<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        self.span
    }
}
#[derive(Debug, Clone)]
pub struct SplatType<'tree> {
    pub span: ::treesitter_types::Span,
    pub children: Identifier<'tree>,
}
impl<'tree> ::treesitter_types::FromNode<'tree> for SplatType<'tree> {
    #[allow(clippy::match_single_binding, clippy::suspicious_else_formatting)]
    fn from_node(
        node: ::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        debug_assert_eq!(node.kind(), "splat_type");
        Ok(Self {
            span: ::treesitter_types::Span::from(node),
            children: {
                #[allow(clippy::suspicious_else_formatting)]
                let non_field_children = {
                    let mut cursor = node.walk();
                    let mut result = ::std::vec::Vec::new();
                    if cursor.goto_first_child() {
                        loop {
                            if cursor.field_name().is_none()
                                && cursor.node().is_named()
                                && !cursor.node().is_extra()
                            {
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
                    fallback_child.ok_or_else(|| {
                        ::treesitter_types::ParseError::missing_field("children", node)
                    })?
                };
                <Identifier as ::treesitter_types::FromNode>::from_node(child, src)?
            },
        })
    }
}
impl ::treesitter_types::Spanned for SplatType<'_> {
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
pub struct StringContent<'tree> {
    pub span: ::treesitter_types::Span,
    pub children: ::std::vec::Vec<StringContentChildren<'tree>>,
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
                    items.push(
                        <StringContentChildren as ::treesitter_types::FromNode>::from_node(
                            child, src,
                        )?,
                    );
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
#[derive(Debug, Clone)]
pub struct Subscript<'tree> {
    pub span: ::treesitter_types::Span,
    pub subscript: ::std::vec::Vec<SubscriptSubscript<'tree>>,
    pub value: PrimaryExpression<'tree>,
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
            subscript: {
                let mut cursor = node.walk();
                let mut items = ::std::vec::Vec::new();
                for child in node.children_by_field_name("subscript", &mut cursor) {
                    items.push(
                        <SubscriptSubscript as ::treesitter_types::FromNode>::from_node(
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
                <PrimaryExpression as ::treesitter_types::FromNode>::from_node(child, src)?
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
pub struct TuplePattern<'tree> {
    pub span: ::treesitter_types::Span,
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
            children: {
                #[allow(clippy::suspicious_else_formatting)]
                let non_field_children = {
                    let mut cursor = node.walk();
                    let mut result = ::std::vec::Vec::new();
                    if cursor.goto_first_child() {
                        loop {
                            if cursor.field_name().is_none()
                                && cursor.node().is_named()
                                && !cursor.node().is_extra()
                            {
                                result.push(cursor.node());
                            }
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
pub struct Type<'tree> {
    pub span: ::treesitter_types::Span,
    pub children: TypeChildren<'tree>,
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
                                        <TypeChildren as ::treesitter_types::FromNode>::from_node(
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
                <TypeChildren as ::treesitter_types::FromNode>::from_node(child, src)?
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
pub struct TypeAliasStatement<'tree> {
    pub span: ::treesitter_types::Span,
    pub left: Type<'tree>,
    pub right: Type<'tree>,
}
impl<'tree> ::treesitter_types::FromNode<'tree> for TypeAliasStatement<'tree> {
    #[allow(clippy::match_single_binding, clippy::suspicious_else_formatting)]
    fn from_node(
        node: ::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        debug_assert_eq!(node.kind(), "type_alias_statement");
        Ok(Self {
            span: ::treesitter_types::Span::from(node),
            left: {
                let child = node
                    .child_by_field_name("left")
                    .ok_or_else(|| ::treesitter_types::ParseError::missing_field("left", node))?;
                <Type as ::treesitter_types::FromNode>::from_node(child, src)?
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
impl ::treesitter_types::Spanned for TypeAliasStatement<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        self.span
    }
}
#[derive(Debug, Clone)]
pub struct TypeParameter<'tree> {
    pub span: ::treesitter_types::Span,
    pub children: ::std::vec::Vec<Type<'tree>>,
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
            children: {
                #[allow(clippy::suspicious_else_formatting)]
                let non_field_children = {
                    let mut cursor = node.walk();
                    let mut result = ::std::vec::Vec::new();
                    if cursor.goto_first_child() {
                        loop {
                            if cursor.field_name().is_none()
                                && cursor.node().is_named()
                                && !cursor.node().is_extra()
                            {
                                result.push(cursor.node());
                            }
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
impl ::treesitter_types::Spanned for TypeParameter<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        self.span
    }
}
#[derive(Debug, Clone)]
pub struct TypedDefaultParameter<'tree> {
    pub span: ::treesitter_types::Span,
    pub name: Identifier<'tree>,
    pub r#type: Type<'tree>,
    pub value: Expression<'tree>,
}
impl<'tree> ::treesitter_types::FromNode<'tree> for TypedDefaultParameter<'tree> {
    #[allow(clippy::match_single_binding, clippy::suspicious_else_formatting)]
    fn from_node(
        node: ::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        debug_assert_eq!(node.kind(), "typed_default_parameter");
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
            value: {
                let child = node
                    .child_by_field_name("value")
                    .ok_or_else(|| ::treesitter_types::ParseError::missing_field("value", node))?;
                <Expression as ::treesitter_types::FromNode>::from_node(child, src)?
            },
        })
    }
}
impl ::treesitter_types::Spanned for TypedDefaultParameter<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        self.span
    }
}
#[derive(Debug, Clone)]
pub struct TypedParameter<'tree> {
    pub span: ::treesitter_types::Span,
    pub r#type: Type<'tree>,
    pub children: TypedParameterChildren<'tree>,
}
impl<'tree> ::treesitter_types::FromNode<'tree> for TypedParameter<'tree> {
    #[allow(clippy::match_single_binding, clippy::suspicious_else_formatting)]
    fn from_node(
        node: ::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        debug_assert_eq!(node.kind(), "typed_parameter");
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
                                        <TypedParameterChildren as ::treesitter_types::FromNode>::from_node(
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
                <TypedParameterChildren as ::treesitter_types::FromNode>::from_node(child, src)?
            },
        })
    }
}
impl ::treesitter_types::Spanned for TypedParameter<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        self.span
    }
}
#[derive(Debug, Clone)]
pub struct UnaryOperator<'tree> {
    pub span: ::treesitter_types::Span,
    pub argument: PrimaryExpression<'tree>,
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
            argument: {
                let child = node.child_by_field_name("argument").ok_or_else(|| {
                    ::treesitter_types::ParseError::missing_field("argument", node)
                })?;
                <PrimaryExpression as ::treesitter_types::FromNode>::from_node(child, src)?
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
#[derive(Debug, Clone)]
pub struct UnionPattern<'tree> {
    pub span: ::treesitter_types::Span,
    pub children: ::std::vec::Vec<UnionPatternChildren<'tree>>,
}
impl<'tree> ::treesitter_types::FromNode<'tree> for UnionPattern<'tree> {
    #[allow(clippy::match_single_binding, clippy::suspicious_else_formatting)]
    fn from_node(
        node: ::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        debug_assert_eq!(node.kind(), "union_pattern");
        Ok(Self {
            span: ::treesitter_types::Span::from(node),
            children: {
                #[allow(clippy::suspicious_else_formatting)]
                let non_field_children = {
                    let mut cursor = node.walk();
                    let mut result = ::std::vec::Vec::new();
                    if cursor.goto_first_child() {
                        loop {
                            if cursor.field_name().is_none()
                                && cursor.node().is_named()
                                && !cursor.node().is_extra()
                            {
                                result.push(cursor.node());
                            }
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
                        <UnionPatternChildren as ::treesitter_types::FromNode>::from_node(
                            child, src,
                        )?,
                    );
                }
                items
            },
        })
    }
}
impl ::treesitter_types::Spanned for UnionPattern<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        self.span
    }
}
#[derive(Debug, Clone)]
pub struct UnionType<'tree> {
    pub span: ::treesitter_types::Span,
    pub children: ::std::vec::Vec<Type<'tree>>,
}
impl<'tree> ::treesitter_types::FromNode<'tree> for UnionType<'tree> {
    #[allow(clippy::match_single_binding, clippy::suspicious_else_formatting)]
    fn from_node(
        node: ::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        debug_assert_eq!(node.kind(), "union_type");
        Ok(Self {
            span: ::treesitter_types::Span::from(node),
            children: {
                #[allow(clippy::suspicious_else_formatting)]
                let non_field_children = {
                    let mut cursor = node.walk();
                    let mut result = ::std::vec::Vec::new();
                    if cursor.goto_first_child() {
                        loop {
                            if cursor.field_name().is_none()
                                && cursor.node().is_named()
                                && !cursor.node().is_extra()
                            {
                                result.push(cursor.node());
                            }
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
impl ::treesitter_types::Spanned for UnionType<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        self.span
    }
}
#[derive(Debug, Clone)]
pub struct WhileStatement<'tree> {
    pub span: ::treesitter_types::Span,
    pub alternative: ::core::option::Option<ElseClause<'tree>>,
    pub body: Block<'tree>,
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
            alternative: match node.child_by_field_name("alternative") {
                Some(child) => Some(<ElseClause as ::treesitter_types::FromNode>::from_node(
                    child, src,
                )?),
                None => None,
            },
            body: {
                let child = node
                    .child_by_field_name("body")
                    .ok_or_else(|| ::treesitter_types::ParseError::missing_field("body", node))?;
                <Block as ::treesitter_types::FromNode>::from_node(child, src)?
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
pub struct WildcardImport<'tree> {
    pub span: ::treesitter_types::Span,
    text: &'tree str,
}
impl<'tree> ::treesitter_types::FromNode<'tree> for WildcardImport<'tree> {
    fn from_node(
        node: ::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        debug_assert_eq!(node.kind(), "wildcard_import");
        Ok(Self {
            span: ::treesitter_types::Span::from(node),
            text: node.utf8_text(src)?,
        })
    }
}
impl<'tree> ::treesitter_types::LeafNode<'tree> for WildcardImport<'tree> {
    fn text(&self) -> &'tree str {
        self.text
    }
}
impl ::treesitter_types::Spanned for WildcardImport<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        self.span
    }
}
#[derive(Debug, Clone)]
pub struct WithClause<'tree> {
    pub span: ::treesitter_types::Span,
    pub children: ::std::vec::Vec<WithItem<'tree>>,
}
impl<'tree> ::treesitter_types::FromNode<'tree> for WithClause<'tree> {
    #[allow(clippy::match_single_binding, clippy::suspicious_else_formatting)]
    fn from_node(
        node: ::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        debug_assert_eq!(node.kind(), "with_clause");
        Ok(Self {
            span: ::treesitter_types::Span::from(node),
            children: {
                #[allow(clippy::suspicious_else_formatting)]
                let non_field_children = {
                    let mut cursor = node.walk();
                    let mut result = ::std::vec::Vec::new();
                    if cursor.goto_first_child() {
                        loop {
                            if cursor.field_name().is_none()
                                && cursor.node().is_named()
                                && !cursor.node().is_extra()
                            {
                                result.push(cursor.node());
                            }
                            if !cursor.goto_next_sibling() {
                                break;
                            }
                        }
                    }
                    result
                };
                let mut items = ::std::vec::Vec::new();
                for child in non_field_children {
                    items.push(<WithItem as ::treesitter_types::FromNode>::from_node(
                        child, src,
                    )?);
                }
                items
            },
        })
    }
}
impl ::treesitter_types::Spanned for WithClause<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        self.span
    }
}
#[derive(Debug, Clone)]
pub struct WithItem<'tree> {
    pub span: ::treesitter_types::Span,
    pub value: Expression<'tree>,
}
impl<'tree> ::treesitter_types::FromNode<'tree> for WithItem<'tree> {
    #[allow(clippy::match_single_binding, clippy::suspicious_else_formatting)]
    fn from_node(
        node: ::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        debug_assert_eq!(node.kind(), "with_item");
        Ok(Self {
            span: ::treesitter_types::Span::from(node),
            value: {
                let child = node
                    .child_by_field_name("value")
                    .ok_or_else(|| ::treesitter_types::ParseError::missing_field("value", node))?;
                <Expression as ::treesitter_types::FromNode>::from_node(child, src)?
            },
        })
    }
}
impl ::treesitter_types::Spanned for WithItem<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        self.span
    }
}
#[derive(Debug, Clone)]
pub struct WithStatement<'tree> {
    pub span: ::treesitter_types::Span,
    pub body: Block<'tree>,
    pub children: WithClause<'tree>,
}
impl<'tree> ::treesitter_types::FromNode<'tree> for WithStatement<'tree> {
    #[allow(clippy::match_single_binding, clippy::suspicious_else_formatting)]
    fn from_node(
        node: ::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        debug_assert_eq!(node.kind(), "with_statement");
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
                                        <WithClause as ::treesitter_types::FromNode>::from_node(
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
                <WithClause as ::treesitter_types::FromNode>::from_node(child, src)?
            },
        })
    }
}
impl ::treesitter_types::Spanned for WithStatement<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        self.span
    }
}
#[derive(Debug, Clone)]
pub struct Yield<'tree> {
    pub span: ::treesitter_types::Span,
    pub children: ::core::option::Option<YieldChildren<'tree>>,
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
                    Some(&child) => Some(
                        <YieldChildren as ::treesitter_types::FromNode>::from_node(child, src)?,
                    ),
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
pub struct Ellipsis<'tree> {
    pub span: ::treesitter_types::Span,
    text: &'tree str,
}
impl<'tree> ::treesitter_types::FromNode<'tree> for Ellipsis<'tree> {
    fn from_node(
        node: ::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        debug_assert_eq!(node.kind(), "ellipsis");
        Ok(Self {
            span: ::treesitter_types::Span::from(node),
            text: node.utf8_text(src)?,
        })
    }
}
impl<'tree> ::treesitter_types::LeafNode<'tree> for Ellipsis<'tree> {
    fn text(&self) -> &'tree str {
        self.text
    }
}
impl ::treesitter_types::Spanned for Ellipsis<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        self.span
    }
}
#[derive(Debug, Clone)]
pub struct EscapeInterpolation<'tree> {
    pub span: ::treesitter_types::Span,
    text: &'tree str,
}
impl<'tree> ::treesitter_types::FromNode<'tree> for EscapeInterpolation<'tree> {
    fn from_node(
        node: ::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        debug_assert_eq!(node.kind(), "escape_interpolation");
        Ok(Self {
            span: ::treesitter_types::Span::from(node),
            text: node.utf8_text(src)?,
        })
    }
}
impl<'tree> ::treesitter_types::LeafNode<'tree> for EscapeInterpolation<'tree> {
    fn text(&self) -> &'tree str {
        self.text
    }
}
impl ::treesitter_types::Spanned for EscapeInterpolation<'_> {
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
#[derive(Debug, Clone)]
pub struct LineContinuation<'tree> {
    pub span: ::treesitter_types::Span,
    text: &'tree str,
}
impl<'tree> ::treesitter_types::FromNode<'tree> for LineContinuation<'tree> {
    fn from_node(
        node: ::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        debug_assert_eq!(node.kind(), "line_continuation");
        Ok(Self {
            span: ::treesitter_types::Span::from(node),
            text: node.utf8_text(src)?,
        })
    }
}
impl<'tree> ::treesitter_types::LeafNode<'tree> for LineContinuation<'tree> {
    fn text(&self) -> &'tree str {
        self.text
    }
}
impl ::treesitter_types::Spanned for LineContinuation<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        self.span
    }
}
#[derive(Debug, Clone)]
pub struct None<'tree> {
    pub span: ::treesitter_types::Span,
    text: &'tree str,
}
impl<'tree> ::treesitter_types::FromNode<'tree> for None<'tree> {
    fn from_node(
        node: ::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        debug_assert_eq!(node.kind(), "none");
        Ok(Self {
            span: ::treesitter_types::Span::from(node),
            text: node.utf8_text(src)?,
        })
    }
}
impl<'tree> ::treesitter_types::LeafNode<'tree> for None<'tree> {
    fn text(&self) -> &'tree str {
        self.text
    }
}
impl ::treesitter_types::Spanned for None<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        self.span
    }
}
#[derive(Debug, Clone)]
pub struct StringEnd<'tree> {
    pub span: ::treesitter_types::Span,
    text: &'tree str,
}
impl<'tree> ::treesitter_types::FromNode<'tree> for StringEnd<'tree> {
    fn from_node(
        node: ::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        debug_assert_eq!(node.kind(), "string_end");
        Ok(Self {
            span: ::treesitter_types::Span::from(node),
            text: node.utf8_text(src)?,
        })
    }
}
impl<'tree> ::treesitter_types::LeafNode<'tree> for StringEnd<'tree> {
    fn text(&self) -> &'tree str {
        self.text
    }
}
impl ::treesitter_types::Spanned for StringEnd<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        self.span
    }
}
#[derive(Debug, Clone)]
pub struct StringStart<'tree> {
    pub span: ::treesitter_types::Span,
    text: &'tree str,
}
impl<'tree> ::treesitter_types::FromNode<'tree> for StringStart<'tree> {
    fn from_node(
        node: ::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        debug_assert_eq!(node.kind(), "string_start");
        Ok(Self {
            span: ::treesitter_types::Span::from(node),
            text: node.utf8_text(src)?,
        })
    }
}
impl<'tree> ::treesitter_types::LeafNode<'tree> for StringStart<'tree> {
    fn text(&self) -> &'tree str {
        self.text
    }
}
impl ::treesitter_types::Spanned for StringStart<'_> {
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
pub struct TypeConversion<'tree> {
    pub span: ::treesitter_types::Span,
    text: &'tree str,
}
impl<'tree> ::treesitter_types::FromNode<'tree> for TypeConversion<'tree> {
    fn from_node(
        node: ::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        debug_assert_eq!(node.kind(), "type_conversion");
        Ok(Self {
            span: ::treesitter_types::Span::from(node),
            text: node.utf8_text(src)?,
        })
    }
}
impl<'tree> ::treesitter_types::LeafNode<'tree> for TypeConversion<'tree> {
    fn text(&self) -> &'tree str {
        self.text
    }
}
impl ::treesitter_types::Spanned for TypeConversion<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        self.span
    }
}
#[derive(Debug, Clone)]
pub struct AsPatternTarget<'tree> {
    pub span: ::treesitter_types::Span,
    text: &'tree str,
}
impl<'tree> ::treesitter_types::FromNode<'tree> for AsPatternTarget<'tree> {
    fn from_node(
        node: ::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        debug_assert_eq!(node.kind(), "as_pattern_target");
        Ok(Self {
            span: ::treesitter_types::Span::from(node),
            text: node.utf8_text(src)?,
        })
    }
}
impl<'tree> ::treesitter_types::LeafNode<'tree> for AsPatternTarget<'tree> {
    fn text(&self) -> &'tree str {
        self.text
    }
}
impl ::treesitter_types::Spanned for AsPatternTarget<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        self.span
    }
}
#[derive(Debug, Clone)]
pub enum ArgumentListChildren<'tree> {
    DictionarySplat(::std::boxed::Box<DictionarySplat<'tree>>),
    Expression(::std::boxed::Box<Expression<'tree>>),
    KeywordArgument(::std::boxed::Box<KeywordArgument<'tree>>),
    ListSplat(::std::boxed::Box<ListSplat<'tree>>),
    ParenthesizedExpression(::std::boxed::Box<ParenthesizedExpression<'tree>>),
}
impl<'tree> ::treesitter_types::FromNode<'tree> for ArgumentListChildren<'tree> {
    #[allow(clippy::collapsible_else_if)]
    fn from_node(
        node: ::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        match node.kind() {
            "dictionary_splat" => Ok(Self::DictionarySplat(::std::boxed::Box::new(
                <DictionarySplat as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "keyword_argument" => Ok(Self::KeywordArgument(::std::boxed::Box::new(
                <KeywordArgument as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "list_splat" => Ok(Self::ListSplat(::std::boxed::Box::new(
                <ListSplat as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "parenthesized_expression" => {
                Ok(Self::ParenthesizedExpression(::std::boxed::Box::new(
                    <ParenthesizedExpression as ::treesitter_types::FromNode>::from_node(
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
impl ::treesitter_types::Spanned for ArgumentListChildren<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        match self {
            Self::DictionarySplat(inner) => inner.span(),
            Self::Expression(inner) => inner.span(),
            Self::KeywordArgument(inner) => inner.span(),
            Self::ListSplat(inner) => inner.span(),
            Self::ParenthesizedExpression(inner) => inner.span(),
        }
    }
}
#[derive(Debug, Clone)]
pub enum AsPatternChildren<'tree> {
    CasePattern(::std::boxed::Box<CasePattern<'tree>>),
    Expression(::std::boxed::Box<Expression<'tree>>),
    Identifier(::std::boxed::Box<Identifier<'tree>>),
}
impl<'tree> ::treesitter_types::FromNode<'tree> for AsPatternChildren<'tree> {
    #[allow(clippy::collapsible_else_if)]
    fn from_node(
        node: ::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        match node.kind() {
            "case_pattern" => Ok(Self::CasePattern(::std::boxed::Box::new(
                <CasePattern as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
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
impl ::treesitter_types::Spanned for AsPatternChildren<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        match self {
            Self::CasePattern(inner) => inner.span(),
            Self::Expression(inner) => inner.span(),
            Self::Identifier(inner) => inner.span(),
        }
    }
}
#[derive(Debug, Clone)]
pub enum AssignmentLeft<'tree> {
    Pattern(::std::boxed::Box<Pattern<'tree>>),
    PatternList(::std::boxed::Box<PatternList<'tree>>),
}
impl<'tree> ::treesitter_types::FromNode<'tree> for AssignmentLeft<'tree> {
    #[allow(clippy::collapsible_else_if)]
    fn from_node(
        node: ::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        match node.kind() {
            "pattern_list" => Ok(Self::PatternList(::std::boxed::Box::new(
                <PatternList as ::treesitter_types::FromNode>::from_node(node, src)?,
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
impl ::treesitter_types::Spanned for AssignmentLeft<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        match self {
            Self::Pattern(inner) => inner.span(),
            Self::PatternList(inner) => inner.span(),
        }
    }
}
#[derive(Debug, Clone)]
pub enum AssignmentRight<'tree> {
    Assignment(::std::boxed::Box<Assignment<'tree>>),
    AugmentedAssignment(::std::boxed::Box<AugmentedAssignment<'tree>>),
    Expression(::std::boxed::Box<Expression<'tree>>),
    ExpressionList(::std::boxed::Box<ExpressionList<'tree>>),
    PatternList(::std::boxed::Box<PatternList<'tree>>),
    Yield(::std::boxed::Box<Yield<'tree>>),
}
impl<'tree> ::treesitter_types::FromNode<'tree> for AssignmentRight<'tree> {
    #[allow(clippy::collapsible_else_if)]
    fn from_node(
        node: ::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        match node.kind() {
            "assignment" => Ok(Self::Assignment(::std::boxed::Box::new(
                <Assignment as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "augmented_assignment" => Ok(Self::AugmentedAssignment(::std::boxed::Box::new(
                <AugmentedAssignment as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "expression_list" => Ok(Self::ExpressionList(::std::boxed::Box::new(
                <ExpressionList as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "pattern_list" => Ok(Self::PatternList(::std::boxed::Box::new(
                <PatternList as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "yield" => Ok(Self::Yield(::std::boxed::Box::new(
                <Yield as ::treesitter_types::FromNode>::from_node(node, src)?,
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
impl ::treesitter_types::Spanned for AssignmentRight<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        match self {
            Self::Assignment(inner) => inner.span(),
            Self::AugmentedAssignment(inner) => inner.span(),
            Self::Expression(inner) => inner.span(),
            Self::ExpressionList(inner) => inner.span(),
            Self::PatternList(inner) => inner.span(),
            Self::Yield(inner) => inner.span(),
        }
    }
}
#[derive(Debug, Clone)]
pub enum AugmentedAssignmentLeft<'tree> {
    Pattern(::std::boxed::Box<Pattern<'tree>>),
    PatternList(::std::boxed::Box<PatternList<'tree>>),
}
impl<'tree> ::treesitter_types::FromNode<'tree> for AugmentedAssignmentLeft<'tree> {
    #[allow(clippy::collapsible_else_if)]
    fn from_node(
        node: ::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        match node.kind() {
            "pattern_list" => Ok(Self::PatternList(::std::boxed::Box::new(
                <PatternList as ::treesitter_types::FromNode>::from_node(node, src)?,
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
impl ::treesitter_types::Spanned for AugmentedAssignmentLeft<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        match self {
            Self::Pattern(inner) => inner.span(),
            Self::PatternList(inner) => inner.span(),
        }
    }
}
#[derive(Debug, Clone)]
pub enum AugmentedAssignmentOperator {
    PercentEq(::treesitter_types::Span),
    AmpEq(::treesitter_types::Span),
    StarStarEq(::treesitter_types::Span),
    StarEq(::treesitter_types::Span),
    PlusEq(::treesitter_types::Span),
    MinusEq(::treesitter_types::Span),
    SlashSlashEq(::treesitter_types::Span),
    SlashEq(::treesitter_types::Span),
    ShlEq(::treesitter_types::Span),
    ShrEq(::treesitter_types::Span),
    AtEq(::treesitter_types::Span),
    CaretEq(::treesitter_types::Span),
    PipeEq(::treesitter_types::Span),
}
impl<'tree> ::treesitter_types::FromNode<'tree> for AugmentedAssignmentOperator {
    #[allow(clippy::collapsible_else_if)]
    fn from_node(
        node: ::tree_sitter::Node<'tree>,
        _src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        match node.kind() {
            "%=" => Ok(Self::PercentEq(::treesitter_types::Span::from(node))),
            "&=" => Ok(Self::AmpEq(::treesitter_types::Span::from(node))),
            "**=" => Ok(Self::StarStarEq(::treesitter_types::Span::from(node))),
            "*=" => Ok(Self::StarEq(::treesitter_types::Span::from(node))),
            "+=" => Ok(Self::PlusEq(::treesitter_types::Span::from(node))),
            "-=" => Ok(Self::MinusEq(::treesitter_types::Span::from(node))),
            "//=" => Ok(Self::SlashSlashEq(::treesitter_types::Span::from(node))),
            "/=" => Ok(Self::SlashEq(::treesitter_types::Span::from(node))),
            "<<=" => Ok(Self::ShlEq(::treesitter_types::Span::from(node))),
            ">>=" => Ok(Self::ShrEq(::treesitter_types::Span::from(node))),
            "@=" => Ok(Self::AtEq(::treesitter_types::Span::from(node))),
            "^=" => Ok(Self::CaretEq(::treesitter_types::Span::from(node))),
            "|=" => Ok(Self::PipeEq(::treesitter_types::Span::from(node))),
            other => Err(::treesitter_types::ParseError::unexpected_kind(other, node)),
        }
    }
}
impl ::treesitter_types::Spanned for AugmentedAssignmentOperator {
    fn span(&self) -> ::treesitter_types::Span {
        match self {
            Self::PercentEq(span) => *span,
            Self::AmpEq(span) => *span,
            Self::StarStarEq(span) => *span,
            Self::StarEq(span) => *span,
            Self::PlusEq(span) => *span,
            Self::MinusEq(span) => *span,
            Self::SlashSlashEq(span) => *span,
            Self::SlashEq(span) => *span,
            Self::ShlEq(span) => *span,
            Self::ShrEq(span) => *span,
            Self::AtEq(span) => *span,
            Self::CaretEq(span) => *span,
            Self::PipeEq(span) => *span,
        }
    }
}
#[derive(Debug, Clone)]
pub enum AugmentedAssignmentRight<'tree> {
    Assignment(::std::boxed::Box<Assignment<'tree>>),
    AugmentedAssignment(::std::boxed::Box<AugmentedAssignment<'tree>>),
    Expression(::std::boxed::Box<Expression<'tree>>),
    ExpressionList(::std::boxed::Box<ExpressionList<'tree>>),
    PatternList(::std::boxed::Box<PatternList<'tree>>),
    Yield(::std::boxed::Box<Yield<'tree>>),
}
impl<'tree> ::treesitter_types::FromNode<'tree> for AugmentedAssignmentRight<'tree> {
    #[allow(clippy::collapsible_else_if)]
    fn from_node(
        node: ::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        match node.kind() {
            "assignment" => Ok(Self::Assignment(::std::boxed::Box::new(
                <Assignment as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "augmented_assignment" => Ok(Self::AugmentedAssignment(::std::boxed::Box::new(
                <AugmentedAssignment as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "expression_list" => Ok(Self::ExpressionList(::std::boxed::Box::new(
                <ExpressionList as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "pattern_list" => Ok(Self::PatternList(::std::boxed::Box::new(
                <PatternList as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "yield" => Ok(Self::Yield(::std::boxed::Box::new(
                <Yield as ::treesitter_types::FromNode>::from_node(node, src)?,
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
impl ::treesitter_types::Spanned for AugmentedAssignmentRight<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        match self {
            Self::Assignment(inner) => inner.span(),
            Self::AugmentedAssignment(inner) => inner.span(),
            Self::Expression(inner) => inner.span(),
            Self::ExpressionList(inner) => inner.span(),
            Self::PatternList(inner) => inner.span(),
            Self::Yield(inner) => inner.span(),
        }
    }
}
#[derive(Debug, Clone)]
pub enum BinaryOperatorOperator {
    Percent(::treesitter_types::Span),
    Amp(::treesitter_types::Span),
    Star(::treesitter_types::Span),
    StarStar(::treesitter_types::Span),
    Plus(::treesitter_types::Span),
    Minus(::treesitter_types::Span),
    Slash(::treesitter_types::Span),
    SlashSlash(::treesitter_types::Span),
    Shl(::treesitter_types::Span),
    Shr(::treesitter_types::Span),
    At(::treesitter_types::Span),
    Caret(::treesitter_types::Span),
    Pipe(::treesitter_types::Span),
}
impl<'tree> ::treesitter_types::FromNode<'tree> for BinaryOperatorOperator {
    #[allow(clippy::collapsible_else_if)]
    fn from_node(
        node: ::tree_sitter::Node<'tree>,
        _src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        match node.kind() {
            "%" => Ok(Self::Percent(::treesitter_types::Span::from(node))),
            "&" => Ok(Self::Amp(::treesitter_types::Span::from(node))),
            "*" => Ok(Self::Star(::treesitter_types::Span::from(node))),
            "**" => Ok(Self::StarStar(::treesitter_types::Span::from(node))),
            "+" => Ok(Self::Plus(::treesitter_types::Span::from(node))),
            "-" => Ok(Self::Minus(::treesitter_types::Span::from(node))),
            "/" => Ok(Self::Slash(::treesitter_types::Span::from(node))),
            "//" => Ok(Self::SlashSlash(::treesitter_types::Span::from(node))),
            "<<" => Ok(Self::Shl(::treesitter_types::Span::from(node))),
            ">>" => Ok(Self::Shr(::treesitter_types::Span::from(node))),
            "@" => Ok(Self::At(::treesitter_types::Span::from(node))),
            "^" => Ok(Self::Caret(::treesitter_types::Span::from(node))),
            "|" => Ok(Self::Pipe(::treesitter_types::Span::from(node))),
            other => Err(::treesitter_types::ParseError::unexpected_kind(other, node)),
        }
    }
}
impl ::treesitter_types::Spanned for BinaryOperatorOperator {
    fn span(&self) -> ::treesitter_types::Span {
        match self {
            Self::Percent(span) => *span,
            Self::Amp(span) => *span,
            Self::Star(span) => *span,
            Self::StarStar(span) => *span,
            Self::Plus(span) => *span,
            Self::Minus(span) => *span,
            Self::Slash(span) => *span,
            Self::SlashSlash(span) => *span,
            Self::Shl(span) => *span,
            Self::Shr(span) => *span,
            Self::At(span) => *span,
            Self::Caret(span) => *span,
            Self::Pipe(span) => *span,
        }
    }
}
#[derive(Debug, Clone)]
pub enum BlockChildren<'tree> {
    CompoundStatement(::std::boxed::Box<CompoundStatement<'tree>>),
    SimpleStatement(::std::boxed::Box<SimpleStatement<'tree>>),
}
impl<'tree> ::treesitter_types::FromNode<'tree> for BlockChildren<'tree> {
    #[allow(clippy::collapsible_else_if)]
    fn from_node(
        node: ::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        if let Ok(v) = <CompoundStatement as ::treesitter_types::FromNode>::from_node(node, src) {
            Ok(Self::CompoundStatement(::std::boxed::Box::new(v)))
        } else {
            if let Ok(v) = <SimpleStatement as ::treesitter_types::FromNode>::from_node(node, src) {
                Ok(Self::SimpleStatement(::std::boxed::Box::new(v)))
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
            Self::CompoundStatement(inner) => inner.span(),
            Self::SimpleStatement(inner) => inner.span(),
        }
    }
}
#[derive(Debug, Clone)]
pub enum BooleanOperatorOperator {
    And(::treesitter_types::Span),
    Or(::treesitter_types::Span),
}
impl<'tree> ::treesitter_types::FromNode<'tree> for BooleanOperatorOperator {
    #[allow(clippy::collapsible_else_if)]
    fn from_node(
        node: ::tree_sitter::Node<'tree>,
        _src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        match node.kind() {
            "and" => Ok(Self::And(::treesitter_types::Span::from(node))),
            "or" => Ok(Self::Or(::treesitter_types::Span::from(node))),
            other => Err(::treesitter_types::ParseError::unexpected_kind(other, node)),
        }
    }
}
impl ::treesitter_types::Spanned for BooleanOperatorOperator {
    fn span(&self) -> ::treesitter_types::Span {
        match self {
            Self::And(span) => *span,
            Self::Or(span) => *span,
        }
    }
}
#[derive(Debug, Clone)]
pub enum CallArguments<'tree> {
    ArgumentList(::std::boxed::Box<ArgumentList<'tree>>),
    GeneratorExpression(::std::boxed::Box<GeneratorExpression<'tree>>),
}
impl<'tree> ::treesitter_types::FromNode<'tree> for CallArguments<'tree> {
    #[allow(clippy::collapsible_else_if)]
    fn from_node(
        node: ::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        match node.kind() {
            "argument_list" => Ok(Self::ArgumentList(::std::boxed::Box::new(
                <ArgumentList as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "generator_expression" => Ok(Self::GeneratorExpression(::std::boxed::Box::new(
                <GeneratorExpression as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            other => Err(::treesitter_types::ParseError::unexpected_kind(other, node)),
        }
    }
}
impl ::treesitter_types::Spanned for CallArguments<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        match self {
            Self::ArgumentList(inner) => inner.span(),
            Self::GeneratorExpression(inner) => inner.span(),
        }
    }
}
#[derive(Debug, Clone)]
pub enum CasePatternChildren<'tree> {
    AsPattern(::std::boxed::Box<AsPattern<'tree>>),
    ClassPattern(::std::boxed::Box<ClassPattern<'tree>>),
    ComplexPattern(::std::boxed::Box<ComplexPattern<'tree>>),
    ConcatenatedString(::std::boxed::Box<ConcatenatedString<'tree>>),
    DictPattern(::std::boxed::Box<DictPattern<'tree>>),
    DottedName(::std::boxed::Box<DottedName<'tree>>),
    False(::std::boxed::Box<False<'tree>>),
    Float(::std::boxed::Box<Float<'tree>>),
    Integer(::std::boxed::Box<Integer<'tree>>),
    KeywordPattern(::std::boxed::Box<KeywordPattern<'tree>>),
    ListPattern(::std::boxed::Box<ListPattern<'tree>>),
    None(::std::boxed::Box<None<'tree>>),
    SplatPattern(::std::boxed::Box<SplatPattern<'tree>>),
    String(::std::boxed::Box<String<'tree>>),
    True(::std::boxed::Box<True<'tree>>),
    TuplePattern(::std::boxed::Box<TuplePattern<'tree>>),
    UnionPattern(::std::boxed::Box<UnionPattern<'tree>>),
}
impl<'tree> ::treesitter_types::FromNode<'tree> for CasePatternChildren<'tree> {
    #[allow(clippy::collapsible_else_if)]
    fn from_node(
        node: ::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        match node.kind() {
            "as_pattern" => Ok(Self::AsPattern(::std::boxed::Box::new(
                <AsPattern as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "class_pattern" => Ok(Self::ClassPattern(::std::boxed::Box::new(
                <ClassPattern as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "complex_pattern" => Ok(Self::ComplexPattern(::std::boxed::Box::new(
                <ComplexPattern as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "concatenated_string" => Ok(Self::ConcatenatedString(::std::boxed::Box::new(
                <ConcatenatedString as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "dict_pattern" => Ok(Self::DictPattern(::std::boxed::Box::new(
                <DictPattern as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "dotted_name" => Ok(Self::DottedName(::std::boxed::Box::new(
                <DottedName as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "false" => Ok(Self::False(::std::boxed::Box::new(
                <False as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "float" => Ok(Self::Float(::std::boxed::Box::new(
                <Float as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "integer" => Ok(Self::Integer(::std::boxed::Box::new(
                <Integer as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "keyword_pattern" => Ok(Self::KeywordPattern(::std::boxed::Box::new(
                <KeywordPattern as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "list_pattern" => Ok(Self::ListPattern(::std::boxed::Box::new(
                <ListPattern as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "none" => Ok(Self::None(::std::boxed::Box::new(
                <None as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "splat_pattern" => Ok(Self::SplatPattern(::std::boxed::Box::new(
                <SplatPattern as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "string" => Ok(Self::String(::std::boxed::Box::new(
                <String as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "true" => Ok(Self::True(::std::boxed::Box::new(
                <True as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "tuple_pattern" => Ok(Self::TuplePattern(::std::boxed::Box::new(
                <TuplePattern as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "union_pattern" => Ok(Self::UnionPattern(::std::boxed::Box::new(
                <UnionPattern as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            other => Err(::treesitter_types::ParseError::unexpected_kind(other, node)),
        }
    }
}
impl ::treesitter_types::Spanned for CasePatternChildren<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        match self {
            Self::AsPattern(inner) => inner.span(),
            Self::ClassPattern(inner) => inner.span(),
            Self::ComplexPattern(inner) => inner.span(),
            Self::ConcatenatedString(inner) => inner.span(),
            Self::DictPattern(inner) => inner.span(),
            Self::DottedName(inner) => inner.span(),
            Self::False(inner) => inner.span(),
            Self::Float(inner) => inner.span(),
            Self::Integer(inner) => inner.span(),
            Self::KeywordPattern(inner) => inner.span(),
            Self::ListPattern(inner) => inner.span(),
            Self::None(inner) => inner.span(),
            Self::SplatPattern(inner) => inner.span(),
            Self::String(inner) => inner.span(),
            Self::True(inner) => inner.span(),
            Self::TuplePattern(inner) => inner.span(),
            Self::UnionPattern(inner) => inner.span(),
        }
    }
}
#[derive(Debug, Clone)]
pub enum ClassPatternChildren<'tree> {
    CasePattern(::std::boxed::Box<CasePattern<'tree>>),
    DottedName(::std::boxed::Box<DottedName<'tree>>),
}
impl<'tree> ::treesitter_types::FromNode<'tree> for ClassPatternChildren<'tree> {
    #[allow(clippy::collapsible_else_if)]
    fn from_node(
        node: ::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        match node.kind() {
            "case_pattern" => Ok(Self::CasePattern(::std::boxed::Box::new(
                <CasePattern as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "dotted_name" => Ok(Self::DottedName(::std::boxed::Box::new(
                <DottedName as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            other => Err(::treesitter_types::ParseError::unexpected_kind(other, node)),
        }
    }
}
impl ::treesitter_types::Spanned for ClassPatternChildren<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        match self {
            Self::CasePattern(inner) => inner.span(),
            Self::DottedName(inner) => inner.span(),
        }
    }
}
#[derive(Debug, Clone)]
pub enum ComparisonOperatorOperators {
    NotEq(::treesitter_types::Span),
    Lt(::treesitter_types::Span),
    LtEq(::treesitter_types::Span),
    LtGt(::treesitter_types::Span),
    EqEq(::treesitter_types::Span),
    Gt(::treesitter_types::Span),
    GtEq(::treesitter_types::Span),
    In(::treesitter_types::Span),
    Is(::treesitter_types::Span),
    ISXNOT(::treesitter_types::Span),
    NOTXIN(::treesitter_types::Span),
}
impl<'tree> ::treesitter_types::FromNode<'tree> for ComparisonOperatorOperators {
    #[allow(clippy::collapsible_else_if)]
    fn from_node(
        node: ::tree_sitter::Node<'tree>,
        _src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        match node.kind() {
            "!=" => Ok(Self::NotEq(::treesitter_types::Span::from(node))),
            "<" => Ok(Self::Lt(::treesitter_types::Span::from(node))),
            "<=" => Ok(Self::LtEq(::treesitter_types::Span::from(node))),
            "<>" => Ok(Self::LtGt(::treesitter_types::Span::from(node))),
            "==" => Ok(Self::EqEq(::treesitter_types::Span::from(node))),
            ">" => Ok(Self::Gt(::treesitter_types::Span::from(node))),
            ">=" => Ok(Self::GtEq(::treesitter_types::Span::from(node))),
            "in" => Ok(Self::In(::treesitter_types::Span::from(node))),
            "is" => Ok(Self::Is(::treesitter_types::Span::from(node))),
            "is not" => Ok(Self::ISXNOT(::treesitter_types::Span::from(node))),
            "not in" => Ok(Self::NOTXIN(::treesitter_types::Span::from(node))),
            other => Err(::treesitter_types::ParseError::unexpected_kind(other, node)),
        }
    }
}
impl ::treesitter_types::Spanned for ComparisonOperatorOperators {
    fn span(&self) -> ::treesitter_types::Span {
        match self {
            Self::NotEq(span) => *span,
            Self::Lt(span) => *span,
            Self::LtEq(span) => *span,
            Self::LtGt(span) => *span,
            Self::EqEq(span) => *span,
            Self::Gt(span) => *span,
            Self::GtEq(span) => *span,
            Self::In(span) => *span,
            Self::Is(span) => *span,
            Self::ISXNOT(span) => *span,
            Self::NOTXIN(span) => *span,
        }
    }
}
#[derive(Debug, Clone)]
pub enum ComplexPatternChildren<'tree> {
    Float(::std::boxed::Box<Float<'tree>>),
    Integer(::std::boxed::Box<Integer<'tree>>),
}
impl<'tree> ::treesitter_types::FromNode<'tree> for ComplexPatternChildren<'tree> {
    #[allow(clippy::collapsible_else_if)]
    fn from_node(
        node: ::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        match node.kind() {
            "float" => Ok(Self::Float(::std::boxed::Box::new(
                <Float as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "integer" => Ok(Self::Integer(::std::boxed::Box::new(
                <Integer as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            other => Err(::treesitter_types::ParseError::unexpected_kind(other, node)),
        }
    }
}
impl ::treesitter_types::Spanned for ComplexPatternChildren<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        match self {
            Self::Float(inner) => inner.span(),
            Self::Integer(inner) => inner.span(),
        }
    }
}
#[derive(Debug, Clone)]
pub enum DecoratedDefinitionDefinition<'tree> {
    ClassDefinition(::std::boxed::Box<ClassDefinition<'tree>>),
    FunctionDefinition(::std::boxed::Box<FunctionDefinition<'tree>>),
}
impl<'tree> ::treesitter_types::FromNode<'tree> for DecoratedDefinitionDefinition<'tree> {
    #[allow(clippy::collapsible_else_if)]
    fn from_node(
        node: ::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        match node.kind() {
            "class_definition" => Ok(Self::ClassDefinition(::std::boxed::Box::new(
                <ClassDefinition as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "function_definition" => Ok(Self::FunctionDefinition(::std::boxed::Box::new(
                <FunctionDefinition as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            other => Err(::treesitter_types::ParseError::unexpected_kind(other, node)),
        }
    }
}
impl ::treesitter_types::Spanned for DecoratedDefinitionDefinition<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        match self {
            Self::ClassDefinition(inner) => inner.span(),
            Self::FunctionDefinition(inner) => inner.span(),
        }
    }
}
#[derive(Debug, Clone)]
pub enum DefaultParameterName<'tree> {
    Identifier(::std::boxed::Box<Identifier<'tree>>),
    TuplePattern(::std::boxed::Box<TuplePattern<'tree>>),
}
impl<'tree> ::treesitter_types::FromNode<'tree> for DefaultParameterName<'tree> {
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
            other => Err(::treesitter_types::ParseError::unexpected_kind(other, node)),
        }
    }
}
impl ::treesitter_types::Spanned for DefaultParameterName<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        match self {
            Self::Identifier(inner) => inner.span(),
            Self::TuplePattern(inner) => inner.span(),
        }
    }
}
#[derive(Debug, Clone)]
pub enum DeleteStatementChildren<'tree> {
    Expression(::std::boxed::Box<Expression<'tree>>),
    ExpressionList(::std::boxed::Box<ExpressionList<'tree>>),
}
impl<'tree> ::treesitter_types::FromNode<'tree> for DeleteStatementChildren<'tree> {
    #[allow(clippy::collapsible_else_if)]
    fn from_node(
        node: ::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        match node.kind() {
            "expression_list" => Ok(Self::ExpressionList(::std::boxed::Box::new(
                <ExpressionList as ::treesitter_types::FromNode>::from_node(node, src)?,
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
impl ::treesitter_types::Spanned for DeleteStatementChildren<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        match self {
            Self::Expression(inner) => inner.span(),
            Self::ExpressionList(inner) => inner.span(),
        }
    }
}
#[derive(Debug, Clone)]
pub enum DictPatternKey<'tree> {
    Minus(::treesitter_types::Span),
    Blank(::treesitter_types::Span),
    ClassPattern(::std::boxed::Box<ClassPattern<'tree>>),
    ComplexPattern(::std::boxed::Box<ComplexPattern<'tree>>),
    ConcatenatedString(::std::boxed::Box<ConcatenatedString<'tree>>),
    DictPattern(::std::boxed::Box<DictPattern<'tree>>),
    DottedName(::std::boxed::Box<DottedName<'tree>>),
    False(::std::boxed::Box<False<'tree>>),
    Float(::std::boxed::Box<Float<'tree>>),
    Integer(::std::boxed::Box<Integer<'tree>>),
    ListPattern(::std::boxed::Box<ListPattern<'tree>>),
    None(::std::boxed::Box<None<'tree>>),
    SplatPattern(::std::boxed::Box<SplatPattern<'tree>>),
    String(::std::boxed::Box<String<'tree>>),
    True(::std::boxed::Box<True<'tree>>),
    TuplePattern(::std::boxed::Box<TuplePattern<'tree>>),
    UnionPattern(::std::boxed::Box<UnionPattern<'tree>>),
}
impl<'tree> ::treesitter_types::FromNode<'tree> for DictPatternKey<'tree> {
    #[allow(clippy::collapsible_else_if)]
    fn from_node(
        node: ::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        match node.kind() {
            "-" => Ok(Self::Minus(::treesitter_types::Span::from(node))),
            "_" => Ok(Self::Blank(::treesitter_types::Span::from(node))),
            "class_pattern" => Ok(Self::ClassPattern(::std::boxed::Box::new(
                <ClassPattern as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "complex_pattern" => Ok(Self::ComplexPattern(::std::boxed::Box::new(
                <ComplexPattern as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "concatenated_string" => Ok(Self::ConcatenatedString(::std::boxed::Box::new(
                <ConcatenatedString as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "dict_pattern" => Ok(Self::DictPattern(::std::boxed::Box::new(
                <DictPattern as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "dotted_name" => Ok(Self::DottedName(::std::boxed::Box::new(
                <DottedName as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "false" => Ok(Self::False(::std::boxed::Box::new(
                <False as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "float" => Ok(Self::Float(::std::boxed::Box::new(
                <Float as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "integer" => Ok(Self::Integer(::std::boxed::Box::new(
                <Integer as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "list_pattern" => Ok(Self::ListPattern(::std::boxed::Box::new(
                <ListPattern as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "none" => Ok(Self::None(::std::boxed::Box::new(
                <None as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "splat_pattern" => Ok(Self::SplatPattern(::std::boxed::Box::new(
                <SplatPattern as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "string" => Ok(Self::String(::std::boxed::Box::new(
                <String as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "true" => Ok(Self::True(::std::boxed::Box::new(
                <True as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "tuple_pattern" => Ok(Self::TuplePattern(::std::boxed::Box::new(
                <TuplePattern as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "union_pattern" => Ok(Self::UnionPattern(::std::boxed::Box::new(
                <UnionPattern as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            other => Err(::treesitter_types::ParseError::unexpected_kind(other, node)),
        }
    }
}
impl ::treesitter_types::Spanned for DictPatternKey<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        match self {
            Self::Minus(span) => *span,
            Self::Blank(span) => *span,
            Self::ClassPattern(inner) => inner.span(),
            Self::ComplexPattern(inner) => inner.span(),
            Self::ConcatenatedString(inner) => inner.span(),
            Self::DictPattern(inner) => inner.span(),
            Self::DottedName(inner) => inner.span(),
            Self::False(inner) => inner.span(),
            Self::Float(inner) => inner.span(),
            Self::Integer(inner) => inner.span(),
            Self::ListPattern(inner) => inner.span(),
            Self::None(inner) => inner.span(),
            Self::SplatPattern(inner) => inner.span(),
            Self::String(inner) => inner.span(),
            Self::True(inner) => inner.span(),
            Self::TuplePattern(inner) => inner.span(),
            Self::UnionPattern(inner) => inner.span(),
        }
    }
}
#[derive(Debug, Clone)]
pub enum DictionaryChildren<'tree> {
    DictionarySplat(::std::boxed::Box<DictionarySplat<'tree>>),
    Pair(::std::boxed::Box<Pair<'tree>>),
}
impl<'tree> ::treesitter_types::FromNode<'tree> for DictionaryChildren<'tree> {
    #[allow(clippy::collapsible_else_if)]
    fn from_node(
        node: ::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        match node.kind() {
            "dictionary_splat" => Ok(Self::DictionarySplat(::std::boxed::Box::new(
                <DictionarySplat as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "pair" => Ok(Self::Pair(::std::boxed::Box::new(
                <Pair as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            other => Err(::treesitter_types::ParseError::unexpected_kind(other, node)),
        }
    }
}
impl ::treesitter_types::Spanned for DictionaryChildren<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        match self {
            Self::DictionarySplat(inner) => inner.span(),
            Self::Pair(inner) => inner.span(),
        }
    }
}
#[derive(Debug, Clone)]
pub enum DictionaryComprehensionChildren<'tree> {
    ForInClause(::std::boxed::Box<ForInClause<'tree>>),
    IfClause(::std::boxed::Box<IfClause<'tree>>),
}
impl<'tree> ::treesitter_types::FromNode<'tree> for DictionaryComprehensionChildren<'tree> {
    #[allow(clippy::collapsible_else_if)]
    fn from_node(
        node: ::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        match node.kind() {
            "for_in_clause" => Ok(Self::ForInClause(::std::boxed::Box::new(
                <ForInClause as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "if_clause" => Ok(Self::IfClause(::std::boxed::Box::new(
                <IfClause as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            other => Err(::treesitter_types::ParseError::unexpected_kind(other, node)),
        }
    }
}
impl ::treesitter_types::Spanned for DictionaryComprehensionChildren<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        match self {
            Self::ForInClause(inner) => inner.span(),
            Self::IfClause(inner) => inner.span(),
        }
    }
}
#[derive(Debug, Clone)]
pub enum DictionarySplatPatternChildren<'tree> {
    Attribute(::std::boxed::Box<Attribute<'tree>>),
    Identifier(::std::boxed::Box<Identifier<'tree>>),
    Subscript(::std::boxed::Box<Subscript<'tree>>),
}
impl<'tree> ::treesitter_types::FromNode<'tree> for DictionarySplatPatternChildren<'tree> {
    #[allow(clippy::collapsible_else_if)]
    fn from_node(
        node: ::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        match node.kind() {
            "attribute" => Ok(Self::Attribute(::std::boxed::Box::new(
                <Attribute as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "identifier" => Ok(Self::Identifier(::std::boxed::Box::new(
                <Identifier as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "subscript" => Ok(Self::Subscript(::std::boxed::Box::new(
                <Subscript as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            other => Err(::treesitter_types::ParseError::unexpected_kind(other, node)),
        }
    }
}
impl ::treesitter_types::Spanned for DictionarySplatPatternChildren<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        match self {
            Self::Attribute(inner) => inner.span(),
            Self::Identifier(inner) => inner.span(),
            Self::Subscript(inner) => inner.span(),
        }
    }
}
#[derive(Debug, Clone)]
pub enum ExecStatementCode<'tree> {
    Identifier(::std::boxed::Box<Identifier<'tree>>),
    String(::std::boxed::Box<String<'tree>>),
}
impl<'tree> ::treesitter_types::FromNode<'tree> for ExecStatementCode<'tree> {
    #[allow(clippy::collapsible_else_if)]
    fn from_node(
        node: ::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        match node.kind() {
            "identifier" => Ok(Self::Identifier(::std::boxed::Box::new(
                <Identifier as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "string" => Ok(Self::String(::std::boxed::Box::new(
                <String as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            other => Err(::treesitter_types::ParseError::unexpected_kind(other, node)),
        }
    }
}
impl ::treesitter_types::Spanned for ExecStatementCode<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        match self {
            Self::Identifier(inner) => inner.span(),
            Self::String(inner) => inner.span(),
        }
    }
}
#[derive(Debug, Clone)]
pub enum ExpressionStatementChildren<'tree> {
    Assignment(::std::boxed::Box<Assignment<'tree>>),
    AugmentedAssignment(::std::boxed::Box<AugmentedAssignment<'tree>>),
    Expression(::std::boxed::Box<Expression<'tree>>),
    Yield(::std::boxed::Box<Yield<'tree>>),
}
impl<'tree> ::treesitter_types::FromNode<'tree> for ExpressionStatementChildren<'tree> {
    #[allow(clippy::collapsible_else_if)]
    fn from_node(
        node: ::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        match node.kind() {
            "assignment" => Ok(Self::Assignment(::std::boxed::Box::new(
                <Assignment as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "augmented_assignment" => Ok(Self::AugmentedAssignment(::std::boxed::Box::new(
                <AugmentedAssignment as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "yield" => Ok(Self::Yield(::std::boxed::Box::new(
                <Yield as ::treesitter_types::FromNode>::from_node(node, src)?,
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
            Self::Assignment(inner) => inner.span(),
            Self::AugmentedAssignment(inner) => inner.span(),
            Self::Expression(inner) => inner.span(),
            Self::Yield(inner) => inner.span(),
        }
    }
}
#[derive(Debug, Clone)]
pub enum ForInClauseLeft<'tree> {
    Pattern(::std::boxed::Box<Pattern<'tree>>),
    PatternList(::std::boxed::Box<PatternList<'tree>>),
}
impl<'tree> ::treesitter_types::FromNode<'tree> for ForInClauseLeft<'tree> {
    #[allow(clippy::collapsible_else_if)]
    fn from_node(
        node: ::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        match node.kind() {
            "pattern_list" => Ok(Self::PatternList(::std::boxed::Box::new(
                <PatternList as ::treesitter_types::FromNode>::from_node(node, src)?,
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
impl ::treesitter_types::Spanned for ForInClauseLeft<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        match self {
            Self::Pattern(inner) => inner.span(),
            Self::PatternList(inner) => inner.span(),
        }
    }
}
#[derive(Debug, Clone)]
pub enum ForInClauseRight<'tree> {
    Comma(::treesitter_types::Span),
    Expression(::std::boxed::Box<Expression<'tree>>),
}
impl<'tree> ::treesitter_types::FromNode<'tree> for ForInClauseRight<'tree> {
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
impl ::treesitter_types::Spanned for ForInClauseRight<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        match self {
            Self::Comma(span) => *span,
            Self::Expression(inner) => inner.span(),
        }
    }
}
#[derive(Debug, Clone)]
pub enum ForStatementLeft<'tree> {
    Pattern(::std::boxed::Box<Pattern<'tree>>),
    PatternList(::std::boxed::Box<PatternList<'tree>>),
}
impl<'tree> ::treesitter_types::FromNode<'tree> for ForStatementLeft<'tree> {
    #[allow(clippy::collapsible_else_if)]
    fn from_node(
        node: ::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        match node.kind() {
            "pattern_list" => Ok(Self::PatternList(::std::boxed::Box::new(
                <PatternList as ::treesitter_types::FromNode>::from_node(node, src)?,
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
impl ::treesitter_types::Spanned for ForStatementLeft<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        match self {
            Self::Pattern(inner) => inner.span(),
            Self::PatternList(inner) => inner.span(),
        }
    }
}
#[derive(Debug, Clone)]
pub enum ForStatementRight<'tree> {
    Expression(::std::boxed::Box<Expression<'tree>>),
    ExpressionList(::std::boxed::Box<ExpressionList<'tree>>),
}
impl<'tree> ::treesitter_types::FromNode<'tree> for ForStatementRight<'tree> {
    #[allow(clippy::collapsible_else_if)]
    fn from_node(
        node: ::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        match node.kind() {
            "expression_list" => Ok(Self::ExpressionList(::std::boxed::Box::new(
                <ExpressionList as ::treesitter_types::FromNode>::from_node(node, src)?,
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
impl ::treesitter_types::Spanned for ForStatementRight<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        match self {
            Self::Expression(inner) => inner.span(),
            Self::ExpressionList(inner) => inner.span(),
        }
    }
}
#[derive(Debug, Clone)]
pub enum FormatExpressionExpression<'tree> {
    Expression(::std::boxed::Box<Expression<'tree>>),
    ExpressionList(::std::boxed::Box<ExpressionList<'tree>>),
    PatternList(::std::boxed::Box<PatternList<'tree>>),
    Yield(::std::boxed::Box<Yield<'tree>>),
}
impl<'tree> ::treesitter_types::FromNode<'tree> for FormatExpressionExpression<'tree> {
    #[allow(clippy::collapsible_else_if)]
    fn from_node(
        node: ::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        match node.kind() {
            "expression_list" => Ok(Self::ExpressionList(::std::boxed::Box::new(
                <ExpressionList as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "pattern_list" => Ok(Self::PatternList(::std::boxed::Box::new(
                <PatternList as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "yield" => Ok(Self::Yield(::std::boxed::Box::new(
                <Yield as ::treesitter_types::FromNode>::from_node(node, src)?,
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
impl ::treesitter_types::Spanned for FormatExpressionExpression<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        match self {
            Self::Expression(inner) => inner.span(),
            Self::ExpressionList(inner) => inner.span(),
            Self::PatternList(inner) => inner.span(),
            Self::Yield(inner) => inner.span(),
        }
    }
}
#[derive(Debug, Clone)]
pub enum FutureImportStatementName<'tree> {
    AliasedImport(::std::boxed::Box<AliasedImport<'tree>>),
    DottedName(::std::boxed::Box<DottedName<'tree>>),
}
impl<'tree> ::treesitter_types::FromNode<'tree> for FutureImportStatementName<'tree> {
    #[allow(clippy::collapsible_else_if)]
    fn from_node(
        node: ::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        match node.kind() {
            "aliased_import" => Ok(Self::AliasedImport(::std::boxed::Box::new(
                <AliasedImport as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "dotted_name" => Ok(Self::DottedName(::std::boxed::Box::new(
                <DottedName as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            other => Err(::treesitter_types::ParseError::unexpected_kind(other, node)),
        }
    }
}
impl ::treesitter_types::Spanned for FutureImportStatementName<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        match self {
            Self::AliasedImport(inner) => inner.span(),
            Self::DottedName(inner) => inner.span(),
        }
    }
}
#[derive(Debug, Clone)]
pub enum GeneratorExpressionChildren<'tree> {
    ForInClause(::std::boxed::Box<ForInClause<'tree>>),
    IfClause(::std::boxed::Box<IfClause<'tree>>),
}
impl<'tree> ::treesitter_types::FromNode<'tree> for GeneratorExpressionChildren<'tree> {
    #[allow(clippy::collapsible_else_if)]
    fn from_node(
        node: ::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        match node.kind() {
            "for_in_clause" => Ok(Self::ForInClause(::std::boxed::Box::new(
                <ForInClause as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "if_clause" => Ok(Self::IfClause(::std::boxed::Box::new(
                <IfClause as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            other => Err(::treesitter_types::ParseError::unexpected_kind(other, node)),
        }
    }
}
impl ::treesitter_types::Spanned for GeneratorExpressionChildren<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        match self {
            Self::ForInClause(inner) => inner.span(),
            Self::IfClause(inner) => inner.span(),
        }
    }
}
#[derive(Debug, Clone)]
pub enum GenericTypeChildren<'tree> {
    Identifier(::std::boxed::Box<Identifier<'tree>>),
    TypeParameter(::std::boxed::Box<TypeParameter<'tree>>),
}
impl<'tree> ::treesitter_types::FromNode<'tree> for GenericTypeChildren<'tree> {
    #[allow(clippy::collapsible_else_if)]
    fn from_node(
        node: ::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        match node.kind() {
            "identifier" => Ok(Self::Identifier(::std::boxed::Box::new(
                <Identifier as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "type_parameter" => Ok(Self::TypeParameter(::std::boxed::Box::new(
                <TypeParameter as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            other => Err(::treesitter_types::ParseError::unexpected_kind(other, node)),
        }
    }
}
impl ::treesitter_types::Spanned for GenericTypeChildren<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        match self {
            Self::Identifier(inner) => inner.span(),
            Self::TypeParameter(inner) => inner.span(),
        }
    }
}
#[derive(Debug, Clone)]
pub enum IfStatementAlternative<'tree> {
    ElifClause(::std::boxed::Box<ElifClause<'tree>>),
    ElseClause(::std::boxed::Box<ElseClause<'tree>>),
}
impl<'tree> ::treesitter_types::FromNode<'tree> for IfStatementAlternative<'tree> {
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
            other => Err(::treesitter_types::ParseError::unexpected_kind(other, node)),
        }
    }
}
impl ::treesitter_types::Spanned for IfStatementAlternative<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        match self {
            Self::ElifClause(inner) => inner.span(),
            Self::ElseClause(inner) => inner.span(),
        }
    }
}
#[derive(Debug, Clone)]
pub enum ImportFromStatementModuleName<'tree> {
    DottedName(::std::boxed::Box<DottedName<'tree>>),
    RelativeImport(::std::boxed::Box<RelativeImport<'tree>>),
}
impl<'tree> ::treesitter_types::FromNode<'tree> for ImportFromStatementModuleName<'tree> {
    #[allow(clippy::collapsible_else_if)]
    fn from_node(
        node: ::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        match node.kind() {
            "dotted_name" => Ok(Self::DottedName(::std::boxed::Box::new(
                <DottedName as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "relative_import" => Ok(Self::RelativeImport(::std::boxed::Box::new(
                <RelativeImport as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            other => Err(::treesitter_types::ParseError::unexpected_kind(other, node)),
        }
    }
}
impl ::treesitter_types::Spanned for ImportFromStatementModuleName<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        match self {
            Self::DottedName(inner) => inner.span(),
            Self::RelativeImport(inner) => inner.span(),
        }
    }
}
#[derive(Debug, Clone)]
pub enum ImportFromStatementName<'tree> {
    AliasedImport(::std::boxed::Box<AliasedImport<'tree>>),
    DottedName(::std::boxed::Box<DottedName<'tree>>),
}
impl<'tree> ::treesitter_types::FromNode<'tree> for ImportFromStatementName<'tree> {
    #[allow(clippy::collapsible_else_if)]
    fn from_node(
        node: ::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        match node.kind() {
            "aliased_import" => Ok(Self::AliasedImport(::std::boxed::Box::new(
                <AliasedImport as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "dotted_name" => Ok(Self::DottedName(::std::boxed::Box::new(
                <DottedName as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            other => Err(::treesitter_types::ParseError::unexpected_kind(other, node)),
        }
    }
}
impl ::treesitter_types::Spanned for ImportFromStatementName<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        match self {
            Self::AliasedImport(inner) => inner.span(),
            Self::DottedName(inner) => inner.span(),
        }
    }
}
#[derive(Debug, Clone)]
pub enum ImportStatementName<'tree> {
    AliasedImport(::std::boxed::Box<AliasedImport<'tree>>),
    DottedName(::std::boxed::Box<DottedName<'tree>>),
}
impl<'tree> ::treesitter_types::FromNode<'tree> for ImportStatementName<'tree> {
    #[allow(clippy::collapsible_else_if)]
    fn from_node(
        node: ::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        match node.kind() {
            "aliased_import" => Ok(Self::AliasedImport(::std::boxed::Box::new(
                <AliasedImport as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "dotted_name" => Ok(Self::DottedName(::std::boxed::Box::new(
                <DottedName as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            other => Err(::treesitter_types::ParseError::unexpected_kind(other, node)),
        }
    }
}
impl ::treesitter_types::Spanned for ImportStatementName<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        match self {
            Self::AliasedImport(inner) => inner.span(),
            Self::DottedName(inner) => inner.span(),
        }
    }
}
#[derive(Debug, Clone)]
pub enum InterpolationExpression<'tree> {
    Expression(::std::boxed::Box<Expression<'tree>>),
    ExpressionList(::std::boxed::Box<ExpressionList<'tree>>),
    PatternList(::std::boxed::Box<PatternList<'tree>>),
    Yield(::std::boxed::Box<Yield<'tree>>),
}
impl<'tree> ::treesitter_types::FromNode<'tree> for InterpolationExpression<'tree> {
    #[allow(clippy::collapsible_else_if)]
    fn from_node(
        node: ::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        match node.kind() {
            "expression_list" => Ok(Self::ExpressionList(::std::boxed::Box::new(
                <ExpressionList as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "pattern_list" => Ok(Self::PatternList(::std::boxed::Box::new(
                <PatternList as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "yield" => Ok(Self::Yield(::std::boxed::Box::new(
                <Yield as ::treesitter_types::FromNode>::from_node(node, src)?,
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
impl ::treesitter_types::Spanned for InterpolationExpression<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        match self {
            Self::Expression(inner) => inner.span(),
            Self::ExpressionList(inner) => inner.span(),
            Self::PatternList(inner) => inner.span(),
            Self::Yield(inner) => inner.span(),
        }
    }
}
#[derive(Debug, Clone)]
pub enum KeywordPatternChildren<'tree> {
    ClassPattern(::std::boxed::Box<ClassPattern<'tree>>),
    ComplexPattern(::std::boxed::Box<ComplexPattern<'tree>>),
    ConcatenatedString(::std::boxed::Box<ConcatenatedString<'tree>>),
    DictPattern(::std::boxed::Box<DictPattern<'tree>>),
    DottedName(::std::boxed::Box<DottedName<'tree>>),
    False(::std::boxed::Box<False<'tree>>),
    Float(::std::boxed::Box<Float<'tree>>),
    Identifier(::std::boxed::Box<Identifier<'tree>>),
    Integer(::std::boxed::Box<Integer<'tree>>),
    ListPattern(::std::boxed::Box<ListPattern<'tree>>),
    None(::std::boxed::Box<None<'tree>>),
    SplatPattern(::std::boxed::Box<SplatPattern<'tree>>),
    String(::std::boxed::Box<String<'tree>>),
    True(::std::boxed::Box<True<'tree>>),
    TuplePattern(::std::boxed::Box<TuplePattern<'tree>>),
    UnionPattern(::std::boxed::Box<UnionPattern<'tree>>),
}
impl<'tree> ::treesitter_types::FromNode<'tree> for KeywordPatternChildren<'tree> {
    #[allow(clippy::collapsible_else_if)]
    fn from_node(
        node: ::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        match node.kind() {
            "class_pattern" => Ok(Self::ClassPattern(::std::boxed::Box::new(
                <ClassPattern as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "complex_pattern" => Ok(Self::ComplexPattern(::std::boxed::Box::new(
                <ComplexPattern as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "concatenated_string" => Ok(Self::ConcatenatedString(::std::boxed::Box::new(
                <ConcatenatedString as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "dict_pattern" => Ok(Self::DictPattern(::std::boxed::Box::new(
                <DictPattern as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "dotted_name" => Ok(Self::DottedName(::std::boxed::Box::new(
                <DottedName as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "false" => Ok(Self::False(::std::boxed::Box::new(
                <False as ::treesitter_types::FromNode>::from_node(node, src)?,
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
            "list_pattern" => Ok(Self::ListPattern(::std::boxed::Box::new(
                <ListPattern as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "none" => Ok(Self::None(::std::boxed::Box::new(
                <None as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "splat_pattern" => Ok(Self::SplatPattern(::std::boxed::Box::new(
                <SplatPattern as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "string" => Ok(Self::String(::std::boxed::Box::new(
                <String as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "true" => Ok(Self::True(::std::boxed::Box::new(
                <True as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "tuple_pattern" => Ok(Self::TuplePattern(::std::boxed::Box::new(
                <TuplePattern as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "union_pattern" => Ok(Self::UnionPattern(::std::boxed::Box::new(
                <UnionPattern as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            other => Err(::treesitter_types::ParseError::unexpected_kind(other, node)),
        }
    }
}
impl ::treesitter_types::Spanned for KeywordPatternChildren<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        match self {
            Self::ClassPattern(inner) => inner.span(),
            Self::ComplexPattern(inner) => inner.span(),
            Self::ConcatenatedString(inner) => inner.span(),
            Self::DictPattern(inner) => inner.span(),
            Self::DottedName(inner) => inner.span(),
            Self::False(inner) => inner.span(),
            Self::Float(inner) => inner.span(),
            Self::Identifier(inner) => inner.span(),
            Self::Integer(inner) => inner.span(),
            Self::ListPattern(inner) => inner.span(),
            Self::None(inner) => inner.span(),
            Self::SplatPattern(inner) => inner.span(),
            Self::String(inner) => inner.span(),
            Self::True(inner) => inner.span(),
            Self::TuplePattern(inner) => inner.span(),
            Self::UnionPattern(inner) => inner.span(),
        }
    }
}
#[derive(Debug, Clone)]
pub enum ListChildren<'tree> {
    Expression(::std::boxed::Box<Expression<'tree>>),
    ListSplat(::std::boxed::Box<ListSplat<'tree>>),
    ParenthesizedListSplat(::std::boxed::Box<ParenthesizedListSplat<'tree>>),
    Yield(::std::boxed::Box<Yield<'tree>>),
}
impl<'tree> ::treesitter_types::FromNode<'tree> for ListChildren<'tree> {
    #[allow(clippy::collapsible_else_if)]
    fn from_node(
        node: ::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        match node.kind() {
            "list_splat" => Ok(Self::ListSplat(::std::boxed::Box::new(
                <ListSplat as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "parenthesized_list_splat" => Ok(Self::ParenthesizedListSplat(::std::boxed::Box::new(
                <ParenthesizedListSplat as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "yield" => Ok(Self::Yield(::std::boxed::Box::new(
                <Yield as ::treesitter_types::FromNode>::from_node(node, src)?,
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
impl ::treesitter_types::Spanned for ListChildren<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        match self {
            Self::Expression(inner) => inner.span(),
            Self::ListSplat(inner) => inner.span(),
            Self::ParenthesizedListSplat(inner) => inner.span(),
            Self::Yield(inner) => inner.span(),
        }
    }
}
#[derive(Debug, Clone)]
pub enum ListComprehensionChildren<'tree> {
    ForInClause(::std::boxed::Box<ForInClause<'tree>>),
    IfClause(::std::boxed::Box<IfClause<'tree>>),
}
impl<'tree> ::treesitter_types::FromNode<'tree> for ListComprehensionChildren<'tree> {
    #[allow(clippy::collapsible_else_if)]
    fn from_node(
        node: ::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        match node.kind() {
            "for_in_clause" => Ok(Self::ForInClause(::std::boxed::Box::new(
                <ForInClause as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "if_clause" => Ok(Self::IfClause(::std::boxed::Box::new(
                <IfClause as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            other => Err(::treesitter_types::ParseError::unexpected_kind(other, node)),
        }
    }
}
impl ::treesitter_types::Spanned for ListComprehensionChildren<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        match self {
            Self::ForInClause(inner) => inner.span(),
            Self::IfClause(inner) => inner.span(),
        }
    }
}
#[derive(Debug, Clone)]
pub enum ListPatternChildren<'tree> {
    CasePattern(::std::boxed::Box<CasePattern<'tree>>),
    Pattern(::std::boxed::Box<Pattern<'tree>>),
}
impl<'tree> ::treesitter_types::FromNode<'tree> for ListPatternChildren<'tree> {
    #[allow(clippy::collapsible_else_if)]
    fn from_node(
        node: ::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        match node.kind() {
            "case_pattern" => Ok(Self::CasePattern(::std::boxed::Box::new(
                <CasePattern as ::treesitter_types::FromNode>::from_node(node, src)?,
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
impl ::treesitter_types::Spanned for ListPatternChildren<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        match self {
            Self::CasePattern(inner) => inner.span(),
            Self::Pattern(inner) => inner.span(),
        }
    }
}
#[derive(Debug, Clone)]
pub enum ListSplatChildren<'tree> {
    Attribute(::std::boxed::Box<Attribute<'tree>>),
    Expression(::std::boxed::Box<Expression<'tree>>),
    Identifier(::std::boxed::Box<Identifier<'tree>>),
    Subscript(::std::boxed::Box<Subscript<'tree>>),
}
impl<'tree> ::treesitter_types::FromNode<'tree> for ListSplatChildren<'tree> {
    #[allow(clippy::collapsible_else_if)]
    fn from_node(
        node: ::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        match node.kind() {
            "attribute" => Ok(Self::Attribute(::std::boxed::Box::new(
                <Attribute as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "identifier" => Ok(Self::Identifier(::std::boxed::Box::new(
                <Identifier as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "subscript" => Ok(Self::Subscript(::std::boxed::Box::new(
                <Subscript as ::treesitter_types::FromNode>::from_node(node, src)?,
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
impl ::treesitter_types::Spanned for ListSplatChildren<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        match self {
            Self::Attribute(inner) => inner.span(),
            Self::Expression(inner) => inner.span(),
            Self::Identifier(inner) => inner.span(),
            Self::Subscript(inner) => inner.span(),
        }
    }
}
#[derive(Debug, Clone)]
pub enum ListSplatPatternChildren<'tree> {
    Attribute(::std::boxed::Box<Attribute<'tree>>),
    Identifier(::std::boxed::Box<Identifier<'tree>>),
    Subscript(::std::boxed::Box<Subscript<'tree>>),
}
impl<'tree> ::treesitter_types::FromNode<'tree> for ListSplatPatternChildren<'tree> {
    #[allow(clippy::collapsible_else_if)]
    fn from_node(
        node: ::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        match node.kind() {
            "attribute" => Ok(Self::Attribute(::std::boxed::Box::new(
                <Attribute as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "identifier" => Ok(Self::Identifier(::std::boxed::Box::new(
                <Identifier as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "subscript" => Ok(Self::Subscript(::std::boxed::Box::new(
                <Subscript as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            other => Err(::treesitter_types::ParseError::unexpected_kind(other, node)),
        }
    }
}
impl ::treesitter_types::Spanned for ListSplatPatternChildren<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        match self {
            Self::Attribute(inner) => inner.span(),
            Self::Identifier(inner) => inner.span(),
            Self::Subscript(inner) => inner.span(),
        }
    }
}
#[derive(Debug, Clone)]
pub enum MemberTypeChildren<'tree> {
    Identifier(::std::boxed::Box<Identifier<'tree>>),
    Type(::std::boxed::Box<Type<'tree>>),
}
impl<'tree> ::treesitter_types::FromNode<'tree> for MemberTypeChildren<'tree> {
    #[allow(clippy::collapsible_else_if)]
    fn from_node(
        node: ::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        match node.kind() {
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
impl ::treesitter_types::Spanned for MemberTypeChildren<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        match self {
            Self::Identifier(inner) => inner.span(),
            Self::Type(inner) => inner.span(),
        }
    }
}
#[derive(Debug, Clone)]
pub enum ModuleChildren<'tree> {
    CompoundStatement(::std::boxed::Box<CompoundStatement<'tree>>),
    SimpleStatement(::std::boxed::Box<SimpleStatement<'tree>>),
}
impl<'tree> ::treesitter_types::FromNode<'tree> for ModuleChildren<'tree> {
    #[allow(clippy::collapsible_else_if)]
    fn from_node(
        node: ::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        if let Ok(v) = <CompoundStatement as ::treesitter_types::FromNode>::from_node(node, src) {
            Ok(Self::CompoundStatement(::std::boxed::Box::new(v)))
        } else {
            if let Ok(v) = <SimpleStatement as ::treesitter_types::FromNode>::from_node(node, src) {
                Ok(Self::SimpleStatement(::std::boxed::Box::new(v)))
            } else {
                Err(::treesitter_types::ParseError::unexpected_kind(
                    node.kind(),
                    node,
                ))
            }
        }
    }
}
impl ::treesitter_types::Spanned for ModuleChildren<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        match self {
            Self::CompoundStatement(inner) => inner.span(),
            Self::SimpleStatement(inner) => inner.span(),
        }
    }
}
#[derive(Debug, Clone)]
pub enum ParenthesizedExpressionChildren<'tree> {
    Expression(::std::boxed::Box<Expression<'tree>>),
    ListSplat(::std::boxed::Box<ListSplat<'tree>>),
    ParenthesizedExpression(::std::boxed::Box<ParenthesizedExpression<'tree>>),
    Yield(::std::boxed::Box<Yield<'tree>>),
}
impl<'tree> ::treesitter_types::FromNode<'tree> for ParenthesizedExpressionChildren<'tree> {
    #[allow(clippy::collapsible_else_if)]
    fn from_node(
        node: ::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        match node.kind() {
            "list_splat" => Ok(Self::ListSplat(::std::boxed::Box::new(
                <ListSplat as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "parenthesized_expression" => {
                Ok(Self::ParenthesizedExpression(::std::boxed::Box::new(
                    <ParenthesizedExpression as ::treesitter_types::FromNode>::from_node(
                        node, src,
                    )?,
                )))
            }
            "yield" => Ok(Self::Yield(::std::boxed::Box::new(
                <Yield as ::treesitter_types::FromNode>::from_node(node, src)?,
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
            Self::ListSplat(inner) => inner.span(),
            Self::ParenthesizedExpression(inner) => inner.span(),
            Self::Yield(inner) => inner.span(),
        }
    }
}
#[derive(Debug, Clone)]
pub enum ParenthesizedListSplatChildren<'tree> {
    ListSplat(::std::boxed::Box<ListSplat<'tree>>),
    ParenthesizedExpression(::std::boxed::Box<ParenthesizedExpression<'tree>>),
}
impl<'tree> ::treesitter_types::FromNode<'tree> for ParenthesizedListSplatChildren<'tree> {
    #[allow(clippy::collapsible_else_if)]
    fn from_node(
        node: ::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        match node.kind() {
            "list_splat" => Ok(Self::ListSplat(::std::boxed::Box::new(
                <ListSplat as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "parenthesized_expression" => {
                Ok(Self::ParenthesizedExpression(::std::boxed::Box::new(
                    <ParenthesizedExpression as ::treesitter_types::FromNode>::from_node(
                        node, src,
                    )?,
                )))
            }
            other => Err(::treesitter_types::ParseError::unexpected_kind(other, node)),
        }
    }
}
impl ::treesitter_types::Spanned for ParenthesizedListSplatChildren<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        match self {
            Self::ListSplat(inner) => inner.span(),
            Self::ParenthesizedExpression(inner) => inner.span(),
        }
    }
}
#[derive(Debug, Clone)]
pub enum RaiseStatementChildren<'tree> {
    Expression(::std::boxed::Box<Expression<'tree>>),
    ExpressionList(::std::boxed::Box<ExpressionList<'tree>>),
}
impl<'tree> ::treesitter_types::FromNode<'tree> for RaiseStatementChildren<'tree> {
    #[allow(clippy::collapsible_else_if)]
    fn from_node(
        node: ::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        match node.kind() {
            "expression_list" => Ok(Self::ExpressionList(::std::boxed::Box::new(
                <ExpressionList as ::treesitter_types::FromNode>::from_node(node, src)?,
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
impl ::treesitter_types::Spanned for RaiseStatementChildren<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        match self {
            Self::Expression(inner) => inner.span(),
            Self::ExpressionList(inner) => inner.span(),
        }
    }
}
#[derive(Debug, Clone)]
pub enum RelativeImportChildren<'tree> {
    DottedName(::std::boxed::Box<DottedName<'tree>>),
    ImportPrefix(::std::boxed::Box<ImportPrefix<'tree>>),
}
impl<'tree> ::treesitter_types::FromNode<'tree> for RelativeImportChildren<'tree> {
    #[allow(clippy::collapsible_else_if)]
    fn from_node(
        node: ::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        match node.kind() {
            "dotted_name" => Ok(Self::DottedName(::std::boxed::Box::new(
                <DottedName as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "import_prefix" => Ok(Self::ImportPrefix(::std::boxed::Box::new(
                <ImportPrefix as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            other => Err(::treesitter_types::ParseError::unexpected_kind(other, node)),
        }
    }
}
impl ::treesitter_types::Spanned for RelativeImportChildren<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        match self {
            Self::DottedName(inner) => inner.span(),
            Self::ImportPrefix(inner) => inner.span(),
        }
    }
}
#[derive(Debug, Clone)]
pub enum ReturnStatementChildren<'tree> {
    Expression(::std::boxed::Box<Expression<'tree>>),
    ExpressionList(::std::boxed::Box<ExpressionList<'tree>>),
}
impl<'tree> ::treesitter_types::FromNode<'tree> for ReturnStatementChildren<'tree> {
    #[allow(clippy::collapsible_else_if)]
    fn from_node(
        node: ::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        match node.kind() {
            "expression_list" => Ok(Self::ExpressionList(::std::boxed::Box::new(
                <ExpressionList as ::treesitter_types::FromNode>::from_node(node, src)?,
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
            Self::Expression(inner) => inner.span(),
            Self::ExpressionList(inner) => inner.span(),
        }
    }
}
#[derive(Debug, Clone)]
pub enum SetChildren<'tree> {
    Expression(::std::boxed::Box<Expression<'tree>>),
    ListSplat(::std::boxed::Box<ListSplat<'tree>>),
    ParenthesizedListSplat(::std::boxed::Box<ParenthesizedListSplat<'tree>>),
    Yield(::std::boxed::Box<Yield<'tree>>),
}
impl<'tree> ::treesitter_types::FromNode<'tree> for SetChildren<'tree> {
    #[allow(clippy::collapsible_else_if)]
    fn from_node(
        node: ::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        match node.kind() {
            "list_splat" => Ok(Self::ListSplat(::std::boxed::Box::new(
                <ListSplat as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "parenthesized_list_splat" => Ok(Self::ParenthesizedListSplat(::std::boxed::Box::new(
                <ParenthesizedListSplat as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "yield" => Ok(Self::Yield(::std::boxed::Box::new(
                <Yield as ::treesitter_types::FromNode>::from_node(node, src)?,
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
impl ::treesitter_types::Spanned for SetChildren<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        match self {
            Self::Expression(inner) => inner.span(),
            Self::ListSplat(inner) => inner.span(),
            Self::ParenthesizedListSplat(inner) => inner.span(),
            Self::Yield(inner) => inner.span(),
        }
    }
}
#[derive(Debug, Clone)]
pub enum SetComprehensionChildren<'tree> {
    ForInClause(::std::boxed::Box<ForInClause<'tree>>),
    IfClause(::std::boxed::Box<IfClause<'tree>>),
}
impl<'tree> ::treesitter_types::FromNode<'tree> for SetComprehensionChildren<'tree> {
    #[allow(clippy::collapsible_else_if)]
    fn from_node(
        node: ::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        match node.kind() {
            "for_in_clause" => Ok(Self::ForInClause(::std::boxed::Box::new(
                <ForInClause as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "if_clause" => Ok(Self::IfClause(::std::boxed::Box::new(
                <IfClause as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            other => Err(::treesitter_types::ParseError::unexpected_kind(other, node)),
        }
    }
}
impl ::treesitter_types::Spanned for SetComprehensionChildren<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        match self {
            Self::ForInClause(inner) => inner.span(),
            Self::IfClause(inner) => inner.span(),
        }
    }
}
#[derive(Debug, Clone)]
pub enum StringChildren<'tree> {
    Interpolation(::std::boxed::Box<Interpolation<'tree>>),
    StringContent(::std::boxed::Box<StringContent<'tree>>),
    StringEnd(::std::boxed::Box<StringEnd<'tree>>),
    StringStart(::std::boxed::Box<StringStart<'tree>>),
}
impl<'tree> ::treesitter_types::FromNode<'tree> for StringChildren<'tree> {
    #[allow(clippy::collapsible_else_if)]
    fn from_node(
        node: ::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        match node.kind() {
            "interpolation" => Ok(Self::Interpolation(::std::boxed::Box::new(
                <Interpolation as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "string_content" => Ok(Self::StringContent(::std::boxed::Box::new(
                <StringContent as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "string_end" => Ok(Self::StringEnd(::std::boxed::Box::new(
                <StringEnd as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "string_start" => Ok(Self::StringStart(::std::boxed::Box::new(
                <StringStart as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            other => Err(::treesitter_types::ParseError::unexpected_kind(other, node)),
        }
    }
}
impl ::treesitter_types::Spanned for StringChildren<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        match self {
            Self::Interpolation(inner) => inner.span(),
            Self::StringContent(inner) => inner.span(),
            Self::StringEnd(inner) => inner.span(),
            Self::StringStart(inner) => inner.span(),
        }
    }
}
#[derive(Debug, Clone)]
pub enum StringContentChildren<'tree> {
    EscapeInterpolation(::std::boxed::Box<EscapeInterpolation<'tree>>),
    EscapeSequence(::std::boxed::Box<EscapeSequence<'tree>>),
}
impl<'tree> ::treesitter_types::FromNode<'tree> for StringContentChildren<'tree> {
    #[allow(clippy::collapsible_else_if)]
    fn from_node(
        node: ::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        match node.kind() {
            "escape_interpolation" => Ok(Self::EscapeInterpolation(::std::boxed::Box::new(
                <EscapeInterpolation as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "escape_sequence" => Ok(Self::EscapeSequence(::std::boxed::Box::new(
                <EscapeSequence as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            other => Err(::treesitter_types::ParseError::unexpected_kind(other, node)),
        }
    }
}
impl ::treesitter_types::Spanned for StringContentChildren<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        match self {
            Self::EscapeInterpolation(inner) => inner.span(),
            Self::EscapeSequence(inner) => inner.span(),
        }
    }
}
#[derive(Debug, Clone)]
pub enum SubscriptSubscript<'tree> {
    Expression(::std::boxed::Box<Expression<'tree>>),
    Slice(::std::boxed::Box<Slice<'tree>>),
}
impl<'tree> ::treesitter_types::FromNode<'tree> for SubscriptSubscript<'tree> {
    #[allow(clippy::collapsible_else_if)]
    fn from_node(
        node: ::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        match node.kind() {
            "slice" => Ok(Self::Slice(::std::boxed::Box::new(
                <Slice as ::treesitter_types::FromNode>::from_node(node, src)?,
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
impl ::treesitter_types::Spanned for SubscriptSubscript<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        match self {
            Self::Expression(inner) => inner.span(),
            Self::Slice(inner) => inner.span(),
        }
    }
}
#[derive(Debug, Clone)]
pub enum TryStatementChildren<'tree> {
    ElseClause(::std::boxed::Box<ElseClause<'tree>>),
    ExceptClause(::std::boxed::Box<ExceptClause<'tree>>),
    FinallyClause(::std::boxed::Box<FinallyClause<'tree>>),
}
impl<'tree> ::treesitter_types::FromNode<'tree> for TryStatementChildren<'tree> {
    #[allow(clippy::collapsible_else_if)]
    fn from_node(
        node: ::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        match node.kind() {
            "else_clause" => Ok(Self::ElseClause(::std::boxed::Box::new(
                <ElseClause as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "except_clause" => Ok(Self::ExceptClause(::std::boxed::Box::new(
                <ExceptClause as ::treesitter_types::FromNode>::from_node(node, src)?,
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
            Self::ElseClause(inner) => inner.span(),
            Self::ExceptClause(inner) => inner.span(),
            Self::FinallyClause(inner) => inner.span(),
        }
    }
}
#[derive(Debug, Clone)]
pub enum TupleChildren<'tree> {
    Expression(::std::boxed::Box<Expression<'tree>>),
    ListSplat(::std::boxed::Box<ListSplat<'tree>>),
    ParenthesizedListSplat(::std::boxed::Box<ParenthesizedListSplat<'tree>>),
    Yield(::std::boxed::Box<Yield<'tree>>),
}
impl<'tree> ::treesitter_types::FromNode<'tree> for TupleChildren<'tree> {
    #[allow(clippy::collapsible_else_if)]
    fn from_node(
        node: ::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        match node.kind() {
            "list_splat" => Ok(Self::ListSplat(::std::boxed::Box::new(
                <ListSplat as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "parenthesized_list_splat" => Ok(Self::ParenthesizedListSplat(::std::boxed::Box::new(
                <ParenthesizedListSplat as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "yield" => Ok(Self::Yield(::std::boxed::Box::new(
                <Yield as ::treesitter_types::FromNode>::from_node(node, src)?,
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
impl ::treesitter_types::Spanned for TupleChildren<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        match self {
            Self::Expression(inner) => inner.span(),
            Self::ListSplat(inner) => inner.span(),
            Self::ParenthesizedListSplat(inner) => inner.span(),
            Self::Yield(inner) => inner.span(),
        }
    }
}
#[derive(Debug, Clone)]
pub enum TuplePatternChildren<'tree> {
    CasePattern(::std::boxed::Box<CasePattern<'tree>>),
    Pattern(::std::boxed::Box<Pattern<'tree>>),
}
impl<'tree> ::treesitter_types::FromNode<'tree> for TuplePatternChildren<'tree> {
    #[allow(clippy::collapsible_else_if)]
    fn from_node(
        node: ::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        match node.kind() {
            "case_pattern" => Ok(Self::CasePattern(::std::boxed::Box::new(
                <CasePattern as ::treesitter_types::FromNode>::from_node(node, src)?,
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
impl ::treesitter_types::Spanned for TuplePatternChildren<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        match self {
            Self::CasePattern(inner) => inner.span(),
            Self::Pattern(inner) => inner.span(),
        }
    }
}
#[derive(Debug, Clone)]
pub enum TypeChildren<'tree> {
    ConstrainedType(::std::boxed::Box<ConstrainedType<'tree>>),
    Expression(::std::boxed::Box<Expression<'tree>>),
    GenericType(::std::boxed::Box<GenericType<'tree>>),
    MemberType(::std::boxed::Box<MemberType<'tree>>),
    SplatType(::std::boxed::Box<SplatType<'tree>>),
    UnionType(::std::boxed::Box<UnionType<'tree>>),
}
impl<'tree> ::treesitter_types::FromNode<'tree> for TypeChildren<'tree> {
    #[allow(clippy::collapsible_else_if)]
    fn from_node(
        node: ::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        match node.kind() {
            "constrained_type" => Ok(Self::ConstrainedType(::std::boxed::Box::new(
                <ConstrainedType as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "generic_type" => Ok(Self::GenericType(::std::boxed::Box::new(
                <GenericType as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "member_type" => Ok(Self::MemberType(::std::boxed::Box::new(
                <MemberType as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "splat_type" => Ok(Self::SplatType(::std::boxed::Box::new(
                <SplatType as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "union_type" => Ok(Self::UnionType(::std::boxed::Box::new(
                <UnionType as ::treesitter_types::FromNode>::from_node(node, src)?,
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
impl ::treesitter_types::Spanned for TypeChildren<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        match self {
            Self::ConstrainedType(inner) => inner.span(),
            Self::Expression(inner) => inner.span(),
            Self::GenericType(inner) => inner.span(),
            Self::MemberType(inner) => inner.span(),
            Self::SplatType(inner) => inner.span(),
            Self::UnionType(inner) => inner.span(),
        }
    }
}
#[derive(Debug, Clone)]
pub enum TypedParameterChildren<'tree> {
    DictionarySplatPattern(::std::boxed::Box<DictionarySplatPattern<'tree>>),
    Identifier(::std::boxed::Box<Identifier<'tree>>),
    ListSplatPattern(::std::boxed::Box<ListSplatPattern<'tree>>),
}
impl<'tree> ::treesitter_types::FromNode<'tree> for TypedParameterChildren<'tree> {
    #[allow(clippy::collapsible_else_if)]
    fn from_node(
        node: ::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        match node.kind() {
            "dictionary_splat_pattern" => Ok(Self::DictionarySplatPattern(::std::boxed::Box::new(
                <DictionarySplatPattern as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "identifier" => Ok(Self::Identifier(::std::boxed::Box::new(
                <Identifier as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "list_splat_pattern" => Ok(Self::ListSplatPattern(::std::boxed::Box::new(
                <ListSplatPattern as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            other => Err(::treesitter_types::ParseError::unexpected_kind(other, node)),
        }
    }
}
impl ::treesitter_types::Spanned for TypedParameterChildren<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        match self {
            Self::DictionarySplatPattern(inner) => inner.span(),
            Self::Identifier(inner) => inner.span(),
            Self::ListSplatPattern(inner) => inner.span(),
        }
    }
}
#[derive(Debug, Clone)]
pub enum UnaryOperatorOperator {
    Plus(::treesitter_types::Span),
    Minus(::treesitter_types::Span),
    Tilde(::treesitter_types::Span),
}
impl<'tree> ::treesitter_types::FromNode<'tree> for UnaryOperatorOperator {
    #[allow(clippy::collapsible_else_if)]
    fn from_node(
        node: ::tree_sitter::Node<'tree>,
        _src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        match node.kind() {
            "+" => Ok(Self::Plus(::treesitter_types::Span::from(node))),
            "-" => Ok(Self::Minus(::treesitter_types::Span::from(node))),
            "~" => Ok(Self::Tilde(::treesitter_types::Span::from(node))),
            other => Err(::treesitter_types::ParseError::unexpected_kind(other, node)),
        }
    }
}
impl ::treesitter_types::Spanned for UnaryOperatorOperator {
    fn span(&self) -> ::treesitter_types::Span {
        match self {
            Self::Plus(span) => *span,
            Self::Minus(span) => *span,
            Self::Tilde(span) => *span,
        }
    }
}
#[derive(Debug, Clone)]
pub enum UnionPatternChildren<'tree> {
    ClassPattern(::std::boxed::Box<ClassPattern<'tree>>),
    ComplexPattern(::std::boxed::Box<ComplexPattern<'tree>>),
    ConcatenatedString(::std::boxed::Box<ConcatenatedString<'tree>>),
    DictPattern(::std::boxed::Box<DictPattern<'tree>>),
    DottedName(::std::boxed::Box<DottedName<'tree>>),
    False(::std::boxed::Box<False<'tree>>),
    Float(::std::boxed::Box<Float<'tree>>),
    Integer(::std::boxed::Box<Integer<'tree>>),
    ListPattern(::std::boxed::Box<ListPattern<'tree>>),
    None(::std::boxed::Box<None<'tree>>),
    SplatPattern(::std::boxed::Box<SplatPattern<'tree>>),
    String(::std::boxed::Box<String<'tree>>),
    True(::std::boxed::Box<True<'tree>>),
    TuplePattern(::std::boxed::Box<TuplePattern<'tree>>),
    UnionPattern(::std::boxed::Box<UnionPattern<'tree>>),
}
impl<'tree> ::treesitter_types::FromNode<'tree> for UnionPatternChildren<'tree> {
    #[allow(clippy::collapsible_else_if)]
    fn from_node(
        node: ::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        match node.kind() {
            "class_pattern" => Ok(Self::ClassPattern(::std::boxed::Box::new(
                <ClassPattern as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "complex_pattern" => Ok(Self::ComplexPattern(::std::boxed::Box::new(
                <ComplexPattern as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "concatenated_string" => Ok(Self::ConcatenatedString(::std::boxed::Box::new(
                <ConcatenatedString as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "dict_pattern" => Ok(Self::DictPattern(::std::boxed::Box::new(
                <DictPattern as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "dotted_name" => Ok(Self::DottedName(::std::boxed::Box::new(
                <DottedName as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "false" => Ok(Self::False(::std::boxed::Box::new(
                <False as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "float" => Ok(Self::Float(::std::boxed::Box::new(
                <Float as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "integer" => Ok(Self::Integer(::std::boxed::Box::new(
                <Integer as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "list_pattern" => Ok(Self::ListPattern(::std::boxed::Box::new(
                <ListPattern as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "none" => Ok(Self::None(::std::boxed::Box::new(
                <None as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "splat_pattern" => Ok(Self::SplatPattern(::std::boxed::Box::new(
                <SplatPattern as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "string" => Ok(Self::String(::std::boxed::Box::new(
                <String as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "true" => Ok(Self::True(::std::boxed::Box::new(
                <True as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "tuple_pattern" => Ok(Self::TuplePattern(::std::boxed::Box::new(
                <TuplePattern as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "union_pattern" => Ok(Self::UnionPattern(::std::boxed::Box::new(
                <UnionPattern as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            other => Err(::treesitter_types::ParseError::unexpected_kind(other, node)),
        }
    }
}
impl ::treesitter_types::Spanned for UnionPatternChildren<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        match self {
            Self::ClassPattern(inner) => inner.span(),
            Self::ComplexPattern(inner) => inner.span(),
            Self::ConcatenatedString(inner) => inner.span(),
            Self::DictPattern(inner) => inner.span(),
            Self::DottedName(inner) => inner.span(),
            Self::False(inner) => inner.span(),
            Self::Float(inner) => inner.span(),
            Self::Integer(inner) => inner.span(),
            Self::ListPattern(inner) => inner.span(),
            Self::None(inner) => inner.span(),
            Self::SplatPattern(inner) => inner.span(),
            Self::String(inner) => inner.span(),
            Self::True(inner) => inner.span(),
            Self::TuplePattern(inner) => inner.span(),
            Self::UnionPattern(inner) => inner.span(),
        }
    }
}
#[derive(Debug, Clone)]
pub enum YieldChildren<'tree> {
    Expression(::std::boxed::Box<Expression<'tree>>),
    ExpressionList(::std::boxed::Box<ExpressionList<'tree>>),
}
impl<'tree> ::treesitter_types::FromNode<'tree> for YieldChildren<'tree> {
    #[allow(clippy::collapsible_else_if)]
    fn from_node(
        node: ::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        match node.kind() {
            "expression_list" => Ok(Self::ExpressionList(::std::boxed::Box::new(
                <ExpressionList as ::treesitter_types::FromNode>::from_node(node, src)?,
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
impl ::treesitter_types::Spanned for YieldChildren<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        match self {
            Self::Expression(inner) => inner.span(),
            Self::ExpressionList(inner) => inner.span(),
        }
    }
}
#[derive(Debug, Clone)]
pub enum AnyNode<'tree> {
    CompoundStatement(CompoundStatement<'tree>),
    SimpleStatement(SimpleStatement<'tree>),
    Expression(Expression<'tree>),
    Parameter(Parameter<'tree>),
    Pattern(Pattern<'tree>),
    PrimaryExpression(PrimaryExpression<'tree>),
    AliasedImport(AliasedImport<'tree>),
    ArgumentList(ArgumentList<'tree>),
    AsPattern(AsPattern<'tree>),
    AssertStatement(AssertStatement<'tree>),
    Assignment(Assignment<'tree>),
    Attribute(Attribute<'tree>),
    AugmentedAssignment(AugmentedAssignment<'tree>),
    Await(Await<'tree>),
    BinaryOperator(BinaryOperator<'tree>),
    Block(Block<'tree>),
    BooleanOperator(BooleanOperator<'tree>),
    BreakStatement(BreakStatement<'tree>),
    Call(Call<'tree>),
    CaseClause(CaseClause<'tree>),
    CasePattern(CasePattern<'tree>),
    Chevron(Chevron<'tree>),
    ClassDefinition(ClassDefinition<'tree>),
    ClassPattern(ClassPattern<'tree>),
    ComparisonOperator(ComparisonOperator<'tree>),
    ComplexPattern(ComplexPattern<'tree>),
    ConcatenatedString(ConcatenatedString<'tree>),
    ConditionalExpression(ConditionalExpression<'tree>),
    ConstrainedType(ConstrainedType<'tree>),
    ContinueStatement(ContinueStatement<'tree>),
    DecoratedDefinition(DecoratedDefinition<'tree>),
    Decorator(Decorator<'tree>),
    DefaultParameter(DefaultParameter<'tree>),
    DeleteStatement(DeleteStatement<'tree>),
    DictPattern(DictPattern<'tree>),
    Dictionary(Dictionary<'tree>),
    DictionaryComprehension(DictionaryComprehension<'tree>),
    DictionarySplat(DictionarySplat<'tree>),
    DictionarySplatPattern(DictionarySplatPattern<'tree>),
    DottedName(DottedName<'tree>),
    ElifClause(ElifClause<'tree>),
    ElseClause(ElseClause<'tree>),
    ExceptClause(ExceptClause<'tree>),
    ExecStatement(ExecStatement<'tree>),
    ExpressionList(ExpressionList<'tree>),
    ExpressionStatement(ExpressionStatement<'tree>),
    FinallyClause(FinallyClause<'tree>),
    ForInClause(ForInClause<'tree>),
    ForStatement(ForStatement<'tree>),
    FormatExpression(FormatExpression<'tree>),
    FormatSpecifier(FormatSpecifier<'tree>),
    FunctionDefinition(FunctionDefinition<'tree>),
    FutureImportStatement(FutureImportStatement<'tree>),
    GeneratorExpression(GeneratorExpression<'tree>),
    GenericType(GenericType<'tree>),
    GlobalStatement(GlobalStatement<'tree>),
    IfClause(IfClause<'tree>),
    IfStatement(IfStatement<'tree>),
    ImportFromStatement(ImportFromStatement<'tree>),
    ImportPrefix(ImportPrefix<'tree>),
    ImportStatement(ImportStatement<'tree>),
    Interpolation(Interpolation<'tree>),
    KeywordArgument(KeywordArgument<'tree>),
    KeywordPattern(KeywordPattern<'tree>),
    KeywordSeparator(KeywordSeparator<'tree>),
    Lambda(Lambda<'tree>),
    LambdaParameters(LambdaParameters<'tree>),
    List(List<'tree>),
    ListComprehension(ListComprehension<'tree>),
    ListPattern(ListPattern<'tree>),
    ListSplat(ListSplat<'tree>),
    ListSplatPattern(ListSplatPattern<'tree>),
    MatchStatement(MatchStatement<'tree>),
    MemberType(MemberType<'tree>),
    Module(Module<'tree>),
    NamedExpression(NamedExpression<'tree>),
    NonlocalStatement(NonlocalStatement<'tree>),
    NotOperator(NotOperator<'tree>),
    Pair(Pair<'tree>),
    Parameters(Parameters<'tree>),
    ParenthesizedExpression(ParenthesizedExpression<'tree>),
    ParenthesizedListSplat(ParenthesizedListSplat<'tree>),
    PassStatement(PassStatement<'tree>),
    PatternList(PatternList<'tree>),
    PositionalSeparator(PositionalSeparator<'tree>),
    PrintStatement(PrintStatement<'tree>),
    RaiseStatement(RaiseStatement<'tree>),
    RelativeImport(RelativeImport<'tree>),
    ReturnStatement(ReturnStatement<'tree>),
    Set(Set<'tree>),
    SetComprehension(SetComprehension<'tree>),
    Slice(Slice<'tree>),
    SplatPattern(SplatPattern<'tree>),
    SplatType(SplatType<'tree>),
    String(String<'tree>),
    StringContent(StringContent<'tree>),
    Subscript(Subscript<'tree>),
    TryStatement(TryStatement<'tree>),
    Tuple(Tuple<'tree>),
    TuplePattern(TuplePattern<'tree>),
    Type(Type<'tree>),
    TypeAliasStatement(TypeAliasStatement<'tree>),
    TypeParameter(TypeParameter<'tree>),
    TypedDefaultParameter(TypedDefaultParameter<'tree>),
    TypedParameter(TypedParameter<'tree>),
    UnaryOperator(UnaryOperator<'tree>),
    UnionPattern(UnionPattern<'tree>),
    UnionType(UnionType<'tree>),
    WhileStatement(WhileStatement<'tree>),
    WildcardImport(WildcardImport<'tree>),
    WithClause(WithClause<'tree>),
    WithItem(WithItem<'tree>),
    WithStatement(WithStatement<'tree>),
    Yield(Yield<'tree>),
    Comment(Comment<'tree>),
    Ellipsis(Ellipsis<'tree>),
    EscapeInterpolation(EscapeInterpolation<'tree>),
    EscapeSequence(EscapeSequence<'tree>),
    False(False<'tree>),
    Float(Float<'tree>),
    Identifier(Identifier<'tree>),
    Integer(Integer<'tree>),
    LineContinuation(LineContinuation<'tree>),
    None(None<'tree>),
    StringEnd(StringEnd<'tree>),
    StringStart(StringStart<'tree>),
    True(True<'tree>),
    TypeConversion(TypeConversion<'tree>),
    AsPatternTarget(AsPatternTarget<'tree>),
    Unknown(::tree_sitter::Node<'tree>),
}
impl<'tree> AnyNode<'tree> {
    pub fn from_node(node: ::tree_sitter::Node<'tree>, src: &'tree [u8]) -> Self {
        match node.kind() {
            "_compound_statement" => {
                <CompoundStatement as ::treesitter_types::FromNode>::from_node(node, src)
                    .map(Self::CompoundStatement)
                    .unwrap_or(Self::Unknown(node))
            }
            "_simple_statement" => {
                <SimpleStatement as ::treesitter_types::FromNode>::from_node(node, src)
                    .map(Self::SimpleStatement)
                    .unwrap_or(Self::Unknown(node))
            }
            "expression" => <Expression as ::treesitter_types::FromNode>::from_node(node, src)
                .map(Self::Expression)
                .unwrap_or(Self::Unknown(node)),
            "parameter" => <Parameter as ::treesitter_types::FromNode>::from_node(node, src)
                .map(Self::Parameter)
                .unwrap_or(Self::Unknown(node)),
            "pattern" => <Pattern as ::treesitter_types::FromNode>::from_node(node, src)
                .map(Self::Pattern)
                .unwrap_or(Self::Unknown(node)),
            "primary_expression" => {
                <PrimaryExpression as ::treesitter_types::FromNode>::from_node(node, src)
                    .map(Self::PrimaryExpression)
                    .unwrap_or(Self::Unknown(node))
            }
            "aliased_import" => {
                <AliasedImport as ::treesitter_types::FromNode>::from_node(node, src)
                    .map(Self::AliasedImport)
                    .unwrap_or(Self::Unknown(node))
            }
            "argument_list" => <ArgumentList as ::treesitter_types::FromNode>::from_node(node, src)
                .map(Self::ArgumentList)
                .unwrap_or(Self::Unknown(node)),
            "as_pattern" => <AsPattern as ::treesitter_types::FromNode>::from_node(node, src)
                .map(Self::AsPattern)
                .unwrap_or(Self::Unknown(node)),
            "assert_statement" => {
                <AssertStatement as ::treesitter_types::FromNode>::from_node(node, src)
                    .map(Self::AssertStatement)
                    .unwrap_or(Self::Unknown(node))
            }
            "assignment" => <Assignment as ::treesitter_types::FromNode>::from_node(node, src)
                .map(Self::Assignment)
                .unwrap_or(Self::Unknown(node)),
            "attribute" => <Attribute as ::treesitter_types::FromNode>::from_node(node, src)
                .map(Self::Attribute)
                .unwrap_or(Self::Unknown(node)),
            "augmented_assignment" => {
                <AugmentedAssignment as ::treesitter_types::FromNode>::from_node(node, src)
                    .map(Self::AugmentedAssignment)
                    .unwrap_or(Self::Unknown(node))
            }
            "await" => <Await as ::treesitter_types::FromNode>::from_node(node, src)
                .map(Self::Await)
                .unwrap_or(Self::Unknown(node)),
            "binary_operator" => {
                <BinaryOperator as ::treesitter_types::FromNode>::from_node(node, src)
                    .map(Self::BinaryOperator)
                    .unwrap_or(Self::Unknown(node))
            }
            "block" => <Block as ::treesitter_types::FromNode>::from_node(node, src)
                .map(Self::Block)
                .unwrap_or(Self::Unknown(node)),
            "boolean_operator" => {
                <BooleanOperator as ::treesitter_types::FromNode>::from_node(node, src)
                    .map(Self::BooleanOperator)
                    .unwrap_or(Self::Unknown(node))
            }
            "break_statement" => {
                <BreakStatement as ::treesitter_types::FromNode>::from_node(node, src)
                    .map(Self::BreakStatement)
                    .unwrap_or(Self::Unknown(node))
            }
            "call" => <Call as ::treesitter_types::FromNode>::from_node(node, src)
                .map(Self::Call)
                .unwrap_or(Self::Unknown(node)),
            "case_clause" => <CaseClause as ::treesitter_types::FromNode>::from_node(node, src)
                .map(Self::CaseClause)
                .unwrap_or(Self::Unknown(node)),
            "case_pattern" => <CasePattern as ::treesitter_types::FromNode>::from_node(node, src)
                .map(Self::CasePattern)
                .unwrap_or(Self::Unknown(node)),
            "chevron" => <Chevron as ::treesitter_types::FromNode>::from_node(node, src)
                .map(Self::Chevron)
                .unwrap_or(Self::Unknown(node)),
            "class_definition" => {
                <ClassDefinition as ::treesitter_types::FromNode>::from_node(node, src)
                    .map(Self::ClassDefinition)
                    .unwrap_or(Self::Unknown(node))
            }
            "class_pattern" => <ClassPattern as ::treesitter_types::FromNode>::from_node(node, src)
                .map(Self::ClassPattern)
                .unwrap_or(Self::Unknown(node)),
            "comparison_operator" => {
                <ComparisonOperator as ::treesitter_types::FromNode>::from_node(node, src)
                    .map(Self::ComparisonOperator)
                    .unwrap_or(Self::Unknown(node))
            }
            "complex_pattern" => {
                <ComplexPattern as ::treesitter_types::FromNode>::from_node(node, src)
                    .map(Self::ComplexPattern)
                    .unwrap_or(Self::Unknown(node))
            }
            "concatenated_string" => {
                <ConcatenatedString as ::treesitter_types::FromNode>::from_node(node, src)
                    .map(Self::ConcatenatedString)
                    .unwrap_or(Self::Unknown(node))
            }
            "conditional_expression" => {
                <ConditionalExpression as ::treesitter_types::FromNode>::from_node(node, src)
                    .map(Self::ConditionalExpression)
                    .unwrap_or(Self::Unknown(node))
            }
            "constrained_type" => {
                <ConstrainedType as ::treesitter_types::FromNode>::from_node(node, src)
                    .map(Self::ConstrainedType)
                    .unwrap_or(Self::Unknown(node))
            }
            "continue_statement" => {
                <ContinueStatement as ::treesitter_types::FromNode>::from_node(node, src)
                    .map(Self::ContinueStatement)
                    .unwrap_or(Self::Unknown(node))
            }
            "decorated_definition" => {
                <DecoratedDefinition as ::treesitter_types::FromNode>::from_node(node, src)
                    .map(Self::DecoratedDefinition)
                    .unwrap_or(Self::Unknown(node))
            }
            "decorator" => <Decorator as ::treesitter_types::FromNode>::from_node(node, src)
                .map(Self::Decorator)
                .unwrap_or(Self::Unknown(node)),
            "default_parameter" => {
                <DefaultParameter as ::treesitter_types::FromNode>::from_node(node, src)
                    .map(Self::DefaultParameter)
                    .unwrap_or(Self::Unknown(node))
            }
            "delete_statement" => {
                <DeleteStatement as ::treesitter_types::FromNode>::from_node(node, src)
                    .map(Self::DeleteStatement)
                    .unwrap_or(Self::Unknown(node))
            }
            "dict_pattern" => <DictPattern as ::treesitter_types::FromNode>::from_node(node, src)
                .map(Self::DictPattern)
                .unwrap_or(Self::Unknown(node)),
            "dictionary" => <Dictionary as ::treesitter_types::FromNode>::from_node(node, src)
                .map(Self::Dictionary)
                .unwrap_or(Self::Unknown(node)),
            "dictionary_comprehension" => {
                <DictionaryComprehension as ::treesitter_types::FromNode>::from_node(node, src)
                    .map(Self::DictionaryComprehension)
                    .unwrap_or(Self::Unknown(node))
            }
            "dictionary_splat" => {
                <DictionarySplat as ::treesitter_types::FromNode>::from_node(node, src)
                    .map(Self::DictionarySplat)
                    .unwrap_or(Self::Unknown(node))
            }
            "dictionary_splat_pattern" => {
                <DictionarySplatPattern as ::treesitter_types::FromNode>::from_node(node, src)
                    .map(Self::DictionarySplatPattern)
                    .unwrap_or(Self::Unknown(node))
            }
            "dotted_name" => <DottedName as ::treesitter_types::FromNode>::from_node(node, src)
                .map(Self::DottedName)
                .unwrap_or(Self::Unknown(node)),
            "elif_clause" => <ElifClause as ::treesitter_types::FromNode>::from_node(node, src)
                .map(Self::ElifClause)
                .unwrap_or(Self::Unknown(node)),
            "else_clause" => <ElseClause as ::treesitter_types::FromNode>::from_node(node, src)
                .map(Self::ElseClause)
                .unwrap_or(Self::Unknown(node)),
            "except_clause" => <ExceptClause as ::treesitter_types::FromNode>::from_node(node, src)
                .map(Self::ExceptClause)
                .unwrap_or(Self::Unknown(node)),
            "exec_statement" => {
                <ExecStatement as ::treesitter_types::FromNode>::from_node(node, src)
                    .map(Self::ExecStatement)
                    .unwrap_or(Self::Unknown(node))
            }
            "expression_list" => {
                <ExpressionList as ::treesitter_types::FromNode>::from_node(node, src)
                    .map(Self::ExpressionList)
                    .unwrap_or(Self::Unknown(node))
            }
            "expression_statement" => {
                <ExpressionStatement as ::treesitter_types::FromNode>::from_node(node, src)
                    .map(Self::ExpressionStatement)
                    .unwrap_or(Self::Unknown(node))
            }
            "finally_clause" => {
                <FinallyClause as ::treesitter_types::FromNode>::from_node(node, src)
                    .map(Self::FinallyClause)
                    .unwrap_or(Self::Unknown(node))
            }
            "for_in_clause" => <ForInClause as ::treesitter_types::FromNode>::from_node(node, src)
                .map(Self::ForInClause)
                .unwrap_or(Self::Unknown(node)),
            "for_statement" => <ForStatement as ::treesitter_types::FromNode>::from_node(node, src)
                .map(Self::ForStatement)
                .unwrap_or(Self::Unknown(node)),
            "format_expression" => {
                <FormatExpression as ::treesitter_types::FromNode>::from_node(node, src)
                    .map(Self::FormatExpression)
                    .unwrap_or(Self::Unknown(node))
            }
            "format_specifier" => {
                <FormatSpecifier as ::treesitter_types::FromNode>::from_node(node, src)
                    .map(Self::FormatSpecifier)
                    .unwrap_or(Self::Unknown(node))
            }
            "function_definition" => {
                <FunctionDefinition as ::treesitter_types::FromNode>::from_node(node, src)
                    .map(Self::FunctionDefinition)
                    .unwrap_or(Self::Unknown(node))
            }
            "future_import_statement" => {
                <FutureImportStatement as ::treesitter_types::FromNode>::from_node(node, src)
                    .map(Self::FutureImportStatement)
                    .unwrap_or(Self::Unknown(node))
            }
            "generator_expression" => {
                <GeneratorExpression as ::treesitter_types::FromNode>::from_node(node, src)
                    .map(Self::GeneratorExpression)
                    .unwrap_or(Self::Unknown(node))
            }
            "generic_type" => <GenericType as ::treesitter_types::FromNode>::from_node(node, src)
                .map(Self::GenericType)
                .unwrap_or(Self::Unknown(node)),
            "global_statement" => {
                <GlobalStatement as ::treesitter_types::FromNode>::from_node(node, src)
                    .map(Self::GlobalStatement)
                    .unwrap_or(Self::Unknown(node))
            }
            "if_clause" => <IfClause as ::treesitter_types::FromNode>::from_node(node, src)
                .map(Self::IfClause)
                .unwrap_or(Self::Unknown(node)),
            "if_statement" => <IfStatement as ::treesitter_types::FromNode>::from_node(node, src)
                .map(Self::IfStatement)
                .unwrap_or(Self::Unknown(node)),
            "import_from_statement" => {
                <ImportFromStatement as ::treesitter_types::FromNode>::from_node(node, src)
                    .map(Self::ImportFromStatement)
                    .unwrap_or(Self::Unknown(node))
            }
            "import_prefix" => <ImportPrefix as ::treesitter_types::FromNode>::from_node(node, src)
                .map(Self::ImportPrefix)
                .unwrap_or(Self::Unknown(node)),
            "import_statement" => {
                <ImportStatement as ::treesitter_types::FromNode>::from_node(node, src)
                    .map(Self::ImportStatement)
                    .unwrap_or(Self::Unknown(node))
            }
            "interpolation" => {
                <Interpolation as ::treesitter_types::FromNode>::from_node(node, src)
                    .map(Self::Interpolation)
                    .unwrap_or(Self::Unknown(node))
            }
            "keyword_argument" => {
                <KeywordArgument as ::treesitter_types::FromNode>::from_node(node, src)
                    .map(Self::KeywordArgument)
                    .unwrap_or(Self::Unknown(node))
            }
            "keyword_pattern" => {
                <KeywordPattern as ::treesitter_types::FromNode>::from_node(node, src)
                    .map(Self::KeywordPattern)
                    .unwrap_or(Self::Unknown(node))
            }
            "keyword_separator" => {
                <KeywordSeparator as ::treesitter_types::FromNode>::from_node(node, src)
                    .map(Self::KeywordSeparator)
                    .unwrap_or(Self::Unknown(node))
            }
            "lambda" => <Lambda as ::treesitter_types::FromNode>::from_node(node, src)
                .map(Self::Lambda)
                .unwrap_or(Self::Unknown(node)),
            "lambda_parameters" => {
                <LambdaParameters as ::treesitter_types::FromNode>::from_node(node, src)
                    .map(Self::LambdaParameters)
                    .unwrap_or(Self::Unknown(node))
            }
            "list" => <List as ::treesitter_types::FromNode>::from_node(node, src)
                .map(Self::List)
                .unwrap_or(Self::Unknown(node)),
            "list_comprehension" => {
                <ListComprehension as ::treesitter_types::FromNode>::from_node(node, src)
                    .map(Self::ListComprehension)
                    .unwrap_or(Self::Unknown(node))
            }
            "list_pattern" => <ListPattern as ::treesitter_types::FromNode>::from_node(node, src)
                .map(Self::ListPattern)
                .unwrap_or(Self::Unknown(node)),
            "list_splat" => <ListSplat as ::treesitter_types::FromNode>::from_node(node, src)
                .map(Self::ListSplat)
                .unwrap_or(Self::Unknown(node)),
            "list_splat_pattern" => {
                <ListSplatPattern as ::treesitter_types::FromNode>::from_node(node, src)
                    .map(Self::ListSplatPattern)
                    .unwrap_or(Self::Unknown(node))
            }
            "match_statement" => {
                <MatchStatement as ::treesitter_types::FromNode>::from_node(node, src)
                    .map(Self::MatchStatement)
                    .unwrap_or(Self::Unknown(node))
            }
            "member_type" => <MemberType as ::treesitter_types::FromNode>::from_node(node, src)
                .map(Self::MemberType)
                .unwrap_or(Self::Unknown(node)),
            "module" => <Module as ::treesitter_types::FromNode>::from_node(node, src)
                .map(Self::Module)
                .unwrap_or(Self::Unknown(node)),
            "named_expression" => {
                <NamedExpression as ::treesitter_types::FromNode>::from_node(node, src)
                    .map(Self::NamedExpression)
                    .unwrap_or(Self::Unknown(node))
            }
            "nonlocal_statement" => {
                <NonlocalStatement as ::treesitter_types::FromNode>::from_node(node, src)
                    .map(Self::NonlocalStatement)
                    .unwrap_or(Self::Unknown(node))
            }
            "not_operator" => <NotOperator as ::treesitter_types::FromNode>::from_node(node, src)
                .map(Self::NotOperator)
                .unwrap_or(Self::Unknown(node)),
            "pair" => <Pair as ::treesitter_types::FromNode>::from_node(node, src)
                .map(Self::Pair)
                .unwrap_or(Self::Unknown(node)),
            "parameters" => <Parameters as ::treesitter_types::FromNode>::from_node(node, src)
                .map(Self::Parameters)
                .unwrap_or(Self::Unknown(node)),
            "parenthesized_expression" => {
                <ParenthesizedExpression as ::treesitter_types::FromNode>::from_node(node, src)
                    .map(Self::ParenthesizedExpression)
                    .unwrap_or(Self::Unknown(node))
            }
            "parenthesized_list_splat" => {
                <ParenthesizedListSplat as ::treesitter_types::FromNode>::from_node(node, src)
                    .map(Self::ParenthesizedListSplat)
                    .unwrap_or(Self::Unknown(node))
            }
            "pass_statement" => {
                <PassStatement as ::treesitter_types::FromNode>::from_node(node, src)
                    .map(Self::PassStatement)
                    .unwrap_or(Self::Unknown(node))
            }
            "pattern_list" => <PatternList as ::treesitter_types::FromNode>::from_node(node, src)
                .map(Self::PatternList)
                .unwrap_or(Self::Unknown(node)),
            "positional_separator" => {
                <PositionalSeparator as ::treesitter_types::FromNode>::from_node(node, src)
                    .map(Self::PositionalSeparator)
                    .unwrap_or(Self::Unknown(node))
            }
            "print_statement" => {
                <PrintStatement as ::treesitter_types::FromNode>::from_node(node, src)
                    .map(Self::PrintStatement)
                    .unwrap_or(Self::Unknown(node))
            }
            "raise_statement" => {
                <RaiseStatement as ::treesitter_types::FromNode>::from_node(node, src)
                    .map(Self::RaiseStatement)
                    .unwrap_or(Self::Unknown(node))
            }
            "relative_import" => {
                <RelativeImport as ::treesitter_types::FromNode>::from_node(node, src)
                    .map(Self::RelativeImport)
                    .unwrap_or(Self::Unknown(node))
            }
            "return_statement" => {
                <ReturnStatement as ::treesitter_types::FromNode>::from_node(node, src)
                    .map(Self::ReturnStatement)
                    .unwrap_or(Self::Unknown(node))
            }
            "set" => <Set as ::treesitter_types::FromNode>::from_node(node, src)
                .map(Self::Set)
                .unwrap_or(Self::Unknown(node)),
            "set_comprehension" => {
                <SetComprehension as ::treesitter_types::FromNode>::from_node(node, src)
                    .map(Self::SetComprehension)
                    .unwrap_or(Self::Unknown(node))
            }
            "slice" => <Slice as ::treesitter_types::FromNode>::from_node(node, src)
                .map(Self::Slice)
                .unwrap_or(Self::Unknown(node)),
            "splat_pattern" => <SplatPattern as ::treesitter_types::FromNode>::from_node(node, src)
                .map(Self::SplatPattern)
                .unwrap_or(Self::Unknown(node)),
            "splat_type" => <SplatType as ::treesitter_types::FromNode>::from_node(node, src)
                .map(Self::SplatType)
                .unwrap_or(Self::Unknown(node)),
            "string" => <String as ::treesitter_types::FromNode>::from_node(node, src)
                .map(Self::String)
                .unwrap_or(Self::Unknown(node)),
            "string_content" => {
                <StringContent as ::treesitter_types::FromNode>::from_node(node, src)
                    .map(Self::StringContent)
                    .unwrap_or(Self::Unknown(node))
            }
            "subscript" => <Subscript as ::treesitter_types::FromNode>::from_node(node, src)
                .map(Self::Subscript)
                .unwrap_or(Self::Unknown(node)),
            "try_statement" => <TryStatement as ::treesitter_types::FromNode>::from_node(node, src)
                .map(Self::TryStatement)
                .unwrap_or(Self::Unknown(node)),
            "tuple" => <Tuple as ::treesitter_types::FromNode>::from_node(node, src)
                .map(Self::Tuple)
                .unwrap_or(Self::Unknown(node)),
            "tuple_pattern" => <TuplePattern as ::treesitter_types::FromNode>::from_node(node, src)
                .map(Self::TuplePattern)
                .unwrap_or(Self::Unknown(node)),
            "type" => <Type as ::treesitter_types::FromNode>::from_node(node, src)
                .map(Self::Type)
                .unwrap_or(Self::Unknown(node)),
            "type_alias_statement" => {
                <TypeAliasStatement as ::treesitter_types::FromNode>::from_node(node, src)
                    .map(Self::TypeAliasStatement)
                    .unwrap_or(Self::Unknown(node))
            }
            "type_parameter" => {
                <TypeParameter as ::treesitter_types::FromNode>::from_node(node, src)
                    .map(Self::TypeParameter)
                    .unwrap_or(Self::Unknown(node))
            }
            "typed_default_parameter" => {
                <TypedDefaultParameter as ::treesitter_types::FromNode>::from_node(node, src)
                    .map(Self::TypedDefaultParameter)
                    .unwrap_or(Self::Unknown(node))
            }
            "typed_parameter" => {
                <TypedParameter as ::treesitter_types::FromNode>::from_node(node, src)
                    .map(Self::TypedParameter)
                    .unwrap_or(Self::Unknown(node))
            }
            "unary_operator" => {
                <UnaryOperator as ::treesitter_types::FromNode>::from_node(node, src)
                    .map(Self::UnaryOperator)
                    .unwrap_or(Self::Unknown(node))
            }
            "union_pattern" => <UnionPattern as ::treesitter_types::FromNode>::from_node(node, src)
                .map(Self::UnionPattern)
                .unwrap_or(Self::Unknown(node)),
            "union_type" => <UnionType as ::treesitter_types::FromNode>::from_node(node, src)
                .map(Self::UnionType)
                .unwrap_or(Self::Unknown(node)),
            "while_statement" => {
                <WhileStatement as ::treesitter_types::FromNode>::from_node(node, src)
                    .map(Self::WhileStatement)
                    .unwrap_or(Self::Unknown(node))
            }
            "wildcard_import" => {
                <WildcardImport as ::treesitter_types::FromNode>::from_node(node, src)
                    .map(Self::WildcardImport)
                    .unwrap_or(Self::Unknown(node))
            }
            "with_clause" => <WithClause as ::treesitter_types::FromNode>::from_node(node, src)
                .map(Self::WithClause)
                .unwrap_or(Self::Unknown(node)),
            "with_item" => <WithItem as ::treesitter_types::FromNode>::from_node(node, src)
                .map(Self::WithItem)
                .unwrap_or(Self::Unknown(node)),
            "with_statement" => {
                <WithStatement as ::treesitter_types::FromNode>::from_node(node, src)
                    .map(Self::WithStatement)
                    .unwrap_or(Self::Unknown(node))
            }
            "yield" => <Yield as ::treesitter_types::FromNode>::from_node(node, src)
                .map(Self::Yield)
                .unwrap_or(Self::Unknown(node)),
            "comment" => <Comment as ::treesitter_types::FromNode>::from_node(node, src)
                .map(Self::Comment)
                .unwrap_or(Self::Unknown(node)),
            "ellipsis" => <Ellipsis as ::treesitter_types::FromNode>::from_node(node, src)
                .map(Self::Ellipsis)
                .unwrap_or(Self::Unknown(node)),
            "escape_interpolation" => {
                <EscapeInterpolation as ::treesitter_types::FromNode>::from_node(node, src)
                    .map(Self::EscapeInterpolation)
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
            "float" => <Float as ::treesitter_types::FromNode>::from_node(node, src)
                .map(Self::Float)
                .unwrap_or(Self::Unknown(node)),
            "identifier" => <Identifier as ::treesitter_types::FromNode>::from_node(node, src)
                .map(Self::Identifier)
                .unwrap_or(Self::Unknown(node)),
            "integer" => <Integer as ::treesitter_types::FromNode>::from_node(node, src)
                .map(Self::Integer)
                .unwrap_or(Self::Unknown(node)),
            "line_continuation" => {
                <LineContinuation as ::treesitter_types::FromNode>::from_node(node, src)
                    .map(Self::LineContinuation)
                    .unwrap_or(Self::Unknown(node))
            }
            "none" => <None as ::treesitter_types::FromNode>::from_node(node, src)
                .map(Self::None)
                .unwrap_or(Self::Unknown(node)),
            "string_end" => <StringEnd as ::treesitter_types::FromNode>::from_node(node, src)
                .map(Self::StringEnd)
                .unwrap_or(Self::Unknown(node)),
            "string_start" => <StringStart as ::treesitter_types::FromNode>::from_node(node, src)
                .map(Self::StringStart)
                .unwrap_or(Self::Unknown(node)),
            "true" => <True as ::treesitter_types::FromNode>::from_node(node, src)
                .map(Self::True)
                .unwrap_or(Self::Unknown(node)),
            "type_conversion" => {
                <TypeConversion as ::treesitter_types::FromNode>::from_node(node, src)
                    .map(Self::TypeConversion)
                    .unwrap_or(Self::Unknown(node))
            }
            "as_pattern_target" => {
                <AsPatternTarget as ::treesitter_types::FromNode>::from_node(node, src)
                    .map(Self::AsPatternTarget)
                    .unwrap_or(Self::Unknown(node))
            }
            _ => Self::Unknown(node),
        }
    }
}
impl ::treesitter_types::Spanned for AnyNode<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        match self {
            Self::CompoundStatement(inner) => inner.span(),
            Self::SimpleStatement(inner) => inner.span(),
            Self::Expression(inner) => inner.span(),
            Self::Parameter(inner) => inner.span(),
            Self::Pattern(inner) => inner.span(),
            Self::PrimaryExpression(inner) => inner.span(),
            Self::AliasedImport(inner) => inner.span(),
            Self::ArgumentList(inner) => inner.span(),
            Self::AsPattern(inner) => inner.span(),
            Self::AssertStatement(inner) => inner.span(),
            Self::Assignment(inner) => inner.span(),
            Self::Attribute(inner) => inner.span(),
            Self::AugmentedAssignment(inner) => inner.span(),
            Self::Await(inner) => inner.span(),
            Self::BinaryOperator(inner) => inner.span(),
            Self::Block(inner) => inner.span(),
            Self::BooleanOperator(inner) => inner.span(),
            Self::BreakStatement(inner) => inner.span(),
            Self::Call(inner) => inner.span(),
            Self::CaseClause(inner) => inner.span(),
            Self::CasePattern(inner) => inner.span(),
            Self::Chevron(inner) => inner.span(),
            Self::ClassDefinition(inner) => inner.span(),
            Self::ClassPattern(inner) => inner.span(),
            Self::ComparisonOperator(inner) => inner.span(),
            Self::ComplexPattern(inner) => inner.span(),
            Self::ConcatenatedString(inner) => inner.span(),
            Self::ConditionalExpression(inner) => inner.span(),
            Self::ConstrainedType(inner) => inner.span(),
            Self::ContinueStatement(inner) => inner.span(),
            Self::DecoratedDefinition(inner) => inner.span(),
            Self::Decorator(inner) => inner.span(),
            Self::DefaultParameter(inner) => inner.span(),
            Self::DeleteStatement(inner) => inner.span(),
            Self::DictPattern(inner) => inner.span(),
            Self::Dictionary(inner) => inner.span(),
            Self::DictionaryComprehension(inner) => inner.span(),
            Self::DictionarySplat(inner) => inner.span(),
            Self::DictionarySplatPattern(inner) => inner.span(),
            Self::DottedName(inner) => inner.span(),
            Self::ElifClause(inner) => inner.span(),
            Self::ElseClause(inner) => inner.span(),
            Self::ExceptClause(inner) => inner.span(),
            Self::ExecStatement(inner) => inner.span(),
            Self::ExpressionList(inner) => inner.span(),
            Self::ExpressionStatement(inner) => inner.span(),
            Self::FinallyClause(inner) => inner.span(),
            Self::ForInClause(inner) => inner.span(),
            Self::ForStatement(inner) => inner.span(),
            Self::FormatExpression(inner) => inner.span(),
            Self::FormatSpecifier(inner) => inner.span(),
            Self::FunctionDefinition(inner) => inner.span(),
            Self::FutureImportStatement(inner) => inner.span(),
            Self::GeneratorExpression(inner) => inner.span(),
            Self::GenericType(inner) => inner.span(),
            Self::GlobalStatement(inner) => inner.span(),
            Self::IfClause(inner) => inner.span(),
            Self::IfStatement(inner) => inner.span(),
            Self::ImportFromStatement(inner) => inner.span(),
            Self::ImportPrefix(inner) => inner.span(),
            Self::ImportStatement(inner) => inner.span(),
            Self::Interpolation(inner) => inner.span(),
            Self::KeywordArgument(inner) => inner.span(),
            Self::KeywordPattern(inner) => inner.span(),
            Self::KeywordSeparator(inner) => inner.span(),
            Self::Lambda(inner) => inner.span(),
            Self::LambdaParameters(inner) => inner.span(),
            Self::List(inner) => inner.span(),
            Self::ListComprehension(inner) => inner.span(),
            Self::ListPattern(inner) => inner.span(),
            Self::ListSplat(inner) => inner.span(),
            Self::ListSplatPattern(inner) => inner.span(),
            Self::MatchStatement(inner) => inner.span(),
            Self::MemberType(inner) => inner.span(),
            Self::Module(inner) => inner.span(),
            Self::NamedExpression(inner) => inner.span(),
            Self::NonlocalStatement(inner) => inner.span(),
            Self::NotOperator(inner) => inner.span(),
            Self::Pair(inner) => inner.span(),
            Self::Parameters(inner) => inner.span(),
            Self::ParenthesizedExpression(inner) => inner.span(),
            Self::ParenthesizedListSplat(inner) => inner.span(),
            Self::PassStatement(inner) => inner.span(),
            Self::PatternList(inner) => inner.span(),
            Self::PositionalSeparator(inner) => inner.span(),
            Self::PrintStatement(inner) => inner.span(),
            Self::RaiseStatement(inner) => inner.span(),
            Self::RelativeImport(inner) => inner.span(),
            Self::ReturnStatement(inner) => inner.span(),
            Self::Set(inner) => inner.span(),
            Self::SetComprehension(inner) => inner.span(),
            Self::Slice(inner) => inner.span(),
            Self::SplatPattern(inner) => inner.span(),
            Self::SplatType(inner) => inner.span(),
            Self::String(inner) => inner.span(),
            Self::StringContent(inner) => inner.span(),
            Self::Subscript(inner) => inner.span(),
            Self::TryStatement(inner) => inner.span(),
            Self::Tuple(inner) => inner.span(),
            Self::TuplePattern(inner) => inner.span(),
            Self::Type(inner) => inner.span(),
            Self::TypeAliasStatement(inner) => inner.span(),
            Self::TypeParameter(inner) => inner.span(),
            Self::TypedDefaultParameter(inner) => inner.span(),
            Self::TypedParameter(inner) => inner.span(),
            Self::UnaryOperator(inner) => inner.span(),
            Self::UnionPattern(inner) => inner.span(),
            Self::UnionType(inner) => inner.span(),
            Self::WhileStatement(inner) => inner.span(),
            Self::WildcardImport(inner) => inner.span(),
            Self::WithClause(inner) => inner.span(),
            Self::WithItem(inner) => inner.span(),
            Self::WithStatement(inner) => inner.span(),
            Self::Yield(inner) => inner.span(),
            Self::Comment(inner) => inner.span(),
            Self::Ellipsis(inner) => inner.span(),
            Self::EscapeInterpolation(inner) => inner.span(),
            Self::EscapeSequence(inner) => inner.span(),
            Self::False(inner) => inner.span(),
            Self::Float(inner) => inner.span(),
            Self::Identifier(inner) => inner.span(),
            Self::Integer(inner) => inner.span(),
            Self::LineContinuation(inner) => inner.span(),
            Self::None(inner) => inner.span(),
            Self::StringEnd(inner) => inner.span(),
            Self::StringStart(inner) => inner.span(),
            Self::True(inner) => inner.span(),
            Self::TypeConversion(inner) => inner.span(),
            Self::AsPatternTarget(inner) => inner.span(),
            Self::Unknown(node) => ::treesitter_types::Span::from(*node),
        }
    }
}
