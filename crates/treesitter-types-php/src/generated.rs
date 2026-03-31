#[derive(Debug, Clone)]
pub enum Expression<'tree> {
    AssignmentExpression(::std::boxed::Box<AssignmentExpression<'tree>>),
    AugmentedAssignmentExpression(::std::boxed::Box<AugmentedAssignmentExpression<'tree>>),
    BinaryExpression(::std::boxed::Box<BinaryExpression<'tree>>),
    CastExpression(::std::boxed::Box<CastExpression<'tree>>),
    CloneExpression(::std::boxed::Box<CloneExpression<'tree>>),
    ConditionalExpression(::std::boxed::Box<ConditionalExpression<'tree>>),
    ErrorSuppressionExpression(::std::boxed::Box<ErrorSuppressionExpression<'tree>>),
    IncludeExpression(::std::boxed::Box<IncludeExpression<'tree>>),
    IncludeOnceExpression(::std::boxed::Box<IncludeOnceExpression<'tree>>),
    MatchExpression(::std::boxed::Box<MatchExpression<'tree>>),
    PrimaryExpression(::std::boxed::Box<PrimaryExpression<'tree>>),
    ReferenceAssignmentExpression(::std::boxed::Box<ReferenceAssignmentExpression<'tree>>),
    RequireExpression(::std::boxed::Box<RequireExpression<'tree>>),
    RequireOnceExpression(::std::boxed::Box<RequireOnceExpression<'tree>>),
    UnaryOpExpression(::std::boxed::Box<UnaryOpExpression<'tree>>),
    YieldExpression(::std::boxed::Box<YieldExpression<'tree>>),
}
impl<'tree> ::treesitter_types::FromNode<'tree> for Expression<'tree> {
    #[allow(clippy::collapsible_else_if)]
    fn from_node(
        node: ::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        match node.kind() {
            "assignment_expression" => Ok(Self::AssignmentExpression(::std::boxed::Box::new(
                <AssignmentExpression as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "augmented_assignment_expression" => {
                Ok(Self::AugmentedAssignmentExpression(::std::boxed::Box::new(
                    <AugmentedAssignmentExpression as ::treesitter_types::FromNode>::from_node(
                        node, src,
                    )?,
                )))
            }
            "binary_expression" => Ok(Self::BinaryExpression(::std::boxed::Box::new(
                <BinaryExpression as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "cast_expression" => Ok(Self::CastExpression(::std::boxed::Box::new(
                <CastExpression as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "clone_expression" => Ok(Self::CloneExpression(::std::boxed::Box::new(
                <CloneExpression as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "conditional_expression" => Ok(Self::ConditionalExpression(::std::boxed::Box::new(
                <ConditionalExpression as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "error_suppression_expression" => {
                Ok(Self::ErrorSuppressionExpression(::std::boxed::Box::new(
                    <ErrorSuppressionExpression as ::treesitter_types::FromNode>::from_node(
                        node, src,
                    )?,
                )))
            }
            "include_expression" => Ok(Self::IncludeExpression(::std::boxed::Box::new(
                <IncludeExpression as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "include_once_expression" => Ok(Self::IncludeOnceExpression(::std::boxed::Box::new(
                <IncludeOnceExpression as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "match_expression" => Ok(Self::MatchExpression(::std::boxed::Box::new(
                <MatchExpression as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "reference_assignment_expression" => {
                Ok(Self::ReferenceAssignmentExpression(::std::boxed::Box::new(
                    <ReferenceAssignmentExpression as ::treesitter_types::FromNode>::from_node(
                        node, src,
                    )?,
                )))
            }
            "require_expression" => Ok(Self::RequireExpression(::std::boxed::Box::new(
                <RequireExpression as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "require_once_expression" => Ok(Self::RequireOnceExpression(::std::boxed::Box::new(
                <RequireOnceExpression as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "unary_op_expression" => Ok(Self::UnaryOpExpression(::std::boxed::Box::new(
                <UnaryOpExpression as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "yield_expression" => Ok(Self::YieldExpression(::std::boxed::Box::new(
                <YieldExpression as ::treesitter_types::FromNode>::from_node(node, src)?,
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
            Self::AssignmentExpression(inner) => inner.span(),
            Self::AugmentedAssignmentExpression(inner) => inner.span(),
            Self::BinaryExpression(inner) => inner.span(),
            Self::CastExpression(inner) => inner.span(),
            Self::CloneExpression(inner) => inner.span(),
            Self::ConditionalExpression(inner) => inner.span(),
            Self::ErrorSuppressionExpression(inner) => inner.span(),
            Self::IncludeExpression(inner) => inner.span(),
            Self::IncludeOnceExpression(inner) => inner.span(),
            Self::MatchExpression(inner) => inner.span(),
            Self::PrimaryExpression(inner) => inner.span(),
            Self::ReferenceAssignmentExpression(inner) => inner.span(),
            Self::RequireExpression(inner) => inner.span(),
            Self::RequireOnceExpression(inner) => inner.span(),
            Self::UnaryOpExpression(inner) => inner.span(),
            Self::YieldExpression(inner) => inner.span(),
        }
    }
}
#[derive(Debug, Clone)]
pub enum Literal<'tree> {
    Boolean(::std::boxed::Box<Boolean<'tree>>),
    EncapsedString(::std::boxed::Box<EncapsedString<'tree>>),
    Float(::std::boxed::Box<Float<'tree>>),
    Heredoc(::std::boxed::Box<Heredoc<'tree>>),
    Integer(::std::boxed::Box<Integer<'tree>>),
    Nowdoc(::std::boxed::Box<Nowdoc<'tree>>),
    Null(::std::boxed::Box<Null<'tree>>),
    String(::std::boxed::Box<String<'tree>>),
}
impl<'tree> ::treesitter_types::FromNode<'tree> for Literal<'tree> {
    #[allow(clippy::collapsible_else_if)]
    fn from_node(
        node: ::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        match node.kind() {
            "boolean" => Ok(Self::Boolean(::std::boxed::Box::new(
                <Boolean as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "encapsed_string" => Ok(Self::EncapsedString(::std::boxed::Box::new(
                <EncapsedString as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "float" => Ok(Self::Float(::std::boxed::Box::new(
                <Float as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "heredoc" => Ok(Self::Heredoc(::std::boxed::Box::new(
                <Heredoc as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "integer" => Ok(Self::Integer(::std::boxed::Box::new(
                <Integer as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "nowdoc" => Ok(Self::Nowdoc(::std::boxed::Box::new(
                <Nowdoc as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "null" => Ok(Self::Null(::std::boxed::Box::new(
                <Null as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "string" => Ok(Self::String(::std::boxed::Box::new(
                <String as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            other => Err(::treesitter_types::ParseError::unexpected_kind(other, node)),
        }
    }
}
impl ::treesitter_types::Spanned for Literal<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        match self {
            Self::Boolean(inner) => inner.span(),
            Self::EncapsedString(inner) => inner.span(),
            Self::Float(inner) => inner.span(),
            Self::Heredoc(inner) => inner.span(),
            Self::Integer(inner) => inner.span(),
            Self::Nowdoc(inner) => inner.span(),
            Self::Null(inner) => inner.span(),
            Self::String(inner) => inner.span(),
        }
    }
}
#[derive(Debug, Clone)]
pub enum PrimaryExpression<'tree> {
    AnonymousFunction(::std::boxed::Box<AnonymousFunction<'tree>>),
    ArrayCreationExpression(::std::boxed::Box<ArrayCreationExpression<'tree>>),
    ArrowFunction(::std::boxed::Box<ArrowFunction<'tree>>),
    CastExpression(::std::boxed::Box<CastExpression<'tree>>),
    ClassConstantAccessExpression(::std::boxed::Box<ClassConstantAccessExpression<'tree>>),
    DynamicVariableName(::std::boxed::Box<DynamicVariableName<'tree>>),
    FunctionCallExpression(::std::boxed::Box<FunctionCallExpression<'tree>>),
    Literal(::std::boxed::Box<Literal<'tree>>),
    MemberAccessExpression(::std::boxed::Box<MemberAccessExpression<'tree>>),
    MemberCallExpression(::std::boxed::Box<MemberCallExpression<'tree>>),
    Name(::std::boxed::Box<Name<'tree>>),
    NullsafeMemberAccessExpression(::std::boxed::Box<NullsafeMemberAccessExpression<'tree>>),
    NullsafeMemberCallExpression(::std::boxed::Box<NullsafeMemberCallExpression<'tree>>),
    ObjectCreationExpression(::std::boxed::Box<ObjectCreationExpression<'tree>>),
    ParenthesizedExpression(::std::boxed::Box<ParenthesizedExpression<'tree>>),
    PrintIntrinsic(::std::boxed::Box<PrintIntrinsic<'tree>>),
    QualifiedName(::std::boxed::Box<QualifiedName<'tree>>),
    RelativeName(::std::boxed::Box<RelativeName<'tree>>),
    ScopedCallExpression(::std::boxed::Box<ScopedCallExpression<'tree>>),
    ScopedPropertyAccessExpression(::std::boxed::Box<ScopedPropertyAccessExpression<'tree>>),
    ShellCommandExpression(::std::boxed::Box<ShellCommandExpression<'tree>>),
    SubscriptExpression(::std::boxed::Box<SubscriptExpression<'tree>>),
    ThrowExpression(::std::boxed::Box<ThrowExpression<'tree>>),
    UpdateExpression(::std::boxed::Box<UpdateExpression<'tree>>),
    VariableName(::std::boxed::Box<VariableName<'tree>>),
}
impl<'tree> ::treesitter_types::FromNode<'tree> for PrimaryExpression<'tree> {
    #[allow(clippy::collapsible_else_if)]
    fn from_node(
        node: ::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        match node.kind() {
            "anonymous_function" => Ok(Self::AnonymousFunction(::std::boxed::Box::new(
                <AnonymousFunction as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "array_creation_expression" => {
                Ok(Self::ArrayCreationExpression(::std::boxed::Box::new(
                    <ArrayCreationExpression as ::treesitter_types::FromNode>::from_node(
                        node, src,
                    )?,
                )))
            }
            "arrow_function" => Ok(Self::ArrowFunction(::std::boxed::Box::new(
                <ArrowFunction as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "cast_expression" => Ok(Self::CastExpression(::std::boxed::Box::new(
                <CastExpression as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "class_constant_access_expression" => {
                Ok(Self::ClassConstantAccessExpression(::std::boxed::Box::new(
                    <ClassConstantAccessExpression as ::treesitter_types::FromNode>::from_node(
                        node, src,
                    )?,
                )))
            }
            "dynamic_variable_name" => Ok(Self::DynamicVariableName(::std::boxed::Box::new(
                <DynamicVariableName as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "function_call_expression" => Ok(Self::FunctionCallExpression(::std::boxed::Box::new(
                <FunctionCallExpression as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "member_access_expression" => Ok(Self::MemberAccessExpression(::std::boxed::Box::new(
                <MemberAccessExpression as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "member_call_expression" => Ok(Self::MemberCallExpression(::std::boxed::Box::new(
                <MemberCallExpression as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "name" => Ok(Self::Name(::std::boxed::Box::new(
                <Name as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "nullsafe_member_access_expression" => Ok(Self::NullsafeMemberAccessExpression(
                ::std::boxed::Box::new(
                    <NullsafeMemberAccessExpression as ::treesitter_types::FromNode>::from_node(
                        node, src,
                    )?,
                ),
            )),
            "nullsafe_member_call_expression" => {
                Ok(Self::NullsafeMemberCallExpression(::std::boxed::Box::new(
                    <NullsafeMemberCallExpression as ::treesitter_types::FromNode>::from_node(
                        node, src,
                    )?,
                )))
            }
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
            "print_intrinsic" => Ok(Self::PrintIntrinsic(::std::boxed::Box::new(
                <PrintIntrinsic as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "qualified_name" => Ok(Self::QualifiedName(::std::boxed::Box::new(
                <QualifiedName as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "relative_name" => Ok(Self::RelativeName(::std::boxed::Box::new(
                <RelativeName as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "scoped_call_expression" => Ok(Self::ScopedCallExpression(::std::boxed::Box::new(
                <ScopedCallExpression as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "scoped_property_access_expression" => Ok(Self::ScopedPropertyAccessExpression(
                ::std::boxed::Box::new(
                    <ScopedPropertyAccessExpression as ::treesitter_types::FromNode>::from_node(
                        node, src,
                    )?,
                ),
            )),
            "shell_command_expression" => Ok(Self::ShellCommandExpression(::std::boxed::Box::new(
                <ShellCommandExpression as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "subscript_expression" => Ok(Self::SubscriptExpression(::std::boxed::Box::new(
                <SubscriptExpression as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "throw_expression" => Ok(Self::ThrowExpression(::std::boxed::Box::new(
                <ThrowExpression as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "update_expression" => Ok(Self::UpdateExpression(::std::boxed::Box::new(
                <UpdateExpression as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "variable_name" => Ok(Self::VariableName(::std::boxed::Box::new(
                <VariableName as ::treesitter_types::FromNode>::from_node(node, src)?,
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
impl ::treesitter_types::Spanned for PrimaryExpression<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        match self {
            Self::AnonymousFunction(inner) => inner.span(),
            Self::ArrayCreationExpression(inner) => inner.span(),
            Self::ArrowFunction(inner) => inner.span(),
            Self::CastExpression(inner) => inner.span(),
            Self::ClassConstantAccessExpression(inner) => inner.span(),
            Self::DynamicVariableName(inner) => inner.span(),
            Self::FunctionCallExpression(inner) => inner.span(),
            Self::Literal(inner) => inner.span(),
            Self::MemberAccessExpression(inner) => inner.span(),
            Self::MemberCallExpression(inner) => inner.span(),
            Self::Name(inner) => inner.span(),
            Self::NullsafeMemberAccessExpression(inner) => inner.span(),
            Self::NullsafeMemberCallExpression(inner) => inner.span(),
            Self::ObjectCreationExpression(inner) => inner.span(),
            Self::ParenthesizedExpression(inner) => inner.span(),
            Self::PrintIntrinsic(inner) => inner.span(),
            Self::QualifiedName(inner) => inner.span(),
            Self::RelativeName(inner) => inner.span(),
            Self::ScopedCallExpression(inner) => inner.span(),
            Self::ScopedPropertyAccessExpression(inner) => inner.span(),
            Self::ShellCommandExpression(inner) => inner.span(),
            Self::SubscriptExpression(inner) => inner.span(),
            Self::ThrowExpression(inner) => inner.span(),
            Self::UpdateExpression(inner) => inner.span(),
            Self::VariableName(inner) => inner.span(),
        }
    }
}
#[derive(Debug, Clone)]
pub enum Statement<'tree> {
    BreakStatement(::std::boxed::Box<BreakStatement<'tree>>),
    ClassDeclaration(::std::boxed::Box<ClassDeclaration<'tree>>),
    CompoundStatement(::std::boxed::Box<CompoundStatement<'tree>>),
    ConstDeclaration(::std::boxed::Box<ConstDeclaration<'tree>>),
    ContinueStatement(::std::boxed::Box<ContinueStatement<'tree>>),
    DeclareStatement(::std::boxed::Box<DeclareStatement<'tree>>),
    DoStatement(::std::boxed::Box<DoStatement<'tree>>),
    EchoStatement(::std::boxed::Box<EchoStatement<'tree>>),
    EmptyStatement(::std::boxed::Box<EmptyStatement<'tree>>),
    EnumDeclaration(::std::boxed::Box<EnumDeclaration<'tree>>),
    ExitStatement(::std::boxed::Box<ExitStatement<'tree>>),
    ExpressionStatement(::std::boxed::Box<ExpressionStatement<'tree>>),
    ForStatement(::std::boxed::Box<ForStatement<'tree>>),
    ForeachStatement(::std::boxed::Box<ForeachStatement<'tree>>),
    FunctionDefinition(::std::boxed::Box<FunctionDefinition<'tree>>),
    FunctionStaticDeclaration(::std::boxed::Box<FunctionStaticDeclaration<'tree>>),
    GlobalDeclaration(::std::boxed::Box<GlobalDeclaration<'tree>>),
    GotoStatement(::std::boxed::Box<GotoStatement<'tree>>),
    IfStatement(::std::boxed::Box<IfStatement<'tree>>),
    InterfaceDeclaration(::std::boxed::Box<InterfaceDeclaration<'tree>>),
    NamedLabelStatement(::std::boxed::Box<NamedLabelStatement<'tree>>),
    NamespaceDefinition(::std::boxed::Box<NamespaceDefinition<'tree>>),
    NamespaceUseDeclaration(::std::boxed::Box<NamespaceUseDeclaration<'tree>>),
    ReturnStatement(::std::boxed::Box<ReturnStatement<'tree>>),
    SwitchStatement(::std::boxed::Box<SwitchStatement<'tree>>),
    TraitDeclaration(::std::boxed::Box<TraitDeclaration<'tree>>),
    TryStatement(::std::boxed::Box<TryStatement<'tree>>),
    UnsetStatement(::std::boxed::Box<UnsetStatement<'tree>>),
    WhileStatement(::std::boxed::Box<WhileStatement<'tree>>),
}
impl<'tree> ::treesitter_types::FromNode<'tree> for Statement<'tree> {
    #[allow(clippy::collapsible_else_if)]
    fn from_node(
        node: ::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        match node.kind() {
            "break_statement" => Ok(Self::BreakStatement(::std::boxed::Box::new(
                <BreakStatement as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "class_declaration" => Ok(Self::ClassDeclaration(::std::boxed::Box::new(
                <ClassDeclaration as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "compound_statement" => Ok(Self::CompoundStatement(::std::boxed::Box::new(
                <CompoundStatement as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "const_declaration" => Ok(Self::ConstDeclaration(::std::boxed::Box::new(
                <ConstDeclaration as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "continue_statement" => Ok(Self::ContinueStatement(::std::boxed::Box::new(
                <ContinueStatement as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "declare_statement" => Ok(Self::DeclareStatement(::std::boxed::Box::new(
                <DeclareStatement as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "do_statement" => Ok(Self::DoStatement(::std::boxed::Box::new(
                <DoStatement as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "echo_statement" => Ok(Self::EchoStatement(::std::boxed::Box::new(
                <EchoStatement as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "empty_statement" => Ok(Self::EmptyStatement(::std::boxed::Box::new(
                <EmptyStatement as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "enum_declaration" => Ok(Self::EnumDeclaration(::std::boxed::Box::new(
                <EnumDeclaration as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "exit_statement" => Ok(Self::ExitStatement(::std::boxed::Box::new(
                <ExitStatement as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "expression_statement" => Ok(Self::ExpressionStatement(::std::boxed::Box::new(
                <ExpressionStatement as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "for_statement" => Ok(Self::ForStatement(::std::boxed::Box::new(
                <ForStatement as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "foreach_statement" => Ok(Self::ForeachStatement(::std::boxed::Box::new(
                <ForeachStatement as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "function_definition" => Ok(Self::FunctionDefinition(::std::boxed::Box::new(
                <FunctionDefinition as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "function_static_declaration" => {
                Ok(Self::FunctionStaticDeclaration(::std::boxed::Box::new(
                    <FunctionStaticDeclaration as ::treesitter_types::FromNode>::from_node(
                        node, src,
                    )?,
                )))
            }
            "global_declaration" => Ok(Self::GlobalDeclaration(::std::boxed::Box::new(
                <GlobalDeclaration as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "goto_statement" => Ok(Self::GotoStatement(::std::boxed::Box::new(
                <GotoStatement as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "if_statement" => Ok(Self::IfStatement(::std::boxed::Box::new(
                <IfStatement as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "interface_declaration" => Ok(Self::InterfaceDeclaration(::std::boxed::Box::new(
                <InterfaceDeclaration as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "named_label_statement" => Ok(Self::NamedLabelStatement(::std::boxed::Box::new(
                <NamedLabelStatement as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "namespace_definition" => Ok(Self::NamespaceDefinition(::std::boxed::Box::new(
                <NamespaceDefinition as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "namespace_use_declaration" => {
                Ok(Self::NamespaceUseDeclaration(::std::boxed::Box::new(
                    <NamespaceUseDeclaration as ::treesitter_types::FromNode>::from_node(
                        node, src,
                    )?,
                )))
            }
            "return_statement" => Ok(Self::ReturnStatement(::std::boxed::Box::new(
                <ReturnStatement as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "switch_statement" => Ok(Self::SwitchStatement(::std::boxed::Box::new(
                <SwitchStatement as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "trait_declaration" => Ok(Self::TraitDeclaration(::std::boxed::Box::new(
                <TraitDeclaration as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "try_statement" => Ok(Self::TryStatement(::std::boxed::Box::new(
                <TryStatement as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "unset_statement" => Ok(Self::UnsetStatement(::std::boxed::Box::new(
                <UnsetStatement as ::treesitter_types::FromNode>::from_node(node, src)?,
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
            Self::BreakStatement(inner) => inner.span(),
            Self::ClassDeclaration(inner) => inner.span(),
            Self::CompoundStatement(inner) => inner.span(),
            Self::ConstDeclaration(inner) => inner.span(),
            Self::ContinueStatement(inner) => inner.span(),
            Self::DeclareStatement(inner) => inner.span(),
            Self::DoStatement(inner) => inner.span(),
            Self::EchoStatement(inner) => inner.span(),
            Self::EmptyStatement(inner) => inner.span(),
            Self::EnumDeclaration(inner) => inner.span(),
            Self::ExitStatement(inner) => inner.span(),
            Self::ExpressionStatement(inner) => inner.span(),
            Self::ForStatement(inner) => inner.span(),
            Self::ForeachStatement(inner) => inner.span(),
            Self::FunctionDefinition(inner) => inner.span(),
            Self::FunctionStaticDeclaration(inner) => inner.span(),
            Self::GlobalDeclaration(inner) => inner.span(),
            Self::GotoStatement(inner) => inner.span(),
            Self::IfStatement(inner) => inner.span(),
            Self::InterfaceDeclaration(inner) => inner.span(),
            Self::NamedLabelStatement(inner) => inner.span(),
            Self::NamespaceDefinition(inner) => inner.span(),
            Self::NamespaceUseDeclaration(inner) => inner.span(),
            Self::ReturnStatement(inner) => inner.span(),
            Self::SwitchStatement(inner) => inner.span(),
            Self::TraitDeclaration(inner) => inner.span(),
            Self::TryStatement(inner) => inner.span(),
            Self::UnsetStatement(inner) => inner.span(),
            Self::WhileStatement(inner) => inner.span(),
        }
    }
}
#[derive(Debug, Clone)]
pub enum Type<'tree> {
    DisjunctiveNormalFormType(::std::boxed::Box<DisjunctiveNormalFormType<'tree>>),
    IntersectionType(::std::boxed::Box<IntersectionType<'tree>>),
    NamedType(::std::boxed::Box<NamedType<'tree>>),
    OptionalType(::std::boxed::Box<OptionalType<'tree>>),
    PrimitiveType(::std::boxed::Box<PrimitiveType<'tree>>),
    UnionType(::std::boxed::Box<UnionType<'tree>>),
}
impl<'tree> ::treesitter_types::FromNode<'tree> for Type<'tree> {
    #[allow(clippy::collapsible_else_if)]
    fn from_node(
        node: ::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        match node.kind() {
            "disjunctive_normal_form_type" => {
                Ok(Self::DisjunctiveNormalFormType(::std::boxed::Box::new(
                    <DisjunctiveNormalFormType as ::treesitter_types::FromNode>::from_node(
                        node, src,
                    )?,
                )))
            }
            "intersection_type" => Ok(Self::IntersectionType(::std::boxed::Box::new(
                <IntersectionType as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "named_type" => Ok(Self::NamedType(::std::boxed::Box::new(
                <NamedType as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "optional_type" => Ok(Self::OptionalType(::std::boxed::Box::new(
                <OptionalType as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "primitive_type" => Ok(Self::PrimitiveType(::std::boxed::Box::new(
                <PrimitiveType as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "union_type" => Ok(Self::UnionType(::std::boxed::Box::new(
                <UnionType as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            other => Err(::treesitter_types::ParseError::unexpected_kind(other, node)),
        }
    }
}
impl ::treesitter_types::Spanned for Type<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        match self {
            Self::DisjunctiveNormalFormType(inner) => inner.span(),
            Self::IntersectionType(inner) => inner.span(),
            Self::NamedType(inner) => inner.span(),
            Self::OptionalType(inner) => inner.span(),
            Self::PrimitiveType(inner) => inner.span(),
            Self::UnionType(inner) => inner.span(),
        }
    }
}
#[derive(Debug, Clone)]
pub struct AbstractModifier<'tree> {
    pub span: ::treesitter_types::Span,
    text: &'tree str,
}
impl<'tree> ::treesitter_types::FromNode<'tree> for AbstractModifier<'tree> {
    fn from_node(
        node: ::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        debug_assert_eq!(node.kind(), "abstract_modifier");
        Ok(Self {
            span: ::treesitter_types::Span::from(node),
            text: node.utf8_text(src)?,
        })
    }
}
impl<'tree> ::treesitter_types::LeafNode<'tree> for AbstractModifier<'tree> {
    fn text(&self) -> &'tree str {
        self.text
    }
}
impl ::treesitter_types::Spanned for AbstractModifier<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        self.span
    }
}
#[derive(Debug, Clone)]
pub struct AnonymousClass<'tree> {
    pub span: ::treesitter_types::Span,
    pub attributes: ::core::option::Option<AttributeList<'tree>>,
    pub body: DeclarationList<'tree>,
    pub children: ::std::vec::Vec<AnonymousClassChildren<'tree>>,
}
impl<'tree> ::treesitter_types::FromNode<'tree> for AnonymousClass<'tree> {
    #[allow(clippy::match_single_binding, clippy::suspicious_else_formatting)]
    fn from_node(
        node: ::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        debug_assert_eq!(node.kind(), "anonymous_class");
        Ok(Self {
            span: ::treesitter_types::Span::from(node),
            attributes: match node.child_by_field_name("attributes") {
                Some(child) => Some(<AttributeList as ::treesitter_types::FromNode>::from_node(
                    child, src,
                )?),
                None => None,
            },
            body: {
                let child = node
                    .child_by_field_name("body")
                    .ok_or_else(|| ::treesitter_types::ParseError::missing_field("body", node))?;
                <DeclarationList as ::treesitter_types::FromNode>::from_node(child, src)?
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
                        <AnonymousClassChildren as ::treesitter_types::FromNode>::from_node(
                            child, src,
                        )?,
                    );
                }
                items
            },
        })
    }
}
impl ::treesitter_types::Spanned for AnonymousClass<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        self.span
    }
}
#[derive(Debug, Clone)]
pub struct AnonymousFunction<'tree> {
    pub span: ::treesitter_types::Span,
    pub attributes: ::core::option::Option<AttributeList<'tree>>,
    pub body: CompoundStatement<'tree>,
    pub parameters: FormalParameters<'tree>,
    pub reference_modifier: ::core::option::Option<ReferenceModifier<'tree>>,
    pub return_type: ::core::option::Option<AnonymousFunctionReturnType<'tree>>,
    pub static_modifier: ::core::option::Option<StaticModifier<'tree>>,
    pub children: ::core::option::Option<AnonymousFunctionUseClause<'tree>>,
}
impl<'tree> ::treesitter_types::FromNode<'tree> for AnonymousFunction<'tree> {
    #[allow(clippy::match_single_binding, clippy::suspicious_else_formatting)]
    fn from_node(
        node: ::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        debug_assert_eq!(node.kind(), "anonymous_function");
        Ok(Self {
            span: ::treesitter_types::Span::from(node),
            attributes: match node.child_by_field_name("attributes") {
                Some(child) => Some(<AttributeList as ::treesitter_types::FromNode>::from_node(
                    child, src,
                )?),
                None => None,
            },
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
                <FormalParameters as ::treesitter_types::FromNode>::from_node(child, src)?
            },
            reference_modifier: match node.child_by_field_name("reference_modifier") {
                Some(child) => Some(
                    <ReferenceModifier as ::treesitter_types::FromNode>::from_node(child, src)?,
                ),
                None => None,
            },
            return_type: match node.child_by_field_name("return_type") {
                Some(child) => Some(
                    <AnonymousFunctionReturnType as ::treesitter_types::FromNode>::from_node(
                        child, src,
                    )?,
                ),
                None => None,
            },
            static_modifier: match node.child_by_field_name("static_modifier") {
                Some(child) => Some(<StaticModifier as ::treesitter_types::FromNode>::from_node(
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
                        <AnonymousFunctionUseClause as ::treesitter_types::FromNode>::from_node(
                            child, src,
                        )?,
                    ),
                    None => None,
                }
            },
        })
    }
}
impl ::treesitter_types::Spanned for AnonymousFunction<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        self.span
    }
}
#[derive(Debug, Clone)]
pub struct AnonymousFunctionUseClause<'tree> {
    pub span: ::treesitter_types::Span,
    pub children: ::std::vec::Vec<AnonymousFunctionUseClauseChildren<'tree>>,
}
impl<'tree> ::treesitter_types::FromNode<'tree> for AnonymousFunctionUseClause<'tree> {
    #[allow(clippy::match_single_binding, clippy::suspicious_else_formatting)]
    fn from_node(
        node: ::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        debug_assert_eq!(node.kind(), "anonymous_function_use_clause");
        Ok(Self {
            span: ::treesitter_types::Span::from(node),
            children: {
                #[allow(clippy::suspicious_else_formatting)]
                let non_field_children = {
                    let mut cursor = node.walk();
                    let mut result = ::std::vec::Vec::new();
                    if cursor.goto_first_child() {
                        loop {
                            if cursor.field_name().is_none()
                                && cursor.node().is_named()
                                && !cursor.node().is_extra()
                            {
                                result.push(cursor.node());
                            }
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
                            <AnonymousFunctionUseClauseChildren as ::treesitter_types::FromNode>::from_node(
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
impl ::treesitter_types::Spanned for AnonymousFunctionUseClause<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        self.span
    }
}
#[derive(Debug, Clone)]
pub struct Argument<'tree> {
    pub span: ::treesitter_types::Span,
    pub name: ::core::option::Option<Name<'tree>>,
    pub reference_modifier: ::core::option::Option<ReferenceModifier<'tree>>,
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
                Some(child) => Some(<Name as ::treesitter_types::FromNode>::from_node(
                    child, src,
                )?),
                None => None,
            },
            reference_modifier: match node.child_by_field_name("reference_modifier") {
                Some(child) => Some(
                    <ReferenceModifier as ::treesitter_types::FromNode>::from_node(child, src)?,
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
                                        <ArgumentChildren as ::treesitter_types::FromNode>::from_node(
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
                    items.push(
                        <ArgumentsChildren as ::treesitter_types::FromNode>::from_node(child, src)?,
                    );
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
pub struct ArrayCreationExpression<'tree> {
    pub span: ::treesitter_types::Span,
    pub children: ::std::vec::Vec<ArrayElementInitializer<'tree>>,
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
            children: {
                #[allow(clippy::suspicious_else_formatting)]
                let non_field_children = {
                    let mut cursor = node.walk();
                    let mut result = ::std::vec::Vec::new();
                    if cursor.goto_first_child() {
                        loop {
                            if cursor.field_name().is_none()
                                && cursor.node().is_named()
                                && !cursor.node().is_extra()
                            {
                                result.push(cursor.node());
                            }
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
                        <ArrayElementInitializer as ::treesitter_types::FromNode>::from_node(
                            child, src,
                        )?,
                    );
                }
                items
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
pub struct ArrayElementInitializer<'tree> {
    pub span: ::treesitter_types::Span,
    pub children: ::std::vec::Vec<ArrayElementInitializerChildren<'tree>>,
}
impl<'tree> ::treesitter_types::FromNode<'tree> for ArrayElementInitializer<'tree> {
    #[allow(clippy::match_single_binding, clippy::suspicious_else_formatting)]
    fn from_node(
        node: ::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        debug_assert_eq!(node.kind(), "array_element_initializer");
        Ok(Self {
            span: ::treesitter_types::Span::from(node),
            children: {
                #[allow(clippy::suspicious_else_formatting)]
                let non_field_children = {
                    let mut cursor = node.walk();
                    let mut result = ::std::vec::Vec::new();
                    if cursor.goto_first_child() {
                        loop {
                            if cursor.field_name().is_none()
                                && cursor.node().is_named()
                                && !cursor.node().is_extra()
                            {
                                result.push(cursor.node());
                            }
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
                            <ArrayElementInitializerChildren as ::treesitter_types::FromNode>::from_node(
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
impl ::treesitter_types::Spanned for ArrayElementInitializer<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        self.span
    }
}
#[derive(Debug, Clone)]
pub struct ArrowFunction<'tree> {
    pub span: ::treesitter_types::Span,
    pub attributes: ::core::option::Option<AttributeList<'tree>>,
    pub body: Expression<'tree>,
    pub parameters: FormalParameters<'tree>,
    pub reference_modifier: ::core::option::Option<ReferenceModifier<'tree>>,
    pub return_type: ::core::option::Option<ArrowFunctionReturnType<'tree>>,
    pub static_modifier: ::core::option::Option<StaticModifier<'tree>>,
}
impl<'tree> ::treesitter_types::FromNode<'tree> for ArrowFunction<'tree> {
    #[allow(clippy::match_single_binding, clippy::suspicious_else_formatting)]
    fn from_node(
        node: ::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        debug_assert_eq!(node.kind(), "arrow_function");
        Ok(Self {
            span: ::treesitter_types::Span::from(node),
            attributes: match node.child_by_field_name("attributes") {
                Some(child) => Some(<AttributeList as ::treesitter_types::FromNode>::from_node(
                    child, src,
                )?),
                None => None,
            },
            body: {
                let child = node
                    .child_by_field_name("body")
                    .ok_or_else(|| ::treesitter_types::ParseError::missing_field("body", node))?;
                <Expression as ::treesitter_types::FromNode>::from_node(child, src)?
            },
            parameters: {
                let child = node.child_by_field_name("parameters").ok_or_else(|| {
                    ::treesitter_types::ParseError::missing_field("parameters", node)
                })?;
                <FormalParameters as ::treesitter_types::FromNode>::from_node(child, src)?
            },
            reference_modifier: match node.child_by_field_name("reference_modifier") {
                Some(child) => Some(
                    <ReferenceModifier as ::treesitter_types::FromNode>::from_node(child, src)?,
                ),
                None => None,
            },
            return_type: match node.child_by_field_name("return_type") {
                Some(child) => Some(
                    <ArrowFunctionReturnType as ::treesitter_types::FromNode>::from_node(
                        child, src,
                    )?,
                ),
                None => None,
            },
            static_modifier: match node.child_by_field_name("static_modifier") {
                Some(child) => Some(<StaticModifier as ::treesitter_types::FromNode>::from_node(
                    child, src,
                )?),
                None => None,
            },
        })
    }
}
impl ::treesitter_types::Spanned for ArrowFunction<'_> {
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
pub struct Attribute<'tree> {
    pub span: ::treesitter_types::Span,
    pub parameters: ::core::option::Option<Arguments<'tree>>,
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
            parameters: match node.child_by_field_name("parameters") {
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
                                        <AttributeChildren as ::treesitter_types::FromNode>::from_node(
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
                <AttributeChildren as ::treesitter_types::FromNode>::from_node(child, src)?
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
pub struct AttributeGroup<'tree> {
    pub span: ::treesitter_types::Span,
    pub children: ::std::vec::Vec<Attribute<'tree>>,
}
impl<'tree> ::treesitter_types::FromNode<'tree> for AttributeGroup<'tree> {
    #[allow(clippy::match_single_binding, clippy::suspicious_else_formatting)]
    fn from_node(
        node: ::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        debug_assert_eq!(node.kind(), "attribute_group");
        Ok(Self {
            span: ::treesitter_types::Span::from(node),
            children: {
                #[allow(clippy::suspicious_else_formatting)]
                let non_field_children = {
                    let mut cursor = node.walk();
                    let mut result = ::std::vec::Vec::new();
                    if cursor.goto_first_child() {
                        loop {
                            if cursor.field_name().is_none()
                                && cursor.node().is_named()
                                && !cursor.node().is_extra()
                            {
                                result.push(cursor.node());
                            }
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
impl ::treesitter_types::Spanned for AttributeGroup<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        self.span
    }
}
#[derive(Debug, Clone)]
pub struct AttributeList<'tree> {
    pub span: ::treesitter_types::Span,
    pub children: ::std::vec::Vec<AttributeGroup<'tree>>,
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
                    items.push(<AttributeGroup as ::treesitter_types::FromNode>::from_node(
                        child, src,
                    )?);
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
pub struct AugmentedAssignmentExpression<'tree> {
    pub span: ::treesitter_types::Span,
    pub left: AugmentedAssignmentExpressionLeft<'tree>,
    pub operator: AugmentedAssignmentExpressionOperator,
    pub right: Expression<'tree>,
}
impl<'tree> ::treesitter_types::FromNode<'tree> for AugmentedAssignmentExpression<'tree> {
    #[allow(clippy::match_single_binding, clippy::suspicious_else_formatting)]
    fn from_node(
        node: ::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        debug_assert_eq!(node.kind(), "augmented_assignment_expression");
        Ok(Self {
            span: ::treesitter_types::Span::from(node),
            left: {
                let child = node
                    .child_by_field_name("left")
                    .ok_or_else(|| ::treesitter_types::ParseError::missing_field("left", node))?;
                <AugmentedAssignmentExpressionLeft as ::treesitter_types::FromNode>::from_node(
                    child, src,
                )?
            },
            operator: {
                let child = node.child_by_field_name("operator").ok_or_else(|| {
                    ::treesitter_types::ParseError::missing_field("operator", node)
                })?;
                <AugmentedAssignmentExpressionOperator as ::treesitter_types::FromNode>::from_node(
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
impl ::treesitter_types::Spanned for AugmentedAssignmentExpression<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        self.span
    }
}
#[derive(Debug, Clone)]
pub struct BaseClause<'tree> {
    pub span: ::treesitter_types::Span,
    pub children: ::std::vec::Vec<BaseClauseChildren<'tree>>,
}
impl<'tree> ::treesitter_types::FromNode<'tree> for BaseClause<'tree> {
    #[allow(clippy::match_single_binding, clippy::suspicious_else_formatting)]
    fn from_node(
        node: ::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        debug_assert_eq!(node.kind(), "base_clause");
        Ok(Self {
            span: ::treesitter_types::Span::from(node),
            children: {
                #[allow(clippy::suspicious_else_formatting)]
                let non_field_children = {
                    let mut cursor = node.walk();
                    let mut result = ::std::vec::Vec::new();
                    if cursor.goto_first_child() {
                        loop {
                            if cursor.field_name().is_none()
                                && cursor.node().is_named()
                                && !cursor.node().is_extra()
                            {
                                result.push(cursor.node());
                            }
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
                        <BaseClauseChildren as ::treesitter_types::FromNode>::from_node(
                            child, src,
                        )?,
                    );
                }
                items
            },
        })
    }
}
impl ::treesitter_types::Spanned for BaseClause<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        self.span
    }
}
#[derive(Debug, Clone)]
pub struct BinaryExpression<'tree> {
    pub span: ::treesitter_types::Span,
    pub left: Expression<'tree>,
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
pub struct Boolean<'tree> {
    pub span: ::treesitter_types::Span,
    text: &'tree str,
}
impl<'tree> ::treesitter_types::FromNode<'tree> for Boolean<'tree> {
    fn from_node(
        node: ::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        debug_assert_eq!(node.kind(), "boolean");
        Ok(Self {
            span: ::treesitter_types::Span::from(node),
            text: node.utf8_text(src)?,
        })
    }
}
impl<'tree> ::treesitter_types::LeafNode<'tree> for Boolean<'tree> {
    fn text(&self) -> &'tree str {
        self.text
    }
}
impl ::treesitter_types::Spanned for Boolean<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        self.span
    }
}
#[derive(Debug, Clone)]
pub struct BreakStatement<'tree> {
    pub span: ::treesitter_types::Span,
    pub children: ::core::option::Option<Expression<'tree>>,
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
                            if cursor.field_name().is_none()
                                && cursor.node().is_named()
                                && !cursor.node().is_extra()
                            {
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
impl ::treesitter_types::Spanned for BreakStatement<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        self.span
    }
}
#[derive(Debug, Clone)]
pub struct ByRef<'tree> {
    pub span: ::treesitter_types::Span,
    pub children: ByRefChildren<'tree>,
}
impl<'tree> ::treesitter_types::FromNode<'tree> for ByRef<'tree> {
    #[allow(clippy::match_single_binding, clippy::suspicious_else_formatting)]
    fn from_node(
        node: ::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        debug_assert_eq!(node.kind(), "by_ref");
        Ok(Self {
            span: ::treesitter_types::Span::from(node),
            children: {
                #[allow(clippy::suspicious_else_formatting)]
                let non_field_children = {
                    let mut cursor = node.walk();
                    let mut result = ::std::vec::Vec::new();
                    if cursor.goto_first_child() {
                        loop {
                            if cursor.field_name().is_none()
                                && cursor.node().is_named()
                                && !cursor.node().is_extra()
                            {
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
                                        <ByRefChildren as ::treesitter_types::FromNode>::from_node(
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
                <ByRefChildren as ::treesitter_types::FromNode>::from_node(child, src)?
            },
        })
    }
}
impl ::treesitter_types::Spanned for ByRef<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        self.span
    }
}
#[derive(Debug, Clone)]
pub struct CaseStatement<'tree> {
    pub span: ::treesitter_types::Span,
    pub value: Expression<'tree>,
    pub children: ::std::vec::Vec<Statement<'tree>>,
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
                    items.push(<Statement as ::treesitter_types::FromNode>::from_node(
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
pub struct CastExpression<'tree> {
    pub span: ::treesitter_types::Span,
    pub r#type: CastType<'tree>,
    pub value: CastExpressionValue<'tree>,
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
                <CastType as ::treesitter_types::FromNode>::from_node(child, src)?
            },
            value: {
                let child = node
                    .child_by_field_name("value")
                    .ok_or_else(|| ::treesitter_types::ParseError::missing_field("value", node))?;
                <CastExpressionValue as ::treesitter_types::FromNode>::from_node(child, src)?
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
pub struct CastType<'tree> {
    pub span: ::treesitter_types::Span,
    text: &'tree str,
}
impl<'tree> ::treesitter_types::FromNode<'tree> for CastType<'tree> {
    fn from_node(
        node: ::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        debug_assert_eq!(node.kind(), "cast_type");
        Ok(Self {
            span: ::treesitter_types::Span::from(node),
            text: node.utf8_text(src)?,
        })
    }
}
impl<'tree> ::treesitter_types::LeafNode<'tree> for CastType<'tree> {
    fn text(&self) -> &'tree str {
        self.text
    }
}
impl ::treesitter_types::Spanned for CastType<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        self.span
    }
}
#[derive(Debug, Clone)]
pub struct CatchClause<'tree> {
    pub span: ::treesitter_types::Span,
    pub body: CompoundStatement<'tree>,
    pub name: ::core::option::Option<VariableName<'tree>>,
    pub r#type: TypeList<'tree>,
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
            name: match node.child_by_field_name("name") {
                Some(child) => Some(<VariableName as ::treesitter_types::FromNode>::from_node(
                    child, src,
                )?),
                None => None,
            },
            r#type: {
                let child = node
                    .child_by_field_name("type")
                    .ok_or_else(|| ::treesitter_types::ParseError::missing_field("type", node))?;
                <TypeList as ::treesitter_types::FromNode>::from_node(child, src)?
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
pub struct ClassConstantAccessExpression<'tree> {
    pub span: ::treesitter_types::Span,
    pub children: ::std::vec::Vec<ClassConstantAccessExpressionChildren<'tree>>,
}
impl<'tree> ::treesitter_types::FromNode<'tree> for ClassConstantAccessExpression<'tree> {
    #[allow(clippy::match_single_binding, clippy::suspicious_else_formatting)]
    fn from_node(
        node: ::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        debug_assert_eq!(node.kind(), "class_constant_access_expression");
        Ok(Self {
            span: ::treesitter_types::Span::from(node),
            children: {
                #[allow(clippy::suspicious_else_formatting)]
                let non_field_children = {
                    let mut cursor = node.walk();
                    let mut result = ::std::vec::Vec::new();
                    if cursor.goto_first_child() {
                        loop {
                            if cursor.field_name().is_none()
                                && cursor.node().is_named()
                                && !cursor.node().is_extra()
                            {
                                result.push(cursor.node());
                            }
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
                            <ClassConstantAccessExpressionChildren as ::treesitter_types::FromNode>::from_node(
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
impl ::treesitter_types::Spanned for ClassConstantAccessExpression<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        self.span
    }
}
#[derive(Debug, Clone)]
pub struct ClassDeclaration<'tree> {
    pub span: ::treesitter_types::Span,
    pub attributes: ::core::option::Option<AttributeList<'tree>>,
    pub body: DeclarationList<'tree>,
    pub name: Name<'tree>,
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
            attributes: match node.child_by_field_name("attributes") {
                Some(child) => Some(<AttributeList as ::treesitter_types::FromNode>::from_node(
                    child, src,
                )?),
                None => None,
            },
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
                <Name as ::treesitter_types::FromNode>::from_node(child, src)?
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
pub struct ClassInterfaceClause<'tree> {
    pub span: ::treesitter_types::Span,
    pub children: ::std::vec::Vec<ClassInterfaceClauseChildren<'tree>>,
}
impl<'tree> ::treesitter_types::FromNode<'tree> for ClassInterfaceClause<'tree> {
    #[allow(clippy::match_single_binding, clippy::suspicious_else_formatting)]
    fn from_node(
        node: ::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        debug_assert_eq!(node.kind(), "class_interface_clause");
        Ok(Self {
            span: ::treesitter_types::Span::from(node),
            children: {
                #[allow(clippy::suspicious_else_formatting)]
                let non_field_children = {
                    let mut cursor = node.walk();
                    let mut result = ::std::vec::Vec::new();
                    if cursor.goto_first_child() {
                        loop {
                            if cursor.field_name().is_none()
                                && cursor.node().is_named()
                                && !cursor.node().is_extra()
                            {
                                result.push(cursor.node());
                            }
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
                        <ClassInterfaceClauseChildren as ::treesitter_types::FromNode>::from_node(
                            child, src,
                        )?,
                    );
                }
                items
            },
        })
    }
}
impl ::treesitter_types::Spanned for ClassInterfaceClause<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        self.span
    }
}
#[derive(Debug, Clone)]
pub struct CloneExpression<'tree> {
    pub span: ::treesitter_types::Span,
    pub children: PrimaryExpression<'tree>,
}
impl<'tree> ::treesitter_types::FromNode<'tree> for CloneExpression<'tree> {
    #[allow(clippy::match_single_binding, clippy::suspicious_else_formatting)]
    fn from_node(
        node: ::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        debug_assert_eq!(node.kind(), "clone_expression");
        Ok(Self {
            span: ::treesitter_types::Span::from(node),
            children: {
                #[allow(clippy::suspicious_else_formatting)]
                let non_field_children = {
                    let mut cursor = node.walk();
                    let mut result = ::std::vec::Vec::new();
                    if cursor.goto_first_child() {
                        loop {
                            if cursor.field_name().is_none()
                                && cursor.node().is_named()
                                && !cursor.node().is_extra()
                            {
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
impl ::treesitter_types::Spanned for CloneExpression<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        self.span
    }
}
#[derive(Debug, Clone)]
pub struct ColonBlock<'tree> {
    pub span: ::treesitter_types::Span,
    pub children: ::std::vec::Vec<Statement<'tree>>,
}
impl<'tree> ::treesitter_types::FromNode<'tree> for ColonBlock<'tree> {
    #[allow(clippy::match_single_binding, clippy::suspicious_else_formatting)]
    fn from_node(
        node: ::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        debug_assert_eq!(node.kind(), "colon_block");
        Ok(Self {
            span: ::treesitter_types::Span::from(node),
            children: {
                #[allow(clippy::suspicious_else_formatting)]
                let non_field_children = {
                    let mut cursor = node.walk();
                    let mut result = ::std::vec::Vec::new();
                    if cursor.goto_first_child() {
                        loop {
                            if cursor.field_name().is_none()
                                && cursor.node().is_named()
                                && !cursor.node().is_extra()
                            {
                                result.push(cursor.node());
                            }
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
impl ::treesitter_types::Spanned for ColonBlock<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        self.span
    }
}
#[derive(Debug, Clone)]
pub struct CompoundStatement<'tree> {
    pub span: ::treesitter_types::Span,
    pub children: ::std::vec::Vec<Statement<'tree>>,
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
                    items.push(<Statement as ::treesitter_types::FromNode>::from_node(
                        child, src,
                    )?);
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
pub struct ConditionalExpression<'tree> {
    pub span: ::treesitter_types::Span,
    pub alternative: Expression<'tree>,
    pub body: ::core::option::Option<Expression<'tree>>,
    pub condition: Expression<'tree>,
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
            body: match node.child_by_field_name("body") {
                Some(child) => Some(<Expression as ::treesitter_types::FromNode>::from_node(
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
impl ::treesitter_types::Spanned for ConditionalExpression<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        self.span
    }
}
#[derive(Debug, Clone)]
pub struct ConstDeclaration<'tree> {
    pub span: ::treesitter_types::Span,
    pub attributes: ::core::option::Option<AttributeList<'tree>>,
    pub r#type: ::core::option::Option<Type<'tree>>,
    pub children: ::std::vec::Vec<ConstDeclarationChildren<'tree>>,
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
            attributes: match node.child_by_field_name("attributes") {
                Some(child) => Some(<AttributeList as ::treesitter_types::FromNode>::from_node(
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
                        <ConstDeclarationChildren as ::treesitter_types::FromNode>::from_node(
                            child, src,
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
pub struct ConstElement<'tree> {
    pub span: ::treesitter_types::Span,
    pub children: ::std::vec::Vec<ConstElementChildren<'tree>>,
}
impl<'tree> ::treesitter_types::FromNode<'tree> for ConstElement<'tree> {
    #[allow(clippy::match_single_binding, clippy::suspicious_else_formatting)]
    fn from_node(
        node: ::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        debug_assert_eq!(node.kind(), "const_element");
        Ok(Self {
            span: ::treesitter_types::Span::from(node),
            children: {
                #[allow(clippy::suspicious_else_formatting)]
                let non_field_children = {
                    let mut cursor = node.walk();
                    let mut result = ::std::vec::Vec::new();
                    if cursor.goto_first_child() {
                        loop {
                            if cursor.field_name().is_none()
                                && cursor.node().is_named()
                                && !cursor.node().is_extra()
                            {
                                result.push(cursor.node());
                            }
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
                        <ConstElementChildren as ::treesitter_types::FromNode>::from_node(
                            child, src,
                        )?,
                    );
                }
                items
            },
        })
    }
}
impl ::treesitter_types::Spanned for ConstElement<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        self.span
    }
}
#[derive(Debug, Clone)]
pub struct ContinueStatement<'tree> {
    pub span: ::treesitter_types::Span,
    pub children: ::core::option::Option<Expression<'tree>>,
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
                            if cursor.field_name().is_none()
                                && cursor.node().is_named()
                                && !cursor.node().is_extra()
                            {
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
impl ::treesitter_types::Spanned for ContinueStatement<'_> {
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
pub struct DeclareDirective<'tree> {
    pub span: ::treesitter_types::Span,
    pub children: Literal<'tree>,
}
impl<'tree> ::treesitter_types::FromNode<'tree> for DeclareDirective<'tree> {
    #[allow(clippy::match_single_binding, clippy::suspicious_else_formatting)]
    fn from_node(
        node: ::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        debug_assert_eq!(node.kind(), "declare_directive");
        Ok(Self {
            span: ::treesitter_types::Span::from(node),
            children: {
                #[allow(clippy::suspicious_else_formatting)]
                let non_field_children = {
                    let mut cursor = node.walk();
                    let mut result = ::std::vec::Vec::new();
                    if cursor.goto_first_child() {
                        loop {
                            if cursor.field_name().is_none()
                                && cursor.node().is_named()
                                && !cursor.node().is_extra()
                            {
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
                                        <Literal as ::treesitter_types::FromNode>::from_node(
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
                <Literal as ::treesitter_types::FromNode>::from_node(child, src)?
            },
        })
    }
}
impl ::treesitter_types::Spanned for DeclareDirective<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        self.span
    }
}
#[derive(Debug, Clone)]
pub struct DeclareStatement<'tree> {
    pub span: ::treesitter_types::Span,
    pub children: ::std::vec::Vec<DeclareStatementChildren<'tree>>,
}
impl<'tree> ::treesitter_types::FromNode<'tree> for DeclareStatement<'tree> {
    #[allow(clippy::match_single_binding, clippy::suspicious_else_formatting)]
    fn from_node(
        node: ::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        debug_assert_eq!(node.kind(), "declare_statement");
        Ok(Self {
            span: ::treesitter_types::Span::from(node),
            children: {
                #[allow(clippy::suspicious_else_formatting)]
                let non_field_children = {
                    let mut cursor = node.walk();
                    let mut result = ::std::vec::Vec::new();
                    if cursor.goto_first_child() {
                        loop {
                            if cursor.field_name().is_none()
                                && cursor.node().is_named()
                                && !cursor.node().is_extra()
                            {
                                result.push(cursor.node());
                            }
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
                        <DeclareStatementChildren as ::treesitter_types::FromNode>::from_node(
                            child, src,
                        )?,
                    );
                }
                items
            },
        })
    }
}
impl ::treesitter_types::Spanned for DeclareStatement<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        self.span
    }
}
#[derive(Debug, Clone)]
pub struct DefaultStatement<'tree> {
    pub span: ::treesitter_types::Span,
    pub children: ::std::vec::Vec<Statement<'tree>>,
}
impl<'tree> ::treesitter_types::FromNode<'tree> for DefaultStatement<'tree> {
    #[allow(clippy::match_single_binding, clippy::suspicious_else_formatting)]
    fn from_node(
        node: ::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        debug_assert_eq!(node.kind(), "default_statement");
        Ok(Self {
            span: ::treesitter_types::Span::from(node),
            children: {
                #[allow(clippy::suspicious_else_formatting)]
                let non_field_children = {
                    let mut cursor = node.walk();
                    let mut result = ::std::vec::Vec::new();
                    if cursor.goto_first_child() {
                        loop {
                            if cursor.field_name().is_none()
                                && cursor.node().is_named()
                                && !cursor.node().is_extra()
                            {
                                result.push(cursor.node());
                            }
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
impl ::treesitter_types::Spanned for DefaultStatement<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        self.span
    }
}
#[derive(Debug, Clone)]
pub struct DisjunctiveNormalFormType<'tree> {
    pub span: ::treesitter_types::Span,
    pub children: ::std::vec::Vec<DisjunctiveNormalFormTypeChildren<'tree>>,
}
impl<'tree> ::treesitter_types::FromNode<'tree> for DisjunctiveNormalFormType<'tree> {
    #[allow(clippy::match_single_binding, clippy::suspicious_else_formatting)]
    fn from_node(
        node: ::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        debug_assert_eq!(node.kind(), "disjunctive_normal_form_type");
        Ok(Self {
            span: ::treesitter_types::Span::from(node),
            children: {
                #[allow(clippy::suspicious_else_formatting)]
                let non_field_children = {
                    let mut cursor = node.walk();
                    let mut result = ::std::vec::Vec::new();
                    if cursor.goto_first_child() {
                        loop {
                            if cursor.field_name().is_none()
                                && cursor.node().is_named()
                                && !cursor.node().is_extra()
                            {
                                result.push(cursor.node());
                            }
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
                            <DisjunctiveNormalFormTypeChildren as ::treesitter_types::FromNode>::from_node(
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
impl ::treesitter_types::Spanned for DisjunctiveNormalFormType<'_> {
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
pub struct DynamicVariableName<'tree> {
    pub span: ::treesitter_types::Span,
    pub children: DynamicVariableNameChildren<'tree>,
}
impl<'tree> ::treesitter_types::FromNode<'tree> for DynamicVariableName<'tree> {
    #[allow(clippy::match_single_binding, clippy::suspicious_else_formatting)]
    fn from_node(
        node: ::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        debug_assert_eq!(node.kind(), "dynamic_variable_name");
        Ok(Self {
            span: ::treesitter_types::Span::from(node),
            children: {
                #[allow(clippy::suspicious_else_formatting)]
                let non_field_children = {
                    let mut cursor = node.walk();
                    let mut result = ::std::vec::Vec::new();
                    if cursor.goto_first_child() {
                        loop {
                            if cursor.field_name().is_none()
                                && cursor.node().is_named()
                                && !cursor.node().is_extra()
                            {
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
                                        <DynamicVariableNameChildren as ::treesitter_types::FromNode>::from_node(
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
                <DynamicVariableNameChildren as ::treesitter_types::FromNode>::from_node(
                    child, src,
                )?
            },
        })
    }
}
impl ::treesitter_types::Spanned for DynamicVariableName<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        self.span
    }
}
#[derive(Debug, Clone)]
pub struct EchoStatement<'tree> {
    pub span: ::treesitter_types::Span,
    pub children: EchoStatementChildren<'tree>,
}
impl<'tree> ::treesitter_types::FromNode<'tree> for EchoStatement<'tree> {
    #[allow(clippy::match_single_binding, clippy::suspicious_else_formatting)]
    fn from_node(
        node: ::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        debug_assert_eq!(node.kind(), "echo_statement");
        Ok(Self {
            span: ::treesitter_types::Span::from(node),
            children: {
                #[allow(clippy::suspicious_else_formatting)]
                let non_field_children = {
                    let mut cursor = node.walk();
                    let mut result = ::std::vec::Vec::new();
                    if cursor.goto_first_child() {
                        loop {
                            if cursor.field_name().is_none()
                                && cursor.node().is_named()
                                && !cursor.node().is_extra()
                            {
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
                                        <EchoStatementChildren as ::treesitter_types::FromNode>::from_node(
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
                <EchoStatementChildren as ::treesitter_types::FromNode>::from_node(child, src)?
            },
        })
    }
}
impl ::treesitter_types::Spanned for EchoStatement<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        self.span
    }
}
#[derive(Debug, Clone)]
pub struct ElseClause<'tree> {
    pub span: ::treesitter_types::Span,
    pub body: ElseClauseBody<'tree>,
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
                <ElseClauseBody as ::treesitter_types::FromNode>::from_node(child, src)?
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
pub struct ElseIfClause<'tree> {
    pub span: ::treesitter_types::Span,
    pub body: ElseIfClauseBody<'tree>,
    pub condition: ParenthesizedExpression<'tree>,
}
impl<'tree> ::treesitter_types::FromNode<'tree> for ElseIfClause<'tree> {
    #[allow(clippy::match_single_binding, clippy::suspicious_else_formatting)]
    fn from_node(
        node: ::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        debug_assert_eq!(node.kind(), "else_if_clause");
        Ok(Self {
            span: ::treesitter_types::Span::from(node),
            body: {
                let child = node
                    .child_by_field_name("body")
                    .ok_or_else(|| ::treesitter_types::ParseError::missing_field("body", node))?;
                <ElseIfClauseBody as ::treesitter_types::FromNode>::from_node(child, src)?
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
impl ::treesitter_types::Spanned for ElseIfClause<'_> {
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
pub struct EncapsedString<'tree> {
    pub span: ::treesitter_types::Span,
    pub children: ::std::vec::Vec<EncapsedStringChildren<'tree>>,
}
impl<'tree> ::treesitter_types::FromNode<'tree> for EncapsedString<'tree> {
    #[allow(clippy::match_single_binding, clippy::suspicious_else_formatting)]
    fn from_node(
        node: ::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        debug_assert_eq!(node.kind(), "encapsed_string");
        Ok(Self {
            span: ::treesitter_types::Span::from(node),
            children: {
                #[allow(clippy::suspicious_else_formatting)]
                let non_field_children = {
                    let mut cursor = node.walk();
                    let mut result = ::std::vec::Vec::new();
                    if cursor.goto_first_child() {
                        loop {
                            if cursor.field_name().is_none()
                                && cursor.node().is_named()
                                && !cursor.node().is_extra()
                            {
                                result.push(cursor.node());
                            }
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
                        <EncapsedStringChildren as ::treesitter_types::FromNode>::from_node(
                            child, src,
                        )?,
                    );
                }
                items
            },
        })
    }
}
impl ::treesitter_types::Spanned for EncapsedString<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        self.span
    }
}
#[derive(Debug, Clone)]
pub struct EnumCase<'tree> {
    pub span: ::treesitter_types::Span,
    pub attributes: ::core::option::Option<AttributeList<'tree>>,
    pub name: Name<'tree>,
    pub value: ::core::option::Option<Expression<'tree>>,
}
impl<'tree> ::treesitter_types::FromNode<'tree> for EnumCase<'tree> {
    #[allow(clippy::match_single_binding, clippy::suspicious_else_formatting)]
    fn from_node(
        node: ::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        debug_assert_eq!(node.kind(), "enum_case");
        Ok(Self {
            span: ::treesitter_types::Span::from(node),
            attributes: match node.child_by_field_name("attributes") {
                Some(child) => Some(<AttributeList as ::treesitter_types::FromNode>::from_node(
                    child, src,
                )?),
                None => None,
            },
            name: {
                let child = node
                    .child_by_field_name("name")
                    .ok_or_else(|| ::treesitter_types::ParseError::missing_field("name", node))?;
                <Name as ::treesitter_types::FromNode>::from_node(child, src)?
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
impl ::treesitter_types::Spanned for EnumCase<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        self.span
    }
}
#[derive(Debug, Clone)]
pub struct EnumDeclaration<'tree> {
    pub span: ::treesitter_types::Span,
    pub attributes: ::core::option::Option<AttributeList<'tree>>,
    pub body: EnumDeclarationList<'tree>,
    pub name: Name<'tree>,
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
            attributes: match node.child_by_field_name("attributes") {
                Some(child) => Some(<AttributeList as ::treesitter_types::FromNode>::from_node(
                    child, src,
                )?),
                None => None,
            },
            body: {
                let child = node
                    .child_by_field_name("body")
                    .ok_or_else(|| ::treesitter_types::ParseError::missing_field("body", node))?;
                <EnumDeclarationList as ::treesitter_types::FromNode>::from_node(child, src)?
            },
            name: {
                let child = node
                    .child_by_field_name("name")
                    .ok_or_else(|| ::treesitter_types::ParseError::missing_field("name", node))?;
                <Name as ::treesitter_types::FromNode>::from_node(child, src)?
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
pub struct EnumDeclarationList<'tree> {
    pub span: ::treesitter_types::Span,
    pub children: ::std::vec::Vec<EnumDeclarationListChildren<'tree>>,
}
impl<'tree> ::treesitter_types::FromNode<'tree> for EnumDeclarationList<'tree> {
    #[allow(clippy::match_single_binding, clippy::suspicious_else_formatting)]
    fn from_node(
        node: ::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        debug_assert_eq!(node.kind(), "enum_declaration_list");
        Ok(Self {
            span: ::treesitter_types::Span::from(node),
            children: {
                #[allow(clippy::suspicious_else_formatting)]
                let non_field_children = {
                    let mut cursor = node.walk();
                    let mut result = ::std::vec::Vec::new();
                    if cursor.goto_first_child() {
                        loop {
                            if cursor.field_name().is_none()
                                && cursor.node().is_named()
                                && !cursor.node().is_extra()
                            {
                                result.push(cursor.node());
                            }
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
                        <EnumDeclarationListChildren as ::treesitter_types::FromNode>::from_node(
                            child, src,
                        )?,
                    );
                }
                items
            },
        })
    }
}
impl ::treesitter_types::Spanned for EnumDeclarationList<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        self.span
    }
}
#[derive(Debug, Clone)]
pub struct ErrorSuppressionExpression<'tree> {
    pub span: ::treesitter_types::Span,
    pub children: Expression<'tree>,
}
impl<'tree> ::treesitter_types::FromNode<'tree> for ErrorSuppressionExpression<'tree> {
    #[allow(clippy::match_single_binding, clippy::suspicious_else_formatting)]
    fn from_node(
        node: ::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        debug_assert_eq!(node.kind(), "error_suppression_expression");
        Ok(Self {
            span: ::treesitter_types::Span::from(node),
            children: {
                #[allow(clippy::suspicious_else_formatting)]
                let non_field_children = {
                    let mut cursor = node.walk();
                    let mut result = ::std::vec::Vec::new();
                    if cursor.goto_first_child() {
                        loop {
                            if cursor.field_name().is_none()
                                && cursor.node().is_named()
                                && !cursor.node().is_extra()
                            {
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
impl ::treesitter_types::Spanned for ErrorSuppressionExpression<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        self.span
    }
}
#[derive(Debug, Clone)]
pub struct ExitStatement<'tree> {
    pub span: ::treesitter_types::Span,
    pub children: ::core::option::Option<Expression<'tree>>,
}
impl<'tree> ::treesitter_types::FromNode<'tree> for ExitStatement<'tree> {
    #[allow(clippy::match_single_binding, clippy::suspicious_else_formatting)]
    fn from_node(
        node: ::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        debug_assert_eq!(node.kind(), "exit_statement");
        Ok(Self {
            span: ::treesitter_types::Span::from(node),
            children: {
                #[allow(clippy::suspicious_else_formatting)]
                let non_field_children = {
                    let mut cursor = node.walk();
                    let mut result = ::std::vec::Vec::new();
                    if cursor.goto_first_child() {
                        loop {
                            if cursor.field_name().is_none()
                                && cursor.node().is_named()
                                && !cursor.node().is_extra()
                            {
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
impl ::treesitter_types::Spanned for ExitStatement<'_> {
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
                            if cursor.field_name().is_none()
                                && cursor.node().is_named()
                                && !cursor.node().is_extra()
                            {
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
impl ::treesitter_types::Spanned for ExpressionStatement<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        self.span
    }
}
#[derive(Debug, Clone)]
pub struct FinalModifier<'tree> {
    pub span: ::treesitter_types::Span,
    text: &'tree str,
}
impl<'tree> ::treesitter_types::FromNode<'tree> for FinalModifier<'tree> {
    fn from_node(
        node: ::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        debug_assert_eq!(node.kind(), "final_modifier");
        Ok(Self {
            span: ::treesitter_types::Span::from(node),
            text: node.utf8_text(src)?,
        })
    }
}
impl<'tree> ::treesitter_types::LeafNode<'tree> for FinalModifier<'tree> {
    fn text(&self) -> &'tree str {
        self.text
    }
}
impl ::treesitter_types::Spanned for FinalModifier<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        self.span
    }
}
#[derive(Debug, Clone)]
pub struct FinallyClause<'tree> {
    pub span: ::treesitter_types::Span,
    pub body: CompoundStatement<'tree>,
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
            body: {
                let child = node
                    .child_by_field_name("body")
                    .ok_or_else(|| ::treesitter_types::ParseError::missing_field("body", node))?;
                <CompoundStatement as ::treesitter_types::FromNode>::from_node(child, src)?
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
pub struct ForStatement<'tree> {
    pub span: ::treesitter_types::Span,
    pub body: ::std::vec::Vec<Statement<'tree>>,
    pub condition: ::core::option::Option<ForStatementCondition<'tree>>,
    pub initialize: ::core::option::Option<ForStatementInitialize<'tree>>,
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
                let mut cursor = node.walk();
                let mut items = ::std::vec::Vec::new();
                for child in node.children_by_field_name("body", &mut cursor) {
                    items.push(<Statement as ::treesitter_types::FromNode>::from_node(
                        child, src,
                    )?);
                }
                items
            },
            condition: match node.child_by_field_name("condition") {
                Some(child) => Some(
                    <ForStatementCondition as ::treesitter_types::FromNode>::from_node(child, src)?,
                ),
                None => None,
            },
            initialize: match node.child_by_field_name("initialize") {
                Some(child) => Some(
                    <ForStatementInitialize as ::treesitter_types::FromNode>::from_node(
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
pub struct ForeachStatement<'tree> {
    pub span: ::treesitter_types::Span,
    pub body: ::core::option::Option<ForeachStatementBody<'tree>>,
    pub children: ::std::vec::Vec<ForeachStatementChildren<'tree>>,
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
            body: match node.child_by_field_name("body") {
                Some(child) => Some(
                    <ForeachStatementBody as ::treesitter_types::FromNode>::from_node(child, src)?,
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
                        <ForeachStatementChildren as ::treesitter_types::FromNode>::from_node(
                            child, src,
                        )?,
                    );
                }
                items
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
pub struct FormalParameters<'tree> {
    pub span: ::treesitter_types::Span,
    pub children: ::std::vec::Vec<FormalParametersChildren<'tree>>,
}
impl<'tree> ::treesitter_types::FromNode<'tree> for FormalParameters<'tree> {
    #[allow(clippy::match_single_binding, clippy::suspicious_else_formatting)]
    fn from_node(
        node: ::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        debug_assert_eq!(node.kind(), "formal_parameters");
        Ok(Self {
            span: ::treesitter_types::Span::from(node),
            children: {
                #[allow(clippy::suspicious_else_formatting)]
                let non_field_children = {
                    let mut cursor = node.walk();
                    let mut result = ::std::vec::Vec::new();
                    if cursor.goto_first_child() {
                        loop {
                            if cursor.field_name().is_none()
                                && cursor.node().is_named()
                                && !cursor.node().is_extra()
                            {
                                result.push(cursor.node());
                            }
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
                        <FormalParametersChildren as ::treesitter_types::FromNode>::from_node(
                            child, src,
                        )?,
                    );
                }
                items
            },
        })
    }
}
impl ::treesitter_types::Spanned for FormalParameters<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        self.span
    }
}
#[derive(Debug, Clone)]
pub struct FunctionCallExpression<'tree> {
    pub span: ::treesitter_types::Span,
    pub arguments: Arguments<'tree>,
    pub function: FunctionCallExpressionFunction<'tree>,
}
impl<'tree> ::treesitter_types::FromNode<'tree> for FunctionCallExpression<'tree> {
    #[allow(clippy::match_single_binding, clippy::suspicious_else_formatting)]
    fn from_node(
        node: ::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        debug_assert_eq!(node.kind(), "function_call_expression");
        Ok(Self {
            span: ::treesitter_types::Span::from(node),
            arguments: {
                let child = node.child_by_field_name("arguments").ok_or_else(|| {
                    ::treesitter_types::ParseError::missing_field("arguments", node)
                })?;
                <Arguments as ::treesitter_types::FromNode>::from_node(child, src)?
            },
            function: {
                let child = node.child_by_field_name("function").ok_or_else(|| {
                    ::treesitter_types::ParseError::missing_field("function", node)
                })?;
                <FunctionCallExpressionFunction as ::treesitter_types::FromNode>::from_node(
                    child, src,
                )?
            },
        })
    }
}
impl ::treesitter_types::Spanned for FunctionCallExpression<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        self.span
    }
}
#[derive(Debug, Clone)]
pub struct FunctionDefinition<'tree> {
    pub span: ::treesitter_types::Span,
    pub attributes: ::core::option::Option<AttributeList<'tree>>,
    pub body: CompoundStatement<'tree>,
    pub name: Name<'tree>,
    pub parameters: FormalParameters<'tree>,
    pub return_type: ::core::option::Option<FunctionDefinitionReturnType<'tree>>,
    pub children: ::core::option::Option<ReferenceModifier<'tree>>,
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
            attributes: match node.child_by_field_name("attributes") {
                Some(child) => Some(<AttributeList as ::treesitter_types::FromNode>::from_node(
                    child, src,
                )?),
                None => None,
            },
            body: {
                let child = node
                    .child_by_field_name("body")
                    .ok_or_else(|| ::treesitter_types::ParseError::missing_field("body", node))?;
                <CompoundStatement as ::treesitter_types::FromNode>::from_node(child, src)?
            },
            name: {
                let child = node
                    .child_by_field_name("name")
                    .ok_or_else(|| ::treesitter_types::ParseError::missing_field("name", node))?;
                <Name as ::treesitter_types::FromNode>::from_node(child, src)?
            },
            parameters: {
                let child = node.child_by_field_name("parameters").ok_or_else(|| {
                    ::treesitter_types::ParseError::missing_field("parameters", node)
                })?;
                <FormalParameters as ::treesitter_types::FromNode>::from_node(child, src)?
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
                match non_field_children.first() {
                    Some(&child) => Some(
                        <ReferenceModifier as ::treesitter_types::FromNode>::from_node(child, src)?,
                    ),
                    None => None,
                }
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
pub struct FunctionStaticDeclaration<'tree> {
    pub span: ::treesitter_types::Span,
    pub children: ::std::vec::Vec<StaticVariableDeclaration<'tree>>,
}
impl<'tree> ::treesitter_types::FromNode<'tree> for FunctionStaticDeclaration<'tree> {
    #[allow(clippy::match_single_binding, clippy::suspicious_else_formatting)]
    fn from_node(
        node: ::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        debug_assert_eq!(node.kind(), "function_static_declaration");
        Ok(Self {
            span: ::treesitter_types::Span::from(node),
            children: {
                #[allow(clippy::suspicious_else_formatting)]
                let non_field_children = {
                    let mut cursor = node.walk();
                    let mut result = ::std::vec::Vec::new();
                    if cursor.goto_first_child() {
                        loop {
                            if cursor.field_name().is_none()
                                && cursor.node().is_named()
                                && !cursor.node().is_extra()
                            {
                                result.push(cursor.node());
                            }
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
                        <StaticVariableDeclaration as ::treesitter_types::FromNode>::from_node(
                            child, src,
                        )?,
                    );
                }
                items
            },
        })
    }
}
impl ::treesitter_types::Spanned for FunctionStaticDeclaration<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        self.span
    }
}
#[derive(Debug, Clone)]
pub struct GlobalDeclaration<'tree> {
    pub span: ::treesitter_types::Span,
    pub children: ::std::vec::Vec<GlobalDeclarationChildren<'tree>>,
}
impl<'tree> ::treesitter_types::FromNode<'tree> for GlobalDeclaration<'tree> {
    #[allow(clippy::match_single_binding, clippy::suspicious_else_formatting)]
    fn from_node(
        node: ::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        debug_assert_eq!(node.kind(), "global_declaration");
        Ok(Self {
            span: ::treesitter_types::Span::from(node),
            children: {
                #[allow(clippy::suspicious_else_formatting)]
                let non_field_children = {
                    let mut cursor = node.walk();
                    let mut result = ::std::vec::Vec::new();
                    if cursor.goto_first_child() {
                        loop {
                            if cursor.field_name().is_none()
                                && cursor.node().is_named()
                                && !cursor.node().is_extra()
                            {
                                result.push(cursor.node());
                            }
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
                        <GlobalDeclarationChildren as ::treesitter_types::FromNode>::from_node(
                            child, src,
                        )?,
                    );
                }
                items
            },
        })
    }
}
impl ::treesitter_types::Spanned for GlobalDeclaration<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        self.span
    }
}
#[derive(Debug, Clone)]
pub struct GotoStatement<'tree> {
    pub span: ::treesitter_types::Span,
    pub children: Name<'tree>,
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
                                        <Name as ::treesitter_types::FromNode>::from_node(
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
                <Name as ::treesitter_types::FromNode>::from_node(child, src)?
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
pub struct Heredoc<'tree> {
    pub span: ::treesitter_types::Span,
    pub end_tag: HeredocEnd<'tree>,
    pub identifier: HeredocStart<'tree>,
    pub value: ::core::option::Option<HeredocBody<'tree>>,
}
impl<'tree> ::treesitter_types::FromNode<'tree> for Heredoc<'tree> {
    #[allow(clippy::match_single_binding, clippy::suspicious_else_formatting)]
    fn from_node(
        node: ::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        debug_assert_eq!(node.kind(), "heredoc");
        Ok(Self {
            span: ::treesitter_types::Span::from(node),
            end_tag: {
                let child = node.child_by_field_name("end_tag").ok_or_else(|| {
                    ::treesitter_types::ParseError::missing_field("end_tag", node)
                })?;
                <HeredocEnd as ::treesitter_types::FromNode>::from_node(child, src)?
            },
            identifier: {
                let child = node.child_by_field_name("identifier").ok_or_else(|| {
                    ::treesitter_types::ParseError::missing_field("identifier", node)
                })?;
                <HeredocStart as ::treesitter_types::FromNode>::from_node(child, src)?
            },
            value: match node.child_by_field_name("value") {
                Some(child) => Some(<HeredocBody as ::treesitter_types::FromNode>::from_node(
                    child, src,
                )?),
                None => None,
            },
        })
    }
}
impl ::treesitter_types::Spanned for Heredoc<'_> {
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
pub struct IfStatement<'tree> {
    pub span: ::treesitter_types::Span,
    pub alternative: ::std::vec::Vec<IfStatementAlternative<'tree>>,
    pub body: IfStatementBody<'tree>,
    pub condition: ParenthesizedExpression<'tree>,
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
            body: {
                let child = node
                    .child_by_field_name("body")
                    .ok_or_else(|| ::treesitter_types::ParseError::missing_field("body", node))?;
                <IfStatementBody as ::treesitter_types::FromNode>::from_node(child, src)?
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
impl ::treesitter_types::Spanned for IfStatement<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        self.span
    }
}
#[derive(Debug, Clone)]
pub struct IncludeExpression<'tree> {
    pub span: ::treesitter_types::Span,
    pub children: Expression<'tree>,
}
impl<'tree> ::treesitter_types::FromNode<'tree> for IncludeExpression<'tree> {
    #[allow(clippy::match_single_binding, clippy::suspicious_else_formatting)]
    fn from_node(
        node: ::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        debug_assert_eq!(node.kind(), "include_expression");
        Ok(Self {
            span: ::treesitter_types::Span::from(node),
            children: {
                #[allow(clippy::suspicious_else_formatting)]
                let non_field_children = {
                    let mut cursor = node.walk();
                    let mut result = ::std::vec::Vec::new();
                    if cursor.goto_first_child() {
                        loop {
                            if cursor.field_name().is_none()
                                && cursor.node().is_named()
                                && !cursor.node().is_extra()
                            {
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
impl ::treesitter_types::Spanned for IncludeExpression<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        self.span
    }
}
#[derive(Debug, Clone)]
pub struct IncludeOnceExpression<'tree> {
    pub span: ::treesitter_types::Span,
    pub children: Expression<'tree>,
}
impl<'tree> ::treesitter_types::FromNode<'tree> for IncludeOnceExpression<'tree> {
    #[allow(clippy::match_single_binding, clippy::suspicious_else_formatting)]
    fn from_node(
        node: ::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        debug_assert_eq!(node.kind(), "include_once_expression");
        Ok(Self {
            span: ::treesitter_types::Span::from(node),
            children: {
                #[allow(clippy::suspicious_else_formatting)]
                let non_field_children = {
                    let mut cursor = node.walk();
                    let mut result = ::std::vec::Vec::new();
                    if cursor.goto_first_child() {
                        loop {
                            if cursor.field_name().is_none()
                                && cursor.node().is_named()
                                && !cursor.node().is_extra()
                            {
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
impl ::treesitter_types::Spanned for IncludeOnceExpression<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        self.span
    }
}
#[derive(Debug, Clone)]
pub struct InterfaceDeclaration<'tree> {
    pub span: ::treesitter_types::Span,
    pub attributes: ::core::option::Option<AttributeList<'tree>>,
    pub body: DeclarationList<'tree>,
    pub name: Name<'tree>,
    pub children: ::core::option::Option<BaseClause<'tree>>,
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
            attributes: match node.child_by_field_name("attributes") {
                Some(child) => Some(<AttributeList as ::treesitter_types::FromNode>::from_node(
                    child, src,
                )?),
                None => None,
            },
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
                <Name as ::treesitter_types::FromNode>::from_node(child, src)?
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
                    Some(&child) => Some(<BaseClause as ::treesitter_types::FromNode>::from_node(
                        child, src,
                    )?),
                    None => None,
                }
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
pub struct IntersectionType<'tree> {
    pub span: ::treesitter_types::Span,
    pub children: ::std::vec::Vec<IntersectionTypeChildren<'tree>>,
}
impl<'tree> ::treesitter_types::FromNode<'tree> for IntersectionType<'tree> {
    #[allow(clippy::match_single_binding, clippy::suspicious_else_formatting)]
    fn from_node(
        node: ::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        debug_assert_eq!(node.kind(), "intersection_type");
        Ok(Self {
            span: ::treesitter_types::Span::from(node),
            children: {
                #[allow(clippy::suspicious_else_formatting)]
                let non_field_children = {
                    let mut cursor = node.walk();
                    let mut result = ::std::vec::Vec::new();
                    if cursor.goto_first_child() {
                        loop {
                            if cursor.field_name().is_none()
                                && cursor.node().is_named()
                                && !cursor.node().is_extra()
                            {
                                result.push(cursor.node());
                            }
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
                        <IntersectionTypeChildren as ::treesitter_types::FromNode>::from_node(
                            child, src,
                        )?,
                    );
                }
                items
            },
        })
    }
}
impl ::treesitter_types::Spanned for IntersectionType<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        self.span
    }
}
#[derive(Debug, Clone)]
pub struct ListLiteral<'tree> {
    pub span: ::treesitter_types::Span,
    pub children: ::std::vec::Vec<ListLiteralChildren<'tree>>,
}
impl<'tree> ::treesitter_types::FromNode<'tree> for ListLiteral<'tree> {
    #[allow(clippy::match_single_binding, clippy::suspicious_else_formatting)]
    fn from_node(
        node: ::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        debug_assert_eq!(node.kind(), "list_literal");
        Ok(Self {
            span: ::treesitter_types::Span::from(node),
            children: {
                #[allow(clippy::suspicious_else_formatting)]
                let non_field_children = {
                    let mut cursor = node.walk();
                    let mut result = ::std::vec::Vec::new();
                    if cursor.goto_first_child() {
                        loop {
                            if cursor.field_name().is_none()
                                && cursor.node().is_named()
                                && !cursor.node().is_extra()
                            {
                                result.push(cursor.node());
                            }
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
                        <ListLiteralChildren as ::treesitter_types::FromNode>::from_node(
                            child, src,
                        )?,
                    );
                }
                items
            },
        })
    }
}
impl ::treesitter_types::Spanned for ListLiteral<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        self.span
    }
}
#[derive(Debug, Clone)]
pub struct MatchBlock<'tree> {
    pub span: ::treesitter_types::Span,
    pub children: ::std::vec::Vec<MatchBlockChildren<'tree>>,
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
                    items.push(
                        <MatchBlockChildren as ::treesitter_types::FromNode>::from_node(
                            child, src,
                        )?,
                    );
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
#[derive(Debug, Clone)]
pub struct MatchConditionList<'tree> {
    pub span: ::treesitter_types::Span,
    pub children: ::std::vec::Vec<Expression<'tree>>,
}
impl<'tree> ::treesitter_types::FromNode<'tree> for MatchConditionList<'tree> {
    #[allow(clippy::match_single_binding, clippy::suspicious_else_formatting)]
    fn from_node(
        node: ::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        debug_assert_eq!(node.kind(), "match_condition_list");
        Ok(Self {
            span: ::treesitter_types::Span::from(node),
            children: {
                #[allow(clippy::suspicious_else_formatting)]
                let non_field_children = {
                    let mut cursor = node.walk();
                    let mut result = ::std::vec::Vec::new();
                    if cursor.goto_first_child() {
                        loop {
                            if cursor.field_name().is_none()
                                && cursor.node().is_named()
                                && !cursor.node().is_extra()
                            {
                                result.push(cursor.node());
                            }
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
impl ::treesitter_types::Spanned for MatchConditionList<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        self.span
    }
}
#[derive(Debug, Clone)]
pub struct MatchConditionalExpression<'tree> {
    pub span: ::treesitter_types::Span,
    pub conditional_expressions: MatchConditionList<'tree>,
    pub return_expression: Expression<'tree>,
}
impl<'tree> ::treesitter_types::FromNode<'tree> for MatchConditionalExpression<'tree> {
    #[allow(clippy::match_single_binding, clippy::suspicious_else_formatting)]
    fn from_node(
        node: ::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        debug_assert_eq!(node.kind(), "match_conditional_expression");
        Ok(Self {
            span: ::treesitter_types::Span::from(node),
            conditional_expressions: {
                let child = node
                    .child_by_field_name("conditional_expressions")
                    .ok_or_else(|| {
                        ::treesitter_types::ParseError::missing_field(
                            "conditional_expressions",
                            node,
                        )
                    })?;
                <MatchConditionList as ::treesitter_types::FromNode>::from_node(child, src)?
            },
            return_expression: {
                let child = node
                    .child_by_field_name("return_expression")
                    .ok_or_else(|| {
                        ::treesitter_types::ParseError::missing_field("return_expression", node)
                    })?;
                <Expression as ::treesitter_types::FromNode>::from_node(child, src)?
            },
        })
    }
}
impl ::treesitter_types::Spanned for MatchConditionalExpression<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        self.span
    }
}
#[derive(Debug, Clone)]
pub struct MatchDefaultExpression<'tree> {
    pub span: ::treesitter_types::Span,
    pub return_expression: Expression<'tree>,
}
impl<'tree> ::treesitter_types::FromNode<'tree> for MatchDefaultExpression<'tree> {
    #[allow(clippy::match_single_binding, clippy::suspicious_else_formatting)]
    fn from_node(
        node: ::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        debug_assert_eq!(node.kind(), "match_default_expression");
        Ok(Self {
            span: ::treesitter_types::Span::from(node),
            return_expression: {
                let child = node
                    .child_by_field_name("return_expression")
                    .ok_or_else(|| {
                        ::treesitter_types::ParseError::missing_field("return_expression", node)
                    })?;
                <Expression as ::treesitter_types::FromNode>::from_node(child, src)?
            },
        })
    }
}
impl ::treesitter_types::Spanned for MatchDefaultExpression<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        self.span
    }
}
#[derive(Debug, Clone)]
pub struct MatchExpression<'tree> {
    pub span: ::treesitter_types::Span,
    pub body: MatchBlock<'tree>,
    pub condition: ParenthesizedExpression<'tree>,
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
                <MatchBlock as ::treesitter_types::FromNode>::from_node(child, src)?
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
impl ::treesitter_types::Spanned for MatchExpression<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        self.span
    }
}
#[derive(Debug, Clone)]
pub struct MemberAccessExpression<'tree> {
    pub span: ::treesitter_types::Span,
    pub name: MemberAccessExpressionName<'tree>,
    pub object: MemberAccessExpressionObject<'tree>,
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
            name: {
                let child = node
                    .child_by_field_name("name")
                    .ok_or_else(|| ::treesitter_types::ParseError::missing_field("name", node))?;
                <MemberAccessExpressionName as ::treesitter_types::FromNode>::from_node(child, src)?
            },
            object: {
                let child = node
                    .child_by_field_name("object")
                    .ok_or_else(|| ::treesitter_types::ParseError::missing_field("object", node))?;
                <MemberAccessExpressionObject as ::treesitter_types::FromNode>::from_node(
                    child, src,
                )?
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
pub struct MemberCallExpression<'tree> {
    pub span: ::treesitter_types::Span,
    pub arguments: Arguments<'tree>,
    pub name: MemberCallExpressionName<'tree>,
    pub object: MemberCallExpressionObject<'tree>,
}
impl<'tree> ::treesitter_types::FromNode<'tree> for MemberCallExpression<'tree> {
    #[allow(clippy::match_single_binding, clippy::suspicious_else_formatting)]
    fn from_node(
        node: ::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        debug_assert_eq!(node.kind(), "member_call_expression");
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
                <MemberCallExpressionName as ::treesitter_types::FromNode>::from_node(child, src)?
            },
            object: {
                let child = node
                    .child_by_field_name("object")
                    .ok_or_else(|| ::treesitter_types::ParseError::missing_field("object", node))?;
                <MemberCallExpressionObject as ::treesitter_types::FromNode>::from_node(child, src)?
            },
        })
    }
}
impl ::treesitter_types::Spanned for MemberCallExpression<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        self.span
    }
}
#[derive(Debug, Clone)]
pub struct MethodDeclaration<'tree> {
    pub span: ::treesitter_types::Span,
    pub attributes: ::core::option::Option<AttributeList<'tree>>,
    pub body: ::core::option::Option<CompoundStatement<'tree>>,
    pub name: Name<'tree>,
    pub parameters: FormalParameters<'tree>,
    pub return_type: ::core::option::Option<MethodDeclarationReturnType<'tree>>,
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
            attributes: match node.child_by_field_name("attributes") {
                Some(child) => Some(<AttributeList as ::treesitter_types::FromNode>::from_node(
                    child, src,
                )?),
                None => None,
            },
            body: match node.child_by_field_name("body") {
                Some(child) => Some(
                    <CompoundStatement as ::treesitter_types::FromNode>::from_node(child, src)?,
                ),
                None => None,
            },
            name: {
                let child = node
                    .child_by_field_name("name")
                    .ok_or_else(|| ::treesitter_types::ParseError::missing_field("name", node))?;
                <Name as ::treesitter_types::FromNode>::from_node(child, src)?
            },
            parameters: {
                let child = node.child_by_field_name("parameters").ok_or_else(|| {
                    ::treesitter_types::ParseError::missing_field("parameters", node)
                })?;
                <FormalParameters as ::treesitter_types::FromNode>::from_node(child, src)?
            },
            return_type: match node.child_by_field_name("return_type") {
                Some(child) => Some(
                    <MethodDeclarationReturnType as ::treesitter_types::FromNode>::from_node(
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
pub struct Name<'tree> {
    pub span: ::treesitter_types::Span,
    text: &'tree str,
}
impl<'tree> ::treesitter_types::FromNode<'tree> for Name<'tree> {
    fn from_node(
        node: ::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        debug_assert_eq!(node.kind(), "name");
        Ok(Self {
            span: ::treesitter_types::Span::from(node),
            text: node.utf8_text(src)?,
        })
    }
}
impl<'tree> ::treesitter_types::LeafNode<'tree> for Name<'tree> {
    fn text(&self) -> &'tree str {
        self.text
    }
}
impl ::treesitter_types::Spanned for Name<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        self.span
    }
}
#[derive(Debug, Clone)]
pub struct NamedLabelStatement<'tree> {
    pub span: ::treesitter_types::Span,
    pub children: Name<'tree>,
}
impl<'tree> ::treesitter_types::FromNode<'tree> for NamedLabelStatement<'tree> {
    #[allow(clippy::match_single_binding, clippy::suspicious_else_formatting)]
    fn from_node(
        node: ::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        debug_assert_eq!(node.kind(), "named_label_statement");
        Ok(Self {
            span: ::treesitter_types::Span::from(node),
            children: {
                #[allow(clippy::suspicious_else_formatting)]
                let non_field_children = {
                    let mut cursor = node.walk();
                    let mut result = ::std::vec::Vec::new();
                    if cursor.goto_first_child() {
                        loop {
                            if cursor.field_name().is_none()
                                && cursor.node().is_named()
                                && !cursor.node().is_extra()
                            {
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
                                        <Name as ::treesitter_types::FromNode>::from_node(
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
                <Name as ::treesitter_types::FromNode>::from_node(child, src)?
            },
        })
    }
}
impl ::treesitter_types::Spanned for NamedLabelStatement<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        self.span
    }
}
#[derive(Debug, Clone)]
pub struct NamedType<'tree> {
    pub span: ::treesitter_types::Span,
    pub children: NamedTypeChildren<'tree>,
}
impl<'tree> ::treesitter_types::FromNode<'tree> for NamedType<'tree> {
    #[allow(clippy::match_single_binding, clippy::suspicious_else_formatting)]
    fn from_node(
        node: ::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        debug_assert_eq!(node.kind(), "named_type");
        Ok(Self {
            span: ::treesitter_types::Span::from(node),
            children: {
                #[allow(clippy::suspicious_else_formatting)]
                let non_field_children = {
                    let mut cursor = node.walk();
                    let mut result = ::std::vec::Vec::new();
                    if cursor.goto_first_child() {
                        loop {
                            if cursor.field_name().is_none()
                                && cursor.node().is_named()
                                && !cursor.node().is_extra()
                            {
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
                                        <NamedTypeChildren as ::treesitter_types::FromNode>::from_node(
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
                <NamedTypeChildren as ::treesitter_types::FromNode>::from_node(child, src)?
            },
        })
    }
}
impl ::treesitter_types::Spanned for NamedType<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        self.span
    }
}
#[derive(Debug, Clone)]
pub struct NamespaceDefinition<'tree> {
    pub span: ::treesitter_types::Span,
    pub body: ::core::option::Option<CompoundStatement<'tree>>,
    pub name: ::core::option::Option<NamespaceName<'tree>>,
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
            body: match node.child_by_field_name("body") {
                Some(child) => Some(
                    <CompoundStatement as ::treesitter_types::FromNode>::from_node(child, src)?,
                ),
                None => None,
            },
            name: match node.child_by_field_name("name") {
                Some(child) => Some(<NamespaceName as ::treesitter_types::FromNode>::from_node(
                    child, src,
                )?),
                None => None,
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
pub struct NamespaceName<'tree> {
    pub span: ::treesitter_types::Span,
    pub children: ::std::vec::Vec<Name<'tree>>,
}
impl<'tree> ::treesitter_types::FromNode<'tree> for NamespaceName<'tree> {
    #[allow(clippy::match_single_binding, clippy::suspicious_else_formatting)]
    fn from_node(
        node: ::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        debug_assert_eq!(node.kind(), "namespace_name");
        Ok(Self {
            span: ::treesitter_types::Span::from(node),
            children: {
                #[allow(clippy::suspicious_else_formatting)]
                let non_field_children = {
                    let mut cursor = node.walk();
                    let mut result = ::std::vec::Vec::new();
                    if cursor.goto_first_child() {
                        loop {
                            if cursor.field_name().is_none()
                                && cursor.node().is_named()
                                && !cursor.node().is_extra()
                            {
                                result.push(cursor.node());
                            }
                            if !cursor.goto_next_sibling() {
                                break;
                            }
                        }
                    }
                    result
                };
                let mut items = ::std::vec::Vec::new();
                for child in non_field_children {
                    items.push(<Name as ::treesitter_types::FromNode>::from_node(
                        child, src,
                    )?);
                }
                items
            },
        })
    }
}
impl ::treesitter_types::Spanned for NamespaceName<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        self.span
    }
}
#[derive(Debug, Clone)]
pub struct NamespaceUseClause<'tree> {
    pub span: ::treesitter_types::Span,
    pub alias: ::core::option::Option<Name<'tree>>,
    pub r#type: ::core::option::Option<NamespaceUseClauseType>,
    pub children: NamespaceUseClauseChildren<'tree>,
}
impl<'tree> ::treesitter_types::FromNode<'tree> for NamespaceUseClause<'tree> {
    #[allow(clippy::match_single_binding, clippy::suspicious_else_formatting)]
    fn from_node(
        node: ::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        debug_assert_eq!(node.kind(), "namespace_use_clause");
        Ok(Self {
            span: ::treesitter_types::Span::from(node),
            alias: match node.child_by_field_name("alias") {
                Some(child) => Some(<Name as ::treesitter_types::FromNode>::from_node(
                    child, src,
                )?),
                None => None,
            },
            r#type: match node.child_by_field_name("type") {
                Some(child) => Some(
                    <NamespaceUseClauseType as ::treesitter_types::FromNode>::from_node(
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
                                        <NamespaceUseClauseChildren as ::treesitter_types::FromNode>::from_node(
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
                <NamespaceUseClauseChildren as ::treesitter_types::FromNode>::from_node(child, src)?
            },
        })
    }
}
impl ::treesitter_types::Spanned for NamespaceUseClause<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        self.span
    }
}
#[derive(Debug, Clone)]
pub struct NamespaceUseDeclaration<'tree> {
    pub span: ::treesitter_types::Span,
    pub body: ::core::option::Option<NamespaceUseGroup<'tree>>,
    pub r#type: ::core::option::Option<NamespaceUseDeclarationType>,
    pub children: ::std::vec::Vec<NamespaceUseDeclarationChildren<'tree>>,
}
impl<'tree> ::treesitter_types::FromNode<'tree> for NamespaceUseDeclaration<'tree> {
    #[allow(clippy::match_single_binding, clippy::suspicious_else_formatting)]
    fn from_node(
        node: ::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        debug_assert_eq!(node.kind(), "namespace_use_declaration");
        Ok(Self {
            span: ::treesitter_types::Span::from(node),
            body: match node.child_by_field_name("body") {
                Some(child) => Some(
                    <NamespaceUseGroup as ::treesitter_types::FromNode>::from_node(child, src)?,
                ),
                None => None,
            },
            r#type: match node.child_by_field_name("type") {
                Some(child) => Some(
                    <NamespaceUseDeclarationType as ::treesitter_types::FromNode>::from_node(
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
                    items
                        .push(
                            <NamespaceUseDeclarationChildren as ::treesitter_types::FromNode>::from_node(
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
impl ::treesitter_types::Spanned for NamespaceUseDeclaration<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        self.span
    }
}
#[derive(Debug, Clone)]
pub struct NamespaceUseGroup<'tree> {
    pub span: ::treesitter_types::Span,
    pub children: ::std::vec::Vec<NamespaceUseClause<'tree>>,
}
impl<'tree> ::treesitter_types::FromNode<'tree> for NamespaceUseGroup<'tree> {
    #[allow(clippy::match_single_binding, clippy::suspicious_else_formatting)]
    fn from_node(
        node: ::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        debug_assert_eq!(node.kind(), "namespace_use_group");
        Ok(Self {
            span: ::treesitter_types::Span::from(node),
            children: {
                #[allow(clippy::suspicious_else_formatting)]
                let non_field_children = {
                    let mut cursor = node.walk();
                    let mut result = ::std::vec::Vec::new();
                    if cursor.goto_first_child() {
                        loop {
                            if cursor.field_name().is_none()
                                && cursor.node().is_named()
                                && !cursor.node().is_extra()
                            {
                                result.push(cursor.node());
                            }
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
                        <NamespaceUseClause as ::treesitter_types::FromNode>::from_node(
                            child, src,
                        )?,
                    );
                }
                items
            },
        })
    }
}
impl ::treesitter_types::Spanned for NamespaceUseGroup<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        self.span
    }
}
#[derive(Debug, Clone)]
pub struct Nowdoc<'tree> {
    pub span: ::treesitter_types::Span,
    pub end_tag: HeredocEnd<'tree>,
    pub identifier: HeredocStart<'tree>,
    pub value: ::core::option::Option<NowdocBody<'tree>>,
}
impl<'tree> ::treesitter_types::FromNode<'tree> for Nowdoc<'tree> {
    #[allow(clippy::match_single_binding, clippy::suspicious_else_formatting)]
    fn from_node(
        node: ::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        debug_assert_eq!(node.kind(), "nowdoc");
        Ok(Self {
            span: ::treesitter_types::Span::from(node),
            end_tag: {
                let child = node.child_by_field_name("end_tag").ok_or_else(|| {
                    ::treesitter_types::ParseError::missing_field("end_tag", node)
                })?;
                <HeredocEnd as ::treesitter_types::FromNode>::from_node(child, src)?
            },
            identifier: {
                let child = node.child_by_field_name("identifier").ok_or_else(|| {
                    ::treesitter_types::ParseError::missing_field("identifier", node)
                })?;
                <HeredocStart as ::treesitter_types::FromNode>::from_node(child, src)?
            },
            value: match node.child_by_field_name("value") {
                Some(child) => Some(<NowdocBody as ::treesitter_types::FromNode>::from_node(
                    child, src,
                )?),
                None => None,
            },
        })
    }
}
impl ::treesitter_types::Spanned for Nowdoc<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        self.span
    }
}
#[derive(Debug, Clone)]
pub struct NowdocBody<'tree> {
    pub span: ::treesitter_types::Span,
    pub children: ::std::vec::Vec<NowdocString<'tree>>,
}
impl<'tree> ::treesitter_types::FromNode<'tree> for NowdocBody<'tree> {
    #[allow(clippy::match_single_binding, clippy::suspicious_else_formatting)]
    fn from_node(
        node: ::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        debug_assert_eq!(node.kind(), "nowdoc_body");
        Ok(Self {
            span: ::treesitter_types::Span::from(node),
            children: {
                #[allow(clippy::suspicious_else_formatting)]
                let non_field_children = {
                    let mut cursor = node.walk();
                    let mut result = ::std::vec::Vec::new();
                    if cursor.goto_first_child() {
                        loop {
                            if cursor.field_name().is_none()
                                && cursor.node().is_named()
                                && !cursor.node().is_extra()
                            {
                                result.push(cursor.node());
                            }
                            if !cursor.goto_next_sibling() {
                                break;
                            }
                        }
                    }
                    result
                };
                let mut items = ::std::vec::Vec::new();
                for child in non_field_children {
                    items.push(<NowdocString as ::treesitter_types::FromNode>::from_node(
                        child, src,
                    )?);
                }
                items
            },
        })
    }
}
impl ::treesitter_types::Spanned for NowdocBody<'_> {
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
pub struct NullsafeMemberAccessExpression<'tree> {
    pub span: ::treesitter_types::Span,
    pub name: NullsafeMemberAccessExpressionName<'tree>,
    pub object: NullsafeMemberAccessExpressionObject<'tree>,
}
impl<'tree> ::treesitter_types::FromNode<'tree> for NullsafeMemberAccessExpression<'tree> {
    #[allow(clippy::match_single_binding, clippy::suspicious_else_formatting)]
    fn from_node(
        node: ::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        debug_assert_eq!(node.kind(), "nullsafe_member_access_expression");
        Ok(Self {
            span: ::treesitter_types::Span::from(node),
            name: {
                let child = node
                    .child_by_field_name("name")
                    .ok_or_else(|| ::treesitter_types::ParseError::missing_field("name", node))?;
                <NullsafeMemberAccessExpressionName as ::treesitter_types::FromNode>::from_node(
                    child, src,
                )?
            },
            object: {
                let child = node
                    .child_by_field_name("object")
                    .ok_or_else(|| ::treesitter_types::ParseError::missing_field("object", node))?;
                <NullsafeMemberAccessExpressionObject as ::treesitter_types::FromNode>::from_node(
                    child, src,
                )?
            },
        })
    }
}
impl ::treesitter_types::Spanned for NullsafeMemberAccessExpression<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        self.span
    }
}
#[derive(Debug, Clone)]
pub struct NullsafeMemberCallExpression<'tree> {
    pub span: ::treesitter_types::Span,
    pub arguments: Arguments<'tree>,
    pub name: NullsafeMemberCallExpressionName<'tree>,
    pub object: NullsafeMemberCallExpressionObject<'tree>,
}
impl<'tree> ::treesitter_types::FromNode<'tree> for NullsafeMemberCallExpression<'tree> {
    #[allow(clippy::match_single_binding, clippy::suspicious_else_formatting)]
    fn from_node(
        node: ::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        debug_assert_eq!(node.kind(), "nullsafe_member_call_expression");
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
                <NullsafeMemberCallExpressionName as ::treesitter_types::FromNode>::from_node(
                    child, src,
                )?
            },
            object: {
                let child = node
                    .child_by_field_name("object")
                    .ok_or_else(|| ::treesitter_types::ParseError::missing_field("object", node))?;
                <NullsafeMemberCallExpressionObject as ::treesitter_types::FromNode>::from_node(
                    child, src,
                )?
            },
        })
    }
}
impl ::treesitter_types::Spanned for NullsafeMemberCallExpression<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        self.span
    }
}
#[derive(Debug, Clone)]
pub struct ObjectCreationExpression<'tree> {
    pub span: ::treesitter_types::Span,
    pub children: ::std::vec::Vec<ObjectCreationExpressionChildren<'tree>>,
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
            children: {
                #[allow(clippy::suspicious_else_formatting)]
                let non_field_children = {
                    let mut cursor = node.walk();
                    let mut result = ::std::vec::Vec::new();
                    if cursor.goto_first_child() {
                        loop {
                            if cursor.field_name().is_none()
                                && cursor.node().is_named()
                                && !cursor.node().is_extra()
                            {
                                result.push(cursor.node());
                            }
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
                            <ObjectCreationExpressionChildren as ::treesitter_types::FromNode>::from_node(
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
impl ::treesitter_types::Spanned for ObjectCreationExpression<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        self.span
    }
}
#[derive(Debug, Clone)]
pub struct OptionalType<'tree> {
    pub span: ::treesitter_types::Span,
    pub children: OptionalTypeChildren<'tree>,
}
impl<'tree> ::treesitter_types::FromNode<'tree> for OptionalType<'tree> {
    #[allow(clippy::match_single_binding, clippy::suspicious_else_formatting)]
    fn from_node(
        node: ::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        debug_assert_eq!(node.kind(), "optional_type");
        Ok(Self {
            span: ::treesitter_types::Span::from(node),
            children: {
                #[allow(clippy::suspicious_else_formatting)]
                let non_field_children = {
                    let mut cursor = node.walk();
                    let mut result = ::std::vec::Vec::new();
                    if cursor.goto_first_child() {
                        loop {
                            if cursor.field_name().is_none()
                                && cursor.node().is_named()
                                && !cursor.node().is_extra()
                            {
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
                                        <OptionalTypeChildren as ::treesitter_types::FromNode>::from_node(
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
                <OptionalTypeChildren as ::treesitter_types::FromNode>::from_node(child, src)?
            },
        })
    }
}
impl ::treesitter_types::Spanned for OptionalType<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        self.span
    }
}
#[derive(Debug, Clone)]
pub struct Pair<'tree> {
    pub span: ::treesitter_types::Span,
    pub children: ::std::vec::Vec<PairChildren<'tree>>,
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
            children: {
                #[allow(clippy::suspicious_else_formatting)]
                let non_field_children = {
                    let mut cursor = node.walk();
                    let mut result = ::std::vec::Vec::new();
                    if cursor.goto_first_child() {
                        loop {
                            if cursor.field_name().is_none()
                                && cursor.node().is_named()
                                && !cursor.node().is_extra()
                            {
                                result.push(cursor.node());
                            }
                            if !cursor.goto_next_sibling() {
                                break;
                            }
                        }
                    }
                    result
                };
                let mut items = ::std::vec::Vec::new();
                for child in non_field_children {
                    items.push(<PairChildren as ::treesitter_types::FromNode>::from_node(
                        child, src,
                    )?);
                }
                items
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
pub struct PrintIntrinsic<'tree> {
    pub span: ::treesitter_types::Span,
    pub children: Expression<'tree>,
}
impl<'tree> ::treesitter_types::FromNode<'tree> for PrintIntrinsic<'tree> {
    #[allow(clippy::match_single_binding, clippy::suspicious_else_formatting)]
    fn from_node(
        node: ::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        debug_assert_eq!(node.kind(), "print_intrinsic");
        Ok(Self {
            span: ::treesitter_types::Span::from(node),
            children: {
                #[allow(clippy::suspicious_else_formatting)]
                let non_field_children = {
                    let mut cursor = node.walk();
                    let mut result = ::std::vec::Vec::new();
                    if cursor.goto_first_child() {
                        loop {
                            if cursor.field_name().is_none()
                                && cursor.node().is_named()
                                && !cursor.node().is_extra()
                            {
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
impl ::treesitter_types::Spanned for PrintIntrinsic<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        self.span
    }
}
#[derive(Debug, Clone)]
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
                    items.push(
                        <ProgramChildren as ::treesitter_types::FromNode>::from_node(child, src)?,
                    );
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
pub struct PropertyDeclaration<'tree> {
    pub span: ::treesitter_types::Span,
    pub attributes: ::core::option::Option<AttributeList<'tree>>,
    pub r#type: ::core::option::Option<Type<'tree>>,
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
            attributes: match node.child_by_field_name("attributes") {
                Some(child) => Some(<AttributeList as ::treesitter_types::FromNode>::from_node(
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
pub struct PropertyElement<'tree> {
    pub span: ::treesitter_types::Span,
    pub default_value: ::core::option::Option<Expression<'tree>>,
    pub name: VariableName<'tree>,
}
impl<'tree> ::treesitter_types::FromNode<'tree> for PropertyElement<'tree> {
    #[allow(clippy::match_single_binding, clippy::suspicious_else_formatting)]
    fn from_node(
        node: ::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        debug_assert_eq!(node.kind(), "property_element");
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
                <VariableName as ::treesitter_types::FromNode>::from_node(child, src)?
            },
        })
    }
}
impl ::treesitter_types::Spanned for PropertyElement<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        self.span
    }
}
#[derive(Debug, Clone)]
pub struct PropertyHook<'tree> {
    pub span: ::treesitter_types::Span,
    pub attributes: ::core::option::Option<AttributeList<'tree>>,
    pub body: ::core::option::Option<PropertyHookBody<'tree>>,
    pub r#final: ::core::option::Option<FinalModifier<'tree>>,
    pub parameters: ::core::option::Option<FormalParameters<'tree>>,
    pub reference_modifier: ::core::option::Option<ReferenceModifier<'tree>>,
    pub children: Name<'tree>,
}
impl<'tree> ::treesitter_types::FromNode<'tree> for PropertyHook<'tree> {
    #[allow(clippy::match_single_binding, clippy::suspicious_else_formatting)]
    fn from_node(
        node: ::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        debug_assert_eq!(node.kind(), "property_hook");
        Ok(Self {
            span: ::treesitter_types::Span::from(node),
            attributes: match node.child_by_field_name("attributes") {
                Some(child) => Some(<AttributeList as ::treesitter_types::FromNode>::from_node(
                    child, src,
                )?),
                None => None,
            },
            body: match node.child_by_field_name("body") {
                Some(child) => {
                    Some(<PropertyHookBody as ::treesitter_types::FromNode>::from_node(child, src)?)
                }
                None => None,
            },
            r#final: match node.child_by_field_name("final") {
                Some(child) => Some(<FinalModifier as ::treesitter_types::FromNode>::from_node(
                    child, src,
                )?),
                None => None,
            },
            parameters: match node.child_by_field_name("parameters") {
                Some(child) => {
                    Some(<FormalParameters as ::treesitter_types::FromNode>::from_node(child, src)?)
                }
                None => None,
            },
            reference_modifier: match node.child_by_field_name("reference_modifier") {
                Some(child) => Some(
                    <ReferenceModifier as ::treesitter_types::FromNode>::from_node(child, src)?,
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
                                        <Name as ::treesitter_types::FromNode>::from_node(
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
                <Name as ::treesitter_types::FromNode>::from_node(child, src)?
            },
        })
    }
}
impl ::treesitter_types::Spanned for PropertyHook<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        self.span
    }
}
#[derive(Debug, Clone)]
pub struct PropertyHookList<'tree> {
    pub span: ::treesitter_types::Span,
    pub children: ::std::vec::Vec<PropertyHook<'tree>>,
}
impl<'tree> ::treesitter_types::FromNode<'tree> for PropertyHookList<'tree> {
    #[allow(clippy::match_single_binding, clippy::suspicious_else_formatting)]
    fn from_node(
        node: ::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        debug_assert_eq!(node.kind(), "property_hook_list");
        Ok(Self {
            span: ::treesitter_types::Span::from(node),
            children: {
                #[allow(clippy::suspicious_else_formatting)]
                let non_field_children = {
                    let mut cursor = node.walk();
                    let mut result = ::std::vec::Vec::new();
                    if cursor.goto_first_child() {
                        loop {
                            if cursor.field_name().is_none()
                                && cursor.node().is_named()
                                && !cursor.node().is_extra()
                            {
                                result.push(cursor.node());
                            }
                            if !cursor.goto_next_sibling() {
                                break;
                            }
                        }
                    }
                    result
                };
                let mut items = ::std::vec::Vec::new();
                for child in non_field_children {
                    items.push(<PropertyHook as ::treesitter_types::FromNode>::from_node(
                        child, src,
                    )?);
                }
                items
            },
        })
    }
}
impl ::treesitter_types::Spanned for PropertyHookList<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        self.span
    }
}
#[derive(Debug, Clone)]
pub struct PropertyPromotionParameter<'tree> {
    pub span: ::treesitter_types::Span,
    pub attributes: ::core::option::Option<AttributeList<'tree>>,
    pub default_value: ::core::option::Option<Expression<'tree>>,
    pub name: PropertyPromotionParameterName<'tree>,
    pub readonly: ::core::option::Option<ReadonlyModifier<'tree>>,
    pub r#type: ::core::option::Option<Type<'tree>>,
    pub visibility: VisibilityModifier<'tree>,
    pub children: ::core::option::Option<PropertyHookList<'tree>>,
}
impl<'tree> ::treesitter_types::FromNode<'tree> for PropertyPromotionParameter<'tree> {
    #[allow(clippy::match_single_binding, clippy::suspicious_else_formatting)]
    fn from_node(
        node: ::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        debug_assert_eq!(node.kind(), "property_promotion_parameter");
        Ok(Self {
            span: ::treesitter_types::Span::from(node),
            attributes: match node.child_by_field_name("attributes") {
                Some(child) => Some(<AttributeList as ::treesitter_types::FromNode>::from_node(
                    child, src,
                )?),
                None => None,
            },
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
                <PropertyPromotionParameterName as ::treesitter_types::FromNode>::from_node(
                    child, src,
                )?
            },
            readonly: match node.child_by_field_name("readonly") {
                Some(child) => {
                    Some(<ReadonlyModifier as ::treesitter_types::FromNode>::from_node(child, src)?)
                }
                None => None,
            },
            r#type: match node.child_by_field_name("type") {
                Some(child) => Some(<Type as ::treesitter_types::FromNode>::from_node(
                    child, src,
                )?),
                None => None,
            },
            visibility: {
                let child = node.child_by_field_name("visibility").ok_or_else(|| {
                    ::treesitter_types::ParseError::missing_field("visibility", node)
                })?;
                <VisibilityModifier as ::treesitter_types::FromNode>::from_node(child, src)?
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
                        <PropertyHookList as ::treesitter_types::FromNode>::from_node(child, src)?,
                    ),
                    None => None,
                }
            },
        })
    }
}
impl ::treesitter_types::Spanned for PropertyPromotionParameter<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        self.span
    }
}
#[derive(Debug, Clone)]
pub struct QualifiedName<'tree> {
    pub span: ::treesitter_types::Span,
    pub prefix: ::std::vec::Vec<QualifiedNamePrefix<'tree>>,
    pub children: Name<'tree>,
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
            prefix: {
                let mut cursor = node.walk();
                let mut items = ::std::vec::Vec::new();
                for child in node.children_by_field_name("prefix", &mut cursor) {
                    items.push(
                        <QualifiedNamePrefix as ::treesitter_types::FromNode>::from_node(
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
                                        <Name as ::treesitter_types::FromNode>::from_node(
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
                <Name as ::treesitter_types::FromNode>::from_node(child, src)?
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
pub struct ReadonlyModifier<'tree> {
    pub span: ::treesitter_types::Span,
    text: &'tree str,
}
impl<'tree> ::treesitter_types::FromNode<'tree> for ReadonlyModifier<'tree> {
    fn from_node(
        node: ::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        debug_assert_eq!(node.kind(), "readonly_modifier");
        Ok(Self {
            span: ::treesitter_types::Span::from(node),
            text: node.utf8_text(src)?,
        })
    }
}
impl<'tree> ::treesitter_types::LeafNode<'tree> for ReadonlyModifier<'tree> {
    fn text(&self) -> &'tree str {
        self.text
    }
}
impl ::treesitter_types::Spanned for ReadonlyModifier<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        self.span
    }
}
#[derive(Debug, Clone)]
pub struct ReferenceAssignmentExpression<'tree> {
    pub span: ::treesitter_types::Span,
    pub left: ReferenceAssignmentExpressionLeft<'tree>,
    pub right: Expression<'tree>,
}
impl<'tree> ::treesitter_types::FromNode<'tree> for ReferenceAssignmentExpression<'tree> {
    #[allow(clippy::match_single_binding, clippy::suspicious_else_formatting)]
    fn from_node(
        node: ::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        debug_assert_eq!(node.kind(), "reference_assignment_expression");
        Ok(Self {
            span: ::treesitter_types::Span::from(node),
            left: {
                let child = node
                    .child_by_field_name("left")
                    .ok_or_else(|| ::treesitter_types::ParseError::missing_field("left", node))?;
                <ReferenceAssignmentExpressionLeft as ::treesitter_types::FromNode>::from_node(
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
impl ::treesitter_types::Spanned for ReferenceAssignmentExpression<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        self.span
    }
}
#[derive(Debug, Clone)]
pub struct ReferenceModifier<'tree> {
    pub span: ::treesitter_types::Span,
    text: &'tree str,
}
impl<'tree> ::treesitter_types::FromNode<'tree> for ReferenceModifier<'tree> {
    fn from_node(
        node: ::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        debug_assert_eq!(node.kind(), "reference_modifier");
        Ok(Self {
            span: ::treesitter_types::Span::from(node),
            text: node.utf8_text(src)?,
        })
    }
}
impl<'tree> ::treesitter_types::LeafNode<'tree> for ReferenceModifier<'tree> {
    fn text(&self) -> &'tree str {
        self.text
    }
}
impl ::treesitter_types::Spanned for ReferenceModifier<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        self.span
    }
}
#[derive(Debug, Clone)]
pub struct RelativeName<'tree> {
    pub span: ::treesitter_types::Span,
    pub prefix: ::std::vec::Vec<RelativeNamePrefix<'tree>>,
    pub children: Name<'tree>,
}
impl<'tree> ::treesitter_types::FromNode<'tree> for RelativeName<'tree> {
    #[allow(clippy::match_single_binding, clippy::suspicious_else_formatting)]
    fn from_node(
        node: ::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        debug_assert_eq!(node.kind(), "relative_name");
        Ok(Self {
            span: ::treesitter_types::Span::from(node),
            prefix: {
                let mut cursor = node.walk();
                let mut items = ::std::vec::Vec::new();
                for child in node.children_by_field_name("prefix", &mut cursor) {
                    items.push(
                        <RelativeNamePrefix as ::treesitter_types::FromNode>::from_node(
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
                                        <Name as ::treesitter_types::FromNode>::from_node(
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
                <Name as ::treesitter_types::FromNode>::from_node(child, src)?
            },
        })
    }
}
impl ::treesitter_types::Spanned for RelativeName<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        self.span
    }
}
#[derive(Debug, Clone)]
pub struct RelativeScope<'tree> {
    pub span: ::treesitter_types::Span,
    text: &'tree str,
}
impl<'tree> ::treesitter_types::FromNode<'tree> for RelativeScope<'tree> {
    fn from_node(
        node: ::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        debug_assert_eq!(node.kind(), "relative_scope");
        Ok(Self {
            span: ::treesitter_types::Span::from(node),
            text: node.utf8_text(src)?,
        })
    }
}
impl<'tree> ::treesitter_types::LeafNode<'tree> for RelativeScope<'tree> {
    fn text(&self) -> &'tree str {
        self.text
    }
}
impl ::treesitter_types::Spanned for RelativeScope<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        self.span
    }
}
#[derive(Debug, Clone)]
pub struct RequireExpression<'tree> {
    pub span: ::treesitter_types::Span,
    pub children: Expression<'tree>,
}
impl<'tree> ::treesitter_types::FromNode<'tree> for RequireExpression<'tree> {
    #[allow(clippy::match_single_binding, clippy::suspicious_else_formatting)]
    fn from_node(
        node: ::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        debug_assert_eq!(node.kind(), "require_expression");
        Ok(Self {
            span: ::treesitter_types::Span::from(node),
            children: {
                #[allow(clippy::suspicious_else_formatting)]
                let non_field_children = {
                    let mut cursor = node.walk();
                    let mut result = ::std::vec::Vec::new();
                    if cursor.goto_first_child() {
                        loop {
                            if cursor.field_name().is_none()
                                && cursor.node().is_named()
                                && !cursor.node().is_extra()
                            {
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
impl ::treesitter_types::Spanned for RequireExpression<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        self.span
    }
}
#[derive(Debug, Clone)]
pub struct RequireOnceExpression<'tree> {
    pub span: ::treesitter_types::Span,
    pub children: Expression<'tree>,
}
impl<'tree> ::treesitter_types::FromNode<'tree> for RequireOnceExpression<'tree> {
    #[allow(clippy::match_single_binding, clippy::suspicious_else_formatting)]
    fn from_node(
        node: ::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        debug_assert_eq!(node.kind(), "require_once_expression");
        Ok(Self {
            span: ::treesitter_types::Span::from(node),
            children: {
                #[allow(clippy::suspicious_else_formatting)]
                let non_field_children = {
                    let mut cursor = node.walk();
                    let mut result = ::std::vec::Vec::new();
                    if cursor.goto_first_child() {
                        loop {
                            if cursor.field_name().is_none()
                                && cursor.node().is_named()
                                && !cursor.node().is_extra()
                            {
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
impl ::treesitter_types::Spanned for RequireOnceExpression<'_> {
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
pub struct ScopedCallExpression<'tree> {
    pub span: ::treesitter_types::Span,
    pub arguments: Arguments<'tree>,
    pub name: ScopedCallExpressionName<'tree>,
    pub scope: ScopedCallExpressionScope<'tree>,
}
impl<'tree> ::treesitter_types::FromNode<'tree> for ScopedCallExpression<'tree> {
    #[allow(clippy::match_single_binding, clippy::suspicious_else_formatting)]
    fn from_node(
        node: ::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        debug_assert_eq!(node.kind(), "scoped_call_expression");
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
                <ScopedCallExpressionName as ::treesitter_types::FromNode>::from_node(child, src)?
            },
            scope: {
                let child = node
                    .child_by_field_name("scope")
                    .ok_or_else(|| ::treesitter_types::ParseError::missing_field("scope", node))?;
                <ScopedCallExpressionScope as ::treesitter_types::FromNode>::from_node(child, src)?
            },
        })
    }
}
impl ::treesitter_types::Spanned for ScopedCallExpression<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        self.span
    }
}
#[derive(Debug, Clone)]
pub struct ScopedPropertyAccessExpression<'tree> {
    pub span: ::treesitter_types::Span,
    pub name: ScopedPropertyAccessExpressionName<'tree>,
    pub scope: ScopedPropertyAccessExpressionScope<'tree>,
}
impl<'tree> ::treesitter_types::FromNode<'tree> for ScopedPropertyAccessExpression<'tree> {
    #[allow(clippy::match_single_binding, clippy::suspicious_else_formatting)]
    fn from_node(
        node: ::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        debug_assert_eq!(node.kind(), "scoped_property_access_expression");
        Ok(Self {
            span: ::treesitter_types::Span::from(node),
            name: {
                let child = node
                    .child_by_field_name("name")
                    .ok_or_else(|| ::treesitter_types::ParseError::missing_field("name", node))?;
                <ScopedPropertyAccessExpressionName as ::treesitter_types::FromNode>::from_node(
                    child, src,
                )?
            },
            scope: {
                let child = node
                    .child_by_field_name("scope")
                    .ok_or_else(|| ::treesitter_types::ParseError::missing_field("scope", node))?;
                <ScopedPropertyAccessExpressionScope as ::treesitter_types::FromNode>::from_node(
                    child, src,
                )?
            },
        })
    }
}
impl ::treesitter_types::Spanned for ScopedPropertyAccessExpression<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        self.span
    }
}
#[derive(Debug, Clone)]
pub struct SequenceExpression<'tree> {
    pub span: ::treesitter_types::Span,
    pub children: ::std::vec::Vec<SequenceExpressionChildren<'tree>>,
}
impl<'tree> ::treesitter_types::FromNode<'tree> for SequenceExpression<'tree> {
    #[allow(clippy::match_single_binding, clippy::suspicious_else_formatting)]
    fn from_node(
        node: ::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        debug_assert_eq!(node.kind(), "sequence_expression");
        Ok(Self {
            span: ::treesitter_types::Span::from(node),
            children: {
                #[allow(clippy::suspicious_else_formatting)]
                let non_field_children = {
                    let mut cursor = node.walk();
                    let mut result = ::std::vec::Vec::new();
                    if cursor.goto_first_child() {
                        loop {
                            if cursor.field_name().is_none()
                                && cursor.node().is_named()
                                && !cursor.node().is_extra()
                            {
                                result.push(cursor.node());
                            }
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
                        <SequenceExpressionChildren as ::treesitter_types::FromNode>::from_node(
                            child, src,
                        )?,
                    );
                }
                items
            },
        })
    }
}
impl ::treesitter_types::Spanned for SequenceExpression<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        self.span
    }
}
#[derive(Debug, Clone)]
pub struct ShellCommandExpression<'tree> {
    pub span: ::treesitter_types::Span,
    pub children: ::std::vec::Vec<ShellCommandExpressionChildren<'tree>>,
}
impl<'tree> ::treesitter_types::FromNode<'tree> for ShellCommandExpression<'tree> {
    #[allow(clippy::match_single_binding, clippy::suspicious_else_formatting)]
    fn from_node(
        node: ::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        debug_assert_eq!(node.kind(), "shell_command_expression");
        Ok(Self {
            span: ::treesitter_types::Span::from(node),
            children: {
                #[allow(clippy::suspicious_else_formatting)]
                let non_field_children = {
                    let mut cursor = node.walk();
                    let mut result = ::std::vec::Vec::new();
                    if cursor.goto_first_child() {
                        loop {
                            if cursor.field_name().is_none()
                                && cursor.node().is_named()
                                && !cursor.node().is_extra()
                            {
                                result.push(cursor.node());
                            }
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
                            <ShellCommandExpressionChildren as ::treesitter_types::FromNode>::from_node(
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
impl ::treesitter_types::Spanned for ShellCommandExpression<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        self.span
    }
}
#[derive(Debug, Clone)]
pub struct SimpleParameter<'tree> {
    pub span: ::treesitter_types::Span,
    pub attributes: ::core::option::Option<AttributeList<'tree>>,
    pub default_value: ::core::option::Option<Expression<'tree>>,
    pub name: VariableName<'tree>,
    pub reference_modifier: ::core::option::Option<ReferenceModifier<'tree>>,
    pub r#type: ::core::option::Option<Type<'tree>>,
}
impl<'tree> ::treesitter_types::FromNode<'tree> for SimpleParameter<'tree> {
    #[allow(clippy::match_single_binding, clippy::suspicious_else_formatting)]
    fn from_node(
        node: ::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        debug_assert_eq!(node.kind(), "simple_parameter");
        Ok(Self {
            span: ::treesitter_types::Span::from(node),
            attributes: match node.child_by_field_name("attributes") {
                Some(child) => Some(<AttributeList as ::treesitter_types::FromNode>::from_node(
                    child, src,
                )?),
                None => None,
            },
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
                <VariableName as ::treesitter_types::FromNode>::from_node(child, src)?
            },
            reference_modifier: match node.child_by_field_name("reference_modifier") {
                Some(child) => Some(
                    <ReferenceModifier as ::treesitter_types::FromNode>::from_node(child, src)?,
                ),
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
impl ::treesitter_types::Spanned for SimpleParameter<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        self.span
    }
}
#[derive(Debug, Clone)]
pub struct StaticModifier<'tree> {
    pub span: ::treesitter_types::Span,
    text: &'tree str,
}
impl<'tree> ::treesitter_types::FromNode<'tree> for StaticModifier<'tree> {
    fn from_node(
        node: ::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        debug_assert_eq!(node.kind(), "static_modifier");
        Ok(Self {
            span: ::treesitter_types::Span::from(node),
            text: node.utf8_text(src)?,
        })
    }
}
impl<'tree> ::treesitter_types::LeafNode<'tree> for StaticModifier<'tree> {
    fn text(&self) -> &'tree str {
        self.text
    }
}
impl ::treesitter_types::Spanned for StaticModifier<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        self.span
    }
}
#[derive(Debug, Clone)]
pub struct StaticVariableDeclaration<'tree> {
    pub span: ::treesitter_types::Span,
    pub name: VariableName<'tree>,
    pub value: ::core::option::Option<Expression<'tree>>,
}
impl<'tree> ::treesitter_types::FromNode<'tree> for StaticVariableDeclaration<'tree> {
    #[allow(clippy::match_single_binding, clippy::suspicious_else_formatting)]
    fn from_node(
        node: ::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        debug_assert_eq!(node.kind(), "static_variable_declaration");
        Ok(Self {
            span: ::treesitter_types::Span::from(node),
            name: {
                let child = node
                    .child_by_field_name("name")
                    .ok_or_else(|| ::treesitter_types::ParseError::missing_field("name", node))?;
                <VariableName as ::treesitter_types::FromNode>::from_node(child, src)?
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
impl ::treesitter_types::Spanned for StaticVariableDeclaration<'_> {
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
pub struct SubscriptExpression<'tree> {
    pub span: ::treesitter_types::Span,
    pub children: ::std::vec::Vec<SubscriptExpressionChildren<'tree>>,
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
            children: {
                #[allow(clippy::suspicious_else_formatting)]
                let non_field_children = {
                    let mut cursor = node.walk();
                    let mut result = ::std::vec::Vec::new();
                    if cursor.goto_first_child() {
                        loop {
                            if cursor.field_name().is_none()
                                && cursor.node().is_named()
                                && !cursor.node().is_extra()
                            {
                                result.push(cursor.node());
                            }
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
                        <SubscriptExpressionChildren as ::treesitter_types::FromNode>::from_node(
                            child, src,
                        )?,
                    );
                }
                items
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
pub struct SwitchBlock<'tree> {
    pub span: ::treesitter_types::Span,
    pub children: ::std::vec::Vec<SwitchBlockChildren<'tree>>,
}
impl<'tree> ::treesitter_types::FromNode<'tree> for SwitchBlock<'tree> {
    #[allow(clippy::match_single_binding, clippy::suspicious_else_formatting)]
    fn from_node(
        node: ::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        debug_assert_eq!(node.kind(), "switch_block");
        Ok(Self {
            span: ::treesitter_types::Span::from(node),
            children: {
                #[allow(clippy::suspicious_else_formatting)]
                let non_field_children = {
                    let mut cursor = node.walk();
                    let mut result = ::std::vec::Vec::new();
                    if cursor.goto_first_child() {
                        loop {
                            if cursor.field_name().is_none()
                                && cursor.node().is_named()
                                && !cursor.node().is_extra()
                            {
                                result.push(cursor.node());
                            }
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
                        <SwitchBlockChildren as ::treesitter_types::FromNode>::from_node(
                            child, src,
                        )?,
                    );
                }
                items
            },
        })
    }
}
impl ::treesitter_types::Spanned for SwitchBlock<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        self.span
    }
}
#[derive(Debug, Clone)]
pub struct SwitchStatement<'tree> {
    pub span: ::treesitter_types::Span,
    pub body: SwitchBlock<'tree>,
    pub condition: ParenthesizedExpression<'tree>,
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
                <SwitchBlock as ::treesitter_types::FromNode>::from_node(child, src)?
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
impl ::treesitter_types::Spanned for SwitchStatement<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        self.span
    }
}
#[derive(Debug, Clone)]
pub struct Text<'tree> {
    pub span: ::treesitter_types::Span,
    text: &'tree str,
}
impl<'tree> ::treesitter_types::FromNode<'tree> for Text<'tree> {
    fn from_node(
        node: ::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        debug_assert_eq!(node.kind(), "text");
        Ok(Self {
            span: ::treesitter_types::Span::from(node),
            text: node.utf8_text(src)?,
        })
    }
}
impl<'tree> ::treesitter_types::LeafNode<'tree> for Text<'tree> {
    fn text(&self) -> &'tree str {
        self.text
    }
}
impl ::treesitter_types::Spanned for Text<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        self.span
    }
}
#[derive(Debug, Clone)]
pub struct TextInterpolation<'tree> {
    pub span: ::treesitter_types::Span,
    pub children: ::std::vec::Vec<TextInterpolationChildren<'tree>>,
}
impl<'tree> ::treesitter_types::FromNode<'tree> for TextInterpolation<'tree> {
    #[allow(clippy::match_single_binding, clippy::suspicious_else_formatting)]
    fn from_node(
        node: ::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        debug_assert_eq!(node.kind(), "text_interpolation");
        Ok(Self {
            span: ::treesitter_types::Span::from(node),
            children: {
                #[allow(clippy::suspicious_else_formatting)]
                let non_field_children = {
                    let mut cursor = node.walk();
                    let mut result = ::std::vec::Vec::new();
                    if cursor.goto_first_child() {
                        loop {
                            if cursor.field_name().is_none()
                                && cursor.node().is_named()
                                && !cursor.node().is_extra()
                            {
                                result.push(cursor.node());
                            }
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
                        <TextInterpolationChildren as ::treesitter_types::FromNode>::from_node(
                            child, src,
                        )?,
                    );
                }
                items
            },
        })
    }
}
impl ::treesitter_types::Spanned for TextInterpolation<'_> {
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
pub struct TraitDeclaration<'tree> {
    pub span: ::treesitter_types::Span,
    pub attributes: ::core::option::Option<AttributeList<'tree>>,
    pub body: DeclarationList<'tree>,
    pub name: Name<'tree>,
}
impl<'tree> ::treesitter_types::FromNode<'tree> for TraitDeclaration<'tree> {
    #[allow(clippy::match_single_binding, clippy::suspicious_else_formatting)]
    fn from_node(
        node: ::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        debug_assert_eq!(node.kind(), "trait_declaration");
        Ok(Self {
            span: ::treesitter_types::Span::from(node),
            attributes: match node.child_by_field_name("attributes") {
                Some(child) => Some(<AttributeList as ::treesitter_types::FromNode>::from_node(
                    child, src,
                )?),
                None => None,
            },
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
                <Name as ::treesitter_types::FromNode>::from_node(child, src)?
            },
        })
    }
}
impl ::treesitter_types::Spanned for TraitDeclaration<'_> {
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
pub struct TypeList<'tree> {
    pub span: ::treesitter_types::Span,
    pub children: ::std::vec::Vec<NamedType<'tree>>,
}
impl<'tree> ::treesitter_types::FromNode<'tree> for TypeList<'tree> {
    #[allow(clippy::match_single_binding, clippy::suspicious_else_formatting)]
    fn from_node(
        node: ::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        debug_assert_eq!(node.kind(), "type_list");
        Ok(Self {
            span: ::treesitter_types::Span::from(node),
            children: {
                #[allow(clippy::suspicious_else_formatting)]
                let non_field_children = {
                    let mut cursor = node.walk();
                    let mut result = ::std::vec::Vec::new();
                    if cursor.goto_first_child() {
                        loop {
                            if cursor.field_name().is_none()
                                && cursor.node().is_named()
                                && !cursor.node().is_extra()
                            {
                                result.push(cursor.node());
                            }
                            if !cursor.goto_next_sibling() {
                                break;
                            }
                        }
                    }
                    result
                };
                let mut items = ::std::vec::Vec::new();
                for child in non_field_children {
                    items.push(<NamedType as ::treesitter_types::FromNode>::from_node(
                        child, src,
                    )?);
                }
                items
            },
        })
    }
}
impl ::treesitter_types::Spanned for TypeList<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        self.span
    }
}
#[derive(Debug, Clone)]
pub struct UnaryOpExpression<'tree> {
    pub span: ::treesitter_types::Span,
    pub argument: ::core::option::Option<Expression<'tree>>,
    pub operator: ::core::option::Option<UnaryOpExpressionOperator>,
    pub children: ::core::option::Option<Integer<'tree>>,
}
impl<'tree> ::treesitter_types::FromNode<'tree> for UnaryOpExpression<'tree> {
    #[allow(clippy::match_single_binding, clippy::suspicious_else_formatting)]
    fn from_node(
        node: ::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        debug_assert_eq!(node.kind(), "unary_op_expression");
        Ok(Self {
            span: ::treesitter_types::Span::from(node),
            argument: match node.child_by_field_name("argument") {
                Some(child) => Some(<Expression as ::treesitter_types::FromNode>::from_node(
                    child, src,
                )?),
                None => None,
            },
            operator: match node.child_by_field_name("operator") {
                Some(child) => Some(
                    <UnaryOpExpressionOperator as ::treesitter_types::FromNode>::from_node(
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
                    Some(&child) => Some(<Integer as ::treesitter_types::FromNode>::from_node(
                        child, src,
                    )?),
                    None => None,
                }
            },
        })
    }
}
impl ::treesitter_types::Spanned for UnaryOpExpression<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        self.span
    }
}
#[derive(Debug, Clone)]
pub struct UnionType<'tree> {
    pub span: ::treesitter_types::Span,
    pub children: ::std::vec::Vec<UnionTypeChildren<'tree>>,
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
                    items.push(
                        <UnionTypeChildren as ::treesitter_types::FromNode>::from_node(child, src)?,
                    );
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
pub struct UnsetStatement<'tree> {
    pub span: ::treesitter_types::Span,
    pub children: ::std::vec::Vec<UnsetStatementChildren<'tree>>,
}
impl<'tree> ::treesitter_types::FromNode<'tree> for UnsetStatement<'tree> {
    #[allow(clippy::match_single_binding, clippy::suspicious_else_formatting)]
    fn from_node(
        node: ::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        debug_assert_eq!(node.kind(), "unset_statement");
        Ok(Self {
            span: ::treesitter_types::Span::from(node),
            children: {
                #[allow(clippy::suspicious_else_formatting)]
                let non_field_children = {
                    let mut cursor = node.walk();
                    let mut result = ::std::vec::Vec::new();
                    if cursor.goto_first_child() {
                        loop {
                            if cursor.field_name().is_none()
                                && cursor.node().is_named()
                                && !cursor.node().is_extra()
                            {
                                result.push(cursor.node());
                            }
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
                        <UnsetStatementChildren as ::treesitter_types::FromNode>::from_node(
                            child, src,
                        )?,
                    );
                }
                items
            },
        })
    }
}
impl ::treesitter_types::Spanned for UnsetStatement<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        self.span
    }
}
#[derive(Debug, Clone)]
pub struct UpdateExpression<'tree> {
    pub span: ::treesitter_types::Span,
    pub argument: UpdateExpressionArgument<'tree>,
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
                <UpdateExpressionArgument as ::treesitter_types::FromNode>::from_node(child, src)?
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
pub struct UseAsClause<'tree> {
    pub span: ::treesitter_types::Span,
    pub children: ::std::vec::Vec<UseAsClauseChildren<'tree>>,
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
            children: {
                #[allow(clippy::suspicious_else_formatting)]
                let non_field_children = {
                    let mut cursor = node.walk();
                    let mut result = ::std::vec::Vec::new();
                    if cursor.goto_first_child() {
                        loop {
                            if cursor.field_name().is_none()
                                && cursor.node().is_named()
                                && !cursor.node().is_extra()
                            {
                                result.push(cursor.node());
                            }
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
                        <UseAsClauseChildren as ::treesitter_types::FromNode>::from_node(
                            child, src,
                        )?,
                    );
                }
                items
            },
        })
    }
}
impl ::treesitter_types::Spanned for UseAsClause<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        self.span
    }
}
#[derive(Debug, Clone)]
pub struct UseDeclaration<'tree> {
    pub span: ::treesitter_types::Span,
    pub children: ::std::vec::Vec<UseDeclarationChildren<'tree>>,
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
            children: {
                #[allow(clippy::suspicious_else_formatting)]
                let non_field_children = {
                    let mut cursor = node.walk();
                    let mut result = ::std::vec::Vec::new();
                    if cursor.goto_first_child() {
                        loop {
                            if cursor.field_name().is_none()
                                && cursor.node().is_named()
                                && !cursor.node().is_extra()
                            {
                                result.push(cursor.node());
                            }
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
                        <UseDeclarationChildren as ::treesitter_types::FromNode>::from_node(
                            child, src,
                        )?,
                    );
                }
                items
            },
        })
    }
}
impl ::treesitter_types::Spanned for UseDeclaration<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        self.span
    }
}
#[derive(Debug, Clone)]
pub struct UseInsteadOfClause<'tree> {
    pub span: ::treesitter_types::Span,
    pub children: ::std::vec::Vec<UseInsteadOfClauseChildren<'tree>>,
}
impl<'tree> ::treesitter_types::FromNode<'tree> for UseInsteadOfClause<'tree> {
    #[allow(clippy::match_single_binding, clippy::suspicious_else_formatting)]
    fn from_node(
        node: ::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        debug_assert_eq!(node.kind(), "use_instead_of_clause");
        Ok(Self {
            span: ::treesitter_types::Span::from(node),
            children: {
                #[allow(clippy::suspicious_else_formatting)]
                let non_field_children = {
                    let mut cursor = node.walk();
                    let mut result = ::std::vec::Vec::new();
                    if cursor.goto_first_child() {
                        loop {
                            if cursor.field_name().is_none()
                                && cursor.node().is_named()
                                && !cursor.node().is_extra()
                            {
                                result.push(cursor.node());
                            }
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
                        <UseInsteadOfClauseChildren as ::treesitter_types::FromNode>::from_node(
                            child, src,
                        )?,
                    );
                }
                items
            },
        })
    }
}
impl ::treesitter_types::Spanned for UseInsteadOfClause<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        self.span
    }
}
#[derive(Debug, Clone)]
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
                    items.push(
                        <UseListChildren as ::treesitter_types::FromNode>::from_node(child, src)?,
                    );
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
#[derive(Debug, Clone)]
pub struct VariableName<'tree> {
    pub span: ::treesitter_types::Span,
    pub children: Name<'tree>,
}
impl<'tree> ::treesitter_types::FromNode<'tree> for VariableName<'tree> {
    #[allow(clippy::match_single_binding, clippy::suspicious_else_formatting)]
    fn from_node(
        node: ::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        debug_assert_eq!(node.kind(), "variable_name");
        Ok(Self {
            span: ::treesitter_types::Span::from(node),
            children: {
                #[allow(clippy::suspicious_else_formatting)]
                let non_field_children = {
                    let mut cursor = node.walk();
                    let mut result = ::std::vec::Vec::new();
                    if cursor.goto_first_child() {
                        loop {
                            if cursor.field_name().is_none()
                                && cursor.node().is_named()
                                && !cursor.node().is_extra()
                            {
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
                                        <Name as ::treesitter_types::FromNode>::from_node(
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
                <Name as ::treesitter_types::FromNode>::from_node(child, src)?
            },
        })
    }
}
impl ::treesitter_types::Spanned for VariableName<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        self.span
    }
}
#[derive(Debug, Clone)]
pub struct VariadicParameter<'tree> {
    pub span: ::treesitter_types::Span,
    pub attributes: ::core::option::Option<AttributeList<'tree>>,
    pub name: VariableName<'tree>,
    pub reference_modifier: ::core::option::Option<ReferenceModifier<'tree>>,
    pub r#type: ::core::option::Option<Type<'tree>>,
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
            attributes: match node.child_by_field_name("attributes") {
                Some(child) => Some(<AttributeList as ::treesitter_types::FromNode>::from_node(
                    child, src,
                )?),
                None => None,
            },
            name: {
                let child = node
                    .child_by_field_name("name")
                    .ok_or_else(|| ::treesitter_types::ParseError::missing_field("name", node))?;
                <VariableName as ::treesitter_types::FromNode>::from_node(child, src)?
            },
            reference_modifier: match node.child_by_field_name("reference_modifier") {
                Some(child) => Some(
                    <ReferenceModifier as ::treesitter_types::FromNode>::from_node(child, src)?,
                ),
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
impl ::treesitter_types::Spanned for VariadicParameter<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        self.span
    }
}
#[derive(Debug, Clone)]
pub struct VariadicPlaceholder<'tree> {
    pub span: ::treesitter_types::Span,
    text: &'tree str,
}
impl<'tree> ::treesitter_types::FromNode<'tree> for VariadicPlaceholder<'tree> {
    fn from_node(
        node: ::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        debug_assert_eq!(node.kind(), "variadic_placeholder");
        Ok(Self {
            span: ::treesitter_types::Span::from(node),
            text: node.utf8_text(src)?,
        })
    }
}
impl<'tree> ::treesitter_types::LeafNode<'tree> for VariadicPlaceholder<'tree> {
    fn text(&self) -> &'tree str {
        self.text
    }
}
impl ::treesitter_types::Spanned for VariadicPlaceholder<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        self.span
    }
}
#[derive(Debug, Clone)]
pub struct VariadicUnpacking<'tree> {
    pub span: ::treesitter_types::Span,
    pub children: Expression<'tree>,
}
impl<'tree> ::treesitter_types::FromNode<'tree> for VariadicUnpacking<'tree> {
    #[allow(clippy::match_single_binding, clippy::suspicious_else_formatting)]
    fn from_node(
        node: ::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        debug_assert_eq!(node.kind(), "variadic_unpacking");
        Ok(Self {
            span: ::treesitter_types::Span::from(node),
            children: {
                #[allow(clippy::suspicious_else_formatting)]
                let non_field_children = {
                    let mut cursor = node.walk();
                    let mut result = ::std::vec::Vec::new();
                    if cursor.goto_first_child() {
                        loop {
                            if cursor.field_name().is_none()
                                && cursor.node().is_named()
                                && !cursor.node().is_extra()
                            {
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
impl ::treesitter_types::Spanned for VariadicUnpacking<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        self.span
    }
}
#[derive(Debug, Clone)]
pub struct VisibilityModifier<'tree> {
    pub span: ::treesitter_types::Span,
    pub children: ::core::option::Option<Operation<'tree>>,
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
                    Some(&child) => Some(<Operation as ::treesitter_types::FromNode>::from_node(
                        child, src,
                    )?),
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
#[derive(Debug, Clone)]
pub struct WhileStatement<'tree> {
    pub span: ::treesitter_types::Span,
    pub body: WhileStatementBody<'tree>,
    pub condition: ParenthesizedExpression<'tree>,
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
                <WhileStatementBody as ::treesitter_types::FromNode>::from_node(child, src)?
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
impl ::treesitter_types::Spanned for WhileStatement<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        self.span
    }
}
#[derive(Debug, Clone)]
pub struct YieldExpression<'tree> {
    pub span: ::treesitter_types::Span,
    pub children: ::core::option::Option<YieldExpressionChildren<'tree>>,
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
                    Some(&child) => Some(
                        <YieldExpressionChildren as ::treesitter_types::FromNode>::from_node(
                            child, src,
                        )?,
                    ),
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
#[derive(Debug, Clone)]
pub struct BottomType<'tree> {
    pub span: ::treesitter_types::Span,
    text: &'tree str,
}
impl<'tree> ::treesitter_types::FromNode<'tree> for BottomType<'tree> {
    fn from_node(
        node: ::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        debug_assert_eq!(node.kind(), "bottom_type");
        Ok(Self {
            span: ::treesitter_types::Span::from(node),
            text: node.utf8_text(src)?,
        })
    }
}
impl<'tree> ::treesitter_types::LeafNode<'tree> for BottomType<'tree> {
    fn text(&self) -> &'tree str {
        self.text
    }
}
impl ::treesitter_types::Spanned for BottomType<'_> {
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
pub struct NowdocString<'tree> {
    pub span: ::treesitter_types::Span,
    text: &'tree str,
}
impl<'tree> ::treesitter_types::FromNode<'tree> for NowdocString<'tree> {
    fn from_node(
        node: ::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        debug_assert_eq!(node.kind(), "nowdoc_string");
        Ok(Self {
            span: ::treesitter_types::Span::from(node),
            text: node.utf8_text(src)?,
        })
    }
}
impl<'tree> ::treesitter_types::LeafNode<'tree> for NowdocString<'tree> {
    fn text(&self) -> &'tree str {
        self.text
    }
}
impl ::treesitter_types::Spanned for NowdocString<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        self.span
    }
}
#[derive(Debug, Clone)]
pub struct Operation<'tree> {
    pub span: ::treesitter_types::Span,
    text: &'tree str,
}
impl<'tree> ::treesitter_types::FromNode<'tree> for Operation<'tree> {
    fn from_node(
        node: ::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        debug_assert_eq!(node.kind(), "operation");
        Ok(Self {
            span: ::treesitter_types::Span::from(node),
            text: node.utf8_text(src)?,
        })
    }
}
impl<'tree> ::treesitter_types::LeafNode<'tree> for Operation<'tree> {
    fn text(&self) -> &'tree str {
        self.text
    }
}
impl ::treesitter_types::Spanned for Operation<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        self.span
    }
}
#[derive(Debug, Clone)]
pub struct PhpEndTag<'tree> {
    pub span: ::treesitter_types::Span,
    text: &'tree str,
}
impl<'tree> ::treesitter_types::FromNode<'tree> for PhpEndTag<'tree> {
    fn from_node(
        node: ::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        debug_assert_eq!(node.kind(), "php_end_tag");
        Ok(Self {
            span: ::treesitter_types::Span::from(node),
            text: node.utf8_text(src)?,
        })
    }
}
impl<'tree> ::treesitter_types::LeafNode<'tree> for PhpEndTag<'tree> {
    fn text(&self) -> &'tree str {
        self.text
    }
}
impl ::treesitter_types::Spanned for PhpEndTag<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        self.span
    }
}
#[derive(Debug, Clone)]
pub struct PhpTag<'tree> {
    pub span: ::treesitter_types::Span,
    text: &'tree str,
}
impl<'tree> ::treesitter_types::FromNode<'tree> for PhpTag<'tree> {
    fn from_node(
        node: ::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        debug_assert_eq!(node.kind(), "php_tag");
        Ok(Self {
            span: ::treesitter_types::Span::from(node),
            text: node.utf8_text(src)?,
        })
    }
}
impl<'tree> ::treesitter_types::LeafNode<'tree> for PhpTag<'tree> {
    fn text(&self) -> &'tree str {
        self.text
    }
}
impl ::treesitter_types::Spanned for PhpTag<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        self.span
    }
}
#[derive(Debug, Clone)]
pub struct VarModifier<'tree> {
    pub span: ::treesitter_types::Span,
    text: &'tree str,
}
impl<'tree> ::treesitter_types::FromNode<'tree> for VarModifier<'tree> {
    fn from_node(
        node: ::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        debug_assert_eq!(node.kind(), "var_modifier");
        Ok(Self {
            span: ::treesitter_types::Span::from(node),
            text: node.utf8_text(src)?,
        })
    }
}
impl<'tree> ::treesitter_types::LeafNode<'tree> for VarModifier<'tree> {
    fn text(&self) -> &'tree str {
        self.text
    }
}
impl ::treesitter_types::Spanned for VarModifier<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        self.span
    }
}
#[derive(Debug, Clone)]
pub enum AnonymousClassChildren<'tree> {
    AbstractModifier(::std::boxed::Box<AbstractModifier<'tree>>),
    Arguments(::std::boxed::Box<Arguments<'tree>>),
    BaseClause(::std::boxed::Box<BaseClause<'tree>>),
    ClassInterfaceClause(::std::boxed::Box<ClassInterfaceClause<'tree>>),
    FinalModifier(::std::boxed::Box<FinalModifier<'tree>>),
    ReadonlyModifier(::std::boxed::Box<ReadonlyModifier<'tree>>),
    StaticModifier(::std::boxed::Box<StaticModifier<'tree>>),
    VarModifier(::std::boxed::Box<VarModifier<'tree>>),
    VisibilityModifier(::std::boxed::Box<VisibilityModifier<'tree>>),
}
impl<'tree> ::treesitter_types::FromNode<'tree> for AnonymousClassChildren<'tree> {
    #[allow(clippy::collapsible_else_if)]
    fn from_node(
        node: ::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        match node.kind() {
            "abstract_modifier" => Ok(Self::AbstractModifier(::std::boxed::Box::new(
                <AbstractModifier as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "arguments" => Ok(Self::Arguments(::std::boxed::Box::new(
                <Arguments as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "base_clause" => Ok(Self::BaseClause(::std::boxed::Box::new(
                <BaseClause as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "class_interface_clause" => Ok(Self::ClassInterfaceClause(::std::boxed::Box::new(
                <ClassInterfaceClause as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "final_modifier" => Ok(Self::FinalModifier(::std::boxed::Box::new(
                <FinalModifier as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "readonly_modifier" => Ok(Self::ReadonlyModifier(::std::boxed::Box::new(
                <ReadonlyModifier as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "static_modifier" => Ok(Self::StaticModifier(::std::boxed::Box::new(
                <StaticModifier as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "var_modifier" => Ok(Self::VarModifier(::std::boxed::Box::new(
                <VarModifier as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "visibility_modifier" => Ok(Self::VisibilityModifier(::std::boxed::Box::new(
                <VisibilityModifier as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            other => Err(::treesitter_types::ParseError::unexpected_kind(other, node)),
        }
    }
}
impl ::treesitter_types::Spanned for AnonymousClassChildren<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        match self {
            Self::AbstractModifier(inner) => inner.span(),
            Self::Arguments(inner) => inner.span(),
            Self::BaseClause(inner) => inner.span(),
            Self::ClassInterfaceClause(inner) => inner.span(),
            Self::FinalModifier(inner) => inner.span(),
            Self::ReadonlyModifier(inner) => inner.span(),
            Self::StaticModifier(inner) => inner.span(),
            Self::VarModifier(inner) => inner.span(),
            Self::VisibilityModifier(inner) => inner.span(),
        }
    }
}
#[derive(Debug, Clone)]
pub enum AnonymousFunctionReturnType<'tree> {
    BottomType(::std::boxed::Box<BottomType<'tree>>),
    Type(::std::boxed::Box<Type<'tree>>),
}
impl<'tree> ::treesitter_types::FromNode<'tree> for AnonymousFunctionReturnType<'tree> {
    #[allow(clippy::collapsible_else_if)]
    fn from_node(
        node: ::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        match node.kind() {
            "bottom_type" => Ok(Self::BottomType(::std::boxed::Box::new(
                <BottomType as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
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
impl ::treesitter_types::Spanned for AnonymousFunctionReturnType<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        match self {
            Self::BottomType(inner) => inner.span(),
            Self::Type(inner) => inner.span(),
        }
    }
}
#[derive(Debug, Clone)]
pub enum AnonymousFunctionUseClauseChildren<'tree> {
    ByRef(::std::boxed::Box<ByRef<'tree>>),
    VariableName(::std::boxed::Box<VariableName<'tree>>),
}
impl<'tree> ::treesitter_types::FromNode<'tree> for AnonymousFunctionUseClauseChildren<'tree> {
    #[allow(clippy::collapsible_else_if)]
    fn from_node(
        node: ::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        match node.kind() {
            "by_ref" => Ok(Self::ByRef(::std::boxed::Box::new(
                <ByRef as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "variable_name" => Ok(Self::VariableName(::std::boxed::Box::new(
                <VariableName as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            other => Err(::treesitter_types::ParseError::unexpected_kind(other, node)),
        }
    }
}
impl ::treesitter_types::Spanned for AnonymousFunctionUseClauseChildren<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        match self {
            Self::ByRef(inner) => inner.span(),
            Self::VariableName(inner) => inner.span(),
        }
    }
}
#[derive(Debug, Clone)]
pub enum ArgumentChildren<'tree> {
    Expression(::std::boxed::Box<Expression<'tree>>),
    Name(::std::boxed::Box<Name<'tree>>),
    VariadicUnpacking(::std::boxed::Box<VariadicUnpacking<'tree>>),
}
impl<'tree> ::treesitter_types::FromNode<'tree> for ArgumentChildren<'tree> {
    #[allow(clippy::collapsible_else_if)]
    fn from_node(
        node: ::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        match node.kind() {
            "name" => Ok(Self::Name(::std::boxed::Box::new(
                <Name as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "variadic_unpacking" => Ok(Self::VariadicUnpacking(::std::boxed::Box::new(
                <VariadicUnpacking as ::treesitter_types::FromNode>::from_node(node, src)?,
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
            Self::Expression(inner) => inner.span(),
            Self::Name(inner) => inner.span(),
            Self::VariadicUnpacking(inner) => inner.span(),
        }
    }
}
#[derive(Debug, Clone)]
pub enum ArgumentsChildren<'tree> {
    Argument(::std::boxed::Box<Argument<'tree>>),
    VariadicPlaceholder(::std::boxed::Box<VariadicPlaceholder<'tree>>),
}
impl<'tree> ::treesitter_types::FromNode<'tree> for ArgumentsChildren<'tree> {
    #[allow(clippy::collapsible_else_if)]
    fn from_node(
        node: ::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        match node.kind() {
            "argument" => Ok(Self::Argument(::std::boxed::Box::new(
                <Argument as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "variadic_placeholder" => Ok(Self::VariadicPlaceholder(::std::boxed::Box::new(
                <VariadicPlaceholder as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            other => Err(::treesitter_types::ParseError::unexpected_kind(other, node)),
        }
    }
}
impl ::treesitter_types::Spanned for ArgumentsChildren<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        match self {
            Self::Argument(inner) => inner.span(),
            Self::VariadicPlaceholder(inner) => inner.span(),
        }
    }
}
#[derive(Debug, Clone)]
pub enum ArrayElementInitializerChildren<'tree> {
    ByRef(::std::boxed::Box<ByRef<'tree>>),
    Expression(::std::boxed::Box<Expression<'tree>>),
    VariadicUnpacking(::std::boxed::Box<VariadicUnpacking<'tree>>),
}
impl<'tree> ::treesitter_types::FromNode<'tree> for ArrayElementInitializerChildren<'tree> {
    #[allow(clippy::collapsible_else_if)]
    fn from_node(
        node: ::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        match node.kind() {
            "by_ref" => Ok(Self::ByRef(::std::boxed::Box::new(
                <ByRef as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "variadic_unpacking" => Ok(Self::VariadicUnpacking(::std::boxed::Box::new(
                <VariadicUnpacking as ::treesitter_types::FromNode>::from_node(node, src)?,
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
impl ::treesitter_types::Spanned for ArrayElementInitializerChildren<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        match self {
            Self::ByRef(inner) => inner.span(),
            Self::Expression(inner) => inner.span(),
            Self::VariadicUnpacking(inner) => inner.span(),
        }
    }
}
#[derive(Debug, Clone)]
pub enum ArrowFunctionReturnType<'tree> {
    BottomType(::std::boxed::Box<BottomType<'tree>>),
    Type(::std::boxed::Box<Type<'tree>>),
}
impl<'tree> ::treesitter_types::FromNode<'tree> for ArrowFunctionReturnType<'tree> {
    #[allow(clippy::collapsible_else_if)]
    fn from_node(
        node: ::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        match node.kind() {
            "bottom_type" => Ok(Self::BottomType(::std::boxed::Box::new(
                <BottomType as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
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
impl ::treesitter_types::Spanned for ArrowFunctionReturnType<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        match self {
            Self::BottomType(inner) => inner.span(),
            Self::Type(inner) => inner.span(),
        }
    }
}
#[derive(Debug, Clone)]
pub enum AssignmentExpressionLeft<'tree> {
    CastExpression(::std::boxed::Box<CastExpression<'tree>>),
    DynamicVariableName(::std::boxed::Box<DynamicVariableName<'tree>>),
    FunctionCallExpression(::std::boxed::Box<FunctionCallExpression<'tree>>),
    ListLiteral(::std::boxed::Box<ListLiteral<'tree>>),
    MemberAccessExpression(::std::boxed::Box<MemberAccessExpression<'tree>>),
    MemberCallExpression(::std::boxed::Box<MemberCallExpression<'tree>>),
    NullsafeMemberAccessExpression(::std::boxed::Box<NullsafeMemberAccessExpression<'tree>>),
    NullsafeMemberCallExpression(::std::boxed::Box<NullsafeMemberCallExpression<'tree>>),
    ScopedCallExpression(::std::boxed::Box<ScopedCallExpression<'tree>>),
    ScopedPropertyAccessExpression(::std::boxed::Box<ScopedPropertyAccessExpression<'tree>>),
    SubscriptExpression(::std::boxed::Box<SubscriptExpression<'tree>>),
    VariableName(::std::boxed::Box<VariableName<'tree>>),
}
impl<'tree> ::treesitter_types::FromNode<'tree> for AssignmentExpressionLeft<'tree> {
    #[allow(clippy::collapsible_else_if)]
    fn from_node(
        node: ::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        match node.kind() {
            "cast_expression" => Ok(Self::CastExpression(::std::boxed::Box::new(
                <CastExpression as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "dynamic_variable_name" => Ok(Self::DynamicVariableName(::std::boxed::Box::new(
                <DynamicVariableName as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "function_call_expression" => Ok(Self::FunctionCallExpression(::std::boxed::Box::new(
                <FunctionCallExpression as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "list_literal" => Ok(Self::ListLiteral(::std::boxed::Box::new(
                <ListLiteral as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "member_access_expression" => Ok(Self::MemberAccessExpression(::std::boxed::Box::new(
                <MemberAccessExpression as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "member_call_expression" => Ok(Self::MemberCallExpression(::std::boxed::Box::new(
                <MemberCallExpression as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "nullsafe_member_access_expression" => Ok(Self::NullsafeMemberAccessExpression(
                ::std::boxed::Box::new(
                    <NullsafeMemberAccessExpression as ::treesitter_types::FromNode>::from_node(
                        node, src,
                    )?,
                ),
            )),
            "nullsafe_member_call_expression" => {
                Ok(Self::NullsafeMemberCallExpression(::std::boxed::Box::new(
                    <NullsafeMemberCallExpression as ::treesitter_types::FromNode>::from_node(
                        node, src,
                    )?,
                )))
            }
            "scoped_call_expression" => Ok(Self::ScopedCallExpression(::std::boxed::Box::new(
                <ScopedCallExpression as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "scoped_property_access_expression" => Ok(Self::ScopedPropertyAccessExpression(
                ::std::boxed::Box::new(
                    <ScopedPropertyAccessExpression as ::treesitter_types::FromNode>::from_node(
                        node, src,
                    )?,
                ),
            )),
            "subscript_expression" => Ok(Self::SubscriptExpression(::std::boxed::Box::new(
                <SubscriptExpression as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "variable_name" => Ok(Self::VariableName(::std::boxed::Box::new(
                <VariableName as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            other => Err(::treesitter_types::ParseError::unexpected_kind(other, node)),
        }
    }
}
impl ::treesitter_types::Spanned for AssignmentExpressionLeft<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        match self {
            Self::CastExpression(inner) => inner.span(),
            Self::DynamicVariableName(inner) => inner.span(),
            Self::FunctionCallExpression(inner) => inner.span(),
            Self::ListLiteral(inner) => inner.span(),
            Self::MemberAccessExpression(inner) => inner.span(),
            Self::MemberCallExpression(inner) => inner.span(),
            Self::NullsafeMemberAccessExpression(inner) => inner.span(),
            Self::NullsafeMemberCallExpression(inner) => inner.span(),
            Self::ScopedCallExpression(inner) => inner.span(),
            Self::ScopedPropertyAccessExpression(inner) => inner.span(),
            Self::SubscriptExpression(inner) => inner.span(),
            Self::VariableName(inner) => inner.span(),
        }
    }
}
#[derive(Debug, Clone)]
pub enum AttributeChildren<'tree> {
    Name(::std::boxed::Box<Name<'tree>>),
    QualifiedName(::std::boxed::Box<QualifiedName<'tree>>),
    RelativeName(::std::boxed::Box<RelativeName<'tree>>),
}
impl<'tree> ::treesitter_types::FromNode<'tree> for AttributeChildren<'tree> {
    #[allow(clippy::collapsible_else_if)]
    fn from_node(
        node: ::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        match node.kind() {
            "name" => Ok(Self::Name(::std::boxed::Box::new(
                <Name as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "qualified_name" => Ok(Self::QualifiedName(::std::boxed::Box::new(
                <QualifiedName as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "relative_name" => Ok(Self::RelativeName(::std::boxed::Box::new(
                <RelativeName as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            other => Err(::treesitter_types::ParseError::unexpected_kind(other, node)),
        }
    }
}
impl ::treesitter_types::Spanned for AttributeChildren<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        match self {
            Self::Name(inner) => inner.span(),
            Self::QualifiedName(inner) => inner.span(),
            Self::RelativeName(inner) => inner.span(),
        }
    }
}
#[derive(Debug, Clone)]
pub enum AugmentedAssignmentExpressionLeft<'tree> {
    CastExpression(::std::boxed::Box<CastExpression<'tree>>),
    DynamicVariableName(::std::boxed::Box<DynamicVariableName<'tree>>),
    FunctionCallExpression(::std::boxed::Box<FunctionCallExpression<'tree>>),
    MemberAccessExpression(::std::boxed::Box<MemberAccessExpression<'tree>>),
    MemberCallExpression(::std::boxed::Box<MemberCallExpression<'tree>>),
    NullsafeMemberAccessExpression(::std::boxed::Box<NullsafeMemberAccessExpression<'tree>>),
    NullsafeMemberCallExpression(::std::boxed::Box<NullsafeMemberCallExpression<'tree>>),
    ScopedCallExpression(::std::boxed::Box<ScopedCallExpression<'tree>>),
    ScopedPropertyAccessExpression(::std::boxed::Box<ScopedPropertyAccessExpression<'tree>>),
    SubscriptExpression(::std::boxed::Box<SubscriptExpression<'tree>>),
    VariableName(::std::boxed::Box<VariableName<'tree>>),
}
impl<'tree> ::treesitter_types::FromNode<'tree> for AugmentedAssignmentExpressionLeft<'tree> {
    #[allow(clippy::collapsible_else_if)]
    fn from_node(
        node: ::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        match node.kind() {
            "cast_expression" => Ok(Self::CastExpression(::std::boxed::Box::new(
                <CastExpression as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "dynamic_variable_name" => Ok(Self::DynamicVariableName(::std::boxed::Box::new(
                <DynamicVariableName as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "function_call_expression" => Ok(Self::FunctionCallExpression(::std::boxed::Box::new(
                <FunctionCallExpression as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "member_access_expression" => Ok(Self::MemberAccessExpression(::std::boxed::Box::new(
                <MemberAccessExpression as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "member_call_expression" => Ok(Self::MemberCallExpression(::std::boxed::Box::new(
                <MemberCallExpression as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "nullsafe_member_access_expression" => Ok(Self::NullsafeMemberAccessExpression(
                ::std::boxed::Box::new(
                    <NullsafeMemberAccessExpression as ::treesitter_types::FromNode>::from_node(
                        node, src,
                    )?,
                ),
            )),
            "nullsafe_member_call_expression" => {
                Ok(Self::NullsafeMemberCallExpression(::std::boxed::Box::new(
                    <NullsafeMemberCallExpression as ::treesitter_types::FromNode>::from_node(
                        node, src,
                    )?,
                )))
            }
            "scoped_call_expression" => Ok(Self::ScopedCallExpression(::std::boxed::Box::new(
                <ScopedCallExpression as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "scoped_property_access_expression" => Ok(Self::ScopedPropertyAccessExpression(
                ::std::boxed::Box::new(
                    <ScopedPropertyAccessExpression as ::treesitter_types::FromNode>::from_node(
                        node, src,
                    )?,
                ),
            )),
            "subscript_expression" => Ok(Self::SubscriptExpression(::std::boxed::Box::new(
                <SubscriptExpression as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "variable_name" => Ok(Self::VariableName(::std::boxed::Box::new(
                <VariableName as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            other => Err(::treesitter_types::ParseError::unexpected_kind(other, node)),
        }
    }
}
impl ::treesitter_types::Spanned for AugmentedAssignmentExpressionLeft<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        match self {
            Self::CastExpression(inner) => inner.span(),
            Self::DynamicVariableName(inner) => inner.span(),
            Self::FunctionCallExpression(inner) => inner.span(),
            Self::MemberAccessExpression(inner) => inner.span(),
            Self::MemberCallExpression(inner) => inner.span(),
            Self::NullsafeMemberAccessExpression(inner) => inner.span(),
            Self::NullsafeMemberCallExpression(inner) => inner.span(),
            Self::ScopedCallExpression(inner) => inner.span(),
            Self::ScopedPropertyAccessExpression(inner) => inner.span(),
            Self::SubscriptExpression(inner) => inner.span(),
            Self::VariableName(inner) => inner.span(),
        }
    }
}
#[derive(Debug, Clone)]
pub enum AugmentedAssignmentExpressionOperator {
    PercentEq(::treesitter_types::Span),
    AmpEq(::treesitter_types::Span),
    StarStarEq(::treesitter_types::Span),
    StarEq(::treesitter_types::Span),
    PlusEq(::treesitter_types::Span),
    MinusEq(::treesitter_types::Span),
    DotEq(::treesitter_types::Span),
    SlashEq(::treesitter_types::Span),
    ShlEq(::treesitter_types::Span),
    ShrEq(::treesitter_types::Span),
    QuestionQuestionEq(::treesitter_types::Span),
    CaretEq(::treesitter_types::Span),
    PipeEq(::treesitter_types::Span),
}
impl<'tree> ::treesitter_types::FromNode<'tree> for AugmentedAssignmentExpressionOperator {
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
            ".=" => Ok(Self::DotEq(::treesitter_types::Span::from(node))),
            "/=" => Ok(Self::SlashEq(::treesitter_types::Span::from(node))),
            "<<=" => Ok(Self::ShlEq(::treesitter_types::Span::from(node))),
            ">>=" => Ok(Self::ShrEq(::treesitter_types::Span::from(node))),
            "??=" => Ok(Self::QuestionQuestionEq(::treesitter_types::Span::from(
                node,
            ))),
            "^=" => Ok(Self::CaretEq(::treesitter_types::Span::from(node))),
            "|=" => Ok(Self::PipeEq(::treesitter_types::Span::from(node))),
            other => Err(::treesitter_types::ParseError::unexpected_kind(other, node)),
        }
    }
}
impl ::treesitter_types::Spanned for AugmentedAssignmentExpressionOperator {
    fn span(&self) -> ::treesitter_types::Span {
        match self {
            Self::PercentEq(span) => *span,
            Self::AmpEq(span) => *span,
            Self::StarStarEq(span) => *span,
            Self::StarEq(span) => *span,
            Self::PlusEq(span) => *span,
            Self::MinusEq(span) => *span,
            Self::DotEq(span) => *span,
            Self::SlashEq(span) => *span,
            Self::ShlEq(span) => *span,
            Self::ShrEq(span) => *span,
            Self::QuestionQuestionEq(span) => *span,
            Self::CaretEq(span) => *span,
            Self::PipeEq(span) => *span,
        }
    }
}
#[derive(Debug, Clone)]
pub enum BaseClauseChildren<'tree> {
    Name(::std::boxed::Box<Name<'tree>>),
    QualifiedName(::std::boxed::Box<QualifiedName<'tree>>),
    RelativeName(::std::boxed::Box<RelativeName<'tree>>),
}
impl<'tree> ::treesitter_types::FromNode<'tree> for BaseClauseChildren<'tree> {
    #[allow(clippy::collapsible_else_if)]
    fn from_node(
        node: ::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        match node.kind() {
            "name" => Ok(Self::Name(::std::boxed::Box::new(
                <Name as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "qualified_name" => Ok(Self::QualifiedName(::std::boxed::Box::new(
                <QualifiedName as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "relative_name" => Ok(Self::RelativeName(::std::boxed::Box::new(
                <RelativeName as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            other => Err(::treesitter_types::ParseError::unexpected_kind(other, node)),
        }
    }
}
impl ::treesitter_types::Spanned for BaseClauseChildren<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        match self {
            Self::Name(inner) => inner.span(),
            Self::QualifiedName(inner) => inner.span(),
            Self::RelativeName(inner) => inner.span(),
        }
    }
}
#[derive(Debug, Clone)]
pub enum BinaryExpressionOperator {
    NotEq(::treesitter_types::Span),
    BangEqEq(::treesitter_types::Span),
    Percent(::treesitter_types::Span),
    Amp(::treesitter_types::Span),
    AmpAmp(::treesitter_types::Span),
    Star(::treesitter_types::Span),
    StarStar(::treesitter_types::Span),
    Plus(::treesitter_types::Span),
    Minus(::treesitter_types::Span),
    Dot(::treesitter_types::Span),
    Slash(::treesitter_types::Span),
    Lt(::treesitter_types::Span),
    Shl(::treesitter_types::Span),
    LtEq(::treesitter_types::Span),
    LtEqGt(::treesitter_types::Span),
    LtGt(::treesitter_types::Span),
    EqEq(::treesitter_types::Span),
    EqEqEq(::treesitter_types::Span),
    Gt(::treesitter_types::Span),
    GtEq(::treesitter_types::Span),
    Shr(::treesitter_types::Span),
    QuestionQuestion(::treesitter_types::Span),
    Caret(::treesitter_types::Span),
    And(::treesitter_types::Span),
    Instanceof(::treesitter_types::Span),
    Or(::treesitter_types::Span),
    Xor(::treesitter_types::Span),
    Pipe(::treesitter_types::Span),
    PipeGt(::treesitter_types::Span),
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
            "!==" => Ok(Self::BangEqEq(::treesitter_types::Span::from(node))),
            "%" => Ok(Self::Percent(::treesitter_types::Span::from(node))),
            "&" => Ok(Self::Amp(::treesitter_types::Span::from(node))),
            "&&" => Ok(Self::AmpAmp(::treesitter_types::Span::from(node))),
            "*" => Ok(Self::Star(::treesitter_types::Span::from(node))),
            "**" => Ok(Self::StarStar(::treesitter_types::Span::from(node))),
            "+" => Ok(Self::Plus(::treesitter_types::Span::from(node))),
            "-" => Ok(Self::Minus(::treesitter_types::Span::from(node))),
            "." => Ok(Self::Dot(::treesitter_types::Span::from(node))),
            "/" => Ok(Self::Slash(::treesitter_types::Span::from(node))),
            "<" => Ok(Self::Lt(::treesitter_types::Span::from(node))),
            "<<" => Ok(Self::Shl(::treesitter_types::Span::from(node))),
            "<=" => Ok(Self::LtEq(::treesitter_types::Span::from(node))),
            "<=>" => Ok(Self::LtEqGt(::treesitter_types::Span::from(node))),
            "<>" => Ok(Self::LtGt(::treesitter_types::Span::from(node))),
            "==" => Ok(Self::EqEq(::treesitter_types::Span::from(node))),
            "===" => Ok(Self::EqEqEq(::treesitter_types::Span::from(node))),
            ">" => Ok(Self::Gt(::treesitter_types::Span::from(node))),
            ">=" => Ok(Self::GtEq(::treesitter_types::Span::from(node))),
            ">>" => Ok(Self::Shr(::treesitter_types::Span::from(node))),
            "??" => Ok(Self::QuestionQuestion(::treesitter_types::Span::from(node))),
            "^" => Ok(Self::Caret(::treesitter_types::Span::from(node))),
            "and" => Ok(Self::And(::treesitter_types::Span::from(node))),
            "instanceof" => Ok(Self::Instanceof(::treesitter_types::Span::from(node))),
            "or" => Ok(Self::Or(::treesitter_types::Span::from(node))),
            "xor" => Ok(Self::Xor(::treesitter_types::Span::from(node))),
            "|" => Ok(Self::Pipe(::treesitter_types::Span::from(node))),
            "|>" => Ok(Self::PipeGt(::treesitter_types::Span::from(node))),
            "||" => Ok(Self::PipePipe(::treesitter_types::Span::from(node))),
            other => Err(::treesitter_types::ParseError::unexpected_kind(other, node)),
        }
    }
}
impl ::treesitter_types::Spanned for BinaryExpressionOperator {
    fn span(&self) -> ::treesitter_types::Span {
        match self {
            Self::NotEq(span) => *span,
            Self::BangEqEq(span) => *span,
            Self::Percent(span) => *span,
            Self::Amp(span) => *span,
            Self::AmpAmp(span) => *span,
            Self::Star(span) => *span,
            Self::StarStar(span) => *span,
            Self::Plus(span) => *span,
            Self::Minus(span) => *span,
            Self::Dot(span) => *span,
            Self::Slash(span) => *span,
            Self::Lt(span) => *span,
            Self::Shl(span) => *span,
            Self::LtEq(span) => *span,
            Self::LtEqGt(span) => *span,
            Self::LtGt(span) => *span,
            Self::EqEq(span) => *span,
            Self::EqEqEq(span) => *span,
            Self::Gt(span) => *span,
            Self::GtEq(span) => *span,
            Self::Shr(span) => *span,
            Self::QuestionQuestion(span) => *span,
            Self::Caret(span) => *span,
            Self::And(span) => *span,
            Self::Instanceof(span) => *span,
            Self::Or(span) => *span,
            Self::Xor(span) => *span,
            Self::Pipe(span) => *span,
            Self::PipeGt(span) => *span,
            Self::PipePipe(span) => *span,
        }
    }
}
#[derive(Debug, Clone)]
pub enum BinaryExpressionRight<'tree> {
    DynamicVariableName(::std::boxed::Box<DynamicVariableName<'tree>>),
    Expression(::std::boxed::Box<Expression<'tree>>),
    MemberAccessExpression(::std::boxed::Box<MemberAccessExpression<'tree>>),
    Name(::std::boxed::Box<Name<'tree>>),
    NullsafeMemberAccessExpression(::std::boxed::Box<NullsafeMemberAccessExpression<'tree>>),
    ParenthesizedExpression(::std::boxed::Box<ParenthesizedExpression<'tree>>),
    QualifiedName(::std::boxed::Box<QualifiedName<'tree>>),
    RelativeName(::std::boxed::Box<RelativeName<'tree>>),
    ScopedPropertyAccessExpression(::std::boxed::Box<ScopedPropertyAccessExpression<'tree>>),
    SubscriptExpression(::std::boxed::Box<SubscriptExpression<'tree>>),
    VariableName(::std::boxed::Box<VariableName<'tree>>),
}
impl<'tree> ::treesitter_types::FromNode<'tree> for BinaryExpressionRight<'tree> {
    #[allow(clippy::collapsible_else_if)]
    fn from_node(
        node: ::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        match node.kind() {
            "dynamic_variable_name" => Ok(Self::DynamicVariableName(::std::boxed::Box::new(
                <DynamicVariableName as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "member_access_expression" => Ok(Self::MemberAccessExpression(::std::boxed::Box::new(
                <MemberAccessExpression as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "name" => Ok(Self::Name(::std::boxed::Box::new(
                <Name as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "nullsafe_member_access_expression" => Ok(Self::NullsafeMemberAccessExpression(
                ::std::boxed::Box::new(
                    <NullsafeMemberAccessExpression as ::treesitter_types::FromNode>::from_node(
                        node, src,
                    )?,
                ),
            )),
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
            "relative_name" => Ok(Self::RelativeName(::std::boxed::Box::new(
                <RelativeName as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "scoped_property_access_expression" => Ok(Self::ScopedPropertyAccessExpression(
                ::std::boxed::Box::new(
                    <ScopedPropertyAccessExpression as ::treesitter_types::FromNode>::from_node(
                        node, src,
                    )?,
                ),
            )),
            "subscript_expression" => Ok(Self::SubscriptExpression(::std::boxed::Box::new(
                <SubscriptExpression as ::treesitter_types::FromNode>::from_node(node, src)?,
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
            Self::DynamicVariableName(inner) => inner.span(),
            Self::Expression(inner) => inner.span(),
            Self::MemberAccessExpression(inner) => inner.span(),
            Self::Name(inner) => inner.span(),
            Self::NullsafeMemberAccessExpression(inner) => inner.span(),
            Self::ParenthesizedExpression(inner) => inner.span(),
            Self::QualifiedName(inner) => inner.span(),
            Self::RelativeName(inner) => inner.span(),
            Self::ScopedPropertyAccessExpression(inner) => inner.span(),
            Self::SubscriptExpression(inner) => inner.span(),
            Self::VariableName(inner) => inner.span(),
        }
    }
}
#[derive(Debug, Clone)]
pub enum ByRefChildren<'tree> {
    CastExpression(::std::boxed::Box<CastExpression<'tree>>),
    DynamicVariableName(::std::boxed::Box<DynamicVariableName<'tree>>),
    FunctionCallExpression(::std::boxed::Box<FunctionCallExpression<'tree>>),
    MemberAccessExpression(::std::boxed::Box<MemberAccessExpression<'tree>>),
    MemberCallExpression(::std::boxed::Box<MemberCallExpression<'tree>>),
    NullsafeMemberAccessExpression(::std::boxed::Box<NullsafeMemberAccessExpression<'tree>>),
    NullsafeMemberCallExpression(::std::boxed::Box<NullsafeMemberCallExpression<'tree>>),
    ScopedCallExpression(::std::boxed::Box<ScopedCallExpression<'tree>>),
    ScopedPropertyAccessExpression(::std::boxed::Box<ScopedPropertyAccessExpression<'tree>>),
    SubscriptExpression(::std::boxed::Box<SubscriptExpression<'tree>>),
    VariableName(::std::boxed::Box<VariableName<'tree>>),
}
impl<'tree> ::treesitter_types::FromNode<'tree> for ByRefChildren<'tree> {
    #[allow(clippy::collapsible_else_if)]
    fn from_node(
        node: ::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        match node.kind() {
            "cast_expression" => Ok(Self::CastExpression(::std::boxed::Box::new(
                <CastExpression as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "dynamic_variable_name" => Ok(Self::DynamicVariableName(::std::boxed::Box::new(
                <DynamicVariableName as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "function_call_expression" => Ok(Self::FunctionCallExpression(::std::boxed::Box::new(
                <FunctionCallExpression as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "member_access_expression" => Ok(Self::MemberAccessExpression(::std::boxed::Box::new(
                <MemberAccessExpression as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "member_call_expression" => Ok(Self::MemberCallExpression(::std::boxed::Box::new(
                <MemberCallExpression as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "nullsafe_member_access_expression" => Ok(Self::NullsafeMemberAccessExpression(
                ::std::boxed::Box::new(
                    <NullsafeMemberAccessExpression as ::treesitter_types::FromNode>::from_node(
                        node, src,
                    )?,
                ),
            )),
            "nullsafe_member_call_expression" => {
                Ok(Self::NullsafeMemberCallExpression(::std::boxed::Box::new(
                    <NullsafeMemberCallExpression as ::treesitter_types::FromNode>::from_node(
                        node, src,
                    )?,
                )))
            }
            "scoped_call_expression" => Ok(Self::ScopedCallExpression(::std::boxed::Box::new(
                <ScopedCallExpression as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "scoped_property_access_expression" => Ok(Self::ScopedPropertyAccessExpression(
                ::std::boxed::Box::new(
                    <ScopedPropertyAccessExpression as ::treesitter_types::FromNode>::from_node(
                        node, src,
                    )?,
                ),
            )),
            "subscript_expression" => Ok(Self::SubscriptExpression(::std::boxed::Box::new(
                <SubscriptExpression as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "variable_name" => Ok(Self::VariableName(::std::boxed::Box::new(
                <VariableName as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            other => Err(::treesitter_types::ParseError::unexpected_kind(other, node)),
        }
    }
}
impl ::treesitter_types::Spanned for ByRefChildren<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        match self {
            Self::CastExpression(inner) => inner.span(),
            Self::DynamicVariableName(inner) => inner.span(),
            Self::FunctionCallExpression(inner) => inner.span(),
            Self::MemberAccessExpression(inner) => inner.span(),
            Self::MemberCallExpression(inner) => inner.span(),
            Self::NullsafeMemberAccessExpression(inner) => inner.span(),
            Self::NullsafeMemberCallExpression(inner) => inner.span(),
            Self::ScopedCallExpression(inner) => inner.span(),
            Self::ScopedPropertyAccessExpression(inner) => inner.span(),
            Self::SubscriptExpression(inner) => inner.span(),
            Self::VariableName(inner) => inner.span(),
        }
    }
}
#[derive(Debug, Clone)]
pub enum CastExpressionValue<'tree> {
    CloneExpression(::std::boxed::Box<CloneExpression<'tree>>),
    ErrorSuppressionExpression(::std::boxed::Box<ErrorSuppressionExpression<'tree>>),
    IncludeExpression(::std::boxed::Box<IncludeExpression<'tree>>),
    IncludeOnceExpression(::std::boxed::Box<IncludeOnceExpression<'tree>>),
    PrimaryExpression(::std::boxed::Box<PrimaryExpression<'tree>>),
    UnaryOpExpression(::std::boxed::Box<UnaryOpExpression<'tree>>),
}
impl<'tree> ::treesitter_types::FromNode<'tree> for CastExpressionValue<'tree> {
    #[allow(clippy::collapsible_else_if)]
    fn from_node(
        node: ::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        match node.kind() {
            "clone_expression" => Ok(Self::CloneExpression(::std::boxed::Box::new(
                <CloneExpression as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "error_suppression_expression" => {
                Ok(Self::ErrorSuppressionExpression(::std::boxed::Box::new(
                    <ErrorSuppressionExpression as ::treesitter_types::FromNode>::from_node(
                        node, src,
                    )?,
                )))
            }
            "include_expression" => Ok(Self::IncludeExpression(::std::boxed::Box::new(
                <IncludeExpression as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "include_once_expression" => Ok(Self::IncludeOnceExpression(::std::boxed::Box::new(
                <IncludeOnceExpression as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "unary_op_expression" => Ok(Self::UnaryOpExpression(::std::boxed::Box::new(
                <UnaryOpExpression as ::treesitter_types::FromNode>::from_node(node, src)?,
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
impl ::treesitter_types::Spanned for CastExpressionValue<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        match self {
            Self::CloneExpression(inner) => inner.span(),
            Self::ErrorSuppressionExpression(inner) => inner.span(),
            Self::IncludeExpression(inner) => inner.span(),
            Self::IncludeOnceExpression(inner) => inner.span(),
            Self::PrimaryExpression(inner) => inner.span(),
            Self::UnaryOpExpression(inner) => inner.span(),
        }
    }
}
#[derive(Debug, Clone)]
pub enum ClassConstantAccessExpressionChildren<'tree> {
    ArrayCreationExpression(::std::boxed::Box<ArrayCreationExpression<'tree>>),
    CastExpression(::std::boxed::Box<CastExpression<'tree>>),
    ClassConstantAccessExpression(::std::boxed::Box<ClassConstantAccessExpression<'tree>>),
    DynamicVariableName(::std::boxed::Box<DynamicVariableName<'tree>>),
    EncapsedString(::std::boxed::Box<EncapsedString<'tree>>),
    FunctionCallExpression(::std::boxed::Box<FunctionCallExpression<'tree>>),
    Heredoc(::std::boxed::Box<Heredoc<'tree>>),
    MemberAccessExpression(::std::boxed::Box<MemberAccessExpression<'tree>>),
    MemberCallExpression(::std::boxed::Box<MemberCallExpression<'tree>>),
    Name(::std::boxed::Box<Name<'tree>>),
    Nowdoc(::std::boxed::Box<Nowdoc<'tree>>),
    NullsafeMemberAccessExpression(::std::boxed::Box<NullsafeMemberAccessExpression<'tree>>),
    NullsafeMemberCallExpression(::std::boxed::Box<NullsafeMemberCallExpression<'tree>>),
    ObjectCreationExpression(::std::boxed::Box<ObjectCreationExpression<'tree>>),
    ParenthesizedExpression(::std::boxed::Box<ParenthesizedExpression<'tree>>),
    QualifiedName(::std::boxed::Box<QualifiedName<'tree>>),
    RelativeName(::std::boxed::Box<RelativeName<'tree>>),
    RelativeScope(::std::boxed::Box<RelativeScope<'tree>>),
    ScopedCallExpression(::std::boxed::Box<ScopedCallExpression<'tree>>),
    ScopedPropertyAccessExpression(::std::boxed::Box<ScopedPropertyAccessExpression<'tree>>),
    String(::std::boxed::Box<String<'tree>>),
    SubscriptExpression(::std::boxed::Box<SubscriptExpression<'tree>>),
    VariableName(::std::boxed::Box<VariableName<'tree>>),
}
impl<'tree> ::treesitter_types::FromNode<'tree> for ClassConstantAccessExpressionChildren<'tree> {
    #[allow(clippy::collapsible_else_if)]
    fn from_node(
        node: ::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        match node.kind() {
            "array_creation_expression" => {
                Ok(Self::ArrayCreationExpression(::std::boxed::Box::new(
                    <ArrayCreationExpression as ::treesitter_types::FromNode>::from_node(
                        node, src,
                    )?,
                )))
            }
            "cast_expression" => Ok(Self::CastExpression(::std::boxed::Box::new(
                <CastExpression as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "class_constant_access_expression" => {
                Ok(Self::ClassConstantAccessExpression(::std::boxed::Box::new(
                    <ClassConstantAccessExpression as ::treesitter_types::FromNode>::from_node(
                        node, src,
                    )?,
                )))
            }
            "dynamic_variable_name" => Ok(Self::DynamicVariableName(::std::boxed::Box::new(
                <DynamicVariableName as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "encapsed_string" => Ok(Self::EncapsedString(::std::boxed::Box::new(
                <EncapsedString as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "function_call_expression" => Ok(Self::FunctionCallExpression(::std::boxed::Box::new(
                <FunctionCallExpression as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "heredoc" => Ok(Self::Heredoc(::std::boxed::Box::new(
                <Heredoc as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "member_access_expression" => Ok(Self::MemberAccessExpression(::std::boxed::Box::new(
                <MemberAccessExpression as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "member_call_expression" => Ok(Self::MemberCallExpression(::std::boxed::Box::new(
                <MemberCallExpression as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "name" => Ok(Self::Name(::std::boxed::Box::new(
                <Name as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "nowdoc" => Ok(Self::Nowdoc(::std::boxed::Box::new(
                <Nowdoc as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "nullsafe_member_access_expression" => Ok(Self::NullsafeMemberAccessExpression(
                ::std::boxed::Box::new(
                    <NullsafeMemberAccessExpression as ::treesitter_types::FromNode>::from_node(
                        node, src,
                    )?,
                ),
            )),
            "nullsafe_member_call_expression" => {
                Ok(Self::NullsafeMemberCallExpression(::std::boxed::Box::new(
                    <NullsafeMemberCallExpression as ::treesitter_types::FromNode>::from_node(
                        node, src,
                    )?,
                )))
            }
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
            "qualified_name" => Ok(Self::QualifiedName(::std::boxed::Box::new(
                <QualifiedName as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "relative_name" => Ok(Self::RelativeName(::std::boxed::Box::new(
                <RelativeName as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "relative_scope" => Ok(Self::RelativeScope(::std::boxed::Box::new(
                <RelativeScope as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "scoped_call_expression" => Ok(Self::ScopedCallExpression(::std::boxed::Box::new(
                <ScopedCallExpression as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "scoped_property_access_expression" => Ok(Self::ScopedPropertyAccessExpression(
                ::std::boxed::Box::new(
                    <ScopedPropertyAccessExpression as ::treesitter_types::FromNode>::from_node(
                        node, src,
                    )?,
                ),
            )),
            "string" => Ok(Self::String(::std::boxed::Box::new(
                <String as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "subscript_expression" => Ok(Self::SubscriptExpression(::std::boxed::Box::new(
                <SubscriptExpression as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "variable_name" => Ok(Self::VariableName(::std::boxed::Box::new(
                <VariableName as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            other => Err(::treesitter_types::ParseError::unexpected_kind(other, node)),
        }
    }
}
impl ::treesitter_types::Spanned for ClassConstantAccessExpressionChildren<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        match self {
            Self::ArrayCreationExpression(inner) => inner.span(),
            Self::CastExpression(inner) => inner.span(),
            Self::ClassConstantAccessExpression(inner) => inner.span(),
            Self::DynamicVariableName(inner) => inner.span(),
            Self::EncapsedString(inner) => inner.span(),
            Self::FunctionCallExpression(inner) => inner.span(),
            Self::Heredoc(inner) => inner.span(),
            Self::MemberAccessExpression(inner) => inner.span(),
            Self::MemberCallExpression(inner) => inner.span(),
            Self::Name(inner) => inner.span(),
            Self::Nowdoc(inner) => inner.span(),
            Self::NullsafeMemberAccessExpression(inner) => inner.span(),
            Self::NullsafeMemberCallExpression(inner) => inner.span(),
            Self::ObjectCreationExpression(inner) => inner.span(),
            Self::ParenthesizedExpression(inner) => inner.span(),
            Self::QualifiedName(inner) => inner.span(),
            Self::RelativeName(inner) => inner.span(),
            Self::RelativeScope(inner) => inner.span(),
            Self::ScopedCallExpression(inner) => inner.span(),
            Self::ScopedPropertyAccessExpression(inner) => inner.span(),
            Self::String(inner) => inner.span(),
            Self::SubscriptExpression(inner) => inner.span(),
            Self::VariableName(inner) => inner.span(),
        }
    }
}
#[derive(Debug, Clone)]
pub enum ClassDeclarationChildren<'tree> {
    AbstractModifier(::std::boxed::Box<AbstractModifier<'tree>>),
    BaseClause(::std::boxed::Box<BaseClause<'tree>>),
    ClassInterfaceClause(::std::boxed::Box<ClassInterfaceClause<'tree>>),
    FinalModifier(::std::boxed::Box<FinalModifier<'tree>>),
    ReadonlyModifier(::std::boxed::Box<ReadonlyModifier<'tree>>),
    StaticModifier(::std::boxed::Box<StaticModifier<'tree>>),
    VarModifier(::std::boxed::Box<VarModifier<'tree>>),
    VisibilityModifier(::std::boxed::Box<VisibilityModifier<'tree>>),
}
impl<'tree> ::treesitter_types::FromNode<'tree> for ClassDeclarationChildren<'tree> {
    #[allow(clippy::collapsible_else_if)]
    fn from_node(
        node: ::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        match node.kind() {
            "abstract_modifier" => Ok(Self::AbstractModifier(::std::boxed::Box::new(
                <AbstractModifier as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "base_clause" => Ok(Self::BaseClause(::std::boxed::Box::new(
                <BaseClause as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "class_interface_clause" => Ok(Self::ClassInterfaceClause(::std::boxed::Box::new(
                <ClassInterfaceClause as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "final_modifier" => Ok(Self::FinalModifier(::std::boxed::Box::new(
                <FinalModifier as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "readonly_modifier" => Ok(Self::ReadonlyModifier(::std::boxed::Box::new(
                <ReadonlyModifier as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "static_modifier" => Ok(Self::StaticModifier(::std::boxed::Box::new(
                <StaticModifier as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "var_modifier" => Ok(Self::VarModifier(::std::boxed::Box::new(
                <VarModifier as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "visibility_modifier" => Ok(Self::VisibilityModifier(::std::boxed::Box::new(
                <VisibilityModifier as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            other => Err(::treesitter_types::ParseError::unexpected_kind(other, node)),
        }
    }
}
impl ::treesitter_types::Spanned for ClassDeclarationChildren<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        match self {
            Self::AbstractModifier(inner) => inner.span(),
            Self::BaseClause(inner) => inner.span(),
            Self::ClassInterfaceClause(inner) => inner.span(),
            Self::FinalModifier(inner) => inner.span(),
            Self::ReadonlyModifier(inner) => inner.span(),
            Self::StaticModifier(inner) => inner.span(),
            Self::VarModifier(inner) => inner.span(),
            Self::VisibilityModifier(inner) => inner.span(),
        }
    }
}
#[derive(Debug, Clone)]
pub enum ClassInterfaceClauseChildren<'tree> {
    Name(::std::boxed::Box<Name<'tree>>),
    QualifiedName(::std::boxed::Box<QualifiedName<'tree>>),
    RelativeName(::std::boxed::Box<RelativeName<'tree>>),
}
impl<'tree> ::treesitter_types::FromNode<'tree> for ClassInterfaceClauseChildren<'tree> {
    #[allow(clippy::collapsible_else_if)]
    fn from_node(
        node: ::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        match node.kind() {
            "name" => Ok(Self::Name(::std::boxed::Box::new(
                <Name as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "qualified_name" => Ok(Self::QualifiedName(::std::boxed::Box::new(
                <QualifiedName as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "relative_name" => Ok(Self::RelativeName(::std::boxed::Box::new(
                <RelativeName as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            other => Err(::treesitter_types::ParseError::unexpected_kind(other, node)),
        }
    }
}
impl ::treesitter_types::Spanned for ClassInterfaceClauseChildren<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        match self {
            Self::Name(inner) => inner.span(),
            Self::QualifiedName(inner) => inner.span(),
            Self::RelativeName(inner) => inner.span(),
        }
    }
}
#[derive(Debug, Clone)]
pub enum ConstDeclarationChildren<'tree> {
    AbstractModifier(::std::boxed::Box<AbstractModifier<'tree>>),
    ConstElement(::std::boxed::Box<ConstElement<'tree>>),
    FinalModifier(::std::boxed::Box<FinalModifier<'tree>>),
    ReadonlyModifier(::std::boxed::Box<ReadonlyModifier<'tree>>),
    StaticModifier(::std::boxed::Box<StaticModifier<'tree>>),
    VarModifier(::std::boxed::Box<VarModifier<'tree>>),
    VisibilityModifier(::std::boxed::Box<VisibilityModifier<'tree>>),
}
impl<'tree> ::treesitter_types::FromNode<'tree> for ConstDeclarationChildren<'tree> {
    #[allow(clippy::collapsible_else_if)]
    fn from_node(
        node: ::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        match node.kind() {
            "abstract_modifier" => Ok(Self::AbstractModifier(::std::boxed::Box::new(
                <AbstractModifier as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "const_element" => Ok(Self::ConstElement(::std::boxed::Box::new(
                <ConstElement as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "final_modifier" => Ok(Self::FinalModifier(::std::boxed::Box::new(
                <FinalModifier as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "readonly_modifier" => Ok(Self::ReadonlyModifier(::std::boxed::Box::new(
                <ReadonlyModifier as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "static_modifier" => Ok(Self::StaticModifier(::std::boxed::Box::new(
                <StaticModifier as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "var_modifier" => Ok(Self::VarModifier(::std::boxed::Box::new(
                <VarModifier as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "visibility_modifier" => Ok(Self::VisibilityModifier(::std::boxed::Box::new(
                <VisibilityModifier as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            other => Err(::treesitter_types::ParseError::unexpected_kind(other, node)),
        }
    }
}
impl ::treesitter_types::Spanned for ConstDeclarationChildren<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        match self {
            Self::AbstractModifier(inner) => inner.span(),
            Self::ConstElement(inner) => inner.span(),
            Self::FinalModifier(inner) => inner.span(),
            Self::ReadonlyModifier(inner) => inner.span(),
            Self::StaticModifier(inner) => inner.span(),
            Self::VarModifier(inner) => inner.span(),
            Self::VisibilityModifier(inner) => inner.span(),
        }
    }
}
#[derive(Debug, Clone)]
pub enum ConstElementChildren<'tree> {
    Expression(::std::boxed::Box<Expression<'tree>>),
    Name(::std::boxed::Box<Name<'tree>>),
}
impl<'tree> ::treesitter_types::FromNode<'tree> for ConstElementChildren<'tree> {
    #[allow(clippy::collapsible_else_if)]
    fn from_node(
        node: ::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        match node.kind() {
            "name" => Ok(Self::Name(::std::boxed::Box::new(
                <Name as ::treesitter_types::FromNode>::from_node(node, src)?,
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
impl ::treesitter_types::Spanned for ConstElementChildren<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        match self {
            Self::Expression(inner) => inner.span(),
            Self::Name(inner) => inner.span(),
        }
    }
}
#[derive(Debug, Clone)]
pub enum DeclarationListChildren<'tree> {
    ConstDeclaration(::std::boxed::Box<ConstDeclaration<'tree>>),
    MethodDeclaration(::std::boxed::Box<MethodDeclaration<'tree>>),
    PropertyDeclaration(::std::boxed::Box<PropertyDeclaration<'tree>>),
    UseDeclaration(::std::boxed::Box<UseDeclaration<'tree>>),
}
impl<'tree> ::treesitter_types::FromNode<'tree> for DeclarationListChildren<'tree> {
    #[allow(clippy::collapsible_else_if)]
    fn from_node(
        node: ::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        match node.kind() {
            "const_declaration" => Ok(Self::ConstDeclaration(::std::boxed::Box::new(
                <ConstDeclaration as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "method_declaration" => Ok(Self::MethodDeclaration(::std::boxed::Box::new(
                <MethodDeclaration as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "property_declaration" => Ok(Self::PropertyDeclaration(::std::boxed::Box::new(
                <PropertyDeclaration as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "use_declaration" => Ok(Self::UseDeclaration(::std::boxed::Box::new(
                <UseDeclaration as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            other => Err(::treesitter_types::ParseError::unexpected_kind(other, node)),
        }
    }
}
impl ::treesitter_types::Spanned for DeclarationListChildren<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        match self {
            Self::ConstDeclaration(inner) => inner.span(),
            Self::MethodDeclaration(inner) => inner.span(),
            Self::PropertyDeclaration(inner) => inner.span(),
            Self::UseDeclaration(inner) => inner.span(),
        }
    }
}
#[derive(Debug, Clone)]
pub enum DeclareStatementChildren<'tree> {
    DeclareDirective(::std::boxed::Box<DeclareDirective<'tree>>),
    Statement(::std::boxed::Box<Statement<'tree>>),
}
impl<'tree> ::treesitter_types::FromNode<'tree> for DeclareStatementChildren<'tree> {
    #[allow(clippy::collapsible_else_if)]
    fn from_node(
        node: ::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        match node.kind() {
            "declare_directive" => Ok(Self::DeclareDirective(::std::boxed::Box::new(
                <DeclareDirective as ::treesitter_types::FromNode>::from_node(node, src)?,
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
impl ::treesitter_types::Spanned for DeclareStatementChildren<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        match self {
            Self::DeclareDirective(inner) => inner.span(),
            Self::Statement(inner) => inner.span(),
        }
    }
}
#[derive(Debug, Clone)]
pub enum DisjunctiveNormalFormTypeChildren<'tree> {
    IntersectionType(::std::boxed::Box<IntersectionType<'tree>>),
    NamedType(::std::boxed::Box<NamedType<'tree>>),
    OptionalType(::std::boxed::Box<OptionalType<'tree>>),
    PrimitiveType(::std::boxed::Box<PrimitiveType<'tree>>),
}
impl<'tree> ::treesitter_types::FromNode<'tree> for DisjunctiveNormalFormTypeChildren<'tree> {
    #[allow(clippy::collapsible_else_if)]
    fn from_node(
        node: ::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        match node.kind() {
            "intersection_type" => Ok(Self::IntersectionType(::std::boxed::Box::new(
                <IntersectionType as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "named_type" => Ok(Self::NamedType(::std::boxed::Box::new(
                <NamedType as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "optional_type" => Ok(Self::OptionalType(::std::boxed::Box::new(
                <OptionalType as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "primitive_type" => Ok(Self::PrimitiveType(::std::boxed::Box::new(
                <PrimitiveType as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            other => Err(::treesitter_types::ParseError::unexpected_kind(other, node)),
        }
    }
}
impl ::treesitter_types::Spanned for DisjunctiveNormalFormTypeChildren<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        match self {
            Self::IntersectionType(inner) => inner.span(),
            Self::NamedType(inner) => inner.span(),
            Self::OptionalType(inner) => inner.span(),
            Self::PrimitiveType(inner) => inner.span(),
        }
    }
}
#[derive(Debug, Clone)]
pub enum DynamicVariableNameChildren<'tree> {
    DynamicVariableName(::std::boxed::Box<DynamicVariableName<'tree>>),
    Expression(::std::boxed::Box<Expression<'tree>>),
    VariableName(::std::boxed::Box<VariableName<'tree>>),
}
impl<'tree> ::treesitter_types::FromNode<'tree> for DynamicVariableNameChildren<'tree> {
    #[allow(clippy::collapsible_else_if)]
    fn from_node(
        node: ::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        match node.kind() {
            "dynamic_variable_name" => Ok(Self::DynamicVariableName(::std::boxed::Box::new(
                <DynamicVariableName as ::treesitter_types::FromNode>::from_node(node, src)?,
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
impl ::treesitter_types::Spanned for DynamicVariableNameChildren<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        match self {
            Self::DynamicVariableName(inner) => inner.span(),
            Self::Expression(inner) => inner.span(),
            Self::VariableName(inner) => inner.span(),
        }
    }
}
#[derive(Debug, Clone)]
pub enum EchoStatementChildren<'tree> {
    Expression(::std::boxed::Box<Expression<'tree>>),
    SequenceExpression(::std::boxed::Box<SequenceExpression<'tree>>),
}
impl<'tree> ::treesitter_types::FromNode<'tree> for EchoStatementChildren<'tree> {
    #[allow(clippy::collapsible_else_if)]
    fn from_node(
        node: ::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        match node.kind() {
            "sequence_expression" => Ok(Self::SequenceExpression(::std::boxed::Box::new(
                <SequenceExpression as ::treesitter_types::FromNode>::from_node(node, src)?,
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
impl ::treesitter_types::Spanned for EchoStatementChildren<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        match self {
            Self::Expression(inner) => inner.span(),
            Self::SequenceExpression(inner) => inner.span(),
        }
    }
}
#[derive(Debug, Clone)]
pub enum ElseClauseBody<'tree> {
    ColonBlock(::std::boxed::Box<ColonBlock<'tree>>),
    Statement(::std::boxed::Box<Statement<'tree>>),
}
impl<'tree> ::treesitter_types::FromNode<'tree> for ElseClauseBody<'tree> {
    #[allow(clippy::collapsible_else_if)]
    fn from_node(
        node: ::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        match node.kind() {
            "colon_block" => Ok(Self::ColonBlock(::std::boxed::Box::new(
                <ColonBlock as ::treesitter_types::FromNode>::from_node(node, src)?,
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
impl ::treesitter_types::Spanned for ElseClauseBody<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        match self {
            Self::ColonBlock(inner) => inner.span(),
            Self::Statement(inner) => inner.span(),
        }
    }
}
#[derive(Debug, Clone)]
pub enum ElseIfClauseBody<'tree> {
    ColonBlock(::std::boxed::Box<ColonBlock<'tree>>),
    Statement(::std::boxed::Box<Statement<'tree>>),
}
impl<'tree> ::treesitter_types::FromNode<'tree> for ElseIfClauseBody<'tree> {
    #[allow(clippy::collapsible_else_if)]
    fn from_node(
        node: ::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        match node.kind() {
            "colon_block" => Ok(Self::ColonBlock(::std::boxed::Box::new(
                <ColonBlock as ::treesitter_types::FromNode>::from_node(node, src)?,
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
impl ::treesitter_types::Spanned for ElseIfClauseBody<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        match self {
            Self::ColonBlock(inner) => inner.span(),
            Self::Statement(inner) => inner.span(),
        }
    }
}
#[derive(Debug, Clone)]
pub enum EncapsedStringChildren<'tree> {
    DynamicVariableName(::std::boxed::Box<DynamicVariableName<'tree>>),
    EscapeSequence(::std::boxed::Box<EscapeSequence<'tree>>),
    Expression(::std::boxed::Box<Expression<'tree>>),
    MemberAccessExpression(::std::boxed::Box<MemberAccessExpression<'tree>>),
    StringContent(::std::boxed::Box<StringContent<'tree>>),
    SubscriptExpression(::std::boxed::Box<SubscriptExpression<'tree>>),
    VariableName(::std::boxed::Box<VariableName<'tree>>),
}
impl<'tree> ::treesitter_types::FromNode<'tree> for EncapsedStringChildren<'tree> {
    #[allow(clippy::collapsible_else_if)]
    fn from_node(
        node: ::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        match node.kind() {
            "dynamic_variable_name" => Ok(Self::DynamicVariableName(::std::boxed::Box::new(
                <DynamicVariableName as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "escape_sequence" => Ok(Self::EscapeSequence(::std::boxed::Box::new(
                <EscapeSequence as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "member_access_expression" => Ok(Self::MemberAccessExpression(::std::boxed::Box::new(
                <MemberAccessExpression as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "string_content" => Ok(Self::StringContent(::std::boxed::Box::new(
                <StringContent as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "subscript_expression" => Ok(Self::SubscriptExpression(::std::boxed::Box::new(
                <SubscriptExpression as ::treesitter_types::FromNode>::from_node(node, src)?,
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
impl ::treesitter_types::Spanned for EncapsedStringChildren<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        match self {
            Self::DynamicVariableName(inner) => inner.span(),
            Self::EscapeSequence(inner) => inner.span(),
            Self::Expression(inner) => inner.span(),
            Self::MemberAccessExpression(inner) => inner.span(),
            Self::StringContent(inner) => inner.span(),
            Self::SubscriptExpression(inner) => inner.span(),
            Self::VariableName(inner) => inner.span(),
        }
    }
}
#[derive(Debug, Clone)]
pub enum EnumDeclarationChildren<'tree> {
    ClassInterfaceClause(::std::boxed::Box<ClassInterfaceClause<'tree>>),
    PrimitiveType(::std::boxed::Box<PrimitiveType<'tree>>),
}
impl<'tree> ::treesitter_types::FromNode<'tree> for EnumDeclarationChildren<'tree> {
    #[allow(clippy::collapsible_else_if)]
    fn from_node(
        node: ::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        match node.kind() {
            "class_interface_clause" => Ok(Self::ClassInterfaceClause(::std::boxed::Box::new(
                <ClassInterfaceClause as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "primitive_type" => Ok(Self::PrimitiveType(::std::boxed::Box::new(
                <PrimitiveType as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            other => Err(::treesitter_types::ParseError::unexpected_kind(other, node)),
        }
    }
}
impl ::treesitter_types::Spanned for EnumDeclarationChildren<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        match self {
            Self::ClassInterfaceClause(inner) => inner.span(),
            Self::PrimitiveType(inner) => inner.span(),
        }
    }
}
#[derive(Debug, Clone)]
pub enum EnumDeclarationListChildren<'tree> {
    ConstDeclaration(::std::boxed::Box<ConstDeclaration<'tree>>),
    EnumCase(::std::boxed::Box<EnumCase<'tree>>),
    MethodDeclaration(::std::boxed::Box<MethodDeclaration<'tree>>),
    UseDeclaration(::std::boxed::Box<UseDeclaration<'tree>>),
}
impl<'tree> ::treesitter_types::FromNode<'tree> for EnumDeclarationListChildren<'tree> {
    #[allow(clippy::collapsible_else_if)]
    fn from_node(
        node: ::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        match node.kind() {
            "const_declaration" => Ok(Self::ConstDeclaration(::std::boxed::Box::new(
                <ConstDeclaration as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "enum_case" => Ok(Self::EnumCase(::std::boxed::Box::new(
                <EnumCase as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "method_declaration" => Ok(Self::MethodDeclaration(::std::boxed::Box::new(
                <MethodDeclaration as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "use_declaration" => Ok(Self::UseDeclaration(::std::boxed::Box::new(
                <UseDeclaration as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            other => Err(::treesitter_types::ParseError::unexpected_kind(other, node)),
        }
    }
}
impl ::treesitter_types::Spanned for EnumDeclarationListChildren<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        match self {
            Self::ConstDeclaration(inner) => inner.span(),
            Self::EnumCase(inner) => inner.span(),
            Self::MethodDeclaration(inner) => inner.span(),
            Self::UseDeclaration(inner) => inner.span(),
        }
    }
}
#[derive(Debug, Clone)]
pub enum ForStatementCondition<'tree> {
    Expression(::std::boxed::Box<Expression<'tree>>),
    SequenceExpression(::std::boxed::Box<SequenceExpression<'tree>>),
}
impl<'tree> ::treesitter_types::FromNode<'tree> for ForStatementCondition<'tree> {
    #[allow(clippy::collapsible_else_if)]
    fn from_node(
        node: ::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        match node.kind() {
            "sequence_expression" => Ok(Self::SequenceExpression(::std::boxed::Box::new(
                <SequenceExpression as ::treesitter_types::FromNode>::from_node(node, src)?,
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
            Self::Expression(inner) => inner.span(),
            Self::SequenceExpression(inner) => inner.span(),
        }
    }
}
#[derive(Debug, Clone)]
pub enum ForStatementInitialize<'tree> {
    Expression(::std::boxed::Box<Expression<'tree>>),
    SequenceExpression(::std::boxed::Box<SequenceExpression<'tree>>),
}
impl<'tree> ::treesitter_types::FromNode<'tree> for ForStatementInitialize<'tree> {
    #[allow(clippy::collapsible_else_if)]
    fn from_node(
        node: ::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        match node.kind() {
            "sequence_expression" => Ok(Self::SequenceExpression(::std::boxed::Box::new(
                <SequenceExpression as ::treesitter_types::FromNode>::from_node(node, src)?,
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
impl ::treesitter_types::Spanned for ForStatementInitialize<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        match self {
            Self::Expression(inner) => inner.span(),
            Self::SequenceExpression(inner) => inner.span(),
        }
    }
}
#[derive(Debug, Clone)]
pub enum ForStatementUpdate<'tree> {
    Expression(::std::boxed::Box<Expression<'tree>>),
    SequenceExpression(::std::boxed::Box<SequenceExpression<'tree>>),
}
impl<'tree> ::treesitter_types::FromNode<'tree> for ForStatementUpdate<'tree> {
    #[allow(clippy::collapsible_else_if)]
    fn from_node(
        node: ::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        match node.kind() {
            "sequence_expression" => Ok(Self::SequenceExpression(::std::boxed::Box::new(
                <SequenceExpression as ::treesitter_types::FromNode>::from_node(node, src)?,
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
            Self::Expression(inner) => inner.span(),
            Self::SequenceExpression(inner) => inner.span(),
        }
    }
}
#[derive(Debug, Clone)]
pub enum ForeachStatementBody<'tree> {
    ColonBlock(::std::boxed::Box<ColonBlock<'tree>>),
    Statement(::std::boxed::Box<Statement<'tree>>),
}
impl<'tree> ::treesitter_types::FromNode<'tree> for ForeachStatementBody<'tree> {
    #[allow(clippy::collapsible_else_if)]
    fn from_node(
        node: ::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        match node.kind() {
            "colon_block" => Ok(Self::ColonBlock(::std::boxed::Box::new(
                <ColonBlock as ::treesitter_types::FromNode>::from_node(node, src)?,
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
impl ::treesitter_types::Spanned for ForeachStatementBody<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        match self {
            Self::ColonBlock(inner) => inner.span(),
            Self::Statement(inner) => inner.span(),
        }
    }
}
#[derive(Debug, Clone)]
pub enum ForeachStatementChildren<'tree> {
    ByRef(::std::boxed::Box<ByRef<'tree>>),
    Expression(::std::boxed::Box<Expression<'tree>>),
    ListLiteral(::std::boxed::Box<ListLiteral<'tree>>),
    Pair(::std::boxed::Box<Pair<'tree>>),
}
impl<'tree> ::treesitter_types::FromNode<'tree> for ForeachStatementChildren<'tree> {
    #[allow(clippy::collapsible_else_if)]
    fn from_node(
        node: ::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        match node.kind() {
            "by_ref" => Ok(Self::ByRef(::std::boxed::Box::new(
                <ByRef as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "list_literal" => Ok(Self::ListLiteral(::std::boxed::Box::new(
                <ListLiteral as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "pair" => Ok(Self::Pair(::std::boxed::Box::new(
                <Pair as ::treesitter_types::FromNode>::from_node(node, src)?,
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
impl ::treesitter_types::Spanned for ForeachStatementChildren<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        match self {
            Self::ByRef(inner) => inner.span(),
            Self::Expression(inner) => inner.span(),
            Self::ListLiteral(inner) => inner.span(),
            Self::Pair(inner) => inner.span(),
        }
    }
}
#[derive(Debug, Clone)]
pub enum FormalParametersChildren<'tree> {
    PropertyPromotionParameter(::std::boxed::Box<PropertyPromotionParameter<'tree>>),
    SimpleParameter(::std::boxed::Box<SimpleParameter<'tree>>),
    VariadicParameter(::std::boxed::Box<VariadicParameter<'tree>>),
}
impl<'tree> ::treesitter_types::FromNode<'tree> for FormalParametersChildren<'tree> {
    #[allow(clippy::collapsible_else_if)]
    fn from_node(
        node: ::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        match node.kind() {
            "property_promotion_parameter" => {
                Ok(Self::PropertyPromotionParameter(::std::boxed::Box::new(
                    <PropertyPromotionParameter as ::treesitter_types::FromNode>::from_node(
                        node, src,
                    )?,
                )))
            }
            "simple_parameter" => Ok(Self::SimpleParameter(::std::boxed::Box::new(
                <SimpleParameter as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "variadic_parameter" => Ok(Self::VariadicParameter(::std::boxed::Box::new(
                <VariadicParameter as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            other => Err(::treesitter_types::ParseError::unexpected_kind(other, node)),
        }
    }
}
impl ::treesitter_types::Spanned for FormalParametersChildren<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        match self {
            Self::PropertyPromotionParameter(inner) => inner.span(),
            Self::SimpleParameter(inner) => inner.span(),
            Self::VariadicParameter(inner) => inner.span(),
        }
    }
}
#[derive(Debug, Clone)]
pub enum FunctionCallExpressionFunction<'tree> {
    ArrayCreationExpression(::std::boxed::Box<ArrayCreationExpression<'tree>>),
    DynamicVariableName(::std::boxed::Box<DynamicVariableName<'tree>>),
    EncapsedString(::std::boxed::Box<EncapsedString<'tree>>),
    FunctionCallExpression(::std::boxed::Box<FunctionCallExpression<'tree>>),
    Heredoc(::std::boxed::Box<Heredoc<'tree>>),
    MemberCallExpression(::std::boxed::Box<MemberCallExpression<'tree>>),
    Name(::std::boxed::Box<Name<'tree>>),
    Nowdoc(::std::boxed::Box<Nowdoc<'tree>>),
    NullsafeMemberCallExpression(::std::boxed::Box<NullsafeMemberCallExpression<'tree>>),
    ObjectCreationExpression(::std::boxed::Box<ObjectCreationExpression<'tree>>),
    ParenthesizedExpression(::std::boxed::Box<ParenthesizedExpression<'tree>>),
    QualifiedName(::std::boxed::Box<QualifiedName<'tree>>),
    RelativeName(::std::boxed::Box<RelativeName<'tree>>),
    ScopedCallExpression(::std::boxed::Box<ScopedCallExpression<'tree>>),
    String(::std::boxed::Box<String<'tree>>),
    SubscriptExpression(::std::boxed::Box<SubscriptExpression<'tree>>),
    VariableName(::std::boxed::Box<VariableName<'tree>>),
}
impl<'tree> ::treesitter_types::FromNode<'tree> for FunctionCallExpressionFunction<'tree> {
    #[allow(clippy::collapsible_else_if)]
    fn from_node(
        node: ::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        match node.kind() {
            "array_creation_expression" => {
                Ok(Self::ArrayCreationExpression(::std::boxed::Box::new(
                    <ArrayCreationExpression as ::treesitter_types::FromNode>::from_node(
                        node, src,
                    )?,
                )))
            }
            "dynamic_variable_name" => Ok(Self::DynamicVariableName(::std::boxed::Box::new(
                <DynamicVariableName as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "encapsed_string" => Ok(Self::EncapsedString(::std::boxed::Box::new(
                <EncapsedString as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "function_call_expression" => Ok(Self::FunctionCallExpression(::std::boxed::Box::new(
                <FunctionCallExpression as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "heredoc" => Ok(Self::Heredoc(::std::boxed::Box::new(
                <Heredoc as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "member_call_expression" => Ok(Self::MemberCallExpression(::std::boxed::Box::new(
                <MemberCallExpression as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "name" => Ok(Self::Name(::std::boxed::Box::new(
                <Name as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "nowdoc" => Ok(Self::Nowdoc(::std::boxed::Box::new(
                <Nowdoc as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "nullsafe_member_call_expression" => {
                Ok(Self::NullsafeMemberCallExpression(::std::boxed::Box::new(
                    <NullsafeMemberCallExpression as ::treesitter_types::FromNode>::from_node(
                        node, src,
                    )?,
                )))
            }
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
            "qualified_name" => Ok(Self::QualifiedName(::std::boxed::Box::new(
                <QualifiedName as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "relative_name" => Ok(Self::RelativeName(::std::boxed::Box::new(
                <RelativeName as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "scoped_call_expression" => Ok(Self::ScopedCallExpression(::std::boxed::Box::new(
                <ScopedCallExpression as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "string" => Ok(Self::String(::std::boxed::Box::new(
                <String as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "subscript_expression" => Ok(Self::SubscriptExpression(::std::boxed::Box::new(
                <SubscriptExpression as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "variable_name" => Ok(Self::VariableName(::std::boxed::Box::new(
                <VariableName as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            other => Err(::treesitter_types::ParseError::unexpected_kind(other, node)),
        }
    }
}
impl ::treesitter_types::Spanned for FunctionCallExpressionFunction<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        match self {
            Self::ArrayCreationExpression(inner) => inner.span(),
            Self::DynamicVariableName(inner) => inner.span(),
            Self::EncapsedString(inner) => inner.span(),
            Self::FunctionCallExpression(inner) => inner.span(),
            Self::Heredoc(inner) => inner.span(),
            Self::MemberCallExpression(inner) => inner.span(),
            Self::Name(inner) => inner.span(),
            Self::Nowdoc(inner) => inner.span(),
            Self::NullsafeMemberCallExpression(inner) => inner.span(),
            Self::ObjectCreationExpression(inner) => inner.span(),
            Self::ParenthesizedExpression(inner) => inner.span(),
            Self::QualifiedName(inner) => inner.span(),
            Self::RelativeName(inner) => inner.span(),
            Self::ScopedCallExpression(inner) => inner.span(),
            Self::String(inner) => inner.span(),
            Self::SubscriptExpression(inner) => inner.span(),
            Self::VariableName(inner) => inner.span(),
        }
    }
}
#[derive(Debug, Clone)]
pub enum FunctionDefinitionReturnType<'tree> {
    BottomType(::std::boxed::Box<BottomType<'tree>>),
    Type(::std::boxed::Box<Type<'tree>>),
}
impl<'tree> ::treesitter_types::FromNode<'tree> for FunctionDefinitionReturnType<'tree> {
    #[allow(clippy::collapsible_else_if)]
    fn from_node(
        node: ::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        match node.kind() {
            "bottom_type" => Ok(Self::BottomType(::std::boxed::Box::new(
                <BottomType as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
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
impl ::treesitter_types::Spanned for FunctionDefinitionReturnType<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        match self {
            Self::BottomType(inner) => inner.span(),
            Self::Type(inner) => inner.span(),
        }
    }
}
#[derive(Debug, Clone)]
pub enum GlobalDeclarationChildren<'tree> {
    DynamicVariableName(::std::boxed::Box<DynamicVariableName<'tree>>),
    VariableName(::std::boxed::Box<VariableName<'tree>>),
}
impl<'tree> ::treesitter_types::FromNode<'tree> for GlobalDeclarationChildren<'tree> {
    #[allow(clippy::collapsible_else_if)]
    fn from_node(
        node: ::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        match node.kind() {
            "dynamic_variable_name" => Ok(Self::DynamicVariableName(::std::boxed::Box::new(
                <DynamicVariableName as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "variable_name" => Ok(Self::VariableName(::std::boxed::Box::new(
                <VariableName as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            other => Err(::treesitter_types::ParseError::unexpected_kind(other, node)),
        }
    }
}
impl ::treesitter_types::Spanned for GlobalDeclarationChildren<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        match self {
            Self::DynamicVariableName(inner) => inner.span(),
            Self::VariableName(inner) => inner.span(),
        }
    }
}
#[derive(Debug, Clone)]
pub enum HeredocBodyChildren<'tree> {
    DynamicVariableName(::std::boxed::Box<DynamicVariableName<'tree>>),
    EscapeSequence(::std::boxed::Box<EscapeSequence<'tree>>),
    Expression(::std::boxed::Box<Expression<'tree>>),
    MemberAccessExpression(::std::boxed::Box<MemberAccessExpression<'tree>>),
    StringContent(::std::boxed::Box<StringContent<'tree>>),
    SubscriptExpression(::std::boxed::Box<SubscriptExpression<'tree>>),
    VariableName(::std::boxed::Box<VariableName<'tree>>),
}
impl<'tree> ::treesitter_types::FromNode<'tree> for HeredocBodyChildren<'tree> {
    #[allow(clippy::collapsible_else_if)]
    fn from_node(
        node: ::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        match node.kind() {
            "dynamic_variable_name" => Ok(Self::DynamicVariableName(::std::boxed::Box::new(
                <DynamicVariableName as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "escape_sequence" => Ok(Self::EscapeSequence(::std::boxed::Box::new(
                <EscapeSequence as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "member_access_expression" => Ok(Self::MemberAccessExpression(::std::boxed::Box::new(
                <MemberAccessExpression as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "string_content" => Ok(Self::StringContent(::std::boxed::Box::new(
                <StringContent as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "subscript_expression" => Ok(Self::SubscriptExpression(::std::boxed::Box::new(
                <SubscriptExpression as ::treesitter_types::FromNode>::from_node(node, src)?,
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
impl ::treesitter_types::Spanned for HeredocBodyChildren<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        match self {
            Self::DynamicVariableName(inner) => inner.span(),
            Self::EscapeSequence(inner) => inner.span(),
            Self::Expression(inner) => inner.span(),
            Self::MemberAccessExpression(inner) => inner.span(),
            Self::StringContent(inner) => inner.span(),
            Self::SubscriptExpression(inner) => inner.span(),
            Self::VariableName(inner) => inner.span(),
        }
    }
}
#[derive(Debug, Clone)]
pub enum IfStatementAlternative<'tree> {
    ElseClause(::std::boxed::Box<ElseClause<'tree>>),
    ElseIfClause(::std::boxed::Box<ElseIfClause<'tree>>),
}
impl<'tree> ::treesitter_types::FromNode<'tree> for IfStatementAlternative<'tree> {
    #[allow(clippy::collapsible_else_if)]
    fn from_node(
        node: ::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        match node.kind() {
            "else_clause" => Ok(Self::ElseClause(::std::boxed::Box::new(
                <ElseClause as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "else_if_clause" => Ok(Self::ElseIfClause(::std::boxed::Box::new(
                <ElseIfClause as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            other => Err(::treesitter_types::ParseError::unexpected_kind(other, node)),
        }
    }
}
impl ::treesitter_types::Spanned for IfStatementAlternative<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        match self {
            Self::ElseClause(inner) => inner.span(),
            Self::ElseIfClause(inner) => inner.span(),
        }
    }
}
#[derive(Debug, Clone)]
pub enum IfStatementBody<'tree> {
    ColonBlock(::std::boxed::Box<ColonBlock<'tree>>),
    Statement(::std::boxed::Box<Statement<'tree>>),
}
impl<'tree> ::treesitter_types::FromNode<'tree> for IfStatementBody<'tree> {
    #[allow(clippy::collapsible_else_if)]
    fn from_node(
        node: ::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        match node.kind() {
            "colon_block" => Ok(Self::ColonBlock(::std::boxed::Box::new(
                <ColonBlock as ::treesitter_types::FromNode>::from_node(node, src)?,
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
impl ::treesitter_types::Spanned for IfStatementBody<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        match self {
            Self::ColonBlock(inner) => inner.span(),
            Self::Statement(inner) => inner.span(),
        }
    }
}
#[derive(Debug, Clone)]
pub enum IntersectionTypeChildren<'tree> {
    NamedType(::std::boxed::Box<NamedType<'tree>>),
    OptionalType(::std::boxed::Box<OptionalType<'tree>>),
    PrimitiveType(::std::boxed::Box<PrimitiveType<'tree>>),
}
impl<'tree> ::treesitter_types::FromNode<'tree> for IntersectionTypeChildren<'tree> {
    #[allow(clippy::collapsible_else_if)]
    fn from_node(
        node: ::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        match node.kind() {
            "named_type" => Ok(Self::NamedType(::std::boxed::Box::new(
                <NamedType as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "optional_type" => Ok(Self::OptionalType(::std::boxed::Box::new(
                <OptionalType as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "primitive_type" => Ok(Self::PrimitiveType(::std::boxed::Box::new(
                <PrimitiveType as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            other => Err(::treesitter_types::ParseError::unexpected_kind(other, node)),
        }
    }
}
impl ::treesitter_types::Spanned for IntersectionTypeChildren<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        match self {
            Self::NamedType(inner) => inner.span(),
            Self::OptionalType(inner) => inner.span(),
            Self::PrimitiveType(inner) => inner.span(),
        }
    }
}
#[derive(Debug, Clone)]
pub enum ListLiteralChildren<'tree> {
    ByRef(::std::boxed::Box<ByRef<'tree>>),
    DynamicVariableName(::std::boxed::Box<DynamicVariableName<'tree>>),
    Expression(::std::boxed::Box<Expression<'tree>>),
    FunctionCallExpression(::std::boxed::Box<FunctionCallExpression<'tree>>),
    ListLiteral(::std::boxed::Box<ListLiteral<'tree>>),
    MemberAccessExpression(::std::boxed::Box<MemberAccessExpression<'tree>>),
    MemberCallExpression(::std::boxed::Box<MemberCallExpression<'tree>>),
    NullsafeMemberAccessExpression(::std::boxed::Box<NullsafeMemberAccessExpression<'tree>>),
    NullsafeMemberCallExpression(::std::boxed::Box<NullsafeMemberCallExpression<'tree>>),
    ScopedCallExpression(::std::boxed::Box<ScopedCallExpression<'tree>>),
    ScopedPropertyAccessExpression(::std::boxed::Box<ScopedPropertyAccessExpression<'tree>>),
    SubscriptExpression(::std::boxed::Box<SubscriptExpression<'tree>>),
    VariableName(::std::boxed::Box<VariableName<'tree>>),
}
impl<'tree> ::treesitter_types::FromNode<'tree> for ListLiteralChildren<'tree> {
    #[allow(clippy::collapsible_else_if)]
    fn from_node(
        node: ::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        match node.kind() {
            "by_ref" => Ok(Self::ByRef(::std::boxed::Box::new(
                <ByRef as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "dynamic_variable_name" => Ok(Self::DynamicVariableName(::std::boxed::Box::new(
                <DynamicVariableName as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "function_call_expression" => Ok(Self::FunctionCallExpression(::std::boxed::Box::new(
                <FunctionCallExpression as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "list_literal" => Ok(Self::ListLiteral(::std::boxed::Box::new(
                <ListLiteral as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "member_access_expression" => Ok(Self::MemberAccessExpression(::std::boxed::Box::new(
                <MemberAccessExpression as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "member_call_expression" => Ok(Self::MemberCallExpression(::std::boxed::Box::new(
                <MemberCallExpression as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "nullsafe_member_access_expression" => Ok(Self::NullsafeMemberAccessExpression(
                ::std::boxed::Box::new(
                    <NullsafeMemberAccessExpression as ::treesitter_types::FromNode>::from_node(
                        node, src,
                    )?,
                ),
            )),
            "nullsafe_member_call_expression" => {
                Ok(Self::NullsafeMemberCallExpression(::std::boxed::Box::new(
                    <NullsafeMemberCallExpression as ::treesitter_types::FromNode>::from_node(
                        node, src,
                    )?,
                )))
            }
            "scoped_call_expression" => Ok(Self::ScopedCallExpression(::std::boxed::Box::new(
                <ScopedCallExpression as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "scoped_property_access_expression" => Ok(Self::ScopedPropertyAccessExpression(
                ::std::boxed::Box::new(
                    <ScopedPropertyAccessExpression as ::treesitter_types::FromNode>::from_node(
                        node, src,
                    )?,
                ),
            )),
            "subscript_expression" => Ok(Self::SubscriptExpression(::std::boxed::Box::new(
                <SubscriptExpression as ::treesitter_types::FromNode>::from_node(node, src)?,
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
impl ::treesitter_types::Spanned for ListLiteralChildren<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        match self {
            Self::ByRef(inner) => inner.span(),
            Self::DynamicVariableName(inner) => inner.span(),
            Self::Expression(inner) => inner.span(),
            Self::FunctionCallExpression(inner) => inner.span(),
            Self::ListLiteral(inner) => inner.span(),
            Self::MemberAccessExpression(inner) => inner.span(),
            Self::MemberCallExpression(inner) => inner.span(),
            Self::NullsafeMemberAccessExpression(inner) => inner.span(),
            Self::NullsafeMemberCallExpression(inner) => inner.span(),
            Self::ScopedCallExpression(inner) => inner.span(),
            Self::ScopedPropertyAccessExpression(inner) => inner.span(),
            Self::SubscriptExpression(inner) => inner.span(),
            Self::VariableName(inner) => inner.span(),
        }
    }
}
#[derive(Debug, Clone)]
pub enum MatchBlockChildren<'tree> {
    MatchConditionalExpression(::std::boxed::Box<MatchConditionalExpression<'tree>>),
    MatchDefaultExpression(::std::boxed::Box<MatchDefaultExpression<'tree>>),
}
impl<'tree> ::treesitter_types::FromNode<'tree> for MatchBlockChildren<'tree> {
    #[allow(clippy::collapsible_else_if)]
    fn from_node(
        node: ::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        match node.kind() {
            "match_conditional_expression" => {
                Ok(Self::MatchConditionalExpression(::std::boxed::Box::new(
                    <MatchConditionalExpression as ::treesitter_types::FromNode>::from_node(
                        node, src,
                    )?,
                )))
            }
            "match_default_expression" => Ok(Self::MatchDefaultExpression(::std::boxed::Box::new(
                <MatchDefaultExpression as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            other => Err(::treesitter_types::ParseError::unexpected_kind(other, node)),
        }
    }
}
impl ::treesitter_types::Spanned for MatchBlockChildren<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        match self {
            Self::MatchConditionalExpression(inner) => inner.span(),
            Self::MatchDefaultExpression(inner) => inner.span(),
        }
    }
}
#[derive(Debug, Clone)]
pub enum MemberAccessExpressionName<'tree> {
    DynamicVariableName(::std::boxed::Box<DynamicVariableName<'tree>>),
    Expression(::std::boxed::Box<Expression<'tree>>),
    Name(::std::boxed::Box<Name<'tree>>),
    VariableName(::std::boxed::Box<VariableName<'tree>>),
}
impl<'tree> ::treesitter_types::FromNode<'tree> for MemberAccessExpressionName<'tree> {
    #[allow(clippy::collapsible_else_if)]
    fn from_node(
        node: ::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        match node.kind() {
            "dynamic_variable_name" => Ok(Self::DynamicVariableName(::std::boxed::Box::new(
                <DynamicVariableName as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "name" => Ok(Self::Name(::std::boxed::Box::new(
                <Name as ::treesitter_types::FromNode>::from_node(node, src)?,
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
impl ::treesitter_types::Spanned for MemberAccessExpressionName<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        match self {
            Self::DynamicVariableName(inner) => inner.span(),
            Self::Expression(inner) => inner.span(),
            Self::Name(inner) => inner.span(),
            Self::VariableName(inner) => inner.span(),
        }
    }
}
#[derive(Debug, Clone)]
pub enum MemberAccessExpressionObject<'tree> {
    ArrayCreationExpression(::std::boxed::Box<ArrayCreationExpression<'tree>>),
    CastExpression(::std::boxed::Box<CastExpression<'tree>>),
    ClassConstantAccessExpression(::std::boxed::Box<ClassConstantAccessExpression<'tree>>),
    DynamicVariableName(::std::boxed::Box<DynamicVariableName<'tree>>),
    EncapsedString(::std::boxed::Box<EncapsedString<'tree>>),
    FunctionCallExpression(::std::boxed::Box<FunctionCallExpression<'tree>>),
    Heredoc(::std::boxed::Box<Heredoc<'tree>>),
    MemberAccessExpression(::std::boxed::Box<MemberAccessExpression<'tree>>),
    MemberCallExpression(::std::boxed::Box<MemberCallExpression<'tree>>),
    Name(::std::boxed::Box<Name<'tree>>),
    Nowdoc(::std::boxed::Box<Nowdoc<'tree>>),
    NullsafeMemberAccessExpression(::std::boxed::Box<NullsafeMemberAccessExpression<'tree>>),
    NullsafeMemberCallExpression(::std::boxed::Box<NullsafeMemberCallExpression<'tree>>),
    ObjectCreationExpression(::std::boxed::Box<ObjectCreationExpression<'tree>>),
    ParenthesizedExpression(::std::boxed::Box<ParenthesizedExpression<'tree>>),
    QualifiedName(::std::boxed::Box<QualifiedName<'tree>>),
    RelativeName(::std::boxed::Box<RelativeName<'tree>>),
    ScopedCallExpression(::std::boxed::Box<ScopedCallExpression<'tree>>),
    ScopedPropertyAccessExpression(::std::boxed::Box<ScopedPropertyAccessExpression<'tree>>),
    String(::std::boxed::Box<String<'tree>>),
    SubscriptExpression(::std::boxed::Box<SubscriptExpression<'tree>>),
    VariableName(::std::boxed::Box<VariableName<'tree>>),
}
impl<'tree> ::treesitter_types::FromNode<'tree> for MemberAccessExpressionObject<'tree> {
    #[allow(clippy::collapsible_else_if)]
    fn from_node(
        node: ::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        match node.kind() {
            "array_creation_expression" => {
                Ok(Self::ArrayCreationExpression(::std::boxed::Box::new(
                    <ArrayCreationExpression as ::treesitter_types::FromNode>::from_node(
                        node, src,
                    )?,
                )))
            }
            "cast_expression" => Ok(Self::CastExpression(::std::boxed::Box::new(
                <CastExpression as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "class_constant_access_expression" => {
                Ok(Self::ClassConstantAccessExpression(::std::boxed::Box::new(
                    <ClassConstantAccessExpression as ::treesitter_types::FromNode>::from_node(
                        node, src,
                    )?,
                )))
            }
            "dynamic_variable_name" => Ok(Self::DynamicVariableName(::std::boxed::Box::new(
                <DynamicVariableName as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "encapsed_string" => Ok(Self::EncapsedString(::std::boxed::Box::new(
                <EncapsedString as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "function_call_expression" => Ok(Self::FunctionCallExpression(::std::boxed::Box::new(
                <FunctionCallExpression as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "heredoc" => Ok(Self::Heredoc(::std::boxed::Box::new(
                <Heredoc as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "member_access_expression" => Ok(Self::MemberAccessExpression(::std::boxed::Box::new(
                <MemberAccessExpression as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "member_call_expression" => Ok(Self::MemberCallExpression(::std::boxed::Box::new(
                <MemberCallExpression as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "name" => Ok(Self::Name(::std::boxed::Box::new(
                <Name as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "nowdoc" => Ok(Self::Nowdoc(::std::boxed::Box::new(
                <Nowdoc as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "nullsafe_member_access_expression" => Ok(Self::NullsafeMemberAccessExpression(
                ::std::boxed::Box::new(
                    <NullsafeMemberAccessExpression as ::treesitter_types::FromNode>::from_node(
                        node, src,
                    )?,
                ),
            )),
            "nullsafe_member_call_expression" => {
                Ok(Self::NullsafeMemberCallExpression(::std::boxed::Box::new(
                    <NullsafeMemberCallExpression as ::treesitter_types::FromNode>::from_node(
                        node, src,
                    )?,
                )))
            }
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
            "qualified_name" => Ok(Self::QualifiedName(::std::boxed::Box::new(
                <QualifiedName as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "relative_name" => Ok(Self::RelativeName(::std::boxed::Box::new(
                <RelativeName as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "scoped_call_expression" => Ok(Self::ScopedCallExpression(::std::boxed::Box::new(
                <ScopedCallExpression as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "scoped_property_access_expression" => Ok(Self::ScopedPropertyAccessExpression(
                ::std::boxed::Box::new(
                    <ScopedPropertyAccessExpression as ::treesitter_types::FromNode>::from_node(
                        node, src,
                    )?,
                ),
            )),
            "string" => Ok(Self::String(::std::boxed::Box::new(
                <String as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "subscript_expression" => Ok(Self::SubscriptExpression(::std::boxed::Box::new(
                <SubscriptExpression as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "variable_name" => Ok(Self::VariableName(::std::boxed::Box::new(
                <VariableName as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            other => Err(::treesitter_types::ParseError::unexpected_kind(other, node)),
        }
    }
}
impl ::treesitter_types::Spanned for MemberAccessExpressionObject<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        match self {
            Self::ArrayCreationExpression(inner) => inner.span(),
            Self::CastExpression(inner) => inner.span(),
            Self::ClassConstantAccessExpression(inner) => inner.span(),
            Self::DynamicVariableName(inner) => inner.span(),
            Self::EncapsedString(inner) => inner.span(),
            Self::FunctionCallExpression(inner) => inner.span(),
            Self::Heredoc(inner) => inner.span(),
            Self::MemberAccessExpression(inner) => inner.span(),
            Self::MemberCallExpression(inner) => inner.span(),
            Self::Name(inner) => inner.span(),
            Self::Nowdoc(inner) => inner.span(),
            Self::NullsafeMemberAccessExpression(inner) => inner.span(),
            Self::NullsafeMemberCallExpression(inner) => inner.span(),
            Self::ObjectCreationExpression(inner) => inner.span(),
            Self::ParenthesizedExpression(inner) => inner.span(),
            Self::QualifiedName(inner) => inner.span(),
            Self::RelativeName(inner) => inner.span(),
            Self::ScopedCallExpression(inner) => inner.span(),
            Self::ScopedPropertyAccessExpression(inner) => inner.span(),
            Self::String(inner) => inner.span(),
            Self::SubscriptExpression(inner) => inner.span(),
            Self::VariableName(inner) => inner.span(),
        }
    }
}
#[derive(Debug, Clone)]
pub enum MemberCallExpressionName<'tree> {
    DynamicVariableName(::std::boxed::Box<DynamicVariableName<'tree>>),
    Expression(::std::boxed::Box<Expression<'tree>>),
    Name(::std::boxed::Box<Name<'tree>>),
    VariableName(::std::boxed::Box<VariableName<'tree>>),
}
impl<'tree> ::treesitter_types::FromNode<'tree> for MemberCallExpressionName<'tree> {
    #[allow(clippy::collapsible_else_if)]
    fn from_node(
        node: ::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        match node.kind() {
            "dynamic_variable_name" => Ok(Self::DynamicVariableName(::std::boxed::Box::new(
                <DynamicVariableName as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "name" => Ok(Self::Name(::std::boxed::Box::new(
                <Name as ::treesitter_types::FromNode>::from_node(node, src)?,
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
impl ::treesitter_types::Spanned for MemberCallExpressionName<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        match self {
            Self::DynamicVariableName(inner) => inner.span(),
            Self::Expression(inner) => inner.span(),
            Self::Name(inner) => inner.span(),
            Self::VariableName(inner) => inner.span(),
        }
    }
}
#[derive(Debug, Clone)]
pub enum MemberCallExpressionObject<'tree> {
    ArrayCreationExpression(::std::boxed::Box<ArrayCreationExpression<'tree>>),
    CastExpression(::std::boxed::Box<CastExpression<'tree>>),
    ClassConstantAccessExpression(::std::boxed::Box<ClassConstantAccessExpression<'tree>>),
    DynamicVariableName(::std::boxed::Box<DynamicVariableName<'tree>>),
    EncapsedString(::std::boxed::Box<EncapsedString<'tree>>),
    FunctionCallExpression(::std::boxed::Box<FunctionCallExpression<'tree>>),
    Heredoc(::std::boxed::Box<Heredoc<'tree>>),
    MemberAccessExpression(::std::boxed::Box<MemberAccessExpression<'tree>>),
    MemberCallExpression(::std::boxed::Box<MemberCallExpression<'tree>>),
    Name(::std::boxed::Box<Name<'tree>>),
    Nowdoc(::std::boxed::Box<Nowdoc<'tree>>),
    NullsafeMemberAccessExpression(::std::boxed::Box<NullsafeMemberAccessExpression<'tree>>),
    NullsafeMemberCallExpression(::std::boxed::Box<NullsafeMemberCallExpression<'tree>>),
    ObjectCreationExpression(::std::boxed::Box<ObjectCreationExpression<'tree>>),
    ParenthesizedExpression(::std::boxed::Box<ParenthesizedExpression<'tree>>),
    QualifiedName(::std::boxed::Box<QualifiedName<'tree>>),
    RelativeName(::std::boxed::Box<RelativeName<'tree>>),
    ScopedCallExpression(::std::boxed::Box<ScopedCallExpression<'tree>>),
    ScopedPropertyAccessExpression(::std::boxed::Box<ScopedPropertyAccessExpression<'tree>>),
    String(::std::boxed::Box<String<'tree>>),
    SubscriptExpression(::std::boxed::Box<SubscriptExpression<'tree>>),
    VariableName(::std::boxed::Box<VariableName<'tree>>),
}
impl<'tree> ::treesitter_types::FromNode<'tree> for MemberCallExpressionObject<'tree> {
    #[allow(clippy::collapsible_else_if)]
    fn from_node(
        node: ::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        match node.kind() {
            "array_creation_expression" => {
                Ok(Self::ArrayCreationExpression(::std::boxed::Box::new(
                    <ArrayCreationExpression as ::treesitter_types::FromNode>::from_node(
                        node, src,
                    )?,
                )))
            }
            "cast_expression" => Ok(Self::CastExpression(::std::boxed::Box::new(
                <CastExpression as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "class_constant_access_expression" => {
                Ok(Self::ClassConstantAccessExpression(::std::boxed::Box::new(
                    <ClassConstantAccessExpression as ::treesitter_types::FromNode>::from_node(
                        node, src,
                    )?,
                )))
            }
            "dynamic_variable_name" => Ok(Self::DynamicVariableName(::std::boxed::Box::new(
                <DynamicVariableName as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "encapsed_string" => Ok(Self::EncapsedString(::std::boxed::Box::new(
                <EncapsedString as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "function_call_expression" => Ok(Self::FunctionCallExpression(::std::boxed::Box::new(
                <FunctionCallExpression as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "heredoc" => Ok(Self::Heredoc(::std::boxed::Box::new(
                <Heredoc as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "member_access_expression" => Ok(Self::MemberAccessExpression(::std::boxed::Box::new(
                <MemberAccessExpression as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "member_call_expression" => Ok(Self::MemberCallExpression(::std::boxed::Box::new(
                <MemberCallExpression as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "name" => Ok(Self::Name(::std::boxed::Box::new(
                <Name as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "nowdoc" => Ok(Self::Nowdoc(::std::boxed::Box::new(
                <Nowdoc as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "nullsafe_member_access_expression" => Ok(Self::NullsafeMemberAccessExpression(
                ::std::boxed::Box::new(
                    <NullsafeMemberAccessExpression as ::treesitter_types::FromNode>::from_node(
                        node, src,
                    )?,
                ),
            )),
            "nullsafe_member_call_expression" => {
                Ok(Self::NullsafeMemberCallExpression(::std::boxed::Box::new(
                    <NullsafeMemberCallExpression as ::treesitter_types::FromNode>::from_node(
                        node, src,
                    )?,
                )))
            }
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
            "qualified_name" => Ok(Self::QualifiedName(::std::boxed::Box::new(
                <QualifiedName as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "relative_name" => Ok(Self::RelativeName(::std::boxed::Box::new(
                <RelativeName as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "scoped_call_expression" => Ok(Self::ScopedCallExpression(::std::boxed::Box::new(
                <ScopedCallExpression as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "scoped_property_access_expression" => Ok(Self::ScopedPropertyAccessExpression(
                ::std::boxed::Box::new(
                    <ScopedPropertyAccessExpression as ::treesitter_types::FromNode>::from_node(
                        node, src,
                    )?,
                ),
            )),
            "string" => Ok(Self::String(::std::boxed::Box::new(
                <String as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "subscript_expression" => Ok(Self::SubscriptExpression(::std::boxed::Box::new(
                <SubscriptExpression as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "variable_name" => Ok(Self::VariableName(::std::boxed::Box::new(
                <VariableName as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            other => Err(::treesitter_types::ParseError::unexpected_kind(other, node)),
        }
    }
}
impl ::treesitter_types::Spanned for MemberCallExpressionObject<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        match self {
            Self::ArrayCreationExpression(inner) => inner.span(),
            Self::CastExpression(inner) => inner.span(),
            Self::ClassConstantAccessExpression(inner) => inner.span(),
            Self::DynamicVariableName(inner) => inner.span(),
            Self::EncapsedString(inner) => inner.span(),
            Self::FunctionCallExpression(inner) => inner.span(),
            Self::Heredoc(inner) => inner.span(),
            Self::MemberAccessExpression(inner) => inner.span(),
            Self::MemberCallExpression(inner) => inner.span(),
            Self::Name(inner) => inner.span(),
            Self::Nowdoc(inner) => inner.span(),
            Self::NullsafeMemberAccessExpression(inner) => inner.span(),
            Self::NullsafeMemberCallExpression(inner) => inner.span(),
            Self::ObjectCreationExpression(inner) => inner.span(),
            Self::ParenthesizedExpression(inner) => inner.span(),
            Self::QualifiedName(inner) => inner.span(),
            Self::RelativeName(inner) => inner.span(),
            Self::ScopedCallExpression(inner) => inner.span(),
            Self::ScopedPropertyAccessExpression(inner) => inner.span(),
            Self::String(inner) => inner.span(),
            Self::SubscriptExpression(inner) => inner.span(),
            Self::VariableName(inner) => inner.span(),
        }
    }
}
#[derive(Debug, Clone)]
pub enum MethodDeclarationReturnType<'tree> {
    BottomType(::std::boxed::Box<BottomType<'tree>>),
    Type(::std::boxed::Box<Type<'tree>>),
}
impl<'tree> ::treesitter_types::FromNode<'tree> for MethodDeclarationReturnType<'tree> {
    #[allow(clippy::collapsible_else_if)]
    fn from_node(
        node: ::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        match node.kind() {
            "bottom_type" => Ok(Self::BottomType(::std::boxed::Box::new(
                <BottomType as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
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
impl ::treesitter_types::Spanned for MethodDeclarationReturnType<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        match self {
            Self::BottomType(inner) => inner.span(),
            Self::Type(inner) => inner.span(),
        }
    }
}
#[derive(Debug, Clone)]
pub enum MethodDeclarationChildren<'tree> {
    AbstractModifier(::std::boxed::Box<AbstractModifier<'tree>>),
    FinalModifier(::std::boxed::Box<FinalModifier<'tree>>),
    ReadonlyModifier(::std::boxed::Box<ReadonlyModifier<'tree>>),
    ReferenceModifier(::std::boxed::Box<ReferenceModifier<'tree>>),
    StaticModifier(::std::boxed::Box<StaticModifier<'tree>>),
    VarModifier(::std::boxed::Box<VarModifier<'tree>>),
    VisibilityModifier(::std::boxed::Box<VisibilityModifier<'tree>>),
}
impl<'tree> ::treesitter_types::FromNode<'tree> for MethodDeclarationChildren<'tree> {
    #[allow(clippy::collapsible_else_if)]
    fn from_node(
        node: ::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        match node.kind() {
            "abstract_modifier" => Ok(Self::AbstractModifier(::std::boxed::Box::new(
                <AbstractModifier as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "final_modifier" => Ok(Self::FinalModifier(::std::boxed::Box::new(
                <FinalModifier as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "readonly_modifier" => Ok(Self::ReadonlyModifier(::std::boxed::Box::new(
                <ReadonlyModifier as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "reference_modifier" => Ok(Self::ReferenceModifier(::std::boxed::Box::new(
                <ReferenceModifier as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "static_modifier" => Ok(Self::StaticModifier(::std::boxed::Box::new(
                <StaticModifier as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "var_modifier" => Ok(Self::VarModifier(::std::boxed::Box::new(
                <VarModifier as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "visibility_modifier" => Ok(Self::VisibilityModifier(::std::boxed::Box::new(
                <VisibilityModifier as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            other => Err(::treesitter_types::ParseError::unexpected_kind(other, node)),
        }
    }
}
impl ::treesitter_types::Spanned for MethodDeclarationChildren<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        match self {
            Self::AbstractModifier(inner) => inner.span(),
            Self::FinalModifier(inner) => inner.span(),
            Self::ReadonlyModifier(inner) => inner.span(),
            Self::ReferenceModifier(inner) => inner.span(),
            Self::StaticModifier(inner) => inner.span(),
            Self::VarModifier(inner) => inner.span(),
            Self::VisibilityModifier(inner) => inner.span(),
        }
    }
}
#[derive(Debug, Clone)]
pub enum NamedTypeChildren<'tree> {
    Name(::std::boxed::Box<Name<'tree>>),
    QualifiedName(::std::boxed::Box<QualifiedName<'tree>>),
    RelativeName(::std::boxed::Box<RelativeName<'tree>>),
}
impl<'tree> ::treesitter_types::FromNode<'tree> for NamedTypeChildren<'tree> {
    #[allow(clippy::collapsible_else_if)]
    fn from_node(
        node: ::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        match node.kind() {
            "name" => Ok(Self::Name(::std::boxed::Box::new(
                <Name as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "qualified_name" => Ok(Self::QualifiedName(::std::boxed::Box::new(
                <QualifiedName as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "relative_name" => Ok(Self::RelativeName(::std::boxed::Box::new(
                <RelativeName as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            other => Err(::treesitter_types::ParseError::unexpected_kind(other, node)),
        }
    }
}
impl ::treesitter_types::Spanned for NamedTypeChildren<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        match self {
            Self::Name(inner) => inner.span(),
            Self::QualifiedName(inner) => inner.span(),
            Self::RelativeName(inner) => inner.span(),
        }
    }
}
#[derive(Debug, Clone)]
pub enum NamespaceUseClauseType {
    Const(::treesitter_types::Span),
    Function(::treesitter_types::Span),
}
impl<'tree> ::treesitter_types::FromNode<'tree> for NamespaceUseClauseType {
    #[allow(clippy::collapsible_else_if)]
    fn from_node(
        node: ::tree_sitter::Node<'tree>,
        _src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        match node.kind() {
            "const" => Ok(Self::Const(::treesitter_types::Span::from(node))),
            "function" => Ok(Self::Function(::treesitter_types::Span::from(node))),
            other => Err(::treesitter_types::ParseError::unexpected_kind(other, node)),
        }
    }
}
impl ::treesitter_types::Spanned for NamespaceUseClauseType {
    fn span(&self) -> ::treesitter_types::Span {
        match self {
            Self::Const(span) => *span,
            Self::Function(span) => *span,
        }
    }
}
#[derive(Debug, Clone)]
pub enum NamespaceUseClauseChildren<'tree> {
    Name(::std::boxed::Box<Name<'tree>>),
    QualifiedName(::std::boxed::Box<QualifiedName<'tree>>),
}
impl<'tree> ::treesitter_types::FromNode<'tree> for NamespaceUseClauseChildren<'tree> {
    #[allow(clippy::collapsible_else_if)]
    fn from_node(
        node: ::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        match node.kind() {
            "name" => Ok(Self::Name(::std::boxed::Box::new(
                <Name as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "qualified_name" => Ok(Self::QualifiedName(::std::boxed::Box::new(
                <QualifiedName as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            other => Err(::treesitter_types::ParseError::unexpected_kind(other, node)),
        }
    }
}
impl ::treesitter_types::Spanned for NamespaceUseClauseChildren<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        match self {
            Self::Name(inner) => inner.span(),
            Self::QualifiedName(inner) => inner.span(),
        }
    }
}
#[derive(Debug, Clone)]
pub enum NamespaceUseDeclarationType {
    Const(::treesitter_types::Span),
    Function(::treesitter_types::Span),
}
impl<'tree> ::treesitter_types::FromNode<'tree> for NamespaceUseDeclarationType {
    #[allow(clippy::collapsible_else_if)]
    fn from_node(
        node: ::tree_sitter::Node<'tree>,
        _src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        match node.kind() {
            "const" => Ok(Self::Const(::treesitter_types::Span::from(node))),
            "function" => Ok(Self::Function(::treesitter_types::Span::from(node))),
            other => Err(::treesitter_types::ParseError::unexpected_kind(other, node)),
        }
    }
}
impl ::treesitter_types::Spanned for NamespaceUseDeclarationType {
    fn span(&self) -> ::treesitter_types::Span {
        match self {
            Self::Const(span) => *span,
            Self::Function(span) => *span,
        }
    }
}
#[derive(Debug, Clone)]
pub enum NamespaceUseDeclarationChildren<'tree> {
    NamespaceName(::std::boxed::Box<NamespaceName<'tree>>),
    NamespaceUseClause(::std::boxed::Box<NamespaceUseClause<'tree>>),
}
impl<'tree> ::treesitter_types::FromNode<'tree> for NamespaceUseDeclarationChildren<'tree> {
    #[allow(clippy::collapsible_else_if)]
    fn from_node(
        node: ::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        match node.kind() {
            "namespace_name" => Ok(Self::NamespaceName(::std::boxed::Box::new(
                <NamespaceName as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "namespace_use_clause" => Ok(Self::NamespaceUseClause(::std::boxed::Box::new(
                <NamespaceUseClause as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            other => Err(::treesitter_types::ParseError::unexpected_kind(other, node)),
        }
    }
}
impl ::treesitter_types::Spanned for NamespaceUseDeclarationChildren<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        match self {
            Self::NamespaceName(inner) => inner.span(),
            Self::NamespaceUseClause(inner) => inner.span(),
        }
    }
}
#[derive(Debug, Clone)]
pub enum NullsafeMemberAccessExpressionName<'tree> {
    DynamicVariableName(::std::boxed::Box<DynamicVariableName<'tree>>),
    Expression(::std::boxed::Box<Expression<'tree>>),
    Name(::std::boxed::Box<Name<'tree>>),
    VariableName(::std::boxed::Box<VariableName<'tree>>),
}
impl<'tree> ::treesitter_types::FromNode<'tree> for NullsafeMemberAccessExpressionName<'tree> {
    #[allow(clippy::collapsible_else_if)]
    fn from_node(
        node: ::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        match node.kind() {
            "dynamic_variable_name" => Ok(Self::DynamicVariableName(::std::boxed::Box::new(
                <DynamicVariableName as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "name" => Ok(Self::Name(::std::boxed::Box::new(
                <Name as ::treesitter_types::FromNode>::from_node(node, src)?,
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
impl ::treesitter_types::Spanned for NullsafeMemberAccessExpressionName<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        match self {
            Self::DynamicVariableName(inner) => inner.span(),
            Self::Expression(inner) => inner.span(),
            Self::Name(inner) => inner.span(),
            Self::VariableName(inner) => inner.span(),
        }
    }
}
#[derive(Debug, Clone)]
pub enum NullsafeMemberAccessExpressionObject<'tree> {
    ArrayCreationExpression(::std::boxed::Box<ArrayCreationExpression<'tree>>),
    CastExpression(::std::boxed::Box<CastExpression<'tree>>),
    ClassConstantAccessExpression(::std::boxed::Box<ClassConstantAccessExpression<'tree>>),
    DynamicVariableName(::std::boxed::Box<DynamicVariableName<'tree>>),
    EncapsedString(::std::boxed::Box<EncapsedString<'tree>>),
    FunctionCallExpression(::std::boxed::Box<FunctionCallExpression<'tree>>),
    Heredoc(::std::boxed::Box<Heredoc<'tree>>),
    MemberAccessExpression(::std::boxed::Box<MemberAccessExpression<'tree>>),
    MemberCallExpression(::std::boxed::Box<MemberCallExpression<'tree>>),
    Name(::std::boxed::Box<Name<'tree>>),
    Nowdoc(::std::boxed::Box<Nowdoc<'tree>>),
    NullsafeMemberAccessExpression(::std::boxed::Box<NullsafeMemberAccessExpression<'tree>>),
    NullsafeMemberCallExpression(::std::boxed::Box<NullsafeMemberCallExpression<'tree>>),
    ObjectCreationExpression(::std::boxed::Box<ObjectCreationExpression<'tree>>),
    ParenthesizedExpression(::std::boxed::Box<ParenthesizedExpression<'tree>>),
    QualifiedName(::std::boxed::Box<QualifiedName<'tree>>),
    RelativeName(::std::boxed::Box<RelativeName<'tree>>),
    ScopedCallExpression(::std::boxed::Box<ScopedCallExpression<'tree>>),
    ScopedPropertyAccessExpression(::std::boxed::Box<ScopedPropertyAccessExpression<'tree>>),
    String(::std::boxed::Box<String<'tree>>),
    SubscriptExpression(::std::boxed::Box<SubscriptExpression<'tree>>),
    VariableName(::std::boxed::Box<VariableName<'tree>>),
}
impl<'tree> ::treesitter_types::FromNode<'tree> for NullsafeMemberAccessExpressionObject<'tree> {
    #[allow(clippy::collapsible_else_if)]
    fn from_node(
        node: ::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        match node.kind() {
            "array_creation_expression" => {
                Ok(Self::ArrayCreationExpression(::std::boxed::Box::new(
                    <ArrayCreationExpression as ::treesitter_types::FromNode>::from_node(
                        node, src,
                    )?,
                )))
            }
            "cast_expression" => Ok(Self::CastExpression(::std::boxed::Box::new(
                <CastExpression as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "class_constant_access_expression" => {
                Ok(Self::ClassConstantAccessExpression(::std::boxed::Box::new(
                    <ClassConstantAccessExpression as ::treesitter_types::FromNode>::from_node(
                        node, src,
                    )?,
                )))
            }
            "dynamic_variable_name" => Ok(Self::DynamicVariableName(::std::boxed::Box::new(
                <DynamicVariableName as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "encapsed_string" => Ok(Self::EncapsedString(::std::boxed::Box::new(
                <EncapsedString as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "function_call_expression" => Ok(Self::FunctionCallExpression(::std::boxed::Box::new(
                <FunctionCallExpression as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "heredoc" => Ok(Self::Heredoc(::std::boxed::Box::new(
                <Heredoc as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "member_access_expression" => Ok(Self::MemberAccessExpression(::std::boxed::Box::new(
                <MemberAccessExpression as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "member_call_expression" => Ok(Self::MemberCallExpression(::std::boxed::Box::new(
                <MemberCallExpression as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "name" => Ok(Self::Name(::std::boxed::Box::new(
                <Name as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "nowdoc" => Ok(Self::Nowdoc(::std::boxed::Box::new(
                <Nowdoc as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "nullsafe_member_access_expression" => Ok(Self::NullsafeMemberAccessExpression(
                ::std::boxed::Box::new(
                    <NullsafeMemberAccessExpression as ::treesitter_types::FromNode>::from_node(
                        node, src,
                    )?,
                ),
            )),
            "nullsafe_member_call_expression" => {
                Ok(Self::NullsafeMemberCallExpression(::std::boxed::Box::new(
                    <NullsafeMemberCallExpression as ::treesitter_types::FromNode>::from_node(
                        node, src,
                    )?,
                )))
            }
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
            "qualified_name" => Ok(Self::QualifiedName(::std::boxed::Box::new(
                <QualifiedName as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "relative_name" => Ok(Self::RelativeName(::std::boxed::Box::new(
                <RelativeName as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "scoped_call_expression" => Ok(Self::ScopedCallExpression(::std::boxed::Box::new(
                <ScopedCallExpression as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "scoped_property_access_expression" => Ok(Self::ScopedPropertyAccessExpression(
                ::std::boxed::Box::new(
                    <ScopedPropertyAccessExpression as ::treesitter_types::FromNode>::from_node(
                        node, src,
                    )?,
                ),
            )),
            "string" => Ok(Self::String(::std::boxed::Box::new(
                <String as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "subscript_expression" => Ok(Self::SubscriptExpression(::std::boxed::Box::new(
                <SubscriptExpression as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "variable_name" => Ok(Self::VariableName(::std::boxed::Box::new(
                <VariableName as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            other => Err(::treesitter_types::ParseError::unexpected_kind(other, node)),
        }
    }
}
impl ::treesitter_types::Spanned for NullsafeMemberAccessExpressionObject<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        match self {
            Self::ArrayCreationExpression(inner) => inner.span(),
            Self::CastExpression(inner) => inner.span(),
            Self::ClassConstantAccessExpression(inner) => inner.span(),
            Self::DynamicVariableName(inner) => inner.span(),
            Self::EncapsedString(inner) => inner.span(),
            Self::FunctionCallExpression(inner) => inner.span(),
            Self::Heredoc(inner) => inner.span(),
            Self::MemberAccessExpression(inner) => inner.span(),
            Self::MemberCallExpression(inner) => inner.span(),
            Self::Name(inner) => inner.span(),
            Self::Nowdoc(inner) => inner.span(),
            Self::NullsafeMemberAccessExpression(inner) => inner.span(),
            Self::NullsafeMemberCallExpression(inner) => inner.span(),
            Self::ObjectCreationExpression(inner) => inner.span(),
            Self::ParenthesizedExpression(inner) => inner.span(),
            Self::QualifiedName(inner) => inner.span(),
            Self::RelativeName(inner) => inner.span(),
            Self::ScopedCallExpression(inner) => inner.span(),
            Self::ScopedPropertyAccessExpression(inner) => inner.span(),
            Self::String(inner) => inner.span(),
            Self::SubscriptExpression(inner) => inner.span(),
            Self::VariableName(inner) => inner.span(),
        }
    }
}
#[derive(Debug, Clone)]
pub enum NullsafeMemberCallExpressionName<'tree> {
    DynamicVariableName(::std::boxed::Box<DynamicVariableName<'tree>>),
    Expression(::std::boxed::Box<Expression<'tree>>),
    Name(::std::boxed::Box<Name<'tree>>),
    VariableName(::std::boxed::Box<VariableName<'tree>>),
}
impl<'tree> ::treesitter_types::FromNode<'tree> for NullsafeMemberCallExpressionName<'tree> {
    #[allow(clippy::collapsible_else_if)]
    fn from_node(
        node: ::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        match node.kind() {
            "dynamic_variable_name" => Ok(Self::DynamicVariableName(::std::boxed::Box::new(
                <DynamicVariableName as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "name" => Ok(Self::Name(::std::boxed::Box::new(
                <Name as ::treesitter_types::FromNode>::from_node(node, src)?,
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
impl ::treesitter_types::Spanned for NullsafeMemberCallExpressionName<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        match self {
            Self::DynamicVariableName(inner) => inner.span(),
            Self::Expression(inner) => inner.span(),
            Self::Name(inner) => inner.span(),
            Self::VariableName(inner) => inner.span(),
        }
    }
}
#[derive(Debug, Clone)]
pub enum NullsafeMemberCallExpressionObject<'tree> {
    ArrayCreationExpression(::std::boxed::Box<ArrayCreationExpression<'tree>>),
    CastExpression(::std::boxed::Box<CastExpression<'tree>>),
    ClassConstantAccessExpression(::std::boxed::Box<ClassConstantAccessExpression<'tree>>),
    DynamicVariableName(::std::boxed::Box<DynamicVariableName<'tree>>),
    EncapsedString(::std::boxed::Box<EncapsedString<'tree>>),
    FunctionCallExpression(::std::boxed::Box<FunctionCallExpression<'tree>>),
    Heredoc(::std::boxed::Box<Heredoc<'tree>>),
    MemberAccessExpression(::std::boxed::Box<MemberAccessExpression<'tree>>),
    MemberCallExpression(::std::boxed::Box<MemberCallExpression<'tree>>),
    Name(::std::boxed::Box<Name<'tree>>),
    Nowdoc(::std::boxed::Box<Nowdoc<'tree>>),
    NullsafeMemberAccessExpression(::std::boxed::Box<NullsafeMemberAccessExpression<'tree>>),
    NullsafeMemberCallExpression(::std::boxed::Box<NullsafeMemberCallExpression<'tree>>),
    ObjectCreationExpression(::std::boxed::Box<ObjectCreationExpression<'tree>>),
    ParenthesizedExpression(::std::boxed::Box<ParenthesizedExpression<'tree>>),
    QualifiedName(::std::boxed::Box<QualifiedName<'tree>>),
    RelativeName(::std::boxed::Box<RelativeName<'tree>>),
    ScopedCallExpression(::std::boxed::Box<ScopedCallExpression<'tree>>),
    ScopedPropertyAccessExpression(::std::boxed::Box<ScopedPropertyAccessExpression<'tree>>),
    String(::std::boxed::Box<String<'tree>>),
    SubscriptExpression(::std::boxed::Box<SubscriptExpression<'tree>>),
    VariableName(::std::boxed::Box<VariableName<'tree>>),
}
impl<'tree> ::treesitter_types::FromNode<'tree> for NullsafeMemberCallExpressionObject<'tree> {
    #[allow(clippy::collapsible_else_if)]
    fn from_node(
        node: ::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        match node.kind() {
            "array_creation_expression" => {
                Ok(Self::ArrayCreationExpression(::std::boxed::Box::new(
                    <ArrayCreationExpression as ::treesitter_types::FromNode>::from_node(
                        node, src,
                    )?,
                )))
            }
            "cast_expression" => Ok(Self::CastExpression(::std::boxed::Box::new(
                <CastExpression as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "class_constant_access_expression" => {
                Ok(Self::ClassConstantAccessExpression(::std::boxed::Box::new(
                    <ClassConstantAccessExpression as ::treesitter_types::FromNode>::from_node(
                        node, src,
                    )?,
                )))
            }
            "dynamic_variable_name" => Ok(Self::DynamicVariableName(::std::boxed::Box::new(
                <DynamicVariableName as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "encapsed_string" => Ok(Self::EncapsedString(::std::boxed::Box::new(
                <EncapsedString as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "function_call_expression" => Ok(Self::FunctionCallExpression(::std::boxed::Box::new(
                <FunctionCallExpression as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "heredoc" => Ok(Self::Heredoc(::std::boxed::Box::new(
                <Heredoc as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "member_access_expression" => Ok(Self::MemberAccessExpression(::std::boxed::Box::new(
                <MemberAccessExpression as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "member_call_expression" => Ok(Self::MemberCallExpression(::std::boxed::Box::new(
                <MemberCallExpression as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "name" => Ok(Self::Name(::std::boxed::Box::new(
                <Name as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "nowdoc" => Ok(Self::Nowdoc(::std::boxed::Box::new(
                <Nowdoc as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "nullsafe_member_access_expression" => Ok(Self::NullsafeMemberAccessExpression(
                ::std::boxed::Box::new(
                    <NullsafeMemberAccessExpression as ::treesitter_types::FromNode>::from_node(
                        node, src,
                    )?,
                ),
            )),
            "nullsafe_member_call_expression" => {
                Ok(Self::NullsafeMemberCallExpression(::std::boxed::Box::new(
                    <NullsafeMemberCallExpression as ::treesitter_types::FromNode>::from_node(
                        node, src,
                    )?,
                )))
            }
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
            "qualified_name" => Ok(Self::QualifiedName(::std::boxed::Box::new(
                <QualifiedName as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "relative_name" => Ok(Self::RelativeName(::std::boxed::Box::new(
                <RelativeName as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "scoped_call_expression" => Ok(Self::ScopedCallExpression(::std::boxed::Box::new(
                <ScopedCallExpression as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "scoped_property_access_expression" => Ok(Self::ScopedPropertyAccessExpression(
                ::std::boxed::Box::new(
                    <ScopedPropertyAccessExpression as ::treesitter_types::FromNode>::from_node(
                        node, src,
                    )?,
                ),
            )),
            "string" => Ok(Self::String(::std::boxed::Box::new(
                <String as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "subscript_expression" => Ok(Self::SubscriptExpression(::std::boxed::Box::new(
                <SubscriptExpression as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "variable_name" => Ok(Self::VariableName(::std::boxed::Box::new(
                <VariableName as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            other => Err(::treesitter_types::ParseError::unexpected_kind(other, node)),
        }
    }
}
impl ::treesitter_types::Spanned for NullsafeMemberCallExpressionObject<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        match self {
            Self::ArrayCreationExpression(inner) => inner.span(),
            Self::CastExpression(inner) => inner.span(),
            Self::ClassConstantAccessExpression(inner) => inner.span(),
            Self::DynamicVariableName(inner) => inner.span(),
            Self::EncapsedString(inner) => inner.span(),
            Self::FunctionCallExpression(inner) => inner.span(),
            Self::Heredoc(inner) => inner.span(),
            Self::MemberAccessExpression(inner) => inner.span(),
            Self::MemberCallExpression(inner) => inner.span(),
            Self::Name(inner) => inner.span(),
            Self::Nowdoc(inner) => inner.span(),
            Self::NullsafeMemberAccessExpression(inner) => inner.span(),
            Self::NullsafeMemberCallExpression(inner) => inner.span(),
            Self::ObjectCreationExpression(inner) => inner.span(),
            Self::ParenthesizedExpression(inner) => inner.span(),
            Self::QualifiedName(inner) => inner.span(),
            Self::RelativeName(inner) => inner.span(),
            Self::ScopedCallExpression(inner) => inner.span(),
            Self::ScopedPropertyAccessExpression(inner) => inner.span(),
            Self::String(inner) => inner.span(),
            Self::SubscriptExpression(inner) => inner.span(),
            Self::VariableName(inner) => inner.span(),
        }
    }
}
#[derive(Debug, Clone)]
pub enum ObjectCreationExpressionChildren<'tree> {
    AnonymousClass(::std::boxed::Box<AnonymousClass<'tree>>),
    Arguments(::std::boxed::Box<Arguments<'tree>>),
    DynamicVariableName(::std::boxed::Box<DynamicVariableName<'tree>>),
    MemberAccessExpression(::std::boxed::Box<MemberAccessExpression<'tree>>),
    Name(::std::boxed::Box<Name<'tree>>),
    NullsafeMemberAccessExpression(::std::boxed::Box<NullsafeMemberAccessExpression<'tree>>),
    ParenthesizedExpression(::std::boxed::Box<ParenthesizedExpression<'tree>>),
    QualifiedName(::std::boxed::Box<QualifiedName<'tree>>),
    RelativeName(::std::boxed::Box<RelativeName<'tree>>),
    ScopedPropertyAccessExpression(::std::boxed::Box<ScopedPropertyAccessExpression<'tree>>),
    SubscriptExpression(::std::boxed::Box<SubscriptExpression<'tree>>),
    VariableName(::std::boxed::Box<VariableName<'tree>>),
}
impl<'tree> ::treesitter_types::FromNode<'tree> for ObjectCreationExpressionChildren<'tree> {
    #[allow(clippy::collapsible_else_if)]
    fn from_node(
        node: ::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        match node.kind() {
            "anonymous_class" => Ok(Self::AnonymousClass(::std::boxed::Box::new(
                <AnonymousClass as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "arguments" => Ok(Self::Arguments(::std::boxed::Box::new(
                <Arguments as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "dynamic_variable_name" => Ok(Self::DynamicVariableName(::std::boxed::Box::new(
                <DynamicVariableName as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "member_access_expression" => Ok(Self::MemberAccessExpression(::std::boxed::Box::new(
                <MemberAccessExpression as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "name" => Ok(Self::Name(::std::boxed::Box::new(
                <Name as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "nullsafe_member_access_expression" => Ok(Self::NullsafeMemberAccessExpression(
                ::std::boxed::Box::new(
                    <NullsafeMemberAccessExpression as ::treesitter_types::FromNode>::from_node(
                        node, src,
                    )?,
                ),
            )),
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
            "relative_name" => Ok(Self::RelativeName(::std::boxed::Box::new(
                <RelativeName as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "scoped_property_access_expression" => Ok(Self::ScopedPropertyAccessExpression(
                ::std::boxed::Box::new(
                    <ScopedPropertyAccessExpression as ::treesitter_types::FromNode>::from_node(
                        node, src,
                    )?,
                ),
            )),
            "subscript_expression" => Ok(Self::SubscriptExpression(::std::boxed::Box::new(
                <SubscriptExpression as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "variable_name" => Ok(Self::VariableName(::std::boxed::Box::new(
                <VariableName as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            other => Err(::treesitter_types::ParseError::unexpected_kind(other, node)),
        }
    }
}
impl ::treesitter_types::Spanned for ObjectCreationExpressionChildren<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        match self {
            Self::AnonymousClass(inner) => inner.span(),
            Self::Arguments(inner) => inner.span(),
            Self::DynamicVariableName(inner) => inner.span(),
            Self::MemberAccessExpression(inner) => inner.span(),
            Self::Name(inner) => inner.span(),
            Self::NullsafeMemberAccessExpression(inner) => inner.span(),
            Self::ParenthesizedExpression(inner) => inner.span(),
            Self::QualifiedName(inner) => inner.span(),
            Self::RelativeName(inner) => inner.span(),
            Self::ScopedPropertyAccessExpression(inner) => inner.span(),
            Self::SubscriptExpression(inner) => inner.span(),
            Self::VariableName(inner) => inner.span(),
        }
    }
}
#[derive(Debug, Clone)]
pub enum OptionalTypeChildren<'tree> {
    NamedType(::std::boxed::Box<NamedType<'tree>>),
    PrimitiveType(::std::boxed::Box<PrimitiveType<'tree>>),
}
impl<'tree> ::treesitter_types::FromNode<'tree> for OptionalTypeChildren<'tree> {
    #[allow(clippy::collapsible_else_if)]
    fn from_node(
        node: ::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        match node.kind() {
            "named_type" => Ok(Self::NamedType(::std::boxed::Box::new(
                <NamedType as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "primitive_type" => Ok(Self::PrimitiveType(::std::boxed::Box::new(
                <PrimitiveType as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            other => Err(::treesitter_types::ParseError::unexpected_kind(other, node)),
        }
    }
}
impl ::treesitter_types::Spanned for OptionalTypeChildren<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        match self {
            Self::NamedType(inner) => inner.span(),
            Self::PrimitiveType(inner) => inner.span(),
        }
    }
}
#[derive(Debug, Clone)]
pub enum PairChildren<'tree> {
    ByRef(::std::boxed::Box<ByRef<'tree>>),
    Expression(::std::boxed::Box<Expression<'tree>>),
    ListLiteral(::std::boxed::Box<ListLiteral<'tree>>),
}
impl<'tree> ::treesitter_types::FromNode<'tree> for PairChildren<'tree> {
    #[allow(clippy::collapsible_else_if)]
    fn from_node(
        node: ::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        match node.kind() {
            "by_ref" => Ok(Self::ByRef(::std::boxed::Box::new(
                <ByRef as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "list_literal" => Ok(Self::ListLiteral(::std::boxed::Box::new(
                <ListLiteral as ::treesitter_types::FromNode>::from_node(node, src)?,
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
impl ::treesitter_types::Spanned for PairChildren<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        match self {
            Self::ByRef(inner) => inner.span(),
            Self::Expression(inner) => inner.span(),
            Self::ListLiteral(inner) => inner.span(),
        }
    }
}
#[derive(Debug, Clone)]
pub enum ProgramChildren<'tree> {
    PhpTag(::std::boxed::Box<PhpTag<'tree>>),
    Statement(::std::boxed::Box<Statement<'tree>>),
    Text(::std::boxed::Box<Text<'tree>>),
}
impl<'tree> ::treesitter_types::FromNode<'tree> for ProgramChildren<'tree> {
    #[allow(clippy::collapsible_else_if)]
    fn from_node(
        node: ::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        match node.kind() {
            "php_tag" => Ok(Self::PhpTag(::std::boxed::Box::new(
                <PhpTag as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "text" => Ok(Self::Text(::std::boxed::Box::new(
                <Text as ::treesitter_types::FromNode>::from_node(node, src)?,
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
impl ::treesitter_types::Spanned for ProgramChildren<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        match self {
            Self::PhpTag(inner) => inner.span(),
            Self::Statement(inner) => inner.span(),
            Self::Text(inner) => inner.span(),
        }
    }
}
#[derive(Debug, Clone)]
pub enum PropertyDeclarationChildren<'tree> {
    AbstractModifier(::std::boxed::Box<AbstractModifier<'tree>>),
    FinalModifier(::std::boxed::Box<FinalModifier<'tree>>),
    PropertyElement(::std::boxed::Box<PropertyElement<'tree>>),
    PropertyHookList(::std::boxed::Box<PropertyHookList<'tree>>),
    ReadonlyModifier(::std::boxed::Box<ReadonlyModifier<'tree>>),
    StaticModifier(::std::boxed::Box<StaticModifier<'tree>>),
    VarModifier(::std::boxed::Box<VarModifier<'tree>>),
    VisibilityModifier(::std::boxed::Box<VisibilityModifier<'tree>>),
}
impl<'tree> ::treesitter_types::FromNode<'tree> for PropertyDeclarationChildren<'tree> {
    #[allow(clippy::collapsible_else_if)]
    fn from_node(
        node: ::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        match node.kind() {
            "abstract_modifier" => Ok(Self::AbstractModifier(::std::boxed::Box::new(
                <AbstractModifier as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "final_modifier" => Ok(Self::FinalModifier(::std::boxed::Box::new(
                <FinalModifier as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "property_element" => Ok(Self::PropertyElement(::std::boxed::Box::new(
                <PropertyElement as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "property_hook_list" => Ok(Self::PropertyHookList(::std::boxed::Box::new(
                <PropertyHookList as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "readonly_modifier" => Ok(Self::ReadonlyModifier(::std::boxed::Box::new(
                <ReadonlyModifier as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "static_modifier" => Ok(Self::StaticModifier(::std::boxed::Box::new(
                <StaticModifier as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "var_modifier" => Ok(Self::VarModifier(::std::boxed::Box::new(
                <VarModifier as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "visibility_modifier" => Ok(Self::VisibilityModifier(::std::boxed::Box::new(
                <VisibilityModifier as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            other => Err(::treesitter_types::ParseError::unexpected_kind(other, node)),
        }
    }
}
impl ::treesitter_types::Spanned for PropertyDeclarationChildren<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        match self {
            Self::AbstractModifier(inner) => inner.span(),
            Self::FinalModifier(inner) => inner.span(),
            Self::PropertyElement(inner) => inner.span(),
            Self::PropertyHookList(inner) => inner.span(),
            Self::ReadonlyModifier(inner) => inner.span(),
            Self::StaticModifier(inner) => inner.span(),
            Self::VarModifier(inner) => inner.span(),
            Self::VisibilityModifier(inner) => inner.span(),
        }
    }
}
#[derive(Debug, Clone)]
pub enum PropertyHookBody<'tree> {
    CompoundStatement(::std::boxed::Box<CompoundStatement<'tree>>),
    Expression(::std::boxed::Box<Expression<'tree>>),
}
impl<'tree> ::treesitter_types::FromNode<'tree> for PropertyHookBody<'tree> {
    #[allow(clippy::collapsible_else_if)]
    fn from_node(
        node: ::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        match node.kind() {
            "compound_statement" => Ok(Self::CompoundStatement(::std::boxed::Box::new(
                <CompoundStatement as ::treesitter_types::FromNode>::from_node(node, src)?,
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
impl ::treesitter_types::Spanned for PropertyHookBody<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        match self {
            Self::CompoundStatement(inner) => inner.span(),
            Self::Expression(inner) => inner.span(),
        }
    }
}
#[derive(Debug, Clone)]
pub enum PropertyPromotionParameterName<'tree> {
    ByRef(::std::boxed::Box<ByRef<'tree>>),
    VariableName(::std::boxed::Box<VariableName<'tree>>),
}
impl<'tree> ::treesitter_types::FromNode<'tree> for PropertyPromotionParameterName<'tree> {
    #[allow(clippy::collapsible_else_if)]
    fn from_node(
        node: ::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        match node.kind() {
            "by_ref" => Ok(Self::ByRef(::std::boxed::Box::new(
                <ByRef as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "variable_name" => Ok(Self::VariableName(::std::boxed::Box::new(
                <VariableName as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            other => Err(::treesitter_types::ParseError::unexpected_kind(other, node)),
        }
    }
}
impl ::treesitter_types::Spanned for PropertyPromotionParameterName<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        match self {
            Self::ByRef(inner) => inner.span(),
            Self::VariableName(inner) => inner.span(),
        }
    }
}
#[derive(Debug, Clone)]
pub enum QualifiedNamePrefix<'tree> {
    Backslash(::treesitter_types::Span),
    NamespaceName(::std::boxed::Box<NamespaceName<'tree>>),
}
impl<'tree> ::treesitter_types::FromNode<'tree> for QualifiedNamePrefix<'tree> {
    #[allow(clippy::collapsible_else_if)]
    fn from_node(
        node: ::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        match node.kind() {
            "\\" => Ok(Self::Backslash(::treesitter_types::Span::from(node))),
            "namespace_name" => Ok(Self::NamespaceName(::std::boxed::Box::new(
                <NamespaceName as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            other => Err(::treesitter_types::ParseError::unexpected_kind(other, node)),
        }
    }
}
impl ::treesitter_types::Spanned for QualifiedNamePrefix<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        match self {
            Self::Backslash(span) => *span,
            Self::NamespaceName(inner) => inner.span(),
        }
    }
}
#[derive(Debug, Clone)]
pub enum ReferenceAssignmentExpressionLeft<'tree> {
    CastExpression(::std::boxed::Box<CastExpression<'tree>>),
    DynamicVariableName(::std::boxed::Box<DynamicVariableName<'tree>>),
    FunctionCallExpression(::std::boxed::Box<FunctionCallExpression<'tree>>),
    ListLiteral(::std::boxed::Box<ListLiteral<'tree>>),
    MemberAccessExpression(::std::boxed::Box<MemberAccessExpression<'tree>>),
    MemberCallExpression(::std::boxed::Box<MemberCallExpression<'tree>>),
    NullsafeMemberAccessExpression(::std::boxed::Box<NullsafeMemberAccessExpression<'tree>>),
    NullsafeMemberCallExpression(::std::boxed::Box<NullsafeMemberCallExpression<'tree>>),
    ScopedCallExpression(::std::boxed::Box<ScopedCallExpression<'tree>>),
    ScopedPropertyAccessExpression(::std::boxed::Box<ScopedPropertyAccessExpression<'tree>>),
    SubscriptExpression(::std::boxed::Box<SubscriptExpression<'tree>>),
    VariableName(::std::boxed::Box<VariableName<'tree>>),
}
impl<'tree> ::treesitter_types::FromNode<'tree> for ReferenceAssignmentExpressionLeft<'tree> {
    #[allow(clippy::collapsible_else_if)]
    fn from_node(
        node: ::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        match node.kind() {
            "cast_expression" => Ok(Self::CastExpression(::std::boxed::Box::new(
                <CastExpression as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "dynamic_variable_name" => Ok(Self::DynamicVariableName(::std::boxed::Box::new(
                <DynamicVariableName as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "function_call_expression" => Ok(Self::FunctionCallExpression(::std::boxed::Box::new(
                <FunctionCallExpression as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "list_literal" => Ok(Self::ListLiteral(::std::boxed::Box::new(
                <ListLiteral as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "member_access_expression" => Ok(Self::MemberAccessExpression(::std::boxed::Box::new(
                <MemberAccessExpression as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "member_call_expression" => Ok(Self::MemberCallExpression(::std::boxed::Box::new(
                <MemberCallExpression as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "nullsafe_member_access_expression" => Ok(Self::NullsafeMemberAccessExpression(
                ::std::boxed::Box::new(
                    <NullsafeMemberAccessExpression as ::treesitter_types::FromNode>::from_node(
                        node, src,
                    )?,
                ),
            )),
            "nullsafe_member_call_expression" => {
                Ok(Self::NullsafeMemberCallExpression(::std::boxed::Box::new(
                    <NullsafeMemberCallExpression as ::treesitter_types::FromNode>::from_node(
                        node, src,
                    )?,
                )))
            }
            "scoped_call_expression" => Ok(Self::ScopedCallExpression(::std::boxed::Box::new(
                <ScopedCallExpression as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "scoped_property_access_expression" => Ok(Self::ScopedPropertyAccessExpression(
                ::std::boxed::Box::new(
                    <ScopedPropertyAccessExpression as ::treesitter_types::FromNode>::from_node(
                        node, src,
                    )?,
                ),
            )),
            "subscript_expression" => Ok(Self::SubscriptExpression(::std::boxed::Box::new(
                <SubscriptExpression as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "variable_name" => Ok(Self::VariableName(::std::boxed::Box::new(
                <VariableName as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            other => Err(::treesitter_types::ParseError::unexpected_kind(other, node)),
        }
    }
}
impl ::treesitter_types::Spanned for ReferenceAssignmentExpressionLeft<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        match self {
            Self::CastExpression(inner) => inner.span(),
            Self::DynamicVariableName(inner) => inner.span(),
            Self::FunctionCallExpression(inner) => inner.span(),
            Self::ListLiteral(inner) => inner.span(),
            Self::MemberAccessExpression(inner) => inner.span(),
            Self::MemberCallExpression(inner) => inner.span(),
            Self::NullsafeMemberAccessExpression(inner) => inner.span(),
            Self::NullsafeMemberCallExpression(inner) => inner.span(),
            Self::ScopedCallExpression(inner) => inner.span(),
            Self::ScopedPropertyAccessExpression(inner) => inner.span(),
            Self::SubscriptExpression(inner) => inner.span(),
            Self::VariableName(inner) => inner.span(),
        }
    }
}
#[derive(Debug, Clone)]
pub enum RelativeNamePrefix<'tree> {
    Backslash(::treesitter_types::Span),
    Namespace(::treesitter_types::Span),
    NamespaceName(::std::boxed::Box<NamespaceName<'tree>>),
}
impl<'tree> ::treesitter_types::FromNode<'tree> for RelativeNamePrefix<'tree> {
    #[allow(clippy::collapsible_else_if)]
    fn from_node(
        node: ::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        match node.kind() {
            "\\" => Ok(Self::Backslash(::treesitter_types::Span::from(node))),
            "namespace" => Ok(Self::Namespace(::treesitter_types::Span::from(node))),
            "namespace_name" => Ok(Self::NamespaceName(::std::boxed::Box::new(
                <NamespaceName as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            other => Err(::treesitter_types::ParseError::unexpected_kind(other, node)),
        }
    }
}
impl ::treesitter_types::Spanned for RelativeNamePrefix<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        match self {
            Self::Backslash(span) => *span,
            Self::Namespace(span) => *span,
            Self::NamespaceName(inner) => inner.span(),
        }
    }
}
#[derive(Debug, Clone)]
pub enum ScopedCallExpressionName<'tree> {
    DynamicVariableName(::std::boxed::Box<DynamicVariableName<'tree>>),
    Expression(::std::boxed::Box<Expression<'tree>>),
    Name(::std::boxed::Box<Name<'tree>>),
    VariableName(::std::boxed::Box<VariableName<'tree>>),
}
impl<'tree> ::treesitter_types::FromNode<'tree> for ScopedCallExpressionName<'tree> {
    #[allow(clippy::collapsible_else_if)]
    fn from_node(
        node: ::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        match node.kind() {
            "dynamic_variable_name" => Ok(Self::DynamicVariableName(::std::boxed::Box::new(
                <DynamicVariableName as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "name" => Ok(Self::Name(::std::boxed::Box::new(
                <Name as ::treesitter_types::FromNode>::from_node(node, src)?,
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
impl ::treesitter_types::Spanned for ScopedCallExpressionName<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        match self {
            Self::DynamicVariableName(inner) => inner.span(),
            Self::Expression(inner) => inner.span(),
            Self::Name(inner) => inner.span(),
            Self::VariableName(inner) => inner.span(),
        }
    }
}
#[derive(Debug, Clone)]
pub enum ScopedCallExpressionScope<'tree> {
    ArrayCreationExpression(::std::boxed::Box<ArrayCreationExpression<'tree>>),
    CastExpression(::std::boxed::Box<CastExpression<'tree>>),
    ClassConstantAccessExpression(::std::boxed::Box<ClassConstantAccessExpression<'tree>>),
    DynamicVariableName(::std::boxed::Box<DynamicVariableName<'tree>>),
    EncapsedString(::std::boxed::Box<EncapsedString<'tree>>),
    FunctionCallExpression(::std::boxed::Box<FunctionCallExpression<'tree>>),
    Heredoc(::std::boxed::Box<Heredoc<'tree>>),
    MemberAccessExpression(::std::boxed::Box<MemberAccessExpression<'tree>>),
    MemberCallExpression(::std::boxed::Box<MemberCallExpression<'tree>>),
    Name(::std::boxed::Box<Name<'tree>>),
    Nowdoc(::std::boxed::Box<Nowdoc<'tree>>),
    NullsafeMemberAccessExpression(::std::boxed::Box<NullsafeMemberAccessExpression<'tree>>),
    NullsafeMemberCallExpression(::std::boxed::Box<NullsafeMemberCallExpression<'tree>>),
    ObjectCreationExpression(::std::boxed::Box<ObjectCreationExpression<'tree>>),
    ParenthesizedExpression(::std::boxed::Box<ParenthesizedExpression<'tree>>),
    QualifiedName(::std::boxed::Box<QualifiedName<'tree>>),
    RelativeName(::std::boxed::Box<RelativeName<'tree>>),
    RelativeScope(::std::boxed::Box<RelativeScope<'tree>>),
    ScopedCallExpression(::std::boxed::Box<ScopedCallExpression<'tree>>),
    ScopedPropertyAccessExpression(::std::boxed::Box<ScopedPropertyAccessExpression<'tree>>),
    String(::std::boxed::Box<String<'tree>>),
    SubscriptExpression(::std::boxed::Box<SubscriptExpression<'tree>>),
    VariableName(::std::boxed::Box<VariableName<'tree>>),
}
impl<'tree> ::treesitter_types::FromNode<'tree> for ScopedCallExpressionScope<'tree> {
    #[allow(clippy::collapsible_else_if)]
    fn from_node(
        node: ::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        match node.kind() {
            "array_creation_expression" => {
                Ok(Self::ArrayCreationExpression(::std::boxed::Box::new(
                    <ArrayCreationExpression as ::treesitter_types::FromNode>::from_node(
                        node, src,
                    )?,
                )))
            }
            "cast_expression" => Ok(Self::CastExpression(::std::boxed::Box::new(
                <CastExpression as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "class_constant_access_expression" => {
                Ok(Self::ClassConstantAccessExpression(::std::boxed::Box::new(
                    <ClassConstantAccessExpression as ::treesitter_types::FromNode>::from_node(
                        node, src,
                    )?,
                )))
            }
            "dynamic_variable_name" => Ok(Self::DynamicVariableName(::std::boxed::Box::new(
                <DynamicVariableName as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "encapsed_string" => Ok(Self::EncapsedString(::std::boxed::Box::new(
                <EncapsedString as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "function_call_expression" => Ok(Self::FunctionCallExpression(::std::boxed::Box::new(
                <FunctionCallExpression as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "heredoc" => Ok(Self::Heredoc(::std::boxed::Box::new(
                <Heredoc as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "member_access_expression" => Ok(Self::MemberAccessExpression(::std::boxed::Box::new(
                <MemberAccessExpression as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "member_call_expression" => Ok(Self::MemberCallExpression(::std::boxed::Box::new(
                <MemberCallExpression as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "name" => Ok(Self::Name(::std::boxed::Box::new(
                <Name as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "nowdoc" => Ok(Self::Nowdoc(::std::boxed::Box::new(
                <Nowdoc as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "nullsafe_member_access_expression" => Ok(Self::NullsafeMemberAccessExpression(
                ::std::boxed::Box::new(
                    <NullsafeMemberAccessExpression as ::treesitter_types::FromNode>::from_node(
                        node, src,
                    )?,
                ),
            )),
            "nullsafe_member_call_expression" => {
                Ok(Self::NullsafeMemberCallExpression(::std::boxed::Box::new(
                    <NullsafeMemberCallExpression as ::treesitter_types::FromNode>::from_node(
                        node, src,
                    )?,
                )))
            }
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
            "qualified_name" => Ok(Self::QualifiedName(::std::boxed::Box::new(
                <QualifiedName as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "relative_name" => Ok(Self::RelativeName(::std::boxed::Box::new(
                <RelativeName as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "relative_scope" => Ok(Self::RelativeScope(::std::boxed::Box::new(
                <RelativeScope as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "scoped_call_expression" => Ok(Self::ScopedCallExpression(::std::boxed::Box::new(
                <ScopedCallExpression as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "scoped_property_access_expression" => Ok(Self::ScopedPropertyAccessExpression(
                ::std::boxed::Box::new(
                    <ScopedPropertyAccessExpression as ::treesitter_types::FromNode>::from_node(
                        node, src,
                    )?,
                ),
            )),
            "string" => Ok(Self::String(::std::boxed::Box::new(
                <String as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "subscript_expression" => Ok(Self::SubscriptExpression(::std::boxed::Box::new(
                <SubscriptExpression as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "variable_name" => Ok(Self::VariableName(::std::boxed::Box::new(
                <VariableName as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            other => Err(::treesitter_types::ParseError::unexpected_kind(other, node)),
        }
    }
}
impl ::treesitter_types::Spanned for ScopedCallExpressionScope<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        match self {
            Self::ArrayCreationExpression(inner) => inner.span(),
            Self::CastExpression(inner) => inner.span(),
            Self::ClassConstantAccessExpression(inner) => inner.span(),
            Self::DynamicVariableName(inner) => inner.span(),
            Self::EncapsedString(inner) => inner.span(),
            Self::FunctionCallExpression(inner) => inner.span(),
            Self::Heredoc(inner) => inner.span(),
            Self::MemberAccessExpression(inner) => inner.span(),
            Self::MemberCallExpression(inner) => inner.span(),
            Self::Name(inner) => inner.span(),
            Self::Nowdoc(inner) => inner.span(),
            Self::NullsafeMemberAccessExpression(inner) => inner.span(),
            Self::NullsafeMemberCallExpression(inner) => inner.span(),
            Self::ObjectCreationExpression(inner) => inner.span(),
            Self::ParenthesizedExpression(inner) => inner.span(),
            Self::QualifiedName(inner) => inner.span(),
            Self::RelativeName(inner) => inner.span(),
            Self::RelativeScope(inner) => inner.span(),
            Self::ScopedCallExpression(inner) => inner.span(),
            Self::ScopedPropertyAccessExpression(inner) => inner.span(),
            Self::String(inner) => inner.span(),
            Self::SubscriptExpression(inner) => inner.span(),
            Self::VariableName(inner) => inner.span(),
        }
    }
}
#[derive(Debug, Clone)]
pub enum ScopedPropertyAccessExpressionName<'tree> {
    DynamicVariableName(::std::boxed::Box<DynamicVariableName<'tree>>),
    VariableName(::std::boxed::Box<VariableName<'tree>>),
}
impl<'tree> ::treesitter_types::FromNode<'tree> for ScopedPropertyAccessExpressionName<'tree> {
    #[allow(clippy::collapsible_else_if)]
    fn from_node(
        node: ::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        match node.kind() {
            "dynamic_variable_name" => Ok(Self::DynamicVariableName(::std::boxed::Box::new(
                <DynamicVariableName as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "variable_name" => Ok(Self::VariableName(::std::boxed::Box::new(
                <VariableName as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            other => Err(::treesitter_types::ParseError::unexpected_kind(other, node)),
        }
    }
}
impl ::treesitter_types::Spanned for ScopedPropertyAccessExpressionName<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        match self {
            Self::DynamicVariableName(inner) => inner.span(),
            Self::VariableName(inner) => inner.span(),
        }
    }
}
#[derive(Debug, Clone)]
pub enum ScopedPropertyAccessExpressionScope<'tree> {
    ArrayCreationExpression(::std::boxed::Box<ArrayCreationExpression<'tree>>),
    CastExpression(::std::boxed::Box<CastExpression<'tree>>),
    ClassConstantAccessExpression(::std::boxed::Box<ClassConstantAccessExpression<'tree>>),
    DynamicVariableName(::std::boxed::Box<DynamicVariableName<'tree>>),
    EncapsedString(::std::boxed::Box<EncapsedString<'tree>>),
    FunctionCallExpression(::std::boxed::Box<FunctionCallExpression<'tree>>),
    Heredoc(::std::boxed::Box<Heredoc<'tree>>),
    MemberAccessExpression(::std::boxed::Box<MemberAccessExpression<'tree>>),
    MemberCallExpression(::std::boxed::Box<MemberCallExpression<'tree>>),
    Name(::std::boxed::Box<Name<'tree>>),
    Nowdoc(::std::boxed::Box<Nowdoc<'tree>>),
    NullsafeMemberAccessExpression(::std::boxed::Box<NullsafeMemberAccessExpression<'tree>>),
    NullsafeMemberCallExpression(::std::boxed::Box<NullsafeMemberCallExpression<'tree>>),
    ObjectCreationExpression(::std::boxed::Box<ObjectCreationExpression<'tree>>),
    ParenthesizedExpression(::std::boxed::Box<ParenthesizedExpression<'tree>>),
    QualifiedName(::std::boxed::Box<QualifiedName<'tree>>),
    RelativeName(::std::boxed::Box<RelativeName<'tree>>),
    RelativeScope(::std::boxed::Box<RelativeScope<'tree>>),
    ScopedCallExpression(::std::boxed::Box<ScopedCallExpression<'tree>>),
    ScopedPropertyAccessExpression(::std::boxed::Box<ScopedPropertyAccessExpression<'tree>>),
    String(::std::boxed::Box<String<'tree>>),
    SubscriptExpression(::std::boxed::Box<SubscriptExpression<'tree>>),
    VariableName(::std::boxed::Box<VariableName<'tree>>),
}
impl<'tree> ::treesitter_types::FromNode<'tree> for ScopedPropertyAccessExpressionScope<'tree> {
    #[allow(clippy::collapsible_else_if)]
    fn from_node(
        node: ::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        match node.kind() {
            "array_creation_expression" => {
                Ok(Self::ArrayCreationExpression(::std::boxed::Box::new(
                    <ArrayCreationExpression as ::treesitter_types::FromNode>::from_node(
                        node, src,
                    )?,
                )))
            }
            "cast_expression" => Ok(Self::CastExpression(::std::boxed::Box::new(
                <CastExpression as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "class_constant_access_expression" => {
                Ok(Self::ClassConstantAccessExpression(::std::boxed::Box::new(
                    <ClassConstantAccessExpression as ::treesitter_types::FromNode>::from_node(
                        node, src,
                    )?,
                )))
            }
            "dynamic_variable_name" => Ok(Self::DynamicVariableName(::std::boxed::Box::new(
                <DynamicVariableName as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "encapsed_string" => Ok(Self::EncapsedString(::std::boxed::Box::new(
                <EncapsedString as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "function_call_expression" => Ok(Self::FunctionCallExpression(::std::boxed::Box::new(
                <FunctionCallExpression as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "heredoc" => Ok(Self::Heredoc(::std::boxed::Box::new(
                <Heredoc as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "member_access_expression" => Ok(Self::MemberAccessExpression(::std::boxed::Box::new(
                <MemberAccessExpression as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "member_call_expression" => Ok(Self::MemberCallExpression(::std::boxed::Box::new(
                <MemberCallExpression as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "name" => Ok(Self::Name(::std::boxed::Box::new(
                <Name as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "nowdoc" => Ok(Self::Nowdoc(::std::boxed::Box::new(
                <Nowdoc as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "nullsafe_member_access_expression" => Ok(Self::NullsafeMemberAccessExpression(
                ::std::boxed::Box::new(
                    <NullsafeMemberAccessExpression as ::treesitter_types::FromNode>::from_node(
                        node, src,
                    )?,
                ),
            )),
            "nullsafe_member_call_expression" => {
                Ok(Self::NullsafeMemberCallExpression(::std::boxed::Box::new(
                    <NullsafeMemberCallExpression as ::treesitter_types::FromNode>::from_node(
                        node, src,
                    )?,
                )))
            }
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
            "qualified_name" => Ok(Self::QualifiedName(::std::boxed::Box::new(
                <QualifiedName as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "relative_name" => Ok(Self::RelativeName(::std::boxed::Box::new(
                <RelativeName as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "relative_scope" => Ok(Self::RelativeScope(::std::boxed::Box::new(
                <RelativeScope as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "scoped_call_expression" => Ok(Self::ScopedCallExpression(::std::boxed::Box::new(
                <ScopedCallExpression as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "scoped_property_access_expression" => Ok(Self::ScopedPropertyAccessExpression(
                ::std::boxed::Box::new(
                    <ScopedPropertyAccessExpression as ::treesitter_types::FromNode>::from_node(
                        node, src,
                    )?,
                ),
            )),
            "string" => Ok(Self::String(::std::boxed::Box::new(
                <String as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "subscript_expression" => Ok(Self::SubscriptExpression(::std::boxed::Box::new(
                <SubscriptExpression as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "variable_name" => Ok(Self::VariableName(::std::boxed::Box::new(
                <VariableName as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            other => Err(::treesitter_types::ParseError::unexpected_kind(other, node)),
        }
    }
}
impl ::treesitter_types::Spanned for ScopedPropertyAccessExpressionScope<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        match self {
            Self::ArrayCreationExpression(inner) => inner.span(),
            Self::CastExpression(inner) => inner.span(),
            Self::ClassConstantAccessExpression(inner) => inner.span(),
            Self::DynamicVariableName(inner) => inner.span(),
            Self::EncapsedString(inner) => inner.span(),
            Self::FunctionCallExpression(inner) => inner.span(),
            Self::Heredoc(inner) => inner.span(),
            Self::MemberAccessExpression(inner) => inner.span(),
            Self::MemberCallExpression(inner) => inner.span(),
            Self::Name(inner) => inner.span(),
            Self::Nowdoc(inner) => inner.span(),
            Self::NullsafeMemberAccessExpression(inner) => inner.span(),
            Self::NullsafeMemberCallExpression(inner) => inner.span(),
            Self::ObjectCreationExpression(inner) => inner.span(),
            Self::ParenthesizedExpression(inner) => inner.span(),
            Self::QualifiedName(inner) => inner.span(),
            Self::RelativeName(inner) => inner.span(),
            Self::RelativeScope(inner) => inner.span(),
            Self::ScopedCallExpression(inner) => inner.span(),
            Self::ScopedPropertyAccessExpression(inner) => inner.span(),
            Self::String(inner) => inner.span(),
            Self::SubscriptExpression(inner) => inner.span(),
            Self::VariableName(inner) => inner.span(),
        }
    }
}
#[derive(Debug, Clone)]
pub enum SequenceExpressionChildren<'tree> {
    Expression(::std::boxed::Box<Expression<'tree>>),
    SequenceExpression(::std::boxed::Box<SequenceExpression<'tree>>),
}
impl<'tree> ::treesitter_types::FromNode<'tree> for SequenceExpressionChildren<'tree> {
    #[allow(clippy::collapsible_else_if)]
    fn from_node(
        node: ::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        match node.kind() {
            "sequence_expression" => Ok(Self::SequenceExpression(::std::boxed::Box::new(
                <SequenceExpression as ::treesitter_types::FromNode>::from_node(node, src)?,
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
impl ::treesitter_types::Spanned for SequenceExpressionChildren<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        match self {
            Self::Expression(inner) => inner.span(),
            Self::SequenceExpression(inner) => inner.span(),
        }
    }
}
#[derive(Debug, Clone)]
pub enum ShellCommandExpressionChildren<'tree> {
    DynamicVariableName(::std::boxed::Box<DynamicVariableName<'tree>>),
    EscapeSequence(::std::boxed::Box<EscapeSequence<'tree>>),
    Expression(::std::boxed::Box<Expression<'tree>>),
    MemberAccessExpression(::std::boxed::Box<MemberAccessExpression<'tree>>),
    StringContent(::std::boxed::Box<StringContent<'tree>>),
    SubscriptExpression(::std::boxed::Box<SubscriptExpression<'tree>>),
    VariableName(::std::boxed::Box<VariableName<'tree>>),
}
impl<'tree> ::treesitter_types::FromNode<'tree> for ShellCommandExpressionChildren<'tree> {
    #[allow(clippy::collapsible_else_if)]
    fn from_node(
        node: ::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        match node.kind() {
            "dynamic_variable_name" => Ok(Self::DynamicVariableName(::std::boxed::Box::new(
                <DynamicVariableName as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "escape_sequence" => Ok(Self::EscapeSequence(::std::boxed::Box::new(
                <EscapeSequence as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "member_access_expression" => Ok(Self::MemberAccessExpression(::std::boxed::Box::new(
                <MemberAccessExpression as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "string_content" => Ok(Self::StringContent(::std::boxed::Box::new(
                <StringContent as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "subscript_expression" => Ok(Self::SubscriptExpression(::std::boxed::Box::new(
                <SubscriptExpression as ::treesitter_types::FromNode>::from_node(node, src)?,
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
impl ::treesitter_types::Spanned for ShellCommandExpressionChildren<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        match self {
            Self::DynamicVariableName(inner) => inner.span(),
            Self::EscapeSequence(inner) => inner.span(),
            Self::Expression(inner) => inner.span(),
            Self::MemberAccessExpression(inner) => inner.span(),
            Self::StringContent(inner) => inner.span(),
            Self::SubscriptExpression(inner) => inner.span(),
            Self::VariableName(inner) => inner.span(),
        }
    }
}
#[derive(Debug, Clone)]
pub enum StringChildren<'tree> {
    EscapeSequence(::std::boxed::Box<EscapeSequence<'tree>>),
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
                <EscapeSequence as ::treesitter_types::FromNode>::from_node(node, src)?,
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
            Self::EscapeSequence(inner) => inner.span(),
            Self::StringContent(inner) => inner.span(),
        }
    }
}
#[derive(Debug, Clone)]
pub enum SubscriptExpressionChildren<'tree> {
    ArrayCreationExpression(::std::boxed::Box<ArrayCreationExpression<'tree>>),
    ClassConstantAccessExpression(::std::boxed::Box<ClassConstantAccessExpression<'tree>>),
    DynamicVariableName(::std::boxed::Box<DynamicVariableName<'tree>>),
    EncapsedString(::std::boxed::Box<EncapsedString<'tree>>),
    Expression(::std::boxed::Box<Expression<'tree>>),
    FunctionCallExpression(::std::boxed::Box<FunctionCallExpression<'tree>>),
    Heredoc(::std::boxed::Box<Heredoc<'tree>>),
    Integer(::std::boxed::Box<Integer<'tree>>),
    MemberAccessExpression(::std::boxed::Box<MemberAccessExpression<'tree>>),
    MemberCallExpression(::std::boxed::Box<MemberCallExpression<'tree>>),
    Name(::std::boxed::Box<Name<'tree>>),
    Nowdoc(::std::boxed::Box<Nowdoc<'tree>>),
    NullsafeMemberAccessExpression(::std::boxed::Box<NullsafeMemberAccessExpression<'tree>>),
    NullsafeMemberCallExpression(::std::boxed::Box<NullsafeMemberCallExpression<'tree>>),
    ObjectCreationExpression(::std::boxed::Box<ObjectCreationExpression<'tree>>),
    ParenthesizedExpression(::std::boxed::Box<ParenthesizedExpression<'tree>>),
    QualifiedName(::std::boxed::Box<QualifiedName<'tree>>),
    RelativeName(::std::boxed::Box<RelativeName<'tree>>),
    ScopedCallExpression(::std::boxed::Box<ScopedCallExpression<'tree>>),
    ScopedPropertyAccessExpression(::std::boxed::Box<ScopedPropertyAccessExpression<'tree>>),
    String(::std::boxed::Box<String<'tree>>),
    SubscriptExpression(::std::boxed::Box<SubscriptExpression<'tree>>),
    VariableName(::std::boxed::Box<VariableName<'tree>>),
}
impl<'tree> ::treesitter_types::FromNode<'tree> for SubscriptExpressionChildren<'tree> {
    #[allow(clippy::collapsible_else_if)]
    fn from_node(
        node: ::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        match node.kind() {
            "array_creation_expression" => {
                Ok(Self::ArrayCreationExpression(::std::boxed::Box::new(
                    <ArrayCreationExpression as ::treesitter_types::FromNode>::from_node(
                        node, src,
                    )?,
                )))
            }
            "class_constant_access_expression" => {
                Ok(Self::ClassConstantAccessExpression(::std::boxed::Box::new(
                    <ClassConstantAccessExpression as ::treesitter_types::FromNode>::from_node(
                        node, src,
                    )?,
                )))
            }
            "dynamic_variable_name" => Ok(Self::DynamicVariableName(::std::boxed::Box::new(
                <DynamicVariableName as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "encapsed_string" => Ok(Self::EncapsedString(::std::boxed::Box::new(
                <EncapsedString as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "function_call_expression" => Ok(Self::FunctionCallExpression(::std::boxed::Box::new(
                <FunctionCallExpression as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "heredoc" => Ok(Self::Heredoc(::std::boxed::Box::new(
                <Heredoc as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "integer" => Ok(Self::Integer(::std::boxed::Box::new(
                <Integer as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "member_access_expression" => Ok(Self::MemberAccessExpression(::std::boxed::Box::new(
                <MemberAccessExpression as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "member_call_expression" => Ok(Self::MemberCallExpression(::std::boxed::Box::new(
                <MemberCallExpression as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "name" => Ok(Self::Name(::std::boxed::Box::new(
                <Name as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "nowdoc" => Ok(Self::Nowdoc(::std::boxed::Box::new(
                <Nowdoc as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "nullsafe_member_access_expression" => Ok(Self::NullsafeMemberAccessExpression(
                ::std::boxed::Box::new(
                    <NullsafeMemberAccessExpression as ::treesitter_types::FromNode>::from_node(
                        node, src,
                    )?,
                ),
            )),
            "nullsafe_member_call_expression" => {
                Ok(Self::NullsafeMemberCallExpression(::std::boxed::Box::new(
                    <NullsafeMemberCallExpression as ::treesitter_types::FromNode>::from_node(
                        node, src,
                    )?,
                )))
            }
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
            "qualified_name" => Ok(Self::QualifiedName(::std::boxed::Box::new(
                <QualifiedName as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "relative_name" => Ok(Self::RelativeName(::std::boxed::Box::new(
                <RelativeName as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "scoped_call_expression" => Ok(Self::ScopedCallExpression(::std::boxed::Box::new(
                <ScopedCallExpression as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "scoped_property_access_expression" => Ok(Self::ScopedPropertyAccessExpression(
                ::std::boxed::Box::new(
                    <ScopedPropertyAccessExpression as ::treesitter_types::FromNode>::from_node(
                        node, src,
                    )?,
                ),
            )),
            "string" => Ok(Self::String(::std::boxed::Box::new(
                <String as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "subscript_expression" => Ok(Self::SubscriptExpression(::std::boxed::Box::new(
                <SubscriptExpression as ::treesitter_types::FromNode>::from_node(node, src)?,
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
impl ::treesitter_types::Spanned for SubscriptExpressionChildren<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        match self {
            Self::ArrayCreationExpression(inner) => inner.span(),
            Self::ClassConstantAccessExpression(inner) => inner.span(),
            Self::DynamicVariableName(inner) => inner.span(),
            Self::EncapsedString(inner) => inner.span(),
            Self::Expression(inner) => inner.span(),
            Self::FunctionCallExpression(inner) => inner.span(),
            Self::Heredoc(inner) => inner.span(),
            Self::Integer(inner) => inner.span(),
            Self::MemberAccessExpression(inner) => inner.span(),
            Self::MemberCallExpression(inner) => inner.span(),
            Self::Name(inner) => inner.span(),
            Self::Nowdoc(inner) => inner.span(),
            Self::NullsafeMemberAccessExpression(inner) => inner.span(),
            Self::NullsafeMemberCallExpression(inner) => inner.span(),
            Self::ObjectCreationExpression(inner) => inner.span(),
            Self::ParenthesizedExpression(inner) => inner.span(),
            Self::QualifiedName(inner) => inner.span(),
            Self::RelativeName(inner) => inner.span(),
            Self::ScopedCallExpression(inner) => inner.span(),
            Self::ScopedPropertyAccessExpression(inner) => inner.span(),
            Self::String(inner) => inner.span(),
            Self::SubscriptExpression(inner) => inner.span(),
            Self::VariableName(inner) => inner.span(),
        }
    }
}
#[derive(Debug, Clone)]
pub enum SwitchBlockChildren<'tree> {
    CaseStatement(::std::boxed::Box<CaseStatement<'tree>>),
    DefaultStatement(::std::boxed::Box<DefaultStatement<'tree>>),
}
impl<'tree> ::treesitter_types::FromNode<'tree> for SwitchBlockChildren<'tree> {
    #[allow(clippy::collapsible_else_if)]
    fn from_node(
        node: ::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        match node.kind() {
            "case_statement" => Ok(Self::CaseStatement(::std::boxed::Box::new(
                <CaseStatement as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "default_statement" => Ok(Self::DefaultStatement(::std::boxed::Box::new(
                <DefaultStatement as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            other => Err(::treesitter_types::ParseError::unexpected_kind(other, node)),
        }
    }
}
impl ::treesitter_types::Spanned for SwitchBlockChildren<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        match self {
            Self::CaseStatement(inner) => inner.span(),
            Self::DefaultStatement(inner) => inner.span(),
        }
    }
}
#[derive(Debug, Clone)]
pub enum TextInterpolationChildren<'tree> {
    PhpEndTag(::std::boxed::Box<PhpEndTag<'tree>>),
    PhpTag(::std::boxed::Box<PhpTag<'tree>>),
    Text(::std::boxed::Box<Text<'tree>>),
}
impl<'tree> ::treesitter_types::FromNode<'tree> for TextInterpolationChildren<'tree> {
    #[allow(clippy::collapsible_else_if)]
    fn from_node(
        node: ::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        match node.kind() {
            "php_end_tag" => Ok(Self::PhpEndTag(::std::boxed::Box::new(
                <PhpEndTag as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "php_tag" => Ok(Self::PhpTag(::std::boxed::Box::new(
                <PhpTag as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "text" => Ok(Self::Text(::std::boxed::Box::new(
                <Text as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            other => Err(::treesitter_types::ParseError::unexpected_kind(other, node)),
        }
    }
}
impl ::treesitter_types::Spanned for TextInterpolationChildren<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        match self {
            Self::PhpEndTag(inner) => inner.span(),
            Self::PhpTag(inner) => inner.span(),
            Self::Text(inner) => inner.span(),
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
pub enum UnaryOpExpressionOperator {
    Bang(::treesitter_types::Span),
    Plus(::treesitter_types::Span),
    Minus(::treesitter_types::Span),
    Tilde(::treesitter_types::Span),
}
impl<'tree> ::treesitter_types::FromNode<'tree> for UnaryOpExpressionOperator {
    #[allow(clippy::collapsible_else_if)]
    fn from_node(
        node: ::tree_sitter::Node<'tree>,
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
impl ::treesitter_types::Spanned for UnaryOpExpressionOperator {
    fn span(&self) -> ::treesitter_types::Span {
        match self {
            Self::Bang(span) => *span,
            Self::Plus(span) => *span,
            Self::Minus(span) => *span,
            Self::Tilde(span) => *span,
        }
    }
}
#[derive(Debug, Clone)]
pub enum UnionTypeChildren<'tree> {
    NamedType(::std::boxed::Box<NamedType<'tree>>),
    OptionalType(::std::boxed::Box<OptionalType<'tree>>),
    PrimitiveType(::std::boxed::Box<PrimitiveType<'tree>>),
}
impl<'tree> ::treesitter_types::FromNode<'tree> for UnionTypeChildren<'tree> {
    #[allow(clippy::collapsible_else_if)]
    fn from_node(
        node: ::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        match node.kind() {
            "named_type" => Ok(Self::NamedType(::std::boxed::Box::new(
                <NamedType as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "optional_type" => Ok(Self::OptionalType(::std::boxed::Box::new(
                <OptionalType as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "primitive_type" => Ok(Self::PrimitiveType(::std::boxed::Box::new(
                <PrimitiveType as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            other => Err(::treesitter_types::ParseError::unexpected_kind(other, node)),
        }
    }
}
impl ::treesitter_types::Spanned for UnionTypeChildren<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        match self {
            Self::NamedType(inner) => inner.span(),
            Self::OptionalType(inner) => inner.span(),
            Self::PrimitiveType(inner) => inner.span(),
        }
    }
}
#[derive(Debug, Clone)]
pub enum UnsetStatementChildren<'tree> {
    CastExpression(::std::boxed::Box<CastExpression<'tree>>),
    DynamicVariableName(::std::boxed::Box<DynamicVariableName<'tree>>),
    FunctionCallExpression(::std::boxed::Box<FunctionCallExpression<'tree>>),
    MemberAccessExpression(::std::boxed::Box<MemberAccessExpression<'tree>>),
    MemberCallExpression(::std::boxed::Box<MemberCallExpression<'tree>>),
    NullsafeMemberAccessExpression(::std::boxed::Box<NullsafeMemberAccessExpression<'tree>>),
    NullsafeMemberCallExpression(::std::boxed::Box<NullsafeMemberCallExpression<'tree>>),
    ScopedCallExpression(::std::boxed::Box<ScopedCallExpression<'tree>>),
    ScopedPropertyAccessExpression(::std::boxed::Box<ScopedPropertyAccessExpression<'tree>>),
    SubscriptExpression(::std::boxed::Box<SubscriptExpression<'tree>>),
    VariableName(::std::boxed::Box<VariableName<'tree>>),
}
impl<'tree> ::treesitter_types::FromNode<'tree> for UnsetStatementChildren<'tree> {
    #[allow(clippy::collapsible_else_if)]
    fn from_node(
        node: ::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        match node.kind() {
            "cast_expression" => Ok(Self::CastExpression(::std::boxed::Box::new(
                <CastExpression as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "dynamic_variable_name" => Ok(Self::DynamicVariableName(::std::boxed::Box::new(
                <DynamicVariableName as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "function_call_expression" => Ok(Self::FunctionCallExpression(::std::boxed::Box::new(
                <FunctionCallExpression as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "member_access_expression" => Ok(Self::MemberAccessExpression(::std::boxed::Box::new(
                <MemberAccessExpression as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "member_call_expression" => Ok(Self::MemberCallExpression(::std::boxed::Box::new(
                <MemberCallExpression as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "nullsafe_member_access_expression" => Ok(Self::NullsafeMemberAccessExpression(
                ::std::boxed::Box::new(
                    <NullsafeMemberAccessExpression as ::treesitter_types::FromNode>::from_node(
                        node, src,
                    )?,
                ),
            )),
            "nullsafe_member_call_expression" => {
                Ok(Self::NullsafeMemberCallExpression(::std::boxed::Box::new(
                    <NullsafeMemberCallExpression as ::treesitter_types::FromNode>::from_node(
                        node, src,
                    )?,
                )))
            }
            "scoped_call_expression" => Ok(Self::ScopedCallExpression(::std::boxed::Box::new(
                <ScopedCallExpression as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "scoped_property_access_expression" => Ok(Self::ScopedPropertyAccessExpression(
                ::std::boxed::Box::new(
                    <ScopedPropertyAccessExpression as ::treesitter_types::FromNode>::from_node(
                        node, src,
                    )?,
                ),
            )),
            "subscript_expression" => Ok(Self::SubscriptExpression(::std::boxed::Box::new(
                <SubscriptExpression as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "variable_name" => Ok(Self::VariableName(::std::boxed::Box::new(
                <VariableName as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            other => Err(::treesitter_types::ParseError::unexpected_kind(other, node)),
        }
    }
}
impl ::treesitter_types::Spanned for UnsetStatementChildren<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        match self {
            Self::CastExpression(inner) => inner.span(),
            Self::DynamicVariableName(inner) => inner.span(),
            Self::FunctionCallExpression(inner) => inner.span(),
            Self::MemberAccessExpression(inner) => inner.span(),
            Self::MemberCallExpression(inner) => inner.span(),
            Self::NullsafeMemberAccessExpression(inner) => inner.span(),
            Self::NullsafeMemberCallExpression(inner) => inner.span(),
            Self::ScopedCallExpression(inner) => inner.span(),
            Self::ScopedPropertyAccessExpression(inner) => inner.span(),
            Self::SubscriptExpression(inner) => inner.span(),
            Self::VariableName(inner) => inner.span(),
        }
    }
}
#[derive(Debug, Clone)]
pub enum UpdateExpressionArgument<'tree> {
    CastExpression(::std::boxed::Box<CastExpression<'tree>>),
    DynamicVariableName(::std::boxed::Box<DynamicVariableName<'tree>>),
    FunctionCallExpression(::std::boxed::Box<FunctionCallExpression<'tree>>),
    MemberAccessExpression(::std::boxed::Box<MemberAccessExpression<'tree>>),
    MemberCallExpression(::std::boxed::Box<MemberCallExpression<'tree>>),
    NullsafeMemberAccessExpression(::std::boxed::Box<NullsafeMemberAccessExpression<'tree>>),
    NullsafeMemberCallExpression(::std::boxed::Box<NullsafeMemberCallExpression<'tree>>),
    ScopedCallExpression(::std::boxed::Box<ScopedCallExpression<'tree>>),
    ScopedPropertyAccessExpression(::std::boxed::Box<ScopedPropertyAccessExpression<'tree>>),
    SubscriptExpression(::std::boxed::Box<SubscriptExpression<'tree>>),
    VariableName(::std::boxed::Box<VariableName<'tree>>),
}
impl<'tree> ::treesitter_types::FromNode<'tree> for UpdateExpressionArgument<'tree> {
    #[allow(clippy::collapsible_else_if)]
    fn from_node(
        node: ::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        match node.kind() {
            "cast_expression" => Ok(Self::CastExpression(::std::boxed::Box::new(
                <CastExpression as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "dynamic_variable_name" => Ok(Self::DynamicVariableName(::std::boxed::Box::new(
                <DynamicVariableName as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "function_call_expression" => Ok(Self::FunctionCallExpression(::std::boxed::Box::new(
                <FunctionCallExpression as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "member_access_expression" => Ok(Self::MemberAccessExpression(::std::boxed::Box::new(
                <MemberAccessExpression as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "member_call_expression" => Ok(Self::MemberCallExpression(::std::boxed::Box::new(
                <MemberCallExpression as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "nullsafe_member_access_expression" => Ok(Self::NullsafeMemberAccessExpression(
                ::std::boxed::Box::new(
                    <NullsafeMemberAccessExpression as ::treesitter_types::FromNode>::from_node(
                        node, src,
                    )?,
                ),
            )),
            "nullsafe_member_call_expression" => {
                Ok(Self::NullsafeMemberCallExpression(::std::boxed::Box::new(
                    <NullsafeMemberCallExpression as ::treesitter_types::FromNode>::from_node(
                        node, src,
                    )?,
                )))
            }
            "scoped_call_expression" => Ok(Self::ScopedCallExpression(::std::boxed::Box::new(
                <ScopedCallExpression as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "scoped_property_access_expression" => Ok(Self::ScopedPropertyAccessExpression(
                ::std::boxed::Box::new(
                    <ScopedPropertyAccessExpression as ::treesitter_types::FromNode>::from_node(
                        node, src,
                    )?,
                ),
            )),
            "subscript_expression" => Ok(Self::SubscriptExpression(::std::boxed::Box::new(
                <SubscriptExpression as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "variable_name" => Ok(Self::VariableName(::std::boxed::Box::new(
                <VariableName as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            other => Err(::treesitter_types::ParseError::unexpected_kind(other, node)),
        }
    }
}
impl ::treesitter_types::Spanned for UpdateExpressionArgument<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        match self {
            Self::CastExpression(inner) => inner.span(),
            Self::DynamicVariableName(inner) => inner.span(),
            Self::FunctionCallExpression(inner) => inner.span(),
            Self::MemberAccessExpression(inner) => inner.span(),
            Self::MemberCallExpression(inner) => inner.span(),
            Self::NullsafeMemberAccessExpression(inner) => inner.span(),
            Self::NullsafeMemberCallExpression(inner) => inner.span(),
            Self::ScopedCallExpression(inner) => inner.span(),
            Self::ScopedPropertyAccessExpression(inner) => inner.span(),
            Self::SubscriptExpression(inner) => inner.span(),
            Self::VariableName(inner) => inner.span(),
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
pub enum UseAsClauseChildren<'tree> {
    ClassConstantAccessExpression(::std::boxed::Box<ClassConstantAccessExpression<'tree>>),
    Name(::std::boxed::Box<Name<'tree>>),
    VisibilityModifier(::std::boxed::Box<VisibilityModifier<'tree>>),
}
impl<'tree> ::treesitter_types::FromNode<'tree> for UseAsClauseChildren<'tree> {
    #[allow(clippy::collapsible_else_if)]
    fn from_node(
        node: ::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        match node.kind() {
            "class_constant_access_expression" => {
                Ok(Self::ClassConstantAccessExpression(::std::boxed::Box::new(
                    <ClassConstantAccessExpression as ::treesitter_types::FromNode>::from_node(
                        node, src,
                    )?,
                )))
            }
            "name" => Ok(Self::Name(::std::boxed::Box::new(
                <Name as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "visibility_modifier" => Ok(Self::VisibilityModifier(::std::boxed::Box::new(
                <VisibilityModifier as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            other => Err(::treesitter_types::ParseError::unexpected_kind(other, node)),
        }
    }
}
impl ::treesitter_types::Spanned for UseAsClauseChildren<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        match self {
            Self::ClassConstantAccessExpression(inner) => inner.span(),
            Self::Name(inner) => inner.span(),
            Self::VisibilityModifier(inner) => inner.span(),
        }
    }
}
#[derive(Debug, Clone)]
pub enum UseDeclarationChildren<'tree> {
    Name(::std::boxed::Box<Name<'tree>>),
    QualifiedName(::std::boxed::Box<QualifiedName<'tree>>),
    RelativeName(::std::boxed::Box<RelativeName<'tree>>),
    UseList(::std::boxed::Box<UseList<'tree>>),
}
impl<'tree> ::treesitter_types::FromNode<'tree> for UseDeclarationChildren<'tree> {
    #[allow(clippy::collapsible_else_if)]
    fn from_node(
        node: ::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        match node.kind() {
            "name" => Ok(Self::Name(::std::boxed::Box::new(
                <Name as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "qualified_name" => Ok(Self::QualifiedName(::std::boxed::Box::new(
                <QualifiedName as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "relative_name" => Ok(Self::RelativeName(::std::boxed::Box::new(
                <RelativeName as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "use_list" => Ok(Self::UseList(::std::boxed::Box::new(
                <UseList as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            other => Err(::treesitter_types::ParseError::unexpected_kind(other, node)),
        }
    }
}
impl ::treesitter_types::Spanned for UseDeclarationChildren<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        match self {
            Self::Name(inner) => inner.span(),
            Self::QualifiedName(inner) => inner.span(),
            Self::RelativeName(inner) => inner.span(),
            Self::UseList(inner) => inner.span(),
        }
    }
}
#[derive(Debug, Clone)]
pub enum UseInsteadOfClauseChildren<'tree> {
    ClassConstantAccessExpression(::std::boxed::Box<ClassConstantAccessExpression<'tree>>),
    Name(::std::boxed::Box<Name<'tree>>),
}
impl<'tree> ::treesitter_types::FromNode<'tree> for UseInsteadOfClauseChildren<'tree> {
    #[allow(clippy::collapsible_else_if)]
    fn from_node(
        node: ::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        match node.kind() {
            "class_constant_access_expression" => {
                Ok(Self::ClassConstantAccessExpression(::std::boxed::Box::new(
                    <ClassConstantAccessExpression as ::treesitter_types::FromNode>::from_node(
                        node, src,
                    )?,
                )))
            }
            "name" => Ok(Self::Name(::std::boxed::Box::new(
                <Name as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            other => Err(::treesitter_types::ParseError::unexpected_kind(other, node)),
        }
    }
}
impl ::treesitter_types::Spanned for UseInsteadOfClauseChildren<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        match self {
            Self::ClassConstantAccessExpression(inner) => inner.span(),
            Self::Name(inner) => inner.span(),
        }
    }
}
#[derive(Debug, Clone)]
pub enum UseListChildren<'tree> {
    UseAsClause(::std::boxed::Box<UseAsClause<'tree>>),
    UseInsteadOfClause(::std::boxed::Box<UseInsteadOfClause<'tree>>),
}
impl<'tree> ::treesitter_types::FromNode<'tree> for UseListChildren<'tree> {
    #[allow(clippy::collapsible_else_if)]
    fn from_node(
        node: ::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        match node.kind() {
            "use_as_clause" => Ok(Self::UseAsClause(::std::boxed::Box::new(
                <UseAsClause as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            "use_instead_of_clause" => Ok(Self::UseInsteadOfClause(::std::boxed::Box::new(
                <UseInsteadOfClause as ::treesitter_types::FromNode>::from_node(node, src)?,
            ))),
            other => Err(::treesitter_types::ParseError::unexpected_kind(other, node)),
        }
    }
}
impl ::treesitter_types::Spanned for UseListChildren<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        match self {
            Self::UseAsClause(inner) => inner.span(),
            Self::UseInsteadOfClause(inner) => inner.span(),
        }
    }
}
#[derive(Debug, Clone)]
pub enum WhileStatementBody<'tree> {
    ColonBlock(::std::boxed::Box<ColonBlock<'tree>>),
    Statement(::std::boxed::Box<Statement<'tree>>),
}
impl<'tree> ::treesitter_types::FromNode<'tree> for WhileStatementBody<'tree> {
    #[allow(clippy::collapsible_else_if)]
    fn from_node(
        node: ::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        match node.kind() {
            "colon_block" => Ok(Self::ColonBlock(::std::boxed::Box::new(
                <ColonBlock as ::treesitter_types::FromNode>::from_node(node, src)?,
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
impl ::treesitter_types::Spanned for WhileStatementBody<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        match self {
            Self::ColonBlock(inner) => inner.span(),
            Self::Statement(inner) => inner.span(),
        }
    }
}
#[derive(Debug, Clone)]
pub enum YieldExpressionChildren<'tree> {
    ArrayElementInitializer(::std::boxed::Box<ArrayElementInitializer<'tree>>),
    Expression(::std::boxed::Box<Expression<'tree>>),
}
impl<'tree> ::treesitter_types::FromNode<'tree> for YieldExpressionChildren<'tree> {
    #[allow(clippy::collapsible_else_if)]
    fn from_node(
        node: ::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        match node.kind() {
            "array_element_initializer" => {
                Ok(Self::ArrayElementInitializer(::std::boxed::Box::new(
                    <ArrayElementInitializer as ::treesitter_types::FromNode>::from_node(
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
impl ::treesitter_types::Spanned for YieldExpressionChildren<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        match self {
            Self::ArrayElementInitializer(inner) => inner.span(),
            Self::Expression(inner) => inner.span(),
        }
    }
}
#[derive(Debug, Clone)]
pub enum AnyNode<'tree> {
    Expression(Expression<'tree>),
    Literal(Literal<'tree>),
    PrimaryExpression(PrimaryExpression<'tree>),
    Statement(Statement<'tree>),
    Type(Type<'tree>),
    AbstractModifier(AbstractModifier<'tree>),
    AnonymousClass(AnonymousClass<'tree>),
    AnonymousFunction(AnonymousFunction<'tree>),
    AnonymousFunctionUseClause(AnonymousFunctionUseClause<'tree>),
    Argument(Argument<'tree>),
    Arguments(Arguments<'tree>),
    ArrayCreationExpression(ArrayCreationExpression<'tree>),
    ArrayElementInitializer(ArrayElementInitializer<'tree>),
    ArrowFunction(ArrowFunction<'tree>),
    AssignmentExpression(AssignmentExpression<'tree>),
    Attribute(Attribute<'tree>),
    AttributeGroup(AttributeGroup<'tree>),
    AttributeList(AttributeList<'tree>),
    AugmentedAssignmentExpression(AugmentedAssignmentExpression<'tree>),
    BaseClause(BaseClause<'tree>),
    BinaryExpression(BinaryExpression<'tree>),
    Boolean(Boolean<'tree>),
    BreakStatement(BreakStatement<'tree>),
    ByRef(ByRef<'tree>),
    CaseStatement(CaseStatement<'tree>),
    CastExpression(CastExpression<'tree>),
    CastType(CastType<'tree>),
    CatchClause(CatchClause<'tree>),
    ClassConstantAccessExpression(ClassConstantAccessExpression<'tree>),
    ClassDeclaration(ClassDeclaration<'tree>),
    ClassInterfaceClause(ClassInterfaceClause<'tree>),
    CloneExpression(CloneExpression<'tree>),
    ColonBlock(ColonBlock<'tree>),
    CompoundStatement(CompoundStatement<'tree>),
    ConditionalExpression(ConditionalExpression<'tree>),
    ConstDeclaration(ConstDeclaration<'tree>),
    ConstElement(ConstElement<'tree>),
    ContinueStatement(ContinueStatement<'tree>),
    DeclarationList(DeclarationList<'tree>),
    DeclareDirective(DeclareDirective<'tree>),
    DeclareStatement(DeclareStatement<'tree>),
    DefaultStatement(DefaultStatement<'tree>),
    DisjunctiveNormalFormType(DisjunctiveNormalFormType<'tree>),
    DoStatement(DoStatement<'tree>),
    DynamicVariableName(DynamicVariableName<'tree>),
    EchoStatement(EchoStatement<'tree>),
    ElseClause(ElseClause<'tree>),
    ElseIfClause(ElseIfClause<'tree>),
    EmptyStatement(EmptyStatement<'tree>),
    EncapsedString(EncapsedString<'tree>),
    EnumCase(EnumCase<'tree>),
    EnumDeclaration(EnumDeclaration<'tree>),
    EnumDeclarationList(EnumDeclarationList<'tree>),
    ErrorSuppressionExpression(ErrorSuppressionExpression<'tree>),
    ExitStatement(ExitStatement<'tree>),
    ExpressionStatement(ExpressionStatement<'tree>),
    FinalModifier(FinalModifier<'tree>),
    FinallyClause(FinallyClause<'tree>),
    ForStatement(ForStatement<'tree>),
    ForeachStatement(ForeachStatement<'tree>),
    FormalParameters(FormalParameters<'tree>),
    FunctionCallExpression(FunctionCallExpression<'tree>),
    FunctionDefinition(FunctionDefinition<'tree>),
    FunctionStaticDeclaration(FunctionStaticDeclaration<'tree>),
    GlobalDeclaration(GlobalDeclaration<'tree>),
    GotoStatement(GotoStatement<'tree>),
    Heredoc(Heredoc<'tree>),
    HeredocBody(HeredocBody<'tree>),
    IfStatement(IfStatement<'tree>),
    IncludeExpression(IncludeExpression<'tree>),
    IncludeOnceExpression(IncludeOnceExpression<'tree>),
    InterfaceDeclaration(InterfaceDeclaration<'tree>),
    IntersectionType(IntersectionType<'tree>),
    ListLiteral(ListLiteral<'tree>),
    MatchBlock(MatchBlock<'tree>),
    MatchConditionList(MatchConditionList<'tree>),
    MatchConditionalExpression(MatchConditionalExpression<'tree>),
    MatchDefaultExpression(MatchDefaultExpression<'tree>),
    MatchExpression(MatchExpression<'tree>),
    MemberAccessExpression(MemberAccessExpression<'tree>),
    MemberCallExpression(MemberCallExpression<'tree>),
    MethodDeclaration(MethodDeclaration<'tree>),
    Name(Name<'tree>),
    NamedLabelStatement(NamedLabelStatement<'tree>),
    NamedType(NamedType<'tree>),
    NamespaceDefinition(NamespaceDefinition<'tree>),
    NamespaceName(NamespaceName<'tree>),
    NamespaceUseClause(NamespaceUseClause<'tree>),
    NamespaceUseDeclaration(NamespaceUseDeclaration<'tree>),
    NamespaceUseGroup(NamespaceUseGroup<'tree>),
    Nowdoc(Nowdoc<'tree>),
    NowdocBody(NowdocBody<'tree>),
    Null(Null<'tree>),
    NullsafeMemberAccessExpression(NullsafeMemberAccessExpression<'tree>),
    NullsafeMemberCallExpression(NullsafeMemberCallExpression<'tree>),
    ObjectCreationExpression(ObjectCreationExpression<'tree>),
    OptionalType(OptionalType<'tree>),
    Pair(Pair<'tree>),
    ParenthesizedExpression(ParenthesizedExpression<'tree>),
    PrimitiveType(PrimitiveType<'tree>),
    PrintIntrinsic(PrintIntrinsic<'tree>),
    Program(Program<'tree>),
    PropertyDeclaration(PropertyDeclaration<'tree>),
    PropertyElement(PropertyElement<'tree>),
    PropertyHook(PropertyHook<'tree>),
    PropertyHookList(PropertyHookList<'tree>),
    PropertyPromotionParameter(PropertyPromotionParameter<'tree>),
    QualifiedName(QualifiedName<'tree>),
    ReadonlyModifier(ReadonlyModifier<'tree>),
    ReferenceAssignmentExpression(ReferenceAssignmentExpression<'tree>),
    ReferenceModifier(ReferenceModifier<'tree>),
    RelativeName(RelativeName<'tree>),
    RelativeScope(RelativeScope<'tree>),
    RequireExpression(RequireExpression<'tree>),
    RequireOnceExpression(RequireOnceExpression<'tree>),
    ReturnStatement(ReturnStatement<'tree>),
    ScopedCallExpression(ScopedCallExpression<'tree>),
    ScopedPropertyAccessExpression(ScopedPropertyAccessExpression<'tree>),
    SequenceExpression(SequenceExpression<'tree>),
    ShellCommandExpression(ShellCommandExpression<'tree>),
    SimpleParameter(SimpleParameter<'tree>),
    StaticModifier(StaticModifier<'tree>),
    StaticVariableDeclaration(StaticVariableDeclaration<'tree>),
    String(String<'tree>),
    StringContent(StringContent<'tree>),
    SubscriptExpression(SubscriptExpression<'tree>),
    SwitchBlock(SwitchBlock<'tree>),
    SwitchStatement(SwitchStatement<'tree>),
    Text(Text<'tree>),
    TextInterpolation(TextInterpolation<'tree>),
    ThrowExpression(ThrowExpression<'tree>),
    TraitDeclaration(TraitDeclaration<'tree>),
    TryStatement(TryStatement<'tree>),
    TypeList(TypeList<'tree>),
    UnaryOpExpression(UnaryOpExpression<'tree>),
    UnionType(UnionType<'tree>),
    UnsetStatement(UnsetStatement<'tree>),
    UpdateExpression(UpdateExpression<'tree>),
    UseAsClause(UseAsClause<'tree>),
    UseDeclaration(UseDeclaration<'tree>),
    UseInsteadOfClause(UseInsteadOfClause<'tree>),
    UseList(UseList<'tree>),
    VariableName(VariableName<'tree>),
    VariadicParameter(VariadicParameter<'tree>),
    VariadicPlaceholder(VariadicPlaceholder<'tree>),
    VariadicUnpacking(VariadicUnpacking<'tree>),
    VisibilityModifier(VisibilityModifier<'tree>),
    WhileStatement(WhileStatement<'tree>),
    YieldExpression(YieldExpression<'tree>),
    BottomType(BottomType<'tree>),
    Comment(Comment<'tree>),
    EscapeSequence(EscapeSequence<'tree>),
    Float(Float<'tree>),
    HeredocEnd(HeredocEnd<'tree>),
    HeredocStart(HeredocStart<'tree>),
    Integer(Integer<'tree>),
    NowdocString(NowdocString<'tree>),
    Operation(Operation<'tree>),
    PhpEndTag(PhpEndTag<'tree>),
    PhpTag(PhpTag<'tree>),
    VarModifier(VarModifier<'tree>),
    Unknown(::tree_sitter::Node<'tree>),
}
impl<'tree> AnyNode<'tree> {
    pub fn from_node(node: ::tree_sitter::Node<'tree>, src: &'tree [u8]) -> Self {
        match node.kind() {
            "expression" => <Expression as ::treesitter_types::FromNode>::from_node(node, src)
                .map(Self::Expression)
                .unwrap_or(Self::Unknown(node)),
            "literal" => <Literal as ::treesitter_types::FromNode>::from_node(node, src)
                .map(Self::Literal)
                .unwrap_or(Self::Unknown(node)),
            "primary_expression" => {
                <PrimaryExpression as ::treesitter_types::FromNode>::from_node(node, src)
                    .map(Self::PrimaryExpression)
                    .unwrap_or(Self::Unknown(node))
            }
            "statement" => <Statement as ::treesitter_types::FromNode>::from_node(node, src)
                .map(Self::Statement)
                .unwrap_or(Self::Unknown(node)),
            "type" => <Type as ::treesitter_types::FromNode>::from_node(node, src)
                .map(Self::Type)
                .unwrap_or(Self::Unknown(node)),
            "abstract_modifier" => {
                <AbstractModifier as ::treesitter_types::FromNode>::from_node(node, src)
                    .map(Self::AbstractModifier)
                    .unwrap_or(Self::Unknown(node))
            }
            "anonymous_class" => {
                <AnonymousClass as ::treesitter_types::FromNode>::from_node(node, src)
                    .map(Self::AnonymousClass)
                    .unwrap_or(Self::Unknown(node))
            }
            "anonymous_function" => {
                <AnonymousFunction as ::treesitter_types::FromNode>::from_node(node, src)
                    .map(Self::AnonymousFunction)
                    .unwrap_or(Self::Unknown(node))
            }
            "anonymous_function_use_clause" => {
                <AnonymousFunctionUseClause as ::treesitter_types::FromNode>::from_node(node, src)
                    .map(Self::AnonymousFunctionUseClause)
                    .unwrap_or(Self::Unknown(node))
            }
            "argument" => <Argument as ::treesitter_types::FromNode>::from_node(node, src)
                .map(Self::Argument)
                .unwrap_or(Self::Unknown(node)),
            "arguments" => <Arguments as ::treesitter_types::FromNode>::from_node(node, src)
                .map(Self::Arguments)
                .unwrap_or(Self::Unknown(node)),
            "array_creation_expression" => {
                <ArrayCreationExpression as ::treesitter_types::FromNode>::from_node(node, src)
                    .map(Self::ArrayCreationExpression)
                    .unwrap_or(Self::Unknown(node))
            }
            "array_element_initializer" => {
                <ArrayElementInitializer as ::treesitter_types::FromNode>::from_node(node, src)
                    .map(Self::ArrayElementInitializer)
                    .unwrap_or(Self::Unknown(node))
            }
            "arrow_function" => {
                <ArrowFunction as ::treesitter_types::FromNode>::from_node(node, src)
                    .map(Self::ArrowFunction)
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
            "attribute_group" => {
                <AttributeGroup as ::treesitter_types::FromNode>::from_node(node, src)
                    .map(Self::AttributeGroup)
                    .unwrap_or(Self::Unknown(node))
            }
            "attribute_list" => {
                <AttributeList as ::treesitter_types::FromNode>::from_node(node, src)
                    .map(Self::AttributeList)
                    .unwrap_or(Self::Unknown(node))
            }
            "augmented_assignment_expression" => {
                <AugmentedAssignmentExpression as ::treesitter_types::FromNode>::from_node(
                    node, src,
                )
                .map(Self::AugmentedAssignmentExpression)
                .unwrap_or(Self::Unknown(node))
            }
            "base_clause" => <BaseClause as ::treesitter_types::FromNode>::from_node(node, src)
                .map(Self::BaseClause)
                .unwrap_or(Self::Unknown(node)),
            "binary_expression" => {
                <BinaryExpression as ::treesitter_types::FromNode>::from_node(node, src)
                    .map(Self::BinaryExpression)
                    .unwrap_or(Self::Unknown(node))
            }
            "boolean" => <Boolean as ::treesitter_types::FromNode>::from_node(node, src)
                .map(Self::Boolean)
                .unwrap_or(Self::Unknown(node)),
            "break_statement" => {
                <BreakStatement as ::treesitter_types::FromNode>::from_node(node, src)
                    .map(Self::BreakStatement)
                    .unwrap_or(Self::Unknown(node))
            }
            "by_ref" => <ByRef as ::treesitter_types::FromNode>::from_node(node, src)
                .map(Self::ByRef)
                .unwrap_or(Self::Unknown(node)),
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
            "cast_type" => <CastType as ::treesitter_types::FromNode>::from_node(node, src)
                .map(Self::CastType)
                .unwrap_or(Self::Unknown(node)),
            "catch_clause" => <CatchClause as ::treesitter_types::FromNode>::from_node(node, src)
                .map(Self::CatchClause)
                .unwrap_or(Self::Unknown(node)),
            "class_constant_access_expression" => {
                <ClassConstantAccessExpression as ::treesitter_types::FromNode>::from_node(
                    node, src,
                )
                .map(Self::ClassConstantAccessExpression)
                .unwrap_or(Self::Unknown(node))
            }
            "class_declaration" => {
                <ClassDeclaration as ::treesitter_types::FromNode>::from_node(node, src)
                    .map(Self::ClassDeclaration)
                    .unwrap_or(Self::Unknown(node))
            }
            "class_interface_clause" => {
                <ClassInterfaceClause as ::treesitter_types::FromNode>::from_node(node, src)
                    .map(Self::ClassInterfaceClause)
                    .unwrap_or(Self::Unknown(node))
            }
            "clone_expression" => {
                <CloneExpression as ::treesitter_types::FromNode>::from_node(node, src)
                    .map(Self::CloneExpression)
                    .unwrap_or(Self::Unknown(node))
            }
            "colon_block" => <ColonBlock as ::treesitter_types::FromNode>::from_node(node, src)
                .map(Self::ColonBlock)
                .unwrap_or(Self::Unknown(node)),
            "compound_statement" => {
                <CompoundStatement as ::treesitter_types::FromNode>::from_node(node, src)
                    .map(Self::CompoundStatement)
                    .unwrap_or(Self::Unknown(node))
            }
            "conditional_expression" => {
                <ConditionalExpression as ::treesitter_types::FromNode>::from_node(node, src)
                    .map(Self::ConditionalExpression)
                    .unwrap_or(Self::Unknown(node))
            }
            "const_declaration" => {
                <ConstDeclaration as ::treesitter_types::FromNode>::from_node(node, src)
                    .map(Self::ConstDeclaration)
                    .unwrap_or(Self::Unknown(node))
            }
            "const_element" => <ConstElement as ::treesitter_types::FromNode>::from_node(node, src)
                .map(Self::ConstElement)
                .unwrap_or(Self::Unknown(node)),
            "continue_statement" => {
                <ContinueStatement as ::treesitter_types::FromNode>::from_node(node, src)
                    .map(Self::ContinueStatement)
                    .unwrap_or(Self::Unknown(node))
            }
            "declaration_list" => {
                <DeclarationList as ::treesitter_types::FromNode>::from_node(node, src)
                    .map(Self::DeclarationList)
                    .unwrap_or(Self::Unknown(node))
            }
            "declare_directive" => {
                <DeclareDirective as ::treesitter_types::FromNode>::from_node(node, src)
                    .map(Self::DeclareDirective)
                    .unwrap_or(Self::Unknown(node))
            }
            "declare_statement" => {
                <DeclareStatement as ::treesitter_types::FromNode>::from_node(node, src)
                    .map(Self::DeclareStatement)
                    .unwrap_or(Self::Unknown(node))
            }
            "default_statement" => {
                <DefaultStatement as ::treesitter_types::FromNode>::from_node(node, src)
                    .map(Self::DefaultStatement)
                    .unwrap_or(Self::Unknown(node))
            }
            "disjunctive_normal_form_type" => {
                <DisjunctiveNormalFormType as ::treesitter_types::FromNode>::from_node(node, src)
                    .map(Self::DisjunctiveNormalFormType)
                    .unwrap_or(Self::Unknown(node))
            }
            "do_statement" => <DoStatement as ::treesitter_types::FromNode>::from_node(node, src)
                .map(Self::DoStatement)
                .unwrap_or(Self::Unknown(node)),
            "dynamic_variable_name" => {
                <DynamicVariableName as ::treesitter_types::FromNode>::from_node(node, src)
                    .map(Self::DynamicVariableName)
                    .unwrap_or(Self::Unknown(node))
            }
            "echo_statement" => {
                <EchoStatement as ::treesitter_types::FromNode>::from_node(node, src)
                    .map(Self::EchoStatement)
                    .unwrap_or(Self::Unknown(node))
            }
            "else_clause" => <ElseClause as ::treesitter_types::FromNode>::from_node(node, src)
                .map(Self::ElseClause)
                .unwrap_or(Self::Unknown(node)),
            "else_if_clause" => {
                <ElseIfClause as ::treesitter_types::FromNode>::from_node(node, src)
                    .map(Self::ElseIfClause)
                    .unwrap_or(Self::Unknown(node))
            }
            "empty_statement" => {
                <EmptyStatement as ::treesitter_types::FromNode>::from_node(node, src)
                    .map(Self::EmptyStatement)
                    .unwrap_or(Self::Unknown(node))
            }
            "encapsed_string" => {
                <EncapsedString as ::treesitter_types::FromNode>::from_node(node, src)
                    .map(Self::EncapsedString)
                    .unwrap_or(Self::Unknown(node))
            }
            "enum_case" => <EnumCase as ::treesitter_types::FromNode>::from_node(node, src)
                .map(Self::EnumCase)
                .unwrap_or(Self::Unknown(node)),
            "enum_declaration" => {
                <EnumDeclaration as ::treesitter_types::FromNode>::from_node(node, src)
                    .map(Self::EnumDeclaration)
                    .unwrap_or(Self::Unknown(node))
            }
            "enum_declaration_list" => {
                <EnumDeclarationList as ::treesitter_types::FromNode>::from_node(node, src)
                    .map(Self::EnumDeclarationList)
                    .unwrap_or(Self::Unknown(node))
            }
            "error_suppression_expression" => {
                <ErrorSuppressionExpression as ::treesitter_types::FromNode>::from_node(node, src)
                    .map(Self::ErrorSuppressionExpression)
                    .unwrap_or(Self::Unknown(node))
            }
            "exit_statement" => {
                <ExitStatement as ::treesitter_types::FromNode>::from_node(node, src)
                    .map(Self::ExitStatement)
                    .unwrap_or(Self::Unknown(node))
            }
            "expression_statement" => {
                <ExpressionStatement as ::treesitter_types::FromNode>::from_node(node, src)
                    .map(Self::ExpressionStatement)
                    .unwrap_or(Self::Unknown(node))
            }
            "final_modifier" => {
                <FinalModifier as ::treesitter_types::FromNode>::from_node(node, src)
                    .map(Self::FinalModifier)
                    .unwrap_or(Self::Unknown(node))
            }
            "finally_clause" => {
                <FinallyClause as ::treesitter_types::FromNode>::from_node(node, src)
                    .map(Self::FinallyClause)
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
            "formal_parameters" => {
                <FormalParameters as ::treesitter_types::FromNode>::from_node(node, src)
                    .map(Self::FormalParameters)
                    .unwrap_or(Self::Unknown(node))
            }
            "function_call_expression" => {
                <FunctionCallExpression as ::treesitter_types::FromNode>::from_node(node, src)
                    .map(Self::FunctionCallExpression)
                    .unwrap_or(Self::Unknown(node))
            }
            "function_definition" => {
                <FunctionDefinition as ::treesitter_types::FromNode>::from_node(node, src)
                    .map(Self::FunctionDefinition)
                    .unwrap_or(Self::Unknown(node))
            }
            "function_static_declaration" => {
                <FunctionStaticDeclaration as ::treesitter_types::FromNode>::from_node(node, src)
                    .map(Self::FunctionStaticDeclaration)
                    .unwrap_or(Self::Unknown(node))
            }
            "global_declaration" => {
                <GlobalDeclaration as ::treesitter_types::FromNode>::from_node(node, src)
                    .map(Self::GlobalDeclaration)
                    .unwrap_or(Self::Unknown(node))
            }
            "goto_statement" => {
                <GotoStatement as ::treesitter_types::FromNode>::from_node(node, src)
                    .map(Self::GotoStatement)
                    .unwrap_or(Self::Unknown(node))
            }
            "heredoc" => <Heredoc as ::treesitter_types::FromNode>::from_node(node, src)
                .map(Self::Heredoc)
                .unwrap_or(Self::Unknown(node)),
            "heredoc_body" => <HeredocBody as ::treesitter_types::FromNode>::from_node(node, src)
                .map(Self::HeredocBody)
                .unwrap_or(Self::Unknown(node)),
            "if_statement" => <IfStatement as ::treesitter_types::FromNode>::from_node(node, src)
                .map(Self::IfStatement)
                .unwrap_or(Self::Unknown(node)),
            "include_expression" => {
                <IncludeExpression as ::treesitter_types::FromNode>::from_node(node, src)
                    .map(Self::IncludeExpression)
                    .unwrap_or(Self::Unknown(node))
            }
            "include_once_expression" => {
                <IncludeOnceExpression as ::treesitter_types::FromNode>::from_node(node, src)
                    .map(Self::IncludeOnceExpression)
                    .unwrap_or(Self::Unknown(node))
            }
            "interface_declaration" => {
                <InterfaceDeclaration as ::treesitter_types::FromNode>::from_node(node, src)
                    .map(Self::InterfaceDeclaration)
                    .unwrap_or(Self::Unknown(node))
            }
            "intersection_type" => {
                <IntersectionType as ::treesitter_types::FromNode>::from_node(node, src)
                    .map(Self::IntersectionType)
                    .unwrap_or(Self::Unknown(node))
            }
            "list_literal" => <ListLiteral as ::treesitter_types::FromNode>::from_node(node, src)
                .map(Self::ListLiteral)
                .unwrap_or(Self::Unknown(node)),
            "match_block" => <MatchBlock as ::treesitter_types::FromNode>::from_node(node, src)
                .map(Self::MatchBlock)
                .unwrap_or(Self::Unknown(node)),
            "match_condition_list" => {
                <MatchConditionList as ::treesitter_types::FromNode>::from_node(node, src)
                    .map(Self::MatchConditionList)
                    .unwrap_or(Self::Unknown(node))
            }
            "match_conditional_expression" => {
                <MatchConditionalExpression as ::treesitter_types::FromNode>::from_node(node, src)
                    .map(Self::MatchConditionalExpression)
                    .unwrap_or(Self::Unknown(node))
            }
            "match_default_expression" => {
                <MatchDefaultExpression as ::treesitter_types::FromNode>::from_node(node, src)
                    .map(Self::MatchDefaultExpression)
                    .unwrap_or(Self::Unknown(node))
            }
            "match_expression" => {
                <MatchExpression as ::treesitter_types::FromNode>::from_node(node, src)
                    .map(Self::MatchExpression)
                    .unwrap_or(Self::Unknown(node))
            }
            "member_access_expression" => {
                <MemberAccessExpression as ::treesitter_types::FromNode>::from_node(node, src)
                    .map(Self::MemberAccessExpression)
                    .unwrap_or(Self::Unknown(node))
            }
            "member_call_expression" => {
                <MemberCallExpression as ::treesitter_types::FromNode>::from_node(node, src)
                    .map(Self::MemberCallExpression)
                    .unwrap_or(Self::Unknown(node))
            }
            "method_declaration" => {
                <MethodDeclaration as ::treesitter_types::FromNode>::from_node(node, src)
                    .map(Self::MethodDeclaration)
                    .unwrap_or(Self::Unknown(node))
            }
            "name" => <Name as ::treesitter_types::FromNode>::from_node(node, src)
                .map(Self::Name)
                .unwrap_or(Self::Unknown(node)),
            "named_label_statement" => {
                <NamedLabelStatement as ::treesitter_types::FromNode>::from_node(node, src)
                    .map(Self::NamedLabelStatement)
                    .unwrap_or(Self::Unknown(node))
            }
            "named_type" => <NamedType as ::treesitter_types::FromNode>::from_node(node, src)
                .map(Self::NamedType)
                .unwrap_or(Self::Unknown(node)),
            "namespace_definition" => {
                <NamespaceDefinition as ::treesitter_types::FromNode>::from_node(node, src)
                    .map(Self::NamespaceDefinition)
                    .unwrap_or(Self::Unknown(node))
            }
            "namespace_name" => {
                <NamespaceName as ::treesitter_types::FromNode>::from_node(node, src)
                    .map(Self::NamespaceName)
                    .unwrap_or(Self::Unknown(node))
            }
            "namespace_use_clause" => {
                <NamespaceUseClause as ::treesitter_types::FromNode>::from_node(node, src)
                    .map(Self::NamespaceUseClause)
                    .unwrap_or(Self::Unknown(node))
            }
            "namespace_use_declaration" => {
                <NamespaceUseDeclaration as ::treesitter_types::FromNode>::from_node(node, src)
                    .map(Self::NamespaceUseDeclaration)
                    .unwrap_or(Self::Unknown(node))
            }
            "namespace_use_group" => {
                <NamespaceUseGroup as ::treesitter_types::FromNode>::from_node(node, src)
                    .map(Self::NamespaceUseGroup)
                    .unwrap_or(Self::Unknown(node))
            }
            "nowdoc" => <Nowdoc as ::treesitter_types::FromNode>::from_node(node, src)
                .map(Self::Nowdoc)
                .unwrap_or(Self::Unknown(node)),
            "nowdoc_body" => <NowdocBody as ::treesitter_types::FromNode>::from_node(node, src)
                .map(Self::NowdocBody)
                .unwrap_or(Self::Unknown(node)),
            "null" => <Null as ::treesitter_types::FromNode>::from_node(node, src)
                .map(Self::Null)
                .unwrap_or(Self::Unknown(node)),
            "nullsafe_member_access_expression" => {
                <NullsafeMemberAccessExpression as ::treesitter_types::FromNode>::from_node(
                    node, src,
                )
                .map(Self::NullsafeMemberAccessExpression)
                .unwrap_or(Self::Unknown(node))
            }
            "nullsafe_member_call_expression" => {
                <NullsafeMemberCallExpression as ::treesitter_types::FromNode>::from_node(node, src)
                    .map(Self::NullsafeMemberCallExpression)
                    .unwrap_or(Self::Unknown(node))
            }
            "object_creation_expression" => {
                <ObjectCreationExpression as ::treesitter_types::FromNode>::from_node(node, src)
                    .map(Self::ObjectCreationExpression)
                    .unwrap_or(Self::Unknown(node))
            }
            "optional_type" => <OptionalType as ::treesitter_types::FromNode>::from_node(node, src)
                .map(Self::OptionalType)
                .unwrap_or(Self::Unknown(node)),
            "pair" => <Pair as ::treesitter_types::FromNode>::from_node(node, src)
                .map(Self::Pair)
                .unwrap_or(Self::Unknown(node)),
            "parenthesized_expression" => {
                <ParenthesizedExpression as ::treesitter_types::FromNode>::from_node(node, src)
                    .map(Self::ParenthesizedExpression)
                    .unwrap_or(Self::Unknown(node))
            }
            "primitive_type" => {
                <PrimitiveType as ::treesitter_types::FromNode>::from_node(node, src)
                    .map(Self::PrimitiveType)
                    .unwrap_or(Self::Unknown(node))
            }
            "print_intrinsic" => {
                <PrintIntrinsic as ::treesitter_types::FromNode>::from_node(node, src)
                    .map(Self::PrintIntrinsic)
                    .unwrap_or(Self::Unknown(node))
            }
            "program" => <Program as ::treesitter_types::FromNode>::from_node(node, src)
                .map(Self::Program)
                .unwrap_or(Self::Unknown(node)),
            "property_declaration" => {
                <PropertyDeclaration as ::treesitter_types::FromNode>::from_node(node, src)
                    .map(Self::PropertyDeclaration)
                    .unwrap_or(Self::Unknown(node))
            }
            "property_element" => {
                <PropertyElement as ::treesitter_types::FromNode>::from_node(node, src)
                    .map(Self::PropertyElement)
                    .unwrap_or(Self::Unknown(node))
            }
            "property_hook" => <PropertyHook as ::treesitter_types::FromNode>::from_node(node, src)
                .map(Self::PropertyHook)
                .unwrap_or(Self::Unknown(node)),
            "property_hook_list" => {
                <PropertyHookList as ::treesitter_types::FromNode>::from_node(node, src)
                    .map(Self::PropertyHookList)
                    .unwrap_or(Self::Unknown(node))
            }
            "property_promotion_parameter" => {
                <PropertyPromotionParameter as ::treesitter_types::FromNode>::from_node(node, src)
                    .map(Self::PropertyPromotionParameter)
                    .unwrap_or(Self::Unknown(node))
            }
            "qualified_name" => {
                <QualifiedName as ::treesitter_types::FromNode>::from_node(node, src)
                    .map(Self::QualifiedName)
                    .unwrap_or(Self::Unknown(node))
            }
            "readonly_modifier" => {
                <ReadonlyModifier as ::treesitter_types::FromNode>::from_node(node, src)
                    .map(Self::ReadonlyModifier)
                    .unwrap_or(Self::Unknown(node))
            }
            "reference_assignment_expression" => {
                <ReferenceAssignmentExpression as ::treesitter_types::FromNode>::from_node(
                    node, src,
                )
                .map(Self::ReferenceAssignmentExpression)
                .unwrap_or(Self::Unknown(node))
            }
            "reference_modifier" => {
                <ReferenceModifier as ::treesitter_types::FromNode>::from_node(node, src)
                    .map(Self::ReferenceModifier)
                    .unwrap_or(Self::Unknown(node))
            }
            "relative_name" => <RelativeName as ::treesitter_types::FromNode>::from_node(node, src)
                .map(Self::RelativeName)
                .unwrap_or(Self::Unknown(node)),
            "relative_scope" => {
                <RelativeScope as ::treesitter_types::FromNode>::from_node(node, src)
                    .map(Self::RelativeScope)
                    .unwrap_or(Self::Unknown(node))
            }
            "require_expression" => {
                <RequireExpression as ::treesitter_types::FromNode>::from_node(node, src)
                    .map(Self::RequireExpression)
                    .unwrap_or(Self::Unknown(node))
            }
            "require_once_expression" => {
                <RequireOnceExpression as ::treesitter_types::FromNode>::from_node(node, src)
                    .map(Self::RequireOnceExpression)
                    .unwrap_or(Self::Unknown(node))
            }
            "return_statement" => {
                <ReturnStatement as ::treesitter_types::FromNode>::from_node(node, src)
                    .map(Self::ReturnStatement)
                    .unwrap_or(Self::Unknown(node))
            }
            "scoped_call_expression" => {
                <ScopedCallExpression as ::treesitter_types::FromNode>::from_node(node, src)
                    .map(Self::ScopedCallExpression)
                    .unwrap_or(Self::Unknown(node))
            }
            "scoped_property_access_expression" => {
                <ScopedPropertyAccessExpression as ::treesitter_types::FromNode>::from_node(
                    node, src,
                )
                .map(Self::ScopedPropertyAccessExpression)
                .unwrap_or(Self::Unknown(node))
            }
            "sequence_expression" => {
                <SequenceExpression as ::treesitter_types::FromNode>::from_node(node, src)
                    .map(Self::SequenceExpression)
                    .unwrap_or(Self::Unknown(node))
            }
            "shell_command_expression" => {
                <ShellCommandExpression as ::treesitter_types::FromNode>::from_node(node, src)
                    .map(Self::ShellCommandExpression)
                    .unwrap_or(Self::Unknown(node))
            }
            "simple_parameter" => {
                <SimpleParameter as ::treesitter_types::FromNode>::from_node(node, src)
                    .map(Self::SimpleParameter)
                    .unwrap_or(Self::Unknown(node))
            }
            "static_modifier" => {
                <StaticModifier as ::treesitter_types::FromNode>::from_node(node, src)
                    .map(Self::StaticModifier)
                    .unwrap_or(Self::Unknown(node))
            }
            "static_variable_declaration" => {
                <StaticVariableDeclaration as ::treesitter_types::FromNode>::from_node(node, src)
                    .map(Self::StaticVariableDeclaration)
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
            "subscript_expression" => {
                <SubscriptExpression as ::treesitter_types::FromNode>::from_node(node, src)
                    .map(Self::SubscriptExpression)
                    .unwrap_or(Self::Unknown(node))
            }
            "switch_block" => <SwitchBlock as ::treesitter_types::FromNode>::from_node(node, src)
                .map(Self::SwitchBlock)
                .unwrap_or(Self::Unknown(node)),
            "switch_statement" => {
                <SwitchStatement as ::treesitter_types::FromNode>::from_node(node, src)
                    .map(Self::SwitchStatement)
                    .unwrap_or(Self::Unknown(node))
            }
            "text" => <Text as ::treesitter_types::FromNode>::from_node(node, src)
                .map(Self::Text)
                .unwrap_or(Self::Unknown(node)),
            "text_interpolation" => {
                <TextInterpolation as ::treesitter_types::FromNode>::from_node(node, src)
                    .map(Self::TextInterpolation)
                    .unwrap_or(Self::Unknown(node))
            }
            "throw_expression" => {
                <ThrowExpression as ::treesitter_types::FromNode>::from_node(node, src)
                    .map(Self::ThrowExpression)
                    .unwrap_or(Self::Unknown(node))
            }
            "trait_declaration" => {
                <TraitDeclaration as ::treesitter_types::FromNode>::from_node(node, src)
                    .map(Self::TraitDeclaration)
                    .unwrap_or(Self::Unknown(node))
            }
            "try_statement" => <TryStatement as ::treesitter_types::FromNode>::from_node(node, src)
                .map(Self::TryStatement)
                .unwrap_or(Self::Unknown(node)),
            "type_list" => <TypeList as ::treesitter_types::FromNode>::from_node(node, src)
                .map(Self::TypeList)
                .unwrap_or(Self::Unknown(node)),
            "unary_op_expression" => {
                <UnaryOpExpression as ::treesitter_types::FromNode>::from_node(node, src)
                    .map(Self::UnaryOpExpression)
                    .unwrap_or(Self::Unknown(node))
            }
            "union_type" => <UnionType as ::treesitter_types::FromNode>::from_node(node, src)
                .map(Self::UnionType)
                .unwrap_or(Self::Unknown(node)),
            "unset_statement" => {
                <UnsetStatement as ::treesitter_types::FromNode>::from_node(node, src)
                    .map(Self::UnsetStatement)
                    .unwrap_or(Self::Unknown(node))
            }
            "update_expression" => {
                <UpdateExpression as ::treesitter_types::FromNode>::from_node(node, src)
                    .map(Self::UpdateExpression)
                    .unwrap_or(Self::Unknown(node))
            }
            "use_as_clause" => <UseAsClause as ::treesitter_types::FromNode>::from_node(node, src)
                .map(Self::UseAsClause)
                .unwrap_or(Self::Unknown(node)),
            "use_declaration" => {
                <UseDeclaration as ::treesitter_types::FromNode>::from_node(node, src)
                    .map(Self::UseDeclaration)
                    .unwrap_or(Self::Unknown(node))
            }
            "use_instead_of_clause" => {
                <UseInsteadOfClause as ::treesitter_types::FromNode>::from_node(node, src)
                    .map(Self::UseInsteadOfClause)
                    .unwrap_or(Self::Unknown(node))
            }
            "use_list" => <UseList as ::treesitter_types::FromNode>::from_node(node, src)
                .map(Self::UseList)
                .unwrap_or(Self::Unknown(node)),
            "variable_name" => <VariableName as ::treesitter_types::FromNode>::from_node(node, src)
                .map(Self::VariableName)
                .unwrap_or(Self::Unknown(node)),
            "variadic_parameter" => {
                <VariadicParameter as ::treesitter_types::FromNode>::from_node(node, src)
                    .map(Self::VariadicParameter)
                    .unwrap_or(Self::Unknown(node))
            }
            "variadic_placeholder" => {
                <VariadicPlaceholder as ::treesitter_types::FromNode>::from_node(node, src)
                    .map(Self::VariadicPlaceholder)
                    .unwrap_or(Self::Unknown(node))
            }
            "variadic_unpacking" => {
                <VariadicUnpacking as ::treesitter_types::FromNode>::from_node(node, src)
                    .map(Self::VariadicUnpacking)
                    .unwrap_or(Self::Unknown(node))
            }
            "visibility_modifier" => {
                <VisibilityModifier as ::treesitter_types::FromNode>::from_node(node, src)
                    .map(Self::VisibilityModifier)
                    .unwrap_or(Self::Unknown(node))
            }
            "while_statement" => {
                <WhileStatement as ::treesitter_types::FromNode>::from_node(node, src)
                    .map(Self::WhileStatement)
                    .unwrap_or(Self::Unknown(node))
            }
            "yield_expression" => {
                <YieldExpression as ::treesitter_types::FromNode>::from_node(node, src)
                    .map(Self::YieldExpression)
                    .unwrap_or(Self::Unknown(node))
            }
            "bottom_type" => <BottomType as ::treesitter_types::FromNode>::from_node(node, src)
                .map(Self::BottomType)
                .unwrap_or(Self::Unknown(node)),
            "comment" => <Comment as ::treesitter_types::FromNode>::from_node(node, src)
                .map(Self::Comment)
                .unwrap_or(Self::Unknown(node)),
            "escape_sequence" => {
                <EscapeSequence as ::treesitter_types::FromNode>::from_node(node, src)
                    .map(Self::EscapeSequence)
                    .unwrap_or(Self::Unknown(node))
            }
            "float" => <Float as ::treesitter_types::FromNode>::from_node(node, src)
                .map(Self::Float)
                .unwrap_or(Self::Unknown(node)),
            "heredoc_end" => <HeredocEnd as ::treesitter_types::FromNode>::from_node(node, src)
                .map(Self::HeredocEnd)
                .unwrap_or(Self::Unknown(node)),
            "heredoc_start" => <HeredocStart as ::treesitter_types::FromNode>::from_node(node, src)
                .map(Self::HeredocStart)
                .unwrap_or(Self::Unknown(node)),
            "integer" => <Integer as ::treesitter_types::FromNode>::from_node(node, src)
                .map(Self::Integer)
                .unwrap_or(Self::Unknown(node)),
            "nowdoc_string" => <NowdocString as ::treesitter_types::FromNode>::from_node(node, src)
                .map(Self::NowdocString)
                .unwrap_or(Self::Unknown(node)),
            "operation" => <Operation as ::treesitter_types::FromNode>::from_node(node, src)
                .map(Self::Operation)
                .unwrap_or(Self::Unknown(node)),
            "php_end_tag" => <PhpEndTag as ::treesitter_types::FromNode>::from_node(node, src)
                .map(Self::PhpEndTag)
                .unwrap_or(Self::Unknown(node)),
            "php_tag" => <PhpTag as ::treesitter_types::FromNode>::from_node(node, src)
                .map(Self::PhpTag)
                .unwrap_or(Self::Unknown(node)),
            "var_modifier" => <VarModifier as ::treesitter_types::FromNode>::from_node(node, src)
                .map(Self::VarModifier)
                .unwrap_or(Self::Unknown(node)),
            _ => Self::Unknown(node),
        }
    }
}
impl ::treesitter_types::Spanned for AnyNode<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        match self {
            Self::Expression(inner) => inner.span(),
            Self::Literal(inner) => inner.span(),
            Self::PrimaryExpression(inner) => inner.span(),
            Self::Statement(inner) => inner.span(),
            Self::Type(inner) => inner.span(),
            Self::AbstractModifier(inner) => inner.span(),
            Self::AnonymousClass(inner) => inner.span(),
            Self::AnonymousFunction(inner) => inner.span(),
            Self::AnonymousFunctionUseClause(inner) => inner.span(),
            Self::Argument(inner) => inner.span(),
            Self::Arguments(inner) => inner.span(),
            Self::ArrayCreationExpression(inner) => inner.span(),
            Self::ArrayElementInitializer(inner) => inner.span(),
            Self::ArrowFunction(inner) => inner.span(),
            Self::AssignmentExpression(inner) => inner.span(),
            Self::Attribute(inner) => inner.span(),
            Self::AttributeGroup(inner) => inner.span(),
            Self::AttributeList(inner) => inner.span(),
            Self::AugmentedAssignmentExpression(inner) => inner.span(),
            Self::BaseClause(inner) => inner.span(),
            Self::BinaryExpression(inner) => inner.span(),
            Self::Boolean(inner) => inner.span(),
            Self::BreakStatement(inner) => inner.span(),
            Self::ByRef(inner) => inner.span(),
            Self::CaseStatement(inner) => inner.span(),
            Self::CastExpression(inner) => inner.span(),
            Self::CastType(inner) => inner.span(),
            Self::CatchClause(inner) => inner.span(),
            Self::ClassConstantAccessExpression(inner) => inner.span(),
            Self::ClassDeclaration(inner) => inner.span(),
            Self::ClassInterfaceClause(inner) => inner.span(),
            Self::CloneExpression(inner) => inner.span(),
            Self::ColonBlock(inner) => inner.span(),
            Self::CompoundStatement(inner) => inner.span(),
            Self::ConditionalExpression(inner) => inner.span(),
            Self::ConstDeclaration(inner) => inner.span(),
            Self::ConstElement(inner) => inner.span(),
            Self::ContinueStatement(inner) => inner.span(),
            Self::DeclarationList(inner) => inner.span(),
            Self::DeclareDirective(inner) => inner.span(),
            Self::DeclareStatement(inner) => inner.span(),
            Self::DefaultStatement(inner) => inner.span(),
            Self::DisjunctiveNormalFormType(inner) => inner.span(),
            Self::DoStatement(inner) => inner.span(),
            Self::DynamicVariableName(inner) => inner.span(),
            Self::EchoStatement(inner) => inner.span(),
            Self::ElseClause(inner) => inner.span(),
            Self::ElseIfClause(inner) => inner.span(),
            Self::EmptyStatement(inner) => inner.span(),
            Self::EncapsedString(inner) => inner.span(),
            Self::EnumCase(inner) => inner.span(),
            Self::EnumDeclaration(inner) => inner.span(),
            Self::EnumDeclarationList(inner) => inner.span(),
            Self::ErrorSuppressionExpression(inner) => inner.span(),
            Self::ExitStatement(inner) => inner.span(),
            Self::ExpressionStatement(inner) => inner.span(),
            Self::FinalModifier(inner) => inner.span(),
            Self::FinallyClause(inner) => inner.span(),
            Self::ForStatement(inner) => inner.span(),
            Self::ForeachStatement(inner) => inner.span(),
            Self::FormalParameters(inner) => inner.span(),
            Self::FunctionCallExpression(inner) => inner.span(),
            Self::FunctionDefinition(inner) => inner.span(),
            Self::FunctionStaticDeclaration(inner) => inner.span(),
            Self::GlobalDeclaration(inner) => inner.span(),
            Self::GotoStatement(inner) => inner.span(),
            Self::Heredoc(inner) => inner.span(),
            Self::HeredocBody(inner) => inner.span(),
            Self::IfStatement(inner) => inner.span(),
            Self::IncludeExpression(inner) => inner.span(),
            Self::IncludeOnceExpression(inner) => inner.span(),
            Self::InterfaceDeclaration(inner) => inner.span(),
            Self::IntersectionType(inner) => inner.span(),
            Self::ListLiteral(inner) => inner.span(),
            Self::MatchBlock(inner) => inner.span(),
            Self::MatchConditionList(inner) => inner.span(),
            Self::MatchConditionalExpression(inner) => inner.span(),
            Self::MatchDefaultExpression(inner) => inner.span(),
            Self::MatchExpression(inner) => inner.span(),
            Self::MemberAccessExpression(inner) => inner.span(),
            Self::MemberCallExpression(inner) => inner.span(),
            Self::MethodDeclaration(inner) => inner.span(),
            Self::Name(inner) => inner.span(),
            Self::NamedLabelStatement(inner) => inner.span(),
            Self::NamedType(inner) => inner.span(),
            Self::NamespaceDefinition(inner) => inner.span(),
            Self::NamespaceName(inner) => inner.span(),
            Self::NamespaceUseClause(inner) => inner.span(),
            Self::NamespaceUseDeclaration(inner) => inner.span(),
            Self::NamespaceUseGroup(inner) => inner.span(),
            Self::Nowdoc(inner) => inner.span(),
            Self::NowdocBody(inner) => inner.span(),
            Self::Null(inner) => inner.span(),
            Self::NullsafeMemberAccessExpression(inner) => inner.span(),
            Self::NullsafeMemberCallExpression(inner) => inner.span(),
            Self::ObjectCreationExpression(inner) => inner.span(),
            Self::OptionalType(inner) => inner.span(),
            Self::Pair(inner) => inner.span(),
            Self::ParenthesizedExpression(inner) => inner.span(),
            Self::PrimitiveType(inner) => inner.span(),
            Self::PrintIntrinsic(inner) => inner.span(),
            Self::Program(inner) => inner.span(),
            Self::PropertyDeclaration(inner) => inner.span(),
            Self::PropertyElement(inner) => inner.span(),
            Self::PropertyHook(inner) => inner.span(),
            Self::PropertyHookList(inner) => inner.span(),
            Self::PropertyPromotionParameter(inner) => inner.span(),
            Self::QualifiedName(inner) => inner.span(),
            Self::ReadonlyModifier(inner) => inner.span(),
            Self::ReferenceAssignmentExpression(inner) => inner.span(),
            Self::ReferenceModifier(inner) => inner.span(),
            Self::RelativeName(inner) => inner.span(),
            Self::RelativeScope(inner) => inner.span(),
            Self::RequireExpression(inner) => inner.span(),
            Self::RequireOnceExpression(inner) => inner.span(),
            Self::ReturnStatement(inner) => inner.span(),
            Self::ScopedCallExpression(inner) => inner.span(),
            Self::ScopedPropertyAccessExpression(inner) => inner.span(),
            Self::SequenceExpression(inner) => inner.span(),
            Self::ShellCommandExpression(inner) => inner.span(),
            Self::SimpleParameter(inner) => inner.span(),
            Self::StaticModifier(inner) => inner.span(),
            Self::StaticVariableDeclaration(inner) => inner.span(),
            Self::String(inner) => inner.span(),
            Self::StringContent(inner) => inner.span(),
            Self::SubscriptExpression(inner) => inner.span(),
            Self::SwitchBlock(inner) => inner.span(),
            Self::SwitchStatement(inner) => inner.span(),
            Self::Text(inner) => inner.span(),
            Self::TextInterpolation(inner) => inner.span(),
            Self::ThrowExpression(inner) => inner.span(),
            Self::TraitDeclaration(inner) => inner.span(),
            Self::TryStatement(inner) => inner.span(),
            Self::TypeList(inner) => inner.span(),
            Self::UnaryOpExpression(inner) => inner.span(),
            Self::UnionType(inner) => inner.span(),
            Self::UnsetStatement(inner) => inner.span(),
            Self::UpdateExpression(inner) => inner.span(),
            Self::UseAsClause(inner) => inner.span(),
            Self::UseDeclaration(inner) => inner.span(),
            Self::UseInsteadOfClause(inner) => inner.span(),
            Self::UseList(inner) => inner.span(),
            Self::VariableName(inner) => inner.span(),
            Self::VariadicParameter(inner) => inner.span(),
            Self::VariadicPlaceholder(inner) => inner.span(),
            Self::VariadicUnpacking(inner) => inner.span(),
            Self::VisibilityModifier(inner) => inner.span(),
            Self::WhileStatement(inner) => inner.span(),
            Self::YieldExpression(inner) => inner.span(),
            Self::BottomType(inner) => inner.span(),
            Self::Comment(inner) => inner.span(),
            Self::EscapeSequence(inner) => inner.span(),
            Self::Float(inner) => inner.span(),
            Self::HeredocEnd(inner) => inner.span(),
            Self::HeredocStart(inner) => inner.span(),
            Self::Integer(inner) => inner.span(),
            Self::NowdocString(inner) => inner.span(),
            Self::Operation(inner) => inner.span(),
            Self::PhpEndTag(inner) => inner.span(),
            Self::PhpTag(inner) => inner.span(),
            Self::VarModifier(inner) => inner.span(),
            Self::Unknown(node) => ::treesitter_types::Span::from(*node),
        }
    }
}
