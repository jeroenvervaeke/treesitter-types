#[derive(Debug, Clone, PartialEq, Eq)]
pub enum DeclarationStatement<'tree> {
    AssociatedType(::std::boxed::Box<AssociatedType<'tree>>),
    AttributeItem(::std::boxed::Box<AttributeItem<'tree>>),
    ConstItem(::std::boxed::Box<ConstItem<'tree>>),
    EmptyStatement(::std::boxed::Box<EmptyStatement<'tree>>),
    EnumItem(::std::boxed::Box<EnumItem<'tree>>),
    ExternCrateDeclaration(::std::boxed::Box<ExternCrateDeclaration<'tree>>),
    ForeignModItem(::std::boxed::Box<ForeignModItem<'tree>>),
    FunctionItem(::std::boxed::Box<FunctionItem<'tree>>),
    FunctionSignatureItem(::std::boxed::Box<FunctionSignatureItem<'tree>>),
    ImplItem(::std::boxed::Box<ImplItem<'tree>>),
    InnerAttributeItem(::std::boxed::Box<InnerAttributeItem<'tree>>),
    LetDeclaration(::std::boxed::Box<LetDeclaration<'tree>>),
    MacroDefinition(::std::boxed::Box<MacroDefinition<'tree>>),
    MacroInvocation(::std::boxed::Box<MacroInvocation<'tree>>),
    ModItem(::std::boxed::Box<ModItem<'tree>>),
    StaticItem(::std::boxed::Box<StaticItem<'tree>>),
    StructItem(::std::boxed::Box<StructItem<'tree>>),
    TraitItem(::std::boxed::Box<TraitItem<'tree>>),
    TypeItem(::std::boxed::Box<TypeItem<'tree>>),
    UnionItem(::std::boxed::Box<UnionItem<'tree>>),
    UseDeclaration(::std::boxed::Box<UseDeclaration<'tree>>),
}
impl<'tree> ::treesitter_types::FromNode<'tree> for DeclarationStatement<'tree> {
    #[allow(clippy::collapsible_else_if)]
    fn from_node(
        node: ::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        match node.kind() {
            "associated_type" => Ok(Self::AssociatedType(::std::boxed::Box::new(
                ::treesitter_types::runtime::maybe_grow_stack(|| {
                    <AssociatedType as ::treesitter_types::FromNode>::from_node(node, src)
                })?,
            ))),
            "attribute_item" => Ok(Self::AttributeItem(::std::boxed::Box::new(
                ::treesitter_types::runtime::maybe_grow_stack(|| {
                    <AttributeItem as ::treesitter_types::FromNode>::from_node(node, src)
                })?,
            ))),
            "const_item" => Ok(Self::ConstItem(::std::boxed::Box::new(
                ::treesitter_types::runtime::maybe_grow_stack(|| {
                    <ConstItem as ::treesitter_types::FromNode>::from_node(node, src)
                })?,
            ))),
            "empty_statement" => Ok(Self::EmptyStatement(::std::boxed::Box::new(
                ::treesitter_types::runtime::maybe_grow_stack(|| {
                    <EmptyStatement as ::treesitter_types::FromNode>::from_node(node, src)
                })?,
            ))),
            "enum_item" => Ok(Self::EnumItem(::std::boxed::Box::new(
                ::treesitter_types::runtime::maybe_grow_stack(|| {
                    <EnumItem as ::treesitter_types::FromNode>::from_node(node, src)
                })?,
            ))),
            "extern_crate_declaration" => Ok(Self::ExternCrateDeclaration(::std::boxed::Box::new(
                ::treesitter_types::runtime::maybe_grow_stack(|| {
                    <ExternCrateDeclaration as ::treesitter_types::FromNode>::from_node(node, src)
                })?,
            ))),
            "foreign_mod_item" => Ok(Self::ForeignModItem(::std::boxed::Box::new(
                ::treesitter_types::runtime::maybe_grow_stack(|| {
                    <ForeignModItem as ::treesitter_types::FromNode>::from_node(node, src)
                })?,
            ))),
            "function_item" => Ok(Self::FunctionItem(::std::boxed::Box::new(
                ::treesitter_types::runtime::maybe_grow_stack(|| {
                    <FunctionItem as ::treesitter_types::FromNode>::from_node(node, src)
                })?,
            ))),
            "function_signature_item" => Ok(Self::FunctionSignatureItem(::std::boxed::Box::new(
                ::treesitter_types::runtime::maybe_grow_stack(|| {
                    <FunctionSignatureItem as ::treesitter_types::FromNode>::from_node(node, src)
                })?,
            ))),
            "impl_item" => Ok(Self::ImplItem(::std::boxed::Box::new(
                ::treesitter_types::runtime::maybe_grow_stack(|| {
                    <ImplItem as ::treesitter_types::FromNode>::from_node(node, src)
                })?,
            ))),
            "inner_attribute_item" => Ok(Self::InnerAttributeItem(::std::boxed::Box::new(
                ::treesitter_types::runtime::maybe_grow_stack(|| {
                    <InnerAttributeItem as ::treesitter_types::FromNode>::from_node(node, src)
                })?,
            ))),
            "let_declaration" => Ok(Self::LetDeclaration(::std::boxed::Box::new(
                ::treesitter_types::runtime::maybe_grow_stack(|| {
                    <LetDeclaration as ::treesitter_types::FromNode>::from_node(node, src)
                })?,
            ))),
            "macro_definition" => Ok(Self::MacroDefinition(::std::boxed::Box::new(
                ::treesitter_types::runtime::maybe_grow_stack(|| {
                    <MacroDefinition as ::treesitter_types::FromNode>::from_node(node, src)
                })?,
            ))),
            "macro_invocation" => Ok(Self::MacroInvocation(::std::boxed::Box::new(
                ::treesitter_types::runtime::maybe_grow_stack(|| {
                    <MacroInvocation as ::treesitter_types::FromNode>::from_node(node, src)
                })?,
            ))),
            "mod_item" => Ok(Self::ModItem(::std::boxed::Box::new(
                ::treesitter_types::runtime::maybe_grow_stack(|| {
                    <ModItem as ::treesitter_types::FromNode>::from_node(node, src)
                })?,
            ))),
            "static_item" => Ok(Self::StaticItem(::std::boxed::Box::new(
                ::treesitter_types::runtime::maybe_grow_stack(|| {
                    <StaticItem as ::treesitter_types::FromNode>::from_node(node, src)
                })?,
            ))),
            "struct_item" => Ok(Self::StructItem(::std::boxed::Box::new(
                ::treesitter_types::runtime::maybe_grow_stack(|| {
                    <StructItem as ::treesitter_types::FromNode>::from_node(node, src)
                })?,
            ))),
            "trait_item" => Ok(Self::TraitItem(::std::boxed::Box::new(
                ::treesitter_types::runtime::maybe_grow_stack(|| {
                    <TraitItem as ::treesitter_types::FromNode>::from_node(node, src)
                })?,
            ))),
            "type_item" => Ok(Self::TypeItem(::std::boxed::Box::new(
                ::treesitter_types::runtime::maybe_grow_stack(|| {
                    <TypeItem as ::treesitter_types::FromNode>::from_node(node, src)
                })?,
            ))),
            "union_item" => Ok(Self::UnionItem(::std::boxed::Box::new(
                ::treesitter_types::runtime::maybe_grow_stack(|| {
                    <UnionItem as ::treesitter_types::FromNode>::from_node(node, src)
                })?,
            ))),
            "use_declaration" => Ok(Self::UseDeclaration(::std::boxed::Box::new(
                ::treesitter_types::runtime::maybe_grow_stack(|| {
                    <UseDeclaration as ::treesitter_types::FromNode>::from_node(node, src)
                })?,
            ))),
            other => Err(::treesitter_types::ParseError::unexpected_kind(other, node)),
        }
    }
}
impl ::treesitter_types::Spanned for DeclarationStatement<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        match self {
            Self::AssociatedType(inner) => inner.span(),
            Self::AttributeItem(inner) => inner.span(),
            Self::ConstItem(inner) => inner.span(),
            Self::EmptyStatement(inner) => inner.span(),
            Self::EnumItem(inner) => inner.span(),
            Self::ExternCrateDeclaration(inner) => inner.span(),
            Self::ForeignModItem(inner) => inner.span(),
            Self::FunctionItem(inner) => inner.span(),
            Self::FunctionSignatureItem(inner) => inner.span(),
            Self::ImplItem(inner) => inner.span(),
            Self::InnerAttributeItem(inner) => inner.span(),
            Self::LetDeclaration(inner) => inner.span(),
            Self::MacroDefinition(inner) => inner.span(),
            Self::MacroInvocation(inner) => inner.span(),
            Self::ModItem(inner) => inner.span(),
            Self::StaticItem(inner) => inner.span(),
            Self::StructItem(inner) => inner.span(),
            Self::TraitItem(inner) => inner.span(),
            Self::TypeItem(inner) => inner.span(),
            Self::UnionItem(inner) => inner.span(),
            Self::UseDeclaration(inner) => inner.span(),
        }
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Expression<'tree> {
    Literal(::std::boxed::Box<Literal<'tree>>),
    ArrayExpression(::std::boxed::Box<ArrayExpression<'tree>>),
    AssignmentExpression(::std::boxed::Box<AssignmentExpression<'tree>>),
    AsyncBlock(::std::boxed::Box<AsyncBlock<'tree>>),
    AwaitExpression(::std::boxed::Box<AwaitExpression<'tree>>),
    BinaryExpression(::std::boxed::Box<BinaryExpression<'tree>>),
    Block(::std::boxed::Box<Block<'tree>>),
    BreakExpression(::std::boxed::Box<BreakExpression<'tree>>),
    CallExpression(::std::boxed::Box<CallExpression<'tree>>),
    ClosureExpression(::std::boxed::Box<ClosureExpression<'tree>>),
    CompoundAssignmentExpr(::std::boxed::Box<CompoundAssignmentExpr<'tree>>),
    ConstBlock(::std::boxed::Box<ConstBlock<'tree>>),
    ContinueExpression(::std::boxed::Box<ContinueExpression<'tree>>),
    FieldExpression(::std::boxed::Box<FieldExpression<'tree>>),
    ForExpression(::std::boxed::Box<ForExpression<'tree>>),
    GenBlock(::std::boxed::Box<GenBlock<'tree>>),
    GenericFunction(::std::boxed::Box<GenericFunction<'tree>>),
    Identifier(::std::boxed::Box<Identifier<'tree>>),
    IfExpression(::std::boxed::Box<IfExpression<'tree>>),
    IndexExpression(::std::boxed::Box<IndexExpression<'tree>>),
    LoopExpression(::std::boxed::Box<LoopExpression<'tree>>),
    MacroInvocation(::std::boxed::Box<MacroInvocation<'tree>>),
    MatchExpression(::std::boxed::Box<MatchExpression<'tree>>),
    Metavariable(::std::boxed::Box<Metavariable<'tree>>),
    ParenthesizedExpression(::std::boxed::Box<ParenthesizedExpression<'tree>>),
    RangeExpression(::std::boxed::Box<RangeExpression<'tree>>),
    ReferenceExpression(::std::boxed::Box<ReferenceExpression<'tree>>),
    ReturnExpression(::std::boxed::Box<ReturnExpression<'tree>>),
    ScopedIdentifier(::std::boxed::Box<ScopedIdentifier<'tree>>),
    SelfType(::std::boxed::Box<SelfType<'tree>>),
    StructExpression(::std::boxed::Box<StructExpression<'tree>>),
    TryBlock(::std::boxed::Box<TryBlock<'tree>>),
    TryExpression(::std::boxed::Box<TryExpression<'tree>>),
    TupleExpression(::std::boxed::Box<TupleExpression<'tree>>),
    TypeCastExpression(::std::boxed::Box<TypeCastExpression<'tree>>),
    UnaryExpression(::std::boxed::Box<UnaryExpression<'tree>>),
    UnitExpression(::std::boxed::Box<UnitExpression<'tree>>),
    UnsafeBlock(::std::boxed::Box<UnsafeBlock<'tree>>),
    WhileExpression(::std::boxed::Box<WhileExpression<'tree>>),
    YieldExpression(::std::boxed::Box<YieldExpression<'tree>>),
}
impl<'tree> ::treesitter_types::FromNode<'tree> for Expression<'tree> {
    #[allow(clippy::collapsible_else_if)]
    fn from_node(
        node: ::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        match node.kind() {
            "array_expression" => Ok(Self::ArrayExpression(::std::boxed::Box::new(
                ::treesitter_types::runtime::maybe_grow_stack(|| {
                    <ArrayExpression as ::treesitter_types::FromNode>::from_node(node, src)
                })?,
            ))),
            "assignment_expression" => Ok(Self::AssignmentExpression(::std::boxed::Box::new(
                ::treesitter_types::runtime::maybe_grow_stack(|| {
                    <AssignmentExpression as ::treesitter_types::FromNode>::from_node(node, src)
                })?,
            ))),
            "async_block" => Ok(Self::AsyncBlock(::std::boxed::Box::new(
                ::treesitter_types::runtime::maybe_grow_stack(|| {
                    <AsyncBlock as ::treesitter_types::FromNode>::from_node(node, src)
                })?,
            ))),
            "await_expression" => Ok(Self::AwaitExpression(::std::boxed::Box::new(
                ::treesitter_types::runtime::maybe_grow_stack(|| {
                    <AwaitExpression as ::treesitter_types::FromNode>::from_node(node, src)
                })?,
            ))),
            "binary_expression" => Ok(Self::BinaryExpression(::std::boxed::Box::new(
                ::treesitter_types::runtime::maybe_grow_stack(|| {
                    <BinaryExpression as ::treesitter_types::FromNode>::from_node(node, src)
                })?,
            ))),
            "block" => Ok(Self::Block(::std::boxed::Box::new(
                ::treesitter_types::runtime::maybe_grow_stack(|| {
                    <Block as ::treesitter_types::FromNode>::from_node(node, src)
                })?,
            ))),
            "break_expression" => Ok(Self::BreakExpression(::std::boxed::Box::new(
                ::treesitter_types::runtime::maybe_grow_stack(|| {
                    <BreakExpression as ::treesitter_types::FromNode>::from_node(node, src)
                })?,
            ))),
            "call_expression" => Ok(Self::CallExpression(::std::boxed::Box::new(
                ::treesitter_types::runtime::maybe_grow_stack(|| {
                    <CallExpression as ::treesitter_types::FromNode>::from_node(node, src)
                })?,
            ))),
            "closure_expression" => Ok(Self::ClosureExpression(::std::boxed::Box::new(
                ::treesitter_types::runtime::maybe_grow_stack(|| {
                    <ClosureExpression as ::treesitter_types::FromNode>::from_node(node, src)
                })?,
            ))),
            "compound_assignment_expr" => Ok(Self::CompoundAssignmentExpr(::std::boxed::Box::new(
                ::treesitter_types::runtime::maybe_grow_stack(|| {
                    <CompoundAssignmentExpr as ::treesitter_types::FromNode>::from_node(node, src)
                })?,
            ))),
            "const_block" => Ok(Self::ConstBlock(::std::boxed::Box::new(
                ::treesitter_types::runtime::maybe_grow_stack(|| {
                    <ConstBlock as ::treesitter_types::FromNode>::from_node(node, src)
                })?,
            ))),
            "continue_expression" => Ok(Self::ContinueExpression(::std::boxed::Box::new(
                ::treesitter_types::runtime::maybe_grow_stack(|| {
                    <ContinueExpression as ::treesitter_types::FromNode>::from_node(node, src)
                })?,
            ))),
            "field_expression" => Ok(Self::FieldExpression(::std::boxed::Box::new(
                ::treesitter_types::runtime::maybe_grow_stack(|| {
                    <FieldExpression as ::treesitter_types::FromNode>::from_node(node, src)
                })?,
            ))),
            "for_expression" => Ok(Self::ForExpression(::std::boxed::Box::new(
                ::treesitter_types::runtime::maybe_grow_stack(|| {
                    <ForExpression as ::treesitter_types::FromNode>::from_node(node, src)
                })?,
            ))),
            "gen_block" => Ok(Self::GenBlock(::std::boxed::Box::new(
                ::treesitter_types::runtime::maybe_grow_stack(|| {
                    <GenBlock as ::treesitter_types::FromNode>::from_node(node, src)
                })?,
            ))),
            "generic_function" => Ok(Self::GenericFunction(::std::boxed::Box::new(
                ::treesitter_types::runtime::maybe_grow_stack(|| {
                    <GenericFunction as ::treesitter_types::FromNode>::from_node(node, src)
                })?,
            ))),
            "identifier" => Ok(Self::Identifier(::std::boxed::Box::new(
                ::treesitter_types::runtime::maybe_grow_stack(|| {
                    <Identifier as ::treesitter_types::FromNode>::from_node(node, src)
                })?,
            ))),
            "if_expression" => Ok(Self::IfExpression(::std::boxed::Box::new(
                ::treesitter_types::runtime::maybe_grow_stack(|| {
                    <IfExpression as ::treesitter_types::FromNode>::from_node(node, src)
                })?,
            ))),
            "index_expression" => Ok(Self::IndexExpression(::std::boxed::Box::new(
                ::treesitter_types::runtime::maybe_grow_stack(|| {
                    <IndexExpression as ::treesitter_types::FromNode>::from_node(node, src)
                })?,
            ))),
            "loop_expression" => Ok(Self::LoopExpression(::std::boxed::Box::new(
                ::treesitter_types::runtime::maybe_grow_stack(|| {
                    <LoopExpression as ::treesitter_types::FromNode>::from_node(node, src)
                })?,
            ))),
            "macro_invocation" => Ok(Self::MacroInvocation(::std::boxed::Box::new(
                ::treesitter_types::runtime::maybe_grow_stack(|| {
                    <MacroInvocation as ::treesitter_types::FromNode>::from_node(node, src)
                })?,
            ))),
            "match_expression" => Ok(Self::MatchExpression(::std::boxed::Box::new(
                ::treesitter_types::runtime::maybe_grow_stack(|| {
                    <MatchExpression as ::treesitter_types::FromNode>::from_node(node, src)
                })?,
            ))),
            "metavariable" => Ok(Self::Metavariable(::std::boxed::Box::new(
                ::treesitter_types::runtime::maybe_grow_stack(|| {
                    <Metavariable as ::treesitter_types::FromNode>::from_node(node, src)
                })?,
            ))),
            "parenthesized_expression" => Ok(Self::ParenthesizedExpression(
                ::std::boxed::Box::new(::treesitter_types::runtime::maybe_grow_stack(|| {
                    <ParenthesizedExpression as ::treesitter_types::FromNode>::from_node(node, src)
                })?),
            )),
            "range_expression" => Ok(Self::RangeExpression(::std::boxed::Box::new(
                ::treesitter_types::runtime::maybe_grow_stack(|| {
                    <RangeExpression as ::treesitter_types::FromNode>::from_node(node, src)
                })?,
            ))),
            "reference_expression" => Ok(Self::ReferenceExpression(::std::boxed::Box::new(
                ::treesitter_types::runtime::maybe_grow_stack(|| {
                    <ReferenceExpression as ::treesitter_types::FromNode>::from_node(node, src)
                })?,
            ))),
            "return_expression" => Ok(Self::ReturnExpression(::std::boxed::Box::new(
                ::treesitter_types::runtime::maybe_grow_stack(|| {
                    <ReturnExpression as ::treesitter_types::FromNode>::from_node(node, src)
                })?,
            ))),
            "scoped_identifier" => Ok(Self::ScopedIdentifier(::std::boxed::Box::new(
                ::treesitter_types::runtime::maybe_grow_stack(|| {
                    <ScopedIdentifier as ::treesitter_types::FromNode>::from_node(node, src)
                })?,
            ))),
            "self" => Ok(Self::SelfType(::std::boxed::Box::new(
                ::treesitter_types::runtime::maybe_grow_stack(|| {
                    <SelfType as ::treesitter_types::FromNode>::from_node(node, src)
                })?,
            ))),
            "struct_expression" => Ok(Self::StructExpression(::std::boxed::Box::new(
                ::treesitter_types::runtime::maybe_grow_stack(|| {
                    <StructExpression as ::treesitter_types::FromNode>::from_node(node, src)
                })?,
            ))),
            "try_block" => Ok(Self::TryBlock(::std::boxed::Box::new(
                ::treesitter_types::runtime::maybe_grow_stack(|| {
                    <TryBlock as ::treesitter_types::FromNode>::from_node(node, src)
                })?,
            ))),
            "try_expression" => Ok(Self::TryExpression(::std::boxed::Box::new(
                ::treesitter_types::runtime::maybe_grow_stack(|| {
                    <TryExpression as ::treesitter_types::FromNode>::from_node(node, src)
                })?,
            ))),
            "tuple_expression" => Ok(Self::TupleExpression(::std::boxed::Box::new(
                ::treesitter_types::runtime::maybe_grow_stack(|| {
                    <TupleExpression as ::treesitter_types::FromNode>::from_node(node, src)
                })?,
            ))),
            "type_cast_expression" => Ok(Self::TypeCastExpression(::std::boxed::Box::new(
                ::treesitter_types::runtime::maybe_grow_stack(|| {
                    <TypeCastExpression as ::treesitter_types::FromNode>::from_node(node, src)
                })?,
            ))),
            "unary_expression" => Ok(Self::UnaryExpression(::std::boxed::Box::new(
                ::treesitter_types::runtime::maybe_grow_stack(|| {
                    <UnaryExpression as ::treesitter_types::FromNode>::from_node(node, src)
                })?,
            ))),
            "unit_expression" => Ok(Self::UnitExpression(::std::boxed::Box::new(
                ::treesitter_types::runtime::maybe_grow_stack(|| {
                    <UnitExpression as ::treesitter_types::FromNode>::from_node(node, src)
                })?,
            ))),
            "unsafe_block" => Ok(Self::UnsafeBlock(::std::boxed::Box::new(
                ::treesitter_types::runtime::maybe_grow_stack(|| {
                    <UnsafeBlock as ::treesitter_types::FromNode>::from_node(node, src)
                })?,
            ))),
            "while_expression" => Ok(Self::WhileExpression(::std::boxed::Box::new(
                ::treesitter_types::runtime::maybe_grow_stack(|| {
                    <WhileExpression as ::treesitter_types::FromNode>::from_node(node, src)
                })?,
            ))),
            "yield_expression" => Ok(Self::YieldExpression(::std::boxed::Box::new(
                ::treesitter_types::runtime::maybe_grow_stack(|| {
                    <YieldExpression as ::treesitter_types::FromNode>::from_node(node, src)
                })?,
            ))),
            _other => {
                if let Ok(v) = ::treesitter_types::runtime::maybe_grow_stack(|| {
                    <Literal as ::treesitter_types::FromNode>::from_node(node, src)
                }) {
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
impl ::treesitter_types::Spanned for Expression<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        match self {
            Self::Literal(inner) => inner.span(),
            Self::ArrayExpression(inner) => inner.span(),
            Self::AssignmentExpression(inner) => inner.span(),
            Self::AsyncBlock(inner) => inner.span(),
            Self::AwaitExpression(inner) => inner.span(),
            Self::BinaryExpression(inner) => inner.span(),
            Self::Block(inner) => inner.span(),
            Self::BreakExpression(inner) => inner.span(),
            Self::CallExpression(inner) => inner.span(),
            Self::ClosureExpression(inner) => inner.span(),
            Self::CompoundAssignmentExpr(inner) => inner.span(),
            Self::ConstBlock(inner) => inner.span(),
            Self::ContinueExpression(inner) => inner.span(),
            Self::FieldExpression(inner) => inner.span(),
            Self::ForExpression(inner) => inner.span(),
            Self::GenBlock(inner) => inner.span(),
            Self::GenericFunction(inner) => inner.span(),
            Self::Identifier(inner) => inner.span(),
            Self::IfExpression(inner) => inner.span(),
            Self::IndexExpression(inner) => inner.span(),
            Self::LoopExpression(inner) => inner.span(),
            Self::MacroInvocation(inner) => inner.span(),
            Self::MatchExpression(inner) => inner.span(),
            Self::Metavariable(inner) => inner.span(),
            Self::ParenthesizedExpression(inner) => inner.span(),
            Self::RangeExpression(inner) => inner.span(),
            Self::ReferenceExpression(inner) => inner.span(),
            Self::ReturnExpression(inner) => inner.span(),
            Self::ScopedIdentifier(inner) => inner.span(),
            Self::SelfType(inner) => inner.span(),
            Self::StructExpression(inner) => inner.span(),
            Self::TryBlock(inner) => inner.span(),
            Self::TryExpression(inner) => inner.span(),
            Self::TupleExpression(inner) => inner.span(),
            Self::TypeCastExpression(inner) => inner.span(),
            Self::UnaryExpression(inner) => inner.span(),
            Self::UnitExpression(inner) => inner.span(),
            Self::UnsafeBlock(inner) => inner.span(),
            Self::WhileExpression(inner) => inner.span(),
            Self::YieldExpression(inner) => inner.span(),
        }
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Literal<'tree> {
    BooleanLiteral(::std::boxed::Box<BooleanLiteral<'tree>>),
    CharLiteral(::std::boxed::Box<CharLiteral<'tree>>),
    FloatLiteral(::std::boxed::Box<FloatLiteral<'tree>>),
    IntegerLiteral(::std::boxed::Box<IntegerLiteral<'tree>>),
    RawStringLiteral(::std::boxed::Box<RawStringLiteral<'tree>>),
    StringLiteral(::std::boxed::Box<StringLiteral<'tree>>),
}
impl<'tree> ::treesitter_types::FromNode<'tree> for Literal<'tree> {
    #[allow(clippy::collapsible_else_if)]
    fn from_node(
        node: ::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        match node.kind() {
            "boolean_literal" => Ok(Self::BooleanLiteral(::std::boxed::Box::new(
                ::treesitter_types::runtime::maybe_grow_stack(|| {
                    <BooleanLiteral as ::treesitter_types::FromNode>::from_node(node, src)
                })?,
            ))),
            "char_literal" => Ok(Self::CharLiteral(::std::boxed::Box::new(
                ::treesitter_types::runtime::maybe_grow_stack(|| {
                    <CharLiteral as ::treesitter_types::FromNode>::from_node(node, src)
                })?,
            ))),
            "float_literal" => Ok(Self::FloatLiteral(::std::boxed::Box::new(
                ::treesitter_types::runtime::maybe_grow_stack(|| {
                    <FloatLiteral as ::treesitter_types::FromNode>::from_node(node, src)
                })?,
            ))),
            "integer_literal" => Ok(Self::IntegerLiteral(::std::boxed::Box::new(
                ::treesitter_types::runtime::maybe_grow_stack(|| {
                    <IntegerLiteral as ::treesitter_types::FromNode>::from_node(node, src)
                })?,
            ))),
            "raw_string_literal" => Ok(Self::RawStringLiteral(::std::boxed::Box::new(
                ::treesitter_types::runtime::maybe_grow_stack(|| {
                    <RawStringLiteral as ::treesitter_types::FromNode>::from_node(node, src)
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
impl ::treesitter_types::Spanned for Literal<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        match self {
            Self::BooleanLiteral(inner) => inner.span(),
            Self::CharLiteral(inner) => inner.span(),
            Self::FloatLiteral(inner) => inner.span(),
            Self::IntegerLiteral(inner) => inner.span(),
            Self::RawStringLiteral(inner) => inner.span(),
            Self::StringLiteral(inner) => inner.span(),
        }
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum LiteralPattern<'tree> {
    BooleanLiteral(::std::boxed::Box<BooleanLiteral<'tree>>),
    CharLiteral(::std::boxed::Box<CharLiteral<'tree>>),
    FloatLiteral(::std::boxed::Box<FloatLiteral<'tree>>),
    IntegerLiteral(::std::boxed::Box<IntegerLiteral<'tree>>),
    NegativeLiteral(::std::boxed::Box<NegativeLiteral<'tree>>),
    RawStringLiteral(::std::boxed::Box<RawStringLiteral<'tree>>),
    StringLiteral(::std::boxed::Box<StringLiteral<'tree>>),
}
impl<'tree> ::treesitter_types::FromNode<'tree> for LiteralPattern<'tree> {
    #[allow(clippy::collapsible_else_if)]
    fn from_node(
        node: ::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        match node.kind() {
            "boolean_literal" => Ok(Self::BooleanLiteral(::std::boxed::Box::new(
                ::treesitter_types::runtime::maybe_grow_stack(|| {
                    <BooleanLiteral as ::treesitter_types::FromNode>::from_node(node, src)
                })?,
            ))),
            "char_literal" => Ok(Self::CharLiteral(::std::boxed::Box::new(
                ::treesitter_types::runtime::maybe_grow_stack(|| {
                    <CharLiteral as ::treesitter_types::FromNode>::from_node(node, src)
                })?,
            ))),
            "float_literal" => Ok(Self::FloatLiteral(::std::boxed::Box::new(
                ::treesitter_types::runtime::maybe_grow_stack(|| {
                    <FloatLiteral as ::treesitter_types::FromNode>::from_node(node, src)
                })?,
            ))),
            "integer_literal" => Ok(Self::IntegerLiteral(::std::boxed::Box::new(
                ::treesitter_types::runtime::maybe_grow_stack(|| {
                    <IntegerLiteral as ::treesitter_types::FromNode>::from_node(node, src)
                })?,
            ))),
            "negative_literal" => Ok(Self::NegativeLiteral(::std::boxed::Box::new(
                ::treesitter_types::runtime::maybe_grow_stack(|| {
                    <NegativeLiteral as ::treesitter_types::FromNode>::from_node(node, src)
                })?,
            ))),
            "raw_string_literal" => Ok(Self::RawStringLiteral(::std::boxed::Box::new(
                ::treesitter_types::runtime::maybe_grow_stack(|| {
                    <RawStringLiteral as ::treesitter_types::FromNode>::from_node(node, src)
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
impl ::treesitter_types::Spanned for LiteralPattern<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        match self {
            Self::BooleanLiteral(inner) => inner.span(),
            Self::CharLiteral(inner) => inner.span(),
            Self::FloatLiteral(inner) => inner.span(),
            Self::IntegerLiteral(inner) => inner.span(),
            Self::NegativeLiteral(inner) => inner.span(),
            Self::RawStringLiteral(inner) => inner.span(),
            Self::StringLiteral(inner) => inner.span(),
        }
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Pattern<'tree> {
    Blank(::treesitter_types::Span),
    LiteralPattern(::std::boxed::Box<LiteralPattern<'tree>>),
    CapturedPattern(::std::boxed::Box<CapturedPattern<'tree>>),
    ConstBlock(::std::boxed::Box<ConstBlock<'tree>>),
    GenericPattern(::std::boxed::Box<GenericPattern<'tree>>),
    Identifier(::std::boxed::Box<Identifier<'tree>>),
    MacroInvocation(::std::boxed::Box<MacroInvocation<'tree>>),
    MutPattern(::std::boxed::Box<MutPattern<'tree>>),
    OrPattern(::std::boxed::Box<OrPattern<'tree>>),
    RangePattern(::std::boxed::Box<RangePattern<'tree>>),
    RefPattern(::std::boxed::Box<RefPattern<'tree>>),
    ReferencePattern(::std::boxed::Box<ReferencePattern<'tree>>),
    RemainingFieldPattern(::std::boxed::Box<RemainingFieldPattern<'tree>>),
    ScopedIdentifier(::std::boxed::Box<ScopedIdentifier<'tree>>),
    SlicePattern(::std::boxed::Box<SlicePattern<'tree>>),
    StructPattern(::std::boxed::Box<StructPattern<'tree>>),
    TuplePattern(::std::boxed::Box<TuplePattern<'tree>>),
    TupleStructPattern(::std::boxed::Box<TupleStructPattern<'tree>>),
}
impl<'tree> ::treesitter_types::FromNode<'tree> for Pattern<'tree> {
    #[allow(clippy::collapsible_else_if)]
    fn from_node(
        node: ::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        match node.kind() {
            "_" => Ok(Self::Blank(::treesitter_types::Span::from(node))),
            "captured_pattern" => Ok(Self::CapturedPattern(::std::boxed::Box::new(
                ::treesitter_types::runtime::maybe_grow_stack(|| {
                    <CapturedPattern as ::treesitter_types::FromNode>::from_node(node, src)
                })?,
            ))),
            "const_block" => Ok(Self::ConstBlock(::std::boxed::Box::new(
                ::treesitter_types::runtime::maybe_grow_stack(|| {
                    <ConstBlock as ::treesitter_types::FromNode>::from_node(node, src)
                })?,
            ))),
            "generic_pattern" => Ok(Self::GenericPattern(::std::boxed::Box::new(
                ::treesitter_types::runtime::maybe_grow_stack(|| {
                    <GenericPattern as ::treesitter_types::FromNode>::from_node(node, src)
                })?,
            ))),
            "identifier" => Ok(Self::Identifier(::std::boxed::Box::new(
                ::treesitter_types::runtime::maybe_grow_stack(|| {
                    <Identifier as ::treesitter_types::FromNode>::from_node(node, src)
                })?,
            ))),
            "macro_invocation" => Ok(Self::MacroInvocation(::std::boxed::Box::new(
                ::treesitter_types::runtime::maybe_grow_stack(|| {
                    <MacroInvocation as ::treesitter_types::FromNode>::from_node(node, src)
                })?,
            ))),
            "mut_pattern" => Ok(Self::MutPattern(::std::boxed::Box::new(
                ::treesitter_types::runtime::maybe_grow_stack(|| {
                    <MutPattern as ::treesitter_types::FromNode>::from_node(node, src)
                })?,
            ))),
            "or_pattern" => Ok(Self::OrPattern(::std::boxed::Box::new(
                ::treesitter_types::runtime::maybe_grow_stack(|| {
                    <OrPattern as ::treesitter_types::FromNode>::from_node(node, src)
                })?,
            ))),
            "range_pattern" => Ok(Self::RangePattern(::std::boxed::Box::new(
                ::treesitter_types::runtime::maybe_grow_stack(|| {
                    <RangePattern as ::treesitter_types::FromNode>::from_node(node, src)
                })?,
            ))),
            "ref_pattern" => Ok(Self::RefPattern(::std::boxed::Box::new(
                ::treesitter_types::runtime::maybe_grow_stack(|| {
                    <RefPattern as ::treesitter_types::FromNode>::from_node(node, src)
                })?,
            ))),
            "reference_pattern" => Ok(Self::ReferencePattern(::std::boxed::Box::new(
                ::treesitter_types::runtime::maybe_grow_stack(|| {
                    <ReferencePattern as ::treesitter_types::FromNode>::from_node(node, src)
                })?,
            ))),
            "remaining_field_pattern" => Ok(Self::RemainingFieldPattern(::std::boxed::Box::new(
                ::treesitter_types::runtime::maybe_grow_stack(|| {
                    <RemainingFieldPattern as ::treesitter_types::FromNode>::from_node(node, src)
                })?,
            ))),
            "scoped_identifier" => Ok(Self::ScopedIdentifier(::std::boxed::Box::new(
                ::treesitter_types::runtime::maybe_grow_stack(|| {
                    <ScopedIdentifier as ::treesitter_types::FromNode>::from_node(node, src)
                })?,
            ))),
            "slice_pattern" => Ok(Self::SlicePattern(::std::boxed::Box::new(
                ::treesitter_types::runtime::maybe_grow_stack(|| {
                    <SlicePattern as ::treesitter_types::FromNode>::from_node(node, src)
                })?,
            ))),
            "struct_pattern" => Ok(Self::StructPattern(::std::boxed::Box::new(
                ::treesitter_types::runtime::maybe_grow_stack(|| {
                    <StructPattern as ::treesitter_types::FromNode>::from_node(node, src)
                })?,
            ))),
            "tuple_pattern" => Ok(Self::TuplePattern(::std::boxed::Box::new(
                ::treesitter_types::runtime::maybe_grow_stack(|| {
                    <TuplePattern as ::treesitter_types::FromNode>::from_node(node, src)
                })?,
            ))),
            "tuple_struct_pattern" => Ok(Self::TupleStructPattern(::std::boxed::Box::new(
                ::treesitter_types::runtime::maybe_grow_stack(|| {
                    <TupleStructPattern as ::treesitter_types::FromNode>::from_node(node, src)
                })?,
            ))),
            _other => {
                if let Ok(v) = ::treesitter_types::runtime::maybe_grow_stack(|| {
                    <LiteralPattern as ::treesitter_types::FromNode>::from_node(node, src)
                }) {
                    Ok(Self::LiteralPattern(::std::boxed::Box::new(v)))
                } else {
                    Err(::treesitter_types::ParseError::unexpected_kind(
                        _other, node,
                    ))
                }
            }
        }
    }
}
impl ::treesitter_types::Spanned for Pattern<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        match self {
            Self::Blank(span) => *span,
            Self::LiteralPattern(inner) => inner.span(),
            Self::CapturedPattern(inner) => inner.span(),
            Self::ConstBlock(inner) => inner.span(),
            Self::GenericPattern(inner) => inner.span(),
            Self::Identifier(inner) => inner.span(),
            Self::MacroInvocation(inner) => inner.span(),
            Self::MutPattern(inner) => inner.span(),
            Self::OrPattern(inner) => inner.span(),
            Self::RangePattern(inner) => inner.span(),
            Self::RefPattern(inner) => inner.span(),
            Self::ReferencePattern(inner) => inner.span(),
            Self::RemainingFieldPattern(inner) => inner.span(),
            Self::ScopedIdentifier(inner) => inner.span(),
            Self::SlicePattern(inner) => inner.span(),
            Self::StructPattern(inner) => inner.span(),
            Self::TuplePattern(inner) => inner.span(),
            Self::TupleStructPattern(inner) => inner.span(),
        }
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Type<'tree> {
    AbstractType(::std::boxed::Box<AbstractType<'tree>>),
    ArrayType(::std::boxed::Box<ArrayType<'tree>>),
    BoundedType(::std::boxed::Box<BoundedType<'tree>>),
    DynamicType(::std::boxed::Box<DynamicType<'tree>>),
    FunctionType(::std::boxed::Box<FunctionType<'tree>>),
    GenericType(::std::boxed::Box<GenericType<'tree>>),
    MacroInvocation(::std::boxed::Box<MacroInvocation<'tree>>),
    Metavariable(::std::boxed::Box<Metavariable<'tree>>),
    NeverType(::std::boxed::Box<NeverType<'tree>>),
    PointerType(::std::boxed::Box<PointerType<'tree>>),
    PrimitiveType(::std::boxed::Box<PrimitiveType<'tree>>),
    ReferenceType(::std::boxed::Box<ReferenceType<'tree>>),
    RemovedTraitBound(::std::boxed::Box<RemovedTraitBound<'tree>>),
    ScopedTypeIdentifier(::std::boxed::Box<ScopedTypeIdentifier<'tree>>),
    TupleType(::std::boxed::Box<TupleType<'tree>>),
    TypeIdentifier(::std::boxed::Box<TypeIdentifier<'tree>>),
    UnitType(::std::boxed::Box<UnitType<'tree>>),
}
impl<'tree> ::treesitter_types::FromNode<'tree> for Type<'tree> {
    #[allow(clippy::collapsible_else_if)]
    fn from_node(
        node: ::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        match node.kind() {
            "abstract_type" => Ok(Self::AbstractType(::std::boxed::Box::new(
                ::treesitter_types::runtime::maybe_grow_stack(|| {
                    <AbstractType as ::treesitter_types::FromNode>::from_node(node, src)
                })?,
            ))),
            "array_type" => Ok(Self::ArrayType(::std::boxed::Box::new(
                ::treesitter_types::runtime::maybe_grow_stack(|| {
                    <ArrayType as ::treesitter_types::FromNode>::from_node(node, src)
                })?,
            ))),
            "bounded_type" => Ok(Self::BoundedType(::std::boxed::Box::new(
                ::treesitter_types::runtime::maybe_grow_stack(|| {
                    <BoundedType as ::treesitter_types::FromNode>::from_node(node, src)
                })?,
            ))),
            "dynamic_type" => Ok(Self::DynamicType(::std::boxed::Box::new(
                ::treesitter_types::runtime::maybe_grow_stack(|| {
                    <DynamicType as ::treesitter_types::FromNode>::from_node(node, src)
                })?,
            ))),
            "function_type" => Ok(Self::FunctionType(::std::boxed::Box::new(
                ::treesitter_types::runtime::maybe_grow_stack(|| {
                    <FunctionType as ::treesitter_types::FromNode>::from_node(node, src)
                })?,
            ))),
            "generic_type" => Ok(Self::GenericType(::std::boxed::Box::new(
                ::treesitter_types::runtime::maybe_grow_stack(|| {
                    <GenericType as ::treesitter_types::FromNode>::from_node(node, src)
                })?,
            ))),
            "macro_invocation" => Ok(Self::MacroInvocation(::std::boxed::Box::new(
                ::treesitter_types::runtime::maybe_grow_stack(|| {
                    <MacroInvocation as ::treesitter_types::FromNode>::from_node(node, src)
                })?,
            ))),
            "metavariable" => Ok(Self::Metavariable(::std::boxed::Box::new(
                ::treesitter_types::runtime::maybe_grow_stack(|| {
                    <Metavariable as ::treesitter_types::FromNode>::from_node(node, src)
                })?,
            ))),
            "never_type" => Ok(Self::NeverType(::std::boxed::Box::new(
                ::treesitter_types::runtime::maybe_grow_stack(|| {
                    <NeverType as ::treesitter_types::FromNode>::from_node(node, src)
                })?,
            ))),
            "pointer_type" => Ok(Self::PointerType(::std::boxed::Box::new(
                ::treesitter_types::runtime::maybe_grow_stack(|| {
                    <PointerType as ::treesitter_types::FromNode>::from_node(node, src)
                })?,
            ))),
            "primitive_type" => Ok(Self::PrimitiveType(::std::boxed::Box::new(
                ::treesitter_types::runtime::maybe_grow_stack(|| {
                    <PrimitiveType as ::treesitter_types::FromNode>::from_node(node, src)
                })?,
            ))),
            "reference_type" => Ok(Self::ReferenceType(::std::boxed::Box::new(
                ::treesitter_types::runtime::maybe_grow_stack(|| {
                    <ReferenceType as ::treesitter_types::FromNode>::from_node(node, src)
                })?,
            ))),
            "removed_trait_bound" => Ok(Self::RemovedTraitBound(::std::boxed::Box::new(
                ::treesitter_types::runtime::maybe_grow_stack(|| {
                    <RemovedTraitBound as ::treesitter_types::FromNode>::from_node(node, src)
                })?,
            ))),
            "scoped_type_identifier" => Ok(Self::ScopedTypeIdentifier(::std::boxed::Box::new(
                ::treesitter_types::runtime::maybe_grow_stack(|| {
                    <ScopedTypeIdentifier as ::treesitter_types::FromNode>::from_node(node, src)
                })?,
            ))),
            "tuple_type" => Ok(Self::TupleType(::std::boxed::Box::new(
                ::treesitter_types::runtime::maybe_grow_stack(|| {
                    <TupleType as ::treesitter_types::FromNode>::from_node(node, src)
                })?,
            ))),
            "type_identifier" => Ok(Self::TypeIdentifier(::std::boxed::Box::new(
                ::treesitter_types::runtime::maybe_grow_stack(|| {
                    <TypeIdentifier as ::treesitter_types::FromNode>::from_node(node, src)
                })?,
            ))),
            "unit_type" => Ok(Self::UnitType(::std::boxed::Box::new(
                ::treesitter_types::runtime::maybe_grow_stack(|| {
                    <UnitType as ::treesitter_types::FromNode>::from_node(node, src)
                })?,
            ))),
            other => Err(::treesitter_types::ParseError::unexpected_kind(other, node)),
        }
    }
}
impl ::treesitter_types::Spanned for Type<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        match self {
            Self::AbstractType(inner) => inner.span(),
            Self::ArrayType(inner) => inner.span(),
            Self::BoundedType(inner) => inner.span(),
            Self::DynamicType(inner) => inner.span(),
            Self::FunctionType(inner) => inner.span(),
            Self::GenericType(inner) => inner.span(),
            Self::MacroInvocation(inner) => inner.span(),
            Self::Metavariable(inner) => inner.span(),
            Self::NeverType(inner) => inner.span(),
            Self::PointerType(inner) => inner.span(),
            Self::PrimitiveType(inner) => inner.span(),
            Self::ReferenceType(inner) => inner.span(),
            Self::RemovedTraitBound(inner) => inner.span(),
            Self::ScopedTypeIdentifier(inner) => inner.span(),
            Self::TupleType(inner) => inner.span(),
            Self::TypeIdentifier(inner) => inner.span(),
            Self::UnitType(inner) => inner.span(),
        }
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct AbstractType<'tree> {
    pub span: ::treesitter_types::Span,
    pub r#trait: AbstractTypeTrait<'tree>,
    pub children: ::core::option::Option<TypeParameters<'tree>>,
}
impl<'tree> ::treesitter_types::FromNode<'tree> for AbstractType<'tree> {
    #[allow(clippy::match_single_binding, clippy::suspicious_else_formatting)]
    fn from_node(
        node: ::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        debug_assert_eq!(node.kind(), "abstract_type");
        Ok(Self {
            span: ::treesitter_types::Span::from(node),
            r#trait: {
                let child = node
                    .child_by_field_name("trait")
                    .ok_or_else(|| ::treesitter_types::ParseError::missing_field("trait", node))?;
                ::treesitter_types::runtime::maybe_grow_stack(|| {
                    <AbstractTypeTrait as ::treesitter_types::FromNode>::from_node(child, src)
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
                match non_field_children.first() {
                    Some(&child) => Some(::treesitter_types::runtime::maybe_grow_stack(|| {
                        <TypeParameters as ::treesitter_types::FromNode>::from_node(child, src)
                    })?),
                    None => None,
                }
            },
        })
    }
}
impl ::treesitter_types::Spanned for AbstractType<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        self.span
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Arguments<'tree> {
    pub span: ::treesitter_types::Span,
    pub children: ::std::vec::Vec<ArgumentsChildren<'tree>>,
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
                    items.push(::treesitter_types::runtime::maybe_grow_stack(|| {
                        <ArgumentsChildren as ::treesitter_types::FromNode>::from_node(child, src)
                    })?);
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
pub struct ArrayExpression<'tree> {
    pub span: ::treesitter_types::Span,
    pub length: ::core::option::Option<Expression<'tree>>,
    pub children: ::std::vec::Vec<ArrayExpressionChildren<'tree>>,
}
impl<'tree> ::treesitter_types::FromNode<'tree> for ArrayExpression<'tree> {
    #[allow(clippy::match_single_binding, clippy::suspicious_else_formatting)]
    fn from_node(
        node: ::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        debug_assert_eq!(node.kind(), "array_expression");
        Ok(Self {
            span: ::treesitter_types::Span::from(node),
            length: match node.child_by_field_name("length") {
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
                        <ArrayExpressionChildren as ::treesitter_types::FromNode>::from_node(
                            child, src,
                        )
                    })?);
                }
                items
            },
        })
    }
}
impl ::treesitter_types::Spanned for ArrayExpression<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        self.span
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ArrayType<'tree> {
    pub span: ::treesitter_types::Span,
    pub element: Type<'tree>,
    pub length: ::core::option::Option<Expression<'tree>>,
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
                let child = node.child_by_field_name("element").ok_or_else(|| {
                    ::treesitter_types::ParseError::missing_field("element", node)
                })?;
                ::treesitter_types::runtime::maybe_grow_stack(|| {
                    <Type as ::treesitter_types::FromNode>::from_node(child, src)
                })?
            },
            length: match node.child_by_field_name("length") {
                Some(child) => Some(::treesitter_types::runtime::maybe_grow_stack(|| {
                    <Expression as ::treesitter_types::FromNode>::from_node(child, src)
                })?),
                None => None,
            },
        })
    }
}
impl ::treesitter_types::Spanned for ArrayType<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        self.span
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct AssignmentExpression<'tree> {
    pub span: ::treesitter_types::Span,
    pub left: Expression<'tree>,
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
                ::treesitter_types::runtime::maybe_grow_stack(|| {
                    <Expression as ::treesitter_types::FromNode>::from_node(child, src)
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
pub struct AssociatedType<'tree> {
    pub span: ::treesitter_types::Span,
    pub bounds: ::core::option::Option<TraitBounds<'tree>>,
    pub name: TypeIdentifier<'tree>,
    pub type_parameters: ::core::option::Option<TypeParameters<'tree>>,
    pub children: ::core::option::Option<WhereClause<'tree>>,
}
impl<'tree> ::treesitter_types::FromNode<'tree> for AssociatedType<'tree> {
    #[allow(clippy::match_single_binding, clippy::suspicious_else_formatting)]
    fn from_node(
        node: ::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        debug_assert_eq!(node.kind(), "associated_type");
        Ok(Self {
            span: ::treesitter_types::Span::from(node),
            bounds: match node.child_by_field_name("bounds") {
                Some(child) => Some(::treesitter_types::runtime::maybe_grow_stack(|| {
                    <TraitBounds as ::treesitter_types::FromNode>::from_node(child, src)
                })?),
                None => None,
            },
            name: {
                let child = node
                    .child_by_field_name("name")
                    .ok_or_else(|| ::treesitter_types::ParseError::missing_field("name", node))?;
                ::treesitter_types::runtime::maybe_grow_stack(|| {
                    <TypeIdentifier as ::treesitter_types::FromNode>::from_node(child, src)
                })?
            },
            type_parameters: match node.child_by_field_name("type_parameters") {
                Some(child) => Some(::treesitter_types::runtime::maybe_grow_stack(|| {
                    <TypeParameters as ::treesitter_types::FromNode>::from_node(child, src)
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
                        <WhereClause as ::treesitter_types::FromNode>::from_node(child, src)
                    })?),
                    None => None,
                }
            },
        })
    }
}
impl ::treesitter_types::Spanned for AssociatedType<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        self.span
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct AsyncBlock<'tree> {
    pub span: ::treesitter_types::Span,
    pub children: Block<'tree>,
}
impl<'tree> ::treesitter_types::FromNode<'tree> for AsyncBlock<'tree> {
    #[allow(clippy::match_single_binding, clippy::suspicious_else_formatting)]
    fn from_node(
        node: ::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        debug_assert_eq!(node.kind(), "async_block");
        Ok(Self {
            span: ::treesitter_types::Span::from(node),
            children: {
                #[allow(clippy::suspicious_else_formatting)]
                let non_field_children = {
                    let mut cursor = node.walk();
                    let mut result = ::std::vec::Vec::new();
                    if cursor.goto_first_child() {
                        loop {
                            if cursor.field_name().is_none()
                                && cursor.node().is_named()
                                && !cursor.node().is_extra()
                            {
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
                                        ::treesitter_types::runtime::maybe_grow_stack(|| <Block as ::treesitter_types::FromNode>::from_node(
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
                                            ::treesitter_types::runtime::maybe_grow_stack(|| <Block as ::treesitter_types::FromNode>::from_node(
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
                    <Block as ::treesitter_types::FromNode>::from_node(child, src)
                })?
            },
        })
    }
}
impl ::treesitter_types::Spanned for AsyncBlock<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        self.span
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Attribute<'tree> {
    pub span: ::treesitter_types::Span,
    pub arguments: ::core::option::Option<TokenTree<'tree>>,
    pub value: ::core::option::Option<Expression<'tree>>,
    pub children: AttributeChildren<'tree>,
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
            arguments: match node.child_by_field_name("arguments") {
                Some(child) => Some(::treesitter_types::runtime::maybe_grow_stack(|| {
                    <TokenTree as ::treesitter_types::FromNode>::from_node(child, src)
                })?),
                None => None,
            },
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
                                        ::treesitter_types::runtime::maybe_grow_stack(|| <AttributeChildren as ::treesitter_types::FromNode>::from_node(
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
                                            ::treesitter_types::runtime::maybe_grow_stack(|| <AttributeChildren as ::treesitter_types::FromNode>::from_node(
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
                    <AttributeChildren as ::treesitter_types::FromNode>::from_node(child, src)
                })?
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
pub struct AttributeItem<'tree> {
    pub span: ::treesitter_types::Span,
    pub children: Attribute<'tree>,
}
impl<'tree> ::treesitter_types::FromNode<'tree> for AttributeItem<'tree> {
    #[allow(clippy::match_single_binding, clippy::suspicious_else_formatting)]
    fn from_node(
        node: ::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        debug_assert_eq!(node.kind(), "attribute_item");
        Ok(Self {
            span: ::treesitter_types::Span::from(node),
            children: {
                #[allow(clippy::suspicious_else_formatting)]
                let non_field_children = {
                    let mut cursor = node.walk();
                    let mut result = ::std::vec::Vec::new();
                    if cursor.goto_first_child() {
                        loop {
                            if cursor.field_name().is_none()
                                && cursor.node().is_named()
                                && !cursor.node().is_extra()
                            {
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
                                        ::treesitter_types::runtime::maybe_grow_stack(|| <Attribute as ::treesitter_types::FromNode>::from_node(
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
                                            ::treesitter_types::runtime::maybe_grow_stack(|| <Attribute as ::treesitter_types::FromNode>::from_node(
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
                    <Attribute as ::treesitter_types::FromNode>::from_node(child, src)
                })?
            },
        })
    }
}
impl ::treesitter_types::Spanned for AttributeItem<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        self.span
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
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
impl ::treesitter_types::Spanned for AwaitExpression<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        self.span
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct BaseFieldInitializer<'tree> {
    pub span: ::treesitter_types::Span,
    pub children: Expression<'tree>,
}
impl<'tree> ::treesitter_types::FromNode<'tree> for BaseFieldInitializer<'tree> {
    #[allow(clippy::match_single_binding, clippy::suspicious_else_formatting)]
    fn from_node(
        node: ::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        debug_assert_eq!(node.kind(), "base_field_initializer");
        Ok(Self {
            span: ::treesitter_types::Span::from(node),
            children: {
                #[allow(clippy::suspicious_else_formatting)]
                let non_field_children = {
                    let mut cursor = node.walk();
                    let mut result = ::std::vec::Vec::new();
                    if cursor.goto_first_child() {
                        loop {
                            if cursor.field_name().is_none()
                                && cursor.node().is_named()
                                && !cursor.node().is_extra()
                            {
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
impl ::treesitter_types::Spanned for BaseFieldInitializer<'_> {
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
                ::treesitter_types::runtime::maybe_grow_stack(|| {
                    <Expression as ::treesitter_types::FromNode>::from_node(child, src)
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
                    <Expression as ::treesitter_types::FromNode>::from_node(child, src)
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
                    items.push(::treesitter_types::runtime::maybe_grow_stack(|| {
                        <BlockChildren as ::treesitter_types::FromNode>::from_node(child, src)
                    })?);
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
pub struct BlockComment<'tree> {
    pub span: ::treesitter_types::Span,
    pub doc: ::core::option::Option<DocComment<'tree>>,
    pub inner: ::core::option::Option<InnerDocCommentMarker<'tree>>,
    pub outer: ::core::option::Option<OuterDocCommentMarker<'tree>>,
}
impl<'tree> ::treesitter_types::FromNode<'tree> for BlockComment<'tree> {
    #[allow(clippy::match_single_binding, clippy::suspicious_else_formatting)]
    fn from_node(
        node: ::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        debug_assert_eq!(node.kind(), "block_comment");
        Ok(Self {
            span: ::treesitter_types::Span::from(node),
            doc: match node.child_by_field_name("doc") {
                Some(child) => Some(::treesitter_types::runtime::maybe_grow_stack(|| {
                    <DocComment as ::treesitter_types::FromNode>::from_node(child, src)
                })?),
                None => None,
            },
            inner: match node.child_by_field_name("inner") {
                Some(child) => Some(::treesitter_types::runtime::maybe_grow_stack(|| {
                    <InnerDocCommentMarker as ::treesitter_types::FromNode>::from_node(child, src)
                })?),
                None => None,
            },
            outer: match node.child_by_field_name("outer") {
                Some(child) => Some(::treesitter_types::runtime::maybe_grow_stack(|| {
                    <OuterDocCommentMarker as ::treesitter_types::FromNode>::from_node(child, src)
                })?),
                None => None,
            },
        })
    }
}
impl ::treesitter_types::Spanned for BlockComment<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        self.span
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
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
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct BoundedType<'tree> {
    pub span: ::treesitter_types::Span,
    pub children: ::std::vec::Vec<BoundedTypeChildren<'tree>>,
}
impl<'tree> ::treesitter_types::FromNode<'tree> for BoundedType<'tree> {
    #[allow(clippy::match_single_binding, clippy::suspicious_else_formatting)]
    fn from_node(
        node: ::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        debug_assert_eq!(node.kind(), "bounded_type");
        Ok(Self {
            span: ::treesitter_types::Span::from(node),
            children: {
                #[allow(clippy::suspicious_else_formatting)]
                let non_field_children = {
                    let mut cursor = node.walk();
                    let mut result = ::std::vec::Vec::new();
                    if cursor.goto_first_child() {
                        loop {
                            if cursor.field_name().is_none()
                                && cursor.node().is_named()
                                && !cursor.node().is_extra()
                            {
                                result.push(cursor.node());
                            }
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
                        <BoundedTypeChildren as ::treesitter_types::FromNode>::from_node(child, src)
                    })?);
                }
                items
            },
        })
    }
}
impl ::treesitter_types::Spanned for BoundedType<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        self.span
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct BracketedType<'tree> {
    pub span: ::treesitter_types::Span,
    pub children: BracketedTypeChildren<'tree>,
}
impl<'tree> ::treesitter_types::FromNode<'tree> for BracketedType<'tree> {
    #[allow(clippy::match_single_binding, clippy::suspicious_else_formatting)]
    fn from_node(
        node: ::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        debug_assert_eq!(node.kind(), "bracketed_type");
        Ok(Self {
            span: ::treesitter_types::Span::from(node),
            children: {
                #[allow(clippy::suspicious_else_formatting)]
                let non_field_children = {
                    let mut cursor = node.walk();
                    let mut result = ::std::vec::Vec::new();
                    if cursor.goto_first_child() {
                        loop {
                            if cursor.field_name().is_none()
                                && cursor.node().is_named()
                                && !cursor.node().is_extra()
                            {
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
                                        ::treesitter_types::runtime::maybe_grow_stack(|| <BracketedTypeChildren as ::treesitter_types::FromNode>::from_node(
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
                                            ::treesitter_types::runtime::maybe_grow_stack(|| <BracketedTypeChildren as ::treesitter_types::FromNode>::from_node(
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
                    <BracketedTypeChildren as ::treesitter_types::FromNode>::from_node(child, src)
                })?
            },
        })
    }
}
impl ::treesitter_types::Spanned for BracketedType<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        self.span
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct BreakExpression<'tree> {
    pub span: ::treesitter_types::Span,
    pub children: ::std::vec::Vec<BreakExpressionChildren<'tree>>,
}
impl<'tree> ::treesitter_types::FromNode<'tree> for BreakExpression<'tree> {
    #[allow(clippy::match_single_binding, clippy::suspicious_else_formatting)]
    fn from_node(
        node: ::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        debug_assert_eq!(node.kind(), "break_expression");
        Ok(Self {
            span: ::treesitter_types::Span::from(node),
            children: {
                #[allow(clippy::suspicious_else_formatting)]
                let non_field_children = {
                    let mut cursor = node.walk();
                    let mut result = ::std::vec::Vec::new();
                    if cursor.goto_first_child() {
                        loop {
                            if cursor.field_name().is_none()
                                && cursor.node().is_named()
                                && !cursor.node().is_extra()
                            {
                                result.push(cursor.node());
                            }
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
                        <BreakExpressionChildren as ::treesitter_types::FromNode>::from_node(
                            child, src,
                        )
                    })?);
                }
                items
            },
        })
    }
}
impl ::treesitter_types::Spanned for BreakExpression<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        self.span
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CallExpression<'tree> {
    pub span: ::treesitter_types::Span,
    pub arguments: Arguments<'tree>,
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
                ::treesitter_types::runtime::maybe_grow_stack(|| {
                    <Arguments as ::treesitter_types::FromNode>::from_node(child, src)
                })?
            },
            function: {
                let child = node.child_by_field_name("function").ok_or_else(|| {
                    ::treesitter_types::ParseError::missing_field("function", node)
                })?;
                ::treesitter_types::runtime::maybe_grow_stack(|| {
                    <CallExpressionFunction as ::treesitter_types::FromNode>::from_node(child, src)
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
pub struct CapturedPattern<'tree> {
    pub span: ::treesitter_types::Span,
    pub children: ::std::vec::Vec<Pattern<'tree>>,
}
impl<'tree> ::treesitter_types::FromNode<'tree> for CapturedPattern<'tree> {
    #[allow(clippy::match_single_binding, clippy::suspicious_else_formatting)]
    fn from_node(
        node: ::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        debug_assert_eq!(node.kind(), "captured_pattern");
        Ok(Self {
            span: ::treesitter_types::Span::from(node),
            children: {
                #[allow(clippy::suspicious_else_formatting)]
                let non_field_children = {
                    let mut cursor = node.walk();
                    let mut result = ::std::vec::Vec::new();
                    if cursor.goto_first_child() {
                        loop {
                            if cursor.field_name().is_none()
                                && cursor.node().is_named()
                                && !cursor.node().is_extra()
                            {
                                result.push(cursor.node());
                            }
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
                        <Pattern as ::treesitter_types::FromNode>::from_node(child, src)
                    })?);
                }
                items
            },
        })
    }
}
impl ::treesitter_types::Spanned for CapturedPattern<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        self.span
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ClosureExpression<'tree> {
    pub span: ::treesitter_types::Span,
    pub body: ClosureExpressionBody<'tree>,
    pub parameters: ClosureParameters<'tree>,
    pub return_type: ::core::option::Option<Type<'tree>>,
}
impl<'tree> ::treesitter_types::FromNode<'tree> for ClosureExpression<'tree> {
    #[allow(clippy::match_single_binding, clippy::suspicious_else_formatting)]
    fn from_node(
        node: ::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        debug_assert_eq!(node.kind(), "closure_expression");
        Ok(Self {
            span: ::treesitter_types::Span::from(node),
            body: {
                let child = node
                    .child_by_field_name("body")
                    .ok_or_else(|| ::treesitter_types::ParseError::missing_field("body", node))?;
                ::treesitter_types::runtime::maybe_grow_stack(|| {
                    <ClosureExpressionBody as ::treesitter_types::FromNode>::from_node(child, src)
                })?
            },
            parameters: {
                let child = node.child_by_field_name("parameters").ok_or_else(|| {
                    ::treesitter_types::ParseError::missing_field("parameters", node)
                })?;
                ::treesitter_types::runtime::maybe_grow_stack(|| {
                    <ClosureParameters as ::treesitter_types::FromNode>::from_node(child, src)
                })?
            },
            return_type: match node.child_by_field_name("return_type") {
                Some(child) => Some(::treesitter_types::runtime::maybe_grow_stack(|| {
                    <Type as ::treesitter_types::FromNode>::from_node(child, src)
                })?),
                None => None,
            },
        })
    }
}
impl ::treesitter_types::Spanned for ClosureExpression<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        self.span
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ClosureParameters<'tree> {
    pub span: ::treesitter_types::Span,
    pub children: ::std::vec::Vec<ClosureParametersChildren<'tree>>,
}
impl<'tree> ::treesitter_types::FromNode<'tree> for ClosureParameters<'tree> {
    #[allow(clippy::match_single_binding, clippy::suspicious_else_formatting)]
    fn from_node(
        node: ::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        debug_assert_eq!(node.kind(), "closure_parameters");
        Ok(Self {
            span: ::treesitter_types::Span::from(node),
            children: {
                #[allow(clippy::suspicious_else_formatting)]
                let non_field_children = {
                    let mut cursor = node.walk();
                    let mut result = ::std::vec::Vec::new();
                    if cursor.goto_first_child() {
                        loop {
                            if cursor.field_name().is_none()
                                && cursor.node().is_named()
                                && !cursor.node().is_extra()
                            {
                                result.push(cursor.node());
                            }
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
                        <ClosureParametersChildren as ::treesitter_types::FromNode>::from_node(
                            child, src,
                        )
                    })?);
                }
                items
            },
        })
    }
}
impl ::treesitter_types::Spanned for ClosureParameters<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        self.span
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CompoundAssignmentExpr<'tree> {
    pub span: ::treesitter_types::Span,
    pub left: Expression<'tree>,
    pub operator: CompoundAssignmentExprOperator,
    pub right: Expression<'tree>,
}
impl<'tree> ::treesitter_types::FromNode<'tree> for CompoundAssignmentExpr<'tree> {
    #[allow(clippy::match_single_binding, clippy::suspicious_else_formatting)]
    fn from_node(
        node: ::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        debug_assert_eq!(node.kind(), "compound_assignment_expr");
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
            operator: {
                let child = node.child_by_field_name("operator").ok_or_else(|| {
                    ::treesitter_types::ParseError::missing_field("operator", node)
                })?;
                ::treesitter_types::runtime::maybe_grow_stack(|| {
                    <CompoundAssignmentExprOperator as ::treesitter_types::FromNode>::from_node(
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
impl ::treesitter_types::Spanned for CompoundAssignmentExpr<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        self.span
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ConstBlock<'tree> {
    pub span: ::treesitter_types::Span,
    pub body: Block<'tree>,
}
impl<'tree> ::treesitter_types::FromNode<'tree> for ConstBlock<'tree> {
    #[allow(clippy::match_single_binding, clippy::suspicious_else_formatting)]
    fn from_node(
        node: ::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        debug_assert_eq!(node.kind(), "const_block");
        Ok(Self {
            span: ::treesitter_types::Span::from(node),
            body: {
                let child = node
                    .child_by_field_name("body")
                    .ok_or_else(|| ::treesitter_types::ParseError::missing_field("body", node))?;
                ::treesitter_types::runtime::maybe_grow_stack(|| {
                    <Block as ::treesitter_types::FromNode>::from_node(child, src)
                })?
            },
        })
    }
}
impl ::treesitter_types::Spanned for ConstBlock<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        self.span
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ConstItem<'tree> {
    pub span: ::treesitter_types::Span,
    pub name: Identifier<'tree>,
    pub r#type: Type<'tree>,
    pub value: ::core::option::Option<Expression<'tree>>,
    pub children: ::core::option::Option<VisibilityModifier<'tree>>,
}
impl<'tree> ::treesitter_types::FromNode<'tree> for ConstItem<'tree> {
    #[allow(clippy::match_single_binding, clippy::suspicious_else_formatting)]
    fn from_node(
        node: ::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        debug_assert_eq!(node.kind(), "const_item");
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
                    <Type as ::treesitter_types::FromNode>::from_node(child, src)
                })?
            },
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
                match non_field_children.first() {
                    Some(&child) => Some(::treesitter_types::runtime::maybe_grow_stack(|| {
                        <VisibilityModifier as ::treesitter_types::FromNode>::from_node(child, src)
                    })?),
                    None => None,
                }
            },
        })
    }
}
impl ::treesitter_types::Spanned for ConstItem<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        self.span
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ConstParameter<'tree> {
    pub span: ::treesitter_types::Span,
    pub name: Identifier<'tree>,
    pub r#type: Type<'tree>,
    pub value: ::core::option::Option<ConstParameterValue<'tree>>,
}
impl<'tree> ::treesitter_types::FromNode<'tree> for ConstParameter<'tree> {
    #[allow(clippy::match_single_binding, clippy::suspicious_else_formatting)]
    fn from_node(
        node: ::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        debug_assert_eq!(node.kind(), "const_parameter");
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
                    <Type as ::treesitter_types::FromNode>::from_node(child, src)
                })?
            },
            value: match node.child_by_field_name("value") {
                Some(child) => Some(::treesitter_types::runtime::maybe_grow_stack(|| {
                    <ConstParameterValue as ::treesitter_types::FromNode>::from_node(child, src)
                })?),
                None => None,
            },
        })
    }
}
impl ::treesitter_types::Spanned for ConstParameter<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        self.span
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ContinueExpression<'tree> {
    pub span: ::treesitter_types::Span,
    pub children: ::core::option::Option<Label<'tree>>,
}
impl<'tree> ::treesitter_types::FromNode<'tree> for ContinueExpression<'tree> {
    #[allow(clippy::match_single_binding, clippy::suspicious_else_formatting)]
    fn from_node(
        node: ::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        debug_assert_eq!(node.kind(), "continue_expression");
        Ok(Self {
            span: ::treesitter_types::Span::from(node),
            children: {
                #[allow(clippy::suspicious_else_formatting)]
                let non_field_children = {
                    let mut cursor = node.walk();
                    let mut result = ::std::vec::Vec::new();
                    if cursor.goto_first_child() {
                        loop {
                            if cursor.field_name().is_none()
                                && cursor.node().is_named()
                                && !cursor.node().is_extra()
                            {
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
                        <Label as ::treesitter_types::FromNode>::from_node(child, src)
                    })?),
                    None => None,
                }
            },
        })
    }
}
impl ::treesitter_types::Spanned for ContinueExpression<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        self.span
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct DeclarationList<'tree> {
    pub span: ::treesitter_types::Span,
    pub children: ::std::vec::Vec<DeclarationStatement<'tree>>,
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
                    items.push(::treesitter_types::runtime::maybe_grow_stack(|| {
                        <DeclarationStatement as ::treesitter_types::FromNode>::from_node(
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
pub struct DynamicType<'tree> {
    pub span: ::treesitter_types::Span,
    pub r#trait: DynamicTypeTrait<'tree>,
}
impl<'tree> ::treesitter_types::FromNode<'tree> for DynamicType<'tree> {
    #[allow(clippy::match_single_binding, clippy::suspicious_else_formatting)]
    fn from_node(
        node: ::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        debug_assert_eq!(node.kind(), "dynamic_type");
        Ok(Self {
            span: ::treesitter_types::Span::from(node),
            r#trait: {
                let child = node
                    .child_by_field_name("trait")
                    .ok_or_else(|| ::treesitter_types::ParseError::missing_field("trait", node))?;
                ::treesitter_types::runtime::maybe_grow_stack(|| {
                    <DynamicTypeTrait as ::treesitter_types::FromNode>::from_node(child, src)
                })?
            },
        })
    }
}
impl ::treesitter_types::Spanned for DynamicType<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        self.span
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ElseClause<'tree> {
    pub span: ::treesitter_types::Span,
    pub children: ElseClauseChildren<'tree>,
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
                                let candidate = fallback_cursor.node();
                                #[allow(clippy::needless_question_mark)]
                                if (|| -> ::core::result::Result<
                                    _,
                                    ::treesitter_types::ParseError,
                                > {
                                    let child = candidate;
                                    Ok(
                                        ::treesitter_types::runtime::maybe_grow_stack(|| <ElseClauseChildren as ::treesitter_types::FromNode>::from_node(
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
                                            ::treesitter_types::runtime::maybe_grow_stack(|| <ElseClauseChildren as ::treesitter_types::FromNode>::from_node(
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
                    <ElseClauseChildren as ::treesitter_types::FromNode>::from_node(child, src)
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
pub struct EnumItem<'tree> {
    pub span: ::treesitter_types::Span,
    pub body: EnumVariantList<'tree>,
    pub name: TypeIdentifier<'tree>,
    pub type_parameters: ::core::option::Option<TypeParameters<'tree>>,
    pub children: ::std::vec::Vec<EnumItemChildren<'tree>>,
}
impl<'tree> ::treesitter_types::FromNode<'tree> for EnumItem<'tree> {
    #[allow(clippy::match_single_binding, clippy::suspicious_else_formatting)]
    fn from_node(
        node: ::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        debug_assert_eq!(node.kind(), "enum_item");
        Ok(Self {
            span: ::treesitter_types::Span::from(node),
            body: {
                let child = node
                    .child_by_field_name("body")
                    .ok_or_else(|| ::treesitter_types::ParseError::missing_field("body", node))?;
                ::treesitter_types::runtime::maybe_grow_stack(|| {
                    <EnumVariantList as ::treesitter_types::FromNode>::from_node(child, src)
                })?
            },
            name: {
                let child = node
                    .child_by_field_name("name")
                    .ok_or_else(|| ::treesitter_types::ParseError::missing_field("name", node))?;
                ::treesitter_types::runtime::maybe_grow_stack(|| {
                    <TypeIdentifier as ::treesitter_types::FromNode>::from_node(child, src)
                })?
            },
            type_parameters: match node.child_by_field_name("type_parameters") {
                Some(child) => Some(::treesitter_types::runtime::maybe_grow_stack(|| {
                    <TypeParameters as ::treesitter_types::FromNode>::from_node(child, src)
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
                        <EnumItemChildren as ::treesitter_types::FromNode>::from_node(child, src)
                    })?);
                }
                items
            },
        })
    }
}
impl ::treesitter_types::Spanned for EnumItem<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        self.span
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct EnumVariant<'tree> {
    pub span: ::treesitter_types::Span,
    pub body: ::core::option::Option<EnumVariantBody<'tree>>,
    pub name: Identifier<'tree>,
    pub value: ::core::option::Option<Expression<'tree>>,
    pub children: ::core::option::Option<VisibilityModifier<'tree>>,
}
impl<'tree> ::treesitter_types::FromNode<'tree> for EnumVariant<'tree> {
    #[allow(clippy::match_single_binding, clippy::suspicious_else_formatting)]
    fn from_node(
        node: ::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        debug_assert_eq!(node.kind(), "enum_variant");
        Ok(Self {
            span: ::treesitter_types::Span::from(node),
            body: match node.child_by_field_name("body") {
                Some(child) => Some(::treesitter_types::runtime::maybe_grow_stack(|| {
                    <EnumVariantBody as ::treesitter_types::FromNode>::from_node(child, src)
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
                match non_field_children.first() {
                    Some(&child) => Some(::treesitter_types::runtime::maybe_grow_stack(|| {
                        <VisibilityModifier as ::treesitter_types::FromNode>::from_node(child, src)
                    })?),
                    None => None,
                }
            },
        })
    }
}
impl ::treesitter_types::Spanned for EnumVariant<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        self.span
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct EnumVariantList<'tree> {
    pub span: ::treesitter_types::Span,
    pub children: ::std::vec::Vec<EnumVariantListChildren<'tree>>,
}
impl<'tree> ::treesitter_types::FromNode<'tree> for EnumVariantList<'tree> {
    #[allow(clippy::match_single_binding, clippy::suspicious_else_formatting)]
    fn from_node(
        node: ::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        debug_assert_eq!(node.kind(), "enum_variant_list");
        Ok(Self {
            span: ::treesitter_types::Span::from(node),
            children: {
                #[allow(clippy::suspicious_else_formatting)]
                let non_field_children = {
                    let mut cursor = node.walk();
                    let mut result = ::std::vec::Vec::new();
                    if cursor.goto_first_child() {
                        loop {
                            if cursor.field_name().is_none()
                                && cursor.node().is_named()
                                && !cursor.node().is_extra()
                            {
                                result.push(cursor.node());
                            }
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
                        <EnumVariantListChildren as ::treesitter_types::FromNode>::from_node(
                            child, src,
                        )
                    })?);
                }
                items
            },
        })
    }
}
impl ::treesitter_types::Spanned for EnumVariantList<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        self.span
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
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
                            if cursor.field_name().is_none()
                                && cursor.node().is_named()
                                && !cursor.node().is_extra()
                            {
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
impl ::treesitter_types::Spanned for ExpressionStatement<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        self.span
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ExternCrateDeclaration<'tree> {
    pub span: ::treesitter_types::Span,
    pub alias: ::core::option::Option<Identifier<'tree>>,
    pub name: Identifier<'tree>,
    pub children: ::std::vec::Vec<ExternCrateDeclarationChildren<'tree>>,
}
impl<'tree> ::treesitter_types::FromNode<'tree> for ExternCrateDeclaration<'tree> {
    #[allow(clippy::match_single_binding, clippy::suspicious_else_formatting)]
    fn from_node(
        node: ::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        debug_assert_eq!(node.kind(), "extern_crate_declaration");
        Ok(Self {
            span: ::treesitter_types::Span::from(node),
            alias: match node.child_by_field_name("alias") {
                Some(child) => Some(::treesitter_types::runtime::maybe_grow_stack(|| {
                    <Identifier as ::treesitter_types::FromNode>::from_node(child, src)
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
                        <ExternCrateDeclarationChildren as ::treesitter_types::FromNode>::from_node(
                            child, src,
                        )
                    })?);
                }
                items
            },
        })
    }
}
impl ::treesitter_types::Spanned for ExternCrateDeclaration<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        self.span
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ExternModifier<'tree> {
    pub span: ::treesitter_types::Span,
    pub children: ::core::option::Option<StringLiteral<'tree>>,
}
impl<'tree> ::treesitter_types::FromNode<'tree> for ExternModifier<'tree> {
    #[allow(clippy::match_single_binding, clippy::suspicious_else_formatting)]
    fn from_node(
        node: ::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        debug_assert_eq!(node.kind(), "extern_modifier");
        Ok(Self {
            span: ::treesitter_types::Span::from(node),
            children: {
                #[allow(clippy::suspicious_else_formatting)]
                let non_field_children = {
                    let mut cursor = node.walk();
                    let mut result = ::std::vec::Vec::new();
                    if cursor.goto_first_child() {
                        loop {
                            if cursor.field_name().is_none()
                                && cursor.node().is_named()
                                && !cursor.node().is_extra()
                            {
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
                        <StringLiteral as ::treesitter_types::FromNode>::from_node(child, src)
                    })?),
                    None => None,
                }
            },
        })
    }
}
impl ::treesitter_types::Spanned for ExternModifier<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        self.span
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct FieldDeclaration<'tree> {
    pub span: ::treesitter_types::Span,
    pub name: FieldIdentifier<'tree>,
    pub r#type: Type<'tree>,
    pub children: ::core::option::Option<VisibilityModifier<'tree>>,
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
                let child = node
                    .child_by_field_name("name")
                    .ok_or_else(|| ::treesitter_types::ParseError::missing_field("name", node))?;
                ::treesitter_types::runtime::maybe_grow_stack(|| {
                    <FieldIdentifier as ::treesitter_types::FromNode>::from_node(child, src)
                })?
            },
            r#type: {
                let child = node
                    .child_by_field_name("type")
                    .ok_or_else(|| ::treesitter_types::ParseError::missing_field("type", node))?;
                ::treesitter_types::runtime::maybe_grow_stack(|| {
                    <Type as ::treesitter_types::FromNode>::from_node(child, src)
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
                match non_field_children.first() {
                    Some(&child) => Some(::treesitter_types::runtime::maybe_grow_stack(|| {
                        <VisibilityModifier as ::treesitter_types::FromNode>::from_node(child, src)
                    })?),
                    None => None,
                }
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
pub struct FieldExpression<'tree> {
    pub span: ::treesitter_types::Span,
    pub field: FieldExpressionField<'tree>,
    pub value: Expression<'tree>,
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
                ::treesitter_types::runtime::maybe_grow_stack(|| {
                    <FieldExpressionField as ::treesitter_types::FromNode>::from_node(child, src)
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
impl ::treesitter_types::Spanned for FieldExpression<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        self.span
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct FieldInitializer<'tree> {
    pub span: ::treesitter_types::Span,
    pub field: FieldInitializerField<'tree>,
    pub value: Expression<'tree>,
    pub children: ::std::vec::Vec<AttributeItem<'tree>>,
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
            field: {
                let child = node
                    .child_by_field_name("field")
                    .ok_or_else(|| ::treesitter_types::ParseError::missing_field("field", node))?;
                ::treesitter_types::runtime::maybe_grow_stack(|| {
                    <FieldInitializerField as ::treesitter_types::FromNode>::from_node(child, src)
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
            children: {
                #[allow(clippy::suspicious_else_formatting)]
                let non_field_children = {
                    let mut cursor = node.walk();
                    let mut result = ::std::vec::Vec::new();
                    if cursor.goto_first_child() {
                        loop {
                            if cursor.field_name().is_none()
                                && cursor.node().is_named()
                                && !cursor.node().is_extra()
                            {
                                result.push(cursor.node());
                            }
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
                        <AttributeItem as ::treesitter_types::FromNode>::from_node(child, src)
                    })?);
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
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct FieldInitializerList<'tree> {
    pub span: ::treesitter_types::Span,
    pub children: ::std::vec::Vec<FieldInitializerListChildren<'tree>>,
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
                    items.push(::treesitter_types::runtime::maybe_grow_stack(|| {
                        <FieldInitializerListChildren as ::treesitter_types::FromNode>::from_node(
                            child, src,
                        )
                    })?);
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
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct FieldPattern<'tree> {
    pub span: ::treesitter_types::Span,
    pub name: FieldPatternName<'tree>,
    pub pattern: ::core::option::Option<Pattern<'tree>>,
    pub children: ::core::option::Option<MutableSpecifier<'tree>>,
}
impl<'tree> ::treesitter_types::FromNode<'tree> for FieldPattern<'tree> {
    #[allow(clippy::match_single_binding, clippy::suspicious_else_formatting)]
    fn from_node(
        node: ::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        debug_assert_eq!(node.kind(), "field_pattern");
        Ok(Self {
            span: ::treesitter_types::Span::from(node),
            name: {
                let child = node
                    .child_by_field_name("name")
                    .ok_or_else(|| ::treesitter_types::ParseError::missing_field("name", node))?;
                ::treesitter_types::runtime::maybe_grow_stack(|| {
                    <FieldPatternName as ::treesitter_types::FromNode>::from_node(child, src)
                })?
            },
            pattern: match node.child_by_field_name("pattern") {
                Some(child) => Some(::treesitter_types::runtime::maybe_grow_stack(|| {
                    <Pattern as ::treesitter_types::FromNode>::from_node(child, src)
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
                        <MutableSpecifier as ::treesitter_types::FromNode>::from_node(child, src)
                    })?),
                    None => None,
                }
            },
        })
    }
}
impl ::treesitter_types::Spanned for FieldPattern<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        self.span
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ForExpression<'tree> {
    pub span: ::treesitter_types::Span,
    pub body: Block<'tree>,
    pub pattern: Pattern<'tree>,
    pub value: Expression<'tree>,
    pub children: ::core::option::Option<Label<'tree>>,
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
                ::treesitter_types::runtime::maybe_grow_stack(|| {
                    <Block as ::treesitter_types::FromNode>::from_node(child, src)
                })?
            },
            pattern: {
                let child = node.child_by_field_name("pattern").ok_or_else(|| {
                    ::treesitter_types::ParseError::missing_field("pattern", node)
                })?;
                ::treesitter_types::runtime::maybe_grow_stack(|| {
                    <Pattern as ::treesitter_types::FromNode>::from_node(child, src)
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
            children: {
                #[allow(clippy::suspicious_else_formatting)]
                let non_field_children = {
                    let mut cursor = node.walk();
                    let mut result = ::std::vec::Vec::new();
                    if cursor.goto_first_child() {
                        loop {
                            if cursor.field_name().is_none()
                                && cursor.node().is_named()
                                && !cursor.node().is_extra()
                            {
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
                        <Label as ::treesitter_types::FromNode>::from_node(child, src)
                    })?),
                    None => None,
                }
            },
        })
    }
}
impl ::treesitter_types::Spanned for ForExpression<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        self.span
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ForLifetimes<'tree> {
    pub span: ::treesitter_types::Span,
    pub children: ::std::vec::Vec<Lifetime<'tree>>,
}
impl<'tree> ::treesitter_types::FromNode<'tree> for ForLifetimes<'tree> {
    #[allow(clippy::match_single_binding, clippy::suspicious_else_formatting)]
    fn from_node(
        node: ::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        debug_assert_eq!(node.kind(), "for_lifetimes");
        Ok(Self {
            span: ::treesitter_types::Span::from(node),
            children: {
                #[allow(clippy::suspicious_else_formatting)]
                let non_field_children = {
                    let mut cursor = node.walk();
                    let mut result = ::std::vec::Vec::new();
                    if cursor.goto_first_child() {
                        loop {
                            if cursor.field_name().is_none()
                                && cursor.node().is_named()
                                && !cursor.node().is_extra()
                            {
                                result.push(cursor.node());
                            }
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
                        <Lifetime as ::treesitter_types::FromNode>::from_node(child, src)
                    })?);
                }
                items
            },
        })
    }
}
impl ::treesitter_types::Spanned for ForLifetimes<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        self.span
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ForeignModItem<'tree> {
    pub span: ::treesitter_types::Span,
    pub body: ::core::option::Option<DeclarationList<'tree>>,
    pub children: ExternModifier<'tree>,
}
impl<'tree> ::treesitter_types::FromNode<'tree> for ForeignModItem<'tree> {
    #[allow(clippy::match_single_binding, clippy::suspicious_else_formatting)]
    fn from_node(
        node: ::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        debug_assert_eq!(node.kind(), "foreign_mod_item");
        Ok(Self {
            span: ::treesitter_types::Span::from(node),
            body: match node.child_by_field_name("body") {
                Some(child) => Some(::treesitter_types::runtime::maybe_grow_stack(|| {
                    <DeclarationList as ::treesitter_types::FromNode>::from_node(child, src)
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
                                        ::treesitter_types::runtime::maybe_grow_stack(|| <ExternModifier as ::treesitter_types::FromNode>::from_node(
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
                                            ::treesitter_types::runtime::maybe_grow_stack(|| <ExternModifier as ::treesitter_types::FromNode>::from_node(
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
                    <ExternModifier as ::treesitter_types::FromNode>::from_node(child, src)
                })?
            },
        })
    }
}
impl ::treesitter_types::Spanned for ForeignModItem<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        self.span
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct FragmentSpecifier<'tree> {
    pub span: ::treesitter_types::Span,
    text: &'tree str,
}
impl<'tree> ::treesitter_types::FromNode<'tree> for FragmentSpecifier<'tree> {
    fn from_node(
        node: ::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        debug_assert_eq!(node.kind(), "fragment_specifier");
        Ok(Self {
            span: ::treesitter_types::Span::from(node),
            text: node.utf8_text(src)?,
        })
    }
}
impl<'tree> ::treesitter_types::LeafNode<'tree> for FragmentSpecifier<'tree> {
    fn text(&self) -> &'tree str {
        self.text
    }
}
impl ::treesitter_types::Spanned for FragmentSpecifier<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        self.span
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct FunctionItem<'tree> {
    pub span: ::treesitter_types::Span,
    pub body: Block<'tree>,
    pub name: FunctionItemName<'tree>,
    pub parameters: Parameters<'tree>,
    pub return_type: ::core::option::Option<Type<'tree>>,
    pub type_parameters: ::core::option::Option<TypeParameters<'tree>>,
    pub children: ::std::vec::Vec<FunctionItemChildren<'tree>>,
}
impl<'tree> ::treesitter_types::FromNode<'tree> for FunctionItem<'tree> {
    #[allow(clippy::match_single_binding, clippy::suspicious_else_formatting)]
    fn from_node(
        node: ::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        debug_assert_eq!(node.kind(), "function_item");
        Ok(Self {
            span: ::treesitter_types::Span::from(node),
            body: {
                let child = node
                    .child_by_field_name("body")
                    .ok_or_else(|| ::treesitter_types::ParseError::missing_field("body", node))?;
                ::treesitter_types::runtime::maybe_grow_stack(|| {
                    <Block as ::treesitter_types::FromNode>::from_node(child, src)
                })?
            },
            name: {
                let child = node
                    .child_by_field_name("name")
                    .ok_or_else(|| ::treesitter_types::ParseError::missing_field("name", node))?;
                ::treesitter_types::runtime::maybe_grow_stack(|| {
                    <FunctionItemName as ::treesitter_types::FromNode>::from_node(child, src)
                })?
            },
            parameters: {
                let child = node.child_by_field_name("parameters").ok_or_else(|| {
                    ::treesitter_types::ParseError::missing_field("parameters", node)
                })?;
                ::treesitter_types::runtime::maybe_grow_stack(|| {
                    <Parameters as ::treesitter_types::FromNode>::from_node(child, src)
                })?
            },
            return_type: match node.child_by_field_name("return_type") {
                Some(child) => Some(::treesitter_types::runtime::maybe_grow_stack(|| {
                    <Type as ::treesitter_types::FromNode>::from_node(child, src)
                })?),
                None => None,
            },
            type_parameters: match node.child_by_field_name("type_parameters") {
                Some(child) => Some(::treesitter_types::runtime::maybe_grow_stack(|| {
                    <TypeParameters as ::treesitter_types::FromNode>::from_node(child, src)
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
                        <FunctionItemChildren as ::treesitter_types::FromNode>::from_node(
                            child, src,
                        )
                    })?);
                }
                items
            },
        })
    }
}
impl ::treesitter_types::Spanned for FunctionItem<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        self.span
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct FunctionModifiers<'tree> {
    pub span: ::treesitter_types::Span,
    pub children: ::std::vec::Vec<ExternModifier<'tree>>,
}
impl<'tree> ::treesitter_types::FromNode<'tree> for FunctionModifiers<'tree> {
    #[allow(clippy::match_single_binding, clippy::suspicious_else_formatting)]
    fn from_node(
        node: ::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        debug_assert_eq!(node.kind(), "function_modifiers");
        Ok(Self {
            span: ::treesitter_types::Span::from(node),
            children: {
                #[allow(clippy::suspicious_else_formatting)]
                let non_field_children = {
                    let mut cursor = node.walk();
                    let mut result = ::std::vec::Vec::new();
                    if cursor.goto_first_child() {
                        loop {
                            if cursor.field_name().is_none()
                                && cursor.node().is_named()
                                && !cursor.node().is_extra()
                            {
                                result.push(cursor.node());
                            }
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
                        <ExternModifier as ::treesitter_types::FromNode>::from_node(child, src)
                    })?);
                }
                items
            },
        })
    }
}
impl ::treesitter_types::Spanned for FunctionModifiers<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        self.span
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct FunctionSignatureItem<'tree> {
    pub span: ::treesitter_types::Span,
    pub name: FunctionSignatureItemName<'tree>,
    pub parameters: Parameters<'tree>,
    pub return_type: ::core::option::Option<Type<'tree>>,
    pub type_parameters: ::core::option::Option<TypeParameters<'tree>>,
    pub children: ::std::vec::Vec<FunctionSignatureItemChildren<'tree>>,
}
impl<'tree> ::treesitter_types::FromNode<'tree> for FunctionSignatureItem<'tree> {
    #[allow(clippy::match_single_binding, clippy::suspicious_else_formatting)]
    fn from_node(
        node: ::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        debug_assert_eq!(node.kind(), "function_signature_item");
        Ok(Self {
            span: ::treesitter_types::Span::from(node),
            name: {
                let child = node
                    .child_by_field_name("name")
                    .ok_or_else(|| ::treesitter_types::ParseError::missing_field("name", node))?;
                ::treesitter_types::runtime::maybe_grow_stack(|| {
                    <FunctionSignatureItemName as ::treesitter_types::FromNode>::from_node(
                        child, src,
                    )
                })?
            },
            parameters: {
                let child = node.child_by_field_name("parameters").ok_or_else(|| {
                    ::treesitter_types::ParseError::missing_field("parameters", node)
                })?;
                ::treesitter_types::runtime::maybe_grow_stack(|| {
                    <Parameters as ::treesitter_types::FromNode>::from_node(child, src)
                })?
            },
            return_type: match node.child_by_field_name("return_type") {
                Some(child) => Some(::treesitter_types::runtime::maybe_grow_stack(|| {
                    <Type as ::treesitter_types::FromNode>::from_node(child, src)
                })?),
                None => None,
            },
            type_parameters: match node.child_by_field_name("type_parameters") {
                Some(child) => Some(::treesitter_types::runtime::maybe_grow_stack(|| {
                    <TypeParameters as ::treesitter_types::FromNode>::from_node(child, src)
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
                        <FunctionSignatureItemChildren as ::treesitter_types::FromNode>::from_node(
                            child, src,
                        )
                    })?);
                }
                items
            },
        })
    }
}
impl ::treesitter_types::Spanned for FunctionSignatureItem<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        self.span
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct FunctionType<'tree> {
    pub span: ::treesitter_types::Span,
    pub parameters: Parameters<'tree>,
    pub return_type: ::core::option::Option<Type<'tree>>,
    pub r#trait: ::core::option::Option<FunctionTypeTrait<'tree>>,
    pub children: ::std::vec::Vec<FunctionTypeChildren<'tree>>,
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
                let child = node.child_by_field_name("parameters").ok_or_else(|| {
                    ::treesitter_types::ParseError::missing_field("parameters", node)
                })?;
                ::treesitter_types::runtime::maybe_grow_stack(|| {
                    <Parameters as ::treesitter_types::FromNode>::from_node(child, src)
                })?
            },
            return_type: match node.child_by_field_name("return_type") {
                Some(child) => Some(::treesitter_types::runtime::maybe_grow_stack(|| {
                    <Type as ::treesitter_types::FromNode>::from_node(child, src)
                })?),
                None => None,
            },
            r#trait: match node.child_by_field_name("trait") {
                Some(child) => Some(::treesitter_types::runtime::maybe_grow_stack(|| {
                    <FunctionTypeTrait as ::treesitter_types::FromNode>::from_node(child, src)
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
                        <FunctionTypeChildren as ::treesitter_types::FromNode>::from_node(
                            child, src,
                        )
                    })?);
                }
                items
            },
        })
    }
}
impl ::treesitter_types::Spanned for FunctionType<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        self.span
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct GenBlock<'tree> {
    pub span: ::treesitter_types::Span,
    pub children: Block<'tree>,
}
impl<'tree> ::treesitter_types::FromNode<'tree> for GenBlock<'tree> {
    #[allow(clippy::match_single_binding, clippy::suspicious_else_formatting)]
    fn from_node(
        node: ::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        debug_assert_eq!(node.kind(), "gen_block");
        Ok(Self {
            span: ::treesitter_types::Span::from(node),
            children: {
                #[allow(clippy::suspicious_else_formatting)]
                let non_field_children = {
                    let mut cursor = node.walk();
                    let mut result = ::std::vec::Vec::new();
                    if cursor.goto_first_child() {
                        loop {
                            if cursor.field_name().is_none()
                                && cursor.node().is_named()
                                && !cursor.node().is_extra()
                            {
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
                                        ::treesitter_types::runtime::maybe_grow_stack(|| <Block as ::treesitter_types::FromNode>::from_node(
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
                                            ::treesitter_types::runtime::maybe_grow_stack(|| <Block as ::treesitter_types::FromNode>::from_node(
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
                    <Block as ::treesitter_types::FromNode>::from_node(child, src)
                })?
            },
        })
    }
}
impl ::treesitter_types::Spanned for GenBlock<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        self.span
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct GenericFunction<'tree> {
    pub span: ::treesitter_types::Span,
    pub function: GenericFunctionFunction<'tree>,
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
                ::treesitter_types::runtime::maybe_grow_stack(|| {
                    <GenericFunctionFunction as ::treesitter_types::FromNode>::from_node(child, src)
                })?
            },
            type_arguments: {
                let child = node.child_by_field_name("type_arguments").ok_or_else(|| {
                    ::treesitter_types::ParseError::missing_field("type_arguments", node)
                })?;
                ::treesitter_types::runtime::maybe_grow_stack(|| {
                    <TypeArguments as ::treesitter_types::FromNode>::from_node(child, src)
                })?
            },
        })
    }
}
impl ::treesitter_types::Spanned for GenericFunction<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        self.span
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct GenericPattern<'tree> {
    pub span: ::treesitter_types::Span,
    pub type_arguments: TypeArguments<'tree>,
    pub children: GenericPatternChildren<'tree>,
}
impl<'tree> ::treesitter_types::FromNode<'tree> for GenericPattern<'tree> {
    #[allow(clippy::match_single_binding, clippy::suspicious_else_formatting)]
    fn from_node(
        node: ::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        debug_assert_eq!(node.kind(), "generic_pattern");
        Ok(Self {
            span: ::treesitter_types::Span::from(node),
            type_arguments: {
                let child = node.child_by_field_name("type_arguments").ok_or_else(|| {
                    ::treesitter_types::ParseError::missing_field("type_arguments", node)
                })?;
                ::treesitter_types::runtime::maybe_grow_stack(|| {
                    <TypeArguments as ::treesitter_types::FromNode>::from_node(child, src)
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
                                        ::treesitter_types::runtime::maybe_grow_stack(|| <GenericPatternChildren as ::treesitter_types::FromNode>::from_node(
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
                                            ::treesitter_types::runtime::maybe_grow_stack(|| <GenericPatternChildren as ::treesitter_types::FromNode>::from_node(
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
                    <GenericPatternChildren as ::treesitter_types::FromNode>::from_node(child, src)
                })?
            },
        })
    }
}
impl ::treesitter_types::Spanned for GenericPattern<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        self.span
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
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
                ::treesitter_types::runtime::maybe_grow_stack(|| {
                    <GenericTypeType as ::treesitter_types::FromNode>::from_node(child, src)
                })?
            },
            type_arguments: {
                let child = node.child_by_field_name("type_arguments").ok_or_else(|| {
                    ::treesitter_types::ParseError::missing_field("type_arguments", node)
                })?;
                ::treesitter_types::runtime::maybe_grow_stack(|| {
                    <TypeArguments as ::treesitter_types::FromNode>::from_node(child, src)
                })?
            },
        })
    }
}
impl ::treesitter_types::Spanned for GenericType<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        self.span
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct GenericTypeWithTurbofish<'tree> {
    pub span: ::treesitter_types::Span,
    pub r#type: GenericTypeWithTurbofishType<'tree>,
    pub type_arguments: TypeArguments<'tree>,
}
impl<'tree> ::treesitter_types::FromNode<'tree> for GenericTypeWithTurbofish<'tree> {
    #[allow(clippy::match_single_binding, clippy::suspicious_else_formatting)]
    fn from_node(
        node: ::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        debug_assert_eq!(node.kind(), "generic_type_with_turbofish");
        Ok(Self {
            span: ::treesitter_types::Span::from(node),
            r#type: {
                let child = node
                    .child_by_field_name("type")
                    .ok_or_else(|| ::treesitter_types::ParseError::missing_field("type", node))?;
                ::treesitter_types::runtime::maybe_grow_stack(|| {
                    <GenericTypeWithTurbofishType as ::treesitter_types::FromNode>::from_node(
                        child, src,
                    )
                })?
            },
            type_arguments: {
                let child = node.child_by_field_name("type_arguments").ok_or_else(|| {
                    ::treesitter_types::ParseError::missing_field("type_arguments", node)
                })?;
                ::treesitter_types::runtime::maybe_grow_stack(|| {
                    <TypeArguments as ::treesitter_types::FromNode>::from_node(child, src)
                })?
            },
        })
    }
}
impl ::treesitter_types::Spanned for GenericTypeWithTurbofish<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        self.span
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct HigherRankedTraitBound<'tree> {
    pub span: ::treesitter_types::Span,
    pub r#type: Type<'tree>,
    pub type_parameters: TypeParameters<'tree>,
}
impl<'tree> ::treesitter_types::FromNode<'tree> for HigherRankedTraitBound<'tree> {
    #[allow(clippy::match_single_binding, clippy::suspicious_else_formatting)]
    fn from_node(
        node: ::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        debug_assert_eq!(node.kind(), "higher_ranked_trait_bound");
        Ok(Self {
            span: ::treesitter_types::Span::from(node),
            r#type: {
                let child = node
                    .child_by_field_name("type")
                    .ok_or_else(|| ::treesitter_types::ParseError::missing_field("type", node))?;
                ::treesitter_types::runtime::maybe_grow_stack(|| {
                    <Type as ::treesitter_types::FromNode>::from_node(child, src)
                })?
            },
            type_parameters: {
                let child = node.child_by_field_name("type_parameters").ok_or_else(|| {
                    ::treesitter_types::ParseError::missing_field("type_parameters", node)
                })?;
                ::treesitter_types::runtime::maybe_grow_stack(|| {
                    <TypeParameters as ::treesitter_types::FromNode>::from_node(child, src)
                })?
            },
        })
    }
}
impl ::treesitter_types::Spanned for HigherRankedTraitBound<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        self.span
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct IfExpression<'tree> {
    pub span: ::treesitter_types::Span,
    pub alternative: ::core::option::Option<ElseClause<'tree>>,
    pub condition: IfExpressionCondition<'tree>,
    pub consequence: Block<'tree>,
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
                    <IfExpressionCondition as ::treesitter_types::FromNode>::from_node(child, src)
                })?
            },
            consequence: {
                let child = node.child_by_field_name("consequence").ok_or_else(|| {
                    ::treesitter_types::ParseError::missing_field("consequence", node)
                })?;
                ::treesitter_types::runtime::maybe_grow_stack(|| {
                    <Block as ::treesitter_types::FromNode>::from_node(child, src)
                })?
            },
        })
    }
}
impl ::treesitter_types::Spanned for IfExpression<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        self.span
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ImplItem<'tree> {
    pub span: ::treesitter_types::Span,
    pub body: ::core::option::Option<DeclarationList<'tree>>,
    pub r#trait: ::core::option::Option<ImplItemTrait<'tree>>,
    pub r#type: Type<'tree>,
    pub type_parameters: ::core::option::Option<TypeParameters<'tree>>,
    pub children: ::core::option::Option<WhereClause<'tree>>,
}
impl<'tree> ::treesitter_types::FromNode<'tree> for ImplItem<'tree> {
    #[allow(clippy::match_single_binding, clippy::suspicious_else_formatting)]
    fn from_node(
        node: ::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        debug_assert_eq!(node.kind(), "impl_item");
        Ok(Self {
            span: ::treesitter_types::Span::from(node),
            body: match node.child_by_field_name("body") {
                Some(child) => Some(::treesitter_types::runtime::maybe_grow_stack(|| {
                    <DeclarationList as ::treesitter_types::FromNode>::from_node(child, src)
                })?),
                None => None,
            },
            r#trait: match node.child_by_field_name("trait") {
                Some(child) => Some(::treesitter_types::runtime::maybe_grow_stack(|| {
                    <ImplItemTrait as ::treesitter_types::FromNode>::from_node(child, src)
                })?),
                None => None,
            },
            r#type: {
                let child = node
                    .child_by_field_name("type")
                    .ok_or_else(|| ::treesitter_types::ParseError::missing_field("type", node))?;
                ::treesitter_types::runtime::maybe_grow_stack(|| {
                    <Type as ::treesitter_types::FromNode>::from_node(child, src)
                })?
            },
            type_parameters: match node.child_by_field_name("type_parameters") {
                Some(child) => Some(::treesitter_types::runtime::maybe_grow_stack(|| {
                    <TypeParameters as ::treesitter_types::FromNode>::from_node(child, src)
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
                        <WhereClause as ::treesitter_types::FromNode>::from_node(child, src)
                    })?),
                    None => None,
                }
            },
        })
    }
}
impl ::treesitter_types::Spanned for ImplItem<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        self.span
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct IndexExpression<'tree> {
    pub span: ::treesitter_types::Span,
    pub children: ::std::vec::Vec<Expression<'tree>>,
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
            children: {
                #[allow(clippy::suspicious_else_formatting)]
                let non_field_children = {
                    let mut cursor = node.walk();
                    let mut result = ::std::vec::Vec::new();
                    if cursor.goto_first_child() {
                        loop {
                            if cursor.field_name().is_none()
                                && cursor.node().is_named()
                                && !cursor.node().is_extra()
                            {
                                result.push(cursor.node());
                            }
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
                        <Expression as ::treesitter_types::FromNode>::from_node(child, src)
                    })?);
                }
                items
            },
        })
    }
}
impl ::treesitter_types::Spanned for IndexExpression<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        self.span
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct InnerAttributeItem<'tree> {
    pub span: ::treesitter_types::Span,
    pub children: Attribute<'tree>,
}
impl<'tree> ::treesitter_types::FromNode<'tree> for InnerAttributeItem<'tree> {
    #[allow(clippy::match_single_binding, clippy::suspicious_else_formatting)]
    fn from_node(
        node: ::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        debug_assert_eq!(node.kind(), "inner_attribute_item");
        Ok(Self {
            span: ::treesitter_types::Span::from(node),
            children: {
                #[allow(clippy::suspicious_else_formatting)]
                let non_field_children = {
                    let mut cursor = node.walk();
                    let mut result = ::std::vec::Vec::new();
                    if cursor.goto_first_child() {
                        loop {
                            if cursor.field_name().is_none()
                                && cursor.node().is_named()
                                && !cursor.node().is_extra()
                            {
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
                                        ::treesitter_types::runtime::maybe_grow_stack(|| <Attribute as ::treesitter_types::FromNode>::from_node(
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
                                            ::treesitter_types::runtime::maybe_grow_stack(|| <Attribute as ::treesitter_types::FromNode>::from_node(
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
                    <Attribute as ::treesitter_types::FromNode>::from_node(child, src)
                })?
            },
        })
    }
}
impl ::treesitter_types::Spanned for InnerAttributeItem<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        self.span
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct InnerDocCommentMarker<'tree> {
    pub span: ::treesitter_types::Span,
    text: &'tree str,
}
impl<'tree> ::treesitter_types::FromNode<'tree> for InnerDocCommentMarker<'tree> {
    fn from_node(
        node: ::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        debug_assert_eq!(node.kind(), "inner_doc_comment_marker");
        Ok(Self {
            span: ::treesitter_types::Span::from(node),
            text: node.utf8_text(src)?,
        })
    }
}
impl<'tree> ::treesitter_types::LeafNode<'tree> for InnerDocCommentMarker<'tree> {
    fn text(&self) -> &'tree str {
        self.text
    }
}
impl ::treesitter_types::Spanned for InnerDocCommentMarker<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        self.span
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Label<'tree> {
    pub span: ::treesitter_types::Span,
    pub children: Identifier<'tree>,
}
impl<'tree> ::treesitter_types::FromNode<'tree> for Label<'tree> {
    #[allow(clippy::match_single_binding, clippy::suspicious_else_formatting)]
    fn from_node(
        node: ::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        debug_assert_eq!(node.kind(), "label");
        Ok(Self {
            span: ::treesitter_types::Span::from(node),
            children: {
                #[allow(clippy::suspicious_else_formatting)]
                let non_field_children = {
                    let mut cursor = node.walk();
                    let mut result = ::std::vec::Vec::new();
                    if cursor.goto_first_child() {
                        loop {
                            if cursor.field_name().is_none()
                                && cursor.node().is_named()
                                && !cursor.node().is_extra()
                            {
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
impl ::treesitter_types::Spanned for Label<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        self.span
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct LetChain<'tree> {
    pub span: ::treesitter_types::Span,
    pub children: ::std::vec::Vec<LetChainChildren<'tree>>,
}
impl<'tree> ::treesitter_types::FromNode<'tree> for LetChain<'tree> {
    #[allow(clippy::match_single_binding, clippy::suspicious_else_formatting)]
    fn from_node(
        node: ::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        debug_assert_eq!(node.kind(), "let_chain");
        Ok(Self {
            span: ::treesitter_types::Span::from(node),
            children: {
                #[allow(clippy::suspicious_else_formatting)]
                let non_field_children = {
                    let mut cursor = node.walk();
                    let mut result = ::std::vec::Vec::new();
                    if cursor.goto_first_child() {
                        loop {
                            if cursor.field_name().is_none()
                                && cursor.node().is_named()
                                && !cursor.node().is_extra()
                            {
                                result.push(cursor.node());
                            }
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
                        <LetChainChildren as ::treesitter_types::FromNode>::from_node(child, src)
                    })?);
                }
                items
            },
        })
    }
}
impl ::treesitter_types::Spanned for LetChain<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        self.span
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct LetCondition<'tree> {
    pub span: ::treesitter_types::Span,
    pub pattern: Pattern<'tree>,
    pub value: Expression<'tree>,
}
impl<'tree> ::treesitter_types::FromNode<'tree> for LetCondition<'tree> {
    #[allow(clippy::match_single_binding, clippy::suspicious_else_formatting)]
    fn from_node(
        node: ::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        debug_assert_eq!(node.kind(), "let_condition");
        Ok(Self {
            span: ::treesitter_types::Span::from(node),
            pattern: {
                let child = node.child_by_field_name("pattern").ok_or_else(|| {
                    ::treesitter_types::ParseError::missing_field("pattern", node)
                })?;
                ::treesitter_types::runtime::maybe_grow_stack(|| {
                    <Pattern as ::treesitter_types::FromNode>::from_node(child, src)
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
impl ::treesitter_types::Spanned for LetCondition<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        self.span
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct LetDeclaration<'tree> {
    pub span: ::treesitter_types::Span,
    pub alternative: ::core::option::Option<Block<'tree>>,
    pub pattern: Pattern<'tree>,
    pub r#type: ::core::option::Option<Type<'tree>>,
    pub value: ::core::option::Option<Expression<'tree>>,
    pub children: ::core::option::Option<MutableSpecifier<'tree>>,
}
impl<'tree> ::treesitter_types::FromNode<'tree> for LetDeclaration<'tree> {
    #[allow(clippy::match_single_binding, clippy::suspicious_else_formatting)]
    fn from_node(
        node: ::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        debug_assert_eq!(node.kind(), "let_declaration");
        Ok(Self {
            span: ::treesitter_types::Span::from(node),
            alternative: match node.child_by_field_name("alternative") {
                Some(child) => Some(::treesitter_types::runtime::maybe_grow_stack(|| {
                    <Block as ::treesitter_types::FromNode>::from_node(child, src)
                })?),
                None => None,
            },
            pattern: {
                let child = node.child_by_field_name("pattern").ok_or_else(|| {
                    ::treesitter_types::ParseError::missing_field("pattern", node)
                })?;
                ::treesitter_types::runtime::maybe_grow_stack(|| {
                    <Pattern as ::treesitter_types::FromNode>::from_node(child, src)
                })?
            },
            r#type: match node.child_by_field_name("type") {
                Some(child) => Some(::treesitter_types::runtime::maybe_grow_stack(|| {
                    <Type as ::treesitter_types::FromNode>::from_node(child, src)
                })?),
                None => None,
            },
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
                match non_field_children.first() {
                    Some(&child) => Some(::treesitter_types::runtime::maybe_grow_stack(|| {
                        <MutableSpecifier as ::treesitter_types::FromNode>::from_node(child, src)
                    })?),
                    None => None,
                }
            },
        })
    }
}
impl ::treesitter_types::Spanned for LetDeclaration<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        self.span
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Lifetime<'tree> {
    pub span: ::treesitter_types::Span,
    pub children: Identifier<'tree>,
}
impl<'tree> ::treesitter_types::FromNode<'tree> for Lifetime<'tree> {
    #[allow(clippy::match_single_binding, clippy::suspicious_else_formatting)]
    fn from_node(
        node: ::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        debug_assert_eq!(node.kind(), "lifetime");
        Ok(Self {
            span: ::treesitter_types::Span::from(node),
            children: {
                #[allow(clippy::suspicious_else_formatting)]
                let non_field_children = {
                    let mut cursor = node.walk();
                    let mut result = ::std::vec::Vec::new();
                    if cursor.goto_first_child() {
                        loop {
                            if cursor.field_name().is_none()
                                && cursor.node().is_named()
                                && !cursor.node().is_extra()
                            {
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
impl ::treesitter_types::Spanned for Lifetime<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        self.span
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct LifetimeParameter<'tree> {
    pub span: ::treesitter_types::Span,
    pub bounds: ::core::option::Option<TraitBounds<'tree>>,
    pub name: Lifetime<'tree>,
}
impl<'tree> ::treesitter_types::FromNode<'tree> for LifetimeParameter<'tree> {
    #[allow(clippy::match_single_binding, clippy::suspicious_else_formatting)]
    fn from_node(
        node: ::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        debug_assert_eq!(node.kind(), "lifetime_parameter");
        Ok(Self {
            span: ::treesitter_types::Span::from(node),
            bounds: match node.child_by_field_name("bounds") {
                Some(child) => Some(::treesitter_types::runtime::maybe_grow_stack(|| {
                    <TraitBounds as ::treesitter_types::FromNode>::from_node(child, src)
                })?),
                None => None,
            },
            name: {
                let child = node
                    .child_by_field_name("name")
                    .ok_or_else(|| ::treesitter_types::ParseError::missing_field("name", node))?;
                ::treesitter_types::runtime::maybe_grow_stack(|| {
                    <Lifetime as ::treesitter_types::FromNode>::from_node(child, src)
                })?
            },
        })
    }
}
impl ::treesitter_types::Spanned for LifetimeParameter<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        self.span
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct LineComment<'tree> {
    pub span: ::treesitter_types::Span,
    pub doc: ::core::option::Option<DocComment<'tree>>,
    pub inner: ::core::option::Option<InnerDocCommentMarker<'tree>>,
    pub outer: ::core::option::Option<OuterDocCommentMarker<'tree>>,
}
impl<'tree> ::treesitter_types::FromNode<'tree> for LineComment<'tree> {
    #[allow(clippy::match_single_binding, clippy::suspicious_else_formatting)]
    fn from_node(
        node: ::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        debug_assert_eq!(node.kind(), "line_comment");
        Ok(Self {
            span: ::treesitter_types::Span::from(node),
            doc: match node.child_by_field_name("doc") {
                Some(child) => Some(::treesitter_types::runtime::maybe_grow_stack(|| {
                    <DocComment as ::treesitter_types::FromNode>::from_node(child, src)
                })?),
                None => None,
            },
            inner: match node.child_by_field_name("inner") {
                Some(child) => Some(::treesitter_types::runtime::maybe_grow_stack(|| {
                    <InnerDocCommentMarker as ::treesitter_types::FromNode>::from_node(child, src)
                })?),
                None => None,
            },
            outer: match node.child_by_field_name("outer") {
                Some(child) => Some(::treesitter_types::runtime::maybe_grow_stack(|| {
                    <OuterDocCommentMarker as ::treesitter_types::FromNode>::from_node(child, src)
                })?),
                None => None,
            },
        })
    }
}
impl ::treesitter_types::Spanned for LineComment<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        self.span
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct LoopExpression<'tree> {
    pub span: ::treesitter_types::Span,
    pub body: Block<'tree>,
    pub children: ::core::option::Option<Label<'tree>>,
}
impl<'tree> ::treesitter_types::FromNode<'tree> for LoopExpression<'tree> {
    #[allow(clippy::match_single_binding, clippy::suspicious_else_formatting)]
    fn from_node(
        node: ::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        debug_assert_eq!(node.kind(), "loop_expression");
        Ok(Self {
            span: ::treesitter_types::Span::from(node),
            body: {
                let child = node
                    .child_by_field_name("body")
                    .ok_or_else(|| ::treesitter_types::ParseError::missing_field("body", node))?;
                ::treesitter_types::runtime::maybe_grow_stack(|| {
                    <Block as ::treesitter_types::FromNode>::from_node(child, src)
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
                match non_field_children.first() {
                    Some(&child) => Some(::treesitter_types::runtime::maybe_grow_stack(|| {
                        <Label as ::treesitter_types::FromNode>::from_node(child, src)
                    })?),
                    None => None,
                }
            },
        })
    }
}
impl ::treesitter_types::Spanned for LoopExpression<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        self.span
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct MacroDefinition<'tree> {
    pub span: ::treesitter_types::Span,
    pub name: Identifier<'tree>,
    pub children: ::std::vec::Vec<MacroRule<'tree>>,
}
impl<'tree> ::treesitter_types::FromNode<'tree> for MacroDefinition<'tree> {
    #[allow(clippy::match_single_binding, clippy::suspicious_else_formatting)]
    fn from_node(
        node: ::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        debug_assert_eq!(node.kind(), "macro_definition");
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
            children: {
                #[allow(clippy::suspicious_else_formatting)]
                let non_field_children = {
                    let mut cursor = node.walk();
                    let mut result = ::std::vec::Vec::new();
                    if cursor.goto_first_child() {
                        loop {
                            if cursor.field_name().is_none()
                                && cursor.node().is_named()
                                && !cursor.node().is_extra()
                            {
                                result.push(cursor.node());
                            }
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
                        <MacroRule as ::treesitter_types::FromNode>::from_node(child, src)
                    })?);
                }
                items
            },
        })
    }
}
impl ::treesitter_types::Spanned for MacroDefinition<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        self.span
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct MacroInvocation<'tree> {
    pub span: ::treesitter_types::Span,
    pub r#macro: MacroInvocationMacro<'tree>,
    pub children: TokenTree<'tree>,
}
impl<'tree> ::treesitter_types::FromNode<'tree> for MacroInvocation<'tree> {
    #[allow(clippy::match_single_binding, clippy::suspicious_else_formatting)]
    fn from_node(
        node: ::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        debug_assert_eq!(node.kind(), "macro_invocation");
        Ok(Self {
            span: ::treesitter_types::Span::from(node),
            r#macro: {
                let child = node
                    .child_by_field_name("macro")
                    .ok_or_else(|| ::treesitter_types::ParseError::missing_field("macro", node))?;
                ::treesitter_types::runtime::maybe_grow_stack(|| {
                    <MacroInvocationMacro as ::treesitter_types::FromNode>::from_node(child, src)
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
                                        ::treesitter_types::runtime::maybe_grow_stack(|| <TokenTree as ::treesitter_types::FromNode>::from_node(
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
                                            ::treesitter_types::runtime::maybe_grow_stack(|| <TokenTree as ::treesitter_types::FromNode>::from_node(
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
                    <TokenTree as ::treesitter_types::FromNode>::from_node(child, src)
                })?
            },
        })
    }
}
impl ::treesitter_types::Spanned for MacroInvocation<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        self.span
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct MacroRule<'tree> {
    pub span: ::treesitter_types::Span,
    pub left: TokenTreePattern<'tree>,
    pub right: TokenTree<'tree>,
}
impl<'tree> ::treesitter_types::FromNode<'tree> for MacroRule<'tree> {
    #[allow(clippy::match_single_binding, clippy::suspicious_else_formatting)]
    fn from_node(
        node: ::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        debug_assert_eq!(node.kind(), "macro_rule");
        Ok(Self {
            span: ::treesitter_types::Span::from(node),
            left: {
                let child = node
                    .child_by_field_name("left")
                    .ok_or_else(|| ::treesitter_types::ParseError::missing_field("left", node))?;
                ::treesitter_types::runtime::maybe_grow_stack(|| {
                    <TokenTreePattern as ::treesitter_types::FromNode>::from_node(child, src)
                })?
            },
            right: {
                let child = node
                    .child_by_field_name("right")
                    .ok_or_else(|| ::treesitter_types::ParseError::missing_field("right", node))?;
                ::treesitter_types::runtime::maybe_grow_stack(|| {
                    <TokenTree as ::treesitter_types::FromNode>::from_node(child, src)
                })?
            },
        })
    }
}
impl ::treesitter_types::Spanned for MacroRule<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        self.span
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct MatchArm<'tree> {
    pub span: ::treesitter_types::Span,
    pub pattern: MatchPattern<'tree>,
    pub value: Expression<'tree>,
    pub children: ::std::vec::Vec<MatchArmChildren<'tree>>,
}
impl<'tree> ::treesitter_types::FromNode<'tree> for MatchArm<'tree> {
    #[allow(clippy::match_single_binding, clippy::suspicious_else_formatting)]
    fn from_node(
        node: ::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        debug_assert_eq!(node.kind(), "match_arm");
        Ok(Self {
            span: ::treesitter_types::Span::from(node),
            pattern: {
                let child = node.child_by_field_name("pattern").ok_or_else(|| {
                    ::treesitter_types::ParseError::missing_field("pattern", node)
                })?;
                ::treesitter_types::runtime::maybe_grow_stack(|| {
                    <MatchPattern as ::treesitter_types::FromNode>::from_node(child, src)
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
            children: {
                #[allow(clippy::suspicious_else_formatting)]
                let non_field_children = {
                    let mut cursor = node.walk();
                    let mut result = ::std::vec::Vec::new();
                    if cursor.goto_first_child() {
                        loop {
                            if cursor.field_name().is_none()
                                && cursor.node().is_named()
                                && !cursor.node().is_extra()
                            {
                                result.push(cursor.node());
                            }
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
                        <MatchArmChildren as ::treesitter_types::FromNode>::from_node(child, src)
                    })?);
                }
                items
            },
        })
    }
}
impl ::treesitter_types::Spanned for MatchArm<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        self.span
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct MatchBlock<'tree> {
    pub span: ::treesitter_types::Span,
    pub children: ::std::vec::Vec<MatchArm<'tree>>,
}
impl<'tree> ::treesitter_types::FromNode<'tree> for MatchBlock<'tree> {
    #[allow(clippy::match_single_binding, clippy::suspicious_else_formatting)]
    fn from_node(
        node: ::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        debug_assert_eq!(node.kind(), "match_block");
        Ok(Self {
            span: ::treesitter_types::Span::from(node),
            children: {
                #[allow(clippy::suspicious_else_formatting)]
                let non_field_children = {
                    let mut cursor = node.walk();
                    let mut result = ::std::vec::Vec::new();
                    if cursor.goto_first_child() {
                        loop {
                            if cursor.field_name().is_none()
                                && cursor.node().is_named()
                                && !cursor.node().is_extra()
                            {
                                result.push(cursor.node());
                            }
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
                        <MatchArm as ::treesitter_types::FromNode>::from_node(child, src)
                    })?);
                }
                items
            },
        })
    }
}
impl ::treesitter_types::Spanned for MatchBlock<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        self.span
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct MatchExpression<'tree> {
    pub span: ::treesitter_types::Span,
    pub body: MatchBlock<'tree>,
    pub value: Expression<'tree>,
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
                ::treesitter_types::runtime::maybe_grow_stack(|| {
                    <MatchBlock as ::treesitter_types::FromNode>::from_node(child, src)
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
impl ::treesitter_types::Spanned for MatchExpression<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        self.span
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct MatchPattern<'tree> {
    pub span: ::treesitter_types::Span,
    pub condition: ::core::option::Option<MatchPatternCondition<'tree>>,
    pub children: Pattern<'tree>,
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
            condition: match node.child_by_field_name("condition") {
                Some(child) => Some(::treesitter_types::runtime::maybe_grow_stack(|| {
                    <MatchPatternCondition as ::treesitter_types::FromNode>::from_node(child, src)
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
                                        ::treesitter_types::runtime::maybe_grow_stack(|| <Pattern as ::treesitter_types::FromNode>::from_node(
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
                                            ::treesitter_types::runtime::maybe_grow_stack(|| <Pattern as ::treesitter_types::FromNode>::from_node(
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
                    <Pattern as ::treesitter_types::FromNode>::from_node(child, src)
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
pub struct ModItem<'tree> {
    pub span: ::treesitter_types::Span,
    pub body: ::core::option::Option<DeclarationList<'tree>>,
    pub name: Identifier<'tree>,
    pub children: ::core::option::Option<VisibilityModifier<'tree>>,
}
impl<'tree> ::treesitter_types::FromNode<'tree> for ModItem<'tree> {
    #[allow(clippy::match_single_binding, clippy::suspicious_else_formatting)]
    fn from_node(
        node: ::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        debug_assert_eq!(node.kind(), "mod_item");
        Ok(Self {
            span: ::treesitter_types::Span::from(node),
            body: match node.child_by_field_name("body") {
                Some(child) => Some(::treesitter_types::runtime::maybe_grow_stack(|| {
                    <DeclarationList as ::treesitter_types::FromNode>::from_node(child, src)
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
                match non_field_children.first() {
                    Some(&child) => Some(::treesitter_types::runtime::maybe_grow_stack(|| {
                        <VisibilityModifier as ::treesitter_types::FromNode>::from_node(child, src)
                    })?),
                    None => None,
                }
            },
        })
    }
}
impl ::treesitter_types::Spanned for ModItem<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        self.span
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct MutPattern<'tree> {
    pub span: ::treesitter_types::Span,
    pub children: ::std::vec::Vec<MutPatternChildren<'tree>>,
}
impl<'tree> ::treesitter_types::FromNode<'tree> for MutPattern<'tree> {
    #[allow(clippy::match_single_binding, clippy::suspicious_else_formatting)]
    fn from_node(
        node: ::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        debug_assert_eq!(node.kind(), "mut_pattern");
        Ok(Self {
            span: ::treesitter_types::Span::from(node),
            children: {
                #[allow(clippy::suspicious_else_formatting)]
                let non_field_children = {
                    let mut cursor = node.walk();
                    let mut result = ::std::vec::Vec::new();
                    if cursor.goto_first_child() {
                        loop {
                            if cursor.field_name().is_none()
                                && cursor.node().is_named()
                                && !cursor.node().is_extra()
                            {
                                result.push(cursor.node());
                            }
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
                        <MutPatternChildren as ::treesitter_types::FromNode>::from_node(child, src)
                    })?);
                }
                items
            },
        })
    }
}
impl ::treesitter_types::Spanned for MutPattern<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        self.span
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct NegativeLiteral<'tree> {
    pub span: ::treesitter_types::Span,
    pub children: NegativeLiteralChildren<'tree>,
}
impl<'tree> ::treesitter_types::FromNode<'tree> for NegativeLiteral<'tree> {
    #[allow(clippy::match_single_binding, clippy::suspicious_else_formatting)]
    fn from_node(
        node: ::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        debug_assert_eq!(node.kind(), "negative_literal");
        Ok(Self {
            span: ::treesitter_types::Span::from(node),
            children: {
                #[allow(clippy::suspicious_else_formatting)]
                let non_field_children = {
                    let mut cursor = node.walk();
                    let mut result = ::std::vec::Vec::new();
                    if cursor.goto_first_child() {
                        loop {
                            if cursor.field_name().is_none()
                                && cursor.node().is_named()
                                && !cursor.node().is_extra()
                            {
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
                                        ::treesitter_types::runtime::maybe_grow_stack(|| <NegativeLiteralChildren as ::treesitter_types::FromNode>::from_node(
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
                                            ::treesitter_types::runtime::maybe_grow_stack(|| <NegativeLiteralChildren as ::treesitter_types::FromNode>::from_node(
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
                    <NegativeLiteralChildren as ::treesitter_types::FromNode>::from_node(child, src)
                })?
            },
        })
    }
}
impl ::treesitter_types::Spanned for NegativeLiteral<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        self.span
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct NeverType<'tree> {
    pub span: ::treesitter_types::Span,
    text: &'tree str,
}
impl<'tree> ::treesitter_types::FromNode<'tree> for NeverType<'tree> {
    fn from_node(
        node: ::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        debug_assert_eq!(node.kind(), "never_type");
        Ok(Self {
            span: ::treesitter_types::Span::from(node),
            text: node.utf8_text(src)?,
        })
    }
}
impl<'tree> ::treesitter_types::LeafNode<'tree> for NeverType<'tree> {
    fn text(&self) -> &'tree str {
        self.text
    }
}
impl ::treesitter_types::Spanned for NeverType<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        self.span
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct OrPattern<'tree> {
    pub span: ::treesitter_types::Span,
    pub children: ::std::vec::Vec<Pattern<'tree>>,
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
            children: {
                #[allow(clippy::suspicious_else_formatting)]
                let non_field_children = {
                    let mut cursor = node.walk();
                    let mut result = ::std::vec::Vec::new();
                    if cursor.goto_first_child() {
                        loop {
                            if cursor.field_name().is_none()
                                && cursor.node().is_named()
                                && !cursor.node().is_extra()
                            {
                                result.push(cursor.node());
                            }
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
                        <Pattern as ::treesitter_types::FromNode>::from_node(child, src)
                    })?);
                }
                items
            },
        })
    }
}
impl ::treesitter_types::Spanned for OrPattern<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        self.span
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct OrderedFieldDeclarationList<'tree> {
    pub span: ::treesitter_types::Span,
    pub r#type: ::std::vec::Vec<Type<'tree>>,
    pub children: ::std::vec::Vec<OrderedFieldDeclarationListChildren<'tree>>,
}
impl<'tree> ::treesitter_types::FromNode<'tree> for OrderedFieldDeclarationList<'tree> {
    #[allow(clippy::match_single_binding, clippy::suspicious_else_formatting)]
    fn from_node(
        node: ::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        debug_assert_eq!(node.kind(), "ordered_field_declaration_list");
        Ok(Self {
            span: ::treesitter_types::Span::from(node),
            r#type: {
                let mut cursor = node.walk();
                let mut items = ::std::vec::Vec::new();
                for child in node.children_by_field_name("type", &mut cursor) {
                    items.push(::treesitter_types::runtime::maybe_grow_stack(|| {
                        <Type as ::treesitter_types::FromNode>::from_node(child, src)
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
                    items
                        .push(
                            ::treesitter_types::runtime::maybe_grow_stack(|| <OrderedFieldDeclarationListChildren as ::treesitter_types::FromNode>::from_node(
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
impl ::treesitter_types::Spanned for OrderedFieldDeclarationList<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        self.span
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct OuterDocCommentMarker<'tree> {
    pub span: ::treesitter_types::Span,
    text: &'tree str,
}
impl<'tree> ::treesitter_types::FromNode<'tree> for OuterDocCommentMarker<'tree> {
    fn from_node(
        node: ::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        debug_assert_eq!(node.kind(), "outer_doc_comment_marker");
        Ok(Self {
            span: ::treesitter_types::Span::from(node),
            text: node.utf8_text(src)?,
        })
    }
}
impl<'tree> ::treesitter_types::LeafNode<'tree> for OuterDocCommentMarker<'tree> {
    fn text(&self) -> &'tree str {
        self.text
    }
}
impl ::treesitter_types::Spanned for OuterDocCommentMarker<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        self.span
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Parameter<'tree> {
    pub span: ::treesitter_types::Span,
    pub pattern: ParameterPattern<'tree>,
    pub r#type: Type<'tree>,
    pub children: ::core::option::Option<MutableSpecifier<'tree>>,
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
            pattern: {
                let child = node.child_by_field_name("pattern").ok_or_else(|| {
                    ::treesitter_types::ParseError::missing_field("pattern", node)
                })?;
                ::treesitter_types::runtime::maybe_grow_stack(|| {
                    <ParameterPattern as ::treesitter_types::FromNode>::from_node(child, src)
                })?
            },
            r#type: {
                let child = node
                    .child_by_field_name("type")
                    .ok_or_else(|| ::treesitter_types::ParseError::missing_field("type", node))?;
                ::treesitter_types::runtime::maybe_grow_stack(|| {
                    <Type as ::treesitter_types::FromNode>::from_node(child, src)
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
                match non_field_children.first() {
                    Some(&child) => Some(::treesitter_types::runtime::maybe_grow_stack(|| {
                        <MutableSpecifier as ::treesitter_types::FromNode>::from_node(child, src)
                    })?),
                    None => None,
                }
            },
        })
    }
}
impl ::treesitter_types::Spanned for Parameter<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        self.span
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
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
                    items.push(::treesitter_types::runtime::maybe_grow_stack(|| {
                        <ParametersChildren as ::treesitter_types::FromNode>::from_node(child, src)
                    })?);
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
impl ::treesitter_types::Spanned for ParenthesizedExpression<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        self.span
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PointerType<'tree> {
    pub span: ::treesitter_types::Span,
    pub r#type: Type<'tree>,
    pub children: ::core::option::Option<MutableSpecifier<'tree>>,
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
                ::treesitter_types::runtime::maybe_grow_stack(|| {
                    <Type as ::treesitter_types::FromNode>::from_node(child, src)
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
                match non_field_children.first() {
                    Some(&child) => Some(::treesitter_types::runtime::maybe_grow_stack(|| {
                        <MutableSpecifier as ::treesitter_types::FromNode>::from_node(child, src)
                    })?),
                    None => None,
                }
            },
        })
    }
}
impl ::treesitter_types::Spanned for PointerType<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        self.span
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct QualifiedType<'tree> {
    pub span: ::treesitter_types::Span,
    pub alias: Type<'tree>,
    pub r#type: Type<'tree>,
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
            alias: {
                let child = node
                    .child_by_field_name("alias")
                    .ok_or_else(|| ::treesitter_types::ParseError::missing_field("alias", node))?;
                ::treesitter_types::runtime::maybe_grow_stack(|| {
                    <Type as ::treesitter_types::FromNode>::from_node(child, src)
                })?
            },
            r#type: {
                let child = node
                    .child_by_field_name("type")
                    .ok_or_else(|| ::treesitter_types::ParseError::missing_field("type", node))?;
                ::treesitter_types::runtime::maybe_grow_stack(|| {
                    <Type as ::treesitter_types::FromNode>::from_node(child, src)
                })?
            },
        })
    }
}
impl ::treesitter_types::Spanned for QualifiedType<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        self.span
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
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
                    items.push(::treesitter_types::runtime::maybe_grow_stack(|| {
                        <Expression as ::treesitter_types::FromNode>::from_node(child, src)
                    })?);
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
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct RangePattern<'tree> {
    pub span: ::treesitter_types::Span,
    pub left: ::core::option::Option<RangePatternLeft<'tree>>,
    pub right: ::core::option::Option<RangePatternRight<'tree>>,
}
impl<'tree> ::treesitter_types::FromNode<'tree> for RangePattern<'tree> {
    #[allow(clippy::match_single_binding, clippy::suspicious_else_formatting)]
    fn from_node(
        node: ::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        debug_assert_eq!(node.kind(), "range_pattern");
        Ok(Self {
            span: ::treesitter_types::Span::from(node),
            left: match node.child_by_field_name("left") {
                Some(child) => Some(::treesitter_types::runtime::maybe_grow_stack(|| {
                    <RangePatternLeft as ::treesitter_types::FromNode>::from_node(child, src)
                })?),
                None => None,
            },
            right: match node.child_by_field_name("right") {
                Some(child) => Some(::treesitter_types::runtime::maybe_grow_stack(|| {
                    <RangePatternRight as ::treesitter_types::FromNode>::from_node(child, src)
                })?),
                None => None,
            },
        })
    }
}
impl ::treesitter_types::Spanned for RangePattern<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        self.span
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct RawStringLiteral<'tree> {
    pub span: ::treesitter_types::Span,
    pub children: StringContent<'tree>,
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
                                        ::treesitter_types::runtime::maybe_grow_stack(|| <StringContent as ::treesitter_types::FromNode>::from_node(
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
                                            ::treesitter_types::runtime::maybe_grow_stack(|| <StringContent as ::treesitter_types::FromNode>::from_node(
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
                    <StringContent as ::treesitter_types::FromNode>::from_node(child, src)
                })?
            },
        })
    }
}
impl ::treesitter_types::Spanned for RawStringLiteral<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        self.span
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct RefPattern<'tree> {
    pub span: ::treesitter_types::Span,
    pub children: Pattern<'tree>,
}
impl<'tree> ::treesitter_types::FromNode<'tree> for RefPattern<'tree> {
    #[allow(clippy::match_single_binding, clippy::suspicious_else_formatting)]
    fn from_node(
        node: ::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        debug_assert_eq!(node.kind(), "ref_pattern");
        Ok(Self {
            span: ::treesitter_types::Span::from(node),
            children: {
                #[allow(clippy::suspicious_else_formatting)]
                let non_field_children = {
                    let mut cursor = node.walk();
                    let mut result = ::std::vec::Vec::new();
                    if cursor.goto_first_child() {
                        loop {
                            if cursor.field_name().is_none()
                                && cursor.node().is_named()
                                && !cursor.node().is_extra()
                            {
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
                                        ::treesitter_types::runtime::maybe_grow_stack(|| <Pattern as ::treesitter_types::FromNode>::from_node(
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
                                            ::treesitter_types::runtime::maybe_grow_stack(|| <Pattern as ::treesitter_types::FromNode>::from_node(
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
                    <Pattern as ::treesitter_types::FromNode>::from_node(child, src)
                })?
            },
        })
    }
}
impl ::treesitter_types::Spanned for RefPattern<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        self.span
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ReferenceExpression<'tree> {
    pub span: ::treesitter_types::Span,
    pub value: Expression<'tree>,
    pub children: ::core::option::Option<MutableSpecifier<'tree>>,
}
impl<'tree> ::treesitter_types::FromNode<'tree> for ReferenceExpression<'tree> {
    #[allow(clippy::match_single_binding, clippy::suspicious_else_formatting)]
    fn from_node(
        node: ::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        debug_assert_eq!(node.kind(), "reference_expression");
        Ok(Self {
            span: ::treesitter_types::Span::from(node),
            value: {
                let child = node
                    .child_by_field_name("value")
                    .ok_or_else(|| ::treesitter_types::ParseError::missing_field("value", node))?;
                ::treesitter_types::runtime::maybe_grow_stack(|| {
                    <Expression as ::treesitter_types::FromNode>::from_node(child, src)
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
                match non_field_children.first() {
                    Some(&child) => Some(::treesitter_types::runtime::maybe_grow_stack(|| {
                        <MutableSpecifier as ::treesitter_types::FromNode>::from_node(child, src)
                    })?),
                    None => None,
                }
            },
        })
    }
}
impl ::treesitter_types::Spanned for ReferenceExpression<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        self.span
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ReferencePattern<'tree> {
    pub span: ::treesitter_types::Span,
    pub children: ::std::vec::Vec<ReferencePatternChildren<'tree>>,
}
impl<'tree> ::treesitter_types::FromNode<'tree> for ReferencePattern<'tree> {
    #[allow(clippy::match_single_binding, clippy::suspicious_else_formatting)]
    fn from_node(
        node: ::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        debug_assert_eq!(node.kind(), "reference_pattern");
        Ok(Self {
            span: ::treesitter_types::Span::from(node),
            children: {
                #[allow(clippy::suspicious_else_formatting)]
                let non_field_children = {
                    let mut cursor = node.walk();
                    let mut result = ::std::vec::Vec::new();
                    if cursor.goto_first_child() {
                        loop {
                            if cursor.field_name().is_none()
                                && cursor.node().is_named()
                                && !cursor.node().is_extra()
                            {
                                result.push(cursor.node());
                            }
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
                        <ReferencePatternChildren as ::treesitter_types::FromNode>::from_node(
                            child, src,
                        )
                    })?);
                }
                items
            },
        })
    }
}
impl ::treesitter_types::Spanned for ReferencePattern<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        self.span
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ReferenceType<'tree> {
    pub span: ::treesitter_types::Span,
    pub r#type: Type<'tree>,
    pub children: ::std::vec::Vec<ReferenceTypeChildren<'tree>>,
}
impl<'tree> ::treesitter_types::FromNode<'tree> for ReferenceType<'tree> {
    #[allow(clippy::match_single_binding, clippy::suspicious_else_formatting)]
    fn from_node(
        node: ::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        debug_assert_eq!(node.kind(), "reference_type");
        Ok(Self {
            span: ::treesitter_types::Span::from(node),
            r#type: {
                let child = node
                    .child_by_field_name("type")
                    .ok_or_else(|| ::treesitter_types::ParseError::missing_field("type", node))?;
                ::treesitter_types::runtime::maybe_grow_stack(|| {
                    <Type as ::treesitter_types::FromNode>::from_node(child, src)
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
                        <ReferenceTypeChildren as ::treesitter_types::FromNode>::from_node(
                            child, src,
                        )
                    })?);
                }
                items
            },
        })
    }
}
impl ::treesitter_types::Spanned for ReferenceType<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        self.span
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct RemainingFieldPattern<'tree> {
    pub span: ::treesitter_types::Span,
    text: &'tree str,
}
impl<'tree> ::treesitter_types::FromNode<'tree> for RemainingFieldPattern<'tree> {
    fn from_node(
        node: ::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        debug_assert_eq!(node.kind(), "remaining_field_pattern");
        Ok(Self {
            span: ::treesitter_types::Span::from(node),
            text: node.utf8_text(src)?,
        })
    }
}
impl<'tree> ::treesitter_types::LeafNode<'tree> for RemainingFieldPattern<'tree> {
    fn text(&self) -> &'tree str {
        self.text
    }
}
impl ::treesitter_types::Spanned for RemainingFieldPattern<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        self.span
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct RemovedTraitBound<'tree> {
    pub span: ::treesitter_types::Span,
    pub children: Type<'tree>,
}
impl<'tree> ::treesitter_types::FromNode<'tree> for RemovedTraitBound<'tree> {
    #[allow(clippy::match_single_binding, clippy::suspicious_else_formatting)]
    fn from_node(
        node: ::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        debug_assert_eq!(node.kind(), "removed_trait_bound");
        Ok(Self {
            span: ::treesitter_types::Span::from(node),
            children: {
                #[allow(clippy::suspicious_else_formatting)]
                let non_field_children = {
                    let mut cursor = node.walk();
                    let mut result = ::std::vec::Vec::new();
                    if cursor.goto_first_child() {
                        loop {
                            if cursor.field_name().is_none()
                                && cursor.node().is_named()
                                && !cursor.node().is_extra()
                            {
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
                                        ::treesitter_types::runtime::maybe_grow_stack(|| <Type as ::treesitter_types::FromNode>::from_node(
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
                                            ::treesitter_types::runtime::maybe_grow_stack(|| <Type as ::treesitter_types::FromNode>::from_node(
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
                    <Type as ::treesitter_types::FromNode>::from_node(child, src)
                })?
            },
        })
    }
}
impl ::treesitter_types::Spanned for RemovedTraitBound<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        self.span
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
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
                    Some(&child) => Some(::treesitter_types::runtime::maybe_grow_stack(|| {
                        <Expression as ::treesitter_types::FromNode>::from_node(child, src)
                    })?),
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
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ScopedIdentifier<'tree> {
    pub span: ::treesitter_types::Span,
    pub name: ScopedIdentifierName<'tree>,
    pub path: ::core::option::Option<ScopedIdentifierPath<'tree>>,
}
impl<'tree> ::treesitter_types::FromNode<'tree> for ScopedIdentifier<'tree> {
    #[allow(clippy::match_single_binding, clippy::suspicious_else_formatting)]
    fn from_node(
        node: ::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        debug_assert_eq!(node.kind(), "scoped_identifier");
        Ok(Self {
            span: ::treesitter_types::Span::from(node),
            name: {
                let child = node
                    .child_by_field_name("name")
                    .ok_or_else(|| ::treesitter_types::ParseError::missing_field("name", node))?;
                ::treesitter_types::runtime::maybe_grow_stack(|| {
                    <ScopedIdentifierName as ::treesitter_types::FromNode>::from_node(child, src)
                })?
            },
            path: match node.child_by_field_name("path") {
                Some(child) => Some(::treesitter_types::runtime::maybe_grow_stack(|| {
                    <ScopedIdentifierPath as ::treesitter_types::FromNode>::from_node(child, src)
                })?),
                None => None,
            },
        })
    }
}
impl ::treesitter_types::Spanned for ScopedIdentifier<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        self.span
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ScopedTypeIdentifier<'tree> {
    pub span: ::treesitter_types::Span,
    pub name: TypeIdentifier<'tree>,
    pub path: ::core::option::Option<ScopedTypeIdentifierPath<'tree>>,
}
impl<'tree> ::treesitter_types::FromNode<'tree> for ScopedTypeIdentifier<'tree> {
    #[allow(clippy::match_single_binding, clippy::suspicious_else_formatting)]
    fn from_node(
        node: ::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        debug_assert_eq!(node.kind(), "scoped_type_identifier");
        Ok(Self {
            span: ::treesitter_types::Span::from(node),
            name: {
                let child = node
                    .child_by_field_name("name")
                    .ok_or_else(|| ::treesitter_types::ParseError::missing_field("name", node))?;
                ::treesitter_types::runtime::maybe_grow_stack(|| {
                    <TypeIdentifier as ::treesitter_types::FromNode>::from_node(child, src)
                })?
            },
            path: match node.child_by_field_name("path") {
                Some(child) => Some(::treesitter_types::runtime::maybe_grow_stack(|| {
                    <ScopedTypeIdentifierPath as ::treesitter_types::FromNode>::from_node(
                        child, src,
                    )
                })?),
                None => None,
            },
        })
    }
}
impl ::treesitter_types::Spanned for ScopedTypeIdentifier<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        self.span
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ScopedUseList<'tree> {
    pub span: ::treesitter_types::Span,
    pub list: UseList<'tree>,
    pub path: ::core::option::Option<ScopedUseListPath<'tree>>,
}
impl<'tree> ::treesitter_types::FromNode<'tree> for ScopedUseList<'tree> {
    #[allow(clippy::match_single_binding, clippy::suspicious_else_formatting)]
    fn from_node(
        node: ::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        debug_assert_eq!(node.kind(), "scoped_use_list");
        Ok(Self {
            span: ::treesitter_types::Span::from(node),
            list: {
                let child = node
                    .child_by_field_name("list")
                    .ok_or_else(|| ::treesitter_types::ParseError::missing_field("list", node))?;
                ::treesitter_types::runtime::maybe_grow_stack(|| {
                    <UseList as ::treesitter_types::FromNode>::from_node(child, src)
                })?
            },
            path: match node.child_by_field_name("path") {
                Some(child) => Some(::treesitter_types::runtime::maybe_grow_stack(|| {
                    <ScopedUseListPath as ::treesitter_types::FromNode>::from_node(child, src)
                })?),
                None => None,
            },
        })
    }
}
impl ::treesitter_types::Spanned for ScopedUseList<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        self.span
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SelfParameter<'tree> {
    pub span: ::treesitter_types::Span,
    pub children: ::std::vec::Vec<SelfParameterChildren<'tree>>,
}
impl<'tree> ::treesitter_types::FromNode<'tree> for SelfParameter<'tree> {
    #[allow(clippy::match_single_binding, clippy::suspicious_else_formatting)]
    fn from_node(
        node: ::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        debug_assert_eq!(node.kind(), "self_parameter");
        Ok(Self {
            span: ::treesitter_types::Span::from(node),
            children: {
                #[allow(clippy::suspicious_else_formatting)]
                let non_field_children = {
                    let mut cursor = node.walk();
                    let mut result = ::std::vec::Vec::new();
                    if cursor.goto_first_child() {
                        loop {
                            if cursor.field_name().is_none()
                                && cursor.node().is_named()
                                && !cursor.node().is_extra()
                            {
                                result.push(cursor.node());
                            }
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
                        <SelfParameterChildren as ::treesitter_types::FromNode>::from_node(
                            child, src,
                        )
                    })?);
                }
                items
            },
        })
    }
}
impl ::treesitter_types::Spanned for SelfParameter<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        self.span
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ShorthandFieldInitializer<'tree> {
    pub span: ::treesitter_types::Span,
    pub children: ::std::vec::Vec<ShorthandFieldInitializerChildren<'tree>>,
}
impl<'tree> ::treesitter_types::FromNode<'tree> for ShorthandFieldInitializer<'tree> {
    #[allow(clippy::match_single_binding, clippy::suspicious_else_formatting)]
    fn from_node(
        node: ::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        debug_assert_eq!(node.kind(), "shorthand_field_initializer");
        Ok(Self {
            span: ::treesitter_types::Span::from(node),
            children: {
                #[allow(clippy::suspicious_else_formatting)]
                let non_field_children = {
                    let mut cursor = node.walk();
                    let mut result = ::std::vec::Vec::new();
                    if cursor.goto_first_child() {
                        loop {
                            if cursor.field_name().is_none()
                                && cursor.node().is_named()
                                && !cursor.node().is_extra()
                            {
                                result.push(cursor.node());
                            }
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
                            ::treesitter_types::runtime::maybe_grow_stack(|| <ShorthandFieldInitializerChildren as ::treesitter_types::FromNode>::from_node(
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
impl ::treesitter_types::Spanned for ShorthandFieldInitializer<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        self.span
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SlicePattern<'tree> {
    pub span: ::treesitter_types::Span,
    pub children: ::std::vec::Vec<Pattern<'tree>>,
}
impl<'tree> ::treesitter_types::FromNode<'tree> for SlicePattern<'tree> {
    #[allow(clippy::match_single_binding, clippy::suspicious_else_formatting)]
    fn from_node(
        node: ::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        debug_assert_eq!(node.kind(), "slice_pattern");
        Ok(Self {
            span: ::treesitter_types::Span::from(node),
            children: {
                #[allow(clippy::suspicious_else_formatting)]
                let non_field_children = {
                    let mut cursor = node.walk();
                    let mut result = ::std::vec::Vec::new();
                    if cursor.goto_first_child() {
                        loop {
                            if cursor.field_name().is_none()
                                && cursor.node().is_named()
                                && !cursor.node().is_extra()
                            {
                                result.push(cursor.node());
                            }
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
                        <Pattern as ::treesitter_types::FromNode>::from_node(child, src)
                    })?);
                }
                items
            },
        })
    }
}
impl ::treesitter_types::Spanned for SlicePattern<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        self.span
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
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
                            if cursor.field_name().is_none()
                                && cursor.node().is_named()
                                && !cursor.node().is_extra()
                            {
                                result.push(cursor.node());
                            }
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
                        <SourceFileChildren as ::treesitter_types::FromNode>::from_node(child, src)
                    })?);
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
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct StaticItem<'tree> {
    pub span: ::treesitter_types::Span,
    pub name: Identifier<'tree>,
    pub r#type: Type<'tree>,
    pub value: ::core::option::Option<Expression<'tree>>,
    pub children: ::std::vec::Vec<StaticItemChildren<'tree>>,
}
impl<'tree> ::treesitter_types::FromNode<'tree> for StaticItem<'tree> {
    #[allow(clippy::match_single_binding, clippy::suspicious_else_formatting)]
    fn from_node(
        node: ::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        debug_assert_eq!(node.kind(), "static_item");
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
                    <Type as ::treesitter_types::FromNode>::from_node(child, src)
                })?
            },
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
                        <StaticItemChildren as ::treesitter_types::FromNode>::from_node(child, src)
                    })?);
                }
                items
            },
        })
    }
}
impl ::treesitter_types::Spanned for StaticItem<'_> {
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
pub struct StructExpression<'tree> {
    pub span: ::treesitter_types::Span,
    pub body: FieldInitializerList<'tree>,
    pub name: StructExpressionName<'tree>,
}
impl<'tree> ::treesitter_types::FromNode<'tree> for StructExpression<'tree> {
    #[allow(clippy::match_single_binding, clippy::suspicious_else_formatting)]
    fn from_node(
        node: ::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        debug_assert_eq!(node.kind(), "struct_expression");
        Ok(Self {
            span: ::treesitter_types::Span::from(node),
            body: {
                let child = node
                    .child_by_field_name("body")
                    .ok_or_else(|| ::treesitter_types::ParseError::missing_field("body", node))?;
                ::treesitter_types::runtime::maybe_grow_stack(|| {
                    <FieldInitializerList as ::treesitter_types::FromNode>::from_node(child, src)
                })?
            },
            name: {
                let child = node
                    .child_by_field_name("name")
                    .ok_or_else(|| ::treesitter_types::ParseError::missing_field("name", node))?;
                ::treesitter_types::runtime::maybe_grow_stack(|| {
                    <StructExpressionName as ::treesitter_types::FromNode>::from_node(child, src)
                })?
            },
        })
    }
}
impl ::treesitter_types::Spanned for StructExpression<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        self.span
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct StructItem<'tree> {
    pub span: ::treesitter_types::Span,
    pub body: ::core::option::Option<StructItemBody<'tree>>,
    pub name: TypeIdentifier<'tree>,
    pub type_parameters: ::core::option::Option<TypeParameters<'tree>>,
    pub children: ::std::vec::Vec<StructItemChildren<'tree>>,
}
impl<'tree> ::treesitter_types::FromNode<'tree> for StructItem<'tree> {
    #[allow(clippy::match_single_binding, clippy::suspicious_else_formatting)]
    fn from_node(
        node: ::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        debug_assert_eq!(node.kind(), "struct_item");
        Ok(Self {
            span: ::treesitter_types::Span::from(node),
            body: match node.child_by_field_name("body") {
                Some(child) => Some(::treesitter_types::runtime::maybe_grow_stack(|| {
                    <StructItemBody as ::treesitter_types::FromNode>::from_node(child, src)
                })?),
                None => None,
            },
            name: {
                let child = node
                    .child_by_field_name("name")
                    .ok_or_else(|| ::treesitter_types::ParseError::missing_field("name", node))?;
                ::treesitter_types::runtime::maybe_grow_stack(|| {
                    <TypeIdentifier as ::treesitter_types::FromNode>::from_node(child, src)
                })?
            },
            type_parameters: match node.child_by_field_name("type_parameters") {
                Some(child) => Some(::treesitter_types::runtime::maybe_grow_stack(|| {
                    <TypeParameters as ::treesitter_types::FromNode>::from_node(child, src)
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
                        <StructItemChildren as ::treesitter_types::FromNode>::from_node(child, src)
                    })?);
                }
                items
            },
        })
    }
}
impl ::treesitter_types::Spanned for StructItem<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        self.span
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct StructPattern<'tree> {
    pub span: ::treesitter_types::Span,
    pub r#type: StructPatternType<'tree>,
    pub children: ::std::vec::Vec<StructPatternChildren<'tree>>,
}
impl<'tree> ::treesitter_types::FromNode<'tree> for StructPattern<'tree> {
    #[allow(clippy::match_single_binding, clippy::suspicious_else_formatting)]
    fn from_node(
        node: ::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        debug_assert_eq!(node.kind(), "struct_pattern");
        Ok(Self {
            span: ::treesitter_types::Span::from(node),
            r#type: {
                let child = node
                    .child_by_field_name("type")
                    .ok_or_else(|| ::treesitter_types::ParseError::missing_field("type", node))?;
                ::treesitter_types::runtime::maybe_grow_stack(|| {
                    <StructPatternType as ::treesitter_types::FromNode>::from_node(child, src)
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
                        <StructPatternChildren as ::treesitter_types::FromNode>::from_node(
                            child, src,
                        )
                    })?);
                }
                items
            },
        })
    }
}
impl ::treesitter_types::Spanned for StructPattern<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        self.span
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct TokenBindingPattern<'tree> {
    pub span: ::treesitter_types::Span,
    pub name: Metavariable<'tree>,
    pub r#type: FragmentSpecifier<'tree>,
}
impl<'tree> ::treesitter_types::FromNode<'tree> for TokenBindingPattern<'tree> {
    #[allow(clippy::match_single_binding, clippy::suspicious_else_formatting)]
    fn from_node(
        node: ::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        debug_assert_eq!(node.kind(), "token_binding_pattern");
        Ok(Self {
            span: ::treesitter_types::Span::from(node),
            name: {
                let child = node
                    .child_by_field_name("name")
                    .ok_or_else(|| ::treesitter_types::ParseError::missing_field("name", node))?;
                ::treesitter_types::runtime::maybe_grow_stack(|| {
                    <Metavariable as ::treesitter_types::FromNode>::from_node(child, src)
                })?
            },
            r#type: {
                let child = node
                    .child_by_field_name("type")
                    .ok_or_else(|| ::treesitter_types::ParseError::missing_field("type", node))?;
                ::treesitter_types::runtime::maybe_grow_stack(|| {
                    <FragmentSpecifier as ::treesitter_types::FromNode>::from_node(child, src)
                })?
            },
        })
    }
}
impl ::treesitter_types::Spanned for TokenBindingPattern<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        self.span
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct TokenRepetition<'tree> {
    pub span: ::treesitter_types::Span,
    pub children: ::std::vec::Vec<TokenRepetitionChildren<'tree>>,
}
impl<'tree> ::treesitter_types::FromNode<'tree> for TokenRepetition<'tree> {
    #[allow(clippy::match_single_binding, clippy::suspicious_else_formatting)]
    fn from_node(
        node: ::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        debug_assert_eq!(node.kind(), "token_repetition");
        Ok(Self {
            span: ::treesitter_types::Span::from(node),
            children: {
                #[allow(clippy::suspicious_else_formatting)]
                let non_field_children = {
                    let mut cursor = node.walk();
                    let mut result = ::std::vec::Vec::new();
                    if cursor.goto_first_child() {
                        loop {
                            if cursor.field_name().is_none()
                                && cursor.node().is_named()
                                && !cursor.node().is_extra()
                            {
                                result.push(cursor.node());
                            }
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
                        <TokenRepetitionChildren as ::treesitter_types::FromNode>::from_node(
                            child, src,
                        )
                    })?);
                }
                items
            },
        })
    }
}
impl ::treesitter_types::Spanned for TokenRepetition<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        self.span
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct TokenRepetitionPattern<'tree> {
    pub span: ::treesitter_types::Span,
    pub children: ::std::vec::Vec<TokenRepetitionPatternChildren<'tree>>,
}
impl<'tree> ::treesitter_types::FromNode<'tree> for TokenRepetitionPattern<'tree> {
    #[allow(clippy::match_single_binding, clippy::suspicious_else_formatting)]
    fn from_node(
        node: ::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        debug_assert_eq!(node.kind(), "token_repetition_pattern");
        Ok(Self {
            span: ::treesitter_types::Span::from(node),
            children: {
                #[allow(clippy::suspicious_else_formatting)]
                let non_field_children = {
                    let mut cursor = node.walk();
                    let mut result = ::std::vec::Vec::new();
                    if cursor.goto_first_child() {
                        loop {
                            if cursor.field_name().is_none()
                                && cursor.node().is_named()
                                && !cursor.node().is_extra()
                            {
                                result.push(cursor.node());
                            }
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
                        <TokenRepetitionPatternChildren as ::treesitter_types::FromNode>::from_node(
                            child, src,
                        )
                    })?);
                }
                items
            },
        })
    }
}
impl ::treesitter_types::Spanned for TokenRepetitionPattern<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        self.span
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct TokenTree<'tree> {
    pub span: ::treesitter_types::Span,
    pub children: ::std::vec::Vec<TokenTreeChildren<'tree>>,
}
impl<'tree> ::treesitter_types::FromNode<'tree> for TokenTree<'tree> {
    #[allow(clippy::match_single_binding, clippy::suspicious_else_formatting)]
    fn from_node(
        node: ::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        debug_assert_eq!(node.kind(), "token_tree");
        Ok(Self {
            span: ::treesitter_types::Span::from(node),
            children: {
                #[allow(clippy::suspicious_else_formatting)]
                let non_field_children = {
                    let mut cursor = node.walk();
                    let mut result = ::std::vec::Vec::new();
                    if cursor.goto_first_child() {
                        loop {
                            if cursor.field_name().is_none()
                                && cursor.node().is_named()
                                && !cursor.node().is_extra()
                            {
                                result.push(cursor.node());
                            }
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
                        <TokenTreeChildren as ::treesitter_types::FromNode>::from_node(child, src)
                    })?);
                }
                items
            },
        })
    }
}
impl ::treesitter_types::Spanned for TokenTree<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        self.span
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct TokenTreePattern<'tree> {
    pub span: ::treesitter_types::Span,
    pub children: ::std::vec::Vec<TokenTreePatternChildren<'tree>>,
}
impl<'tree> ::treesitter_types::FromNode<'tree> for TokenTreePattern<'tree> {
    #[allow(clippy::match_single_binding, clippy::suspicious_else_formatting)]
    fn from_node(
        node: ::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        debug_assert_eq!(node.kind(), "token_tree_pattern");
        Ok(Self {
            span: ::treesitter_types::Span::from(node),
            children: {
                #[allow(clippy::suspicious_else_formatting)]
                let non_field_children = {
                    let mut cursor = node.walk();
                    let mut result = ::std::vec::Vec::new();
                    if cursor.goto_first_child() {
                        loop {
                            if cursor.field_name().is_none()
                                && cursor.node().is_named()
                                && !cursor.node().is_extra()
                            {
                                result.push(cursor.node());
                            }
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
                        <TokenTreePatternChildren as ::treesitter_types::FromNode>::from_node(
                            child, src,
                        )
                    })?);
                }
                items
            },
        })
    }
}
impl ::treesitter_types::Spanned for TokenTreePattern<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        self.span
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct TraitBounds<'tree> {
    pub span: ::treesitter_types::Span,
    pub children: ::std::vec::Vec<TraitBoundsChildren<'tree>>,
}
impl<'tree> ::treesitter_types::FromNode<'tree> for TraitBounds<'tree> {
    #[allow(clippy::match_single_binding, clippy::suspicious_else_formatting)]
    fn from_node(
        node: ::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        debug_assert_eq!(node.kind(), "trait_bounds");
        Ok(Self {
            span: ::treesitter_types::Span::from(node),
            children: {
                #[allow(clippy::suspicious_else_formatting)]
                let non_field_children = {
                    let mut cursor = node.walk();
                    let mut result = ::std::vec::Vec::new();
                    if cursor.goto_first_child() {
                        loop {
                            if cursor.field_name().is_none()
                                && cursor.node().is_named()
                                && !cursor.node().is_extra()
                            {
                                result.push(cursor.node());
                            }
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
                        <TraitBoundsChildren as ::treesitter_types::FromNode>::from_node(child, src)
                    })?);
                }
                items
            },
        })
    }
}
impl ::treesitter_types::Spanned for TraitBounds<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        self.span
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct TraitItem<'tree> {
    pub span: ::treesitter_types::Span,
    pub body: DeclarationList<'tree>,
    pub bounds: ::core::option::Option<TraitBounds<'tree>>,
    pub name: TypeIdentifier<'tree>,
    pub type_parameters: ::core::option::Option<TypeParameters<'tree>>,
    pub children: ::std::vec::Vec<TraitItemChildren<'tree>>,
}
impl<'tree> ::treesitter_types::FromNode<'tree> for TraitItem<'tree> {
    #[allow(clippy::match_single_binding, clippy::suspicious_else_formatting)]
    fn from_node(
        node: ::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        debug_assert_eq!(node.kind(), "trait_item");
        Ok(Self {
            span: ::treesitter_types::Span::from(node),
            body: {
                let child = node
                    .child_by_field_name("body")
                    .ok_or_else(|| ::treesitter_types::ParseError::missing_field("body", node))?;
                ::treesitter_types::runtime::maybe_grow_stack(|| {
                    <DeclarationList as ::treesitter_types::FromNode>::from_node(child, src)
                })?
            },
            bounds: match node.child_by_field_name("bounds") {
                Some(child) => Some(::treesitter_types::runtime::maybe_grow_stack(|| {
                    <TraitBounds as ::treesitter_types::FromNode>::from_node(child, src)
                })?),
                None => None,
            },
            name: {
                let child = node
                    .child_by_field_name("name")
                    .ok_or_else(|| ::treesitter_types::ParseError::missing_field("name", node))?;
                ::treesitter_types::runtime::maybe_grow_stack(|| {
                    <TypeIdentifier as ::treesitter_types::FromNode>::from_node(child, src)
                })?
            },
            type_parameters: match node.child_by_field_name("type_parameters") {
                Some(child) => Some(::treesitter_types::runtime::maybe_grow_stack(|| {
                    <TypeParameters as ::treesitter_types::FromNode>::from_node(child, src)
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
                        <TraitItemChildren as ::treesitter_types::FromNode>::from_node(child, src)
                    })?);
                }
                items
            },
        })
    }
}
impl ::treesitter_types::Spanned for TraitItem<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        self.span
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct TryBlock<'tree> {
    pub span: ::treesitter_types::Span,
    pub children: Block<'tree>,
}
impl<'tree> ::treesitter_types::FromNode<'tree> for TryBlock<'tree> {
    #[allow(clippy::match_single_binding, clippy::suspicious_else_formatting)]
    fn from_node(
        node: ::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        debug_assert_eq!(node.kind(), "try_block");
        Ok(Self {
            span: ::treesitter_types::Span::from(node),
            children: {
                #[allow(clippy::suspicious_else_formatting)]
                let non_field_children = {
                    let mut cursor = node.walk();
                    let mut result = ::std::vec::Vec::new();
                    if cursor.goto_first_child() {
                        loop {
                            if cursor.field_name().is_none()
                                && cursor.node().is_named()
                                && !cursor.node().is_extra()
                            {
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
                                        ::treesitter_types::runtime::maybe_grow_stack(|| <Block as ::treesitter_types::FromNode>::from_node(
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
                                            ::treesitter_types::runtime::maybe_grow_stack(|| <Block as ::treesitter_types::FromNode>::from_node(
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
                    <Block as ::treesitter_types::FromNode>::from_node(child, src)
                })?
            },
        })
    }
}
impl ::treesitter_types::Spanned for TryBlock<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        self.span
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct TryExpression<'tree> {
    pub span: ::treesitter_types::Span,
    pub children: Expression<'tree>,
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
            children: {
                #[allow(clippy::suspicious_else_formatting)]
                let non_field_children = {
                    let mut cursor = node.walk();
                    let mut result = ::std::vec::Vec::new();
                    if cursor.goto_first_child() {
                        loop {
                            if cursor.field_name().is_none()
                                && cursor.node().is_named()
                                && !cursor.node().is_extra()
                            {
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
impl ::treesitter_types::Spanned for TryExpression<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        self.span
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct TupleExpression<'tree> {
    pub span: ::treesitter_types::Span,
    pub children: ::std::vec::Vec<TupleExpressionChildren<'tree>>,
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
                    items.push(::treesitter_types::runtime::maybe_grow_stack(|| {
                        <TupleExpressionChildren as ::treesitter_types::FromNode>::from_node(
                            child, src,
                        )
                    })?);
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
#[derive(Debug, Clone, PartialEq, Eq)]
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
                    items.push(::treesitter_types::runtime::maybe_grow_stack(|| {
                        <TuplePatternChildren as ::treesitter_types::FromNode>::from_node(
                            child, src,
                        )
                    })?);
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
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct TupleStructPattern<'tree> {
    pub span: ::treesitter_types::Span,
    pub r#type: TupleStructPatternType<'tree>,
    pub children: ::std::vec::Vec<Pattern<'tree>>,
}
impl<'tree> ::treesitter_types::FromNode<'tree> for TupleStructPattern<'tree> {
    #[allow(clippy::match_single_binding, clippy::suspicious_else_formatting)]
    fn from_node(
        node: ::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        debug_assert_eq!(node.kind(), "tuple_struct_pattern");
        Ok(Self {
            span: ::treesitter_types::Span::from(node),
            r#type: {
                let child = node
                    .child_by_field_name("type")
                    .ok_or_else(|| ::treesitter_types::ParseError::missing_field("type", node))?;
                ::treesitter_types::runtime::maybe_grow_stack(|| {
                    <TupleStructPatternType as ::treesitter_types::FromNode>::from_node(child, src)
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
                        <Pattern as ::treesitter_types::FromNode>::from_node(child, src)
                    })?);
                }
                items
            },
        })
    }
}
impl ::treesitter_types::Spanned for TupleStructPattern<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        self.span
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct TupleType<'tree> {
    pub span: ::treesitter_types::Span,
    pub children: ::std::vec::Vec<Type<'tree>>,
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
                    items.push(::treesitter_types::runtime::maybe_grow_stack(|| {
                        <Type as ::treesitter_types::FromNode>::from_node(child, src)
                    })?);
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
#[derive(Debug, Clone, PartialEq, Eq)]
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
                    items.push(::treesitter_types::runtime::maybe_grow_stack(|| {
                        <TypeArgumentsChildren as ::treesitter_types::FromNode>::from_node(
                            child, src,
                        )
                    })?);
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
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct TypeBinding<'tree> {
    pub span: ::treesitter_types::Span,
    pub name: TypeIdentifier<'tree>,
    pub r#type: Type<'tree>,
    pub type_arguments: ::core::option::Option<TypeArguments<'tree>>,
}
impl<'tree> ::treesitter_types::FromNode<'tree> for TypeBinding<'tree> {
    #[allow(clippy::match_single_binding, clippy::suspicious_else_formatting)]
    fn from_node(
        node: ::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        debug_assert_eq!(node.kind(), "type_binding");
        Ok(Self {
            span: ::treesitter_types::Span::from(node),
            name: {
                let child = node
                    .child_by_field_name("name")
                    .ok_or_else(|| ::treesitter_types::ParseError::missing_field("name", node))?;
                ::treesitter_types::runtime::maybe_grow_stack(|| {
                    <TypeIdentifier as ::treesitter_types::FromNode>::from_node(child, src)
                })?
            },
            r#type: {
                let child = node
                    .child_by_field_name("type")
                    .ok_or_else(|| ::treesitter_types::ParseError::missing_field("type", node))?;
                ::treesitter_types::runtime::maybe_grow_stack(|| {
                    <Type as ::treesitter_types::FromNode>::from_node(child, src)
                })?
            },
            type_arguments: match node.child_by_field_name("type_arguments") {
                Some(child) => Some(::treesitter_types::runtime::maybe_grow_stack(|| {
                    <TypeArguments as ::treesitter_types::FromNode>::from_node(child, src)
                })?),
                None => None,
            },
        })
    }
}
impl ::treesitter_types::Spanned for TypeBinding<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        self.span
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct TypeCastExpression<'tree> {
    pub span: ::treesitter_types::Span,
    pub r#type: Type<'tree>,
    pub value: Expression<'tree>,
}
impl<'tree> ::treesitter_types::FromNode<'tree> for TypeCastExpression<'tree> {
    #[allow(clippy::match_single_binding, clippy::suspicious_else_formatting)]
    fn from_node(
        node: ::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        debug_assert_eq!(node.kind(), "type_cast_expression");
        Ok(Self {
            span: ::treesitter_types::Span::from(node),
            r#type: {
                let child = node
                    .child_by_field_name("type")
                    .ok_or_else(|| ::treesitter_types::ParseError::missing_field("type", node))?;
                ::treesitter_types::runtime::maybe_grow_stack(|| {
                    <Type as ::treesitter_types::FromNode>::from_node(child, src)
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
impl ::treesitter_types::Spanned for TypeCastExpression<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        self.span
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct TypeItem<'tree> {
    pub span: ::treesitter_types::Span,
    pub name: TypeIdentifier<'tree>,
    pub r#type: Type<'tree>,
    pub type_parameters: ::core::option::Option<TypeParameters<'tree>>,
    pub children: ::std::vec::Vec<TypeItemChildren<'tree>>,
}
impl<'tree> ::treesitter_types::FromNode<'tree> for TypeItem<'tree> {
    #[allow(clippy::match_single_binding, clippy::suspicious_else_formatting)]
    fn from_node(
        node: ::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        debug_assert_eq!(node.kind(), "type_item");
        Ok(Self {
            span: ::treesitter_types::Span::from(node),
            name: {
                let child = node
                    .child_by_field_name("name")
                    .ok_or_else(|| ::treesitter_types::ParseError::missing_field("name", node))?;
                ::treesitter_types::runtime::maybe_grow_stack(|| {
                    <TypeIdentifier as ::treesitter_types::FromNode>::from_node(child, src)
                })?
            },
            r#type: {
                let child = node
                    .child_by_field_name("type")
                    .ok_or_else(|| ::treesitter_types::ParseError::missing_field("type", node))?;
                ::treesitter_types::runtime::maybe_grow_stack(|| {
                    <Type as ::treesitter_types::FromNode>::from_node(child, src)
                })?
            },
            type_parameters: match node.child_by_field_name("type_parameters") {
                Some(child) => Some(::treesitter_types::runtime::maybe_grow_stack(|| {
                    <TypeParameters as ::treesitter_types::FromNode>::from_node(child, src)
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
                        <TypeItemChildren as ::treesitter_types::FromNode>::from_node(child, src)
                    })?);
                }
                items
            },
        })
    }
}
impl ::treesitter_types::Spanned for TypeItem<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        self.span
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct TypeParameter<'tree> {
    pub span: ::treesitter_types::Span,
    pub bounds: ::core::option::Option<TraitBounds<'tree>>,
    pub default_type: ::core::option::Option<Type<'tree>>,
    pub name: TypeIdentifier<'tree>,
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
            bounds: match node.child_by_field_name("bounds") {
                Some(child) => Some(::treesitter_types::runtime::maybe_grow_stack(|| {
                    <TraitBounds as ::treesitter_types::FromNode>::from_node(child, src)
                })?),
                None => None,
            },
            default_type: match node.child_by_field_name("default_type") {
                Some(child) => Some(::treesitter_types::runtime::maybe_grow_stack(|| {
                    <Type as ::treesitter_types::FromNode>::from_node(child, src)
                })?),
                None => None,
            },
            name: {
                let child = node
                    .child_by_field_name("name")
                    .ok_or_else(|| ::treesitter_types::ParseError::missing_field("name", node))?;
                ::treesitter_types::runtime::maybe_grow_stack(|| {
                    <TypeIdentifier as ::treesitter_types::FromNode>::from_node(child, src)
                })?
            },
        })
    }
}
impl ::treesitter_types::Spanned for TypeParameter<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        self.span
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct TypeParameters<'tree> {
    pub span: ::treesitter_types::Span,
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
            children: {
                #[allow(clippy::suspicious_else_formatting)]
                let non_field_children = {
                    let mut cursor = node.walk();
                    let mut result = ::std::vec::Vec::new();
                    if cursor.goto_first_child() {
                        loop {
                            if cursor.field_name().is_none()
                                && cursor.node().is_named()
                                && !cursor.node().is_extra()
                            {
                                result.push(cursor.node());
                            }
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
                        <TypeParametersChildren as ::treesitter_types::FromNode>::from_node(
                            child, src,
                        )
                    })?);
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
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct UnaryExpression<'tree> {
    pub span: ::treesitter_types::Span,
    pub children: Expression<'tree>,
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
            children: {
                #[allow(clippy::suspicious_else_formatting)]
                let non_field_children = {
                    let mut cursor = node.walk();
                    let mut result = ::std::vec::Vec::new();
                    if cursor.goto_first_child() {
                        loop {
                            if cursor.field_name().is_none()
                                && cursor.node().is_named()
                                && !cursor.node().is_extra()
                            {
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
impl ::treesitter_types::Spanned for UnaryExpression<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        self.span
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct UnionItem<'tree> {
    pub span: ::treesitter_types::Span,
    pub body: FieldDeclarationList<'tree>,
    pub name: TypeIdentifier<'tree>,
    pub type_parameters: ::core::option::Option<TypeParameters<'tree>>,
    pub children: ::std::vec::Vec<UnionItemChildren<'tree>>,
}
impl<'tree> ::treesitter_types::FromNode<'tree> for UnionItem<'tree> {
    #[allow(clippy::match_single_binding, clippy::suspicious_else_formatting)]
    fn from_node(
        node: ::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        debug_assert_eq!(node.kind(), "union_item");
        Ok(Self {
            span: ::treesitter_types::Span::from(node),
            body: {
                let child = node
                    .child_by_field_name("body")
                    .ok_or_else(|| ::treesitter_types::ParseError::missing_field("body", node))?;
                ::treesitter_types::runtime::maybe_grow_stack(|| {
                    <FieldDeclarationList as ::treesitter_types::FromNode>::from_node(child, src)
                })?
            },
            name: {
                let child = node
                    .child_by_field_name("name")
                    .ok_or_else(|| ::treesitter_types::ParseError::missing_field("name", node))?;
                ::treesitter_types::runtime::maybe_grow_stack(|| {
                    <TypeIdentifier as ::treesitter_types::FromNode>::from_node(child, src)
                })?
            },
            type_parameters: match node.child_by_field_name("type_parameters") {
                Some(child) => Some(::treesitter_types::runtime::maybe_grow_stack(|| {
                    <TypeParameters as ::treesitter_types::FromNode>::from_node(child, src)
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
                        <UnionItemChildren as ::treesitter_types::FromNode>::from_node(child, src)
                    })?);
                }
                items
            },
        })
    }
}
impl ::treesitter_types::Spanned for UnionItem<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        self.span
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct UnitExpression<'tree> {
    pub span: ::treesitter_types::Span,
    text: &'tree str,
}
impl<'tree> ::treesitter_types::FromNode<'tree> for UnitExpression<'tree> {
    fn from_node(
        node: ::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        debug_assert_eq!(node.kind(), "unit_expression");
        Ok(Self {
            span: ::treesitter_types::Span::from(node),
            text: node.utf8_text(src)?,
        })
    }
}
impl<'tree> ::treesitter_types::LeafNode<'tree> for UnitExpression<'tree> {
    fn text(&self) -> &'tree str {
        self.text
    }
}
impl ::treesitter_types::Spanned for UnitExpression<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        self.span
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct UnitType<'tree> {
    pub span: ::treesitter_types::Span,
    text: &'tree str,
}
impl<'tree> ::treesitter_types::FromNode<'tree> for UnitType<'tree> {
    fn from_node(
        node: ::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        debug_assert_eq!(node.kind(), "unit_type");
        Ok(Self {
            span: ::treesitter_types::Span::from(node),
            text: node.utf8_text(src)?,
        })
    }
}
impl<'tree> ::treesitter_types::LeafNode<'tree> for UnitType<'tree> {
    fn text(&self) -> &'tree str {
        self.text
    }
}
impl ::treesitter_types::Spanned for UnitType<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        self.span
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct UnsafeBlock<'tree> {
    pub span: ::treesitter_types::Span,
    pub children: Block<'tree>,
}
impl<'tree> ::treesitter_types::FromNode<'tree> for UnsafeBlock<'tree> {
    #[allow(clippy::match_single_binding, clippy::suspicious_else_formatting)]
    fn from_node(
        node: ::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        debug_assert_eq!(node.kind(), "unsafe_block");
        Ok(Self {
            span: ::treesitter_types::Span::from(node),
            children: {
                #[allow(clippy::suspicious_else_formatting)]
                let non_field_children = {
                    let mut cursor = node.walk();
                    let mut result = ::std::vec::Vec::new();
                    if cursor.goto_first_child() {
                        loop {
                            if cursor.field_name().is_none()
                                && cursor.node().is_named()
                                && !cursor.node().is_extra()
                            {
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
                                        ::treesitter_types::runtime::maybe_grow_stack(|| <Block as ::treesitter_types::FromNode>::from_node(
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
                                            ::treesitter_types::runtime::maybe_grow_stack(|| <Block as ::treesitter_types::FromNode>::from_node(
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
                    <Block as ::treesitter_types::FromNode>::from_node(child, src)
                })?
            },
        })
    }
}
impl ::treesitter_types::Spanned for UnsafeBlock<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        self.span
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct UseAsClause<'tree> {
    pub span: ::treesitter_types::Span,
    pub alias: Identifier<'tree>,
    pub path: UseAsClausePath<'tree>,
}
impl<'tree> ::treesitter_types::FromNode<'tree> for UseAsClause<'tree> {
    #[allow(clippy::match_single_binding, clippy::suspicious_else_formatting)]
    fn from_node(
        node: ::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        debug_assert_eq!(node.kind(), "use_as_clause");
        Ok(Self {
            span: ::treesitter_types::Span::from(node),
            alias: {
                let child = node
                    .child_by_field_name("alias")
                    .ok_or_else(|| ::treesitter_types::ParseError::missing_field("alias", node))?;
                ::treesitter_types::runtime::maybe_grow_stack(|| {
                    <Identifier as ::treesitter_types::FromNode>::from_node(child, src)
                })?
            },
            path: {
                let child = node
                    .child_by_field_name("path")
                    .ok_or_else(|| ::treesitter_types::ParseError::missing_field("path", node))?;
                ::treesitter_types::runtime::maybe_grow_stack(|| {
                    <UseAsClausePath as ::treesitter_types::FromNode>::from_node(child, src)
                })?
            },
        })
    }
}
impl ::treesitter_types::Spanned for UseAsClause<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        self.span
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct UseBounds<'tree> {
    pub span: ::treesitter_types::Span,
    pub children: ::std::vec::Vec<UseBoundsChildren<'tree>>,
}
impl<'tree> ::treesitter_types::FromNode<'tree> for UseBounds<'tree> {
    #[allow(clippy::match_single_binding, clippy::suspicious_else_formatting)]
    fn from_node(
        node: ::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        debug_assert_eq!(node.kind(), "use_bounds");
        Ok(Self {
            span: ::treesitter_types::Span::from(node),
            children: {
                #[allow(clippy::suspicious_else_formatting)]
                let non_field_children = {
                    let mut cursor = node.walk();
                    let mut result = ::std::vec::Vec::new();
                    if cursor.goto_first_child() {
                        loop {
                            if cursor.field_name().is_none()
                                && cursor.node().is_named()
                                && !cursor.node().is_extra()
                            {
                                result.push(cursor.node());
                            }
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
                        <UseBoundsChildren as ::treesitter_types::FromNode>::from_node(child, src)
                    })?);
                }
                items
            },
        })
    }
}
impl ::treesitter_types::Spanned for UseBounds<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        self.span
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct UseDeclaration<'tree> {
    pub span: ::treesitter_types::Span,
    pub argument: UseDeclarationArgument<'tree>,
    pub children: ::core::option::Option<VisibilityModifier<'tree>>,
}
impl<'tree> ::treesitter_types::FromNode<'tree> for UseDeclaration<'tree> {
    #[allow(clippy::match_single_binding, clippy::suspicious_else_formatting)]
    fn from_node(
        node: ::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        debug_assert_eq!(node.kind(), "use_declaration");
        Ok(Self {
            span: ::treesitter_types::Span::from(node),
            argument: {
                let child = node.child_by_field_name("argument").ok_or_else(|| {
                    ::treesitter_types::ParseError::missing_field("argument", node)
                })?;
                ::treesitter_types::runtime::maybe_grow_stack(|| {
                    <UseDeclarationArgument as ::treesitter_types::FromNode>::from_node(child, src)
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
                match non_field_children.first() {
                    Some(&child) => Some(::treesitter_types::runtime::maybe_grow_stack(|| {
                        <VisibilityModifier as ::treesitter_types::FromNode>::from_node(child, src)
                    })?),
                    None => None,
                }
            },
        })
    }
}
impl ::treesitter_types::Spanned for UseDeclaration<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        self.span
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct UseList<'tree> {
    pub span: ::treesitter_types::Span,
    pub children: ::std::vec::Vec<UseListChildren<'tree>>,
}
impl<'tree> ::treesitter_types::FromNode<'tree> for UseList<'tree> {
    #[allow(clippy::match_single_binding, clippy::suspicious_else_formatting)]
    fn from_node(
        node: ::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        debug_assert_eq!(node.kind(), "use_list");
        Ok(Self {
            span: ::treesitter_types::Span::from(node),
            children: {
                #[allow(clippy::suspicious_else_formatting)]
                let non_field_children = {
                    let mut cursor = node.walk();
                    let mut result = ::std::vec::Vec::new();
                    if cursor.goto_first_child() {
                        loop {
                            if cursor.field_name().is_none()
                                && cursor.node().is_named()
                                && !cursor.node().is_extra()
                            {
                                result.push(cursor.node());
                            }
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
                        <UseListChildren as ::treesitter_types::FromNode>::from_node(child, src)
                    })?);
                }
                items
            },
        })
    }
}
impl ::treesitter_types::Spanned for UseList<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        self.span
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct UseWildcard<'tree> {
    pub span: ::treesitter_types::Span,
    pub children: ::core::option::Option<UseWildcardChildren<'tree>>,
}
impl<'tree> ::treesitter_types::FromNode<'tree> for UseWildcard<'tree> {
    #[allow(clippy::match_single_binding, clippy::suspicious_else_formatting)]
    fn from_node(
        node: ::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        debug_assert_eq!(node.kind(), "use_wildcard");
        Ok(Self {
            span: ::treesitter_types::Span::from(node),
            children: {
                #[allow(clippy::suspicious_else_formatting)]
                let non_field_children = {
                    let mut cursor = node.walk();
                    let mut result = ::std::vec::Vec::new();
                    if cursor.goto_first_child() {
                        loop {
                            if cursor.field_name().is_none()
                                && cursor.node().is_named()
                                && !cursor.node().is_extra()
                            {
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
                        <UseWildcardChildren as ::treesitter_types::FromNode>::from_node(child, src)
                    })?),
                    None => None,
                }
            },
        })
    }
}
impl ::treesitter_types::Spanned for UseWildcard<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        self.span
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct VariadicParameter<'tree> {
    pub span: ::treesitter_types::Span,
    pub pattern: ::core::option::Option<Pattern<'tree>>,
    pub children: ::core::option::Option<MutableSpecifier<'tree>>,
}
impl<'tree> ::treesitter_types::FromNode<'tree> for VariadicParameter<'tree> {
    #[allow(clippy::match_single_binding, clippy::suspicious_else_formatting)]
    fn from_node(
        node: ::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        debug_assert_eq!(node.kind(), "variadic_parameter");
        Ok(Self {
            span: ::treesitter_types::Span::from(node),
            pattern: match node.child_by_field_name("pattern") {
                Some(child) => Some(::treesitter_types::runtime::maybe_grow_stack(|| {
                    <Pattern as ::treesitter_types::FromNode>::from_node(child, src)
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
                        <MutableSpecifier as ::treesitter_types::FromNode>::from_node(child, src)
                    })?),
                    None => None,
                }
            },
        })
    }
}
impl ::treesitter_types::Spanned for VariadicParameter<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        self.span
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct VisibilityModifier<'tree> {
    pub span: ::treesitter_types::Span,
    pub children: ::core::option::Option<VisibilityModifierChildren<'tree>>,
}
impl<'tree> ::treesitter_types::FromNode<'tree> for VisibilityModifier<'tree> {
    #[allow(clippy::match_single_binding, clippy::suspicious_else_formatting)]
    fn from_node(
        node: ::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        debug_assert_eq!(node.kind(), "visibility_modifier");
        Ok(Self {
            span: ::treesitter_types::Span::from(node),
            children: {
                #[allow(clippy::suspicious_else_formatting)]
                let non_field_children = {
                    let mut cursor = node.walk();
                    let mut result = ::std::vec::Vec::new();
                    if cursor.goto_first_child() {
                        loop {
                            if cursor.field_name().is_none()
                                && cursor.node().is_named()
                                && !cursor.node().is_extra()
                            {
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
                        <VisibilityModifierChildren as ::treesitter_types::FromNode>::from_node(
                            child, src,
                        )
                    })?),
                    None => None,
                }
            },
        })
    }
}
impl ::treesitter_types::Spanned for VisibilityModifier<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        self.span
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct WhereClause<'tree> {
    pub span: ::treesitter_types::Span,
    pub children: ::std::vec::Vec<WherePredicate<'tree>>,
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
                let mut items = ::std::vec::Vec::new();
                for child in non_field_children {
                    items.push(::treesitter_types::runtime::maybe_grow_stack(|| {
                        <WherePredicate as ::treesitter_types::FromNode>::from_node(child, src)
                    })?);
                }
                items
            },
        })
    }
}
impl ::treesitter_types::Spanned for WhereClause<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        self.span
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct WherePredicate<'tree> {
    pub span: ::treesitter_types::Span,
    pub bounds: TraitBounds<'tree>,
    pub left: WherePredicateLeft<'tree>,
}
impl<'tree> ::treesitter_types::FromNode<'tree> for WherePredicate<'tree> {
    #[allow(clippy::match_single_binding, clippy::suspicious_else_formatting)]
    fn from_node(
        node: ::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        debug_assert_eq!(node.kind(), "where_predicate");
        Ok(Self {
            span: ::treesitter_types::Span::from(node),
            bounds: {
                let child = node
                    .child_by_field_name("bounds")
                    .ok_or_else(|| ::treesitter_types::ParseError::missing_field("bounds", node))?;
                ::treesitter_types::runtime::maybe_grow_stack(|| {
                    <TraitBounds as ::treesitter_types::FromNode>::from_node(child, src)
                })?
            },
            left: {
                let child = node
                    .child_by_field_name("left")
                    .ok_or_else(|| ::treesitter_types::ParseError::missing_field("left", node))?;
                ::treesitter_types::runtime::maybe_grow_stack(|| {
                    <WherePredicateLeft as ::treesitter_types::FromNode>::from_node(child, src)
                })?
            },
        })
    }
}
impl ::treesitter_types::Spanned for WherePredicate<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        self.span
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct WhileExpression<'tree> {
    pub span: ::treesitter_types::Span,
    pub body: Block<'tree>,
    pub condition: WhileExpressionCondition<'tree>,
    pub children: ::core::option::Option<Label<'tree>>,
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
                ::treesitter_types::runtime::maybe_grow_stack(|| {
                    <Block as ::treesitter_types::FromNode>::from_node(child, src)
                })?
            },
            condition: {
                let child = node.child_by_field_name("condition").ok_or_else(|| {
                    ::treesitter_types::ParseError::missing_field("condition", node)
                })?;
                ::treesitter_types::runtime::maybe_grow_stack(|| {
                    <WhileExpressionCondition as ::treesitter_types::FromNode>::from_node(
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
                match non_field_children.first() {
                    Some(&child) => Some(::treesitter_types::runtime::maybe_grow_stack(|| {
                        <Label as ::treesitter_types::FromNode>::from_node(child, src)
                    })?),
                    None => None,
                }
            },
        })
    }
}
impl ::treesitter_types::Spanned for WhileExpression<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        self.span
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct YieldExpression<'tree> {
    pub span: ::treesitter_types::Span,
    pub children: ::core::option::Option<Expression<'tree>>,
}
impl<'tree> ::treesitter_types::FromNode<'tree> for YieldExpression<'tree> {
    #[allow(clippy::match_single_binding, clippy::suspicious_else_formatting)]
    fn from_node(
        node: ::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        debug_assert_eq!(node.kind(), "yield_expression");
        Ok(Self {
            span: ::treesitter_types::Span::from(node),
            children: {
                #[allow(clippy::suspicious_else_formatting)]
                let non_field_children = {
                    let mut cursor = node.walk();
                    let mut result = ::std::vec::Vec::new();
                    if cursor.goto_first_child() {
                        loop {
                            if cursor.field_name().is_none()
                                && cursor.node().is_named()
                                && !cursor.node().is_extra()
                            {
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
                        <Expression as ::treesitter_types::FromNode>::from_node(child, src)
                    })?),
                    None => None,
                }
            },
        })
    }
}
impl ::treesitter_types::Spanned for YieldExpression<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        self.span
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CharLiteral<'tree> {
    pub span: ::treesitter_types::Span,
    text: &'tree str,
}
impl<'tree> ::treesitter_types::FromNode<'tree> for CharLiteral<'tree> {
    fn from_node(
        node: ::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        debug_assert_eq!(node.kind(), "char_literal");
        Ok(Self {
            span: ::treesitter_types::Span::from(node),
            text: node.utf8_text(src)?,
        })
    }
}
impl<'tree> ::treesitter_types::LeafNode<'tree> for CharLiteral<'tree> {
    fn text(&self) -> &'tree str {
        self.text
    }
}
impl ::treesitter_types::Spanned for CharLiteral<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        self.span
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Crate<'tree> {
    pub span: ::treesitter_types::Span,
    text: &'tree str,
}
impl<'tree> ::treesitter_types::FromNode<'tree> for Crate<'tree> {
    fn from_node(
        node: ::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        debug_assert_eq!(node.kind(), "crate");
        Ok(Self {
            span: ::treesitter_types::Span::from(node),
            text: node.utf8_text(src)?,
        })
    }
}
impl<'tree> ::treesitter_types::LeafNode<'tree> for Crate<'tree> {
    fn text(&self) -> &'tree str {
        self.text
    }
}
impl ::treesitter_types::Spanned for Crate<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        self.span
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct DocComment<'tree> {
    pub span: ::treesitter_types::Span,
    text: &'tree str,
}
impl<'tree> ::treesitter_types::FromNode<'tree> for DocComment<'tree> {
    fn from_node(
        node: ::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        debug_assert_eq!(node.kind(), "doc_comment");
        Ok(Self {
            span: ::treesitter_types::Span::from(node),
            text: node.utf8_text(src)?,
        })
    }
}
impl<'tree> ::treesitter_types::LeafNode<'tree> for DocComment<'tree> {
    fn text(&self) -> &'tree str {
        self.text
    }
}
impl ::treesitter_types::Spanned for DocComment<'_> {
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
#[derive(Debug, Clone, PartialEq, Eq)]
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
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Metavariable<'tree> {
    pub span: ::treesitter_types::Span,
    text: &'tree str,
}
impl<'tree> ::treesitter_types::FromNode<'tree> for Metavariable<'tree> {
    fn from_node(
        node: ::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        debug_assert_eq!(node.kind(), "metavariable");
        Ok(Self {
            span: ::treesitter_types::Span::from(node),
            text: node.utf8_text(src)?,
        })
    }
}
impl<'tree> ::treesitter_types::LeafNode<'tree> for Metavariable<'tree> {
    fn text(&self) -> &'tree str {
        self.text
    }
}
impl ::treesitter_types::Spanned for Metavariable<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        self.span
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct MutableSpecifier<'tree> {
    pub span: ::treesitter_types::Span,
    text: &'tree str,
}
impl<'tree> ::treesitter_types::FromNode<'tree> for MutableSpecifier<'tree> {
    fn from_node(
        node: ::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        debug_assert_eq!(node.kind(), "mutable_specifier");
        Ok(Self {
            span: ::treesitter_types::Span::from(node),
            text: node.utf8_text(src)?,
        })
    }
}
impl<'tree> ::treesitter_types::LeafNode<'tree> for MutableSpecifier<'tree> {
    fn text(&self) -> &'tree str {
        self.text
    }
}
impl ::treesitter_types::Spanned for MutableSpecifier<'_> {
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
pub struct Shebang<'tree> {
    pub span: ::treesitter_types::Span,
    text: &'tree str,
}
impl<'tree> ::treesitter_types::FromNode<'tree> for Shebang<'tree> {
    fn from_node(
        node: ::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        debug_assert_eq!(node.kind(), "shebang");
        Ok(Self {
            span: ::treesitter_types::Span::from(node),
            text: node.utf8_text(src)?,
        })
    }
}
impl<'tree> ::treesitter_types::LeafNode<'tree> for Shebang<'tree> {
    fn text(&self) -> &'tree str {
        self.text
    }
}
impl ::treesitter_types::Spanned for Shebang<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        self.span
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ShorthandFieldIdentifier<'tree> {
    pub span: ::treesitter_types::Span,
    text: &'tree str,
}
impl<'tree> ::treesitter_types::FromNode<'tree> for ShorthandFieldIdentifier<'tree> {
    fn from_node(
        node: ::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        debug_assert_eq!(node.kind(), "shorthand_field_identifier");
        Ok(Self {
            span: ::treesitter_types::Span::from(node),
            text: node.utf8_text(src)?,
        })
    }
}
impl<'tree> ::treesitter_types::LeafNode<'tree> for ShorthandFieldIdentifier<'tree> {
    fn text(&self) -> &'tree str {
        self.text
    }
}
impl ::treesitter_types::Spanned for ShorthandFieldIdentifier<'_> {
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
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum AbstractTypeTrait<'tree> {
    BoundedType(::std::boxed::Box<BoundedType<'tree>>),
    FunctionType(::std::boxed::Box<FunctionType<'tree>>),
    GenericType(::std::boxed::Box<GenericType<'tree>>),
    RemovedTraitBound(::std::boxed::Box<RemovedTraitBound<'tree>>),
    ScopedTypeIdentifier(::std::boxed::Box<ScopedTypeIdentifier<'tree>>),
    TupleType(::std::boxed::Box<TupleType<'tree>>),
    TypeIdentifier(::std::boxed::Box<TypeIdentifier<'tree>>),
}
impl<'tree> ::treesitter_types::FromNode<'tree> for AbstractTypeTrait<'tree> {
    #[allow(clippy::collapsible_else_if)]
    fn from_node(
        node: ::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        match node.kind() {
            "bounded_type" => Ok(Self::BoundedType(::std::boxed::Box::new(
                ::treesitter_types::runtime::maybe_grow_stack(|| {
                    <BoundedType as ::treesitter_types::FromNode>::from_node(node, src)
                })?,
            ))),
            "function_type" => Ok(Self::FunctionType(::std::boxed::Box::new(
                ::treesitter_types::runtime::maybe_grow_stack(|| {
                    <FunctionType as ::treesitter_types::FromNode>::from_node(node, src)
                })?,
            ))),
            "generic_type" => Ok(Self::GenericType(::std::boxed::Box::new(
                ::treesitter_types::runtime::maybe_grow_stack(|| {
                    <GenericType as ::treesitter_types::FromNode>::from_node(node, src)
                })?,
            ))),
            "removed_trait_bound" => Ok(Self::RemovedTraitBound(::std::boxed::Box::new(
                ::treesitter_types::runtime::maybe_grow_stack(|| {
                    <RemovedTraitBound as ::treesitter_types::FromNode>::from_node(node, src)
                })?,
            ))),
            "scoped_type_identifier" => Ok(Self::ScopedTypeIdentifier(::std::boxed::Box::new(
                ::treesitter_types::runtime::maybe_grow_stack(|| {
                    <ScopedTypeIdentifier as ::treesitter_types::FromNode>::from_node(node, src)
                })?,
            ))),
            "tuple_type" => Ok(Self::TupleType(::std::boxed::Box::new(
                ::treesitter_types::runtime::maybe_grow_stack(|| {
                    <TupleType as ::treesitter_types::FromNode>::from_node(node, src)
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
impl ::treesitter_types::Spanned for AbstractTypeTrait<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        match self {
            Self::BoundedType(inner) => inner.span(),
            Self::FunctionType(inner) => inner.span(),
            Self::GenericType(inner) => inner.span(),
            Self::RemovedTraitBound(inner) => inner.span(),
            Self::ScopedTypeIdentifier(inner) => inner.span(),
            Self::TupleType(inner) => inner.span(),
            Self::TypeIdentifier(inner) => inner.span(),
        }
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ArgumentsChildren<'tree> {
    Expression(::std::boxed::Box<Expression<'tree>>),
    AttributeItem(::std::boxed::Box<AttributeItem<'tree>>),
}
impl<'tree> ::treesitter_types::FromNode<'tree> for ArgumentsChildren<'tree> {
    #[allow(clippy::collapsible_else_if)]
    fn from_node(
        node: ::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        match node.kind() {
            "attribute_item" => Ok(Self::AttributeItem(::std::boxed::Box::new(
                ::treesitter_types::runtime::maybe_grow_stack(|| {
                    <AttributeItem as ::treesitter_types::FromNode>::from_node(node, src)
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
impl ::treesitter_types::Spanned for ArgumentsChildren<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        match self {
            Self::Expression(inner) => inner.span(),
            Self::AttributeItem(inner) => inner.span(),
        }
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ArrayExpressionChildren<'tree> {
    Expression(::std::boxed::Box<Expression<'tree>>),
    AttributeItem(::std::boxed::Box<AttributeItem<'tree>>),
}
impl<'tree> ::treesitter_types::FromNode<'tree> for ArrayExpressionChildren<'tree> {
    #[allow(clippy::collapsible_else_if)]
    fn from_node(
        node: ::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        match node.kind() {
            "attribute_item" => Ok(Self::AttributeItem(::std::boxed::Box::new(
                ::treesitter_types::runtime::maybe_grow_stack(|| {
                    <AttributeItem as ::treesitter_types::FromNode>::from_node(node, src)
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
impl ::treesitter_types::Spanned for ArrayExpressionChildren<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        match self {
            Self::Expression(inner) => inner.span(),
            Self::AttributeItem(inner) => inner.span(),
        }
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum AttributeChildren<'tree> {
    Crate(::std::boxed::Box<Crate<'tree>>),
    Identifier(::std::boxed::Box<Identifier<'tree>>),
    Metavariable(::std::boxed::Box<Metavariable<'tree>>),
    ScopedIdentifier(::std::boxed::Box<ScopedIdentifier<'tree>>),
    SelfType(::std::boxed::Box<SelfType<'tree>>),
    Super(::std::boxed::Box<Super<'tree>>),
}
impl<'tree> ::treesitter_types::FromNode<'tree> for AttributeChildren<'tree> {
    #[allow(clippy::collapsible_else_if)]
    fn from_node(
        node: ::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        match node.kind() {
            "crate" => Ok(Self::Crate(::std::boxed::Box::new(
                ::treesitter_types::runtime::maybe_grow_stack(|| {
                    <Crate as ::treesitter_types::FromNode>::from_node(node, src)
                })?,
            ))),
            "identifier" => Ok(Self::Identifier(::std::boxed::Box::new(
                ::treesitter_types::runtime::maybe_grow_stack(|| {
                    <Identifier as ::treesitter_types::FromNode>::from_node(node, src)
                })?,
            ))),
            "metavariable" => Ok(Self::Metavariable(::std::boxed::Box::new(
                ::treesitter_types::runtime::maybe_grow_stack(|| {
                    <Metavariable as ::treesitter_types::FromNode>::from_node(node, src)
                })?,
            ))),
            "scoped_identifier" => Ok(Self::ScopedIdentifier(::std::boxed::Box::new(
                ::treesitter_types::runtime::maybe_grow_stack(|| {
                    <ScopedIdentifier as ::treesitter_types::FromNode>::from_node(node, src)
                })?,
            ))),
            "self" => Ok(Self::SelfType(::std::boxed::Box::new(
                ::treesitter_types::runtime::maybe_grow_stack(|| {
                    <SelfType as ::treesitter_types::FromNode>::from_node(node, src)
                })?,
            ))),
            "super" => Ok(Self::Super(::std::boxed::Box::new(
                ::treesitter_types::runtime::maybe_grow_stack(|| {
                    <Super as ::treesitter_types::FromNode>::from_node(node, src)
                })?,
            ))),
            other => Err(::treesitter_types::ParseError::unexpected_kind(other, node)),
        }
    }
}
impl ::treesitter_types::Spanned for AttributeChildren<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        match self {
            Self::Crate(inner) => inner.span(),
            Self::Identifier(inner) => inner.span(),
            Self::Metavariable(inner) => inner.span(),
            Self::ScopedIdentifier(inner) => inner.span(),
            Self::SelfType(inner) => inner.span(),
            Self::Super(inner) => inner.span(),
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
pub enum BlockChildren<'tree> {
    DeclarationStatement(::std::boxed::Box<DeclarationStatement<'tree>>),
    Expression(::std::boxed::Box<Expression<'tree>>),
    ExpressionStatement(::std::boxed::Box<ExpressionStatement<'tree>>),
    Label(::std::boxed::Box<Label<'tree>>),
}
impl<'tree> ::treesitter_types::FromNode<'tree> for BlockChildren<'tree> {
    #[allow(clippy::collapsible_else_if)]
    fn from_node(
        node: ::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        match node.kind() {
            "expression_statement" => Ok(Self::ExpressionStatement(::std::boxed::Box::new(
                ::treesitter_types::runtime::maybe_grow_stack(|| {
                    <ExpressionStatement as ::treesitter_types::FromNode>::from_node(node, src)
                })?,
            ))),
            "label" => Ok(Self::Label(::std::boxed::Box::new(
                ::treesitter_types::runtime::maybe_grow_stack(|| {
                    <Label as ::treesitter_types::FromNode>::from_node(node, src)
                })?,
            ))),
            _other => {
                if let Ok(v) = ::treesitter_types::runtime::maybe_grow_stack(|| {
                    <DeclarationStatement as ::treesitter_types::FromNode>::from_node(node, src)
                }) {
                    Ok(Self::DeclarationStatement(::std::boxed::Box::new(v)))
                } else {
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
}
impl ::treesitter_types::Spanned for BlockChildren<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        match self {
            Self::DeclarationStatement(inner) => inner.span(),
            Self::Expression(inner) => inner.span(),
            Self::ExpressionStatement(inner) => inner.span(),
            Self::Label(inner) => inner.span(),
        }
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum BoundedTypeChildren<'tree> {
    Type(::std::boxed::Box<Type<'tree>>),
    Lifetime(::std::boxed::Box<Lifetime<'tree>>),
    UseBounds(::std::boxed::Box<UseBounds<'tree>>),
}
impl<'tree> ::treesitter_types::FromNode<'tree> for BoundedTypeChildren<'tree> {
    #[allow(clippy::collapsible_else_if)]
    fn from_node(
        node: ::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        match node.kind() {
            "lifetime" => Ok(Self::Lifetime(::std::boxed::Box::new(
                ::treesitter_types::runtime::maybe_grow_stack(|| {
                    <Lifetime as ::treesitter_types::FromNode>::from_node(node, src)
                })?,
            ))),
            "use_bounds" => Ok(Self::UseBounds(::std::boxed::Box::new(
                ::treesitter_types::runtime::maybe_grow_stack(|| {
                    <UseBounds as ::treesitter_types::FromNode>::from_node(node, src)
                })?,
            ))),
            _other => {
                if let Ok(v) = ::treesitter_types::runtime::maybe_grow_stack(|| {
                    <Type as ::treesitter_types::FromNode>::from_node(node, src)
                }) {
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
impl ::treesitter_types::Spanned for BoundedTypeChildren<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        match self {
            Self::Type(inner) => inner.span(),
            Self::Lifetime(inner) => inner.span(),
            Self::UseBounds(inner) => inner.span(),
        }
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum BracketedTypeChildren<'tree> {
    Type(::std::boxed::Box<Type<'tree>>),
    QualifiedType(::std::boxed::Box<QualifiedType<'tree>>),
}
impl<'tree> ::treesitter_types::FromNode<'tree> for BracketedTypeChildren<'tree> {
    #[allow(clippy::collapsible_else_if)]
    fn from_node(
        node: ::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        match node.kind() {
            "qualified_type" => Ok(Self::QualifiedType(::std::boxed::Box::new(
                ::treesitter_types::runtime::maybe_grow_stack(|| {
                    <QualifiedType as ::treesitter_types::FromNode>::from_node(node, src)
                })?,
            ))),
            _other => {
                if let Ok(v) = ::treesitter_types::runtime::maybe_grow_stack(|| {
                    <Type as ::treesitter_types::FromNode>::from_node(node, src)
                }) {
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
impl ::treesitter_types::Spanned for BracketedTypeChildren<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        match self {
            Self::Type(inner) => inner.span(),
            Self::QualifiedType(inner) => inner.span(),
        }
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum BreakExpressionChildren<'tree> {
    Expression(::std::boxed::Box<Expression<'tree>>),
    Label(::std::boxed::Box<Label<'tree>>),
}
impl<'tree> ::treesitter_types::FromNode<'tree> for BreakExpressionChildren<'tree> {
    #[allow(clippy::collapsible_else_if)]
    fn from_node(
        node: ::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        match node.kind() {
            "label" => Ok(Self::Label(::std::boxed::Box::new(
                ::treesitter_types::runtime::maybe_grow_stack(|| {
                    <Label as ::treesitter_types::FromNode>::from_node(node, src)
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
impl ::treesitter_types::Spanned for BreakExpressionChildren<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        match self {
            Self::Expression(inner) => inner.span(),
            Self::Label(inner) => inner.span(),
        }
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum CallExpressionFunction<'tree> {
    Literal(::std::boxed::Box<Literal<'tree>>),
    ArrayExpression(::std::boxed::Box<ArrayExpression<'tree>>),
    AssignmentExpression(::std::boxed::Box<AssignmentExpression<'tree>>),
    AsyncBlock(::std::boxed::Box<AsyncBlock<'tree>>),
    AwaitExpression(::std::boxed::Box<AwaitExpression<'tree>>),
    BinaryExpression(::std::boxed::Box<BinaryExpression<'tree>>),
    Block(::std::boxed::Box<Block<'tree>>),
    BreakExpression(::std::boxed::Box<BreakExpression<'tree>>),
    CallExpression(::std::boxed::Box<CallExpression<'tree>>),
    ClosureExpression(::std::boxed::Box<ClosureExpression<'tree>>),
    CompoundAssignmentExpr(::std::boxed::Box<CompoundAssignmentExpr<'tree>>),
    ConstBlock(::std::boxed::Box<ConstBlock<'tree>>),
    ContinueExpression(::std::boxed::Box<ContinueExpression<'tree>>),
    FieldExpression(::std::boxed::Box<FieldExpression<'tree>>),
    ForExpression(::std::boxed::Box<ForExpression<'tree>>),
    GenBlock(::std::boxed::Box<GenBlock<'tree>>),
    GenericFunction(::std::boxed::Box<GenericFunction<'tree>>),
    Identifier(::std::boxed::Box<Identifier<'tree>>),
    IfExpression(::std::boxed::Box<IfExpression<'tree>>),
    IndexExpression(::std::boxed::Box<IndexExpression<'tree>>),
    LoopExpression(::std::boxed::Box<LoopExpression<'tree>>),
    MacroInvocation(::std::boxed::Box<MacroInvocation<'tree>>),
    MatchExpression(::std::boxed::Box<MatchExpression<'tree>>),
    Metavariable(::std::boxed::Box<Metavariable<'tree>>),
    ParenthesizedExpression(::std::boxed::Box<ParenthesizedExpression<'tree>>),
    ReferenceExpression(::std::boxed::Box<ReferenceExpression<'tree>>),
    ReturnExpression(::std::boxed::Box<ReturnExpression<'tree>>),
    ScopedIdentifier(::std::boxed::Box<ScopedIdentifier<'tree>>),
    SelfType(::std::boxed::Box<SelfType<'tree>>),
    StructExpression(::std::boxed::Box<StructExpression<'tree>>),
    TryBlock(::std::boxed::Box<TryBlock<'tree>>),
    TryExpression(::std::boxed::Box<TryExpression<'tree>>),
    TupleExpression(::std::boxed::Box<TupleExpression<'tree>>),
    TypeCastExpression(::std::boxed::Box<TypeCastExpression<'tree>>),
    UnaryExpression(::std::boxed::Box<UnaryExpression<'tree>>),
    UnitExpression(::std::boxed::Box<UnitExpression<'tree>>),
    UnsafeBlock(::std::boxed::Box<UnsafeBlock<'tree>>),
    WhileExpression(::std::boxed::Box<WhileExpression<'tree>>),
    YieldExpression(::std::boxed::Box<YieldExpression<'tree>>),
}
impl<'tree> ::treesitter_types::FromNode<'tree> for CallExpressionFunction<'tree> {
    #[allow(clippy::collapsible_else_if)]
    fn from_node(
        node: ::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        match node.kind() {
            "array_expression" => Ok(Self::ArrayExpression(::std::boxed::Box::new(
                ::treesitter_types::runtime::maybe_grow_stack(|| {
                    <ArrayExpression as ::treesitter_types::FromNode>::from_node(node, src)
                })?,
            ))),
            "assignment_expression" => Ok(Self::AssignmentExpression(::std::boxed::Box::new(
                ::treesitter_types::runtime::maybe_grow_stack(|| {
                    <AssignmentExpression as ::treesitter_types::FromNode>::from_node(node, src)
                })?,
            ))),
            "async_block" => Ok(Self::AsyncBlock(::std::boxed::Box::new(
                ::treesitter_types::runtime::maybe_grow_stack(|| {
                    <AsyncBlock as ::treesitter_types::FromNode>::from_node(node, src)
                })?,
            ))),
            "await_expression" => Ok(Self::AwaitExpression(::std::boxed::Box::new(
                ::treesitter_types::runtime::maybe_grow_stack(|| {
                    <AwaitExpression as ::treesitter_types::FromNode>::from_node(node, src)
                })?,
            ))),
            "binary_expression" => Ok(Self::BinaryExpression(::std::boxed::Box::new(
                ::treesitter_types::runtime::maybe_grow_stack(|| {
                    <BinaryExpression as ::treesitter_types::FromNode>::from_node(node, src)
                })?,
            ))),
            "block" => Ok(Self::Block(::std::boxed::Box::new(
                ::treesitter_types::runtime::maybe_grow_stack(|| {
                    <Block as ::treesitter_types::FromNode>::from_node(node, src)
                })?,
            ))),
            "break_expression" => Ok(Self::BreakExpression(::std::boxed::Box::new(
                ::treesitter_types::runtime::maybe_grow_stack(|| {
                    <BreakExpression as ::treesitter_types::FromNode>::from_node(node, src)
                })?,
            ))),
            "call_expression" => Ok(Self::CallExpression(::std::boxed::Box::new(
                ::treesitter_types::runtime::maybe_grow_stack(|| {
                    <CallExpression as ::treesitter_types::FromNode>::from_node(node, src)
                })?,
            ))),
            "closure_expression" => Ok(Self::ClosureExpression(::std::boxed::Box::new(
                ::treesitter_types::runtime::maybe_grow_stack(|| {
                    <ClosureExpression as ::treesitter_types::FromNode>::from_node(node, src)
                })?,
            ))),
            "compound_assignment_expr" => Ok(Self::CompoundAssignmentExpr(::std::boxed::Box::new(
                ::treesitter_types::runtime::maybe_grow_stack(|| {
                    <CompoundAssignmentExpr as ::treesitter_types::FromNode>::from_node(node, src)
                })?,
            ))),
            "const_block" => Ok(Self::ConstBlock(::std::boxed::Box::new(
                ::treesitter_types::runtime::maybe_grow_stack(|| {
                    <ConstBlock as ::treesitter_types::FromNode>::from_node(node, src)
                })?,
            ))),
            "continue_expression" => Ok(Self::ContinueExpression(::std::boxed::Box::new(
                ::treesitter_types::runtime::maybe_grow_stack(|| {
                    <ContinueExpression as ::treesitter_types::FromNode>::from_node(node, src)
                })?,
            ))),
            "field_expression" => Ok(Self::FieldExpression(::std::boxed::Box::new(
                ::treesitter_types::runtime::maybe_grow_stack(|| {
                    <FieldExpression as ::treesitter_types::FromNode>::from_node(node, src)
                })?,
            ))),
            "for_expression" => Ok(Self::ForExpression(::std::boxed::Box::new(
                ::treesitter_types::runtime::maybe_grow_stack(|| {
                    <ForExpression as ::treesitter_types::FromNode>::from_node(node, src)
                })?,
            ))),
            "gen_block" => Ok(Self::GenBlock(::std::boxed::Box::new(
                ::treesitter_types::runtime::maybe_grow_stack(|| {
                    <GenBlock as ::treesitter_types::FromNode>::from_node(node, src)
                })?,
            ))),
            "generic_function" => Ok(Self::GenericFunction(::std::boxed::Box::new(
                ::treesitter_types::runtime::maybe_grow_stack(|| {
                    <GenericFunction as ::treesitter_types::FromNode>::from_node(node, src)
                })?,
            ))),
            "identifier" => Ok(Self::Identifier(::std::boxed::Box::new(
                ::treesitter_types::runtime::maybe_grow_stack(|| {
                    <Identifier as ::treesitter_types::FromNode>::from_node(node, src)
                })?,
            ))),
            "if_expression" => Ok(Self::IfExpression(::std::boxed::Box::new(
                ::treesitter_types::runtime::maybe_grow_stack(|| {
                    <IfExpression as ::treesitter_types::FromNode>::from_node(node, src)
                })?,
            ))),
            "index_expression" => Ok(Self::IndexExpression(::std::boxed::Box::new(
                ::treesitter_types::runtime::maybe_grow_stack(|| {
                    <IndexExpression as ::treesitter_types::FromNode>::from_node(node, src)
                })?,
            ))),
            "loop_expression" => Ok(Self::LoopExpression(::std::boxed::Box::new(
                ::treesitter_types::runtime::maybe_grow_stack(|| {
                    <LoopExpression as ::treesitter_types::FromNode>::from_node(node, src)
                })?,
            ))),
            "macro_invocation" => Ok(Self::MacroInvocation(::std::boxed::Box::new(
                ::treesitter_types::runtime::maybe_grow_stack(|| {
                    <MacroInvocation as ::treesitter_types::FromNode>::from_node(node, src)
                })?,
            ))),
            "match_expression" => Ok(Self::MatchExpression(::std::boxed::Box::new(
                ::treesitter_types::runtime::maybe_grow_stack(|| {
                    <MatchExpression as ::treesitter_types::FromNode>::from_node(node, src)
                })?,
            ))),
            "metavariable" => Ok(Self::Metavariable(::std::boxed::Box::new(
                ::treesitter_types::runtime::maybe_grow_stack(|| {
                    <Metavariable as ::treesitter_types::FromNode>::from_node(node, src)
                })?,
            ))),
            "parenthesized_expression" => Ok(Self::ParenthesizedExpression(
                ::std::boxed::Box::new(::treesitter_types::runtime::maybe_grow_stack(|| {
                    <ParenthesizedExpression as ::treesitter_types::FromNode>::from_node(node, src)
                })?),
            )),
            "reference_expression" => Ok(Self::ReferenceExpression(::std::boxed::Box::new(
                ::treesitter_types::runtime::maybe_grow_stack(|| {
                    <ReferenceExpression as ::treesitter_types::FromNode>::from_node(node, src)
                })?,
            ))),
            "return_expression" => Ok(Self::ReturnExpression(::std::boxed::Box::new(
                ::treesitter_types::runtime::maybe_grow_stack(|| {
                    <ReturnExpression as ::treesitter_types::FromNode>::from_node(node, src)
                })?,
            ))),
            "scoped_identifier" => Ok(Self::ScopedIdentifier(::std::boxed::Box::new(
                ::treesitter_types::runtime::maybe_grow_stack(|| {
                    <ScopedIdentifier as ::treesitter_types::FromNode>::from_node(node, src)
                })?,
            ))),
            "self" => Ok(Self::SelfType(::std::boxed::Box::new(
                ::treesitter_types::runtime::maybe_grow_stack(|| {
                    <SelfType as ::treesitter_types::FromNode>::from_node(node, src)
                })?,
            ))),
            "struct_expression" => Ok(Self::StructExpression(::std::boxed::Box::new(
                ::treesitter_types::runtime::maybe_grow_stack(|| {
                    <StructExpression as ::treesitter_types::FromNode>::from_node(node, src)
                })?,
            ))),
            "try_block" => Ok(Self::TryBlock(::std::boxed::Box::new(
                ::treesitter_types::runtime::maybe_grow_stack(|| {
                    <TryBlock as ::treesitter_types::FromNode>::from_node(node, src)
                })?,
            ))),
            "try_expression" => Ok(Self::TryExpression(::std::boxed::Box::new(
                ::treesitter_types::runtime::maybe_grow_stack(|| {
                    <TryExpression as ::treesitter_types::FromNode>::from_node(node, src)
                })?,
            ))),
            "tuple_expression" => Ok(Self::TupleExpression(::std::boxed::Box::new(
                ::treesitter_types::runtime::maybe_grow_stack(|| {
                    <TupleExpression as ::treesitter_types::FromNode>::from_node(node, src)
                })?,
            ))),
            "type_cast_expression" => Ok(Self::TypeCastExpression(::std::boxed::Box::new(
                ::treesitter_types::runtime::maybe_grow_stack(|| {
                    <TypeCastExpression as ::treesitter_types::FromNode>::from_node(node, src)
                })?,
            ))),
            "unary_expression" => Ok(Self::UnaryExpression(::std::boxed::Box::new(
                ::treesitter_types::runtime::maybe_grow_stack(|| {
                    <UnaryExpression as ::treesitter_types::FromNode>::from_node(node, src)
                })?,
            ))),
            "unit_expression" => Ok(Self::UnitExpression(::std::boxed::Box::new(
                ::treesitter_types::runtime::maybe_grow_stack(|| {
                    <UnitExpression as ::treesitter_types::FromNode>::from_node(node, src)
                })?,
            ))),
            "unsafe_block" => Ok(Self::UnsafeBlock(::std::boxed::Box::new(
                ::treesitter_types::runtime::maybe_grow_stack(|| {
                    <UnsafeBlock as ::treesitter_types::FromNode>::from_node(node, src)
                })?,
            ))),
            "while_expression" => Ok(Self::WhileExpression(::std::boxed::Box::new(
                ::treesitter_types::runtime::maybe_grow_stack(|| {
                    <WhileExpression as ::treesitter_types::FromNode>::from_node(node, src)
                })?,
            ))),
            "yield_expression" => Ok(Self::YieldExpression(::std::boxed::Box::new(
                ::treesitter_types::runtime::maybe_grow_stack(|| {
                    <YieldExpression as ::treesitter_types::FromNode>::from_node(node, src)
                })?,
            ))),
            _other => {
                if let Ok(v) = ::treesitter_types::runtime::maybe_grow_stack(|| {
                    <Literal as ::treesitter_types::FromNode>::from_node(node, src)
                }) {
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
impl ::treesitter_types::Spanned for CallExpressionFunction<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        match self {
            Self::Literal(inner) => inner.span(),
            Self::ArrayExpression(inner) => inner.span(),
            Self::AssignmentExpression(inner) => inner.span(),
            Self::AsyncBlock(inner) => inner.span(),
            Self::AwaitExpression(inner) => inner.span(),
            Self::BinaryExpression(inner) => inner.span(),
            Self::Block(inner) => inner.span(),
            Self::BreakExpression(inner) => inner.span(),
            Self::CallExpression(inner) => inner.span(),
            Self::ClosureExpression(inner) => inner.span(),
            Self::CompoundAssignmentExpr(inner) => inner.span(),
            Self::ConstBlock(inner) => inner.span(),
            Self::ContinueExpression(inner) => inner.span(),
            Self::FieldExpression(inner) => inner.span(),
            Self::ForExpression(inner) => inner.span(),
            Self::GenBlock(inner) => inner.span(),
            Self::GenericFunction(inner) => inner.span(),
            Self::Identifier(inner) => inner.span(),
            Self::IfExpression(inner) => inner.span(),
            Self::IndexExpression(inner) => inner.span(),
            Self::LoopExpression(inner) => inner.span(),
            Self::MacroInvocation(inner) => inner.span(),
            Self::MatchExpression(inner) => inner.span(),
            Self::Metavariable(inner) => inner.span(),
            Self::ParenthesizedExpression(inner) => inner.span(),
            Self::ReferenceExpression(inner) => inner.span(),
            Self::ReturnExpression(inner) => inner.span(),
            Self::ScopedIdentifier(inner) => inner.span(),
            Self::SelfType(inner) => inner.span(),
            Self::StructExpression(inner) => inner.span(),
            Self::TryBlock(inner) => inner.span(),
            Self::TryExpression(inner) => inner.span(),
            Self::TupleExpression(inner) => inner.span(),
            Self::TypeCastExpression(inner) => inner.span(),
            Self::UnaryExpression(inner) => inner.span(),
            Self::UnitExpression(inner) => inner.span(),
            Self::UnsafeBlock(inner) => inner.span(),
            Self::WhileExpression(inner) => inner.span(),
            Self::YieldExpression(inner) => inner.span(),
        }
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ClosureExpressionBody<'tree> {
    Blank(::treesitter_types::Span),
    Expression(::std::boxed::Box<Expression<'tree>>),
}
impl<'tree> ::treesitter_types::FromNode<'tree> for ClosureExpressionBody<'tree> {
    #[allow(clippy::collapsible_else_if)]
    fn from_node(
        node: ::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        match node.kind() {
            "_" => Ok(Self::Blank(::treesitter_types::Span::from(node))),
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
impl ::treesitter_types::Spanned for ClosureExpressionBody<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        match self {
            Self::Blank(span) => *span,
            Self::Expression(inner) => inner.span(),
        }
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ClosureParametersChildren<'tree> {
    Pattern(::std::boxed::Box<Pattern<'tree>>),
    Parameter(::std::boxed::Box<Parameter<'tree>>),
}
impl<'tree> ::treesitter_types::FromNode<'tree> for ClosureParametersChildren<'tree> {
    #[allow(clippy::collapsible_else_if)]
    fn from_node(
        node: ::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        match node.kind() {
            "parameter" => Ok(Self::Parameter(::std::boxed::Box::new(
                ::treesitter_types::runtime::maybe_grow_stack(|| {
                    <Parameter as ::treesitter_types::FromNode>::from_node(node, src)
                })?,
            ))),
            _other => {
                if let Ok(v) = ::treesitter_types::runtime::maybe_grow_stack(|| {
                    <Pattern as ::treesitter_types::FromNode>::from_node(node, src)
                }) {
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
impl ::treesitter_types::Spanned for ClosureParametersChildren<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        match self {
            Self::Pattern(inner) => inner.span(),
            Self::Parameter(inner) => inner.span(),
        }
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum CompoundAssignmentExprOperator {
    PercentEq(::treesitter_types::Span),
    AmpEq(::treesitter_types::Span),
    StarEq(::treesitter_types::Span),
    PlusEq(::treesitter_types::Span),
    MinusEq(::treesitter_types::Span),
    SlashEq(::treesitter_types::Span),
    ShlEq(::treesitter_types::Span),
    ShrEq(::treesitter_types::Span),
    CaretEq(::treesitter_types::Span),
    PipeEq(::treesitter_types::Span),
}
impl<'tree> ::treesitter_types::FromNode<'tree> for CompoundAssignmentExprOperator {
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
            ">>=" => Ok(Self::ShrEq(::treesitter_types::Span::from(node))),
            "^=" => Ok(Self::CaretEq(::treesitter_types::Span::from(node))),
            "|=" => Ok(Self::PipeEq(::treesitter_types::Span::from(node))),
            other => Err(::treesitter_types::ParseError::unexpected_kind(other, node)),
        }
    }
}
impl ::treesitter_types::Spanned for CompoundAssignmentExprOperator {
    fn span(&self) -> ::treesitter_types::Span {
        match self {
            Self::PercentEq(span) => *span,
            Self::AmpEq(span) => *span,
            Self::StarEq(span) => *span,
            Self::PlusEq(span) => *span,
            Self::MinusEq(span) => *span,
            Self::SlashEq(span) => *span,
            Self::ShlEq(span) => *span,
            Self::ShrEq(span) => *span,
            Self::CaretEq(span) => *span,
            Self::PipeEq(span) => *span,
        }
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ConstParameterValue<'tree> {
    Literal(::std::boxed::Box<Literal<'tree>>),
    Block(::std::boxed::Box<Block<'tree>>),
    Identifier(::std::boxed::Box<Identifier<'tree>>),
    NegativeLiteral(::std::boxed::Box<NegativeLiteral<'tree>>),
}
impl<'tree> ::treesitter_types::FromNode<'tree> for ConstParameterValue<'tree> {
    #[allow(clippy::collapsible_else_if)]
    fn from_node(
        node: ::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        match node.kind() {
            "block" => Ok(Self::Block(::std::boxed::Box::new(
                ::treesitter_types::runtime::maybe_grow_stack(|| {
                    <Block as ::treesitter_types::FromNode>::from_node(node, src)
                })?,
            ))),
            "identifier" => Ok(Self::Identifier(::std::boxed::Box::new(
                ::treesitter_types::runtime::maybe_grow_stack(|| {
                    <Identifier as ::treesitter_types::FromNode>::from_node(node, src)
                })?,
            ))),
            "negative_literal" => Ok(Self::NegativeLiteral(::std::boxed::Box::new(
                ::treesitter_types::runtime::maybe_grow_stack(|| {
                    <NegativeLiteral as ::treesitter_types::FromNode>::from_node(node, src)
                })?,
            ))),
            _other => {
                if let Ok(v) = ::treesitter_types::runtime::maybe_grow_stack(|| {
                    <Literal as ::treesitter_types::FromNode>::from_node(node, src)
                }) {
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
impl ::treesitter_types::Spanned for ConstParameterValue<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        match self {
            Self::Literal(inner) => inner.span(),
            Self::Block(inner) => inner.span(),
            Self::Identifier(inner) => inner.span(),
            Self::NegativeLiteral(inner) => inner.span(),
        }
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum DynamicTypeTrait<'tree> {
    FunctionType(::std::boxed::Box<FunctionType<'tree>>),
    GenericType(::std::boxed::Box<GenericType<'tree>>),
    HigherRankedTraitBound(::std::boxed::Box<HigherRankedTraitBound<'tree>>),
    ScopedTypeIdentifier(::std::boxed::Box<ScopedTypeIdentifier<'tree>>),
    TupleType(::std::boxed::Box<TupleType<'tree>>),
    TypeIdentifier(::std::boxed::Box<TypeIdentifier<'tree>>),
}
impl<'tree> ::treesitter_types::FromNode<'tree> for DynamicTypeTrait<'tree> {
    #[allow(clippy::collapsible_else_if)]
    fn from_node(
        node: ::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        match node.kind() {
            "function_type" => Ok(Self::FunctionType(::std::boxed::Box::new(
                ::treesitter_types::runtime::maybe_grow_stack(|| {
                    <FunctionType as ::treesitter_types::FromNode>::from_node(node, src)
                })?,
            ))),
            "generic_type" => Ok(Self::GenericType(::std::boxed::Box::new(
                ::treesitter_types::runtime::maybe_grow_stack(|| {
                    <GenericType as ::treesitter_types::FromNode>::from_node(node, src)
                })?,
            ))),
            "higher_ranked_trait_bound" => Ok(Self::HigherRankedTraitBound(
                ::std::boxed::Box::new(::treesitter_types::runtime::maybe_grow_stack(|| {
                    <HigherRankedTraitBound as ::treesitter_types::FromNode>::from_node(node, src)
                })?),
            )),
            "scoped_type_identifier" => Ok(Self::ScopedTypeIdentifier(::std::boxed::Box::new(
                ::treesitter_types::runtime::maybe_grow_stack(|| {
                    <ScopedTypeIdentifier as ::treesitter_types::FromNode>::from_node(node, src)
                })?,
            ))),
            "tuple_type" => Ok(Self::TupleType(::std::boxed::Box::new(
                ::treesitter_types::runtime::maybe_grow_stack(|| {
                    <TupleType as ::treesitter_types::FromNode>::from_node(node, src)
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
impl ::treesitter_types::Spanned for DynamicTypeTrait<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        match self {
            Self::FunctionType(inner) => inner.span(),
            Self::GenericType(inner) => inner.span(),
            Self::HigherRankedTraitBound(inner) => inner.span(),
            Self::ScopedTypeIdentifier(inner) => inner.span(),
            Self::TupleType(inner) => inner.span(),
            Self::TypeIdentifier(inner) => inner.span(),
        }
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ElseClauseChildren<'tree> {
    Block(::std::boxed::Box<Block<'tree>>),
    IfExpression(::std::boxed::Box<IfExpression<'tree>>),
}
impl<'tree> ::treesitter_types::FromNode<'tree> for ElseClauseChildren<'tree> {
    #[allow(clippy::collapsible_else_if)]
    fn from_node(
        node: ::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        match node.kind() {
            "block" => Ok(Self::Block(::std::boxed::Box::new(
                ::treesitter_types::runtime::maybe_grow_stack(|| {
                    <Block as ::treesitter_types::FromNode>::from_node(node, src)
                })?,
            ))),
            "if_expression" => Ok(Self::IfExpression(::std::boxed::Box::new(
                ::treesitter_types::runtime::maybe_grow_stack(|| {
                    <IfExpression as ::treesitter_types::FromNode>::from_node(node, src)
                })?,
            ))),
            other => Err(::treesitter_types::ParseError::unexpected_kind(other, node)),
        }
    }
}
impl ::treesitter_types::Spanned for ElseClauseChildren<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        match self {
            Self::Block(inner) => inner.span(),
            Self::IfExpression(inner) => inner.span(),
        }
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum EnumItemChildren<'tree> {
    VisibilityModifier(::std::boxed::Box<VisibilityModifier<'tree>>),
    WhereClause(::std::boxed::Box<WhereClause<'tree>>),
}
impl<'tree> ::treesitter_types::FromNode<'tree> for EnumItemChildren<'tree> {
    #[allow(clippy::collapsible_else_if)]
    fn from_node(
        node: ::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        match node.kind() {
            "visibility_modifier" => Ok(Self::VisibilityModifier(::std::boxed::Box::new(
                ::treesitter_types::runtime::maybe_grow_stack(|| {
                    <VisibilityModifier as ::treesitter_types::FromNode>::from_node(node, src)
                })?,
            ))),
            "where_clause" => Ok(Self::WhereClause(::std::boxed::Box::new(
                ::treesitter_types::runtime::maybe_grow_stack(|| {
                    <WhereClause as ::treesitter_types::FromNode>::from_node(node, src)
                })?,
            ))),
            other => Err(::treesitter_types::ParseError::unexpected_kind(other, node)),
        }
    }
}
impl ::treesitter_types::Spanned for EnumItemChildren<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        match self {
            Self::VisibilityModifier(inner) => inner.span(),
            Self::WhereClause(inner) => inner.span(),
        }
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum EnumVariantBody<'tree> {
    FieldDeclarationList(::std::boxed::Box<FieldDeclarationList<'tree>>),
    OrderedFieldDeclarationList(::std::boxed::Box<OrderedFieldDeclarationList<'tree>>),
}
impl<'tree> ::treesitter_types::FromNode<'tree> for EnumVariantBody<'tree> {
    #[allow(clippy::collapsible_else_if)]
    fn from_node(
        node: ::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        match node.kind() {
            "field_declaration_list" => Ok(Self::FieldDeclarationList(::std::boxed::Box::new(
                ::treesitter_types::runtime::maybe_grow_stack(|| {
                    <FieldDeclarationList as ::treesitter_types::FromNode>::from_node(node, src)
                })?,
            ))),
            "ordered_field_declaration_list" => Ok(Self::OrderedFieldDeclarationList(
                ::std::boxed::Box::new(::treesitter_types::runtime::maybe_grow_stack(|| {
                    <OrderedFieldDeclarationList as ::treesitter_types::FromNode>::from_node(
                        node, src,
                    )
                })?),
            )),
            other => Err(::treesitter_types::ParseError::unexpected_kind(other, node)),
        }
    }
}
impl ::treesitter_types::Spanned for EnumVariantBody<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        match self {
            Self::FieldDeclarationList(inner) => inner.span(),
            Self::OrderedFieldDeclarationList(inner) => inner.span(),
        }
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum EnumVariantListChildren<'tree> {
    AttributeItem(::std::boxed::Box<AttributeItem<'tree>>),
    EnumVariant(::std::boxed::Box<EnumVariant<'tree>>),
}
impl<'tree> ::treesitter_types::FromNode<'tree> for EnumVariantListChildren<'tree> {
    #[allow(clippy::collapsible_else_if)]
    fn from_node(
        node: ::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        match node.kind() {
            "attribute_item" => Ok(Self::AttributeItem(::std::boxed::Box::new(
                ::treesitter_types::runtime::maybe_grow_stack(|| {
                    <AttributeItem as ::treesitter_types::FromNode>::from_node(node, src)
                })?,
            ))),
            "enum_variant" => Ok(Self::EnumVariant(::std::boxed::Box::new(
                ::treesitter_types::runtime::maybe_grow_stack(|| {
                    <EnumVariant as ::treesitter_types::FromNode>::from_node(node, src)
                })?,
            ))),
            other => Err(::treesitter_types::ParseError::unexpected_kind(other, node)),
        }
    }
}
impl ::treesitter_types::Spanned for EnumVariantListChildren<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        match self {
            Self::AttributeItem(inner) => inner.span(),
            Self::EnumVariant(inner) => inner.span(),
        }
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ExternCrateDeclarationChildren<'tree> {
    Crate(::std::boxed::Box<Crate<'tree>>),
    VisibilityModifier(::std::boxed::Box<VisibilityModifier<'tree>>),
}
impl<'tree> ::treesitter_types::FromNode<'tree> for ExternCrateDeclarationChildren<'tree> {
    #[allow(clippy::collapsible_else_if)]
    fn from_node(
        node: ::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        match node.kind() {
            "crate" => Ok(Self::Crate(::std::boxed::Box::new(
                ::treesitter_types::runtime::maybe_grow_stack(|| {
                    <Crate as ::treesitter_types::FromNode>::from_node(node, src)
                })?,
            ))),
            "visibility_modifier" => Ok(Self::VisibilityModifier(::std::boxed::Box::new(
                ::treesitter_types::runtime::maybe_grow_stack(|| {
                    <VisibilityModifier as ::treesitter_types::FromNode>::from_node(node, src)
                })?,
            ))),
            other => Err(::treesitter_types::ParseError::unexpected_kind(other, node)),
        }
    }
}
impl ::treesitter_types::Spanned for ExternCrateDeclarationChildren<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        match self {
            Self::Crate(inner) => inner.span(),
            Self::VisibilityModifier(inner) => inner.span(),
        }
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum FieldDeclarationListChildren<'tree> {
    AttributeItem(::std::boxed::Box<AttributeItem<'tree>>),
    FieldDeclaration(::std::boxed::Box<FieldDeclaration<'tree>>),
}
impl<'tree> ::treesitter_types::FromNode<'tree> for FieldDeclarationListChildren<'tree> {
    #[allow(clippy::collapsible_else_if)]
    fn from_node(
        node: ::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        match node.kind() {
            "attribute_item" => Ok(Self::AttributeItem(::std::boxed::Box::new(
                ::treesitter_types::runtime::maybe_grow_stack(|| {
                    <AttributeItem as ::treesitter_types::FromNode>::from_node(node, src)
                })?,
            ))),
            "field_declaration" => Ok(Self::FieldDeclaration(::std::boxed::Box::new(
                ::treesitter_types::runtime::maybe_grow_stack(|| {
                    <FieldDeclaration as ::treesitter_types::FromNode>::from_node(node, src)
                })?,
            ))),
            other => Err(::treesitter_types::ParseError::unexpected_kind(other, node)),
        }
    }
}
impl ::treesitter_types::Spanned for FieldDeclarationListChildren<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        match self {
            Self::AttributeItem(inner) => inner.span(),
            Self::FieldDeclaration(inner) => inner.span(),
        }
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum FieldExpressionField<'tree> {
    FieldIdentifier(::std::boxed::Box<FieldIdentifier<'tree>>),
    IntegerLiteral(::std::boxed::Box<IntegerLiteral<'tree>>),
}
impl<'tree> ::treesitter_types::FromNode<'tree> for FieldExpressionField<'tree> {
    #[allow(clippy::collapsible_else_if)]
    fn from_node(
        node: ::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        match node.kind() {
            "field_identifier" => Ok(Self::FieldIdentifier(::std::boxed::Box::new(
                ::treesitter_types::runtime::maybe_grow_stack(|| {
                    <FieldIdentifier as ::treesitter_types::FromNode>::from_node(node, src)
                })?,
            ))),
            "integer_literal" => Ok(Self::IntegerLiteral(::std::boxed::Box::new(
                ::treesitter_types::runtime::maybe_grow_stack(|| {
                    <IntegerLiteral as ::treesitter_types::FromNode>::from_node(node, src)
                })?,
            ))),
            other => Err(::treesitter_types::ParseError::unexpected_kind(other, node)),
        }
    }
}
impl ::treesitter_types::Spanned for FieldExpressionField<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        match self {
            Self::FieldIdentifier(inner) => inner.span(),
            Self::IntegerLiteral(inner) => inner.span(),
        }
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum FieldInitializerField<'tree> {
    FieldIdentifier(::std::boxed::Box<FieldIdentifier<'tree>>),
    IntegerLiteral(::std::boxed::Box<IntegerLiteral<'tree>>),
}
impl<'tree> ::treesitter_types::FromNode<'tree> for FieldInitializerField<'tree> {
    #[allow(clippy::collapsible_else_if)]
    fn from_node(
        node: ::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        match node.kind() {
            "field_identifier" => Ok(Self::FieldIdentifier(::std::boxed::Box::new(
                ::treesitter_types::runtime::maybe_grow_stack(|| {
                    <FieldIdentifier as ::treesitter_types::FromNode>::from_node(node, src)
                })?,
            ))),
            "integer_literal" => Ok(Self::IntegerLiteral(::std::boxed::Box::new(
                ::treesitter_types::runtime::maybe_grow_stack(|| {
                    <IntegerLiteral as ::treesitter_types::FromNode>::from_node(node, src)
                })?,
            ))),
            other => Err(::treesitter_types::ParseError::unexpected_kind(other, node)),
        }
    }
}
impl ::treesitter_types::Spanned for FieldInitializerField<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        match self {
            Self::FieldIdentifier(inner) => inner.span(),
            Self::IntegerLiteral(inner) => inner.span(),
        }
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum FieldInitializerListChildren<'tree> {
    BaseFieldInitializer(::std::boxed::Box<BaseFieldInitializer<'tree>>),
    FieldInitializer(::std::boxed::Box<FieldInitializer<'tree>>),
    ShorthandFieldInitializer(::std::boxed::Box<ShorthandFieldInitializer<'tree>>),
}
impl<'tree> ::treesitter_types::FromNode<'tree> for FieldInitializerListChildren<'tree> {
    #[allow(clippy::collapsible_else_if)]
    fn from_node(
        node: ::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        match node.kind() {
            "base_field_initializer" => Ok(Self::BaseFieldInitializer(::std::boxed::Box::new(
                ::treesitter_types::runtime::maybe_grow_stack(|| {
                    <BaseFieldInitializer as ::treesitter_types::FromNode>::from_node(node, src)
                })?,
            ))),
            "field_initializer" => Ok(Self::FieldInitializer(::std::boxed::Box::new(
                ::treesitter_types::runtime::maybe_grow_stack(|| {
                    <FieldInitializer as ::treesitter_types::FromNode>::from_node(node, src)
                })?,
            ))),
            "shorthand_field_initializer" => Ok(Self::ShorthandFieldInitializer(
                ::std::boxed::Box::new(::treesitter_types::runtime::maybe_grow_stack(|| {
                    <ShorthandFieldInitializer as ::treesitter_types::FromNode>::from_node(
                        node, src,
                    )
                })?),
            )),
            other => Err(::treesitter_types::ParseError::unexpected_kind(other, node)),
        }
    }
}
impl ::treesitter_types::Spanned for FieldInitializerListChildren<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        match self {
            Self::BaseFieldInitializer(inner) => inner.span(),
            Self::FieldInitializer(inner) => inner.span(),
            Self::ShorthandFieldInitializer(inner) => inner.span(),
        }
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum FieldPatternName<'tree> {
    FieldIdentifier(::std::boxed::Box<FieldIdentifier<'tree>>),
    ShorthandFieldIdentifier(::std::boxed::Box<ShorthandFieldIdentifier<'tree>>),
}
impl<'tree> ::treesitter_types::FromNode<'tree> for FieldPatternName<'tree> {
    #[allow(clippy::collapsible_else_if)]
    fn from_node(
        node: ::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        match node.kind() {
            "field_identifier" => Ok(Self::FieldIdentifier(::std::boxed::Box::new(
                ::treesitter_types::runtime::maybe_grow_stack(|| {
                    <FieldIdentifier as ::treesitter_types::FromNode>::from_node(node, src)
                })?,
            ))),
            "shorthand_field_identifier" => Ok(Self::ShorthandFieldIdentifier(
                ::std::boxed::Box::new(::treesitter_types::runtime::maybe_grow_stack(|| {
                    <ShorthandFieldIdentifier as ::treesitter_types::FromNode>::from_node(node, src)
                })?),
            )),
            other => Err(::treesitter_types::ParseError::unexpected_kind(other, node)),
        }
    }
}
impl ::treesitter_types::Spanned for FieldPatternName<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        match self {
            Self::FieldIdentifier(inner) => inner.span(),
            Self::ShorthandFieldIdentifier(inner) => inner.span(),
        }
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum FunctionItemName<'tree> {
    Identifier(::std::boxed::Box<Identifier<'tree>>),
    Metavariable(::std::boxed::Box<Metavariable<'tree>>),
}
impl<'tree> ::treesitter_types::FromNode<'tree> for FunctionItemName<'tree> {
    #[allow(clippy::collapsible_else_if)]
    fn from_node(
        node: ::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        match node.kind() {
            "identifier" => Ok(Self::Identifier(::std::boxed::Box::new(
                ::treesitter_types::runtime::maybe_grow_stack(|| {
                    <Identifier as ::treesitter_types::FromNode>::from_node(node, src)
                })?,
            ))),
            "metavariable" => Ok(Self::Metavariable(::std::boxed::Box::new(
                ::treesitter_types::runtime::maybe_grow_stack(|| {
                    <Metavariable as ::treesitter_types::FromNode>::from_node(node, src)
                })?,
            ))),
            other => Err(::treesitter_types::ParseError::unexpected_kind(other, node)),
        }
    }
}
impl ::treesitter_types::Spanned for FunctionItemName<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        match self {
            Self::Identifier(inner) => inner.span(),
            Self::Metavariable(inner) => inner.span(),
        }
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum FunctionItemChildren<'tree> {
    FunctionModifiers(::std::boxed::Box<FunctionModifiers<'tree>>),
    VisibilityModifier(::std::boxed::Box<VisibilityModifier<'tree>>),
    WhereClause(::std::boxed::Box<WhereClause<'tree>>),
}
impl<'tree> ::treesitter_types::FromNode<'tree> for FunctionItemChildren<'tree> {
    #[allow(clippy::collapsible_else_if)]
    fn from_node(
        node: ::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        match node.kind() {
            "function_modifiers" => Ok(Self::FunctionModifiers(::std::boxed::Box::new(
                ::treesitter_types::runtime::maybe_grow_stack(|| {
                    <FunctionModifiers as ::treesitter_types::FromNode>::from_node(node, src)
                })?,
            ))),
            "visibility_modifier" => Ok(Self::VisibilityModifier(::std::boxed::Box::new(
                ::treesitter_types::runtime::maybe_grow_stack(|| {
                    <VisibilityModifier as ::treesitter_types::FromNode>::from_node(node, src)
                })?,
            ))),
            "where_clause" => Ok(Self::WhereClause(::std::boxed::Box::new(
                ::treesitter_types::runtime::maybe_grow_stack(|| {
                    <WhereClause as ::treesitter_types::FromNode>::from_node(node, src)
                })?,
            ))),
            other => Err(::treesitter_types::ParseError::unexpected_kind(other, node)),
        }
    }
}
impl ::treesitter_types::Spanned for FunctionItemChildren<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        match self {
            Self::FunctionModifiers(inner) => inner.span(),
            Self::VisibilityModifier(inner) => inner.span(),
            Self::WhereClause(inner) => inner.span(),
        }
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum FunctionSignatureItemName<'tree> {
    Identifier(::std::boxed::Box<Identifier<'tree>>),
    Metavariable(::std::boxed::Box<Metavariable<'tree>>),
}
impl<'tree> ::treesitter_types::FromNode<'tree> for FunctionSignatureItemName<'tree> {
    #[allow(clippy::collapsible_else_if)]
    fn from_node(
        node: ::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        match node.kind() {
            "identifier" => Ok(Self::Identifier(::std::boxed::Box::new(
                ::treesitter_types::runtime::maybe_grow_stack(|| {
                    <Identifier as ::treesitter_types::FromNode>::from_node(node, src)
                })?,
            ))),
            "metavariable" => Ok(Self::Metavariable(::std::boxed::Box::new(
                ::treesitter_types::runtime::maybe_grow_stack(|| {
                    <Metavariable as ::treesitter_types::FromNode>::from_node(node, src)
                })?,
            ))),
            other => Err(::treesitter_types::ParseError::unexpected_kind(other, node)),
        }
    }
}
impl ::treesitter_types::Spanned for FunctionSignatureItemName<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        match self {
            Self::Identifier(inner) => inner.span(),
            Self::Metavariable(inner) => inner.span(),
        }
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum FunctionSignatureItemChildren<'tree> {
    FunctionModifiers(::std::boxed::Box<FunctionModifiers<'tree>>),
    VisibilityModifier(::std::boxed::Box<VisibilityModifier<'tree>>),
    WhereClause(::std::boxed::Box<WhereClause<'tree>>),
}
impl<'tree> ::treesitter_types::FromNode<'tree> for FunctionSignatureItemChildren<'tree> {
    #[allow(clippy::collapsible_else_if)]
    fn from_node(
        node: ::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        match node.kind() {
            "function_modifiers" => Ok(Self::FunctionModifiers(::std::boxed::Box::new(
                ::treesitter_types::runtime::maybe_grow_stack(|| {
                    <FunctionModifiers as ::treesitter_types::FromNode>::from_node(node, src)
                })?,
            ))),
            "visibility_modifier" => Ok(Self::VisibilityModifier(::std::boxed::Box::new(
                ::treesitter_types::runtime::maybe_grow_stack(|| {
                    <VisibilityModifier as ::treesitter_types::FromNode>::from_node(node, src)
                })?,
            ))),
            "where_clause" => Ok(Self::WhereClause(::std::boxed::Box::new(
                ::treesitter_types::runtime::maybe_grow_stack(|| {
                    <WhereClause as ::treesitter_types::FromNode>::from_node(node, src)
                })?,
            ))),
            other => Err(::treesitter_types::ParseError::unexpected_kind(other, node)),
        }
    }
}
impl ::treesitter_types::Spanned for FunctionSignatureItemChildren<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        match self {
            Self::FunctionModifiers(inner) => inner.span(),
            Self::VisibilityModifier(inner) => inner.span(),
            Self::WhereClause(inner) => inner.span(),
        }
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum FunctionTypeTrait<'tree> {
    ScopedTypeIdentifier(::std::boxed::Box<ScopedTypeIdentifier<'tree>>),
    TypeIdentifier(::std::boxed::Box<TypeIdentifier<'tree>>),
}
impl<'tree> ::treesitter_types::FromNode<'tree> for FunctionTypeTrait<'tree> {
    #[allow(clippy::collapsible_else_if)]
    fn from_node(
        node: ::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        match node.kind() {
            "scoped_type_identifier" => Ok(Self::ScopedTypeIdentifier(::std::boxed::Box::new(
                ::treesitter_types::runtime::maybe_grow_stack(|| {
                    <ScopedTypeIdentifier as ::treesitter_types::FromNode>::from_node(node, src)
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
impl ::treesitter_types::Spanned for FunctionTypeTrait<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        match self {
            Self::ScopedTypeIdentifier(inner) => inner.span(),
            Self::TypeIdentifier(inner) => inner.span(),
        }
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum FunctionTypeChildren<'tree> {
    ForLifetimes(::std::boxed::Box<ForLifetimes<'tree>>),
    FunctionModifiers(::std::boxed::Box<FunctionModifiers<'tree>>),
}
impl<'tree> ::treesitter_types::FromNode<'tree> for FunctionTypeChildren<'tree> {
    #[allow(clippy::collapsible_else_if)]
    fn from_node(
        node: ::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        match node.kind() {
            "for_lifetimes" => Ok(Self::ForLifetimes(::std::boxed::Box::new(
                ::treesitter_types::runtime::maybe_grow_stack(|| {
                    <ForLifetimes as ::treesitter_types::FromNode>::from_node(node, src)
                })?,
            ))),
            "function_modifiers" => Ok(Self::FunctionModifiers(::std::boxed::Box::new(
                ::treesitter_types::runtime::maybe_grow_stack(|| {
                    <FunctionModifiers as ::treesitter_types::FromNode>::from_node(node, src)
                })?,
            ))),
            other => Err(::treesitter_types::ParseError::unexpected_kind(other, node)),
        }
    }
}
impl ::treesitter_types::Spanned for FunctionTypeChildren<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        match self {
            Self::ForLifetimes(inner) => inner.span(),
            Self::FunctionModifiers(inner) => inner.span(),
        }
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum GenericFunctionFunction<'tree> {
    FieldExpression(::std::boxed::Box<FieldExpression<'tree>>),
    Identifier(::std::boxed::Box<Identifier<'tree>>),
    ScopedIdentifier(::std::boxed::Box<ScopedIdentifier<'tree>>),
}
impl<'tree> ::treesitter_types::FromNode<'tree> for GenericFunctionFunction<'tree> {
    #[allow(clippy::collapsible_else_if)]
    fn from_node(
        node: ::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        match node.kind() {
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
            "scoped_identifier" => Ok(Self::ScopedIdentifier(::std::boxed::Box::new(
                ::treesitter_types::runtime::maybe_grow_stack(|| {
                    <ScopedIdentifier as ::treesitter_types::FromNode>::from_node(node, src)
                })?,
            ))),
            other => Err(::treesitter_types::ParseError::unexpected_kind(other, node)),
        }
    }
}
impl ::treesitter_types::Spanned for GenericFunctionFunction<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        match self {
            Self::FieldExpression(inner) => inner.span(),
            Self::Identifier(inner) => inner.span(),
            Self::ScopedIdentifier(inner) => inner.span(),
        }
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum GenericPatternChildren<'tree> {
    Identifier(::std::boxed::Box<Identifier<'tree>>),
    ScopedIdentifier(::std::boxed::Box<ScopedIdentifier<'tree>>),
}
impl<'tree> ::treesitter_types::FromNode<'tree> for GenericPatternChildren<'tree> {
    #[allow(clippy::collapsible_else_if)]
    fn from_node(
        node: ::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        match node.kind() {
            "identifier" => Ok(Self::Identifier(::std::boxed::Box::new(
                ::treesitter_types::runtime::maybe_grow_stack(|| {
                    <Identifier as ::treesitter_types::FromNode>::from_node(node, src)
                })?,
            ))),
            "scoped_identifier" => Ok(Self::ScopedIdentifier(::std::boxed::Box::new(
                ::treesitter_types::runtime::maybe_grow_stack(|| {
                    <ScopedIdentifier as ::treesitter_types::FromNode>::from_node(node, src)
                })?,
            ))),
            other => Err(::treesitter_types::ParseError::unexpected_kind(other, node)),
        }
    }
}
impl ::treesitter_types::Spanned for GenericPatternChildren<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        match self {
            Self::Identifier(inner) => inner.span(),
            Self::ScopedIdentifier(inner) => inner.span(),
        }
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum GenericTypeType<'tree> {
    Identifier(::std::boxed::Box<Identifier<'tree>>),
    ScopedIdentifier(::std::boxed::Box<ScopedIdentifier<'tree>>),
    ScopedTypeIdentifier(::std::boxed::Box<ScopedTypeIdentifier<'tree>>),
    TypeIdentifier(::std::boxed::Box<TypeIdentifier<'tree>>),
}
impl<'tree> ::treesitter_types::FromNode<'tree> for GenericTypeType<'tree> {
    #[allow(clippy::collapsible_else_if)]
    fn from_node(
        node: ::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        match node.kind() {
            "identifier" => Ok(Self::Identifier(::std::boxed::Box::new(
                ::treesitter_types::runtime::maybe_grow_stack(|| {
                    <Identifier as ::treesitter_types::FromNode>::from_node(node, src)
                })?,
            ))),
            "scoped_identifier" => Ok(Self::ScopedIdentifier(::std::boxed::Box::new(
                ::treesitter_types::runtime::maybe_grow_stack(|| {
                    <ScopedIdentifier as ::treesitter_types::FromNode>::from_node(node, src)
                })?,
            ))),
            "scoped_type_identifier" => Ok(Self::ScopedTypeIdentifier(::std::boxed::Box::new(
                ::treesitter_types::runtime::maybe_grow_stack(|| {
                    <ScopedTypeIdentifier as ::treesitter_types::FromNode>::from_node(node, src)
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
impl ::treesitter_types::Spanned for GenericTypeType<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        match self {
            Self::Identifier(inner) => inner.span(),
            Self::ScopedIdentifier(inner) => inner.span(),
            Self::ScopedTypeIdentifier(inner) => inner.span(),
            Self::TypeIdentifier(inner) => inner.span(),
        }
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum GenericTypeWithTurbofishType<'tree> {
    ScopedIdentifier(::std::boxed::Box<ScopedIdentifier<'tree>>),
    TypeIdentifier(::std::boxed::Box<TypeIdentifier<'tree>>),
}
impl<'tree> ::treesitter_types::FromNode<'tree> for GenericTypeWithTurbofishType<'tree> {
    #[allow(clippy::collapsible_else_if)]
    fn from_node(
        node: ::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        match node.kind() {
            "scoped_identifier" => Ok(Self::ScopedIdentifier(::std::boxed::Box::new(
                ::treesitter_types::runtime::maybe_grow_stack(|| {
                    <ScopedIdentifier as ::treesitter_types::FromNode>::from_node(node, src)
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
impl ::treesitter_types::Spanned for GenericTypeWithTurbofishType<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        match self {
            Self::ScopedIdentifier(inner) => inner.span(),
            Self::TypeIdentifier(inner) => inner.span(),
        }
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum IfExpressionCondition<'tree> {
    Expression(::std::boxed::Box<Expression<'tree>>),
    LetChain(::std::boxed::Box<LetChain<'tree>>),
    LetCondition(::std::boxed::Box<LetCondition<'tree>>),
}
impl<'tree> ::treesitter_types::FromNode<'tree> for IfExpressionCondition<'tree> {
    #[allow(clippy::collapsible_else_if)]
    fn from_node(
        node: ::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        match node.kind() {
            "let_chain" => Ok(Self::LetChain(::std::boxed::Box::new(
                ::treesitter_types::runtime::maybe_grow_stack(|| {
                    <LetChain as ::treesitter_types::FromNode>::from_node(node, src)
                })?,
            ))),
            "let_condition" => Ok(Self::LetCondition(::std::boxed::Box::new(
                ::treesitter_types::runtime::maybe_grow_stack(|| {
                    <LetCondition as ::treesitter_types::FromNode>::from_node(node, src)
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
impl ::treesitter_types::Spanned for IfExpressionCondition<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        match self {
            Self::Expression(inner) => inner.span(),
            Self::LetChain(inner) => inner.span(),
            Self::LetCondition(inner) => inner.span(),
        }
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ImplItemTrait<'tree> {
    GenericType(::std::boxed::Box<GenericType<'tree>>),
    ScopedTypeIdentifier(::std::boxed::Box<ScopedTypeIdentifier<'tree>>),
    TypeIdentifier(::std::boxed::Box<TypeIdentifier<'tree>>),
}
impl<'tree> ::treesitter_types::FromNode<'tree> for ImplItemTrait<'tree> {
    #[allow(clippy::collapsible_else_if)]
    fn from_node(
        node: ::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        match node.kind() {
            "generic_type" => Ok(Self::GenericType(::std::boxed::Box::new(
                ::treesitter_types::runtime::maybe_grow_stack(|| {
                    <GenericType as ::treesitter_types::FromNode>::from_node(node, src)
                })?,
            ))),
            "scoped_type_identifier" => Ok(Self::ScopedTypeIdentifier(::std::boxed::Box::new(
                ::treesitter_types::runtime::maybe_grow_stack(|| {
                    <ScopedTypeIdentifier as ::treesitter_types::FromNode>::from_node(node, src)
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
impl ::treesitter_types::Spanned for ImplItemTrait<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        match self {
            Self::GenericType(inner) => inner.span(),
            Self::ScopedTypeIdentifier(inner) => inner.span(),
            Self::TypeIdentifier(inner) => inner.span(),
        }
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum LetChainChildren<'tree> {
    Expression(::std::boxed::Box<Expression<'tree>>),
    LetCondition(::std::boxed::Box<LetCondition<'tree>>),
}
impl<'tree> ::treesitter_types::FromNode<'tree> for LetChainChildren<'tree> {
    #[allow(clippy::collapsible_else_if)]
    fn from_node(
        node: ::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        match node.kind() {
            "let_condition" => Ok(Self::LetCondition(::std::boxed::Box::new(
                ::treesitter_types::runtime::maybe_grow_stack(|| {
                    <LetCondition as ::treesitter_types::FromNode>::from_node(node, src)
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
impl ::treesitter_types::Spanned for LetChainChildren<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        match self {
            Self::Expression(inner) => inner.span(),
            Self::LetCondition(inner) => inner.span(),
        }
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum MacroInvocationMacro<'tree> {
    Identifier(::std::boxed::Box<Identifier<'tree>>),
    ScopedIdentifier(::std::boxed::Box<ScopedIdentifier<'tree>>),
}
impl<'tree> ::treesitter_types::FromNode<'tree> for MacroInvocationMacro<'tree> {
    #[allow(clippy::collapsible_else_if)]
    fn from_node(
        node: ::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        match node.kind() {
            "identifier" => Ok(Self::Identifier(::std::boxed::Box::new(
                ::treesitter_types::runtime::maybe_grow_stack(|| {
                    <Identifier as ::treesitter_types::FromNode>::from_node(node, src)
                })?,
            ))),
            "scoped_identifier" => Ok(Self::ScopedIdentifier(::std::boxed::Box::new(
                ::treesitter_types::runtime::maybe_grow_stack(|| {
                    <ScopedIdentifier as ::treesitter_types::FromNode>::from_node(node, src)
                })?,
            ))),
            other => Err(::treesitter_types::ParseError::unexpected_kind(other, node)),
        }
    }
}
impl ::treesitter_types::Spanned for MacroInvocationMacro<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        match self {
            Self::Identifier(inner) => inner.span(),
            Self::ScopedIdentifier(inner) => inner.span(),
        }
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum MatchArmChildren<'tree> {
    AttributeItem(::std::boxed::Box<AttributeItem<'tree>>),
    InnerAttributeItem(::std::boxed::Box<InnerAttributeItem<'tree>>),
}
impl<'tree> ::treesitter_types::FromNode<'tree> for MatchArmChildren<'tree> {
    #[allow(clippy::collapsible_else_if)]
    fn from_node(
        node: ::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        match node.kind() {
            "attribute_item" => Ok(Self::AttributeItem(::std::boxed::Box::new(
                ::treesitter_types::runtime::maybe_grow_stack(|| {
                    <AttributeItem as ::treesitter_types::FromNode>::from_node(node, src)
                })?,
            ))),
            "inner_attribute_item" => Ok(Self::InnerAttributeItem(::std::boxed::Box::new(
                ::treesitter_types::runtime::maybe_grow_stack(|| {
                    <InnerAttributeItem as ::treesitter_types::FromNode>::from_node(node, src)
                })?,
            ))),
            other => Err(::treesitter_types::ParseError::unexpected_kind(other, node)),
        }
    }
}
impl ::treesitter_types::Spanned for MatchArmChildren<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        match self {
            Self::AttributeItem(inner) => inner.span(),
            Self::InnerAttributeItem(inner) => inner.span(),
        }
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum MatchPatternCondition<'tree> {
    Expression(::std::boxed::Box<Expression<'tree>>),
    LetChain(::std::boxed::Box<LetChain<'tree>>),
    LetCondition(::std::boxed::Box<LetCondition<'tree>>),
}
impl<'tree> ::treesitter_types::FromNode<'tree> for MatchPatternCondition<'tree> {
    #[allow(clippy::collapsible_else_if)]
    fn from_node(
        node: ::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        match node.kind() {
            "let_chain" => Ok(Self::LetChain(::std::boxed::Box::new(
                ::treesitter_types::runtime::maybe_grow_stack(|| {
                    <LetChain as ::treesitter_types::FromNode>::from_node(node, src)
                })?,
            ))),
            "let_condition" => Ok(Self::LetCondition(::std::boxed::Box::new(
                ::treesitter_types::runtime::maybe_grow_stack(|| {
                    <LetCondition as ::treesitter_types::FromNode>::from_node(node, src)
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
impl ::treesitter_types::Spanned for MatchPatternCondition<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        match self {
            Self::Expression(inner) => inner.span(),
            Self::LetChain(inner) => inner.span(),
            Self::LetCondition(inner) => inner.span(),
        }
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum MutPatternChildren<'tree> {
    Pattern(::std::boxed::Box<Pattern<'tree>>),
    MutableSpecifier(::std::boxed::Box<MutableSpecifier<'tree>>),
}
impl<'tree> ::treesitter_types::FromNode<'tree> for MutPatternChildren<'tree> {
    #[allow(clippy::collapsible_else_if)]
    fn from_node(
        node: ::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        match node.kind() {
            "mutable_specifier" => Ok(Self::MutableSpecifier(::std::boxed::Box::new(
                ::treesitter_types::runtime::maybe_grow_stack(|| {
                    <MutableSpecifier as ::treesitter_types::FromNode>::from_node(node, src)
                })?,
            ))),
            _other => {
                if let Ok(v) = ::treesitter_types::runtime::maybe_grow_stack(|| {
                    <Pattern as ::treesitter_types::FromNode>::from_node(node, src)
                }) {
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
impl ::treesitter_types::Spanned for MutPatternChildren<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        match self {
            Self::Pattern(inner) => inner.span(),
            Self::MutableSpecifier(inner) => inner.span(),
        }
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum NegativeLiteralChildren<'tree> {
    FloatLiteral(::std::boxed::Box<FloatLiteral<'tree>>),
    IntegerLiteral(::std::boxed::Box<IntegerLiteral<'tree>>),
}
impl<'tree> ::treesitter_types::FromNode<'tree> for NegativeLiteralChildren<'tree> {
    #[allow(clippy::collapsible_else_if)]
    fn from_node(
        node: ::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        match node.kind() {
            "float_literal" => Ok(Self::FloatLiteral(::std::boxed::Box::new(
                ::treesitter_types::runtime::maybe_grow_stack(|| {
                    <FloatLiteral as ::treesitter_types::FromNode>::from_node(node, src)
                })?,
            ))),
            "integer_literal" => Ok(Self::IntegerLiteral(::std::boxed::Box::new(
                ::treesitter_types::runtime::maybe_grow_stack(|| {
                    <IntegerLiteral as ::treesitter_types::FromNode>::from_node(node, src)
                })?,
            ))),
            other => Err(::treesitter_types::ParseError::unexpected_kind(other, node)),
        }
    }
}
impl ::treesitter_types::Spanned for NegativeLiteralChildren<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        match self {
            Self::FloatLiteral(inner) => inner.span(),
            Self::IntegerLiteral(inner) => inner.span(),
        }
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum OrderedFieldDeclarationListChildren<'tree> {
    AttributeItem(::std::boxed::Box<AttributeItem<'tree>>),
    VisibilityModifier(::std::boxed::Box<VisibilityModifier<'tree>>),
}
impl<'tree> ::treesitter_types::FromNode<'tree> for OrderedFieldDeclarationListChildren<'tree> {
    #[allow(clippy::collapsible_else_if)]
    fn from_node(
        node: ::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        match node.kind() {
            "attribute_item" => Ok(Self::AttributeItem(::std::boxed::Box::new(
                ::treesitter_types::runtime::maybe_grow_stack(|| {
                    <AttributeItem as ::treesitter_types::FromNode>::from_node(node, src)
                })?,
            ))),
            "visibility_modifier" => Ok(Self::VisibilityModifier(::std::boxed::Box::new(
                ::treesitter_types::runtime::maybe_grow_stack(|| {
                    <VisibilityModifier as ::treesitter_types::FromNode>::from_node(node, src)
                })?,
            ))),
            other => Err(::treesitter_types::ParseError::unexpected_kind(other, node)),
        }
    }
}
impl ::treesitter_types::Spanned for OrderedFieldDeclarationListChildren<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        match self {
            Self::AttributeItem(inner) => inner.span(),
            Self::VisibilityModifier(inner) => inner.span(),
        }
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ParameterPattern<'tree> {
    Pattern(::std::boxed::Box<Pattern<'tree>>),
    SelfType(::std::boxed::Box<SelfType<'tree>>),
}
impl<'tree> ::treesitter_types::FromNode<'tree> for ParameterPattern<'tree> {
    #[allow(clippy::collapsible_else_if)]
    fn from_node(
        node: ::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        match node.kind() {
            "self" => Ok(Self::SelfType(::std::boxed::Box::new(
                ::treesitter_types::runtime::maybe_grow_stack(|| {
                    <SelfType as ::treesitter_types::FromNode>::from_node(node, src)
                })?,
            ))),
            _other => {
                if let Ok(v) = ::treesitter_types::runtime::maybe_grow_stack(|| {
                    <Pattern as ::treesitter_types::FromNode>::from_node(node, src)
                }) {
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
impl ::treesitter_types::Spanned for ParameterPattern<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        match self {
            Self::Pattern(inner) => inner.span(),
            Self::SelfType(inner) => inner.span(),
        }
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ParametersChildren<'tree> {
    Type(::std::boxed::Box<Type<'tree>>),
    AttributeItem(::std::boxed::Box<AttributeItem<'tree>>),
    Parameter(::std::boxed::Box<Parameter<'tree>>),
    SelfParameter(::std::boxed::Box<SelfParameter<'tree>>),
    VariadicParameter(::std::boxed::Box<VariadicParameter<'tree>>),
}
impl<'tree> ::treesitter_types::FromNode<'tree> for ParametersChildren<'tree> {
    #[allow(clippy::collapsible_else_if)]
    fn from_node(
        node: ::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        match node.kind() {
            "attribute_item" => Ok(Self::AttributeItem(::std::boxed::Box::new(
                ::treesitter_types::runtime::maybe_grow_stack(|| {
                    <AttributeItem as ::treesitter_types::FromNode>::from_node(node, src)
                })?,
            ))),
            "parameter" => Ok(Self::Parameter(::std::boxed::Box::new(
                ::treesitter_types::runtime::maybe_grow_stack(|| {
                    <Parameter as ::treesitter_types::FromNode>::from_node(node, src)
                })?,
            ))),
            "self_parameter" => Ok(Self::SelfParameter(::std::boxed::Box::new(
                ::treesitter_types::runtime::maybe_grow_stack(|| {
                    <SelfParameter as ::treesitter_types::FromNode>::from_node(node, src)
                })?,
            ))),
            "variadic_parameter" => Ok(Self::VariadicParameter(::std::boxed::Box::new(
                ::treesitter_types::runtime::maybe_grow_stack(|| {
                    <VariadicParameter as ::treesitter_types::FromNode>::from_node(node, src)
                })?,
            ))),
            _other => {
                if let Ok(v) = ::treesitter_types::runtime::maybe_grow_stack(|| {
                    <Type as ::treesitter_types::FromNode>::from_node(node, src)
                }) {
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
impl ::treesitter_types::Spanned for ParametersChildren<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        match self {
            Self::Type(inner) => inner.span(),
            Self::AttributeItem(inner) => inner.span(),
            Self::Parameter(inner) => inner.span(),
            Self::SelfParameter(inner) => inner.span(),
            Self::VariadicParameter(inner) => inner.span(),
        }
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum RangePatternLeft<'tree> {
    LiteralPattern(::std::boxed::Box<LiteralPattern<'tree>>),
    Crate(::std::boxed::Box<Crate<'tree>>),
    Identifier(::std::boxed::Box<Identifier<'tree>>),
    Metavariable(::std::boxed::Box<Metavariable<'tree>>),
    ScopedIdentifier(::std::boxed::Box<ScopedIdentifier<'tree>>),
    SelfType(::std::boxed::Box<SelfType<'tree>>),
    Super(::std::boxed::Box<Super<'tree>>),
}
impl<'tree> ::treesitter_types::FromNode<'tree> for RangePatternLeft<'tree> {
    #[allow(clippy::collapsible_else_if)]
    fn from_node(
        node: ::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        match node.kind() {
            "crate" => Ok(Self::Crate(::std::boxed::Box::new(
                ::treesitter_types::runtime::maybe_grow_stack(|| {
                    <Crate as ::treesitter_types::FromNode>::from_node(node, src)
                })?,
            ))),
            "identifier" => Ok(Self::Identifier(::std::boxed::Box::new(
                ::treesitter_types::runtime::maybe_grow_stack(|| {
                    <Identifier as ::treesitter_types::FromNode>::from_node(node, src)
                })?,
            ))),
            "metavariable" => Ok(Self::Metavariable(::std::boxed::Box::new(
                ::treesitter_types::runtime::maybe_grow_stack(|| {
                    <Metavariable as ::treesitter_types::FromNode>::from_node(node, src)
                })?,
            ))),
            "scoped_identifier" => Ok(Self::ScopedIdentifier(::std::boxed::Box::new(
                ::treesitter_types::runtime::maybe_grow_stack(|| {
                    <ScopedIdentifier as ::treesitter_types::FromNode>::from_node(node, src)
                })?,
            ))),
            "self" => Ok(Self::SelfType(::std::boxed::Box::new(
                ::treesitter_types::runtime::maybe_grow_stack(|| {
                    <SelfType as ::treesitter_types::FromNode>::from_node(node, src)
                })?,
            ))),
            "super" => Ok(Self::Super(::std::boxed::Box::new(
                ::treesitter_types::runtime::maybe_grow_stack(|| {
                    <Super as ::treesitter_types::FromNode>::from_node(node, src)
                })?,
            ))),
            _other => {
                if let Ok(v) = ::treesitter_types::runtime::maybe_grow_stack(|| {
                    <LiteralPattern as ::treesitter_types::FromNode>::from_node(node, src)
                }) {
                    Ok(Self::LiteralPattern(::std::boxed::Box::new(v)))
                } else {
                    Err(::treesitter_types::ParseError::unexpected_kind(
                        _other, node,
                    ))
                }
            }
        }
    }
}
impl ::treesitter_types::Spanned for RangePatternLeft<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        match self {
            Self::LiteralPattern(inner) => inner.span(),
            Self::Crate(inner) => inner.span(),
            Self::Identifier(inner) => inner.span(),
            Self::Metavariable(inner) => inner.span(),
            Self::ScopedIdentifier(inner) => inner.span(),
            Self::SelfType(inner) => inner.span(),
            Self::Super(inner) => inner.span(),
        }
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum RangePatternRight<'tree> {
    LiteralPattern(::std::boxed::Box<LiteralPattern<'tree>>),
    Crate(::std::boxed::Box<Crate<'tree>>),
    Identifier(::std::boxed::Box<Identifier<'tree>>),
    Metavariable(::std::boxed::Box<Metavariable<'tree>>),
    ScopedIdentifier(::std::boxed::Box<ScopedIdentifier<'tree>>),
    SelfType(::std::boxed::Box<SelfType<'tree>>),
    Super(::std::boxed::Box<Super<'tree>>),
}
impl<'tree> ::treesitter_types::FromNode<'tree> for RangePatternRight<'tree> {
    #[allow(clippy::collapsible_else_if)]
    fn from_node(
        node: ::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        match node.kind() {
            "crate" => Ok(Self::Crate(::std::boxed::Box::new(
                ::treesitter_types::runtime::maybe_grow_stack(|| {
                    <Crate as ::treesitter_types::FromNode>::from_node(node, src)
                })?,
            ))),
            "identifier" => Ok(Self::Identifier(::std::boxed::Box::new(
                ::treesitter_types::runtime::maybe_grow_stack(|| {
                    <Identifier as ::treesitter_types::FromNode>::from_node(node, src)
                })?,
            ))),
            "metavariable" => Ok(Self::Metavariable(::std::boxed::Box::new(
                ::treesitter_types::runtime::maybe_grow_stack(|| {
                    <Metavariable as ::treesitter_types::FromNode>::from_node(node, src)
                })?,
            ))),
            "scoped_identifier" => Ok(Self::ScopedIdentifier(::std::boxed::Box::new(
                ::treesitter_types::runtime::maybe_grow_stack(|| {
                    <ScopedIdentifier as ::treesitter_types::FromNode>::from_node(node, src)
                })?,
            ))),
            "self" => Ok(Self::SelfType(::std::boxed::Box::new(
                ::treesitter_types::runtime::maybe_grow_stack(|| {
                    <SelfType as ::treesitter_types::FromNode>::from_node(node, src)
                })?,
            ))),
            "super" => Ok(Self::Super(::std::boxed::Box::new(
                ::treesitter_types::runtime::maybe_grow_stack(|| {
                    <Super as ::treesitter_types::FromNode>::from_node(node, src)
                })?,
            ))),
            _other => {
                if let Ok(v) = ::treesitter_types::runtime::maybe_grow_stack(|| {
                    <LiteralPattern as ::treesitter_types::FromNode>::from_node(node, src)
                }) {
                    Ok(Self::LiteralPattern(::std::boxed::Box::new(v)))
                } else {
                    Err(::treesitter_types::ParseError::unexpected_kind(
                        _other, node,
                    ))
                }
            }
        }
    }
}
impl ::treesitter_types::Spanned for RangePatternRight<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        match self {
            Self::LiteralPattern(inner) => inner.span(),
            Self::Crate(inner) => inner.span(),
            Self::Identifier(inner) => inner.span(),
            Self::Metavariable(inner) => inner.span(),
            Self::ScopedIdentifier(inner) => inner.span(),
            Self::SelfType(inner) => inner.span(),
            Self::Super(inner) => inner.span(),
        }
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ReferencePatternChildren<'tree> {
    Pattern(::std::boxed::Box<Pattern<'tree>>),
    MutableSpecifier(::std::boxed::Box<MutableSpecifier<'tree>>),
}
impl<'tree> ::treesitter_types::FromNode<'tree> for ReferencePatternChildren<'tree> {
    #[allow(clippy::collapsible_else_if)]
    fn from_node(
        node: ::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        match node.kind() {
            "mutable_specifier" => Ok(Self::MutableSpecifier(::std::boxed::Box::new(
                ::treesitter_types::runtime::maybe_grow_stack(|| {
                    <MutableSpecifier as ::treesitter_types::FromNode>::from_node(node, src)
                })?,
            ))),
            _other => {
                if let Ok(v) = ::treesitter_types::runtime::maybe_grow_stack(|| {
                    <Pattern as ::treesitter_types::FromNode>::from_node(node, src)
                }) {
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
impl ::treesitter_types::Spanned for ReferencePatternChildren<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        match self {
            Self::Pattern(inner) => inner.span(),
            Self::MutableSpecifier(inner) => inner.span(),
        }
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ReferenceTypeChildren<'tree> {
    Lifetime(::std::boxed::Box<Lifetime<'tree>>),
    MutableSpecifier(::std::boxed::Box<MutableSpecifier<'tree>>),
}
impl<'tree> ::treesitter_types::FromNode<'tree> for ReferenceTypeChildren<'tree> {
    #[allow(clippy::collapsible_else_if)]
    fn from_node(
        node: ::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        match node.kind() {
            "lifetime" => Ok(Self::Lifetime(::std::boxed::Box::new(
                ::treesitter_types::runtime::maybe_grow_stack(|| {
                    <Lifetime as ::treesitter_types::FromNode>::from_node(node, src)
                })?,
            ))),
            "mutable_specifier" => Ok(Self::MutableSpecifier(::std::boxed::Box::new(
                ::treesitter_types::runtime::maybe_grow_stack(|| {
                    <MutableSpecifier as ::treesitter_types::FromNode>::from_node(node, src)
                })?,
            ))),
            other => Err(::treesitter_types::ParseError::unexpected_kind(other, node)),
        }
    }
}
impl ::treesitter_types::Spanned for ReferenceTypeChildren<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        match self {
            Self::Lifetime(inner) => inner.span(),
            Self::MutableSpecifier(inner) => inner.span(),
        }
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ScopedIdentifierName<'tree> {
    Identifier(::std::boxed::Box<Identifier<'tree>>),
    Super(::std::boxed::Box<Super<'tree>>),
}
impl<'tree> ::treesitter_types::FromNode<'tree> for ScopedIdentifierName<'tree> {
    #[allow(clippy::collapsible_else_if)]
    fn from_node(
        node: ::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        match node.kind() {
            "identifier" => Ok(Self::Identifier(::std::boxed::Box::new(
                ::treesitter_types::runtime::maybe_grow_stack(|| {
                    <Identifier as ::treesitter_types::FromNode>::from_node(node, src)
                })?,
            ))),
            "super" => Ok(Self::Super(::std::boxed::Box::new(
                ::treesitter_types::runtime::maybe_grow_stack(|| {
                    <Super as ::treesitter_types::FromNode>::from_node(node, src)
                })?,
            ))),
            other => Err(::treesitter_types::ParseError::unexpected_kind(other, node)),
        }
    }
}
impl ::treesitter_types::Spanned for ScopedIdentifierName<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        match self {
            Self::Identifier(inner) => inner.span(),
            Self::Super(inner) => inner.span(),
        }
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ScopedIdentifierPath<'tree> {
    BracketedType(::std::boxed::Box<BracketedType<'tree>>),
    Crate(::std::boxed::Box<Crate<'tree>>),
    GenericType(::std::boxed::Box<GenericType<'tree>>),
    Identifier(::std::boxed::Box<Identifier<'tree>>),
    Metavariable(::std::boxed::Box<Metavariable<'tree>>),
    ScopedIdentifier(::std::boxed::Box<ScopedIdentifier<'tree>>),
    SelfType(::std::boxed::Box<SelfType<'tree>>),
    Super(::std::boxed::Box<Super<'tree>>),
}
impl<'tree> ::treesitter_types::FromNode<'tree> for ScopedIdentifierPath<'tree> {
    #[allow(clippy::collapsible_else_if)]
    fn from_node(
        node: ::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        match node.kind() {
            "bracketed_type" => Ok(Self::BracketedType(::std::boxed::Box::new(
                ::treesitter_types::runtime::maybe_grow_stack(|| {
                    <BracketedType as ::treesitter_types::FromNode>::from_node(node, src)
                })?,
            ))),
            "crate" => Ok(Self::Crate(::std::boxed::Box::new(
                ::treesitter_types::runtime::maybe_grow_stack(|| {
                    <Crate as ::treesitter_types::FromNode>::from_node(node, src)
                })?,
            ))),
            "generic_type" => Ok(Self::GenericType(::std::boxed::Box::new(
                ::treesitter_types::runtime::maybe_grow_stack(|| {
                    <GenericType as ::treesitter_types::FromNode>::from_node(node, src)
                })?,
            ))),
            "identifier" => Ok(Self::Identifier(::std::boxed::Box::new(
                ::treesitter_types::runtime::maybe_grow_stack(|| {
                    <Identifier as ::treesitter_types::FromNode>::from_node(node, src)
                })?,
            ))),
            "metavariable" => Ok(Self::Metavariable(::std::boxed::Box::new(
                ::treesitter_types::runtime::maybe_grow_stack(|| {
                    <Metavariable as ::treesitter_types::FromNode>::from_node(node, src)
                })?,
            ))),
            "scoped_identifier" => Ok(Self::ScopedIdentifier(::std::boxed::Box::new(
                ::treesitter_types::runtime::maybe_grow_stack(|| {
                    <ScopedIdentifier as ::treesitter_types::FromNode>::from_node(node, src)
                })?,
            ))),
            "self" => Ok(Self::SelfType(::std::boxed::Box::new(
                ::treesitter_types::runtime::maybe_grow_stack(|| {
                    <SelfType as ::treesitter_types::FromNode>::from_node(node, src)
                })?,
            ))),
            "super" => Ok(Self::Super(::std::boxed::Box::new(
                ::treesitter_types::runtime::maybe_grow_stack(|| {
                    <Super as ::treesitter_types::FromNode>::from_node(node, src)
                })?,
            ))),
            other => Err(::treesitter_types::ParseError::unexpected_kind(other, node)),
        }
    }
}
impl ::treesitter_types::Spanned for ScopedIdentifierPath<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        match self {
            Self::BracketedType(inner) => inner.span(),
            Self::Crate(inner) => inner.span(),
            Self::GenericType(inner) => inner.span(),
            Self::Identifier(inner) => inner.span(),
            Self::Metavariable(inner) => inner.span(),
            Self::ScopedIdentifier(inner) => inner.span(),
            Self::SelfType(inner) => inner.span(),
            Self::Super(inner) => inner.span(),
        }
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ScopedTypeIdentifierPath<'tree> {
    BracketedType(::std::boxed::Box<BracketedType<'tree>>),
    Crate(::std::boxed::Box<Crate<'tree>>),
    GenericType(::std::boxed::Box<GenericType<'tree>>),
    Identifier(::std::boxed::Box<Identifier<'tree>>),
    Metavariable(::std::boxed::Box<Metavariable<'tree>>),
    ScopedIdentifier(::std::boxed::Box<ScopedIdentifier<'tree>>),
    SelfType(::std::boxed::Box<SelfType<'tree>>),
    Super(::std::boxed::Box<Super<'tree>>),
}
impl<'tree> ::treesitter_types::FromNode<'tree> for ScopedTypeIdentifierPath<'tree> {
    #[allow(clippy::collapsible_else_if)]
    fn from_node(
        node: ::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        match node.kind() {
            "bracketed_type" => Ok(Self::BracketedType(::std::boxed::Box::new(
                ::treesitter_types::runtime::maybe_grow_stack(|| {
                    <BracketedType as ::treesitter_types::FromNode>::from_node(node, src)
                })?,
            ))),
            "crate" => Ok(Self::Crate(::std::boxed::Box::new(
                ::treesitter_types::runtime::maybe_grow_stack(|| {
                    <Crate as ::treesitter_types::FromNode>::from_node(node, src)
                })?,
            ))),
            "generic_type" => Ok(Self::GenericType(::std::boxed::Box::new(
                ::treesitter_types::runtime::maybe_grow_stack(|| {
                    <GenericType as ::treesitter_types::FromNode>::from_node(node, src)
                })?,
            ))),
            "identifier" => Ok(Self::Identifier(::std::boxed::Box::new(
                ::treesitter_types::runtime::maybe_grow_stack(|| {
                    <Identifier as ::treesitter_types::FromNode>::from_node(node, src)
                })?,
            ))),
            "metavariable" => Ok(Self::Metavariable(::std::boxed::Box::new(
                ::treesitter_types::runtime::maybe_grow_stack(|| {
                    <Metavariable as ::treesitter_types::FromNode>::from_node(node, src)
                })?,
            ))),
            "scoped_identifier" => Ok(Self::ScopedIdentifier(::std::boxed::Box::new(
                ::treesitter_types::runtime::maybe_grow_stack(|| {
                    <ScopedIdentifier as ::treesitter_types::FromNode>::from_node(node, src)
                })?,
            ))),
            "self" => Ok(Self::SelfType(::std::boxed::Box::new(
                ::treesitter_types::runtime::maybe_grow_stack(|| {
                    <SelfType as ::treesitter_types::FromNode>::from_node(node, src)
                })?,
            ))),
            "super" => Ok(Self::Super(::std::boxed::Box::new(
                ::treesitter_types::runtime::maybe_grow_stack(|| {
                    <Super as ::treesitter_types::FromNode>::from_node(node, src)
                })?,
            ))),
            other => Err(::treesitter_types::ParseError::unexpected_kind(other, node)),
        }
    }
}
impl ::treesitter_types::Spanned for ScopedTypeIdentifierPath<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        match self {
            Self::BracketedType(inner) => inner.span(),
            Self::Crate(inner) => inner.span(),
            Self::GenericType(inner) => inner.span(),
            Self::Identifier(inner) => inner.span(),
            Self::Metavariable(inner) => inner.span(),
            Self::ScopedIdentifier(inner) => inner.span(),
            Self::SelfType(inner) => inner.span(),
            Self::Super(inner) => inner.span(),
        }
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ScopedUseListPath<'tree> {
    Crate(::std::boxed::Box<Crate<'tree>>),
    Identifier(::std::boxed::Box<Identifier<'tree>>),
    Metavariable(::std::boxed::Box<Metavariable<'tree>>),
    ScopedIdentifier(::std::boxed::Box<ScopedIdentifier<'tree>>),
    SelfType(::std::boxed::Box<SelfType<'tree>>),
    Super(::std::boxed::Box<Super<'tree>>),
}
impl<'tree> ::treesitter_types::FromNode<'tree> for ScopedUseListPath<'tree> {
    #[allow(clippy::collapsible_else_if)]
    fn from_node(
        node: ::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        match node.kind() {
            "crate" => Ok(Self::Crate(::std::boxed::Box::new(
                ::treesitter_types::runtime::maybe_grow_stack(|| {
                    <Crate as ::treesitter_types::FromNode>::from_node(node, src)
                })?,
            ))),
            "identifier" => Ok(Self::Identifier(::std::boxed::Box::new(
                ::treesitter_types::runtime::maybe_grow_stack(|| {
                    <Identifier as ::treesitter_types::FromNode>::from_node(node, src)
                })?,
            ))),
            "metavariable" => Ok(Self::Metavariable(::std::boxed::Box::new(
                ::treesitter_types::runtime::maybe_grow_stack(|| {
                    <Metavariable as ::treesitter_types::FromNode>::from_node(node, src)
                })?,
            ))),
            "scoped_identifier" => Ok(Self::ScopedIdentifier(::std::boxed::Box::new(
                ::treesitter_types::runtime::maybe_grow_stack(|| {
                    <ScopedIdentifier as ::treesitter_types::FromNode>::from_node(node, src)
                })?,
            ))),
            "self" => Ok(Self::SelfType(::std::boxed::Box::new(
                ::treesitter_types::runtime::maybe_grow_stack(|| {
                    <SelfType as ::treesitter_types::FromNode>::from_node(node, src)
                })?,
            ))),
            "super" => Ok(Self::Super(::std::boxed::Box::new(
                ::treesitter_types::runtime::maybe_grow_stack(|| {
                    <Super as ::treesitter_types::FromNode>::from_node(node, src)
                })?,
            ))),
            other => Err(::treesitter_types::ParseError::unexpected_kind(other, node)),
        }
    }
}
impl ::treesitter_types::Spanned for ScopedUseListPath<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        match self {
            Self::Crate(inner) => inner.span(),
            Self::Identifier(inner) => inner.span(),
            Self::Metavariable(inner) => inner.span(),
            Self::ScopedIdentifier(inner) => inner.span(),
            Self::SelfType(inner) => inner.span(),
            Self::Super(inner) => inner.span(),
        }
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum SelfParameterChildren<'tree> {
    Lifetime(::std::boxed::Box<Lifetime<'tree>>),
    MutableSpecifier(::std::boxed::Box<MutableSpecifier<'tree>>),
    SelfType(::std::boxed::Box<SelfType<'tree>>),
}
impl<'tree> ::treesitter_types::FromNode<'tree> for SelfParameterChildren<'tree> {
    #[allow(clippy::collapsible_else_if)]
    fn from_node(
        node: ::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        match node.kind() {
            "lifetime" => Ok(Self::Lifetime(::std::boxed::Box::new(
                ::treesitter_types::runtime::maybe_grow_stack(|| {
                    <Lifetime as ::treesitter_types::FromNode>::from_node(node, src)
                })?,
            ))),
            "mutable_specifier" => Ok(Self::MutableSpecifier(::std::boxed::Box::new(
                ::treesitter_types::runtime::maybe_grow_stack(|| {
                    <MutableSpecifier as ::treesitter_types::FromNode>::from_node(node, src)
                })?,
            ))),
            "self" => Ok(Self::SelfType(::std::boxed::Box::new(
                ::treesitter_types::runtime::maybe_grow_stack(|| {
                    <SelfType as ::treesitter_types::FromNode>::from_node(node, src)
                })?,
            ))),
            other => Err(::treesitter_types::ParseError::unexpected_kind(other, node)),
        }
    }
}
impl ::treesitter_types::Spanned for SelfParameterChildren<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        match self {
            Self::Lifetime(inner) => inner.span(),
            Self::MutableSpecifier(inner) => inner.span(),
            Self::SelfType(inner) => inner.span(),
        }
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ShorthandFieldInitializerChildren<'tree> {
    AttributeItem(::std::boxed::Box<AttributeItem<'tree>>),
    Identifier(::std::boxed::Box<Identifier<'tree>>),
}
impl<'tree> ::treesitter_types::FromNode<'tree> for ShorthandFieldInitializerChildren<'tree> {
    #[allow(clippy::collapsible_else_if)]
    fn from_node(
        node: ::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        match node.kind() {
            "attribute_item" => Ok(Self::AttributeItem(::std::boxed::Box::new(
                ::treesitter_types::runtime::maybe_grow_stack(|| {
                    <AttributeItem as ::treesitter_types::FromNode>::from_node(node, src)
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
impl ::treesitter_types::Spanned for ShorthandFieldInitializerChildren<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        match self {
            Self::AttributeItem(inner) => inner.span(),
            Self::Identifier(inner) => inner.span(),
        }
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum SourceFileChildren<'tree> {
    DeclarationStatement(::std::boxed::Box<DeclarationStatement<'tree>>),
    ExpressionStatement(::std::boxed::Box<ExpressionStatement<'tree>>),
    Shebang(::std::boxed::Box<Shebang<'tree>>),
}
impl<'tree> ::treesitter_types::FromNode<'tree> for SourceFileChildren<'tree> {
    #[allow(clippy::collapsible_else_if)]
    fn from_node(
        node: ::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        match node.kind() {
            "expression_statement" => Ok(Self::ExpressionStatement(::std::boxed::Box::new(
                ::treesitter_types::runtime::maybe_grow_stack(|| {
                    <ExpressionStatement as ::treesitter_types::FromNode>::from_node(node, src)
                })?,
            ))),
            "shebang" => Ok(Self::Shebang(::std::boxed::Box::new(
                ::treesitter_types::runtime::maybe_grow_stack(|| {
                    <Shebang as ::treesitter_types::FromNode>::from_node(node, src)
                })?,
            ))),
            _other => {
                if let Ok(v) = ::treesitter_types::runtime::maybe_grow_stack(|| {
                    <DeclarationStatement as ::treesitter_types::FromNode>::from_node(node, src)
                }) {
                    Ok(Self::DeclarationStatement(::std::boxed::Box::new(v)))
                } else {
                    Err(::treesitter_types::ParseError::unexpected_kind(
                        _other, node,
                    ))
                }
            }
        }
    }
}
impl ::treesitter_types::Spanned for SourceFileChildren<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        match self {
            Self::DeclarationStatement(inner) => inner.span(),
            Self::ExpressionStatement(inner) => inner.span(),
            Self::Shebang(inner) => inner.span(),
        }
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum StaticItemChildren<'tree> {
    MutableSpecifier(::std::boxed::Box<MutableSpecifier<'tree>>),
    VisibilityModifier(::std::boxed::Box<VisibilityModifier<'tree>>),
}
impl<'tree> ::treesitter_types::FromNode<'tree> for StaticItemChildren<'tree> {
    #[allow(clippy::collapsible_else_if)]
    fn from_node(
        node: ::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        match node.kind() {
            "mutable_specifier" => Ok(Self::MutableSpecifier(::std::boxed::Box::new(
                ::treesitter_types::runtime::maybe_grow_stack(|| {
                    <MutableSpecifier as ::treesitter_types::FromNode>::from_node(node, src)
                })?,
            ))),
            "visibility_modifier" => Ok(Self::VisibilityModifier(::std::boxed::Box::new(
                ::treesitter_types::runtime::maybe_grow_stack(|| {
                    <VisibilityModifier as ::treesitter_types::FromNode>::from_node(node, src)
                })?,
            ))),
            other => Err(::treesitter_types::ParseError::unexpected_kind(other, node)),
        }
    }
}
impl ::treesitter_types::Spanned for StaticItemChildren<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        match self {
            Self::MutableSpecifier(inner) => inner.span(),
            Self::VisibilityModifier(inner) => inner.span(),
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
        node: ::tree_sitter::Node<'tree>,
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
pub enum StructExpressionName<'tree> {
    GenericTypeWithTurbofish(::std::boxed::Box<GenericTypeWithTurbofish<'tree>>),
    ScopedTypeIdentifier(::std::boxed::Box<ScopedTypeIdentifier<'tree>>),
    TypeIdentifier(::std::boxed::Box<TypeIdentifier<'tree>>),
}
impl<'tree> ::treesitter_types::FromNode<'tree> for StructExpressionName<'tree> {
    #[allow(clippy::collapsible_else_if)]
    fn from_node(
        node: ::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        match node.kind() {
            "generic_type_with_turbofish" => Ok(Self::GenericTypeWithTurbofish(
                ::std::boxed::Box::new(::treesitter_types::runtime::maybe_grow_stack(|| {
                    <GenericTypeWithTurbofish as ::treesitter_types::FromNode>::from_node(node, src)
                })?),
            )),
            "scoped_type_identifier" => Ok(Self::ScopedTypeIdentifier(::std::boxed::Box::new(
                ::treesitter_types::runtime::maybe_grow_stack(|| {
                    <ScopedTypeIdentifier as ::treesitter_types::FromNode>::from_node(node, src)
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
impl ::treesitter_types::Spanned for StructExpressionName<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        match self {
            Self::GenericTypeWithTurbofish(inner) => inner.span(),
            Self::ScopedTypeIdentifier(inner) => inner.span(),
            Self::TypeIdentifier(inner) => inner.span(),
        }
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum StructItemBody<'tree> {
    FieldDeclarationList(::std::boxed::Box<FieldDeclarationList<'tree>>),
    OrderedFieldDeclarationList(::std::boxed::Box<OrderedFieldDeclarationList<'tree>>),
}
impl<'tree> ::treesitter_types::FromNode<'tree> for StructItemBody<'tree> {
    #[allow(clippy::collapsible_else_if)]
    fn from_node(
        node: ::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        match node.kind() {
            "field_declaration_list" => Ok(Self::FieldDeclarationList(::std::boxed::Box::new(
                ::treesitter_types::runtime::maybe_grow_stack(|| {
                    <FieldDeclarationList as ::treesitter_types::FromNode>::from_node(node, src)
                })?,
            ))),
            "ordered_field_declaration_list" => Ok(Self::OrderedFieldDeclarationList(
                ::std::boxed::Box::new(::treesitter_types::runtime::maybe_grow_stack(|| {
                    <OrderedFieldDeclarationList as ::treesitter_types::FromNode>::from_node(
                        node, src,
                    )
                })?),
            )),
            other => Err(::treesitter_types::ParseError::unexpected_kind(other, node)),
        }
    }
}
impl ::treesitter_types::Spanned for StructItemBody<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        match self {
            Self::FieldDeclarationList(inner) => inner.span(),
            Self::OrderedFieldDeclarationList(inner) => inner.span(),
        }
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum StructItemChildren<'tree> {
    VisibilityModifier(::std::boxed::Box<VisibilityModifier<'tree>>),
    WhereClause(::std::boxed::Box<WhereClause<'tree>>),
}
impl<'tree> ::treesitter_types::FromNode<'tree> for StructItemChildren<'tree> {
    #[allow(clippy::collapsible_else_if)]
    fn from_node(
        node: ::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        match node.kind() {
            "visibility_modifier" => Ok(Self::VisibilityModifier(::std::boxed::Box::new(
                ::treesitter_types::runtime::maybe_grow_stack(|| {
                    <VisibilityModifier as ::treesitter_types::FromNode>::from_node(node, src)
                })?,
            ))),
            "where_clause" => Ok(Self::WhereClause(::std::boxed::Box::new(
                ::treesitter_types::runtime::maybe_grow_stack(|| {
                    <WhereClause as ::treesitter_types::FromNode>::from_node(node, src)
                })?,
            ))),
            other => Err(::treesitter_types::ParseError::unexpected_kind(other, node)),
        }
    }
}
impl ::treesitter_types::Spanned for StructItemChildren<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        match self {
            Self::VisibilityModifier(inner) => inner.span(),
            Self::WhereClause(inner) => inner.span(),
        }
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum StructPatternType<'tree> {
    ScopedTypeIdentifier(::std::boxed::Box<ScopedTypeIdentifier<'tree>>),
    TypeIdentifier(::std::boxed::Box<TypeIdentifier<'tree>>),
}
impl<'tree> ::treesitter_types::FromNode<'tree> for StructPatternType<'tree> {
    #[allow(clippy::collapsible_else_if)]
    fn from_node(
        node: ::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        match node.kind() {
            "scoped_type_identifier" => Ok(Self::ScopedTypeIdentifier(::std::boxed::Box::new(
                ::treesitter_types::runtime::maybe_grow_stack(|| {
                    <ScopedTypeIdentifier as ::treesitter_types::FromNode>::from_node(node, src)
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
impl ::treesitter_types::Spanned for StructPatternType<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        match self {
            Self::ScopedTypeIdentifier(inner) => inner.span(),
            Self::TypeIdentifier(inner) => inner.span(),
        }
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum StructPatternChildren<'tree> {
    FieldPattern(::std::boxed::Box<FieldPattern<'tree>>),
    RemainingFieldPattern(::std::boxed::Box<RemainingFieldPattern<'tree>>),
}
impl<'tree> ::treesitter_types::FromNode<'tree> for StructPatternChildren<'tree> {
    #[allow(clippy::collapsible_else_if)]
    fn from_node(
        node: ::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        match node.kind() {
            "field_pattern" => Ok(Self::FieldPattern(::std::boxed::Box::new(
                ::treesitter_types::runtime::maybe_grow_stack(|| {
                    <FieldPattern as ::treesitter_types::FromNode>::from_node(node, src)
                })?,
            ))),
            "remaining_field_pattern" => Ok(Self::RemainingFieldPattern(::std::boxed::Box::new(
                ::treesitter_types::runtime::maybe_grow_stack(|| {
                    <RemainingFieldPattern as ::treesitter_types::FromNode>::from_node(node, src)
                })?,
            ))),
            other => Err(::treesitter_types::ParseError::unexpected_kind(other, node)),
        }
    }
}
impl ::treesitter_types::Spanned for StructPatternChildren<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        match self {
            Self::FieldPattern(inner) => inner.span(),
            Self::RemainingFieldPattern(inner) => inner.span(),
        }
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum TokenRepetitionChildren<'tree> {
    Literal(::std::boxed::Box<Literal<'tree>>),
    Crate(::std::boxed::Box<Crate<'tree>>),
    Identifier(::std::boxed::Box<Identifier<'tree>>),
    Metavariable(::std::boxed::Box<Metavariable<'tree>>),
    MutableSpecifier(::std::boxed::Box<MutableSpecifier<'tree>>),
    PrimitiveType(::std::boxed::Box<PrimitiveType<'tree>>),
    SelfType(::std::boxed::Box<SelfType<'tree>>),
    Super(::std::boxed::Box<Super<'tree>>),
    TokenRepetition(::std::boxed::Box<TokenRepetition<'tree>>),
    TokenTree(::std::boxed::Box<TokenTree<'tree>>),
}
impl<'tree> ::treesitter_types::FromNode<'tree> for TokenRepetitionChildren<'tree> {
    #[allow(clippy::collapsible_else_if)]
    fn from_node(
        node: ::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        match node.kind() {
            "crate" => Ok(Self::Crate(::std::boxed::Box::new(
                ::treesitter_types::runtime::maybe_grow_stack(|| {
                    <Crate as ::treesitter_types::FromNode>::from_node(node, src)
                })?,
            ))),
            "identifier" => Ok(Self::Identifier(::std::boxed::Box::new(
                ::treesitter_types::runtime::maybe_grow_stack(|| {
                    <Identifier as ::treesitter_types::FromNode>::from_node(node, src)
                })?,
            ))),
            "metavariable" => Ok(Self::Metavariable(::std::boxed::Box::new(
                ::treesitter_types::runtime::maybe_grow_stack(|| {
                    <Metavariable as ::treesitter_types::FromNode>::from_node(node, src)
                })?,
            ))),
            "mutable_specifier" => Ok(Self::MutableSpecifier(::std::boxed::Box::new(
                ::treesitter_types::runtime::maybe_grow_stack(|| {
                    <MutableSpecifier as ::treesitter_types::FromNode>::from_node(node, src)
                })?,
            ))),
            "primitive_type" => Ok(Self::PrimitiveType(::std::boxed::Box::new(
                ::treesitter_types::runtime::maybe_grow_stack(|| {
                    <PrimitiveType as ::treesitter_types::FromNode>::from_node(node, src)
                })?,
            ))),
            "self" => Ok(Self::SelfType(::std::boxed::Box::new(
                ::treesitter_types::runtime::maybe_grow_stack(|| {
                    <SelfType as ::treesitter_types::FromNode>::from_node(node, src)
                })?,
            ))),
            "super" => Ok(Self::Super(::std::boxed::Box::new(
                ::treesitter_types::runtime::maybe_grow_stack(|| {
                    <Super as ::treesitter_types::FromNode>::from_node(node, src)
                })?,
            ))),
            "token_repetition" => Ok(Self::TokenRepetition(::std::boxed::Box::new(
                ::treesitter_types::runtime::maybe_grow_stack(|| {
                    <TokenRepetition as ::treesitter_types::FromNode>::from_node(node, src)
                })?,
            ))),
            "token_tree" => Ok(Self::TokenTree(::std::boxed::Box::new(
                ::treesitter_types::runtime::maybe_grow_stack(|| {
                    <TokenTree as ::treesitter_types::FromNode>::from_node(node, src)
                })?,
            ))),
            _other => {
                if let Ok(v) = ::treesitter_types::runtime::maybe_grow_stack(|| {
                    <Literal as ::treesitter_types::FromNode>::from_node(node, src)
                }) {
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
impl ::treesitter_types::Spanned for TokenRepetitionChildren<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        match self {
            Self::Literal(inner) => inner.span(),
            Self::Crate(inner) => inner.span(),
            Self::Identifier(inner) => inner.span(),
            Self::Metavariable(inner) => inner.span(),
            Self::MutableSpecifier(inner) => inner.span(),
            Self::PrimitiveType(inner) => inner.span(),
            Self::SelfType(inner) => inner.span(),
            Self::Super(inner) => inner.span(),
            Self::TokenRepetition(inner) => inner.span(),
            Self::TokenTree(inner) => inner.span(),
        }
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum TokenRepetitionPatternChildren<'tree> {
    Literal(::std::boxed::Box<Literal<'tree>>),
    Crate(::std::boxed::Box<Crate<'tree>>),
    Identifier(::std::boxed::Box<Identifier<'tree>>),
    Metavariable(::std::boxed::Box<Metavariable<'tree>>),
    MutableSpecifier(::std::boxed::Box<MutableSpecifier<'tree>>),
    PrimitiveType(::std::boxed::Box<PrimitiveType<'tree>>),
    SelfType(::std::boxed::Box<SelfType<'tree>>),
    Super(::std::boxed::Box<Super<'tree>>),
    TokenBindingPattern(::std::boxed::Box<TokenBindingPattern<'tree>>),
    TokenRepetitionPattern(::std::boxed::Box<TokenRepetitionPattern<'tree>>),
    TokenTreePattern(::std::boxed::Box<TokenTreePattern<'tree>>),
}
impl<'tree> ::treesitter_types::FromNode<'tree> for TokenRepetitionPatternChildren<'tree> {
    #[allow(clippy::collapsible_else_if)]
    fn from_node(
        node: ::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        match node.kind() {
            "crate" => Ok(Self::Crate(::std::boxed::Box::new(
                ::treesitter_types::runtime::maybe_grow_stack(|| {
                    <Crate as ::treesitter_types::FromNode>::from_node(node, src)
                })?,
            ))),
            "identifier" => Ok(Self::Identifier(::std::boxed::Box::new(
                ::treesitter_types::runtime::maybe_grow_stack(|| {
                    <Identifier as ::treesitter_types::FromNode>::from_node(node, src)
                })?,
            ))),
            "metavariable" => Ok(Self::Metavariable(::std::boxed::Box::new(
                ::treesitter_types::runtime::maybe_grow_stack(|| {
                    <Metavariable as ::treesitter_types::FromNode>::from_node(node, src)
                })?,
            ))),
            "mutable_specifier" => Ok(Self::MutableSpecifier(::std::boxed::Box::new(
                ::treesitter_types::runtime::maybe_grow_stack(|| {
                    <MutableSpecifier as ::treesitter_types::FromNode>::from_node(node, src)
                })?,
            ))),
            "primitive_type" => Ok(Self::PrimitiveType(::std::boxed::Box::new(
                ::treesitter_types::runtime::maybe_grow_stack(|| {
                    <PrimitiveType as ::treesitter_types::FromNode>::from_node(node, src)
                })?,
            ))),
            "self" => Ok(Self::SelfType(::std::boxed::Box::new(
                ::treesitter_types::runtime::maybe_grow_stack(|| {
                    <SelfType as ::treesitter_types::FromNode>::from_node(node, src)
                })?,
            ))),
            "super" => Ok(Self::Super(::std::boxed::Box::new(
                ::treesitter_types::runtime::maybe_grow_stack(|| {
                    <Super as ::treesitter_types::FromNode>::from_node(node, src)
                })?,
            ))),
            "token_binding_pattern" => Ok(Self::TokenBindingPattern(::std::boxed::Box::new(
                ::treesitter_types::runtime::maybe_grow_stack(|| {
                    <TokenBindingPattern as ::treesitter_types::FromNode>::from_node(node, src)
                })?,
            ))),
            "token_repetition_pattern" => Ok(Self::TokenRepetitionPattern(::std::boxed::Box::new(
                ::treesitter_types::runtime::maybe_grow_stack(|| {
                    <TokenRepetitionPattern as ::treesitter_types::FromNode>::from_node(node, src)
                })?,
            ))),
            "token_tree_pattern" => Ok(Self::TokenTreePattern(::std::boxed::Box::new(
                ::treesitter_types::runtime::maybe_grow_stack(|| {
                    <TokenTreePattern as ::treesitter_types::FromNode>::from_node(node, src)
                })?,
            ))),
            _other => {
                if let Ok(v) = ::treesitter_types::runtime::maybe_grow_stack(|| {
                    <Literal as ::treesitter_types::FromNode>::from_node(node, src)
                }) {
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
impl ::treesitter_types::Spanned for TokenRepetitionPatternChildren<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        match self {
            Self::Literal(inner) => inner.span(),
            Self::Crate(inner) => inner.span(),
            Self::Identifier(inner) => inner.span(),
            Self::Metavariable(inner) => inner.span(),
            Self::MutableSpecifier(inner) => inner.span(),
            Self::PrimitiveType(inner) => inner.span(),
            Self::SelfType(inner) => inner.span(),
            Self::Super(inner) => inner.span(),
            Self::TokenBindingPattern(inner) => inner.span(),
            Self::TokenRepetitionPattern(inner) => inner.span(),
            Self::TokenTreePattern(inner) => inner.span(),
        }
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum TokenTreeChildren<'tree> {
    Literal(::std::boxed::Box<Literal<'tree>>),
    Crate(::std::boxed::Box<Crate<'tree>>),
    Identifier(::std::boxed::Box<Identifier<'tree>>),
    Metavariable(::std::boxed::Box<Metavariable<'tree>>),
    MutableSpecifier(::std::boxed::Box<MutableSpecifier<'tree>>),
    PrimitiveType(::std::boxed::Box<PrimitiveType<'tree>>),
    SelfType(::std::boxed::Box<SelfType<'tree>>),
    Super(::std::boxed::Box<Super<'tree>>),
    TokenRepetition(::std::boxed::Box<TokenRepetition<'tree>>),
    TokenTree(::std::boxed::Box<TokenTree<'tree>>),
}
impl<'tree> ::treesitter_types::FromNode<'tree> for TokenTreeChildren<'tree> {
    #[allow(clippy::collapsible_else_if)]
    fn from_node(
        node: ::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        match node.kind() {
            "crate" => Ok(Self::Crate(::std::boxed::Box::new(
                ::treesitter_types::runtime::maybe_grow_stack(|| {
                    <Crate as ::treesitter_types::FromNode>::from_node(node, src)
                })?,
            ))),
            "identifier" => Ok(Self::Identifier(::std::boxed::Box::new(
                ::treesitter_types::runtime::maybe_grow_stack(|| {
                    <Identifier as ::treesitter_types::FromNode>::from_node(node, src)
                })?,
            ))),
            "metavariable" => Ok(Self::Metavariable(::std::boxed::Box::new(
                ::treesitter_types::runtime::maybe_grow_stack(|| {
                    <Metavariable as ::treesitter_types::FromNode>::from_node(node, src)
                })?,
            ))),
            "mutable_specifier" => Ok(Self::MutableSpecifier(::std::boxed::Box::new(
                ::treesitter_types::runtime::maybe_grow_stack(|| {
                    <MutableSpecifier as ::treesitter_types::FromNode>::from_node(node, src)
                })?,
            ))),
            "primitive_type" => Ok(Self::PrimitiveType(::std::boxed::Box::new(
                ::treesitter_types::runtime::maybe_grow_stack(|| {
                    <PrimitiveType as ::treesitter_types::FromNode>::from_node(node, src)
                })?,
            ))),
            "self" => Ok(Self::SelfType(::std::boxed::Box::new(
                ::treesitter_types::runtime::maybe_grow_stack(|| {
                    <SelfType as ::treesitter_types::FromNode>::from_node(node, src)
                })?,
            ))),
            "super" => Ok(Self::Super(::std::boxed::Box::new(
                ::treesitter_types::runtime::maybe_grow_stack(|| {
                    <Super as ::treesitter_types::FromNode>::from_node(node, src)
                })?,
            ))),
            "token_repetition" => Ok(Self::TokenRepetition(::std::boxed::Box::new(
                ::treesitter_types::runtime::maybe_grow_stack(|| {
                    <TokenRepetition as ::treesitter_types::FromNode>::from_node(node, src)
                })?,
            ))),
            "token_tree" => Ok(Self::TokenTree(::std::boxed::Box::new(
                ::treesitter_types::runtime::maybe_grow_stack(|| {
                    <TokenTree as ::treesitter_types::FromNode>::from_node(node, src)
                })?,
            ))),
            _other => {
                if let Ok(v) = ::treesitter_types::runtime::maybe_grow_stack(|| {
                    <Literal as ::treesitter_types::FromNode>::from_node(node, src)
                }) {
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
impl ::treesitter_types::Spanned for TokenTreeChildren<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        match self {
            Self::Literal(inner) => inner.span(),
            Self::Crate(inner) => inner.span(),
            Self::Identifier(inner) => inner.span(),
            Self::Metavariable(inner) => inner.span(),
            Self::MutableSpecifier(inner) => inner.span(),
            Self::PrimitiveType(inner) => inner.span(),
            Self::SelfType(inner) => inner.span(),
            Self::Super(inner) => inner.span(),
            Self::TokenRepetition(inner) => inner.span(),
            Self::TokenTree(inner) => inner.span(),
        }
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum TokenTreePatternChildren<'tree> {
    Literal(::std::boxed::Box<Literal<'tree>>),
    Crate(::std::boxed::Box<Crate<'tree>>),
    Identifier(::std::boxed::Box<Identifier<'tree>>),
    Metavariable(::std::boxed::Box<Metavariable<'tree>>),
    MutableSpecifier(::std::boxed::Box<MutableSpecifier<'tree>>),
    PrimitiveType(::std::boxed::Box<PrimitiveType<'tree>>),
    SelfType(::std::boxed::Box<SelfType<'tree>>),
    Super(::std::boxed::Box<Super<'tree>>),
    TokenBindingPattern(::std::boxed::Box<TokenBindingPattern<'tree>>),
    TokenRepetitionPattern(::std::boxed::Box<TokenRepetitionPattern<'tree>>),
    TokenTreePattern(::std::boxed::Box<TokenTreePattern<'tree>>),
}
impl<'tree> ::treesitter_types::FromNode<'tree> for TokenTreePatternChildren<'tree> {
    #[allow(clippy::collapsible_else_if)]
    fn from_node(
        node: ::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        match node.kind() {
            "crate" => Ok(Self::Crate(::std::boxed::Box::new(
                ::treesitter_types::runtime::maybe_grow_stack(|| {
                    <Crate as ::treesitter_types::FromNode>::from_node(node, src)
                })?,
            ))),
            "identifier" => Ok(Self::Identifier(::std::boxed::Box::new(
                ::treesitter_types::runtime::maybe_grow_stack(|| {
                    <Identifier as ::treesitter_types::FromNode>::from_node(node, src)
                })?,
            ))),
            "metavariable" => Ok(Self::Metavariable(::std::boxed::Box::new(
                ::treesitter_types::runtime::maybe_grow_stack(|| {
                    <Metavariable as ::treesitter_types::FromNode>::from_node(node, src)
                })?,
            ))),
            "mutable_specifier" => Ok(Self::MutableSpecifier(::std::boxed::Box::new(
                ::treesitter_types::runtime::maybe_grow_stack(|| {
                    <MutableSpecifier as ::treesitter_types::FromNode>::from_node(node, src)
                })?,
            ))),
            "primitive_type" => Ok(Self::PrimitiveType(::std::boxed::Box::new(
                ::treesitter_types::runtime::maybe_grow_stack(|| {
                    <PrimitiveType as ::treesitter_types::FromNode>::from_node(node, src)
                })?,
            ))),
            "self" => Ok(Self::SelfType(::std::boxed::Box::new(
                ::treesitter_types::runtime::maybe_grow_stack(|| {
                    <SelfType as ::treesitter_types::FromNode>::from_node(node, src)
                })?,
            ))),
            "super" => Ok(Self::Super(::std::boxed::Box::new(
                ::treesitter_types::runtime::maybe_grow_stack(|| {
                    <Super as ::treesitter_types::FromNode>::from_node(node, src)
                })?,
            ))),
            "token_binding_pattern" => Ok(Self::TokenBindingPattern(::std::boxed::Box::new(
                ::treesitter_types::runtime::maybe_grow_stack(|| {
                    <TokenBindingPattern as ::treesitter_types::FromNode>::from_node(node, src)
                })?,
            ))),
            "token_repetition_pattern" => Ok(Self::TokenRepetitionPattern(::std::boxed::Box::new(
                ::treesitter_types::runtime::maybe_grow_stack(|| {
                    <TokenRepetitionPattern as ::treesitter_types::FromNode>::from_node(node, src)
                })?,
            ))),
            "token_tree_pattern" => Ok(Self::TokenTreePattern(::std::boxed::Box::new(
                ::treesitter_types::runtime::maybe_grow_stack(|| {
                    <TokenTreePattern as ::treesitter_types::FromNode>::from_node(node, src)
                })?,
            ))),
            _other => {
                if let Ok(v) = ::treesitter_types::runtime::maybe_grow_stack(|| {
                    <Literal as ::treesitter_types::FromNode>::from_node(node, src)
                }) {
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
impl ::treesitter_types::Spanned for TokenTreePatternChildren<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        match self {
            Self::Literal(inner) => inner.span(),
            Self::Crate(inner) => inner.span(),
            Self::Identifier(inner) => inner.span(),
            Self::Metavariable(inner) => inner.span(),
            Self::MutableSpecifier(inner) => inner.span(),
            Self::PrimitiveType(inner) => inner.span(),
            Self::SelfType(inner) => inner.span(),
            Self::Super(inner) => inner.span(),
            Self::TokenBindingPattern(inner) => inner.span(),
            Self::TokenRepetitionPattern(inner) => inner.span(),
            Self::TokenTreePattern(inner) => inner.span(),
        }
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum TraitBoundsChildren<'tree> {
    Type(::std::boxed::Box<Type<'tree>>),
    HigherRankedTraitBound(::std::boxed::Box<HigherRankedTraitBound<'tree>>),
    Lifetime(::std::boxed::Box<Lifetime<'tree>>),
}
impl<'tree> ::treesitter_types::FromNode<'tree> for TraitBoundsChildren<'tree> {
    #[allow(clippy::collapsible_else_if)]
    fn from_node(
        node: ::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        match node.kind() {
            "higher_ranked_trait_bound" => Ok(Self::HigherRankedTraitBound(
                ::std::boxed::Box::new(::treesitter_types::runtime::maybe_grow_stack(|| {
                    <HigherRankedTraitBound as ::treesitter_types::FromNode>::from_node(node, src)
                })?),
            )),
            "lifetime" => Ok(Self::Lifetime(::std::boxed::Box::new(
                ::treesitter_types::runtime::maybe_grow_stack(|| {
                    <Lifetime as ::treesitter_types::FromNode>::from_node(node, src)
                })?,
            ))),
            _other => {
                if let Ok(v) = ::treesitter_types::runtime::maybe_grow_stack(|| {
                    <Type as ::treesitter_types::FromNode>::from_node(node, src)
                }) {
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
impl ::treesitter_types::Spanned for TraitBoundsChildren<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        match self {
            Self::Type(inner) => inner.span(),
            Self::HigherRankedTraitBound(inner) => inner.span(),
            Self::Lifetime(inner) => inner.span(),
        }
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum TraitItemChildren<'tree> {
    VisibilityModifier(::std::boxed::Box<VisibilityModifier<'tree>>),
    WhereClause(::std::boxed::Box<WhereClause<'tree>>),
}
impl<'tree> ::treesitter_types::FromNode<'tree> for TraitItemChildren<'tree> {
    #[allow(clippy::collapsible_else_if)]
    fn from_node(
        node: ::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        match node.kind() {
            "visibility_modifier" => Ok(Self::VisibilityModifier(::std::boxed::Box::new(
                ::treesitter_types::runtime::maybe_grow_stack(|| {
                    <VisibilityModifier as ::treesitter_types::FromNode>::from_node(node, src)
                })?,
            ))),
            "where_clause" => Ok(Self::WhereClause(::std::boxed::Box::new(
                ::treesitter_types::runtime::maybe_grow_stack(|| {
                    <WhereClause as ::treesitter_types::FromNode>::from_node(node, src)
                })?,
            ))),
            other => Err(::treesitter_types::ParseError::unexpected_kind(other, node)),
        }
    }
}
impl ::treesitter_types::Spanned for TraitItemChildren<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        match self {
            Self::VisibilityModifier(inner) => inner.span(),
            Self::WhereClause(inner) => inner.span(),
        }
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum TupleExpressionChildren<'tree> {
    Expression(::std::boxed::Box<Expression<'tree>>),
    AttributeItem(::std::boxed::Box<AttributeItem<'tree>>),
}
impl<'tree> ::treesitter_types::FromNode<'tree> for TupleExpressionChildren<'tree> {
    #[allow(clippy::collapsible_else_if)]
    fn from_node(
        node: ::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        match node.kind() {
            "attribute_item" => Ok(Self::AttributeItem(::std::boxed::Box::new(
                ::treesitter_types::runtime::maybe_grow_stack(|| {
                    <AttributeItem as ::treesitter_types::FromNode>::from_node(node, src)
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
impl ::treesitter_types::Spanned for TupleExpressionChildren<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        match self {
            Self::Expression(inner) => inner.span(),
            Self::AttributeItem(inner) => inner.span(),
        }
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum TuplePatternChildren<'tree> {
    Pattern(::std::boxed::Box<Pattern<'tree>>),
    ClosureExpression(::std::boxed::Box<ClosureExpression<'tree>>),
}
impl<'tree> ::treesitter_types::FromNode<'tree> for TuplePatternChildren<'tree> {
    #[allow(clippy::collapsible_else_if)]
    fn from_node(
        node: ::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        match node.kind() {
            "closure_expression" => Ok(Self::ClosureExpression(::std::boxed::Box::new(
                ::treesitter_types::runtime::maybe_grow_stack(|| {
                    <ClosureExpression as ::treesitter_types::FromNode>::from_node(node, src)
                })?,
            ))),
            _other => {
                if let Ok(v) = ::treesitter_types::runtime::maybe_grow_stack(|| {
                    <Pattern as ::treesitter_types::FromNode>::from_node(node, src)
                }) {
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
            Self::Pattern(inner) => inner.span(),
            Self::ClosureExpression(inner) => inner.span(),
        }
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum TupleStructPatternType<'tree> {
    GenericType(::std::boxed::Box<GenericType<'tree>>),
    Identifier(::std::boxed::Box<Identifier<'tree>>),
    ScopedIdentifier(::std::boxed::Box<ScopedIdentifier<'tree>>),
}
impl<'tree> ::treesitter_types::FromNode<'tree> for TupleStructPatternType<'tree> {
    #[allow(clippy::collapsible_else_if)]
    fn from_node(
        node: ::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        match node.kind() {
            "generic_type" => Ok(Self::GenericType(::std::boxed::Box::new(
                ::treesitter_types::runtime::maybe_grow_stack(|| {
                    <GenericType as ::treesitter_types::FromNode>::from_node(node, src)
                })?,
            ))),
            "identifier" => Ok(Self::Identifier(::std::boxed::Box::new(
                ::treesitter_types::runtime::maybe_grow_stack(|| {
                    <Identifier as ::treesitter_types::FromNode>::from_node(node, src)
                })?,
            ))),
            "scoped_identifier" => Ok(Self::ScopedIdentifier(::std::boxed::Box::new(
                ::treesitter_types::runtime::maybe_grow_stack(|| {
                    <ScopedIdentifier as ::treesitter_types::FromNode>::from_node(node, src)
                })?,
            ))),
            other => Err(::treesitter_types::ParseError::unexpected_kind(other, node)),
        }
    }
}
impl ::treesitter_types::Spanned for TupleStructPatternType<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        match self {
            Self::GenericType(inner) => inner.span(),
            Self::Identifier(inner) => inner.span(),
            Self::ScopedIdentifier(inner) => inner.span(),
        }
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum TypeArgumentsChildren<'tree> {
    Literal(::std::boxed::Box<Literal<'tree>>),
    Type(::std::boxed::Box<Type<'tree>>),
    Block(::std::boxed::Box<Block<'tree>>),
    Lifetime(::std::boxed::Box<Lifetime<'tree>>),
    TraitBounds(::std::boxed::Box<TraitBounds<'tree>>),
    TypeBinding(::std::boxed::Box<TypeBinding<'tree>>),
}
impl<'tree> ::treesitter_types::FromNode<'tree> for TypeArgumentsChildren<'tree> {
    #[allow(clippy::collapsible_else_if)]
    fn from_node(
        node: ::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        match node.kind() {
            "block" => Ok(Self::Block(::std::boxed::Box::new(
                ::treesitter_types::runtime::maybe_grow_stack(|| {
                    <Block as ::treesitter_types::FromNode>::from_node(node, src)
                })?,
            ))),
            "lifetime" => Ok(Self::Lifetime(::std::boxed::Box::new(
                ::treesitter_types::runtime::maybe_grow_stack(|| {
                    <Lifetime as ::treesitter_types::FromNode>::from_node(node, src)
                })?,
            ))),
            "trait_bounds" => Ok(Self::TraitBounds(::std::boxed::Box::new(
                ::treesitter_types::runtime::maybe_grow_stack(|| {
                    <TraitBounds as ::treesitter_types::FromNode>::from_node(node, src)
                })?,
            ))),
            "type_binding" => Ok(Self::TypeBinding(::std::boxed::Box::new(
                ::treesitter_types::runtime::maybe_grow_stack(|| {
                    <TypeBinding as ::treesitter_types::FromNode>::from_node(node, src)
                })?,
            ))),
            _other => {
                if let Ok(v) = ::treesitter_types::runtime::maybe_grow_stack(|| {
                    <Literal as ::treesitter_types::FromNode>::from_node(node, src)
                }) {
                    Ok(Self::Literal(::std::boxed::Box::new(v)))
                } else {
                    if let Ok(v) = ::treesitter_types::runtime::maybe_grow_stack(|| {
                        <Type as ::treesitter_types::FromNode>::from_node(node, src)
                    }) {
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
}
impl ::treesitter_types::Spanned for TypeArgumentsChildren<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        match self {
            Self::Literal(inner) => inner.span(),
            Self::Type(inner) => inner.span(),
            Self::Block(inner) => inner.span(),
            Self::Lifetime(inner) => inner.span(),
            Self::TraitBounds(inner) => inner.span(),
            Self::TypeBinding(inner) => inner.span(),
        }
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum TypeItemChildren<'tree> {
    VisibilityModifier(::std::boxed::Box<VisibilityModifier<'tree>>),
    WhereClause(::std::boxed::Box<WhereClause<'tree>>),
}
impl<'tree> ::treesitter_types::FromNode<'tree> for TypeItemChildren<'tree> {
    #[allow(clippy::collapsible_else_if)]
    fn from_node(
        node: ::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        match node.kind() {
            "visibility_modifier" => Ok(Self::VisibilityModifier(::std::boxed::Box::new(
                ::treesitter_types::runtime::maybe_grow_stack(|| {
                    <VisibilityModifier as ::treesitter_types::FromNode>::from_node(node, src)
                })?,
            ))),
            "where_clause" => Ok(Self::WhereClause(::std::boxed::Box::new(
                ::treesitter_types::runtime::maybe_grow_stack(|| {
                    <WhereClause as ::treesitter_types::FromNode>::from_node(node, src)
                })?,
            ))),
            other => Err(::treesitter_types::ParseError::unexpected_kind(other, node)),
        }
    }
}
impl ::treesitter_types::Spanned for TypeItemChildren<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        match self {
            Self::VisibilityModifier(inner) => inner.span(),
            Self::WhereClause(inner) => inner.span(),
        }
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum TypeParametersChildren<'tree> {
    AttributeItem(::std::boxed::Box<AttributeItem<'tree>>),
    ConstParameter(::std::boxed::Box<ConstParameter<'tree>>),
    LifetimeParameter(::std::boxed::Box<LifetimeParameter<'tree>>),
    Metavariable(::std::boxed::Box<Metavariable<'tree>>),
    TypeParameter(::std::boxed::Box<TypeParameter<'tree>>),
}
impl<'tree> ::treesitter_types::FromNode<'tree> for TypeParametersChildren<'tree> {
    #[allow(clippy::collapsible_else_if)]
    fn from_node(
        node: ::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        match node.kind() {
            "attribute_item" => Ok(Self::AttributeItem(::std::boxed::Box::new(
                ::treesitter_types::runtime::maybe_grow_stack(|| {
                    <AttributeItem as ::treesitter_types::FromNode>::from_node(node, src)
                })?,
            ))),
            "const_parameter" => Ok(Self::ConstParameter(::std::boxed::Box::new(
                ::treesitter_types::runtime::maybe_grow_stack(|| {
                    <ConstParameter as ::treesitter_types::FromNode>::from_node(node, src)
                })?,
            ))),
            "lifetime_parameter" => Ok(Self::LifetimeParameter(::std::boxed::Box::new(
                ::treesitter_types::runtime::maybe_grow_stack(|| {
                    <LifetimeParameter as ::treesitter_types::FromNode>::from_node(node, src)
                })?,
            ))),
            "metavariable" => Ok(Self::Metavariable(::std::boxed::Box::new(
                ::treesitter_types::runtime::maybe_grow_stack(|| {
                    <Metavariable as ::treesitter_types::FromNode>::from_node(node, src)
                })?,
            ))),
            "type_parameter" => Ok(Self::TypeParameter(::std::boxed::Box::new(
                ::treesitter_types::runtime::maybe_grow_stack(|| {
                    <TypeParameter as ::treesitter_types::FromNode>::from_node(node, src)
                })?,
            ))),
            other => Err(::treesitter_types::ParseError::unexpected_kind(other, node)),
        }
    }
}
impl ::treesitter_types::Spanned for TypeParametersChildren<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        match self {
            Self::AttributeItem(inner) => inner.span(),
            Self::ConstParameter(inner) => inner.span(),
            Self::LifetimeParameter(inner) => inner.span(),
            Self::Metavariable(inner) => inner.span(),
            Self::TypeParameter(inner) => inner.span(),
        }
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum UnionItemChildren<'tree> {
    VisibilityModifier(::std::boxed::Box<VisibilityModifier<'tree>>),
    WhereClause(::std::boxed::Box<WhereClause<'tree>>),
}
impl<'tree> ::treesitter_types::FromNode<'tree> for UnionItemChildren<'tree> {
    #[allow(clippy::collapsible_else_if)]
    fn from_node(
        node: ::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        match node.kind() {
            "visibility_modifier" => Ok(Self::VisibilityModifier(::std::boxed::Box::new(
                ::treesitter_types::runtime::maybe_grow_stack(|| {
                    <VisibilityModifier as ::treesitter_types::FromNode>::from_node(node, src)
                })?,
            ))),
            "where_clause" => Ok(Self::WhereClause(::std::boxed::Box::new(
                ::treesitter_types::runtime::maybe_grow_stack(|| {
                    <WhereClause as ::treesitter_types::FromNode>::from_node(node, src)
                })?,
            ))),
            other => Err(::treesitter_types::ParseError::unexpected_kind(other, node)),
        }
    }
}
impl ::treesitter_types::Spanned for UnionItemChildren<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        match self {
            Self::VisibilityModifier(inner) => inner.span(),
            Self::WhereClause(inner) => inner.span(),
        }
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum UseAsClausePath<'tree> {
    Crate(::std::boxed::Box<Crate<'tree>>),
    Identifier(::std::boxed::Box<Identifier<'tree>>),
    Metavariable(::std::boxed::Box<Metavariable<'tree>>),
    ScopedIdentifier(::std::boxed::Box<ScopedIdentifier<'tree>>),
    SelfType(::std::boxed::Box<SelfType<'tree>>),
    Super(::std::boxed::Box<Super<'tree>>),
}
impl<'tree> ::treesitter_types::FromNode<'tree> for UseAsClausePath<'tree> {
    #[allow(clippy::collapsible_else_if)]
    fn from_node(
        node: ::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        match node.kind() {
            "crate" => Ok(Self::Crate(::std::boxed::Box::new(
                ::treesitter_types::runtime::maybe_grow_stack(|| {
                    <Crate as ::treesitter_types::FromNode>::from_node(node, src)
                })?,
            ))),
            "identifier" => Ok(Self::Identifier(::std::boxed::Box::new(
                ::treesitter_types::runtime::maybe_grow_stack(|| {
                    <Identifier as ::treesitter_types::FromNode>::from_node(node, src)
                })?,
            ))),
            "metavariable" => Ok(Self::Metavariable(::std::boxed::Box::new(
                ::treesitter_types::runtime::maybe_grow_stack(|| {
                    <Metavariable as ::treesitter_types::FromNode>::from_node(node, src)
                })?,
            ))),
            "scoped_identifier" => Ok(Self::ScopedIdentifier(::std::boxed::Box::new(
                ::treesitter_types::runtime::maybe_grow_stack(|| {
                    <ScopedIdentifier as ::treesitter_types::FromNode>::from_node(node, src)
                })?,
            ))),
            "self" => Ok(Self::SelfType(::std::boxed::Box::new(
                ::treesitter_types::runtime::maybe_grow_stack(|| {
                    <SelfType as ::treesitter_types::FromNode>::from_node(node, src)
                })?,
            ))),
            "super" => Ok(Self::Super(::std::boxed::Box::new(
                ::treesitter_types::runtime::maybe_grow_stack(|| {
                    <Super as ::treesitter_types::FromNode>::from_node(node, src)
                })?,
            ))),
            other => Err(::treesitter_types::ParseError::unexpected_kind(other, node)),
        }
    }
}
impl ::treesitter_types::Spanned for UseAsClausePath<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        match self {
            Self::Crate(inner) => inner.span(),
            Self::Identifier(inner) => inner.span(),
            Self::Metavariable(inner) => inner.span(),
            Self::ScopedIdentifier(inner) => inner.span(),
            Self::SelfType(inner) => inner.span(),
            Self::Super(inner) => inner.span(),
        }
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum UseBoundsChildren<'tree> {
    Lifetime(::std::boxed::Box<Lifetime<'tree>>),
    TypeIdentifier(::std::boxed::Box<TypeIdentifier<'tree>>),
}
impl<'tree> ::treesitter_types::FromNode<'tree> for UseBoundsChildren<'tree> {
    #[allow(clippy::collapsible_else_if)]
    fn from_node(
        node: ::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        match node.kind() {
            "lifetime" => Ok(Self::Lifetime(::std::boxed::Box::new(
                ::treesitter_types::runtime::maybe_grow_stack(|| {
                    <Lifetime as ::treesitter_types::FromNode>::from_node(node, src)
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
impl ::treesitter_types::Spanned for UseBoundsChildren<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        match self {
            Self::Lifetime(inner) => inner.span(),
            Self::TypeIdentifier(inner) => inner.span(),
        }
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum UseDeclarationArgument<'tree> {
    Crate(::std::boxed::Box<Crate<'tree>>),
    Identifier(::std::boxed::Box<Identifier<'tree>>),
    Metavariable(::std::boxed::Box<Metavariable<'tree>>),
    ScopedIdentifier(::std::boxed::Box<ScopedIdentifier<'tree>>),
    ScopedUseList(::std::boxed::Box<ScopedUseList<'tree>>),
    SelfType(::std::boxed::Box<SelfType<'tree>>),
    Super(::std::boxed::Box<Super<'tree>>),
    UseAsClause(::std::boxed::Box<UseAsClause<'tree>>),
    UseList(::std::boxed::Box<UseList<'tree>>),
    UseWildcard(::std::boxed::Box<UseWildcard<'tree>>),
}
impl<'tree> ::treesitter_types::FromNode<'tree> for UseDeclarationArgument<'tree> {
    #[allow(clippy::collapsible_else_if)]
    fn from_node(
        node: ::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        match node.kind() {
            "crate" => Ok(Self::Crate(::std::boxed::Box::new(
                ::treesitter_types::runtime::maybe_grow_stack(|| {
                    <Crate as ::treesitter_types::FromNode>::from_node(node, src)
                })?,
            ))),
            "identifier" => Ok(Self::Identifier(::std::boxed::Box::new(
                ::treesitter_types::runtime::maybe_grow_stack(|| {
                    <Identifier as ::treesitter_types::FromNode>::from_node(node, src)
                })?,
            ))),
            "metavariable" => Ok(Self::Metavariable(::std::boxed::Box::new(
                ::treesitter_types::runtime::maybe_grow_stack(|| {
                    <Metavariable as ::treesitter_types::FromNode>::from_node(node, src)
                })?,
            ))),
            "scoped_identifier" => Ok(Self::ScopedIdentifier(::std::boxed::Box::new(
                ::treesitter_types::runtime::maybe_grow_stack(|| {
                    <ScopedIdentifier as ::treesitter_types::FromNode>::from_node(node, src)
                })?,
            ))),
            "scoped_use_list" => Ok(Self::ScopedUseList(::std::boxed::Box::new(
                ::treesitter_types::runtime::maybe_grow_stack(|| {
                    <ScopedUseList as ::treesitter_types::FromNode>::from_node(node, src)
                })?,
            ))),
            "self" => Ok(Self::SelfType(::std::boxed::Box::new(
                ::treesitter_types::runtime::maybe_grow_stack(|| {
                    <SelfType as ::treesitter_types::FromNode>::from_node(node, src)
                })?,
            ))),
            "super" => Ok(Self::Super(::std::boxed::Box::new(
                ::treesitter_types::runtime::maybe_grow_stack(|| {
                    <Super as ::treesitter_types::FromNode>::from_node(node, src)
                })?,
            ))),
            "use_as_clause" => Ok(Self::UseAsClause(::std::boxed::Box::new(
                ::treesitter_types::runtime::maybe_grow_stack(|| {
                    <UseAsClause as ::treesitter_types::FromNode>::from_node(node, src)
                })?,
            ))),
            "use_list" => Ok(Self::UseList(::std::boxed::Box::new(
                ::treesitter_types::runtime::maybe_grow_stack(|| {
                    <UseList as ::treesitter_types::FromNode>::from_node(node, src)
                })?,
            ))),
            "use_wildcard" => Ok(Self::UseWildcard(::std::boxed::Box::new(
                ::treesitter_types::runtime::maybe_grow_stack(|| {
                    <UseWildcard as ::treesitter_types::FromNode>::from_node(node, src)
                })?,
            ))),
            other => Err(::treesitter_types::ParseError::unexpected_kind(other, node)),
        }
    }
}
impl ::treesitter_types::Spanned for UseDeclarationArgument<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        match self {
            Self::Crate(inner) => inner.span(),
            Self::Identifier(inner) => inner.span(),
            Self::Metavariable(inner) => inner.span(),
            Self::ScopedIdentifier(inner) => inner.span(),
            Self::ScopedUseList(inner) => inner.span(),
            Self::SelfType(inner) => inner.span(),
            Self::Super(inner) => inner.span(),
            Self::UseAsClause(inner) => inner.span(),
            Self::UseList(inner) => inner.span(),
            Self::UseWildcard(inner) => inner.span(),
        }
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum UseListChildren<'tree> {
    Crate(::std::boxed::Box<Crate<'tree>>),
    Identifier(::std::boxed::Box<Identifier<'tree>>),
    Metavariable(::std::boxed::Box<Metavariable<'tree>>),
    ScopedIdentifier(::std::boxed::Box<ScopedIdentifier<'tree>>),
    ScopedUseList(::std::boxed::Box<ScopedUseList<'tree>>),
    SelfType(::std::boxed::Box<SelfType<'tree>>),
    Super(::std::boxed::Box<Super<'tree>>),
    UseAsClause(::std::boxed::Box<UseAsClause<'tree>>),
    UseList(::std::boxed::Box<UseList<'tree>>),
    UseWildcard(::std::boxed::Box<UseWildcard<'tree>>),
}
impl<'tree> ::treesitter_types::FromNode<'tree> for UseListChildren<'tree> {
    #[allow(clippy::collapsible_else_if)]
    fn from_node(
        node: ::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        match node.kind() {
            "crate" => Ok(Self::Crate(::std::boxed::Box::new(
                ::treesitter_types::runtime::maybe_grow_stack(|| {
                    <Crate as ::treesitter_types::FromNode>::from_node(node, src)
                })?,
            ))),
            "identifier" => Ok(Self::Identifier(::std::boxed::Box::new(
                ::treesitter_types::runtime::maybe_grow_stack(|| {
                    <Identifier as ::treesitter_types::FromNode>::from_node(node, src)
                })?,
            ))),
            "metavariable" => Ok(Self::Metavariable(::std::boxed::Box::new(
                ::treesitter_types::runtime::maybe_grow_stack(|| {
                    <Metavariable as ::treesitter_types::FromNode>::from_node(node, src)
                })?,
            ))),
            "scoped_identifier" => Ok(Self::ScopedIdentifier(::std::boxed::Box::new(
                ::treesitter_types::runtime::maybe_grow_stack(|| {
                    <ScopedIdentifier as ::treesitter_types::FromNode>::from_node(node, src)
                })?,
            ))),
            "scoped_use_list" => Ok(Self::ScopedUseList(::std::boxed::Box::new(
                ::treesitter_types::runtime::maybe_grow_stack(|| {
                    <ScopedUseList as ::treesitter_types::FromNode>::from_node(node, src)
                })?,
            ))),
            "self" => Ok(Self::SelfType(::std::boxed::Box::new(
                ::treesitter_types::runtime::maybe_grow_stack(|| {
                    <SelfType as ::treesitter_types::FromNode>::from_node(node, src)
                })?,
            ))),
            "super" => Ok(Self::Super(::std::boxed::Box::new(
                ::treesitter_types::runtime::maybe_grow_stack(|| {
                    <Super as ::treesitter_types::FromNode>::from_node(node, src)
                })?,
            ))),
            "use_as_clause" => Ok(Self::UseAsClause(::std::boxed::Box::new(
                ::treesitter_types::runtime::maybe_grow_stack(|| {
                    <UseAsClause as ::treesitter_types::FromNode>::from_node(node, src)
                })?,
            ))),
            "use_list" => Ok(Self::UseList(::std::boxed::Box::new(
                ::treesitter_types::runtime::maybe_grow_stack(|| {
                    <UseList as ::treesitter_types::FromNode>::from_node(node, src)
                })?,
            ))),
            "use_wildcard" => Ok(Self::UseWildcard(::std::boxed::Box::new(
                ::treesitter_types::runtime::maybe_grow_stack(|| {
                    <UseWildcard as ::treesitter_types::FromNode>::from_node(node, src)
                })?,
            ))),
            other => Err(::treesitter_types::ParseError::unexpected_kind(other, node)),
        }
    }
}
impl ::treesitter_types::Spanned for UseListChildren<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        match self {
            Self::Crate(inner) => inner.span(),
            Self::Identifier(inner) => inner.span(),
            Self::Metavariable(inner) => inner.span(),
            Self::ScopedIdentifier(inner) => inner.span(),
            Self::ScopedUseList(inner) => inner.span(),
            Self::SelfType(inner) => inner.span(),
            Self::Super(inner) => inner.span(),
            Self::UseAsClause(inner) => inner.span(),
            Self::UseList(inner) => inner.span(),
            Self::UseWildcard(inner) => inner.span(),
        }
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum UseWildcardChildren<'tree> {
    Crate(::std::boxed::Box<Crate<'tree>>),
    Identifier(::std::boxed::Box<Identifier<'tree>>),
    Metavariable(::std::boxed::Box<Metavariable<'tree>>),
    ScopedIdentifier(::std::boxed::Box<ScopedIdentifier<'tree>>),
    SelfType(::std::boxed::Box<SelfType<'tree>>),
    Super(::std::boxed::Box<Super<'tree>>),
}
impl<'tree> ::treesitter_types::FromNode<'tree> for UseWildcardChildren<'tree> {
    #[allow(clippy::collapsible_else_if)]
    fn from_node(
        node: ::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        match node.kind() {
            "crate" => Ok(Self::Crate(::std::boxed::Box::new(
                ::treesitter_types::runtime::maybe_grow_stack(|| {
                    <Crate as ::treesitter_types::FromNode>::from_node(node, src)
                })?,
            ))),
            "identifier" => Ok(Self::Identifier(::std::boxed::Box::new(
                ::treesitter_types::runtime::maybe_grow_stack(|| {
                    <Identifier as ::treesitter_types::FromNode>::from_node(node, src)
                })?,
            ))),
            "metavariable" => Ok(Self::Metavariable(::std::boxed::Box::new(
                ::treesitter_types::runtime::maybe_grow_stack(|| {
                    <Metavariable as ::treesitter_types::FromNode>::from_node(node, src)
                })?,
            ))),
            "scoped_identifier" => Ok(Self::ScopedIdentifier(::std::boxed::Box::new(
                ::treesitter_types::runtime::maybe_grow_stack(|| {
                    <ScopedIdentifier as ::treesitter_types::FromNode>::from_node(node, src)
                })?,
            ))),
            "self" => Ok(Self::SelfType(::std::boxed::Box::new(
                ::treesitter_types::runtime::maybe_grow_stack(|| {
                    <SelfType as ::treesitter_types::FromNode>::from_node(node, src)
                })?,
            ))),
            "super" => Ok(Self::Super(::std::boxed::Box::new(
                ::treesitter_types::runtime::maybe_grow_stack(|| {
                    <Super as ::treesitter_types::FromNode>::from_node(node, src)
                })?,
            ))),
            other => Err(::treesitter_types::ParseError::unexpected_kind(other, node)),
        }
    }
}
impl ::treesitter_types::Spanned for UseWildcardChildren<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        match self {
            Self::Crate(inner) => inner.span(),
            Self::Identifier(inner) => inner.span(),
            Self::Metavariable(inner) => inner.span(),
            Self::ScopedIdentifier(inner) => inner.span(),
            Self::SelfType(inner) => inner.span(),
            Self::Super(inner) => inner.span(),
        }
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum VisibilityModifierChildren<'tree> {
    Crate(::std::boxed::Box<Crate<'tree>>),
    Identifier(::std::boxed::Box<Identifier<'tree>>),
    Metavariable(::std::boxed::Box<Metavariable<'tree>>),
    ScopedIdentifier(::std::boxed::Box<ScopedIdentifier<'tree>>),
    SelfType(::std::boxed::Box<SelfType<'tree>>),
    Super(::std::boxed::Box<Super<'tree>>),
}
impl<'tree> ::treesitter_types::FromNode<'tree> for VisibilityModifierChildren<'tree> {
    #[allow(clippy::collapsible_else_if)]
    fn from_node(
        node: ::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        match node.kind() {
            "crate" => Ok(Self::Crate(::std::boxed::Box::new(
                ::treesitter_types::runtime::maybe_grow_stack(|| {
                    <Crate as ::treesitter_types::FromNode>::from_node(node, src)
                })?,
            ))),
            "identifier" => Ok(Self::Identifier(::std::boxed::Box::new(
                ::treesitter_types::runtime::maybe_grow_stack(|| {
                    <Identifier as ::treesitter_types::FromNode>::from_node(node, src)
                })?,
            ))),
            "metavariable" => Ok(Self::Metavariable(::std::boxed::Box::new(
                ::treesitter_types::runtime::maybe_grow_stack(|| {
                    <Metavariable as ::treesitter_types::FromNode>::from_node(node, src)
                })?,
            ))),
            "scoped_identifier" => Ok(Self::ScopedIdentifier(::std::boxed::Box::new(
                ::treesitter_types::runtime::maybe_grow_stack(|| {
                    <ScopedIdentifier as ::treesitter_types::FromNode>::from_node(node, src)
                })?,
            ))),
            "self" => Ok(Self::SelfType(::std::boxed::Box::new(
                ::treesitter_types::runtime::maybe_grow_stack(|| {
                    <SelfType as ::treesitter_types::FromNode>::from_node(node, src)
                })?,
            ))),
            "super" => Ok(Self::Super(::std::boxed::Box::new(
                ::treesitter_types::runtime::maybe_grow_stack(|| {
                    <Super as ::treesitter_types::FromNode>::from_node(node, src)
                })?,
            ))),
            other => Err(::treesitter_types::ParseError::unexpected_kind(other, node)),
        }
    }
}
impl ::treesitter_types::Spanned for VisibilityModifierChildren<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        match self {
            Self::Crate(inner) => inner.span(),
            Self::Identifier(inner) => inner.span(),
            Self::Metavariable(inner) => inner.span(),
            Self::ScopedIdentifier(inner) => inner.span(),
            Self::SelfType(inner) => inner.span(),
            Self::Super(inner) => inner.span(),
        }
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum WherePredicateLeft<'tree> {
    ArrayType(::std::boxed::Box<ArrayType<'tree>>),
    GenericType(::std::boxed::Box<GenericType<'tree>>),
    HigherRankedTraitBound(::std::boxed::Box<HigherRankedTraitBound<'tree>>),
    Lifetime(::std::boxed::Box<Lifetime<'tree>>),
    PointerType(::std::boxed::Box<PointerType<'tree>>),
    PrimitiveType(::std::boxed::Box<PrimitiveType<'tree>>),
    ReferenceType(::std::boxed::Box<ReferenceType<'tree>>),
    ScopedTypeIdentifier(::std::boxed::Box<ScopedTypeIdentifier<'tree>>),
    TupleType(::std::boxed::Box<TupleType<'tree>>),
    TypeIdentifier(::std::boxed::Box<TypeIdentifier<'tree>>),
}
impl<'tree> ::treesitter_types::FromNode<'tree> for WherePredicateLeft<'tree> {
    #[allow(clippy::collapsible_else_if)]
    fn from_node(
        node: ::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        match node.kind() {
            "array_type" => Ok(Self::ArrayType(::std::boxed::Box::new(
                ::treesitter_types::runtime::maybe_grow_stack(|| {
                    <ArrayType as ::treesitter_types::FromNode>::from_node(node, src)
                })?,
            ))),
            "generic_type" => Ok(Self::GenericType(::std::boxed::Box::new(
                ::treesitter_types::runtime::maybe_grow_stack(|| {
                    <GenericType as ::treesitter_types::FromNode>::from_node(node, src)
                })?,
            ))),
            "higher_ranked_trait_bound" => Ok(Self::HigherRankedTraitBound(
                ::std::boxed::Box::new(::treesitter_types::runtime::maybe_grow_stack(|| {
                    <HigherRankedTraitBound as ::treesitter_types::FromNode>::from_node(node, src)
                })?),
            )),
            "lifetime" => Ok(Self::Lifetime(::std::boxed::Box::new(
                ::treesitter_types::runtime::maybe_grow_stack(|| {
                    <Lifetime as ::treesitter_types::FromNode>::from_node(node, src)
                })?,
            ))),
            "pointer_type" => Ok(Self::PointerType(::std::boxed::Box::new(
                ::treesitter_types::runtime::maybe_grow_stack(|| {
                    <PointerType as ::treesitter_types::FromNode>::from_node(node, src)
                })?,
            ))),
            "primitive_type" => Ok(Self::PrimitiveType(::std::boxed::Box::new(
                ::treesitter_types::runtime::maybe_grow_stack(|| {
                    <PrimitiveType as ::treesitter_types::FromNode>::from_node(node, src)
                })?,
            ))),
            "reference_type" => Ok(Self::ReferenceType(::std::boxed::Box::new(
                ::treesitter_types::runtime::maybe_grow_stack(|| {
                    <ReferenceType as ::treesitter_types::FromNode>::from_node(node, src)
                })?,
            ))),
            "scoped_type_identifier" => Ok(Self::ScopedTypeIdentifier(::std::boxed::Box::new(
                ::treesitter_types::runtime::maybe_grow_stack(|| {
                    <ScopedTypeIdentifier as ::treesitter_types::FromNode>::from_node(node, src)
                })?,
            ))),
            "tuple_type" => Ok(Self::TupleType(::std::boxed::Box::new(
                ::treesitter_types::runtime::maybe_grow_stack(|| {
                    <TupleType as ::treesitter_types::FromNode>::from_node(node, src)
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
impl ::treesitter_types::Spanned for WherePredicateLeft<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        match self {
            Self::ArrayType(inner) => inner.span(),
            Self::GenericType(inner) => inner.span(),
            Self::HigherRankedTraitBound(inner) => inner.span(),
            Self::Lifetime(inner) => inner.span(),
            Self::PointerType(inner) => inner.span(),
            Self::PrimitiveType(inner) => inner.span(),
            Self::ReferenceType(inner) => inner.span(),
            Self::ScopedTypeIdentifier(inner) => inner.span(),
            Self::TupleType(inner) => inner.span(),
            Self::TypeIdentifier(inner) => inner.span(),
        }
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum WhileExpressionCondition<'tree> {
    Expression(::std::boxed::Box<Expression<'tree>>),
    LetChain(::std::boxed::Box<LetChain<'tree>>),
    LetCondition(::std::boxed::Box<LetCondition<'tree>>),
}
impl<'tree> ::treesitter_types::FromNode<'tree> for WhileExpressionCondition<'tree> {
    #[allow(clippy::collapsible_else_if)]
    fn from_node(
        node: ::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        match node.kind() {
            "let_chain" => Ok(Self::LetChain(::std::boxed::Box::new(
                ::treesitter_types::runtime::maybe_grow_stack(|| {
                    <LetChain as ::treesitter_types::FromNode>::from_node(node, src)
                })?,
            ))),
            "let_condition" => Ok(Self::LetCondition(::std::boxed::Box::new(
                ::treesitter_types::runtime::maybe_grow_stack(|| {
                    <LetCondition as ::treesitter_types::FromNode>::from_node(node, src)
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
impl ::treesitter_types::Spanned for WhileExpressionCondition<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        match self {
            Self::Expression(inner) => inner.span(),
            Self::LetChain(inner) => inner.span(),
            Self::LetCondition(inner) => inner.span(),
        }
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum AnyNode<'tree> {
    DeclarationStatement(DeclarationStatement<'tree>),
    Expression(Expression<'tree>),
    Literal(Literal<'tree>),
    LiteralPattern(LiteralPattern<'tree>),
    Pattern(Pattern<'tree>),
    Type(Type<'tree>),
    AbstractType(AbstractType<'tree>),
    Arguments(Arguments<'tree>),
    ArrayExpression(ArrayExpression<'tree>),
    ArrayType(ArrayType<'tree>),
    AssignmentExpression(AssignmentExpression<'tree>),
    AssociatedType(AssociatedType<'tree>),
    AsyncBlock(AsyncBlock<'tree>),
    Attribute(Attribute<'tree>),
    AttributeItem(AttributeItem<'tree>),
    AwaitExpression(AwaitExpression<'tree>),
    BaseFieldInitializer(BaseFieldInitializer<'tree>),
    BinaryExpression(BinaryExpression<'tree>),
    Block(Block<'tree>),
    BlockComment(BlockComment<'tree>),
    BooleanLiteral(BooleanLiteral<'tree>),
    BoundedType(BoundedType<'tree>),
    BracketedType(BracketedType<'tree>),
    BreakExpression(BreakExpression<'tree>),
    CallExpression(CallExpression<'tree>),
    CapturedPattern(CapturedPattern<'tree>),
    ClosureExpression(ClosureExpression<'tree>),
    ClosureParameters(ClosureParameters<'tree>),
    CompoundAssignmentExpr(CompoundAssignmentExpr<'tree>),
    ConstBlock(ConstBlock<'tree>),
    ConstItem(ConstItem<'tree>),
    ConstParameter(ConstParameter<'tree>),
    ContinueExpression(ContinueExpression<'tree>),
    DeclarationList(DeclarationList<'tree>),
    DynamicType(DynamicType<'tree>),
    ElseClause(ElseClause<'tree>),
    EmptyStatement(EmptyStatement<'tree>),
    EnumItem(EnumItem<'tree>),
    EnumVariant(EnumVariant<'tree>),
    EnumVariantList(EnumVariantList<'tree>),
    ExpressionStatement(ExpressionStatement<'tree>),
    ExternCrateDeclaration(ExternCrateDeclaration<'tree>),
    ExternModifier(ExternModifier<'tree>),
    FieldDeclaration(FieldDeclaration<'tree>),
    FieldDeclarationList(FieldDeclarationList<'tree>),
    FieldExpression(FieldExpression<'tree>),
    FieldInitializer(FieldInitializer<'tree>),
    FieldInitializerList(FieldInitializerList<'tree>),
    FieldPattern(FieldPattern<'tree>),
    ForExpression(ForExpression<'tree>),
    ForLifetimes(ForLifetimes<'tree>),
    ForeignModItem(ForeignModItem<'tree>),
    FragmentSpecifier(FragmentSpecifier<'tree>),
    FunctionItem(FunctionItem<'tree>),
    FunctionModifiers(FunctionModifiers<'tree>),
    FunctionSignatureItem(FunctionSignatureItem<'tree>),
    FunctionType(FunctionType<'tree>),
    GenBlock(GenBlock<'tree>),
    GenericFunction(GenericFunction<'tree>),
    GenericPattern(GenericPattern<'tree>),
    GenericType(GenericType<'tree>),
    GenericTypeWithTurbofish(GenericTypeWithTurbofish<'tree>),
    HigherRankedTraitBound(HigherRankedTraitBound<'tree>),
    IfExpression(IfExpression<'tree>),
    ImplItem(ImplItem<'tree>),
    IndexExpression(IndexExpression<'tree>),
    InnerAttributeItem(InnerAttributeItem<'tree>),
    InnerDocCommentMarker(InnerDocCommentMarker<'tree>),
    Label(Label<'tree>),
    LetChain(LetChain<'tree>),
    LetCondition(LetCondition<'tree>),
    LetDeclaration(LetDeclaration<'tree>),
    Lifetime(Lifetime<'tree>),
    LifetimeParameter(LifetimeParameter<'tree>),
    LineComment(LineComment<'tree>),
    LoopExpression(LoopExpression<'tree>),
    MacroDefinition(MacroDefinition<'tree>),
    MacroInvocation(MacroInvocation<'tree>),
    MacroRule(MacroRule<'tree>),
    MatchArm(MatchArm<'tree>),
    MatchBlock(MatchBlock<'tree>),
    MatchExpression(MatchExpression<'tree>),
    MatchPattern(MatchPattern<'tree>),
    ModItem(ModItem<'tree>),
    MutPattern(MutPattern<'tree>),
    NegativeLiteral(NegativeLiteral<'tree>),
    NeverType(NeverType<'tree>),
    OrPattern(OrPattern<'tree>),
    OrderedFieldDeclarationList(OrderedFieldDeclarationList<'tree>),
    OuterDocCommentMarker(OuterDocCommentMarker<'tree>),
    Parameter(Parameter<'tree>),
    Parameters(Parameters<'tree>),
    ParenthesizedExpression(ParenthesizedExpression<'tree>),
    PointerType(PointerType<'tree>),
    QualifiedType(QualifiedType<'tree>),
    RangeExpression(RangeExpression<'tree>),
    RangePattern(RangePattern<'tree>),
    RawStringLiteral(RawStringLiteral<'tree>),
    RefPattern(RefPattern<'tree>),
    ReferenceExpression(ReferenceExpression<'tree>),
    ReferencePattern(ReferencePattern<'tree>),
    ReferenceType(ReferenceType<'tree>),
    RemainingFieldPattern(RemainingFieldPattern<'tree>),
    RemovedTraitBound(RemovedTraitBound<'tree>),
    ReturnExpression(ReturnExpression<'tree>),
    ScopedIdentifier(ScopedIdentifier<'tree>),
    ScopedTypeIdentifier(ScopedTypeIdentifier<'tree>),
    ScopedUseList(ScopedUseList<'tree>),
    SelfParameter(SelfParameter<'tree>),
    ShorthandFieldInitializer(ShorthandFieldInitializer<'tree>),
    SlicePattern(SlicePattern<'tree>),
    SourceFile(SourceFile<'tree>),
    StaticItem(StaticItem<'tree>),
    StringLiteral(StringLiteral<'tree>),
    StructExpression(StructExpression<'tree>),
    StructItem(StructItem<'tree>),
    StructPattern(StructPattern<'tree>),
    TokenBindingPattern(TokenBindingPattern<'tree>),
    TokenRepetition(TokenRepetition<'tree>),
    TokenRepetitionPattern(TokenRepetitionPattern<'tree>),
    TokenTree(TokenTree<'tree>),
    TokenTreePattern(TokenTreePattern<'tree>),
    TraitBounds(TraitBounds<'tree>),
    TraitItem(TraitItem<'tree>),
    TryBlock(TryBlock<'tree>),
    TryExpression(TryExpression<'tree>),
    TupleExpression(TupleExpression<'tree>),
    TuplePattern(TuplePattern<'tree>),
    TupleStructPattern(TupleStructPattern<'tree>),
    TupleType(TupleType<'tree>),
    TypeArguments(TypeArguments<'tree>),
    TypeBinding(TypeBinding<'tree>),
    TypeCastExpression(TypeCastExpression<'tree>),
    TypeItem(TypeItem<'tree>),
    TypeParameter(TypeParameter<'tree>),
    TypeParameters(TypeParameters<'tree>),
    UnaryExpression(UnaryExpression<'tree>),
    UnionItem(UnionItem<'tree>),
    UnitExpression(UnitExpression<'tree>),
    UnitType(UnitType<'tree>),
    UnsafeBlock(UnsafeBlock<'tree>),
    UseAsClause(UseAsClause<'tree>),
    UseBounds(UseBounds<'tree>),
    UseDeclaration(UseDeclaration<'tree>),
    UseList(UseList<'tree>),
    UseWildcard(UseWildcard<'tree>),
    VariadicParameter(VariadicParameter<'tree>),
    VisibilityModifier(VisibilityModifier<'tree>),
    WhereClause(WhereClause<'tree>),
    WherePredicate(WherePredicate<'tree>),
    WhileExpression(WhileExpression<'tree>),
    YieldExpression(YieldExpression<'tree>),
    CharLiteral(CharLiteral<'tree>),
    Crate(Crate<'tree>),
    DocComment(DocComment<'tree>),
    EscapeSequence(EscapeSequence<'tree>),
    FieldIdentifier(FieldIdentifier<'tree>),
    FloatLiteral(FloatLiteral<'tree>),
    Identifier(Identifier<'tree>),
    IntegerLiteral(IntegerLiteral<'tree>),
    Metavariable(Metavariable<'tree>),
    MutableSpecifier(MutableSpecifier<'tree>),
    PrimitiveType(PrimitiveType<'tree>),
    SelfType(SelfType<'tree>),
    Shebang(Shebang<'tree>),
    ShorthandFieldIdentifier(ShorthandFieldIdentifier<'tree>),
    StringContent(StringContent<'tree>),
    Super(Super<'tree>),
    TypeIdentifier(TypeIdentifier<'tree>),
    Unknown(::tree_sitter::Node<'tree>),
}
impl<'tree> AnyNode<'tree> {
    pub fn from_node(node: ::tree_sitter::Node<'tree>, src: &'tree [u8]) -> Self {
        match node.kind() {
            "_declaration_statement" => ::treesitter_types::runtime::maybe_grow_stack(|| {
                <DeclarationStatement as ::treesitter_types::FromNode>::from_node(node, src)
            })
            .map(Self::DeclarationStatement)
            .unwrap_or(Self::Unknown(node)),
            "_expression" => ::treesitter_types::runtime::maybe_grow_stack(|| {
                <Expression as ::treesitter_types::FromNode>::from_node(node, src)
            })
            .map(Self::Expression)
            .unwrap_or(Self::Unknown(node)),
            "_literal" => ::treesitter_types::runtime::maybe_grow_stack(|| {
                <Literal as ::treesitter_types::FromNode>::from_node(node, src)
            })
            .map(Self::Literal)
            .unwrap_or(Self::Unknown(node)),
            "_literal_pattern" => ::treesitter_types::runtime::maybe_grow_stack(|| {
                <LiteralPattern as ::treesitter_types::FromNode>::from_node(node, src)
            })
            .map(Self::LiteralPattern)
            .unwrap_or(Self::Unknown(node)),
            "_pattern" => ::treesitter_types::runtime::maybe_grow_stack(|| {
                <Pattern as ::treesitter_types::FromNode>::from_node(node, src)
            })
            .map(Self::Pattern)
            .unwrap_or(Self::Unknown(node)),
            "_type" => ::treesitter_types::runtime::maybe_grow_stack(|| {
                <Type as ::treesitter_types::FromNode>::from_node(node, src)
            })
            .map(Self::Type)
            .unwrap_or(Self::Unknown(node)),
            "abstract_type" => ::treesitter_types::runtime::maybe_grow_stack(|| {
                <AbstractType as ::treesitter_types::FromNode>::from_node(node, src)
            })
            .map(Self::AbstractType)
            .unwrap_or(Self::Unknown(node)),
            "arguments" => ::treesitter_types::runtime::maybe_grow_stack(|| {
                <Arguments as ::treesitter_types::FromNode>::from_node(node, src)
            })
            .map(Self::Arguments)
            .unwrap_or(Self::Unknown(node)),
            "array_expression" => ::treesitter_types::runtime::maybe_grow_stack(|| {
                <ArrayExpression as ::treesitter_types::FromNode>::from_node(node, src)
            })
            .map(Self::ArrayExpression)
            .unwrap_or(Self::Unknown(node)),
            "array_type" => ::treesitter_types::runtime::maybe_grow_stack(|| {
                <ArrayType as ::treesitter_types::FromNode>::from_node(node, src)
            })
            .map(Self::ArrayType)
            .unwrap_or(Self::Unknown(node)),
            "assignment_expression" => ::treesitter_types::runtime::maybe_grow_stack(|| {
                <AssignmentExpression as ::treesitter_types::FromNode>::from_node(node, src)
            })
            .map(Self::AssignmentExpression)
            .unwrap_or(Self::Unknown(node)),
            "associated_type" => ::treesitter_types::runtime::maybe_grow_stack(|| {
                <AssociatedType as ::treesitter_types::FromNode>::from_node(node, src)
            })
            .map(Self::AssociatedType)
            .unwrap_or(Self::Unknown(node)),
            "async_block" => ::treesitter_types::runtime::maybe_grow_stack(|| {
                <AsyncBlock as ::treesitter_types::FromNode>::from_node(node, src)
            })
            .map(Self::AsyncBlock)
            .unwrap_or(Self::Unknown(node)),
            "attribute" => ::treesitter_types::runtime::maybe_grow_stack(|| {
                <Attribute as ::treesitter_types::FromNode>::from_node(node, src)
            })
            .map(Self::Attribute)
            .unwrap_or(Self::Unknown(node)),
            "attribute_item" => ::treesitter_types::runtime::maybe_grow_stack(|| {
                <AttributeItem as ::treesitter_types::FromNode>::from_node(node, src)
            })
            .map(Self::AttributeItem)
            .unwrap_or(Self::Unknown(node)),
            "await_expression" => ::treesitter_types::runtime::maybe_grow_stack(|| {
                <AwaitExpression as ::treesitter_types::FromNode>::from_node(node, src)
            })
            .map(Self::AwaitExpression)
            .unwrap_or(Self::Unknown(node)),
            "base_field_initializer" => ::treesitter_types::runtime::maybe_grow_stack(|| {
                <BaseFieldInitializer as ::treesitter_types::FromNode>::from_node(node, src)
            })
            .map(Self::BaseFieldInitializer)
            .unwrap_or(Self::Unknown(node)),
            "binary_expression" => ::treesitter_types::runtime::maybe_grow_stack(|| {
                <BinaryExpression as ::treesitter_types::FromNode>::from_node(node, src)
            })
            .map(Self::BinaryExpression)
            .unwrap_or(Self::Unknown(node)),
            "block" => ::treesitter_types::runtime::maybe_grow_stack(|| {
                <Block as ::treesitter_types::FromNode>::from_node(node, src)
            })
            .map(Self::Block)
            .unwrap_or(Self::Unknown(node)),
            "block_comment" => ::treesitter_types::runtime::maybe_grow_stack(|| {
                <BlockComment as ::treesitter_types::FromNode>::from_node(node, src)
            })
            .map(Self::BlockComment)
            .unwrap_or(Self::Unknown(node)),
            "boolean_literal" => ::treesitter_types::runtime::maybe_grow_stack(|| {
                <BooleanLiteral as ::treesitter_types::FromNode>::from_node(node, src)
            })
            .map(Self::BooleanLiteral)
            .unwrap_or(Self::Unknown(node)),
            "bounded_type" => ::treesitter_types::runtime::maybe_grow_stack(|| {
                <BoundedType as ::treesitter_types::FromNode>::from_node(node, src)
            })
            .map(Self::BoundedType)
            .unwrap_or(Self::Unknown(node)),
            "bracketed_type" => ::treesitter_types::runtime::maybe_grow_stack(|| {
                <BracketedType as ::treesitter_types::FromNode>::from_node(node, src)
            })
            .map(Self::BracketedType)
            .unwrap_or(Self::Unknown(node)),
            "break_expression" => ::treesitter_types::runtime::maybe_grow_stack(|| {
                <BreakExpression as ::treesitter_types::FromNode>::from_node(node, src)
            })
            .map(Self::BreakExpression)
            .unwrap_or(Self::Unknown(node)),
            "call_expression" => ::treesitter_types::runtime::maybe_grow_stack(|| {
                <CallExpression as ::treesitter_types::FromNode>::from_node(node, src)
            })
            .map(Self::CallExpression)
            .unwrap_or(Self::Unknown(node)),
            "captured_pattern" => ::treesitter_types::runtime::maybe_grow_stack(|| {
                <CapturedPattern as ::treesitter_types::FromNode>::from_node(node, src)
            })
            .map(Self::CapturedPattern)
            .unwrap_or(Self::Unknown(node)),
            "closure_expression" => ::treesitter_types::runtime::maybe_grow_stack(|| {
                <ClosureExpression as ::treesitter_types::FromNode>::from_node(node, src)
            })
            .map(Self::ClosureExpression)
            .unwrap_or(Self::Unknown(node)),
            "closure_parameters" => ::treesitter_types::runtime::maybe_grow_stack(|| {
                <ClosureParameters as ::treesitter_types::FromNode>::from_node(node, src)
            })
            .map(Self::ClosureParameters)
            .unwrap_or(Self::Unknown(node)),
            "compound_assignment_expr" => ::treesitter_types::runtime::maybe_grow_stack(|| {
                <CompoundAssignmentExpr as ::treesitter_types::FromNode>::from_node(node, src)
            })
            .map(Self::CompoundAssignmentExpr)
            .unwrap_or(Self::Unknown(node)),
            "const_block" => ::treesitter_types::runtime::maybe_grow_stack(|| {
                <ConstBlock as ::treesitter_types::FromNode>::from_node(node, src)
            })
            .map(Self::ConstBlock)
            .unwrap_or(Self::Unknown(node)),
            "const_item" => ::treesitter_types::runtime::maybe_grow_stack(|| {
                <ConstItem as ::treesitter_types::FromNode>::from_node(node, src)
            })
            .map(Self::ConstItem)
            .unwrap_or(Self::Unknown(node)),
            "const_parameter" => ::treesitter_types::runtime::maybe_grow_stack(|| {
                <ConstParameter as ::treesitter_types::FromNode>::from_node(node, src)
            })
            .map(Self::ConstParameter)
            .unwrap_or(Self::Unknown(node)),
            "continue_expression" => ::treesitter_types::runtime::maybe_grow_stack(|| {
                <ContinueExpression as ::treesitter_types::FromNode>::from_node(node, src)
            })
            .map(Self::ContinueExpression)
            .unwrap_or(Self::Unknown(node)),
            "declaration_list" => ::treesitter_types::runtime::maybe_grow_stack(|| {
                <DeclarationList as ::treesitter_types::FromNode>::from_node(node, src)
            })
            .map(Self::DeclarationList)
            .unwrap_or(Self::Unknown(node)),
            "dynamic_type" => ::treesitter_types::runtime::maybe_grow_stack(|| {
                <DynamicType as ::treesitter_types::FromNode>::from_node(node, src)
            })
            .map(Self::DynamicType)
            .unwrap_or(Self::Unknown(node)),
            "else_clause" => ::treesitter_types::runtime::maybe_grow_stack(|| {
                <ElseClause as ::treesitter_types::FromNode>::from_node(node, src)
            })
            .map(Self::ElseClause)
            .unwrap_or(Self::Unknown(node)),
            "empty_statement" => ::treesitter_types::runtime::maybe_grow_stack(|| {
                <EmptyStatement as ::treesitter_types::FromNode>::from_node(node, src)
            })
            .map(Self::EmptyStatement)
            .unwrap_or(Self::Unknown(node)),
            "enum_item" => ::treesitter_types::runtime::maybe_grow_stack(|| {
                <EnumItem as ::treesitter_types::FromNode>::from_node(node, src)
            })
            .map(Self::EnumItem)
            .unwrap_or(Self::Unknown(node)),
            "enum_variant" => ::treesitter_types::runtime::maybe_grow_stack(|| {
                <EnumVariant as ::treesitter_types::FromNode>::from_node(node, src)
            })
            .map(Self::EnumVariant)
            .unwrap_or(Self::Unknown(node)),
            "enum_variant_list" => ::treesitter_types::runtime::maybe_grow_stack(|| {
                <EnumVariantList as ::treesitter_types::FromNode>::from_node(node, src)
            })
            .map(Self::EnumVariantList)
            .unwrap_or(Self::Unknown(node)),
            "expression_statement" => ::treesitter_types::runtime::maybe_grow_stack(|| {
                <ExpressionStatement as ::treesitter_types::FromNode>::from_node(node, src)
            })
            .map(Self::ExpressionStatement)
            .unwrap_or(Self::Unknown(node)),
            "extern_crate_declaration" => ::treesitter_types::runtime::maybe_grow_stack(|| {
                <ExternCrateDeclaration as ::treesitter_types::FromNode>::from_node(node, src)
            })
            .map(Self::ExternCrateDeclaration)
            .unwrap_or(Self::Unknown(node)),
            "extern_modifier" => ::treesitter_types::runtime::maybe_grow_stack(|| {
                <ExternModifier as ::treesitter_types::FromNode>::from_node(node, src)
            })
            .map(Self::ExternModifier)
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
            "field_expression" => ::treesitter_types::runtime::maybe_grow_stack(|| {
                <FieldExpression as ::treesitter_types::FromNode>::from_node(node, src)
            })
            .map(Self::FieldExpression)
            .unwrap_or(Self::Unknown(node)),
            "field_initializer" => ::treesitter_types::runtime::maybe_grow_stack(|| {
                <FieldInitializer as ::treesitter_types::FromNode>::from_node(node, src)
            })
            .map(Self::FieldInitializer)
            .unwrap_or(Self::Unknown(node)),
            "field_initializer_list" => ::treesitter_types::runtime::maybe_grow_stack(|| {
                <FieldInitializerList as ::treesitter_types::FromNode>::from_node(node, src)
            })
            .map(Self::FieldInitializerList)
            .unwrap_or(Self::Unknown(node)),
            "field_pattern" => ::treesitter_types::runtime::maybe_grow_stack(|| {
                <FieldPattern as ::treesitter_types::FromNode>::from_node(node, src)
            })
            .map(Self::FieldPattern)
            .unwrap_or(Self::Unknown(node)),
            "for_expression" => ::treesitter_types::runtime::maybe_grow_stack(|| {
                <ForExpression as ::treesitter_types::FromNode>::from_node(node, src)
            })
            .map(Self::ForExpression)
            .unwrap_or(Self::Unknown(node)),
            "for_lifetimes" => ::treesitter_types::runtime::maybe_grow_stack(|| {
                <ForLifetimes as ::treesitter_types::FromNode>::from_node(node, src)
            })
            .map(Self::ForLifetimes)
            .unwrap_or(Self::Unknown(node)),
            "foreign_mod_item" => ::treesitter_types::runtime::maybe_grow_stack(|| {
                <ForeignModItem as ::treesitter_types::FromNode>::from_node(node, src)
            })
            .map(Self::ForeignModItem)
            .unwrap_or(Self::Unknown(node)),
            "fragment_specifier" => ::treesitter_types::runtime::maybe_grow_stack(|| {
                <FragmentSpecifier as ::treesitter_types::FromNode>::from_node(node, src)
            })
            .map(Self::FragmentSpecifier)
            .unwrap_or(Self::Unknown(node)),
            "function_item" => ::treesitter_types::runtime::maybe_grow_stack(|| {
                <FunctionItem as ::treesitter_types::FromNode>::from_node(node, src)
            })
            .map(Self::FunctionItem)
            .unwrap_or(Self::Unknown(node)),
            "function_modifiers" => ::treesitter_types::runtime::maybe_grow_stack(|| {
                <FunctionModifiers as ::treesitter_types::FromNode>::from_node(node, src)
            })
            .map(Self::FunctionModifiers)
            .unwrap_or(Self::Unknown(node)),
            "function_signature_item" => ::treesitter_types::runtime::maybe_grow_stack(|| {
                <FunctionSignatureItem as ::treesitter_types::FromNode>::from_node(node, src)
            })
            .map(Self::FunctionSignatureItem)
            .unwrap_or(Self::Unknown(node)),
            "function_type" => ::treesitter_types::runtime::maybe_grow_stack(|| {
                <FunctionType as ::treesitter_types::FromNode>::from_node(node, src)
            })
            .map(Self::FunctionType)
            .unwrap_or(Self::Unknown(node)),
            "gen_block" => ::treesitter_types::runtime::maybe_grow_stack(|| {
                <GenBlock as ::treesitter_types::FromNode>::from_node(node, src)
            })
            .map(Self::GenBlock)
            .unwrap_or(Self::Unknown(node)),
            "generic_function" => ::treesitter_types::runtime::maybe_grow_stack(|| {
                <GenericFunction as ::treesitter_types::FromNode>::from_node(node, src)
            })
            .map(Self::GenericFunction)
            .unwrap_or(Self::Unknown(node)),
            "generic_pattern" => ::treesitter_types::runtime::maybe_grow_stack(|| {
                <GenericPattern as ::treesitter_types::FromNode>::from_node(node, src)
            })
            .map(Self::GenericPattern)
            .unwrap_or(Self::Unknown(node)),
            "generic_type" => ::treesitter_types::runtime::maybe_grow_stack(|| {
                <GenericType as ::treesitter_types::FromNode>::from_node(node, src)
            })
            .map(Self::GenericType)
            .unwrap_or(Self::Unknown(node)),
            "generic_type_with_turbofish" => ::treesitter_types::runtime::maybe_grow_stack(|| {
                <GenericTypeWithTurbofish as ::treesitter_types::FromNode>::from_node(node, src)
            })
            .map(Self::GenericTypeWithTurbofish)
            .unwrap_or(Self::Unknown(node)),
            "higher_ranked_trait_bound" => ::treesitter_types::runtime::maybe_grow_stack(|| {
                <HigherRankedTraitBound as ::treesitter_types::FromNode>::from_node(node, src)
            })
            .map(Self::HigherRankedTraitBound)
            .unwrap_or(Self::Unknown(node)),
            "if_expression" => ::treesitter_types::runtime::maybe_grow_stack(|| {
                <IfExpression as ::treesitter_types::FromNode>::from_node(node, src)
            })
            .map(Self::IfExpression)
            .unwrap_or(Self::Unknown(node)),
            "impl_item" => ::treesitter_types::runtime::maybe_grow_stack(|| {
                <ImplItem as ::treesitter_types::FromNode>::from_node(node, src)
            })
            .map(Self::ImplItem)
            .unwrap_or(Self::Unknown(node)),
            "index_expression" => ::treesitter_types::runtime::maybe_grow_stack(|| {
                <IndexExpression as ::treesitter_types::FromNode>::from_node(node, src)
            })
            .map(Self::IndexExpression)
            .unwrap_or(Self::Unknown(node)),
            "inner_attribute_item" => ::treesitter_types::runtime::maybe_grow_stack(|| {
                <InnerAttributeItem as ::treesitter_types::FromNode>::from_node(node, src)
            })
            .map(Self::InnerAttributeItem)
            .unwrap_or(Self::Unknown(node)),
            "inner_doc_comment_marker" => ::treesitter_types::runtime::maybe_grow_stack(|| {
                <InnerDocCommentMarker as ::treesitter_types::FromNode>::from_node(node, src)
            })
            .map(Self::InnerDocCommentMarker)
            .unwrap_or(Self::Unknown(node)),
            "label" => ::treesitter_types::runtime::maybe_grow_stack(|| {
                <Label as ::treesitter_types::FromNode>::from_node(node, src)
            })
            .map(Self::Label)
            .unwrap_or(Self::Unknown(node)),
            "let_chain" => ::treesitter_types::runtime::maybe_grow_stack(|| {
                <LetChain as ::treesitter_types::FromNode>::from_node(node, src)
            })
            .map(Self::LetChain)
            .unwrap_or(Self::Unknown(node)),
            "let_condition" => ::treesitter_types::runtime::maybe_grow_stack(|| {
                <LetCondition as ::treesitter_types::FromNode>::from_node(node, src)
            })
            .map(Self::LetCondition)
            .unwrap_or(Self::Unknown(node)),
            "let_declaration" => ::treesitter_types::runtime::maybe_grow_stack(|| {
                <LetDeclaration as ::treesitter_types::FromNode>::from_node(node, src)
            })
            .map(Self::LetDeclaration)
            .unwrap_or(Self::Unknown(node)),
            "lifetime" => ::treesitter_types::runtime::maybe_grow_stack(|| {
                <Lifetime as ::treesitter_types::FromNode>::from_node(node, src)
            })
            .map(Self::Lifetime)
            .unwrap_or(Self::Unknown(node)),
            "lifetime_parameter" => ::treesitter_types::runtime::maybe_grow_stack(|| {
                <LifetimeParameter as ::treesitter_types::FromNode>::from_node(node, src)
            })
            .map(Self::LifetimeParameter)
            .unwrap_or(Self::Unknown(node)),
            "line_comment" => ::treesitter_types::runtime::maybe_grow_stack(|| {
                <LineComment as ::treesitter_types::FromNode>::from_node(node, src)
            })
            .map(Self::LineComment)
            .unwrap_or(Self::Unknown(node)),
            "loop_expression" => ::treesitter_types::runtime::maybe_grow_stack(|| {
                <LoopExpression as ::treesitter_types::FromNode>::from_node(node, src)
            })
            .map(Self::LoopExpression)
            .unwrap_or(Self::Unknown(node)),
            "macro_definition" => ::treesitter_types::runtime::maybe_grow_stack(|| {
                <MacroDefinition as ::treesitter_types::FromNode>::from_node(node, src)
            })
            .map(Self::MacroDefinition)
            .unwrap_or(Self::Unknown(node)),
            "macro_invocation" => ::treesitter_types::runtime::maybe_grow_stack(|| {
                <MacroInvocation as ::treesitter_types::FromNode>::from_node(node, src)
            })
            .map(Self::MacroInvocation)
            .unwrap_or(Self::Unknown(node)),
            "macro_rule" => ::treesitter_types::runtime::maybe_grow_stack(|| {
                <MacroRule as ::treesitter_types::FromNode>::from_node(node, src)
            })
            .map(Self::MacroRule)
            .unwrap_or(Self::Unknown(node)),
            "match_arm" => ::treesitter_types::runtime::maybe_grow_stack(|| {
                <MatchArm as ::treesitter_types::FromNode>::from_node(node, src)
            })
            .map(Self::MatchArm)
            .unwrap_or(Self::Unknown(node)),
            "match_block" => ::treesitter_types::runtime::maybe_grow_stack(|| {
                <MatchBlock as ::treesitter_types::FromNode>::from_node(node, src)
            })
            .map(Self::MatchBlock)
            .unwrap_or(Self::Unknown(node)),
            "match_expression" => ::treesitter_types::runtime::maybe_grow_stack(|| {
                <MatchExpression as ::treesitter_types::FromNode>::from_node(node, src)
            })
            .map(Self::MatchExpression)
            .unwrap_or(Self::Unknown(node)),
            "match_pattern" => ::treesitter_types::runtime::maybe_grow_stack(|| {
                <MatchPattern as ::treesitter_types::FromNode>::from_node(node, src)
            })
            .map(Self::MatchPattern)
            .unwrap_or(Self::Unknown(node)),
            "mod_item" => ::treesitter_types::runtime::maybe_grow_stack(|| {
                <ModItem as ::treesitter_types::FromNode>::from_node(node, src)
            })
            .map(Self::ModItem)
            .unwrap_or(Self::Unknown(node)),
            "mut_pattern" => ::treesitter_types::runtime::maybe_grow_stack(|| {
                <MutPattern as ::treesitter_types::FromNode>::from_node(node, src)
            })
            .map(Self::MutPattern)
            .unwrap_or(Self::Unknown(node)),
            "negative_literal" => ::treesitter_types::runtime::maybe_grow_stack(|| {
                <NegativeLiteral as ::treesitter_types::FromNode>::from_node(node, src)
            })
            .map(Self::NegativeLiteral)
            .unwrap_or(Self::Unknown(node)),
            "never_type" => ::treesitter_types::runtime::maybe_grow_stack(|| {
                <NeverType as ::treesitter_types::FromNode>::from_node(node, src)
            })
            .map(Self::NeverType)
            .unwrap_or(Self::Unknown(node)),
            "or_pattern" => ::treesitter_types::runtime::maybe_grow_stack(|| {
                <OrPattern as ::treesitter_types::FromNode>::from_node(node, src)
            })
            .map(Self::OrPattern)
            .unwrap_or(Self::Unknown(node)),
            "ordered_field_declaration_list" => ::treesitter_types::runtime::maybe_grow_stack(|| {
                <OrderedFieldDeclarationList as ::treesitter_types::FromNode>::from_node(node, src)
            })
            .map(Self::OrderedFieldDeclarationList)
            .unwrap_or(Self::Unknown(node)),
            "outer_doc_comment_marker" => ::treesitter_types::runtime::maybe_grow_stack(|| {
                <OuterDocCommentMarker as ::treesitter_types::FromNode>::from_node(node, src)
            })
            .map(Self::OuterDocCommentMarker)
            .unwrap_or(Self::Unknown(node)),
            "parameter" => ::treesitter_types::runtime::maybe_grow_stack(|| {
                <Parameter as ::treesitter_types::FromNode>::from_node(node, src)
            })
            .map(Self::Parameter)
            .unwrap_or(Self::Unknown(node)),
            "parameters" => ::treesitter_types::runtime::maybe_grow_stack(|| {
                <Parameters as ::treesitter_types::FromNode>::from_node(node, src)
            })
            .map(Self::Parameters)
            .unwrap_or(Self::Unknown(node)),
            "parenthesized_expression" => ::treesitter_types::runtime::maybe_grow_stack(|| {
                <ParenthesizedExpression as ::treesitter_types::FromNode>::from_node(node, src)
            })
            .map(Self::ParenthesizedExpression)
            .unwrap_or(Self::Unknown(node)),
            "pointer_type" => ::treesitter_types::runtime::maybe_grow_stack(|| {
                <PointerType as ::treesitter_types::FromNode>::from_node(node, src)
            })
            .map(Self::PointerType)
            .unwrap_or(Self::Unknown(node)),
            "qualified_type" => ::treesitter_types::runtime::maybe_grow_stack(|| {
                <QualifiedType as ::treesitter_types::FromNode>::from_node(node, src)
            })
            .map(Self::QualifiedType)
            .unwrap_or(Self::Unknown(node)),
            "range_expression" => ::treesitter_types::runtime::maybe_grow_stack(|| {
                <RangeExpression as ::treesitter_types::FromNode>::from_node(node, src)
            })
            .map(Self::RangeExpression)
            .unwrap_or(Self::Unknown(node)),
            "range_pattern" => ::treesitter_types::runtime::maybe_grow_stack(|| {
                <RangePattern as ::treesitter_types::FromNode>::from_node(node, src)
            })
            .map(Self::RangePattern)
            .unwrap_or(Self::Unknown(node)),
            "raw_string_literal" => ::treesitter_types::runtime::maybe_grow_stack(|| {
                <RawStringLiteral as ::treesitter_types::FromNode>::from_node(node, src)
            })
            .map(Self::RawStringLiteral)
            .unwrap_or(Self::Unknown(node)),
            "ref_pattern" => ::treesitter_types::runtime::maybe_grow_stack(|| {
                <RefPattern as ::treesitter_types::FromNode>::from_node(node, src)
            })
            .map(Self::RefPattern)
            .unwrap_or(Self::Unknown(node)),
            "reference_expression" => ::treesitter_types::runtime::maybe_grow_stack(|| {
                <ReferenceExpression as ::treesitter_types::FromNode>::from_node(node, src)
            })
            .map(Self::ReferenceExpression)
            .unwrap_or(Self::Unknown(node)),
            "reference_pattern" => ::treesitter_types::runtime::maybe_grow_stack(|| {
                <ReferencePattern as ::treesitter_types::FromNode>::from_node(node, src)
            })
            .map(Self::ReferencePattern)
            .unwrap_or(Self::Unknown(node)),
            "reference_type" => ::treesitter_types::runtime::maybe_grow_stack(|| {
                <ReferenceType as ::treesitter_types::FromNode>::from_node(node, src)
            })
            .map(Self::ReferenceType)
            .unwrap_or(Self::Unknown(node)),
            "remaining_field_pattern" => ::treesitter_types::runtime::maybe_grow_stack(|| {
                <RemainingFieldPattern as ::treesitter_types::FromNode>::from_node(node, src)
            })
            .map(Self::RemainingFieldPattern)
            .unwrap_or(Self::Unknown(node)),
            "removed_trait_bound" => ::treesitter_types::runtime::maybe_grow_stack(|| {
                <RemovedTraitBound as ::treesitter_types::FromNode>::from_node(node, src)
            })
            .map(Self::RemovedTraitBound)
            .unwrap_or(Self::Unknown(node)),
            "return_expression" => ::treesitter_types::runtime::maybe_grow_stack(|| {
                <ReturnExpression as ::treesitter_types::FromNode>::from_node(node, src)
            })
            .map(Self::ReturnExpression)
            .unwrap_or(Self::Unknown(node)),
            "scoped_identifier" => ::treesitter_types::runtime::maybe_grow_stack(|| {
                <ScopedIdentifier as ::treesitter_types::FromNode>::from_node(node, src)
            })
            .map(Self::ScopedIdentifier)
            .unwrap_or(Self::Unknown(node)),
            "scoped_type_identifier" => ::treesitter_types::runtime::maybe_grow_stack(|| {
                <ScopedTypeIdentifier as ::treesitter_types::FromNode>::from_node(node, src)
            })
            .map(Self::ScopedTypeIdentifier)
            .unwrap_or(Self::Unknown(node)),
            "scoped_use_list" => ::treesitter_types::runtime::maybe_grow_stack(|| {
                <ScopedUseList as ::treesitter_types::FromNode>::from_node(node, src)
            })
            .map(Self::ScopedUseList)
            .unwrap_or(Self::Unknown(node)),
            "self_parameter" => ::treesitter_types::runtime::maybe_grow_stack(|| {
                <SelfParameter as ::treesitter_types::FromNode>::from_node(node, src)
            })
            .map(Self::SelfParameter)
            .unwrap_or(Self::Unknown(node)),
            "shorthand_field_initializer" => ::treesitter_types::runtime::maybe_grow_stack(|| {
                <ShorthandFieldInitializer as ::treesitter_types::FromNode>::from_node(node, src)
            })
            .map(Self::ShorthandFieldInitializer)
            .unwrap_or(Self::Unknown(node)),
            "slice_pattern" => ::treesitter_types::runtime::maybe_grow_stack(|| {
                <SlicePattern as ::treesitter_types::FromNode>::from_node(node, src)
            })
            .map(Self::SlicePattern)
            .unwrap_or(Self::Unknown(node)),
            "source_file" => ::treesitter_types::runtime::maybe_grow_stack(|| {
                <SourceFile as ::treesitter_types::FromNode>::from_node(node, src)
            })
            .map(Self::SourceFile)
            .unwrap_or(Self::Unknown(node)),
            "static_item" => ::treesitter_types::runtime::maybe_grow_stack(|| {
                <StaticItem as ::treesitter_types::FromNode>::from_node(node, src)
            })
            .map(Self::StaticItem)
            .unwrap_or(Self::Unknown(node)),
            "string_literal" => ::treesitter_types::runtime::maybe_grow_stack(|| {
                <StringLiteral as ::treesitter_types::FromNode>::from_node(node, src)
            })
            .map(Self::StringLiteral)
            .unwrap_or(Self::Unknown(node)),
            "struct_expression" => ::treesitter_types::runtime::maybe_grow_stack(|| {
                <StructExpression as ::treesitter_types::FromNode>::from_node(node, src)
            })
            .map(Self::StructExpression)
            .unwrap_or(Self::Unknown(node)),
            "struct_item" => ::treesitter_types::runtime::maybe_grow_stack(|| {
                <StructItem as ::treesitter_types::FromNode>::from_node(node, src)
            })
            .map(Self::StructItem)
            .unwrap_or(Self::Unknown(node)),
            "struct_pattern" => ::treesitter_types::runtime::maybe_grow_stack(|| {
                <StructPattern as ::treesitter_types::FromNode>::from_node(node, src)
            })
            .map(Self::StructPattern)
            .unwrap_or(Self::Unknown(node)),
            "token_binding_pattern" => ::treesitter_types::runtime::maybe_grow_stack(|| {
                <TokenBindingPattern as ::treesitter_types::FromNode>::from_node(node, src)
            })
            .map(Self::TokenBindingPattern)
            .unwrap_or(Self::Unknown(node)),
            "token_repetition" => ::treesitter_types::runtime::maybe_grow_stack(|| {
                <TokenRepetition as ::treesitter_types::FromNode>::from_node(node, src)
            })
            .map(Self::TokenRepetition)
            .unwrap_or(Self::Unknown(node)),
            "token_repetition_pattern" => ::treesitter_types::runtime::maybe_grow_stack(|| {
                <TokenRepetitionPattern as ::treesitter_types::FromNode>::from_node(node, src)
            })
            .map(Self::TokenRepetitionPattern)
            .unwrap_or(Self::Unknown(node)),
            "token_tree" => ::treesitter_types::runtime::maybe_grow_stack(|| {
                <TokenTree as ::treesitter_types::FromNode>::from_node(node, src)
            })
            .map(Self::TokenTree)
            .unwrap_or(Self::Unknown(node)),
            "token_tree_pattern" => ::treesitter_types::runtime::maybe_grow_stack(|| {
                <TokenTreePattern as ::treesitter_types::FromNode>::from_node(node, src)
            })
            .map(Self::TokenTreePattern)
            .unwrap_or(Self::Unknown(node)),
            "trait_bounds" => ::treesitter_types::runtime::maybe_grow_stack(|| {
                <TraitBounds as ::treesitter_types::FromNode>::from_node(node, src)
            })
            .map(Self::TraitBounds)
            .unwrap_or(Self::Unknown(node)),
            "trait_item" => ::treesitter_types::runtime::maybe_grow_stack(|| {
                <TraitItem as ::treesitter_types::FromNode>::from_node(node, src)
            })
            .map(Self::TraitItem)
            .unwrap_or(Self::Unknown(node)),
            "try_block" => ::treesitter_types::runtime::maybe_grow_stack(|| {
                <TryBlock as ::treesitter_types::FromNode>::from_node(node, src)
            })
            .map(Self::TryBlock)
            .unwrap_or(Self::Unknown(node)),
            "try_expression" => ::treesitter_types::runtime::maybe_grow_stack(|| {
                <TryExpression as ::treesitter_types::FromNode>::from_node(node, src)
            })
            .map(Self::TryExpression)
            .unwrap_or(Self::Unknown(node)),
            "tuple_expression" => ::treesitter_types::runtime::maybe_grow_stack(|| {
                <TupleExpression as ::treesitter_types::FromNode>::from_node(node, src)
            })
            .map(Self::TupleExpression)
            .unwrap_or(Self::Unknown(node)),
            "tuple_pattern" => ::treesitter_types::runtime::maybe_grow_stack(|| {
                <TuplePattern as ::treesitter_types::FromNode>::from_node(node, src)
            })
            .map(Self::TuplePattern)
            .unwrap_or(Self::Unknown(node)),
            "tuple_struct_pattern" => ::treesitter_types::runtime::maybe_grow_stack(|| {
                <TupleStructPattern as ::treesitter_types::FromNode>::from_node(node, src)
            })
            .map(Self::TupleStructPattern)
            .unwrap_or(Self::Unknown(node)),
            "tuple_type" => ::treesitter_types::runtime::maybe_grow_stack(|| {
                <TupleType as ::treesitter_types::FromNode>::from_node(node, src)
            })
            .map(Self::TupleType)
            .unwrap_or(Self::Unknown(node)),
            "type_arguments" => ::treesitter_types::runtime::maybe_grow_stack(|| {
                <TypeArguments as ::treesitter_types::FromNode>::from_node(node, src)
            })
            .map(Self::TypeArguments)
            .unwrap_or(Self::Unknown(node)),
            "type_binding" => ::treesitter_types::runtime::maybe_grow_stack(|| {
                <TypeBinding as ::treesitter_types::FromNode>::from_node(node, src)
            })
            .map(Self::TypeBinding)
            .unwrap_or(Self::Unknown(node)),
            "type_cast_expression" => ::treesitter_types::runtime::maybe_grow_stack(|| {
                <TypeCastExpression as ::treesitter_types::FromNode>::from_node(node, src)
            })
            .map(Self::TypeCastExpression)
            .unwrap_or(Self::Unknown(node)),
            "type_item" => ::treesitter_types::runtime::maybe_grow_stack(|| {
                <TypeItem as ::treesitter_types::FromNode>::from_node(node, src)
            })
            .map(Self::TypeItem)
            .unwrap_or(Self::Unknown(node)),
            "type_parameter" => ::treesitter_types::runtime::maybe_grow_stack(|| {
                <TypeParameter as ::treesitter_types::FromNode>::from_node(node, src)
            })
            .map(Self::TypeParameter)
            .unwrap_or(Self::Unknown(node)),
            "type_parameters" => ::treesitter_types::runtime::maybe_grow_stack(|| {
                <TypeParameters as ::treesitter_types::FromNode>::from_node(node, src)
            })
            .map(Self::TypeParameters)
            .unwrap_or(Self::Unknown(node)),
            "unary_expression" => ::treesitter_types::runtime::maybe_grow_stack(|| {
                <UnaryExpression as ::treesitter_types::FromNode>::from_node(node, src)
            })
            .map(Self::UnaryExpression)
            .unwrap_or(Self::Unknown(node)),
            "union_item" => ::treesitter_types::runtime::maybe_grow_stack(|| {
                <UnionItem as ::treesitter_types::FromNode>::from_node(node, src)
            })
            .map(Self::UnionItem)
            .unwrap_or(Self::Unknown(node)),
            "unit_expression" => ::treesitter_types::runtime::maybe_grow_stack(|| {
                <UnitExpression as ::treesitter_types::FromNode>::from_node(node, src)
            })
            .map(Self::UnitExpression)
            .unwrap_or(Self::Unknown(node)),
            "unit_type" => ::treesitter_types::runtime::maybe_grow_stack(|| {
                <UnitType as ::treesitter_types::FromNode>::from_node(node, src)
            })
            .map(Self::UnitType)
            .unwrap_or(Self::Unknown(node)),
            "unsafe_block" => ::treesitter_types::runtime::maybe_grow_stack(|| {
                <UnsafeBlock as ::treesitter_types::FromNode>::from_node(node, src)
            })
            .map(Self::UnsafeBlock)
            .unwrap_or(Self::Unknown(node)),
            "use_as_clause" => ::treesitter_types::runtime::maybe_grow_stack(|| {
                <UseAsClause as ::treesitter_types::FromNode>::from_node(node, src)
            })
            .map(Self::UseAsClause)
            .unwrap_or(Self::Unknown(node)),
            "use_bounds" => ::treesitter_types::runtime::maybe_grow_stack(|| {
                <UseBounds as ::treesitter_types::FromNode>::from_node(node, src)
            })
            .map(Self::UseBounds)
            .unwrap_or(Self::Unknown(node)),
            "use_declaration" => ::treesitter_types::runtime::maybe_grow_stack(|| {
                <UseDeclaration as ::treesitter_types::FromNode>::from_node(node, src)
            })
            .map(Self::UseDeclaration)
            .unwrap_or(Self::Unknown(node)),
            "use_list" => ::treesitter_types::runtime::maybe_grow_stack(|| {
                <UseList as ::treesitter_types::FromNode>::from_node(node, src)
            })
            .map(Self::UseList)
            .unwrap_or(Self::Unknown(node)),
            "use_wildcard" => ::treesitter_types::runtime::maybe_grow_stack(|| {
                <UseWildcard as ::treesitter_types::FromNode>::from_node(node, src)
            })
            .map(Self::UseWildcard)
            .unwrap_or(Self::Unknown(node)),
            "variadic_parameter" => ::treesitter_types::runtime::maybe_grow_stack(|| {
                <VariadicParameter as ::treesitter_types::FromNode>::from_node(node, src)
            })
            .map(Self::VariadicParameter)
            .unwrap_or(Self::Unknown(node)),
            "visibility_modifier" => ::treesitter_types::runtime::maybe_grow_stack(|| {
                <VisibilityModifier as ::treesitter_types::FromNode>::from_node(node, src)
            })
            .map(Self::VisibilityModifier)
            .unwrap_or(Self::Unknown(node)),
            "where_clause" => ::treesitter_types::runtime::maybe_grow_stack(|| {
                <WhereClause as ::treesitter_types::FromNode>::from_node(node, src)
            })
            .map(Self::WhereClause)
            .unwrap_or(Self::Unknown(node)),
            "where_predicate" => ::treesitter_types::runtime::maybe_grow_stack(|| {
                <WherePredicate as ::treesitter_types::FromNode>::from_node(node, src)
            })
            .map(Self::WherePredicate)
            .unwrap_or(Self::Unknown(node)),
            "while_expression" => ::treesitter_types::runtime::maybe_grow_stack(|| {
                <WhileExpression as ::treesitter_types::FromNode>::from_node(node, src)
            })
            .map(Self::WhileExpression)
            .unwrap_or(Self::Unknown(node)),
            "yield_expression" => ::treesitter_types::runtime::maybe_grow_stack(|| {
                <YieldExpression as ::treesitter_types::FromNode>::from_node(node, src)
            })
            .map(Self::YieldExpression)
            .unwrap_or(Self::Unknown(node)),
            "char_literal" => ::treesitter_types::runtime::maybe_grow_stack(|| {
                <CharLiteral as ::treesitter_types::FromNode>::from_node(node, src)
            })
            .map(Self::CharLiteral)
            .unwrap_or(Self::Unknown(node)),
            "crate" => ::treesitter_types::runtime::maybe_grow_stack(|| {
                <Crate as ::treesitter_types::FromNode>::from_node(node, src)
            })
            .map(Self::Crate)
            .unwrap_or(Self::Unknown(node)),
            "doc_comment" => ::treesitter_types::runtime::maybe_grow_stack(|| {
                <DocComment as ::treesitter_types::FromNode>::from_node(node, src)
            })
            .map(Self::DocComment)
            .unwrap_or(Self::Unknown(node)),
            "escape_sequence" => ::treesitter_types::runtime::maybe_grow_stack(|| {
                <EscapeSequence as ::treesitter_types::FromNode>::from_node(node, src)
            })
            .map(Self::EscapeSequence)
            .unwrap_or(Self::Unknown(node)),
            "field_identifier" => ::treesitter_types::runtime::maybe_grow_stack(|| {
                <FieldIdentifier as ::treesitter_types::FromNode>::from_node(node, src)
            })
            .map(Self::FieldIdentifier)
            .unwrap_or(Self::Unknown(node)),
            "float_literal" => ::treesitter_types::runtime::maybe_grow_stack(|| {
                <FloatLiteral as ::treesitter_types::FromNode>::from_node(node, src)
            })
            .map(Self::FloatLiteral)
            .unwrap_or(Self::Unknown(node)),
            "identifier" => ::treesitter_types::runtime::maybe_grow_stack(|| {
                <Identifier as ::treesitter_types::FromNode>::from_node(node, src)
            })
            .map(Self::Identifier)
            .unwrap_or(Self::Unknown(node)),
            "integer_literal" => ::treesitter_types::runtime::maybe_grow_stack(|| {
                <IntegerLiteral as ::treesitter_types::FromNode>::from_node(node, src)
            })
            .map(Self::IntegerLiteral)
            .unwrap_or(Self::Unknown(node)),
            "metavariable" => ::treesitter_types::runtime::maybe_grow_stack(|| {
                <Metavariable as ::treesitter_types::FromNode>::from_node(node, src)
            })
            .map(Self::Metavariable)
            .unwrap_or(Self::Unknown(node)),
            "mutable_specifier" => ::treesitter_types::runtime::maybe_grow_stack(|| {
                <MutableSpecifier as ::treesitter_types::FromNode>::from_node(node, src)
            })
            .map(Self::MutableSpecifier)
            .unwrap_or(Self::Unknown(node)),
            "primitive_type" => ::treesitter_types::runtime::maybe_grow_stack(|| {
                <PrimitiveType as ::treesitter_types::FromNode>::from_node(node, src)
            })
            .map(Self::PrimitiveType)
            .unwrap_or(Self::Unknown(node)),
            "self" => ::treesitter_types::runtime::maybe_grow_stack(|| {
                <SelfType as ::treesitter_types::FromNode>::from_node(node, src)
            })
            .map(Self::SelfType)
            .unwrap_or(Self::Unknown(node)),
            "shebang" => ::treesitter_types::runtime::maybe_grow_stack(|| {
                <Shebang as ::treesitter_types::FromNode>::from_node(node, src)
            })
            .map(Self::Shebang)
            .unwrap_or(Self::Unknown(node)),
            "shorthand_field_identifier" => ::treesitter_types::runtime::maybe_grow_stack(|| {
                <ShorthandFieldIdentifier as ::treesitter_types::FromNode>::from_node(node, src)
            })
            .map(Self::ShorthandFieldIdentifier)
            .unwrap_or(Self::Unknown(node)),
            "string_content" => ::treesitter_types::runtime::maybe_grow_stack(|| {
                <StringContent as ::treesitter_types::FromNode>::from_node(node, src)
            })
            .map(Self::StringContent)
            .unwrap_or(Self::Unknown(node)),
            "super" => ::treesitter_types::runtime::maybe_grow_stack(|| {
                <Super as ::treesitter_types::FromNode>::from_node(node, src)
            })
            .map(Self::Super)
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
            Self::DeclarationStatement(inner) => inner.span(),
            Self::Expression(inner) => inner.span(),
            Self::Literal(inner) => inner.span(),
            Self::LiteralPattern(inner) => inner.span(),
            Self::Pattern(inner) => inner.span(),
            Self::Type(inner) => inner.span(),
            Self::AbstractType(inner) => inner.span(),
            Self::Arguments(inner) => inner.span(),
            Self::ArrayExpression(inner) => inner.span(),
            Self::ArrayType(inner) => inner.span(),
            Self::AssignmentExpression(inner) => inner.span(),
            Self::AssociatedType(inner) => inner.span(),
            Self::AsyncBlock(inner) => inner.span(),
            Self::Attribute(inner) => inner.span(),
            Self::AttributeItem(inner) => inner.span(),
            Self::AwaitExpression(inner) => inner.span(),
            Self::BaseFieldInitializer(inner) => inner.span(),
            Self::BinaryExpression(inner) => inner.span(),
            Self::Block(inner) => inner.span(),
            Self::BlockComment(inner) => inner.span(),
            Self::BooleanLiteral(inner) => inner.span(),
            Self::BoundedType(inner) => inner.span(),
            Self::BracketedType(inner) => inner.span(),
            Self::BreakExpression(inner) => inner.span(),
            Self::CallExpression(inner) => inner.span(),
            Self::CapturedPattern(inner) => inner.span(),
            Self::ClosureExpression(inner) => inner.span(),
            Self::ClosureParameters(inner) => inner.span(),
            Self::CompoundAssignmentExpr(inner) => inner.span(),
            Self::ConstBlock(inner) => inner.span(),
            Self::ConstItem(inner) => inner.span(),
            Self::ConstParameter(inner) => inner.span(),
            Self::ContinueExpression(inner) => inner.span(),
            Self::DeclarationList(inner) => inner.span(),
            Self::DynamicType(inner) => inner.span(),
            Self::ElseClause(inner) => inner.span(),
            Self::EmptyStatement(inner) => inner.span(),
            Self::EnumItem(inner) => inner.span(),
            Self::EnumVariant(inner) => inner.span(),
            Self::EnumVariantList(inner) => inner.span(),
            Self::ExpressionStatement(inner) => inner.span(),
            Self::ExternCrateDeclaration(inner) => inner.span(),
            Self::ExternModifier(inner) => inner.span(),
            Self::FieldDeclaration(inner) => inner.span(),
            Self::FieldDeclarationList(inner) => inner.span(),
            Self::FieldExpression(inner) => inner.span(),
            Self::FieldInitializer(inner) => inner.span(),
            Self::FieldInitializerList(inner) => inner.span(),
            Self::FieldPattern(inner) => inner.span(),
            Self::ForExpression(inner) => inner.span(),
            Self::ForLifetimes(inner) => inner.span(),
            Self::ForeignModItem(inner) => inner.span(),
            Self::FragmentSpecifier(inner) => inner.span(),
            Self::FunctionItem(inner) => inner.span(),
            Self::FunctionModifiers(inner) => inner.span(),
            Self::FunctionSignatureItem(inner) => inner.span(),
            Self::FunctionType(inner) => inner.span(),
            Self::GenBlock(inner) => inner.span(),
            Self::GenericFunction(inner) => inner.span(),
            Self::GenericPattern(inner) => inner.span(),
            Self::GenericType(inner) => inner.span(),
            Self::GenericTypeWithTurbofish(inner) => inner.span(),
            Self::HigherRankedTraitBound(inner) => inner.span(),
            Self::IfExpression(inner) => inner.span(),
            Self::ImplItem(inner) => inner.span(),
            Self::IndexExpression(inner) => inner.span(),
            Self::InnerAttributeItem(inner) => inner.span(),
            Self::InnerDocCommentMarker(inner) => inner.span(),
            Self::Label(inner) => inner.span(),
            Self::LetChain(inner) => inner.span(),
            Self::LetCondition(inner) => inner.span(),
            Self::LetDeclaration(inner) => inner.span(),
            Self::Lifetime(inner) => inner.span(),
            Self::LifetimeParameter(inner) => inner.span(),
            Self::LineComment(inner) => inner.span(),
            Self::LoopExpression(inner) => inner.span(),
            Self::MacroDefinition(inner) => inner.span(),
            Self::MacroInvocation(inner) => inner.span(),
            Self::MacroRule(inner) => inner.span(),
            Self::MatchArm(inner) => inner.span(),
            Self::MatchBlock(inner) => inner.span(),
            Self::MatchExpression(inner) => inner.span(),
            Self::MatchPattern(inner) => inner.span(),
            Self::ModItem(inner) => inner.span(),
            Self::MutPattern(inner) => inner.span(),
            Self::NegativeLiteral(inner) => inner.span(),
            Self::NeverType(inner) => inner.span(),
            Self::OrPattern(inner) => inner.span(),
            Self::OrderedFieldDeclarationList(inner) => inner.span(),
            Self::OuterDocCommentMarker(inner) => inner.span(),
            Self::Parameter(inner) => inner.span(),
            Self::Parameters(inner) => inner.span(),
            Self::ParenthesizedExpression(inner) => inner.span(),
            Self::PointerType(inner) => inner.span(),
            Self::QualifiedType(inner) => inner.span(),
            Self::RangeExpression(inner) => inner.span(),
            Self::RangePattern(inner) => inner.span(),
            Self::RawStringLiteral(inner) => inner.span(),
            Self::RefPattern(inner) => inner.span(),
            Self::ReferenceExpression(inner) => inner.span(),
            Self::ReferencePattern(inner) => inner.span(),
            Self::ReferenceType(inner) => inner.span(),
            Self::RemainingFieldPattern(inner) => inner.span(),
            Self::RemovedTraitBound(inner) => inner.span(),
            Self::ReturnExpression(inner) => inner.span(),
            Self::ScopedIdentifier(inner) => inner.span(),
            Self::ScopedTypeIdentifier(inner) => inner.span(),
            Self::ScopedUseList(inner) => inner.span(),
            Self::SelfParameter(inner) => inner.span(),
            Self::ShorthandFieldInitializer(inner) => inner.span(),
            Self::SlicePattern(inner) => inner.span(),
            Self::SourceFile(inner) => inner.span(),
            Self::StaticItem(inner) => inner.span(),
            Self::StringLiteral(inner) => inner.span(),
            Self::StructExpression(inner) => inner.span(),
            Self::StructItem(inner) => inner.span(),
            Self::StructPattern(inner) => inner.span(),
            Self::TokenBindingPattern(inner) => inner.span(),
            Self::TokenRepetition(inner) => inner.span(),
            Self::TokenRepetitionPattern(inner) => inner.span(),
            Self::TokenTree(inner) => inner.span(),
            Self::TokenTreePattern(inner) => inner.span(),
            Self::TraitBounds(inner) => inner.span(),
            Self::TraitItem(inner) => inner.span(),
            Self::TryBlock(inner) => inner.span(),
            Self::TryExpression(inner) => inner.span(),
            Self::TupleExpression(inner) => inner.span(),
            Self::TuplePattern(inner) => inner.span(),
            Self::TupleStructPattern(inner) => inner.span(),
            Self::TupleType(inner) => inner.span(),
            Self::TypeArguments(inner) => inner.span(),
            Self::TypeBinding(inner) => inner.span(),
            Self::TypeCastExpression(inner) => inner.span(),
            Self::TypeItem(inner) => inner.span(),
            Self::TypeParameter(inner) => inner.span(),
            Self::TypeParameters(inner) => inner.span(),
            Self::UnaryExpression(inner) => inner.span(),
            Self::UnionItem(inner) => inner.span(),
            Self::UnitExpression(inner) => inner.span(),
            Self::UnitType(inner) => inner.span(),
            Self::UnsafeBlock(inner) => inner.span(),
            Self::UseAsClause(inner) => inner.span(),
            Self::UseBounds(inner) => inner.span(),
            Self::UseDeclaration(inner) => inner.span(),
            Self::UseList(inner) => inner.span(),
            Self::UseWildcard(inner) => inner.span(),
            Self::VariadicParameter(inner) => inner.span(),
            Self::VisibilityModifier(inner) => inner.span(),
            Self::WhereClause(inner) => inner.span(),
            Self::WherePredicate(inner) => inner.span(),
            Self::WhileExpression(inner) => inner.span(),
            Self::YieldExpression(inner) => inner.span(),
            Self::CharLiteral(inner) => inner.span(),
            Self::Crate(inner) => inner.span(),
            Self::DocComment(inner) => inner.span(),
            Self::EscapeSequence(inner) => inner.span(),
            Self::FieldIdentifier(inner) => inner.span(),
            Self::FloatLiteral(inner) => inner.span(),
            Self::Identifier(inner) => inner.span(),
            Self::IntegerLiteral(inner) => inner.span(),
            Self::Metavariable(inner) => inner.span(),
            Self::MutableSpecifier(inner) => inner.span(),
            Self::PrimitiveType(inner) => inner.span(),
            Self::SelfType(inner) => inner.span(),
            Self::Shebang(inner) => inner.span(),
            Self::ShorthandFieldIdentifier(inner) => inner.span(),
            Self::StringContent(inner) => inner.span(),
            Self::Super(inner) => inner.span(),
            Self::TypeIdentifier(inner) => inner.span(),
            Self::Unknown(node) => ::treesitter_types::Span::from(*node),
        }
    }
}
