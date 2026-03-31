#[derive(Debug, Clone)]
pub enum Expression<'tree> {
    BinaryExpression(::std::boxed::Box<BinaryExpression<'tree>>),
    CallExpression(::std::boxed::Box<CallExpression<'tree>>),
    CompositeLiteral(::std::boxed::Box<CompositeLiteral<'tree>>),
    False(::std::boxed::Box<False<'tree>>),
    FloatLiteral(::std::boxed::Box<FloatLiteral<'tree>>),
    FuncLiteral(::std::boxed::Box<FuncLiteral<'tree>>),
    Identifier(::std::boxed::Box<Identifier<'tree>>),
    ImaginaryLiteral(::std::boxed::Box<ImaginaryLiteral<'tree>>),
    IndexExpression(::std::boxed::Box<IndexExpression<'tree>>),
    IntLiteral(::std::boxed::Box<IntLiteral<'tree>>),
    InterpretedStringLiteral(::std::boxed::Box<InterpretedStringLiteral<'tree>>),
    Iota(::std::boxed::Box<Iota<'tree>>),
    Nil(::std::boxed::Box<Nil<'tree>>),
    ParenthesizedExpression(::std::boxed::Box<ParenthesizedExpression<'tree>>),
    RawStringLiteral(::std::boxed::Box<RawStringLiteral<'tree>>),
    RuneLiteral(::std::boxed::Box<RuneLiteral<'tree>>),
    SelectorExpression(::std::boxed::Box<SelectorExpression<'tree>>),
    SliceExpression(::std::boxed::Box<SliceExpression<'tree>>),
    True(::std::boxed::Box<True<'tree>>),
    TypeAssertionExpression(::std::boxed::Box<TypeAssertionExpression<'tree>>),
    TypeConversionExpression(::std::boxed::Box<TypeConversionExpression<'tree>>),
    TypeInstantiationExpression(::std::boxed::Box<TypeInstantiationExpression<'tree>>),
    UnaryExpression(::std::boxed::Box<UnaryExpression<'tree>>),
}
impl<'tree> ::treesitter_types::FromNode<'tree> for Expression<'tree> {
    #[allow(clippy::collapsible_else_if)]
    fn from_node(
        node: ::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        match node.kind() {
            "binary_expression" => {
                Ok(
                    Self::BinaryExpression(
                        ::std::boxed::Box::new(
                            <BinaryExpression as ::treesitter_types::FromNode>::from_node(
                                node,
                                src,
                            )?,
                        ),
                    ),
                )
            }
            "call_expression" => {
                Ok(
                    Self::CallExpression(
                        ::std::boxed::Box::new(
                            <CallExpression as ::treesitter_types::FromNode>::from_node(
                                node,
                                src,
                            )?,
                        ),
                    ),
                )
            }
            "composite_literal" => {
                Ok(
                    Self::CompositeLiteral(
                        ::std::boxed::Box::new(
                            <CompositeLiteral as ::treesitter_types::FromNode>::from_node(
                                node,
                                src,
                            )?,
                        ),
                    ),
                )
            }
            "false" => {
                Ok(
                    Self::False(
                        ::std::boxed::Box::new(
                            <False as ::treesitter_types::FromNode>::from_node(
                                node,
                                src,
                            )?,
                        ),
                    ),
                )
            }
            "float_literal" => {
                Ok(
                    Self::FloatLiteral(
                        ::std::boxed::Box::new(
                            <FloatLiteral as ::treesitter_types::FromNode>::from_node(
                                node,
                                src,
                            )?,
                        ),
                    ),
                )
            }
            "func_literal" => {
                Ok(
                    Self::FuncLiteral(
                        ::std::boxed::Box::new(
                            <FuncLiteral as ::treesitter_types::FromNode>::from_node(
                                node,
                                src,
                            )?,
                        ),
                    ),
                )
            }
            "identifier" => {
                Ok(
                    Self::Identifier(
                        ::std::boxed::Box::new(
                            <Identifier as ::treesitter_types::FromNode>::from_node(
                                node,
                                src,
                            )?,
                        ),
                    ),
                )
            }
            "imaginary_literal" => {
                Ok(
                    Self::ImaginaryLiteral(
                        ::std::boxed::Box::new(
                            <ImaginaryLiteral as ::treesitter_types::FromNode>::from_node(
                                node,
                                src,
                            )?,
                        ),
                    ),
                )
            }
            "index_expression" => {
                Ok(
                    Self::IndexExpression(
                        ::std::boxed::Box::new(
                            <IndexExpression as ::treesitter_types::FromNode>::from_node(
                                node,
                                src,
                            )?,
                        ),
                    ),
                )
            }
            "int_literal" => {
                Ok(
                    Self::IntLiteral(
                        ::std::boxed::Box::new(
                            <IntLiteral as ::treesitter_types::FromNode>::from_node(
                                node,
                                src,
                            )?,
                        ),
                    ),
                )
            }
            "interpreted_string_literal" => {
                Ok(
                    Self::InterpretedStringLiteral(
                        ::std::boxed::Box::new(
                            <InterpretedStringLiteral as ::treesitter_types::FromNode>::from_node(
                                node,
                                src,
                            )?,
                        ),
                    ),
                )
            }
            "iota" => {
                Ok(
                    Self::Iota(
                        ::std::boxed::Box::new(
                            <Iota as ::treesitter_types::FromNode>::from_node(node, src)?,
                        ),
                    ),
                )
            }
            "nil" => {
                Ok(
                    Self::Nil(
                        ::std::boxed::Box::new(
                            <Nil as ::treesitter_types::FromNode>::from_node(node, src)?,
                        ),
                    ),
                )
            }
            "parenthesized_expression" => {
                Ok(
                    Self::ParenthesizedExpression(
                        ::std::boxed::Box::new(
                            <ParenthesizedExpression as ::treesitter_types::FromNode>::from_node(
                                node,
                                src,
                            )?,
                        ),
                    ),
                )
            }
            "raw_string_literal" => {
                Ok(
                    Self::RawStringLiteral(
                        ::std::boxed::Box::new(
                            <RawStringLiteral as ::treesitter_types::FromNode>::from_node(
                                node,
                                src,
                            )?,
                        ),
                    ),
                )
            }
            "rune_literal" => {
                Ok(
                    Self::RuneLiteral(
                        ::std::boxed::Box::new(
                            <RuneLiteral as ::treesitter_types::FromNode>::from_node(
                                node,
                                src,
                            )?,
                        ),
                    ),
                )
            }
            "selector_expression" => {
                Ok(
                    Self::SelectorExpression(
                        ::std::boxed::Box::new(
                            <SelectorExpression as ::treesitter_types::FromNode>::from_node(
                                node,
                                src,
                            )?,
                        ),
                    ),
                )
            }
            "slice_expression" => {
                Ok(
                    Self::SliceExpression(
                        ::std::boxed::Box::new(
                            <SliceExpression as ::treesitter_types::FromNode>::from_node(
                                node,
                                src,
                            )?,
                        ),
                    ),
                )
            }
            "true" => {
                Ok(
                    Self::True(
                        ::std::boxed::Box::new(
                            <True as ::treesitter_types::FromNode>::from_node(node, src)?,
                        ),
                    ),
                )
            }
            "type_assertion_expression" => {
                Ok(
                    Self::TypeAssertionExpression(
                        ::std::boxed::Box::new(
                            <TypeAssertionExpression as ::treesitter_types::FromNode>::from_node(
                                node,
                                src,
                            )?,
                        ),
                    ),
                )
            }
            "type_conversion_expression" => {
                Ok(
                    Self::TypeConversionExpression(
                        ::std::boxed::Box::new(
                            <TypeConversionExpression as ::treesitter_types::FromNode>::from_node(
                                node,
                                src,
                            )?,
                        ),
                    ),
                )
            }
            "type_instantiation_expression" => {
                Ok(
                    Self::TypeInstantiationExpression(
                        ::std::boxed::Box::new(
                            <TypeInstantiationExpression as ::treesitter_types::FromNode>::from_node(
                                node,
                                src,
                            )?,
                        ),
                    ),
                )
            }
            "unary_expression" => {
                Ok(
                    Self::UnaryExpression(
                        ::std::boxed::Box::new(
                            <UnaryExpression as ::treesitter_types::FromNode>::from_node(
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
impl ::treesitter_types::Spanned for Expression<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        match self {
            Self::BinaryExpression(inner) => inner.span(),
            Self::CallExpression(inner) => inner.span(),
            Self::CompositeLiteral(inner) => inner.span(),
            Self::False(inner) => inner.span(),
            Self::FloatLiteral(inner) => inner.span(),
            Self::FuncLiteral(inner) => inner.span(),
            Self::Identifier(inner) => inner.span(),
            Self::ImaginaryLiteral(inner) => inner.span(),
            Self::IndexExpression(inner) => inner.span(),
            Self::IntLiteral(inner) => inner.span(),
            Self::InterpretedStringLiteral(inner) => inner.span(),
            Self::Iota(inner) => inner.span(),
            Self::Nil(inner) => inner.span(),
            Self::ParenthesizedExpression(inner) => inner.span(),
            Self::RawStringLiteral(inner) => inner.span(),
            Self::RuneLiteral(inner) => inner.span(),
            Self::SelectorExpression(inner) => inner.span(),
            Self::SliceExpression(inner) => inner.span(),
            Self::True(inner) => inner.span(),
            Self::TypeAssertionExpression(inner) => inner.span(),
            Self::TypeConversionExpression(inner) => inner.span(),
            Self::TypeInstantiationExpression(inner) => inner.span(),
            Self::UnaryExpression(inner) => inner.span(),
        }
    }
}
#[derive(Debug, Clone)]
pub enum SimpleStatement<'tree> {
    AssignmentStatement(::std::boxed::Box<AssignmentStatement<'tree>>),
    DecStatement(::std::boxed::Box<DecStatement<'tree>>),
    ExpressionStatement(::std::boxed::Box<ExpressionStatement<'tree>>),
    IncStatement(::std::boxed::Box<IncStatement<'tree>>),
    SendStatement(::std::boxed::Box<SendStatement<'tree>>),
    ShortVarDeclaration(::std::boxed::Box<ShortVarDeclaration<'tree>>),
}
impl<'tree> ::treesitter_types::FromNode<'tree> for SimpleStatement<'tree> {
    #[allow(clippy::collapsible_else_if)]
    fn from_node(
        node: ::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        match node.kind() {
            "assignment_statement" => {
                Ok(
                    Self::AssignmentStatement(
                        ::std::boxed::Box::new(
                            <AssignmentStatement as ::treesitter_types::FromNode>::from_node(
                                node,
                                src,
                            )?,
                        ),
                    ),
                )
            }
            "dec_statement" => {
                Ok(
                    Self::DecStatement(
                        ::std::boxed::Box::new(
                            <DecStatement as ::treesitter_types::FromNode>::from_node(
                                node,
                                src,
                            )?,
                        ),
                    ),
                )
            }
            "expression_statement" => {
                Ok(
                    Self::ExpressionStatement(
                        ::std::boxed::Box::new(
                            <ExpressionStatement as ::treesitter_types::FromNode>::from_node(
                                node,
                                src,
                            )?,
                        ),
                    ),
                )
            }
            "inc_statement" => {
                Ok(
                    Self::IncStatement(
                        ::std::boxed::Box::new(
                            <IncStatement as ::treesitter_types::FromNode>::from_node(
                                node,
                                src,
                            )?,
                        ),
                    ),
                )
            }
            "send_statement" => {
                Ok(
                    Self::SendStatement(
                        ::std::boxed::Box::new(
                            <SendStatement as ::treesitter_types::FromNode>::from_node(
                                node,
                                src,
                            )?,
                        ),
                    ),
                )
            }
            "short_var_declaration" => {
                Ok(
                    Self::ShortVarDeclaration(
                        ::std::boxed::Box::new(
                            <ShortVarDeclaration as ::treesitter_types::FromNode>::from_node(
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
impl ::treesitter_types::Spanned for SimpleStatement<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        match self {
            Self::AssignmentStatement(inner) => inner.span(),
            Self::DecStatement(inner) => inner.span(),
            Self::ExpressionStatement(inner) => inner.span(),
            Self::IncStatement(inner) => inner.span(),
            Self::SendStatement(inner) => inner.span(),
            Self::ShortVarDeclaration(inner) => inner.span(),
        }
    }
}
#[derive(Debug, Clone)]
pub enum SimpleType<'tree> {
    ArrayType(::std::boxed::Box<ArrayType<'tree>>),
    ChannelType(::std::boxed::Box<ChannelType<'tree>>),
    FunctionType(::std::boxed::Box<FunctionType<'tree>>),
    GenericType(::std::boxed::Box<GenericType<'tree>>),
    InterfaceType(::std::boxed::Box<InterfaceType<'tree>>),
    MapType(::std::boxed::Box<MapType<'tree>>),
    NegatedType(::std::boxed::Box<NegatedType<'tree>>),
    PointerType(::std::boxed::Box<PointerType<'tree>>),
    QualifiedType(::std::boxed::Box<QualifiedType<'tree>>),
    SliceType(::std::boxed::Box<SliceType<'tree>>),
    StructType(::std::boxed::Box<StructType<'tree>>),
    TypeIdentifier(::std::boxed::Box<TypeIdentifier<'tree>>),
}
impl<'tree> ::treesitter_types::FromNode<'tree> for SimpleType<'tree> {
    #[allow(clippy::collapsible_else_if)]
    fn from_node(
        node: ::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        match node.kind() {
            "array_type" => {
                Ok(
                    Self::ArrayType(
                        ::std::boxed::Box::new(
                            <ArrayType as ::treesitter_types::FromNode>::from_node(
                                node,
                                src,
                            )?,
                        ),
                    ),
                )
            }
            "channel_type" => {
                Ok(
                    Self::ChannelType(
                        ::std::boxed::Box::new(
                            <ChannelType as ::treesitter_types::FromNode>::from_node(
                                node,
                                src,
                            )?,
                        ),
                    ),
                )
            }
            "function_type" => {
                Ok(
                    Self::FunctionType(
                        ::std::boxed::Box::new(
                            <FunctionType as ::treesitter_types::FromNode>::from_node(
                                node,
                                src,
                            )?,
                        ),
                    ),
                )
            }
            "generic_type" => {
                Ok(
                    Self::GenericType(
                        ::std::boxed::Box::new(
                            <GenericType as ::treesitter_types::FromNode>::from_node(
                                node,
                                src,
                            )?,
                        ),
                    ),
                )
            }
            "interface_type" => {
                Ok(
                    Self::InterfaceType(
                        ::std::boxed::Box::new(
                            <InterfaceType as ::treesitter_types::FromNode>::from_node(
                                node,
                                src,
                            )?,
                        ),
                    ),
                )
            }
            "map_type" => {
                Ok(
                    Self::MapType(
                        ::std::boxed::Box::new(
                            <MapType as ::treesitter_types::FromNode>::from_node(
                                node,
                                src,
                            )?,
                        ),
                    ),
                )
            }
            "negated_type" => {
                Ok(
                    Self::NegatedType(
                        ::std::boxed::Box::new(
                            <NegatedType as ::treesitter_types::FromNode>::from_node(
                                node,
                                src,
                            )?,
                        ),
                    ),
                )
            }
            "pointer_type" => {
                Ok(
                    Self::PointerType(
                        ::std::boxed::Box::new(
                            <PointerType as ::treesitter_types::FromNode>::from_node(
                                node,
                                src,
                            )?,
                        ),
                    ),
                )
            }
            "qualified_type" => {
                Ok(
                    Self::QualifiedType(
                        ::std::boxed::Box::new(
                            <QualifiedType as ::treesitter_types::FromNode>::from_node(
                                node,
                                src,
                            )?,
                        ),
                    ),
                )
            }
            "slice_type" => {
                Ok(
                    Self::SliceType(
                        ::std::boxed::Box::new(
                            <SliceType as ::treesitter_types::FromNode>::from_node(
                                node,
                                src,
                            )?,
                        ),
                    ),
                )
            }
            "struct_type" => {
                Ok(
                    Self::StructType(
                        ::std::boxed::Box::new(
                            <StructType as ::treesitter_types::FromNode>::from_node(
                                node,
                                src,
                            )?,
                        ),
                    ),
                )
            }
            "type_identifier" => {
                Ok(
                    Self::TypeIdentifier(
                        ::std::boxed::Box::new(
                            <TypeIdentifier as ::treesitter_types::FromNode>::from_node(
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
impl ::treesitter_types::Spanned for SimpleType<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        match self {
            Self::ArrayType(inner) => inner.span(),
            Self::ChannelType(inner) => inner.span(),
            Self::FunctionType(inner) => inner.span(),
            Self::GenericType(inner) => inner.span(),
            Self::InterfaceType(inner) => inner.span(),
            Self::MapType(inner) => inner.span(),
            Self::NegatedType(inner) => inner.span(),
            Self::PointerType(inner) => inner.span(),
            Self::QualifiedType(inner) => inner.span(),
            Self::SliceType(inner) => inner.span(),
            Self::StructType(inner) => inner.span(),
            Self::TypeIdentifier(inner) => inner.span(),
        }
    }
}
#[derive(Debug, Clone)]
pub enum Statement<'tree> {
    SimpleStatement(::std::boxed::Box<SimpleStatement<'tree>>),
    Block(::std::boxed::Box<Block<'tree>>),
    BreakStatement(::std::boxed::Box<BreakStatement<'tree>>),
    ConstDeclaration(::std::boxed::Box<ConstDeclaration<'tree>>),
    ContinueStatement(::std::boxed::Box<ContinueStatement<'tree>>),
    DeferStatement(::std::boxed::Box<DeferStatement<'tree>>),
    EmptyStatement(::std::boxed::Box<EmptyStatement<'tree>>),
    ExpressionSwitchStatement(::std::boxed::Box<ExpressionSwitchStatement<'tree>>),
    FallthroughStatement(::std::boxed::Box<FallthroughStatement<'tree>>),
    ForStatement(::std::boxed::Box<ForStatement<'tree>>),
    GoStatement(::std::boxed::Box<GoStatement<'tree>>),
    GotoStatement(::std::boxed::Box<GotoStatement<'tree>>),
    IfStatement(::std::boxed::Box<IfStatement<'tree>>),
    LabeledStatement(::std::boxed::Box<LabeledStatement<'tree>>),
    ReturnStatement(::std::boxed::Box<ReturnStatement<'tree>>),
    SelectStatement(::std::boxed::Box<SelectStatement<'tree>>),
    TypeDeclaration(::std::boxed::Box<TypeDeclaration<'tree>>),
    TypeSwitchStatement(::std::boxed::Box<TypeSwitchStatement<'tree>>),
    VarDeclaration(::std::boxed::Box<VarDeclaration<'tree>>),
}
impl<'tree> ::treesitter_types::FromNode<'tree> for Statement<'tree> {
    #[allow(clippy::collapsible_else_if)]
    fn from_node(
        node: ::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        match node.kind() {
            "block" => {
                Ok(
                    Self::Block(
                        ::std::boxed::Box::new(
                            <Block as ::treesitter_types::FromNode>::from_node(
                                node,
                                src,
                            )?,
                        ),
                    ),
                )
            }
            "break_statement" => {
                Ok(
                    Self::BreakStatement(
                        ::std::boxed::Box::new(
                            <BreakStatement as ::treesitter_types::FromNode>::from_node(
                                node,
                                src,
                            )?,
                        ),
                    ),
                )
            }
            "const_declaration" => {
                Ok(
                    Self::ConstDeclaration(
                        ::std::boxed::Box::new(
                            <ConstDeclaration as ::treesitter_types::FromNode>::from_node(
                                node,
                                src,
                            )?,
                        ),
                    ),
                )
            }
            "continue_statement" => {
                Ok(
                    Self::ContinueStatement(
                        ::std::boxed::Box::new(
                            <ContinueStatement as ::treesitter_types::FromNode>::from_node(
                                node,
                                src,
                            )?,
                        ),
                    ),
                )
            }
            "defer_statement" => {
                Ok(
                    Self::DeferStatement(
                        ::std::boxed::Box::new(
                            <DeferStatement as ::treesitter_types::FromNode>::from_node(
                                node,
                                src,
                            )?,
                        ),
                    ),
                )
            }
            "empty_statement" => {
                Ok(
                    Self::EmptyStatement(
                        ::std::boxed::Box::new(
                            <EmptyStatement as ::treesitter_types::FromNode>::from_node(
                                node,
                                src,
                            )?,
                        ),
                    ),
                )
            }
            "expression_switch_statement" => {
                Ok(
                    Self::ExpressionSwitchStatement(
                        ::std::boxed::Box::new(
                            <ExpressionSwitchStatement as ::treesitter_types::FromNode>::from_node(
                                node,
                                src,
                            )?,
                        ),
                    ),
                )
            }
            "fallthrough_statement" => {
                Ok(
                    Self::FallthroughStatement(
                        ::std::boxed::Box::new(
                            <FallthroughStatement as ::treesitter_types::FromNode>::from_node(
                                node,
                                src,
                            )?,
                        ),
                    ),
                )
            }
            "for_statement" => {
                Ok(
                    Self::ForStatement(
                        ::std::boxed::Box::new(
                            <ForStatement as ::treesitter_types::FromNode>::from_node(
                                node,
                                src,
                            )?,
                        ),
                    ),
                )
            }
            "go_statement" => {
                Ok(
                    Self::GoStatement(
                        ::std::boxed::Box::new(
                            <GoStatement as ::treesitter_types::FromNode>::from_node(
                                node,
                                src,
                            )?,
                        ),
                    ),
                )
            }
            "goto_statement" => {
                Ok(
                    Self::GotoStatement(
                        ::std::boxed::Box::new(
                            <GotoStatement as ::treesitter_types::FromNode>::from_node(
                                node,
                                src,
                            )?,
                        ),
                    ),
                )
            }
            "if_statement" => {
                Ok(
                    Self::IfStatement(
                        ::std::boxed::Box::new(
                            <IfStatement as ::treesitter_types::FromNode>::from_node(
                                node,
                                src,
                            )?,
                        ),
                    ),
                )
            }
            "labeled_statement" => {
                Ok(
                    Self::LabeledStatement(
                        ::std::boxed::Box::new(
                            <LabeledStatement as ::treesitter_types::FromNode>::from_node(
                                node,
                                src,
                            )?,
                        ),
                    ),
                )
            }
            "return_statement" => {
                Ok(
                    Self::ReturnStatement(
                        ::std::boxed::Box::new(
                            <ReturnStatement as ::treesitter_types::FromNode>::from_node(
                                node,
                                src,
                            )?,
                        ),
                    ),
                )
            }
            "select_statement" => {
                Ok(
                    Self::SelectStatement(
                        ::std::boxed::Box::new(
                            <SelectStatement as ::treesitter_types::FromNode>::from_node(
                                node,
                                src,
                            )?,
                        ),
                    ),
                )
            }
            "type_declaration" => {
                Ok(
                    Self::TypeDeclaration(
                        ::std::boxed::Box::new(
                            <TypeDeclaration as ::treesitter_types::FromNode>::from_node(
                                node,
                                src,
                            )?,
                        ),
                    ),
                )
            }
            "type_switch_statement" => {
                Ok(
                    Self::TypeSwitchStatement(
                        ::std::boxed::Box::new(
                            <TypeSwitchStatement as ::treesitter_types::FromNode>::from_node(
                                node,
                                src,
                            )?,
                        ),
                    ),
                )
            }
            "var_declaration" => {
                Ok(
                    Self::VarDeclaration(
                        ::std::boxed::Box::new(
                            <VarDeclaration as ::treesitter_types::FromNode>::from_node(
                                node,
                                src,
                            )?,
                        ),
                    ),
                )
            }
            _other => {
                if let Ok(v) = <SimpleStatement as ::treesitter_types::FromNode>::from_node(
                    node,
                    src,
                ) {
                    Ok(Self::SimpleStatement(::std::boxed::Box::new(v)))
                } else {
                    Err(::treesitter_types::ParseError::unexpected_kind(_other, node))
                }
            }
        }
    }
}
impl ::treesitter_types::Spanned for Statement<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        match self {
            Self::SimpleStatement(inner) => inner.span(),
            Self::Block(inner) => inner.span(),
            Self::BreakStatement(inner) => inner.span(),
            Self::ConstDeclaration(inner) => inner.span(),
            Self::ContinueStatement(inner) => inner.span(),
            Self::DeferStatement(inner) => inner.span(),
            Self::EmptyStatement(inner) => inner.span(),
            Self::ExpressionSwitchStatement(inner) => inner.span(),
            Self::FallthroughStatement(inner) => inner.span(),
            Self::ForStatement(inner) => inner.span(),
            Self::GoStatement(inner) => inner.span(),
            Self::GotoStatement(inner) => inner.span(),
            Self::IfStatement(inner) => inner.span(),
            Self::LabeledStatement(inner) => inner.span(),
            Self::ReturnStatement(inner) => inner.span(),
            Self::SelectStatement(inner) => inner.span(),
            Self::TypeDeclaration(inner) => inner.span(),
            Self::TypeSwitchStatement(inner) => inner.span(),
            Self::VarDeclaration(inner) => inner.span(),
        }
    }
}
#[derive(Debug, Clone)]
pub enum Type<'tree> {
    SimpleType(::std::boxed::Box<SimpleType<'tree>>),
    ParenthesizedType(::std::boxed::Box<ParenthesizedType<'tree>>),
}
impl<'tree> ::treesitter_types::FromNode<'tree> for Type<'tree> {
    #[allow(clippy::collapsible_else_if)]
    fn from_node(
        node: ::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        match node.kind() {
            "parenthesized_type" => {
                Ok(
                    Self::ParenthesizedType(
                        ::std::boxed::Box::new(
                            <ParenthesizedType as ::treesitter_types::FromNode>::from_node(
                                node,
                                src,
                            )?,
                        ),
                    ),
                )
            }
            _other => {
                if let Ok(v) = <SimpleType as ::treesitter_types::FromNode>::from_node(
                    node,
                    src,
                ) {
                    Ok(Self::SimpleType(::std::boxed::Box::new(v)))
                } else {
                    Err(::treesitter_types::ParseError::unexpected_kind(_other, node))
                }
            }
        }
    }
}
impl ::treesitter_types::Spanned for Type<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        match self {
            Self::SimpleType(inner) => inner.span(),
            Self::ParenthesizedType(inner) => inner.span(),
        }
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
                            <ArgumentListChildren as ::treesitter_types::FromNode>::from_node(
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
impl ::treesitter_types::Spanned for ArgumentList<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        self.span
    }
}
#[derive(Debug, Clone)]
pub struct ArrayType<'tree> {
    pub span: ::treesitter_types::Span,
    pub element: Type<'tree>,
    pub length: Expression<'tree>,
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
            element: {
                let child = node
                    .child_by_field_name("element")
                    .ok_or_else(|| ::treesitter_types::ParseError::missing_field(
                        "element",
                        node,
                    ))?;
                <Type as ::treesitter_types::FromNode>::from_node(child, src)?
            },
            length: {
                let child = node
                    .child_by_field_name("length")
                    .ok_or_else(|| ::treesitter_types::ParseError::missing_field(
                        "length",
                        node,
                    ))?;
                <Expression as ::treesitter_types::FromNode>::from_node(child, src)?
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
pub struct AssignmentStatement<'tree> {
    pub span: ::treesitter_types::Span,
    pub left: ExpressionList<'tree>,
    pub operator: AssignmentStatementOperator,
    pub right: ExpressionList<'tree>,
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
            left: {
                let child = node
                    .child_by_field_name("left")
                    .ok_or_else(|| ::treesitter_types::ParseError::missing_field(
                        "left",
                        node,
                    ))?;
                <ExpressionList as ::treesitter_types::FromNode>::from_node(child, src)?
            },
            operator: {
                let child = node
                    .child_by_field_name("operator")
                    .ok_or_else(|| ::treesitter_types::ParseError::missing_field(
                        "operator",
                        node,
                    ))?;
                <AssignmentStatementOperator as ::treesitter_types::FromNode>::from_node(
                    child,
                    src,
                )?
            },
            right: {
                let child = node
                    .child_by_field_name("right")
                    .ok_or_else(|| ::treesitter_types::ParseError::missing_field(
                        "right",
                        node,
                    ))?;
                <ExpressionList as ::treesitter_types::FromNode>::from_node(child, src)?
            },
        })
    }
}
impl ::treesitter_types::Spanned for AssignmentStatement<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        self.span
    }
}
#[derive(Debug, Clone)]
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
                    .ok_or_else(|| ::treesitter_types::ParseError::missing_field(
                        "left",
                        node,
                    ))?;
                <Expression as ::treesitter_types::FromNode>::from_node(child, src)?
            },
            operator: {
                let child = node
                    .child_by_field_name("operator")
                    .ok_or_else(|| ::treesitter_types::ParseError::missing_field(
                        "operator",
                        node,
                    ))?;
                <BinaryExpressionOperator as ::treesitter_types::FromNode>::from_node(
                    child,
                    src,
                )?
            },
            right: {
                let child = node
                    .child_by_field_name("right")
                    .ok_or_else(|| ::treesitter_types::ParseError::missing_field(
                        "right",
                        node,
                    ))?;
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
#[derive(Debug, Clone)]
pub struct Block<'tree> {
    pub span: ::treesitter_types::Span,
    pub children: ::core::option::Option<StatementList<'tree>>,
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
                            <StatementList as ::treesitter_types::FromNode>::from_node(
                                child,
                                src,
                            )?,
                        )
                    }
                    None => {
                        let mut _result = None;
                        let mut _fc = node.walk();
                        if _fc.goto_first_child() {
                            loop {
                                if !_fc.node().is_extra() {
                                    let child = _fc.node();
                                    if let Ok(v) = (|| -> ::core::result::Result<
                                        _,
                                        ::treesitter_types::ParseError,
                                    > {
                                        Ok(
                                            Some(
                                                <StatementList as ::treesitter_types::FromNode>::from_node(
                                                    child,
                                                    src,
                                                )?,
                                            ),
                                        )
                                    })() {
                                        _result = Some(v);
                                        break;
                                    }
                                }
                                if !_fc.goto_next_sibling() {
                                    break;
                                }
                            }
                        }
                        _result.flatten()
                    }
                }
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
pub struct BreakStatement<'tree> {
    pub span: ::treesitter_types::Span,
    pub children: ::core::option::Option<LabelName<'tree>>,
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
                            <LabelName as ::treesitter_types::FromNode>::from_node(
                                child,
                                src,
                            )?,
                        )
                    }
                    None => {
                        let mut _result = None;
                        let mut _fc = node.walk();
                        if _fc.goto_first_child() {
                            loop {
                                if !_fc.node().is_extra() {
                                    let child = _fc.node();
                                    if let Ok(v) = (|| -> ::core::result::Result<
                                        _,
                                        ::treesitter_types::ParseError,
                                    > {
                                        Ok(
                                            Some(
                                                <LabelName as ::treesitter_types::FromNode>::from_node(
                                                    child,
                                                    src,
                                                )?,
                                            ),
                                        )
                                    })() {
                                        _result = Some(v);
                                        break;
                                    }
                                }
                                if !_fc.goto_next_sibling() {
                                    break;
                                }
                            }
                        }
                        _result.flatten()
                    }
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
pub struct CallExpression<'tree> {
    pub span: ::treesitter_types::Span,
    pub arguments: ArgumentList<'tree>,
    pub function: Expression<'tree>,
    pub type_arguments: ::core::option::Option<TypeArguments<'tree>>,
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
                let child = node
                    .child_by_field_name("arguments")
                    .ok_or_else(|| ::treesitter_types::ParseError::missing_field(
                        "arguments",
                        node,
                    ))?;
                <ArgumentList as ::treesitter_types::FromNode>::from_node(child, src)?
            },
            function: {
                let child = node
                    .child_by_field_name("function")
                    .ok_or_else(|| ::treesitter_types::ParseError::missing_field(
                        "function",
                        node,
                    ))?;
                <Expression as ::treesitter_types::FromNode>::from_node(child, src)?
            },
            type_arguments: match node.child_by_field_name("type_arguments") {
                Some(child) => {
                    Some(
                        <TypeArguments as ::treesitter_types::FromNode>::from_node(
                            child,
                            src,
                        )?,
                    )
                }
                None => None,
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
pub struct ChannelType<'tree> {
    pub span: ::treesitter_types::Span,
    pub value: Type<'tree>,
}
impl<'tree> ::treesitter_types::FromNode<'tree> for ChannelType<'tree> {
    #[allow(clippy::match_single_binding, clippy::suspicious_else_formatting)]
    fn from_node(
        node: ::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        debug_assert_eq!(node.kind(), "channel_type");
        Ok(Self {
            span: ::treesitter_types::Span::from(node),
            value: {
                let child = node
                    .child_by_field_name("value")
                    .ok_or_else(|| ::treesitter_types::ParseError::missing_field(
                        "value",
                        node,
                    ))?;
                <Type as ::treesitter_types::FromNode>::from_node(child, src)?
            },
        })
    }
}
impl ::treesitter_types::Spanned for ChannelType<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        self.span
    }
}
#[derive(Debug, Clone)]
pub struct CommunicationCase<'tree> {
    pub span: ::treesitter_types::Span,
    pub communication: CommunicationCaseCommunication<'tree>,
    pub children: ::core::option::Option<StatementList<'tree>>,
}
impl<'tree> ::treesitter_types::FromNode<'tree> for CommunicationCase<'tree> {
    #[allow(clippy::match_single_binding, clippy::suspicious_else_formatting)]
    fn from_node(
        node: ::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        debug_assert_eq!(node.kind(), "communication_case");
        Ok(Self {
            span: ::treesitter_types::Span::from(node),
            communication: {
                let child = node
                    .child_by_field_name("communication")
                    .ok_or_else(|| ::treesitter_types::ParseError::missing_field(
                        "communication",
                        node,
                    ))?;
                <CommunicationCaseCommunication as ::treesitter_types::FromNode>::from_node(
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
                            <StatementList as ::treesitter_types::FromNode>::from_node(
                                child,
                                src,
                            )?,
                        )
                    }
                    None => {
                        let mut _result = None;
                        let mut _fc = node.walk();
                        if _fc.goto_first_child() {
                            loop {
                                if !_fc.node().is_extra() {
                                    let child = _fc.node();
                                    if let Ok(v) = (|| -> ::core::result::Result<
                                        _,
                                        ::treesitter_types::ParseError,
                                    > {
                                        Ok(
                                            Some(
                                                <StatementList as ::treesitter_types::FromNode>::from_node(
                                                    child,
                                                    src,
                                                )?,
                                            ),
                                        )
                                    })() {
                                        _result = Some(v);
                                        break;
                                    }
                                }
                                if !_fc.goto_next_sibling() {
                                    break;
                                }
                            }
                        }
                        _result.flatten()
                    }
                }
            },
        })
    }
}
impl ::treesitter_types::Spanned for CommunicationCase<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        self.span
    }
}
#[derive(Debug, Clone)]
pub struct CompositeLiteral<'tree> {
    pub span: ::treesitter_types::Span,
    pub body: LiteralValue<'tree>,
    pub r#type: CompositeLiteralType<'tree>,
}
impl<'tree> ::treesitter_types::FromNode<'tree> for CompositeLiteral<'tree> {
    #[allow(clippy::match_single_binding, clippy::suspicious_else_formatting)]
    fn from_node(
        node: ::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        debug_assert_eq!(node.kind(), "composite_literal");
        Ok(Self {
            span: ::treesitter_types::Span::from(node),
            body: {
                let child = node
                    .child_by_field_name("body")
                    .ok_or_else(|| ::treesitter_types::ParseError::missing_field(
                        "body",
                        node,
                    ))?;
                <LiteralValue as ::treesitter_types::FromNode>::from_node(child, src)?
            },
            r#type: {
                let child = node
                    .child_by_field_name("type")
                    .ok_or_else(|| ::treesitter_types::ParseError::missing_field(
                        "type",
                        node,
                    ))?;
                <CompositeLiteralType as ::treesitter_types::FromNode>::from_node(
                    child,
                    src,
                )?
            },
        })
    }
}
impl ::treesitter_types::Spanned for CompositeLiteral<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        self.span
    }
}
#[derive(Debug, Clone)]
pub struct ConstDeclaration<'tree> {
    pub span: ::treesitter_types::Span,
    pub children: ::std::vec::Vec<ConstSpec<'tree>>,
}
impl<'tree> ::treesitter_types::FromNode<'tree> for ConstDeclaration<'tree> {
    #[allow(clippy::match_single_binding, clippy::suspicious_else_formatting)]
    fn from_node(
        node: ::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        debug_assert_eq!(node.kind(), "const_declaration");
        Ok(Self {
            span: ::treesitter_types::Span::from(node),
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
                            <ConstSpec as ::treesitter_types::FromNode>::from_node(
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
impl ::treesitter_types::Spanned for ConstDeclaration<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        self.span
    }
}
#[derive(Debug, Clone)]
pub struct ConstSpec<'tree> {
    pub span: ::treesitter_types::Span,
    pub name: ::std::vec::Vec<ConstSpecName<'tree>>,
    pub r#type: ::core::option::Option<Type<'tree>>,
    pub value: ::core::option::Option<ExpressionList<'tree>>,
}
impl<'tree> ::treesitter_types::FromNode<'tree> for ConstSpec<'tree> {
    #[allow(clippy::match_single_binding, clippy::suspicious_else_formatting)]
    fn from_node(
        node: ::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        debug_assert_eq!(node.kind(), "const_spec");
        Ok(Self {
            span: ::treesitter_types::Span::from(node),
            name: {
                let mut cursor = node.walk();
                let mut items = ::std::vec::Vec::new();
                for child in node.children_by_field_name("name", &mut cursor) {
                    items
                        .push(
                            <ConstSpecName as ::treesitter_types::FromNode>::from_node(
                                child,
                                src,
                            )?,
                        );
                }
                items
            },
            r#type: match node.child_by_field_name("type") {
                Some(child) => {
                    Some(<Type as ::treesitter_types::FromNode>::from_node(child, src)?)
                }
                None => None,
            },
            value: match node.child_by_field_name("value") {
                Some(child) => {
                    Some(
                        <ExpressionList as ::treesitter_types::FromNode>::from_node(
                            child,
                            src,
                        )?,
                    )
                }
                None => None,
            },
        })
    }
}
impl ::treesitter_types::Spanned for ConstSpec<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        self.span
    }
}
#[derive(Debug, Clone)]
pub struct ContinueStatement<'tree> {
    pub span: ::treesitter_types::Span,
    pub children: ::core::option::Option<LabelName<'tree>>,
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
                            <LabelName as ::treesitter_types::FromNode>::from_node(
                                child,
                                src,
                            )?,
                        )
                    }
                    None => {
                        let mut _result = None;
                        let mut _fc = node.walk();
                        if _fc.goto_first_child() {
                            loop {
                                if !_fc.node().is_extra() {
                                    let child = _fc.node();
                                    if let Ok(v) = (|| -> ::core::result::Result<
                                        _,
                                        ::treesitter_types::ParseError,
                                    > {
                                        Ok(
                                            Some(
                                                <LabelName as ::treesitter_types::FromNode>::from_node(
                                                    child,
                                                    src,
                                                )?,
                                            ),
                                        )
                                    })() {
                                        _result = Some(v);
                                        break;
                                    }
                                }
                                if !_fc.goto_next_sibling() {
                                    break;
                                }
                            }
                        }
                        _result.flatten()
                    }
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
pub struct DecStatement<'tree> {
    pub span: ::treesitter_types::Span,
    pub children: Expression<'tree>,
}
impl<'tree> ::treesitter_types::FromNode<'tree> for DecStatement<'tree> {
    #[allow(clippy::match_single_binding, clippy::suspicious_else_formatting)]
    fn from_node(
        node: ::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        debug_assert_eq!(node.kind(), "dec_statement");
        Ok(Self {
            span: ::treesitter_types::Span::from(node),
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
                            if !fallback_cursor.node().is_extra() {
                                let candidate = fallback_cursor.node();
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
                    fallback_child
                        .ok_or_else(|| ::treesitter_types::ParseError::missing_field(
                            "children",
                            node,
                        ))?
                };
                <Expression as ::treesitter_types::FromNode>::from_node(child, src)?
            },
        })
    }
}
impl ::treesitter_types::Spanned for DecStatement<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        self.span
    }
}
#[derive(Debug, Clone)]
pub struct DefaultCase<'tree> {
    pub span: ::treesitter_types::Span,
    pub children: ::core::option::Option<StatementList<'tree>>,
}
impl<'tree> ::treesitter_types::FromNode<'tree> for DefaultCase<'tree> {
    #[allow(clippy::match_single_binding, clippy::suspicious_else_formatting)]
    fn from_node(
        node: ::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        debug_assert_eq!(node.kind(), "default_case");
        Ok(Self {
            span: ::treesitter_types::Span::from(node),
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
                            <StatementList as ::treesitter_types::FromNode>::from_node(
                                child,
                                src,
                            )?,
                        )
                    }
                    None => {
                        let mut _result = None;
                        let mut _fc = node.walk();
                        if _fc.goto_first_child() {
                            loop {
                                if !_fc.node().is_extra() {
                                    let child = _fc.node();
                                    if let Ok(v) = (|| -> ::core::result::Result<
                                        _,
                                        ::treesitter_types::ParseError,
                                    > {
                                        Ok(
                                            Some(
                                                <StatementList as ::treesitter_types::FromNode>::from_node(
                                                    child,
                                                    src,
                                                )?,
                                            ),
                                        )
                                    })() {
                                        _result = Some(v);
                                        break;
                                    }
                                }
                                if !_fc.goto_next_sibling() {
                                    break;
                                }
                            }
                        }
                        _result.flatten()
                    }
                }
            },
        })
    }
}
impl ::treesitter_types::Spanned for DefaultCase<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        self.span
    }
}
#[derive(Debug, Clone)]
pub struct DeferStatement<'tree> {
    pub span: ::treesitter_types::Span,
    pub children: Expression<'tree>,
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
                            if !fallback_cursor.node().is_extra() {
                                let candidate = fallback_cursor.node();
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
                    fallback_child
                        .ok_or_else(|| ::treesitter_types::ParseError::missing_field(
                            "children",
                            node,
                        ))?
                };
                <Expression as ::treesitter_types::FromNode>::from_node(child, src)?
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
pub struct Dot<'tree> {
    pub span: ::treesitter_types::Span,
    text: &'tree str,
}
impl<'tree> ::treesitter_types::FromNode<'tree> for Dot<'tree> {
    fn from_node(
        node: ::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        debug_assert_eq!(node.kind(), "dot");
        Ok(Self {
            span: ::treesitter_types::Span::from(node),
            text: node.utf8_text(src)?,
        })
    }
}
impl<'tree> ::treesitter_types::LeafNode<'tree> for Dot<'tree> {
    fn text(&self) -> &'tree str {
        self.text
    }
}
impl ::treesitter_types::Spanned for Dot<'_> {
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
pub struct ExpressionCase<'tree> {
    pub span: ::treesitter_types::Span,
    pub value: ExpressionList<'tree>,
    pub children: ::core::option::Option<StatementList<'tree>>,
}
impl<'tree> ::treesitter_types::FromNode<'tree> for ExpressionCase<'tree> {
    #[allow(clippy::match_single_binding, clippy::suspicious_else_formatting)]
    fn from_node(
        node: ::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        debug_assert_eq!(node.kind(), "expression_case");
        Ok(Self {
            span: ::treesitter_types::Span::from(node),
            value: {
                let child = node
                    .child_by_field_name("value")
                    .ok_or_else(|| ::treesitter_types::ParseError::missing_field(
                        "value",
                        node,
                    ))?;
                <ExpressionList as ::treesitter_types::FromNode>::from_node(child, src)?
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
                            <StatementList as ::treesitter_types::FromNode>::from_node(
                                child,
                                src,
                            )?,
                        )
                    }
                    None => {
                        let mut _result = None;
                        let mut _fc = node.walk();
                        if _fc.goto_first_child() {
                            loop {
                                if !_fc.node().is_extra() {
                                    let child = _fc.node();
                                    if let Ok(v) = (|| -> ::core::result::Result<
                                        _,
                                        ::treesitter_types::ParseError,
                                    > {
                                        Ok(
                                            Some(
                                                <StatementList as ::treesitter_types::FromNode>::from_node(
                                                    child,
                                                    src,
                                                )?,
                                            ),
                                        )
                                    })() {
                                        _result = Some(v);
                                        break;
                                    }
                                }
                                if !_fc.goto_next_sibling() {
                                    break;
                                }
                            }
                        }
                        _result.flatten()
                    }
                }
            },
        })
    }
}
impl ::treesitter_types::Spanned for ExpressionCase<'_> {
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
                            <Expression as ::treesitter_types::FromNode>::from_node(
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
impl ::treesitter_types::Spanned for ExpressionList<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        self.span
    }
}
#[derive(Debug, Clone)]
pub struct ExpressionStatement<'tree> {
    pub span: ::treesitter_types::Span,
    pub children: Expression<'tree>,
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
                            if !fallback_cursor.node().is_extra() {
                                let candidate = fallback_cursor.node();
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
                    fallback_child
                        .ok_or_else(|| ::treesitter_types::ParseError::missing_field(
                            "children",
                            node,
                        ))?
                };
                <Expression as ::treesitter_types::FromNode>::from_node(child, src)?
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
pub struct ExpressionSwitchStatement<'tree> {
    pub span: ::treesitter_types::Span,
    pub initializer: ::core::option::Option<SimpleStatement<'tree>>,
    pub value: ::core::option::Option<Expression<'tree>>,
    pub children: ::std::vec::Vec<ExpressionSwitchStatementChildren<'tree>>,
}
impl<'tree> ::treesitter_types::FromNode<'tree> for ExpressionSwitchStatement<'tree> {
    #[allow(clippy::match_single_binding, clippy::suspicious_else_formatting)]
    fn from_node(
        node: ::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        debug_assert_eq!(node.kind(), "expression_switch_statement");
        Ok(Self {
            span: ::treesitter_types::Span::from(node),
            initializer: match node.child_by_field_name("initializer") {
                Some(child) => {
                    Some(
                        <SimpleStatement as ::treesitter_types::FromNode>::from_node(
                            child,
                            src,
                        )?,
                    )
                }
                None => None,
            },
            value: match node.child_by_field_name("value") {
                Some(child) => {
                    Some(
                        <Expression as ::treesitter_types::FromNode>::from_node(
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
                let mut items = ::std::vec::Vec::new();
                for child in non_field_children {
                    items
                        .push(
                            <ExpressionSwitchStatementChildren as ::treesitter_types::FromNode>::from_node(
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
impl ::treesitter_types::Spanned for ExpressionSwitchStatement<'_> {
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
pub struct FieldDeclaration<'tree> {
    pub span: ::treesitter_types::Span,
    pub name: ::std::vec::Vec<FieldIdentifier<'tree>>,
    pub tag: ::core::option::Option<FieldDeclarationTag<'tree>>,
    pub r#type: FieldDeclarationType<'tree>,
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
            name: {
                let mut cursor = node.walk();
                let mut items = ::std::vec::Vec::new();
                for child in node.children_by_field_name("name", &mut cursor) {
                    items
                        .push(
                            <FieldIdentifier as ::treesitter_types::FromNode>::from_node(
                                child,
                                src,
                            )?,
                        );
                }
                items
            },
            tag: match node.child_by_field_name("tag") {
                Some(child) => {
                    Some(
                        <FieldDeclarationTag as ::treesitter_types::FromNode>::from_node(
                            child,
                            src,
                        )?,
                    )
                }
                None => None,
            },
            r#type: {
                let child = node
                    .child_by_field_name("type")
                    .ok_or_else(|| ::treesitter_types::ParseError::missing_field(
                        "type",
                        node,
                    ))?;
                <FieldDeclarationType as ::treesitter_types::FromNode>::from_node(
                    child,
                    src,
                )?
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
    pub children: ::std::vec::Vec<FieldDeclaration<'tree>>,
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
                            <FieldDeclaration as ::treesitter_types::FromNode>::from_node(
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
impl ::treesitter_types::Spanned for FieldDeclarationList<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        self.span
    }
}
#[derive(Debug, Clone)]
pub struct ForClause<'tree> {
    pub span: ::treesitter_types::Span,
    pub condition: ::core::option::Option<Expression<'tree>>,
    pub initializer: ::core::option::Option<SimpleStatement<'tree>>,
    pub update: ::core::option::Option<SimpleStatement<'tree>>,
}
impl<'tree> ::treesitter_types::FromNode<'tree> for ForClause<'tree> {
    #[allow(clippy::match_single_binding, clippy::suspicious_else_formatting)]
    fn from_node(
        node: ::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        debug_assert_eq!(node.kind(), "for_clause");
        Ok(Self {
            span: ::treesitter_types::Span::from(node),
            condition: match node.child_by_field_name("condition") {
                Some(child) => {
                    Some(
                        <Expression as ::treesitter_types::FromNode>::from_node(
                            child,
                            src,
                        )?,
                    )
                }
                None => None,
            },
            initializer: match node.child_by_field_name("initializer") {
                Some(child) => {
                    Some(
                        <SimpleStatement as ::treesitter_types::FromNode>::from_node(
                            child,
                            src,
                        )?,
                    )
                }
                None => None,
            },
            update: match node.child_by_field_name("update") {
                Some(child) => {
                    Some(
                        <SimpleStatement as ::treesitter_types::FromNode>::from_node(
                            child,
                            src,
                        )?,
                    )
                }
                None => None,
            },
        })
    }
}
impl ::treesitter_types::Spanned for ForClause<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        self.span
    }
}
#[derive(Debug, Clone)]
pub struct ForStatement<'tree> {
    pub span: ::treesitter_types::Span,
    pub body: Block<'tree>,
    pub children: ::core::option::Option<ForStatementChildren<'tree>>,
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
                    .ok_or_else(|| ::treesitter_types::ParseError::missing_field(
                        "body",
                        node,
                    ))?;
                <Block as ::treesitter_types::FromNode>::from_node(child, src)?
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
                            <ForStatementChildren as ::treesitter_types::FromNode>::from_node(
                                child,
                                src,
                            )?,
                        )
                    }
                    None => {
                        let mut _result = None;
                        let mut _fc = node.walk();
                        if _fc.goto_first_child() {
                            loop {
                                if !_fc.node().is_extra() {
                                    let child = _fc.node();
                                    if let Ok(v) = (|| -> ::core::result::Result<
                                        _,
                                        ::treesitter_types::ParseError,
                                    > {
                                        Ok(
                                            Some(
                                                <ForStatementChildren as ::treesitter_types::FromNode>::from_node(
                                                    child,
                                                    src,
                                                )?,
                                            ),
                                        )
                                    })() {
                                        _result = Some(v);
                                        break;
                                    }
                                }
                                if !_fc.goto_next_sibling() {
                                    break;
                                }
                            }
                        }
                        _result.flatten()
                    }
                }
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
pub struct FuncLiteral<'tree> {
    pub span: ::treesitter_types::Span,
    pub body: Block<'tree>,
    pub parameters: ParameterList<'tree>,
    pub result: ::core::option::Option<FuncLiteralResult<'tree>>,
}
impl<'tree> ::treesitter_types::FromNode<'tree> for FuncLiteral<'tree> {
    #[allow(clippy::match_single_binding, clippy::suspicious_else_formatting)]
    fn from_node(
        node: ::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        debug_assert_eq!(node.kind(), "func_literal");
        Ok(Self {
            span: ::treesitter_types::Span::from(node),
            body: {
                let child = node
                    .child_by_field_name("body")
                    .ok_or_else(|| ::treesitter_types::ParseError::missing_field(
                        "body",
                        node,
                    ))?;
                <Block as ::treesitter_types::FromNode>::from_node(child, src)?
            },
            parameters: {
                let child = node
                    .child_by_field_name("parameters")
                    .ok_or_else(|| ::treesitter_types::ParseError::missing_field(
                        "parameters",
                        node,
                    ))?;
                <ParameterList as ::treesitter_types::FromNode>::from_node(child, src)?
            },
            result: match node.child_by_field_name("result") {
                Some(child) => {
                    Some(
                        <FuncLiteralResult as ::treesitter_types::FromNode>::from_node(
                            child,
                            src,
                        )?,
                    )
                }
                None => None,
            },
        })
    }
}
impl ::treesitter_types::Spanned for FuncLiteral<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        self.span
    }
}
#[derive(Debug, Clone)]
pub struct FunctionDeclaration<'tree> {
    pub span: ::treesitter_types::Span,
    pub body: ::core::option::Option<Block<'tree>>,
    pub name: Identifier<'tree>,
    pub parameters: ParameterList<'tree>,
    pub result: ::core::option::Option<FunctionDeclarationResult<'tree>>,
    pub type_parameters: ::core::option::Option<TypeParameterList<'tree>>,
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
                Some(child) => {
                    Some(<Block as ::treesitter_types::FromNode>::from_node(child, src)?)
                }
                None => None,
            },
            name: {
                let child = node
                    .child_by_field_name("name")
                    .ok_or_else(|| ::treesitter_types::ParseError::missing_field(
                        "name",
                        node,
                    ))?;
                <Identifier as ::treesitter_types::FromNode>::from_node(child, src)?
            },
            parameters: {
                let child = node
                    .child_by_field_name("parameters")
                    .ok_or_else(|| ::treesitter_types::ParseError::missing_field(
                        "parameters",
                        node,
                    ))?;
                <ParameterList as ::treesitter_types::FromNode>::from_node(child, src)?
            },
            result: match node.child_by_field_name("result") {
                Some(child) => {
                    Some(
                        <FunctionDeclarationResult as ::treesitter_types::FromNode>::from_node(
                            child,
                            src,
                        )?,
                    )
                }
                None => None,
            },
            type_parameters: match node.child_by_field_name("type_parameters") {
                Some(child) => {
                    Some(
                        <TypeParameterList as ::treesitter_types::FromNode>::from_node(
                            child,
                            src,
                        )?,
                    )
                }
                None => None,
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
pub struct FunctionType<'tree> {
    pub span: ::treesitter_types::Span,
    pub parameters: ParameterList<'tree>,
    pub result: ::core::option::Option<FunctionTypeResult<'tree>>,
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
            parameters: {
                let child = node
                    .child_by_field_name("parameters")
                    .ok_or_else(|| ::treesitter_types::ParseError::missing_field(
                        "parameters",
                        node,
                    ))?;
                <ParameterList as ::treesitter_types::FromNode>::from_node(child, src)?
            },
            result: match node.child_by_field_name("result") {
                Some(child) => {
                    Some(
                        <FunctionTypeResult as ::treesitter_types::FromNode>::from_node(
                            child,
                            src,
                        )?,
                    )
                }
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
                    .ok_or_else(|| ::treesitter_types::ParseError::missing_field(
                        "type",
                        node,
                    ))?;
                <GenericTypeType as ::treesitter_types::FromNode>::from_node(child, src)?
            },
            type_arguments: {
                let child = node
                    .child_by_field_name("type_arguments")
                    .ok_or_else(|| ::treesitter_types::ParseError::missing_field(
                        "type_arguments",
                        node,
                    ))?;
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
pub struct GoStatement<'tree> {
    pub span: ::treesitter_types::Span,
    pub children: Expression<'tree>,
}
impl<'tree> ::treesitter_types::FromNode<'tree> for GoStatement<'tree> {
    #[allow(clippy::match_single_binding, clippy::suspicious_else_formatting)]
    fn from_node(
        node: ::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        debug_assert_eq!(node.kind(), "go_statement");
        Ok(Self {
            span: ::treesitter_types::Span::from(node),
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
                            if !fallback_cursor.node().is_extra() {
                                let candidate = fallback_cursor.node();
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
                    fallback_child
                        .ok_or_else(|| ::treesitter_types::ParseError::missing_field(
                            "children",
                            node,
                        ))?
                };
                <Expression as ::treesitter_types::FromNode>::from_node(child, src)?
            },
        })
    }
}
impl ::treesitter_types::Spanned for GoStatement<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        self.span
    }
}
#[derive(Debug, Clone)]
pub struct GotoStatement<'tree> {
    pub span: ::treesitter_types::Span,
    pub children: LabelName<'tree>,
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
                            if !fallback_cursor.node().is_extra() {
                                let candidate = fallback_cursor.node();
                                if (|| -> ::core::result::Result<
                                    _,
                                    ::treesitter_types::ParseError,
                                > {
                                    let child = candidate;
                                    Ok(
                                        <LabelName as ::treesitter_types::FromNode>::from_node(
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
                    fallback_child
                        .ok_or_else(|| ::treesitter_types::ParseError::missing_field(
                            "children",
                            node,
                        ))?
                };
                <LabelName as ::treesitter_types::FromNode>::from_node(child, src)?
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
    pub alternative: ::core::option::Option<IfStatementAlternative<'tree>>,
    pub condition: Expression<'tree>,
    pub consequence: Block<'tree>,
    pub initializer: ::core::option::Option<SimpleStatement<'tree>>,
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
                Some(child) => {
                    Some(
                        <IfStatementAlternative as ::treesitter_types::FromNode>::from_node(
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
                <Expression as ::treesitter_types::FromNode>::from_node(child, src)?
            },
            consequence: {
                let child = node
                    .child_by_field_name("consequence")
                    .ok_or_else(|| ::treesitter_types::ParseError::missing_field(
                        "consequence",
                        node,
                    ))?;
                <Block as ::treesitter_types::FromNode>::from_node(child, src)?
            },
            initializer: match node.child_by_field_name("initializer") {
                Some(child) => {
                    Some(
                        <SimpleStatement as ::treesitter_types::FromNode>::from_node(
                            child,
                            src,
                        )?,
                    )
                }
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
#[derive(Debug, Clone)]
pub struct ImplicitLengthArrayType<'tree> {
    pub span: ::treesitter_types::Span,
    pub element: Type<'tree>,
}
impl<'tree> ::treesitter_types::FromNode<'tree> for ImplicitLengthArrayType<'tree> {
    #[allow(clippy::match_single_binding, clippy::suspicious_else_formatting)]
    fn from_node(
        node: ::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        debug_assert_eq!(node.kind(), "implicit_length_array_type");
        Ok(Self {
            span: ::treesitter_types::Span::from(node),
            element: {
                let child = node
                    .child_by_field_name("element")
                    .ok_or_else(|| ::treesitter_types::ParseError::missing_field(
                        "element",
                        node,
                    ))?;
                <Type as ::treesitter_types::FromNode>::from_node(child, src)?
            },
        })
    }
}
impl ::treesitter_types::Spanned for ImplicitLengthArrayType<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        self.span
    }
}
#[derive(Debug, Clone)]
pub struct ImportDeclaration<'tree> {
    pub span: ::treesitter_types::Span,
    pub children: ImportDeclarationChildren<'tree>,
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
                            if !fallback_cursor.node().is_extra() {
                                let candidate = fallback_cursor.node();
                                if (|| -> ::core::result::Result<
                                    _,
                                    ::treesitter_types::ParseError,
                                > {
                                    let child = candidate;
                                    Ok(
                                        <ImportDeclarationChildren as ::treesitter_types::FromNode>::from_node(
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
                    fallback_child
                        .ok_or_else(|| ::treesitter_types::ParseError::missing_field(
                            "children",
                            node,
                        ))?
                };
                <ImportDeclarationChildren as ::treesitter_types::FromNode>::from_node(
                    child,
                    src,
                )?
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
pub struct ImportSpec<'tree> {
    pub span: ::treesitter_types::Span,
    pub name: ::core::option::Option<ImportSpecName<'tree>>,
    pub path: ImportSpecPath<'tree>,
}
impl<'tree> ::treesitter_types::FromNode<'tree> for ImportSpec<'tree> {
    #[allow(clippy::match_single_binding, clippy::suspicious_else_formatting)]
    fn from_node(
        node: ::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        debug_assert_eq!(node.kind(), "import_spec");
        Ok(Self {
            span: ::treesitter_types::Span::from(node),
            name: match node.child_by_field_name("name") {
                Some(child) => {
                    Some(
                        <ImportSpecName as ::treesitter_types::FromNode>::from_node(
                            child,
                            src,
                        )?,
                    )
                }
                None => None,
            },
            path: {
                let child = node
                    .child_by_field_name("path")
                    .ok_or_else(|| ::treesitter_types::ParseError::missing_field(
                        "path",
                        node,
                    ))?;
                <ImportSpecPath as ::treesitter_types::FromNode>::from_node(child, src)?
            },
        })
    }
}
impl ::treesitter_types::Spanned for ImportSpec<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        self.span
    }
}
#[derive(Debug, Clone)]
pub struct ImportSpecList<'tree> {
    pub span: ::treesitter_types::Span,
    pub children: ::std::vec::Vec<ImportSpec<'tree>>,
}
impl<'tree> ::treesitter_types::FromNode<'tree> for ImportSpecList<'tree> {
    #[allow(clippy::match_single_binding, clippy::suspicious_else_formatting)]
    fn from_node(
        node: ::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        debug_assert_eq!(node.kind(), "import_spec_list");
        Ok(Self {
            span: ::treesitter_types::Span::from(node),
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
                            <ImportSpec as ::treesitter_types::FromNode>::from_node(
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
impl ::treesitter_types::Spanned for ImportSpecList<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        self.span
    }
}
#[derive(Debug, Clone)]
pub struct IncStatement<'tree> {
    pub span: ::treesitter_types::Span,
    pub children: Expression<'tree>,
}
impl<'tree> ::treesitter_types::FromNode<'tree> for IncStatement<'tree> {
    #[allow(clippy::match_single_binding, clippy::suspicious_else_formatting)]
    fn from_node(
        node: ::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        debug_assert_eq!(node.kind(), "inc_statement");
        Ok(Self {
            span: ::treesitter_types::Span::from(node),
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
                            if !fallback_cursor.node().is_extra() {
                                let candidate = fallback_cursor.node();
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
                    fallback_child
                        .ok_or_else(|| ::treesitter_types::ParseError::missing_field(
                            "children",
                            node,
                        ))?
                };
                <Expression as ::treesitter_types::FromNode>::from_node(child, src)?
            },
        })
    }
}
impl ::treesitter_types::Spanned for IncStatement<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        self.span
    }
}
#[derive(Debug, Clone)]
pub struct IndexExpression<'tree> {
    pub span: ::treesitter_types::Span,
    pub index: Expression<'tree>,
    pub operand: Expression<'tree>,
}
impl<'tree> ::treesitter_types::FromNode<'tree> for IndexExpression<'tree> {
    #[allow(clippy::match_single_binding, clippy::suspicious_else_formatting)]
    fn from_node(
        node: ::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        debug_assert_eq!(node.kind(), "index_expression");
        Ok(Self {
            span: ::treesitter_types::Span::from(node),
            index: {
                let child = node
                    .child_by_field_name("index")
                    .ok_or_else(|| ::treesitter_types::ParseError::missing_field(
                        "index",
                        node,
                    ))?;
                <Expression as ::treesitter_types::FromNode>::from_node(child, src)?
            },
            operand: {
                let child = node
                    .child_by_field_name("operand")
                    .ok_or_else(|| ::treesitter_types::ParseError::missing_field(
                        "operand",
                        node,
                    ))?;
                <Expression as ::treesitter_types::FromNode>::from_node(child, src)?
            },
        })
    }
}
impl ::treesitter_types::Spanned for IndexExpression<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        self.span
    }
}
#[derive(Debug, Clone)]
pub struct InterfaceType<'tree> {
    pub span: ::treesitter_types::Span,
    pub children: ::std::vec::Vec<InterfaceTypeChildren<'tree>>,
}
impl<'tree> ::treesitter_types::FromNode<'tree> for InterfaceType<'tree> {
    #[allow(clippy::match_single_binding, clippy::suspicious_else_formatting)]
    fn from_node(
        node: ::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        debug_assert_eq!(node.kind(), "interface_type");
        Ok(Self {
            span: ::treesitter_types::Span::from(node),
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
                            <InterfaceTypeChildren as ::treesitter_types::FromNode>::from_node(
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
impl ::treesitter_types::Spanned for InterfaceType<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        self.span
    }
}
#[derive(Debug, Clone)]
pub struct InterpretedStringLiteral<'tree> {
    pub span: ::treesitter_types::Span,
    pub children: ::std::vec::Vec<InterpretedStringLiteralChildren<'tree>>,
}
impl<'tree> ::treesitter_types::FromNode<'tree> for InterpretedStringLiteral<'tree> {
    #[allow(clippy::match_single_binding, clippy::suspicious_else_formatting)]
    fn from_node(
        node: ::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        debug_assert_eq!(node.kind(), "interpreted_string_literal");
        Ok(Self {
            span: ::treesitter_types::Span::from(node),
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
                            <InterpretedStringLiteralChildren as ::treesitter_types::FromNode>::from_node(
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
impl ::treesitter_types::Spanned for InterpretedStringLiteral<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        self.span
    }
}
#[derive(Debug, Clone)]
pub struct KeyedElement<'tree> {
    pub span: ::treesitter_types::Span,
    pub key: LiteralElement<'tree>,
    pub value: LiteralElement<'tree>,
}
impl<'tree> ::treesitter_types::FromNode<'tree> for KeyedElement<'tree> {
    #[allow(clippy::match_single_binding, clippy::suspicious_else_formatting)]
    fn from_node(
        node: ::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        debug_assert_eq!(node.kind(), "keyed_element");
        Ok(Self {
            span: ::treesitter_types::Span::from(node),
            key: {
                let child = node
                    .child_by_field_name("key")
                    .ok_or_else(|| ::treesitter_types::ParseError::missing_field(
                        "key",
                        node,
                    ))?;
                <LiteralElement as ::treesitter_types::FromNode>::from_node(child, src)?
            },
            value: {
                let child = node
                    .child_by_field_name("value")
                    .ok_or_else(|| ::treesitter_types::ParseError::missing_field(
                        "value",
                        node,
                    ))?;
                <LiteralElement as ::treesitter_types::FromNode>::from_node(child, src)?
            },
        })
    }
}
impl ::treesitter_types::Spanned for KeyedElement<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        self.span
    }
}
#[derive(Debug, Clone)]
pub struct LabeledStatement<'tree> {
    pub span: ::treesitter_types::Span,
    pub label: LabelName<'tree>,
    pub children: ::core::option::Option<Statement<'tree>>,
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
                    .ok_or_else(|| ::treesitter_types::ParseError::missing_field(
                        "label",
                        node,
                    ))?;
                <LabelName as ::treesitter_types::FromNode>::from_node(child, src)?
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
                            <Statement as ::treesitter_types::FromNode>::from_node(
                                child,
                                src,
                            )?,
                        )
                    }
                    None => {
                        let mut _result = None;
                        let mut _fc = node.walk();
                        if _fc.goto_first_child() {
                            loop {
                                if !_fc.node().is_extra() {
                                    let child = _fc.node();
                                    if let Ok(v) = (|| -> ::core::result::Result<
                                        _,
                                        ::treesitter_types::ParseError,
                                    > {
                                        Ok(
                                            Some(
                                                <Statement as ::treesitter_types::FromNode>::from_node(
                                                    child,
                                                    src,
                                                )?,
                                            ),
                                        )
                                    })() {
                                        _result = Some(v);
                                        break;
                                    }
                                }
                                if !_fc.goto_next_sibling() {
                                    break;
                                }
                            }
                        }
                        _result.flatten()
                    }
                }
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
pub struct LiteralElement<'tree> {
    pub span: ::treesitter_types::Span,
    pub children: LiteralElementChildren<'tree>,
}
impl<'tree> ::treesitter_types::FromNode<'tree> for LiteralElement<'tree> {
    #[allow(clippy::match_single_binding, clippy::suspicious_else_formatting)]
    fn from_node(
        node: ::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        debug_assert_eq!(node.kind(), "literal_element");
        Ok(Self {
            span: ::treesitter_types::Span::from(node),
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
                            if !fallback_cursor.node().is_extra() {
                                let candidate = fallback_cursor.node();
                                if (|| -> ::core::result::Result<
                                    _,
                                    ::treesitter_types::ParseError,
                                > {
                                    let child = candidate;
                                    Ok(
                                        <LiteralElementChildren as ::treesitter_types::FromNode>::from_node(
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
                    fallback_child
                        .ok_or_else(|| ::treesitter_types::ParseError::missing_field(
                            "children",
                            node,
                        ))?
                };
                <LiteralElementChildren as ::treesitter_types::FromNode>::from_node(
                    child,
                    src,
                )?
            },
        })
    }
}
impl ::treesitter_types::Spanned for LiteralElement<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        self.span
    }
}
#[derive(Debug, Clone)]
pub struct LiteralValue<'tree> {
    pub span: ::treesitter_types::Span,
    pub children: ::std::vec::Vec<LiteralValueChildren<'tree>>,
}
impl<'tree> ::treesitter_types::FromNode<'tree> for LiteralValue<'tree> {
    #[allow(clippy::match_single_binding, clippy::suspicious_else_formatting)]
    fn from_node(
        node: ::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        debug_assert_eq!(node.kind(), "literal_value");
        Ok(Self {
            span: ::treesitter_types::Span::from(node),
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
                            <LiteralValueChildren as ::treesitter_types::FromNode>::from_node(
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
impl ::treesitter_types::Spanned for LiteralValue<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        self.span
    }
}
#[derive(Debug, Clone)]
pub struct MapType<'tree> {
    pub span: ::treesitter_types::Span,
    pub key: Type<'tree>,
    pub value: Type<'tree>,
}
impl<'tree> ::treesitter_types::FromNode<'tree> for MapType<'tree> {
    #[allow(clippy::match_single_binding, clippy::suspicious_else_formatting)]
    fn from_node(
        node: ::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        debug_assert_eq!(node.kind(), "map_type");
        Ok(Self {
            span: ::treesitter_types::Span::from(node),
            key: {
                let child = node
                    .child_by_field_name("key")
                    .ok_or_else(|| ::treesitter_types::ParseError::missing_field(
                        "key",
                        node,
                    ))?;
                <Type as ::treesitter_types::FromNode>::from_node(child, src)?
            },
            value: {
                let child = node
                    .child_by_field_name("value")
                    .ok_or_else(|| ::treesitter_types::ParseError::missing_field(
                        "value",
                        node,
                    ))?;
                <Type as ::treesitter_types::FromNode>::from_node(child, src)?
            },
        })
    }
}
impl ::treesitter_types::Spanned for MapType<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        self.span
    }
}
#[derive(Debug, Clone)]
pub struct MethodDeclaration<'tree> {
    pub span: ::treesitter_types::Span,
    pub body: ::core::option::Option<Block<'tree>>,
    pub name: FieldIdentifier<'tree>,
    pub parameters: ParameterList<'tree>,
    pub receiver: ParameterList<'tree>,
    pub result: ::core::option::Option<MethodDeclarationResult<'tree>>,
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
                Some(child) => {
                    Some(<Block as ::treesitter_types::FromNode>::from_node(child, src)?)
                }
                None => None,
            },
            name: {
                let child = node
                    .child_by_field_name("name")
                    .ok_or_else(|| ::treesitter_types::ParseError::missing_field(
                        "name",
                        node,
                    ))?;
                <FieldIdentifier as ::treesitter_types::FromNode>::from_node(child, src)?
            },
            parameters: {
                let child = node
                    .child_by_field_name("parameters")
                    .ok_or_else(|| ::treesitter_types::ParseError::missing_field(
                        "parameters",
                        node,
                    ))?;
                <ParameterList as ::treesitter_types::FromNode>::from_node(child, src)?
            },
            receiver: {
                let child = node
                    .child_by_field_name("receiver")
                    .ok_or_else(|| ::treesitter_types::ParseError::missing_field(
                        "receiver",
                        node,
                    ))?;
                <ParameterList as ::treesitter_types::FromNode>::from_node(child, src)?
            },
            result: match node.child_by_field_name("result") {
                Some(child) => {
                    Some(
                        <MethodDeclarationResult as ::treesitter_types::FromNode>::from_node(
                            child,
                            src,
                        )?,
                    )
                }
                None => None,
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
pub struct MethodElem<'tree> {
    pub span: ::treesitter_types::Span,
    pub name: FieldIdentifier<'tree>,
    pub parameters: ParameterList<'tree>,
    pub result: ::core::option::Option<MethodElemResult<'tree>>,
}
impl<'tree> ::treesitter_types::FromNode<'tree> for MethodElem<'tree> {
    #[allow(clippy::match_single_binding, clippy::suspicious_else_formatting)]
    fn from_node(
        node: ::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        debug_assert_eq!(node.kind(), "method_elem");
        Ok(Self {
            span: ::treesitter_types::Span::from(node),
            name: {
                let child = node
                    .child_by_field_name("name")
                    .ok_or_else(|| ::treesitter_types::ParseError::missing_field(
                        "name",
                        node,
                    ))?;
                <FieldIdentifier as ::treesitter_types::FromNode>::from_node(child, src)?
            },
            parameters: {
                let child = node
                    .child_by_field_name("parameters")
                    .ok_or_else(|| ::treesitter_types::ParseError::missing_field(
                        "parameters",
                        node,
                    ))?;
                <ParameterList as ::treesitter_types::FromNode>::from_node(child, src)?
            },
            result: match node.child_by_field_name("result") {
                Some(child) => {
                    Some(
                        <MethodElemResult as ::treesitter_types::FromNode>::from_node(
                            child,
                            src,
                        )?,
                    )
                }
                None => None,
            },
        })
    }
}
impl ::treesitter_types::Spanned for MethodElem<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        self.span
    }
}
#[derive(Debug, Clone)]
pub struct NegatedType<'tree> {
    pub span: ::treesitter_types::Span,
    pub children: Type<'tree>,
}
impl<'tree> ::treesitter_types::FromNode<'tree> for NegatedType<'tree> {
    #[allow(clippy::match_single_binding, clippy::suspicious_else_formatting)]
    fn from_node(
        node: ::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        debug_assert_eq!(node.kind(), "negated_type");
        Ok(Self {
            span: ::treesitter_types::Span::from(node),
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
                            if !fallback_cursor.node().is_extra() {
                                let candidate = fallback_cursor.node();
                                if (|| -> ::core::result::Result<
                                    _,
                                    ::treesitter_types::ParseError,
                                > {
                                    let child = candidate;
                                    Ok(
                                        <Type as ::treesitter_types::FromNode>::from_node(
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
                    fallback_child
                        .ok_or_else(|| ::treesitter_types::ParseError::missing_field(
                            "children",
                            node,
                        ))?
                };
                <Type as ::treesitter_types::FromNode>::from_node(child, src)?
            },
        })
    }
}
impl ::treesitter_types::Spanned for NegatedType<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        self.span
    }
}
#[derive(Debug, Clone)]
pub struct PackageClause<'tree> {
    pub span: ::treesitter_types::Span,
    pub children: PackageIdentifier<'tree>,
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
                            if !fallback_cursor.node().is_extra() {
                                let candidate = fallback_cursor.node();
                                if (|| -> ::core::result::Result<
                                    _,
                                    ::treesitter_types::ParseError,
                                > {
                                    let child = candidate;
                                    Ok(
                                        <PackageIdentifier as ::treesitter_types::FromNode>::from_node(
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
                    fallback_child
                        .ok_or_else(|| ::treesitter_types::ParseError::missing_field(
                            "children",
                            node,
                        ))?
                };
                <PackageIdentifier as ::treesitter_types::FromNode>::from_node(
                    child,
                    src,
                )?
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
pub struct ParameterDeclaration<'tree> {
    pub span: ::treesitter_types::Span,
    pub name: ::std::vec::Vec<Identifier<'tree>>,
    pub r#type: Type<'tree>,
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
            name: {
                let mut cursor = node.walk();
                let mut items = ::std::vec::Vec::new();
                for child in node.children_by_field_name("name", &mut cursor) {
                    items
                        .push(
                            <Identifier as ::treesitter_types::FromNode>::from_node(
                                child,
                                src,
                            )?,
                        );
                }
                items
            },
            r#type: {
                let child = node
                    .child_by_field_name("type")
                    .ok_or_else(|| ::treesitter_types::ParseError::missing_field(
                        "type",
                        node,
                    ))?;
                <Type as ::treesitter_types::FromNode>::from_node(child, src)?
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
                            <ParameterListChildren as ::treesitter_types::FromNode>::from_node(
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
impl ::treesitter_types::Spanned for ParameterList<'_> {
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
                            if !fallback_cursor.node().is_extra() {
                                let candidate = fallback_cursor.node();
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
                    fallback_child
                        .ok_or_else(|| ::treesitter_types::ParseError::missing_field(
                            "children",
                            node,
                        ))?
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
pub struct ParenthesizedType<'tree> {
    pub span: ::treesitter_types::Span,
    pub children: Type<'tree>,
}
impl<'tree> ::treesitter_types::FromNode<'tree> for ParenthesizedType<'tree> {
    #[allow(clippy::match_single_binding, clippy::suspicious_else_formatting)]
    fn from_node(
        node: ::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        debug_assert_eq!(node.kind(), "parenthesized_type");
        Ok(Self {
            span: ::treesitter_types::Span::from(node),
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
                            if !fallback_cursor.node().is_extra() {
                                let candidate = fallback_cursor.node();
                                if (|| -> ::core::result::Result<
                                    _,
                                    ::treesitter_types::ParseError,
                                > {
                                    let child = candidate;
                                    Ok(
                                        <Type as ::treesitter_types::FromNode>::from_node(
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
                    fallback_child
                        .ok_or_else(|| ::treesitter_types::ParseError::missing_field(
                            "children",
                            node,
                        ))?
                };
                <Type as ::treesitter_types::FromNode>::from_node(child, src)?
            },
        })
    }
}
impl ::treesitter_types::Spanned for ParenthesizedType<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        self.span
    }
}
#[derive(Debug, Clone)]
pub struct PointerType<'tree> {
    pub span: ::treesitter_types::Span,
    pub children: Type<'tree>,
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
                            if !fallback_cursor.node().is_extra() {
                                let candidate = fallback_cursor.node();
                                if (|| -> ::core::result::Result<
                                    _,
                                    ::treesitter_types::ParseError,
                                > {
                                    let child = candidate;
                                    Ok(
                                        <Type as ::treesitter_types::FromNode>::from_node(
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
                    fallback_child
                        .ok_or_else(|| ::treesitter_types::ParseError::missing_field(
                            "children",
                            node,
                        ))?
                };
                <Type as ::treesitter_types::FromNode>::from_node(child, src)?
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
pub struct QualifiedType<'tree> {
    pub span: ::treesitter_types::Span,
    pub name: TypeIdentifier<'tree>,
    pub package: PackageIdentifier<'tree>,
}
impl<'tree> ::treesitter_types::FromNode<'tree> for QualifiedType<'tree> {
    #[allow(clippy::match_single_binding, clippy::suspicious_else_formatting)]
    fn from_node(
        node: ::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        debug_assert_eq!(node.kind(), "qualified_type");
        Ok(Self {
            span: ::treesitter_types::Span::from(node),
            name: {
                let child = node
                    .child_by_field_name("name")
                    .ok_or_else(|| ::treesitter_types::ParseError::missing_field(
                        "name",
                        node,
                    ))?;
                <TypeIdentifier as ::treesitter_types::FromNode>::from_node(child, src)?
            },
            package: {
                let child = node
                    .child_by_field_name("package")
                    .ok_or_else(|| ::treesitter_types::ParseError::missing_field(
                        "package",
                        node,
                    ))?;
                <PackageIdentifier as ::treesitter_types::FromNode>::from_node(
                    child,
                    src,
                )?
            },
        })
    }
}
impl ::treesitter_types::Spanned for QualifiedType<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        self.span
    }
}
#[derive(Debug, Clone)]
pub struct RangeClause<'tree> {
    pub span: ::treesitter_types::Span,
    pub left: ::core::option::Option<ExpressionList<'tree>>,
    pub right: Expression<'tree>,
}
impl<'tree> ::treesitter_types::FromNode<'tree> for RangeClause<'tree> {
    #[allow(clippy::match_single_binding, clippy::suspicious_else_formatting)]
    fn from_node(
        node: ::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        debug_assert_eq!(node.kind(), "range_clause");
        Ok(Self {
            span: ::treesitter_types::Span::from(node),
            left: match node.child_by_field_name("left") {
                Some(child) => {
                    Some(
                        <ExpressionList as ::treesitter_types::FromNode>::from_node(
                            child,
                            src,
                        )?,
                    )
                }
                None => None,
            },
            right: {
                let child = node
                    .child_by_field_name("right")
                    .ok_or_else(|| ::treesitter_types::ParseError::missing_field(
                        "right",
                        node,
                    ))?;
                <Expression as ::treesitter_types::FromNode>::from_node(child, src)?
            },
        })
    }
}
impl ::treesitter_types::Spanned for RangeClause<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        self.span
    }
}
#[derive(Debug, Clone)]
pub struct RawStringLiteral<'tree> {
    pub span: ::treesitter_types::Span,
    pub children: RawStringLiteralContent<'tree>,
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
                            if !fallback_cursor.node().is_extra() {
                                let candidate = fallback_cursor.node();
                                if (|| -> ::core::result::Result<
                                    _,
                                    ::treesitter_types::ParseError,
                                > {
                                    let child = candidate;
                                    Ok(
                                        <RawStringLiteralContent as ::treesitter_types::FromNode>::from_node(
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
                    fallback_child
                        .ok_or_else(|| ::treesitter_types::ParseError::missing_field(
                            "children",
                            node,
                        ))?
                };
                <RawStringLiteralContent as ::treesitter_types::FromNode>::from_node(
                    child,
                    src,
                )?
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
pub struct ReceiveStatement<'tree> {
    pub span: ::treesitter_types::Span,
    pub left: ::core::option::Option<ExpressionList<'tree>>,
    pub right: Expression<'tree>,
}
impl<'tree> ::treesitter_types::FromNode<'tree> for ReceiveStatement<'tree> {
    #[allow(clippy::match_single_binding, clippy::suspicious_else_formatting)]
    fn from_node(
        node: ::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        debug_assert_eq!(node.kind(), "receive_statement");
        Ok(Self {
            span: ::treesitter_types::Span::from(node),
            left: match node.child_by_field_name("left") {
                Some(child) => {
                    Some(
                        <ExpressionList as ::treesitter_types::FromNode>::from_node(
                            child,
                            src,
                        )?,
                    )
                }
                None => None,
            },
            right: {
                let child = node
                    .child_by_field_name("right")
                    .ok_or_else(|| ::treesitter_types::ParseError::missing_field(
                        "right",
                        node,
                    ))?;
                <Expression as ::treesitter_types::FromNode>::from_node(child, src)?
            },
        })
    }
}
impl ::treesitter_types::Spanned for ReceiveStatement<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        self.span
    }
}
#[derive(Debug, Clone)]
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
                            <ExpressionList as ::treesitter_types::FromNode>::from_node(
                                child,
                                src,
                            )?,
                        )
                    }
                    None => {
                        let mut _result = None;
                        let mut _fc = node.walk();
                        if _fc.goto_first_child() {
                            loop {
                                if !_fc.node().is_extra() {
                                    let child = _fc.node();
                                    if let Ok(v) = (|| -> ::core::result::Result<
                                        _,
                                        ::treesitter_types::ParseError,
                                    > {
                                        Ok(
                                            Some(
                                                <ExpressionList as ::treesitter_types::FromNode>::from_node(
                                                    child,
                                                    src,
                                                )?,
                                            ),
                                        )
                                    })() {
                                        _result = Some(v);
                                        break;
                                    }
                                }
                                if !_fc.goto_next_sibling() {
                                    break;
                                }
                            }
                        }
                        _result.flatten()
                    }
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
pub struct SelectStatement<'tree> {
    pub span: ::treesitter_types::Span,
    pub children: ::std::vec::Vec<SelectStatementChildren<'tree>>,
}
impl<'tree> ::treesitter_types::FromNode<'tree> for SelectStatement<'tree> {
    #[allow(clippy::match_single_binding, clippy::suspicious_else_formatting)]
    fn from_node(
        node: ::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        debug_assert_eq!(node.kind(), "select_statement");
        Ok(Self {
            span: ::treesitter_types::Span::from(node),
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
                            <SelectStatementChildren as ::treesitter_types::FromNode>::from_node(
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
impl ::treesitter_types::Spanned for SelectStatement<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        self.span
    }
}
#[derive(Debug, Clone)]
pub struct SelectorExpression<'tree> {
    pub span: ::treesitter_types::Span,
    pub field: FieldIdentifier<'tree>,
    pub operand: Expression<'tree>,
}
impl<'tree> ::treesitter_types::FromNode<'tree> for SelectorExpression<'tree> {
    #[allow(clippy::match_single_binding, clippy::suspicious_else_formatting)]
    fn from_node(
        node: ::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        debug_assert_eq!(node.kind(), "selector_expression");
        Ok(Self {
            span: ::treesitter_types::Span::from(node),
            field: {
                let child = node
                    .child_by_field_name("field")
                    .ok_or_else(|| ::treesitter_types::ParseError::missing_field(
                        "field",
                        node,
                    ))?;
                <FieldIdentifier as ::treesitter_types::FromNode>::from_node(child, src)?
            },
            operand: {
                let child = node
                    .child_by_field_name("operand")
                    .ok_or_else(|| ::treesitter_types::ParseError::missing_field(
                        "operand",
                        node,
                    ))?;
                <Expression as ::treesitter_types::FromNode>::from_node(child, src)?
            },
        })
    }
}
impl ::treesitter_types::Spanned for SelectorExpression<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        self.span
    }
}
#[derive(Debug, Clone)]
pub struct SendStatement<'tree> {
    pub span: ::treesitter_types::Span,
    pub channel: Expression<'tree>,
    pub value: Expression<'tree>,
}
impl<'tree> ::treesitter_types::FromNode<'tree> for SendStatement<'tree> {
    #[allow(clippy::match_single_binding, clippy::suspicious_else_formatting)]
    fn from_node(
        node: ::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        debug_assert_eq!(node.kind(), "send_statement");
        Ok(Self {
            span: ::treesitter_types::Span::from(node),
            channel: {
                let child = node
                    .child_by_field_name("channel")
                    .ok_or_else(|| ::treesitter_types::ParseError::missing_field(
                        "channel",
                        node,
                    ))?;
                <Expression as ::treesitter_types::FromNode>::from_node(child, src)?
            },
            value: {
                let child = node
                    .child_by_field_name("value")
                    .ok_or_else(|| ::treesitter_types::ParseError::missing_field(
                        "value",
                        node,
                    ))?;
                <Expression as ::treesitter_types::FromNode>::from_node(child, src)?
            },
        })
    }
}
impl ::treesitter_types::Spanned for SendStatement<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        self.span
    }
}
#[derive(Debug, Clone)]
pub struct ShortVarDeclaration<'tree> {
    pub span: ::treesitter_types::Span,
    pub left: ExpressionList<'tree>,
    pub right: ExpressionList<'tree>,
}
impl<'tree> ::treesitter_types::FromNode<'tree> for ShortVarDeclaration<'tree> {
    #[allow(clippy::match_single_binding, clippy::suspicious_else_formatting)]
    fn from_node(
        node: ::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        debug_assert_eq!(node.kind(), "short_var_declaration");
        Ok(Self {
            span: ::treesitter_types::Span::from(node),
            left: {
                let child = node
                    .child_by_field_name("left")
                    .ok_or_else(|| ::treesitter_types::ParseError::missing_field(
                        "left",
                        node,
                    ))?;
                <ExpressionList as ::treesitter_types::FromNode>::from_node(child, src)?
            },
            right: {
                let child = node
                    .child_by_field_name("right")
                    .ok_or_else(|| ::treesitter_types::ParseError::missing_field(
                        "right",
                        node,
                    ))?;
                <ExpressionList as ::treesitter_types::FromNode>::from_node(child, src)?
            },
        })
    }
}
impl ::treesitter_types::Spanned for ShortVarDeclaration<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        self.span
    }
}
#[derive(Debug, Clone)]
pub struct SliceExpression<'tree> {
    pub span: ::treesitter_types::Span,
    pub capacity: ::core::option::Option<Expression<'tree>>,
    pub end: ::core::option::Option<Expression<'tree>>,
    pub operand: Expression<'tree>,
    pub start: ::core::option::Option<Expression<'tree>>,
}
impl<'tree> ::treesitter_types::FromNode<'tree> for SliceExpression<'tree> {
    #[allow(clippy::match_single_binding, clippy::suspicious_else_formatting)]
    fn from_node(
        node: ::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        debug_assert_eq!(node.kind(), "slice_expression");
        Ok(Self {
            span: ::treesitter_types::Span::from(node),
            capacity: match node.child_by_field_name("capacity") {
                Some(child) => {
                    Some(
                        <Expression as ::treesitter_types::FromNode>::from_node(
                            child,
                            src,
                        )?,
                    )
                }
                None => None,
            },
            end: match node.child_by_field_name("end") {
                Some(child) => {
                    Some(
                        <Expression as ::treesitter_types::FromNode>::from_node(
                            child,
                            src,
                        )?,
                    )
                }
                None => None,
            },
            operand: {
                let child = node
                    .child_by_field_name("operand")
                    .ok_or_else(|| ::treesitter_types::ParseError::missing_field(
                        "operand",
                        node,
                    ))?;
                <Expression as ::treesitter_types::FromNode>::from_node(child, src)?
            },
            start: match node.child_by_field_name("start") {
                Some(child) => {
                    Some(
                        <Expression as ::treesitter_types::FromNode>::from_node(
                            child,
                            src,
                        )?,
                    )
                }
                None => None,
            },
        })
    }
}
impl ::treesitter_types::Spanned for SliceExpression<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        self.span
    }
}
#[derive(Debug, Clone)]
pub struct SliceType<'tree> {
    pub span: ::treesitter_types::Span,
    pub element: Type<'tree>,
}
impl<'tree> ::treesitter_types::FromNode<'tree> for SliceType<'tree> {
    #[allow(clippy::match_single_binding, clippy::suspicious_else_formatting)]
    fn from_node(
        node: ::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        debug_assert_eq!(node.kind(), "slice_type");
        Ok(Self {
            span: ::treesitter_types::Span::from(node),
            element: {
                let child = node
                    .child_by_field_name("element")
                    .ok_or_else(|| ::treesitter_types::ParseError::missing_field(
                        "element",
                        node,
                    ))?;
                <Type as ::treesitter_types::FromNode>::from_node(child, src)?
            },
        })
    }
}
impl ::treesitter_types::Spanned for SliceType<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        self.span
    }
}
#[derive(Debug, Clone)]
pub struct SourceFile<'tree> {
    pub span: ::treesitter_types::Span,
    pub children: ::std::vec::Vec<SourceFileChildren<'tree>>,
}
impl<'tree> ::treesitter_types::FromNode<'tree> for SourceFile<'tree> {
    #[allow(clippy::match_single_binding, clippy::suspicious_else_formatting)]
    fn from_node(
        node: ::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        debug_assert_eq!(node.kind(), "source_file");
        Ok(Self {
            span: ::treesitter_types::Span::from(node),
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
                            <SourceFileChildren as ::treesitter_types::FromNode>::from_node(
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
impl ::treesitter_types::Spanned for SourceFile<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        self.span
    }
}
#[derive(Debug, Clone)]
pub struct StatementList<'tree> {
    pub span: ::treesitter_types::Span,
    pub children: ::std::vec::Vec<Statement<'tree>>,
}
impl<'tree> ::treesitter_types::FromNode<'tree> for StatementList<'tree> {
    #[allow(clippy::match_single_binding, clippy::suspicious_else_formatting)]
    fn from_node(
        node: ::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        debug_assert_eq!(node.kind(), "statement_list");
        Ok(Self {
            span: ::treesitter_types::Span::from(node),
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
                            <Statement as ::treesitter_types::FromNode>::from_node(
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
impl ::treesitter_types::Spanned for StatementList<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        self.span
    }
}
#[derive(Debug, Clone)]
pub struct StructType<'tree> {
    pub span: ::treesitter_types::Span,
    pub children: FieldDeclarationList<'tree>,
}
impl<'tree> ::treesitter_types::FromNode<'tree> for StructType<'tree> {
    #[allow(clippy::match_single_binding, clippy::suspicious_else_formatting)]
    fn from_node(
        node: ::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        debug_assert_eq!(node.kind(), "struct_type");
        Ok(Self {
            span: ::treesitter_types::Span::from(node),
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
                            if !fallback_cursor.node().is_extra() {
                                let candidate = fallback_cursor.node();
                                if (|| -> ::core::result::Result<
                                    _,
                                    ::treesitter_types::ParseError,
                                > {
                                    let child = candidate;
                                    Ok(
                                        <FieldDeclarationList as ::treesitter_types::FromNode>::from_node(
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
                    fallback_child
                        .ok_or_else(|| ::treesitter_types::ParseError::missing_field(
                            "children",
                            node,
                        ))?
                };
                <FieldDeclarationList as ::treesitter_types::FromNode>::from_node(
                    child,
                    src,
                )?
            },
        })
    }
}
impl ::treesitter_types::Spanned for StructType<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        self.span
    }
}
#[derive(Debug, Clone)]
pub struct TypeAlias<'tree> {
    pub span: ::treesitter_types::Span,
    pub name: TypeIdentifier<'tree>,
    pub r#type: Type<'tree>,
    pub type_parameters: ::core::option::Option<TypeParameterList<'tree>>,
}
impl<'tree> ::treesitter_types::FromNode<'tree> for TypeAlias<'tree> {
    #[allow(clippy::match_single_binding, clippy::suspicious_else_formatting)]
    fn from_node(
        node: ::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        debug_assert_eq!(node.kind(), "type_alias");
        Ok(Self {
            span: ::treesitter_types::Span::from(node),
            name: {
                let child = node
                    .child_by_field_name("name")
                    .ok_or_else(|| ::treesitter_types::ParseError::missing_field(
                        "name",
                        node,
                    ))?;
                <TypeIdentifier as ::treesitter_types::FromNode>::from_node(child, src)?
            },
            r#type: {
                let child = node
                    .child_by_field_name("type")
                    .ok_or_else(|| ::treesitter_types::ParseError::missing_field(
                        "type",
                        node,
                    ))?;
                <Type as ::treesitter_types::FromNode>::from_node(child, src)?
            },
            type_parameters: match node.child_by_field_name("type_parameters") {
                Some(child) => {
                    Some(
                        <TypeParameterList as ::treesitter_types::FromNode>::from_node(
                            child,
                            src,
                        )?,
                    )
                }
                None => None,
            },
        })
    }
}
impl ::treesitter_types::Spanned for TypeAlias<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        self.span
    }
}
#[derive(Debug, Clone)]
pub struct TypeArguments<'tree> {
    pub span: ::treesitter_types::Span,
    pub children: ::std::vec::Vec<TypeElem<'tree>>,
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
                            <TypeElem as ::treesitter_types::FromNode>::from_node(
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
impl ::treesitter_types::Spanned for TypeArguments<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        self.span
    }
}
#[derive(Debug, Clone)]
pub struct TypeAssertionExpression<'tree> {
    pub span: ::treesitter_types::Span,
    pub operand: Expression<'tree>,
    pub r#type: Type<'tree>,
}
impl<'tree> ::treesitter_types::FromNode<'tree> for TypeAssertionExpression<'tree> {
    #[allow(clippy::match_single_binding, clippy::suspicious_else_formatting)]
    fn from_node(
        node: ::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        debug_assert_eq!(node.kind(), "type_assertion_expression");
        Ok(Self {
            span: ::treesitter_types::Span::from(node),
            operand: {
                let child = node
                    .child_by_field_name("operand")
                    .ok_or_else(|| ::treesitter_types::ParseError::missing_field(
                        "operand",
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
                <Type as ::treesitter_types::FromNode>::from_node(child, src)?
            },
        })
    }
}
impl ::treesitter_types::Spanned for TypeAssertionExpression<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        self.span
    }
}
#[derive(Debug, Clone)]
pub struct TypeCase<'tree> {
    pub span: ::treesitter_types::Span,
    pub r#type: ::std::vec::Vec<TypeCaseType<'tree>>,
    pub children: ::core::option::Option<StatementList<'tree>>,
}
impl<'tree> ::treesitter_types::FromNode<'tree> for TypeCase<'tree> {
    #[allow(clippy::match_single_binding, clippy::suspicious_else_formatting)]
    fn from_node(
        node: ::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        debug_assert_eq!(node.kind(), "type_case");
        Ok(Self {
            span: ::treesitter_types::Span::from(node),
            r#type: {
                let mut cursor = node.walk();
                let mut items = ::std::vec::Vec::new();
                for child in node.children_by_field_name("type", &mut cursor) {
                    items
                        .push(
                            <TypeCaseType as ::treesitter_types::FromNode>::from_node(
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
                            <StatementList as ::treesitter_types::FromNode>::from_node(
                                child,
                                src,
                            )?,
                        )
                    }
                    None => {
                        let mut _result = None;
                        let mut _fc = node.walk();
                        if _fc.goto_first_child() {
                            loop {
                                if !_fc.node().is_extra() {
                                    let child = _fc.node();
                                    if let Ok(v) = (|| -> ::core::result::Result<
                                        _,
                                        ::treesitter_types::ParseError,
                                    > {
                                        Ok(
                                            Some(
                                                <StatementList as ::treesitter_types::FromNode>::from_node(
                                                    child,
                                                    src,
                                                )?,
                                            ),
                                        )
                                    })() {
                                        _result = Some(v);
                                        break;
                                    }
                                }
                                if !_fc.goto_next_sibling() {
                                    break;
                                }
                            }
                        }
                        _result.flatten()
                    }
                }
            },
        })
    }
}
impl ::treesitter_types::Spanned for TypeCase<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        self.span
    }
}
#[derive(Debug, Clone)]
pub struct TypeConstraint<'tree> {
    pub span: ::treesitter_types::Span,
    pub children: ::std::vec::Vec<Type<'tree>>,
}
impl<'tree> ::treesitter_types::FromNode<'tree> for TypeConstraint<'tree> {
    #[allow(clippy::match_single_binding, clippy::suspicious_else_formatting)]
    fn from_node(
        node: ::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        debug_assert_eq!(node.kind(), "type_constraint");
        Ok(Self {
            span: ::treesitter_types::Span::from(node),
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
                            <Type as ::treesitter_types::FromNode>::from_node(
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
impl ::treesitter_types::Spanned for TypeConstraint<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        self.span
    }
}
#[derive(Debug, Clone)]
pub struct TypeConversionExpression<'tree> {
    pub span: ::treesitter_types::Span,
    pub operand: Expression<'tree>,
    pub r#type: Type<'tree>,
}
impl<'tree> ::treesitter_types::FromNode<'tree> for TypeConversionExpression<'tree> {
    #[allow(clippy::match_single_binding, clippy::suspicious_else_formatting)]
    fn from_node(
        node: ::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        debug_assert_eq!(node.kind(), "type_conversion_expression");
        Ok(Self {
            span: ::treesitter_types::Span::from(node),
            operand: {
                let child = node
                    .child_by_field_name("operand")
                    .ok_or_else(|| ::treesitter_types::ParseError::missing_field(
                        "operand",
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
                <Type as ::treesitter_types::FromNode>::from_node(child, src)?
            },
        })
    }
}
impl ::treesitter_types::Spanned for TypeConversionExpression<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        self.span
    }
}
#[derive(Debug, Clone)]
pub struct TypeDeclaration<'tree> {
    pub span: ::treesitter_types::Span,
    pub children: ::std::vec::Vec<TypeDeclarationChildren<'tree>>,
}
impl<'tree> ::treesitter_types::FromNode<'tree> for TypeDeclaration<'tree> {
    #[allow(clippy::match_single_binding, clippy::suspicious_else_formatting)]
    fn from_node(
        node: ::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        debug_assert_eq!(node.kind(), "type_declaration");
        Ok(Self {
            span: ::treesitter_types::Span::from(node),
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
                            <TypeDeclarationChildren as ::treesitter_types::FromNode>::from_node(
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
impl ::treesitter_types::Spanned for TypeDeclaration<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        self.span
    }
}
#[derive(Debug, Clone)]
pub struct TypeElem<'tree> {
    pub span: ::treesitter_types::Span,
    pub children: ::std::vec::Vec<Type<'tree>>,
}
impl<'tree> ::treesitter_types::FromNode<'tree> for TypeElem<'tree> {
    #[allow(clippy::match_single_binding, clippy::suspicious_else_formatting)]
    fn from_node(
        node: ::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        debug_assert_eq!(node.kind(), "type_elem");
        Ok(Self {
            span: ::treesitter_types::Span::from(node),
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
                            <Type as ::treesitter_types::FromNode>::from_node(
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
impl ::treesitter_types::Spanned for TypeElem<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        self.span
    }
}
#[derive(Debug, Clone)]
pub struct TypeInstantiationExpression<'tree> {
    pub span: ::treesitter_types::Span,
    pub r#type: Type<'tree>,
    pub children: ::std::vec::Vec<Type<'tree>>,
}
impl<'tree> ::treesitter_types::FromNode<'tree> for TypeInstantiationExpression<'tree> {
    #[allow(clippy::match_single_binding, clippy::suspicious_else_formatting)]
    fn from_node(
        node: ::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        debug_assert_eq!(node.kind(), "type_instantiation_expression");
        Ok(Self {
            span: ::treesitter_types::Span::from(node),
            r#type: {
                let child = node
                    .child_by_field_name("type")
                    .ok_or_else(|| ::treesitter_types::ParseError::missing_field(
                        "type",
                        node,
                    ))?;
                <Type as ::treesitter_types::FromNode>::from_node(child, src)?
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
                            <Type as ::treesitter_types::FromNode>::from_node(
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
impl ::treesitter_types::Spanned for TypeInstantiationExpression<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        self.span
    }
}
#[derive(Debug, Clone)]
pub struct TypeParameterDeclaration<'tree> {
    pub span: ::treesitter_types::Span,
    pub name: ::std::vec::Vec<Identifier<'tree>>,
    pub r#type: TypeConstraint<'tree>,
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
            name: {
                let mut cursor = node.walk();
                let mut items = ::std::vec::Vec::new();
                for child in node.children_by_field_name("name", &mut cursor) {
                    items
                        .push(
                            <Identifier as ::treesitter_types::FromNode>::from_node(
                                child,
                                src,
                            )?,
                        );
                }
                items
            },
            r#type: {
                let child = node
                    .child_by_field_name("type")
                    .ok_or_else(|| ::treesitter_types::ParseError::missing_field(
                        "type",
                        node,
                    ))?;
                <TypeConstraint as ::treesitter_types::FromNode>::from_node(child, src)?
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
pub struct TypeParameterList<'tree> {
    pub span: ::treesitter_types::Span,
    pub children: ::std::vec::Vec<TypeParameterDeclaration<'tree>>,
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
                            <TypeParameterDeclaration as ::treesitter_types::FromNode>::from_node(
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
impl ::treesitter_types::Spanned for TypeParameterList<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        self.span
    }
}
#[derive(Debug, Clone)]
pub struct TypeSpec<'tree> {
    pub span: ::treesitter_types::Span,
    pub name: TypeIdentifier<'tree>,
    pub r#type: Type<'tree>,
    pub type_parameters: ::core::option::Option<TypeParameterList<'tree>>,
}
impl<'tree> ::treesitter_types::FromNode<'tree> for TypeSpec<'tree> {
    #[allow(clippy::match_single_binding, clippy::suspicious_else_formatting)]
    fn from_node(
        node: ::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        debug_assert_eq!(node.kind(), "type_spec");
        Ok(Self {
            span: ::treesitter_types::Span::from(node),
            name: {
                let child = node
                    .child_by_field_name("name")
                    .ok_or_else(|| ::treesitter_types::ParseError::missing_field(
                        "name",
                        node,
                    ))?;
                <TypeIdentifier as ::treesitter_types::FromNode>::from_node(child, src)?
            },
            r#type: {
                let child = node
                    .child_by_field_name("type")
                    .ok_or_else(|| ::treesitter_types::ParseError::missing_field(
                        "type",
                        node,
                    ))?;
                <Type as ::treesitter_types::FromNode>::from_node(child, src)?
            },
            type_parameters: match node.child_by_field_name("type_parameters") {
                Some(child) => {
                    Some(
                        <TypeParameterList as ::treesitter_types::FromNode>::from_node(
                            child,
                            src,
                        )?,
                    )
                }
                None => None,
            },
        })
    }
}
impl ::treesitter_types::Spanned for TypeSpec<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        self.span
    }
}
#[derive(Debug, Clone)]
pub struct TypeSwitchStatement<'tree> {
    pub span: ::treesitter_types::Span,
    pub alias: ::core::option::Option<ExpressionList<'tree>>,
    pub initializer: ::core::option::Option<SimpleStatement<'tree>>,
    pub value: Expression<'tree>,
    pub children: ::std::vec::Vec<TypeSwitchStatementChildren<'tree>>,
}
impl<'tree> ::treesitter_types::FromNode<'tree> for TypeSwitchStatement<'tree> {
    #[allow(clippy::match_single_binding, clippy::suspicious_else_formatting)]
    fn from_node(
        node: ::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        debug_assert_eq!(node.kind(), "type_switch_statement");
        Ok(Self {
            span: ::treesitter_types::Span::from(node),
            alias: match node.child_by_field_name("alias") {
                Some(child) => {
                    Some(
                        <ExpressionList as ::treesitter_types::FromNode>::from_node(
                            child,
                            src,
                        )?,
                    )
                }
                None => None,
            },
            initializer: match node.child_by_field_name("initializer") {
                Some(child) => {
                    Some(
                        <SimpleStatement as ::treesitter_types::FromNode>::from_node(
                            child,
                            src,
                        )?,
                    )
                }
                None => None,
            },
            value: {
                let child = node
                    .child_by_field_name("value")
                    .ok_or_else(|| ::treesitter_types::ParseError::missing_field(
                        "value",
                        node,
                    ))?;
                <Expression as ::treesitter_types::FromNode>::from_node(child, src)?
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
                            <TypeSwitchStatementChildren as ::treesitter_types::FromNode>::from_node(
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
impl ::treesitter_types::Spanned for TypeSwitchStatement<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        self.span
    }
}
#[derive(Debug, Clone)]
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
                let child = node
                    .child_by_field_name("operand")
                    .ok_or_else(|| ::treesitter_types::ParseError::missing_field(
                        "operand",
                        node,
                    ))?;
                <Expression as ::treesitter_types::FromNode>::from_node(child, src)?
            },
            operator: {
                let child = node
                    .child_by_field_name("operator")
                    .ok_or_else(|| ::treesitter_types::ParseError::missing_field(
                        "operator",
                        node,
                    ))?;
                <UnaryExpressionOperator as ::treesitter_types::FromNode>::from_node(
                    child,
                    src,
                )?
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
pub struct VarDeclaration<'tree> {
    pub span: ::treesitter_types::Span,
    pub children: VarDeclarationChildren<'tree>,
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
                            if !fallback_cursor.node().is_extra() {
                                let candidate = fallback_cursor.node();
                                if (|| -> ::core::result::Result<
                                    _,
                                    ::treesitter_types::ParseError,
                                > {
                                    let child = candidate;
                                    Ok(
                                        <VarDeclarationChildren as ::treesitter_types::FromNode>::from_node(
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
                    fallback_child
                        .ok_or_else(|| ::treesitter_types::ParseError::missing_field(
                            "children",
                            node,
                        ))?
                };
                <VarDeclarationChildren as ::treesitter_types::FromNode>::from_node(
                    child,
                    src,
                )?
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
pub struct VarSpec<'tree> {
    pub span: ::treesitter_types::Span,
    pub name: ::std::vec::Vec<Identifier<'tree>>,
    pub r#type: ::core::option::Option<Type<'tree>>,
    pub value: ::core::option::Option<ExpressionList<'tree>>,
}
impl<'tree> ::treesitter_types::FromNode<'tree> for VarSpec<'tree> {
    #[allow(clippy::match_single_binding, clippy::suspicious_else_formatting)]
    fn from_node(
        node: ::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        debug_assert_eq!(node.kind(), "var_spec");
        Ok(Self {
            span: ::treesitter_types::Span::from(node),
            name: {
                let mut cursor = node.walk();
                let mut items = ::std::vec::Vec::new();
                for child in node.children_by_field_name("name", &mut cursor) {
                    items
                        .push(
                            <Identifier as ::treesitter_types::FromNode>::from_node(
                                child,
                                src,
                            )?,
                        );
                }
                items
            },
            r#type: match node.child_by_field_name("type") {
                Some(child) => {
                    Some(<Type as ::treesitter_types::FromNode>::from_node(child, src)?)
                }
                None => None,
            },
            value: match node.child_by_field_name("value") {
                Some(child) => {
                    Some(
                        <ExpressionList as ::treesitter_types::FromNode>::from_node(
                            child,
                            src,
                        )?,
                    )
                }
                None => None,
            },
        })
    }
}
impl ::treesitter_types::Spanned for VarSpec<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        self.span
    }
}
#[derive(Debug, Clone)]
pub struct VarSpecList<'tree> {
    pub span: ::treesitter_types::Span,
    pub children: ::std::vec::Vec<VarSpec<'tree>>,
}
impl<'tree> ::treesitter_types::FromNode<'tree> for VarSpecList<'tree> {
    #[allow(clippy::match_single_binding, clippy::suspicious_else_formatting)]
    fn from_node(
        node: ::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        debug_assert_eq!(node.kind(), "var_spec_list");
        Ok(Self {
            span: ::treesitter_types::Span::from(node),
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
                            <VarSpec as ::treesitter_types::FromNode>::from_node(
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
impl ::treesitter_types::Spanned for VarSpecList<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        self.span
    }
}
#[derive(Debug, Clone)]
pub struct VariadicArgument<'tree> {
    pub span: ::treesitter_types::Span,
    pub children: Expression<'tree>,
}
impl<'tree> ::treesitter_types::FromNode<'tree> for VariadicArgument<'tree> {
    #[allow(clippy::match_single_binding, clippy::suspicious_else_formatting)]
    fn from_node(
        node: ::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        debug_assert_eq!(node.kind(), "variadic_argument");
        Ok(Self {
            span: ::treesitter_types::Span::from(node),
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
                            if !fallback_cursor.node().is_extra() {
                                let candidate = fallback_cursor.node();
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
                    fallback_child
                        .ok_or_else(|| ::treesitter_types::ParseError::missing_field(
                            "children",
                            node,
                        ))?
                };
                <Expression as ::treesitter_types::FromNode>::from_node(child, src)?
            },
        })
    }
}
impl ::treesitter_types::Spanned for VariadicArgument<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        self.span
    }
}
#[derive(Debug, Clone)]
pub struct VariadicParameterDeclaration<'tree> {
    pub span: ::treesitter_types::Span,
    pub name: ::core::option::Option<Identifier<'tree>>,
    pub r#type: Type<'tree>,
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
            name: match node.child_by_field_name("name") {
                Some(child) => {
                    Some(
                        <Identifier as ::treesitter_types::FromNode>::from_node(
                            child,
                            src,
                        )?,
                    )
                }
                None => None,
            },
            r#type: {
                let child = node
                    .child_by_field_name("type")
                    .ok_or_else(|| ::treesitter_types::ParseError::missing_field(
                        "type",
                        node,
                    ))?;
                <Type as ::treesitter_types::FromNode>::from_node(child, src)?
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
pub struct BlankIdentifier<'tree> {
    pub span: ::treesitter_types::Span,
    text: &'tree str,
}
impl<'tree> ::treesitter_types::FromNode<'tree> for BlankIdentifier<'tree> {
    fn from_node(
        node: ::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        debug_assert_eq!(node.kind(), "blank_identifier");
        Ok(Self {
            span: ::treesitter_types::Span::from(node),
            text: node.utf8_text(src)?,
        })
    }
}
impl<'tree> ::treesitter_types::LeafNode<'tree> for BlankIdentifier<'tree> {
    fn text(&self) -> &'tree str {
        self.text
    }
}
impl ::treesitter_types::Spanned for BlankIdentifier<'_> {
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
pub struct FloatLiteral<'tree> {
    pub span: ::treesitter_types::Span,
    text: &'tree str,
}
impl<'tree> ::treesitter_types::FromNode<'tree> for FloatLiteral<'tree> {
    fn from_node(
        node: ::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        debug_assert_eq!(node.kind(), "float_literal");
        Ok(Self {
            span: ::treesitter_types::Span::from(node),
            text: node.utf8_text(src)?,
        })
    }
}
impl<'tree> ::treesitter_types::LeafNode<'tree> for FloatLiteral<'tree> {
    fn text(&self) -> &'tree str {
        self.text
    }
}
impl ::treesitter_types::Spanned for FloatLiteral<'_> {
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
pub struct ImaginaryLiteral<'tree> {
    pub span: ::treesitter_types::Span,
    text: &'tree str,
}
impl<'tree> ::treesitter_types::FromNode<'tree> for ImaginaryLiteral<'tree> {
    fn from_node(
        node: ::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        debug_assert_eq!(node.kind(), "imaginary_literal");
        Ok(Self {
            span: ::treesitter_types::Span::from(node),
            text: node.utf8_text(src)?,
        })
    }
}
impl<'tree> ::treesitter_types::LeafNode<'tree> for ImaginaryLiteral<'tree> {
    fn text(&self) -> &'tree str {
        self.text
    }
}
impl ::treesitter_types::Spanned for ImaginaryLiteral<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        self.span
    }
}
#[derive(Debug, Clone)]
pub struct IntLiteral<'tree> {
    pub span: ::treesitter_types::Span,
    text: &'tree str,
}
impl<'tree> ::treesitter_types::FromNode<'tree> for IntLiteral<'tree> {
    fn from_node(
        node: ::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        debug_assert_eq!(node.kind(), "int_literal");
        Ok(Self {
            span: ::treesitter_types::Span::from(node),
            text: node.utf8_text(src)?,
        })
    }
}
impl<'tree> ::treesitter_types::LeafNode<'tree> for IntLiteral<'tree> {
    fn text(&self) -> &'tree str {
        self.text
    }
}
impl ::treesitter_types::Spanned for IntLiteral<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        self.span
    }
}
#[derive(Debug, Clone)]
pub struct InterpretedStringLiteralContent<'tree> {
    pub span: ::treesitter_types::Span,
    text: &'tree str,
}
impl<'tree> ::treesitter_types::FromNode<'tree>
for InterpretedStringLiteralContent<'tree> {
    fn from_node(
        node: ::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        debug_assert_eq!(node.kind(), "interpreted_string_literal_content");
        Ok(Self {
            span: ::treesitter_types::Span::from(node),
            text: node.utf8_text(src)?,
        })
    }
}
impl<'tree> ::treesitter_types::LeafNode<'tree>
for InterpretedStringLiteralContent<'tree> {
    fn text(&self) -> &'tree str {
        self.text
    }
}
impl ::treesitter_types::Spanned for InterpretedStringLiteralContent<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        self.span
    }
}
#[derive(Debug, Clone)]
pub struct Iota<'tree> {
    pub span: ::treesitter_types::Span,
    text: &'tree str,
}
impl<'tree> ::treesitter_types::FromNode<'tree> for Iota<'tree> {
    fn from_node(
        node: ::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        debug_assert_eq!(node.kind(), "iota");
        Ok(Self {
            span: ::treesitter_types::Span::from(node),
            text: node.utf8_text(src)?,
        })
    }
}
impl<'tree> ::treesitter_types::LeafNode<'tree> for Iota<'tree> {
    fn text(&self) -> &'tree str {
        self.text
    }
}
impl ::treesitter_types::Spanned for Iota<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        self.span
    }
}
#[derive(Debug, Clone)]
pub struct LabelName<'tree> {
    pub span: ::treesitter_types::Span,
    text: &'tree str,
}
impl<'tree> ::treesitter_types::FromNode<'tree> for LabelName<'tree> {
    fn from_node(
        node: ::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        debug_assert_eq!(node.kind(), "label_name");
        Ok(Self {
            span: ::treesitter_types::Span::from(node),
            text: node.utf8_text(src)?,
        })
    }
}
impl<'tree> ::treesitter_types::LeafNode<'tree> for LabelName<'tree> {
    fn text(&self) -> &'tree str {
        self.text
    }
}
impl ::treesitter_types::Spanned for LabelName<'_> {
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
pub struct PackageIdentifier<'tree> {
    pub span: ::treesitter_types::Span,
    text: &'tree str,
}
impl<'tree> ::treesitter_types::FromNode<'tree> for PackageIdentifier<'tree> {
    fn from_node(
        node: ::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        debug_assert_eq!(node.kind(), "package_identifier");
        Ok(Self {
            span: ::treesitter_types::Span::from(node),
            text: node.utf8_text(src)?,
        })
    }
}
impl<'tree> ::treesitter_types::LeafNode<'tree> for PackageIdentifier<'tree> {
    fn text(&self) -> &'tree str {
        self.text
    }
}
impl ::treesitter_types::Spanned for PackageIdentifier<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        self.span
    }
}
#[derive(Debug, Clone)]
pub struct RawStringLiteralContent<'tree> {
    pub span: ::treesitter_types::Span,
    text: &'tree str,
}
impl<'tree> ::treesitter_types::FromNode<'tree> for RawStringLiteralContent<'tree> {
    fn from_node(
        node: ::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        debug_assert_eq!(node.kind(), "raw_string_literal_content");
        Ok(Self {
            span: ::treesitter_types::Span::from(node),
            text: node.utf8_text(src)?,
        })
    }
}
impl<'tree> ::treesitter_types::LeafNode<'tree> for RawStringLiteralContent<'tree> {
    fn text(&self) -> &'tree str {
        self.text
    }
}
impl ::treesitter_types::Spanned for RawStringLiteralContent<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        self.span
    }
}
#[derive(Debug, Clone)]
pub struct RuneLiteral<'tree> {
    pub span: ::treesitter_types::Span,
    text: &'tree str,
}
impl<'tree> ::treesitter_types::FromNode<'tree> for RuneLiteral<'tree> {
    fn from_node(
        node: ::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        debug_assert_eq!(node.kind(), "rune_literal");
        Ok(Self {
            span: ::treesitter_types::Span::from(node),
            text: node.utf8_text(src)?,
        })
    }
}
impl<'tree> ::treesitter_types::LeafNode<'tree> for RuneLiteral<'tree> {
    fn text(&self) -> &'tree str {
        self.text
    }
}
impl ::treesitter_types::Spanned for RuneLiteral<'_> {
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
pub enum ArgumentListChildren<'tree> {
    Expression(::std::boxed::Box<Expression<'tree>>),
    Type(::std::boxed::Box<Type<'tree>>),
    VariadicArgument(::std::boxed::Box<VariadicArgument<'tree>>),
}
impl<'tree> ::treesitter_types::FromNode<'tree> for ArgumentListChildren<'tree> {
    #[allow(clippy::collapsible_else_if)]
    fn from_node(
        node: ::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        match node.kind() {
            "variadic_argument" => {
                Ok(
                    Self::VariadicArgument(
                        ::std::boxed::Box::new(
                            <VariadicArgument as ::treesitter_types::FromNode>::from_node(
                                node,
                                src,
                            )?,
                        ),
                    ),
                )
            }
            _other => {
                if let Ok(v) = <Expression as ::treesitter_types::FromNode>::from_node(
                    node,
                    src,
                ) {
                    Ok(Self::Expression(::std::boxed::Box::new(v)))
                } else {
                    if let Ok(v) = <Type as ::treesitter_types::FromNode>::from_node(
                        node,
                        src,
                    ) {
                        Ok(Self::Type(::std::boxed::Box::new(v)))
                    } else {
                        Err(
                            ::treesitter_types::ParseError::unexpected_kind(_other, node),
                        )
                    }
                }
            }
        }
    }
}
impl ::treesitter_types::Spanned for ArgumentListChildren<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        match self {
            Self::Expression(inner) => inner.span(),
            Self::Type(inner) => inner.span(),
            Self::VariadicArgument(inner) => inner.span(),
        }
    }
}
#[derive(Debug, Clone)]
pub enum AssignmentStatementOperator {
    PercentEq(::treesitter_types::Span),
    AmpEq(::treesitter_types::Span),
    AmpCaretEq(::treesitter_types::Span),
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
impl<'tree> ::treesitter_types::FromNode<'tree> for AssignmentStatementOperator {
    #[allow(clippy::collapsible_else_if)]
    fn from_node(
        node: ::tree_sitter::Node<'tree>,
        _src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        match node.kind() {
            "%=" => Ok(Self::PercentEq(::treesitter_types::Span::from(node))),
            "&=" => Ok(Self::AmpEq(::treesitter_types::Span::from(node))),
            "&^=" => Ok(Self::AmpCaretEq(::treesitter_types::Span::from(node))),
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
impl ::treesitter_types::Spanned for AssignmentStatementOperator {
    fn span(&self) -> ::treesitter_types::Span {
        match self {
            Self::PercentEq(span) => *span,
            Self::AmpEq(span) => *span,
            Self::AmpCaretEq(span) => *span,
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
#[derive(Debug, Clone)]
pub enum BinaryExpressionOperator {
    NotEq(::treesitter_types::Span),
    Percent(::treesitter_types::Span),
    Amp(::treesitter_types::Span),
    AmpAmp(::treesitter_types::Span),
    AmpCaret(::treesitter_types::Span),
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
        node: ::tree_sitter::Node<'tree>,
        _src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        match node.kind() {
            "!=" => Ok(Self::NotEq(::treesitter_types::Span::from(node))),
            "%" => Ok(Self::Percent(::treesitter_types::Span::from(node))),
            "&" => Ok(Self::Amp(::treesitter_types::Span::from(node))),
            "&&" => Ok(Self::AmpAmp(::treesitter_types::Span::from(node))),
            "&^" => Ok(Self::AmpCaret(::treesitter_types::Span::from(node))),
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
            Self::AmpCaret(span) => *span,
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
#[derive(Debug, Clone)]
pub enum CommunicationCaseCommunication<'tree> {
    ReceiveStatement(::std::boxed::Box<ReceiveStatement<'tree>>),
    SendStatement(::std::boxed::Box<SendStatement<'tree>>),
}
impl<'tree> ::treesitter_types::FromNode<'tree>
for CommunicationCaseCommunication<'tree> {
    #[allow(clippy::collapsible_else_if)]
    fn from_node(
        node: ::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        match node.kind() {
            "receive_statement" => {
                Ok(
                    Self::ReceiveStatement(
                        ::std::boxed::Box::new(
                            <ReceiveStatement as ::treesitter_types::FromNode>::from_node(
                                node,
                                src,
                            )?,
                        ),
                    ),
                )
            }
            "send_statement" => {
                Ok(
                    Self::SendStatement(
                        ::std::boxed::Box::new(
                            <SendStatement as ::treesitter_types::FromNode>::from_node(
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
impl ::treesitter_types::Spanned for CommunicationCaseCommunication<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        match self {
            Self::ReceiveStatement(inner) => inner.span(),
            Self::SendStatement(inner) => inner.span(),
        }
    }
}
#[derive(Debug, Clone)]
pub enum CompositeLiteralType<'tree> {
    ArrayType(::std::boxed::Box<ArrayType<'tree>>),
    GenericType(::std::boxed::Box<GenericType<'tree>>),
    ImplicitLengthArrayType(::std::boxed::Box<ImplicitLengthArrayType<'tree>>),
    MapType(::std::boxed::Box<MapType<'tree>>),
    QualifiedType(::std::boxed::Box<QualifiedType<'tree>>),
    SliceType(::std::boxed::Box<SliceType<'tree>>),
    StructType(::std::boxed::Box<StructType<'tree>>),
    TypeIdentifier(::std::boxed::Box<TypeIdentifier<'tree>>),
}
impl<'tree> ::treesitter_types::FromNode<'tree> for CompositeLiteralType<'tree> {
    #[allow(clippy::collapsible_else_if)]
    fn from_node(
        node: ::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        match node.kind() {
            "array_type" => {
                Ok(
                    Self::ArrayType(
                        ::std::boxed::Box::new(
                            <ArrayType as ::treesitter_types::FromNode>::from_node(
                                node,
                                src,
                            )?,
                        ),
                    ),
                )
            }
            "generic_type" => {
                Ok(
                    Self::GenericType(
                        ::std::boxed::Box::new(
                            <GenericType as ::treesitter_types::FromNode>::from_node(
                                node,
                                src,
                            )?,
                        ),
                    ),
                )
            }
            "implicit_length_array_type" => {
                Ok(
                    Self::ImplicitLengthArrayType(
                        ::std::boxed::Box::new(
                            <ImplicitLengthArrayType as ::treesitter_types::FromNode>::from_node(
                                node,
                                src,
                            )?,
                        ),
                    ),
                )
            }
            "map_type" => {
                Ok(
                    Self::MapType(
                        ::std::boxed::Box::new(
                            <MapType as ::treesitter_types::FromNode>::from_node(
                                node,
                                src,
                            )?,
                        ),
                    ),
                )
            }
            "qualified_type" => {
                Ok(
                    Self::QualifiedType(
                        ::std::boxed::Box::new(
                            <QualifiedType as ::treesitter_types::FromNode>::from_node(
                                node,
                                src,
                            )?,
                        ),
                    ),
                )
            }
            "slice_type" => {
                Ok(
                    Self::SliceType(
                        ::std::boxed::Box::new(
                            <SliceType as ::treesitter_types::FromNode>::from_node(
                                node,
                                src,
                            )?,
                        ),
                    ),
                )
            }
            "struct_type" => {
                Ok(
                    Self::StructType(
                        ::std::boxed::Box::new(
                            <StructType as ::treesitter_types::FromNode>::from_node(
                                node,
                                src,
                            )?,
                        ),
                    ),
                )
            }
            "type_identifier" => {
                Ok(
                    Self::TypeIdentifier(
                        ::std::boxed::Box::new(
                            <TypeIdentifier as ::treesitter_types::FromNode>::from_node(
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
impl ::treesitter_types::Spanned for CompositeLiteralType<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        match self {
            Self::ArrayType(inner) => inner.span(),
            Self::GenericType(inner) => inner.span(),
            Self::ImplicitLengthArrayType(inner) => inner.span(),
            Self::MapType(inner) => inner.span(),
            Self::QualifiedType(inner) => inner.span(),
            Self::SliceType(inner) => inner.span(),
            Self::StructType(inner) => inner.span(),
            Self::TypeIdentifier(inner) => inner.span(),
        }
    }
}
#[derive(Debug, Clone)]
pub enum ConstSpecName<'tree> {
    Comma(::treesitter_types::Span),
    Identifier(::std::boxed::Box<Identifier<'tree>>),
}
impl<'tree> ::treesitter_types::FromNode<'tree> for ConstSpecName<'tree> {
    #[allow(clippy::collapsible_else_if)]
    fn from_node(
        node: ::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        match node.kind() {
            "," => Ok(Self::Comma(::treesitter_types::Span::from(node))),
            "identifier" => {
                Ok(
                    Self::Identifier(
                        ::std::boxed::Box::new(
                            <Identifier as ::treesitter_types::FromNode>::from_node(
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
impl ::treesitter_types::Spanned for ConstSpecName<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        match self {
            Self::Comma(span) => *span,
            Self::Identifier(inner) => inner.span(),
        }
    }
}
#[derive(Debug, Clone)]
pub enum ExpressionSwitchStatementChildren<'tree> {
    DefaultCase(::std::boxed::Box<DefaultCase<'tree>>),
    ExpressionCase(::std::boxed::Box<ExpressionCase<'tree>>),
}
impl<'tree> ::treesitter_types::FromNode<'tree>
for ExpressionSwitchStatementChildren<'tree> {
    #[allow(clippy::collapsible_else_if)]
    fn from_node(
        node: ::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        match node.kind() {
            "default_case" => {
                Ok(
                    Self::DefaultCase(
                        ::std::boxed::Box::new(
                            <DefaultCase as ::treesitter_types::FromNode>::from_node(
                                node,
                                src,
                            )?,
                        ),
                    ),
                )
            }
            "expression_case" => {
                Ok(
                    Self::ExpressionCase(
                        ::std::boxed::Box::new(
                            <ExpressionCase as ::treesitter_types::FromNode>::from_node(
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
impl ::treesitter_types::Spanned for ExpressionSwitchStatementChildren<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        match self {
            Self::DefaultCase(inner) => inner.span(),
            Self::ExpressionCase(inner) => inner.span(),
        }
    }
}
#[derive(Debug, Clone)]
pub enum FieldDeclarationTag<'tree> {
    InterpretedStringLiteral(::std::boxed::Box<InterpretedStringLiteral<'tree>>),
    RawStringLiteral(::std::boxed::Box<RawStringLiteral<'tree>>),
}
impl<'tree> ::treesitter_types::FromNode<'tree> for FieldDeclarationTag<'tree> {
    #[allow(clippy::collapsible_else_if)]
    fn from_node(
        node: ::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        match node.kind() {
            "interpreted_string_literal" => {
                Ok(
                    Self::InterpretedStringLiteral(
                        ::std::boxed::Box::new(
                            <InterpretedStringLiteral as ::treesitter_types::FromNode>::from_node(
                                node,
                                src,
                            )?,
                        ),
                    ),
                )
            }
            "raw_string_literal" => {
                Ok(
                    Self::RawStringLiteral(
                        ::std::boxed::Box::new(
                            <RawStringLiteral as ::treesitter_types::FromNode>::from_node(
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
impl ::treesitter_types::Spanned for FieldDeclarationTag<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        match self {
            Self::InterpretedStringLiteral(inner) => inner.span(),
            Self::RawStringLiteral(inner) => inner.span(),
        }
    }
}
#[derive(Debug, Clone)]
pub enum FieldDeclarationType<'tree> {
    Type(::std::boxed::Box<Type<'tree>>),
    GenericType(::std::boxed::Box<GenericType<'tree>>),
    QualifiedType(::std::boxed::Box<QualifiedType<'tree>>),
    TypeIdentifier(::std::boxed::Box<TypeIdentifier<'tree>>),
}
impl<'tree> ::treesitter_types::FromNode<'tree> for FieldDeclarationType<'tree> {
    #[allow(clippy::collapsible_else_if)]
    fn from_node(
        node: ::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        match node.kind() {
            "generic_type" => {
                Ok(
                    Self::GenericType(
                        ::std::boxed::Box::new(
                            <GenericType as ::treesitter_types::FromNode>::from_node(
                                node,
                                src,
                            )?,
                        ),
                    ),
                )
            }
            "qualified_type" => {
                Ok(
                    Self::QualifiedType(
                        ::std::boxed::Box::new(
                            <QualifiedType as ::treesitter_types::FromNode>::from_node(
                                node,
                                src,
                            )?,
                        ),
                    ),
                )
            }
            "type_identifier" => {
                Ok(
                    Self::TypeIdentifier(
                        ::std::boxed::Box::new(
                            <TypeIdentifier as ::treesitter_types::FromNode>::from_node(
                                node,
                                src,
                            )?,
                        ),
                    ),
                )
            }
            _other => {
                if let Ok(v) = <Type as ::treesitter_types::FromNode>::from_node(
                    node,
                    src,
                ) {
                    Ok(Self::Type(::std::boxed::Box::new(v)))
                } else {
                    Err(::treesitter_types::ParseError::unexpected_kind(_other, node))
                }
            }
        }
    }
}
impl ::treesitter_types::Spanned for FieldDeclarationType<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        match self {
            Self::Type(inner) => inner.span(),
            Self::GenericType(inner) => inner.span(),
            Self::QualifiedType(inner) => inner.span(),
            Self::TypeIdentifier(inner) => inner.span(),
        }
    }
}
#[derive(Debug, Clone)]
pub enum ForStatementChildren<'tree> {
    Expression(::std::boxed::Box<Expression<'tree>>),
    ForClause(::std::boxed::Box<ForClause<'tree>>),
    RangeClause(::std::boxed::Box<RangeClause<'tree>>),
}
impl<'tree> ::treesitter_types::FromNode<'tree> for ForStatementChildren<'tree> {
    #[allow(clippy::collapsible_else_if)]
    fn from_node(
        node: ::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        match node.kind() {
            "for_clause" => {
                Ok(
                    Self::ForClause(
                        ::std::boxed::Box::new(
                            <ForClause as ::treesitter_types::FromNode>::from_node(
                                node,
                                src,
                            )?,
                        ),
                    ),
                )
            }
            "range_clause" => {
                Ok(
                    Self::RangeClause(
                        ::std::boxed::Box::new(
                            <RangeClause as ::treesitter_types::FromNode>::from_node(
                                node,
                                src,
                            )?,
                        ),
                    ),
                )
            }
            _other => {
                if let Ok(v) = <Expression as ::treesitter_types::FromNode>::from_node(
                    node,
                    src,
                ) {
                    Ok(Self::Expression(::std::boxed::Box::new(v)))
                } else {
                    Err(::treesitter_types::ParseError::unexpected_kind(_other, node))
                }
            }
        }
    }
}
impl ::treesitter_types::Spanned for ForStatementChildren<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        match self {
            Self::Expression(inner) => inner.span(),
            Self::ForClause(inner) => inner.span(),
            Self::RangeClause(inner) => inner.span(),
        }
    }
}
#[derive(Debug, Clone)]
pub enum FuncLiteralResult<'tree> {
    SimpleType(::std::boxed::Box<SimpleType<'tree>>),
    ParameterList(::std::boxed::Box<ParameterList<'tree>>),
}
impl<'tree> ::treesitter_types::FromNode<'tree> for FuncLiteralResult<'tree> {
    #[allow(clippy::collapsible_else_if)]
    fn from_node(
        node: ::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        match node.kind() {
            "parameter_list" => {
                Ok(
                    Self::ParameterList(
                        ::std::boxed::Box::new(
                            <ParameterList as ::treesitter_types::FromNode>::from_node(
                                node,
                                src,
                            )?,
                        ),
                    ),
                )
            }
            _other => {
                if let Ok(v) = <SimpleType as ::treesitter_types::FromNode>::from_node(
                    node,
                    src,
                ) {
                    Ok(Self::SimpleType(::std::boxed::Box::new(v)))
                } else {
                    Err(::treesitter_types::ParseError::unexpected_kind(_other, node))
                }
            }
        }
    }
}
impl ::treesitter_types::Spanned for FuncLiteralResult<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        match self {
            Self::SimpleType(inner) => inner.span(),
            Self::ParameterList(inner) => inner.span(),
        }
    }
}
#[derive(Debug, Clone)]
pub enum FunctionDeclarationResult<'tree> {
    SimpleType(::std::boxed::Box<SimpleType<'tree>>),
    ParameterList(::std::boxed::Box<ParameterList<'tree>>),
}
impl<'tree> ::treesitter_types::FromNode<'tree> for FunctionDeclarationResult<'tree> {
    #[allow(clippy::collapsible_else_if)]
    fn from_node(
        node: ::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        match node.kind() {
            "parameter_list" => {
                Ok(
                    Self::ParameterList(
                        ::std::boxed::Box::new(
                            <ParameterList as ::treesitter_types::FromNode>::from_node(
                                node,
                                src,
                            )?,
                        ),
                    ),
                )
            }
            _other => {
                if let Ok(v) = <SimpleType as ::treesitter_types::FromNode>::from_node(
                    node,
                    src,
                ) {
                    Ok(Self::SimpleType(::std::boxed::Box::new(v)))
                } else {
                    Err(::treesitter_types::ParseError::unexpected_kind(_other, node))
                }
            }
        }
    }
}
impl ::treesitter_types::Spanned for FunctionDeclarationResult<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        match self {
            Self::SimpleType(inner) => inner.span(),
            Self::ParameterList(inner) => inner.span(),
        }
    }
}
#[derive(Debug, Clone)]
pub enum FunctionTypeResult<'tree> {
    SimpleType(::std::boxed::Box<SimpleType<'tree>>),
    ParameterList(::std::boxed::Box<ParameterList<'tree>>),
}
impl<'tree> ::treesitter_types::FromNode<'tree> for FunctionTypeResult<'tree> {
    #[allow(clippy::collapsible_else_if)]
    fn from_node(
        node: ::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        match node.kind() {
            "parameter_list" => {
                Ok(
                    Self::ParameterList(
                        ::std::boxed::Box::new(
                            <ParameterList as ::treesitter_types::FromNode>::from_node(
                                node,
                                src,
                            )?,
                        ),
                    ),
                )
            }
            _other => {
                if let Ok(v) = <SimpleType as ::treesitter_types::FromNode>::from_node(
                    node,
                    src,
                ) {
                    Ok(Self::SimpleType(::std::boxed::Box::new(v)))
                } else {
                    Err(::treesitter_types::ParseError::unexpected_kind(_other, node))
                }
            }
        }
    }
}
impl ::treesitter_types::Spanned for FunctionTypeResult<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        match self {
            Self::SimpleType(inner) => inner.span(),
            Self::ParameterList(inner) => inner.span(),
        }
    }
}
#[derive(Debug, Clone)]
pub enum GenericTypeType<'tree> {
    NegatedType(::std::boxed::Box<NegatedType<'tree>>),
    QualifiedType(::std::boxed::Box<QualifiedType<'tree>>),
    TypeIdentifier(::std::boxed::Box<TypeIdentifier<'tree>>),
}
impl<'tree> ::treesitter_types::FromNode<'tree> for GenericTypeType<'tree> {
    #[allow(clippy::collapsible_else_if)]
    fn from_node(
        node: ::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        match node.kind() {
            "negated_type" => {
                Ok(
                    Self::NegatedType(
                        ::std::boxed::Box::new(
                            <NegatedType as ::treesitter_types::FromNode>::from_node(
                                node,
                                src,
                            )?,
                        ),
                    ),
                )
            }
            "qualified_type" => {
                Ok(
                    Self::QualifiedType(
                        ::std::boxed::Box::new(
                            <QualifiedType as ::treesitter_types::FromNode>::from_node(
                                node,
                                src,
                            )?,
                        ),
                    ),
                )
            }
            "type_identifier" => {
                Ok(
                    Self::TypeIdentifier(
                        ::std::boxed::Box::new(
                            <TypeIdentifier as ::treesitter_types::FromNode>::from_node(
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
impl ::treesitter_types::Spanned for GenericTypeType<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        match self {
            Self::NegatedType(inner) => inner.span(),
            Self::QualifiedType(inner) => inner.span(),
            Self::TypeIdentifier(inner) => inner.span(),
        }
    }
}
#[derive(Debug, Clone)]
pub enum IfStatementAlternative<'tree> {
    Block(::std::boxed::Box<Block<'tree>>),
    IfStatement(::std::boxed::Box<IfStatement<'tree>>),
}
impl<'tree> ::treesitter_types::FromNode<'tree> for IfStatementAlternative<'tree> {
    #[allow(clippy::collapsible_else_if)]
    fn from_node(
        node: ::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        match node.kind() {
            "block" => {
                Ok(
                    Self::Block(
                        ::std::boxed::Box::new(
                            <Block as ::treesitter_types::FromNode>::from_node(
                                node,
                                src,
                            )?,
                        ),
                    ),
                )
            }
            "if_statement" => {
                Ok(
                    Self::IfStatement(
                        ::std::boxed::Box::new(
                            <IfStatement as ::treesitter_types::FromNode>::from_node(
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
impl ::treesitter_types::Spanned for IfStatementAlternative<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        match self {
            Self::Block(inner) => inner.span(),
            Self::IfStatement(inner) => inner.span(),
        }
    }
}
#[derive(Debug, Clone)]
pub enum ImportDeclarationChildren<'tree> {
    ImportSpec(::std::boxed::Box<ImportSpec<'tree>>),
    ImportSpecList(::std::boxed::Box<ImportSpecList<'tree>>),
}
impl<'tree> ::treesitter_types::FromNode<'tree> for ImportDeclarationChildren<'tree> {
    #[allow(clippy::collapsible_else_if)]
    fn from_node(
        node: ::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        match node.kind() {
            "import_spec" => {
                Ok(
                    Self::ImportSpec(
                        ::std::boxed::Box::new(
                            <ImportSpec as ::treesitter_types::FromNode>::from_node(
                                node,
                                src,
                            )?,
                        ),
                    ),
                )
            }
            "import_spec_list" => {
                Ok(
                    Self::ImportSpecList(
                        ::std::boxed::Box::new(
                            <ImportSpecList as ::treesitter_types::FromNode>::from_node(
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
impl ::treesitter_types::Spanned for ImportDeclarationChildren<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        match self {
            Self::ImportSpec(inner) => inner.span(),
            Self::ImportSpecList(inner) => inner.span(),
        }
    }
}
#[derive(Debug, Clone)]
pub enum ImportSpecName<'tree> {
    BlankIdentifier(::std::boxed::Box<BlankIdentifier<'tree>>),
    Dot(::std::boxed::Box<Dot<'tree>>),
    PackageIdentifier(::std::boxed::Box<PackageIdentifier<'tree>>),
}
impl<'tree> ::treesitter_types::FromNode<'tree> for ImportSpecName<'tree> {
    #[allow(clippy::collapsible_else_if)]
    fn from_node(
        node: ::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        match node.kind() {
            "blank_identifier" => {
                Ok(
                    Self::BlankIdentifier(
                        ::std::boxed::Box::new(
                            <BlankIdentifier as ::treesitter_types::FromNode>::from_node(
                                node,
                                src,
                            )?,
                        ),
                    ),
                )
            }
            "dot" => {
                Ok(
                    Self::Dot(
                        ::std::boxed::Box::new(
                            <Dot as ::treesitter_types::FromNode>::from_node(node, src)?,
                        ),
                    ),
                )
            }
            "package_identifier" => {
                Ok(
                    Self::PackageIdentifier(
                        ::std::boxed::Box::new(
                            <PackageIdentifier as ::treesitter_types::FromNode>::from_node(
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
impl ::treesitter_types::Spanned for ImportSpecName<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        match self {
            Self::BlankIdentifier(inner) => inner.span(),
            Self::Dot(inner) => inner.span(),
            Self::PackageIdentifier(inner) => inner.span(),
        }
    }
}
#[derive(Debug, Clone)]
pub enum ImportSpecPath<'tree> {
    InterpretedStringLiteral(::std::boxed::Box<InterpretedStringLiteral<'tree>>),
    RawStringLiteral(::std::boxed::Box<RawStringLiteral<'tree>>),
}
impl<'tree> ::treesitter_types::FromNode<'tree> for ImportSpecPath<'tree> {
    #[allow(clippy::collapsible_else_if)]
    fn from_node(
        node: ::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        match node.kind() {
            "interpreted_string_literal" => {
                Ok(
                    Self::InterpretedStringLiteral(
                        ::std::boxed::Box::new(
                            <InterpretedStringLiteral as ::treesitter_types::FromNode>::from_node(
                                node,
                                src,
                            )?,
                        ),
                    ),
                )
            }
            "raw_string_literal" => {
                Ok(
                    Self::RawStringLiteral(
                        ::std::boxed::Box::new(
                            <RawStringLiteral as ::treesitter_types::FromNode>::from_node(
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
impl ::treesitter_types::Spanned for ImportSpecPath<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        match self {
            Self::InterpretedStringLiteral(inner) => inner.span(),
            Self::RawStringLiteral(inner) => inner.span(),
        }
    }
}
#[derive(Debug, Clone)]
pub enum InterfaceTypeChildren<'tree> {
    MethodElem(::std::boxed::Box<MethodElem<'tree>>),
    TypeElem(::std::boxed::Box<TypeElem<'tree>>),
}
impl<'tree> ::treesitter_types::FromNode<'tree> for InterfaceTypeChildren<'tree> {
    #[allow(clippy::collapsible_else_if)]
    fn from_node(
        node: ::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        match node.kind() {
            "method_elem" => {
                Ok(
                    Self::MethodElem(
                        ::std::boxed::Box::new(
                            <MethodElem as ::treesitter_types::FromNode>::from_node(
                                node,
                                src,
                            )?,
                        ),
                    ),
                )
            }
            "type_elem" => {
                Ok(
                    Self::TypeElem(
                        ::std::boxed::Box::new(
                            <TypeElem as ::treesitter_types::FromNode>::from_node(
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
impl ::treesitter_types::Spanned for InterfaceTypeChildren<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        match self {
            Self::MethodElem(inner) => inner.span(),
            Self::TypeElem(inner) => inner.span(),
        }
    }
}
#[derive(Debug, Clone)]
pub enum InterpretedStringLiteralChildren<'tree> {
    EscapeSequence(::std::boxed::Box<EscapeSequence<'tree>>),
    InterpretedStringLiteralContent(
        ::std::boxed::Box<InterpretedStringLiteralContent<'tree>>,
    ),
}
impl<'tree> ::treesitter_types::FromNode<'tree>
for InterpretedStringLiteralChildren<'tree> {
    #[allow(clippy::collapsible_else_if)]
    fn from_node(
        node: ::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        match node.kind() {
            "escape_sequence" => {
                Ok(
                    Self::EscapeSequence(
                        ::std::boxed::Box::new(
                            <EscapeSequence as ::treesitter_types::FromNode>::from_node(
                                node,
                                src,
                            )?,
                        ),
                    ),
                )
            }
            "interpreted_string_literal_content" => {
                Ok(
                    Self::InterpretedStringLiteralContent(
                        ::std::boxed::Box::new(
                            <InterpretedStringLiteralContent as ::treesitter_types::FromNode>::from_node(
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
impl ::treesitter_types::Spanned for InterpretedStringLiteralChildren<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        match self {
            Self::EscapeSequence(inner) => inner.span(),
            Self::InterpretedStringLiteralContent(inner) => inner.span(),
        }
    }
}
#[derive(Debug, Clone)]
pub enum LiteralElementChildren<'tree> {
    Expression(::std::boxed::Box<Expression<'tree>>),
    LiteralValue(::std::boxed::Box<LiteralValue<'tree>>),
}
impl<'tree> ::treesitter_types::FromNode<'tree> for LiteralElementChildren<'tree> {
    #[allow(clippy::collapsible_else_if)]
    fn from_node(
        node: ::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        match node.kind() {
            "literal_value" => {
                Ok(
                    Self::LiteralValue(
                        ::std::boxed::Box::new(
                            <LiteralValue as ::treesitter_types::FromNode>::from_node(
                                node,
                                src,
                            )?,
                        ),
                    ),
                )
            }
            _other => {
                if let Ok(v) = <Expression as ::treesitter_types::FromNode>::from_node(
                    node,
                    src,
                ) {
                    Ok(Self::Expression(::std::boxed::Box::new(v)))
                } else {
                    Err(::treesitter_types::ParseError::unexpected_kind(_other, node))
                }
            }
        }
    }
}
impl ::treesitter_types::Spanned for LiteralElementChildren<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        match self {
            Self::Expression(inner) => inner.span(),
            Self::LiteralValue(inner) => inner.span(),
        }
    }
}
#[derive(Debug, Clone)]
pub enum LiteralValueChildren<'tree> {
    KeyedElement(::std::boxed::Box<KeyedElement<'tree>>),
    LiteralElement(::std::boxed::Box<LiteralElement<'tree>>),
}
impl<'tree> ::treesitter_types::FromNode<'tree> for LiteralValueChildren<'tree> {
    #[allow(clippy::collapsible_else_if)]
    fn from_node(
        node: ::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        match node.kind() {
            "keyed_element" => {
                Ok(
                    Self::KeyedElement(
                        ::std::boxed::Box::new(
                            <KeyedElement as ::treesitter_types::FromNode>::from_node(
                                node,
                                src,
                            )?,
                        ),
                    ),
                )
            }
            "literal_element" => {
                Ok(
                    Self::LiteralElement(
                        ::std::boxed::Box::new(
                            <LiteralElement as ::treesitter_types::FromNode>::from_node(
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
impl ::treesitter_types::Spanned for LiteralValueChildren<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        match self {
            Self::KeyedElement(inner) => inner.span(),
            Self::LiteralElement(inner) => inner.span(),
        }
    }
}
#[derive(Debug, Clone)]
pub enum MethodDeclarationResult<'tree> {
    SimpleType(::std::boxed::Box<SimpleType<'tree>>),
    ParameterList(::std::boxed::Box<ParameterList<'tree>>),
}
impl<'tree> ::treesitter_types::FromNode<'tree> for MethodDeclarationResult<'tree> {
    #[allow(clippy::collapsible_else_if)]
    fn from_node(
        node: ::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        match node.kind() {
            "parameter_list" => {
                Ok(
                    Self::ParameterList(
                        ::std::boxed::Box::new(
                            <ParameterList as ::treesitter_types::FromNode>::from_node(
                                node,
                                src,
                            )?,
                        ),
                    ),
                )
            }
            _other => {
                if let Ok(v) = <SimpleType as ::treesitter_types::FromNode>::from_node(
                    node,
                    src,
                ) {
                    Ok(Self::SimpleType(::std::boxed::Box::new(v)))
                } else {
                    Err(::treesitter_types::ParseError::unexpected_kind(_other, node))
                }
            }
        }
    }
}
impl ::treesitter_types::Spanned for MethodDeclarationResult<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        match self {
            Self::SimpleType(inner) => inner.span(),
            Self::ParameterList(inner) => inner.span(),
        }
    }
}
#[derive(Debug, Clone)]
pub enum MethodElemResult<'tree> {
    SimpleType(::std::boxed::Box<SimpleType<'tree>>),
    ParameterList(::std::boxed::Box<ParameterList<'tree>>),
}
impl<'tree> ::treesitter_types::FromNode<'tree> for MethodElemResult<'tree> {
    #[allow(clippy::collapsible_else_if)]
    fn from_node(
        node: ::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        match node.kind() {
            "parameter_list" => {
                Ok(
                    Self::ParameterList(
                        ::std::boxed::Box::new(
                            <ParameterList as ::treesitter_types::FromNode>::from_node(
                                node,
                                src,
                            )?,
                        ),
                    ),
                )
            }
            _other => {
                if let Ok(v) = <SimpleType as ::treesitter_types::FromNode>::from_node(
                    node,
                    src,
                ) {
                    Ok(Self::SimpleType(::std::boxed::Box::new(v)))
                } else {
                    Err(::treesitter_types::ParseError::unexpected_kind(_other, node))
                }
            }
        }
    }
}
impl ::treesitter_types::Spanned for MethodElemResult<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        match self {
            Self::SimpleType(inner) => inner.span(),
            Self::ParameterList(inner) => inner.span(),
        }
    }
}
#[derive(Debug, Clone)]
pub enum ParameterListChildren<'tree> {
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
            Self::ParameterDeclaration(inner) => inner.span(),
            Self::VariadicParameterDeclaration(inner) => inner.span(),
        }
    }
}
#[derive(Debug, Clone)]
pub enum SelectStatementChildren<'tree> {
    CommunicationCase(::std::boxed::Box<CommunicationCase<'tree>>),
    DefaultCase(::std::boxed::Box<DefaultCase<'tree>>),
}
impl<'tree> ::treesitter_types::FromNode<'tree> for SelectStatementChildren<'tree> {
    #[allow(clippy::collapsible_else_if)]
    fn from_node(
        node: ::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        match node.kind() {
            "communication_case" => {
                Ok(
                    Self::CommunicationCase(
                        ::std::boxed::Box::new(
                            <CommunicationCase as ::treesitter_types::FromNode>::from_node(
                                node,
                                src,
                            )?,
                        ),
                    ),
                )
            }
            "default_case" => {
                Ok(
                    Self::DefaultCase(
                        ::std::boxed::Box::new(
                            <DefaultCase as ::treesitter_types::FromNode>::from_node(
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
impl ::treesitter_types::Spanned for SelectStatementChildren<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        match self {
            Self::CommunicationCase(inner) => inner.span(),
            Self::DefaultCase(inner) => inner.span(),
        }
    }
}
#[derive(Debug, Clone)]
pub enum SourceFileChildren<'tree> {
    Statement(::std::boxed::Box<Statement<'tree>>),
    FunctionDeclaration(::std::boxed::Box<FunctionDeclaration<'tree>>),
    ImportDeclaration(::std::boxed::Box<ImportDeclaration<'tree>>),
    MethodDeclaration(::std::boxed::Box<MethodDeclaration<'tree>>),
    PackageClause(::std::boxed::Box<PackageClause<'tree>>),
}
impl<'tree> ::treesitter_types::FromNode<'tree> for SourceFileChildren<'tree> {
    #[allow(clippy::collapsible_else_if)]
    fn from_node(
        node: ::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        match node.kind() {
            "function_declaration" => {
                Ok(
                    Self::FunctionDeclaration(
                        ::std::boxed::Box::new(
                            <FunctionDeclaration as ::treesitter_types::FromNode>::from_node(
                                node,
                                src,
                            )?,
                        ),
                    ),
                )
            }
            "import_declaration" => {
                Ok(
                    Self::ImportDeclaration(
                        ::std::boxed::Box::new(
                            <ImportDeclaration as ::treesitter_types::FromNode>::from_node(
                                node,
                                src,
                            )?,
                        ),
                    ),
                )
            }
            "method_declaration" => {
                Ok(
                    Self::MethodDeclaration(
                        ::std::boxed::Box::new(
                            <MethodDeclaration as ::treesitter_types::FromNode>::from_node(
                                node,
                                src,
                            )?,
                        ),
                    ),
                )
            }
            "package_clause" => {
                Ok(
                    Self::PackageClause(
                        ::std::boxed::Box::new(
                            <PackageClause as ::treesitter_types::FromNode>::from_node(
                                node,
                                src,
                            )?,
                        ),
                    ),
                )
            }
            _other => {
                if let Ok(v) = <Statement as ::treesitter_types::FromNode>::from_node(
                    node,
                    src,
                ) {
                    Ok(Self::Statement(::std::boxed::Box::new(v)))
                } else {
                    Err(::treesitter_types::ParseError::unexpected_kind(_other, node))
                }
            }
        }
    }
}
impl ::treesitter_types::Spanned for SourceFileChildren<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        match self {
            Self::Statement(inner) => inner.span(),
            Self::FunctionDeclaration(inner) => inner.span(),
            Self::ImportDeclaration(inner) => inner.span(),
            Self::MethodDeclaration(inner) => inner.span(),
            Self::PackageClause(inner) => inner.span(),
        }
    }
}
#[derive(Debug, Clone)]
pub enum TypeCaseType<'tree> {
    Comma(::treesitter_types::Span),
    Type(::std::boxed::Box<Type<'tree>>),
}
impl<'tree> ::treesitter_types::FromNode<'tree> for TypeCaseType<'tree> {
    #[allow(clippy::collapsible_else_if)]
    fn from_node(
        node: ::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        match node.kind() {
            "," => Ok(Self::Comma(::treesitter_types::Span::from(node))),
            _other => {
                if let Ok(v) = <Type as ::treesitter_types::FromNode>::from_node(
                    node,
                    src,
                ) {
                    Ok(Self::Type(::std::boxed::Box::new(v)))
                } else {
                    Err(::treesitter_types::ParseError::unexpected_kind(_other, node))
                }
            }
        }
    }
}
impl ::treesitter_types::Spanned for TypeCaseType<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        match self {
            Self::Comma(span) => *span,
            Self::Type(inner) => inner.span(),
        }
    }
}
#[derive(Debug, Clone)]
pub enum TypeDeclarationChildren<'tree> {
    TypeAlias(::std::boxed::Box<TypeAlias<'tree>>),
    TypeSpec(::std::boxed::Box<TypeSpec<'tree>>),
}
impl<'tree> ::treesitter_types::FromNode<'tree> for TypeDeclarationChildren<'tree> {
    #[allow(clippy::collapsible_else_if)]
    fn from_node(
        node: ::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        match node.kind() {
            "type_alias" => {
                Ok(
                    Self::TypeAlias(
                        ::std::boxed::Box::new(
                            <TypeAlias as ::treesitter_types::FromNode>::from_node(
                                node,
                                src,
                            )?,
                        ),
                    ),
                )
            }
            "type_spec" => {
                Ok(
                    Self::TypeSpec(
                        ::std::boxed::Box::new(
                            <TypeSpec as ::treesitter_types::FromNode>::from_node(
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
impl ::treesitter_types::Spanned for TypeDeclarationChildren<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        match self {
            Self::TypeAlias(inner) => inner.span(),
            Self::TypeSpec(inner) => inner.span(),
        }
    }
}
#[derive(Debug, Clone)]
pub enum TypeSwitchStatementChildren<'tree> {
    DefaultCase(::std::boxed::Box<DefaultCase<'tree>>),
    TypeCase(::std::boxed::Box<TypeCase<'tree>>),
}
impl<'tree> ::treesitter_types::FromNode<'tree> for TypeSwitchStatementChildren<'tree> {
    #[allow(clippy::collapsible_else_if)]
    fn from_node(
        node: ::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        match node.kind() {
            "default_case" => {
                Ok(
                    Self::DefaultCase(
                        ::std::boxed::Box::new(
                            <DefaultCase as ::treesitter_types::FromNode>::from_node(
                                node,
                                src,
                            )?,
                        ),
                    ),
                )
            }
            "type_case" => {
                Ok(
                    Self::TypeCase(
                        ::std::boxed::Box::new(
                            <TypeCase as ::treesitter_types::FromNode>::from_node(
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
impl ::treesitter_types::Spanned for TypeSwitchStatementChildren<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        match self {
            Self::DefaultCase(inner) => inner.span(),
            Self::TypeCase(inner) => inner.span(),
        }
    }
}
#[derive(Debug, Clone)]
pub enum UnaryExpressionOperator {
    Bang(::treesitter_types::Span),
    Amp(::treesitter_types::Span),
    Star(::treesitter_types::Span),
    Plus(::treesitter_types::Span),
    Minus(::treesitter_types::Span),
    LArrow(::treesitter_types::Span),
    Caret(::treesitter_types::Span),
}
impl<'tree> ::treesitter_types::FromNode<'tree> for UnaryExpressionOperator {
    #[allow(clippy::collapsible_else_if)]
    fn from_node(
        node: ::tree_sitter::Node<'tree>,
        _src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        match node.kind() {
            "!" => Ok(Self::Bang(::treesitter_types::Span::from(node))),
            "&" => Ok(Self::Amp(::treesitter_types::Span::from(node))),
            "*" => Ok(Self::Star(::treesitter_types::Span::from(node))),
            "+" => Ok(Self::Plus(::treesitter_types::Span::from(node))),
            "-" => Ok(Self::Minus(::treesitter_types::Span::from(node))),
            "<-" => Ok(Self::LArrow(::treesitter_types::Span::from(node))),
            "^" => Ok(Self::Caret(::treesitter_types::Span::from(node))),
            other => Err(::treesitter_types::ParseError::unexpected_kind(other, node)),
        }
    }
}
impl ::treesitter_types::Spanned for UnaryExpressionOperator {
    fn span(&self) -> ::treesitter_types::Span {
        match self {
            Self::Bang(span) => *span,
            Self::Amp(span) => *span,
            Self::Star(span) => *span,
            Self::Plus(span) => *span,
            Self::Minus(span) => *span,
            Self::LArrow(span) => *span,
            Self::Caret(span) => *span,
        }
    }
}
#[derive(Debug, Clone)]
pub enum VarDeclarationChildren<'tree> {
    VarSpec(::std::boxed::Box<VarSpec<'tree>>),
    VarSpecList(::std::boxed::Box<VarSpecList<'tree>>),
}
impl<'tree> ::treesitter_types::FromNode<'tree> for VarDeclarationChildren<'tree> {
    #[allow(clippy::collapsible_else_if)]
    fn from_node(
        node: ::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        match node.kind() {
            "var_spec" => {
                Ok(
                    Self::VarSpec(
                        ::std::boxed::Box::new(
                            <VarSpec as ::treesitter_types::FromNode>::from_node(
                                node,
                                src,
                            )?,
                        ),
                    ),
                )
            }
            "var_spec_list" => {
                Ok(
                    Self::VarSpecList(
                        ::std::boxed::Box::new(
                            <VarSpecList as ::treesitter_types::FromNode>::from_node(
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
impl ::treesitter_types::Spanned for VarDeclarationChildren<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        match self {
            Self::VarSpec(inner) => inner.span(),
            Self::VarSpecList(inner) => inner.span(),
        }
    }
}
#[derive(Debug, Clone)]
pub enum AnyNode<'tree> {
    Expression(Expression<'tree>),
    SimpleStatement(SimpleStatement<'tree>),
    SimpleType(SimpleType<'tree>),
    Statement(Statement<'tree>),
    Type(Type<'tree>),
    ArgumentList(ArgumentList<'tree>),
    ArrayType(ArrayType<'tree>),
    AssignmentStatement(AssignmentStatement<'tree>),
    BinaryExpression(BinaryExpression<'tree>),
    Block(Block<'tree>),
    BreakStatement(BreakStatement<'tree>),
    CallExpression(CallExpression<'tree>),
    ChannelType(ChannelType<'tree>),
    CommunicationCase(CommunicationCase<'tree>),
    CompositeLiteral(CompositeLiteral<'tree>),
    ConstDeclaration(ConstDeclaration<'tree>),
    ConstSpec(ConstSpec<'tree>),
    ContinueStatement(ContinueStatement<'tree>),
    DecStatement(DecStatement<'tree>),
    DefaultCase(DefaultCase<'tree>),
    DeferStatement(DeferStatement<'tree>),
    Dot(Dot<'tree>),
    EmptyStatement(EmptyStatement<'tree>),
    ExpressionCase(ExpressionCase<'tree>),
    ExpressionList(ExpressionList<'tree>),
    ExpressionStatement(ExpressionStatement<'tree>),
    ExpressionSwitchStatement(ExpressionSwitchStatement<'tree>),
    FallthroughStatement(FallthroughStatement<'tree>),
    FieldDeclaration(FieldDeclaration<'tree>),
    FieldDeclarationList(FieldDeclarationList<'tree>),
    ForClause(ForClause<'tree>),
    ForStatement(ForStatement<'tree>),
    FuncLiteral(FuncLiteral<'tree>),
    FunctionDeclaration(FunctionDeclaration<'tree>),
    FunctionType(FunctionType<'tree>),
    GenericType(GenericType<'tree>),
    GoStatement(GoStatement<'tree>),
    GotoStatement(GotoStatement<'tree>),
    IfStatement(IfStatement<'tree>),
    ImplicitLengthArrayType(ImplicitLengthArrayType<'tree>),
    ImportDeclaration(ImportDeclaration<'tree>),
    ImportSpec(ImportSpec<'tree>),
    ImportSpecList(ImportSpecList<'tree>),
    IncStatement(IncStatement<'tree>),
    IndexExpression(IndexExpression<'tree>),
    InterfaceType(InterfaceType<'tree>),
    InterpretedStringLiteral(InterpretedStringLiteral<'tree>),
    KeyedElement(KeyedElement<'tree>),
    LabeledStatement(LabeledStatement<'tree>),
    LiteralElement(LiteralElement<'tree>),
    LiteralValue(LiteralValue<'tree>),
    MapType(MapType<'tree>),
    MethodDeclaration(MethodDeclaration<'tree>),
    MethodElem(MethodElem<'tree>),
    NegatedType(NegatedType<'tree>),
    PackageClause(PackageClause<'tree>),
    ParameterDeclaration(ParameterDeclaration<'tree>),
    ParameterList(ParameterList<'tree>),
    ParenthesizedExpression(ParenthesizedExpression<'tree>),
    ParenthesizedType(ParenthesizedType<'tree>),
    PointerType(PointerType<'tree>),
    QualifiedType(QualifiedType<'tree>),
    RangeClause(RangeClause<'tree>),
    RawStringLiteral(RawStringLiteral<'tree>),
    ReceiveStatement(ReceiveStatement<'tree>),
    ReturnStatement(ReturnStatement<'tree>),
    SelectStatement(SelectStatement<'tree>),
    SelectorExpression(SelectorExpression<'tree>),
    SendStatement(SendStatement<'tree>),
    ShortVarDeclaration(ShortVarDeclaration<'tree>),
    SliceExpression(SliceExpression<'tree>),
    SliceType(SliceType<'tree>),
    SourceFile(SourceFile<'tree>),
    StatementList(StatementList<'tree>),
    StructType(StructType<'tree>),
    TypeAlias(TypeAlias<'tree>),
    TypeArguments(TypeArguments<'tree>),
    TypeAssertionExpression(TypeAssertionExpression<'tree>),
    TypeCase(TypeCase<'tree>),
    TypeConstraint(TypeConstraint<'tree>),
    TypeConversionExpression(TypeConversionExpression<'tree>),
    TypeDeclaration(TypeDeclaration<'tree>),
    TypeElem(TypeElem<'tree>),
    TypeInstantiationExpression(TypeInstantiationExpression<'tree>),
    TypeParameterDeclaration(TypeParameterDeclaration<'tree>),
    TypeParameterList(TypeParameterList<'tree>),
    TypeSpec(TypeSpec<'tree>),
    TypeSwitchStatement(TypeSwitchStatement<'tree>),
    UnaryExpression(UnaryExpression<'tree>),
    VarDeclaration(VarDeclaration<'tree>),
    VarSpec(VarSpec<'tree>),
    VarSpecList(VarSpecList<'tree>),
    VariadicArgument(VariadicArgument<'tree>),
    VariadicParameterDeclaration(VariadicParameterDeclaration<'tree>),
    BlankIdentifier(BlankIdentifier<'tree>),
    Comment(Comment<'tree>),
    EscapeSequence(EscapeSequence<'tree>),
    False(False<'tree>),
    FieldIdentifier(FieldIdentifier<'tree>),
    FloatLiteral(FloatLiteral<'tree>),
    Identifier(Identifier<'tree>),
    ImaginaryLiteral(ImaginaryLiteral<'tree>),
    IntLiteral(IntLiteral<'tree>),
    InterpretedStringLiteralContent(InterpretedStringLiteralContent<'tree>),
    Iota(Iota<'tree>),
    LabelName(LabelName<'tree>),
    Nil(Nil<'tree>),
    PackageIdentifier(PackageIdentifier<'tree>),
    RawStringLiteralContent(RawStringLiteralContent<'tree>),
    RuneLiteral(RuneLiteral<'tree>),
    True(True<'tree>),
    TypeIdentifier(TypeIdentifier<'tree>),
    Unknown(::tree_sitter::Node<'tree>),
}
impl<'tree> AnyNode<'tree> {
    pub fn from_node(node: ::tree_sitter::Node<'tree>, src: &'tree [u8]) -> Self {
        match node.kind() {
            "_expression" => {
                <Expression as ::treesitter_types::FromNode>::from_node(node, src)
                    .map(Self::Expression)
                    .unwrap_or(Self::Unknown(node))
            }
            "_simple_statement" => {
                <SimpleStatement as ::treesitter_types::FromNode>::from_node(node, src)
                    .map(Self::SimpleStatement)
                    .unwrap_or(Self::Unknown(node))
            }
            "_simple_type" => {
                <SimpleType as ::treesitter_types::FromNode>::from_node(node, src)
                    .map(Self::SimpleType)
                    .unwrap_or(Self::Unknown(node))
            }
            "_statement" => {
                <Statement as ::treesitter_types::FromNode>::from_node(node, src)
                    .map(Self::Statement)
                    .unwrap_or(Self::Unknown(node))
            }
            "_type" => {
                <Type as ::treesitter_types::FromNode>::from_node(node, src)
                    .map(Self::Type)
                    .unwrap_or(Self::Unknown(node))
            }
            "argument_list" => {
                <ArgumentList as ::treesitter_types::FromNode>::from_node(node, src)
                    .map(Self::ArgumentList)
                    .unwrap_or(Self::Unknown(node))
            }
            "array_type" => {
                <ArrayType as ::treesitter_types::FromNode>::from_node(node, src)
                    .map(Self::ArrayType)
                    .unwrap_or(Self::Unknown(node))
            }
            "assignment_statement" => {
                <AssignmentStatement as ::treesitter_types::FromNode>::from_node(
                        node,
                        src,
                    )
                    .map(Self::AssignmentStatement)
                    .unwrap_or(Self::Unknown(node))
            }
            "binary_expression" => {
                <BinaryExpression as ::treesitter_types::FromNode>::from_node(node, src)
                    .map(Self::BinaryExpression)
                    .unwrap_or(Self::Unknown(node))
            }
            "block" => {
                <Block as ::treesitter_types::FromNode>::from_node(node, src)
                    .map(Self::Block)
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
            "channel_type" => {
                <ChannelType as ::treesitter_types::FromNode>::from_node(node, src)
                    .map(Self::ChannelType)
                    .unwrap_or(Self::Unknown(node))
            }
            "communication_case" => {
                <CommunicationCase as ::treesitter_types::FromNode>::from_node(node, src)
                    .map(Self::CommunicationCase)
                    .unwrap_or(Self::Unknown(node))
            }
            "composite_literal" => {
                <CompositeLiteral as ::treesitter_types::FromNode>::from_node(node, src)
                    .map(Self::CompositeLiteral)
                    .unwrap_or(Self::Unknown(node))
            }
            "const_declaration" => {
                <ConstDeclaration as ::treesitter_types::FromNode>::from_node(node, src)
                    .map(Self::ConstDeclaration)
                    .unwrap_or(Self::Unknown(node))
            }
            "const_spec" => {
                <ConstSpec as ::treesitter_types::FromNode>::from_node(node, src)
                    .map(Self::ConstSpec)
                    .unwrap_or(Self::Unknown(node))
            }
            "continue_statement" => {
                <ContinueStatement as ::treesitter_types::FromNode>::from_node(node, src)
                    .map(Self::ContinueStatement)
                    .unwrap_or(Self::Unknown(node))
            }
            "dec_statement" => {
                <DecStatement as ::treesitter_types::FromNode>::from_node(node, src)
                    .map(Self::DecStatement)
                    .unwrap_or(Self::Unknown(node))
            }
            "default_case" => {
                <DefaultCase as ::treesitter_types::FromNode>::from_node(node, src)
                    .map(Self::DefaultCase)
                    .unwrap_or(Self::Unknown(node))
            }
            "defer_statement" => {
                <DeferStatement as ::treesitter_types::FromNode>::from_node(node, src)
                    .map(Self::DeferStatement)
                    .unwrap_or(Self::Unknown(node))
            }
            "dot" => {
                <Dot as ::treesitter_types::FromNode>::from_node(node, src)
                    .map(Self::Dot)
                    .unwrap_or(Self::Unknown(node))
            }
            "empty_statement" => {
                <EmptyStatement as ::treesitter_types::FromNode>::from_node(node, src)
                    .map(Self::EmptyStatement)
                    .unwrap_or(Self::Unknown(node))
            }
            "expression_case" => {
                <ExpressionCase as ::treesitter_types::FromNode>::from_node(node, src)
                    .map(Self::ExpressionCase)
                    .unwrap_or(Self::Unknown(node))
            }
            "expression_list" => {
                <ExpressionList as ::treesitter_types::FromNode>::from_node(node, src)
                    .map(Self::ExpressionList)
                    .unwrap_or(Self::Unknown(node))
            }
            "expression_statement" => {
                <ExpressionStatement as ::treesitter_types::FromNode>::from_node(
                        node,
                        src,
                    )
                    .map(Self::ExpressionStatement)
                    .unwrap_or(Self::Unknown(node))
            }
            "expression_switch_statement" => {
                <ExpressionSwitchStatement as ::treesitter_types::FromNode>::from_node(
                        node,
                        src,
                    )
                    .map(Self::ExpressionSwitchStatement)
                    .unwrap_or(Self::Unknown(node))
            }
            "fallthrough_statement" => {
                <FallthroughStatement as ::treesitter_types::FromNode>::from_node(
                        node,
                        src,
                    )
                    .map(Self::FallthroughStatement)
                    .unwrap_or(Self::Unknown(node))
            }
            "field_declaration" => {
                <FieldDeclaration as ::treesitter_types::FromNode>::from_node(node, src)
                    .map(Self::FieldDeclaration)
                    .unwrap_or(Self::Unknown(node))
            }
            "field_declaration_list" => {
                <FieldDeclarationList as ::treesitter_types::FromNode>::from_node(
                        node,
                        src,
                    )
                    .map(Self::FieldDeclarationList)
                    .unwrap_or(Self::Unknown(node))
            }
            "for_clause" => {
                <ForClause as ::treesitter_types::FromNode>::from_node(node, src)
                    .map(Self::ForClause)
                    .unwrap_or(Self::Unknown(node))
            }
            "for_statement" => {
                <ForStatement as ::treesitter_types::FromNode>::from_node(node, src)
                    .map(Self::ForStatement)
                    .unwrap_or(Self::Unknown(node))
            }
            "func_literal" => {
                <FuncLiteral as ::treesitter_types::FromNode>::from_node(node, src)
                    .map(Self::FuncLiteral)
                    .unwrap_or(Self::Unknown(node))
            }
            "function_declaration" => {
                <FunctionDeclaration as ::treesitter_types::FromNode>::from_node(
                        node,
                        src,
                    )
                    .map(Self::FunctionDeclaration)
                    .unwrap_or(Self::Unknown(node))
            }
            "function_type" => {
                <FunctionType as ::treesitter_types::FromNode>::from_node(node, src)
                    .map(Self::FunctionType)
                    .unwrap_or(Self::Unknown(node))
            }
            "generic_type" => {
                <GenericType as ::treesitter_types::FromNode>::from_node(node, src)
                    .map(Self::GenericType)
                    .unwrap_or(Self::Unknown(node))
            }
            "go_statement" => {
                <GoStatement as ::treesitter_types::FromNode>::from_node(node, src)
                    .map(Self::GoStatement)
                    .unwrap_or(Self::Unknown(node))
            }
            "goto_statement" => {
                <GotoStatement as ::treesitter_types::FromNode>::from_node(node, src)
                    .map(Self::GotoStatement)
                    .unwrap_or(Self::Unknown(node))
            }
            "if_statement" => {
                <IfStatement as ::treesitter_types::FromNode>::from_node(node, src)
                    .map(Self::IfStatement)
                    .unwrap_or(Self::Unknown(node))
            }
            "implicit_length_array_type" => {
                <ImplicitLengthArrayType as ::treesitter_types::FromNode>::from_node(
                        node,
                        src,
                    )
                    .map(Self::ImplicitLengthArrayType)
                    .unwrap_or(Self::Unknown(node))
            }
            "import_declaration" => {
                <ImportDeclaration as ::treesitter_types::FromNode>::from_node(node, src)
                    .map(Self::ImportDeclaration)
                    .unwrap_or(Self::Unknown(node))
            }
            "import_spec" => {
                <ImportSpec as ::treesitter_types::FromNode>::from_node(node, src)
                    .map(Self::ImportSpec)
                    .unwrap_or(Self::Unknown(node))
            }
            "import_spec_list" => {
                <ImportSpecList as ::treesitter_types::FromNode>::from_node(node, src)
                    .map(Self::ImportSpecList)
                    .unwrap_or(Self::Unknown(node))
            }
            "inc_statement" => {
                <IncStatement as ::treesitter_types::FromNode>::from_node(node, src)
                    .map(Self::IncStatement)
                    .unwrap_or(Self::Unknown(node))
            }
            "index_expression" => {
                <IndexExpression as ::treesitter_types::FromNode>::from_node(node, src)
                    .map(Self::IndexExpression)
                    .unwrap_or(Self::Unknown(node))
            }
            "interface_type" => {
                <InterfaceType as ::treesitter_types::FromNode>::from_node(node, src)
                    .map(Self::InterfaceType)
                    .unwrap_or(Self::Unknown(node))
            }
            "interpreted_string_literal" => {
                <InterpretedStringLiteral as ::treesitter_types::FromNode>::from_node(
                        node,
                        src,
                    )
                    .map(Self::InterpretedStringLiteral)
                    .unwrap_or(Self::Unknown(node))
            }
            "keyed_element" => {
                <KeyedElement as ::treesitter_types::FromNode>::from_node(node, src)
                    .map(Self::KeyedElement)
                    .unwrap_or(Self::Unknown(node))
            }
            "labeled_statement" => {
                <LabeledStatement as ::treesitter_types::FromNode>::from_node(node, src)
                    .map(Self::LabeledStatement)
                    .unwrap_or(Self::Unknown(node))
            }
            "literal_element" => {
                <LiteralElement as ::treesitter_types::FromNode>::from_node(node, src)
                    .map(Self::LiteralElement)
                    .unwrap_or(Self::Unknown(node))
            }
            "literal_value" => {
                <LiteralValue as ::treesitter_types::FromNode>::from_node(node, src)
                    .map(Self::LiteralValue)
                    .unwrap_or(Self::Unknown(node))
            }
            "map_type" => {
                <MapType as ::treesitter_types::FromNode>::from_node(node, src)
                    .map(Self::MapType)
                    .unwrap_or(Self::Unknown(node))
            }
            "method_declaration" => {
                <MethodDeclaration as ::treesitter_types::FromNode>::from_node(node, src)
                    .map(Self::MethodDeclaration)
                    .unwrap_or(Self::Unknown(node))
            }
            "method_elem" => {
                <MethodElem as ::treesitter_types::FromNode>::from_node(node, src)
                    .map(Self::MethodElem)
                    .unwrap_or(Self::Unknown(node))
            }
            "negated_type" => {
                <NegatedType as ::treesitter_types::FromNode>::from_node(node, src)
                    .map(Self::NegatedType)
                    .unwrap_or(Self::Unknown(node))
            }
            "package_clause" => {
                <PackageClause as ::treesitter_types::FromNode>::from_node(node, src)
                    .map(Self::PackageClause)
                    .unwrap_or(Self::Unknown(node))
            }
            "parameter_declaration" => {
                <ParameterDeclaration as ::treesitter_types::FromNode>::from_node(
                        node,
                        src,
                    )
                    .map(Self::ParameterDeclaration)
                    .unwrap_or(Self::Unknown(node))
            }
            "parameter_list" => {
                <ParameterList as ::treesitter_types::FromNode>::from_node(node, src)
                    .map(Self::ParameterList)
                    .unwrap_or(Self::Unknown(node))
            }
            "parenthesized_expression" => {
                <ParenthesizedExpression as ::treesitter_types::FromNode>::from_node(
                        node,
                        src,
                    )
                    .map(Self::ParenthesizedExpression)
                    .unwrap_or(Self::Unknown(node))
            }
            "parenthesized_type" => {
                <ParenthesizedType as ::treesitter_types::FromNode>::from_node(node, src)
                    .map(Self::ParenthesizedType)
                    .unwrap_or(Self::Unknown(node))
            }
            "pointer_type" => {
                <PointerType as ::treesitter_types::FromNode>::from_node(node, src)
                    .map(Self::PointerType)
                    .unwrap_or(Self::Unknown(node))
            }
            "qualified_type" => {
                <QualifiedType as ::treesitter_types::FromNode>::from_node(node, src)
                    .map(Self::QualifiedType)
                    .unwrap_or(Self::Unknown(node))
            }
            "range_clause" => {
                <RangeClause as ::treesitter_types::FromNode>::from_node(node, src)
                    .map(Self::RangeClause)
                    .unwrap_or(Self::Unknown(node))
            }
            "raw_string_literal" => {
                <RawStringLiteral as ::treesitter_types::FromNode>::from_node(node, src)
                    .map(Self::RawStringLiteral)
                    .unwrap_or(Self::Unknown(node))
            }
            "receive_statement" => {
                <ReceiveStatement as ::treesitter_types::FromNode>::from_node(node, src)
                    .map(Self::ReceiveStatement)
                    .unwrap_or(Self::Unknown(node))
            }
            "return_statement" => {
                <ReturnStatement as ::treesitter_types::FromNode>::from_node(node, src)
                    .map(Self::ReturnStatement)
                    .unwrap_or(Self::Unknown(node))
            }
            "select_statement" => {
                <SelectStatement as ::treesitter_types::FromNode>::from_node(node, src)
                    .map(Self::SelectStatement)
                    .unwrap_or(Self::Unknown(node))
            }
            "selector_expression" => {
                <SelectorExpression as ::treesitter_types::FromNode>::from_node(
                        node,
                        src,
                    )
                    .map(Self::SelectorExpression)
                    .unwrap_or(Self::Unknown(node))
            }
            "send_statement" => {
                <SendStatement as ::treesitter_types::FromNode>::from_node(node, src)
                    .map(Self::SendStatement)
                    .unwrap_or(Self::Unknown(node))
            }
            "short_var_declaration" => {
                <ShortVarDeclaration as ::treesitter_types::FromNode>::from_node(
                        node,
                        src,
                    )
                    .map(Self::ShortVarDeclaration)
                    .unwrap_or(Self::Unknown(node))
            }
            "slice_expression" => {
                <SliceExpression as ::treesitter_types::FromNode>::from_node(node, src)
                    .map(Self::SliceExpression)
                    .unwrap_or(Self::Unknown(node))
            }
            "slice_type" => {
                <SliceType as ::treesitter_types::FromNode>::from_node(node, src)
                    .map(Self::SliceType)
                    .unwrap_or(Self::Unknown(node))
            }
            "source_file" => {
                <SourceFile as ::treesitter_types::FromNode>::from_node(node, src)
                    .map(Self::SourceFile)
                    .unwrap_or(Self::Unknown(node))
            }
            "statement_list" => {
                <StatementList as ::treesitter_types::FromNode>::from_node(node, src)
                    .map(Self::StatementList)
                    .unwrap_or(Self::Unknown(node))
            }
            "struct_type" => {
                <StructType as ::treesitter_types::FromNode>::from_node(node, src)
                    .map(Self::StructType)
                    .unwrap_or(Self::Unknown(node))
            }
            "type_alias" => {
                <TypeAlias as ::treesitter_types::FromNode>::from_node(node, src)
                    .map(Self::TypeAlias)
                    .unwrap_or(Self::Unknown(node))
            }
            "type_arguments" => {
                <TypeArguments as ::treesitter_types::FromNode>::from_node(node, src)
                    .map(Self::TypeArguments)
                    .unwrap_or(Self::Unknown(node))
            }
            "type_assertion_expression" => {
                <TypeAssertionExpression as ::treesitter_types::FromNode>::from_node(
                        node,
                        src,
                    )
                    .map(Self::TypeAssertionExpression)
                    .unwrap_or(Self::Unknown(node))
            }
            "type_case" => {
                <TypeCase as ::treesitter_types::FromNode>::from_node(node, src)
                    .map(Self::TypeCase)
                    .unwrap_or(Self::Unknown(node))
            }
            "type_constraint" => {
                <TypeConstraint as ::treesitter_types::FromNode>::from_node(node, src)
                    .map(Self::TypeConstraint)
                    .unwrap_or(Self::Unknown(node))
            }
            "type_conversion_expression" => {
                <TypeConversionExpression as ::treesitter_types::FromNode>::from_node(
                        node,
                        src,
                    )
                    .map(Self::TypeConversionExpression)
                    .unwrap_or(Self::Unknown(node))
            }
            "type_declaration" => {
                <TypeDeclaration as ::treesitter_types::FromNode>::from_node(node, src)
                    .map(Self::TypeDeclaration)
                    .unwrap_or(Self::Unknown(node))
            }
            "type_elem" => {
                <TypeElem as ::treesitter_types::FromNode>::from_node(node, src)
                    .map(Self::TypeElem)
                    .unwrap_or(Self::Unknown(node))
            }
            "type_instantiation_expression" => {
                <TypeInstantiationExpression as ::treesitter_types::FromNode>::from_node(
                        node,
                        src,
                    )
                    .map(Self::TypeInstantiationExpression)
                    .unwrap_or(Self::Unknown(node))
            }
            "type_parameter_declaration" => {
                <TypeParameterDeclaration as ::treesitter_types::FromNode>::from_node(
                        node,
                        src,
                    )
                    .map(Self::TypeParameterDeclaration)
                    .unwrap_or(Self::Unknown(node))
            }
            "type_parameter_list" => {
                <TypeParameterList as ::treesitter_types::FromNode>::from_node(node, src)
                    .map(Self::TypeParameterList)
                    .unwrap_or(Self::Unknown(node))
            }
            "type_spec" => {
                <TypeSpec as ::treesitter_types::FromNode>::from_node(node, src)
                    .map(Self::TypeSpec)
                    .unwrap_or(Self::Unknown(node))
            }
            "type_switch_statement" => {
                <TypeSwitchStatement as ::treesitter_types::FromNode>::from_node(
                        node,
                        src,
                    )
                    .map(Self::TypeSwitchStatement)
                    .unwrap_or(Self::Unknown(node))
            }
            "unary_expression" => {
                <UnaryExpression as ::treesitter_types::FromNode>::from_node(node, src)
                    .map(Self::UnaryExpression)
                    .unwrap_or(Self::Unknown(node))
            }
            "var_declaration" => {
                <VarDeclaration as ::treesitter_types::FromNode>::from_node(node, src)
                    .map(Self::VarDeclaration)
                    .unwrap_or(Self::Unknown(node))
            }
            "var_spec" => {
                <VarSpec as ::treesitter_types::FromNode>::from_node(node, src)
                    .map(Self::VarSpec)
                    .unwrap_or(Self::Unknown(node))
            }
            "var_spec_list" => {
                <VarSpecList as ::treesitter_types::FromNode>::from_node(node, src)
                    .map(Self::VarSpecList)
                    .unwrap_or(Self::Unknown(node))
            }
            "variadic_argument" => {
                <VariadicArgument as ::treesitter_types::FromNode>::from_node(node, src)
                    .map(Self::VariadicArgument)
                    .unwrap_or(Self::Unknown(node))
            }
            "variadic_parameter_declaration" => {
                <VariadicParameterDeclaration as ::treesitter_types::FromNode>::from_node(
                        node,
                        src,
                    )
                    .map(Self::VariadicParameterDeclaration)
                    .unwrap_or(Self::Unknown(node))
            }
            "blank_identifier" => {
                <BlankIdentifier as ::treesitter_types::FromNode>::from_node(node, src)
                    .map(Self::BlankIdentifier)
                    .unwrap_or(Self::Unknown(node))
            }
            "comment" => {
                <Comment as ::treesitter_types::FromNode>::from_node(node, src)
                    .map(Self::Comment)
                    .unwrap_or(Self::Unknown(node))
            }
            "escape_sequence" => {
                <EscapeSequence as ::treesitter_types::FromNode>::from_node(node, src)
                    .map(Self::EscapeSequence)
                    .unwrap_or(Self::Unknown(node))
            }
            "false" => {
                <False as ::treesitter_types::FromNode>::from_node(node, src)
                    .map(Self::False)
                    .unwrap_or(Self::Unknown(node))
            }
            "field_identifier" => {
                <FieldIdentifier as ::treesitter_types::FromNode>::from_node(node, src)
                    .map(Self::FieldIdentifier)
                    .unwrap_or(Self::Unknown(node))
            }
            "float_literal" => {
                <FloatLiteral as ::treesitter_types::FromNode>::from_node(node, src)
                    .map(Self::FloatLiteral)
                    .unwrap_or(Self::Unknown(node))
            }
            "identifier" => {
                <Identifier as ::treesitter_types::FromNode>::from_node(node, src)
                    .map(Self::Identifier)
                    .unwrap_or(Self::Unknown(node))
            }
            "imaginary_literal" => {
                <ImaginaryLiteral as ::treesitter_types::FromNode>::from_node(node, src)
                    .map(Self::ImaginaryLiteral)
                    .unwrap_or(Self::Unknown(node))
            }
            "int_literal" => {
                <IntLiteral as ::treesitter_types::FromNode>::from_node(node, src)
                    .map(Self::IntLiteral)
                    .unwrap_or(Self::Unknown(node))
            }
            "interpreted_string_literal_content" => {
                <InterpretedStringLiteralContent as ::treesitter_types::FromNode>::from_node(
                        node,
                        src,
                    )
                    .map(Self::InterpretedStringLiteralContent)
                    .unwrap_or(Self::Unknown(node))
            }
            "iota" => {
                <Iota as ::treesitter_types::FromNode>::from_node(node, src)
                    .map(Self::Iota)
                    .unwrap_or(Self::Unknown(node))
            }
            "label_name" => {
                <LabelName as ::treesitter_types::FromNode>::from_node(node, src)
                    .map(Self::LabelName)
                    .unwrap_or(Self::Unknown(node))
            }
            "nil" => {
                <Nil as ::treesitter_types::FromNode>::from_node(node, src)
                    .map(Self::Nil)
                    .unwrap_or(Self::Unknown(node))
            }
            "package_identifier" => {
                <PackageIdentifier as ::treesitter_types::FromNode>::from_node(node, src)
                    .map(Self::PackageIdentifier)
                    .unwrap_or(Self::Unknown(node))
            }
            "raw_string_literal_content" => {
                <RawStringLiteralContent as ::treesitter_types::FromNode>::from_node(
                        node,
                        src,
                    )
                    .map(Self::RawStringLiteralContent)
                    .unwrap_or(Self::Unknown(node))
            }
            "rune_literal" => {
                <RuneLiteral as ::treesitter_types::FromNode>::from_node(node, src)
                    .map(Self::RuneLiteral)
                    .unwrap_or(Self::Unknown(node))
            }
            "true" => {
                <True as ::treesitter_types::FromNode>::from_node(node, src)
                    .map(Self::True)
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
            Self::Expression(inner) => inner.span(),
            Self::SimpleStatement(inner) => inner.span(),
            Self::SimpleType(inner) => inner.span(),
            Self::Statement(inner) => inner.span(),
            Self::Type(inner) => inner.span(),
            Self::ArgumentList(inner) => inner.span(),
            Self::ArrayType(inner) => inner.span(),
            Self::AssignmentStatement(inner) => inner.span(),
            Self::BinaryExpression(inner) => inner.span(),
            Self::Block(inner) => inner.span(),
            Self::BreakStatement(inner) => inner.span(),
            Self::CallExpression(inner) => inner.span(),
            Self::ChannelType(inner) => inner.span(),
            Self::CommunicationCase(inner) => inner.span(),
            Self::CompositeLiteral(inner) => inner.span(),
            Self::ConstDeclaration(inner) => inner.span(),
            Self::ConstSpec(inner) => inner.span(),
            Self::ContinueStatement(inner) => inner.span(),
            Self::DecStatement(inner) => inner.span(),
            Self::DefaultCase(inner) => inner.span(),
            Self::DeferStatement(inner) => inner.span(),
            Self::Dot(inner) => inner.span(),
            Self::EmptyStatement(inner) => inner.span(),
            Self::ExpressionCase(inner) => inner.span(),
            Self::ExpressionList(inner) => inner.span(),
            Self::ExpressionStatement(inner) => inner.span(),
            Self::ExpressionSwitchStatement(inner) => inner.span(),
            Self::FallthroughStatement(inner) => inner.span(),
            Self::FieldDeclaration(inner) => inner.span(),
            Self::FieldDeclarationList(inner) => inner.span(),
            Self::ForClause(inner) => inner.span(),
            Self::ForStatement(inner) => inner.span(),
            Self::FuncLiteral(inner) => inner.span(),
            Self::FunctionDeclaration(inner) => inner.span(),
            Self::FunctionType(inner) => inner.span(),
            Self::GenericType(inner) => inner.span(),
            Self::GoStatement(inner) => inner.span(),
            Self::GotoStatement(inner) => inner.span(),
            Self::IfStatement(inner) => inner.span(),
            Self::ImplicitLengthArrayType(inner) => inner.span(),
            Self::ImportDeclaration(inner) => inner.span(),
            Self::ImportSpec(inner) => inner.span(),
            Self::ImportSpecList(inner) => inner.span(),
            Self::IncStatement(inner) => inner.span(),
            Self::IndexExpression(inner) => inner.span(),
            Self::InterfaceType(inner) => inner.span(),
            Self::InterpretedStringLiteral(inner) => inner.span(),
            Self::KeyedElement(inner) => inner.span(),
            Self::LabeledStatement(inner) => inner.span(),
            Self::LiteralElement(inner) => inner.span(),
            Self::LiteralValue(inner) => inner.span(),
            Self::MapType(inner) => inner.span(),
            Self::MethodDeclaration(inner) => inner.span(),
            Self::MethodElem(inner) => inner.span(),
            Self::NegatedType(inner) => inner.span(),
            Self::PackageClause(inner) => inner.span(),
            Self::ParameterDeclaration(inner) => inner.span(),
            Self::ParameterList(inner) => inner.span(),
            Self::ParenthesizedExpression(inner) => inner.span(),
            Self::ParenthesizedType(inner) => inner.span(),
            Self::PointerType(inner) => inner.span(),
            Self::QualifiedType(inner) => inner.span(),
            Self::RangeClause(inner) => inner.span(),
            Self::RawStringLiteral(inner) => inner.span(),
            Self::ReceiveStatement(inner) => inner.span(),
            Self::ReturnStatement(inner) => inner.span(),
            Self::SelectStatement(inner) => inner.span(),
            Self::SelectorExpression(inner) => inner.span(),
            Self::SendStatement(inner) => inner.span(),
            Self::ShortVarDeclaration(inner) => inner.span(),
            Self::SliceExpression(inner) => inner.span(),
            Self::SliceType(inner) => inner.span(),
            Self::SourceFile(inner) => inner.span(),
            Self::StatementList(inner) => inner.span(),
            Self::StructType(inner) => inner.span(),
            Self::TypeAlias(inner) => inner.span(),
            Self::TypeArguments(inner) => inner.span(),
            Self::TypeAssertionExpression(inner) => inner.span(),
            Self::TypeCase(inner) => inner.span(),
            Self::TypeConstraint(inner) => inner.span(),
            Self::TypeConversionExpression(inner) => inner.span(),
            Self::TypeDeclaration(inner) => inner.span(),
            Self::TypeElem(inner) => inner.span(),
            Self::TypeInstantiationExpression(inner) => inner.span(),
            Self::TypeParameterDeclaration(inner) => inner.span(),
            Self::TypeParameterList(inner) => inner.span(),
            Self::TypeSpec(inner) => inner.span(),
            Self::TypeSwitchStatement(inner) => inner.span(),
            Self::UnaryExpression(inner) => inner.span(),
            Self::VarDeclaration(inner) => inner.span(),
            Self::VarSpec(inner) => inner.span(),
            Self::VarSpecList(inner) => inner.span(),
            Self::VariadicArgument(inner) => inner.span(),
            Self::VariadicParameterDeclaration(inner) => inner.span(),
            Self::BlankIdentifier(inner) => inner.span(),
            Self::Comment(inner) => inner.span(),
            Self::EscapeSequence(inner) => inner.span(),
            Self::False(inner) => inner.span(),
            Self::FieldIdentifier(inner) => inner.span(),
            Self::FloatLiteral(inner) => inner.span(),
            Self::Identifier(inner) => inner.span(),
            Self::ImaginaryLiteral(inner) => inner.span(),
            Self::IntLiteral(inner) => inner.span(),
            Self::InterpretedStringLiteralContent(inner) => inner.span(),
            Self::Iota(inner) => inner.span(),
            Self::LabelName(inner) => inner.span(),
            Self::Nil(inner) => inner.span(),
            Self::PackageIdentifier(inner) => inner.span(),
            Self::RawStringLiteralContent(inner) => inner.span(),
            Self::RuneLiteral(inner) => inner.span(),
            Self::True(inner) => inner.span(),
            Self::TypeIdentifier(inner) => inner.span(),
            Self::Unknown(node) => ::treesitter_types::Span::from(*node),
        }
    }
}
