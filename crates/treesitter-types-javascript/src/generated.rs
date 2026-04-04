#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Declaration<'tree> {
    ClassDeclaration(::std::boxed::Box<ClassDeclaration<'tree>>),
    FunctionDeclaration(::std::boxed::Box<FunctionDeclaration<'tree>>),
    GeneratorFunctionDeclaration(::std::boxed::Box<GeneratorFunctionDeclaration<'tree>>),
    LexicalDeclaration(::std::boxed::Box<LexicalDeclaration<'tree>>),
    UsingDeclaration(::std::boxed::Box<UsingDeclaration<'tree>>),
    VariableDeclaration(::std::boxed::Box<VariableDeclaration<'tree>>),
}
impl<'tree> ::treesitter_types::FromNode<'tree> for Declaration<'tree> {
    #[allow(clippy::collapsible_else_if)]
    fn from_node(
        node: ::treesitter_types::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        match node.kind() {
            "class_declaration" => Ok(Self::ClassDeclaration(::std::boxed::Box::new(
                ::treesitter_types::runtime::maybe_grow_stack(|| {
                    <ClassDeclaration as ::treesitter_types::FromNode>::from_node(node, src)
                })?,
            ))),
            "function_declaration" => Ok(Self::FunctionDeclaration(::std::boxed::Box::new(
                ::treesitter_types::runtime::maybe_grow_stack(|| {
                    <FunctionDeclaration as ::treesitter_types::FromNode>::from_node(node, src)
                })?,
            ))),
            "generator_function_declaration" => Ok(Self::GeneratorFunctionDeclaration(
                ::std::boxed::Box::new(::treesitter_types::runtime::maybe_grow_stack(|| {
                    <GeneratorFunctionDeclaration as ::treesitter_types::FromNode>::from_node(
                        node, src,
                    )
                })?),
            )),
            "lexical_declaration" => Ok(Self::LexicalDeclaration(::std::boxed::Box::new(
                ::treesitter_types::runtime::maybe_grow_stack(|| {
                    <LexicalDeclaration as ::treesitter_types::FromNode>::from_node(node, src)
                })?,
            ))),
            "using_declaration" => Ok(Self::UsingDeclaration(::std::boxed::Box::new(
                ::treesitter_types::runtime::maybe_grow_stack(|| {
                    <UsingDeclaration as ::treesitter_types::FromNode>::from_node(node, src)
                })?,
            ))),
            "variable_declaration" => Ok(Self::VariableDeclaration(::std::boxed::Box::new(
                ::treesitter_types::runtime::maybe_grow_stack(|| {
                    <VariableDeclaration as ::treesitter_types::FromNode>::from_node(node, src)
                })?,
            ))),
            other => Err(::treesitter_types::ParseError::unexpected_kind(other, node)),
        }
    }
}
impl ::treesitter_types::Spanned for Declaration<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        match self {
            Self::ClassDeclaration(inner) => inner.span(),
            Self::FunctionDeclaration(inner) => inner.span(),
            Self::GeneratorFunctionDeclaration(inner) => inner.span(),
            Self::LexicalDeclaration(inner) => inner.span(),
            Self::UsingDeclaration(inner) => inner.span(),
            Self::VariableDeclaration(inner) => inner.span(),
        }
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Expression<'tree> {
    AssignmentExpression(::std::boxed::Box<AssignmentExpression<'tree>>),
    AugmentedAssignmentExpression(::std::boxed::Box<AugmentedAssignmentExpression<'tree>>),
    AwaitExpression(::std::boxed::Box<AwaitExpression<'tree>>),
    BinaryExpression(::std::boxed::Box<BinaryExpression<'tree>>),
    JsxElement(::std::boxed::Box<JsxElement<'tree>>),
    JsxSelfClosingElement(::std::boxed::Box<JsxSelfClosingElement<'tree>>),
    NewExpression(::std::boxed::Box<NewExpression<'tree>>),
    PrimaryExpression(::std::boxed::Box<PrimaryExpression<'tree>>),
    TernaryExpression(::std::boxed::Box<TernaryExpression<'tree>>),
    UnaryExpression(::std::boxed::Box<UnaryExpression<'tree>>),
    UpdateExpression(::std::boxed::Box<UpdateExpression<'tree>>),
    YieldExpression(::std::boxed::Box<YieldExpression<'tree>>),
}
impl<'tree> ::treesitter_types::FromNode<'tree> for Expression<'tree> {
    #[allow(clippy::collapsible_else_if)]
    fn from_node(
        node: ::treesitter_types::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        match node.kind() {
            "assignment_expression" => Ok(Self::AssignmentExpression(::std::boxed::Box::new(
                ::treesitter_types::runtime::maybe_grow_stack(|| {
                    <AssignmentExpression as ::treesitter_types::FromNode>::from_node(node, src)
                })?,
            ))),
            "augmented_assignment_expression" => Ok(Self::AugmentedAssignmentExpression(
                ::std::boxed::Box::new(::treesitter_types::runtime::maybe_grow_stack(|| {
                    <AugmentedAssignmentExpression as ::treesitter_types::FromNode>::from_node(
                        node, src,
                    )
                })?),
            )),
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
            "jsx_element" => Ok(Self::JsxElement(::std::boxed::Box::new(
                ::treesitter_types::runtime::maybe_grow_stack(|| {
                    <JsxElement as ::treesitter_types::FromNode>::from_node(node, src)
                })?,
            ))),
            "jsx_self_closing_element" => Ok(Self::JsxSelfClosingElement(::std::boxed::Box::new(
                ::treesitter_types::runtime::maybe_grow_stack(|| {
                    <JsxSelfClosingElement as ::treesitter_types::FromNode>::from_node(node, src)
                })?,
            ))),
            "new_expression" => Ok(Self::NewExpression(::std::boxed::Box::new(
                ::treesitter_types::runtime::maybe_grow_stack(|| {
                    <NewExpression as ::treesitter_types::FromNode>::from_node(node, src)
                })?,
            ))),
            "ternary_expression" => Ok(Self::TernaryExpression(::std::boxed::Box::new(
                ::treesitter_types::runtime::maybe_grow_stack(|| {
                    <TernaryExpression as ::treesitter_types::FromNode>::from_node(node, src)
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
            "yield_expression" => Ok(Self::YieldExpression(::std::boxed::Box::new(
                ::treesitter_types::runtime::maybe_grow_stack(|| {
                    <YieldExpression as ::treesitter_types::FromNode>::from_node(node, src)
                })?,
            ))),
            _other => {
                if let Ok(v) = ::treesitter_types::runtime::maybe_grow_stack(|| {
                    <PrimaryExpression as ::treesitter_types::FromNode>::from_node(node, src)
                }) {
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
            Self::AwaitExpression(inner) => inner.span(),
            Self::BinaryExpression(inner) => inner.span(),
            Self::JsxElement(inner) => inner.span(),
            Self::JsxSelfClosingElement(inner) => inner.span(),
            Self::NewExpression(inner) => inner.span(),
            Self::PrimaryExpression(inner) => inner.span(),
            Self::TernaryExpression(inner) => inner.span(),
            Self::UnaryExpression(inner) => inner.span(),
            Self::UpdateExpression(inner) => inner.span(),
            Self::YieldExpression(inner) => inner.span(),
        }
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Pattern<'tree> {
    ArrayPattern(::std::boxed::Box<ArrayPattern<'tree>>),
    Identifier(::std::boxed::Box<Identifier<'tree>>),
    MemberExpression(::std::boxed::Box<MemberExpression<'tree>>),
    ObjectPattern(::std::boxed::Box<ObjectPattern<'tree>>),
    RestPattern(::std::boxed::Box<RestPattern<'tree>>),
    SubscriptExpression(::std::boxed::Box<SubscriptExpression<'tree>>),
    Undefined(::std::boxed::Box<Undefined<'tree>>),
}
impl<'tree> ::treesitter_types::FromNode<'tree> for Pattern<'tree> {
    #[allow(clippy::collapsible_else_if)]
    fn from_node(
        node: ::treesitter_types::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        match node.kind() {
            "array_pattern" => Ok(Self::ArrayPattern(::std::boxed::Box::new(
                ::treesitter_types::runtime::maybe_grow_stack(|| {
                    <ArrayPattern as ::treesitter_types::FromNode>::from_node(node, src)
                })?,
            ))),
            "identifier" => Ok(Self::Identifier(::std::boxed::Box::new(
                ::treesitter_types::runtime::maybe_grow_stack(|| {
                    <Identifier as ::treesitter_types::FromNode>::from_node(node, src)
                })?,
            ))),
            "member_expression" => Ok(Self::MemberExpression(::std::boxed::Box::new(
                ::treesitter_types::runtime::maybe_grow_stack(|| {
                    <MemberExpression as ::treesitter_types::FromNode>::from_node(node, src)
                })?,
            ))),
            "object_pattern" => Ok(Self::ObjectPattern(::std::boxed::Box::new(
                ::treesitter_types::runtime::maybe_grow_stack(|| {
                    <ObjectPattern as ::treesitter_types::FromNode>::from_node(node, src)
                })?,
            ))),
            "rest_pattern" => Ok(Self::RestPattern(::std::boxed::Box::new(
                ::treesitter_types::runtime::maybe_grow_stack(|| {
                    <RestPattern as ::treesitter_types::FromNode>::from_node(node, src)
                })?,
            ))),
            "subscript_expression" => Ok(Self::SubscriptExpression(::std::boxed::Box::new(
                ::treesitter_types::runtime::maybe_grow_stack(|| {
                    <SubscriptExpression as ::treesitter_types::FromNode>::from_node(node, src)
                })?,
            ))),
            "undefined" => Ok(Self::Undefined(::std::boxed::Box::new(
                ::treesitter_types::runtime::maybe_grow_stack(|| {
                    <Undefined as ::treesitter_types::FromNode>::from_node(node, src)
                })?,
            ))),
            other => Err(::treesitter_types::ParseError::unexpected_kind(other, node)),
        }
    }
}
impl ::treesitter_types::Spanned for Pattern<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        match self {
            Self::ArrayPattern(inner) => inner.span(),
            Self::Identifier(inner) => inner.span(),
            Self::MemberExpression(inner) => inner.span(),
            Self::ObjectPattern(inner) => inner.span(),
            Self::RestPattern(inner) => inner.span(),
            Self::SubscriptExpression(inner) => inner.span(),
            Self::Undefined(inner) => inner.span(),
        }
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum PrimaryExpression<'tree> {
    Array(::std::boxed::Box<Array<'tree>>),
    ArrowFunction(::std::boxed::Box<ArrowFunction<'tree>>),
    CallExpression(::std::boxed::Box<CallExpression<'tree>>),
    Class(::std::boxed::Box<Class<'tree>>),
    False(::std::boxed::Box<False<'tree>>),
    FunctionExpression(::std::boxed::Box<FunctionExpression<'tree>>),
    GeneratorFunction(::std::boxed::Box<GeneratorFunction<'tree>>),
    Identifier(::std::boxed::Box<Identifier<'tree>>),
    MemberExpression(::std::boxed::Box<MemberExpression<'tree>>),
    MetaProperty(::std::boxed::Box<MetaProperty<'tree>>),
    Null(::std::boxed::Box<Null<'tree>>),
    Number(::std::boxed::Box<Number<'tree>>),
    Object(::std::boxed::Box<Object<'tree>>),
    ParenthesizedExpression(::std::boxed::Box<ParenthesizedExpression<'tree>>),
    Regex(::std::boxed::Box<Regex<'tree>>),
    String(::std::boxed::Box<String<'tree>>),
    SubscriptExpression(::std::boxed::Box<SubscriptExpression<'tree>>),
    Super(::std::boxed::Box<Super<'tree>>),
    TemplateString(::std::boxed::Box<TemplateString<'tree>>),
    This(::std::boxed::Box<This<'tree>>),
    True(::std::boxed::Box<True<'tree>>),
    Undefined(::std::boxed::Box<Undefined<'tree>>),
}
impl<'tree> ::treesitter_types::FromNode<'tree> for PrimaryExpression<'tree> {
    #[allow(clippy::collapsible_else_if)]
    fn from_node(
        node: ::treesitter_types::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        match node.kind() {
            "array" => Ok(Self::Array(::std::boxed::Box::new(
                ::treesitter_types::runtime::maybe_grow_stack(|| {
                    <Array as ::treesitter_types::FromNode>::from_node(node, src)
                })?,
            ))),
            "arrow_function" => Ok(Self::ArrowFunction(::std::boxed::Box::new(
                ::treesitter_types::runtime::maybe_grow_stack(|| {
                    <ArrowFunction as ::treesitter_types::FromNode>::from_node(node, src)
                })?,
            ))),
            "call_expression" => Ok(Self::CallExpression(::std::boxed::Box::new(
                ::treesitter_types::runtime::maybe_grow_stack(|| {
                    <CallExpression as ::treesitter_types::FromNode>::from_node(node, src)
                })?,
            ))),
            "class" => Ok(Self::Class(::std::boxed::Box::new(
                ::treesitter_types::runtime::maybe_grow_stack(|| {
                    <Class as ::treesitter_types::FromNode>::from_node(node, src)
                })?,
            ))),
            "false" => Ok(Self::False(::std::boxed::Box::new(
                ::treesitter_types::runtime::maybe_grow_stack(|| {
                    <False as ::treesitter_types::FromNode>::from_node(node, src)
                })?,
            ))),
            "function_expression" => Ok(Self::FunctionExpression(::std::boxed::Box::new(
                ::treesitter_types::runtime::maybe_grow_stack(|| {
                    <FunctionExpression as ::treesitter_types::FromNode>::from_node(node, src)
                })?,
            ))),
            "generator_function" => Ok(Self::GeneratorFunction(::std::boxed::Box::new(
                ::treesitter_types::runtime::maybe_grow_stack(|| {
                    <GeneratorFunction as ::treesitter_types::FromNode>::from_node(node, src)
                })?,
            ))),
            "identifier" => Ok(Self::Identifier(::std::boxed::Box::new(
                ::treesitter_types::runtime::maybe_grow_stack(|| {
                    <Identifier as ::treesitter_types::FromNode>::from_node(node, src)
                })?,
            ))),
            "member_expression" => Ok(Self::MemberExpression(::std::boxed::Box::new(
                ::treesitter_types::runtime::maybe_grow_stack(|| {
                    <MemberExpression as ::treesitter_types::FromNode>::from_node(node, src)
                })?,
            ))),
            "meta_property" => Ok(Self::MetaProperty(::std::boxed::Box::new(
                ::treesitter_types::runtime::maybe_grow_stack(|| {
                    <MetaProperty as ::treesitter_types::FromNode>::from_node(node, src)
                })?,
            ))),
            "null" => Ok(Self::Null(::std::boxed::Box::new(
                ::treesitter_types::runtime::maybe_grow_stack(|| {
                    <Null as ::treesitter_types::FromNode>::from_node(node, src)
                })?,
            ))),
            "number" => Ok(Self::Number(::std::boxed::Box::new(
                ::treesitter_types::runtime::maybe_grow_stack(|| {
                    <Number as ::treesitter_types::FromNode>::from_node(node, src)
                })?,
            ))),
            "object" => Ok(Self::Object(::std::boxed::Box::new(
                ::treesitter_types::runtime::maybe_grow_stack(|| {
                    <Object as ::treesitter_types::FromNode>::from_node(node, src)
                })?,
            ))),
            "parenthesized_expression" => Ok(Self::ParenthesizedExpression(
                ::std::boxed::Box::new(::treesitter_types::runtime::maybe_grow_stack(|| {
                    <ParenthesizedExpression as ::treesitter_types::FromNode>::from_node(node, src)
                })?),
            )),
            "regex" => Ok(Self::Regex(::std::boxed::Box::new(
                ::treesitter_types::runtime::maybe_grow_stack(|| {
                    <Regex as ::treesitter_types::FromNode>::from_node(node, src)
                })?,
            ))),
            "string" => Ok(Self::String(::std::boxed::Box::new(
                ::treesitter_types::runtime::maybe_grow_stack(|| {
                    <String as ::treesitter_types::FromNode>::from_node(node, src)
                })?,
            ))),
            "subscript_expression" => Ok(Self::SubscriptExpression(::std::boxed::Box::new(
                ::treesitter_types::runtime::maybe_grow_stack(|| {
                    <SubscriptExpression as ::treesitter_types::FromNode>::from_node(node, src)
                })?,
            ))),
            "super" => Ok(Self::Super(::std::boxed::Box::new(
                ::treesitter_types::runtime::maybe_grow_stack(|| {
                    <Super as ::treesitter_types::FromNode>::from_node(node, src)
                })?,
            ))),
            "template_string" => Ok(Self::TemplateString(::std::boxed::Box::new(
                ::treesitter_types::runtime::maybe_grow_stack(|| {
                    <TemplateString as ::treesitter_types::FromNode>::from_node(node, src)
                })?,
            ))),
            "this" => Ok(Self::This(::std::boxed::Box::new(
                ::treesitter_types::runtime::maybe_grow_stack(|| {
                    <This as ::treesitter_types::FromNode>::from_node(node, src)
                })?,
            ))),
            "true" => Ok(Self::True(::std::boxed::Box::new(
                ::treesitter_types::runtime::maybe_grow_stack(|| {
                    <True as ::treesitter_types::FromNode>::from_node(node, src)
                })?,
            ))),
            "undefined" => Ok(Self::Undefined(::std::boxed::Box::new(
                ::treesitter_types::runtime::maybe_grow_stack(|| {
                    <Undefined as ::treesitter_types::FromNode>::from_node(node, src)
                })?,
            ))),
            other => Err(::treesitter_types::ParseError::unexpected_kind(other, node)),
        }
    }
}
impl ::treesitter_types::Spanned for PrimaryExpression<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        match self {
            Self::Array(inner) => inner.span(),
            Self::ArrowFunction(inner) => inner.span(),
            Self::CallExpression(inner) => inner.span(),
            Self::Class(inner) => inner.span(),
            Self::False(inner) => inner.span(),
            Self::FunctionExpression(inner) => inner.span(),
            Self::GeneratorFunction(inner) => inner.span(),
            Self::Identifier(inner) => inner.span(),
            Self::MemberExpression(inner) => inner.span(),
            Self::MetaProperty(inner) => inner.span(),
            Self::Null(inner) => inner.span(),
            Self::Number(inner) => inner.span(),
            Self::Object(inner) => inner.span(),
            Self::ParenthesizedExpression(inner) => inner.span(),
            Self::Regex(inner) => inner.span(),
            Self::String(inner) => inner.span(),
            Self::SubscriptExpression(inner) => inner.span(),
            Self::Super(inner) => inner.span(),
            Self::TemplateString(inner) => inner.span(),
            Self::This(inner) => inner.span(),
            Self::True(inner) => inner.span(),
            Self::Undefined(inner) => inner.span(),
        }
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Statement<'tree> {
    BreakStatement(::std::boxed::Box<BreakStatement<'tree>>),
    ContinueStatement(::std::boxed::Box<ContinueStatement<'tree>>),
    DebuggerStatement(::std::boxed::Box<DebuggerStatement<'tree>>),
    Declaration(::std::boxed::Box<Declaration<'tree>>),
    DoStatement(::std::boxed::Box<DoStatement<'tree>>),
    EmptyStatement(::std::boxed::Box<EmptyStatement<'tree>>),
    ExportStatement(::std::boxed::Box<ExportStatement<'tree>>),
    ExpressionStatement(::std::boxed::Box<ExpressionStatement<'tree>>),
    ForInStatement(::std::boxed::Box<ForInStatement<'tree>>),
    ForStatement(::std::boxed::Box<ForStatement<'tree>>),
    IfStatement(::std::boxed::Box<IfStatement<'tree>>),
    ImportStatement(::std::boxed::Box<ImportStatement<'tree>>),
    LabeledStatement(::std::boxed::Box<LabeledStatement<'tree>>),
    ReturnStatement(::std::boxed::Box<ReturnStatement<'tree>>),
    StatementBlock(::std::boxed::Box<StatementBlock<'tree>>),
    SwitchStatement(::std::boxed::Box<SwitchStatement<'tree>>),
    ThrowStatement(::std::boxed::Box<ThrowStatement<'tree>>),
    TryStatement(::std::boxed::Box<TryStatement<'tree>>),
    WhileStatement(::std::boxed::Box<WhileStatement<'tree>>),
    WithStatement(::std::boxed::Box<WithStatement<'tree>>),
}
impl<'tree> ::treesitter_types::FromNode<'tree> for Statement<'tree> {
    #[allow(clippy::collapsible_else_if)]
    fn from_node(
        node: ::treesitter_types::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        match node.kind() {
            "break_statement" => Ok(Self::BreakStatement(::std::boxed::Box::new(
                ::treesitter_types::runtime::maybe_grow_stack(|| {
                    <BreakStatement as ::treesitter_types::FromNode>::from_node(node, src)
                })?,
            ))),
            "continue_statement" => Ok(Self::ContinueStatement(::std::boxed::Box::new(
                ::treesitter_types::runtime::maybe_grow_stack(|| {
                    <ContinueStatement as ::treesitter_types::FromNode>::from_node(node, src)
                })?,
            ))),
            "debugger_statement" => Ok(Self::DebuggerStatement(::std::boxed::Box::new(
                ::treesitter_types::runtime::maybe_grow_stack(|| {
                    <DebuggerStatement as ::treesitter_types::FromNode>::from_node(node, src)
                })?,
            ))),
            "do_statement" => Ok(Self::DoStatement(::std::boxed::Box::new(
                ::treesitter_types::runtime::maybe_grow_stack(|| {
                    <DoStatement as ::treesitter_types::FromNode>::from_node(node, src)
                })?,
            ))),
            "empty_statement" => Ok(Self::EmptyStatement(::std::boxed::Box::new(
                ::treesitter_types::runtime::maybe_grow_stack(|| {
                    <EmptyStatement as ::treesitter_types::FromNode>::from_node(node, src)
                })?,
            ))),
            "export_statement" => Ok(Self::ExportStatement(::std::boxed::Box::new(
                ::treesitter_types::runtime::maybe_grow_stack(|| {
                    <ExportStatement as ::treesitter_types::FromNode>::from_node(node, src)
                })?,
            ))),
            "expression_statement" => Ok(Self::ExpressionStatement(::std::boxed::Box::new(
                ::treesitter_types::runtime::maybe_grow_stack(|| {
                    <ExpressionStatement as ::treesitter_types::FromNode>::from_node(node, src)
                })?,
            ))),
            "for_in_statement" => Ok(Self::ForInStatement(::std::boxed::Box::new(
                ::treesitter_types::runtime::maybe_grow_stack(|| {
                    <ForInStatement as ::treesitter_types::FromNode>::from_node(node, src)
                })?,
            ))),
            "for_statement" => Ok(Self::ForStatement(::std::boxed::Box::new(
                ::treesitter_types::runtime::maybe_grow_stack(|| {
                    <ForStatement as ::treesitter_types::FromNode>::from_node(node, src)
                })?,
            ))),
            "if_statement" => Ok(Self::IfStatement(::std::boxed::Box::new(
                ::treesitter_types::runtime::maybe_grow_stack(|| {
                    <IfStatement as ::treesitter_types::FromNode>::from_node(node, src)
                })?,
            ))),
            "import_statement" => Ok(Self::ImportStatement(::std::boxed::Box::new(
                ::treesitter_types::runtime::maybe_grow_stack(|| {
                    <ImportStatement as ::treesitter_types::FromNode>::from_node(node, src)
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
            "statement_block" => Ok(Self::StatementBlock(::std::boxed::Box::new(
                ::treesitter_types::runtime::maybe_grow_stack(|| {
                    <StatementBlock as ::treesitter_types::FromNode>::from_node(node, src)
                })?,
            ))),
            "switch_statement" => Ok(Self::SwitchStatement(::std::boxed::Box::new(
                ::treesitter_types::runtime::maybe_grow_stack(|| {
                    <SwitchStatement as ::treesitter_types::FromNode>::from_node(node, src)
                })?,
            ))),
            "throw_statement" => Ok(Self::ThrowStatement(::std::boxed::Box::new(
                ::treesitter_types::runtime::maybe_grow_stack(|| {
                    <ThrowStatement as ::treesitter_types::FromNode>::from_node(node, src)
                })?,
            ))),
            "try_statement" => Ok(Self::TryStatement(::std::boxed::Box::new(
                ::treesitter_types::runtime::maybe_grow_stack(|| {
                    <TryStatement as ::treesitter_types::FromNode>::from_node(node, src)
                })?,
            ))),
            "while_statement" => Ok(Self::WhileStatement(::std::boxed::Box::new(
                ::treesitter_types::runtime::maybe_grow_stack(|| {
                    <WhileStatement as ::treesitter_types::FromNode>::from_node(node, src)
                })?,
            ))),
            "with_statement" => Ok(Self::WithStatement(::std::boxed::Box::new(
                ::treesitter_types::runtime::maybe_grow_stack(|| {
                    <WithStatement as ::treesitter_types::FromNode>::from_node(node, src)
                })?,
            ))),
            _other => {
                if let Ok(v) = ::treesitter_types::runtime::maybe_grow_stack(|| {
                    <Declaration as ::treesitter_types::FromNode>::from_node(node, src)
                }) {
                    Ok(Self::Declaration(::std::boxed::Box::new(v)))
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
            Self::BreakStatement(inner) => inner.span(),
            Self::ContinueStatement(inner) => inner.span(),
            Self::DebuggerStatement(inner) => inner.span(),
            Self::Declaration(inner) => inner.span(),
            Self::DoStatement(inner) => inner.span(),
            Self::EmptyStatement(inner) => inner.span(),
            Self::ExportStatement(inner) => inner.span(),
            Self::ExpressionStatement(inner) => inner.span(),
            Self::ForInStatement(inner) => inner.span(),
            Self::ForStatement(inner) => inner.span(),
            Self::IfStatement(inner) => inner.span(),
            Self::ImportStatement(inner) => inner.span(),
            Self::LabeledStatement(inner) => inner.span(),
            Self::ReturnStatement(inner) => inner.span(),
            Self::StatementBlock(inner) => inner.span(),
            Self::SwitchStatement(inner) => inner.span(),
            Self::ThrowStatement(inner) => inner.span(),
            Self::TryStatement(inner) => inner.span(),
            Self::WhileStatement(inner) => inner.span(),
            Self::WithStatement(inner) => inner.span(),
        }
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
        node: ::treesitter_types::tree_sitter::Node<'tree>,
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
pub struct Array<'tree> {
    pub span: ::treesitter_types::Span,
    pub children: ::std::vec::Vec<ArrayChildren<'tree>>,
}
impl<'tree> ::treesitter_types::FromNode<'tree> for Array<'tree> {
    #[allow(clippy::match_single_binding, clippy::suspicious_else_formatting)]
    fn from_node(
        node: ::treesitter_types::tree_sitter::Node<'tree>,
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
                    items.push(::treesitter_types::runtime::maybe_grow_stack(|| {
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
    pub children: ::std::vec::Vec<ArrayPatternChildren<'tree>>,
}
impl<'tree> ::treesitter_types::FromNode<'tree> for ArrayPattern<'tree> {
    #[allow(clippy::match_single_binding, clippy::suspicious_else_formatting)]
    fn from_node(
        node: ::treesitter_types::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        debug_assert_eq!(node.kind(), "array_pattern");
        Ok(Self {
            span: ::treesitter_types::Span::from(node),
            children: {
                #[allow(clippy::suspicious_else_formatting)]
                let non_field_children = {
                    let mut cursor = node.walk();
                    let mut result = ::std::vec::Vec::new();
                    if cursor.goto_first_child() {
                        loop {
                            if cursor.field_name().is_none()
                                && cursor.node().is_named()
                                && !cursor.node().is_extra()
                            {
                                result.push(cursor.node());
                            }
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
pub struct ArrowFunction<'tree> {
    pub span: ::treesitter_types::Span,
    pub body: ArrowFunctionBody<'tree>,
    pub parameter: ::core::option::Option<Identifier<'tree>>,
    pub parameters: ::core::option::Option<FormalParameters<'tree>>,
}
impl<'tree> ::treesitter_types::FromNode<'tree> for ArrowFunction<'tree> {
    #[allow(clippy::match_single_binding, clippy::suspicious_else_formatting)]
    fn from_node(
        node: ::treesitter_types::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        debug_assert_eq!(node.kind(), "arrow_function");
        Ok(Self {
            span: ::treesitter_types::Span::from(node),
            body: {
                let child = node
                    .child_by_field_name("body")
                    .ok_or_else(|| ::treesitter_types::ParseError::missing_field("body", node))?;
                ::treesitter_types::runtime::maybe_grow_stack(|| {
                    <ArrowFunctionBody as ::treesitter_types::FromNode>::from_node(child, src)
                })?
            },
            parameter: match node.child_by_field_name("parameter") {
                Some(child) => Some(::treesitter_types::runtime::maybe_grow_stack(|| {
                    <Identifier as ::treesitter_types::FromNode>::from_node(child, src)
                })?),
                None => None,
            },
            parameters: match node.child_by_field_name("parameters") {
                Some(child) => Some(::treesitter_types::runtime::maybe_grow_stack(|| {
                    <FormalParameters as ::treesitter_types::FromNode>::from_node(child, src)
                })?),
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
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct AssignmentExpression<'tree> {
    pub span: ::treesitter_types::Span,
    pub left: AssignmentExpressionLeft<'tree>,
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
pub struct AssignmentPattern<'tree> {
    pub span: ::treesitter_types::Span,
    pub left: Pattern<'tree>,
    pub right: Expression<'tree>,
}
impl<'tree> ::treesitter_types::FromNode<'tree> for AssignmentPattern<'tree> {
    #[allow(clippy::match_single_binding, clippy::suspicious_else_formatting)]
    fn from_node(
        node: ::treesitter_types::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        debug_assert_eq!(node.kind(), "assignment_pattern");
        Ok(Self {
            span: ::treesitter_types::Span::from(node),
            left: {
                let child = node
                    .child_by_field_name("left")
                    .ok_or_else(|| ::treesitter_types::ParseError::missing_field("left", node))?;
                ::treesitter_types::runtime::maybe_grow_stack(|| {
                    <Pattern as ::treesitter_types::FromNode>::from_node(child, src)
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
impl ::treesitter_types::Spanned for AssignmentPattern<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        self.span
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct AugmentedAssignmentExpression<'tree> {
    pub span: ::treesitter_types::Span,
    pub left: AugmentedAssignmentExpressionLeft<'tree>,
    pub operator: AugmentedAssignmentExpressionOperator,
    pub right: Expression<'tree>,
}
impl<'tree> ::treesitter_types::FromNode<'tree> for AugmentedAssignmentExpression<'tree> {
    #[allow(clippy::match_single_binding, clippy::suspicious_else_formatting)]
    fn from_node(
        node: ::treesitter_types::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        debug_assert_eq!(node.kind(), "augmented_assignment_expression");
        Ok(Self {
            span: ::treesitter_types::Span::from(node),
            left: {
                let child = node
                    .child_by_field_name("left")
                    .ok_or_else(|| ::treesitter_types::ParseError::missing_field("left", node))?;
                ::treesitter_types::runtime::maybe_grow_stack(|| {
                    <AugmentedAssignmentExpressionLeft as ::treesitter_types::FromNode>::from_node(
                        child, src,
                    )
                })?
            },
            operator: {
                let child = node.child_by_field_name("operator").ok_or_else(|| {
                    ::treesitter_types::ParseError::missing_field("operator", node)
                })?;
                ::treesitter_types::runtime::maybe_grow_stack(|| {
                    <AugmentedAssignmentExpressionOperator as ::treesitter_types::FromNode>::from_node(
                    child,
                    src,
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
impl ::treesitter_types::Spanned for AugmentedAssignmentExpression<'_> {
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
        node: ::treesitter_types::tree_sitter::Node<'tree>,
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
pub struct BinaryExpression<'tree> {
    pub span: ::treesitter_types::Span,
    pub left: BinaryExpressionLeft<'tree>,
    pub operator: BinaryExpressionOperator,
    pub right: Expression<'tree>,
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
pub struct BreakStatement<'tree> {
    pub span: ::treesitter_types::Span,
    pub label: ::core::option::Option<StatementIdentifier<'tree>>,
}
impl<'tree> ::treesitter_types::FromNode<'tree> for BreakStatement<'tree> {
    #[allow(clippy::match_single_binding, clippy::suspicious_else_formatting)]
    fn from_node(
        node: ::treesitter_types::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        debug_assert_eq!(node.kind(), "break_statement");
        Ok(Self {
            span: ::treesitter_types::Span::from(node),
            label: match node.child_by_field_name("label") {
                Some(child) => Some(::treesitter_types::runtime::maybe_grow_stack(|| {
                    <StatementIdentifier as ::treesitter_types::FromNode>::from_node(child, src)
                })?),
                None => None,
            },
        })
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
    pub arguments: CallExpressionArguments<'tree>,
    pub function: CallExpressionFunction<'tree>,
    pub optional_chain: ::core::option::Option<OptionalChain<'tree>>,
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
                    <CallExpressionArguments as ::treesitter_types::FromNode>::from_node(child, src)
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
            optional_chain: match node.child_by_field_name("optional_chain") {
                Some(child) => Some(::treesitter_types::runtime::maybe_grow_stack(|| {
                    <OptionalChain as ::treesitter_types::FromNode>::from_node(child, src)
                })?),
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
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CatchClause<'tree> {
    pub span: ::treesitter_types::Span,
    pub body: StatementBlock<'tree>,
    pub parameter: ::core::option::Option<CatchClauseParameter<'tree>>,
}
impl<'tree> ::treesitter_types::FromNode<'tree> for CatchClause<'tree> {
    #[allow(clippy::match_single_binding, clippy::suspicious_else_formatting)]
    fn from_node(
        node: ::treesitter_types::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        debug_assert_eq!(node.kind(), "catch_clause");
        Ok(Self {
            span: ::treesitter_types::Span::from(node),
            body: {
                let child = node
                    .child_by_field_name("body")
                    .ok_or_else(|| ::treesitter_types::ParseError::missing_field("body", node))?;
                ::treesitter_types::runtime::maybe_grow_stack(|| {
                    <StatementBlock as ::treesitter_types::FromNode>::from_node(child, src)
                })?
            },
            parameter: match node.child_by_field_name("parameter") {
                Some(child) => Some(::treesitter_types::runtime::maybe_grow_stack(|| {
                    <CatchClauseParameter as ::treesitter_types::FromNode>::from_node(child, src)
                })?),
                None => None,
            },
        })
    }
}
impl ::treesitter_types::Spanned for CatchClause<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        self.span
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Class<'tree> {
    pub span: ::treesitter_types::Span,
    pub body: ClassBody<'tree>,
    pub decorator: ::std::vec::Vec<Decorator<'tree>>,
    pub name: ::core::option::Option<Identifier<'tree>>,
    pub children: ::core::option::Option<ClassHeritage<'tree>>,
}
impl<'tree> ::treesitter_types::FromNode<'tree> for Class<'tree> {
    #[allow(clippy::match_single_binding, clippy::suspicious_else_formatting)]
    fn from_node(
        node: ::treesitter_types::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        debug_assert_eq!(node.kind(), "class");
        Ok(Self {
            span: ::treesitter_types::Span::from(node),
            body: {
                let child = node
                    .child_by_field_name("body")
                    .ok_or_else(|| ::treesitter_types::ParseError::missing_field("body", node))?;
                ::treesitter_types::runtime::maybe_grow_stack(|| {
                    <ClassBody as ::treesitter_types::FromNode>::from_node(child, src)
                })?
            },
            decorator: {
                let mut cursor = node.walk();
                let mut items = ::std::vec::Vec::new();
                for child in node.children_by_field_name("decorator", &mut cursor) {
                    items.push(::treesitter_types::runtime::maybe_grow_stack(|| {
                        <Decorator as ::treesitter_types::FromNode>::from_node(child, src)
                    })?);
                }
                items
            },
            name: match node.child_by_field_name("name") {
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
                        <ClassHeritage as ::treesitter_types::FromNode>::from_node(child, src)
                    })?),
                    None => None,
                }
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
pub struct ClassBody<'tree> {
    pub span: ::treesitter_types::Span,
    pub member: ::std::vec::Vec<ClassBodyMember<'tree>>,
}
impl<'tree> ::treesitter_types::FromNode<'tree> for ClassBody<'tree> {
    #[allow(clippy::match_single_binding, clippy::suspicious_else_formatting)]
    fn from_node(
        node: ::treesitter_types::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        debug_assert_eq!(node.kind(), "class_body");
        Ok(Self {
            span: ::treesitter_types::Span::from(node),
            member: {
                let mut cursor = node.walk();
                let mut items = ::std::vec::Vec::new();
                for child in node.children_by_field_name("member", &mut cursor) {
                    items.push(::treesitter_types::runtime::maybe_grow_stack(|| {
                        <ClassBodyMember as ::treesitter_types::FromNode>::from_node(child, src)
                    })?);
                }
                items
            },
        })
    }
}
impl ::treesitter_types::Spanned for ClassBody<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        self.span
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ClassDeclaration<'tree> {
    pub span: ::treesitter_types::Span,
    pub body: ClassBody<'tree>,
    pub decorator: ::std::vec::Vec<Decorator<'tree>>,
    pub name: Identifier<'tree>,
    pub children: ::core::option::Option<ClassHeritage<'tree>>,
}
impl<'tree> ::treesitter_types::FromNode<'tree> for ClassDeclaration<'tree> {
    #[allow(clippy::match_single_binding, clippy::suspicious_else_formatting)]
    fn from_node(
        node: ::treesitter_types::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        debug_assert_eq!(node.kind(), "class_declaration");
        Ok(Self {
            span: ::treesitter_types::Span::from(node),
            body: {
                let child = node
                    .child_by_field_name("body")
                    .ok_or_else(|| ::treesitter_types::ParseError::missing_field("body", node))?;
                ::treesitter_types::runtime::maybe_grow_stack(|| {
                    <ClassBody as ::treesitter_types::FromNode>::from_node(child, src)
                })?
            },
            decorator: {
                let mut cursor = node.walk();
                let mut items = ::std::vec::Vec::new();
                for child in node.children_by_field_name("decorator", &mut cursor) {
                    items.push(::treesitter_types::runtime::maybe_grow_stack(|| {
                        <Decorator as ::treesitter_types::FromNode>::from_node(child, src)
                    })?);
                }
                items
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
                        <ClassHeritage as ::treesitter_types::FromNode>::from_node(child, src)
                    })?),
                    None => None,
                }
            },
        })
    }
}
impl ::treesitter_types::Spanned for ClassDeclaration<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        self.span
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ClassHeritage<'tree> {
    pub span: ::treesitter_types::Span,
    pub children: Expression<'tree>,
}
impl<'tree> ::treesitter_types::FromNode<'tree> for ClassHeritage<'tree> {
    #[allow(clippy::match_single_binding, clippy::suspicious_else_formatting)]
    fn from_node(
        node: ::treesitter_types::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        debug_assert_eq!(node.kind(), "class_heritage");
        Ok(Self {
            span: ::treesitter_types::Span::from(node),
            children: {
                #[allow(clippy::suspicious_else_formatting)]
                let non_field_children = {
                    let mut cursor = node.walk();
                    let mut result = ::std::vec::Vec::new();
                    if cursor.goto_first_child() {
                        loop {
                            if cursor.field_name().is_none()
                                && cursor.node().is_named()
                                && !cursor.node().is_extra()
                            {
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
impl ::treesitter_types::Spanned for ClassHeritage<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        self.span
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ClassStaticBlock<'tree> {
    pub span: ::treesitter_types::Span,
    pub body: StatementBlock<'tree>,
}
impl<'tree> ::treesitter_types::FromNode<'tree> for ClassStaticBlock<'tree> {
    #[allow(clippy::match_single_binding, clippy::suspicious_else_formatting)]
    fn from_node(
        node: ::treesitter_types::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        debug_assert_eq!(node.kind(), "class_static_block");
        Ok(Self {
            span: ::treesitter_types::Span::from(node),
            body: {
                let child = node
                    .child_by_field_name("body")
                    .ok_or_else(|| ::treesitter_types::ParseError::missing_field("body", node))?;
                ::treesitter_types::runtime::maybe_grow_stack(|| {
                    <StatementBlock as ::treesitter_types::FromNode>::from_node(child, src)
                })?
            },
        })
    }
}
impl ::treesitter_types::Spanned for ClassStaticBlock<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        self.span
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ComputedPropertyName<'tree> {
    pub span: ::treesitter_types::Span,
    pub children: Expression<'tree>,
}
impl<'tree> ::treesitter_types::FromNode<'tree> for ComputedPropertyName<'tree> {
    #[allow(clippy::match_single_binding, clippy::suspicious_else_formatting)]
    fn from_node(
        node: ::treesitter_types::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        debug_assert_eq!(node.kind(), "computed_property_name");
        Ok(Self {
            span: ::treesitter_types::Span::from(node),
            children: {
                #[allow(clippy::suspicious_else_formatting)]
                let non_field_children = {
                    let mut cursor = node.walk();
                    let mut result = ::std::vec::Vec::new();
                    if cursor.goto_first_child() {
                        loop {
                            if cursor.field_name().is_none()
                                && cursor.node().is_named()
                                && !cursor.node().is_extra()
                            {
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
impl ::treesitter_types::Spanned for ComputedPropertyName<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        self.span
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ContinueStatement<'tree> {
    pub span: ::treesitter_types::Span,
    pub label: ::core::option::Option<StatementIdentifier<'tree>>,
}
impl<'tree> ::treesitter_types::FromNode<'tree> for ContinueStatement<'tree> {
    #[allow(clippy::match_single_binding, clippy::suspicious_else_formatting)]
    fn from_node(
        node: ::treesitter_types::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        debug_assert_eq!(node.kind(), "continue_statement");
        Ok(Self {
            span: ::treesitter_types::Span::from(node),
            label: match node.child_by_field_name("label") {
                Some(child) => Some(::treesitter_types::runtime::maybe_grow_stack(|| {
                    <StatementIdentifier as ::treesitter_types::FromNode>::from_node(child, src)
                })?),
                None => None,
            },
        })
    }
}
impl ::treesitter_types::Spanned for ContinueStatement<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        self.span
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct DebuggerStatement<'tree> {
    pub span: ::treesitter_types::Span,
    text: &'tree str,
}
impl<'tree> ::treesitter_types::FromNode<'tree> for DebuggerStatement<'tree> {
    fn from_node(
        node: ::treesitter_types::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        debug_assert_eq!(node.kind(), "debugger_statement");
        Ok(Self {
            span: ::treesitter_types::Span::from(node),
            text: node.utf8_text(src)?,
        })
    }
}
impl<'tree> ::treesitter_types::LeafNode<'tree> for DebuggerStatement<'tree> {
    fn text(&self) -> &'tree str {
        self.text
    }
}
impl ::treesitter_types::Spanned for DebuggerStatement<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        self.span
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Decorator<'tree> {
    pub span: ::treesitter_types::Span,
    pub children: DecoratorChildren<'tree>,
}
impl<'tree> ::treesitter_types::FromNode<'tree> for Decorator<'tree> {
    #[allow(clippy::match_single_binding, clippy::suspicious_else_formatting)]
    fn from_node(
        node: ::treesitter_types::tree_sitter::Node<'tree>,
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
                                        ::treesitter_types::runtime::maybe_grow_stack(|| <DecoratorChildren as ::treesitter_types::FromNode>::from_node(
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
                                            ::treesitter_types::runtime::maybe_grow_stack(|| <DecoratorChildren as ::treesitter_types::FromNode>::from_node(
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
                    <DecoratorChildren as ::treesitter_types::FromNode>::from_node(child, src)
                })?
            },
        })
    }
}
impl ::treesitter_types::Spanned for Decorator<'_> {
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
pub struct EmptyStatement<'tree> {
    pub span: ::treesitter_types::Span,
    text: &'tree str,
}
impl<'tree> ::treesitter_types::FromNode<'tree> for EmptyStatement<'tree> {
    fn from_node(
        node: ::treesitter_types::tree_sitter::Node<'tree>,
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
pub struct ExportClause<'tree> {
    pub span: ::treesitter_types::Span,
    pub children: ::std::vec::Vec<ExportSpecifier<'tree>>,
}
impl<'tree> ::treesitter_types::FromNode<'tree> for ExportClause<'tree> {
    #[allow(clippy::match_single_binding, clippy::suspicious_else_formatting)]
    fn from_node(
        node: ::treesitter_types::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        debug_assert_eq!(node.kind(), "export_clause");
        Ok(Self {
            span: ::treesitter_types::Span::from(node),
            children: {
                #[allow(clippy::suspicious_else_formatting)]
                let non_field_children = {
                    let mut cursor = node.walk();
                    let mut result = ::std::vec::Vec::new();
                    if cursor.goto_first_child() {
                        loop {
                            if cursor.field_name().is_none()
                                && cursor.node().is_named()
                                && !cursor.node().is_extra()
                            {
                                result.push(cursor.node());
                            }
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
                        <ExportSpecifier as ::treesitter_types::FromNode>::from_node(child, src)
                    })?);
                }
                items
            },
        })
    }
}
impl ::treesitter_types::Spanned for ExportClause<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        self.span
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ExportSpecifier<'tree> {
    pub span: ::treesitter_types::Span,
    pub alias: ::core::option::Option<ExportSpecifierAlias<'tree>>,
    pub name: ExportSpecifierName<'tree>,
}
impl<'tree> ::treesitter_types::FromNode<'tree> for ExportSpecifier<'tree> {
    #[allow(clippy::match_single_binding, clippy::suspicious_else_formatting)]
    fn from_node(
        node: ::treesitter_types::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        debug_assert_eq!(node.kind(), "export_specifier");
        Ok(Self {
            span: ::treesitter_types::Span::from(node),
            alias: match node.child_by_field_name("alias") {
                Some(child) => Some(::treesitter_types::runtime::maybe_grow_stack(|| {
                    <ExportSpecifierAlias as ::treesitter_types::FromNode>::from_node(child, src)
                })?),
                None => None,
            },
            name: {
                let child = node
                    .child_by_field_name("name")
                    .ok_or_else(|| ::treesitter_types::ParseError::missing_field("name", node))?;
                ::treesitter_types::runtime::maybe_grow_stack(|| {
                    <ExportSpecifierName as ::treesitter_types::FromNode>::from_node(child, src)
                })?
            },
        })
    }
}
impl ::treesitter_types::Spanned for ExportSpecifier<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        self.span
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ExportStatement<'tree> {
    pub span: ::treesitter_types::Span,
    pub declaration: ::core::option::Option<Declaration<'tree>>,
    pub decorator: ::std::vec::Vec<Decorator<'tree>>,
    pub source: ::core::option::Option<String<'tree>>,
    pub value: ::core::option::Option<Expression<'tree>>,
    pub children: ::core::option::Option<ExportStatementChildren<'tree>>,
}
impl<'tree> ::treesitter_types::FromNode<'tree> for ExportStatement<'tree> {
    #[allow(clippy::match_single_binding, clippy::suspicious_else_formatting)]
    fn from_node(
        node: ::treesitter_types::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        debug_assert_eq!(node.kind(), "export_statement");
        Ok(Self {
            span: ::treesitter_types::Span::from(node),
            declaration: match node.child_by_field_name("declaration") {
                Some(child) => Some(::treesitter_types::runtime::maybe_grow_stack(|| {
                    <Declaration as ::treesitter_types::FromNode>::from_node(child, src)
                })?),
                None => None,
            },
            decorator: {
                let mut cursor = node.walk();
                let mut items = ::std::vec::Vec::new();
                for child in node.children_by_field_name("decorator", &mut cursor) {
                    items.push(::treesitter_types::runtime::maybe_grow_stack(|| {
                        <Decorator as ::treesitter_types::FromNode>::from_node(child, src)
                    })?);
                }
                items
            },
            source: match node.child_by_field_name("source") {
                Some(child) => Some(::treesitter_types::runtime::maybe_grow_stack(|| {
                    <String as ::treesitter_types::FromNode>::from_node(child, src)
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
                        <ExportStatementChildren as ::treesitter_types::FromNode>::from_node(
                            child, src,
                        )
                    })?),
                    None => None,
                }
            },
        })
    }
}
impl ::treesitter_types::Spanned for ExportStatement<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        self.span
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ExpressionStatement<'tree> {
    pub span: ::treesitter_types::Span,
    pub children: ExpressionStatementChildren<'tree>,
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
                                        ::treesitter_types::runtime::maybe_grow_stack(|| <ExpressionStatementChildren as ::treesitter_types::FromNode>::from_node(
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
                                            ::treesitter_types::runtime::maybe_grow_stack(|| <ExpressionStatementChildren as ::treesitter_types::FromNode>::from_node(
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
                    <ExpressionStatementChildren as ::treesitter_types::FromNode>::from_node(
                        child, src,
                    )
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
pub struct FieldDefinition<'tree> {
    pub span: ::treesitter_types::Span,
    pub decorator: ::std::vec::Vec<Decorator<'tree>>,
    pub property: FieldDefinitionProperty<'tree>,
    pub value: ::core::option::Option<Expression<'tree>>,
}
impl<'tree> ::treesitter_types::FromNode<'tree> for FieldDefinition<'tree> {
    #[allow(clippy::match_single_binding, clippy::suspicious_else_formatting)]
    fn from_node(
        node: ::treesitter_types::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        debug_assert_eq!(node.kind(), "field_definition");
        Ok(Self {
            span: ::treesitter_types::Span::from(node),
            decorator: {
                let mut cursor = node.walk();
                let mut items = ::std::vec::Vec::new();
                for child in node.children_by_field_name("decorator", &mut cursor) {
                    items.push(::treesitter_types::runtime::maybe_grow_stack(|| {
                        <Decorator as ::treesitter_types::FromNode>::from_node(child, src)
                    })?);
                }
                items
            },
            property: {
                let child = node.child_by_field_name("property").ok_or_else(|| {
                    ::treesitter_types::ParseError::missing_field("property", node)
                })?;
                ::treesitter_types::runtime::maybe_grow_stack(|| {
                    <FieldDefinitionProperty as ::treesitter_types::FromNode>::from_node(child, src)
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
impl ::treesitter_types::Spanned for FieldDefinition<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        self.span
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct FinallyClause<'tree> {
    pub span: ::treesitter_types::Span,
    pub body: StatementBlock<'tree>,
}
impl<'tree> ::treesitter_types::FromNode<'tree> for FinallyClause<'tree> {
    #[allow(clippy::match_single_binding, clippy::suspicious_else_formatting)]
    fn from_node(
        node: ::treesitter_types::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        debug_assert_eq!(node.kind(), "finally_clause");
        Ok(Self {
            span: ::treesitter_types::Span::from(node),
            body: {
                let child = node
                    .child_by_field_name("body")
                    .ok_or_else(|| ::treesitter_types::ParseError::missing_field("body", node))?;
                ::treesitter_types::runtime::maybe_grow_stack(|| {
                    <StatementBlock as ::treesitter_types::FromNode>::from_node(child, src)
                })?
            },
        })
    }
}
impl ::treesitter_types::Spanned for FinallyClause<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        self.span
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ForInStatement<'tree> {
    pub span: ::treesitter_types::Span,
    pub body: Statement<'tree>,
    pub kind: ::std::vec::Vec<ForInStatementKind>,
    pub left: ForInStatementLeft<'tree>,
    pub operator: ForInStatementOperator,
    pub right: ForInStatementRight<'tree>,
    pub value: ::core::option::Option<Expression<'tree>>,
}
impl<'tree> ::treesitter_types::FromNode<'tree> for ForInStatement<'tree> {
    #[allow(clippy::match_single_binding, clippy::suspicious_else_formatting)]
    fn from_node(
        node: ::treesitter_types::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        debug_assert_eq!(node.kind(), "for_in_statement");
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
            kind: {
                let mut cursor = node.walk();
                let mut items = ::std::vec::Vec::new();
                for child in node.children_by_field_name("kind", &mut cursor) {
                    items.push(::treesitter_types::runtime::maybe_grow_stack(|| {
                        <ForInStatementKind as ::treesitter_types::FromNode>::from_node(child, src)
                    })?);
                }
                items
            },
            left: {
                let child = node
                    .child_by_field_name("left")
                    .ok_or_else(|| ::treesitter_types::ParseError::missing_field("left", node))?;
                ::treesitter_types::runtime::maybe_grow_stack(|| {
                    <ForInStatementLeft as ::treesitter_types::FromNode>::from_node(child, src)
                })?
            },
            operator: {
                let child = node.child_by_field_name("operator").ok_or_else(|| {
                    ::treesitter_types::ParseError::missing_field("operator", node)
                })?;
                ::treesitter_types::runtime::maybe_grow_stack(|| {
                    <ForInStatementOperator as ::treesitter_types::FromNode>::from_node(child, src)
                })?
            },
            right: {
                let child = node
                    .child_by_field_name("right")
                    .ok_or_else(|| ::treesitter_types::ParseError::missing_field("right", node))?;
                ::treesitter_types::runtime::maybe_grow_stack(|| {
                    <ForInStatementRight as ::treesitter_types::FromNode>::from_node(child, src)
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
impl ::treesitter_types::Spanned for ForInStatement<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        self.span
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ForStatement<'tree> {
    pub span: ::treesitter_types::Span,
    pub body: Statement<'tree>,
    pub condition: ::std::vec::Vec<ForStatementCondition<'tree>>,
    pub increment: ::core::option::Option<ForStatementIncrement<'tree>>,
    pub initializer: ForStatementInitializer<'tree>,
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
            condition: {
                let mut cursor = node.walk();
                let mut items = ::std::vec::Vec::new();
                for child in node.children_by_field_name("condition", &mut cursor) {
                    items.push(::treesitter_types::runtime::maybe_grow_stack(|| {
                        <ForStatementCondition as ::treesitter_types::FromNode>::from_node(
                            child, src,
                        )
                    })?);
                }
                items
            },
            increment: match node.child_by_field_name("increment") {
                Some(child) => Some(::treesitter_types::runtime::maybe_grow_stack(|| {
                    <ForStatementIncrement as ::treesitter_types::FromNode>::from_node(child, src)
                })?),
                None => None,
            },
            initializer: {
                let child = node.child_by_field_name("initializer").ok_or_else(|| {
                    ::treesitter_types::ParseError::missing_field("initializer", node)
                })?;
                ::treesitter_types::runtime::maybe_grow_stack(|| {
                    <ForStatementInitializer as ::treesitter_types::FromNode>::from_node(child, src)
                })?
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
pub struct FormalParameters<'tree> {
    pub span: ::treesitter_types::Span,
    pub children: ::std::vec::Vec<FormalParametersChildren<'tree>>,
}
impl<'tree> ::treesitter_types::FromNode<'tree> for FormalParameters<'tree> {
    #[allow(clippy::match_single_binding, clippy::suspicious_else_formatting)]
    fn from_node(
        node: ::treesitter_types::tree_sitter::Node<'tree>,
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
                    items.push(::treesitter_types::runtime::maybe_grow_stack(|| {
                        <FormalParametersChildren as ::treesitter_types::FromNode>::from_node(
                            child, src,
                        )
                    })?);
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
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct FunctionDeclaration<'tree> {
    pub span: ::treesitter_types::Span,
    pub body: StatementBlock<'tree>,
    pub name: Identifier<'tree>,
    pub parameters: FormalParameters<'tree>,
}
impl<'tree> ::treesitter_types::FromNode<'tree> for FunctionDeclaration<'tree> {
    #[allow(clippy::match_single_binding, clippy::suspicious_else_formatting)]
    fn from_node(
        node: ::treesitter_types::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        debug_assert_eq!(node.kind(), "function_declaration");
        Ok(Self {
            span: ::treesitter_types::Span::from(node),
            body: {
                let child = node
                    .child_by_field_name("body")
                    .ok_or_else(|| ::treesitter_types::ParseError::missing_field("body", node))?;
                ::treesitter_types::runtime::maybe_grow_stack(|| {
                    <StatementBlock as ::treesitter_types::FromNode>::from_node(child, src)
                })?
            },
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
                    <FormalParameters as ::treesitter_types::FromNode>::from_node(child, src)
                })?
            },
        })
    }
}
impl ::treesitter_types::Spanned for FunctionDeclaration<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        self.span
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct FunctionExpression<'tree> {
    pub span: ::treesitter_types::Span,
    pub body: StatementBlock<'tree>,
    pub name: ::core::option::Option<Identifier<'tree>>,
    pub parameters: FormalParameters<'tree>,
}
impl<'tree> ::treesitter_types::FromNode<'tree> for FunctionExpression<'tree> {
    #[allow(clippy::match_single_binding, clippy::suspicious_else_formatting)]
    fn from_node(
        node: ::treesitter_types::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        debug_assert_eq!(node.kind(), "function_expression");
        Ok(Self {
            span: ::treesitter_types::Span::from(node),
            body: {
                let child = node
                    .child_by_field_name("body")
                    .ok_or_else(|| ::treesitter_types::ParseError::missing_field("body", node))?;
                ::treesitter_types::runtime::maybe_grow_stack(|| {
                    <StatementBlock as ::treesitter_types::FromNode>::from_node(child, src)
                })?
            },
            name: match node.child_by_field_name("name") {
                Some(child) => Some(::treesitter_types::runtime::maybe_grow_stack(|| {
                    <Identifier as ::treesitter_types::FromNode>::from_node(child, src)
                })?),
                None => None,
            },
            parameters: {
                let child = node.child_by_field_name("parameters").ok_or_else(|| {
                    ::treesitter_types::ParseError::missing_field("parameters", node)
                })?;
                ::treesitter_types::runtime::maybe_grow_stack(|| {
                    <FormalParameters as ::treesitter_types::FromNode>::from_node(child, src)
                })?
            },
        })
    }
}
impl ::treesitter_types::Spanned for FunctionExpression<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        self.span
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct GeneratorFunction<'tree> {
    pub span: ::treesitter_types::Span,
    pub body: StatementBlock<'tree>,
    pub name: ::core::option::Option<Identifier<'tree>>,
    pub parameters: FormalParameters<'tree>,
}
impl<'tree> ::treesitter_types::FromNode<'tree> for GeneratorFunction<'tree> {
    #[allow(clippy::match_single_binding, clippy::suspicious_else_formatting)]
    fn from_node(
        node: ::treesitter_types::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        debug_assert_eq!(node.kind(), "generator_function");
        Ok(Self {
            span: ::treesitter_types::Span::from(node),
            body: {
                let child = node
                    .child_by_field_name("body")
                    .ok_or_else(|| ::treesitter_types::ParseError::missing_field("body", node))?;
                ::treesitter_types::runtime::maybe_grow_stack(|| {
                    <StatementBlock as ::treesitter_types::FromNode>::from_node(child, src)
                })?
            },
            name: match node.child_by_field_name("name") {
                Some(child) => Some(::treesitter_types::runtime::maybe_grow_stack(|| {
                    <Identifier as ::treesitter_types::FromNode>::from_node(child, src)
                })?),
                None => None,
            },
            parameters: {
                let child = node.child_by_field_name("parameters").ok_or_else(|| {
                    ::treesitter_types::ParseError::missing_field("parameters", node)
                })?;
                ::treesitter_types::runtime::maybe_grow_stack(|| {
                    <FormalParameters as ::treesitter_types::FromNode>::from_node(child, src)
                })?
            },
        })
    }
}
impl ::treesitter_types::Spanned for GeneratorFunction<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        self.span
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct GeneratorFunctionDeclaration<'tree> {
    pub span: ::treesitter_types::Span,
    pub body: StatementBlock<'tree>,
    pub name: Identifier<'tree>,
    pub parameters: FormalParameters<'tree>,
}
impl<'tree> ::treesitter_types::FromNode<'tree> for GeneratorFunctionDeclaration<'tree> {
    #[allow(clippy::match_single_binding, clippy::suspicious_else_formatting)]
    fn from_node(
        node: ::treesitter_types::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        debug_assert_eq!(node.kind(), "generator_function_declaration");
        Ok(Self {
            span: ::treesitter_types::Span::from(node),
            body: {
                let child = node
                    .child_by_field_name("body")
                    .ok_or_else(|| ::treesitter_types::ParseError::missing_field("body", node))?;
                ::treesitter_types::runtime::maybe_grow_stack(|| {
                    <StatementBlock as ::treesitter_types::FromNode>::from_node(child, src)
                })?
            },
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
                    <FormalParameters as ::treesitter_types::FromNode>::from_node(child, src)
                })?
            },
        })
    }
}
impl ::treesitter_types::Spanned for GeneratorFunctionDeclaration<'_> {
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
pub struct Import<'tree> {
    pub span: ::treesitter_types::Span,
    text: &'tree str,
}
impl<'tree> ::treesitter_types::FromNode<'tree> for Import<'tree> {
    fn from_node(
        node: ::treesitter_types::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        debug_assert_eq!(node.kind(), "import");
        Ok(Self {
            span: ::treesitter_types::Span::from(node),
            text: node.utf8_text(src)?,
        })
    }
}
impl<'tree> ::treesitter_types::LeafNode<'tree> for Import<'tree> {
    fn text(&self) -> &'tree str {
        self.text
    }
}
impl ::treesitter_types::Spanned for Import<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        self.span
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ImportAttribute<'tree> {
    pub span: ::treesitter_types::Span,
    pub children: Object<'tree>,
}
impl<'tree> ::treesitter_types::FromNode<'tree> for ImportAttribute<'tree> {
    #[allow(clippy::match_single_binding, clippy::suspicious_else_formatting)]
    fn from_node(
        node: ::treesitter_types::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        debug_assert_eq!(node.kind(), "import_attribute");
        Ok(Self {
            span: ::treesitter_types::Span::from(node),
            children: {
                #[allow(clippy::suspicious_else_formatting)]
                let non_field_children = {
                    let mut cursor = node.walk();
                    let mut result = ::std::vec::Vec::new();
                    if cursor.goto_first_child() {
                        loop {
                            if cursor.field_name().is_none()
                                && cursor.node().is_named()
                                && !cursor.node().is_extra()
                            {
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
                                        ::treesitter_types::runtime::maybe_grow_stack(|| <Object as ::treesitter_types::FromNode>::from_node(
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
                                            ::treesitter_types::runtime::maybe_grow_stack(|| <Object as ::treesitter_types::FromNode>::from_node(
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
                    <Object as ::treesitter_types::FromNode>::from_node(child, src)
                })?
            },
        })
    }
}
impl ::treesitter_types::Spanned for ImportAttribute<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        self.span
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ImportClause<'tree> {
    pub span: ::treesitter_types::Span,
    pub children: ::std::vec::Vec<ImportClauseChildren<'tree>>,
}
impl<'tree> ::treesitter_types::FromNode<'tree> for ImportClause<'tree> {
    #[allow(clippy::match_single_binding, clippy::suspicious_else_formatting)]
    fn from_node(
        node: ::treesitter_types::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        debug_assert_eq!(node.kind(), "import_clause");
        Ok(Self {
            span: ::treesitter_types::Span::from(node),
            children: {
                #[allow(clippy::suspicious_else_formatting)]
                let non_field_children = {
                    let mut cursor = node.walk();
                    let mut result = ::std::vec::Vec::new();
                    if cursor.goto_first_child() {
                        loop {
                            if cursor.field_name().is_none()
                                && cursor.node().is_named()
                                && !cursor.node().is_extra()
                            {
                                result.push(cursor.node());
                            }
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
                        <ImportClauseChildren as ::treesitter_types::FromNode>::from_node(
                            child, src,
                        )
                    })?);
                }
                items
            },
        })
    }
}
impl ::treesitter_types::Spanned for ImportClause<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        self.span
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ImportSpecifier<'tree> {
    pub span: ::treesitter_types::Span,
    pub alias: ::core::option::Option<Identifier<'tree>>,
    pub name: ImportSpecifierName<'tree>,
}
impl<'tree> ::treesitter_types::FromNode<'tree> for ImportSpecifier<'tree> {
    #[allow(clippy::match_single_binding, clippy::suspicious_else_formatting)]
    fn from_node(
        node: ::treesitter_types::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        debug_assert_eq!(node.kind(), "import_specifier");
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
                    <ImportSpecifierName as ::treesitter_types::FromNode>::from_node(child, src)
                })?
            },
        })
    }
}
impl ::treesitter_types::Spanned for ImportSpecifier<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        self.span
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ImportStatement<'tree> {
    pub span: ::treesitter_types::Span,
    pub source: String<'tree>,
    pub children: ::std::vec::Vec<ImportStatementChildren<'tree>>,
}
impl<'tree> ::treesitter_types::FromNode<'tree> for ImportStatement<'tree> {
    #[allow(clippy::match_single_binding, clippy::suspicious_else_formatting)]
    fn from_node(
        node: ::treesitter_types::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        debug_assert_eq!(node.kind(), "import_statement");
        Ok(Self {
            span: ::treesitter_types::Span::from(node),
            source: {
                let child = node
                    .child_by_field_name("source")
                    .ok_or_else(|| ::treesitter_types::ParseError::missing_field("source", node))?;
                ::treesitter_types::runtime::maybe_grow_stack(|| {
                    <String as ::treesitter_types::FromNode>::from_node(child, src)
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
                        <ImportStatementChildren as ::treesitter_types::FromNode>::from_node(
                            child, src,
                        )
                    })?);
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
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct JsxAttribute<'tree> {
    pub span: ::treesitter_types::Span,
    pub children: ::std::vec::Vec<JsxAttributeChildren<'tree>>,
}
impl<'tree> ::treesitter_types::FromNode<'tree> for JsxAttribute<'tree> {
    #[allow(clippy::match_single_binding, clippy::suspicious_else_formatting)]
    fn from_node(
        node: ::treesitter_types::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        debug_assert_eq!(node.kind(), "jsx_attribute");
        Ok(Self {
            span: ::treesitter_types::Span::from(node),
            children: {
                #[allow(clippy::suspicious_else_formatting)]
                let non_field_children = {
                    let mut cursor = node.walk();
                    let mut result = ::std::vec::Vec::new();
                    if cursor.goto_first_child() {
                        loop {
                            if cursor.field_name().is_none()
                                && cursor.node().is_named()
                                && !cursor.node().is_extra()
                            {
                                result.push(cursor.node());
                            }
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
                        <JsxAttributeChildren as ::treesitter_types::FromNode>::from_node(
                            child, src,
                        )
                    })?);
                }
                items
            },
        })
    }
}
impl ::treesitter_types::Spanned for JsxAttribute<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        self.span
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct JsxClosingElement<'tree> {
    pub span: ::treesitter_types::Span,
    pub name: ::core::option::Option<JsxClosingElementName<'tree>>,
}
impl<'tree> ::treesitter_types::FromNode<'tree> for JsxClosingElement<'tree> {
    #[allow(clippy::match_single_binding, clippy::suspicious_else_formatting)]
    fn from_node(
        node: ::treesitter_types::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        debug_assert_eq!(node.kind(), "jsx_closing_element");
        Ok(Self {
            span: ::treesitter_types::Span::from(node),
            name: match node.child_by_field_name("name") {
                Some(child) => Some(::treesitter_types::runtime::maybe_grow_stack(|| {
                    <JsxClosingElementName as ::treesitter_types::FromNode>::from_node(child, src)
                })?),
                None => None,
            },
        })
    }
}
impl ::treesitter_types::Spanned for JsxClosingElement<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        self.span
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct JsxElement<'tree> {
    pub span: ::treesitter_types::Span,
    pub close_tag: JsxClosingElement<'tree>,
    pub open_tag: JsxOpeningElement<'tree>,
    pub children: ::std::vec::Vec<JsxElementChildren<'tree>>,
}
impl<'tree> ::treesitter_types::FromNode<'tree> for JsxElement<'tree> {
    #[allow(clippy::match_single_binding, clippy::suspicious_else_formatting)]
    fn from_node(
        node: ::treesitter_types::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        debug_assert_eq!(node.kind(), "jsx_element");
        Ok(Self {
            span: ::treesitter_types::Span::from(node),
            close_tag: {
                let child = node.child_by_field_name("close_tag").ok_or_else(|| {
                    ::treesitter_types::ParseError::missing_field("close_tag", node)
                })?;
                ::treesitter_types::runtime::maybe_grow_stack(|| {
                    <JsxClosingElement as ::treesitter_types::FromNode>::from_node(child, src)
                })?
            },
            open_tag: {
                let child = node.child_by_field_name("open_tag").ok_or_else(|| {
                    ::treesitter_types::ParseError::missing_field("open_tag", node)
                })?;
                ::treesitter_types::runtime::maybe_grow_stack(|| {
                    <JsxOpeningElement as ::treesitter_types::FromNode>::from_node(child, src)
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
                        <JsxElementChildren as ::treesitter_types::FromNode>::from_node(child, src)
                    })?);
                }
                items
            },
        })
    }
}
impl ::treesitter_types::Spanned for JsxElement<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        self.span
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct JsxExpression<'tree> {
    pub span: ::treesitter_types::Span,
    pub children: ::core::option::Option<JsxExpressionChildren<'tree>>,
}
impl<'tree> ::treesitter_types::FromNode<'tree> for JsxExpression<'tree> {
    #[allow(clippy::match_single_binding, clippy::suspicious_else_formatting)]
    fn from_node(
        node: ::treesitter_types::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        debug_assert_eq!(node.kind(), "jsx_expression");
        Ok(Self {
            span: ::treesitter_types::Span::from(node),
            children: {
                #[allow(clippy::suspicious_else_formatting)]
                let non_field_children = {
                    let mut cursor = node.walk();
                    let mut result = ::std::vec::Vec::new();
                    if cursor.goto_first_child() {
                        loop {
                            if cursor.field_name().is_none()
                                && cursor.node().is_named()
                                && !cursor.node().is_extra()
                            {
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
                        <JsxExpressionChildren as ::treesitter_types::FromNode>::from_node(
                            child, src,
                        )
                    })?),
                    None => None,
                }
            },
        })
    }
}
impl ::treesitter_types::Spanned for JsxExpression<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        self.span
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct JsxNamespaceName<'tree> {
    pub span: ::treesitter_types::Span,
    pub children: ::std::vec::Vec<Identifier<'tree>>,
}
impl<'tree> ::treesitter_types::FromNode<'tree> for JsxNamespaceName<'tree> {
    #[allow(clippy::match_single_binding, clippy::suspicious_else_formatting)]
    fn from_node(
        node: ::treesitter_types::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        debug_assert_eq!(node.kind(), "jsx_namespace_name");
        Ok(Self {
            span: ::treesitter_types::Span::from(node),
            children: {
                #[allow(clippy::suspicious_else_formatting)]
                let non_field_children = {
                    let mut cursor = node.walk();
                    let mut result = ::std::vec::Vec::new();
                    if cursor.goto_first_child() {
                        loop {
                            if cursor.field_name().is_none()
                                && cursor.node().is_named()
                                && !cursor.node().is_extra()
                            {
                                result.push(cursor.node());
                            }
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
impl ::treesitter_types::Spanned for JsxNamespaceName<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        self.span
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct JsxOpeningElement<'tree> {
    pub span: ::treesitter_types::Span,
    pub attribute: ::std::vec::Vec<JsxOpeningElementAttribute<'tree>>,
    pub name: ::core::option::Option<JsxOpeningElementName<'tree>>,
}
impl<'tree> ::treesitter_types::FromNode<'tree> for JsxOpeningElement<'tree> {
    #[allow(clippy::match_single_binding, clippy::suspicious_else_formatting)]
    fn from_node(
        node: ::treesitter_types::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        debug_assert_eq!(node.kind(), "jsx_opening_element");
        Ok(Self {
            span: ::treesitter_types::Span::from(node),
            attribute: {
                let mut cursor = node.walk();
                let mut items = ::std::vec::Vec::new();
                for child in node.children_by_field_name("attribute", &mut cursor) {
                    items.push(::treesitter_types::runtime::maybe_grow_stack(|| {
                        <JsxOpeningElementAttribute as ::treesitter_types::FromNode>::from_node(
                            child, src,
                        )
                    })?);
                }
                items
            },
            name: match node.child_by_field_name("name") {
                Some(child) => Some(::treesitter_types::runtime::maybe_grow_stack(|| {
                    <JsxOpeningElementName as ::treesitter_types::FromNode>::from_node(child, src)
                })?),
                None => None,
            },
        })
    }
}
impl ::treesitter_types::Spanned for JsxOpeningElement<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        self.span
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct JsxSelfClosingElement<'tree> {
    pub span: ::treesitter_types::Span,
    pub attribute: ::std::vec::Vec<JsxSelfClosingElementAttribute<'tree>>,
    pub name: JsxSelfClosingElementName<'tree>,
}
impl<'tree> ::treesitter_types::FromNode<'tree> for JsxSelfClosingElement<'tree> {
    #[allow(clippy::match_single_binding, clippy::suspicious_else_formatting)]
    fn from_node(
        node: ::treesitter_types::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        debug_assert_eq!(node.kind(), "jsx_self_closing_element");
        Ok(Self {
            span: ::treesitter_types::Span::from(node),
            attribute: {
                let mut cursor = node.walk();
                let mut items = ::std::vec::Vec::new();
                for child in node.children_by_field_name("attribute", &mut cursor) {
                    items.push(::treesitter_types::runtime::maybe_grow_stack(|| {
                        <JsxSelfClosingElementAttribute as ::treesitter_types::FromNode>::from_node(
                            child, src,
                        )
                    })?);
                }
                items
            },
            name: {
                let child = node
                    .child_by_field_name("name")
                    .ok_or_else(|| ::treesitter_types::ParseError::missing_field("name", node))?;
                ::treesitter_types::runtime::maybe_grow_stack(|| {
                    <JsxSelfClosingElementName as ::treesitter_types::FromNode>::from_node(
                        child, src,
                    )
                })?
            },
        })
    }
}
impl ::treesitter_types::Spanned for JsxSelfClosingElement<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        self.span
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct LabeledStatement<'tree> {
    pub span: ::treesitter_types::Span,
    pub body: Statement<'tree>,
    pub label: StatementIdentifier<'tree>,
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
            body: {
                let child = node
                    .child_by_field_name("body")
                    .ok_or_else(|| ::treesitter_types::ParseError::missing_field("body", node))?;
                ::treesitter_types::runtime::maybe_grow_stack(|| {
                    <Statement as ::treesitter_types::FromNode>::from_node(child, src)
                })?
            },
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
impl ::treesitter_types::Spanned for LabeledStatement<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        self.span
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct LexicalDeclaration<'tree> {
    pub span: ::treesitter_types::Span,
    pub kind: LexicalDeclarationKind,
    pub children: ::std::vec::Vec<VariableDeclarator<'tree>>,
}
impl<'tree> ::treesitter_types::FromNode<'tree> for LexicalDeclaration<'tree> {
    #[allow(clippy::match_single_binding, clippy::suspicious_else_formatting)]
    fn from_node(
        node: ::treesitter_types::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        debug_assert_eq!(node.kind(), "lexical_declaration");
        Ok(Self {
            span: ::treesitter_types::Span::from(node),
            kind: {
                let child = node
                    .child_by_field_name("kind")
                    .ok_or_else(|| ::treesitter_types::ParseError::missing_field("kind", node))?;
                ::treesitter_types::runtime::maybe_grow_stack(|| {
                    <LexicalDeclarationKind as ::treesitter_types::FromNode>::from_node(child, src)
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
                        <VariableDeclarator as ::treesitter_types::FromNode>::from_node(child, src)
                    })?);
                }
                items
            },
        })
    }
}
impl ::treesitter_types::Spanned for LexicalDeclaration<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        self.span
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct MemberExpression<'tree> {
    pub span: ::treesitter_types::Span,
    pub object: MemberExpressionObject<'tree>,
    pub optional_chain: ::core::option::Option<OptionalChain<'tree>>,
    pub property: MemberExpressionProperty<'tree>,
}
impl<'tree> ::treesitter_types::FromNode<'tree> for MemberExpression<'tree> {
    #[allow(clippy::match_single_binding, clippy::suspicious_else_formatting)]
    fn from_node(
        node: ::treesitter_types::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        debug_assert_eq!(node.kind(), "member_expression");
        Ok(Self {
            span: ::treesitter_types::Span::from(node),
            object: {
                let child = node
                    .child_by_field_name("object")
                    .ok_or_else(|| ::treesitter_types::ParseError::missing_field("object", node))?;
                ::treesitter_types::runtime::maybe_grow_stack(|| {
                    <MemberExpressionObject as ::treesitter_types::FromNode>::from_node(child, src)
                })?
            },
            optional_chain: match node.child_by_field_name("optional_chain") {
                Some(child) => Some(::treesitter_types::runtime::maybe_grow_stack(|| {
                    <OptionalChain as ::treesitter_types::FromNode>::from_node(child, src)
                })?),
                None => None,
            },
            property: {
                let child = node.child_by_field_name("property").ok_or_else(|| {
                    ::treesitter_types::ParseError::missing_field("property", node)
                })?;
                ::treesitter_types::runtime::maybe_grow_stack(|| {
                    <MemberExpressionProperty as ::treesitter_types::FromNode>::from_node(
                        child, src,
                    )
                })?
            },
        })
    }
}
impl ::treesitter_types::Spanned for MemberExpression<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        self.span
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct MetaProperty<'tree> {
    pub span: ::treesitter_types::Span,
    text: &'tree str,
}
impl<'tree> ::treesitter_types::FromNode<'tree> for MetaProperty<'tree> {
    fn from_node(
        node: ::treesitter_types::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        debug_assert_eq!(node.kind(), "meta_property");
        Ok(Self {
            span: ::treesitter_types::Span::from(node),
            text: node.utf8_text(src)?,
        })
    }
}
impl<'tree> ::treesitter_types::LeafNode<'tree> for MetaProperty<'tree> {
    fn text(&self) -> &'tree str {
        self.text
    }
}
impl ::treesitter_types::Spanned for MetaProperty<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        self.span
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct MethodDefinition<'tree> {
    pub span: ::treesitter_types::Span,
    pub body: StatementBlock<'tree>,
    pub decorator: ::std::vec::Vec<Decorator<'tree>>,
    pub name: MethodDefinitionName<'tree>,
    pub parameters: FormalParameters<'tree>,
}
impl<'tree> ::treesitter_types::FromNode<'tree> for MethodDefinition<'tree> {
    #[allow(clippy::match_single_binding, clippy::suspicious_else_formatting)]
    fn from_node(
        node: ::treesitter_types::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        debug_assert_eq!(node.kind(), "method_definition");
        Ok(Self {
            span: ::treesitter_types::Span::from(node),
            body: {
                let child = node
                    .child_by_field_name("body")
                    .ok_or_else(|| ::treesitter_types::ParseError::missing_field("body", node))?;
                ::treesitter_types::runtime::maybe_grow_stack(|| {
                    <StatementBlock as ::treesitter_types::FromNode>::from_node(child, src)
                })?
            },
            decorator: {
                let mut cursor = node.walk();
                let mut items = ::std::vec::Vec::new();
                for child in node.children_by_field_name("decorator", &mut cursor) {
                    items.push(::treesitter_types::runtime::maybe_grow_stack(|| {
                        <Decorator as ::treesitter_types::FromNode>::from_node(child, src)
                    })?);
                }
                items
            },
            name: {
                let child = node
                    .child_by_field_name("name")
                    .ok_or_else(|| ::treesitter_types::ParseError::missing_field("name", node))?;
                ::treesitter_types::runtime::maybe_grow_stack(|| {
                    <MethodDefinitionName as ::treesitter_types::FromNode>::from_node(child, src)
                })?
            },
            parameters: {
                let child = node.child_by_field_name("parameters").ok_or_else(|| {
                    ::treesitter_types::ParseError::missing_field("parameters", node)
                })?;
                ::treesitter_types::runtime::maybe_grow_stack(|| {
                    <FormalParameters as ::treesitter_types::FromNode>::from_node(child, src)
                })?
            },
        })
    }
}
impl ::treesitter_types::Spanned for MethodDefinition<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        self.span
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct NamedImports<'tree> {
    pub span: ::treesitter_types::Span,
    pub children: ::std::vec::Vec<ImportSpecifier<'tree>>,
}
impl<'tree> ::treesitter_types::FromNode<'tree> for NamedImports<'tree> {
    #[allow(clippy::match_single_binding, clippy::suspicious_else_formatting)]
    fn from_node(
        node: ::treesitter_types::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        debug_assert_eq!(node.kind(), "named_imports");
        Ok(Self {
            span: ::treesitter_types::Span::from(node),
            children: {
                #[allow(clippy::suspicious_else_formatting)]
                let non_field_children = {
                    let mut cursor = node.walk();
                    let mut result = ::std::vec::Vec::new();
                    if cursor.goto_first_child() {
                        loop {
                            if cursor.field_name().is_none()
                                && cursor.node().is_named()
                                && !cursor.node().is_extra()
                            {
                                result.push(cursor.node());
                            }
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
                        <ImportSpecifier as ::treesitter_types::FromNode>::from_node(child, src)
                    })?);
                }
                items
            },
        })
    }
}
impl ::treesitter_types::Spanned for NamedImports<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        self.span
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct NamespaceExport<'tree> {
    pub span: ::treesitter_types::Span,
    pub children: ::core::option::Option<NamespaceExportChildren<'tree>>,
}
impl<'tree> ::treesitter_types::FromNode<'tree> for NamespaceExport<'tree> {
    #[allow(clippy::match_single_binding, clippy::suspicious_else_formatting)]
    fn from_node(
        node: ::treesitter_types::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        debug_assert_eq!(node.kind(), "namespace_export");
        Ok(Self {
            span: ::treesitter_types::Span::from(node),
            children: {
                #[allow(clippy::suspicious_else_formatting)]
                let non_field_children = {
                    let mut cursor = node.walk();
                    let mut result = ::std::vec::Vec::new();
                    if cursor.goto_first_child() {
                        loop {
                            if cursor.field_name().is_none()
                                && cursor.node().is_named()
                                && !cursor.node().is_extra()
                            {
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
                        <NamespaceExportChildren as ::treesitter_types::FromNode>::from_node(
                            child, src,
                        )
                    })?),
                    None => None,
                }
            },
        })
    }
}
impl ::treesitter_types::Spanned for NamespaceExport<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        self.span
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct NamespaceImport<'tree> {
    pub span: ::treesitter_types::Span,
    pub children: Identifier<'tree>,
}
impl<'tree> ::treesitter_types::FromNode<'tree> for NamespaceImport<'tree> {
    #[allow(clippy::match_single_binding, clippy::suspicious_else_formatting)]
    fn from_node(
        node: ::treesitter_types::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        debug_assert_eq!(node.kind(), "namespace_import");
        Ok(Self {
            span: ::treesitter_types::Span::from(node),
            children: {
                #[allow(clippy::suspicious_else_formatting)]
                let non_field_children = {
                    let mut cursor = node.walk();
                    let mut result = ::std::vec::Vec::new();
                    if cursor.goto_first_child() {
                        loop {
                            if cursor.field_name().is_none()
                                && cursor.node().is_named()
                                && !cursor.node().is_extra()
                            {
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
impl ::treesitter_types::Spanned for NamespaceImport<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        self.span
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct NewExpression<'tree> {
    pub span: ::treesitter_types::Span,
    pub arguments: ::core::option::Option<Arguments<'tree>>,
    pub constructor: NewExpressionConstructor<'tree>,
}
impl<'tree> ::treesitter_types::FromNode<'tree> for NewExpression<'tree> {
    #[allow(clippy::match_single_binding, clippy::suspicious_else_formatting)]
    fn from_node(
        node: ::treesitter_types::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        debug_assert_eq!(node.kind(), "new_expression");
        Ok(Self {
            span: ::treesitter_types::Span::from(node),
            arguments: match node.child_by_field_name("arguments") {
                Some(child) => Some(::treesitter_types::runtime::maybe_grow_stack(|| {
                    <Arguments as ::treesitter_types::FromNode>::from_node(child, src)
                })?),
                None => None,
            },
            constructor: {
                let child = node.child_by_field_name("constructor").ok_or_else(|| {
                    ::treesitter_types::ParseError::missing_field("constructor", node)
                })?;
                ::treesitter_types::runtime::maybe_grow_stack(|| {
                    <NewExpressionConstructor as ::treesitter_types::FromNode>::from_node(
                        child, src,
                    )
                })?
            },
        })
    }
}
impl ::treesitter_types::Spanned for NewExpression<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        self.span
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Object<'tree> {
    pub span: ::treesitter_types::Span,
    pub children: ::std::vec::Vec<ObjectChildren<'tree>>,
}
impl<'tree> ::treesitter_types::FromNode<'tree> for Object<'tree> {
    #[allow(clippy::match_single_binding, clippy::suspicious_else_formatting)]
    fn from_node(
        node: ::treesitter_types::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        debug_assert_eq!(node.kind(), "object");
        Ok(Self {
            span: ::treesitter_types::Span::from(node),
            children: {
                #[allow(clippy::suspicious_else_formatting)]
                let non_field_children = {
                    let mut cursor = node.walk();
                    let mut result = ::std::vec::Vec::new();
                    if cursor.goto_first_child() {
                        loop {
                            if cursor.field_name().is_none()
                                && cursor.node().is_named()
                                && !cursor.node().is_extra()
                            {
                                result.push(cursor.node());
                            }
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
                        <ObjectChildren as ::treesitter_types::FromNode>::from_node(child, src)
                    })?);
                }
                items
            },
        })
    }
}
impl ::treesitter_types::Spanned for Object<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        self.span
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ObjectAssignmentPattern<'tree> {
    pub span: ::treesitter_types::Span,
    pub left: ObjectAssignmentPatternLeft<'tree>,
    pub right: Expression<'tree>,
}
impl<'tree> ::treesitter_types::FromNode<'tree> for ObjectAssignmentPattern<'tree> {
    #[allow(clippy::match_single_binding, clippy::suspicious_else_formatting)]
    fn from_node(
        node: ::treesitter_types::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        debug_assert_eq!(node.kind(), "object_assignment_pattern");
        Ok(Self {
            span: ::treesitter_types::Span::from(node),
            left: {
                let child = node
                    .child_by_field_name("left")
                    .ok_or_else(|| ::treesitter_types::ParseError::missing_field("left", node))?;
                ::treesitter_types::runtime::maybe_grow_stack(|| {
                    <ObjectAssignmentPatternLeft as ::treesitter_types::FromNode>::from_node(
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
impl ::treesitter_types::Spanned for ObjectAssignmentPattern<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        self.span
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ObjectPattern<'tree> {
    pub span: ::treesitter_types::Span,
    pub children: ::std::vec::Vec<ObjectPatternChildren<'tree>>,
}
impl<'tree> ::treesitter_types::FromNode<'tree> for ObjectPattern<'tree> {
    #[allow(clippy::match_single_binding, clippy::suspicious_else_formatting)]
    fn from_node(
        node: ::treesitter_types::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        debug_assert_eq!(node.kind(), "object_pattern");
        Ok(Self {
            span: ::treesitter_types::Span::from(node),
            children: {
                #[allow(clippy::suspicious_else_formatting)]
                let non_field_children = {
                    let mut cursor = node.walk();
                    let mut result = ::std::vec::Vec::new();
                    if cursor.goto_first_child() {
                        loop {
                            if cursor.field_name().is_none()
                                && cursor.node().is_named()
                                && !cursor.node().is_extra()
                            {
                                result.push(cursor.node());
                            }
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
                        <ObjectPatternChildren as ::treesitter_types::FromNode>::from_node(
                            child, src,
                        )
                    })?);
                }
                items
            },
        })
    }
}
impl ::treesitter_types::Spanned for ObjectPattern<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        self.span
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Pair<'tree> {
    pub span: ::treesitter_types::Span,
    pub key: PairKey<'tree>,
    pub value: Expression<'tree>,
}
impl<'tree> ::treesitter_types::FromNode<'tree> for Pair<'tree> {
    #[allow(clippy::match_single_binding, clippy::suspicious_else_formatting)]
    fn from_node(
        node: ::treesitter_types::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        debug_assert_eq!(node.kind(), "pair");
        Ok(Self {
            span: ::treesitter_types::Span::from(node),
            key: {
                let child = node
                    .child_by_field_name("key")
                    .ok_or_else(|| ::treesitter_types::ParseError::missing_field("key", node))?;
                ::treesitter_types::runtime::maybe_grow_stack(|| {
                    <PairKey as ::treesitter_types::FromNode>::from_node(child, src)
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
impl ::treesitter_types::Spanned for Pair<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        self.span
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PairPattern<'tree> {
    pub span: ::treesitter_types::Span,
    pub key: PairPatternKey<'tree>,
    pub value: PairPatternValue<'tree>,
}
impl<'tree> ::treesitter_types::FromNode<'tree> for PairPattern<'tree> {
    #[allow(clippy::match_single_binding, clippy::suspicious_else_formatting)]
    fn from_node(
        node: ::treesitter_types::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        debug_assert_eq!(node.kind(), "pair_pattern");
        Ok(Self {
            span: ::treesitter_types::Span::from(node),
            key: {
                let child = node
                    .child_by_field_name("key")
                    .ok_or_else(|| ::treesitter_types::ParseError::missing_field("key", node))?;
                ::treesitter_types::runtime::maybe_grow_stack(|| {
                    <PairPatternKey as ::treesitter_types::FromNode>::from_node(child, src)
                })?
            },
            value: {
                let child = node
                    .child_by_field_name("value")
                    .ok_or_else(|| ::treesitter_types::ParseError::missing_field("value", node))?;
                ::treesitter_types::runtime::maybe_grow_stack(|| {
                    <PairPatternValue as ::treesitter_types::FromNode>::from_node(child, src)
                })?
            },
        })
    }
}
impl ::treesitter_types::Spanned for PairPattern<'_> {
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
pub struct Program<'tree> {
    pub span: ::treesitter_types::Span,
    pub children: ::std::vec::Vec<ProgramChildren<'tree>>,
}
impl<'tree> ::treesitter_types::FromNode<'tree> for Program<'tree> {
    #[allow(clippy::match_single_binding, clippy::suspicious_else_formatting)]
    fn from_node(
        node: ::treesitter_types::tree_sitter::Node<'tree>,
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
                    items.push(::treesitter_types::runtime::maybe_grow_stack(|| {
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
pub struct Regex<'tree> {
    pub span: ::treesitter_types::Span,
    pub flags: ::core::option::Option<RegexFlags<'tree>>,
    pub pattern: RegexPattern<'tree>,
}
impl<'tree> ::treesitter_types::FromNode<'tree> for Regex<'tree> {
    #[allow(clippy::match_single_binding, clippy::suspicious_else_formatting)]
    fn from_node(
        node: ::treesitter_types::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        debug_assert_eq!(node.kind(), "regex");
        Ok(Self {
            span: ::treesitter_types::Span::from(node),
            flags: match node.child_by_field_name("flags") {
                Some(child) => Some(::treesitter_types::runtime::maybe_grow_stack(|| {
                    <RegexFlags as ::treesitter_types::FromNode>::from_node(child, src)
                })?),
                None => None,
            },
            pattern: {
                let child = node.child_by_field_name("pattern").ok_or_else(|| {
                    ::treesitter_types::ParseError::missing_field("pattern", node)
                })?;
                ::treesitter_types::runtime::maybe_grow_stack(|| {
                    <RegexPattern as ::treesitter_types::FromNode>::from_node(child, src)
                })?
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
pub struct RestPattern<'tree> {
    pub span: ::treesitter_types::Span,
    pub children: RestPatternChildren<'tree>,
}
impl<'tree> ::treesitter_types::FromNode<'tree> for RestPattern<'tree> {
    #[allow(clippy::match_single_binding, clippy::suspicious_else_formatting)]
    fn from_node(
        node: ::treesitter_types::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        debug_assert_eq!(node.kind(), "rest_pattern");
        Ok(Self {
            span: ::treesitter_types::Span::from(node),
            children: {
                #[allow(clippy::suspicious_else_formatting)]
                let non_field_children = {
                    let mut cursor = node.walk();
                    let mut result = ::std::vec::Vec::new();
                    if cursor.goto_first_child() {
                        loop {
                            if cursor.field_name().is_none()
                                && cursor.node().is_named()
                                && !cursor.node().is_extra()
                            {
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
                                        ::treesitter_types::runtime::maybe_grow_stack(|| <RestPatternChildren as ::treesitter_types::FromNode>::from_node(
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
                                            ::treesitter_types::runtime::maybe_grow_stack(|| <RestPatternChildren as ::treesitter_types::FromNode>::from_node(
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
                    <RestPatternChildren as ::treesitter_types::FromNode>::from_node(child, src)
                })?
            },
        })
    }
}
impl ::treesitter_types::Spanned for RestPattern<'_> {
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
pub struct SequenceExpression<'tree> {
    pub span: ::treesitter_types::Span,
    pub children: ::std::vec::Vec<Expression<'tree>>,
}
impl<'tree> ::treesitter_types::FromNode<'tree> for SequenceExpression<'tree> {
    #[allow(clippy::match_single_binding, clippy::suspicious_else_formatting)]
    fn from_node(
        node: ::treesitter_types::tree_sitter::Node<'tree>,
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
                    items.push(::treesitter_types::runtime::maybe_grow_stack(|| {
                        <Expression as ::treesitter_types::FromNode>::from_node(child, src)
                    })?);
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
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SpreadElement<'tree> {
    pub span: ::treesitter_types::Span,
    pub children: Expression<'tree>,
}
impl<'tree> ::treesitter_types::FromNode<'tree> for SpreadElement<'tree> {
    #[allow(clippy::match_single_binding, clippy::suspicious_else_formatting)]
    fn from_node(
        node: ::treesitter_types::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        debug_assert_eq!(node.kind(), "spread_element");
        Ok(Self {
            span: ::treesitter_types::Span::from(node),
            children: {
                #[allow(clippy::suspicious_else_formatting)]
                let non_field_children = {
                    let mut cursor = node.walk();
                    let mut result = ::std::vec::Vec::new();
                    if cursor.goto_first_child() {
                        loop {
                            if cursor.field_name().is_none()
                                && cursor.node().is_named()
                                && !cursor.node().is_extra()
                            {
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
impl ::treesitter_types::Spanned for SpreadElement<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        self.span
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct StatementBlock<'tree> {
    pub span: ::treesitter_types::Span,
    pub children: ::std::vec::Vec<Statement<'tree>>,
}
impl<'tree> ::treesitter_types::FromNode<'tree> for StatementBlock<'tree> {
    #[allow(clippy::match_single_binding, clippy::suspicious_else_formatting)]
    fn from_node(
        node: ::treesitter_types::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        debug_assert_eq!(node.kind(), "statement_block");
        Ok(Self {
            span: ::treesitter_types::Span::from(node),
            children: {
                #[allow(clippy::suspicious_else_formatting)]
                let non_field_children = {
                    let mut cursor = node.walk();
                    let mut result = ::std::vec::Vec::new();
                    if cursor.goto_first_child() {
                        loop {
                            if cursor.field_name().is_none()
                                && cursor.node().is_named()
                                && !cursor.node().is_extra()
                            {
                                result.push(cursor.node());
                            }
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
                        <Statement as ::treesitter_types::FromNode>::from_node(child, src)
                    })?);
                }
                items
            },
        })
    }
}
impl ::treesitter_types::Spanned for StatementBlock<'_> {
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
        node: ::treesitter_types::tree_sitter::Node<'tree>,
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
                    items.push(::treesitter_types::runtime::maybe_grow_stack(|| {
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
pub struct SubscriptExpression<'tree> {
    pub span: ::treesitter_types::Span,
    pub index: SubscriptExpressionIndex<'tree>,
    pub object: Expression<'tree>,
    pub optional_chain: ::core::option::Option<OptionalChain<'tree>>,
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
            index: {
                let child = node
                    .child_by_field_name("index")
                    .ok_or_else(|| ::treesitter_types::ParseError::missing_field("index", node))?;
                ::treesitter_types::runtime::maybe_grow_stack(|| {
                    <SubscriptExpressionIndex as ::treesitter_types::FromNode>::from_node(
                        child, src,
                    )
                })?
            },
            object: {
                let child = node
                    .child_by_field_name("object")
                    .ok_or_else(|| ::treesitter_types::ParseError::missing_field("object", node))?;
                ::treesitter_types::runtime::maybe_grow_stack(|| {
                    <Expression as ::treesitter_types::FromNode>::from_node(child, src)
                })?
            },
            optional_chain: match node.child_by_field_name("optional_chain") {
                Some(child) => Some(::treesitter_types::runtime::maybe_grow_stack(|| {
                    <OptionalChain as ::treesitter_types::FromNode>::from_node(child, src)
                })?),
                None => None,
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
pub struct SwitchBody<'tree> {
    pub span: ::treesitter_types::Span,
    pub children: ::std::vec::Vec<SwitchBodyChildren<'tree>>,
}
impl<'tree> ::treesitter_types::FromNode<'tree> for SwitchBody<'tree> {
    #[allow(clippy::match_single_binding, clippy::suspicious_else_formatting)]
    fn from_node(
        node: ::treesitter_types::tree_sitter::Node<'tree>,
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
                    items.push(::treesitter_types::runtime::maybe_grow_stack(|| {
                        <SwitchBodyChildren as ::treesitter_types::FromNode>::from_node(child, src)
                    })?);
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
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SwitchCase<'tree> {
    pub span: ::treesitter_types::Span,
    pub body: ::std::vec::Vec<Statement<'tree>>,
    pub value: SwitchCaseValue<'tree>,
}
impl<'tree> ::treesitter_types::FromNode<'tree> for SwitchCase<'tree> {
    #[allow(clippy::match_single_binding, clippy::suspicious_else_formatting)]
    fn from_node(
        node: ::treesitter_types::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        debug_assert_eq!(node.kind(), "switch_case");
        Ok(Self {
            span: ::treesitter_types::Span::from(node),
            body: {
                let mut cursor = node.walk();
                let mut items = ::std::vec::Vec::new();
                for child in node.children_by_field_name("body", &mut cursor) {
                    items.push(::treesitter_types::runtime::maybe_grow_stack(|| {
                        <Statement as ::treesitter_types::FromNode>::from_node(child, src)
                    })?);
                }
                items
            },
            value: {
                let child = node
                    .child_by_field_name("value")
                    .ok_or_else(|| ::treesitter_types::ParseError::missing_field("value", node))?;
                ::treesitter_types::runtime::maybe_grow_stack(|| {
                    <SwitchCaseValue as ::treesitter_types::FromNode>::from_node(child, src)
                })?
            },
        })
    }
}
impl ::treesitter_types::Spanned for SwitchCase<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        self.span
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SwitchDefault<'tree> {
    pub span: ::treesitter_types::Span,
    pub body: ::std::vec::Vec<Statement<'tree>>,
}
impl<'tree> ::treesitter_types::FromNode<'tree> for SwitchDefault<'tree> {
    #[allow(clippy::match_single_binding, clippy::suspicious_else_formatting)]
    fn from_node(
        node: ::treesitter_types::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        debug_assert_eq!(node.kind(), "switch_default");
        Ok(Self {
            span: ::treesitter_types::Span::from(node),
            body: {
                let mut cursor = node.walk();
                let mut items = ::std::vec::Vec::new();
                for child in node.children_by_field_name("body", &mut cursor) {
                    items.push(::treesitter_types::runtime::maybe_grow_stack(|| {
                        <Statement as ::treesitter_types::FromNode>::from_node(child, src)
                    })?);
                }
                items
            },
        })
    }
}
impl ::treesitter_types::Spanned for SwitchDefault<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        self.span
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SwitchStatement<'tree> {
    pub span: ::treesitter_types::Span,
    pub body: SwitchBody<'tree>,
    pub value: ParenthesizedExpression<'tree>,
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
                    <SwitchBody as ::treesitter_types::FromNode>::from_node(child, src)
                })?
            },
            value: {
                let child = node
                    .child_by_field_name("value")
                    .ok_or_else(|| ::treesitter_types::ParseError::missing_field("value", node))?;
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
pub struct TemplateString<'tree> {
    pub span: ::treesitter_types::Span,
    pub children: ::std::vec::Vec<TemplateStringChildren<'tree>>,
}
impl<'tree> ::treesitter_types::FromNode<'tree> for TemplateString<'tree> {
    #[allow(clippy::match_single_binding, clippy::suspicious_else_formatting)]
    fn from_node(
        node: ::treesitter_types::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        debug_assert_eq!(node.kind(), "template_string");
        Ok(Self {
            span: ::treesitter_types::Span::from(node),
            children: {
                #[allow(clippy::suspicious_else_formatting)]
                let non_field_children = {
                    let mut cursor = node.walk();
                    let mut result = ::std::vec::Vec::new();
                    if cursor.goto_first_child() {
                        loop {
                            if cursor.field_name().is_none()
                                && cursor.node().is_named()
                                && !cursor.node().is_extra()
                            {
                                result.push(cursor.node());
                            }
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
                        <TemplateStringChildren as ::treesitter_types::FromNode>::from_node(
                            child, src,
                        )
                    })?);
                }
                items
            },
        })
    }
}
impl ::treesitter_types::Spanned for TemplateString<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        self.span
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct TemplateSubstitution<'tree> {
    pub span: ::treesitter_types::Span,
    pub children: TemplateSubstitutionChildren<'tree>,
}
impl<'tree> ::treesitter_types::FromNode<'tree> for TemplateSubstitution<'tree> {
    #[allow(clippy::match_single_binding, clippy::suspicious_else_formatting)]
    fn from_node(
        node: ::treesitter_types::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        debug_assert_eq!(node.kind(), "template_substitution");
        Ok(Self {
            span: ::treesitter_types::Span::from(node),
            children: {
                #[allow(clippy::suspicious_else_formatting)]
                let non_field_children = {
                    let mut cursor = node.walk();
                    let mut result = ::std::vec::Vec::new();
                    if cursor.goto_first_child() {
                        loop {
                            if cursor.field_name().is_none()
                                && cursor.node().is_named()
                                && !cursor.node().is_extra()
                            {
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
                                        ::treesitter_types::runtime::maybe_grow_stack(|| <TemplateSubstitutionChildren as ::treesitter_types::FromNode>::from_node(
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
                                            ::treesitter_types::runtime::maybe_grow_stack(|| <TemplateSubstitutionChildren as ::treesitter_types::FromNode>::from_node(
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
                    <TemplateSubstitutionChildren as ::treesitter_types::FromNode>::from_node(
                        child, src,
                    )
                })?
            },
        })
    }
}
impl ::treesitter_types::Spanned for TemplateSubstitution<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        self.span
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct TernaryExpression<'tree> {
    pub span: ::treesitter_types::Span,
    pub alternative: Expression<'tree>,
    pub condition: Expression<'tree>,
    pub consequence: Expression<'tree>,
}
impl<'tree> ::treesitter_types::FromNode<'tree> for TernaryExpression<'tree> {
    #[allow(clippy::match_single_binding, clippy::suspicious_else_formatting)]
    fn from_node(
        node: ::treesitter_types::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        debug_assert_eq!(node.kind(), "ternary_expression");
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
            consequence: {
                let child = node.child_by_field_name("consequence").ok_or_else(|| {
                    ::treesitter_types::ParseError::missing_field("consequence", node)
                })?;
                ::treesitter_types::runtime::maybe_grow_stack(|| {
                    <Expression as ::treesitter_types::FromNode>::from_node(child, src)
                })?
            },
        })
    }
}
impl ::treesitter_types::Spanned for TernaryExpression<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        self.span
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ThrowStatement<'tree> {
    pub span: ::treesitter_types::Span,
    pub children: ThrowStatementChildren<'tree>,
}
impl<'tree> ::treesitter_types::FromNode<'tree> for ThrowStatement<'tree> {
    #[allow(clippy::match_single_binding, clippy::suspicious_else_formatting)]
    fn from_node(
        node: ::treesitter_types::tree_sitter::Node<'tree>,
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
                                        ::treesitter_types::runtime::maybe_grow_stack(|| <ThrowStatementChildren as ::treesitter_types::FromNode>::from_node(
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
                                            ::treesitter_types::runtime::maybe_grow_stack(|| <ThrowStatementChildren as ::treesitter_types::FromNode>::from_node(
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
                    <ThrowStatementChildren as ::treesitter_types::FromNode>::from_node(child, src)
                })?
            },
        })
    }
}
impl ::treesitter_types::Spanned for ThrowStatement<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        self.span
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct TryStatement<'tree> {
    pub span: ::treesitter_types::Span,
    pub body: StatementBlock<'tree>,
    pub finalizer: ::core::option::Option<FinallyClause<'tree>>,
    pub handler: ::core::option::Option<CatchClause<'tree>>,
}
impl<'tree> ::treesitter_types::FromNode<'tree> for TryStatement<'tree> {
    #[allow(clippy::match_single_binding, clippy::suspicious_else_formatting)]
    fn from_node(
        node: ::treesitter_types::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        debug_assert_eq!(node.kind(), "try_statement");
        Ok(Self {
            span: ::treesitter_types::Span::from(node),
            body: {
                let child = node
                    .child_by_field_name("body")
                    .ok_or_else(|| ::treesitter_types::ParseError::missing_field("body", node))?;
                ::treesitter_types::runtime::maybe_grow_stack(|| {
                    <StatementBlock as ::treesitter_types::FromNode>::from_node(child, src)
                })?
            },
            finalizer: match node.child_by_field_name("finalizer") {
                Some(child) => Some(::treesitter_types::runtime::maybe_grow_stack(|| {
                    <FinallyClause as ::treesitter_types::FromNode>::from_node(child, src)
                })?),
                None => None,
            },
            handler: match node.child_by_field_name("handler") {
                Some(child) => Some(::treesitter_types::runtime::maybe_grow_stack(|| {
                    <CatchClause as ::treesitter_types::FromNode>::from_node(child, src)
                })?),
                None => None,
            },
        })
    }
}
impl ::treesitter_types::Spanned for TryStatement<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        self.span
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct UnaryExpression<'tree> {
    pub span: ::treesitter_types::Span,
    pub argument: Expression<'tree>,
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
                    <Expression as ::treesitter_types::FromNode>::from_node(child, src)
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
pub struct UsingDeclaration<'tree> {
    pub span: ::treesitter_types::Span,
    pub kind: ::std::vec::Vec<UsingDeclarationKind>,
    pub children: ::std::vec::Vec<VariableDeclarator<'tree>>,
}
impl<'tree> ::treesitter_types::FromNode<'tree> for UsingDeclaration<'tree> {
    #[allow(clippy::match_single_binding, clippy::suspicious_else_formatting)]
    fn from_node(
        node: ::treesitter_types::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        debug_assert_eq!(node.kind(), "using_declaration");
        Ok(Self {
            span: ::treesitter_types::Span::from(node),
            kind: {
                let mut cursor = node.walk();
                let mut items = ::std::vec::Vec::new();
                for child in node.children_by_field_name("kind", &mut cursor) {
                    items.push(::treesitter_types::runtime::maybe_grow_stack(|| {
                        <UsingDeclarationKind as ::treesitter_types::FromNode>::from_node(
                            child, src,
                        )
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
                    items.push(::treesitter_types::runtime::maybe_grow_stack(|| {
                        <VariableDeclarator as ::treesitter_types::FromNode>::from_node(child, src)
                    })?);
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
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct VariableDeclaration<'tree> {
    pub span: ::treesitter_types::Span,
    pub children: ::std::vec::Vec<VariableDeclarator<'tree>>,
}
impl<'tree> ::treesitter_types::FromNode<'tree> for VariableDeclaration<'tree> {
    #[allow(clippy::match_single_binding, clippy::suspicious_else_formatting)]
    fn from_node(
        node: ::treesitter_types::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        debug_assert_eq!(node.kind(), "variable_declaration");
        Ok(Self {
            span: ::treesitter_types::Span::from(node),
            children: {
                #[allow(clippy::suspicious_else_formatting)]
                let non_field_children = {
                    let mut cursor = node.walk();
                    let mut result = ::std::vec::Vec::new();
                    if cursor.goto_first_child() {
                        loop {
                            if cursor.field_name().is_none()
                                && cursor.node().is_named()
                                && !cursor.node().is_extra()
                            {
                                result.push(cursor.node());
                            }
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
                        <VariableDeclarator as ::treesitter_types::FromNode>::from_node(child, src)
                    })?);
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
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct VariableDeclarator<'tree> {
    pub span: ::treesitter_types::Span,
    pub name: VariableDeclaratorName<'tree>,
    pub value: ::core::option::Option<Expression<'tree>>,
}
impl<'tree> ::treesitter_types::FromNode<'tree> for VariableDeclarator<'tree> {
    #[allow(clippy::match_single_binding, clippy::suspicious_else_formatting)]
    fn from_node(
        node: ::treesitter_types::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        debug_assert_eq!(node.kind(), "variable_declarator");
        Ok(Self {
            span: ::treesitter_types::Span::from(node),
            name: {
                let child = node
                    .child_by_field_name("name")
                    .ok_or_else(|| ::treesitter_types::ParseError::missing_field("name", node))?;
                ::treesitter_types::runtime::maybe_grow_stack(|| {
                    <VariableDeclaratorName as ::treesitter_types::FromNode>::from_node(child, src)
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
impl ::treesitter_types::Spanned for VariableDeclarator<'_> {
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
pub struct WithStatement<'tree> {
    pub span: ::treesitter_types::Span,
    pub body: Statement<'tree>,
    pub object: ParenthesizedExpression<'tree>,
}
impl<'tree> ::treesitter_types::FromNode<'tree> for WithStatement<'tree> {
    #[allow(clippy::match_single_binding, clippy::suspicious_else_formatting)]
    fn from_node(
        node: ::treesitter_types::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        debug_assert_eq!(node.kind(), "with_statement");
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
            object: {
                let child = node
                    .child_by_field_name("object")
                    .ok_or_else(|| ::treesitter_types::ParseError::missing_field("object", node))?;
                ::treesitter_types::runtime::maybe_grow_stack(|| {
                    <ParenthesizedExpression as ::treesitter_types::FromNode>::from_node(child, src)
                })?
            },
        })
    }
}
impl ::treesitter_types::Spanned for WithStatement<'_> {
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
        node: ::treesitter_types::tree_sitter::Node<'tree>,
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
pub struct HashBangLine<'tree> {
    pub span: ::treesitter_types::Span,
    text: &'tree str,
}
impl<'tree> ::treesitter_types::FromNode<'tree> for HashBangLine<'tree> {
    fn from_node(
        node: ::treesitter_types::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        debug_assert_eq!(node.kind(), "hash_bang_line");
        Ok(Self {
            span: ::treesitter_types::Span::from(node),
            text: node.utf8_text(src)?,
        })
    }
}
impl<'tree> ::treesitter_types::LeafNode<'tree> for HashBangLine<'tree> {
    fn text(&self) -> &'tree str {
        self.text
    }
}
impl ::treesitter_types::Spanned for HashBangLine<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        self.span
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct HtmlCharacterReference<'tree> {
    pub span: ::treesitter_types::Span,
    text: &'tree str,
}
impl<'tree> ::treesitter_types::FromNode<'tree> for HtmlCharacterReference<'tree> {
    fn from_node(
        node: ::treesitter_types::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        debug_assert_eq!(node.kind(), "html_character_reference");
        Ok(Self {
            span: ::treesitter_types::Span::from(node),
            text: node.utf8_text(src)?,
        })
    }
}
impl<'tree> ::treesitter_types::LeafNode<'tree> for HtmlCharacterReference<'tree> {
    fn text(&self) -> &'tree str {
        self.text
    }
}
impl ::treesitter_types::Spanned for HtmlCharacterReference<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        self.span
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct HtmlComment<'tree> {
    pub span: ::treesitter_types::Span,
    text: &'tree str,
}
impl<'tree> ::treesitter_types::FromNode<'tree> for HtmlComment<'tree> {
    fn from_node(
        node: ::treesitter_types::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        debug_assert_eq!(node.kind(), "html_comment");
        Ok(Self {
            span: ::treesitter_types::Span::from(node),
            text: node.utf8_text(src)?,
        })
    }
}
impl<'tree> ::treesitter_types::LeafNode<'tree> for HtmlComment<'tree> {
    fn text(&self) -> &'tree str {
        self.text
    }
}
impl ::treesitter_types::Spanned for HtmlComment<'_> {
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
pub struct JsxText<'tree> {
    pub span: ::treesitter_types::Span,
    text: &'tree str,
}
impl<'tree> ::treesitter_types::FromNode<'tree> for JsxText<'tree> {
    fn from_node(
        node: ::treesitter_types::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        debug_assert_eq!(node.kind(), "jsx_text");
        Ok(Self {
            span: ::treesitter_types::Span::from(node),
            text: node.utf8_text(src)?,
        })
    }
}
impl<'tree> ::treesitter_types::LeafNode<'tree> for JsxText<'tree> {
    fn text(&self) -> &'tree str {
        self.text
    }
}
impl ::treesitter_types::Spanned for JsxText<'_> {
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
pub struct Number<'tree> {
    pub span: ::treesitter_types::Span,
    text: &'tree str,
}
impl<'tree> ::treesitter_types::FromNode<'tree> for Number<'tree> {
    fn from_node(
        node: ::treesitter_types::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        debug_assert_eq!(node.kind(), "number");
        Ok(Self {
            span: ::treesitter_types::Span::from(node),
            text: node.utf8_text(src)?,
        })
    }
}
impl<'tree> ::treesitter_types::LeafNode<'tree> for Number<'tree> {
    fn text(&self) -> &'tree str {
        self.text
    }
}
impl ::treesitter_types::Spanned for Number<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        self.span
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct OptionalChain<'tree> {
    pub span: ::treesitter_types::Span,
    text: &'tree str,
}
impl<'tree> ::treesitter_types::FromNode<'tree> for OptionalChain<'tree> {
    fn from_node(
        node: ::treesitter_types::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        debug_assert_eq!(node.kind(), "optional_chain");
        Ok(Self {
            span: ::treesitter_types::Span::from(node),
            text: node.utf8_text(src)?,
        })
    }
}
impl<'tree> ::treesitter_types::LeafNode<'tree> for OptionalChain<'tree> {
    fn text(&self) -> &'tree str {
        self.text
    }
}
impl ::treesitter_types::Spanned for OptionalChain<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        self.span
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PrivatePropertyIdentifier<'tree> {
    pub span: ::treesitter_types::Span,
    text: &'tree str,
}
impl<'tree> ::treesitter_types::FromNode<'tree> for PrivatePropertyIdentifier<'tree> {
    fn from_node(
        node: ::treesitter_types::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        debug_assert_eq!(node.kind(), "private_property_identifier");
        Ok(Self {
            span: ::treesitter_types::Span::from(node),
            text: node.utf8_text(src)?,
        })
    }
}
impl<'tree> ::treesitter_types::LeafNode<'tree> for PrivatePropertyIdentifier<'tree> {
    fn text(&self) -> &'tree str {
        self.text
    }
}
impl ::treesitter_types::Spanned for PrivatePropertyIdentifier<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        self.span
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PropertyIdentifier<'tree> {
    pub span: ::treesitter_types::Span,
    text: &'tree str,
}
impl<'tree> ::treesitter_types::FromNode<'tree> for PropertyIdentifier<'tree> {
    fn from_node(
        node: ::treesitter_types::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        debug_assert_eq!(node.kind(), "property_identifier");
        Ok(Self {
            span: ::treesitter_types::Span::from(node),
            text: node.utf8_text(src)?,
        })
    }
}
impl<'tree> ::treesitter_types::LeafNode<'tree> for PropertyIdentifier<'tree> {
    fn text(&self) -> &'tree str {
        self.text
    }
}
impl ::treesitter_types::Spanned for PropertyIdentifier<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        self.span
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct RegexFlags<'tree> {
    pub span: ::treesitter_types::Span,
    text: &'tree str,
}
impl<'tree> ::treesitter_types::FromNode<'tree> for RegexFlags<'tree> {
    fn from_node(
        node: ::treesitter_types::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        debug_assert_eq!(node.kind(), "regex_flags");
        Ok(Self {
            span: ::treesitter_types::Span::from(node),
            text: node.utf8_text(src)?,
        })
    }
}
impl<'tree> ::treesitter_types::LeafNode<'tree> for RegexFlags<'tree> {
    fn text(&self) -> &'tree str {
        self.text
    }
}
impl ::treesitter_types::Spanned for RegexFlags<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        self.span
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct RegexPattern<'tree> {
    pub span: ::treesitter_types::Span,
    text: &'tree str,
}
impl<'tree> ::treesitter_types::FromNode<'tree> for RegexPattern<'tree> {
    fn from_node(
        node: ::treesitter_types::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        debug_assert_eq!(node.kind(), "regex_pattern");
        Ok(Self {
            span: ::treesitter_types::Span::from(node),
            text: node.utf8_text(src)?,
        })
    }
}
impl<'tree> ::treesitter_types::LeafNode<'tree> for RegexPattern<'tree> {
    fn text(&self) -> &'tree str {
        self.text
    }
}
impl ::treesitter_types::Spanned for RegexPattern<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        self.span
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ShorthandPropertyIdentifier<'tree> {
    pub span: ::treesitter_types::Span,
    text: &'tree str,
}
impl<'tree> ::treesitter_types::FromNode<'tree> for ShorthandPropertyIdentifier<'tree> {
    fn from_node(
        node: ::treesitter_types::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        debug_assert_eq!(node.kind(), "shorthand_property_identifier");
        Ok(Self {
            span: ::treesitter_types::Span::from(node),
            text: node.utf8_text(src)?,
        })
    }
}
impl<'tree> ::treesitter_types::LeafNode<'tree> for ShorthandPropertyIdentifier<'tree> {
    fn text(&self) -> &'tree str {
        self.text
    }
}
impl ::treesitter_types::Spanned for ShorthandPropertyIdentifier<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        self.span
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ShorthandPropertyIdentifierPattern<'tree> {
    pub span: ::treesitter_types::Span,
    text: &'tree str,
}
impl<'tree> ::treesitter_types::FromNode<'tree> for ShorthandPropertyIdentifierPattern<'tree> {
    fn from_node(
        node: ::treesitter_types::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        debug_assert_eq!(node.kind(), "shorthand_property_identifier_pattern");
        Ok(Self {
            span: ::treesitter_types::Span::from(node),
            text: node.utf8_text(src)?,
        })
    }
}
impl<'tree> ::treesitter_types::LeafNode<'tree> for ShorthandPropertyIdentifierPattern<'tree> {
    fn text(&self) -> &'tree str {
        self.text
    }
}
impl ::treesitter_types::Spanned for ShorthandPropertyIdentifierPattern<'_> {
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
pub struct StringFragment<'tree> {
    pub span: ::treesitter_types::Span,
    text: &'tree str,
}
impl<'tree> ::treesitter_types::FromNode<'tree> for StringFragment<'tree> {
    fn from_node(
        node: ::treesitter_types::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        debug_assert_eq!(node.kind(), "string_fragment");
        Ok(Self {
            span: ::treesitter_types::Span::from(node),
            text: node.utf8_text(src)?,
        })
    }
}
impl<'tree> ::treesitter_types::LeafNode<'tree> for StringFragment<'tree> {
    fn text(&self) -> &'tree str {
        self.text
    }
}
impl ::treesitter_types::Spanned for StringFragment<'_> {
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
        node: ::treesitter_types::tree_sitter::Node<'tree>,
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
pub struct This<'tree> {
    pub span: ::treesitter_types::Span,
    text: &'tree str,
}
impl<'tree> ::treesitter_types::FromNode<'tree> for This<'tree> {
    fn from_node(
        node: ::treesitter_types::tree_sitter::Node<'tree>,
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
pub struct Undefined<'tree> {
    pub span: ::treesitter_types::Span,
    text: &'tree str,
}
impl<'tree> ::treesitter_types::FromNode<'tree> for Undefined<'tree> {
    fn from_node(
        node: ::treesitter_types::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        debug_assert_eq!(node.kind(), "undefined");
        Ok(Self {
            span: ::treesitter_types::Span::from(node),
            text: node.utf8_text(src)?,
        })
    }
}
impl<'tree> ::treesitter_types::LeafNode<'tree> for Undefined<'tree> {
    fn text(&self) -> &'tree str {
        self.text
    }
}
impl ::treesitter_types::Spanned for Undefined<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        self.span
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ArgumentsChildren<'tree> {
    Expression(::std::boxed::Box<Expression<'tree>>),
    SpreadElement(::std::boxed::Box<SpreadElement<'tree>>),
}
impl<'tree> ::treesitter_types::FromNode<'tree> for ArgumentsChildren<'tree> {
    #[allow(clippy::collapsible_else_if)]
    fn from_node(
        node: ::treesitter_types::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        match node.kind() {
            "spread_element" => Ok(Self::SpreadElement(::std::boxed::Box::new(
                ::treesitter_types::runtime::maybe_grow_stack(|| {
                    <SpreadElement as ::treesitter_types::FromNode>::from_node(node, src)
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
            Self::SpreadElement(inner) => inner.span(),
        }
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ArrayChildren<'tree> {
    Expression(::std::boxed::Box<Expression<'tree>>),
    SpreadElement(::std::boxed::Box<SpreadElement<'tree>>),
}
impl<'tree> ::treesitter_types::FromNode<'tree> for ArrayChildren<'tree> {
    #[allow(clippy::collapsible_else_if)]
    fn from_node(
        node: ::treesitter_types::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        match node.kind() {
            "spread_element" => Ok(Self::SpreadElement(::std::boxed::Box::new(
                ::treesitter_types::runtime::maybe_grow_stack(|| {
                    <SpreadElement as ::treesitter_types::FromNode>::from_node(node, src)
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
impl ::treesitter_types::Spanned for ArrayChildren<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        match self {
            Self::Expression(inner) => inner.span(),
            Self::SpreadElement(inner) => inner.span(),
        }
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ArrayPatternChildren<'tree> {
    AssignmentPattern(::std::boxed::Box<AssignmentPattern<'tree>>),
    Pattern(::std::boxed::Box<Pattern<'tree>>),
}
impl<'tree> ::treesitter_types::FromNode<'tree> for ArrayPatternChildren<'tree> {
    #[allow(clippy::collapsible_else_if)]
    fn from_node(
        node: ::treesitter_types::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        match node.kind() {
            "assignment_pattern" => Ok(Self::AssignmentPattern(::std::boxed::Box::new(
                ::treesitter_types::runtime::maybe_grow_stack(|| {
                    <AssignmentPattern as ::treesitter_types::FromNode>::from_node(node, src)
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
impl ::treesitter_types::Spanned for ArrayPatternChildren<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        match self {
            Self::AssignmentPattern(inner) => inner.span(),
            Self::Pattern(inner) => inner.span(),
        }
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ArrowFunctionBody<'tree> {
    Expression(::std::boxed::Box<Expression<'tree>>),
    StatementBlock(::std::boxed::Box<StatementBlock<'tree>>),
}
impl<'tree> ::treesitter_types::FromNode<'tree> for ArrowFunctionBody<'tree> {
    #[allow(clippy::collapsible_else_if)]
    fn from_node(
        node: ::treesitter_types::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        match node.kind() {
            "statement_block" => Ok(Self::StatementBlock(::std::boxed::Box::new(
                ::treesitter_types::runtime::maybe_grow_stack(|| {
                    <StatementBlock as ::treesitter_types::FromNode>::from_node(node, src)
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
impl ::treesitter_types::Spanned for ArrowFunctionBody<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        match self {
            Self::Expression(inner) => inner.span(),
            Self::StatementBlock(inner) => inner.span(),
        }
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum AssignmentExpressionLeft<'tree> {
    ArrayPattern(::std::boxed::Box<ArrayPattern<'tree>>),
    Identifier(::std::boxed::Box<Identifier<'tree>>),
    MemberExpression(::std::boxed::Box<MemberExpression<'tree>>),
    ObjectPattern(::std::boxed::Box<ObjectPattern<'tree>>),
    ParenthesizedExpression(::std::boxed::Box<ParenthesizedExpression<'tree>>),
    SubscriptExpression(::std::boxed::Box<SubscriptExpression<'tree>>),
    Undefined(::std::boxed::Box<Undefined<'tree>>),
}
impl<'tree> ::treesitter_types::FromNode<'tree> for AssignmentExpressionLeft<'tree> {
    #[allow(clippy::collapsible_else_if)]
    fn from_node(
        node: ::treesitter_types::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        match node.kind() {
            "array_pattern" => Ok(Self::ArrayPattern(::std::boxed::Box::new(
                ::treesitter_types::runtime::maybe_grow_stack(|| {
                    <ArrayPattern as ::treesitter_types::FromNode>::from_node(node, src)
                })?,
            ))),
            "identifier" => Ok(Self::Identifier(::std::boxed::Box::new(
                ::treesitter_types::runtime::maybe_grow_stack(|| {
                    <Identifier as ::treesitter_types::FromNode>::from_node(node, src)
                })?,
            ))),
            "member_expression" => Ok(Self::MemberExpression(::std::boxed::Box::new(
                ::treesitter_types::runtime::maybe_grow_stack(|| {
                    <MemberExpression as ::treesitter_types::FromNode>::from_node(node, src)
                })?,
            ))),
            "object_pattern" => Ok(Self::ObjectPattern(::std::boxed::Box::new(
                ::treesitter_types::runtime::maybe_grow_stack(|| {
                    <ObjectPattern as ::treesitter_types::FromNode>::from_node(node, src)
                })?,
            ))),
            "parenthesized_expression" => Ok(Self::ParenthesizedExpression(
                ::std::boxed::Box::new(::treesitter_types::runtime::maybe_grow_stack(|| {
                    <ParenthesizedExpression as ::treesitter_types::FromNode>::from_node(node, src)
                })?),
            )),
            "subscript_expression" => Ok(Self::SubscriptExpression(::std::boxed::Box::new(
                ::treesitter_types::runtime::maybe_grow_stack(|| {
                    <SubscriptExpression as ::treesitter_types::FromNode>::from_node(node, src)
                })?,
            ))),
            "undefined" => Ok(Self::Undefined(::std::boxed::Box::new(
                ::treesitter_types::runtime::maybe_grow_stack(|| {
                    <Undefined as ::treesitter_types::FromNode>::from_node(node, src)
                })?,
            ))),
            other => Err(::treesitter_types::ParseError::unexpected_kind(other, node)),
        }
    }
}
impl ::treesitter_types::Spanned for AssignmentExpressionLeft<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        match self {
            Self::ArrayPattern(inner) => inner.span(),
            Self::Identifier(inner) => inner.span(),
            Self::MemberExpression(inner) => inner.span(),
            Self::ObjectPattern(inner) => inner.span(),
            Self::ParenthesizedExpression(inner) => inner.span(),
            Self::SubscriptExpression(inner) => inner.span(),
            Self::Undefined(inner) => inner.span(),
        }
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum AugmentedAssignmentExpressionLeft<'tree> {
    Identifier(::std::boxed::Box<Identifier<'tree>>),
    MemberExpression(::std::boxed::Box<MemberExpression<'tree>>),
    ParenthesizedExpression(::std::boxed::Box<ParenthesizedExpression<'tree>>),
    SubscriptExpression(::std::boxed::Box<SubscriptExpression<'tree>>),
}
impl<'tree> ::treesitter_types::FromNode<'tree> for AugmentedAssignmentExpressionLeft<'tree> {
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
            "member_expression" => Ok(Self::MemberExpression(::std::boxed::Box::new(
                ::treesitter_types::runtime::maybe_grow_stack(|| {
                    <MemberExpression as ::treesitter_types::FromNode>::from_node(node, src)
                })?,
            ))),
            "parenthesized_expression" => Ok(Self::ParenthesizedExpression(
                ::std::boxed::Box::new(::treesitter_types::runtime::maybe_grow_stack(|| {
                    <ParenthesizedExpression as ::treesitter_types::FromNode>::from_node(node, src)
                })?),
            )),
            "subscript_expression" => Ok(Self::SubscriptExpression(::std::boxed::Box::new(
                ::treesitter_types::runtime::maybe_grow_stack(|| {
                    <SubscriptExpression as ::treesitter_types::FromNode>::from_node(node, src)
                })?,
            ))),
            other => Err(::treesitter_types::ParseError::unexpected_kind(other, node)),
        }
    }
}
impl ::treesitter_types::Spanned for AugmentedAssignmentExpressionLeft<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        match self {
            Self::Identifier(inner) => inner.span(),
            Self::MemberExpression(inner) => inner.span(),
            Self::ParenthesizedExpression(inner) => inner.span(),
            Self::SubscriptExpression(inner) => inner.span(),
        }
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum AugmentedAssignmentExpressionOperator {
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
    GtGtGtEq(::treesitter_types::Span),
    QuestionQuestionEq(::treesitter_types::Span),
    CaretEq(::treesitter_types::Span),
    PipeEq(::treesitter_types::Span),
    PipePipeEq(::treesitter_types::Span),
}
impl<'tree> ::treesitter_types::FromNode<'tree> for AugmentedAssignmentExpressionOperator {
    #[allow(clippy::collapsible_else_if)]
    fn from_node(
        node: ::treesitter_types::tree_sitter::Node<'tree>,
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
            ">>>=" => Ok(Self::GtGtGtEq(::treesitter_types::Span::from(node))),
            "??=" => Ok(Self::QuestionQuestionEq(::treesitter_types::Span::from(
                node,
            ))),
            "^=" => Ok(Self::CaretEq(::treesitter_types::Span::from(node))),
            "|=" => Ok(Self::PipeEq(::treesitter_types::Span::from(node))),
            "||=" => Ok(Self::PipePipeEq(::treesitter_types::Span::from(node))),
            other => Err(::treesitter_types::ParseError::unexpected_kind(other, node)),
        }
    }
}
impl ::treesitter_types::Spanned for AugmentedAssignmentExpressionOperator {
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
            Self::GtGtGtEq(span) => *span,
            Self::QuestionQuestionEq(span) => *span,
            Self::CaretEq(span) => *span,
            Self::PipeEq(span) => *span,
            Self::PipePipeEq(span) => *span,
        }
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum BinaryExpressionLeft<'tree> {
    Expression(::std::boxed::Box<Expression<'tree>>),
    PrivatePropertyIdentifier(::std::boxed::Box<PrivatePropertyIdentifier<'tree>>),
}
impl<'tree> ::treesitter_types::FromNode<'tree> for BinaryExpressionLeft<'tree> {
    #[allow(clippy::collapsible_else_if)]
    fn from_node(
        node: ::treesitter_types::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        match node.kind() {
            "private_property_identifier" => Ok(Self::PrivatePropertyIdentifier(
                ::std::boxed::Box::new(::treesitter_types::runtime::maybe_grow_stack(|| {
                    <PrivatePropertyIdentifier as ::treesitter_types::FromNode>::from_node(
                        node, src,
                    )
                })?),
            )),
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
            Self::PrivatePropertyIdentifier(inner) => inner.span(),
        }
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
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
    Slash(::treesitter_types::Span),
    Lt(::treesitter_types::Span),
    Shl(::treesitter_types::Span),
    LtEq(::treesitter_types::Span),
    EqEq(::treesitter_types::Span),
    EqEqEq(::treesitter_types::Span),
    Gt(::treesitter_types::Span),
    GtEq(::treesitter_types::Span),
    Shr(::treesitter_types::Span),
    GtGtGt(::treesitter_types::Span),
    QuestionQuestion(::treesitter_types::Span),
    Caret(::treesitter_types::Span),
    In(::treesitter_types::Span),
    Instanceof(::treesitter_types::Span),
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
            "!==" => Ok(Self::BangEqEq(::treesitter_types::Span::from(node))),
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
            "==" => Ok(Self::EqEq(::treesitter_types::Span::from(node))),
            "===" => Ok(Self::EqEqEq(::treesitter_types::Span::from(node))),
            ">" => Ok(Self::Gt(::treesitter_types::Span::from(node))),
            ">=" => Ok(Self::GtEq(::treesitter_types::Span::from(node))),
            ">>" => Ok(Self::Shr(::treesitter_types::Span::from(node))),
            ">>>" => Ok(Self::GtGtGt(::treesitter_types::Span::from(node))),
            "??" => Ok(Self::QuestionQuestion(::treesitter_types::Span::from(node))),
            "^" => Ok(Self::Caret(::treesitter_types::Span::from(node))),
            "in" => Ok(Self::In(::treesitter_types::Span::from(node))),
            "instanceof" => Ok(Self::Instanceof(::treesitter_types::Span::from(node))),
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
            Self::BangEqEq(span) => *span,
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
            Self::EqEq(span) => *span,
            Self::EqEqEq(span) => *span,
            Self::Gt(span) => *span,
            Self::GtEq(span) => *span,
            Self::Shr(span) => *span,
            Self::GtGtGt(span) => *span,
            Self::QuestionQuestion(span) => *span,
            Self::Caret(span) => *span,
            Self::In(span) => *span,
            Self::Instanceof(span) => *span,
            Self::Pipe(span) => *span,
            Self::PipePipe(span) => *span,
        }
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum CallExpressionArguments<'tree> {
    Arguments(::std::boxed::Box<Arguments<'tree>>),
    TemplateString(::std::boxed::Box<TemplateString<'tree>>),
}
impl<'tree> ::treesitter_types::FromNode<'tree> for CallExpressionArguments<'tree> {
    #[allow(clippy::collapsible_else_if)]
    fn from_node(
        node: ::treesitter_types::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        match node.kind() {
            "arguments" => Ok(Self::Arguments(::std::boxed::Box::new(
                ::treesitter_types::runtime::maybe_grow_stack(|| {
                    <Arguments as ::treesitter_types::FromNode>::from_node(node, src)
                })?,
            ))),
            "template_string" => Ok(Self::TemplateString(::std::boxed::Box::new(
                ::treesitter_types::runtime::maybe_grow_stack(|| {
                    <TemplateString as ::treesitter_types::FromNode>::from_node(node, src)
                })?,
            ))),
            other => Err(::treesitter_types::ParseError::unexpected_kind(other, node)),
        }
    }
}
impl ::treesitter_types::Spanned for CallExpressionArguments<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        match self {
            Self::Arguments(inner) => inner.span(),
            Self::TemplateString(inner) => inner.span(),
        }
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum CallExpressionFunction<'tree> {
    Expression(::std::boxed::Box<Expression<'tree>>),
    Import(::std::boxed::Box<Import<'tree>>),
}
impl<'tree> ::treesitter_types::FromNode<'tree> for CallExpressionFunction<'tree> {
    #[allow(clippy::collapsible_else_if)]
    fn from_node(
        node: ::treesitter_types::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        match node.kind() {
            "import" => Ok(Self::Import(::std::boxed::Box::new(
                ::treesitter_types::runtime::maybe_grow_stack(|| {
                    <Import as ::treesitter_types::FromNode>::from_node(node, src)
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
impl ::treesitter_types::Spanned for CallExpressionFunction<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        match self {
            Self::Expression(inner) => inner.span(),
            Self::Import(inner) => inner.span(),
        }
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum CatchClauseParameter<'tree> {
    ArrayPattern(::std::boxed::Box<ArrayPattern<'tree>>),
    Identifier(::std::boxed::Box<Identifier<'tree>>),
    ObjectPattern(::std::boxed::Box<ObjectPattern<'tree>>),
}
impl<'tree> ::treesitter_types::FromNode<'tree> for CatchClauseParameter<'tree> {
    #[allow(clippy::collapsible_else_if)]
    fn from_node(
        node: ::treesitter_types::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        match node.kind() {
            "array_pattern" => Ok(Self::ArrayPattern(::std::boxed::Box::new(
                ::treesitter_types::runtime::maybe_grow_stack(|| {
                    <ArrayPattern as ::treesitter_types::FromNode>::from_node(node, src)
                })?,
            ))),
            "identifier" => Ok(Self::Identifier(::std::boxed::Box::new(
                ::treesitter_types::runtime::maybe_grow_stack(|| {
                    <Identifier as ::treesitter_types::FromNode>::from_node(node, src)
                })?,
            ))),
            "object_pattern" => Ok(Self::ObjectPattern(::std::boxed::Box::new(
                ::treesitter_types::runtime::maybe_grow_stack(|| {
                    <ObjectPattern as ::treesitter_types::FromNode>::from_node(node, src)
                })?,
            ))),
            other => Err(::treesitter_types::ParseError::unexpected_kind(other, node)),
        }
    }
}
impl ::treesitter_types::Spanned for CatchClauseParameter<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        match self {
            Self::ArrayPattern(inner) => inner.span(),
            Self::Identifier(inner) => inner.span(),
            Self::ObjectPattern(inner) => inner.span(),
        }
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ClassBodyMember<'tree> {
    ClassStaticBlock(::std::boxed::Box<ClassStaticBlock<'tree>>),
    FieldDefinition(::std::boxed::Box<FieldDefinition<'tree>>),
    MethodDefinition(::std::boxed::Box<MethodDefinition<'tree>>),
}
impl<'tree> ::treesitter_types::FromNode<'tree> for ClassBodyMember<'tree> {
    #[allow(clippy::collapsible_else_if)]
    fn from_node(
        node: ::treesitter_types::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        match node.kind() {
            "class_static_block" => Ok(Self::ClassStaticBlock(::std::boxed::Box::new(
                ::treesitter_types::runtime::maybe_grow_stack(|| {
                    <ClassStaticBlock as ::treesitter_types::FromNode>::from_node(node, src)
                })?,
            ))),
            "field_definition" => Ok(Self::FieldDefinition(::std::boxed::Box::new(
                ::treesitter_types::runtime::maybe_grow_stack(|| {
                    <FieldDefinition as ::treesitter_types::FromNode>::from_node(node, src)
                })?,
            ))),
            "method_definition" => Ok(Self::MethodDefinition(::std::boxed::Box::new(
                ::treesitter_types::runtime::maybe_grow_stack(|| {
                    <MethodDefinition as ::treesitter_types::FromNode>::from_node(node, src)
                })?,
            ))),
            other => Err(::treesitter_types::ParseError::unexpected_kind(other, node)),
        }
    }
}
impl ::treesitter_types::Spanned for ClassBodyMember<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        match self {
            Self::ClassStaticBlock(inner) => inner.span(),
            Self::FieldDefinition(inner) => inner.span(),
            Self::MethodDefinition(inner) => inner.span(),
        }
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum DecoratorChildren<'tree> {
    CallExpression(::std::boxed::Box<CallExpression<'tree>>),
    Identifier(::std::boxed::Box<Identifier<'tree>>),
    MemberExpression(::std::boxed::Box<MemberExpression<'tree>>),
}
impl<'tree> ::treesitter_types::FromNode<'tree> for DecoratorChildren<'tree> {
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
            "member_expression" => Ok(Self::MemberExpression(::std::boxed::Box::new(
                ::treesitter_types::runtime::maybe_grow_stack(|| {
                    <MemberExpression as ::treesitter_types::FromNode>::from_node(node, src)
                })?,
            ))),
            other => Err(::treesitter_types::ParseError::unexpected_kind(other, node)),
        }
    }
}
impl ::treesitter_types::Spanned for DecoratorChildren<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        match self {
            Self::CallExpression(inner) => inner.span(),
            Self::Identifier(inner) => inner.span(),
            Self::MemberExpression(inner) => inner.span(),
        }
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ExportSpecifierAlias<'tree> {
    Default(::treesitter_types::Span),
    Identifier(::std::boxed::Box<Identifier<'tree>>),
    String(::std::boxed::Box<String<'tree>>),
}
impl<'tree> ::treesitter_types::FromNode<'tree> for ExportSpecifierAlias<'tree> {
    #[allow(clippy::collapsible_else_if)]
    fn from_node(
        node: ::treesitter_types::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        match node.kind() {
            "default" => Ok(Self::Default(::treesitter_types::Span::from(node))),
            "identifier" => Ok(Self::Identifier(::std::boxed::Box::new(
                ::treesitter_types::runtime::maybe_grow_stack(|| {
                    <Identifier as ::treesitter_types::FromNode>::from_node(node, src)
                })?,
            ))),
            "string" => Ok(Self::String(::std::boxed::Box::new(
                ::treesitter_types::runtime::maybe_grow_stack(|| {
                    <String as ::treesitter_types::FromNode>::from_node(node, src)
                })?,
            ))),
            other => Err(::treesitter_types::ParseError::unexpected_kind(other, node)),
        }
    }
}
impl ::treesitter_types::Spanned for ExportSpecifierAlias<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        match self {
            Self::Default(span) => *span,
            Self::Identifier(inner) => inner.span(),
            Self::String(inner) => inner.span(),
        }
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ExportSpecifierName<'tree> {
    Default(::treesitter_types::Span),
    Identifier(::std::boxed::Box<Identifier<'tree>>),
    String(::std::boxed::Box<String<'tree>>),
}
impl<'tree> ::treesitter_types::FromNode<'tree> for ExportSpecifierName<'tree> {
    #[allow(clippy::collapsible_else_if)]
    fn from_node(
        node: ::treesitter_types::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        match node.kind() {
            "default" => Ok(Self::Default(::treesitter_types::Span::from(node))),
            "identifier" => Ok(Self::Identifier(::std::boxed::Box::new(
                ::treesitter_types::runtime::maybe_grow_stack(|| {
                    <Identifier as ::treesitter_types::FromNode>::from_node(node, src)
                })?,
            ))),
            "string" => Ok(Self::String(::std::boxed::Box::new(
                ::treesitter_types::runtime::maybe_grow_stack(|| {
                    <String as ::treesitter_types::FromNode>::from_node(node, src)
                })?,
            ))),
            other => Err(::treesitter_types::ParseError::unexpected_kind(other, node)),
        }
    }
}
impl ::treesitter_types::Spanned for ExportSpecifierName<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        match self {
            Self::Default(span) => *span,
            Self::Identifier(inner) => inner.span(),
            Self::String(inner) => inner.span(),
        }
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ExportStatementChildren<'tree> {
    ExportClause(::std::boxed::Box<ExportClause<'tree>>),
    NamespaceExport(::std::boxed::Box<NamespaceExport<'tree>>),
}
impl<'tree> ::treesitter_types::FromNode<'tree> for ExportStatementChildren<'tree> {
    #[allow(clippy::collapsible_else_if)]
    fn from_node(
        node: ::treesitter_types::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        match node.kind() {
            "export_clause" => Ok(Self::ExportClause(::std::boxed::Box::new(
                ::treesitter_types::runtime::maybe_grow_stack(|| {
                    <ExportClause as ::treesitter_types::FromNode>::from_node(node, src)
                })?,
            ))),
            "namespace_export" => Ok(Self::NamespaceExport(::std::boxed::Box::new(
                ::treesitter_types::runtime::maybe_grow_stack(|| {
                    <NamespaceExport as ::treesitter_types::FromNode>::from_node(node, src)
                })?,
            ))),
            other => Err(::treesitter_types::ParseError::unexpected_kind(other, node)),
        }
    }
}
impl ::treesitter_types::Spanned for ExportStatementChildren<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        match self {
            Self::ExportClause(inner) => inner.span(),
            Self::NamespaceExport(inner) => inner.span(),
        }
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ExpressionStatementChildren<'tree> {
    Expression(::std::boxed::Box<Expression<'tree>>),
    SequenceExpression(::std::boxed::Box<SequenceExpression<'tree>>),
}
impl<'tree> ::treesitter_types::FromNode<'tree> for ExpressionStatementChildren<'tree> {
    #[allow(clippy::collapsible_else_if)]
    fn from_node(
        node: ::treesitter_types::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        match node.kind() {
            "sequence_expression" => Ok(Self::SequenceExpression(::std::boxed::Box::new(
                ::treesitter_types::runtime::maybe_grow_stack(|| {
                    <SequenceExpression as ::treesitter_types::FromNode>::from_node(node, src)
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
            Self::Expression(inner) => inner.span(),
            Self::SequenceExpression(inner) => inner.span(),
        }
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum FieldDefinitionProperty<'tree> {
    ComputedPropertyName(::std::boxed::Box<ComputedPropertyName<'tree>>),
    Number(::std::boxed::Box<Number<'tree>>),
    PrivatePropertyIdentifier(::std::boxed::Box<PrivatePropertyIdentifier<'tree>>),
    PropertyIdentifier(::std::boxed::Box<PropertyIdentifier<'tree>>),
    String(::std::boxed::Box<String<'tree>>),
}
impl<'tree> ::treesitter_types::FromNode<'tree> for FieldDefinitionProperty<'tree> {
    #[allow(clippy::collapsible_else_if)]
    fn from_node(
        node: ::treesitter_types::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        match node.kind() {
            "computed_property_name" => Ok(Self::ComputedPropertyName(::std::boxed::Box::new(
                ::treesitter_types::runtime::maybe_grow_stack(|| {
                    <ComputedPropertyName as ::treesitter_types::FromNode>::from_node(node, src)
                })?,
            ))),
            "number" => Ok(Self::Number(::std::boxed::Box::new(
                ::treesitter_types::runtime::maybe_grow_stack(|| {
                    <Number as ::treesitter_types::FromNode>::from_node(node, src)
                })?,
            ))),
            "private_property_identifier" => Ok(Self::PrivatePropertyIdentifier(
                ::std::boxed::Box::new(::treesitter_types::runtime::maybe_grow_stack(|| {
                    <PrivatePropertyIdentifier as ::treesitter_types::FromNode>::from_node(
                        node, src,
                    )
                })?),
            )),
            "property_identifier" => Ok(Self::PropertyIdentifier(::std::boxed::Box::new(
                ::treesitter_types::runtime::maybe_grow_stack(|| {
                    <PropertyIdentifier as ::treesitter_types::FromNode>::from_node(node, src)
                })?,
            ))),
            "string" => Ok(Self::String(::std::boxed::Box::new(
                ::treesitter_types::runtime::maybe_grow_stack(|| {
                    <String as ::treesitter_types::FromNode>::from_node(node, src)
                })?,
            ))),
            other => Err(::treesitter_types::ParseError::unexpected_kind(other, node)),
        }
    }
}
impl ::treesitter_types::Spanned for FieldDefinitionProperty<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        match self {
            Self::ComputedPropertyName(inner) => inner.span(),
            Self::Number(inner) => inner.span(),
            Self::PrivatePropertyIdentifier(inner) => inner.span(),
            Self::PropertyIdentifier(inner) => inner.span(),
            Self::String(inner) => inner.span(),
        }
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ForInStatementKind {
    Await(::treesitter_types::Span),
    Const(::treesitter_types::Span),
    Let(::treesitter_types::Span),
    Using(::treesitter_types::Span),
    Var(::treesitter_types::Span),
}
impl<'tree> ::treesitter_types::FromNode<'tree> for ForInStatementKind {
    #[allow(clippy::collapsible_else_if)]
    fn from_node(
        node: ::treesitter_types::tree_sitter::Node<'tree>,
        _src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        match node.kind() {
            "await" => Ok(Self::Await(::treesitter_types::Span::from(node))),
            "const" => Ok(Self::Const(::treesitter_types::Span::from(node))),
            "let" => Ok(Self::Let(::treesitter_types::Span::from(node))),
            "using" => Ok(Self::Using(::treesitter_types::Span::from(node))),
            "var" => Ok(Self::Var(::treesitter_types::Span::from(node))),
            other => Err(::treesitter_types::ParseError::unexpected_kind(other, node)),
        }
    }
}
impl ::treesitter_types::Spanned for ForInStatementKind {
    fn span(&self) -> ::treesitter_types::Span {
        match self {
            Self::Await(span) => *span,
            Self::Const(span) => *span,
            Self::Let(span) => *span,
            Self::Using(span) => *span,
            Self::Var(span) => *span,
        }
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ForInStatementLeft<'tree> {
    ArrayPattern(::std::boxed::Box<ArrayPattern<'tree>>),
    Identifier(::std::boxed::Box<Identifier<'tree>>),
    MemberExpression(::std::boxed::Box<MemberExpression<'tree>>),
    ObjectPattern(::std::boxed::Box<ObjectPattern<'tree>>),
    ParenthesizedExpression(::std::boxed::Box<ParenthesizedExpression<'tree>>),
    SubscriptExpression(::std::boxed::Box<SubscriptExpression<'tree>>),
    Undefined(::std::boxed::Box<Undefined<'tree>>),
}
impl<'tree> ::treesitter_types::FromNode<'tree> for ForInStatementLeft<'tree> {
    #[allow(clippy::collapsible_else_if)]
    fn from_node(
        node: ::treesitter_types::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        match node.kind() {
            "array_pattern" => Ok(Self::ArrayPattern(::std::boxed::Box::new(
                ::treesitter_types::runtime::maybe_grow_stack(|| {
                    <ArrayPattern as ::treesitter_types::FromNode>::from_node(node, src)
                })?,
            ))),
            "identifier" => Ok(Self::Identifier(::std::boxed::Box::new(
                ::treesitter_types::runtime::maybe_grow_stack(|| {
                    <Identifier as ::treesitter_types::FromNode>::from_node(node, src)
                })?,
            ))),
            "member_expression" => Ok(Self::MemberExpression(::std::boxed::Box::new(
                ::treesitter_types::runtime::maybe_grow_stack(|| {
                    <MemberExpression as ::treesitter_types::FromNode>::from_node(node, src)
                })?,
            ))),
            "object_pattern" => Ok(Self::ObjectPattern(::std::boxed::Box::new(
                ::treesitter_types::runtime::maybe_grow_stack(|| {
                    <ObjectPattern as ::treesitter_types::FromNode>::from_node(node, src)
                })?,
            ))),
            "parenthesized_expression" => Ok(Self::ParenthesizedExpression(
                ::std::boxed::Box::new(::treesitter_types::runtime::maybe_grow_stack(|| {
                    <ParenthesizedExpression as ::treesitter_types::FromNode>::from_node(node, src)
                })?),
            )),
            "subscript_expression" => Ok(Self::SubscriptExpression(::std::boxed::Box::new(
                ::treesitter_types::runtime::maybe_grow_stack(|| {
                    <SubscriptExpression as ::treesitter_types::FromNode>::from_node(node, src)
                })?,
            ))),
            "undefined" => Ok(Self::Undefined(::std::boxed::Box::new(
                ::treesitter_types::runtime::maybe_grow_stack(|| {
                    <Undefined as ::treesitter_types::FromNode>::from_node(node, src)
                })?,
            ))),
            other => Err(::treesitter_types::ParseError::unexpected_kind(other, node)),
        }
    }
}
impl ::treesitter_types::Spanned for ForInStatementLeft<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        match self {
            Self::ArrayPattern(inner) => inner.span(),
            Self::Identifier(inner) => inner.span(),
            Self::MemberExpression(inner) => inner.span(),
            Self::ObjectPattern(inner) => inner.span(),
            Self::ParenthesizedExpression(inner) => inner.span(),
            Self::SubscriptExpression(inner) => inner.span(),
            Self::Undefined(inner) => inner.span(),
        }
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ForInStatementOperator {
    In(::treesitter_types::Span),
    Of(::treesitter_types::Span),
}
impl<'tree> ::treesitter_types::FromNode<'tree> for ForInStatementOperator {
    #[allow(clippy::collapsible_else_if)]
    fn from_node(
        node: ::treesitter_types::tree_sitter::Node<'tree>,
        _src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        match node.kind() {
            "in" => Ok(Self::In(::treesitter_types::Span::from(node))),
            "of" => Ok(Self::Of(::treesitter_types::Span::from(node))),
            other => Err(::treesitter_types::ParseError::unexpected_kind(other, node)),
        }
    }
}
impl ::treesitter_types::Spanned for ForInStatementOperator {
    fn span(&self) -> ::treesitter_types::Span {
        match self {
            Self::In(span) => *span,
            Self::Of(span) => *span,
        }
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ForInStatementRight<'tree> {
    Expression(::std::boxed::Box<Expression<'tree>>),
    SequenceExpression(::std::boxed::Box<SequenceExpression<'tree>>),
}
impl<'tree> ::treesitter_types::FromNode<'tree> for ForInStatementRight<'tree> {
    #[allow(clippy::collapsible_else_if)]
    fn from_node(
        node: ::treesitter_types::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        match node.kind() {
            "sequence_expression" => Ok(Self::SequenceExpression(::std::boxed::Box::new(
                ::treesitter_types::runtime::maybe_grow_stack(|| {
                    <SequenceExpression as ::treesitter_types::FromNode>::from_node(node, src)
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
impl ::treesitter_types::Spanned for ForInStatementRight<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        match self {
            Self::Expression(inner) => inner.span(),
            Self::SequenceExpression(inner) => inner.span(),
        }
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ForStatementCondition<'tree> {
    Semicolon(::treesitter_types::Span),
    EmptyStatement(::std::boxed::Box<EmptyStatement<'tree>>),
    Expression(::std::boxed::Box<Expression<'tree>>),
    SequenceExpression(::std::boxed::Box<SequenceExpression<'tree>>),
}
impl<'tree> ::treesitter_types::FromNode<'tree> for ForStatementCondition<'tree> {
    #[allow(clippy::collapsible_else_if)]
    fn from_node(
        node: ::treesitter_types::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        match node.kind() {
            ";" => Ok(Self::Semicolon(::treesitter_types::Span::from(node))),
            "empty_statement" => Ok(Self::EmptyStatement(::std::boxed::Box::new(
                ::treesitter_types::runtime::maybe_grow_stack(|| {
                    <EmptyStatement as ::treesitter_types::FromNode>::from_node(node, src)
                })?,
            ))),
            "sequence_expression" => Ok(Self::SequenceExpression(::std::boxed::Box::new(
                ::treesitter_types::runtime::maybe_grow_stack(|| {
                    <SequenceExpression as ::treesitter_types::FromNode>::from_node(node, src)
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
            Self::Semicolon(span) => *span,
            Self::EmptyStatement(inner) => inner.span(),
            Self::Expression(inner) => inner.span(),
            Self::SequenceExpression(inner) => inner.span(),
        }
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ForStatementIncrement<'tree> {
    Expression(::std::boxed::Box<Expression<'tree>>),
    SequenceExpression(::std::boxed::Box<SequenceExpression<'tree>>),
}
impl<'tree> ::treesitter_types::FromNode<'tree> for ForStatementIncrement<'tree> {
    #[allow(clippy::collapsible_else_if)]
    fn from_node(
        node: ::treesitter_types::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        match node.kind() {
            "sequence_expression" => Ok(Self::SequenceExpression(::std::boxed::Box::new(
                ::treesitter_types::runtime::maybe_grow_stack(|| {
                    <SequenceExpression as ::treesitter_types::FromNode>::from_node(node, src)
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
impl ::treesitter_types::Spanned for ForStatementIncrement<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        match self {
            Self::Expression(inner) => inner.span(),
            Self::SequenceExpression(inner) => inner.span(),
        }
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ForStatementInitializer<'tree> {
    EmptyStatement(::std::boxed::Box<EmptyStatement<'tree>>),
    Expression(::std::boxed::Box<Expression<'tree>>),
    LexicalDeclaration(::std::boxed::Box<LexicalDeclaration<'tree>>),
    SequenceExpression(::std::boxed::Box<SequenceExpression<'tree>>),
    VariableDeclaration(::std::boxed::Box<VariableDeclaration<'tree>>),
}
impl<'tree> ::treesitter_types::FromNode<'tree> for ForStatementInitializer<'tree> {
    #[allow(clippy::collapsible_else_if)]
    fn from_node(
        node: ::treesitter_types::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        match node.kind() {
            "empty_statement" => Ok(Self::EmptyStatement(::std::boxed::Box::new(
                ::treesitter_types::runtime::maybe_grow_stack(|| {
                    <EmptyStatement as ::treesitter_types::FromNode>::from_node(node, src)
                })?,
            ))),
            "lexical_declaration" => Ok(Self::LexicalDeclaration(::std::boxed::Box::new(
                ::treesitter_types::runtime::maybe_grow_stack(|| {
                    <LexicalDeclaration as ::treesitter_types::FromNode>::from_node(node, src)
                })?,
            ))),
            "sequence_expression" => Ok(Self::SequenceExpression(::std::boxed::Box::new(
                ::treesitter_types::runtime::maybe_grow_stack(|| {
                    <SequenceExpression as ::treesitter_types::FromNode>::from_node(node, src)
                })?,
            ))),
            "variable_declaration" => Ok(Self::VariableDeclaration(::std::boxed::Box::new(
                ::treesitter_types::runtime::maybe_grow_stack(|| {
                    <VariableDeclaration as ::treesitter_types::FromNode>::from_node(node, src)
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
            Self::EmptyStatement(inner) => inner.span(),
            Self::Expression(inner) => inner.span(),
            Self::LexicalDeclaration(inner) => inner.span(),
            Self::SequenceExpression(inner) => inner.span(),
            Self::VariableDeclaration(inner) => inner.span(),
        }
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum FormalParametersChildren<'tree> {
    AssignmentPattern(::std::boxed::Box<AssignmentPattern<'tree>>),
    Pattern(::std::boxed::Box<Pattern<'tree>>),
}
impl<'tree> ::treesitter_types::FromNode<'tree> for FormalParametersChildren<'tree> {
    #[allow(clippy::collapsible_else_if)]
    fn from_node(
        node: ::treesitter_types::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        match node.kind() {
            "assignment_pattern" => Ok(Self::AssignmentPattern(::std::boxed::Box::new(
                ::treesitter_types::runtime::maybe_grow_stack(|| {
                    <AssignmentPattern as ::treesitter_types::FromNode>::from_node(node, src)
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
impl ::treesitter_types::Spanned for FormalParametersChildren<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        match self {
            Self::AssignmentPattern(inner) => inner.span(),
            Self::Pattern(inner) => inner.span(),
        }
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ImportClauseChildren<'tree> {
    Identifier(::std::boxed::Box<Identifier<'tree>>),
    NamedImports(::std::boxed::Box<NamedImports<'tree>>),
    NamespaceImport(::std::boxed::Box<NamespaceImport<'tree>>),
}
impl<'tree> ::treesitter_types::FromNode<'tree> for ImportClauseChildren<'tree> {
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
            "named_imports" => Ok(Self::NamedImports(::std::boxed::Box::new(
                ::treesitter_types::runtime::maybe_grow_stack(|| {
                    <NamedImports as ::treesitter_types::FromNode>::from_node(node, src)
                })?,
            ))),
            "namespace_import" => Ok(Self::NamespaceImport(::std::boxed::Box::new(
                ::treesitter_types::runtime::maybe_grow_stack(|| {
                    <NamespaceImport as ::treesitter_types::FromNode>::from_node(node, src)
                })?,
            ))),
            other => Err(::treesitter_types::ParseError::unexpected_kind(other, node)),
        }
    }
}
impl ::treesitter_types::Spanned for ImportClauseChildren<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        match self {
            Self::Identifier(inner) => inner.span(),
            Self::NamedImports(inner) => inner.span(),
            Self::NamespaceImport(inner) => inner.span(),
        }
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ImportSpecifierName<'tree> {
    Default(::treesitter_types::Span),
    Identifier(::std::boxed::Box<Identifier<'tree>>),
    String(::std::boxed::Box<String<'tree>>),
}
impl<'tree> ::treesitter_types::FromNode<'tree> for ImportSpecifierName<'tree> {
    #[allow(clippy::collapsible_else_if)]
    fn from_node(
        node: ::treesitter_types::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        match node.kind() {
            "default" => Ok(Self::Default(::treesitter_types::Span::from(node))),
            "identifier" => Ok(Self::Identifier(::std::boxed::Box::new(
                ::treesitter_types::runtime::maybe_grow_stack(|| {
                    <Identifier as ::treesitter_types::FromNode>::from_node(node, src)
                })?,
            ))),
            "string" => Ok(Self::String(::std::boxed::Box::new(
                ::treesitter_types::runtime::maybe_grow_stack(|| {
                    <String as ::treesitter_types::FromNode>::from_node(node, src)
                })?,
            ))),
            other => Err(::treesitter_types::ParseError::unexpected_kind(other, node)),
        }
    }
}
impl ::treesitter_types::Spanned for ImportSpecifierName<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        match self {
            Self::Default(span) => *span,
            Self::Identifier(inner) => inner.span(),
            Self::String(inner) => inner.span(),
        }
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ImportStatementChildren<'tree> {
    ImportAttribute(::std::boxed::Box<ImportAttribute<'tree>>),
    ImportClause(::std::boxed::Box<ImportClause<'tree>>),
}
impl<'tree> ::treesitter_types::FromNode<'tree> for ImportStatementChildren<'tree> {
    #[allow(clippy::collapsible_else_if)]
    fn from_node(
        node: ::treesitter_types::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        match node.kind() {
            "import_attribute" => Ok(Self::ImportAttribute(::std::boxed::Box::new(
                ::treesitter_types::runtime::maybe_grow_stack(|| {
                    <ImportAttribute as ::treesitter_types::FromNode>::from_node(node, src)
                })?,
            ))),
            "import_clause" => Ok(Self::ImportClause(::std::boxed::Box::new(
                ::treesitter_types::runtime::maybe_grow_stack(|| {
                    <ImportClause as ::treesitter_types::FromNode>::from_node(node, src)
                })?,
            ))),
            other => Err(::treesitter_types::ParseError::unexpected_kind(other, node)),
        }
    }
}
impl ::treesitter_types::Spanned for ImportStatementChildren<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        match self {
            Self::ImportAttribute(inner) => inner.span(),
            Self::ImportClause(inner) => inner.span(),
        }
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum JsxAttributeChildren<'tree> {
    JsxElement(::std::boxed::Box<JsxElement<'tree>>),
    JsxExpression(::std::boxed::Box<JsxExpression<'tree>>),
    JsxNamespaceName(::std::boxed::Box<JsxNamespaceName<'tree>>),
    JsxSelfClosingElement(::std::boxed::Box<JsxSelfClosingElement<'tree>>),
    PropertyIdentifier(::std::boxed::Box<PropertyIdentifier<'tree>>),
    String(::std::boxed::Box<String<'tree>>),
}
impl<'tree> ::treesitter_types::FromNode<'tree> for JsxAttributeChildren<'tree> {
    #[allow(clippy::collapsible_else_if)]
    fn from_node(
        node: ::treesitter_types::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        match node.kind() {
            "jsx_element" => Ok(Self::JsxElement(::std::boxed::Box::new(
                ::treesitter_types::runtime::maybe_grow_stack(|| {
                    <JsxElement as ::treesitter_types::FromNode>::from_node(node, src)
                })?,
            ))),
            "jsx_expression" => Ok(Self::JsxExpression(::std::boxed::Box::new(
                ::treesitter_types::runtime::maybe_grow_stack(|| {
                    <JsxExpression as ::treesitter_types::FromNode>::from_node(node, src)
                })?,
            ))),
            "jsx_namespace_name" => Ok(Self::JsxNamespaceName(::std::boxed::Box::new(
                ::treesitter_types::runtime::maybe_grow_stack(|| {
                    <JsxNamespaceName as ::treesitter_types::FromNode>::from_node(node, src)
                })?,
            ))),
            "jsx_self_closing_element" => Ok(Self::JsxSelfClosingElement(::std::boxed::Box::new(
                ::treesitter_types::runtime::maybe_grow_stack(|| {
                    <JsxSelfClosingElement as ::treesitter_types::FromNode>::from_node(node, src)
                })?,
            ))),
            "property_identifier" => Ok(Self::PropertyIdentifier(::std::boxed::Box::new(
                ::treesitter_types::runtime::maybe_grow_stack(|| {
                    <PropertyIdentifier as ::treesitter_types::FromNode>::from_node(node, src)
                })?,
            ))),
            "string" => Ok(Self::String(::std::boxed::Box::new(
                ::treesitter_types::runtime::maybe_grow_stack(|| {
                    <String as ::treesitter_types::FromNode>::from_node(node, src)
                })?,
            ))),
            other => Err(::treesitter_types::ParseError::unexpected_kind(other, node)),
        }
    }
}
impl ::treesitter_types::Spanned for JsxAttributeChildren<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        match self {
            Self::JsxElement(inner) => inner.span(),
            Self::JsxExpression(inner) => inner.span(),
            Self::JsxNamespaceName(inner) => inner.span(),
            Self::JsxSelfClosingElement(inner) => inner.span(),
            Self::PropertyIdentifier(inner) => inner.span(),
            Self::String(inner) => inner.span(),
        }
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum JsxClosingElementName<'tree> {
    Identifier(::std::boxed::Box<Identifier<'tree>>),
    JsxNamespaceName(::std::boxed::Box<JsxNamespaceName<'tree>>),
    MemberExpression(::std::boxed::Box<MemberExpression<'tree>>),
}
impl<'tree> ::treesitter_types::FromNode<'tree> for JsxClosingElementName<'tree> {
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
            "jsx_namespace_name" => Ok(Self::JsxNamespaceName(::std::boxed::Box::new(
                ::treesitter_types::runtime::maybe_grow_stack(|| {
                    <JsxNamespaceName as ::treesitter_types::FromNode>::from_node(node, src)
                })?,
            ))),
            "member_expression" => Ok(Self::MemberExpression(::std::boxed::Box::new(
                ::treesitter_types::runtime::maybe_grow_stack(|| {
                    <MemberExpression as ::treesitter_types::FromNode>::from_node(node, src)
                })?,
            ))),
            other => Err(::treesitter_types::ParseError::unexpected_kind(other, node)),
        }
    }
}
impl ::treesitter_types::Spanned for JsxClosingElementName<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        match self {
            Self::Identifier(inner) => inner.span(),
            Self::JsxNamespaceName(inner) => inner.span(),
            Self::MemberExpression(inner) => inner.span(),
        }
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum JsxElementChildren<'tree> {
    HtmlCharacterReference(::std::boxed::Box<HtmlCharacterReference<'tree>>),
    JsxElement(::std::boxed::Box<JsxElement<'tree>>),
    JsxExpression(::std::boxed::Box<JsxExpression<'tree>>),
    JsxSelfClosingElement(::std::boxed::Box<JsxSelfClosingElement<'tree>>),
    JsxText(::std::boxed::Box<JsxText<'tree>>),
}
impl<'tree> ::treesitter_types::FromNode<'tree> for JsxElementChildren<'tree> {
    #[allow(clippy::collapsible_else_if)]
    fn from_node(
        node: ::treesitter_types::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        match node.kind() {
            "html_character_reference" => Ok(Self::HtmlCharacterReference(::std::boxed::Box::new(
                ::treesitter_types::runtime::maybe_grow_stack(|| {
                    <HtmlCharacterReference as ::treesitter_types::FromNode>::from_node(node, src)
                })?,
            ))),
            "jsx_element" => Ok(Self::JsxElement(::std::boxed::Box::new(
                ::treesitter_types::runtime::maybe_grow_stack(|| {
                    <JsxElement as ::treesitter_types::FromNode>::from_node(node, src)
                })?,
            ))),
            "jsx_expression" => Ok(Self::JsxExpression(::std::boxed::Box::new(
                ::treesitter_types::runtime::maybe_grow_stack(|| {
                    <JsxExpression as ::treesitter_types::FromNode>::from_node(node, src)
                })?,
            ))),
            "jsx_self_closing_element" => Ok(Self::JsxSelfClosingElement(::std::boxed::Box::new(
                ::treesitter_types::runtime::maybe_grow_stack(|| {
                    <JsxSelfClosingElement as ::treesitter_types::FromNode>::from_node(node, src)
                })?,
            ))),
            "jsx_text" => Ok(Self::JsxText(::std::boxed::Box::new(
                ::treesitter_types::runtime::maybe_grow_stack(|| {
                    <JsxText as ::treesitter_types::FromNode>::from_node(node, src)
                })?,
            ))),
            other => Err(::treesitter_types::ParseError::unexpected_kind(other, node)),
        }
    }
}
impl ::treesitter_types::Spanned for JsxElementChildren<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        match self {
            Self::HtmlCharacterReference(inner) => inner.span(),
            Self::JsxElement(inner) => inner.span(),
            Self::JsxExpression(inner) => inner.span(),
            Self::JsxSelfClosingElement(inner) => inner.span(),
            Self::JsxText(inner) => inner.span(),
        }
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum JsxExpressionChildren<'tree> {
    Expression(::std::boxed::Box<Expression<'tree>>),
    SequenceExpression(::std::boxed::Box<SequenceExpression<'tree>>),
    SpreadElement(::std::boxed::Box<SpreadElement<'tree>>),
}
impl<'tree> ::treesitter_types::FromNode<'tree> for JsxExpressionChildren<'tree> {
    #[allow(clippy::collapsible_else_if)]
    fn from_node(
        node: ::treesitter_types::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        match node.kind() {
            "sequence_expression" => Ok(Self::SequenceExpression(::std::boxed::Box::new(
                ::treesitter_types::runtime::maybe_grow_stack(|| {
                    <SequenceExpression as ::treesitter_types::FromNode>::from_node(node, src)
                })?,
            ))),
            "spread_element" => Ok(Self::SpreadElement(::std::boxed::Box::new(
                ::treesitter_types::runtime::maybe_grow_stack(|| {
                    <SpreadElement as ::treesitter_types::FromNode>::from_node(node, src)
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
impl ::treesitter_types::Spanned for JsxExpressionChildren<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        match self {
            Self::Expression(inner) => inner.span(),
            Self::SequenceExpression(inner) => inner.span(),
            Self::SpreadElement(inner) => inner.span(),
        }
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum JsxOpeningElementAttribute<'tree> {
    JsxAttribute(::std::boxed::Box<JsxAttribute<'tree>>),
    JsxExpression(::std::boxed::Box<JsxExpression<'tree>>),
}
impl<'tree> ::treesitter_types::FromNode<'tree> for JsxOpeningElementAttribute<'tree> {
    #[allow(clippy::collapsible_else_if)]
    fn from_node(
        node: ::treesitter_types::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        match node.kind() {
            "jsx_attribute" => Ok(Self::JsxAttribute(::std::boxed::Box::new(
                ::treesitter_types::runtime::maybe_grow_stack(|| {
                    <JsxAttribute as ::treesitter_types::FromNode>::from_node(node, src)
                })?,
            ))),
            "jsx_expression" => Ok(Self::JsxExpression(::std::boxed::Box::new(
                ::treesitter_types::runtime::maybe_grow_stack(|| {
                    <JsxExpression as ::treesitter_types::FromNode>::from_node(node, src)
                })?,
            ))),
            other => Err(::treesitter_types::ParseError::unexpected_kind(other, node)),
        }
    }
}
impl ::treesitter_types::Spanned for JsxOpeningElementAttribute<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        match self {
            Self::JsxAttribute(inner) => inner.span(),
            Self::JsxExpression(inner) => inner.span(),
        }
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum JsxOpeningElementName<'tree> {
    Identifier(::std::boxed::Box<Identifier<'tree>>),
    JsxNamespaceName(::std::boxed::Box<JsxNamespaceName<'tree>>),
    MemberExpression(::std::boxed::Box<MemberExpression<'tree>>),
}
impl<'tree> ::treesitter_types::FromNode<'tree> for JsxOpeningElementName<'tree> {
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
            "jsx_namespace_name" => Ok(Self::JsxNamespaceName(::std::boxed::Box::new(
                ::treesitter_types::runtime::maybe_grow_stack(|| {
                    <JsxNamespaceName as ::treesitter_types::FromNode>::from_node(node, src)
                })?,
            ))),
            "member_expression" => Ok(Self::MemberExpression(::std::boxed::Box::new(
                ::treesitter_types::runtime::maybe_grow_stack(|| {
                    <MemberExpression as ::treesitter_types::FromNode>::from_node(node, src)
                })?,
            ))),
            other => Err(::treesitter_types::ParseError::unexpected_kind(other, node)),
        }
    }
}
impl ::treesitter_types::Spanned for JsxOpeningElementName<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        match self {
            Self::Identifier(inner) => inner.span(),
            Self::JsxNamespaceName(inner) => inner.span(),
            Self::MemberExpression(inner) => inner.span(),
        }
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum JsxSelfClosingElementAttribute<'tree> {
    JsxAttribute(::std::boxed::Box<JsxAttribute<'tree>>),
    JsxExpression(::std::boxed::Box<JsxExpression<'tree>>),
}
impl<'tree> ::treesitter_types::FromNode<'tree> for JsxSelfClosingElementAttribute<'tree> {
    #[allow(clippy::collapsible_else_if)]
    fn from_node(
        node: ::treesitter_types::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        match node.kind() {
            "jsx_attribute" => Ok(Self::JsxAttribute(::std::boxed::Box::new(
                ::treesitter_types::runtime::maybe_grow_stack(|| {
                    <JsxAttribute as ::treesitter_types::FromNode>::from_node(node, src)
                })?,
            ))),
            "jsx_expression" => Ok(Self::JsxExpression(::std::boxed::Box::new(
                ::treesitter_types::runtime::maybe_grow_stack(|| {
                    <JsxExpression as ::treesitter_types::FromNode>::from_node(node, src)
                })?,
            ))),
            other => Err(::treesitter_types::ParseError::unexpected_kind(other, node)),
        }
    }
}
impl ::treesitter_types::Spanned for JsxSelfClosingElementAttribute<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        match self {
            Self::JsxAttribute(inner) => inner.span(),
            Self::JsxExpression(inner) => inner.span(),
        }
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum JsxSelfClosingElementName<'tree> {
    Identifier(::std::boxed::Box<Identifier<'tree>>),
    JsxNamespaceName(::std::boxed::Box<JsxNamespaceName<'tree>>),
    MemberExpression(::std::boxed::Box<MemberExpression<'tree>>),
}
impl<'tree> ::treesitter_types::FromNode<'tree> for JsxSelfClosingElementName<'tree> {
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
            "jsx_namespace_name" => Ok(Self::JsxNamespaceName(::std::boxed::Box::new(
                ::treesitter_types::runtime::maybe_grow_stack(|| {
                    <JsxNamespaceName as ::treesitter_types::FromNode>::from_node(node, src)
                })?,
            ))),
            "member_expression" => Ok(Self::MemberExpression(::std::boxed::Box::new(
                ::treesitter_types::runtime::maybe_grow_stack(|| {
                    <MemberExpression as ::treesitter_types::FromNode>::from_node(node, src)
                })?,
            ))),
            other => Err(::treesitter_types::ParseError::unexpected_kind(other, node)),
        }
    }
}
impl ::treesitter_types::Spanned for JsxSelfClosingElementName<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        match self {
            Self::Identifier(inner) => inner.span(),
            Self::JsxNamespaceName(inner) => inner.span(),
            Self::MemberExpression(inner) => inner.span(),
        }
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum LexicalDeclarationKind {
    Const(::treesitter_types::Span),
    Let(::treesitter_types::Span),
}
impl<'tree> ::treesitter_types::FromNode<'tree> for LexicalDeclarationKind {
    #[allow(clippy::collapsible_else_if)]
    fn from_node(
        node: ::treesitter_types::tree_sitter::Node<'tree>,
        _src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        match node.kind() {
            "const" => Ok(Self::Const(::treesitter_types::Span::from(node))),
            "let" => Ok(Self::Let(::treesitter_types::Span::from(node))),
            other => Err(::treesitter_types::ParseError::unexpected_kind(other, node)),
        }
    }
}
impl ::treesitter_types::Spanned for LexicalDeclarationKind {
    fn span(&self) -> ::treesitter_types::Span {
        match self {
            Self::Const(span) => *span,
            Self::Let(span) => *span,
        }
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum MemberExpressionObject<'tree> {
    Expression(::std::boxed::Box<Expression<'tree>>),
    Import(::std::boxed::Box<Import<'tree>>),
}
impl<'tree> ::treesitter_types::FromNode<'tree> for MemberExpressionObject<'tree> {
    #[allow(clippy::collapsible_else_if)]
    fn from_node(
        node: ::treesitter_types::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        match node.kind() {
            "import" => Ok(Self::Import(::std::boxed::Box::new(
                ::treesitter_types::runtime::maybe_grow_stack(|| {
                    <Import as ::treesitter_types::FromNode>::from_node(node, src)
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
impl ::treesitter_types::Spanned for MemberExpressionObject<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        match self {
            Self::Expression(inner) => inner.span(),
            Self::Import(inner) => inner.span(),
        }
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum MemberExpressionProperty<'tree> {
    PrivatePropertyIdentifier(::std::boxed::Box<PrivatePropertyIdentifier<'tree>>),
    PropertyIdentifier(::std::boxed::Box<PropertyIdentifier<'tree>>),
}
impl<'tree> ::treesitter_types::FromNode<'tree> for MemberExpressionProperty<'tree> {
    #[allow(clippy::collapsible_else_if)]
    fn from_node(
        node: ::treesitter_types::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        match node.kind() {
            "private_property_identifier" => Ok(Self::PrivatePropertyIdentifier(
                ::std::boxed::Box::new(::treesitter_types::runtime::maybe_grow_stack(|| {
                    <PrivatePropertyIdentifier as ::treesitter_types::FromNode>::from_node(
                        node, src,
                    )
                })?),
            )),
            "property_identifier" => Ok(Self::PropertyIdentifier(::std::boxed::Box::new(
                ::treesitter_types::runtime::maybe_grow_stack(|| {
                    <PropertyIdentifier as ::treesitter_types::FromNode>::from_node(node, src)
                })?,
            ))),
            other => Err(::treesitter_types::ParseError::unexpected_kind(other, node)),
        }
    }
}
impl ::treesitter_types::Spanned for MemberExpressionProperty<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        match self {
            Self::PrivatePropertyIdentifier(inner) => inner.span(),
            Self::PropertyIdentifier(inner) => inner.span(),
        }
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum MethodDefinitionName<'tree> {
    ComputedPropertyName(::std::boxed::Box<ComputedPropertyName<'tree>>),
    Number(::std::boxed::Box<Number<'tree>>),
    PrivatePropertyIdentifier(::std::boxed::Box<PrivatePropertyIdentifier<'tree>>),
    PropertyIdentifier(::std::boxed::Box<PropertyIdentifier<'tree>>),
    String(::std::boxed::Box<String<'tree>>),
}
impl<'tree> ::treesitter_types::FromNode<'tree> for MethodDefinitionName<'tree> {
    #[allow(clippy::collapsible_else_if)]
    fn from_node(
        node: ::treesitter_types::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        match node.kind() {
            "computed_property_name" => Ok(Self::ComputedPropertyName(::std::boxed::Box::new(
                ::treesitter_types::runtime::maybe_grow_stack(|| {
                    <ComputedPropertyName as ::treesitter_types::FromNode>::from_node(node, src)
                })?,
            ))),
            "number" => Ok(Self::Number(::std::boxed::Box::new(
                ::treesitter_types::runtime::maybe_grow_stack(|| {
                    <Number as ::treesitter_types::FromNode>::from_node(node, src)
                })?,
            ))),
            "private_property_identifier" => Ok(Self::PrivatePropertyIdentifier(
                ::std::boxed::Box::new(::treesitter_types::runtime::maybe_grow_stack(|| {
                    <PrivatePropertyIdentifier as ::treesitter_types::FromNode>::from_node(
                        node, src,
                    )
                })?),
            )),
            "property_identifier" => Ok(Self::PropertyIdentifier(::std::boxed::Box::new(
                ::treesitter_types::runtime::maybe_grow_stack(|| {
                    <PropertyIdentifier as ::treesitter_types::FromNode>::from_node(node, src)
                })?,
            ))),
            "string" => Ok(Self::String(::std::boxed::Box::new(
                ::treesitter_types::runtime::maybe_grow_stack(|| {
                    <String as ::treesitter_types::FromNode>::from_node(node, src)
                })?,
            ))),
            other => Err(::treesitter_types::ParseError::unexpected_kind(other, node)),
        }
    }
}
impl ::treesitter_types::Spanned for MethodDefinitionName<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        match self {
            Self::ComputedPropertyName(inner) => inner.span(),
            Self::Number(inner) => inner.span(),
            Self::PrivatePropertyIdentifier(inner) => inner.span(),
            Self::PropertyIdentifier(inner) => inner.span(),
            Self::String(inner) => inner.span(),
        }
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum NamespaceExportChildren<'tree> {
    Identifier(::std::boxed::Box<Identifier<'tree>>),
    String(::std::boxed::Box<String<'tree>>),
}
impl<'tree> ::treesitter_types::FromNode<'tree> for NamespaceExportChildren<'tree> {
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
            "string" => Ok(Self::String(::std::boxed::Box::new(
                ::treesitter_types::runtime::maybe_grow_stack(|| {
                    <String as ::treesitter_types::FromNode>::from_node(node, src)
                })?,
            ))),
            other => Err(::treesitter_types::ParseError::unexpected_kind(other, node)),
        }
    }
}
impl ::treesitter_types::Spanned for NamespaceExportChildren<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        match self {
            Self::Identifier(inner) => inner.span(),
            Self::String(inner) => inner.span(),
        }
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum NewExpressionConstructor<'tree> {
    NewExpression(::std::boxed::Box<NewExpression<'tree>>),
    PrimaryExpression(::std::boxed::Box<PrimaryExpression<'tree>>),
}
impl<'tree> ::treesitter_types::FromNode<'tree> for NewExpressionConstructor<'tree> {
    #[allow(clippy::collapsible_else_if)]
    fn from_node(
        node: ::treesitter_types::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        match node.kind() {
            "new_expression" => Ok(Self::NewExpression(::std::boxed::Box::new(
                ::treesitter_types::runtime::maybe_grow_stack(|| {
                    <NewExpression as ::treesitter_types::FromNode>::from_node(node, src)
                })?,
            ))),
            _other => {
                if let Ok(v) = ::treesitter_types::runtime::maybe_grow_stack(|| {
                    <PrimaryExpression as ::treesitter_types::FromNode>::from_node(node, src)
                }) {
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
impl ::treesitter_types::Spanned for NewExpressionConstructor<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        match self {
            Self::NewExpression(inner) => inner.span(),
            Self::PrimaryExpression(inner) => inner.span(),
        }
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ObjectChildren<'tree> {
    MethodDefinition(::std::boxed::Box<MethodDefinition<'tree>>),
    Pair(::std::boxed::Box<Pair<'tree>>),
    ShorthandPropertyIdentifier(::std::boxed::Box<ShorthandPropertyIdentifier<'tree>>),
    SpreadElement(::std::boxed::Box<SpreadElement<'tree>>),
}
impl<'tree> ::treesitter_types::FromNode<'tree> for ObjectChildren<'tree> {
    #[allow(clippy::collapsible_else_if)]
    fn from_node(
        node: ::treesitter_types::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        match node.kind() {
            "method_definition" => Ok(Self::MethodDefinition(::std::boxed::Box::new(
                ::treesitter_types::runtime::maybe_grow_stack(|| {
                    <MethodDefinition as ::treesitter_types::FromNode>::from_node(node, src)
                })?,
            ))),
            "pair" => Ok(Self::Pair(::std::boxed::Box::new(
                ::treesitter_types::runtime::maybe_grow_stack(|| {
                    <Pair as ::treesitter_types::FromNode>::from_node(node, src)
                })?,
            ))),
            "shorthand_property_identifier" => Ok(Self::ShorthandPropertyIdentifier(
                ::std::boxed::Box::new(::treesitter_types::runtime::maybe_grow_stack(|| {
                    <ShorthandPropertyIdentifier as ::treesitter_types::FromNode>::from_node(
                        node, src,
                    )
                })?),
            )),
            "spread_element" => Ok(Self::SpreadElement(::std::boxed::Box::new(
                ::treesitter_types::runtime::maybe_grow_stack(|| {
                    <SpreadElement as ::treesitter_types::FromNode>::from_node(node, src)
                })?,
            ))),
            other => Err(::treesitter_types::ParseError::unexpected_kind(other, node)),
        }
    }
}
impl ::treesitter_types::Spanned for ObjectChildren<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        match self {
            Self::MethodDefinition(inner) => inner.span(),
            Self::Pair(inner) => inner.span(),
            Self::ShorthandPropertyIdentifier(inner) => inner.span(),
            Self::SpreadElement(inner) => inner.span(),
        }
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ObjectAssignmentPatternLeft<'tree> {
    ArrayPattern(::std::boxed::Box<ArrayPattern<'tree>>),
    ObjectPattern(::std::boxed::Box<ObjectPattern<'tree>>),
    ShorthandPropertyIdentifierPattern(
        ::std::boxed::Box<ShorthandPropertyIdentifierPattern<'tree>>,
    ),
}
impl<'tree> ::treesitter_types::FromNode<'tree> for ObjectAssignmentPatternLeft<'tree> {
    #[allow(clippy::collapsible_else_if)]
    fn from_node(
        node: ::treesitter_types::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        match node.kind() {
            "array_pattern" => Ok(Self::ArrayPattern(::std::boxed::Box::new(
                ::treesitter_types::runtime::maybe_grow_stack(|| {
                    <ArrayPattern as ::treesitter_types::FromNode>::from_node(node, src)
                })?,
            ))),
            "object_pattern" => Ok(Self::ObjectPattern(::std::boxed::Box::new(
                ::treesitter_types::runtime::maybe_grow_stack(|| {
                    <ObjectPattern as ::treesitter_types::FromNode>::from_node(node, src)
                })?,
            ))),
            "shorthand_property_identifier_pattern" => {
                Ok(Self::ShorthandPropertyIdentifierPattern(
                    ::std::boxed::Box::new(::treesitter_types::runtime::maybe_grow_stack(|| {
                        <ShorthandPropertyIdentifierPattern as ::treesitter_types::FromNode>::from_node(
                                node,
                                src,
                            )
                    })?),
                ))
            }
            other => Err(::treesitter_types::ParseError::unexpected_kind(other, node)),
        }
    }
}
impl ::treesitter_types::Spanned for ObjectAssignmentPatternLeft<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        match self {
            Self::ArrayPattern(inner) => inner.span(),
            Self::ObjectPattern(inner) => inner.span(),
            Self::ShorthandPropertyIdentifierPattern(inner) => inner.span(),
        }
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ObjectPatternChildren<'tree> {
    ObjectAssignmentPattern(::std::boxed::Box<ObjectAssignmentPattern<'tree>>),
    PairPattern(::std::boxed::Box<PairPattern<'tree>>),
    RestPattern(::std::boxed::Box<RestPattern<'tree>>),
    ShorthandPropertyIdentifierPattern(
        ::std::boxed::Box<ShorthandPropertyIdentifierPattern<'tree>>,
    ),
}
impl<'tree> ::treesitter_types::FromNode<'tree> for ObjectPatternChildren<'tree> {
    #[allow(clippy::collapsible_else_if)]
    fn from_node(
        node: ::treesitter_types::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        match node.kind() {
            "object_assignment_pattern" => Ok(Self::ObjectAssignmentPattern(
                ::std::boxed::Box::new(::treesitter_types::runtime::maybe_grow_stack(|| {
                    <ObjectAssignmentPattern as ::treesitter_types::FromNode>::from_node(node, src)
                })?),
            )),
            "pair_pattern" => Ok(Self::PairPattern(::std::boxed::Box::new(
                ::treesitter_types::runtime::maybe_grow_stack(|| {
                    <PairPattern as ::treesitter_types::FromNode>::from_node(node, src)
                })?,
            ))),
            "rest_pattern" => Ok(Self::RestPattern(::std::boxed::Box::new(
                ::treesitter_types::runtime::maybe_grow_stack(|| {
                    <RestPattern as ::treesitter_types::FromNode>::from_node(node, src)
                })?,
            ))),
            "shorthand_property_identifier_pattern" => {
                Ok(Self::ShorthandPropertyIdentifierPattern(
                    ::std::boxed::Box::new(::treesitter_types::runtime::maybe_grow_stack(|| {
                        <ShorthandPropertyIdentifierPattern as ::treesitter_types::FromNode>::from_node(
                                node,
                                src,
                            )
                    })?),
                ))
            }
            other => Err(::treesitter_types::ParseError::unexpected_kind(other, node)),
        }
    }
}
impl ::treesitter_types::Spanned for ObjectPatternChildren<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        match self {
            Self::ObjectAssignmentPattern(inner) => inner.span(),
            Self::PairPattern(inner) => inner.span(),
            Self::RestPattern(inner) => inner.span(),
            Self::ShorthandPropertyIdentifierPattern(inner) => inner.span(),
        }
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum PairKey<'tree> {
    ComputedPropertyName(::std::boxed::Box<ComputedPropertyName<'tree>>),
    Number(::std::boxed::Box<Number<'tree>>),
    PrivatePropertyIdentifier(::std::boxed::Box<PrivatePropertyIdentifier<'tree>>),
    PropertyIdentifier(::std::boxed::Box<PropertyIdentifier<'tree>>),
    String(::std::boxed::Box<String<'tree>>),
}
impl<'tree> ::treesitter_types::FromNode<'tree> for PairKey<'tree> {
    #[allow(clippy::collapsible_else_if)]
    fn from_node(
        node: ::treesitter_types::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        match node.kind() {
            "computed_property_name" => Ok(Self::ComputedPropertyName(::std::boxed::Box::new(
                ::treesitter_types::runtime::maybe_grow_stack(|| {
                    <ComputedPropertyName as ::treesitter_types::FromNode>::from_node(node, src)
                })?,
            ))),
            "number" => Ok(Self::Number(::std::boxed::Box::new(
                ::treesitter_types::runtime::maybe_grow_stack(|| {
                    <Number as ::treesitter_types::FromNode>::from_node(node, src)
                })?,
            ))),
            "private_property_identifier" => Ok(Self::PrivatePropertyIdentifier(
                ::std::boxed::Box::new(::treesitter_types::runtime::maybe_grow_stack(|| {
                    <PrivatePropertyIdentifier as ::treesitter_types::FromNode>::from_node(
                        node, src,
                    )
                })?),
            )),
            "property_identifier" => Ok(Self::PropertyIdentifier(::std::boxed::Box::new(
                ::treesitter_types::runtime::maybe_grow_stack(|| {
                    <PropertyIdentifier as ::treesitter_types::FromNode>::from_node(node, src)
                })?,
            ))),
            "string" => Ok(Self::String(::std::boxed::Box::new(
                ::treesitter_types::runtime::maybe_grow_stack(|| {
                    <String as ::treesitter_types::FromNode>::from_node(node, src)
                })?,
            ))),
            other => Err(::treesitter_types::ParseError::unexpected_kind(other, node)),
        }
    }
}
impl ::treesitter_types::Spanned for PairKey<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        match self {
            Self::ComputedPropertyName(inner) => inner.span(),
            Self::Number(inner) => inner.span(),
            Self::PrivatePropertyIdentifier(inner) => inner.span(),
            Self::PropertyIdentifier(inner) => inner.span(),
            Self::String(inner) => inner.span(),
        }
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum PairPatternKey<'tree> {
    ComputedPropertyName(::std::boxed::Box<ComputedPropertyName<'tree>>),
    Number(::std::boxed::Box<Number<'tree>>),
    PrivatePropertyIdentifier(::std::boxed::Box<PrivatePropertyIdentifier<'tree>>),
    PropertyIdentifier(::std::boxed::Box<PropertyIdentifier<'tree>>),
    String(::std::boxed::Box<String<'tree>>),
}
impl<'tree> ::treesitter_types::FromNode<'tree> for PairPatternKey<'tree> {
    #[allow(clippy::collapsible_else_if)]
    fn from_node(
        node: ::treesitter_types::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        match node.kind() {
            "computed_property_name" => Ok(Self::ComputedPropertyName(::std::boxed::Box::new(
                ::treesitter_types::runtime::maybe_grow_stack(|| {
                    <ComputedPropertyName as ::treesitter_types::FromNode>::from_node(node, src)
                })?,
            ))),
            "number" => Ok(Self::Number(::std::boxed::Box::new(
                ::treesitter_types::runtime::maybe_grow_stack(|| {
                    <Number as ::treesitter_types::FromNode>::from_node(node, src)
                })?,
            ))),
            "private_property_identifier" => Ok(Self::PrivatePropertyIdentifier(
                ::std::boxed::Box::new(::treesitter_types::runtime::maybe_grow_stack(|| {
                    <PrivatePropertyIdentifier as ::treesitter_types::FromNode>::from_node(
                        node, src,
                    )
                })?),
            )),
            "property_identifier" => Ok(Self::PropertyIdentifier(::std::boxed::Box::new(
                ::treesitter_types::runtime::maybe_grow_stack(|| {
                    <PropertyIdentifier as ::treesitter_types::FromNode>::from_node(node, src)
                })?,
            ))),
            "string" => Ok(Self::String(::std::boxed::Box::new(
                ::treesitter_types::runtime::maybe_grow_stack(|| {
                    <String as ::treesitter_types::FromNode>::from_node(node, src)
                })?,
            ))),
            other => Err(::treesitter_types::ParseError::unexpected_kind(other, node)),
        }
    }
}
impl ::treesitter_types::Spanned for PairPatternKey<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        match self {
            Self::ComputedPropertyName(inner) => inner.span(),
            Self::Number(inner) => inner.span(),
            Self::PrivatePropertyIdentifier(inner) => inner.span(),
            Self::PropertyIdentifier(inner) => inner.span(),
            Self::String(inner) => inner.span(),
        }
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum PairPatternValue<'tree> {
    AssignmentPattern(::std::boxed::Box<AssignmentPattern<'tree>>),
    Pattern(::std::boxed::Box<Pattern<'tree>>),
}
impl<'tree> ::treesitter_types::FromNode<'tree> for PairPatternValue<'tree> {
    #[allow(clippy::collapsible_else_if)]
    fn from_node(
        node: ::treesitter_types::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        match node.kind() {
            "assignment_pattern" => Ok(Self::AssignmentPattern(::std::boxed::Box::new(
                ::treesitter_types::runtime::maybe_grow_stack(|| {
                    <AssignmentPattern as ::treesitter_types::FromNode>::from_node(node, src)
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
impl ::treesitter_types::Spanned for PairPatternValue<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        match self {
            Self::AssignmentPattern(inner) => inner.span(),
            Self::Pattern(inner) => inner.span(),
        }
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ParenthesizedExpressionChildren<'tree> {
    Expression(::std::boxed::Box<Expression<'tree>>),
    SequenceExpression(::std::boxed::Box<SequenceExpression<'tree>>),
}
impl<'tree> ::treesitter_types::FromNode<'tree> for ParenthesizedExpressionChildren<'tree> {
    #[allow(clippy::collapsible_else_if)]
    fn from_node(
        node: ::treesitter_types::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        match node.kind() {
            "sequence_expression" => Ok(Self::SequenceExpression(::std::boxed::Box::new(
                ::treesitter_types::runtime::maybe_grow_stack(|| {
                    <SequenceExpression as ::treesitter_types::FromNode>::from_node(node, src)
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
            Self::Expression(inner) => inner.span(),
            Self::SequenceExpression(inner) => inner.span(),
        }
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ProgramChildren<'tree> {
    HashBangLine(::std::boxed::Box<HashBangLine<'tree>>),
    Statement(::std::boxed::Box<Statement<'tree>>),
}
impl<'tree> ::treesitter_types::FromNode<'tree> for ProgramChildren<'tree> {
    #[allow(clippy::collapsible_else_if)]
    fn from_node(
        node: ::treesitter_types::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        match node.kind() {
            "hash_bang_line" => Ok(Self::HashBangLine(::std::boxed::Box::new(
                ::treesitter_types::runtime::maybe_grow_stack(|| {
                    <HashBangLine as ::treesitter_types::FromNode>::from_node(node, src)
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
impl ::treesitter_types::Spanned for ProgramChildren<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        match self {
            Self::HashBangLine(inner) => inner.span(),
            Self::Statement(inner) => inner.span(),
        }
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum RestPatternChildren<'tree> {
    ArrayPattern(::std::boxed::Box<ArrayPattern<'tree>>),
    Identifier(::std::boxed::Box<Identifier<'tree>>),
    MemberExpression(::std::boxed::Box<MemberExpression<'tree>>),
    ObjectPattern(::std::boxed::Box<ObjectPattern<'tree>>),
    SubscriptExpression(::std::boxed::Box<SubscriptExpression<'tree>>),
    Undefined(::std::boxed::Box<Undefined<'tree>>),
}
impl<'tree> ::treesitter_types::FromNode<'tree> for RestPatternChildren<'tree> {
    #[allow(clippy::collapsible_else_if)]
    fn from_node(
        node: ::treesitter_types::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        match node.kind() {
            "array_pattern" => Ok(Self::ArrayPattern(::std::boxed::Box::new(
                ::treesitter_types::runtime::maybe_grow_stack(|| {
                    <ArrayPattern as ::treesitter_types::FromNode>::from_node(node, src)
                })?,
            ))),
            "identifier" => Ok(Self::Identifier(::std::boxed::Box::new(
                ::treesitter_types::runtime::maybe_grow_stack(|| {
                    <Identifier as ::treesitter_types::FromNode>::from_node(node, src)
                })?,
            ))),
            "member_expression" => Ok(Self::MemberExpression(::std::boxed::Box::new(
                ::treesitter_types::runtime::maybe_grow_stack(|| {
                    <MemberExpression as ::treesitter_types::FromNode>::from_node(node, src)
                })?,
            ))),
            "object_pattern" => Ok(Self::ObjectPattern(::std::boxed::Box::new(
                ::treesitter_types::runtime::maybe_grow_stack(|| {
                    <ObjectPattern as ::treesitter_types::FromNode>::from_node(node, src)
                })?,
            ))),
            "subscript_expression" => Ok(Self::SubscriptExpression(::std::boxed::Box::new(
                ::treesitter_types::runtime::maybe_grow_stack(|| {
                    <SubscriptExpression as ::treesitter_types::FromNode>::from_node(node, src)
                })?,
            ))),
            "undefined" => Ok(Self::Undefined(::std::boxed::Box::new(
                ::treesitter_types::runtime::maybe_grow_stack(|| {
                    <Undefined as ::treesitter_types::FromNode>::from_node(node, src)
                })?,
            ))),
            other => Err(::treesitter_types::ParseError::unexpected_kind(other, node)),
        }
    }
}
impl ::treesitter_types::Spanned for RestPatternChildren<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        match self {
            Self::ArrayPattern(inner) => inner.span(),
            Self::Identifier(inner) => inner.span(),
            Self::MemberExpression(inner) => inner.span(),
            Self::ObjectPattern(inner) => inner.span(),
            Self::SubscriptExpression(inner) => inner.span(),
            Self::Undefined(inner) => inner.span(),
        }
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ReturnStatementChildren<'tree> {
    Expression(::std::boxed::Box<Expression<'tree>>),
    SequenceExpression(::std::boxed::Box<SequenceExpression<'tree>>),
}
impl<'tree> ::treesitter_types::FromNode<'tree> for ReturnStatementChildren<'tree> {
    #[allow(clippy::collapsible_else_if)]
    fn from_node(
        node: ::treesitter_types::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        match node.kind() {
            "sequence_expression" => Ok(Self::SequenceExpression(::std::boxed::Box::new(
                ::treesitter_types::runtime::maybe_grow_stack(|| {
                    <SequenceExpression as ::treesitter_types::FromNode>::from_node(node, src)
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
            Self::Expression(inner) => inner.span(),
            Self::SequenceExpression(inner) => inner.span(),
        }
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum StringChildren<'tree> {
    EscapeSequence(::std::boxed::Box<EscapeSequence<'tree>>),
    HtmlCharacterReference(::std::boxed::Box<HtmlCharacterReference<'tree>>),
    StringFragment(::std::boxed::Box<StringFragment<'tree>>),
}
impl<'tree> ::treesitter_types::FromNode<'tree> for StringChildren<'tree> {
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
            "html_character_reference" => Ok(Self::HtmlCharacterReference(::std::boxed::Box::new(
                ::treesitter_types::runtime::maybe_grow_stack(|| {
                    <HtmlCharacterReference as ::treesitter_types::FromNode>::from_node(node, src)
                })?,
            ))),
            "string_fragment" => Ok(Self::StringFragment(::std::boxed::Box::new(
                ::treesitter_types::runtime::maybe_grow_stack(|| {
                    <StringFragment as ::treesitter_types::FromNode>::from_node(node, src)
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
            Self::HtmlCharacterReference(inner) => inner.span(),
            Self::StringFragment(inner) => inner.span(),
        }
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum SubscriptExpressionIndex<'tree> {
    Expression(::std::boxed::Box<Expression<'tree>>),
    SequenceExpression(::std::boxed::Box<SequenceExpression<'tree>>),
}
impl<'tree> ::treesitter_types::FromNode<'tree> for SubscriptExpressionIndex<'tree> {
    #[allow(clippy::collapsible_else_if)]
    fn from_node(
        node: ::treesitter_types::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        match node.kind() {
            "sequence_expression" => Ok(Self::SequenceExpression(::std::boxed::Box::new(
                ::treesitter_types::runtime::maybe_grow_stack(|| {
                    <SequenceExpression as ::treesitter_types::FromNode>::from_node(node, src)
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
impl ::treesitter_types::Spanned for SubscriptExpressionIndex<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        match self {
            Self::Expression(inner) => inner.span(),
            Self::SequenceExpression(inner) => inner.span(),
        }
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum SwitchBodyChildren<'tree> {
    SwitchCase(::std::boxed::Box<SwitchCase<'tree>>),
    SwitchDefault(::std::boxed::Box<SwitchDefault<'tree>>),
}
impl<'tree> ::treesitter_types::FromNode<'tree> for SwitchBodyChildren<'tree> {
    #[allow(clippy::collapsible_else_if)]
    fn from_node(
        node: ::treesitter_types::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        match node.kind() {
            "switch_case" => Ok(Self::SwitchCase(::std::boxed::Box::new(
                ::treesitter_types::runtime::maybe_grow_stack(|| {
                    <SwitchCase as ::treesitter_types::FromNode>::from_node(node, src)
                })?,
            ))),
            "switch_default" => Ok(Self::SwitchDefault(::std::boxed::Box::new(
                ::treesitter_types::runtime::maybe_grow_stack(|| {
                    <SwitchDefault as ::treesitter_types::FromNode>::from_node(node, src)
                })?,
            ))),
            other => Err(::treesitter_types::ParseError::unexpected_kind(other, node)),
        }
    }
}
impl ::treesitter_types::Spanned for SwitchBodyChildren<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        match self {
            Self::SwitchCase(inner) => inner.span(),
            Self::SwitchDefault(inner) => inner.span(),
        }
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum SwitchCaseValue<'tree> {
    Expression(::std::boxed::Box<Expression<'tree>>),
    SequenceExpression(::std::boxed::Box<SequenceExpression<'tree>>),
}
impl<'tree> ::treesitter_types::FromNode<'tree> for SwitchCaseValue<'tree> {
    #[allow(clippy::collapsible_else_if)]
    fn from_node(
        node: ::treesitter_types::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        match node.kind() {
            "sequence_expression" => Ok(Self::SequenceExpression(::std::boxed::Box::new(
                ::treesitter_types::runtime::maybe_grow_stack(|| {
                    <SequenceExpression as ::treesitter_types::FromNode>::from_node(node, src)
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
impl ::treesitter_types::Spanned for SwitchCaseValue<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        match self {
            Self::Expression(inner) => inner.span(),
            Self::SequenceExpression(inner) => inner.span(),
        }
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum TemplateStringChildren<'tree> {
    EscapeSequence(::std::boxed::Box<EscapeSequence<'tree>>),
    StringFragment(::std::boxed::Box<StringFragment<'tree>>),
    TemplateSubstitution(::std::boxed::Box<TemplateSubstitution<'tree>>),
}
impl<'tree> ::treesitter_types::FromNode<'tree> for TemplateStringChildren<'tree> {
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
            "string_fragment" => Ok(Self::StringFragment(::std::boxed::Box::new(
                ::treesitter_types::runtime::maybe_grow_stack(|| {
                    <StringFragment as ::treesitter_types::FromNode>::from_node(node, src)
                })?,
            ))),
            "template_substitution" => Ok(Self::TemplateSubstitution(::std::boxed::Box::new(
                ::treesitter_types::runtime::maybe_grow_stack(|| {
                    <TemplateSubstitution as ::treesitter_types::FromNode>::from_node(node, src)
                })?,
            ))),
            other => Err(::treesitter_types::ParseError::unexpected_kind(other, node)),
        }
    }
}
impl ::treesitter_types::Spanned for TemplateStringChildren<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        match self {
            Self::EscapeSequence(inner) => inner.span(),
            Self::StringFragment(inner) => inner.span(),
            Self::TemplateSubstitution(inner) => inner.span(),
        }
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum TemplateSubstitutionChildren<'tree> {
    Expression(::std::boxed::Box<Expression<'tree>>),
    SequenceExpression(::std::boxed::Box<SequenceExpression<'tree>>),
}
impl<'tree> ::treesitter_types::FromNode<'tree> for TemplateSubstitutionChildren<'tree> {
    #[allow(clippy::collapsible_else_if)]
    fn from_node(
        node: ::treesitter_types::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        match node.kind() {
            "sequence_expression" => Ok(Self::SequenceExpression(::std::boxed::Box::new(
                ::treesitter_types::runtime::maybe_grow_stack(|| {
                    <SequenceExpression as ::treesitter_types::FromNode>::from_node(node, src)
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
impl ::treesitter_types::Spanned for TemplateSubstitutionChildren<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        match self {
            Self::Expression(inner) => inner.span(),
            Self::SequenceExpression(inner) => inner.span(),
        }
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ThrowStatementChildren<'tree> {
    Expression(::std::boxed::Box<Expression<'tree>>),
    SequenceExpression(::std::boxed::Box<SequenceExpression<'tree>>),
}
impl<'tree> ::treesitter_types::FromNode<'tree> for ThrowStatementChildren<'tree> {
    #[allow(clippy::collapsible_else_if)]
    fn from_node(
        node: ::treesitter_types::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        match node.kind() {
            "sequence_expression" => Ok(Self::SequenceExpression(::std::boxed::Box::new(
                ::treesitter_types::runtime::maybe_grow_stack(|| {
                    <SequenceExpression as ::treesitter_types::FromNode>::from_node(node, src)
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
impl ::treesitter_types::Spanned for ThrowStatementChildren<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        match self {
            Self::Expression(inner) => inner.span(),
            Self::SequenceExpression(inner) => inner.span(),
        }
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum UnaryExpressionOperator {
    Bang(::treesitter_types::Span),
    Plus(::treesitter_types::Span),
    Minus(::treesitter_types::Span),
    Delete(::treesitter_types::Span),
    Typeof(::treesitter_types::Span),
    Void(::treesitter_types::Span),
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
            "delete" => Ok(Self::Delete(::treesitter_types::Span::from(node))),
            "typeof" => Ok(Self::Typeof(::treesitter_types::Span::from(node))),
            "void" => Ok(Self::Void(::treesitter_types::Span::from(node))),
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
            Self::Delete(span) => *span,
            Self::Typeof(span) => *span,
            Self::Void(span) => *span,
            Self::Tilde(span) => *span,
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
pub enum UsingDeclarationKind {
    Await(::treesitter_types::Span),
    Using(::treesitter_types::Span),
}
impl<'tree> ::treesitter_types::FromNode<'tree> for UsingDeclarationKind {
    #[allow(clippy::collapsible_else_if)]
    fn from_node(
        node: ::treesitter_types::tree_sitter::Node<'tree>,
        _src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        match node.kind() {
            "await" => Ok(Self::Await(::treesitter_types::Span::from(node))),
            "using" => Ok(Self::Using(::treesitter_types::Span::from(node))),
            other => Err(::treesitter_types::ParseError::unexpected_kind(other, node)),
        }
    }
}
impl ::treesitter_types::Spanned for UsingDeclarationKind {
    fn span(&self) -> ::treesitter_types::Span {
        match self {
            Self::Await(span) => *span,
            Self::Using(span) => *span,
        }
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum VariableDeclaratorName<'tree> {
    ArrayPattern(::std::boxed::Box<ArrayPattern<'tree>>),
    Identifier(::std::boxed::Box<Identifier<'tree>>),
    ObjectPattern(::std::boxed::Box<ObjectPattern<'tree>>),
}
impl<'tree> ::treesitter_types::FromNode<'tree> for VariableDeclaratorName<'tree> {
    #[allow(clippy::collapsible_else_if)]
    fn from_node(
        node: ::treesitter_types::tree_sitter::Node<'tree>,
        src: &'tree [u8],
    ) -> ::core::result::Result<Self, ::treesitter_types::ParseError> {
        match node.kind() {
            "array_pattern" => Ok(Self::ArrayPattern(::std::boxed::Box::new(
                ::treesitter_types::runtime::maybe_grow_stack(|| {
                    <ArrayPattern as ::treesitter_types::FromNode>::from_node(node, src)
                })?,
            ))),
            "identifier" => Ok(Self::Identifier(::std::boxed::Box::new(
                ::treesitter_types::runtime::maybe_grow_stack(|| {
                    <Identifier as ::treesitter_types::FromNode>::from_node(node, src)
                })?,
            ))),
            "object_pattern" => Ok(Self::ObjectPattern(::std::boxed::Box::new(
                ::treesitter_types::runtime::maybe_grow_stack(|| {
                    <ObjectPattern as ::treesitter_types::FromNode>::from_node(node, src)
                })?,
            ))),
            other => Err(::treesitter_types::ParseError::unexpected_kind(other, node)),
        }
    }
}
impl ::treesitter_types::Spanned for VariableDeclaratorName<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        match self {
            Self::ArrayPattern(inner) => inner.span(),
            Self::Identifier(inner) => inner.span(),
            Self::ObjectPattern(inner) => inner.span(),
        }
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum AnyNode<'tree> {
    Declaration(Declaration<'tree>),
    Expression(Expression<'tree>),
    Pattern(Pattern<'tree>),
    PrimaryExpression(PrimaryExpression<'tree>),
    Statement(Statement<'tree>),
    Arguments(Arguments<'tree>),
    Array(Array<'tree>),
    ArrayPattern(ArrayPattern<'tree>),
    ArrowFunction(ArrowFunction<'tree>),
    AssignmentExpression(AssignmentExpression<'tree>),
    AssignmentPattern(AssignmentPattern<'tree>),
    AugmentedAssignmentExpression(AugmentedAssignmentExpression<'tree>),
    AwaitExpression(AwaitExpression<'tree>),
    BinaryExpression(BinaryExpression<'tree>),
    BreakStatement(BreakStatement<'tree>),
    CallExpression(CallExpression<'tree>),
    CatchClause(CatchClause<'tree>),
    Class(Class<'tree>),
    ClassBody(ClassBody<'tree>),
    ClassDeclaration(ClassDeclaration<'tree>),
    ClassHeritage(ClassHeritage<'tree>),
    ClassStaticBlock(ClassStaticBlock<'tree>),
    ComputedPropertyName(ComputedPropertyName<'tree>),
    ContinueStatement(ContinueStatement<'tree>),
    DebuggerStatement(DebuggerStatement<'tree>),
    Decorator(Decorator<'tree>),
    DoStatement(DoStatement<'tree>),
    ElseClause(ElseClause<'tree>),
    EmptyStatement(EmptyStatement<'tree>),
    ExportClause(ExportClause<'tree>),
    ExportSpecifier(ExportSpecifier<'tree>),
    ExportStatement(ExportStatement<'tree>),
    ExpressionStatement(ExpressionStatement<'tree>),
    FieldDefinition(FieldDefinition<'tree>),
    FinallyClause(FinallyClause<'tree>),
    ForInStatement(ForInStatement<'tree>),
    ForStatement(ForStatement<'tree>),
    FormalParameters(FormalParameters<'tree>),
    FunctionDeclaration(FunctionDeclaration<'tree>),
    FunctionExpression(FunctionExpression<'tree>),
    GeneratorFunction(GeneratorFunction<'tree>),
    GeneratorFunctionDeclaration(GeneratorFunctionDeclaration<'tree>),
    IfStatement(IfStatement<'tree>),
    Import(Import<'tree>),
    ImportAttribute(ImportAttribute<'tree>),
    ImportClause(ImportClause<'tree>),
    ImportSpecifier(ImportSpecifier<'tree>),
    ImportStatement(ImportStatement<'tree>),
    JsxAttribute(JsxAttribute<'tree>),
    JsxClosingElement(JsxClosingElement<'tree>),
    JsxElement(JsxElement<'tree>),
    JsxExpression(JsxExpression<'tree>),
    JsxNamespaceName(JsxNamespaceName<'tree>),
    JsxOpeningElement(JsxOpeningElement<'tree>),
    JsxSelfClosingElement(JsxSelfClosingElement<'tree>),
    LabeledStatement(LabeledStatement<'tree>),
    LexicalDeclaration(LexicalDeclaration<'tree>),
    MemberExpression(MemberExpression<'tree>),
    MetaProperty(MetaProperty<'tree>),
    MethodDefinition(MethodDefinition<'tree>),
    NamedImports(NamedImports<'tree>),
    NamespaceExport(NamespaceExport<'tree>),
    NamespaceImport(NamespaceImport<'tree>),
    NewExpression(NewExpression<'tree>),
    Object(Object<'tree>),
    ObjectAssignmentPattern(ObjectAssignmentPattern<'tree>),
    ObjectPattern(ObjectPattern<'tree>),
    Pair(Pair<'tree>),
    PairPattern(PairPattern<'tree>),
    ParenthesizedExpression(ParenthesizedExpression<'tree>),
    Program(Program<'tree>),
    Regex(Regex<'tree>),
    RestPattern(RestPattern<'tree>),
    ReturnStatement(ReturnStatement<'tree>),
    SequenceExpression(SequenceExpression<'tree>),
    SpreadElement(SpreadElement<'tree>),
    StatementBlock(StatementBlock<'tree>),
    String(String<'tree>),
    SubscriptExpression(SubscriptExpression<'tree>),
    SwitchBody(SwitchBody<'tree>),
    SwitchCase(SwitchCase<'tree>),
    SwitchDefault(SwitchDefault<'tree>),
    SwitchStatement(SwitchStatement<'tree>),
    TemplateString(TemplateString<'tree>),
    TemplateSubstitution(TemplateSubstitution<'tree>),
    TernaryExpression(TernaryExpression<'tree>),
    ThrowStatement(ThrowStatement<'tree>),
    TryStatement(TryStatement<'tree>),
    UnaryExpression(UnaryExpression<'tree>),
    UpdateExpression(UpdateExpression<'tree>),
    UsingDeclaration(UsingDeclaration<'tree>),
    VariableDeclaration(VariableDeclaration<'tree>),
    VariableDeclarator(VariableDeclarator<'tree>),
    WhileStatement(WhileStatement<'tree>),
    WithStatement(WithStatement<'tree>),
    YieldExpression(YieldExpression<'tree>),
    Comment(Comment<'tree>),
    EscapeSequence(EscapeSequence<'tree>),
    False(False<'tree>),
    HashBangLine(HashBangLine<'tree>),
    HtmlCharacterReference(HtmlCharacterReference<'tree>),
    HtmlComment(HtmlComment<'tree>),
    Identifier(Identifier<'tree>),
    JsxText(JsxText<'tree>),
    Null(Null<'tree>),
    Number(Number<'tree>),
    OptionalChain(OptionalChain<'tree>),
    PrivatePropertyIdentifier(PrivatePropertyIdentifier<'tree>),
    PropertyIdentifier(PropertyIdentifier<'tree>),
    RegexFlags(RegexFlags<'tree>),
    RegexPattern(RegexPattern<'tree>),
    ShorthandPropertyIdentifier(ShorthandPropertyIdentifier<'tree>),
    ShorthandPropertyIdentifierPattern(ShorthandPropertyIdentifierPattern<'tree>),
    StatementIdentifier(StatementIdentifier<'tree>),
    StringFragment(StringFragment<'tree>),
    Super(Super<'tree>),
    This(This<'tree>),
    True(True<'tree>),
    Undefined(Undefined<'tree>),
    Unknown(::treesitter_types::tree_sitter::Node<'tree>),
}
impl<'tree> AnyNode<'tree> {
    pub fn from_node(node: ::treesitter_types::tree_sitter::Node<'tree>, src: &'tree [u8]) -> Self {
        match node.kind() {
            "declaration" => ::treesitter_types::runtime::maybe_grow_stack(|| {
                <Declaration as ::treesitter_types::FromNode>::from_node(node, src)
            })
            .map(Self::Declaration)
            .unwrap_or(Self::Unknown(node)),
            "expression" => ::treesitter_types::runtime::maybe_grow_stack(|| {
                <Expression as ::treesitter_types::FromNode>::from_node(node, src)
            })
            .map(Self::Expression)
            .unwrap_or(Self::Unknown(node)),
            "pattern" => ::treesitter_types::runtime::maybe_grow_stack(|| {
                <Pattern as ::treesitter_types::FromNode>::from_node(node, src)
            })
            .map(Self::Pattern)
            .unwrap_or(Self::Unknown(node)),
            "primary_expression" => ::treesitter_types::runtime::maybe_grow_stack(|| {
                <PrimaryExpression as ::treesitter_types::FromNode>::from_node(node, src)
            })
            .map(Self::PrimaryExpression)
            .unwrap_or(Self::Unknown(node)),
            "statement" => ::treesitter_types::runtime::maybe_grow_stack(|| {
                <Statement as ::treesitter_types::FromNode>::from_node(node, src)
            })
            .map(Self::Statement)
            .unwrap_or(Self::Unknown(node)),
            "arguments" => ::treesitter_types::runtime::maybe_grow_stack(|| {
                <Arguments as ::treesitter_types::FromNode>::from_node(node, src)
            })
            .map(Self::Arguments)
            .unwrap_or(Self::Unknown(node)),
            "array" => ::treesitter_types::runtime::maybe_grow_stack(|| {
                <Array as ::treesitter_types::FromNode>::from_node(node, src)
            })
            .map(Self::Array)
            .unwrap_or(Self::Unknown(node)),
            "array_pattern" => ::treesitter_types::runtime::maybe_grow_stack(|| {
                <ArrayPattern as ::treesitter_types::FromNode>::from_node(node, src)
            })
            .map(Self::ArrayPattern)
            .unwrap_or(Self::Unknown(node)),
            "arrow_function" => ::treesitter_types::runtime::maybe_grow_stack(|| {
                <ArrowFunction as ::treesitter_types::FromNode>::from_node(node, src)
            })
            .map(Self::ArrowFunction)
            .unwrap_or(Self::Unknown(node)),
            "assignment_expression" => ::treesitter_types::runtime::maybe_grow_stack(|| {
                <AssignmentExpression as ::treesitter_types::FromNode>::from_node(node, src)
            })
            .map(Self::AssignmentExpression)
            .unwrap_or(Self::Unknown(node)),
            "assignment_pattern" => ::treesitter_types::runtime::maybe_grow_stack(|| {
                <AssignmentPattern as ::treesitter_types::FromNode>::from_node(node, src)
            })
            .map(Self::AssignmentPattern)
            .unwrap_or(Self::Unknown(node)),
            "augmented_assignment_expression" => {
                ::treesitter_types::runtime::maybe_grow_stack(|| {
                    <AugmentedAssignmentExpression as ::treesitter_types::FromNode>::from_node(
                        node, src,
                    )
                })
                .map(Self::AugmentedAssignmentExpression)
                .unwrap_or(Self::Unknown(node))
            }
            "await_expression" => ::treesitter_types::runtime::maybe_grow_stack(|| {
                <AwaitExpression as ::treesitter_types::FromNode>::from_node(node, src)
            })
            .map(Self::AwaitExpression)
            .unwrap_or(Self::Unknown(node)),
            "binary_expression" => ::treesitter_types::runtime::maybe_grow_stack(|| {
                <BinaryExpression as ::treesitter_types::FromNode>::from_node(node, src)
            })
            .map(Self::BinaryExpression)
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
            "catch_clause" => ::treesitter_types::runtime::maybe_grow_stack(|| {
                <CatchClause as ::treesitter_types::FromNode>::from_node(node, src)
            })
            .map(Self::CatchClause)
            .unwrap_or(Self::Unknown(node)),
            "class" => ::treesitter_types::runtime::maybe_grow_stack(|| {
                <Class as ::treesitter_types::FromNode>::from_node(node, src)
            })
            .map(Self::Class)
            .unwrap_or(Self::Unknown(node)),
            "class_body" => ::treesitter_types::runtime::maybe_grow_stack(|| {
                <ClassBody as ::treesitter_types::FromNode>::from_node(node, src)
            })
            .map(Self::ClassBody)
            .unwrap_or(Self::Unknown(node)),
            "class_declaration" => ::treesitter_types::runtime::maybe_grow_stack(|| {
                <ClassDeclaration as ::treesitter_types::FromNode>::from_node(node, src)
            })
            .map(Self::ClassDeclaration)
            .unwrap_or(Self::Unknown(node)),
            "class_heritage" => ::treesitter_types::runtime::maybe_grow_stack(|| {
                <ClassHeritage as ::treesitter_types::FromNode>::from_node(node, src)
            })
            .map(Self::ClassHeritage)
            .unwrap_or(Self::Unknown(node)),
            "class_static_block" => ::treesitter_types::runtime::maybe_grow_stack(|| {
                <ClassStaticBlock as ::treesitter_types::FromNode>::from_node(node, src)
            })
            .map(Self::ClassStaticBlock)
            .unwrap_or(Self::Unknown(node)),
            "computed_property_name" => ::treesitter_types::runtime::maybe_grow_stack(|| {
                <ComputedPropertyName as ::treesitter_types::FromNode>::from_node(node, src)
            })
            .map(Self::ComputedPropertyName)
            .unwrap_or(Self::Unknown(node)),
            "continue_statement" => ::treesitter_types::runtime::maybe_grow_stack(|| {
                <ContinueStatement as ::treesitter_types::FromNode>::from_node(node, src)
            })
            .map(Self::ContinueStatement)
            .unwrap_or(Self::Unknown(node)),
            "debugger_statement" => ::treesitter_types::runtime::maybe_grow_stack(|| {
                <DebuggerStatement as ::treesitter_types::FromNode>::from_node(node, src)
            })
            .map(Self::DebuggerStatement)
            .unwrap_or(Self::Unknown(node)),
            "decorator" => ::treesitter_types::runtime::maybe_grow_stack(|| {
                <Decorator as ::treesitter_types::FromNode>::from_node(node, src)
            })
            .map(Self::Decorator)
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
            "empty_statement" => ::treesitter_types::runtime::maybe_grow_stack(|| {
                <EmptyStatement as ::treesitter_types::FromNode>::from_node(node, src)
            })
            .map(Self::EmptyStatement)
            .unwrap_or(Self::Unknown(node)),
            "export_clause" => ::treesitter_types::runtime::maybe_grow_stack(|| {
                <ExportClause as ::treesitter_types::FromNode>::from_node(node, src)
            })
            .map(Self::ExportClause)
            .unwrap_or(Self::Unknown(node)),
            "export_specifier" => ::treesitter_types::runtime::maybe_grow_stack(|| {
                <ExportSpecifier as ::treesitter_types::FromNode>::from_node(node, src)
            })
            .map(Self::ExportSpecifier)
            .unwrap_or(Self::Unknown(node)),
            "export_statement" => ::treesitter_types::runtime::maybe_grow_stack(|| {
                <ExportStatement as ::treesitter_types::FromNode>::from_node(node, src)
            })
            .map(Self::ExportStatement)
            .unwrap_or(Self::Unknown(node)),
            "expression_statement" => ::treesitter_types::runtime::maybe_grow_stack(|| {
                <ExpressionStatement as ::treesitter_types::FromNode>::from_node(node, src)
            })
            .map(Self::ExpressionStatement)
            .unwrap_or(Self::Unknown(node)),
            "field_definition" => ::treesitter_types::runtime::maybe_grow_stack(|| {
                <FieldDefinition as ::treesitter_types::FromNode>::from_node(node, src)
            })
            .map(Self::FieldDefinition)
            .unwrap_or(Self::Unknown(node)),
            "finally_clause" => ::treesitter_types::runtime::maybe_grow_stack(|| {
                <FinallyClause as ::treesitter_types::FromNode>::from_node(node, src)
            })
            .map(Self::FinallyClause)
            .unwrap_or(Self::Unknown(node)),
            "for_in_statement" => ::treesitter_types::runtime::maybe_grow_stack(|| {
                <ForInStatement as ::treesitter_types::FromNode>::from_node(node, src)
            })
            .map(Self::ForInStatement)
            .unwrap_or(Self::Unknown(node)),
            "for_statement" => ::treesitter_types::runtime::maybe_grow_stack(|| {
                <ForStatement as ::treesitter_types::FromNode>::from_node(node, src)
            })
            .map(Self::ForStatement)
            .unwrap_or(Self::Unknown(node)),
            "formal_parameters" => ::treesitter_types::runtime::maybe_grow_stack(|| {
                <FormalParameters as ::treesitter_types::FromNode>::from_node(node, src)
            })
            .map(Self::FormalParameters)
            .unwrap_or(Self::Unknown(node)),
            "function_declaration" => ::treesitter_types::runtime::maybe_grow_stack(|| {
                <FunctionDeclaration as ::treesitter_types::FromNode>::from_node(node, src)
            })
            .map(Self::FunctionDeclaration)
            .unwrap_or(Self::Unknown(node)),
            "function_expression" => ::treesitter_types::runtime::maybe_grow_stack(|| {
                <FunctionExpression as ::treesitter_types::FromNode>::from_node(node, src)
            })
            .map(Self::FunctionExpression)
            .unwrap_or(Self::Unknown(node)),
            "generator_function" => ::treesitter_types::runtime::maybe_grow_stack(|| {
                <GeneratorFunction as ::treesitter_types::FromNode>::from_node(node, src)
            })
            .map(Self::GeneratorFunction)
            .unwrap_or(Self::Unknown(node)),
            "generator_function_declaration" => {
                ::treesitter_types::runtime::maybe_grow_stack(|| {
                    <GeneratorFunctionDeclaration as ::treesitter_types::FromNode>::from_node(
                        node, src,
                    )
                })
                .map(Self::GeneratorFunctionDeclaration)
                .unwrap_or(Self::Unknown(node))
            }
            "if_statement" => ::treesitter_types::runtime::maybe_grow_stack(|| {
                <IfStatement as ::treesitter_types::FromNode>::from_node(node, src)
            })
            .map(Self::IfStatement)
            .unwrap_or(Self::Unknown(node)),
            "import" => ::treesitter_types::runtime::maybe_grow_stack(|| {
                <Import as ::treesitter_types::FromNode>::from_node(node, src)
            })
            .map(Self::Import)
            .unwrap_or(Self::Unknown(node)),
            "import_attribute" => ::treesitter_types::runtime::maybe_grow_stack(|| {
                <ImportAttribute as ::treesitter_types::FromNode>::from_node(node, src)
            })
            .map(Self::ImportAttribute)
            .unwrap_or(Self::Unknown(node)),
            "import_clause" => ::treesitter_types::runtime::maybe_grow_stack(|| {
                <ImportClause as ::treesitter_types::FromNode>::from_node(node, src)
            })
            .map(Self::ImportClause)
            .unwrap_or(Self::Unknown(node)),
            "import_specifier" => ::treesitter_types::runtime::maybe_grow_stack(|| {
                <ImportSpecifier as ::treesitter_types::FromNode>::from_node(node, src)
            })
            .map(Self::ImportSpecifier)
            .unwrap_or(Self::Unknown(node)),
            "import_statement" => ::treesitter_types::runtime::maybe_grow_stack(|| {
                <ImportStatement as ::treesitter_types::FromNode>::from_node(node, src)
            })
            .map(Self::ImportStatement)
            .unwrap_or(Self::Unknown(node)),
            "jsx_attribute" => ::treesitter_types::runtime::maybe_grow_stack(|| {
                <JsxAttribute as ::treesitter_types::FromNode>::from_node(node, src)
            })
            .map(Self::JsxAttribute)
            .unwrap_or(Self::Unknown(node)),
            "jsx_closing_element" => ::treesitter_types::runtime::maybe_grow_stack(|| {
                <JsxClosingElement as ::treesitter_types::FromNode>::from_node(node, src)
            })
            .map(Self::JsxClosingElement)
            .unwrap_or(Self::Unknown(node)),
            "jsx_element" => ::treesitter_types::runtime::maybe_grow_stack(|| {
                <JsxElement as ::treesitter_types::FromNode>::from_node(node, src)
            })
            .map(Self::JsxElement)
            .unwrap_or(Self::Unknown(node)),
            "jsx_expression" => ::treesitter_types::runtime::maybe_grow_stack(|| {
                <JsxExpression as ::treesitter_types::FromNode>::from_node(node, src)
            })
            .map(Self::JsxExpression)
            .unwrap_or(Self::Unknown(node)),
            "jsx_namespace_name" => ::treesitter_types::runtime::maybe_grow_stack(|| {
                <JsxNamespaceName as ::treesitter_types::FromNode>::from_node(node, src)
            })
            .map(Self::JsxNamespaceName)
            .unwrap_or(Self::Unknown(node)),
            "jsx_opening_element" => ::treesitter_types::runtime::maybe_grow_stack(|| {
                <JsxOpeningElement as ::treesitter_types::FromNode>::from_node(node, src)
            })
            .map(Self::JsxOpeningElement)
            .unwrap_or(Self::Unknown(node)),
            "jsx_self_closing_element" => ::treesitter_types::runtime::maybe_grow_stack(|| {
                <JsxSelfClosingElement as ::treesitter_types::FromNode>::from_node(node, src)
            })
            .map(Self::JsxSelfClosingElement)
            .unwrap_or(Self::Unknown(node)),
            "labeled_statement" => ::treesitter_types::runtime::maybe_grow_stack(|| {
                <LabeledStatement as ::treesitter_types::FromNode>::from_node(node, src)
            })
            .map(Self::LabeledStatement)
            .unwrap_or(Self::Unknown(node)),
            "lexical_declaration" => ::treesitter_types::runtime::maybe_grow_stack(|| {
                <LexicalDeclaration as ::treesitter_types::FromNode>::from_node(node, src)
            })
            .map(Self::LexicalDeclaration)
            .unwrap_or(Self::Unknown(node)),
            "member_expression" => ::treesitter_types::runtime::maybe_grow_stack(|| {
                <MemberExpression as ::treesitter_types::FromNode>::from_node(node, src)
            })
            .map(Self::MemberExpression)
            .unwrap_or(Self::Unknown(node)),
            "meta_property" => ::treesitter_types::runtime::maybe_grow_stack(|| {
                <MetaProperty as ::treesitter_types::FromNode>::from_node(node, src)
            })
            .map(Self::MetaProperty)
            .unwrap_or(Self::Unknown(node)),
            "method_definition" => ::treesitter_types::runtime::maybe_grow_stack(|| {
                <MethodDefinition as ::treesitter_types::FromNode>::from_node(node, src)
            })
            .map(Self::MethodDefinition)
            .unwrap_or(Self::Unknown(node)),
            "named_imports" => ::treesitter_types::runtime::maybe_grow_stack(|| {
                <NamedImports as ::treesitter_types::FromNode>::from_node(node, src)
            })
            .map(Self::NamedImports)
            .unwrap_or(Self::Unknown(node)),
            "namespace_export" => ::treesitter_types::runtime::maybe_grow_stack(|| {
                <NamespaceExport as ::treesitter_types::FromNode>::from_node(node, src)
            })
            .map(Self::NamespaceExport)
            .unwrap_or(Self::Unknown(node)),
            "namespace_import" => ::treesitter_types::runtime::maybe_grow_stack(|| {
                <NamespaceImport as ::treesitter_types::FromNode>::from_node(node, src)
            })
            .map(Self::NamespaceImport)
            .unwrap_or(Self::Unknown(node)),
            "new_expression" => ::treesitter_types::runtime::maybe_grow_stack(|| {
                <NewExpression as ::treesitter_types::FromNode>::from_node(node, src)
            })
            .map(Self::NewExpression)
            .unwrap_or(Self::Unknown(node)),
            "object" => ::treesitter_types::runtime::maybe_grow_stack(|| {
                <Object as ::treesitter_types::FromNode>::from_node(node, src)
            })
            .map(Self::Object)
            .unwrap_or(Self::Unknown(node)),
            "object_assignment_pattern" => ::treesitter_types::runtime::maybe_grow_stack(|| {
                <ObjectAssignmentPattern as ::treesitter_types::FromNode>::from_node(node, src)
            })
            .map(Self::ObjectAssignmentPattern)
            .unwrap_or(Self::Unknown(node)),
            "object_pattern" => ::treesitter_types::runtime::maybe_grow_stack(|| {
                <ObjectPattern as ::treesitter_types::FromNode>::from_node(node, src)
            })
            .map(Self::ObjectPattern)
            .unwrap_or(Self::Unknown(node)),
            "pair" => ::treesitter_types::runtime::maybe_grow_stack(|| {
                <Pair as ::treesitter_types::FromNode>::from_node(node, src)
            })
            .map(Self::Pair)
            .unwrap_or(Self::Unknown(node)),
            "pair_pattern" => ::treesitter_types::runtime::maybe_grow_stack(|| {
                <PairPattern as ::treesitter_types::FromNode>::from_node(node, src)
            })
            .map(Self::PairPattern)
            .unwrap_or(Self::Unknown(node)),
            "parenthesized_expression" => ::treesitter_types::runtime::maybe_grow_stack(|| {
                <ParenthesizedExpression as ::treesitter_types::FromNode>::from_node(node, src)
            })
            .map(Self::ParenthesizedExpression)
            .unwrap_or(Self::Unknown(node)),
            "program" => ::treesitter_types::runtime::maybe_grow_stack(|| {
                <Program as ::treesitter_types::FromNode>::from_node(node, src)
            })
            .map(Self::Program)
            .unwrap_or(Self::Unknown(node)),
            "regex" => ::treesitter_types::runtime::maybe_grow_stack(|| {
                <Regex as ::treesitter_types::FromNode>::from_node(node, src)
            })
            .map(Self::Regex)
            .unwrap_or(Self::Unknown(node)),
            "rest_pattern" => ::treesitter_types::runtime::maybe_grow_stack(|| {
                <RestPattern as ::treesitter_types::FromNode>::from_node(node, src)
            })
            .map(Self::RestPattern)
            .unwrap_or(Self::Unknown(node)),
            "return_statement" => ::treesitter_types::runtime::maybe_grow_stack(|| {
                <ReturnStatement as ::treesitter_types::FromNode>::from_node(node, src)
            })
            .map(Self::ReturnStatement)
            .unwrap_or(Self::Unknown(node)),
            "sequence_expression" => ::treesitter_types::runtime::maybe_grow_stack(|| {
                <SequenceExpression as ::treesitter_types::FromNode>::from_node(node, src)
            })
            .map(Self::SequenceExpression)
            .unwrap_or(Self::Unknown(node)),
            "spread_element" => ::treesitter_types::runtime::maybe_grow_stack(|| {
                <SpreadElement as ::treesitter_types::FromNode>::from_node(node, src)
            })
            .map(Self::SpreadElement)
            .unwrap_or(Self::Unknown(node)),
            "statement_block" => ::treesitter_types::runtime::maybe_grow_stack(|| {
                <StatementBlock as ::treesitter_types::FromNode>::from_node(node, src)
            })
            .map(Self::StatementBlock)
            .unwrap_or(Self::Unknown(node)),
            "string" => ::treesitter_types::runtime::maybe_grow_stack(|| {
                <String as ::treesitter_types::FromNode>::from_node(node, src)
            })
            .map(Self::String)
            .unwrap_or(Self::Unknown(node)),
            "subscript_expression" => ::treesitter_types::runtime::maybe_grow_stack(|| {
                <SubscriptExpression as ::treesitter_types::FromNode>::from_node(node, src)
            })
            .map(Self::SubscriptExpression)
            .unwrap_or(Self::Unknown(node)),
            "switch_body" => ::treesitter_types::runtime::maybe_grow_stack(|| {
                <SwitchBody as ::treesitter_types::FromNode>::from_node(node, src)
            })
            .map(Self::SwitchBody)
            .unwrap_or(Self::Unknown(node)),
            "switch_case" => ::treesitter_types::runtime::maybe_grow_stack(|| {
                <SwitchCase as ::treesitter_types::FromNode>::from_node(node, src)
            })
            .map(Self::SwitchCase)
            .unwrap_or(Self::Unknown(node)),
            "switch_default" => ::treesitter_types::runtime::maybe_grow_stack(|| {
                <SwitchDefault as ::treesitter_types::FromNode>::from_node(node, src)
            })
            .map(Self::SwitchDefault)
            .unwrap_or(Self::Unknown(node)),
            "switch_statement" => ::treesitter_types::runtime::maybe_grow_stack(|| {
                <SwitchStatement as ::treesitter_types::FromNode>::from_node(node, src)
            })
            .map(Self::SwitchStatement)
            .unwrap_or(Self::Unknown(node)),
            "template_string" => ::treesitter_types::runtime::maybe_grow_stack(|| {
                <TemplateString as ::treesitter_types::FromNode>::from_node(node, src)
            })
            .map(Self::TemplateString)
            .unwrap_or(Self::Unknown(node)),
            "template_substitution" => ::treesitter_types::runtime::maybe_grow_stack(|| {
                <TemplateSubstitution as ::treesitter_types::FromNode>::from_node(node, src)
            })
            .map(Self::TemplateSubstitution)
            .unwrap_or(Self::Unknown(node)),
            "ternary_expression" => ::treesitter_types::runtime::maybe_grow_stack(|| {
                <TernaryExpression as ::treesitter_types::FromNode>::from_node(node, src)
            })
            .map(Self::TernaryExpression)
            .unwrap_or(Self::Unknown(node)),
            "throw_statement" => ::treesitter_types::runtime::maybe_grow_stack(|| {
                <ThrowStatement as ::treesitter_types::FromNode>::from_node(node, src)
            })
            .map(Self::ThrowStatement)
            .unwrap_or(Self::Unknown(node)),
            "try_statement" => ::treesitter_types::runtime::maybe_grow_stack(|| {
                <TryStatement as ::treesitter_types::FromNode>::from_node(node, src)
            })
            .map(Self::TryStatement)
            .unwrap_or(Self::Unknown(node)),
            "unary_expression" => ::treesitter_types::runtime::maybe_grow_stack(|| {
                <UnaryExpression as ::treesitter_types::FromNode>::from_node(node, src)
            })
            .map(Self::UnaryExpression)
            .unwrap_or(Self::Unknown(node)),
            "update_expression" => ::treesitter_types::runtime::maybe_grow_stack(|| {
                <UpdateExpression as ::treesitter_types::FromNode>::from_node(node, src)
            })
            .map(Self::UpdateExpression)
            .unwrap_or(Self::Unknown(node)),
            "using_declaration" => ::treesitter_types::runtime::maybe_grow_stack(|| {
                <UsingDeclaration as ::treesitter_types::FromNode>::from_node(node, src)
            })
            .map(Self::UsingDeclaration)
            .unwrap_or(Self::Unknown(node)),
            "variable_declaration" => ::treesitter_types::runtime::maybe_grow_stack(|| {
                <VariableDeclaration as ::treesitter_types::FromNode>::from_node(node, src)
            })
            .map(Self::VariableDeclaration)
            .unwrap_or(Self::Unknown(node)),
            "variable_declarator" => ::treesitter_types::runtime::maybe_grow_stack(|| {
                <VariableDeclarator as ::treesitter_types::FromNode>::from_node(node, src)
            })
            .map(Self::VariableDeclarator)
            .unwrap_or(Self::Unknown(node)),
            "while_statement" => ::treesitter_types::runtime::maybe_grow_stack(|| {
                <WhileStatement as ::treesitter_types::FromNode>::from_node(node, src)
            })
            .map(Self::WhileStatement)
            .unwrap_or(Self::Unknown(node)),
            "with_statement" => ::treesitter_types::runtime::maybe_grow_stack(|| {
                <WithStatement as ::treesitter_types::FromNode>::from_node(node, src)
            })
            .map(Self::WithStatement)
            .unwrap_or(Self::Unknown(node)),
            "yield_expression" => ::treesitter_types::runtime::maybe_grow_stack(|| {
                <YieldExpression as ::treesitter_types::FromNode>::from_node(node, src)
            })
            .map(Self::YieldExpression)
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
            "hash_bang_line" => ::treesitter_types::runtime::maybe_grow_stack(|| {
                <HashBangLine as ::treesitter_types::FromNode>::from_node(node, src)
            })
            .map(Self::HashBangLine)
            .unwrap_or(Self::Unknown(node)),
            "html_character_reference" => ::treesitter_types::runtime::maybe_grow_stack(|| {
                <HtmlCharacterReference as ::treesitter_types::FromNode>::from_node(node, src)
            })
            .map(Self::HtmlCharacterReference)
            .unwrap_or(Self::Unknown(node)),
            "html_comment" => ::treesitter_types::runtime::maybe_grow_stack(|| {
                <HtmlComment as ::treesitter_types::FromNode>::from_node(node, src)
            })
            .map(Self::HtmlComment)
            .unwrap_or(Self::Unknown(node)),
            "identifier" => ::treesitter_types::runtime::maybe_grow_stack(|| {
                <Identifier as ::treesitter_types::FromNode>::from_node(node, src)
            })
            .map(Self::Identifier)
            .unwrap_or(Self::Unknown(node)),
            "jsx_text" => ::treesitter_types::runtime::maybe_grow_stack(|| {
                <JsxText as ::treesitter_types::FromNode>::from_node(node, src)
            })
            .map(Self::JsxText)
            .unwrap_or(Self::Unknown(node)),
            "null" => ::treesitter_types::runtime::maybe_grow_stack(|| {
                <Null as ::treesitter_types::FromNode>::from_node(node, src)
            })
            .map(Self::Null)
            .unwrap_or(Self::Unknown(node)),
            "number" => ::treesitter_types::runtime::maybe_grow_stack(|| {
                <Number as ::treesitter_types::FromNode>::from_node(node, src)
            })
            .map(Self::Number)
            .unwrap_or(Self::Unknown(node)),
            "optional_chain" => ::treesitter_types::runtime::maybe_grow_stack(|| {
                <OptionalChain as ::treesitter_types::FromNode>::from_node(node, src)
            })
            .map(Self::OptionalChain)
            .unwrap_or(Self::Unknown(node)),
            "private_property_identifier" => ::treesitter_types::runtime::maybe_grow_stack(|| {
                <PrivatePropertyIdentifier as ::treesitter_types::FromNode>::from_node(node, src)
            })
            .map(Self::PrivatePropertyIdentifier)
            .unwrap_or(Self::Unknown(node)),
            "property_identifier" => ::treesitter_types::runtime::maybe_grow_stack(|| {
                <PropertyIdentifier as ::treesitter_types::FromNode>::from_node(node, src)
            })
            .map(Self::PropertyIdentifier)
            .unwrap_or(Self::Unknown(node)),
            "regex_flags" => ::treesitter_types::runtime::maybe_grow_stack(|| {
                <RegexFlags as ::treesitter_types::FromNode>::from_node(node, src)
            })
            .map(Self::RegexFlags)
            .unwrap_or(Self::Unknown(node)),
            "regex_pattern" => ::treesitter_types::runtime::maybe_grow_stack(|| {
                <RegexPattern as ::treesitter_types::FromNode>::from_node(node, src)
            })
            .map(Self::RegexPattern)
            .unwrap_or(Self::Unknown(node)),
            "shorthand_property_identifier" => {
                ::treesitter_types::runtime::maybe_grow_stack(|| {
                    <ShorthandPropertyIdentifier as ::treesitter_types::FromNode>::from_node(
                        node, src,
                    )
                })
                .map(Self::ShorthandPropertyIdentifier)
                .unwrap_or(Self::Unknown(node))
            }
            "shorthand_property_identifier_pattern" => {
                ::treesitter_types::runtime::maybe_grow_stack(|| {
                    <ShorthandPropertyIdentifierPattern as ::treesitter_types::FromNode>::from_node(
                        node, src,
                    )
                })
                .map(Self::ShorthandPropertyIdentifierPattern)
                .unwrap_or(Self::Unknown(node))
            }
            "statement_identifier" => ::treesitter_types::runtime::maybe_grow_stack(|| {
                <StatementIdentifier as ::treesitter_types::FromNode>::from_node(node, src)
            })
            .map(Self::StatementIdentifier)
            .unwrap_or(Self::Unknown(node)),
            "string_fragment" => ::treesitter_types::runtime::maybe_grow_stack(|| {
                <StringFragment as ::treesitter_types::FromNode>::from_node(node, src)
            })
            .map(Self::StringFragment)
            .unwrap_or(Self::Unknown(node)),
            "super" => ::treesitter_types::runtime::maybe_grow_stack(|| {
                <Super as ::treesitter_types::FromNode>::from_node(node, src)
            })
            .map(Self::Super)
            .unwrap_or(Self::Unknown(node)),
            "this" => ::treesitter_types::runtime::maybe_grow_stack(|| {
                <This as ::treesitter_types::FromNode>::from_node(node, src)
            })
            .map(Self::This)
            .unwrap_or(Self::Unknown(node)),
            "true" => ::treesitter_types::runtime::maybe_grow_stack(|| {
                <True as ::treesitter_types::FromNode>::from_node(node, src)
            })
            .map(Self::True)
            .unwrap_or(Self::Unknown(node)),
            "undefined" => ::treesitter_types::runtime::maybe_grow_stack(|| {
                <Undefined as ::treesitter_types::FromNode>::from_node(node, src)
            })
            .map(Self::Undefined)
            .unwrap_or(Self::Unknown(node)),
            _ => Self::Unknown(node),
        }
    }
}
impl ::treesitter_types::Spanned for AnyNode<'_> {
    fn span(&self) -> ::treesitter_types::Span {
        match self {
            Self::Declaration(inner) => inner.span(),
            Self::Expression(inner) => inner.span(),
            Self::Pattern(inner) => inner.span(),
            Self::PrimaryExpression(inner) => inner.span(),
            Self::Statement(inner) => inner.span(),
            Self::Arguments(inner) => inner.span(),
            Self::Array(inner) => inner.span(),
            Self::ArrayPattern(inner) => inner.span(),
            Self::ArrowFunction(inner) => inner.span(),
            Self::AssignmentExpression(inner) => inner.span(),
            Self::AssignmentPattern(inner) => inner.span(),
            Self::AugmentedAssignmentExpression(inner) => inner.span(),
            Self::AwaitExpression(inner) => inner.span(),
            Self::BinaryExpression(inner) => inner.span(),
            Self::BreakStatement(inner) => inner.span(),
            Self::CallExpression(inner) => inner.span(),
            Self::CatchClause(inner) => inner.span(),
            Self::Class(inner) => inner.span(),
            Self::ClassBody(inner) => inner.span(),
            Self::ClassDeclaration(inner) => inner.span(),
            Self::ClassHeritage(inner) => inner.span(),
            Self::ClassStaticBlock(inner) => inner.span(),
            Self::ComputedPropertyName(inner) => inner.span(),
            Self::ContinueStatement(inner) => inner.span(),
            Self::DebuggerStatement(inner) => inner.span(),
            Self::Decorator(inner) => inner.span(),
            Self::DoStatement(inner) => inner.span(),
            Self::ElseClause(inner) => inner.span(),
            Self::EmptyStatement(inner) => inner.span(),
            Self::ExportClause(inner) => inner.span(),
            Self::ExportSpecifier(inner) => inner.span(),
            Self::ExportStatement(inner) => inner.span(),
            Self::ExpressionStatement(inner) => inner.span(),
            Self::FieldDefinition(inner) => inner.span(),
            Self::FinallyClause(inner) => inner.span(),
            Self::ForInStatement(inner) => inner.span(),
            Self::ForStatement(inner) => inner.span(),
            Self::FormalParameters(inner) => inner.span(),
            Self::FunctionDeclaration(inner) => inner.span(),
            Self::FunctionExpression(inner) => inner.span(),
            Self::GeneratorFunction(inner) => inner.span(),
            Self::GeneratorFunctionDeclaration(inner) => inner.span(),
            Self::IfStatement(inner) => inner.span(),
            Self::Import(inner) => inner.span(),
            Self::ImportAttribute(inner) => inner.span(),
            Self::ImportClause(inner) => inner.span(),
            Self::ImportSpecifier(inner) => inner.span(),
            Self::ImportStatement(inner) => inner.span(),
            Self::JsxAttribute(inner) => inner.span(),
            Self::JsxClosingElement(inner) => inner.span(),
            Self::JsxElement(inner) => inner.span(),
            Self::JsxExpression(inner) => inner.span(),
            Self::JsxNamespaceName(inner) => inner.span(),
            Self::JsxOpeningElement(inner) => inner.span(),
            Self::JsxSelfClosingElement(inner) => inner.span(),
            Self::LabeledStatement(inner) => inner.span(),
            Self::LexicalDeclaration(inner) => inner.span(),
            Self::MemberExpression(inner) => inner.span(),
            Self::MetaProperty(inner) => inner.span(),
            Self::MethodDefinition(inner) => inner.span(),
            Self::NamedImports(inner) => inner.span(),
            Self::NamespaceExport(inner) => inner.span(),
            Self::NamespaceImport(inner) => inner.span(),
            Self::NewExpression(inner) => inner.span(),
            Self::Object(inner) => inner.span(),
            Self::ObjectAssignmentPattern(inner) => inner.span(),
            Self::ObjectPattern(inner) => inner.span(),
            Self::Pair(inner) => inner.span(),
            Self::PairPattern(inner) => inner.span(),
            Self::ParenthesizedExpression(inner) => inner.span(),
            Self::Program(inner) => inner.span(),
            Self::Regex(inner) => inner.span(),
            Self::RestPattern(inner) => inner.span(),
            Self::ReturnStatement(inner) => inner.span(),
            Self::SequenceExpression(inner) => inner.span(),
            Self::SpreadElement(inner) => inner.span(),
            Self::StatementBlock(inner) => inner.span(),
            Self::String(inner) => inner.span(),
            Self::SubscriptExpression(inner) => inner.span(),
            Self::SwitchBody(inner) => inner.span(),
            Self::SwitchCase(inner) => inner.span(),
            Self::SwitchDefault(inner) => inner.span(),
            Self::SwitchStatement(inner) => inner.span(),
            Self::TemplateString(inner) => inner.span(),
            Self::TemplateSubstitution(inner) => inner.span(),
            Self::TernaryExpression(inner) => inner.span(),
            Self::ThrowStatement(inner) => inner.span(),
            Self::TryStatement(inner) => inner.span(),
            Self::UnaryExpression(inner) => inner.span(),
            Self::UpdateExpression(inner) => inner.span(),
            Self::UsingDeclaration(inner) => inner.span(),
            Self::VariableDeclaration(inner) => inner.span(),
            Self::VariableDeclarator(inner) => inner.span(),
            Self::WhileStatement(inner) => inner.span(),
            Self::WithStatement(inner) => inner.span(),
            Self::YieldExpression(inner) => inner.span(),
            Self::Comment(inner) => inner.span(),
            Self::EscapeSequence(inner) => inner.span(),
            Self::False(inner) => inner.span(),
            Self::HashBangLine(inner) => inner.span(),
            Self::HtmlCharacterReference(inner) => inner.span(),
            Self::HtmlComment(inner) => inner.span(),
            Self::Identifier(inner) => inner.span(),
            Self::JsxText(inner) => inner.span(),
            Self::Null(inner) => inner.span(),
            Self::Number(inner) => inner.span(),
            Self::OptionalChain(inner) => inner.span(),
            Self::PrivatePropertyIdentifier(inner) => inner.span(),
            Self::PropertyIdentifier(inner) => inner.span(),
            Self::RegexFlags(inner) => inner.span(),
            Self::RegexPattern(inner) => inner.span(),
            Self::ShorthandPropertyIdentifier(inner) => inner.span(),
            Self::ShorthandPropertyIdentifierPattern(inner) => inner.span(),
            Self::StatementIdentifier(inner) => inner.span(),
            Self::StringFragment(inner) => inner.span(),
            Self::Super(inner) => inner.span(),
            Self::This(inner) => inner.span(),
            Self::True(inner) => inner.span(),
            Self::Undefined(inner) => inner.span(),
            Self::Unknown(node) => ::treesitter_types::Span::from(*node),
        }
    }
}
